//! Das Dateifenster: Tableiste und `NSTableView` in einer `NSScrollView`,
//! angebunden an das Tabmodell aus [`crate::tabs`].
//!
//! ```text
//! ┌──────────────────────────────┐
//! │ Tableiste (appkit::tableiste)│  ein Abschnitt je Tab
//! ├──────────────────────────────┤
//! │ NSScrollView                 │  der Inhalt des sichtbaren Tabs
//! │   NSTableView, vier Spalten  │
//! └──────────────────────────────┘
//! ```
//!
//! **Die Statuszeile stand bis zur Runde 6 als dritte Ansicht am Fuss und
//! steht seither nicht mehr hier.** Es gibt eine Zeile ueber die volle
//! Fensterbreite, gehalten vom Anwendungsdelegierten; dieses Dateifenster
//! haelt weiter seine vier Meldungsfelder mit ihren je einer Loeschregel,
//! reicht sie ueber [`DateifensterQuelle::meldungsquellen`] heraus und meldet
//! ueber [`QuelleIvars::meldungswechsel`], dass sich etwas geaendert hat. Die
//! Auswahl unter den zwoelf Bewerbern trifft [`super::statuszeile::zeile`].
//!
//! Zwei Objective-C-Klassen teilen sich die Arbeit, weil AppKit sie an zwei
//! Protokollen entgegennimmt. [`DateifensterQuelle`] ist die Datenquelle: sie
//! haelt das Tabmodell, startet Lesevorgaenge und meldet die Zeilenzahl.
//! [`DateifensterDelegierter`] ist der Delegierte: er baut die Zellen und
//! beschriftet sie. Der Delegierte haelt die Quelle, nicht umgekehrt, denn er
//! liest aus ihr; die Gegenrichtung gibt es nicht und damit auch keinen Zyklus.
//!
//! Eine dritte Klasse steht daneben und teilt sich diese Arbeit nicht:
//! [`Namensfeld`] ist die Zelle der Namensspalte selbst. Es gibt sie, weil ein
//! Ordner in dieser Spalte einen Schraegstrich hinter dem Namen traegt und
//! dasselbe Feld zugleich der Editor des Umbenennens ist; ihr Kopf sagt, warum
//! AppKit den Beginn einer Bearbeitung nirgends sonst abfangen laesst.
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
//! Modell, zu welchem Lauf es gehoert. Eine Prueferei je
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
//!
//! **Den Freigabeeintrag baut diese Datei nicht, ihre drei eigenen schon.**
//! Die Tabelle traegt seit C1 der Runde 6 ein `NSMenu`, dessen Delegierter die
//! Quelle ist; sie beantwortet in `menuNeedsUpdate:`, welche Eintraege
//! betroffen sind, haengt seit der Runde 17 die drei eigenen Befehle aus
//! [`Kontextbefehl`] an und laesst danach
//! [`super::teilen::eintrag_anfuegen`] den Freigabeeintrag setzen. Ein zweiter
//! Bauer **jenes** Eintrags waere hier die Wiederholung, die jener Kopf
//! ausschliesst; die drei eigenen kann er nicht bauen, denn er kennt keine der
//! Flaechen und keinen Befehl von KRK.
//!
//! **Die Reihenfolge im Rumpf traegt die Form des Menues und ist keine
//! Geschmacksfrage.** [`super::teilen::eintrag_anfuegen`] fuegt **vorn** ein
//! (`insertItem_atIndex(…, 0)`) und setzt seinen Trenner nur, wenn schon etwas
//! dasteht. Erst die drei eigenen anhaengen, dann jenen rufen, ergibt damit
//! von selbst „Teilen, Trenner, Zip, Unzip, Im Finder oeffnen"; die umgekehrte
//! Folge ergaebe ein Menue ohne Trenner, in dem der Freigabeeintrag zwischen
//! den eigenen stuende.
//!
//! **Was in den drei Eintraegen steht und worauf sie wirken, entscheidet
//! [`crate::kommandos::kontextmenue`] ohne AppKit.** Titel und Marke kommen von
//! dort, gebaut wird ueber [`Kontextbefehl::ALLE`], und die drei teilen sich
//! **einen** Selektor `kontextbefehl:`; unterschieden werden sie allein an
//! ihrer Marke. Das ist die Sperre gegen den Menueeintrag, der dasteht und
//! nichts tut: die Marke wird ueber [`Kontextbefehl::von_menuemarke`]
//! zurueckgelesen, und die Ausfuehrung beim Anwendungsdelegierten verzweigt
//! ueber den Wert vollstaendig und ohne Auffangzweig. Ein vierter Wert haelt
//! damit den Bau an, statt still nichts zu tun. Der Freigabeeintrag gehoert
//! nicht in jene Aufzaehlung: er traegt Ziel und Handlung des Systems, und KRK
//! fuehrt ihn nicht aus.
//!
//! **Der Rechtsklick rueckt dabei die Auswahl auf die angeklickte Zeile, es
//! sei denn, sie ist markiert** (Nutzerentscheid vom 260812-1200,
//! `decisions/260812-1145_*_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md`
//! der Runde 6). Die Zeile liefert `clickedRow`, die Entscheidung
//! [`crate::kommandos::operationen::rechtsklick_zielzeile`] ohne Fenster, und
//! gesetzt wird sie ueber `zeile_setzen` wie jede Auswahl der Tastatur.
//! **Worauf ein Befehl danach wirkt, sagt weiterhin allein
//! [`crate::kommandos::operationen::betroffene`]**; die Auswahl aendert sich
//! vor ihr, nicht sie selbst.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! Aus AppKit spricht diese Datei `NSTableView`, `NSTableColumn`,
//! `NSScrollView`, `NSClipView` (ueber `contentView`), `NSTextField`, `NSView`
//! und `NSControl` (die Herkunft von `setTarget:` und `setAction:`) an, dazu
//! `NSColor` und `NSFont`; aus Foundation `NSObject`, `NSString`, `NSDate`,
//! `NSDateFormatter`, `NSIndexSet`, `NSNotification`, `NSRunLoop`, `NSTimer`
//! und `NSByteCountFormatter`, seit C1 der Runde 6 dazu `NSMenu` und die
//! Eigenschaft `menu` von `NSResponder` (`NSResponder.h:111`). **Alle stehen
//! seit macOS 10.0 zur Verfuegung**, `NSByteCountFormatter` als einzige
//! Ausnahme seit 10.8 (`NSByteCountFormatter.h:38`). Dasselbe gilt fuer die
//! fuenf angenommenen Protokolle `NSObjectProtocol`, `NSTableViewDataSource`,
//! `NSTableViewDelegate`, `NSControlTextEditingDelegate` und `NSMenuDelegate`
//! (`NSTableView.h:580` und `:737`, `NSControl.h:97`, `NSMenu.h:269`), und
//! ebenso fuer `menuNeedsUpdate:` (`NSMenu.h:271`) und `NSMenu`s Setzer
//! `delegate` (`NSMenu.h:156`). `NSWindow` kommt allein
//! aus `NSView::window` heraus und geht unangetastet an [`super::blaetter`];
//! dieses Modul ruft nichts daran auf. Die reinen Werttypen `NSPoint`,
//! `NSRect`, `NSSize`, `NSInteger` und `NSTimeInterval` stellen die Frage
//! nicht. Das Buendel zielt auf 15.0 (`.cargo/config.toml`).
//!
//! **Einzelne Beruehrungen sind juenger als ihre Klasse, und keine von ihnen
//! liegt ueber dem Zielsystem**; eine Verfuegbarkeitspruefung zur Laufzeit
//! braucht deshalb keine:
//!
//! - 10.5: die Modenkonstante `NSRunLoopCommonModes` (`NSRunLoop.h:14`) und
//!   `NSTableColumn`s Eigenschaft `hidden` (`NSTableColumn.h:80`), ueber die
//!   die Bereichsleisten-Runde eine Spalte verbirgt.
//! - 10.6: `reloadDataForRowIndexes:columnIndexes:` (`NSTableView.h:266`) und
//!   `NSMenu`s `removeAllItems` (`NSMenu.h:112`).
//! - 10.7: `rowForView:` und `makeViewWithIdentifier:owner:`
//!   (`NSTableView.h:477` und `:482`), die Delegiertenmethode
//!   `tableView:viewForTableColumn:row:` (`:593`) und das Protokoll
//!   `NSUserInterfaceItemIdentification`
//!   (`NSUserInterfaceItemIdentification.h:17`), aus dem die Kennung der
//!   Spalte gelesen und die der Zelle gesetzt wird.
//! - 10.10: `labelColor`, `secondaryLabelColor` und `systemOrangeColor`
//!   (`NSColor.h:201`, `:202` und `:253`). Die zweite ist seit der Runde 11
//!   die gedaempfte Farbe der Zeile, die allein wegen ihres Inhalts steht;
//!   die Leiste faerbt eine Ueberschrift schon damit.
//! - 10.11: `monospacedDigitSystemFontOfSize:weight:` samt
//!   `NSFontWeightRegular` und `NSFontWeightBold` — die drei Stellen im Kopf
//!   des Systems nennt der Block bei der Schriftwahl weiter unten — sowie
//!   `maximumNumberOfLines` (`NSTextField.h:49`).
//! - 10.12: `labelWithString:` (`NSTextField.h:93`).
//! - 10.13: `usesAutomaticRowHeights` (`NSTableView.h:574`).
//! - 11.0: `style` und die Aufzaehlung `NSTableViewStyle`
//!   (`NSTableView.h:377` und `:77-96`). **Die juengste Beruehrung dieser
//!   Datei**, und damit vier Hauptfassungen unter der Untergrenze.
//!
//! **Die neun Beruehrungen des Abwurfs stehen seit 10.0** (C4 bis C7 der
//! Runde 13), jede am SDK nachgelesen und keine mit einem `API_AVAILABLE` im
//! Kopf: `registerForDraggedTypes:` (`NSView.h:488`), das Protokoll
//! `NSDraggingInfo` (`NSDragging.h:69`) mit `draggingPasteboard`
//! (`NSDragging.h:79`) und `draggingSequenceNumber` (`NSDragging.h:81`), die
//! Aufzaehlung `NSDragOperation` (`NSDragging.h:25`),
//! `NSTableViewDropOperation` (`NSTableView.h:25`),
//! `setDropRow:dropOperation:` (`NSTableView.h:319`) und die beiden
//! Protokollmethoden `tableView:validateDrop:proposedRow:proposedDropOperation:`
//! und `tableView:acceptDrop:row:dropOperation:` (`NSTableView.h:783` und
//! `:787`). Jede Zeilenangabe ist am 260818 in
//! `$(xcrun --show-sdk-path)/System/Library/Frameworks/AppKit.framework/Headers/`
//! nachgelesen, die neunte am 260819. Die uebrigen Beruehrungen jener Runde
//! stehen im Kopf von
//! [`super::abwurf`], das sie geschlossen fuehrt; hier stehen die, die diese
//! Datei selbst anspricht.
//!
//! **Die fuenf Beruehrungen des Kontextmenues aus der Runde 17 stehen seit
//! 10.0**, jede am SDK nachgelesen und keine mit einem `API_AVAILABLE` im Kopf:
//! die Klasse `NSMenuItem` selbst (`NSMenuItem.h:23`), ihre Eigenschaft `tag`
//! in beiden Richtungen (`NSMenuItem.h:96` — Lesen und Setzen stehen in
//! derselben Deklaration), ihre schwach gehaltene Eigenschaft `target`
//! (`NSMenuItem.h:93`) und `NSMenu`s `addItem:` (`NSMenu.h:92`). Die
//! Zeilenangaben sind am 260825 in
//! `$(xcrun --show-sdk-path)/System/Library/Frameworks/AppKit.framework/Headers/`
//! nachgelesen.
//!
//! **Der Erzeuger `initWithTitle:action:keyEquivalent:` (`NSMenuItem.h:38`,
//! ebenfalls ohne Angabe und damit seit 10.0) steht bewusst nicht in dieser
//! Liste**: diese Datei ruft ihn nicht. Sie geht ueber
//! [`super::menue::ohne_kuerzel`], weil C2.10 der Runde 7 genau eine Stelle im
//! Baum zusagt, die ein `NSMenuItem` anlegt. `setAction:` (`NSMenuItem.h:94`)
//! braucht sie aus demselben Grund nicht: der Selektor geht durch jene Huelle
//! in den Erzeuger.
//!
//! **`clickedRow` steht seit 10.0** (`NSTableView.h:276`, am SDK gelesen: die
//! Eigenschaft traegt kein `API_AVAILABLE`). Sie hat seit dem 260812 zwei
//! Abnehmer statt einen, den Doppelklick aus C3 der Runde 4 und die Auswahl
//! vor dem Rechtsklick aus C1 der Runde 6.
//!
//! **Die Zelle der Namensspalte ist seit dem Ordnerzeichen eine eigene
//! Unterklasse von `NSTextField`** ([`Namensfeld`]). Sie ueberschreibt
//! `becomeFirstResponder` von `NSResponder` (`NSResponder.h:105`),
//! `abortEditing` von `NSControl` (`NSControl.h:89`) und seit dem Aufschub der
//! Auffrischung `textDidEndEditing:` von `NSTextField` selbst
//! (`NSTextField.h:37`) — letzteres traegt seit dem 260816 zusaetzlich die
//! Wiederherstellung der Anzeigeform an jedem Ende ohne Umbenennung —, und sie
//! liest `target` von `NSControl`
//! (`NSControl.h:24`); alle vier stehen seit 10.0 — keine der drei
//! Deklarationen traegt im Kopf des Systems ein `API_AVAILABLE`. Angelegt
//! wird sie ueber `labelWithString:`, also die 10.12 aus der Liste darunter.
//!
//! Alle uebrigen Setzer und Abfragen der genannten Klassen tragen im Kopf des
//! Systems keine Angabe und stehen damit seit 10.0; der Block beim Doppelklick
//! weiter unten fuehrt das fuer `setTarget:`, `setDoubleAction:` und
//! `clickedRow` einzeln aus. Dasselbe gilt fuer die drei Abfragen, an denen die
//! Spaltenverteilung haengt: `rectOfColumn:` (`NSTableView.h:393`),
//! `columnWithIdentifier:` (`:238`) und `NSTableColumn`s Breitenpaar `width`
//! und `minWidth` (`NSTableColumn.h:42` und `:48`). `objc2` fuehrt keine Verfuegbarkeitsangaben mit
//! sich, und der Uebersetzer haelt die Untergrenze nicht; die Nennung hier ist
//! die Gegenmassnahme.

use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{ClassType, DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSColor, NSControlTextEditingDelegate, NSDragOperation,
    NSDraggingInfo, NSFont, NSFontWeightBold, NSFontWeightRegular, NSMenu, NSMenuDelegate,
    NSMenuItem, NSScrollView, NSTableColumn, NSTableView, NSTableViewColumnAutoresizingStyle,
    NSTableViewDataSource, NSTableViewDelegate, NSTableViewDropOperation, NSTableViewStyle,
    NSTextAlignment, NSTextField, NSUserInterfaceItemIdentification, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSByteCountFormatter, NSByteCountFormatterCountStyle, NSDate,
    NSDateFormatter, NSDateFormatterStyle, NSIndexSet, NSInteger, NSNotification, NSObject,
    NSObjectProtocol, NSPoint, NSRect, NSRunLoop, NSRunLoopCommonModes, NSSize, NSString,
    NSTimeInterval, NSTimer, ns_string,
};

use krk_core::ablage::Dateifenster as Fensterzustand;
use krk_core::tasten::Kommando;
use krk_core::verzeichnis::filter::traegt_ein_dateiname;
use krk_core::verzeichnis::verweisziel::{self, Verweisziel};
use krk_core::verzeichnis::{
    Eintrag, Markierungsstand, Ordnermodell, Schluessel, Sortierung, Typ, aufwaerts,
};
use krk_core::zwischenablage::{self, Ziel};

use crate::kommandos::abwurfregel::{
    self, Abwurfgrund, Abwurflage, Abwurfmarke, Abwurfurteil, Abwurfvorgang,
};
use crate::kommandos::auswahl::{self, markieren_und_weiter};
use crate::kommandos::kontextmenue::{self, Entpackbefund, Kontextbefehl};
use crate::kommandos::navigation::{Bewegung, ersatzzeile, zielzeile};
use crate::kommandos::operationen::{self, Umbenennungswunsch};
use crate::kommandos::pfadeingabe::{self, Ergebnis};
use crate::spalten::Spalte;
use crate::tabs::{Auswahlversuch, Tabliste};

use super::abwurf;
use super::blaetter;
use super::menue;
use super::standardprogramm;
use super::statuszeile::{self, Filterstand, Quellen};
use super::tableiste::Tableiste;
use super::teilen;

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

/// Was AppKit ueber eine [`Spalte`] wissen muss: Kennung, Ueberschrift,
/// Breiten, Ausrichtung und der Weg von einer Kennung zurueck zur Spalte.
///
/// Die Aufzaehlung selbst steht in [`crate::spalten`] und nennt keine
/// `objc2`-Kiste; hier stehen die fuenf Funktionen, die eine nennen. Dasselbe
/// Muster tragen `aufteilung::sichtbar_im` und `aufteilung::rahmenfarbe` ueber
/// [`crate::fenstermodell::Bereich`]. Freie Funktionen und keine Methoden:
/// eine Methode zoege `NSString`, `NSTextAlignment` und damit AppKit an eine
/// Aufzaehlung, die seit der Bereichsleisten-Runde zwei Leser hat und deren
/// zweiter die Tabelle nicht braucht.
///
/// Die Kennung dient zugleich als Kennung der wiederverwendeten Zellenansicht:
/// eine Ansicht, die aus der Namensspalte zurueckkommt, landet nur wieder in
/// der Namensspalte und behaelt damit ihre Ausrichtung.
fn kennung(spalte: Spalte) -> &'static NSString {
    match spalte {
        Spalte::Name => ns_string!("name"),
        Spalte::Groesse => ns_string!("groesse"),
        Spalte::Geaendert => ns_string!("geaendert"),
        Spalte::Typ => ns_string!("typ"),
    }
}

/// Die Ueberschrift der Spalte ueber der Tabelle.
///
/// **Abgeleitet aus [`Spalte::beschriftung`], wo beide denselben Text
/// tragen**, damit eine Umbenennung nicht an zwei Stellen zu erledigen ist.
/// [`Spalte::Geaendert`] weicht ab, und das ist gewollt: ueber der Spalte
/// steht "Änderungsdatum", der Schalter der Bereichsleiste heisst "Datum". Die
/// Ueberschrift hat die Breite dafuer und die Zelle darunter zeigt neben dem
/// Datum die Uhrzeit; die Leiste ist 18 Punkte hoch, traegt neun Schalter
/// nebeneinander, und "Datum" ist der Name, den der Nutzer dem Schalter
/// gegeben hat.
///
/// Der Rueckgabewert ist deshalb kein `&'static NSString` aus `ns_string!`:
/// dieses Makro verlangt ein Literal an Ort und Stelle, und damit stuenden die
/// drei uebernommenen Texte ein zweites Mal da. Gebaut wird die Zeichenkette
/// achtmal, beim Aufbau der vier Spalten der beiden Dateifenster.
fn titel(spalte: Spalte) -> Retained<NSString> {
    let text = match spalte {
        Spalte::Geaendert => "Änderungsdatum",
        Spalte::Name | Spalte::Groesse | Spalte::Typ => spalte.beschriftung(),
    };
    NSString::from_str(text)
}

/// Anfangsbreite und Mindestbreite in Punkten.
fn breiten(spalte: Spalte) -> (f64, f64) {
    match spalte {
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
fn ausrichtung(spalte: Spalte) -> NSTextAlignment {
    match spalte {
        Spalte::Groesse => NSTextAlignment::Right,
        Spalte::Name | Spalte::Geaendert | Spalte::Typ => NSTextAlignment::Left,
    }
}

/// Die Spalte zu einer Kennung, falls es sie gibt.
///
/// Der Parameter heisst `gesucht` und nicht `kennung`: der Name der Funktion
/// darueber waere sonst in diesem Rumpf verdeckt.
fn aus_kennung(gesucht: &NSString) -> Option<Spalte> {
    Spalte::ALLE
        .into_iter()
        .find(|spalte| kennung(*spalte) == gesucht)
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

/// Das Zeichen, das ein Ordner in der Spalte `Name` hinter seinem Namen
/// traegt.
///
/// **Es ist Anzeige und nie Name.** Sortierung, Filter, Zwischenablage,
/// Vorschau und jede Dateioperation lesen weiter `eintrag.name`; keine von
/// ihnen sieht es. Der Nutzerentscheid vom 260815-2058 steht in
/// `shared/decisions/260815-2056_*_woran-erkennt-der-nutzer-in-der-dateiliste-einen-ordner.md`.
///
/// Der Schraegstrich und kein anderes Zeichen: er kann in keinem Namen
/// vorkommen, den ein Dateisystem hergibt, und damit ist [`ohne_ordnerzeichen`]
/// eindeutig umkehrbar.
const ORDNERZEICHEN: char = '/';

/// Die Anzeigeform eines Eintrags in der Spalte `Name`.
///
/// **Die Bedingung ist [`Eintrag::ist_ordner`]**, also `Typ::Ordner`, und damit
/// dieselbe wie beim `--` der Spalte `Groesse` und bei der Gruppe der
/// Sortierung. **Diese Gleichheit entscheidet die Sache**: eine Verknuepfung
/// auf einen Ordner bekommt kein Zeichen, weil sie in der Spalte `Groesse`
/// ebenfalls keine bekommt.
///
/// **Warum das Verweisziel nicht erfragt wird.** Es zu erfragen hiesse ein
/// `stat` je sichtbarer Zeile, und das stuende in der Zeichenschleife der
/// Dateiliste. Die misst **keine** der zehn Zusagen aus C8: L2, L3 und L10
/// laufen auf der kopflosen Strecke, die keine `NSTableView` baut und
/// [`DateifensterDelegierter::zellenansicht`] nie ruft
/// (`krk-bench/src/messen.rs:1199`). Ein Systemaufruf je Zeile kaeme damit als
/// Kostenstelle ohne Abnahmekriterium in den Baum, und das ist der staerkere
/// Grund, ihn zu unterlassen, nicht der schwaechere.
///
/// Bis zum 260816 stand hier, genau diese Schleife messe L3 und L10. Das war
/// falsch (Befund `shared/issues/260815-2202_*_…`); die berichtigte Begruendung
/// steht im Nutzerentscheid vom 260815-2058 in
/// `shared/decisions/260815-2056_*_woran-erkennt-der-nutzer-in-der-dateiliste-einen-ordner.md`.
#[must_use]
fn namensform(eintrag: &Eintrag) -> String {
    let mut anzeige = eintrag.name.clone();
    if eintrag.ist_ordner() {
        anzeige.push(ORDNERZEICHEN);
    }
    anzeige
}

/// Der wirkliche Name zu einer Anzeigeform, falls sie das Ordnerzeichen
/// traegt.
///
/// `None` heisst "da war keines" und nicht "der Name ist leer": der Aufrufer
/// laesst die Zeichenkette dann unberuehrt, statt eine gleiche zweite zu
/// bauen.
#[must_use]
fn ohne_ordnerzeichen(anzeige: &str) -> Option<&str> {
    anzeige.strip_suffix(ORDNERZEICHEN)
}

/// Was mit einem umbenannten Eintrag zu geschehen hat: alter Name, neuer Name.
///
/// Ein eigener Name fuer den Rueckruf, damit das Feld und sein Setzer dieselbe
/// Schreibweise tragen und keine von beiden zu lesen ist wie ein Bandwurm.
pub type Umbenennungsmelder = Box<dyn Fn(&str, &str)>;

/// Was mit einer neuen Auswahl zu geschehen hat: der vollstaendige Pfad des
/// ausgewaehlten Eintrags, `None` fuer eine aufgehobene Auswahl.
///
/// Ein eigener Name aus demselben Grund wie beim [`Umbenennungsmelder`]
/// darueber.
pub type Auswahlmelder = Box<dyn Fn(Option<PathBuf>)>;

/// Was zu geschehen hat, wenn der Nutzer dieses Dateifenster angefasst hat:
/// [`Rangmitnahme`] sagt, ob der Ersthelferrang mitkommt.
///
/// Ein eigener Name aus demselben Grund wie bei den beiden darueber, und seit
/// dem 260825 auch aus einem zweiten: ohne ihn steht `clippy::type_complexity`
/// am Feld, und `make lint` macht daraus einen Fehler.
pub type Aktivierungsmelder = Box<dyn Fn(Rangmitnahme)>;

/// Ob KRK gerade schon einen Vorgang haelt, ohne dass die Frage etwas meldet
/// (C6 der Runde 13).
///
/// Der Weg zu `Anwendungsdelegierter::vorgang_laeuft`, also zu der einen
/// Stelle, die diese Frage beantwortet. **Ohne den meldenden Mantel**, den die
/// drei Tastenwege nehmen: [`DateifensterQuelle::abwurf_pruefen`] fragt bei
/// jeder Zeigerbewegung, und eine Meldung von dort schriebe die Statuszeile
/// mehrmals je Sekunde voll.
pub type Vorgangsfrage = Box<dyn Fn() -> bool>;

/// Was mit einem angenommenen Abwurf zu geschehen hat: der Zielordner, die
/// Quellen und der Vorgang (C4 bis C6 der Runde 13).
///
/// Ein eigener Name aus demselben Grund wie beim [`Umbenennungsmelder`]
/// darueber. Ausgefuehrt wird der Abwurf beim Anwendungsdelegierten, weil die
/// Operationsmaschine dort haengt und die Quellen einer fremden Anwendung
/// gehoeren, also nicht aus der Auswahl dieses Dateifensters kommen.
pub type Abwurfmelder = Box<dyn Fn(PathBuf, Vec<PathBuf>, Abwurfvorgang)>;

/// Was mit einem angeklickten Eintrag des Kontextmenues zu geschehen ist
/// (Runde 17).
///
/// Ein eigener Name aus demselben Grund wie beim [`Umbenennungsmelder`]
/// darueber. Ausgefuehrt wird der Befehl beim Anwendungsdelegierten, weil zwei
/// der drei Wege die Operationsmaschine brauchen und der dritte die
/// Statuszeile ueber beide Dateifenster hinweg; von einer Quelle aus ist
/// keines von beiden zu erreichen.
///
/// **Der Wert und nicht die Marke.** Die Zurueckrechnung der Menuemarke steht
/// in [`Kontextbefehl::von_menuemarke`] und damit vor diesem Rueckruf; was
/// hier herauskommt, ist bereits einer der drei Befehle, und der Empfaenger
/// verzweigt darueber vollstaendig und ohne Auffangzweig.
pub type Kontextmelder = Box<dyn Fn(Kontextbefehl)>;

/// Was die Statuszeile sagt, wenn die Quelle keine Datei auf dem Datentraeger
/// liefert (C7).
///
/// **Die eine Meldung des Abwurfs.** Die vier uebrigen [`Abwurfgrund`]-Werte
/// zeigen sich allein am Zeiger; warum, steht an jener Aufzaehlung.
const KEINE_DATEI: &str = "die Quelle liefert keine Datei auf dem Datenträger";

/// Ob ein neu gefaelltes Urteil eine Meldung in die Statuszeile schreibt (C7).
///
/// `gemerkt` ist der Grund, den [`DateifensterQuelle::abwurf_pruefen`] beim
/// vorigen Durchgang gefaellt hat, `jetzt` der eben gefaellte; `None` steht in
/// beiden fuer „kein Grund", also fuer ein angenommenes Urteil. Zurueck kommt
/// der Text oder `None` fuer „nichts schreiben".
///
/// | `gemerkt` | `jetzt` | Ausgang |
/// |---|---|---|
/// | gleich `jetzt` | gleichgueltig | nichts schreiben |
/// | ungleich | `Some(KeineDatei)` | [`KEINE_DATEI`] |
/// | ungleich | jeder andere Wert | nichts schreiben |
///
/// **Die erste Zeile ist die ganze Entdopplung, und sie ist eine Zusage des
/// Spec:** die Meldung darf nicht bei jeder Zeigerbewegung neu geschrieben
/// werden, und `validateDrop:` laeuft bei jeder. Ohne sie schriebe ein
/// stehender Zeiger dieselbe Zeile mehrmals je Sekunde, und jede
/// Vorgangsanzeige darunter flackerte mit.
///
/// **Sie entscheidet nicht, ob der Abwurf durchgeht** — das tut
/// [`abwurfregel::urteil`] und niemand sonst. Sie entscheidet allein, ob eine
/// Zeile geschrieben wird, und das ist eine andere Frage: vier der fuenf
/// Gruende weisen ab, ohne etwas zu sagen.
///
/// **Steht die Meldung einmal, bleibt sie stehen**, auch nachdem der Zeiger die
/// Liste verlassen hat, und faellt mit der naechsten Befehlsantwort. Das ist
/// die Loeschregel des Rangs 1 und keine Ausnahme fuer den Abwurf; der gemerkte
/// Grund faellt an derselben Stelle mit, sonst bliebe eine zweite gleiche
/// Ziehbewegung nach einem Tastendruck stumm
/// ([`DateifensterQuelle::befehlsantwort_loeschen`]).
///
/// **An demselben `Some` haengt die Raeumung beider Dateifenster.** Der
/// Aufrufer raeumt den Rang 1 auf beiden Seiten, bevor er schreibt, und tut das
/// genau dann, wenn diese Funktion einen Text liefert — sonst liefe die
/// Raeumung bei jeder Zeigerbewegung. Warum sie ueberhaupt beide Seiten
/// braucht, steht bei
/// [`DateifensterQuelle::befehlsantwort_beidseitig_loeschen`].
#[must_use]
fn abwurfmeldung(gemerkt: Option<Abwurfgrund>, jetzt: Option<Abwurfgrund>) -> Option<&'static str> {
    if gemerkt == jetzt {
        return None;
    }
    match jetzt {
        Some(Abwurfgrund::KeineDatei) => Some(KEINE_DATEI),
        // Ein angenommenes Urteil raeumt die stehende Meldung ausdruecklich
        // nicht weg: das taete eine zweite Loeschregel neben der des Rangs 1.
        None
        | Some(
            Abwurfgrund::VorgangLaeuft
            | Abwurfgrund::NichtBeschreibbar
            | Abwurfgrund::SelberOrdner
            | Abwurfgrund::KeinAngebot,
        ) => None,
    }
}

/// Der Ordner, in dem **alle** gezogenen Eintraege liegen, falls sie alle in
/// einem liegen (C6, Lage 3 der Runde 13).
///
/// Rein und ohne AppKit, damit die Rechnung ohne Ziehsitzung zu pruefen ist;
/// gerufen wird sie von [`DateifensterQuelle::abwurfquellen`], hoechstens
/// einmal je Ziehsitzung.
///
/// **`None` heisst dreierlei, und alle drei fuehren zu demselben Ausgang:** die
/// Ablage traegt keinen Eintrag; die Eintraege kommen aus mehreren Ordnern; ein
/// Eintrag hat gar keinen uebergeordneten Ordner, ist also die Wurzel des
/// Datentraegers. In keinem der drei Faelle ist der Zielordner „der
/// Quellordner", und C6 Lage 3 greift nicht. Ein eigener Wert je Fall waere ein
/// Unterschied, den niemand liest.
///
/// **Der Vergleich mit dem Ziel steht nicht hier**, sondern beim Aufrufer, und
/// zwar aus einem Grund: das Ziel wechselt mit jeder Zeigerbewegung, dieses
/// Ergebnis nicht. Was hier steht, ist genau der Teil, der eine Ziehsitzung
/// lang gilt.
#[must_use]
fn gemeinsamer_quellordner(quellen: &[PathBuf]) -> Option<PathBuf> {
    let mut uebrige = quellen.iter();
    let erster = uebrige.next()?.parent()?;
    uebrige
        .all(|weiterer| weiterer.parent() == Some(erster))
        .then(|| erster.to_path_buf())
}

/// Welcher Ordner beim **Loslassen** das Ziel ist (C4 der Runde 13).
///
/// `benennt_eine_zeile` ist wahr, wenn AppKit eine Zeilennummer und nicht die
/// `-1` liefert; `-1` heisst „die ganze Liste" (`NSTableView.h:317`).
/// `typ_der_zeile` ist, was **jetzt** an dieser Zeile steht, und `None` heisst,
/// dass dort nichts mehr steht.
///
/// Der Rumpf ist diese Tafel, und sie steht ausgeschrieben und nicht gerechnet:
///
/// | `benennt_eine_zeile` | `typ_der_zeile` | Ausgang |
/// |---|---|---|
/// | nein | gleichgueltig | [`Abwurfziel::AngezeigterOrdner`] |
/// | ja | `Some(Typ::Ordner)` | [`Abwurfziel::Zeile`] |
/// | ja | `Some(Typ::Datei)` | [`Abwurfziel::Keines`] |
/// | ja | `Some(Typ::Verknuepfung)` | [`Abwurfziel::Keines`] |
/// | ja | `None` | [`Abwurfziel::Keines`] |
///
/// **Fuenf Arme ueber acht Kombinationen**, ohne Auffangzweig; [`Typ`] ist
/// geschlossen, also haelt der Uebersetzer die Vollstaendigkeit, und eine
/// vierte Eintragsart hielte den Bau an. Die Probe `die_tafel_des_abwurfziels_
/// geht_auf` schreibt alle acht aus.
///
/// # Warum das nicht [`abwurfregel::marke`] ist
///
/// Die beiden Tafeln sehen einander aehnlich und beantworten zwei
/// verschiedene Fragen; sichtbar wird der Unterschied an der Dateizeile.
///
/// [`abwurfregel::marke`] laeuft **waehrend** des Ziehens und entscheidet, was
/// der Zeiger zeigt. Ueber einer Dateizeile springt die Marke auf die ganze
/// Liste, und der angezeigte Ordner ist damit ein angekuendigtes und voellig
/// gueltiges Ziel — der Nutzer sieht es vor dem Loslassen.
///
/// Diese Tafel laeuft **beim** Loslassen und entscheidet, ob das, was AppKit
/// zurueckmeldet, noch das ist, was gezeigt wurde. Eine Zeilennummer kommt hier
/// nur an, wenn `abwurf_pruefen` sie ueber `setDropRow:` gesetzt hat, und
/// gesetzt hat sie es allein fuer einen Ordner. Steht dort jetzt eine Datei
/// oder gar nichts mehr, hat sich die Liste unter dem Zeiger geaendert; „der
/// angezeigte Ordner" war dann nie angekuendigt, und ihn stillschweigend zu
/// nehmen hiesse, in einen anderen Ordner zu schreiben als den, auf den der
/// Nutzer gezeigt hat.
///
/// Eine gemeinsame Tafel muesste den Fall der Dateizeile in beide Richtungen
/// zugleich beantworten. Ausserdem sagt `abwurfregel`s Probe
/// `die_marke_hat_genau_einen_aufrufer` zu, dass aus einer Zeilennummer an
/// genau einer Stelle ein Ziel **waehrend des Ziehens** wird; ein zweiter
/// Aufrufer dort waere eine zweite Antwort auf jene Frage.
///
/// `#[must_use]`, weil das stille Fallenlassen des Rueckgabewerts unbemerkt
/// bliebe: der Abwurf naehme dann wieder jedes Ziel an, das die Zeile gerade
/// hergibt.
#[must_use]
fn abwurfziel(benennt_eine_zeile: bool, typ_der_zeile: Option<Typ>) -> Abwurfziel {
    match (benennt_eine_zeile, typ_der_zeile) {
        // Die `-1`: der Abwurf gilt der ganzen Liste, also dem angezeigten
        // Ordner. Das ist ein Ziel aus C4 und keine Notloesung.
        (false, _) => Abwurfziel::AngezeigterOrdner,
        // Die Ordnerzeile, die `abwurf_pruefen` markiert hat, steht noch.
        (true, Some(Typ::Ordner)) => Abwurfziel::Zeile,
        // An der markierten Zeile steht jetzt eine Datei.
        (true, Some(Typ::Datei)) => Abwurfziel::Keines,
        // Oder eine Verknuepfung; sie zaehlt nicht als Ordner, hier so wenig
        // wie in `abwurfregel::marke`.
        (true, Some(Typ::Verknuepfung)) => Abwurfziel::Keines,
        // Oder gar nichts mehr: die Liste ist kuerzer geworden.
        (true, None) => Abwurfziel::Keines,
    }
}

/// Welcher Ordner einen angenommenen Abwurf bekommt, oder keiner.
///
/// Drei Werte, ueberschneidungsfrei und vollstaendig. Der dritte ist der, den
/// es bis zum 260819 nicht gab: er heisst „die Zeile ist nicht mehr die, die
/// der Zeiger gezeigt hat", und sein Ausgang ist ein `false` an AppKit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Abwurfziel {
    /// Der Ordner der Zeile, die AppKit nennt.
    Zeile,
    /// Der angezeigte Ordner des sichtbaren Tabs.
    AngezeigterOrdner,
    /// Keiner: der Abwurf wird nicht angenommen.
    Keines,
}

/// Was ein Einstiegsversuch in eine Zeile ergeben hat.
///
/// **Drei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig**, und
/// der dritte ist der Grund, aus dem hier eine Aufzaehlung steht, wo bis zum
/// Defekt `260814-1612` ein Wahrheitswert genuegte: eine Verknuepfung, deren
/// Ziel nicht erreichbar ist, ist weder ein Einstieg noch etwas, das der
/// Aufrufer an das System weiterreichen sollte. Sie ist gemeldet, und das ist
/// eine dritte Antwort.
///
/// `#[must_use]`, weil ein stilles Fallenlassen unbemerkt bliebe: der
/// Doppelklick verzweigt daran, und ohne die Verzweigung oeffnete er eine Datei
/// nicht mehr. Der eine Aufrufer, der die Antwort nicht braucht, schreibt
/// `let _ =` und sagt damit ausdruecklich, dass er sie nicht braucht.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Einstieg {
    /// Der Ordner wird gelesen.
    Eingestiegen,
    /// Kein Ordner, oder die Zeile gibt es nicht. Was sonst geschieht,
    /// entscheidet der Aufrufer.
    KeinOrdner,
    /// Eine Verknuepfung, deren Ziel sich nicht aufloesen laesst. Der Grund
    /// steht bereits in der Statuszeile; der Aufrufer tut nichts weiter.
    Gemeldet,
}

/// Wer den Ersthelferrang mitnimmt, wenn ein Klick das aktive Dateifenster
/// umsetzt.
///
/// **Zwei Fokusgroessen, und diese Aufzaehlung sagt, wer die zweite bewegt.**
/// `Fenstermodell::aktiv` sagt, welches Dateifenster die Befehle meinen; der
/// Ersthelferrang von AppKit sagt, wohin die Tastendruecke gehen. Beide Wege in
/// `DateifensterQuelle::angefasst` schreiben das erste, und sie unterscheiden
/// sich allein darin, ob das zweite von selbst nachkommt.
///
/// **Gemessen und nicht angenommen.** Ein Klick auf eine Zeile laeuft ueber
/// `tableView:shouldSelectRow:`, und AppKit hat den Rang dabei schon in die
/// `NSTableView` gelegt. Ein Klick auf die Tableiste trifft ein
/// `NSSegmentedControl`, und das nimmt den Rang **nicht** an, obwohl sein
/// `acceptsFirstResponder` `1` liefert; am 260825 an einem Nachbau gemessen
/// (`shared/decisions/260825-1725_*_nimmt-ein-klick-auf-die-tableiste-des-
/// anderen-dateifensters-den-ersthelferrang-mit.md`). Ohne die Unterscheidung
/// stuende der Rang nach einem Tableistenklick in der Liste, die danach nicht
/// mehr die aktive ist.
///
/// **Zwei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig.** Ein
/// dritter Weg in `angefasst` haelt damit den Bau an, statt sich stillschweigend
/// den einen oder den anderen Fall auszusuchen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rangmitnahme {
    /// AppKit hat den Rang schon umgesetzt; KRK ruehrt ihn nicht an.
    ///
    /// Der Klick auf eine Zeile. Ein `makeFirstResponder:` daneben traefe die
    /// Ansicht, die den Rang gerade bekommt, und liefe mitten in AppKits eigener
    /// Behandlung des Mausklicks.
    Appkit,
    /// KRK nimmt den Rang mit.
    ///
    /// Der Klick auf die Tableiste. Getragen wird er von
    /// `Anwendungsdelegierter::fokus_setzen`, der einen Stelle, die
    /// `makeFirstResponder:` ruft.
    Krk,
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
    /// Was gerufen wird, wenn eine der fuenf Meldungsquellen dieses
    /// Dateifensters sich geaendert hat.
    ///
    /// **Der Ersatz fuer die eigene Statuszeile am Fuss** (C5 der Runde 6).
    /// Dieses Dateifenster haelt seine Quellen weiter und entscheidet weiter
    /// nichts darueber, was in der Zeile steht; es sagt nur, dass sich etwas
    /// geaendert hat. Wer die Zeile schreibt, holt danach beide Quellensaetze
    /// und die aktive Seite: das ist `Anwendungsdelegierter::
    /// statuszeile_nachziehen` und niemand sonst.
    ///
    /// Wahlfrei, im Zuschnitt der vier Rueckrufe darunter und aus demselben
    /// Grund: die Quelle kommt vor dem Anwendungsdelegierten zur Welt.
    meldungswechsel: RefCell<Option<Box<dyn Fn()>>>,
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
    ///
    /// **Das Argument sagt, ob der Ersthelferrang mitkommt**; welcher der beiden
    /// Klicks welchen Wert mitbringt und warum, steht an [`Rangmitnahme`].
    aktivierung: RefCell<Option<Aktivierungsmelder>>,
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
    /// Was gerufen wird, wenn die Auswahl auf einem anderen Eintrag steht
    /// (C6).
    ///
    /// Der Weg, auf dem eine neue Auswahl die Vorschau anstoesst: gemeldet
    /// wird der vollstaendige Pfad des ausgewaehlten Eintrags, `None` fuer
    /// eine aufgehobene Auswahl. Gerufen aus [`DateifensterQuelle::
    /// auswahl_merken`], der einen Stelle, die eine Zeile in einen Eintrag
    /// uebersetzt; Tastatur und Maus muenden beide dort. Wahlfrei, weil die
    /// Quelle vor dem Anwendungsdelegierten zur Welt kommt, wie die Rueckrufe
    /// darueber.
    auswahlmelder: RefCell<Option<Auswahlmelder>>,
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
    /// Der oberste der fuenf Raenge und der einzige, der ueber der
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
    /// oder Tabwechsel; siehe [`DateifensterQuelle::meldung_gewechselt`].
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
    /// Ob in diesem Dateifenster gerade eine Namenszelle bearbeitet wird (C4).
    ///
    /// **Gesetzt und geloescht an den beiden Enden, die AppKit hat**, und an
    /// keinem dritten: [`Namensfeld::wird_ersthelfer`] setzt, die
    /// Ueberschreibungen `textDidEndEditing:` und `abortEditing` loeschen. Dass
    /// diese zwei jedes Ende abdecken, ist gemessen; die Tabelle steht bei
    /// [`Namensfeld`].
    ///
    /// **Wozu das Kennzeichen dasteht.** Solange es steht, laesst
    /// [`crate::auffrischung::ordner_neu_lesen`] dieses Dateifenster nicht
    /// lesen: ein `reloadData` beendete die Bearbeitung, ohne die Aktion zu
    /// schicken, und der getippte Name waere fort (Nutzerentscheid vom
    /// 260816-0021).
    namensbearbeitung: Cell<bool>,
    /// Ob eine Auffrischung ausgefallen ist, waehrend die Namenszelle offen
    /// stand.
    ///
    /// **Der Nachhol-Weg, den der Vorgangsaufschub nicht braucht.** Dort holt
    /// der Abschluss der Operation die Ordner ohnehin neu; das Ende einer
    /// Bearbeitung hat nichts Vergleichbares, und ohne dieses Kennzeichen
    /// bliebe die Liste auf ihrem alten Stand stehen, bis irgendetwas anderes
    /// sie anfasst.
    ///
    /// Gesetzt von [`DateifensterQuelle::auffrischung_vormerken`], eingeloest
    /// von [`DateifensterQuelle::aufgeschobene_auffrischung_nachholen`] und
    /// geloescht von jedem wirklichen [`DateifensterQuelle::neu_lesen`] —
    /// letzteres, damit die Auffrischung, die die Umbenennung selbst
    /// ausloest, das Nachholen ueberfluessig macht statt es zu verdoppeln.
    auffrischung_vorgemerkt: Cell<bool>,
    /// Ob KRK gerade schon einen Vorgang haelt (C6 der Runde 13).
    ///
    /// Der sechste Rueckruf, wahlfrei wie die fuenf darueber und aus demselben
    /// Grund: die Quelle kommt vor dem Anwendungsdelegierten zur Welt. Was ein
    /// fehlender Rueckruf bedeutet, entscheidet
    /// [`DateifensterQuelle::vorgang_laeuft_fragen`] an einer Stelle.
    vorgang_laeuft: RefCell<Option<Vorgangsfrage>>,
    /// Was mit einem angenommenen Abwurf zu geschehen hat (C4 bis C6 der
    /// Runde 13).
    ///
    /// Der siebte Rueckruf. Er geht an den Anwendungsdelegierten, weil die
    /// Operationsmaschine dort haengt; von hier aus ist sie nicht zu erreichen,
    /// und ein zweiter Weg hinein entstuende sonst. Wahlfrei aus demselben
    /// Grund wie die sechs darueber.
    abwurf: RefCell<Option<Abwurfmelder>>,
    /// Wie sich die Befehlsantwort an **beiden** Dateifenstern raeumen laesst
    /// (C7 der Runde 13).
    ///
    /// Der achte Rueckruf, wahlfrei wie die sieben darueber. Er geht an
    /// `Anwendungsdelegierter::befehlsantwort_beidseitig_loeschen`, also an die
    /// eine Loeschregel des Rangs 1, und traegt keine eigene daneben: von einer
    /// Quelle aus ist die andere Seite nicht zu erreichen, und die Meldung des
    /// Abwurfs bliebe sonst hinter einer stehenden Befehlsantwort im aktiven
    /// Dateifenster liegen. Was ein fehlender Rueckruf bedeutet, entscheidet
    /// [`DateifensterQuelle::befehlsantwort_beidseitig_loeschen`] an einer
    /// Stelle.
    befehlsantwort_raeumer: RefCell<Option<Box<dyn Fn()>>>,
    /// Was mit einem angeklickten Eintrag des Kontextmenues zu geschehen ist
    /// (Runde 17).
    ///
    /// Der neunte Rueckruf, wahlfrei wie die acht darueber und aus demselben
    /// Grund: die Quelle kommt vor dem Anwendungsdelegierten zur Welt. Was ein
    /// fehlender Rueckruf bedeutet, entscheidet
    /// [`DateifensterQuelle::kontextbefehl_melden`] an einer Stelle, naemlich
    /// gar nichts zu tun. Eintreten kann der Fall nicht: der Rueckruf steht
    /// seit dem Aufbau der Oberflaeche, und ein Rechtsklick braucht ein
    /// stehendes Fenster.
    kontextbefehl: RefCell<Option<Kontextmelder>>,
    /// Der Grund, den der vorige Durchgang von
    /// [`DateifensterQuelle::abwurf_pruefen`] gefaellt hat, `None` fuer ein
    /// angenommenes Urteil (C7).
    ///
    /// **Keine zweite Wahrheit ueber irgendetwas, sondern die Entdopplung der
    /// Meldung.** `validateDrop:` laeuft bei jeder Zeigerbewegung; geschrieben
    /// wird die Zeile nur, wenn der Grund sich gegenueber diesem Feld geaendert
    /// hat. Die Regel dazu steht als reine Funktion in [`abwurfmeldung`], samt
    /// der Tafel und dem Grund.
    ///
    /// **Seine Loeschregel ist die des Rangs 1.** Es faellt mit der
    /// Befehlsantwort, die es beschreibt, und an keiner anderen Stelle; siehe
    /// [`DateifensterQuelle::befehlsantwort_loeschen`]. Ein Feld, das laenger
    /// stuende als die Zeile, die es meint, liesse eine zweite gleiche
    /// Ziehbewegung stumm.
    ///
    /// **Dass die Regel beide Seiten raeumt, gilt auch hier**, und es ist die
    /// richtige Wirkung: schreibt der Abwurf seine Meldung ueber dem einen
    /// Dateifenster, faellt der gemerkte Grund des anderen mit dessen Zeile.
    /// Zieht der Nutzer danach zurueck, wird die Meldung dort neu geschrieben —
    /// sie folgt dem Zeiger, statt an der Seite zu kleben, die sie zuerst hatte.
    gemeldeter_abwurfgrund: Cell<Option<Abwurfgrund>>,
    /// Der Vorgang, den [`DateifensterQuelle::abwurf_pruefen`] zuletzt
    /// beschlossen hat, `None` fuer ein abgewiesenes Urteil (C5).
    ///
    /// **Ein eigenes Feld neben dem gemerkten Grund und nicht dasselbe**, weil
    /// die Loeschregeln die entgegengesetzten sind: der gemerkte Grund faellt
    /// mit der Befehlsantwort, dieses Feld darf das nicht — ein Tastendruck
    /// waehrend eines stehenden Ziehvorgangs naehme dem Loslassen sonst seinen
    /// Vorgang. Ein Feld mit zwei Loeschregeln waere derselbe Sonderfall, den
    /// `fenstermeldung` und `vorgangsanzeige` weiter oben schon vermeiden.
    ///
    /// **Es traegt das Urteil vom Ziehen zum Loslassen**, und genau deshalb
    /// faellt es kein zweites: `tableView:acceptDrop:…` bekommt von AppKit
    /// keine Zusatztaste und keine angebotene Menge mehr, und eine zweite
    /// Beurteilung koennte anders ausfallen als die, die der Zeiger gezeigt
    /// hat. C5 sagt zu, dass beide uebereinstimmen.
    beschlossener_vorgang: Cell<Option<Abwurfvorgang>>,
    /// Was die Ablage des laufenden Ziehvorgangs traegt, samt der Nummer der
    /// Ziehsitzung, zu der sie gehoert (C4 bis C7 der Runde 13).
    ///
    /// **Kein Zwischenspeicher aus Bequemlichkeit, sondern gegen eine gemessene
    /// Zahl.** `validateDrop:` laeuft bei **jeder** Zeigerbewegung, und
    /// [`super::zwischenablage::dateiverweise`] baut dabei je gezogenem Eintrag
    /// ein `NSURL` ueber den Ablageserver und einen `PathBuf` daneben. Am
    /// 260819 auf dem Referenzgeraet gemessen, im Profil `release` und je
    /// Durchgang: 1 Eintrag 0,13 ms, 100 Eintraege 6,0 ms, 1.000 Eintraege
    /// 155 ms, 5.000 Eintraege 585 ms. Ein Bild dauert bei 60 Hz 16,7 ms; von
    /// hundert gezogenen Eintraegen an verbraucht der Aufruf allein davon mehr
    /// als ein Drittel, von tausend an steht die Anwendung. Das ist genau das
    /// Kriterium, das diese Runde an die Stelle einer elften Zeitzusage gesetzt
    /// hat: „waehrend ein Ziehvorgang ueber der Dateiliste steht, bleibt die
    /// Liste bildlauffaehig".
    ///
    /// **Gemerkt wird, was allein an der Ablage haengt**, und nichts, was am
    /// Zeiger haengt: ob ueberhaupt ein Dateiverweis kommt, und in welchem
    /// Ordner die gezogenen Eintraege liegen, falls sie alle in einem liegen.
    /// Der Zielordner wechselt mit jeder Zeile und steht deshalb nicht hier.
    ///
    /// # Die Loeschregel
    ///
    /// **Es wird ersetzt und nicht geraeumt, und der Schluessel dazu ist
    /// `NSDraggingInfo::draggingSequenceNumber`** (`NSDragging.h:81`), die
    /// Nummer, die eine Ziehsitzung eindeutig benennt.
    ///
    /// - **Geschrieben** wird das Feld allein in
    ///   [`DateifensterQuelle::abwurfquellen`], und dort allein dann, wenn die
    ///   gemerkte Nummer nicht die des laufenden Ziehvorgangs ist.
    /// - **Gelesen** wird es allein an derselben Stelle, und allein dann, wenn
    ///   die Nummern uebereinstimmen.
    /// - **Es faellt nie von selbst.** Ein `draggingExited:` gibt es hier
    ///   nicht, und es soll auch keines geben.
    ///
    /// Die drei Lagen, in denen ein geraeumtes Feld noetig schiene, und warum
    /// keine es verlangt:
    ///
    /// - **Der Ziehvorgang verlaesst die Liste und kehrt zurueck.** Die Nummer
    ///   ist dieselbe, die Ablage ist dieselbe, das Gemerkte stimmt. Wer hier
    ///   raeumte, zahlte die gemessene Zeit oben bei jedem Wiedereintritt noch
    ///   einmal.
    /// - **Der Ziehvorgang wird abgebrochen.** Es kommt kein weiterer Aufruf.
    ///   Das Feld bleibt stehen und traegt Pfade und einen Wahrheitswert, keine
    ///   Griffe auf etwas, das ablaufen koennte.
    /// - **Ein zweiter Ziehvorgang beginnt ohne ein Loslassen dazwischen.** Er
    ///   traegt eine andere Nummer, und der erste `validateDrop:` ersetzt das
    ///   Feld. Genau dafuer steht die Nummer darin.
    ///
    /// **Es ist damit die Bauform von [`QuelleIvars::beschlossener_vorgang`]
    /// und ausdruecklich nicht die von
    /// [`QuelleIvars::gemeldeter_abwurfgrund`]**: es faellt nicht mit der
    /// Befehlsantwort, denn ein Tastendruck waehrend eines stehenden
    /// Ziehvorgangs hat mit dessen Ablage nichts zu schaffen. Ein dritter
    /// Ausgang neben den zweien, die diese Datei fuehrt, entsteht nicht.
    ///
    /// **Was es nicht traegt, ist die Sicherheit gegen eine wiederverwendete
    /// Nummer.** `speculation:` — der Kopf des Systems sagt „uniquely
    /// identifies", ohne eine Aussage ueber die Wiederverwendung nach dem Ende
    /// einer Sitzung zu machen. Eine wiederverwendete Nummer liesse einen
    /// zweiten Ziehvorgang die Quellen des ersten sehen. Die Nummer ist ein
    /// `NSInteger`; dass sie innerhalb einer Anwendungssitzung ueberlaeuft, ist
    /// nicht anzunehmen.
    abwurfquellen: RefCell<Option<Abwurfquellen>>,
}

/// Was die Ablage eines Ziehvorgangs traegt, und zu welcher Ziehsitzung sie
/// gehoert (C4 bis C7 der Runde 13).
///
/// Der Inhalt von [`QuelleIvars::abwurfquellen`]; die Loeschregel und der
/// gemessene Grund stehen dort. Beide Tatsachen haengen allein an der Ablage
/// und nicht am Zeiger, und genau deshalb duerfen sie eine Ziehsitzung lang
/// stehen bleiben.
#[derive(Debug, Clone)]
struct Abwurfquellen {
    /// `NSDraggingInfo::draggingSequenceNumber` der Sitzung, aus der die
    /// beiden Tatsachen darunter stammen.
    folge: NSInteger,
    /// Ob die Ablage mindestens einen Dateiverweis liefert (C7).
    traegt_dateien: bool,
    /// Der Ordner, in dem **alle** gezogenen Eintraege liegen, falls sie alle
    /// in einem liegen (C6, Lage 3). Gebaut von [`gemeinsamer_quellordner`].
    gemeinsamer_ordner: Option<PathBuf>,
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

        /// Der Klick auf einen der drei eigenen Kontextmenue-Eintraege
        /// (Runde 17).
        ///
        /// **Ein Selektor fuer alle drei, und die Marke sagt, welcher gemeint
        /// war.** Drei Selektoren nebeneinander waeren drei Stellen, an denen
        /// einer fehlen koennte, ohne dass etwas meldet; so gibt es genau eine,
        /// und wer einen vierten Befehl anlegt, kommt an
        /// [`Kontextbefehl`] nicht vorbei.
        ///
        /// Der Rumpf steht daneben, in
        /// [`DateifensterQuelle::kontextbefehl_melden`], nach der Bauform von
        /// `validateDrop:` weiter unten: dieser Block ist die Liste dessen,
        /// was KRK entgegennimmt, und bleibt eine Liste.
        // SAFETY: Die Signatur passt zu der, die ein `NSMenuItem` an sein Ziel
        // schickt: ein Argument, der Absender. Er ist der Eintrag, an dem die
        // Handlung gesetzt wurde, und traegt damit die Marke, die
        // `eigene_kontexteintraege_anfuegen` ihm gegeben hat.
        #[unsafe(method(kontextbefehl:))]
        fn kontextbefehl(&self, absender: &NSMenuItem) {
            self.kontextbefehl_melden(absender.tag());
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

        /// Waehrend der Nutzer die Maustaste noch haelt: was geschaehe, wenn
        /// er hier loslaesst (C4 bis C7 der Runde 13).
        ///
        /// **Der Rumpf steht daneben, in [`DateifensterQuelle::
        /// abwurf_pruefen`]**, nach der Bauform von `ansicht_fuer_zelle`
        /// weiter unten. Dieser Block ist die Liste dessen, was KRK dem
        /// Protokoll beantwortet, und er ist nur so lange zu ueberblicken, wie
        /// er eine Liste bleibt; ein Rumpf von vierzig Zeilen mittendrin
        /// verdeckte die beiden Nachbarn.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableView:validateDrop:proposedRow:proposedDropOperation:))]
        fn ziehen_pruefen(
            &self,
            _tabelle: &NSTableView,
            zug: &ProtocolObject<dyn NSDraggingInfo>,
            zeile: NSInteger,
            vorgeschlagen: NSTableViewDropOperation,
        ) -> NSDragOperation {
            self.abwurf_pruefen(zug, zeile, vorgeschlagen)
        }

        /// Der Nutzer hat losgelassen (C4 bis C6 der Runde 13).
        ///
        /// AppKit ruft dies **allein dann**, wenn `validateDrop:` etwas
        /// anderes als `NSDragOperation::None` zurueckgegeben hat. Das Urteil
        /// ist damit gefaellt, und der Rumpf faellt kein zweites; der Rumpf
        /// steht daneben, aus demselben Grund wie oben.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableView:acceptDrop:row:dropOperation:))]
        fn ziehen_annehmen(
            &self,
            _tabelle: &NSTableView,
            zug: &ProtocolObject<dyn NSDraggingInfo>,
            zeile: NSInteger,
            _vorgeschlagen: NSTableViewDropOperation,
        ) -> bool {
            self.abwurf_annehmen(zug, zeile)
        }
    }

    // SAFETY: `NSMenuDelegate` stellt keine Bedingungen. Das Menue haelt
    // seinen Delegierten **schwach** ("This is a weak property",
    // `objc2-app-kit-0.3.2/src/generated/NSMenu.rs:356-361`, und der Kopf des
    // Systems sagt dasselbe: `@property (nullable, weak) id<NSMenuDelegate>
    // delegate;`, `NSMenu.h:156`); die Tabelle haelt das Menue stark und
    // `Dateifenster` die Quelle. Der Ring Quelle → Tabelle → Menue →
    // Delegierter → Quelle bleibt damit an der letzten Kante offen.
    unsafe impl NSMenuDelegate for DateifensterQuelle {
        /// Baut das Kontextmenue der Dateiliste, bei jedem Rechtsklick neu
        /// (C1 der Runde 6, sechstes Kriterium).
        ///
        /// **Neu und nicht ergaenzt.** Die betroffenen Eintraege aendern sich
        /// zwischen zwei Klicks, und ein Menue, das vom vorigen Mal
        /// stehenbliebe, teilte etwas anderes als das, was der Eintrag
        /// verspricht. Eine Tabelle bringt kein eigenes Kontextmenue mit, es
        /// geht deshalb nichts verloren; die `NSTextView` des Editors, die
        /// eines mitbringt, haengt sich aus genau diesem Grund anders an
        /// (siehe den Kopf von [`super::teilen`]).
        ///
        /// **Der Rechtsklick rueckt die Auswahl nach, bevor die betroffenen
        /// Eintraege nachgeschlagen werden** — es sei denn, die angeklickte
        /// Zeile ist markiert. So hat der Nutzer es am 260812-1200 entschieden
        /// (`decisions/260812-1145_*_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md`
        /// dieser Runde), und deshalb steht
        /// [`Self::rechtsklick_auswahl_nachziehen`] hier **vor**
        /// [`Self::betroffene_eintraege`] und nicht in ihm.
        ///
        /// **Eine zweite Auswahlregel entsteht dabei nicht.**
        /// [`crate::kommandos::operationen::betroffene`] bleibt unangetastet
        /// und beantwortet weiterhin allein, worauf ein Befehl wirkt; geaendert
        /// wird die Auswahl vor ihr. Wer die Ausnahme hier sucht, findet sie
        /// nicht: sie steht als reine Funktion in
        /// [`crate::kommandos::operationen::rechtsklick_zielzeile`], samt der
        /// Begruendung und der Ablehnung der beiden anderen Moeglichkeiten.
        ///
        /// **Erst die drei eigenen Eintraege, dann der Freigabeeintrag**, und
        /// die Folge ist die Form des Menues und keine Geschmacksfrage: jener
        /// Bauer fuegt vorn ein und setzt seinen Trenner nur, wenn schon etwas
        /// dasteht. Der Modulkopf schreibt es aus.
        ///
        /// Die Ausleihe des Tabmodells endet in jeder der beiden ersten
        /// Zeilen, vor dem ersten Objective-C-Aufruf; siehe den Modulkopf.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(menuNeedsUpdate:))]
        fn menue_auffrischen(&self, menue: &NSMenu) {
            self.rechtsklick_auswahl_nachziehen();
            let betroffen = self.betroffene_eintraege();
            menue.removeAllItems();
            self.eigene_kontexteintraege_anfuegen(menue);
            teilen::eintrag_anfuegen(menue, &betroffen.pfade, self.mtm());
        }
    }
);

impl DateifensterQuelle {
    /// Eine Datenquelle fuer die genannte Tabelle.
    fn neu(
        mtm: MainThreadMarker,
        tabelle: Retained<NSTableView>,
        sicht: Retained<NSScrollView>,
        tabs: Tabliste,
    ) -> Retained<Self> {
        let groessenformat = NSByteCountFormatter::new();
        groessenformat.setCountStyle(NSByteCountFormatterCountStyle::File);
        let this = Self::alloc(mtm).set_ivars(QuelleIvars {
            tabelle,
            sicht,
            meldungswechsel: RefCell::new(None),
            tableiste: RefCell::new(None),
            tabs: RefCell::new(tabs),
            einzug: RefCell::new(None),
            aktivierung: RefCell::new(None),
            ordnerwechsel: RefCell::new(None),
            auswahlmelder: RefCell::new(None),
            umbenennung: RefCell::new(None),
            befehlsantwort: RefCell::new(None),
            fenstermeldung: RefCell::new(None),
            vorgangsanzeige: RefCell::new(None),
            groessenformat,
            namensbearbeitung: Cell::new(false),
            auffrischung_vorgemerkt: Cell::new(false),
            vorgang_laeuft: RefCell::new(None),
            abwurf: RefCell::new(None),
            befehlsantwort_raeumer: RefCell::new(None),
            kontextbefehl: RefCell::new(None),
            gemeldeter_abwurfgrund: Cell::new(None),
            beschlossener_vorgang: Cell::new(None),
            abwurfquellen: RefCell::new(None),
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
    pub fn aktivierung_setzen(&self, melden: Aktivierungsmelder) {
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

    /// Hinterlegt, was bei einer neuen Auswahl zu tun ist (C6).
    pub fn auswahlmelder_setzen(&self, melden: Auswahlmelder) {
        *self.ivars().auswahlmelder.borrow_mut() = Some(melden);
    }

    /// Hinterlegt, was zu tun ist, wenn eine Meldungsquelle sich geaendert hat
    /// (C5 der Runde 6).
    pub fn meldungswechsel_setzen(&self, melden: Box<dyn Fn()>) {
        *self.ivars().meldungswechsel.borrow_mut() = Some(melden);
    }

    /// Hinterlegt, wie sich erfragen laesst, ob schon ein Vorgang laeuft
    /// (C6 der Runde 13).
    pub fn vorgang_laeuft_setzen(&self, fragen: Vorgangsfrage) {
        *self.ivars().vorgang_laeuft.borrow_mut() = Some(fragen);
    }

    /// Hinterlegt, was mit einem angenommenen Abwurf zu geschehen hat
    /// (C4 bis C6 der Runde 13).
    pub fn abwurf_setzen(&self, melden: Abwurfmelder) {
        *self.ivars().abwurf.borrow_mut() = Some(melden);
    }

    /// Hinterlegt, wie sich die Befehlsantwort an beiden Dateifenstern raeumen
    /// laesst (C7 der Runde 13).
    pub fn befehlsantwort_raeumer_setzen(&self, raeumen: Box<dyn Fn()>) {
        *self.ivars().befehlsantwort_raeumer.borrow_mut() = Some(raeumen);
    }

    /// Hinterlegt, was mit einem angeklickten Eintrag des Kontextmenues zu
    /// geschehen ist (Runde 17).
    ///
    /// **Der eine Aufrufer steht im Aufbau der Oberflaeche**, je Fensterseite
    /// einmal (`Anwendungsdelegierter::oberflaeche_aufbauen`). Ohne ihn faellt
    /// [`Self::kontextbefehl_melden`] still durch, und die drei Menueeintraege
    /// stuenden da und taeten nichts — genau die Falle, gegen die der eine
    /// Selektor und die Marke gebaut sind. Dass er gesetzt wird, haelt die
    /// Probe `der_kontextmelder_wird_beim_aufbau_gesetzt` beim
    /// Anwendungsdelegierten; eine Probe an dieser Stelle koennte es nicht,
    /// denn sie brauchte den Hauptfaden.
    pub fn kontextmelder_setzen(&self, melden: Kontextmelder) {
        *self.ivars().kontextbefehl.borrow_mut() = Some(melden);
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
    ///
    /// **Seit der Runde 13 nennt ein Tastenbefehl ein anderes Dateifenster als
    /// das aktive:** `Anwendungsdelegierter::ordner_angleichen` stellt das
    /// Gegenueber auf den Ordner des aktiven (C1). Unter den Befehlen ist er
    /// darin der erste; eine Seite fest zu benennen kannte der Baum bis dahin
    /// nur aus dem Messmodus, und der ist kein Befehl. Fuer diese Methode
    /// aendert das nichts: sie wechselt den Ordner des **sichtbaren** Tabs und
    /// legt keinen neuen an, gleich von welcher Seite aus gerufen.
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
    ///
    /// **Die Liste bleibt waehrenddessen stehen.** Der Tab behaelt seinen
    /// gelesenen Bestand, bis der neue Lesevorgang seinen ersten Stapel
    /// liefert; siehe [`Tabliste::aktiven_neu_lesen`]. Ohne das lief die Liste
    /// waehrend eines Stapel-Umbenennens im angezeigten Ordner leer.
    pub fn neu_lesen(&self) {
        // Was aufgeschoben war, ist damit erledigt: dieser Lesevorgang holt
        // genau das, was der Aufschub hat ausfallen lassen. Ohne diese Zeile
        // liefe nach einer Umbenennung ein zweiter Lesevorgang, der dem ersten
        // seine vorgemerkte Auswahl naehme.
        self.ivars().auffrischung_vorgemerkt.set(false);
        // Zuerst die Bildlaufposition aus der Ansicht in den Tab holen: sie
        // steht in der `NSScrollView` und nirgends sonst, und der naechste
        // Schritt merkt sie als noch herzustellen vor.
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
        self.ivars().tabelle.reloadData();
        // Die Auswahl des Modells an die Tabelle geben, statt sich darauf zu
        // verlassen, dass `reloadData` eine Zeilennummer jenseits der neuen
        // Zeilenzahl von selbst fallen laesst; das hiesse, eine Zusage von
        // AppKit anzunehmen statt sie zu geben. Welche Auswahl das ist, haengt
        // am Weg: eine Navigation hat den Tab ausgetauscht und hat keine, eine
        // Auffrischung zeigt weiter die Zeilen des bisherigen Lesevorgangs und
        // behaelt die Auswahl darauf, bis dessen Bestand abgeloest wird. In
        // beiden Faellen kommt sie mit dem Abschluss ueber `wunschauswahl`
        // zurueck, also ueber dieselbe Huelle, die sie nach einem Umsortieren
        // wiederherstellt.
        self.auswahl_anzeigen();
        self.meldung_gewechselt();
        self.einzug_starten();
        self.tableiste_nachziehen();
    }

    /// Nach einem Tabwechsel: Inhalt, Auswahl, Bildlauf und Leiste nachziehen.
    fn tab_gewechselt(&self) {
        self.fenstermeldung_loeschen();
        self.ivars().tabelle.reloadData();
        self.auswahl_anzeigen();
        let bildlauf = self.ivars().tabs.borrow().aktiver().bildlauf();
        self.bildlauf_herstellen(bildlauf);
        self.meldung_gewechselt();
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

    /// Der vollstaendige Pfad des ausgewaehlten Eintrags; `None` ohne Auswahl.
    ///
    /// Nur zum Ablesen, fuer die Endbedingung von L7: die Vorschau ist
    /// fertig, wenn sie genau diesen Pfad zeigt.
    pub fn auswahl_pfad(&self) -> Option<PathBuf> {
        let zeile = self.ivars().tabelle.selectedRow();
        if zeile < 0 {
            return None;
        }
        let tabs = self.ivars().tabs.borrow();
        let tab = tabs.aktiver();
        tab.modell()
            .zeile(zeile as usize)
            .map(|eintrag| tab.ordner().join(&eintrag.name))
    }

    /// Ob die Vorgangsanzeige einer Dateioperation in der Statuszeile steht.
    ///
    /// Nur zum Ablesen, fuer die Endbedingung von L8: die Zeile erscheint mit
    /// dem naechsten Zeichendurchgang, nachdem sie hier gesetzt wurde.
    pub fn vorgang_sichtbar(&self) -> bool {
        self.ivars().vorgangsanzeige.borrow().is_some()
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
            Kommando::OrdnerpfadKopieren => self.ordnerpfad_kopieren(),
            Kommando::EintragspfadKopieren => self.eintragspfad_kopieren(),
            Kommando::MitStandardprogrammOeffnen => {
                self.mit_standardprogramm_oeffnen(&self.betroffene_eintraege().pfade)
            }
            Kommando::Umbenennen => return self.umbenennung_beginnen(),
            // Nicht Sache eines einzelnen Dateifensters.
            _ => return false,
        }
        true
    }

    /// Rueckt die Auswahl vor einem Rechtsklick auf die angeklickte Zeile.
    ///
    /// Der eine Aufrufer ist `menuNeedsUpdate:` oben, und der Zeitpunkt ist
    /// die halbe Regel: gerufen wird **vor** [`Self::betroffene_eintraege`].
    /// Ob ueberhaupt gerueckt wird, entscheidet
    /// [`operationen::rechtsklick_zielzeile`] ohne Fenster; hier bleibt allein,
    /// was AppKit betrifft, die angeklickte Zeile zu erfragen und die neue zu
    /// setzen.
    ///
    /// **Gesetzt wird ueber [`Self::zeile_setzen`]**, also ueber denselben Weg,
    /// den die Tastatur nimmt. Der Datensatz verlangt das ausdruecklich: nur
    /// dort laeuft [`Self::auswahl_merken`] mit, und ohne das erfuehre die
    /// Vorschau aus C6 nichts von der neuen Auswahl. Ein zweiter Weg an
    /// `auswahl_merken` vorbei waere der Fehler, den diese Datei sonst
    /// ueberall vermeidet.
    fn rechtsklick_auswahl_nachziehen(&self) {
        // `clickedRow` liefert -1, wenn der Klick auf keine Zeile fiel;
        // `rechtsklick_zielzeile` faengt das ab und antwortet `None`.
        let angeklickt = self.ivars().tabelle.clickedRow();
        let ziel = {
            let tabs = self.ivars().tabs.borrow();
            operationen::rechtsklick_zielzeile(tabs.aktiver().modell(), angeklickt)
        };
        // Nach dem Ende der Ausleihe: `zeile_setzen` ruft in AppKit und ueber
        // den Auswahlrueckruf in dieselbe Quelle zurueck.
        if let Some(zeile) = ziel {
            self.zeile_setzen(zeile);
        }
    }

    /// Haengt die drei eigenen Eintraege an das Kontextmenue (Runde 17).
    ///
    /// **Gebaut wird ueber [`Kontextbefehl::ALLE`] und nicht aus drei Zeilen
    /// von Hand.** Damit ist die Reihenfolge im Menue dieselbe Angabe wie die
    /// Reihenfolge jener Liste, und ein vierter Befehl erscheint, ohne dass
    /// hier jemand etwas nachzieht. Titel und Marke kommen aus
    /// [`crate::kommandos::kontextmenue`]; diese Stelle rechnet nichts nach.
    ///
    /// **Angehaengt und nicht vorn eingefuegt.** Der Freigabeeintrag kommt
    /// danach und setzt sich mit seinem Trenner selbst nach vorn; wer hier
    /// einfuegte statt anzuhaengen, drehte die drei um.
    ///
    /// **Angelegt wird der Eintrag nicht hier, sondern in
    /// [`super::menue::ohne_kuerzel`].** C2.10 der Runde 7 sagt zu, dass genau
    /// eine Stelle im Baum ein `NSMenuItem` anlegt, und die Probe
    /// `es_gibt_eine_stelle_je_anlage_und_uebersetzung` haelt die Zusage ueber
    /// den ganzen Quellbaum. Ein eigener Erzeuger an dieser Stelle waere der
    /// Doppelbau, den sie verhindert; mehrere Aufrufer der Huelle sind
    /// dagegen ausdruecklich vorgesehen.
    ///
    /// **Ein Ziel setzt jene Huelle nicht, und hier bekommt der Eintrag
    /// eines.** Das Hauptmenue laesst die Antwortkette entscheiden; diese drei
    /// Eintraege koennen das nicht, denn ihr Befehl haengt an der Fensterseite,
    /// aus der der Rechtsklick kam, und die kennt nur diese Quelle.
    ///
    /// **Kein Eintrag wird ausgegraut und keiner weggelassen**, auch dann
    /// nicht, wenn er nichts vorfinden wird. Was er vorfindet, entscheidet
    /// sich im Augenblick der Ausfuehrung und wird dort in der Statuszeile
    /// gemeldet; eine zweite Zulaessigkeitsmaschine neben
    /// [`crate::kommandos::zulaessigkeit`], die fuer das Hauptmenue gebaut ist,
    /// entsteht damit nicht.
    fn eigene_kontexteintraege_anfuegen(&self, menue: &NSMenu) {
        let mtm = self.mtm();
        for befehl in Kontextbefehl::ALLE {
            let eintrag = menue::ohne_kuerzel(
                mtm,
                &NSString::from_str(befehl.titel()),
                Some(sel!(kontextbefehl:)),
            );
            eintrag.setTag(befehl.menuemarke());
            // SAFETY: `setTarget:` verlangt, dass das Ziel vom richtigen Typ
            // ist (`NSMenuItem.h:93`), also auf die gesetzte Handlung
            // antwortet; diese Quelle traegt `kontextbefehl:` selbst. Gehalten
            // wird das Ziel **schwach** ("@property (nullable, weak) id
            // target"), und das haelt den Ring offen: Quelle → Tabelle → Menue
            // → Eintrag → Ziel bricht an der letzten Kante, genau wie beim
            // Delegierten des Menues. Ein gestorbenes Ziel liefert `nil` und
            // keinen Absturz.
            unsafe { eintrag.setTarget(Some(self)) };
            menue.addItem(&eintrag);
        }
    }

    /// Meldet den angeklickten Kontextmenue-Befehl weiter (Runde 17).
    ///
    /// Der Rumpf hinter dem Selektor `kontextbefehl:`. **Er entscheidet
    /// nichts**, sondern rechnet die Marke ueber
    /// [`Kontextbefehl::von_menuemarke`] zurueck und reicht den Wert an den
    /// Anwendungsdelegierten; ausgefuehrt wird dort, weil die
    /// Operationsmaschine dort haengt.
    ///
    /// **Zwei Wege enden hier still, und beide sind gewollt.** Eine Marke, die
    /// keinen der drei Befehle benennt, kommt von einem Eintrag, den nicht
    /// diese Datei gesetzt hat — die Null eines ungesetzten `NSMenuItem`
    /// eingeschlossen —, und ein fehlender Rueckruf heisst, dass die
    /// Oberflaeche noch nicht steht. In beiden Lagen ist nichts zu tun die
    /// richtige Antwort und nicht die bequeme: ein Rueckfall auf den ersten
    /// Befehl packte Dateien, die niemand packen wollte.
    fn kontextbefehl_melden(&self, marke: NSInteger) {
        let Some(befehl) = Kontextbefehl::von_menuemarke(marke) else {
            return;
        };
        let melden = self.ivars().kontextbefehl.borrow();
        let Some(melden) = melden.as_ref() else {
            return;
        };
        melden(befehl);
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

    /// Worauf der Unzip-Eintrag des Kontextmenues in diesem Dateifenster wirkt
    /// (Runde 17).
    ///
    /// Das Gegenstueck zu [`Self::betroffene_eintraege`] und aus demselben
    /// Zuschnitt: die Regel steht in [`kontextmenue::entpackziel`] und ist ohne
    /// Fenster pruefbar, hier bleibt allein die Ausleihe des Tabmodells.
    ///
    /// **Sie steht hier und nicht beim Anwendungsdelegierten, der den Befehl
    /// ausfuehrt.** Die Regel fragt neben den betroffenen Eintraegen die
    /// **sichtbaren Zeilen**, also das [`Ordnermodell`] des sichtbaren Tabs;
    /// das liegt in den Ivars dieser Quelle, und der Delegierte kommt an es
    /// nicht heran, ohne einen zweiten Weg an ihm vorbei zu bauen.
    ///
    /// **Beide Fragen gehen durch dieselbe Ausleihe.** Zwei nacheinander
    /// koennten einen Lesevorgang zwischen sich haben und Auswahl und Bestand
    /// aus zwei Staenden desselben Ordners mischen; hier sieht die Regel eine
    /// Liste und dieselbe, die der Nutzer beim Klicken vor sich hatte.
    pub fn entpackbefund(&self) -> Entpackbefund {
        let tabs = self.ivars().tabs.borrow();
        let tab = tabs.aktiver();
        let betroffen = operationen::betroffene(tab.modell(), tab.ordner());
        kontextmenue::entpackziel(tab.modell(), &betroffen.pfade, tab.ordner())
    }

    /// Legt den Pfad des angezeigten Ordners in die Zwischenablage (C1).
    ///
    /// **Markierung und Auswahl gehen nicht ein.** Der Befehl fragt nach dem
    /// Ordner und nicht nach dem, was darin steht; das Ergebnis ist dasselbe,
    /// ob nichts oder dreissig Eintraege markiert sind.
    ///
    /// Dass der Ordner der des **aktiven** Dateifensters ist, kostet hier keine
    /// Zeile: der Wirkungsbereich `Dateifenster` fuehrt das Kommando ueber
    /// `bereichskommando` an die aktive Fensterseite, und diese Quelle gehoert
    /// ihr.
    fn ordnerpfad_kopieren(&self) {
        let ordner = self.angezeigter_ordner();
        let text = operationen::pfadtext(&ordner);
        if super::zwischenablage::text_schreiben(&text) {
            self.befehlsantwort_zeigen(&operationen::kopiermeldung(std::slice::from_ref(&ordner)));
        } else {
            self.befehlsantwort_zeigen(&operationen::ablage_weist_ab());
        }
    }

    /// Legt die Pfade der betroffenen Eintraege in die Zwischenablage (C2).
    ///
    /// Der fuenfte Abnehmer von [`operationen::betroffene`]: die Markierung hat
    /// den Vorrang, sonst gilt der Eintrag unter der Auswahl, und gezaehlt
    /// werden allein die sichtbaren, in Sichtreihenfolge. Eine zweite Regel
    /// daneben entsteht nicht.
    ///
    /// **Bei leerer Menge bleibt die Zwischenablage unberuehrt**, und die
    /// Statuszeile sagt es: wortlos nichts zu tun ist nach C2 nicht zulaessig.
    ///
    /// Eine Rueckfrage entsteht in keinem Fall, auch nicht bei dreissig
    /// markierten Eintraegen. Der Befehl zerstoert nichts, und die Meldung mit
    /// der Zahl ist die Antwort darauf, dass der Nutzer der Zwischenablage
    /// nicht ansieht, wie viele Zeilen er erzeugt hat.
    fn eintragspfad_kopieren(&self) {
        let betroffen = self.betroffene_eintraege();
        if betroffen.ist_leer() {
            self.befehlsantwort_zeigen(&operationen::nichts_zu_kopieren());
            return;
        }
        let text = operationen::pfadzeilen(&betroffen.pfade);
        if super::zwischenablage::text_schreiben(&text) {
            self.befehlsantwort_zeigen(&operationen::kopiermeldung(&betroffen.pfade));
        } else {
            self.befehlsantwort_zeigen(&operationen::ablage_weist_ab());
        }
    }

    /// Gibt die Eintraege an das Standardprogramm des Systems (C3).
    ///
    /// **Die eine Umsetzung des Oeffnens.** Sie nimmt die Menge, auf die sie
    /// wirken soll, statt sie selbst zu bestimmen: die Taste uebergibt ihr
    /// [`Self::betroffene_eintraege`], und damit wird der Oeffner der sechste
    /// Abnehmer von [`operationen::betroffene`] — dieselbe Regel wie bei den
    /// vier Dateioperationen und beim Pfadkopierer, ohne Ausnahme
    /// (Nutzerantwort vom 260811-1610).
    ///
    /// **Der Typ des Eintrags wird nicht geprueft.** Die Taste verzweigt nach
    /// der Nutzerantwort vom 260811-1505 ausdruecklich nicht; ein Ordner geht
    /// damit an das System und oeffnet sich im Finder.
    ///
    /// **Eine Rueckfrage entsteht in keinem Fall**, auch nicht bei fuenfzig
    /// markierten Eintraegen. Der Nutzer hat am 260811-1710 gegen eine Schwelle
    /// entschieden: eine Regel statt einer Zahl, die niemand gemessen hat. Der
    /// Preis ist benannt und angenommen — fuenfzig markierte Dateien starten
    /// fuenfzig Programme, und Rueckgaengig gibt es dafuer nicht.
    ///
    /// **Was die Meldung sagt, ist die Uebergabe und nicht das Oeffnen.**
    /// [`standardprogramm::oeffnen`] liefert, ob das System die Adresse
    /// angenommen hat; ob ein Programm danach startet, weiss KRK nicht. Der
    /// Unterschied steht im Kopf jenes Moduls, und
    /// [`operationen::oeffnungsmeldung`] haelt ihn im Wortlaut ein.
    fn mit_standardprogramm_oeffnen(&self, pfade: &[PathBuf]) {
        if pfade.is_empty() {
            self.befehlsantwort_zeigen(&operationen::nichts_zu_oeffnen());
            return;
        }
        let mut uebergeben: Vec<PathBuf> = Vec::new();
        let mut abgewiesen: Vec<PathBuf> = Vec::new();
        for pfad in pfade {
            if standardprogramm::oeffnen(pfad) {
                uebergeben.push(pfad.clone());
            } else {
                abgewiesen.push(pfad.clone());
            }
        }
        self.befehlsantwort_zeigen(&operationen::oeffnungsmeldung(&uebergeben, &abgewiesen));
    }

    /// Der Doppelklick auf eine Zeile des Dateifensters (C3).
    ///
    /// **Der Doppelklick verzweigt, die Taste nicht.** Das ist die
    /// Nutzerantwort vom 260811-1505
    /// (`decisions/260811-1259_*_was-tut-ein-doppelklick-auf-einen-ordner.md`)
    /// und der einzige Unterschied im Verhalten: auf einem Ordner steigt der
    /// Doppelklick in ihn ein, wie es ein Doppelklick auf dem Mac tut, und auf
    /// allem uebrigen gibt er den Eintrag an das Standardprogramm.
    /// [`Kommando::MitStandardprogrammOeffnen`] gibt auch einen Ordner an das
    /// System, und der Nutzer hat damit beide Wege.
    ///
    /// **Was ein Ordner ist, entscheidet dabei [`Self::in_zeile_einsteigen`]**
    /// und nicht dieser Rumpf; seit dem Defekt `260814-1612` zaehlt eine
    /// Verknuepfung auf einen Ordner mit. Eine dritte Antwort kommt von dort:
    /// eine Verknuepfung, deren Ziel sich nicht aufloesen laesst, ist gemeldet
    /// und geht nicht an das Standardprogramm.
    ///
    /// **Die zweite Ungleichheit ist die Menge.** Die Taste erbt
    /// [`Self::betroffene_eintraege`] und oeffnet alle betroffenen Eintraege;
    /// der Doppelklick nimmt genau die angeklickte Zeile. Eine Markierung
    /// anderswo geht ihn nichts an — wer bei dreissig markierten Eintraegen auf
    /// einen davon doppelklickt, bekommt diesen einen.
    ///
    /// **Eine zweite Umsetzung entsteht in keiner der beiden Richtungen.** Der
    /// Einstieg laeuft ueber [`Self::in_zeile_einsteigen`], denselben Rumpf,
    /// den der Rechts-Pfeil nimmt; das Oeffnen ueber
    /// [`Self::mit_standardprogramm_oeffnen`], dieselbe Methode, die die Taste
    /// ruft. Der Unterschied liegt allein in der Zeile und in der Menge, die
    /// dieser Weg uebergibt.
    ///
    /// **Warum hier geloescht wird, und nur an dieser Seite.** Ein Doppelklick
    /// ist kein Kommando und laeuft deshalb nicht durch
    /// `Anwendungsdelegierter::kommando_ausfuehren`, das die Antwort auf den
    /// vorigen Befehl sonst wegraeumt. Ohne diese Zeile stuende "7 Pfade
    /// kopiert" ueber dem Ordner, in den der Nutzer eben hineingeklickt hat.
    ///
    /// Es ist dieselbe Regel und keine zweite, angewandt auf eine Handlung mit
    /// engerer Reichweite: **geraeumt wird so weit, wie die Handlung reicht.**
    /// Ein Kommando reicht ueber beide Dateifenster — `Kopieren` schreibt in
    /// das unbeteiligte —, und der Delegierte raeumt darum beide Seiten
    /// (`for seite in Fensterseite::ALLE`, `appkit/anwendung.rs`). Der
    /// Doppelklick reicht ueber die eine angeklickte Zeile, und er raeumt darum
    /// die eine Statuszeile, an der er sitzt. Eine Befehlsantwort im anderen
    /// Dateifenster bleibt stehen, bis der naechste Tastenbefehl sie mitnimmt;
    /// sie ist dort weiterhin wahr, und kein Abnahmekriterium verlangt mehr.
    ///
    /// Der Fall ist geprueft und ausdruecklich so entschieden: der breitere Weg
    /// braeuchte einen weiteren Rueckruf von der Quelle zum
    /// Anwendungsdelegierten, also einen neuen Mechanismus fuer eine Zeile
    /// Anzeige
    /// (`issues/260811-1916_*_der-doppelklick-raeumt-die-befehlsantwort-nur-an-seiner-eigenen-fensterseite-weg.md`).
    ///
    /// **Der Rueckruf ist seit der Runde 13 gebaut, und der Entscheid steht
    /// trotzdem.** Der Abwurf aus C7 hat ihn bekommen, weil seine Meldung ohne
    /// ihn im nicht aktiven Dateifenster unsichtbar blieb — die Handlung reicht
    /// dort ueber beide Seiten. Beim Doppelklick tut sie das nicht, und das
    /// Argument war nie die Verfuegbarkeit des Mechanismus, sondern die
    /// Reichweite der Handlung. Wer ihn hier nachtraeglich anhaengt, weil es
    /// ihn jetzt gibt, hat die Begruendung mit ihrem Anlass verwechselt.
    ///
    /// Eine Zeile kleiner als null ist der Klick unter die letzte Zeile, also
    /// auf die leere Flaeche der Liste; er fuehrt zu nichts.
    fn doppelklick(&self, zeile: NSInteger) {
        self.befehlsantwort_loeschen();
        let Ok(zeile) = usize::try_from(zeile) else {
            return;
        };
        match self.in_zeile_einsteigen(zeile) {
            // Gemeldet ist gemeldet: eine Verknuepfung, deren Ziel fehlt, geht
            // nicht auch noch an das System, das an ihr ebenso scheiterte und
            // die eben geschriebene Statuszeile mit seiner eigenen Antwort
            // ueberschriebe.
            Einstieg::Eingestiegen | Einstieg::Gemeldet => return,
            Einstieg::KeinOrdner => {}
        }
        let Some((pfad, _)) = self.eintrag_in_zeile(zeile) else {
            return;
        };
        self.mit_standardprogramm_oeffnen(std::slice::from_ref(&pfad));
    }

    /// Ein getipptes Zeichen fuer den Filtertext des sichtbaren Tabs (C1.1).
    ///
    /// **Die eine Senke des Tippens.** Sie loeste zum 260814 die Sprungmarke
    /// aus C2 der Runde 1 ab: dasselbe Zeichen aus demselben Zweig des
    /// Anwendungsdelegierten, nur ein anderes Ziel. Der Filtertext gehoert dem
    /// `Ordnermodell` des sichtbaren Tabs und nicht mehr der Ansicht; damit
    /// gehoert er dem Tab, und ein Tabwechsel zeigt den des anderen Tabs
    /// (C1.8), ohne dass diese Stelle dafuer etwas tut.
    ///
    /// Liefert, ob KRK das Zeichen verbraucht hat — derselbe Rueckgabewert wie
    /// zuvor und dieselbe Zusage: nur ein nicht verbrauchter Tastendruck laeuft
    /// unveraendert an AppKit weiter. Ein Zeichen, das kein Dateiname tragen
    /// kann, ist deshalb nicht verbraucht.
    ///
    /// **Die Zeichenregel bleibt
    /// [`traegt_ein_dateiname`](krk_core::verzeichnis::filter::traegt_ein_dateiname)**
    /// (C1.4), dieselbe, die die Tippsuche der Belegungsansicht aus der Runde 7
    /// liest. Gefragt wird sie hier und nicht im Kern: `zeichen_anhaengen` hat
    /// keinen Rueckgabewert, und ein dort still verworfenes Zeichen waere ein
    /// Ausgang, den niemand sieht.
    ///
    /// **Findet der Filtertext nichts, gilt das Zeichen trotzdem als
    /// verbraucht.** Die Liste ist dann leer, und der naechste Rueckschritt
    /// gibt sie zurueck; ein Zeichen, das bei fehlendem Treffer an AppKit
    /// weiterliefe, machte den Filtertext von seinen Treffern abhaengig.
    pub fn filterzeichen_tippen(&self, zeichen: char) -> bool {
        if !traegt_ein_dateiname(zeichen) {
            return false;
        }
        {
            let mut tabs = self.ivars().tabs.borrow_mut();
            tabs.aktiver_mut().modell_mut().zeichen_anhaengen(zeichen);
        }
        self.nach_filteraenderung();
        true
    }

    /// Zieht die Ansicht nach, nachdem sich der Filtertext geaendert hat.
    ///
    /// Drei Schritte in dieser Reihenfolge: die neue Sicht anzeigen, die
    /// Auswahl des Modells in die Tabelle geben, und erst danach entscheiden,
    /// ob eine Ersatzzeile faellig ist. Die Reihenfolge traegt: die Frage aus
    /// C1.11 laesst sich nur an der **neuen** Sicht stellen, denn erst sie
    /// weiss, ob die Zeile der Auswahl weggefallen ist.
    ///
    /// **Der eine Weg der Anzeige nach einer Filteraenderung**, gerufen vom
    /// Tippen und vom Ruecknehmen eines Zeichens. Die Rechnung selbst steht
    /// als reine Funktion in [`crate::kommandos::navigation`] neben
    /// `zielzeile`, damit sie ohne `NSTableView` zu pruefen ist; hier bleibt
    /// allein, was AppKit betrifft.
    ///
    /// **Die Ersatzzeile geht ueber [`Self::zeile_setzen`]** und nicht ueber
    /// `zeile_auswaehlen` daneben: die Auswahl muss auch im Modell stehen,
    /// sonst zeigte der naechste Aufbau der Sicht wieder die alte.
    ///
    /// **Der Durchlauf wird hier nachgezogen** (C3.6). Jede Aenderung des
    /// Filtertexts bricht den laufenden ab und stoesst, wenn „Deep" steht,
    /// einen neuen an; die Regel dafuer steht in
    /// [`Tabliste::durchlauf_nachziehen`](crate::tabs::Tabliste::durchlauf_nachziehen)
    /// und nicht hier. Diese Stelle ist der eine Weg jeder Filteraenderung,
    /// also auch der eine Ort dieses Rufs — die drei Aufrufer daneben je
    /// einzeln rufen zu lassen waeren drei Gelegenheiten, es verschieden zu
    /// tun.
    ///
    /// **Der fuenfte Rang der Statuszeile wird hier nachgezogen** (C4). Er
    /// nennt den Filtertext und die Zahl der gezeigten Zeilen, und beide
    /// aendert genau diese Stelle; sie ist der eine Weg der Anzeige nach einer
    /// Filteraenderung, also ist sie auch der eine Ort dieses Rufs. Gezeichnet
    /// werden muss dafuer wie beim Markierungsstand daneben, denn ein
    /// gerechneter Rang hat kein Feld, das jemand setzt.
    fn nach_filteraenderung(&self) {
        self.durchlauf_nachziehen();
        self.umsortiert();
        self.meldung_gewechselt();
        let (hatte_auswahl, zeile_jetzt, zeilen) = {
            let tabs = self.ivars().tabs.borrow();
            let modell = tabs.aktiver().modell();
            (
                modell.auswahl().is_some(),
                modell.auswahl_zeile(),
                modell.zeilenzahl(),
            )
        };
        if let Some(zeile) = ersatzzeile(hatte_auswahl, zeile_jetzt, zeilen) {
            self.zeile_setzen(zeile);
        }
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
    /// aus C2, die Ersatzzeile des Filters aus C1.11 und das Markieren mit
    /// Weiterruecken enden alle hier.
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
        let pfad = {
            let mut tabs = self.ivars().tabs.borrow_mut();
            let ordner = tabs.aktiver().ordner().to_path_buf();
            let modell = tabs.aktiver_mut().modell_mut();
            let eintrag = zeile.and_then(|zeile| modell.eintragsindex(zeile));
            modell.auswahl_setzen(eintrag);
            eintrag
                .and_then(|eintrag| modell.eintraege().get(eintrag as usize))
                .map(|eintrag| ordner.join(&eintrag.name))
        };
        // Nach dem Ende der Ausleihe: der Melder fuellt die Vorschau aus C6,
        // und die gehoert einem anderen Halter.
        let melden = self.ivars().auswahlmelder.borrow();
        if let Some(melden) = melden.as_ref() {
            melden(pfad);
        }
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
    /// symbolischen Verknuepfung folgt KRK hier seit dem Defekt `260814-1612`
    /// dagegen schon: zeigt sie auf einen Ordner, geht der Einstieg hinein. Wo
    /// das entschieden wird, steht bei [`Self::in_zeile_einsteigen`].
    ///
    /// Der Rechts-Pfeil ist seit C3 der Runde 4 nicht mehr der einzige
    /// Einstieg; der Doppelklick nimmt denselben Weg. Was diese Funktion
    /// beisteuert, ist deshalb allein die **Zeile**: sie fragt die Auswahl,
    /// [`Self::doppelklick`] fragt die angeklickte Zeile, und der Rumpf steht
    /// beiden Wegen in [`Self::in_zeile_einsteigen`] gemeinsam zur Verfuegung.
    fn auswahl_oeffnen(&self) {
        let Ok(zeile) = usize::try_from(self.ivars().tabelle.selectedRow()) else {
            return;
        };
        // Der Rueckgabewert interessiert hier nicht: auf einer Datei loest der
        // Rechts-Pfeil unveraendert nichts aus, und eine unerreichbare
        // Verknuepfung hat ihre Meldung schon gesetzt.
        let _ = self.in_zeile_einsteigen(zeile);
    }

    /// Steigt in den Ordner dieser Zeile hinein und meldet, ob es einer war.
    ///
    /// **Der eine Absteiger im Baum**, und der Grund, aus dem es ihn getrennt
    /// von [`Self::auswahl_oeffnen`] gibt: seine beiden Aufrufer beantworten
    /// die Frage "welche Zeile" verschieden. Der Rechts-Pfeil nimmt
    /// `selectedRow`, der Doppelklick `clickedRow`. Alles danach ist dasselbe,
    /// und deshalb steht es hier und nicht zweimal.
    ///
    /// [`Einstieg::KeinOrdner`] heisst "diese Zeile war kein Ordner" **und**
    /// "diese Zeile gibt es nicht"; beides fuehrt zu keinem Einstieg, und ein
    /// Aufrufer, der die Faelle trennen wollte, fragte danach ohnehin
    /// [`Self::eintrag_in_zeile`].
    ///
    /// # Eine Verknuepfung wird hier aufgeloest und nicht beim Lesen
    ///
    /// Der Verzeichnisleser folgt keiner Verknuepfung: er meldet sie als
    /// [`Typ::Verknuepfung`], gleichgueltig worauf sie zeigt, und `ist_ordner`
    /// antwortet fuer sie deshalb mit `false`. Bis zum Defekt `260814-1612`
    /// endete der Einstieg damit, und eine Verknuepfung auf einen Ordner liess
    /// sich nicht betreten.
    ///
    /// Aufgeloest wird sie **allein an dieser Stelle**, ueber
    /// [`verweisziel::bestimmen`] am Namen, und **nur fuer eine
    /// Verknuepfung**. Der Lesevorgang bekommt dafuer keinen zusaetzlichen
    /// Systemaufruf: an seiner Rechnung haengen die Zeitzusagen L3 und L10, und
    /// ein `stat` je Verknuepfung bei jeder Anzeige aenderte sie. Der eine
    /// Aufruf faellt an, wenn der Nutzer tatsaechlich hineingeht.
    ///
    /// **Die anderen Rufer von `Eintrag::ist_ordner` bleiben, wie sie sind**,
    /// weil sie eine andere Frage stellen: `kommandos::operationen` zaehlt
    /// Ordner einer Auswahl fuer eine Dateioperation, und dort ist die
    /// Verknuepfung selbst das Ziel und nicht ihr Verweisziel. Dieselbe
    /// Trennung, die `text::datei::lesen` und `pfadeingabe::pruefen` schon
    /// ziehen: als Ziel eines Sprungs gilt das Verweisziel, in der Liste die
    /// Verknuepfung.
    fn in_zeile_einsteigen(&self, zeile: usize) -> Einstieg {
        let Some((ziel, typ)) = self.eintrag_in_zeile(zeile) else {
            return Einstieg::KeinOrdner;
        };
        let ist_ordner = match typ {
            Typ::Ordner => true,
            // Eine Datei bleibt eine Datei: der Doppelklick gibt sie an das
            // System, der Rechts-Pfeil tut nichts. Kein Systemaufruf.
            Typ::Datei => false,
            Typ::Verknuepfung => match verweisziel::bestimmen(&ziel) {
                Verweisziel::Ordner => true,
                // Zeigt sie auf eine Datei, geschieht, was bei einer Datei
                // geschieht: der Aufrufer reicht sie an das System, und das
                // loest sie seinerseits auf.
                Verweisziel::KeinOrdner => false,
                // Ins Leere, im Ring oder ohne Recht: nicht still verschlucken.
                // Die Statuszeile aus C1 ist die eine Meldeflaeche dafuer,
                // dieselbe, die "die Zwischenablage ist leer" traegt.
                Verweisziel::Unerreichbar { grund } => {
                    self.befehlsantwort_zeigen(&format!(
                        "{} lässt sich nicht öffnen: {grund}",
                        ziel.display()
                    ));
                    return Einstieg::Gemeldet;
                }
            },
        };
        if !ist_ordner {
            return Einstieg::KeinOrdner;
        }
        self.ordner_lesen(&ziel, None);
        Einstieg::Eingestiegen
    }

    /// Der volle Pfad des Eintrags in dieser Zeile und seine Art.
    ///
    /// Die eine Stelle dieses Weges, die eine Zeilennummer in einen Pfad
    /// uebersetzt. Die Pfadarithmetik `ordner.join(name)` steht daneben nur
    /// noch in [`operationen::betroffene`], und die beantwortet eine andere
    /// Frage: nicht "welche Zeile", sondern "welche Eintraege sind betroffen".
    ///
    /// Geliefert wird der [`Typ`] und nicht mehr die Antwort von `ist_ordner`:
    /// [`Self::in_zeile_einsteigen`] behandelt die drei Arten verschieden, und
    /// ein Wahrheitswert warf zwei von ihnen zusammen.
    ///
    /// Die Ausleihe des Tabmodells endet mit der Rueckgabe: der Pfad ist
    /// eigener Besitz, und der Aufrufer darf danach AppKit rufen.
    fn eintrag_in_zeile(&self, zeile: usize) -> Option<(PathBuf, Typ)> {
        let tabs = self.ivars().tabs.borrow();
        let tab = tabs.aktiver();
        tab.modell()
            .zeile(zeile)
            .map(|eintrag| (tab.ordner().join(&eintrag.name), eintrag.typ))
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
    /// **Die AppKit-Seite der einen Stelle, die einen Namen zur Auswahl
    /// macht.** Entschieden wird in [`Tabliste::auswahl_auf_namen`], und dort
    /// allein: dass ein laufender Lesevorgang den Namen vormerkt, statt eine
    /// Zeile zu waehlen, steht in dieser einen Methode. Hier bleibt, was ein
    /// Fenster braucht — die Zeile in der `NSTableView` setzen, sie ins Bild
    /// blaettern und die Vorschau aus C6 melden, alles ueber
    /// [`DateifensterQuelle::zeile_setzen`].
    ///
    /// Der Rueckgabewert sagt, welcher der drei Faelle eingetreten ist. Allein
    /// [`Auswahlversuch::Unbekannt`] ist eine Auskunft an den Nutzer wert: der
    /// Name steht in einer fertig gelesenen Liste nicht.
    ///
    /// **Fallenlassen geht nur ausdruecklich.** [`Auswahlversuch`] traegt ein
    /// `#[must_use]`; wer die Auskunft nicht braucht, schreibt `let _ =` davor
    /// und begruendet es. Die Regel steht am Doc-Kommentar der Aufzaehlung.
    pub fn eintrag_waehlen(&self, name: &str) -> Auswahlversuch {
        let versuch = self.ivars().tabs.borrow_mut().auswahl_auf_namen(name);
        if let Auswahlversuch::Gewaehlt(zeile) = versuch {
            self.zeile_setzen(zeile);
        }
        versuch
    }

    /// Alle Namen des angezeigten Ordners, auch die ausgeblendeten.
    ///
    /// Der Bestand, gegen den die Kollisionspruefung des Stapel-Umbenennens
    /// vergleicht (C4). Ausdruecklich nicht die Sichtreihenfolge: ein
    /// ausgeblendeter Eintrag belegt seinen Namen genauso wie ein sichtbarer,
    /// und eine Pruefung, die ihn uebersaehe, liesse das Umbenennen erst im
    /// Dateisystem scheitern.
    ///
    /// **Waehrend eines laufenden Lesevorgangs ist die Antwort der Bestand des
    /// vorigen Laufs** — desselben Ordners, einen Augenblick alt. Das ist
    /// gewollt und keine dritte Sonderfallzeile: die Vorschau des
    /// Stapel-Umbenennens ist eine Hilfe und nicht die Wahrheit ueber vergebene
    /// Namen; die haelt das Dateisystem, wie es bei
    /// [`crate::appkit::anwendung`]s `umbenennen_ausfuehren` ausgeschrieben
    /// steht. Bis zum 260807 war die Antwort in dieser Spanne eine **leere**
    /// Liste, weil eine Auffrischung den Bestand vorab wegwarf; ein alter
    /// Bestand ist dagegen die bessere Naeherung.
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
        self.meldung_gewechselt();
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
    /// Eintrag steht. Der Wert entscheidet ueber den Nachzug der Aufteilung und
    /// die vorgemerkte Sitzung und nicht darueber, ob der Tastendruck
    /// weiterlaeuft: `Anwendungsdelegierter::kommando_ausfuehren` schluckt seit
    /// der Runde 7 jeden zulaessigen Befehl.
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
    /// **Gerufen aus der Aktion des Feldes, und die kommt allein von Return.**
    /// Die drei Ausgaenge und wo jeder landet:
    ///
    /// - **Return** schickt die Aktion und landet hier.
    /// - **Escape** laeuft ueber `abortEditing` und landet bei
    ///   [`Namensfeld::bearbeitung_abbrechen`].
    /// - **Jedes uebrige Ende** — der Klick daneben, der Fokuswechsel, ein
    ///   Zeichendurchgang der Tabelle — schickt **keine** Aktion und landet bei
    ///   [`Namensfeld::bearbeitung_beendet`]. Ein Klick neben die Zelle benennt
    ///   damit nichts um.
    ///
    /// Am 260816 am wirklichen Hauptfaden gemessen; die Tabelle der Enden steht
    /// im Kopf von [`Namensfeld`]. Bis zum 260816 stand hier "wenn der Nutzer
    /// die Eingabe mit Return abschliesst oder die Zelle verlaesst", und die
    /// zweite Haelfte davon war falsch.
    ///
    /// # Woher der Satz ueber die drei Ausgaenge stammt
    ///
    /// Das Abnahmekriterium von C4 verlangt allein "ein Tastenbefehl benennt
    /// den ausgewaehlten Eintrag um, direkt in der Liste"
    /// (`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md:254`).
    /// Der Satz "Return uebernimmt, Escape verwirft" stammt aus dem **Plan**
    /// derselben Runde und nicht aus dem Spec; bis zum 260816 stand er hier als
    /// Zitat aus C4 und schrieb dem Spec damit eine Zusage zu, die er nicht
    /// traegt.
    ///
    /// Den dritten Ausgang, das Ende ohne Return, hat der Nutzer am 260816-0935
    /// entschieden: er **verwirft wie Escape**
    /// (`shared/decisions/260816-0021_*_verwirft-oder-uebernimmt-ein-klick-neben-die-offene-namenszelle.md`).
    /// Zu bauen war daran allein die Anzeige, denn umbenannt wird ohne Aktion
    /// ohnehin nichts; hergestellt wird sie ueber
    /// [`Self::anzeigeform_herstellen`] an beiden Enden, die AppKit hat.
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

    /// Holt die Anzeigeform der Namenszelle aus dem Modell zurueck (C4).
    ///
    /// **Die eine Zusage des Nutzerentscheids vom 260816-0935:** jedes Ende
    /// einer Bearbeitung, dem keine Umbenennung folgt, stellt die Anzeigeform
    /// wieder her. Deshalb hat diese Methode beide Enden als Rufer, die AppKit
    /// hat — Escape ueber [`Namensfeld::bearbeitung_abbrechen`], jedes uebrige
    /// Ende ueber [`Namensfeld::bearbeitung_beendet`]. Zurueckzuholen ist der
    /// getippte Text, der sonst stehen bliebe und eine Umbenennung behauptete,
    /// die nicht stattgefunden hat, und mit ihm das Ordnerzeichen, das
    /// [`Namensfeld::wird_ersthelfer`] fuer die Dauer der Bearbeitung abgelegt
    /// hat.
    ///
    /// **Gefragt wird nicht, ob eine Umbenennung folgte, und das ist gemessen
    /// und nicht angenommen.** Nach Return hat die Aktion die Zeile schon
    /// gezeichnet, und dieser zweite Durchgang findet seine Zeile und ist
    /// folgenlos; hat die Umbenennung eine Auffrischung ausgeloest, liefert
    /// `rowForView:` dem Feld gar keine Zeile mehr, und der Durchgang faellt
    /// still aus, waehrend der Zeichendurchgang der Auffrischung die
    /// Anzeigeform ohnehin schon geholt hat. Beide Faelle sind am 260816 auf
    /// macOS 15.7.7 am wirklichen Hauptfaden gemessen
    /// (`shared/history/260816-1017-coder-anzeigeform-an-jedem-ende-ohne-umbenennung.md`,
    /// Messungen C, F und G). Eine Fallunterscheidung "kam eine Aktion?" waere
    /// damit eine Regel ohne Wirkung.
    ///
    /// Die Zeile kommt ueber `rowForView:` und nicht aus einem gemerkten
    /// Zustand, aus demselben Grund wie bei [`Self::umbenennung_beenden`].
    fn anzeigeform_herstellen(&self, feld: &NSTextField) {
        let zeile = self.ivars().tabelle.rowForView(feld);
        let Ok(zeile) = usize::try_from(zeile) else {
            return;
        };
        self.zeile_neu_zeichnen(zeile);
    }

    /// Eine Namenszelle dieses Dateifensters steht jetzt in Bearbeitung (C4).
    fn namensbearbeitung_begonnen(&self) {
        self.ivars().namensbearbeitung.set(true);
    }

    /// Die Bearbeitung ist zu Ende; das Dateifenster liest wieder (C4).
    ///
    /// **Getrennt vom Nachholen darunter, und der Grund ist die Reihenfolge.**
    /// Auf dem Return-Weg schickt `NSTextField` die Aktion aus
    /// `textDidEndEditing:` heraus; das Kennzeichen muss davor fallen, sonst
    /// schoebe die Auffrischung, die die Umbenennung selbst ausloest, sich
    /// gleich wieder auf, und `eintrag_waehlen` fande keinen laufenden
    /// Lesevorgang mehr, an den es seinen Wunsch haengen koennte. Der
    /// Zeichendurchgang des Nachholens darf umgekehrt erst **nach** der Aktion
    /// laufen, weil er dem Feld seine Zeile nimmt.
    fn namensbearbeitung_beendet(&self) {
        self.ivars().namensbearbeitung.set(false);
    }

    /// Ob in diesem Dateifenster gerade eine Namenszelle bearbeitet wird (C4).
    pub fn namenszelle_in_bearbeitung(&self) -> bool {
        self.ivars().namensbearbeitung.get()
    }

    /// Merkt vor, dass eine Auffrischung wegen der offenen Zelle ausfiel (C4).
    ///
    /// Gerufen von [`crate::auffrischung::ordner_neu_lesen`] anstelle von
    /// [`Self::neu_lesen`]. Mehrere ausgefallene Auffrischungen ergeben ein
    /// Nachholen und nicht mehrere: gelesen wird der Ordner, wie er beim
    /// Nachholen dasteht, und nicht jede Zwischenstufe.
    pub fn auffrischung_vormerken(&self) {
        self.ivars().auffrischung_vorgemerkt.set(true);
    }

    /// Holt eine Auffrischung nach, die die offene Namenszelle aufgehalten hat
    /// (C4).
    ///
    /// Gerufen an **jedem** Ende einer Bearbeitung, also nach Return, nach
    /// Escape und nach dem Klick daneben. Steht nichts vorgemerkt, geschieht
    /// nichts — der gewoehnliche Fall, denn zwischen dem Beginn und dem Ende
    /// einer Umbenennung schreibt meist niemand in den Ordner.
    fn aufgeschobene_auffrischung_nachholen(&self) {
        if self.ivars().auffrischung_vorgemerkt.get() {
            self.neu_lesen();
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
        self.meldung_gewechselt();
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
    ///
    /// **Seit dem 260816 zieht dieser Schalter den Durchlauf nach**, in
    /// derselben Bauart wie [`Self::tiefe_suche_umschalten`] und
    /// [`Self::inhaltssuche_umschalten`] darunter. Der Grund ist, dass er seit
    /// diesem Tag eine Eingabe der Auftragsliste aendert: ein ausgeblendeter
    /// Eintrag bekommt keinen Auftrag mehr, weil seine Zeile ohnehin nicht
    /// stehen kann, und ein eben eingeblendeter braucht deshalb einen, den er
    /// vorher nicht hatte. Bis dahin bekam **jeder** Eintrag seinen Auftrag,
    /// auch der unsichtbare; das kostete bei ausgeschaltetem „Content" einen
    /// Metadatengang und seit der Runde 11 ein `open(2)` samt bis zu 1 MB
    /// gelesener Bytes je verstecktem Eintrag
    /// (`issues/260816-1931_*_der-inhaltsfilter-liest-versteckte-dateien-und-steigt-in-versteckte-ordner-ab.md`).
    ///
    /// Der Handel ist damit umgedreht: wer nie einblendet, zahlt nichts mehr;
    /// wer einblendet, zahlt einen neuen Lauf. Das Anstossen und Abbrechen
    /// selbst gehoert `Tabliste::durchlauf_nachziehen`, und hier faellt dafuer
    /// kein Zweig an.
    fn verstecke_umschalten(&self) {
        {
            let mut tabs = self.ivars().tabs.borrow_mut();
            tabs.aktiver_mut().modell_mut().verstecke_umschalten();
        }
        self.durchlauf_nachziehen();
        self.umsortiert();
        // Der Schalter aendert, wie viele Zeilen stehen, und ob ein Lauf
        // laeuft; beides schreibt die Statuszeile.
        self.meldung_gewechselt();
    }

    /// Kippt das Kennzeichen "Deep" am Modell des sichtbaren Tabs (C2 und C5
    /// der Filter-Runde).
    ///
    /// **Der eine Schreiber des Kennzeichens**, nach dem Vorbild von
    /// [`Self::verstecke_umschalten`] darueber: beide aendern, was die Liste
    /// zeigt, und beide lassen danach [`Self::umsortiert`] die neue Sicht
    /// anzeigen. Das Neuaufbauen der Sicht selbst gehoert
    /// `Ordnermodell::tief_setzen` und nicht dieser Stelle.
    ///
    /// **Steht kein Filtertext, aendert das Kippen an der Liste nichts**, und
    /// gemeldet wird nichts (C2.4). Der Schalter steht trotzdem um: sein Stand
    /// ist eine Einstellung des Tabs und keine Auskunft darueber, ob der Filter
    /// gerade etwas findet.
    ///
    /// Der Aufrufer ist der Anwendungsdelegierte und nicht
    /// [`Self::kommando_ausfuehren`] daneben: der Befehl traegt
    /// `Wirkungsbereich::Ueberall` und richtet sich an das **aktive**
    /// Dateifenster, nicht an das fokussierte.
    pub fn tiefe_suche_umschalten(&self) {
        {
            let mut tabs = self.ivars().tabs.borrow_mut();
            let modell = tabs.aktiver_mut().modell_mut();
            let tief = modell.tief();
            modell.tief_setzen(!tief);
        }
        // Einschalten stoesst den Durchlauf an, Ausschalten bricht ihn ab
        // (C3.7). Beide Haelften stehen in einer Regel, und die Regel steht in
        // `Tabliste::durchlauf_nachziehen`; hier faellt kein Zweig an.
        self.durchlauf_nachziehen();
        self.umsortiert();
        // Der Schalter aendert, wie viele Zeilen stehen, und damit den fuenften
        // Rang der Statuszeile (C4.3).
        self.meldung_gewechselt();
    }

    /// Ob am Modell des sichtbaren Tabs das Kennzeichen "Deep" steht (C2.1).
    ///
    /// **Die Leseseite von [`Self::tiefe_suche_umschalten`]**, in derselben
    /// Bauart wie [`Self::filter_steht`] darunter: eine Ausleihe, eine Frage an
    /// das `Ordnermodell` des sichtbaren Tabs, kein zweiter Halteort. Gefragt
    /// wird sie von `Anwendungsdelegierter::bereichsleiste_nachziehen`, dem
    /// einen Schreiber des angezeigten Schalterstands; die Leiste selbst haelt
    /// den Wert nicht.
    ///
    /// **Sie sagt nichts darueber, ob der Schalter gerade etwas bewirkt**
    /// (C2.4). Ohne Filtertext aendert "Deep" an der Liste nichts, und das
    /// Kaestchen steht trotzdem so, wie der Nutzer es gesetzt hat: sein Stand
    /// ist eine Einstellung des Tabs und keine Auskunft ueber Treffer.
    ///
    /// **Der Aufrufer waehlt das Dateifenster**, und er waehlt das aktive und
    /// nicht das fokussierte — dieselbe Adresse, an die
    /// [`Self::tiefe_suche_umschalten`] schreibt. Zwei verschiedene Adressen
    /// fuer Schreiben und Lesen zeigten einen Stand, den der Klick nicht
    /// gekippt hat.
    pub fn tiefe_suche_steht(&self) -> bool {
        let tabs = self.ivars().tabs.borrow();
        tabs.aktiver().modell().tief()
    }

    /// Kippt das Kennzeichen "Content" am Modell des sichtbaren Tabs (C2 der
    /// Inhaltsfilter-Runde).
    ///
    /// **Zeile fuer Zeile die Bauart von [`Self::tiefe_suche_umschalten`]**
    /// darueber: eine Ausleihe, das Kippen am Modell, danach
    /// [`Self::durchlauf_nachziehen`] fuer den Lauf, [`Self::umsortiert`] fuer
    /// die Sicht und [`Self::meldung_gewechselt`] fuer die Statuszeile. Das
    /// Neuaufbauen der Sicht gehoert `Ordnermodell::inhalt_setzen` und nicht
    /// dieser Stelle; das Anstossen und Abbrechen des Laufs gehoert
    /// `Tabliste::durchlauf_nachziehen`, und hier faellt dafuer kein Zweig an.
    ///
    /// **Ist der Filtertext zu kurz, aendert das Kippen an der Liste nichts**,
    /// und gemeldet wird nichts. Der Schalter steht trotzdem um: sein Stand ist
    /// eine Einstellung des Tabs und keine Auskunft darueber, ob der Filter
    /// gerade Inhalte liest. Ob er wirkt, beantwortet
    /// `Ordnermodell::inhalt_wirkt` an einer Stelle und nicht dieser Rumpf.
    ///
    /// Der Aufrufer ist der Anwendungsdelegierte und nicht
    /// [`Self::kommando_ausfuehren`] daneben: der Befehl traegt
    /// `Wirkungsbereich::Ueberall` und richtet sich an das **aktive**
    /// Dateifenster, nicht an das fokussierte.
    pub fn inhaltssuche_umschalten(&self) {
        {
            let mut tabs = self.ivars().tabs.borrow_mut();
            let modell = tabs.aktiver_mut().modell_mut();
            let inhalt = modell.inhalt();
            modell.inhalt_setzen(!inhalt);
        }
        // Einschalten stoesst den Durchlauf an, Ausschalten bricht ihn ab.
        // Beide Haelften stehen in einer Regel, und die Regel steht in
        // `Tabliste::durchlauf_nachziehen`; hier faellt kein Zweig an.
        self.durchlauf_nachziehen();
        self.umsortiert();
        // Der Schalter aendert, wie viele Zeilen stehen, und ob der Lesehinweis
        // faellig ist; beides schreibt die Statuszeile.
        self.meldung_gewechselt();
    }

    /// Ob am Modell des sichtbaren Tabs das Kennzeichen "Content" steht (C2.1).
    ///
    /// **Die Leseseite von [`Self::inhaltssuche_umschalten`]**, in derselben
    /// Bauart wie [`Self::tiefe_suche_steht`] darueber: eine Ausleihe, eine
    /// Frage an das `Ordnermodell` des sichtbaren Tabs, kein zweiter Halteort.
    /// Gefragt wird sie von `Anwendungsdelegierter::bereichsleiste_nachziehen`,
    /// dem einen Schreiber des angezeigten Schalterstands.
    ///
    /// **Sie sagt nichts darueber, ob der Schalter gerade etwas bewirkt.** Bei
    /// zu kurzem Filtertext liest KRK keine Inhalte, und das Kaestchen steht
    /// trotzdem so, wie der Nutzer es gesetzt hat. Die Frage nach der Wirkung
    /// beantwortet `Ordnermodell::inhalt_wirkt`, und sie hat einen anderen
    /// Frager.
    ///
    /// **Der Aufrufer waehlt das Dateifenster**, und er waehlt das aktive und
    /// nicht das fokussierte — dieselbe Adresse, an die
    /// [`Self::inhaltssuche_umschalten`] schreibt.
    pub fn inhaltssuche_steht(&self) -> bool {
        let tabs = self.ivars().tabs.borrow();
        tabs.aktiver().modell().inhalt()
    }

    /// Ob das Modell des sichtbaren Tabs einen Filtertext fuehrt.
    ///
    /// **Die eine Groesse, an der die Rueckschritt-Taste ihre Bedeutung
    /// entscheidet** (C1.14 bis C1.16). Gefragt wird sie vom
    /// Anwendungsdelegierten, der die Regel in
    /// [`crate::kommandos::rueckschritt`] stellt; die Antwort steht am Modell
    /// und wird nicht daneben ein zweites Mal gehalten.
    ///
    /// **Ob der Filtertext Treffer hat, sagt sie nicht**, und das ist Absicht:
    /// ein Filtertext ohne Treffer schuetzt genauso wie einer mit Treffern
    /// (C6.10).
    pub fn filter_steht(&self) -> bool {
        let tabs = self.ivars().tabs.borrow();
        tabs.aktiver().modell().filter_steht()
    }

    /// Nimmt das letzte Zeichen des Filtertexts zurueck und zeigt die neue
    /// Sicht (C1.14).
    ///
    /// In der Bauart von [`Self::tiefe_suche_umschalten`] darueber: Ausleihe,
    /// Aenderung am Modell, danach [`Self::nach_filteraenderung`] fuer die
    /// Anzeige — dieselbe Stelle, die auch das Tippen nachzieht, damit jede
    /// Aenderung des Filtertexts denselben Weg nimmt. Die
    /// Liste waechst dabei um die Eintraege, die mit dem kuerzeren Filtertext
    /// wieder passen; das Neuaufbauen der Sicht gehoert
    /// `Ordnermodell::letztes_zeichen_weg` und nicht dieser Stelle.
    ///
    /// **Ohne Rueckgabewert, und der Grund gehoert dazu.** Der Wert von
    /// `Ordnermodell::letztes_zeichen_weg` traegt `#[must_use]` und wird hier
    /// verbraucht: er entscheidet, ob die Anzeige nachzuziehen ist. Fuer den
    /// Aufrufer bleibt nichts zu entscheiden — er hat die Frage „steht ein
    /// Filtertext" ueber [`Self::filter_steht`] schon gestellt, und die Regel
    /// hat auf ihr geantwortet. Ein zweiter Wahrheitswert zurueck waere eine
    /// zweite Gelegenheit, dieselbe Frage anders zu beantworten.
    ///
    /// **Weder Auswahl noch Markierung werden angefasst** (C6.9). Die Auswahl
    /// haengt am Eintragsindex und wandert mit; [`Self::nach_filteraenderung`]
    /// zeigt sie nur neu an. Der Ersatzzweig aus C1.11 kann hier nicht
    /// greifen — ein Zeichen weniger nimmt der Sicht keine Zeile —, und er
    /// steht trotzdem im Weg, weil zwei Wege fuer dieselbe Aenderung zwei
    /// Gelegenheiten waeren, auseinanderzulaufen.
    pub fn letztes_filterzeichen_weg(&self) {
        let weggenommen = {
            let mut tabs = self.ivars().tabs.borrow_mut();
            tabs.aktiver_mut().modell_mut().letztes_zeichen_weg()
        };
        if weggenommen {
            self.nach_filteraenderung();
        }
    }

    /// Loescht den ganzen Filtertext und zeigt die neue Sicht (C1.7).
    ///
    /// Der Rumpf des dritten und letzten Rangs von `esc`; die Reihenfolge der
    /// Raenge steht beim Aufrufer in `Anwendungsdelegierter::abbrechen` und
    /// nicht hier. In der Bauart von [`Self::letztes_filterzeichen_weg`]
    /// darueber, mit demselben Nachzug ueber [`Self::nach_filteraenderung`]:
    /// jede Aenderung des Filtertexts nimmt denselben Weg in die Anzeige.
    ///
    /// **Liefert, ob etwas zu loeschen war.** Der Wert entscheidet beim
    /// Aufrufer allein darueber, ob `esc` als gewirkt gilt; ohne ihn muesste er
    /// die Frage "steht ein Filtertext" ein zweites Mal stellen, und zwei
    /// Frager an derselben Groesse waeren zwei Gelegenheiten, verschieden zu
    /// antworten.
    ///
    /// **Der Filter der Tiefe bleibt stehen** (C3.5): `esc` loescht den
    /// Gegenstand des Durchlaufs und beendet ihn damit, legt aber keinen
    /// Schalter um, den der Nutzer gesetzt hat. Ein Schalter, den eine Taste
    /// unbemerkt umlegte, waere eine zweite Quelle fuer seinen Stand neben der
    /// Bereichsleiste.
    #[must_use]
    pub fn filter_leeren(&self) -> bool {
        {
            let mut tabs = self.ivars().tabs.borrow_mut();
            let modell = tabs.aktiver_mut().modell_mut();
            if !modell.filter_steht() {
                return false;
            }
            modell.filter_leeren();
        }
        self.nach_filteraenderung();
        true
    }

    /// Zieht den Durchlauf des sichtbaren Tabs nach und wirft den Einzugstakt
    /// an, falls jetzt einer laeuft.
    ///
    /// **Der ganze AppKit-Anteil des Durchlaufs an dieser Stelle**: die Regel,
    /// wann einer faellt und wann einer beginnt, steht in
    /// [`Tabliste::durchlauf_nachziehen`](crate::tabs::Tabliste::durchlauf_nachziehen).
    /// Hier bleibt allein der Zeitgeber, denn der ist AppKit: ohne ihn liefe
    /// der Arbeitsfaden, und seine Befunde blieben im Kanal stehen, bis der
    /// naechste Lesevorgang den Takt zufaellig wieder anwirft.
    ///
    /// Der Wert von `durchlauf_nachziehen` traegt `#[must_use]` und wird hier
    /// verbraucht; fuer den Aufrufer bleibt nichts zu entscheiden.
    fn durchlauf_nachziehen(&self) {
        let laeuft = self.ivars().tabs.borrow_mut().durchlauf_nachziehen();
        if laeuft {
            self.einzug_starten();
        }
    }

    /// Nach einem Wechsel der Reihenfolge oder der Sichtbarkeit.
    ///
    /// Die Auswahl haengt am Eintrag und nicht an der Zeile; sie wandert
    /// deshalb mit und wird hier nur neu angezeigt.
    ///
    /// **Faellt die Zeile der Auswahl weg, bleibt die Auswahl hier leer.** Das
    /// ist das Verhalten, das das Ausblenden der Verstecke seit der Runde 1
    /// zeigt, und diese Stelle aendert es nicht. Der Filter braucht eine
    /// andere Antwort; sie steht in [`Self::nach_filteraenderung`].
    fn umsortiert(&self) {
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

    /// Ob der Eintrag dieser Zeile allein wegen seines Inhalts dasteht (C5).
    ///
    /// **Die Regel wird hier nicht nachgebaut.** Sie steht als
    /// [`krk_core::verzeichnis::Ordnermodell::steht_wegen_des_inhalts`] im Kern
    /// und traegt ihre Vorbedingungen selbst; diese Methode rechnet die Zeile
    /// in den Eintragsindex um und fragt, genau wie [`Self::zeile_markiert`]
    /// daneben. Eine zweite Fassung in `krk-ui` waere die zweite Antwort auf
    /// dieselbe Frage und liefe irgendwann von der ersten weg.
    fn zeile_steht_wegen_des_inhalts(&self, zeile: usize) -> bool {
        let tabs = self.ivars().tabs.borrow();
        let modell = tabs.aktiver().modell();
        modell
            .eintragsindex(zeile)
            .is_some_and(|index| modell.steht_wegen_des_inhalts(index))
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
    /// (`issues/260804-1040_*_die-bildlaufposition-in-der-session-toml-steht-am-oberen-rand-auf-minus-28.md`).
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

    /// Meldet, dass eine der sechs Quellen dieses Dateifensters sich geaendert
    /// hat.
    ///
    /// **Sie schreibt nichts und entscheidet nichts.** Bis zur Runde 6 hiess
    /// diese Methode `meldung_anzeigen` und setzte die eigene Zeile am Fuss
    /// dieses Dateifensters; seit es eine Zeile fuer beide gibt, kann ein
    /// einzelnes Dateifenster die Frage gar nicht mehr beantworten — was in
    /// der Zeile steht, haengt an den Quellen **beider** Seiten und an der
    /// aktiven. Der Ruf geht deshalb an den einen, der beides sieht.
    ///
    /// Die Ausleihe steht waehrend des Rufs, wie bei
    /// [`Self::ordnerwechsel_melden`]. Sie ist lesend, und der einzige
    /// schreibende Zugriff auf dieselbe Zelle ist
    /// [`Self::meldungswechsel_setzen`] beim Aufbau; der Rueckruf holt sich
    /// gleich darauf [`Self::meldungsquellen`], und das leiht andere Felder
    /// aus.
    fn meldung_gewechselt(&self) {
        let melden = self.ivars().meldungswechsel.borrow();
        if let Some(melden) = melden.as_ref() {
            melden();
        }
    }

    /// Was dieses Dateifenster der Statuszeile anzubieten hat.
    ///
    /// Die vier Felder abgeschrieben, die beiden untersten Raenge gerechnet;
    /// die Regel darueber, welche der zwoelf Aussagen gewinnt, steht bei
    /// [`super::statuszeile::zeile`] und nicht hier. Diese Methode entscheidet
    /// nichts, damit die Entscheidung an genau einer Stelle steht und ohne
    /// AppKit pruefbar ist.
    ///
    /// **Eigene Zeichenketten und keine Ausleihen**, weil der Aufrufer
    /// unmittelbar danach das zweite Dateifenster fragt und dann AppKit ruft;
    /// eine Ausleihe des Tabmodells, die einen Objective-C-Aufruf ueberlebt,
    /// schliesst der Modulkopf aus.
    pub fn meldungsquellen(&self) -> Quellen {
        // Vor jeder Ausleihe: die beiden gerechneten Raenge holen ihre Zahlen
        // in einem Zug aus dem Tabmodell, und der Markierungsstand ruft danach
        // den Groessenformatierer, also Objective-C.
        let (markierung, filtertext, filterstand) = self.gerechnete_raenge();
        let markierungsstand = self.markierungsstand_text(markierung);
        let ivars = self.ivars();
        Quellen {
            befehlsantwort: ivars.befehlsantwort.borrow().clone(),
            vorgangsanzeige: ivars.vorgangsanzeige.borrow().clone(),
            fenstermeldung: ivars.fenstermeldung.borrow().clone(),
            tabmeldung: ivars.tabs.borrow().aktiver().meldung().map(str::to_owned),
            filterstand: statuszeile::filterstand_text(&filtertext, filterstand),
            markierungsstand,
        }
    }

    /// Die Eingaben der beiden Raenge ohne eigenes Feld, in **einer** Ausleihe
    /// des Tabmodells.
    ///
    /// **Ein Durchlauf ueber die Markierung fuer beide.** Der Markierungsstand
    /// aus C2 zaehlt ueber alle gelesenen Eintraege; der Filterstand aus C4
    /// braucht daneben, wie viele dieser Markierungen der Filter gerade
    /// ausblendet, und das ist dieselbe Zahl weniger den markierten Eintraegen
    /// der Sichtreihenfolge. Beide getrennt zu erheben hiesse,
    /// `Ordnermodell::markierungsstand` zweimal je Schreiben der Zeile ueber
    /// den ganzen Bestand laufen zu lassen.
    ///
    /// **Gerechnet wird hier nichts, was Modell oder Tab schon wissen.** Die
    /// drei Zahlen und der ausstehende Ersatz kommen als Fragen an das
    /// `Ordnermodell` herein, der Lesehinweis und die Zahl der zu grossen
    /// Dateien als Fragen an den [`crate::tabs::Tabinhalt`], der den Durchlauf
    /// haelt. Was daraus in der Zeile steht, entscheidet
    /// [`super::statuszeile::filterstand_text`], und das ist ohne Fenster
    /// pruefbar.
    fn gerechnete_raenge(&self) -> (Markierungsstand, String, Filterstand) {
        // Die Ausleihe endet mit diesem Block: alles hier Gelesene ist ein
        // eigener Wert, und der Aufrufer ruft gleich darauf Objective-C.
        let tabs = self.ivars().tabs.borrow();
        let tab = tabs.aktiver();
        let modell = tab.modell();
        let markierung = modell.markierungsstand();
        let markiert_sichtbar = modell
            .sichtreihenfolge()
            .iter()
            .filter(|index| modell.ist_markiert(**index))
            .count();
        let filterstand = Filterstand {
            gezeigt: modell.zeilenzahl(),
            vorhanden: modell.eintraege().len(),
            ausgeblendete_markierungen: markierung.zahl.saturating_sub(markiert_sichtbar),
            ersetzt_beim_naechsten_stapel: modell.ersetzt_beim_naechsten_stapel(),
            liest_inhalt: tab.liest_inhalt(),
            zu_gross: tab.zu_gross(),
        };
        (markierung, modell.filtertext().to_owned(), filterstand)
    }

    /// Der sechste Rang der Statuszeile: was im sichtbaren Tab markiert ist
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
    /// dafuer [`Self::meldung_gewechselt`], so wie sie die Tabelle neu laden.
    /// Das ist etwas anderes als ein Feld mit vier Schreibern: verpasst einer
    /// den Aufruf, steht ein alter Text in der Zeile, bis der naechste
    /// Zeichenanlass kommt, und nirgends ein falscher Zustand. Die beiden
    /// uebrigen Anlaesse zeichnen ohnehin: [`Self::tab_gewechselt`] und
    /// [`Self::nach_lesebeginn`] rufen [`Self::meldung_gewechselt`] seit S12 und
    /// S14, und der Lesebeginn deckt die Auffrischung mit ab, die die
    /// Markierung leert. Das Umsortieren und das Ein- und Ausblenden brauchen
    /// es nicht, weil sie die Markierung nicht anfassen und der Stand ueber
    /// alle gelesenen Eintraege zaehlt, nicht ueber die sichtbaren.
    ///
    /// **Der Stand kommt herein und wird hier nicht geholt.** Seit der Runde 10
    /// braucht ihn auch der Rang darueber, der Filterstand, und beide nehmen
    /// ihn aus [`Self::gerechnete_raenge`]; zwei Erhebungen je Schreiben der
    /// Zeile waeren zweimal derselbe Durchlauf ueber den ganzen Bestand.
    fn markierungsstand_text(&self, stand: Markierungsstand) -> Option<String> {
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
    /// [`DateifensterQuelle::meldung_gewechselt`]. Diese Methode setzt die Zeile
    /// **nicht** selbst: sie kaeme sonst an der Rangfolge vorbei und
    /// ueberschriebe eine laufende Vorgangsanzeige.
    pub fn meldung_zeigen(&self, meldung: &str) {
        *self.ivars().fenstermeldung.borrow_mut() = Some(meldung.to_owned());
        self.meldung_gewechselt();
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
        self.meldung_gewechselt();
    }

    /// Raeumt die Antwort auf den vorigen Tastenbefehl weg.
    ///
    /// Die einzige Loeschregel dieses Feldes, gerufen von ihren zwei
    /// Aufrufstellen: `Anwendungsdelegierter::befehlsantwort_beidseitig_loeschen`
    /// fuer beide Dateifenster und [`DateifensterQuelle::doppelklick`] an
    /// seinem einen Eingang. Der zweite Aufruf ist keine zweite Regel, sondern
    /// dieselbe an der Stelle, die der erste nicht erreicht: ein Doppelklick
    /// ist kein Kommando und laeuft an `kommando_ausfuehren` vorbei. Er raeumt
    /// dabei **nur diese eine** Statuszeile, waehrend die beidseitige Regel
    /// ueber beide Seiten laeuft; der Grund fuer den Unterschied steht bei
    /// [`DateifensterQuelle::doppelklick`].
    ///
    /// **Zwei Aufrufstellen, aber drei Anlaesse.** Die beidseitige Regel hat
    /// seit der Runde 13 selbst zwei Wege hinein — jeden Tastenbefehl und die
    /// Meldung des Abwurfs aus C7 —, und beide gehen durch dieselbe Schleife;
    /// die Aufzaehlung steht dort und wird hier nicht wiederholt. Stand
    /// keine Antwort, geschieht nichts: sonst schriebe jeder Pfeiltastendruck
    /// die Zeile neu, die sich nicht geaendert hat. Stand eine, kommt zum
    /// Vorschein, was darunter liegt — der Fortschritt der laufenden Operation
    /// oder die verdraengte Auswurfmeldung.
    pub fn befehlsantwort_loeschen(&self) {
        // **Der gemerkte Abwurfgrund faellt mit**, und zwar unabhaengig davon,
        // ob eine Antwort stand: er beschreibt die Zeile, die eben geraeumt
        // wurde, und ein Gedaechtnis, das laenger stuende als sein Gegenstand,
        // liesse eine zweite gleiche Ziehbewegung stumm — die Entdopplung in
        // [`abwurfmeldung`] vergliche gegen einen Grund, dessen Meldung es
        // nicht mehr gibt. Die Loeschregel des Rangs 1 gilt damit fuer beide
        // Felder und steht an einer Stelle.
        self.ivars().gemeldeter_abwurfgrund.set(None);
        if self.ivars().befehlsantwort.borrow_mut().take().is_some() {
            self.meldung_gewechselt();
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
        self.meldung_gewechselt();
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
        self.meldung_gewechselt();
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
        } else if einzug.ersetzt || einzug.befunde_neu {
            // Dieser Stapel hat die Liste des vorigen Lesevorgangs abgeloest.
            // `noteNumberOfRowsChanged` genuegt dafuer nicht: die Tabelle
            // zeigte weiter die Zellen des alten Ordners, und ihre Auswahl
            // stuende auf einer Zeile, die es nicht mehr gibt.
            //
            // Fuer einen eingetroffenen Befund gilt dasselbe aus einem
            // verwandten Grund: er stellt seine Zeile an die Stelle, die die
            // Sortierung ihr zuweist, also mitten in die Liste, und alle Zeilen
            // darunter tragen danach einen anderen Eintrag (C3.11). Der
            // Bildlauf bleibt dabei stehen, wo er steht — `reloadData` ruehrt
            // ihn nicht an —, und die Auswahl haengt am Eintragsindex und
            // wandert mit; [`Self::auswahl_anzeigen`] zeigt sie nur neu.
            self.ivars().tabelle.reloadData();
            self.auswahl_anzeigen();
        } else if einzug.angehaengt {
            self.ivars().tabelle.noteNumberOfRowsChanged();
        }
        // Die Tabmeldung wechselt selten, der fuenfte Rang bei jedem Stapel:
        // er nennt die Zahl der gezeigten und die der vorhandenen Eintraege,
        // und beide waechst der Lesevorgang. Ohne Filtertext meldet er nichts
        // (C4.8), und dann hat kein Stapel etwas an der Zeile zu aendern; die
        // zweite Bedingung haelt den Neubau beider Quellensaetze aus dem Takt
        // heraus, solange kein Filter steht.
        if einzug.meldung_neu || self.filter_steht() {
            self.meldung_gewechselt();
        }

        // Die zweite Stufe der Lesereihenfolge: der sichtbare Tab steht, jetzt
        // duerfen die verdeckten lesen.
        if self.ivars().tabs.borrow().nachzuegler_faellig() {
            self.ivars().tabs.borrow_mut().nachzuegler_starten();
        }
        // Gefragt wird `arbeitet_noch` und nicht `liest_noch`: der Takt bedient
        // zwei Kanaele, und ein Durchlauf laeuft gerade dann, wenn kein
        // Lesevorgang mehr laeuft. Mit der engeren Frage hielte der Takt an,
        // bevor der erste Befund da ist.
        if !self.ivars().tabs.borrow().arbeitet_noch() {
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
    ///
    /// **Der Aufrufer sagt mit [`Rangmitnahme`], ob der Ersthelferrang
    /// mitkommt.** Es gibt genau zwei Aufrufer, und sie geben verschiedene
    /// Werte mit; die Begruendung steht an der Aufzaehlung und nicht hier, damit
    /// sie nicht an zwei Stellen auseinanderlaeuft.
    fn angefasst(&self, mitnahme: Rangmitnahme) {
        let melden = self.ivars().aktivierung.borrow();
        if let Some(melden) = melden.as_ref() {
            melden(mitnahme);
        }
    }

    // ------------------------------------------------------------------
    // Der Abwurf aus einer fremden Anwendung (C4 bis C7 der Runde 13)
    // ------------------------------------------------------------------

    /// Ob KRK gerade schon einen Vorgang haelt.
    ///
    /// Der Weg zu der einen Stelle, die diese Frage beantwortet, ohne die
    /// Meldung, die die drei Tastenwege bekommen; der Grund steht an
    /// [`Vorgangsfrage`].
    ///
    /// **Steht der Rueckruf nicht, gilt „es laeuft einer".** Das ist die
    /// vorsichtige Fuellung einer Tatsache, die KRK dann nicht messen kann, und
    /// sie faellt mit den uebrigen fuenf in dieselbe Regel statt in einen
    /// eigenen Ausgang daneben. Eintreten kann der Fall nicht: der Rueckruf
    /// steht seit `Anwendungsdelegierter::oberflaeche_aufbauen`, und ein
    /// Ziehvorgang braucht ein stehendes Fenster.
    fn vorgang_laeuft_fragen(&self) -> bool {
        let fragen = self.ivars().vorgang_laeuft.borrow();
        fragen.as_ref().is_none_or(|fragen| fragen())
    }

    /// Raeumt die Befehlsantwort an **beiden** Dateifenstern weg (C7).
    ///
    /// Der Weg zu `Anwendungsdelegierter::befehlsantwort_beidseitig_loeschen`,
    /// also zu der einen Loeschregel des Rangs 1, und keine zweite daneben:
    /// eine Quelle erreicht von sich aus nur ihre eigene Seite, und der Rang 1
    /// gehoert beiden Dateifenstern gemeinsam.
    ///
    /// **Warum der Abwurf ueberhaupt beide Seiten braucht**, wo der Doppelklick
    /// daneben ausdruecklich nur seine eigene raeumt, steht am Ziel dieses
    /// Rueckrufs: [`statuszeile::zeile`](super::statuszeile::zeile) nimmt
    /// innerhalb eines Rangs die aktive Seite zuerst, und eine Meldung, die im
    /// **nicht** aktiven Dateifenster steht, verliert gegen jede stehende
    /// Befehlsantwort im aktiven.
    ///
    /// **Steht der Rueckruf nicht, geschieht nichts.** Das ist kein zweiter,
    /// engerer Ausgang, sondern gar keiner: eine halbe Raeumung allein an
    /// dieser Seite waere die zweite Loeschregel, die es hier nicht geben soll,
    /// und [`Self::befehlsantwort_zeigen`] ueberschreibt das eigene Feld
    /// ohnehin. Eintreten kann der Fall nicht — der Rueckruf steht seit
    /// `Anwendungsdelegierter::oberflaeche_aufbauen`, und ein Ziehvorgang
    /// braucht ein stehendes Fenster —, es ist derselbe Grund wie bei
    /// [`Self::vorgang_laeuft_fragen`] darueber.
    fn befehlsantwort_beidseitig_loeschen(&self) {
        let raeumen = self.ivars().befehlsantwort_raeumer.borrow();
        if let Some(raeumen) = raeumen.as_ref() {
            raeumen();
        }
    }

    /// Was die Ablage dieses Ziehvorgangs traegt, hoechstens einmal je
    /// Ziehsitzung erhoben (C4, C6, C7).
    ///
    /// **Die eine Stelle, die [`QuelleIvars::abwurfquellen`] liest und
    /// schreibt.** Stimmt die gemerkte Nummer der Ziehsitzung mit der des
    /// laufenden Ziehvorgangs ueberein, kommt das Gemerkte zurueck; sonst wird
    /// die Ablage einmal gelesen und das Feld ersetzt. Die Loeschregel im
    /// Einzelnen und die gemessenen Zahlen, die den Zwischenspeicher
    /// verlangen, stehen an jenem Feld.
    ///
    /// **Die Ausleihe endet vor dem Griff in die Ablage.** Das Gemerkte wird
    /// herauskopiert, und erst danach faellt die Entscheidung, ob
    /// [`super::zwischenablage::dateiverweise`] laeuft; ein `borrow()`, das
    /// diesen Aufruf ueberlebte, traefe auf das `borrow_mut()` unmittelbar
    /// dahinter. Das ist dieselbe Regel, unter der auch das Tabmodell in
    /// dieser Datei steht.
    fn abwurfquellen(&self, zug: &ProtocolObject<dyn NSDraggingInfo>) -> Abwurfquellen {
        let folge = zug.draggingSequenceNumber();
        let gemerkt = self.ivars().abwurfquellen.borrow().clone();
        if let Some(gemerkt) = gemerkt
            && gemerkt.folge == folge
        {
            return gemerkt;
        }

        let quellen = super::zwischenablage::dateiverweise(&zug.draggingPasteboard());
        let frisch = Abwurfquellen {
            folge,
            traegt_dateien: !quellen.is_empty(),
            gemeinsamer_ordner: gemeinsamer_quellordner(&quellen),
        };
        *self.ivars().abwurfquellen.borrow_mut() = Some(frisch.clone());
        frisch
    }

    /// Was geschaehe, wenn der Nutzer jetzt loslaesst (C4 bis C7).
    ///
    /// Sechs Tatsachen herein, ein Urteil heraus, und keine der sechs wird hier
    /// beurteilt: die Marke und der Zielordner kommen aus
    /// [`abwurfregel::marke`], das Urteil aus [`abwurfregel::urteil`], die
    /// Uebersetzung in die Sprache des Zeigers aus [`abwurf::zeiger`]. Diese
    /// Funktion **beschafft** und ordnet nichts ein.
    ///
    /// Der Reihe nach: der laufende Vorgang, die Marke und mit ihr der
    /// Zielordner, die Quellen aus der Ablage des Ziehvorgangs — ueber
    /// [`Self::abwurfquellen`] und damit hoechstens einmal je Ziehsitzung statt
    /// einmal je Zeigerbewegung —, das Schreibrecht des Ziels, die angebotene
    /// Menge. Danach das Urteil, die
    /// Marke an der Tabelle, die entdoppelte Meldung — und mit ihr, an
    /// derselben Kante, die Raeumung des Rangs 1 an **beiden** Dateifenstern —
    /// und zuletzt der Zeiger.
    ///
    /// **Die Ausleihe des Tabmodells endet vor dem ersten Objective-C-Aufruf**,
    /// und zwar in [`Self::eintrag_in_zeile`] und [`Self::angezeigter_ordner`]
    /// selbst: beide geben eigenen Besitz zurueck. Das ist die Regel des
    /// Modulkopfs, und ein `borrow()`, das eine AppKit-Zeile ueberlebte, waere
    /// der Absturz, gegen den sie geschrieben ist — `setDropRow:` und die
    /// Meldung rufen beide in AppKit hinein, und die Meldung kommt ueber die
    /// Statuszeile in dieselbe Quelle zurueck.
    ///
    /// **Der Zielordner entsteht an genau einer Stelle**, naemlich hier, und
    /// eine zweite, die aus einer Zeilennummer einen Pfad macht, entsteht
    /// nicht: [`Self::eintrag_in_zeile`] ist die eine Uebersetzung, und
    /// [`Self::abwurf_annehmen`] liest die Zeile zurueck, die diese Funktion
    /// gesetzt hat, statt sie neu zu bestimmen.
    ///
    /// **„Das Ziel ist der Quellordner" heisst: jede Quelle liegt darin.**
    /// Liegt nur ein Teil der gezogenen Eintraege im Ziel, ist der Ziehvorgang
    /// nicht der Fall aus C6 — er kommt aus mehreren Ordnern —, und die
    /// Eintraege, die dort schon stehen, treffen auf dieselbe Konfliktrueckfrage
    /// wie bei F5 und F6. Das ist genau die Antwort, die `auftrag_stellen` fuer
    /// die Auswahl gibt, wo alle Quellen ohnehin aus einem Ordner kommen.
    ///
    /// **Dieser Vergleich ist eine Vorhersage und keine Entscheidung**, und das
    /// steht hier, weil er als Entscheidung gelesen worden ist. Verglichen
    /// werden zwei Pfade als Text, und die beiden Seiten kommen seit dieser
    /// Runde nicht mehr aus derselben Quelle: der Zielordner stammt aus KRKs
    /// Ordnermodell, die Quellpfade aus `NSURL::path` auf Verweisen, die eine
    /// fremde Anwendung geschrieben hat — und die schreibt sie aufgeloest.
    /// Derselbe Ordner unter zwei Schreibweisen — `/tmp` gegen `/private/tmp`,
    /// ein Lesezeichen ueber einen symbolischen Verweis, ein Unterschied in der
    /// Gross- und Kleinschreibung — liest sich damit als zwei Ordner, und die
    /// Abweisung aus C6 Lage 3 bleibt aus.
    ///
    /// **Genauer laesst sich die Frage an dieser Stelle nicht beantworten, und
    /// sie muss es auch nicht.** Genauer waere sie nur ueber `st_dev` und
    /// `st_ino`, also ueber einen Systemaufruf je Zeigerbewegung; und selbst
    /// dann bliebe sie eine Vorhersage, denn zwischen der letzten
    /// Zeigerbewegung und dem Loslassen kann der Ordner ein anderer werden.
    /// Entschieden wird sie deshalb dort, wo sie zu entscheiden ist: in
    /// `krk_core::operation`s `zielpfad`, im Augenblick des Zugriffs und ueber
    /// die Naemlichkeit statt ueber die Schreibweise. Was hier durchrutscht,
    /// kostet den Nutzer damit einen Zeiger, der annimmt, und danach je Eintrag
    /// die Zeile „Quelle und Ziel sind derselbe Eintrag" in der Abschlussliste
    /// — auf demselben Weg, auf dem C6 auch seine vierte Lage beantwortet:
    /// nachtraeglich entschieden und nicht vorhergesagt.
    fn abwurf_pruefen(
        &self,
        zug: &ProtocolObject<dyn NSDraggingInfo>,
        zeile: NSInteger,
        vorgeschlagen: NSTableViewDropOperation,
    ) -> NSDragOperation {
        let eintrag = usize::try_from(zeile)
            .ok()
            .and_then(|zeile| self.eintrag_in_zeile(zeile));
        let marke = abwurfregel::marke(
            vorgeschlagen == NSTableViewDropOperation::On,
            eintrag.as_ref().map(|(_, typ)| *typ),
        );
        let ziel = match (marke, eintrag) {
            (Abwurfmarke::Zeile, Some((pfad, _))) => pfad,
            // `Zeile` ohne Eintrag kann nicht entstehen — die Marke folgt
            // allein aus `Some(Typ::Ordner)`. Der Zweig steht, weil der Typ ihn
            // verlangt, und faellt auf dasselbe Ziel wie `Liste`.
            (Abwurfmarke::Zeile, None) | (Abwurfmarke::Liste, _) => self.angezeigter_ordner(),
        };

        let quellen = self.abwurfquellen(zug);
        let (bietet_kopieren, bietet_verschieben) = abwurf::angebot(zug);
        let gefaellt = abwurfregel::urteil(&Abwurflage {
            traegt_dateien: quellen.traegt_dateien,
            vorgang_laeuft: self.vorgang_laeuft_fragen(),
            schreibrecht: abwurf::beschreibbarkeit(&ziel),
            ziel_ist_quellordner: quellen.gemeinsamer_ordner.as_deref() == Some(ziel.as_path()),
            bietet_kopieren,
            bietet_verschieben,
        });

        // Die Marke des Systems und keine eigene Zeichnung: eine Zeilennummer
        // hebt die Zeile hervor, `-1` umrandet die ganze Liste
        // (`NSTableView.h:317`). Beide Male `On`, denn zwischen zwei Zeilen
        // gibt es hier nichts einzufuegen.
        let (marken_zeile, marken_art) = match marke {
            Abwurfmarke::Zeile => (zeile, NSTableViewDropOperation::On),
            Abwurfmarke::Liste => (-1, NSTableViewDropOperation::On),
        };
        self.ivars()
            .tabelle
            .setDropRow_dropOperation(marken_zeile, marken_art);

        let grund = match gefaellt {
            Abwurfurteil::Abweisen(grund) => Some(grund),
            Abwurfurteil::Ausfuehren(_) => None,
        };
        if let Some(meldung) = abwurfmeldung(self.ivars().gemeldeter_abwurfgrund.get(), grund) {
            // **Erst raeumen, dann schreiben, und beides an derselben Kante.**
            // Die Raeumung haengt an demselben `Some` wie die Meldung und
            // laeuft deshalb nicht bei jeder Zeigerbewegung: `validateDrop:`
            // laeuft bei jeder, und [`abwurfmeldung`] ist die Entdopplung, die
            // daraus die wenigen Wechsel macht. Ohne die Raeumung stuende die
            // Meldung im **nicht** aktiven Dateifenster hinter einer noch
            // stehenden Befehlsantwort des aktiven und waere nie zu sehen.
            //
            // `beschlossener_vorgang` faellt dabei ausdruecklich **nicht** mit;
            // seine Loeschregel ist die entgegengesetzte, und die Begruendung
            // steht an dem Feld.
            self.befehlsantwort_beidseitig_loeschen();
            self.befehlsantwort_zeigen(meldung);
        }
        // Nach der Raeumung, denn sie hat dieses Feld an beiden Seiten auf
        // `None` gestellt; der eben gefaellte Grund gehoert hierher und nicht
        // der geraeumte.
        self.ivars().gemeldeter_abwurfgrund.set(grund);
        self.ivars().beschlossener_vorgang.set(match gefaellt {
            Abwurfurteil::Ausfuehren(vorgang) => Some(vorgang),
            Abwurfurteil::Abweisen(_) => None,
        });

        abwurf::zeiger(gefaellt)
    }

    /// Gibt einen angenommenen Abwurf an den Anwendungsdelegierten weiter
    /// (C4 bis C6).
    ///
    /// **Hier wird nichts mehr beurteilt.** Der Vorgang steht seit
    /// [`Self::abwurf_pruefen`] in `beschlossener_vorgang`, und die Zeile ist
    /// die, die jene Funktion an der Tabelle gesetzt hat: `-1` heisst „der
    /// angezeigte Ordner", jede andere benennt die Ordnerzeile. Gelesen wird
    /// damit zurueck, was das Urteil entschieden hat, statt es zu wiederholen —
    /// eine zweite Beurteilung koennte anders ausfallen als die, die der Zeiger
    /// gezeigt hat, und C5 sagt zu, dass beide uebereinstimmen.
    ///
    /// **Die Quellen werden hier noch einmal gelesen und nicht dem
    /// Zwischenspeicher entnommen**, den [`Self::abwurfquellen`] fuehrt. Billig
    /// ist das nicht: gemessen sind 585 ms fuer 5.000 gezogene Eintraege, und
    /// die Zahlen stehen bei [`super::zwischenablage::dateiverweise`]. Sie
    /// fallen aber **einmal** an, im Augenblick des Loslassens, unmittelbar vor
    /// einem Vorgang, der laenger dauert — und nicht je Zeigerbewegung, was der
    /// gemessene Grund fuer jenen Zwischenspeicher war. Was er fuehrt, sind
    /// zwei abgeleitete Groessen, ein Wahrheitswert und ein Pfad; die ganze
    /// Liste dazuzunehmen hiesse, eine Ziehsitzung voller Pfade ueber deren
    /// Ende hinaus zu halten, ohne dass eine gemessene Zahl es verlangte.
    ///
    /// **Was zwischen dem letzten Zeigerpunkt und dem Loslassen geschieht, ist
    /// nicht vorherzusagen**, und die drei Ausgaenge stehen in [`abwurfziel`]
    /// als Tafel. Frischt die Liste in dieser Spanne auf und steht an der Zeile
    /// ein **anderer Ordner**, geht dessen Pfad als Ziel mit; die
    /// Operationsmaschine haengt jeden Namen daran an und meldet den
    /// gescheiterten Eintrag mit seinem Grund in der Abschlussliste, auf
    /// demselben Weg, den F5 und F6 gehen. Der Plan der Runde 13 nennt diese
    /// Sorte Frage in seiner `Decidability`-Zeile: nachtraeglich entschieden
    /// und nicht vorhergesagt.
    ///
    /// **Steht an der Zeile dagegen nichts mehr oder keine Ordnerzeile, wird
    /// der Abwurf nicht angenommen.** Bis zum 260819 gingen beide Faelle
    /// stillschweigend anders aus. Eine **verschwundene** Zeile ergab dasselbe
    /// `None` wie die `-1`, die „die ganze Liste" heisst, und der Abwurf landete
    /// im angezeigten Ordner — also im uebergeordneten Ordner dessen, worauf
    /// der Nutzer gezeigt hatte. Nichts war dabei gescheitert, also stand auch
    /// nichts in der Abschlussliste. Eine Zeile, an der jetzt eine **Datei**
    /// steht, ging als Zielordner mit, und die Operationsmaschine haengt die
    /// Namen an einen Pfad an, der kein Ordner ist. Der erste Ausgang ist der,
    /// den der Nutzer nicht wiederholen kann: er sieht nicht, dass etwas
    /// anderes geschehen ist als das, worauf er gezeigt hat. `false` ist
    /// dagegen billig: AppKit laesst die Eintraege zurueckfliegen, geschrieben
    /// wird nirgends etwas, und die Geste kostet den Nutzer eine Sekunde.
    ///
    /// Zurueck kommt, ob der Abwurf angenommen wurde. `false` heisst „es ist
    /// nichts geschehen"; AppKit laesst die Einträge dann an ihren Ort
    /// zurueckfliegen.
    fn abwurf_annehmen(&self, zug: &ProtocolObject<dyn NSDraggingInfo>, zeile: NSInteger) -> bool {
        let Some(vorgang) = self.ivars().beschlossener_vorgang.get() else {
            return false;
        };
        let eintrag = usize::try_from(zeile)
            .ok()
            .and_then(|zeile| self.eintrag_in_zeile(zeile));
        let ziel = match (
            abwurfziel(zeile >= 0, eintrag.as_ref().map(|(_, typ)| *typ)),
            eintrag,
        ) {
            (Abwurfziel::Zeile, Some((pfad, _))) => pfad,
            (Abwurfziel::AngezeigterOrdner, _) => self.angezeigter_ordner(),
            // `Zeile` ohne Eintrag kann nicht entstehen — die Marke folgt
            // allein aus `Some(Typ::Ordner)`. Der Zweig steht, weil der Typ ihn
            // verlangt, und faellt auf denselben Ausgang wie `Keines`.
            (Abwurfziel::Zeile, None) | (Abwurfziel::Keines, _) => return false,
        };
        let quellen = super::zwischenablage::dateiverweise(&zug.draggingPasteboard());
        if quellen.is_empty() {
            return false;
        }

        let melden = self.ivars().abwurf.borrow();
        let Some(melden) = melden.as_ref() else {
            return false;
        };
        melden(ziel, quellen, vorgang);
        true
    }
}

/// Was der Delegierte haelt.
pub struct DelegiertenIvars {
    /// Die Quelle, aus der der Delegierte die Zeilen liest.
    quelle: Retained<DateifensterQuelle>,
    /// Der Formatierer fuer den Datumsteil der Spalte mit dem
    /// Aenderungsdatum.
    ///
    /// **Zwei Formatierer und nicht einer**, der Datum und Zeit zusammen
    /// setzt: den Trenner zwischen beiden Teilen waehlt `NSDateFormatter`
    /// sonst nach der Sprachregion, und im deutschen Raum ist das ein Komma.
    /// Die Begruendung im Einzelnen steht bei [`DateifensterDelegierter::neu`],
    /// zusammengesetzt wird in
    /// [`DateifensterDelegierter::datum_beschriften`].
    ///
    /// Beide entstehen einmal und nicht je Zelle: ein `NSDateFormatter` baut
    /// beim Anlegen die Kalender- und Sprachtabellen auf und ist damit das
    /// teuerste Objekt im Zeichenweg. Zwei feste Formatierer sind dasselbe
    /// Argument und kein Widerspruch dazu: gezaehlt wird, was je gezeichneter
    /// Zelle entsteht, und das bleibt nichts.
    datumsformat: Retained<NSDateFormatter>,
    /// Der Formatierer fuer den Zeitteil derselben Spalte.
    ///
    /// Sein Gegenstueck ist [`DelegiertenIvars::datumsformat`]; die
    /// Begruendung steht dort.
    zeitformat: Retained<NSDateFormatter>,
    /// Die Schrift einer unmarkierten Zelle.
    ///
    /// Die beiden Schriften entstehen einmal und nicht je Zelle, aus demselben
    /// Grund wie die Datumsformatierer: sie liegen im Zeichenweg, den ein
    /// Ordner mit 100.000 Eintraegen oft genug durchlaeuft.
    schrift: Retained<NSFont>,
    /// Die Schrift einer markierten Zelle: dieselbe Groesse, fett.
    ///
    /// Das zweite Kennzeichen der Markierung aus C2 neben der Farbe. Eine Form
    /// und keine Farbe, damit es bei jeder Farbfehlsichtigkeit wirkt
    /// (`decisions/260805-0000_*_zweites-kennzeichen-der-markierung-und-ihr-platz-in-der-statuszeile.md`).
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

        /// Der Doppelklick auf eine Zeile der Tabelle (C3).
        ///
        /// AppKit schickt sie an das Ziel der Tabelle, wenn ein Doppelklick
        /// niedergeht; welche Zeile gemeint ist, steht danach in `clickedRow`
        /// des Absenders und nicht in der Auswahl. Der Delegierte entscheidet
        /// hier nichts, sondern reicht die Zeile weiter, wie es
        /// `umbenennungBeendet:` daneben tut.
        // SAFETY: Die Signatur passt zu der, die NSControl an sein Ziel
        // schickt: ein Argument, der Absender. Er ist die Tabelle, an der die
        // Aktion gesetzt wurde.
        #[unsafe(method(doppelklick:))]
        fn doppelklick(&self, absender: &NSTableView) {
            self.ivars().quelle.doppelklick(absender.clickedRow());
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
        /// und nicht bei `selectRowIndexes:byExtendingSelection:`. Deshalb
        /// steht hier eine Umschaltung des aktiven Dateifensters: ein Klick in
        /// die andere Liste macht sie zur aktiven, waehrend ein vom Programm
        /// gelesener Ordner im verdeckten Tab nichts umschaltet.
        ///
        /// **Die einzige Umschaltung ist sie seit dem 260819 nicht mehr.** Bis
        /// dahin stand hier, sie stehe genau deshalb an dieser Stelle; das galt
        /// fuer den Klick auf eine Zeile und wurde stillschweigend fuer die
        /// ganze Regel genommen. Ein Klick unter die letzte Zeile ruft diese
        /// Methode nicht — es gibt dort keine Zeile —, und das aktive
        /// Dateifenster blieb auf der anderen Seite stehen
        /// (`shared/issues/260819-1043_*_ein-klick-unter-die-letzte-zeile-…`).
        /// Der Nutzerentscheid vom 260819 sagt: jede Flaeche eines
        /// Dateifensters macht es zum aktiven. Getragen wird das von
        /// `Anwendungsdelegierter::aktives_dem_ersthelfer_nachziehen`, das am
        /// Ersthelferwechsel des Hauptfensters haengt und damit jeden Klick
        /// sieht, den AppKit in einen Rangwechsel uebersetzt.
        ///
        /// **Diese Zeile bleibt trotzdem stehen und ist keine Wiederholung.**
        /// Der Weg ueber den Ersthelfer greift nur, wenn der Rang wirklich
        /// wechselt; verweigert der bisherige Ersthelfer ihn — der Editor kann
        /// das ueber seinen Delegierten —, bleibt er stehen, und dann ist diese
        /// Zeile die einzige, die den Klick auf eine Zeile noch umsetzt. Beide
        /// Wege muenden in dieselbe Funktion, `aktives_setzen`, und die zweite
        /// Meldung ist dort folgenlos.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(tableView:shouldSelectRow:))]
        fn zeile_waehlbar(&self, _tabelle: &NSTableView, _zeile: NSInteger) -> bool {
            self.ivars().quelle.angefasst(Rangmitnahme::Appkit);
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
    ///
    /// # Eine Zeile mit offener Namensbearbeitung kommt hier nicht an
    ///
    /// Das `setStringValue:` weiter unten laeuft in **jedem** Durchgang und
    /// fragt nicht, ob diese Zelle gerade der Editor einer Umbenennung ist.
    /// Das sieht nach einem Defekt aus, und der Fehlbefund
    /// `shared/issues/260815-2203_*_…` ist genau daraus entstanden: waehrend
    /// einer Bearbeitung schreibt ein `setStringValue:` wirklich in den
    /// Feldeditor zurueck (der Kopf von [`Namensfeld`] sagt es), also stuende
    /// nach einem Durchgang die Anzeigeform `Bilder/` im Feldeditor und Return
    /// meldete dem Nutzer einen Schraegstrich, den er nie getippt hat.
    ///
    /// **Diesen Durchgang gibt es nicht.** AppKit haelt die Zeile mit dem
    /// offenen Feldeditor aus dem Delegierten heraus, und zwar auf zwei Weisen
    /// je nach Anlass: `reloadData` und
    /// `reloadDataForRowIndexes:columnIndexes:` **beenden die Bearbeitung**,
    /// bevor der erste Durchgang laeuft, und ein Bildlauf **ueberspringt** die
    /// bearbeitete Zeile, waehrend er ihre Nachbarn neu holt. `currentEditor`
    /// ist hier deshalb immer `None`, und eine Abfrage darauf waere toter
    /// Code.
    ///
    /// Am 260816 auf macOS 15.7.7 mit einem weggeworfenen Programm auf dem
    /// wirklichen Hauptfaden gemessen, an einer `NSTableView` in einer
    /// `NSScrollView` mit derselben Verdrahtung wie hier. Sechs Anlaesse, in
    /// keinem einzigen Durchgang ein Feldeditor:
    ///
    /// | Anlass | Bearbeitung danach | Durchgang der bearbeiteten Zeile |
    /// |---|---|---|
    /// | `reloadData` | beendet | ja, danach, ohne Feldeditor |
    /// | `reloadDataForRowIndexes:columnIndexes:` | beendet | ja, danach, ohne Feldeditor |
    /// | `noteNumberOfRowsChanged` | steht weiter | keiner |
    /// | `selectRowIndexes:byExtendingSelection:` | beendet | keiner |
    /// | Bildlauf aus dem Bild und zurueck | steht weiter | keiner, die Zeile wird uebersprungen |
    /// | erstmaliger Aufbau einer Zeile | — | ohne Feldeditor |
    ///
    /// **`NSTableView::editedRow` und `editedColumn` beantworten die Frage
    /// nicht**, und das ist dieselbe Messung: beide stehen waehrend einer
    /// offenen Bearbeitung dieser Tabelle auf `-1`. Sie gehoeren der
    /// zellenbasierten Tabelle; die hier ist ansichtsbasiert. Wer die Zelle in
    /// Bearbeitung doch einmal erkennen muss, fragt `currentEditor` am Feld
    /// und nicht die Tabelle.
    ///
    /// Was an derselben Messung **nicht** in Ordnung ist, gehoert nicht
    /// hierher: die beiden Zeichendurchgaenge beenden die Bearbeitung, ohne
    /// die Aktion zu schicken, und werfen damit den getippten Text fort. Das
    /// ist der dritte Ausgang aus
    /// `shared/issues/260815-2125_*_verlaesst-der-nutzer-die-offene-namenszelle-…`.
    /// **Seit dem Nutzerentscheid vom 260816-0021 erreichen die beiden
    /// Zeichendurchgaenge eine offene Zelle nicht mehr von selbst:**
    /// [`crate::auffrischung::ordner_neu_lesen`] laesst dieses Dateifenster
    /// nicht lesen, solange sie steht. Offen bleibt allein der wirkliche Klick
    /// des Nutzers neben die Zelle
    /// (`shared/decisions/260816-0021_*_verwirft-oder-uebernimmt-ein-klick-neben-die-offene-namenszelle.md`).
    fn zellenansicht(
        &self,
        tabelle: &NSTableView,
        spalte: Option<&NSTableColumn>,
        zeile: NSInteger,
    ) -> Option<Retained<NSView>> {
        let spalte = aus_kennung(&spalte?.identifier())?;
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
        //
        // **Die Daempfung aus C5 teilt sich diese eine Eigenschaft mit der
        // Markierung, und bei einem Zusammentreffen schreibt die Markierung.**
        // Eine Zeile, die allein wegen ihres Inhalts dasteht, steht in
        // `secondaryLabelColor`; ein markierter Eintrag bleibt orange und fett,
        // gleich aus welchem Grund er in der Liste steht. Die Rangfolge folgt
        // den Folgen: die Markierung entscheidet, worauf Loeschen, Verschieben
        // und Kopieren wirken, und wer sie uebersieht, verliert Dateien; die
        // Daempfung ist eine Auskunft ueber die Herkunft der Zeile, und wer sie
        // uebersieht, oeffnet die Datei und sieht nach. **Der Verlust ist benannt
        // und angenommen:** unter den markierten Eintraegen ist ein
        // Inhaltstreffer nicht mehr von einem Namenstreffer zu unterscheiden.
        // Die Abwaegung im Einzelnen steht in
        // `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/decisions/260816-1359_*_welche-aussage-schreibt-die-dateizelle-wenn-markierung-und-inhaltsdaempfung-zusammentreffen.md`.
        //
        // **`secondaryLabelColor` und nicht `tertiaryLabelColor`.** Die Leiste
        // traegt das Vokabular schon: das zweite steht dort fuer eine Marke, deren
        // Ziel fehlt, also fuer etwas Kaputtes; das erste fuer eine Ueberschrift,
        // also fuer etwas Gueltiges mit anderem Rang. Ein Inhaltstreffer ist eine
        // gueltige Zeile mit anderer Herkunft und keine beschaedigte Datei.
        //
        // **Die Schriftwahl bleibt zweiwertig.** Fett gehoert der Markierung;
        // die Daempfung bekommt kein zweites Kennzeichen, sonst entstuende der
        // dritte Zustand, den C5.4 ausschliesst.
        //
        // **Alle drei sind dynamische Systemfarben.** Ein Wechsel der Farbtafel
        // zieht damit von selbst nach, und die Tabelle braucht keinen Beobachter
        // der Erscheinung. Die Auswahl bleibt AppKit ueberlassen: KRK schreibt
        // keine Auswahlfarbe, und eine ausgewaehlte Zeile bleibt blau unterlegt,
        // gleich welche Textfarbe sie traegt.
        let markiert = self.ivars().quelle.zeile_markiert(zeile);
        let farbe = if markiert {
            NSColor::systemOrangeColor()
        } else if self.ivars().quelle.zeile_steht_wegen_des_inhalts(zeile) {
            NSColor::secondaryLabelColor()
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
    ///
    /// # Warum der Datumsteil und der Zeitteil zwei Formatierer haben
    ///
    /// Ein einzelner `NSDateFormatter` mit beiden Stilen setzt zwischen Datum
    /// und Zeit den Trenner, den die Sprachregion dafuer vorsieht; im
    /// deutschen Raum ist das ein Komma, und KRK schreibt es nirgends hin.
    /// Zwei Formatierer nebeneinander, jeder mit genau einem der beiden
    /// Stile, machen den Trenner zu KRKs Sache und lassen beiden Teilen ihre
    /// Sprachregion. Die beiden anderen Wege sind verworfen: ein eigenes
    /// `setDateFormat:` entfernte das Komma und hoebe zugleich die Anpassung
    /// an die Sprachregion des Nutzers auf, und das Komma nachtraeglich aus
    /// der fertigen Zeichenkette zu schneiden waere eine Regel ueber einen
    /// Aufbau, den KRK nicht kennt. Die Abwaegung im Einzelnen steht in
    /// `shared/issues/260811-1730_*_ziffern-in-dateiliste-und-leiste-laufen-auseinander-und-das-datum-traegt-ein-komma.md`.
    fn neu(mtm: MainThreadMarker, quelle: Retained<DateifensterQuelle>) -> Retained<Self> {
        let datumsformat = NSDateFormatter::new();
        datumsformat.setDateStyle(NSDateFormatterStyle::ShortStyle);
        datumsformat.setTimeStyle(NSDateFormatterStyle::NoStyle);
        let zeitformat = NSDateFormatter::new();
        zeitformat.setDateStyle(NSDateFormatterStyle::NoStyle);
        zeitformat.setTimeStyle(NSDateFormatterStyle::ShortStyle);
        // Die Systemschriftgroesse und nicht eine Zahl im Programmtext: sie
        // haengt an der Einstellung des Nutzers. Beide Schriften nehmen
        // dieselbe, damit eine markierte Zeile fett wird und nicht groesser.
        //
        // **Festbreite Ziffern bei proportionalen Buchstaben**, und nicht eine
        // durchgehende Festbreitenschrift: die Spalten `Groesse` und
        // `Geaendert` sollen untereinander stehen, waehrend die Dateinamen der
        // Namensspalte ihre Proportionalschrift behalten. Gemessen am 260811
        // auf macOS 15.7.7 mit `NSAttributedString::size` bei 13 Punkt:
        // „11.11.11 11:11“ und „08.08.88 08:88“ sind in `systemFontOfSize:`
        // 73,07 und 95,01 Punkt breit, also 22 Punkt auseinander, und in
        // dieser Schrift beide 96,05 Punkt. Der Name „Ablage.rs“ misst in
        // beiden Schriften dieselben 57,36 Punkt: die Buchstaben ruehrt der
        // Wechsel nicht an.
        //
        // **Das Gewicht der fetten Fassung ist `NSFontWeightBold`**, und die
        // Wahl ist gemessen und nicht angenommen: `boldSystemFontOfSize:` und
        // `monospacedDigitSystemFontOfSize:weight:` mit diesem Gewicht setzen
        // „Ablage.rs“ beide auf 62,38 Punkt und tragen denselben Auf- und
        // Abstrich (12,568 / -2,742). Die markierte Zeile wird damit fett und
        // nicht breiter, gerade so, wie sie oben nicht groesser wird — und die
        // Ziffernbreite springt beim Markieren nicht, weil beide Schriften sie
        // festhalten.
        let groesse = NSFont::systemFontSize();
        // SAFETY: `NSFontWeightRegular` und `NSFontWeightBold` sind zwei
        // Fremdsymbole von AppKit, beide `CGFloat`. Sie werden gelesen und
        // nicht geschrieben, wie die Merkmalsnamen in `nummernspalte.rs`.
        //
        // `monospacedDigitSystemFontOfSize:weight:` und die beiden
        // Gewichtskonstanten tragen im Kopf des Systems dasselbe
        // `API_AVAILABLE(macos(10.11))`: `NSFont.h:62` fuer die Methode,
        // `NSFontDescriptor.h:170` und `:173` fuer die Konstanten. Die
        // Untergrenze des Buendels ist 15.0 (`.cargo/config.toml`), und keine
        // der drei Stellen braucht deshalb eine Verfuegbarkeitspruefung zur
        // Laufzeit.
        let (gewoehnlich, fett) = unsafe { (NSFontWeightRegular, NSFontWeightBold) };
        let schrift = NSFont::monospacedDigitSystemFontOfSize_weight(groesse, gewoehnlich);
        let fettschrift = NSFont::monospacedDigitSystemFontOfSize_weight(groesse, fett);
        let this = Self::alloc(mtm).set_ivars(DelegiertenIvars {
            quelle,
            datumsformat,
            zeitformat,
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

    /// Holt die Anzeigeform einer Namenszelle aus dem Modell zurueck (C4).
    ///
    /// Gerufen von beiden Enden einer Bearbeitung,
    /// [`Namensfeld::bearbeitung_abbrechen`] und
    /// [`Namensfeld::bearbeitung_beendet`]. Der Delegierte reicht die Zelle an
    /// die Quelle weiter, wie es `umbenennungBeendet:` daneben tut; welche
    /// Zeile gemeint ist, weiss die Tabelle.
    fn anzeigeform_herstellen(&self, feld: &NSTextField) {
        self.ivars().quelle.anzeigeform_herstellen(feld);
    }

    /// Meldet der Quelle den Beginn einer Namensbearbeitung (C4).
    fn namensbearbeitung_begonnen(&self) {
        self.ivars().quelle.namensbearbeitung_begonnen();
    }

    /// Meldet der Quelle das Ende einer Namensbearbeitung (C4).
    fn namensbearbeitung_beendet(&self) {
        self.ivars().quelle.namensbearbeitung_beendet();
    }

    /// Laesst die Quelle nachholen, was die offene Zelle aufgehalten hat (C4).
    fn aufgeschobene_auffrischung_nachholen(&self) {
        self.ivars().quelle.aufgeschobene_auffrischung_nachholen();
    }

    /// Der Text, der in dieser Spalte fuer diesen Eintrag steht.
    fn beschriften(&self, spalte: Spalte, eintrag: &Eintrag) -> String {
        match spalte {
            // Ein Ordner traegt hier einen Schraegstrich hinter dem Namen; wie
            // die Anzeigeform entsteht und warum sie nie ein Name ist, steht
            // bei [`namensform`].
            Spalte::Name => namensform(eintrag),
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
            // Die Endung, nicht die Eintragsart: sie ist derselbe Wert, aus
            // dem die Sortierung nach Typ ihren Schluessel bildet, damit
            // Anzeige und Ordnung uebereinstimmen. Ein Eintrag ohne Endung
            // laesst die Zelle leer.
            Spalte::Typ => eintrag.endung().to_owned(),
        }
    }

    /// Ein Zeitpunkt in der Schreibweise, die der Nutzer eingestellt hat.
    ///
    /// Datum und Zeit kommen aus zwei Formatierern und werden hier mit einem
    /// Leerzeichen verbunden; warum der Trenner KRKs Sache ist, steht bei
    /// [`Self::neu`].
    fn datum_beschriften(&self, zeitpunkt: SystemTime) -> String {
        let Ok(seit_epoche) = zeitpunkt.duration_since(UNIX_EPOCH) else {
            // Ein Zeitpunkt vor 1970 ist auf einem Dateisystem moeglich, aber
            // kein Fall, fuer den eine eigene Darstellung lohnt.
            return String::new();
        };
        let datum = NSDate::dateWithTimeIntervalSince1970(seit_epoche.as_secs_f64());
        let tag = self.ivars().datumsformat.stringFromDate(&datum);
        let zeit = self.ivars().zeitformat.stringFromDate(&datum);
        format!("{tag} {zeit}")
    }

    /// Holt eine Zellenansicht aus dem Vorrat der Tabelle oder baut eine neue.
    ///
    /// Die Wiederverwendung ist der Grund, aus dem ein Ordner mit 100.000
    /// Eintraegen ohne Ruckeln blaettert: AppKit haelt nur die sichtbaren
    /// Ansichten und reicht die aus dem Bild gelaufenen zurueck.
    ///
    /// **Alle vier Spalten ruecken gleich weit ein, und das ist gemessen.** Am
    /// 260811 auf macOS 15.7.7, an vier Feldern aus dieser Methode: die
    /// Zeichenflaeche der Zelle beginnt in jeder Spalte bei 0 und nimmt die
    /// volle Breite, die Randabstaende sind ueberall 2 Punkt links wie rechts,
    /// die Grundlinie liegt ueberall 13 Punkt unter der Oberkante. Das
    /// `setEditable(true)` der Namensspalte aendert daran nichts. Die Tabelle
    /// legt die Spalten mit demselben Zwischenraum von 17 Punkt aus, den sie
    /// je zur Haelfte auf beide Seiten verteilt, und die Ueberschriften
    /// beginnen ebenso in jeder Spalte bei 0. Wer also den Verdacht hat, Name
    /// und Aenderungsdatum stuenden verschieden weit eingerueckt, sucht an
    /// dieser Stelle vergeblich: was die Spalten frueher zerrissen aussehen
    /// liess, war allein die Breite der Ziffern, und die haelt jetzt die
    /// Schrift aus [`Self::neu`] fest.
    fn feld(&self, tabelle: &NSTableView, spalte: Spalte) -> Retained<NSTextField> {
        let kennung = kennung(spalte);
        // SAFETY: `self` ist der Eigentuemer, den AppKit an eine neu geladene
        // Ansicht weiterreicht; die Kennung ist eine gueltige Zeichenkette.
        let vorrat = unsafe { tabelle.makeViewWithIdentifier_owner(kennung, Some(self)) };
        if let Some(gebraucht) = vorrat.and_then(|ansicht| ansicht.downcast::<NSTextField>().ok()) {
            return gebraucht;
        }
        let mtm = self.mtm();
        // Die beschreibbare Spalte bekommt die Unterklasse [`Namensfeld`]: ihre
        // Zelle ist zugleich der Editor des Umbenennens und muss das
        // Ordnerzeichen beim Beginn einer Bearbeitung ablegen. Die drei
        // uebrigen Spalten bleiben, was sie waren. Es ist dieselbe Bedingung
        // wie beim `setEditable(true)` weiter unten, weil es dieselbe Sache
        // ist.
        let feld = if spalte.beschreibbar() {
            Retained::into_super(Namensfeld::neu(mtm))
        } else {
            NSTextField::labelWithString(ns_string!(""), mtm)
        };
        feld.setIdentifier(Some(kennung));
        feld.setAlignment(ausrichtung(spalte));
        feld.setMaximumNumberOfLines(1);
        if spalte.beschreibbar() {
            // Das Umbenennen "direkt in der Liste" aus C4. Gesetzt wird es
            // einmal beim Bau und nicht je Zeichendurchgang: die Kennung der
            // Zellenansicht ist die der Spalte, ein Feld der Namensspalte
            // kommt also nur wieder in die Namensspalte, und mit ihm seine
            // Aktion.
            feld.setEditable(true);
            // **Das Ziel hat seit dem Ordnerzeichen zwei Abnehmer statt einen**
            // — wie `clickedRow` im Modulkopf, und aus demselben Grund steht es
            // hier: [`Namensfeld::delegierter`] liest dasselbe `target`
            // zurueck, weil es der einzige Weg von der Zelle zu ihrem
            // Delegierten ist. Drei Ueberschreibungen der Zelle gehen darueber
            // (`becomeFirstResponder`, `textDidEndEditing:`, `abortEditing`),
            // und alle drei tragen Zusage 3 des Nutzerentscheids vom
            // 260815-2058. Wer das Ziel umhaengt oder die Aktion an eine
            // andere Stelle zieht, nimmt der Zelle **still** ihren
            // Delegierten: `delegierter()` liefert dann `None`, und die drei
            // Methoden fallen durch ihr `if let`, ohne dass etwas meldet.
            //
            // SAFETY: Ziel ist der Delegierte, den `Dateifenster` festhaelt;
            // die Aktion ist die Methode, die er oben ausdruecklich fuer diesen
            // Zweck traegt. `NSControl` haelt sein Ziel schwach, und der
            // Delegierte ueberlebt das Feld: er haelt die Tabelle mittelbar
            // ueber die Quelle. Das Zuruecklesen traegt dieselbe Bindung:
            // `NSControl.target` ist `@property (nullable, weak) id`
            // (`NSControl.h:24`, ohne `API_AVAILABLE`), also nullend — ein
            // gestorbener Delegierter liefert `nil` und keinen Absturz.
            unsafe {
                feld.setTarget(Some(self));
                feld.setAction(Some(sel!(umbenennungBeendet:)));
            }
        }
        feld
    }
}

define_class!(
    /// Die Zelle der Namensspalte: sie zeigt die Anzeigeform und gibt zum
    /// Bearbeiten den wirklichen Namen her.
    ///
    /// **Warum es diese Unterklasse gibt.** Dasselbe `NSTextField` ist Zelle
    /// und Editor des Umbenennens aus C4. Seit dem Ordnerzeichen sind
    /// angezeigter und wirklicher Name nicht mehr dieselbe Zeichenkette, und
    /// der Beginn einer Bearbeitung muss deshalb abgefangen werden. **AppKit
    /// meldet ihn nirgends sonst:** `control:textShouldBeginEditing:` des
    /// schon angenommenen `NSControlTextEditingDelegate` kommt beim Einstieg
    /// in die Bearbeitung nicht, sondern erst beim ersten Aendern des Textes
    /// (gemessen am 260815 an einer `NSTableView` mit bearbeitbarer Zelle:
    /// nach `editColumn:row:withEvent:select:` steht der Feldeditor, und der
    /// Haken ist nicht gerufen worden). `becomeFirstResponder` dagegen ist der
    /// eine Weg in die Bearbeitung, gleich ob sie vom Tastenbefehl oder vom
    /// Klick ins Feld ausgeht: AppKit haengt den Feldeditor genau dort ein.
    ///
    /// **Keine der drei Zusagen dieser Klasse steht in einer Probe, und der
    /// Grund ist nicht Nachlaessigkeit:** eine laufende Bearbeitung braucht
    /// einen Feldeditor, ein Feldeditor braucht ein Fenster, und `NSWindow`
    /// wirft ausserhalb des Hauptfadens. `libtest` gibt ihn nicht her
    /// (`issues/260810-1001_*_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`),
    /// und `MainThreadMarker::new_unchecked` behauptet ihn nur gegenueber
    /// Rust, nicht gegenueber AppKit — die Behauptung traegt eine
    /// `NSTextView`, ein `NSWindow` nicht. Gemessen wurde am 260815 mit einem
    /// weggeworfenen Programm auf dem wirklichen Hauptfaden; in `cargo test`
    /// stehen allein die reinen Regeln [`namensform`] und
    /// [`ohne_ordnerzeichen`]. Was die Anwendung am Ende zeigt, nimmt der
    /// Nutzer ab.
    ///
    /// # Die beiden Enden einer Bearbeitung
    ///
    /// AppKit hat genau zwei, und diese Klasse ueberschreibt beide:
    /// `textDidEndEditing:` und `abortEditing`. Am 260816 auf macOS 15.7.7 mit
    /// einem weggeworfenen Programm auf dem wirklichen Hauptfaden gemessen, an
    /// einer `NSTableView` in einer `NSScrollView` mit derselben Verdrahtung
    /// wie hier:
    ///
    /// | Anlass | Bearbeitung danach | Rueckrufe in dieser Reihenfolge |
    /// |---|---|---|
    /// | Return (`insertNewline:`) | beendet | `textDidEndEditing:` → Aktion `umbenennungBeendet:` |
    /// | Escape (`cancelOperation:`) | beendet | `abortEditing` |
    /// | Fokusverlust (`makeFirstResponder:` auf die Tabelle) | beendet | `textDidEndEditing:` |
    /// | `reloadData` | beendet | `textDidEndEditing:` |
    /// | `reloadDataForRowIndexes:columnIndexes:` | beendet | `textDidEndEditing:` |
    /// | `selectRowIndexes:byExtendingSelection:` | beendet | `textDidEndEditing:` |
    /// | `noteNumberOfRowsChanged` | steht weiter | — |
    ///
    /// **Die Aktion kommt aus `textDidEndEditing:` heraus und nicht davor.**
    /// Das ist der Grund, aus dem beide Stuecke, die nach dem Ende zu tun sind,
    /// hinter `super` stehen und allein das Kennzeichen des aufgeschobenen
    /// Lesens davor faellt; die Begruendung im Einzelnen steht bei
    /// [`Self::bearbeitung_beendet`].
    ///
    /// **Die Aktion kommt nur nach Return, und die uebrigen sechs Zeilen der
    /// Tabelle sind Enden ohne Umbenennung.** Sie stellen seit dem
    /// Nutzerentscheid vom 260816-0935 die Anzeigeform wieder her, ueber
    /// [`DateifensterQuelle::anzeigeform_herstellen`] und damit ueber dieselbe
    /// Methode wie Escape. Ein Klick neben die offene Zelle verwirft damit, wie
    /// Escape verwirft.
    ///
    /// **`controlTextDidEndEditing:` waere die falsche Stelle**, obwohl das
    /// Protokoll schon angenommen ist: die Delegiertenmeldung kommt vor der
    /// Aktion, und wer die Zeile dort neu zeichnete, naehme der Aktion ihren
    /// getippten Text.
    ///
    /// Ab welchem macOS die vier ueberschriebenen und angesprochenen Methoden
    /// stehen, sagt der Modulkopf.
    // SAFETY:
    // - Die Oberklasse `NSTextField` stellt an eine Unterklasse keine
    //   Bedingungen, die diese Klasse nicht erfuellt: sie fuegt keine
    //   Zustandsvariablen hinzu und ruft in beiden Ueberschreibungen die
    //   Fassung der Oberklasse.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSTextField)]
    #[thread_kind = MainThreadOnly]
    #[ivars = ()]
    pub struct Namensfeld;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Namensfeld {}

    impl Namensfeld {
        /// Nimmt das Ordnerzeichen weg, bevor die Bearbeitung beginnt.
        ///
        /// Die Reihenfolge traegt die Zusage: `stringValue` wird **vor** dem
        /// Aufruf der Oberklasse gesetzt, denn erst diese haengt den
        /// Feldeditor ein und fuellt ihn aus der Zelle. Danach zu setzen
        /// hilft nicht, im Gegenteil — ein `setStringValue:` waehrend der
        /// Bearbeitung schreibt in den Feldeditor zurueck (am 260815
        /// gemessen, am 260816 wiederholt).
        ///
        /// **Ein Zeichendurchgang der Zeile ist trotzdem kein Weg dorthin**,
        /// und wer beides zusammenliest, kommt sonst zu dem Fehlbefund
        /// `shared/issues/260815-2203_*_…`: AppKit reicht dem Delegierten nie
        /// eine Zelle mit offenem Feldeditor. Die Messung dazu steht bei
        /// [`DateifensterDelegierter::zellenansicht`].
        ///
        /// Die Auswahl des Textes richtet AppKit danach selbst ein: der
        /// Tastenbefehl kommt mit `select: true` und hat damit den ganzen
        /// Namen ausgewaehlt, der Klick setzt die Schreibmarke an die
        /// geklickte Stelle.
        ///
        /// **Der abgelehnte Rang nimmt zurueck, was diese Methode genommen
        /// hat.** Warum der Zweig dasteht, obwohl kein Weg zum Nein gemessen
        /// ist, steht als Kommentar am `match` darunter.
        // SAFETY: Die Signatur ist die von `NSResponder`: kein Argument, ein
        // `BOOL` zurueck.
        #[unsafe(method(becomeFirstResponder))]
        fn wird_ersthelfer(&self) -> bool {
            let anzeige = self.stringValue().to_string();
            // `true` heisst: diese Methode hat die Zelle umgestellt und ist
            // damit diejenige, die sie zuruecksetzt, wenn nichts daraus wird.
            let abgelegt = if let Some(name) = ohne_ordnerzeichen(&anzeige) {
                self.setStringValue(&NSString::from_str(name));
                true
            } else {
                false
            };
            // SAFETY: `becomeFirstResponder` von `NSResponder` hat die hier
            // angenommene Signatur.
            let angenommen: bool = unsafe { msg_send![super(self), becomeFirstResponder] };
            // **Beide Ausgaenge der Oberklasse, und der zweite ist der Grund
            // fuer diese Fallunterscheidung.** Lehnt sie ab, steht kein
            // Feldeditor und es beginnt keine Bearbeitung; das Zeichen ist
            // trotzdem schon weg, denn es **muss** vor `super` weg. Die Zelle
            // stuende dann bis zum naechsten Zeichendurchgang ohne ihr
            // Kennzeichen da.
            //
            // **Gemessen ist dieser Ausgang nicht**, und der Satz gehoert
            // dazu: fuer ein beschreibbares Feld in einem Schluesselfenster
            // liefert `becomeFirstResponder` "ja", und ein Weg zum Nein ist in
            // diesem Baum nirgends gezeigt (Befund
            // `shared/issues/260815-2207_*_…`). Der Zweig steht nicht als
            // Schutz vor einem bekannten Fall, sondern weil eine Methode
            // aufraeumt, was sie selbst umgestellt hat, sobald der Schritt
            // scheitert, fuer den sie es umgestellt hat. Er kostet keinen
            // neuen Mechanismus: die Zeichenkette liegt in `anzeige` schon
            // vor, und `setStringValue:` steht zehn Zeilen darueber.
            //
            // **`textShouldBeginEditing:` waere die genauere Tuer und hilft
            // nicht.** `NSText` stellt die Frage erst beim ersten *Aendern*
            // des Textes und nicht beim Erscheinen des Feldeditors — dieselbe
            // Messung, an der die Delegiertenfassung
            // `control:textShouldBeginEditing:` gescheitert ist (Kopf von
            // [`Namensfeld`]). Der Schraegstrich stuende dann bis zum ersten
            // Tastendruck im Editor.
            match (angenommen, abgelegt) {
                // Erst wenn die Oberklasse angenommen hat, steht der
                // Feldeditor; ein abgelehnter Ersthelferrang ist keine
                // Bearbeitung. Ein `Namensfeld` gibt es nur in der
                // beschreibbaren Spalte, die Annahme ist also zugleich der
                // Beginn einer Umbenennung. Ob die Zelle vorher ein Zeichen
                // abgelegt hatte, aendert daran nichts: eine Datei trug nie
                // eines, und ihre Bearbeitung beginnt genauso.
                (true, _) => {
                    if let Some(delegierter) = self.delegierter() {
                        delegierter.namensbearbeitung_begonnen();
                    }
                }
                // Abgelehnt, und diese Methode hatte das Zeichen weggenommen:
                // sie holt es zurueck. Der Kopf sagt, warum der Zweig
                // dasteht, obwohl kein Weg hierher gemessen ist.
                (false, true) => self.setStringValue(&NSString::from_str(&anzeige)),
                // Abgelehnt, und nichts weggenommen: die Zelle steht
                // unveraendert da, es gibt nichts zurueckzuholen.
                (false, false) => {}
            }
            angenommen
        }

        /// Traegt jedes Ende der Bearbeitung ausser Escape (C4).
        ///
        /// **Hier und nicht in `controlTextDidEndEditing:`**: die
        /// Delegiertenmeldung kommt **vor** der Aktion, diese Methode
        /// **schickt** sie. Beide Stellungen werden hier gebraucht, und nur
        /// von hier aus sind sie zu haben.
        ///
        /// # Die drei Stuecke und ihre Reihenfolge
        ///
        /// **Vor `super` faellt das Kennzeichen** der Namensbearbeitung, damit
        /// die Umbenennung, die `super` ausloest, ihre eigene Auffrischung
        /// nicht am Aufschub verliert; die Begruendung im Einzelnen steht bei
        /// [`DateifensterQuelle::namensbearbeitung_beendet`].
        ///
        /// **Nach `super` steht die Anzeigeform**, denn ein Zeichendurchgang
        /// davor naehme dem Feld seine Zeile, und
        /// [`DateifensterQuelle::umbenennung_beenden`] fande ueber
        /// `rowForView:` nichts mehr. Das ist die Umsetzung des
        /// Nutzerentscheids vom 260816-0935: ein Ende ohne Umbenennung
        /// verwirft, und verworfen wird sichtbar erst dadurch, dass die Zelle
        /// den getippten Text wieder hergibt.
        ///
        /// **Danach erst das Nachholen.** Beide Reihenfolgen halten die Zusage,
        /// aber nur diese laesst den Zeichendurchgang sie selbst tragen: laeuft
        /// das Nachholen zuerst, nimmt sein `reloadData` dem Feld die Zeile
        /// (`rowForView` = -1 am 260816 gemessen, Messung H), der
        /// Zeichendurchgang faellt still aus, und die Anzeigeform haengt daran,
        /// dass ueberhaupt etwas vorgemerkt war.
        ///
        /// **In die Quere kommen sich die beiden nicht.** Ein
        /// Zeichendurchgang ist kein Lesevorgang: er ruft
        /// [`DateifensterQuelle::zeile_neu_zeichnen`], nicht
        /// [`DateifensterQuelle::neu_lesen`], fasst das Kennzeichen
        /// `auffrischung_vorgemerkt` nicht an und liest allein das Modell.
        /// Die zwei Lesevorgaenge hintereinander, wegen derer es das
        /// Kennzeichen gibt, entstehen daraus nicht (am 260816 gemessen,
        /// Messung D).
        // SAFETY: Die Signatur ist die von `NSTextField`: ein
        // `NSNotification`, kein Rueckgabewert.
        #[unsafe(method(textDidEndEditing:))]
        fn bearbeitung_beendet(&self, meldung: &NSNotification) {
            let delegierter = self.delegierter();
            if let Some(delegierter) = delegierter.as_ref() {
                delegierter.namensbearbeitung_beendet();
            }
            // SAFETY: `textDidEndEditing:` von `NSTextField` hat die hier
            // angenommene Signatur.
            unsafe { msg_send![super(self), textDidEndEditing: meldung] }
            if let Some(delegierter) = delegierter.as_ref() {
                delegierter.anzeigeform_herstellen(self);
                delegierter.aufgeschobene_auffrischung_nachholen();
            }
        }

        /// Holt die Anzeigeform zurueck, nachdem Escape die Bearbeitung
        /// verworfen hat.
        ///
        /// **Escape kommt hier an und nirgends sonst.** Der Feldeditor
        /// beantwortet die Taste mit `cancelOperation:`, und `NSTextField`
        /// macht daraus diesen Aufruf; `controlTextDidEndEditing:` bleibt
        /// dabei aus (am 260815 an derselben Tabelle gemessen). Die
        /// Oberklasse stellt den Stand vor der Bearbeitung wieder her, und
        /// das ist seit [`Self::wird_ersthelfer`] der Name ohne Zeichen.
        ///
        /// Zurueckgeholt wird die Anzeigeform ueber einen Zeichendurchgang der
        /// Zeile und nicht durch ein angehaengtes Zeichen: das Modell ist die
        /// eine Quelle der Anzeigeform, und derselbe Weg laesst schon eine
        /// abgelehnte Eingabe verschwinden
        /// ([`DateifensterQuelle::zeile_neu_zeichnen`]). Gerufen wird dafuer
        /// [`DateifensterQuelle::anzeigeform_herstellen`], und zwar dieselbe
        /// Methode, die seit dem 260816 auch [`Self::bearbeitung_beendet`]
        /// ruft: die Zusage gilt jedem Ende ohne Umbenennung, also gehoert sie
        /// an eine Stelle und nicht an zwei.
        // SAFETY: Die Signatur ist die von `NSControl`: kein Argument, ein
        // `BOOL` zurueck.
        #[unsafe(method(abortEditing))]
        fn bearbeitung_abbrechen(&self) -> bool {
            // SAFETY: `abortEditing` von `NSControl` hat die hier angenommene
            // Signatur.
            let abgebrochen: bool = unsafe { msg_send![super(self), abortEditing] };
            // **Der Rueckgabewert entscheidet und nicht der Aufruf.** AppKit
            // ruft `abortEditing` auch an Feldern, die gar nichts bearbeiten
            // — bei jedem `reloadData` etwa —, und dort ist nichts zu Ende
            // gegangen (am 260816 am wirklichen Hauptfaden gemessen).
            if abgebrochen && let Some(delegierter) = self.delegierter() {
                delegierter.namensbearbeitung_beendet();
                delegierter.anzeigeform_herstellen(self);
                delegierter.aufgeschobene_auffrischung_nachholen();
            }
            abgebrochen
        }
    }
);

impl Namensfeld {
    /// Ein leeres Feld der Namensspalte.
    ///
    /// **Ueber `+labelWithString:` und nicht ueber `initWithFrame:`**, damit
    /// diese Zelle in jeder Eigenschaft die des Vorbilds bleibt, das die drei
    /// uebrigen Spalten weiter unmittelbar bauen. Die Sammelmethode legt ueber
    /// die empfangende Klasse an und liefert deshalb ein `Namensfeld`; am
    /// 260815 gemessen, zusammen mit dem Vergleich beider Wege: sie
    /// unterscheiden sich in `alignment` und `textColor`, und beide setzt
    /// [`DateifensterDelegierter::feld`] beziehungsweise
    /// [`DateifensterDelegierter::zellenansicht`] ohnehin selbst.
    fn neu(mtm: MainThreadMarker) -> Retained<Self> {
        // Der Nachweis des Hauptfadens geht in den Aufruf ein: `Namensfeld`
        // ist `MainThreadOnly`, und diese Methode legt eine Instanz an.
        let _ = mtm;
        // SAFETY: `+labelWithString:` von `NSTextField` nimmt eine
        // Zeichenkette und liefert eine Instanz der empfangenden Klasse,
        // hier also ein `Namensfeld`. Der Rueckgabewert ist autofreigegeben;
        // `Retained` als Ergebnistyp sagt `msg_send!`, dass es ihn festhaelt.
        unsafe { msg_send![Self::class(), labelWithString: ns_string!("")] }
    }

    /// Der Delegierte des Dateifensters, in dem diese Zelle steht.
    ///
    /// Er ist das Ziel der Aktion, die [`DateifensterDelegierter::feld`] an
    /// dieses Feld haengt; die Zelle liest ihn zurueck, statt eine zweite
    /// Verbindung zu halten. `None`, solange das Feld noch keins hat.
    ///
    /// **Damit ist diese Methode der zweite Abnehmer des Ziels**, und die
    /// Setzstelle in [`DateifensterDelegierter::feld`] sagt es dort ebenfalls:
    /// wer das Ziel umhaengt oder die Aktion an eine andere Stelle zieht,
    /// nimmt der Zelle ihren Delegierten und damit alle drei Zusagen des
    /// Nutzerentscheids vom 260815-2058.
    ///
    /// **`None` bleibt hier ohne Meldung**, obwohl dieses Projekt kein stilles
    /// Fallenlassen kennt. Der Grund ist keine Nachsicht, sondern der einzige
    /// Weg, den es gaebe: eine Meldung landete in der Statuszeile, und die
    /// erreicht diese Zelle allein ueber die Quelle, an die sie nur ueber
    /// genau diesen Delegierten kommt. Wer `None` melden wollte, brauchte das,
    /// was `None` gerade sagt, dass es fehlt. Gehalten wird der Fall deshalb
    /// nicht von einer Meldung, sondern von der Notiz an der Setzstelle.
    fn delegierter(&self) -> Option<Retained<DateifensterDelegierter>> {
        let ziel = self.target()?;
        ziel.downcast::<DateifensterDelegierter>().ok()
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
    /// Baut Tableiste, Tabelle, Bildlaufansicht, Datenquelle und Delegierten.
    ///
    /// **Ohne Statuszeile seit der Runde 6**: es gibt eine ueber die volle
    /// Fensterbreite, und die baut der Anwendungsdelegierte.
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

        let quelle = DateifensterQuelle::neu(mtm, tabelle.clone(), sicht.clone(), tabs);
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

        // Der Abwurf aus einer fremden Anwendung (C4 bis C7 der Runde 13).
        // Welche Sorten angemeldet werden und warum die Zusagesorten dabei
        // sind, obwohl KRK jede Zusagedatei abweist, sagt `abwurf::sorten`:
        // eine nicht angemeldete Sorte erreicht die Ansicht nie, und KRK
        // bekaeme keine Gelegenheit, etwas dazu zu sagen.
        //
        // **Die Lesezeichen- und Geraeteleiste bleibt aussen vor.** Sie meldet
        // sich nicht an, und weil die beiden Tabellen keine Zeile Code teilen,
        // verlangt das keine Abwehr, sondern nur das Unterlassen; eine Probe
        // unten haelt die Zahl der Anmeldestellen im Baum bei eins.
        //
        // **Ohne `unsafe`-Block, und das ist gemessen und nicht angenommen:**
        // `objc2` bindet `registerForDraggedTypes:` sicher
        // (`objc2-app-kit-0.3.2/src/generated/NSView.rs:1412-1414`, ein `pub fn`
        // ohne `unsafe`), weil die Methode ausser einer Liste von Sortennamen
        // nichts verlangt. Ein Block davor waere ein `unused_unsafe` und
        // brauchte einen SAFETY-Satz, der nichts zu tragen haette. Die Ansicht
        // kopiert die Liste; das `Retained` darf danach fallen. Die Methode
        // steht seit macOS 10.0 (`NSView.h:488`, ohne `API_AVAILABLE`), die
        // Untergrenze des Buendels ist 15.0.
        tabelle.registerForDraggedTypes(&abwurf::sorten());

        // Der Doppelklick aus C3 der Runde 4. Er steht hier und nicht bei den
        // Kommandos, weil er keines ist: er bekommt keinen Eintrag in
        // `resources/default-keymap.toml` und keine Variante in `Kommando`.
        //
        // SAFETY: Ziel ist der Delegierte, den `Dateifenster` festhaelt; die
        // Aktion ist die Methode, die er oben ausdruecklich fuer diesen Zweck
        // traegt. Ueber die Lebensdauer verlangt die Bindung nichts.
        //
        // **Es entsteht kein Haltering, und das ist gemessen und nicht
        // angenommen.** `NSTableView` erklaert `target` nicht selbst, sondern
        // erbt es von `NSControl` (`NSTableView : NSControl`,
        // `objc2-app-kit-0.3.2/src/generated/NSTableView.rs:242`), und dort
        // steht ueber dem Setzer dieselbe Art, die den Block oben fuer
        // `dataSource` und `delegate` traegt: "This is a weak property"
        // (`objc2-app-kit-0.3.2/src/generated/NSControl.rs:91-93`). Die
        // Gegenprobe am Kopf des Systems sagt dasselbe und nennt die Bedingung:
        // `@property (nullable, weak) id target;` mit dem Zusatz "Target is
        // weak for zeroing-weak compatible objects in apps linked on 10.10 or
        // later"
        // (`MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/NSControl.h:24`).
        // KRK bindet gegen 15.0 und faellt damit auf keinen Fall in das alte
        // `assign`.
        //
        // **Die Wurzel dieses Pfades ist der SDK-Kopf und nicht das laufende
        // System.** `xcrun --show-sdk-path` nennt sie; unter
        // `/System/Library/Frameworks/` liegt auf diesem Geraet kein
        // `Headers`-Ordner, und wer den Beleg dort sucht, findet ihn nicht.
        // Dieselbe Schreibweise nimmt [`crate::hervorhebung`] fuer seinen
        // Beleg aus `NSLayoutManager.h`; die uebrigen Fundstellen dieses
        // Moduls nennen nur den Dateinamen und meinen denselben Ordner.
        //
        // Der Ring Quelle → Tabelle → Ziel → Delegierter → Quelle bleibt also
        // an der Kante Tabelle → Ziel offen; ein schwach haltendes
        // Zwischenobjekt nach dem Vorbild des Rueckrufs der Tableiste weiter
        // unten braucht dieser Weg nicht. Der Halter des Ziels ist
        // `Dateifenster::delegierter`, wie fuer das Namensfeld auch.
        //
        // Alle drei angesprochenen Stellen stehen seit macOS 10.0 zur
        // Verfuegung und tragen im Kopf des Systems kein `API_AVAILABLE`:
        // `NSControl::setTarget:` (`NSControl.h:24`), `NSTableView`s
        // `setDoubleAction:` und `clickedRow` (`NSTableView.h:275-278`). Die
        // Untergrenze des Buendels ist 15.0.
        unsafe {
            tabelle.setTarget(Some(&*delegierter));
            tabelle.setDoubleAction(Some(sel!(doppelklick:)));
        }

        // Das Kontextmenue aus C1 der Runde 6. Es entsteht hier leer und
        // bekommt seine vier Eintraege erst beim Rechtsklick, in
        // `menuNeedsUpdate:` oben; ein Menue mit festem Bestand koennte die
        // betroffenen Eintraege nicht nennen, weil die sich zwischen zwei
        // Klicks aendern. Seit der Runde 17 sind es vier statt einem: der
        // Freigabeeintrag des Systems und die drei eigenen Befehle.
        //
        // SAFETY: `setMenu:` ist als Setzer einer `strong`-Eigenschaft
        // unsicher gebunden und verlangt nichts weiter, als dass das Menue
        // eines ist. Die Tabelle haelt es danach; das `Retained` hier darf
        // fallen. Der Delegierte wird schwach gehalten, siehe den Block an
        // `unsafe impl NSMenuDelegate` oben.
        let kontextmenue = NSMenu::new(mtm);
        kontextmenue.setDelegate(Some(ProtocolObject::from_ref(delegierter.quelle())));
        unsafe { tabelle.setMenu(Some(&kontextmenue)) };

        // Die Leiste zuletzt: ihr Rueckruf braucht die Quelle. Er haelt sie
        // **schwach**, sonst schloesse sich der Ring Quelle → Leiste → Ziel →
        // Rueckruf → Quelle.
        //
        // **Er fordert den Ersthelferrang an, und darin unterscheidet er sich
        // vom Klick auf eine Zeile.** Die Leiste ist ein `NSSegmentedControl`,
        // und das nimmt den Rang bei einem Klick nicht an; ohne diese
        // Anforderung bliebe er in der Liste sitzen, die nach dem Klick nicht
        // mehr die aktive ist. Die Begruendung im Einzelnen steht an
        // [`Rangmitnahme`].
        let schwach = objc2::rc::Weak::from_retained(&delegierter.quelle().retain());
        let leiste = Tableiste::bauen(mtm, move |stelle| {
            if let Some(quelle) = schwach.load() {
                quelle.angefasst(Rangmitnahme::Krk);
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

    /// Die Datenquelle, ueber die ein Ordner gelesen wird.
    pub fn quelle(&self) -> &DateifensterQuelle {
        self.delegierter.quelle()
    }

    /// Die Liste selbst, fuer den Fokuswechsel aus C5.
    ///
    /// Nicht die Bildlaufansicht aus [`Dateifenster::sicht`]: den Eingabefokus
    /// traegt die `NSTableView` darin, und `makeFirstResponder:` will genau
    /// die. Sie wird sonst nirgends nach aussen gereicht.
    pub fn liste(&self) -> &NSTableView {
        &self.quelle().ivars().tabelle
    }

    /// Blendet eine Spalte aus oder wieder ein (C3 der Bereichsleisten-Runde).
    ///
    /// **Die Breiten bleiben dabei ungerechnet.** Wer schaltet, ruft danach
    /// [`Dateifenster::spaltenbreiten_verteilen`]; warum das noetig ist, steht
    /// dort.
    ///
    /// **Eine verborgene Spalte bleibt eine Spalte.** `setHidden:` nimmt sie
    /// weder aus `tableColumns` noch aus `numberOfColumns` — der Kopf des
    /// Systems sagt es ausdruecklich (`NSTableColumn.h:78`) —, und das traegt
    /// zwei Zusagen der Runde ohne einen einzigen Zweig: der Sortierschluessel
    /// bleibt stehen, auch wenn seine Spalte verborgen ist (Kriterium C3.3),
    /// und die Datenquelle liefert weiter dieselben Zellen. Ein `removeTableColumn:`
    /// taete beides nicht.
    ///
    /// Gesucht wird ueber die Kennung, weil sie das eine ist, was diese Klasse
    /// und die Aufzaehlung [`Spalte`] gemeinsam haben; der Weg dorthin ist
    /// dieselbe Funktion [`kennung`], die den Kopf beim Aufbau benannt hat.
    /// Findet sich keine Spalte, geschieht nichts: dann hat der Aufbau sie nicht
    /// angelegt, und das waere ein Fehler dort und keiner hier.
    pub fn spalte_verbergen(&self, spalte: Spalte, verborgen: bool) {
        let liste = self.liste();
        let Some(kopf) = liste.tableColumnWithIdentifier(kennung(spalte)) else {
            return;
        };
        kopf.setHidden(verborgen);
    }

    /// Setzt die sichtbaren Spalten auf ihre natuerliche Breite und gibt der
    /// Namensspalte, was bis zur Sichtflaeche fehlt.
    ///
    /// **Die Regel stammt vom Nutzer**, Moeglichkeit 1 seines Entscheids vom
    /// 260812-0910:
    /// `shared/decisions/260812-0910_*_wie-werden-die-spaltenbreiten-nach-dem-wegschalten-verteilt.md`.
    /// Groesse 80, Datum 130 und Typ 90 Punkte stehen bei **jeder**
    /// Schalterstellung gleich, damit dieselbe Spalte immer an derselben Stelle
    /// und in derselben Breite steht; Name nimmt den Rest und faellt dabei nicht
    /// unter seine Mindestbreite. Eine Verhaeltnisrechnung ueber alle Spalten
    /// und eine Deckelung der schmalen sind ausdruecklich abgelehnt worden.
    ///
    /// **Was AppKit von sich aus tut, und warum das zu wenig ist.** Gemessen am
    /// 260812 an einer Tabelle ohne Fenster, mit denselben Breiten und derselben
    /// Betriebsart wie hier:
    ///
    /// - Aendert sich die **Sichtflaeche**, trifft
    ///   `FirstColumnOnlyAutoresizingStyle` diese Regel schon von selbst: bei 900
    ///   Punkten steht Name auf 537, bei 500 auf 137, bei 400 auf seiner
    ///   Mindestbreite 100, und die drei schmalen ruehrt AppKit nicht an.
    /// - Wird dagegen eine Spalte **verborgen**, haelt AppKit die Gesamtbreite
    ///   der Tabelle fest und schlaegt die frei werdenden Punkte samt einem
    ///   Zellenabstand der Namensspalte zu: bei 700 Punkten Sichtflaeche waechst
    ///   Name von 337 auf 434 (Groesse weg) und auf 541 (Typ zusaetzlich weg).
    ///   Der Gewinn erreicht die Sichtflaeche also nie. Steht die Tabelle
    ///   vorher schon breiter als ihre Sichtflaeche — vier Spalten brauchen
    ///   rund 603 Punkte, zwei Dateifenster nebeneinander sind schmaler —, dann
    ///   bleibt sie es auch nach dem Wegschalten, und die letzte sichtbare
    ///   Spalte steht weiter ausserhalb des Bildes. Genau das war der Defekt
    ///   `shared/issues/260812-0907_*`.
    ///
    /// **Gemessen wird ueber `rectOfColumn:` und nicht gerechnet.** Zwischen der
    /// Summe der Spaltenbreiten und der Breite der Tabelle liegen der
    /// Zellenabstand je Spalte und die Randpolsterung, die `NSTableViewStyle`
    /// zusetzt (`NSTableView.h:81`: "content padding ... independent of
    /// intercellSpacing"); beide sind nirgends zugesagt. Das Feld der letzten
    /// sichtbaren Spalte traegt sie fertig, und sein rechter Rand ist damit die
    /// eine Zahl, die zaehlt. Eine verborgene Spalte liefert ein leeres Feld,
    /// deshalb fragt der Durchgang vorher nach `isHidden`.
    ///
    /// Erst danach steht die Namensspalte fest, denn ihr Zuwachs verschiebt den
    /// Rand, an dem er gemessen wird: der erste Durchgang setzt sie auf ihre
    /// natuerliche Breite, der zweite misst, der dritte legt sie fest.
    pub fn spaltenbreiten_verteilen(&self) {
        let liste = self.liste();
        for spalte in Spalte::ALLE {
            let Some(kopf) = liste.tableColumnWithIdentifier(kennung(spalte)) else {
                continue;
            };
            if kopf.isHidden() {
                continue;
            }
            kopf.setWidth(breiten(spalte).0);
        }

        let mut rand: f64 = 0.0;
        for spalte in Spalte::ALLE {
            let Some(kopf) = liste.tableColumnWithIdentifier(kennung(spalte)) else {
                continue;
            };
            if kopf.isHidden() {
                continue;
            }
            let feld = liste.rectOfColumn(liste.columnWithIdentifier(kennung(spalte)));
            rand = rand.max(feld.origin.x + feld.size.width);
        }

        let Some(kopf) = liste.tableColumnWithIdentifier(kennung(Spalte::Name)) else {
            return;
        };
        let (natuerlich, mindestens) = breiten(Spalte::Name);
        let sichtflaeche = self.sicht.contentView().bounds().size.width;
        kopf.setWidth(namensbreite(natuerlich, mindestens, rand, sichtflaeche));
    }
}

/// Die Breite der Namensspalte: ihre natuerliche Breite zuzueglich dessen, was
/// vom rechten Rand der letzten sichtbaren Spalte bis zur Sichtflaeche fehlt.
///
/// Rein und ohne AppKit, damit die Rechnung ohne Fenster zu pruefen ist; die
/// vier Zahlen liest [`Dateifenster::spaltenbreiten_verteilen`] ab.
///
/// **Der Fehlbetrag darf negativ sein.** Passen die sichtbaren Spalten in ihrer
/// natuerlichen Breite nicht mehr in die Sichtflaeche, schrumpft Name bis auf
/// `mindestens` und keinen Punkt weiter; darunter bleibt die Tabelle breiter
/// als ihre Sichtflaeche und `NSScrollView` blendet seinen waagerechten
/// Schieber ein. Die Mindestbreite gewinnt gegen die Verteilung, so steht es im
/// Entscheid.
#[must_use]
fn namensbreite(natuerlich: f64, mindestens: f64, rechter_rand: f64, sichtflaeche: f64) -> f64 {
    (natuerlich + (sichtflaeche - rechter_rand)).max(mindestens)
}

/// Eine Spalte mit Kennung, Ueberschrift und Breiten.
fn spaltenkopf(mtm: MainThreadMarker, spalte: Spalte) -> Retained<NSTableColumn> {
    let (breite, mindestbreite) = breiten(spalte);
    let kopf = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), kennung(spalte));
    kopf.setTitle(&titel(spalte));
    kopf.setWidth(breite);
    kopf.setMinWidth(mindestbreite);
    kopf
}

/// Die Benennung einer Eintragsart.
///
/// Einziger Aufrufer ist die Metadatenanzeige der Vorschau (C6), die die
/// Eintragsart weiterhin zusagt. Die Spalte `Typ` der Tabelle zeigt seit dem
/// Entscheid vom 260806 die Dateiendung und ruft diese Wortliste nicht mehr.
pub(super) fn typ_beschriften(typ: Typ) -> &'static str {
    match typ {
        Typ::Ordner => "Ordner",
        Typ::Datei => "Datei",
        Typ::Verknuepfung => "Verknüpfung",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quellbaum::{aufrufstellen, quelldateien};

    /// Der Pfad dieser Datei in dem Baum, den [`quelldateien`] liest.
    const DIESE_DATEI: &str = "krk-ui/src/appkit/tabelle.rs";

    /// Ein Eintrag der genannten Art. Groesse und Zeitpunkt spielen fuer die
    /// Anzeigeform der Namensspalte keine Rolle.
    fn eintrag(name: &str, typ: Typ) -> Eintrag {
        Eintrag::neu(name.to_owned(), 0, UNIX_EPOCH, typ)
    }

    /// Die Code-Zeilen einer Quelldatei: alles, was nicht Kommentar ist.
    ///
    /// Die Doc-Kommentare dieser Datei nennen jede Nadel darunter im Klartext,
    /// damit ein Leser weiss, wonach gesucht wird. Gefragt ist aber, wer eine
    /// Sache **tut**, und das steht nie hinter `//`. Dieselbe Unterscheidung
    /// trifft [`aufrufstellen`] fuer sich selbst; hier steht sie noch einmal,
    /// weil die Zaehlung des Zeichens keine Aufrufzaehlung ist.
    fn code_zeilen(inhalt: &str) -> impl Iterator<Item = &str> {
        inhalt
            .lines()
            .filter(|zeile| !zeile.trim_start().starts_with("//"))
    }

    /// Ob eine Zeile den Namen als **ganzen** Bezeichner fuehrt.
    ///
    /// Ein blosses `contains` findet ihn auch mitten in einem laengeren Namen,
    /// und der Baum fuehrt genau so einen Fall: `krk-bench/src/bericht.rs`
    /// nennt eine Probe `der_kurzstempel_passt_zur_namensform_des_projekts`,
    /// und das ist kein zweiter Bau der Anzeigeform, sondern dasselbe deutsche
    /// Wort. Entschieden wird an den beiden Nachbarzeichen: gehoert eines von
    /// ihnen zu einem Bezeichner, faellt die Fundstelle heraus. Dieselbe
    /// Grenze zieht [`aufrufstellen`] fuer ihre Seite.
    fn fuehrt_den_namen(zeile: &str, name: &str) -> bool {
        zeile.match_indices(name).any(|(stelle, _)| {
            let gehoert_dazu = |zeichen: char| zeichen.is_alphanumeric() || zeichen == '_';
            let davor = zeile[..stelle].chars().next_back();
            let danach = zeile[stelle + name.len()..].chars().next();
            !davor.is_some_and(gehoert_dazu) && !danach.is_some_and(gehoert_dazu)
        })
    }

    /// Der Teil einer Quelldatei vor ihrem Pruefmodul.
    ///
    /// Die Proben dieser Datei rufen beide Regeln selbst, und sie sind keine
    /// Aufrufer im Sinne des Nutzerentscheids. Die Bauform stammt von
    /// `code_zeilen_vor_dem_pruefmodul` in `krk-core/tests/verzeichnis.rs`.
    fn vor_dem_pruefmodul(inhalt: &str) -> &str {
        let vermerk = concat!("#[cfg(", "test)]");
        inhalt.split(vermerk).next().unwrap_or(inhalt)
    }

    /// Der Inhalt dieser Datei, so wie die Zaehlproben ihn lesen.
    fn diese_datei() -> String {
        quelldateien()
            .into_iter()
            .find(|(name, _)| name == DIESE_DATEI)
            .map(|(_, inhalt)| inhalt)
            .unwrap_or_else(|| panic!("der Quellbaum fuehrt {DIESE_DATEI} nicht"))
    }

    /// Das Ordnerzeichen entsteht an genau einer Stelle im Baum.
    ///
    /// **Die Zusage ist eine Aussage ueber den Baum und an keinem
    /// Rueckgabewert abzulesen.** Der Nutzerentscheid vom 260815-2058 sagt:
    /// „Er entsteht in `DateifensterDelegierter::beschriften` fuer
    /// `Spalte::Name` und nirgends sonst. Sortierung, Filter, Zwischenablage,
    /// Vorschau und jede Dateioperation lesen weiterhin `eintrag.name`."
    /// Bliebe sie ungezaehlt, faende ein zweiter Rufer, den ein spaeterer Turn
    /// anlegt, keine Pruefung: der Bau bliebe gruen, die beiden Proben darueber
    /// blieben gruen, und die teuerste Zusage dieser Aenderung fiele still
    /// (`shared/issues/260815-2205_*_…`).
    ///
    /// **Eine Aufruferzaehlung und ausdruecklich die richtige Form.** Der Kopf
    /// von [`crate::quellbaum`] warnt vor ihr, wo sie als Stellvertreter fuer
    /// „es gibt keinen Doppelbau" steht; sie gehoert dorthin, wo ein
    /// Abnahmekriterium die **Zahl selbst** zusagt, und hier tut es das.
    /// Faengt sie spaeter einen neuen Rufer, ist die Frage, warum es ihn gibt,
    /// und nicht die Zahl hier.
    ///
    /// Drei Zaehlungen. Die erste haelt fest, dass keine andere Datei des
    /// Baums die drei Namen ueberhaupt fuehrt — sie sind privat, und ein
    /// gleichnamiger Doppelbau anderswo wuerde rot. Die zweite haelt die zwei
    /// Regeln bei je einem Rufer. Die dritte haelt das Zeichen selbst bei drei
    /// Code-Zeilen: seiner Erklaerung und den zwei Regeln, die es tragen.
    ///
    /// **Ihre Blindheit**, und der Satz gehoert dazu: eine zweite Fassung
    /// derselben Regel unter anderem Namen sieht auch diese Zaehlung nicht,
    /// und ein `'/'` als Zeichenkonstante neben [`ORDNERZEICHEN`] ginge ihr
    /// ebenso durch. Der Kopf von [`crate::quellbaum`] sagt, warum keine Suche
    /// im Quelltext das leisten kann. Was sie ausdruecklich **nicht** meldet,
    /// steht bei [`fuehrt_den_namen`]: ein Name, der nur als Wortbestandteil
    /// eines laengeren Bezeichners dasteht, ist keine Fundstelle, und der Baum
    /// fuehrt genau so einen Fall.
    ///
    /// Die Nadeln stehen zusammengesetzt da, weil die Probe in dem Baum liegt,
    /// den sie liest.
    #[test]
    fn das_ordnerzeichen_entsteht_an_genau_einer_stelle() {
        let bilden = concat!("namens", "form");
        let zuruecknehmen = concat!("ohne_ordner", "zeichen");
        let zeichen = concat!("ORDNER", "ZEICHEN");

        for nadel in [bilden, zuruecknehmen, zeichen] {
            let anderswo: Vec<String> = quelldateien()
                .into_iter()
                .filter(|(name, inhalt)| {
                    name != DIESE_DATEI
                        && code_zeilen(inhalt).any(|zeile| fuehrt_den_namen(zeile, nadel))
                })
                .map(|(name, _)| name)
                .collect();
            assert!(
                anderswo.is_empty(),
                "{nadel} steht ausser in {DIESE_DATEI} auch in {anderswo:?}"
            );
        }

        let inhalt = diese_datei();
        let code = vor_dem_pruefmodul(&inhalt);
        assert_eq!(
            aufrufstellen(code, bilden),
            1,
            "die Anzeigeform entsteht nicht an genau einer Stelle; \
             der einzige Rufer ist DateifensterDelegierter::beschriften"
        );
        assert_eq!(
            aufrufstellen(code, zuruecknehmen),
            1,
            "das Zeichen wird nicht an genau einer Stelle zurueckgenommen; \
             der einzige Rufer ist Namensfeld::wird_ersthelfer"
        );
        let zeilen = code_zeilen(code)
            .filter(|zeile| fuehrt_den_namen(zeile, zeichen))
            .count();
        assert_eq!(
            zeilen, 3,
            "das Ordnerzeichen steht nicht in genau drei Code-Zeilen, \
             seiner Erklaerung und den zwei Regeln"
        );
    }

    /// Die Anzeigeform hat genau die zwei Leser, die sie heute hat.
    ///
    /// Die zweite Haelfte derselben Zusage: der Schraegstrich entsteht an einer
    /// Stelle, und was ihn zu sehen bekommt, ist gezaehlt. Ein Namenszellentext
    /// wird an zwei Stellen gelesen, und beide sind Enden einer Bearbeitung —
    /// [`DateifensterQuelle::umbenennung_beenden`] liest den getippten Text
    /// nach Return, [`Namensfeld::wird_ersthelfer`] die Anzeigeform vor dem
    /// Ablegen des Zeichens. Ein dritter Leser waere eine Stelle, an der der
    /// Schraegstrich als Name durchginge, und genau das schliesst der
    /// Nutzerentscheid aus.
    ///
    /// **Gezaehlt wird allein in dieser Datei**, und das ist keine Verengung:
    /// die Namenszelle ist [`Namensfeld`], und die Probe darueber haelt fest,
    /// dass keine andere Datei des Baums sie kennt. Andere `stringValue`-Rufe
    /// im Baum lesen andere Felder.
    ///
    /// **Ihre Blindheit:** ein Leser, der den Text ueber den Feldeditor holt
    /// (`currentEditor`) statt ueber `stringValue`, faellt dieser Nadel nicht
    /// auf.
    ///
    /// Die Nadel steht zusammengesetzt da, aus demselben Grund wie oben.
    #[test]
    fn die_anzeigeform_hat_genau_zwei_leser() {
        let lesen = concat!("string", "Value");
        let inhalt = diese_datei();
        let code = vor_dem_pruefmodul(&inhalt);
        assert_eq!(
            aufrufstellen(code, lesen),
            2,
            "der Text einer Namenszelle hat nicht genau zwei Leser, \
             umbenennung_beenden und wird_ersthelfer"
        );
    }

    #[test]
    fn allein_ein_ordner_traegt_den_schraegstrich() {
        assert_eq!(namensform(&eintrag("Bilder", Typ::Ordner)), "Bilder/");
        assert_eq!(namensform(&eintrag("Ablage.rs", Typ::Datei)), "Ablage.rs");
        // Eine Verknuepfung bekommt keinen, auch wenn sie auf einen Ordner
        // zeigt: die Bedingung ist `Typ::Ordner` und nicht das Verweisziel,
        // sonst stuende ein `stat` je sichtbarer Zeile in der Zeichenschleife
        // der Dateiliste — und die misst keine der zehn Zusagen aus C8. Die
        // Begruendung im Einzelnen steht bei [`namensform`].
        assert_eq!(namensform(&eintrag("Kurz", Typ::Verknuepfung)), "Kurz");
    }

    #[test]
    fn die_anzeigeform_laesst_sich_auf_den_namen_zuruecknehmen() {
        // Der Weg, den [`Namensfeld::wird_ersthelfer`] beim Beginn einer
        // Bearbeitung geht: aus der Anzeigeform wird der wirkliche Name.
        for typ in [Typ::Ordner, Typ::Datei, Typ::Verknuepfung] {
            let eintrag = eintrag("Bilder", typ);
            let anzeige = namensform(&eintrag);
            let name = ohne_ordnerzeichen(&anzeige).unwrap_or(&anzeige);
            assert_eq!(name, eintrag.name, "die Art {typ:?} verliert ihren Namen");
        }
        // Ein Name ohne Zeichen bleibt unberuehrt, und `None` sagt das an.
        assert_eq!(ohne_ordnerzeichen("Ablage.rs"), None);
        // Ein Schraegstrich mitten im Text bleibt stehen: nur der letzte ist
        // das Kennzeichen. In einem Dateinamen kann ohnehin keiner stehen.
        assert_eq!(ohne_ordnerzeichen("a/b"), None);
        assert_eq!(ohne_ordnerzeichen("a/b/"), Some("a/b"));
    }

    #[test]
    fn jede_spalte_findet_sich_ueber_ihre_kennung_wieder() {
        for spalte in Spalte::ALLE {
            assert_eq!(aus_kennung(kennung(spalte)), Some(spalte));
        }
        assert_eq!(aus_kennung(ns_string!("unbekannt")), None);
    }

    #[test]
    fn die_namensspalte_nimmt_auf_was_bis_zur_sichtflaeche_fehlt() {
        let (natuerlich, mindestens) = breiten(Spalte::Name);
        // Vier Spalten in ihrer natuerlichen Breite enden bei 603 Punkten
        // (gemessen am 260812); in einer Sichtflaeche von 700 fehlen 97.
        assert_eq!(namensbreite(natuerlich, mindestens, 603.0, 700.0), 337.0);
        // Ohne Groesse und Typ endet dieselbe Reihe bei 399 Punkten.
        assert_eq!(namensbreite(natuerlich, mindestens, 399.0, 500.0), 341.0);
        // Passt die Reihe genau, bleibt es bei der natuerlichen Breite.
        assert_eq!(
            namensbreite(natuerlich, mindestens, 603.0, 603.0),
            natuerlich
        );
    }

    #[test]
    fn die_mindestbreite_gewinnt_gegen_die_verteilung() {
        let (natuerlich, mindestens) = breiten(Spalte::Name);
        // Eine Sichtflaeche, die schmaler ist als die Reihe: Name schrumpft bis
        // auf seine Mindestbreite und keinen Punkt weiter.
        assert_eq!(
            namensbreite(natuerlich, mindestens, 603.0, 400.0),
            mindestens
        );
        // Auch die Flaeche 0, die beim Aufbau vor der ersten Aufteilung steht,
        // liefert eine gueltige Breite und keine negative.
        assert_eq!(namensbreite(natuerlich, mindestens, 0.0, 0.0), natuerlich);
        assert_eq!(namensbreite(natuerlich, mindestens, 603.0, 0.0), mindestens);
    }

    #[test]
    fn jede_spalte_hat_eine_eigene_kennung_und_ueberschrift() {
        for (stelle, spalte) in Spalte::ALLE.into_iter().enumerate() {
            for andere in Spalte::ALLE.into_iter().skip(stelle + 1) {
                assert_ne!(kennung(spalte), kennung(andere));
                assert_ne!(titel(spalte), titel(andere));
            }
        }
    }

    /// Genau eine Ansicht im Baum meldet sich fuer einen Abwurf an, und es ist
    /// die Dateiliste (C4, letztes Kriterium der Runde 13).
    ///
    /// **Die Zusage ist eine Aussage ueber den Baum und an keinem
    /// Rueckgabewert abzulesen.** Der Spec sagt unter C4: „Die Lesezeichen- und
    /// Geraeteleiste nimmt keinen Abwurf an: der Zeiger weist dort ab, und
    /// nichts wird kopiert oder verschoben." Getragen wird das nicht von einer
    /// Abwehr in [`super::leiste`], sondern davon, dass jene Datei sich nicht
    /// anmeldet — eine Ansicht ohne angemeldete Sorte bekommt vom System gar
    /// keinen Ziehvorgang zu sehen. Bliebe das ungezaehlt, machte eine spaetere
    /// Anmeldung in der Leiste sie unbemerkt zum Abwurfziel, ohne dass eine
    /// Probe rot wuerde: die beiden Tabellen teilen keine Zeile Code, und kein
    /// Rueckgabewert dieser Datei aendert sich dabei.
    ///
    /// **Eine Aufruferzaehlung und ausdruecklich die richtige Form.** Der Kopf
    /// von [`crate::quellbaum`] laesst sie dort zu, wo ein Abnahmekriterium die
    /// Zahl selbst zusagt, und hier tut es das. Faengt sie spaeter einen
    /// zweiten Rufer, ist die Frage, welche Ansicht sich da anmeldet, und nicht
    /// die Zahl hier.
    ///
    /// **Ihre Blindheit**, in der Form, die [`crate::quellbaum`] verlangt: eine
    /// Anmeldung ueber einen anderen Weg — `NSView`s Eigenschaft in einer
    /// Unterklasse ueberschrieben, oder eine Ansicht, die
    /// `NSDraggingDestination` von Hand umsetzt — sieht diese Nadel nicht. Sie
    /// zaehlt die eine Schreibweise, die der Baum kennt.
    ///
    /// Die Nadel steht zusammengesetzt da, weil die Probe in dem Baum liegt,
    /// den sie liest.
    #[test]
    fn genau_eine_ansicht_meldet_sich_fuer_einen_abwurf_an() {
        let anmelden = concat!("registerFor", "DraggedTypes");
        let anmeldestellen: Vec<String> = quelldateien()
            .into_iter()
            .filter(|(_, inhalt)| aufrufstellen(inhalt, anmelden) > 0)
            .map(|(name, _)| name)
            .collect();

        assert_eq!(
            anmeldestellen,
            vec![DIESE_DATEI.to_owned()],
            "die Anmeldung fuer einen Abwurf steht nicht allein in {DIESE_DATEI}; \
             die Lesezeichen- und Geraeteleiste nimmt keinen Abwurf an"
        );

        let inhalt = diese_datei();
        assert_eq!(
            aufrufstellen(&inhalt, anmelden),
            1,
            "die Dateiliste meldet sich nicht an genau einer Stelle an; \
             die eine ist Dateifenster::bauen"
        );
    }

    /// Die Tafel der Abwurfmeldung, vollstaendig: sechs gemerkte Gruende mal
    /// sechs eben gefaellte (C7).
    ///
    /// Sie schreibt aus, was die erste Zeile der Tafel an [`abwurfmeldung`] mit
    /// „gleich `jetzt`" zusammenfasst, und zeigt, dass keine Kombination fehlt.
    /// Die Erwartungen stehen als Werte da und werden nicht gerechnet: eine
    /// gerechnete Erwartung waere die Umsetzung ein zweites Mal. Dieselbe
    /// Bauform tragen die Tafeln in [`abwurfregel`] und
    /// [`crate::kommandos::rueckschritt`].
    ///
    /// **Was sie misst, ist die Zusage des Spec unter C7:** die Meldung darf
    /// nicht bei jeder Zeigerbewegung neu geschrieben werden. In der Tafel ist
    /// das die Hauptdiagonale — gleicher Grund, keine Meldung —, und sie ist
    /// der einzige Teil, den ein Ziehvorgang oft durchlaeuft: `validateDrop:`
    /// laeuft bei jeder Bewegung, und der Grund aendert sich dabei selten.
    ///
    /// **Was sie nicht misst**, und der Satz gehoert dazu: ob die Meldung
    /// wirklich in der Statuszeile ankommt und ob sie im richtigen
    /// Dateifenster steht. Beides verlangt ein stehendes Fenster und einen
    /// Ziehvorgang aus einer zweiten Anwendung; es steht als Nutzerarbeit im
    /// Plan der Runde 13.
    /// Die ganze Tafel des Abwurfziels: zwei Zeilenlagen mal vier
    /// Zeilenbefunde, also acht Faelle (C4).
    ///
    /// Sie schreibt aus, was die erste Zeile der Tafel an [`abwurfziel`] mit
    /// „gleichgueltig" zusammenfasst. Die Erwartungen stehen als Werte da und
    /// nicht als Rechnung: eine gerechnete Erwartung waere die Umsetzung ein
    /// zweites Mal. Dieselbe Bauform tragen die Tafeln in [`abwurfregel`] und
    /// [`crate::kommandos::rueckschritt`].
    ///
    /// **Was sie misst**, ist der Ausgang, den es bis zum 260819 nicht gab:
    /// eine Zeilennummer, an der jetzt kein Ordner mehr steht, faellt nicht
    /// mehr auf den angezeigten Ordner zurueck. Die drei unteren Zeilen sind
    /// dieser Ausgang; wer den Rueckfall wieder einbaut, macht sie rot.
    ///
    /// **Was sie nicht misst**, und der Satz gehoert dazu: dass AppKit
    /// wirklich `-1` fuer die ganze Liste liefert und die gesetzte Zeilennummer
    /// zurueckmeldet. Das verlangt eine Ziehsitzung aus einer zweiten
    /// Anwendung und steht als Nutzerarbeit im Plan der Runde 13.
    #[test]
    fn die_tafel_des_abwurfziels_geht_auf() {
        // benennt_eine_zeile, typ_der_zeile, Ziel.
        const TAFEL: [(bool, Option<Typ>, Abwurfziel); 8] = [
            (false, None, Abwurfziel::AngezeigterOrdner),
            (false, Some(Typ::Ordner), Abwurfziel::AngezeigterOrdner),
            (false, Some(Typ::Datei), Abwurfziel::AngezeigterOrdner),
            (
                false,
                Some(Typ::Verknuepfung),
                Abwurfziel::AngezeigterOrdner,
            ),
            (true, Some(Typ::Ordner), Abwurfziel::Zeile),
            (true, Some(Typ::Datei), Abwurfziel::Keines),
            (true, Some(Typ::Verknuepfung), Abwurfziel::Keines),
            (true, None, Abwurfziel::Keines),
        ];

        for (benennt_eine_zeile, typ, erwartet) in TAFEL {
            assert_eq!(
                abwurfziel(benennt_eine_zeile, typ),
                erwartet,
                "benennt_eine_zeile={benennt_eine_zeile}, typ={typ:?}"
            );
        }
    }

    /// Der gemeinsame Quellordner entsteht nur, wenn wirklich alle gezogenen
    /// Eintraege in einem Ordner liegen (C6, Lage 3).
    ///
    /// Die drei `None`-Faelle stehen einzeln da, weil sie drei verschiedene
    /// Lagen sind und derselbe Ausgang: eine leere Ablage, Eintraege aus zwei
    /// Ordnern, und ein Eintrag ohne uebergeordneten Ordner. Der letzte ist
    /// der, den ein `unwrap` verschluckt haette.
    #[test]
    fn der_gemeinsame_quellordner_entsteht_nur_aus_einem_ordner() {
        assert_eq!(gemeinsamer_quellordner(&[]), None, "eine leere Ablage");
        assert_eq!(
            gemeinsamer_quellordner(&[PathBuf::from("/haus/eins.txt")]),
            Some(PathBuf::from("/haus")),
            "ein einzelner Eintrag"
        );
        assert_eq!(
            gemeinsamer_quellordner(&[
                PathBuf::from("/haus/eins.txt"),
                PathBuf::from("/haus/zwei.txt"),
            ]),
            Some(PathBuf::from("/haus")),
            "zwei Eintraege aus demselben Ordner"
        );
        assert_eq!(
            gemeinsamer_quellordner(&[
                PathBuf::from("/haus/eins.txt"),
                PathBuf::from("/hof/zwei.txt"),
            ]),
            None,
            "zwei Eintraege aus zwei Ordnern"
        );
        assert_eq!(
            gemeinsamer_quellordner(&[PathBuf::from("/")]),
            None,
            "die Wurzel hat keinen uebergeordneten Ordner"
        );
    }

    #[test]
    fn die_tafel_der_abwurfmeldung_geht_auf() {
        const GRUENDE: [Option<Abwurfgrund>; 6] = [
            None,
            Some(Abwurfgrund::KeineDatei),
            Some(Abwurfgrund::VorgangLaeuft),
            Some(Abwurfgrund::NichtBeschreibbar),
            Some(Abwurfgrund::SelberOrdner),
            Some(Abwurfgrund::KeinAngebot),
        ];
        // Zeile: der gemerkte Grund. Spalte: der eben gefaellte, in derselben
        // Reihenfolge. `true` heisst: es wird geschrieben.
        //
        //                        —      KeineDatei  Vorgang  Recht  Ordner  Angebot
        const TAFEL: [[bool; 6]; 6] = [
            /* —          */ [false, true, false, false, false, false],
            /* KeineDatei */ [false, false, false, false, false, false],
            /* Vorgang    */ [false, true, false, false, false, false],
            /* Recht      */ [false, true, false, false, false, false],
            /* Ordner     */ [false, true, false, false, false, false],
            /* Angebot    */ [false, true, false, false, false, false],
        ];

        for (gemerkt, zeile) in GRUENDE.into_iter().zip(TAFEL) {
            for (jetzt, erwartet) in GRUENDE.into_iter().zip(zeile) {
                let meldung = abwurfmeldung(gemerkt, jetzt);
                assert_eq!(
                    meldung.is_some(),
                    erwartet,
                    "gemerkt={gemerkt:?}, jetzt={jetzt:?}"
                );
                if erwartet {
                    assert_eq!(meldung, Some(KEINE_DATEI));
                }
            }
        }
    }
    /// Der Selektor des Kontextmenues wird genau einmal erklaert und genau
    /// einmal gesetzt (Runde 17).
    ///
    /// **Die eine Falle dieser Runde, und der Uebersetzer haelt sie nicht.**
    /// `sel!` baut einen Selektornamen und fragt nicht, ob eine Methode ihn
    /// beantwortet; ein Vertipper zwischen der Erklaerung im
    /// `define_class!`-Block und dem Aufruf in
    /// [`DateifensterQuelle::eigene_kontexteintraege_anfuegen`] uebersetzt,
    /// besteht jede Probe, steht im Menue und tut nichts. Genau die Gestalt,
    /// die `CLAUDE.md` fuer Tastenbefehle beschreibt.
    ///
    /// Gezaehlt werden Fundstellen und nicht Dateien: eine zweite Erklaerung
    /// in dieser Datei waere genauso ein zweiter Empfaenger wie eine in einer
    /// anderen. Kommentarzeilen zaehlen nicht mit — die Koepfe darueber nennen
    /// den Namen mehrfach in Prosa, und das ist keine Beruehrung.
    ///
    /// Die Nadeln stehen zusammengesetzt da, wie bei
    /// `es_gibt_genau_einen_menuebauer` in [`super::super::teilen`]: als ein
    /// Stueck geschrieben faenden sie sich selbst.
    #[test]
    fn der_kontextmenue_selektor_hat_einen_empfaenger_und_einen_setzer() {
        let dateien = quelldateien();
        let zaehlen = |nadel: &str| -> usize {
            dateien
                .iter()
                .map(|(_, inhalt)| {
                    code_zeilen(inhalt)
                        .filter(|zeile| zeile.contains(nadel))
                        .count()
                })
                .sum()
        };
        assert_eq!(
            zaehlen(concat!("method(", "kontextbefehl:)")),
            1,
            "der Selektor `kontextbefehl:` wird nicht genau einmal erklaert"
        );
        assert_eq!(
            zaehlen(concat!("sel!(", "kontextbefehl:)")),
            1,
            "der Selektor `kontextbefehl:` wird nicht genau einmal gesetzt"
        );
    }
}
