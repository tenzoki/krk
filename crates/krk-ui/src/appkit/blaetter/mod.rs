//! Die gemeinsame Huelle fuer die Blaetter am Fenster.
//!
//! Ein Blatt ist ein Dialog, der am oberen Rand des Fensters herunterfaehrt und
//! es blockiert, solange er steht. AppKit nennt das ein Sheet. KRK hat in dieser
//! Runde sechs: die Pfadeingabe aus C2 und fuenf zu C4 (Konflikt, Rueckfrage vor
//! dem endgueltigen Loeschen, Abschlussliste der uebersprungenen Eintraege und
//! seit Schritt 17 die Namenseingabe fuer das Anlegen sowie das Umbenennen im
//! Stapel).
//!
//! **Der Stand einer laufenden Dateioperation ist seit Schritt 16b keines
//! mehr.** Er stand bis dahin als fuenftes Blatt hier und ist in die
//! Statuszeile des Dateifensters gewandert, weil ein Blatt genau die
//! Oberflaeche sperrt, die C4 waehrend einer laufenden Operation bedienbar
//! zusagt, und weil es zum Aufgehen laenger braucht, als L8 zusagt. Die drei
//! uebrigen bleiben: die beiden Rueckfragen **sollen** sperren, weil die
//! Operation ohne die Antwort nicht weiterlaeuft, und die Abschlussliste fuehrt
//! mehrere Eintraege mit Grund und passt in keine einzeilige Statuszeile.
//! Bindend ist der Entscheid des Nutzers vom 260804-1832, "Blatt oder
//! Statuszeile", im Entscheidungsspeicher des Circles.
//!
//! ```text
//! Blatt::neu ──> textfeld_setzen ──> zeigen(fenster, fertig)
//!                                         │
//!                        fertig(true|false) auf dem Hauptfaden
//!
//! Blatt::mit_schaltflaechen ──> zeigen_mit_wahl(fenster, fertig) ──> Blattgriff
//!                                         │                             │
//!                         fertig(Stelle der Schaltflaeche)     schliessen()
//! ```
//!
//! **Die Antwort kommt als gewoehnlicher Rust-Wert zurueck.** Der Aufrufer sieht
//! einen `bool` oder die Stelle der gedrueckten Schaltflaeche und nicht eine
//! `NSModalResponse`; was AppKit dafuer als Zahl fuehrt, bleibt in dieser Datei.
//!
//! **Der Grund fuer eine gemeinsame Huelle** ist derselbe wie ueberall in
//! diesem Entwurf: fuenf Blaetter mit je eigenem Aufbau waeren fuenf Stellen,
//! die dieselbe Frage beantworten, und die erste Abweichung zwischen ihnen
//! faende keine Pruefung.
//!
//! # Die Tastenentsprechungen stehen ausdruecklich da
//!
//! `NSAlert` gibt die Eingabetaste von sich aus der **ersten** Schaltflaeche und
//! die Escape-Taste nur einer mit dem Titel "Cancel", den eine
//! deutschsprachige Anwendung nicht traegt. Beides waere fuer die Rueckfrage vor
//! dem endgueltigen Loeschen falsch: C4 verlangt dort **Abbrechen** als
//! Vorbelegung, damit ein reflexhaftes Bestaetigen mit der Eingabetaste nichts
//! loescht. [`Blatt::mit_schaltflaechen`] nimmt die Taste deshalb je
//! Schaltflaeche entgegen und loescht die Vorgabe von `NSAlert`, wo sie nicht
//! gemeint ist.
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
//!
//! **Ein Blatt haelt genau einen Waechter, auch bei mehreren Feldern.** Das
//! Stapel-Umbenennen aus Schritt 17 traegt vier Eingabefelder; der Waechter
//! entscheidet nicht nach Feld, sondern beantwortet zwei Tasten, und die
//! bedeuten in jedem Feld dasselbe. Vier Waechter waeren vier Wahrheiten
//! darueber, was die Eingabetaste in einem Blatt tut. Ueber denselben Waechter
//! laeuft die Meldung, dass sich ein Text geaendert hat
//! ([`Blatt::textaenderung_melden`]): daran haengt die Vorschau des
//! Stapel-Umbenennens, die mit jedem getippten Zeichen neu rechnet.

pub mod konflikt;
pub mod loeschbestaetigung;
pub mod namenseingabe;
pub mod pfadeingabe;
pub mod stapelumbenennen;
pub mod uebersprungen;

use std::cell::RefCell;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::{ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAlert, NSAlertFirstButtonReturn, NSAlertSecondButtonReturn, NSAlertStyle, NSButton,
    NSControl, NSControlStateValueOff, NSControlTextEditingDelegate, NSEventModifierFlags,
    NSModalResponse, NSTextField, NSTextFieldDelegate, NSTextView, NSView, NSWindow,
};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol, NSString, ns_string};

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
    /// Was zu tun ist, wenn sich der Text eines bewachten Feldes geaendert hat.
    ///
    /// Wahlfrei, weil die meisten Blaetter nichts damit anfangen: allein die
    /// Vorschau des Stapel-Umbenennens rechnet mit jedem Zeichen neu.
    aenderung: RefCell<Option<Box<dyn Fn()>>>,
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

        /// Der Text eines bewachten Feldes hat sich geaendert.
        ///
        /// Gemeldet wird die Aenderung selbst und nicht das Feld: das Blatt
        /// liest ohnehin alle seine Felder, wenn es neu rechnet, und ein
        /// Rueckruf je Feld waere eine Fallunterscheidung ohne Fall.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(controlTextDidChange:))]
        fn text_geaendert(&self, _meldung: &objc2_foundation::NSNotification) {
            let aenderung = self.ivars().aenderung.borrow();
            if let Some(aenderung) = aenderung.as_ref() {
                aenderung();
            }
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
            aenderung: RefCell::new(None),
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

/// Welche Taste eine Schaltflaeche ausloest.
///
/// # Warum es die beiden Eingabetasten mit Zusatztaste gibt
///
/// Ein `NSButton` traegt genau **eine** Tastenentsprechung, und der Tabulator
/// erreicht die Schaltflaechen eines Blattes nur, wenn der Nutzer im System die
/// vollstaendige Tastaturnavigation eingeschaltet hat. Ein Blatt mit mehr als
/// zwei Antworten waere ohne Maus damit nicht zu beantworten, und C4 verlangt
/// fuer die Rueckfrage vor dem endgueltigen Loeschen ausdruecklich das
/// Gegenteil. Die beiden Kombinationen mit Zusatztaste geben jeder weiteren
/// Antwort einen eigenen Griff; **das Blatt schreibt sie in seinen
/// erlaeuternden Text**, sonst waeren sie unauffindbar.
///
/// Sie kollidieren mit nichts: `resources/default-keymap.toml` belegt weder die
/// Eingabetaste noch eine ihrer Kombinationen, der Ereignisabgriff findet
/// nichts und reicht den Tastendruck an AppKit weiter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Taste {
    /// Die Eingabetaste. Hoechstens eine Schaltflaeche je Blatt traegt sie.
    Eingabe,
    /// Cmd und die Eingabetaste.
    EingabeMitBefehl,
    /// Wahltaste und die Eingabetaste.
    EingabeMitWahl,
    /// Die Escape-Taste. Hoechstens eine Schaltflaeche je Blatt traegt sie.
    Escape,
}

impl Taste {
    /// Das Zeichen, das `NSButton.keyEquivalent` dafuer traegt.
    fn zeichen(self) -> &'static NSString {
        match self {
            Taste::Eingabe | Taste::EingabeMitBefehl | Taste::EingabeMitWahl => ns_string!("\r"),
            Taste::Escape => ns_string!("\u{1B}"),
        }
    }

    /// Die Zusatztasten, die dazu gehalten werden muessen.
    fn zusatztasten(self) -> NSEventModifierFlags {
        match self {
            Taste::EingabeMitBefehl => NSEventModifierFlags::Command,
            Taste::EingabeMitWahl => NSEventModifierFlags::Option,
            _ => NSEventModifierFlags::empty(),
        }
    }
}

/// Eine Schaltflaeche eines Blattes.
#[derive(Debug, Clone, Copy)]
pub struct Schaltflaeche<'a> {
    /// Die Beschriftung.
    pub titel: &'a str,
    /// Die Taste, die sie ausloest.
    pub taste: Taste,
}

impl<'a> Schaltflaeche<'a> {
    /// Eine Schaltflaeche mit dieser Beschriftung und dieser Taste.
    pub fn neu(titel: &'a str, taste: Taste) -> Self {
        Self { titel, taste }
    }
}

/// Ein stehendes Blatt, das der Aufrufer wieder schliessen kann.
///
/// Der Abbruchbefehl braucht das: `esc` schliesst ein stehendes Blatt ueber
/// seinen Griff, weil ein `NSButton` genau eine Tastenentsprechung traegt und
/// die Rueckfrage vor dem endgueltigen Loeschen die Eingabetaste auf
/// "Abbrechen" gelegt hat. Wer den Griff nicht braucht, laesst ihn fallen; das
/// schadet nicht, weil AppKit das Blatt haelt, solange es steht.
pub struct Blattgriff {
    warnung: Retained<NSAlert>,
    fenster: Retained<NSWindow>,
    /// Der Rueckgabewert, den [`Blattgriff::abbrechen`] einsetzt.
    abbruchcode: NSModalResponse,
}

impl Blattgriff {
    /// Schliesst das Blatt mit dem Rueckgabewert seiner abbrechenden
    /// Schaltflaeche.
    ///
    /// Der Abschlussblock laeuft dabei ganz gewoehnlich; ein programmatischer
    /// Abbruch und ein Klick auf "Abbrechen" gehen damit denselben Weg, und es
    /// gibt keine zweite Stelle, die einen Abbruch behandelt.
    pub fn abbrechen(&self) {
        self.fenster
            .endSheet_returnCode(&self.warnung.window(), self.abbruchcode);
    }
}

/// Ein Blatt mit einer Frage und Schaltflaechen.
pub struct Blatt {
    warnung: Retained<NSAlert>,
    /// Die Rueckgabewerte der Schaltflaechen, in der Reihenfolge, in der sie
    /// angelegt wurden.
    antworten: Vec<NSModalResponse>,
    /// Die Stelle der abbrechenden Schaltflaeche, falls es eine gibt.
    abbruchstelle: Option<usize>,
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
    pub fn neu(mtm: MainThreadMarker, frage: &str, bestaetigen: &str) -> Self {
        Self::mit_schaltflaechen(
            mtm,
            frage,
            &[
                Schaltflaeche::neu(bestaetigen, Taste::Eingabe),
                Schaltflaeche::neu("Abbrechen", Taste::Escape),
            ],
        )
    }

    /// Ein Blatt mit dieser Frage und diesen Schaltflaechen.
    ///
    /// Die erste Schaltflaeche steht rechts und ist die hervorgehobene; welche
    /// die Eingabetaste traegt, entscheidet allein das Feld
    /// [`Schaltflaeche::taste`]. Genau deshalb kann die Rueckfrage vor dem
    /// endgueltigen Loeschen "Abbrechen" vorbelegen, ohne die Reihenfolge zu
    /// verdrehen, die C4 aufzaehlt.
    pub fn mit_schaltflaechen(
        mtm: MainThreadMarker,
        frage: &str,
        schaltflaechen: &[Schaltflaeche<'_>],
    ) -> Self {
        let warnung = NSAlert::new(mtm);
        warnung.setMessageText(&NSString::from_str(frage));
        let mut antworten = Vec::with_capacity(schaltflaechen.len());
        let mut abbruchstelle = None;
        for (stelle, schaltflaeche) in schaltflaechen.iter().enumerate() {
            let knopf = warnung.addButtonWithTitle(&NSString::from_str(schaltflaeche.titel));
            // Auch `Taste::Keine` wird gesetzt und nicht ausgelassen: `NSAlert`
            // gibt der ersten Schaltflaeche von sich aus die Eingabetaste, und
            // ohne das Loeschen traegt sie zwei Blaetter spaeter eine Taste, die
            // niemand ihr zugedacht hat.
            knopf.setKeyEquivalent(schaltflaeche.taste.zeichen());
            knopf.setKeyEquivalentModifierMask(schaltflaeche.taste.zusatztasten());
            antworten.push(antwort_von_stelle(stelle));
            if schaltflaeche.taste == Taste::Escape {
                abbruchstelle = Some(stelle);
            }
        }
        Self {
            warnung,
            antworten,
            abbruchstelle,
            waechter: None,
        }
    }

    /// Setzt den erlaeuternden Text unter der Frage.
    pub fn erlaeuterung_setzen(&self, text: &str) {
        self.warnung.setInformativeText(&NSString::from_str(text));
    }

    /// Macht das Blatt zur Warnung, mit dem Warnzeichen des Systems.
    ///
    /// Fuer die Rueckfrage vor dem endgueltigen Loeschen: sie ist der eine
    /// Vorgang in KRK, der keinen Rueckweg hat.
    pub fn als_warnung(&self) {
        self.warnung.setAlertStyle(NSAlertStyle::Critical);
    }

    /// Haengt eine beliebige Ansicht unter die Frage.
    ///
    /// Der allgemeine Fall von [`Blatt::textfeld_setzen`], ohne Ersthelfer und
    /// ohne Waechter, fuer eine Beigabe, die ihren Fokus selbst regelt.
    pub fn beigabe_setzen(&self, sicht: &NSView) {
        self.warnung.setAccessoryView(Some(sicht));
    }

    /// Zeigt das Kaestchen "fuer alle weiteren uebernehmen" (C4).
    ///
    /// `NSAlert` fuehrt es als Unterdrueckungskaestchen. Es dafuer zu benutzen
    /// spart eine eigene Beigabe samt Anordnung, und die Bedeutung ist
    /// dieselbe: diese Antwort gilt auch fuer die naechsten Faelle.
    pub fn wahl_fuer_alle_zeigen(&self, titel: &str) {
        self.warnung.setShowsSuppressionButton(true);
        if let Some(kaestchen) = self.warnung.suppressionButton() {
            let knopf: &NSButton = &kaestchen;
            knopf.setTitle(&NSString::from_str(titel));
        }
    }

    /// Macht diese Ansicht zum Ersthelfer, sobald das Blatt steht.
    ///
    /// Ohne sie muesste der Nutzer in das erste Feld klicken, und ein Blatt,
    /// das ohne Maus nicht anfaengt, ist ohne Maus nicht bedienbar.
    pub fn ersthelfer_setzen(&self, sicht: &NSView) {
        self.warnung.window().setInitialFirstResponder(Some(sicht));
    }

    /// Gibt diesem Textfeld den [`Eingabewaechter`] des Blattes.
    ///
    /// Mehrere Felder teilen sich **einen** Waechter; der Grund steht im
    /// Modulkopf. Ohne ihn verbraucht der Feldeditor die Eingabe- und die
    /// Escape-Taste selbst, und das Blatt liesse sich mit keiner von beiden
    /// beantworten.
    pub fn waechter_anhaengen(&mut self, mtm: MainThreadMarker, feld: &NSTextField) {
        let waechter = self
            .waechter
            .get_or_insert_with(|| Eingabewaechter::neu(mtm))
            .clone();
        // SAFETY: Der Waechter beantwortet `NSTextFieldDelegate`, das er oben
        // implementiert. Ueber die Lebensdauer verlangt die Bindung nichts; das
        // Feld haelt den Delegierten schwach, und `self.waechter` haelt ihn
        // stark, solange das Blatt lebt.
        unsafe { feld.setDelegate(Some(ProtocolObject::from_ref(&*waechter))) };
    }

    /// Hinterlegt, was bei jeder Textaenderung in einem bewachten Feld
    /// geschieht.
    ///
    /// Der Weg der Vorschau des Stapel-Umbenennens: sie rechnet mit jedem
    /// getippten Zeichen neu, damit der Nutzer die Regel an ihrem Ergebnis
    /// pruefen kann, bevor er sie ausfuehrt (C4). Ohne einen Waechter geschieht
    /// nichts; die Meldung braucht ein bewachtes Feld.
    pub fn textaenderung_melden(&self, melden: Box<dyn Fn()>) {
        if let Some(waechter) = &self.waechter {
            *waechter.ivars().aenderung.borrow_mut() = Some(melden);
        }
    }

    /// Haengt ein Textfeld unter die Frage und macht es bedienbar.
    ///
    /// Drei Dinge auf einmal, weil sie zusammengehoeren: das Feld wird zur
    /// Beigabe des Blattes, es wird der Ersthelfer (sonst muesste der Nutzer
    /// erst hineinklicken), und es bekommt den [`Eingabewaechter`] als
    /// Delegierten. Ein Blatt mit mehreren Feldern setzt die drei Schritte
    /// einzeln; die Beigabe ist dann der Rahmen um die Felder und nicht eines
    /// davon.
    pub fn textfeld_setzen(&mut self, mtm: MainThreadMarker, feld: &NSTextField) {
        let sicht: &NSView = feld;
        self.beigabe_setzen(sicht);
        self.ersthelfer_setzen(sicht);
        self.waechter_anhaengen(mtm, feld);
    }

    /// Zeigt das Blatt am Fenster und meldet, ob bestaetigt wurde.
    ///
    /// Kehrt sofort zurueck. Der Rueckruf laeuft auf dem Hauptfaden, sobald der
    /// Nutzer geantwortet hat, und genau einmal: beide Wege, die Schaltflaeche
    /// und die Taste im Feld, muenden in denselben Abschlussblock von AppKit.
    ///
    /// "Bestaetigt" heisst: die **erste** Schaltflaeche. Fuer ein Blatt mit
    /// mehr als zweien ist [`Blatt::zeigen_mit_wahl`] der richtige Weg.
    pub fn zeigen(self, fenster: &NSWindow, fertig: impl Fn(bool) + 'static) {
        let _griff = self.zeigen_mit_wahl(fenster, move |stelle, _fuer_alle| fertig(stelle == 0));
    }

    /// Zeigt das Blatt am Fenster und meldet die Stelle der gedrueckten
    /// Schaltflaeche.
    ///
    /// Gezaehlt wird in der Reihenfolge, in der die Schaltflaechen angelegt
    /// wurden. Das zweite Argument des Rueckrufs sagt, ob das Kaestchen aus
    /// [`Blatt::wahl_fuer_alle_zeigen`] angekreuzt war; ohne Kaestchen ist es
    /// immer `false`. Der zurueckgegebene [`Blattgriff`] schliesst das Blatt von
    /// aussen; wer ihn nicht braucht, laesst ihn fallen.
    pub fn zeigen_mit_wahl(
        self,
        fenster: &NSWindow,
        fertig: impl Fn(usize, bool) + 'static,
    ) -> Blattgriff {
        // Der Block haelt Warnung und Waechter fest. Ohne das fielen beide mit
        // diesem Aufruf, denn der Aufrufer gibt sie hier ab und AppKit haelt nur
        // das Fenster der Warnung. Der Ring bricht, sobald AppKit den Rueckruf
        // nach der Antwort freigibt.
        let warnung = self.warnung.clone();
        let waechter = self.waechter.clone();
        let antworten = self.antworten.clone();
        let block = RcBlock::new(move |antwort: NSModalResponse| {
            let _haelt = &waechter;
            // Eine unbekannte Antwort gilt als die letzte Schaltflaeche, und
            // die ist in jedem Blatt dieser Runde die abbrechende. Lieber
            // nichts tun als raten.
            let stelle = antworten
                .iter()
                .position(|kandidat| *kandidat == antwort)
                .unwrap_or(antworten.len().saturating_sub(1));
            let fuer_alle = warnung
                .suppressionButton()
                .is_some_and(|kaestchen| kaestchen.state() != NSControlStateValueOff);
            fertig(stelle, fuer_alle);
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

        let abbruchcode = self
            .abbruchstelle
            .map_or(NSAlertFirstButtonReturn, antwort_von_stelle);
        Blattgriff {
            warnung: self.warnung,
            fenster: fenster.retain(),
            abbruchcode,
        }
    }
}

/// Der Rueckgabewert, den `NSAlert` fuer die Schaltflaeche an dieser Stelle
/// liefert.
///
/// AppKit zaehlt sie ab `NSAlertFirstButtonReturn` fortlaufend hoch. Die
/// Umrechnung steht hier einmal, damit keine Zaehlung mit den Zahlen von AppKit
/// rechnet.
fn antwort_von_stelle(stelle: usize) -> NSModalResponse {
    NSAlertFirstButtonReturn + stelle as NSModalResponse
}
