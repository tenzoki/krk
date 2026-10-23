//! Das Inhaltsverzeichnis eines Archivs: was darin steht, ohne es zu
//! entpacken, und die Leser fuer tar, tar.gz und gz.
//!
//! Jedes Format liefert seine Eintraege in der Reihenfolge, in der es sie
//! fuehrt, an einen [`Sammler`], und erst [`Sammler::abschliessen`] macht
//! daraus ein [`Inhaltsverzeichnis`]. **Die Regeln ueber alle Eintraege
//! zugleich stehen deshalb an einer Stelle und nicht je Format:**
//!
//! - Ein Name, den [`Innenpfad::aus_archivname`] abweist, kommt nicht hinein
//!   und zaehlt in [`Inhaltsverzeichnis::ausgelassen`].
//! - **Doppelte Namen:** der spaetere gilt, wie beim Entpacken auf die
//!   Platte, und der fruehere zaehlt. Ist ein Name zugleich Ordner und etwas
//!   anderes, gilt der Ordner, gleich in welcher Folge.
//! - **Der Weg ueber eine Verknuepfung:** ein Eintrag, dessen Weg durch einen
//!   Namen fuehrt, der im selben Archiv eine Verknuepfung ist
//!   (`hinaus -> /etc`, dann `hinaus/passwd`), faellt weg und zaehlt. Das ist
//!   der zweite Weg aus einem Zielordner heraus, den `operation::entpacken`
//!   mit `kette_anlegen` versperrt; im Fenster kommt er gar nicht erst vor.
//! - **Ordner ohne eigenen Eintrag** (`a/b/c.txt` ohne `a/` und `a/b/`)
//!   entstehen als erschlossene Ordner ([`Fundstelle::Erschlossen`]) mit dem
//!   Aenderungsdatum der Archivdatei. Eine Datei, deren Name zugleich ein
//!   solcher Ordner sein muesste, faellt weg und zaehlt.
//!
//! Die drei Regeln, die einen Eintrag mit gueltigem Namen weglassen (der
//! fruehere von doppelten Namen, der Weg ueber eine Verknuepfung, die Datei,
//! die einem Ordner weicht), zaehlen ihn zusaetzlich als **verdeckt**
//! ([`Inhaltsverzeichnis::verdeckt`]). Ein Archiv mit einem verdeckten Eintrag
//! schreibt KRK nicht um (`archiv::umbau`, Entscheid
//! `261017-1923-darf-jeder-umbau-die-verdeckten-eintraege-eines-archivs-verwerfen.md`,
//! Moeglichkeit 3): jeder Umbau liesse ihn sonst fallen oder kaeme beim
//! Loeschen des verdeckenden wieder hervor.
//!
//! # Die Obergrenze fuer die Vorkoepfe eines tar
//!
//! Die Kiste `tar` liest einen langen Namen, einen langen Verweis und einen
//! PAX-Kopf ganz in den Speicher, so gross, wie sein Kopf ihn angibt; ein
//! tar.gz von wenigen Kilobyte kann so einen Kopf von einem Gigabyte Nullen
//! tragen. **Jeder Leser eines tar-Stroms geht deshalb ueber
//! `Kopfwache::naechster`**: waehrend die Kiste den naechsten Eintrag
//! sucht, darf sie nach jedem Sprung hoechstens `HOECHSTENS_VORKOPF` und
//! einen Kopf lesen, dann scheitert das Lesen, und das Archiv ist ungueltig
//! ([`Verzeichnisfehler::Ungueltig`]). Das ist dieselbe Grenze, die der Umbau
//! an seinen Vorkoepfen haelt. Die Kiste springt vor jedem Kopf (auch um null
//! Bytes, wenn der Strom `Seek` kann), also faellt jeder Vorkopf einzeln unter
//! die Grenze; der gepackte Strom bekommt dafuer einen Sprung, der vorwaerts
//! liest (`Vorwaerts`). Was die Kiste dabei zuteilt, waechst mit dem
//! Gelesenen und bleibt deshalb unter dem Doppelten der Grenze je Vorkopf.
//! **Was die Wache nicht begrenzt:** die Erweiterungsbloecke einer GNU-Datei
//! mit Luecken, die die Kiste nach dem Kopf ohne Sprung liest, fallen unter
//! dieselbe Zahl, nicht unter eine eigene.
//!
//! # Abbruch
//!
//! Gefragt wird je Eintrag und, fuer die gepackten Stroeme, je MiB entpackter
//! Bytes ([`Abbruchleser`]). Ein abgebrochener Aufbau liefert
//! [`Verzeichnisfehler::Abgebrochen`] und kein halbes Verzeichnis: der
//! [`Sammler`] wird dann verworfen, und nur ein abgeschlossenes Verzeichnis
//! erreicht den Zwischenspeicher des Zugangs.
//!
//! # Was ein Eintrag ueber seine Lesbarkeit sagt
//!
//! [`Lesbarkeit`] haelt fest, was beim Herauskopieren nicht herauskommt
//! (verschluesselt, ein Verfahren, das `deflate-flate2` nicht entpackt, ein
//! harter Verweis oder eine Sonderdatei im tar). Gezeigt wird ein solcher
//! Eintrag trotzdem: er steht im Archiv, und ihn zu verschweigen waere die
//! falsche Auskunft (E3 und E11 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`).

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use flate2::read::MultiGzDecoder;
use tar::EntryType;

use crate::verzeichnis::Typ;

use super::HOECHSTENS_EINTRAEGE;
use super::innenpfad::Innenpfad;
use super::zugang::Dateikennung;

/// Warum kein Inhaltsverzeichnis entstanden ist.
#[derive(Debug)]
pub enum Verzeichnisfehler {
    /// Die Abbruchmarke war gesetzt.
    Abgebrochen,
    /// Die Datei liess sich nicht oeffnen oder lesen; der Grund ist der des
    /// Systems.
    Lesen(io::Error),
    /// Unter dem Namen steht keine gewoehnliche Datei, etwa ein Ordner, eine
    /// Roehre oder ein Geraet.
    KeineDatei,
    /// Die Datei ist kein Archiv des Formats, das ihre Endung verspricht. Der
    /// Grund ist der Wortlaut der Kiste, aus dem Grund, aus dem
    /// `operation::entpacken` ihn weitergibt: er sagt genauer als eine eigene
    /// Formulierung, woran das Lesen gescheitert ist.
    Ungueltig(String),
    /// Das Archiv fuehrt mehr als [`HOECHSTENS_EINTRAEGE`] Eintraege.
    ZuGross,
    /// Ein anderes Programm haelt die Sperre einer SQLite-Datenbank ueber die
    /// Wartezeit hinaus (`SQLITE_BUSY`, `SQLITE_LOCKED`); gemeldet als „wird
    /// gerade geschrieben“ (P8 des Plans
    /// `261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md`).
    Belegt,
}

impl Verzeichnisfehler {
    /// Ordnet einen Fehler beim Lesen eines Stroms zu.
    ///
    /// Ein Fehler mit einer Nummer des Systems ist ein Lesefehler an der Datei;
    /// jeder andere kommt aus dem Zerlegen des Formats und heisst, dass die
    /// Datei kein gueltiges Archiv ist. Eine gesetzte Abbruchmarke geht vor,
    /// denn der [`Abbruchleser`] meldet den Abbruch selbst als Fehler.
    pub(crate) fn aus_strom(fehler: io::Error, abbruch: &AtomicBool) -> Self {
        if abbruch.load(Ordering::Acquire) {
            Verzeichnisfehler::Abgebrochen
        } else if fehler.raw_os_error().is_some() {
            Verzeichnisfehler::Lesen(fehler)
        } else {
            Verzeichnisfehler::Ungueltig(fehler.to_string())
        }
    }
}

/// Wo die Daten eines Eintrags im Archiv stehen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fundstelle {
    /// Der Platz im Zentralverzeichnis eines Zip, wie `ZipArchive::by_index`
    /// ihn nimmt.
    Zip(usize),
    /// Der Versatz des Kopfs im entpackten tar-Strom. Die Eintraege eines
    /// Stroms stehen in dieser Folge, und ein Lesezeiger, der nur vorwaerts
    /// laeuft, erreicht sie in ihr ohne Neustart.
    Tar {
        /// Versatz in Bytes vom Anfang des (entpackten) Stroms.
        versatz: u64,
    },
    /// Der ganze entpackte Strom einer gz-Datei.
    GzGanz,
    /// Ein Ordner ohne eigenen Eintrag, erschlossen aus den Namen darunter.
    /// Er hat keine Daten und keinen Kopf; der Umbau braucht das Kennzeichen,
    /// um ihn stehen zu lassen, wenn sein letzter Eintrag wegfaellt.
    Erschlossen,
    /// Eine Tabelle einer SQLite-Datenbank, mit ihrer Zeilenzahl beim Aufbau
    /// des Verzeichnisses; `None`, wenn das Zaehlen scheiterte, etwa an einer
    /// virtuellen Tabelle, deren Modul die Bibliothek nicht kennt
    /// (`archiv::sqlite`).
    SqliteTabelle {
        /// Die Zeilen nach `SELECT count(*)`, oder `None`.
        zeilen: Option<u64>,
    },
    /// Eine Sicht einer SQLite-Datenbank. Sie wird nicht ausgefuehrt und hat
    /// keine Zeilenzahl.
    SqliteSicht,
}

/// Ob ein Eintrag beim Herauskopieren herauskommt, und wenn nicht, warum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lesbarkeit {
    /// Der Eintrag laesst sich lesen.
    Lesbar,
    /// Der Eintrag ist verschluesselt.
    Verschluesselt,
    /// Der Eintrag ist mit einem Verfahren gepackt, das KRK nicht entpackt;
    /// der Text ist der Name des Verfahrens.
    Verfahren(String),
    /// Ein harter Verweis im tar.
    HarterVerweis,
    /// Eine Geraetedatei, eine benannte Roehre oder ein unbekannter Typ.
    Sonderdatei,
}

/// Ein Eintrag des Inhaltsverzeichnisses.
#[derive(Debug, Clone)]
pub struct Archiveintrag {
    /// Der Pfad im Archiv.
    pub innen: Innenpfad,
    /// Ordner, Datei oder Verknuepfung.
    pub typ: Typ,
    /// Die entpackte Groesse in Bytes; bei einem Ordner null. **In einer
    /// SQLite-Datenbank die Zeilenzahl einer Tabelle** (0, wenn sie unbekannt
    /// ist) und 0 fuer eine Sicht: die Zahl der Spalte „Groesse“ (Entscheid
    /// `261023-1340-was-zeigt-die-spalte-groesse-einer-tabelle.md`). Wer Bytes
    /// braucht, fragt [`Archiveintrag::datenlaenge`].
    pub groesse: u64,
    /// Das Aenderungsdatum. Ein Zip-Eintrag ohne Zeitfeld traegt die
    /// Unix-Epoche, wie ein ferner Eintrag ohne Angabe; das MS-DOS-Feld wird
    /// aus dem Grund im Kopf von `operation::entpacken` nicht gelesen.
    pub geaendert: SystemTime,
    /// Die Rechte (`& 0o7777`), sofern das Archiv sie fuehrt.
    pub rechte: Option<u32>,
    /// Wo die Daten stehen.
    pub fundstelle: Fundstelle,
    /// Ob der Eintrag herauskommt.
    pub lesbarkeit: Lesbarkeit,
}

impl Archiveintrag {
    /// Ob der Eintrag ein Ordner ohne eigenen Eintrag im Archiv ist.
    #[must_use]
    pub fn ist_erschlossen(&self) -> bool {
        self.fundstelle == Fundstelle::Erschlossen
    }

    /// Wie viele Bytes die Daten des Eintrags haben, oder `None`, wenn das vor
    /// dem Lesen nicht feststeht.
    ///
    /// **Jede Frage nach Bytes stellt diese Methode und nicht
    /// [`Archiveintrag::groesse`]**: in einer SQLite-Datenbank ist `groesse`
    /// die Zeilenzahl einer Tabelle, und die Laenge ihrer TSV steht erst nach
    /// der Ausfuhr fest. Eine Sicht liefert ihren SQL-Text, dessen Laenge das
    /// Verzeichnis ebenso nicht kennt. Fuer Zip, tar und gz ist es `groesse`.
    #[must_use]
    pub fn datenlaenge(&self) -> Option<u64> {
        match self.fundstelle {
            Fundstelle::Zip(_)
            | Fundstelle::Tar { .. }
            | Fundstelle::GzGanz
            | Fundstelle::Erschlossen => Some(self.groesse),
            Fundstelle::SqliteTabelle { .. } | Fundstelle::SqliteSicht => None,
        }
    }

    /// Der Name, unter dem der Eintrag ausserhalb des Archivs ankommt, oder
    /// `None` an der Wurzel.
    ///
    /// Die eine Stelle, an der dieser Name entsteht (E-Ausfuhr): eine Tabelle
    /// kommt als `<name>.tsv`, eine Sicht als `<name>.sql` mit ihrem SQL-Text,
    /// jeder andere Eintrag unter seinem Namen, wie er steht.
    #[must_use]
    pub fn ausfuhrname(&self) -> Option<String> {
        let name = self.innen.name()?;
        Some(match self.fundstelle {
            Fundstelle::SqliteTabelle { .. } => format!("{name}.tsv"),
            Fundstelle::SqliteSicht => format!("{name}.sql"),
            Fundstelle::Zip(_)
            | Fundstelle::Tar { .. }
            | Fundstelle::GzGanz
            | Fundstelle::Erschlossen => name.to_owned(),
        })
    }
}

/// Was in einem Archiv steht, mit der Kennung der Datei beim Lesen.
#[derive(Debug)]
pub struct Inhaltsverzeichnis {
    kennung: Dateikennung,
    eintraege: Vec<Archiveintrag>,
    stellen: HashMap<Innenpfad, usize>,
    kinder: HashMap<Innenpfad, Vec<usize>>,
    ausgelassen: u64,
    verdeckt: u64,
    verschluesselt: bool,
}

impl Inhaltsverzeichnis {
    /// Die Kennung der Archivdatei, aus der dieses Verzeichnis gelesen ist.
    #[must_use]
    pub fn kennung(&self) -> Dateikennung {
        self.kennung
    }

    /// Alle Eintraege, erschlossene Ordner eingeschlossen, ohne die Wurzel.
    #[must_use]
    pub fn eintraege(&self) -> &[Archiveintrag] {
        &self.eintraege
    }

    /// Der Eintrag unter `innen`, oder `None`. Die Wurzel ist kein Eintrag.
    #[must_use]
    pub fn eintrag(&self, innen: &Innenpfad) -> Option<&Archiveintrag> {
        self.stellen
            .get(innen)
            .map(|&stelle| &self.eintraege[stelle])
    }

    /// Die Eintraege unmittelbar unter `ordner`, oder `None`, wenn `ordner`
    /// weder die Wurzel noch ein Ordner des Archivs ist.
    #[must_use]
    pub fn kinder(&self, ordner: &Innenpfad) -> Option<impl Iterator<Item = &Archiveintrag>> {
        self.kinder
            .get(ordner)
            .map(|stellen| stellen.iter().map(|&stelle| &self.eintraege[stelle]))
    }

    /// Wie viele Eintraege des Archivs nicht im Verzeichnis stehen: unsichere
    /// Namen, Namen, die kein UTF-8 sind, fruehere von doppelten Namen und
    /// Eintraege, die einem Ordner weichen mussten.
    #[must_use]
    pub fn ausgelassen(&self) -> u64 {
        self.ausgelassen
    }

    /// Wie viele der [`ausgelassenen`](Self::ausgelassen) Eintraege einen
    /// gueltigen Namen tragen und von einem anderen verdeckt sind: fruehere von
    /// doppelten Namen, Eintraege hinter einer Verknuepfung und Dateien, die
    /// einem Ordner weichen mussten. Ein solches Archiv schreibt KRK nicht um
    /// (Modulkopf).
    #[must_use]
    pub fn verdeckt(&self) -> u64 {
        self.verdeckt
    }

    /// Ob ein Eintrag des Archivs verschluesselt ist. Ein solches Archiv
    /// schreibt KRK nicht (E13 des Plans).
    #[must_use]
    pub fn verschluesselt(&self) -> bool {
        self.verschluesselt
    }

    /// Die Zeilenzahl der Tabelle unter `innen`, oder `None` fuer jeden
    /// anderen Eintrag, fuer eine Tabelle, deren Zaehlen scheiterte, und fuer
    /// einen Pfad, der keinen Eintrag benennt. Stellt keinen Systemaufruf.
    #[must_use]
    pub fn zeilen(&self, innen: &Innenpfad) -> Option<u64> {
        match self.eintrag(innen)?.fundstelle {
            Fundstelle::SqliteTabelle { zeilen } => zeilen,
            Fundstelle::Zip(_)
            | Fundstelle::Tar { .. }
            | Fundstelle::GzGanz
            | Fundstelle::Erschlossen
            | Fundstelle::SqliteSicht => None,
        }
    }
}

/// Sammelt die Eintraege eines Formats in Lesefolge und schliesst sie zu
/// einem [`Inhaltsverzeichnis`] ab.
#[derive(Debug, Default)]
pub(crate) struct Sammler {
    zeilen: Vec<Archiveintrag>,
    ausgelassen: u64,
}

impl Sammler {
    /// Nimmt einen Eintrag auf. Ein Eintrag an der Wurzel (`./`) nennt den
    /// Ordner, in dem das Fenster ohnehin steht, und faellt ohne Zaehlung weg.
    pub(crate) fn aufnehmen(&mut self, eintrag: Archiveintrag) -> Result<(), Verzeichnisfehler> {
        if eintrag.innen.ist_wurzel() {
            return Ok(());
        }
        if self.zeilen.len() >= HOECHSTENS_EINTRAEGE {
            return Err(Verzeichnisfehler::ZuGross);
        }
        self.zeilen.push(eintrag);
        Ok(())
    }

    /// Zaehlt einen Eintrag, der nicht ins Verzeichnis kommt.
    pub(crate) fn auslassen(&mut self) {
        self.ausgelassen += 1;
    }

    /// Wendet die Regeln aus dem Kopf dieses Moduls an und baut das
    /// Verzeichnis.
    pub(crate) fn abschliessen(
        self,
        kennung: Dateikennung,
        archivzeit: SystemTime,
    ) -> Inhaltsverzeichnis {
        let Sammler {
            zeilen,
            mut ausgelassen,
        } = self;
        let mut verdeckt = 0;

        // Doppelte Namen: der spaetere gilt, ein Ordner geht vor.
        let mut plaetze: Vec<Option<Archiveintrag>> = Vec::with_capacity(zeilen.len());
        let mut stelle_von: HashMap<Innenpfad, usize> = HashMap::with_capacity(zeilen.len());
        for zeile in zeilen {
            match stelle_von.get(&zeile.innen) {
                Some(&stelle) => {
                    ausgelassen += 1;
                    verdeckt += 1;
                    let bisher_ordner = plaetze[stelle]
                        .as_ref()
                        .is_some_and(|bisher| matches!(bisher.typ, Typ::Ordner));
                    let neu_ordner = matches!(zeile.typ, Typ::Ordner);
                    if !bisher_ordner || neu_ordner {
                        plaetze[stelle] = Some(zeile);
                    }
                }
                None => {
                    stelle_von.insert(zeile.innen.clone(), plaetze.len());
                    plaetze.push(Some(zeile));
                }
            }
        }
        drop(stelle_von);

        // Der Weg ueber eine Verknuepfung.
        let verknuepfungen: HashSet<Innenpfad> = plaetze
            .iter()
            .flatten()
            .filter(|eintrag| matches!(eintrag.typ, Typ::Verknuepfung))
            .map(|eintrag| eintrag.innen.clone())
            .collect();
        if !verknuepfungen.is_empty() {
            for platz in &mut plaetze {
                let durch_verknuepfung = platz.as_ref().is_some_and(|eintrag| {
                    vorfahren(&eintrag.innen).any(|vorfahr| verknuepfungen.contains(&vorfahr))
                });
                if durch_verknuepfung {
                    *platz = None;
                    ausgelassen += 1;
                    verdeckt += 1;
                }
            }
        }

        // Jeder Vorfahr eines Eintrags muss ein Ordner sein.
        let mut noetige_ordner: HashSet<Innenpfad> = HashSet::new();
        for eintrag in plaetze.iter().flatten() {
            for vorfahr in vorfahren(&eintrag.innen) {
                if !noetige_ordner.insert(vorfahr) {
                    // Wer diesen Vorfahren schon hat, hat auch dessen Vorfahren.
                    break;
                }
            }
        }
        for platz in &mut plaetze {
            let weicht = platz.as_ref().is_some_and(|eintrag| {
                !matches!(eintrag.typ, Typ::Ordner) && noetige_ordner.contains(&eintrag.innen)
            });
            if weicht {
                *platz = None;
                ausgelassen += 1;
                verdeckt += 1;
            }
        }

        let mut eintraege: Vec<Archiveintrag> = plaetze.into_iter().flatten().collect();
        let mut stellen: HashMap<Innenpfad, usize> = eintraege
            .iter()
            .enumerate()
            .map(|(stelle, eintrag)| (eintrag.innen.clone(), stelle))
            .collect();

        // Erschlossene Ordner, in einer festen Folge, damit zwei Laeufe ueber
        // dasselbe Archiv dasselbe Verzeichnis ergeben.
        let mut fehlende: Vec<Innenpfad> = noetige_ordner
            .into_iter()
            .filter(|ordner| !stellen.contains_key(ordner))
            .collect();
        fehlende.sort();
        for ordner in fehlende {
            stellen.insert(ordner.clone(), eintraege.len());
            eintraege.push(Archiveintrag {
                innen: ordner,
                typ: Typ::Ordner,
                groesse: 0,
                geaendert: archivzeit,
                rechte: None,
                fundstelle: Fundstelle::Erschlossen,
                lesbarkeit: Lesbarkeit::Lesbar,
            });
        }

        let mut kinder: HashMap<Innenpfad, Vec<usize>> = HashMap::new();
        kinder.insert(Innenpfad::wurzel(), Vec::new());
        for eintrag in &eintraege {
            if matches!(eintrag.typ, Typ::Ordner) {
                kinder.entry(eintrag.innen.clone()).or_default();
            }
        }
        for (stelle, eintrag) in eintraege.iter().enumerate() {
            let eltern = eintrag.innen.aufwaerts().unwrap_or_default();
            kinder.entry(eltern).or_default().push(stelle);
        }

        let verschluesselt = eintraege
            .iter()
            .any(|eintrag| eintrag.lesbarkeit == Lesbarkeit::Verschluesselt);

        Inhaltsverzeichnis {
            kennung,
            eintraege,
            stellen,
            kinder,
            ausgelassen,
            verdeckt,
            verschluesselt,
        }
    }
}

/// Die echten Vorfahren eines Pfads ohne die Wurzel, vom naechsten an.
fn vorfahren(innen: &Innenpfad) -> impl Iterator<Item = Innenpfad> {
    std::iter::successors(innen.aufwaerts(), Innenpfad::aufwaerts)
        .take_while(|vorfahr| !vorfahr.ist_wurzel())
}

/// Wie viele entpackte Bytes zwischen zwei Abbruchfragen liegen.
const ABBRUCHTAKT: u64 = 1 << 20;

/// Ein Leser, der je [`ABBRUCHTAKT`] gelesener Bytes die Abbruchmarke fragt
/// und bei gesetzter Marke mit einem Fehler endet.
///
/// Der Fehler traegt keine Nummer des Systems; wer ihn bekommt, fragt die
/// Marke ueber [`Verzeichnisfehler::aus_strom`] und meldet den Abbruch statt
/// eines ungueltigen Archivs.
pub(crate) struct Abbruchleser<'a, R> {
    innen: R,
    abbruch: &'a AtomicBool,
    seit_frage: u64,
}

impl<'a, R> Abbruchleser<'a, R> {
    pub(crate) fn neu(innen: R, abbruch: &'a AtomicBool) -> Self {
        Abbruchleser {
            innen,
            abbruch,
            seit_frage: 0,
        }
    }
}

impl<R: Read> Read for Abbruchleser<'_, R> {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        let gelesen = self.innen.read(puffer)?;
        self.seit_frage += gelesen as u64;
        if self.seit_frage >= ABBRUCHTAKT {
            self.seit_frage = 0;
            if self.abbruch.load(Ordering::Acquire) {
                return Err(io::Error::other("abgebrochen"));
            }
        }
        Ok(gelesen)
    }
}

impl<R: Seek> Seek for Abbruchleser<'_, R> {
    fn seek(&mut self, stelle: SeekFrom) -> io::Result<u64> {
        self.innen.seek(stelle)
    }
}

/// Fragt die Abbruchmarke zwischen zwei Eintraegen.
pub(crate) fn abbruch_fragen(abbruch: &AtomicBool) -> Result<(), Verzeichnisfehler> {
    if abbruch.load(Ordering::Acquire) {
        Err(Verzeichnisfehler::Abgebrochen)
    } else {
        Ok(())
    }
}

/// Wie gross ein Vorkopf eines tar-Eintrags (langer Name, langer Verweis,
/// PAX) hoechstens sein darf. Groessere sind kein Name mehr, sondern ein
/// kaputtes oder feindliches Archiv (Modulkopf).
pub(crate) const HOECHSTENS_VORKOPF: u64 = 16 * 1024 * 1024;

/// Der Wortlaut, mit dem ein zu grosser Vorkopf ein tar ungueltig macht; in
/// der Form der Kiste wie jeder andere Grund von [`Verzeichnisfehler::Ungueltig`].
pub(crate) const VORKOPF_ZU_GROSS: &str = "tar: extension header too large";

/// Haelt die Kiste `tar` beim Suchen des naechsten Eintrags unter
/// [`HOECHSTENS_VORKOPF`] (Modulkopf).
///
/// Scharf ist sie allein waehrend `Kopfwache::naechster`; die Daten eines
/// Eintrags liest der Rufer danach unbegrenzt. Gezaehlt wird in einem
/// [`Bewacht`], durch das der Strom der Kiste laeuft.
#[derive(Debug, Default)]
pub(crate) struct Kopfwache {
    scharf: Cell<bool>,
    seit_sprung: Cell<u64>,
}

impl Kopfwache {
    /// Der naechste Eintrag, mit scharfer Wache gesucht.
    pub(crate) fn naechster<'a, R: Read>(
        &self,
        eintraege: &mut tar::Entries<'a, R>,
    ) -> Option<io::Result<tar::Entry<'a, R>>> {
        self.seit_sprung.set(0);
        self.scharf.set(true);
        let naechster = eintraege.next();
        self.scharf.set(false);
        naechster
    }

    /// Der Strom `innen`, bewacht von dieser Wache.
    pub(crate) fn bewachen<R>(&self, innen: R) -> Bewacht<'_, R> {
        Bewacht { innen, wache: self }
    }
}

/// Ein Strom unter einer [`Kopfwache`]: zaehlt, was die Kiste seit ihrem
/// letzten Sprung liest, und laesst das Lesen ueber der Grenze scheitern.
pub(crate) struct Bewacht<'w, R> {
    innen: R,
    wache: &'w Kopfwache,
}

impl<R: Read> Read for Bewacht<'_, R> {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        if !self.wache.scharf.get() {
            return self.innen.read(puffer);
        }
        let grenze = HOECHSTENS_VORKOPF + 512;
        let rest = grenze.saturating_sub(self.wache.seit_sprung.get());
        if rest == 0 && !puffer.is_empty() {
            return Err(io::Error::other(VORKOPF_ZU_GROSS));
        }
        let erlaubt = puffer
            .len()
            .min(usize::try_from(rest).unwrap_or(usize::MAX));
        let gelesen = self.innen.read(&mut puffer[..erlaubt])?;
        self.wache
            .seit_sprung
            .set(self.wache.seit_sprung.get() + gelesen as u64);
        Ok(gelesen)
    }
}

impl<R: Seek> Seek for Bewacht<'_, R> {
    fn seek(&mut self, stelle: SeekFrom) -> io::Result<u64> {
        let neu = self.innen.seek(stelle)?;
        self.wache.seit_sprung.set(0);
        Ok(neu)
    }
}

/// Ein Strom, der allein vorwaerts springt, indem er liest und verwirft: fuer
/// den gepackten tar-Strom, damit die Kiste auch dort vor jedem Kopf springt
/// und die [`Kopfwache`] jeden Vorkopf einzeln misst.
pub(crate) struct Vorwaerts<R> {
    innen: R,
    stelle: u64,
}

impl<R> Vorwaerts<R> {
    pub(crate) fn neu(innen: R) -> Self {
        Vorwaerts { innen, stelle: 0 }
    }
}

impl<R: Read> Read for Vorwaerts<R> {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        let gelesen = self.innen.read(puffer)?;
        self.stelle += gelesen as u64;
        Ok(gelesen)
    }
}

impl<R: Read> Seek for Vorwaerts<R> {
    fn seek(&mut self, ziel: SeekFrom) -> io::Result<u64> {
        match ziel {
            SeekFrom::Current(sprung) if sprung >= 0 => {
                let mut rest = sprung.unsigned_abs();
                let mut puffer = [0u8; 32 * 1024];
                while rest > 0 {
                    let stueck = usize::try_from(rest.min(puffer.len() as u64)).unwrap_or(0);
                    let gelesen = match self.innen.read(&mut puffer[..stueck]) {
                        Ok(0) => return Err(io::ErrorKind::UnexpectedEof.into()),
                        Ok(gelesen) => gelesen,
                        Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => continue,
                        Err(fehler) => return Err(fehler),
                    };
                    rest -= gelesen as u64;
                    self.stelle += gelesen as u64;
                }
                Ok(self.stelle)
            }
            SeekFrom::Current(_) | SeekFrom::Start(_) | SeekFrom::End(_) => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "tar: the archive reader only seeks forward",
            )),
        }
    }
}

/// Prueft die zwei Bytes, mit denen jeder gzip-Strom beginnt, und stellt die
/// Datei danach an den Anfang zurueck.
///
/// `MultiGzDecoder` liefert fuer eine leere Datei einen leeren Strom und
/// keinen Fehler; ohne diese Frage hiesse eine leere `.tar.gz` ein leeres
/// Archiv statt keines.
fn gzip_kopf_pruefen(datei: &mut File) -> Result<(), Verzeichnisfehler> {
    let mut kopf = [0u8; 2];
    let gelesen = datei.read(&mut kopf).map_err(Verzeichnisfehler::Lesen)?;
    if gelesen < 2 || kopf != [0x1f, 0x8b] {
        return Err(Verzeichnisfehler::Ungueltig(
            "invalid gzip header".to_owned(),
        ));
    }
    datei
        .seek(SeekFrom::Start(0))
        .map_err(Verzeichnisfehler::Lesen)?;
    Ok(())
}

/// Liest das Verzeichnis eines tar-Archivs ohne Packung.
///
/// Die Daten werden ueberlesen und nicht gelesen: `entries_with_seek` springt
/// ueber sie, also kostet ein tar mit einer grossen Datei einen Sprung und
/// kein Lesen.
pub(crate) fn tar_lesen(
    datei: File,
    sammler: &mut Sammler,
    abbruch: &AtomicBool,
) -> Result<(), Verzeichnisfehler> {
    let wache = Kopfwache::default();
    let mut archiv =
        tar::Archive::new(wache.bewachen(Abbruchleser::neu(BufReader::new(datei), abbruch)));
    let eintraege = archiv
        .entries_with_seek()
        .map_err(|fehler| Verzeichnisfehler::aus_strom(fehler, abbruch))?;
    tar_eintraege_sammeln(eintraege, &wache, sammler, abbruch)
}

/// Liest das Verzeichnis eines gepackten tar-Stroms.
///
/// Der ganze Strom wird einmal entpackt und verworfen; anders ist das Ende
/// eines Eintrags in einem gzip-Strom nicht zu finden. Gesprungen wird
/// dabei ueber [`Vorwaerts`], damit die [`Kopfwache`] jeden Vorkopf misst.
pub(crate) fn tar_gz_lesen(
    mut datei: File,
    sammler: &mut Sammler,
    abbruch: &AtomicBool,
) -> Result<(), Verzeichnisfehler> {
    gzip_kopf_pruefen(&mut datei)?;
    let strom = MultiGzDecoder::new(BufReader::new(datei));
    let wache = Kopfwache::default();
    let mut archiv =
        tar::Archive::new(wache.bewachen(Vorwaerts::neu(Abbruchleser::neu(strom, abbruch))));
    let eintraege = archiv
        .entries_with_seek()
        .map_err(|fehler| Verzeichnisfehler::aus_strom(fehler, abbruch))?;
    tar_eintraege_sammeln(eintraege, &wache, sammler, abbruch)
}

/// Der gemeinsame Teil von [`tar_lesen`] und [`tar_gz_lesen`].
fn tar_eintraege_sammeln<R: Read>(
    mut eintraege: tar::Entries<'_, R>,
    wache: &Kopfwache,
    sammler: &mut Sammler,
    abbruch: &AtomicBool,
) -> Result<(), Verzeichnisfehler> {
    while let Some(eintrag) = wache.naechster(&mut eintraege) {
        abbruch_fragen(abbruch)?;
        let eintrag = eintrag.map_err(|fehler| Verzeichnisfehler::aus_strom(fehler, abbruch))?;
        let kopf = eintrag.header();
        let (typ, lesbarkeit) = match kopf.entry_type() {
            EntryType::Directory => (Typ::Ordner, Lesbarkeit::Lesbar),
            EntryType::Symlink => (Typ::Verknuepfung, Lesbarkeit::Lesbar),
            EntryType::Regular | EntryType::Continuous | EntryType::GNUSparse => {
                (Typ::Datei, Lesbarkeit::Lesbar)
            }
            EntryType::Link => (Typ::Datei, Lesbarkeit::HarterVerweis),
            EntryType::Char
            | EntryType::Block
            | EntryType::Fifo
            | EntryType::__Nonexhaustive(_) => (Typ::Datei, Lesbarkeit::Sonderdatei),
            // Koepfe, die einen anderen Eintrag beschreiben, und keine
            // Eintraege: lange Namen und die Erweiterungen nach PAX liest die
            // Kiste selbst in den naechsten Eintrag hinein, ein globaler
            // Kopf gilt fuer das ganze Archiv.
            EntryType::GNULongName
            | EntryType::GNULongLink
            | EntryType::XHeader
            | EntryType::XGlobalHeader => continue,
        };
        let name = eintrag.path_bytes();
        let Some(innen) = std::str::from_utf8(&name)
            .ok()
            .and_then(Innenpfad::aus_archivname)
        else {
            sammler.auslassen();
            continue;
        };
        let geaendert = kopf.mtime().map_or(UNIX_EPOCH, |sekunden| {
            UNIX_EPOCH + Duration::from_secs(sekunden)
        });
        let groesse = match typ {
            Typ::Ordner => 0,
            Typ::Datei | Typ::Verknuepfung => eintrag.size(),
        };
        sammler.aufnehmen(Archiveintrag {
            innen,
            typ,
            groesse,
            geaendert,
            rechte: kopf.mode().ok().map(|rechte| rechte & 0o7777),
            fundstelle: Fundstelle::Tar {
                versatz: eintrag.raw_header_position(),
            },
            lesbarkeit,
        })?;
    }
    Ok(())
}

/// Liest eine gzip-Datei, die kein tar traegt: genau ein Eintrag.
///
/// Der Name ist der Dateiname ohne die letzte Endung `.gz`
/// (`bericht.txt.gz` zeigt `bericht.txt`), wie `gunzip` ohne `-N` ihn
/// erzeugt; das Feld `FNAME` im Kopf liest KRK nicht. Hiesse der Rest leer
/// oder waere er kein einzelner Name, heisst der Eintrag `inhalt`.
///
/// **Die Groesse wird gezaehlt und nicht gelesen**: das Feld `ISIZE` am Ende
/// zaehlt nur das letzte Glied und nur modulo 2^32. Gezaehlt wird mit
/// `MultiGzDecoder`, also ueber alle Glieder.
pub(crate) fn gz_lesen(
    mut datei: File,
    dateiname: &str,
    angaben: &std::fs::Metadata,
    sammler: &mut Sammler,
    abbruch: &AtomicBool,
) -> Result<(), Verzeichnisfehler> {
    use std::os::unix::fs::PermissionsExt;

    gzip_kopf_pruefen(&mut datei)?;
    let mut strom = Abbruchleser::neu(MultiGzDecoder::new(BufReader::new(datei)), abbruch);
    let mut puffer = vec![0u8; 64 * 1024];
    let mut groesse: u64 = 0;
    loop {
        match strom.read(&mut puffer) {
            Ok(0) => break,
            Ok(gelesen) => groesse += gelesen as u64,
            Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => {}
            Err(fehler) => return Err(Verzeichnisfehler::aus_strom(fehler, abbruch)),
        }
    }
    abbruch_fragen(abbruch)?;

    let stamm = dateiname
        .len()
        .checked_sub(3)
        .filter(|&schnitt| dateiname.is_char_boundary(schnitt))
        .map_or(dateiname, |schnitt| &dateiname[..schnitt]);
    let innen = Innenpfad::wurzel()
        .verbinden(stamm)
        .or_else(|| Innenpfad::wurzel().verbinden("inhalt"))
        .unwrap_or_default();
    sammler.aufnehmen(Archiveintrag {
        innen,
        typ: Typ::Datei,
        groesse,
        geaendert: angaben.modified().unwrap_or(UNIX_EPOCH),
        rechte: Some(angaben.permissions().mode() & 0o7777),
        fundstelle: Fundstelle::GzGanz,
        lesbarkeit: Lesbarkeit::Lesbar,
    })
}
