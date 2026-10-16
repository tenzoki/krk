//! Ein SFTP-Gespraech ueber die Pipes eines Kindprozesses.
//!
//! ```text
//!  anfragende Faeden (je ein &Fernfaden)        Gespraechsfaden krk-sftp-<name>
//!  senden(anfrage) ──┐                            liest die Standardausgabe
//!    Kennung + Sender in die Tabelle              paket_lesen, Paket::zerlegen
//!    Paket unter dem Mutex der Eingabe ──> Kind ──> Antwort mit Kennung k
//!  Ausstehend::warten  <── Sender aus der Tabelle, Eintrag k  <──┘
//!                                                 Ende oder Unsinn:
//!                                                   Kind ernten (wait),
//!                                                   Zustand Getrennt,
//!                                                   jede wartende Anfrage: Getrennt
//!                                                Fehlerfaden krk-sftp-fehler
//!                                                   haelt die letzten Zeilen der
//!                                                   Fehlerausgabe
//! ```
//!
//! **Geschrieben wird von den anfragenden Faeden selbst, gelesen allein vom
//! Gespraechsfaden.** Eine Anfrage traegt sich mit ihrer Kennung und einem
//! Sender in die Tabelle ein, bevor ihr Paket geschrieben wird, damit eine
//! schnelle Antwort ihren Empfaenger schon vorfindet; das Paket geht unter dem
//! Mutex der Standardeingabe ganz hinaus, und zwei Faeden verschraenken ihre
//! Bytes deshalb nie. Der Gespraechsfaden nimmt jede Antwort an ihrer Kennung
//! aus der Tabelle und reicht sie dem Wartenden. Damit duerfen beliebig viele
//! Anfragen zugleich offen sein, aus einem Faden oder aus mehreren: ein
//! Lesevorgang im linken Tab, einer im rechten und ein Vorgang teilen sich ein
//! Gespraech (Entscheidung 4 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`).
//!
//! **Jede Anfrage nimmt ein `&Fernfaden`**, auch das blosse Senden: das
//! Schreiben in eine volle Pipe wartet, bis die Gegenseite liest, und die
//! Gegenseite ist ein Server im Netz. Warum das den Hauptfaden fernhaelt,
//! steht in [`super::faden`](mod@super::faden).
//!
//! # Getrennt ist endgueltig
//!
//! Endet die Ausgabe des Kindprozesses, kommt ein Paket, das sich nicht
//! zerlegen laesst, oder eine Antwort auf eine Kennung, die niemand gestellt
//! hat, dann ist das Gespraech getrennt. Der Gespraechsfaden beendet bei
//! Unsinn den Prozess, erntet ihn in jedem Fall mit `wait`, wartet das Ende
//! der Fehlerausgabe ab und setzt erst dann den [`Zustand::Getrennt`]; so
//! traegt die [`Trennung`] die Zeilen, mit denen `ssh` sein Scheitern
//! begruendet. Jede wartende Anfrage bekommt [`Fernfehler::Getrennt`], jede
//! spaetere ebenso, ohne dass noch ein Byte geschrieben wird. Ein neues
//! Gespraech baut der Zugang, nicht dieses Modul.
//!
//! **Wer das Gespraech fallen laesst, schliesst die Standardeingabe des Kindes
//! und wartet auf nichts**; auch auf dem Hauptfaden ist das Fallenlassen
//! deshalb erlaubt. `sftp-server` und `ssh` enden am Ende ihrer Eingabe, der
//! Gespraechsfaden sieht dann das Ende der Ausgabe und erntet den Prozess.
//!
//! # Was dieses Modul nicht haelt
//!
//! Eine Zeitgrenze fuer eine einzelne Antwort gibt es nicht. Ein Server, der
//! eine Anfrage nie beantwortet und die Verbindung offen haelt, haelt den
//! wartenden Arbeitsfaden fest, bis die Verbindung faellt; gegen eine tote
//! Leitung stehen die Optionen `ServerAliveInterval` und `ServerAliveCountMax`
//! des `ssh`-Aufrufs, gegen einen lebenden, aber stummen Server steht nichts.
//! Und der Gespraechsfaden erntet den Prozess, nachdem seine Ausgabe endete;
//! ein Prozess, der seine Ausgabe schliesst und trotzdem weiterlaeuft, hielte
//! die Trennung auf. `ssh` und `sftp-server` tun das nicht.

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::ops::ControlFlow;
use std::process::{Child, ChildStdin, ChildStdout};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

use super::faden::Fernfaden;
use super::paket::{
    Anfrage, Antwort, Attribute, FASSUNG, Name, Paare, Paket, Paketfehler, paket_lesen, status,
};

/// Wie viele Bytes ein Block beim Lesen und Schreiben einer Datei traegt.
pub const BLOCK: u32 = 32 * 1024;

/// Wie viele Lesungen [`Gespraech::lesen_in_bloecken`] zugleich offen haelt.
pub const OFFENE_LESUNGEN: usize = 16;

/// Wie viele Schreibungen ein Schreiber in Bloecken zugleich offen haelt
/// (`operation::fern`); dieselbe Zahl wie beim Lesen, Entscheidung 9 des
/// Plans.
pub const OFFENE_SCHREIBUNGEN: usize = 16;

/// Wie viele Zeilen der Fehlerausgabe eine [`Trennung`] mitnimmt.
const FEHLERZEILEN: usize = 8;

/// Wie viele Bytes einer Zeile der Fehlerausgabe gehalten werden. Der Rest
/// der Zeile bis zu ihrem Ende wird gelesen und verworfen; die Statuszeile
/// zeigt ohnehin nur eine Zeile.
const FEHLERZEILENLAENGE: usize = 4096;

/// Die Erweiterung, an der sich ein Server als OpenSSH zu erkennen gibt.
const OPENSSH_KENNUNG: &[u8] = b"posix-rename@openssh.com";

/// Warum eine Anfrage keine brauchbare Antwort bekam.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fernfehler {
    /// Das Gespraech ist getrennt; ein neues baut der Zugang.
    Getrennt(Trennung),
    /// Der Server hat mit einem Statuscode ausser `OK` geantwortet.
    Status {
        /// Der Code aus [`status`].
        code: u32,
        /// Der Text des Servers, verlustbehaftet als UTF-8 gelesen.
        meldung: String,
    },
    /// Die Antwort passt nicht zur Anfrage, oder ihr Inhalt verletzt das
    /// Protokoll, ohne den Strom zu zerreissen.
    Protokoll(String),
    /// Der Server bietet an, was die Anfrage braucht, nicht in einer Form, die
    /// KRK sicher lesen kann.
    NichtAngeboten(&'static str),
    /// Der Rufer hat abgebrochen.
    Abgebrochen,
}

impl Fernfehler {
    /// Ob der Server mit genau diesem Statuscode geantwortet hat.
    #[must_use]
    pub fn ist_status(&self, gesucht: u32) -> bool {
        matches!(self, Fernfehler::Status { code, .. } if *code == gesucht)
    }
}

impl fmt::Display for Fernfehler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fernfehler::Getrennt(trennung) => write!(f, "getrennt: {trennung}"),
            Fernfehler::Status { code, meldung } => write!(f, "Status {code}: {meldung}"),
            Fernfehler::Protokoll(text) => write!(f, "Protokoll: {text}"),
            Fernfehler::NichtAngeboten(was) => write!(f, "nicht angeboten: {was}"),
            Fernfehler::Abgebrochen => write!(f, "abgebrochen"),
        }
    }
}

/// Wie und warum ein Gespraech endete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trennung {
    /// Der Grund.
    pub grund: Trennungsgrund,
    /// Die letzten Zeilen der Fehlerausgabe des Kindprozesses, die juengste
    /// zuletzt. Bei `ssh` steht hier, woran die Verbindung scheiterte.
    pub fehlerausgabe: Vec<String>,
    /// Der Ausgangscode des Kindprozesses, wenn er mit einem endete.
    pub ausgang: Option<i32>,
}

impl fmt::Display for Trennung {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grund)?;
        if let Some(zeile) = self.fehlerausgabe.last() {
            write!(f, " ({zeile})")?;
        }
        Ok(())
    }
}

/// Der Grund einer [`Trennung`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trennungsgrund {
    /// Das Gespraech kam nicht zustande, bevor ein Byte gesprochen war.
    Aufbau(String),
    /// Die Ausgabe des Kindprozesses endete.
    AusgabeEndete,
    /// Ein Paket liess sich nicht zerlegen oder passte zu keiner Anfrage.
    Unsinn(String),
    /// Die Ausgabe meldete einen Lesefehler.
    Lesefehler(String),
    /// Die Eingabe nahm ein Paket nicht an.
    Schreibfehler(String),
}

impl fmt::Display for Trennungsgrund {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trennungsgrund::Aufbau(text) => write!(f, "Aufbau gescheitert: {text}"),
            Trennungsgrund::AusgabeEndete => write!(f, "Ausgabe endete"),
            Trennungsgrund::Unsinn(text) => write!(f, "Unsinn: {text}"),
            Trennungsgrund::Lesefehler(text) => write!(f, "Lesefehler: {text}"),
            Trennungsgrund::Schreibfehler(text) => write!(f, "Schreibfehler: {text}"),
        }
    }
}

/// Ob ein Gespraech noch Anfragen annimmt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zustand {
    /// Es nimmt Anfragen an.
    Offen,
    /// Es ist getrennt, und jede Anfrage bekommt [`Fernfehler::Getrennt`].
    Getrennt(Trennung),
}

/// Ein offenes Handle auf dem Server, aus `OPEN` oder `OPENDIR`.
///
/// Nicht `Clone`: [`Gespraech::close`] nimmt es, und ein zweites Schliessen
/// desselben Handles soll nicht baubar sein.
#[derive(Debug, PartialEq, Eq)]
pub struct Handle(Vec<u8>);

/// Eine gesendete Anfrage, deren Antwort noch aussteht.
#[must_use = "fallengelassen geht die Antwort verloren; fuer eine Anfrage ohne Blick auf die \
              Antwort steht ausdruecklich `let _ =`"]
pub struct Ausstehend {
    empfang: Receiver<Result<Antwort, Fernfehler>>,
}

impl Ausstehend {
    /// Wartet auf die Antwort.
    pub fn warten(self, _faden: &Fernfaden) -> Result<Antwort, Fernfehler> {
        match self.empfang.recv() {
            Ok(antwort) => antwort,
            // Der Sender faellt allein mit der Tabelle, also mit dem Gespraech
            // selbst; dann gibt es auch niemanden mehr, der antworten koennte.
            Err(_) => Err(Fernfehler::Getrennt(Trennung {
                grund: Trennungsgrund::Lesefehler("Antwortweg geschlossen".to_owned()),
                fehlerausgabe: Vec::new(),
                ausgang: None,
            })),
        }
    }

    /// Wartet auf die Antwort einer Anfrage, die mit `STATUS OK` quittiert
    /// wird, etwa `WRITE`, und liefert jede andere Antwort als Fehler.
    pub fn bestaetigen(self, faden: &Fernfaden) -> Result<(), Fernfehler> {
        erfolg(self.warten(faden)?)
    }
}

/// Die Tabelle der wartenden Anfragen und der Zustand, unter einem Mutex.
struct Tabelle {
    naechste: u32,
    wartend: HashMap<u32, Sender<Result<Antwort, Fernfehler>>>,
    zustand: Zustand,
}

impl Tabelle {
    fn freie_kennung(&mut self) -> u32 {
        loop {
            let kennung = self.naechste;
            self.naechste = self.naechste.wrapping_add(1);
            if !self.wartend.contains_key(&kennung) {
                return kennung;
            }
        }
    }
}

/// Die letzten Zeilen der Fehlerausgabe, geteilt zwischen dem Fehlerfaden und
/// dem, der die Trennung schreibt.
type Fehlerzeilen = Arc<Mutex<VecDeque<String>>>;

/// Was der Gespraechsfaden nach seinem Start bekommt: das Kind, seine
/// Ausgabe und den Fehlerfaden, den er nach der Trennung abwartet.
type Uebergabe = (Child, ChildStdout, JoinHandle<()>);

/// Ein SFTP-Gespraech ueber die Pipes eines Kindprozesses.
pub struct Gespraech {
    tabelle: Arc<Mutex<Tabelle>>,
    eingabe: Mutex<ChildStdin>,
    zeilen: Fehlerzeilen,
    fassung: u32,
    erweiterungen: Paare,
    prozess: u32,
}

/// Ein vergifteter Mutex wird uebergangen: er entstuende nur aus einem
/// Absturz auf einem anderen Faden, und ein zweiter Absturz hier machte den
/// ersten nicht besser.
fn sperren<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    match mutex.lock() {
        Ok(wache) => wache,
        Err(vergiftet) => vergiftet.into_inner(),
    }
}

impl Gespraech {
    /// Eroeffnet ein Gespraech mit dem Kindprozess `kind`, dessen Standardein-,
    /// -aus- und Fehlerausgabe Pipes sein muessen: `INIT`, dann `VERSION`,
    /// dann laeuft der Gespraechsfaden `krk-sftp-<bezeichnung>`.
    ///
    /// Scheitert die Eroeffnung, ist das Kind beendet und geerntet, und der
    /// Fehler ist [`Fernfehler::Getrennt`] mit den Zeilen der Fehlerausgabe.
    pub fn eroeffnen(
        _faden: &Fernfaden,
        mut kind: Child,
        bezeichnung: &str,
    ) -> Result<Gespraech, Fernfehler> {
        let prozess = kind.id();
        let (Some(mut eingabe), Some(mut ausgabe), Some(fehler)) =
            (kind.stdin.take(), kind.stdout.take(), kind.stderr.take())
        else {
            let grund = Trennungsgrund::Aufbau("Kindprozess ohne drei Pipes".to_owned());
            return Err(Fernfehler::Getrennt(abschliessen(kind, None, None, grund)));
        };

        let zeilen: Fehlerzeilen = Arc::new(Mutex::new(VecDeque::new()));
        let fuer_den_fehlerfaden = Arc::clone(&zeilen);
        let fehlerfaden = match thread::Builder::new()
            .name("krk-sftp-fehler".to_owned())
            .spawn(move || fehlerausgabe_lesen(fehler, &fuer_den_fehlerfaden))
        {
            Ok(faden) => faden,
            Err(fehler) => {
                let grund = Trennungsgrund::Aufbau(format!("kein Fehlerfaden: {fehler}"));
                drop(eingabe);
                return Err(Fernfehler::Getrennt(abschliessen(kind, None, None, grund)));
            }
        };

        // Ein Schreibfehler hier heisst, dass das Kind schon endet; die Ausgabe
        // endet dann ebenso, und das Lesen darunter traegt den Grund.
        let init = Paket::Init {
            fassung: FASSUNG,
            erweiterungen: Vec::new(),
        }
        .kodieren();
        let _ = eingabe.write_all(&init).and_then(|()| eingabe.flush());

        let version = match paket_lesen(&mut ausgabe) {
            Ok(Some(rumpf)) => match Paket::zerlegen(&rumpf) {
                Ok(Paket::Version {
                    fassung,
                    erweiterungen,
                }) if fassung >= FASSUNG => Ok((fassung, erweiterungen)),
                Ok(Paket::Version { fassung, .. }) => Err(Trennungsgrund::Unsinn(format!(
                    "der Server spricht Fassung {fassung}"
                ))),
                Ok(_) => Err(Trennungsgrund::Unsinn(
                    "erstes Paket ist nicht VERSION".to_owned(),
                )),
                Err(fehler) => Err(Trennungsgrund::Unsinn(fehler.to_string())),
            },
            Ok(None) => Err(Trennungsgrund::AusgabeEndete),
            Err(fehler) => Err(grund_aus_paketfehler(fehler)),
        };
        let (fassung, erweiterungen) = match version {
            Ok(version) => version,
            Err(grund) => {
                drop(eingabe);
                drop(ausgabe);
                return Err(Fernfehler::Getrennt(abschliessen(
                    kind,
                    Some(fehlerfaden),
                    Some(&zeilen),
                    grund,
                )));
            }
        };

        let tabelle = Arc::new(Mutex::new(Tabelle {
            naechste: 1,
            wartend: HashMap::new(),
            zustand: Zustand::Offen,
        }));

        // Kind und Ausgabe reisen ueber einen Kanal in den Gespraechsfaden und
        // nicht im Abschluss: scheitert der Fadenstart, sind sie noch hier und
        // werden beendet und geerntet, statt mit dem Abschluss still zu fallen.
        let (uebergabe, annahme) = mpsc::channel::<Uebergabe>();
        let fuer_den_gespraechsfaden = Arc::clone(&tabelle);
        let zeilen_des_gespraechs = Arc::clone(&zeilen);
        let gestartet = thread::Builder::new()
            .name(format!("krk-sftp-{bezeichnung}"))
            .spawn(move || {
                if let Ok((kind, ausgabe, fehlerfaden)) = annahme.recv() {
                    gespraech_fuehren(
                        kind,
                        ausgabe,
                        fehlerfaden,
                        &zeilen_des_gespraechs,
                        &fuer_den_gespraechsfaden,
                    );
                }
            });
        if let Err(fehler) = gestartet {
            let grund = Trennungsgrund::Aufbau(format!("kein Gespraechsfaden: {fehler}"));
            drop(eingabe);
            drop(ausgabe);
            return Err(Fernfehler::Getrennt(abschliessen(
                kind,
                Some(fehlerfaden),
                Some(&zeilen),
                grund,
            )));
        }
        if let Err(mpsc::SendError((kind, ausgabe, fehlerfaden))) =
            uebergabe.send((kind, ausgabe, fehlerfaden))
        {
            // Der Faden lief an und ist vor der Annahme verschwunden.
            let grund = Trennungsgrund::Aufbau("Gespraechsfaden ohne Annahme".to_owned());
            drop(eingabe);
            drop(ausgabe);
            return Err(Fernfehler::Getrennt(abschliessen(
                kind,
                Some(fehlerfaden),
                Some(&zeilen),
                grund,
            )));
        }

        Ok(Gespraech {
            tabelle,
            eingabe: Mutex::new(eingabe),
            zeilen,
            fassung,
            erweiterungen,
            prozess,
        })
    }

    /// Die Fassung, die der Server in `VERSION` genannt hat.
    #[must_use]
    pub fn fassung(&self) -> u32 {
        self.fassung
    }

    /// Die Erweiterungen aus `VERSION`, als Name und Wert.
    #[must_use]
    pub fn erweiterungen(&self) -> &Paare {
        &self.erweiterungen
    }

    /// Ob der Server die Erweiterung `name` anbietet.
    #[must_use]
    pub fn kennt_erweiterung(&self, name: &[u8]) -> bool {
        self.erweiterungen.iter().any(|(n, _)| n == name)
    }

    /// Ob der Server sich durch `posix-rename@openssh.com` als OpenSSH zu
    /// erkennen gibt. Daran haengt die Argumentfolge von `SYMLINK`.
    #[must_use]
    pub fn spricht_openssh(&self) -> bool {
        self.kennt_erweiterung(OPENSSH_KENNUNG)
    }

    /// Die Prozesskennung des Kindes.
    #[must_use]
    pub fn prozesskennung(&self) -> u32 {
        self.prozess
    }

    /// Ob das Gespraech offen oder getrennt ist.
    #[must_use]
    pub fn zustand(&self) -> Zustand {
        sperren(&self.tabelle).zustand.clone()
    }

    /// Sendet eine Anfrage und kehrt zurueck, ohne auf die Antwort zu warten.
    ///
    /// Das Schreiben selbst kann warten, wenn die Pipe voll ist; deshalb
    /// verlangt auch das Senden ein [`Fernfaden`].
    pub fn senden(&self, _faden: &Fernfaden, anfrage: Anfrage) -> Result<Ausstehend, Fernfehler> {
        let (sender, empfang) = mpsc::channel();
        let kennung = {
            let mut tabelle = sperren(&self.tabelle);
            if let Zustand::Getrennt(trennung) = &tabelle.zustand {
                return Err(Fernfehler::Getrennt(trennung.clone()));
            }
            let kennung = tabelle.freie_kennung();
            tabelle.wartend.insert(kennung, sender);
            kennung
        };
        let bytes = Paket::Anfrage { kennung, anfrage }.kodieren();
        let geschrieben = {
            let mut eingabe = sperren(&self.eingabe);
            eingabe.write_all(&bytes).and_then(|()| eingabe.flush())
        };
        if let Err(fehler) = geschrieben {
            // Steht der Eintrag noch in der Tabelle, hat niemand geantwortet,
            // und er wird hier beantwortet; sonst hat der Gespraechsfaden das
            // schon getan, und die Antwort liegt im Kanal.
            let noch_wartend = sperren(&self.tabelle).wartend.remove(&kennung).is_some();
            if noch_wartend {
                return Err(Fernfehler::Getrennt(Trennung {
                    grund: Trennungsgrund::Schreibfehler(fehler.to_string()),
                    fehlerausgabe: sperren(&self.zeilen).iter().cloned().collect(),
                    ausgang: None,
                }));
            }
        }
        Ok(Ausstehend { empfang })
    }

    /// Sendet eine Anfrage und wartet auf ihre Antwort.
    pub fn anfragen(&self, faden: &Fernfaden, anfrage: Anfrage) -> Result<Antwort, Fernfehler> {
        self.senden(faden, anfrage)?.warten(faden)
    }

    /// `REALPATH`: der aufgeloeste Pfad als Bytes.
    pub fn realpath(&self, faden: &Fernfaden, pfad: &str) -> Result<Vec<u8>, Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Realpath {
                pfad: pfad.as_bytes().to_vec(),
            },
        )?;
        ein_name(antwort)
    }

    /// `STAT`: die Attribute, einer Verknuepfung folgend.
    pub fn stat(&self, faden: &Fernfaden, pfad: &str) -> Result<Attribute, Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Stat {
                pfad: pfad.as_bytes().to_vec(),
            },
        )?;
        attribute(antwort)
    }

    /// `LSTAT`: die Attribute, ohne einer Verknuepfung zu folgen.
    pub fn lstat(&self, faden: &Fernfaden, pfad: &str) -> Result<Attribute, Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Lstat {
                pfad: pfad.as_bytes().to_vec(),
            },
        )?;
        attribute(antwort)
    }

    /// `OPENDIR`: ein Handle zum Lesen eines Ordners.
    pub fn opendir(&self, faden: &Fernfaden, pfad: &str) -> Result<Handle, Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Opendir {
                pfad: pfad.as_bytes().to_vec(),
            },
        )?;
        handle(antwort)
    }

    /// `READDIR`: die naechsten Namen, oder `None` am Ende des Ordners.
    pub fn readdir(
        &self,
        faden: &Fernfaden,
        handle: &Handle,
    ) -> Result<Option<Vec<Name>>, Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Readdir {
                handle: handle.0.clone(),
            },
        )?;
        match antwort {
            Antwort::Name(namen) => Ok(Some(namen)),
            Antwort::Status {
                code: status::EOF, ..
            } => Ok(None),
            andere => Err(fehlschlag(andere)),
        }
    }

    /// `OPEN`: eine Datei oeffnen, mit den Kennzeichen aus
    /// [`super::paket::oeffnen`] und den Attributen einer neu angelegten
    /// Datei.
    pub fn open(
        &self,
        faden: &Fernfaden,
        pfad: &str,
        kennzeichen: u32,
        attribute: Attribute,
    ) -> Result<Handle, Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Open {
                pfad: pfad.as_bytes().to_vec(),
                kennzeichen,
                attribute,
            },
        )?;
        handle(antwort)
    }

    /// `READ`: hoechstens `laenge` Bytes ab `versatz`, oder `None` am Ende der
    /// Datei. Weniger Bytes als verlangt sind kein Ende.
    pub fn read(
        &self,
        faden: &Fernfaden,
        handle: &Handle,
        versatz: u64,
        laenge: u32,
    ) -> Result<Option<Vec<u8>>, Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Read {
                handle: handle.0.clone(),
                versatz,
                laenge,
            },
        )?;
        daten(antwort, laenge)
    }

    /// `WRITE`: Bytes ab `versatz`.
    pub fn write(
        &self,
        faden: &Fernfaden,
        handle: &Handle,
        versatz: u64,
        daten: &[u8],
    ) -> Result<(), Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Write {
                handle: handle.0.clone(),
                versatz,
                daten: daten.to_vec(),
            },
        )?;
        erfolg(antwort)
    }

    /// `WRITE` senden, ohne auf die Antwort zu warten.
    ///
    /// Fuer einen Schreiber, der mehrere Bloecke zugleich offen haelt; ob der
    /// Block angekommen ist, sagt [`Ausstehend::bestaetigen`].
    pub fn write_senden(
        &self,
        faden: &Fernfaden,
        handle: &Handle,
        versatz: u64,
        daten: &[u8],
    ) -> Result<Ausstehend, Fernfehler> {
        self.senden(
            faden,
            Anfrage::Write {
                handle: handle.0.clone(),
                versatz,
                daten: daten.to_vec(),
            },
        )
    }

    /// `CLOSE`: das Handle schliessen.
    pub fn close(&self, faden: &Fernfaden, handle: Handle) -> Result<(), Fernfehler> {
        let antwort = self.anfragen(faden, Anfrage::Close { handle: handle.0 })?;
        erfolg(antwort)
    }

    /// `SETSTAT`: Attribute setzen.
    pub fn setstat(
        &self,
        faden: &Fernfaden,
        pfad: &str,
        attribute: Attribute,
    ) -> Result<(), Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Setstat {
                pfad: pfad.as_bytes().to_vec(),
                attribute,
            },
        )?;
        erfolg(antwort)
    }

    /// `MKDIR`: einen Ordner anlegen; ein vorhandener Name scheitert.
    pub fn mkdir(
        &self,
        faden: &Fernfaden,
        pfad: &str,
        attribute: Attribute,
    ) -> Result<(), Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Mkdir {
                pfad: pfad.as_bytes().to_vec(),
                attribute,
            },
        )?;
        erfolg(antwort)
    }

    /// `RMDIR`: einen leeren Ordner entfernen.
    pub fn rmdir(&self, faden: &Fernfaden, pfad: &str) -> Result<(), Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Rmdir {
                pfad: pfad.as_bytes().to_vec(),
            },
        )?;
        erfolg(antwort)
    }

    /// `REMOVE`: eine Datei entfernen.
    pub fn remove(&self, faden: &Fernfaden, pfad: &str) -> Result<(), Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Remove {
                pfad: pfad.as_bytes().to_vec(),
            },
        )?;
        erfolg(antwort)
    }

    /// `RENAME`: umbenennen.
    ///
    /// **Nicht `posix-rename@openssh.com`**, auch wo der Server es anbietet:
    /// der Entwurf schreibt vor, dass `RENAME` an einem vorhandenen Ziel
    /// scheitert, und genau das verlangt die lokale Regel, die `renamex_np`
    /// mit `RENAME_EXCL` ruft; `posix-rename` ersetzte das Ziel still
    /// (Entscheidung 10 des Plans). Am Server des Systems haelt das die Probe
    /// `rename_weist_ein_vorhandenes_ziel_ab`; fuer jeden anderen Server ist
    /// es eine Zusage des Entwurfs und keine von KRK.
    pub fn rename(&self, faden: &Fernfaden, alt: &str, neu: &str) -> Result<(), Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Rename {
                alt: alt.as_bytes().to_vec(),
                neu: neu.as_bytes().to_vec(),
            },
        )?;
        erfolg(antwort)
    }

    /// `READLINK`: das Ziel einer Verknuepfung als Bytes.
    pub fn readlink(&self, faden: &Fernfaden, pfad: &str) -> Result<Vec<u8>, Fernfehler> {
        let antwort = self.anfragen(
            faden,
            Anfrage::Readlink {
                pfad: pfad.as_bytes().to_vec(),
            },
        )?;
        ein_name(antwort)
    }

    /// `SYMLINK`: legt unter `verknuepfung` eine Verknuepfung auf `ziel` an.
    ///
    /// OpenSSH liest die zwei Zeichenketten in umgekehrter Folge als der
    /// Entwurf, und ein anderer Server kann jede der beiden lesen. Gesendet
    /// wird deshalb allein an einen Server, der sich als OpenSSH zu erkennen
    /// gibt ([`Gespraech::spricht_openssh`]); jeder andere bekommt
    /// [`Fernfehler::NichtAngeboten`], und der Rufer ueberspringt die
    /// Verknuepfung, statt eine mit vertauschten Rollen anzulegen.
    pub fn symlink(
        &self,
        faden: &Fernfaden,
        verknuepfung: &str,
        ziel: &str,
    ) -> Result<(), Fernfehler> {
        if !self.spricht_openssh() {
            return Err(Fernfehler::NichtAngeboten(
                "SYMLINK ohne bekannte Argumentfolge",
            ));
        }
        let antwort = self.anfragen(
            faden,
            Anfrage::Symlink {
                erste: ziel.as_bytes().to_vec(),
                zweite: verknuepfung.as_bytes().to_vec(),
            },
        )?;
        erfolg(antwort)
    }

    /// Liest die Datei hinter `handle` vom Anfang bis zum Ende in Bloecken zu
    /// [`BLOCK`] Bytes, mit bis zu [`OFFENE_LESUNGEN`] Lesungen zugleich, und
    /// reicht die Bytes **in ihrer Reihenfolge** an `empfang`. Liefert die
    /// Zahl der gelesenen Bytes.
    ///
    /// `empfang` antwortet mit `Break`, um abzubrechen; das Ergebnis ist dann
    /// [`Fernfehler::Abgebrochen`], und die noch offenen Lesungen werden
    /// fallengelassen, ihre Antworten verworfen. Liefert der Server fuer einen
    /// Block weniger Bytes als verlangt, holt die Schleife den Rest dieses
    /// Blocks, bevor sie den naechsten weiterreicht, sodass keine Luecke
    /// entsteht.
    pub fn lesen_in_bloecken(
        &self,
        faden: &Fernfaden,
        handle: &Handle,
        mut empfang: impl FnMut(&[u8]) -> ControlFlow<()>,
    ) -> Result<u64, Fernfehler> {
        let mut offen: VecDeque<(u64, Ausstehend)> = VecDeque::new();
        let mut naechster: u64 = 0;
        let mut gelesen: u64 = 0;
        while offen.len() < OFFENE_LESUNGEN {
            offen.push_back((naechster, self.lesung(faden, handle, naechster, BLOCK)?));
            naechster += u64::from(BLOCK);
        }
        while let Some((versatz, ausstehend)) = offen.pop_front() {
            let Some(daten) = daten(ausstehend.warten(faden)?, BLOCK)? else {
                break;
            };
            gelesen += daten.len() as u64;
            if empfang(&daten).is_break() {
                return Err(Fernfehler::Abgebrochen);
            }
            let blockende = versatz + u64::from(BLOCK);
            let mut stelle = versatz + daten.len() as u64;
            let mut am_ende = false;
            while stelle < blockende {
                let rest = u32::try_from(blockende - stelle).unwrap_or(BLOCK);
                match self.read(faden, handle, stelle, rest)? {
                    Some(nachschub) if nachschub.is_empty() => {
                        return Err(Fernfehler::Protokoll(
                            "READ liefert null Bytes ohne Ende".to_owned(),
                        ));
                    }
                    Some(nachschub) => {
                        gelesen += nachschub.len() as u64;
                        stelle += nachschub.len() as u64;
                        if empfang(&nachschub).is_break() {
                            return Err(Fernfehler::Abgebrochen);
                        }
                    }
                    None => {
                        am_ende = true;
                        break;
                    }
                }
            }
            if am_ende {
                break;
            }
            offen.push_back((naechster, self.lesung(faden, handle, naechster, BLOCK)?));
            naechster += u64::from(BLOCK);
        }
        Ok(gelesen)
    }

    fn lesung(
        &self,
        faden: &Fernfaden,
        handle: &Handle,
        versatz: u64,
        laenge: u32,
    ) -> Result<Ausstehend, Fernfehler> {
        self.senden(
            faden,
            Anfrage::Read {
                handle: handle.0.clone(),
                versatz,
                laenge,
            },
        )
    }
}

/// Die Fehlerausgabe zeilenweise lesen und die letzten [`FEHLERZEILEN`]
/// halten, jede hoechstens [`FEHLERZEILENLAENGE`] Bytes lang. Ohne diesen
/// Leser fuellte `ssh` die Pipe und hielte an.
///
/// **Gehalten wird nie mehr als die zwei Grenzen zusammen**, auch an einem
/// Strom ohne Zeilenende: `ssh` reicht die Fehlerausgabe des Servers weiter,
/// und ein feindlicher oder fehlerhafter Server liesse eine Zeile ohne Grenze
/// den Speicher von KRK unbegrenzt wachsen. Was ueber die Laenge hinausgeht,
/// wird bis zum naechsten Zeilenende weiter gelesen und verworfen, damit
/// `ssh` nicht an einer vollen Pipe anhaelt. Eine letzte Zeile ohne
/// Zeilenende zaehlt wie eine mit.
fn fehlerausgabe_lesen(fehler: impl Read, zeilen: &Fehlerzeilen) {
    let mut leser = BufReader::new(fehler);
    let mut zeile: Vec<u8> = Vec::new();
    loop {
        let puffer = match leser.fill_buf() {
            Ok([]) => break,
            Ok(puffer) => puffer,
            Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        };
        let (stueck, zeilenende) = match puffer.iter().position(|&byte| byte == b'\n') {
            Some(stelle) => (&puffer[..stelle], Some(stelle)),
            None => (puffer, None),
        };
        let platz = FEHLERZEILENLAENGE.saturating_sub(zeile.len());
        zeile.extend_from_slice(&stueck[..stueck.len().min(platz)]);
        let verbraucht = zeilenende.map_or(puffer.len(), |stelle| stelle + 1);
        leser.consume(verbraucht);
        if zeilenende.is_some() {
            zeile_halten(&mut zeile, zeilen);
        }
    }
    zeile_halten(&mut zeile, zeilen);
}

/// Nimmt eine gelesene Zeile in die gehaltenen auf, ohne Leerraum am Ende,
/// und leert sie fuer die naechste; eine leere Zeile zaehlt nicht.
fn zeile_halten(zeile: &mut Vec<u8>, zeilen: &Fehlerzeilen) {
    let text = String::from_utf8_lossy(zeile).trim_end().to_owned();
    zeile.clear();
    if text.is_empty() {
        return;
    }
    let mut gehalten = sperren(zeilen);
    if gehalten.len() == FEHLERZEILEN {
        gehalten.pop_front();
    }
    gehalten.push_back(text);
}

/// Der Rumpf des Gespraechsfadens: Antworten zustellen, bis das Gespraech
/// endet, dann das Kind ernten und trennen.
fn gespraech_fuehren(
    kind: Child,
    mut ausgabe: ChildStdout,
    fehlerfaden: JoinHandle<()>,
    zeilen: &Fehlerzeilen,
    tabelle: &Mutex<Tabelle>,
) {
    let grund = loop {
        let rumpf = match paket_lesen(&mut ausgabe) {
            Ok(Some(rumpf)) => rumpf,
            Ok(None) => break Trennungsgrund::AusgabeEndete,
            Err(fehler) => break grund_aus_paketfehler(fehler),
        };
        let (kennung, antwort) = match Paket::zerlegen(&rumpf) {
            Ok(Paket::Antwort { kennung, antwort }) => (kennung, antwort),
            Ok(_) => break Trennungsgrund::Unsinn("Paket ohne Antwortcharakter".to_owned()),
            Err(fehler) => break Trennungsgrund::Unsinn(fehler.to_string()),
        };
        let wartender = sperren(tabelle).wartend.remove(&kennung);
        match wartender {
            // Hat der Wartende aufgegeben, etwa nach einem Abbruch, faellt die
            // Antwort; das ist kein Fehler des Gespraechs.
            Some(sender) => {
                let _ = sender.send(Ok(antwort));
            }
            None => break Trennungsgrund::Unsinn(format!("Antwort auf fremde Kennung {kennung}")),
        }
    };
    drop(ausgabe);
    let trennung = abschliessen(kind, Some(fehlerfaden), Some(zeilen), grund);
    let mut tabelle = sperren(tabelle);
    for (_, sender) in tabelle.wartend.drain() {
        let _ = sender.send(Err(Fernfehler::Getrennt(trennung.clone())));
    }
    tabelle.zustand = Zustand::Getrennt(trennung);
}

/// Beendet das Kind, wenn der Grund es verlangt, erntet es, wartet das Ende
/// der Fehlerausgabe ab und schreibt die [`Trennung`].
///
/// Beendet wird bei Unsinn, einem Lesefehler und einem gescheiterten Aufbau:
/// dann spricht das Kind noch und haette sonst keinen Grund zu enden. Endete
/// seine Ausgabe, endet es von selbst.
fn abschliessen(
    mut kind: Child,
    fehlerfaden: Option<JoinHandle<()>>,
    zeilen: Option<&Fehlerzeilen>,
    grund: Trennungsgrund,
) -> Trennung {
    let beenden = match &grund {
        Trennungsgrund::AusgabeEndete => false,
        Trennungsgrund::Aufbau(_)
        | Trennungsgrund::Unsinn(_)
        | Trennungsgrund::Lesefehler(_)
        | Trennungsgrund::Schreibfehler(_) => true,
    };
    if beenden {
        // Scheitert das Beenden, ist das Kind schon fort; `wait` erntet es.
        let _ = kind.kill();
    }
    let ausgang = kind.wait().ok().and_then(|stand| stand.code());
    if let Some(fehlerfaden) = fehlerfaden {
        // Ein abgestuerzter Fehlerfaden laesst die bis dahin gehaltenen Zeilen
        // stehen; mehr gibt es dann nicht.
        let _ = fehlerfaden.join();
    }
    let fehlerausgabe = zeilen
        .map(|zeilen| sperren(zeilen).iter().cloned().collect())
        .unwrap_or_default();
    Trennung {
        grund,
        fehlerausgabe,
        ausgang,
    }
}

fn grund_aus_paketfehler(fehler: Paketfehler) -> Trennungsgrund {
    match fehler {
        Paketfehler::Lesefehler(fehler) => Trennungsgrund::Lesefehler(fehler.to_string()),
        Paketfehler::Abgerissen => Trennungsgrund::AusgabeEndete,
        andere => Trennungsgrund::Unsinn(andere.to_string()),
    }
}

/// Eine Antwort, die nicht die erwartete war: ein Status wird zu
/// [`Fernfehler::Status`], alles andere zu [`Fernfehler::Protokoll`].
fn fehlschlag(antwort: Antwort) -> Fernfehler {
    match antwort {
        Antwort::Status { code, meldung, .. } => Fernfehler::Status {
            code,
            meldung: String::from_utf8_lossy(&meldung).into_owned(),
        },
        Antwort::Handle(_) => Fernfehler::Protokoll("unerwartet HANDLE".to_owned()),
        Antwort::Data(_) => Fernfehler::Protokoll("unerwartet DATA".to_owned()),
        Antwort::Name(_) => Fernfehler::Protokoll("unerwartet NAME".to_owned()),
        Antwort::Attrs(_) => Fernfehler::Protokoll("unerwartet ATTRS".to_owned()),
    }
}

fn erfolg(antwort: Antwort) -> Result<(), Fernfehler> {
    match antwort {
        Antwort::Status {
            code: status::OK, ..
        } => Ok(()),
        andere => Err(fehlschlag(andere)),
    }
}

fn handle(antwort: Antwort) -> Result<Handle, Fernfehler> {
    match antwort {
        Antwort::Handle(handle) => Ok(Handle(handle)),
        andere => Err(fehlschlag(andere)),
    }
}

fn attribute(antwort: Antwort) -> Result<Attribute, Fernfehler> {
    match antwort {
        Antwort::Attrs(attribute) => Ok(attribute),
        andere => Err(fehlschlag(andere)),
    }
}

fn ein_name(antwort: Antwort) -> Result<Vec<u8>, Fernfehler> {
    match antwort {
        Antwort::Name(mut namen) if namen.len() == 1 => Ok(namen.remove(0).name),
        Antwort::Name(namen) => Err(Fernfehler::Protokoll(format!(
            "{} Namen statt eines",
            namen.len()
        ))),
        andere => Err(fehlschlag(andere)),
    }
}

fn daten(antwort: Antwort, verlangt: u32) -> Result<Option<Vec<u8>>, Fernfehler> {
    match antwort {
        Antwort::Data(daten) if daten.len() > verlangt as usize => Err(Fernfehler::Protokoll(
            format!("DATA mit {} Bytes auf {verlangt} verlangte", daten.len()),
        )),
        Antwort::Data(daten) => Ok(Some(daten)),
        Antwort::Status {
            code: status::EOF, ..
        } => Ok(None),
        andere => Err(fehlschlag(andere)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ein Strom von 10 MiB ohne Zeilenende haelt hoechstens die Grenze einer
    /// Zeile, und die Zeile danach kommt als letzte an (Befund
    /// `261017-0004-die-fehlerausgabe-von-ssh-wird-ohne-laengengrenze-je-zeile-gelesen`).
    ///
    /// Ohne die Grenze hielt der Leser die ganze erste Zeile, 10 MiB.
    #[test]
    fn eine_endlose_fehlerzeile_wird_gekuerzt_und_die_naechste_kommt_an() {
        let mut strom = vec![b'x'; 10 * 1024 * 1024];
        strom.extend_from_slice(b"\nende\n");
        let zeilen: Fehlerzeilen = Arc::new(Mutex::new(VecDeque::new()));
        fehlerausgabe_lesen(strom.as_slice(), &zeilen);
        let gehalten = sperren(&zeilen);
        assert_eq!(
            gehalten.len(),
            2,
            "{:?}",
            gehalten.iter().map(String::len).collect::<Vec<_>>()
        );
        assert_eq!(gehalten.front().map(String::len), Some(FEHLERZEILENLAENGE));
        assert_eq!(gehalten.back().map(String::as_str), Some("ende"));
    }

    /// Gehalten werden die letzten [`FEHLERZEILEN`] Zeilen, und eine letzte
    /// ohne Zeilenende zaehlt mit.
    #[test]
    fn die_letzten_zeilen_bleiben_auch_ohne_zeilenende_am_schluss() {
        let strom: String = (1..=12).map(|zahl| format!("zeile {zahl}\n\n")).collect();
        let strom = format!("{strom}rest");
        let zeilen: Fehlerzeilen = Arc::new(Mutex::new(VecDeque::new()));
        fehlerausgabe_lesen(strom.as_bytes(), &zeilen);
        let gehalten: Vec<String> = sperren(&zeilen).iter().cloned().collect();
        let mut erwartet: Vec<String> = (6..=12).map(|zahl| format!("zeile {zahl}")).collect();
        erwartet.push("rest".to_owned());
        assert_eq!(gehalten, erwartet);
    }
}
