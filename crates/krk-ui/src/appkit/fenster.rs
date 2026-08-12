//! Das Fenster und sein Delegierter.
//!
//! Die Inhaltsansicht des Fensters ist seit dem 260812 eine Traegerflaeche aus
//! [`fensterinhalt`], und darin liegen zwei Ansichten uebereinander: die
//! Aufteilung aus [`super::aufteilung`] und darunter die Bereichsleiste aus
//! [`super::bereichsleiste`]. Bis dahin war die Aufteilung selbst die
//! Inhaltsansicht. Sie traegt seit Schritt 16 der Editor-Runde **fuenf**
//! Bereiche und nicht mehr die vier der Runde 1; zugleich zu sehen sind
//! hoechstens vier, weil C1 jener Runde zusagt, dass die Vorschau und der
//! Editor sich dieselbe Flaeche teilen.
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
//! `setContentMinSize:`, `setContentView:`, `setDelegate:` und `center`.
//! [`fensterinhalt`] kommt mit `initWithFrame:`, `setFrame:`, `addSubview:`
//! und `setAutoresizingMask:` dazu; auch sie stehen seit 10.0, und keine der
//! beiden gesetzten Masken (`NSViewWidthSizable`, `NSViewHeightSizable`,
//! `NSViewMaxYMargin`) traegt eine eigene Angabe. Das
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
    NSAutoresizingMaskOptions, NSBackingStoreType, NSResponder, NSView, NSWindow, NSWindowDelegate,
    NSWindowStyleMask,
};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
    ns_string,
};

use super::bereichsleiste;
use super::tabelle::DateifensterQuelle;

/// Die Groesse, mit der das Fenster beim ersten Start aufgeht.
///
/// Breiter als in Schritt 6: vier Bereiche nebeneinander brauchen mehr Platz
/// als eine Dateiliste.
const ANFANGSGROESSE: NSSize = NSSize::new(1280.0, 720.0);

/// Die Groesse, unter die sich das Fenster nicht ziehen laesst.
///
/// **Die Zahl stammt aus der Runde 1 und deckt deren vier Bereiche.** Ihre
/// Mindestbreiten aus [`crate::fenstermodell::Bereich`] summieren sich auf 760
/// Punkte (Lesezeichen 120, beide Dateifenster je 240, Vorschau 160); die
/// uebrigen 20 sind Luft fuer die Trennlinien. Darunter faende keiner von ihnen
/// mehr Platz, und die Zusage aus C7, dass jeder bedienbar bleibt, waere nicht
/// zu halten.
///
/// **Der Editor der Runde 2 geht in diese Rechnung nicht ein**, und die Zahl
/// ist seither nicht nachgezogen worden. Er steht nie zugleich mit der
/// Vorschau (C1 der Editor-Runde), tritt aber an ihre Stelle mit 320 statt 160
/// Punkten Mindestbreite; die Summe seines Vierersatzes ist 920. Zwischen 780
/// und 920 Punkten Fensterbreite passen diese vier Mindestbreiten nicht mehr
/// nebeneinander, und [`crate::fenstermodell::bereichsbreiten`] schickt dann
/// **alle vier** mit demselben Faktor unter ihr Mindestmass. Bis zur
/// Bereichsleisten-Runde traf es allein den Editor, weil die festen Bereiche
/// der Reihe nach bedient wurden und er hinten stand. Ob die Untergrenze
/// deshalb auf 940 steigen soll oder die Bereiche in diesem Band gedrueckt
/// bleiben duerfen, ist eine Frage an den Nutzer und keine, die hier still
/// beantwortet wird. Am 260812-0430 hat er die Breite bei 780 belassen.
///
/// **Die Hoehe ist am 260812 von 300 auf 318 gestiegen**, und zwar um genau
/// [`bereichsleiste::HOEHE`]: die Leiste am Fensterfuss nimmt der Fensterzeile
/// diese Punkte ab, und ohne den Nachtrag haetten die fuenf Bereiche darueber
/// weniger Hoehe zur Verfuegung als vor der Runde. Kriterium C1.3 verlangt,
/// dass sie ihre bisherige Mindesthoehe behalten; die Zahl ist deshalb die
/// Summe und keine neu gewaehlte.
const MINDESTGROESSE: NSSize = NSSize::new(780.0, 300.0 + bereichsleiste::HOEHE);

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

/// Legt die Fensterzeile und die Bereichsleiste uebereinander.
///
/// Die Inhaltsansicht des Fensters war bis zum 260812 die Fensterzeile selbst;
/// seit der Bereichsleisten-Runde steht unter ihr eine Leiste ueber die volle
/// Breite, und beide brauchen deshalb eine Traegerflaeche. Dasselbe Muster wie
/// `aufteilung::dateifensterinhalt`, das Tableiste, Liste und Statuszeile eines
/// Dateifensters uebereinanderlegt.
///
/// **Die beiden Autogroessen tragen die Aufteilung ueber jede
/// Groessenaenderung.** Die Leiste haengt unten und behaelt ihre Hoehe
/// ([`NSAutoresizingMaskOptions::ViewMaxYMargin`] laesst allein den Abstand
/// nach oben wachsen), die Fensterzeile nimmt, was darueber frei wird. Ohne
/// die Hoehenaenderung an [`MINDESTGROESSE`] verloeren die fuenf Bereiche
/// dabei die 18 Punkte, die die Leiste bekommt.
///
/// **Die Leiste ist keine Unteransicht der Fensterzeile, sondern ihre
/// Schwester**, und das ist keine Geschmacksfrage: `ersthelferbereich` sucht
/// den Ersthelfer in den fuenf Bereichen der `NSSplitView`, und eine Leiste
/// darin waere entweder ein sechster Bereich oder ein blinder Fleck.
pub fn fensterinhalt(
    mtm: MainThreadMarker,
    fensterzeile: &NSView,
    leiste: &NSView,
) -> Retained<NSView> {
    let inhalt = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::ZERO, ANFANGSGROESSE),
    );

    leiste.setFrame(NSRect::new(
        NSPoint::ZERO,
        NSSize::new(ANFANGSGROESSE.width, bereichsleiste::HOEHE),
    ));
    leiste.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewMaxYMargin,
    );
    inhalt.addSubview(leiste);

    fensterzeile.setFrame(NSRect::new(
        NSPoint::new(0.0, bereichsleiste::HOEHE),
        NSSize::new(
            ANFANGSGROESSE.width,
            ANFANGSGROESSE.height - bereichsleiste::HOEHE,
        ),
    ));
    fensterzeile.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
    );
    inhalt.addSubview(fensterzeile);

    inhalt
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
