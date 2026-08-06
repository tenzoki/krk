//! Der Messmodus: der Ablauf der Messungen, ohne eine Zeile AppKit.
//!
//! Dieses Modul haelt, was kein AppKit beruehrt: welche Messung als naechste
//! dran ist, wann eine begonnene Messung zu Ende ist, die zwanzig
//! Wiederholungen je Groesse und die Ausgabe der Einzelwerte. Dazu kommen seit
//! S21 das Einlesen des Messplans und das Herstellen der Pruefsitzung aus C8.
//! Die andere Seite der Grenze liegt in [`crate::appkit`]; herueber kommen
//! gewoehnliche Rust-Werte, die Bildwiederholrate als Zahl, die Zeitpunkte der
//! Bildgrenzen und der [`Zustand`] der Oberflaeche. **In dieser Datei steht
//! keine `use objc2`-Zeile**, und das ist nachpruefbar, nicht nur gemeint.
//!
//! # Vier Aufgaben, weil sie verschiedene Dinge messen
//!
//! - [`Aufgabe::Start`] misst L4 am Durchstich (S8) und braucht dafuer
//!   **einen Prozessstart je Wiederholung**. Die Anwendung meldet den
//!   Zeitpunkt, an dem die Oberflaeche bedienbar ist, und beendet sich. Die
//!   Spanne selbst zieht der aeussere Aufrufer, weil nur er den Zeitpunkt vor
//!   dem Start kennt.
//! - [`Aufgabe::Spannen`] misst L1, L2, L3 und L10 **innerhalb eines
//!   Prozesses**, weil alle vier Spannen in einer laufenden Anwendung liegen.
//! - [`Aufgabe::Sitzung`] ist die Strecke aus S21: sie stellt die
//!   Pruefsitzung aus C8 her und misst L1, L5, L6, L7, L8 und L9 auf ihr,
//!   jede Groesse als Spanne vom Ausloeser bis zum Ende des
//!   Zeichendurchgangs, der die Aenderung traegt.
//! - [`Aufgabe::SitzungsStart`] ist das L4 der Pruefsitzung: die Anwendung
//!   stellt die Sitzung aus `session.toml` wieder her — geschrieben hat sie
//!   der Sitzungslauf davor — und meldet den Zeitpunkt, an dem beide
//!   sichtbaren Tabs ihre erste Bildschirmseite zeigen.
//!
//! # Wie eine Spanne hier zustande kommt
//!
//! Jede Messung beginnt an einem Ausloeser und endet an einer Bildgrenze.
//!
//! ```text
//! Ausloeser (Zeitgeber, 97 ms)                Ende (Bildgrenze, ~60/s)
//!   ordner_lesen(A)      ──────────────────>  erste Zeile im Modell   = L2
//!                        ──────────────────>  Lesevorgang beendet     = L3
//!   ordner_lesen(100k)   ──────────────────>  erste Zeile im Modell   = L10
//!   Pfeil ab in die Ereignisschlange ──────>  Auswahl umgesprungen    = L1
//! ```
//!
//! **Warum der Ausloeser an einem eigenen Zeitgeber haengt und nicht an der
//! Bildgrenze.** Loeste die Bildgrenze selbst den Tastendruck aus, laege
//! zwischen Druck und naechster Bildgrenze immer genau ein volles Bild, und L1
//! haette bei 60 Hz konstant 16,7 ms — nicht gemessen, sondern gebaut. Der
//! Zeitgeber laeuft deshalb mit [`AUSLOESETAKT`], einer Spanne, die kein
//! Vielfaches der Bildlaenge ist; ueber zwanzig Wiederholungen wandert der
//! Druckzeitpunkt damit durch das Bild, und das 95. Perzentil sagt etwas.
//!
//! Jede Spanne ist auf ein Bild genau. Das ist keine Ungenauigkeit der
//! Messstrecke, sondern die Sache selbst: vor der naechsten Bildgrenze ist
//! nichts zu sehen.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use krk_core::ablage::sitzung::Sitzungsschreiber;
use krk_core::ablage::{Ablage, Sitzung};

/// Wie oft jede Messgroesse wiederholt wird. C8 schreibt zwanzig vor.
pub const WIEDERHOLUNGEN: usize = 20;

/// Die Spanne zwischen zwei Ausloesern, in Sekunden.
///
/// 97 ms sind bei 60 Hz 5,82 Bilder und bei 120 Hz 11,64. Beide Male kein
/// ganzes Vielfaches, damit der Ausloesezeitpunkt durch das Bild wandert statt
/// an einer Stelle zu kleben. Zugleich lang genug, dass ein Lesevorgang auf dem
/// Ordner mit 10.000 Eintraegen dazwischen fertig wird.
pub const AUSLOESETAKT: f64 = 0.097;

/// Wie lange eine begonnene Messung hoechstens dauern darf.
///
/// **Die Uhr und nicht der Bildzaehler.** Die erste Fassung dieser Schranke
/// zaehlte Bildgrenzen, und genau daran ist sie am 260803 gescheitert: der
/// `CADisplayLink` hoerte mitten in einer Messreihe auf zu takten, damit zaehlte
/// nichts mehr, und der Lauf stand still, bis der aeussere Aufrufer ihn nach
/// fuenf Minuten abschoss. Eine Schranke, die dasselbe Ereignis zaehlt, dessen
/// Ausbleiben sie abfangen soll, kann nicht greifen. Der Ausloesetakt laeuft
/// unabhaengig vom Bildtakt und prueft die Uhr.
const GEDULD: Duration = Duration::from_secs(10);

/// Die Meldung, mit der ein Messlauf ohne Bildschirm abbricht.
pub const OHNE_BILDSCHIRM: &str = "das Fenster steht auf keinem Bildschirm, \
     die Bildwiederholrate ist damit nicht erhebbar. Es wird keine Zahl \
     ausgegeben; auf den Hauptbildschirm weicht die Messung nicht aus.";

/// Die Meldung, mit der die Sitzungsstrecke im Hintergrund abbricht.
///
/// **Warum das ein Abbruch ist und keine langsame Oberflaeche.** Ist KRK nicht
/// die vorderste Anwendung, hat sein Fenster keinen Tastaturfokus, und
/// `Anwendungsdelegierter::kommando_ausfuehren` weist dann **jeden** Befehl
/// ab, der einen Wirkungsbereich nennt: der Fokus liegt nirgends, den ein
/// solcher Befehl braucht. Die synthetischen Tastenereignisse gehen weiterhin
/// durch den Abgriff, sie loesen nur nichts mehr aus. Was uebrig bleibt, sind
/// die Befehle mit `Wirkungsbereich::Ueberall` — genau `auswahl_runter`, mit
/// dem L1 und L7 gemessen werden. Die Strecke lief deshalb bis L5-Tab durch
/// und blieb dort zehn Sekunden stehen, weil `tab_naechster` im
/// Wirkungsbereich `Tabbereich` liegt (Defekt vom 260806-1235). Die Meldung
/// nennt seitdem die Ursache statt der Groesse, die als erste darauf traf.
pub const NICHT_IM_VORDERGRUND: &str = "KRK ist nicht die vorderste Anwendung. \
     Ohne Tastaturfokus weist KRK jeden Befehl ab, der einen Wirkungsbereich \
     nennt, und die Sitzungsstrecke misst nichts als L1 und L7. Starte den Lauf \
     so, dass KRK nach vorn kommen darf, etwa aus einem Terminalfenster im \
     Vordergrund, und arbeite waehrend des Laufs nicht in einer anderen \
     Anwendung weiter. Es wird keine Zahl ausgegeben; im Hintergrund misst die \
     Strecke nicht.";

/// Die Befehlszeilenmarke, die den Messmodus einschaltet.
const MARKE: &str = "--messmodus";

/// Was der Messmodus zu tun hat.
#[derive(Debug, Clone, PartialEq)]
pub enum Aufgabe {
    /// L4 am Durchstich: melden, wann die Oberflaeche bedienbar ist, dann
    /// beenden.
    Start {
        /// Der Ordner, den das Fenster beim Start zeigt.
        ordner: PathBuf,
    },
    /// L1, L2, L3 und L10 an der laufenden Anwendung (S8).
    Spannen {
        /// Pruefordner A mit 10.000 Eintraegen, fuer L1, L2 und L3.
        ordner_a: PathBuf,
        /// Der Ordner mit 100.000 Eintraegen, fuer L10.
        ordner100k: PathBuf,
    },
    /// L1, L5, L6, L7, L8 und L9 auf der Pruefsitzung aus C8 (S21).
    Sitzung {
        /// Der eingelesene Messplan.
        plan: Messplan,
    },
    /// L4 auf der Pruefsitzung: `session.toml` wiederherstellen und melden,
    /// wann beide sichtbaren Tabs ihre erste Bildschirmseite zeigen (S21).
    SitzungsStart,
}

impl Aufgabe {
    /// Liest die Aufgabe aus der Befehlszeile.
    ///
    /// Liefert `Ok(None)`, wenn `--messmodus` gar nicht vorkommt: das ist der
    /// gewoehnliche Start und kein Fehler. Unbekannte Marken werden dabei
    /// uebergangen, weil LaunchServices einem ueber den Finder gestarteten
    /// Buendel eigene anhaengt.
    ///
    /// Jeder Wert, der keine der drei benannten Aufgaben ist, gilt als Pfad
    /// eines Messplans: `--messmodus <plan.toml>` ist seit S21 die Schreibweise
    /// des Sitzungslaufs, und ein Tippfehler in einem Aufgabennamen faellt
    /// damit als "Messplan nicht lesbar" auf statt still als gewoehnlicher
    /// Start.
    pub fn aus_argumenten(argumente: &[String]) -> Result<Option<Self>, String> {
        let Some(stelle) = argumente.iter().position(|marke| marke == MARKE) else {
            return Ok(None);
        };
        let art = argumente.get(stelle + 1).ok_or_else(|| {
            format!("{MARKE} braucht eine Aufgabe: start, spannen, sitzungsstart oder <plan.toml>")
        })?;

        match art.as_str() {
            "start" => Ok(Some(Aufgabe::Start {
                ordner: pfad(argumente, "--ordner")?,
            })),
            "spannen" => Ok(Some(Aufgabe::Spannen {
                ordner_a: pfad(argumente, "--ordner-a")?,
                ordner100k: pfad(argumente, "--ordner100k")?,
            })),
            "sitzungsstart" => Ok(Some(Aufgabe::SitzungsStart)),
            planpfad => Ok(Some(Aufgabe::Sitzung {
                plan: Messplan::lesen(Path::new(planpfad))?,
            })),
        }
    }

    /// Der Ordner, den das linke Dateifenster beim Start allein liest.
    ///
    /// `None` fuer die beiden Sitzungsaufgaben: dort stellt die Anwendung die
    /// Pruefsitzung her und liest ihre sichtbaren Tabs auf dem gewoehnlichen
    /// Weg, denn genau dieser Weg ist Teil dessen, was L4 und L5 messen.
    pub fn startordner(&self) -> Option<&Path> {
        match self {
            Aufgabe::Start { ordner } => Some(ordner),
            Aufgabe::Spannen { ordner_a, .. } => Some(ordner_a),
            Aufgabe::Sitzung { .. } | Aufgabe::SitzungsStart => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Der Messplan (S21)
// ---------------------------------------------------------------------------

/// Der Messplan: die Pruefsitzung aus C8 samt Kopierziel und L6-Unterordner.
///
/// Der Abschnitt `[sitzung]` ist **dieselbe Struktur, die `session.toml`
/// traegt**, ueber dieselbe Serialisierung aus
/// `krk-core/src/ablage/sitzung.rs`; ein zweites Format fuer dieselbe Sitzung
/// entsteht nicht. Geschrieben wird der Plan vom Messwerkzeug `krk-bench`;
/// die beiden Kisten haengen ueber diese Datei zusammen und nicht ueber eine
/// Abhaengigkeit, dieselbe Form wie beim Praefix der Messzeilen.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Messplan {
    /// Das Kopierziel fuer L8 und L9, auf demselben APFS-Datentraeger wie
    /// Pruefordner A und zu Beginn leer.
    pub kopierziel: PathBuf,
    /// Der Unterordner mit 1.000 Eintraegen fuer L6, neben Pruefordner A.
    pub unterordner: PathBuf,
    /// Die Pruefsitzung aus C8, in der Serialisierung von `session.toml`.
    pub sitzung: Sitzung,
}

impl Messplan {
    /// Liest den Messplan und prueft seine Voraussetzungen.
    pub fn lesen(pfad: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(pfad).map_err(|fehler| {
            format!(
                "der Messplan {} laesst sich nicht lesen: {fehler}",
                pfad.display()
            )
        })?;
        let plan: Messplan = toml::from_str(&text).map_err(|fehler| {
            format!(
                "der Messplan {} ist kein gueltiger Plan: {fehler}",
                pfad.display()
            )
        })?;
        plan.pruefen()?;
        Ok(plan)
    }

    /// Pruefordner A: der sichtbare Tab des linken Dateifensters.
    pub fn ordner_a(&self) -> &Path {
        self.sitzung.fenster[0]
            .aktiver_tab()
            .map(|tab| tab.ordner.as_path())
            .unwrap_or_else(|| Path::new("/"))
    }

    /// Die Voraussetzungen, ohne die die Strecke nicht anlaeuft.
    ///
    /// **Ein Kopierziel auf einem anderen Datentraeger wird nicht
    /// angenommen**, weil die duennbesetzten Pruefdateien dort als Nullen
    /// ausgeschrieben wuerden und L8 dann einen Durchsatz misst statt der
    /// Sichtbarkeit des Fortschritts. Und es muss **leer** sein, weil der
    /// Lauf es zwischen den Wiederholungen leert; ein fremder Inhalt darf
    /// dabei nicht verschwinden koennen.
    fn pruefen(&self) -> Result<(), String> {
        use std::os::unix::fs::MetadataExt as _;
        for (name, ordner) in [
            ("Pruefordner A", self.ordner_a()),
            ("der Unterordner fuer L6", &self.unterordner),
            ("das Kopierziel", &self.kopierziel),
        ] {
            if !ordner.is_dir() {
                return Err(format!(
                    "{name} ({}) ist kein Verzeichnis",
                    ordner.display()
                ));
            }
        }
        let mut eintraege = std::fs::read_dir(&self.kopierziel)
            .map_err(|fehler| format!("das Kopierziel laesst sich nicht lesen: {fehler}"))?;
        if eintraege.next().is_some() {
            return Err(format!(
                "das Kopierziel {} ist nicht leer. Der Messlauf leert es zwischen den \
                 Wiederholungen; fremder Inhalt darf dabei nicht verschwinden.",
                self.kopierziel.display()
            ));
        }
        let geraet = |pfad: &Path| {
            std::fs::metadata(pfad)
                .map(|angaben| angaben.dev())
                .map_err(|fehler| format!("{} laesst sich nicht pruefen: {fehler}", pfad.display()))
        };
        if geraet(self.ordner_a())? != geraet(&self.kopierziel)? {
            return Err(format!(
                "das Kopierziel {} liegt auf einem anderen Datentraeger als Pruefordner A. \
                 L8 und L9 messen auf demselben APFS-Datentraeger; auf einem anderen wuerden \
                 die duennbesetzten Pruefdateien als Nullen ausgeschrieben, und die Zahl \
                 waere ein Durchsatz und keine Sichtbarkeitszusage.",
                self.kopierziel.display()
            ));
        }
        Ok(())
    }

    /// Stellt die Pruefsitzung her: schreibt sie als `session.toml`.
    ///
    /// Ueber [`Sitzungsschreiber`], also durch **dieselbe** Schreibfunktion,
    /// die die Anwendung beim Beenden benutzt. Die folgenden L4-Starts
    /// (`--messmodus sitzungsstart`) finden damit dieselbe Lage vor, und ein
    /// zweites Format fuer dieselbe Datei entsteht nicht.
    pub fn herstellen(&self) -> Result<PathBuf, String> {
        let ablage = Ablage::im_benutzerverzeichnis()
            .map_err(|fehler| format!("der Ablageordner laesst sich nicht oeffnen: {fehler}"))?;
        let pfad = ablage.pfad(krk_core::ablage::Datei::Sitzung);
        let mut schreiber = Sitzungsschreiber::neu(&pfad);
        schreiber
            .vormerken(self.sitzung.clone(), Instant::now())
            .map_err(|fehler| {
                format!(
                    "die Pruefsitzung liess sich nicht nach {} schreiben: {fehler}",
                    pfad.display()
                )
            })?;
        Ok(pfad)
    }

    /// Leert das Kopierziel zwischen zwei Wiederholungen.
    fn kopierziel_leeren(&self) -> std::io::Result<()> {
        for eintrag in std::fs::read_dir(&self.kopierziel)? {
            let eintrag = eintrag?;
            if eintrag.file_type()?.is_dir() {
                std::fs::remove_dir_all(eintrag.path())?;
            } else {
                std::fs::remove_file(eintrag.path())?;
            }
        }
        Ok(())
    }
}

/// Holt den Wert einer benannten Marke aus der Befehlszeile.
fn pfad(argumente: &[String], marke: &str) -> Result<PathBuf, String> {
    let stelle = argumente
        .iter()
        .position(|wort| wort == marke)
        .ok_or_else(|| format!("{marke} fehlt"))?;
    argumente
        .get(stelle + 1)
        .map(PathBuf::from)
        .ok_or_else(|| format!("{marke} braucht einen Pfad"))
}

/// Was die Oberflaeche an einer Bildgrenze ueber sich sagt.
///
/// Gewoehnliche Rust-Werte, kein AppKit-Wert. Die drei Zahlen oben sind
/// alles, was die Strecken aus S8 brauchen; die [`Sitzungslage`] dahinter
/// fuellt die Oberflaeche fuer die Sitzungsaufgaben aus S21.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zustand {
    /// Wie viele Zeilen der sichtbare Tab des linken Dateifensters traegt.
    pub zeilen: usize,
    /// Ob dort noch ein Lesevorgang laeuft.
    pub liest: bool,
    /// Welche Zeile dort ausgewaehlt ist; -1, wenn keine.
    pub auswahl: isize,
    /// Die Lage der ganzen Oberflaeche; `None` auf den Strecken aus S8.
    pub sitzung: Option<Sitzungslage>,
}

/// Die Lage der Oberflaeche, soweit die Sitzungsstrecke aus S21 sie braucht.
///
/// "Aktiv" heisst: das aktive Dateifenster und sein sichtbarer Tab. Jedes
/// Feld beantwortet genau eine Endbedingung einer Messgroesse; was keine
/// Groesse braucht, steht nicht hier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sitzungslage {
    /// Ob KRK die vorderste Anwendung ist.
    ///
    /// Keine Messgroesse, sondern die Voraussetzung aller uebrigen: ohne sie
    /// laeuft kein Befehl mit Wirkungsbereich. Siehe
    /// [`NICHT_IM_VORDERGRUND`].
    pub im_vordergrund: bool,
    /// Ob das linke Dateifenster das aktive ist.
    pub aktiv_links: bool,
    /// Zeilen des sichtbaren Tabs im aktiven Dateifenster.
    pub zeilen_aktiv: usize,
    /// Ob dort noch ein Lesevorgang laeuft.
    pub liest_aktiv: bool,
    /// Die ausgewaehlte Zeile dort; -1, wenn keine.
    pub auswahl_aktiv: isize,
    /// Die Stelle des sichtbaren Tabs im aktiven Dateifenster.
    pub tab_aktiv: usize,
    /// Der Ordner des sichtbaren Tabs im aktiven Dateifenster.
    pub ordner_aktiv: PathBuf,
    /// Der vollstaendige Pfad des ausgewaehlten Eintrags dort.
    pub auswahl_pfad: Option<PathBuf>,
    /// Zeilen des sichtbaren Tabs im rechten Dateifenster.
    pub zeilen_rechts: usize,
    /// Ob dort noch ein Lesevorgang laeuft.
    pub liest_rechts: bool,
    /// Der Ordner des sichtbaren Tabs im rechten Dateifenster.
    pub ordner_rechts: PathBuf,
    /// Welche Datei der aktive Vorschau-Tab zeigt (L7).
    pub vorschau_pfad: Option<PathBuf>,
    /// Ob ein Vorschau-Tab noch auf seinen Arbeitsfaden wartet (L7).
    pub vorschau_laedt: bool,
    /// Ob die Vorgangsanzeige einer Dateioperation in einer Statuszeile
    /// steht (L8).
    pub vorgang_sichtbar: bool,
    /// Ob eine Dateioperation laeuft (L9 und das Aufraeumen danach).
    pub vorgang_laeuft: bool,
}

/// Was die Oberflaeche als naechstes tun soll.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Anweisung {
    /// Nichts; eine Messung laeuft noch.
    Warten,
    /// Den genannten Ordner im linken Dateifenster lesen (Strecke aus S8).
    Lesen(PathBuf),
    /// Einen Pfeil-ab-Tastendruck in die eigene Ereignisschlange stellen
    /// (Strecke aus S8; sie kommt ohne Belegung aus).
    Taste,
    /// Die erste Kombination der genannten Funktion als synthetisches
    /// Tastenereignis in die eigene Ereignisschlange stellen (S21).
    Funktionstaste(&'static str),
    /// Eine ungemessene Vorbereitung ausfuehren (S21).
    Handeln(Handlung),
    /// Alles gemessen; ausgeben und beenden.
    Fertig,
    /// Abbrechen mit dieser Meldung.
    Abbruch(String),
}

/// Eine ungemessene Vorbereitung zwischen zwei Messungen (S21).
///
/// Diese Schritte gehen absichtlich **nicht** ueber die Ereignisschlange:
/// gemessen wird nur, was C8 zusagt, und eine Vorbereitung, die denselben Weg
/// nimmt wie die Messung, stuende ihr in der Schlange im Weg.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Handlung {
    /// In beiden sichtbaren Tabs die Auswahl auf den ersten Eintrag setzen,
    /// wie die Pruefsitzung aus C8 es vorschreibt.
    Listenanfaenge,
    /// Im aktiven Dateifenster den Eintrag mit diesem Namen auswaehlen.
    Auswaehlen(String),
    /// Im aktiven Dateifenster alle Eintraege markieren.
    AlleMarkieren,
    /// Im aktiven Dateifenster den genannten Ordner in den sichtbaren Tab
    /// lesen.
    AktivLesen(PathBuf),
    /// Im rechten Dateifenster den genannten Ordner in den sichtbaren Tab
    /// lesen.
    RechtsLesen(PathBuf),
}

/// Ein Schritt des Ablaufs.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Schritt {
    /// Ein ungezaehlter Lesevorgang, der den Cache waermt.
    ///
    /// C8 sagt: "warm" heisst jeder **weitere** Zugriff. Ohne diesen Vorlauf
    /// truege der erste von zwanzig Laeufen eine kalte Zahl in eine warme Reihe.
    Vorlauf(PathBuf),
    /// Pruefordner A lesen und L2 sowie L3 zaehlen.
    LesenA,
    /// Den Ordner mit 100.000 Eintraegen lesen und L10 zaehlen.
    Lesen100k,
    /// Einen Tastendruck absetzen und L1 zaehlen.
    Taste,
}

/// Wie eine laufende Messung gezaehlt wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Zaehlung {
    /// Gar nicht: ein Vorlauf.
    Keine,
    /// Als L2 und L3.
    A,
    /// Als L10.
    Gross,
}

/// Was gerade laeuft.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lage {
    /// Nichts; der naechste Schritt darf beginnen.
    Bereit,
    /// Ein Lesevorgang laeuft.
    Liest {
        t0: Instant,
        zaehlung: Zaehlung,
        erste_seite: Option<Duration>,
        bilder: u32,
    },
    /// Ein Tastendruck ist unterwegs.
    Taste {
        t0: Instant,
        auswahl_vorher: isize,
        bilder: u32,
    },
}

/// Die gesammelten Einzelwerte.
#[derive(Debug, Clone, Default)]
struct Werte {
    /// L1: Tastendruck bis Bildgrenze mit umgesprungener Auswahl.
    l1: Vec<Duration>,
    /// L2: Lesebeginn bis Bildgrenze mit erster Bildschirmseite, Ordner A.
    l2: Vec<Duration>,
    /// L3: Lesebeginn bis Bildgrenze mit vollstaendig gelesenem Ordner A.
    l3: Vec<Duration>,
    /// L10: wie L2, auf dem Ordner mit 100.000 Eintraegen.
    l10_erste: Vec<Duration>,
    /// Das vollstaendige Lesen desselben Ordners. C8 sagt dafuer 4 s warm zu;
    /// das Gate von Schritt 8 fragt die Zahl nicht ab, der Bericht nennt sie.
    l10_voll: Vec<Duration>,
}

// ---------------------------------------------------------------------------
// Die Sitzungsstrecke (S21)
// ---------------------------------------------------------------------------

/// Eine der sechs Messgroessen der Sitzungsstrecke.
///
/// L5 steht zweimal, weil C8 es zweimal misst: einmal fuer den Wechsel auf
/// den verdeckten Tab und einmal fuer den Wechsel des aktiven Dateifensters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sitzungsgroesse {
    L1,
    L5Tab,
    L5Fenster,
    L6,
    L7,
    L8,
    L9,
}

impl Sitzungsgroesse {
    /// Der Name der Messzeile, unter dem der aeussere Aufrufer die
    /// Einzelwerte einsammelt.
    const fn name(self) -> &'static str {
        match self {
            Self::L1 => "l1",
            Self::L5Tab => "l5-tab",
            Self::L5Fenster => "l5-fenster",
            Self::L6 => "l6",
            Self::L7 => "l7",
            Self::L8 => "l8",
            Self::L9 => "l9",
        }
    }
}

/// Ein Schritt des Sitzungsablaufs.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Sitzungsschritt {
    /// Ungemessen warten, bis die Bedingung steht.
    Warten(Bedingung),
    /// Eine ungemessene Vorbereitung an die Oberflaeche geben.
    Handeln(Handlung),
    /// Eine Taste in die Ereignisschlange stellen. Mit Messgroesse ist es
    /// eine Messung; ohne ist es ein ungemessener Druck, etwa der Abbruch
    /// einer Kopie, und der naechste [`Sitzungsschritt::Warten`] faengt seine
    /// Wirkung ab.
    Taste {
        funktion: &'static str,
        messung: Option<Sitzungsgroesse>,
    },
    /// Das Kopierziel leeren; laeuft synchron im Messlauf selbst.
    Aufraeumen,
}

/// Worauf ein ungemessener Warteschritt wartet.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Bedingung {
    /// Der sichtbare Tab links ist gelesen.
    LinksGelesen,
    /// Der sichtbare Tab rechts zeigt den Ordner und ist gelesen. Auf die
    /// Zeilenzahl kommt es nicht an: das leere Kopierziel ist der gedachte
    /// Fall.
    RechtsZeigt(PathBuf),
    /// Der sichtbare Tab des aktiven Fensters zeigt den Ordner, ist gelesen
    /// und traegt Zeilen.
    AktivZeigt(PathBuf),
    /// Der sichtbare Tab des aktiven Fensters ist gelesen und traegt Zeilen.
    AktivGelesen,
    /// Keine Dateioperation laeuft mehr.
    VorgangVorbei,
}

impl Bedingung {
    /// Ob die Bedingung im gemeldeten Zustand steht.
    fn steht(&self, zustand: &Zustand) -> bool {
        let Some(lage) = zustand.sitzung.as_ref() else {
            return false;
        };
        match self {
            Bedingung::LinksGelesen => zustand.zeilen > 0 && !zustand.liest,
            Bedingung::RechtsZeigt(pfad) => &lage.ordner_rechts == pfad && !lage.liest_rechts,
            Bedingung::AktivZeigt(pfad) => {
                &lage.ordner_aktiv == pfad && !lage.liest_aktiv && lage.zeilen_aktiv > 0
            }
            Bedingung::AktivGelesen => !lage.liest_aktiv && lage.zeilen_aktiv > 0,
            Bedingung::VorgangVorbei => !lage.vorgang_laeuft,
        }
    }
}

/// Der festgehaltene Stand unmittelbar vor einer Sitzungsmessung.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vorher {
    auswahl: isize,
    tab: usize,
    aktiv_links: bool,
}

impl Vorher {
    fn aus(lage: &Sitzungslage) -> Self {
        Self {
            auswahl: lage.auswahl_aktiv,
            tab: lage.tab_aktiv,
            aktiv_links: lage.aktiv_links,
        }
    }
}

/// Was auf der Sitzungsstrecke gerade laeuft.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sitzungslauf {
    /// Nichts; der naechste Schritt darf beginnen.
    Bereit,
    /// Ein ungemessener Warteschritt.
    Wartet { seit: Instant, bilder: u32 },
    /// Eine Messung laeuft.
    Misst {
        t0: Instant,
        groesse: Sitzungsgroesse,
        vorher: Vorher,
        bilder: u32,
    },
}

/// Die Einzelwerte der Sitzungsstrecke.
#[derive(Debug, Clone, Default)]
struct Sitzungswerte {
    l1: Vec<Duration>,
    l5_tab: Vec<Duration>,
    l5_fenster: Vec<Duration>,
    l6: Vec<Duration>,
    l7: Vec<Duration>,
    l8: Vec<Duration>,
    l9: Vec<Duration>,
}

impl Sitzungswerte {
    fn eintragen(&mut self, groesse: Sitzungsgroesse, wert: Duration) {
        match groesse {
            Sitzungsgroesse::L1 => self.l1.push(wert),
            Sitzungsgroesse::L5Tab => self.l5_tab.push(wert),
            Sitzungsgroesse::L5Fenster => self.l5_fenster.push(wert),
            Sitzungsgroesse::L6 => self.l6.push(wert),
            Sitzungsgroesse::L7 => self.l7.push(wert),
            Sitzungsgroesse::L8 => self.l8.push(wert),
            Sitzungsgroesse::L9 => self.l9.push(wert),
        }
    }

    fn alle(&self) -> [(&'static str, &Vec<Duration>); 7] {
        [
            ("l1", &self.l1),
            ("l5-tab", &self.l5_tab),
            ("l5-fenster", &self.l5_fenster),
            ("l6", &self.l6),
            ("l7", &self.l7),
            ("l8", &self.l8),
            ("l9", &self.l9),
        ]
    }
}

/// Ob eine begonnene Sitzungsmessung an dieser Bildgrenze zu Ende ist.
///
/// Jede Endbedingung ist die Sichtbarkeitszusage der Groesse, uebersetzt in
/// den Zustand der Oberflaeche: die Auswahl ist umgesprungen, der Zieltab
/// steht mit seiner ersten Bildschirmseite, der Unterordner ist vollstaendig
/// gelesen, die Vorschau zeigt den ausgewaehlten Eintrag, die Vorgangsanzeige
/// steht in der Statuszeile.
fn sitzungsmessung_fertig(
    groesse: Sitzungsgroesse,
    vorher: Vorher,
    lage: &Sitzungslage,
    unterordner: &Path,
) -> bool {
    match groesse {
        Sitzungsgroesse::L1 | Sitzungsgroesse::L9 => {
            lage.auswahl_aktiv >= 0 && lage.auswahl_aktiv != vorher.auswahl
        }
        Sitzungsgroesse::L5Tab => lage.tab_aktiv != vorher.tab && lage.zeilen_aktiv > 0,
        Sitzungsgroesse::L5Fenster => {
            lage.aktiv_links != vorher.aktiv_links && lage.zeilen_aktiv > 0
        }
        Sitzungsgroesse::L6 => {
            lage.ordner_aktiv == unterordner && !lage.liest_aktiv && lage.zeilen_aktiv > 0
        }
        Sitzungsgroesse::L7 => {
            lage.auswahl_aktiv != vorher.auswahl
                && lage.auswahl_pfad.is_some()
                && lage.auswahl_pfad == lage.vorschau_pfad
                && !lage.vorschau_laedt
        }
        Sitzungsgroesse::L8 => lage.vorgang_sichtbar,
    }
}

/// Ob eine Sitzungsmessung an dieser Stelle ueberhaupt beginnen kann.
///
/// Ein Verstoss ist ein Fehler der Strecke oder der Umgebung, keine langsame
/// Oberflaeche; er fuehrt zum Abbruch ohne Zahl statt zu einem Wert, der
/// etwas anderes misst als seine Zusage.
fn messung_unmoeglich(groesse: Sitzungsgroesse, lage: &Sitzungslage) -> Option<String> {
    // Vor jeder Groesse und fuer jede dieselbe Frage: steht KRK vorn? Der
    // Vorbehalt haengt an der Strecke und nicht an einer einzelnen Zusage,
    // und er steht deshalb vor der Aufzaehlung statt in ihr.
    if !lage.im_vordergrund {
        return Some(NICHT_IM_VORDERGRUND.to_owned());
    }
    match groesse {
        Sitzungsgroesse::L1 | Sitzungsgroesse::L7 => (lage.zeilen_aktiv == 0)
            .then(|| "die Liste ist leer; ein Tastendruck kann keine Auswahl bewegen".to_owned()),
        Sitzungsgroesse::L9 => (!lage.vorgang_laeuft).then(|| {
            "die Kopie ist schon fertig; L9 misst die Tastatur waehrend einer laufenden \
             Stapeloperation"
                .to_owned()
        }),
        Sitzungsgroesse::L8 => lage.vorgang_laeuft.then(|| {
            "es laeuft noch eine Dateioperation; L8 braucht einen frischen Start".to_owned()
        }),
        _ => None,
    }
}

/// Baut die Schrittliste der Sitzungsstrecke.
///
/// Die Reihenfolge folgt der Pruefsitzung: erst die Messungen, die auf der
/// unveraenderten Sitzung laufen (L1, L7, L5), dann L6 mit seinen
/// Ordnerwechseln, zuletzt L8 und L9, weil sie das rechte Fenster auf das
/// Kopierziel stellen. Vor jeder Reihe steht ein ungemessener Vorlauf, wo
/// der erste von zwanzig Laeufen sonst eine kalte Zahl in eine warme Reihe
/// truege — dieselbe Regel wie bei [`Schritt::Vorlauf`].
fn sitzungsschritte(plan: &Messplan) -> Vec<Sitzungsschritt> {
    use Sitzungsschritt as S;
    let ordner_a = plan.ordner_a().to_path_buf();
    let eltern = plan
        .unterordner
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("/"));
    let unterordner_name = plan
        .unterordner
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();

    let mut schritte = Vec::new();
    // Die Pruefsitzung steht: beide sichtbaren Tabs gelesen, Auswahl auf dem
    // ersten Eintrag.
    schritte.push(S::Warten(Bedingung::LinksGelesen));
    schritte.push(S::Warten(Bedingung::RechtsZeigt(
        plan.sitzung.fenster[1]
            .aktiver_tab()
            .map(|tab| tab.ordner.clone())
            .unwrap_or_default(),
    )));
    schritte.push(S::Handeln(Handlung::Listenanfaenge));

    // L1: zwanzig Pfeil ab im aktiven Dateifenster.
    for _ in 0..WIEDERHOLUNGEN {
        schritte.push(S::Taste {
            funktion: "auswahl_runter",
            messung: Some(Sitzungsgroesse::L1),
        });
    }

    // L7: dieselbe Taste, gemessen bis die Vorschau den neuen Eintrag zeigt.
    for _ in 0..WIEDERHOLUNGEN {
        schritte.push(S::Taste {
            funktion: "auswahl_runter",
            messung: Some(Sitzungsgroesse::L7),
        });
    }

    // L5, erster Fall: der Wechsel auf den verdeckten Tab. Ein ungemessener
    // Wechsel davor, damit jede gemessene Wiederholung einen gelesenen
    // Zielordner trifft; der Bericht weist genau diesen Fall aus.
    schritte.push(S::Taste {
        funktion: "tab_naechster",
        messung: None,
    });
    schritte.push(S::Warten(Bedingung::AktivGelesen));
    for _ in 0..WIEDERHOLUNGEN {
        schritte.push(S::Taste {
            funktion: "tab_naechster",
            messung: Some(Sitzungsgroesse::L5Tab),
        });
    }
    // Nach 21 Wechseln steht der zweite Tab vorn; einer bringt Tab A zurueck.
    schritte.push(S::Taste {
        funktion: "tab_naechster",
        messung: None,
    });
    schritte.push(S::Warten(Bedingung::AktivGelesen));

    // L5, zweiter Fall: der Wechsel des aktiven Dateifensters. Zwanzig
    // Wechsel enden wieder links.
    for _ in 0..WIEDERHOLUNGEN {
        schritte.push(S::Taste {
            funktion: "fenster_wechseln",
            messung: Some(Sitzungsgroesse::L5Fenster),
        });
    }

    // L6: der Einstieg in den Unterordner mit 1.000 Eintraegen. Ein
    // ungemessener Vorlauf waermt ihn; danach je Wiederholung in den
    // Elternordner, den Unterordner auswaehlen, gemessen einsteigen, zurueck
    // auf Pruefordner A.
    schritte.push(S::Handeln(Handlung::AktivLesen(plan.unterordner.clone())));
    schritte.push(S::Warten(Bedingung::AktivZeigt(plan.unterordner.clone())));
    schritte.push(S::Handeln(Handlung::AktivLesen(ordner_a.clone())));
    schritte.push(S::Warten(Bedingung::AktivZeigt(ordner_a.clone())));
    for _ in 0..WIEDERHOLUNGEN {
        schritte.push(S::Handeln(Handlung::AktivLesen(eltern.clone())));
        schritte.push(S::Warten(Bedingung::AktivZeigt(eltern.clone())));
        schritte.push(S::Handeln(Handlung::Auswaehlen(unterordner_name.clone())));
        schritte.push(S::Taste {
            funktion: "oeffnen",
            messung: Some(Sitzungsgroesse::L6),
        });
        schritte.push(S::Handeln(Handlung::AktivLesen(ordner_a.clone())));
        schritte.push(S::Warten(Bedingung::AktivZeigt(ordner_a.clone())));
    }

    // L8 und L9: das rechte Fenster zeigt das Kopierziel, links sind alle
    // Eintraege markiert. Je Wiederholung: F5 gemessen bis zur sichtbaren
    // Vorgangsanzeige (L8), ein Pfeil ab waehrend der laufenden Kopie (L9),
    // dann Abbruch, Aufraeumen und die Auffrischung abwarten.
    schritte.push(S::Handeln(Handlung::RechtsLesen(plan.kopierziel.clone())));
    schritte.push(S::Warten(Bedingung::RechtsZeigt(plan.kopierziel.clone())));
    for _ in 0..WIEDERHOLUNGEN {
        schritte.push(S::Handeln(Handlung::AlleMarkieren));
        schritte.push(S::Taste {
            funktion: "kopieren",
            messung: Some(Sitzungsgroesse::L8),
        });
        schritte.push(S::Taste {
            funktion: "auswahl_runter",
            messung: Some(Sitzungsgroesse::L9),
        });
        schritte.push(S::Taste {
            funktion: "abbrechen",
            messung: None,
        });
        schritte.push(S::Warten(Bedingung::VorgangVorbei));
        schritte.push(S::Aufraeumen);
        schritte.push(S::Warten(Bedingung::AktivZeigt(ordner_a.clone())));
    }

    schritte
}

/// Ein laufender Messlauf.
pub struct Messlauf {
    aufgabe: Aufgabe,
    schritte: Vec<Schritt>,
    stelle: usize,
    lage: Lage,
    werte: Werte,
    /// Der Ablauf der Sitzungsstrecke (S21); leer auf den Strecken aus S8.
    sitzungsschritte: Vec<Sitzungsschritt>,
    sitzungsstelle: usize,
    sitzungslauf: Sitzungslauf,
    sitzungswerte: Sitzungswerte,
    bildwiederholrate: Option<isize>,
    /// Nur fuer die beiden Startaufgaben: ob der Zeitpunkt schon gemeldet ist.
    gemeldet: bool,
}

impl Messlauf {
    /// Legt den Ablauf zur genannten Aufgabe an.
    pub fn neu(aufgabe: Aufgabe) -> Self {
        let sitzungsschritte = match &aufgabe {
            Aufgabe::Sitzung { plan } => sitzungsschritte(plan),
            _ => Vec::new(),
        };
        let schritte = match &aufgabe {
            Aufgabe::Start { .. } | Aufgabe::Sitzung { .. } | Aufgabe::SitzungsStart => Vec::new(),
            Aufgabe::Spannen {
                ordner_a,
                ordner100k,
            } => {
                let mut schritte = Vec::with_capacity(3 * WIEDERHOLUNGEN + 3);
                // Der Ordner, der beim Start ohnehin gelesen wird, ist noch
                // kein Vorlauf: er laeuft, bevor der Messlauf steht.
                schritte.push(Schritt::Vorlauf(ordner_a.clone()));
                schritte.extend(std::iter::repeat_n(Schritt::LesenA, WIEDERHOLUNGEN));
                schritte.push(Schritt::Vorlauf(ordner100k.clone()));
                schritte.extend(std::iter::repeat_n(Schritt::Lesen100k, WIEDERHOLUNGEN));
                // Vor den Tastendruecken zurueck auf den kleinen Ordner: L1
                // misst die Auswahlbewegung in einer stehenden Liste, nicht
                // waehrend eines Lesevorgangs.
                schritte.push(Schritt::Vorlauf(ordner_a.clone()));
                schritte.extend(std::iter::repeat_n(Schritt::Taste, WIEDERHOLUNGEN));
                schritte
            }
        };
        Self {
            aufgabe,
            schritte,
            stelle: 0,
            lage: Lage::Bereit,
            werte: Werte::default(),
            sitzungsschritte,
            sitzungsstelle: 0,
            sitzungslauf: Sitzungslauf::Bereit,
            sitzungswerte: Sitzungswerte::default(),
            bildwiederholrate: None,
            gemeldet: false,
        }
    }

    /// Haelt die Bildwiederholrate fest, die die Oberflaeche ausgelesen hat.
    pub fn rate_setzen(&mut self, hertz: isize) {
        self.bildwiederholrate = Some(hertz);
    }

    /// Fragt, was als naechstes zu tun ist.
    ///
    /// Wird vom Ausloesetakt gerufen. Der Zeitpunkt, ab dem gemessen wird, ist
    /// der dieses Aufrufs und damit **vor** dem AppKit-Aufruf, den der Aufrufer
    /// gleich absetzt.
    pub fn naechster_schritt(&mut self, zustand: Zustand) -> Anweisung {
        // Die Startaufgaben haben keinen Ablauf: sie warten auf die eine
        // Bildgrenze, an der die erste Bildschirmseite steht. Ohne diese Zeile
        // faende der Ausloesetakt eine leere Schrittliste vor und meldete
        // `Fertig`, sobald er vor der ersten Bildgrenze drankaeme — ein Rennen,
        // das ein langsamer Startordner gewinnt und das dann eine Messung ohne
        // Zahl ausgaebe.
        if matches!(self.aufgabe, Aufgabe::Start { .. } | Aufgabe::SitzungsStart) {
            return Anweisung::Warten;
        }
        if matches!(self.aufgabe, Aufgabe::Sitzung { .. }) {
            return self.sitzung_weiter(&zustand);
        }
        if let Some(grund) = self.haengt() {
            return Anweisung::Abbruch(grund);
        }
        if self.lage != Lage::Bereit {
            return Anweisung::Warten;
        }
        let Some(schritt) = self.schritte.get(self.stelle).cloned() else {
            return Anweisung::Fertig;
        };
        match schritt {
            Schritt::Vorlauf(pfad) => {
                self.lage = Lage::Liest {
                    t0: Instant::now(),
                    zaehlung: Zaehlung::Keine,
                    erste_seite: None,
                    bilder: 0,
                };
                Anweisung::Lesen(pfad)
            }
            Schritt::LesenA | Schritt::Lesen100k => {
                let (zaehlung, pfad) = match schritt {
                    Schritt::LesenA => (Zaehlung::A, self.ordner_a().to_path_buf()),
                    _ => (Zaehlung::Gross, self.ordner100k().to_path_buf()),
                };
                self.lage = Lage::Liest {
                    t0: Instant::now(),
                    zaehlung,
                    erste_seite: None,
                    bilder: 0,
                };
                Anweisung::Lesen(pfad)
            }
            Schritt::Taste => {
                if zustand.zeilen == 0 {
                    return Anweisung::Abbruch(
                        "die Liste ist leer; ein Tastendruck kann keine Auswahl bewegen".to_owned(),
                    );
                }
                self.lage = Lage::Taste {
                    t0: Instant::now(),
                    auswahl_vorher: zustand.auswahl,
                    bilder: 0,
                };
                Anweisung::Taste
            }
        }
    }

    /// Ob die laufende Messung ueber ihre Geduld hinaus ist.
    ///
    /// Geprueft wird an der Uhr und nicht am Bildzaehler; die Zahl der bisher
    /// eingegangenen Bildgrenzen steht trotzdem in der Meldung, weil sie die
    /// beiden Faelle trennt. Null Bildgrenzen heisst: der `CADisplayLink`
    /// taktet nicht, etwa weil das Fenster verdeckt ist. Viele Bildgrenzen
    /// heissen: die Oberflaeche taktet, kommt aber nicht ans Ziel.
    fn haengt(&self) -> Option<String> {
        let (t0, bilder, was) = match self.lage {
            Lage::Bereit => return None,
            Lage::Liest { t0, bilder, .. } => (t0, bilder, "ein Lesevorgang"),
            Lage::Taste { t0, bilder, .. } => (t0, bilder, "ein Tastendruck"),
        };
        if t0.elapsed() <= GEDULD {
            return None;
        }
        Some(format!(
            "{was} ist nach {} s nicht am Ziel; seit dem Beginn sind {bilder} Bildgrenzen \
             eingegangen{}",
            GEDULD.as_secs(),
            if bilder == 0 {
                ". Der Bildtakt steht, das Fenster ist vermutlich verdeckt oder der \
                 Bildschirm aus"
            } else {
                ""
            }
        ))
    }

    /// Meldet eine Bildgrenze samt dem Zustand der Oberflaeche.
    ///
    /// Liefert `true`, sobald der Lauf beendet ist und der Aufrufer die
    /// Anwendung verlassen soll.
    pub fn bildgrenze(&mut self, jetzt: Instant, zustand: Zustand) -> bool {
        if matches!(self.aufgabe, Aufgabe::Start { .. } | Aufgabe::SitzungsStart) {
            return self.bildgrenze_beim_start(&zustand);
        }
        if matches!(self.aufgabe, Aufgabe::Sitzung { .. }) {
            return self.sitzung_bildgrenze(jetzt, &zustand);
        }

        // Die Lage wird kopiert und am Ende zurueckgeschrieben. So bleibt sie
        // nicht ausgeliehen, waehrend die Werte wachsen und der Schritt
        // weiterrueckt.
        match self.lage {
            Lage::Bereit => false,
            Lage::Liest {
                t0,
                zaehlung,
                mut erste_seite,
                bilder,
            } => {
                if erste_seite.is_none() && zustand.zeilen > 0 {
                    erste_seite = Some(jetzt.saturating_duration_since(t0));
                }
                if zustand.liest {
                    self.lage = Lage::Liest {
                        t0,
                        zaehlung,
                        erste_seite,
                        bilder: bilder + 1,
                    };
                    return false;
                }
                let vollstaendig = jetzt.saturating_duration_since(t0);
                let erste = erste_seite.unwrap_or(vollstaendig);
                match zaehlung {
                    Zaehlung::Keine => {}
                    Zaehlung::A => {
                        self.werte.l2.push(erste);
                        self.werte.l3.push(vollstaendig);
                    }
                    Zaehlung::Gross => {
                        self.werte.l10_erste.push(erste);
                        self.werte.l10_voll.push(vollstaendig);
                    }
                }
                self.weiter();
                false
            }
            Lage::Taste {
                t0,
                auswahl_vorher,
                bilder,
            } => {
                if zustand.auswahl == auswahl_vorher {
                    self.lage = Lage::Taste {
                        t0,
                        auswahl_vorher,
                        bilder: bilder + 1,
                    };
                    return false;
                }
                self.werte.l1.push(jetzt.saturating_duration_since(t0));
                self.weiter();
                false
            }
        }
    }

    /// Die Bildgrenze in den Aufgaben [`Aufgabe::Start`] und
    /// [`Aufgabe::SitzungsStart`].
    ///
    /// Bedienbar heisst hier: das Fenster steht, der Tastenabgriff ist
    /// eingerichtet, und die erste Bildschirmseite ist im Modell, sichtbar
    /// spaetestens mit dieser Bildgrenze. Genau diese Lesart hat der Nutzer
    /// am 260802-1735 entschieden. Auf der Pruefsitzung heisst "jeder
    /// sichtbare Tab": der linke **und** der rechte, wie C8 es fuer L4
    /// ausschreibt.
    fn bildgrenze_beim_start(&mut self, zustand: &Zustand) -> bool {
        let bedienbar = match self.aufgabe {
            Aufgabe::SitzungsStart => {
                zustand.zeilen > 0
                    && zustand
                        .sitzung
                        .as_ref()
                        .is_some_and(|lage| lage.zeilen_rechts > 0)
            }
            _ => zustand.zeilen > 0,
        };
        if self.gemeldet || !bedienbar {
            return false;
        }
        self.gemeldet = true;
        let jetzt = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        self.rate_ausgeben();
        melden(&format!("bedienbar {}", jetzt.as_nanos()));
        true
    }

    /// Ein Takt des Ausloesers auf der Sitzungsstrecke (S21).
    fn sitzung_weiter(&mut self, zustand: &Zustand) -> Anweisung {
        if let Some(grund) = self.sitzung_haengt() {
            return Anweisung::Abbruch(grund);
        }
        match self.sitzungslauf {
            Sitzungslauf::Misst { .. } => return Anweisung::Warten,
            Sitzungslauf::Wartet { .. } => {
                // Steht die Bedingung inzwischen, rueckt der Ablauf weiter;
                // der naechste Takt nimmt den folgenden Schritt.
                if let Some(Sitzungsschritt::Warten(bedingung)) =
                    self.sitzungsschritte.get(self.sitzungsstelle)
                    && bedingung.steht(zustand)
                {
                    self.sitzungslauf = Sitzungslauf::Bereit;
                    self.sitzungsstelle += 1;
                }
                return Anweisung::Warten;
            }
            Sitzungslauf::Bereit => {}
        }
        let Some(schritt) = self.sitzungsschritte.get(self.sitzungsstelle).cloned() else {
            return Anweisung::Fertig;
        };
        match schritt {
            Sitzungsschritt::Warten(bedingung) => {
                if bedingung.steht(zustand) {
                    self.sitzungsstelle += 1;
                } else {
                    self.sitzungslauf = Sitzungslauf::Wartet {
                        seit: Instant::now(),
                        bilder: 0,
                    };
                }
                Anweisung::Warten
            }
            Sitzungsschritt::Handeln(handlung) => {
                self.sitzungsstelle += 1;
                Anweisung::Handeln(handlung)
            }
            Sitzungsschritt::Taste { funktion, messung } => {
                let Some(lage) = zustand.sitzung.as_ref() else {
                    return Anweisung::Abbruch(
                        "die Oberflaeche meldet keine Sitzungslage; die Sitzungsstrecke \
                         kann so nichts messen"
                            .to_owned(),
                    );
                };
                match messung {
                    Some(groesse) => {
                        if let Some(grund) = messung_unmoeglich(groesse, lage) {
                            return Anweisung::Abbruch(grund);
                        }
                        // Der Zeitpunkt liegt **vor** dem AppKit-Aufruf, den
                        // der Aufrufer gleich absetzt; dieselbe Regel wie bei
                        // der Strecke aus S8.
                        self.sitzungslauf = Sitzungslauf::Misst {
                            t0: Instant::now(),
                            groesse,
                            vorher: Vorher::aus(lage),
                            bilder: 0,
                        };
                    }
                    // Ein ungemessener Druck; seine Wirkung faengt der
                    // naechste Warteschritt ab.
                    None => self.sitzungsstelle += 1,
                }
                Anweisung::Funktionstaste(funktion)
            }
            Sitzungsschritt::Aufraeumen => {
                let Aufgabe::Sitzung { plan } = &self.aufgabe else {
                    return Anweisung::Abbruch(
                        "ein Aufraeumschritt ohne Messplan; das ist ein Fehler der Strecke"
                            .to_owned(),
                    );
                };
                if let Err(fehler) = plan.kopierziel_leeren() {
                    return Anweisung::Abbruch(format!(
                        "das Kopierziel liess sich nicht leeren: {fehler}"
                    ));
                }
                self.sitzungsstelle += 1;
                Anweisung::Warten
            }
        }
    }

    /// Ob die Sitzungsstrecke ueber ihre Geduld hinaus ist.
    ///
    /// Dieselbe Uhr-statt-Bildzaehler-Regel wie bei [`Messlauf::haengt`], mit
    /// derselben Trennung der beiden Faelle in der Meldung.
    fn sitzung_haengt(&self) -> Option<String> {
        let (seit, bilder, was) = match &self.sitzungslauf {
            Sitzungslauf::Bereit => return None,
            Sitzungslauf::Wartet { seit, bilder } => (
                *seit,
                *bilder,
                format!(
                    "ein Warteschritt ({:?})",
                    self.sitzungsschritte.get(self.sitzungsstelle)
                ),
            ),
            Sitzungslauf::Misst {
                t0,
                groesse,
                bilder,
                ..
            } => (*t0, *bilder, format!("die Messung {}", groesse.name())),
        };
        if seit.elapsed() <= GEDULD {
            return None;
        }
        Some(format!(
            "{was} ist nach {} s nicht am Ziel; seit dem Beginn sind {bilder} Bildgrenzen \
             eingegangen{}",
            GEDULD.as_secs(),
            if bilder == 0 {
                ". Der Bildtakt steht, das Fenster ist vermutlich verdeckt oder der \
                 Bildschirm aus"
            } else {
                ""
            }
        ))
    }

    /// Die Bildgrenze auf der Sitzungsstrecke: beendet eine laufende Messung.
    fn sitzung_bildgrenze(&mut self, jetzt: Instant, zustand: &Zustand) -> bool {
        match self.sitzungslauf {
            Sitzungslauf::Bereit => {}
            Sitzungslauf::Wartet { seit, bilder } => {
                self.sitzungslauf = Sitzungslauf::Wartet {
                    seit,
                    bilder: bilder + 1,
                };
            }
            Sitzungslauf::Misst {
                t0,
                groesse,
                vorher,
                bilder,
            } => {
                let fertig = match (&self.aufgabe, zustand.sitzung.as_ref()) {
                    (Aufgabe::Sitzung { plan }, Some(lage)) => {
                        sitzungsmessung_fertig(groesse, vorher, lage, &plan.unterordner)
                    }
                    _ => false,
                };
                if fertig {
                    self.sitzungswerte
                        .eintragen(groesse, jetzt.saturating_duration_since(t0));
                    self.sitzungsstelle += 1;
                    self.sitzungslauf = Sitzungslauf::Bereit;
                } else {
                    self.sitzungslauf = Sitzungslauf::Misst {
                        t0,
                        groesse,
                        vorher,
                        bilder: bilder + 1,
                    };
                }
            }
        }
        false
    }

    /// Ein Schritt ist erledigt.
    fn weiter(&mut self) {
        self.stelle += 1;
        self.lage = Lage::Bereit;
    }

    fn ordner_a(&self) -> &Path {
        match &self.aufgabe {
            Aufgabe::Spannen { ordner_a, .. } => ordner_a,
            Aufgabe::Start { ordner } => ordner,
            // Die Sitzungsstrecke laeuft nicht ueber diese Schritte.
            Aufgabe::Sitzung { plan } => plan.ordner_a(),
            Aufgabe::SitzungsStart => Path::new("/"),
        }
    }

    fn ordner100k(&self) -> &Path {
        match &self.aufgabe {
            Aufgabe::Spannen { ordner100k, .. } => ordner100k,
            Aufgabe::Start { ordner } => ordner,
            // Die Sitzungsstrecke laeuft nicht ueber diese Schritte.
            Aufgabe::Sitzung { plan } => plan.ordner_a(),
            Aufgabe::SitzungsStart => Path::new("/"),
        }
    }

    /// Schreibt die Bildwiederholrate auf die Standardausgabe.
    fn rate_ausgeben(&self) {
        match self.bildwiederholrate {
            Some(hertz) => melden(&format!("bildwiederholrate {hertz}")),
            // Erreichbar ist das nicht: ohne Rate bricht der Aufrufer ab, bevor
            // ein Messlauf beginnt. Eine erfundene 60 kaeme trotzdem nicht in
            // Frage.
            None => melden("bildwiederholrate unbekannt"),
        }
    }

    /// Schreibt jeden gemessenen Einzelwert auf die Standardausgabe.
    ///
    /// Ausgewertet wird woanders. Der Grund ist die Aufteilung der Strecke:
    /// L4 misst der aeussere Aufrufer ueber zwanzig Prozessstarts, L1 bis L10
    /// misst dieser Prozess, und **ein** Bericht ueber alle fuenf Zusagen
    /// entsteht nur, wo beide Haelften zusammenkommen. Das Perzentil und der
    /// Berichtskopf liegen deshalb in `krk-bench`, das beides schon fuehrt; ein
    /// zweiter Berichtsschreiber daneben waere eine zweite Wahrheit.
    pub fn ausgeben(&self) {
        self.rate_ausgeben();
        if matches!(self.aufgabe, Aufgabe::Sitzung { .. }) {
            for (name, werte) in self.sitzungswerte.alle() {
                for wert in werte {
                    melden(&format!("wert {name} {}", wert.as_nanos()));
                }
            }
        } else {
            for (name, werte) in [
                ("l1", &self.werte.l1),
                ("l2", &self.werte.l2),
                ("l3", &self.werte.l3),
                ("l10-erste", &self.werte.l10_erste),
                ("l10-voll", &self.werte.l10_voll),
            ] {
                for wert in werte {
                    melden(&format!("wert {name} {}", wert.as_nanos()));
                }
            }
        }
        melden("fertig");
        let _ = std::io::stdout().flush();
    }
}

/// Das Praefix, an dem der aeussere Aufrufer eine Messzeile erkennt.
///
/// Die Anwendung schreibt auch anderes auf die Standardausgabe, etwa das
/// Tastenprotokoll. Ein Praefix trennt das Maschinenlesbare vom Uebrigen,
/// statt den Leser raten zu lassen.
pub const PRAEFIX: &str = "krk-messung";

fn melden(zeile: &str) {
    println!("{PRAEFIX} {zeile}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worte(zeile: &[&str]) -> Vec<String> {
        zeile.iter().map(|wort| (*wort).to_owned()).collect()
    }

    #[test]
    fn ohne_die_marke_gibt_es_keine_aufgabe() {
        assert_eq!(
            Aufgabe::aus_argumenten(&worte(&["--tasten-protokoll"])),
            Ok(None)
        );
        assert_eq!(Aufgabe::aus_argumenten(&[]), Ok(None));
    }

    #[test]
    fn die_startaufgabe_braucht_ihren_ordner() {
        assert_eq!(
            Aufgabe::aus_argumenten(&worte(&["--messmodus", "start", "--ordner", "/tmp/a"])),
            Ok(Some(Aufgabe::Start {
                ordner: PathBuf::from("/tmp/a")
            }))
        );
        assert!(Aufgabe::aus_argumenten(&worte(&["--messmodus", "start"])).is_err());
    }

    #[test]
    fn die_spannenaufgabe_braucht_beide_ordner() {
        assert_eq!(
            Aufgabe::aus_argumenten(&worte(&[
                "--messmodus",
                "spannen",
                "--ordner-a",
                "/tmp/a",
                "--ordner100k",
                "/tmp/gross"
            ])),
            Ok(Some(Aufgabe::Spannen {
                ordner_a: PathBuf::from("/tmp/a"),
                ordner100k: PathBuf::from("/tmp/gross")
            }))
        );
        assert!(
            Aufgabe::aus_argumenten(&worte(&["--messmodus", "spannen", "--ordner-a", "/tmp/a"]))
                .is_err()
        );
    }

    #[test]
    fn eine_unbekannte_aufgabe_ist_ein_fehler() {
        assert!(Aufgabe::aus_argumenten(&worte(&["--messmodus", "alles"])).is_err());
        assert!(Aufgabe::aus_argumenten(&worte(&["--messmodus"])).is_err());
    }

    fn spannenlauf() -> Messlauf {
        Messlauf::neu(Aufgabe::Spannen {
            ordner_a: PathBuf::from("/tmp/a"),
            ordner100k: PathBuf::from("/tmp/gross"),
        })
    }

    #[test]
    fn der_ablauf_traegt_drei_vorlaeufe_und_dreimal_zwanzig_messungen() {
        let lauf = spannenlauf();
        let zahl = |gesucht: &Schritt| lauf.schritte.iter().filter(|s| *s == gesucht).count();
        assert_eq!(zahl(&Schritt::LesenA), WIEDERHOLUNGEN);
        assert_eq!(zahl(&Schritt::Lesen100k), WIEDERHOLUNGEN);
        assert_eq!(zahl(&Schritt::Taste), WIEDERHOLUNGEN);
        assert_eq!(lauf.schritte.len(), 3 * WIEDERHOLUNGEN + 3);
        assert_eq!(
            lauf.schritte.first(),
            Some(&Schritt::Vorlauf(PathBuf::from("/tmp/a"))),
            "ohne Vorlauf traegt der erste Lauf eine kalte Zahl in eine warme Reihe"
        );
    }

    /// Ein Lesevorgang von Anfang bis Ende, Bildgrenze fuer Bildgrenze.
    #[test]
    fn ein_lesevorgang_liefert_erste_seite_und_vollstaendiges_lesen() {
        let mut lauf = spannenlauf();
        // Den Vorlauf abarbeiten.
        assert!(matches!(
            lauf.naechster_schritt(leer()),
            Anweisung::Lesen(_)
        ));
        lauf.bildgrenze(Instant::now(), fertig(10));
        assert_eq!(lauf.stelle, 1);

        // Die erste gezaehlte Messung.
        assert!(matches!(
            lauf.naechster_schritt(fertig(10)),
            Anweisung::Lesen(_)
        ));
        let start = Instant::now();
        // Bild 1: noch keine Zeile.
        assert!(!lauf.bildgrenze(start + ms(16), leer_und_liest()));
        // Bild 2: erste Zeilen stehen, es wird noch gelesen.
        assert!(!lauf.bildgrenze(start + ms(33), liest_mit(500)));
        // Bild 3: fertig.
        assert!(!lauf.bildgrenze(start + ms(66), fertig(10_000)));

        assert_eq!(lauf.werte.l2.len(), 1);
        assert_eq!(lauf.werte.l3.len(), 1);
        assert!(
            lauf.werte.l2[0] < lauf.werte.l3[0],
            "die erste Seite kam nach dem vollstaendigen Lesen"
        );
        assert_eq!(lauf.lage, Lage::Bereit);
    }

    #[test]
    fn eine_laufende_messung_nimmt_keinen_neuen_schritt_an() {
        let mut lauf = spannenlauf();
        assert!(matches!(
            lauf.naechster_schritt(leer()),
            Anweisung::Lesen(_)
        ));
        assert_eq!(lauf.naechster_schritt(liest_mit(5)), Anweisung::Warten);
    }

    #[test]
    fn ein_tastendruck_zaehlt_erst_wenn_die_auswahl_umspringt() {
        let mut lauf = spannenlauf();
        // Bis zum ersten Tastenschritt vorspulen.
        lauf.stelle = lauf.schritte.len() - WIEDERHOLUNGEN;
        assert_eq!(lauf.naechster_schritt(fertig(10_000)), Anweisung::Taste);
        let start = Instant::now();
        assert!(!lauf.bildgrenze(start + ms(8), fertig(10_000)));
        assert_eq!(lauf.werte.l1.len(), 0, "ohne Umsprung darf nichts zaehlen");
        assert!(!lauf.bildgrenze(start + ms(16), ausgewaehlt(0)));
        assert_eq!(lauf.werte.l1.len(), 1);
    }

    #[test]
    fn ein_leerer_ordner_laesst_keinen_tastendruck_zu() {
        let mut lauf = spannenlauf();
        lauf.stelle = lauf.schritte.len() - WIEDERHOLUNGEN;
        assert!(matches!(
            lauf.naechster_schritt(leer()),
            Anweisung::Abbruch(_)
        ));
    }

    #[test]
    fn nach_dem_letzten_schritt_ist_der_lauf_fertig() {
        let mut lauf = spannenlauf();
        lauf.stelle = lauf.schritte.len();
        assert_eq!(lauf.naechster_schritt(fertig(10)), Anweisung::Fertig);
    }

    #[test]
    fn die_startaufgabe_meldet_genau_einmal() {
        let mut lauf = Messlauf::neu(Aufgabe::Start {
            ordner: PathBuf::from("/tmp/a"),
        });
        lauf.rate_setzen(60);
        assert!(!lauf.bildgrenze(Instant::now(), leer()));
        assert!(lauf.bildgrenze(Instant::now(), fertig(10)));
        assert!(!lauf.bildgrenze(Instant::now(), fertig(10)));
    }

    /// Der Ausloesetakt darf der Startaufgabe nicht zuvorkommen.
    #[test]
    fn die_startaufgabe_wartet_auf_die_bildgrenze() {
        let mut lauf = Messlauf::neu(Aufgabe::Start {
            ordner: PathBuf::from("/tmp/a"),
        });
        for _ in 0..3 {
            assert_eq!(
                lauf.naechster_schritt(leer()),
                Anweisung::Warten,
                "der Ausloesetakt hat die Startaufgabe vorzeitig beendet"
            );
        }
    }

    #[test]
    fn eine_stehende_messung_wird_an_der_uhr_abgebrochen_und_nicht_am_bildzaehler() {
        let mut lauf = spannenlauf();
        assert!(matches!(
            lauf.naechster_schritt(leer()),
            Anweisung::Lesen(_)
        ));
        // Noch keine einzige Bildgrenze eingegangen, und trotzdem greift die
        // Schranke, sobald die Geduld um ist.
        assert_eq!(lauf.haengt(), None);
        lauf.lage = Lage::Liest {
            t0: Instant::now() - GEDULD - Duration::from_secs(1),
            zaehlung: Zaehlung::A,
            erste_seite: None,
            bilder: 0,
        };
        let grund = lauf.haengt().expect("die Schranke haette greifen muessen");
        assert!(
            grund.contains("0 Bildgrenzen") && grund.contains("Bildtakt steht"),
            "die Meldung trennt die beiden Faelle nicht: {grund}"
        );
        assert!(matches!(
            lauf.naechster_schritt(leer()),
            Anweisung::Abbruch(_)
        ));
    }

    fn ms(zahl: u64) -> Duration {
        Duration::from_millis(zahl)
    }

    fn leer() -> Zustand {
        Zustand {
            zeilen: 0,
            liest: false,
            auswahl: -1,
            sitzung: None,
        }
    }

    fn leer_und_liest() -> Zustand {
        Zustand {
            zeilen: 0,
            liest: true,
            auswahl: -1,
            sitzung: None,
        }
    }

    fn liest_mit(zeilen: usize) -> Zustand {
        Zustand {
            zeilen,
            liest: true,
            auswahl: -1,
            sitzung: None,
        }
    }

    fn fertig(zeilen: usize) -> Zustand {
        Zustand {
            zeilen,
            liest: false,
            auswahl: -1,
            sitzung: None,
        }
    }

    fn ausgewaehlt(zeile: isize) -> Zustand {
        Zustand {
            zeilen: 10_000,
            liest: false,
            auswahl: zeile,
            sitzung: None,
        }
    }

    // ------------------------------------------------------------------
    // Die Sitzungsstrecke (S21)
    // ------------------------------------------------------------------

    use krk_core::ablage::sitzung::{Dateifenster, Tab};
    use std::fs;

    /// Ein Wegwerf-Wurzelordner mit den vier Ordnern eines Messplans.
    struct Planordner {
        wurzel: PathBuf,
    }

    impl Planordner {
        fn neu(zweck: &str) -> Self {
            let laufnummer = ZAEHLER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let wurzel = std::env::temp_dir().join(format!(
                "krk-messmodus-{zweck}-{}-{laufnummer}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&wurzel);
            for unter in ["a", "b", "a-l6", "ziel"] {
                fs::create_dir_all(wurzel.join(unter)).expect("Anlegen gescheitert");
            }
            Self { wurzel }
        }

        fn plan(&self) -> Messplan {
            Messplan {
                kopierziel: self.wurzel.join("ziel"),
                unterordner: self.wurzel.join("a-l6"),
                sitzung: pruefsitzung(self.wurzel.join("a"), self.wurzel.join("b")),
            }
        }
    }

    impl Drop for Planordner {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.wurzel);
        }
    }

    static ZAEHLER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    /// Die Pruefsitzung aus C8, wie das Messwerkzeug sie in den Plan schreibt.
    fn pruefsitzung(a: PathBuf, b: PathBuf) -> Sitzung {
        Sitzung {
            fenster: [
                Dateifenster {
                    aktiver_tab: 0,
                    tabs: vec![Tab::auf(&a), Tab::auf(&b)],
                },
                Dateifenster {
                    aktiver_tab: 0,
                    tabs: vec![Tab::auf(&b), Tab::auf(&a)],
                },
            ],
            ..Sitzung::default()
        }
    }

    /// Eine Sitzungslage mit unauffaelligen Werten, zum Abwandeln je Pruefung.
    fn lage(ordner: &Planordner) -> Sitzungslage {
        Sitzungslage {
            im_vordergrund: true,
            aktiv_links: true,
            zeilen_aktiv: 10_000,
            liest_aktiv: false,
            auswahl_aktiv: 0,
            tab_aktiv: 0,
            ordner_aktiv: ordner.wurzel.join("a"),
            auswahl_pfad: Some(ordner.wurzel.join("a/datei-1")),
            zeilen_rechts: 10_000,
            liest_rechts: false,
            ordner_rechts: ordner.wurzel.join("b"),
            vorschau_pfad: None,
            vorschau_laedt: false,
            vorgang_sichtbar: false,
            vorgang_laeuft: false,
        }
    }

    fn mit_lage(lage: Sitzungslage) -> Zustand {
        Zustand {
            zeilen: 10_000,
            liest: false,
            auswahl: 0,
            sitzung: Some(lage),
        }
    }

    #[test]
    fn die_sitzungsaufgabe_kommt_aus_einem_planpfad() {
        assert_eq!(
            Aufgabe::aus_argumenten(&worte(&["--messmodus", "sitzungsstart"])),
            Ok(Some(Aufgabe::SitzungsStart))
        );
        // Ein Pfad, den es nicht gibt, ist ein Fehler und kein gewoehnlicher
        // Start.
        assert!(
            Aufgabe::aus_argumenten(&worte(&["--messmodus", "/gibt/es/nicht/plan.toml"])).is_err()
        );
    }

    #[test]
    fn der_messplan_liest_das_format_des_werkzeugs() {
        let ordner = Planordner::neu("format");
        // Derselbe Aufbau, den `krk-bench` schreibt: zwei Pfadschluessel und
        // der Abschnitt [sitzung] in der Serialisierung von session.toml.
        let mut wurzel = toml::Table::new();
        wurzel.insert(
            "kopierziel".to_owned(),
            toml::Value::String(ordner.wurzel.join("ziel").display().to_string()),
        );
        wurzel.insert(
            "unterordner".to_owned(),
            toml::Value::String(ordner.wurzel.join("a-l6").display().to_string()),
        );
        wurzel.insert(
            "sitzung".to_owned(),
            toml::Value::try_from(pruefsitzung(
                ordner.wurzel.join("a"),
                ordner.wurzel.join("b"),
            ))
            .expect("die Sitzung ist serialisierbar"),
        );
        let pfad = ordner.wurzel.join("plan.toml");
        fs::write(&pfad, toml::to_string(&wurzel).expect("schreibbar")).expect("schreibbar");

        let plan = Messplan::lesen(&pfad).expect("der Plan ist lesbar");
        assert_eq!(plan.ordner_a(), ordner.wurzel.join("a"));
        assert_eq!(plan.kopierziel, ordner.wurzel.join("ziel"));
    }

    #[test]
    fn ein_gefuelltes_kopierziel_wird_abgewiesen() {
        let ordner = Planordner::neu("volles-ziel");
        fs::write(ordner.wurzel.join("ziel/fremd.txt"), "fremd").expect("schreibbar");
        let fehler = ordner
            .plan()
            .pruefen()
            .expect_err("haette scheitern muessen");
        assert!(
            fehler.contains("nicht leer"),
            "unerwartete Meldung: {fehler}"
        );
    }

    #[test]
    fn die_schrittliste_traegt_je_groesse_zwanzig_messungen() {
        let ordner = Planordner::neu("schritte");
        let schritte = sitzungsschritte(&ordner.plan());
        let zahl = |gesucht: Sitzungsgroesse| {
            schritte
                .iter()
                .filter(|schritt| {
                    matches!(schritt, Sitzungsschritt::Taste { messung: Some(groesse), .. }
                        if *groesse == gesucht)
                })
                .count()
        };
        for groesse in [
            Sitzungsgroesse::L1,
            Sitzungsgroesse::L5Tab,
            Sitzungsgroesse::L5Fenster,
            Sitzungsgroesse::L6,
            Sitzungsgroesse::L7,
            Sitzungsgroesse::L8,
            Sitzungsgroesse::L9,
        ] {
            assert_eq!(
                zahl(groesse),
                WIEDERHOLUNGEN,
                "{} hat die falsche Zahl an Messungen",
                groesse.name()
            );
        }
        // Je Kopie ein Aufraeumschritt, sonst misst die naechste Wiederholung
        // einen Konfliktfall statt einer Kopie.
        assert_eq!(
            schritte
                .iter()
                .filter(|schritt| **schritt == Sitzungsschritt::Aufraeumen)
                .count(),
            WIEDERHOLUNGEN
        );
    }

    /// Im Hintergrund bricht die Strecke ab, statt eine Groesse anzuklagen.
    ///
    /// Die Pruefung geht ueber **jede** Messgroesse und nicht nur ueber die,
    /// die am 260806 als erste darauf traf: der Vorbehalt gehoert der Strecke,
    /// und L1 und L7 kamen nur deshalb durch, weil `auswahl_runter` als
    /// einziger Befehl der Strecke ohne Wirkungsbereich auskommt.
    #[test]
    fn im_hintergrund_beginnt_keine_messung() {
        let ordner = Planordner::neu("hintergrund");
        let hintergrund = Sitzungslage {
            im_vordergrund: false,
            ..lage(&ordner)
        };
        for groesse in [
            Sitzungsgroesse::L1,
            Sitzungsgroesse::L5Tab,
            Sitzungsgroesse::L5Fenster,
            Sitzungsgroesse::L6,
            Sitzungsgroesse::L7,
            Sitzungsgroesse::L8,
            Sitzungsgroesse::L9,
        ] {
            assert_eq!(
                messung_unmoeglich(groesse, &hintergrund).as_deref(),
                Some(NICHT_IM_VORDERGRUND),
                "{} beginnt im Hintergrund trotzdem",
                groesse.name()
            );
        }
        // Die Gegenprobe: im Vordergrund steht keiner dieser Groessen der
        // Vorbehalt im Weg.
        let vordergrund = lage(&ordner);
        for groesse in [
            Sitzungsgroesse::L1,
            Sitzungsgroesse::L5Tab,
            Sitzungsgroesse::L5Fenster,
            Sitzungsgroesse::L6,
            Sitzungsgroesse::L7,
            Sitzungsgroesse::L8,
        ] {
            assert_eq!(
                messung_unmoeglich(groesse, &vordergrund),
                None,
                "{} wird im Vordergrund abgewiesen",
                groesse.name()
            );
        }
    }

    /// Der Abbruch faellt am ersten Tastenschritt und nicht erst nach zehn
    /// Sekunden Geduld.
    #[test]
    fn der_hintergrund_bricht_die_erste_messung_ab() {
        let ordner = Planordner::neu("hintergrund-abbruch");
        let mut lauf = Messlauf::neu(Aufgabe::Sitzung {
            plan: ordner.plan(),
        });
        lauf.sitzungsstelle = lauf
            .sitzungsschritte
            .iter()
            .position(|schritt| {
                matches!(
                    schritt,
                    Sitzungsschritt::Taste {
                        messung: Some(Sitzungsgroesse::L1),
                        ..
                    }
                )
            })
            .expect("es gibt eine L1-Messung");

        let hintergrund = mit_lage(Sitzungslage {
            im_vordergrund: false,
            ..lage(&ordner)
        });
        assert_eq!(
            lauf.naechster_schritt(hintergrund),
            Anweisung::Abbruch(NICHT_IM_VORDERGRUND.to_owned())
        );
    }

    #[test]
    fn eine_l1_messung_endet_mit_der_umgesprungenen_auswahl() {
        let ordner = Planordner::neu("l1");
        let mut lauf = Messlauf::neu(Aufgabe::Sitzung {
            plan: ordner.plan(),
        });
        // Bis zur ersten L1-Messung vorspulen.
        lauf.sitzungsstelle = lauf
            .sitzungsschritte
            .iter()
            .position(|schritt| {
                matches!(
                    schritt,
                    Sitzungsschritt::Taste {
                        messung: Some(Sitzungsgroesse::L1),
                        ..
                    }
                )
            })
            .expect("es gibt eine L1-Messung");

        let vorher = mit_lage(lage(&ordner));
        assert_eq!(
            lauf.naechster_schritt(vorher.clone()),
            Anweisung::Funktionstaste("auswahl_runter")
        );
        // Dieselbe Auswahl: die Messung laeuft weiter.
        assert!(!lauf.bildgrenze(Instant::now(), vorher));
        assert_eq!(lauf.sitzungswerte.l1.len(), 0);
        // Die Auswahl ist umgesprungen: der Wert steht.
        let nachher = mit_lage(Sitzungslage {
            auswahl_aktiv: 1,
            ..lage(&ordner)
        });
        assert!(!lauf.bildgrenze(Instant::now(), nachher));
        assert_eq!(lauf.sitzungswerte.l1.len(), 1);
        assert_eq!(lauf.sitzungslauf, Sitzungslauf::Bereit);
    }

    #[test]
    fn eine_l8_messung_endet_mit_der_sichtbaren_vorgangsanzeige() {
        let ordner = Planordner::neu("l8");
        let mut lauf = Messlauf::neu(Aufgabe::Sitzung {
            plan: ordner.plan(),
        });
        lauf.sitzungsstelle = lauf
            .sitzungsschritte
            .iter()
            .position(|schritt| {
                matches!(
                    schritt,
                    Sitzungsschritt::Taste {
                        messung: Some(Sitzungsgroesse::L8),
                        ..
                    }
                )
            })
            .expect("es gibt eine L8-Messung");

        assert_eq!(
            lauf.naechster_schritt(mit_lage(lage(&ordner))),
            Anweisung::Funktionstaste("kopieren")
        );
        assert!(!lauf.bildgrenze(Instant::now(), mit_lage(lage(&ordner))));
        assert_eq!(lauf.sitzungswerte.l8.len(), 0);
        let sichtbar = mit_lage(Sitzungslage {
            vorgang_sichtbar: true,
            vorgang_laeuft: true,
            ..lage(&ordner)
        });
        assert!(!lauf.bildgrenze(Instant::now(), sichtbar));
        assert_eq!(lauf.sitzungswerte.l8.len(), 1);
    }

    #[test]
    fn l9_bricht_ab_wenn_keine_kopie_mehr_laeuft() {
        let ordner = Planordner::neu("l9");
        let mut lauf = Messlauf::neu(Aufgabe::Sitzung {
            plan: ordner.plan(),
        });
        lauf.sitzungsstelle = lauf
            .sitzungsschritte
            .iter()
            .position(|schritt| {
                matches!(
                    schritt,
                    Sitzungsschritt::Taste {
                        messung: Some(Sitzungsgroesse::L9),
                        ..
                    }
                )
            })
            .expect("es gibt eine L9-Messung");
        assert!(matches!(
            lauf.naechster_schritt(mit_lage(lage(&ordner))),
            Anweisung::Abbruch(_)
        ));
    }

    #[test]
    fn ein_warteschritt_haelt_bis_seine_bedingung_steht() {
        let ordner = Planordner::neu("warten");
        let mut lauf = Messlauf::neu(Aufgabe::Sitzung {
            plan: ordner.plan(),
        });
        // Der erste Schritt wartet auf den gelesenen linken Tab.
        let noch_leer = Zustand {
            zeilen: 0,
            liest: true,
            auswahl: -1,
            sitzung: Some(lage(&ordner)),
        };
        assert_eq!(lauf.naechster_schritt(noch_leer.clone()), Anweisung::Warten);
        assert!(matches!(lauf.sitzungslauf, Sitzungslauf::Wartet { .. }));
        assert_eq!(lauf.sitzungsstelle, 0);
        // Die Bedingung steht: der Ablauf rueckt weiter.
        assert_eq!(
            lauf.naechster_schritt(mit_lage(lage(&ordner))),
            Anweisung::Warten
        );
        assert_eq!(lauf.sitzungsstelle, 1);
        assert_eq!(lauf.sitzungslauf, Sitzungslauf::Bereit);
    }

    #[test]
    fn die_l6_messung_endet_im_vollstaendig_gelesenen_unterordner() {
        let ordner = Planordner::neu("l6");
        let plan = ordner.plan();
        let vorher = Vorher {
            auswahl: 0,
            tab: 0,
            aktiv_links: true,
        };
        let drin = Sitzungslage {
            ordner_aktiv: plan.unterordner.clone(),
            zeilen_aktiv: 1_000,
            ..lage(&ordner)
        };
        assert!(sitzungsmessung_fertig(
            Sitzungsgroesse::L6,
            vorher,
            &drin,
            &plan.unterordner
        ));
        // Noch lesend zaehlt nicht: "vollstaendig sichtbar" heisst gelesen.
        let liest_noch = Sitzungslage {
            liest_aktiv: true,
            ..drin
        };
        assert!(!sitzungsmessung_fertig(
            Sitzungsgroesse::L6,
            vorher,
            &liest_noch,
            &plan.unterordner
        ));
    }

    #[test]
    fn die_l7_messung_wartet_auf_die_vorschau_des_neuen_eintrags() {
        let ordner = Planordner::neu("l7");
        let plan = ordner.plan();
        let vorher = Vorher {
            auswahl: 0,
            tab: 0,
            aktiv_links: true,
        };
        let pfad = ordner.wurzel.join("a/datei-2");
        let umgesprungen_ohne_vorschau = Sitzungslage {
            auswahl_aktiv: 1,
            auswahl_pfad: Some(pfad.clone()),
            vorschau_pfad: None,
            ..lage(&ordner)
        };
        assert!(!sitzungsmessung_fertig(
            Sitzungsgroesse::L7,
            vorher,
            &umgesprungen_ohne_vorschau,
            &plan.unterordner
        ));
        let vorschau_da = Sitzungslage {
            vorschau_pfad: Some(pfad),
            ..umgesprungen_ohne_vorschau
        };
        assert!(sitzungsmessung_fertig(
            Sitzungsgroesse::L7,
            vorher,
            &vorschau_da,
            &plan.unterordner
        ));
    }

    #[test]
    fn der_sitzungsstart_meldet_erst_wenn_beide_seiten_stehen() {
        let ordner = Planordner::neu("start");
        let mut lauf = Messlauf::neu(Aufgabe::SitzungsStart);
        lauf.rate_setzen(60);
        // Links steht, rechts noch nicht: keine Meldung.
        let rechts_leer = mit_lage(Sitzungslage {
            zeilen_rechts: 0,
            ..lage(&ordner)
        });
        assert!(!lauf.bildgrenze(Instant::now(), rechts_leer));
        // Beide sichtbaren Tabs zeigen ihre erste Bildschirmseite: gemeldet.
        assert!(lauf.bildgrenze(Instant::now(), mit_lage(lage(&ordner))));
        assert!(!lauf.bildgrenze(Instant::now(), mit_lage(lage(&ordner))));
    }
}
