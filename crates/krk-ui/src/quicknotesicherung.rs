//! Die Sicherung der Quicknote in `scratch.txt`, ohne AppKit (Schritt 3 des
//! Plans `261031-1034-plan-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md`).
//!
//! ```text
//!  Editorbereich (Hauptfaden)                 Faden krk-quicknote
//!  ───────────────────────────                ───────────────────
//!  Sicherung::starten ── Lesen ─────────────> stat, Stempel, lesen
//!                                               │
//!  einziehen() <── Gelesen / Fehlt / ─────────┘
//!                  NichtGelesen
//!  schreiben(text) ── Schreiben ────────────> juengsten nehmen, Stempel
//!                                             vergleichen, sichern
//!  einziehen() <── Gesichert / OrdnerFehlt / ─┘
//!                  FremdGeaendert / Gescheitert
//!  abschliessen(2 s) ── Abschliessen ───────> Quittung nach allem davor
//! ```
//!
//! **Diese Datei ist der eine Schreiber von `scratch.txt`.** Den Pfad bekommt
//! sie hereingereicht, gebaut ueber `Heimordner::quicknotedatei`; die Probe
//! `scratch_txt_hat_genau_einen_schreiber` im Pruefmodul haelt das, soweit eine
//! Nadel es kann.
//!
//! # Erst gelesen, dann geschrieben
//!
//! [`Sicherung::starten`] gibt dem Faden als ersten Auftrag `Lesen`, und
//! derselbe Faden bedient danach die Schreibauftraege; dass nichts vor dem
//! Lesen geschrieben wird, ist damit eine Eigenschaft der Reihenfolge. **Dazu
//! nimmt [`Sicherung::schreiben`] keinen Auftrag an, bevor [`Sicherung::einziehen`]
//! die Antwort auf das Lesen abgeholt hat**: sonst koennte ein Text, der vor dem
//! Einsetzen des gelesenen Inhalts entstand (der leere Puffer), die Datei
//! ueberschreiben, obwohl der Faden sie zuerst gelesen hat.
//!
//! # Wann nicht mehr gesichert wird
//!
//! - **Von aussen geaendert** ([`Sicherungsmeldung::FremdGeaendert`]): vor
//!   jedem Schreiben vergleicht der Faden den Stempel der Datei mit dem nach
//!   dem Lesen oder dem letzten eigenen Schreiben; weicht er ab, oder ist die
//!   Datei da, wo sie fehlen sollte, schreibt er nicht und sperrt sich bis zum
//!   Ende des Prozesses. Die Datei bleibt unberuehrt.
//! - **Nicht lesbar** ([`Sicherungsmeldung::NichtGelesen`]): zu gross, kein
//!   UTF-8, keine gewoehnliche Datei, ein Fehler. Die Quicknote oeffnet leer,
//!   und der Faden sperrt sich ebenso.
//! - **Der Notizordner fehlt** ([`Sicherungsmeldung::OrdnerFehlt`]): das sperrt
//!   nicht. Geschrieben wird nichts und kein Ordner angelegt, denn allein F2
//!   legt ihn an und entscheidet dabei ueber die Uebernahme der alten Zettel;
//!   die naechste Aenderung versucht es wieder. Ohne Heimordner (kein
//!   Benutzerverzeichnis) gilt dasselbe.
//!
//! Jede Meldung kommt einmal je Uebergang und nicht je Auftrag: eine
//! gesperrte Sicherung schweigt, ein fehlender Ordner meldet sich beim ersten
//! vergeblichen Versuch, ein gescheitertes Schreiben beim ersten in Folge.
//!
//! # Wie geschrieben wird
//!
//! Ueber `datei::sichern` mit `Zeilenenden::WieGelesen` und ohne Marke, also
//! Byte fuer Byte und ohne angehaengtes `\n`, ueber `ablage::atomar`. **Fehlt
//! die Datei, legt der Faden sie zuerst exklusiv mit den Rechten `0600` an**;
//! `atomar` uebertraegt die Rechte einer bestehenden Datei auf jede spaetere
//! Fassung. Die Quicknote nimmt eingefuegten Text auch aus `secrets.txt` an,
//! und eine fuer jedes Konto lesbare Datei waere ein Leck. Scheitert das
//! Anlegen an `EEXIST`, ist die Datei von anderer Hand entstanden, und es gilt
//! die Sperre.
//!
//! Warten tut allein [`Sicherung::abschliessen`], beim Beenden und mit einer
//! Hoechstzeit; jeder andere Weg ist auf dem Hauptfaden ohne Systemaufruf.

use std::fs::OpenOptions;
use std::io::{self, ErrorKind};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender, SyncSender, TryRecvError};
use std::thread;
use std::time::Duration;

use krk_core::text::datei::{self, Dateiform, Lesehindernis, Platzhalterregel, Zeilenenden};

use crate::editormodell::Stempel;

/// Die Form, in der `scratch.txt` geschrieben wird: der Puffer Byte fuer Byte.
const FORM: Dateiform = Dateiform {
    marke: false,
    zeilenenden: Zeilenenden::WieGelesen,
};

/// Die Rechte einer neu angelegten `scratch.txt`: allein das eigene Konto.
const RECHTE: u32 = 0o600;

/// Was der Faden meldet. **Vollstaendig und ohne Auffangzweig** zu behandeln.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sicherungsmeldung {
    /// `scratch.txt` war lesbar; der Text in gehaltener Form.
    Gelesen(String),
    /// `scratch.txt` fehlt (oder es gibt keinen Heimordner): ein leerer Puffer.
    Fehlt,
    /// `scratch.txt` liess sich nicht lesen; die Sicherung ist gesperrt.
    NichtGelesen(Unlesbar),
    /// Der juengste Text steht auf der Platte.
    Gesichert,
    /// Der Notizordner fehlt; nichts geschrieben, der naechste Auftrag
    /// versucht es wieder.
    OrdnerFehlt,
    /// Die Datei wurde ausserhalb der Quicknote geaendert; die Sicherung ist
    /// gesperrt.
    FremdGeaendert,
    /// Das Schreiben ist an etwas anderem gescheitert; der Grund vom System.
    Gescheitert(String),
}

/// Warum `scratch.txt` sich nicht lesen liess.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unlesbar {
    /// Ein Grund der Leseschleuse, vollstaendig aus `Lesehindernis`.
    Hindernis(Lesehindernis),
    /// Gelesen, aber kein gueltiges UTF-8.
    KeinText,
}

/// Wo die Sicherung steht, so wie der Hauptfaden es aus den abgeholten
/// Meldungen weiss.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sicherungsstand {
    /// Die Antwort auf das Lesen ist noch nicht abgeholt.
    Ungelesen,
    /// Gelesen oder als fehlend erkannt; Aenderungen werden gesichert.
    Bereit,
    /// Der Notizordner fehlte beim letzten Versuch; Aenderungen werden weiter
    /// versucht.
    OrdnerFehlt,
    /// Bis zum Ende des Prozesses wird nicht mehr gesichert.
    Gesperrt,
}

impl Sicherungsstand {
    /// Ob eine Aenderung einen Schreibauftrag bekommen soll.
    #[must_use]
    pub fn nimmt_auftraege(self) -> bool {
        match self {
            Sicherungsstand::Bereit | Sicherungsstand::OrdnerFehlt => true,
            Sicherungsstand::Ungelesen | Sicherungsstand::Gesperrt => false,
        }
    }
}

/// Ein Auftrag an den Faden.
enum Auftrag {
    Lesen,
    Schreiben(String),
    /// Quittiert, sobald alles davor erledigt ist.
    Abschliessen(SyncSender<()>),
}

/// Eine Antwort des Fadens: eine Meldung oder keine, und wie viele
/// Schreibauftraege mit ihr erledigt sind (zusammengefasste eingerechnet).
struct Antwort {
    meldung: Option<Sicherungsmeldung>,
    erledigt: usize,
}

/// Die Sicherung der Quicknote: die Kanaele zum Faden `krk-quicknote` und
/// was der Hauptfaden aus seinen Antworten weiss.
pub struct Sicherung {
    auftraege: Sender<Auftrag>,
    antworten: Receiver<Antwort>,
    stand: std::cell::Cell<Sicherungsstand>,
    /// Gesendete Schreibauftraege, deren Antwort noch nicht abgeholt ist.
    unterwegs: std::cell::Cell<usize>,
}

impl Sicherung {
    /// Startet den Faden fuer `pfad` und gibt ihm als ersten Auftrag `Lesen`.
    ///
    /// `pfad` ist `None`, wenn es keinen Heimordner gibt; dann meldet das
    /// Lesen [`Sicherungsmeldung::Fehlt`] und jedes Schreiben
    /// [`Sicherungsmeldung::OrdnerFehlt`]. `grenze` ist die Grenze des
    /// Puffers, `QUICKNOTEGRENZE`.
    pub fn starten(pfad: Option<PathBuf>, grenze: u64) -> io::Result<Self> {
        let (auftraege, eingang) = mpsc::channel();
        let (ausgang, antworten) = mpsc::channel();
        thread::Builder::new()
            .name("krk-quicknote".to_owned())
            .spawn(move || faden(pfad.as_deref(), grenze, &eingang, &ausgang))?;
        let sicherung = Self::aus_kanaelen(auftraege, antworten);
        // Der Faden lebt, also nimmt der Kanal den Auftrag an; scheitert es
        // trotzdem, bleibt die Sicherung ungelesen und nimmt nichts an.
        let _ = sicherung.auftraege.send(Auftrag::Lesen);
        Ok(sicherung)
    }

    fn aus_kanaelen(auftraege: Sender<Auftrag>, antworten: Receiver<Antwort>) -> Self {
        Self {
            auftraege,
            antworten,
            stand: std::cell::Cell::new(Sicherungsstand::Ungelesen),
            unterwegs: std::cell::Cell::new(0),
        }
    }

    /// Wo die Sicherung nach den abgeholten Meldungen steht.
    #[must_use]
    pub fn stand(&self) -> Sicherungsstand {
        self.stand.get()
    }

    /// Ob ein Schreibauftrag gesendet und seine Antwort noch nicht abgeholt
    /// ist; solange das gilt, braucht der Editorbereich seinen Takt.
    #[must_use]
    pub fn unterwegs(&self) -> bool {
        self.unterwegs.get() > 0
    }

    /// Gibt dem Faden den ganzen Text zum Sichern.
    ///
    /// Antwortet, ob der Auftrag angenommen ist: nein, solange die Antwort auf
    /// das Lesen nicht abgeholt ist, wenn die Sicherung gesperrt ist und wenn
    /// der Faden nicht mehr lebt.
    #[must_use = "ein abgewiesener Auftrag heisst, dass dieser Stand nicht auf die Platte kommt"]
    pub fn schreiben(&self, text: String) -> bool {
        if !self.stand.get().nimmt_auftraege() {
            return false;
        }
        if self.auftraege.send(Auftrag::Schreiben(text)).is_err() {
            return false;
        }
        self.unterwegs.set(self.unterwegs.get() + 1);
        true
    }

    /// Holt die Meldungen des Fadens ab, ohne zu warten, und fuehrt den Stand
    /// nach.
    #[must_use = "die Meldungen sind die Auskunft an den Nutzer und das gelesene Ergebnis"]
    pub fn einziehen(&self) -> Vec<Sicherungsmeldung> {
        let mut meldungen = Vec::new();
        loop {
            match self.antworten.try_recv() {
                Ok(antwort) => {
                    self.unterwegs
                        .set(self.unterwegs.get().saturating_sub(antwort.erledigt));
                    if let Some(meldung) = antwort.meldung {
                        self.stand.set(stand_nach(self.stand.get(), &meldung));
                        meldungen.push(meldung);
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    // Ein Faden, der nicht mehr antwortet, sichert nichts mehr;
                    // was unterwegs war, kommt nicht mehr an.
                    self.unterwegs.set(0);
                    self.stand.set(Sicherungsstand::Gesperrt);
                    break;
                }
            }
        }
        meldungen
    }

    /// Wartet hoechstens `warten`, bis der Faden jeden Auftrag davor erledigt
    /// hat; fuer das Beenden. Antwortet, ob die Quittung kam.
    #[must_use = "ohne Quittung ist der letzte Stand vielleicht nicht auf der Platte"]
    pub fn abschliessen(&self, warten: Duration) -> bool {
        let (quittung, eingang) = mpsc::sync_channel(1);
        if self
            .auftraege
            .send(Auftrag::Abschliessen(quittung))
            .is_err()
        {
            return false;
        }
        eingang.recv_timeout(warten).is_ok()
    }
}

/// Der Stand, den eine Meldung herstellt; vollstaendig und ohne Auffangzweig.
fn stand_nach(vorher: Sicherungsstand, meldung: &Sicherungsmeldung) -> Sicherungsstand {
    match meldung {
        Sicherungsmeldung::Gelesen(_) | Sicherungsmeldung::Fehlt | Sicherungsmeldung::Gesichert => {
            Sicherungsstand::Bereit
        }
        Sicherungsmeldung::OrdnerFehlt => Sicherungsstand::OrdnerFehlt,
        Sicherungsmeldung::NichtGelesen(_) | Sicherungsmeldung::FremdGeaendert => {
            Sicherungsstand::Gesperrt
        }
        // Ein gescheitertes Schreiben laesst die Datei, wie sie war; der
        // naechste Auftrag versucht es wieder.
        Sicherungsmeldung::Gescheitert(_) => vorher,
    }
}

/// Was der Faden ueber die Datei weiss.
enum Zustand {
    Ungelesen,
    /// `stempel` ist der Stempel nach dem Lesen oder dem letzten eigenen
    /// Schreiben, `None` fuer eine Datei, die fehlt.
    Bereit {
        stempel: Option<Stempel>,
        ordner_fehlte: bool,
        gescheitert: bool,
    },
    Gesperrt,
}

/// Der Rumpf des Fadens: bedient die Auftraege in ihrer Reihenfolge und fasst
/// wartende Schreibauftraege zum juengsten zusammen.
fn faden(pfad: Option<&Path>, grenze: u64, eingang: &Receiver<Auftrag>, ausgang: &Sender<Antwort>) {
    let mut zustand = Zustand::Ungelesen;
    let mut naechster: Option<Auftrag> = None;
    loop {
        let auftrag = match naechster.take() {
            Some(auftrag) => auftrag,
            None => match eingang.recv() {
                Ok(auftrag) => auftrag,
                Err(_) => return,
            },
        };
        let antwort = match auftrag {
            Auftrag::Lesen => Antwort {
                meldung: lesen(pfad, grenze, &mut zustand),
                erledigt: 0,
            },
            Auftrag::Schreiben(mut text) => {
                let mut erledigt = 1;
                loop {
                    match eingang.try_recv() {
                        Ok(Auftrag::Schreiben(juenger)) => {
                            text = juenger;
                            erledigt += 1;
                        }
                        Ok(anderer) => {
                            naechster = Some(anderer);
                            break;
                        }
                        Err(_) => break,
                    }
                }
                Antwort {
                    meldung: schreiben(pfad, &text, &mut zustand),
                    erledigt,
                }
            }
            Auftrag::Abschliessen(quittung) => {
                let _ = quittung.send(());
                continue;
            }
        };
        if ausgang.send(antwort).is_err() {
            return;
        }
    }
}

/// Liest `scratch.txt` (E3). Ein zweiter Leseauftrag liest nichts.
fn lesen(pfad: Option<&Path>, grenze: u64, zustand: &mut Zustand) -> Option<Sicherungsmeldung> {
    if !matches!(zustand, Zustand::Ungelesen) {
        return None;
    }
    let fehlt = |zustand: &mut Zustand| {
        *zustand = Zustand::Bereit {
            stempel: None,
            ordner_fehlte: false,
            gescheitert: false,
        };
        Some(Sicherungsmeldung::Fehlt)
    };
    let Some(pfad) = pfad else {
        return fehlt(zustand);
    };
    if std::fs::metadata(pfad).is_err_and(|fehler| fehler.kind() == ErrorKind::NotFound) {
        return fehlt(zustand);
    }
    // Der Stempel vor dem Lesen, aus dem Grund an `Ladevorgang::starten`: eine
    // Aenderung waehrend des Lesens laesst ihn aelter sein als den Inhalt und
    // faellt beim ersten Schreiben auf.
    let stempel = Stempel::von_pfad(pfad);
    let gelesen = datei::bis_zur_grenze_lesen(pfad, grenze, Platzhalterregel::Laden, &|| true)
        .map_err(Unlesbar::Hindernis)
        .and_then(|bytes| datei::einlesen(bytes).ok_or(Unlesbar::KeinText));
    match gelesen {
        Ok(text) => {
            *zustand = Zustand::Bereit {
                stempel,
                ordner_fehlte: false,
                gescheitert: false,
            };
            Some(Sicherungsmeldung::Gelesen(text))
        }
        Err(grund) => {
            *zustand = Zustand::Gesperrt;
            Some(Sicherungsmeldung::NichtGelesen(grund))
        }
    }
}

/// Sichert `text` (E5, E6) und antwortet mit der Meldung des Uebergangs.
fn schreiben(pfad: Option<&Path>, text: &str, zustand: &mut Zustand) -> Option<Sicherungsmeldung> {
    let Zustand::Bereit {
        stempel,
        ordner_fehlte,
        gescheitert,
    } = zustand
    else {
        return None;
    };
    let ausgang = match pfad {
        Some(pfad) => sichern(pfad, text, stempel),
        None => Ausgang::OrdnerFehlt,
    };
    match ausgang {
        Ausgang::Gesichert => {
            *ordner_fehlte = false;
            *gescheitert = false;
            Some(Sicherungsmeldung::Gesichert)
        }
        Ausgang::OrdnerFehlt => {
            let neu = !*ordner_fehlte;
            *ordner_fehlte = true;
            neu.then_some(Sicherungsmeldung::OrdnerFehlt)
        }
        Ausgang::FremdGeaendert => {
            *zustand = Zustand::Gesperrt;
            Some(Sicherungsmeldung::FremdGeaendert)
        }
        Ausgang::Gescheitert(fehler) => {
            let neu = !*gescheitert;
            *gescheitert = true;
            neu.then(|| Sicherungsmeldung::Gescheitert(fehler.to_string()))
        }
    }
}

/// Der Ausgang eines Schreibversuchs.
enum Ausgang {
    Gesichert,
    OrdnerFehlt,
    FremdGeaendert,
    Gescheitert(io::Error),
}

/// Ein Schreibversuch an `pfad`: Stempel vergleichen, wenn noetig exklusiv
/// anlegen, sichern, neuen Stempel merken.
fn sichern(pfad: &Path, text: &str, stempel: &mut Option<Stempel>) -> Ausgang {
    let jetzt = Stempel::von_pfad(pfad);
    match (stempel.as_ref(), jetzt) {
        (Some(bekannt), Some(jetzt)) if bekannt.gleicht(&jetzt) => {}
        (Some(_), Some(_)) => return Ausgang::FremdGeaendert,
        (Some(_), None) => {
            // Die bekannte Datei ist fort: weil der Ordner fort ist, oder weil
            // eine andere Hand sie entfernt hat.
            return if ordner_fehlt(pfad) {
                Ausgang::OrdnerFehlt
            } else {
                Ausgang::FremdGeaendert
            };
        }
        (None, Some(_)) => return Ausgang::FremdGeaendert,
        (None, None) => match exklusiv_anlegen(pfad) {
            Ok(()) => *stempel = Stempel::von_pfad(pfad),
            Err(fehler) if fehler.kind() == ErrorKind::AlreadyExists => {
                return Ausgang::FremdGeaendert;
            }
            Err(fehler) if fehler.kind() == ErrorKind::NotFound => return Ausgang::OrdnerFehlt,
            Err(fehler) => return Ausgang::Gescheitert(fehler),
        },
    }
    if let Err(fehler) = datei::sichern(pfad, text, FORM) {
        return if fehler.kind() == ErrorKind::NotFound {
            Ausgang::OrdnerFehlt
        } else {
            Ausgang::Gescheitert(fehler)
        };
    }
    *stempel = Stempel::von_pfad(pfad);
    Ausgang::Gesichert
}

/// Legt `pfad` leer und exklusiv mit [`RECHTE`] an; einen Ordner legt dieser
/// Weg nie an.
fn exklusiv_anlegen(pfad: &Path) -> io::Result<()> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(RECHTE)
        .open(pfad)
        .map(drop)
}

/// Ob der Ordner von `pfad` fehlt.
fn ordner_fehlt(pfad: &Path) -> bool {
    pfad.parent().is_none_or(|ordner| {
        std::fs::metadata(ordner).is_err_and(|fehler| fehler.kind() == ErrorKind::NotFound)
    })
}

#[cfg(test)]
mod proben {
    use std::os::unix::fs::PermissionsExt;
    use std::time::Instant;

    use super::*;
    use crate::pruefordner::Pruefordner;
    use crate::quellbaum::{codezeilen, quelldateien};

    const GRENZE: u64 = 1024;

    /// Wartet, bis der Faden mindestens eine Meldung geschickt hat, und holt
    /// alle bis dahin eingetroffenen ab.
    fn abholen(sicherung: &Sicherung) -> Vec<Sicherungsmeldung> {
        let beginn = Instant::now();
        loop {
            let meldungen = sicherung.einziehen();
            if !meldungen.is_empty() {
                return meldungen;
            }
            assert!(
                beginn.elapsed() < Duration::from_secs(5),
                "der Faden antwortet nicht"
            );
            thread::sleep(Duration::from_millis(2));
        }
    }

    /// Schreibt `text` und wartet auf die Quittung des Fadens.
    fn sichern_und_warten(sicherung: &Sicherung, text: &str) -> Vec<Sicherungsmeldung> {
        assert!(sicherung.schreiben(text.to_owned()), "Auftrag abgewiesen");
        assert!(sicherung.abschliessen(Duration::from_secs(5)));
        sicherung.einziehen()
    }

    fn gestartet(pfad: &Path) -> Sicherung {
        Sicherung::starten(Some(pfad.to_path_buf()), GRENZE).expect("Faden startet")
    }

    fn rechte(pfad: &Path) -> u32 {
        std::fs::metadata(pfad)
            .expect("Datei steht")
            .permissions()
            .mode()
            & 0o777
    }

    #[test]
    fn eine_fehlende_datei_ergibt_einen_leeren_puffer_und_entsteht_beim_ersten_sichern_mit_0600() {
        let ordner = Pruefordner::neu("qn-fehlt");
        let pfad = ordner.unter("scratch.txt");
        let sicherung = gestartet(&pfad);
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        assert!(!pfad.exists(), "das Lesen legt nichts an");

        assert_eq!(
            sichern_und_warten(&sicherung, "erste Notiz"),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"erste Notiz");
        assert_eq!(rechte(&pfad), 0o600);
        assert!(!sicherung.unterwegs());
    }

    #[test]
    fn ein_gelesener_text_kommt_byte_fuer_byte_zurueck_ohne_angehaengten_umbruch() {
        let ordner = Pruefordner::neu("qn-rundlauf");
        let pfad = ordner.unter("scratch.txt");
        let text = "Gr\u{fc}\u{df}e\nzweite Zeile ohne Schluss";

        let erste = gestartet(&pfad);
        assert_eq!(abholen(&erste), vec![Sicherungsmeldung::Fehlt]);
        assert_eq!(
            sichern_und_warten(&erste, text),
            vec![Sicherungsmeldung::Gesichert]
        );
        drop(erste);
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), text.as_bytes());

        // Ein zweiter Wert wie nach einem Neustart.
        let zweite = gestartet(&pfad);
        assert_eq!(
            abholen(&zweite),
            vec![Sicherungsmeldung::Gelesen(text.to_owned())]
        );
        assert_eq!(
            sichern_und_warten(&zweite, text),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), text.as_bytes());
    }

    #[test]
    fn vor_der_antwort_auf_das_lesen_wird_nichts_geschrieben() {
        let ordner = Pruefordner::neu("qn-reihenfolge");
        let pfad = ordner.datei("scratch.txt", "vorhanden");
        let sicherung = gestartet(&pfad);
        // Die Antwort auf das Lesen ist nicht abgeholt: kein Auftrag.
        assert_eq!(sicherung.stand(), Sicherungsstand::Ungelesen);
        assert!(!sicherung.schreiben(String::new()));
        assert!(sicherung.abschliessen(Duration::from_secs(5)));
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"vorhanden");
        assert_eq!(
            sicherung.einziehen(),
            vec![Sicherungsmeldung::Gelesen("vorhanden".to_owned())]
        );
        assert_eq!(sicherung.stand(), Sicherungsstand::Bereit);
    }

    #[test]
    fn wartende_auftraege_schreiben_nur_den_juengsten() {
        let ordner = Pruefordner::neu("qn-juengster");
        let pfad = ordner.unter("scratch.txt");
        let (auftraege, eingang) = mpsc::channel();
        let (ausgang, antworten) = mpsc::channel();
        let sicherung = Sicherung::aus_kanaelen(auftraege, antworten);
        // Die Auftraege stehen schon an, bevor der Faden laeuft.
        let _ = sicherung.auftraege.send(Auftrag::Lesen);
        let fuer_faden = pfad.clone();
        let faden_griff =
            thread::spawn(move || faden(Some(fuer_faden.as_path()), GRENZE, &eingang, &ausgang));
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);

        // Ob der Faden zwischen den drei Auftraegen aufwacht, entscheidet der
        // Planer; geprueft wird hier deshalb, dass am Ende der juengste Text
        // steht und jeder Auftrag erledigt ist. Das Zusammenfassen selbst
        // haelt die Probe darunter ohne Planer.
        for text in ["eins", "zwei", "drei"] {
            assert!(sicherung.schreiben(text.to_owned()));
        }
        assert!(sicherung.abschliessen(Duration::from_secs(5)));
        let meldungen = sicherung.einziehen();
        assert!(!meldungen.is_empty());
        assert!(
            meldungen
                .iter()
                .all(|meldung| *meldung == Sicherungsmeldung::Gesichert)
        );
        assert!(!sicherung.unterwegs(), "jeder Auftrag ist erledigt");
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"drei");
        drop(sicherung);
        faden_griff.join().expect("der Faden endet ohne Panik");
    }

    #[test]
    fn wartende_auftraege_werden_zu_einem_zusammengefasst() {
        // Dieselbe Eigenschaft ohne Planer: die Auftraege stehen im Kanal,
        // bevor der Faden ueberhaupt startet.
        let ordner = Pruefordner::neu("qn-zusammen");
        let pfad = ordner.unter("scratch.txt");
        let (auftraege, eingang) = mpsc::channel();
        let (ausgang, antworten) = mpsc::channel();
        for auftrag in [
            Auftrag::Lesen,
            Auftrag::Schreiben("eins".to_owned()),
            Auftrag::Schreiben("zwei".to_owned()),
            Auftrag::Schreiben("drei".to_owned()),
        ] {
            let _ = auftraege.send(auftrag);
        }
        drop(auftraege);
        faden(Some(pfad.as_path()), GRENZE, &eingang, &ausgang);
        let gesammelt: Vec<(Option<Sicherungsmeldung>, usize)> = antworten
            .try_iter()
            .map(|antwort| (antwort.meldung, antwort.erledigt))
            .collect();
        assert_eq!(
            gesammelt,
            vec![
                (Some(Sicherungsmeldung::Fehlt), 0),
                (Some(Sicherungsmeldung::Gesichert), 3),
            ]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"drei");
    }

    #[test]
    fn ein_fehlender_ordner_schreibt_nichts_legt_nichts_an_und_sichert_sobald_er_steht() {
        let ordner = Pruefordner::neu("qn-ordner");
        let heim = ordner.unter("krkhome");
        let pfad = heim.join("scratch.txt");
        let sicherung = gestartet(&pfad);
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);

        assert_eq!(
            sichern_und_warten(&sicherung, "eins"),
            vec![Sicherungsmeldung::OrdnerFehlt]
        );
        assert_eq!(sicherung.stand(), Sicherungsstand::OrdnerFehlt);
        // Einmal je Uebergang: der zweite vergebliche Versuch schweigt.
        assert_eq!(sichern_und_warten(&sicherung, "zwei"), vec![]);
        assert!(!heim.exists(), "die Sicherung legt keinen Ordner an");

        std::fs::create_dir(&heim).expect("Ordner wie nach F2");
        assert_eq!(
            sichern_und_warten(&sicherung, "drei"),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"drei");
        assert_eq!(sicherung.stand(), Sicherungsstand::Bereit);
    }

    #[test]
    fn ohne_heimordner_fehlt_die_datei_und_der_ordner() {
        let sicherung = Sicherung::starten(None, GRENZE).expect("Faden startet");
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        assert_eq!(
            sichern_und_warten(&sicherung, "eins"),
            vec![Sicherungsmeldung::OrdnerFehlt]
        );
    }

    #[test]
    fn eine_aenderung_von_aussen_sperrt_und_laesst_die_datei_unberuehrt() {
        let ordner = Pruefordner::neu("qn-fremd");
        let pfad = ordner.datei("scratch.txt", "vorher");
        let sicherung = gestartet(&pfad);
        assert_eq!(
            abholen(&sicherung),
            vec![Sicherungsmeldung::Gelesen("vorher".to_owned())]
        );
        std::fs::write(&pfad, "von der zweiten Instanz").expect("geschrieben");

        assert_eq!(
            sichern_und_warten(&sicherung, "aus der Quicknote"),
            vec![Sicherungsmeldung::FremdGeaendert]
        );
        assert_eq!(sicherung.stand(), Sicherungsstand::Gesperrt);
        assert!(!sicherung.schreiben("noch einmal".to_owned()));
        assert_eq!(
            std::fs::read(&pfad).expect("gelesen"),
            b"von der zweiten Instanz"
        );
    }

    #[test]
    fn eine_datei_die_waehrend_der_fehlanzeige_entsteht_wird_nicht_ueberschrieben() {
        let ordner = Pruefordner::neu("qn-entsteht");
        let pfad = ordner.unter("scratch.txt");
        let sicherung = gestartet(&pfad);
        assert_eq!(abholen(&sicherung), vec![Sicherungsmeldung::Fehlt]);
        std::fs::write(&pfad, "fremd").expect("geschrieben");

        assert_eq!(
            sichern_und_warten(&sicherung, "eigen"),
            vec![Sicherungsmeldung::FremdGeaendert]
        );
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"fremd");
    }

    #[test]
    fn das_exklusive_anlegen_weist_eine_vorhandene_datei_ab() {
        let ordner = Pruefordner::neu("qn-exklusiv");
        let pfad = ordner.datei("scratch.txt", "fremd");
        let mut stempel = None;
        // Der Wettlauf zwischen Stempel und Anlegen, von Hand hergestellt: der
        // Faden glaubt die Datei fehlend, sie steht aber.
        assert!(matches!(
            exklusiv_anlegen(&pfad),
            Err(fehler) if fehler.kind() == ErrorKind::AlreadyExists
        ));
        assert!(matches!(
            sichern(&pfad, "eigen", &mut stempel),
            Ausgang::FremdGeaendert
        ));
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"fremd");
    }

    #[test]
    fn eine_zu_grosse_oder_nicht_lesbare_datei_sperrt_und_bleibt_byte_fuer_byte() {
        let ordner = Pruefordner::neu("qn-unlesbar");
        let gross = vec![b'x'; usize::try_from(GRENZE).expect("passt") + 1];
        let kein_text = vec![0xff, 0xfe, b'a'];
        for (name, inhalt, grund) in [
            (
                "gross.txt",
                gross,
                Unlesbar::Hindernis(Lesehindernis::ZuGross),
            ),
            ("binaer.txt", kein_text, Unlesbar::KeinText),
        ] {
            let pfad = ordner.datei(name, &inhalt);
            let sicherung = gestartet(&pfad);
            assert_eq!(
                abholen(&sicherung),
                vec![Sicherungsmeldung::NichtGelesen(grund)],
                "{name}"
            );
            assert_eq!(sicherung.stand(), Sicherungsstand::Gesperrt);
            assert!(!sicherung.schreiben("neu".to_owned()), "{name}");
            assert!(sicherung.abschliessen(Duration::from_secs(5)));
            assert_eq!(std::fs::read(&pfad).expect("gelesen"), inhalt, "{name}");
        }

        let roehre = ordner.roehre("scratch.txt");
        let sicherung = gestartet(&roehre);
        assert_eq!(
            abholen(&sicherung),
            vec![Sicherungsmeldung::NichtGelesen(Unlesbar::Hindernis(
                Lesehindernis::KeineDatei
            ))]
        );
    }

    #[test]
    fn die_rechte_einer_vorhandenen_datei_bleiben() {
        let ordner = Pruefordner::neu("qn-rechte");
        let pfad = ordner.datei("scratch.txt", "vorher");
        std::fs::set_permissions(&pfad, std::fs::Permissions::from_mode(0o640))
            .expect("Rechte gesetzt");
        let sicherung = gestartet(&pfad);
        let _ = abholen(&sicherung);
        assert_eq!(
            sichern_und_warten(&sicherung, "nachher"),
            vec![Sicherungsmeldung::Gesichert]
        );
        assert_eq!(rechte(&pfad), 0o640);
        assert_eq!(std::fs::read(&pfad).expect("gelesen"), b"nachher");
    }

    #[test]
    fn abschliessen_wartet_hoechstens_die_genannte_zeit() {
        // Ein Faden, der nie antwortet: der Empfaenger lebt, bedient aber
        // nichts.
        let (auftraege, _eingang_haengt) = mpsc::channel();
        let (_ausgang, antworten) = mpsc::channel();
        let sicherung = Sicherung::aus_kanaelen(auftraege, antworten);
        let beginn = Instant::now();
        assert!(!sicherung.abschliessen(Duration::from_millis(100)));
        let gewartet = beginn.elapsed();
        assert!(gewartet >= Duration::from_millis(100), "{gewartet:?}");
        assert!(gewartet < Duration::from_secs(2), "{gewartet:?}");

        // Ein lebender Faden quittiert sofort.
        let ordner = Pruefordner::neu("qn-abschluss");
        let lebend = gestartet(&ordner.unter("scratch.txt"));
        let beginn = Instant::now();
        assert!(lebend.abschliessen(Duration::from_secs(2)));
        assert!(beginn.elapsed() < Duration::from_secs(1));
    }

    /// `scratch.txt` hat genau einen Schreiber, und den Pfad baut allein der
    /// Kern.
    ///
    /// Gehalten am Quelltext von `crates/krk-ui/src`:
    ///
    /// - das exklusive Anlegen (`create_new`) steht in dieser Kiste allein in
    ///   dieser Datei;
    /// - der Name `QUICKNOTEDATEI` steht in keiner Codezeile dieser Kiste, der
    ///   Pfad entsteht also allein ueber `Heimordner::quicknotedatei`;
    /// - `quicknotedatei(` wird in dieser Kiste genau in `appkit/editor.rs`
    ///   gerufen, der Stelle, die die Sicherung startet und ihr den Pfad
    ///   reicht, und in keiner Datei sonst;
    /// - diese Datei ruft `datei::sichern`.
    ///
    /// **Was sie nicht sieht:** einen Pfad auf `scratch.txt`, der anders gebaut
    /// wird (ein Literal `"scratch.txt"`, ein `join` an den Notizordner), und
    /// einen Schreiber, der den Pfad aus `quicknotedatei` bekommt und ihn an
    /// `datei::sichern` oder `ablage::atomar` in einer anderen Datei reicht.
    /// Eine Nadel entscheidet nicht, wohin ein Pfad fliesst.
    #[test]
    fn scratch_txt_hat_genau_einen_schreiber() {
        let diese = "krk-ui/src/quicknotesicherung.rs";
        let erlaubte_rufer = ["krk-ui/src/appkit/editor.rs"];
        let anlegen = concat!("create", "_new(");
        let name = concat!("QUICKNOTE", "DATEI");
        let ruf = concat!("quicknote", "datei(");
        let schreibweg = concat!("datei::", "sichern(");
        let mut diese_gelesen = false;
        let mut rufer: Vec<String> = Vec::new();
        for (datei, inhalt) in quelldateien() {
            if !datei.starts_with("krk-ui/src/") {
                continue;
            }
            let code: Vec<&str> = codezeilen(&inhalt).collect();
            let nennt = |nadel: &str| code.iter().any(|zeile| zeile.contains(nadel));
            if datei == diese {
                diese_gelesen = true;
                assert!(
                    nennt(schreibweg),
                    "{diese} sichert nicht ueber datei::sichern"
                );
                assert!(nennt(anlegen), "{diese} legt nicht exklusiv an");
                continue;
            }
            assert!(!nennt(anlegen), "{datei} legt exklusiv an");
            assert!(!nennt(name), "{datei} nennt den Namen der Quicknote-Datei");
            if nennt(ruf) {
                rufer.push(datei);
            }
        }
        assert!(diese_gelesen, "{diese} steht nicht im Baum");
        assert_eq!(
            rufer, erlaubte_rufer,
            "quicknotedatei wird nicht genau in den erlaubten Dateien gerufen"
        );
    }
}
