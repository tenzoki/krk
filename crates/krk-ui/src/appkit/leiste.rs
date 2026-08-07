//! Die Lesezeichen- und Geraeteleiste links im Fenster (C5).
//!
//! Eine `NSTableView` mit einer Spalte, ohne Kopfzeile, im Stil einer
//! Seitenleiste. Ihr Inhalt steht in [`crate::leistenmodell`]; dieses Modul
//! zeigt ihn an und trifft keine eigene Entscheidung darueber, was in welcher
//! Reihenfolge in der Leiste steht.
//!
//! ```text
//!  Leistenmodell ──> Leistenquelle ──> NSTableView (eine Spalte, Sidebar-Stil)
//!        ^                  │
//!        │                  └──> Senke: eine Auswahl ist gefallen
//!  Anwendungsdelegierter <───────────── Ordner setzen oder Grund melden
//! ```
//!
//! # Warum die Leiste keine eigene Tastenbehandlung hat
//!
//! Sie hat keine `keyDown:`-Methode und kein eigenes Kuerzel. Jeder
//! Tastendruck laeuft durch den einen Ereignisabgriff aus
//! [`super::ereignisse`], wird im Kern nachgeschlagen und kommt als
//! [`Kommando`] hier an — genauso wie im Dateifenster. Eine Ansicht, die eine
//! Taste selbst abfinge, waere die Sonderregel mit eigenem Rueckfallweg, die
//! die Maxime "supersimpel" ausschliesst; und die Auswahl der `NSTableView`
//! wuerde sich dann zweimal bewegen, einmal durch AppKit und einmal durch KRK.
//!
//! Ausgefuehrt wird hier deshalb genau, was C5 der Leiste zuschreibt: der Auf-
//! und der Ab-Pfeil bewegen die Auswahl. Alles uebrige weist
//! [`Leistenquelle::kommando_ausfuehren`] ab, und die vier Befehle, die ein
//! Lesezeichen aendern, fuehrt der Anwendungsdelegierte, weil sie danach die
//! Datei schreiben muessen.
//!
//! # Die Auswahl ist der Befehl
//!
//! C5 sagt: "Die Auswahl eines Eintrags setzt den Ordner des aktiven
//! Dateifensters." Es gibt also keinen zweiten Tastendruck zum Oeffnen, und
//! jede Bewegung der Auswahl navigiert — mit der Maus wie mit der Tastatur,
//! ueber dieselbe Senke. Zeigt ein Lesezeichen ins Leere, geht statt des
//! Ordners der Grund an den Aufrufer.

use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSColor, NSControlTextEditingDelegate, NSFont, NSScrollView,
    NSTableColumn, NSTableColumnResizingOptions, NSTableView, NSTableViewDataSource,
    NSTableViewDelegate, NSTableViewStyle, NSTextField, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSIndexSet, NSInteger, NSNotification, NSObject, NSObjectProtocol, NSPoint,
    NSRect, NSSize, NSString, ns_string,
};

use krk_core::tasten::Kommando;

use crate::leistenmodell::{Auswahl, Leistenmodell, Ort, Zeile};

/// Die Hoehe einer Zeile in Punkten.
///
/// Dieselbe wie in der Dateiliste, und aus demselben Grund fest: eine Liste mit
/// gleich hohen Zeilen laesst AppKit die Gesamthoehe rechnen, statt jede Zeile
/// zu messen.
const ZEILENHOEHE: f64 = 20.0;

/// Der Einzug einer gewoehnlichen Zeile gegenueber der Ueberschrift.
const EINZUG: f64 = 12.0;

/// Was die Leiste ihrem Halter meldet.
///
/// Genau ein Ereignis: der Nutzer hat eine Zeile ausgewaehlt. Was daraus folgt,
/// entscheidet der Anwendungsdelegierte — er kennt das aktive Dateifenster und
/// die Statuszeile, die Leiste kennt beide nicht.
pub type Auswahlsenke = Box<dyn Fn(Auswahl)>;

/// Was die Datenquelle der Leiste haelt.
pub struct LeistenIvars {
    /// Die Tabelle, der die Quelle Aenderungen meldet.
    ///
    /// `NSTableView` haelt Datenquelle und Delegierten nur schwach; die starke
    /// Richtung laeuft deshalb von hier nach dort.
    tabelle: Retained<NSTableView>,
    /// Der Inhalt der Leiste.
    modell: RefCell<Leistenmodell>,
    /// Was bei einer Auswahl zu tun ist.
    ///
    /// Wahlfrei, weil die Quelle vor dem Anwendungsdelegierten zur Welt kommt,
    /// wie die Rueckrufe des Dateifensters auch.
    gewaehlt: RefCell<Option<Auswahlsenke>>,
    /// Wahr, solange die Quelle die Auswahl der Tabelle selbst setzt.
    ///
    /// Ohne dieses Kennzeichen liefe jede Bewegung doppelt: die Quelle setzt
    /// die Auswahl, AppKit meldet die Aenderung, und die Meldung liefe wieder
    /// in dieselbe Quelle. Der Nutzer saehe davon nichts, aber der Ordner
    /// wuerde zweimal gelesen.
    setzt_selbst: RefCell<bool>,
}

define_class!(
    /// Die Datenquelle und der Delegierte der Leiste in einem Objekt.
    ///
    /// Anders als beim Dateifenster, wo Quelle und Delegierter getrennt sind:
    /// dort haelt der Delegierte drei Formatierer und zwei Schriften fuer die
    /// vier Spalten, hier gibt es eine Spalte mit einer Beschriftung. Zwei
    /// Objekte dafuer waeren zwei Halter fuer denselben Zustand.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = LeistenIvars]
    pub struct Leistenquelle;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Leistenquelle {}

    // SAFETY: `NSTableViewDataSource` stellt keine Bedingungen.
    unsafe impl NSTableViewDataSource for Leistenquelle {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(numberOfRowsInTableView:))]
        fn zeilenzahl(&self, _tabelle: &NSTableView) -> NSInteger {
            self.ivars().modell.borrow().zeilen().len() as NSInteger
        }
    }

    // SAFETY: `NSControlTextEditingDelegate` ist Oberprotokoll von
    // `NSTableViewDelegate` und hat nur wahlfreie Methoden. Die Leiste
    // bearbeitet keinen Text; sie erfuellt das Protokoll leer, wie der
    // Delegierte des Dateifensters vor Schritt 17b auch.
    unsafe impl NSControlTextEditingDelegate for Leistenquelle {}

    // SAFETY: `NSTableViewDelegate` stellt keine Bedingungen.
    unsafe impl NSTableViewDelegate for Leistenquelle {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method_id(tableView:viewForTableColumn:row:))]
        fn ansicht_fuer_zelle(
            &self,
            _tabelle: &NSTableView,
            _spalte: Option<&NSTableColumn>,
            zeile: NSInteger,
        ) -> Option<Retained<NSView>> {
            self.zellenansicht(zeile)
        }

        /// Eine Ueberschrift ist eine Gruppenzeile: sie traegt die
        /// Zwischenraeume und den Schriftschnitt, die macOS fuer eine
        /// Seitenleiste vorsieht.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableView:isGroupRow:))]
        fn gruppenzeile(&self, _tabelle: &NSTableView, zeile: NSInteger) -> bool {
            matches!(self.zeile(zeile), Some(Zeile::Ueberschrift(_)))
        }

        /// Eine Ueberschrift laesst sich nicht auswaehlen (C5).
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableView:shouldSelectRow:))]
        fn zeile_waehlbar(&self, _tabelle: &NSTableView, zeile: NSInteger) -> bool {
            self.zeile(zeile).is_some_and(Zeile::waehlbar)
        }

        /// Die Auswahl hat sich geaendert, meist durch einen Mausklick.
        ///
        /// Die Tastatur laeuft nicht hierueber, sondern ueber
        /// [`Leistenquelle::kommando_ausfuehren`]. Beide muenden in
        /// [`Leistenquelle::auswahl_melden`], damit es genau eine Stelle gibt,
        /// die aus einer Zeile einen Ordner macht.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableViewSelectionDidChange:))]
        fn auswahl_geaendert(&self, _meldung: &NSNotification) {
            if *self.ivars().setzt_selbst.borrow() {
                return;
            }
            let zeile = self.ivars().tabelle.selectedRow();
            let Ok(zeile) = usize::try_from(zeile) else {
                return;
            };
            if self.ivars().modell.borrow_mut().waehlen(zeile) {
                self.auswahl_melden();
            }
        }
    }
);

impl Leistenquelle {
    /// Eine Datenquelle fuer die genannte Tabelle.
    fn neu(mtm: MainThreadMarker, tabelle: Retained<NSTableView>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(LeistenIvars {
            tabelle,
            modell: RefCell::new(Leistenmodell::neu()),
            gewaehlt: RefCell::new(None),
            setzt_selbst: RefCell::new(false),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Hinterlegt, was bei einer Auswahl zu tun ist.
    pub fn auswahl_setzen(&self, melden: Auswahlsenke) {
        *self.ivars().gewaehlt.borrow_mut() = Some(melden);
    }

    /// Uebernimmt die Lesezeichen des Nutzers (C5).
    pub fn lesezeichen_setzen(&self, liste: &krk_core::ablage::Lesezeichenliste) {
        self.ivars().modell.borrow_mut().lesezeichen_setzen(liste);
        self.nachziehen();
    }

    /// Uebernimmt die Geraete und Standardorte (C5).
    ///
    /// Gerufen beim Aufbau und nach jedem Ein- und Aushaengen. Die
    /// Gueltigkeitspruefung der Lesezeichen laeuft im selben Zug: ein
    /// eingehaengter Datentraeger macht ein Lesezeichen darauf wieder gueltig,
    /// und ein Auswurf macht es ungueltig. Sie steht seit dem 260807 in
    /// [`Leistenmodell::orte_setzen`] und nicht mehr hier; hier stand sie als
    /// zweiter Weg zu derselben Marke neben
    /// [`Leistenquelle::gueltigkeit_nachziehen`].
    pub fn orte_setzen(&self, orte: Vec<Ort>) {
        self.ivars().modell.borrow_mut().orte_setzen(orte);
        self.nachziehen();
    }

    /// Prueft die Ordner der Lesezeichen und zeichnet neu, falls noetig (C5).
    ///
    /// Zwei Aufrufer: [`Leistenquelle::auswahl_melden`], bevor eine Auswahl
    /// gemeldet wird, und
    /// `crate::appkit::anwendung::Anwendungsdelegierter::vorgang_beenden`, sobald
    /// eine Dateioperation aus C4 abgeschlossen ist. Die beiden anderen Anlaesse
    /// aus [`Leistenmodell::gueltigkeit_pruefen`] laufen ueber
    /// [`Leistenquelle::lesezeichen_setzen`] und [`Leistenquelle::orte_setzen`],
    /// die ohnehin die ganze Liste neu zeichnen.
    ///
    /// **Nach dem Neuzeichnen die Auswahl wieder setzen**, und das ist keine
    /// Vorsichtsmassnahme: `reloadData` nimmt der `NSTableView` ihre Auswahl,
    /// waehrend das Modell seine behaelt. Ohne die zweite Zeile verschwand die
    /// blaue Zeile unter der Hand des Nutzers, sobald ein Lesezeichen seine
    /// Gueltigkeit wechselte, und der naechste Pfeil sprang scheinbar aus dem
    /// Nichts weiter. Beobachtet am 260805 im laufenden Buendel.
    pub fn gueltigkeit_nachziehen(&self) {
        if self.ivars().modell.borrow_mut().gueltigkeit_pruefen() {
            self.ivars().tabelle.reloadData();
            self.auswahl_anzeigen();
        }
    }

    /// Die Lesezeichen, wie sie auf die Platte gehoeren.
    pub fn lesezeichenliste(&self) -> krk_core::ablage::Lesezeichenliste {
        self.ivars().modell.borrow().lesezeichenliste()
    }

    /// Der Name des ausgewaehlten Lesezeichens, falls eines ausgewaehlt ist.
    ///
    /// Die Vorbelegung des Umbenennungsblattes: wer umbenennt, faengt beim
    /// alten Namen an.
    pub fn gewaehlter_lesezeichenname(&self) -> Option<String> {
        let modell = self.ivars().modell.borrow();
        modell.gewaehltes_lesezeichen()?;
        modell.gewaehlt().map(|auswahl| auswahl.name)
    }

    /// Legt ein Lesezeichen an und waehlt es aus (C5).
    pub fn lesezeichen_anlegen(&self, name: &str, ordner: &std::path::Path) {
        self.ivars().modell.borrow_mut().anlegen(name, ordner);
        self.nachziehen();
    }

    /// Benennt das ausgewaehlte Lesezeichen um (C5).
    pub fn lesezeichen_umbenennen(&self, name: &str) -> bool {
        let geaendert = self.ivars().modell.borrow_mut().umbenennen(name);
        if geaendert {
            self.nachziehen();
        }
        geaendert
    }

    /// Loescht das ausgewaehlte Lesezeichen (C5).
    pub fn lesezeichen_loeschen(&self) -> bool {
        let geaendert = self.ivars().modell.borrow_mut().loeschen();
        if geaendert {
            self.nachziehen();
        }
        geaendert
    }

    /// Schiebt das ausgewaehlte Lesezeichen einen Platz weiter (C5).
    pub fn lesezeichen_verschieben(&self, richtung: krk_core::ablage::Verschiebung) -> bool {
        let geaendert = self.ivars().modell.borrow_mut().verschieben(richtung);
        if geaendert {
            self.nachziehen();
        }
        geaendert
    }

    /// Fuehrt ein Kommando aus, das der Ereignisabgriff nachgeschlagen hat.
    ///
    /// Genau die beiden Befehle, die C5 der Leiste zuschreibt. Alles uebrige
    /// gehoert nicht hierher: entweder hat der Wirkungsbereich es schon
    /// abgewiesen, oder es ist ein Befehl des Fensters, den der
    /// Anwendungsdelegierte selbst ausfuehrt.
    pub fn kommando_ausfuehren(&self, kommando: Kommando) -> bool {
        let schritt = match kommando {
            Kommando::AuswahlHoch => -1,
            Kommando::AuswahlRunter => 1,
            _ => return false,
        };
        if !self.ivars().modell.borrow_mut().auswahl_bewegen(schritt) {
            // Am Rand bewegt sich nichts, und der Tastendruck ist trotzdem
            // verbraucht: sonst raeumte AppKit ihn an die `NSTableView` weiter,
            // die daraufhin ihre eigene Auswahl bewegte.
            return true;
        }
        self.auswahl_anzeigen();
        self.auswahl_melden();
        true
    }

    /// Die Tabelle, die den Eingabefokus traegt.
    pub fn liste(&self) -> &NSTableView {
        &self.ivars().tabelle
    }

    /// Schreibt Inhalt und Auswahl in die Tabelle.
    fn nachziehen(&self) {
        self.ivars().tabelle.reloadData();
        self.auswahl_anzeigen();
    }

    /// Setzt die Auswahl der Tabelle auf die des Modells.
    ///
    /// Waehrend des Setzens steht das Kennzeichen `setzt_selbst`: AppKit meldet
    /// jede Aenderung, auch die selbst gesetzte, und ohne das Kennzeichen liefe
    /// die Meldung in dieselbe Quelle zurueck.
    fn auswahl_anzeigen(&self) {
        *self.ivars().setzt_selbst.borrow_mut() = true;
        match self.ivars().modell.borrow().auswahl() {
            Some(zeile) => {
                let stelle = NSIndexSet::indexSetWithIndex(zeile);
                self.ivars()
                    .tabelle
                    .selectRowIndexes_byExtendingSelection(&stelle, false);
                self.ivars().tabelle.scrollRowToVisible(zeile as NSInteger);
            }
            // SAFETY: `deselectAll:` nimmt einen beliebigen Absender; `None`
            // heisst, dass kein Steuerelement den Aufruf ausgeloest hat. Die
            // Bindung ist unsicher, weil der Absender ein `AnyObject` ist.
            None => unsafe { self.ivars().tabelle.deselectAll(None) },
        }
        *self.ivars().setzt_selbst.borrow_mut() = false;
    }

    /// Meldet die ausgewaehlte Zeile an den Halter.
    ///
    /// Die eine Stelle, die aus einer Zeile einen Ordner macht. Vorher wird die
    /// Gueltigkeit nachgezogen, damit die gemeldete Auswahl den Zustand von
    /// jetzt traegt und nicht den vom letzten Neuaufbau; die Begruendung steht
    /// an [`Leistenmodell::gueltigkeit_pruefen`].
    ///
    /// Die Ausleihe des Modells endet vor dem Rueckruf: der Aufrufer liest die
    /// Leiste womoeglich selbst.
    fn auswahl_melden(&self) {
        self.gueltigkeit_nachziehen();
        let Some(auswahl) = self.ivars().modell.borrow().gewaehlt() else {
            return;
        };
        let melden = self.ivars().gewaehlt.borrow();
        if let Some(melden) = melden.as_ref() {
            melden(auswahl);
        }
    }

    /// Die Zeile an dieser Stelle, soweit es sie gibt.
    fn zeile(&self, zeile: NSInteger) -> Option<Zeile> {
        let stelle = usize::try_from(zeile).ok()?;
        self.ivars().modell.borrow().zeile(stelle)
    }

    /// Die beschriftete Ansicht fuer eine Zelle.
    ///
    /// Ein ungueltiges Lesezeichen traegt zwei Kennzeichen: den Zusatz im Text
    /// aus [`crate::leistenmodell`] und die gedaempfte Farbe hier. Zwei, weil
    /// eine Farbe allein bei Farbfehlsichtigkeit keines ist; dieselbe
    /// Ueberlegung wie bei der Markierung aus C2.
    fn zellenansicht(&self, zeile: NSInteger) -> Option<Retained<NSView>> {
        let mtm = self.mtm();
        let stelle = usize::try_from(zeile).ok()?;
        let (text, ungueltig, ueberschrift) = {
            let modell = self.ivars().modell.borrow();
            (
                modell.beschriftung(stelle)?,
                modell.ungueltig(stelle),
                matches!(modell.zeile(stelle), Some(Zeile::Ueberschrift(_))),
            )
        };

        let beschriftung = NSTextField::labelWithString(&NSString::from_str(&text), mtm);
        beschriftung.setFont(Some(&NSFont::systemFontOfSize(
            NSFont::smallSystemFontSize(),
        )));
        if ungueltig {
            beschriftung.setTextColor(Some(&NSColor::tertiaryLabelColor()));
        } else if ueberschrift {
            beschriftung.setTextColor(Some(&NSColor::secondaryLabelColor()));
        }
        // Der Einzug trennt die Eintraege von ihrer Ueberschrift. Er steht im
        // Rahmen und nicht in einer eigenen Zellenklasse: eine Beschriftung mit
        // Abstand nach links ist keine neue Ansichtsart.
        let einzug = if ueberschrift { 0.0 } else { EINZUG };
        beschriftung.setFrame(NSRect::new(
            NSPoint::new(einzug, 0.0),
            NSSize::new(0.0, ZEILENHOEHE),
        ));
        beschriftung.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);

        let zelle = NSView::initWithFrame(
            NSView::alloc(mtm),
            NSRect::new(NSPoint::ZERO, NSSize::new(0.0, ZEILENHOEHE)),
        );
        zelle.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
        zelle.addSubview(&beschriftung);
        Some(zelle)
    }
}

/// Die aufgebaute Leiste: ihre Ansicht und die Quelle, die AppKit nur schwach
/// haelt.
pub struct Leiste {
    sicht: Retained<NSScrollView>,
    quelle: Retained<Leistenquelle>,
}

impl Leiste {
    /// Baut Tabelle, Bildlaufansicht und Datenquelle.
    ///
    /// Die Ansicht entsteht ohne Groesse und bekommt ihre erste beim Einhaengen
    /// in die Aufteilung, wie das Dateifenster auch.
    pub fn bauen(mtm: MainThreadMarker) -> Self {
        let rahmen = NSRect::new(NSPoint::ZERO, NSSize::ZERO);
        let tabelle = NSTableView::initWithFrame(NSTableView::alloc(mtm), rahmen);
        tabelle.setRowHeight(ZEILENHOEHE);
        tabelle.setUsesAutomaticRowHeights(false);
        // Der Stil einer Seitenleiste: durchscheinender Grund, gerundete
        // Auswahl, die Zwischenraeume um eine Gruppenzeile. Ihn selbst zu
        // zeichnen hiesse, das Erscheinungsbild von Hell und Dunkel nachzubauen.
        tabelle.setStyle(NSTableViewStyle::SourceList);
        tabelle.setHeaderView(None);
        tabelle.setAllowsEmptySelection(true);
        tabelle.setAllowsMultipleSelection(false);

        let spalte =
            NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), ns_string!("leiste"));
        spalte.setResizingMask(NSTableColumnResizingOptions::AutoresizingMask);
        tabelle.addTableColumn(&spalte);

        let sicht = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), rahmen);
        sicht.setHasVerticalScroller(true);
        sicht.setAutohidesScrollers(true);
        sicht.setDocumentView(Some(&tabelle));
        sicht.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        let quelle = Leistenquelle::neu(mtm, tabelle.clone());
        // SAFETY: Die Quelle beantwortet beide Protokolle, die sie oben
        // implementiert. Ueber die Lebensdauer verlangt die Bindung nichts;
        // getragen wird der Aufruf davon, dass `dataSource` und `delegate`
        // nullende schwache Eigenschaften sind ("This is a weak property",
        // `objc2-app-kit-0.3.2/src/generated/NSTableView.rs:402-421`), und dass
        // `Leiste` die Quelle selbst festhaelt.
        unsafe {
            tabelle.setDataSource(Some(ProtocolObject::from_ref(&*quelle)));
            tabelle.setDelegate(Some(ProtocolObject::from_ref(&*quelle)));
        }

        Self { sicht, quelle }
    }

    /// Die Ansicht, die in die Aufteilung gehaengt wird.
    pub fn sicht(&self) -> &NSView {
        &self.sicht
    }

    /// Die Datenquelle mit dem Inhalt der Leiste.
    pub fn quelle(&self) -> &Leistenquelle {
        &self.quelle
    }
}
