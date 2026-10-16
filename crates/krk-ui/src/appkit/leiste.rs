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
//!
//! # Ein Klick verbindet, eine Auswahl nicht (261016)
//!
//! **Fuer eine Verbindung gilt der Satz darueber nicht.** Ihre Auswahl tut
//! nichts; verbunden wird allein auf einen ausdruecklichen Anstoss, den
//! Mausklick auf die Zeile oder den Befehl „Verbindung herstellen“
//! (Entscheidung 12 des Plans
//! `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`).
//! Eine Pfeiltaste, die ueber eine Verbindung laeuft, fragt sonst je Zeile
//! nach einer PIN oder haengt eine Freigabe ein.
//!
//! Der Klick kommt deshalb nicht ueber die Auswahlmeldung, sondern ueber die
//! `action` der Tabelle ([`Leistenquelle`] ist ihr Ziel): AppKit sendet sie
//! auf jeden Mausklick, auch auf eine schon gewaehlte Zeile, und nie auf eine
//! Bewegung der Auswahl von der Tastatur. Welche Zeile geklickt wurde, sagt
//! `clickedRow`; ob sie eine Verbindung ist, sagt
//! [`Leistenmodell::verbindung_der_zeile`], und die Senke dafuer haelt der
//! Anwendungsdelegierte.
//!
//! # Ein Datentraeger laesst sich aus der Leiste auswerfen (261016)
//!
//! **Zwei Anstoesse, ein Weg.** Ein Rechtsklick auf eine Geraetezeile oeffnet
//! ein Kontextmenue mit dem einen Eintrag „Auswerfen“, und rechts in derselben
//! Zeile steht, wie im Finder, ein Auswerfsinnbild (`eject`), dessen Klick
//! dasselbe tut. Beide melden den Einhaengepunkt ueber dieselbe Senke
//! ([`Auswurfsenke`]) an den Anwendungsdelegierten, und der wirft ueber
//! denselben Rumpf aus wie das Kontextmenue der Dateiliste
//! (`Anwendungsdelegierter::auswurf_beginnen`): dieselbe Regel, dieselbe
//! Sperre gegen den zweiten Auftrag, dieselbe Antwort in der Statuszeile.
//! **Diese Datei wirft nichts aus**; sie kennt `super::auswurf` nicht.
//!
//! **Welche Zeile auswerfbar ist, entscheidet
//! [`Leistenmodell::auswurfziel`]** ueber `kontextmenue::auswurfbezug`, also
//! dieselbe Regel wie in der Dateiliste: ein Ort, dessen Pfad ein
//! Einhaengepunkt ist, und nie die Wurzel. Das Benutzerverzeichnis, der
//! Startdatentraeger, die Lesezeichen und die Verbindungen bekommen weder
//! Menue noch Sinnbild; ein Rechtsklick auf sie oeffnet nichts.
//!
//! **Das Menue entsteht in `menuForEvent:` einer Unterklasse
//! ([`Leistenliste`])** und nicht in `menuNeedsUpdate:` an einem stehenden
//! Menue, aus dem Grund, den der Kopf von [`super::tabelle`] fuer die
//! Dateiliste ausschreibt: die Zeile kommt aus dem Ereignis und nicht aus
//! `clickedRow`, ueber dessen Stand der Kopf des Systems zu diesem Zeitpunkt
//! nichts sagt. Die Unterklasse setzt das gebaute Menue (oder keines) als
//! `menu` der Tabelle und reicht die Frage an die Oberklasse weiter, damit
//! AppKit die angeklickte Zeile wie gewohnt umrandet. **Die Auswahl rueckt
//! dabei nicht nach**: in der Leiste ist die Auswahl der Befehl, und ein
//! Rechtsklick, der den Ordner des Dateifensters wechselte, taete mehr, als
//! der Nutzer verlangt hat. Der Einhaengepunkt reist deshalb nicht ueber die
//! Auswahl, sondern wird beim Menuebau gemerkt und beim Klick gelesen.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSView`, `NSScrollView`, `NSTableView`, `NSTableColumn`, `NSTextField`,
//! `NSImageView`, `NSImage`, `NSFont`, `NSColor`, `NSIndexSet`,
//! `NSNotification` und `NSString` stehen seit macOS 10.0 zur Verfuegung,
//! ebenso die drei bedienten Protokolle `NSTableViewDataSource`,
//! `NSTableViewDelegate` und `NSControlTextEditingDelegate`. Das Buendel zielt
//! auf 15.0 (`.cargo/config.toml`). Fuer den Klick (261016) kommen
//! `setTarget:`, `setAction:` (`NSControl.h`) und `clickedRow`
//! (`NSTableView.h`) hinzu, alle drei ohne eigene Angabe und damit seit 10.0;
//! das Sinnbild der Verbindung, der Systemname `network`, geht durch dieselbe
//! `imageWithSystemSymbolName:accessibilityDescription:` wie die zwei
//! anderen, seit 11.0.
//!
//! **Fuer das Auswerfen (261016) kommen hinzu**, jede am SDK nachgelesen:
//! `menuForEvent:` (`NSView.h:291`), das [`Leistenliste`] ueberschreibt,
//! `convertPoint:fromView:` (`NSView.h:149`), `initWithFrame:` (`NSView.h:83`)
//! und `rowAtPoint:` (`NSTableView.h:411`), `NSEvent` (`NSEvent.h:317`) mit
//! `locationInWindow` (`NSEvent.h:337`), `NSMenu` (`NSMenu.h:56`) mit `new`
//! und `addItem:` (`NSMenu.h:92`), die Eigenschaft `menu` von `NSResponder`
//! (`NSResponder.h:111`), `NSMenuItem` (`NSMenuItem.h:23`) mit dem schwach
//! gehaltenen `target` (`NSMenuItem.h:93`), `NSButton` mit `setBordered:` (`NSButton.h:109`), die
//! Eigenschaft `tag` von `NSView` und `NSControl` und `toolTip` von `NSView`
//! (`NSView.h:310`) — alle ohne `API_AVAILABLE` und damit seit 10.0. Juenger
//! sind zwei: `buttonWithImage:target:action:` seit 10.12
//! (`NSButton.h:50`) und `contentTintColor` an `NSButton` seit 10.14
//! (`NSButton.h:121`). Das `NSMenuItem` legt diese Datei nicht selbst an,
//! sondern ueber [`super::menue::ohne_kuerzel`]; das Sinnbild `eject` geht
//! durch dieselbe `imageWithSystemSymbolName:accessibilityDescription:` wie
//! die anderen, seit 11.0, und das Schwachverweisen ueber `Weak` ist ein
//! Rust-Typ der Kiste ohne macOS-Alter.
//!
//! **Etliche Beruehrungen sind juenger als ihre Klasse, und alle liegen unter
//! dem Zielsystem:**
//!
//! - `tableView:isGroupRow:` seit 10.5 (`NSTableView.h:685`)
//! - `tableView:viewForTableColumn:row:` seit 10.7 (`NSTableView.h:593`)
//! - `secondaryLabelColor` und `tertiaryLabelColor` seit 10.10
//!   (`NSColor.h:202` und `:203`)
//! - `NSFontWeightRegular` und `monospacedDigitSystemFontOfSize:weight:` seit
//!   10.11 (`NSFontDescriptor.h:170`, `NSFont.h:62`). Dieselben beiden Stellen
//!   nennt die SAFETY-Begruendung an der Lesestelle des Fremdsymbols; sie
//!   bleibt dort, weil sie den `unsafe`-Block traegt.
//! - `NSTextField::labelWithString:` und `NSImageView::imageViewWithImage:`
//!   seit 10.12 (`NSTextField.h:93`, `NSImageView.h:40`)
//! - `setUsesAutomaticRowHeights:` seit 10.13 (`NSTableView.h:574`)
//! - `setContentTintColor:` seit 10.14 (`NSImageView.h:58`)
//! - `NSTableViewStyle` samt `setStyle:` und
//!   `imageWithSystemSymbolName:accessibilityDescription:` seit 11.0
//!   (`NSTableView.h:77` und `:377`, `NSImage.h:82`) — die hoechste
//!   Untergrenze dieser Datei
//!
//! Keine von ihnen ist nach macOS 15 hinzugekommen, und keine Beruehrung in
//! dieser Datei braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit.
//! `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer
//! haelt die Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.
//!
//! **Was die `use`-Zeilen daneben hereinholen, und warum keines davon die
//! Untergrenze dieser Datei anhebt:** `MainThreadMarker` ist ein Rust-Typ der
//! Kiste und hat kein macOS-Alter; das Makro `ns_string!` baut die
//! Zeichenkette beim Uebersetzen und hat keines; `NSObjectProtocol` ist der
//! Kistenname des Protokolls `NSObject` (`objc/NSObject.h`, ohne eigene
//! Angabe); `NSPoint`, `NSRect` und `NSSize` sind C-Strukturen
//! (`NSGeometry.h:23`, `:33` und `:28`); `NSInteger` ist ein Ganzzahltyp
//! (`objc/NSObjCRuntime.h:13`); `NSObject` steht seit 10.0
//! (`objc/NSObject.h`); die Aufzaehlungen `NSAutoresizingMaskOptions`
//! (`NSView.h:33`) und `NSTableColumnResizingOptions` (`NSTableColumn.h:20`)
//! schliessen mit blossem `};`, `NSImageScaling` (`NSCell.h:60`) traegt
//! `API_AVAILABLE(macos(10.5))`; alle uebrigen tragen im SDK keine eigene
//! Verfuegbarkeitsangabe und stehen damit seit 10.0.

use std::cell::RefCell;
use std::path::PathBuf;

use objc2::rc::{Retained, Weak};
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSButton, NSColor, NSControlTextEditingDelegate, NSEvent, NSFont,
    NSFontWeightRegular, NSImage, NSImageScaling, NSImageView, NSMenu, NSScrollView, NSTableColumn,
    NSTableColumnResizingOptions, NSTableView, NSTableViewDataSource, NSTableViewDelegate,
    NSTableViewStyle, NSTextField, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSIndexSet, NSInteger, NSNotification, NSObject, NSObjectProtocol, NSPoint,
    NSRect, NSSize, NSString, ns_string,
};

use krk_core::sprache::{Text, text};
use krk_core::tasten::Kommando;

use crate::leistenmodell::{Auswahl, Leistenmodell, Ort, Sinnbild, Zeile};

use super::menue;

/// Die Hoehe einer Zeile in Punkten.
///
/// Dieselbe wie in der Dateiliste, und aus demselben Grund fest: eine Liste mit
/// gleich hohen Zeilen laesst AppKit die Gesamthoehe rechnen, statt jede Zeile
/// zu messen.
const ZEILENHOEHE: f64 = 20.0;

/// Der Einzug einer gewoehnlichen Zeile gegenueber der Ueberschrift.
const EINZUG: f64 = 12.0;

/// Die Breite der Spalte, in der das Sinnbild steht (C6).
///
/// Sie steht fuer **jede** waehlbare Zeile, auch wenn das System das Sinnbild
/// nicht liefert: sonst rueckten die Beschriftungen zweier benachbarter Zeilen
/// unterschiedlich weit ein, und die Liste saehe zerrissen aus.
const SINNBILDBREITE: f64 = 16.0;

/// Der Abstand zwischen dem Sinnbild und der Beschriftung.
const SINNBILDABSTAND: f64 = 5.0;

/// Die Hoehe des Sinnbilds in seiner Spalte.
const SINNBILDHOEHE: f64 = 14.0;

/// Der Systemname des Sinnbilds fuer einen Ordner (C5).
const SINNBILD_ORDNER: &str = "folder";

/// Der Systemname des Sinnbilds fuer eine Stelle in einer Datei (C6).
const SINNBILD_TEXTSTELLE: &str = "doc.text";

/// Der Systemname des Sinnbilds fuer eine Verbindung (261016).
const SINNBILD_VERBINDUNG: &str = "network";

/// Der Systemname des Auswerfsinnbilds rechts in einer auswerfbaren Zeile
/// (261016), dasselbe Zeichen wie im Finder.
const SINNBILD_AUSWERFEN: &str = "eject";

/// Die Breite der Schaltflaeche mit dem Auswerfsinnbild, gemessen vom
/// rechten Rand der Zelle (261016).
const AUSWERFKNOPFBREITE: f64 = 18.0;

/// Der Abstand der Schaltflaeche zum rechten Rand der Zelle.
const AUSWERFKNOPFABSTAND: f64 = 4.0;

/// Die Breite, mit der eine Zelle entsteht, bevor die Tabelle sie auslegt.
///
/// Der Wert selbst ist gleichgueltig; das Verhaeltnis in ihm ist es nicht. Eine
/// Beschriftung mit fester linker Kante und beweglicher Breite behaelt beim
/// Auslegen ihren rechten Abstand, und der ist genau dann null, wenn sie hier
/// bis an den rechten Rand der Zelle reicht. Mit der Nullbreite, mit der die
/// Zelle bis zum 260810 entstand, war er **negativ**, und die Beschriftung ragte
/// um ihren Einzug ueber die Zelle hinaus; ein langer Name wurde deshalb erst
/// hinter dem sichtbaren Rand gekuerzt.
const AUFBAUBREITE: f64 = 200.0;

/// Was die Leiste ihrem Halter meldet.
///
/// Genau ein Ereignis: der Nutzer hat eine Zeile ausgewaehlt. Was daraus folgt,
/// entscheidet der Anwendungsdelegierte — er kennt das aktive Dateifenster und
/// die Statuszeile, die Leiste kennt beide nicht.
pub type Auswahlsenke = Box<dyn Fn(Auswahl)>;

/// Was ein Klick auf eine Verbindung meldet: den Namen ihres Eintrags in
/// `secrets.txt` (261016).
pub type Verbindungssenke = Box<dyn Fn(String)>;

/// Was ein Auswurf aus der Leiste meldet: den Einhaengepunkt der Zeile
/// (261016). Ausgeworfen wird beim Halter, ueber denselben Weg wie aus der
/// Dateiliste.
pub type Auswurfsenke = Box<dyn Fn(PathBuf)>;

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
    /// Was bei einem Klick auf eine Verbindung zu tun ist (261016); wahlfrei
    /// aus demselben Grund wie [`Self::gewaehlt`].
    verbinden: RefCell<Option<Verbindungssenke>>,
    /// Was bei „Auswerfen“ zu tun ist (261016); wahlfrei aus demselben Grund
    /// wie [`Self::gewaehlt`].
    auswerfen: RefCell<Option<Auswurfsenke>>,
    /// Der Einhaengepunkt, fuer den das zuletzt gebaute Kontextmenue steht
    /// (261016).
    ///
    /// Gemerkt beim Bau und nicht beim Klick aus der Zeile nachgerechnet:
    /// zwischen beiden kann ein Aushaengen die Zeilen neu ordnen, und dann
    /// stuende unter derselben Nummer ein anderer Datentraeger.
    menueziel: RefCell<Option<PathBuf>>,
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
    /// dort haelt der Delegierte zwei Formatierer und zwei Schriften fuer die
    /// fuenf Spalten, hier gibt es eine Spalte mit einer Beschriftung. Zwei
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

    impl Leistenquelle {
        /// Die `action` der Tabelle: ein Mausklick auf eine Zeile (261016).
        ///
        /// AppKit sendet sie nach der Auswahlmeldung, also hat
        /// [`Leistenquelle::auswahl_geaendert`] eine neu gewaehlte Zeile schon
        /// gemeldet; fuer eine Verbindung tut jene Meldung nichts, und
        /// verbunden wird allein hier. Ein Klick neben die Zeilen meldet
        /// `clickedRow` als `-1` und tut nichts.
        // SAFETY: Die Signatur ist die einer Handlung von AppKit, ein Absender
        // und kein Rueckgabewert.
        #[unsafe(method(zeileGeklickt:))]
        fn zeile_geklickt(&self, _absender: Option<&AnyObject>) {
            let Ok(zeile) = usize::try_from(self.ivars().tabelle.clickedRow()) else {
                return;
            };
            let name = self.ivars().modell.borrow().verbindung_der_zeile(zeile);
            if let Some(name) = name {
                self.verbindung_melden(name);
            }
        }

        /// Der Eintrag „Auswerfen“ des Kontextmenues (261016).
        ///
        /// Er liest den Einhaengepunkt, den [`Leistenquelle::kontextmenue`]
        /// beim Bau gemerkt hat, und nimmt ihn dabei heraus; ein zweiter
        /// Klick auf dasselbe Menue gibt es nicht, denn es schliesst mit dem
        /// ersten.
        // SAFETY: Die Signatur ist die einer Handlung von AppKit, ein Absender
        // und kein Rueckgabewert.
        #[unsafe(method(auswerfenAusDemMenue:))]
        fn auswerfen_aus_dem_menue(&self, _absender: Option<&AnyObject>) {
            let ziel = self.ivars().menueziel.borrow_mut().take();
            if let Some(ziel) = ziel {
                self.auswurf_melden(ziel);
            }
        }

        /// Die Schaltflaeche mit dem Auswerfsinnbild rechts in der Zeile
        /// (261016).
        ///
        /// Ihre Marke ist die Zeile, und die Zeile wird hier gegen das Modell
        /// von jetzt gelesen: die Schaltflaeche entsteht mit der Zellenansicht
        /// und faellt mit jedem Neuzeichnen, also traegt sie die Nummer des
        /// Standes, den der Nutzer sieht.
        // SAFETY: Die Signatur ist die einer Handlung von AppKit; der Absender
        // ist die Schaltflaeche, die diese Datei mit diesem Selektor baut.
        #[unsafe(method(auswerfknopfGeklickt:))]
        fn auswerfknopf_geklickt(&self, absender: &NSButton) {
            let Ok(zeile) = usize::try_from(absender.tag()) else {
                return;
            };
            let ziel = self.ivars().modell.borrow().auswurfziel(zeile);
            if let Some(ziel) = ziel {
                self.auswurf_melden(ziel);
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
            verbinden: RefCell::new(None),
            auswerfen: RefCell::new(None),
            menueziel: RefCell::new(None),
            setzt_selbst: RefCell::new(false),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Hinterlegt, was bei einer Auswahl zu tun ist.
    pub fn auswahl_setzen(&self, melden: Auswahlsenke) {
        *self.ivars().gewaehlt.borrow_mut() = Some(melden);
    }

    /// Hinterlegt, was bei einem Klick auf eine Verbindung zu tun ist
    /// (261016).
    pub fn verbinden_setzen(&self, verbinden: Verbindungssenke) {
        *self.ivars().verbinden.borrow_mut() = Some(verbinden);
    }

    /// Hinterlegt, was bei „Auswerfen“ zu tun ist (261016).
    pub fn auswerfen_setzen(&self, auswerfen: Auswurfsenke) {
        *self.ivars().auswerfen.borrow_mut() = Some(auswerfen);
    }

    /// Meldet einen Auswurf an den Halter (261016).
    ///
    /// Die eine Stelle, an der Menueeintrag und Sinnbild zusammenlaufen. Die
    /// Ausleihe der Senke haelt, solange der Rueckruf laeuft, wie in
    /// [`Self::auswahl_melden`]; die des Modells ist beim Aufruf zu Ende.
    fn auswurf_melden(&self, einhaengepunkt: PathBuf) {
        let auswerfen = self.ivars().auswerfen.borrow();
        if let Some(auswerfen) = auswerfen.as_ref() {
            auswerfen(einhaengepunkt);
        }
    }

    /// Das Kontextmenue fuer die angeklickte Zeile, oder keines (261016).
    ///
    /// **Allein eine auswerfbare Zeile bekommt eines**, mit dem einen Eintrag
    /// „Auswerfen“; jede andere Zeile und der Raum neben den Zeilen bekommen
    /// keines, und der Rechtsklick tut dort nichts. Der Einhaengepunkt wird in
    /// [`LeistenIvars::menueziel`] gemerkt, und zwar auch dann geraeumt, wenn
    /// kein Menue entsteht: ein Eintrag von vorhin faende sonst ein Ziel, das
    /// er nicht meint.
    ///
    /// `angeklickt` ist, was `rowAtPoint:` geliefert hat, `-1` eingeschlossen.
    fn kontextmenue(&self, angeklickt: NSInteger) -> Option<Retained<NSMenu>> {
        let ziel = usize::try_from(angeklickt)
            .ok()
            .and_then(|zeile| self.ivars().modell.borrow().auswurfziel(zeile));
        *self.ivars().menueziel.borrow_mut() = ziel.clone();
        ziel?;
        let mtm = self.mtm();
        let menue = NSMenu::new(mtm);
        let eintrag = menue::ohne_kuerzel(
            mtm,
            &NSString::from_str(text(Text::KontextAuswerfen)),
            Some(sel!(auswerfenAusDemMenue:)),
        );
        // SAFETY: `setTarget:` verlangt ein Ziel vom richtigen Typ
        // (`NSMenuItem.h:93`); diese Quelle traegt `auswerfenAusDemMenue:`
        // selbst. Gehalten wird es schwach, und ein gestorbenes Ziel liefert
        // `nil` und keinen Absturz.
        unsafe { eintrag.setTarget(Some(self)) };
        menue.addItem(&eintrag);
        Some(menue)
    }

    /// Der Name des Eintrags in `secrets.txt`, wenn die gewaehlte Zeile eine
    /// Verbindung ist; die Frage des Befehls „Verbindung herstellen“
    /// (261016).
    #[must_use]
    pub fn gewaehlte_verbindung(&self) -> Option<String> {
        self.ivars().modell.borrow().gewaehlte_verbindung()
    }

    /// Meldet einen Klick auf eine Verbindung an den Halter.
    ///
    /// Die Ausleihe des Modells ist beim Aufruf schon zu Ende; die der Senke
    /// haelt, solange der Rueckruf laeuft, wie in [`Self::auswahl_melden`].
    fn verbindung_melden(&self, name: String) {
        let verbinden = self.ivars().verbinden.borrow();
        if let Some(verbinden) = verbinden.as_ref() {
            verbinden(name);
        }
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
    ///
    /// `einhaengepunkte` stammen aus derselben Aufzaehlung wie `orte` und
    /// entscheiden, welche Zeile Auswerfsinnbild und Kontextmenue traegt
    /// (261016).
    pub fn orte_setzen(&self, orte: Vec<Ort>, einhaengepunkte: Vec<PathBuf>) {
        self.ivars()
            .modell
            .borrow_mut()
            .orte_setzen(orte, einhaengepunkte);
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
    #[must_use]
    pub fn lesezeichenliste(&self) -> krk_core::ablage::Lesezeichenliste {
        self.ivars().modell.borrow().lesezeichenliste()
    }

    /// Das ausgewaehlte Lesezeichen als Wert, falls eines ausgewaehlt ist (C5).
    ///
    /// Das Ziel der drei Befehle, die ein vorhandenes Lesezeichen aendern. Der
    /// Eintrag und nicht seine Stelle; der Grund steht bei
    /// [`Leistenmodell::gewaehltes_lesezeichen_wert`].
    #[must_use]
    pub fn gewaehltes_lesezeichen(&self) -> Option<krk_core::ablage::Lesezeichen> {
        self.ivars().modell.borrow().gewaehltes_lesezeichen_wert()
    }

    /// Uebernimmt eine gelesene Lesezeichenliste und stellt die Auswahl (C5).
    ///
    /// Der eine Weg, auf dem eine geaenderte Liste in die Leiste kommt. Bis zur
    /// Runde 7 standen hier vier Methoden, die selbst rechneten und deren
    /// Ergebnis der Delegierte danach schrieb; seither entsteht die Liste unter
    /// der Schreibsperre aus der Datei, und die Leiste zeigt, was dabei
    /// herauskam.
    pub fn lesezeichen_uebernehmen(
        &self,
        liste: &krk_core::ablage::Lesezeichenliste,
        stelle: Option<usize>,
    ) {
        self.ivars().modell.borrow_mut().uebernehmen(liste, stelle);
        self.nachziehen();
    }

    /// Fuehrt ein Kommando aus, das der Ereignisabgriff nachgeschlagen hat.
    ///
    /// Genau die beiden Befehle, die C5 der Leiste zuschreibt. Alles uebrige
    /// gehoert nicht hierher: entweder hat der Wirkungsbereich es schon
    /// abgewiesen, oder es ist ein Befehl des Fensters, den der
    /// Anwendungsdelegierte selbst ausfuehrt.
    #[must_use = "die Antwort sagt, ob die Leiste den Befehl ausgefuehrt hat; fallengelassen laeuft der Tastendruck ungeprueft weiter"]
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
    ///
    /// **Vor der Beschriftung steht das Sinnbild der Sorte** (C6): ein Ordner
    /// oder ein Dokument, aus [`Leistenmodell::sinnbild`]. Es ist das
    /// Kennzeichen, an dem der Nutzer eine Textmarke von einer Ordnermarke
    /// unterscheidet, und es steht aus demselben Grund neben der Farbe wie der
    /// Zusatz "(fehlt)". Seine Spalte steht in jeder waehlbaren Zeile, auch
    /// wenn das System das Bild nicht liefert; ohne sie ruecken die
    /// Beschriftungen unterschiedlich weit ein.
    fn zellenansicht(&self, zeile: NSInteger) -> Option<Retained<NSView>> {
        let mtm = self.mtm();
        let stelle = usize::try_from(zeile).ok()?;
        let (text, ungueltig, ueberschrift, sinnbild, auswerfbar) = {
            let modell = self.ivars().modell.borrow();
            (
                modell.beschriftung(stelle)?,
                modell.ungueltig(stelle),
                matches!(modell.zeile(stelle), Some(Zeile::Ueberschrift(_))),
                modell.sinnbild(stelle),
                modell.auswurfziel(stelle).is_some(),
            )
        };

        let beschriftung = NSTextField::labelWithString(&NSString::from_str(&text), mtm);
        // Festbreite Ziffern bei proportionalen Buchstaben, dieselbe Wahl wie
        // in der Dateiliste und aus demselben Grund: der Vorschlag fuer den
        // Namen einer Textmarke ist `Dateiname:Zeilennummer`
        // (`Anwendungsdelegierter::anlegeziel`), und untereinander gelesen
        // ergeben diese Nummern in der Proportionalschrift keine Spalte. Die
        // Buchstaben der Namen ruehrt der Wechsel nicht an; die Messung dazu
        // steht bei `DateifensterDelegierter::neu` in [`super::tabelle`].
        //
        // SAFETY: `NSFontWeightRegular` ist ein Fremdsymbol von AppKit, ein
        // `CGFloat`. Es wird gelesen und nicht geschrieben. Die Konstante und
        // `monospacedDigitSystemFontOfSize:weight:` tragen im Kopf des Systems
        // beide `API_AVAILABLE(macos(10.11))` (`NSFontDescriptor.h:170`,
        // `NSFont.h:62`); die Untergrenze des Buendels ist 15.0
        // (`.cargo/config.toml`), eine Verfuegbarkeitspruefung zur Laufzeit
        // braucht keine der beiden Stellen.
        let gewoehnlich = unsafe { NSFontWeightRegular };
        beschriftung.setFont(Some(&NSFont::monospacedDigitSystemFontOfSize_weight(
            NSFont::smallSystemFontSize(),
            gewoehnlich,
        )));
        if ungueltig {
            beschriftung.setTextColor(Some(&NSColor::tertiaryLabelColor()));
        } else if ueberschrift {
            beschriftung.setTextColor(Some(&NSColor::secondaryLabelColor()));
        }
        // Der Einzug trennt die Eintraege von ihrer Ueberschrift, und die
        // Sinnbildspalte kommt bei jeder waehlbaren Zeile hinzu. Beides steht im
        // Rahmen und nicht in einer eigenen Zellenklasse: eine Beschriftung mit
        // Abstand nach links ist keine neue Ansichtsart.
        let einzug = match sinnbild.is_some() {
            true => EINZUG + SINNBILDBREITE + SINNBILDABSTAND,
            false => 0.0,
        };
        // Eine auswerfbare Zeile laesst rechts Platz fuer ihr Sinnbild, damit
        // ein langer Name vor ihm gekuerzt wird und nicht darunter.
        let rechts = match auswerfbar {
            true => AUSWERFKNOPFBREITE + 2.0 * AUSWERFKNOPFABSTAND,
            false => 0.0,
        };
        beschriftung.setFrame(NSRect::new(
            NSPoint::new(einzug, 0.0),
            NSSize::new(AUFBAUBREITE - einzug - rechts, ZEILENHOEHE),
        ));
        beschriftung.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);

        let zelle = NSView::initWithFrame(
            NSView::alloc(mtm),
            NSRect::new(NSPoint::ZERO, NSSize::new(AUFBAUBREITE, ZEILENHOEHE)),
        );
        zelle.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
        if let Some(bildsicht) =
            sinnbild.and_then(|sinnbild| sinnbildansicht(mtm, sinnbild, ungueltig))
        {
            zelle.addSubview(&bildsicht);
        }
        zelle.addSubview(&beschriftung);
        if auswerfbar && let Some(knopf) = self.auswerfknopf(zeile) {
            zelle.addSubview(&knopf);
        }
        Some(zelle)
    }

    /// Die Schaltflaeche mit dem Auswerfsinnbild am rechten Rand einer
    /// auswerfbaren Zeile (261016).
    ///
    /// **Ohne Rahmen, mit derselben gedaempften Farbe wie das Sinnbild links**,
    /// wie im Finder; ihre Marke ist die Zeile, gelesen in
    /// [`Leistenquelle::auswerfknopf_geklickt`]. `None`, wenn das laufende
    /// System das Sinnbild nicht kennt; dann bleibt das Kontextmenue der Weg.
    /// Der Tooltip und die Beschreibung fuer VoiceOver sind der Titel des
    /// Menueeintrags, denn beide nennen dieselbe Handlung.
    fn auswerfknopf(&self, zeile: NSInteger) -> Option<Retained<NSButton>> {
        let mtm = self.mtm();
        let titel = NSString::from_str(text(Text::KontextAuswerfen));
        let bild = NSImage::imageWithSystemSymbolName_accessibilityDescription(
            &NSString::from_str(SINNBILD_AUSWERFEN),
            Some(&titel),
        )?;
        // SAFETY: Das Ziel ist diese Quelle, die `auswerfknopfGeklickt:` mit
        // der Signatur einer Handlung beantwortet, und `sel!` liefert einen
        // gueltigen Selektor. `NSControl` haelt sein Ziel schwach; die Quelle
        // haelt `Leiste`.
        let knopf = unsafe {
            NSButton::buttonWithImage_target_action(
                &bild,
                Some(self),
                Some(sel!(auswerfknopfGeklickt:)),
                mtm,
            )
        };
        knopf.setBordered(false);
        knopf.setContentTintColor(Some(&NSColor::secondaryLabelColor()));
        knopf.setTag(zeile);
        knopf.setToolTip(Some(&titel));
        knopf.setFrame(NSRect::new(
            NSPoint::new(
                AUFBAUBREITE - AUSWERFKNOPFBREITE - AUSWERFKNOPFABSTAND,
                (ZEILENHOEHE - SINNBILDHOEHE) / 2.0,
            ),
            NSSize::new(AUSWERFKNOPFBREITE, SINNBILDHOEHE),
        ));
        // Am rechten Rand festgehalten: die Zelle waechst mit der Spalte, und
        // die Schaltflaeche soll mit ihr nach rechts gehen statt stehen zu
        // bleiben.
        knopf.setAutoresizingMask(NSAutoresizingMaskOptions::ViewMinXMargin);
        Some(knopf)
    }
}

/// Die Bildansicht fuer das Sinnbild einer Zeile (C6).
///
/// `None`, wenn das laufende System das Systemsinnbild nicht kennt. Dann bleibt
/// die Spalte leer, und die Zeile behaelt ihren Einzug: eine Beschriftung, die
/// dafuer nach links ruecken wuerde, waere ein zweiter Rechenweg fuer denselben
/// Rahmen.
///
/// Die Fallunterscheidung ueber [`Sinnbild`] ist vollstaendig und hat keinen
/// Auffangzweig; eine dritte Sorte haelt den Bau an und erzwingt ihr Bild.
fn sinnbildansicht(
    mtm: MainThreadMarker,
    sinnbild: Sinnbild,
    ungueltig: bool,
) -> Option<Retained<NSImageView>> {
    // Die Beschreibung ist die Auskunft, die VoiceOver vorliest; sie nennt die
    // Sorte und nicht das Bedienelement, wie AppKit es fuer jede
    // Bildbeschreibung verlangt.
    let (name, beschreibung) = match sinnbild {
        Sinnbild::Ordner => (SINNBILD_ORDNER, text(Text::TypOrdner)),
        Sinnbild::Textstelle => (SINNBILD_TEXTSTELLE, text(Text::LeisteSinnbildTextstelle)),
        Sinnbild::Verbindung => (SINNBILD_VERBINDUNG, text(Text::LeisteSinnbildVerbindung)),
    };
    let bild = NSImage::imageWithSystemSymbolName_accessibilityDescription(
        &NSString::from_str(name),
        Some(&NSString::from_str(beschreibung)),
    )?;

    let sicht = NSImageView::imageViewWithImage(&bild, mtm);
    sicht.setImageScaling(NSImageScaling::ScaleProportionallyDown);
    sicht.setFrame(NSRect::new(
        NSPoint::new(EINZUG, (ZEILENHOEHE - SINNBILDHOEHE) / 2.0),
        NSSize::new(SINNBILDBREITE, SINNBILDHOEHE),
    ));
    // Dieselben beiden Farben wie die Beschriftung daneben: das Sinnbild einer
    // ungueltigen Marke ist so gedaempft wie ihr Text, damit die Zeile als eine
    // gelesen wird. Die Akzentfarbe waere die dritte, und auf dem blauen Grund
    // der ausgewaehlten Zeile die schlechter lesbare.
    let farbe = match ungueltig {
        true => NSColor::tertiaryLabelColor(),
        false => NSColor::secondaryLabelColor(),
    };
    sicht.setContentTintColor(Some(&farbe));
    Some(sicht)
}

define_class!(
    /// Die Tabelle der Leiste, als Unterklasse allein fuer `menuForEvent:`
    /// (261016).
    ///
    /// **Sie baut kein Menue und kennt keinen Eintrag**: sie rechnet die
    /// angeklickte Zeile aus dem Ereignis aus, laesst die Quelle das Menue
    /// bauen (oder keines), setzt es als `menu` der Tabelle und reicht die
    /// Frage an die Oberklasse weiter. Dieselbe Form wie `Dateiliste` in
    /// [`super::tabelle`], und aus demselben Grund: die Zeile kommt aus dem
    /// Ereignis und nicht aus `clickedRow`.
    ///
    /// **Der Rueckverweis ist schwach.** Die Quelle haelt die Tabelle stark,
    /// also bricht der Ring an dieser Kante. Fehlt der Verweis, kommt kein
    /// Menue.
    // SAFETY:
    // - Die Oberklasse `NSTableView` stellt an eine Unterklasse keine
    //   Bedingung, die diese Klasse verletzt: sie ruft den bezeichneten
    //   Erzeuger `initWithFrame:` der Oberklasse, sie ist weder ihre eigene
    //   Datenquelle noch ihr eigener Delegierter, und ihre eine
    //   Ueberschreibung ruft die Fassung der Oberklasse.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSTableView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = RefCell<Option<Weak<Leistenquelle>>>]
    pub struct Leistenliste;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Leistenliste {}

    impl Leistenliste {
        /// Der Rechtsklick: das Menue der angeklickten Zeile, oder keines.
        // SAFETY: Die Signatur entspricht der von NSView (`NSView.h:291`): ein
        // Ereignis, ein optionales Menue zurueck.
        #[unsafe(method_id(menuForEvent:))]
        fn kontextmenue(&self, ereignis: &NSEvent) -> Option<Retained<NSMenu>> {
            let quelle = self.ivars().borrow().as_ref().and_then(Weak::load);
            let menue = quelle.and_then(|quelle| {
                let ort = self.convertPoint_fromView(ereignis.locationInWindow(), None);
                quelle.kontextmenue(self.rowAtPoint(ort))
            });
            // SAFETY: `setMenu:` ist unsicher gebunden und verlangt nichts
            // weiter, als dass das Menue eines ist oder `None`; dieselbe
            // Antwort wie beim Kontextmenue der Dateiliste. Die Tabelle haelt
            // es danach stark, bis zum naechsten Rechtsklick.
            unsafe { self.setMenu(menue.as_deref()) };
            // SAFETY: Die Oberklasse beantwortet dieselbe Nachricht mit
            // demselben Argument und liefert ein optionales Menue: das eben
            // gesetzte, oder keines.
            unsafe { msg_send![super(self), menuForEvent: ereignis] }
        }
    }
);

impl Leistenliste {
    /// Eine Tabelle mit dem genannten Rahmen, noch ohne Rueckverweis.
    fn neu(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(RefCell::new(None));
        // SAFETY: `initWithFrame:` von NSView hat die hier angenommene
        // Signatur.
        unsafe { msg_send![super(this), initWithFrame: rahmen] }
    }

    /// Traegt den Rueckverweis nach, sobald es die Quelle gibt; die eine
    /// Stelle, die ihn setzt, ist [`Leiste::bauen`].
    fn ziel_setzen(&self, quelle: &Leistenquelle) {
        *self.ivars().borrow_mut() = Some(Weak::from_retained(&quelle.retain()));
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
        let liste = Leistenliste::neu(mtm, rahmen);
        let tabelle: Retained<NSTableView> = Retained::into_super(liste.clone());
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
        // Der Klick auf eine Verbindung (261016). Das Ziel haelt die Tabelle
        // schwach, wie Datenquelle und Delegierten; `Leiste` haelt die Quelle.
        // SAFETY: `zeileGeklickt:` ist oben mit der Signatur einer Handlung
        // erklaert, und das Ziel ist die Quelle, die sie beantwortet.
        unsafe {
            tabelle.setTarget(Some(&quelle));
            tabelle.setAction(Some(sel!(zeileGeklickt:)));
        }

        liste.ziel_setzen(&quelle);

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
