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
//! falsche. [`Zeilen::Geheimnisse`] traegt seit dem 261016 sechs Spalten,
//! Name, Protokoll, Adresse, Konto, Passwort und Notizen, in der Folge der
//! Datei ([`geheimniszeilen`]); fuenf sind einzeilig wie das Thema, die
//! Notizen mehrzeilig wie die Notiz ([`mehrzeilige_spalte`]). Bis dahin
//! zeigten sich die Geheimnisse als Notizen.
//!
//! # Die Tabelle einer CSV- oder TSV-Datei
//!
//! Seit Schritt 3 des Plans
//! `261018-0822_*_plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md`
//! zeigt dieselbe Ansicht eine fuenfte Art, [`Zeilen::Trenntext`], und sie
//! unterscheidet sich von den vier anderen in drei Stuecken:
//!
//! - **Die Spalten kommen aus der Datei** und nicht aus der Art: so viele, wie
//!   der Satzindex des Kerns meldet, davor eine schmale Nummernspalte
//!   ([`NUMMERNSPALTE`]), die mitrollt und nicht bearbeitbar ist. Die
//!   Tabellenspalte ist damit nicht das Feld der Datei, und die Umrechnung
//!   steht an einer Stelle neben der von Stelle und Zeile:
//!   [`Zeilen::feld_der_tabellenspalte`] und
//!   [`Zeilen::tabellenspalte_des_feldes`]. **Nach aussen spricht die Ansicht
//!   auch hier allein in Feldern**: [`Zelle::spalte`] ist in dieser Art das
//!   Feld der Datei, und jede Spaltennummer von AppKit (`columnForView:`,
//!   `clickedColumn`, `editColumn:row:`) geht durch die zwei Umrechnungen.
//!   Fuer die vier anderen Arten sind beide die Gleichheit.
//! - **Die Zeilen sind keine abgeleiteten Werte, sondern eine Sicht auf den
//!   Index** ([`Trenntextsicht`]): Text und Satzindex hinter einem `Rc`, dazu
//!   eine Generation, an der [`Eintragsansicht::zeilen_zeigen`] einen neuen
//!   Stand erkennt, statt Zeilen zu vergleichen. Die Zellen kommen ueber
//!   `makeViewWithIdentifier:owner:` wieder, und ein zerlegter Satz bleibt fuer
//!   die Zellen derselben Zeile zwischengespeichert, damit eine Zeile mit
//!   dreissig sichtbaren Zellen einmal zerlegt wird (E9).
//! - **Die Tabelle rollt waagerecht** und richtet ihre Spaltenbreiten nicht
//!   nach der Rolle, sondern nach dem Inhalt der ersten Saetze (E8); die
//!   vier anderen Arten behalten `LastColumnOnly` ohne waagerechten
//!   Rollbalken. Beides stellt [`spalten_einrichten`] je Art ein, an der einen
//!   Stelle, an der es die Spalten einrichtet.
//!
//! Eine Zelle zeigt einen Zeilenumbruch als `⏎` und einen Tabulator als `⇥`
//! ([`zellanzeige`]); bearbeitet wird der wirkliche Wert. **Darum ist ihr Feld
//! nur waehrend einer Bearbeitung bearbeitbar**: [`Eintragsansicht::zelle_beginnen`]
//! setzt den wirklichen Wert ein und schaltet das Feld bearbeitbar, das Ende
//! der Zelle schaltet es zurueck. Ein einfacher Klick waehlt die Zeile und
//! beginnt keine Bearbeitung, die sonst den gezeigten Ersatz als Text naehme.
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
//! - fuer die Tabelle einer CSV `makeViewWithIdentifier:owner:` seit 10.7
//!   (`NSTableView.h:488`) und das Protokoll
//!   `NSUserInterfaceItemIdentification` samt `setIdentifier:` seit 10.7
//!   (`NSUserInterfaceItemIdentification.h:14`); ohne Angabe und damit seit
//!   10.0 `setHasHorizontalScroller:`, `hasHorizontalScroller`,
//!   `setAllowsColumnReordering:`, `columnAutoresizingStyle`,
//!   `scrollColumnToVisible:`, `numberOfColumns`, `title` und `pointSize`
//! - fuer die Rechteckauswahl der Tabelle einer CSV (Schritt 4 des Plans der
//!   CSV-Tabelle) `NSTableViewSelectionHighlightStyle` samt
//!   `setSelectionHighlightStyle:` und `selectionHighlightStyle` seit 10.5
//!   (`NSTableView.h`), ohne Angabe und damit seit 10.0 `NSEvent` und die
//!   Aufzaehlung `NSEventModifierFlags` samt `modifierFlags` und
//!   `clickCount`, `rowAtPoint:`, `columnAtPoint:`, `autoscroll:`,
//!   `deselectAll:`, `nextResponder`, `tryToPerform:with:`,
//!   `setBackgroundColor:`, `drawsBackground` und
//!   `selectedTextBackgroundColor`, dazu die **gebauten** Methoden
//!   `mouseDown:` und `mouseDragged:` (`NSResponder.h`) und die Aktionen
//!   `paste:` und `selectAll:`; seit 10.7 `enumerateAvailableRowViewsUsingBlock:`
//!   (`NSTableView.h`), dessen Block `StackBlock` aus `block2` ist, ein
//!   Rust-Typ der Kiste ohne macOS-Alter
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

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::ptr::NonNull;
use std::rc::Rc;

use block2::StackBlock;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, Bool, ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSApplication, NSAutoresizingMaskOptions, NSButton, NSColor, NSControl, NSControlStateValueOff,
    NSControlStateValueOn, NSControlTextEditingDelegate, NSEvent, NSEventModifierFlags, NSFont,
    NSImage, NSLayoutConstraintOrientation, NSLayoutPriorityRequired, NSLineBreakMode, NSResponder,
    NSScrollView, NSTableCellView, NSTableColumn, NSTableColumnResizingOptions, NSTableHeaderView,
    NSTableRowView, NSTableView, NSTableViewColumnAutoresizingStyle, NSTableViewDataSource,
    NSTableViewDelegate, NSTableViewSelectionHighlightStyle, NSTableViewStyle, NSText, NSTextField,
    NSTextFieldDelegate, NSTextView, NSUserInterfaceItemIdentification, NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSIndexSet, NSInteger, NSNotification, NSObject, NSObjectProtocol, NSPoint,
    NSRect, NSSize, NSString, NSUndoManager, ns_string,
};

use krk_core::heimordner::eintraege::geheimnisse::{Feld, Geheimnisse};
use krk_core::heimordner::eintraege::{Aufgaben, Notizen, Tag, aufgabenzeile, termine};
use krk_core::sprache::{Text, text};
use krk_core::tasten::{ModMaske, Tastendruck};
use krk_core::trenntext::{self, Satzindex, Zellbereich};
use krk_core::verzeichnis::Richtung;

use crate::trenntextvorschau::spaltenbuchstabe;

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

/// Die Spalte der Geheimnistabelle, die ein Feld zeigt: seine Stelle in
/// `Feld::ALLE`, denn die Spalten stehen in der Folge der Felder
/// (Entscheidung 18 des Plans
/// `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`).
#[must_use]
pub fn spalte_des_feldes(feld: Feld) -> usize {
    Feld::ALLE
        .iter()
        .position(|bekannt| *bekannt == feld)
        .unwrap_or(THEMENSPALTE)
}

/// Das Feld, das die Spalte `spalte` der Geheimnistabelle zeigt; `None`
/// jenseits der sechsten.
#[must_use]
pub fn feld_der_spalte(spalte: usize) -> Option<Feld> {
    Feld::ALLE.get(spalte).copied()
}

/// Die eine mehrzeilige Spalte einer Art, in der `return` einen Umbruch
/// schreibt und das Feld mit seinem Inhalt waechst; `None` fuer die
/// Aufgabentabelle, die keine hat.
///
/// **Die eine Stelle dieser Frage**: [`zellenbefehl`], die Zellen und die
/// Umbruchbreite fragen hier und nicht nach [`NOTIZSPALTE`], denn in der
/// Geheimnistabelle sind die Notizen die sechste Spalte.
#[must_use]
pub fn mehrzeilige_spalte(art: Eintragsart) -> Option<usize> {
    match art {
        // Eine Zelle einer CSV zeigt eine Zeile; ihr Umbruch erscheint als
        // `⏎` ([`zellanzeige`]), und `opt+return` schreibt ihn beim Bearbeiten.
        Eintragsart::Aufgaben | Eintragsart::Trenntext => None,
        Eintragsart::Notizen | Eintragsart::Termine => Some(NOTIZSPALTE),
        Eintragsart::Geheimnisse => Some(spalte_des_feldes(Feld::Notizen)),
    }
}

/// Wie viele Spalten die Tabelle dieser Art traegt, ohne eine Nummernspalte.
///
/// Die Tabelle einer CSV hat keine feste Zahl; hier steht ihre Mindestzahl
/// (E7), und die wirkliche nennt [`Zeilen::spaltenzahl`].
#[must_use]
pub fn spaltenzahl(art: Eintragsart) -> usize {
    match art {
        Eintragsart::Aufgaben | Eintragsart::Trenntext => 1,
        Eintragsart::Notizen | Eintragsart::Termine => 2,
        Eintragsart::Geheimnisse => Feld::ALLE.len(),
    }
}

/// Die Tabellenspalte, in der die Tabelle einer CSV die Nummer jeder Zeile
/// zeigt (E8); das erste Feld der Datei steht in der Tabellenspalte danach.
pub const NUMMERNSPALTE: usize = 0;

/// Wie viele Zeichen des Textes der ersten Zeile ein Spaltenkopf der Tabelle
/// einer CSV zeigt, bevor er in „…“ endet (E6).
const KOPFZEICHEN: usize = 24;

/// Aus wie vielen Saetzen die Anfangsbreite einer Spalte der Tabelle einer
/// CSV entsteht (E8).
const BREITENSAETZE: usize = 1000;

/// Die schmalste und die breiteste Anfangsbreite einer Spalte der Tabelle
/// einer CSV in Punkten (E8); ziehen laesst sich jede ueber beide hinaus.
const BREITENGRENZEN: (f64, f64) = (48.0, 320.0);

/// Der Anteil der Punktgroesse, den ein Zeichen der Tabellenschrift im Mittel
/// breit ist.
///
/// **Eine Naeherung und keine Messung**, anders als E8 es vorschlug: die
/// mittlere Breite der Systemschrift liefert `NSFont` nicht, und ein Messen
/// jeder Zelle mit `sizeWithAttributes:` kostete bei tausend Saetzen und
/// tausend Spalten mehr als das Oeffnen. Die Breite ist der Anfang; der Nutzer
/// zieht sie am Kopf, wo sie nicht passt.
const ZEICHENANTEIL: f64 = 0.55;

/// Der Text, den eine Zelle der Tabelle einer CSV fuer diesen Wert zeigt: ein
/// Zeilenumbruch (`\r\n`, `\n` oder ein einzelnes `\r`) als `⏎`, ein
/// Tabulator als `⇥`, damit jede Zelle eine Zeile bleibt (E8). Zu langen Text
/// kuerzt das Feld selbst mit „…“.
#[must_use]
pub fn zellanzeige(wert: &str) -> Cow<'_, str> {
    if !wert.contains(['\n', '\r', '\t']) {
        return Cow::Borrowed(wert);
    }
    Cow::Owned(
        wert.replace("\r\n", "⏎")
            .replace(['\n', '\r'], "⏎")
            .replace('\t', "⇥"),
    )
}

/// Der Titel des Kopfes ueber dem Feld `feld` der Tabelle einer CSV: sein
/// Buchstabe und daneben der Text, den die erste Zeile in ihm traegt, nach
/// [`KOPFZEICHEN`] Zeichen gekuerzt (E6). **Die erste Zeile bleibt dabei eine
/// Zeile der Tabelle**; der Kopf zeigt ihren Text nur an.
#[must_use]
pub fn kopftitel(feld: usize, erste_zeile: Option<&str>) -> String {
    let buchstabe = spaltenbuchstabe(feld);
    let anzeige = zellanzeige(erste_zeile.unwrap_or_default());
    let anzeige = anzeige.trim();
    if anzeige.is_empty() {
        return buchstabe;
    }
    let mut kurz: String = anzeige.chars().take(KOPFZEICHEN).collect();
    if anzeige.chars().count() > KOPFZEICHEN {
        kurz.push('…');
    }
    format!("{buchstabe}  {kurz}")
}

/// Die Anfangsbreite einer Spalte, deren laengster Text `zeichen` Zeichen
/// traegt, bei der Punktgroesse `punkte` der Tabellenschrift (E8): die
/// Zeichen mal ihre mittlere Breite samt Rand, begrenzt auf
/// [`BREITENGRENZEN`].
#[must_use]
pub fn spaltenbreite(zeichen: usize, punkte: f64) -> f64 {
    let (schmal, breit) = BREITENGRENZEN;
    (zeichen as f64 * punkte * ZEICHENANTEIL + 2.0 * RAND).clamp(schmal, breit)
}

/// Die Sicht der Tabelle auf eine CSV- oder TSV-Datei (E9): der Text, sein
/// Satzindex und die Generation, mit der beide entstanden sind.
///
/// **Text und Index gehoeren zusammen und entstehen nur zusammen**, in
/// [`Self::neu`]: jeder Leser am Index fragt den Text, aus dem er entstand,
/// und der Kern haelt an, wenn es ein anderer ist. Der Text ist eine
/// Abschrift des Standes, den der Editor haelt, weil die Ansicht das Modell
/// nicht kennt; sie entsteht je neuem Stand einmal.
///
/// **Zwei Sichten sind gleich, wenn ihre Generation gleich ist**, und nicht,
/// wenn ihre Zeilen gleich sind: ein Vergleich von sechzehn Mebibyte je
/// Nachzug waere der Preis, den der Index sparen soll. Die Generation zaehlt
/// der Editorbereich, je neuem Index eins weiter.
#[derive(Clone)]
pub struct Trenntextsicht {
    /// Der Text, aus dem der Index entstand.
    text: Rc<str>,
    /// Der Index dieses Textes.
    index: Rc<Satzindex>,
    /// Die Generation, mit der beide entstanden sind.
    generation: u64,
}

impl Trenntextsicht {
    /// Die Sicht auf `text` mit seinem Index `index`.
    ///
    /// # Panics
    ///
    /// Wenn `index` nicht aus `text` entstand (an der Laenge geprueft, wie im
    /// Kern).
    #[must_use]
    pub fn neu(text: Rc<str>, index: Rc<Satzindex>, generation: u64) -> Self {
        // Die Pruefung des Kerns, einmal hier statt bei jedem Leser: ein
        // Feld, das an einem fremden Text gelesen wird, hielte erst beim
        // Zeichnen an.
        let _ = index.felder(&text, 0);
        Self {
            text,
            index,
            generation,
        }
    }

    /// Der Text, aus dem der Index entstand.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Der Index des Textes.
    #[must_use]
    pub fn index(&self) -> &Satzindex {
        &self.index
    }

    /// Die Generation, mit der Text und Index entstanden sind.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Die Werte der Felder des Satzes `satz`, leer jenseits des letzten.
    #[must_use]
    pub fn werte(&self, satz: usize) -> Vec<String> {
        self.index
            .felder(&self.text, satz)
            .map(|felder| {
                felder
                    .into_iter()
                    .map(|feld| feld.wert.into_owned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Der Wert der Zelle in Zeile `satz` und Feld `feld`; leer hinter dem
    /// letzten Feld des Satzes (E7) und jenseits des letzten Satzes.
    #[must_use]
    pub fn wert(&self, satz: usize, feld: usize) -> String {
        self.werte(satz).into_iter().nth(feld).unwrap_or_default()
    }
}

impl PartialEq for Trenntextsicht {
    fn eq(&self, andere: &Self) -> bool {
        self.generation == andere.generation
    }
}

impl Eq for Trenntextsicht {}

impl std::fmt::Debug for Trenntextsicht {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trenntextsicht")
            .field("generation", &self.generation)
            .field("zeilen", &self.index.zeilenzahl())
            .field("spalten", &self.index.spaltenzahl())
            .finish()
    }
}

impl From<Trenntextsicht> for Zeilen {
    fn from(sicht: Trenntextsicht) -> Self {
        Self::Trenntext(sicht)
    }
}

/// Ein zerlegter Satz der Tabelle einer CSV, fuer die Zellen derselben Zeile
/// (E9): AppKit fragt eine Zeile Zelle fuer Zelle ab, und ohne ihn zerlegte
/// jede Zelle ihren Satz ein weiteres Mal.
struct Satzspeicher {
    /// Die Generation der Sicht, aus der er stammt.
    generation: u64,
    /// Die Zeile.
    zeile: usize,
    /// Die Werte ihrer Felder.
    werte: Vec<String>,
}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eintragsart {
    /// Eine Spalte mit Kaestchen und Text, `tasks.txt`.
    Aufgaben,
    /// Zwei Spalten, Thema und Notiz, `notes.txt`.
    Notizen,
    /// Zwei Spalten, Datum und Termin, `appointments.md`, nach dem Datum
    /// geordnet.
    Termine,
    /// Sechs Spalten, Name, Protokoll, Adresse, Konto, Passwort und Notizen,
    /// `secrets.txt` (Entscheidungen 17 und 18 des Plans
    /// `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`).
    /// Bis dahin zeigten sich die Geheimnisse als Notizen.
    Geheimnisse,
    /// Eine CSV- oder TSV-Datei: so viele Spalten, wie die Datei traegt, davor
    /// die Nummer jeder Zeile (Schritt 3 des Plans der CSV-Tabelle); der
    /// Modulkopf sagt, was daran anders ist.
    Trenntext,
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
    /// Die Zeilen der Geheimnistabelle, in der Folge der Datei.
    Geheimnisse(Vec<Geheimniszeile>),
    /// Die Sicht auf eine CSV- oder TSV-Datei, in der Folge der Datei; ihre
    /// Zeilen entstehen beim Zeichnen aus dem Index (E9).
    Trenntext(Trenntextsicht),
}

impl Zeilen {
    /// Die Art dieser Zeilen.
    #[must_use]
    pub fn art(&self) -> Eintragsart {
        match self {
            Self::Aufgaben(_) => Eintragsart::Aufgaben,
            Self::Notizen(_) => Eintragsart::Notizen,
            Self::Termine(_) => Eintragsart::Termine,
            Self::Geheimnisse(_) => Eintragsart::Geheimnisse,
            Self::Trenntext(_) => Eintragsart::Trenntext,
        }
    }

    /// Wie viele Zeilen es sind.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Aufgaben(zeilen) => zeilen.len(),
            Self::Notizen(zeilen) => zeilen.len(),
            Self::Termine(zeilen) => zeilen.len(),
            Self::Geheimnisse(zeilen) => zeilen.len(),
            Self::Trenntext(sicht) => sicht.index().zeilenzahl(),
        }
    }

    /// Wie viele Spalten der Datei diese Zeilen tragen, ohne eine
    /// Nummernspalte: fuer die Tabelle einer CSV die groesste Feldzahl ihrer
    /// Saetze (E7), sonst die feste Zahl der Art.
    #[must_use]
    pub fn spaltenzahl(&self) -> usize {
        match self {
            Self::Trenntext(sicht) => sicht.index().spaltenzahl(),
            Self::Aufgaben(_) | Self::Notizen(_) | Self::Termine(_) | Self::Geheimnisse(_) => {
                spaltenzahl(self.art())
            }
        }
    }

    /// Das Feld, das AppKits Tabellenspalte `spalte` zeigt; `None` fuer die
    /// Nummernspalte und jenseits der letzten Spalte.
    ///
    /// **Eine der zwei Stellen dieser Umrechnung** (Modulkopf, „Die Tabelle
    /// einer CSV- oder TSV-Datei"): in der Tabelle einer CSV steht davor die
    /// Nummernspalte, in jeder anderen Art ist es die Gleichheit.
    #[must_use]
    pub fn feld_der_tabellenspalte(&self, spalte: usize) -> Option<usize> {
        let feld = match self {
            Self::Trenntext(_) => spalte.checked_sub(NUMMERNSPALTE + 1)?,
            Self::Aufgaben(_) | Self::Notizen(_) | Self::Termine(_) | Self::Geheimnisse(_) => {
                spalte
            }
        };
        (feld < self.spaltenzahl()).then_some(feld)
    }

    /// Die Tabellenspalte, in der AppKit das Feld `feld` zeigt; die Umkehrung
    /// von [`Self::feld_der_tabellenspalte`].
    #[must_use]
    pub fn tabellenspalte_des_feldes(&self, feld: usize) -> usize {
        match self {
            Self::Trenntext(_) => feld + NUMMERNSPALTE + 1,
            Self::Aufgaben(_) | Self::Notizen(_) | Self::Termine(_) | Self::Geheimnisse(_) => feld,
        }
    }

    /// Die Stelle im Stand, die AppKits Zeile `zeile` zeigt; `None` jenseits
    /// der letzten Zeile.
    ///
    /// **Eine der zwei Stellen der Umrechnung** (Modulkopf, „Stelle und
    /// Zeile"). Aufgaben, Notizen und Geheimnisse stehen in der Reihenfolge
    /// der Datei, und dort ist es die Gleichheit; eine Terminzeile nennt ihre
    /// Stelle selbst. Die Geheimnisse gehen trotzdem durch diese Naht und
    /// nicht an ihr vorbei, damit eine spaetere Ordnung der Tabelle allein
    /// diese zwei Funktionen beruehrt.
    #[must_use]
    pub fn stelle_der_zeile(&self, zeile: usize) -> Option<usize> {
        match self {
            Self::Aufgaben(_) | Self::Notizen(_) | Self::Geheimnisse(_) | Self::Trenntext(_) => {
                (zeile < self.len()).then_some(zeile)
            }
            Self::Termine(zeilen) => zeilen.get(zeile).map(|termin| termin.stelle),
        }
    }

    /// Die Zeile, in der AppKit die Stelle `stelle` zeigt; `None`, wenn es sie
    /// nicht gibt.
    ///
    /// Die Umkehrung von [`Self::stelle_der_zeile`], und fuer Aufgaben,
    /// Notizen und Geheimnisse ebenso die Gleichheit; fuer Termine die Zeile,
    /// deren Stelle es ist.
    #[must_use]
    pub fn zeile_der_stelle(&self, stelle: usize) -> Option<usize> {
        match self {
            Self::Aufgaben(_) | Self::Notizen(_) | Self::Geheimnisse(_) | Self::Trenntext(_) => {
                (stelle < self.len()).then_some(stelle)
            }
            Self::Termine(zeilen) => zeilen.iter().position(|termin| termin.stelle == stelle),
        }
    }
}

impl From<Vec<Geheimniszeile>> for Zeilen {
    fn from(zeilen: Vec<Geheimniszeile>) -> Self {
        Self::Geheimnisse(zeilen)
    }
}

/// Eine Zeile der Geheimnistabelle, abgeleitet aus einem Eintrag des
/// Klartexts von `secrets.txt`.
///
/// **Die Passwortspalte zeigt den Klartext** (Entscheidung 17 des Plans):
/// die Rohansicht derselben Datei zeigt ihn ohnehin, und eine Verdeckung
/// allein hier verspraeche einen Schutz, den die zweite Ansicht nicht haelt.
/// `Debug` zeigt das Passwort trotzdem nicht, damit kein Protokoll einer
/// Probe es traegt.
#[derive(Clone, PartialEq, Eq)]
pub struct Geheimniszeile {
    /// Die sechs Werte in der Folge der Spalten, wie `Feld::ALLE` sie nennt.
    werte: [String; 6],
}

impl Geheimniszeile {
    /// Der Wert der Spalte `spalte`; leer jenseits der sechsten.
    #[must_use]
    pub fn wert(&self, spalte: usize) -> &str {
        self.werte.get(spalte).map_or("", String::as_str)
    }

    /// Der Wert des Feldes `feld`.
    #[must_use]
    pub fn feld(&self, feld: Feld) -> &str {
        self.wert(spalte_des_feldes(feld))
    }
}

impl std::fmt::Debug for Geheimniszeile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Geheimniszeile")
            .field("name", &self.feld(Feld::Name))
            .field("protokoll", &self.feld(Feld::Protokoll))
            .field("adresse", &self.feld(Feld::Adresse))
            .field("konto", &self.feld(Feld::Konto))
            .field("passwort", &"****")
            .field("notizen", &self.feld(Feld::Notizen))
            .finish()
    }
}

/// Die Zeilen der Geheimnistabelle zu einem Klartext von `secrets.txt`.
///
/// **Die eine Stelle der Ableitung fuer Geheimnisse**, ueber
/// `geheimnisse::Geheimnisse` und damit ueber dieselbe Leseregel, gegen die
/// `geheimnisse::aendern` eine unveraenderte Zelle erkennt. Ein Eintrag des
/// frueheren Formats erscheint mit leeren Feldern und vollstaendigen Notizen
/// (Entscheidung 2 des Plans). Eine Zeile je Eintrag, in der Folge der Datei.
#[must_use = "die Zeilen sind die ganze Auskunft"]
pub fn geheimniszeilen(stand: &str) -> Vec<Geheimniszeile> {
    let geheimnisse = Geheimnisse::lesen(stand);
    (0..geheimnisse.anzahl())
        .filter_map(|stelle| geheimnisse.geheimnis(stelle))
        .map(|geheimnis| Geheimniszeile {
            werte: Feld::ALLE.map(|feld| geheimnis.wert(feld).to_owned()),
        })
        .collect()
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
/// Themenspalte, die Terminspalte die Notizspalte. Die Geheimnistabelle ebenso,
/// mit fuenf einzeiligen Spalten und den Notizen als der einen mehrzeiligen
/// ([`mehrzeilige_spalte`]). Die Tabelle einer CSV hat keine mehrzeilige
/// Spalte: `return` uebernimmt, `opt+return` schreibt den Umbruch ueber die
/// Belegung des Feldeditors von AppKit, und `tab` geht durch die Felder (E16).
#[must_use]
pub fn zellenbefehl(art: Eintragsart, spalte: usize, befehl: Sel) -> Zellenbefehl {
    match art {
        Eintragsart::Aufgaben => Zellenbefehl::Appkit,
        Eintragsart::Notizen
        | Eintragsart::Termine
        | Eintragsart::Geheimnisse
        | Eintragsart::Trenntext => {
            if befehl == sel!(insertNewline:) && mehrzeilige_spalte(art) == Some(spalte) {
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
/// **naechsten Zeile auf dem Schirm**; in der Geheimnistabelle ebenso durch
/// alle sechs Spalten und in der Tabelle einer CSV durch jedes Feld
/// ([`Zeilen::spaltenzahl`]). Die Zelle kommt und geht als Stelle;
/// gezaehlt wird dazwischen in Zeilen, ueber die zwei Umrechnungen an
/// [`Zeilen`] (Modulkopf, „Stelle und Zeile").
#[must_use]
pub fn naechste_zelle(zelle: Zelle, vorwaerts: bool, zeilen: &Zeilen) -> Option<Zelle> {
    let Zelle { stelle, spalte } = zelle;
    let zeile = zeilen.zeile_der_stelle(stelle)?;
    let letzte = zeilen.spaltenzahl().saturating_sub(1);
    let (ziel, spalte) = if vorwaerts {
        if spalte < letzte {
            (zeile, spalte + 1)
        } else {
            (zeile + 1, THEMENSPALTE)
        }
    } else if spalte > THEMENSPALTE {
        (zeile, spalte - 1)
    } else {
        (zeile.checked_sub(1)?, letzte)
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
    /// hat allein die erste, die Geheimnistabelle sechs
    /// ([`spalte_des_feldes`]). In der Tabelle einer CSV das Feld der Datei
    /// und **nie** AppKits Tabellenspalte, die die Nummernspalte mitzaehlt
    /// ([`Zeilen::feld_der_tabellenspalte`]).
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

/// Wie gross die Tabelle einer CSV ist: Zeilen und Felder, ohne die
/// Nummernspalte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tabellenmass {
    /// Die Zahl der Zeilen.
    pub zeilen: usize,
    /// Die Zahl der Felder, mindestens eins, sobald es Zeilen gibt.
    pub spalten: usize,
}

impl Tabellenmass {
    /// Die letzte Zelle, `None` fuer eine Tabelle ohne Zeilen oder Spalten.
    fn letzte(self) -> Option<(usize, usize)> {
        Some((self.zeilen.checked_sub(1)?, self.spalten.checked_sub(1)?))
    }
}

/// Eine Pfeiltaste, wie sie die Tabelle einer CSV beantwortet (E10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pfeil {
    /// Eine Zeile hoeher.
    Hoch,
    /// Eine Zeile tiefer.
    Runter,
    /// Ein Feld nach links.
    Links,
    /// Ein Feld nach rechts.
    Rechts,
}

/// Wohin ein Klick in der Tabelle einer CSV faellt (E10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Klickort {
    /// Eine Zelle, in Zeile und Feld der Datei.
    Zelle {
        /// Die Zeile.
        zeile: usize,
        /// Das Feld, nie die Tabellenspalte.
        feld: usize,
    },
    /// Die Nummer einer Zeile in der Nummernspalte: die ganze Zeile.
    Nummer {
        /// Die Zeile.
        zeile: usize,
    },
    /// Der Kopf einer Spalte: die ganze Spalte.
    Kopf {
        /// Das Feld der Spalte.
        feld: usize,
    },
}

/// Die Auswahl der Tabelle einer CSV: **immer ein Rechteck aus Zellen**,
/// gegeben durch einen Anker und ein Ende in Zeile und Feld (E10 des Plans
/// `261018-0822_*_plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md`).
///
/// **Rein und ohne AppKit.** Eine ganze Zeile und eine ganze Spalte sind
/// Rechtecke ueber alle Felder oder alle Zeilen und kein eigener Fall. Jede
/// Handlung an diesem Wert beschneidet auf das [`Tabellenmass`], also liegt
/// das Rechteck immer in der Tabelle und umfasst mindestens eine Zelle; eine
/// Tabelle ohne Zeilen hat keine Auswahl, und das steht als `None` beim
/// Halter und nicht als leerer Wert hier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zellauswahl {
    /// Wo die Auswahl begann: der Klick, die Zeile, der Kopf.
    anker: (usize, usize),
    /// Wohin sie reicht: das Ende eines Umschaltklicks, eines Ziehens, eines
    /// Umschaltpfeils. Die Pfeile ohne Umschalttaste gehen von hier aus.
    ende: (usize, usize),
}

impl Zellauswahl {
    /// Die Auswahl genau einer Zelle, auf das Mass beschnitten; `None` ohne
    /// Zeilen.
    #[must_use]
    pub fn zelle(zeile: usize, feld: usize, mass: Tabellenmass) -> Option<Self> {
        let punkt = beschnitten((zeile, feld), mass)?;
        Some(Self {
            anker: punkt,
            ende: punkt,
        })
    }

    /// Die Auswahl, die ein Zellbereich des Kerns meint, auf das Mass
    /// beschnitten; `None` ohne Zeilen. Ein leerer Bereich meint seine erste
    /// Zelle.
    #[must_use]
    pub fn aus_bereich(bereich: &Zellbereich, mass: Tabellenmass) -> Option<Self> {
        let anker = beschnitten((bereich.zeilen.start, bereich.spalten.start), mass)?;
        let ende = beschnitten(
            (
                bereich
                    .zeilen
                    .end
                    .saturating_sub(1)
                    .max(bereich.zeilen.start),
                bereich
                    .spalten
                    .end
                    .saturating_sub(1)
                    .max(bereich.spalten.start),
            ),
            mass,
        )?;
        Some(Self { anker, ende })
    }

    /// Alle Zellen der Tabelle; `None` ohne Zeilen.
    #[must_use]
    pub fn alles(mass: Tabellenmass) -> Option<Self> {
        Some(Self {
            anker: (0, 0),
            ende: mass.letzte()?,
        })
    }

    /// Der Anker, in Zeile und Feld.
    #[must_use]
    pub fn anker(self) -> (usize, usize) {
        self.anker
    }

    /// Das Ende, in Zeile und Feld.
    #[must_use]
    pub fn ende(self) -> (usize, usize) {
        self.ende
    }

    /// Das Rechteck als Zellbereich des Kerns, nie leer.
    #[must_use]
    pub fn bereich(self) -> Zellbereich {
        let (oben, unten) = (self.anker.0.min(self.ende.0), self.anker.0.max(self.ende.0));
        let (links, rechts) = (self.anker.1.min(self.ende.1), self.anker.1.max(self.ende.1));
        Zellbereich {
            zeilen: oben..unten + 1,
            spalten: links..rechts + 1,
        }
    }

    /// Ob die Zelle in Zeile `zeile` und Feld `feld` im Rechteck liegt.
    #[must_use]
    pub fn enthaelt(self, zeile: usize, feld: usize) -> bool {
        let bereich = self.bereich();
        bereich.zeilen.contains(&zeile) && bereich.spalten.contains(&feld)
    }

    /// Dieselbe Auswahl mit dem Ende an der genannten Zelle: der
    /// Umschaltklick und das Ziehen.
    #[must_use]
    pub fn bis(self, zeile: usize, feld: usize, mass: Tabellenmass) -> Option<Self> {
        Some(Self {
            anker: beschnitten(self.anker, mass)?,
            ende: beschnitten((zeile, feld), mass)?,
        })
    }

    /// Die Auswahl nach einer Pfeiltaste: ohne `ausdehnen` eine Zelle neben
    /// dem Ende, mit `ausdehnen` dasselbe Rechteck mit dem Ende eine Zelle
    /// weiter; am Rand bleibt es, wo es ist.
    #[must_use]
    pub fn bewegt(self, pfeil: Pfeil, ausdehnen: bool, mass: Tabellenmass) -> Option<Self> {
        let (letzte_zeile, letztes_feld) = mass.letzte()?;
        let (zeile, feld) = beschnitten(self.ende, mass)?;
        let ziel = match pfeil {
            Pfeil::Hoch => (zeile.saturating_sub(1), feld),
            Pfeil::Runter => ((zeile + 1).min(letzte_zeile), feld),
            Pfeil::Links => (zeile, feld.saturating_sub(1)),
            Pfeil::Rechts => (zeile, (feld + 1).min(letztes_feld)),
        };
        if ausdehnen {
            self.bis(ziel.0, ziel.1, mass)
        } else {
            Self::zelle(ziel.0, ziel.1, mass)
        }
    }

    /// Dieselbe Auswahl auf ein neues Mass beschnitten, nach einem Umbau.
    #[must_use]
    pub fn beschnitten(self, mass: Tabellenmass) -> Option<Self> {
        self.bis(self.ende.0, self.ende.1, mass)
    }
}

/// Ein Punkt in Zeile und Feld, auf das Mass beschnitten.
fn beschnitten(punkt: (usize, usize), mass: Tabellenmass) -> Option<(usize, usize)> {
    let (letzte_zeile, letztes_feld) = mass.letzte()?;
    Some((punkt.0.min(letzte_zeile), punkt.1.min(letztes_feld)))
}

/// Was ein Ziehen mit gedrueckter Maustaste ausdehnt: Zellen, wenn der Klick
/// davor in eine Zelle fiel, ganze Zeilen, wenn er in die Nummernspalte fiel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ziehart {
    /// Das Ziehen setzt das Ende auf die Zelle unter dem Zeiger.
    Zellen,
    /// Das Ziehen setzt das Ende auf die ganze Zeile unter dem Zeiger.
    Zeilen,
    /// Das Ziehen tut nichts, etwa nach einem Klick unter die Zeilen.
    Keine,
}

/// Die Auswahl nach einem Klick (E10): ein Klick setzt Anker und Ende, ein
/// Umschaltklick allein das Ende; die Nummer waehlt die Zeile, der Kopf die
/// Spalte, mit der Umschalttaste bis zu ihr. Dazu, was ein Ziehen danach
/// ausdehnt.
///
/// **Rein und ohne Fenster pruefbar**; die Umrechnung der Spaltennummer von
/// AppKit in ein Feld ist vorher geschehen ([`klickort`]).
#[must_use]
pub fn klick_waehlen(
    vorher: Option<Zellauswahl>,
    ort: Klickort,
    umschalt: bool,
    mass: Tabellenmass,
) -> (Option<Zellauswahl>, Ziehart) {
    let Some((letzte_zeile, letztes_feld)) = mass.letzte() else {
        return (None, Ziehart::Keine);
    };
    let anker = vorher.filter(|_| umschalt).map(Zellauswahl::anker);
    let auswahl = match ort {
        Klickort::Zelle { zeile, feld } => match vorher.filter(|_| umschalt) {
            Some(vorher) => vorher.bis(zeile, feld, mass),
            None => Zellauswahl::zelle(zeile, feld, mass),
        },
        Klickort::Nummer { zeile } => Some(Zellauswahl {
            anker: (anker.map_or(zeile, |anker| anker.0).min(letzte_zeile), 0),
            ende: (zeile.min(letzte_zeile), letztes_feld),
        }),
        Klickort::Kopf { feld } => Some(Zellauswahl {
            anker: (0, anker.map_or(feld, |anker| anker.1).min(letztes_feld)),
            ende: (letzte_zeile, feld.min(letztes_feld)),
        }),
    };
    let ziehart = match ort {
        Klickort::Zelle { .. } => Ziehart::Zellen,
        Klickort::Nummer { .. } => Ziehart::Zeilen,
        Klickort::Kopf { .. } => Ziehart::Keine,
    };
    (auswahl, ziehart)
}

/// Die Auswahl, waehrend die Maus nach einem Klick ueber Zeile `zeile` und
/// Feld `feld` gezogen wird; beides ist schon auf die Tabelle beschnitten.
#[must_use]
pub fn ziehen_waehlen(
    vorher: Option<Zellauswahl>,
    ziehart: Ziehart,
    zeile: usize,
    feld: usize,
    mass: Tabellenmass,
) -> Option<Zellauswahl> {
    let vorher = vorher?;
    match ziehart {
        Ziehart::Zellen => vorher.bis(zeile, feld, mass),
        Ziehart::Zeilen => {
            let letztes_feld = mass.letzte()?.1;
            Some(Zellauswahl {
                anker: (beschnitten(vorher.anker, mass)?.0, 0),
                ende: (beschnitten((zeile, 0), mass)?.0, letztes_feld),
            })
        }
        Ziehart::Keine => Some(vorher),
    }
}

/// Wohin ein Klick an AppKits Zeile `zeile` und Tabellenspalte `spalte`
/// faellt, ueber die Umrechnung an [`Zeilen`]; `None` ausserhalb jeder Zeile
/// und jenseits der letzten Spalte, und in jeder Art ausser der Tabelle
/// einer CSV, die allein eine Zellauswahl traegt.
#[must_use]
pub fn klickort(zeilen: &Zeilen, zeile: NSInteger, spalte: NSInteger) -> Option<Klickort> {
    if zeilen.art() != Eintragsart::Trenntext {
        return None;
    }
    let zeile = zeilen.stelle_der_zeile(usize::try_from(zeile).ok()?)?;
    let spalte = usize::try_from(spalte).ok()?;
    if spalte == NUMMERNSPALTE {
        return Some(Klickort::Nummer { zeile });
    }
    let feld = zeilen.feld_der_tabellenspalte(spalte)?;
    Some(Klickort::Zelle { zeile, feld })
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
    /// `cmd+v` in der Tabelle einer CSV ohne laufende Zelle: der Text der
    /// Zwischenablage soll als Zellbereich ab der Auswahl eingesetzt werden
    /// (E12 des Plans der CSV-Tabelle). Der Text kommt allein aus
    /// `zwischenablage::nur_text_lesen`; was daraus wird, rechnet der Editor.
    pub einfuegen: Box<dyn Fn(&str)>,
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
            if let Some(Some(text)) = self.mit_ansicht(Eintragsansicht::gewaehlter_text) {
                let _ = zwischenablage::text_schreiben(&text);
            }
        }

        /// `cmd+v` und "Bearbeiten › Einfügen" mit der Tabelle als
        /// Ersthelfer: in der Tabelle einer CSV der Text der Zwischenablage
        /// als Zellbereich ab der Auswahl (E12 des Plans der CSV-Tabelle).
        ///
        /// **Gelesen wird allein Text**, ueber
        /// [`zwischenablage::nur_text_lesen`]: ein im Finder kopierter Eintrag
        /// kaeme sonst als Pfad in die Zellen. Was daraus wird, rechnet der
        /// Editor ueber [`Zellenwege::einfuegen`]. In jeder anderen Art geht
        /// die Aktion an das naechste Glied der Antwortkette, als stuende
        /// diese Methode nicht da; eine laufende Zelle sieht sie nie, denn
        /// dann ist ihr Feldeditor der Ersthelfer und fuegt selbst ein.
        // SAFETY: Die Signatur ist die einer Aktion: ein optionales
        // Objektargument, keine Rueckgabe.
        #[unsafe(method(paste:))]
        fn einfuegen(&self, absender: Option<&AnyObject>) {
            let trenntext = self
                .mit_ansicht(|ansicht| ansicht.art() == Eintragsart::Trenntext)
                .unwrap_or(false);
            if !trenntext {
                self.weiterreichen(sel!(paste:), absender);
                return;
            }
            // Ohne Text geht ein leerer Text weiter und nicht nichts: der
            // Editor antwortet darauf mit `Trenntextantwort::AblageOhneText`
            // in der Statuszeile, statt dass `cmd+v` stumm bleibt
            // (`261019-0037-cmd-v-in-der-csv-tabelle-ohne-text-in-der-ablage-bleibt-stumm.md`).
            let text = zwischenablage::nur_text_lesen().unwrap_or_default();
            let _ = self.mit_ansicht(|ansicht| ansicht.bereich_einfuegen(&text));
        }

        /// `cmd+a` mit der Tabelle als Ersthelfer: in der Tabelle einer CSV
        /// jede Zelle (E10), in jeder anderen Art die Antwort der Oberklasse.
        // SAFETY: wie bei `paste:`.
        #[unsafe(method(selectAll:))]
        fn alles_waehlen(&self, absender: Option<&AnyObject>) {
            if self.mit_ansicht(Eintragsansicht::alles_waehlen) != Some(true) {
                // SAFETY: Die Oberklasse beantwortet dieselbe Aktion mit
                // demselben Argument.
                let _: () = unsafe { msg_send![super(self), selectAll: absender] };
            }
        }

        /// Ein Mausklick: in der Tabelle einer CSV waehlt er Zellen, Zeilen
        /// und Spalten selbst (E10), und die Oberklasse sieht ihn nicht; in
        /// jeder anderen Art geht er an sie wie bisher.
        ///
        /// **Die Oberklasse faehrt bei einem Klick ihre eigene Verfolgung**,
        /// und ein Ziehen kaeme dann nie hier an. Ohne sie stellt das Fenster
        /// die folgenden `mouseDragged:` dieser Ansicht zu, und der
        /// Doppelklick kommt als `clickCount` 2 statt ueber die Doppelaktion;
        /// er geht durch dieselbe Regel [`doppelklick`].
        // SAFETY: Die Signatur entspricht der von NSResponder: ein Ereignis,
        // keine Rueckgabe.
        #[unsafe(method(mouseDown:))]
        fn maus_gedrueckt(&self, ereignis: &NSEvent) {
            if self.mit_ansicht(|ansicht| ansicht.maus_gedrueckt(ereignis)) != Some(true) {
                // SAFETY: Die Oberklasse beantwortet dieselbe Nachricht mit
                // demselben Argument.
                let _: () = unsafe { msg_send![super(self), mouseDown: ereignis] };
            }
        }

        /// Ein Ziehen nach einem Klick, der in der Tabelle einer CSV hier
        /// beantwortet wurde.
        // SAFETY: wie bei `mouseDown:`.
        #[unsafe(method(mouseDragged:))]
        fn maus_gezogen(&self, ereignis: &NSEvent) {
            if self.mit_ansicht(|ansicht| ansicht.maus_gezogen(ereignis)) != Some(true) {
                // SAFETY: wie bei `mouseDown:`.
                let _: () = unsafe { msg_send![super(self), mouseDragged: ereignis] };
            }
        }
    }
);

impl Eintragstabelle {
    /// Ruft `tun` mit der Eintragsansicht, die diese Tabelle als Datenquelle
    /// fuehrt; `None` ohne eine.
    ///
    /// Gelesen ueber die Datenquelle und nicht ueber einen eigenen Verweis:
    /// die Tabelle hat keine Ivars, und die Datenquelle ist die eine
    /// Verbindung, die schon besteht.
    fn mit_ansicht<R>(&self, tun: impl FnOnce(&Eintragsansicht) -> R) -> Option<R> {
        // SAFETY: Ein Leser ohne Vorbedingung; die Datenquelle ist eine
        // nullende schwache Eigenschaft.
        let quelle = unsafe { self.dataSource() }?;
        let ansicht = AsRef::<AnyObject>::as_ref(&*quelle).downcast_ref::<Eintragsansicht>()?;
        Some(tun(ansicht))
    }

    /// Reicht eine Aktion an das naechste Glied der Antwortkette weiter, als
    /// beantwortete diese Tabelle sie nicht.
    fn weiterreichen(&self, aktion: Sel, absender: Option<&AnyObject>) {
        // SAFETY: Ein Leser ohne Vorbedingung; das naechste Glied haelt die
        // Antwortkette, solange die Tabelle in einem Fenster steht.
        if let Some(naechstes) = unsafe { self.nextResponder() } {
            // SAFETY: `tryToPerform:with:` schickt die Aktion mit dem einen
            // Objektargument, das jede Aktion traegt, an das erste Glied, das
            // sie beantwortet.
            let _ = unsafe { naechstes.tryToPerform_with(aktion, absender) };
        }
    }
}

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
    /// Der zuletzt zerlegte Satz der Tabelle einer CSV, fuer die uebrigen
    /// Zellen seiner Zeile (E9); siehe [`Satzspeicher`].
    satzspeicher: RefCell<Option<Satzspeicher>>,
    /// Die Rechteckauswahl der Tabelle einer CSV (E10); `None` in jeder
    /// anderen Art und in einer Tabelle ohne Zeilen.
    ///
    /// **Die Auswahl dieser Art steht hier und nicht in der Tabelle**: deren
    /// Zeilenauswahl kennt kein Rechteck, und ihre Hervorhebung ist in dieser
    /// Art aus ([`spalten_einrichten`]). Geschrieben allein ueber
    /// [`Eintragsansicht::zellauswahl_setzen`], das die Zellen danach
    /// nachzeichnet.
    zellauswahl: Cell<Option<Zellauswahl>>,
    /// Was ein Ziehen nach dem letzten Klick in die Tabelle einer CSV
    /// ausdehnt; gesetzt beim Klick und gelesen allein beim Ziehen danach.
    ziehart: Cell<Ziehart>,
    /// Ob die naechste Sicht einer CSV eine neue Datei zeigt: dann entstehen
    /// ihre Spalten mit Anfangsbreiten neu, und die Auswahl beginnt bei A1,
    /// auch wenn die Tabelle eben eine andere CSV zeigte. Gesetzt allein von
    /// [`Eintragsansicht::neue_datei`], verbraucht von
    /// [`Eintragsansicht::zeilen_zeigen`].
    frisch: Cell<bool>,
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
            let stelle = tabelle.columnWithIdentifier(&spalte.identifier());
            match self.art() {
                Eintragsart::Termine => {
                    if usize::try_from(stelle) != Ok(DATUMSSPALTE) {
                        return;
                    }
                    self.fokus_in_die_tabelle();
                    if let Some(wege) = self.ivars().wege.borrow().as_ref() {
                        (wege.kopf_geklickt)();
                    }
                }
                // Die Tabelle einer CSV waehlt mit dem Kopf die Spalte, mit
                // der Umschalttaste bis zu ihr, und mit dem Kopf der
                // Nummernspalte jede Zelle (E10); auch hier zuerst der Fokus.
                Eintragsart::Trenntext => {
                    self.fokus_in_die_tabelle();
                    let umschalt = NSApplication::sharedApplication(self.ivars().mtm)
                        .currentEvent()
                        .is_some_and(|ereignis| {
                            ereignis.modifierFlags().contains(NSEventModifierFlags::Shift)
                        });
                    self.kopf_angeklickt(stelle, umschalt);
                }
                Eintragsart::Aufgaben | Eintragsart::Notizen | Eintragsart::Geheimnisse => {}
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
            // Die Spalte als Feld, wie jede Spaltennummer von AppKit; die
            // Nummernspalte einer CSV zaehlt wie keine Spalte, und der Klick
            // oeffnet die erste Zelle der Zeile.
            let feld = self
                .feld_an(tabelle.clickedColumn())
                .and_then(|feld| NSInteger::try_from(feld).ok())
                .unwrap_or(-1);
            self.doppelklick_ausfuehren(doppelklick(
                self.stelle_an(tabelle.clickedRow()),
                feld,
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

        let zeilen = Zeilen::Aufgaben(Vec::new());
        spalten_einrichten(mtm, &tabelle, &rolle, &kopfzeile, &zeilen);

        let this = Self::alloc(mtm).set_ivars(EintragsansichtIvars {
            rolle,
            tabelle,
            kopfzeile,
            zeilen: RefCell::new(zeilen),
            satzspeicher: RefCell::new(None),
            zellauswahl: Cell::new(None),
            ziehart: Cell::new(Ziehart::Keine),
            frisch: Cell::new(false),
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
        let art = zeilen.art();
        // Eine neue CSV richtet ihre Spalten neu ein, auch wenn die Tabelle
        // eben eine andere zeigte und der Vergleich der Generation sonst nur
        // nachzoege ([`Self::neue_datei`]).
        let frisch = art == Eintragsart::Trenntext && self.ivars().frisch.replace(false);
        if !frisch && *self.ivars().zeilen.borrow() == zeilen {
            return;
        }
        let laenge = zeilen.len();
        let artwechsel = self.art() != art || frisch;
        let vorher = if artwechsel {
            None
        } else {
            self.gewaehlte_stelle()
        };
        let vorige_auswahl = if artwechsel {
            None
        } else {
            self.ivars().zellauswahl.get()
        };
        // Die Ausleihe endet an ihrem Semikolon: `reloadData` fragt die Zeilen
        // gleich wieder ab.
        *self.ivars().zeilen.borrow_mut() = zeilen;
        let ivars = self.ivars();
        if artwechsel {
            spalten_einrichten(
                ivars.mtm,
                &ivars.tabelle,
                &ivars.rolle,
                &ivars.kopfzeile,
                &ivars.zeilen.borrow(),
            );
            self.richtungspfeil_setzen();
        } else if let Zeilen::Trenntext(sicht) = &*ivars.zeilen.borrow() {
            // Dieselbe Art, ein neuer Stand: die Spalten der Datei koennen
            // dazugekommen oder fortgefallen sein, und der Kopf zeigt den Text
            // der ersten Zeile. Gezogene Breiten bleiben stehen.
            trenntextspalten_angleichen(ivars.mtm, &ivars.tabelle, sicht, false);
        }
        // Die Rechteckauswahl einer CSV steht vor dem Neuladen, damit die neu
        // gebauten Zellen sie schon zeigen: die vorige auf das neue Mass
        // beschnitten, ohne eine die Zelle A1 (E10). In jeder anderen Art
        // gibt es kein Mass und damit keine.
        let zellauswahl = self.tabellenmass().and_then(|mass| match vorige_auswahl {
            Some(auswahl) => auswahl.beschnitten(mass),
            None => Zellauswahl::zelle(0, 0, mass),
        });
        self.ivars().zellauswahl.set(zellauswahl);
        if art == Eintragsart::Trenntext && artwechsel {
            // SAFETY: Eine Aktion mit optionalem Absender; ohne Absender
            // nimmt die Tabelle allein ihre Zeilenauswahl zurueck.
            unsafe { self.ivars().tabelle.deselectAll(None) };
        }
        self.ivars().tabelle.reloadData();
        // Die Tabelle einer CSV waehlt ueber ihre Rechteckauswahl oben und
        // nicht ueber die Zeilenauswahl von AppKit.
        if art == Eintragsart::Trenntext {
            return;
        }
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
            Zeilen::Aufgaben(_)
            | Zeilen::Notizen(_)
            | Zeilen::Geheimnisse(_)
            | Zeilen::Trenntext(_) => false,
        }
    }

    /// Das Mass der Tabelle einer CSV; `None` in jeder anderen Art.
    #[must_use]
    fn tabellenmass(&self) -> Option<Tabellenmass> {
        match &*self.ivars().zeilen.borrow() {
            Zeilen::Trenntext(sicht) => Some(Tabellenmass {
                zeilen: sicht.index().zeilenzahl(),
                spalten: sicht.index().spaltenzahl(),
            }),
            Zeilen::Aufgaben(_)
            | Zeilen::Notizen(_)
            | Zeilen::Termine(_)
            | Zeilen::Geheimnisse(_) => None,
        }
    }

    /// Die Rechteckauswahl der Tabelle einer CSV; `None` in jeder anderen Art
    /// und ohne Zeilen. Allein fuer die Proben; der Betrieb fragt
    /// [`Self::gewaehlter_bereich`].
    #[cfg(test)]
    #[must_use]
    pub(super) fn zellauswahl(&self) -> Option<Zellauswahl> {
        self.ivars().zellauswahl.get()
    }

    /// Die Auswahl der Tabelle einer CSV als Zellbereich des Kerns, den die
    /// Handlungen nehmen; `None` in jeder anderen Art und ohne Zeilen.
    #[must_use]
    pub fn gewaehlter_bereich(&self) -> Option<Zellbereich> {
        self.ivars().zellauswahl.get().map(Zellauswahl::bereich)
    }

    /// Setzt die Auswahl der Tabelle einer CSV auf den Bereich, den eine
    /// Handlung des Kerns danach nennt (E10), und bringt ihr Ende ins Bild; in
    /// jeder anderen Art tut es nichts.
    pub fn bereich_setzen(&self, bereich: &Zellbereich) {
        let Some(mass) = self.tabellenmass() else {
            return;
        };
        self.zellauswahl_setzen(Zellauswahl::aus_bereich(bereich, mass), true);
    }

    /// Sagt der Ansicht, dass die naechste Sicht einer CSV eine neue Datei
    /// zeigt: ihre Spalten entstehen mit Anfangsbreiten neu, und die Auswahl
    /// beginnt bei A1. Ohne diesen Ruf behielte eine CSV die gezogenen
    /// Breiten der zuvor gezeigten, denn beide sind dieselbe Art.
    ///
    /// Gerufen vom Editorbereich, wenn eine andere Datei eintrifft oder das
    /// Trennzeichen wechselt und damit die Spalten etwas anderes meinen.
    pub fn neue_datei(&self) {
        self.ivars().frisch.set(true);
    }

    /// Schreibt die Rechteckauswahl und zeichnet die Zellen nach; mit
    /// `zeigen` rollt die Tabelle ihr Ende ins Bild.
    ///
    /// **Die eine Stelle, an der die Auswahl geschrieben wird**, ausser dem
    /// Neuladen in [`Self::zeilen_zeigen`], das die Zellen ohnehin neu baut.
    fn zellauswahl_setzen(&self, auswahl: Option<Zellauswahl>, zeigen: bool) {
        self.ivars().zellauswahl.set(auswahl);
        if zeigen && let Some(auswahl) = auswahl {
            let (stelle, feld) = auswahl.ende();
            let ziel = {
                let zeilen = self.ivars().zeilen.borrow();
                zeilen
                    .zeile_der_stelle(stelle)
                    .map(|zeile| (zeile, zeilen.tabellenspalte_des_feldes(feld)))
            };
            if let Some((zeile, spalte)) = ziel {
                let tabelle = &self.ivars().tabelle;
                tabelle.scrollRowToVisible(NSInteger::try_from(zeile).unwrap_or(NSInteger::MAX));
                tabelle
                    .scrollColumnToVisible(NSInteger::try_from(spalte).unwrap_or(NSInteger::MAX));
            }
        }
        self.auswahl_zeichnen();
    }

    /// Faerbt jede gebaute Zelle der Tabelle einer CSV nach der Auswahl
    /// (E10).
    ///
    /// **Jede Zeile, die AppKit gerade haelt**, ueber
    /// `enumerateAvailableRowViewsUsingBlock:`, und nicht allein die
    /// sichtbaren: die Tabelle baut Zeilen ueber den sichtbaren Teil hinaus vor
    /// und fragt sie beim Rollen nicht neu ab, und eine solche Zelle zeigte
    /// sonst die Auswahl von vorher. Gefragt werden allein schon gebaute
    /// Zellen (`makeIfNecessary` nein); jede spaeter gebaute bekommt ihre Farbe
    /// in [`Self::zellenansicht`]. **Kein `reloadData`**: eine Auswahl ist kein
    /// neuer Stand, und ein Neuladen baute bei jedem Schritt eines Ziehens jede
    /// Zelle neu.
    fn auswahl_zeichnen(&self) {
        let ivars = self.ivars();
        let auswahl = ivars.zellauswahl.get();
        let tabelle = &ivars.tabelle;
        let gehaltene = RefCell::new(Vec::new());
        let sammler = StackBlock::new(|_: NonNull<NSTableRowView>, zeile: NSInteger| {
            gehaltene.borrow_mut().push(zeile);
        });
        tabelle.enumerateAvailableRowViewsUsingBlock(&sammler);
        let spalten = tabelle.numberOfColumns();
        for zeile in gehaltene.into_inner() {
            let Some(stelle) = self.stelle_an(zeile) else {
                continue;
            };
            for spalte in 0..spalten {
                let Some(feld) = self.feld_an(spalte) else {
                    continue;
                };
                let textfeld = tabelle
                    .viewAtColumn_row_makeIfNecessary(spalte, zeile, false)
                    .and_then(|zelle| zelle.downcast::<NSTableCellView>().ok())
                    // SAFETY: Ein Leser ohne Vorbedingung.
                    .and_then(|zelle| unsafe { zelle.textField() });
                if let Some(textfeld) = textfeld {
                    auswahl_faerben(
                        &textfeld,
                        auswahl.is_some_and(|auswahl| auswahl.enthaelt(stelle, feld)),
                    );
                }
            }
        }
    }

    /// Ein Klick an diesen Ort der Tabelle einer CSV, mit oder ohne
    /// Umschalttaste, nach der reinen Regel [`klick_waehlen`].
    ///
    /// `pub(super)` allein fuer die Proben in [`super::editor`], die kein
    /// Mausereignis haben und den Ort deshalb hereinreichen.
    pub(super) fn klick(&self, ort: Option<Klickort>, umschalt: bool) {
        let (Some(ort), Some(mass)) = (ort, self.tabellenmass()) else {
            self.ivars().ziehart.set(Ziehart::Keine);
            return;
        };
        let (auswahl, ziehart) = klick_waehlen(self.ivars().zellauswahl.get(), ort, umschalt, mass);
        self.ivars().ziehart.set(ziehart);
        self.zellauswahl_setzen(auswahl, true);
    }

    /// Ein Klick auf den Kopf der Tabellenspalte `spalte` einer CSV: die
    /// Spalte, und der Kopf der Nummernspalte jede Zelle (E10).
    ///
    /// `pub(super)` aus demselben Grund wie [`Self::klick`].
    pub(super) fn kopf_angeklickt(&self, spalte: NSInteger, umschalt: bool) {
        if usize::try_from(spalte) == Ok(NUMMERNSPALTE) {
            let _ = self.alles_waehlen();
            return;
        }
        let ort = self.feld_an(spalte).map(|feld| Klickort::Kopf { feld });
        self.klick(ort, umschalt);
    }

    /// Ein Mausklick in die Tabelle; `true`, wenn sie die einer CSV ist und
    /// der Klick hier beantwortet wurde (Doc-Kommentar an
    /// [`Eintragstabelle`], `mouseDown:`).
    ///
    /// **Zuerst der Fokus**, ueber `makeFirstResponder:` des Fensters: eine
    /// laufende Zelle endet dabei ueber die zwei Delegiertenwege wie bei jedem
    /// Klick daneben, und lehnt AppKit das ab, geschieht nichts weiter.
    fn maus_gedrueckt(&self, ereignis: &NSEvent) -> bool {
        if self.art() != Eintragsart::Trenntext {
            return false;
        }
        let tabelle = &self.ivars().tabelle;
        if let Some(fenster) = tabelle.window()
            && !fenster.makeFirstResponder(Some(tabelle))
        {
            return true;
        }
        let ort = tabelle.convertPoint_fromView(ereignis.locationInWindow(), None);
        let (zeile, spalte) = (tabelle.rowAtPoint(ort), tabelle.columnAtPoint(ort));
        if ereignis.clickCount() >= 2 {
            let feld = self
                .feld_an(spalte)
                .and_then(|feld| NSInteger::try_from(feld).ok())
                .unwrap_or(-1);
            self.doppelklick_ausfuehren(doppelklick(self.stelle_an(zeile), feld, true));
            return true;
        }
        let umschalt = ereignis
            .modifierFlags()
            .contains(NSEventModifierFlags::Shift);
        let ort = klickort(&self.ivars().zeilen.borrow(), zeile, spalte);
        self.klick(ort, umschalt);
        true
    }

    /// Ein Ziehen nach einem Klick in die Tabelle einer CSV: das Ende folgt
    /// dem Zeiger, auf die Tabelle beschnitten, und die Rolle folgt mit.
    fn maus_gezogen(&self, ereignis: &NSEvent) -> bool {
        let Some(mass) = self.tabellenmass() else {
            return false;
        };
        let tabelle = &self.ivars().tabelle;
        let _ = tabelle.autoscroll(ereignis);
        let ort = tabelle.convertPoint_fromView(ereignis.locationInWindow(), None);
        let letzte_zeile = mass.zeilen.saturating_sub(1);
        let zeile = self
            .stelle_an(tabelle.rowAtPoint(ort))
            .unwrap_or(if ort.y < 0.0 { 0 } else { letzte_zeile });
        let spalte = tabelle.columnAtPoint(ort);
        let feld = self.feld_an(spalte).unwrap_or(
            if ort.x < 0.0 || usize::try_from(spalte) == Ok(NUMMERNSPALTE) {
                0
            } else {
                mass.spalten.saturating_sub(1)
            },
        );
        let auswahl = ziehen_waehlen(
            self.ivars().zellauswahl.get(),
            self.ivars().ziehart.get(),
            zeile,
            feld,
            mass,
        );
        self.zellauswahl_setzen(auswahl, false);
        true
    }

    /// Fuehrt eine Tabellentaste aus.
    ///
    /// `pub(super)` aus demselben Grund wie [`Self::klick`].
    pub(super) fn tabellentaste_ausfuehren(&self, taste: Tabellentaste) {
        let Some(mass) = self.tabellenmass() else {
            return;
        };
        match taste {
            Tabellentaste::Pfeil(pfeil, ausdehnen) => {
                let auswahl = self
                    .ivars()
                    .zellauswahl
                    .get()
                    .and_then(|auswahl| auswahl.bewegt(pfeil, ausdehnen, mass))
                    .or_else(|| Zellauswahl::zelle(0, 0, mass));
                self.zellauswahl_setzen(auswahl, true);
            }
            Tabellentaste::Oeffnen => {
                if let Some(bereich) = self.gewaehlter_bereich() {
                    let _ = self.zelle_beginnen(Zelle {
                        stelle: bereich.zeilen.start,
                        spalte: bereich.spalten.start,
                    });
                }
            }
        }
    }

    /// `cmd+a` in der Tabelle einer CSV: jede Zelle (E10); `false` in jeder
    /// anderen Art.
    #[must_use = "in jeder anderen Art beantwortet die Oberklasse die Aktion"]
    pub(super) fn alles_waehlen(&self) -> bool {
        let Some(mass) = self.tabellenmass() else {
            return false;
        };
        self.zellauswahl_setzen(Zellauswahl::alles(mass), false);
        true
    }

    /// `cmd+v` in der Tabelle einer CSV mit dem Text der Zwischenablage: geht
    /// an den Editor ueber [`Zellenwege::einfuegen`].
    fn bereich_einfuegen(&self, text: &str) {
        if let Some(wege) = self.ivars().wege.borrow().as_ref() {
            (wege.einfuegen)(text);
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
        // In der Tabelle einer CSV die Zelle in dieser Zeile, im Feld des
        // bisherigen Endes; wer ein Rechteck meint, setzt danach
        // [`Self::bereich_setzen`].
        if self.art() == Eintragsart::Trenntext {
            let Some(stelle) = stelle else {
                return;
            };
            let feld = self
                .ivars()
                .zellauswahl
                .get()
                .map_or(0, |auswahl| auswahl.ende().1);
            let auswahl = self
                .tabellenmass()
                .and_then(|mass| Zellauswahl::zelle(stelle, feld, mass));
            self.zellauswahl_setzen(auswahl, true);
            return;
        }
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
        // Die Tabelle einer CSV waehlt ein Rechteck und keine Zeile; ihre
        // gewaehlte Stelle ist die oberste Zeile des Rechtecks.
        if self.art() == Eintragsart::Trenntext {
            return self
                .ivars()
                .zellauswahl
                .get()
                .map(|auswahl| auswahl.bereich().zeilen.start);
        }
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
    /// Von einem Termin ebenso sein Text, und das Datum nur ohne Text. Von
    /// einem Geheimnis seine Notizen, und der Name nur ohne Notizen: dieselbe
    /// Regel wie bei der Notiz, deren Form es traegt.
    #[must_use]
    pub(super) fn gewaehlter_text(&self) -> Option<String> {
        // Die Tabelle einer CSV kopiert ihr Rechteck als TSV (E11).
        if let Zeilen::Trenntext(sicht) = &*self.ivars().zeilen.borrow() {
            let bereich = self.ivars().zellauswahl.get()?.bereich();
            return Some(trenntext::bereich_als_tsv(
                sicht.text(),
                sicht.index(),
                &bereich,
            ));
        }
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
            Zeilen::Geheimnisse(zeilen) => zeilen.get(zeile).map(|geheimnis| {
                text_oder(geheimnis.feld(Feld::Notizen), geheimnis.feld(Feld::Name))
            }),
            // Oben beantwortet, ueber das Rechteck der Auswahl.
            Zeilen::Trenntext(_) => None,
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
        let spalte = self.feld_an(tabelle.columnForView(ansicht))?;
        Some(Zelle { stelle, spalte })
    }

    /// Das Feld, das AppKits Tabellenspalte `spalte` zeigt: die Umrechnung
    /// jeder Spaltennummer, die von AppKit hereinkommt. -1, die Nummernspalte
    /// der Tabelle einer CSV und eine Spalte jenseits der letzten ergeben
    /// `None`.
    fn feld_an(&self, spalte: NSInteger) -> Option<usize> {
        let spalte = usize::try_from(spalte).ok()?;
        self.ivars().zeilen.borrow().feld_der_tabellenspalte(spalte)
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
        // In der Tabelle einer CSV ist die geoeffnete Zelle die Auswahl, auch
        // nach `tab` in die naechste; ein Umbau danach behaelt sie.
        if self.art() == Eintragsart::Trenntext {
            let auswahl = self
                .tabellenmass()
                .and_then(|mass| Zellauswahl::zelle(zelle.stelle, zelle.spalte, mass));
            self.zellauswahl_setzen(auswahl, false);
        }
        let tabelle = &self.ivars().tabelle;
        tabelle.scrollRowToVisible(zeile);
        tabelle.scrollColumnToVisible(spalte);
        self.trenntextfeld_oeffnen(zelle, zeile, spalte);
        tabelle.editColumn_row_withEvent_select(spalte, zeile, None, true);
        true
    }

    /// Stellt in der Tabelle einer CSV das Feld der Zelle auf ihren
    /// wirklichen Wert und schaltet es bearbeitbar, bevor AppKit den
    /// Feldeditor hineinsetzt (E16, Modulkopf); in jeder anderen Art tut es
    /// nichts. Zurueckgeschaltet wird in [`Self::zelle_geendet`].
    fn trenntextfeld_oeffnen(&self, zelle: Zelle, zeile: NSInteger, spalte: NSInteger) {
        let wert = match &*self.ivars().zeilen.borrow() {
            Zeilen::Trenntext(sicht) => sicht.wert(zelle.stelle, zelle.spalte),
            Zeilen::Aufgaben(_)
            | Zeilen::Notizen(_)
            | Zeilen::Termine(_)
            | Zeilen::Geheimnisse(_) => {
                return;
            }
        };
        let feld = self
            .ivars()
            .tabelle
            .viewAtColumn_row_makeIfNecessary(spalte, zeile, true)
            .and_then(|zelle| zelle.downcast::<NSTableCellView>().ok())
            // SAFETY: Ein Leser ohne Vorbedingung.
            .and_then(|zelle| unsafe { zelle.textField() });
        if let Some(feld) = feld {
            feld.setStringValue(&NSString::from_str(&wert));
            trenntextfeld_schalten(&feld, true);
        }
    }

    /// Zeile und Spalte, an denen AppKit die genannte Zelle zeigt: die Zeile
    /// ueber [`Self::zeile_von`] aus der Stelle der Zelle, die Spalte ueber
    /// [`Zeilen::tabellenspalte_des_feldes`] aus ihrem Feld. `None`, wenn es
    /// den Eintrag oder die Spalte nicht gibt.
    ///
    /// `pub(super)` allein fuer die Proben in [`super::editor`]: der Beginn
    /// einer Bearbeitung braucht ein Fenster, das `libtest` nicht hergibt, und
    /// die Proben fahren deshalb die Umrechnung davor.
    #[must_use]
    pub(super) fn ziel_der_zelle(&self, zelle: Zelle) -> Option<(NSInteger, NSInteger)> {
        let zeile = NSInteger::try_from(self.zeile_von(zelle.stelle)?).ok()?;
        if zelle.spalte >= self.ivars().zeilen.borrow().spaltenzahl() {
            return None;
        }
        let spalte = self
            .ivars()
            .zeilen
            .borrow()
            .tabellenspalte_des_feldes(zelle.spalte);
        let spalte = NSInteger::try_from(spalte).ok()?;
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
        // Das Feld einer Zelle der Tabelle einer CSV ist allein waehrend der
        // Bearbeitung bearbeitbar (Modulkopf); nach einem Umbau steht es in
        // keiner Zeile mehr und kommt ueber `makeViewWithIdentifier:owner:`
        // wieder, wo es ebenso zurueckgeschaltet wird.
        if self.art() == Eintragsart::Trenntext {
            trenntextfeld_schalten(feld, false);
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
            Zeilen::Geheimnisse(zeilen) => zeilen
                .get(zeile)
                .map(|geheimnis| geheimnis.wert(zelle.spalte).to_owned()),
            Zeilen::Trenntext(sicht) => {
                Some(zellanzeige(&sicht.wert(zeile, zelle.spalte)).into_owned())
            }
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
        let Some(mehrzeilig) = mehrzeilige_spalte(self.art()) else {
            return;
        };
        let tabelle = &self.ivars().tabelle;
        let Ok(spalte) = NSInteger::try_from(mehrzeilig) else {
            return;
        };
        let breite = self.umbruchbreite(mehrzeilig);
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
        match &zeilen {
            Zeilen::Aufgaben(zeilen) => Some(self.aufgabenzelle(zeilen.get(zeile)?)),
            Zeilen::Notizen(zeilen) => {
                let notiz = zeilen.get(zeile)?;
                let inhalt = if spalte == NOTIZSPALTE {
                    &notiz.text
                } else {
                    &notiz.thema
                };
                Some(self.notizzelle(inhalt, spalte, Eintragsart::Notizen))
            }
            // Datum und Termin in denselben zwei Zellen wie Thema und Notiz.
            Zeilen::Termine(zeilen) => {
                let termin = zeilen.get(zeile)?;
                let inhalt = if spalte == TERMINSPALTE {
                    &termin.text
                } else {
                    &termin.datum
                };
                Some(self.notizzelle(inhalt, spalte, Eintragsart::Termine))
            }
            // Sechs Zellen derselben Bauart: fuenf einzeilige wie das Thema,
            // die Notizen mehrzeilig wie die Notiz.
            Zeilen::Geheimnisse(zeilen) => {
                let geheimnis = zeilen.get(zeile)?;
                Some(self.notizzelle(geheimnis.wert(spalte), spalte, Eintragsart::Geheimnisse))
            }
            // Die Sicht ist ein `Rc` und kein Vektor; die Abschrift oben kostet
            // hier nichts.
            Zeilen::Trenntext(sicht) => {
                if zeile >= sicht.index().zeilenzahl() {
                    return None;
                }
                if spalte == NUMMERNSPALTE {
                    return Some(self.trenntextzelle(&(zeile + 1).to_string(), true, false));
                }
                let feld = zeilen.feld_der_tabellenspalte(spalte)?;
                let inhalt = zellanzeige(&self.satzwert(sicht, zeile, feld)).into_owned();
                let gewaehlt = self
                    .ivars()
                    .zellauswahl
                    .get()
                    .is_some_and(|auswahl| auswahl.enthaelt(zeile, feld));
                Some(self.trenntextzelle(&inhalt, false, gewaehlt))
            }
        }
    }

    /// Der Wert der Zelle in Zeile `zeile` und Feld `feld`, aus dem
    /// zwischengespeicherten Satz, wenn er zu dieser Zeile und Sicht gehoert
    /// (E9, [`Satzspeicher`]).
    fn satzwert(&self, sicht: &Trenntextsicht, zeile: usize, feld: usize) -> String {
        let mut speicher = self.ivars().satzspeicher.borrow_mut();
        let passt = speicher.as_ref().is_some_and(|gespeichert| {
            gespeichert.generation == sicht.generation() && gespeichert.zeile == zeile
        });
        if !passt {
            *speicher = Some(Satzspeicher {
                generation: sicht.generation(),
                zeile,
                werte: sicht.werte(zeile),
            });
        }
        speicher
            .as_ref()
            .and_then(|gespeichert| gespeichert.werte.get(feld).cloned())
            .unwrap_or_default()
    }

    /// Die Ansicht einer Zelle der Tabelle einer CSV mit dem genannten Text:
    /// eine wiederverwendete, wenn AppKit eine hat, sonst eine neue (E9).
    ///
    /// **Das Feld ist nicht bearbeitbar**, auch das wiederverwendete nicht:
    /// es wird es allein in [`Self::trenntextfeld_oeffnen`] (Modulkopf).
    /// `nummer` nennt die Nummernspalte, die eine eigene Kennung traegt, damit
    /// eine Zelle der Nummern nie als Zelle eines Feldes wiederkommt;
    /// `gewaehlt`, ob die Zelle in der Rechteckauswahl liegt (E10), und das
    /// setzt jede, auch eine wiederverwendete, ueber [`auswahl_faerben`].
    fn trenntextzelle(&self, inhalt: &str, nummer: bool, gewaehlt: bool) -> Retained<NSView> {
        let kennung = if nummer {
            ns_string!("trenntextnummer")
        } else {
            ns_string!("trenntextzelle")
        };
        let tabelle = &self.ivars().tabelle;
        // SAFETY: Der Eigentuemer ist `None`; AppKit laedt dann kein Nib und
        // spricht kein Objekt an.
        let wieder = unsafe { tabelle.makeViewWithIdentifier_owner(kennung, None) }
            .and_then(|ansicht| ansicht.downcast::<NSTableCellView>().ok());
        let zelle = wieder.unwrap_or_else(|| self.trenntextzelle_bauen(kennung));
        // SAFETY: Ein Leser ohne Vorbedingung.
        if let Some(feld) = unsafe { zelle.textField() } {
            feld.setStringValue(&NSString::from_str(inhalt));
            trenntextfeld_schalten(&feld, false);
            auswahl_faerben(&feld, gewaehlt);
        }
        Retained::into_super(zelle)
    }

    /// Baut die Ansicht einer Zelle der Tabelle einer CSV: ein einzeiliges
    /// Feld, das am Ende kuerzt, an den Kanten seiner `NSTableCellView`.
    fn trenntextzelle_bauen(&self, kennung: &NSString) -> Retained<NSTableCellView> {
        let mtm = self.ivars().mtm;
        let feld = NSTextField::textFieldWithString(ns_string!(""), mtm);
        feld.setFont(Some(&NSFont::systemFontOfSize(NSFont::systemFontSize())));
        feld.setLineBreakMode(NSLineBreakMode::ByTruncatingTail);
        feld.setBordered(false);
        feld.setDrawsBackground(false);
        trenntextfeld_schalten(&feld, false);
        // Nicht einzeilig: `opt+return` schreibt beim Bearbeiten einen
        // Zeilenumbruch in den Wert (E16); gezeigt wird er als `⏎`, und die
        // Zelle bricht nicht um.
        feld.setUsesSingleLineMode(false);
        if let Some(zelle) = feld.cell() {
            zelle.setWraps(false);
            zelle.setScrollable(true);
        }
        feld.setFrame(NSRect::new(
            NSPoint::new(RAND, 2.0),
            NSSize::new(AUFBAUBREITE - 2.0 * RAND, ZEILENHOEHE - 4.0),
        ));
        feld.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
        // SAFETY: wie in `aufgabenzelle`.
        unsafe { feld.setDelegate(Some(ProtocolObject::from_ref(self))) };

        let zelle = NSTableCellView::initWithFrame(
            NSTableCellView::alloc(mtm),
            NSRect::new(NSPoint::ZERO, NSSize::new(AUFBAUBREITE, ZEILENHOEHE)),
        );
        zelle.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
        zelle.addSubview(&feld);
        // SAFETY: Das Feld ist eine Unteransicht der Zelle und lebt so lange
        // wie sie; die Eigenschaft ist `weak` (`NSTableCellView.h`).
        unsafe { zelle.setTextField(Some(&feld)) };
        zelle.setIdentifier(Some(kennung));
        zelle
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
    /// Thema und dem Termintext als Notiz, und die Geheimnistabelle alle sechs.
    /// Mehrzeilig ist allein die Spalte, die [`mehrzeilige_spalte`] der Art
    /// nennt; `inhalt` ist der Text der Zelle.
    fn notizzelle(&self, inhalt: &str, spalte: usize, art: Eintragsart) -> Retained<NSView> {
        let mtm = self.ivars().mtm;
        let ist_notiz = mehrzeilige_spalte(art) == Some(spalte);
        let feld = Notizfeld::neu(mtm, inhalt);
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
            feld.setPreferredMaxLayoutWidth(self.umbruchbreite(spalte));
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

/// Die Breite, mit der eine einzeilige Spalte der Geheimnistabelle entsteht,
/// ausser Protokoll und Adresse; der Nutzer kann jede ziehen.
const GEHEIMNISBREITE: f64 = 110.0;

/// Kennung, Titel und Anfangsbreite der Spalte eines Feldes der
/// Geheimnistabelle; vollstaendig ueber `Feld` und ohne Auffangzweig. Die
/// Kennung ist ein Bezeichner und bleibt ein Literal.
fn geheimnisspalte(feld: Feld) -> (&'static NSString, Text, f64) {
    match feld {
        Feld::Name => (
            ns_string!("name"),
            Text::EintragsspalteName,
            GEHEIMNISBREITE,
        ),
        Feld::Protokoll => (ns_string!("protokoll"), Text::EintragsspalteProtokoll, 70.0),
        Feld::Adresse => (ns_string!("adresse"), Text::EintragsspalteAdresse, 140.0),
        Feld::Konto => (
            ns_string!("konto"),
            Text::EintragsspalteKonto,
            GEHEIMNISBREITE,
        ),
        Feld::Passwort => (
            ns_string!("passwort"),
            Text::EintragsspaltePasswort,
            GEHEIMNISBREITE,
        ),
        Feld::Notizen => (
            ns_string!("notizen"),
            Text::EintragsspalteNotizen,
            GEHEIMNISBREITE,
        ),
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
///
/// **Die Tabelle einer CSV** traegt die Nummernspalte und so viele Spalten,
/// wie die Datei Felder hat, eine Kopfzeile, eine feste Zeilenhoehe und als
/// einzige Art einen waagerechten Rollbalken ohne Anpassung der
/// Spaltenbreiten an die Rolle (E8). Beides stellt diese Funktion fuer jede
/// Art ein, damit ein Wechsel zurueck keine Einstellung der CSV stehen
/// laesst; ihre Spalten zieht danach [`trenntextspalten_angleichen`] nach.
fn spalten_einrichten(
    mtm: MainThreadMarker,
    tabelle: &NSTableView,
    rolle: &NSScrollView,
    kopfzeile: &NSTableHeaderView,
    zeilen: &Zeilen,
) {
    let alte: Vec<_> = tabelle.tableColumns().iter().collect();
    for spalte in alte {
        tabelle.removeTableColumn(&spalte);
    }
    let art = zeilen.art();
    let waagerecht = rollt_waagerecht(art);
    rolle.setHasHorizontalScroller(waagerecht);
    tabelle.setColumnAutoresizingStyle(if waagerecht {
        NSTableViewColumnAutoresizingStyle::NoColumnAutoresizing
    } else {
        NSTableViewColumnAutoresizingStyle::LastColumnOnlyAutoresizingStyle
    });
    // Die Spalten der Datei stehen in ihrer Folge; ein Vertauschen am Kopf
    // truege die Nummer einer Tabellenspalte auf ein anderes Feld.
    tabelle.setAllowsColumnReordering(!waagerecht);
    // Die Tabelle einer CSV zeichnet ihre Rechteckauswahl selbst, ueber den
    // Hintergrund der Zellen (E10); die Hervorhebung der Zeile ist dort aus,
    // in jeder anderen Art an wie vorher.
    tabelle.setSelectionHighlightStyle(hervorhebung(art));
    // Die Kennung ist ein Bezeichner und bleibt ein Literal; der Titel kommt
    // aus der Sprachtabelle.
    let spalte_bauen = |kennung: &NSString, titel: Text| {
        let spalte = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), kennung);
        spalte.setTitle(&NSString::from_str(text(titel)));
        spalte
    };
    match art {
        Eintragsart::Aufgaben => {
            let spalte = spalte_bauen(ns_string!("aufgabe"), Text::EintragsspalteAufgabe);
            spalte.setResizingMask(NSTableColumnResizingOptions::AutoresizingMask);
            tabelle.addTableColumn(&spalte);
            tabelle.setUsesAutomaticRowHeights(false);
            tabelle.setHeaderView(None);
        }
        Eintragsart::Notizen => {
            let thema = spalte_bauen(ns_string!("thema"), Text::EintragsspalteThema);
            thema.setResizingMask(NSTableColumnResizingOptions::UserResizingMask);
            thema.setWidth(THEMENBREITE);
            thema.setMinWidth(THEMENBREITE / 2.0);
            tabelle.addTableColumn(&thema);
            let notiz = spalte_bauen(ns_string!("notiz"), Text::EintragsspalteNotiz);
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
            let datum = spalte_bauen(ns_string!("datum"), Text::EintragsspalteDatum);
            datum.setResizingMask(NSTableColumnResizingOptions::UserResizingMask);
            datum.setWidth(DATUMSBREITE);
            datum.setMinWidth(DATUMSBREITE / 2.0);
            tabelle.addTableColumn(&datum);
            let termin = spalte_bauen(ns_string!("termin"), Text::EintragsspalteTermin);
            termin.setResizingMask(
                NSTableColumnResizingOptions::AutoresizingMask
                    | NSTableColumnResizingOptions::UserResizingMask,
            );
            termin.setMinWidth(THEMENBREITE / 2.0);
            tabelle.addTableColumn(&termin);
            tabelle.setUsesAutomaticRowHeights(true);
            tabelle.setHeaderView(Some(kopfzeile));
        }
        // Sechs Spalten in der Folge von `Feld::ALLE` (Entscheidung 18 des
        // Plans); die Notizen nehmen den Rest der Breite wie die Notizspalte.
        Eintragsart::Geheimnisse => {
            for feld in Feld::ALLE {
                let (kennung, titel, breite) = geheimnisspalte(feld);
                let spalte = spalte_bauen(kennung, titel);
                if feld == Feld::Notizen {
                    spalte.setResizingMask(
                        NSTableColumnResizingOptions::AutoresizingMask
                            | NSTableColumnResizingOptions::UserResizingMask,
                    );
                    spalte.setMinWidth(GEHEIMNISBREITE / 2.0);
                } else {
                    spalte.setResizingMask(NSTableColumnResizingOptions::UserResizingMask);
                    spalte.setWidth(breite);
                    spalte.setMinWidth(breite / 2.0);
                }
                tabelle.addTableColumn(&spalte);
            }
            tabelle.setUsesAutomaticRowHeights(true);
            tabelle.setHeaderView(Some(kopfzeile));
        }
        Eintragsart::Trenntext => {
            if let Zeilen::Trenntext(sicht) = zeilen {
                trenntextspalten_angleichen(mtm, tabelle, sicht, true);
            }
            tabelle.setUsesAutomaticRowHeights(false);
            tabelle.setHeaderView(Some(kopfzeile));
        }
    }
    // Die Tabelle einer CSV ist so breit wie ihre Spalten; die letzte auf die
    // Rolle zu strecken, nahme ihr den Sinn des waagerechten Rollens.
    if !waagerecht {
        tabelle.sizeLastColumnToFit();
    }
    rolle.tile();
}

/// Schaltet das Feld einer Zelle der Tabelle einer CSV bearbeitbar oder
/// zurueck (Modulkopf, „Die Tabelle einer CSV- oder TSV-Datei"): **die eine
/// Stelle dieses Schalters**, damit der Beginn einer Bearbeitung, ihr Ende
/// und das Wiederverwenden einer Zelle ihn gleich setzen.
fn trenntextfeld_schalten(feld: &NSTextField, bearbeitbar: bool) {
    feld.setEditable(bearbeitbar);
}

/// Wie die Tabelle dieser Art ihre Auswahl hervorhebt: die einer CSV gar
/// nicht, weil sie ihre Rechteckauswahl selbst zeichnet (E10), jede andere
/// mit der Zeile wie vorher.
#[must_use]
pub fn hervorhebung(art: Eintragsart) -> NSTableViewSelectionHighlightStyle {
    match art {
        Eintragsart::Trenntext => NSTableViewSelectionHighlightStyle::None,
        Eintragsart::Aufgaben
        | Eintragsart::Notizen
        | Eintragsart::Termine
        | Eintragsart::Geheimnisse => NSTableViewSelectionHighlightStyle::Regular,
    }
}

/// Faerbt das Feld einer Zelle der Tabelle einer CSV als gewaehlt oder nicht
/// (E10): **die eine Stelle dieser Faerbung**, fuer gebaute, wiederverwendete
/// und schon stehende Zellen.
///
/// Der Hintergrund ist die Farbe ausgewaehlten Textes und nicht die einer
/// gewaehlten Zeile: die Schrift der Zelle bleibt dunkel, und lesbar bleibt
/// sie in hell wie in dunkel, weil das System die Farbe je Erscheinung
/// stellt.
fn auswahl_faerben(feld: &NSTextField, gewaehlt: bool) {
    feld.setDrawsBackground(gewaehlt);
    if gewaehlt {
        feld.setBackgroundColor(Some(&NSColor::selectedTextBackgroundColor()));
    }
}

/// Was eine Taste in der Tabelle einer CSV ohne laufende Zelle tut (E10, E16).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tabellentaste {
    /// Ein Pfeil bewegt die Auswahl, mit der Umschalttaste ihr Ende.
    Pfeil(Pfeil, bool),
    /// `return` oeffnet die obere linke Zelle der Auswahl.
    Oeffnen,
}

/// Der Pfeil nach links, nach dem Tastencode der einen Tastentabelle.
const TASTE_LINKS: u16 = krk_core::tasten::code_von_pflicht("left");
/// Der Pfeil nach rechts.
const TASTE_RECHTS: u16 = krk_core::tasten::code_von_pflicht("right");
/// Der Pfeil nach oben.
const TASTE_HOCH: u16 = krk_core::tasten::code_von_pflicht("up");
/// Der Pfeil nach unten.
const TASTE_RUNTER: u16 = krk_core::tasten::code_von_pflicht("down");
/// Die Eingabetaste.
const TASTE_RETURN: u16 = krk_core::tasten::code_von_pflicht("return");

/// Welche Tabellentaste ein Tastendruck ist: ein Pfeil oder `return` ohne
/// Befehls-, Wahl- und Steuerungstaste; jeder andere Druck ist keine.
///
/// **Rein und ohne Fenster pruefbar.** Gefragt wird sie vom Faenger des
/// Ereignisabgriffs, und allein dann, wenn die Tabelle einer CSV der
/// Ersthelfer ist (`Anwendungsdelegierter::tabellentaste_fangen`): die
/// Tabelle selbst ueberschreibt `keyDown:` nicht, denn einen Tastendruck
/// faengt in diesem Baum allein der Abgriff, und `NSTableView` stellt die
/// Pfeile nicht als `moveLeft:` und seine Geschwister zu (gemessen am 261018
/// unter `libtest`: ein `keyDown:` mit dem Pfeil nach unten erreichte eine
/// ueberschriebene `moveDown:` nicht). Die Umschalttaste dehnt beim Pfeil die
/// Auswahl aus und laesst `return` unberuehrt.
#[must_use]
pub fn tabellentaste(druck: Tastendruck) -> Option<Tabellentaste> {
    if druck.maske.enthaelt(ModMaske::BEFEHL)
        || druck.maske.enthaelt(ModMaske::WAHL)
        || druck.maske.enthaelt(ModMaske::STEUERUNG)
    {
        return None;
    }
    let umschalt = druck.maske.enthaelt(ModMaske::UMSCHALT);
    let pfeil = match druck.code {
        TASTE_LINKS => Pfeil::Links,
        TASTE_RECHTS => Pfeil::Rechts,
        TASTE_HOCH => Pfeil::Hoch,
        TASTE_RUNTER => Pfeil::Runter,
        TASTE_RETURN if !umschalt => return Some(Tabellentaste::Oeffnen),
        _ => return None,
    };
    Some(Tabellentaste::Pfeil(pfeil, umschalt))
}

/// Ob die Tabelle dieser Art waagerecht rollt: allein die einer CSV (E8).
#[must_use]
pub fn rollt_waagerecht(art: Eintragsart) -> bool {
    match art {
        Eintragsart::Trenntext => true,
        Eintragsart::Aufgaben
        | Eintragsart::Notizen
        | Eintragsart::Termine
        | Eintragsart::Geheimnisse => false,
    }
}

/// Bringt die Spalten der Tabelle einer CSV auf die Sicht: die Nummernspalte
/// und je Feld eine Spalte, mit dem Kopf aus [`kopftitel`] (E6, E8).
///
/// **Gezogene Breiten bleiben stehen.** Eine Spalte bekommt ihre
/// Anfangsbreite allein, wenn sie entsteht, aus den ersten
/// [`BREITENSAETZE`] Saetzen; `neu` heisst, die Tabelle hat eben alle
/// Spalten verloren, und dann entstehen alle. Ein Kopf wird nur neu gesetzt,
/// wenn sein Titel sich geaendert hat, damit eine Zellaenderung in einer
/// breiten Datei nicht jeden Kopf anfasst.
fn trenntextspalten_angleichen(
    mtm: MainThreadMarker,
    tabelle: &NSTableView,
    sicht: &Trenntextsicht,
    neu: bool,
) {
    let punkte = NSFont::systemFontSize();
    let spalten = sicht.index().spaltenzahl();
    let soll = NSInteger::try_from(spalten + NUMMERNSPALTE + 1).unwrap_or(NSInteger::MAX);
    if neu || tabelle.numberOfColumns() == 0 {
        let nummer = NSTableColumn::initWithIdentifier(
            NSTableColumn::alloc(mtm),
            ns_string!("trenntextnummer"),
        );
        nummer.setTitle(ns_string!(""));
        nummer.setResizingMask(NSTableColumnResizingOptions::UserResizingMask);
        tabelle.addTableColumn(&nummer);
    }
    // Die Nummernspalte waechst mit der Zahl der Stellen der letzten Zeile.
    if let Some(nummer) = tabelle.tableColumns().iter().next() {
        let stellen = sicht.index().zeilenzahl().max(1).to_string().len();
        let breite = spaltenbreite(stellen, punkte).min(BREITENGRENZEN.0 + 24.0);
        if nummer.width() < breite {
            nummer.setWidth(breite);
        }
    }
    let alte = tabelle.tableColumns().iter().count();
    for _ in usize::try_from(soll).unwrap_or(usize::MAX)..alte {
        if let Some(letzte) = tabelle.tableColumns().iter().last() {
            tabelle.removeTableColumn(&letzte);
        }
    }
    let vorhandene = tabelle.tableColumns().iter().count().saturating_sub(1);
    if vorhandene < spalten {
        let breiten = zeichenzahlen(sicht, BREITENSAETZE);
        for feld in vorhandene..spalten {
            let kennung = NSString::from_str(&format!("feld{feld}"));
            let spalte = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), &kennung);
            spalte.setResizingMask(NSTableColumnResizingOptions::UserResizingMask);
            spalte.setWidth(spaltenbreite(
                breiten.get(feld).copied().unwrap_or(0),
                punkte,
            ));
            tabelle.addTableColumn(&spalte);
        }
    }
    let erste = sicht.werte(0);
    for (stelle, spalte) in tabelle.tableColumns().iter().enumerate().skip(1) {
        let feld = stelle - (NUMMERNSPALTE + 1);
        let titel = kopftitel(feld, erste.get(feld).map(String::as_str));
        if spalte.title().to_string() != titel {
            spalte.setTitle(&NSString::from_str(&titel));
        }
    }
}

/// Die groesste Zahl gezeigter Zeichen je Feld in den ersten `saetze` Saetzen,
/// fuer die Anfangsbreite der Spalten (E8).
#[must_use]
fn zeichenzahlen(sicht: &Trenntextsicht, saetze: usize) -> Vec<usize> {
    let mut zahlen = vec![0; sicht.index().spaltenzahl()];
    for satz in 0..sicht.index().zeilenzahl().min(saetze) {
        for (feld, wert) in sicht.werte(satz).iter().enumerate() {
            if let Some(zahl) = zahlen.get_mut(feld) {
                *zahl = (*zahl).max(zellanzeige(wert).chars().count());
            }
        }
    }
    zahlen
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

    /// Die Rechteckauswahl der Tabelle einer CSV ist nach jeder Folge aus
    /// Klick, Umschaltklick, Pfeil, Umschaltpfeil, Zeile, Spalte, Ziehen und
    /// Beschneiden ein Rechteck in der Tabelle mit mindestens einer Zelle
    /// (E10); gefahren an den Raendern und mit einem eigenen Zufallsgenerator
    /// mit festem Startwert, damit ein Fehler wiederholbar bleibt.
    #[test]
    fn die_zellauswahl_ist_immer_ein_rechteck_mit_mindestens_einer_zelle() {
        let mut zustand: u64 = 0x5eed_2610_1800_0004;
        let mut zufall = |grenze: usize| -> usize {
            zustand ^= zustand << 13;
            zustand ^= zustand >> 7;
            zustand ^= zustand << 17;
            usize::try_from(zustand % u64::try_from(grenze.max(1)).unwrap_or(1)).unwrap_or(0)
        };
        let pfeile = [Pfeil::Hoch, Pfeil::Runter, Pfeil::Links, Pfeil::Rechts];
        for (zeilen, spalten) in [(1, 1), (1, 5), (5, 1), (4, 3), (30, 12)] {
            let mass = Tabellenmass { zeilen, spalten };
            let mut auswahl = Zellauswahl::zelle(0, 0, mass);
            let mut ziehart = Ziehart::Keine;
            for schritt in 0..2000 {
                // Auch Orte jenseits des Randes, damit das Beschneiden greift.
                let zeile = zufall(zeilen + 3);
                let feld = zufall(spalten + 3);
                let umschalt = zufall(2) == 1;
                auswahl = match zufall(7) {
                    0 => {
                        let (neu, art) =
                            klick_waehlen(auswahl, Klickort::Zelle { zeile, feld }, umschalt, mass);
                        ziehart = art;
                        neu
                    }
                    1 => klick_waehlen(auswahl, Klickort::Nummer { zeile }, umschalt, mass).0,
                    2 => klick_waehlen(auswahl, Klickort::Kopf { feld }, umschalt, mass).0,
                    3 | 4 => auswahl
                        .and_then(|auswahl| auswahl.bewegt(pfeile[zufall(4)], umschalt, mass)),
                    5 => ziehen_waehlen(auswahl, ziehart, zeile, feld, mass),
                    _ => auswahl.and_then(|auswahl| auswahl.beschnitten(mass)),
                };
                let auswahl =
                    auswahl.unwrap_or_else(|| panic!("ohne Auswahl nach Schritt {schritt}"));
                let bereich = auswahl.bereich();
                assert!(
                    !bereich.zeilen.is_empty() && !bereich.spalten.is_empty(),
                    "leer nach Schritt {schritt}: {bereich:?}"
                );
                assert!(
                    bereich.zeilen.end <= zeilen && bereich.spalten.end <= spalten,
                    "ausserhalb der Tabelle {zeilen}x{spalten} nach Schritt {schritt}: {bereich:?}"
                );
                assert!(auswahl.enthaelt(auswahl.anker().0, auswahl.anker().1));
                assert!(auswahl.enthaelt(auswahl.ende().0, auswahl.ende().1));
            }
        }

        // An den Raendern ausgeschrieben.
        let mass = Tabellenmass {
            zeilen: 3,
            spalten: 2,
        };
        let a1 = Zellauswahl::zelle(0, 0, mass).expect("A1");
        assert_eq!(
            a1.bewegt(Pfeil::Hoch, false, mass),
            Some(a1),
            "oben bleibt oben"
        );
        assert_eq!(
            a1.bewegt(Pfeil::Links, true, mass),
            Some(a1),
            "links bleibt links"
        );
        let ecke = Zellauswahl::zelle(9, 9, mass).expect("beschnitten");
        assert_eq!(ecke.bereich(), Zellbereich::zelle(2, 1));
        assert_eq!(
            ecke.bewegt(Pfeil::Rechts, false, mass)
                .map(Zellauswahl::bereich),
            Some(Zellbereich::zelle(2, 1))
        );
        let gezogen = a1
            .bewegt(Pfeil::Runter, true, mass)
            .and_then(|auswahl| auswahl.bewegt(Pfeil::Rechts, true, mass))
            .expect("ausgedehnt");
        assert_eq!(
            gezogen.bereich(),
            Zellbereich {
                zeilen: 0..2,
                spalten: 0..2
            }
        );
        assert_eq!(
            gezogen
                .bewegt(Pfeil::Hoch, false, mass)
                .map(Zellauswahl::bereich),
            Some(Zellbereich::zelle(0, 1)),
            "ohne Umschalttaste eine Zelle neben dem Ende"
        );
        assert_eq!(
            gezogen
                .beschnitten(Tabellenmass {
                    zeilen: 1,
                    spalten: 1
                })
                .map(Zellauswahl::bereich),
            Some(Zellbereich::zelle(0, 0)),
            "nach einem Umbau auf das neue Mass"
        );
        assert_eq!(
            gezogen.beschnitten(Tabellenmass {
                zeilen: 0,
                spalten: 1
            }),
            None,
            "ohne Zeilen keine Auswahl"
        );
        assert_eq!(
            Zellauswahl::aus_bereich(
                &Zellbereich {
                    zeilen: 1..1,
                    spalten: 1..1
                },
                mass
            )
            .map(Zellauswahl::bereich),
            Some(Zellbereich::zelle(1, 1)),
            "ein leerer Bereich meint seine erste Zelle"
        );
    }

    /// `cmd+v` in der Tabelle einer CSV ohne Text in der Ablage reicht einen
    /// leeren Text an den Editor weiter, statt stumm zurueckzukehren, und der
    /// antwortet darauf mit `Trenntextantwort::AblageOhneText`
    /// (`261019-0037-cmd-v-in-der-csv-tabelle-ohne-text-in-der-ablage-bleibt-stumm.md`).
    /// Den Rechenweg haelt `cmd_v_setzt_einen_tsv_block_ab_der_zelle_und_ein_cmd_z_nimmt_ihn_ganz_zurueck` in
    /// `appkit/editor.rs`. **Was sie nicht sieht:** ob AppKit `paste:` bei
    /// einer Ablage ohne Text ueberhaupt an die Tabelle schickt; das zeigt allein
    /// das laufende Buendel.
    #[test]
    fn das_einfuegen_ohne_text_in_der_ablage_erreicht_den_editor() {
        use super::super::anwendung::quelltextproben::datei;
        let quelle = datei("krk-ui/src/appkit/eintragsansicht.rs");
        let (_, ab) = quelle
            .split_once(concat!("fn einfu", "egen(&self, absender"))
            .expect("die Methode paste: steht in der Datei");
        let (rumpf, _) = ab
            .split_once(concat!("fn alles_", "waehlen("))
            .expect("die Methode selectAll: steht dahinter");
        let code: String = rumpf
            .lines()
            .filter(|zeile| !zeile.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(code.contains(concat!("nur_text_lesen().", "unwrap_or_default()")));
        assert!(code.contains(concat!("bereich_", "einfuegen(&text)")));
        assert_eq!(
            code.matches(concat!("re", "turn;")).count(),
            1,
            "allein die fremde Art kehrt vor dem Editor zurueck"
        );
    }
}
