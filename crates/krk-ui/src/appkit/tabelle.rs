//! Das Dateifenster: `NSTableView` in einer `NSScrollView`, angebunden an das
//! Ordnermodell des Kerns.
//!
//! Zwei Objective-C-Klassen teilen sich die Arbeit, weil AppKit sie an zwei
//! Protokollen entgegennimmt. [`DateifensterQuelle`] ist die Datenquelle: sie
//! haelt das Ordnermodell, startet Lesevorgaenge und meldet die Zeilenzahl.
//! [`DateifensterDelegierter`] ist der Delegierte: er baut die Zellen und
//! beschriftet sie. Der Delegierte haelt die Quelle, nicht umgekehrt, denn er
//! liest aus ihr; die Gegenrichtung gibt es nicht und damit auch keinen Zyklus.
//!
//! **Wie die Stapel den Hauptfaden erreichen.** Der Leser aus `krk-core` laeuft
//! auf einem Arbeitsfaden und schickt Stapel zu 1.024 Eintraegen ueber einen
//! Kanal der Tiefe 1. Ein Zeitgeber auf dem Hauptfaden raeumt den Kanal
//! sechzigmal je Sekunde leer, haengt die Stapel an das Modell und meldet der
//! Tabelle **einmal** je Takt eine neue Zeilenzahl. Damit erfuellt der erste
//! Stapel die Zusage L2 (erste Bildschirmseite sichtbar), waehrend der Rest
//! anhaengt, und die Tabelle zeichnet hoechstens einmal je Bild neu.
//!
//! Jeder Stapel traegt seine Generationsnummer. Wer schnell durch Ordner
//! navigiert, hat mehrere Lesevorgaenge unterwegs; der Hauptfaden verwirft
//! jeden Stapel, dessen Generation nicht mehr die des Modells ist. Das ersetzt
//! eine Abbruchbehandlung je Lesevorgang durch eine Bedingung.

use std::cell::{Cell, RefCell};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSControlTextEditingDelegate, NSScrollView, NSTableColumn,
    NSTableView, NSTableViewColumnAutoresizingStyle, NSTableViewDataSource, NSTableViewDelegate,
    NSTableViewStyle, NSTextAlignment, NSTextField, NSUserInterfaceItemIdentification, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSByteCountFormatter, NSByteCountFormatterCountStyle, NSDate,
    NSDateFormatter, NSDateFormatterStyle, NSInteger, NSObject, NSObjectProtocol, NSPoint, NSRect,
    NSRunLoop, NSRunLoopCommonModes, NSSize, NSString, NSTimeInterval, NSTimer, ns_string,
};

use krk_core::verzeichnis::{Abschluss, Eintrag, Lesevorgang, Meldung, Ordnermodell, Typ};

/// Die Hoehe einer Zeile in Punkten.
///
/// Sie ist fest und wird nicht je Zeile geschaetzt. Eine Dateiliste hat gleich
/// hohe Zeilen, damit rechnet AppKit die Gesamthoehe konstant statt linear, und
/// erst das macht die Bildlaufleiste eines Ordners mit 100.000 Eintraegen
/// sofort richtig (L10).
const ZEILENHOEHE: f64 = 20.0;

/// Der Takt, in dem der Hauptfaden den Kanal des Lesers leerraeumt.
///
/// Ein Sechzigstel einer Sekunde ist ein Bild auf dem Referenzgeraet. Haeufiger
/// zu raeumen brauchte es nicht, weil die Tabelle ohnehin nicht oefter zeichnet.
const EINZUGSTAKT: NSTimeInterval = 1.0 / 60.0;

/// Die Generation eines Modells, das noch nichts gelesen hat.
const GENERATION_LEER: u64 = 0;

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
            Spalte::Name => (320.0, 120.0),
            Spalte::Groesse => (100.0, 60.0),
            Spalte::Geaendert => (160.0, 120.0),
            Spalte::Typ => (110.0, 70.0),
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
    /// Die gelesenen Eintraege und ihre Sichtreihenfolge.
    modell: RefCell<Ordnermodell>,
    /// Der Lesevorgang, der gerade laeuft, falls einer laeuft.
    lesevorgang: RefCell<Option<Lesevorgang>>,
    /// Die Generation, die der naechste Lesevorgang bekommt.
    letzte_generation: Cell<u64>,
    /// Der Zeitgeber, der den Kanal des Lesers leerraeumt.
    ///
    /// Er haelt die Quelle als Ziel fest, und die Quelle haelt ihn. Der Ring
    /// bricht mit `invalidate`, das jeder Lauf am Ende aufruft.
    einzug: RefCell<Option<Retained<NSTimer>>>,
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
            self.ivars().modell.borrow().zeilenzahl() as NSInteger
        }
    }
);

impl DateifensterQuelle {
    /// Eine Datenquelle fuer die genannte Tabelle.
    fn neu(mtm: MainThreadMarker, tabelle: Retained<NSTableView>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(QuelleIvars {
            tabelle,
            modell: RefCell::new(Ordnermodell::neu(GENERATION_LEER)),
            lesevorgang: RefCell::new(None),
            letzte_generation: Cell::new(GENERATION_LEER),
            einzug: RefCell::new(None),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Liest den genannten Ordner und ersetzt damit den bisherigen Inhalt.
    ///
    /// Kehrt sofort zurueck. Der Inhalt trifft gestueckelt ein; die erste
    /// Bildschirmseite steht mit dem ersten Stapel.
    pub fn ordner_lesen(&self, pfad: &Path) {
        let generation = self.ivars().letzte_generation.get() + 1;
        self.ivars().letzte_generation.set(generation);

        // Der bisherige Lesevorgang faellt hier. Sein Arbeitsfaden bemerkt den
        // Abbruch und endet von selbst; auf ihn zu warten hiesse, eine
        // Navigation an den verlassenen Ordner zu haengen.
        *self.ivars().lesevorgang.borrow_mut() = None;
        self.ivars().modell.borrow_mut().leeren(generation);
        self.ivars().tabelle.reloadData();

        *self.ivars().lesevorgang.borrow_mut() = Some(Lesevorgang::starten(pfad, generation));
        self.einzug_starten();
    }

    /// Bricht einen laufenden Lesevorgang ab und laesst stehen, was schon da ist.
    pub fn lesen_abbrechen(&self) {
        self.einzug_beenden();
        if let Some(vorgang) = self.ivars().lesevorgang.borrow_mut().take() {
            vorgang.abbrechen();
        }
        self.ivars().modell.borrow_mut().abschliessen();
        self.ivars().tabelle.reloadData();
    }

    /// Reicht den Eintrag der genannten Zeile an eine Auswertung weiter.
    ///
    /// Der Zugriff laeuft ueber einen Rueckruf und nicht ueber eine
    /// herausgegebene Referenz, damit die Ausleihe des Modells hier endet und
    /// kein Aufrufer sie ueber einen AppKit-Aufruf hinweg haelt.
    fn mit_zeile<T>(&self, zeile: usize, auswerten: impl FnOnce(&Eintrag) -> T) -> Option<T> {
        let modell = self.ivars().modell.borrow();
        modell.zeile(zeile).map(auswerten)
    }

    /// Ein Takt des Zeitgebers: Stapel uebernehmen, Tabelle benachrichtigen.
    fn einziehen(&self) {
        let (angehaengt, fertig) = self.stapel_uebernehmen();
        if fertig {
            self.einzug_beenden();
            *self.ivars().lesevorgang.borrow_mut() = None;
            // Erst jetzt steht die Sortierung. Die bisher angezeigten Zeilen
            // standen in Lesereihenfolge, also muss die Tabelle sie neu holen.
            self.ivars().tabelle.reloadData();
        } else if angehaengt {
            self.ivars().tabelle.noteNumberOfRowsChanged();
        }
    }

    /// Holt alle wartenden Meldungen aus dem Kanal.
    ///
    /// Liefert, ob Eintraege angehaengt wurden und ob der Lauf zu Ende ist.
    fn stapel_uebernehmen(&self) -> (bool, bool) {
        let vorgang = self.ivars().lesevorgang.borrow();
        let Some(vorgang) = vorgang.as_ref() else {
            return (false, true);
        };
        let mut modell = self.ivars().modell.borrow_mut();
        let mut angehaengt = false;
        let mut fertig = false;
        for meldung in vorgang.meldungen().try_iter() {
            if !modell.gehoert_dazu(meldung.generation()) {
                continue;
            }
            match meldung {
                Meldung::Stapel { eintraege, .. } => {
                    modell.anhaengen(eintraege);
                    angehaengt = true;
                }
                Meldung::Fertig { abschluss, .. } => {
                    if let Abschluss::Fehler(fehler) = &abschluss {
                        eprintln!("krk: Ordner nicht vollstaendig lesbar: {fehler}");
                    }
                    modell.abschliessen();
                    fertig = true;
                    break;
                }
            }
        }
        (angehaengt, fertig)
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

/// Ein aufgebautes Dateifenster: die Bildlaufansicht und die Objekte, die
/// AppKit nur schwach referenziert.
///
/// `NSTableView` haelt Datenquelle und Delegierten schwach. Wer die Tabelle
/// baut, muss beide anderswo festhalten, sonst fallen sie noch vor dem ersten
/// Zeichendurchgang. Hier ist dieses Anderswo.
pub struct Dateifenster {
    sicht: Retained<NSScrollView>,
    delegierter: Retained<DateifensterDelegierter>,
}

impl Dateifenster {
    /// Baut Tabelle, Bildlaufansicht, Datenquelle und Delegierten.
    ///
    /// Die Ansicht entsteht ohne Groesse. Sie bekommt ihre erste beim
    /// Einhaengen ins Fenster, das seinen Inhalt auf den Inhaltsbereich zieht,
    /// und jede weitere ueber ihre Autogroesse.
    pub fn bauen(mtm: MainThreadMarker) -> Self {
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

        let quelle = DateifensterQuelle::neu(mtm, tabelle.clone());
        let delegierter = DateifensterDelegierter::neu(mtm, quelle);
        // SAFETY: Beide Objekte beantworten die Protokolle, die sie oben
        // implementieren, und leben laenger als die Tabelle: `Dateifenster`
        // haelt sie fest.
        unsafe {
            tabelle.setDataSource(Some(ProtocolObject::from_ref(delegierter.quelle())));
            tabelle.setDelegate(Some(ProtocolObject::from_ref(&*delegierter)));
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

        Self { sicht, delegierter }
    }

    /// Die Ansicht, die in das Fenster gehaengt wird.
    pub fn sicht(&self) -> &NSView {
        &self.sicht
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
