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
use std::time::{SystemTime, UNIX_EPOCH};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSControlTextEditingDelegate, NSScrollView, NSTableColumn,
    NSTableView, NSTableViewColumnAutoresizingStyle, NSTableViewDataSource, NSTableViewDelegate,
    NSTableViewStyle, NSTextAlignment, NSTextField, NSUserInterfaceItemIdentification, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSByteCountFormatter, NSByteCountFormatterCountStyle, NSDate,
    NSDateFormatter, NSDateFormatterStyle, NSIndexSet, NSInteger, NSNotification, NSObject,
    NSObjectProtocol, NSPoint, NSRect, NSRunLoop, NSRunLoopCommonModes, NSSize, NSString,
    NSTimeInterval, NSTimer, ns_string,
};

use krk_core::ablage::Dateifenster as Fensterzustand;
use krk_core::tasten::Kommando;
use krk_core::verzeichnis::{Eintrag, Typ};

use crate::tabs::Tabliste;

use super::statuszeile::Statuszeile;
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
}

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
        let this = Self::alloc(mtm).set_ivars(QuelleIvars {
            tabelle,
            sicht,
            statuszeile,
            tableiste: RefCell::new(None),
            tabs: RefCell::new(tabs),
            einzug: RefCell::new(None),
            aktivierung: RefCell::new(None),
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
    pub fn ordner_lesen(&self, pfad: &Path) {
        self.ivars().tabs.borrow_mut().ordner_setzen(pfad);
        self.ivars().tabelle.reloadData();
        // `ordner_setzen` hat die Auswahl des alten Ordners aufgehoben; die
        // Tabelle erfaehrt es hier. Sich darauf zu verlassen, dass `reloadData`
        // eine Zeilennummer jenseits der neuen Zeilenzahl von selbst fallen
        // laesst, hiesse, eine Zusage von AppKit anzunehmen statt sie zu geben.
        self.auswahl_anzeigen();
        self.meldung_anzeigen();
        self.einzug_starten();
        self.tableiste_nachziehen();
    }

    /// Nach einem Tabwechsel: Inhalt, Auswahl, Bildlauf und Leiste nachziehen.
    fn tab_gewechselt(&self) {
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
            Kommando::AuswahlHoch => self.auswahl_verschieben(-1),
            Kommando::AuswahlRunter => self.auswahl_verschieben(1),
            Kommando::SeiteHoch => self.auswahl_verschieben(-self.seitenhoehe()),
            Kommando::SeiteRunter => self.auswahl_verschieben(self.seitenhoehe()),
            Kommando::Oeffnen => self.auswahl_oeffnen(),
            Kommando::TabNeu => self.tab_neu(),
            Kommando::TabSchliessen => self.tab_schliessen(),
            Kommando::TabNaechster => self.tab_naechster(),
            Kommando::TabVoriger => self.tab_voriger(),
            // Nicht Sache eines einzelnen Dateifensters.
            _ => return false,
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

    /// Verschiebt die Auswahl um die genannte Zahl von Zeilen.
    ///
    /// Am Rand bleibt sie stehen, statt umzulaufen. Ohne bestehende Auswahl
    /// faengt sie an dem Rand an, aus dem die Bewegung kommt: Pfeil ab setzt
    /// auf die erste Zeile, Pfeil auf auf die letzte.
    fn auswahl_verschieben(&self, schritte: isize) {
        let zeilen = self.zeilen();
        let Some(letzte) = zeilen.checked_sub(1) else {
            return;
        };
        let letzte = letzte as isize;

        let tabelle = &self.ivars().tabelle;
        // `selectedRow` liefert -1, solange nichts ausgewaehlt ist.
        let jetzt = tabelle.selectedRow();
        let ziel = if jetzt < 0 {
            if schritte < 0 { letzte } else { 0 }
        } else {
            jetzt.saturating_add(schritte).clamp(0, letzte)
        };

        let auswahl = NSIndexSet::indexSetWithIndex(ziel as usize);
        tabelle.selectRowIndexes_byExtendingSelection(&auswahl, false);
        tabelle.scrollRowToVisible(ziel as NSInteger);
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
            self.ordner_lesen(&ziel);
        }
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

    // ------------------------------------------------------------------
    // Bildlauf, Statuszeile, Einzugstakt
    // ------------------------------------------------------------------

    /// Liest die Bildlaufposition aus der Ansicht in den sichtbaren Tab.
    fn bildlauf_merken(&self) {
        let hoehe = self.ivars().sicht.contentView().bounds().origin.y;
        self.ivars()
            .tabs
            .borrow_mut()
            .aktiver_mut()
            .bildlauf_setzen(hoehe);
    }

    /// Stellt die genannte Bildlaufposition in der Ansicht her.
    fn bildlauf_herstellen(&self, hoehe: f64) {
        let inhalt = self.ivars().sicht.contentView();
        inhalt.scrollToPoint(NSPoint::new(0.0, hoehe));
        self.ivars().sicht.reflectScrolledClipView(&inhalt);
    }

    /// Zeigt die Meldung des sichtbaren Tabs in der Statuszeile.
    fn meldung_anzeigen(&self) {
        let meldung = self
            .ivars()
            .tabs
            .borrow()
            .aktiver()
            .meldung()
            .map(str::to_owned);
        self.ivars().statuszeile.zeigen(meldung.as_deref());
    }

    /// Stellt eine Meldung in die Statuszeile, die nicht von einem Tab kommt.
    ///
    /// Der Weg der Startmeldungen: eine beschaedigte `keymap.toml` oder
    /// `session.toml` gehoert dem Fenster und keinem einzelnen Tab. Der naechste
    /// Ordnerwechsel loescht sie wieder, und das ist richtig: sie betrifft den
    /// Start und nicht den Ordner, den der Nutzer gerade ansieht.
    pub fn meldung_zeigen(&self, meldung: &str) {
        self.ivars().statuszeile.zeigen(Some(meldung));
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
    /// Der Formatierer fuer die Spalte mit der Groesse.
    ///
    /// Foundation bringt ihn mit, und er zaehlt in derselben Weise wie der
    /// Finder: dezimale Vorsaetze, Trennzeichen nach der Spracheinstellung des
    /// Nutzers. Eine eigene Rechnung waere eine zweite Wahrheit neben der des
    /// Systems.
    groessenformat: Retained<NSByteCountFormatter>,
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
        Some(Retained::into_super(Retained::into_super(feld)))
    }

    /// Einen Delegierten fuer die genannte Quelle.
    fn neu(mtm: MainThreadMarker, quelle: Retained<DateifensterQuelle>) -> Retained<Self> {
        let datumsformat = NSDateFormatter::new();
        datumsformat.setDateStyle(NSDateFormatterStyle::ShortStyle);
        datumsformat.setTimeStyle(NSDateFormatterStyle::ShortStyle);
        let groessenformat = NSByteCountFormatter::new();
        groessenformat.setCountStyle(NSByteCountFormatterCountStyle::File);
        let this = Self::alloc(mtm).set_ivars(DelegiertenIvars {
            quelle,
            datumsformat,
            groessenformat,
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
                    self.groesse_beschriften(eintrag.groesse)
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

    /// Eine Byte-Zahl in der Schreibweise des Systems.
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
