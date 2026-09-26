//! Die Eintragsansicht: `tasks.txt` aus dem erkannten `~/krkhome/` als Tabelle
//! in der Formatansicht des Editors (C6 des Arbeitspakets
//! `260925-2356-f2-oeffnet-krkhome-statt-notizfenster`).
//!
//! ```text
//! ┌──────────────────────────────┐
//! │ • tasks.txt                  │  der Kopf gehoert dem Editor
//! ├──────────────────────────────┤
//! │ NSScrollView                 │  liegt deckungsgleich ueber der Rolle
//! │  ┌─────────────────────────┐ │  der Textflaeche; eine der beiden ist
//! │  │ ☐ Brot kaufen           │ │  ausgeblendet
//! │  │ ☑ Steuer                │ │
//! │  └─────────────────────────┘ │
//! └──────────────────────────────┘
//! ```
//!
//! # Eine Sicht auf den Stand des Editors und kein eigenes Modell
//!
//! **Die Tabelle haelt keine Eintraege, sondern eine Ableitung.** Ihre Zeilen
//! entstehen aus dem Stand, den das [`crate::editormodell::Editormodell`] haelt,
//! ueber [`aufgabenzeilen`], und das ist die eine Stelle der Ableitung; sie
//! liest den Stand mit `krk_core::heimordner::eintraege::Aufgaben`, derselben
//! Zerlegung, mit der die Handlungen des Kerns rechnen. Gerufen wird sie vom
//! Editorbereich nach jeder Aenderung des Standes, und
//! [`Eintragsansicht::zeilen_zeigen`] laedt die Tabelle nur dann neu, wenn sich
//! die abgeleiteten Zeilen wirklich geaendert haben: der Editorbereich ruft
//! auch beim Tippen in einer ganz anderen Datei, und dann ist nichts zu tun.
//!
//! **Jede Aenderung geht deshalb durch den Editor.** Eine Handlung an der
//! Tabelle rechnet der Kern aus dem alten Stand in einen neuen, und
//! `Editorbereich::umbau_anwenden` schreibt ihn auf demselben Weg ein wie das
//! Ersetzen der Suche: ein Stand, ein Rueckgaengigverwalter, eine
//! Abweichungsmarke. Ein zweiter Stapel fuer die Tabelle entsteht nicht; wie der
//! Verwalter des Fensters auch mit der Tabelle als Ersthelfer derselbe bleibt,
//! steht im Modulkopf von [`super::editor`] unter
//! `rueckgaengigstapel_leeren`.
//!
//! # Zellen, und wer sie beendet
//!
//! Seit Schritt 3.2b des Plans
//! `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`
//! ist das Textfeld jeder Zeile bearbeitbar und das Ankreuzfeld eingeschaltet.
//! **Die Tabelle rechnet dabei nichts selbst.** Was aus einem getippten Text
//! oder einem Klick ins Kaestchen wird, entscheidet der Editorbereich ueber die
//! drei Wege in [`Zellenwege`]; diese Datei meldet allein, welche Zeile es
//! betrifft und was in ihr steht.
//!
//! ```text
//!   Klick daneben, tab, return ──> control:textShouldEndEditing: ──> pruefen
//!                                   │ ja                            (nein: Zelle bleibt)
//!                                   └> controlTextDidEndEditing:  ──> festschreiben
//!   Editorbereich::zelle_uebernehmen ──> bearbeitung_beenden ──┘ (derselbe Weg)
//!   esc (Rang in `abbrechen`)        ──> bearbeitung_verwerfen  ──> Anzeige zurueck
//!   Ankreuzfeld                      ──> abhaken
//! ```
//!
//! **Welche Zelle laeuft, wird im Augenblick der Frage gelesen und nirgends
//! gemerkt.** [`Eintragsansicht::laufende_zelle`] sagt ja, wenn der Ersthelfer
//! ein Feldeditor ist und sein Delegierter unter [`Eintragstabelle`] liegt.
//! `controlTextDidBeginEditing:` kaeme erst mit dem ersten Zeichen (gemessen am
//! 260815 an der Umbenennung in [`super::tabelle`]), und ein gemerktes Feld
//! braeuchte eine zweite Stelle, die es wieder vergisst.
//!
//! **Die Tabelle ist deshalb eine eigene Klasse:** sie ist die Naemlichkeit,
//! an der die laufende Zelle erkannt wird, und die Stelle, an der `copy:`
//! ankommt.
//!
//! # Solange eine Zelle laeuft, aendert nichts von aussen den Stand
//!
//! Zwei Wege, und beide sind gemessen
//! (`messungen/260926-0828-zellen-rueckgaengig.txt`, Belege unter
//! `spikes/zellen-rueckgaengig/`):
//!
//! - **`cmd+z` endet am Anfang der Zelle.** Mit dem Feldeditor von AppKit ging
//!   `NSWindow.undo:` bei leerem Zellenstapel an den Verwalter des Fensters und
//!   nahm den letzten Tabellenumbau zurueck, waehrend die Zelle offen stand
//!   (`issues/260926-0813_*_cmd-z-bei-offener-zelle-…`). Die Zellen bekommen
//!   deshalb einen eigenen Feldeditor, [`Zelleneditor`], ueber
//!   `windowWillReturnFieldEditor:toObject:` beim Fensterdelegierten
//!   (`super::fenster`) und [`Eintragsansicht::feldeditor_fuer`]. **Ein eigener
//!   Verwalter allein haelt es nicht**, auch das ist gemessen: `NSWindow.undo:`
//!   fragt beim Feldeditor nicht dessen `undoManager`. Der Zelleneditor
//!   beantwortet `undo:` und `redo:` deshalb selbst, als erstes Glied der
//!   Antwortkette, und `NSWindow` bekommt sie nicht mehr zu sehen.
//! - **Jedes Neuladen beendet eine offene Zelle vorher, verwerfend.**
//!   `reloadData` unter einer offenen Zelle ruft beide Delegiertenwege mit
//!   Zeile -1, und der getippte Text fiele still. [`Eintragsansicht::zeilen_zeigen`]
//!   verwirft deshalb eine Zelle, die unter einem von aussen gewechselten Stand
//!   steht, bevor es neu laedt: der Stand darunter ist nicht mehr der, gegen
//!   den sie begonnen hat. Ausgenommen ist das eigene Ende der Zelle, dessen
//!   Festschreiben ueber den Umbau selbst hierher kommt; waehrend
//!   `controlTextDidEndEditing:` meldet [`Eintragsansicht::laufende_zelle`] die
//!   Zelle naemlich noch.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSView`, `NSScrollView`, `NSTableView`, `NSTableColumn`, `NSButton`,
//! `NSTextField`, `NSControl`, `NSText`, `NSTextView`, `NSResponder`,
//! `NSWindow`, `NSFont`, `NSIndexSet`, `NSNotification`, `NSObject` und
//! `NSString` stehen seit macOS 10.0 zur Verfuegung, ebenso die vier bedienten
//! Protokolle `NSTableViewDataSource`, `NSTableViewDelegate`,
//! `NSControlTextEditingDelegate` und `NSTextFieldDelegate`
//! (`NSTextField.h:124`, ohne Angabe) samt `NSObjectProtocol`, dem Kistennamen
//! des Protokolls `NSObject` (`objc/NSObject.h`), und die Aufzaehlungen
//! `NSAutoresizingMaskOptions`, `NSTableColumnResizingOptions` und
//! `NSLineBreakMode` (`NSParagraphStyle.h:25`). Ohne eigene
//! Verfuegbarkeitsangabe und damit seit 10.0 stehen die hier gerufenen Methoden
//! `alloc`, `init`, `initWithFrame:`, `addSubview:`, `setFrame:`,
//! `setAutoresizingMask:`, `setHidden:`, `setHasVerticalScroller:`,
//! `setAutohidesScrollers:`, `setDocumentView:`, `setRowHeight:`
//! (`NSTableView.h:206`), `setHeaderView:` (`:156`), `initWithIdentifier:`
//! (`NSTableColumn.h:31`), `setResizingMask:`, `addTableColumn:`
//! (`NSTableView.h:226`), `reloadData` (`:256`), `numberOfRows` (`:222`),
//! `scrollRowToVisible:` (`:250`), `selectRowIndexes:byExtendingSelection:`
//! (`:353`), `selectedRow` (`:361`), `clickedRow`, `setAllowsEmptySelection:`
//! (`:330`), `setAllowsMultipleSelection:` (`:326`), `setDataSource:`,
//! `dataSource`, `setDelegate:`, `setDoubleAction:`,
//! `editColumn:row:withEvent:select:`, `setTarget:`, `setAction:`,
//! `setEnabled:`, `setState:`, `setFont:`, `setEditable:`, `setBordered:`,
//! `setDrawsBackground:`, `stringValue`, `setStringValue:`,
//! `setRefusesFirstResponder:`, `isFieldEditor` (`NSText.h:93`), `string`,
//! `delegate`, `isDescendantOf:`, `firstResponder`, `makeFirstResponder:`,
//! `object`, `systemFontOfSize:`, `smallSystemFontSize`, `systemFontSize` und
//! `indexSetWithIndex:`, dazu die hier **gebauten** Methoden
//! `numberOfRowsInTableView:` (`NSTableView.h:743`),
//! `control:textShouldEndEditing:` und `controlTextDidEndEditing:`
//! (`NSControl.h`) und die Aktion `copy:`. Fuer den [`Zelleneditor`] stehen
//! ebenso seit 10.0 `NSUndoManager` samt `undo`, `redo`, `canUndo`, `canRedo`
//! und `removeAllActions`, `setAllowsUndo:` und `setFieldEditor:`
//! (`NSTextView.h`, `NSText.h:93`) und die **gebauten** Methoden `undoManager`
//! (`NSResponder.h:309`), `undo:`, `redo:` und `becomeFirstResponder`
//! (`NSResponder.h:105`). Die Konstanten
//! `NSControlStateValueOn` und `NSControlStateValueOff` (`NSCell.h:74` und
//! `:73`) tragen keine Angabe. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`).
//!
//! **Diese Beruehrungen sind juenger als ihre Klasse, und alle liegen unter dem
//! Zielsystem:**
//!
//! - `tableView:viewForTableColumn:row:` seit 10.7 (`NSTableView.h:593`)
//! - `NSTableCellView` samt `setTextField:` seit 10.7
//!   (`NSTableCellView.h:23`), `rowForView:` und `columnForView:` seit 10.7
//!   (`NSTableView.h:477` und `:478`), `viewAtColumn:row:makeIfNecessary:`
//!   seit 10.7 (`:472`, allein in einer Probe)
//! - `lineBreakMode` seit 10.10 (`NSControl.h:65`)
//! - `setUsesAutomaticRowHeights:` seit 10.13 (`NSTableView.h:574`)
//! - `NSTextField::textFieldWithString:` und
//!   `NSButton::checkboxWithTitle:target:action:` seit 10.12
//!   (`NSTextField.h:115`, `NSButton.h:59`)
//! - `NSTableViewStyle` samt `setStyle:` seit 11.0 (`NSTableView.h:77` und
//!   `:377`) — die hoechste Untergrenze dieser Datei
//!
//! Keine von ihnen ist nach macOS 15 hinzugekommen, und keine Beruehrung in
//! dieser Datei braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit.
//! `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer
//! haelt die Untergrenze nicht; die Nennung hier ist die Gegenmassnahme. Die
//! Angaben sind aus den Koepfen von [`super::git`] und [`super::bereichsleiste`]
//! uebernommen, die dieselben Namen am SDK nachgelesen haben.
//!
//! **Was die `use`-Zeilen daneben hereinholen, und warum keines davon die
//! Untergrenze dieser Datei anhebt:** `MainThreadMarker` ist ein Rust-Typ der
//! Kiste und hat kein macOS-Alter, `AnyObject` und `Sel` sind Typen der
//! Laufzeit; das Makro `ns_string!` baut die Zeichenkette
//! beim Uebersetzen und hat keines; `NSPoint`, `NSRect` und `NSSize` sind
//! C-Strukturen (`NSGeometry.h:23`, `:33` und `:28`); `NSInteger` ist ein
//! Ganzzahltyp (`objc/NSObjCRuntime.h:13`); alle uebrigen tragen im SDK keine
//! eigene Verfuegbarkeitsangabe und stehen damit seit 10.0.

use std::cell::{Cell, RefCell};

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSButton, NSControl, NSControlStateValueOff, NSControlStateValueOn,
    NSControlTextEditingDelegate, NSFont, NSLineBreakMode, NSResponder, NSScrollView,
    NSTableCellView, NSTableColumn, NSTableColumnResizingOptions, NSTableView,
    NSTableViewDataSource, NSTableViewDelegate, NSTableViewStyle, NSText, NSTextField,
    NSTextFieldDelegate, NSTextView, NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSIndexSet, NSInteger, NSNotification, NSObject, NSObjectProtocol, NSPoint,
    NSRect, NSSize, NSString, NSUndoManager, ns_string,
};

use krk_core::heimordner::eintraege::{Aufgaben, aufgabenzeile};

use super::zwischenablage;

/// Die Hoehe einer Zeile in Punkten.
///
/// Etwas hoeher als die zwanzig der Dateiliste: die Zeile traegt ein
/// Ankreuzfeld in der gewoehnlichen Groesse und Text in der Systemschrift, weil
/// sie gelesen und nicht ueberflogen wird. Fest, weil eine Aufgabe eine Zeile
/// ist (`krk_core::heimordner::eintraege::Abweisung::UmbruchImAufgabentext`).
const ZEILENHOEHE: f64 = 24.0;

/// Der Einzug des Ankreuzfeldes gegenueber dem Rand der Spalte.
const EINZUG: f64 = 4.0;

/// Die Breite, die das Ankreuzfeld samt Abstand zum Text belegt.
const KASTENBREITE: f64 = 22.0;

/// Die Breite, mit der eine Zelle entsteht, bevor die Tabelle sie auslegt.
///
/// Wie in [`super::git`]: der Wert selbst ist gleichgueltig, das Verhaeltnis
/// in ihm nicht. Eine Beschriftung mit fester linker Kante und beweglicher
/// Breite behaelt beim Auslegen ihren rechten Abstand.
const AUFBAUBREITE: f64 = 400.0;

/// Die Spalte des Textes; die Aufgabentabelle hat nur diese eine.
const TEXTSPALTE: NSInteger = 0;

/// Eine Zeile der Aufgabentabelle, abgeleitet aus einem Block des Standes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eintragszeile {
    /// Ob das Kaestchen `[x]` oder `[X]` traegt.
    pub erledigt: bool,
    /// Der Text der Aufgabe, ohne Kaestchen und ohne Zeilenumbruch.
    pub text: String,
}

/// Die Zeilen der Aufgabentabelle zu einem Stand von `tasks.txt`.
///
/// **Die eine Stelle der Ableitung**, und sie zaehlt wie der Kern: eine Zeile
/// je Block aus `Aufgaben::bloecke`, der Vorspann ohne Zeile, eine fremde Zeile
/// als Teil der Aufgabe darueber und ohne eigene Zeile. Damit ist die Stelle
/// einer Zeile dieselbe Zahl, die `Neustand::auswahl` und jede Handlung des
/// Kerns als `index` fuehren.
///
/// Die Kopfzeile eines Blocks ist nach der Zerlegung immer eine Aufgabenzeile;
/// `filter_map` statt eines Abbruchs, weil eine Tabelle mit einer Zeile weniger
/// ein Anzeigefehler waere und ein Absturz des Editors ein Verlust.
#[must_use = "die Zeilen sind die ganze Auskunft"]
pub fn aufgabenzeilen(stand: &str) -> Vec<Eintragszeile> {
    Aufgaben::lesen(stand)
        .bloecke()
        .iter()
        .filter_map(|block| aufgabenzeile(block.kopf()))
        .map(|zeile| Eintragszeile {
            erledigt: zeile.erledigt,
            text: zeile.text.to_owned(),
        })
        .collect()
}

/// Die Zelle, deren Text gerade im Feldeditor steht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zelle {
    /// Die Stelle der Zeile, gezaehlt wie `Neustand::auswahl`.
    pub zeile: usize,
    /// Die Spalte; die Aufgabentabelle hat eine, die Notiztabelle aus Schritt
    /// 4.3 bekommt zwei.
    pub spalte: usize,
}

/// Der Weg der Pruefung: Zeile und getippter Text, zurueck ein Ja oder Nein.
pub type Zellenpruefung = Box<dyn Fn(usize, &str) -> bool>;

/// Der Weg des Festschreibens: Zeile und Text, mit dem die Zelle geendet hat.
pub type Zellenende = Box<dyn Fn(usize, &str)>;

/// Die drei Wege aus der Tabelle in den Editor.
///
/// **Rueckrufe und kein Verweis auf den Editorbereich**, aus zwei Gruenden:
/// die Tabelle soll die Rechnung nicht kennen, und die Proben koennen die
/// Ansicht ohne einen Editorbereich bauen, der sich unter `libtest` nicht
/// bauen laesst (Kopf der Proben in [`super::editor`]). Der Editorbereich traegt
/// sie in `Editorbereich::bauen` ein und haelt sich darin **schwach**, sonst
/// schloesse sich der Ring Editorbereich → Ansicht → Rueckruf → Editorbereich.
pub struct Zellenwege {
    /// Ob die Zelle mit diesem Text enden darf. `false` laesst sie in
    /// Bearbeitung; den Grund meldet der Editor selbst.
    pub pruefen: Zellenpruefung,
    /// Die Zelle ist mit diesem Text zu Ende gegangen.
    pub festschreiben: Zellenende,
    /// Das Ankreuzfeld dieser Zeile ist angeklickt worden.
    pub abhaken: Box<dyn Fn(usize)>,
}

define_class!(
    /// Die Tabelle der Eintragsansicht.
    ///
    /// Eine eigene Klasse, weil sie die Naemlichkeit ist, an der die laufende
    /// Zelle erkannt wird, und weil `copy:` hier ankommt; siehe den Modulkopf.
    // SAFETY:
    // - Die Oberklasse `NSTableView` stellt an eine Unterklasse keine
    //   Bedingung, die diese Klasse verletzt: sie ruft den bezeichneten
    //   Erzeuger `initWithFrame:` der Oberklasse und ist weder ihre eigene
    //   Datenquelle noch ihr eigener Delegierter.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSTableView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = ()]
    pub struct Eintragstabelle;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Eintragstabelle {}

    impl Eintragstabelle {
        /// `cmd+c` und "Bearbeiten › Kopieren" mit der Tabelle als Ersthelfer:
        /// der Text der gewaehlten Aufgabe geht als Text in die
        /// Zwischenablage (C6).
        ///
        /// Gelesen wird aus der Ableitung und nicht aus der Zelle, ueber die
        /// Datenquelle, die die Ableitung haelt. Ohne gewaehlte Zeile bleibt
        /// die Ablage, wie sie war. Die Oberklasse wird nicht gerufen:
        /// `NSTableView` beantwortet `copy:` nicht, und der Weg ginge sonst die
        /// Kette hinauf zum Kopieren der Dateiverweise beim
        /// Anwendungsdelegierten, das hier nichts zu kopieren hat. Die Antwort
        /// der Ablage faellt, wie bei der Textanzeige der Vorschau: die Tabelle
        /// hat keine eigene Meldezeile.
        // SAFETY: Die Signatur ist die einer Aktion: ein optionales
        // Objektargument, keine Rueckgabe.
        #[unsafe(method(copy:))]
        fn kopieren(&self, _absender: Option<&AnyObject>) {
            // SAFETY: Ein Leser ohne Vorbedingung; die Datenquelle ist eine
            // nullende schwache Eigenschaft.
            let quelle = unsafe { self.dataSource() };
            let Some(quelle) = quelle else {
                return;
            };
            let Some(ansicht) =
                AsRef::<AnyObject>::as_ref(&*quelle).downcast_ref::<Eintragsansicht>()
            else {
                return;
            };
            if let Some(text) = ansicht.gewaehlter_text() {
                let _ = zwischenablage::text_schreiben(&text);
            }
        }
    }
);

define_class!(
    /// Der Feldeditor der Eintragszellen, mit eigenem Rueckgaengigverwalter.
    ///
    /// **Einer fuer alle Zellen**, wie der Feldeditor des Fensters, den er fuer
    /// sie ersetzt; die Eintragsansicht haelt ihn, und der Fensterdelegierte
    /// reicht ihn ueber [`Eintragsansicht::feldeditor_fuer`] an AppKit. Warum es
    /// ihn gibt, steht im Modulkopf unter „Solange eine Zelle laeuft".
    ///
    /// **AppKit richtet ihn beim Beginn jeder Bearbeitung selbst ein**, wie den
    /// eigenen: gemessen sind 21 Eigenschaften, darunter Rich Text, die
    /// Automatiken und `allowsUndo`, und alle stehen wie beim Feldeditor von
    /// AppKit. Diese Klasse aendert allein, wohin `cmd+z` geht.
    // SAFETY:
    // - Die Oberklasse NSTextView stellt an eine Unterklasse keine Bedingung,
    //   die diese Klasse verletzt: sie ruft den bezeichneten Erzeuger
    //   `initWithFrame:` der Oberklasse, und `becomeFirstResponder` geht
    //   unveraendert an die Oberklasse.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSTextView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = Retained<NSUndoManager>]
    pub struct Zelleneditor;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Zelleneditor {}

    impl Zelleneditor {
        /// Der eigene Verwalter: in ihn meldet die Oberklasse das Tippen an.
        // SAFETY: Die Signatur entspricht der Eigenschaft von NSResponder
        // (`NSResponder.h:309`).
        #[unsafe(method_id(undoManager))]
        fn verwalter(&self) -> Option<Retained<NSUndoManager>> {
            Some(self.ivars().clone())
        }

        /// `cmd+z` in einer Zelle nimmt Getipptes dieser Zelle zurueck und
        /// sonst nichts.
        ///
        /// **Hier und nicht bei `NSWindow`**, das `undo:` sonst beantwortet:
        /// der Feldeditor ist das erste Glied der Antwortkette, und ohne diese
        /// Methode nahm das Fenster bei leerem Zellenstapel den letzten
        /// Tabellenumbau zurueck (gemessen, Modulkopf).
        ///
        /// **Der Menueeintrag bleibt dabei bedienbar**, auch am Anfang der
        /// Zelle, und ein `cmd+z` dort tut nichts. Grau wuerde er erst mit
        /// einer eigenen Antwort auf `validateMenuItem:`, und die Ausgrauung
        /// entscheidet in diesem Baum genau eine Stelle, beim
        /// Anwendungsdelegierten (C2.17 der Runde 7,
        /// `die_freigabe_eines_eintrags_wird_nirgends_gesetzt` in
        /// [`super::menue`]). Gemessen: ohne eigene Antwort sagt die Oberklasse
        /// fuer beide Eintraege ja.
        // SAFETY: Die Signatur ist die einer Aktion: ein optionales
        // Objektargument, keine Rueckgabe.
        #[unsafe(method(undo:))]
        fn rueckgaengig(&self, _absender: Option<&AnyObject>) {
            let verwalter = self.ivars();
            if verwalter.canUndo() {
                verwalter.undo();
            }
        }

        /// `shift+cmd+z` in einer Zelle, das Gegenstueck zu `undo:`.
        // SAFETY: wie bei `undo:`.
        #[unsafe(method(redo:))]
        fn wiederholen(&self, _absender: Option<&AnyObject>) {
            let verwalter = self.ivars();
            if verwalter.canRedo() {
                verwalter.redo();
            }
        }

        /// Jede Zelle beginnt mit leerem Stapel.
        ///
        /// Der Editor ist einer fuer alle Zellen, und ein Rest aus der vorigen
        /// naehme deren Tippen am Text dieser zurueck. Gemessen: AppKit ruft
        /// dies bei jedem Beginn einer Bearbeitung, und ein Wechsel des
        /// Schluesselfensters mitten im Tippen ruft es nicht (Fall 5 und 6 der
        /// Messung).
        // SAFETY: Die Signatur entspricht der von NSResponder
        // (`NSResponder.h:105`).
        #[unsafe(method(becomeFirstResponder))]
        fn wird_ersthelfer(&self) -> bool {
            // SAFETY: `becomeFirstResponder` der Oberklasse hat die hier
            // angenommene Signatur.
            let angenommen: bool = unsafe { msg_send![super(self), becomeFirstResponder] };
            if angenommen {
                self.ivars().removeAllActions();
            }
            angenommen
        }
    }
);

impl Zelleneditor {
    /// Ein Feldeditor mit leerem eigenem Verwalter.
    ///
    /// `setFieldEditor(true)` ist die Eigenschaft, an der AppKit und
    /// [`Eintragsansicht::laufende_zelle`] ihn als Feldeditor erkennen;
    /// `allowsUndo` setzte AppKit beim Beginn ohnehin, es steht hier, weil
    /// [`Self::rueckgaengig`] ohne es nichts zurueckzunehmen haette.
    fn neu(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(NSUndoManager::new(mtm));
        // SAFETY: `initWithFrame:` von NSTextView hat die hier angenommene
        // Signatur; der Rahmen ist gleichgueltig, AppKit legt den Feldeditor
        // ueber das bearbeitete Feld.
        let this: Retained<Self> = unsafe { msg_send![super(this), initWithFrame: NSRect::ZERO] };
        this.setFieldEditor(true);
        this.setAllowsUndo(true);
        this
    }
}

impl Eintragstabelle {
    /// Eine leere Tabelle mit dem genannten Rahmen.
    fn neu(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: `initWithFrame:` von NSView, die NSTableView erbt, hat die
        // hier angenommene Signatur.
        unsafe { msg_send![super(this), initWithFrame: rahmen] }
    }
}

/// Was die Eintragsansicht haelt.
pub struct EintragsansichtIvars {
    /// Die Rolle um die Tabelle; sie haengt der Editorbereich in seine Ansicht.
    rolle: Retained<NSScrollView>,
    /// Die Tabelle selbst; sie nimmt den Ersthelferrang an.
    tabelle: Retained<Eintragstabelle>,
    /// Die zuletzt abgeleiteten Zeilen.
    ///
    /// Keine zweite Wahrheit neben dem Stand: sie werden allein von
    /// [`Eintragsansicht::zeilen_zeigen`] geschrieben, und das bekommt sie aus
    /// [`aufgabenzeilen`] ueber den Stand des Editors.
    zeilen: RefCell<Vec<Eintragszeile>>,
    /// Die Wege in den Editor; `None`, bis der Editorbereich sie eintraegt.
    wege: RefCell<Option<Zellenwege>>,
    /// Ob die gerade endende Bearbeitung verworfen wird.
    ///
    /// Gesetzt allein fuer die Dauer von [`Eintragsansicht::bearbeitung_verwerfen`],
    /// und das ist kein gemerkter Zustand ueber die Zelle, sondern die Antwort
    /// auf die Frage, **wie** sie endet: die beiden Delegiertenmethoden laufen
    /// innerhalb jenes einen `makeFirstResponder:`.
    verwerfen: Cell<bool>,
    /// Ob die laufende Zelle gerade selbst endet und festschreibt.
    ///
    /// Gesetzt allein fuer die Dauer des Festschreibens in
    /// [`Eintragsansicht::zelle_geendet`], und aus demselben Grund kein
    /// gemerkter Zustand ueber die Zelle wie [`Self::verwerfen`]: das
    /// Festschreiben kommt ueber den Umbau nach
    /// [`Eintragsansicht::zeilen_zeigen`], und dort meldet
    /// [`Eintragsansicht::laufende_zelle`] die endende Zelle noch (gemessen,
    /// Fall 4 von `messungen/260926-0828-zellen-rueckgaengig.txt`). Ohne diese
    /// Antwort verwuerfe das Neuladen die Zelle, die gerade uebernommen wird.
    endet: Cell<bool>,
    /// Der Feldeditor der Zellen, siehe [`Zelleneditor`].
    zelleneditor: Retained<Zelleneditor>,
    /// Der Hauptfadenbeweis vom Aufbau.
    ///
    /// Gemerkt und nicht mit `mtm()` erfragt, weil `mtm()` den wirklichen
    /// Hauptfaden prueft und die Proben unter `libtest` ihn nur behaupten
    /// (`an_einer_flaeche` in [`super::editor`]); im Programm sind beide
    /// Antworten dieselbe.
    mtm: MainThreadMarker,
}

define_class!(
    /// Datenquelle und Delegierter der Eintragstabelle und ihrer Textfelder,
    /// und Halter ihrer Rolle.
    ///
    /// Ein Objekt fuer alle Rollen, wie beim Git-Bereich: eine Spalte, ein
    /// Zustand, und mehrere Objekte dafuer waeren mehrere Halter desselben.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = EintragsansichtIvars]
    pub struct Eintragsansicht;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Eintragsansicht {}

    // SAFETY: `NSTableViewDataSource` stellt keine Bedingungen.
    unsafe impl NSTableViewDataSource for Eintragsansicht {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(numberOfRowsInTableView:))]
        fn zeilenzahl(&self, _tabelle: &NSTableView) -> NSInteger {
            NSInteger::try_from(self.ivars().zeilen.borrow().len()).unwrap_or(NSInteger::MAX)
        }
    }

    // SAFETY: `NSControlTextEditingDelegate` ist Oberprotokoll von
    // `NSTableViewDelegate` und `NSTextFieldDelegate` und hat nur wahlfreie
    // Methoden. Die beiden hier sind die Uebernahme einer Zelle (Modulkopf).
    unsafe impl NSControlTextEditingDelegate for Eintragsansicht {
        /// Darf die Zelle mit dem getippten Text enden?
        ///
        /// Der Text kommt aus dem Feldeditor, den AppKit mitgibt; das Feld
        /// selbst traegt ihn erst nach dem Ende.
        // SAFETY: Die Signatur entspricht der des Protokolls
        // (`objc2-app-kit-0.3.2/src/generated/NSControl.rs:514`).
        #[unsafe(method(control:textShouldEndEditing:))]
        fn darf_enden(&self, feld: &NSControl, feldeditor: &NSText) -> bool {
            self.zelle_darf_enden(feld, &feldeditor.string().to_string())
        }

        /// Die Zelle ist zu Ende gegangen.
        // SAFETY: Die Signatur entspricht der des Protokolls
        // (`objc2-app-kit-0.3.2/src/generated/NSControl.rs:493`).
        #[unsafe(method(controlTextDidEndEditing:))]
        fn hat_geendet(&self, meldung: &NSNotification) {
            let feld = meldung
                .object()
                .and_then(|feld| feld.downcast::<NSTextField>().ok());
            if let Some(feld) = feld {
                self.zelle_geendet(&feld);
            }
        }
    }

    // SAFETY: `NSTextFieldDelegate` stellt keine Bedingungen; es ist der
    // Protokolltyp, den `NSTextField::setDelegate:` verlangt.
    unsafe impl NSTextFieldDelegate for Eintragsansicht {}

    // SAFETY: `NSTableViewDelegate` stellt keine Bedingungen.
    unsafe impl NSTableViewDelegate for Eintragsansicht {
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
    }

    impl Eintragsansicht {
        /// Das Ankreuzfeld einer Zeile ist angeklickt worden.
        ///
        /// Die Zeile kommt von der Tabelle ueber `rowForView:` und nicht aus
        /// einem gemerkten Zustand, wie beim Umbenennen in [`super::tabelle`].
        /// **Das Kaestchen zeigt danach die Ableitung und nicht den Klick:**
        /// AppKit schaltet es beim Klick selbst um, und wird die Handlung
        /// abgewiesen, laedt die Tabelle nicht neu und liesse es falsch stehen.
        // SAFETY: Die Signatur ist die einer Aktion mit dem Absender als
        // Argument; Absender ist allein das Ankreuzfeld aus `zellenansicht`.
        #[unsafe(method(kastenGeklickt:))]
        fn kasten_geklickt(&self, kasten: &NSButton) {
            let Ok(zeile) = usize::try_from(self.ivars().tabelle.rowForView(kasten)) else {
                return;
            };
            if let Some(wege) = self.ivars().wege.borrow().as_ref() {
                (wege.abhaken)(zeile);
            }
            if let Ok(zeile) = usize::try_from(self.ivars().tabelle.rowForView(kasten))
                && let Some(eintrag) = self.ivars().zeilen.borrow().get(zeile)
            {
                kasten.setState(kastenzustand(eintrag.erledigt));
            }
        }

        /// Doppelklick auf eine Zeile: ihre Zelle geht in Bearbeitung.
        // SAFETY: Die Signatur ist die einer Aktion; der Absender ist die
        // Tabelle und wird nicht gebraucht.
        #[unsafe(method(zeileDoppelt:))]
        fn zeile_doppelt(&self, _absender: Option<&AnyObject>) {
            if let Ok(zeile) = usize::try_from(self.ivars().tabelle.clickedRow()) {
                let _ = self.bearbeitung_beginnen(zeile);
            }
        }
    }
);

/// Der Zustand eines Ankreuzfeldes zu einem Erledigt-Wert.
fn kastenzustand(erledigt: bool) -> NSInteger {
    if erledigt {
        NSControlStateValueOn
    } else {
        NSControlStateValueOff
    }
}

impl Eintragsansicht {
    /// Baut Rolle und Tabelle, leer und ausgeblendet.
    ///
    /// **Ausgeblendet**, weil die Textflaeche die Flaeche ist, mit der der
    /// Editor entsteht; welche der beiden zu sehen ist, entscheidet allein
    /// `Editorbereich::flaeche_waehlen`.
    pub fn bauen(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let tabelle = Eintragstabelle::neu(mtm, rahmen);
        tabelle.setRowHeight(ZEILENHOEHE);
        tabelle.setUsesAutomaticRowHeights(false);
        tabelle.setStyle(NSTableViewStyle::FullWidth);
        tabelle.setHeaderView(None);
        tabelle.setAllowsEmptySelection(true);
        tabelle.setAllowsMultipleSelection(false);

        let spalte =
            NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), ns_string!("aufgabe"));
        spalte.setResizingMask(NSTableColumnResizingOptions::AutoresizingMask);
        tabelle.addTableColumn(&spalte);

        let rolle = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), rahmen);
        rolle.setHasVerticalScroller(true);
        rolle.setAutohidesScrollers(true);
        rolle.setDocumentView(Some(&tabelle));
        rolle.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );
        rolle.setHidden(true);

        let this = Self::alloc(mtm).set_ivars(EintragsansichtIvars {
            rolle,
            tabelle,
            zeilen: RefCell::new(Vec::new()),
            wege: RefCell::new(None),
            verwerfen: Cell::new(false),
            endet: Cell::new(false),
            zelleneditor: Zelleneditor::neu(mtm),
            mtm,
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };

        // SAFETY: Das Objekt beantwortet beide Protokolle, die es oben
        // implementiert. Getragen wird der Aufruf davon, dass `dataSource` und
        // `delegate` nullende schwache Eigenschaften sind ("This is a weak
        // property", `objc2-app-kit-0.3.2/src/generated/NSTableView.rs:402-421`)
        // und dass dieses Objekt die Tabelle selbst festhaelt. Ziel und
        // Doppelaktion: `NSControl` haelt sein Ziel schwach (`NSControl.h:24`),
        // und die Aktion ist die Methode, die dieses Objekt oben dafuer traegt.
        unsafe {
            let tabelle = &this.ivars().tabelle;
            tabelle.setDataSource(Some(ProtocolObject::from_ref(&*this)));
            tabelle.setDelegate(Some(ProtocolObject::from_ref(&*this)));
            tabelle.setTarget(Some(&this));
            tabelle.setDoubleAction(Some(sel!(zeileDoppelt:)));
        }
        this
    }

    /// Traegt die drei Wege in den Editor ein (siehe [`Zellenwege`]).
    pub fn wege_setzen(&self, wege: Zellenwege) {
        *self.ivars().wege.borrow_mut() = Some(wege);
    }

    /// Die Rolle um die Tabelle, die der Editorbereich einhaengt und ein- und
    /// ausblendet.
    pub fn rolle(&self) -> &NSScrollView {
        &self.ivars().rolle
    }

    /// Die Tabelle, die den Eingabefokus traegt, solange die Ansicht zu sehen
    /// ist.
    pub fn tabelle(&self) -> &NSTableView {
        &self.ivars().tabelle
    }

    /// Zeigt die genannten Zeilen, falls sie sich von den gezeigten
    /// unterscheiden.
    ///
    /// **Gleiche Zeilen laden nicht neu.** Der Editorbereich ruft nach jeder
    /// Aenderung seines Standes, also auch bei jedem Anschlag in einer
    /// Quelltextdatei, fuer die hier die leere Liste ankommt; ein `reloadData`
    /// je Anschlag waere ein Auslegen fuer nichts.
    ///
    /// **Die Auswahl bleibt auf ihrer Stelle**, auf die neue Laenge
    /// beschnitten: loescht ein `cmd+z` die letzte Aufgabe, steht sie danach
    /// auf der neuen letzten und nicht auf keiner. Wohin eine Handlung sie
    /// setzt, sagt der Kern ueber `Neustand::auswahl` und
    /// [`Self::auswahl_setzen`], nicht diese Funktion.
    ///
    /// **Eine offene Zelle wird zuerst verworfen**, und zwar vor dem Vergleich:
    /// gerufen wird nach jeder Aenderung des Standes, und steht dabei eine Zelle
    /// offen, die nicht selbst gerade festschreibt, dann hat sich der Stand von
    /// aussen unter ihr geaendert — ein `cmd+z` mit der Tabelle im Fokus, eine
    /// eingetroffene Datei. `reloadData` beendete sie sonst mit Zeile -1, und
    /// der Text fiele still (Modulkopf, "Solange eine Zelle laeuft"). Verworfen
    /// und nicht uebernommen, weil der Stand darunter nicht mehr der ist, gegen
    /// den sie begonnen hat.
    pub fn zeilen_zeigen(&self, zeilen: Vec<Eintragszeile>) {
        self.fremde_zelle_verwerfen();
        if *self.ivars().zeilen.borrow() == zeilen {
            return;
        }
        let laenge = zeilen.len();
        let vorher = self.gewaehlte_zeile();
        // Die Ausleihe endet an ihrem Semikolon: `reloadData` fragt die Zeilen
        // gleich wieder ab.
        *self.ivars().zeilen.borrow_mut() = zeilen;
        self.ivars().tabelle.reloadData();
        let stelle =
            vorher.and_then(|stelle| laenge.checked_sub(1).map(|letzte| stelle.min(letzte)));
        self.auswahl_setzen(stelle);
    }

    /// Verwirft eine offene Zelle, unter der sich der Stand von aussen
    /// geaendert hat; siehe [`Self::zeilen_zeigen`].
    fn fremde_zelle_verwerfen(&self) {
        let Some(fenster) = self.ivars().tabelle.window() else {
            return;
        };
        let Some(ersthelfer) = fenster.firstResponder() else {
            return;
        };
        if self.zelle_unter_fremdem_stand(&ersthelfer) {
            self.bearbeitung_verwerfen(&fenster);
        }
    }

    /// Ob dieser Ersthelfer eine Zelle bearbeitet, die nicht gerade selbst
    /// endet: die Frage vor jedem Neuladen.
    ///
    /// `pub(super)` allein fuer die Proben in [`super::editor`], die kein
    /// Fenster haben und den Ersthelfer deshalb hereinreichen.
    #[must_use]
    pub(super) fn zelle_unter_fremdem_stand(&self, ersthelfer: &NSResponder) -> bool {
        !self.ivars().endet.get()
            && !self.ivars().verwerfen.get()
            && self.laufende_zelle(ersthelfer).is_some()
    }

    /// Der Feldeditor fuer das genannte Objekt, falls es ein Feld unter dieser
    /// Tabelle ist: die Antwort auf `windowWillReturnFieldEditor:toObject:`
    /// beim Fensterdelegierten (`super::fenster`).
    ///
    /// **Allein fuer Felder unter der Tabelle**, gefragt an derselben
    /// Naemlichkeit wie [`Self::laufende_zelle`]; jedes andere Feld im Fenster,
    /// das Umbenennen im Dateifenster eingeschlossen, bekommt `None` und damit
    /// den Feldeditor von AppKit. Die Tabelle selbst ist kein Feld, obwohl
    /// `isDescendantOf:` fuer sie ja sagt.
    #[must_use]
    pub fn feldeditor_fuer(&self, klient: &AnyObject) -> Option<Retained<NSTextView>> {
        let ansicht = klient.downcast_ref::<NSView>()?;
        let tabelle: &NSView = &self.ivars().tabelle;
        if !ansicht.isDescendantOf(tabelle) || std::ptr::eq(ansicht, tabelle) {
            return None;
        }
        Some(Retained::into_super(self.ivars().zelleneditor.clone()))
    }

    /// Waehlt die Zeile an der genannten Stelle und bringt sie ins Bild.
    ///
    /// `None` und eine Stelle hinter der letzten Zeile lassen die Auswahl, wie
    /// sie ist: eine Handlung ohne Auswahl danach hat keine gewaehlt, und eine
    /// zu grosse Stelle beantwortete AppKit mit einer Ausnahme, die Rust nicht
    /// fangen kann.
    pub fn auswahl_setzen(&self, stelle: Option<usize>) {
        let Some(stelle) = stelle else {
            return;
        };
        if stelle >= self.ivars().zeilen.borrow().len() {
            return;
        }
        let tabelle = &self.ivars().tabelle;
        tabelle
            .selectRowIndexes_byExtendingSelection(&NSIndexSet::indexSetWithIndex(stelle), false);
        tabelle.scrollRowToVisible(NSInteger::try_from(stelle).unwrap_or(NSInteger::MAX));
    }

    /// Die gewaehlte Zeile, falls eine gewaehlt ist.
    #[must_use]
    pub fn gewaehlte_zeile(&self) -> Option<usize> {
        usize::try_from(self.ivars().tabelle.selectedRow()).ok()
    }

    /// Der abgeleitete Text der gewaehlten Aufgabe, fuer `copy:`.
    #[must_use]
    fn gewaehlter_text(&self) -> Option<String> {
        let zeile = self.gewaehlte_zeile()?;
        self.ivars()
            .zeilen
            .borrow()
            .get(zeile)
            .map(|eintrag| eintrag.text.clone())
    }

    /// Die Zelle, die gerade bearbeitet wird, falls der Ersthelfer ihr
    /// Feldeditor ist (C6.7).
    ///
    /// **Gelesen im Augenblick der Frage und nirgends gemerkt**, im selben
    /// Zuschnitt wie `bereich_des_ersthelfers` beim Anwendungsdelegierten: der
    /// Ersthelfer ist ein Feldeditor (`isFieldEditor`), und sein Delegierter,
    /// das Textfeld, dessen Text er traegt, liegt unter [`Eintragstabelle`].
    /// Damit ist eine Zelle erkannt, sobald AppKit den Feldeditor einsetzt,
    /// und nicht erst nach dem ersten Zeichen; und der Feldeditor eines
    /// Blattes oder eines anderen Textfeldes faellt heraus, weil sein
    /// Delegierter anderswo liegt.
    ///
    /// Die Textflaeche des Editors ist kein Feldeditor und faellt an der
    /// ersten Frage heraus.
    #[must_use]
    pub fn laufende_zelle(&self, ersthelfer: &NSResponder) -> Option<Zelle> {
        let feldeditor = ersthelfer.downcast_ref::<NSTextView>()?;
        if !feldeditor.isFieldEditor() {
            return None;
        }
        let delegierter = feldeditor.delegate()?;
        let feld = AsRef::<AnyObject>::as_ref(&*delegierter).downcast_ref::<NSView>()?;
        self.zelle_von(feld)
    }

    /// Die Zelle, in der die genannte Ansicht liegt, falls sie unter der
    /// Tabelle liegt.
    fn zelle_von(&self, ansicht: &NSView) -> Option<Zelle> {
        let tabelle = &self.ivars().tabelle;
        if !ansicht.isDescendantOf(tabelle) {
            return None;
        }
        let zeile = usize::try_from(tabelle.rowForView(ansicht)).ok()?;
        let spalte = usize::try_from(tabelle.columnForView(ansicht)).ok()?;
        Some(Zelle { zeile, spalte })
    }

    /// Setzt die Zelle der genannten Zeile in Bearbeitung, der Text ist
    /// ausgewaehlt.
    ///
    /// Derselbe Weg wie das Umbenennen in [`super::tabelle`]:
    /// `editColumn:row:withEvent:select:` macht den Feldeditor des Fensters
    /// zum Ersthelfer und stellt ihn in das Textfeld der Zelle. Liefert
    /// `false`, wenn es die Zeile nicht gibt.
    pub fn bearbeitung_beginnen(&self, zeile: usize) -> bool {
        if zeile >= self.ivars().zeilen.borrow().len() {
            return false;
        }
        let Ok(zeile) = NSInteger::try_from(zeile) else {
            return false;
        };
        let tabelle = &self.ivars().tabelle;
        tabelle.scrollRowToVisible(zeile);
        tabelle.editColumn_row_withEvent_select(TEXTSPALTE, zeile, None, true);
        true
    }

    /// Beendet eine laufende Bearbeitung **uebernehmend**: die Tabelle nimmt
    /// den Ersthelfer, und AppKit fragt dabei die beiden Delegiertenmethoden.
    ///
    /// Liefert, ob AppKit den Wechsel angenommen hat. `false` heisst: die
    /// Pruefung hat den Text abgewiesen, und die Zelle steht weiter in
    /// Bearbeitung. Der eine Rufer ist `Editorbereich::zelle_uebernehmen`.
    #[must_use = "ein abgelehnter Wechsel heisst, dass die Zelle weiter bearbeitet wird"]
    pub fn bearbeitung_beenden(&self, fenster: &NSWindow) -> bool {
        fenster.makeFirstResponder(Some(&self.ivars().tabelle))
    }

    /// Beendet eine laufende Bearbeitung **verwerfend**: kein Umbau, und die
    /// Zelle zeigt danach wieder den abgeleiteten Text.
    ///
    /// Derselbe Wechsel wie [`Self::bearbeitung_beenden`], mit gesetztem
    /// [`EintragsansichtIvars::verwerfen`]: die Pruefung sagt ja, ohne zu
    /// rechnen, und das Ende schreibt nichts fest. Nicht `abortEditing`: das
    /// laesst den Ersthelferrang an keiner benannten Stelle zurueck, und der
    /// Fokus soll in der Tabelle bleiben.
    pub fn bearbeitung_verwerfen(&self, fenster: &NSWindow) {
        self.ivars().verwerfen.set(true);
        let _ = fenster.makeFirstResponder(Some(&self.ivars().tabelle));
        self.ivars().verwerfen.set(false);
    }

    /// Die Antwort auf `control:textShouldEndEditing:`.
    ///
    /// `pub(super)` allein fuer die Proben in [`super::editor`], die den
    /// Feldeditor nicht beschreiben koennen (`setString:` endet unter `libtest`
    /// mit `SIGSEGV`) und den Text deshalb unmittelbar hereinreichen.
    pub(super) fn zelle_darf_enden(&self, feld: &NSControl, text: &str) -> bool {
        if self.ivars().verwerfen.get() {
            return true;
        }
        let Ok(zeile) = usize::try_from(self.ivars().tabelle.rowForView(feld)) else {
            return true;
        };
        self.ivars()
            .wege
            .borrow()
            .as_ref()
            .is_none_or(|wege| (wege.pruefen)(zeile, text))
    }

    /// Die Antwort auf `controlTextDidEndEditing:`.
    ///
    /// **Das Feld zeigt danach die Ableitung.** Nach einem Umbau hat
    /// `reloadData` die Zelle ohnehin neu gebaut, und das Feld steht in keiner
    /// Zeile mehr; nach einem Ende ohne Umbau — verworfen, unveraendert, oder
    /// ein Ende, bei dem AppKit nicht gefragt hat — stuende sonst der getippte
    /// Text da und behauptete eine Aenderung, die nicht im Stand steht.
    pub(super) fn zelle_geendet(&self, feld: &NSTextField) {
        if !self.ivars().verwerfen.get()
            && let Ok(zeile) = usize::try_from(self.ivars().tabelle.rowForView(feld))
        {
            let text = feld.stringValue().to_string();
            if let Some(wege) = self.ivars().wege.borrow().as_ref() {
                self.ivars().endet.set(true);
                (wege.festschreiben)(zeile, &text);
                self.ivars().endet.set(false);
            }
        }
        if let Ok(zeile) = usize::try_from(self.ivars().tabelle.rowForView(feld))
            && let Some(eintrag) = self.ivars().zeilen.borrow().get(zeile)
        {
            feld.setStringValue(&NSString::from_str(&eintrag.text));
        }
    }

    /// Die Ansicht fuer eine Zeile: das Ankreuzfeld und daneben der Text.
    ///
    /// Eine `NSTableCellView` mit dem Textfeld als `textField`, damit
    /// `editColumn:row:withEvent:select:` weiss, welches Feld es bearbeitet.
    /// Das Ankreuzfeld nimmt den Ersthelferrang nicht an: der gehoert der
    /// Tabelle, und ein Klick darauf soll keine laufende Zelle beenden, ohne
    /// dass `aufgabe_abhaken` sie vorher uebernommen hat.
    fn zellenansicht(&self, zeile: NSInteger) -> Option<Retained<NSView>> {
        let mtm = self.ivars().mtm;
        let stelle = usize::try_from(zeile).ok()?;
        let eintrag = self.ivars().zeilen.borrow().get(stelle)?.clone();

        // SAFETY: Ziel ist dieses Objekt, das die Tabelle ueberlebt, und
        // `NSControl` haelt es schwach (`NSControl.h:24`); die Aktion ist die
        // Methode, die es oben dafuer traegt, mit dem Absender als Argument.
        let kasten = unsafe {
            NSButton::checkboxWithTitle_target_action(
                ns_string!(""),
                Some(self),
                Some(sel!(kastenGeklickt:)),
                mtm,
            )
        };
        kasten.setState(kastenzustand(eintrag.erledigt));
        kasten.setRefusesFirstResponder(true);
        kasten.setFrame(NSRect::new(
            NSPoint::new(EINZUG, 0.0),
            NSSize::new(KASTENBREITE, ZEILENHOEHE),
        ));

        let feld = NSTextField::textFieldWithString(&NSString::from_str(&eintrag.text), mtm);
        feld.setFont(Some(&NSFont::systemFontOfSize(NSFont::systemFontSize())));
        feld.setLineBreakMode(NSLineBreakMode::ByTruncatingTail);
        feld.setBordered(false);
        feld.setDrawsBackground(false);
        feld.setEditable(true);
        let links = EINZUG + KASTENBREITE;
        feld.setFrame(NSRect::new(
            NSPoint::new(links, 2.0),
            NSSize::new(AUFBAUBREITE - links - EINZUG, ZEILENHOEHE - 4.0),
        ));
        feld.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
        // SAFETY: Der Delegierte ist dieses Objekt, das die Tabelle und damit
        // das Feld ueberlebt; `delegate` ist eine nullende schwache
        // Eigenschaft (`objc2-app-kit-0.3.2/src/generated/NSTextField.rs:238-250`).
        unsafe { feld.setDelegate(Some(ProtocolObject::from_ref(self))) };

        let zelle = NSTableCellView::initWithFrame(
            NSTableCellView::alloc(mtm),
            NSRect::new(NSPoint::ZERO, NSSize::new(AUFBAUBREITE, ZEILENHOEHE)),
        );
        zelle.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
        zelle.addSubview(&kasten);
        zelle.addSubview(&feld);
        // SAFETY: Das Feld ist eine Unteransicht der Zelle und lebt so lange
        // wie sie; die Eigenschaft ist `weak` (`NSTableCellView.h`).
        unsafe { zelle.setTextField(Some(&feld)) };
        Some(Retained::into_super(zelle))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// C6.2: aus Vorspann, zwei Aufgaben und einer fremden Zeile entstehen
    /// genau zwei Zeilen, in der Reihenfolge der Datei, mit dem Zustand ihres
    /// Kaestchens; die fremde Zeile ist keine Zeile der Tabelle.
    #[test]
    fn aus_vorspann_zwei_aufgaben_und_einer_fremden_zeile_entstehen_zwei_zeilen() {
        let stand = "# Aufgaben\n\n- [ ] Brot\n  fremde Zeile\n  * [X] Steuer\n";
        assert_eq!(
            aufgabenzeilen(stand),
            vec![
                Eintragszeile {
                    erledigt: false,
                    text: "Brot".to_owned(),
                },
                Eintragszeile {
                    erledigt: true,
                    text: "Steuer".to_owned(),
                },
            ]
        );
    }

    /// Ein Stand ohne Aufgabe gibt keine Zeile, auch der leere.
    #[test]
    fn ohne_aufgabe_gibt_es_keine_zeile() {
        assert!(aufgabenzeilen("").is_empty());
        assert!(aufgabenzeilen("nur Vorspann\n").is_empty());
    }
}
