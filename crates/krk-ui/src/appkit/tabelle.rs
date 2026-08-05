//! Das Dateifenster: Tableiste, `NSTableView` in einer `NSScrollView` und
//! Statuszeile, angebunden an das Tabmodell aus [`crate::tabs`].
//!
//! ```text
//! ┌──────────────────────────────┐
//! │ Tableiste (appkit::tableiste)│  ein Abschnitt je Tab
//! ├──────────────────────────────┤
//! │ NSScrollView                 │  der Inhalt des sichtbaren Tabs
//! │   NSTableView, vier Spalten  │
//! ├──────────────────────────────┤
//! │ Statuszeile                  │  Meldungen zu diesem Dateifenster
//! └──────────────────────────────┘
//! ```
//!
//! Zwei Objective-C-Klassen teilen sich die Arbeit, weil AppKit sie an zwei
//! Protokollen entgegennimmt. [`DateifensterQuelle`] ist die Datenquelle: sie
//! haelt das Tabmodell, startet Lesevorgaenge und meldet die Zeilenzahl.
//! [`DateifensterDelegierter`] ist der Delegierte: er baut die Zellen und
//! beschriftet sie. Der Delegierte haelt die Quelle, nicht umgekehrt, denn er
//! liest aus ihr; die Gegenrichtung gibt es nicht und damit auch keinen Zyklus.
//!
//! **Was hier steht und was im Modell.** Die Tabs, ihre Ordner, ihr Inhalt,
//! ihre Auswahl und ihre Bildlaufposition wohnen in [`Tabliste`] und damit
//! ausserhalb von `appkit/`. Diese Datei setzt und liest sie an der
//! `NSTableView` und der `NSScrollView`, weil beides AppKit-Aufrufe sind, und
//! trifft keine Entscheidung darueber, welcher Tab wohin gehoert.
//!
//! **Wie die Stapel den Hauptfaden erreichen.** Der Leser aus `krk-core` laeuft
//! je Tab auf einem Arbeitsfaden und schickt Stapel zu 1.024 Eintraegen ueber
//! einen Kanal der Tiefe 1. Ein Zeitgeber auf dem Hauptfaden raeumt alle Kanaele
//! dieses Dateifensters sechzigmal je Sekunde leer, haengt die Stapel an das
//! Modell des jeweiligen Tabs und meldet der Tabelle **einmal** je Takt eine
//! neue Zeilenzahl, sofern der sichtbare Tab betroffen war. Damit erfuellt der
//! erste Stapel die Zusage L2 (erste Bildschirmseite sichtbar), waehrend der
//! Rest anhaengt, und die Tabelle zeichnet hoechstens einmal je Bild neu.
//!
//! **Was einen Ordnerwechsel mitten im Lesen traegt.** Nicht die
//! Generationsnummer, die jeder Stapel mitfuehrt. Ein Tab haelt immer nur genau
//! einen Lesevorgang und liest allein aus dessen Kanal; jede Meldung, die er zu
//! sehen bekommt, traegt deshalb ohnehin die Generation seines Modells.
//! Getragen wird der Wechsel davon, dass `Tabliste::ordner_setzen` den alten
//! `Lesevorgang` fallen laesst. Damit faellt sein Empfaenger, und der alte Lauf
//! wird wirklich beendet statt nur ueberhoert: `Lesevorgang::drop` setzt das
//! Abbruchkennzeichen, das der Lesefaden vor jedem Systemaufruf und zwischen
//! zwei Stapeln prueft, und spaetestens das naechste `send` scheitert am
//! verschwundenen Empfaenger. Der Abbruch greift innerhalb von zwei Stapeln.
//!
//! Die Nummer bleibt und traegt anderes: sie benennt den Lesefaden und sagt dem
//! Modell beim Leeren, zu welchem Lauf sein Inhalt gehoert. Eine Prueferei je
//! Stapel stand hier bis zum 260803 daneben. Sie konnte nie greifen, weil
//! Modell- und Lesevorgangsgeneration in denselben zwei Zeilen auf denselben
//! Wert gesetzt werden, und sie verdeckte den Mechanismus, der wirklich traegt.
//! Wer sie fuer mehrere gleichzeitige Lesevorgaenge **desselben Tabs** wieder
//! braucht, bringt den Fall mit, in dem sie greift.
//!
//! **Die Ausleihen des Tabmodells enden vor jedem AppKit-Aufruf.** `RefCell`
//! ist der Punkt, an dem ein Rueckschlag aus AppKit das Programm abstuerzen
//! liesse: `reloadData` ruft die Datenquelle, und die will dasselbe Modell
//! lesen. Jede Ausleihe unten steht deshalb in einer eigenen Anweisung, und
//! keine ueberlebt eine Zeile mit einem Objective-C-Aufruf.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSColor, NSControlTextEditingDelegate, NSFont, NSScrollView,
    NSTableColumn, NSTableView, NSTableViewColumnAutoresizingStyle, NSTableViewDataSource,
    NSTableViewDelegate, NSTableViewStyle, NSTextAlignment, NSTextField,
    NSUserInterfaceItemIdentification, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSByteCountFormatter, NSByteCountFormatterCountStyle, NSDate,
    NSDateFormatter, NSDateFormatterStyle, NSIndexSet, NSInteger, NSNotification, NSObject,
    NSObjectProtocol, NSPoint, NSRect, NSRunLoop, NSRunLoopCommonModes, NSSize, NSString,
    NSTimeInterval, NSTimer, ns_string,
};

use krk_core::ablage::Dateifenster as Fensterzustand;
use krk_core::tasten::Kommando;
use krk_core::verzeichnis::sprungmarke::{self, Sprungmarke};
use krk_core::verzeichnis::{Eintrag, Ordnermodell, Schluessel, Sortierung, Typ, aufwaerts};
use krk_core::zwischenablage::{self, Ziel};

use crate::kommandos::auswahl::{self, markieren_und_weiter};
use crate::kommandos::navigation::{Bewegung, zielzeile};
use crate::kommandos::operationen::{self, Umbenennungswunsch};
use crate::kommandos::pfadeingabe::{self, Ergebnis};
use crate::tabs::Tabliste;

use super::blaetter;
use super::statuszeile::{self, Statuszeile};
use super::tableiste::Tableiste;

/// Die Hoehe einer Zeile in Punkten.
///
/// Sie ist fest und wird nicht je Zeile geschaetzt. Eine Dateiliste hat gleich
/// hohe Zeilen, damit rechnet AppKit die Gesamthoehe konstant statt linear, und
/// erst das macht die Bildlaufleiste eines Ordners mit 100.000 Eintraegen
/// sofort richtig (L10).
const ZEILENHOEHE: f64 = 20.0;

/// Der Takt, in dem der Hauptfaden die Kanaele der Leser leerraeumt.
///
/// Ein Sechzigstel einer Sekunde ist ein Bild auf dem Referenzgeraet, und das
/// ist seit Schritt 8 erhoben und keine Annahme mehr: [`super::bildtakt`] liest
/// die Rate aus `NSScreen.maximumFramesPerSecond` am Bildschirm des gemessenen
/// Fensters, und der Bedingungskopf jedes Messberichts schreibt sie aus, zuletzt
/// `messungen/260803-1641-durchstich.txt` mit 60 Hz. Haeufiger zu raeumen
/// brauchte es dort nicht, weil die Tabelle ohnehin nicht oefter zeichnet.
///
/// **Der Takt liest die Rate trotzdem nicht, und das ist eine Festlegung.** Auf
/// einem Bildschirm mit 120 Hz raeumte er nur bei jedem zweiten Bild: die Liste
/// baute sich langsamer auf, als der Schirm es zuliesse, waehrend die Zusage aus
/// dem Modulkopf, hoechstens einmal je Bild zu zeichnen, weiter haelt. Dagegen
/// staende ein Zeitgeber, der bei jedem Bildschirmwechsel des Fensters neu
/// aufzuhaengen waere, und die Frage, was er tut, wenn das Fenster auf keinem
/// Bildschirm steht. Die Antwort des Projekts darauf ist der Abbruch mit
/// Meldung ([`super::bildtakt::bildwiederholrate`]); fuer einen gewoehnlichen
/// Lesevorgang kommt sie nicht in Frage, und ein fester Rueckfallwert waere die
/// Sonderregel mit eigenem Rueckfallweg, die die Maxime "supersimpel"
/// ausschliesst.
const EINZUGSTAKT: NSTimeInterval = 1.0 / 60.0;

/// Eine der vier Spalten des Dateifensters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spalte {
    /// Der Name des Eintrags.
    Name,
    /// Die Groesse der Daten.
    Groesse,
    /// Der Zeitpunkt der letzten Aenderung.
    Geaendert,
    /// Ordner, Datei oder Verknuepfung.
    Typ,
}

impl Spalte {
    /// Alle vier Spalten in der Reihenfolge, in der sie im Fenster stehen.
    const ALLE: [Spalte; 4] = [
        Spalte::Name,
        Spalte::Groesse,
        Spalte::Geaendert,
        Spalte::Typ,
    ];

    /// Die Kennung, unter der AppKit die Spalte fuehrt.
    ///
    /// Sie dient zugleich als Kennung der wiederverwendeten Zellenansicht: eine
    /// Ansicht, die aus der Namensspalte zurueckkommt, landet nur wieder in der
    /// Namensspalte und behaelt damit ihre Ausrichtung.
    fn kennung(self) -> &'static NSString {
        match self {
            Spalte::Name => ns_string!("name"),
            Spalte::Groesse => ns_string!("groesse"),
            Spalte::Geaendert => ns_string!("geaendert"),
            Spalte::Typ => ns_string!("typ"),
        }
    }

    /// Die Ueberschrift der Spalte.
    fn titel(self) -> &'static NSString {
        match self {
            Spalte::Name => ns_string!("Name"),
            Spalte::Groesse => ns_string!("Größe"),
            Spalte::Geaendert => ns_string!("Änderungsdatum"),
            Spalte::Typ => ns_string!("Typ"),
        }
    }

    /// Anfangsbreite und Mindestbreite in Punkten.
    fn breiten(self) -> (f64, f64) {
        match self {
            Spalte::Name => (240.0, 100.0),
            Spalte::Groesse => (80.0, 60.0),
            Spalte::Geaendert => (130.0, 100.0),
            Spalte::Typ => (90.0, 60.0),
        }
    }

    /// Wie der Text in der Zelle ausgerichtet wird.
    ///
    /// Groessen stehen rechtsbuendig, damit die Ziffern untereinander liegen
    /// und zwei Zahlen sich der Laenge nach vergleichen lassen.
    fn ausrichtung(self) -> NSTextAlignment {
        match self {
            Spalte::Groesse => NSTextAlignment::Right,
            _ => NSTextAlignment::Left,
        }
    }

    /// Die Spalte zu einer Kennung, falls es sie gibt.
    fn aus_kennung(kennung: &NSString) -> Option<Spalte> {
        Spalte::ALLE
            .into_iter()
            .find(|spalte| spalte.kennung() == kennung)
    }

    /// Ob der Nutzer in dieser Spalte schreiben darf (C4).
    ///
    /// Allein der Name: die drei uebrigen Spalten zeigen, was das Dateisystem
    /// ueber den Eintrag sagt, und keine davon laesst sich durch Hinschreiben
    /// aendern.
    const fn beschreibbar(self) -> bool {
        matches!(self, Spalte::Name)
    }
}

/// Die Stelle der Namensspalte, wie `editColumn:row:withEvent:select:` sie
/// nimmt.
///
/// Abgeleitet aus [`Spalte::ALLE`] und nicht hingeschrieben: die Reihenfolge
/// der Spalten steht dort, und eine 0 im Programmtext waere beim naechsten
/// Umsortieren still falsch.
const NAMENSSPALTE: NSInteger = {
    let mut stelle = 0;
    while stelle < Spalte::ALLE.len() {
        if Spalte::ALLE[stelle].beschreibbar() {
            break;
        }
        stelle += 1;
    }
    stelle as NSInteger
};

/// Was beim Auswaehlen eines Eintrags anhand seines Namens herauskam.
///
/// Siehe [`DateifensterQuelle::eintrag_waehlen`], die eine Stelle, die eine
/// Zeile anhand ihres Namens auswaehlt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Auswahlversuch {
    /// Der Eintrag stand in der Liste; die Auswahl steht auf ihm.
    Gewaehlt,
    /// Es laeuft noch ein Lesevorgang. Die Auswahl springt auf den Eintrag,
    /// sobald er eintrifft.
    Vorgemerkt,
    /// Die Liste ist gelesen und kennt den Namen nicht.
    Unbekannt,
}

/// Was mit einem umbenannten Eintrag zu geschehen hat: alter Name, neuer Name.
///
/// Ein eigener Name fuer den Rueckruf, damit das Feld und sein Setzer dieselbe
/// Schreibweise tragen und keine von beiden zu lesen ist wie ein Bandwurm.
pub type Umbenennungsmelder = Box<dyn Fn(&str, &str)>;

/// Was die Datenquelle haelt.
pub struct QuelleIvars {
    /// Die Tabelle, der die Quelle Aenderungen meldet.
    ///
    /// `NSTableView` haelt Datenquelle und Delegierten nur schwach; die starke
    /// Richtung laeuft deshalb von hier nach dort und nicht umgekehrt.
    tabelle: Retained<NSTableView>,
    /// Die Bildlaufansicht um die Tabelle. Sie traegt die Bildlaufposition.
    sicht: Retained<NSScrollView>,
    /// Die Zeile am Fuss, in der die Meldungen dieses Dateifensters stehen.
    statuszeile: Statuszeile,
    /// Die Leiste am Kopf. Sie kommt nach der Quelle zur Welt, weil ihr
    /// Rueckruf die Quelle braucht; siehe [`Dateifenster::bauen`].
    tableiste: RefCell<Option<Tableiste>>,
    /// Die Tabs dieses Dateifensters mit ihrem Inhalt.
    tabs: RefCell<Tabliste>,
    /// Der Zeitgeber, der die Kanaele der Leser leerraeumt.
    ///
    /// Er haelt die Quelle als Ziel fest, und die Quelle haelt ihn. Der Ring
    /// bricht mit `invalidate`, das jeder Lauf am Ende aufruft.
    einzug: RefCell<Option<Retained<NSTimer>>>,
    /// Was gerufen wird, wenn der Nutzer dieses Dateifenster angefasst hat.
    ///
    /// Der Weg, auf dem ein Mausklick das aktive Dateifenster umsetzt. Er ist
    /// wahlfrei, weil die Quelle vor dem Anwendungsdelegierten zur Welt kommt.
    aktivierung: RefCell<Option<Box<dyn Fn()>>>,
    /// Die getippten Anfangsbuchstaben aus C2.
    ///
    /// Je Dateifenster und nicht je Tab: gesucht wird in der Liste, die gerade
    /// auf dem Schirm steht, und jeder Tabwechsel setzt sie zurueck.
    sprungmarke: RefCell<Sprungmarke>,
    /// Was gerufen wird, wenn dieses Dateifenster einen anderen Ordner zeigt.
    ///
    /// Der Weg, auf dem die Dateisystembeobachtung aus C9 erfaehrt, dass sie
    /// neu aufzusetzen ist: ein `FSEventStream` aendert seine Pfadliste nach
    /// dem Anlegen nicht mehr. Wahlfrei, weil die Quelle vor dem
    /// Anwendungsdelegierten zur Welt kommt.
    ///
    /// **Eine Auffrischung ruft ihn ausdruecklich nicht.** Sie wechselt den
    /// Ordner nicht, und sie laeuft im Rueckruf des Stroms: den Strom von dort
    /// aus freizugeben hiesse, ihn mitten in seinem eigenen Aufruf abzubauen.
    ordnerwechsel: RefCell<Option<Box<dyn Fn()>>>,
    /// Was gerufen wird, wenn der Nutzer einen Eintrag umbenannt hat (C4).
    ///
    /// Zwei Namen, der alte und der neue, beide schon geprueft. Was mit ihnen
    /// geschieht, entscheidet der Anwendungsdelegierte: die Umbenennung selbst
    /// laeuft ueber `krk_core::operation::umbenennen`, und die Auffrischung
    /// muss **beide** Dateifenster erreichen, was von hier aus nicht geht.
    /// Wahlfrei, weil die Quelle vor dem Anwendungsdelegierten zur Welt kommt,
    /// wie die beiden Rueckrufe darueber.
    umbenennung: RefCell<Option<Umbenennungsmelder>>,
    /// Was KRK auf den letzten Tastenbefehl des Nutzers zu sagen hat.
    ///
    /// Der oberste der vier Raenge und der einzige, der ueber der
    /// Vorgangsanzeige steht: "es laeuft bereits eine Operation", "es ist
    /// nichts ausgewaehlt", "die Zwischenablage ist leer", der Abschlusstext
    /// eines Vorgangs. Der Nutzer hat eben eine Taste gedrueckt und sieht auf
    /// die Zeile; eine Antwort, die hinter dem Fortschritt verschwindet, ist
    /// keine.
    ///
    /// **Ihre Loeschregel ist der naechste Tastenbefehl** und sonst nichts:
    /// `Anwendungsdelegierter::kommando_ausfuehren` raeumt sie an beiden
    /// Dateifenstern weg, bevor es den naechsten Befehl ausfuehrt. Damit haengt
    /// sie an einem Ereignis und an keinem Zeitgeber, wie die drei uebrigen
    /// Quellen auch.
    befehlsantwort: RefCell<Option<String>>,
    /// Eine Meldung, die dem Fenster gehoert und keinem einzelnen Tab.
    ///
    /// Die beschaedigte Belegungsdatei beim Start und der ausgeworfene
    /// Datentraeger aus C9: ein Ereignis, das der Nutzer nicht angefordert hat.
    /// Sie hat Vorrang vor der Tabmeldung und faellt beim naechsten Ordner-
    /// oder Tabwechsel; siehe [`DateifensterQuelle::meldung_anzeigen`].
    ///
    /// **Eine Antwort auf einen Tastenbefehl gehoert nicht hierher**, sondern
    /// in `befehlsantwort`. Bis zum 260804-1915 lagen beide in diesem Feld, und
    /// daran sind die zwei Defekte dieses Tages haengengeblieben: die
    /// Auswurfmeldung wurde vom Abschlusstext ueberschrieben, weil sie
    /// dasselbe Feld teilten, und die Meldung auf den zweiten
    /// Operationsbefehl erbte den Rang eines Ereignisses, obwohl sie eine
    /// Antwort ist.
    fenstermeldung: RefCell<Option<String>>,
    /// Der Stand einer Dateioperation, die dieses Dateifenster begonnen hat
    /// (C4).
    ///
    /// **Ein eigenes Feld und keines, das es sich mit der Fenstermeldung
    /// teilt.** Die Lebensdauern sind die entgegengesetzten: eine
    /// Fenstermeldung soll beim Ordnerwechsel verschwinden, eine laufende
    /// Anzeige muss ihn ueberleben, weil die Operation weiterlaeuft und der
    /// Nutzer seit dem 260804-1832 waehrenddessen navigieren darf. Ein Feld mit
    /// zwei Loeschregeln waere der Sonderfall, den die Maxime "supersimpel"
    /// ausschliesst.
    vorgangsanzeige: RefCell<Option<String>>,
    /// Der Formatierer fuer Byte-Zahlen.
    ///
    /// Foundation bringt ihn mit, und er zaehlt in derselben Weise wie der
    /// Finder: dezimale Vorsaetze, Trennzeichen nach der Spracheinstellung des
    /// Nutzers. Eine eigene Rechnung waere eine zweite Wahrheit neben der des
    /// Systems.
    ///
    /// **Er wohnt seit S16c hier und nicht mehr beim Delegierten**, weil er
    /// zwei Aufrufer hat: die Groessenspalte, die der Delegierte beschriftet,
    /// und den Markierungsstand in der Statuszeile, den diese Quelle rechnet.
    /// Ein zweiter Formatierer daneben waere eine zweite Schreibweise fuer
    /// dieselbe Zahl, sobald einer der beiden anders eingestellt wuerde. Der
    /// Delegierte kommt ueber [`DateifensterDelegierter::quelle`] an ihn heran;
    /// die starke Richtung geht ohnehin von ihm zur Quelle.
    groessenformat: Retained<NSByteCountFormatter>,
}

define_class!(
    /// Die Datenquelle eines Dateifensters.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = QuelleIvars]
    pub struct DateifensterQuelle;

    impl DateifensterQuelle {
        /// Der Rueckruf des Zeitgebers.
        // SAFETY: Die Signatur passt zu der, die NSTimer aufruft.
        #[unsafe(method(stapelEinziehen:))]
        fn stapel_einziehen(&self, _zeitgeber: &NSTimer) {
            self.einziehen();
        }
    }

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for DateifensterQuelle {}

    // SAFETY: `NSTableViewDataSource` stellt keine Bedingungen.
    unsafe impl NSTableViewDataSource for DateifensterQuelle {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(numberOfRowsInTableView:))]
        fn zeilenzahl(&self, _tabelle: &NSTableView) -> NSInteger {
            self.zeilen() as NSInteger
        }
    }
);

impl DateifensterQuelle {
    /// Eine Datenquelle fuer die genannte Tabelle.
    fn neu(
        mtm: MainThreadMarker,
        tabelle: Retained<NSTableView>,
        sicht: Retained<NSScrollView>,
        statuszeile: Statuszeile,
        tabs: Tabliste,
    ) -> Retained<Self> {
        let groessenformat = NSByteCountFormatter::new();
        groessenformat.setCountStyle(NSByteCountFormatterCountStyle::File);
        let this = Self::alloc(mtm).set_ivars(QuelleIvars {
            tabelle,
            sicht,
            statuszeile,
            tableiste: RefCell::new(None),
            tabs: RefCell::new(tabs),
            einzug: RefCell::new(None),
            aktivierung: RefCell::new(None),
            sprungmarke: RefCell::new(Sprungmarke::neu()),
            ordnerwechsel: RefCell::new(None),
            umbenennung: RefCell::new(None),
            befehlsantwort: RefCell::new(None),
            fenstermeldung: RefCell::new(None),
            vorgangsanzeige: RefCell::new(None),
            groessenformat,
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Haengt die Tableiste ein, sobald sie gebaut ist.
    fn tableiste_setzen(&self, leiste: Tableiste) {
        *self.ivars().tableiste.borrow_mut() = Some(leiste);
        self.tableiste_nachziehen();
    }

    /// Hinterlegt, was beim Anfassen dieses Dateifensters zu tun ist.
    pub fn aktivierung_setzen(&self, melden: Box<dyn Fn()>) {
        *self.ivars().aktivierung.borrow_mut() = Some(melden);
    }

    /// Hinterlegt, was nach einem Ordnerwechsel dieses Dateifensters zu tun ist.
    pub fn ordnerwechsel_setzen(&self, melden: Box<dyn Fn()>) {
        *self.ivars().ordnerwechsel.borrow_mut() = Some(melden);
    }

    /// Hinterlegt, was mit einem umbenannten Eintrag zu geschehen hat (C4).
    pub fn umbenennung_setzen(&self, melden: Umbenennungsmelder) {
        *self.ivars().umbenennung.borrow_mut() = Some(melden);
    }

    /// Der Ordner, den der sichtbare Tab gerade zeigt.
    pub fn angezeigter_ordner(&self) -> PathBuf {
        self.ivars().tabs.borrow().aktiver().ordner().to_path_buf()
    }

    /// Die Ordner aller Tabs, in der Reihenfolge der Leiste (C9).
    pub fn tabordner(&self) -> Vec<PathBuf> {
        self.ivars().tabs.borrow().tabordner()
    }

    /// Die Stelle des sichtbaren Tabs in [`Self::tabordner`].
    pub fn sichtbarer_tab(&self) -> usize {
        self.ivars().tabs.borrow().aktive_stelle()
    }

    /// Laesst den Tab an der genannten Stelle einen anderen Ordner zeigen (C9).
    ///
    /// Der Weg des Auswurfs aus [`crate::auffrischung::datentraeger_verloren`],
    /// und die eine Stelle, an der ein **verdeckter** Tab seinen Ordner
    /// wechselt. Der sichtbare geht denselben Weg wie jede Navigation; der
    /// verdeckte bekommt allein seinen neuen Ordner, ohne Lesevorgang und ohne
    /// Ansichtsarbeit, weil auf keinem Schirm etwas steht, das nachzuziehen
    /// waere. Gelesen wird er, sobald der Nutzer auf ihn wechselt.
    ///
    /// Die Tableiste zieht trotzdem nach: sie zeigt den Namen des Ordners je
    /// Tab, und der ist ein anderer geworden.
    pub fn tab_ordner_setzen(&self, stelle: usize, pfad: &Path) {
        if stelle == self.ivars().tabs.borrow().aktive_stelle() {
            self.ordner_lesen(pfad, None);
            return;
        }
        self.ivars()
            .tabs
            .borrow_mut()
            .verdeckten_tab_setzen(stelle, pfad);
        self.tableiste_nachziehen();
    }

    /// Meldet, dass dieses Dateifenster jetzt einen anderen Ordner zeigt.
    ///
    /// Steht ausdruecklich am Ende der Aufrufer und nicht mittendrin: der
    /// Empfaenger fragt die Ordner beider Dateifenster ab, und eine noch
    /// gehaltene Ausleihe des Tabmodells waere der doppelte Zugriff.
    fn ordnerwechsel_melden(&self) {
        let melden = self.ivars().ordnerwechsel.borrow();
        if let Some(melden) = melden.as_ref() {
            melden();
        }
    }

    // ------------------------------------------------------------------
    // Tabs
    // ------------------------------------------------------------------

    /// Der gespeicherte Zustand dieses Dateifensters fuer `session.toml`.
    ///
    /// Liest vorher die Bildlaufposition aus der Ansicht nach: sie steht in der
    /// `NSScrollView` und nirgends sonst.
    pub fn zustand(&self) -> Fensterzustand {
        self.bildlauf_merken();
        self.ivars().tabs.borrow().zustand()
    }

    /// Was die Lesereihenfolge von diesem Dateifenster wissen muss.
    pub fn uebersicht(&self) -> crate::tabs::Tabuebersicht {
        self.ivars().tabs.borrow().uebersicht()
    }

    /// Startet den Lesevorgang des sichtbaren Tabs.
    ///
    /// Die erste Stufe der Lesereihenfolge aus dem Modulkopf von
    /// [`crate::tabs`]. Die zweite loest der Einzugstakt aus, sobald der
    /// sichtbare Tab bedienbar ist.
    pub fn sichtbaren_lesen(&self) {
        self.ivars().tabs.borrow_mut().sichtbaren_lesen();
        self.einzug_starten();
        self.tableiste_nachziehen();
        self.ordnerwechsel_melden();
    }

    /// Oeffnet einen neuen Tab auf dem Ordner des sichtbaren (C1).
    pub fn tab_neu(&self) {
        let ordner = self.ivars().tabs.borrow().aktiver().ordner().to_path_buf();
        self.bildlauf_merken();
        self.ivars().tabs.borrow_mut().oeffnen(ordner);
        self.tab_gewechselt();
    }

    /// Schliesst den sichtbaren Tab (C1).
    pub fn tab_schliessen(&self) {
        self.bildlauf_merken();
        let veraendert = self.ivars().tabs.borrow_mut().schliessen();
        if veraendert {
            self.tab_gewechselt();
        }
    }

    /// Wechselt zum naechsten Tab (C1).
    pub fn tab_naechster(&self) {
        self.bildlauf_merken();
        if self.ivars().tabs.borrow_mut().naechster() {
            self.tab_gewechselt();
        }
    }

    /// Wechselt zum vorigen Tab (C1).
    pub fn tab_voriger(&self) {
        self.bildlauf_merken();
        if self.ivars().tabs.borrow_mut().voriger() {
            self.tab_gewechselt();
        }
    }

    /// Wechselt auf den Tab an der genannten Stelle (Klick in der Tableiste).
    pub fn tab_waehlen(&self, stelle: usize) {
        self.bildlauf_merken();
        if self.ivars().tabs.borrow_mut().waehlen(stelle) {
            self.tab_gewechselt();
        }
    }

    /// Liest den genannten Ordner in den sichtbaren Tab.
    ///
    /// Kehrt sofort zurueck. Der Inhalt trifft gestueckelt ein; die erste
    /// Bildschirmseite steht mit dem ersten Stapel.
    ///
    /// `auswahl` ist der Name des Eintrags, auf den die Auswahl springt, sobald
    /// gelesen ist: beim Aufstieg der verlassene Ordner (C2), beim Sprung aus
    /// der Zwischenablage die genannte Datei (C10).
    pub fn ordner_lesen(&self, pfad: &Path, auswahl: Option<String>) {
        self.fenstermeldung_loeschen();
        self.ivars().tabs.borrow_mut().ordner_setzen(pfad, auswahl);
        self.nach_lesebeginn();
        self.ordnerwechsel_melden();
    }

    /// Liest den angezeigten Ordner noch einmal (C9).
    ///
    /// **Die eine Stelle, an der eine Auffrischung die Ansicht erreicht.** Der
    /// Weg dorthin ist [`crate::auffrischung::ordner_neu_lesen`], und den ruft
    /// heute der FSEvents-Rueckruf und ab S16 zusaetzlich der gemeldete
    /// Abschluss einer Dateioperation.
    ///
    /// Der Ordner bleibt derselbe, also meldet diese Methode keinen
    /// Ordnerwechsel: der `FSEventStream` beobachtet weiter dieselben Pfade,
    /// und ihn aus seinem eigenen Rueckruf heraus neu aufzusetzen hiesse, ihn
    /// mitten im Aufruf freizugeben.
    ///
    /// Gelesen wird ueber denselben gestueckelten Lesevorgang wie jede
    /// Navigation, samt Generationszaehler. Deshalb blockiert auch die
    /// Auffrischung eines Ordners mit 100.000 Eintraegen die Eingabe nicht.
    pub fn neu_lesen(&self) {
        // Zuerst die Bildlaufposition aus der Ansicht in den Tab holen: sie
        // steht in der `NSScrollView` und nirgends sonst, und der naechste
        // Schritt baut den Tab neu auf.
        self.bildlauf_merken();
        self.ivars().tabs.borrow_mut().aktiven_neu_lesen();
        self.nach_lesebeginn();
    }

    /// Zieht die Ansicht nach, nachdem im sichtbaren Tab ein Lesevorgang
    /// begonnen hat.
    ///
    /// Gemeinsam fuer die Navigation und die Auffrischung; ein zweiter
    /// Ansichtsweg neben diesem entsteht nicht.
    fn nach_lesebeginn(&self) {
        // Der Puffer der Sprungmarke gehoert der Liste, die er durchsucht hat.
        self.ivars().sprungmarke.borrow_mut().zuruecksetzen();
        self.ivars().tabelle.reloadData();
        // Der neue Tab hat noch keine Auswahl; die Tabelle erfaehrt es hier.
        // Sich darauf zu verlassen, dass `reloadData` eine Zeilennummer
        // jenseits der neuen Zeilenzahl von selbst fallen laesst, hiesse, eine
        // Zusage von AppKit anzunehmen statt sie zu geben. Die alte Auswahl
        // kommt mit dem Abschluss des Lesevorgangs zurueck, ueber
        // `wunschauswahl` und dieselbe Huelle, die sie nach einem Umsortieren
        // wiederherstellt.
        self.auswahl_anzeigen();
        self.meldung_anzeigen();
        self.einzug_starten();
        self.tableiste_nachziehen();
    }

    /// Nach einem Tabwechsel: Inhalt, Auswahl, Bildlauf und Leiste nachziehen.
    fn tab_gewechselt(&self) {
        self.fenstermeldung_loeschen();
        self.ivars().sprungmarke.borrow_mut().zuruecksetzen();
        self.ivars().tabelle.reloadData();
        self.auswahl_anzeigen();
        let bildlauf = self.ivars().tabs.borrow().aktiver().bildlauf();
        self.bildlauf_herstellen(bildlauf);
        self.meldung_anzeigen();
        self.tableiste_nachziehen();
        // Der Wechsel kann einen ungelesenen Tab getroffen haben; dann laeuft
        // seit `waehlen` ein Lesevorgang, den der Takt einziehen muss.
        if self.ivars().tabs.borrow().liest_noch() {
            self.einzug_starten();
        }
        // Ein anderer Tab heisst ein anderer Ordner auf dem Schirm, und die
        // Dateisystembeobachtung aus C9 haengt daran.
        self.ordnerwechsel_melden();
    }

    /// Schreibt Beschriftungen und sichtbare Stelle in die Tableiste.
    fn tableiste_nachziehen(&self) {
        let (titel, aktiv) = {
            let tabs = self.ivars().tabs.borrow();
            (tabs.titel(), tabs.aktive_stelle())
        };
        let leiste = self.ivars().tableiste.borrow();
        if let Some(leiste) = leiste.as_ref() {
            leiste.setzen(&titel, aktiv);
        }
    }

    // ------------------------------------------------------------------
    // Was der Messmodus abliest
    // ------------------------------------------------------------------

    /// Wie viele Zeilen der sichtbare Tab gerade traegt.
    ///
    /// Nur zum Ablesen. Die Zahl ist dieselbe, die die Datenquelle AppKit
    /// meldet, und sie ist zugleich die Antwort auf die Frage, ob die erste
    /// Bildschirmseite steht: der Einzugstakt haengt den Stapel an und meldet
    /// der Tabelle im selben Zug die neue Zeilenzahl.
    pub fn zeilen(&self) -> usize {
        self.ivars().tabs.borrow().aktiver().modell().zeilenzahl()
    }

    /// Ob im sichtbaren Tab gerade ein Lesevorgang laeuft.
    ///
    /// Nur zum Ablesen. `false` heisst: gelesen **und** sortiert, denn der
    /// Einzugstakt gibt den Vorgang erst nach `abschliessen` frei.
    pub fn liest_noch(&self) -> bool {
        self.ivars().tabs.borrow().aktiver().liest()
    }

    /// Welche Zeile ausgewaehlt ist; -1, wenn keine.
    ///
    /// Nur zum Ablesen, und ausdruecklich von der `NSTableView` und nicht vom
    /// Modell: die Messung von L1 fragt, welche Zeile der Nutzer *sieht*, und
    /// das ist die der Tabelle.
    pub fn auswahlzeile(&self) -> isize {
        self.ivars().tabelle.selectedRow()
    }

    /// Bricht jeden laufenden Lesevorgang ab und laesst stehen, was da ist.
    pub fn lesen_abbrechen(&self) {
        self.einzug_beenden();
        self.ivars().tabs.borrow_mut().abbrechen();
        let ausgewaehlt = self.ivars().tabs.borrow().aktiver().modell().auswahl();
        self.ivars().tabelle.reloadData();
        // Auch der Abbruch sortiert; auch hier zeigt die alte Zeilennummer
        // danach auf einen anderen Eintrag.
        self.auswahl_setzen(ausgewaehlt);
    }

    // ------------------------------------------------------------------
    // Kommandos
    // ------------------------------------------------------------------

    /// Fuehrt ein Kommando aus, das der Ereignisabgriff nachgeschlagen hat.
    ///
    /// Nur die Kommandos, die dieses eine Dateifenster betreffen. Was das
    /// Fenster als ganzes angeht, das Wechseln des aktiven Dateifensters und
    /// das Ein- und Ausblenden der Bereiche, faengt der Anwendungsdelegierte
    /// vorher ab; es kommt hier nicht an.
    pub fn kommando_ausfuehren(&self, kommando: Kommando) -> bool {
        match kommando {
            Kommando::AuswahlHoch => self.auswahl_bewegen(Bewegung::Um(-1)),
            Kommando::AuswahlRunter => self.auswahl_bewegen(Bewegung::Um(1)),
            Kommando::SeiteHoch => self.auswahl_bewegen(Bewegung::Um(-self.seitenhoehe())),
            Kommando::SeiteRunter => self.auswahl_bewegen(Bewegung::Um(self.seitenhoehe())),
            Kommando::Listenanfang => self.auswahl_bewegen(Bewegung::Anfang),
            Kommando::Listenende => self.auswahl_bewegen(Bewegung::Ende),
            Kommando::Oeffnen => self.auswahl_oeffnen(),
            Kommando::OrdnerAufwaerts => self.ordner_aufwaerts(),
            Kommando::Pfadeingabe => self.pfadeingabe_zeigen(),
            Kommando::ZwischenablageSpringen => self.zwischenablage_springen(),
            Kommando::MarkierungUmschalten => self.markieren_und_weiter(),
            Kommando::AlleMarkieren => self.markierung_aendern(Ordnermodell::alle_markieren),
            Kommando::MarkierungAufheben => {
                self.markierung_aendern(Ordnermodell::markierung_aufheben)
            }
            Kommando::MarkierungUmkehren => {
                self.markierung_aendern(Ordnermodell::markierung_umkehren)
            }
            Kommando::SortierungName => self.nach_schluessel_sortieren(Schluessel::Name),
            Kommando::SortierungGroesse => self.nach_schluessel_sortieren(Schluessel::Groesse),
            Kommando::SortierungDatum => self.nach_schluessel_sortieren(Schluessel::Geaendert),
            Kommando::SortierungTyp => self.nach_schluessel_sortieren(Schluessel::Typ),
            Kommando::SortierrichtungUmkehren => self.sortierrichtung_umkehren(),
            Kommando::VersteckteUmschalten => self.verstecke_umschalten(),
            Kommando::TabNeu => self.tab_neu(),
            Kommando::TabSchliessen => self.tab_schliessen(),
            Kommando::TabNaechster => self.tab_naechster(),
            Kommando::TabVoriger => self.tab_voriger(),
            Kommando::Umbenennen => return self.umbenennung_beginnen(),
            // Nicht Sache eines einzelnen Dateifensters.
            _ => return false,
        }
        true
    }

    /// Worauf ein Dateioperations-Befehl in diesem Dateifenster wirkt (C4).
    ///
    /// Die Regel dahinter, dass die Markierung Vorrang vor der Auswahl hat,
    /// steht in [`crate::kommandos::operationen::betroffene`] und ist ohne
    /// Fenster pruefbar; hier bleibt allein die Ausleihe des Tabmodells.
    pub fn betroffene_eintraege(&self) -> operationen::Auswahl {
        let tabs = self.ivars().tabs.borrow();
        let tab = tabs.aktiver();
        operationen::betroffene(tab.modell(), tab.ordner())
    }

    /// Ein getipptes Zeichen fuer die Sprungmarke aus C2.
    ///
    /// Liefert, ob KRK es verbraucht hat. Ein Zeichen, das kein Dateiname
    /// tragen kann, weist der Kern ab; der Tastendruck geht dann unveraendert
    /// weiter, statt ins Leere geschluckt zu werden. Findet sich kein Eintrag,
    /// bleibt die Auswahl stehen und das Zeichen gilt trotzdem als verbraucht:
    /// der Puffer traegt es, und der naechste Buchstabe baut darauf auf.
    pub fn sprungmarke_tippen(&self, zeichen: char) -> bool {
        let zeile = {
            let mut marke = self.ivars().sprungmarke.borrow_mut();
            let Some(praefix) = marke.tippen(zeichen, Instant::now()) else {
                return false;
            };
            let tabs = self.ivars().tabs.borrow();
            sprungmarke::erste_zeile_mit(tabs.aktiver().modell(), praefix)
        };
        if let Some(zeile) = zeile {
            self.zeile_setzen(zeile);
        }
        true
    }

    /// Wie viele Zeilen eine Bildschirmseite fasst.
    ///
    /// Gefragt wird die Tabelle und nicht gerechnet: die Zahl der sichtbaren
    /// Zeilen haengt an der Fenstergroesse, und die aendert der Nutzer. Das
    /// Mindestmass von einer Zeile faengt den Fall ab, dass die Tabelle noch
    /// keine Groesse hat; eine Seitentaste, die um null Zeilen springt, waere
    /// eine tote Taste.
    fn seitenhoehe(&self) -> isize {
        let tabelle = &self.ivars().tabelle;
        let sichtbare = tabelle.rowsInRect(tabelle.visibleRect()).length as isize;
        sichtbare.max(1)
    }

    /// Bewegt die Auswahl (C2).
    ///
    /// Die Rechnung dahinter steht in [`crate::kommandos::navigation`]; hier
    /// bleibt allein, was AppKit betrifft: die Zeilennummer der Tabelle
    /// abfragen und die neue setzen.
    fn auswahl_bewegen(&self, bewegung: Bewegung) {
        // `selectedRow` liefert -1, solange nichts ausgewaehlt ist.
        let jetzt = self.ivars().tabelle.selectedRow();
        if let Some(ziel) = zielzeile(bewegung, jetzt, self.zeilen()) {
            self.zeile_setzen(ziel);
        }
    }

    /// Setzt die Auswahl auf diese Zeile und blaettert sie ins Bild.
    ///
    /// Der eine Weg, auf dem die Tastatur die Auswahl umsetzt: die Bewegungen
    /// aus C2, die Sprungmarke und das Markieren mit Weiterruecken enden alle
    /// hier.
    fn zeile_setzen(&self, zeile: usize) {
        let tabelle = &self.ivars().tabelle;
        let auswahl = NSIndexSet::indexSetWithIndex(zeile);
        tabelle.selectRowIndexes_byExtendingSelection(&auswahl, false);
        tabelle.scrollRowToVisible(zeile as NSInteger);
        // Ausdruecklich und nicht ueber den Delegiertenrueckruf: ob AppKit die
        // Auswahlmeldung auch bei einer selbst gesetzten Auswahl schickt, ist
        // eine Zusage, die dieser Weg nicht braucht.
        self.auswahl_merken();
    }

    /// Haelt fest, welcher Eintrag der ausgewaehlten Zeile entspricht.
    ///
    /// Der Weg von der Zeile zum Eintrag laeuft genau hier und sonst nirgends.
    /// Gerufen wird er von jeder Stelle, an der sich die Auswahl der Tabelle
    /// aendert: von [`DateifensterQuelle::auswahl_verschieben`] und vom
    /// Auswahlrueckruf des Delegierten, den die Maus ausloest.
    fn auswahl_merken(&self) {
        let zeile = usize::try_from(self.ivars().tabelle.selectedRow()).ok();
        let mut tabs = self.ivars().tabs.borrow_mut();
        let modell = tabs.aktiver_mut().modell_mut();
        let eintrag = zeile.and_then(|zeile| modell.eintragsindex(zeile));
        modell.auswahl_setzen(eintrag);
    }

    /// Zeigt in der Tabelle die Auswahl, die im Modell steht.
    ///
    /// Die Gegenrichtung zu [`DateifensterQuelle::auswahl_merken`] und die
    /// Stelle, an der die Auswahl ein Umsortieren uebersteht: der Eintrag
    /// bleibt derselbe, seine Zeile ist eine andere.
    fn auswahl_anzeigen(&self) {
        let zeile = self
            .ivars()
            .tabs
            .borrow()
            .aktiver()
            .modell()
            .auswahl_zeile();
        self.zeile_auswaehlen(zeile);
    }

    /// Setzt die Auswahl auf den genannten Eintrag und zieht die Tabelle nach.
    ///
    /// `None` hebt die Auswahl auf.
    fn auswahl_setzen(&self, eintrag: Option<u32>) {
        let zeile = {
            let mut tabs = self.ivars().tabs.borrow_mut();
            let modell = tabs.aktiver_mut().modell_mut();
            modell.auswahl_setzen(eintrag);
            modell.auswahl_zeile()
        };
        self.zeile_auswaehlen(zeile);
    }

    /// Waehlt die genannte Zeile in der Tabelle aus.
    fn zeile_auswaehlen(&self, zeile: Option<usize>) {
        let tabelle = &self.ivars().tabelle;
        // Eine leere Indexmenge hebt die Auswahl auf. Der Fall tritt ein, wenn
        // nichts ausgewaehlt war und wenn der ausgewaehlte Eintrag gerade
        // ausgeblendet ist.
        let auswahl = match zeile {
            Some(zeile) => NSIndexSet::indexSetWithIndex(zeile),
            None => NSIndexSet::new(),
        };
        tabelle.selectRowIndexes_byExtendingSelection(&auswahl, false);
        if let Some(zeile) = zeile {
            tabelle.scrollRowToVisible(zeile as NSInteger);
        }
    }

    /// Steigt in den ausgewaehlten Ordner hinein.
    ///
    /// Eine Datei oeffnet nichts: das Ansehen und das Bearbeiten sind eigene
    /// Funktionen und kommen mit dem Editor, nicht mit diesem Schritt. Einer
    /// symbolischen Verknuepfung folgt KRK hier ebenfalls nicht, weil der Leser
    /// sie als Verknuepfung meldet und nicht als das, worauf sie zeigt.
    fn auswahl_oeffnen(&self) {
        let Ok(zeile) = usize::try_from(self.ivars().tabelle.selectedRow()) else {
            return;
        };
        let ziel: Option<PathBuf> = {
            let tabs = self.ivars().tabs.borrow();
            let tab = tabs.aktiver();
            tab.modell()
                .zeile(zeile)
                .filter(|eintrag| eintrag.ist_ordner())
                .map(|eintrag| tab.ordner().join(&eintrag.name))
        };
        if let Some(ziel) = ziel {
            self.ordner_lesen(&ziel, None);
        }
    }

    /// Steigt in den uebergeordneten Ordner auf (C2).
    ///
    /// Die Auswahl steht danach auf dem Ordner, aus dem der Nutzer kam. Die
    /// Rechnung dafuer ist reine Pfadarithmetik und steht im Kern.
    fn ordner_aufwaerts(&self) {
        let hier = self.ivars().tabs.borrow().aktiver().ordner().to_path_buf();
        if let Some((eltern, verlassen)) = aufwaerts(&hier) {
            self.ordner_lesen(&eltern, Some(verlassen));
        }
    }

    // ------------------------------------------------------------------
    // Der eine Navigationsweg: Pfadeingabe und Zwischenablage (C2, C10)
    // ------------------------------------------------------------------

    /// Zeigt die Pfadeingabe als Blatt am Fenster (C2).
    ///
    /// Der erste der beiden Ausloeser des einen Navigationswegs. Steht das
    /// Fenster nicht (etwa waehrend des Aufbaus), geschieht nichts: ein Blatt
    /// ohne Fenster gibt es nicht.
    fn pfadeingabe_zeigen(&self) {
        let Some(fenster) = self.ivars().tabelle.window() else {
            return;
        };
        let hier = self.ivars().tabs.borrow().aktiver().ordner().to_path_buf();
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        blaetter::pfadeingabe::zeigen(
            self.mtm(),
            &fenster,
            &hier.to_string_lossy(),
            move |eingabe| {
                if let Some(selbst) = schwach.load() {
                    selbst.pfad_anspringen(Path::new(eingabe.trim()));
                }
            },
        );
    }

    /// Springt zu dem, was in der Zwischenablage steht (C10).
    ///
    /// Der zweite Ausloeser desselben Navigationswegs. Der Unterschied zur
    /// Pfadeingabe von Hand ist allein, woher der Wert kommt; die Pruefung und
    /// die Navigation dahinter sind dieselben.
    fn zwischenablage_springen(&self) {
        let Some(inhalt) = super::zwischenablage::lesen() else {
            self.befehlsantwort_zeigen("die Zwischenablage ist leer");
            return;
        };
        match zwischenablage::deuten(&inhalt) {
            Ziel::Pfad(pfad) => self.pfad_anspringen(&pfad),
            Ziel::Web(adresse) => {
                if !super::zwischenablage::im_browser_oeffnen(&adresse) {
                    self.befehlsantwort_zeigen(&format!(
                        "{adresse} liess sich nicht an den Systembrowser uebergeben"
                    ));
                }
            }
            Ziel::Nichts => self.befehlsantwort_zeigen(
                "die Zwischenablage traegt weder einen absoluten Pfad noch eine Web-Adresse",
            ),
        }
    }

    /// Prueft einen Pfad und geht dorthin.
    ///
    /// **Die eine Stelle, die ein geprueftes Ergebnis anwendet.** Beide
    /// Ausloeser oben enden hier, und ein zweiter Navigationsweg daneben
    /// entsteht nicht. Was geprueft wird, steht in
    /// [`crate::kommandos::pfadeingabe`] und ist ohne Fenster pruefbar.
    fn pfad_anspringen(&self, pfad: &Path) {
        let angezeigt = self.ivars().tabs.borrow().aktiver().ordner().to_path_buf();
        match pfadeingabe::pruefen(pfad, &angezeigt) {
            Ergebnis::Wechseln { ordner, auswahl } => self.ordner_lesen(&ordner, auswahl),
            Ergebnis::NurAuswahl { name } => self.eintrag_anspringen(&name),
            Ergebnis::Meldung(text) => self.befehlsantwort_zeigen(&text),
        }
    }

    /// Setzt die Auswahl auf den Eintrag dieses Namens im angezeigten Ordner.
    ///
    /// Der Fall aus C10, in dem die genannte Datei bereits vor dem Nutzer
    /// liegt: KRK wechselt den Ordner nicht, sondern blaettert den Eintrag ins
    /// Bild. Steht der Name nicht in der fertig gelesenen Liste, meldet die
    /// Statuszeile das, statt wortlos nichts zu tun.
    fn eintrag_anspringen(&self, name: &str) {
        if self.eintrag_waehlen(name) == Auswahlversuch::Unbekannt {
            self.befehlsantwort_zeigen(&format!("{name} steht nicht in der Liste"));
        }
    }

    /// Waehlt den Eintrag dieses Namens: jetzt, oder sobald er gelesen ist.
    ///
    /// **Die eine Stelle, die eine Zeile anhand ihres Namens auswaehlt.** Zwei
    /// Aufrufer mit derselben Frage und verschiedener Lage: der Sprung aus der
    /// Zwischenablage (C10, S13) trifft eine Liste, die schon steht, und das
    /// Anlegen aus C4 eine, deren Lesevorgang gerade erst begonnen hat. Der
    /// zweite Fall geht ueber die `wunschauswahl` des Tabs, dieselbe, die auch
    /// die Sitzungswiederherstellung und der Aufstieg aus C2 benutzen; ein
    /// zweiter Auswahlweg entsteht nicht.
    ///
    /// Der Rueckgabewert sagt, welcher der drei Faelle eingetreten ist. Allein
    /// [`Auswahlversuch::Unbekannt`] ist eine Auskunft an den Nutzer wert: der
    /// Name steht in einer fertig gelesenen Liste nicht.
    pub fn eintrag_waehlen(&self, name: &str) -> Auswahlversuch {
        let zeile = {
            let tabs = self.ivars().tabs.borrow();
            let modell = tabs.aktiver().modell();
            modell
                .index_von_namen(name)
                .and_then(|index| modell.zeile_von(index))
        };
        if let Some(zeile) = zeile {
            self.zeile_setzen(zeile);
            return Auswahlversuch::Gewaehlt;
        }
        // Ein laufender Lesevorgang kennt den Eintrag noch nicht. Der Name
        // ueberlebt ihn, eine Zeilennummer nicht: sortiert wird erst mit dem
        // Abschluss.
        let liest = self.ivars().tabs.borrow().aktiver().liest();
        if liest {
            self.ivars().tabs.borrow_mut().wunschauswahl_setzen(name);
            return Auswahlversuch::Vorgemerkt;
        }
        Auswahlversuch::Unbekannt
    }

    /// Alle Namen des angezeigten Ordners, auch die ausgeblendeten.
    ///
    /// Der Bestand, gegen den die Kollisionspruefung des Stapel-Umbenennens
    /// vergleicht (C4). Ausdruecklich nicht die Sichtreihenfolge: ein
    /// ausgeblendeter Eintrag belegt seinen Namen genauso wie ein sichtbarer,
    /// und eine Pruefung, die ihn uebersaehe, liesse das Umbenennen erst im
    /// Dateisystem scheitern.
    pub fn alle_namen(&self) -> Vec<String> {
        self.ivars()
            .tabs
            .borrow()
            .aktiver()
            .modell()
            .eintraege()
            .iter()
            .map(|eintrag| eintrag.name.clone())
            .collect()
    }

    // ------------------------------------------------------------------
    // Mehrfachauswahl, Sortierung, versteckte Eintraege (C2)
    // ------------------------------------------------------------------

    /// Markiert den Eintrag unter der Auswahl und rueckt weiter (C2).
    fn markieren_und_weiter(&self) {
        let Ok(zeile) = usize::try_from(self.ivars().tabelle.selectedRow()) else {
            return;
        };
        let weiter = {
            let mut tabs = self.ivars().tabs.borrow_mut();
            markieren_und_weiter(tabs.aktiver_mut().modell_mut(), zeile)
        };
        self.ivars().tabelle.reloadData();
        match weiter {
            Some(weiter) => self.zeile_setzen(weiter),
            // Die letzte Zeile: die Markierung steht, die Auswahl bleibt.
            None => self.auswahl_anzeigen(),
        }
        self.meldung_anzeigen();
    }

    // ------------------------------------------------------------------
    // Umbenennen direkt in der Liste (C4, Schritt 17b)
    // ------------------------------------------------------------------

    /// Schaltet die Namenszelle des ausgewaehlten Eintrags in den
    /// Bearbeitungszustand (C4).
    ///
    /// C4 verlangt das Umbenennen "direkt in der Liste", also kein Blatt,
    /// sondern die Zelle selbst. `editColumn:row:withEvent:select:` macht den
    /// Feldeditor des Fensters zum Ersthelfer und stellt ihn in die Zelle; der
    /// vorhandene Name steht darin und ist ausgewaehlt, sodass ein Tastendruck
    /// ihn ersetzt und ein Pfeil ihn behaelt.
    ///
    /// Liefert `false`, wenn keine Zeile ausgewaehlt ist oder in ihr kein
    /// Eintrag steht; dann ist der Tastendruck nicht verbraucht.
    fn umbenennung_beginnen(&self) -> bool {
        let Ok(zeile) = usize::try_from(self.ivars().tabelle.selectedRow()) else {
            return false;
        };
        // Waehrend eines Lesevorgangs kann die Zeilennummer der Tabelle dem
        // Modell um einen Takt voraus sein.
        if self.mit_zeile(zeile, |_| ()).is_none() {
            return false;
        }
        let Ok(zeile) = NSInteger::try_from(zeile) else {
            return false;
        };
        // Bearbeitet werden kann nur eine sichtbare Zeile. Die Auswahl steht
        // nach jeder Bewegung im Bild; nach einer Wiederherstellung aus der
        // Sitzung nicht zwingend.
        self.ivars().tabelle.scrollRowToVisible(zeile);
        self.ivars()
            .tabelle
            .editColumn_row_withEvent_select(NAMENSSPALTE, zeile, None, true);
        true
    }

    /// Wertet aus, was in der Namenszelle steht, und benennt um (C4).
    ///
    /// Gerufen aus der Aktion des Feldes, also wenn der Nutzer die Eingabe mit
    /// Return abschliesst oder die Zelle verlaesst. **Escape kommt hier nicht
    /// an:** AppKit bricht die Bearbeitung dann ueber `abortEditing` ab, stellt
    /// den alten Text wieder her und schickt keine Aktion. Genau das verlangt
    /// C4, "Return uebernimmt, Escape verwirft", und es kostet keine eigene
    /// Regel.
    ///
    /// Die Zeile kommt von der Tabelle ueber `rowForView:` und nicht aus einem
    /// gemerkten Zustand: die Zellenansicht **ist** das Feld, das die Aktion
    /// schickt, und die Tabelle weiss, in welcher Zeile sie steht. Ein
    /// gemerkter Zustand haette eine zweite Loeschregel gebraucht, fuer den
    /// Fall, dass die Bearbeitung ohne Aktion endet.
    fn umbenennung_beenden(&self, feld: &NSTextField) {
        let zeile = self.ivars().tabelle.rowForView(feld);
        let Ok(zeile) = usize::try_from(zeile) else {
            return;
        };
        let Some(alt) = self.mit_zeile(zeile, |eintrag| eintrag.name.clone()) else {
            return;
        };
        let eingabe = feld.stringValue().to_string();

        match operationen::umbenennung_pruefen(&alt, &eingabe) {
            // Der haeufigste Ausgang: die Zelle war offen und schliesst wieder,
            // ohne dass sich etwas geaendert hat.
            Umbenennungswunsch::Unveraendert => self.zeile_neu_zeichnen(zeile),
            Umbenennungswunsch::Abgelehnt(grund) => {
                // Erst die Zelle zuruecksetzen, dann melden: der Nutzer soll
                // keinen halben Namen stehen sehen, waehrend die Zeile den
                // Grund nennt.
                self.zeile_neu_zeichnen(zeile);
                self.befehlsantwort_zeigen(grund);
            }
            Umbenennungswunsch::Neu(neu) => {
                self.zeile_neu_zeichnen(zeile);
                let melden = self.ivars().umbenennung.borrow();
                if let Some(melden) = melden.as_ref() {
                    melden(&alt, &neu);
                }
            }
        }
    }

    /// Holt sich die Beschriftung einer Zeile aus dem Modell zurueck.
    ///
    /// Der Weg, auf dem eine abgelehnte Eingabe verschwindet: das Feld traegt
    /// noch den getippten Text, das Modell den unveraenderten Namen, und ein
    /// Zeichendurchgang schreibt den Namen wieder hinein. Eine eigene
    /// Zuruecksetzung am Feld waere ein zweiter Weg zu demselben Text.
    fn zeile_neu_zeichnen(&self, zeile: usize) {
        let Ok(zeile) = NSInteger::try_from(zeile) else {
            return;
        };
        let zeilen = NSIndexSet::indexSetWithIndex(zeile as usize);
        let spalten = NSIndexSet::indexSetWithIndex(NAMENSSPALTE as usize);
        self.ivars()
            .tabelle
            .reloadDataForRowIndexes_columnIndexes(&zeilen, &spalten);
    }

    /// Wendet einen der drei uebrigen Markierungsbefehle an (C2).
    fn markierung_aendern(&self, aendern: impl FnOnce(&mut Ordnermodell)) {
        {
            let mut tabs = self.ivars().tabs.borrow_mut();
            aendern(tabs.aktiver_mut().modell_mut());
        }
        self.ivars().tabelle.reloadData();
        self.auswahl_anzeigen();
        self.meldung_anzeigen();
    }

    /// Sortiert nach diesem Schluessel und schaltet bei Wiederholung die
    /// Richtung um (C2).
    fn nach_schluessel_sortieren(&self, schluessel: Schluessel) {
        {
            let mut tabs = self.ivars().tabs.borrow_mut();
            tabs.aktiver_mut()
                .modell_mut()
                .nach_schluessel_sortieren(schluessel);
        }
        self.umsortiert();
    }

    /// Kehrt die Sortierrichtung um, ohne den Schluessel zu wechseln (C2).
    fn sortierrichtung_umkehren(&self) {
        {
            let mut tabs = self.ivars().tabs.borrow_mut();
            let modell = tabs.aktiver_mut().modell_mut();
            let jetzt = modell.sortierung();
            modell.sortierung_setzen(Sortierung::neu(
                jetzt.schluessel,
                jetzt.richtung.umgekehrt(),
            ));
        }
        self.umsortiert();
    }

    /// Blendet versteckte Eintraege ein und wieder aus (C2).
    fn verstecke_umschalten(&self) {
        {
            let mut tabs = self.ivars().tabs.borrow_mut();
            tabs.aktiver_mut().modell_mut().verstecke_umschalten();
        }
        self.umsortiert();
    }

    /// Nach einem Wechsel der Reihenfolge oder der Sichtbarkeit.
    ///
    /// Die Auswahl haengt am Eintrag und nicht an der Zeile; sie wandert
    /// deshalb mit und wird hier nur neu angezeigt. Der Puffer der Sprungmarke
    /// faellt: er hatte die alte Reihenfolge durchsucht.
    fn umsortiert(&self) {
        self.ivars().sprungmarke.borrow_mut().zuruecksetzen();
        self.ivars().tabelle.reloadData();
        self.auswahl_anzeigen();
    }

    /// Reicht den Eintrag der genannten Zeile an eine Auswertung weiter.
    ///
    /// Der Zugriff laeuft ueber einen Rueckruf und nicht ueber eine
    /// herausgegebene Referenz, damit die Ausleihe des Modells hier endet und
    /// kein Aufrufer sie ueber einen AppKit-Aufruf hinweg haelt.
    fn mit_zeile<T>(&self, zeile: usize, auswerten: impl FnOnce(&Eintrag) -> T) -> Option<T> {
        let tabs = self.ivars().tabs.borrow();
        tabs.aktiver().modell().zeile(zeile).map(auswerten)
    }

    /// Ob der Eintrag dieser Zeile markiert ist (C2).
    fn zeile_markiert(&self, zeile: usize) -> bool {
        let tabs = self.ivars().tabs.borrow();
        let modell = tabs.aktiver().modell();
        modell
            .eintragsindex(zeile)
            .is_some_and(|index| modell.ist_markiert(index))
    }

    // ------------------------------------------------------------------
    // Bildlauf, Statuszeile, Einzugstakt
    // ------------------------------------------------------------------

    /// Der Wert, den `bounds().origin.y` der Bildlaufansicht am oberen Rand
    /// traegt.
    ///
    /// **Nicht null.** Die Spaltenueberschriften stehen ueber der Liste, und
    /// AppKit legt dafuer einen Inhaltsrand an, unter den die Liste laeuft; der
    /// Ursprung der Bildlaufansicht liegt deshalb um die Kopfhoehe ueber dem
    /// oberen Rand der Liste. Am 260804 stand in jedem ungescrollten Tab der
    /// `session.toml` deshalb `bildlauf = -28.0`
    /// (`issues/260804-1040_c_die-bildlaufposition-in-der-session-toml-steht-am-oberen-rand-auf-minus-28.md`).
    ///
    /// Gefragt ist die Kopfansicht der Tabelle und **nicht** der Inhaltsrand
    /// der Bildlaufansicht. Beides waere denkbar, gemessen ist nur eines: am
    /// 260805 lieferte eine Sonde im laufenden Buendel `roh=-28`,
    /// `contentInsets.top=0` und die Hoehe der Kopfansicht `28`. AppKit haelt
    /// den Spaltenkopf hier also in der eigenen Kopfansicht der Tabelle und
    /// nicht als Rand, und ein `contentInsets` an dieser Stelle rechnete
    /// dauerhaft mit null.
    ///
    /// Abgefragt und nicht hingeschrieben: die Kopfhoehe haengt an der
    /// Systemschriftgroesse, und eine 28 im Programmtext waere auf dem naechsten
    /// Mac mit anderer Einstellung falsch. Ohne Kopfansicht ist der Ursprung
    /// null, und die Umrechnung faellt von selbst weg.
    fn bildlauf_ursprung(&self) -> f64 {
        match self.ivars().tabelle.headerView() {
            Some(kopf) => -kopf.frame().size.height,
            None => 0.0,
        }
    }

    /// Liest die Bildlaufposition aus der Ansicht in den sichtbaren Tab.
    ///
    /// Gemerkt wird der Abstand vom oberen Rand der Liste und nicht der rohe
    /// Ursprung der Ansicht. Der Nutzer soll `session.toml` lesen und von Hand
    /// aendern koennen, und dort heisst 0 damit "ganz oben".
    fn bildlauf_merken(&self) {
        let hoehe = self.ivars().sicht.contentView().bounds().origin.y - self.bildlauf_ursprung();
        self.ivars()
            .tabs
            .borrow_mut()
            .aktiver_mut()
            .bildlauf_setzen(hoehe);
    }

    /// Stellt die genannte Bildlaufposition in der Ansicht her.
    ///
    /// Der Rueckweg zu [`Self::bildlauf_merken`], mit demselben Ursprung.
    fn bildlauf_herstellen(&self, hoehe: f64) {
        let inhalt = self.ivars().sicht.contentView();
        inhalt.scrollToPoint(NSPoint::new(0.0, hoehe + self.bildlauf_ursprung()));
        self.ivars().sicht.reflectScrolledClipView(&inhalt);
    }

    /// Schreibt in die Statuszeile, was gerade dort stehen soll.
    ///
    /// **Die eine Stelle, die entscheidet, was in der Zeile steht.** Fuenf
    /// Quellen, ein Rang; die Regel und ihre Begruendung stehen bei
    /// [`statuszeile::zeile`], die Lebensdauern bei den Feldern in
    /// [`QuelleIvars`]. Diese Methode liest die vier Felder, rechnet den
    /// fuenften Rang und uebergibt alles; sie entscheidet selbst nichts, damit
    /// die Entscheidung an genau einer Stelle steht und ohne AppKit pruefbar
    /// ist.
    fn meldung_anzeigen(&self) {
        // Vor jeder Ausleihe: die Rechnung leiht das Tabmodell selbst aus und
        // ruft dabei den Groessenformatierer.
        let markierungsstand = self.markierungsstand_text();
        let ivars = self.ivars();
        let befehlsantwort = ivars.befehlsantwort.borrow();
        let vorgangsanzeige = ivars.vorgangsanzeige.borrow();
        let fenstermeldung = ivars.fenstermeldung.borrow();
        let tabs = ivars.tabs.borrow();
        ivars.statuszeile.zeigen(statuszeile::zeile(
            befehlsantwort.as_deref(),
            vorgangsanzeige.as_deref(),
            fenstermeldung.as_deref(),
            tabs.aktiver().meldung(),
            markierungsstand.as_deref(),
        ));
    }

    /// Der fuenfte Rang der Statuszeile: was im sichtbaren Tab markiert ist
    /// (C2).
    ///
    /// **Die einzige Quelle der Zeile ohne eigenes Feld, und das ist der
    /// Entwurf.** Die vier anderen halten je einen Text, den jemand setzt und
    /// eine Regel loescht. Ein Feld haette hier vier Schreiber, die vier
    /// Markierungsbefehle, die Auffrischung, den Tabwechsel und den
    /// Sortierwechsel, und damit vier Gelegenheiten, veraltet zu sein. Die
    /// Rechnung laeuft ueber eine Liste, die ohnehin im Speicher steht, und hat
    /// keine.
    ///
    /// **Gezeichnet werden muss trotzdem.** Die beiden Markierungsmethoden
    /// [`Self::markieren_und_weiter`] und [`Self::markierung_aendern`] rufen
    /// dafuer [`Self::meldung_anzeigen`], so wie sie die Tabelle neu laden.
    /// Das ist etwas anderes als ein Feld mit vier Schreibern: verpasst einer
    /// den Aufruf, steht ein alter Text in der Zeile, bis der naechste
    /// Zeichenanlass kommt, und nirgends ein falscher Zustand. Die beiden
    /// uebrigen Anlaesse zeichnen ohnehin: [`Self::tab_gewechselt`] und
    /// [`Self::nach_lesebeginn`] rufen [`Self::meldung_anzeigen`] seit S12 und
    /// S14, und der Lesebeginn deckt die Auffrischung mit ab, die die
    /// Markierung leert. Das Umsortieren und das Ein- und Ausblenden brauchen
    /// es nicht, weil sie die Markierung nicht anfassen und der Stand ueber
    /// alle gelesenen Eintraege zaehlt, nicht ueber die sichtbaren.
    fn markierungsstand_text(&self) -> Option<String> {
        // Die Ausleihe endet mit dieser Anweisung: `markierungsstand` liefert
        // einen eigenen Wert, und die Zeile darunter ruft Objective-C.
        let stand = self
            .ivars()
            .tabs
            .borrow()
            .aktiver()
            .modell()
            .markierungsstand();
        auswahl::markierungsstand_text(stand, &self.groesse_beschriften(stand.groesse))
    }

    /// Eine Byte-Zahl in der Schreibweise des Systems.
    ///
    /// Zwei Aufrufer, ein Formatierer: die Groessenspalte ueber den
    /// Delegierten und der Markierungsstand oben.
    fn groesse_beschriften(&self, bytes: u64) -> String {
        // `stringFromByteCount:` nimmt eine vorzeichenbehaftete Zahl. Eine
        // Datei jenseits von acht Exabyte gibt es nicht; die Saettigung ist
        // trotzdem ehrlicher als ein Ueberlauf ins Negative.
        let bytes = i64::try_from(bytes).unwrap_or(i64::MAX);
        self.ivars()
            .groessenformat
            .stringFromByteCount(bytes)
            .to_string()
    }

    /// Loescht die Meldung des Fensters, falls eine steht.
    ///
    /// Gerufen von jedem echten Ordnerwechsel und von jedem Tabwechsel, und
    /// ausdruecklich **nicht** von einer Auffrischung: die zeigt denselben
    /// Ordner, und eine Meldung, die eine Sekunde spaeter von einer fremden
    /// Aenderung im Hintergrund weggeraeumt wird, ist keine Meldung. Genau das
    /// geschah am 260804 im laufenden Buendel mit der Auswurfmeldung aus C9,
    /// weil das Benutzerverzeichnis, auf das ein Dateifenster ausweicht,
    /// staendig beschrieben wird.
    fn fenstermeldung_loeschen(&self) {
        *self.ivars().fenstermeldung.borrow_mut() = None;
    }

    /// Stellt eine Meldung in die Statuszeile, die nicht von einem Tab kommt.
    ///
    /// Der Weg der Startmeldungen und der Auswurfmeldung aus C9: eine
    /// beschaedigte `keymap.toml`, eine beschaedigte `session.toml` oder ein
    /// verschwundener Datentraeger gehoeren dem Fenster und keinem einzelnen
    /// Tab. Der naechste Ordner- oder Tabwechsel loescht sie wieder, und das
    /// ist richtig: sie betrifft nicht den Ordner, den der Nutzer dann ansieht.
    ///
    /// Geschrieben wird das Feld, gezeichnet wird ueber
    /// [`DateifensterQuelle::meldung_anzeigen`]. Diese Methode setzt die Zeile
    /// **nicht** selbst: sie kaeme sonst an der Rangfolge vorbei und
    /// ueberschriebe eine laufende Vorgangsanzeige.
    pub fn meldung_zeigen(&self, meldung: &str) {
        *self.ivars().fenstermeldung.borrow_mut() = Some(meldung.to_owned());
        self.meldung_anzeigen();
    }

    /// Stellt die Antwort auf einen Tastenbefehl in die Statuszeile.
    ///
    /// Der oberste Rang: sie steht auch dann in der Zeile, wenn dieses
    /// Dateifenster gerade den Fortschritt einer Operation zeigt. Der Nutzer
    /// hat eben eine Taste gedrueckt und sieht hierher.
    ///
    /// Der Weg fuer "es laeuft bereits eine Operation", "es ist nichts
    /// ausgewaehlt", "die Zwischenablage ist leer" und den Abschlusstext eines
    /// Vorgangs. Ein Ereignis, das der Nutzer nicht angefordert hat, geht
    /// weiter ueber [`DateifensterQuelle::meldung_zeigen`].
    pub fn befehlsantwort_zeigen(&self, antwort: &str) {
        *self.ivars().befehlsantwort.borrow_mut() = Some(antwort.to_owned());
        self.meldung_anzeigen();
    }

    /// Raeumt die Antwort auf den vorigen Tastenbefehl weg.
    ///
    /// Die einzige Loeschregel dieses Feldes, gerufen von
    /// `Anwendungsdelegierter::kommando_ausfuehren` vor jedem Befehl. Stand
    /// keine Antwort, geschieht nichts: sonst schriebe jeder Pfeiltastendruck
    /// die Zeile neu, die sich nicht geaendert hat. Stand eine, kommt zum
    /// Vorschein, was darunter liegt — der Fortschritt der laufenden Operation
    /// oder die verdraengte Auswurfmeldung.
    pub fn befehlsantwort_loeschen(&self) {
        if self.ivars().befehlsantwort.borrow_mut().take().is_some() {
            self.meldung_anzeigen();
        }
    }

    /// Schreibt den Stand einer Dateioperation in die Statuszeile (C4).
    ///
    /// Gerufen vom Anwendungsdelegierten fuer das Dateifenster, das den
    /// Vorgang **begonnen** hat, und nicht fuer das gerade aktive: der Nutzer
    /// darf waehrend der Operation das Fenster wechseln, und "das aktive
    /// Fenster" sagt danach nichts mehr darueber, wohin der Fortschritt
    /// gehoert.
    pub fn vorgang_zeigen(&self, stand: &str) {
        *self.ivars().vorgangsanzeige.borrow_mut() = Some(stand.to_owned());
        self.meldung_anzeigen();
    }

    /// Nimmt die Vorgangsanzeige weg (C4).
    ///
    /// Danach steht in der Zeile wieder, was ohne den Vorgang dort stuende.
    /// Unmittelbar danach setzt `Anwendungsdelegierter::vorgang_beenden` den
    /// Abschlusstext als Befehlsantwort, und die steht darueber; eine waehrend
    /// der Operation eingetroffene Fenstermeldung ist deshalb nicht sofort zu
    /// sehen, sondern sobald der naechste Tastenbefehl den Abschlusstext
    /// wegraeumt. Verloren ist sie nicht: die beiden Texte liegen in zwei
    /// Feldern mit zwei Lebensdauern.
    pub fn vorgang_beenden(&self) {
        *self.ivars().vorgangsanzeige.borrow_mut() = None;
        self.meldung_anzeigen();
    }

    /// Ein Takt des Zeitgebers: Stapel uebernehmen, Tabelle benachrichtigen.
    fn einziehen(&self) {
        let einzug = self.ivars().tabs.borrow_mut().einziehen();
        if einzug.fertig {
            // Erst jetzt steht die Sortierung. Die bisher angezeigten Zeilen
            // standen in Lesereihenfolge, also muss die Tabelle sie neu holen.
            self.ivars().tabelle.reloadData();
            // Und die Auswahl des Nutzers wandert mit ihrem Eintrag an dessen
            // neue Zeile. Die Spanne, in der er waehlen kann, ist gemessen: auf
            // dem Ordner mit 100.000 Eintraegen liegen zwischen der ersten
            // Bildschirmseite und der fertigen Sortierung rund 800 ms.
            self.auswahl_anzeigen();
            self.gemerkten_bildlauf_herstellen();
        } else if einzug.angehaengt {
            self.ivars().tabelle.noteNumberOfRowsChanged();
        }
        if einzug.meldung_neu {
            self.meldung_anzeigen();
        }

        // Die zweite Stufe der Lesereihenfolge: der sichtbare Tab steht, jetzt
        // duerfen die verdeckten lesen.
        if self.ivars().tabs.borrow().nachzuegler_faellig() {
            self.ivars().tabs.borrow_mut().nachzuegler_starten();
        }
        if !self.ivars().tabs.borrow().liest_noch() {
            self.einzug_beenden();
        }
    }

    /// Stellt die aus der Sitzung gemerkte Bildlaufposition her, einmalig.
    ///
    /// Sie steht in `session.toml`, die Liste dazu ist beim Start aber leer;
    /// erst mit dem fertigen Lesevorgang gibt es eine Position, auf die sich
    /// springen laesst.
    fn gemerkten_bildlauf_herstellen(&self) {
        let hoehe = {
            let tabs = self.ivars().tabs.borrow();
            tabs.aktiver()
                .bildlauf_ausstehend()
                .then(|| tabs.aktiver().bildlauf())
        };
        let Some(hoehe) = hoehe else {
            return;
        };
        self.ivars()
            .tabs
            .borrow_mut()
            .aktiver_mut()
            .bildlauf_hergestellt();
        self.bildlauf_herstellen(hoehe);
    }

    /// Haengt den Zeitgeber in die Laufschleife, falls er noch nicht laeuft.
    fn einzug_starten(&self) {
        if self.ivars().einzug.borrow().is_some() {
            return;
        }
        // SAFETY: `self` ist das Ziel und beantwortet `stapelEinziehen:` mit der
        // erwarteten Signatur. Der Zeitgeber wird unten in die Laufschleife
        // gehaengt; `NSRunLoopCommonModes` ist ein Fremdsymbol von Foundation.
        let zeitgeber = unsafe {
            let zeitgeber = NSTimer::timerWithTimeInterval_target_selector_userInfo_repeats(
                EINZUGSTAKT,
                self,
                sel!(stapelEinziehen:),
                None,
                true,
            );
            // Der gewoehnliche Modus ruht, solange der Nutzer blaettert oder ein
            // Menue offen haelt. In den gemeinsamen Modi laeuft das Lesen weiter.
            NSRunLoop::currentRunLoop().addTimer_forMode(&zeitgeber, NSRunLoopCommonModes);
            zeitgeber
        };
        *self.ivars().einzug.borrow_mut() = Some(zeitgeber);
    }

    /// Nimmt den Zeitgeber aus der Laufschleife und loest den Ring auf.
    fn einzug_beenden(&self) {
        if let Some(zeitgeber) = self.ivars().einzug.borrow_mut().take() {
            zeitgeber.invalidate();
        }
    }

    /// Meldet, dass der Nutzer dieses Dateifenster angefasst hat.
    fn angefasst(&self) {
        let melden = self.ivars().aktivierung.borrow();
        if let Some(melden) = melden.as_ref() {
            melden();
        }
    }
}

/// Was der Delegierte haelt.
pub struct DelegiertenIvars {
    /// Die Quelle, aus der der Delegierte die Zeilen liest.
    quelle: Retained<DateifensterQuelle>,
    /// Der Formatierer fuer die Spalte mit dem Aenderungsdatum.
    ///
    /// Er entsteht einmal und nicht je Zelle: ein `NSDateFormatter` baut beim
    /// Anlegen die Kalender- und Sprachtabellen auf und ist damit das teuerste
    /// Objekt im Zeichenweg.
    datumsformat: Retained<NSDateFormatter>,
    /// Die Schrift einer unmarkierten Zelle.
    ///
    /// Die beiden Schriften entstehen einmal und nicht je Zelle, aus demselben
    /// Grund wie der Datumsformatierer: sie liegen im Zeichenweg, den ein
    /// Ordner mit 100.000 Eintraegen oft genug durchlaeuft.
    schrift: Retained<NSFont>,
    /// Die Schrift einer markierten Zelle: dieselbe Groesse, fett.
    ///
    /// Das zweite Kennzeichen der Markierung aus C2 neben der Farbe. Eine Form
    /// und keine Farbe, damit es bei jeder Farbfehlsichtigkeit wirkt
    /// (`decisions/260805-0000_a_zweites-kennzeichen-der-markierung-und-ihr-platz-in-der-statuszeile.md`).
    fettschrift: Retained<NSFont>,
}

define_class!(
    /// Der Delegierte eines Dateifensters: er baut und beschriftet die Zellen.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = DelegiertenIvars]
    pub struct DateifensterDelegierter;

    impl DateifensterDelegierter {
        /// Die Aktion der bearbeitbaren Namenszelle (C4).
        ///
        /// AppKit schickt sie, wenn die Bearbeitung mit Return endet oder die
        /// Zelle den Fokus verliert, und ausdruecklich **nicht** nach Escape.
        // SAFETY: Die Signatur passt zu der, die NSControl an sein Ziel
        // schickt: ein Argument, der Absender.
        #[unsafe(method(umbenennungBeendet:))]
        fn umbenennung_beendet(&self, absender: &NSTextField) {
            self.ivars().quelle.umbenennung_beenden(absender);
        }
    }

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for DateifensterDelegierter {}

    // SAFETY: `NSControlTextEditingDelegate` ist Oberprotokoll von
    // `NSTableViewDelegate` und hat nur wahlfreie Methoden.
    unsafe impl NSControlTextEditingDelegate for DateifensterDelegierter {}

    // SAFETY: `NSTableViewDelegate` stellt keine Bedingungen.
    unsafe impl NSTableViewDelegate for DateifensterDelegierter {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method_id(tableView:viewForTableColumn:row:))]
        fn ansicht_fuer_zelle(
            &self,
            tabelle: &NSTableView,
            spalte: Option<&NSTableColumn>,
            zeile: NSInteger,
        ) -> Option<Retained<NSView>> {
            // Der Rumpf steht in `zellenansicht`, weil `define_class!` den
            // Rueckgabetyp umschreibt und der Fragezeichenoperator hier
            // deshalb nicht greift.
            self.zellenansicht(tabelle, spalte, zeile)
        }

        /// Der Nutzer versucht, eine Zeile auszuwaehlen.
        ///
        /// AppKit ruft das **nur** bei einer Auswahl, die vom Nutzer ausgeht,
        /// und nicht bei `selectRowIndexes:byExtendingSelection:`. Genau
        /// deshalb steht hier die Umschaltung des aktiven Dateifensters: ein
        /// Klick in die andere Liste macht sie zur aktiven, waehrend ein vom
        /// Programm gelesener Ordner im verdeckten Tab nichts umschaltet.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableView:shouldSelectRow:))]
        fn zeile_waehlbar(&self, _tabelle: &NSTableView, _zeile: NSInteger) -> bool {
            self.ivars().quelle.angefasst();
            true
        }

        /// Die Auswahl hat sich geaendert, meist durch einen Mausklick.
        ///
        /// Die Tastatur laeuft nicht hierueber, sondern meldet sich in
        /// `auswahl_verschieben` selbst. Beide muenden in dieselbe Funktion,
        /// damit es nur eine Stelle gibt, die eine Zeile in einen Eintrag
        /// uebersetzt.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableViewSelectionDidChange:))]
        fn auswahl_geaendert(&self, _meldung: &NSNotification) {
            self.ivars().quelle.auswahl_merken();
        }
    }
);

impl DateifensterDelegierter {
    /// Die beschriftete Ansicht fuer eine Zelle.
    ///
    /// Liefert `None` fuer eine Spalte, die KRK nicht kennt, und fuer eine
    /// Zeile, die es im Modell nicht gibt. Beides kann AppKit waehrend eines
    /// Lesevorgangs anfragen, wenn Zeilenzahl und Zeichendurchgang um einen
    /// Takt auseinanderliegen.
    fn zellenansicht(
        &self,
        tabelle: &NSTableView,
        spalte: Option<&NSTableColumn>,
        zeile: NSInteger,
    ) -> Option<Retained<NSView>> {
        let spalte = Spalte::aus_kennung(&spalte?.identifier())?;
        let zeile = usize::try_from(zeile).ok()?;
        let text = self
            .ivars()
            .quelle
            .mit_zeile(zeile, |eintrag| self.beschriften(spalte, eintrag))?;
        let feld = self.feld(tabelle, spalte);
        feld.setStringValue(&NSString::from_str(&text));
        // Die Markierung aus C2 sichtbar machen. Ohne ein Zeichen auf dem
        // Schirm waeren die vier Markierungsbefehle nicht nachweisbar, und der
        // Nutzer wuesste vor einer Dateioperation nicht, worauf sie wirkt.
        // Orange und nicht blau: die Auswahl faerbt AppKit bereits blau, und
        // zwei blaue Kennzeichen liessen sich nicht unterscheiden.
        //
        // **Zwei Kennzeichen und nicht nur die Farbe.** Eine markierte Zeile
        // steht in allen vier Spalten fett; wer die Farbe nicht unterscheiden
        // kann, sieht die Form (S16c). Beide Eigenschaften werden in **jedem**
        // Durchgang gesetzt und nicht nur im markierten Fall: die
        // Zellenansichten sind wiederverwendet, und eine ungesetzte Eigenschaft
        // bliebe die des vorigen Eintrags.
        let markiert = self.ivars().quelle.zeile_markiert(zeile);
        let farbe = if markiert {
            NSColor::systemOrangeColor()
        } else {
            NSColor::labelColor()
        };
        feld.setTextColor(Some(&farbe));
        let schrift = if markiert {
            &self.ivars().fettschrift
        } else {
            &self.ivars().schrift
        };
        feld.setFont(Some(schrift));
        Some(Retained::into_super(Retained::into_super(feld)))
    }

    /// Einen Delegierten fuer die genannte Quelle.
    fn neu(mtm: MainThreadMarker, quelle: Retained<DateifensterQuelle>) -> Retained<Self> {
        let datumsformat = NSDateFormatter::new();
        datumsformat.setDateStyle(NSDateFormatterStyle::ShortStyle);
        datumsformat.setTimeStyle(NSDateFormatterStyle::ShortStyle);
        // Die Systemschriftgroesse und nicht eine Zahl im Programmtext: sie
        // haengt an der Einstellung des Nutzers. Beide Schriften nehmen
        // dieselbe, damit eine markierte Zeile fett wird und nicht groesser.
        let groesse = NSFont::systemFontSize();
        let schrift = NSFont::systemFontOfSize(groesse);
        let fettschrift = NSFont::boldSystemFontOfSize(groesse);
        let this = Self::alloc(mtm).set_ivars(DelegiertenIvars {
            quelle,
            datumsformat,
            schrift,
            fettschrift,
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Die Quelle, aus der dieser Delegierte liest.
    fn quelle(&self) -> &DateifensterQuelle {
        &self.ivars().quelle
    }

    /// Der Text, der in dieser Spalte fuer diesen Eintrag steht.
    fn beschriften(&self, spalte: Spalte, eintrag: &Eintrag) -> String {
        match spalte {
            Spalte::Name => eintrag.name.clone(),
            Spalte::Groesse => {
                if eintrag.ist_ordner() {
                    // Ein Ordner hat keine eigene Groesse, und die seines
                    // Inhalts zu summieren hiesse, ihn zu durchlaufen.
                    "--".to_owned()
                } else {
                    self.quelle().groesse_beschriften(eintrag.groesse)
                }
            }
            Spalte::Geaendert => self.datum_beschriften(eintrag.geaendert),
            Spalte::Typ => typ_beschriften(eintrag.typ).to_owned(),
        }
    }

    /// Ein Zeitpunkt in der Schreibweise, die der Nutzer eingestellt hat.
    fn datum_beschriften(&self, zeitpunkt: SystemTime) -> String {
        let Ok(seit_epoche) = zeitpunkt.duration_since(UNIX_EPOCH) else {
            // Ein Zeitpunkt vor 1970 ist auf einem Dateisystem moeglich, aber
            // kein Fall, fuer den eine eigene Darstellung lohnt.
            return String::new();
        };
        let datum = NSDate::dateWithTimeIntervalSince1970(seit_epoche.as_secs_f64());
        self.ivars().datumsformat.stringFromDate(&datum).to_string()
    }

    /// Holt eine Zellenansicht aus dem Vorrat der Tabelle oder baut eine neue.
    ///
    /// Die Wiederverwendung ist der Grund, aus dem ein Ordner mit 100.000
    /// Eintraegen ohne Ruckeln blaettert: AppKit haelt nur die sichtbaren
    /// Ansichten und reicht die aus dem Bild gelaufenen zurueck.
    fn feld(&self, tabelle: &NSTableView, spalte: Spalte) -> Retained<NSTextField> {
        let kennung = spalte.kennung();
        // SAFETY: `self` ist der Eigentuemer, den AppKit an eine neu geladene
        // Ansicht weiterreicht; die Kennung ist eine gueltige Zeichenkette.
        let vorrat = unsafe { tabelle.makeViewWithIdentifier_owner(kennung, Some(self)) };
        if let Some(gebraucht) = vorrat.and_then(|ansicht| ansicht.downcast::<NSTextField>().ok()) {
            return gebraucht;
        }
        let mtm = self.mtm();
        let feld = NSTextField::labelWithString(ns_string!(""), mtm);
        feld.setIdentifier(Some(kennung));
        feld.setAlignment(spalte.ausrichtung());
        feld.setMaximumNumberOfLines(1);
        if spalte.beschreibbar() {
            // Das Umbenennen "direkt in der Liste" aus C4. Gesetzt wird es
            // einmal beim Bau und nicht je Zeichendurchgang: die Kennung der
            // Zellenansicht ist die der Spalte, ein Feld der Namensspalte
            // kommt also nur wieder in die Namensspalte, und mit ihm seine
            // Aktion.
            feld.setEditable(true);
            // SAFETY: Ziel ist der Delegierte, den `Dateifenster` festhaelt;
            // die Aktion ist die Methode, die er oben ausdruecklich fuer diesen
            // Zweck traegt. `NSControl` haelt sein Ziel schwach, und der
            // Delegierte ueberlebt das Feld: er haelt die Tabelle mittelbar
            // ueber die Quelle.
            unsafe {
                feld.setTarget(Some(self));
                feld.setAction(Some(sel!(umbenennungBeendet:)));
            }
        }
        feld
    }
}

/// Ein aufgebautes Dateifenster: seine drei Ansichten und die Objekte, die
/// AppKit nur schwach referenziert.
///
/// `NSTableView` haelt Datenquelle und Delegierten schwach, `NSControl` sein
/// Ziel ebenso. Wer die Tabelle baut, muss beide anderswo festhalten, sonst
/// fallen sie noch vor dem ersten Zeichendurchgang. Hier ist dieses Anderswo.
pub struct Dateifenster {
    sicht: Retained<NSScrollView>,
    delegierter: Retained<DateifensterDelegierter>,
}

impl Dateifenster {
    /// Baut Tableiste, Tabelle, Bildlaufansicht, Statuszeile, Datenquelle und
    /// Delegierten.
    ///
    /// Die Ansichten entstehen ohne Groesse. Sie bekommen ihre erste beim
    /// Einhaengen in die Aufteilung und jede weitere ueber ihre Autogroesse.
    /// Gelesen wird hier noch nicht: die Reihenfolge, in der die Tabs lesen,
    /// bestimmt das Fenstermodell und nicht der Aufbau.
    pub fn bauen(mtm: MainThreadMarker, tabs: Tabliste) -> Self {
        let rahmen = NSRect::new(NSPoint::ZERO, NSSize::ZERO);
        let tabelle = NSTableView::initWithFrame(NSTableView::alloc(mtm), rahmen);
        tabelle.setRowHeight(ZEILENHOEHE);
        // Ausdruecklich, obwohl es die Vorbelegung ist: an dieser Zeile haengt,
        // dass AppKit die Gesamthoehe rechnet statt jede Zeile zu messen.
        tabelle.setUsesAutomaticRowHeights(false);
        tabelle.setUsesAlternatingRowBackgroundColors(true);
        tabelle.setStyle(NSTableViewStyle::FullWidth);
        // Die Namensspalte nimmt die Breite auf, die beim Vergroessern des
        // Fensters frei wird; die drei rechten tragen feste Inhalte.
        tabelle.setColumnAutoresizingStyle(
            NSTableViewColumnAutoresizingStyle::FirstColumnOnlyAutoresizingStyle,
        );
        for spalte in Spalte::ALLE {
            tabelle.addTableColumn(&spaltenkopf(mtm, spalte));
        }

        let sicht = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), rahmen);
        sicht.setHasVerticalScroller(true);
        sicht.setHasHorizontalScroller(true);
        sicht.setAutohidesScrollers(true);
        sicht.setDocumentView(Some(&tabelle));
        sicht.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        let statuszeile = Statuszeile::bauen(mtm);
        let quelle =
            DateifensterQuelle::neu(mtm, tabelle.clone(), sicht.clone(), statuszeile, tabs);
        let delegierter = DateifensterDelegierter::neu(mtm, quelle);
        // SAFETY: Beide Objekte beantworten die Protokolle, die sie oben
        // implementieren. Ueber ihre Lebensdauer verlangt die Bindung nichts,
        // und die Tabelle ueberlebt beide: sie faellt mitten im Abbau von
        // Quelle und Delegiertem. Getragen wird der Aufruf von der Art der
        // beiden Eigenschaften, die `objc2` an derselben Stelle nennt: "This is
        // a weak property"
        // (`objc2-app-kit-0.3.2/src/generated/NSTableView.rs:402-421`). Weil
        // beide nullende schwache Verweise sind, steht dort `nil`, sobald
        // Quelle oder Delegierter in ihren `dealloc` gehen, und die Tabelle
        // sendet danach an niemanden mehr.
        unsafe {
            tabelle.setDataSource(Some(ProtocolObject::from_ref(delegierter.quelle())));
            tabelle.setDelegate(Some(ProtocolObject::from_ref(&*delegierter)));
        }

        // Die Leiste zuletzt: ihr Rueckruf braucht die Quelle. Er haelt sie
        // **schwach**, sonst schloesse sich der Ring Quelle → Leiste → Ziel →
        // Rueckruf → Quelle.
        let schwach = objc2::rc::Weak::from_retained(&delegierter.quelle().retain());
        let leiste = Tableiste::bauen(mtm, move |stelle| {
            if let Some(quelle) = schwach.load() {
                quelle.angefasst();
                quelle.tab_waehlen(stelle);
            }
        });
        delegierter.quelle().tableiste_setzen(leiste);

        Self { sicht, delegierter }
    }

    /// Die Bildlaufansicht mit der Dateiliste.
    pub fn sicht(&self) -> &NSView {
        &self.sicht
    }

    /// Die Leiste am Kopf mit einem Abschnitt je Tab.
    pub fn tableiste_sicht(&self) -> Retained<NSView> {
        let leiste = self.quelle().ivars().tableiste.borrow();
        let leiste = leiste
            .as_ref()
            .expect("die Tableiste steht seit `Dateifenster::bauen`");
        leiste.sicht().retain()
    }

    /// Die Zeile am Fuss mit den Meldungen dieses Dateifensters.
    pub fn statuszeile_sicht(&self) -> &NSView {
        self.quelle().ivars().statuszeile.sicht()
    }

    /// Die Datenquelle, ueber die ein Ordner gelesen wird.
    pub fn quelle(&self) -> &DateifensterQuelle {
        self.delegierter.quelle()
    }
}

/// Eine Spalte mit Kennung, Ueberschrift und Breiten.
fn spaltenkopf(mtm: MainThreadMarker, spalte: Spalte) -> Retained<NSTableColumn> {
    let (breite, mindestbreite) = spalte.breiten();
    let kopf = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), spalte.kennung());
    kopf.setTitle(spalte.titel());
    kopf.setWidth(breite);
    kopf.setMinWidth(mindestbreite);
    kopf
}

/// Die Benennung einer Eintragsart.
fn typ_beschriften(typ: Typ) -> &'static str {
    match typ {
        Typ::Ordner => "Ordner",
        Typ::Datei => "Datei",
        Typ::Verknuepfung => "Verknüpfung",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jede_spalte_findet_sich_ueber_ihre_kennung_wieder() {
        for spalte in Spalte::ALLE {
            assert_eq!(Spalte::aus_kennung(spalte.kennung()), Some(spalte));
        }
        assert_eq!(Spalte::aus_kennung(ns_string!("unbekannt")), None);
    }

    #[test]
    fn jede_spalte_hat_eine_eigene_kennung_und_ueberschrift() {
        for (stelle, spalte) in Spalte::ALLE.into_iter().enumerate() {
            for andere in Spalte::ALLE.into_iter().skip(stelle + 1) {
                assert_ne!(spalte.kennung(), andere.kennung());
                assert_ne!(spalte.titel(), andere.titel());
            }
        }
    }
}
