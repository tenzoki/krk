//! Das Fenster und sein Delegierter.
//!
//! Die Inhaltsansicht des Fensters ist seit Schritt 12 die Aufteilung aus
//! [`super::aufteilung`] mit ihren vier Bereichen und nicht mehr die eine
//! Tabelle aus Schritt 6.
//!
//! Der Delegierte hat eine Aufgabe, und sie ist nicht kosmetisch: er bricht die
//! laufenden Lesevorgaenge **beider** Dateifenster ab, sobald das Fenster
//! schliesst. Ohne ihn liesse ein Ordner mit 100.000 Eintraegen seinen
//! Arbeitsfaden und seinen Zeitgeber gegen eine Tabelle weiterlaufen, die
//! niemand mehr sieht.
//!
//! **Das Fenster ueberlebt sein Schliessen.** `setReleasedWhenClosed(false)`
//! sorgt dafuer, und der Anwendungsdelegierte haelt es weiter. Genau darauf
//! baut der Rueckweg aus C7: "Fenster einblenden" auf Cmd+N und der Klick auf
//! das Dock-Symbol holen dieses eine Fenster nach vorn, statt ein zweites
//! anzulegen.
//!
//! # Der eine Ausloesepunkt der Fokusanzeige (C9)
//!
//! Seit S45 ist das Fenster eine eigene Klasse, [`Hauptfenster`], und sie hat
//! genau eine Aufgabe: **melden, dass sich der Ersthelfer oder der
//! Vordergrund geaendert hat.** C9 verlangt, dass die Anzeige dem Fokus folgt,
//! gleich auf welchem Weg er dorthin kam, und der Mausklick ist einer davon.
//!
//! ```text
//!  KRK setzt den Fokus ─┐
//!                       ├─> makeFirstResponder: ──> melden ──> Anwendungsdelegierter
//!  Mausklick in eine ───┘                                       fokusanzeige_nachziehen
//!  Flaeche, die den Rang annimmt
//!
//!  becomeKeyWindow / resignKeyWindow ────────────> melden
//! ```
//!
//! **Es gibt keine zweite Tuer.** `makeFirstResponder:` ist der einzige Weg,
//! auf dem der Ersthelferrang wechselt: KRK ruft sie in
//! `Anwendungsdelegierter::fokus_setzen`, und AppKit ruft dieselbe Methode beim
//! Mausklick in eine Flaeche, die den Rang annimmt. Drei naheliegende
//! Alternativen scheiden aus: `NSWindow` verschickt keine Benachrichtigung
//! ueber den Ersthelfer, die Schluesselwertbeobachtung der Eigenschaft
//! `firstResponder` ist von Apple nicht zugesagt, und ein Takt, der die Frage
//! sechzigmal je Sekunde stellt, kostete Strom fuer eine Frage, die sich fast
//! nie aendert.
//!
//! **Die Ueberschreibung ruft zuerst die Oberklasse und meldet nur bei
//! Erfolg.** Ein abgelehnter Wechsel — eine Flaeche, die den Rang nicht annimmt
//! — aendert nichts, und eine Meldung darueber liesse die Anzeige ohne Anlass
//! neu schreiben.
//!
//! **Der Griff auf den Anwendungsdelegierten ist schwach**, aus demselben Grund
//! wie bei jedem Rueckruf dieses Projekts: der Ring Delegierter → Fenster →
//! Rueckruf → Delegierter schloesse sich sonst, und das Fenster lebt ueber sein
//! Schliessen hinaus.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSWindow`, `NSResponder`, `NSView` und `NSObject` stehen seit macOS 10.0
//! zur Verfuegung, ebenso das Protokoll `NSWindowDelegate` mit
//! `windowWillClose:` und jede hier gerufene oder ueberschriebene Methode:
//! `initWithContentRect:styleMask:backing:defer:`, `makeFirstResponder:`,
//! `becomeKeyWindow`, `resignKeyWindow`, `setReleasedWhenClosed:`, `setTitle:`,
//! `setContentMinSize:`, `setContentView:`, `setDelegate:` und `center`. Das
//! Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.
//!
//! Dass die drei Ueberschreibungen selbst seit 10.0 stehen, ist hier die
//! tragende Angabe und nicht eine unter mehreren: eine Methode, die es auf dem
//! Zielsystem nicht gaebe, ueberschriebe nichts, und der Ausloesepunkt der
//! Fokusanzeige bliebe stumm, statt abzustuerzen.

use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSBackingStoreType, NSResponder, NSView, NSWindow, NSWindowDelegate, NSWindowStyleMask,
};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
    ns_string,
};

use super::tabelle::DateifensterQuelle;

/// Die Groesse, mit der das Fenster beim ersten Start aufgeht.
///
/// Breiter als in Schritt 6: vier Bereiche nebeneinander brauchen mehr Platz
/// als eine Dateiliste.
const ANFANGSGROESSE: NSSize = NSSize::new(1280.0, 720.0);

/// Die Groesse, unter die sich das Fenster nicht ziehen laesst.
///
/// Die Summe der vier Mindestbreiten aus [`crate::fenstermodell::Bereich`] plus
/// Luft fuer die Trennlinien. Darunter faenden die Bereiche keinen Platz mehr,
/// und die Zusage aus C7, dass jeder von ihnen bedienbar bleibt, waere nicht zu
/// halten.
const MINDESTGROESSE: NSSize = NSSize::new(780.0, 300.0);

/// Was das Hauptfenster haelt.
pub struct HauptfensterIvars {
    /// Der Melder, den [`Hauptfenster::melder_setzen`] eintraegt.
    ///
    /// Er haelt den Anwendungsdelegierten **schwach**; die Begruendung steht im
    /// Modulkopf. `None` heisst: der Aufbau ist noch nicht so weit, und dann
    /// gibt es auch nichts nachzuziehen.
    melden: RefCell<Option<Box<dyn Fn()>>>,
}

define_class!(
    /// Das Hauptfenster: der eine Ausloesepunkt fuer jeden Wechsel des
    /// Ersthelfers und des Vordergrunds (C9).
    // SAFETY:
    // - Die Oberklasse NSWindow stellt an eine Unterklasse keine Bedingungen,
    //   die hier verletzt wuerden: die drei ueberschriebenen Methoden rufen
    //   jede zuerst die Fassung der Oberklasse und geben deren Ergebnis
    //   unveraendert zurueck.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSWindow)]
    #[thread_kind = MainThreadOnly]
    #[ivars = HauptfensterIvars]
    pub struct Hauptfenster;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Hauptfenster {}

    impl Hauptfenster {
        /// Der Ersthelferrang wechselt: erst die Oberklasse, dann melden.
        // SAFETY: Die Signatur entspricht der von NSWindow.
        #[unsafe(method(makeFirstResponder:))]
        fn ersthelfer_setzen(&self, ansicht: Option<&NSResponder>) -> bool {
            // SAFETY: `makeFirstResponder:` von NSWindow hat die hier
            // angenommene Signatur.
            let erfolg: bool = unsafe { msg_send![super(self), makeFirstResponder: ansicht] };
            if erfolg {
                self.melden();
            }
            // Unveraendert zurueck: der Aufrufer entscheidet daran, ob der
            // Wechsel stattgefunden hat.
            erfolg
        }

        /// Das Fenster kommt in den Vordergrund (C9, achtes Kriterium).
        // SAFETY: Die Signatur entspricht der von NSWindow.
        #[unsafe(method(becomeKeyWindow))]
        fn wird_schluesselfenster(&self) {
            // SAFETY: `becomeKeyWindow` von NSWindow hat die hier angenommene
            // Signatur.
            let () = unsafe { msg_send![super(self), becomeKeyWindow] };
            self.melden();
        }

        /// Das Fenster geht in den Hintergrund (C9, achtes Kriterium).
        // SAFETY: Die Signatur entspricht der von NSWindow.
        #[unsafe(method(resignKeyWindow))]
        fn gibt_schluesselrang_ab(&self) {
            // SAFETY: `resignKeyWindow` von NSWindow hat die hier angenommene
            // Signatur.
            let () = unsafe { msg_send![super(self), resignKeyWindow] };
            self.melden();
        }
    }
);

impl Hauptfenster {
    /// Traegt den Melder ein, der jeden Wechsel weitergibt.
    ///
    /// Gerufen vom Aufbau der Oberflaeche, mit einem Rueckruf, der den
    /// Anwendungsdelegierten schwach haelt. Derselbe Zuschnitt wie
    /// `DateifensterQuelle::ordnerwechsel_setzen` und die drei anderen Melder
    /// dieses Projekts.
    pub fn melder_setzen(&self, melden: Box<dyn Fn()>) {
        *self.ivars().melden.borrow_mut() = Some(melden);
    }

    /// Gibt den Wechsel weiter, falls jemand zuhoert.
    ///
    /// Die Ausleihe steht waehrend des Rufs, wie bei
    /// `DateifensterQuelle::ordnerwechsel_melden`. Sie ist lesend, und der
    /// einzige schreibende Zugriff auf dieselbe Zelle ist
    /// [`Self::melder_setzen`] beim Aufbau; ein Ruf, der ueber AppKit hierher
    /// zuruecklaeuft, nimmt eine zweite Leseausleihe und keine schreibende.
    fn melden(&self) {
        let melden = self.ivars().melden.borrow();
        if let Some(melden) = melden.as_ref() {
            melden();
        }
    }
}

/// Was der Fensterdelegierte haelt.
pub struct FensterIvars {
    /// Die Datenquellen der beiden Dateifenster, links zuerst.
    quellen: [Retained<DateifensterQuelle>; 2],
}

define_class!(
    /// Der Delegierte des Hauptfensters.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = FensterIvars]
    pub struct FensterDelegierter;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for FensterDelegierter {}

    // SAFETY: `NSWindowDelegate` stellt keine Bedingungen.
    unsafe impl NSWindowDelegate for FensterDelegierter {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(windowWillClose:))]
        fn fenster_schliesst(&self, _meldung: &NSNotification) {
            for quelle in &self.ivars().quellen {
                quelle.lesen_abbrechen();
            }
        }
    }
);

impl FensterDelegierter {
    /// Einen Delegierten fuer das Fenster mit den genannten Dateifenstern.
    pub fn neu(
        mtm: MainThreadMarker,
        quellen: [Retained<DateifensterQuelle>; 2],
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(FensterIvars { quellen });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }
}

/// Baut das Hauptfenster um die genannte Ansicht.
///
/// **Liefert seit S45 die Unterklasse und nicht `NSWindow`.** Der einzige
/// Aufrufer ist `Anwendungsdelegierter::oberflaeche_aufbauen`; er traegt den
/// Melder ein und legt das Fenster danach als `Retained<NSWindow>` in seine
/// Ivars. Damit bleibt jede der uebrigen Fensterberuehrungen unveraendert, weil
/// sie ohnehin nur `NSWindow`-Methoden ruft.
pub fn hauptfenster(
    mtm: MainThreadMarker,
    inhalt: &NSView,
    delegierter: &FensterDelegierter,
) -> Retained<Hauptfenster> {
    let this = Hauptfenster::alloc(mtm).set_ivars(HauptfensterIvars {
        melden: RefCell::new(None),
    });
    // SAFETY: `initWithContentRect:styleMask:backing:defer:` von NSWindow hat
    // die hier angenommene Signatur. Das Fenster gibt sich beim Schliessen
    // nicht selbst frei, siehe `setReleasedWhenClosed` darunter: ohne diese
    // Abschaltung waere die Referenz, die der Anwendungsdelegierte haelt, nach
    // dem Schliessen tot, und der Rueckweg aus C7 zeigte auf ein freigegebenes
    // Objekt.
    let fenster: Retained<Hauptfenster> = unsafe {
        let fenster: Retained<Hauptfenster> = msg_send![
            super(this),
            initWithContentRect: NSRect::new(NSPoint::new(0.0, 0.0), ANFANGSGROESSE),
            styleMask: NSWindowStyleMask::Titled
                | NSWindowStyleMask::Closable
                | NSWindowStyleMask::Miniaturizable
                | NSWindowStyleMask::Resizable,
            backing: NSBackingStoreType::Buffered,
            defer: false,
        ];
        fenster.setReleasedWhenClosed(false);
        fenster
    };

    fenster.setTitle(ns_string!("KRK"));
    fenster.setContentMinSize(MINDESTGROESSE);
    fenster.setContentView(Some(inhalt));
    fenster.setDelegate(Some(ProtocolObject::from_ref(delegierter)));
    fenster.center();
    fenster
}
