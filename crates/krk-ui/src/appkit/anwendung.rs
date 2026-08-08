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
//! Funktion; ein Zeichen gehoert der Sprungmarke aus C2 und damit immer dem
//! aktiven Dateifenster, weil sie die Liste durchsucht, die vor dem Nutzer
//! steht.
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
//! Seit Schritt 18 gibt es zwei fokussierbare Bereiche, und
//! [`Anwendungsdelegierter::kommando_ausfuehren`] fragt **einmal**, wo der
//! Fokus steht:
//!
//! ```text
//!  Kommando ──> steht ein Blatt? ──> fokus() ──> fokus::wirkt(Wirkungsbereich)
//!                                       │                    │  nein: nichts
//!                                       │                    ▼
//!                                       │            fensterweiter Befehl
//!                                       └───Adresse──> Dateifenster / Leiste
//! ```
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

use std::cell::{Cell, OnceCell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use dispatch2::DispatchQueue;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSRunLoop, NSRunLoopCommonModes,
    NSTimer,
};

use krk_core::ablage::sitzung::Sitzungsschreiber;
use krk_core::ablage::{
    Ablage, Datei, Einstellungen, Fensterseite, Lesezeichenliste, Sitzung, Verschiebung,
    einstellungen, lesezeichen, pfade,
};
use krk_core::operation::{
    self, Abschluss, Art, Auftrag, Bericht, Konfliktantwort, Konfliktentscheid, Lauf, Meldung,
    Namensfehler, freier_name,
};
use krk_core::stapelumbenennen::Vorschau;
use krk_core::tasten::belegung;
use krk_core::tasten::{Belegung, Kommando, Tastendruck};

use crate::auffrischung::{self, Dateifenstersicht};
use crate::belegungsmodell::Belegungsmodell;
use crate::fenstermodell::{BREITENSCHRITT, Bereich, Fenstermodell};
use crate::kommandos::fokus::{self, Fokus};
use crate::kommandos::operationen::{self, Anlegeart, Konfliktfrage, Vorgangszustand};
use crate::leistenmodell::Ort;
use crate::messmodus::{Anweisung, Aufgabe, Handlung, Messlauf, Sitzungslage, Zustand};
use crate::tabs::{Auswahlversuch, Tabliste};

use super::aufteilung::Aufteilung;
use super::belegungsansicht::{self, Belegungsquelle};
use super::bildtakt::{self, Zeichenende};
use super::blaetter::{
    Blattgriff, konflikt, loeschbestaetigung, namenseingabe, stapelumbenennen, uebersprungen,
};
use super::ereignisse::{self, Eingabe, Tastenabgriff};
use super::fenster::{self, FensterDelegierter};
use super::fsevents::Dateisystemwache;
use super::hinweis;
use super::leiste::Leiste;
use super::menue;
use super::papierkorb::Systempapierkorb;
use super::tabelle::Dateifenster;
use super::terminal;
use super::volumes::{Datentraeger, Datentraegerwache, Wechsel};
use super::vorschau::Vorschaufenster;

/// Der Rueckgabewert, mit dem ein Messlauf ohne Bildschirm endet.
const OHNE_BILDSCHIRM: i32 = 3;

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
            Art::InDenPapierkorb | Art::EndgueltigLoeschen | Art::UmbenennenImStapel { .. } => {}
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
    /// Die beiden Dateifenster, links zuerst.
    dateifenster: OnceCell<[Dateifenster; 2]>,
    /// Die Lesezeichen- und Geraeteleiste (C5), der zweite fokussierbare
    /// Bereich.
    leiste: OnceCell<Leiste>,
    /// Das Vorschaufenster (C6), der dritte fokussierbare Bereich.
    vorschau: OnceCell<Retained<Vorschaufenster>>,
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
    /// die Rueckfrage vor dem endgueltigen Loeschen oder die Abschlussliste.
    ///
    /// Es steht hier, damit die Escape-Taste es schliessen kann. Ein `NSButton`
    /// traegt genau eine Tastenentsprechung, und die Eingabetaste liegt in der
    /// Rueckfrage auf "Abbrechen"; der zweite Weg zum Abbruch laeuft deshalb
    /// ueber den Befehl `abbrechen` aus `resources/default-keymap.toml`.
    offenes_blatt: RefCell<Option<Blattgriff>>,
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

        /// Der Menueeintrag "Fenster einblenden" (C7).
        ///
        /// Er erreicht den Delegierten ueber die Antwortkette, an deren Ende
        /// `NSApplication` seinen Delegierten fragt. Genau deshalb traegt der
        /// Eintrag kein festes Ziel: er bleibt damit auch dann bedienbar, wenn
        /// kein Fenster offen ist, und das ist der Fall, fuer den es ihn gibt.
        // SAFETY: Die Signatur ist die einer gewoehnlichen Menueaktion: ein
        // Argument, der Absender.
        #[unsafe(method(fensterEinblenden:))]
        fn fenster_einblenden(&self, _absender: Option<&AnyObject>) {
            self.fenster_zeigen();
        }

        /// Der Menueeintrag "Fenster schliessen" (C7).
        ///
        /// Ein eigener Selektor und nicht `performClose:`, und das ist der
        /// ganze Zweck: zu einem Menueeintrag mit `performClose:` stellt AppKit
        /// von sich aus eine Zweitform "Close All" auf Opt+Shift+Cmd+W dazu,
        /// eine Kombination, die weder in der Belegung steht noch umbelegbar
        /// ist (gemessen am 260804-1040). Am Verhalten aendert der Umweg
        /// nichts: der Delegierte ruft `performClose:` am Fenster selbst.
        // SAFETY: Die Signatur ist die einer gewoehnlichen Menueaktion: ein
        // Argument, der Absender.
        #[unsafe(method(fensterSchliessen:))]
        fn fenster_schliessen_aktion(&self, _absender: Option<&AnyObject>) {
            self.fenster_schliessen();
        }

        /// Der Menueeintrag "KRK beenden" (C3).
        ///
        /// Ein eigener Selektor und nicht `terminate:`, aus demselben Grund wie
        /// bei `fensterSchliessen:` daneben: zu einem Menueeintrag mit
        /// `terminate:` stellt AppKit von sich aus eine Zweitform "Quit and
        /// Keep Windows" auf Opt+Cmd+Q dazu, mit englischer Beschriftung und
        /// einer Kombination, die weder in der Belegung steht noch umbelegbar
        /// ist (gemessen am 260805-0753 am laufenden Buendel,
        /// `issues/260805-0753_*_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md`).
        /// Am Verhalten aendert der Umweg nichts: der Delegierte ruft
        /// `terminate:` an `NSApplication` selbst.
        // SAFETY: Die Signatur ist die einer gewoehnlichen Menueaktion: ein
        // Argument, der Absender.
        #[unsafe(method(beenden:))]
        fn beenden_aktion(&self, _absender: Option<&AnyObject>) {
            self.beenden();
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

        /// KRK wird beendet: den letzten Sitzungsstand schreiben.
        ///
        /// Der eine Schreibvorgang ohne Ruecksicht auf den Takt, den
        /// `### Frage 4` des Plans neben der Buendelung zusagt.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(applicationWillTerminate:))]
        fn wird_beendet(&self, _meldung: &NSNotification) {
            self.sitzung_vormerken();
            let sitzung = self.sitzung_bauen();
            let mut schreiber = self.ivars().sitzungsschreiber.borrow_mut();
            if let Some(schreiber) = schreiber.as_mut() {
                let jetzt = Instant::now();
                let _ = schreiber.vormerken(sitzung, jetzt);
                let _ = schreiber.beenden(jetzt);
            }
        }
    }
);

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
            dateifenster: OnceCell::new(),
            leiste: OnceCell::new(),
            vorschau: OnceCell::new(),
            vorschau_nachtrag: RefCell::new(None),
            ablage: RefCell::new(None),
            tastenabgriff: RefCell::new(None),
            belegungsansicht: RefCell::new(None),
            dateisystemwache: RefCell::new(None),
            datentraegerwache: OnceCell::new(),
            sitzungsschreiber: RefCell::new(None),
            schreibfehler_gemeldet: Cell::new(false),
            vorgang: RefCell::new(None),
            offenes_blatt: RefCell::new(None),
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

        let dateifenster = [
            Dateifenster::bauen(mtm, Tabliste::aus_zustand(&sitzung.fenster[0])),
            Dateifenster::bauen(mtm, Tabliste::aus_zustand(&sitzung.fenster[1])),
        ];
        let leiste = Leiste::bauen(mtm);
        let vorschau = Vorschaufenster::bauen(mtm);
        let aufteilung = Aufteilung::bauen(
            mtm,
            [&dateifenster[0], &dateifenster[1]],
            leiste.sicht(),
            vorschau.sicht(),
        );
        let fenster_delegierter = FensterDelegierter::neu(
            mtm,
            [
                dateifenster[0].quelle().retain(),
                dateifenster[1].quelle().retain(),
            ],
        );
        let fenster = fenster::hauptfenster(mtm, aufteilung.sicht(), &fenster_delegierter);

        // Erst festhalten, dann anzeigen: das Fenster haelt seinen Delegierten
        // schwach, die Tabelle haelt Datenquelle und Delegierten schwach.
        let _ = ivars.dateifenster.set(dateifenster);
        let _ = ivars.leiste.set(leiste);
        let _ = ivars.vorschau.set(vorschau);
        let _ = ivars.aufteilung.set(aufteilung);
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
            // Jede Navigation setzt die Dateisystembeobachtung neu auf. Auch
            // dieser Rueckruf haelt den Delegierten **schwach**, aus demselben
            // Grund wie der darueber.
            let schwach = objc2::rc::Weak::from_retained(&self.retain());
            self.dateifenster(seite)
                .quelle()
                .ordnerwechsel_setzen(Box::new(move || {
                    if let Some(selbst) = schwach.load() {
                        selbst.dateisystemwache_nachziehen();
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
        }

        self.aufteilung_nachziehen();
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
                let geladen = ablage.laden::<Sitzung>(Datei::Sitzung);
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
        *ivars.sitzungsschreiber.borrow_mut() = Some(ablage.sitzungsschreiber());
        let (sitzung, meldung) = ablage.laden::<Sitzung>(Datei::Sitzung).mit_meldung();
        meldungen.extend(meldung);
        // Die Einstellungen aus C11, ueber denselben Zugang. Der Aufruf legt
        // `settings.toml` beim ersten Start an; ohne diese Anlage haette der
        // Nutzer nichts zu pflegen, weil in dieser Runde keine Ansicht die
        // Datei schreibt.
        let (eingestellt, meldung) = einstellungen::laden(&ablage).mit_meldung();
        *ivars.einstellungen.borrow_mut() = eingestellt;
        meldungen.extend(meldung);
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
        self.bereich_einblenden(Bereich::Vorschau);
        true
    }

    /// Fuellt die Leiste und haengt ihren Rueckruf ein (C5).
    ///
    /// Die Lesezeichen kommen aus `bookmarks.toml`, die Geraete vom System.
    /// Eine beschaedigte Lesezeichendatei geht denselben Weg wie eine
    /// beschaedigte Sitzung: Auslieferungszustand, also eine leere Liste, und
    /// eine Meldung in der Statuszeile.
    fn leiste_einrichten(&self, meldungen: &mut Vec<String>) {
        let geladen = match self.ivars().ablage.borrow().as_ref() {
            Some(ablage) => {
                let (liste, meldung) = ablage
                    .laden::<Lesezeichenliste>(Datei::Lesezeichen)
                    .mit_meldung();
                meldungen.extend(meldung);
                liste
            }
            // Ohne Ablageordner gibt es nichts zu laden und nichts zu sichern.
            // Die Meldung darueber hat `sitzung_laden` schon gestellt; eine
            // zweite waere dieselbe Auskunft ein zweites Mal.
            None => Lesezeichenliste::default(),
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

    /// Der Nutzer hat in der Leiste einen Eintrag ausgewaehlt (C5).
    ///
    /// **Die Auswahl setzt den Ordner des aktiven Dateifensters, ohne den Tab
    /// zu wechseln**: [`DateifensterQuelle::ordner_lesen`] liest in den
    /// sichtbaren Tab, denselben Weg, den jede Navigation aus C2 geht. Ein
    /// eigener Lesepfad fuer die Leiste entstuende sonst.
    ///
    /// Zeigt ein Lesezeichen auf einen Ordner, den es nicht mehr gibt, nennt
    /// die Statuszeile den Grund, und es wird nichts gelesen. Das ist die
    /// Zusage aus C5, "statt kommentarlos nichts zu tun", und sie ist eine
    /// **Befehlsantwort**: der Nutzer hat die Auswahl eben selbst bewegt.
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
                    auswahl.ordner.display()
                ),
            );
            return;
        }
        self.dateifenster(aktiv)
            .quelle()
            .ordner_lesen(&auswahl.ordner, None);
        self.sitzung_vormerken();
    }

    /// Schreibt die Lesezeichen nach `bookmarks.toml` (C5).
    ///
    /// Nach **jeder** Aenderung, wie `### Frage 4` des Plans es fuer diese
    /// Datei vorschreibt, und nicht gebuendelt wie die Sitzung: eine Aenderung
    /// an den Lesezeichen ist eine Handlung des Nutzers und keine Nebenwirkung
    /// des Arbeitens, davon gibt es wenige, und jede soll einen Absturz
    /// ueberleben.
    fn lesezeichen_sichern(&self, seite: Fensterseite) {
        let liste = self.leiste().quelle().lesezeichenliste();
        let ergebnis = match self.ivars().ablage.borrow().as_ref() {
            Some(ablage) => ablage.sichern(Datei::Lesezeichen, &liste),
            None => return,
        };
        if let Err(fehler) = ergebnis {
            self.antwort_zeigen(
                seite,
                &format!("die Lesezeichen liessen sich nicht sichern: {fehler}"),
            );
        }
    }

    /// Legt den Ordner des aktiven Dateifensters als Lesezeichen an (C5).
    ///
    /// Der Name kommt aus demselben Eingabeblatt, das C4 fuer das Anlegen
    /// benutzt, vorbelegt mit dem Namen des Ordners: das ist in den meisten
    /// Faellen der Name, den der Nutzer ohnehin vergeben haette.
    ///
    /// Liefert `true`, sobald das Blatt steht: der Tastendruck ist dann
    /// verbraucht.
    fn lesezeichen_anlegen(&self) -> bool {
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };
        let seite = self.ivars().modell.borrow().aktiv();
        let ordner = self.dateifenster(seite).quelle().angezeigter_ordner();
        let vorschlag = ordner
            .file_name()
            .map(|teil| teil.to_string_lossy().into_owned())
            .unwrap_or_else(|| ordner.display().to_string());

        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        namenseingabe::frei_zeigen(
            self.mtm(),
            fenster,
            "Wie soll das Lesezeichen heißen?",
            "Anlegen",
            &vorschlag,
            move |name| {
                if let Some(selbst) = schwach.load() {
                    selbst.lesezeichen_anlegen_ausfuehren(seite, &ordner, &name);
                }
            },
        );
        true
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
    /// Befehlsantwort in die Statuszeile, den ersten der fuenf Raenge; ein
    /// eigenes Blatt entsteht nicht. Der dritte Fehler aus C11, die beschaedigte
    /// `settings.toml`, hat sich beim Start gemeldet, denn dort faellt er an.
    ///
    /// Liefert immer `true`: der Befehl war zustaendig, auch wenn er nur etwas
    /// zu melden hatte. Ein `false` gaebe den Tastendruck an AppKit weiter, das
    /// mit ihm nichts anfangen kann.
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

    /// Legt das Lesezeichen an und sichert die Datei (C5).
    fn lesezeichen_anlegen_ausfuehren(&self, seite: Fensterseite, ordner: &Path, name: &str) {
        if let Err(hinweis) = lesezeichen::name_pruefen(name) {
            self.antwort_zeigen(seite, hinweis.grund());
            return;
        }
        self.leiste().quelle().lesezeichen_anlegen(name, ordner);
        self.lesezeichen_sichern(seite);
        self.antwort_zeigen(seite, &format!("Lesezeichen „{}“ angelegt", name.trim()));
    }

    /// Benennt das ausgewaehlte Lesezeichen um (C5).
    ///
    /// Ueber dasselbe Blatt wie das Anlegen, vorbelegt mit dem alten Namen.
    /// Steht die Auswahl nicht auf einem Lesezeichen, geschieht nichts und wird
    /// nichts gemeldet: dieselbe Antwort, die der Wirkungsbereich gibt.
    fn lesezeichen_umbenennen(&self) -> bool {
        let (Some(fenster), Some(alt)) = (
            self.ivars().fenster.get(),
            self.leiste().quelle().gewaehlter_lesezeichenname(),
        ) else {
            return false;
        };
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
        if self.leiste().quelle().lesezeichen_umbenennen(name) {
            self.lesezeichen_sichern(seite);
        }
    }

    /// Loescht das ausgewaehlte Lesezeichen und sichert die Datei (C5).
    fn lesezeichen_loeschen(&self) -> bool {
        if !self.leiste().quelle().lesezeichen_loeschen() {
            return false;
        }
        self.lesezeichen_sichern(self.ivars().modell.borrow().aktiv());
        true
    }

    /// Schiebt das ausgewaehlte Lesezeichen einen Platz weiter (C5).
    fn lesezeichen_verschieben(&self, richtung: Verschiebung) -> bool {
        if !self.leiste().quelle().lesezeichen_verschieben(richtung) {
            return false;
        }
        self.lesezeichen_sichern(self.ivars().modell.borrow().aktiv());
        true
    }

    /// Fuehrt einen Fokusbefehl aus: erst den Bereich hervorholen, dann den
    /// Fokus setzen (C2, C5, C6).
    ///
    /// Der Weg aller drei Fokusbefehle, und sie gehen ihn ohne Sonderfall.
    /// Welchen Bereich einer hervorholt, sagt
    /// [`fokus::holt_hervor`](crate::kommandos::fokus::holt_hervor) und sonst
    /// nichts; dort steht auch, warum das seit dem Nutzerentscheid vom 260807
    /// geschieht, statt einen ausgeblendeten Bereich stumm abzuweisen.
    ///
    /// "Ausgefuehrt" heisst hier: **irgendetwas** ist geschehen. Der Befehl auf
    /// eine ausgeblendete Leiste blendet sie ein, auch wenn der Fokus danach
    /// aus einem anderen Grund nicht umzieht; ohne das oder-Zeichen liesse er
    /// die Aufteilung ungezeichnet stehen.
    fn fokus_holen(&self, ziel: Fokus) -> bool {
        let eingeblendet = match fokus::holt_hervor(ziel) {
            Some(bereich) => self.bereich_einblenden(bereich),
            None => false,
        };
        let gesetzt = self.fokus_setzen(ziel);
        eingeblendet || gesetzt
    }

    /// Setzt den Eingabefokus in die Leiste, in die Vorschau oder in das
    /// aktive Dateifenster (C5, C6).
    ///
    /// Die eine Stelle, die den Fokus **setzt**, so wie
    /// [`Anwendungsdelegierter::fokus`] die eine ist, die ihn liest. In einen
    /// ausgeblendeten Randbereich geht der Fokus nicht: dort saehe der Nutzer
    /// weder seine Auswahl noch, dass seine Tasten irgendwo ankommen. Die
    /// Sperre bleibt stehen, obwohl [`Self::fokus_holen`] den Bereich vorher
    /// hervorholt — sie gilt fuer jeden Aufrufer und nicht nur fuer den einen,
    /// der vorbaut.
    ///
    /// Drei Aufrufer: die drei Fokusbefehle ueber [`Self::fokus_holen`], das
    /// Ausblenden eines Randbereichs, und der Aufbau der Oberflaeche mit
    /// [`crate::kommandos::fokus::BEIM_START`].
    fn fokus_setzen(&self, ziel: Fokus) -> bool {
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };
        match ziel {
            Fokus::Leiste => {
                if !self.ivars().modell.borrow().sichtbar(Bereich::Lesezeichen) {
                    return false;
                }
                fenster.makeFirstResponder(Some(self.leiste().quelle().liste()))
            }
            // In eine ausgeblendete Vorschau geht der Fokus nicht, aus
            // demselben Grund wie bei der Leiste.
            Fokus::Vorschau => {
                if !self.ivars().modell.borrow().sichtbar(Bereich::Vorschau) {
                    return false;
                }
                fenster.makeFirstResponder(Some(self.vorschau().fokusansicht()))
            }
            // Bis der Editor gebaut ist, gibt es keine Textflaeche, auf die
            // der Ersthelfer zu setzen waere; der Befehl scheitert und meldet
            // nichts, wie jeder Fokusbefehl auf einen Bereich, der nicht da
            // ist. **S17 loest diese Zeile ab** und setzt den Ersthelfer auf
            // die Textflaeche des Editors.
            Fokus::Editor => false,
            Fokus::Dateifenster | Fokus::Anderswo => {
                let aktiv = self.ivars().modell.borrow().aktiv();
                fenster.makeFirstResponder(Some(self.dateifenster(aktiv).liste()))
            }
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
    /// Abbruch ueberleben. `terminate:` kehrt nicht zurueck, solange kein
    /// `applicationShouldTerminate:` widerspricht, und ein solches gibt es
    /// nicht; die Aufrufer rechnen trotzdem nicht damit, sondern tun danach
    /// schlicht nichts mehr.
    fn ohne_tastenabgriff_beenden(&self) {
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
    fn abgriff_aufsetzen(&self) -> Option<Tastenabgriff> {
        let belegung = self.ivars().belegung.borrow().clone();
        let fuer_faenger = objc2::rc::Weak::from_retained(&self.retain());
        let fuer_senke = objc2::rc::Weak::from_retained(&self.retain());
        Tastenabgriff::einrichten(
            self.mtm(),
            belegung,
            self.ivars().tasten_protokoll,
            move |druck| match fuer_faenger.load() {
                Some(selbst) => selbst.tastendruck_fangen(druck),
                None => false,
            },
            move |eingabe| match fuer_senke.load() {
                Some(selbst) => selbst.eingabe_ausfuehren(eingabe),
                None => false,
            },
        )
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

    /// Der Faenger des Ereignisabgriffs: nimmt die Belegungsansicht gerade
    /// eine Kombination auf, gehoert ihr dieser Tastendruck (C3).
    fn tastendruck_fangen(&self, druck: Tastendruck) -> bool {
        let quelle = {
            let ansicht = self.ivars().belegungsansicht.borrow();
            match ansicht.as_ref() {
                Some(quelle) if quelle.nimmt_auf() => quelle.clone(),
                _ => return false,
            }
        };
        quelle.tastendruck_aufnehmen(druck);
        true
    }

    // ------------------------------------------------------------------
    // Dateisystem und Datentraeger (C9)
    // ------------------------------------------------------------------

    /// Setzt die Beobachtung der sichtbaren Ordner neu auf (C9).
    ///
    /// Gerufen nach jeder Navigation und nach jedem Ein- oder Ausblenden des
    /// zweiten Dateifensters. Der alte Strom faellt dabei; ein
    /// `FSEventStream` aendert seine Pfadliste nach dem Anlegen nicht mehr,
    /// und einen zweiten Strom danebenzustellen hiesse, denselben Ordner
    /// doppelt zu beobachten.
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
    /// getipptes Zeichen gehoert immer dem aktiven Dateifenster, weil die
    /// Sprungmarke aus C2 die Liste durchsucht, die vor dem Nutzer steht.
    fn eingabe_ausfuehren(&self, eingabe: Eingabe) -> bool {
        if self.ivars().dateifenster.get().is_none() {
            return false;
        }
        match eingabe {
            Eingabe::Kommando(kommando) => self.kommando_ausfuehren(kommando),
            Eingabe::Zeichen(zeichen) => {
                // Ein getipptes Zeichen gehoert dem Blatt, solange eines steht:
                // die Sprungmarke durchsucht eine Liste, die der Nutzer gerade
                // nicht bedient.
                if self.blatt_steht() {
                    return false;
                }
                let aktiv = self.ivars().modell.borrow().aktiv();
                self.dateifenster(aktiv)
                    .quelle()
                    .sprungmarke_tippen(zeichen)
            }
        }
    }

    /// Ob am Hauptfenster gerade ein Blatt steht.
    ///
    /// Die eine Abfrage dafuer. Sie deckt jedes Blatt ab, auch die Pfadeingabe
    /// aus C2 und die kommenden aus S17, und nicht nur die vier aus diesem
    /// Schritt.
    fn blatt_steht(&self) -> bool {
        self.ivars()
            .fenster
            .get()
            .and_then(|fenster| fenster.attachedSheet())
            .is_some()
    }

    /// Fuehrt ein Kommando aus, das der Ereignisabgriff nachgeschlagen hat.
    ///
    /// Liefert, ob es ausgefuehrt wurde; nur dann schluckt der Abgriff das
    /// Ereignis.
    fn kommando_ausfuehren(&self, kommando: Kommando) -> bool {
        // Solange ein Blatt steht, kommt allein der Abbruch durch. Alles
        // uebrige geht unveraendert an AppKit weiter, damit das Blatt seine
        // eigene Tastaturbedienung behaelt.
        //
        // Ein laufender Vorgang sperrt seit S16b **nicht** mehr: C4 sagt zu,
        // dass Navigation, Markierung und Tabwechsel waehrend einer Operation
        // wirken, und der Fortschritt steht in der Statuszeile statt in einem
        // Blatt. Dass ein zweiter Operationsbefehl nichts startet, prueft
        // `auftrag_stellen` und meldet es; eine Tastensperre dafuer waere zu
        // grob.
        if self.blatt_steht() && !operationen::waehrend_blatt_erlaubt(kommando) {
            return false;
        }

        // **Die eine Stelle, die vor dem Ausfuehren nach dem Fokus fragt.**
        // Der Wirkungsbereich sagt, welchen Bereich dieser Befehl braucht, und
        // `fokus` sagt, welcher ihn hat; passt beides nicht zusammen, geschieht
        // nichts und wird nichts gemeldet. Bis Schritt 18 stand diese Frage an
        // den beiden Loeschbefehlen; sie ist hier aufgegangen und steht nicht
        // daneben. Der Wert wird gleich ein zweites Mal gebraucht, dann aber
        // als Adresse und nicht als Vorbehalt; siehe den Modulkopf.
        let fokus = self.fokus();
        if !fokus::wirkt(kommando.wirkungsbereich(), fokus) {
            return false;
        }

        // **Die eine Loeschregel der Befehlsantwort.** Was KRK auf den vorigen
        // Befehl geantwortet hat, gilt bis zum naechsten und keinen Tastendruck
        // laenger; erst danach darf der Befehl seine eigene Antwort setzen. An
        // beiden Dateifenstern, weil es genau einen letzten Befehl gibt und
        // nicht einen je Seite: der Abschlusstext einer Kopie steht im Fenster
        // des Vorgangs, und ein Befehl im anderen Fenster ist trotzdem neuer.
        // Damit haengt der oberste Rang an einem Ereignis und an keiner Uhr.
        for seite in Fensterseite::ALLE {
            self.dateifenster(seite).quelle().befehlsantwort_loeschen();
        }

        let ausgefuehrt = match kommando {
            Kommando::Kopieren => self.uebertragen(kommando),
            Kommando::Verschieben => self.uebertragen(kommando),
            Kommando::InPapierkorb => self.in_den_papierkorb(),
            Kommando::EndgueltigLoeschen => self.endgueltig_loeschen(),
            Kommando::Abbrechen => self.abbrechen(),
            Kommando::OrdnerAnlegen => self.anlegen(Anlegeart::Ordner),
            Kommando::DateiAnlegen => self.anlegen(Anlegeart::Datei),
            Kommando::UmbenennenStapel => self.stapel_umbenennen(),
            Kommando::TerminalOeffnen => self.terminal_oeffnen(),
            Kommando::ZwischenablageAnsehen => self.zwischenablage_ansehen(),
            Kommando::FensterWechseln => self.ivars().modell.borrow_mut().fenster_wechseln(),
            Kommando::LeisteUmschalten => self.bereich_umschalten(Bereich::Lesezeichen),
            Kommando::ZweitesFensterUmschalten => self.bereich_umschalten(Bereich::Rechts),
            Kommando::VorschauUmschalten => self.bereich_umschalten(Bereich::Vorschau),
            Kommando::FensterEinblenden => {
                self.fenster_zeigen();
                true
            }
            Kommando::FensterSchliessen => self.fenster_schliessen(),
            Kommando::Beenden => self.beenden(),
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
            Kommando::FokusVorschau => self.fokus_holen(Fokus::Vorschau),
            Kommando::BelegungAnsehen => self.belegung_ansehen(),
            // Alles uebrige gehoert dem Bereich, der den Fokus hat.
            andere => self.bereichskommando(fokus, andere),
        };
        if ausgefuehrt {
            self.aufteilung_nachziehen();
            self.sitzung_vormerken();
        }
        ausgefuehrt
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
            // (C6); alles andere fuehrt die Vorschau nicht aus, und der
            // Tastendruck laeuft wie ein unbelegter weiter.
            Fokus::Vorschau => self.vorschau().kommando_ausfuehren(kommando),
            // Solange es keinen Editor gibt, liefert `Anwendungsdelegierter::
            // fokus` diesen Wert nie, und der Zweig ist unerreichbar. Er
            // fuehrt den Befehl deshalb nicht aus, statt ihn an das
            // Dateifenster umzuleiten: dorthin gehoert er nicht, und ein
            // Tastendruck, den niemand ausfuehrt, laeuft unveraendert an
            // AppKit weiter. **S17 loest diese Zeile ab** und reicht das
            // Kommando an den Editor.
            Fokus::Editor => false,
            Fokus::Dateifenster | Fokus::Anderswo => {
                let aktiv = self.ivars().modell.borrow().aktiv();
                self.dateifenster(aktiv)
                    .quelle()
                    .kommando_ausfuehren(kommando)
            }
        }
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
        let meldung = match self.ivars().ablage.borrow().as_ref() {
            Some(ablage) => match belegung.sichern(ablage) {
                Ok(()) => None,
                Err(fehler) => Some(format!(
                    "die Belegung gilt, liess sich aber nicht sichern: {fehler}"
                )),
            },
            None => Some(
                "die Belegung gilt, ist aber ohne Ablageordner nicht gesichert und geht mit dem Beenden verloren"
                    .to_owned(),
            ),
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

    /// Blendet einen Bereich aus oder wieder ein (C7).
    fn bereich_umschalten(&self, bereich: Bereich) -> bool {
        let umgeschaltet = self.ivars().modell.borrow_mut().umschalten(bereich);
        if umgeschaltet {
            self.nach_dem_sichtbarkeitswechsel(bereich);
        }
        umgeschaltet
    }

    /// Holt einen ausgeblendeten Bereich hervor und blendet nie einen aus.
    ///
    /// Der Weg der Befehle, die einen Bereich **brauchen** statt ihn
    /// umzuschalten: `shift+f3` aus C10 und die Fokusbefehle seit dem
    /// Nutzerentscheid vom 260807. Die Regel selbst steht in
    /// [`Fenstermodell::einblenden`] und damit ausserhalb von AppKit; hier
    /// kommen allein die Nachzuege dazu, die jeder Sichtbarkeitswechsel
    /// braucht.
    fn bereich_einblenden(&self, bereich: Bereich) -> bool {
        let eingeblendet = self.ivars().modell.borrow_mut().einblenden(bereich);
        if eingeblendet {
            self.nach_dem_sichtbarkeitswechsel(bereich);
        }
        eingeblendet
    }

    /// Was nach jedem Wechsel der Sichtbarkeit nachzuziehen ist.
    ///
    /// Die eine Stelle dafuer, gerufen von [`Self::bereich_umschalten`] wie von
    /// [`Self::bereich_einblenden`] und nur, wenn sich etwas geaendert hat. Die
    /// drei Nachzuege sind nach dem Bereich unterschieden und nicht danach,
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
        if matches!(bereich, Bereich::Lesezeichen | Bereich::Vorschau)
            && !self.ivars().modell.borrow().sichtbar(bereich)
        {
            self.fokus_setzen(Fokus::Dateifenster);
        }
        // Die eingeblendete Vorschau holt nach, was sie im ausgeblendeten
        // Zustand ausgesetzt hat; die Begruendung steht an
        // [`AnwendungsIvars::vorschau_nachtrag`].
        if bereich == Bereich::Vorschau && self.ivars().modell.borrow().sichtbar(bereich) {
            self.vorschau_nachtragen();
        }
    }

    /// Aendert die Breite des aktiven Dateifensters um einen Schritt (C7).
    ///
    /// Der "aktive Bereich" der beiden Kuerzel ist das aktive Dateifenster.
    /// Die Lesezeichenleiste und die Vorschau bekommen ihre Breite mit der
    /// Maus; ihnen ein eigenes Kuerzelpaar zu geben, hiesse vier Befehle fuer
    /// eine Sache, und C7 verlangt sie nicht.
    fn breite_aendern(&self, betrag: f64) -> bool {
        // Zuerst nachlesen, was wirklich auf dem Schirm steht: der Nutzer kann
        // die Trennlinie zwischendurch mit der Maus verschoben haben, und ein
        // Schritt auf eine ueberholte Zahl spraenge zurueck.
        if let Some(aufteilung) = self.ivars().aufteilung.get() {
            self.ivars()
                .modell
                .borrow_mut()
                .breiten_uebernehmen(aufteilung.gemessene_breiten());
        }
        let mut modell = self.ivars().modell.borrow_mut();
        let bereich = Bereich::von_seite(modell.aktiv());
        modell.breite_aendern(bereich, betrag);
        true
    }

    /// Macht das genannte Dateifenster zum aktiven.
    fn aktives_setzen(&self, seite: Fensterseite) {
        if self.ivars().modell.borrow_mut().aktiv_setzen(seite) {
            self.aufteilung_nachziehen();
            self.sitzung_vormerken();
        }
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
    /// Ereignisabgriff mit [`Kommando::FensterSchliessen`] und der
    /// Menueeintrag ueber den Selektor `fensterSchliessen:`. `performClose:`
    /// und nicht `close`, damit der Fensterdelegierte gefragt wird und die
    /// Schliessanimation dieselbe bleibt wie beim Klick auf den roten Knopf.
    /// Das Fenster ueberlebt sein Schliessen; "Fenster einblenden" holt es
    /// zurueck.
    fn fenster_schliessen(&self) -> bool {
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };
        fenster.performClose(None);
        true
    }

    /// Beendet die Anwendung (C3).
    ///
    /// Die eine Stelle dafuer, und beide Wege gehen darueber: der
    /// Ereignisabgriff mit [`Kommando::Beenden`] und der Menueeintrag ueber den
    /// Selektor `beenden:`. `terminate:` und nicht `exit`, damit AppKit seinen
    /// Ablauf geht und `applicationWillTerminate:` den letzten Sitzungsstand
    /// noch schreibt.
    fn beenden(&self) -> bool {
        // `None` als Absender heisst: kein Steuerelement hat den Aufruf
        // ausgeloest.
        NSApplication::sharedApplication(self.mtm()).terminate(None);
        true
    }

    /// Schreibt Sichtbarkeit, Breiten und die Markierung des aktiven
    /// Dateifensters in die Ansicht.
    fn aufteilung_nachziehen(&self) {
        let Some(aufteilung) = self.ivars().aufteilung.get() else {
            return;
        };
        let (breiten, sichtbar, aktiv) = {
            let modell = self.ivars().modell.borrow();
            (modell.breiten(), modell.sichtbarkeit(), modell.aktiv())
        };
        aufteilung.anwenden(&breiten, &sichtbar);
        aufteilung.aktives_markieren(aktiv);
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
    /// Sofort und ohne Rueckfrage: der Rueckweg ist der Papierkorb des Systems,
    /// und einen eigenen Rueckgaengig-Speicher fuehrt KRK nicht
    /// (`shared/decisions/260802-0842_*_loeschen-papierkorb-oder-endgueltig.md`).
    ///
    /// **Der Fokusvorbehalt steht seit Schritt 18 nicht mehr hier.** Er stand
    /// als eigene Abfrage an dieser Stelle und an der von
    /// [`Self::endgueltig_loeschen`]; heute tragen beide Befehle
    /// `Wirkungsbereich::Dateifenster`, und die Zuleitung weist sie ab, bevor
    /// sie hier ankommen.
    fn in_den_papierkorb(&self) -> bool {
        self.auftrag_stellen(Art::InDenPapierkorb)
    }

    /// Die Auswahl endgueltig loeschen, nach genau einer Rueckfrage (C4, F8).
    fn endgueltig_loeschen(&self) -> bool {
        let aktiv = self.ivars().modell.borrow().aktiv();
        let auswahl = self.dateifenster(aktiv).quelle().betroffene_eintraege();
        if auswahl.ist_leer() {
            self.antwort_zeigen(aktiv, "es ist nichts ausgewählt");
            return true;
        }
        let Some(fenster) = self.ivars().fenster.get() else {
            return false;
        };

        let (frage, erlaeuterung) = operationen::loeschfrage(&auswahl);
        let schwach = objc2::rc::Weak::from_retained(&self.retain());
        let griff = loeschbestaetigung::zeigen(
            self.mtm(),
            fenster,
            &frage,
            &erlaeuterung,
            move |bestaetigt| {
                let Some(selbst) = schwach.load() else {
                    return;
                };
                *selbst.ivars().offenes_blatt.borrow_mut() = None;
                if bestaetigt {
                    selbst.auftrag_stellen(Art::EndgueltigLoeschen);
                }
            },
        );
        *self.ivars().offenes_blatt.borrow_mut() = Some(griff);
        true
    }

    /// Der Abbruchbefehl (C4).
    ///
    /// Er bedient zwei Faelle, und die Reihenfolge ist bindend: ein offenes
    /// Blatt zuerst, weil die Konfliktfrage waehrend eines laufenden Vorgangs
    /// steht und der Abbruch dann ihr gilt.
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
        let (art, seite) = {
            let vorgang = self.ivars().vorgang.borrow();
            let Some(vorgang) = vorgang.as_ref() else {
                return false;
            };
            vorgang.zustand.abbrechen();
            (vorgang.art.clone(), vorgang.seite)
        };
        self.fortschritt_zeigen(seite, &operationen::abbruchzeile(&art));
        true
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
    /// Zeile anhand ihres Namens waehlt. Der Lesevorgang laeuft zu diesem
    /// Zeitpunkt noch, also merkt sie den Namen vor und springt, sobald er
    /// eintrifft.
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
        self.dateifenster(seite).quelle().eintrag_waehlen(&name);
        self.antwort_zeigen(seite, &operationen::angelegt_text(art, &name));
    }

    /// Benennt den Eintrag um, den der Nutzer in der Liste bearbeitet hat (C4).
    ///
    /// Dieselbe Reihenfolge wie beim Anlegen, und aus denselben Gruenden: erst
    /// [`krk_core::operation::umbenennen`] aus S15, dann
    /// [`auffrischung::ordner_neu_lesen`], der eine Auffrischungspfad aus S14,
    /// dann die Auswahl auf den neuen Namen ueber die eine Stelle, die eine
    /// Zeile anhand ihres Namens waehlt.
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
        self.dateifenster(seite).quelle().eintrag_waehlen(neu);
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
    /// Drei Faelle. Steht ein Blatt am Fenster, ist dessen Panel das
    /// Schluesselfenster und nicht das Hauptfenster: [`Fokus::Anderswo`], und
    /// ohne diese Antwort loeschte ein Delete vor der stehenden Rueckfrage in
    /// dem Ordner dahinter. Ist der Ersthelfer die Liste der Leiste,
    /// [`Fokus::Leiste`]. Sonst eine der beiden Dateilisten. Die Schreibmarke
    /// in einem Textfeld kommt hier nicht vor: der Ereignisabgriff reicht den
    /// Tastendruck dann weiter und erzeugt gar kein Kommando.
    fn fokus(&self) -> Fokus {
        let (Some(schluessel), Some(haupt)) = (
            NSApplication::sharedApplication(self.mtm()).keyWindow(),
            self.ivars().fenster.get(),
        ) else {
            return Fokus::Anderswo;
        };
        if !schluessel.isEqual(Some(haupt)) {
            return Fokus::Anderswo;
        }
        let in_der_leiste = self.ivars().leiste.get().is_some_and(|leiste| {
            haupt
                .firstResponder()
                .is_some_and(|ersthelfer| ersthelfer.isEqual(Some(leiste.quelle().liste())))
        });
        if in_der_leiste {
            return Fokus::Leiste;
        }
        let in_der_vorschau = self.ivars().vorschau.get().is_some_and(|vorschau| {
            haupt
                .firstResponder()
                .is_some_and(|ersthelfer| ersthelfer.isEqual(Some(vorschau.fokusansicht())))
        });
        if in_der_vorschau {
            Fokus::Vorschau
        } else {
            Fokus::Dateifenster
        }
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

    /// Meldet einen bereits laufenden Vorgang und sagt, ob deshalb nichts
    /// startet (C4).
    ///
    /// KRK haelt genau einen Vorgang. Die Meldung geht als **Befehlsantwort** an
    /// das Dateifenster, in dem der Nutzer die Taste gedrueckt hat, und steht
    /// damit auch dann in der Zeile, wenn genau dieses Fenster den laufenden
    /// Vorgang begonnen hat. Bis zum 260804-1915 war sie eine Fenstermeldung und
    /// verschwand im haeufigen Fall hinter dem eigenen Fortschritt,
    /// `issues/260804-1915_*_der-zweite-operationsbefehl-meldet-sich-im-fenster-des-vorgangs-unsichtbar.md`.
    ///
    /// **Beide Wege in die Operationsmaschine fragen hier.** Die vier Befehle
    /// aus der Auswahl gehen ueber [`Self::auftrag_stellen`], das
    /// Stapel-Umbenennen ueber [`Self::stapel_beauftragen`]; eine zweite Prueferei
    /// waeren zwei Antworten auf dieselbe Frage.
    fn vorgang_laeuft_schon(&self, seite: Fensterseite) -> bool {
        let laufende_art = self
            .ivars()
            .vorgang
            .borrow()
            .as_ref()
            .map(|vorgang| vorgang.art.clone());
        let Some(laufende_art) = laufende_art else {
            return false;
        };
        self.antwort_zeigen(seite, &operationen::schon_ein_vorgang(&laufende_art));
        true
    }

    /// Startet einen fertigen Auftrag auf der Operationsmaschine.
    ///
    /// Der gemeinsame Teil der beiden Wege: Arbeitsfaden ueber
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
                    self.dateifenster(vorgang.seite)
                        .quelle()
                        .eintrag_waehlen(erster);
                }
            }
            Art::Kopieren { .. }
            | Art::Verschieben { .. }
            | Art::InDenPapierkorb
            | Art::EndgueltigLoeschen => {}
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
    /// Der oberste der vier Raenge, siehe
    /// [`crate::appkit::statuszeile::zeile`]. Nicht zu verwechseln mit
    /// [`Dateifenstersicht::melden`] weiter unten: das ist der Weg der
    /// Ereignisse, die niemand angefordert hat, und der steht einen Rang
    /// tiefer.
    fn antwort_zeigen(&self, seite: Fensterseite, text: &str) {
        self.dateifenster(seite)
            .quelle()
            .befehlsantwort_zeigen(text);
    }

    // ------------------------------------------------------------------
    // Sitzung
    // ------------------------------------------------------------------

    /// Der Sitzungszustand, wie er auf die Platte gehoert.
    fn sitzung_bauen(&self) -> Sitzung {
        if let Some(aufteilung) = self.ivars().aufteilung.get() {
            self.ivars()
                .modell
                .borrow_mut()
                .breiten_uebernehmen(aufteilung.gemessene_breiten());
        }
        let fenster = [
            self.dateifenster(Fensterseite::Links).quelle().zustand(),
            self.dateifenster(Fensterseite::Rechts).quelle().zustand(),
        ];
        self.ivars().modell.borrow().sitzung(fenster)
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
            schreiber.vormerken(sitzung, Instant::now())
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
