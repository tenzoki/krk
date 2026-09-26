//! Die Eintragsansicht: `tasks.txt`, `notes.txt` und `appointments.md` aus dem
//! erkannten `~/krkhome/` als Tabelle in der Formatansicht des Editors (C6 und
//! C5 des Arbeitspakets `260925-2356-f2-oeffnet-krkhome-statt-notizfenster`,
//! die Termine aus Schritt 6 des Plans
//! `260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md`).
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
//! # Eine Ansicht fuer jede Art
//!
//! **Aufgaben, Notizen und Termine teilen sich diese eine Ansicht**, und welche
//! Art sie zeigt, sagen die Zeilen selbst: [`Zeilen::Aufgaben`] traegt eine
//! Spalte mit Kaestchen und Text in fester Zeilenhoehe, [`Zeilen::Notizen`]
//! zwei Spalten, Thema und Notiz, mit Kopfzeile und einer Zeilenhoehe nach dem
//! Inhalt (Schritt 4.3). [`Zeilen::Termine`] traegt dieselben zwei Spalten
//! unter den Namen Datum und Termin, die erste schmaler, und steht als einzige
//! Art **nicht in der Reihenfolge der Datei**, sondern nach dem Datum
//! ([`terminzeilen`]); was daraus folgt, steht unter „Stelle und Zeile". [`Eintragsansicht::zeilen_zeigen`] richtet die Spalten neu
//! ein, wenn die Art wechselt, und an keiner anderen Stelle. **Eine Ansicht und
//! nicht zwei**, weil alles, was an der Naemlichkeit der Tabelle haengt, damit
//! eine Stelle behaelt: [`Eintragsansicht::laufende_zelle`], der eigene
//! Feldeditor ueber [`Eintragsansicht::feldeditor_fuer`], das Fokusziel und der
//! Flaechentausch des Editors. Eine zweite Tabelle muesste jede dieser Fragen
//! ein zweites Mal beantworten, und der Fensterdelegierte fragte sonst die
//! falsche. Die Geheimnisse aus Stufe 5 zeigen sich als Notizen.
//!
//! # Die mehrzeilige Notizzelle
//!
//! Gemessen in `messungen/260926-0818-mehrzeilige-zelle.txt` (Belege unter
//! `spikes/mehrzeilige-zelle/`): **eine Zeile waechst beim Tippen nicht von
//! selbst mit**, auch nicht nach `noteHeightOfRowsWithIndexesChanged:`, weil
//! das Feld waehrend der Bearbeitung die Eigenhoehe seines alten Werts meldet.
//! Was traegt, steht hier in drei Stuecken: das Feld der Zelle ist ein
//! [`Notizfeld`], das waehrend der Bearbeitung die belegte Hoehe des
//! Feldeditortexts als Eigenhoehe meldet; `controlTextDidChange:` macht diese
//! Eigenhoehe je Anschlag ungueltig; und das Feld traegt vertikal den
//! Stauchwiderstand `NSLayoutPriorityRequired` und haengt ueber Auto Layout an
//! den vier Kanten seiner Zelle, sonst gewinnt die Hoehenbindung, die die
//! Tabelle an jede Zeile legt. `noteHeightOfRowsWithIndexesChanged:` steht
//! deshalb nirgends. **Nicht gemessen** sind das Schrumpfen beim Loeschen von
//! Umbruechen, die Hoehe nach dem Ende der Bearbeitung und der Umbruch einer
//! ueberlangen Zeile; die Umbruchbreite folgt der Spaltenbreite ueber
//! `tableViewColumnDidResize:`.
//!
//! In der Notizspalte schreibt `return` einen Zeilenumbruch
//! (`260926-0112_*_was-tut-return-in-einer-notizzelle-und-welche-tasten-tragen-die-editoren.md`,
//! Moeglichkeit 1), und `tab` wechselt uebernehmend in die naechste Zelle;
//! beides entscheidet die reine Regel [`zellenbefehl`]. In der Themenspalte
//! und in der Aufgabenzelle beendet `return` die Zelle wie bisher, denn ein
//! Thema traegt keinen Umbruch (`Abweisung::UmbruchImThema`).
//!
//! # Eine Sicht auf den Stand des Editors und kein eigenes Modell
//!
//! **Die Tabelle haelt keine Eintraege, sondern eine Ableitung.** Ihre Zeilen
//! entstehen aus dem Stand, den das [`crate::editormodell::Editormodell`] haelt,
//! ueber [`aufgabenzeilen`] und [`notizzeilen`], je Art die eine Stelle der
//! Ableitung; sie lesen den Stand mit `krk_core::heimordner::eintraege::Aufgaben`
//! und `Notizen`, derselben Zerlegung, mit der die Handlungen des Kerns
//! rechnen. Gerufen wird sie vom
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
//! # Stelle und Zeile
//!
//! **Zwei Zahlen, und die Naht zwischen ihnen liegt in dieser Datei.** Die
//! *Stelle* ist die Zahl, mit der der Kern einen Eintrag zaehlt: in der
//! Reihenfolge der Datei, ohne den Vorspann, wie `Neustand::auswahl` und jede
//! Handlung in `krk_core::heimordner::eintraege`. Die *Zeile* ist AppKits
//! Zeilennummer, also die Reihenfolge auf dem Schirm. Fuer Aufgaben und
//! Notizen sind beide dieselbe Zahl; die Termintabelle ordnet nach dem Datum
//! und trennt sie, und jede [`Terminzeile`] traegt deshalb ihre Stelle.
//!
//! **Nach aussen spricht diese Ansicht allein in Stellen**: die gewaehlte
//! Stelle ([`Eintragsansicht::gewaehlte_stelle`]), die gemeldete [`Zelle`],
//! die Auswahl nach einem Umbau ([`Eintragsansicht::auswahl_setzen`]), der
//! Beginn einer Bearbeitung ([`Eintragsansicht::bearbeitung_beginnen`]) und der
//! Rueckruf des Ankreuzfeldes ([`Zellenwege::abhaken`]). Der Editorbereich und
//! der Kern sehen damit nie eine Zeilennummer. Jede Stelle, an der eine
//! Zeilennummer von AppKit herein- oder hinausgeht — `selectedRow`,
//! `clickedRow`, `rowForView:`, `selectRowIndexes:`, `editColumn:row:` und der
//! Zellenwechsel mit `tab` —, rechnet ueber [`Zeilen::stelle_der_zeile`] und
//! [`Zeilen::zeile_der_stelle`] und ueber keinen dritten Weg. Wer hier eine
//! Zeilennummer ohne die Umrechnung weiterreicht, aendert oder loescht in einer
//! umgeordneten Tabelle den falschen Eintrag.
//!
//! Innen bleibt es bei Zeilen: die abgeleiteten Zeilen stehen in der
//! Reihenfolge des Schirms, und wer sie mit einer Zeilennummer von AppKit
//! befragt, braucht nichts umzurechnen.
//!
//! # Zellen, und wer sie beendet
//!
//! Seit Schritt 3.2b des Plans
//! `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`
//! ist das Textfeld jeder Zeile bearbeitbar und das Ankreuzfeld eingeschaltet.
//! **Die Tabelle rechnet dabei nichts selbst.** Was aus einem getippten Text
//! oder einem Klick ins Kaestchen wird, entscheidet der Editorbereich ueber die
//! Wege in [`Zellenwege`]; diese Datei meldet allein, welche Stelle es
//! betrifft und was in ihr steht.
//!
//! ```text
//!   Klick daneben, tab, return ──> control:textShouldEndEditing: ──> pruefen
//!   (return in der Notizspalte: Zeilenumbruch, kein Ende)
//!                                   │ ja                            (nein: Zelle bleibt)
//!                                   └> controlTextDidEndEditing:  ──> festschreiben
//!   Editorbereich::zelle_uebernehmen ──> bearbeitung_beenden ──┘ (derselbe Weg)
//!   esc (Rang in `abbrechen`)        ──> bearbeitung_verwerfen  ──> Anzeige zurueck
//!                                        (Notizzelle: zelle_uebernehmen, Schritt 4.3)
//!   tab in der Notiztabelle          ──> bearbeitung_beenden, dann naechste Zelle
//!   Ankreuzfeld                      ──> abhaken, dann Fokus in die Tabelle
//!   Doppelklick auf eine Zelle       ──> zelle_beginnen
//!   Doppelklick unter die Zeilen     ──> anlegen (auch in der leeren Tabelle)
//! ```
//!
//! **Der Doppelklick ins Leere legt einen Eintrag an**, nach der reinen Regel
//! [`doppelklick`]. Der Spec verlangt jede Handlung „mit der Tastatur und
//! ebenso mit der Maus" (C5, C6), und in einer leeren Tabelle gab es fuer die
//! Maus nichts anzuklicken als den Menueeintrag; eine eigene `keymap.toml` aus
//! einer Fassung vor diesen Befehlen laesst sie ausserdem ohne Taste
//! (`issues/260926-1400_*_eine-zelle-der-eintragstabelle-laesst-sich-im-laufenden-buendel-nicht-bearbeiten.md`).
//! Ein Doppelklick auf die Kopfzeile ist kein Klick ins Leere, obwohl AppKit
//! auch dann `clickedRow` -1 meldet: er liegt ausserhalb der Tabelle, und die
//! Regel fragt deshalb nach dem Ort des Ereignisses.
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
//! `NSApplication`, `NSEvent`, `NSView`, `NSScrollView`, `NSTableView`, `NSTableColumn`, `NSButton`,
//! `NSTextField`, `NSControl`, `NSText`, `NSTextView`, `NSResponder`,
//! `NSWindow`, `NSFont`, `NSIndexSet`, `NSNotification`, `NSObject`,
//! `NSTableHeaderView`, `NSLayoutManager`, `NSTextContainer`, `NSCell` und
//! `NSString` stehen seit macOS 10.0 zur Verfuegung, ebenso die vier bedienten
//! Protokolle `NSTableViewDataSource`, `NSTableViewDelegate`,
//! `NSControlTextEditingDelegate` und `NSTextFieldDelegate`
//! (`NSTextField.h:124`, ohne Angabe) samt `NSObjectProtocol`, dem Kistennamen
//! des Protokolls `NSObject` (`objc/NSObject.h`), und die Aufzaehlungen
//! `NSAutoresizingMaskOptions`, `NSTableColumnResizingOptions`,
//! `NSTableViewColumnAutoresizingStyle` und `NSLineBreakMode`
//! (`NSParagraphStyle.h:25`). Ohne eigene
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
//! `delegate`, `isDescendantOf:`, `window`, `firstResponder`, `makeFirstResponder:`,
//! `object`, `systemFontOfSize:`, `smallSystemFontSize`, `systemFontSize`,
//! `indexSetWithIndex:`, `tableColumns`, `removeTableColumn:`, `setTitle:`,
//! `setWidth:`, `setMinWidth:`, `width`, `identifier`, `columnWithIdentifier:`,
//! `sizeLastColumnToFit`, `setColumnAutoresizingStyle:`, `headerView`,
//! `setHeaderView:`, `tile`, `clickedColumn`, `setBezeled:`, `setSelectable:`,
//! `setUsesSingleLineMode:`, `cell`, `setWraps:`, `setScrollable:`,
//! `currentEditor`, `layoutManager`, `textContainer`, `textContainerInset`,
//! `ensureLayoutForTextContainer:`, `usedRectForTextContainer:` und
//! `insertNewlineIgnoringFieldEditor:`, fuer den Doppelklick ins Leere
//! `sharedApplication`, `currentEvent`, `locationInWindow`,
//! `convertPoint:fromView:` und `bounds`, dazu die hier **gebauten** Methoden
//! `numberOfRowsInTableView:` (`NSTableView.h:743`),
//! `control:textShouldEndEditing:`, `controlTextDidEndEditing:`,
//! `controlTextDidChange:` und `control:textView:doCommandBySelector:`
//! (`NSControl.h`), `tableViewColumnDidResize:` (`NSTableView.h`), die Aktion
//! `copy:` und fuer das [`Notizfeld`] `intrinsicContentSize`. Fuer den [`Zelleneditor`] stehen
//! ebenso seit 10.0 `NSUndoManager` samt `undo`, `redo`, `canUndo`, `canRedo`
//! und `removeAllActions`, `setAllowsUndo:` und `setFieldEditor:`
//! (`NSTextView.h`, `NSText.h:93`) und die **gebauten** Methoden `undoManager`
//! (`NSResponder.h:309`), `undo:`, `redo:` und `becomeFirstResponder`
//! (`NSResponder.h:105`). Die Abschaltung der Automatiken am [`Zelleneditor`]
//! beruehrt ihre Setzer nicht in dieser Datei, sondern ueber
//! [`super::textautomatik::automatiken_abschalten`]; deren Alter fuehrt der
//! Kopf jenes Moduls (die juengsten seit 15.0, einer ueber der Untergrenze und
//! deshalb erst nach einer Frage an die Laufzeit gesetzt). Die Konstanten
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
//! - `NSLayoutConstraint` samt `setActive:`, `setTranslatesAutoresizingMaskIntoConstraints:`,
//!   `setContentCompressionResistancePriority:forOrientation:`,
//!   `invalidateIntrinsicContentSize`, der Konstante `NSLayoutPriorityRequired`
//!   und der Aufzaehlung `NSLayoutConstraintOrientation` seit 10.7
//!   (`NSLayoutConstraint.h`), `setPreferredMaxLayoutWidth:` seit 10.8
//!   (`NSTextField.h`), `setMaximumNumberOfLines:` seit 10.11
//!   (`NSTextField.h`)
//! - `leadingAnchor`, `trailingAnchor`, `topAnchor`, `bottomAnchor` und
//!   `constraintEqualToAnchor:constant:` samt
//!   `constraintLessThanOrEqualToAnchor:constant:` seit 10.11
//!   (`NSLayoutAnchor.h`)
//! - `lineBreakMode` seit 10.10 (`NSControl.h:65`)
//! - `setUsesAutomaticRowHeights:` seit 10.13 (`NSTableView.h:574`)
//! - `NSTextField::textFieldWithString:` und
//!   `NSButton::checkboxWithTitle:target:action:` seit 10.12
//!   (`NSTextField.h:115`, `NSButton.h:59`)
//! - `NSTableRowView` seit 10.7 (`NSTableRowView.h:21`), samt seiner
//!   Eigenschaft `backgroundColor` und damit `setBackgroundColor:`
//!   (`NSTableRowView.h:61`, ohne eigene Angabe), und die **gebaute** Methode
//!   `tableView:didAddRowView:forRow:` seit 10.7 (`NSTableView.h:601`), fuer
//!   die Hervorhebung des heutigen Tages in der Termintabelle
//! - `NSColor` samt `colorWithAlphaComponent:` (`NSColor.h:311`) und
//!   `backgroundColor` von `NSTableView` (`NSTableView.h:189`) ohne Angabe und
//!   damit seit 10.0; `systemYellowColor` seit 10.10 (`NSColor.h:254`); allein
//!   in einer Probe `selectedContentBackgroundColor` und
//!   `unemphasizedSelectedContentBackgroundColor` seit 10.14 (`NSColor.h:221`
//!   und `:222`) und `isEqual:` aus `NSObjectProtocol`
//! - fuer den Kopf der Termintabelle `NSImage` samt `imageNamed:`
//!   (`NSImage.h:77`), `setIndicatorImage:inTableColumn:` (`NSTableView.h:287`),
//!   `tableColumnWithIdentifier:` (`:242`), `setAllowsColumnSelection:`
//!   (`:334`) und die **gebaute** Methode `tableView:didClickTableColumn:`
//!   (`:659`), alle ohne Angabe und damit seit 10.0
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
use objc2::runtime::{AnyObject, Bool, ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSApplication, NSAutoresizingMaskOptions, NSButton, NSColor, NSControl, NSControlStateValueOff,
    NSControlStateValueOn, NSControlTextEditingDelegate, NSFont, NSImage,
    NSLayoutConstraintOrientation, NSLayoutPriorityRequired, NSLineBreakMode, NSResponder,
    NSScrollView, NSTableCellView, NSTableColumn, NSTableColumnResizingOptions, NSTableHeaderView,
    NSTableRowView, NSTableView, NSTableViewColumnAutoresizingStyle, NSTableViewDataSource,
    NSTableViewDelegate, NSTableViewStyle, NSText, NSTextField, NSTextFieldDelegate, NSTextView,
    NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSIndexSet, NSInteger, NSNotification, NSObject, NSObjectProtocol, NSPoint,
    NSRect, NSSize, NSString, NSUndoManager, ns_string,
};

use krk_core::heimordner::eintraege::{Aufgaben, Notizen, Tag, aufgabenzeile, termine};
use krk_core::verzeichnis::Richtung;

use super::{textautomatik, zwischenablage};

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

/// Die erste Spalte: der Aufgabentext, in der Notiztabelle das Thema.
pub const THEMENSPALTE: usize = 0;

/// Die zweite Spalte der Notiztabelle, der Notiztext; die Aufgabentabelle hat
/// sie nicht.
pub const NOTIZSPALTE: usize = 1;

/// Die erste Spalte der Termintabelle, das Datum: dieselbe Spalte wie
/// [`THEMENSPALTE`], denn das Datum ist das Thema eines Termins.
pub const DATUMSSPALTE: usize = THEMENSPALTE;

/// Die zweite Spalte der Termintabelle, der Termintext: dieselbe Spalte wie
/// [`NOTIZSPALTE`].
pub const TERMINSPALTE: usize = NOTIZSPALTE;

/// Die Breite, mit der die Datumsspalte entsteht: schmaler als die
/// Themenspalte, weil `YYMMDD HH:MM` zwoelf Zeichen hat.
const DATUMSBREITE: f64 = 110.0;

/// Die Breite, mit der die Themenspalte entsteht; der Nutzer kann sie ziehen,
/// die Notizspalte nimmt den Rest.
const THEMENBREITE: f64 = 160.0;

/// Der Abstand eines Notizfeldes zu den Kanten seiner Zelle, waagrecht.
const RAND: f64 = 4.0;

/// Der Abstand eines Notizfeldes zu den Kanten seiner Zelle, senkrecht.
const RAND_OBEN: f64 = 2.0;

/// Welche Art von Eintraegen die Tabelle zeigt.
///
/// Die Geheimnisse aus Stufe 5 sind der Darstellung nach Notizen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eintragsart {
    /// Eine Spalte mit Kaestchen und Text, `tasks.txt`.
    Aufgaben,
    /// Zwei Spalten, Thema und Notiz, `notes.txt`.
    Notizen,
    /// Zwei Spalten, Datum und Termin, `appointments.md`, nach dem Datum
    /// geordnet.
    Termine,
}

/// Die abgeleiteten Zeilen, je Art in ihrer eigenen Gestalt.
///
/// **Die Art steckt in den Zeilen und nirgends daneben**: welche Spalten die
/// Tabelle traegt, folgt aus dem Wert, den [`Eintragsansicht::zeilen_zeigen`]
/// zuletzt bekommen hat, und nicht aus einem zweiten Feld, das mit ihm
/// auseinanderlaufen koennte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zeilen {
    /// Die Zeilen der Aufgabentabelle.
    Aufgaben(Vec<Eintragszeile>),
    /// Die Zeilen der Notiztabelle.
    Notizen(Vec<Notizzeile>),
    /// Die Zeilen der Termintabelle, in der Folge des Schirms.
    Termine(Vec<Terminzeile>),
}

impl Zeilen {
    /// Die Art dieser Zeilen.
    #[must_use]
    pub fn art(&self) -> Eintragsart {
        match self {
            Self::Aufgaben(_) => Eintragsart::Aufgaben,
            Self::Notizen(_) => Eintragsart::Notizen,
            Self::Termine(_) => Eintragsart::Termine,
        }
    }

    /// Wie viele Zeilen es sind.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Aufgaben(zeilen) => zeilen.len(),
            Self::Notizen(zeilen) => zeilen.len(),
            Self::Termine(zeilen) => zeilen.len(),
        }
    }

    /// Die Stelle im Stand, die AppKits Zeile `zeile` zeigt; `None` jenseits
    /// der letzten Zeile.
    ///
    /// **Eine der zwei Stellen der Umrechnung** (Modulkopf, „Stelle und
    /// Zeile"). Aufgaben und Notizen stehen in der Reihenfolge der Datei, und
    /// dort ist es die Gleichheit; eine Terminzeile nennt ihre Stelle selbst.
    #[must_use]
    pub fn stelle_der_zeile(&self, zeile: usize) -> Option<usize> {
        match self {
            Self::Aufgaben(_) | Self::Notizen(_) => (zeile < self.len()).then_some(zeile),
            Self::Termine(zeilen) => zeilen.get(zeile).map(|termin| termin.stelle),
        }
    }

    /// Die Zeile, in der AppKit die Stelle `stelle` zeigt; `None`, wenn es sie
    /// nicht gibt.
    ///
    /// Die Umkehrung von [`Self::stelle_der_zeile`], und fuer Aufgaben und
    /// Notizen ebenso die Gleichheit; fuer Termine die Zeile, deren Stelle es
    /// ist.
    #[must_use]
    pub fn zeile_der_stelle(&self, stelle: usize) -> Option<usize> {
        match self {
            Self::Aufgaben(_) | Self::Notizen(_) => (stelle < self.len()).then_some(stelle),
            Self::Termine(zeilen) => zeilen.iter().position(|termin| termin.stelle == stelle),
        }
    }
}

impl From<Vec<Eintragszeile>> for Zeilen {
    fn from(zeilen: Vec<Eintragszeile>) -> Self {
        Self::Aufgaben(zeilen)
    }
}

impl From<Vec<Notizzeile>> for Zeilen {
    fn from(zeilen: Vec<Notizzeile>) -> Self {
        Self::Notizen(zeilen)
    }
}

impl From<Vec<Terminzeile>> for Zeilen {
    fn from(zeilen: Vec<Terminzeile>) -> Self {
        Self::Termine(zeilen)
    }
}

/// Eine Zeile der Termintabelle, abgeleitet aus einem Block des Standes.
///
/// **Sie traegt ihre Stelle**, weil die Termintabelle nach dem Datum ordnet
/// und die Zeile auf dem Schirm deshalb nicht die Stelle im Stand ist
/// (Modulkopf, „Stelle und Zeile").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Terminzeile {
    /// Die Stelle des Termins im Stand, gezaehlt wie `Neustand::auswahl`.
    pub stelle: usize,
    /// Der Datumstext der Kopfzeile, ohne `## ` davor, auch ein ungueltiger.
    pub datum: String,
    /// Der Termintext, Zeilen durch `\n` getrennt, wie der Text einer
    /// [`Notizzeile`].
    pub text: String,
    /// Ob der Termin am heutigen Tag liegt, mit oder ohne Uhrzeit; die Zeile
    /// ist dann hervorgehoben ([`heute_farbe`], Schritt 7 des Plans der
    /// Termine). Ein Feld der Zeile und nicht eine Frage beim Zeichnen: eine
    /// geaenderte Hervorhebung ist damit eine geaenderte Zeilenfolge, und
    /// [`Eintragsansicht::zeilen_zeigen`] laedt neu.
    pub heute: bool,
}

/// Die Zeilen der Termintabelle zu einem Stand von `appointments.md`, in der
/// Folge des Schirms.
///
/// **Die eine Stelle der Ableitung fuer Termine**: die Folge ist
/// `termine::anzeigeordnung` in der genannten Richtung, Datum und Text kommen
/// ueber `Notizen::notiz` wie in [`notizzeilen`], und jede Zeile nennt ihre
/// Stelle. `heute` ist der Tag, dessen Termine hervorgehoben sind; die Uhr
/// liest der Rufer ueber `termine::heute`, damit diese Ableitung ohne Uhr
/// pruefbar bleibt, und ohne Tag ist keine Zeile hervorgehoben.
#[must_use = "die Zeilen sind die ganze Auskunft"]
pub fn terminzeilen(stand: &str, richtung: Richtung, heute: Option<Tag>) -> Vec<Terminzeile> {
    let notizen = Notizen::lesen(stand);
    termine::anzeigeordnung(stand, richtung)
        .into_iter()
        .filter_map(|stelle| {
            notizen.notiz(stelle).map(|notiz| Terminzeile {
                stelle,
                heute: heute.is_some_and(|tag| termine::ist_heute(notiz.thema, tag)),
                datum: notiz.thema.to_owned(),
                text: notiz.text,
            })
        })
        .collect()
}

/// Eine Zeile der Notiztabelle, abgeleitet aus einem Block des Standes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notizzeile {
    /// Das Thema, ohne `## ` davor.
    pub thema: String,
    /// Der Notiztext, Zeilen durch `\n` getrennt, ohne Schlussumbruch und
    /// ohne den Trenner zur naechsten Notiz (`krk_core::heimordner::eintraege::Notiz`).
    pub text: String,
}

/// Die Zeilen der Notiztabelle zu einem Stand von `notes.txt`.
///
/// **Die eine Stelle der Ableitung fuer Notizen**, und sie zaehlt wie der
/// Kern: eine Zeile je Block aus `Notizen::bloecke`, der Vorspann ohne Zeile,
/// Thema und Text ueber `Notizen::notiz`, dieselbe Lesart, gegen die
/// `notizen::aendern` eine unveraenderte Zelle erkennt. Damit ist die Stelle
/// einer Zeile die Zahl, die jede Handlung des Kerns als `index` fuehrt.
#[must_use = "die Zeilen sind die ganze Auskunft"]
pub fn notizzeilen(stand: &str) -> Vec<Notizzeile> {
    let notizen = Notizen::lesen(stand);
    (0..notizen.bloecke().len())
        .filter_map(|index| notizen.notiz(index))
        .map(|notiz| Notizzeile {
            thema: notiz.thema.to_owned(),
            text: notiz.text,
        })
        .collect()
}

/// Der Hintergrund einer Zeile der Termintabelle, deren Termin heute liegt
/// (T6, Entscheidung 4 des Plans
/// `260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md`).
///
/// **Die eine Stelle der Farbe.** `systemYellowColor` mit dem Deckungsgrad
/// 0,25: die Systemfarbe liefert AppKit je Erscheinungsbild, und der geringe
/// Deckungsgrad laesst den Text im hellen wie im dunklen Erscheinungsbild
/// lesbar. Gelb steht fuer den Textmarker. Die Auswahl zeichnet
/// `NSTableRowView` ueber den Hintergrund; eine gewaehlte Zeile zeigt also die
/// Auswahl, und sobald sie weiterwandert, erscheint die Faerbung wieder.
///
/// **Verworfen:** `findHighlightColor`, weil sie fuer dunklen Text auf hellem
/// Grund gemacht ist und im dunklen Erscheinungsbild den weissen Text
/// schluckt; `unemphasizedSelectedContentBackgroundColor`, weil sie die Farbe
/// der Auswahl einer Tabelle ohne Fokus ist und genau damit verwechselt
/// wuerde; `controlAccentColor`, weil sie die Auswahlfarbe ist. Ob Gelb auf
/// dem Referenzgeraet in beiden Erscheinungsbildern traegt, ist Nutzerabnahme
/// (T6).
#[must_use]
pub fn heute_farbe() -> Retained<NSColor> {
    NSColor::systemYellowColor().colorWithAlphaComponent(0.25)
}

/// Der Name des Systembildes, das AppKit als Pfeil einer Sortierrichtung im
/// Spaltenkopf zeigt.
#[must_use]
pub fn richtungsbild(richtung: Richtung) -> &'static NSString {
    match richtung {
        Richtung::Aufsteigend => ns_string!("NSAscendingSortIndicator"),
        Richtung::Absteigend => ns_string!("NSDescendingSortIndicator"),
    }
}

/// Was ein Befehl des Feldeditors in einer Zelle bewirkt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zellenbefehl {
    /// Der Befehl geht an AppKit wie in jedem Textfeld; `return` und `tab`
    /// beenden die Zelle dann.
    Appkit,
    /// Ein Zeilenumbruch in den Text der Zelle; die Bearbeitung laeuft weiter.
    Umbruch,
    /// Die Zelle uebernehmen und die naechste (`true`) oder vorige (`false`)
    /// Zelle der Notiztabelle in Bearbeitung setzen.
    Weiter(bool),
}

/// Die Regel fuer `return` und `tab` in einer Zelle (C5, C6).
///
/// **Rein und ohne Fenster pruefbar.** In der Notizspalte schreibt `return`
/// einen Umbruch (`260926-0112_*_was-tut-return-in-einer-notizzelle-…`,
/// Moeglichkeit 1); in der Themenspalte und in der Aufgabenzelle nicht, denn
/// beide tragen keinen Umbruch, und `return` beendet dort die Zelle. `tab` und
/// `shift+tab` wechseln in der Notiztabelle von Zelle zu Zelle; in der
/// Aufgabentabelle gibt es keine zweite Zelle der Zeile, und `tab` beendet wie
/// bisher. Die Termintabelle folgt der Notiztabelle: die Datumsspalte ist die
/// Themenspalte, die Terminspalte die Notizspalte.
#[must_use]
pub fn zellenbefehl(art: Eintragsart, spalte: usize, befehl: Sel) -> Zellenbefehl {
    match art {
        Eintragsart::Aufgaben => Zellenbefehl::Appkit,
        Eintragsart::Notizen | Eintragsart::Termine => {
            if befehl == sel!(insertNewline:) && spalte == NOTIZSPALTE {
                Zellenbefehl::Umbruch
            } else if befehl == sel!(insertTab:) {
                Zellenbefehl::Weiter(true)
            } else if befehl == sel!(insertBacktab:) {
                Zellenbefehl::Weiter(false)
            } else {
                Zellenbefehl::Appkit
            }
        }
    }
}

/// Die Zelle, in die `tab` (`vorwaerts`) oder `shift+tab` aus `zelle` fuehrt,
/// in der Notiztabelle mit diesen `zeilen`; `None` am Rand.
///
/// Zeilenweise, wie gelesen wird: Thema, Notiz, dann das Thema der
/// **naechsten Zeile auf dem Schirm**. Die Zelle kommt und geht als Stelle;
/// gezaehlt wird dazwischen in Zeilen, ueber die zwei Umrechnungen an
/// [`Zeilen`] (Modulkopf, „Stelle und Zeile").
#[must_use]
pub fn naechste_zelle(zelle: Zelle, vorwaerts: bool, zeilen: &Zeilen) -> Option<Zelle> {
    let Zelle { stelle, spalte } = zelle;
    let zeile = zeilen.zeile_der_stelle(stelle)?;
    let (ziel, spalte) = if vorwaerts {
        if spalte == THEMENSPALTE {
            (zeile, NOTIZSPALTE)
        } else {
            (zeile + 1, THEMENSPALTE)
        }
    } else if spalte == NOTIZSPALTE {
        (zeile, THEMENSPALTE)
    } else {
        (zeile.checked_sub(1)?, NOTIZSPALTE)
    };
    zeilen
        .stelle_der_zeile(ziel)
        .map(|stelle| Zelle { stelle, spalte })
}

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
    /// Die Stelle des Eintrags im Stand, gezaehlt wie `Neustand::auswahl`,
    /// und **nie** die Zeilennummer von AppKit (Modulkopf, „Stelle und
    /// Zeile").
    pub stelle: usize,
    /// Die Spalte: [`THEMENSPALTE`] oder [`NOTIZSPALTE`]; die Aufgabentabelle
    /// hat allein die erste.
    pub spalte: usize,
}

/// Der Weg der Pruefung: Zelle und getippter Text, zurueck ein Ja oder Nein.
pub type Zellenpruefung = Box<dyn Fn(Zelle, &str) -> bool>;

/// Der Weg des Festschreibens: Zelle und Text, mit dem sie geendet hat.
pub type Zellenende = Box<dyn Fn(Zelle, &str)>;

/// Was ein Doppelklick in die Tabelle ausloest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Doppelklick {
    /// Die geklickte Zelle geht in Bearbeitung.
    Zelle(Zelle),
    /// Ein neuer Eintrag entsteht, wie mit „Eintrag hinzufuegen".
    Anlegen,
    /// Nichts, etwa ein Doppelklick auf die Kopfzeile.
    Nichts,
}

/// Die Regel fuer den Doppelklick, aus der Stelle der geklickten Zeile
/// (`clickedRow`, schon umgerechnet), `clickedColumn` und der Frage, ob das
/// Ereignis in der Flaeche der Tabelle liegt.
///
/// **Rein und ohne Fenster pruefbar.** Eine Zeile mit Spalte ist ihre Zelle,
/// eine Zeile ohne Spalte ihre erste Zelle. Keine Zeile heisst entweder die
/// Flaeche unter den Zeilen, auch die ganze Flaeche einer leeren Tabelle, und
/// dann entsteht ein Eintrag; oder die Kopfzeile, fuer die AppKit dieselbe -1
/// meldet, und dann geschieht nichts: wer eine Spalte breiter ziehen will,
/// soll dabei keine Notiz anlegen.
#[must_use]
pub fn doppelklick(stelle: Option<usize>, spalte: NSInteger, in_der_tabelle: bool) -> Doppelklick {
    match stelle {
        Some(stelle) => Doppelklick::Zelle(Zelle {
            stelle,
            spalte: usize::try_from(spalte).unwrap_or(THEMENSPALTE),
        }),
        None if in_der_tabelle => Doppelklick::Anlegen,
        None => Doppelklick::Nichts,
    }
}

/// Welche Zeile nach einem Neuladen der Tabelle gewaehlt ist, aus der
/// Auswahl davor und der neuen Zeilenzahl.
///
/// **Rein und ohne Fenster pruefbar.** Eine Auswahl davor bleibt auf ihrer
/// Stelle, auf die letzte Zeile beschnitten. Ohne Auswahl davor ist es die
/// erste Zeile: jeder Befehl der Tabelle bis auf das Anlegen gilt der
/// gewaehlten Zeile, und ohne eine antworteten Bearbeiten, Verschieben,
/// Loeschen und Abhaken nach dem Oeffnen nur in der Statuszeile
/// (`issues/260926-1737_*_die-neuen-tastenkombinationen-der-eintragstabelle-wirken-beim-nutzer-nicht-und-am-nachbau-in-jeder-lage.md`).
/// Eine leere Tabelle hat keine Zeile zu waehlen.
#[must_use]
pub fn auswahl_nach_neuladen(vorher: Option<usize>, laenge: usize) -> Option<usize> {
    let letzte = laenge.checked_sub(1)?;
    Some(vorher.map_or(0, |stelle| stelle.min(letzte)))
}

/// Die Wege aus der Tabelle in den Editor.
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
    /// Das Ankreuzfeld des Eintrags an dieser **Stelle** ist angeklickt
    /// worden; umgerechnet aus der Zeile des Kaestchens.
    pub abhaken: Box<dyn Fn(usize)>,
    /// Ein Doppelklick unter die Zeilen: ein Eintrag soll entstehen
    /// ([`doppelklick`]).
    pub anlegen: Box<dyn Fn()>,
    /// Ein Klick auf den Kopf der Datumsspalte der Termintabelle: die Richtung
    /// soll sich umkehren (Entscheidung 8 des Plans
    /// `260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md`).
    /// Der Editorbereich reicht ihn als Kommando weiter, damit er denselben
    /// Weg geht wie die Taste.
    pub kopf_geklickt: Box<dyn Fn()>,
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
    /// AppKit. Diese Klasse aendert daran zweierlei: wohin `cmd+z` geht, und
    /// dass die Automatiken aus [`super::textautomatik::automatiken_abschalten`]
    /// abgeschaltet sind.
    ///
    /// **Die Abschaltung steht an zwei Zeitpunkten, und beide sind gemessen.**
    /// Ab Werk stehen an diesem Feldeditor Textersetzung, Rechtschreibkorrektur
    /// und `smartInsertDelete` an (`messung-eigener.txt` unter
    /// `spikes/zellen-rueckgaengig/`), und am 260926 auf macOS 15.7.9 ersetzte
    /// derselbe Feldeditor ohne Abschaltung `omw` durch `On my way!` — in
    /// `secrets.txt` ein still geaendertes Passwort. Beim Bau gesetzt, bleiben
    /// acht der neun Einstellungen ueber den Beginn einer Bearbeitung stehen.
    /// Die neunte nicht: AppKit setzt `writingToolsBehavior` beim Einrichten
    /// **vor** `becomeFirstResponder` auf `Limited` (2) zurueck. Deshalb ruft
    /// [`Self::wird_ersthelfer`] die eine Regel nach der Oberklasse ein zweites
    /// Mal, und danach steht sie auf `None`. Eine zweite Aufzaehlung der
    /// Schalter entsteht dabei nicht.
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
        ///
        /// **Und jede Zelle beginnt mit abgeschalteten Automatiken.** AppKit
        /// hat den Feldeditor zu diesem Zeitpunkt schon eingerichtet und dabei
        /// die Schreibwerkzeuge wieder auf `Limited` gestellt; der Ruf nach der
        /// Oberklasse ist der erste Punkt, an dem die Abschaltung das letzte
        /// Wort hat (gemessen, Doc-Kommentar der Klasse).
        // SAFETY: Die Signatur entspricht der von NSResponder
        // (`NSResponder.h:105`).
        #[unsafe(method(becomeFirstResponder))]
        fn wird_ersthelfer(&self) -> bool {
            // SAFETY: `becomeFirstResponder` der Oberklasse hat die hier
            // angenommene Signatur.
            let angenommen: bool = unsafe { msg_send![super(self), becomeFirstResponder] };
            if angenommen {
                self.ivars().removeAllActions();
                textautomatik::automatiken_abschalten(self);
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
    ///
    /// Die Abschaltung der Automatiken steht schon hier und nicht erst beim
    /// Beginn einer Bearbeitung: so traegt der gebaute Feldeditor sie, ohne
    /// dass ein Fenster ihn einrichtet, und die Probe ueber jede bearbeitbare
    /// Flaeche misst ihn wie die des Editors. Warum sie beim Beginn ein
    /// zweites Mal steht, sagt der Doc-Kommentar der Klasse.
    fn neu(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(NSUndoManager::new(mtm));
        // SAFETY: `initWithFrame:` von NSTextView hat die hier angenommene
        // Signatur; der Rahmen ist gleichgueltig, AppKit legt den Feldeditor
        // ueber das bearbeitete Feld.
        let this: Retained<Self> = unsafe { msg_send![super(this), initWithFrame: NSRect::ZERO] };
        this.setFieldEditor(true);
        this.setAllowsUndo(true);
        textautomatik::automatiken_abschalten(&this);
        this
    }
}

define_class!(
    /// Das Textfeld einer Zelle der Notiztabelle.
    ///
    /// **Es meldet waehrend der Bearbeitung die Hoehe des Feldeditortexts als
    /// Eigenhoehe**, und nur deshalb gibt es die Klasse: ohne sie meldet
    /// `NSTextField` bis zum Ende der Bearbeitung die Hoehe seines alten Werts,
    /// und die Zeile waechst beim Tippen nicht mit (gemessen, Modulkopf unter
    /// „Die mehrzeilige Notizzelle"). Ausserhalb der Bearbeitung antwortet die
    /// Oberklasse.
    // SAFETY:
    // - Die Oberklasse NSTextField stellt an eine Unterklasse keine Bedingung,
    //   die diese Klasse verletzt: sie ruft den bezeichneten Erzeuger
    //   `initWithFrame:` der Oberklasse, und die eine Ueberschreibung fragt
    //   zuerst die Oberklasse und aendert allein die Hoehe.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSTextField)]
    #[thread_kind = MainThreadOnly]
    #[ivars = ()]
    pub struct Notizfeld;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Notizfeld {}

    impl Notizfeld {
        /// Die Eigengroesse: waehrend der Bearbeitung so hoch wie der Text im
        /// Feldeditor, sonst die der Oberklasse.
        // SAFETY: Die Signatur entspricht der Eigenschaft von NSView
        // (`NSLayoutConstraint.h`): kein Argument, eine `NSSize` zurueck.
        #[unsafe(method(intrinsicContentSize))]
        fn eigengroesse(&self) -> NSSize {
            // SAFETY: `intrinsicContentSize` der Oberklasse hat die hier
            // angenommene Signatur.
            let mass: NSSize = unsafe { msg_send![super(self), intrinsicContentSize] };
            self.feldeditorhoehe()
                .map_or(mass, |hoehe| NSSize::new(mass.width, hoehe))
        }
    }
);

impl Notizfeld {
    /// Ein Feld mit dem genannten Text, noch ohne Gestalt.
    fn neu(mtm: MainThreadMarker, text: &str) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: `initWithFrame:` von NSTextField hat die hier angenommene
        // Signatur; der Rahmen ist gleichgueltig, das Feld haengt ueber Auto
        // Layout an seiner Zelle.
        let this: Retained<Self> = unsafe {
            msg_send![super(this), initWithFrame: NSRect::new(NSPoint::ZERO, NSSize::new(AUFBAUBREITE, ZEILENHOEHE))]
        };
        this.setStringValue(&NSString::from_str(text));
        this
    }

    /// Die belegte Hoehe des Textes im Feldeditor samt seinem Innenabstand,
    /// falls das Feld gerade bearbeitet wird; dieselbe Rechnung wie in der
    /// Messung (`layoutManager.usedRect(for: textContainer)` plus zweimal
    /// `textContainerInset.height`).
    fn feldeditorhoehe(&self) -> Option<f64> {
        let editor = self.currentEditor()?.downcast::<NSTextView>().ok()?;
        // SAFETY: Zwei Leser ohne Vorbedingung; der Feldeditor haelt beide
        // Objekte, solange er lebt, und die Antworten werden hier festgehalten.
        let (layout, behaelter) = unsafe { (editor.layoutManager()?, editor.textContainer()?) };
        layout.ensureLayoutForTextContainer(&behaelter);
        let belegt = layout.usedRectForTextContainer(&behaelter).size.height;
        Some((belegt + 2.0 * editor.textContainerInset().height).ceil())
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
    /// Die Kopfzeile mit den Spaltennamen; eingehaengt allein in der
    /// Notiztabelle, die Aufgabentabelle traegt keine.
    kopfzeile: Retained<NSTableHeaderView>,
    /// Die zuletzt abgeleiteten Zeilen.
    ///
    /// Keine zweite Wahrheit neben dem Stand: sie werden allein von
    /// [`Eintragsansicht::zeilen_zeigen`] geschrieben, und das bekommt sie aus
    /// [`aufgabenzeilen`] oder [`notizzeilen`] ueber den Stand des Editors.
    zeilen: RefCell<Zeilen>,
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
    /// Die Richtung, die der Pfeil im Kopf der Datumsspalte zeigt.
    ///
    /// Keine zweite Wahrheit neben dem Editorbereich, der die Richtung haelt:
    /// geschrieben allein von [`Eintragsansicht::terminrichtung_zeigen`], und
    /// gebraucht, weil die Spalten beim Wechsel der Art neu entstehen und ihr
    /// Pfeil dann neu zu setzen ist.
    terminrichtung: Cell<Richtung>,
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

        /// Der Feldeditor fragt, ob jemand anders diesen Befehl uebernimmt:
        /// `return` und `tab` in der Notiztabelle, nach [`zellenbefehl`].
        // SAFETY: Die Signatur entspricht der des Protokolls (`NSControl.h`),
        // wie beim Eingabewaechter der Blaetter.
        #[unsafe(method(control:textView:doCommandBySelector:))]
        fn befehl_der_zelle(&self, feld: &NSControl, feldeditor: &NSTextView, befehl: Sel) -> Bool {
            Bool::new(self.zellenbefehl_ausfuehren(feld, feldeditor, befehl))
        }

        /// Ein Anschlag in einer Zelle: das Notizfeld misst seine Hoehe neu.
        // SAFETY: Die Signatur entspricht der des Protokolls (`NSControl.h`).
        #[unsafe(method(controlTextDidChange:))]
        fn text_geaendert(&self, meldung: &NSNotification) {
            let feld = meldung
                .object()
                .and_then(|feld| feld.downcast::<Notizfeld>().ok());
            if let Some(feld) = feld {
                feld.invalidateIntrinsicContentSize();
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
            tabelle: &NSTableView,
            spalte: Option<&NSTableColumn>,
            zeile: NSInteger,
        ) -> Option<Retained<NSView>> {
            let spalte = spalte.map_or(Ok(THEMENSPALTE), |spalte| {
                usize::try_from(tabelle.columnWithIdentifier(&spalte.identifier()))
            });
            spalte.ok().and_then(|spalte| self.zellenansicht(zeile, spalte))
        }

        /// Eine Zeilenansicht ist in die Tabelle gekommen, neu gebaut oder
        /// wiederverwendet: sie bekommt den Hintergrund des heutigen Tages,
        /// wenn ihre Zeile einen Termin von heute zeigt, und sonst den der
        /// Tabelle zurueck (Schritt 7 des Plans der Termine).
        ///
        /// **Zurueckgesetzt wird immer**, weil AppKit Zeilenansichten
        /// wiederverwendet: eine Ansicht, die eben eine heutige Zeile trug,
        /// behielte sonst ihre Farbe in einer anderen. Der Hintergrund der
        /// Tabelle ist der Vorgabewert der Eigenschaft (`NSTableRowView.h`).
        /// Gelesen an AppKits Zeile, denn die abgeleiteten Zeilen stehen in der
        /// Folge des Schirms.
        // SAFETY: Die Signatur entspricht der des Protokolls
        // (`NSTableView.h:601`).
        #[unsafe(method(tableView:didAddRowView:forRow:))]
        fn zeilenansicht_hinzugefuegt(
            &self,
            tabelle: &NSTableView,
            zeilenansicht: &NSTableRowView,
            zeile: NSInteger,
        ) {
            let heute = usize::try_from(zeile)
                .ok()
                .is_some_and(|zeile| self.zeigt_heute(zeile));
            let farbe = if heute {
                heute_farbe()
            } else {
                tabelle.backgroundColor()
            };
            zeilenansicht.setBackgroundColor(&farbe);
        }

        /// Ein Klick auf einen Spaltenkopf (Entscheidung 8 des Plans der
        /// Termine): allein der Kopf der Datumsspalte der Termintabelle tut
        /// etwas, und zwar dasselbe wie `cmd+1`.
        ///
        /// **Erst der Fokus, dann das Kommando**: der Klick auf den Kopf holt
        /// den Ersthelfer nicht von selbst, und das Kommando wirkt allein mit
        /// dem Fokus im Editor. Der Fokus kommt ueber
        /// [`Eintragsansicht::fokus_in_die_tabelle`], also ueber
        /// `makeFirstResponder:` des Fensters; das Kommando geht ueber
        /// [`Zellenwege::kopf_geklickt`] an den Editorbereich und von dort als
        /// Kommando an den Anwendungsdelegierten, also durch dieselbe
        /// Zulaessigkeit wie die Taste.
        // SAFETY: Die Signatur entspricht der des Protokolls
        // (`NSTableView.h:659`).
        #[unsafe(method(tableView:didClickTableColumn:))]
        fn kopf_geklickt(&self, tabelle: &NSTableView, spalte: &NSTableColumn) {
            if self.art() != Eintragsart::Termine
                || usize::try_from(tabelle.columnWithIdentifier(&spalte.identifier()))
                    != Ok(DATUMSSPALTE)
            {
                return;
            }
            self.fokus_in_die_tabelle();
            if let Some(wege) = self.ivars().wege.borrow().as_ref() {
                (wege.kopf_geklickt)();
            }
        }

        /// Eine Spalte hat ihre Breite geaendert: die Notizfelder brechen ab
        /// jetzt an der neuen Breite um.
        // SAFETY: Die Signatur entspricht der des Protokolls (`NSTableView.h`).
        #[unsafe(method(tableViewColumnDidResize:))]
        fn spalte_geaendert(&self, _meldung: &NSNotification) {
            self.umbruchbreite_nachziehen();
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
        ///
        /// **Danach liegt der Fokus in der Tabelle**, wie nach einem Klick auf
        /// den Text der Zeile ([`Eintragsansicht::fokus_in_die_tabelle`]).
        /// Das Kaestchen selbst nimmt den Rang nicht an, und ohne diesen
        /// Schritt blieb er, wo er war, etwa im Dateifenster, und die Tasten
        /// der Tabelle gingen ins Leere. Erst nach dem Abhaken, weil das eine
        /// laufende Zelle zuerst uebernimmt und eine abgewiesene stehen laesst.
        // SAFETY: Die Signatur ist die einer Aktion mit dem Absender als
        // Argument; Absender ist allein das Ankreuzfeld aus `zellenansicht`.
        #[unsafe(method(kastenGeklickt:))]
        fn kasten_geklickt(&self, kasten: &NSButton) {
            let Some(stelle) = self.stelle_an(self.ivars().tabelle.rowForView(kasten)) else {
                return;
            };
            if let Some(wege) = self.ivars().wege.borrow().as_ref() {
                (wege.abhaken)(stelle);
            }
            self.fokus_in_die_tabelle();
            if let Some(zeile) = self
                .stelle_an(self.ivars().tabelle.rowForView(kasten))
                .and_then(|stelle| self.zeile_von(stelle))
                && let Zeilen::Aufgaben(zeilen) = &*self.ivars().zeilen.borrow()
                && let Some(eintrag) = zeilen.get(zeile)
            {
                kasten.setState(kastenzustand(eintrag.erledigt));
            }
        }

        /// Doppelklick in die Tabelle: auf eine Zeile ihre Zelle in
        /// Bearbeitung, darunter ein neuer Eintrag ([`doppelklick`]).
        // SAFETY: Die Signatur ist die einer Aktion; der Absender ist die
        // Tabelle und wird nicht gebraucht.
        #[unsafe(method(zeileDoppelt:))]
        fn zeile_doppelt(&self, _absender: Option<&AnyObject>) {
            let tabelle = &self.ivars().tabelle;
            let in_der_tabelle = self.ereignis_in_der_tabelle();
            self.doppelklick_ausfuehren(doppelklick(
                self.stelle_an(tabelle.clickedRow()),
                tabelle.clickedColumn(),
                in_der_tabelle,
            ));
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
        tabelle.setStyle(NSTableViewStyle::FullWidth);
        tabelle.setAllowsEmptySelection(true);
        tabelle.setAllowsMultipleSelection(false);
        // Ein Klick auf einen Spaltenkopf waehlt keine Spalte; in der
        // Termintabelle kehrt er die Richtung um (Schritt 8 des Plans der
        // Termine).
        tabelle.setAllowsColumnSelection(false);
        // Die letzte Spalte nimmt jede Aenderung der Breite: in der
        // Aufgabentabelle die einzige, in der Notiztabelle die Notiz, und das
        // Thema behaelt die Breite, die der Nutzer ihm gezogen hat.
        tabelle.setColumnAutoresizingStyle(
            NSTableViewColumnAutoresizingStyle::LastColumnOnlyAutoresizingStyle,
        );
        // Die Kopfzeile, die AppKit der Tabelle mitgibt, wird gehalten und
        // allein in der Notiztabelle eingehaengt; `spalten_einrichten` nimmt
        // sie der Aufgabentabelle gleich wieder ab.
        let kopfzeile = tabelle.headerView().unwrap_or_else(|| {
            NSTableHeaderView::initWithFrame(
                NSTableHeaderView::alloc(mtm),
                NSRect::new(NSPoint::ZERO, NSSize::new(AUFBAUBREITE, ZEILENHOEHE)),
            )
        });

        let rolle = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), rahmen);
        rolle.setHasVerticalScroller(true);
        rolle.setAutohidesScrollers(true);
        rolle.setDocumentView(Some(&tabelle));
        rolle.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );
        rolle.setHidden(true);

        spalten_einrichten(mtm, &tabelle, &rolle, &kopfzeile, Eintragsart::Aufgaben);

        let this = Self::alloc(mtm).set_ivars(EintragsansichtIvars {
            rolle,
            tabelle,
            kopfzeile,
            zeilen: RefCell::new(Zeilen::Aufgaben(Vec::new())),
            wege: RefCell::new(None),
            verwerfen: Cell::new(false),
            endet: Cell::new(false),
            zelleneditor: Zelleneditor::neu(mtm),
            terminrichtung: Cell::new(Richtung::Aufsteigend),
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

    /// Traegt die Wege in den Editor ein (siehe [`Zellenwege`]).
    pub fn wege_setzen(&self, wege: Zellenwege) {
        *self.ivars().wege.borrow_mut() = Some(wege);
    }

    /// Fuehrt aus, was die Regel [`doppelklick`] entschieden hat.
    ///
    /// `pub(super)` allein fuer die Proben in [`super::editor`]: ein
    /// Doppelklick braucht ein Ereignis, das `libtest` nicht hergibt, und die
    /// Proben reichen deshalb das Urteil herein.
    pub(super) fn doppelklick_ausfuehren(&self, urteil: Doppelklick) {
        match urteil {
            Doppelklick::Zelle(zelle) => {
                let _ = self.zelle_beginnen(zelle);
            }
            Doppelklick::Anlegen => {
                if let Some(wege) = self.ivars().wege.borrow().as_ref() {
                    (wege.anlegen)();
                }
            }
            Doppelklick::Nichts => {}
        }
    }

    /// Ob das laufende Ereignis in der Flaeche der Tabelle liegt und nicht
    /// etwa auf ihrer Kopfzeile; ohne Ereignis nein.
    fn ereignis_in_der_tabelle(&self) -> bool {
        let tabelle = &self.ivars().tabelle;
        let Some(ereignis) = NSApplication::sharedApplication(self.ivars().mtm).currentEvent()
        else {
            return false;
        };
        let ort = tabelle.convertPoint_fromView(ereignis.locationInWindow(), None);
        let flaeche = tabelle.bounds();
        ort.x >= flaeche.origin.x
            && ort.y >= flaeche.origin.y
            && ort.x < flaeche.origin.x + flaeche.size.width
            && ort.y < flaeche.origin.y + flaeche.size.height
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
    /// auf der neuen letzten und nicht auf keiner. **War keine gewaehlt, ist
    /// es danach die erste Zeile**, nach der Regel [`auswahl_nach_neuladen`]:
    /// nach dem Oeffnen, nach dem Neuladen am neuen Ort und nach einem
    /// `umkehren` ohne Auswahl. Wohin eine Handlung sie setzt, sagt der Kern
    /// ueber `Neustand::auswahl` und [`Self::auswahl_setzen`], nicht diese
    /// Funktion; `Editorbereich::umbau_anwenden` setzt sie nach dem Neuladen.
    ///
    /// **Eine offene Zelle wird zuerst verworfen**, und zwar vor dem Vergleich:
    /// gerufen wird nach jeder Aenderung des Standes, und steht dabei eine Zelle
    /// offen, die nicht selbst gerade festschreibt, dann hat sich der Stand von
    /// aussen unter ihr geaendert — ein `cmd+z` mit der Tabelle im Fokus, eine
    /// eingetroffene Datei. `reloadData` beendete sie sonst mit Zeile -1, und
    /// der Text fiele still (Modulkopf, "Solange eine Zelle laeuft"). Verworfen
    /// und nicht uebernommen, weil der Stand darunter nicht mehr der ist, gegen
    /// den sie begonnen hat.
    ///
    /// **Wechselt die Art, werden zuerst die Spalten neu eingerichtet**, und
    /// die Auswahl faellt: eine Stelle in den Aufgaben meint keine Notiz.
    pub fn zeilen_zeigen(&self, zeilen: impl Into<Zeilen>) {
        let zeilen = zeilen.into();
        self.fremde_zelle_verwerfen();
        if *self.ivars().zeilen.borrow() == zeilen {
            return;
        }
        let laenge = zeilen.len();
        let art = zeilen.art();
        let artwechsel = self.art() != art;
        let vorher = if artwechsel {
            None
        } else {
            self.gewaehlte_stelle()
        };
        // Die Ausleihe endet an ihrem Semikolon: `reloadData` fragt die Zeilen
        // gleich wieder ab.
        *self.ivars().zeilen.borrow_mut() = zeilen;
        if artwechsel {
            let ivars = self.ivars();
            spalten_einrichten(
                ivars.mtm,
                &ivars.tabelle,
                &ivars.rolle,
                &ivars.kopfzeile,
                art,
            );
            self.richtungspfeil_setzen();
        }
        self.ivars().tabelle.reloadData();
        // Die Regel rechnet eine Auswahl davor als Stelle weiter; ohne eine
        // meint sie die erste Zeile des Schirms, und die zeigt in der
        // Termintabelle nicht die Stelle 0.
        let auswahl = match vorher {
            Some(_) => auswahl_nach_neuladen(vorher, laenge),
            None => auswahl_nach_neuladen(None, laenge)
                .and_then(|zeile| self.ivars().zeilen.borrow().stelle_der_zeile(zeile)),
        };
        self.auswahl_setzen(auswahl);
    }

    /// Ob AppKits Zeile `zeile` einen Termin von heute zeigt; ausserhalb der
    /// Termintabelle nie.
    #[must_use]
    fn zeigt_heute(&self, zeile: usize) -> bool {
        match &*self.ivars().zeilen.borrow() {
            Zeilen::Termine(zeilen) => zeilen.get(zeile).is_some_and(|termin| termin.heute),
            Zeilen::Aufgaben(_) | Zeilen::Notizen(_) => false,
        }
    }

    /// Welche Art von Eintraegen die Tabelle gerade zeigt.
    #[must_use]
    pub fn art(&self) -> Eintragsart {
        self.ivars().zeilen.borrow().art()
    }

    /// Zeigt die Richtung der Termintabelle als Pfeil im Kopf der
    /// Datumsspalte (Schritt 8 des Plans der Termine).
    ///
    /// Gerufen vom Editorbereich vor jedem Nachzug der Termintabelle; der Pfeil
    /// steht damit auch dann richtig, wenn die Zeilen gleich bleiben und
    /// [`Self::zeilen_zeigen`] nicht neu laedt. Ohne Datumsspalte, also in jeder
    /// anderen Art, wird er gemerkt und beim naechsten Einrichten der Spalten
    /// gesetzt.
    pub fn terminrichtung_zeigen(&self, richtung: Richtung) {
        self.ivars().terminrichtung.set(richtung);
        self.richtungspfeil_setzen();
    }

    /// Setzt den Pfeil der gemerkten Richtung in den Kopf der Datumsspalte,
    /// falls die Tabelle eine traegt.
    fn richtungspfeil_setzen(&self) {
        let tabelle = &self.ivars().tabelle;
        let Some(spalte) = tabelle.tableColumnWithIdentifier(ns_string!("datum")) else {
            return;
        };
        let bild = NSImage::imageNamed(richtungsbild(self.ivars().terminrichtung.get()));
        tabelle.setIndicatorImage_inTableColumn(bild.as_deref(), &spalte);
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

    /// Waehlt die Zeile, die den Eintrag an der genannten **Stelle** zeigt,
    /// und bringt sie ins Bild.
    ///
    /// `None` und eine Stelle, die keine Zeile zeigt, lassen die Auswahl, wie
    /// sie ist: eine Handlung ohne Auswahl danach hat keine gewaehlt, und eine
    /// zu grosse Zeile beantwortete AppKit mit einer Ausnahme, die Rust nicht
    /// fangen kann.
    pub fn auswahl_setzen(&self, stelle: Option<usize>) {
        let Some(zeile) = stelle.and_then(|stelle| self.zeile_von(stelle)) else {
            return;
        };
        let tabelle = &self.ivars().tabelle;
        tabelle.selectRowIndexes_byExtendingSelection(&NSIndexSet::indexSetWithIndex(zeile), false);
        tabelle.scrollRowToVisible(NSInteger::try_from(zeile).unwrap_or(NSInteger::MAX));
    }

    /// Die **Stelle** des gewaehlten Eintrags im Stand, falls einer gewaehlt
    /// ist; nie die Zeilennummer (Modulkopf, „Stelle und Zeile").
    #[must_use]
    pub fn gewaehlte_stelle(&self) -> Option<usize> {
        self.stelle_an(self.ivars().tabelle.selectedRow())
    }

    /// Die Stelle, die AppKits Zeile `zeile` zeigt: die Umrechnung jeder
    /// Zeilennummer, die von AppKit hereinkommt. -1 und eine Zeile jenseits
    /// der letzten ergeben `None`.
    fn stelle_an(&self, zeile: NSInteger) -> Option<usize> {
        let zeile = usize::try_from(zeile).ok()?;
        self.ivars().zeilen.borrow().stelle_der_zeile(zeile)
    }

    /// Die Zeile, die den Eintrag an `stelle` zeigt: die Umrechnung jeder
    /// Stelle, die zu AppKit hinausgeht.
    fn zeile_von(&self, stelle: usize) -> Option<usize> {
        self.ivars().zeilen.borrow().zeile_der_stelle(stelle)
    }

    /// Der abgeleitete Text des gewaehlten Eintrags, fuer `copy:`.
    ///
    /// Von einer Notiz ihr Text, und das Thema nur, wenn sie keinen Text
    /// traegt: kopiert wird, was der Nutzer woanders einsetzen will, und bei
    /// einer Notiz ist das der Inhalt und nicht die Ueberschrift.
    ///
    /// Von einem Termin ebenso sein Text, und das Datum nur ohne Text.
    #[must_use]
    fn gewaehlter_text(&self) -> Option<String> {
        let zeile = self.zeile_von(self.gewaehlte_stelle()?)?;
        let text_oder = |text: &str, kopf: &str| {
            if text.is_empty() {
                kopf.to_owned()
            } else {
                text.to_owned()
            }
        };
        match &*self.ivars().zeilen.borrow() {
            Zeilen::Aufgaben(zeilen) => zeilen.get(zeile).map(|eintrag| eintrag.text.clone()),
            Zeilen::Notizen(zeilen) => zeilen
                .get(zeile)
                .map(|notiz| text_oder(&notiz.text, &notiz.thema)),
            Zeilen::Termine(zeilen) => zeilen
                .get(zeile)
                .map(|termin| text_oder(&termin.text, &termin.datum)),
        }
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
        let stelle = self.stelle_an(tabelle.rowForView(ansicht))?;
        let spalte = usize::try_from(tabelle.columnForView(ansicht)).ok()?;
        Some(Zelle { stelle, spalte })
    }

    /// Setzt die erste Zelle des Eintrags an der genannten **Stelle** in
    /// Bearbeitung, der Text ist ausgewaehlt: den Aufgabentext, in der
    /// Notiztabelle das Thema.
    ///
    /// Liefert `false`, wenn es den Eintrag nicht gibt.
    pub fn bearbeitung_beginnen(&self, stelle: usize) -> bool {
        self.zelle_beginnen(Zelle {
            stelle,
            spalte: THEMENSPALTE,
        })
    }

    /// Setzt die genannte Zelle in Bearbeitung, der Text ist ausgewaehlt.
    ///
    /// Derselbe Weg wie das Umbenennen in [`super::tabelle`]:
    /// `editColumn:row:withEvent:select:` macht den Feldeditor zum Ersthelfer
    /// und stellt ihn in das Textfeld der Zelle, an der Zeile, die die Stelle
    /// der Zelle zeigt. Liefert `false`, wenn es den Eintrag oder die Spalte
    /// nicht gibt.
    pub fn zelle_beginnen(&self, zelle: Zelle) -> bool {
        let Some((zeile, spalte)) = self.ziel_der_zelle(zelle) else {
            return false;
        };
        let tabelle = &self.ivars().tabelle;
        tabelle.scrollRowToVisible(zeile);
        tabelle.editColumn_row_withEvent_select(spalte, zeile, None, true);
        true
    }

    /// Zeile und Spalte, an denen AppKit die genannte Zelle zeigt: die Zeile
    /// ueber [`Self::zeile_von`] aus der Stelle der Zelle. `None`, wenn es den
    /// Eintrag oder die Spalte nicht gibt.
    ///
    /// `pub(super)` allein fuer die Proben in [`super::editor`]: der Beginn
    /// einer Bearbeitung braucht ein Fenster, das `libtest` nicht hergibt, und
    /// die Proben fahren deshalb die Umrechnung davor.
    #[must_use]
    pub(super) fn ziel_der_zelle(&self, zelle: Zelle) -> Option<(NSInteger, NSInteger)> {
        let zeile = NSInteger::try_from(self.zeile_von(zelle.stelle)?).ok()?;
        let spalte = NSInteger::try_from(zelle.spalte).ok()?;
        (spalte < self.ivars().tabelle.numberOfColumns()).then_some((zeile, spalte))
    }

    /// Beendet eine laufende Bearbeitung **uebernehmend**: die Tabelle nimmt
    /// den Ersthelfer, und AppKit fragt dabei die beiden Delegiertenmethoden.
    ///
    /// Liefert, ob AppKit den Wechsel angenommen hat. `false` heisst: die
    /// Pruefung hat den Text abgewiesen, und die Zelle steht weiter in
    /// Bearbeitung. Zwei Rufer: `Editorbereich::zelle_uebernehmen` fuer KRKs
    /// eigene Wege und [`Self::zellenbefehl_ausfuehren`] fuer `tab` in der
    /// Notiztabelle, der dort den Weg von AppKit ersetzt und nicht einen der
    /// Wege, die `zelle_uebernehmen` sammelt.
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

    /// Holt den Fokus in die Tabelle, falls ihn nicht schon sie oder eine
    /// ihrer Zellen haelt: der zweite Schritt eines Klicks ins Ankreuzfeld.
    ///
    /// **Ueber `makeFirstResponder:` des Fensters**, und damit durch die
    /// Ueberschreibung in `super::fenster`, deren Melder Fokusanzeige und
    /// aktiven Bereich nachzieht wie bei jedem anderen Wechsel. Getauscht wird
    /// nichts: die Tabelle ist zu sehen, sonst gaebe es kein Kaestchen zum
    /// Anklicken. Haelt eine Zelle den Rang, bleibt er bei ihr; eine
    /// abgewiesene Zelle nach dem Abhaken beendete der Wechsel sonst ein
    /// zweites Mal.
    fn fokus_in_die_tabelle(&self) {
        let tabelle = &self.ivars().tabelle;
        let Some(fenster) = tabelle.window() else {
            return;
        };
        let ersthelfer = fenster.firstResponder();
        if !self.haelt_den_fokus(ersthelfer.as_deref()) {
            let _ = fenster.makeFirstResponder(Some(tabelle));
        }
    }

    /// Ob dieser Ersthelfer die Tabelle ist oder unter ihr liegt, der
    /// Feldeditor einer laufenden Zelle eingeschlossen.
    ///
    /// `pub(super)` allein fuer die Proben in [`super::editor`], die kein
    /// Fenster haben und den Ersthelfer deshalb hereinreichen.
    #[must_use]
    pub(super) fn haelt_den_fokus(&self, ersthelfer: Option<&NSResponder>) -> bool {
        let tabelle: &NSView = &self.ivars().tabelle;
        ersthelfer
            .and_then(|ersthelfer| ersthelfer.downcast_ref::<NSView>())
            .is_some_and(|ansicht| ansicht.isDescendantOf(tabelle))
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
        let Some(zelle) = self.zelle_von(feld) else {
            return true;
        };
        self.ivars()
            .wege
            .borrow()
            .as_ref()
            .is_none_or(|wege| (wege.pruefen)(zelle, text))
    }

    /// Die Antwort auf `controlTextDidEndEditing:`.
    ///
    /// **Das Feld zeigt danach die Ableitung.** Nach einem Umbau hat
    /// `reloadData` die Zelle ohnehin neu gebaut, und das Feld steht in keiner
    /// Zeile mehr; nach einem Ende ohne Umbau — verworfen, unveraendert, oder
    /// ein Ende, bei dem AppKit nicht gefragt hat — stuende sonst der getippte
    /// Text da und behauptete eine Aenderung, die nicht im Stand steht.
    ///
    /// **Ein Notizfeld misst danach neu**: seine Eigenhoehe war waehrend der
    /// Bearbeitung die des Feldeditors, und ohne den Anstoss behielte eine
    /// Zeile ohne Umbau die Hoehe des getippten Textes.
    pub(super) fn zelle_geendet(&self, feld: &NSTextField) {
        if !self.ivars().verwerfen.get()
            && let Some(zelle) = self.zelle_von(feld)
        {
            let text = feld.stringValue().to_string();
            if let Some(wege) = self.ivars().wege.borrow().as_ref() {
                self.ivars().endet.set(true);
                (wege.festschreiben)(zelle, &text);
                self.ivars().endet.set(false);
            }
        }
        if let Some(zelle) = self.zelle_von(feld)
            && let Some(text) = self.abgeleiteter_text(zelle)
        {
            feld.setStringValue(&NSString::from_str(&text));
        }
        feld.invalidateIntrinsicContentSize();
    }

    /// Der abgeleitete Text einer Zelle, wie die Tabelle ihn zeigt; gelesen an
    /// der Zeile, die die Stelle der Zelle zeigt.
    fn abgeleiteter_text(&self, zelle: Zelle) -> Option<String> {
        let zeile = self.zeile_von(zelle.stelle)?;
        match &*self.ivars().zeilen.borrow() {
            Zeilen::Aufgaben(zeilen) => zeilen.get(zeile).map(|eintrag| eintrag.text.clone()),
            Zeilen::Notizen(zeilen) => zeilen.get(zeile).map(|notiz| {
                if zelle.spalte == NOTIZSPALTE {
                    notiz.text.clone()
                } else {
                    notiz.thema.clone()
                }
            }),
            Zeilen::Termine(zeilen) => zeilen.get(zeile).map(|termin| {
                if zelle.spalte == TERMINSPALTE {
                    termin.text.clone()
                } else {
                    termin.datum.clone()
                }
            }),
        }
    }

    /// Fuehrt einen Befehl des Feldeditors nach [`zellenbefehl`] aus; `true`
    /// heisst, er ist hier beantwortet und geht nicht an AppKit.
    ///
    /// **`tab` uebernimmt ueber [`Self::bearbeitung_beenden`]**, also ueber die
    /// zwei Delegiertenwege wie jedes andere Ende, und beginnt die naechste
    /// Zelle erst, wenn AppKit den Wechsel angenommen hat; eine abgewiesene
    /// Zelle bleibt offen, und ihren Grund hat die Pruefung schon gemeldet.
    /// Beantwortet ist die Taste in beiden Faellen: ginge sie an AppKit, liefe
    /// der Fokus zur naechsten Schluesselansicht des Fensters und aus der
    /// Tabelle hinaus.
    fn zellenbefehl_ausfuehren(
        &self,
        feld: &NSControl,
        feldeditor: &NSTextView,
        befehl: Sel,
    ) -> bool {
        let Some(zelle) = self.zelle_von(feld) else {
            return false;
        };
        match zellenbefehl(self.art(), zelle.spalte, befehl) {
            Zellenbefehl::Appkit => false,
            Zellenbefehl::Umbruch => {
                // SAFETY: Eine Aktion mit optionalem Absender; der Feldeditor
                // ist der, der den Befehl gerade meldet, und lebt ueber den
                // Aufruf hinaus.
                let _: () = unsafe {
                    msg_send![feldeditor, insertNewlineIgnoringFieldEditor: Option::<&AnyObject>::None]
                };
                true
            }
            Zellenbefehl::Weiter(vorwaerts) => {
                let ziel = naechste_zelle(zelle, vorwaerts, &self.ivars().zeilen.borrow());
                if let Some(fenster) = self.ivars().tabelle.window()
                    && self.bearbeitung_beenden(&fenster)
                    && let Some(ziel) = ziel
                {
                    let _ = self.zelle_beginnen(ziel);
                }
                true
            }
        }
    }

    /// Die Breite, an der ein Notizfeld dieser Spalte umbricht.
    fn umbruchbreite(&self, spalte: usize) -> f64 {
        let spalten = self.ivars().tabelle.tableColumns();
        let breite = spalten
            .iter()
            .nth(spalte)
            .map_or(AUFBAUBREITE, |spalte| spalte.width());
        (breite - 2.0 * RAND).max(RAND)
    }

    /// Stellt die Umbruchbreite jedes gebauten Notizfeldes auf die Breite
    /// seiner Spalte.
    ///
    /// Gefragt werden allein die Zellen, die die Tabelle schon gebaut hat
    /// (`makeIfNecessary` nein); jede spaeter gebaute bekommt die Breite in
    /// [`Self::notizzelle`]. **Kein `reloadData`**: eine offene Zelle fiele
    /// damit, und eine geaenderte Breite ist kein geaenderter Stand.
    fn umbruchbreite_nachziehen(&self) {
        if !matches!(self.art(), Eintragsart::Notizen | Eintragsart::Termine) {
            return;
        }
        let tabelle = &self.ivars().tabelle;
        let Ok(spalte) = NSInteger::try_from(NOTIZSPALTE) else {
            return;
        };
        let breite = self.umbruchbreite(NOTIZSPALTE);
        for zeile in 0..tabelle.numberOfRows() {
            let feld = tabelle
                .viewAtColumn_row_makeIfNecessary(spalte, zeile, false)
                .and_then(|zelle| zelle.downcast::<NSTableCellView>().ok())
                // SAFETY: Ein Leser ohne Vorbedingung.
                .and_then(|zelle| unsafe { zelle.textField() });
            if let Some(feld) = feld {
                feld.setPreferredMaxLayoutWidth(breite);
            }
        }
    }

    /// Die Ansicht fuer eine Zelle, je nach Art der Zeilen.
    ///
    /// Gerufen mit AppKits Zeile und gelesen an ihr: die abgeleiteten Zeilen
    /// stehen in der Folge des Schirms.
    fn zellenansicht(&self, zeile: NSInteger, spalte: usize) -> Option<Retained<NSView>> {
        let zeile = usize::try_from(zeile).ok()?;
        // Abgeschrieben und die Ausleihe beendet, bevor gebaut wird: der Bau
        // ruft nach AppKit hinaus.
        let zeilen = self.ivars().zeilen.borrow().clone();
        match zeilen {
            Zeilen::Aufgaben(zeilen) => Some(self.aufgabenzelle(zeilen.get(zeile)?)),
            Zeilen::Notizen(zeilen) => {
                let notiz = zeilen.get(zeile)?;
                Some(self.notizzelle(&notiz.thema, &notiz.text, spalte))
            }
            // Datum und Termin in denselben zwei Zellen wie Thema und Notiz.
            Zeilen::Termine(zeilen) => {
                let termin = zeilen.get(zeile)?;
                Some(self.notizzelle(&termin.datum, &termin.text, spalte))
            }
        }
    }

    /// Die Zelle einer Notiz in der genannten Spalte: ein [`Notizfeld`] ueber
    /// Auto Layout an den Kanten einer `NSTableCellView`.
    ///
    /// **Die Notizspalte bricht um und waechst**: mehrzeilig, Umbruch nach
    /// Woertern, Umbruchbreite nach der Spalte, vertikaler Stauchwiderstand
    /// `required` und an allen vier Kanten gebunden, wie gemessen (Modulkopf).
    /// **Die Themenspalte bleibt eine Zeile** und haengt unten nur mit
    /// „hoechstens" an der Zelle, damit eine hohe Notiz daneben die Zeile
    /// strecken darf, ohne dass die zwei Bindungen einander widersprechen.
    ///
    /// Die Termintabelle baut ihre Zellen hier ebenso, mit dem Datum als
    /// Thema und dem Termintext als Notiz.
    fn notizzelle(&self, thema: &str, text: &str, spalte: usize) -> Retained<NSView> {
        let mtm = self.ivars().mtm;
        let ist_notiz = spalte == NOTIZSPALTE;
        let feld = Notizfeld::neu(mtm, if ist_notiz { text } else { thema });
        feld.setFont(Some(&NSFont::systemFontOfSize(NSFont::systemFontSize())));
        feld.setBezeled(false);
        feld.setBordered(false);
        feld.setDrawsBackground(false);
        feld.setEditable(true);
        feld.setSelectable(true);
        if ist_notiz {
            feld.setUsesSingleLineMode(false);
            feld.setMaximumNumberOfLines(0);
            feld.setLineBreakMode(NSLineBreakMode::ByWordWrapping);
            if let Some(zelle) = feld.cell() {
                zelle.setWraps(true);
                zelle.setScrollable(false);
            }
            feld.setPreferredMaxLayoutWidth(self.umbruchbreite(NOTIZSPALTE));
        } else {
            feld.setLineBreakMode(NSLineBreakMode::ByTruncatingTail);
            if let Some(zelle) = feld.cell() {
                zelle.setWraps(false);
                zelle.setScrollable(true);
            }
        }
        feld.setContentCompressionResistancePriority_forOrientation(
            NSLayoutPriorityRequired,
            NSLayoutConstraintOrientation::Vertical,
        );
        feld.setTranslatesAutoresizingMaskIntoConstraints(false);
        // SAFETY: wie in `aufgabenzelle`.
        unsafe { feld.setDelegate(Some(ProtocolObject::from_ref(self))) };

        let zelle = NSTableCellView::initWithFrame(
            NSTableCellView::alloc(mtm),
            NSRect::new(NSPoint::ZERO, NSSize::new(AUFBAUBREITE, ZEILENHOEHE)),
        );
        zelle.addSubview(&feld);
        // SAFETY: Das Feld ist eine Unteransicht der Zelle und lebt so lange
        // wie sie; die Eigenschaft ist `weak` (`NSTableCellView.h`).
        unsafe { zelle.setTextField(Some(&feld)) };
        let unten = if ist_notiz {
            feld.bottomAnchor()
                .constraintEqualToAnchor_constant(&zelle.bottomAnchor(), -RAND_OBEN)
        } else {
            feld.bottomAnchor()
                .constraintLessThanOrEqualToAnchor_constant(&zelle.bottomAnchor(), -RAND_OBEN)
        };
        for bindung in [
            feld.leadingAnchor()
                .constraintEqualToAnchor_constant(&zelle.leadingAnchor(), RAND),
            feld.trailingAnchor()
                .constraintEqualToAnchor_constant(&zelle.trailingAnchor(), -RAND),
            feld.topAnchor()
                .constraintEqualToAnchor_constant(&zelle.topAnchor(), RAND_OBEN),
            unten,
        ] {
            bindung.setActive(true);
        }
        Retained::into_super(zelle)
    }

    /// Die Ansicht fuer eine Aufgabe: das Ankreuzfeld und daneben der Text.
    ///
    /// Eine `NSTableCellView` mit dem Textfeld als `textField`, damit
    /// `editColumn:row:withEvent:select:` weiss, welches Feld es bearbeitet.
    /// Das Ankreuzfeld nimmt den Ersthelferrang nicht an: der gehoert der
    /// Tabelle, und ein Klick darauf soll keine laufende Zelle beenden, ohne
    /// dass `aufgabe_abhaken` sie vorher uebernommen hat. Nach dem Abhaken
    /// holt der Klick den Rang in die Tabelle (`kastenGeklickt:`).
    fn aufgabenzelle(&self, eintrag: &Eintragszeile) -> Retained<NSView> {
        let mtm = self.ivars().mtm;

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
        Retained::into_super(zelle)
    }
}

/// Richtet die Spalten, die Zeilenhoehe und die Kopfzeile fuer eine Art ein.
///
/// **Gerufen beim Bau und bei einem Wechsel der Art in
/// [`Eintragsansicht::zeilen_zeigen`], und sonst nie**: die Spalten sind eine
/// Eigenschaft der Art und nicht des Standes. Die Aufgabentabelle traegt eine
/// Spalte in fester Zeilenhoehe und keine Kopfzeile, die Notiztabelle Thema und
/// Notiz mit Kopfzeile und automatischer Zeilenhoehe, die Termintabelle Datum
/// und Termin ebenso.
fn spalten_einrichten(
    mtm: MainThreadMarker,
    tabelle: &NSTableView,
    rolle: &NSScrollView,
    kopfzeile: &NSTableHeaderView,
    art: Eintragsart,
) {
    let alte: Vec<_> = tabelle.tableColumns().iter().collect();
    for spalte in alte {
        tabelle.removeTableColumn(&spalte);
    }
    let spalte_bauen = |kennung: &NSString, titel: &NSString| {
        let spalte = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), kennung);
        spalte.setTitle(titel);
        spalte
    };
    match art {
        Eintragsart::Aufgaben => {
            let spalte = spalte_bauen(ns_string!("aufgabe"), ns_string!("Aufgabe"));
            spalte.setResizingMask(NSTableColumnResizingOptions::AutoresizingMask);
            tabelle.addTableColumn(&spalte);
            tabelle.setUsesAutomaticRowHeights(false);
            tabelle.setHeaderView(None);
        }
        Eintragsart::Notizen => {
            let thema = spalte_bauen(ns_string!("thema"), ns_string!("Thema"));
            thema.setResizingMask(NSTableColumnResizingOptions::UserResizingMask);
            thema.setWidth(THEMENBREITE);
            thema.setMinWidth(THEMENBREITE / 2.0);
            tabelle.addTableColumn(&thema);
            let notiz = spalte_bauen(ns_string!("notiz"), ns_string!("Notiz"));
            notiz.setResizingMask(
                NSTableColumnResizingOptions::AutoresizingMask
                    | NSTableColumnResizingOptions::UserResizingMask,
            );
            notiz.setMinWidth(THEMENBREITE / 2.0);
            tabelle.addTableColumn(&notiz);
            tabelle.setUsesAutomaticRowHeights(true);
            tabelle.setHeaderView(Some(kopfzeile));
        }
        // Dieselben zwei Spalten wie die Notiztabelle, unter ihren eigenen
        // Namen und mit einer schmaleren ersten.
        Eintragsart::Termine => {
            let datum = spalte_bauen(ns_string!("datum"), ns_string!("Datum"));
            datum.setResizingMask(NSTableColumnResizingOptions::UserResizingMask);
            datum.setWidth(DATUMSBREITE);
            datum.setMinWidth(DATUMSBREITE / 2.0);
            tabelle.addTableColumn(&datum);
            let termin = spalte_bauen(ns_string!("termin"), ns_string!("Termin"));
            termin.setResizingMask(
                NSTableColumnResizingOptions::AutoresizingMask
                    | NSTableColumnResizingOptions::UserResizingMask,
            );
            termin.setMinWidth(THEMENBREITE / 2.0);
            tabelle.addTableColumn(&termin);
            tabelle.setUsesAutomaticRowHeights(true);
            tabelle.setHeaderView(Some(kopfzeile));
        }
    }
    tabelle.sizeLastColumnToFit();
    rolle.tile();
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

    /// Die Regel des Doppelklicks: Zeile und Spalte sind die Zelle, eine Zeile
    /// ohne Spalte ihre erste Zelle, keine Zeile in der Tabelle legt an, keine
    /// Zeile ausserhalb (die Kopfzeile) tut nichts.
    #[test]
    fn der_doppelklick_trifft_eine_zelle_legt_im_leeren_an_und_auf_der_kopfzeile_nichts() {
        assert_eq!(
            doppelklick(Some(2), 1, true),
            Doppelklick::Zelle(Zelle {
                stelle: 2,
                spalte: NOTIZSPALTE
            })
        );
        assert_eq!(
            doppelklick(Some(0), -1, true),
            Doppelklick::Zelle(Zelle {
                stelle: 0,
                spalte: THEMENSPALTE
            })
        );
        assert_eq!(doppelklick(None, -1, true), Doppelklick::Anlegen);
        assert_eq!(doppelklick(None, 0, true), Doppelklick::Anlegen);
        assert_eq!(doppelklick(None, 1, false), Doppelklick::Nichts);
    }

    /// Die zwei Umrechnungen zwischen Stelle und Zeile (Modulkopf, „Stelle
    /// und Zeile"): fuer Aufgaben und Notizen die Gleichheit, jenseits der
    /// Laenge `None` in beiden Richtungen, und in der leeren Tabelle immer
    /// `None`.
    #[test]
    fn stelle_und_zeile_sind_fuer_aufgaben_und_notizen_dieselbe_zahl() {
        let aufgaben = Zeilen::from(aufgabenzeilen("- [ ] a\n- [x] b\n- [ ] c\n"));
        let notizen = Zeilen::from(notizzeilen("Vorspann\n## a\n## b\n"));
        for zeilen in [&aufgaben, &notizen] {
            for zahl in 0..zeilen.len() {
                assert_eq!(zeilen.stelle_der_zeile(zahl), Some(zahl), "{zeilen:?}");
                assert_eq!(zeilen.zeile_der_stelle(zahl), Some(zahl), "{zeilen:?}");
            }
            assert_eq!(zeilen.stelle_der_zeile(zeilen.len()), None);
            assert_eq!(zeilen.zeile_der_stelle(zeilen.len()), None);
        }
        let leer = Zeilen::Aufgaben(Vec::new());
        assert_eq!(leer.stelle_der_zeile(0), None);
        assert_eq!(leer.zeile_der_stelle(0), None);
    }

    /// Die Regel der Auswahl nach dem Neuladen: ohne Auswahl davor die erste
    /// Zeile, eine Auswahl davor bleibt und wird auf die letzte Zeile
    /// beschnitten, und eine leere Tabelle waehlt nichts
    /// (`issues/260926-1737_*_…`).
    #[test]
    fn ohne_auswahl_ist_nach_dem_neuladen_die_erste_zeile_gewaehlt() {
        assert_eq!(auswahl_nach_neuladen(None, 3), Some(0));
        assert_eq!(auswahl_nach_neuladen(None, 1), Some(0));
        assert_eq!(auswahl_nach_neuladen(None, 0), None);
        assert_eq!(
            auswahl_nach_neuladen(Some(2), 3),
            Some(2),
            "eine Auswahl bleibt"
        );
        assert_eq!(auswahl_nach_neuladen(Some(5), 3), Some(2), "beschnitten");
        assert_eq!(auswahl_nach_neuladen(Some(0), 0), None);
    }

    /// Ein Stand ohne Aufgabe gibt keine Zeile, auch der leere.
    #[test]
    fn ohne_aufgabe_gibt_es_keine_zeile() {
        assert!(aufgabenzeilen("").is_empty());
        assert!(aufgabenzeilen("nur Vorspann\n").is_empty());
    }
    /// Termine in umgekehrter Dateireihenfolge: die Zeilen stehen nach dem
    /// Datum, jede traegt ihre Stelle, und die zwei Umrechnungen trennen
    /// Zeile und Stelle; `tab` wechselt in der Folge des Schirms.
    #[test]
    fn die_termintabelle_trennt_zeile_und_stelle() {
        let stand = "Vorspann\n## 261003\ndrei\n## 261002 09:30\nzwei\n## 261001\neins\n";
        let zeilen = Zeilen::from(terminzeilen(stand, Richtung::Aufsteigend, None));
        assert_eq!(zeilen.art(), Eintragsart::Termine);
        let Zeilen::Termine(termine) = &zeilen else {
            unreachable!("die Art ist eben geprueft")
        };
        assert_eq!(
            termine
                .iter()
                .map(|t| (t.stelle, t.datum.as_str(), t.text.as_str()))
                .collect::<Vec<_>>(),
            vec![
                (2, "261001", "eins"),
                (1, "261002 09:30", "zwei"),
                (0, "261003", "drei"),
            ]
        );
        for (zeile, stelle) in [(0, 2), (1, 1), (2, 0)] {
            assert_eq!(zeilen.stelle_der_zeile(zeile), Some(stelle));
            assert_eq!(zeilen.zeile_der_stelle(stelle), Some(zeile));
        }
        assert_eq!(zeilen.stelle_der_zeile(3), None);
        assert_eq!(zeilen.zeile_der_stelle(3), None);

        let absteigend = Zeilen::from(terminzeilen(stand, Richtung::Absteigend, None));
        assert_eq!(
            absteigend.stelle_der_zeile(0),
            Some(0),
            "der spaeteste oben"
        );

        let zelle = |stelle, spalte| Zelle { stelle, spalte };
        assert_eq!(
            naechste_zelle(zelle(2, TERMINSPALTE), true, &zeilen),
            Some(zelle(1, DATUMSSPALTE)),
            "tab fuehrt in die naechste Zeile des Schirms und nicht an die naechste Stelle"
        );
        assert_eq!(naechste_zelle(zelle(0, TERMINSPALTE), true, &zeilen), None);
        assert_eq!(
            naechste_zelle(zelle(1, DATUMSSPALTE), false, &zeilen),
            Some(zelle(2, TERMINSPALTE))
        );
        assert_eq!(naechste_zelle(zelle(2, DATUMSSPALTE), false, &zeilen), None);
        assert_eq!(
            zellenbefehl(Eintragsart::Termine, TERMINSPALTE, sel!(insertNewline:)),
            Zellenbefehl::Umbruch,
            "return schreibt im Termin einen Umbruch"
        );
        assert_eq!(
            zellenbefehl(Eintragsart::Termine, DATUMSSPALTE, sel!(insertNewline:)),
            Zellenbefehl::Appkit,
            "return beendet die Datumszelle"
        );
        assert_eq!(
            zellenbefehl(Eintragsart::Termine, DATUMSSPALTE, sel!(insertTab:)),
            Zellenbefehl::Weiter(true)
        );
    }

    /// Jede Zeilennummer, die AppKit meldet — `rowForView:`, `selectedRow`,
    /// `clickedRow` —, geht im Code dieser Datei ueber `stelle_an`, und jede,
    /// die zu AppKit hinausgeht — `selectRowIndexes:`, `editColumn:row:` —,
    /// steht allein in einem Rumpf, der sie ueber `zeile_von` gewinnt
    /// (Modulkopf, „Stelle und Zeile"). **Was sie nicht sieht:** eine
    /// Zeilennummer, die ueber einen anderen Weg als diese fuenf Namen
    /// hereinkommt; wer einen solchen baut, traegt ihn hier ein.
    #[test]
    fn jede_zeilennummer_von_appkit_geht_ueber_die_umrechnung() {
        use super::super::anwendung::quelltextproben::{datei, rumpf};
        let quelle = datei("krk-ui/src/appkit/eintragsansicht.rs");
        let (code, _) = quelle
            .split_once(concat!("#[cfg(test)]\nmod ", "tests {"))
            .expect("das Pruefmodul steht am Fuss der Datei");
        let codezeilen = || {
            code.lines()
                .filter(|zeile| !zeile.trim_start().starts_with("//"))
        };
        for nadel in [
            concat!("rowFor", "View("),
            concat!("selected", "Row()"),
            concat!("clicked", "Row()"),
        ] {
            let zeilen: Vec<_> = codezeilen().filter(|zeile| zeile.contains(nadel)).collect();
            assert!(!zeilen.is_empty(), "{nadel} steht nicht mehr im Code");
            for zeile in zeilen {
                assert!(
                    zeile.contains(concat!("stelle_", "an(")),
                    "{nadel} ohne Umrechnung: {zeile}"
                );
            }
        }
        for (nadel, name) in [
            (
                concat!("selectRowIndexes_", "byExtendingSelection("),
                "auswahl_setzen",
            ),
            (
                concat!("editColumn_row_", "withEvent_select("),
                "zelle_beginnen",
            ),
        ] {
            assert_eq!(
                codezeilen().filter(|zeile| zeile.contains(nadel)).count(),
                1,
                "{nadel} steht an mehr als einer Stelle"
            );
            assert!(rumpf(&quelle, name).contains(nadel));
        }
        assert!(rumpf(&quelle, "auswahl_setzen").contains(concat!("self.zeile_", "von(")));
        assert!(rumpf(&quelle, "zelle_beginnen").contains(concat!("self.ziel_der_", "zelle(")));
        assert!(rumpf(&quelle, "ziel_der_zelle").contains(concat!("self.zeile_", "von(")));
    }
    /// T6.1 in der Oberflaechenhaelfte: mit festem Tag tragen genau die zwei
    /// Termine dieses Tages, mit und ohne Uhrzeit, die Hervorhebung; der
    /// Vortag, der Folgetag und ein ungueltiges Datum nicht, und ohne Tag
    /// keiner.
    #[test]
    fn genau_die_termine_des_tages_sind_hervorgehoben() {
        let stand = concat!(
            "## 261003\nFolgetag\n",
            "## 261002 09:30\nmit Uhrzeit\n",
            "## 261001\nVortag\n",
            "## xyz\nungueltig\n",
            "## 261002\nohne Uhrzeit\n",
        );
        let heute = Tag::neu(2026, 10, 2).expect("ein Tag im Kalender");
        let hervorgehoben: Vec<(usize, bool)> =
            terminzeilen(stand, Richtung::Aufsteigend, Some(heute))
                .into_iter()
                .map(|termin| (termin.stelle, termin.heute))
                .collect();
        assert_eq!(
            hervorgehoben,
            vec![(2, false), (4, true), (1, true), (0, false), (3, false)]
        );
        assert!(
            terminzeilen(stand, Richtung::Aufsteigend, None)
                .iter()
                .all(|termin| !termin.heute),
            "ohne Tag ist keine Zeile hervorgehoben"
        );
    }

    /// T6.3: die Farbe des heutigen Tages steht an einer Stelle, ist die
    /// Systemfarbe Gelb und kein fest gebauter Farbwert, und sie ist weder die
    /// Farbe der Auswahl noch die der Auswahl ohne Fokus. Dazu setzt der
    /// Delegierte sie je Zeile und setzt sonst zurueck.
    #[test]
    fn die_farbe_des_heutigen_tages_ist_die_systemfarbe_gelb_und_keine_auswahlfarbe() {
        use super::super::anwendung::quelltextproben::{datei, rumpf};
        let quelle = datei("krk-ui/src/appkit/eintragsansicht.rs");
        let farbe = rumpf(&quelle, "heute_farbe");
        assert!(farbe.contains(concat!("systemYellow", "Color()")));
        for nadel in [
            concat!("colorWith", "Red"),
            concat!("colorWith", "SRGB"),
            concat!("colorWith", "Calibrated"),
            concat!("colorWith", "DeviceRed"),
        ] {
            assert!(
                !farbe.contains(nadel),
                "heute_farbe baut eine feste Farbe: {nadel}"
            );
        }
        let (code, _) = quelle
            .split_once(concat!("#[cfg(test)]\nmod ", "tests {"))
            .expect("das Pruefmodul steht am Fuss der Datei");
        assert_eq!(
            code.lines()
                .filter(|zeile| !zeile.trim_start().starts_with("//"))
                .filter(|zeile| zeile.contains(concat!("systemYellow", "Color(")))
                .count(),
            1,
            "die Farbe steht an einer Stelle"
        );
        let hinzugefuegt = rumpf(&quelle, "zeilenansicht_hinzugefuegt");
        for nadel in [
            concat!("heute_", "farbe()"),
            concat!("tabelle.background", "Color()"),
            concat!("setBackground", "Color("),
        ] {
            assert!(
                hinzugefuegt.contains(nadel),
                "die Zeilenansicht setzt {nadel} nicht"
            );
        }

        let heute = heute_farbe();
        let heute: &AnyObject = &heute;
        assert!(!NSColor::selectedContentBackgroundColor().isEqual(Some(heute)));
        assert!(!NSColor::unemphasizedSelectedContentBackgroundColor().isEqual(Some(heute)));
    }
}
