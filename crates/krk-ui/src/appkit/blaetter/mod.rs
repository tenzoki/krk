//! Die gemeinsame Huelle fuer die Blaetter am Fenster.
//!
//! Ein Blatt ist ein Dialog, der am oberen Rand des Fensters herunterfaehrt und
//! es blockiert, solange er steht. AppKit nennt das ein Sheet. KRK braucht in
//! dieser Runde eines, die Pfadeingabe aus C2; fuenf weitere kommen mit den
//! Schritten 16 und 17 in dieses Verzeichnis, fuer Fortschritt, Abbruch,
//! Konflikt, Rueckfrage und das Umbenennen im Stapel.
//!
//! ```text
//! Blatt::neu ──> textfeld_setzen ──> zeigen(fenster, fertig)
//!                                         │
//!                        fertig(true|false) auf dem Hauptfaden
//! ```
//!
//! **Die Antwort kommt als gewoehnlicher Rust-Wert zurueck.** Der Aufrufer sieht
//! einen `bool` und nicht eine `NSModalResponse`; was AppKit dafuer als Zahl
//! fuehrt, bleibt in dieser Datei.
//!
//! **Der Grund fuer eine gemeinsame Huelle** ist derselbe wie ueberall in
//! diesem Entwurf: fuenf Blaetter mit je eigenem Aufbau waeren fuenf Stellen,
//! die dieselbe Frage beantworten, und die erste Abweichung zwischen ihnen
//! faende keine Pruefung.
//!
//! # Ein Blatt ist mit der Tastatur bedienbar, und das kostet zwei Vorkehrungen
//!
//! Die erste ist der **Fokusvorbehalt** im Ereignisabgriff. Solange das Blatt
//! steht, ist sein Textfeld der Ersthelfer des Schluesselfensters, und
//! [`super::ereignisse`] reicht jeden Tastendruck unveraendert an AppKit
//! weiter. Erst dadurch bewegen Cmd+Links und Cmd+Rechts im Feld die
//! Schreibmarke, statt hinter dem Blatt den Ordner zu wechseln. Der Vorbehalt
//! sitzt im Abgriff und nicht hier, damit jedes weitere Blatt ihn erbt.
//!
//! Die zweite ist der [`Eingabewaechter`]. Ein Textfeld im Bearbeitungszustand
//! verbraucht die Eingabe- und die Escape-Taste selbst: sein Feldeditor macht
//! daraus `insertNewline:` beziehungsweise `cancelOperation:` und beendet damit
//! nur die Bearbeitung. Die Schaltflaechen des Blattes sehen die beiden Tasten
//! dann nie. Der Waechter faengt genau diese zwei Befehle ab und beendet das
//! Blatt. **Am laufenden Buendel gemessen am 260804:** ohne ihn laesst sich das
//! Blatt weder mit der Eingabe- noch mit der Escape-Taste schliessen, und die
//! Pfadeingabe waere allein mit der Maus bedienbar.

pub mod pfadeingabe;

use std::cell::RefCell;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::{ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAlert, NSAlertFirstButtonReturn, NSAlertSecondButtonReturn, NSControl,
    NSControlTextEditingDelegate, NSModalResponse, NSTextField, NSTextFieldDelegate, NSTextView,
    NSView, NSWindow,
};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol, NSString};

/// Was der Waechter tut, wenn der Nutzer im Feld bestaetigt oder abbricht.
///
/// `true` heisst bestaetigt.
type Antwortweg = Box<dyn Fn(bool)>;

/// Was der Eingabewaechter haelt.
pub struct WaechterIvars {
    /// Was zu tun ist, wenn der Nutzer im Feld bestaetigt oder abbricht.
    ///
    /// Wahlfrei, weil der Waechter vor dem Blatt zur Welt kommt: das Fenster,
    /// an dem das Blatt haengt, kennt erst [`Blatt::zeigen`].
    antwort: RefCell<Option<Antwortweg>>,
}

define_class!(
    /// Der Delegierte des Eingabefeldes eines Blattes.
    ///
    /// Er macht die Eingabe- und die Escape-Taste im Textfeld wirksam; siehe
    /// den Modulkopf.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = WaechterIvars]
    pub struct Eingabewaechter;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Eingabewaechter {}

    // SAFETY: `NSControlTextEditingDelegate` hat nur wahlfreie Methoden.
    unsafe impl NSControlTextEditingDelegate for Eingabewaechter {
        /// Der Feldeditor fragt, ob jemand anders diesen Befehl uebernimmt.
        ///
        /// Wir uebernehmen genau zwei: `insertNewline:` (die Eingabetaste) und
        /// `cancelOperation:` (die Escape-Taste). Alles uebrige, darunter jede
        /// Bewegung der Schreibmarke, bleibt beim Feldeditor.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(control:textView:doCommandBySelector:))]
        fn befehl_umleiten(
            &self,
            _steuerung: &NSControl,
            _sicht: &NSTextView,
            befehl: Sel,
        ) -> objc2::runtime::Bool {
            if befehl == sel!(insertNewline:) {
                self.antworten(true);
                return objc2::runtime::Bool::YES;
            }
            if befehl == sel!(cancelOperation:) {
                self.antworten(false);
                return objc2::runtime::Bool::YES;
            }
            objc2::runtime::Bool::NO
        }
    }

    // SAFETY: `NSTextFieldDelegate` hat nur wahlfreie Methoden.
    unsafe impl NSTextFieldDelegate for Eingabewaechter {}
);

impl Eingabewaechter {
    /// Einen Waechter ohne Antwortweg.
    fn neu(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(WaechterIvars {
            antwort: RefCell::new(None),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Hinterlegt, was beim Bestaetigen und beim Abbrechen zu tun ist.
    fn antwort_setzen(&self, antwort: Antwortweg) {
        *self.ivars().antwort.borrow_mut() = Some(antwort);
    }

    /// Ruft den hinterlegten Antwortweg.
    fn antworten(&self, bestaetigt: bool) {
        // Die Ausleihe endet vor dem Aufruf: der Antwortweg schliesst das
        // Blatt, und AppKit kann dabei erneut hierher zurueckrufen.
        let antwort = self.ivars().antwort.borrow_mut().take();
        if let Some(antwort) = antwort {
            antwort(bestaetigt);
        }
    }
}

/// Ein Blatt mit einer Frage und zwei Schaltflaechen.
pub struct Blatt {
    warnung: Retained<NSAlert>,
    /// Der Delegierte des Eingabefeldes, falls es eines gibt.
    ///
    /// Ein `NSControl` haelt seinen Delegierten schwach; die starke Richtung
    /// laeuft deshalb von hier nach dort.
    waechter: Option<Retained<Eingabewaechter>>,
}

impl Blatt {
    /// Ein Blatt mit dieser Frage, einer bestaetigenden und einer abbrechenden
    /// Schaltflaeche.
    ///
    /// Die Reihenfolge ist bindend: die **erste** Schaltflaeche bestaetigt und
    /// traegt die Eingabetaste, die zweite bricht ab und traegt die
    /// Escape-Taste. Beides ist die Mac-Gewohnheit, und C2 verlangt sie
    /// ausdruecklich fuer jedes Textfeld.
    ///
    /// Beide Tastenentsprechungen stehen hier ausdruecklich und werden nicht
    /// AppKit ueberlassen: `NSAlert` gibt die Escape-Taste von sich aus allein
    /// einer Schaltflaeche mit dem Titel "Cancel", und den traegt eine
    /// deutschsprachige Anwendung nicht. Sie greifen, solange kein Textfeld im
    /// Bearbeitungszustand steht; fuer diesen Fall gibt es den
    /// [`Eingabewaechter`].
    pub fn neu(mtm: MainThreadMarker, frage: &str, bestaetigen: &str) -> Self {
        let warnung = NSAlert::new(mtm);
        warnung.setMessageText(&NSString::from_str(frage));
        let ja = warnung.addButtonWithTitle(&NSString::from_str(bestaetigen));
        ja.setKeyEquivalent(&NSString::from_str("\r"));
        let nein = warnung.addButtonWithTitle(&NSString::from_str("Abbrechen"));
        nein.setKeyEquivalent(&NSString::from_str("\u{1B}"));
        Self {
            warnung,
            waechter: None,
        }
    }

    /// Haengt ein Textfeld unter die Frage und macht es bedienbar.
    ///
    /// Drei Dinge auf einmal, weil sie zusammengehoeren: das Feld wird zur
    /// Beigabe des Blattes, es wird der Ersthelfer (sonst muesste der Nutzer
    /// erst hineinklicken), und es bekommt den [`Eingabewaechter`] als
    /// Delegierten.
    pub fn textfeld_setzen(&mut self, mtm: MainThreadMarker, feld: &NSTextField) {
        let sicht: &NSView = feld;
        self.warnung.setAccessoryView(Some(sicht));
        self.warnung.window().setInitialFirstResponder(Some(sicht));

        let waechter = Eingabewaechter::neu(mtm);
        // SAFETY: Der Waechter beantwortet `NSTextFieldDelegate`, das er oben
        // implementiert. Ueber die Lebensdauer verlangt die Bindung nichts; das
        // Feld haelt den Delegierten schwach, und `self.waechter` haelt ihn
        // stark, solange das Blatt lebt.
        unsafe { feld.setDelegate(Some(ProtocolObject::from_ref(&*waechter))) };
        self.waechter = Some(waechter);
    }

    /// Zeigt das Blatt am Fenster und meldet, ob bestaetigt wurde.
    ///
    /// Kehrt sofort zurueck. Der Rueckruf laeuft auf dem Hauptfaden, sobald der
    /// Nutzer geantwortet hat, und genau einmal: beide Wege, die Schaltflaeche
    /// und die Taste im Feld, muenden in denselben Abschlussblock von AppKit.
    pub fn zeigen(self, fenster: &NSWindow, fertig: impl Fn(bool) + 'static) {
        // Der Block haelt Warnung und Waechter fest. Ohne das fielen beide mit
        // diesem Aufruf, denn der Aufrufer gibt sie hier ab und AppKit haelt nur
        // das Fenster der Warnung. Der Ring bricht, sobald AppKit den Rueckruf
        // nach der Antwort freigibt.
        let warnung = self.warnung.clone();
        let waechter = self.waechter.clone();
        let block = RcBlock::new(move |antwort: NSModalResponse| {
            let _haelt = (&warnung, &waechter);
            fertig(antwort == NSAlertFirstButtonReturn);
        });
        self.warnung
            .beginSheetModalForWindow_completionHandler(fenster, Some(&block));

        // Der Waechter kann das Blatt erst jetzt beenden: das Fenster, an dem
        // es haengt, steht erst mit diesem Aufruf fest.
        if let Some(waechter) = &self.waechter {
            let blattfenster = self.warnung.window();
            let elternfenster = fenster.retain();
            waechter.antwort_setzen(Box::new(move |bestaetigt| {
                let antwort = if bestaetigt {
                    NSAlertFirstButtonReturn
                } else {
                    NSAlertSecondButtonReturn
                };
                elternfenster.endSheet_returnCode(&blattfenster, antwort);
            }));
        }
    }
}
