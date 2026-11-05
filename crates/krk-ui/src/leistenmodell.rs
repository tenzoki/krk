//! Das Leistenmodell: was in der Lesezeichen- und Geraeteleiste steht, in
//! welcher Reihenfolge, und welche Zeile ausgewaehlt ist (C5).
//!
//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile. Die
//! Ansicht dazu ist `crate::appkit::leiste`, die aus den Zeilen hier eine
//! `NSTableView` macht; die Datentraeger, die den unteren Teil fuellen, zaehlt
//! `crate::appkit::volumes` auf und reicht sie als gewoehnliche Rust-Werte
//! herein.
//!
//! # Drei Teile, eine Liste
//!
//! ```text
//! ┌───────────────────────┐
//! │ Lesezeichen           │  Ueberschrift, nicht waehlbar
//! │  📁 Projekte          │  aus bookmarks.toml, Reihenfolge = Reihenfolge
//! │  📄 Die Lesestelle    │  eine Textmarke aus C6, dieselbe Liste
//! │  📁 Sicherung (fehlt) │  ungueltig: der Ordner ist fort
//! │ Verbindungen          │  Ueberschrift, nur mit mindestens einer Verbindung
//! │  🌐 NAS Büro          │  eine Verbindung (261016), dieselbe Liste
//! │ Geräte und Orte       │  Ueberschrift, nicht waehlbar
//! │  📁 k1                │  das Benutzerverzeichnis
//! │  📁 Macintosh HD      │  aus NSFileManager.mountedVolumeURLs…
//! └───────────────────────┘
//! ```
//!
//! Das Sinnbild vor der Beschriftung kommt aus [`Leistenmodell::sinnbild`] und
//! sagt, **was die Zeile oeffnet**: einen Ordner oder eine Stelle in einer
//! Datei. Welches Systemsinnbild das ist, entscheidet die Ansicht; hier steht
//! die Sorte und kein Bildname.
//!
//! **Die Verbindungen stehen in derselben Lesezeichenliste und bilden hier
//! eine eigene Gruppe** (261016, Entscheidung 7 des Plans
//! `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`).
//! Welche Gruppe ein Eintrag hat, sagt `Ziel::gruppe` und kein Feld daneben;
//! ihre Ueberschrift steht, sobald mindestens eine Verbindung abgelegt ist, und
//! fehlt sonst, damit eine Leiste ohne Verbindungen aussieht wie vorher.
//! [`Zeile::Lesezeichen`] traegt weiter die Stelle in der **einen** Liste; die
//! Zeilen einer Gruppe sind die Eintraege dieser Gruppe in der Reihenfolge der
//! Liste.
//!
//! C5 verlangt "trennt Lesezeichen sichtbar von Geraeten und Standardorten".
//! Zwei Tabellen untereinander waeren die andere Antwort; sie haetten zwei
//! Auswahlen, zwei Fokuszustaende und zwei Wege fuer den Auf- und den Ab-Pfeil
//! gehabt. Eine Liste mit zwei Ueberschriften hat eine Auswahl, und die
//! Ueberschriften sind Zeilen, die sie ueberspringt.
//!
//! # Woher die Eintraege kommen
//!
//! Die Lesezeichen kommen aus `bookmarks.toml` und gehoeren dem Nutzer; die
//! Liste selbst wohnt in [`krk_core::ablage::Lesezeichenliste`], weil sie
//! abgelegt wird. Die Geraete erfragt die Anwendung auf einem Faden beim System
//! und reicht sie herein ([`Leistenmodell::orte_setzen`]); abgelegt werden sie
//! nie: was eingehaengt ist, weiss das System besser als eine Datei von gestern.
//! Bleibt die Antwort aus oder scheitert die Frage, gilt der letzte Stand, nach
//! dem Start also das Benutzerverzeichnis ohne Datentraeger.
//!
//! # Die Marke „fehlt“ kommt vom Faden
//!
//! **Dieses Modell fragt die Platte nicht, ausser in den zwei Funktionen, die
//! auf einem Faden laufen** ([`Gueltigkeitsfrage::beantworten`] und
//! [`Auswahl::nachgefragt`]). Bis zum 261105 setzte es die Marke beim Aufbau
//! und nach jeder Geraetemeldung mit einem `is_dir` oder `is_file` je
//! Lesezeichen auf dem Hauptfaden, und ein Lesezeichen auf einer haengenden
//! Freigabe hielt damit die Ereignisschleife an.
//!
//! ```text
//!  lesezeichen_setzen, orte_setzen   setzen allein Liste und Zeilen
//!  gueltigkeit_anstossen             je Plattengruppe hoechstens eine Frage
//!    Faden krk-plattenfrage          Gueltigkeitsfrage::beantworten
//!  gueltigkeit_uebernehmen           Marken nach Ziel, dazu die Nachfrage
//! ```
//!
//! **Bis zur Antwort gilt der letzte Stand**, und ein Eintrag ohne Stand gilt
//! als gueltig: beim Start und direkt nach dem Hinzufuegen. Ein Ziel, das
//! fehlt, steht also bis zur Antwort ohne den Zusatz „(fehlt)“ da; andersherum
//! stuende bei jedem Start jede Zeile kurz als kaputt da. Was eine Auswahl
//! dann tut, entscheidet die Marke nicht: sie fragt ihr eines Ziel selbst
//! ([`Auswahl::nachgefragt`]).
//!
//! **Eine Antwort gilt je Ziel und nicht je Stelle.** Die Liste kann sich
//! geaendert haben, seit die Frage gestellt ist; ein entfernter Eintrag findet
//! dann keinen Abnehmer, ein neuer bekommt nichts, und ein umbenannter oder
//! verschobener behaelt, was sein Ziel sagt.
//!
//! **Was eine haengende Freigabe kostet**, steht bei [`plattengruppe`].
//!
//! # Eine Zeile kann gewaehlt sein, ohne dass ihre Auswahl gewirkt hat
//!
//! Die Auswahl eines Lesezeichens wartet auf ihre Frage an die Platte, und die
//! Antwort kann verfallen, weil der Nutzer inzwischen Tab oder Dateifenster
//! gewechselt hat. Die Zeile bleibt dann gewaehlt, damit er sieht, worauf er
//! geklickt hat, und **ein Klick auf sie loest die Auswahl erneut aus**. Nach einer
//! vollzogenen Auswahl tut derselbe Klick nichts.
//!
//! ```text
//!  auswahl_zum_melden   die Auswahl ist offen, bis sie vollzogen ist
//!  auswahl_vollzogen    nur die juengste Antwort kommt hierher
//!  klick_loest_aus      offen und dieselbe Zeile: noch einmal melden
//! ```
//!
//! **Eine verworfene und eine ausstehende Antwort sind hier dasselbe**: es gibt
//! keinen Uebergang „verworfen“, denn eine verfallene Antwort aendert nichts. Ein
//! Klick bei noch ausstehender Antwort zieht beim Delegierten eine neue Nummer; die
//! erste Antwort verfaellt, und die juengste gilt.
//!
//! **Offen ist ein Ziel und keine Zeile**, aus dem Grund, aus dem auch eine Antwort
//! je Ziel gilt: die Liste kann sich aendern, und unter derselben Stelle steht dann
//! ein anderer Eintrag, dessen Auswahl niemand gemeldet hat.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use krk_core::ablage::{Gruppe, Lesezeichen, Lesezeichenliste, Ziel};
use krk_core::sprache::{Text, satz, text};

/// Die Ueberschrift des oberen Teils, als Schluessel der Sprachtabelle.
pub const UEBERSCHRIFT_LESEZEICHEN: Text = Text::LeisteUeberschriftLesezeichen;

/// Die Ueberschrift des unteren Teils, als Schluessel der Sprachtabelle.
pub const UEBERSCHRIFT_GERAETE: Text = Text::LeisteUeberschriftGeraete;

/// Die Ueberschrift der Verbindungen (261016), als Schluessel der
/// Sprachtabelle.
pub const UEBERSCHRIFT_VERBINDUNGEN: Text = Text::LeisteUeberschriftVerbindungen;

/// Die Zeile eines ungueltigen Lesezeichens in der Leiste: der Name mit einem
/// Zusatz (C5), als Schluessel der Sprachtabelle mit dem Platzhalter `{name}`.
///
/// Ein Wort und keine Farbe allein: die Leiste faerbt die Zeile zusaetzlich
/// grau, aber eine Farbe ist bei Farbfehlsichtigkeit kein Kennzeichen. Dieselbe
/// Ueberlegung wie bei der Markierung aus C2, die seit S16c neben der Farbe
/// eine fette Schrift traegt.
pub const ZUSATZ_UNGUELTIG: Text = Text::LeisteLesezeichenFehlt;

/// Der Takt einer Frage an die Platte: hoechstens eine laeuft, und hoechstens
/// eine Nachfrage folgt ihrer Antwort.
///
/// **Die Meldungen kommen in Stoessen.** Das System meldet ein Ein- oder
/// Aushaengen mehrfach, und jede Meldung einen Faden starten hiesse, die
/// Fadenzahl mit ihnen wachsen zu lassen. Der Takt faltet deshalb alles, was
/// waehrend einer laufenden Frage kommt, zu einer einzigen Nachfrage zusammen.
/// **Eine Nummer braucht er nicht**: es laeuft immer nur eine, also kommen die
/// Antworten in der Reihenfolge an, in der sie erhoben sind. Eine Antwort, die
/// vor der letzten Meldung erhoben ist, gilt deshalb bis zur Antwort der
/// Nachfrage; sie ist die neueste, die es gibt.
///
/// **Was das kostet: eine Frage, die nie zurueckkommt, friert ihren Stand ein.**
/// Abbrechen laesst sie sich nicht (`crate::appkit::nebenfaden`), und eine zweite
/// daneben fragte dieselbe haengende Freigabe; das ist die Fadenzahl, die diese
/// Regel begrenzt.
///
/// Ohne AppKit und ohne Systemaufruf, damit eine Probe die Regel ohne Fenster
/// fahren kann. Zwei Abnehmer: die Aufzaehlung der Datentraeger
/// (`crate::appkit::anwendung`) und die Marken der Lesezeichen, dort je
/// [`plattengruppe`] einer.
#[derive(Debug, Default)]
pub struct Fragetakt {
    /// Steht eine Frage auf dem Faden?
    laeuft: Cell<bool>,
    /// Kam seit ihrem Beginn eine weitere Meldung?
    nachfrage: Cell<bool>,
}

impl Fragetakt {
    /// Eine Meldung ist gekommen. `true` heisst: der Rufer stellt jetzt die Frage.
    #[must_use = "bei true steht die Frage schon als laufend im Merker; ungestellt kaeme nie eine Antwort"]
    pub fn anstossen(&self) -> bool {
        if self.laeuft.replace(true) {
            self.nachfrage.set(true);
            return false;
        }
        true
    }

    /// Die Antwort ist da. `true` heisst: der Rufer fragt gleich noch einmal.
    #[must_use = "bei true steht die Frage schon als laufend im Merker; ungestellt kaeme nie eine Antwort"]
    pub fn antwort_da(&self) -> bool {
        self.laeuft.set(false);
        self.nachfrage.replace(false) && self.anstossen()
    }
}

/// Die Gruppe, in der die Marke eines Pfads erfragt wird: der Datentraeger unter
/// `/Volumes` oder der Rechner unter `/net`, sonst die Wurzel.
///
/// **Je Gruppe ein Faden und nicht je Lesezeichen.** Eine haengende Freigabe
/// haelt damit allein die Marken ihrer eigenen Gruppe beim letzten Stand; die
/// Fadenzahl ist hoechstens die Zahl der Gruppen, die die Lesezeichen nennen.
///
/// **Rein am Pfadtext und ohne Systemaufruf**, sonst stellte die Einteilung
/// selbst die Frage, vor der sie schuetzen soll. **Der Preis:** eine Freigabe,
/// die der Text nicht als solche zu erkennen gibt, liegt in der Gruppe der
/// Wurzel, und das ist ein Lesezeichen ueber einen symbolischen Verweis oder auf
/// einen Einhaengepunkt anderswo. Haengt sie, bleiben die Marken aller
/// gewoehnlichen Lesezeichen beim letzten Stand, bis sie antwortet. Die Auswahl
/// eines jeden Lesezeichens wirkt trotzdem, denn sie fragt auf eigenem Faden.
fn plattengruppe(pfad: &Path) -> PathBuf {
    let mut glieder = pfad.components();
    match (glieder.next(), glieder.next(), glieder.next()) {
        (
            Some(Component::RootDir),
            Some(Component::Normal(wurzel)),
            Some(name @ Component::Normal(_)),
        ) if wurzel == "Volumes" || wurzel == "net" => Path::new("/").join(wurzel).join(name),
        _ => PathBuf::from("/"),
    }
}

/// Der Pfad, an dem ein Ziel liegt; `None` fuer eine Verbindung, die keinen hat.
fn pfad_des_ziels(ziel: &Ziel) -> Option<&Path> {
    match ziel {
        Ziel::Ordner { ordner, .. } => Some(ordner),
        Ziel::Textstelle { datei, .. } => Some(datei),
        Ziel::Verbindung { .. } => None,
    }
}

/// Eine Frage nach den Marken einer [`plattengruppe`]; allein Werte, damit sie
/// den Faden wechseln kann.
#[derive(Debug)]
pub struct Gueltigkeitsfrage {
    gruppe: PathBuf,
    lesezeichen: Vec<Lesezeichen>,
}

impl Gueltigkeitsfrage {
    /// Fragt die Platte. **Laeuft auf dem Faden**, nie auf dem Hauptfaden.
    #[must_use]
    pub fn beantworten(self) -> Gueltigkeitsantwort {
        let befunde = self
            .lesezeichen
            .into_iter()
            .map(|lesezeichen| {
                let gueltig = lesezeichen.gueltig();
                (lesezeichen.ziel, gueltig)
            })
            .collect();
        Gueltigkeitsantwort {
            gruppe: self.gruppe,
            befunde,
        }
    }

    /// Die Antwort ohne Befund, wenn kein Faden entsteht oder er in Panik geraet:
    /// die Marken bleiben beim letzten Stand, und der Takt laeuft weiter.
    #[must_use]
    pub fn ohne_befund(&self) -> Gueltigkeitsantwort {
        Gueltigkeitsantwort {
            gruppe: self.gruppe.clone(),
            befunde: Vec::new(),
        }
    }
}

/// Die Antwort auf eine [`Gueltigkeitsfrage`].
#[derive(Debug)]
pub struct Gueltigkeitsantwort {
    gruppe: PathBuf,
    befunde: Vec<(Ziel, bool)>,
}

/// Was [`Leistenmodell::gueltigkeit_uebernehmen`] dem Rufer hinterlaesst.
#[derive(Debug)]
#[must_use = "die Nachfrage steht schon als laufend im Takt; ungestellt kaeme nie eine Antwort"]
pub struct Gueltigkeitsuebernahme {
    /// Ob sich eine Marke geaendert hat; nur dann muss die Ansicht neu zeichnen.
    pub geaendert: bool,
    /// Die Nachfrage, falls seit dem Beginn der Frage eine Meldung gekommen ist.
    pub nachfrage: Option<Gueltigkeitsfrage>,
}

/// Ein Ort im unteren Teil der Leiste: ein Geraet oder ein Standardort.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ort {
    /// Der Name, den der Nutzer im Finder sieht.
    pub name: String,
    /// Der Ordner, den die Auswahl oeffnet.
    pub pfad: PathBuf,
}

impl Ort {
    /// Ein Ort aus Name und Pfad.
    #[must_use]
    pub fn neu(name: impl Into<String>, pfad: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            pfad: pfad.into(),
        }
    }
}

/// Eine Zeile der Leiste.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zeile {
    /// Eine der Ueberschriften. Nicht waehlbar.
    Ueberschrift(Teil),
    /// Ein Lesezeichen an dieser Stelle der [`Lesezeichenliste`].
    Lesezeichen(usize),
    /// Ein Geraet oder Standardort an dieser Stelle der Ortsliste.
    Ort(usize),
}

impl Zeile {
    /// Ob der Nutzer diese Zeile auswaehlen kann.
    #[must_use]
    pub fn waehlbar(self) -> bool {
        !matches!(self, Zeile::Ueberschrift(_))
    }
}

/// Welcher der Teile der Leiste.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Teil {
    /// Der obere Teil mit den Lesezeichen des Nutzers.
    Lesezeichen,
    /// Der mittlere Teil mit den Verbindungen (261016); er steht allein, wenn
    /// mindestens eine abgelegt ist.
    Verbindungen,
    /// Der untere Teil mit den Geraeten und Standardorten.
    Geraete,
}

impl Teil {
    /// Die Beschriftung der Ueberschriftszeile.
    #[must_use]
    pub fn ueberschrift(self) -> &'static str {
        match self {
            Teil::Lesezeichen => text(UEBERSCHRIFT_LESEZEICHEN),
            Teil::Verbindungen => text(UEBERSCHRIFT_VERBINDUNGEN),
            Teil::Geraete => text(UEBERSCHRIFT_GERAETE),
        }
    }
}

/// Woran der Nutzer die Sorte einer Zeile erkennt (C6).
///
/// **Eine Regel ueber alle waehlbaren Zeilen, keine Sonderregel fuer
/// Textmarken:** das Sinnbild sagt, was die Zeile oeffnet. Ein Geraet oeffnet
/// einen Ordner und traegt deshalb dasselbe Sinnbild wie ein
/// Ordner-Lesezeichen; eine Ueberschrift oeffnet nichts und traegt keines.
/// Zwei Regeln nebeneinander — eine fuer die Lesezeichen, eine fuer die
/// Geraete — haetten zwei Ausnahmen, sobald der untere Teil einmal etwas
/// anderes als Ordner fuehrt.
///
/// **Ein Sinnbild und keine Farbe.** Eine Farbe allein waere bei
/// Farbfehlsichtigkeit kein Kennzeichen; dieselbe Ueberlegung, die C2 und C5
/// zu zwei Kennzeichen gefuehrt hat und die auch hinter [`ZUSATZ_UNGUELTIG`]
/// steht.
///
/// Welches Systemsinnbild daraus wird, entscheidet `crate::appkit::leiste`:
/// hier steht keine Zeile AppKit und deshalb auch kein Bildname.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sinnbild {
    /// Die Zeile oeffnet einen Ordner: ein Ordner-Lesezeichen (C5), ein Geraet
    /// oder ein Standardort.
    Ordner,
    /// Die Zeile oeffnet eine Stelle in einer Datei (C6).
    Textstelle,
    /// Die Zeile stellt auf einen Klick eine Verbindung her (261016).
    Verbindung,
}

/// Was hinter einer ausgewaehlten Zeile steht.
///
/// **Die Sorte steht im [`Ziel`] und nicht daneben** (C6). Ein zweiter
/// Sortenwert hier waere die zweite Wahrheit darueber, was eine Zeile oeffnet,
/// und liefe von dem weg, was in `bookmarks.toml` steht; [`Leistenmodell::sinnbild`]
/// liest dieselbe Eigenschaft fuer die Anzeige. Ein Geraet und ein Standardort
/// tragen [`Ziel::Ordner`], denn genau das oeffnen sie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Auswahl {
    /// Der Name, wie er in der Leiste steht.
    pub name: String,
    /// Was sie oeffnet: einen Ordner (C5) oder eine Stelle in einer Datei (C6).
    pub ziel: Ziel,
    /// Ob das Ziel noch da ist (C5, C6), nach dem letzten bekannten Stand.
    ///
    /// **Frisch ist der Wert erst nach [`Self::nachgefragt`]**;
    /// [`Self::fragt_die_platte`] sagt, ob das noetig ist.
    ///
    /// **Ungueltig heisst allein, dass Ordner oder Datei fehlen.** Ob der
    /// gemerkte Zeileninhalt einer Textmarke noch auf seiner Nummer steht,
    /// entscheidet sich beim Sprung und nur dort; die Begruendung steht im
    /// Modulkopf von [`krk_core::ablage::lesezeichen`].
    ///
    /// Immer `true` fuer ein Geraet: die Aufzaehlung nennt nur, was gerade
    /// eingehaengt ist.
    pub gueltig: bool,
    /// Ob vor der Ausfuehrung die Platte zu fragen ist: ja fuer ein Lesezeichen
    /// mit Pfad, nein fuer eine Verbindung und ein Geraet.
    pub fragt_die_platte: bool,
}

impl Auswahl {
    /// Der Pfad, an dem das Ziel liegt: der Ordner oder die Datei; `None`
    /// fuer eine Verbindung, die keinen Pfad hat (261016).
    ///
    /// **Die eine Stelle, die die Sorten mit Pfad auf einen Pfad bringt.** Der
    /// Aufrufer braucht ihn fuer den Satz ueber ein fehlendes Ziel, und der
    /// lautet fuer beide Sorten gleich: was fehlt, ist ein Eintrag im
    /// Dateisystem, und welcher Art er waere, sagt der Satz nicht. Eine
    /// Verbindung ist nie ungueltig (`Lesezeichen::gueltig`), braucht diesen
    /// Satz also nie. Wer die Sorte braucht, sieht auf [`Self::ziel`] und
    /// bekommt dort eine vollstaendige Fallunterscheidung.
    #[must_use]
    pub fn pfad(&self) -> Option<&Path> {
        pfad_des_ziels(&self.ziel)
    }

    /// Die Auswahl mit dem Stand von jetzt. **Laeuft auf dem Faden**, nie auf dem
    /// Hauptfaden.
    ///
    /// **Hier haelt die Zusage aus C5**, dass eine Auswahl den Grund meldet, statt
    /// in einen Ordner zu fuehren, den es nicht mehr gibt: die Marke in der Zeile
    /// kann veraltet sein, nach beiden Seiten, und diese Antwort ist es nicht.
    #[must_use]
    pub fn nachgefragt(mut self) -> Self {
        if self.fragt_die_platte {
            let lesezeichen = Lesezeichen {
                name: self.name.clone(),
                ziel: self.ziel.clone(),
            };
            self.gueltig = lesezeichen.gueltig();
        }
        self
    }
}

/// Ein Lesezeichen mit dem letzten bekannten Zustand seines Ziels.
///
/// **Eine Stelle faellt die Antwort auf die Frage, was gueltig heisst**,
/// [`Lesezeichen::gueltig`]; bis zum 260807 fragten zwei Wege je fuer sich danach
/// (`issues/260807-0012_*_vier-anlaesse-pruefen-die-lesezeichengueltigkeit-auf-drei-verschiedenen-wegen.md`).
/// Das Feld hier schreiben allein [`Gemerkt::neu`] und [`Gemerkt::befund_setzen`],
/// und keine von beiden fragt die Platte.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Gemerkt {
    lesezeichen: Lesezeichen,
    gueltig: bool,
}

impl Gemerkt {
    /// Ein Eintrag mit dem Stand, den sein Ziel in der vorigen Liste hatte, und
    /// ohne einen solchen als gueltig: beim Start und direkt nach dem Hinzufuegen.
    fn neu(lesezeichen: Lesezeichen, vorige: &[Gemerkt]) -> Self {
        let gueltig = vorige
            .iter()
            .find(|gemerkt| gemerkt.lesezeichen.ziel == lesezeichen.ziel)
            .is_none_or(|gemerkt| gemerkt.gueltig);
        Self {
            lesezeichen,
            gueltig,
        }
    }

    /// Uebernimmt einen Befund und liefert, ob die Marke sich dabei geaendert hat.
    fn befund_setzen(&mut self, gueltig: bool) -> bool {
        std::mem::replace(&mut self.gueltig, gueltig) != gueltig
    }
}

/// Der Inhalt der Leiste und ihre Auswahl.
#[derive(Debug, Default)]
pub struct Leistenmodell {
    lesezeichen: Vec<Gemerkt>,
    orte: Vec<Ort>,
    /// Die Einhaengepunkte, an denen die Leiste „Auswerfen“ anbietet
    /// (261016), aus derselben Aufzaehlung wie [`Self::orte`].
    ///
    /// **Gesetzt im selben Zug wie die Orte** ([`Self::orte_setzen`]), damit
    /// zwischen der Zeile und ihrer Auswerfbarkeit kein Ein- oder Aushaengen
    /// liegen kann. Ausgeworfen wird trotzdem gegen die Liste des
    /// Anwendungsdelegierten, die aus derselben Aufzaehlung stammt; die hier
    /// entscheidet allein, ob die Zeile Sinnbild und Menueeintrag traegt.
    einhaengepunkte: Vec<PathBuf>,
    zeilen: Vec<Zeile>,
    /// Die ausgewaehlte Zeile, oder `None`, solange keine gewaehlt ist.
    ///
    /// Sie zeigt immer auf eine waehlbare Zeile: jede Aenderung an den beiden
    /// Listen zieht sie ueber [`Leistenmodell::auswahl_nachziehen`] nach.
    auswahl: Option<usize>,
    /// Der Takt der Marken, je [`plattengruppe`] einer. Eine Gruppe, die die
    /// Liste nicht mehr nennt, bleibt stehen, damit eine noch laufende Frage ihre
    /// Antwort abgeben kann; die Zahl ist die der Datentraeger, die je in einem
    /// Lesezeichen standen.
    fragen: BTreeMap<PathBuf, Fragetakt>,
    /// Das Ziel der Auswahl, die gemeldet und noch nicht vollzogen ist.
    offene_auswahl: Option<Ziel>,
    /// Ob die Auswahl des laufenden Mausklicks schon gemeldet ist.
    ///
    /// AppKit schickt die `action` der Tabelle nach der Auswahlmeldung, also kaeme
    /// jeder Klick auf eine neue Zeile mit offener Auswahl bei
    /// [`Self::klick_loest_aus`] an und fragte die Platte ein zweites Mal.
    klick_gemeldet: bool,
}

impl Leistenmodell {
    /// Ein leeres Modell mit den beiden Ueberschriften.
    #[must_use]
    pub fn neu() -> Self {
        let mut modell = Self::default();
        modell.zeilen_bauen();
        modell
    }

    /// Uebernimmt die Lesezeichen (C5); **ohne Frage an die Platte**.
    ///
    /// Jeder Eintrag behaelt den Stand seines Ziels ([`Gemerkt::neu`]). Dass die
    /// Marken danach erfragt werden, ist Sache der Ansicht, die ihn hierher reicht
    /// (`crate::appkit::leiste::Leistenquelle`), ueber
    /// [`Self::gueltigkeit_anstossen`].
    pub fn lesezeichen_setzen(&mut self, liste: &Lesezeichenliste) {
        let neu = liste
            .eintraege
            .iter()
            .cloned()
            .map(|lesezeichen| Gemerkt::neu(lesezeichen, &self.lesezeichen))
            .collect();
        self.lesezeichen = neu;
        self.zeilen_bauen();
    }

    /// Uebernimmt die Geraete und Standardorte (C5); **ohne Frage an die Platte**.
    ///
    /// Die Ortsliste aendert sich genau dann, wenn ein Datentraeger gekommen oder
    /// gegangen ist, und damit aendert sich, was ein Lesezeichen darauf wert ist;
    /// die Ansicht stoesst deshalb danach die Marken an.
    ///
    /// **`einhaengepunkte` sind die blossen Einhaengepunkte derselben
    /// Aufzaehlung** (261016); an ihnen entscheidet
    /// [`Self::auswurfziel`], welche Zeile auswerfbar ist.
    pub fn orte_setzen(&mut self, orte: Vec<Ort>, einhaengepunkte: Vec<PathBuf>) {
        self.orte = orte;
        self.einhaengepunkte = einhaengepunkte;
        self.zeilen_bauen();
    }

    /// Eine Meldung ist gekommen: die Fragen, die der Rufer jetzt stellt.
    ///
    /// Je [`plattengruppe`] hoechstens eine, gebaut aus der Liste von jetzt. Laeuft
    /// fuer eine Gruppe schon eine, wird aus der Meldung deren Nachfrage
    /// ([`Fragetakt`]). Eine Verbindung fragt nie, also fragt eine Liste ohne
    /// Pfade so wenig wie eine leere, und das ist der Messmodus.
    ///
    /// Vier Anlaesse in der Ansicht: die Lesezeichen aendern sich, die Ortsliste
    /// aendert sich, eine Dateioperation aus C4 ist abgeschlossen
    /// (`crate::appkit::anwendung::Anwendungsdelegierter::vorgang_beenden`, dort
    /// begruendet), und eine Auswahl hat einen anderen Stand gefunden. Loescht ein
    /// **fremdes** Programm den Ordner, steht die Marke deshalb weiterhin bis zum
    /// naechsten Anlass falsch; die Zusage aus C5 haelt auch dann, weil die Auswahl
    /// selbst fragt.
    #[must_use = "jede gelieferte Frage steht schon als laufend im Takt; ungestellt kaeme nie eine Antwort"]
    pub fn gueltigkeit_anstossen(&mut self) -> Vec<Gueltigkeitsfrage> {
        let mut gruppen: Vec<PathBuf> = self
            .lesezeichen
            .iter()
            .filter_map(|gemerkt| pfad_des_ziels(&gemerkt.lesezeichen.ziel))
            .map(plattengruppe)
            .collect();
        gruppen.sort();
        gruppen.dedup();
        let mut fragen = Vec::new();
        for gruppe in gruppen {
            if self.fragen.entry(gruppe.clone()).or_default().anstossen() {
                fragen.push(self.frage_der_gruppe(gruppe));
            }
        }
        fragen
    }

    /// Uebernimmt die Antwort einer Frage und liefert die Nachfrage, falls eine
    /// faellig ist.
    ///
    /// **Die Marken gehen nach dem Ziel**: siehe den Modulkopf. Die Nachfrage liest
    /// die Liste von jetzt, also erreicht sie auch einen Eintrag, der nach dem
    /// Beginn der Frage hinzugekommen ist.
    pub fn gueltigkeit_uebernehmen(
        &mut self,
        antwort: Gueltigkeitsantwort,
    ) -> Gueltigkeitsuebernahme {
        let Gueltigkeitsantwort { gruppe, befunde } = antwort;
        let geaendert = self.befunde_anwenden(&befunde);
        let takt = self.fragen.entry(gruppe.clone()).or_default();
        let nachfrage = takt
            .antwort_da()
            .then(|| self.frage_der_gruppe(gruppe.clone()))
            .and_then(|frage| {
                if !frage.lesezeichen.is_empty() {
                    return Some(frage);
                }
                // Die Gruppe ist inzwischen leer: nichts zu fragen, und der Takt
                // gibt seine Frage zurueck, die nie gestellt wird. Eine Nachfrage
                // kann er dabei nicht melden, sie laeuft ja nicht.
                if let Some(takt) = self.fragen.get(&gruppe) {
                    let _ = takt.antwort_da();
                }
                None
            });
        Gueltigkeitsuebernahme {
            geaendert,
            nachfrage,
        }
    }

    /// Uebernimmt, was eine Auswahl ueber ihr eines Ziel erfahren hat, und liefert,
    /// ob sich dabei eine Marke geaendert hat.
    ///
    /// Sie laeuft neben dem Takt, also kann eine aeltere Antwort der Gruppe sie noch
    /// ueberschreiben. **Der Rufer stoesst bei `true` deshalb die Marken an**: die
    /// Nachfrage beginnt nach beiden und hat das letzte Wort.
    #[must_use = "bei true stoesst der Rufer die Marken an und zeichnet neu"]
    pub fn auswahlbefund_uebernehmen(&mut self, auswahl: &Auswahl) -> bool {
        self.befunde_anwenden(std::slice::from_ref(&(
            auswahl.ziel.clone(),
            auswahl.gueltig,
        )))
    }

    /// Setzt die Marken nach den Befunden; liefert, ob sich eine geaendert hat.
    fn befunde_anwenden(&mut self, befunde: &[(Ziel, bool)]) -> bool {
        let mut geaendert = false;
        for gemerkt in &mut self.lesezeichen {
            if let Some((_, gueltig)) = befunde
                .iter()
                .find(|(ziel, _)| *ziel == gemerkt.lesezeichen.ziel)
            {
                // `|=` und nicht `||`: jeder Eintrag bekommt seinen Befund, auch
                // wenn ein frueherer sich schon geaendert hat.
                geaendert |= gemerkt.befund_setzen(*gueltig);
            }
        }
        geaendert
    }

    /// Die Frage nach den Eintraegen einer Gruppe, aus der Liste von jetzt.
    fn frage_der_gruppe(&self, gruppe: PathBuf) -> Gueltigkeitsfrage {
        let lesezeichen = self
            .lesezeichen
            .iter()
            .map(|gemerkt| &gemerkt.lesezeichen)
            .filter(|lesezeichen| {
                pfad_des_ziels(&lesezeichen.ziel).is_some_and(|pfad| plattengruppe(pfad) == gruppe)
            })
            .cloned()
            .collect();
        Gueltigkeitsfrage {
            gruppe,
            lesezeichen,
        }
    }

    /// Die Lesezeichen, wie sie auf die Platte gehoeren.
    #[must_use]
    pub fn lesezeichenliste(&self) -> Lesezeichenliste {
        Lesezeichenliste::aus(
            self.lesezeichen
                .iter()
                .map(|gemerkt| gemerkt.lesezeichen.clone())
                .collect(),
        )
    }

    /// Alle Zeilen, von oben nach unten.
    #[must_use]
    pub fn zeilen(&self) -> &[Zeile] {
        &self.zeilen
    }

    /// Die Zeile an dieser Stelle.
    #[must_use]
    pub fn zeile(&self, stelle: usize) -> Option<Zeile> {
        self.zeilen.get(stelle).copied()
    }

    /// Die Beschriftung einer Zeile, so wie sie in der Leiste steht.
    #[must_use]
    pub fn beschriftung(&self, stelle: usize) -> Option<String> {
        match self.zeile(stelle)? {
            Zeile::Ueberschrift(teil) => Some(teil.ueberschrift().to_owned()),
            Zeile::Lesezeichen(stelle) => {
                let gemerkt = self.lesezeichen.get(stelle)?;
                Some(match gemerkt.gueltig {
                    true => gemerkt.lesezeichen.name.clone(),
                    false => satz(ZUSATZ_UNGUELTIG, &[("name", &gemerkt.lesezeichen.name)]),
                })
            }
            Zeile::Ort(stelle) => Some(self.orte.get(stelle)?.name.clone()),
        }
    }

    /// Ob die Zeile ein ungueltiges Lesezeichen ist (C5).
    ///
    /// Die Leiste faerbt sie danach; der Zusatz im Text kommt aus
    /// [`Leistenmodell::beschriftung`].
    #[must_use]
    pub fn ungueltig(&self, stelle: usize) -> bool {
        match self.zeile(stelle) {
            Some(Zeile::Lesezeichen(stelle)) => {
                self.lesezeichen.get(stelle).is_some_and(|g| !g.gueltig)
            }
            _ => false,
        }
    }

    /// Das Sinnbild, das vor der Beschriftung steht (C6).
    ///
    /// `None` fuer eine Ueberschrift: sie oeffnet nichts. Die
    /// Fallunterscheidung ueber [`Ziel`] ist vollstaendig und hat keinen
    /// Auffangzweig; eine dritte Sorte hielte den Bau an und erzwaenge die
    /// Antwort, woran der Nutzer sie erkennt.
    ///
    /// **Sie stellt keine Frage an das Dateisystem.** Die Sorte steht im
    /// Eintrag; ob sein Ziel noch da ist, beantwortet
    /// [`Leistenmodell::ungueltig`] aus der Marke, die die letzte Antwort
    /// gesetzt hat.
    #[must_use]
    pub fn sinnbild(&self, stelle: usize) -> Option<Sinnbild> {
        match self.zeile(stelle)? {
            Zeile::Ueberschrift(_) => None,
            Zeile::Lesezeichen(stelle) => {
                Some(match self.lesezeichen.get(stelle)?.lesezeichen.ziel {
                    Ziel::Ordner { .. } => Sinnbild::Ordner,
                    Ziel::Textstelle { .. } => Sinnbild::Textstelle,
                    Ziel::Verbindung { .. } => Sinnbild::Verbindung,
                })
            }
            Zeile::Ort(stelle) => {
                self.orte.get(stelle)?;
                Some(Sinnbild::Ordner)
            }
        }
    }

    /// Die ausgewaehlte Zeile.
    #[must_use]
    pub fn auswahl(&self) -> Option<usize> {
        self.auswahl
    }

    /// Waehlt die Zeile an dieser Stelle, falls sie waehlbar ist.
    ///
    /// Liefert, ob sich die Auswahl dadurch geaendert hat. Eine Ueberschrift
    /// laesst die Auswahl stehen: sie ist keine, und der Mausklick auf sie soll
    /// die vorige nicht wegnehmen.
    ///
    /// **Dies ist der Weg der Maus**, und die `action` desselben Klicks folgt ihm;
    /// die Tastatur geht ueber [`Self::auswahl_bewegen`], und ihr folgt keine.
    #[must_use]
    pub fn waehlen(&mut self, stelle: usize) -> bool {
        if !self.zeile(stelle).is_some_and(Zeile::waehlbar) || self.auswahl == Some(stelle) {
            return false;
        }
        self.auswahl = Some(stelle);
        self.klick_gemeldet = true;
        true
    }

    /// Was an den Halter zu melden ist; die Auswahl ist von hier an offen, bis
    /// [`Self::auswahl_vollzogen`] sie schliesst.
    #[must_use = "die Auswahl steht schon als offen im Modell; ungemeldet wird sie nie vollzogen"]
    pub fn auswahl_zum_melden(&mut self) -> Option<Auswahl> {
        let auswahl = self.gewaehlt()?;
        self.offene_auswahl = Some(auswahl.ziel.clone());
        Some(auswahl)
    }

    /// Die zuletzt gemeldete Auswahl hat gewirkt.
    ///
    /// **Sie meldet nicht, wofuer**: der Halter kommt allein mit der juengsten
    /// Auswahl hierher, eine ueberholte Antwort also nie. Auch der Satz ueber ein
    /// fehlendes Ziel zaehlt als Wirkung, denn der Nutzer hat seine Antwort.
    pub fn auswahl_vollzogen(&mut self) {
        self.offene_auswahl = None;
    }

    /// Ein Mausklick auf die Zeile `geklickt`, `None` neben die Zeilen: `true`
    /// heisst, der Rufer meldet die Auswahl noch einmal.
    ///
    /// Allein fuer die gewaehlte Zeile, solange ihr Ziel offen ist, und nicht fuer
    /// den Klick, dessen Auswahl eben erst gemeldet ist.
    ///
    /// **Bei noch ausstehender Antwort startet jeder weitere Klick einen weiteren
    /// Faden**, auch der zweite eines Doppelklicks; die Auswahl hat dafuer keine
    /// Obergrenze (`crate::appkit::anwendung`, `Leistenauswahlstand`).
    #[must_use = "bei true schuldet der Rufer die Meldung, und der Merker fuer den laufenden Klick ist schon verbraucht"]
    pub fn klick_loest_aus(&mut self, geklickt: Option<usize>) -> bool {
        if std::mem::take(&mut self.klick_gemeldet) {
            return false;
        }
        let Some(offen) = &self.offene_auswahl else {
            return false;
        };
        geklickt.is_some()
            && geklickt == self.auswahl
            && self
                .gewaehlt()
                .is_some_and(|gewaehlt| gewaehlt.ziel == *offen)
    }

    /// Bewegt die Auswahl um eine waehlbare Zeile nach oben oder unten (C5).
    ///
    /// Ueberschriften werden uebersprungen, am Rand haelt die Auswahl an.
    /// Liefert, ob sie sich bewegt hat. Ohne Auswahl faengt sie bei der ersten
    /// waehlbaren Zeile an, gleich in welche Richtung: die Leiste hat gerade
    /// den Fokus bekommen, und der Nutzer erwartet einen sichtbaren Anfang.
    #[must_use]
    pub fn auswahl_bewegen(&mut self, schritt: isize) -> bool {
        // Einer Bewegung von der Tastatur folgt keine `action`. Steht der Merker
        // noch, ist er von einem Mausklick liegen geblieben, und er nahme dem
        // naechsten Klick seine Wirkung.
        self.klick_gemeldet = false;
        let Some(jetzt) = self.auswahl else {
            let erste = self.naechste_waehlbare(0, 1);
            self.auswahl = erste;
            return erste.is_some();
        };
        let Some(ziel) = jetzt
            .checked_add_signed(schritt)
            .and_then(|von| self.naechste_waehlbare(von, schritt))
        else {
            return false;
        };
        if ziel == jetzt {
            return false;
        }
        self.auswahl = Some(ziel);
        true
    }

    /// Was hinter der ausgewaehlten Zeile steht.
    ///
    /// **Beide Sorten liefern eine Auswahl** (C6). Die Fallunterscheidung
    /// steht nicht hier, sondern beim Aufrufer: dieses Modell sagt, **was** die
    /// Zeile bezeichnet, und `crate::appkit::anwendung` entscheidet, was daraus
    /// folgt — ein Ordner im aktiven Dateifenster oder eine Datei im Editor.
    ///
    /// Bis S39 lieferte eine Textmarke hier `None`, weil [`Auswahl`] einen
    /// Ordner trug und eine Textmarke keinen hat; ihre Auswahl blieb damit
    /// folgenlos. Der Platzhalter ist gefallen, seit `Auswahl` das [`Ziel`]
    /// selbst traegt.
    ///
    /// `None` bleibt fuer eine Ueberschrift: sie oeffnet nichts, wie das
    /// fehlende Sinnbild in [`Self::sinnbild`] es schon sagt.
    #[must_use]
    pub fn gewaehlt(&self) -> Option<Auswahl> {
        match self.zeile(self.auswahl?)? {
            Zeile::Ueberschrift(_) => None,
            Zeile::Lesezeichen(stelle) => {
                let gemerkt = self.lesezeichen.get(stelle)?;
                Some(Auswahl {
                    name: gemerkt.lesezeichen.name.clone(),
                    ziel: gemerkt.lesezeichen.ziel.clone(),
                    gueltig: gemerkt.gueltig,
                    fragt_die_platte: pfad_des_ziels(&gemerkt.lesezeichen.ziel).is_some(),
                })
            }
            Zeile::Ort(stelle) => {
                let ort = self.orte.get(stelle)?;
                Some(Auswahl {
                    name: ort.name.clone(),
                    ziel: Ziel::Ordner {
                        ordner: ort.pfad.clone(),
                        auswahl: None,
                    },
                    gueltig: true,
                    fragt_die_platte: false,
                })
            }
        }
    }

    /// Die Stelle des ausgewaehlten Lesezeichens in der Lesezeichenliste.
    ///
    /// `None`, wenn die Auswahl auf einer Ueberschrift oder einem Geraet steht.
    /// Die vier Befehle, die ein Lesezeichen aendern, wirken dann nicht; sie
    /// melden das nicht, wie der Wirkungsbereich es auch nicht tut.
    #[must_use]
    pub fn gewaehltes_lesezeichen(&self) -> Option<usize> {
        match self.zeile(self.auswahl?)? {
            Zeile::Lesezeichen(stelle) => Some(stelle),
            _ => None,
        }
    }

    /// Der Name des Eintrags in `secrets.txt`, wenn die Zeile an dieser
    /// Stelle eine Verbindung ist (261016).
    ///
    /// **Die eine Frage, die der Klick und der Befehl „Verbindung
    /// herstellen“ stellen**: der Klick mit der angeklickten Zeile, der Befehl
    /// mit der gewaehlten. Sie stellt keine Frage an das Dateisystem.
    #[must_use]
    pub fn verbindung_der_zeile(&self, stelle: usize) -> Option<String> {
        let Zeile::Lesezeichen(stelle) = self.zeile(stelle)? else {
            return None;
        };
        match &self.lesezeichen.get(stelle)?.lesezeichen.ziel {
            Ziel::Verbindung { verbindung } => Some(verbindung.clone()),
            Ziel::Ordner { .. } | Ziel::Textstelle { .. } => None,
        }
    }

    /// Der Einhaengepunkt, den die Zeile an dieser Stelle auswerfen laesst,
    /// oder `None` (261016).
    ///
    /// **Dieselbe Regel wie im Kontextmenue der Dateiliste**, und nicht eine
    /// zweite: gefragt wird [`crate::kommandos::kontextmenue::auswurfbezug`]
    /// mit dem Pfad der Zeile als einzigem Betroffenen. Auswerfbar ist damit
    /// genau ein Ort, dessen Pfad ein Einhaengepunkt ist und nicht die Wurzel
    /// `/`; das Benutzerverzeichnis, die Lesezeichen, die Verbindungen und die
    /// Ueberschriften sind es nie, und der Startdatentraeger auch nicht.
    ///
    /// **Sie stellt keine Frage an das Dateisystem**, wie `auswurfbezug`
    /// selbst; sie wird je gezeichneter Zeile gefragt.
    #[must_use]
    pub fn auswurfziel(&self, stelle: usize) -> Option<PathBuf> {
        let Zeile::Ort(stelle) = self.zeile(stelle)? else {
            return None;
        };
        let ort = self.orte.get(stelle)?;
        crate::kommandos::kontextmenue::auswurfbezug(
            std::slice::from_ref(&ort.pfad),
            &self.einhaengepunkte,
        )
        .into_iter()
        .next()
    }

    /// Der Name des Eintrags in `secrets.txt`, wenn die gewaehlte Zeile eine
    /// Verbindung ist (261016).
    #[must_use]
    pub fn gewaehlte_verbindung(&self) -> Option<String> {
        self.verbindung_der_zeile(self.auswahl?)
    }

    /// Das ausgewaehlte Lesezeichen als Wert, falls eines ausgewaehlt ist (C5).
    ///
    /// **Der Eintrag und nicht seine Stelle**, und darauf beruht seit der
    /// Runde 7 die Zusage aus C3.8. Die drei Befehle, die ein vorhandenes
    /// Lesezeichen aendern, nennen ihr Ziel damit als Wert; eine Stelle waere
    /// eine Zahl in **dieser** Liste, und geschrieben wird auf die frisch von
    /// der Platte gelesene, in der an derselben Stelle ein anderes stehen kann.
    /// Der Modulkopf von [`krk_core::ablage::lesezeichen`] schreibt es aus.
    #[must_use]
    pub fn gewaehltes_lesezeichen_wert(&self) -> Option<Lesezeichen> {
        let stelle = self.gewaehltes_lesezeichen()?;
        self.lesezeichen
            .get(stelle)
            .map(|gemerkt| gemerkt.lesezeichen.clone())
    }

    /// Uebernimmt eine gelesene Lesezeichenliste und stellt die Auswahl (C5).
    ///
    /// Der eine Weg, auf dem eine geaenderte Liste in die Leiste kommt. Bis zur
    /// Runde 7 rechneten hier vier Methoden die neue Liste selbst aus und
    /// schrieben sie danach; seither entsteht sie unter der Schreibsperre aus
    /// der Datei, und die Leiste uebernimmt das Ergebnis.
    ///
    /// `stelle` ist der betroffene Eintrag: die Auswahl haengt am Lesezeichen
    /// und nicht an der Zeilennummer, sonst schoebe der zweite Tastendruck den
    /// naechsten Eintrag.
    ///
    /// **Zwei Faelle lassen die Auswahl, wo sie ist**, und in beiden hat
    /// `lesezeichen_setzen` sie ueber `auswahl_nachziehen` schon auf eine
    /// waehlbare Zeile gestellt:
    ///
    /// - `None`, also kein betroffener Eintrag.
    /// - Eine Stelle, die die neue Liste nicht mehr fuehrt. Das ist der Fall
    ///   nach dem Loeschen des letzten Lesezeichens: die Zeile darunter ist die
    ///   Ueberschrift der Geraete, und `auswahl_nachziehen` geht von dort auf
    ///   das erste Geraet. Nach dem Loeschen eines Eintrags in der Mitte gibt es
    ///   die Stelle dagegen weiter, und sie trifft den nachgerueckten Eintrag —
    ///   dieselbe Antwort, die die Auswahl in der Dateiliste gibt.
    pub fn uebernehmen(&mut self, liste: &Lesezeichenliste, stelle: Option<usize>) {
        self.lesezeichen_setzen(liste);
        if let Some(zeile) = stelle.and_then(|stelle| self.zeile_des_lesezeichens(stelle)) {
            self.auswahl = Some(zeile);
        }
    }

    /// Baut die Zeilenliste aus den Teilen neu auf.
    ///
    /// **Die Gruppe fragt die Sorte und nicht das Dateisystem**
    /// (`Ziel::gruppe`). Die Ueberschrift der Verbindungen steht allein mit
    /// mindestens einer Verbindung; die der Lesezeichen und der Geraete stehen
    /// immer, wie vor den Verbindungen.
    fn zeilen_bauen(&mut self) {
        let in_gruppe = |gruppe: Gruppe| {
            self.lesezeichen
                .iter()
                .enumerate()
                .filter(move |(_, gemerkt)| gemerkt.lesezeichen.ziel.gruppe() == gruppe)
                .map(|(stelle, _)| Zeile::Lesezeichen(stelle))
        };
        let mut zeilen = Vec::with_capacity(self.lesezeichen.len() + self.orte.len() + 3);
        zeilen.push(Zeile::Ueberschrift(Teil::Lesezeichen));
        zeilen.extend(in_gruppe(Gruppe::Lesezeichen));
        let mut verbindungen = in_gruppe(Gruppe::Verbindungen).peekable();
        if verbindungen.peek().is_some() {
            zeilen.push(Zeile::Ueberschrift(Teil::Verbindungen));
            zeilen.extend(verbindungen);
        }
        zeilen.push(Zeile::Ueberschrift(Teil::Geraete));
        zeilen.extend((0..self.orte.len()).map(Zeile::Ort));
        self.zeilen = zeilen;
        self.auswahl_nachziehen();
    }

    /// Haelt die Auswahl auf einer waehlbaren Zeile.
    fn auswahl_nachziehen(&mut self) {
        let Some(jetzt) = self.auswahl else {
            return;
        };
        if self.zeile(jetzt).is_some_and(Zeile::waehlbar) {
            return;
        }
        self.auswahl = self
            .naechste_waehlbare(jetzt, 1)
            .or_else(|| self.naechste_waehlbare(jetzt.saturating_sub(1), -1));
    }

    /// Die naechste waehlbare Zeile ab `von`, in Richtung `schritt`.
    fn naechste_waehlbare(&self, von: usize, schritt: isize) -> Option<usize> {
        let mut stelle = von;
        loop {
            match self.zeile(stelle) {
                Some(zeile) if zeile.waehlbar() => return Some(stelle),
                Some(_) => stelle = stelle.checked_add_signed(schritt)?,
                None => return None,
            }
        }
    }

    /// Die Zeile, in der das Lesezeichen an dieser Stelle steht.
    fn zeile_des_lesezeichens(&self, stelle: usize) -> Option<usize> {
        self.zeilen
            .iter()
            .position(|zeile| *zeile == Zeile::Lesezeichen(stelle))
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use krk_core::ablage::{Aenderung, Ausgang, Verschiebung};

    use super::*;
    use crate::pruefordner::Pruefordner;

    /// Stellt alle Fragen, die das Modell gerade hat, und zieht ihre Antworten und
    /// Nachfragen ein, wie die Ansicht es ueber den Faden tut. Liefert, ob sich eine
    /// Marke geaendert hat.
    fn nachpruefen(modell: &mut Leistenmodell) -> bool {
        let mut offen = modell.gueltigkeit_anstossen();
        let mut geaendert = false;
        while let Some(frage) = offen.pop() {
            let uebernahme = modell.gueltigkeit_uebernehmen(frage.beantworten());
            geaendert |= uebernahme.geaendert;
            offen.extend(uebernahme.nachfrage);
        }
        geaendert
    }

    fn modell() -> Leistenmodell {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Eins", "/eins"),
            Lesezeichen::neu("Zwei", "/zwei"),
        ]));
        modell.orte_setzen(
            vec![
                Ort::neu("Benutzer", "/Users/pruefung"),
                Ort::neu("Macintosh HD", "/"),
            ],
            Vec::new(),
        );
        let _ = nachpruefen(&mut modell);
        modell
    }

    fn beschriftungen(modell: &Leistenmodell) -> Vec<String> {
        (0..modell.zeilen().len())
            .filter_map(|stelle| modell.beschriftung(stelle))
            .collect()
    }

    fn sinnbilder(modell: &Leistenmodell) -> Vec<Option<Sinnbild>> {
        (0..modell.zeilen().len())
            .map(|stelle| modell.sinnbild(stelle))
            .collect()
    }

    /// Fuehrt eine Aenderung an den Lesezeichen so aus, wie der Delegierte es
    /// tut: rechnen und das Ergebnis samt betroffener Stelle uebernehmen.
    ///
    /// Seit der Runde 7 rechnet das Modell nicht mehr selbst; die geaenderte
    /// Liste kommt unter der Schreibsperre von der Platte. Die Proben nehmen
    /// denselben Weg, nur ohne Datei, und pruefen damit weiter, was hier zu
    /// pruefen ist: wo die Auswahl danach steht.
    fn aendern(modell: &mut Leistenmodell, aenderung: &Aenderung) -> Ausgang {
        let mut liste = modell.lesezeichenliste();
        let ausgang = liste.anwenden(aenderung);
        let stelle = match ausgang {
            Ausgang::Geaendert(stelle) => Some(stelle),
            Ausgang::Unveraendert | Ausgang::Verschwunden => None,
        };
        modell.uebernehmen(&liste, stelle);
        let _ = nachpruefen(modell);
        ausgang
    }

    /// Aendert das ausgewaehlte Lesezeichen, oder meldet, dass keines gewaehlt
    /// ist.
    ///
    /// Die drei Befehle, die ein vorhandenes Lesezeichen treffen, nennen ihr
    /// Ziel als Eintrag. Steht die Auswahl auf einer Ueberschrift oder einem
    /// Geraet, gibt es keinen, und der Befehl wirkt nicht.
    fn gewaehltes_aendern(
        modell: &mut Leistenmodell,
        bauen: impl FnOnce(Lesezeichen) -> Aenderung,
    ) -> Option<Ausgang> {
        let welches = modell.gewaehltes_lesezeichen_wert()?;
        Some(aendern(modell, &bauen(welches)))
    }

    /// Ein Ordnerziel aus einem Pfad, damit die Proben kurz bleiben.
    ///
    /// Dasselbe Gegenstueck wie in `krk_core::ablage::lesezeichen`.
    fn ordnerziel(pfad: impl Into<PathBuf>) -> Ziel {
        Ziel::Ordner {
            ordner: pfad.into(),
            auswahl: None,
        }
    }

    /// Ein Textstellenziel, das auf eine Datei zeigt, die es nicht gibt.
    fn textziel(datei: impl Into<PathBuf>, zeile: u32, zeileninhalt: &str) -> Ziel {
        Ziel::Textstelle {
            datei: datei.into(),
            zeile,
            zeileninhalt: zeileninhalt.to_owned(),
        }
    }

    /// Die Ueberschrift „Verbindungen“ steht allein mit mindestens einer
    /// Verbindung, und die Teile stehen in der Folge Lesezeichen,
    /// Verbindungen, Geraete; die Verbindungen einer gemischten Liste ziehen in
    /// ihre Gruppe (Akzeptanz (4) von Schritt 6).
    #[test]
    fn die_verbindungen_stehen_als_eigene_gruppe_zwischen_lesezeichen_und_geraeten() {
        let ohne = modell();
        assert!(
            !beschriftungen(&ohne)
                .iter()
                .any(|zeile| zeile == "Verbindungen"),
            "ohne Verbindung keine Ueberschrift"
        );

        let mut modell = Leistenmodell::neu();
        let ordner = std::env::temp_dir();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::verbindung("NAS"),
            Lesezeichen::neu("Temp", &ordner),
            Lesezeichen::verbindung("Server"),
        ]));
        modell.orte_setzen(vec![Ort::neu("Macintosh HD", "/")], Vec::new());
        assert_eq!(
            beschriftungen(&modell),
            [
                "Lesezeichen",
                "Temp",
                "Verbindungen",
                "NAS",
                "Server",
                "Geräte und Orte",
                "Macintosh HD"
            ]
        );
        assert_eq!(modell.sinnbild(3), Some(Sinnbild::Verbindung));
        assert!(!modell.zeilen()[2].waehlbar());
        assert!(!modell.ungueltig(3), "eine Verbindung ist nie ungueltig");
        assert_eq!(modell.verbindung_der_zeile(4).as_deref(), Some("Server"));
        assert_eq!(modell.verbindung_der_zeile(1), None, "ein Ordner ist keine");
        assert_eq!(
            modell.verbindung_der_zeile(2),
            None,
            "eine Ueberschrift auch nicht"
        );
    }

    /// Die Pfeiltaste waehlt eine Verbindung und liefert ihre Auswahl ohne
    /// Pfad; ob verbunden wird, entscheidet der Aufrufer, und der tut es auf
    /// eine Auswahl nicht (Entscheidung 12, Akzeptanz (5) von Schritt 6). Der
    /// Befehl fragt die gewaehlte Verbindung.
    #[test]
    fn eine_gewaehlte_verbindung_hat_keinen_pfad_und_nennt_ihren_eintrag() {
        let mut modell = Leistenmodell::neu();
        let mut umbenannt = Lesezeichen::verbindung("NAS");
        umbenannt.name = "Büro".to_owned();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![umbenannt]));
        assert!(modell.auswahl_bewegen(1));
        let auswahl = modell.gewaehlt().expect("die Verbindung ist gewaehlt");
        assert_eq!(auswahl.pfad(), None);
        assert!(auswahl.gueltig);
        assert_eq!(auswahl.name, "Büro");
        assert_eq!(
            modell.gewaehlte_verbindung().as_deref(),
            Some("NAS"),
            "der Befehl nimmt den Eintrag und nicht die Beschriftung"
        );
    }

    #[test]
    fn die_leiste_trennt_lesezeichen_sichtbar_von_geraeten() {
        let modell = modell();
        assert_eq!(
            beschriftungen(&modell),
            [
                // Die beiden Pruefpfade gibt es nicht, also traegt jeder von
                // ihnen sein Kennzeichen; geprueft wird hier die Reihenfolge
                // und die Trennung, nicht die Gueltigkeit.
                "Lesezeichen",
                "Eins (fehlt)",
                "Zwei (fehlt)",
                "Geräte und Orte",
                "Benutzer",
                "Macintosh HD"
            ]
        );
        assert!(!modell.zeilen()[0].waehlbar());
        assert!(!modell.zeilen()[3].waehlbar());
    }

    #[test]
    fn die_beiden_ueberschriften_stehen_auch_in_einer_leeren_leiste() {
        let modell = Leistenmodell::neu();
        assert_eq!(beschriftungen(&modell), ["Lesezeichen", "Geräte und Orte"]);
        assert_eq!(modell.auswahl(), None);
        assert_eq!(modell.gewaehlt(), None);
    }

    #[test]
    fn der_auf_und_der_ab_pfeil_ueberspringen_die_ueberschriften() {
        let mut modell = modell();
        // Ohne Auswahl faengt jede Bewegung bei der ersten waehlbaren Zeile an.
        assert!(modell.auswahl_bewegen(1));
        assert_eq!(modell.auswahl(), Some(1));
        assert!(modell.auswahl_bewegen(1));
        assert_eq!(modell.auswahl(), Some(2));
        // Zeile 3 ist die zweite Ueberschrift und wird uebersprungen.
        assert!(modell.auswahl_bewegen(1));
        assert_eq!(modell.auswahl(), Some(4));
        assert!(modell.auswahl_bewegen(-1));
        assert_eq!(modell.auswahl(), Some(2));
    }

    #[test]
    fn am_rand_haelt_die_auswahl_an() {
        let mut modell = modell();
        let _ = modell.waehlen(1);
        assert!(
            !modell.auswahl_bewegen(-1),
            "oberhalb steht die Ueberschrift"
        );
        assert_eq!(modell.auswahl(), Some(1));
        let _ = modell.waehlen(5);
        assert!(!modell.auswahl_bewegen(1), "darunter steht nichts mehr");
        assert_eq!(modell.auswahl(), Some(5));
    }

    #[test]
    fn eine_ueberschrift_laesst_sich_nicht_waehlen() {
        let mut modell = modell();
        let _ = modell.waehlen(1);
        assert!(!modell.waehlen(0));
        assert!(!modell.waehlen(3));
        assert_eq!(modell.auswahl(), Some(1));
    }

    #[test]
    fn die_auswahl_nennt_ordner_und_gueltigkeit() {
        let mut modell = modell();
        let _ = modell.waehlen(2);
        assert_eq!(
            modell.gewaehlt(),
            Some(Auswahl {
                name: "Zwei".to_owned(),
                ziel: Ziel::Ordner {
                    ordner: PathBuf::from("/zwei"),
                    auswahl: None
                },
                gueltig: false,
                fragt_die_platte: true,
            }),
            "/zwei gibt es nicht, also ist das Lesezeichen ungueltig"
        );
        let _ = modell.waehlen(5);
        assert_eq!(
            modell.gewaehlt(),
            Some(Auswahl {
                name: "Macintosh HD".to_owned(),
                ziel: Ziel::Ordner {
                    ordner: PathBuf::from("/"),
                    auswahl: None
                },
                gueltig: true,
                fragt_die_platte: false,
            })
        );
    }

    /// Die Ablösung des Platzhalters aus [`Leistenmodell::gewaehlt`] (S39): eine
    /// Textmarke liefert eine Auswahl, und die traegt ihr [`Ziel`] mitsamt
    /// Zeilennummer und gemerktem Inhalt. Bis S39 lieferte sie `None` und blieb
    /// damit folgenlos.
    #[test]
    fn eine_textmarke_liefert_ihre_stelle_und_nicht_nichts() {
        let ordner = Pruefordner::nur_name("markenauswahl");
        ordner.anlegen();
        let datei = ordner.datei("quelle.rs", "eine Zeile\n");

        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::textstelle(
            "Stelle",
            &datei,
            7,
            "eine Zeile",
        )]));
        assert!(modell.waehlen(1));
        assert_eq!(
            modell.gewaehlt(),
            Some(Auswahl {
                name: "Stelle".to_owned(),
                ziel: Ziel::Textstelle {
                    datei: datei.clone(),
                    zeile: 7,
                    zeileninhalt: "eine Zeile".to_owned(),
                },
                gueltig: true,
                fragt_die_platte: true,
            })
        );
        assert_eq!(
            modell.gewaehlt().as_ref().and_then(Auswahl::pfad),
            Some(datei.as_path()),
            "der Pfad einer Textmarke ist ihre Datei"
        );
    }

    /// Das zehnte Abnahmekriterium von C6: fehlt die Datei, ist die Auswahl
    /// ungueltig und nennt den Pfad, den der Aufrufer meldet — dieselbe Auskunft
    /// wie bei einer Ordnermarke auf einen verschwundenen Ordner.
    #[test]
    fn eine_textmarke_auf_eine_fehlende_datei_liefert_eine_ungueltige_auswahl() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::textstelle(
            "Fort",
            "/gibt/es/nicht/datei.txt",
            3,
            "eine Zeile",
        )]));
        let _ = nachpruefen(&mut modell);
        assert!(modell.waehlen(1));
        let auswahl = modell
            .gewaehlt()
            .expect("die Textmarke liefert eine Auswahl");
        assert!(!auswahl.gueltig);
        assert_eq!(auswahl.pfad(), Some(Path::new("/gibt/es/nicht/datei.txt")));
    }

    #[test]
    fn ein_ungueltiges_lesezeichen_traegt_seinen_zusatz_und_ein_geraet_nicht() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Temp", std::env::temp_dir()),
            Lesezeichen::neu("Fort", "/gibt-es-nicht-krk"),
        ]));
        let _ = nachpruefen(&mut modell);
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Temp"));
        assert!(!modell.ungueltig(1));
        assert_eq!(modell.beschriftung(2).as_deref(), Some("Fort (fehlt)"));
        assert!(modell.ungueltig(2));
    }

    #[test]
    fn ein_neues_lesezeichen_steht_unten_und_ist_ausgewaehlt() {
        let mut modell = modell();
        assert_eq!(
            aendern(
                &mut modell,
                &Aenderung::Anlegen {
                    name: "Drei".to_owned(),
                    ziel: ordnerziel("/drei"),
                },
            ),
            Ausgang::Geaendert(2),
            "ein neues Lesezeichen haengt unten an"
        );
        assert_eq!(modell.auswahl(), Some(3));
        assert_eq!(modell.beschriftung(3).as_deref(), Some("Drei (fehlt)"));
        assert_eq!(modell.lesezeichenliste().zahl(), 3);
    }

    /// Beide Sorten gehen durch dieselbe Tuer und stehen in einer Ordnung (C6).
    ///
    /// Die Probe zu [`Aenderung::Anlegen`] nach S38: der Vorgang traegt das
    /// fertige [`Ziel`], und die angelegte Textmarke haengt unten an wie eine
    /// Ordnermarke — keine eigene Ordnung, keine Sortierung nach Sorte.
    #[test]
    fn eine_angelegte_textmarke_haengt_unten_an_und_ist_ausgewaehlt() {
        let mut modell = modell();
        assert_eq!(
            aendern(
                &mut modell,
                &Aenderung::Anlegen {
                    name: "Die Lesestelle".to_owned(),
                    ziel: textziel("/eins/leser.rs", 118, "let x = 1;"),
                },
            ),
            Ausgang::Geaendert(2),
            "ein neues Lesezeichen haengt unten an"
        );
        assert_eq!(modell.auswahl(), Some(3));
        assert_eq!(modell.sinnbild(3), Some(Sinnbild::Textstelle));
        assert_eq!(
            modell.beschriftung(3).as_deref(),
            Some("Die Lesestelle (fehlt)")
        );
        assert_eq!(modell.lesezeichenliste().zahl(), 3);
    }

    #[test]
    fn die_vier_befehle_wirken_nur_auf_einem_lesezeichen() {
        let mut modell = modell();
        // Die Auswahl steht auf einem Geraet: es gibt kein Ziel, und die drei
        // Befehle, die eines brauchen, kommen gar nicht erst zum Rechnen.
        let _ = modell.waehlen(4);
        assert!(modell.gewaehltes_lesezeichen_wert().is_none());
        assert!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Umbenennen {
                welches,
                name: "Neu".to_owned(),
            })
            .is_none()
        );
        assert!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Loeschen { welches }).is_none()
        );
        assert!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Verschieben {
                welches,
                richtung: Verschiebung::Hoch,
            })
            .is_none()
        );
        assert_eq!(modell.lesezeichenliste().zahl(), 2);
    }

    /// Eine Liste, eine Ordnung, zwei Sinnbilder (C6).
    ///
    /// Die Reihenfolge ist die des Anlegens; nach Sorte sortiert wird nicht,
    /// und eine getrennte Ordnung fuer Textmarken gibt es nicht. Der Modulkopf
    /// von `krk_core::ablage::lesezeichen` begruendet es damit, dass zwei
    /// Ordnungen zwei Wahrheiten waeren.
    #[test]
    fn eine_gemischte_liste_behaelt_ihre_reihenfolge_und_zeigt_beide_sorten() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Projekte", "/p"),
            Lesezeichen::textstelle("Die Lesestelle", "/p/leser.rs", 118, "let x = 1;"),
            Lesezeichen::neu("Sicherung", "/s"),
        ]));
        let _ = nachpruefen(&mut modell);
        modell.orte_setzen(vec![Ort::neu("Macintosh HD", "/")], Vec::new());
        let _ = nachpruefen(&mut modell);

        assert_eq!(
            beschriftungen(&modell),
            [
                "Lesezeichen",
                "Projekte (fehlt)",
                "Die Lesestelle (fehlt)",
                "Sicherung (fehlt)",
                "Geräte und Orte",
                "Macintosh HD",
            ]
        );
        assert_eq!(
            sinnbilder(&modell),
            [
                None,
                Some(Sinnbild::Ordner),
                Some(Sinnbild::Textstelle),
                Some(Sinnbild::Ordner),
                None,
                Some(Sinnbild::Ordner),
            ],
            "eine Ueberschrift oeffnet nichts, ein Geraet einen Ordner"
        );
    }

    /// Ungueltig heisst fuer eine Textmarke allein, dass die Datei fehlt (C6).
    ///
    /// Der tragende Teil der Antwort vom 260808-0017: eine Marke, deren
    /// gemerkter Zeileninhalt nicht mehr auf der gemerkten Nummer steht, bleibt
    /// gueltig und bleibt ohne Kennzeichen. Ob der Inhalt noch stimmt,
    /// entscheidet sich beim Sprung und nur dort.
    #[test]
    fn eine_textmarke_ist_ungueltig_wenn_die_datei_fehlt_und_sonst_nie() {
        let ordner = Pruefordner::nur_name("textmarke-gueltigkeit");
        ordner.anlegen();
        let datei = ordner.datei("leser.rs", "fn eins() {}\nfn zwei() {}\n");

        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::textstelle("Verschoben", &datei, 2, "fn ganz_anders() {}"),
            Lesezeichen::textstelle("Fort", ordner.pfad().join("fehlt.rs"), 1, "fn eins() {}"),
        ]));
        let _ = nachpruefen(&mut modell);

        assert!(
            !modell.ungueltig(1),
            "der gemerkte Inhalt steht nicht mehr dort, die Datei aber schon"
        );
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Verschoben"));
        assert!(modell.ungueltig(2));
        assert_eq!(modell.beschriftung(2).as_deref(), Some("Fort (fehlt)"));
    }

    /// Die vier Lesezeichenbefehle aus C5 wirken auf eine Textmarke wie auf
    /// eine Ordnermarke (C6).
    ///
    /// Ohne eigenen Bau: [`Lesezeichenliste`] fragt an keiner Stelle nach der
    /// Sorte, und diese Probe haelt es an der Leiste fest.
    #[test]
    fn die_vier_lesezeichenbefehle_wirken_auf_eine_textmarke() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Projekte", "/p"),
            Lesezeichen::textstelle("Stelle", "/p/leser.rs", 118, "let x = 1;"),
        ]));
        let _ = nachpruefen(&mut modell);

        assert_eq!(
            aendern(
                &mut modell,
                &Aenderung::Anlegen {
                    name: "Zweite Stelle".to_owned(),
                    ziel: textziel("/p/schreiber.rs", 7, "fn sieben() {}"),
                },
            ),
            Ausgang::Geaendert(2),
            "ein neues Lesezeichen haengt unten an"
        );
        assert_eq!(modell.auswahl(), Some(3));

        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Umbenennen {
                welches,
                name: "Die zweite Stelle".to_owned(),
            }),
            Some(Ausgang::Geaendert(2))
        );
        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Verschieben {
                welches,
                richtung: Verschiebung::Hoch,
            }),
            Some(Ausgang::Geaendert(1))
        );
        assert_eq!(modell.auswahl(), Some(2), "die Auswahl wandert mit");
        assert_eq!(
            modell.beschriftung(2).as_deref(),
            Some("Die zweite Stelle (fehlt)")
        );
        assert_eq!(modell.sinnbild(2), Some(Sinnbild::Textstelle));

        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Loeschen { welches }),
            Some(Ausgang::Geaendert(1))
        );
        assert_eq!(modell.lesezeichenliste().zahl(), 2);
    }

    /// Das elfte Abnahmekriterium von C6: eine Textmarke kostet in der Leiste
    /// nicht mehr als eine Ordnermarke.
    ///
    /// **Was diese Probe misst und was nicht.** Die Zahl der Systemaufrufe ist
    /// von innerhalb des Prozesses nicht zu zaehlen. Messbar sind zwei
    /// Aussagen, und beide zusammen tragen die Zusage:
    ///
    /// - **Je Marke eine eigene Frage.** Zehn Marken auf fuenf vorhandene und
    ///   fuenf fehlende Dateien ergeben fuenf gueltige, und eine Datei, die
    ///   danach verschwindet, aendert genau ihre Marke. Eine gemeinsame oder
    ///   eine gemerkte Antwort fiele hier auf.
    /// - **Kein Lesevorgang.** Die letzte Marke zeigt auf eine Datei ohne
    ///   Leserecht. Sie bleibt gueltig; wer sie oeffnete, bekaeme `EACCES` und
    ///   muesste sie fuer ungueltig erklaeren. Unter `root` sagt diese Haelfte
    ///   weniger, weil dort auch das Oeffnen gelingt — falsch anschlagen kann
    ///   sie deshalb nicht.
    #[test]
    fn zehn_textmarken_kosten_je_eine_frage_und_keinen_lesevorgang() {
        let ordner = Pruefordner::nur_name("zehn-textmarken");
        ordner.anlegen();
        let mut marken = Vec::new();
        let mut vorhandene = Vec::new();
        for nummer in 0..10u32 {
            let name = format!("zeile-{nummer}.txt");
            let pfad = match nummer % 2 == 0 {
                true => {
                    let pfad = ordner.datei(&name, "eine Zeile\n");
                    vorhandene.push(pfad.clone());
                    pfad
                }
                false => ordner.pfad().join(&name),
            };
            marken.push(Lesezeichen::textstelle(
                format!("Marke {nummer}"),
                pfad,
                1,
                "eine Zeile",
            ));
        }
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(marken));
        let _ = nachpruefen(&mut modell);
        assert_eq!(gueltige_marken(&modell, 10), 5);

        std::fs::remove_file(&vorhandene[0]).expect("die Pruefdatei laesst sich nicht loeschen");
        assert!(
            nachpruefen(&mut modell),
            "ohne die gemeldete Aenderung zeichnete die Leiste nicht neu"
        );
        assert!(modell.ungueltig(1));
        assert_eq!(gueltige_marken(&modell, 10), 4);

        let verschlossen = ordner.datei("verschlossen.txt", "eine Zeile\n");
        std::fs::set_permissions(&verschlossen, std::fs::Permissions::from_mode(0o000))
            .expect("das Leserecht laesst sich nicht nehmen");
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::textstelle(
            "Verschlossen",
            &verschlossen,
            1,
            "eine Zeile",
        )]));
        let _ = nachpruefen(&mut modell);
        assert!(
            !modell.ungueltig(1),
            "die Pruefung fragt nach der Datei und liest sie nicht"
        );
    }

    /// Wie viele der ersten `zahl` Lesezeichenzeilen gueltig sind.
    fn gueltige_marken(modell: &Leistenmodell, zahl: usize) -> usize {
        (1..=zahl)
            .filter(|stelle| !modell.ungueltig(*stelle))
            .count()
    }

    #[test]
    fn die_auswahl_wandert_mit_dem_verschobenen_lesezeichen() {
        let mut modell = modell();
        let _ = modell.waehlen(2);
        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Verschieben {
                welches,
                richtung: Verschiebung::Hoch,
            }),
            Some(Ausgang::Geaendert(0))
        );
        assert_eq!(modell.auswahl(), Some(1));
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Zwei (fehlt)"));
        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Verschieben {
                welches,
                richtung: Verschiebung::Hoch,
            }),
            Some(Ausgang::Unveraendert),
            "oben ist Schluss"
        );
    }

    #[test]
    fn nach_dem_loeschen_des_letzten_lesezeichens_steht_die_auswahl_auf_dem_ersten_geraet() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Eins", "/eins",
        )]));
        modell.orte_setzen(vec![Ort::neu("Benutzer", "/Users/pruefung")], Vec::new());
        let _ = modell.waehlen(1);

        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Loeschen { welches }),
            Some(Ausgang::Geaendert(0))
        );

        assert_eq!(modell.lesezeichenliste().zahl(), 0);
        assert_eq!(
            modell.gewaehlt().map(|auswahl| auswahl.name),
            Some("Benutzer".to_owned()),
            "die Auswahl darf nicht auf der Ueberschrift liegenbleiben"
        );
    }

    #[test]
    fn ein_geraet_mehr_laesst_die_auswahl_stehen() {
        let mut modell = modell();
        let _ = modell.waehlen(2);
        modell.orte_setzen(
            vec![
                Ort::neu("Benutzer", "/Users/pruefung"),
                Ort::neu("Macintosh HD", "/"),
                Ort::neu("Sicherung", "/Volumes/Sicherung"),
            ],
            Vec::new(),
        );
        assert_eq!(
            modell.gewaehlt().map(|auswahl| auswahl.name),
            Some("Zwei".to_owned()),
            "ein eingehaengter Datentraeger verschiebt die Auswahl des Nutzers nicht"
        );
    }

    #[test]
    fn ein_eingehaengter_datentraeger_macht_sein_lesezeichen_gueltig() {
        let ordner = Pruefordner::nur_name("gueltigkeit");
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Kommt",
            ordner.pfad(),
        )]));
        let _ = nachpruefen(&mut modell);
        assert!(modell.ungueltig(1));

        ordner.anlegen();
        let _ = nachpruefen(&mut modell);
        assert!(!modell.ungueltig(1));

        ordner.loeschen();
        let _ = nachpruefen(&mut modell);
        assert!(modell.ungueltig(1));
    }

    /// Der Anlass aus C5: eine abgeschlossene Dateioperation hat den Ordner eines
    /// Lesezeichens geloescht.
    ///
    /// Die Stelle, die den Anlass ausloest, sitzt in
    /// `crate::appkit::anwendung::Anwendungsdelegierter::vorgang_beenden` und
    /// ist nur ueber AppKit erreichbar. Pruefbar ohne Fenster ist, was sie dort
    /// ruft, und dabei vor allem der **Rueckgabewert**: an ihm allein haengt das
    /// Neuzeichnen der Leiste. Meldete die Pruefung `false`, bliebe die Zeile
    /// schwarz, obwohl der Ordner fort ist.
    #[test]
    fn nach_einer_dateioperation_meldet_die_pruefung_den_geloeschten_ordner() {
        let ordner = Pruefordner::nur_name("vorgang-beenden");
        ordner.anlegen();
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Sicherung",
            ordner.pfad(),
        )]));
        assert!(!modell.ungueltig(1), "der Ordner steht noch");

        // Was eine Dateioperation aus C4 tut, wenn sie ihn loescht.
        ordner.loeschen();

        assert!(
            nachpruefen(&mut modell),
            "ohne die gemeldete Aenderung zeichnete die Leiste nicht neu"
        );
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Sicherung (fehlt)"));
        assert!(
            !nachpruefen(&mut modell),
            "ein zweiter Anlass ohne Aenderung zeichnet die Leiste nicht noch einmal"
        );
    }

    /// Eine zweite Frage ohne Aenderung auf der Platte findet nichts zu aendern.
    ///
    /// Die Probe haengt allein am **Rueckgabewert**: an ihm haengt das Neuzeichnen
    /// der Leiste, und eine Antwort ohne Aenderung zeichnet nicht.
    #[test]
    fn der_aufbau_und_das_nachziehen_kommen_zum_selben_ergebnis() {
        let vorhanden = Pruefordner::nur_name("aufbau-vorhanden");
        vorhanden.anlegen();
        let fehlend = Pruefordner::nur_name("aufbau-fehlend");

        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Da", vorhanden.pfad()),
            Lesezeichen::neu("Fort", fehlend.pfad()),
        ]));
        let _ = nachpruefen(&mut modell);
        assert!(!modell.ungueltig(1));
        assert!(modell.ungueltig(2));

        assert!(
            !nachpruefen(&mut modell),
            "die erste Antwort hat die Marken schon richtig gesetzt"
        );
    }

    /// Eine neue Ortsliste und ein Anstoss danach ziehen die Marken mit. Dass die
    /// Ansicht ihn stoesst, steht in `Leistenquelle::orte_setzen`.
    #[test]
    fn eine_neue_ortsliste_zieht_die_gueltigkeit_nach() {
        let ordner = Pruefordner::nur_name("orte-setzen");
        ordner.anlegen();
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Sicherung",
            ordner.pfad(),
        )]));
        let _ = nachpruefen(&mut modell);
        assert!(!modell.ungueltig(1), "der Datentraeger ist eingehaengt");

        // Was ein Auswurf hinterlaesst: der Ordner ist fort, und das System
        // meldet die kuerzere Ortsliste.
        ordner.loeschen();
        modell.orte_setzen(vec![Ort::neu("Macintosh HD", "/")], Vec::new());
        let _ = nachpruefen(&mut modell);

        assert!(modell.ungueltig(1));
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Sicherung (fehlt)"));
    }

    /// Eine Leiste mit einem Lesezeichen auf einen Datentraeger, dem
    /// Benutzerverzeichnis, dem Startdatentraeger und zwei eingehaengten
    /// Datentraegern, so wie `geraete_uebernehmen` sie setzt: die Orte samt
    /// Benutzerverzeichnis, die Einhaengepunkte ohne es.
    fn leiste_mit_datentraegern() -> Leistenmodell {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Stick",
            "/Volumes/Stick",
        )]));
        modell.orte_setzen(
            vec![
                Ort::neu("pruefung", "/Users/pruefung"),
                Ort::neu("Macintosh HD", "/"),
                Ort::neu("Stick", "/Volumes/Stick"),
                Ort::neu("Freigabe", "/Volumes/Freigabe"),
            ],
            vec![
                PathBuf::from("/"),
                PathBuf::from("/Volumes/Stick"),
                PathBuf::from("/Volumes/Freigabe"),
            ],
        );
        modell
    }

    /// Auswerfbar ist genau ein Ort, dessen Pfad ein Einhaengepunkt ist und
    /// nicht die Wurzel (261016).
    ///
    /// Die Zeilen: 0 Ueberschrift, 1 Lesezeichen, 2 Ueberschrift, 3
    /// Benutzerverzeichnis, 4 Startdatentraeger, 5 und 6 die Datentraeger. Das
    /// Lesezeichen zeigt auf denselben Pfad wie Zeile 5 und ist trotzdem
    /// nicht auswerfbar: es ist eine Marke des Nutzers und kein Geraet.
    #[test]
    fn auswerfbar_ist_allein_ein_eingehaengter_datentraeger_ausser_der_wurzel() {
        let modell = leiste_mit_datentraegern();
        let ziele: Vec<Option<PathBuf>> = (0..modell.zeilen().len())
            .map(|stelle| modell.auswurfziel(stelle))
            .collect();
        assert_eq!(
            ziele,
            [
                None,
                None,
                None,
                None,
                None,
                Some(PathBuf::from("/Volumes/Stick")),
                Some(PathBuf::from("/Volumes/Freigabe")),
            ]
        );
        assert_eq!(modell.auswurfziel(99), None);
    }

    /// Ohne bekannte Einhaengepunkte ist keine Zeile auswerfbar, auch eine
    /// unter `/Volumes` nicht: die Regel fragt die Liste und nicht den Pfad.
    #[test]
    fn ohne_einhaengepunkte_ist_keine_zeile_auswerfbar() {
        let mut modell = leiste_mit_datentraegern();
        modell.orte_setzen(vec![Ort::neu("Stick", "/Volumes/Stick")], Vec::new());
        assert!((0..modell.zeilen().len()).all(|stelle| modell.auswurfziel(stelle).is_none()));
    }

    /// Ein Datentraeger, der ausgehaengt ist, verliert mit der neuen
    /// Ortsliste Zeile und Auswerfbarkeit zugleich (261016).
    #[test]
    fn nach_dem_aushaengen_ist_der_datentraeger_nicht_mehr_auswerfbar() {
        let mut modell = leiste_mit_datentraegern();
        modell.orte_setzen(
            vec![
                Ort::neu("pruefung", "/Users/pruefung"),
                Ort::neu("Macintosh HD", "/"),
                Ort::neu("Freigabe", "/Volumes/Freigabe"),
            ],
            vec![PathBuf::from("/"), PathBuf::from("/Volumes/Freigabe")],
        );
        let ziele: Vec<PathBuf> = (0..modell.zeilen().len())
            .filter_map(|stelle| modell.auswurfziel(stelle))
            .collect();
        assert_eq!(ziele, [PathBuf::from("/Volumes/Freigabe")]);
    }

    /// Beim Start und direkt nach dem Hinzufuegen steht kein Eintrag als kaputt
    /// da, auch nicht einer, dessen Ziel fehlt; die Antwort setzt die Marke.
    #[test]
    fn bis_zur_antwort_gilt_ein_neues_lesezeichen_als_gueltig_und_die_antwort_setzt_die_marke() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Fort",
            "/gibt-es-nicht-krk",
        )]));
        assert!(!modell.ungueltig(1), "vor der Antwort kein Zusatz");
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Fort"));

        assert!(nachpruefen(&mut modell), "die Antwort aendert die Marke");
        assert!(modell.ungueltig(1));
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Fort (fehlt)"));

        assert!(!nachpruefen(&mut modell), "ohne Aenderung kein Neuzeichnen");
    }

    /// Eine Antwort geht nach dem Ziel: der entfernte Eintrag findet keinen
    /// Abnehmer, der neue bekommt nichts, und der umbenannte behaelt seine Marke.
    #[test]
    fn eine_ueberholte_antwort_setzt_fuer_ein_entferntes_oder_neues_lesezeichen_nichts_falsches() {
        let bleibt = Lesezeichen::neu("Bleibt", "/gibt-es-nicht-krk/bleibt");
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Entfernt", "/gibt-es-nicht-krk/entfernt"),
            bleibt.clone(),
        ]));
        let mut fragen = modell.gueltigkeit_anstossen();
        assert_eq!(fragen.len(), 1);
        let antwort = fragen.remove(0).beantworten();

        let mut umbenannt = bleibt;
        umbenannt.name = "Neuer Name".to_owned();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Neu", "/gibt-es-nicht-krk/neu"),
            umbenannt,
        ]));
        assert!(modell.gueltigkeit_anstossen().is_empty(), "sie laeuft noch");

        let uebernahme = modell.gueltigkeit_uebernehmen(antwort);
        assert!(uebernahme.geaendert);
        assert!(
            !modell.ungueltig(1),
            "fuer den neuen sagt die Antwort nichts"
        );
        assert_eq!(
            modell.beschriftung(2).as_deref(),
            Some("Neuer Name (fehlt)"),
            "der umbenannte bekommt die Antwort seines Ziels"
        );
        assert_eq!(modell.lesezeichenliste().zahl(), 2);

        let nachfrage = uebernahme.nachfrage.expect("die Liste hat sich geaendert");
        let uebernahme = modell.gueltigkeit_uebernehmen(nachfrage.beantworten());
        assert!(uebernahme.nachfrage.is_none());
        assert!(modell.ungueltig(1), "die Nachfrage erreicht den neuen");
    }

    /// Eine neue Liste nimmt keinem Eintrag seinen Stand: was seine Marke schon
    /// hat, behaelt sie auf dem Weg durch eine Aenderung an einem anderen Eintrag.
    #[test]
    fn eine_neue_liste_behaelt_den_stand_jedes_ziels_das_sie_weiter_fuehrt() {
        let fort = Lesezeichen::neu("Fort", "/gibt-es-nicht-krk");
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![fort.clone()]));
        let _ = nachpruefen(&mut modell);
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Temp", std::env::temp_dir()),
            fort,
        ]));
        assert!(!modell.ungueltig(1));
        assert!(modell.ungueltig(2), "die Zeile blinkt nicht auf gueltig");
    }

    /// Hundert Meldungen stellen eine Frage und eine Nachfrage, und danach keine
    /// weitere.
    #[test]
    fn ein_sturm_von_meldungen_stellt_eine_frage_und_eine_nachfrage() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Temp",
            std::env::temp_dir(),
        )]));
        let mut fragen = modell.gueltigkeit_anstossen();
        assert_eq!(fragen.len(), 1, "die erste Meldung stellt die Frage");
        for _ in 0..100 {
            assert!(modell.gueltigkeit_anstossen().is_empty());
        }
        let uebernahme = modell.gueltigkeit_uebernehmen(fragen.remove(0).beantworten());
        let nachfrage = uebernahme.nachfrage.expect("die eine Nachfrage");
        assert!(
            modell.gueltigkeit_anstossen().is_empty(),
            "sie laeuft schon"
        );
        let uebernahme = modell.gueltigkeit_uebernehmen(nachfrage.beantworten());
        let nachfrage = uebernahme.nachfrage.expect("die Meldung in die Nachfrage");
        let uebernahme = modell.gueltigkeit_uebernehmen(nachfrage.beantworten());
        assert!(uebernahme.nachfrage.is_none(), "ohne Meldung keine weitere");
        assert_eq!(modell.gueltigkeit_anstossen().len(), 1, "danach neu");
    }

    /// Eine Frage, die nie zurueckkommt, haelt allein die Marken ihrer Gruppe an;
    /// die Fadenzahl waechst dabei nicht, auch nicht unter weiteren Meldungen.
    #[test]
    fn eine_haengende_freigabe_reisst_die_marken_der_uebrigen_nicht_mit() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Haengt", "/Volumes/krk-gibt-es-nicht-a/ordner"),
            Lesezeichen::neu("Auch dort", "/Volumes/krk-gibt-es-nicht-a/anderer"),
            Lesezeichen::neu("Zweite", "/Volumes/krk-gibt-es-nicht-b/ordner"),
            Lesezeichen::neu("Hier", "/gibt-es-nicht-krk"),
            Lesezeichen::verbindung("NAS"),
        ]));
        let fragen = modell.gueltigkeit_anstossen();
        assert_eq!(
            fragen.len(),
            3,
            "je Gruppe eine, die Verbindung fragt nicht"
        );

        // Die erste Frage, die der Gruppe `/`, steht vorn: die Gruppen sind
        // sortiert. Es haengt die von `/Volumes/krk-gibt-es-nicht-a`.
        let mut fragen = fragen.into_iter();
        let wurzel = fragen.next().expect("die Wurzel");
        let haengt = fragen.next().expect("die haengende Freigabe");
        let zweite = fragen.next().expect("die zweite Freigabe");
        let _ = modell.gueltigkeit_uebernehmen(wurzel.beantworten());
        let _ = modell.gueltigkeit_uebernehmen(zweite.beantworten());

        assert!(
            !modell.ungueltig(1),
            "ihre Marken bleiben beim letzten Stand"
        );
        assert!(!modell.ungueltig(2));
        assert!(modell.ungueltig(3), "die andere Freigabe hat geantwortet");
        assert!(modell.ungueltig(4), "der lokale Eintrag hat geantwortet");

        let neue = modell.gueltigkeit_anstossen();
        assert_eq!(neue.len(), 2, "die haengende Gruppe stellt keine zweite");
        drop(neue);
        drop(haengt);
    }

    /// Die Antwort fuer eine Gruppe, die inzwischen leer ist, hinterlaesst keinen
    /// Takt, der fuer immer als laufend steht.
    #[test]
    fn eine_leer_gewordene_gruppe_gibt_ihren_takt_frei() {
        let mut modell = Leistenmodell::neu();
        let marke = Lesezeichen::neu("Fort", "/Volumes/krk-gibt-es-nicht/ordner");
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![marke.clone()]));
        let mut fragen = modell.gueltigkeit_anstossen();
        let frage = fragen.remove(0);
        assert!(modell.gueltigkeit_anstossen().is_empty(), "die Nachfrage");
        modell.lesezeichen_setzen(&Lesezeichenliste::default());
        let uebernahme = modell.gueltigkeit_uebernehmen(frage.beantworten());
        assert!(
            uebernahme.nachfrage.is_none(),
            "es gibt nichts mehr zu fragen"
        );

        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![marke]));
        assert_eq!(modell.gueltigkeit_anstossen().len(), 1, "der Takt ist frei");
    }

    /// Eine Antwort ohne Befund, nach gescheitertem Fadenstart oder einer Panik,
    /// laesst die Marken stehen und den Takt weiterlaufen.
    #[test]
    fn eine_antwort_ohne_befund_laesst_die_marken_stehen_und_gibt_den_takt_frei() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Fort",
            "/gibt-es-nicht-krk",
        )]));
        let mut fragen = modell.gueltigkeit_anstossen();
        let uebernahme = modell.gueltigkeit_uebernehmen(fragen.remove(0).ohne_befund());
        assert!(!uebernahme.geaendert);
        assert!(uebernahme.nachfrage.is_none());
        assert!(!modell.ungueltig(1));
        assert_eq!(modell.gueltigkeit_anstossen().len(), 1);
    }

    /// Eine Liste ohne Pfade stellt keine Frage, also auch die leere des Messmodus.
    #[test]
    fn ohne_pfade_entsteht_keine_frage() {
        let mut modell = Leistenmodell::neu();
        modell.orte_setzen(vec![Ort::neu("Macintosh HD", "/")], Vec::new());
        assert!(
            modell.gueltigkeit_anstossen().is_empty(),
            "null Lesezeichen"
        );
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::verbindung("NAS")]));
        assert!(modell.gueltigkeit_anstossen().is_empty(), "eine Verbindung");
    }

    /// Die Auswahl eines Lesezeichens fragt die Platte, die eines Geraets und einer
    /// Verbindung nicht; `nachgefragt` setzt den Stand von jetzt, nach beiden
    /// Seiten, und der Befund erreicht die Marke.
    #[test]
    fn eine_auswahl_erfaehrt_den_stand_von_jetzt() {
        let ordner = Pruefordner::nur_name("auswahlfrage");
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Marke", ordner.pfad()),
            Lesezeichen::verbindung("NAS"),
        ]));
        modell.orte_setzen(vec![Ort::neu("Fort", "/gibt-es-nicht-krk")], Vec::new());

        assert!(modell.waehlen(1));
        let auswahl = modell.gewaehlt().expect("das Lesezeichen");
        assert!(auswahl.fragt_die_platte);
        assert!(auswahl.gueltig, "angezeigt als gueltig");
        let antwort = auswahl.nachgefragt();
        assert!(!antwort.gueltig, "tatsaechlich fehlt es");
        assert!(modell.auswahlbefund_uebernehmen(&antwort));
        assert!(modell.ungueltig(1));

        ordner.anlegen();
        let antwort = modell.gewaehlt().expect("das Lesezeichen").nachgefragt();
        assert!(
            antwort.gueltig,
            "angezeigt als fehlend, und es ist wieder da"
        );
        assert!(modell.auswahlbefund_uebernehmen(&antwort));
        assert!(!modell.ungueltig(1));
        assert!(
            !modell.auswahlbefund_uebernehmen(&antwort),
            "keine Aenderung"
        );

        let letzte = modell.zeilen().len() - 1;
        assert!(modell.waehlen(letzte));
        let geraet = modell.gewaehlt().expect("das Geraet");
        assert!(!geraet.fragt_die_platte);
        assert!(
            geraet.nachgefragt().gueltig,
            "ein Geraet fragt auch dann nicht"
        );

        assert!(modell.waehlen(3));
        let verbindung = modell.gewaehlt().expect("die Verbindung");
        assert_eq!(
            verbindung.ziel,
            Ziel::Verbindung {
                verbindung: "NAS".to_owned()
            }
        );
        assert!(!verbindung.fragt_die_platte);
    }

    /// Eine Liste mit zwei Lesezeichen und einer Verbindung, in dieser Folge ab
    /// Zeile 1; Zeile 3 ist die Ueberschrift, Zeile 4 die Verbindung.
    fn modell_fuer_klicks() -> Leistenmodell {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Eins", "/eins"),
            Lesezeichen::neu("Zwei", "/zwei"),
            Lesezeichen::verbindung("NAS"),
        ]));
        modell
    }

    /// Der Mausklick auf eine neue Zeile, wie AppKit ihn zustellt: erst die
    /// Auswahlmeldung, die den Halter erreicht, dann die `action`. Liefert, ob die
    /// `action` die Auswahl ein zweites Mal ausgeloest haette.
    fn neue_zeile_klicken(modell: &mut Leistenmodell, zeile: usize) -> bool {
        assert!(modell.waehlen(zeile));
        assert!(modell.auswahl_zum_melden().is_some());
        modell.klick_loest_aus(Some(zeile))
    }

    /// Die Abnahme des Defekts: nach einer verworfenen Antwort loest derselbe Klick
    /// die Auswahl erneut aus, nach einer vollzogenen nicht.
    #[test]
    fn nach_verworfener_antwort_loest_der_klick_erneut_aus_nach_vollzogener_nicht() {
        let mut modell = modell_fuer_klicks();

        assert!(
            !neue_zeile_klicken(&mut modell, 1),
            "der erste Klick hat seine Auswahl schon gemeldet und fragt nicht zweimal"
        );
        // Die Antwort faellt: es kommt kein `auswahl_vollzogen`.
        assert!(modell.klick_loest_aus(Some(1)), "verworfen: noch einmal");
        assert!(modell.auswahl_zum_melden().is_some());
        modell.auswahl_vollzogen();
        assert!(!modell.klick_loest_aus(Some(1)), "vollzogen: nichts");
        assert!(!modell.klick_loest_aus(Some(1)), "und bleibt so");
        assert_eq!(modell.auswahl(), Some(1), "die Zeile bleibt gewaehlt");
    }

    /// Ein Klick bei noch ausstehender Antwort loest aus, und der Stand wartet auf
    /// die juengste: eine ueberholte Antwort erreicht `auswahl_vollzogen` nie.
    #[test]
    fn jeder_klick_bei_ausstehender_antwort_loest_aus_bis_die_juengste_gilt() {
        let mut modell = modell_fuer_klicks();
        assert!(!neue_zeile_klicken(&mut modell, 1));

        for _ in 0..2 {
            assert!(modell.klick_loest_aus(Some(1)));
            assert!(modell.auswahl_zum_melden().is_some());
        }
        modell.auswahl_vollzogen();
        assert!(!modell.klick_loest_aus(Some(1)));
    }

    /// Die Tastatur meldet wie die Maus, aber ihr folgt keine `action`: der erste
    /// Klick danach ist schon der zweite Versuch. Ein von der Maus liegen
    /// gebliebener Merker nimmt ihm das nicht.
    #[test]
    fn die_tastatur_laesst_den_ersten_klick_wirken() {
        let mut modell = modell_fuer_klicks();
        assert!(modell.auswahl_bewegen(1), "ohne Auswahl die erste Zeile");
        assert_eq!(modell.auswahl(), Some(1));
        assert!(modell.auswahl_zum_melden().is_some());
        assert!(modell.klick_loest_aus(Some(1)));

        let mut modell = modell_fuer_klicks();
        // Der Merker bleibt liegen: der Klick endet ohne `action`.
        assert!(modell.waehlen(1));
        assert!(modell.auswahl_zum_melden().is_some());
        assert!(modell.auswahl_bewegen(1));
        assert!(modell.auswahl_zum_melden().is_some());
        assert!(modell.klick_loest_aus(Some(2)));
    }

    /// Eine vollzogene Auswahl von der Tastatur ist so vollzogen wie eine der Maus.
    #[test]
    fn nach_vollzogener_auswahl_von_der_tastatur_tut_der_klick_nichts() {
        let mut modell = modell_fuer_klicks();
        assert!(modell.auswahl_bewegen(1));
        assert!(modell.auswahl_zum_melden().is_some());
        modell.auswahl_vollzogen();
        assert!(!modell.klick_loest_aus(Some(1)));
    }

    /// Ausgeloest wird allein die gewaehlte Zeile, und ein Klick daneben verbraucht
    /// den Merker seines eigenen Klicks trotzdem.
    #[test]
    fn nur_ein_klick_auf_die_gewaehlte_zeile_loest_aus() {
        let mut modell = modell_fuer_klicks();
        assert!(!neue_zeile_klicken(&mut modell, 1));

        assert!(!modell.klick_loest_aus(Some(2)), "eine andere Zeile");
        assert!(!modell.klick_loest_aus(None), "neben die Zeilen");
        assert!(!modell.klick_loest_aus(Some(0)), "eine Ueberschrift");
        assert!(modell.klick_loest_aus(Some(1)));

        assert!(modell.waehlen(2));
        assert!(modell.auswahl_zum_melden().is_some());
        assert!(!modell.klick_loest_aus(None), "der Merker faellt auch hier");
        assert!(modell.klick_loest_aus(Some(2)));

        let mut leer = Leistenmodell::neu();
        assert!(!leer.klick_loest_aus(None), "ohne Auswahl nichts");
    }

    /// Offen ist das Ziel und nicht die Zeile: rueckt unter derselben Stelle ein
    /// anderer Eintrag nach, hat niemand dessen Auswahl gemeldet.
    #[test]
    fn offen_bleibt_das_ziel_und_nicht_die_stelle() {
        let mut modell = modell_fuer_klicks();
        assert!(!neue_zeile_klicken(&mut modell, 1));

        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Zwei", "/zwei"),
            Lesezeichen::verbindung("NAS"),
        ]));
        assert_eq!(modell.auswahl(), Some(1));
        assert!(!modell.klick_loest_aus(Some(1)), "ein anderes Ziel");

        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Null", "/null"),
            Lesezeichen::neu("Eins", "/eins"),
        ]));
        assert_eq!(modell.auswahl(), Some(1));
        assert!(!modell.klick_loest_aus(Some(1)), "dieselbe Stelle, /null");
        assert!(!modell.klick_loest_aus(Some(2)), "/eins ist nicht gewaehlt");
    }

    /// Eine umbenannte Marke bleibt offen, weil ihr Ziel dasselbe ist.
    #[test]
    fn eine_umbenannte_marke_bleibt_offen() {
        let mut modell = modell_fuer_klicks();
        assert!(!neue_zeile_klicken(&mut modell, 1));
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Umbenannt", "/eins"),
            Lesezeichen::neu("Zwei", "/zwei"),
            Lesezeichen::verbindung("NAS"),
        ]));
        assert!(modell.klick_loest_aus(Some(1)));
    }

    /// Die Einteilung liest allein den Pfadtext.
    #[test]
    fn die_plattengruppe_folgt_dem_pfadtext() {
        for (pfad, gruppe) in [
            ("/Volumes/NAS/Ordner/Datei", "/Volumes/NAS"),
            ("/Volumes/NAS", "/Volumes/NAS"),
            ("/net/rechner/freigabe", "/net/rechner"),
            ("/Volumes", "/"),
            ("/Users/k1/Volumes/NAS", "/"),
            ("/", "/"),
            ("relativ/Volumes/NAS", "/"),
        ] {
            assert_eq!(plattengruppe(Path::new(pfad)), Path::new(gruppe), "{pfad}");
        }
    }
}
