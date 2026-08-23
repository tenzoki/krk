//! Der Start: `NSApplication`, der Anwendungsdelegierte, das eine Fenster.
//!
//! KRK laeuft als gewoehnliche Anwendung im Vordergrund
//! (`NSApplicationActivationPolicy::Regular`), auch wenn `cargo run` sie ohne
//! Buendel startet. Fuer die Abnahme zaehlt trotzdem allein der Start ueber
//! `target/KRK.app`: nur ein signiertes Buendel loest die Rueckfragen von TCC
//! aus, und ein nacktes Binaerprogramm erbt stattdessen die Freigaben des
//! Terminals.
//!
//! # Was der Delegierte haelt
//!
//! ```text
//! Anwendungsdelegierter
//!   ├─ Fenstermodell        aktives Dateifenster, Sichtbarkeit, Breiten
//!   ├─ Aufteilung           die NSSplitView mit ihren vier Bereichen
//!   ├─ Dateifenster × 2     Tableiste, Dateiliste, Statuszeile, Tabs
//!   ├─ Leiste               Lesezeichen und Geraete, der zweite Bereich (C5)
//!   ├─ NSWindow             genau eines, siehe unten
//!   ├─ Tastenabgriff        der eine Eintrittspunkt fuer Tastendruecke
//!   ├─ Dateisystemwache     FSEvents auf den sichtbaren Ordnern (C9)
//!   ├─ Datentraegerwache    NSWorkspace auf Einhaengen und Auswerfen (C9)
//!   ├─ Ablage               der Zugang zu bookmarks.toml (C5)
//!   └─ Sitzungsschreiber    gebuendelt, hoechstens alle zwei Sekunden
//! ```
//!
//! Die beiden Wachen stehen hier aus demselben Grund wie der Tastenabgriff:
//! ohne Halter meldet sich ein Beobachter beim Fallenlassen sofort wieder ab.
//!
//! # Der Weg einer fremden Aenderung
//!
//! ```text
//!  Dateisystemwache ──> auffrischung::ordner_neu_lesen ──> Dateifenster::neu_lesen
//!  Datentraegerwache ─> auffrischung::datentraeger_verloren ─> je getroffenem Tab
//!                                                              tab_wechseln, dann
//!                                                              einmal melden
//!
//!  jede Navigation ───> Dateisystemwache neu aufsetzen
//! ```
//!
//! Der Anwendungsdelegierte setzt beides zusammen: er ist die einzige Stelle,
//! die beide Dateifenster **und** das Fenstermodell haelt, und damit die
//! einzige, die die Frage "welche Ordner stehen gerade auf dem Schirm"
//! beantworten kann. Die Antwort selbst rechnet [`crate::auffrischung`]; hier
//! steht nur die Zuleitung.
//!
//! **KRK haelt in dieser Runde genau ein Anwendungsfenster.** Die beiden
//! Dateifenster aus C1 sind Bereiche darin und keine zwei Fenster des Systems.
//! Der Nutzer hat das am 260804-0830 mit Moeglichkeit 2 aus
//! `decisions/260803-2007_*_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`
//! festgelegt: das Fenster ueberlebt sein Schliessen, und zwei Wege holen es
//! zurueck, der Menueeintrag "Fenster einblenden" auf Cmd+N und der Klick auf
//! das Dock-Symbol ueber `applicationShouldHandleReopen:`. Ein laufendes KRK
//! ohne Fenster und ohne Rueckweg gibt es damit nicht mehr.
//!
//! # Der Weg eines Tastendrucks
//!
//! Der Ereignisabgriff kennt kein Dateifenster; er liefert eine [`Eingabe`] an
//! [`Anwendungsdelegierter::eingabe_ausfuehren`]. Der teilt auf: was das
//! Fenster als ganzes betrifft, bleibt hier, alles uebrige geht an die
//! Datenquelle des **aktiven** Dateifensters. Eine zweite Stelle, die
//! entscheidet, wohin ein Tastendruck geht, entsteht nicht.
//!
//! Zwei Sorten von Eingabe kommen an. Ein [`Kommando`] ist eine nachgeschlagene
//! Funktion; ein Zeichen gehoert dem Filtertext des sichtbaren Tabs und damit
//! dem aktiven Dateifenster, weil er die Liste verkuerzt, die vor dem Nutzer
//! steht. **Beide gehen durch denselben Fokusvorbehalt**: das Zeichen erreicht
//! den Filtertext nur mit dem Fokus im Dateifenster, sonst laeuft der
//! Tastendruck unveraendert an AppKit weiter. Ohne diese Zeile tippte ein
//! Buchstabe mit der Schreibmarke im Editor in die Dateiliste.
//!
//! **Laesst sich der Abgriff nicht einrichten, laeuft KRK nicht weiter.** Beide
//! Stellen, die ihn aufsetzen — der Aufbau der Oberflaeche und das Nachziehen
//! nach einer Umbelegung —, gehen dann durch
//! [`Anwendungsdelegierter::ohne_tastenabgriff_beenden`]: ein modales
//! Hinweisfenster aus [`super::hinweis`], danach `terminate:`. Der Grund und
//! der Entscheid des Nutzers stehen an jener Funktion.
//!
//! # Der eine Fokusvorbehalt (C5)
//!
//! Seit S17 der Editor-Runde gibt es vier fokussierbare Bereiche — die beiden
//! Dateilisten, die Leiste, die Vorschau und die Textflaeche des Editors —, und
//! [`Anwendungsdelegierter::kommando_ausfuehren`] fragt **einmal**, wo der
//! Fokus steht:
//!
//! ```text
//!  Kommando ──> gehoert das Schluesselfenster KRK?
//!                    │  nein: nichts, ausser der Ausnahmeliste
//!                    ▼
//!               steht ein Blatt? ──> fokus() ──> fokus::wirkt(Wirkungsbereich)
//!                                       │                    │  nein: nichts
//!                                       │                    ▼
//!                                       │            fensterweiter Befehl
//!                                       └───Adresse──> Dateifenster / Leiste
//!                                                       / Vorschau / Editor
//! ```
//!
//! **Die erste Frage ist seit der Runde 8 dabei**, und sie ist der vierte
//! Bestandteil der Zulaessigkeitsregel: gehoert das Schluesselfenster nicht
//! KRK, wirkt kein Befehl auf das Hauptfenster. Erhoben wird sie in
//! [`Anwendungsdelegierter::schluesselfenster`], einmal je Eingabe, und
//! [`Anwendungsdelegierter::fokus_bei`] bekommt denselben Wert, statt
//! `NSApplication::keyWindow` ein zweites Mal zu fragen. Ohne sie wirkte hinter
//! einem freistehenden Panel — dem Ueber-Dialog aus C5 der Runde 8 — jeder
//! Befehl weiter, der `Wirkungsbereich::Ueberall` traegt.
//!
//! Der Wert wird zweimal gebraucht und einmal erhoben. Zuerst als
//! **Vorbehalt**: [`crate::kommandos::fokus::wirkt`] sagt, ob der Befehl hier
//! ueberhaupt wirkt, und ein abgewiesener tut nichts und meldet nichts. Danach
//! als **Adresse**: was weder dem Fenster als ganzem gehoert noch schon
//! abgewiesen ist, geht an den Bereich, der den Fokus hat, denn beide Bereiche
//! sind Listen mit einer Auswahl und der Auf- und der Ab-Pfeil bewegen die des
//! Bereichs vor dem Nutzer. Die einzelne Abfrage der Loeschtasten aus Schritt
//! 16 ist in dem Vorbehalt aufgegangen und steht nicht daneben.
//!
//! **Gefragt wird AppKit und nicht ein eigenes Feld.** Welcher Bereich den
//! Fokus hat, sagt der Ersthelfer des Fensters; ein Kennzeichen daneben waere
//! eine zweite Wahrheit, die jeder Mausklick in eine der drei Listen
//! nachzuziehen haette. Die beiden Fokusbefehle aus C5 setzen deshalb den
//! Ersthelfer und nichts sonst.
//!
//! **Woran ein Bereich zu erkennen ist, steht an einer Stelle**, in
//! [`Anwendungsdelegierter::fokusansicht`]: eine erschoepfende Zuordnung von
//! einem Fokuswert auf die Ansicht, die seinen Ersthelferrang traegt. Lesen und
//! Setzen gehen beide darueber, seit C1 der Runde 6 auch das Teilen, das
//! dieselbe Ansicht als **Anker** seines Freigabedialogs nimmt; ein sechster
//! Fokuswert haelt dort den Bau an.
//!
//! **Wo er beim Start steht, sagt trotzdem KRK und nicht AppKit.** Ueberliesse
//! [`Anwendungsdelegierter::oberflaeche_aufbauen`] die erste Vergabe der
//! Schluesselansichtskette, bekaeme sie deren erste Ansicht — seit S18 die
//! Leiste, und damit wirkte kein Befehl des Dateifensters, bis der Nutzer den
//! Fokus einmal von Hand setzt. Die letzte Zeile des Aufbaus setzt ihn deshalb
//! auf [`crate::kommandos::fokus::BEIM_START`], ueber dieselbe eine Stelle, die
//! auch die beiden Fokusbefehle gehen.
//!
//! # Der Messmodus haengt an derselben Stelle wie der Tastenabgriff
//!
//! Ist `--messmodus` gesetzt, richtet [`Anwendungsdelegierter::oberflaeche_aufbauen`]
//! nach dem Tastenabgriff zwei weitere Dinge ein: den Bildtakt aus
//! [`super::bildtakt`], der jede Bildgrenze meldet, und einen Ausloesetakt, der
//! den naechsten Messschritt anstoesst. Beide reichen ausschliesslich
//! gewoehnliche Rust-Werte an [`crate::messmodus`] weiter — die Zeitpunkte der
//! Bildgrenzen und drei Zahlen ueber den Zustand der Liste.
//!
//! ```text
//!  Ausloesetakt (97 ms) ──> messmodus::naechster_schritt ──> Anweisung
//!                                                             │
//!            ordner_lesen / pfeil_ab_senden  <────────────────┘
//!
//!  Bildtakt (CADisplayLink) ──> messmodus::bildgrenze(Zeitpunkt, Zustand)
//! ```
//!
//! **Die Strecken aus S8 ruehren die Sitzung des Nutzers nicht an.** Sie
//! laden `session.toml` nicht und schreiben sie nicht, und allein das linke
//! Dateifenster liest den Pruefordner. Beides haelt gemessen, was Schritt 8
//! gemessen hat: eine wiederhergestellte Sitzung braechte fremde Ordner in die
//! Messung, und ein zweiter Lesevorgang auf denselben Pruefordner machte den
//! Kaltstart zur Haelfte warm.
//!
//! **Die Sitzungsstrecke aus S21 stellt dagegen die Pruefsitzung aus C8 her
//! und schreibt sie als `session.toml`**, denn C8 misst L4 und L5 auf genau
//! dieser Lage, und die folgenden `sitzungsstart`-Laeufe muessen sie beim
//! gewoehnlichen Wiederherstellen vorfinden. Geschrieben wird ueber den
//! [`Sitzungsschreiber`], also denselben Weg wie beim Beenden; einen
//! laufenden Sitzungsschreiber gibt es im Messmodus weiterhin nicht, und die
//! beiden Wachen aus C9 bleiben aus.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSApplication`, `NSWindow`, `NSResponder`, `NSView`, `NSNotification`,
//! `NSObject`, `NSRunLoop`, `NSString` und `NSTimer` stehen seit macOS 10.0 zur
//! Verfuegung, ebenso die beiden angenommenen Protokolle `NSObjectProtocol`
//! und `NSApplicationDelegate` samt den vier Rueckrufen, die diese Datei aus
//! dem zweiten bedient. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine von ihnen ist nach macOS 15 hinzugekommen.
//! `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer
//! haelt die Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.
//!
//! **Drei Beruehrungen sind juenger als ihre Klasse, und alle drei liegen unter
//! dem Zielsystem**; keine von ihnen braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `NSApplication::activate` steht seit
//! macOS 14 (`NSApplication.h:231`) — die aeltere `activateIgnoringOtherApps:`
//! traegt `API_DEPRECATED(..., macos(10.0, API_TO_BE_DEPRECATED))`, ist also
//! angekuendigt und nicht abgekuendigt, und wird hier nicht angesprochen;
//! `setActivationPolicy:` steht seit 10.6 (`NSApplication.h:301`) und
//! `NSRunLoopCommonModes` seit 10.5 (`NSRunLoop.h:14`). Die Aufzaehlung
//! `NSApplicationActivationPolicy` traegt selbst keine Angabe; sie steht in
//! `NSRunningApplication.h` und gehoert zu `setActivationPolicy:`.
//!
//! **Nichts in dieser Datei liegt ueber 15.0.** Alles Uebrige — darunter
//! `attachedSheet`, `makeFirstResponder:`, `firstResponder`, `performClose:`,
//! `replyToApplicationShouldTerminate:` und
//! `timerWithTimeInterval:target:selector:userInfo:repeats:` — traegt im
//! SDK-Kopf gar keine Verfuegbarkeitsangabe und steht damit seit 10.0.

use std::cell::{Cell, OnceCell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use dispatch2::DispatchQueue;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate,
    NSApplicationTerminateReply, NSMenuItem, NSMenuItemValidation, NSResponder, NSTextView, NSView,
    NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSRunLoop, NSRunLoopCommonModes,
    NSString, NSTimer,
};

use krk_core::ablage::sitzung::Sitzungsschreiber;
use krk_core::ablage::{
    Ablage, Aenderung, Ausgang, Datei, Einstellungen, Fensterseite, Lesezeichen, Lesezeichenliste,
    Sitzung, Sitzungsrecht, Verschiebung, Ziel, Zugang, einstellungen, lesezeichen, pfade,
};
use krk_core::operation::{
    self, Abschluss, Art, Auftrag, Bericht, Konfliktantwort, Konfliktentscheid, Lauf, Meldung,
    Namensfehler, freier_name,
};
use krk_core::stapelumbenennen::Vorschau;
use krk_core::tasten::belegung;
use krk_core::tasten::normalisierung::ModMaske;
use krk_core::tasten::{Belegung, Kommando, Tastendruck, code_von_pflicht};
use krk_core::verzeichnis::{Loeschzielbefund, arbeitsbaum, umfang};

use crate::angezeigtedatei;
use crate::auffrischung::{self, Dateifenstersicht};
use crate::belegungsausgabe;
use crate::belegungsmodell::Belegungsmodell;
use crate::editormodell::{Ladeausgang, Sicherungsausgang};
use crate::fenstermodell::{
    BREITENSCHRITT, Bereich, Fenstermodell, Zeilenmass, sichtbar_in, spalte_sichtbar_in,
};
use crate::fenstertitel;
use crate::kommandos::abwurfregel::Abwurfvorgang;
use crate::kommandos::fokus::{self, Fokus};
use crate::kommandos::loeschwarnung::{self, Loeschziel, Nachstufe, Vorstufe};
use crate::kommandos::operationen::{self, Anlegeart, Auswahl, Konfliktfrage, Vorgangszustand};
use crate::kommandos::rueckschritt::{Rueckschritt, rueckschritt};
use crate::kommandos::rundweg::{Rundweg, rundweg};
use crate::kommandos::zulaessigkeit::{self, Lage};
use crate::leistenmodell::Ort;
use crate::messmodus::{Anweisung, Aufgabe, Handlung, Messlauf, Sitzungslage, Zustand};
use crate::spalten::Spalte;
use crate::tabs::{Auswahlversuch, Tabliste};
// `Wechsel` heisst hier `zettelmodell::Wechsel` und nicht kurz: `super::volumes`
// fuehrt einen gleichnamigen Typ, und das ist ein anderer Gegenstand.
use crate::zettelmodell::{self, Zettelmodell};

use super::aufteilung::Aufteilung;
use super::belegungsansicht::{self, Belegungsquelle};
use super::bereichsleiste::Bereichsleiste;
use super::bildtakt::{self, Zeichenende};
use super::blaetter::ungesichert::{self, Antwort};
use super::blaetter::{
    self, Blattgriff, konflikt, loeschbestaetigung, namenseingabe, stapelumbenennen, uebersprungen,
    zettel,
};
use super::editor::{Editorbereich, Editormeldung, Oeffnungsherkunft};
use super::ereignisse::{self, Anschlag, Eingabe, Tastenabgriff};
use super::fenster::{self, FensterDelegierter};
use super::fsevents::Dateisystemwache;
use super::hinweis;
use super::leiste::Leiste;
use super::menue;
use super::papierkorb::{self, Systempapierkorb};
use super::statuszeile::{self, Statuszeile};
use super::tabelle::Dateifenster;
use super::teilen;
use super::terminal;
use super::volumes::{self, Datentraeger, Datentraegerwache, Wechsel};
use super::vorschau::Vorschaufenster;
use super::weitereinstanz;

/// Der Rueckgabewert, mit dem ein Messlauf ohne Bildschirm endet.
const OHNE_BILDSCHIRM: i32 = 3;

/// Der Satz, den eine Instanz ohne Sitzungsrecht beim Start zeigt (C3.10).
///
/// **Er nennt die Folge und nicht den Mechanismus.** „Sitzungsrecht" ist ein
/// Wort dieses Bauplans; was der Nutzer merkt, ist, dass dieses Fenster seine
/// Tabs und seine Aufteilung nicht wiederfindet. Er steht als Konstante da,
/// damit die Probe ihn nennen kann, ohne ihn abzuschreiben: `sitzung_laden`
/// braucht einen Ablageordner und ein Fenster und ist ohne beides nicht zu
/// pruefen. **Dass der Satz in der Statuszeile ankommt, sieht der Nutzer am
/// laufenden Buendel**; er geht denselben Weg wie jede andere Startmeldung.
const OHNE_SITZUNGSRECHT: &str = "eine weitere Instanz von KRK laeuft schon; Tabs und Aufteilung \
                                  dieses Fensters werden nicht gesichert";

/// Welche Station des Faengers einen Tastendruck der Belegungsansicht bekommt.
///
/// **Eine vollstaendige Fallunterscheidung ohne Auffangzweig, und ohne eine
/// Zeile AppKit.** Die Reihenfolge, in der [`faengerstation`] die Faelle
/// abfragt, **ist** der Vorrang aus C1.15; sie steht als Wert da, damit sie
/// ohne Fenster zu pruefen ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Faengerstation {
    /// Erste Station: die Aufnahme bekommt den rohen Tastendruck (C3).
    ///
    /// Sie bekommt **jeden**, auch `esc` und jedes Zeichen. Genau das ist der
    /// Vorrang aus C1.15.
    Aufnahme,
    /// Zweite Station: die Eingabetaste geht zum naechsten Treffer (C1.7).
    NaechsterTreffer,
    /// Zweite Station: die Ruecktaste kuerzt den Suchtext (C1.8).
    ZeichenWeg,
    /// Zweite Station: das Zeichen wird der Suche **angeboten** (C1.1).
    ///
    /// Ob sie es nimmt, entscheidet allein
    /// [`Suchlage::zeichen_anhaengen`](crate::belegungsmodell::Suchlage::zeichen_anhaengen);
    /// eine zweite Zeichenregel entsteht hier nicht (C1.2). Daran haengt, dass
    /// `esc` weiterlaeuft und die Ansicht verlaesst: sein Zeichen ist ein
    /// Steuerzeichen, die Suche weist es ab, und der Faenger verbraucht das
    /// Ereignis deshalb nicht (C1.13).
    Suchzeichen(char),
    /// Keine: der Tastendruck laeuft unveraendert in den Nachschlag.
    Keine,
}

/// Die Station, die diesen Tastendruck bekommt, wenn die Belegungsansicht
/// steht.
///
/// `nimmt_auf` ist [`Belegungsquelle::nimmt_auf`], `zeichen` das **getippte**
/// Zeichen aus dem Ereignis und nicht [`Tastendruck::zeichen`].
///
/// Rein und deshalb hier neben dem Delegierten statt in ihm: die Zuordnung ist
/// ohne Fenster pruefbar, und C1.13 und C1.15 sind damit gewoehnliche
/// Pruefungen.
fn faengerstation(nimmt_auf: bool, druck: Tastendruck, zeichen: Option<char>) -> Faengerstation {
    if nimmt_auf {
        return Faengerstation::Aufnahme;
    }
    // Eine Kombination mit Befehls-, Steuerungs- oder Wahltaste gehoert nicht
    // der Suche: sie traegt die Kuerzel der drei Schaltflaechen (Cmd+T, Cmd+R,
    // Cmd+Eingabe) und jedes Kuerzel des Hauptmenues. Die Umschalttaste bleibt
    // zugelassen, denn sie ist die Grossschreibung eines Zeichens.
    if druck.maske.enthaelt(ModMaske::BEFEHL)
        || druck.maske.enthaelt(ModMaske::STEUERUNG)
        || druck.maske.enthaelt(ModMaske::WAHL)
    {
        return Faengerstation::Keine;
    }
    if druck.code == CODE_EINGABE {
        return Faengerstation::NaechsterTreffer;
    }
    if druck.code == CODE_RUECKTASTE {
        return Faengerstation::ZeichenWeg;
    }
    match zeichen {
        Some(zeichen) => Faengerstation::Suchzeichen(zeichen),
        None => Faengerstation::Keine,
    }
}

/// Der Tastencode der Eingabetaste, aus der einen Tastentabelle des Kerns.
///
/// Die zweite Station des Faengers erkennt sie daran und nicht am getippten
/// Zeichen: `\r` ist ein Steuerzeichen und faellt durch die Aufnahmeregel der
/// Suche, und eine zweite Zeichenregel daneben entstuende sonst.
const CODE_EINGABE: u16 = code_von_pflicht("return");

/// Der Tastencode der Ruecktaste, aus derselben Tabelle.
///
/// Sie heisst dort `delete`, wie auf der Mac-Tastatur; der Code 51 ist
/// `kVK_Delete`, die Taste ueber dem Backslash, und nicht `kVK_ForwardDelete`.
const CODE_RUECKTASTE: u16 = code_von_pflicht("delete");

/// Ein Vorgang, der den ungesicherten Stand des Editors verlieren wuerde (C4).
///
/// **Die drei Anlaesse aus C4, als Wert.** Jeder von ihnen verliert den Stand
/// wirklich: das Schliessen gibt die Datei frei, der Wechsel ersetzt sie, das
/// Beenden nimmt den ganzen Prozess mit.
///
/// **Ein vierter Anlass stand hier bis zum 260810**, das Einblenden der
/// Vorschau, und er ist mit dem Nutzerentscheid vom 260810-0250 gefallen. Er
/// verliert nichts: ein Wechsel der Sichtbarkeit setzt `hidden` an den
/// Ansichten und fasst das [`crate::editormodell::Editormodell`] nicht an, und
/// so verwarf "Verwerfen" an dieser einen Stelle nichts. Der Datensatz ist
/// `decisions/260810-0021_*_was-verwirft-verwerfen-wenn-die-vorschau-den-editor-nur-verdraengt.md`.
///
/// **Er steht in keinem Feld.** Der Wert wird in die Schliessung des Blattes
/// hineinkopiert und faellt mit ihr; siehe
/// [`Anwendungsdelegierter::nachfrage_zeigen`]. Was er dem Programm bringt, ist
/// die Erzwingung: [`Anwendungsdelegierter::anlass_ausfuehren`] und
/// [`Anwendungsdelegierter::anlass_unterbleibt`] sind zwei vollstaendige
/// Fallunterscheidungen ohne Auffangzweig, und ein vierter Anlass haelt an
/// beiden den Bau an, statt still den Zweig des Nachbarn zu bekommen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Anlass {
    /// `opt+cmd+e` und der Rueckweg von `cmd+e`: der Editor wird ausgeblendet
    /// und gibt seine Datei frei.
    EditorSchliessen {
        /// Ob danach die Vorschau wieder eingeblendet wird.
        ///
        /// **Der eine Unterschied zwischen den beiden Ruefern.** `opt+cmd+e`
        /// schliesst und laesst die Flaeche leer, wie seit der Editor-Runde;
        /// der Rueckweg von `cmd+e` holt die Vorschau zurueck (Nutzerentscheid
        /// vom 260823-0942, dort ohne Vorbehalt: „die Vorschau zeigt die Datei
        /// wieder").
        ///
        /// **Die Zeile traegt eine Regel und keine Umkehrung.** Der Rueckweg
        /// endet immer in derselben Lage, gleich wo er begonnen hat — auch
        /// dann, wenn der Nutzer die Vorschau vorher mit `f3` ausgeschaltet
        /// hatte und der Hinweg also nichts verdraengt hat. Aus der Lage beim
        /// Druecken ist der Unterschied nicht abzulesen: der gegenseitige
        /// Ausschluss aus C1 haelt die Vorschau ausgeblendet, solange der Editor
        /// die Flaeche hat, gleich aus welchem Grund. Wer die Zeile bedingt
        /// machen will, braucht einen gemerkten Zustand und einen Setzer an
        /// jedem Weg in den Editor; die Frage steht dem Nutzer vor
        /// (`shared/decisions/260823-1137_*_holt-der-rueckweg-von-cmd-e-die-vorschau-*`).
        ///
        /// **Ein Feld und kein vierter Anlass.** Der Anlass ist derselbe — der
        /// Editor gibt seine Datei auf, und dieselbe Nachfrage aus C4 geht
        /// voraus —, und ein vierter Wert daneben hiesse, dass `anlass_ausfuehren`
        /// und `anlass_unterbleibt` zwei Zweige fuer ein und dieselbe Sache
        /// fuehren.
        ///
        /// **Er reist bis hinter die Nachfrage mit**, und das ist der Grund, aus
        /// dem er hier steht und nicht beim Aufrufer: sagt der Nutzer
        /// "Abbrechen", bleibt der Editor stehen, und die Vorschau darf ihn dann
        /// gerade nicht verdraengen.
        vorschau_danach: bool,
    },
    /// Der Editor nimmt eine andere Datei auf, die schon gelesen und geprueft
    /// ist und auf die Antwort wartet (C2).
    AndereDatei,
    /// KRK wird beendet.
    Beenden,
}

/// Ein laufender Dateivorgang, aus der Sicht des Hauptfadens (C4).
///
/// **Es gibt hoechstens einen.** Bis S16 hielt die Tastensperre einen zweiten
/// fern, weil ein Blatt stand und alles ausser dem Abbruch abfing. Seit der
/// Fortschritt in der Statuszeile steht, ist die Oberflaeche bedienbar und der
/// Nutzer kann F5 ein zweites Mal druecken; [`Anwendungsdelegierter::auftrag_stellen`]
/// prueft deshalb selbst und meldet den laufenden Vorgang, statt ihn
/// stillschweigend zu ueberschreiben. Eine Warteschlange waere die andere
/// Antwort; sie baut einen Zustand mehr, den keine Zusage verlangt.
struct Vorgang {
    /// Was geschieht. Traegt die Ueberschrift und die Abschlussmeldung.
    art: Art,
    /// Das Dateifenster, das den Vorgang begonnen hat.
    ///
    /// **Nicht das gerade aktive.** Seit dem 260804-1832 darf der Nutzer
    /// waehrend einer Operation das Fenster wechseln; danach sagt "das aktive
    /// Fenster" nichts mehr darueber aus, in welche Statuszeile der Fortschritt
    /// und der Abschlusstext gehoeren.
    seite: Fensterseite,
    /// Der Ordner, aus dem die Eintraege stammen.
    quellordner: PathBuf,
    /// Wie viele Positionen der Nutzer ausgewaehlt hatte.
    positionen: usize,
    /// Wann der Vorgang begonnen hat. Der Verzug misst ab hier.
    begonnen: Instant,
    /// Der Zustand, den der Vermittlerfaden fuellt.
    zustand: Arc<Vorgangszustand>,
}

impl Vorgang {
    /// Die Ordner, die dieser Vorgang umschreibt: erst die Quelle, dann das
    /// Ziel, falls es eines gibt.
    ///
    /// **Die eine Stelle, die diese Frage beantwortet.** Zwei Aufrufer stellen
    /// sie, und beide brauchen dieselbe Antwort: der Abschluss frischt genau
    /// diese Ordner auf, und die Dateisystemwache schiebt genau fuer sie die
    /// Auffrischung auf, solange ein aufschiebender Vorgang laeuft. Zwei
    /// Aufzaehlungen nebeneinander waeren zwei Wahrheiten darueber, was eine
    /// Operation anfasst.
    ///
    /// **Ob aufgeschoben wird, entscheidet diese Aufzaehlung nicht.** Das tut
    /// [`auffrischung::schiebt_auffrischung_auf`] anhand der Operationsart. Der
    /// Abschluss frischt danach unveraendert fuer jede Art auf; aufgeschoben
    /// wird allein beim Stapel-Umbenennen.
    fn ordner(&self) -> Vec<PathBuf> {
        let mut ordner = vec![self.quellordner.clone()];
        match &self.art {
            Art::Kopieren { ziel } | Art::Verschieben { ziel } => ordner.push(ziel.clone()),
            // Loeschen, Papierkorb und das Stapel-Umbenennen bleiben im
            // Quellordner.
            Art::InDenPapierkorb | Art::UmbenennenImStapel { .. } => {}
        }
        ordner
    }
}

/// Was der Anwendungsdelegierte haelt.
///
/// Die Zellen tragen Objekte, die AppKit nur schwach referenziert oder gar
/// nicht kennt. Faellt eines von ihnen, faellt das Fenster mit; faellt der
/// Tastenabgriff, meldet er sich bei AppKit ab, und faellt der Bildtakt, gibt
/// er den `CADisplayLink` frei.
pub struct AnwendungsIvars {
    /// Ob der Protokollmodus `--tasten-protokoll` laeuft.
    tasten_protokoll: bool,
    /// Die Aufgabe des Messmodus, falls einer laeuft.
    messaufgabe: Option<Aufgabe>,
    /// Die Belegung des Nutzers, einmal geladen.
    ///
    /// Sie kommt von [`starten`] herein und nicht aus einem zweiten Aufruf von
    /// [`belegung::fuer_den_betrieb`]: seit Schritt 13c nimmt das Hauptmenue
    /// seine Kuerzel aus derselben Belegung, und zweimal zu laden hiesse, zwei
    /// Staende derselben Datei nebeneinander zu halten und die Meldung ueber
    /// eine beschaedigte `keymap.toml` zweimal zu erzeugen.
    ///
    /// Veraenderlich seit Schritt 20: verlaesst der Nutzer die
    /// Belegungsansicht mit Aenderungen, tritt die gesicherte Belegung hier an
    /// die Stelle der geladenen, und Menue wie Ereignisabgriff werden auf sie
    /// nachgezogen. Eine Quelle, dieselben zwei Abnehmer.
    belegung: RefCell<Belegung>,
    /// Die von Hand gepflegten Einstellungen aus `settings.toml` (C11).
    ///
    /// Sie haengen hier, wo schon die Belegung und die Sitzung haengen: einmal
    /// beim Start geladen, danach unveraendert. Kein Weg in dieser Runde
    /// schreibt sie, und keiner liest die Datei ein zweites Mal. Bis
    /// [`Self::sitzung_laden`] gelaufen ist, steht hier die eingebettete
    /// Auslieferungsfassung; im Messmodus bleibt es dabei, weil dort nichts
    /// geladen wird.
    einstellungen: RefCell<Einstellungen>,
    /// Die Meldung, falls die Belegung ersetzt werden musste.
    ///
    /// Sie steht hier und nicht in der Statuszeile, weil es die Statuszeile
    /// beim Laden noch nicht gibt: [`starten`] laeuft vor
    /// `applicationDidFinishLaunching:`.
    belegungsmeldung: Option<String>,
    /// Das aktive Dateifenster, die Sichtbarkeit und die Breiten.
    modell: RefCell<Fenstermodell>,
    fenster: OnceCell<Retained<NSWindow>>,
    fenster_delegierter: OnceCell<Retained<FensterDelegierter>>,
    aufteilung: OnceCell<Aufteilung>,
    /// Die Bereichsleiste am Fensterfuss (C1 bis C3 der Bereichsleisten-Runde).
    ///
    /// Sie steht hier und nicht in der Aufteilung: sie ist kein Bereich der
    /// Fensterzeile, sondern deren Schwester unter der Inhaltsflaeche, und ihre
    /// acht Zustaende kommen aus dem Fenstermodell, das ebenfalls hier haengt.
    bereichsleiste: OnceCell<Bereichsleiste>,
    /// Die **eine** Statuszeile ueber die volle Fensterbreite (C5 der Runde 6).
    ///
    /// Sie steht hier aus demselben Grund wie die Bereichsleiste darueber, und
    /// aus einem zweiten: was in ihr steht, haengt an den Meldungsquellen
    /// **beider** Dateifenster und an der aktiven Seite, und der
    /// Anwendungsdelegierte ist der einzige, der alle drei sieht. Bis zur Runde
    /// 6 hielt jedes Dateifenster seine eigene Zeile und schrieb sie selbst.
    statuszeile: OnceCell<Statuszeile>,
    /// Die beiden Dateifenster, links zuerst.
    dateifenster: OnceCell<[Dateifenster; 2]>,
    /// Die Lesezeichen- und Geraeteleiste (C5), der zweite fokussierbare
    /// Bereich.
    leiste: OnceCell<Leiste>,
    /// Das Vorschaufenster (C6), der dritte fokussierbare Bereich.
    vorschau: OnceCell<Retained<Vorschaufenster>>,
    /// Der eingebaute Editor, der fuenfte Bereich der Fensterzeile.
    ///
    /// Er steht hier, weil der Anwendungsdelegierte die eine Stelle ist, die
    /// den Ersthelfer des Fensters gegen die Textflaeche haelt: der
    /// Ereignisabgriff fragt ihn nach der Naemlichkeit, und `appkit::ereignisse`
    /// kennt den Editor nicht.
    editor: OnceCell<Retained<Editorbereich>>,
    /// Der Pfad, den die ausgeblendete Vorschau beim Einblenden nachholt (C6,
    /// C7).
    ///
    /// Bei ausgeblendetem Vorschaufenster wird eine neue Auswahl nur hier
    /// vermerkt und nicht gelesen: bis zum 260806 stiess jeder Zeilenschritt
    /// auch dann `stat(2)`, einen Arbeitsfaden und ein Dateilesen an, wenn die
    /// Flaeche, fuer die gelesen wurde, auf keinem Schirm stand. Wer die
    /// Vorschau ausblendet, will gerade diese Kosten sparen.
    ///
    /// Leer heisst: es ist nichts nachzuholen. Der Tab behaelt dann seinen
    /// Inhalt, wie das Halteverhalten aus C6 es ohnehin sagt.
    vorschau_nachtrag: RefCell<Option<PathBuf>>,
    /// Der Zugang zu `bookmarks.toml` (C5).
    ///
    /// Er steht hier und nicht in der Leiste: der Kern legt ab, die Leiste
    /// zeigt an, und der Delegierte ist die Stelle, an der beide zusammenkommen
    /// — dieselbe Aufgabenteilung wie bei der Sitzung. Leer, wenn sich der
    /// Ablageordner nicht oeffnen liess; die Meldung dazu steht dann in der
    /// Statuszeile, und die Leiste arbeitet ohne zu sichern weiter.
    ablage: RefCell<Option<Ablage>>,
    /// Das Sitzungsrecht dieses Prozesses (C3 der Runde 7).
    ///
    /// **Es steht hier, weil es gehalten werden muss, und nicht, weil jemand es
    /// abfragt.** Mit dem Wert faellt der Deskriptor, und mit dem Deskriptor die
    /// Sperre; ein Recht, das nur genommen und dann fallengelassen wuerde,
    /// liesse die naechste Instanz sich fuer die erste halten. Gefragt wird es
    /// genau einmal, beim Start: wer es hat, bekommt einen
    /// [`Sitzungsschreiber`], wer nicht, bekommt keinen. Die Regel „nur die
    /// Halterin schreibt die Sitzung" haelt danach der Uebersetzer —
    /// [`Sitzungsschreiber::neu`] verlangt das Recht als Argument und liefert
    /// ohne es `None`.
    ///
    /// Leer, solange `sitzung_laden` nicht gelaufen ist, und in den vier
    /// Messmodus-Faellen, die keinen bleibenden Ablageordner oeffnen.
    sitzungsrecht: OnceCell<Sitzungsrecht>,
    /// Der eine Eintrittspunkt fuer Tastendruecke.
    ///
    /// Veraenderlich seit Schritt 20: der Abgriff haelt seine Belegung selbst,
    /// und nach einer Umbelegung wird er fallen gelassen und mit der neuen
    /// Belegung neu eingerichtet, statt zwei Staende nebeneinander zu halten.
    tastenabgriff: RefCell<Option<Tastenabgriff>>,
    /// Die offene Belegungsansicht aus C3, falls eine steht.
    ///
    /// Ihr Blattgriff liegt daneben in [`Self::offenes_blatt`], damit `esc`
    /// sie wie jede Rueckfrage schliesst. Hier steht die Quelle, weil der
    /// Faenger des Ereignisabgriffs sie waehrend der Aufnahme braucht und der
    /// Abschluss ihr die Arbeitskopie abnimmt.
    belegungsansicht: RefCell<Option<Retained<Belegungsquelle>>>,
    /// Die Beobachtung der sichtbaren Ordner (C9).
    ///
    /// Veraenderlich und nicht einmalig wie die uebrigen Halter: ein
    /// `FSEventStream` aendert seine Pfadliste nach dem Anlegen nicht mehr,
    /// also wird bei jeder Navigation ein neuer eingerichtet und der alte
    /// fallen gelassen. Leer, solange kein Ordner feststeht, und dann, wenn
    /// sich der Strom nicht einrichten liess.
    dateisystemwache: RefCell<Option<Dateisystemwache>>,
    /// Die Beobachtung der Datentraeger (C9). Sie steht fuer die ganze
    /// Laufzeit, weil sie an keinem Pfad haengt.
    datentraegerwache: OnceCell<Datentraegerwache>,
    /// Der gebuendelte Schreiber fuer `session.toml`.
    ///
    /// Leer im Messmodus und dann, wenn sich der Ablageordner nicht oeffnen
    /// liess. Im zweiten Fall steht die Meldung dazu in der Statuszeile.
    sitzungsschreiber: RefCell<Option<Sitzungsschreiber>>,
    /// Ob eine Meldung ueber einen gescheiterten Schreibvorgang schon steht.
    ///
    /// Ohne dieses Kennzeichen ueberschriebe ein dauerhaft scheiternder
    /// Schreibvorgang alle zwei Sekunden jede andere Meldung.
    schreibfehler_gemeldet: Cell<bool>,
    /// Die laufende Dateioperation aus C4, falls eine laeuft.
    vorgang: RefCell<Option<Vorgang>>,
    /// Ein Blatt, das auf eine Antwort des Nutzers wartet: die Konfliktfrage,
    /// die Rueckfrage vor dem Raeumen in den Papierkorb oder die
    /// Abschlussliste.
    ///
    /// Es steht hier, damit die Escape-Taste es schliessen kann. Ein `NSButton`
    /// traegt genau eine Tastenentsprechung, und die Eingabetaste liegt in der
    /// Rueckfrage auf "Abbrechen"; der zweite Weg zum Abbruch laeuft deshalb
    /// ueber den Befehl `abbrechen` aus `resources/default-keymap.toml`.
    offenes_blatt: RefCell<Option<Blattgriff>>,
    /// Ob das laufende Beenden an der Nachfrage aus C4 vorbeigeht.
    ///
    /// **Ein Feld, ein Schreiber, ein Leser.** Geschrieben allein von
    /// [`Anwendungsdelegierter::ohne_tastenabgriff_beenden`], gelesen allein von
    /// `applicationShouldTerminate:`. Dort ist der Tastenabgriff kaputt, es
    /// steht bereits ein anwendungsmodaler Hinweis, und ein Blatt mit Rueckfrage
    /// waere weder bedienbar noch sinnvoll — der Nutzer koennte es nicht
    /// beantworten, und KRK bliebe stehen.
    beenden_ohne_nachfrage: Cell<bool>,
    /// Ob die laufende Wiederholung der Rueckschritt-Taste bei **stehendem**
    /// Filtertext begonnen hat (C1.18, C1.20).
    ///
    /// **Das eine Bit, das `isARepeat` nicht mitbringt.** AppKit meldet an
    /// einem Tastenereignis allein, ob es eine Wiederholung ist, und nicht,
    /// wobei die Wiederholung anfing; die Frage aus C1.18 lautet aber genau
    /// das. Der Modulkopf von [`crate::kommandos::rueckschritt`] schreibt aus,
    /// warum es ohne dieses Bit nicht geht.
    ///
    /// **Es wohnt hier und nicht am Tab.** Eine Tastenwiederholung gehoert
    /// keinem Tab und keinem Dateifenster: ein Tabwechsel braucht einen anderen
    /// Tastendruck, und der beendet die Wiederholung. Je Tab gehalten waere
    /// dasselbe Faktum N-mal da.
    ///
    /// Fortgeschrieben wird es allein von der Regel in
    /// [`Self::papierkorb_oder_zeichen_zurueck`]; zurueckgesetzt wird es von
    /// jeder anderen Eingabe, am Kopf von [`Self::eingabe_ausfuehren`].
    rueckschritt_merker: Cell<bool>,
    /// Die Stelle, auf die der laufende Ladevorgang des Editors springen soll
    /// (C6): gemerkte Zeilennummer und gemerkter Zeileninhalt.
    ///
    /// **Warum die Auskunft warten muss.** Der Sprung auf eine Textmarke
    /// oeffnet ihre Datei ueber [`Editorbereich::datei_oeffnen`], und das kehrt
    /// seit S24 sofort zurueck, ohne gelesen zu haben. Wohin die Schreibmarke
    /// gehoert, laesst sich erst am Text entscheiden, und der steht erst beim
    /// Ausgang. Das Paar ist dieselbe Form, die
    /// [`Editorbereich::schreibmarkenzeile`] beim Anlegen liefert.
    ///
    /// **Ein Schreiber je Anlass, ein Leser, und der Leser verbraucht es**, wie
    /// bei [`Self::beenden_ohne_nachfrage`] darueber:
    /// [`Anwendungsdelegierter::editorausgang_behandeln`] nimmt es beim ersten
    /// Ausgang heraus. Zwei Wege legen es zurueck oder ab, und beide gehoeren
    /// zur Rueckhaltung aus C4: [`Ladeausgang::Zurueckgehalten`] legt es
    /// zurueck, weil die zurueckgehaltene Datei genau die der Marke ist, und
    /// bricht der Nutzer die Nachfrage ab, faellt es mit ihr. Eine abgewiesene
    /// Datei laesst es ohne eine eigene Zeile fallen: es ist dann schon
    /// herausgenommen.
    vorgemerkte_marke: RefCell<Option<(u32, String)>>,
    /// Was die beiden Notizzettel der Runde 9 tragen und welcher offen ist.
    ///
    /// Es steht hier und nicht im Blatt, weil es das Blatt ueberdauert: der
    /// Zettel geht zu und wieder auf, und beim Aufgehen soll derselbe Tab offen
    /// sein. Was daran den **Neustart** ueberdauert, ist die Zettelwahl in der
    /// `session.toml`, und die traegt `Sitzung`.
    zettel: RefCell<Zettelmodell>,
    /// Die Textflaeche des stehenden Notizzettels, falls einer steht.
    ///
    /// **Der Delegierte haelt sie, weil er ihren Stand von aussen braucht.**
    /// Der Zettel wird nicht nur ueber sein eigenes Blatt verlassen: `cmd+q` und
    /// `shift+cmd+w` treffen ihn, waehrend er steht, und beide muessen den
    /// getippten Text sichern. Ohne einen Griff auf die Flaeche haette der
    /// Delegierte in diesem Augenblick keinen Zugang zu ihm.
    ///
    /// Leer, solange kein Zettel steht. Der Abschluss des Blattes nimmt den
    /// Stand ein letztes Mal ab und raeumt sie ab.
    zettelflaeche: RefCell<Option<Retained<NSTextView>>>,
    /// Der Ablauf der Messung. Der Bildtakt haelt eine zweite Referenz.
    messlauf: OnceCell<Rc<RefCell<Messlauf>>>,
    zeichenende: OnceCell<Zeichenende>,
    /// Der Zeitgeber, der den naechsten Messschritt anstoesst.
    ausloesetakt: OnceCell<Retained<NSTimer>>,
}

define_class!(
    /// Der Anwendungsdelegierte.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = AnwendungsIvars]
    pub struct Anwendungsdelegierter;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Anwendungsdelegierter {}

    impl Anwendungsdelegierter {
        /// Der Rueckruf des Ausloesetakts.
        // SAFETY: Die Signatur passt zu der, die NSTimer aufruft.
        #[unsafe(method(messSchritt:))]
        fn mess_schritt(&self, _zeitgeber: &NSTimer) {
            self.messen_weiter();
        }

        /// Der eine Selektor jedes Menueeintrags, der ein [`Kommando`] traegt
        /// (C2.14).
        ///
        /// **Ein Menueeintrag geht denselben Weg wie ein Tastendruck.** Er
        /// ruft [`Self::kommando_ausfuehren`] und damit die eine Stelle, die
        /// entscheidet, wohin ein Befehl geht; ein zweiter Ausfuehrungsweg
        /// entsteht nicht. Bis zur Runde 7 liefen drei Eintraege — "KRK
        /// beenden", "Fenster einblenden" und "Fenster schliessen" — ueber je
        /// einen eigenen Selektor und damit **an** `kommando_ausfuehren`
        /// **vorbei**; mit einem Kuerzel an jedem Eintrag der Leiste waere
        /// daraus eine Regel geworden statt einer Ausnahme.
        ///
        /// **Und es ist trotzdem nicht `terminate:`.** Der Grund, aus dem
        /// "KRK beenden" seinerzeit einen eigenen Selektor bekam, war die
        /// Zweitform "Quit and Keep Windows", die AppKit zu einem Eintrag mit
        /// `terminate:` von sich aus auf Opt+Cmd+Q dazustellt. Der
        /// Sammelselektor ist so wenig `terminate:` wie `beenden:` es war, und
        /// [`Self::beenden`] ruft `terminate:` weiterhin selbst.
        ///
        /// Welchen Befehl der Eintrag meint, steht in seinem `tag`; die
        /// Uebersetzung dorthin und zurueck steht in [`menue`], weil `tag` ein
        /// AppKit-Begriff ist. Ein Absender ohne brauchbaren `tag` tut nichts:
        /// die Ausgrauung in `validateMenuItem:` weist denselben Eintrag schon
        /// ab, und ein Absturz waere hier die falsche Antwort.
        // SAFETY: Die Signatur ist die einer gewoehnlichen Menueaktion: ein
        // Argument, der Absender.
        #[unsafe(method(krkKommando:))]
        fn krk_kommando(&self, absender: Option<&NSMenuItem>) {
            let Some(absender) = absender else {
                return;
            };
            let Some(kommando) = menue::kommando_zum_tag(absender.tag()) else {
                return;
            };
            // Der Rueckgabewert sagt, ob der Befehl zulaessig war; wer den
            // Eintrag angeklickt hat, hat ihn nicht ausgegraut vorgefunden, und
            // der Abgriff wartet hier auf keine Antwort.
            //
            // **Kein Anschlag**, und das ist die Aussage und keine Auslassung:
            // einen Menueeintrag anzuklicken ist kein Tastendruck. Damit
            // bekommt "In den Papierkorb raeumen" die Fallunterscheidung der
            // Rueckschritt-Taste nicht ab und raeumt aus dem Menue heraus
            // immer (C1.19, C6.11).
            let _ = self.kommando_ausfuehren(kommando, None);
        }

        /// Der Menueeintrag "Tastenbelegung als Markdown sichern" (C1 der
        /// Runde 3).
        ///
        /// Wie die drei Eintraege darueber ohne festes Ziel und ueber die
        /// Antwortkette. Hier zaehlt der Grund doppelt: der Eintrag soll auch
        /// dann auswaehlbar sein, wenn kein Dateifenster den Fokus haelt, und
        /// die Kette endet bei `NSApplication` und ihrem Delegierten.
        // SAFETY: Die Signatur ist die einer gewoehnlichen Menueaktion: ein
        // Argument, der Absender.
        #[unsafe(method(tastenbelegungSichern:))]
        fn tastenbelegung_sichern_aktion(&self, _absender: Option<&AnyObject>) {
            self.tastenbelegung_sichern();
        }
    }

    /// Die Ausgrauung des Hauptmenues, und sie fragt dieselbe Regel wie der
    /// Ereignisabgriff (C2.5, C2.16).
    // SAFETY: `NSMenuItemValidation` stellt keine Bedingungen ueber die
    // Signatur hinaus, und die ist die des Protokolls.
    unsafe impl NSMenuItemValidation for Anwendungsdelegierter {
        /// Ob dieser Eintrag jetzt bedienbar ist.
        ///
        /// **Der zweite Frager von [`zulaessigkeit::zulaessig`], und er baut
        /// keine eigene Regel.** Der erste ist
        /// [`Self::kommando_ausfuehren`]. Beide fragen dieselbe Funktion auf
        /// derselben [`Lage`] aus [`Self::lage`]; ihre Antworten koennen
        /// deshalb nicht auseinanderlaufen, und genau daran haengt diese Runde.
        /// Ein freigegebener Eintrag zu einem abgewiesenen Tastendruck fuehrte
        /// den Befehl aus, den der Abgriff eben verweigert hat: mit dem Fokus
        /// im Editor bewegte ein Auf-Pfeil dann die Dateiliste.
        ///
        /// **Zuerst die Aktion, dann der `tag`.** Der Vorgabewert eines `tag`
        /// ist Null, und Null ist ein gueltiger Index in
        /// [`Kommando::KENNUNGEN`]. Fuer jede fremde Aktion antwortet diese
        /// Methode deshalb `true` und ueberlaesst AppKit seine gewohnte
        /// Entscheidung; die sechs Textbefehle (C2.8) und der Eintrag der
        /// Markdown-Ausgabe (C2.9) behalten damit genau das Verhalten, das sie
        /// heute haben, und ihre Ausgrauung kommt weiter aus der Antwortkette.
        ///
        /// **Kein Beobachter am Fokus, und das ist kein Versehen.** Eine
        /// Anzeige, die dem Fokus folgt, gehoert nach `CLAUDE.md` an die
        /// Ueberschreibung von `makeFirstResponder:` in [`super::fenster`].
        /// Hier wird nichts gesetzt, sondern gefragt: AppKit erhebt die
        /// Zulaessigkeit von sich aus, bevor es ein Menue oeffnet oder eine
        /// Tastenentsprechung zustellt. Ein zweiter Beobachter waere ein
        /// zweiter Weg zu derselben Antwort.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(validateMenuItem:))]
        fn eintrag_pruefen(&self, eintrag: &NSMenuItem) -> bool {
            // Ohne `return`, und das ist kein Geschmack: `define_class!` setzt
            // den Rumpf in eine Huelle mit dem Rueckgabetyp `Bool`, und ein
            // `return` verliesse die Huelle statt den Rumpf.
            if eintrag.action() == Some(Sel::register(menue::KRK_KOMMANDO)) {
                match menue::kommando_zum_tag(eintrag.tag()) {
                    Some(kommando) => zulaessigkeit::zulaessig(kommando, self.lage()),
                    // Ein `tag`, den `KENNUNGEN` nicht fuehrt, ist ein
                    // Programmfehler. Grau ist dafuer die richtige Anzeige: der
                    // Eintrag taete ohnehin nichts.
                    None => false,
                }
            } else {
                true
            }
        }
    }

    // SAFETY: `NSApplicationDelegate` stellt keine Bedingungen.
    unsafe impl NSApplicationDelegate for Anwendungsdelegierter {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn start_abgeschlossen(&self, _meldung: &NSNotification) {
            self.oberflaeche_aufbauen();
        }

        /// Der Klick auf das Dock-Symbol (C7).
        ///
        /// Der zweite der beiden Wege zurueck zum geschlossenen Fenster. Er
        /// liefert `false`, weil KRK das Fenster selbst nach vorn holt und
        /// AppKit nichts weiter tun soll.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(applicationShouldHandleReopen:hasVisibleWindows:))]
        fn wieder_geoeffnet(&self, _absender: &NSApplication, sichtbare_fenster: bool) -> bool {
            if !sichtbare_fenster {
                self.fenster_zeigen();
            }
            false
        }

        /// KRK soll beendet werden: haelt der Editor ungesicherten Stand,
        /// steht die Nachfrage aus C4 davor.
        ///
        /// **Der dritte Anlass der Nachfrage, und der einzige, der eine Antwort
        /// an AppKit zurueckgeben muss.** Ein Blatt kehrt sofort zurueck, und
        /// `terminate:` darf nicht auf eine Rueckgabe warten; deshalb
        /// `TerminateLater` und die endgueltige Antwort aus dem Rueckruf ueber
        /// `replyToApplicationShouldTerminate:`. Das ist der Weg, den AppKit
        /// fuer genau diesen Fall vorsieht.
        ///
        /// Weil die Antwort vom 260808-0017 den Anlass Sitzungssicherung in das
        /// Beenden hineingezogen hat, ist diese Stelle die einzige, an der ein
        /// ungesicherter Stand vor einem Programmende ueberhaupt bemerkt wird.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(applicationShouldTerminate:))]
        fn soll_beendet_werden(&self, _absender: &NSApplication) -> NSApplicationTerminateReply {
            self.beenden_erlauben()
        }

        /// KRK wird beendet: den letzten Sitzungsstand schreiben.
        ///
        /// Der eine Schreibvorgang ohne Ruecksicht auf den Takt, den
        /// `### Frage 4` des Plans neben der Buendelung zusagt.
        ///
        /// **Unveraendert seit der Runde 1, und das ist die Zusage:** dieser
        /// Rueckruf laeuft **nach** der Zustimmung aus
        /// `applicationShouldTerminate:` und nicht vor ihr. Ein abgebrochenes
        /// Beenden erreicht ihn nie, und die getaktete Sitzungssicherung traegt
        /// den ungesicherten Stand weiterhin nicht mit; das sechste
        /// Abnahmekriterium von C4 verlangt beides.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(applicationWillTerminate:))]
        fn wird_beendet(&self, _meldung: &NSNotification) {
            // **Kein `sitzung_vormerken()` davor.** Bis zur Runde 7 stand hier
            // eines, und es nahm die Schreibsperre ein zweites Mal: beim
            // Beenden liefen damit zwei Durchgaenge hintereinander, und genau
            // dazwischen konnte eine andere Instanz schreiben — der Fall, den
            // der Kommentar unten ausschliesst. Wirkungslos war es dazu, denn
            // die Zeilen darunter bauen denselben Stand und ueberschreiben den
            // vorgemerkten
            // (`issues/260813-0540_*_beim-beenden-laufen-zwei-durchgaenge-und-der-kommentar-nennt-einen.md`).
            let sitzung = self.sitzung_bauen();
            self.zettel_stand_uebernehmen();
            let mut schreiber = self.ivars().sitzungsschreiber.borrow_mut();
            let jetzt = Instant::now();
            // **Ein Durchgang und nicht zwei.** Das Vormerken und das Beenden
            // laufen unter derselben Schreibsperre; zwei Durchgaenge liessen
            // dazwischen eine andere Instanz schreiben, ohne dass es einen
            // Grund dafuer gaebe. Der Zettel teilt sich denselben Durchgang und
            // nimmt aus demselben Grund keinen eigenen.
            let _ = self.unter_der_sperre(|zugang| {
                // **Der vierte Sicherungsmoment aus C4** (Runde 9). Er haengt an
                // diesem Rueckruf und nicht am Tastendruck, und damit faellt die
                // Bedingung des Kriteriums von selbst: AppKit ruft
                // `applicationWillTerminate:` erst **nach** der Zustimmung aus
                // `applicationShouldTerminate:`. Weist `beenden_erlauben` das
                // Beenden ab, kommt der Rueckruf nie, und der Zettel sichert
                // nicht. Es gibt dafuer nichts abzufragen.
                //
                // **Dieser Moment ist der letzte, und deshalb schreibt er jeden
                // abweichenden Zettel.** Die drei anderen duerfen einen
                // Fehlschlag an den naechsten Moment weiterreichen; nach diesem
                // hier laeuft nichts mehr, das ihn nachholte. Was „jeden"
                // heisst, steht in `zettel_sichern` und nicht hier.
                //
                // Der Rueckgabewert ist der Satz fuer die Statuszeile. Beim
                // Beenden gibt es keine mehr, an der er ankaeme; deshalb steht
                // hier `let _ =` und kein Melder. **Der Preis ist benannt und
                // angenommen:** scheitert die Sicherung hier, erfaehrt der
                // Nutzer es nicht. Ein Fenster dafuer waere eine Rueckfrage
                // beim Beenden, und die fuehrt diese Runde ausdruecklich nicht
                // — der Spec bindet die Meldezusage an die drei Momente, nach
                // denen KRK weiterlaeuft, und fuehrt die Alternative unter
                // „Ausdruecklich ausserhalb dieser Runde".
                let _ = self.zettel_sichern(zugang);
                // Kein Schreiber heisst: kein Sitzungsrecht, oder kein
                // Ablageordner. Beides hat der Start gemeldet, und beim
                // Beenden gibt es dafuer keine Statuszeile mehr. **Der Zettel
                // haengt nicht daran**: er wird auch von der Instanz gesichert,
                // die die Sitzung nicht schreiben darf — sonst verloere die
                // zweite Instanz von KRK ihren Zettel beim Beenden, waehrend
                // C4 den Preis zweier Instanzen ausdruecklich nur auf
                // „die zuletzt schliessende gewinnt" beschraenkt.
                if let Some(schreiber) = schreiber.as_mut() {
                    let _ = schreiber.vormerken(sitzung, jetzt, zugang);
                    let _ = schreiber.beenden(jetzt, zugang);
                }
            });
        }
    }
);

/// Warum ein Durchgang durch die Ablage nicht zustande kam.
///
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig. Sie steht neben dem
/// Delegierten und nicht im Kern: der Kern kennt nur den zweiten Fall, den
/// Fehlschlag beim Nehmen der Sperre. Dass es ueberhaupt keinen Ablageordner
/// gibt, ist eine Lage der Oberflaeche, und sie ist kein Fehler — KRK laeuft
/// dann und sichert nicht.
#[derive(Debug)]
enum Sperrhindernis {
    /// Es gibt keinen Ablageordner. Der Start hat es gemeldet.
    OhneOrdner,
    /// Die Schreibsperre liess sich nicht nehmen.
    Gesperrt(std::io::Error),
}

/// Welches Fenster von KRK aus gesehen gerade das Schluesselfenster ist.
///
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig, erhoben aus
/// `NSApplication::keyWindow`. Sie steht hier und nicht in
/// [`crate::kommandos::zulaessigkeit`], weil sie AppKit fragt; was die Regel
/// daraus macht, steht dort.
///
/// **`Fremd` deckt zwei Lagen, und beide sollen dieselbe Antwort bekommen:** ein
/// fremdes Fenster im Vordergrund — das freistehende Panel des Ueber-Dialogs so
/// gut wie das Fenster einer anderen Anwendung — und ein KRK ohne
/// Schluesselfenster, also im Hintergrund. In beiden Faellen darf kein
/// Tastenbefehl auf das Hauptfenster wirken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Schluesselfenster {
    /// KRKs Hauptfenster selbst.
    Hauptfenster,
    /// Ein Blatt, das am Hauptfenster haengt.
    ///
    /// Es **ist** das Schluesselfenster und deshalb ein eigener Wert und kein
    /// `Fremd`. Ueber ein stehendes Blatt entscheidet
    /// [`Anwendungsdelegierter::blatt_steht`] zusammen mit
    /// [`kommandos::operationen::waehrend_blatt_erlaubt`](crate::kommandos::operationen::waehrend_blatt_erlaubt),
    /// und diese Aufteilung bleibt, wo sie ist.
    BlattAmHauptfenster,
    /// Ein fremdes Fenster, oder gar keines.
    Fremd,
}

impl Schluesselfenster {
    /// Ob das Schluesselfenster KRK gehoert.
    ///
    /// Der vierte Bestandteil der Zulaessigkeitsregel, als Wahrheitswert fuer
    /// [`Lage::schluesselfenster_gehoert_krk`]. Die Fallunterscheidung ist
    /// vollstaendig und hat keinen Auffangzweig: ein vierter Wert haelt hier den
    /// Bau an und erzwingt eine bewusste Einordnung.
    fn gehoert_krk(self) -> bool {
        match self {
            Self::Hauptfenster | Self::BlattAmHauptfenster => true,
            Self::Fremd => false,
        }
    }
}

impl Anwendungsdelegierter {
    /// Einen Anwendungsdelegierten ohne Oberflaeche.
    fn neu(
        mtm: MainThreadMarker,
        tasten_protokoll: bool,
        messaufgabe: Option<Aufgabe>,
        belegung: Belegung,
        belegungsmeldung: Option<String>,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AnwendungsIvars {
            tasten_protokoll,
            messaufgabe,
            belegung: RefCell::new(belegung),
            einstellungen: RefCell::new(Einstellungen::default()),
            belegungsmeldung,
            modell: RefCell::new(Fenstermodell::aus_sitzung(&Sitzung::default())),
            fenster: OnceCell::new(),
            fenster_delegierter: OnceCell::new(),
            aufteilung: OnceCell::new(),
            bereichsleiste: OnceCell::new(),
            statuszeile: OnceCell::new(),
            dateifenster: OnceCell::new(),
            leiste: OnceCell::new(),
            vorschau: OnceCell::new(),
            editor: OnceCell::new(),
            vorschau_nachtrag: RefCell::new(None),
            ablage: RefCell::new(None),
            sitzungsrecht: OnceCell::new(),
            tastenabgriff: RefCell::new(None),
            belegungsansicht: RefCell::new(None),
            dateisystemwache: RefCell::new(None),
            datentraegerwache: OnceCell::new(),
            sitzungsschreiber: RefCell::new(None),
            schreibfehler_gemeldet: Cell::new(false),
            vorgang: RefCell::new(None),
            offenes_blatt: RefCell::new(None),
            beenden_ohne_nachfrage: Cell::new(false),
            rueckschritt_merker: Cell::new(false),
            vorgemerkte_marke: RefCell::new(None),
            zettel: RefCell::new(Zettelmodell::default()),
            zettelflaeche: RefCell::new(None),
            messlauf: OnceCell::new(),
            zeichenende: OnceCell::new(),
            ausloesetakt: OnceCell::new(),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Baut die vier Bereiche, stellt die Sitzung her und liest die Ordner.
    fn oberflaeche_aufbauen(&self) {
        let mtm = self.mtm();
        let ivars = self.ivars();

        let (sitzung, mut meldungen) = self.sitzung_laden();
        *ivars.modell.borrow_mut() = Fenstermodell::aus_sitzung(&sitzung);
        // **Welcher Zettel offen war, kommt aus der Sitzung; sein Text nicht.**
        // Die Zetteldateien werden beim Start nicht gelesen — C4 sagt es zu, und
        // der Spec haengt daran das Verhaeltnis zur Zeitzusage L4. Gelesen wird
        // erst beim ersten Oeffnen des Blattes, in `notizzettel_zeigen`.
        ivars.zettel.borrow_mut().offenen_setzen(sitzung.zettel);

        let dateifenster = [
            Dateifenster::bauen(mtm, Tabliste::aus_zustand(&sitzung.fenster[0])),
            Dateifenster::bauen(mtm, Tabliste::aus_zustand(&sitzung.fenster[1])),
        ];
        let leiste = Leiste::bauen(mtm);
        let vorschau = Vorschaufenster::bauen(mtm);
        let editor = Editorbereich::bauen(mtm);
        // **Der Rueckweg des Editors.** Seit S24 liest er auf einem
        // Arbeitsfaden, und wie ein Oeffnen ausgegangen ist, steht erst fest,
        // wenn der Befehl, der es angefordert hat, laengst zurueck ist. Der
        // Ausgang bringt deshalb die `Oeffnungsherkunft` mit, die der Befehl
        // beim Oeffnen genannt hat, statt sie hier in einem Feld zu erwarten;
        // die Begruendung steht an `Editorbereich::datei_oeffnen`.
        //
        // Der Rueckruf haelt den Delegierten **schwach**, aus demselben Grund wie
        // die vier anderen Melder hier: sonst schloesse sich der Ring
        // Delegierter → Editorbereich → Rueckruf → Delegierter.
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        editor.melder_setzen(Box::new(move |ausgang, herkunft| {
            if let Some(selbst) = schwach.load() {
                selbst.editorausgang_behandeln(ausgang, herkunft);
            }
        }));
        let aufteilung = Aufteilung::bauen(
            mtm,
            [&dateifenster[0], &dateifenster[1]],
            leiste.sicht(),
            vorschau.sicht(),
            editor.sicht(),
        );
        let fenster_delegierter = FensterDelegierter::neu(
            mtm,
            [
                dateifenster[0].quelle().retain(),
                dateifenster[1].quelle().retain(),
            ],
        );
        // **Die Leiste am Fensterfuss, darueber die eine Statuszeile, darueber
        // die Fensterzeile.** Alle drei liegen in derselben Traegerflaeche;
        // weder Leiste noch Zeile sind eine Unteransicht der Aufteilung, weil
        // `ersthelferbereich` deren fuenf Bereiche durchgeht und eine Ansicht
        // darin ein sechster Bereich waere.
        let bereichsleiste = Bereichsleiste::bauen(mtm);
        let statuszeile = Statuszeile::bauen(mtm);
        let inhalt = fenster::fensterinhalt(
            mtm,
            aufteilung.sicht(),
            statuszeile.sicht(),
            bereichsleiste.sicht(),
        );
        let fenster = fenster::hauptfenster(mtm, &inhalt, &fenster_delegierter);

        // **Der eine Ausloesepunkt der Fokusanzeige aus C9.** Das Fenster
        // meldet jeden erfolgreichen Wechsel des Ersthelfers und jeden Wechsel
        // zwischen Vorder- und Hintergrund; damit folgt die Anzeige dem Fokus
        // auch dort, wo nicht KRK ihn gesetzt hat, naemlich beim Mausklick in
        // eine Flaeche. Der Rueckruf haelt den Delegierten **schwach**, aus
        // demselben Grund wie die vier darunter: sonst schloesse sich der Ring
        // Delegierter → Fenster → Rueckruf → Delegierter, und das Fenster lebt
        // ueber sein Schliessen hinaus.
        //
        // **Zwei Empfaenger haengen daran, und der erste ist der neuere.**
        // `aktives_dem_ersthelfer_nachziehen` setzt den Nutzerentscheid vom
        // 260819 um: liegt der Rang nach dem Wechsel in einem Dateifenster, ist
        // dieses das aktive. Es steht **vor** dem Nachzug der Anzeige, damit
        // die schon mit dem neuen `aktiv` rechnet; bis dahin malte der Klick
        // auf eine Zeile den Rahmen einmal falsch und liess ihn erst von
        // `aktives_setzen` berichtigen.
        //
        // **Der Nachzug der Anzeige steht trotzdem unbedingt daneben.** Hat das
        // aktive Dateifenster gewechselt, schreibt `aufteilung_nachziehen` die
        // Farben schon, und diese Zeile schreibt dieselben Werte ein zweites
        // Mal. Das ist der billigere Fehler: eine Bedingung davor hinge daran,
        // dass `aktives_setzen` die Anzeige mitnimmt, und liesse sie still
        // ausfallen, sobald jemand das aendert. Der eine Schreiber der Anzeige
        // laeuft auf diesem Ausloesepunkt immer.
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        fenster.melder_setzen(Box::new(move || {
            if let Some(selbst) = schwach.load() {
                selbst.aktives_dem_ersthelfer_nachziehen();
                selbst.fokusanzeige_nachziehen();
            }
        }));
        // **Der Klick auf einen Schalter der Bereichsleiste geht denselben Weg
        // wie der Tastenbefehl** (C2.2): die Leiste kennt nur ihr Kommando,
        // ausgefuehrt wird es in `kommando_ausfuehren` samt Blattpruefung,
        // Fokusvorbehalt und Abweisung im Modell. **Kein Nachzug daneben**: die
        // Selbstkippung des Ankreuzfelds nimmt `Leistenquelle::geklickt` schon
        // zurueck, bevor das Kommando hier ankommt, und den Rest schreibt
        // `bereichsleiste_nachziehen` als der eine Schreiber. Eine zweite Zeile
        // hier liefe nach einem angenommenen Klick zusaetzlich zu jenem. **Kein
        // Anschlag**, aus demselben Grund wie beim Menueeintrag: ein Klick auf
        // einen Schalter ist kein Tastendruck. Der Rueckruf haelt den
        // Delegierten **schwach**, aus demselben Grund wie die uebrigen Melder
        // hier.
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        bereichsleiste.melder_setzen(Box::new(move |kommando| {
            if let Some(selbst) = schwach.load() {
                selbst.kommando_ausfuehren(kommando, None);
            }
        }));

        // Ab hier nur noch als `NSWindow`: jede uebrige Fensterberuehrung ruft
        // ohnehin nur Methoden der Oberklasse.
        let fenster = Retained::into_super(fenster);

        // Erst festhalten, dann anzeigen: das Fenster haelt seinen Delegierten
        // schwach, die Tabelle haelt Datenquelle und Delegierten schwach.
        let _ = ivars.dateifenster.set(dateifenster);
        let _ = ivars.leiste.set(leiste);
        let _ = ivars.vorschau.set(vorschau);
        let _ = ivars.editor.set(editor);
        let _ = ivars.aufteilung.set(aufteilung);
        let _ = ivars.bereichsleiste.set(bereichsleiste);
        let _ = ivars.statuszeile.set(statuszeile);
        let _ = ivars.fenster_delegierter.set(fenster_delegierter);
        let _ = ivars.fenster.set(fenster);

        // Ein Klick in eine der beiden Listen macht sie zur aktiven. Der
        // Rueckruf haelt den Delegierten **schwach**, sonst schloesse sich der
        // Ring Delegierter → Dateifenster → Quelle → Rueckruf → Delegierter.
        for seite in Fensterseite::ALLE {
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .aktivierung_setzen(Box::new(move || {
                    if let Some(selbst) = schwach.load() {
                        selbst.aktives_setzen(seite);
                    }
                }));
            // Jede Navigation setzt die Dateisystembeobachtung neu auf und
            // schreibt den Fenstertitel neu (C11): der Ordnerwechsel und der
            // Tabwechsel eines Dateifensters sind der erste der vier Anlaesse,
            // an denen der genannte Pfad sich aendert. Auch dieser Rueckruf
            // haelt den Delegierten **schwach**, aus demselben Grund wie der
            // darueber.
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .ordnerwechsel_setzen(Box::new(move || {
                    if let Some(selbst) = schwach.load() {
                        selbst.dateisystemwache_nachziehen();
                        selbst.titel_nachziehen(selbst.fokus());
                        // **Und die Schalter "Deep" und "Content"**, denn sie
                        // gehoeren dem Tab und nicht dem Fenster: ein
                        // Tabwechsel und ein Ordnerwechsel koennen sie anders
                        // stehen lassen, ohne dass ein Befehl gelaufen waere
                        // (C2.3 der Inhaltsfilter-Runde). Der Nachzug steht
                        // **neben** den beiden darueber und nicht in ihnen,
                        // aus demselben Grund, aus dem die Statuszeile neben
                        // der Leiste steht: jede dieser Funktionen hat genau
                        // einen Gegenstand.
                        selbst.bereichsleiste_nachziehen();
                    }
                }));
            // Eine neue Auswahl fuellt den aktiven Vorschau-Tab (C6). Auch
            // dieser Rueckruf haelt den Delegierten **schwach**, aus demselben
            // Grund wie die beiden darueber.
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .auswahlmelder_setzen(Box::new(move |pfad| {
                    if let Some(selbst) = schwach.load() {
                        selbst.vorschau_fuellen(seite, pfad);
                    }
                }));
            // Das Umbenennen in der Liste aus C4. Die Zelle sammelt den Namen
            // und prueft ihn; ausgefuehrt wird er hier, weil die Auffrischung
            // **beide** Dateifenster erreichen muss und das von der Quelle aus
            // nicht geht. Auch dieser Rueckruf haelt den Delegierten
            // **schwach**, aus demselben Grund wie die beiden darueber.
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .umbenennung_setzen(Box::new(move |alt, neu| {
                    if let Some(selbst) = schwach.load() {
                        selbst.umbenennen_ausfuehren(seite, alt, neu);
                    }
                }));
            // **Die eine Statuszeile gehoert beiden Dateifenstern** (C5 der
            // Runde 6). Jedes meldet, dass eine seiner sechs Quellen sich
            // geaendert hat; welche der zwoelf Aussagen dann in der Zeile steht,
            // entscheidet `statuszeile_nachziehen` mit beiden Quellensaetzen
            // und der aktiven Seite. Der Rueckruf traegt die Seite nicht mit,
            // weil der Nachzug ohnehin beide fragt. Auch er haelt den
            // Delegierten **schwach**, aus demselben Grund wie die drei
            // darueber.
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .meldungswechsel_setzen(Box::new(move || {
                    if let Some(selbst) = schwach.load() {
                        selbst.statuszeile_nachziehen();
                    }
                }));
            // Der Abwurf aus einer fremden Anwendung (C4 bis C7 der Runde 13).
            // Drei Rueckrufe, weil die Dateiliste drei Dinge braucht, die sie
            // selbst nicht hat: die Frage nach dem laufenden Vorgang **ohne**
            // ihre Meldung — `validateDrop:` laeuft bei jeder Zeigerbewegung —,
            // den Weg in die Operationsmaschine und die Raeumung des Rangs 1
            // an **beiden** Dateifenstern, die von einer Quelle aus nicht zu
            // erreichen ist. Auch sie halten den Delegierten **schwach**, aus
            // demselben Grund wie die vier darueber.
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .vorgang_laeuft_setzen(Box::new(move || {
                    schwach
                        .load()
                        .is_some_and(|selbst| selbst.vorgang_laeuft().is_some())
                }));
            // **`seite` ist das Dateifenster, ueber dem der Zeiger stand**, und
            // nicht das aktive. Sie reist bis in den `Vorgang` mit; damit
            // erscheinen Fortschritt, Konfliktrueckfrage und Abschlusstext in
            // der Statuszeile jenes Dateifensters, wie C4 es verlangt.
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .abwurf_setzen(Box::new(move |ziel, quellen, art| {
                    if let Some(selbst) = schwach.load() {
                        selbst.abwurf_ausfuehren(seite, ziel, quellen, art);
                    }
                }));
            // **Die Meldung aus C7 nimmt dieselbe Loeschregel wie ein
            // Tastenbefehl**, und deshalb geht sie hier heraus statt an der
            // Quelle zu bleiben: der Rang 1 gehoert beiden Dateifenstern, und
            // eine Meldung im nicht aktiven verloere sonst gegen eine noch
            // stehende Befehlsantwort im aktiven. Der Rueckruf traegt die Seite
            // nicht mit, weil die Regel ohnehin beide raeumt. Auch er haelt den
            // Delegierten **schwach**, aus demselben Grund wie die fuenf
            // darueber.
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .befehlsantwort_raeumer_setzen(Box::new(move || {
                    if let Some(selbst) = schwach.load() {
                        selbst.befehlsantwort_beidseitig_loeschen();
                    }
                }));
        }

        self.aufteilung_nachziehen();
        // **Einmal beim Aufbau, damit die geladene Sitzung ankommt** (C7.2 der
        // Bereichsleisten-Runde). Die Tabelle baut ihre vier Spalten immer
        // sichtbar; welche davon der Nutzer weggeschaltet hatte, steht im
        // Modell und erreicht die Anzeige allein ueber diese Zeile.
        self.spaltenanzeige_nachziehen();
        self.leiste_einrichten(&mut meldungen);
        self.tastenabgriff_einrichten(&mut meldungen);
        self.datentraegerwache_einrichten();
        self.lesevorgaenge_starten();
        // **Der Start geht denselben Weg nach vorn wie die drei anderen**: der
        // Menueeintrag "Fenster einblenden", der Klick auf das Dock-Symbol und
        // das Kommando aus C7 rufen alle `fenster_zeigen`. Bis zum 260807 stand
        // hier ein nacktes `makeKeyAndOrderFront`, und das ordnet das Fenster
        // nur **innerhalb** von KRK nach vorn; vorderste Anwendung wird KRK erst
        // mit `activate()`. Ueber den Finder oder `open` gestartet nimmt
        // LaunchServices die Aktivierung ab, als Kindprozess eines Terminals
        // gestartet niemand. Die Sitzungsstrecke aus S21 misst nur, wenn KRK
        // vorn steht, und brach ohne diese Zeile mit `NICHT_IM_VORDERGRUND` ab.
        self.fenster_zeigen();
        // **Der Eingabefokus gehoert in das aktive Dateifenster, und zwar
        // nach dem `makeKeyAndOrderFront` in `fenster_zeigen`.** Setzte diese
        // Zeile davor, ueberschriebe AppKit sie beim ersten Anzeigen mit der
        // ersten Ansicht der Schluesselansichtskette; das ist seit S18 die
        // Leiste, und genau das war der Defekt vom 260805-1845. Das
        // `activate()` derselben Funktion aendert daran nichts: es macht KRK
        // zur vordersten Anwendung und ruehrt den ersten Beantworter des
        // Fensters nicht an. Eine eigene Zeile am Ende des Aufbaus
        // und keine Zeile der Sitzungswiederherstellung: der Fokus wird nicht
        // gespeichert, die Begruendung steht an `fokus::BEIM_START`. Aus der
        // Sitzung kommt allein, **welches** der beiden Dateifenster das aktive
        // ist.
        self.fokus_setzen(fokus::BEIM_START);
        // **Der Fenstertitel als letzte Handlung des Aufbaus (C11).** Erst
        // jetzt steht der Fokus, und der Titel folgt ihm. `appkit::fenster`
        // setzt ihn beim Aufbau des Fensters einmal auf die leere
        // Zeichenkette, weil Name und Version seit der Titelleisten-Runde im
        // eigenen Bereich links daneben stehen; diese Zeile setzt an seine
        // Stelle den Pfad, den das aktive Dateifenster zeigt.
        self.titel_nachziehen(self.fokus());
        // **Nach dem Fokus und nach dem Titel**, weil die Wiederherstellung
        // beide nicht anfassen darf; der Ruf steht deshalb hinter ihnen und
        // nicht bei den uebrigen Einrichtungen weiter oben.
        self.editor_wiederherstellen(&sitzung);
        // Die Startmeldungen betreffen die Anwendung und kein einzelnes
        // Dateifenster: die beschaedigte Belegungs- oder Sitzungsdatei, der
        // unerreichbare Ablageordner. Sie gehen deshalb in die Zeile des
        // aktiven Dateifensters, dieselbe Wahl wie bei der fehlgeschlagenen
        // Dateisystembeobachtung weiter unten. Bis zum 260804-1915 standen sie
        // fest im linken; hat die Sitzung das rechte als aktiv
        // wiederhergestellt, sah der Nutzer sie in der Zeile, auf die er nicht
        // blickt.
        let aktiv = self.ivars().modell.borrow().aktiv();
        for meldung in meldungen {
            self.dateifenster(aktiv).quelle().meldung_zeigen(&meldung);
        }
        self.messmodus_einrichten();
    }

    /// Warum ein Durchgang durch die Ablage nicht zustande kam.
    ///
    /// Zwei Faelle, und sie sind nicht derselbe: ohne Ablageordner hat der Start
    /// schon gemeldet, dass nichts gesichert wird, und ein zweiter Satz waere
    /// dieselbe Auskunft ein zweites Mal; eine Sperre, die sich nicht nehmen
    /// laesst, ist dagegen neu und gehoert gemeldet. Jeder Aufrufer von
    /// [`Anwendungsdelegierter::unter_der_sperre`] entscheidet beide einzeln.
    fn unter_der_sperre<T>(
        &self,
        arbeit: impl FnOnce(&Zugang<'_>) -> T,
    ) -> Result<T, Sperrhindernis> {
        match self.ivars().ablage.borrow().as_ref() {
            Some(ablage) => ablage.durchgang(arbeit).map_err(Sperrhindernis::Gesperrt),
            None => Err(Sperrhindernis::OhneOrdner),
        }
    }

    /// Laedt die Sitzung und den Ablageordner, oder liefert den
    /// Auslieferungszustand.
    ///
    /// Im Messmodus haengt der Weg an der Aufgabe, siehe den Modulkopf: die
    /// Strecken aus S8 laden nichts, die Sitzungsstrecke stellt die
    /// Pruefsitzung her, und der Sitzungsstart stellt sie wie ein
    /// gewoehnlicher Start aus `session.toml` wieder her. Ein
    /// Sitzungsschreiber entsteht in keinem der vier Faelle.
    fn sitzung_laden(&self) -> (Sitzung, Vec<String>) {
        let ivars = self.ivars();
        match &ivars.messaufgabe {
            None => {}
            Some(Aufgabe::Start { .. } | Aufgabe::Spannen { .. }) => {
                return (Sitzung::default(), Vec::new());
            }
            Some(Aufgabe::Sitzung { plan }) => {
                // Herstellen heisst schreiben: die folgenden L4-Starts finden
                // dieselbe Lage in `session.toml` vor. Scheitert das, gibt es
                // keine Zahl; still auf einer anderen Lage zu messen waere
                // die schlechteste aller Antworten.
                if let Err(meldung) = plan.herstellen() {
                    eprintln!("krk: {meldung}. Es wird keine Zahl ausgegeben.");
                    std::process::exit(4);
                }
                return (plan.sitzung.clone(), Vec::new());
            }
            Some(Aufgabe::SitzungsStart) => {
                let ablage = match Ablage::im_benutzerverzeichnis() {
                    Ok(ablage) => ablage,
                    Err(fehler) => {
                        eprintln!(
                            "krk: der Ablageordner laesst sich nicht oeffnen ({fehler}); \
                             ohne ihn gibt es keine Pruefsitzung und keine Zahl."
                        );
                        std::process::exit(4);
                    }
                };
                let geladen =
                    match ablage.durchgang(|zugang| zugang.laden::<Sitzung>(Datei::Sitzung)) {
                        Ok(geladen) => geladen,
                        Err(fehler) => {
                            eprintln!(
                                "krk: die Schreibsperre der Ablage laesst sich nicht nehmen \
                             ({fehler}); ohne sie gibt es keine Pruefsitzung und keine Zahl."
                            );
                            std::process::exit(4);
                        }
                    };
                if geladen.ist_ersetzt() {
                    eprintln!(
                        "krk: in session.toml steht keine lesbare Pruefsitzung. Der \
                         Sitzungslauf (--messmodus <plan.toml>) schreibt sie; fahre ihn \
                         zuerst. Es wird keine Zahl ausgegeben."
                    );
                    std::process::exit(4);
                }
                let (sitzung, _meldung) = geladen.mit_meldung();
                return (sitzung, Vec::new());
            }
        }
        let mut meldungen = Vec::new();
        let ablage = match Ablage::im_benutzerverzeichnis() {
            Ok(ablage) => ablage,
            Err(fehler) => {
                meldungen.push(format!(
                    "der Ablageordner liess sich nicht oeffnen, die Sitzung wird nicht gesichert: {fehler}"
                ));
                return (Sitzung::default(), meldungen);
            }
        };
        // **Das Sitzungsrecht zuerst, und ohne zu warten** (C3.9, C3.11 der
        // Runde 7). Wer es bekommt, schreibt die Sitzung; wer nicht, laeuft
        // ohne Sitzungsschreiber weiter und sagt es einmal. Ein zweiter Versuch
        // findet nie statt: die Zustaendigkeit wandert innerhalb eines
        // Prozesslebens nicht.
        //
        // Es steht **vor** dem Durchgang und nicht darin. Die beiden Sperren
        // sind zwei Absprachen mit zwei Lebensdauern, und die Reihenfolge ist
        // damit fest und ohne Ring; siehe den Kopf von
        // `krk_core::ablage::sperre`.
        let recht = match Sitzungsrecht::nehmen(ablage.ort()) {
            Ok(recht) => recht,
            Err(fehler) => {
                meldungen.push(format!(
                    "das Sitzungsrecht laesst sich nicht anfordern, die Sitzung wird nicht \
                     gesichert: {fehler}"
                ));
                Sitzungsrecht::ohne()
            }
        };
        *ivars.sitzungsschreiber.borrow_mut() = Sitzungsschreiber::neu(&recht);
        if !recht.gehalten() {
            // C3.10: eine Instanz, die die Sitzung nicht schreibt, sagt es beim
            // Start einmal. Der Satz nennt die Folge und nicht den Mechanismus:
            // was der Nutzer merkt, ist die nicht gemerkte Aufteilung.
            meldungen.push(OHNE_SITZUNGSRECHT.to_owned());
        }
        let _ = ivars.sitzungsrecht.set(recht);

        // **Ein Durchgang fuer beide Dateien.** Die Sitzung und die
        // Einstellungen werden unter derselben Schreibsperre gelesen; das Lesen
        // steht mit darunter, weil schon `Zugang::laden` schreibt, wenn eine
        // Datei beschaedigt ist und zur Seite gelegt wird.
        let gelesen = ablage.durchgang(|zugang| {
            let sitzung = zugang.laden::<Sitzung>(Datei::Sitzung).mit_meldung();
            // Die Einstellungen aus C11, ueber denselben Zugang. Der Aufruf legt
            // `settings.toml` beim ersten Start an; ohne diese Anlage haette der
            // Nutzer nichts zu pflegen, weil in dieser Runde keine Ansicht die
            // Datei schreibt.
            let eingestellt = einstellungen::laden(zugang).mit_meldung();
            (sitzung, eingestellt)
        });
        let ((sitzung, meldung), (eingestellt, meldung_einstellungen)) = match gelesen {
            Ok(beides) => beides,
            Err(fehler) => {
                meldungen.push(format!(
                    "die Schreibsperre der Ablage laesst sich nicht nehmen, es wird nichts \
                     geladen und nichts gesichert: {fehler}"
                ));
                return (Sitzung::default(), meldungen);
            }
        };
        meldungen.extend(meldung);
        *ivars.einstellungen.borrow_mut() = eingestellt;
        meldungen.extend(meldung_einstellungen);
        // Derselbe Zugang traegt die Lesezeichen aus C5. Er wird hier einmal
        // geoeffnet und nicht je Datei ein zweites Mal: `Ablage::oeffnen` legt
        // den Ordner an, und zweimal anzulegen hiesse, dieselbe Frage zweimal an
        // das Dateisystem zu stellen.
        *ivars.ablage.borrow_mut() = Some(ablage);
        (sitzung, meldungen)
    }

    // ------------------------------------------------------------------
    // Die Lesezeichen- und Geraeteleiste (C5)
    // ------------------------------------------------------------------

    /// Die Leiste (C5).
    fn leiste(&self) -> &Leiste {
        self.ivars()
            .leiste
            .get()
            .expect("die Leiste steht seit `oberflaeche_aufbauen`")
    }

    /// Das Vorschaufenster (C6).
    fn vorschau(&self) -> &Vorschaufenster {
        self.ivars()
            .vorschau
            .get()
            .expect("das Vorschaufenster steht seit `oberflaeche_aufbauen`")
    }

    /// Eine neue Auswahl fuellt den aktiven Vorschau-Tab (C6).
    ///
    /// Nur die Auswahl des **aktiven** Dateifensters: der Rueckruf kommt aus
    /// beiden, und die Vorschau zeigt, was vor dem Nutzer liegt. Eine
    /// aufgehobene Auswahl laesst den Tab stehen; das Zustandsdiagramm des
    /// Specs kennt allein die **neue** Auswahl als Ausloeser.
    ///
    /// **Bei ausgeblendeter Vorschau wird nichts gelesen.** Der Pfad geht
    /// dann in [`AnwendungsIvars::vorschau_nachtrag`], und das Einblenden aus
    /// C7 holt ihn nach; die Begruendung steht am Feld. Ein zweiter Weg in die
    /// Vorschau entsteht dabei nicht: nachgeholt wird mit demselben
    /// `datei_anzeigen`, das auch hier steht.
    fn vorschau_fuellen(&self, seite: Fensterseite, pfad: Option<PathBuf>) {
        if seite != self.ivars().modell.borrow().aktiv() {
            return;
        }
        let Some(pfad) = pfad else {
            return;
        };
        if !self.ivars().modell.borrow().sichtbar(Bereich::Vorschau) {
            *self.ivars().vorschau_nachtrag.borrow_mut() = Some(pfad);
            return;
        }
        // Gelesen wird sofort, also ist nichts mehr nachzuholen.
        *self.ivars().vorschau_nachtrag.borrow_mut() = None;
        self.vorschau().datei_anzeigen(&pfad);
    }

    /// Holt das Laden nach, das die ausgeblendete Vorschau ausgesetzt hat.
    ///
    /// Gerufen, sobald das Vorschaufenster wieder auf dem Schirm steht. Ohne
    /// Vermerk geschieht nichts, und der Tab zeigt weiter, was er beim
    /// Ausblenden zeigte.
    fn vorschau_nachtragen(&self) {
        let Some(pfad) = self.ivars().vorschau_nachtrag.borrow_mut().take() else {
            return;
        };
        self.vorschau().datei_anzeigen(&pfad);
    }

    /// Zeigt den Inhalt der Zwischenablage im aktiven Vorschau-Tab (C10).
    ///
    /// Blendet das Vorschaufenster ein, falls es ausgeblendet war, und
    /// blendet es **nie** aus: zum Ausblenden bleibt der Befehl aus C7 auf
    /// F3. Zwei Befehle, die beide umschalten, waeren zwei Wahrheiten ueber
    /// den Zustand desselben Bereichs. Der Weg dorthin ist seit dem
    /// Nutzerentscheid vom 260807 derselbe, den auch die Fokusbefehle nehmen:
    /// [`Self::bereich_einblenden`].
    fn zwischenablage_ansehen(&self) -> bool {
        let inhalt = super::zwischenablage::inhalt_lesen();
        self.vorschau().zwischenablage_anzeigen(inhalt);
        // Die Zwischenablage ist die neuere Quelle fuer denselben Tab; ein
        // waehrend der ausgeblendeten Vorschau vermerkter Pfad ist damit
        // ueberholt und wird nicht mehr nachgeholt. Deshalb steht die Zeile
        // vor dem Einblenden: danach faende der Nachtrag ihn noch vor und
        // ueberschriebe den gerade gezeigten Inhalt.
        *self.ivars().vorschau_nachtrag.borrow_mut() = None;
        let _ = self.bereich_einblenden(Bereich::Vorschau);
        true
    }

    /// Fuellt die Leiste und haengt ihren Rueckruf ein (C5).
    ///
    /// Die Lesezeichen kommen aus `bookmarks.toml`, die Geraete vom System.
    /// Eine beschaedigte Lesezeichendatei geht denselben Weg wie eine
    /// beschaedigte Sitzung: Auslieferungszustand, also eine leere Liste, und
    /// eine Meldung in der Statuszeile.
    fn leiste_einrichten(&self, meldungen: &mut Vec<String>) {
        let geladen = match self.unter_der_sperre(|zugang| {
            zugang
                .laden::<Lesezeichenliste>(Datei::Lesezeichen)
                .mit_meldung()
        }) {
            Ok((liste, meldung)) => {
                meldungen.extend(meldung);
                liste
            }
            // Ohne Ablageordner gibt es nichts zu laden und nichts zu sichern.
            // Die Meldung darueber hat `sitzung_laden` schon gestellt; eine
            // zweite waere dieselbe Auskunft ein zweites Mal.
            Err(Sperrhindernis::OhneOrdner) => Lesezeichenliste::default(),
            Err(Sperrhindernis::Gesperrt(fehler)) => {
                meldungen.push(format!(
                    "die Lesezeichen liessen sich nicht laden, die Schreibsperre der Ablage \
                     ist nicht zu nehmen: {fehler}"
                ));
                Lesezeichenliste::default()
            }
        };
        let leiste = self.leiste();
        leiste.quelle().lesezeichen_setzen(&geladen);
        leiste.quelle().orte_setzen(orte());

        // Der Rueckruf haelt den Delegierten **schwach**, sonst schloesse sich
        // der Ring Delegierter → Leiste → Quelle → Rueckruf → Delegierter.
        // Dieselbe Form wie bei den drei Rueckrufen der Dateifenster.
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        leiste.quelle().auswahl_setzen(Box::new(move |auswahl| {
            if let Some(selbst) = schwach.load() {
                selbst.leistenauswahl_ausfuehren(&auswahl);
            }
        }));
    }

    /// Der Nutzer hat in der Leiste einen Eintrag ausgewaehlt (C5, C6).
    ///
    /// **Erst die Gueltigkeit, dann die Sorte.** Der erste Zweig gilt beiden
    /// Sorten und stellt dieselbe Frage: ist das Ziel noch da. Ungueltig heisst
    /// dabei allein, dass Ordner oder Datei fehlen; ob der gemerkte
    /// Zeileninhalt einer Textmarke noch steht, entscheidet sich beim Sprung
    /// und nur dort, und der Grund dafuer steht im Modulkopf von
    /// [`krk_core::ablage::lesezeichen`]. Die Meldung ist eine
    /// **Befehlsantwort**: der Nutzer hat die Auswahl eben selbst bewegt.
    ///
    /// **Eine Ordnermarke setzt den Ordner des aktiven Dateifensters, ohne den
    /// Tab zu wechseln**: [`DateifensterQuelle::ordner_lesen`] liest in den
    /// sichtbaren Tab, denselben Weg, den jede Navigation aus C2 geht. Ein
    /// eigener Lesepfad fuer die Leiste entstuende sonst.
    ///
    /// **Eine Textmarke oeffnet ihre Datei im Editor und springt an die
    /// gemerkte Stelle.** Die Fallunterscheidung ueber das [`Ziel`] ist
    /// vollstaendig und hat keinen Auffangzweig; eine dritte Sorte haelt den Bau
    /// an und erzwingt die Antwort, was ihre Auswahl tut.
    fn leistenauswahl_ausfuehren(&self, auswahl: &crate::leistenmodell::Auswahl) {
        let aktiv = self.ivars().modell.borrow().aktiv();
        if !auswahl.gueltig {
            self.antwort_zeigen(
                aktiv,
                // Kurz genug fuer die Statuszeile: sie ist einzeilig, und ein
                // laengerer Satz endet am rechten Rand des Dateifensters mit
                // drei Punkten. Gemessen am 260805 im laufenden Buendel.
                &format!(
                    "„{}“ fehlt: {} gibt es nicht mehr",
                    auswahl.name,
                    auswahl.pfad().display()
                ),
            );
            return;
        }
        match &auswahl.ziel {
            Ziel::Ordner { ordner } => {
                self.dateifenster(aktiv).quelle().ordner_lesen(ordner, None);
                self.sitzung_vormerken();
            }
            Ziel::Textstelle {
                datei,
                zeile,
                zeileninhalt,
            } => self.textmarke_anspringen(datei, *zeile, zeileninhalt),
        }
    }

    /// Oeffnet die Datei einer Textmarke im Editor und merkt vor, wohin die
    /// Schreibmarke danach gehoert (C6).
    ///
    /// **Kein zweiter Weg und keine zweite Regel.** Geoeffnet wird ueber
    /// [`Editorbereich::datei_oeffnen`] wie bei F4 und wie beim Uebergang aus
    /// der Vorschau, und geprueft damit von `krk_core::text::datei::oeffnen`,
    /// der einen Stelle, die entscheidet, ob der Editor eine Datei annimmt. Alles,
    /// was auf das Oeffnen folgt, erbt dieser Weg von
    /// [`Self::editorausgang_behandeln`], ohne eine Zeile dafuer zu schreiben:
    /// das Hervorholen des ausgeblendeten Editors, den Fokus, den Titel, die
    /// Abweisungsmeldung und die Nachfrage aus C4 beim Wechsel auf eine andere
    /// Datei.
    ///
    /// **Die Marke bleibt gueltig, auch wenn der Editor die Datei abweist.** An
    /// ihr hat sich nichts geaendert; gueltig heisst allein, dass die Datei da
    /// ist, und das ist sie.
    ///
    /// **Gesprungen wird erst beim Ausgang**, und der Grund steht an
    /// [`AnwendungsIvars::vorgemerkte_marke`]: hier ist noch nicht gelesen, und
    /// wohin die Schreibmarke gehoert, entscheidet sich am Text.
    fn textmarke_anspringen(&self, datei: &Path, zeile: u32, zeileninhalt: &str) {
        *self.ivars().vorgemerkte_marke.borrow_mut() = Some((zeile, zeileninhalt.to_owned()));
        if !self.editor_oeffnen_lassen(datei, Oeffnungsherkunft::Befehl) {
            // Ohne Editorbereich kommt kein Ladeausgang, und damit niemand, der
            // die Stelle herausnimmt; sie bliebe liegen und griffe beim naechsten
            // Oeffnen. Vorgemerkt wird trotzdem zuerst, weil das Oeffnen
            // unverzueglich zurueckkehrt und der Ausgang die Stelle schon
            // braucht.
            *self.ivars().vorgemerkte_marke.borrow_mut() = None;
        }
    }

    /// Fuehrt eine Aenderung an den Lesezeichen aus und sichert sie (C5, C6).
    ///
    /// **Ein vollstaendiger Durchgang aus Lesen, Aendern und Schreiben unter der
    /// Schreibsperre** (C3.8 der Runde 7). Bis dahin schrieb diese Stelle die
    /// Liste, die die Leiste seit dem Programmstart hielt, blind ueber die
    /// Datei; ein Lesezeichen, das eine zweite Instanz inzwischen angelegt
    /// hatte, war damit fort. Jetzt wird `bookmarks.toml` unter der Sperre
    /// frisch gelesen, die eine [`Aenderung`] darauf angewandt und das Ergebnis
    /// geschrieben. Laege das Lesen ausserhalb der Sperre, waere die verlorene
    /// Aenderung nur seltener und nicht fort.
    ///
    /// **Die Leiste zeigt danach das Ergebnis und nicht ihre eigene Rechnung.**
    /// Sie bekommt die geschriebene Liste zurueck; was die andere Instanz
    /// beigetragen hat, steht damit sofort in der Leiste, ohne dass diese Runde
    /// dafuer eine Beobachtung des Ablageordners baut.
    ///
    /// **Ohne Ablageordner wird gerechnet und nicht geschrieben.** Der Befehl
    /// wirkt dann in der laufenden Sitzung und ist mit dem Beenden fort; die
    /// Meldung darueber hat der Start gestellt. Es ist derselbe Rechenweg,
    /// [`Lesezeichenliste::anwenden`], und kein zweiter.
    ///
    /// Nach **jeder** Aenderung geschrieben, wie `### Frage 4` des Plans es fuer
    /// diese Datei vorschreibt, und nicht gebuendelt wie die Sitzung: eine
    /// Aenderung an den Lesezeichen ist eine Handlung des Nutzers und keine
    /// Nebenwirkung des Arbeitens, davon gibt es wenige, und jede soll einen
    /// Absturz ueberleben.
    fn lesezeichen_aendern(&self, seite: Fensterseite, aenderung: &Aenderung) {
        let ergebnis = self.unter_der_sperre(|zugang| {
            let (mut liste, meldung) = zugang
                .laden::<Lesezeichenliste>(Datei::Lesezeichen)
                .mit_meldung();
            let ausgang = liste.anwenden(aenderung);
            let geschrieben = match ausgang {
                Ausgang::Geaendert(_) => Some(zugang.sichern(Datei::Lesezeichen, &liste)),
                Ausgang::Unveraendert | Ausgang::Verschwunden => None,
            };
            (liste, ausgang, geschrieben, meldung)
        });

        let (liste, ausgang, geschrieben, meldung) = match ergebnis {
            Ok(alles) => alles,
            Err(Sperrhindernis::OhneOrdner) => {
                let mut liste = self.leiste().quelle().lesezeichenliste();
                let ausgang = liste.anwenden(aenderung);
                (liste, ausgang, None, None)
            }
            Err(Sperrhindernis::Gesperrt(fehler)) => {
                self.antwort_zeigen(
                    seite,
                    &format!(
                        "die Lesezeichen liessen sich nicht aendern, die Schreibsperre der \
                         Ablage ist nicht zu nehmen: {fehler}"
                    ),
                );
                return;
            }
        };

        // Die Leiste zeigt in jedem der drei Ausgaenge die gelesene Liste: auch
        // ein verschwundenes Lesezeichen ist eine Auskunft, die der Nutzer
        // sehen soll.
        let stelle = match ausgang {
            Ausgang::Geaendert(stelle) => Some(stelle),
            Ausgang::Unveraendert | Ausgang::Verschwunden => None,
        };
        self.leiste()
            .quelle()
            .lesezeichen_uebernehmen(&liste, stelle);

        if let Some(meldung) = meldung {
            self.antwort_zeigen(seite, &meldung);
            return;
        }
        if matches!(ausgang, Ausgang::Verschwunden) {
            // **„geaendert oder geloescht" und nicht „geloescht".**
            // `Lesezeichenliste::stelle_von` vergleicht den ganzen Eintrag,
            // Name und Ziel; der Ausgang tritt deshalb auch ein, wenn die
            // andere Instanz das Lesezeichen nur umbenannt oder sein Ziel
            // geaendert hat. Der Satz nannte bis zur Runde 7 allein die
            // Loeschung und schickte den Nutzer damit ein Lesezeichen suchen,
            // das umbenannt in der Leiste steht.
            self.antwort_zeigen(
                seite,
                "dieses Lesezeichen steht nicht mehr so in der Liste; eine andere Instanz \
                 von KRK hat es geaendert oder geloescht",
            );
            return;
        }
        if let Some(Err(fehler)) = geschrieben {
            self.antwort_zeigen(
                seite,
                &format!("die Lesezeichen liessen sich nicht sichern: {fehler}"),
            );
        }
    }

    /// Das ausgewaehlte Lesezeichen, oder nichts.
    ///
    /// Das Ziel der drei Befehle, die ein vorhandenes Lesezeichen aendern.
    /// Steht die Auswahl auf einer Ueberschrift oder einem Geraet, wirken sie
    /// nicht und melden es nicht, wie der Wirkungsbereich es auch nicht tut.
    fn gewaehltes_lesezeichen(&self) -> Option<Lesezeichen> {
        self.leiste().quelle().gewaehltes_lesezeichen()
    }

    /// `cmd+d` legt ein Lesezeichen an: einen Ordner oder eine Textstelle (C5,
    /// C6).
    ///
    /// **Ein Befehl fuer beide Sorten, und der Fokus waehlt.** Steht er im
    /// Editor, merkt der Befehl die Zeile der Schreibmarke; sonst den Ordner
    /// des aktiven Dateifensters. Ein zweiter Anlegebefehl daneben entsteht
    /// nicht: es ist dieselbe Handlung an derselben Liste, und die eine Liste
    /// mit zwei Sorten, die C6 zusagt, haette sonst zwei Tueren. Was der Fokus
    /// hier entschieden hat, fragt von hier bis in `bookmarks.toml` niemand mehr
    /// nach — die Kette nimmt seit S38 das fertige [`Ziel`] entgegen.
    ///
    /// Der Name kommt in beiden Faellen aus demselben Eingabeblatt, das C4 fuer
    /// das Anlegen benutzt. Vorbelegt ist er mit dem, was der Nutzer ohnehin
    /// vergeben haette: dem Namen des Ordners, oder dem Dateinamen mit der
    /// Zeilennummer.
    ///
    /// Liefert `true`, sobald das Blatt steht: der Tastendruck ist dann
    /// verbraucht.
    fn lesezeichen_anlegen(&self) -> bool {
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };
        let seite = self.ivars().modell.borrow().aktiv();
        let Some((ziel, vorschlag)) = self.anlegeziel(seite) else {
            // Der Grund steht in der Statuszeile, und der Tastendruck ist
            // verbraucht: der Befehl war zustaendig und hatte etwas zu melden.
            return true;
        };

        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        namenseingabe::frei_zeigen(
            self.mtm(),
            fenster,
            "Wie soll das Lesezeichen heißen?",
            "Anlegen",
            &vorschlag,
            move |name| {
                if let Some(selbst) = schwach.load() {
                    selbst.lesezeichen_anlegen_ausfuehren(seite, &ziel, &name);
                }
            },
        );
        true
    }

    /// Worauf `cmd+d` gerade zeigt, und wie das Blatt es vorschlaegt (C5, C6).
    ///
    /// **Die eine Stelle, an der die Sorte entschieden wird.** Sie ist die
    /// Frage nach dem Fokus und keine zweite Regel daneben: der Befehl traegt
    /// [`Wirkungsbereich::Ueberall`](krk_core::tasten::Wirkungsbereich) und
    /// erreicht damit auch die Leiste und die Vorschau, und dort ist der
    /// gemeinte Ordner derselbe wie mit dem Fokus im Dateifenster, naemlich der
    /// des aktiven.
    ///
    /// **Eine Marke bezeichnet genau eine Zeile.** Welche das bei mehrzeiliger
    /// Auswahl ist, entscheidet
    /// [`Editorbereich::schreibmarkenzeile`](super::editor::Editorbereich::schreibmarkenzeile)
    /// und nicht diese Zeile.
    ///
    /// `None` heisst: der Fokus steht im Editor, und der haelt keine Datei. Es
    /// gibt dann keine Stelle, die eine Marke bezeichnen koennte; gemeldet wird
    /// derselbe Satz, den [`Self::editorblatt_moeglich`] fuer diesen Fall
    /// fuehrt. Auf den Ordner des aktiven Dateifensters auszuweichen waere
    /// falsch: der Nutzer bekaeme dann stillschweigend ein anderes Lesezeichen,
    /// als er verlangt hat.
    fn anlegeziel(&self, seite: Fensterseite) -> Option<(Ziel, String)> {
        if self.fokus() == Fokus::Editor {
            let editor = self.ivars().editor.get()?;
            let (Some(datei), Some((zeile, zeileninhalt))) =
                (editor.pfad(), editor.schreibmarkenzeile())
            else {
                self.antwort_zeigen(seite, "der Editor hält keine Datei");
                return None;
            };
            let vorschlag = match datei.file_name() {
                Some(name) => format!("{}:{zeile}", name.to_string_lossy()),
                None => format!("{}:{zeile}", datei.display()),
            };
            return Some((
                Ziel::Textstelle {
                    datei,
                    zeile,
                    zeileninhalt,
                },
                vorschlag,
            ));
        }

        let ordner = self.dateifenster(seite).quelle().angezeigter_ordner();
        let vorschlag = ordner
            .file_name()
            .map(|teil| teil.to_string_lossy().into_owned())
            .unwrap_or_else(|| ordner.display().to_string());
        Some((Ziel::Ordner { ordner }, vorschlag))
    }

    /// Oeffnet den angezeigten Ordner in der eingestellten Anwendung (C11).
    ///
    /// Der Ordner ist der des **sichtbaren Tabs im aktiven Dateifenster**, und
    /// er kommt aus der Tabliste, wie bei jedem Befehl, der auf den angezeigten
    /// Ordner wirkt. Dass der Fokus dafuer im Dateifenster stehen muss, hat die
    /// eine Abfrage in [`Self::kommando_ausfuehren`] schon entschieden: das
    /// Kommando traegt `Wirkungsbereich::Dateifenster`. Steht der Fokus in der
    /// Leiste, ist dieser Rumpf nie erreicht, und **gemeldet wird dann
    /// nichts** — der Wirkungsbereich ist stumm.
    ///
    /// Zwei Fehler kann der Nutzer beheben, und beide stellt der Befehl vor dem
    /// Aufruf fest: der Ordner ist nicht mehr da, oder zu der eingestellten
    /// Buendelkennung ist keine Anwendung installiert. Beide gehen als
    /// Befehlsantwort in die Statuszeile, den ersten der sechs Raenge; ein
    /// eigenes Blatt entsteht nicht. Der dritte Fehler aus C11, die beschaedigte
    /// `settings.toml`, hat sich beim Start gemeldet, denn dort faellt er an.
    ///
    /// Liefert immer `true`: der Befehl war zustaendig, auch wenn er nur etwas
    /// zu melden hatte. Ein `false` liesse den Nachzug der Aufteilung und die
    /// vorgemerkte Sitzung ausfallen; den Tastendruck gaebe es nicht her, denn
    /// [`Self::kommando_ausfuehren`] liefert seit der Runde 7 immer `true`.
    fn terminal_oeffnen(&self) -> bool {
        let seite = self.ivars().modell.borrow().aktiv();
        let ordner = self.dateifenster(seite).quelle().angezeigter_ordner();
        if let Some(meldung) = operationen::terminalordner_fehlt(&ordner) {
            self.antwort_zeigen(seite, &meldung);
            return true;
        }
        let kennung = self.ivars().einstellungen.borrow().terminal.clone();
        if !terminal::ordner_oeffnen(&kennung, &ordner) {
            self.antwort_zeigen(seite, &operationen::kein_terminal(&kennung));
        }
        true
    }

    /// Startet eine weitere Instanz von KRK (C3 der Runde 7).
    ///
    /// Liefert immer `true`: der Befehl war zustaendig, auch wenn er nur etwas
    /// zu melden hatte — dieselbe Ueberlegung wie bei
    /// [`Self::terminal_oeffnen`] darueber, und dort steht auch, was ein
    /// `false` hier ausfallen liesse und was nicht.
    ///
    /// Der eine Fall, den der Nutzer sieht, ist der Entwicklungslauf ohne
    /// Buendel; er geht als Befehlsantwort in die Statuszeile, den ersten der
    /// sechs Raenge.
    fn weitere_instanz_starten(&self) -> bool {
        if let Some(meldung) = weitereinstanz::starten() {
            let seite = self.ivars().modell.borrow().aktiv();
            self.antwort_zeigen(seite, meldung);
        }
        true
    }

    /// Legt das Lesezeichen an und sichert die Datei (C5, C6).
    ///
    /// Nimmt das fertige [`Ziel`] entgegen und fragt nicht nach der Sorte: die
    /// Sorte hat der Fokus in [`Self::lesezeichen_anlegen`] entschieden, und
    /// von hier bis in `bookmarks.toml` gibt es fuer beide eine Tuer.
    fn lesezeichen_anlegen_ausfuehren(&self, seite: Fensterseite, ziel: &Ziel, name: &str) {
        if let Err(hinweis) = lesezeichen::name_pruefen(name) {
            self.antwort_zeigen(seite, hinweis.grund());
            return;
        }
        self.lesezeichen_aendern(
            seite,
            &Aenderung::Anlegen {
                name: name.to_owned(),
                ziel: ziel.clone(),
            },
        );
        self.antwort_zeigen(seite, &format!("Lesezeichen „{}“ angelegt", name.trim()));
    }

    /// Benennt das ausgewaehlte Lesezeichen um (C5).
    ///
    /// Ueber dasselbe Blatt wie das Anlegen, vorbelegt mit dem alten Namen.
    /// Steht die Auswahl nicht auf einem Lesezeichen, geschieht nichts und wird
    /// nichts gemeldet: dieselbe Antwort, die der Wirkungsbereich gibt.
    fn lesezeichen_umbenennen(&self) -> bool {
        let (Some(fenster), Some(gewaehlt)) =
            (self.ivars().fenster.get(), self.gewaehltes_lesezeichen())
        else {
            return false;
        };
        let alt = gewaehlt.name;
        let seite = self.ivars().modell.borrow().aktiv();

        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        namenseingabe::frei_zeigen(
            self.mtm(),
            fenster,
            "Wie soll das Lesezeichen heißen?",
            "Umbenennen",
            &alt,
            move |name| {
                if let Some(selbst) = schwach.load() {
                    selbst.lesezeichen_umbenennen_ausfuehren(seite, &name);
                }
            },
        );
        true
    }

    /// Schreibt den neuen Namen und sichert die Datei (C5).
    fn lesezeichen_umbenennen_ausfuehren(&self, seite: Fensterseite, name: &str) {
        if let Err(hinweis) = lesezeichen::name_pruefen(name) {
            self.antwort_zeigen(seite, hinweis.grund());
            return;
        }
        let Some(welches) = self.gewaehltes_lesezeichen() else {
            return;
        };
        self.lesezeichen_aendern(
            seite,
            &Aenderung::Umbenennen {
                welches,
                name: name.to_owned(),
            },
        );
    }

    /// Loescht das ausgewaehlte Lesezeichen und sichert die Datei (C5).
    fn lesezeichen_loeschen(&self) -> bool {
        let Some(welches) = self.gewaehltes_lesezeichen() else {
            return false;
        };
        let seite = self.ivars().modell.borrow().aktiv();
        self.lesezeichen_aendern(seite, &Aenderung::Loeschen { welches });
        true
    }

    /// Schiebt das ausgewaehlte Lesezeichen einen Platz weiter (C5).
    fn lesezeichen_verschieben(&self, richtung: Verschiebung) -> bool {
        let Some(welches) = self.gewaehltes_lesezeichen() else {
            return false;
        };
        let seite = self.ivars().modell.borrow().aktiv();
        self.lesezeichen_aendern(seite, &Aenderung::Verschieben { welches, richtung });
        true
    }

    /// Fuehrt einen Fokusbefehl aus: erst den Bereich hervorholen, dann den
    /// Fokus setzen (C2, C5, C6).
    ///
    /// Der Weg aller drei Fokusbefehle, und sie gehen ihn ohne Sonderfall.
    /// In welchem Bereich ein Fokuswert wohnt, sagt
    /// [`fokus::bereich_mit_fokus`](crate::kommandos::fokus::bereich_mit_fokus)
    /// und sonst nichts; dort steht auch, warum ein Fokusbefehl seinen Bereich
    /// seit dem Nutzerentscheid vom 260807 hervorholt, statt ihn stumm
    /// abzuweisen.
    ///
    /// **Das aktive Dateifenster kommt hier ohne Ausnahme mit durch.**
    /// `bereich_mit_fokus` nennt es fuer [`Fokus::Dateifenster`], und
    /// [`Fenstermodell::einblenden`](crate::fenstermodell::Fenstermodell::einblenden)
    /// liefert dafuer `false`, weil es nie ausgeblendet ist: eines der beiden
    /// Dateifenster bleibt stehen, und wird das aktive ausgeblendet, wandert
    /// die Aktivitaet auf das andere. Bis zum 260809 stand hier `holt_hervor`,
    /// das genau deshalb `None` lieferte; die Antwort ist dieselbe, und die
    /// Zuordnung steht jetzt einmal statt zweimal.
    ///
    /// "Ausgefuehrt" heisst hier: **irgendetwas** ist geschehen. Der Befehl auf
    /// eine ausgeblendete Leiste blendet sie ein, auch wenn der Fokus danach
    /// aus einem anderen Grund nicht umzieht; ohne das oder-Zeichen liesse er
    /// die Aufteilung ungezeichnet stehen.
    ///
    /// **Hervorholen und Fokussetzen sind zwei Handlungen, und die Flaeche
    /// steht zuerst auf dem Schirm.** [`Self::bereich_einblenden`] kehrt seit
    /// dem 260823 erst zurueck, wenn [`Self::sichtbarkeit_aendern`] die neue
    /// Sichtbarkeit an die Ansichten geschrieben hat; erst danach ruft
    /// [`Self::fokus_setzen`] `makeFirstResponder:`. Die Reihenfolge ist
    /// tragend und nicht kosmetisch: eine Ansicht, die AppKit noch als
    /// ausgeblendet fuehrt, nimmt den Ersthelferrang nicht verlaesslich an, und
    /// der Fokus landet dann irgendwo statt im hervorgeholten Bereich.
    fn fokus_holen(&self, ziel: Fokus) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();
        let eingeblendet = match fokus::bereich_mit_fokus(ziel, aktiv) {
            Some(bereich) => self.bereich_einblenden(bereich),
            None => false,
        };
        let gesetzt = self.fokus_setzen(ziel);
        eingeblendet || gesetzt
    }

    /// Der vierte Fokusbefehl: in den eingebauten Editor (C1 der Editor-Runde).
    ///
    /// Gebaut wie die drei bestehenden, ueber [`Self::fokus_holen`], mit der
    /// einen Zusatzbedingung aus dem sechsten Abnahmekriterium von C1:
    /// **haelt der Editor keine Datei und ist er ausgeblendet, tut der Befehl
    /// nichts.** Ein leerer Editor, den niemand verlangt hat, naehme den
    /// Dateifenstern Platz fuer nichts und verdraengte dabei nach S18 noch die
    /// Vorschau; dieselbe Begruendung traegt `Sichtbarkeit::default` fuer den
    /// Auslieferungszustand.
    ///
    /// **Die Bedingung steht hier und nicht in
    /// [`crate::kommandos::fokus::holt_hervor`].** Jene ist eine reine
    /// Zuordnung von einem Fokusziel auf einen Bereich und kennt keinen
    /// Zustand; ihr Doc-Kommentar sagt es ausdruecklich, und ein
    /// Zustandsvorbehalt darin traefe die drei uebrigen Fokusbefehle mit.
    ///
    /// Steht der Editor schon auf dem Schirm, geht der Fokus hinein, auch ohne
    /// Datei: der Nutzer sieht die Flaeche und soll erfahren, wo seine Tasten
    /// ankommen. Der Weg zurueck braucht keinen zweiten Befehl, das siebte
    /// Abnahmekriterium von C1 sagt es — `fokus_dateifenster` traegt
    /// [`Wirkungsbereich::Ueberall`](krk_core::tasten::Wirkungsbereich) und
    /// wirkt im Editor.
    fn fokus_editor_holen(&self) -> bool {
        if !self.editor_ist_ansprechbar() {
            return false;
        }
        self.fokus_holen(Fokus::Editor)
    }

    /// Ob ein Befehl den Editor ueberhaupt ansprechen darf: **er steht, oder
    /// er haelt eine Datei.**
    ///
    /// **Die eine Fassung dieser Bedingung**, gelesen von
    /// [`Self::fokus_editor_holen`] (sechstes Abnahmekriterium von C1 der
    /// Editor-Runde) und von [`Self::editor_umschalten`] (C6 der
    /// Bereichsleisten-Runde). Beide beantworten dieselbe Frage, und bis zum
    /// 260812 stand sie zweimal wortgleich da
    /// (`issues/260812-0727_*_editor-umschalten-schreibt-die-erreichbarkeitspruefung-von-fokus-editor-holen-wortgleich-ab.md`).
    ///
    /// **Sie steht hier und nicht im Fenstermodell**, weil das Fenstermodell
    /// von Dateien nichts weiss; die Sichtbarkeit kommt aus ihm, das Halten
    /// einer Datei aus dem Editorbereich. Ein leerer Editor, den niemand
    /// verlangt hat, naehme den Dateifenstern Platz fuer nichts und
    /// verdraengte dabei die Vorschau — dieselbe Begruendung traegt
    /// `Sichtbarkeit::default` fuer den Auslieferungszustand.
    fn editor_ist_ansprechbar(&self) -> bool {
        let sichtbar = self.ivars().modell.borrow().sichtbar(Bereich::Editor);
        sichtbar
            || self
                .ivars()
                .editor
                .get()
                .is_some_and(|editor| editor.haelt_datei())
    }

    /// Die Ansicht, an der ein Fokuswert haengt.
    ///
    /// **Die eine Zuordnung von einem Fokuswert auf sein Objekt**, und sie
    /// bedient beide Richtungen: [`Self::fokus_setzen`] macht die genannte
    /// Ansicht zum Ersthelfer des Fensters, [`Anwendungsdelegierter::fokus`]
    /// erkennt den Ersthelfer daran wieder. Zwei getrennte Aufzaehlungen
    /// waeren zwei Wahrheiten darueber, woran ein Bereich zu erkennen ist, und
    /// genau die eine, die im Lesen fehlte, hat den Editor bis zum 260809
    /// stumm zum Dateifenster gemacht: `fokus` fragte die Leiste und die
    /// Vorschau und fiel sonst auf [`Fokus::Dateifenster`] zurueck, worauf
    /// `delete` mit der Schreibmarke im Text die ausgewaehlte Datei in den
    /// Papierkorb warf
    /// (`issues/260809-1640_*_der-fokus-kennt-den-editor-nicht-obwohl-der-abgriff-ihn-seit-s4-durchlaesst.md`).
    ///
    /// **Die Fallunterscheidung ist erschoepfend und ohne Auffangzweig.** Ein
    /// sechster Fokuswert haelt hier den Bau an, wie bei
    /// [`Kommando::wirkungsbereich`] und
    /// [`crate::kommandos::fokus::holt_hervor`]; genau diese Erzwingung fehlte
    /// der `if`-Kette, die vorher las.
    ///
    /// Gefragt ist die **Naemlichkeit** und nicht die Art, aus dem Grund, der
    /// an [`Self::ist_eigene_textflaeche`] steht: die Textflaeche des Editors ist
    /// dieselbe Art wie der Feldeditor eines Textfeldes.
    ///
    /// `None` heisst: dieser Wert haengt an keiner Ansicht. Fuer
    /// [`Fokus::Anderswo`] ist das dauerhaft so — der Wert ist ein Befund und
    /// kein Ziel, wie es an `holt_hervor` steht. Fuer die drei Randbereiche
    /// gilt es, solange sie nicht gebaut sind; deshalb `get` und nicht
    /// `expect`, denn die Reihenfolge im Aufbau der Oberflaeche ist keine
    /// Zusage dieser Funktion.
    ///
    /// **Die Antwort ist eine `NSView` und kein `NSResponder`**, seit das
    /// Teilen aus C1 der Runde 6 sie als **Anker** braucht und nicht nur als
    /// Ersthelfer: ein Freigabedialog haengt sich an eine Flaeche und an deren
    /// Rechteck. Alle vier Zweige liefern ohnehin eine Ansicht, und
    /// [`Self::fokus_setzen`] kommt mit ihr aus, weil eine `NSView` ein
    /// `NSResponder` ist. Der engere Typ ist der Preis dafuer, dass es bei
    /// **einer** Zuordnung bleibt; eine zweite daneben waeren wieder zwei
    /// Wahrheiten darueber, welche Flaeche zu einem Fokuswert gehoert.
    fn fokusansicht(&self, ziel: Fokus) -> Option<&NSView> {
        match ziel {
            Fokus::Leiste => Some(self.ivars().leiste.get()?.quelle().liste()),
            Fokus::Vorschau => Some(self.ivars().vorschau.get()?.fokusansicht()),
            Fokus::Editor => Some(self.ivars().editor.get()?.textflaeche()),
            // Das **aktive** Dateifenster: es gibt zwei Listen und einen
            // Fokuswert, und welche der beiden gemeint ist, sagt das
            // Fenstermodell.
            Fokus::Dateifenster => {
                let aktiv = self.ivars().modell.borrow().aktiv();
                Some(self.ivars().dateifenster.get()?[aktiv.index()].liste())
            }
            Fokus::Anderswo => None,
        }
    }

    /// Setzt den Eingabefokus in einen der vier Bereiche (C5, C6, C1 der
    /// Editor-Runde).
    ///
    /// Die eine Stelle, die den Fokus **setzt**, so wie
    /// [`Anwendungsdelegierter::fokus`] die eine ist, die ihn liest. Welche
    /// Ansicht der Ersthelfer wird, sagt [`Self::fokusansicht`] und sonst
    /// nichts.
    ///
    /// In einen ausgeblendeten Randbereich geht der Fokus nicht: dort saehe
    /// der Nutzer weder seine Auswahl noch, dass seine Tasten irgendwo
    /// ankommen. Die Sperre bleibt stehen, obwohl [`Self::fokus_holen`] den
    /// Bereich vorher hervorholt — sie gilt fuer jeden Aufrufer und nicht nur
    /// fuer den einen, der vorbaut. **Welcher Bereich zu einem Fokusziel
    /// gehoert, sagt [`crate::kommandos::fokus::bereich_mit_fokus`]**,
    /// dieselbe Zuordnung, die die Fokusbefehle schon zum Hervorholen lesen und
    /// die Anzeige aus C9 zum Einfaerben; drei handgeschriebene
    /// Sichtbarkeitsabfragen daneben waeren eine zweite Wahrheit darueber, in
    /// welchem Bereich ein Fokuswert wohnt. Das aktive Dateifenster ist nie
    /// ausgeblendet und faellt deshalb hier nie durch.
    ///
    /// Drei Aufrufer: die Fokusbefehle ueber [`Self::fokus_holen`], das
    /// Ausblenden eines Randbereichs, und der Aufbau der Oberflaeche mit
    /// [`crate::kommandos::fokus::BEIM_START`].
    ///
    /// **Die Anzeige zieht diese Funktion nicht selbst nach.** Sie ruft
    /// `makeFirstResponder`, und die Ueberschreibung in
    /// [`Hauptfenster`](super::fenster::Hauptfenster) meldet jeden erfolgreichen
    /// Wechsel an [`Self::fokusanzeige_nachziehen`]. Es gibt einen
    /// Ausloesepunkt und nicht zwei; ein Nachzug an dieser Stelle waere der
    /// zweite und liesse den Mausklick weiter aussen vor.
    fn fokus_setzen(&self, ziel: Fokus) -> bool {
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };
        let ausgeblendet = {
            let modell = self.ivars().modell.borrow();
            fokus::bereich_mit_fokus(ziel, modell.aktiv())
                .is_some_and(|bereich| !modell.sichtbar(bereich))
        };
        if ausgeblendet {
            return false;
        }
        match self.fokusansicht(ziel) {
            Some(ansicht) => fenster.makeFirstResponder(Some(ansicht)),
            // Kein Ziel, also kein Umzug: [`Fokus::Anderswo`] ist ein Befund,
            // und ein noch nicht gebauter Bereich hat keine Ansicht. Beides
            // scheitert still, wie jeder Fokusbefehl auf einen Bereich, der
            // nicht da ist.
            None => false,
        }
    }

    /// Zaehlt die Geraete und Standardorte neu auf (C5).
    ///
    /// Gerufen nach jedem Ein- und Aushaengen. Die Leiste prueft dabei zugleich
    /// die Gueltigkeit der Lesezeichen nach; ein eingehaengter Datentraeger
    /// macht ein Lesezeichen darauf wieder erreichbar.
    fn leiste_geraete_nachziehen(&self) {
        if self.ivars().leiste.get().is_none() {
            return;
        }
        self.leiste().quelle().orte_setzen(orte());
    }

    /// Richtet den einen Eintrittspunkt fuer Tastendruecke ein.
    ///
    /// Die Belegung ist die, aus der [`starten`] schon das Hauptmenue gebaut
    /// hat; sie wird hier nicht ein zweites Mal geladen.
    fn tastenabgriff_einrichten(&self, meldungen: &mut Vec<String>) {
        meldungen.extend(self.ivars().belegungsmeldung.clone());
        match self.abgriff_aufsetzen() {
            Some(abgriff) => {
                *self.ivars().tastenabgriff.borrow_mut() = Some(abgriff);
            }
            None => self.ohne_tastenabgriff_beenden(),
        }
    }

    /// Zeigt den Hinweis und beendet KRK, wenn kein Tastenabgriff steht.
    ///
    /// Ohne Abgriff bewegt keine Taste die Auswahl. Das still hinzunehmen
    /// hiesse, eine Anwendung weiterlaufen zu lassen, deren erste Maxime die
    /// Tastatursteuerung ist und die keine hat; alles, was danach auf dem
    /// Schirm steht, waere eine Taeuschung. Der Nutzer hat am 260804-0830
    /// Moeglichkeit 1 aus `decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md`
    /// gewaehlt und diesen einen Fehler ausdruecklich vom Weg ueber die
    /// Statuszeile ausgenommen: die Zeile am Fuss eines Fensters waere die
    /// falsche Lautstaerke, und der Nutzer arbeitete mit einer halben Anwendung
    /// weiter.
    ///
    /// **Die Standardfehlerausgabe scheidet aus.** Ein ueber den Finder oder
    /// ueber `open` gestartetes Buendel hat keine; LaunchServices haengt sie ins
    /// Leere, gemessen am 260803-1309. Bis Schritt 6b stand hier genau das, und
    /// die Meldung erreichte in der einzigen Betriebsart, die die Abnahme
    /// zulaesst, niemanden.
    ///
    /// Beendet wird ueber [`Anwendungsdelegierter::beenden`], also `terminate:`
    /// und nicht `exit`: derselbe Weg wie beim Menueeintrag, damit
    /// `applicationWillTerminate:` den letzten Sitzungsstand noch schreibt.
    /// Beim Start ist das folgenlos, beim Nachziehen nach einer Umbelegung
    /// nicht — dort hat der Nutzer gearbeitet, und seine Tabs sollen den
    /// Abbruch ueberleben.
    ///
    /// # `terminate:` kehrt seit S28 zurueck, und zwar in genau zwei Faellen
    ///
    /// Bis dahin gab es kein `applicationShouldTerminate:`, und der Aufruf kam
    /// nie zurueck. Seither gibt es eines, und es beantwortet die Nachfrage aus
    /// C4: haelt der Editor ungesicherten Stand, zeigt es ein Blatt und
    /// antwortet `TerminateLater`, also kehrt `terminate:` zurueck und KRK
    /// laeuft weiter, bis der Nutzer geantwortet hat; antwortet er mit
    /// "abbrechen" oder scheitert das Sichern, laeuft KRK dauerhaft weiter.
    ///
    /// **Die drei Aufrufer rechnen weiterhin nicht mit einer Rueckkehr, und sie
    /// muessen es nicht.** Sie tun danach schlicht nichts mehr, und das bleibt
    /// richtig: kehrt `terminate:` zurueck, hat entweder der Nutzer das Beenden
    /// angehalten — dann soll nichts weiter geschehen — oder das Blatt steht
    /// noch, und die Antwort kommt aus seinem Rueckruf.
    ///
    /// **Dieser eine Aufrufer geht an der Nachfrage vorbei**, ueber
    /// [`AnwendungsIvars::beenden_ohne_nachfrage`]. Hier ist der Tastenabgriff
    /// kaputt und ein anwendungsmodaler Hinweis steht bereits; ein Blatt mit
    /// Rueckfrage waere weder bedienbar noch sinnvoll, und KRK bliebe mit einer
    /// unbeantwortbaren Frage stehen. Der Preis ist benannt: ein ungesicherter
    /// Stand faellt in diesem Fall ohne Nachfrage.
    fn ohne_tastenabgriff_beenden(&self) {
        self.ivars().beenden_ohne_nachfrage.set(true);
        hinweis::zeigen(
            self.mtm(),
            "KRK kann keine Tastendrücke lesen",
            "Der Tastenabgriff ließ sich nicht einrichten. Ohne ihn bewegt keine \
             Taste die Auswahl, und kein Tastenkürzel wirkt. KRK wird beendet, \
             statt mit einem Fenster ohne Tastatursteuerung weiterzulaufen.",
        );
        self.beenden();
    }

    /// Baut einen Abgriff ueber der Belegung, die gerade gilt.
    ///
    /// Der Faenger ist die Aufnahme der Belegungsansicht aus C3; solange keine
    /// steht oder keine aufnimmt, liefert er `false` und aendert nichts.
    ///
    /// **Zwei Abschluesse und nicht mehr drei.** Bis zur Runde 7 ging die
    /// Naemlichkeitsfrage des Fokusvorbehalts als dritter Abschluss in den
    /// Abgriff; sie steht jetzt in [`Self::lage`], und der Abgriff fragt gar
    /// nicht mehr nach dem Ersthelfer. Damit faellt hier eine der drei
    /// schwachen Referenzen weg, und `appkit::ereignisse` bekommt den Editor
    /// nicht mehr hereingereicht.
    fn abgriff_aufsetzen(&self) -> Option<Tastenabgriff> {
        let belegung = self.ivars().belegung.borrow().clone();
        let fuer_faenger = objc2::rc::Weak::from_retained(&self.retain());
        let fuer_senke = objc2::rc::Weak::from_retained(&self.retain());
        Tastenabgriff::einrichten(
            belegung,
            self.ivars().tasten_protokoll,
            move |druck, zeichen| match fuer_faenger.load() {
                Some(selbst) => selbst.tastendruck_fangen(druck, zeichen),
                None => false,
            },
            move |eingabe| match fuer_senke.load() {
                Some(selbst) => selbst.eingabe_ausfuehren(eingabe),
                None => false,
            },
        )
    }

    /// Ob dieser Ersthelfer eine der beiden **eigenen** Textflaechen von KRK
    /// ist.
    ///
    /// Es sind genau zwei, und sie stehen hier einzeln: die Textflaeche des
    /// Editors ([`Editorbereich::textflaeche`]) und die Textanzeige der
    /// Vorschau ([`Vorschaufenster::textflaeche`]). Beide sind Bereiche der
    /// Fensterzeile, beide **wollen** KRKs Tastenbefehle mit dem Fokus in sich
    /// selbst, und beide sind selbst eine `NSTextView` und fielen ohne diese
    /// Frage unter den Fokusvorbehalt.
    ///
    /// **Die Flaeche eines Blattes wird hier ausdruecklich nicht genannt, und
    /// das ist keine Luecke.** Fuer sie ist das Gegenteil erwuenscht: solange
    /// ihr Ersthelfer AppKit gehoert, bleibt `Kommando::Abbrechen` unzulaessig,
    /// der Tastendruck laeuft unveraendert weiter, und `Esc` schliesst den
    /// Notizzettel. Eine Anmeldung kehrte beides um. Die Kette im Einzelnen
    /// steht im Modulkopf von [`blaetter::zettel`](super::blaetter::zettel).
    /// Wer die Warnung in `CLAUDE.md` ohne diese Fallunterscheidung liest,
    /// meldet die falsche Flaeche an.
    ///
    /// Die eine Ausnahme vom Fokusvorbehalt, also von Bestandteil (2) der
    /// Zulaessigkeitsregel. Gereicht wird sie als Abschluss an
    /// [`ereignisse::ersthelfer_gehoert_appkit`], und der eine Aufruf dazu steht
    /// in [`Self::lage`]; bis zur Runde 7 ging sie in den Ereignisabgriff.
    /// Gefragt ist
    /// die **Naemlichkeit** und nicht die Art: die Textflaeche des Editors ist
    /// eine `NSTextView` wie der Feldeditor eines Textfeldes auch, und eine
    /// Frage nach der Art kann die beiden nicht trennen. Der Vergleich laeuft
    /// deshalb ueber `isEqual`, in derselben Form wie in
    /// [`Anwendungsdelegierter::fokus`], das die Liste der Leiste und die
    /// Inhaltsflaeche der Vorschau seit der Runde 1 genauso erkennt.
    ///
    /// **Die Menge der eigenen Flaechen entsteht hier und nicht im
    /// Ereignisabgriff.** Zwei `isEqual`-Vergleiche in einer Funktion, ein
    /// Abschluss, ein Parameter: [`ereignisse`] kennt weder den Editor noch die
    /// Vorschau und soll beide nicht kennenlernen; es kennt allein die Frage,
    /// die hier beantwortet wird. Eine dritte eigene Flaeche kaeme als dritter
    /// Vergleich in diesen Rumpf.
    ///
    /// **Solange ein Bereich nicht gebaut ist, gibt es keine Textflaeche, mit
    /// der zu vergleichen waere**, und dieser Vergleich antwortet `false`: der
    /// Vorbehalt wirkt fuer ihn wie vor der Runde, die ihn angemeldet hat. Der
    /// Abgriff steht seit `oberflaeche_aufbauen` und die beiden Bereiche auch,
    /// aber die Reihenfolge ist keine Zusage dieser Funktion; deshalb `get` und
    /// nicht `expect`, wie in [`Anwendungsdelegierter::fokus`] fuer Leiste und
    /// Vorschau.
    ///
    /// [`Editorbereich::textflaeche`]: super::editor::Editorbereich::textflaeche
    /// [`Vorschaufenster::textflaeche`]: super::vorschau::Vorschaufenster::textflaeche
    fn ist_eigene_textflaeche(&self, ersthelfer: &NSResponder) -> bool {
        let editorflaeche = self
            .ivars()
            .editor
            .get()
            .is_some_and(|editor| ersthelfer.isEqual(Some(editor.textflaeche())));
        let vorschauflaeche = self
            .ivars()
            .vorschau
            .get()
            .is_some_and(|vorschau| ersthelfer.isEqual(Some(vorschau.textflaeche())));

        editorflaeche || vorschauflaeche
    }

    /// Richtet den Abgriff nach einer Umbelegung neu ein (C3).
    ///
    /// Erst abmelden, dann aufsetzen: zwei Abgriffe nebeneinander saehen jeden
    /// Tastendruck doppelt.
    ///
    /// **Der Fehlschlag geht denselben Weg wie beim Start.** Die Lage ist
    /// dieselbe: KRK steht ohne Tastatursteuerung da, nur diesmal mitten in der
    /// Arbeit. Ein zweiter Weg fuer denselben Fehler waere eine zweite Wahrheit
    /// darueber, was KRK ohne Abgriff tut, und der eine der beiden, der auf der
    /// Standardfehlerausgabe endete, waere im Buendel wieder still.
    fn tastenabgriff_nachziehen(&self) {
        *self.ivars().tastenabgriff.borrow_mut() = None;
        match self.abgriff_aufsetzen() {
            Some(abgriff) => {
                *self.ivars().tastenabgriff.borrow_mut() = Some(abgriff);
            }
            None => self.ohne_tastenabgriff_beenden(),
        }
    }

    /// Der Faenger des Ereignisabgriffs, zwei Stationen hintereinander.
    ///
    /// **Erste Station: die Aufnahme** (C3). Nimmt die Belegungsansicht gerade
    /// eine Kombination auf, gehoert ihr dieser Tastendruck, und zwar jeder,
    /// auch `esc` und jedes Zeichen.
    ///
    /// **Zweite Station: die Suche** (C1 der Runde 7). Steht die
    /// Belegungsansicht, ohne aufzunehmen, bekommt sie das Suchzeichen, die
    /// Eingabetaste und die Ruecktaste.
    ///
    /// **Die Reihenfolge der zwei Stationen ist der Vorrang aus C1.15 und keine
    /// dritte Regel.** Waehrend einer Aufnahme kommt die zweite Station nicht
    /// zum Zug, und deshalb landet ein Suchzeichen dann in der Zuweisung und
    /// nicht im Suchtext, und ein nacktes `esc` bricht die Aufnahme ab, statt
    /// die Ansicht zu verlassen.
    ///
    /// **Die zweite Station fragt zuerst, ob die Belegungsansicht ueberhaupt
    /// steht.** Ohne diese Frage liefe jedes getippte Zeichen der ganzen
    /// Anwendung in ihren Suchtext. Nach dem Ersthelfer fragt sie dagegen
    /// **nicht**, und das ist kein Vergessen: solange das Blatt steht, haelt
    /// seine Tabelle den Ersthelferrang, und ein Textfeld gibt es darin nicht.
    /// Die [`Lage`] entsteht erst hinter dem Nachschlag, in der Senke.
    ///
    /// `esc`, die Pfeiltasten und jede Kombination mit Befehls-, Steuerungs-
    /// oder Wahltaste fallen durch beide Stationen. Fuer `esc` und die Pfeile
    /// besorgt das die Aufnahmeregel der Suche, die Steuerzeichen und den
    /// privaten Bereich U+F700 bis U+F8FF abweist; eine eigene Ausnahme fuer
    /// sie gibt es nicht.
    ///
    /// `zeichen` ist das **getippte** Zeichen aus dem Ereignis und nicht
    /// [`Tastendruck::zeichen`]; der Doc-Kommentar von
    /// [`Tastenabgriff::einrichten`] sagt, warum es zwei sind.
    fn tastendruck_fangen(&self, druck: Tastendruck, zeichen: Option<char>) -> bool {
        let (quelle, nimmt_auf) = {
            let ansicht = self.ivars().belegungsansicht.borrow();
            match ansicht.as_ref() {
                Some(quelle) => (quelle.clone(), quelle.nimmt_auf()),
                None => return false,
            }
        };

        match faengerstation(nimmt_auf, druck, zeichen) {
            Faengerstation::Aufnahme => {
                quelle.tastendruck_aufnehmen(druck);
                true
            }
            Faengerstation::NaechsterTreffer => {
                quelle.zum_naechsten_treffer();
                true
            }
            Faengerstation::ZeichenWeg => {
                quelle.suchzeichen_wegnehmen();
                true
            }
            // Die Aufnahmeregel fuer das Zeichen steht in der Suchlage und
            // nicht hier; verbraucht wird das Ereignis nur, wenn sie es
            // genommen hat.
            Faengerstation::Suchzeichen(zeichen) => quelle.suchzeichen_aufnehmen(zeichen),
            Faengerstation::Keine => false,
        }
    }

    // ------------------------------------------------------------------
    // Dateisystem und Datentraeger (C9)
    // ------------------------------------------------------------------

    /// Setzt die Beobachtung der beobachteten Ordner neu auf (C9, C4).
    ///
    /// Gerufen nach jeder Navigation, nach jedem Ein- oder Ausblenden des
    /// zweiten Dateifensters und seit der Editor-Runde nach jedem Wechsel der
    /// Datei, die der Editor haelt — also beim Oeffnen und beim Schliessen. Der
    /// alte Strom faellt dabei; ein `FSEventStream` aendert seine Pfadliste nach
    /// dem Anlegen nicht mehr, und einen zweiten Strom danebenzustellen hiesse,
    /// denselben Ordner doppelt zu beobachten.
    ///
    /// **Der Editor bekommt keinen eigenen Strom, sondern einen Platz in
    /// diesem.** Die Gueltigkeitsmarke der Lesezeichen hat sich an derselben
    /// Frage anders entschieden, und der Vermerk dazu steht in
    /// [`Self::vorgang_beenden`] („Warum hier und nicht in der
    /// Dateisystembeobachtung"): dort gab es einen billigeren Anlass, naemlich
    /// die eigene abgeschlossene Dateioperation, und der deckte den gemeldeten
    /// Fall ab. Fuer C4 gibt es keinen: die fremde Aenderung, um die es geht,
    /// hat in KRK keinen Anlass, und ein zweiter Strom beobachtete den Ordner
    /// doppelt, sobald die Datei des Editors aus einem angezeigten Ordner kommt
    /// — der Regelfall, weil F4 sie von dort nimmt. Welche Ordner die Liste
    /// traegt, entscheidet [`auffrischung::sichtbare_ordner`] und nicht diese
    /// Funktion.
    ///
    /// **Im Messmodus geschieht nichts.** Ein Messlauf misst die Zusagen aus
    /// C8 auf einem Pruefordner, den niemand nebenher aendert; ein Strom
    /// darauf brachte Arbeit in die Messung, die im Betrieb an anderer Stelle
    /// anfiele. Dieselbe Haltung wie bei der Sitzung, die ein Messlauf weder
    /// laedt noch schreibt.
    fn dateisystemwache_nachziehen(&self) {
        if self.ivars().messaufgabe.is_some() {
            return;
        }
        let ordner = auffrischung::sichtbare_ordner(self);
        // Erst den alten Strom fallen lassen, dann den neuen anlegen: sonst
        // beobachteten beide gleichzeitig dieselben Pfade.
        *self.ivars().dateisystemwache.borrow_mut() = None;

        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let wache = Dateisystemwache::einrichten(&ordner, move |gemeldet| {
            let Some(selbst) = schwach.load() else {
                return;
            };
            // Was ein eigener **schneller** Vorgang gerade umschreibt, wird
            // nicht bei jeder Meldung neu gelesen: die Begruendung steht an
            // `auffrischung::schiebt_auffrischung_auf`, und der Abschluss holt
            // die Auffrischung fuer genau diese Ordner nach. Eine Kopie oder
            // eine Verschiebung schiebt nichts auf; ihr angezeigter Zielordner
            // fuellt sich waehrend des Laufs.
            let aufgeschoben = selbst.aufgeschobene_ordner();
            for pfad in gemeldet {
                if auffrischung::auffrischung_aufgeschoben(pfad, &aufgeschoben) {
                    continue;
                }
                auffrischung::ordner_neu_lesen(&*selbst, pfad);
            }
            // **Einmal je Stapel und nicht je Pfad**, und ausserhalb des
            // Aufschubs darueber: der Aufschub beantwortet, ob eine Dateiliste
            // neu zu lesen ist, und das ist eine andere Frage. Ein
            // Stapel-Umbenennen, das die Datei des Editors erwischt, soll
            // gemeldet werden, auch waehrend die Liste stehen bleibt.
            selbst.editor_fremdaenderung_melden(gemeldet);
        });
        if wache.is_none() && !ordner.is_empty() {
            // Ohne Strom zeigt KRK fremde Aenderungen nicht mehr an. Das still
            // hinzunehmen waere die Sorte Fehler, die erst dem Nutzer auffaellt.
            self.dateifenster(self.ivars().modell.borrow().aktiv())
                .quelle()
                .meldung_zeigen(
                    "die Ordner lassen sich nicht beobachten; fremde Aenderungen erscheinen erst nach einem Ordnerwechsel",
                );
        }
        *self.ivars().dateisystemwache.borrow_mut() = wache;
    }

    /// Meldet dem Nutzer, dass die Datei des Editors sich von aussen geaendert
    /// hat (C4).
    ///
    /// **Der erste der beiden Momente aus dem achten Abnahmekriterium von C4**;
    /// der zweite steht unmittelbar vor dem Sichern. Beide stellen dieselbe
    /// Frage an dasselbe Modell, und dieser hier stellt sie nur, wenn der
    /// gemeldete Stapel den Ordner der Datei ueberhaupt nennt — die Entscheidung
    /// darueber trifft [`auffrischung::betrifft_editordatei`], und diese
    /// Funktion trifft sie nicht ein zweites Mal.
    ///
    /// **In die Fenstermeldung und nicht in die Befehlsantwort.** Die fremde
    /// Aenderung ist ein Ereignis, das niemand angefordert hat, und steht damit
    /// auf Rang 3 der Statuszeile; auf Rang 1 loeschte der naechste Tastendruck
    /// sie weg, bevor der Nutzer sie gelesen hat. Denselben Rang nimmt die
    /// Auswurfmeldung aus C9 der Runde 1.
    ///
    /// **In die Zeile des aktiven Dateifensters**, aus demselben Grund wie jede
    /// andere Meldung des Editors: er steht neben beiden Fenstern und gehoert
    /// keinem.
    fn editor_fremdaenderung_melden(&self, gemeldet: &[PathBuf]) {
        let Some(editor) = self.ivars().editor.get() else {
            return;
        };
        if !auffrischung::betrifft_editordatei(gemeldet, self.editordatei().as_deref()) {
            return;
        }
        let Some(satz) = editor.fremdaenderung_melden() else {
            return;
        };
        let aktiv = self.ivars().modell.borrow().aktiv();
        self.dateifenster(aktiv).quelle().meldung_zeigen(&satz);
    }

    /// Die Ordner, deren Auffrischung ein laufender Vorgang gerade aufschiebt.
    ///
    /// Leer in zwei Faellen: es laeuft keine Dateioperation, oder die laufende
    /// schiebt nicht auf. Welche Art aufschiebt, steht in
    /// [`auffrischung::schiebt_auffrischung_auf`] und wird hier nicht ein
    /// zweites Mal beantwortet; diese Methode reicht nur die Ordner des
    /// Vorgangs hinueber.
    fn aufgeschobene_ordner(&self) -> Vec<PathBuf> {
        self.ivars()
            .vorgang
            .borrow()
            .as_ref()
            .map(|vorgang| auffrischung::aufgeschobene_ordner(&vorgang.art, vorgang.ordner()))
            .unwrap_or_default()
    }

    /// Richtet die Beobachtung der Datentraeger ein (C9).
    ///
    /// Sie haengt an keinem Pfad und wird deshalb genau einmal eingerichtet.
    /// Im Messmodus unterbleibt sie, aus demselben Grund wie die
    /// Dateisystembeobachtung.
    fn datentraegerwache_einrichten(&self) {
        if self.ivars().messaufgabe.is_some() {
            return;
        }
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let wache = Datentraegerwache::einrichten(self.mtm(), move |gemeldet| {
            if let Some(selbst) = schwach.load() {
                selbst.datentraeger_gewechselt(gemeldet);
            }
        });
        let _ = self.ivars().datentraegerwache.set(wache);
    }

    /// Ein Datentraeger ist gekommen oder gegangen (C5 und C9).
    fn datentraeger_gewechselt(&self, gemeldet: Datentraeger) {
        // Die Leiste zieht in jedem der drei Faelle nach: ein eingehaengter
        // Datentraeger erscheint dort ohne Neustart, ein ausgeworfener
        // verschwindet, und die Lesezeichen auf ihn wechseln ihre Gueltigkeit
        // (C5). Die Aufzaehlung fragt das System und nicht die Meldung; welcher
        // Datentraeger genau gemeldet wurde, ist dafuer belanglos, und eine
        // Liste, die aus einzelnen Meldungen fortgeschrieben wird, laeuft mit
        // dem ersten verpassten Ereignis auseinander.
        //
        // `willUnmount` zaehlt dabei noch mit: der Datentraeger ist bis zum
        // Auswurf eingehaengt. Das ist richtig so, denn `didUnmount` folgt
        // unmittelbar und zaehlt ohne ihn.
        self.leiste_geraete_nachziehen();
        match gemeldet.art {
            // Fuer die Dateifenster gibt es beim Einhaengen nichts zu tun:
            // keines zeigt einen Ordner, den es vorher nicht gab.
            Wechsel::Eingehaengt => {}
            // Beide Richtungen enden hier. `willUnmount` ist der geordnete
            // Auswurf und der Zeitpunkt, zu dem KRK den Ordner freigeben muss,
            // damit der Auswurf nicht an ihm scheitert; `didUnmount` faengt das
            // abgezogene Medium ab, das niemand vorher angekuendigt hat. Ein
            // zweites Mal richtet der Aufruf nichts an: nach dem ersten steht
            // kein Dateifenster mehr auf dem Datentraeger.
            Wechsel::WirdAusgeworfen | Wechsel::Ausgeworfen => {
                let ausweichziel = benutzerverzeichnis();
                auffrischung::datentraeger_verloren(
                    self,
                    &gemeldet.pfad,
                    &gemeldet.name,
                    &ausweichziel,
                );
            }
        }
    }

    /// Startet die Lesevorgaenge in der Reihenfolge, die das Modell vorgibt.
    ///
    /// Zuerst der sichtbare Tab jedes sichtbaren Dateifensters. Die verdeckten
    /// folgen, sobald der sichtbare bedienbar ist; das loest der Einzugstakt
    /// des jeweiligen Dateifensters aus, siehe [`crate::tabs`].
    fn lesevorgaenge_starten(&self) {
        if let Some(aufgabe) = &self.ivars().messaufgabe
            && let Some(pfad) = aufgabe.startordner()
        {
            // Die Strecken aus S8 lesen allein den Pruefordner, und allein
            // links. Die beiden Sitzungsaufgaben fallen durch auf den
            // Normalweg darunter: das Lesen der sichtbaren Tabs ist Teil
            // dessen, was L4 und L5 messen.
            let pfad = pfad.to_path_buf();
            self.dateifenster(Fensterseite::Links)
                .quelle()
                .ordner_lesen(&pfad, None);
            return;
        }
        let uebersicht = [
            self.dateifenster(Fensterseite::Links).quelle().uebersicht(),
            self.dateifenster(Fensterseite::Rechts)
                .quelle()
                .uebersicht(),
        ];
        let reihenfolge = self.ivars().modell.borrow().lesereihenfolge(uebersicht);
        for (seite, stelle) in reihenfolge {
            if stelle == uebersicht[seite.index()].sichtbar {
                self.dateifenster(seite).quelle().sichtbaren_lesen();
            }
        }
    }

    /// Eines der beiden Dateifenster.
    fn dateifenster(&self, seite: Fensterseite) -> &Dateifenster {
        &self
            .ivars()
            .dateifenster
            .get()
            .expect("die Dateifenster stehen seit `oberflaeche_aufbauen`")[seite.index()]
    }

    // ------------------------------------------------------------------
    // Kommandos
    // ------------------------------------------------------------------

    /// Fuehrt aus, was der Ereignisabgriff geliefert hat.
    ///
    /// Die eine Stelle, die entscheidet, wohin ein Tastendruck geht. Ein
    /// getipptes Zeichen gehoert dem Filtertext des sichtbaren Tabs, und der
    /// verkuerzt die Dateiliste; es geht deshalb an das aktive Dateifenster,
    /// **wenn der Fokus dort steht**, und sonst nirgendwohin. Bis zum 260814
    /// gehoerte es der Sprungmarke aus C2 der Runde 1; der Zweig hat seine Form
    /// behalten und allein sein Ziel gewechselt.
    ///
    /// **Der Vorbehalt ist derselbe, den jedes Kommando durchlaeuft, und keine
    /// Sonderregel fuer den Editor.** Bis zum 260809 fehlte er hier: ein
    /// Zeichen ist kein Kommando, traegt keinen
    /// [`Wirkungsbereich`](krk_core::tasten::Wirkungsbereich), und der eine
    /// Fokusvorbehalt in [`Self::kommando_ausfuehren`] sitzt im anderen Zweig.
    /// Mit der Schreibmarke im Editor lief jeder Buchstabe in den Suchpuffer
    /// der damaligen Sprungmarke, verschob dort die Auswahl und erreichte die
    /// Textflaeche nie
    /// (`issues/260809-1648_*_die-sprungmarke-geht-ohne-fokuspruefung-in-das-aktive-dateifenster.md`).
    fn eingabe_ausfuehren(&self, eingabe: Eingabe) -> bool {
        if self.ivars().dateifenster.get().is_none() {
            return false;
        }

        // **Der Merker der Tastenwiederholung gehoert der Rueckschritt-Taste
        // und keiner anderen Eingabe.** Jede andere Eingabe setzt ihn zurueck,
        // und diese eine Zeile nimmt der Regel in
        // [`crate::kommandos::rueckschritt`] eine Annahme aus der Rechnung:
        // dass AppKit `isARepeat` nur fuer aufeinanderfolgende Druecke
        // **derselben** Taste setzt. Stimmt die Annahme, aendert die Zeile
        // nichts; stimmt sie nicht, traegt die Regel trotzdem.
        //
        // Gefragt ist dieselbe Funktion, die der Zweig unten fragt, und nicht
        // eine zweite Fassung derselben Frage: zwei Fassungen koennten
        // auseinanderlaufen, und dann liesse die falsche Haelfte die
        // Loeschrueckfrage auf einen berichtigten Vertipper aufgehen.
        let nackter_rueckschritt = match eingabe {
            Eingabe::Kommando { anschlag, .. } => anschlag.ist_nackter_rueckschritt(),
            Eingabe::Zeichen(_) => false,
        };
        if !nackter_rueckschritt {
            self.ivars().rueckschritt_merker.set(false);
        }

        match eingabe {
            Eingabe::Kommando { kommando, anschlag } => {
                self.kommando_ausfuehren(kommando, Some(anschlag))
            }
            Eingabe::Zeichen(zeichen) => {
                // **Dieselbe Erhebung wie im Kommandozweig, und dieselben drei
                // Werte.** Ein getipptes Zeichen ist kein Kommando: es traegt
                // keinen Wirkungsbereich, und `zulaessig` hat ihm nichts zu
                // sagen. Die Eingaben der Frage braucht es trotzdem alle drei.
                //
                // Ein Zeichen gehoert dem Blatt, solange eines steht: der
                // Filter verkuerzt eine Liste, die der Nutzer gerade nicht
                // bedient. Und es gehoert dem Textfeld, solange der
                // Ersthelfer AppKit gehoert — bis zur Runde 7 stand diese Frage
                // als frueher Ausstieg im Ereignisabgriff und erreichte diesen
                // Zweig nie. Ohne sie liefe ein Zeichen waehrend einer
                // Umbenennung in der Liste in den Filtertext.
                let lage = self.lage();
                if lage.blatt_steht || lage.ersthelfer_gehoert_appkit {
                    return false;
                }
                match lage.fokus {
                    Fokus::Dateifenster => {
                        let aktiv = self.ivars().modell.borrow().aktiv();
                        self.dateifenster(aktiv)
                            .quelle()
                            .filterzeichen_tippen(zeichen)
                    }
                    // Keiner dieser vier Bereiche traegt einen Filtertext. Der
                    // Rueckgabewert `false` ist die Zusage: nur ein nicht
                    // ausgefuehrter Tastendruck laeuft unveraendert an AppKit
                    // weiter, und nur dann tippt die Textflaeche des Editors
                    // das Zeichen. Die Leiste und die Vorschau haben bis zum
                    // 260809 stillschweigend die damalige Sprungmarke des
                    // Dateifensters bedient; das endet mit derselben Zeile,
                    // wie S17 es vorsieht.
                    Fokus::Leiste | Fokus::Vorschau | Fokus::Editor | Fokus::Anderswo => false,
                }
            }
        }
    }

    /// Ob am Hauptfenster gerade ein Blatt steht.
    ///
    /// Die eine Abfrage dafuer. Sie deckt jedes Blatt ab, auch die Pfadeingabe
    /// aus C2 und die kommenden aus S17, und nicht nur die vier aus diesem
    /// Schritt.
    ///
    /// **Sie bleibt eine eigene Frage und geht nicht in
    /// [`Self::schluesselfenster`] auf.** Die beiden Werte sind unabhaengig:
    /// steht ein Blatt und oeffnet der Nutzer daneben den Ueber-Dialog, ist
    /// `blatt_steht` wahr und das Schluesselfenster `Fremd`. Umgekehrt meldet
    /// ein Blatt, das selbst das Schluesselfenster ist,
    /// `Schluesselfenster::BlattAmHauptfenster`, und der Abbruch aus dem Blatt
    /// heraus bleibt erreichbar, weil ueber ihn allein
    /// `waehrend_blatt_erlaubt` entscheidet.
    fn blatt_steht(&self) -> bool {
        self.ivars()
            .fenster
            .get()
            .and_then(|fenster| fenster.attachedSheet())
            .is_some()
    }

    /// Welches Fenster gerade das Schluesselfenster ist.
    ///
    /// **Die eine Abfrage von `NSApplication::keyWindow` fuer die
    /// Zulaessigkeit**, und ihr Wert beantwortet zwei Fragen: ob ein Befehl
    /// ueberhaupt wirken darf, ueber [`Schluesselfenster::gehoert_krk`], und ob
    /// [`Self::fokus_bei`] in den Ansichtsbaum des Hauptfensters hineinsehen
    /// darf. [`Self::lage`] erhebt sie deshalb **einmal** und reicht den Wert an
    /// beide weiter; zwei Erhebungen desselben Augenblicks koennten
    /// auseinanderlaufen, eine kann es nicht.
    ///
    /// Gefragt ist die Naemlichkeit und nicht die Klasse, wie in
    /// [`Self::ist_eigene_textflaeche`]: verglichen wird ueber `isEqual:` gegen das
    /// Hauptfenster und gegen dessen `attachedSheet`. Ein Panel, das KRK nicht
    /// gehoert, faellt damit auf [`Schluesselfenster::Fremd`], und ebenso ein
    /// KRK, das gar kein Schluesselfenster hat.
    fn schluesselfenster(&self) -> Schluesselfenster {
        let (Some(schluessel), Some(haupt)) = (
            NSApplication::sharedApplication(self.mtm()).keyWindow(),
            self.ivars().fenster.get(),
        ) else {
            return Schluesselfenster::Fremd;
        };
        if schluessel.isEqual(Some(haupt)) {
            return Schluesselfenster::Hauptfenster;
        }
        match haupt.attachedSheet() {
            Some(blatt) if schluessel.isEqual(Some(&*blatt)) => {
                Schluesselfenster::BlattAmHauptfenster
            }
            _ => Schluesselfenster::Fremd,
        }
    }

    /// Die vier Eingaben der Zulaessigkeitsfrage, an einer Stelle erhoben.
    ///
    /// **Die eine Erhebung, und die eine Aufrufstelle von
    /// [`ereignisse::ersthelfer_gehoert_appkit`].** Drei Abnehmer lesen sie: der
    /// Kommandozweig in [`Self::kommando_ausfuehren`] gibt sie an
    /// [`zulaessigkeit::zulaessig`], der Zeichenzweig von
    /// [`Self::eingabe_ausfuehren`] liest drei der vier Werte einzeln heraus,
    /// und die Ausgrauung des Hauptmenues fragt dieselbe Regel auf demselben
    /// Wert. Zwei Erhebungen desselben Augenblicks koennten auseinanderlaufen;
    /// eine kann es nicht.
    ///
    /// **Das Schluesselfenster wird ebenfalls einmal erhoben**, und aus
    /// demselben Grund. Der Wert aus [`Self::schluesselfenster`] geht an zwei
    /// Stellen: als Wahrheitswert in die `Lage` und als Vorabfrage in
    /// [`Self::fokus_bei`], das ohne ihn selbst wieder `keyWindow` fragen
    /// muesste. Ein `self.fokus()` an dieser Stelle waere genau diese zweite
    /// Erhebung.
    ///
    /// **Der Fokuswert dient danach zweierlei**, und das ist kein Widerspruch:
    /// als Vorbehalt entscheidet er in `zulaessig`, ob der Befehl wirkt, und
    /// weiter unten als Adresse, wohin er geht. Die zweite Verwendung fragt
    /// nicht noch einmal nach.
    fn lage(&self) -> Lage {
        let schluesselfenster = self.schluesselfenster();
        Lage {
            blatt_steht: self.blatt_steht(),
            ersthelfer_gehoert_appkit: ereignisse::ersthelfer_gehoert_appkit(
                self.mtm(),
                &|ersthelfer| self.ist_eigene_textflaeche(ersthelfer),
            ),
            schluesselfenster_gehoert_krk: schluesselfenster.gehoert_krk(),
            fokus: self.fokus_bei(schluesselfenster),
        }
    }

    /// Fuehrt ein Kommando aus, das der Ereignisabgriff nachgeschlagen hat.
    ///
    /// **Liefert, ob der Befehl zulaessig war, und nicht mehr, ob sein Rumpf
    /// etwas getan hat.** Nur bei `true` schluckt der Abgriff das Ereignis. Bis
    /// zur Runde 7 lautete die Grenze „ausgefuehrt", und sie war richtig,
    /// solange das Hauptmenue kein Kuerzel eines KRK-Befehls trug: ein
    /// wirkungsloser Befehl sollte dem Menue sein Kuerzel nicht abnehmen.
    /// Sobald das Menue alle Kuerzel traegt, kehrt sich das um — ein
    /// zulaessiger, aber wirkungsloser Befehl liefe ueber den Umweg Menue ein
    /// zweites Mal. Was der Rumpf gemeldet hat, bleibt darunter erhalten und
    /// entscheidet weiterhin ueber die beiden Nachwirkungen.
    ///
    /// **Die eine Stelle, die vor dem Ausfuehren nach der Zulaessigkeit
    /// fragt**, und sie fragt [`zulaessigkeit::zulaessig`] mit der einen
    /// [`Lage`] aus [`Self::lage`]. Bis zur Runde 7 standen hier zwei getrennte
    /// Vorbehalte, das stehende Blatt und der Fokus, waehrend der dritte
    /// Bestandteil im Ereignisabgriff wohnte; alle drei stehen jetzt in der
    /// einen Regel, die auch das Hauptmenue fragt.
    ///
    /// # Warum der Anschlag mitkommt
    ///
    /// `anschlag` ist der Tastendruck, der diesen Befehl ausgeloest hat, samt
    /// der Auskunft, ob er aus einer Tastenwiederholung stammt. **`None` ist
    /// die Aussage „es gab keinen Tastendruck"**, und damit die Antwort auf
    /// C1.19 und C6.11 in der Signatur statt in einem Zweig: der Menueeintrag
    /// und der Melder der Bereichsleiste geben `None`, allein die Senke des
    /// Ereignisabgriffs reicht einen Anschlag durch.
    ///
    /// Gebraucht wird er von genau einem Zweig,
    /// [`Self::papierkorb_oder_zeichen_zurueck`]. Der Grund steht dort und in
    /// [`crate::kommandos::rueckschritt`]: `resources/default-keymap.toml` legt
    /// `delete` und `cmd+delete` auf dieselbe Funktion, und beide werden im
    /// Nachschlag zu demselben [`Kommando`], bevor irgendjemand fragen kann.
    ///
    /// **Die Zulaessigkeitsregel bekommt ihn nicht.** Sie bleibt unveraendert,
    /// und ihre Tafel aus 280 Faellen behaelt ihre Bedeutung; eine Antwort dort
    /// traefe beide Wege zugleich und graute den Menueeintrag aus. Der
    /// Datensatz dazu ist
    /// `decisions/260814-2102_*_gehoert-die-fallunterscheidung-der-rueckschritt-taste-in-die-zulaessigkeitsregel.md`.
    fn kommando_ausfuehren(&self, kommando: Kommando, anschlag: Option<Anschlag>) -> bool {
        // Die vier Bestandteile und ihre Herleitung stehen in
        // `kommandos::zulaessigkeit`. Kurz: die Blattsperre laesst allein den
        // Abbruch durch, ein Textfeld behaelt seine AppKit-Bedeutung, ein fremdes
        // Schluesselfenster haelt alles auf, und der Wirkungsbereich muss zum
        // Fokus passen. **Die Ausnahmeliste `zulaessigkeit::immer_erreichbar`
        // hebt davon drei auf und nicht nur den dritten**: Beenden,
        // FensterSchliessen und FensterEinblenden kommen auch durch ein
        // stehendes Blatt und durch ein Textfeld. Waehrend eines Blattes sind
        // es damit vier Kommandos und nicht eines; bis zum 260818 stand die
        // Ausnahme hier allein am dritten Glied
        // (`issues/260817-1302_*_zwei-weitere-stellen-tragen-die-verkuerzte-blattsperre-*.md`).
        //
        // Ein laufender Vorgang sperrt seit S16b **nicht**: C4 sagt zu, dass
        // Navigation, Markierung und Tabwechsel waehrend einer Operation
        // wirken, und der Fortschritt steht in der Statuszeile statt in einem
        // Blatt. Dass ein zweiter Operationsbefehl nichts startet, prueft
        // `auftrag_stellen` und meldet es; eine Tastensperre dafuer waere zu
        // grob.
        let lage = self.lage();
        if !zulaessigkeit::zulaessig(kommando, lage) {
            return false;
        }

        // Derselbe Wert, jetzt als Adresse und nicht mehr als Vorbehalt; siehe
        // den Modulkopf und `Self::lage`. Ein zweites `self.fokus()` waere eine
        // zweite Erhebung desselben Augenblicks.
        let fokus = lage.fokus;

        // **Die eine Loeschregel der Befehlsantwort.** Was KRK auf den vorigen
        // Befehl geantwortet hat, gilt bis zum naechsten und keinen Tastendruck
        // laenger; erst danach darf der Befehl seine eigene Antwort setzen. Die
        // Regel selbst steht in `Self::befehlsantwort_beidseitig_loeschen`,
        // seit der Abwurf aus C7 der Runde 13 sie als zweiter Weg braucht.
        self.befehlsantwort_beidseitig_loeschen();

        // **Zuerst nachlesen, was auf dem Schirm steht, dann erst das Modell
        // anfassen.** Der Nutzer kann jede Trennlinie mit der Maus verschoben
        // haben, und im Fenstermodell steht davon nichts, solange niemand
        // nachmisst. Ohne diese Zeile rechnet der Nachzug unten mit einer
        // ueberholten Zahl und setzt die Ziehbewegung zurueck — auch bei einem
        // Befehl, der weder eine Breite noch eine Sichtbarkeit aendert, etwa
        // dem Ab-Pfeil in der Dateiliste (Defekt vom 260811-1245).
        self.bildschirmbreiten_uebernehmen();

        // **Was der Rumpf meldet, ist seit der Runde 7 nicht mehr der
        // Rueckgabewert dieser Funktion.** Der Wert traegt genau eine Aufgabe
        // weiter: er entscheidet ueber die beiden Nachwirkungen unten. Ein
        // Befehl, der nichts getan hat, braucht weder einen Nachzug der
        // Aufteilung noch eine vorgemerkte Sitzung.
        let gewirkt = match kommando {
            Kommando::Kopieren => self.uebertragen(kommando),
            Kommando::Verschieben => self.uebertragen(kommando),
            // **Der eine Zweig, dessen falsche Haelfte die Loeschrueckfrage
            // aufgehen laesst.** Er fragt zuerst, ob der Anschlag die nackte
            // Rueckschritt-Taste war, und ruft dann die Regel; alles andere
            // geht unveraendert in die Rueckfrage vor dem Papierkorb. Die
            // Fallunterscheidung selbst steht in
            // [`crate::kommandos::rueckschritt`] und nicht hier.
            Kommando::InPapierkorb => self.papierkorb_oder_zeichen_zurueck(anschlag),
            Kommando::Abbrechen => self.abbrechen(),
            Kommando::OrdnerAnlegen => self.anlegen(Anlegeart::Ordner),
            Kommando::DateiAnlegen => self.anlegen(Anlegeart::Datei),
            Kommando::UmbenennenStapel => self.stapel_umbenennen(),
            Kommando::TerminalOeffnen => self.terminal_oeffnen(),
            Kommando::ZwischenablageAnsehen => self.zwischenablage_ansehen(),
            Kommando::FensterWechseln => self.ivars().modell.borrow_mut().fenster_wechseln(),
            Kommando::LeisteUmschalten => self.bereich_umschalten(Bereich::Lesezeichen),
            // Die beiden Dateifenster gehen durch dieselbe Stelle, seit das
            // linke ausblendbar ist. Dass eines von beiden stehen bleibt,
            // entscheidet das Fenstermodell und nicht dieser Zweig.
            Kommando::ErstesFensterUmschalten => self.bereich_umschalten(Bereich::Links),
            Kommando::ZweitesFensterUmschalten => self.bereich_umschalten(Bereich::Rechts),
            // Ohne Nachfrage, obwohl eine eingeblendete Vorschau dem Editor
            // nach C1 die Flaeche nimmt: der verdraengte Editor behaelt seinen
            // Stand. Bis zum 260810 hing hier der dritte Anlass aus C4; die
            // Begruendung fuer seinen Wegfall steht bei `Anlass`.
            Kommando::VorschauUmschalten => self.bereich_umschalten(Bereich::Vorschau),
            // Die drei Spaltenschalter aus C3 der Bereichsleisten-Runde. Sie
            // stehen hier und nicht bei `bereichskommando`, obwohl sie eine
            // Dateiliste betreffen: sie betreffen **beide**, und ein einzelnes
            // Dateifenster kommt an das andere nicht heran. Denselben Weg geht
            // das Umbenennen in der Liste, aus demselben Grund.
            Kommando::SpalteGroesseUmschalten => self.spalte_umschalten(Spalte::Groesse),
            Kommando::SpalteDatumUmschalten => self.spalte_umschalten(Spalte::Geaendert),
            Kommando::SpalteTypUmschalten => self.spalte_umschalten(Spalte::Typ),
            // Der Schalter "Deep" aus C5 der Filter-Runde. **Ein eigener
            // Zweig, und der Uebersetzer haette ihn nicht verlangt** (C5.6):
            // das `match` endet mit einem Auffangzweig auf `bereichskommando`,
            // und dort fiele der Befehl stillschweigend hindurch und taete
            // nichts. Er traegt `Wirkungsbereich::Ueberall` und kommt damit
            // auch mit dem Fokus in der Leiste oder im Editor an, wo
            // `bereichskommando` kein Dateifenster anzusprechen wuesste.
            //
            // Er kippt das Kennzeichen am Modell des sichtbaren Tabs im
            // **aktiven** Dateifenster und nicht im fokussierten: geklickt wird
            // das Kaestchen in der Bereichsleiste, und der Fokus steht dabei,
            // wo er eben steht. Denselben Weg gehen die drei Spaltenschalter
            // darueber.
            //
            // Liefert immer `true`. Der Befehl war zustaendig, auch wenn kein
            // Filtertext steht und die Liste sich nicht aendert; ueber die
            // Zulaessigkeit hat der Wirkungsbereich entschieden und nicht das
            // Ergebnis (C2.4).
            Kommando::TiefeSucheUmschalten => {
                let seite = self.ivars().modell.borrow().aktiv();
                self.dateifenster(seite).quelle().tiefe_suche_umschalten();
                true
            }
            // Der Schalter "Content" aus C2 der Inhaltsfilter-Runde. **Ein
            // eigener Zweig, und der Uebersetzer haette ihn nicht verlangt**,
            // aus demselben Grund wie bei "Deep" darueber: das `match` endet
            // mit einem Auffangzweig auf `bereichskommando`, und dort fiele der
            // Befehl stillschweigend hindurch und taete nichts. Von den sechs
            // Stellen, die dieses Kommando anfassen muss, ist diese die
            // einzige, fuer die weder Uebersetzer noch Probe buergt.
            //
            // Adresse und Rueckgabewert folgen "Deep" Zeile fuer Zeile: das
            // **aktive** Dateifenster und nicht das fokussierte, und immer
            // `true`. Der Befehl war zustaendig, auch wenn der Filtertext zu
            // kurz ist und die Liste sich nicht aendert; ueber die
            // Zulaessigkeit hat der Wirkungsbereich entschieden und nicht das
            // Ergebnis.
            Kommando::InhaltssucheUmschalten => {
                let seite = self.ivars().modell.borrow().aktiv();
                self.dateifenster(seite).quelle().inhaltssuche_umschalten();
                true
            }
            Kommando::FensterEinblenden => {
                self.fenster_zeigen();
                true
            }
            Kommando::FensterSchliessen => self.fenster_schliessen(),
            Kommando::Beenden => self.beenden(),
            // **Ein eigener Zweig, und der Uebersetzer haette ihn nicht
            // verlangt.** Das `match` hier endet mit einem Auffangzweig auf
            // `bereichskommando`; ein neues Kommando ohne eigenen Zweig fiele
            // dort stillschweigend hindurch und taete nichts.
            Kommando::WeitereInstanz => self.weitere_instanz_starten(),
            Kommando::BereichVerbreitern => self.breite_aendern(BREITENSCHRITT),
            Kommando::BereichVerschmaelern => self.breite_aendern(-BREITENSCHRITT),
            // Die Lesezeichen aus C5. Sie stehen hier und nicht in der Leiste,
            // weil jeder von ihnen danach `bookmarks.toml` schreiben muss und
            // der Zugang zur Ablage beim Delegierten haengt.
            Kommando::LesezeichenAnlegen => self.lesezeichen_anlegen(),
            Kommando::LesezeichenUmbenennen => self.lesezeichen_umbenennen(),
            Kommando::LesezeichenLoeschen => self.lesezeichen_loeschen(),
            Kommando::LesezeichenHoch => self.lesezeichen_verschieben(Verschiebung::Hoch),
            Kommando::LesezeichenRunter => self.lesezeichen_verschieben(Verschiebung::Runter),
            Kommando::FokusLeiste => self.fokus_holen(Fokus::Leiste),
            Kommando::FokusDateifenster => self.fokus_holen(Fokus::Dateifenster),
            // Wie `VorschauUmschalten` daneben: der Fokusbefehl holt seinen
            // Bereich hervor und verdraengt damit den Editor, ohne dessen Stand
            // anzufassen.
            Kommando::FokusVorschau => self.fokus_holen(Fokus::Vorschau),
            Kommando::FokusEditor => self.fokus_editor_holen(),
            // Der erste der beiden Einstiege in den Editor (C2). Er steht hier
            // und nicht bei `bereichskommando`, obwohl er
            // `Wirkungsbereich::Dateifenster` traegt: er nimmt dessen
            // ausgewaehlten Eintrag, fuellt damit aber einen anderen Bereich,
            // und ein einzelnes Dateifenster kommt an den Editor nicht heran.
            //
            // Derselbe Rumpf laeuft seit dem 260823 auch fuer `cmd+e` in der
            // Dateiliste, ueber den Zweig darunter.
            Kommando::Bearbeiten => self.im_editor_oeffnen(),
            // Der Rundweg aus dem Nutzerentscheid vom 260823-0942. Er steht aus
            // demselben Grund hier wie F4 darueber: jeder seiner drei Wege
            // greift ueber die Grenze eines Bereichs hinweg, und keiner der
            // Bereiche kommt von sich aus an den Delegierten heran.
            //
            // **Ein eigener Zweig, und der Uebersetzer haette ihn nicht
            // verlangt.** Das `match` hier endet mit einem Auffangzweig auf
            // `bereichskommando`; ohne diese Zeile fiele der Befehl dort
            // stillschweigend hindurch und taete nichts — genau die Gestalt, die
            // fuer `cmd+e` als Defekt gemeldet war.
            Kommando::EditorRundweg => self.editor_rundweg(fokus),
            // Das Sichern aus C4. Es traegt `Wirkungsbereich::Editor` und
            // steht trotzdem hier und nicht bei `bereichskommando`: der
            // Editorbereich haengt am Delegierten, und `bereichskommando`
            // reicht dem Editor nichts zu (siehe die Begruendung dort).
            Kommando::EditorSichern => self.editor_sichern(),
            // Der erste Anlass der Nachfrage aus C4. Er steht hier und nicht
            // bei `bereichskommando`, aus demselben Grund wie das Sichern
            // darueber: der Editorbereich haengt am Delegierten.
            //
            // `false`: `opt+cmd+e` laesst die Flaeche leer zurueck. Der zweite
            // Rufer desselben Rumpfs, der Rueckweg von `cmd+e`, uebergibt
            // `true` und holt die Vorschau zurueck.
            Kommando::EditorSchliessen => self.editor_schliessen(false),
            // Der Umschalter aus C6 der Bereichsleisten-Runde. Er steht neben
            // dem Schliessen darueber und ist nicht dasselbe: er blendet aus
            // und behaelt die Datei, loest also keine Nachfrage aus. Der
            // Unterschied im Einzelnen steht an `editor_umschalten`.
            Kommando::EditorUmschalten => self.editor_umschalten(),
            // Der Wechsel zwischen den beiden Ansichten aus C3. Er steht hier
            // und nicht bei `bereichskommando`, aus demselben Grund wie das
            // Sichern und das Schliessen darueber: der Editorbereich haengt am
            // Delegierten.
            Kommando::EditorAnsichtUmschalten => self.editor_ansicht_umschalten(),
            // Die sechs Befehle aus C5. Sie stehen hier und nicht bei
            // `bereichskommando`, aus demselben Grund wie die drei darueber:
            // der Editorbereich haengt am Delegierten. Die beiden mit einem
            // Blatt tragen ihre eigene Funktion, weil sie ein Fenster
            // brauchen; die vier uebrigen sind je ein Ruf in den Editor und
            // eine Meldung zurueck und gehen deshalb durch dieselbe Stelle.
            Kommando::EditorZeileSpringen => self.editor_zeile_springen(),
            Kommando::EditorSuchen => self.editor_suchen(),
            Kommando::EditorWeitersuchen => self.editorbefehl(Editorbereich::weitersuchen),
            Kommando::EditorRueckwaertsSuchen => {
                self.editorbefehl(Editorbereich::rueckwaerts_suchen)
            }
            Kommando::EditorErsetzen => self.editorbefehl(Editorbereich::treffer_ersetzen),
            Kommando::EditorAlleErsetzen => self.editorbefehl(Editorbereich::alle_treffer_ersetzen),
            Kommando::BelegungAnsehen => self.belegung_ansehen(),
            // Der Notizzettel aus C1 der Runde 9. Er steht hier und nicht bei
            // `bereichskommando`, weil er `Wirkungsbereich::Ueberall` traegt
            // und keinem Bereich gehoert: er geht aus jedem der fuenf Fokuswerte
            // auf, und ein einzelnes Dateifenster wuesste mit ihm nichts
            // anzufangen. **Ohne diesen Zweig fiele der Befehl durch den
            // Auffangzweig unten und taete nichts**, und der Uebersetzer sagte
            // dazu kein Wort.
            Kommando::Notizzettel => self.notizzettel_zeigen(),
            // Cmd+W aus jedem Fokus (C4 der Runde 4). Der einzige Befehl
            // dieser Runde, der ueber die Bereiche hinweg entscheidet, und
            // deshalb der einzige, der hier einen Zweig bekommt: er traegt
            // seit C4 `Wirkungsbereich::Ueberall` und kommt damit auch mit dem
            // Fokus in der Leiste und im Editor an, wo `bereichskommando`
            // keinen Tab zu schliessen wuesste.
            Kommando::TabSchliessen => self.tab_schliessen(fokus),
            // Der Ordnersprung aus C2 der Runde 6. Er steht hier und nicht bei
            // `bereichskommando`, aus demselben Grund wie `cmd+w` darueber: er
            // traegt `Wirkungsbereich::Ueberall` und nimmt seine Quelle aus
            // der Vorschau oder dem Editor, sein Ziel aber aus dem aktiven
            // Dateifenster. Ein einzelnes Dateifenster kommt an beide Quellen
            // nicht heran.
            Kommando::OrdnerDerDatei => self.ordner_der_datei_zeigen(),
            // Das Angleichen aus C1 der Runde 13. **Ein eigener Zweig, und der
            // Uebersetzer haette ihn nicht verlangt**: das `match` hier endet
            // mit einem Auffangzweig auf `bereichskommando`, und dort fiele der
            // Befehl stillschweigend hindurch und taete nichts. Er traegt
            // `Wirkungsbereich::Dateifenster` und stuende damit scheinbar dem
            // Auffangzweig zu; das Ziel ist aber das **andere** Dateifenster,
            // und an das kommt ein einzelnes nicht heran. Denselben Weg gehen
            // die drei Spaltenschalter weiter oben, aus demselben Grund.
            Kommando::OrdnerAngleichen => self.ordner_angleichen(),
            // Das Teilen aus C1 der Runde 6. Es steht hier und nicht bei
            // `bereichskommando`, aus demselben Grund wie die beiden Befehle
            // darueber: es traegt `Wirkungsbereich::Ueberall` und kommt damit
            // aus jedem Fokus an, auch aus der Leiste und dem Editor, wo ein
            // einzelnes Dateifenster nichts beizutragen haette. Der Fokuswert
            // geht mit, weil er hier keinen Vorbehalt mehr traegt, sondern die
            // Adresse ist — wie bei `tab_schliessen` darueber.
            Kommando::Teilen => self.teilen(fokus),
            // Alles uebrige gehoert dem Bereich, der den Fokus hat.
            andere => self.bereichskommando(fokus, andere),
        };
        if gewirkt {
            self.aufteilung_nachziehen();
            self.sitzung_vormerken();
        }
        true
    }

    /// Reicht ein Kommando an den Bereich weiter, der den Fokus hat.
    ///
    /// **Keine zweite Fokusabfrage.** Der Wert kommt aus der einen Abfrage in
    /// [`Self::kommando_ausfuehren`] und beantwortet hier eine andere Frage:
    /// nicht, **ob** der Befehl wirkt — das hat der Wirkungsbereich schon
    /// entschieden —, sondern **wohin** er geht. Beide Bereiche sind Listen mit
    /// einer Auswahl, und der Auf- und der Ab-Pfeil bewegen nach C2 wie nach C5
    /// die des Bereichs, vor dem der Nutzer steht; ohne eine Adresse gaebe es
    /// keinen Ort, an den ein solcher Befehl zu richten waere.
    ///
    /// [`Fokus::Anderswo`] geht an das Dateifenster. Ein Befehl, der einen
    /// Bereich braucht, ist dort schon abgewiesen; was uebrig bleibt, ist die
    /// Bewegung der Auswahl, und die gehoert der Liste, die der Nutzer zuletzt
    /// bedient hat.
    fn bereichskommando(&self, fokus: Fokus, kommando: Kommando) -> bool {
        match fokus {
            Fokus::Leiste => self.leiste().quelle().kommando_ausfuehren(kommando),
            // Die vier Tabbefehle aus C1 bedienen hier die Vorschau-Tabs
            // (C6); alles andere fuehrt die Vorschau nicht aus, und dann faellt
            // kein Nachzug an. Verbraucht ist der Tastendruck auch dann, denn
            // geschluckt wird seit der Runde 7, was zulaessig war, und nicht
            // mehr, was gewirkt hat.
            Fokus::Vorschau => {
                let ausgefuehrt = self.vorschau().kommando_ausfuehren(kommando);
                if ausgefuehrt {
                    // Der dritte der vier Anlaesse aus C11: ein Tabwechsel der
                    // Vorschau zeigt eine andere Datei, und mit dem Fokus hier
                    // steht deren Pfad im Titel. Der Ersthelfer wechselt dabei
                    // nicht, also meldet das Fenster nichts.
                    self.titel_nachziehen(fokus);
                }
                ausgefuehrt
            }
            // **Seit S17 erreichbar, und `false` ist die Antwort, die
            // bleibt.** Der Editor bekommt hier keine Adresse, weil ihm hier
            // nichts zugestellt wird: die neun Befehle mit
            // [`Wirkungsbereich::Editor`] holen sich ihren eigenen Zweig in
            // [`Self::kommando_ausfuehren`], so wie die Fokusbefehle es tun
            // (S20, S22, S23, S25, S32, S34 und die folgenden). Was mit dem
            // Fokus im Editor bis hierher durchkommt, ist ein Befehl mit
            // [`Wirkungsbereich::Ueberall`], den das Fenster selbst nicht
            // ausfuehrt — und der gehoert nicht ins Dateifenster umgeleitet.
            // `false` heisst dann, was es in diesem `match` ueberall heisst:
            // kein Nachzug der Aufteilung und keine vorgemerkte Sitzung. **Den
            // Tastendruck gibt es nicht an AppKit zurueck** — bis zur Runde 7
            // tat es das, seither schluckt der Abgriff jeden zulaessigen
            // Befehl, und ein `Wirkungsbereich::Ueberall` ist mit dem Fokus im
            // Editor zulaessig. Der Befehl wird in der Textflaeche also weder
            // zu einem Zeichen noch zu einer Bewegung der Schreibmarke; er tut
            // nichts, und das ist die Wahl der Runde 7 und kein Versehen.
            Fokus::Editor => false,
            Fokus::Dateifenster | Fokus::Anderswo => {
                let aktiv = self.ivars().modell.borrow().aktiv();
                self.dateifenster(aktiv)
                    .quelle()
                    .kommando_ausfuehren(kommando)
            }
        }
    }

    /// Schliesst den aktiven Tab, aus jedem Fokus heraus (C4 der Runde 4).
    ///
    /// **Die eine Verzweigung des Befehls, und sie hat zwei Ausgaenge.** Ueber
    /// die fuenf Fokuswerte ist sie vollstaendig und ueberschneidungsfrei:
    ///
    /// - [`Fokus::Dateifenster`] und [`Fokus::Vorschau`] gehen an
    ///   [`Self::bereichskommando`], also an den Bereich vor dem Nutzer. Das
    ///   ist die Zuordnung aus C6 der Runde 1, und C4 sagt ausdruecklich zu,
    ///   dass sie fuer diese beiden gueltig bleibt: an diesen Tastendruck
    ///   aendert die Runde 4 nichts.
    /// - [`Fokus::Leiste`], [`Fokus::Editor`] und [`Fokus::Anderswo`] gehen an
    ///   den sichtbaren Tab der aktiven Fensterseite. Fuer die ersten beiden
    ///   ist das die bestellte Luecke (Nutzerantwort vom 260811-1505);
    ///   `Anderswo` steht bei ihnen, weil es kein Bereich mit Tabs ist und
    ///   "der Bereich vor dem Nutzer" dort keine Antwort hat, waehrend die
    ///   aktive Fensterseite immer eine ist.
    ///
    /// **Der Editor wird auf keinem der beiden Wege angefasst.** Er behaelt
    /// seine Datei und seinen Stand; `cmd+w` schliesst dort einen Tab des
    /// Dateifensters und nicht das Dokument, und ein vierter Anlass der
    /// Nachfrage aus C4 der Editor-Runde entsteht nicht. Die dritte
    /// Moeglichkeit des Datensatzes `260811-1257_*` ist ausdruecklich nicht
    /// gewaehlt worden.
    ///
    /// [`Fokus::Anderswo`] ist nach Lage der Dinge nicht erreichbar: ein
    /// stehendes Blatt haelt das Kommando schon in
    /// [`Self::kommando_ausfuehren`] an, und mit der Schreibmarke in einem
    /// Textfeld reicht der Ereignisabgriff den Tastendruck an AppKit weiter,
    /// bevor er nachschlaegt. Der Zweig steht trotzdem ausgeschrieben: eine
    /// Fallunterscheidung, die einen Fall nicht kennt, beantwortet ihn beim
    /// ersten Auftreten falsch.
    fn tab_schliessen(&self, fokus: Fokus) -> bool {
        match fokus {
            Fokus::Dateifenster | Fokus::Vorschau => {
                self.bereichskommando(fokus, Kommando::TabSchliessen)
            }
            Fokus::Leiste | Fokus::Editor | Fokus::Anderswo => {
                let aktiv = self.ivars().modell.borrow().aktiv();
                self.dateifenster(aktiv).quelle().tab_schliessen();
                true
            }
        }
    }

    /// Zeigt den Ordner der angezeigten Datei im aktiven Dateifenster, mit der
    /// Auswahl auf dieser Datei (C2 der Runde 6).
    ///
    /// **Die vier Eingaben stehen hier, die Rechnung darueber nicht.** Welche
    /// Datei "die angezeigte" ist, beantwortet [`angezeigtedatei::welche`]
    /// ohne AppKit und damit ohne Fenster pruefbar; diese Stelle liest die
    /// Sichtbarkeit aus dem Fenstermodell und die beiden Pfade aus den
    /// Bereichen, die sie halten. Das Teilen aus C1 fragt dieselbe Funktion,
    /// und eine zweite Rechnung daneben gaebe zwei Antworten auf eine Frage.
    ///
    /// **Der Sprung geht durch `DateifensterQuelle::ordner_lesen`** und wird deren
    /// dritter Aufrufer neben dem Aufstieg aus C2 der Runde 1 und dem Sprung
    /// aus der Zwischenablage aus C10. Er wechselt den Ordner des **aktiven
    /// Tabs** und oeffnet keinen neuen; die Navigation dieses Programms behaelt
    /// damit ihre eine Regel.
    ///
    /// **Ob die Datei im Zielordner noch steht, wird nicht geprueft.** Der
    /// Wunschname geht an den Lesevorgang; findet der ihn nicht, bleibt die
    /// Auswahl, wo sie ohne Wunschnamen bliebe. Eine Pruefung davor waere ein
    /// zweiter Zugriff auf die Platte fuer eine Frage, die der Lesevorgang
    /// ohnehin beantwortet (C2, sechstes Kriterium).
    ///
    /// Liefert immer `true`, wie [`Self::terminal_oeffnen`]: der Befehl war
    /// zustaendig, auch wenn er nur etwas zu melden hatte.
    fn ordner_der_datei_zeigen(&self) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();
        let Some(datei) = self.angezeigte_datei() else {
            // Der Satz nennt das Ergebnis und nicht die Ursache: er stimmt
            // ebenso fuer den Nutzer, der den Editor abgeschaltet hat, wie
            // fuer den, dessen Vorschau nichts zeigt (C2, fuenftes
            // Kriterium).
            self.antwort_zeigen(
                aktiv,
                "keine angezeigte Datei, zu der gesprungen werden könnte",
            );
            return true;
        };
        // Ein Pfad ohne Elternteil ist die Wurzel selbst: der Ordner der Datei
        // `/x` ist `/`. `Path::parent` liefert dafuer `None`, und das ist kein
        // Fehler, sondern das Ende des Aufstiegs — eine Meldung waere hier
        // falsch. Der Wunschname faellt in diesem Fall weg, weil es keinen
        // Eintrag gibt, auf den die Auswahl springen koennte.
        let (ordner, auswahl) = match datei.parent() {
            Some(eltern) => (
                eltern.to_path_buf(),
                datei
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned()),
            ),
            None => (datei.clone(), None),
        };
        self.dateifenster(aktiv)
            .quelle()
            .ordner_lesen(&ordner, auswahl);
        true
    }

    /// Der Pfad der angezeigten Datei, oder `None`, wenn keine angezeigt wird
    /// (C1 und C2 der Runde 6).
    ///
    /// **Die eine Stelle, die die vier Eingaben abliest.** Welche Datei daraus
    /// folgt, entscheidet [`angezeigtedatei::welche`] ohne AppKit und damit
    /// ohne Fenster pruefbar; hier bleibt allein das Ablesen: die Sichtbarkeit
    /// der beiden Bereiche aus dem Fenstermodell, die beiden Pfade aus den
    /// Bereichen, die sie halten.
    ///
    /// **Zwei Befehle fragen sie**, der Ordnersprung aus C2 und das Teilen aus
    /// C1, sobald dessen Fokus in der Vorschau oder im Editor steht. Zwei
    /// Ablesungen nebeneinander waeren zwei Antworten auf eine Frage, und die
    /// zweite fiele erst am Buendel auf (C2, viertes Kriterium).
    fn angezeigte_datei(&self) -> Option<PathBuf> {
        let (vorschau_sichtbar, editor_sichtbar) = {
            let modell = self.ivars().modell.borrow();
            (
                modell.sichtbar(Bereich::Vorschau),
                modell.sichtbar(Bereich::Editor),
            )
        };
        let vorschau_pfad = self
            .ivars()
            .vorschau
            .get()
            .and_then(|vorschau| vorschau.angezeigter_pfad());
        let editor_pfad = self.ivars().editor.get().and_then(|editor| editor.pfad());
        angezeigtedatei::welche(
            vorschau_sichtbar,
            vorschau_pfad,
            editor_sichtbar,
            editor_pfad,
        )
    }

    /// Stellt das andere Dateifenster auf den Ordner des aktiven
    /// (C1 bis C3 der Runde 13).
    ///
    /// Wirkt in eine Richtung, vom aktiven zum anderen, und trifft dort den
    /// **sichtbaren** Tab. Sortierung, Filtertext, "Deep", Inhaltsfilter und die
    /// Anzeige ausgeblendeter Eintraege bleiben dort stehen, wie sie standen: der
    /// Befehl geht durch [`DateifensterQuelle::ordner_lesen`] und erbt damit die
    /// eine Regel des Ordnerwechsels, statt eine zweite daneben zu setzen (C3).
    ///
    /// **Hervorholen und Stellen sind zwei Handlungen und keine Kette.** Sie
    /// beantworten verschiedene Fragen, und keine von beiden folgt aus der
    /// anderen: ausgeblendet heisst hervorholen, gleich ob der Ordner schon
    /// stimmt, und ein abweichender Ordner heisst stellen, gleich ob dafuer
    /// hervorgeholt werden musste. Damit sind alle vier Lagen bestimmt, und
    /// die eine, in der nichts geschieht, ist das sichtbare Ziel auf demselben
    /// Ordner. So entschieden vom Nutzer am 260818. Davor entschied die
    /// Reihenfolge des Flussdiagramms im Spec die Frage stillschweigend
    /// zugunsten von C1: ein ausgeblendetes Dateifenster mit demselben Ordner
    /// blieb ausgeblendet, waehrend die Statuszeile ueber einen Bereich
    /// berichtete, den der Nutzer nicht sah.
    ///
    /// **Die Sichtbarkeit wird am Fenstermodell gefragt, bevor eingeblendet
    /// wird**, und nicht aus dem Rueckgabewert von [`Self::bereich_einblenden`]
    /// erschlossen: dessen `false` traegt drei Bedeutungen, und nur eine davon
    /// ist eine Abweisung. Wer beide ueber denselben Wert liest, meldet dem
    /// Nutzer ein zu schmales Fenster, wenn das andere Dateifenster laengst
    /// dasteht.
    ///
    /// **Die eine Abweisung haelt auch das Stellen an** (C2, zweites
    /// Kriterium): bleibt der Bereich ausgeblendet, weil das Fenster zu schmal
    /// ist, bleibt er auch auf seinem bisherigen Ordner. Das ist keine
    /// Rueckkehr zur Kette, sondern ihr Preis: ein Lesevorgang dorthin kostete
    /// den Zieltab Auswahl und Bildlaufposition fuer eine Anzeige, die niemand
    /// sieht.
    ///
    /// **Die beiden Ordner werden verglichen, wie sie angezeigt werden, ohne
    /// `canonicalize`.** Die Aufloesung kostete zwei Systemaufrufe je
    /// Tastendruck und braechte einen eigenen Fehlerausgang. Der Vergleich kann
    /// nur in eine Richtung irren: zwei **verschiedene** Ordner teilen nie
    /// denselben `PathBuf`, ein falsches "steht schon dort" ist damit
    /// ausgeschlossen. Was durchrutscht, ist derselbe Ordner unter zwei
    /// Schreibweisen, etwa `/tmp` gegen `/private/tmp`, ein Lesezeichen ueber
    /// einen symbolischen Verweis, oder ein Unterschied in der Gross- und
    /// Kleinschreibung auf dem hier ueblichen Datentraeger.
    ///
    /// **Sein Ausgang ist ein zweiter Lesevorgang, und der ist nicht
    /// folgenlos.** [`DateifensterQuelle::ordner_lesen`] geht durch
    /// [`Tabliste::ordner_setzen`], und das liest den stehenden Tab nicht nach,
    /// sondern ersetzt ihn: Sortierung, "Deep", Inhaltsfilter, die Anzeige
    /// ausgeblendeter Eintraege und der Filtertext gehen von Hand mit,
    /// **Auswahl und Bildlaufposition nicht**. Genau darum besteht
    /// [`Tabliste::aktiven_neu_lesen`] daneben. Der Preis ist hingenommen: er
    /// trifft einen Tab, den der Nutzer gerade nicht ansieht, und faellt nur in
    /// der Lage an, die zwei Schreibweisen eines Ordners braucht.
    ///
    /// **Die Meldung geht an das ausloesende Dateifenster und nicht an das
    /// Ziel.** So haelt es KRK bei jeder Befehlsantwort auf Rang 1: die Zeile
    /// antwortet dem, der gedrueckt hat. Eine Antwort im Zielfenster stuende
    /// gerade dort, wohin der Nutzer nicht sieht, wenn das Ziel ausgeblendet
    /// geblieben ist.
    ///
    /// **Zwei Meldungen und nicht eine**, weil "steht schon dort" seit der
    /// Trennung zwei Lagen benennt. War der Bereich sichtbar, ist nichts
    /// geschehen, und der Satz sagt das. War er ausgeblendet, steht er jetzt
    /// da; ein Satz, der allein "zeigt diesen Ordner bereits" sagte,
    /// verschwiege die eine Aenderung, die der Tastendruck bewirkt hat.
    ///
    /// Der Fokus wird nicht angefasst, [`Fenstermodell::aktiv_setzen`] nicht
    /// gerufen und kein Bereich ausgeblendet (C1, C2).
    ///
    /// **Der Rueckgabewert sagt "hat gewirkt" und nicht "war zustaendig".** So
    /// steht es im Vertrag am Kopf des `match` in
    /// [`Self::kommando_ausfuehren`]: ueber die Zustaendigkeit ist vorher
    /// entschieden, und der Wert traegt allein die zwei Nachwirkungen,
    /// [`Self::aufteilung_nachziehen`] und [`Self::sitzung_vormerken`]. Hier
    /// ist er tragend und nicht kosmetisch, denn
    /// [`Self::nach_dem_sichtbarkeitswechsel`] legt die Fensterzeile **nicht**
    /// neu aus. **Den Nachzug eines hervorgeholten Dateifensters traegt er seit
    /// dem 260823 nicht mehr allein**: [`Self::sichtbarkeit_aendern`] schreibt
    /// die geaenderte Sichtbarkeit selbst auf den Schirm, und
    /// [`Self::bereich_einblenden`] geht darueber. Was hier am Wert haengt,
    /// sind [`Self::sitzung_vormerken`] und die Anzeigen, die keine
    /// Sichtbarkeit sind. Darum `false` in den beiden Zweigen, in denen nichts
    /// geschah, dem zu schmalen Fenster und dem sichtbaren Ziel auf demselben
    /// Ordner, und `true` in den uebrigen.
    /// [`Self::ordner_der_datei_zeigen`] weicht davon ab und liefert auch auf
    /// seinem Leerweg `true`; die Abweichung ist aelter als dieser Befehl und
    /// hier nicht mitgezogen.
    fn ordner_angleichen(&self) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();
        let ziel = aktiv.andere();
        let ordner = self.dateifenster(aktiv).quelle().angezeigter_ordner();
        let dort = self.dateifenster(ziel).quelle().angezeigter_ordner();

        let bereich = Bereich::von_seite(ziel);
        // **Die Ausleihe endet mit dieser Zeile und nicht erst mit der
        // Bedingung darunter.** `bereich_einblenden` nimmt sich denselben
        // `RefCell` veraenderlich; stuende der Aufruf in derselben Bedingung
        // wie dieses `borrow()`, lebte die Ausleihe noch, und der Griff danach
        // waere der Absturz.
        let sichtbar = self.ivars().modell.borrow().sichtbar(bereich);
        if !sichtbar && !self.bereich_einblenden(bereich) {
            self.antwort_zeigen(
                aktiv,
                "das Fenster ist zu schmal; es wurde nichts eingeblendet und nichts gestellt",
            );
            return false;
        }

        if ordner == dort {
            // `!sichtbar` heisst an dieser Stelle "war ausgeblendet und steht
            // jetzt da": die Abweisung hat der Zweig darueber schon
            // abgefangen. Genau dann hat der Tastendruck gewirkt, und genau
            // dann braucht die Fensterzeile ihren Nachzug.
            self.antwort_zeigen(
                aktiv,
                if sichtbar {
                    "das andere Dateifenster zeigt diesen Ordner bereits"
                } else {
                    "das andere Dateifenster wurde eingeblendet und zeigt diesen Ordner bereits"
                },
            );
            return !sichtbar;
        }

        self.dateifenster(ziel).quelle().ordner_lesen(&ordner, None);
        true
    }

    /// Gibt die betroffenen Eintraege an die Freigabedienste des Systems
    /// (C1 der Runde 6).
    ///
    /// **Worauf der Befehl wirkt, entscheidet der Fokus, und die Verzweigung
    /// steht nicht hier.** [`teilen::worauf`] beantwortet sie als reine
    /// Rechnung ueber alle fuenf Fokuswerte und ist damit ohne Fenster
    /// pruefbar; diese Stelle verzweigt nur noch ueber die drei Werte, die
    /// dabei herauskommen, und holt zu jedem seine Pfade. Es ist **keine
    /// zweite Fokusabfrage**: der Wert kommt aus der einen in
    /// [`Self::kommando_ausfuehren`] und ist hier eine Adresse und kein
    /// Vorbehalt, wie bei [`Self::bereichskommando`] und
    /// [`Self::tab_schliessen`].
    ///
    /// **Die Eintraege kommen aus [`operationen::betroffene`]** und aus keiner
    /// zweiten Auswahlregel; Teilen wird deren siebter Abnehmer (C1, drittes
    /// Kriterium). Ordner gehen mit, und der Typ wird nicht geprueft.
    ///
    /// **Der Anker ist die Ansicht des Bereichs, der den Fokus hat, und ihr
    /// `bounds`.** Eine Zeile der Liste oder die Schreibmarke im Text zu nehmen
    /// waere eine zweite Regel je Bereich; die eine Regel ist statthaft, und
    /// wie sie aussieht, ist am Buendel zu beurteilen. Die Ansicht kommt aus
    /// [`Self::fokusansicht`], derselben Zuordnung, die den Ersthelfer setzt —
    /// fuer [`Fokus::Anderswo`] aus der des Dateifensters, denn dieser Wert
    /// haengt an keiner Ansicht und der Befehl nimmt dort dieselbe Menge wie
    /// im Dateifenster.
    ///
    /// **Bleibt der Dialog aus, sagt es die Statuszeile** (C1, fuenftes
    /// Kriterium). Der Satz nennt das Ergebnis und keine Ursache und stimmt
    /// deshalb auch fuer die Lage, in der der Bereich noch gar nicht gebaut
    /// ist.
    ///
    /// Liefert immer `true`, wie [`Self::ordner_der_datei_zeigen`]: der Befehl
    /// war zustaendig, auch wenn er nur etwas zu melden hatte.
    fn teilen(&self, fokus: Fokus) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();
        let (pfade, anker) = match teilen::worauf(fokus) {
            teilen::Quelle::BetroffeneEintraege => (
                self.dateifenster(aktiv)
                    .quelle()
                    .betroffene_eintraege()
                    .pfade,
                self.fokusansicht(Fokus::Dateifenster),
            ),
            teilen::Quelle::AngezeigteDatei => (
                self.angezeigte_datei().into_iter().collect(),
                self.fokusansicht(fokus),
            ),
            teilen::Quelle::Nichts => (Vec::new(), None),
        };
        let gezeigt = match anker {
            Some(flaeche) => teilen::anbieten(&pfade, flaeche, flaeche.bounds()),
            None => false,
        };
        if !gezeigt {
            self.antwort_zeigen(aktiv, &operationen::nichts_zu_teilen());
        }
        true
    }

    // ------------------------------------------------------------------
    // Die Belegungsansicht (C3)
    // ------------------------------------------------------------------

    /// Zeigt die Belegungsansicht als Blatt am Fenster (C3, F1).
    ///
    /// Die Ansicht arbeitet auf einer Kopie der geltenden Belegung;
    /// uebernommen wird sie erst beim Verlassen, in
    /// [`Self::belegungsansicht_verlassen`]. Der Blattgriff geht nach
    /// `offenes_blatt`, damit der Abbruchbefehl auf `esc` das Blatt schliesst
    /// wie jede Rueckfrage.
    fn belegung_ansehen(&self) -> bool {
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };
        let modell = Belegungsmodell::neu(self.ivars().belegung.borrow().clone());
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let (quelle, griff) = belegungsansicht::zeigen(self.mtm(), fenster, modell, move || {
            if let Some(selbst) = schwach.load() {
                selbst.belegungsansicht_verlassen();
            }
        });
        *self.ivars().belegungsansicht.borrow_mut() = Some(quelle);
        *self.ivars().offenes_blatt.borrow_mut() = Some(griff);
        true
    }

    /// Das Blatt der Belegungsansicht ist zu: sichern und nachziehen (C3).
    ///
    /// Ohne Aenderung geschieht nichts, und `keymap.toml` bleibt unberuehrt.
    /// Mit Aenderung wird die Arbeitskopie gesichert und zur geltenden
    /// Belegung; Hauptmenue und Ereignisabgriff werden auf sie neu aufgebaut,
    /// damit die Umbelegung sofort wirkt und nicht erst nach einem Neustart.
    /// Das ist derselbe Aufbauweg wie beim Start: eine Quelle, zwei Abnehmer.
    fn belegungsansicht_verlassen(&self) {
        *self.ivars().offenes_blatt.borrow_mut() = None;
        let Some(quelle) = self.ivars().belegungsansicht.borrow_mut().take() else {
            return;
        };
        let modell = quelle.modell_abgeben();
        if !modell.geaendert() {
            return;
        }
        let belegung = modell.in_belegung();
        // Das Sichern scheitert nicht still: eine Belegung, die der Nutzer
        // gesetzt hat und die den Neustart doch nicht ueberlebt, waere die
        // Sorte Fehler, die erst Tage spaeter auffaellt.
        let meldung = match self.unter_der_sperre(|zugang| belegung.sichern(zugang)) {
            Ok(Ok(())) => None,
            Ok(Err(fehler)) => Some(format!(
                "die Belegung gilt, liess sich aber nicht sichern: {fehler}"
            )),
            Err(Sperrhindernis::OhneOrdner) => Some(
                "die Belegung gilt, ist aber ohne Ablageordner nicht gesichert und geht mit dem Beenden verloren"
                    .to_owned(),
            ),
            Err(Sperrhindernis::Gesperrt(fehler)) => Some(format!(
                "die Belegung gilt, ist aber nicht gesichert: die Schreibsperre der Ablage \
                 laesst sich nicht nehmen ({fehler})"
            )),
        };
        *self.ivars().belegung.borrow_mut() = belegung;

        // Menue und Abgriff auf die neue Belegung, ueber dieselben Wege wie
        // beim Start.
        let hauptmenue = menue::hauptmenue(self.mtm(), &self.ivars().belegung.borrow());
        NSApplication::sharedApplication(self.mtm()).setMainMenu(Some(&hauptmenue));
        self.tastenabgriff_nachziehen();

        if let Some(meldung) = meldung {
            let aktiv = self.ivars().modell.borrow().aktiv();
            self.dateifenster(aktiv).quelle().meldung_zeigen(&meldung);
        }
    }

    // ------------------------------------------------------------------
    // Der Notizzettel (C1 bis C3 der Runde 9)
    // ------------------------------------------------------------------

    /// Zeigt den Notizzettel als Blatt am Hauptfenster (C1).
    ///
    /// **Gelesen wird bei jedem Oeffnen frisch**, und C4 sagt es zu: die
    /// Zetteldateien werden beim Start **nicht** gelesen, sondern erst hier.
    /// Damit sieht der Nutzer, was eine zweite Instanz von KRK inzwischen
    /// geschrieben hat, ohne dass eine dritte Absprache ueber dem Ablageordner
    /// entstuende.
    ///
    /// **Was das Gelesene wird, entscheidet das Modell und nicht diese
    /// Stelle.** Haelt der Zettel einen Text, der noch nicht auf der Platte
    /// steht, so bleibt dieser stehen und das Gelesene wird verworfen; C4 sagt
    /// seit dem 260814-0925 beides zu. Die Zusage aus dem Absatz darueber gilt
    /// deshalb fuer den gewoehnlichen Fall und nicht fuer den abweichenden
    /// Zettel — [`Zettelmodell::oeffnen`](crate::zettelmodell::Zettelmodell::oeffnen)
    /// traegt die Regel und liefert den Text der Flaeche.
    ///
    /// **Der Blattgriff geht in [`Self::offenes_blatt`]** wie der jedes anderen
    /// Blattes. Damit schliesst der Abbruchbefehl den Zettel auf demselben Weg
    /// wie jede Rueckfrage, und es entsteht kein zweiter Weg zum Schliessen.
    ///
    /// Liefert `true`, sobald das Blatt steht: der Tastendruck ist dann
    /// verbraucht.
    fn notizzettel_zeigen(&self) -> bool {
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };
        let offener = self.ivars().zettel.borrow().offener();
        let gelesen = self.zettel_lesen(offener);
        // **Der Text der Flaeche kommt aus dem Modell und nicht aus der
        // Datei.** Haelt der Zettel etwas Ungesichertes, verwirft das Modell
        // das Gelesene und gibt den gehaltenen Stand heraus; wer hier `gelesen`
        // naehme, loeschte genau den Text, den eine gescheiterte Sicherung
        // stehen lassen sollte.
        let text = self
            .ivars()
            .zettel
            .borrow_mut()
            .oeffnen(offener, gelesen)
            .to_owned();

        let beim_tabklick = objc2::rc::Weak::from_retained(&self.retain());
        let beim_abschluss = objc2::rc::Weak::from_retained(&self.retain());
        let (flaeche, griff) = zettel::zeigen(
            self.mtm(),
            fenster,
            offener,
            &text,
            move |ziel| {
                beim_tabklick
                    .load()
                    .and_then(|selbst| selbst.zettel_wechseln(ziel))
            },
            move || {
                if let Some(selbst) = beim_abschluss.load() {
                    selbst.zettel_blatt_geschlossen();
                }
            },
        );
        *self.ivars().zettelflaeche.borrow_mut() = Some(flaeche);
        *self.ivars().offenes_blatt.borrow_mut() = Some(griff);
        true
    }

    /// Liest die Datei eines Zettels und stellt eine etwaige Meldung in die
    /// Statuszeile (C5).
    ///
    /// **Der Zettel kommt in jedem Fall**, notfalls leer. Eine fehlende Datei
    /// ist der erste Start und keine Meldung wert; eine unlesbare wird
    /// beiseitegelegt, und der Nutzer erfaehrt es ueber denselben Meldeweg, den
    /// [`Ersetzung`](krk_core::ablage::Ersetzung) heute fuer `keymap.toml` und
    /// `settings.toml` geht. Was der Kern dazu formuliert, wird hier nicht noch
    /// einmal formuliert: [`Geladen::mit_meldung`](krk_core::ablage::Geladen)
    /// liefert den Satz.
    ///
    /// **Ohne Ablageordner gibt es einen leeren Zettel und eine Meldung.** Still
    /// einen leeren zu zeigen waere die schlechtere Antwort: der naechste
    /// Sicherungsmoment schriebe ihn nirgendwohin, und der Nutzer erfuehre nie,
    /// dass sein Zettel nicht gehalten wird.
    fn zettel_lesen(&self, welcher: pfade::Zettel) -> String {
        let datei = Datei::Zettel(welcher);
        let (text, meldung) = match self.unter_der_sperre(|zugang| zugang.text_laden(datei)) {
            Ok(geladen) => geladen.mit_meldung(),
            Err(Sperrhindernis::OhneOrdner) => (
                String::new(),
                Some("der Notizzettel steht ohne Ablageordner und wird nicht gesichert".to_owned()),
            ),
            Err(Sperrhindernis::Gesperrt(fehler)) => (
                String::new(),
                Some(format!(
                    "der Notizzettel ist nicht lesbar: die Schreibsperre der Ablage laesst sich \
                     nicht nehmen ({fehler})"
                )),
            ),
        };
        if let Some(meldung) = meldung {
            let aktiv = self.ivars().modell.borrow().aktiv();
            self.antwort_zeigen(aktiv, &meldung);
        }
        text
    }

    /// Der Nutzer hat einen Tab des Zettels angeklickt (C2).
    ///
    /// Liefert den Text des Ziels, oder `None`, wenn der Klick dem bereits
    /// offenen Tab galt — dann bleibt die Flaeche unberuehrt, und geschrieben
    /// wird nichts.
    ///
    /// **Zuerst der Stand der Flaeche ins Modell, dann erst die Entscheidung.**
    /// Was der Nutzer getippt hat, steht bis zu dieser Zeile allein in der
    /// `NSTextView`; wer den Wechsel vor der Uebernahme entschiede, entschiede
    /// ihn auf dem Stand von vorhin.
    fn zettel_wechseln(&self, ziel: pfade::Zettel) -> Option<String> {
        // **Vor dem Wechsel und nicht darin.** Danach waere der offene Zettel
        // schon das Ziel, und der Stand der Flaeche ginge in den falschen von
        // beiden. Aus demselben Grund uebernimmt `zettel_sichern` den Stand
        // nicht selbst: es weiss nicht, ob eben gewechselt wurde.
        self.zettel_stand_uebernehmen();
        // Der Wert steht in einer eigenen Zeile und nicht im Kopf des `match`:
        // die Ausleihe des Kopfes lebte sonst durch alle Zweige, und der Zweig
        // darunter fragt das Modell erneut.
        let wechsel = self.ivars().zettel.borrow_mut().wechseln(ziel);
        match wechsel {
            // Der Klick auf den offenen Tab: nichts zu tun und nichts zu
            // schreiben. C2 sagt es ausdruecklich zu.
            zettelmodell::Wechsel::Derselbe => return None,
            zettelmodell::Wechsel::GewechseltUngeaendert => {}
            // **Der erste der vier Sicherungsmomente aus C4.** Er steht hier,
            // weil der Tabklick hier ankommt; die drei uebrigen kommen
            // anderswo an. Was Sichern heisst, steht an keiner der vier Stellen,
            // sondern einmal in `zettel_sichern`.
            //
            // Der eigene Durchgang ist der erste von drei: allein der vierte
            // Moment, das Beenden, findet den Schreibgriff schon genommen vor.
            zettelmodell::Wechsel::GewechseltZuSichern => {
                let ergebnis = self.unter_der_sperre(|zugang| self.zettel_sichern(zugang));
                self.zettel_sicherung_melden(ergebnis);
            }
        }
        // **Die Flaeche bekommt den gehaltenen Stand des Ziels.** Weicht der
        // Zielzettel von seiner Datei ab, verwirft das Modell das eben
        // Gelesene; ein Tabwechsel, der `gelesen` in die Flaeche setzte, waere
        // derselbe Verlust wie ein Neuoeffnen und stand als eigener Weg im
        // Datensatz `260814-0908`.
        let gelesen = self.zettel_lesen(ziel);
        let text = self
            .ivars()
            .zettel
            .borrow_mut()
            .oeffnen(ziel, gelesen)
            .to_owned();
        Some(text)
    }

    /// Was Sichern fuer den Notizzettel heisst — die eine Erklaerung dafuer
    /// (C4).
    ///
    /// Liefert den Satz fuer die Statuszeile, falls ein Schreibvorgang
    /// scheiterte, und `None`, wenn geschrieben wurde oder nichts zu schreiben
    /// war.
    ///
    /// # Geschrieben wird jeder abweichende Zettel
    ///
    /// Nicht der erste, sondern jeder: C4 sagt es seit dem 260814-0925 zu, und
    /// die Schleife kostet nichts, weil ein unveraenderter Zettel ohnehin nicht
    /// geschrieben wird. Der Anlass steht in `issues/260814-0909_*`. Zwei
    /// zugleich abweichende Zettel entstehen aus einer gescheiterten Sicherung:
    /// der eine bleibt abweichend stehen, der Nutzer bearbeitet inzwischen den
    /// anderen. Beim vierten Moment ist das keine Frage der Bequemlichkeit
    /// mehr — nach `applicationWillTerminate:` gibt es kein naechstes Mal, das
    /// den zweiten Zettel nachholte.
    ///
    /// # Die vier Momente, an denen gesichert wird
    ///
    /// Vier Aufrufer sprechen diese Stelle an, und jeder ist ein Weg **aus dem
    /// Zettel heraus**. Das ist der Zuschnitt aus C4, und er steht hier
    /// aufgezaehlt, weil er sonst an vier Stellen halb erklaert waere:
    ///
    /// 1. **Der Tabklick** ([`Self::zettel_wechseln`]): der verlassene Zettel
    ///    verschwindet von der Flaeche, und niemand koennte ihn danach noch
    ///    aus ihr lesen.
    /// 2. **Das Schliessen des Blattes** ([`Self::zettel_blatt_geschlossen`]):
    ///    die Escape-Taste, die Schaltflaeche und der Abbruchbefehl ueber den
    ///    Blattgriff muenden alle drei in denselben Abschlussblock, und mit ihm
    ///    fallen Blatt und Flaeche.
    /// 3. **`shift+cmd+w`** ([`Self::fenster_schliessen`]): das Fenster geht zu,
    ///    und das Blatt haengt daran. Gesichert wird dort **vor**
    ///    `performClose:` und ohne Bedingung; die Begruendung steht an jener
    ///    Stelle.
    /// 4. **Das Beenden von KRK** (`applicationWillTerminate:`): der Prozess
    ///    endet. Dieser Aufrufer bringt seinen [`Zugang`] mit, statt einen
    ///    zweiten Durchgang durch die Ablage zu nehmen — der Grund steht dort.
    ///
    /// # Was durchkommt und trotzdem kein Moment ist
    ///
    /// [`Kommando::FensterEinblenden`] fuehrt **nicht** aus dem Zettel heraus.
    /// Es holt dasselbe Fenster nach vorn, an dem das Blatt haengt; der Zettel
    /// steht danach unveraendert da, und ein Schreibvorgang waere ein Schreiben
    /// ohne Anlass. Eine Probe unter `mod tests` haelt diese Gegenrichtung fest.
    ///
    /// # Was diese Stelle nicht tut
    ///
    /// **Sie liest die Textflaeche nicht.** Was in ihr steht, kommt ueber
    /// [`Self::zettel_stand_uebernehmen`] in das Modell, und das ruft jeder der
    /// vier Momente vorher; beim Tabklick muss es sogar vor dem Wechsel
    /// geschehen, und deshalb steht es nicht hier.
    ///
    /// **Ohne Aenderung geschieht nichts**, und das entscheidet das Modell und
    /// nicht diese Stelle: [`Zettelmodell::zu_sichern`] liefert `None`, solange
    /// der gehaltene Stand der gelesene ist.
    ///
    /// **Eine gescheiterte Sicherung wirft den Stand nicht weg.**
    /// [`Zettelmodell::gesichert`] wird dann gerade **nicht** gerufen: der
    /// Zettel bleibt abweichend, und der naechste Moment versucht es erneut.
    /// Der Grund geht in die Statuszeile, damit der Nutzer nicht darauf baut,
    /// dass sein Text auf der Platte liegt. Die zweite Haelfte dieser Zusage
    /// steht am Modell und nicht hier: seit dem Nachtrag zu C4 setzt auch das
    /// Oeffnen den gehaltenen Text eines abweichenden Zettels nicht mehr
    /// zurueck.
    fn zettel_sichern(&self, zugang: &Zugang<'_>) -> Option<String> {
        // **Erst sammeln, dann schreiben, und die Texte dabei kopieren.** Das
        // Schreiben unten braucht das Modell veraenderlich, um `gesichert` zu
        // melden; eine noch laufende Ausleihe der Staende liesse das nicht zu.
        let abweichende: Vec<(pfade::Zettel, String)> = self
            .ivars()
            .zettel
            .borrow()
            .zu_sichern()
            .map(|(welcher, text)| (welcher, text.to_owned()))
            .collect();
        let mut meldung = None;
        for (welcher, text) in abweichende {
            match zugang.text_sichern(Datei::Zettel(welcher), &text) {
                Ok(()) => self.ivars().zettel.borrow_mut().gesichert(welcher),
                // **Der erste Fehlschlag steht in der Statuszeile**, und ein
                // zweiter verdraengt ihn nicht: die Zeile traegt einen Satz,
                // und scheitern beide Zettel, so scheitern sie am selben
                // Hindernis — kein Ablageordner, kein Schreibrecht, die Sperre
                // nicht zu nehmen. **Abgebrochen wird deshalb nicht:** der
                // zweite Zettel bekommt seinen Versuch, denn der Fehlschlag des
                // ersten sagt ueber ihn nichts.
                Err(fehler) => {
                    meldung.get_or_insert_with(|| {
                        format!("der Notizzettel liess sich nicht sichern: {fehler}")
                    });
                }
            }
        }
        meldung
    }

    /// Nimmt in das Zettelmodell auf, was gerade in der Textflaeche steht.
    ///
    /// **Die Flaeche ist die einzige Stelle, an der das Getippte steht**, und
    /// sie ist mit dem Blatt fort. Jeder der vier Sicherungsmomente ruft dies
    /// deshalb, bevor er sichert; steht kein Zettel, geschieht nichts.
    ///
    /// Der Rueckgabewert von [`Zettelmodell::bearbeiten`] wird hier nicht
    /// gebraucht: er sagt, ob es etwas zu sichern gibt, und genau das
    /// beantwortet [`Zettelmodell::zu_sichern`] gleich danach noch einmal und
    /// nennt dazu den Zettel.
    fn zettel_stand_uebernehmen(&self) {
        let Some(stand) = self.zettelstand() else {
            return;
        };
        let _ = self.ivars().zettel.borrow_mut().bearbeiten(stand);
    }

    /// Stellt in die Statuszeile, was an einer Sicherung des Zettels
    /// scheiterte.
    ///
    /// Der Weg der drei Momente mit eigenem Durchgang. Das Beenden ruft dies
    /// nicht: dort gibt es keine Statuszeile mehr, an der ein Satz ankaeme.
    ///
    /// **Ein Hindernis wird nur gemeldet, wenn wirklich etwas ungesichert
    /// ist.** Ohne Ablageordner scheitert jeder Durchgang, und ein Zettel, der
    /// seiner Datei gleicht, hatte nichts zu schreiben; ein Satz darueber waere
    /// eine Meldung ueber ein Nichtereignis, und der Tabklick trueg sie bei
    /// jedem Klick vor. Der Start hat den fehlenden Ordner ohnehin einmal
    /// gemeldet.
    fn zettel_sicherung_melden(&self, ergebnis: Result<Option<String>, Sperrhindernis>) {
        let meldung = match ergebnis {
            Ok(meldung) => meldung,
            Err(_) if !self.ivars().zettel.borrow().etwas_zu_sichern() => None,
            Err(Sperrhindernis::OhneOrdner) => {
                Some("der Notizzettel ist ohne Ablageordner nicht gesichert".to_owned())
            }
            Err(Sperrhindernis::Gesperrt(fehler)) => Some(format!(
                "der Notizzettel ist nicht gesichert: die Schreibsperre der Ablage laesst sich \
                 nicht nehmen ({fehler})"
            )),
        };
        if let Some(meldung) = meldung {
            let aktiv = self.ivars().modell.borrow().aktiv();
            self.antwort_zeigen(aktiv, &meldung);
        }
    }

    /// Das Blatt des Zettels ist zu (C1).
    ///
    /// **Der Stand kommt noch ins Modell, bevor die Flaeche faellt.** Sie ist
    /// die einzige Stelle, an der das Getippte steht, und mit dem Blatt ist sie
    /// fort; ein Modell, das erst danach gefragt wuerde, saehe den Stand von
    /// vor dem letzten Zeichen.
    ///
    /// **Alle drei Wege heraus kommen hier an** — die Escape-Taste ueber den
    /// Waechter, die Schaltflaeche und der Abbruchbefehl ueber den Blattgriff —,
    /// weil sie in denselben Abschlussblock von AppKit muenden. Deshalb haengt
    /// das Sichern an dieser Stelle und nicht am Waechter: **ein** Aufrufer fuer
    /// drei Wege und nicht drei.
    ///
    /// Der zweite der vier Sicherungsmomente aus C4; was Sichern heisst, steht
    /// in [`Self::zettel_sichern`].
    fn zettel_blatt_geschlossen(&self) {
        self.zettel_stand_uebernehmen();
        *self.ivars().zettelflaeche.borrow_mut() = None;
        *self.ivars().offenes_blatt.borrow_mut() = None;
        // **Nach dem Abraeumen und nicht davor.** Das Schreiben laeuft durch die
        // Ablage; bliebe die Flaeche bis dahin eingetragen, saehe ein Weg, der in
        // dieser Spanne hierher zurueckkaeme, ein Blatt, das es nicht mehr gibt.
        // Der Stand steht zu diesem Zeitpunkt im Modell, und nur von dort liest
        // das Sichern.
        let ergebnis = self.unter_der_sperre(|zugang| self.zettel_sichern(zugang));
        self.zettel_sicherung_melden(ergebnis);
    }

    /// Was gerade in der Textflaeche des Zettels steht, falls einer steht.
    ///
    /// `None` heisst: es steht kein Zettel. Ein leerer Text waere die falsche
    /// Antwort darauf — er hiesse „der Nutzer hat alles geloescht" und
    /// ueberschriebe beim naechsten Sicherungsmoment die Datei.
    fn zettelstand(&self) -> Option<String> {
        self.ivars()
            .zettelflaeche
            .borrow()
            .as_ref()
            .map(|flaeche| flaeche.string().to_string())
    }

    /// Schreibt die geltende Tastenbelegung als Markdown in den
    /// Downloads-Ordner und meldet das Ergebnis (Runde 3).
    ///
    /// **Hier steht keine Blattabfrage, und das ist Absicht.** `blatt_steht`
    /// gilt fuer Kommandos, die der Delegierte aus einem Tastendruck erhaelt;
    /// dieser Eintrag ist keines. C1 verlangt ausdruecklich, dass er auch bei
    /// stehender Belegungsansicht wirkt und dann den **gesicherten** Stand
    /// schreibt. Wer ihn aus Gruenden der Gleichfoermigkeit an `blatt_steht`
    /// haengte, braeche das Kriterium.
    ///
    /// Die Belegung kommt aus den Ivars, also aus dem Wert, der im Betrieb
    /// gilt. Die offene Belegungsansicht arbeitet auf einer Kopie und beruehrt
    /// ihn bis zum Verlassen nicht; der gesicherte Stand faellt damit ohne
    /// einen einzigen Zweig an. Ein Aufruf von `belegung::fuer_den_betrieb()`
    /// an dieser Stelle waere ein zweiter Ladeweg und in einem Fall
    /// nachweislich falsch, siehe den Modulkopf von
    /// [`crate::belegungsausgabe`].
    ///
    /// Die Meldung geht ueber [`Self::antwort_zeigen`] an das **aktive**
    /// Dateifenster. Das ist keine Frage nach dem Fokus, sondern ein Wert des
    /// Fenstermodells, und es liegt immer auf einem sichtbaren Bereich —
    /// derselbe Weg, den [`Self::belegungsansicht_verlassen`] fuer sein
    /// gescheitertes Sichern geht.
    fn tastenbelegung_sichern(&self) {
        let ausgang = belegungsausgabe::ausgeben(&self.ivars().belegung.borrow());
        let aktiv = self.ivars().modell.borrow().aktiv();
        self.antwort_zeigen(aktiv, &ausgang.meldung());
    }

    /// Blendet einen Bereich aus oder wieder ein (C7).
    ///
    /// Das [`Zeilenmass`] kommt aus der Aufteilung; warum das Fenstermodell es
    /// braucht, steht bei [`Self::zeilenmass`].
    fn bereich_umschalten(&self, bereich: Bereich) -> bool {
        let Some(mass) = self.zeilenmass() else {
            return false;
        };
        self.sichtbarkeit_aendern(|modell| modell.umschalten(bereich, mass))
    }

    /// Blendet eine Spalte beider Dateilisten aus oder wieder ein (C3 der
    /// Bereichsleisten-Runde).
    ///
    /// **Die eine Stelle, durch die alle drei Spaltenbefehle gehen.** Der Klick
    /// auf einen Schalter der Bereichsleiste geht durch dasselbe Kommando und
    /// damit durch dieselbe Zeile; einen zweiten Weg an der Abweisung des
    /// Modells vorbei gibt es nicht.
    ///
    /// **Kein [`Zeilenmass`] und kein `aufteilung_nachziehen` von hier aus.**
    /// Eine Spalte liegt in der Dateiliste und nicht in der Fensterzeile, die
    /// Breiten der fuenf Bereiche stehen vorher und nachher gleich (Kriterium
    /// C3.4). Den Nachzug der Aufteilung ruft [`Self::kommando_ausfuehren`]
    /// ohnehin fuer jedes ausgefuehrte Kommando; er findet dann eine
    /// unveraenderte Sichtbarkeit vor.
    fn spalte_umschalten(&self, spalte: Spalte) -> bool {
        if !self.ivars().modell.borrow_mut().spalte_umschalten(spalte) {
            return false;
        }
        self.spaltenanzeige_nachziehen();
        true
    }

    /// Schreibt die Sichtbarkeit der Spalten in beide Dateilisten und verteilt
    /// danach ihre Breiten neu (C3 der Bereichsleisten-Runde).
    ///
    /// **Der eine Schreiber, mit zwei Anlaessen**, nach dem Vorbild von
    /// [`Self::fokusanzeige_nachziehen`]: der Aufbau der Oberflaeche, damit die
    /// geladene Sitzung ankommt, und [`Self::spalte_umschalten`] fuer jede
    /// spaetere Aenderung. Das Modell ist die Quelle, die Anzeige folgt ihm.
    ///
    /// **Sie schreibt alle vier Spalten und nicht nur die geaenderte**, obwohl
    /// die Namensspalte nie verborgen wird. Der Durchgang laeuft ueber
    /// [`Spalte::ALLE`], damit der Aufbau und der Schalter dieselbe Zeile
    /// nehmen; eine Liste der drei schaltbaren daneben waere eine zweite
    /// Aufzaehlung, und [`spalte_sichtbar_in`] beantwortet die Namensspalte
    /// ohnehin mit `true`.
    ///
    /// **Die Breiten stehen erst nach dem zweiten Aufruf richtig.**
    /// `setHidden:` allein schlaegt die frei werdenden Punkte der Namensspalte
    /// zu und laesst die Tabelle so breit, wie sie war; der Gewinn erreicht die
    /// Sichtflaeche nie, und eine Tabelle, die vorher schon breiter war als ihr
    /// Bildlauf, bleibt es. Was [`Dateifenster::spaltenbreiten_verteilen`]
    /// dagegen setzt, welche Regel der Nutzer dafuer gewaehlt hat und woran das
    /// gemessen ist, steht dort.
    fn spaltenanzeige_nachziehen(&self) {
        if self.ivars().dateifenster.get().is_none() {
            return;
        }
        let spalten = self.ivars().modell.borrow().spaltensichtbarkeit();
        for seite in Fensterseite::ALLE {
            for spalte in Spalte::ALLE {
                self.dateifenster(seite)
                    .spalte_verbergen(spalte, !spalte_sichtbar_in(&spalten, spalte));
            }
            // **Einmal je Dateifenster und nicht einmal je Spalte.** Die
            // Verteilung misst den rechten Rand der letzten sichtbaren Spalte;
            // mitten im Durchgang darueber gemessen waere er der Rand eines
            // Zwischenstandes, und die vier Rechnungen ueberschrieben einander.
            self.dateifenster(seite).spaltenbreiten_verteilen();
        }
    }

    /// Holt einen ausgeblendeten Bereich hervor und blendet nie einen aus.
    ///
    /// Der Weg der Befehle, die einen Bereich **brauchen** statt ihn
    /// umzuschalten: `shift+f3` aus C10 und die Fokusbefehle seit dem
    /// Nutzerentscheid vom 260807. Die Regel selbst steht in
    /// [`Fenstermodell::einblenden`] und damit ausserhalb von AppKit; hier
    /// kommen allein die Nachzuege dazu, die jeder Sichtbarkeitswechsel
    /// braucht.
    ///
    /// **`false` traegt hier drei Bedeutungen, das Modell darunter nur zwei.**
    /// Der Rueckgabewert haelt sie nicht auseinander, deshalb stehen sie
    /// einzeln da:
    ///
    /// 1. Der Bereich stand schon da; [`Fenstermodell::einblenden`] weist ihn
    ///    deshalb ab. Es war nichts zu tun, und die gewuenschte Sichtbarkeit
    ///    besteht hinterher genauso wie vorher.
    /// 2. **Die Mindestbreiten passen nicht**, geerbt von
    ///    [`Fenstermodell::umschalten`]. **Das ist die eine Abweisung unter den
    ///    dreien**: der Bereich bleibt ausgeblendet, und das Fenster ist zu
    ///    schmal, um ihn aufzunehmen. Wer dem Nutzer etwas zu sagen hat, sagt
    ///    es zu dieser Lage.
    /// 3. [`Self::zeilenmass`] liefert `None`, die Aufteilung steht also noch
    ///    nicht. Diese dritte legt allein der Mantel ueber die zwei des
    ///    Modells. Fuer einen Tastenbefehl kann sie nicht eintreten, weil die
    ///    Aufteilung seit `oberflaeche_aufbauen` steht und vorher kein
    ///    Tastendruck den Delegierten erreicht; sie steht hier trotzdem, statt
    ///    uebergangen zu werden.
    #[must_use = "eine Abweisung bleibt stumm; wer sie nicht liest, haelt einen Bereich fuer hervorgeholt, den das Modell nicht eingeblendet hat oder den die Aufteilung noch gar nicht aufnehmen kann"]
    fn bereich_einblenden(&self, bereich: Bereich) -> bool {
        let Some(mass) = self.zeilenmass() else {
            return false;
        };
        self.sichtbarkeit_aendern(|modell| modell.einblenden(bereich, mass))
    }

    /// Das Mass der Fensterzeile, falls sie schon steht.
    ///
    /// **Die eine Stelle, an der die Geometrie der Zeile aus AppKit an das
    /// Fenstermodell geht.** Drei Aufrufe brauchen sie, weil ihre Antwort an
    /// der Fensterbreite haengt und das Fenstermodell sie nicht selbst erfragen
    /// kann: [`Self::bereich_umschalten`], [`Self::bereich_einblenden`] und
    /// [`Self::breite_aendern`].
    ///
    /// `None` heisst: die Aufteilung steht noch nicht, also laeuft der Aufbau.
    /// Dann geschieht nichts — dieselbe Antwort, die
    /// [`Self::aufteilung_nachziehen`] und
    /// [`Self::bildschirmbreiten_uebernehmen`] in dieser Lage geben.
    fn zeilenmass(&self) -> Option<Zeilenmass> {
        self.ivars()
            .aufteilung
            .get()
            .map(super::aufteilung::Aufteilung::zeilenmass)
    }

    /// Fuehrt eine Aenderung der Sichtbarkeit aus und zieht fuer **jeden**
    /// Bereich nach, dessen Sichtbarkeit sich dabei geaendert hat.
    ///
    /// **Ein Aufruf kann zwei Bereiche bewegen**, seit der gegenseitige
    /// Ausschluss aus C1 der Editor-Runde in
    /// [`Fenstermodell::umschalten`](crate::fenstermodell::Fenstermodell::umschalten)
    /// steht: wer den Editor einblendet, blendet damit die Vorschau aus. Der
    /// Bereich, den der Aufrufer genannt hat, sagt darueber nichts mehr; ihm
    /// den Nachzug allein zu geben, liesse den Fokus in einer Vorschau stehen,
    /// die niemand mehr sieht.
    ///
    /// Gefragt wird deshalb nicht der Name des Aufrufs, sondern die
    /// Sichtbarkeit vorher gegen die nachher. Damit bleibt der Ausschluss
    /// vollstaendig im Fenstermodell, und diese Datei kennt ihn nicht: sie
    /// erfaehrt sein Ergebnis, statt seine Regel ein zweites Mal zu tragen.
    ///
    /// **Wer die Sichtbarkeit im Modell aendert, schreibt sie auch auf den
    /// Schirm, und zwar hier.** Bis zum 260823 tat das allein
    /// [`Self::kommando_ausfuehren`] am Ende jedes ausgefuehrten Befehls; ein
    /// eingeblendeter Bereich bekam seinen Auslegungsdurchgang nur ueber jenen
    /// Weg, und der Kommentar an `a6b3818` sagte es ausdruecklich zu. **Die
    /// Zusage traegt nicht mehr, seit `784840c` das Lesen des Editors auf einen
    /// Arbeitsfaden gelegt hat**: [`Self::editorausgang_behandeln`] kommt aus
    /// dem Einzugstakt des Editorbereichs und laeuft, wenn der Befehl, der das
    /// Oeffnen angefordert hat, laengst zurueck ist. Der Editor stand danach im
    /// Fenstermodell und nicht auf dem Schirm, und der Nutzer sah weder die
    /// Flaeche noch das Kaestchen der Bereichsleiste umspringen
    /// (`shared/issues/260820-1034_*_f4-setzt-den-fokus-nur-dann-in-den-editor-*`
    /// und `260820-1034_*_cmd-e-bleibt-in-der-vorschau-wirkungslos-*`).
    /// [`Self::anlass_ausfuehren`] traegt denselben Nachzug seit dem 260810 und
    /// aus demselben Grund, dort aber von Hand am Ende der Fortsetzung. **Das
    /// war die erste Antwort auf dieselbe Klasse, an einer einzelnen Stelle**,
    /// und ihre Unvollstaendigkeit ist der Grund, aus dem der Befund hier ein
    /// zweites Mal auflaufen musste; hier steht der Nachzug an der Stelle, die
    /// das Modell aendert, und keine kuenftige Fortsetzung kann ihn vergessen.
    ///
    /// **Er steht vor den Nachzuegen der einzelnen Bereiche und nicht hinter
    /// ihnen.** [`Self::nach_dem_sichtbarkeitswechsel`] setzt den Fokus, und
    /// [`Self::fokus_holen`] setzt ihn gleich danach ein zweites Mal; beide
    /// rufen dafuer `makeFirstResponder:`, und der trifft sonst eine Ansicht,
    /// die AppKit noch als ausgeblendet fuehrt. Das ist dieselbe Trennung, die
    /// `a6b3818` fuer das Angleichen gezogen hat: einblenden und das Zweite
    /// sind zwei Handlungen, und die Flaeche steht zuerst.
    ///
    /// **Der Ruf am Ende von [`Self::kommando_ausfuehren`] bleibt daneben
    /// stehen.** Er deckt die Aenderungen ab, die keine Sichtbarkeit sind — die
    /// Breiten und das aktive Dateifenster —, und trifft nach einem
    /// Sichtbarkeitswechsel auf ein unveraendertes Modell. Ein Befehl, der
    /// einen Bereich umschaltet, legt die Zeile damit zweimal aus; das ist der
    /// Preis dafuer, dass die Zusage an der Quelle haengt statt an der
    /// Vollstaendigkeit einer Aufrufliste.
    fn sichtbarkeit_aendern(&self, aendern: impl FnOnce(&mut Fenstermodell) -> bool) -> bool {
        let vorher = self.ivars().modell.borrow().sichtbarkeit();
        let geaendert = aendern(&mut self.ivars().modell.borrow_mut());
        if !geaendert {
            return false;
        }
        let nachher = self.ivars().modell.borrow().sichtbarkeit();
        // Erst die Flaeche auf den Schirm, dann alles, was einen Ersthelfer
        // setzt. Die Begruendung steht im Doc-Kommentar; die Ausleihe des
        // Modells ist an dieser Stelle beendet, `vorher` und `nachher` sind
        // Werte.
        self.aufteilung_nachziehen();
        for bereich in Bereich::ALLE {
            if sichtbar_in(&vorher, bereich) != sichtbar_in(&nachher, bereich) {
                self.nach_dem_sichtbarkeitswechsel(bereich);
            }
        }
        true
    }

    /// Was nach jedem Wechsel der Sichtbarkeit nachzuziehen ist.
    ///
    /// Die eine Stelle dafuer, gerufen ueber [`Self::sichtbarkeit_aendern`] und
    /// nur fuer einen Bereich, dessen Sichtbarkeit sich wirklich geaendert hat.
    /// Die drei Nachzuege sind nach dem Bereich unterschieden und nicht danach,
    /// welcher Befehl den Wechsel ausgeloest hat; eine zweite Liste neben
    /// dieser waere die erste Abweichung zwischen zwei Wegen in denselben
    /// Zustand.
    fn nach_dem_sichtbarkeitswechsel(&self, bereich: Bereich) {
        // Mit dem zweiten Dateifenster kommt und geht ein beobachteter Ordner.
        // Die beiden Randbereiche zeigen keinen.
        if bereich == Bereich::Rechts {
            self.dateisystemwache_nachziehen();
        }
        // Ein ausgeblendeter Randbereich darf den Fokus nicht behalten: der
        // Nutzer saehe seine Auswahl nicht mehr und wuesste nicht, wo seine
        // Tasten ankommen. Gesetzt wird ohne Nachfrage, wo er gerade steht —
        // steht er im Dateifenster, ist der Aufruf wirkungslos, und eine
        // Abfrage dafuer waere eine zweite Stelle, die nach dem Fokus fragt.
        // Seit S19 gilt das fuer die Leiste wie fuer die Vorschau.
        //
        // **Welche Bereiche Randbereiche sind, sagt `Bereich::seite` und keine
        // Aufzaehlung hier.** Ein Bereich ist genau dann keiner, wenn er kein
        // Dateifenster ist; bis zur Editor-Runde stand an dieser Stelle die
        // Literalliste `[Lesezeichen, Vorschau]`, und der Editor waere darin
        // stumm gefehlt — mit dem Fokus in einer Textflaeche, die nicht mehr
        // auf dem Schirm steht. Seit S18 kommt er hier auch dann an, wenn ihn
        // nicht sein eigener Befehl, sondern die eingeblendete Vorschau
        // verdraengt hat.
        if bereich.seite().is_none() && !self.ivars().modell.borrow().sichtbar(bereich) {
            self.fokus_setzen(Fokus::Dateifenster);
        }
        // Die eingeblendete Vorschau holt nach, was sie im ausgeblendeten
        // Zustand ausgesetzt hat; die Begruendung steht an
        // [`AnwendungsIvars::vorschau_nachtrag`].
        if bereich == Bereich::Vorschau && self.ivars().modell.borrow().sichtbar(bereich) {
            self.vorschau_nachtragen();
        }
    }

    /// Aendert die Breite des Bereichs mit dem Fokus um einen Schritt (C7, C1
    /// der Editor-Runde).
    ///
    /// **Der "aktive Bereich" der beiden Kuerzel ist der, vor dem der Nutzer
    /// steht.** Bis zur Editor-Runde war es fest das aktive Dateifenster: die
    /// Lesezeichenleiste und die Vorschau bekamen ihre Breite mit der Maus, und
    /// C7 verlangte nichts anderes. Das dritte Abnahmekriterium von C1 verlangt
    /// es jetzt fuer den Editor, "solange er den Fokus hat", und die Antwort
    /// darauf ist dieselbe Regel fuer alle vier Bereiche und keine Ausnahme fuer
    /// einen. Ein eigenes Kuerzelpaar fuer den Editor entsteht dafuer nicht:
    /// `bereich_verbreitern` und `bereich_verschmaelern` tragen
    /// [`Wirkungsbereich::Ueberall`](krk_core::tasten::Wirkungsbereich) und
    /// wirken damit aus jedem Bereich heraus.
    ///
    /// **Welcher Bereich zu einem Fokuswert gehoert, sagt
    /// [`crate::kommandos::fokus::bereich_mit_fokus`]** — dieselbe Zuordnung,
    /// die [`Self::fokus_setzen`] schon liest und die die Anzeige aus C9 zum
    /// Einfaerben nimmt, und keine zweite daneben. Bis zum 260809 stand hier
    /// `holt_hervor(...).unwrap_or_else(|| Bereich::von_seite(aktiv))`,
    /// also dieselbe Rechnung ein zweites Mal; danach haetten die Anzeige und
    /// die Breitenaenderung verschiedene Bereiche meinen koennen.
    ///
    /// **Der Unterschied zur Anzeige bleibt beim Aufrufer und ist gewollt.**
    /// [`Fokus::Anderswo`] liefert `None`; die Breitenaenderung faellt dann auf
    /// das aktive Dateifenster, weil ein Befehl ohne eigenen Bereich der Liste
    /// gilt, die der Nutzer zuletzt bedient hat (derselbe Grund wie in
    /// [`Self::bereichskommando`]). Die Anzeige laesst bei `None` dagegen alles
    /// stehen, weil `Anderswo` ein Blatt bedeutet und das siebte
    /// Abnahmekriterium von C9 verlangt, dass ein Blatt keinem Bereich seine
    /// Anzeige nimmt.
    ///
    /// **Der Schritt von 40 Punkten gilt auf dem Schirm**, und das Modell
    /// rechnet ihn mit dem [`Zeilenmass`] in gespeicherte Punkte um. Ohne diese
    /// Umrechnung sprang die Trennlinie bei einem breiten Fenster weiter als
    /// bei einem schmalen; die Begruendung steht an
    /// [`Fenstermodell::breite_aendern`](crate::fenstermodell::Fenstermodell::breite_aendern).
    ///
    /// **Was auf dem Schirm steht, ist hier schon nachgelesen.** Bis zum
    /// 260811 begann diese Funktion mit einem eigenen
    /// [`Self::bildschirmbreiten_uebernehmen`], damit ein Schritt nicht auf
    /// eine ueberholte Zahl spraenge. Seit derselbe Ruf am Kopf von
    /// [`Self::kommando_ausfuehren`] fuer **jeden** Befehl steht, waere er hier
    /// eine zweite Messung derselben Zahl; der einzige Weg hierher fuehrt
    /// ohnehin ueber jenen Kopf.
    fn breite_aendern(&self, betrag: f64) -> bool {
        // Vor der Ausleihe: `fokus` liest das Fenstermodell selbst.
        let fokus = self.fokus();
        let Some(mass) = self.zeilenmass() else {
            return false;
        };
        let mut modell = self.ivars().modell.borrow_mut();
        let aktiv = modell.aktiv();
        let bereich =
            fokus::bereich_mit_fokus(fokus, aktiv).unwrap_or_else(|| Bereich::von_seite(aktiv));
        modell.breite_aendern(bereich, betrag, mass);
        true
    }

    /// Macht das genannte Dateifenster zum aktiven.
    ///
    /// **Die eine Stelle dafuer, und drei Anlaesse gehen darueber.** Zwei
    /// kommen aus [`super::tabelle`]: die Auswahl einer Zeile ueber
    /// `tableView:shouldSelectRow:` und der Klick auf einen Abschnitt der
    /// Tableiste, beide ueber `DateifensterQuelle::angefasst`. Der dritte ist
    /// [`Self::aktives_dem_ersthelfer_nachziehen`] und deckt jede Flaeche eines
    /// Dateifensters ab, die keine Zeile ist.
    fn aktives_setzen(&self, seite: Fensterseite) {
        if self.ivars().modell.borrow_mut().aktiv_setzen(seite) {
            self.aufteilung_nachziehen();
            self.sitzung_vormerken();
        }
    }

    /// Macht das Dateifenster zum aktiven, in dem der Ersthelfer liegt.
    ///
    /// **Der dritte Anlass von [`Self::aktives_setzen`], und er setzt den
    /// Nutzerentscheid vom 260819 um** (`shared/decisions/260819-1043_*_welche-
    /// flaechen-holen-den-fokus-wenn-man-hineinklickt.md`, Moeglichkeit 1):
    /// jede Flaeche eines Bereichs holt den Fokus, und ein Klick in eine
    /// Dateiliste macht sie zur aktiven, ob er eine Zeile trifft oder nicht.
    /// Die Folge ist mitentschieden: F5 und F6 nehmen danach als Quelle das
    /// zuletzt angeklickte Dateifenster, auch ohne Auswahl.
    ///
    /// # Warum hier und nicht an der Tabelle
    ///
    /// `tableView:shouldSelectRow:` feuert bei einem Klick unter die letzte
    /// Zeile nicht, denn es gibt dort keine Zeile; am 260819 auf macOS 15.7.7
    /// an einem Nachbau des Dateifensters gemessen
    /// (`shared/analyses/260819-1043-klick-holt-den-fokus-nicht.md`). Ein
    /// `mouseDown:` an der Tabelle waere trotzdem der falsche Ort: dieselbe
    /// Messung zeigt, dass AppKit den Klick auf die freie Flaeche schon in ein
    /// `makeFirstResponder:` uebersetzt und die Tabelle den Rang annimmt. KRK
    /// muss den Klick also nicht abfangen, sondern nur auf den Rangwechsel
    /// hoeren — und dafuer gibt es seit C9 genau einen Ausloesepunkt,
    /// [`Hauptfenster`](super::fenster::Hauptfenster). Eine Ueberschreibung an
    /// der Tabelle waere die zweite Tuer, die deren Modulkopf ausschliesst,
    /// und traefe die Lesezeichenleiste ohnehin nicht mit.
    ///
    /// # Was er von sich aus in Ruhe laesst
    ///
    /// Liegt der Ersthelfer in der Leiste, der Vorschau oder dem Editor,
    /// geschieht nichts: [`Bereich::seite`] ist die eine Stelle, die aufzaehlt,
    /// welche Bereiche Dateifenster sind, und liefert fuer die drei uebrigen
    /// `None`. Damit bleibt `AktivOhneFokus` erhalten, also die Auskunft,
    /// aus welchem Dateifenster F5 kopiert, waehrend die Tasten im Editor
    /// ankommen.
    ///
    /// **Ein stehendes Blatt braucht keine eigene Abfrage.** Ein Blatt ist
    /// modal zu seinem Fenster, sein Ersthelfer liegt im Blatt und nicht im
    /// Hauptfenster, und AppKit laesst waehrenddessen keinen Klick an die
    /// Bereiche dahinter. Der Ersthelfer des Hauptfensters wechselt also nicht,
    /// und `aktiv_setzen` liefert `false`. Eine Abfrage auf
    /// [`Self::blatt_steht`] waere hier ein Zweig, den nichts erreicht —
    /// anders als in [`Self::fokusanzeige_nachziehen`], das auch ohne
    /// Rangwechsel gerufen wird und die Abfrage deshalb braucht.
    fn aktives_dem_ersthelfer_nachziehen(&self) {
        let Some(seite) = self.bereich_des_ersthelfers().and_then(Bereich::seite) else {
            return;
        };
        self.aktives_setzen(seite);
    }

    /// Holt das Fenster nach vorn (C7).
    ///
    /// Es wird nicht angelegt: `setReleasedWhenClosed(false)` haelt es ueber
    /// sein Schliessen hinweg am Leben, und der Delegierte haelt es weiter.
    ///
    /// Die eine Stelle dafuer, und vier Wege gehen darueber: der Menueeintrag
    /// "Fenster einblenden", der Klick auf das Dock-Symbol, das Kommando aus C7
    /// und seit dem 260807 der Aufbau der Oberflaeche beim Start. Beide Haelften
    /// gehoeren zusammen: `makeKeyAndOrderFront` ordnet das Fenster innerhalb
    /// von KRK nach vorn, `activate()` macht KRK zur vordersten Anwendung.
    fn fenster_zeigen(&self) {
        let Some(fenster) = self.ivars().fenster.get() else {
            return;
        };
        fenster.makeKeyAndOrderFront(None);
        NSApplication::sharedApplication(self.mtm()).activate();
    }

    /// Schliesst das Fenster (C7).
    ///
    /// Die eine Stelle dafuer, und beide Wege gehen darueber: der
    /// Ereignisabgriff und der Menueeintrag, beide seit der Runde 7 ueber
    /// [`Kommando::FensterSchliessen`] und [`Self::kommando_ausfuehren`].
    /// `performClose:`
    /// und nicht `close`, damit der Fensterdelegierte gefragt wird und die
    /// Schliessanimation dieselbe bleibt wie beim Klick auf den roten Knopf.
    ///
    /// **`performClose:` steht hier und an keinem Menueeintrag**, und das ist
    /// der Grund, aus dem der Umweg ueberhaupt besteht: zu einem Eintrag mit
    /// diesem Selektor stellt AppKit von sich aus eine Zweitform "Close All"
    /// auf Opt+Shift+Cmd+W dazu, die weder in der Belegung steht noch umbelegbar
    /// ist (gemessen am 260804-1040).
    /// Das Fenster ueberlebt sein Schliessen; "Fenster einblenden" holt es
    /// zurueck.
    ///
    /// # Der Zettel wird hier gesichert, und zwar vor `performClose:`
    ///
    /// Der dritte der vier Sicherungsmomente aus C4, und der einzige, dessen
    /// Reihenfolge zugesagt ist. Er haengt am **Ausfuehrungsweg** dieses
    /// Befehls und nicht an der Zulaessigkeitsregel: `zulaessigkeit::zulaessig`,
    /// `operationen::waehrend_blatt_erlaubt` und `immer_erreichbar` sind von der
    /// Runde 9 unberuehrt geblieben, und das ist eine ausgeschriebene Zusage des
    /// Spec.
    ///
    /// **Gesichert wird ohne Bedingung.** Was AppKit mit `performClose:` an
    /// einem Fenster mit anhaengendem Blatt tut, ist in diesem Baum nicht
    /// gemessen: es kann das Fenster samt Blatt schliessen oder beides stehen
    /// lassen und einen Ton geben. Ein Code, der die eine oder die andere Kante
    /// annaehme, saette eine Vermutung fest; ein Sichern davor haelt die Zusage
    /// „kein Weg aus dem Zettel heraus verliert Text" in **beiden** Ausgaengen.
    /// Die Messung traegt danach nur nach, welche Kante das laufende Buendel
    /// geht.
    ///
    /// **Das Blatt wird hier nicht abgeraeumt.** Bleibt es stehen, weil AppKit
    /// das Schliessen verweigert, waere ein geleerter [`Self::offenes_blatt`] die
    /// Lage, in der der Abbruchbefehl das sichtbare Blatt nicht mehr schliessen
    /// koennte. Geht das Fenster dagegen zu, kommt der Abschlussblock des
    /// Blattes von selbst hierher zurueck und raeumt beides ab.
    fn fenster_schliessen(&self) -> bool {
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };
        self.zettel_stand_uebernehmen();
        let ergebnis = self.unter_der_sperre(|zugang| self.zettel_sichern(zugang));
        self.zettel_sicherung_melden(ergebnis);
        fenster.performClose(None);
        true
    }

    /// Beendet die Anwendung (C3).
    ///
    /// Die eine Stelle dafuer, und beide Wege gehen darueber: der
    /// Ereignisabgriff und der Menueeintrag, beide seit der Runde 7 ueber
    /// [`Kommando::Beenden`] und [`Self::kommando_ausfuehren`]. `terminate:` und
    /// nicht `exit`, damit AppKit seinen
    /// Ablauf geht: erst `applicationShouldTerminate:` mit der Nachfrage aus C4,
    /// dann `applicationWillTerminate:` mit dem letzten Sitzungsstand.
    ///
    /// **`terminate:` steht hier und an keinem Menueeintrag**, aus demselben
    /// Grund wie `performClose:` bei [`Self::fenster_schliessen`]: zu einem
    /// Eintrag mit diesem Selektor stellt AppKit die Zweitform "Quit and Keep
    /// Windows" auf Opt+Cmd+Q dazu (gemessen am 260805-0753).
    ///
    /// **Der Rueckgabewert `true` sagt nichts ueber das Beenden aus**, sondern
    /// allein, dass der Tastendruck verbraucht ist. Ob KRK wirklich endet,
    /// entscheidet seit S29 [`Self::beenden_erlauben`].
    fn beenden(&self) -> bool {
        // `None` als Absender heisst: kein Steuerelement hat den Aufruf
        // ausgeloest.
        NSApplication::sharedApplication(self.mtm()).terminate(None);
        true
    }

    /// Uebernimmt in das Fenstermodell, was gerade wirklich auf dem Schirm
    /// steht.
    ///
    /// **Die eine Stelle, an der eine mit der Maus verschobene Trennlinie in
    /// das Fenstermodell kommt.** Sie steht in den Rahmen der Ansichten und
    /// nirgends sonst, und der Delegierte der Aufteilung meldet sie nicht
    /// zurueck: er haelt nichts und hat bewusst keinen Rueckweg in das Modell
    /// (siehe den Modulkopf von [`super::aufteilung`]). Das Modell erfaehrt von
    /// ihr also nur, wenn jemand nachmisst. Wer das tut, sagt es an sich selbst,
    /// und eine Aufzaehlung steht hier nicht: sie ist mit `df8163d` schon
    /// einmal falsch geworden, weil ein dritter Messer hinzukam und dieser Satz
    /// weiter von zweien sprach
    /// (`shared/issues/260823-0730_*_drei-prosastellen-um-den-neuen-nachzug-*`).
    /// Gemessen wird, wo zwischen der letzten Messung und dem naechsten Griff
    /// in das Modell eine Ziehbewegung des Nutzers liegen kann: am Kopf jedes
    /// Tastenbefehls ([`Self::kommando_ausfuehren`]), vor jedem Schreiben der
    /// Sitzung ([`Self::sitzung_bauen`]), das auch ohne Befehl faellig wird,
    /// und am Kopf jeder Fortsetzung, die lange nach ihrem Befehl laeuft
    /// ([`Self::editorausgang_behandeln`]).
    ///
    /// **Uebernommen wird dabei nur eine wirkliche Ziehbewegung.** Steht auf
    /// dem Schirm genau das, was die Breitenregel selbst ausgelegt hat, bleibt
    /// der Aufruf ohne Wirkung. Das Mass der Zeile geht allein deshalb mit,
    /// weil
    /// [`Fenstermodell::breiten_uebernehmen`](crate::fenstermodell::Fenstermodell::breiten_uebernehmen)
    /// diese Frage nicht ohne die Fensterbreite beantworten kann.
    ///
    /// **Der Zeitpunkt ist tragend und nicht beliebig: gemessen wird, solange
    /// Modell und Schirm noch dieselbe Sichtbarkeit meinen.**
    /// [`Fenstermodell::breiten_uebernehmen`](crate::fenstermodell::Fenstermodell::breiten_uebernehmen)
    /// entscheidet an der Sichtbarkeit des **Modells**, welche gemessene Zahl
    /// es annimmt. Nach einem Umschaltbefehl sagt das Modell bereits "beide
    /// Dateifenster stehen", waehrend der Schirm noch eines mit der vollen
    /// Breite zeigt; diese Zahl uebernommen, kaeme das wiedereingeblendete
    /// Fenster auf einer falschen Breite zurueck. Genau das ist der am 260804
    /// im laufenden Buendel gemessene Fehler, den der Kommentar an
    /// `breiten_uebernehmen` fuehrt.
    ///
    /// **Ob ein Aufrufer von [`Self::aufteilung_nachziehen`] eine Messung davor
    /// braucht, steht an ihm und nicht hier.** Eine Aufzaehlung an dieser
    /// Stelle war zweimal um eins daneben, ohne dass es jemand bemerkt hat, und
    /// das zweite Mal verdeckte sie einen offenen Verhaltensbefund
    /// (`shared/issues/260823-0730_*_drei-prosastellen-um-den-neuen-nachzug-*`).
    /// Zwei der Aufrufer tragen ihre Begruendung mit: beim Aufbau der
    /// Oberflaeche gibt es noch keine Ziehbewegung, die verlorenginge, und die
    /// Fortsetzung nach einer Rueckfrage aus C4 laeuft hinter einem Blatt, wo
    /// das Fenster keine Maus in seinem Inhalt annimmt. [`Self::aktives_setzen`]
    /// misst nicht und begruendet es auch nicht; **das ist keine gepruefte
    /// Ausnahme, sondern ein offener Befund**
    /// (`shared/issues/260823-0731_*_ein-klick-in-das-andere-dateifenster-*`).
    fn bildschirmbreiten_uebernehmen(&self) {
        let Some(aufteilung) = self.ivars().aufteilung.get() else {
            return;
        };
        self.ivars()
            .modell
            .borrow_mut()
            .breiten_uebernehmen(aufteilung.gemessene_breiten(), aufteilung.zeilenmass());
    }

    /// Schreibt Sichtbarkeit und Breiten in die Ansicht und zieht danach die
    /// Anzeige nach.
    ///
    /// Die Reihenfolge ist bindend: `anwenden` setzt `setHidden`, und eine
    /// ausgeblendete Ansicht, die den Ersthelfer haelt, laesst AppKit den Rang
    /// neu vergeben. Der Fokus danach ist deshalb ein anderer als der davor,
    /// und [`Self::fokusanzeige_nachziehen`] liest ihn frisch.
    ///
    /// **Das Modell ist hier die Quelle, und deshalb muss es aktuell sein.**
    /// Wer diese Funktion ruft, hat vorher entweder das Modell geaendert oder
    /// [`Self::bildschirmbreiten_uebernehmen`] gerufen; sonst schreibt sie eine
    /// ueberholte Breite auf den Schirm und nimmt dem Nutzer seine
    /// Ziehbewegung.
    fn aufteilung_nachziehen(&self) {
        let Some(aufteilung) = self.ivars().aufteilung.get() else {
            return;
        };
        let (breiten, sichtbar) = {
            let modell = self.ivars().modell.borrow();
            (modell.breiten(), modell.sichtbarkeit())
        };
        aufteilung.anwenden(&breiten, &sichtbar);
        self.fokusanzeige_nachziehen();
        self.bereichsleiste_nachziehen();
        // **Auch die Statuszeile**, und seit dem 260812 aus zwei Gruenden:
        // die Sichtbarkeit entscheidet mit, wer sich um die Zeile bewirbt, und
        // das aktive Dateifenster kann sich mit demselben Anlass geaendert
        // haben — daran haengen die zweite Stelle der Rangordnung und der
        // Namenszusatz. Der Grund im Langen steht an
        // [`Self::statuszeile_nachziehen`].
        self.statuszeile_nachziehen();
    }

    /// Schreibt die zehn Schalterzustaende der Bereichsleiste aus dem Modell
    /// (C2.1, C3.1; C2.1 der Filter-Runde; C2.1 und C2.3 der
    /// Inhaltsfilter-Runde).
    ///
    /// **Der eine Schreiber, mit zwei Anlaessen**, nach dem Vorbild von
    /// [`Self::fokusanzeige_nachziehen`] und [`Self::spaltenanzeige_nachziehen`].
    /// Der erste ist [`Self::aufteilung_nachziehen`], das jedem ausgefuehrten
    /// Kommando folgt, fuer den Tastendruck wie fuer den Klick.
    ///
    /// **Mindestens einmal je Weg und seit dem 260823 nicht mehr genau
    /// einmal.** [`Self::sichtbarkeit_aendern`] zieht den Nachzug selbst, damit
    /// eine Fortsetzung ausserhalb von [`Self::kommando_ausfuehren`] den Schirm
    /// nicht schuldig bleibt; ein Befehl, der einen Bereich umschaltet, kommt
    /// damit zweimal hier an. Geschrieben werden beide Male dieselben zehn
    /// Zustaende aus demselben Modell, und die Begruendung fuer den Preis steht
    /// an jener Funktion.
    ///
    /// **Der zweite ist der Ordnerwechsel eines Dateifensters**, und er kam mit
    /// dem neunten Schalter dazu. Die acht ersten stehen im
    /// [`Fenstermodell`](crate::fenstermodell::Fenstermodell) und aendern sich
    /// nur ueber einen Befehl; der neunte und der zehnte stehen am
    /// `Ordnermodell` des sichtbaren Tabs im aktiven Dateifenster und wechseln
    /// damit auch ohne Befehl. Drei Anlaesse haben sie, und
    /// `ordnerwechsel_melden` in [`super::tabelle`] deckt zwei davon ab: den
    /// Tabwechsel und den Ordnerwechsel, auch die mit der Maus. **Der dritte,
    /// der Wechsel des aktiven Dateifensters, braucht keine Zeile**: er laeuft
    /// ueber [`Self::aktives_setzen`] oder ueber `Kommando::FensterWechseln`,
    /// und beide rufen [`Self::aufteilung_nachziehen`], den ersten Anlass.
    ///
    /// **Der zehnte Schalter hat deshalb keinen vierten Anlass gebracht**: der
    /// Stand von "Content" haengt am selben `Ordnermodell` wie der von "Deep",
    /// und damit ist C2.3 der Inhaltsfilter-Runde ohne eine eigene Zeile
    /// erfuellt.
    ///
    /// **Faellt die offene Frage nach dem Gueltigkeitsbereich auf "je
    /// Fenster"**, faellt der zweite Anlass wieder weg und die zwei Werte
    /// kommen aus dem Fenstermodell statt aus dem Tab
    /// (`decisions/260814-1830_*_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md`).
    /// Beruehrt waeren dann diese Funktion und die eine Zeile im
    /// Ordnerwechsel-Rueckruf; [`super::bereichsleiste`] bliebe, wie sie ist.
    ///
    /// **Der abgewiesene Klick braucht keinen zweiten Anlass daneben** (C2.4),
    /// und bis zum 260812 hatte er einen: der Melder der Leiste zog unbedingt
    /// nach, weil ein Ankreuzfeld seinen Zustand selbst kippt, bevor die
    /// Aktion laeuft — nach einem **angenommenen** Klick lief der Nachzug damit
    /// zweimal. Zurueckgenommen wird die Selbstkippung jetzt in
    /// `Leistenquelle::geklickt` ([`super::bereichsleiste`]), also dort, wo sie
    /// entsteht; danach ist das Modell wieder die einzige Quelle jedes
    /// Schalterzustands.
    ///
    /// **Sie schreibt nichts als Schalterzustaende.** Sie ruft weder `anwenden`
    /// noch `setHidden` und fasst den Ersthelfer nicht an, aus demselben Grund,
    /// aus dem [`Self::fokusanzeige_nachziehen`] es nicht tut: eine
    /// ausgeblendete Ansicht, die den Rang haelt, laesst AppKit ihn neu
    /// vergeben und die Meldung ein zweites Mal ausloesen.
    fn bereichsleiste_nachziehen(&self) {
        let Some(leiste) = self.ivars().bereichsleiste.get() else {
            return;
        };
        // Die drei fensterweiten Groessen in **einer** Ausleihe, wie es
        // [`Self::statuszeile_nachziehen`] daneben ebenso haelt. Sie endet vor
        // der Frage an das Dateifenster darunter, denn dessen Quelle leiht
        // ihre eigene Tabliste aus.
        let (sichtbar, spalten, aktiv) = {
            let modell = self.ivars().modell.borrow();
            (
                modell.sichtbarkeit(),
                modell.spaltensichtbarkeit(),
                modell.aktiv(),
            )
        };
        // Steht die Leiste, stehen auch die Dateifenster: `oberflaeche_aufbauen`
        // haelt sie in derselben Folge fest und die Dateifenster zuerst. Eine
        // zweite Abfrage daneben taeuschte eine Lage vor, die es nicht gibt.
        let quelle = self.dateifenster(aktiv).quelle();
        let tief = quelle.tiefe_suche_steht();
        let inhalt = quelle.inhaltssuche_steht();
        leiste.zustaende_setzen(&sichtbar, &spalten, tief, inhalt);
    }

    /// Raeumt die Antwort auf den vorigen Tastenbefehl an **beiden**
    /// Dateifenstern weg (Rang 1).
    ///
    /// **Die eine Loeschregel des obersten Rangs, an einer Stelle und mit zwei
    /// Wegen hinein.** Sie raeumt beide Seiten, weil es genau einen letzten
    /// Befehl gibt und nicht einen je Seite: der Abschlusstext einer Kopie
    /// steht im Fenster des Vorgangs, und ein Befehl im anderen Fenster ist
    /// trotzdem neuer. Damit haengt der oberste Rang an einem Ereignis und an
    /// keiner Uhr.
    ///
    /// Die zwei Wege:
    ///
    /// 1. [`Self::kommando_ausfuehren`] vor jedem Befehl. Der aeltere Weg, und
    ///    bis zur Runde 13 der einzige.
    /// 2. Der Abwurf aus einer fremden Anwendung, wenn er die Meldung aus C7
    ///    schreibt. Er kommt ueber den achten Rueckruf der Dateiliste herein
    ///    (`DateifensterQuelle::befehlsantwort_beidseitig_loeschen`), weil eine
    ///    Quelle von sich aus nur ihre eigene Seite erreicht.
    ///
    /// **Warum der Abwurf diesen Weg braucht und nicht seine eigene Seite
    /// genuegt.** [`statuszeile::zeile`] nimmt innerhalb eines Rangs die
    /// **aktive** Seite zuerst. Stand also eine Befehlsantwort im aktiven
    /// Dateifenster und zog der Nutzer ueber das andere, verlor die Meldung des
    /// Abwurfs den Wettbewerb und war nie zu sehen — das ganze sichtbare
    /// Verhalten von C7
    /// (`issues/260818-2332_*_the-drop-writes-a-rank-1-message-without-clearing-the-other-pane-and-loses-it.md`).
    ///
    /// **Ein dritter Weg gehoert nicht hierher, sondern zu der Reichweite, die
    /// er hat.** `DateifensterQuelle::doppelklick` raeumt ausdruecklich nur
    /// seine eigene Seite: geraeumt wird so weit, wie die Handlung reicht, und
    /// ein Doppelklick reicht ueber die angeklickte Zeile. Der Abwurf reicht
    /// weiter, weil seine Meldung sonst nicht erscheint.
    fn befehlsantwort_beidseitig_loeschen(&self) {
        for seite in Fensterseite::ALLE {
            self.dateifenster(seite).quelle().befehlsantwort_loeschen();
        }
    }

    /// Schreibt die eine Statuszeile ueber die volle Fensterbreite (C5 der
    /// Runde 6).
    ///
    /// **Der eine Schreiber, mit zwei Anlaessen.** Der erste ist der
    /// Meldungswechsel eines der beiden Dateifenster: eine seiner sechs Quellen
    /// hat sich geaendert, und es sagt es ueber den Rueckruf aus dem Aufbau.
    /// Der zweite ist [`Self::aufteilung_nachziehen`], weil die Zeile nicht nur
    /// von den zwoelf Quellen abhaengt, sondern auch davon, **welches**
    /// Dateifenster das aktive und **welches sichtbar** ist: der Rang der
    /// aktiven Seite entscheidet jeden Gleichstand, der Namenszusatz haengt an
    /// derselben Frage, und ein ausgeblendetes Dateifenster bewirbt sich gar
    /// nicht erst. Beides wechselt an demselben Nachzug — der Sichtbarkeit
    /// wegen ist er sogar der einzige Weg —, und ein Wechsel des aktiven
    /// Dateifensters geht auf beiden Wegen durch ihn: Mausklick ueber
    /// [`Self::aktives_setzen`], Tastenbefehl ueber `Kommando::FensterWechseln`.
    ///
    /// **Sie entscheidet selbst nichts.** Die Auswahl unter den zwoelf Bewerbern
    /// trifft [`statuszeile::zeile`], den Satz formt
    /// [`statuszeile::zeilentext`]; beide sind reines Rust ohne AppKit und ohne
    /// Fenster pruefbar. Diese Funktion holt die vier Eingaben und schreibt das
    /// Ergebnis.
    ///
    /// **Auch das ausgeblendete Dateifenster wird gefragt**, und seine Antwort
    /// verwirft [`statuszeile::zeile`]. Die Bedingung hier zu ziehen waere
    /// zwei Zeilen kuerzer und von keiner Probe zu erreichen; genau daran ist
    /// die Zusage aus C5.8 einmal vorbeigelaufen
    /// (`issues/260812-1805_*_die-eine-statuszeile-zeigt-meldungen-eines-ausgeblendeten-dateifensters.md`).
    ///
    /// **Sie steht neben [`Self::bereichsleiste_nachziehen`] und nicht darin.**
    /// Die Leiste zeigt Schalterzustaende, die Zeile zeigt Meldungen; ein
    /// gemeinsamer Nachzug haette zwei Anlaesse in einer Funktion, und der
    /// Meldungswechsel eines Dateifensters ginge die Leiste nichts an.
    fn statuszeile_nachziehen(&self) {
        // Steht die Zeile, stehen auch die Dateifenster: `oberflaeche_aufbauen`
        // haengt sie in derselben Folge ein und die Dateifenster zuerst. Eine
        // zweite Abfrage daneben taeuschte eine Lage vor, die es nicht gibt.
        let Some(zeile) = self.ivars().statuszeile.get() else {
            return;
        };
        // Beide Quellensaetze und die aktive Seite. `meldungsquellen` leiht
        // dabei das Tabmodell seines Dateifensters aus und liefert eigene
        // Zeichenketten; die Ausleihe endet damit in ihm und ueberlebt den
        // Aufruf von AppKit unten nicht.
        let links = self
            .dateifenster(Fensterseite::Links)
            .quelle()
            .meldungsquellen();
        let rechts = self
            .dateifenster(Fensterseite::Rechts)
            .quelle()
            .meldungsquellen();
        // Aktive Seite und Sichtbarkeit in **einer** Ausleihe, wie es
        // [`Self::bereichsleiste_nachziehen`] daneben ebenso haelt.
        let (aktiv, sichtbar) = {
            let modell = self.ivars().modell.borrow();
            (modell.aktiv(), modell.sichtbarkeit())
        };
        let meldung = statuszeile::zeile(&links, &rechts, aktiv, &sichtbar);
        // Der Satz bekommt eine eigene Bindung, weil `zeilentext` eine
        // Zeichenkette **baut** und `zeigen` eine ausleiht: ohne die Bindung
        // gaebe es nichts, woraus die Ausleihe genommen werden koennte.
        let satz = meldung
            .as_ref()
            .map(|meldung| (statuszeile::zeilentext(meldung, aktiv), meldung.art));
        zeile.zeigen(satz.as_ref().map(|(text, art)| (text.as_str(), *art)));
    }

    /// Schreibt die Rahmenfarben der fuenf Bereiche und den Fenstertitel (C9,
    /// C11).
    ///
    /// **Der eine Schreiber der Anzeige, mit zwei Anlaessen.** Der erste ist
    /// die Meldung aus [`Hauptfenster`](super::fenster::Hauptfenster): jeder
    /// erfolgreiche Wechsel des Ersthelfers und jeder Wechsel zwischen Vorder-
    /// und Hintergrund. Der zweite ist [`Self::aufteilung_nachziehen`], weil
    /// ein Sichtbarkeitswechsel die Kaesten neu auslegt.
    ///
    /// **Sie ruft weder `anwenden` noch `setHidden`, und das ist keine
    /// Sparsamkeit, sondern die Vermeidung eines Rings.** Eine ausgeblendete
    /// Ansicht, die den Ersthelfer haelt, laesst AppKit den Rang neu vergeben,
    /// also `makeFirstResponder:` erneut aufrufen — und damit diese Meldung ein
    /// zweites Mal ausloesen. Der Fokusnachzug ist deshalb die kuerzere
    /// Funktion, die ausschliesslich Farben und den Titel schreibt.
    ///
    /// **Steht ein Blatt am Fenster, bleibt alles stehen, wie es stand.** Das
    /// siebte Abnahmekriterium von C9 verlangt es fuer die Rahmen, das achte
    /// von C11 fuer den Titel; ein Blatt ist voruebergehend und gehoert dem
    /// Bereich dahinter. Eine Abfrage, zwei Zusagen.
    ///
    /// **Gefragt wird [`Self::ersthelferbereich`] und nicht [`Self::fokus`]**,
    /// und der Unterschied traegt das achte Abnahmekriterium von C9. `fokus`
    /// antwortet [`Fokus::Anderswo`], sobald KRK kein Schluesselfenster mehr
    /// hat — also gerade dann, wenn das Fenster in den Hintergrund geht und die
    /// Anzeige zuruecktreten soll. Die Begruendung im Langen steht an
    /// `ersthelferbereich`.
    fn fokusanzeige_nachziehen(&self) {
        let (Some(aufteilung), Some(fenster)) =
            (self.ivars().aufteilung.get(), self.ivars().fenster.get())
        else {
            return;
        };
        if self.blatt_steht() {
            return;
        }
        let fokus = self.ersthelferbereich();
        let aktiv = self.ivars().modell.borrow().aktiv();
        aufteilung.rahmen_setzen(fokus, aktiv, fenster.isKeyWindow());
        self.titel_nachziehen(fokus);
    }

    /// Schreibt den absoluten Pfad des Bereichs mit dem Fokus in den
    /// Fenstertitel (C11).
    ///
    /// **Die Regel steht nicht hier.** [`crate::fenstertitel::titel`] ist eine
    /// reine Funktion ueber die fuenf Fokuswerte, ohne AppKit und ohne
    /// Auffangzweig; diese Funktion sammelt die drei Pfade ein und schreibt das
    /// Ergebnis. `None` heisst "den Titel stehen lassen".
    ///
    /// Vier Anlaesse rufen sie, drei davon ueber
    /// [`Self::fokusanzeige_nachziehen`] hinaus: der Ordner- und Tabwechsel
    /// eines Dateifensters ueber den Melder aus dem Aufbau, der Dateiwechsel im
    /// Editor, und der Tabwechsel der Vorschau.
    ///
    /// **Die Bewegung der Auswahl ruft sie nicht**, und das ist eine Zusage und
    /// kein Vergessen: L1 aus C8 der Runde 1 misst die Spanne vom Tastendruck
    /// bis zum Zeichendurchgang im Dateifenster, und ein Titel, der bei jedem
    /// Druck auf eine Pfeiltaste neu geschrieben wuerde, laege in genau dieser
    /// Spanne. Der Ordner aendert sich dabei ohnehin nicht.
    fn titel_nachziehen(&self, fokus: Fokus) {
        let (Some(fenster), Some(dateifenster)) =
            (self.ivars().fenster.get(), self.ivars().dateifenster.get())
        else {
            return;
        };
        let aktiv = self.ivars().modell.borrow().aktiv();
        let ordner = dateifenster[aktiv.index()].quelle().angezeigter_ordner();
        let editordatei = self.ivars().editor.get().and_then(|editor| editor.pfad());
        let vorschaudatei = self
            .ivars()
            .vorschau
            .get()
            .and_then(|vorschau| vorschau.angezeigter_pfad());
        let Some(titel) = fenstertitel::titel(
            fokus,
            &ordner,
            editordatei.as_deref(),
            vorschaudatei.as_deref(),
        ) else {
            return;
        };
        fenster.setTitle(&NSString::from_str(&titel));
    }

    // ------------------------------------------------------------------
    // Dateioperationen (C4)
    // ------------------------------------------------------------------

    /// Kopieren oder Verschieben in den Ordner des anderen Dateifensters (C4).
    fn uebertragen(&self, kommando: Kommando) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();
        let ziel = self
            .dateifenster(aktiv.andere())
            .quelle()
            .angezeigter_ordner();
        let art = match kommando {
            Kommando::Verschieben => Art::Verschieben { ziel },
            // Der Aufrufer schickt nur diese beiden; ein drittes Kommando hier
            // waere ein Fehler im Zweig darueber und nicht in dieser Zeile.
            _ => Art::Kopieren { ziel },
        };
        self.auftrag_stellen(art)
    }

    /// Die Auswahl in den Papierkorb des Systems raeumen (C4, Taste Delete).
    ///
    /// **Nach genau einer Rueckfrage**, in ihrer ruhigen Form, also ohne
    /// Warnzeichen (C2 der Runde 12). Bis zum 260817 lief der Befehl sofort
    /// und ohne Rueckfrage, mit dem Papierkorb als Rueckweg; der Anlass, das
    /// zu aendern, ist ein Schadensfall, bei dem ein einziger Tastendruck 189
    /// verfolgte Dateien geraeumt hat und vier Stunden lang niemandem auffiel.
    /// Der Rueckweg bleibt, was er war, und ersetzt die Rueckfrage nicht: er
    /// hilft allein dem, der den Vorgang bemerkt.
    ///
    /// Der Rumpf ist [`Self::loeschen_nach_rueckfrage`]. Hier steht allein das
    /// eine Stueck, das dieser Befehl mitbringt: die Beschriftung der zweiten
    /// Schaltflaeche.
    ///
    /// **Die Auftragsart stand bis zum 260818 als zweites daneben**, und mit
    /// ihr eine Angabe, die nichts mehr unterschied: der Rumpf nahm ein
    /// `art: Art`, dessen Aufzaehlung vier Werte fuehrt und von denen sein
    /// eigener Vertrag drei ausschliesst. Ein `debug_assert!` daneben waere die
    /// schwaechere Antwort gewesen — es greift im Auslieferungsbau nicht, wie
    /// derselbe Fehler an [`crate::appkit::blaetter::Blatt::mit_schaltflaechen`]
    /// gerade gezeigt hat. Die Angabe ist stattdessen gefallen: was es nicht
    /// gibt, kann kein Aufrufer falsch besetzen, und zwar in jedem Profil
    /// (`issues/260817-2243_*_the-delete-body-takes-an-art-that-admits-three-values-*.md`).
    ///
    /// **Die Texte selbst entstehen seit dem 260817 im Rumpf** und nicht mehr
    /// hier. Der Grund ist die Tafel der Ausloeser: die Frage nennt den ersten
    /// Warngrund, also braucht sie die fuenf Tatsachen ueber das Ziel, und die
    /// beschafft der Rumpf erst, wenn seine beiden billigen Stufen durch sind.
    ///
    /// **Der Fokusvorbehalt steht seit Schritt 18 nicht mehr hier.** Er stand
    /// als eigene Abfrage an dieser Stelle; heute traegt der Befehl
    /// `Wirkungsbereich::Dateifenster`, und die Zuleitung weist ihn ab, bevor
    /// er hier ankommt.
    fn in_den_papierkorb(&self) -> bool {
        self.loeschen_nach_rueckfrage("In den Papierkorb räumen")
    }

    /// Was ein Druck auf `delete` bedeutet: ein Zeichen des Filtertexts
    /// zurueck, gar nichts, oder der Weg in den Papierkorb mit seiner
    /// Rueckfrage (C1.14 bis C1.20, C6.9, C6.11).
    ///
    /// **Der eine Zweig dieser Runde, dessen falsche Haelfte die
    /// Loeschrueckfrage aufgehen laesst**, und deshalb die Stelle, an der die
    /// drei Aussagen einzeln dastehen. Seit dem 260817 raeumt diese Haelfte
    /// nichts mehr, sie fragt; die Unterscheidung ist dadurch milder geworden
    /// und nicht ueberfluessig, denn eine Rueckfrage, die auf jeden
    /// berichtigten Vertipper aufgeht, wird weggeklickt statt gelesen
    /// ([`crate::kommandos::rueckschritt`]):
    ///
    /// 1. **Kein Anschlag heisst Papierkorb.** Der Menueeintrag "In den
    ///    Papierkorb raeumen" ist der eine Weg, der hier ohne Tastendruck
    ///    ankommt; die Fallunterscheidung ist fuer ihn nicht gestellt, und die
    ///    Belegungsansicht, das Hauptmenue und die Markdown-Ausgabe fuehren
    ///    fuer die Taste weiter genau einen Eintrag (C1.19, C6.11). Die
    ///    Bereichsleiste ist kein zweiter solcher Weg, obwohl auch sie ohne
    ///    Anschlag meldet: ihre zehn Kommandos sind Umschalter, und
    ///    [`Kommando::InPapierkorb`] ist keines davon.
    /// 2. **Eine Zusatztaste heisst Papierkorb.** `cmd+delete` faellt an
    ///    [`Anschlag::ist_nackter_rueckschritt`] heraus und raeumt in jeder
    ///    Lage, auch bei stehendem Filtertext (C1.17). `f8` faellt an derselben
    ///    Frage heraus: es traegt seit dem Wegfall des endgueltigen Loeschens
    ///    [`Kommando::InPapierkorb`] und ist keine Rueckschritt-Taste.
    ///    `ctrl+delete` erreicht diese Funktion gar nicht: es wirkt in der
    ///    Lesezeichenleiste und geht durch
    ///    `Leistenquelle::kommando_ausfuehren`.
    /// 3. **Sonst entscheidet die Regel**, und sie steht als reine Funktion in
    ///    [`crate::kommandos::rueckschritt`], mit einer ausgeschriebenen Tafel
    ///    ueber acht Faelle. Sie wird hier gerufen und nicht nachgebaut.
    ///
    /// **`betroffene` wird fuer die beiden ersten Ausgaenge nicht befragt**
    /// (C6.9). Der Weg dorthin fuehrt allein ueber [`Self::in_den_papierkorb`],
    /// und weder eine Auswahl noch eine Markierung wird auf den beiden anderen
    /// Wegen angefasst.
    ///
    /// **Der Merker wird hier fortgeschrieben und sonst nur zurueckgesetzt**;
    /// die eine Ruecksetzzeile steht am Kopf von [`Self::eingabe_ausfuehren`].
    /// Er ist nicht "steht ein Filtertext" in Verkleidung: nach dem Anschlag,
    /// der den Filtertext leert, steht keiner mehr, und der Merker traegt
    /// trotzdem die Sperre fuer die weiteren Anschlaege derselben Wiederholung.
    ///
    /// Der Rueckgabewert ist derselbe wie ueberall in
    /// [`Self::kommando_ausfuehren`]: ob der Rumpf etwas getan hat. Er
    /// entscheidet allein ueber die beiden Nachwirkungen dort und nicht
    /// darueber, ob der Tastendruck geschluckt wird — geschluckt wird jeder der
    /// drei Ausgaenge, denn zulaessig war der Befehl in allen dreien.
    fn papierkorb_oder_zeichen_zurueck(&self, anschlag: Option<Anschlag>) -> bool {
        let Some(anschlag) = anschlag else {
            return self.in_den_papierkorb();
        };
        if !anschlag.ist_nackter_rueckschritt() {
            return self.in_den_papierkorb();
        }

        let aktiv = self.ivars().modell.borrow().aktiv();
        let (ausgang, merker) = rueckschritt(
            self.dateifenster(aktiv).quelle().filter_steht(),
            anschlag.wiederholung,
            self.ivars().rueckschritt_merker.get(),
        );
        self.ivars().rueckschritt_merker.set(merker);

        match ausgang {
            // Ein Vertipper wird berichtigt (C1.14, C1.15).
            Rueckschritt::ZeichenZurueck => {
                self.dateifenster(aktiv)
                    .quelle()
                    .letztes_filterzeichen_weg();
                true
            }
            // Die gehaltene Taste hat den Filtertext geleert und traegt nicht
            // ueber diese Grenze: kein Auftrag, keine Meldung (C1.18). Erst ein
            // neuer Druck raeumt, und `false` sagt hier nur, dass kein Nachzug
            // der Aufteilung und keine Sitzung vorzumerken ist.
            Rueckschritt::Nichts => false,
            // Wie vor der Runde 10 (C1.16, C1.20): der Weg in den Papierkorb,
            // seit dem 260817 mit seiner Rueckfrage davor.
            Rueckschritt::InDenPapierkorb => self.in_den_papierkorb(),
        }
    }

    /// Der eine Rumpf jedes Loeschbefehls: pruefen, fragen, und erst dann den
    /// Auftrag stellen (C2 der Runde 12).
    ///
    /// **Fuenf Stufen in dieser Reihenfolge**, und die Reihenfolge ist die
    /// Zusage:
    ///
    /// ```text
    /// laeuft schon ein Vorgang? ──ja──> Statuszeile, kein Blatt
    ///  │ nein
    ///  └─> Auswahl leer?         ──ja──> Statuszeile, kein Blatt
    ///       │ nein
    ///       └─> fuehrt das Ziel einen Papierkorb?
    ///            │        └──nein oder unentschieden──> Statuszeile, kein Blatt
    ///            │ ja
    ///            └─> Blatt zeigen ──> Cmd+Return? ──nein──> nichts geschieht
    ///                                     │ ja
    ///                                     └──> Auftrag mit der gezeigten Auswahl
    /// ```
    ///
    /// **Die Reihenfolge steht seit dem 260817 nicht mehr hier, sondern in
    /// [`loeschwarnung::vor_der_rueckfrage`].** Sie ist eine Regel ueber drei
    /// Wahrheitswerte und keine AppKit-Sache; als Kette von `if`-Zweigen in
    /// diesem Rumpf war ausgerechnet die Mechanik dieser Runde von keiner Probe
    /// gedeckt, denn ein Blatt laesst sich unter `libtest` nicht bedienen. Dieser
    /// Rumpf beschafft jetzt die drei Tatsachen — jede aus genau einer Quelle —
    /// und fuehrt aus, was die Regel sagt. **Was er dabei nicht mehr tut, ist
    /// entscheiden**, und wer die Reihenfolge aendern will, aendert die Tafel
    /// dort und nicht die Zeilenfolge hier.
    ///
    /// **Der laufende Vorgang wird vor dem Blatt geprueft und nicht danach.**
    /// Bis zum 260817 stand die Frage in [`Self::auftrag_stellen`], also hinter
    /// der Rueckfrage: KRK zeigte dann ein Blatt, liess den Nutzer bestaetigen
    /// und meldete erst danach, dass bereits eine Operation laeuft. Eine
    /// Rueckfrage, deren Ja folgenlos bleibt, gewoehnt den Nutzer daran, sie
    /// wegzudruecken, und genau diese Gewoehnung ist der Gegner dieser Runde.
    ///
    /// **Der Papierkorbtest steht aus demselben Grund vor dem Blatt** (C4). Er
    /// entscheidet, ob es fuer diesen Vorgang einen Rueckweg gibt; danach
    /// gefragt, haette der Nutzer einem Raeumen zugestimmt, das nicht raeumen
    /// kann. Der angezeigte Ordner wird dafuer **einmal** aufgeloest, und ein
    /// Pfad, der sich nicht aufloesen laesst, zaehlt als
    /// [`Loeschzielbefund::Unentschieden`] und loescht damit ebenfalls nicht.
    ///
    /// **Die teuren Tatsachen fallen erst an, wenn die billigen Stufen durch
    /// sind.** Aufloesen und Papierkorbfrage kosten Zugriffe auf das
    /// Dateisystem, die fuenf Tatsachen der Ausloesertafel kosten bis zu 26
    /// geoeffnete Verzeichnisse; erhoben wurden die ersten beiden bis zum
    /// 260817 unbedingt, und damit blockierte ein `delete` ohne Auswahl auf
    /// einem haengenden Netzlaufwerk den Hauptfaden
    /// (`issues/260817-1419_*_der-papierkorbtest-laeuft-vor-den-beiden-billigen-sperren-*.md`).
    /// Der Papierkorbbefund reist deshalb als `FnOnce` in die Tafel, und die
    /// fuenf Tatsachen entstehen im vierten Zweig. **An der Reihenfolge der
    /// Stufen aendert das nichts** — sie steht in
    /// [`loeschwarnung::vor_der_rueckfrage`] und ist eine Zusage des Specs;
    /// verschoben hat sich allein, wann eine Tatsache anfaellt.
    ///
    /// **Die Pruefung kennt keine Ausnahme.** Die Directive dieser Runde sagt
    /// ohne Einschraenkung „ein Ziel ohne Papierkorb wird nicht geloescht,
    /// sondern gemeldet"; ein Zweig, der sie fuer irgendeinen Befehl
    /// ueberspringt, waere ein zweiter Loeschweg an der Stelle, an der diese
    /// Runde den zweiten abschafft.
    ///
    /// **Die beiden Texte entstehen hier und kommen nicht mehr fertig herein.**
    /// Welcher Wortlaut in welcher Form dasteht, gehoert weiter
    /// [`crate::kommandos::loeschwarnung`] und nicht dieser Datei; hier steht,
    /// **wann** er gebaut wird, und das ist seit dem 260817 der vierte Zweig der
    /// Stufenregel. Der Grund ist die Tafel der Ausloeser: die Frage nennt den
    /// ersten Warngrund, also braucht sie die fuenf Tatsachen ueber das Ziel.
    /// Zwei Nebenwirkungen sind damit weg
    /// (`issues/260817-1108_*_die-loeschfrage-entsteht-vor-beiden-sperren-*.md`):
    /// die Texte entstanden vor beiden Sperren und wurden in drei der Ausgaenge
    /// verworfen, und im leeren Fall entstand dabei der Satz „Diese 0 Eintraege
    /// in den Papierkorb raeumen?", den nie ein Schirm zeigte. Die Auswahl wird
    /// ausserdem einmal je Tastendruck gelesen statt zweimal.
    ///
    /// Die Lesung, die auseinanderlaufen kann, ist die **nach** dem Blatt, und
    /// die gibt es nicht mehr; warum, sagt [`Self::loeschauftrag_stellen`].
    ///
    /// **Die Auftragsart nennt der Rumpf selbst, und das ist der Vertrag.**
    /// Bis zum 260818 nahm er sie als `art: Art` entgegen — eine Aufzaehlung mit
    /// vier Werten, von denen dieser Rumpf drei ausschliesst: mit
    /// `Art::Kopieren`, `Art::Verschieben` oder `Art::UmbenennenImStapel`
    /// zeigte er die Loeschrueckfrage und startete auf ein Ja hin eine Kopie,
    /// eine Verschiebung oder ein Stapelumbenennen. Nichts hielt das auf, weder
    /// der Typ noch eine Zusicherung noch eine Probe. Der Nachbarparameter
    /// hatte diesen Halter einmal: die Aufzaehlung `Loeschtexte` bestand allein
    /// dafuer, dass der Uebersetzer den Bau anhaelt, sobald der zweite
    /// Loeschbefehl faellt — sie hat ihre Arbeit getan und ist mit ihm
    /// gefallen. `art` hatte nie einen
    /// (`issues/260817-2243_*_the-delete-body-takes-an-art-that-admits-three-values-*.md`).
    ///
    /// **Der Typ selbst kann die Einschraenkung nicht tragen.**
    /// [`Art`](krk_core::operation::Art) gehoert `krk-core` und fuehrt die vier
    /// Arten, die die Dateioperationen dieses Programms kennen; ein zweiter Typ
    /// daneben, der nur einen Wert kennt, waere eine Aufzaehlung mit einer
    /// Variante samt Ruecktausch an der einen Uebergabestelle. Der kleinste
    /// Typ, der allein die zulaessigen Werte kennt, ist hier **kein
    /// Parameter**: es gibt einen zulaessigen Wert, es gibt einen Aufrufer, und
    /// eine Angabe, die nichts unterscheidet, kann auch nichts falsch
    /// unterscheiden. Das haelt in jedem Profil, anders als ein
    /// `debug_assert!`.
    ///
    /// **Der Schnitt zu [`Self::in_den_papierkorb`] bleibt trotzdem**, und der
    /// Befehl bringt weiterhin ein Stueck mit: die Beschriftung der zweiten
    /// Schaltflaeche. Zusammengelegt truege eine Funktion die Stufenregel und
    /// die zwei Angaben des Befehls in einem Rumpf.
    ///
    /// Liefert `true`, auch wenn nichts geschehen ist: der Tastendruck ist
    /// verbraucht, und die Statuszeile sagt warum. `false` allein dann, wenn es
    /// kein Fenster gibt, an dem das Blatt haengen koennte.
    fn loeschen_nach_rueckfrage(&self, schaltflaeche: &str) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();

        // Die beiden billigen Tatsachen der Regel, jede aus genau einer Quelle.
        // Sie stehen im Speicher, und die Reihenfolge, in der sie hier
        // anfallen, entscheidet nichts — welche Stufe daraus folgt, sagt
        // `vor_der_rueckfrage`, und dass der Papierkorbtest vor der Rueckfrage
        // steht, steht als Zeile in seiner Tafel und nicht als Zeilenfolge hier.
        // Die Frage geht seit der Runde 13 durch `vorgang_laeuft` und nicht
        // mehr an `ivars().vorgang` heran: sie hat eine Antwortstelle, und
        // dieser Zweig braucht von ihrer Antwort allein, ob es sie gibt.
        let vorgang_laeuft = self.vorgang_laeuft().is_some();
        let quelle = self.dateifenster(aktiv).quelle();
        let auswahl = quelle.betroffene_eintraege();
        let quellordner = quelle.angezeigter_ordner();

        // Der aufgeloeste Ordner, gemerkt fuer den vierten Zweig: die Tafel
        // ruft den Abschluss darunter allein im Feld `(false, false)`, und dann
        // brauchen ihn beide Fragen an den Datentraeger. Aufgeloest wird deshalb
        // genau einmal und nur dort, wo die Antwort gebraucht wird.
        let mut aufgeloester_ordner: Option<PathBuf> = None;

        match loeschwarnung::vor_der_rueckfrage(vorgang_laeuft, auswahl.ist_leer(), || {
            // `fuehrt_einen_papierkorb` fasst das Dateisystem nicht an und
            // bekommt den Ordner deshalb aufgeloest herein, sonst meldete eine
            // Verknuepfung den Papierkorb ihres eigenen Ortes statt den ihres
            // Ziels. Ein Pfad, der sich nicht aufloesen laesst, ist keine
            // Aussage ueber das Ziel, sondern eine ueber KRKs Kenntnis von ihm,
            // und zaehlt darum als unentschieden.
            aufgeloester_ordner = std::fs::canonicalize(&quellordner).ok();
            aufgeloester_ordner
                .as_deref()
                .map_or(Loeschzielbefund::Unentschieden, |aufgeloest| {
                    papierkorb::fuehrt_einen_papierkorb(aufgeloest)
                })
        }) {
            // Die Meldung baut `vorgang_laeuft_schon`, die eine Stelle,
            // die sie fuer alle drei Frager baut; sie nennt die Art des
            // laufenden Vorgangs und liest ihn dafuer ein zweites Mal. Es ist
            // derselbe Durchgang der Ereignisschleife wie oben, also dieselbe
            // Antwort, und `let _ =` heisst hier wie ueberall im Baum „ich
            // brauche den Wert nicht": entschieden ist die Stufe schon.
            Vorstufe::VorgangLaeuft => {
                let _ = self.vorgang_laeuft_schon(aktiv);
                true
            }
            Vorstufe::NichtsAusgewaehlt => {
                self.antwort_zeigen(aktiv, "es ist nichts ausgewählt");
                true
            }
            // Kein Blatt, kein Auftrag, und die Statuszeile nennt Befund, Folge
            // und Ausweg (C4).
            Vorstufe::OhnePapierkorb => {
                self.antwort_zeigen(aktiv, loeschwarnung::ohne_papierkorb());
                true
            }
            Vorstufe::Rueckfrage => {
                let Some(fenster) = self.ivars().fenster.get() else {
                    return false;
                };

                let (frage, erlaeuterung, laut) =
                    Self::loeschtexte(&auswahl, &quellordner, aufgeloester_ordner);

                // Der Auftrag reist durch den Rueckruf und nicht neben ihm her.
                // Der Rueckruf ist ein `Fn` und laeuft genau einmal, also traegt
                // eine `Cell` den Inhalt und gibt ihn beim ersten Zugriff heraus.
                let bestaetigter = Cell::new(Some((Art::InDenPapierkorb, auswahl, quellordner)));
                let schwach = objc2::rc::Weak::from_retained(&self.retain());
                let griff = loeschbestaetigung::zeigen(
                    self.mtm(),
                    fenster,
                    &frage,
                    &erlaeuterung,
                    schaltflaeche,
                    laut,
                    move |bestaetigt| {
                        let Some(selbst) = schwach.load() else {
                            return;
                        };
                        *selbst.ivars().offenes_blatt.borrow_mut() = None;
                        // Die fuenfte Stufe, und sie steht als Tafel in
                        // `loeschwarnung::nach_der_rueckfrage`: was KRK aus der
                        // Antwort des Blattes macht, ist eine Rechnung ueber
                        // zwei Wahrheitswerte und ohne Fenster pruefbar. Den
                        // Vordergrund verlangt allein, dass `bestaetigt` richtig
                        // ankommt.
                        let auftrag = bestaetigter.take();
                        match (
                            loeschwarnung::nach_der_rueckfrage(bestaetigt, auftrag.is_some()),
                            auftrag,
                        ) {
                            (Nachstufe::Auftrag, Some((art, auswahl, quellordner))) => {
                                selbst.loeschauftrag_stellen(art, auswahl, quellordner);
                            }
                            // `(Auftrag, None)` kann die Tafel nicht liefern —
                            // sie hat `Auftrag` gerade fuer `is_some()` gesagt —,
                            // und der Zweig steht trotzdem ausgeschrieben da,
                            // damit die Fallunterscheidung ohne Auffangzweig
                            // vollstaendig ist.
                            (Nachstufe::Auftrag, None) | (Nachstufe::KeinAuftrag, _) => {}
                        }
                    },
                );
                *self.ivars().offenes_blatt.borrow_mut() = Some(griff);
                true
            }
        }
    }

    /// Die beiden Zeilen der Rueckfrage und ihre Form, aus den fuenf Tatsachen
    /// ueber das Ziel (C3).
    ///
    /// **Hier stehen die fuenf Quellen und sonst nichts.** Was aus ihnen folgt,
    /// entscheidet [`loeschwarnung::warngruende`] an einer Stelle, und wie es
    /// dasteht, [`loeschwarnung::frage_und_erlaeuterung`]. Jede Tatsache kommt
    /// aus genau einer Quelle:
    ///
    /// ```text
    /// ordner              der angezeigte Ordner, hier schon aufgeloest
    /// benutzerverzeichnis krk_core::ablage::pfade::benutzerverzeichnis
    /// netzlaufwerk        super::volumes::liegt_auf_netzlaufwerk
    /// arbeitsbaum         krk_core::verzeichnis::arbeitsbaum
    /// umfang              krk_core::verzeichnis::umfang::zaehlen
    /// ```
    ///
    /// **Das Benutzerverzeichnis hat weiterhin genau einen Frager.** Gefragt wird
    /// `pfade::benutzerverzeichnis` einmal je Loeschbefehl, der Pfad wird einmal
    /// aufgeloest, und derselbe Wert geht an beide Stellen, die ihn brauchen: an
    /// das Feld des Ziels, aus dem [`loeschwarnung::warngruende`] die Ausloeser 1,
    /// 2 und 4 rechnet, und an die Grenze des Aufwaertsgangs der Git-Pruefung.
    /// Genommen wird **nicht** das freie [`benutzerverzeichnis`] dieses Moduls:
    /// es weicht auf `/` aus, wenn das System keines nennt, und ein `/` an
    /// dieser Stelle machte aus „KRK kennt den Benutzerordner nicht" die Aussage
    /// „der Ordner liegt darin". `None` heisst hier, dass die Frage offen ist,
    /// und die Tafel macht daraus [`loeschwarnung::Warngrund::Unentscheidbar`].
    ///
    /// **Der aufgeloeste Ordner kommt herein und wird nicht neu erfragt.** Der
    /// Rumpf hat ihn fuer den Papierkorbtest schon aufgeloest, und beide Fragen
    /// an den Datentraeger brauchen dieselbe Fassung: eine Verknuepfung meldete
    /// sonst den Datentraeger ihres eigenen Ortes statt den ihres Ziels. Ein
    /// `None` — der Ordner liess sich nicht aufloesen — fuehrt an allen drei
    /// Stellen auf `Unentschieden` beziehungsweise `Unentscheidbar` und nie auf
    /// eine stille Entwarnung.
    ///
    /// **Die Polaritaet ist hier zu lesen und nicht zu erschliessen.**
    /// `liegt_auf_netzlaufwerk` und `beruehrt_einen_arbeitsbaum` liegen auf der
    /// Polaritaet, auf der [`Loeschzielbefund::Ja`] warnt und
    /// [`Loeschzielbefund::Unentschieden`] zu ihm gehoert; genau so nehmen die
    /// beiden Felder des Ziels sie auf. Die Papierkorbfrage im Rumpf darueber
    /// liegt auf der anderen, dort ist `Ja` die Erlaubnis. Der Modulkopf von
    /// [`krk_core::verzeichnis::Loeschzielbefund`] haelt die beiden
    /// auseinander.
    ///
    /// Der dritte Rueckgabewert ist `laut`: die Liste der Warngruende ist nicht
    /// leer. Er gehoert hierher und nicht in
    /// [`loeschwarnung::frage_und_erlaeuterung`], weil ein Text kein Warnzeichen
    /// tragen kann.
    #[must_use = "die drei Werte sind der ganze Inhalt der Rueckfrage; fallengelassen erscheint sie leer oder gar nicht"]
    fn loeschtexte(
        auswahl: &Auswahl,
        quellordner: &Path,
        aufgeloester_ordner: Option<PathBuf>,
    ) -> (String, String, bool) {
        let zuhause =
            pfade::benutzerverzeichnis().and_then(|pfad| std::fs::canonicalize(pfad).ok());
        let netzlaufwerk = aufgeloester_ordner.as_deref().map_or(
            Loeschzielbefund::Unentschieden,
            volumes::liegt_auf_netzlaufwerk,
        );
        let beruehrt_arbeitsbaum =
            aufgeloester_ordner
                .as_deref()
                .map_or(Loeschzielbefund::Unentschieden, |ordner| {
                    arbeitsbaum::beruehrt_einen_arbeitsbaum(
                        ordner,
                        zuhause.as_deref(),
                        &auswahl.pfade,
                    )
                });
        let ziel = Loeschziel {
            ordner: aufgeloester_ordner,
            benutzerverzeichnis: zuhause,
            netzlaufwerk,
            arbeitsbaum: beruehrt_arbeitsbaum,
            umfang: umfang::zaehlen(&auswahl.pfade),
        };
        let gruende = loeschwarnung::warngruende(&ziel);
        let (frage, erlaeuterung) =
            loeschwarnung::frage_und_erlaeuterung(auswahl, quellordner, &gruende);
        (frage, erlaeuterung, !gruende.is_empty())
    }

    /// Stellt den bestaetigten Loeschauftrag, und zwar mit **der Auswahl, die
    /// im Blatt stand**.
    ///
    /// **Das ist der Unterschied zu [`Self::auftrag_stellen`], und er ist der
    /// Grund, aus dem es diese Funktion gibt.** Bis zum 260817 rechnete
    /// das endgueltige Loeschen die Auswahl fuer die Frage, und
    /// `auftrag_stellen` las sie nach der Bestaetigung ueber
    /// `betroffene_eintraege()` ein zweites Mal. Zwischen beiden Lesungen steht
    /// das Blatt, und ein Blatt haelt weder FSEvents noch ein fremdes Programm
    /// an: `auffrischung::schiebt_auffrischung_auf` schiebt allein beim
    /// Stapel-Umbenennen auf, sonst zieht eine gemeldete Aenderung den
    /// angezeigten Ordner nach, waehrend der Nutzer noch liest. KRK loeschte
    /// dann etwas anderes, als es gefragt hatte. Genau diese Klasse von Fehler
    /// ist der Anlass dieser Runde, also reist die gezeigte Auswahl mit dem
    /// bestaetigten Auftrag, und es wird kein zweites Mal gelesen.
    ///
    /// **Die Fensterseite darf dagegen hier gelesen werden**, und weil die
    /// Zusage einer zerstoerenden Handlung daran haengt, steht die Begruendung
    /// hier ausgeschrieben statt in ihrer sonst ueblichen Kurzform.
    ///
    /// Solange ein Blatt steht, laesst
    /// [`Self::kommando_ausfuehren`] genau **vier** Kommandos durch und weist
    /// jedes weitere ab: [`Kommando::Abbrechen`] ueber
    /// [`operationen::waehrend_blatt_erlaubt`], dazu
    /// [`Kommando::Beenden`], [`Kommando::FensterSchliessen`] und
    /// [`Kommando::FensterEinblenden`] ueber
    /// [`zulaessigkeit::immer_erreichbar`], das die Blattsperre ausdruecklich
    /// mit aufhebt. Keines der vier aendert die aktive Seite: der Abbruch
    /// schliesst das Blatt, und der Rueckruf laeuft dann mit `bestaetigt ==
    /// false` und stellt keinen Auftrag; [`Self::beenden`] ruft `terminate:`,
    /// [`Self::fenster_schliessen`] ruft `performClose:`, das ein Fenster mit
    /// anhaengendem Blatt nicht schliesst, und [`Self::fenster_zeigen`] ruft
    /// `makeKeyAndOrderFront:` und `activate`. Geschrieben wird `aktiv` von
    /// keinem davon. Ein Blatt ist ausserdem fenstermodal, nimmt der Maus also
    /// ebenfalls den Zugriff. Die aktive Seite kann sich zwischen Frage und
    /// Antwort damit nicht aendern.
    ///
    /// Die Kopf-an-Kopf-Pruefung von Quelle und Ziel aus `auftrag_stellen`
    /// entfaellt: ein Loeschauftrag hat kein Ziel.
    fn loeschauftrag_stellen(&self, art: Art, auswahl: Auswahl, quellordner: PathBuf) {
        let aktiv = self.ivars().modell.borrow().aktiv();
        let positionen = auswahl.zahl();
        let auftrag = Auftrag {
            quellen: auswahl.pfade,
            art,
            konfliktregel: Default::default(),
            uebertragung: Default::default(),
        };
        // `auftrag_starten` liefert immer `true`; hier gibt es niemanden mehr,
        // der die Antwort brauchte, denn der Tastendruck ist laengst verbraucht.
        let _ = self.auftrag_starten(aktiv, auftrag, quellordner, positionen);
    }

    /// Der Abbruchbefehl (C4, C1.7).
    ///
    /// **Drei Raenge, und die Reihenfolge ist bindend.** Ein offenes Blatt
    /// zuerst, weil die Konfliktfrage waehrend eines laufenden Vorgangs steht
    /// und der Abbruch dann ihr gilt. Dann eine laufende Dateioperation. Und
    /// zuletzt der Filtertext des sichtbaren Tabs im aktiven Dateifenster —
    /// genau an der Stelle, an der die Taste bis zum 260815 nichts mehr zu tun
    /// fand und `false` lieferte.
    ///
    /// ```text
    /// esc ──> steht ein Blatt?            ──ja──> es schliessen
    ///          │ nein
    ///          └──> laeuft eine Operation? ──ja──> sie abbrechen
    ///                │ nein
    ///                └──> steht ein Filtertext? ──ja──> ihn loeschen
    ///                      │ nein
    ///                      └──> nichts, wie vor dieser Runde
    /// ```
    ///
    /// **Der dritte Rang haengt an
    /// `decisions/260814-1830_*_an-welcher-stelle-der-bedeutungen-von-esc-steht-der-filtertext.md`.**
    /// Eine andere Antwort verschiebt ihn innerhalb dieser Funktion und aendert
    /// sonst nichts; die Raenge sind hier eine Reihenfolge und keine verstreute
    /// Zustaendigkeit.
    ///
    /// **Ein eigener Rang fuer das Anhalten des Durchlaufs entsteht nicht**
    /// (C3.5). Ohne Filtertext hat der Durchlauf keinen Gegenstand: das
    /// Loeschen beendet ihn, und ein vierter Rang beantwortete dieselbe Frage
    /// ein zweites Mal.
    ///
    /// Seit S16b erreicht `esc` den Vorgang auf dem gewoehnlichen Weg: solange
    /// kein Blatt steht, schlaegt der Ereignisabgriff `abbrechen` wie jeden
    /// anderen Befehl nach. Der Griff, den das Fortschrittsblatt als
    /// Schaltflaeche trug, ist die Taste selbst, und die Vorgangsanzeige nennt
    /// sie in ihrem Text.
    fn abbrechen(&self) -> bool {
        let blatt = self.ivars().offenes_blatt.borrow_mut().take();
        if let Some(blatt) = blatt {
            blatt.abbrechen();
            return true;
        }
        let laufender = {
            let vorgang = self.ivars().vorgang.borrow();
            vorgang.as_ref().map(|vorgang| {
                vorgang.zustand.abbrechen();
                (vorgang.art.clone(), vorgang.seite)
            })
        };
        if let Some((art, seite)) = laufender {
            self.fortschritt_zeigen(seite, &operationen::abbruchzeile(&art));
            return true;
        }
        // Der dritte Rang. `Kommando::Abbrechen` traegt
        // `Wirkungsbereich::Ueberall`, kommt also auch aus dem Editor und aus
        // der Leiste an; getroffen wird dann derselbe Tab wie beim Umschalten
        // der tiefen Suche, naemlich der sichtbare des **aktiven**
        // Dateifensters. Ein Wirkungsbereich, der dafuer den Fokus verlangte,
        // machte die Taste davon abhaengig, wo die Schreibmarke steht.
        let aktiv = self.ivars().modell.borrow().aktiv();
        self.dateifenster(aktiv).quelle().filter_leeren()
    }

    // ------------------------------------------------------------------
    // Anlegen und Umbenennen im Stapel (C4, Schritt 17)
    // ------------------------------------------------------------------

    /// Fragt den Namen und legt danach einen Ordner oder eine leere Datei an
    /// (C4).
    ///
    /// **Ein Weg fuer beide Befehle.** `f7` und `shift+cmd+n` bringen
    /// [`Anlegeart::Ordner`] mit, `ctrl+cmd+n` [`Anlegeart::Datei`]; alles
    /// andere ist dasselbe, bis hinunter zu der Kernfunktion, die den Namen
    /// prueft. Angelegt wird im Ordner des **aktiven** Dateifensters, wie C4 es
    /// sagt.
    ///
    /// Liefert `true`, sobald das Blatt steht: der Tastendruck ist dann
    /// verbraucht.
    fn anlegen(&self, art: Anlegeart) -> bool {
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };
        let seite = self.ivars().modell.borrow().aktiv();
        let ordner = self.dateifenster(seite).quelle().angezeigter_ordner();

        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        namenseingabe::zeigen(
            self.mtm(),
            fenster,
            art.frage(),
            art.bestaetigen(),
            move |ergebnis| {
                if let Some(selbst) = schwach.load() {
                    selbst.anlegen_ausfuehren(seite, art, &ordner, ergebnis);
                }
            },
        );
        true
    }

    /// Legt den Eintrag an, frischt auf und setzt die Auswahl auf ihn (C4).
    ///
    /// Die Reihenfolge ist bindend. Erst [`auffrischung::ordner_neu_lesen`],
    /// der eine Auffrischungspfad aus S14, damit beide Dateifenster den neuen
    /// Eintrag zeigen; dann die Auswahl ueber
    /// [`Dateifenster::quelle`]`.eintrag_waehlen`, die eine Stelle, die eine
    /// Zeile anhand ihres Namens waehlt. Im gewoehnlichen Fall laeuft deren
    /// Lesevorgang zu diesem Zeitpunkt noch, also merkt sie den Namen vor und
    /// springt, sobald er eintrifft: die Antwort ist `Vorgemerkt`.
    ///
    /// **`Unbekannt` ist hier trotzdem erreichbar, und der Rueckgabewert wird
    /// verworfen.** `ordner` und `seite` stehen seit [`Self::anlegen`] fest,
    /// also seit vor dem Blatt, und das Blatt haelt die Datentraegerwache
    /// nicht an: sie ist kein Befehl, und
    /// `beginSheetModalForWindow:completionHandler:` bringt keine eigene
    /// Ereignisschleife mit. Wirft der Nutzer waehrend des stehenden Blattes
    /// den Datentraeger aus, auf dem `ordner` liegt, schiebt
    /// [`auffrischung::datentraeger_verloren`] jeden getroffenen Tab beider
    /// Seiten auf das Benutzerverzeichnis — bei `willUnmount` ist `ordner`
    /// dabei noch beschreibbar, siehe `datentraeger_gewechselt`. Das Anlegen
    /// gelingt dann, [`auffrischung::ordner_neu_lesen`] findet keine Seite
    /// mehr, die `ordner` zeigt, und `eintrag_waehlen` befragt das Modell des
    /// Benutzerverzeichnisses.
    ///
    /// Das ist derselbe Weg, den der Zweig `Art::UmbenennenImStapel` in
    /// `vorgang_beenden` fuer sich beschreibt, nur ausgeloest von der
    /// Datentraegerwache statt von einem Befehl — und er wird auch gleich
    /// behandelt: gemeldet wird nichts.
    fn anlegen_ausfuehren(
        &self,
        seite: Fensterseite,
        art: Anlegeart,
        ordner: &Path,
        ergebnis: Result<String, Namensfehler>,
    ) {
        let name = match ergebnis {
            Ok(name) => name,
            Err(fehler) => {
                self.antwort_zeigen(seite, fehler.grund());
                return;
            }
        };
        let angelegt = match art {
            Anlegeart::Ordner => operation::ordner_anlegen(ordner, &name),
            Anlegeart::Datei => operation::datei_anlegen(ordner, &name),
        };
        if let Err(fehler) = angelegt {
            self.antwort_zeigen(seite, &operationen::anlegefehler(art, &name, &fehler));
            return;
        }

        auffrischung::ordner_neu_lesen(self, ordner);
        // **Bewusst verworfen.** `Unbekannt` ist ueber die Datentraegerwache
        // erreichbar, und gemeldet wird dann nichts; die Begruendung steht im
        // Doc-Kommentar dieser Funktion. So entschieden vom Nutzer am 260810
        // (`issues/260807-0219_*_drei-aufrufer-von-eintrag-waehlen-…`).
        let _ = self.dateifenster(seite).quelle().eintrag_waehlen(&name);
        self.antwort_zeigen(seite, &operationen::angelegt_text(art, &name));
    }

    /// Benennt den Eintrag um, den der Nutzer in der Liste bearbeitet hat (C4).
    ///
    /// Dieselbe Reihenfolge wie beim Anlegen, und aus denselben Gruenden: erst
    /// [`krk_core::operation::umbenennen`] aus S15, dann
    /// [`auffrischung::ordner_neu_lesen`], der eine Auffrischungspfad aus S14,
    /// dann die Auswahl auf den neuen Namen ueber die eine Stelle, die eine
    /// Zeile anhand ihres Namens waehlt. Auch hier steht deren Lesevorgang
    /// noch aus, ihre Antwort ist also `Vorgemerkt` und nie `Unbekannt`.
    ///
    /// **Ob der Name schon vergeben ist, beantwortet das Dateisystem.**
    /// `umbenennen` scheitert dann mit [`io::ErrorKind::AlreadyExists`], und
    /// der Grund geht in die Statuszeile. Eine Vorabprueferei gegen die
    /// gelesene Liste waere eine zweite Wahrheit ueber denselben Ordner.
    fn umbenennen_ausfuehren(&self, seite: Fensterseite, alt: &str, neu: &str) {
        let ordner = self.dateifenster(seite).quelle().angezeigter_ordner();
        if let Err(fehler) = operation::umbenennen(&ordner.join(alt), neu) {
            self.antwort_zeigen(seite, &operationen::umbenennungsfehler(neu, &fehler));
            return;
        }
        auffrischung::ordner_neu_lesen(self, &ordner);
        // **Bewusst verworfen.** Der Lesevorgang der Auffrischung eine Zeile
        // darueber steht noch aus, die Antwort ist also `Vorgemerkt` und nie
        // `Unbekannt`; die Begruendung steht im Doc-Kommentar dieser Funktion.
        let _ = self.dateifenster(seite).quelle().eintrag_waehlen(neu);
    }

    /// Oeffnet das Blatt fuer das Umbenennen im Stapel (C4).
    ///
    /// Der **erste** der beiden Befehle, die C4 verlangt: er zeigt die
    /// Vorschau. Ausgefuehrt wird erst auf den zweiten, die Schaltflaeche
    /// "Umbenennen" des Blattes.
    fn stapel_umbenennen(&self) -> bool {
        let seite = self.ivars().modell.borrow().aktiv();
        let quelle = self.dateifenster(seite).quelle();
        let auswahl = quelle.betroffene_eintraege();
        if auswahl.ist_leer() {
            self.antwort_zeigen(seite, "es ist nichts ausgewählt");
            return true;
        }
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };

        let ordner = quelle.angezeigter_ordner();
        // Die Namen in Sichtreihenfolge: sie bestimmen die Reihenfolge der
        // fortlaufenden Nummer, und `betroffene_eintraege` liefert sie schon in
        // genau dieser Ordnung.
        let markierte: Vec<String> = auswahl
            .pfade
            .iter()
            .filter_map(|pfad| pfad.file_name())
            .map(|name| name.to_string_lossy().into_owned())
            .collect();
        let bestand = quelle.alle_namen();

        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        stapelumbenennen::zeigen(self.mtm(), fenster, markierte, bestand, move |vorschau| {
            if let Some(selbst) = schwach.load() {
                selbst.stapel_beauftragen(seite, &ordner, &vorschau);
            }
        });
        true
    }

    /// Gibt die bestaetigte Vorschau als Auftrag an die Operationsmaschine (C4).
    ///
    /// Der **zweite**, ausdrueckliche Befehl aus C4. Bis S17c lief hier eine
    /// Schleife auf dem Hauptfaden: je Zeile ein
    /// [`krk_core::operation::umbenennen`], ohne Arbeitsfaden, ohne Fortschritt
    /// und ohne Abbruch. Ueber wenige Dutzend Eintraege war das richtig; ueber
    /// 5.000 brauchte es auf dem Referenzgeraet 525 ms, und so lange stand der
    /// Hauptfaden. Das verfehlte zwei Zusagen aus C4 und L9 aus C8,
    /// `issues/260804-2040_*_das-stapel-umbenennen-laeuft-ohne-fortschritt-und-ohne-abbruch-auf-dem-hauptfaden.md`.
    ///
    /// **Die Schwelle haengt an der Zeit und nicht an einer Eintragszahl.** Ein
    /// Stapel ueber 50 Namen ist nach rund 5 ms durch und laesst keine Zeile
    /// aufblitzen; einer ueber 5.000 ueberschreitet die 150 ms aus
    /// [`operationen::ANZEIGEVERZUG`] und zeigt seinen Fortschritt in derselben
    /// Statuszeile wie Kopieren und Verschieben.
    ///
    /// Eine Zeile mit Hinweis kommt gar nicht erst in den Auftrag. Wie viele das
    /// waren, sagt der Abschlusstext ueber seine beiden Zahlen: umbenannte
    /// Eintraege und bestaetigte Positionen.
    fn stapel_beauftragen(&self, seite: Fensterseite, ordner: &Path, vorschau: &Vorschau) {
        if self.vorgang_laeuft_schon(seite) {
            return;
        }
        let paare: Vec<(PathBuf, String)> = vorschau
            .auszufuehren()
            .map(|zeile| (ordner.join(&zeile.alt), zeile.neu.clone()))
            .collect();
        if paare.is_empty() {
            self.antwort_zeigen(seite, "nichts umzubenennen: jede Zeile trägt einen Hinweis");
            return;
        }
        let auftrag = Auftrag::umbenennen_im_stapel(paare);
        self.auftrag_starten(
            seite,
            auftrag,
            ordner.to_path_buf(),
            vorschau.zeilen().len(),
        );
    }

    /// Wo der Eingabefokus steht (C5).
    ///
    /// **Eine Frage, eine Antwort, und AppKit gibt sie.** Der Ersthelfer des
    /// Fensters ist die Wahrheit ueber den Fokus; ein eigenes Feld daneben
    /// waere eine zweite, die jeder Mausklick in eine der drei Listen
    /// nachzuziehen haette. Die beiden Fokusbefehle aus C5 setzen deshalb den
    /// Ersthelfer, statt ein Kennzeichen umzulegen.
    ///
    /// Eine Vorabfrage und ein Durchgang durch [`Bereich::ALLE`]. Steht ein
    /// Blatt am Fenster, ist dessen Panel das Schluesselfenster und nicht das
    /// Hauptfenster: [`Fokus::Anderswo`], und ohne diese Antwort loeschte ein
    /// Delete vor der stehenden Rueckfrage in dem Ordner dahinter. Dieselbe
    /// Antwort gilt fuer jedes fremde Fenster und fuer ein KRK im Hintergrund.
    ///
    /// **Gefragt ist seit S43 das Enthaltensein und nicht mehr die
    /// Naemlichkeit.** Nicht "**ist** der Ersthelfer diese Ansicht", sondern
    /// "**liegt** er in diesem Bereich". Der Durchgang laeuft ueber
    /// [`Bereich::ALLE`], holt zu jedem Wert die Wurzelansicht ueber
    /// [`Aufteilung::bereichssicht`] und fragt `isDescendantOf:`; von
    /// [`Bereich`] auf [`Fokus`] kommt die erschoepfende Zuordnung
    /// [`fokus::in_bereich`](crate::kommandos::fokus::in_bereich). Die fuenf
    /// Teilbaeume sind zueinander fremd, weil es die fuenf Unteransichten einer
    /// `NSSplitView` sind; ein Ersthelfer liegt deshalb in hoechstens einem,
    /// und der erste Treffer ist die Antwort.
    ///
    /// **Was sich damit am Verhalten aendert, ist genau ein Fall**, und es ist
    /// der des Defekts
    /// `issues/260809-1738_*_der-rueckfall-in-fokus-antwortet-dateifenster-fuer-jede-unteransicht-eines-randbereichs.md`:
    /// ein Ersthelfer innerhalb der Leiste, der Vorschau oder des Editors, der
    /// nicht deren eine genannte Ansicht ist — eine Bildlaufleiste etwa —,
    /// wandert von [`Fokus::Dateifenster`] auf seinen eigenen Bereich. Fuer den
    /// Feldeditor eines Textfeldes im Dateifenster lautet die Antwort vorher
    /// wie nachher `Dateifenster`: er ist eine Unteransicht des Dateifensters,
    /// und der Rueckfall antwortete fuer ihn schon bisher so. Die
    /// Enthaltensfrage aendert an jener Stelle keine einzige Antwort.
    ///
    /// **[`Self::fokusansicht`] bleibt und beantwortet die andere Frage:**
    /// welche Ansicht den Ersthelferrang **annehmen** soll, wenn KRK den Fokus
    /// setzt. Der Rang gehoert genau einer Ansicht, das Enthaltensein gilt fuer
    /// einen ganzen Teilbaum; beide Fragen brauchen ihre eigene Antwort, und
    /// keine ist die Verdopplung der anderen.
    ///
    /// **Der Rueckfall auf [`Fokus::Dateifenster`] bleibt und traegt danach
    /// genau einen Fall:** einen Ersthelfer, der in **keiner** der fuenf
    /// Unteransichten liegt, also das Fenster selbst, die Aufteilung oder den
    /// Titelbalken. `Anderswo` an dieser Stelle hiesse, dass dann **kein**
    /// Befehl des Dateifensters mehr wirkt; genau diesen Zustand hat der Defekt
    /// vom 260805-1845 schon einmal hergestellt.
    fn fokus(&self) -> Fokus {
        self.fokus_bei(self.schluesselfenster())
    }

    /// Dasselbe, mit einem schon erhobenen Schluesselfenster.
    ///
    /// **Die Aufteilung dient der einen Erhebung und keiner zweiten Frage.**
    /// [`Self::lage`] braucht das Schluesselfenster ohnehin, weil es der vierte
    /// Bestandteil der Zulaessigkeitsregel ist; riefe es danach [`Self::fokus`],
    /// fragte AppKit denselben Augenblick ein zweites Mal, und die beiden
    /// Antworten koennten auseinanderlaufen. [`Self::fokus`] bleibt fuer die
    /// fuenf uebrigen Aufrufer stehen, die den Wert nicht schon in der Hand
    /// haben.
    ///
    /// Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig:
    /// allein vor dem Hauptfenster selbst lohnt der Gang durch den
    /// Ansichtsbaum.
    fn fokus_bei(&self, schluesselfenster: Schluesselfenster) -> Fokus {
        match schluesselfenster {
            Schluesselfenster::Hauptfenster => self.ersthelferbereich(),
            Schluesselfenster::BlattAmHauptfenster | Schluesselfenster::Fremd => Fokus::Anderswo,
        }
    }

    /// In welchem Bereich der Ersthelfer des Hauptfensters liegt.
    ///
    /// **Die zweite Haelfte von [`Self::fokus`], und sie beantwortet eine
    /// eigene Frage.** `fokus` fragt: "wohin geht ein Befehl **jetzt**", und
    /// dazu gehoert die Vorabfrage nach dem Schluesselfenster, denn vor einem
    /// stehenden Blatt darf kein Befehl des Dateifensters wirken. Diese
    /// Funktion fragt: "wo liegt der Ersthelfer", und die Antwort darauf haengt
    /// nicht daran, ob das Fenster gerade vorn steht.
    ///
    /// **Der Unterschied ist keine Feinheit, sondern das achte Abnahmekriterium
    /// von C9.** Geht KRK in den Hintergrund, gibt es kein Schluesselfenster
    /// mehr, und `fokus` antwortet [`Fokus::Anderswo`]. Die Anzeige soll dann
    /// aber gerade **zuruecktreten** und nicht stehen bleiben, also braucht sie
    /// die Antwort, die der Hintergrund nicht aendert. Der Ersthelfer selbst
    /// wechselt beim Wechsel in den Hintergrund nicht; macOS haelt ihn.
    ///
    /// Der Durchgang selbst steht in [`Self::bereich_des_ersthelfers`]; diese
    /// Funktion ist nur noch seine Uebersetzung in einen Fokuswert. **Ein
    /// Ersthelfer, der in keinem der fuenf Bereiche liegt, gilt weiter als
    /// Dateifenster**, und das ist der Rueckfall, den es seit S43 gibt: das
    /// Fenster selbst etwa traegt den Rang, bevor der Aufbau den Fokus gesetzt
    /// hat.
    fn ersthelferbereich(&self) -> Fokus {
        match self.bereich_des_ersthelfers() {
            Some(bereich) => fokus::in_bereich(bereich),
            None => Fokus::Dateifenster,
        }
    }

    /// In welchem der fuenf Bereiche der Ersthelfer liegt, wenn er in einem
    /// liegt.
    ///
    /// **Die eine Stelle, die den Ersthelferrang auf einen Bereich abbildet**,
    /// und sie hat zwei Frager mit verschiedenen Fragen.
    /// [`Self::ersthelferbereich`] macht daraus den Fokuswert fuer die Anzeige
    /// und die Zulaessigkeitsregel; [`Self::aktives_dem_ersthelfer_nachziehen`]
    /// braucht den Bereich selbst, weil der Fokuswert die beiden Dateifenster
    /// zusammenwirft und gerade die Unterscheidung zwischen ihnen die gesuchte
    /// Auskunft ist.
    ///
    /// Der Durchgang laeuft ueber [`Bereich::ALLE`] und fragt `isDescendantOf:`
    /// gegen die Wurzelansicht jedes Bereichs; die Begruendung fuer den
    /// Enthaltensschnitt steht an [`Self::fokus`]. `None` heisst dreierlei und
    /// laeuft absichtlich zusammen: es gibt noch kein Fenster, der Rang liegt
    /// bei niemandem, oder sein Traeger ist keine Ansicht und kann damit in
    /// keinem Teilbaum liegen. Kein Frager unterscheidet die drei.
    fn bereich_des_ersthelfers(&self) -> Option<Bereich> {
        let haupt = self.ivars().fenster.get()?;
        let ersthelfer = haupt.firstResponder()?;
        // Allein eine Ansicht kann in einem Teilbaum liegen. Ein Ersthelfer,
        // der keine ist — das Fenster selbst etwa —, faellt hier durch.
        let ansicht = ersthelfer.downcast_ref::<NSView>()?;
        let aufteilung = self.ivars().aufteilung.get()?;
        Bereich::ALLE.into_iter().find(|bereich| {
            aufteilung
                .bereichssicht(*bereich)
                .is_some_and(|wurzel| ansicht.isDescendantOf(&wurzel))
        })
    }

    /// Baut den Auftrag aus der Auswahl des aktiven Dateifensters und startet
    /// ihn.
    ///
    /// Liefert `true`, auch wenn nichts ausgewaehlt war: der Tastendruck ist
    /// dann verbraucht, und die Statuszeile sagt warum. Ihn weiterzureichen
    /// hiesse, dass F5 auf leerer Auswahl in der Menueleiste landet.
    fn auftrag_stellen(&self, art: Art) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();
        if self.vorgang_laeuft_schon(aktiv) {
            return true;
        }

        let quelle = self.dateifenster(aktiv).quelle();
        let auswahl = quelle.betroffene_eintraege();
        if auswahl.ist_leer() {
            self.antwort_zeigen(aktiv, "es ist nichts ausgewählt");
            return true;
        }
        let quellordner = quelle.angezeigter_ordner();
        if art.eq(&Art::Kopieren {
            ziel: quellordner.clone(),
        }) || art.eq(&Art::Verschieben {
            ziel: quellordner.clone(),
        }) {
            self.antwort_zeigen(aktiv, "Quelle und Ziel sind derselbe Ordner");
            return true;
        }

        let positionen = auswahl.zahl();
        let auftrag = Auftrag {
            quellen: auswahl.pfade,
            art,
            konfliktregel: Default::default(),
            uebertragung: Default::default(),
        };
        self.auftrag_starten(aktiv, auftrag, quellordner, positionen)
    }

    /// Ob KRK gerade einen Vorgang haelt, und welcher Art er ist (C4).
    ///
    /// **Die eine Stelle, die diese Frage beantwortet**, und sie meldet nichts.
    /// KRK haelt genau einen Vorgang; wer wissen will, ob ein weiterer
    /// anfangen darf, liest hier `ivars().vorgang` und nirgends sonst. Eine
    /// zweite Prueferei waeren zwei Antworten auf dieselbe Frage.
    ///
    /// **Vier Wege fragen, und sie teilen sich in drei und einen.** Drei folgen
    /// einem Tastendruck und nehmen deshalb den meldenden Mantel
    /// [`Self::vorgang_laeuft_schon`], denn auf einen Tastendruck gehoert eine
    /// Antwort in die Statuszeile: die vier Befehle aus der Auswahl
    /// ([`Self::auftrag_stellen`]), das Stapel-Umbenennen
    /// ([`Self::stapel_beauftragen`]) und die Vorstufe der Loeschrueckfrage
    /// ([`Self::loeschen_nach_rueckfrage`], die von der Antwort allein braucht,
    /// **ob** es einen Vorgang gibt).
    ///
    /// Der vierte ist der Abwurf aus einer fremden Anwendung (C6 der Runde 13),
    /// und er nimmt die Frage **ohne** die Meldung. Der Grund ist der Ort seines
    /// Fragers: `validateDrop:` laeuft bei jeder Zeigerbewegung, und eine
    /// Meldung von dort schriebe die Statuszeile mehrmals je Sekunde voll. Was
    /// der Abwurf stattdessen zeigt, ist der Zeiger selbst.
    ///
    /// Der vierte Weg ist damit **keine zweite Pruefung**, sondern dieselbe ohne
    /// ihre Nebenwirkung.
    ///
    /// `#[must_use]`, weil das stille Fallenlassen des Rueckgabewerts unbemerkt
    /// bliebe: diese Funktion hat seit dem Wegfall ihrer Nebenwirkung gar keine
    /// mehr, ein nackter Aufruf taete also nichts, und was dabei verlorenginge,
    /// ist die Antwort auf die Lage 1 aus C6 — es liefe ein zweiter Vorgang an,
    /// waehrend einer laeuft, und nichts wuerde rot. `unused_results` ist
    /// erlaubt, dieses Attribut ist deshalb die einzige Sperre.
    #[must_use = "die Antwort ist die Lage 1 aus C6; fallengelassen faengt ein zweiter Vorgang an, waehrend einer laeuft"]
    fn vorgang_laeuft(&self) -> Option<Art> {
        self.ivars()
            .vorgang
            .borrow()
            .as_ref()
            .map(|vorgang| vorgang.art.clone())
    }

    /// Meldet einen bereits laufenden Vorgang und sagt, ob deshalb nichts
    /// startet (C4).
    ///
    /// Der meldende Mantel um [`Self::vorgang_laeuft`]; die Frage selbst und
    /// ihre vier Wege stehen dort. Die Meldung geht als **Befehlsantwort** an
    /// das Dateifenster, in dem der Nutzer die Taste gedrueckt hat, und steht
    /// damit auch dann in der Zeile, wenn genau dieses Fenster den laufenden
    /// Vorgang begonnen hat. Bis zum 260804-1915 war sie eine Fenstermeldung und
    /// verschwand im haeufigen Fall hinter dem eigenen Fortschritt,
    /// `issues/260804-1915_*_der-zweite-operationsbefehl-meldet-sich-im-fenster-des-vorgangs-unsichtbar.md`.
    fn vorgang_laeuft_schon(&self, seite: Fensterseite) -> bool {
        // Die Ausleihe endet in `vorgang_laeuft` und damit vor dem Aufruf
        // darunter: `antwort_zeigen` geht nach AppKit hinein.
        let Some(laufende_art) = self.vorgang_laeuft() else {
            return false;
        };
        self.antwort_zeigen(seite, &operationen::schon_ein_vorgang(&laufende_art));
        true
    }

    /// Gibt einen angenommenen Abwurf an die Operationsmaschine (C4 bis C6 der
    /// Runde 13).
    ///
    /// **Der vierte Rufer von [`Self::auftrag_starten`]**, neben den vier
    /// Befehlen aus der Auswahl ([`Self::auftrag_stellen`]), dem
    /// Stapel-Umbenennen ([`Self::stapel_beauftragen`]) und dem bestaetigten
    /// Loeschen ([`Self::loeschauftrag_stellen`]). Der Plan der Runde 13 nennt
    /// ihn den dritten und zaehlt in seinem eigenen Abschnitt „Current State"
    /// drei vorhandene Rufer; gezaehlt wird hier gegen den Baum. Er steht
    /// daneben und nicht in `auftrag_stellen`: jenes nimmt seine Quellen aus der
    /// Auswahl des **aktiven** Dateifensters, waehrend ein Abwurf fremde Pfade
    /// mitbringt und ein Ziel, das nicht das aktive Dateifenster sein muss.
    ///
    /// **Er fragt nicht, ob schon ein Vorgang laeuft.** Gefragt hat
    /// `DateifensterQuelle::abwurf_pruefen` in `validateDrop:`, ueber
    /// [`Self::vorgang_laeuft`], und AppKit ruft `acceptDrop:` allein dann, wenn
    /// `validateDrop:` einen Vorgang zurueckgegeben hat. Eine Nachfrage hier
    /// waere die zweite Antwort auf dieselbe Frage, gegen die
    /// [`Self::vorgang_laeuft`] geschrieben ist.
    ///
    /// **Der Auftrag entsteht ueber [`Auftrag::kopieren`] und
    /// [`Auftrag::verschieben`]** und nicht ueber ein Strukturliteral: die
    /// beiden Erzeuger fuellen `konfliktregel` und `uebertragung` aus ihrer
    /// Vorgabe, und ein Literal daneben muesste beide Felder ein zweites Mal
    /// nennen. Eine eigene Auftragsart bringt der Abwurf nicht mit; er muendet
    /// in dieselben zwei Arten wie F5 und F6.
    ///
    /// **`seite` ist das Dateifenster, ueber dem der Zeiger stand**, und es
    /// reist bis in den [`Vorgang`] mit: Fortschritt, Abschlusstext und
    /// Konfliktantwort erscheinen dort und nicht im aktiven Dateifenster.
    ///
    /// **Als Quellordner geht das Ziel mit**, und das ist eine bewusste
    /// Fuellung: der Ordner, aus dem gezogen wurde, gehoert einer fremden
    /// Anwendung, und KRK zeigt ihn nicht notwendig an. Wo er doch in einem der
    /// beiden Dateifenster steht, zieht ihn die Dateisystemwache nach; ein
    /// Abwurf schiebt die Auffrischung nicht auf
    /// ([`auffrischung::schiebt_auffrischung_auf`] laesst allein das
    /// Stapel-Umbenennen aufschieben).
    ///
    /// Der Preis steht dazu: [`Vorgang::ordner`] nennt den Zielordner damit
    /// zweimal, und der Abschluss liest ihn zweimal. Beide Laeufe kommen am
    /// selben Ergebnis heraus — der zweite `neu_lesen` liest den Auswahlnamen
    /// aus dem noch stehenden Bestand und merkt ihn erneut als
    /// `wunschauswahl` vor, und die Generationszaehlung des
    /// [`krk_core::verzeichnis::Ordnermodell`] laesst die Stapel des ersten
    /// Laufs fallen. Was bleibt, ist ein ueberzaehliger Verzeichnisdurchgang je
    /// Abwurf; er ist als
    /// `issues/260818-2221_*_the-drop-passes-its-target-as-the-source-folder-and-the-completion-reads-it-twice.md`
    /// gefilt.
    fn abwurf_ausfuehren(
        &self,
        seite: Fensterseite,
        ziel: PathBuf,
        quellen: Vec<PathBuf>,
        art: Abwurfvorgang,
    ) {
        let positionen = quellen.len();
        let auftrag = match art {
            Abwurfvorgang::Kopieren => Auftrag::kopieren(quellen, &ziel),
            Abwurfvorgang::Verschieben => Auftrag::verschieben(quellen, &ziel),
        };
        // Der Rueckgabewert sagt "der Tastendruck ist verbraucht", und hier gab
        // es keinen: ein Abwurf ist eine Mausbewegung.
        let _ = self.auftrag_starten(seite, auftrag, ziel, positionen);
    }

    /// Startet einen fertigen Auftrag auf der Operationsmaschine.
    ///
    /// Der gemeinsame Teil aller vier Wege hinein: Arbeitsfaden ueber
    /// [`krk_core::operation::starten`], Vermittlerfaden fuer die Meldungen und
    /// der [`Vorgang`], an dem der Hauptfaden ihn wiederfindet. Liefert immer
    /// `true`: der Tastendruck ist verbraucht, gleich ob der Faden zustande kam.
    fn auftrag_starten(
        &self,
        seite: Fensterseite,
        auftrag: Auftrag,
        quellordner: PathBuf,
        positionen: usize,
    ) -> bool {
        let art = auftrag.art.clone();
        // Hier bekommt die Schnittstelle aus `operation/loeschen.rs` ihre
        // Implementierung: bis zu diesem Aufruf hatte sie im laufenden Programm
        // keine.
        let lauf = operation::starten(auftrag, Arc::new(Systempapierkorb));

        // Der Griff an das Abbruchkennzeichen bleibt beim Hauptfaden, der Lauf
        // geht an den Vermittlerfaden. Beide zeigen auf dasselbe Kennzeichen;
        // siehe `krk_core::operation::Abbruchgriff`.
        let zustand = Arc::new(Vorgangszustand::neu(lauf.abbruchgriff()));
        let fuer_faden = Arc::clone(&zustand);
        let gestartet = thread::Builder::new()
            .name("krk-vermittler".to_owned())
            .spawn(move || vermitteln(lauf, &fuer_faden));
        if let Err(fehler) = gestartet {
            // Der Lauf ist mit `gestartet` gefallen und damit abgebrochen; er
            // hat noch nichts angefasst.
            self.antwort_zeigen(
                seite,
                &format!("die Operation liess sich nicht starten: {fehler}"),
            );
            return true;
        }

        *self.ivars().vorgang.borrow_mut() = Some(Vorgang {
            art,
            seite,
            quellordner,
            positionen,
            begonnen: Instant::now(),
            zustand,
        });
        true
    }

    /// Der Weckruf des Vermittlerfadens, auf dem Hauptfaden angekommen.
    ///
    /// Der Weg dorthin geht ueber die Hauptschlange und den Anwendungsdelegierten
    /// von `NSApplication`, damit der Weckruf selbst nichts festhalten muss, was
    /// dem Hauptfaden gehoert.
    fn vorgang_einziehen(mtm: MainThreadMarker) {
        let Some(delegierter) = NSApplication::sharedApplication(mtm).delegate() else {
            return;
        };
        let Ok(selbst) = delegierter.downcast::<Anwendungsdelegierter>() else {
            return;
        };
        selbst.vorgang_zeichnen();
    }

    /// Zeichnet den Stand des laufenden Vorgangs.
    ///
    /// **Die Reihenfolge ist bindend** und im Modulkopf von
    /// [`crate::kommandos::operationen`] begruendet: erst `gezeichnet`, dann
    /// den Stand lesen, dann zeichnen. Umgekehrt fiele eine Meldung, die
    /// waehrend des Zeichnens eintrifft, zwischen die beiden Schritte.
    fn vorgang_zeichnen(&self) {
        // Die Ausleihe endet vor jedem AppKit-Aufruf: ein Blatt ruft zurueck,
        // und der Rueckruf will denselben `RefCell`.
        let Some((zustand, art, seite, positionen, begonnen)) = ({
            let vorgang = self.ivars().vorgang.borrow();
            vorgang.as_ref().map(|vorgang| {
                (
                    Arc::clone(&vorgang.zustand),
                    vorgang.art.clone(),
                    vorgang.seite,
                    vorgang.positionen,
                    vorgang.begonnen,
                )
            })
        }) else {
            return;
        };

        zustand.buendelung.gezeichnet();
        let (fortschritt, konflikt, bericht) = zustand.aendern(|stand| {
            (
                stand.fortschritt.clone(),
                stand.konflikt.take(),
                stand.bericht.take(),
            )
        });

        if let Some(bericht) = bericht {
            self.vorgang_beenden(&bericht);
            return;
        }
        if let Some(konflikt) = konflikt {
            self.konflikt_fragen(konflikt);
            return;
        }
        if !operationen::anzeige_faellig(begonnen, Instant::now()) {
            return;
        }
        self.fortschritt_zeigen(
            seite,
            &operationen::vorgangszeile(&art, fortschritt.as_ref(), positionen),
        );
    }

    /// Schreibt den Stand des Vorgangs in die Statuszeile seines
    /// Dateifensters (C4).
    ///
    /// Eine Zeile erscheint ohne Einblendung mit dem naechsten
    /// Zeichendurchgang. Genau das macht L8 haltbar: ein Blatt brauchte auf dem
    /// Referenzgeraet 354 bis 403 ms bis zum Anhaengen, und die Zusage lautet
    /// 200 ms.
    fn fortschritt_zeigen(&self, seite: Fensterseite, stand: &str) {
        self.dateifenster(seite).quelle().vorgang_zeigen(stand);
    }

    /// Stellt die Konfliktfrage aus C4 und schickt die Antwort zurueck.
    ///
    /// Die Vorgangsanzeige bleibt dabei stehen: sie ist eine Zeile am Fuss des
    /// Dateifensters und kein zweites Blatt, das AppKit hinter dieses stellen
    /// muesste. Bis S16 wich hier ein Fortschrittsblatt.
    fn konflikt_fragen(&self, frage: Konfliktfrage) {
        let Some(fenster) = self.ivars().fenster.get() else {
            return;
        };

        let vorschlag = freier_name(&frage.ziel);
        let antwortweg = frage.antwort.clone();
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let griff = konflikt::zeigen(
            self.mtm(),
            fenster,
            &frage.quelle,
            &frage.ziel,
            &vorschlag,
            move |entscheid| {
                // Ein leerer Name waere kein Name; dann bleibt der Eintrag
                // stehen, statt unter einem Namen zu landen, den niemand
                // getippt hat. Die Pruefung im Kern faenge ihn ebenfalls ab und
                // meldete ihn als uebersprungen; hier ist sie naeher am Nutzer.
                let entscheid = match &entscheid.antwort {
                    Konfliktantwort::UmbenennenIn(name) if name.is_empty() => Konfliktentscheid {
                        antwort: Konfliktantwort::Ueberspringen,
                        fuer_alle_weiteren: false,
                    },
                    _ => entscheid,
                };
                let _ = antwortweg.send(entscheid);
                if let Some(selbst) = schwach.load() {
                    *selbst.ivars().offenes_blatt.borrow_mut() = None;
                }
            },
        );
        *self.ivars().offenes_blatt.borrow_mut() = Some(griff);
    }

    /// Schliesst den Vorgang ab: Anzeige weg, Meldung, Auffrischung, Liste.
    ///
    /// **Die Meldung geht an das Dateifenster, das den Vorgang begonnen hat**,
    /// und nicht an das gerade aktive: der Nutzer darf waehrend der Operation
    /// gewechselt haben, und der Abschlusstext gehoert zu der Zeile, in der der
    /// Fortschritt stand.
    ///
    /// **Der Abschlusstext ist eine Befehlsantwort und keine Fenstermeldung.**
    /// Er ist die, spaet eintreffende, Antwort auf das F5, mit dem der Nutzer
    /// die Operation gestartet hat, und faellt deshalb mit dem naechsten
    /// Tastenbefehl statt beim naechsten Ordnerwechsel. Zwei Folgen: er
    /// ueberschreibt keine waehrend der Operation eingetroffene
    /// Auswurfmeldung mehr, die deshalb einen Tastendruck spaeter zu sehen ist,
    /// und er ueberlebt weiterhin die Auffrischung unten, weil eine
    /// Auffrischung kein Tastenbefehl ist.
    fn vorgang_beenden(&self, bericht: &Bericht) {
        let Some(vorgang) = self.ivars().vorgang.borrow_mut().take() else {
            return;
        };
        // Erst die Vorgangsanzeige wegnehmen, dann den Abschlusstext setzen.
        // Die Reihenfolge ist inzwischen die eines aufgeraeumten Zustands und
        // keine Bedingung fuer die Sichtbarkeit: eine Befehlsantwort stuende
        // ohnehin ueber der Vorgangsanzeige. Eine stehengebliebene
        // Vorgangsanzeige waere aber nach dem naechsten Tastendruck wieder da,
        // obwohl die Operation vorbei ist.
        self.dateifenster(vorgang.seite).quelle().vorgang_beenden();
        self.antwort_zeigen(
            vorgang.seite,
            &operationen::abschlusstext(&vorgang.art, bericht, vorgang.positionen),
        );

        // **Der eine Auffrischungspfad.** Der gemeldete Abschluss einer
        // Dateioperation ist der zweite Ausloeser von `ordner_neu_lesen`, den
        // S14 angelegt und `### Frage 3` zugesagt hat. Ein eigener Weg fuer die
        // selbst verursachte Aenderung entsteht nicht.
        //
        // Diese Auffrischung ist zugleich die, die die Dateisystemwache
        // waehrend des Vorgangs ausgesetzt hat; `Vorgang::ordner` ist fuer
        // beide dieselbe Aufzaehlung.
        for ordner in vorgang.ordner() {
            auffrischung::ordner_neu_lesen(self, &ordner);
        }
        match &vorgang.art {
            // Nach einem Stapel-Umbenennen steht die Auswahl auf dem ersten
            // neuen Namen, so wie sie nach dem Anlegen auf dem angelegten
            // Eintrag steht. Der Name kommt aus dem Auftrag selbst und braucht
            // kein eigenes Feld.
            //
            // Die Auffrischung eine Zeile darueber laeuft im selben synchronen
            // Aufruf, ihr Lesevorgang steht also noch aus. `eintrag_waehlen`
            // merkt den Namen deshalb vor, statt eine Zeile des alten Bestands
            // zu waehlen — bei einer Umnummerierung nach oben stuende
            // `neue_namen[0]` dort naemlich schon, und der erste Stapel raeumte
            // die Auswahl gleich darauf ersatzlos weg
            // (`issues/260807-0800_*_eintrag-waehlen-trifft-den-noch-nicht-abgeloesten-bestand-…`).
            // Scheiterte gerade die Umbenennung auf diesen Namen, findet ihn
            // der Abschluss des Lesevorgangs nicht, und die Auswahl bleibt
            // leer, wie C9 es zulaesst.
            Art::UmbenennenImStapel { neue_namen } => {
                if let Some(erster) = neue_namen.first() {
                    // **Der Rueckgabewert wird hier bewusst verworfen.**
                    // `eintrag_waehlen` hat fuenf Aufrufer, und zwei von ihnen
                    // werten `Auswahlversuch::Unbekannt` aus: `eintrag_anspringen`
                    // (C10) meldet den fehlenden Namen, und die Messhandlung
                    // `Auswaehlen` macht daraus einen Abbruchgrund des
                    // Messlaufs. Die drei uebrigen verwerfen ihn, und
                    // erreichbar ist er von ihnen hier und in
                    // `anlegen_ausfuehren`; in `umbenennen_ausfuehren` nicht,
                    // weil dort der Ordner unmittelbar vor der Auffrischung aus
                    // derselben Seite kommt. Der Vorgang laeuft im
                    // Hintergrund, und wechselt der Nutzer waehrenddessen den
                    // Ordner dieser Seite, frischt die Schleife darueber sie
                    // nicht auf; dann laeuft kein Lesevorgang, und
                    // `eintrag_waehlen` befragt das Modell des anderen
                    // Ordners. Gemeldet wird trotzdem nichts, anders als in
                    // `eintrag_anspringen`, wo der Nutzer eben nach diesem
                    // Namen gefragt hat: "«datei-1» steht nicht in der Liste"
                    // traefe ihn hier in einem Ordner, ueber den er gerade
                    // nichts wissen wollte, und waere eher Rauschen als
                    // Auskunft. So entschieden vom Nutzer am 260810
                    // (`issues/260807-0219_*_drei-aufrufer-von-eintrag-waehlen-…`).
                    let _ = self
                        .dateifenster(vorgang.seite)
                        .quelle()
                        .eintrag_waehlen(erster);
                }
            }
            Art::Kopieren { .. } | Art::Verschieben { .. } | Art::InDenPapierkorb => {}
        }

        // **Der vierte Anlass fuer die Gueltigkeitsmarke der Lesezeichen (C5).**
        // Ein abgeschlossener Vorgang kann den Ordner eines Lesezeichens
        // beseitigt oder angelegt haben. Bis zum 260806 blieb die Marke danach
        // auf dem Stand des letzten der drei uebrigen Anlaesse stehen: der
        // Nutzer loeschte den Ordner in KRK selbst, sah die Leiste an und fand
        // den Eintrag unveraendert schwarz
        // (`issues/260805-1730_*_die-gueltigkeit-eines-lesezeichens-veraltet-zwischen-zwei-anlaessen.md`).
        //
        // **Warum hier und nicht in der Dateisystembeobachtung.** Der gemeldete
        // Fall ist das Loeschen in KRK selbst, und C9 haelt bereits fest, dass
        // eine abgeschlossene Dateioperation die Auffrischung von sich aus
        // anstoesst. Der Anlass haengt sich an dieselbe Stelle und kostet damit
        // keinen neuen Mechanismus, keine erweiterte Pfadliste in
        // `auffrischung::sichtbare_ordner` und kein Neuaufsetzen des
        // FSEvents-Stroms bei jeder Lesezeichenaenderung. Der Weg ueber die
        // Beobachtung deckte zusaetzlich das fremde Programm ab, greift auf
        // Netzpfaden nach C9 ohnehin nicht und waere ein zweiter Mechanismus
        // fuer eine Marke.
        //
        // **Was offen bleibt.** Loescht ein **fremdes** Programm den Ordner,
        // steht die Marke weiterhin bis zur naechsten Auswahl falsch. Die Zusage
        // aus C5 haelt auch dann, weil die Auswahl den Grund immer meldet.
        //
        // **Auch nach einem Teilabbruch.** Diese Stelle wird ebenso erreicht,
        // wenn der Lauf abgebrochen wurde: der Abbruch traegt seinen Bericht
        // ueber `abbruch_ohne_meldung_nachtragen` nach und laeuft dieselbe Bahn.
        // Ein teilweise geloeschter Ordner ist entweder fort oder noch da, und
        // beides will die Marke wissen.
        self.leiste().quelle().gueltigkeit_nachziehen();

        let Some((frage, liste)) = operationen::uebersprungenliste(&bericht.uebersprungen) else {
            return;
        };
        let Some(fenster) = self.ivars().fenster.get() else {
            return;
        };
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let griff = uebersprungen::zeigen(self.mtm(), fenster, &frage, &liste, move || {
            if let Some(selbst) = schwach.load() {
                *selbst.ivars().offenes_blatt.borrow_mut() = None;
            }
        });
        *self.ivars().offenes_blatt.borrow_mut() = Some(griff);
    }

    /// Stellt die Antwort auf einen Tastenbefehl in die Statuszeile des
    /// genannten Dateifensters.
    ///
    /// Rang 1, der oberste der sechs Raenge, siehe
    /// [`crate::appkit::statuszeile::zeile`]. Nicht zu verwechseln mit
    /// [`Dateifenstersicht::melden`] weiter unten: das ist der Weg der
    /// Ereignisse, die niemand angefordert hat, und der schreibt die
    /// Fenstermeldung auf Rang 3.
    fn antwort_zeigen(&self, seite: Fensterseite, text: &str) {
        self.dateifenster(seite)
            .quelle()
            .befehlsantwort_zeigen(text);
    }

    /// **Die eine Stelle, an der der Delegierte den Editor eine Datei aufnehmen
    /// laesst.**
    ///
    /// Vier Wege fuehren hierher: [`Self::im_editor_oeffnen`], das `f4` und seit
    /// dem 260823 auch `cmd+e` in der Dateiliste nehmen,
    /// [`Self::editor_aus_vorschau`] fuer `cmd+e` in der Vorschau, der Sprung
    /// auf eine Textmarke aus C6 ([`Self::textmarke_anspringen`]) und die
    /// Wiederherstellung der Sitzung beim Start
    /// ([`Self::editor_wiederherstellen`]). Jeder von ihnen **nennt seine
    /// Herkunft**, weil sie ein Pflichtargument ist; ein fuenfter Weg, der sie
    /// nicht nennt, uebersetzt nicht.
    ///
    /// **Es sind vier Wege und fuenf Tasten.** Der Rundweg aus dem
    /// Nutzerentscheid vom 260823-0942 hat keinen eigenen bekommen: er verteilt
    /// sich auf die beiden ersten, statt einen dritten daneben zu stellen.
    ///
    /// **Das ist der Gewinn gegenueber dem billigeren Weg**, die Marke an den
    /// Befehlswegen zu loeschen: der haette eine Zusage auf drei Aufrufstellen
    /// verteilt, und die erste vergessene faende keine Pruefung. Hier gibt es
    /// nichts zu vergessen — wer nichts sagt, kommt nicht durch.
    ///
    /// **Diese Stelle buendelt und erzwingt nicht mehr.** Erzwungen wird die
    /// Angabe seit dem 260810 von [`Editorbereich::datei_oeffnen`], das sie als
    /// Pflichtargument nimmt und mit dem Ausgang zurueckgibt; damit gilt die
    /// Zusage fuer das ganze Programm und nicht mehr nur bis zur Grenze dieser
    /// Datei. Der Datensatz dazu ist
    /// `issues/260810-1028_*_die-herkunft-eines-oeffnens-ist-im-delegierten-erzwungen-und-nicht-am-editorbereich.md`.
    /// Diese Funktion bleibt, weil sie die zweite Frage beantwortet, die kein
    /// Aufrufer doppelt stellen soll: ob es den Editorbereich ueberhaupt schon
    /// gibt.
    ///
    /// **Der Rueckgabewert sagt, ob es den Editorbereich gibt** und nicht, ob die
    /// Datei angenommen wurde: das entscheidet
    /// `krk_core::text::datei::oeffnen` auf dem Arbeitsfaden, und der Ausgang
    /// kommt in [`Self::editorausgang_behandeln`] an. `false` heisst allein, dass
    /// die Oberflaeche noch nicht steht; die beiden Befehle darunter reichen den
    /// Wert weiter, und er entscheidet dort ueber [`Self::aufteilung_nachziehen`]
    /// und [`Self::sitzung_vormerken`] und ueber sonst nichts. **Der Tastendruck
    /// ist auch dann verbraucht**, weil [`Self::kommando_ausfuehren`] seit der
    /// Runde 7 immer `true` liefert; dieselbe Auskunft steht dreissig Zeilen
    /// weiter unten am Leerweg von [`Self::im_editor_oeffnen`].
    ///
    fn editor_oeffnen_lassen(&self, pfad: &Path, herkunft: Oeffnungsherkunft) -> bool {
        let Some(editor) = self.ivars().editor.get() else {
            return false;
        };
        editor.datei_oeffnen(pfad, herkunft);
        true
    }

    /// Den ausgewaehlten Eintrag des aktiven Dateifensters im eingebauten Editor
    /// oeffnen (C2).
    ///
    /// Der erste der beiden Einstiegswege, und **er hat seit dem 260823 zwei
    /// Tasten**: `f4` traegt ihn seit der Editor-Runde, `cmd+e` in der
    /// Dateiliste seit dem Nutzerentscheid vom 260823-0942 ueber
    /// [`Self::editor_rundweg`]. Es ist derselbe Rumpf und keine Kopie daneben;
    /// die beiden koennen deshalb nicht auseinanderlaufen.
    ///
    /// **Die Reihenfolge ist bindend und
    /// steht im elften Abnahmekriterium von C2: erst die Pruefung, dann die
    /// Flaeche.** Eine Datei, die der Editor ohnehin abweist, blendet ihn nicht
    /// ein, verdraengt die Vorschau nicht und kostet den Nutzer spaeter keine
    /// Rueckfrage.
    ///
    /// **Geprueft wird an der einen Stelle** und hier keine zweite Regel
    /// daneben: `krk_core::text::datei::oeffnen` entscheidet, ob der Editor
    /// eine Datei annimmt, weist alles Nichttextliche und alles ueber 16 MB ab
    /// und einen Ordner sicher. Diese Funktion liest die Datei nicht und
    /// beurteilt sie nicht; sie reicht den Pfad hinein und den Grund heraus.
    ///
    /// **Ein Zwischenstand ist geblieben, einer ist gefallen.** Gelesen wird
    /// seit S24 auf dem Arbeitsfaden, und deshalb steht der Ausgang nicht mehr
    /// hier, sondern in [`Self::editorausgang_behandeln`]. Geblieben ist, dass
    /// ein ungesicherter Stand beim Wechsel auf eine andere Datei ohne
    /// Rueckfrage faellt: die Nachfrage aus C4 kommt mit ihrem eigenen Schritt.
    fn im_editor_oeffnen(&self) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();
        let Some(pfad) = self.dateifenster(aktiv).quelle().auswahl_pfad() else {
            // Kein Eintrag, also nichts, was der Editor annehmen oder abweisen
            // koennte — keine Abweisung und deshalb keine `Editormeldung`,
            // sondern derselbe Satz, den KRK seit der Runde 1 fuer die leere
            // Auswahl fuehrt. Fuer den Loeschweg fuehrt ihn seit dem 260817
            // `loeschen_nach_rueckfrage` und nicht mehr `endgueltig_loeschen`,
            // das ihn an jenen gemeinsamen Rumpf abgegeben hat; daneben stehen
            // `auftrag_stellen` und `stapel_umbenennen`. `true` verbraucht den
            // Tastendruck, aus demselben Grund wie dort: F4 auf leerer Auswahl
            // gehoert nicht in die Menueleiste.
            self.antwort_zeigen(aktiv, "es ist nichts ausgewählt");
            return true;
        };
        self.editor_oeffnen_lassen(&pfad, Oeffnungsherkunft::Befehl)
    }

    /// `cmd+e` mit dem Fokus in der Vorschau: die dort angezeigte Datei im
    /// eingebauten Editor oeffnen (C2).
    ///
    /// Der zweite der beiden Einstiegswege, festgelegt vom Nutzer am
    /// 260807-2139 und am 260823-0942 unveraendert beibehalten: er ist seither
    /// die mittlere Zeile des Rundwegs und wird ueber
    /// [`Self::editor_rundweg`] erreicht statt unmittelbar aus
    /// [`Self::kommando_ausfuehren`].
    ///
    /// **Er nimmt die Datei aktiv mit.** Der Editor verdraengt die Vorschau
    /// nach C1, sobald er die Flaeche bekommt; ein Uebergang, der die Datei nur
    /// stehen liesse, verloere sie mit dem Verschwinden der Vorschau. Der Pfad
    /// wird deshalb hier abgeschrieben, **bevor** irgendetwas an der
    /// Sichtbarkeit geschieht.
    ///
    /// **Kein zweiter Weg und keine zweite Regel.** Geoeffnet wird ueber
    /// [`Editorbereich::datei_oeffnen`] wie bei F4, und geprueft damit von
    /// `krk_core::text::datei::oeffnen`, der einen Stelle, die entscheidet, ob
    /// der Editor eine Datei annimmt. Was auf das Oeffnen folgt — Einblenden,
    /// Fokus, Titel, Abweisungsmeldung und die Nachfrage aus C4 beim Wechsel auf
    /// eine andere Datei — steht in [`Self::editorausgang_behandeln`] und im
    /// Modell; dieser Weg erbt alles davon, ohne eine Zeile dafuer zu
    /// schreiben, und stellt insbesondere **keine** zweite Abfrage des
    /// ungesicherten Standes daneben. Sie stuende vor der Pruefung und
    /// verletzte damit das elfte Abnahmekriterium von C2.
    ///
    /// **Dass dieser Rumpf allein aus der Vorschau erreicht wird, traegt die
    /// Regel [`rundweg`]** und keine Abfrage hier; der Wirkungsbereich
    /// `Wirkungsbereich::Dateibereiche` laesst `cmd+e` seit dem 260823 auch aus
    /// der Dateiliste und aus dem Editor durch, und welcher der drei Ruempfe
    /// dann laeuft, entscheidet [`Self::editor_rundweg`]. Was bleibt, ist der
    /// Fall, den weder der Wirkungsbereich noch die Regel abdeckt: die Vorschau
    /// steht im Fokus
    /// und zeigt trotzdem keine Datei, naemlich den Inhalt der Zwischenablage
    /// aus C10 der Runde 1 oder gar nichts. Dann liefert `angezeigter_pfad`
    /// `None`, und der Grund geht in die Statuszeile — kommentarlos nichts zu
    /// tun ist in keinem Fall zulaessig.
    fn editor_aus_vorschau(&self) -> bool {
        let Some(pfad) = self.vorschau().angezeigter_pfad() else {
            // Derselbe Weg wie bei F4 auf leerer Auswahl: es gibt keine Datei,
            // ueber die der Editor etwas zu melden haette, also `antwort_zeigen`
            // und keine `Editormeldung`.
            let aktiv = self.ivars().modell.borrow().aktiv();
            self.antwort_zeigen(aktiv, "die Vorschau zeigt keine Datei zum Bearbeiten");
            return true;
        };
        self.editor_oeffnen_lassen(&pfad, Oeffnungsherkunft::Befehl)
    }

    /// Oeffnet beim Start die Datei wieder, die die Sitzung gemerkt hat (C7).
    ///
    /// **Derselbe eine Weg wie die beiden Einstiege aus C2**, also dieselbe
    /// Pruefung aus `krk_core::text::datei::oeffnen`. Eine Datei, die inzwischen
    /// verschwunden oder zu gross geworden ist, wird abgewiesen wie an jedem
    /// anderen Tag; der Editor bleibt dann leer, wird ausgeblendet, und der
    /// Grund steht in der Statuszeile.
    ///
    /// **Die Sichtbarkeit kommt aus der Sitzung und nicht von hier.**
    /// `Fenstermodell::aus_sitzung` hat sie oben schon gesetzt; ein
    /// ausgeblendeter Editor mit gehaltener Datei ist der Zustand, den der
    /// Fokusbefehl aus C1 hervorholt, und ihn hier einzublenden hiesse, die
    /// gemerkte Sichtbarkeit zu uebergehen.
    fn editor_wiederherstellen(&self, sitzung: &Sitzung) {
        let Some(pfad) = sitzung.editor.as_ref() else {
            return;
        };
        self.editor_oeffnen_lassen(pfad, Oeffnungsherkunft::Sitzung);
    }

    /// Was auf einen Ladevorgang des Editors folgt (C2, C7, C11).
    ///
    /// **Die eine Behandlung fuer beide Zeitpunkte.** Sie kommt aus dem
    /// Rueckruf, den [`Self::oberflaeche_aufbauen`] eingetragen hat, und sie
    /// laeuft entweder sofort — wenn der Editor die Datei schon hielt — oder
    /// beim naechsten Einzugstakt, wenn der Arbeitsfaden geliefert hat. Der
    /// Aufrufer ist in beiden Faellen derselbe, und deshalb steht die
    /// Fallunterscheidung einmal.
    ///
    /// Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig; ein
    /// vierter Ausgang haelt den Bau an.
    ///
    /// **Zwei ihrer Zweige fragen, wer das Oeffnen angefordert hat**, und die
    /// Antwort kommt als Argument herein. Die Wiederherstellung aus der Sitzung
    /// ist kein Befehl: sie holt keinen Fokus, weil der beim Start in das aktive
    /// Dateifenster gehoert, und ihre Abweisung ist die Antwort auf keinen
    /// Tastendruck.
    ///
    /// **Die Antwort gehoert zu dem Oeffnen, dessen Ausgang hier ankommt**, und
    /// dafuer sorgt seit dem 260810 kein Feld mehr, sondern der Weg selbst: die
    /// [`Oeffnungsherkunft`] ist ein Pflichtargument von
    /// [`Editorbereich::datei_oeffnen`] und kommt mit dem Ausgang durch den
    /// Melder zurueck. Bis dahin lag sie als Feld beim Delegierten und wartete
    /// auf ihren Verbrauch; zuerst setzte allein die Wiederherstellung, und ein
    /// F4 in der Spanne bis zum Ausgang erbte ihre Marke, dann blieben Fokus,
    /// Titel und Sitzungsschreiben fuer die Datei des Nutzers aus
    /// (`issues/260810-0418_*_ein-f4-waehrend-der-wiederherstellung-erbt-die-marke-aus-sitzung.md`,
    /// danach
    /// `issues/260810-1028_*_die-herkunft-eines-oeffnens-ist-im-delegierten-erzwungen-und-nicht-am-editorbereich.md`).
    ///
    /// **Der Sprung auf eine Textmarke haengt hier an** (C6), und zwar an
    /// derselben Stelle wie der Fokus: nach dem gelungenen Oeffnen und nachdem
    /// der Editor hervorgeholt ist, damit die angesprungene Zeile auch im Bild
    /// steht. Die vorgemerkte Stelle wird oben herausgenommen und damit
    /// verbraucht; wer sie zurueckstellt, sagt es an seinem Zweig.
    ///
    /// **Und das Sitzungsschreiben aus C7 haengt hier**, aus demselben Grund
    /// wie der Titel: erst mit diesem Ausgang haelt der Editor die neue Datei,
    /// und vorher gefragt nennt [`Self::editordatei`] die vorige. Ein Aufruf je
    /// Oeffnungsweg entsteht dafuer nicht — alle drei laufen durch diese eine
    /// Stelle.
    ///
    /// **Sie ist eine Fortsetzung und kein Befehl, und deshalb beginnt sie wie
    /// [`Self::kommando_ausfuehren`] mit dem Nachlesen der Breiten.** Jeder
    /// Zweig unten aendert die Sichtbarkeit — der erste ueber
    /// [`Self::fokus_holen`], der letzte ueber [`Self::editor_ausblenden`] —,
    /// und [`Self::sichtbarkeit_aendern`] schreibt sie von dort auf den Schirm.
    /// Was der Nutzer waehrend des Lesens mit der Maus verschoben hat, stuende
    /// ohne diese Zeile im Rahmen der Ansicht und nicht im Fenstermodell, und
    /// der Nachzug naehme ihm die Ziehbewegung. **Der Zeitpunkt ist der
    /// richtige**: gemessen wird, bevor irgendetwas die Sichtbarkeit anfasst,
    /// also solange Modell und Schirm dieselbe meinen — die Bedingung, die
    /// [`Self::bildschirmbreiten_uebernehmen`] an ihren Aufrufer stellt.
    fn editorausgang_behandeln(&self, ausgang: Ladeausgang, herkunft: Oeffnungsherkunft) {
        self.bildschirmbreiten_uebernehmen();
        let aus_sitzung = herkunft.ist_aus_sitzung();
        let marke = self.ivars().vorgemerkte_marke.borrow_mut().take();
        match ausgang {
            // Einblenden und Fokus in einem Zug: `fokus_holen` holt den Bereich
            // hervor und setzt danach den Ersthelfer. Der gegenseitige
            // Ausschluss aus C1 nimmt dabei die Vorschau von der Flaeche, ohne
            // dass diese Funktion sie nennt. Damit steht der Eingabefokus im
            // Editor, ohne dass der Nutzer einen zweiten Befehl braucht, wie es
            // das zweite Abnahmekriterium von C2 verlangt.
            // `SchonOffen` steht neben `Geoeffnet` und nicht in einem eigenen
            // Zweig: was hier zu tun bleibt, ist in beiden Faellen dasselbe,
            // naemlich den Editor hervorzuholen und den Fokus hineinzusetzen.
            // Genau das braucht der Nutzer, der die Vorschau eingeblendet und
            // damit den Editor nach C1 verdraengt hat. Verschieden sind die
            // beiden allein beim Modell und bei der Textflaeche, und diese
            // Funktion fasst weder das eine noch die andere an; der Unterschied
            // steht in `Editorbereich::einziehen`.
            Ladeausgang::Geoeffnet | Ladeausgang::SchonOffen => {
                // Der Editor haelt jetzt moeglicherweise eine Datei in einem
                // Ordner, den bisher niemand beobachtet hat; C4 will fremde
                // Aenderungen daran gemeldet haben. `SchonOffen` steht mit im
                // Zweig und aendert dabei nichts: die Liste kommt gleich
                // heraus, und eine Abfrage daneben waere eine zweite Stelle mit
                // einer Meinung darueber, wann sich der beobachtete Bestand
                // aendert.
                self.dateisystemwache_nachziehen();
                // Beim Start bleibt beides, wie es ist: der Fokus steht im
                // aktiven Dateifenster (`fokus::BEIM_START`), und der Titel
                // folgt ihm. Ein wiederhergestellter Editor draengt sich nicht
                // vor.
                if !aus_sitzung {
                    self.fokus_holen(Fokus::Editor);
                    // Der zweite der vier Anlaesse aus C11: der Editor haelt
                    // eine andere Datei als eben noch. Der Fokusnachzug allein
                    // genuegt hier nicht in jedem Fall — steht der Fokus schon
                    // im Editor, meldet `makeFirstResponder:` keinen Wechsel,
                    // weil keiner stattfindet, und der Titel truege weiter die
                    // vorige Datei.
                    self.titel_nachziehen(self.fokus());
                    // **Der Anlass, an dem die gemerkte Datei aus C7 nachzieht,
                    // und alle drei Oeffnungswege gehen ueber ihn.**
                    // `kommando_ausfuehren` hat zwar auch schon vorgemerkt, als
                    // F4 oder der Uebergang aus der Vorschau lief, aber zu
                    // frueh: gelesen wird seit S24 auf dem Arbeitsfaden, und
                    // `editordatei()` antwortet aus dem Modell, das erst mit
                    // diesem Ausgang nachzieht — vorgemerkt wurde damals also
                    // die vorige Datei. Ohne diese Zeile stand die neue in
                    // keiner `session.toml`, bis irgendein spaeterer Anlass
                    // zufaellig eine schrieb, und ein Absturz davor liess den
                    // Editor beim naechsten Start leer
                    // (`issues/260810-0240_*_ein-oeffnen-im-editor-stoesst-kein-sitzungsschreiben-an.md`).
                    //
                    // **Sie steht in diesem Zweig und nicht davor.** Die
                    // Wiederherstellung aus der Sitzung schriebe zurueck, was
                    // sie eben gelesen hat: derselbe Pfad, dieselbe
                    // Sichtbarkeit. Der Schreibvorgang haette nichts zu
                    // melden, faende beim Start `zuletzt == None` vor und ginge
                    // deshalb sofort auf die Platte.
                    self.sitzung_vormerken();
                }
                // Zuletzt, weil `scrollRangeToVisible:` einen Bereich ins Bild
                // holt und der Editor dafuer auf dem Schirm stehen muss. Er ist
                // es nach `fokus_holen` — und beim Start ohnehin, wenn die
                // Sitzung ihn sichtbar hatte; von dort kommt allerdings nie eine
                // vorgemerkte Marke.
                if let Some((zeile, zeileninhalt)) = marke {
                    self.textmarke_ausfuehren(zeile, &zeileninhalt);
                }
            }
            // **Der zweite Anlass der Nachfrage aus C4.** Die Datei ist an
            // dieser Stelle gelesen und geprueft, und der Editor hat sie noch
            // nicht aufgenommen; damit steht die Pruefung vor der Nachfrage, wie
            // das elfte Abnahmekriterium von C2 es verlangt. Der Weg zurueck
            // geht ueber `Anlass::AndereDatei` und endet in `Geoeffnet`, das
            // gleich darueber behandelt wird — dieser Zweig holt weder Fokus
            // noch Titel nach, weil beides dann von selbst hier ankommt.
            Ladeausgang::Zurueckgehalten => {
                // Die vorgemerkte Stelle wartet mit der zurueckgehaltenen Datei:
                // sie gehoert genau zu ihr, und der Weg zurueck aus der
                // Nachfrage endet in `Geoeffnet`, wo sie gebraucht wird. Ihr
                // Gegenstueck steht in `anlass_unterbleibt`, wo die
                // zurueckgehaltene Datei fallengelassen wird.
                *self.ivars().vorgemerkte_marke.borrow_mut() = marke;
                self.nachfrage_zeigen(Anlass::AndereDatei);
            }
            // Kommentarlos nichts zu tun ist in keinem Fall zulaessig: der
            // Grund geht in die Statuszeile und unterscheidet dort zu gross von
            // nicht als Text lesbar (zehntes Abnahmekriterium von C2).
            Ladeausgang::Abgewiesen(abweisung) if !aus_sitzung => {
                self.editormeldung_zeigen(&Editormeldung::Abgewiesen(abweisung));
            }
            // Beim Start ist die Abweisung die Antwort auf keinen Tastendruck,
            // sondern ein Ereignis am Fenster: die gemerkte Datei ist fort oder
            // zu gross geworden, waehrend KRK nicht lief. Sie geht deshalb als
            // Fenstermeldung auf Rang 3 der Statuszeile; auf Rang 1 loeschte
            // der erste Tastendruck sie weg, bevor der Nutzer sie gelesen hat.
            //
            // Der Editor wird dabei ausgeblendet und nicht bloss leer gelassen:
            // hatte die Sitzung ihn sichtbar, naehme er den Dateifenstern sonst
            // Platz fuer nichts — dieselbe Begruendung, aus der
            // `Sichtbarkeit::default` ihn ausgeblendet ausliefert.
            Ladeausgang::Abgewiesen(abweisung) => {
                self.editor_ausblenden();
                let aktiv = self.ivars().modell.borrow().aktiv();
                self.dateifenster(aktiv)
                    .quelle()
                    .meldung_zeigen(&abweisung.meldung());
            }
        }
    }

    /// Setzt die Schreibmarke auf die vorgemerkte Stelle und meldet, falls es
    /// etwas zu melden gibt (C6).
    ///
    /// **Der Sprung, der kommentarlos nichts tut, entsteht nicht.** Wurde der
    /// gemerkte Zeileninhalt weder auf seiner Nummer noch im Fenster daneben
    /// gefunden, fuehrt die Marke trotzdem an die gemerkte Nummer, und der
    /// Nutzer erfaehrt es in der Statuszeile; das achte Abnahmekriterium von C6
    /// verlangt es, weil er erkennen koennen muss, dass er an einer ungeprueften
    /// Stelle gelandet ist. Wohin gesprungen wird und wann gemeldet, entscheidet
    /// `krk_core::text::marke` ueber
    /// [`Editorbereich::marke_anspringen`](super::editor::Editorbereich::marke_anspringen).
    fn textmarke_ausfuehren(&self, zeile: u32, zeileninhalt: &str) {
        let Some(editor) = self.ivars().editor.get() else {
            return;
        };
        if let Some(meldung) = editor.marke_anspringen(zeile, zeileninhalt) {
            self.editormeldung_zeigen(&meldung);
        }
    }

    /// `cmd+s` schreibt den Stand des Editors in seine Datei (C4).
    ///
    /// **Die Fallunterscheidung ueber den Ausgang steht hier einmal**, so wie
    /// die ueber den Ladeausgang in [`Self::editorausgang_behandeln`]. Sie ist
    /// vollstaendig und hat keinen Auffangzweig; ein vierter Ausgang haelt den
    /// Bau an.
    ///
    /// **Beide Ausgaenge, die eine Datei betreffen, gehen ueber
    /// [`Editormeldung`]** und damit in die eine Meldeflaeche des Fensters auf
    /// Rang 1. Der dritte betrifft keine Datei, weil es keine gibt; er nimmt
    /// denselben Weg wie F4 auf leerer Auswahl, naemlich [`Self::antwort_zeigen`]
    /// mit einem eigenen Satz. Eine Variante in [`Editormeldung`] entsteht dafuer
    /// nicht — sie meldet ueber die gehaltene Datei, und hier haelt der Editor
    /// keine. Erreichbar ist der Zweig ohnehin kaum: `Wirkungsbereich::Editor`
    /// laesst den Befehl nur mit dem Fokus in der Textflaeche durch, und die
    /// bekommt ihn nur mit einer Datei.
    ///
    /// **Was der Editor haelt, ist zu diesem Zeitpunkt der getippte Stand.**
    /// `textDidChange:` schreibt ihn seit S26 bei jeder Aenderung ins Modell
    /// zurueck; diese Funktion holt nichts aus der Textflaeche und braucht es
    /// nicht.
    ///
    /// **Ein gelungenes Sichern nimmt der Statuszeile keinen anderen Rang
    /// weg**, und ein gescheitertes wirft nichts weg: der Stand bleibt im
    /// Modell stehen, das Abweichungszeichen am Kopf bleibt sichtbar, und der
    /// Nutzer kann es nach dem Grund erneut versuchen. Das neunte
    /// Abnahmekriterium von C4 verlangt genau das.
    fn editor_sichern(&self) -> bool {
        if self.ivars().editor.get().is_none() {
            return false;
        }
        self.editor_stand_sichern();
        true
    }

    /// Sichert und meldet; liefert, ob der Stand jetzt in der Datei steht.
    ///
    /// **Zwei Aufrufer, eine Fallunterscheidung.** `cmd+s` fragt nicht nach dem
    /// Rueckgabewert, die Nachfrage aus C4 schon: das neunte Abnahmekriterium
    /// von C4 verlangt, dass ein Anlass unterbleibt, wenn die Sicherung
    /// gescheitert ist, statt den Stand mitzunehmen. Genau das ist dieser `bool`
    /// — und er ist die eine Stelle, an der der Ausgang gelesen wird; eine
    /// zweite Fehlerbehandlung an der Nachfrage entsteht nicht.
    ///
    /// **`NichtsGehalten` liefert `true`.** Ohne gehaltene Datei gibt es keinen
    /// Stand, den ein Anlass verlieren koennte, also darf er laufen. Erreichbar
    /// ist der Zweig aus der Nachfrage heraus nicht: sie steht nur, wenn der
    /// Editor ungesicherten Stand haelt, und den haelt er nur mit einer Datei.
    fn editor_stand_sichern(&self) -> bool {
        let Some(editor) = self.ivars().editor.get() else {
            return false;
        };
        match editor.sichern() {
            Sicherungsausgang::Gesichert(pfad) => {
                self.editormeldung_zeigen(&Editormeldung::Gesichert { pfad });
                true
            }
            Sicherungsausgang::Gescheitert(grund) => {
                self.editormeldung_zeigen(&Editormeldung::SichernGescheitert { grund });
                false
            }
            Sicherungsausgang::NichtsGehalten => {
                let aktiv = self.ivars().modell.borrow().aktiv();
                self.antwort_zeigen(aktiv, "der Editor hält keine Datei");
                true
            }
        }
    }

    // ------------------------------------------------------------------
    // Der Zeilensprung, die Suche und das Ersetzen (C5)
    // ------------------------------------------------------------------

    /// Fuehrt einen Editorbefehl aus, der genau eine Meldung liefert.
    ///
    /// **Die eine Stelle fuer die vier Befehle ohne Blatt** — Weitersuchen,
    /// rueckwaerts Weitersuchen, Ersetzen und Alle ersetzen. Sie haben denselben
    /// Zuschnitt: ein Ruf in den Editor, eine Meldung in die Statuszeile, und
    /// der Tastendruck ist verbraucht. Vier gleichlautende Funktionen daneben
    /// waeren vier Gelegenheiten, den Zuschnitt verschieden zu schreiben.
    ///
    /// Was der Befehl tut, entscheidet der Editor, und ob er ueberhaupt etwas
    /// tun kann, ebenfalls: laeuft keine Suche, kommt die Meldung darueber
    /// zurueck. Diese Funktion stellt keine zweite Vorbedingung daneben.
    fn editorbefehl(&self, tun: fn(&Editorbereich) -> Editormeldung) -> bool {
        let Some(editor) = self.ivars().editor.get() else {
            return false;
        };
        let meldung = tun(editor);
        self.editormeldung_zeigen(&meldung);
        true
    }

    /// `cmd+j`: fragt nach einer Zeilennummer und springt dorthin (C5).
    ///
    /// Das Blatt fragt, der Editor springt. **Die Regel fuer eine Nummer ueber
    /// der Zeilenzahl steht in `krk_core::text::zeilen`** und wird weder hier
    /// noch im Editor nachgebaut; der Sprung fuehrt dann an das Dateiende und
    /// meldet den Grund.
    ///
    /// Solange das Blatt steht, gilt der Fokusvorbehalt des Ereignisabgriffs
    /// unveraendert: Ersthelfer ist der Feldeditor des Textfeldes, und die
    /// Befehle des Editors wirken dort nicht. Das siebte Abnahmekriterium von
    /// C7 faellt daraus an.
    fn editor_zeile_springen(&self) -> bool {
        let Some(fenster) = self.editorblatt_moeglich() else {
            // Kein Blatt, und der Grund steht schon in der Statuszeile. `true`
            // verbraucht den Tastendruck trotzdem: der Befehl traegt
            // `Wirkungsbereich::Editor` und erreicht diese Stelle nur mit dem
            // Fokus in der Textflaeche, gehoert also dem Editor und nicht der
            // Menueleiste. Derselbe Grund wie bei F4 auf leerer Auswahl.
            return true;
        };
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        blaetter::zeilennummer::zeigen(self.mtm(), &fenster, move |eingabe| {
            let Some(selbst) = schwach.load() else {
                return;
            };
            let Some(editor) = selbst.ivars().editor.get() else {
                return;
            };
            if let Some(meldung) = editor.zeile_anspringen(&eingabe) {
                selbst.editormeldung_zeigen(&meldung);
            }
        });
        true
    }

    /// `cmd+f`: fragt nach Such- und Ersatztext und beginnt die Suche (C5).
    ///
    /// **Ein Blatt fuer beide Texte**, und der Ersatztext bleibt danach beim
    /// Editor stehen: `shift+cmd+r` und `ctrl+cmd+r` setzen ihn ein, ohne ein
    /// zweites Mal zu fragen. Der Grund steht im Modulkopf von
    /// [`super::blaetter::suche`].
    ///
    /// Die beiden Startwerte kommen aus
    /// [`Editorbereich::suchtexte`](super::editor::Editorbereich::suchtexte)
    /// und nicht aus einem Feld hier: was zuletzt gesucht wurde, weiss der
    /// Suchlauf im Modell, und ein zweiter Vorrat daneben liefe davon weg.
    fn editor_suchen(&self) -> bool {
        // Dieselbe Vorbedingung und derselbe Rueckgabewert wie beim
        // Zeilensprung darueber; die Begruendung steht dort.
        let (Some(fenster), Some(editor)) =
            (self.editorblatt_moeglich(), self.ivars().editor.get())
        else {
            return true;
        };
        let (gesucht, ersatz) = editor.suchtexte();
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        blaetter::suche::zeigen(
            self.mtm(),
            &fenster,
            &gesucht,
            &ersatz,
            move |gesucht, ersatz| {
                let Some(selbst) = schwach.load() else {
                    return;
                };
                let Some(editor) = selbst.ivars().editor.get() else {
                    return;
                };
                let meldung = editor.suche_beginnen(&gesucht, &ersatz);
                selbst.editormeldung_zeigen(&meldung);
            },
        );
        true
    }

    /// Das Fenster, an dem ein Eingabeblatt des Editors haengen kann (C5).
    ///
    /// **Die eine Vorbedingung der beiden Blattbefehle**, und sie stellt genau
    /// zwei Fragen: steht ein Fenster, und haelt der Editor eine Datei. Ohne
    /// Datei gibt es weder eine Zeile, in die gesprungen werden koennte, noch
    /// einen Text, in dem zu suchen waere; gemeldet wird derselbe Satz, den
    /// [`Self::editor_stand_sichern`] fuer diesen Fall seit S25 fuehrt.
    ///
    /// `None` heisst: kein Blatt. Ob der Tastendruck damit verbraucht ist,
    /// entscheidet der Aufrufer — er ist es, sobald es einen Editor gibt, weil
    /// dann eine Antwort in der Statuszeile steht.
    fn editorblatt_moeglich(&self) -> Option<Retained<NSWindow>> {
        let (Some(fenster), Some(editor)) = (self.ivars().fenster.get(), self.ivars().editor.get())
        else {
            return None;
        };
        if !editor.haelt_datei() {
            let aktiv = self.ivars().modell.borrow().aktiv();
            self.antwort_zeigen(aktiv, "der Editor hält keine Datei");
            return None;
        }
        Some(fenster.retain())
    }

    // ------------------------------------------------------------------
    // Die Nachfrage vor den drei Anlaessen (C4)
    // ------------------------------------------------------------------

    /// Ob der Editor Aenderungen haelt, die nicht in seiner Datei stehen (C4).
    ///
    /// Die eine Abfrage dafuer; ohne gebauten Editor ist die Antwort `false`.
    fn editor_haelt_ungesicherten_stand(&self) -> bool {
        self.ivars()
            .editor
            .get()
            .is_some_and(|editor| editor.hat_ungesicherten_stand())
    }

    /// Beginnt einen der Anlaesse aus C4 und stellt die Nachfrage, falls noetig.
    ///
    /// Der Weg der Anlaesse, deren Vorbedingung allein der ungesicherte Stand
    /// ist. Von den dreien geht heute nur das Schliessen des Editors hier
    /// durch; die beiden anderen gehen vorbei und haben je einen Grund dafuer:
    /// das Beenden muss AppKit eine Antwort zurueckgeben und faehrt seinen
    /// eigenen Vorbehalt in [`Self::beenden_erlauben`]; der Wechsel auf eine
    /// andere Datei hat seine Vorbedingung schon im Modell geprueft und kommt
    /// als [`Ladeausgang::Zurueckgehalten`] herein. Beide nehmen danach dieselbe
    /// [`Self::nachfrage_zeigen`].
    ///
    /// Liefert, ob der Tastendruck verbraucht ist — was er in beiden Faellen
    /// ist, sobald es einen Editor gibt: entweder der Anlass ist gelaufen, oder
    /// das Blatt steht.
    fn anlass_beginnen(&self, anlass: Anlass) -> bool {
        if !self.editor_haelt_ungesicherten_stand() {
            self.anlass_ausfuehren(anlass);
            return true;
        }
        self.nachfrage_zeigen(anlass)
    }

    /// Zeigt die Nachfrage aus C4 und laesst den Anlass in der Schliessung
    /// mitreisen.
    ///
    /// **Die eine Aufrufstelle des Blattes**, fuer alle drei Anlaesse. Der
    /// Anlass wird in die Schliessung **hineinkopiert** und steht in keinem
    /// Feld: ein Feld, das eine noch nicht ausgefuehrte Absicht ueber den
    /// Rueckruf hinaus haelt, waere die zweite Wahrheit darueber, was gerade
    /// beantwortet wird, und ueberlebte einen Rueckruf, der ausbleibt.
    ///
    /// Die Schliessung haelt den Anwendungsdelegierten **schwach**, wie alle
    /// bestehenden Blattaufrufer; der Ring Delegierter → Blatt → Rueckruf →
    /// Delegierter schloesse sich sonst. Der [`Blattgriff`] geht nach
    /// `offenes_blatt`, damit `esc` das Blatt wie jede andere Rueckfrage
    /// schliesst, und der Rueckruf leert ihn als erstes.
    ///
    /// Liefert `false`, wenn kein Blatt zu zeigen ist, weil Fenster oder Editor
    /// fehlen. Der Aufrufer entscheidet dann selbst, was daraus folgt.
    fn nachfrage_zeigen(&self, anlass: Anlass) -> bool {
        let (Some(fenster), Some(editor)) = (self.ivars().fenster.get(), self.ivars().editor.get())
        else {
            return false;
        };
        // Genannt wird die Datei, deren Stand auf dem Spiel steht, also die
        // gehaltene — nicht die, die der Editor aufnehmen soll.
        let Some(pfad) = editor.pfad() else {
            return false;
        };
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let griff = ungesichert::zeigen(self.mtm(), fenster, &pfad, move |antwort| {
            let Some(selbst) = schwach.load() else {
                return;
            };
            *selbst.ivars().offenes_blatt.borrow_mut() = None;
            selbst.nachfrage_beantworten(anlass, antwort);
        });
        *self.ivars().offenes_blatt.borrow_mut() = Some(griff);
        true
    }

    /// Was auf die Antwort des Nutzers folgt (C4).
    ///
    /// Die Fallunterscheidung ueber die drei Wahlmoeglichkeiten steht hier
    /// einmal und nicht je Anlass. Bei "sichern" entscheidet der Ausgang des
    /// Schreibens, ob der Anlass laeuft: ein gescheitertes Sichern hat seinen
    /// Grund schon gemeldet, und der Anlass unterbleibt, statt den Stand
    /// mitzunehmen (neuntes Abnahmekriterium von C4).
    fn nachfrage_beantworten(&self, anlass: Anlass, antwort: Antwort) {
        match antwort {
            Antwort::Sichern => {
                if self.editor_stand_sichern() {
                    self.anlass_ausfuehren(anlass);
                } else {
                    self.anlass_unterbleibt(anlass);
                }
            }
            Antwort::Verwerfen => self.anlass_ausfuehren(anlass),
            Antwort::Abbrechen => self.anlass_unterbleibt(anlass),
        }
    }

    /// Fuehrt den Anlass aus, nachdem er zulaessig geworden ist (C4).
    ///
    /// Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig; ein
    /// vierter Wert haelt hier und in [`Self::anlass_unterbleibt`] den Bau an
    /// und erzwingt beide Antworten.
    fn anlass_ausfuehren(&self, anlass: Anlass) {
        match anlass {
            Anlass::EditorSchliessen { vorschau_danach } => {
                self.editor_ausblenden();
                // Der Rueckweg des Rundwegs holt die Vorschau zurueck;
                // `opt+cmd+e` laesst die Flaeche leer. Bedingungslos, und
                // warum, steht am Feld `vorschau_danach`: der Rueckweg endet
                // immer in derselben Lage, gleich wo er begonnen hat.
                //
                // **Die Zeile steht hinter dem Ausblenden und nicht davor.**
                // `editor_ausblenden` setzt ueber `nach_dem_sichtbarkeitswechsel`
                // den Fokus in die Dateiliste, und `bereich_einblenden` laesst
                // ihn dort: es holt einen Bereich hervor und setzt keinen Fokus.
                // Umgekehrt verdraengte die eingeblendete Vorschau den Editor,
                // und das Ausblenden danach traefe einen Bereich, der schon weg
                // ist.
                //
                // Die Abweisung bleibt stumm, wie ueberall in dieser Datei: eine
                // Vorschau, die nicht mehr in die Zeile passt, ist kein Grund,
                // das Schliessen zurueckzunehmen.
                if vorschau_danach {
                    let _ = self.bereich_einblenden(Bereich::Vorschau);
                }
            }
            Anlass::AndereDatei => {
                if let Some(editor) = self.ivars().editor.get() {
                    editor.zurueckgehaltenes_uebernehmen();
                }
            }
            // Der Nachzug unten geht diesen Anlass nichts an: nach der
            // Zustimmung legt niemand mehr eine Ansicht aus, und
            // `applicationWillTerminate:` schreibt die Sitzung ohnehin ein
            // letztes Mal.
            Anlass::Beenden => {
                self.beenden_beantworten(true);
                return;
            }
        }
        // **Was `kommando_ausfuehren` einem ausgefuehrten Befehl nachzieht.**
        // Die Fortsetzung laeuft lange nach ihm, und ohne diese beiden Zeilen
        // bliebe die Statuszeile auf dem Stand vor der Antwort.
        //
        // **Die Sichtbarkeit haengt seit dem 260823 nicht mehr an dieser
        // Zeile.** `sichtbarkeit_aendern` schreibt sie selbst auf den Schirm,
        // und `editor_ausblenden` geht darueber; der Ruf hier ist deshalb fuer
        // diesen Anlass kein Nachzug mehr, sondern eine Wiederholung. Er bleibt
        // stehen, weil `Anlass::AndereDatei` gar keine Sichtbarkeit aendert und
        // die uebrigen Anzeigen trotzdem nachzuziehen sind.
        self.aufteilung_nachziehen();
        self.sitzung_vormerken();
    }

    /// Was aufzuraeumen ist, wenn der Anlass unterbleibt (C4).
    ///
    /// "Abbrechen" und das gescheiterte Sichern gehen denselben Weg: der
    /// gehaltene Stand bleibt mit seiner Abweichungsmarke stehen, und der Editor
    /// bleibt, wo er ist. Zwei der drei Anlaesse haben darueber hinaus etwas
    /// abzulegen.
    ///
    /// Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig.
    fn anlass_unterbleibt(&self, anlass: Anlass) {
        match anlass {
            // Nichts zu tun: der Editor steht, wie er stand. Das gilt fuer
            // beide Rufer — bleibt der Editor stehen, darf die Vorschau ihn
            // gerade nicht verdraengen, und deshalb wird `vorschau_danach` hier
            // nicht gelesen.
            Anlass::EditorSchliessen { .. } => {}
            // Die gelesene Datei wartet nicht weiter: sie kostete sonst bis zu
            // 16 MB Arbeitsspeicher fuer einen Wechsel, den der Nutzer eben
            // abgelehnt hat. Mit ihr faellt die vorgemerkte Stelle einer
            // Textmarke (C6): sie gehoert zu dieser Datei, und bliebe sie
            // stehen, spraenge das naechste F4 auf eine Stelle, die niemand
            // verlangt hat.
            Anlass::AndereDatei => {
                *self.ivars().vorgemerkte_marke.borrow_mut() = None;
                if let Some(editor) = self.ivars().editor.get() {
                    editor.zurueckgehaltenes_fallenlassen();
                }
            }
            Anlass::Beenden => self.beenden_beantworten(false),
        }
    }

    /// Blendet den Editor aus und gibt seine Datei frei (C1, C4).
    ///
    /// Die Fortsetzung des Anlasses `opt+cmd+e`. Beide Haelften gehoeren
    /// zusammen: ein ausgeblendeter Editor, der seine Datei behielte, gaebe dem
    /// Fokusbefehl aus C1 einen Bereich zum Hervorholen, den der Nutzer eben
    /// geschlossen hat.
    ///
    /// Die Sichtbarkeit wird nur geaendert, wenn der Editor sie hat:
    /// [`Fenstermodell::umschalten`](crate::fenstermodell::Fenstermodell::umschalten)
    /// blendet einen ausgeblendeten Bereich sonst ein, und Schliessen brachte
    /// den Editor hervor.
    fn editor_ausblenden(&self) {
        if let Some(editor) = self.ivars().editor.get() {
            editor.schliessen();
        }
        // Der Ordner der aufgegebenen Datei wird nicht laenger beobachtet, wenn
        // ihn kein Dateifenster zeigt. Der Gegenruf steht beim Oeffnen, in
        // `editorausgang_behandeln`; beide zusammen sind die Zusage, dass die
        // beobachtete Liste und die gehaltene Datei nicht auseinanderlaufen.
        self.dateisystemwache_nachziehen();
        if self.ivars().modell.borrow().sichtbar(Bereich::Editor) {
            self.bereich_umschalten(Bereich::Editor);
        }
        // Der Editor haelt keine Datei mehr; steht der Fokus noch bei ihm, nennt
        // der Titel sonst weiter eine Datei, die niemand mehr hat.
        self.titel_nachziehen(self.fokus());
    }

    /// Den Editor schliessen (C1, C4).
    ///
    /// Der erste Anlass der Nachfrage. **Der eine Rumpf mit zwei Ruefern**, und
    /// sie unterscheiden sich in nichts als dem Argument:
    ///
    /// - `opt+cmd+e` ([`Kommando::EditorSchliessen`](krk_core::tasten::Kommando))
    ///   uebergibt `false`. Der Befehl traegt
    ///   [`Wirkungsbereich::Editor`](krk_core::tasten::Wirkungsbereich) und
    ///   erreicht diese Stelle deshalb nur mit dem Fokus in der Textflaeche.
    /// - Der Rueckweg von `cmd+e` ([`Self::editor_rundweg`]) uebergibt `true`
    ///   und holt damit die Vorschau zurueck, die sein Hinweg verdraengt hat
    ///   (Nutzerentscheid vom 260823-0942).
    ///
    /// **Ein zweiter Rumpf daneben waere ein zweiter Weg zum Aufgeben der
    /// Datei**, und mit ihm eine zweite Stelle, die die Nachfrage aus C4 stellt.
    /// Die erste Abweichung zwischen beiden faende keine Pruefung.
    ///
    /// **Nicht dasselbe wie [`Self::editor_umschalten`] darunter, und die
    /// beiden bestehen nebeneinander.** Dieser Befehl **gibt die Datei auf**:
    /// er fragt nach einem ungesicherten Stand, gibt danach ueber
    /// [`Self::editor_ausblenden`] die Datei frei und blendet die Flaeche aus.
    /// Der Umschalter darunter laesst die Datei, wo sie ist. Der Rueckweg des
    /// Rundwegs geht bewusst hier entlang und nicht dort: der Nutzer hat ihn mit
    /// diesem Preis vorgelegt bekommen und so gewaehlt.
    fn editor_schliessen(&self, vorschau_danach: bool) -> bool {
        if self.ivars().editor.get().is_none() {
            return false;
        }
        self.anlass_beginnen(Anlass::EditorSchliessen { vorschau_danach })
    }

    /// `cmd+e`: der Rundweg in den Editor und zurueck (Nutzerentscheid vom
    /// 260823-0942).
    ///
    /// **Der eine Rufer der Regel [`rundweg`]**, und sein Rumpf ist die
    /// Verteilung auf die drei bestehenden Wege und sonst nichts. Was `cmd+e`
    /// von hier aus bedeutet, entscheidet nicht diese Funktion, sondern jene
    /// reine Funktion in `crate::kommandos::rundweg`; die Fallunterscheidung
    /// steht dort und nicht hier, damit sie ohne Fenster pruefbar ist. Dieselbe
    /// Aufteilung traegt [`rueckschritt`] fuer die Rueckschritt-Taste.
    ///
    /// **Keiner der drei Zweige baut etwas Neues:**
    ///
    /// | Fokus | Zweig | derselbe Rumpf wie |
    /// |---|---|---|
    /// | Dateifenster | [`Self::im_editor_oeffnen`] | `f4` |
    /// | Vorschau | [`Self::editor_aus_vorschau`] | `cmd+e` bis zum 260823 |
    /// | Editor | [`Self::editor_schliessen`] | `opt+cmd+e`, mit der Vorschau danach |
    ///
    /// **Keine zweite Fokusabfrage.** Der Wert kommt als Argument aus der einen
    /// Abfrage in [`Self::kommando_ausfuehren`], wie bei
    /// [`Self::bereichskommando`], [`Self::tab_schliessen`] und
    /// [`Self::teilen`]; dort steht die Begruendung ausgeschrieben. Ein
    /// zweites `self.fokus()` waere eine zweite Erhebung desselben Augenblicks,
    /// und liefen die beiden auseinander, oeffnete `cmd+e` eine Datei, wo es
    /// den Editor schliessen sollte.
    ///
    /// `None` heisst: von diesem Fokus aus fuehrt kein Rundweg. Der Fall ist
    /// heute unerreichbar, weil
    /// [`Wirkungsbereich::Dateibereiche`](krk_core::tasten::Wirkungsbereich) die
    /// Leiste und das stehende Blatt schon abgewiesen hat. `false` heisst dann
    /// allein, dass kein Nachzug der Aufteilung und keine vorgemerkte Sitzung
    /// anfaellt; der Tastendruck ist verbraucht, weil
    /// [`Self::kommando_ausfuehren`] seit der Runde 7 immer `true` liefert.
    fn editor_rundweg(&self, fokus: Fokus) -> bool {
        let Some(weg) = rundweg(fokus) else {
            return false;
        };
        match weg {
            Rundweg::AusDerDateiliste => self.im_editor_oeffnen(),
            Rundweg::AusDerVorschau => self.editor_aus_vorschau(),
            Rundweg::ZurueckInDieDateiliste => self.editor_schliessen(true),
        }
    }

    /// `opt+cmd+b`: den Editor ein- und ausblenden, ohne seine Datei
    /// anzufassen (C6 der Bereichsleisten-Runde).
    ///
    /// **Nicht dasselbe wie [`Self::editor_schliessen`] darueber.** Der Weg
    /// geht durch [`Self::bereich_umschalten`] und damit durch dieselbe Stelle
    /// wie die vier anderen Bereiche: die Flaeche verschwindet, der Editor
    /// behaelt seine Datei samt Stand, und keine Nachfrage erscheint. Ein
    /// ungesicherter Stand ist danach nicht verloren, sondern nur unsichtbar;
    /// wer ihn aufgeben will, nimmt das Schliessen.
    ///
    /// **Ist der Editor ausgeblendet und haelt keine Datei, geschieht nichts,
    /// ohne Meldung.** Gefragt ist [`Self::editor_ist_ansprechbar`], dieselbe
    /// Funktion, die [`Self::fokus_editor_holen`] fragt; dort steht auch,
    /// warum die Bedingung nicht im Fenstermodell wohnt. Nutzerantwort
    /// vom 260812-0430, Datensatz
    /// `circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/
    /// 260812-0415_*_was-tut-der-editorschalter-ohne-datei-im-editor.md`.
    ///
    /// Steht der Editor schon auf dem Schirm, blendet der Befehl ihn aus, auch
    /// ohne Datei: die leere Flaeche loszuwerden ist genau das, was der Nutzer
    /// dann will.
    fn editor_umschalten(&self) -> bool {
        if !self.editor_ist_ansprechbar() {
            return false;
        }
        self.bereich_umschalten(Bereich::Editor)
    }

    /// Wechselt zwischen Rohansicht und Formatansicht (C3).
    ///
    /// **Keine Meldung in der Statuszeile.** Der Befehl tut etwas Sichtbares,
    /// naemlich die Darstellung zu wechseln; das erste Abnahmekriterium von C3
    /// verlangt genau das. Ein Satz daneben saegte am Rang der Meldungen, die
    /// eine Antwort **sind** und nicht nur eine Beschreibung dessen, was der
    /// Nutzer ohnehin sieht.
    ///
    /// **Der Wechsel verliert nichts.** Er fasst den Textspeicher nicht an; die
    /// Begruendung im Einzelnen steht an
    /// [`Editorbereich::ansicht_umschalten`](super::editor::Editorbereich::ansicht_umschalten)
    /// und im Modulkopf von [`crate::editormodell`].
    fn editor_ansicht_umschalten(&self) -> bool {
        let Some(editor) = self.ivars().editor.get() else {
            return false;
        };
        editor.ansicht_umschalten();
        true
    }

    /// Ob KRK sich jetzt beenden darf (C4).
    ///
    /// Der dritte Anlass. Drei Wege enden mit `TerminateNow`, und keiner davon
    /// verliert etwas: das Beenden ohne Tastenabgriff, ein Editor ohne
    /// ungesicherten Stand und der Fall, dass sich kein Blatt zeigen laesst,
    /// weil Fenster oder Editor fehlen. Im letzten Fall gaebe es niemanden, der
    /// die Frage beantworten koennte, und ein `TerminateCancel` liesse KRK ohne
    /// Rueckweg stehen.
    ///
    /// **Steht schon ein Blatt, wird nicht beendet.** Waehrend eines Blattes
    /// kommt ausser dem Abbruch allein die Ausnahmeliste aus
    /// [`zulaessigkeit::immer_erreichbar`] durch, und [`Kommando::Beenden`]
    /// steht darauf — Cmd+Q und der Menueeintrag "KRK beenden" erreichen diese
    /// Stelle also auch dann, und genau das ist gewollt. Ein zweites Blatt
    /// darauf zu stapeln hiesse, dem Nutzer zwei Fragen zugleich zu stellen und
    /// die erste unbeantwortet abzuraeumen; er beantwortet stattdessen die
    /// stehende und beendet danach.
    fn beenden_erlauben(&self) -> NSApplicationTerminateReply {
        if self.ivars().beenden_ohne_nachfrage.get() || !self.editor_haelt_ungesicherten_stand() {
            return NSApplicationTerminateReply::TerminateNow;
        }
        if self.blatt_steht() {
            return NSApplicationTerminateReply::TerminateCancel;
        }
        if self.nachfrage_zeigen(Anlass::Beenden) {
            NSApplicationTerminateReply::TerminateLater
        } else {
            NSApplicationTerminateReply::TerminateNow
        }
    }

    /// Bringt die Antwort auf `applicationShouldTerminate:` nach (C4).
    ///
    /// Die eine Stelle, die `replyToApplicationShouldTerminate:` ruft, und sie
    /// wird aus dem Rueckruf der Nachfrage genau einmal erreicht: ueber
    /// [`Self::anlass_ausfuehren`] mit `true`, ueber
    /// [`Self::anlass_unterbleibt`] mit `false`.
    fn beenden_beantworten(&self, beenden: bool) {
        NSApplication::sharedApplication(self.mtm()).replyToApplicationShouldTerminate(beenden);
    }

    /// Stellt eine Meldung des Editors in die Statuszeile des **aktiven**
    /// Dateifensters (C1).
    ///
    /// **Der Editor bekommt keine eigene Meldezeile.** Die Uebergabe an diese
    /// Runde sagt das zu, C1 wiederholt es, und diese Funktion ist die Stelle,
    /// an der die Zusage haelt: alles, was der Editor zu sagen hat, geht durch
    /// sie und landet in der einen Zeile, die es seit der Runde 1 gibt. Eine
    /// sechste Quelle in [`crate::appkit::statuszeile::zeile`] entsteht dabei
    /// nicht.
    ///
    /// **Rang 1 und kein eigener daneben.** Jede Meldung des Editors ist die
    /// Antwort auf einen Tastenbefehl, den der Nutzer eben gegeben hat: eine
    /// Abweisung beim Oeffnen, ein gescheitertes Sichern, eine Zeilennummer
    /// ueber der Zeilenzahl, eine Suche ohne Treffer, die Zahl der ersetzten
    /// Treffer, eine Textmarke, deren Stelle sich geaendert hat. Damit ist sie
    /// dasselbe wie die Antworten der Leiste und der Vorschau, und sie nimmt
    /// deren Weg ueber [`Self::antwort_zeigen`]. Die Vorrangregel bleibt
    /// unangetastet.
    ///
    /// **In das aktive Dateifenster und nicht in eines von beiden nach Wahl.**
    /// Der Editor steht neben beiden Fenstern und gehoert keinem; die Zeile,
    /// die der Nutzer im Blick hat, ist die des Fensters, mit dem er zuletzt
    /// gearbeitet hat. Denselben Bezug nehmen die Befehle ohne eigene Seite
    /// seit der Runde 1: [`Self::loeschen_nach_rueckfrage`], der eine Rumpf
    /// jedes Loeschbefehls, liest `aktiv` und meldet „es ist nichts
    /// ausgewählt" dorthin, und die beiden Operationsbefehle tun es ueber
    /// [`Self::auftrag_stellen`] genauso. Bis zum 260817 stand jene Meldung im
    /// endgueltigen Loeschen; der Bezug hat sich mit ihr nicht geaendert.
    fn editormeldung_zeigen(&self, meldung: &Editormeldung) {
        let aktiv = self.ivars().modell.borrow().aktiv();
        self.antwort_zeigen(aktiv, &meldung.text());
    }

    // ------------------------------------------------------------------
    // Sitzung
    // ------------------------------------------------------------------

    /// Der Sitzungszustand, wie er auf die Platte gehoert.
    ///
    /// **Die Datei des Editors kommt aus dem Editor** und nicht aus dem
    /// Fenstermodell, das vom Editor allein Breite und Sichtbarkeit kennt.
    /// Mitgeschrieben wird der Pfad und nicht der Stand; der Grund steht an
    /// `krk_core::ablage::Sitzung::editor`. Solange kein Editor gebaut ist —
    /// vor `oberflaeche_aufbauen` und im Messmodus — steht dort `None`, und das
    /// ist dieselbe Aussage wie die eines Editors ohne Datei.
    ///
    /// **Vom Notizzettel geht die Merkung mit und nie der Text.** Welcher der
    /// zwei offen ist, steht im Zettelmodell; was auf ihm steht, gehoert in
    /// `note-1.txt` und `note-2.txt`. Der Zwei-Sekunden-Takt der
    /// Sitzungssicherung ruft diese Funktion, und deshalb ist genau hier
    /// entschieden, dass er den Text des Zettels nicht mittraegt (C4 der Runde
    /// 9); eine Probe in `krk-core/tests/ablage.rs` haelt es an der
    /// geschriebenen Datei fest.
    fn sitzung_bauen(&self) -> Sitzung {
        // Einer der Anlaesse, an denen der Schirm in das Modell zurueckgelesen
        // wird; die Regel dafuer steht an `bildschirmbreiten_uebernehmen`.
        // Dieser hier faellt auch ohne Befehl an, naemlich ueber den Takt der
        // Sitzungssicherung und beim Beenden, und deshalb bleibt er neben dem
        // am Kopf von `kommando_ausfuehren` stehen.
        self.bildschirmbreiten_uebernehmen();
        let fenster = [
            self.dateifenster(Fensterseite::Links).quelle().zustand(),
            self.dateifenster(Fensterseite::Rechts).quelle().zustand(),
        ];
        let editor = self.editordatei();
        let zettel = self.ivars().zettel.borrow().offener();
        self.ivars()
            .modell
            .borrow()
            .sitzung(fenster, editor, zettel)
    }

    /// Merkt den Sitzungszustand vor; geschrieben wird gebuendelt.
    ///
    /// Hoechstens alle zwei Sekunden, wie `### Frage 4` es vorschreibt. Ein
    /// liegengebliebener Stand geht spaetestens beim Beenden auf die Platte.
    fn sitzung_vormerken(&self) {
        if self.ivars().sitzungsschreiber.borrow().is_none() {
            return;
        }
        let sitzung = self.sitzung_bauen();
        let ergebnis = {
            let mut schreiber = self.ivars().sitzungsschreiber.borrow_mut();
            let schreiber = schreiber
                .as_mut()
                .expect("oben geprueft, und dazwischen laeuft nichts");
            // Der Durchgang umfasst genau das Schreiben: der Stand steht schon
            // im Schreiber, und gelesen wird hier nichts. Ohne Ablageordner
            // gaebe es keinen Schreiber, also kann `unter_der_sperre` hier nur
            // an der Sperre selbst scheitern.
            match self
                .unter_der_sperre(|zugang| schreiber.vormerken(sitzung, Instant::now(), zugang))
            {
                Ok(ergebnis) => ergebnis,
                Err(Sperrhindernis::OhneOrdner) => Ok(false),
                Err(Sperrhindernis::Gesperrt(fehler)) => Err(fehler),
            }
        };
        if let Err(fehler) = ergebnis
            && !self.ivars().schreibfehler_gemeldet.replace(true)
        {
            // In die Zeile des aktiven Dateifensters, aus demselben Grund wie
            // die Startmeldungen: die Sitzung gehoert der Anwendung und keiner
            // Seite, und der Nutzer sieht auf die Seite, in der er arbeitet.
            let meldung = format!("die Sitzung liess sich nicht sichern: {fehler}");
            let aktiv = self.ivars().modell.borrow().aktiv();
            self.dateifenster(aktiv).quelle().meldung_zeigen(&meldung);
        }
    }

    // ------------------------------------------------------------------
    // Messmodus
    // ------------------------------------------------------------------

    /// Haengt Bildtakt und Ausloesetakt ein, wenn ein Messlauf ansteht.
    fn messmodus_einrichten(&self) {
        let ivars = self.ivars();
        let Some(aufgabe) = ivars.messaufgabe.clone() else {
            return;
        };
        let Some(fenster) = ivars.fenster.get() else {
            return;
        };
        let dateifenster = self.dateifenster(Fensterseite::Links);

        // Die Rate zuerst, und ohne sie kein Messlauf. Die Regel steht in S21
        // des Plans ausgeschrieben: ein Fenster auf keinem Bildschirm heisst
        // Abbruch, nicht Ausweichen auf den Hauptbildschirm.
        let Some(hertz) = bildtakt::bildwiederholrate(fenster) else {
            eprintln!("krk: {}", crate::messmodus::OHNE_BILDSCHIRM);
            std::process::exit(OHNE_BILDSCHIRM);
        };

        let mut lauf = Messlauf::neu(aufgabe);
        lauf.rate_setzen(hertz);
        let lauf = Rc::new(RefCell::new(lauf));
        let _ = ivars.messlauf.set(Rc::clone(&lauf));

        // Der Rueckruf haelt den Delegierten **schwach**, wie jeder Rueckruf
        // dieses Moduls; den Zustand baut `messzustand`, dieselbe Rechnung
        // wie beim Ausloesetakt.
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let takt = Zeichenende::einrichten(self.mtm(), dateifenster.sicht(), move |jetzt| {
            let Some(selbst) = schwach.load() else {
                return;
            };
            let zustand = selbst.messzustand();
            if lauf.borrow_mut().bildgrenze(jetzt, zustand) {
                std::process::exit(0);
            }
        });
        let _ = ivars.zeichenende.set(takt);

        // SAFETY: `self` ist das Ziel und beantwortet `messSchritt:` mit der
        // erwarteten Signatur. Der Zeitgeber wird unten in die Laufschleife
        // gehaengt; `NSRunLoopCommonModes` ist ein Fremdsymbol von Foundation.
        let zeitgeber = unsafe {
            let zeitgeber = NSTimer::timerWithTimeInterval_target_selector_userInfo_repeats(
                crate::messmodus::AUSLOESETAKT,
                self,
                sel!(messSchritt:),
                None,
                true,
            );
            NSRunLoop::currentRunLoop().addTimer_forMode(&zeitgeber, NSRunLoopCommonModes);
            zeitgeber
        };
        let _ = ivars.ausloesetakt.set(zeitgeber);
    }

    /// Ein Takt des Ausloesers: den naechsten Messschritt holen und ausfuehren.
    fn messen_weiter(&self) {
        let ivars = self.ivars();
        let (Some(lauf), Some(fenster)) = (ivars.messlauf.get(), ivars.fenster.get()) else {
            return;
        };
        let zustand = self.messzustand();

        // Die Ausleihe endet vor dem AppKit-Aufruf: der Bildtakt greift auf
        // denselben `RefCell` zu, und ein Zeichendurchgang mitten in einer
        // gehaltenen Ausleihe waere der doppelte Zugriff.
        let anweisung = lauf.borrow_mut().naechster_schritt(zustand);
        match anweisung {
            Anweisung::Warten => {}
            Anweisung::Lesen(pfad) => self
                .dateifenster(Fensterseite::Links)
                .quelle()
                .ordner_lesen(&pfad, None),
            Anweisung::Taste => ereignisse::pfeil_ab_senden(self.mtm(), fenster),
            Anweisung::Funktionstaste(kennung) => {
                let ergebnis = {
                    let belegung = ivars.belegung.borrow();
                    ereignisse::funktion_senden(self.mtm(), fenster, &belegung, kennung)
                };
                if let Err(meldung) = ergebnis {
                    eprintln!("krk: {meldung}. Es wird keine Zahl ausgegeben.");
                    std::process::exit(4);
                }
            }
            // Eine Vorbereitung, die fehlschlaegt, geht in den Messlauf
            // zurueck; er bricht am naechsten Takt ab. Der Umweg ueber den
            // Messlauf statt eines `exit` an Ort und Stelle haelt den einen
            // Abbruchweg der Strecke zusammen.
            Anweisung::Handeln(handlung) => {
                if let Err(grund) = self.messhandlung(handlung) {
                    lauf.borrow_mut().vorbereitung_gescheitert(grund);
                }
            }
            Anweisung::Fertig => {
                lauf.borrow().ausgeben();
                std::process::exit(0);
            }
            Anweisung::Abbruch(grund) => {
                eprintln!("krk: {grund}. Es wird keine Zahl ausgegeben.");
                std::process::exit(4);
            }
        }
    }

    /// Fuehrt eine ungemessene Vorbereitung der Sitzungsstrecke aus (S21).
    ///
    /// Absichtlich als unmittelbare Aufrufe statt ueber die Ereignisschlange:
    /// gemessen wird nur, was C8 zusagt, und eine Vorbereitung auf demselben
    /// Weg stuende der Messung in der Schlange im Weg.
    ///
    /// `Err` heisst: die Vorbereitung hat nicht hergestellt, was der naechste
    /// Schritt voraussetzt. Der Grund geht an den Messlauf zurueck, der
    /// daraufhin abbricht.
    fn messhandlung(&self, handlung: Handlung) -> Result<(), String> {
        let aktiv = self.ivars().modell.borrow().aktiv();
        match handlung {
            Handlung::Listenanfaenge => {
                for seite in Fensterseite::ALLE {
                    self.dateifenster(seite)
                        .quelle()
                        .kommando_ausfuehren(Kommando::Listenanfang);
                }
            }
            Handlung::Auswaehlen(name) => {
                let quelle = self.dateifenster(aktiv).quelle();
                match quelle.eintrag_waehlen(&name) {
                    // Gewaehlt: die Auswahl steht auf dem Eintrag. Vorgemerkt:
                    // es laeuft noch ein Lesevorgang, und die Auswahl springt
                    // mit seinem Abschluss auf den Namen. Beides ist der
                    // gewoehnliche Weg und kein Fehlschlag.
                    Auswahlversuch::Gewaehlt(_) | Auswahlversuch::Vorgemerkt => {}
                    // Die Liste ist gelesen und kennt den Namen nicht. Der
                    // Rueckgabewert wurde bis zum 260807 hier verworfen, und
                    // der Lauf lief danach in die Zehn-Sekunden-Geduld der
                    // naechsten Messung, die den Fehlschlag nicht benennen
                    // konnte.
                    Auswahlversuch::Unbekannt => {
                        return Err(crate::messmodus::auswahl_ohne_eintrag(
                            &name,
                            &quelle.angezeigter_ordner(),
                            quelle.zeilen(),
                        ));
                    }
                }
            }
            Handlung::AlleMarkieren => {
                self.dateifenster(aktiv)
                    .quelle()
                    .kommando_ausfuehren(Kommando::AlleMarkieren);
            }
            Handlung::AktivLesen(pfad) => {
                self.dateifenster(aktiv).quelle().ordner_lesen(&pfad, None);
            }
            Handlung::RechtsLesen(pfad) => {
                self.dateifenster(Fensterseite::Rechts)
                    .quelle()
                    .ordner_lesen(&pfad, None);
            }
        }
        Ok(())
    }

    /// Der Zustand der Oberflaeche, wie der Messmodus ihn abliest.
    ///
    /// Die drei Zahlen des linken Dateifensters tragen die Strecken aus S8;
    /// die [`Sitzungslage`] dahinter fuellt die Sitzungsstrecke aus S21. Sie
    /// wird immer gefuellt, weil sie nur Ablesungen enthaelt und ein zweiter
    /// Zustandsbauer je Aufgabe zwei Wahrheiten ueber dieselbe Oberflaeche
    /// waere.
    fn messzustand(&self) -> Zustand {
        let links = self.dateifenster(Fensterseite::Links).quelle();
        let rechts = self.dateifenster(Fensterseite::Rechts).quelle();
        let aktiv = self.ivars().modell.borrow().aktiv();
        let aktiv_quelle = self.dateifenster(aktiv).quelle();
        let (vorschau_pfad, vorschau_laedt) = match self.ivars().vorschau.get() {
            Some(vorschau) => (vorschau.angezeigter_pfad(), vorschau.laedt_noch()),
            None => (None, false),
        };
        Zustand {
            zeilen: links.zeilen(),
            liest: links.liest_noch(),
            auswahl: links.auswahlzeile(),
            sitzung: Some(Sitzungslage {
                // Gefragt ist die **Anwendung** und nicht das Fenster: waehrend
                // eines Blattes ist dessen Panel das Schluesselfenster, das
                // Hauptfenster also nicht, und KRK steht trotzdem vorn. Ein
                // Blatt hat seinen eigenen Vorbehalt in
                // `kommando_ausfuehren`; hier ginge es als "nicht im
                // Vordergrund" durch und stellte dem Leser die falsche
                // Diagnose.
                im_vordergrund: NSApplication::sharedApplication(self.mtm()).isActive(),
                aktiv_links: aktiv == Fensterseite::Links,
                zeilen_aktiv: aktiv_quelle.zeilen(),
                liest_aktiv: aktiv_quelle.liest_noch(),
                auswahl_aktiv: aktiv_quelle.auswahlzeile(),
                tab_aktiv: aktiv_quelle.sichtbarer_tab(),
                ordner_aktiv: aktiv_quelle.angezeigter_ordner(),
                auswahl_pfad: aktiv_quelle.auswahl_pfad(),
                zeilen_rechts: rechts.zeilen(),
                liest_rechts: rechts.liest_noch(),
                ordner_rechts: rechts.angezeigter_ordner(),
                vorschau_pfad,
                vorschau_laedt,
                vorgang_sichtbar: links.vorgang_sichtbar() || rechts.vorgang_sichtbar(),
                vorgang_laeuft: self.ivars().vorgang.borrow().is_some(),
            }),
        }
    }
}

/// Was der Auffrischungspfad aus C9 von den beiden Dateifenstern braucht.
///
/// Jede Methode ist eine Zeile: der Delegierte ist die einzige Stelle, die
/// beide Dateifenster und das Fenstermodell haelt, und deshalb die einzige,
/// die die Fragen beantworten kann. Die Rechnung darauf steht in
/// [`crate::auffrischung`] und ist ohne Fenster pruefbar.
impl Dateifenstersicht for Anwendungsdelegierter {
    fn ordner(&self, seite: Fensterseite) -> PathBuf {
        self.dateifenster(seite).quelle().angezeigter_ordner()
    }

    fn tabordner(&self, seite: Fensterseite) -> Vec<PathBuf> {
        self.dateifenster(seite).quelle().tabordner()
    }

    fn sichtbarer_tab(&self, seite: Fensterseite) -> usize {
        self.dateifenster(seite).quelle().sichtbarer_tab()
    }

    fn sichtbar(&self, seite: Fensterseite) -> bool {
        self.ivars()
            .modell
            .borrow()
            .sichtbar(Bereich::von_seite(seite))
    }

    /// **Die eine Stelle, die diese Frage an den Editorbereich stellt.** Drei
    /// Aufrufer haben sie: die Sitzung aus C7 ([`Anwendungsdelegierter::sitzung_bauen`]),
    /// die Liste der beobachteten Ordner und die Frage, ob ein gemeldeter Stapel
    /// die gehaltene Datei betrifft. Drei `get`-Ketten nebeneinander waeren drei
    /// Gelegenheiten, den fehlenden Editor verschieden zu behandeln.
    fn editordatei(&self) -> Option<PathBuf> {
        self.ivars().editor.get().and_then(|editor| editor.pfad())
    }

    fn neu_lesen(&self, seite: Fensterseite) {
        self.dateifenster(seite).quelle().neu_lesen();
    }

    fn tab_wechseln(&self, seite: Fensterseite, stelle: usize, ziel: &Path) {
        self.dateifenster(seite)
            .quelle()
            .tab_ordner_setzen(stelle, ziel);
    }

    fn melden(&self, seite: Fensterseite, text: &str) {
        self.dateifenster(seite).quelle().meldung_zeigen(text);
    }

    fn namenszelle_in_bearbeitung(&self, seite: Fensterseite) -> bool {
        self.dateifenster(seite)
            .quelle()
            .namenszelle_in_bearbeitung()
    }

    fn auffrischung_vormerken(&self, seite: Fensterseite) {
        self.dateifenster(seite).quelle().auffrischung_vormerken();
    }
}

/// Der Vermittlerfaden zwischen der Operationsmaschine und dem Hauptfaden.
///
/// **Er ist kein Takt.** Er schlaeft in `recv`, solange nichts zu melden ist,
/// und zieht dabei keinen Strom; geweckt wird er von der Meldung selbst, und er
/// weckt seinerseits den Hauptfaden. Damit haelt die Wahl des Nutzers vom
/// 260804, die Buendelung ohne Zeitgeber zu bauen
/// (`issues/260803-2007_*_s16-nennt-keinen-mechanismus-fuer-die-buendelung-der-fortschrittsmeldungen.md`,
/// Weg 3).
///
/// **Warum es ihn ueberhaupt gibt.** Der Empfaenger des Meldekanals darf nicht
/// zwischen Faeden geteilt werden, und der Hauptfaden darf in `recv` nicht
/// warten: das waere die Dateisystem-Arbeit auf dem Hauptfaden, die
/// `### Frage 6` ausschliesst, und L9 fiele mit ihr. Ein Faden, der wartet, ist
/// der Preis dafuer.
///
/// **Der Abbruchwunsch laeuft nicht mehr ueber diesen Faden.** Bis zum 260805
/// tat er das: der Hauptfaden setzte ein zweites Kennzeichen, und dieser Faden
/// reichte es nach jeder Meldung an den [`Lauf`] weiter. Bei einer Operation,
/// die ueber Sekunden nichts meldet, wirkte der Abbruch entsprechend spaet
/// (`issues/260804-1816_*_der-abbruchwunsch-erreicht-den-lauf-erst-mit-der-naechsten-meldung.md`).
/// Der Hauptfaden haelt seit `Lauf::abbruchgriff` das Kennzeichen des Laufs
/// selbst und setzt es unmittelbar; hier ist dafuer nichts mehr zu tun.
fn vermitteln(lauf: Lauf, zustand: &Arc<Vorgangszustand>) {
    let mut abgeschlossen = false;
    while let Ok(meldung) = lauf.meldungen().recv() {
        let fertig = matches!(meldung, Meldung::Fertig(_));
        zustand.aendern(|stand| match meldung {
            Meldung::Fortschritt(fortschritt) => stand.fortschritt = Some(fortschritt),
            Meldung::Uebersprungen(eintrag) => stand.uebersprungen.push(eintrag),
            Meldung::Konflikt {
                quelle,
                ziel,
                antwort,
            } => {
                stand.konflikt = Some(Konfliktfrage {
                    quelle,
                    ziel,
                    antwort,
                });
            }
            Meldung::Fertig(bericht) => stand.bericht = Some(bericht),
        });
        // Auch der Abschluss und die Konfliktfrage gehen durch die Buendelung.
        // Verworfen wird dabei allein der **Weckruf**, nicht die Meldung: steht
        // schon einer aus, hat der Hauptfaden noch nicht gelesen und findet
        // beides beim naechsten Durchgang vor.
        if zustand.buendelung.melden() {
            hauptfaden_wecken();
        }
        if fertig {
            abgeschlossen = true;
            break;
        }
    }
    if !abgeschlossen {
        abbruch_ohne_meldung_nachtragen(zustand);
    }
    lauf.warten();
}

/// Traegt den Abschlussbericht nach, den der Arbeitsfaden nicht mehr geschickt
/// hat.
///
/// **Wann der Fall eintritt.** Der Meldekanal schliesst ohne
/// [`Meldung::Fertig`] genau dann, wenn der Arbeitsfaden aus
/// `krk_core::operation::starten` vor seiner letzten Zeile abbricht, also bei
/// einer Panik in `ausfuehren`. Einen Panikpfad dorthin gibt es heute nicht;
/// dass die Schleife den Fall stillschweigend behandelte, war trotzdem eine
/// offene Flanke.
///
/// **Warum er nicht folgenlos bleiben darf.** Ohne Bericht setzt der Hauptfaden
/// `stand.bericht` nie, erreicht `vorgang_beenden` nie und leert `ivars.vorgang`
/// nie. Die Fortschrittszeile bliebe stehen, der naechste Operationsbefehl
/// wuerde abgewiesen, und seit die Dateisystemwache die Ordner des laufenden
/// Vorgangs aussetzt, bliebe der Ordner fuer die ganze Laufzeit von jeder
/// Auffrischung ausgeschlossen — auch von den fremden Aenderungen, die C9
/// zusagt.
///
/// **Dieselbe Bahn und kein zweiter Aufraeumweg.** Nachgetragen wird ein
/// gewoehnlicher [`Bericht`], der durch [`Vorgangszustand`] und den Weckruf
/// laeuft wie jeder andere; `vorgang_beenden` raeumt daraufhin von selbst auf.
/// Die Zahlen kommen aus dem letzten Zwischenstand und nicht aus Nullen: was
/// vor dem Abbruch durchlief, ist uebertragen, und der Nutzer liest in der
/// Abschlusszeile, wie weit es kam.
fn abbruch_ohne_meldung_nachtragen(zustand: &Arc<Vorgangszustand>) {
    zustand.aendern(|stand| {
        let (eintraege, bytes) = stand.fortschritt.as_ref().map_or((0, 0), |zwischenstand| {
            (zwischenstand.eintraege, zwischenstand.bytes)
        });
        stand.bericht = Some(Bericht {
            abschluss: Abschluss::Abgebrochen,
            eintraege,
            bytes,
            uebersprungen: std::mem::take(&mut stand.uebersprungen),
        });
    });
    if zustand.buendelung.melden() {
        hauptfaden_wecken();
    }
}

/// Weckt den Hauptfaden, damit er den Stand des Vorgangs zeichnet.
///
/// Der Block haelt nichts fest, was dem Hauptfaden gehoert: er sucht den
/// Anwendungsdelegierten dort, wo er ohnehin steht. Damit braucht der Weckruf
/// keine Verrenkung, um einen `Retained` ueber die Fadengrenze zu tragen.
fn hauptfaden_wecken() {
    DispatchQueue::main().exec_async(|| {
        let Some(mtm) = MainThreadMarker::new() else {
            // Kann nicht eintreten: die Hauptschlange laeuft auf dem
            // Hauptfaden. Ein Abbruch waere hier trotzdem falsch, weil er eine
            // laufende Kopie um ihre Anzeige braechte und nicht um mehr.
            return;
        };
        Anwendungsdelegierter::vorgang_einziehen(mtm);
    });
}

/// Der Ordner, auf den ein Dateifenster ausweicht, wenn sein Datentraeger
/// verschwindet (C9).
///
/// Das Benutzerverzeichnis, und ohne eines die Wurzel. Derselbe Rueckfall wie
/// beim Standardordner eines Tabs in `krk-core`: ein Dateifenster muss einen
/// Ordner zeigen, und `/` gibt es immer.
fn benutzerverzeichnis() -> PathBuf {
    pfade::benutzerverzeichnis().unwrap_or_else(|| PathBuf::from("/"))
}

/// Der untere Teil der Leiste: das Benutzerverzeichnis und die eingehaengten
/// Datentraeger (C5).
///
/// Hier und nicht in [`super::volumes`], weil das Benutzerverzeichnis kein
/// Datentraeger ist: es kommt aus `krk_core::ablage::pfade`, und jenes Modul
/// beantwortet die Frage nach den Datentraegern und nur die. Es steht zuoberst,
/// weil es der Ordner ist, den der Nutzer am haeufigsten will.
///
/// Sein Name ist der letzte Namensteil, also der Anmeldename. Ihn ueber
/// `NSFileManager.displayNameAtPath:` zu uebersetzen brachte nichts: das System
/// liefert fuer das Benutzerverzeichnis denselben Namen.
fn orte() -> Vec<Ort> {
    let zuhause = benutzerverzeichnis();
    let name = zuhause
        .file_name()
        .map(|teil| teil.to_string_lossy().into_owned())
        .unwrap_or_else(|| zuhause.display().to_string());
    let mut orte = vec![Ort::neu(name, zuhause)];
    orte.extend(super::volumes::eingehaengte());
    orte
}

/// Startet die Anwendung. Kehrt zurueck, wenn sie beendet ist.
///
/// `tasten_protokoll` schaltet den Modus `--tasten-protokoll` aus der
/// Befehlszeile durch bis zum Ereignisabgriff, `messaufgabe` den Modus
/// `--messmodus` bis zum Aufbau der Oberflaeche. `menue_protokoll` schreibt das
/// gebaute Hauptmenue auf die Standardausgabe und kehrt zurueck, ohne ein
/// Fenster zu oeffnen.
///
/// **Die Belegung wird hier geladen und nicht im Delegierten**, weil sie seit
/// Schritt 13c zwei Abnehmer hat: das Hauptmenue, das seine Kuerzel daraus
/// nimmt, und den Ereignisabgriff. Eine Quelle, zwei sichtbare Wege.
pub fn starten(tasten_protokoll: bool, menue_protokoll: bool, messaufgabe: Option<Aufgabe>) {
    let mtm = MainThreadMarker::new()
        .expect("die Oberflaeche von KRK laeuft ausschliesslich auf dem Hauptfaden");

    let (belegung, belegungsmeldung) = belegung::fuer_den_betrieb();

    // Vor `NSApplication`, sonst haengt macOS dem Menue "Bearbeiten" eigene
    // Eintraege mit eigenen Kuerzeln an; die Begruendung samt Messung steht an
    // der Funktion.
    menue::systemzusaetze_unterdruecken();

    let anwendung = NSApplication::sharedApplication(mtm);
    anwendung.setActivationPolicy(NSApplicationActivationPolicy::Regular);
    let hauptmenue = menue::hauptmenue(mtm, &belegung);
    anwendung.setMainMenu(Some(&hauptmenue));

    if menue_protokoll {
        // **`finishLaunching` ist der Zeitpunkt, an dem die Messung ueberhaupt
        // etwas sieht.** Bis dahin haengt am Menue genau das, was `hauptmenue`
        // hineingebaut hat; erst dieser Aufruf laesst AppKit seine eigenen
        // Zusaetze dazustellen. Gemessen am 260805 mit einer Sonde, die
        // `performClose:` voruebergehend wieder eintrug: davor stand das
        // Fenstermenue mit zwei Eintraegen da, danach mit dreien, der dritte
        // "Close All" auf Opt+Shift+Cmd+W mit dem Selektor `closeAll:`. Ohne
        // den Aufruf sagte dieser Modus nichts ueber die Zusaetze aus und
        // pruefte allein den eigenen Programmtext.
        //
        // Ein Fenster oeffnet er nicht: der Anwendungsdelegierte ist zu diesem
        // Zeitpunkt noch nicht gesetzt, also erreicht
        // `applicationDidFinishLaunching:` niemanden und
        // `oberflaeche_aufbauen` laeuft nicht.
        anwendung.finishLaunching();
        menue::protokollieren(&hauptmenue);
        return;
    }

    // Der Delegierte bleibt bis zum Ende von `starten` am Leben, weil
    // `NSApplication` ihn nur schwach haelt.
    let delegierter = Anwendungsdelegierter::neu(
        mtm,
        tasten_protokoll,
        messaufgabe,
        belegung,
        belegungsmeldung,
    );
    anwendung.setDelegate(Some(ProtocolObject::from_ref(&*delegierter)));

    anwendung.run();
}

#[cfg(test)]
mod faengerproben {
    use krk_core::tasten::normalisieren;
    use krk_core::tasten::normalisierung::roh;

    use crate::belegungsmodell::Suchlage;

    use super::*;

    /// Das Zeichen, das AppKit der Escape-Taste beilegt.
    const ZEICHEN_ESC: char = '\u{1B}';
    /// Das Zeichen, das AppKit dem Pfeil ab beilegt (`NSDownArrowFunctionKey`).
    const ZEICHEN_PFEIL_AB: char = '\u{F701}';

    /// Ein Tastendruck ohne Zusatztaste, ueber seinen Namen in der einen
    /// Tastentabelle des Kerns.
    fn druck(name: &str) -> Tastendruck {
        Tastendruck::neu(code_von_pflicht(name), ModMaske::LEER)
    }

    /// Ein Tastendruck mit den rohen Zusatztastenbits von AppKit.
    ///
    /// Ueber `Tastendruck::neu`, das sein Zeichen aus derselben Tabelle nimmt
    /// wie den Code; eine von Hand danebengeschriebene Angabe koennte den zwei
    /// Feldern widersprechen.
    fn druck_mit(name: &str, rohe_flaggen: u64) -> Tastendruck {
        Tastendruck::neu(code_von_pflicht(name), normalisieren(rohe_flaggen))
    }

    /// Eine Instanz ohne Sitzungsrecht sagt es, und sie sagt die Folge (C3.10).
    ///
    /// Geprueft wird der Satz und nicht sein Weg in die Statuszeile: den geht er
    /// ueber den Meldungsvektor des Starts wie jede andere Startmeldung, und
    /// sichtbar ist er am laufenden Buendel. Was hier festgehalten wird, ist,
    /// dass er ueberhaupt einen nennt und nicht das Wort „Sitzungsrecht"
    /// weiterreicht, das nur dieser Bauplan kennt.
    #[test]
    fn der_satz_ohne_sitzungsrecht_nennt_die_folge_und_nicht_den_mechanismus() {
        assert!(
            OHNE_SITZUNGSRECHT.contains("weitere Instanz"),
            "der Satz nennt den Grund nicht: {OHNE_SITZUNGSRECHT}"
        );
        assert!(
            OHNE_SITZUNGSRECHT.contains("nicht gesichert"),
            "der Satz nennt die Folge nicht: {OHNE_SITZUNGSRECHT}"
        );
        assert!(
            !OHNE_SITZUNGSRECHT.contains("Sitzungsrecht"),
            "der Satz reicht ein Wort dieses Bauplans an den Nutzer weiter: {OHNE_SITZUNGSRECHT}"
        );
    }

    /// Waehrend der Aufnahme bekommt die Suche nichts (C1.15).
    ///
    /// Der Vorrang ist keine eigene Regel, sondern die Stellung der ersten
    /// Station vor der zweiten: mit laufender Aufnahme antwortet
    /// [`faengerstation`] fuer **jeden** Tastendruck `Aufnahme`, fuer das
    /// Suchzeichen so gut wie fuer die Eingabetaste, die Ruecktaste und `esc`.
    #[test]
    fn waehrend_der_aufnahme_bekommt_die_suche_nichts() {
        let faelle = [
            (druck("d"), Some('d'), "ein Suchzeichen"),
            (druck("space"), Some(' '), "die Leertaste"),
            (druck("return"), Some('\r'), "die Eingabetaste"),
            (druck("delete"), Some('\u{7F}'), "die Ruecktaste"),
            (druck("esc"), Some(ZEICHEN_ESC), "esc"),
            (druck("down"), Some(ZEICHEN_PFEIL_AB), "der Pfeil ab"),
        ];
        for (gedrueckt, zeichen, was) in faelle {
            assert_eq!(
                faengerstation(true, gedrueckt, zeichen),
                Faengerstation::Aufnahme,
                "{was} erreicht waehrend der Aufnahme nicht die Aufnahme"
            );
        }
    }

    /// `esc` behaelt seine zwei Bedeutungen und bekommt keine dritte (C1.13).
    ///
    /// Waehrend der Aufnahme bricht es sie ab; sonst wird es der Suche
    /// angeboten, **die es abweist**, und laeuft deshalb weiter in den
    /// Nachschlag, wo `abbrechen` die Ansicht verlaesst. Einen Suchtext loescht
    /// es nie — der Test haelt beide Haelften zusammen, weil die Zusage nur aus
    /// beiden folgt.
    #[test]
    fn esc_bekommt_keine_dritte_bedeutung() {
        assert_eq!(
            faengerstation(true, druck("esc"), Some(ZEICHEN_ESC)),
            Faengerstation::Aufnahme,
            "waehrend der Aufnahme bricht esc sie ab"
        );
        assert_eq!(
            faengerstation(false, druck("esc"), Some(ZEICHEN_ESC)),
            Faengerstation::Suchzeichen(ZEICHEN_ESC),
            "sonst wird esc der Suche angeboten"
        );

        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let mut lage = Suchlage::neu();
        assert!(
            lage.zeichen_anhaengen('a', &modell),
            "»a« ist ein Suchzeichen"
        );
        let vorher = lage.clone();
        assert!(
            !lage.zeichen_anhaengen(ZEICHEN_ESC, &modell),
            "die Suche nimmt esc auf"
        );
        assert_eq!(lage, vorher, "esc hat den Stand der Suche veraendert");
    }

    /// Die Eingabetaste und die Ruecktaste gehen an die Suche (C1.7, C1.8).
    #[test]
    fn die_eingabetaste_und_die_ruecktaste_gehen_an_die_suche() {
        assert_eq!(
            faengerstation(false, druck("return"), Some('\r')),
            Faengerstation::NaechsterTreffer
        );
        assert_eq!(
            faengerstation(false, druck("delete"), Some('\u{7F}')),
            Faengerstation::ZeichenWeg
        );
    }

    /// Eine Kombination mit Befehls-, Steuerungs- oder Wahltaste gehoert nicht
    /// der Suche; die Umschalttaste dagegen schon.
    ///
    /// Daran haengen die drei Schaltflaechenkuerzel der Ansicht und jedes
    /// Kuerzel des Hauptmenues: liefe Cmd+T in den Suchtext, waere "Zuweisen"
    /// nicht mehr zu erreichen.
    #[test]
    fn eine_kombination_mit_zusatztaste_gehoert_nicht_der_suche() {
        for (bit, name) in [
            (roh::BEFEHL, "cmd"),
            (roh::STEUERUNG, "ctrl"),
            (roh::WAHL, "opt"),
        ] {
            assert_eq!(
                faengerstation(false, druck_mit("t", bit), Some('t')),
                Faengerstation::Keine,
                "{name}+t gehoert nicht der Suche"
            );
            assert_eq!(
                faengerstation(false, druck_mit("r", bit | roh::UMSCHALT), Some('r')),
                Faengerstation::Keine,
                "shift+{name}+r gehoert nicht der Suche"
            );
        }
        assert_eq!(
            faengerstation(false, druck_mit("d", roh::UMSCHALT), Some('D')),
            Faengerstation::Suchzeichen('D'),
            "die Umschalttaste ist die Grossschreibung eines Zeichens"
        );
    }

    /// Cmd+Eingabe gehoert der Schaltflaeche "Fertig" und nicht der Suche.
    ///
    /// Der Fall faellt sonst zwischen die zwei Zweige: die Eingabetaste steht
    /// oben in [`faengerstation`], die Zusatztastenfrage darueber.
    #[test]
    fn cmd_eingabe_gehoert_der_schaltflaeche_fertig() {
        assert_eq!(
            faengerstation(false, druck_mit("return", roh::BEFEHL), Some('\r')),
            Faengerstation::Keine
        );
    }

    /// Ein Ereignis ohne Zeichen laeuft weiter, etwa eine reine Zusatztaste.
    #[test]
    fn ein_ereignis_ohne_zeichen_laeuft_weiter() {
        assert_eq!(
            faengerstation(false, druck("f5"), None),
            Faengerstation::Keine
        );
    }
}

/// Die vier Sicherungsmomente des Notizzettels, am Quelltext gezaehlt (C1, C4
/// der Runde 9).
///
/// **Warum am Baum und nicht an einem Rueckgabewert.** „Die vier Momente sind an
/// genau einer Stelle erklaert und werden von vier Aufrufern angesprochen" ist
/// eine Aussage ueber den Baum: an keinem Ergebnis ist abzulesen, dass es keine
/// zweite Erklaerung daneben gibt. Der Kopf von [`crate::quellbaum`] beschreibt
/// die Bauform und sagt auch, was sie nicht kann.
///
/// **Die Nadeln stehen zusammengesetzt da**, weil diese Proben in der Datei
/// liegen, die sie lesen; als ein Stueck geschrieben faende jede sich selbst.
#[cfg(test)]
mod zettelproben {
    use crate::quellbaum::{aufrufstellen, quelldateien};

    /// Der Quelltext der Datei, in der die vier Momente stehen.
    ///
    /// `pub(super)`, weil [`super::angleichproben`] denselben Quelltext liest.
    /// Eine zweite Fassung daneben waere der Doppelbau, gegen den die Proben
    /// dieser Datei geschrieben sind.
    pub(super) fn diese_datei() -> String {
        quelldateien()
            .into_iter()
            .find(|(name, _)| name == "krk-ui/src/appkit/anwendung.rs")
            .expect("die Datei des Anwendungsdelegierten steht im Quellbaum")
            .1
    }

    /// Der Rumpf einer Methode dieser Datei, ohne Doc-Kommentar und ohne Prosa.
    ///
    /// Der Rumpf endet an der ersten schliessenden Klammer auf der Einrueckung
    /// einer Methode; die Doc-Kommentare stehen vor dem `fn` und kommen damit
    /// gar nicht herein. Die Kommentarzeilen **im** Rumpf werden abgezogen: sie
    /// nennen `performClose:` und das Sichern in Prosa, und eine Nadel darf
    /// keine Prosa treffen.
    ///
    /// `pub(super)` aus demselben Grund wie [`diese_datei`] darueber: die
    /// Angleichproben schneiden ihre Ruempfe mit derselben Regel heraus.
    pub(super) fn rumpf(inhalt: &str, name: &str) -> String {
        let kopf = format!("fn {name}(");
        let beginn = inhalt
            .find(&kopf)
            .unwrap_or_else(|| panic!("{kopf} steht nicht in dieser Datei"));
        let rest = &inhalt[beginn..];
        let ende = rest
            .find("\n    }\n")
            .unwrap_or_else(|| panic!("der Rumpf von {name} endet nicht"));
        rest[..ende]
            .lines()
            .filter(|zeile| !zeile.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Was Sichern fuer den Zettel heisst, ist genau **einmal** erklaert (C4).
    ///
    /// Eine Erklaerungszaehlung ueber den ganzen Baum: eine zweite Fassung
    /// desselben Namens laesst sie rot werden, gleich in welcher Datei sie
    /// steht. **Was sie nicht sieht:** dieselbe Sache noch einmal gebaut unter
    /// einem anderen Namen — etwa ein zweites `text_sichern` von einer dritten
    /// Stelle aus. Dagegen haelt
    /// `krk-core/tests/baum.rs::nur_benannte_dateien_erreichen_das_atomare_schreiben`.
    #[test]
    fn das_sichern_des_zettels_ist_genau_einmal_erklaert() {
        let nadel = concat!("fn ", "zettel_sichern");
        let treffer: usize = quelldateien()
            .iter()
            .map(|(_, inhalt)| inhalt.matches(nadel).count())
            .sum();
        assert_eq!(
            treffer, 1,
            "das Sichern des Zettels ist nicht genau einmal erklärt"
        );
    }

    /// Genau **vier** Stellen sprechen das Sichern an (C4).
    ///
    /// Die vier Momente aus C4: der Tabklick, der Abschlussblock des Blattes,
    /// `shift+cmd+w` und das Beenden. Eine Aufruferzaehlung steht hier, weil das
    /// Abnahmekriterium die Zahl selbst zusagt; der Kopf von
    /// [`crate::quellbaum`] sagt, warum sie sonst nirgends stehen soll — sie ist
    /// in beide Richtungen blind, und der billigste Weg zurueck ins Gruene waere
    /// das Streichen eines berechtigten Fragers.
    ///
    /// **Was sie nicht sieht:** einen fuenften Moment, der ueber eine
    /// Hilfsfunktion sichert, statt selbst zu rufen. Die Zahl bliebe dann bei
    /// vier, und die Aufzaehlung im Doc-Kommentar von `zettel_sichern` waere
    /// still falsch geworden.
    #[test]
    fn genau_vier_stellen_sichern_den_zettel() {
        let name = concat!("zettel_", "sichern");
        let treffer: usize = quelldateien()
            .iter()
            .map(|(_, inhalt)| aufrufstellen(inhalt, name))
            .sum();
        assert_eq!(
            treffer, 4,
            "es sind nicht die vier Sicherungsmomente aus C4, die den Zettel sichern"
        );
    }

    /// `shift+cmd+w` sichert **vor** `performClose:` (C1, C4).
    ///
    /// Die Reihenfolge ist zugesagt, und sie ist der Grund, aus dem die Frage
    /// „was tut AppKit mit `performClose:` an einem Fenster mit anhaengendem
    /// Blatt" den Code nichts mehr kostet: gesichert wird unbedingt und vorher,
    /// also haelt die Zusage in beiden Ausgaengen.
    ///
    /// **Was die Probe nicht sieht:** ein Sichern, das aus dieser Methode in
    /// eine spaeter gerufene Hilfsfunktion gewandert ist. Dann stuende die Nadel
    /// hier nicht mehr, und die Probe faende sie auch nicht in der falschen
    /// Reihenfolge, sondern gar nicht — deshalb prueft sie beide Nadeln erst auf
    /// ihr Dasein.
    #[test]
    fn das_fensterschliessen_sichert_vor_dem_performclose() {
        let sichern = concat!("zettel_", "sichern(");
        let schliessen = concat!("perform", "Close(");
        let rumpf = rumpf(&diese_datei(), "fenster_schliessen");
        let stelle_sichern = rumpf
            .find(sichern)
            .expect("das Fensterschließen sichert den Zettel nicht");
        let stelle_schliessen = rumpf
            .find(schliessen)
            .expect("das Fensterschließen ruft performClose: nicht");
        assert!(
            stelle_sichern < stelle_schliessen,
            "der Zettel wird nach performClose: gesichert; C4 verlangt davor"
        );
    }

    /// „Fenster einblenden" sichert den Zettel **nicht** (C1).
    ///
    /// Die Gegenrichtung, und sie ist ein eigenes Abnahmekriterium: der Befehl
    /// kommt bei stehendem Blatt durch, fuehrt aber nicht aus dem Zettel heraus.
    /// Er holt dasselbe Fenster nach vorn, an dem das Blatt haengt; ein
    /// Schreibvorgang waere ein Schreiben ohne Anlass.
    ///
    /// **Was die Probe nicht sieht:** ein Sichern, das `fenster_zeigen` aus
    /// einer der Funktionen erbt, die es ruft. Sie liest den Rumpf und nicht den
    /// Aufrufbaum darunter.
    #[test]
    fn das_fenstereinblenden_sichert_den_zettel_nicht() {
        let rumpf = rumpf(&diese_datei(), "fenster_zeigen");
        for nadel in [
            concat!("zettel_", "sichern("),
            concat!("zettel_", "stand_uebernehmen("),
        ] {
            assert!(
                !rumpf.contains(nadel),
                "»Fenster einblenden« rührt den Zettel an: {nadel}"
            );
        }
    }
}

/// Was am Angleichen aus C1 bis C3 der Runde 13 **ohne Fenster** zu messen ist.
///
/// Der Befehl selbst laesst sich hier nicht ausfuehren: er braucht das
/// Fenstermodell, zwei Dateifenster und eine Aufteilung, und der
/// Anwendungsdelegierte ist ohne laufende Anwendung nicht zu bauen. Was die
/// Kriterien aus C1 und C2 am gebauten Buendel verlangen, steht deshalb im Plan
/// unter "Nutzerarbeit" und wird hier **nicht** behauptet.
///
/// Was bleibt, sind drei Aussagen ueber den **Baum**, und jede von ihnen ist
/// eine Falle, die kein Uebersetzer haelt: der fehlende Ausfuehrungszweig, die
/// Sichtbarkeitsfrage nach dem Einblenden statt davor, und ein Griff an Fokus
/// oder Sichtbarkeit, den C1 und C2 ausschliessen. Sie werden am Quelltext
/// gelesen, mit derselben Rumpfregel wie in [`zettelproben`].
///
/// **Was die drei nicht sehen:** eine Wirkung, die aus diesem Rumpf in eine
/// spaeter gerufene Hilfsfunktion gewandert ist. Sie lesen den Rumpf und nicht
/// den Aufrufbaum darunter.
#[cfg(test)]
mod angleichproben {
    use super::zettelproben::{diese_datei, rumpf};

    /// Der Ausfuehrungszweig steht **vor** dem Auffangzweig.
    ///
    /// Die eine Pflichtstelle je Kommando, die kein Uebersetzer einfordert:
    /// `Kommando::wirkungsbereich` und `bereich_des_kommandos` sind
    /// vollstaendige Fallunterscheidungen und halten den Bau an, dieses `match`
    /// endet auf `andere => self.bereichskommando(…)`. Ein Kommando ohne
    /// eigenen Zweig uebersetzt, besteht jede Probe, steht mit Namen und
    /// Kombination im Hauptmenue und tut nichts.
    #[test]
    fn der_befehl_steht_vor_dem_auffangzweig() {
        let zweig = concat!("Kommando::Ordner", "Angleichen =>");
        let auffang = concat!("andere ", "=> self.bereichskommando");
        let rumpf = rumpf(&diese_datei(), "kommando_ausfuehren");
        let stelle_zweig = rumpf
            .find(zweig)
            .expect("das Angleichen hat keinen Ausführungszweig und täte damit nichts");
        let stelle_auffang = rumpf
            .find(auffang)
            .expect("der Auffangzweig steht nicht mehr in kommando_ausfuehren");
        assert!(
            stelle_zweig < stelle_auffang,
            "der Zweig des Angleichens steht hinter dem Auffangzweig und ist unerreichbar"
        );
    }

    /// Die Sichtbarkeit wird **vor** dem Einblenden gefragt (C2).
    ///
    /// `bereich_einblenden` liefert `false` in drei Lagen, und nur eine davon
    /// ist eine Abweisung. Wer erst einblendet und dann den Rueckgabewert
    /// deutet, meldet ein zu schmales Fenster, wenn das andere Dateifenster
    /// laengst dasteht.
    #[test]
    fn die_sichtbarkeit_wird_vor_dem_einblenden_gefragt() {
        let frage = concat!("sicht", "bar(bereich)");
        let einblenden = concat!("bereich_ein", "blenden(bereich)");
        let rumpf = rumpf(&diese_datei(), "ordner_angleichen");
        let stelle_frage = rumpf
            .find(frage)
            .expect("das Angleichen fragt die Sichtbarkeit nicht am Fenstermodell");
        let stelle_einblenden = rumpf
            .find(einblenden)
            .expect("das Angleichen blendet den Zielbereich nicht ein");
        assert!(
            stelle_frage < stelle_einblenden,
            "die Sichtbarkeit wird nach dem Einblenden gefragt; C2 verlangt davor"
        );
    }

    /// Der Befehl ruehrt weder den Fokus noch eine Sichtbarkeit an, die er
    /// nicht herstellt (C1, C2).
    ///
    /// C1 sagt zu, dass danach dasselbe Dateifenster aktiv ist wie davor, und
    /// C2, dass in keiner Lage ein Bereich **ausgeblendet** wird. Beides ist am
    /// Rumpf abzulesen: die fuenf Nadeln sind die Wege, auf denen es sonst
    /// geschaehe.
    ///
    /// **Jede Nadel nennt den Namen, den dieser Baum wirklich ruft.** Bis zum
    /// 260818 stand hier `aktiv_setzen(` fuer die aktive Seite; der Setzer des
    /// Delegierten heisst `aktives_setzen`, und `aktiv_setzen(` ist davon keine
    /// Teilzeichenfolge. Die Nadel konnte also nicht anschlagen, und der
    /// Fokusteil dieser Probe mass nichts. `fokus_setzen(` und `fokus_holen(`
    /// sind die beiden Wege, auf denen der Fokus in dieser Datei ueberhaupt
    /// wechselt, und standen gar nicht erst da.
    #[test]
    fn das_angleichen_ruehrt_weder_fokus_noch_sichtbarkeit_an() {
        let rumpf = rumpf(&diese_datei(), "ordner_angleichen");
        for nadel in [
            concat!("aktives_", "setzen("),
            concat!("fokus_", "setzen("),
            concat!("fokus_", "holen("),
            concat!("bereich_um", "schalten("),
            concat!("aus", "blenden("),
        ] {
            assert!(
                !rumpf.contains(nadel),
                "das Angleichen greift an Fokus oder Sichtbarkeit: {nadel}"
            );
        }
    }
}

/// Der Nachzug der Fokusanzeige bleibt frei von der Auslegung (C9).
///
/// **Warum am Quelltext und nicht an einem Rueckgabewert.** Die Zusage lautet
/// „diese Funktion schreibt Farben und Titel und sonst nichts", und das ist eine
/// Aussage darueber, was sie **nicht** tut; an keinem Ergebnis ist sie
/// abzulesen. Der Kopf von [`crate::quellbaum`] beschreibt die Bauform und sagt
/// auch, was sie nicht kann.
///
/// **Die Nadeln stehen zusammengesetzt da**, wie in den Nachbarmodulen: diese
/// Proben liegen in der Datei, die sie lesen.
#[cfg(test)]
mod fokusnachzugproben {
    use super::zettelproben::{diese_datei, rumpf};

    /// [`Anwendungsdelegierter::fokusanzeige_nachziehen`] legt nichts aus und
    /// setzt nichts aktiv.
    ///
    /// **Der Ring, den diese Probe offen haelt.** `anwenden` setzt `setHidden`,
    /// und eine ausgeblendete Ansicht, die den Ersthelfer haelt, laesst AppKit
    /// den Rang neu vergeben — also `makeFirstResponder:` erneut rufen und
    /// damit die Meldung des Hauptfensters ein zweites Mal ausloesen, die
    /// diese Funktion gerade erst gerufen hat.
    ///
    /// **Seit dem 260819 traegt sie ihr Gewicht wirklich.** Bis dahin war der
    /// Nachzug der Anzeige der einzige Empfaenger jener Meldung, und die Frage
    /// stellte sich niemandem. Seither haengt
    /// `aktives_dem_ersthelfer_nachziehen` als zweiter daran, und **der** geht
    /// ueber `aktives_setzen` sehr wohl bis `anwenden` durch. Die beiden
    /// nebeneinander zu stellen statt ineinander ist damit der ganze Unterschied
    /// zwischen einem Ring und keinem, und das laedt dazu ein, sie beim naechsten
    /// Mal zusammenzulegen.
    ///
    /// **Was sie nicht sieht:** einen Weg, der ueber eine dritte Funktion
    /// dorthin fuehrt, ohne sie hier beim Namen zu nennen.
    #[test]
    fn der_nachzug_der_anzeige_ruehrt_die_auslegung_nicht_an() {
        let rumpf = rumpf(&diese_datei(), "fokusanzeige_nachziehen");
        for nadel in [
            concat!("an", "wenden("),
            concat!("set", "Hidden("),
            concat!("aufteilung_", "nachziehen("),
            concat!("aktives_", "setzen("),
        ] {
            assert!(
                !rumpf.contains(nadel),
                "der Nachzug der Fokusanzeige greift an die Auslegung: {nadel}"
            );
        }
    }

    /// Der Nachzug der Anzeige schreibt weiter Rahmen **und** Titel (C9, C11).
    ///
    /// Die Gegenprobe zur Verneinung darueber: eine Funktion, die nichts mehr
    /// tut, bestuende jene muehelos. Beide Schreibvorgaenge sind zugesagt, der
    /// erste von C9 und der zweite vom achten Abnahmekriterium von C11.
    #[test]
    fn der_nachzug_der_anzeige_schreibt_rahmen_und_titel() {
        let rumpf = rumpf(&diese_datei(), "fokusanzeige_nachziehen");
        for nadel in [
            concat!("rahmen_", "setzen("),
            concat!("titel_", "nachziehen("),
        ] {
            assert!(
                rumpf.contains(nadel),
                "der Nachzug der Fokusanzeige schreibt nicht mehr: {nadel}"
            );
        }
    }
}

/// Die Zusage, dass eine geaenderte Sichtbarkeit auf den Schirm kommt.
///
/// **Warum am Quelltext und nicht am Verhalten.** Der Weg von der Sichtbarkeit
/// im Fenstermodell bis zu `setHidden:` laeuft ueber ein `NSSplitView`, und die
/// Ansichten stehen erst, wenn KRK im Vordergrund laeuft; kein Agent kann den
/// Abnahmelauf fahren. Was ohne Fenster pruefbar bleibt, ist die Verdrahtung:
/// dass die eine Stelle, die die Sichtbarkeit aendert, den Nachzug ruft, und
/// dass sie ihn **vor** allem ruft, was einen Ersthelfer setzt.
///
/// **Der Defekt, gegen den sie geschrieben sind.** `784840c` hat das Lesen des
/// Editors auf einen Arbeitsfaden gelegt; seither laeuft
/// [`Anwendungsdelegierter::editorausgang_behandeln`] aus dem Einzugstakt und
/// nicht mehr im Rumpf von [`Anwendungsdelegierter::kommando_ausfuehren`], das
/// den Nachzug bis dahin fuer jeden Weg hinter sich brachte. `f4` und `cmd+e`
/// blendeten den Editor damit im Modell ein und nicht auf dem Schirm
/// (`shared/issues/260820-1034_*`).
///
/// **Was sie nicht sehen:** einen zweiten Schreiber der Sichtbarkeit neben
/// `Fenstermodell::umschalten` und `::einblenden`. Dagegen haelt, dass
/// `Fenstermodell::sichtbar_setzen` privat ist.
#[cfg(test)]
mod sichtbarkeitsproben {
    use super::zettelproben::{diese_datei, rumpf};

    /// Wer die Sichtbarkeit aendert, schreibt sie auf den Schirm.
    ///
    /// Die eine Stelle ist `sichtbarkeit_aendern`; beide Wege in das
    /// Fenstermodell — `bereich_umschalten` und `bereich_einblenden` — gehen
    /// durch sie. Faellt der Ruf hier weg, steht die neue Sichtbarkeit wieder
    /// nur im Modell, sobald der Aenderer keinen Befehlsrumpf hinter sich hat.
    #[test]
    fn die_geaenderte_sichtbarkeit_kommt_auf_den_schirm() {
        let nadel = concat!("aufteilung_", "nachziehen(");
        let rumpf = rumpf(&diese_datei(), "sichtbarkeit_aendern");
        assert!(
            rumpf.contains(nadel),
            "die Sichtbarkeitsaenderung zieht die Aufteilung nicht nach und bleibt damit im Modell stehen"
        );
    }

    /// Die Flaeche steht auf dem Schirm, **bevor** ein Ersthelfer gesetzt wird.
    ///
    /// `nach_dem_sichtbarkeitswechsel` setzt den Fokus, und `fokus_holen` setzt
    /// ihn gleich danach ein zweites Mal. Beide rufen `makeFirstResponder:`,
    /// und der trifft eine Ansicht, die AppKit noch als ausgeblendet fuehrt,
    /// wenn der Nachzug erst hinter der Schleife stuende — dieselbe Trennung,
    /// die `a6b3818` fuer das Angleichen gezogen hat.
    #[test]
    fn der_nachzug_steht_vor_den_bereichsnachzuegen() {
        let nachzug = concat!("aufteilung_", "nachziehen(");
        let bereichsnachzug = concat!("nach_dem_", "sichtbarkeitswechsel(");
        let rumpf = rumpf(&diese_datei(), "sichtbarkeit_aendern");
        let stelle_nachzug = rumpf
            .find(nachzug)
            .expect("die Sichtbarkeitsaenderung zieht die Aufteilung nicht nach");
        let stelle_bereich = rumpf
            .find(bereichsnachzug)
            .expect("die Sichtbarkeitsaenderung zieht die einzelnen Bereiche nicht mehr nach");
        assert!(
            stelle_nachzug < stelle_bereich,
            "der Nachzug der Aufteilung steht hinter den Bereichsnachzügen; der Fokus trifft dann eine noch ausgeblendete Ansicht"
        );
    }

    /// Die erste Anweisung eines Rumpfs, ohne Kommentar- und Leerzeilen.
    ///
    /// [`rumpf`] liefert die `fn`-Zeile mit und zieht die Kommentarzeilen ab.
    /// Der Rumpf beginnt hinter der Klammer, die die Signatur schliesst; sie
    /// steht heute auf der `fn`-Zeile, darf aber ueber mehrere gehen.
    fn erste_anweisung(rumpf: &str) -> String {
        let mut im_rumpf = false;
        for zeile in rumpf.lines() {
            if !im_rumpf {
                im_rumpf = zeile.trim_end().ends_with('{');
                continue;
            }
            let zeile = zeile.trim();
            if !zeile.is_empty() {
                return zeile.to_owned();
            }
        }
        panic!("der Rumpf hat keine Anweisung");
    }

    /// Die Fortsetzung des Editors misst als **erste** Anweisung.
    ///
    /// `editorausgang_behandeln` laeuft aus dem Einzugstakt und damit lange
    /// nach dem Befehl, der das Oeffnen angefordert hat. Sie beginnt deshalb
    /// wie `kommando_ausfuehren`: erst messen, solange Modell und Schirm
    /// dieselbe Sichtbarkeit meinen, dann erst eine aendern. Ohne die Messung
    /// naehme der Nachzug dem Nutzer eine Ziehbewegung, die er waehrend des
    /// Lesens gemacht hat.
    ///
    /// **Gehalten wird die Stellung und nicht eine Reihenfolge gegen eine
    /// Nadel.** Bis zum 260823 verglich die Probe die Messung mit dem ersten
    /// `fokus_holen(`; der Rumpf aendert die Sichtbarkeit aber an zwei Stellen,
    /// im Zweig `Geoeffnet | SchonOffen` ueber `fokus_holen` und im Zweig
    /// `Abgewiesen` ueber `editor_ausblenden`. Eine Messung, die in den ersten
    /// Zweig gewandert waere, haette jenen Vergleich bestanden und den zweiten
    /// Zweig ungemessen laufen lassen
    /// (`shared/issues/260823-0733_*_die-probe-zur-editorfortsetzung-*`). Die
    /// Zusage des Doc-Kommentars von `editorausgang_behandeln` lautet "gemessen
    /// wird, bevor irgendetwas die Sichtbarkeit anfasst", und genau die haelt
    /// diese Fassung — auch fuer den dritten Aenderer, den noch niemand
    /// geschrieben hat.
    #[test]
    fn die_editorfortsetzung_misst_als_erste_anweisung() {
        let messung = concat!("bildschirmbreiten_", "uebernehmen(");
        let rumpf = rumpf(&diese_datei(), "editorausgang_behandeln");
        assert!(
            erste_anweisung(&rumpf).contains(messung),
            "die Fortsetzung des Editors misst nicht als erste Anweisung; jeder Zweig davor ändert die Sichtbarkeit auf einer überholten Zahl"
        );
    }
}

/// Die Verdrahtung des Rundwegs von `cmd+e`, soweit sie ohne Fenster zu lesen
/// ist.
///
/// **Warum am Quelltext und nicht am Verhalten.** Was `vorschau_danach` bewirkt,
/// steht erst am laufenden Buendel zu sehen, und der Abnahmelauf verlangt KRK im
/// Vordergrund. Was ohne Fenster pruefbar bleibt, ist dieselbe Verdrahtung, die
/// [`sichtbarkeitsproben`] fuer die Sichtbarkeit haelt: welcher Rufer welchen
/// Wahrheitswert uebergibt, und dass die abgelehnte Nachfrage ihn nicht liest.
///
/// **Der Uebersetzer haelt davon nichts.** Ein `bool` an einer Aufrufstelle
/// vertauscht uebersetzt, und ein `{ .. }`, das zu `{ vorschau_danach }` wird,
/// auch. `opt+cmd+e` bekaeme im ersten Fall eine andere Bedeutung als seit der
/// Editor-Runde, und im zweiten drehte "Abbrechen" die Wahl des Nutzers vom
/// 260823-0942 um (`shared/issues/260823-1034_*_das-neue-feld-vorschau-danach-*`).
///
/// **Was sie nicht sehen:** einen dritten Rufer von
/// [`Anwendungsdelegierter::editor_schliessen`], der einen eigenen Wert
/// uebergibt. Dagegen haelt, dass die Regel des Rundwegs genau einen Aufrufer
/// hat (`crate::kommandos::rundweg::tests::die_regel_hat_genau_einen_aufrufer`).
#[cfg(test)]
mod rundwegproben {
    use super::zettelproben::{diese_datei, rumpf};

    /// Der Rumpf einer Methode dieser Datei, ohne Kommentare.
    fn rumpf_von(name: &str) -> String {
        rumpf(&diese_datei(), name)
    }

    /// `opt+cmd+e` schliesst und laesst die Flaeche leer.
    ///
    /// Die Kombination traegt ihre Bedeutung seit der Editor-Runde, und
    /// `resources/default-keymap.toml` schliesst an `editor_umschalten` aus,
    /// dass eine ausgelieferte Kombination einer abgenommenen Runde ihre
    /// Bedeutung wechselt. Ein `true` an dieser Aufrufstelle taete genau das.
    #[test]
    fn opt_cmd_e_schliesst_ohne_die_vorschau_danach() {
        let rumpf = rumpf_von("kommando_ausfuehren");
        let zweig = concat!("Kommando::EditorSchliessen => self.editor_", "schliessen(");
        let stelle = rumpf
            .find(zweig)
            .expect("der Ausfuehrungszweig schliesst den Editor nicht mehr");
        assert!(
            rumpf[stelle..].starts_with(&format!("{zweig}false)")),
            "opt+cmd+e übergibt nicht mehr `false` und blendet damit die Vorschau ein, die es seit der Editor-Runde leer lässt"
        );
    }

    /// Der Rueckweg des Rundwegs holt die Vorschau zurueck.
    ///
    /// Der Nutzerentscheid vom 260823-0942 sagt fuer `cmd+e` im Editor: „die
    /// Vorschau zeigt die Datei wieder". Ein `false` an dieser Aufrufstelle
    /// liesse die Flaeche leer und machte den Rueckweg zu `opt+cmd+e`.
    #[test]
    fn der_rueckweg_schliesst_mit_der_vorschau_danach() {
        let rumpf = rumpf_von("editor_rundweg");
        let zweig = concat!(
            "Rundweg::ZurueckInDieDateiliste => self.editor_",
            "schliessen("
        );
        let stelle = rumpf
            .find(zweig)
            .expect("der Rueckweg schliesst den Editor nicht mehr");
        assert!(
            rumpf[stelle..].starts_with(&format!("{zweig}true)")),
            "der Rückweg von cmd+e übergibt nicht mehr `true` und holt die Vorschau nicht zurück"
        );
    }

    /// Die ausgefuehrte Nachfrage liest das Feld.
    ///
    /// Die Gegenprobe zur Verneinung darunter: ein Zweig, der das Feld nirgends
    /// mehr liest, bestuende jene muehelos, und `cmd+e` liesse die Flaeche leer.
    #[test]
    fn die_ausgefuehrte_nachfrage_liest_das_feld() {
        let nadel = concat!("if vorschau_", "danach");
        assert!(
            rumpf_von("anlass_ausfuehren").contains(nadel),
            "die ausgeführte Nachfrage liest `vorschau_danach` nicht mehr; der Rückweg von cmd+e lässt die Fläche dann leer"
        );
    }

    /// „Abbrechen" liest das Feld **nicht**.
    ///
    /// Bleibt der Editor stehen, darf die Vorschau ihn gerade nicht
    /// verdraengen. Wer hier spaeter `if vorschau_danach { … }` ergaenzt, dreht
    /// die Wahl des Nutzers vom 260823-0942 um, und ohne diese Probe wuerde
    /// nichts rot.
    #[test]
    fn die_abgelehnte_nachfrage_liest_das_feld_nicht() {
        let nadel = concat!("vorschau_", "danach");
        assert!(
            !rumpf_von("anlass_unterbleibt").contains(nadel),
            "das abgebrochene Schließen liest `vorschau_danach`; die Vorschau verdrängt dann einen Editor, der stehen bleibt"
        );
    }
}

#[cfg(test)]
mod loeschzielproben {
    use super::*;
    use crate::pruefordner::Pruefordner;

    /// Der Einhaengepunkt der `/home`-Automatik, der eine Ort dieses Baums, an
    /// dem `NSURLVolumeIsLocalKey` `false` antwortet, ohne dass eine Probe ein
    /// Netzlaufwerk einhaengen muesste.
    ///
    /// Er steht auch in `super::super::volumes` neben den dortigen Proben; die
    /// Begruendung im Einzelnen steht dort. Eine gemeinsame Fassung waere ein
    /// `pub(crate)` an einer `#[cfg(test)]`-Konstante quer durch die Kiste, und
    /// das ist teurer als die zwei Zeilen.
    const AUTOMATIK_HOME: &str = "/System/Volumes/Data/home";

    /// Die drei Werte der Rueckfrage zu einem Ordner, mit aufgeloestem Pfad.
    ///
    /// Genau der Weg des Rumpfes: der Ordner wird einmal aufgeloest und geht
    /// aufgeloest an [`Anwendungsdelegierter::loeschtexte`], das ihn von dort an
    /// nicht mehr anfasst.
    fn texte(ordner: &Path, pfade: Vec<PathBuf>) -> (String, String, bool) {
        let auswahl = Auswahl { pfade, ordner: 0 };
        let aufgeloest = std::fs::canonicalize(ordner).ok();
        assert!(
            aufgeloest.is_some(),
            "{} liess sich nicht aufloesen, also misst diese Probe die Verdrahtung nicht",
            ordner.display()
        );
        Anwendungsdelegierter::loeschtexte(&auswahl, ordner, aufgeloest)
    }

    /// Frage und Erlaeuterung in einem Stueck.
    ///
    /// Die Rangfolge aus C3 entscheidet, welcher Grund in der Frage steht und
    /// welcher im Absatz der Erlaeuterung; welcher es ist, ist hier nicht die
    /// Frage, sondern **ob der Grund ueberhaupt der ist, der zur Tatsache
    /// gehoert**. Beide Texte zusammen zu lesen macht die Probe unabhaengig von
    /// der Rangfolge, die `loeschwarnung` schon einzeln misst.
    fn beide(ordner: &Path, pfade: Vec<PathBuf>) -> (String, bool) {
        let (frage, erlaeuterung, laut) = texte(ordner, pfade);
        (format!("{frage}\n{erlaeuterung}"), laut)
    }

    /// Ein Arbeitsbaum kommt als Arbeitsbaum an und nicht als Netzlaufwerk.
    ///
    /// **Die Probe misst die Verdrahtung und nicht die Ausloeser.** Beide
    /// Ausloeser sind einzeln gemessen — `volumes::liegt_auf_netzlaufwerk` in
    /// seiner Datei, `arbeitsbaum::beruehrt_einen_arbeitsbaum` in seiner —, und
    /// `loeschwarnung::warngruende` urteilt ueber die fertigen Tatsachen. Was
    /// dazwischen niemand gelesen hat, sind die zwei Zeilen in
    /// [`Anwendungsdelegierter::loeschtexte`], die die Antworten in die Felder
    /// des Ziels legen: ein Tausch der beiden Feldnamen uebersetzt, laesst jede
    /// jener Proben gruen und macht aus einem Arbeitsbaum ein Netzlaufwerk
    /// (`issues/260817-1759_*`).
    ///
    /// Der Prueforder traegt ein `.git` und liegt auf dem eingebauten
    /// Datenband. Die beiden Tatsachen sind damit **verschieden** — Arbeitsbaum
    /// ja, Netzlaufwerk nein —, und nur bei verschiedenen Tatsachen ist ein
    /// Tausch ueberhaupt sichtbar.
    #[test]
    fn ein_arbeitsbaum_kommt_nicht_als_netzlaufwerk_an() {
        let ordner = Pruefordner::neu("loeschtexte-arbeitsbaum");
        ordner.ordner(".git");
        let eintrag = ordner.datei("eine-datei.txt", b"Inhalt");

        let (text, laut) = beide(ordner.pfad(), vec![eintrag]);

        assert!(
            laut,
            "das Ziel traegt einen Warngrund und die Frage ist still: {text}"
        );
        assert!(
            text.contains("aus einem Git-Arbeitsbaum"),
            "der Arbeitsbaum erreicht sein Feld nicht: {text}"
        );
        assert!(
            !text.contains("von einem Netzlaufwerk"),
            "ein lokaler Ordner wird als Netzlaufwerk angesagt: {text}"
        );
    }

    /// Ein nicht lokaler Datentraeger kommt als Netzlaufwerk an und nicht als
    /// Arbeitsbaum.
    ///
    /// Die Gegenprobe, und sie ist noetig: ohne sie waere ein
    /// [`Anwendungsdelegierter::loeschtexte`], das **nie** ein Netzlaufwerk
    /// meldet, ebenso gruen wie die richtige Fassung. Zusammen halten die beiden
    /// Proben den Tausch von beiden Seiten fest.
    ///
    /// # Warum die Vorbedingung mitgeprueft wird
    ///
    /// Ist die `/home`-Automatik in `/etc/auto_master` abgeschaltet, steht unter
    /// [`AUTOMATIK_HOME`] ein gewoehnlicher, lokaler Ordner, und die Probe
    /// wuerde rot, ohne dass an der Verdrahtung etwas falsch waere. Geprueft
    /// wird deshalb zuerst die Geraetekennung aus `stat(2)`: ein Einhaengepunkt
    /// traegt eine andere als der Ordner ueber ihm. Fehlt er, **haelt die Probe
    /// an statt sich zu ueberspringen** — dieselbe Wahl wie bei der
    /// Schwesterprobe in `super::super::volumes`.
    #[test]
    fn ein_netzlaufwerk_kommt_nicht_als_arbeitsbaum_an() {
        use std::os::unix::fs::MetadataExt;

        let einhaengepunkt = Path::new(AUTOMATIK_HOME);
        let eigen = std::fs::metadata(einhaengepunkt)
            .unwrap_or_else(|fehler| panic!("{AUTOMATIK_HOME} ist nicht lesbar: {fehler}"));
        let darueber = std::fs::metadata(
            einhaengepunkt
                .parent()
                .expect("der Pfad der Automatik hat einen uebergeordneten Ordner"),
        )
        .expect("der Ordner ueber der Automatik ist nicht lesbar");
        assert_ne!(
            eigen.dev(),
            darueber.dev(),
            "unter {AUTOMATIK_HOME} steht kein eigener Einhaengepunkt, \
             also misst diese Probe den nicht lokalen Datentraeger nicht; \
             ist die /home-Automatik in /etc/auto_master abgeschaltet?"
        );

        let (text, laut) = beide(einhaengepunkt, vec![einhaengepunkt.join("nicht-da")]);

        assert!(
            laut,
            "ein nicht lokaler Datentraeger laesst die Frage still: {text}"
        );
        assert!(
            text.contains("von einem Netzlaufwerk"),
            "das Netzlaufwerk erreicht sein Feld nicht: {text}"
        );
        assert!(
            !text.contains("aus einem Git-Arbeitsbaum"),
            "ein Datentraeger wird als Arbeitsbaum angesagt: {text}"
        );
    }
}
