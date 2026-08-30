// **Das ganze Modul haengt an einem Rufer, und der kommt in Schritt 8.**
// `Gitfenster::bauen` ist sein einziger Eingang; von dort aus ist jedes Stueck
// dieser Datei erreichbar, und ohne ihn ist keines es. Die Einhaengung beim
// Anwendungsdelegierten — der Bau des Bereichs, `fokusansicht`,
// `bereichskommando` und der Nachlademelder — steht in Schritt 8 des Plans der
// Runde 23; bis dahin haelt der Platzhalter aus Schritt 1 die Stelle in der
// Aufteilung.
//
// Die Ausnahme steht als `expect` und nicht als `allow`, und das ist ihr
// Ablaufdatum: sobald Schritt 8 `bauen` ruft, ist jedes Stueck lebendig, der
// Uebersetzer meldet die Erwartung als unerfuellt und zwingt zum Entfernen
// dieser Zeile. Dieselbe Form wie in `crate::gitmodell` und in `crate::tabs`.
//
// **Sie steht ohne `cfg_attr(not(test))`, anders als dort**, und der Grund
// gehoert dazu: die Proben unten reichen an die reinen Funktionen heran und
// nicht an die AppKit-Haelfte, denn `libtest` gibt den Hauptfaden nicht her,
// den `NSTableView` und `NSTextField` verlangen. Im Probenbau ist die
// AppKit-Haelfte also genauso ungerufen wie im ausgelieferten, und eine auf
// `not(test)` beschraenkte Ausnahme liesse `cargo clippy --all-targets` unter
// `-D warnings` rot werden.
#![expect(
    dead_code,
    reason = "Rufer ist die Einhaengung beim Anwendungsdelegierten aus Schritt 8"
)]
//! Der Git-Bereich: Kopf, Verlaufsliste und Einzelheiten, angebunden an das
//! Modell aus [`crate::gitmodell`] (C1, C3, C4; Runde 23).
//!
//! ```text
//! ┌──────────────────────────────┐
//! │ Kopf (NSTextField, 2 Zeilen) │  Branch/Kurzhash, darunter die
//! ├──────────────────────────────┤  Zusammenfassung des Status
//! │ Verlaufsliste                │  NSTableView, eine Spalte, keine
//! │   NSScrollView + NSTableView │  Kopfzeile; nimmt den Ersthelferrang
//! │                              │
//! ├──────────────────────────────┤
//! │ Einzelheiten                 │  mehrzeiliges Etikett in einer Rolle;
//! │   NSScrollView + NSTextField │  leer, solange nichts ausgewaehlt ist
//! └──────────────────────────────┘
//! ```
//!
//! Die drei Flaechen stehen untereinander in der Traegeransicht, mit
//! Autoresizing und **ohne zweite `NSSplitView`**: der Nutzer soll sie nicht
//! gegeneinander verschieben, und ein Schieberegler im Bereich waere ein
//! Bedienelement, das der Spec nicht verlangt. Kopf und Einzelheiten haben
//! feste Hoehen, die Liste bekommt, was uebrig bleibt.
//!
//! # Ein Schreiber der drei Flaechen
//!
//! [`Gitfenster::zeigen`] ist die eine Stelle, die aus einem [`Gitmodell`]
//! Anzeige macht — dieselbe Bauform wie `Vorschaufenster::anzeigen`. Dieses
//! Modul formt dabei **keinen Text**: die vier Saetze kommen aus
//! [`krk_core::git::texte`] und reisen ueber die Leseseite des Gitmodells
//! hierher. Der Grund steht im Kopf jener Datei: `krk-ui` hat kein
//! Bibliotheksziel, und ein Satz ohne Probe ist der Satz, den die naechste
//! Runde unbemerkt aendert.
//!
//! **Waehrend des Ladens steht nichts da** (A8, C4.4). Kein Platzhaltertext,
//! keine Platzhalterzeile, kein Fortschrittsanzeiger: dieses Modul kennt
//! keinen Zwischentext und kann deshalb keinen zeigen. Es schreibt, was das
//! Modell hergibt, und das ist vor der ersten Antwort die leere Zeichenkette.
//! Ruhe beim Ordnerwechsel ist in einem Programm, dessen Zusagen in
//! Einzelbildern gemessen werden, eine Eigenschaft und kein Geschmack.
//!
//! **Der Bereich blendet sich nie selbst aus** (C6.4). In dieser Datei steht
//! kein Aufruf von `Fenstermodell::sichtbar_setzen`, und es gibt keinen Zweig,
//! der aus einem Ordnerinhalt eine Sichtbarkeit machte. Ein Ordner ohne
//! Repository ist der Normalfall; er bekommt seinen Satz und behaelt seine
//! Breite. Ebenso geht von hier **keine Meldung** in die Statuszeile, in ein
//! Hinweisfenster oder auf die Standardfehlerausgabe (C6.6).
//!
//! # Warum die Flaeche der Einzelheiten kein `NSTextView` ist
//!
//! Sie ist ein `NSTextField` als mehrzeiliges Etikett, und das ist eine
//! Entscheidung und keine Bequemlichkeit. Eine `NSTextView` waere die dritte
//! eigene Textflaeche von KRK, und dann muesste
//! `Anwendungsdelegierter::ist_eigene_textflaeche` entscheiden, ob sie sich
//! dort anmeldet: [`super::ereignisse::ersthelfer_gehoert_appkit`] fragt nach
//! der **Naemlichkeit** des Ersthelfers und nicht nach seiner Klasse, und wer
//! eine Flaeche dort anmeldet oder nicht anmeldet, entscheidet damit, wem ihre
//! Tasten gehoeren. **Ein Etikett ist keine `NSTextView`, und die Frage stellt
//! sich deshalb gar nicht** — es nimmt den Ersthelferrang nicht an, kein
//! Tastendruck landet in ihm, und es steht folglich nicht bei
//! `ist_eigene_textflaeche`. Das steht hier, damit der naechste Leser nicht
//! danach sucht.
//!
//! Der Preis ist benannt: der Text der Einzelheiten laesst sich nicht
//! markieren und nicht kopieren. E13 verlangt eine Flaeche, die die vier
//! Angaben **zeigt**, und mehr nicht.
//!
//! # Die Auswahl wohnt in der Liste
//!
//! Der Git-Bereich haelt die Stelle, die in seiner Liste blau steht, selbst —
//! [`GitfensterIvars::auswahl`] —, und schreibt sie **nicht** in das
//! [`Gitmodell`] zurueck. Der Grund ist der Zuschnitt dieses Schritts:
//! [`Gitfenster::zeigen`] bekommt das Modell lesend, und `Tabinhalt::gitmodell`
//! sagt ausdruecklich, dass ein Schreiber von aussen eine zweite Quelle fuer
//! denselben Stand waere. Die Folge ist benannt und angenommen: die Auswahl
//! ueberlebt keinen Tabwechsel, weil ein Gitfenster fuer alle Tabs steht,
//! waehrend das Gitmodell je Tab eines ist. Ob sie das soll, ist eine
//! Nutzerfrage und als Datensatz gefilt.
//!
//! **Sie ueberlebt dafuer keinen Ordnerwechsel und keinen Tabwechsel
//! unbemerkt.** [`Gitfenster::zeigen`] behaelt sie nur, wenn die Zeile an
//! ihrer Stelle **wortgleich** dieselbe geblieben ist; die Zeile traegt den
//! Kurzhash (A5), also heisst „derselbe Text an derselben Stelle" hier
//! „derselbe Commit". Ein nachgeladener Schwung haengt hinten an und laesst
//! die vorderen Zeilen stehen, die Auswahl bleibt; ein neuer Ordner ersetzt
//! sie, die Auswahl faellt. Die Frage ist damit aus den Eingaben entscheidbar,
//! die `zeigen` hat, und braucht keine zweite Meldung.
//!
//! # Der Rueckweg ist ein Melder und keine Kenntnis der Tabliste
//!
//! [`Gitfenster::kommando_ausfuehren`] bewegt bei `AuswahlHoch` und
//! `AuswahlRunter` die Auswahl der Liste. Steht sie schon auf dem letzten
//! Eintrag, bewegt ein `down` nichts und meldet stattdessen ueber den
//! [`Nachlademelder`] nach oben; der Anwendungsdelegierte macht daraus einen
//! Lauf mit `Gitfrage::WeitererVerlauf` (C4.2). Damit bleibt dieser Bereich so
//! unwissend ueber die Tabliste, wie die Lesezeichenleiste es ueber die
//! Dateifenster ist, und der Rueckruf haelt seinen Halter **schwach**, wie die
//! sechs vorhandenen Melder.
//!
//! **Ist der Verlauf erschoepft, meldet nichts** (C4.3): [`Gitfenster::zeigen`]
//! traegt mit, ob der letzte Lauf weniger als fuenfzig geliefert hat, und der
//! Melder feuert dann nicht. Ein Lauf, der eine leere Liste zurueckbraechte,
//! entsteht so gar nicht erst.
//!
//! # Warum die Liste keine eigene Tastenbehandlung hat
//!
//! Wie die Lesezeichenleiste: keine `keyDown:`-Methode und kein eigenes
//! Kuerzel. Jeder Tastendruck laeuft durch den einen Ereignisabgriff aus
//! [`super::ereignisse`], wird im Kern nachgeschlagen und kommt als
//! [`Kommando`] hier an. Eine Ansicht, die eine Taste selbst abfinge, waere die
//! Sonderregel mit eigenem Rueckfallweg, die die Maxime „supersimpel"
//! ausschliesst; und die Auswahl der `NSTableView` bewegte sich dann zweimal,
//! einmal durch AppKit und einmal durch KRK.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSView`, `NSScrollView`, `NSTableView`, `NSTableColumn`, `NSTextField`,
//! `NSFont`, `NSColor`, `NSIndexSet`, `NSNotification`, `NSObject` und
//! `NSString` stehen seit macOS 10.0 zur Verfuegung, ebenso die drei bedienten
//! Protokolle `NSTableViewDataSource`, `NSTableViewDelegate` und
//! `NSControlTextEditingDelegate` und die Aufzaehlungen
//! `NSAutoresizingMaskOptions`, `NSTableColumnResizingOptions`, `NSLineBreakMode`
//! (`NSParagraphStyle.h:25`) und `NSTextAlignment`. Ohne eigene
//! Verfuegbarkeitsangabe und damit seit 10.0 stehen die hier gerufenen
//! Methoden `initWithFrame:`, `addSubview:`, `setFrame:`, `frame`,
//! `setAutoresizingMask:`, `resizeSubviewsWithOldSize:` (`NSView.h:122`),
//! `isFlipped` (`NSView.h:236`), `window`, `makeFirstResponder:`,
//! `setDocumentView:`, `contentSize` (`NSScrollView.h:47`),
//! `setHasVerticalScroller:`, `setAutohidesScrollers:`, `setRowHeight:`
//! (`NSTableView.h:206`), `setHeaderView:` (`:156`), `addTableColumn:` (`:226`),
//! `reloadData` (`:256`), `scrollRowToVisible:` (`:250`),
//! `selectRowIndexes:byExtendingSelection:` (`:353`), `selectedRow` (`:361`),
//! `allowsEmptySelection` (`:330`), `allowsMultipleSelection` (`:326`),
//! `setResizingMask:`, `setDataSource:`, `setDelegate:`, `setStringValue:`,
//! `setFont:`, `setTextColor:`, `setAlignment:` (`NSControl.h:66`), `sizeToFit`
//! (`NSControl.h:44`), `smallSystemFontSize`, `systemFontOfSize:` und
//! `indexSetWithIndex:`. Das Buendel zielt auf 15.0 (`.cargo/config.toml`).
//!
//! **Zehn Beruehrungen sind juenger als ihre Klasse, und alle liegen unter dem
//! Zielsystem:**
//!
//! - `preferredMaxLayoutWidth` seit 10.8 (`NSTextField.h:45`)
//! - `usesSingleLineMode` und `lineBreakMode` seit 10.10
//!   (`NSControl.h:62` und `:65`)
//! - `labelColor` und `secondaryLabelColor` seit 10.10
//!   (`NSColor.h:201` und `:202`)
//! - `NSFontWeightRegular` und `monospacedDigitSystemFontOfSize:weight:` seit
//!   10.11 (`NSFontDescriptor.h:170`, `NSFont.h:62`). Dieselben beiden Stellen
//!   nennt die SAFETY-Begruendung an der Lesestelle des Fremdsymbols; sie
//!   bleibt dort, weil sie den `unsafe`-Block traegt.
//! - `maximumNumberOfLines` seit 10.11 (`NSTextField.h:49`)
//! - `NSTextField::labelWithString:` seit 10.12 (`NSTextField.h:93`)
//! - `setUsesAutomaticRowHeights:` seit 10.13 (`NSTableView.h:574`)
//! - `tableView:viewForTableColumn:row:` seit 10.7 (`NSTableView.h:593`)
//! - `NSTableViewStyle` samt `setStyle:` seit 11.0 (`NSTableView.h:77`
//!   und `:377`) — die hoechste Untergrenze dieser Datei
//!
//! Keine von ihnen ist nach macOS 15 hinzugekommen, und keine Beruehrung in
//! dieser Datei braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit.
//! `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer
//! haelt die Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.

use std::cell::{Cell, RefCell};

use objc2::rc::{Retained, Weak};
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSColor, NSControlTextEditingDelegate, NSFont, NSFontWeightRegular,
    NSLineBreakMode, NSScrollView, NSTableColumn, NSTableColumnResizingOptions, NSTableView,
    NSTableViewDataSource, NSTableViewDelegate, NSTableViewStyle, NSTextAlignment, NSTextField,
    NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSIndexSet, NSInteger, NSNotification, NSObject, NSObjectProtocol, NSPoint,
    NSRect, NSSize, NSString, ns_string,
};

use krk_core::tasten::Kommando;

use crate::gitmodell::Gitmodell;

/// Die Groesse, mit der die Ansichten entstehen, bevor die Aufteilung sie
/// auslegt.
///
/// Die Breite ist die Anfangsbreite des Bereichs aus `Bereich::anfangsbreite`;
/// gehalten wird sie dort und nicht hier, diese Zahl ist allein der Rahmen des
/// Aufbaus.
const AUFBAUGROESSE: NSSize = NSSize::new(420.0, 400.0);

/// Die Hoehe des Kopfes: zwei Zeilen und der Rand darum.
const KOPFHOEHE: f64 = 38.0;

/// Die Hoehe der Flaeche der Einzelheiten.
///
/// Fest und nicht am Inhalt gemessen: eine Flaeche, die mit der Laenge der
/// Commit-Nachricht waechst, verschoebe bei jedem Wechsel der Auswahl die
/// Liste darueber. Was nicht hineinpasst, wird gerollt.
const EINZELHEITENHOEHE: f64 = 120.0;

/// Der Rand um Kopf und Einzelheiten.
const RAND: f64 = 6.0;

/// Die Hoehe einer Zeile der Verlaufsliste in Punkten.
///
/// Dieselbe wie in der Dateiliste und in der Lesezeichenleiste, und aus
/// demselben Grund fest: eine Liste mit gleich hohen Zeilen laesst AppKit die
/// Gesamthoehe rechnen, statt jede Zeile zu messen.
const ZEILENHOEHE: f64 = 20.0;

/// Der Einzug einer Zeile der Verlaufsliste gegenueber dem Rand der Spalte.
const ZEILENEINZUG: f64 = 4.0;

/// Die Breite, mit der eine Zelle entsteht, bevor die Tabelle sie auslegt.
///
/// Wie in [`super::leiste`]: der Wert selbst ist gleichgueltig, das
/// Verhaeltnis in ihm nicht. Eine Beschriftung mit fester linker Kante und
/// beweglicher Breite behaelt beim Auslegen ihren rechten Abstand, und der ist
/// genau dann null, wenn sie hier bis an den rechten Rand der Zelle reicht.
const AUFBAUBREITE: f64 = 400.0;

/// Was der Git-Bereich seinem Halter meldet.
///
/// Genau ein Ereignis: die Auswahl steht am Ende der Liste, und der Nutzer
/// will weiter nach unten. Was daraus folgt, entscheidet der
/// Anwendungsdelegierte — er kennt das aktive Dateifenster, der Git-Bereich
/// kennt es nicht.
pub type Nachlademelder = Box<dyn Fn()>;

/// Eine Zeile des Verlaufs, wie der Bereich sie haelt.
///
/// **Beide Texte kommen fertig aus [`krk_core::git::texte`]** und werden hier
/// nicht geformt. Sie stehen nebeneinander, weil die Liste den einen und die
/// Flaeche darunter den anderen zeigt und weil
/// [`Gitfenster::kommando_ausfuehren`] die Auswahl bewegt, ohne das Modell in
/// der Hand zu haben.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Verlaufszeile {
    /// Die einzeilige Form fuer die Liste (A5).
    zeile: String,
    /// Die volle Form fuer die Flaeche darunter (E13).
    einzelheiten: String,
}

define_class!(
    /// Die Traegeransicht des Bereichs, die in die Aufteilung gehaengt wird.
    ///
    /// Sie ist eine eigene Klasse aus **einem** Grund: sie meldet ihre
    /// Groessenaenderung. Die Flaeche der Einzelheiten traegt ein umbrechendes
    /// Etikett, dessen Hoehe von seiner Breite abhaengt, und ohne diese Meldung
    /// bliebe die Hoehe auf dem Stand des letzten [`Gitfenster::zeigen`]
    /// stehen — beim Schmalerziehen des Fensters waeren die untersten Zeilen
    /// abgeschnitten.
    ///
    /// Die Rueckverbindung ist **schwach**, sonst schloesse sich der Ring
    /// Gitfenster → Traegeransicht → Rueckverweis → Gitfenster. Dieselbe
    /// Bauart wie `Inhaltsflaeche` in [`super::vorschau`].
    // SAFETY:
    // - Die Oberklasse NSView stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = RefCell<Option<Weak<Gitfenster>>>]
    pub struct Gitsicht;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Gitsicht {}

    impl Gitsicht {
        /// AppKit legt die drei Flaechen neu aus.
        ///
        /// Erst die Oberklasse, die die Autoresizing-Masken anwendet, dann das
        /// Einpassen des Etiketts in seine neue Breite.
        // SAFETY: Die Signatur entspricht der von NSView (`NSView.h:122`).
        #[unsafe(method(resizeSubviewsWithOldSize:))]
        fn unteransichten_auslegen(&self, alte_groesse: NSSize) {
            // SAFETY: `resizeSubviewsWithOldSize:` von NSView hat die hier
            // angenommene Signatur.
            unsafe {
                let _: () = msg_send![super(self), resizeSubviewsWithOldSize: alte_groesse];
            }
            if let Some(fenster) = self.gitfenster() {
                fenster.einzelheiten_einpassen();
            }
        }
    }
);

impl Gitsicht {
    /// Eine Traegeransicht ohne Rueckverweis; den setzt [`Gitfenster::bauen`]
    /// nach, sobald es das Objekt gibt.
    fn neu(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(RefCell::new(None));
        // SAFETY: `initWithFrame:` von NSView hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), initWithFrame: rahmen] }
    }

    /// Traegt den schwachen Rueckverweis ein.
    fn ziel_setzen(&self, fenster: &Gitfenster) {
        *self.ivars().borrow_mut() = Some(Weak::from_retained(&fenster.retain()));
    }

    /// Das Gitfenster, solange es lebt.
    fn gitfenster(&self) -> Option<Retained<Gitfenster>> {
        self.ivars().borrow().as_ref().and_then(Weak::load)
    }
}

define_class!(
    /// Der Inhalt der Rolle unter der Liste: eine **umgedrehte** Flaeche.
    ///
    /// Sie traegt nichts als das Etikett und hat genau eine Aufgabe:
    /// `isFlipped` zu bejahen. Eine `NSScrollView` zeigt den Ursprung ihres
    /// Inhalts, und der liegt bei einer gewoehnlichen Ansicht **unten**; ohne
    /// diese Klasse begaenne die Anzeige einer langen Commit-Nachricht an ihrem
    /// Ende. Das Etikett selbst kann die Antwort nicht geben: `NSTextField`
    /// legt seinen Text daran aus, und eine umgedrehte Textzelle zeichnete ihn
    /// verkehrt.
    // SAFETY:
    // - Die Oberklasse NSView stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = ()]
    pub struct Einzelheitenflaeche;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Einzelheitenflaeche {}

    impl Einzelheitenflaeche {
        /// Der Ursprung liegt oben links.
        // SAFETY: Die Signatur entspricht der Eigenschaft von NSView
        // (`NSView.h:236`).
        #[unsafe(method(isFlipped))]
        fn ist_umgedreht(&self) -> bool {
            true
        }
    }
);

impl Einzelheitenflaeche {
    /// Eine umgedrehte Flaeche in der genannten Groesse.
    fn neu(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: `initWithFrame:` von NSView hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), initWithFrame: rahmen] }
    }
}

/// Was der Git-Bereich haelt.
pub struct GitfensterIvars {
    /// Die Traegeransicht, die in die Aufteilung gehaengt wird.
    bereich: Retained<Gitsicht>,
    /// Der Kopf: Branch oder Kurzhash, darunter die Zusammenfassung (A6, A3).
    kopf: Retained<NSTextField>,
    /// Die Verlaufsliste. Sie ist die Flaeche, die den Ersthelferrang nimmt.
    liste: Retained<NSTableView>,
    /// Die Rolle unter der Liste, deren Inhalt das Etikett traegt.
    einzelheitenrolle: Retained<NSScrollView>,
    /// Das mehrzeilige Etikett mit den Einzelheiten des ausgewaehlten Commits.
    einzelheiten: Retained<NSTextField>,
    /// Die Zeilen des Verlaufs, wie [`Gitfenster::zeigen`] sie zuletzt
    /// uebernommen hat.
    zeilen: RefCell<Vec<Verlaufszeile>>,
    /// Die Stelle, die in der Liste blau steht.
    ///
    /// Der Modulkopf sagt, warum sie hier wohnt und nicht im [`Gitmodell`],
    /// und wie [`Gitfenster::zeigen`] sie gegen einen Ordnerwechsel haelt.
    auswahl: Cell<Option<usize>>,
    /// Ob der Verlauf erschoepft ist und ein Nachschlag nichts mehr braechte
    /// (C4.3).
    ///
    /// Er kommt mit jedem [`Gitfenster::zeigen`] aus dem Modell herein; ihn
    /// hier zu halten erspart dem Melder die Frage nach einem Modell, das er
    /// zum Zeitpunkt des Tastendrucks nicht in der Hand hat.
    erschoepft: Cell<bool>,
    /// Wahr, solange dieses Objekt die Auswahl der Tabelle selbst setzt.
    ///
    /// Ohne dieses Kennzeichen liefe jede Bewegung doppelt: hier wird die
    /// Auswahl gesetzt, AppKit meldet die Aenderung, und die Meldung liefe
    /// wieder hierher. Dieselbe Sperre wie in [`super::leiste`].
    setzt_selbst: Cell<bool>,
    /// Der Melder, den [`Gitfenster::nachlademelder_setzen`] eintraegt.
    nachlademelder: RefCell<Option<Nachlademelder>>,
}

define_class!(
    /// Der Git-Bereich: Datenquelle, Delegierter und Halter der drei Flaechen
    /// in einem Objekt.
    ///
    /// Wie bei der Lesezeichenleiste und anders als beim Dateifenster, wo
    /// Quelle und Delegierter getrennt sind: dort haelt der Delegierte zwei
    /// Formatierer und zwei Schriften fuer fuenf Spalten, hier gibt es eine
    /// Spalte mit einer Beschriftung. Zwei Objekte dafuer waeren zwei Halter
    /// fuer denselben Zustand.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = GitfensterIvars]
    pub struct Gitfenster;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Gitfenster {}

    // SAFETY: `NSTableViewDataSource` stellt keine Bedingungen.
    unsafe impl NSTableViewDataSource for Gitfenster {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(numberOfRowsInTableView:))]
        fn zeilenzahl(&self, _tabelle: &NSTableView) -> NSInteger {
            NSInteger::try_from(self.ivars().zeilen.borrow().len()).unwrap_or(NSInteger::MAX)
        }
    }

    // SAFETY: `NSControlTextEditingDelegate` ist Oberprotokoll von
    // `NSTableViewDelegate` und hat nur wahlfreie Methoden. Der Git-Bereich
    // bearbeitet keinen Text; er erfuellt das Protokoll leer, wie die
    // Lesezeichenleiste auch.
    unsafe impl NSControlTextEditingDelegate for Gitfenster {}

    // SAFETY: `NSTableViewDelegate` stellt keine Bedingungen.
    unsafe impl NSTableViewDelegate for Gitfenster {
        // SAFETY: Die Signatur entspricht der des Protokolls
        // (`NSTableView.h:593`).
        #[unsafe(method_id(tableView:viewForTableColumn:row:))]
        fn ansicht_fuer_zelle(
            &self,
            _tabelle: &NSTableView,
            _spalte: Option<&NSTableColumn>,
            zeile: NSInteger,
        ) -> Option<Retained<NSView>> {
            self.zellenansicht(zeile)
        }

        /// Die Auswahl hat sich geaendert, und zwar durch einen Mausklick.
        ///
        /// Die Tastatur laeuft nicht hierueber, sondern ueber
        /// [`Gitfenster::kommando_ausfuehren`]; beide muenden in
        /// [`Gitfenster::einzelheiten_schreiben`], damit es genau eine Stelle
        /// gibt, die aus einer Auswahl die Flaeche darunter macht.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableViewSelectionDidChange:))]
        fn auswahl_geaendert(&self, _meldung: &NSNotification) {
            if self.ivars().setzt_selbst.get() {
                return;
            }
            let zeile = self.ivars().liste.selectedRow();
            self.ivars().auswahl.set(usize::try_from(zeile).ok());
            self.einzelheiten_schreiben();
        }
    }
);

impl Gitfenster {
    /// Baut den Git-Bereich mit seinen drei leeren Flaechen.
    ///
    /// **Leer und nicht mit einem Platzhalter besetzt** (A8): vor der ersten
    /// Antwort ist nichts beantwortet, und `Kopf::KeinRepository` waere eine
    /// entschiedene Antwort, die hier noch niemand gegeben hat.
    pub fn bauen(mtm: MainThreadMarker) -> Retained<Self> {
        let rahmen = NSRect::new(NSPoint::ZERO, AUFBAUGROESSE);
        let bereich = Gitsicht::neu(mtm, rahmen);
        bereich.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        // Der Kopf oben, festgemacht am oberen Rand.
        let kopf = kopfetikett(mtm, AUFBAUGROESSE.width);
        kopf.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewMinYMargin,
        );
        bereich.addSubview(&kopf);

        // Die Einzelheiten unten, festgemacht am unteren Rand.
        let einzelheitenrahmen = NSRect::new(
            NSPoint::ZERO,
            NSSize::new(AUFBAUGROESSE.width, EINZELHEITENHOEHE),
        );
        let einzelheitenrolle =
            NSScrollView::initWithFrame(NSScrollView::alloc(mtm), einzelheitenrahmen);
        einzelheitenrolle.setHasVerticalScroller(true);
        einzelheitenrolle.setAutohidesScrollers(true);
        einzelheitenrolle.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewMaxYMargin,
        );
        let einzelheitentraeger =
            Einzelheitenflaeche::neu(mtm, NSRect::new(NSPoint::ZERO, einzelheitenrahmen.size));
        einzelheitentraeger.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
        let einzelheiten = einzelheitenetikett(mtm, AUFBAUGROESSE.width);
        einzelheitentraeger.addSubview(&einzelheiten);
        einzelheitenrolle.setDocumentView(Some(&einzelheitentraeger));
        bereich.addSubview(&einzelheitenrolle);

        // Die Liste dazwischen; sie bekommt, was uebrig bleibt.
        let listenhoehe = AUFBAUGROESSE.height - KOPFHOEHE - EINZELHEITENHOEHE;
        let listenrahmen = NSRect::new(
            NSPoint::new(0.0, EINZELHEITENHOEHE),
            NSSize::new(AUFBAUGROESSE.width, listenhoehe),
        );
        let (listenrolle, liste) = verlaufsliste(mtm, listenrahmen);
        bereich.addSubview(&listenrolle);

        kopf.setFrame(NSRect::new(
            NSPoint::new(0.0, AUFBAUGROESSE.height - KOPFHOEHE),
            NSSize::new(AUFBAUGROESSE.width, KOPFHOEHE),
        ));

        let this = Self::alloc(mtm).set_ivars(GitfensterIvars {
            bereich,
            kopf,
            liste,
            einzelheitenrolle,
            einzelheiten,
            zeilen: RefCell::new(Vec::new()),
            auswahl: Cell::new(None),
            erschoepft: Cell::new(false),
            setzt_selbst: Cell::new(false),
            nachlademelder: RefCell::new(None),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };

        // Der Rueckverweis und die beiden Rollen des Tabellendelegierten
        // stehen hier und nicht in der ersten Haelfte: es gibt das Objekt erst
        // ab dem `init`. Dieselbe Reihenfolge wie beim Vorschaufenster.
        this.ivars().bereich.ziel_setzen(&this);
        // SAFETY: Das Objekt beantwortet beide Protokolle, die es oben
        // implementiert. Ueber die Lebensdauer verlangt die Bindung nichts;
        // getragen wird der Aufruf davon, dass `dataSource` und `delegate`
        // nullende schwache Eigenschaften sind ("This is a weak property",
        // `objc2-app-kit-0.3.2/src/generated/NSTableView.rs:402-421`), und dass
        // das Gitfenster die Tabelle selbst festhaelt.
        unsafe {
            this.ivars()
                .liste
                .setDataSource(Some(ProtocolObject::from_ref(&*this)));
            this.ivars()
                .liste
                .setDelegate(Some(ProtocolObject::from_ref(&*this)));
        }
        this
    }

    /// Die Ansicht, die in die Aufteilung gehaengt wird.
    pub fn sicht(&self) -> &NSView {
        &self.ivars().bereich
    }

    /// Die Flaeche, die den Eingabefokus traegt.
    ///
    /// Die Verlaufsliste und nicht die Traegeransicht: sie ist die einzige der
    /// drei Flaechen, in der ein Tastendruck etwas bewegt, und `up` und `down`
    /// sollen dort ankommen, wo die Auswahl steht. Der Kopf und die
    /// Einzelheiten sind Etiketten und nehmen den Ersthelferrang nicht an.
    pub fn fokusansicht(&self) -> &NSView {
        &self.ivars().liste
    }

    /// Traegt den Melder ein, der ein Nachladen des Verlaufs anfordert (C4.2).
    ///
    /// Gerufen vom Anwendungsdelegierten beim Aufbau der Oberflaeche, mit einem
    /// Rueckruf, der sein Ziel schwach haelt. Derselbe Zuschnitt wie
    /// `Vorschaufenster::seitenmelder_setzen`.
    pub fn nachlademelder_setzen(&self, melden: Nachlademelder) {
        *self.ivars().nachlademelder.borrow_mut() = Some(melden);
    }

    /// Schreibt den Stand eines Tabs in die drei Flaechen.
    ///
    /// **Die eine Stelle, die aus einem [`Gitmodell`] Anzeige macht.** Sie
    /// nimmt das Modell lesend: die Auswahl wohnt in dieser Datei, und der
    /// Modulkopf sagt, warum.
    ///
    /// Die Auswahl ueberlebt den Durchgang genau dann, wenn die Zeile an ihrer
    /// Stelle wortgleich dieselbe geblieben ist; der Modulkopf schreibt die
    /// Regel und ihre Begruendung aus.
    pub fn zeigen(&self, modell: &Gitmodell) {
        let ivars = self.ivars();
        ivars
            .kopf
            .setStringValue(&NSString::from_str(&kopftext(modell)));
        ivars.erschoepft.set(modell.erschoepft());

        let neu: Vec<Verlaufszeile> = (0..modell.verlaufslaenge())
            .map(|stelle| Verlaufszeile {
                zeile: modell.verlaufszeile(stelle).unwrap_or_default(),
                einzelheiten: modell.einzelheiten(stelle).unwrap_or_default(),
            })
            .collect();
        let auswahl = ivars
            .auswahl
            .get()
            .filter(|stelle| haelt_die_auswahl(&ivars.zeilen.borrow(), &neu, *stelle));

        *ivars.zeilen.borrow_mut() = neu;
        ivars.auswahl.set(auswahl);
        ivars.liste.reloadData();
        self.auswahl_anzeigen();
        self.einzelheiten_schreiben();
    }

    /// Fuehrt einen der beiden Auswahlbefehle auf der Verlaufsliste aus.
    ///
    /// Alles andere geht zurueck an den Aufrufer: der Git-Bereich fuehrt keinen
    /// weiteren Befehl, und ein hier nicht ausgefuehrtes Kommando laeuft wie
    /// ein unbelegtes weiter.
    ///
    /// **Am unteren Rand bewegt sich nichts und der Nachschlag wird
    /// angefordert** (C4.2): die Auswahl bleibt auf dem Eintrag, auf dem sie
    /// stand, und die Liste springt nicht. Ist der Verlauf erschoepft, meldet
    /// auch dieser Fall nichts (C4.3).
    #[must_use = "ein nicht ausgefuehrtes Kommando laeuft weiter"]
    pub fn kommando_ausfuehren(&self, kommando: Kommando) -> bool {
        let schritt: isize = match kommando {
            Kommando::AuswahlHoch => -1,
            Kommando::AuswahlRunter => 1,
            _ => return false,
        };
        let ivars = self.ivars();
        let laenge = ivars.zeilen.borrow().len();
        match ziel(ivars.auswahl.get(), laenge, schritt) {
            Some(stelle) => {
                ivars.auswahl.set(Some(stelle));
                self.auswahl_anzeigen();
                self.einzelheiten_schreiben();
            }
            // Am Rand bewegt sich nichts, und der Tastendruck ist trotzdem
            // verbraucht: sonst raeumte AppKit ihn an die `NSTableView` weiter,
            // die daraufhin ihre eigene Auswahl bewegte. Dieselbe Ueberlegung
            // wie in `Leistenquelle::kommando_ausfuehren`.
            None => {
                if schritt > 0 {
                    self.nachladen_melden();
                }
            }
        }
        true
    }

    /// Fordert die naechsten Commits an, falls jemand zuhoert (C4.2, C4.3).
    ///
    /// Die Ausleihe steht waehrend des Rufs und ist lesend; der einzige
    /// schreibende Zugriff auf dieselbe Zelle ist
    /// [`Self::nachlademelder_setzen`] beim Aufbau.
    fn nachladen_melden(&self) {
        if self.ivars().erschoepft.get() {
            return;
        }
        let melden = self.ivars().nachlademelder.borrow();
        if let Some(melden) = melden.as_ref() {
            melden();
        }
    }

    /// Setzt die Auswahl der Tabelle auf die gehaltene Stelle.
    ///
    /// Waehrend des Setzens steht das Kennzeichen `setzt_selbst`: AppKit meldet
    /// jede Aenderung, auch die selbst gesetzte, und ohne das Kennzeichen liefe
    /// die Meldung hierher zurueck.
    fn auswahl_anzeigen(&self) {
        let ivars = self.ivars();
        ivars.setzt_selbst.set(true);
        match ivars.auswahl.get() {
            Some(zeile) => {
                let stelle = NSIndexSet::indexSetWithIndex(zeile);
                ivars
                    .liste
                    .selectRowIndexes_byExtendingSelection(&stelle, false);
                if let Ok(zeile) = NSInteger::try_from(zeile) {
                    ivars.liste.scrollRowToVisible(zeile);
                }
            }
            // SAFETY: `deselectAll:` nimmt einen beliebigen Absender; `None`
            // heisst, dass kein Steuerelement den Aufruf ausgeloest hat. Die
            // Bindung ist unsicher, weil der Absender ein `AnyObject` ist.
            None => unsafe { ivars.liste.deselectAll(None) },
        }
        ivars.setzt_selbst.set(false);
    }

    /// Schreibt die Einzelheiten des ausgewaehlten Commits in ihre Flaeche.
    ///
    /// **Ohne Auswahl bleibt sie leer, und es steht kein Platzhaltertext**
    /// (C3.5). Der leere Text ist hier kein Sonderfall mit eigenem Zweig,
    /// sondern das, was `None` liefert.
    fn einzelheiten_schreiben(&self) {
        let ivars = self.ivars();
        let text = ivars
            .auswahl
            .get()
            .and_then(|stelle| {
                ivars
                    .zeilen
                    .borrow()
                    .get(stelle)
                    .map(|zeile| zeile.einzelheiten.clone())
            })
            .unwrap_or_default();
        ivars
            .einzelheiten
            .setStringValue(&NSString::from_str(&text));
        self.einzelheiten_einpassen();
    }

    /// Passt das Etikett der Einzelheiten in die Breite seiner Rolle ein.
    ///
    /// Zwei Aufrufer, und beide sind ein Anlass, an dem sich die gebrauchte
    /// Hoehe aendert: ein neuer Text ([`Self::einzelheiten_schreiben`]) und
    /// eine neue Breite ([`Gitsicht`]). Der Traeger nimmt danach die Hoehe des
    /// Etiketts, damit die Rolle weiss, wie weit sie rollen kann.
    fn einzelheiten_einpassen(&self) {
        let ivars = self.ivars();
        let breite = ivars.einzelheitenrolle.contentSize().width;
        if breite <= 0.0 {
            return;
        }
        let innen = (breite - 2.0 * RAND).max(1.0);
        let etikett = &ivars.einzelheiten;
        etikett.setPreferredMaxLayoutWidth(innen);
        etikett.setFrame(NSRect::new(
            NSPoint::new(RAND, RAND),
            NSSize::new(innen, etikett.frame().size.height),
        ));
        etikett.sizeToFit();
        let hoehe = etikett.frame().size.height;
        etikett.setFrame(NSRect::new(
            NSPoint::new(RAND, RAND),
            NSSize::new(innen, hoehe),
        ));

        let Some(traeger) = ivars.einzelheitenrolle.documentView() else {
            return;
        };
        let gebraucht = hoehe + 2.0 * RAND;
        traeger.setFrame(NSRect::new(
            NSPoint::ZERO,
            NSSize::new(
                breite,
                gebraucht.max(ivars.einzelheitenrolle.contentSize().height),
            ),
        ));
    }

    /// Die beschriftete Ansicht fuer eine Zelle der Verlaufsliste.
    ///
    /// **Ein Text und nicht vier Spalten** (A5): die vier Angaben stehen in
    /// einer Zelle, weil vier `NSTableColumn` vier Breiten waeren, die bei der
    /// Mindestbreite des Bereichs gegeneinander liefen. Geformt wird die Zeile
    /// im Kern; hier steht kein Wortlaut und kein Trennzeichen.
    fn zellenansicht(&self, zeile: NSInteger) -> Option<Retained<NSView>> {
        let mtm = self.mtm();
        let stelle = usize::try_from(zeile).ok()?;
        let text = self.ivars().zeilen.borrow().get(stelle)?.zeile.clone();

        let beschriftung = NSTextField::labelWithString(&NSString::from_str(&text), mtm);
        // Festbreite Ziffern bei proportionalen Buchstaben, dieselbe Wahl wie
        // in der Dateiliste und in der Lesezeichenleiste: die Zeile traegt ein
        // Datum und einen Kurzhash, und untereinander gelesen ergeben die in
        // der Proportionalschrift keine Spalte.
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
        // Die Kurzbeschreibung steht vorn und bekommt den Platz, der uebrig
        // bleibt; abgeschnitten wird deshalb am Ende und nicht in der Mitte.
        beschriftung.setLineBreakMode(NSLineBreakMode::ByTruncatingTail);
        beschriftung.setFrame(NSRect::new(
            NSPoint::new(ZEILENEINZUG, 0.0),
            NSSize::new(AUFBAUBREITE - 2.0 * ZEILENEINZUG, ZEILENHOEHE),
        ));
        beschriftung.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);

        let zelle = NSView::initWithFrame(
            NSView::alloc(mtm),
            NSRect::new(NSPoint::ZERO, NSSize::new(AUFBAUBREITE, ZEILENHOEHE)),
        );
        zelle.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
        zelle.addSubview(&beschriftung);
        Some(zelle)
    }
}

/// Der zweizeilige Text des Kopfes (A6, A3, A7, A8).
///
/// Zeile eins ist der Branchname, der Kurzhash mit dem Wort „abgelöst" oder
/// der Satz aus A14; Zeile zwei die Zusammenfassung oder „noch kein Commit".
/// Beide kommen fertig aus dem Kern.
///
/// **Die vier Faelle sind vollstaendig und ueberschneidungsfrei** und heissen
/// alle dasselbe: was nichts sagt, wird weggelassen. Ein leerer Text ohne
/// Zweig truege einen fuehrenden oder abschliessenden Umbruch, und der Kopf
/// zeigte eine leere Zeile, wo A8 nichts verlangt.
#[must_use = "der Kopftext ist die Anzeige und keine Nebenwirkung"]
fn kopftext(modell: &Gitmodell) -> String {
    let oben = modell.kopfzeile();
    let unten = modell.zusammenfassung();
    match (oben.is_empty(), unten.is_empty()) {
        (true, true) => String::new(),
        (false, true) => oben,
        (true, false) => unten.to_owned(),
        (false, false) => format!("{oben}\n{unten}"),
    }
}

/// Wohin die Auswahl ein Schritt bringt, oder `None`, wenn sie stehen bleibt.
///
/// **`None` heisst „kein Zug" und nicht „keine Auswahl".** Die Auswahl wird von
/// hier aus nie geraeumt: ein Pfeil, der sie wegnaehme, liesse die Flaeche der
/// Einzelheiten mitten in der Bewegung leer werden.
///
/// Ohne Auswahl faengt `down` oben an und `up` unten — die Zeile, die in
/// Laufrichtung als erste erreicht wird. In einer leeren Liste bewegt sich
/// nichts.
#[must_use = "der Rueckgabewert ist die neue Auswahl"]
fn ziel(auswahl: Option<usize>, laenge: usize, schritt: isize) -> Option<usize> {
    if laenge == 0 {
        return None;
    }
    let Some(stelle) = auswahl else {
        return match schritt > 0 {
            true => Some(0),
            false => Some(laenge - 1),
        };
    };
    let neu = isize::try_from(stelle).ok()?.checked_add(schritt)?;
    let neu = usize::try_from(neu).ok()?;
    match neu < laenge {
        true => Some(neu),
        false => None,
    }
}

/// Ob die Auswahl an dieser Stelle den Wechsel des Bestands ueberlebt.
///
/// Sie ueberlebt genau dann, wenn dort **wortgleich** dieselbe Zeile steht.
/// Die Zeile traegt den Kurzhash (A5), also heisst das „derselbe Commit". Ein
/// nachgeladener Schwung haengt hinten an und laesst die vorderen Zeilen
/// stehen; ein neuer Ordner ersetzt sie.
#[must_use = "die Antwort entscheidet ueber die Auswahl"]
fn haelt_die_auswahl(alt: &[Verlaufszeile], neu: &[Verlaufszeile], stelle: usize) -> bool {
    match (alt.get(stelle), neu.get(stelle)) {
        (Some(alt), Some(neu)) => alt.zeile == neu.zeile,
        _ => false,
    }
}

/// Das Etikett des Kopfes: zwei Zeilen, linksbuendig, in der Grundfarbe.
fn kopfetikett(mtm: MainThreadMarker, breite: f64) -> Retained<NSTextField> {
    let etikett = NSTextField::labelWithString(ns_string!(""), mtm);
    etikett.setFont(Some(&NSFont::systemFontOfSize(
        NSFont::smallSystemFontSize(),
    )));
    etikett.setTextColor(Some(&NSColor::labelColor()));
    etikett.setAlignment(NSTextAlignment::Left);
    etikett.setUsesSingleLineMode(false);
    etikett.setMaximumNumberOfLines(2);
    etikett.setLineBreakMode(NSLineBreakMode::ByTruncatingTail);
    etikett.setFrame(NSRect::new(
        NSPoint::new(RAND, 0.0),
        NSSize::new((breite - 2.0 * RAND).max(1.0), KOPFHOEHE),
    ));
    etikett
}

/// Das Etikett der Einzelheiten: beliebig viele Zeilen, umbrechend, gedaempft.
fn einzelheitenetikett(mtm: MainThreadMarker, breite: f64) -> Retained<NSTextField> {
    let etikett = NSTextField::labelWithString(ns_string!(""), mtm);
    etikett.setFont(Some(&NSFont::systemFontOfSize(
        NSFont::smallSystemFontSize(),
    )));
    etikett.setTextColor(Some(&NSColor::secondaryLabelColor()));
    etikett.setAlignment(NSTextAlignment::Left);
    etikett.setUsesSingleLineMode(false);
    // Null heisst „so viele wie noetig"; die Hoehe der Rolle deckelt, was zu
    // sehen ist, und der Rest wird gerollt.
    etikett.setMaximumNumberOfLines(0);
    etikett.setLineBreakMode(NSLineBreakMode::ByWordWrapping);
    let innen = (breite - 2.0 * RAND).max(1.0);
    etikett.setPreferredMaxLayoutWidth(innen);
    etikett.setFrame(NSRect::new(
        NSPoint::new(RAND, RAND),
        NSSize::new(innen, EINZELHEITENHOEHE - 2.0 * RAND),
    ));
    etikett
}

/// Baut die Verlaufsliste: eine `NSTableView` mit einer Spalte, ohne
/// Kopfzeile, in einer `NSScrollView`.
///
/// Dieselbe Bauform wie die Lesezeichenleiste, bis auf den Stil: die Leiste ist
/// eine Seitenleiste (`SourceList`), diese Liste steht in einem Bereich der
/// Fensterzeile und traegt deshalb `FullWidth`.
fn verlaufsliste(
    mtm: MainThreadMarker,
    rahmen: NSRect,
) -> (Retained<NSScrollView>, Retained<NSTableView>) {
    let liste = NSTableView::initWithFrame(NSTableView::alloc(mtm), rahmen);
    liste.setRowHeight(ZEILENHOEHE);
    liste.setUsesAutomaticRowHeights(false);
    liste.setStyle(NSTableViewStyle::FullWidth);
    liste.setHeaderView(None);
    liste.setAllowsEmptySelection(true);
    liste.setAllowsMultipleSelection(false);

    let spalte =
        NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), ns_string!("verlauf"));
    spalte.setResizingMask(NSTableColumnResizingOptions::AutoresizingMask);
    liste.addTableColumn(&spalte);

    let rolle = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), rahmen);
    rolle.setHasVerticalScroller(true);
    rolle.setAutohidesScrollers(true);
    rolle.setDocumentView(Some(&liste));
    rolle.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
    );

    (rolle, liste)
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use krk_core::git::{Commit, Kopf, Marke, ObjectId, texte};

    use super::*;

    /// Ein Commit, dessen Objektname aus der Nummer entsteht.
    fn commit(nummer: u8) -> Commit {
        let hex = format!("{nummer:02x}").repeat(20);
        Commit {
            id: ObjectId::from_hex(hex.as_bytes()).expect("vierzig Hexziffern sind ein Objektname"),
            kurzbeschreibung: format!("Commit {nummer}"),
            nachricht: format!("Commit {nummer}\n\nmehr dazu"),
            autor: "Wer".to_owned(),
            email: "wer@example.invalid".to_owned(),
            zeit: SystemTime::UNIX_EPOCH,
        }
    }

    /// Die Zeilen, wie [`Gitfenster::zeigen`] sie aus einem Modell nimmt.
    fn zeilen_aus(modell: &Gitmodell) -> Vec<Verlaufszeile> {
        (0..modell.verlaufslaenge())
            .map(|stelle| Verlaufszeile {
                zeile: modell.verlaufszeile(stelle).unwrap_or_default(),
                einzelheiten: modell.einzelheiten(stelle).unwrap_or_default(),
            })
            .collect()
    }

    /// A8, C4.4: vor der ersten Antwort steht im Kopf nichts, kein
    /// Platzhaltertext und keine leere Zeile.
    #[test]
    fn ein_frisches_modell_traegt_einen_leeren_kopf() {
        assert_eq!(kopftext(&Gitmodell::neu()), "");
    }

    /// C3.1, C3.2: steht beides, stehen sie untereinander in dieser
    /// Reihenfolge.
    #[test]
    fn der_kopf_traegt_die_zeile_oben_und_die_zusammenfassung_darunter() {
        let mut modell = Gitmodell::neu();
        modell.kopf_setzen(Kopf::Branch("main".to_owned()));
        modell.marken_setzen(&[("a.txt".to_owned(), Marke::Geaendert)]);
        let text = kopftext(&modell);
        let zeilen: Vec<&str> = text.lines().collect();
        assert_eq!(zeilen.len(), 2, "der Kopf traegt nicht zwei Zeilen: {text}");
        assert_eq!(zeilen[0], "main");
        assert_eq!(
            zeilen[1],
            texte::zusammenfassung(&[("a.txt".to_owned(), Marke::Geaendert)]),
            "die zweite Zeile kommt nicht aus dem Kern"
        );
    }

    /// A8: steht der Branch schon und die Markenmeldung noch aus, traegt der
    /// Kopf **eine** Zeile und keine leere zweite.
    #[test]
    fn ohne_markenmeldung_traegt_der_kopf_genau_eine_zeile() {
        let mut modell = Gitmodell::neu();
        modell.kopf_setzen(Kopf::Branch("main".to_owned()));
        assert_eq!(kopftext(&modell), "main");
    }

    /// C6.1: ein Ordner ohne Repository traegt seinen Satz und sonst nichts.
    #[test]
    fn ohne_repository_steht_der_satz_aus_a14_allein_im_kopf() {
        let mut modell = Gitmodell::neu();
        modell.kopf_setzen(Kopf::KeinRepository);
        assert_eq!(kopftext(&modell), texte::KEIN_REPOSITORY);
    }

    /// Ohne Auswahl faengt `down` oben an und `up` unten.
    #[test]
    fn ohne_auswahl_faengt_der_pfeil_in_seiner_laufrichtung_an() {
        assert_eq!(ziel(None, 3, 1), Some(0));
        assert_eq!(ziel(None, 3, -1), Some(2));
    }

    /// C4.2: am unteren Rand bewegt sich nichts; am oberen ebenso.
    #[test]
    fn an_beiden_raendern_bewegt_sich_nichts() {
        assert_eq!(ziel(Some(2), 3, 1), None, "unten bewegt sich etwas");
        assert_eq!(ziel(Some(0), 3, -1), None, "oben bewegt sich etwas");
        assert_eq!(ziel(Some(1), 3, 1), Some(2));
        assert_eq!(ziel(Some(1), 3, -1), Some(0));
    }

    /// In einer leeren Liste bewegt sich nichts, in keine Richtung.
    #[test]
    fn eine_leere_liste_bewegt_sich_nicht() {
        assert_eq!(ziel(None, 0, 1), None);
        assert_eq!(ziel(None, 0, -1), None);
        assert_eq!(ziel(Some(0), 0, 1), None);
    }

    /// C4.2: ein nachgeladener Schwung haengt hinten an, die Auswahl bleibt
    /// auf dem Eintrag, auf dem sie stand.
    #[test]
    fn ein_nachschlag_laesst_die_auswahl_stehen() {
        let mut modell = Gitmodell::neu();
        modell.verlauf_anhaengen(vec![commit(1), commit(2)]);
        let alt = zeilen_aus(&modell);
        modell.verlauf_anhaengen(vec![commit(3)]);
        let neu = zeilen_aus(&modell);
        assert!(haelt_die_auswahl(&alt, &neu, 1));
    }

    /// C4.6, C3.5: ein anderer Ordner nimmt die Auswahl mit, auch wenn er
    /// gleich viele Commits hat.
    #[test]
    fn ein_anderer_ordner_nimmt_die_auswahl_mit() {
        let mut alter = Gitmodell::neu();
        alter.verlauf_anhaengen(vec![commit(1), commit(2)]);
        let mut neuer = Gitmodell::neu();
        neuer.verlauf_anhaengen(vec![commit(7), commit(8)]);
        let alt = zeilen_aus(&alter);
        let neu = zeilen_aus(&neuer);
        assert_eq!(
            alt.len(),
            neu.len(),
            "die Probe misst die Laenge und nicht den Inhalt"
        );
        assert!(!haelt_die_auswahl(&alt, &neu, 0));
        assert!(!haelt_die_auswahl(&alt, &neu, 1));
    }

    /// C4.6: ein leerer Verlauf laesst keine Auswahl stehen.
    #[test]
    fn ein_zurueckgesetzter_verlauf_laesst_keine_auswahl_stehen() {
        let mut modell = Gitmodell::neu();
        modell.verlauf_anhaengen(vec![commit(1), commit(2)]);
        let alt = zeilen_aus(&modell);
        modell.zuruecksetzen();
        let neu = zeilen_aus(&modell);
        assert!(!haelt_die_auswahl(&alt, &neu, 0));
        assert!(!haelt_die_auswahl(&alt, &neu, 1));
    }

    /// E13, C3.4: die Flaeche der Einzelheiten bekommt ihren Text aus dem
    /// Kern, je Zeile den ihren.
    #[test]
    fn jede_zeile_traegt_die_einzelheiten_ihres_commits() {
        let mut modell = Gitmodell::neu();
        modell.verlauf_anhaengen(vec![commit(1), commit(2)]);
        let zeilen = zeilen_aus(&modell);
        assert_eq!(zeilen[0].einzelheiten, texte::einzelheiten(&commit(1)));
        assert_eq!(zeilen[1].einzelheiten, texte::einzelheiten(&commit(2)));
        assert_ne!(
            zeilen[0].einzelheiten, zeilen[0].zeile,
            "die Flaeche darunter zeigt dasselbe wie die Zeile"
        );
    }
}
