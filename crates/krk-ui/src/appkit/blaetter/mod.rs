//! Die gemeinsame Huelle fuer die Blaetter am Fenster.
//!
//! Ein Blatt ist ein Dialog, der am oberen Rand des Fensters herunterfaehrt und
//! es blockiert, solange er steht. AppKit nennt das ein Sheet. KRK hat zehn:
//! die Pfadeingabe aus C2 und fuenf zu C4 der Runde 1 (Konflikt, Rueckfrage vor
//! dem Raeumen in den Papierkorb, Abschlussliste der uebersprungenen Eintraege
//! und seit Schritt 17 die Namenseingabe fuer das Anlegen sowie das Umbenennen
//! im Stapel), dazu seit S27 der Editor-Runde die Nachfrage vor dem Verlust
//! eines ungesicherten Standes ([`ungesichert`], C4 der Editor-Runde) und seit
//! S35 und S36 die beiden Eingabeblaetter des Editors: die Frage nach der
//! Zeilennummer ([`zeilennummer`]) und die nach Such- und Ersatztext
//! ([`suche`]), beide C5 der Editor-Runde. Das zehnte ist der Notizzettel der
//! Runde 9 ([`zettel`]).
//!
//! **Das zehnte ist das erste mit einem eigenen Waechter**, und der traegt die
//! halbe Regel des [`Eingabewaechter`] darunter: `Esc` schliesst, die
//! Eingabetaste setzt eine Zeile. Warum das ein eigener Typ ist und kein
//! Schalter hier, steht im Kopf von [`zettel`].
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
//! diesem Entwurf: zehn Blaetter mit je eigenem Aufbau waeren zehn Stellen,
//! die dieselbe Frage beantworten, und die erste Abweichung zwischen ihnen
//! faende keine Pruefung.
//!
//! # Die Tastenentsprechungen stehen ausdruecklich da
//!
//! `NSAlert` gibt die Eingabetaste von sich aus der **ersten** Schaltflaeche und
//! die Escape-Taste nur einer mit dem Titel "Cancel", den eine
//! deutschsprachige Anwendung nicht traegt. Beides waere fuer die Rueckfrage vor
//! dem Raeumen in den Papierkorb falsch: C4 verlangt dort **Abbrechen** als
//! Vorbelegung, damit ein reflexhaftes Bestaetigen mit der Eingabetaste nichts
//! loescht. [`Blatt::mit_schaltflaechen`] nimmt die Taste deshalb je
//! Schaltflaeche entgegen und loescht die Vorgabe von `NSAlert`, wo sie nicht
//! gemeint ist.
//!
//! # Welche Schaltflaeche die ungefaehrliche ist, steht genau einmal
//!
//! Drei Stellen dieser Datei brauchen die Antwort: der Abschlussblock, wenn
//! `NSAlert` einen Rueckgabewert liefert, der zu keiner angelegten
//! Schaltflaeche gehoert, [`Blattgriff::abbrechen`], das ein stehendes Blatt
//! von aussen schliesst, und der [`Eingabewaechter`], wenn die Escape-Taste im
//! Textfeld faellt. Sie steht als [`abbruchstelle`] einmal da, ist eine reine
//! Funktion ueber die angelegte Reihenfolge und ohne AppKit pruefbar.
//!
//! **Bis zum 260817 stand sie zweimal und widersprach sich.** Der
//! Abschlussblock nahm die **letzte** Schaltflaeche, der Griff die **erste**.
//! In der Rueckfrage vor dem Loeschen ist die letzte die loeschende, also fiel
//! eine unbekannte Antwort dort auf den zerstoerenden Ausgang
//! (`issues/260817-1106_*_eine-unbekannte-blattantwort-faellt-im-loeschblatt-auf-die-zerstoerende-schaltflaeche.md`).
//!
//! **Abgeleitet wird sie nicht aus der Escape-Taste, sondern aus der
//! [`Wirkung`], die jede Schaltflaeche ausdruecklich mitbringt.** Die
//! Rueckfrage vor dem Loeschen traegt gar keine Schaltflaeche mit
//! [`Taste::Escape`], denn ihr Abbruch laeuft ueber den Befehl `abbrechen` aus
//! `resources/default-keymap.toml`; aus einer Taste, die es nicht gibt, ist
//! nichts abzuleiten. Die [`Wirkung`] hat keine Vorgabe, also kann kein
//! kuenftiges Blatt sie stillschweigend auslassen.
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
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSAlert`, `NSButton`, `NSControl`, `NSTextField`, `NSTextView`, `NSView`,
//! `NSWindow`, `NSObject`, `NSString` und `NSNotification` stehen seit macOS
//! 10.0 zur Verfuegung, ebenso die Protokolle `NSControlTextEditingDelegate`
//! und `NSTextFieldDelegate` samt der beiden hier beantworteten Methoden
//! `control:textView:doCommandBySelector:` und `controlTextDidChange:`, dazu
//! die Aufzaehlungen `NSAlertStyle` und `NSEventModifierFlags` und die
//! Zugriffe `setInitialFirstResponder:`, `setNextKeyView:` und
//! `NSTextField.delegate`.
//!
//! **Fuenf Beruehrungen sind juenger als 10.0**, und alle fuenf liegen unter
//! dem Zielsystem: `setAccessoryView:`, `setShowsSuppressionButton:` und
//! `suppressionButton` seit 10.5 (`NSAlert.h`), und die beiden, ueber die jedes
//! Blatt dieser Datei aufgeht und wieder zugeht, seit 10.9 —
//! `beginSheetModalForWindow:completionHandler:` (`NSAlert.h`) und
//! `endSheet:returnCode:` (`NSWindow.h`). Der Abschlussblock aus `block2` ist
//! kein eigener Gegenstand der Frage: er ist das Argument der ersten von
//! beiden, und seine Untergrenze ist deren.
//!
//! `NSModalResponse` ist ein `typedef` auf `NSInteger` ohne
//! Verfuegbarkeitsangabe; `NSAlertFirstButtonReturn` und
//! `NSControlStateValueOff` sind Uebersetzungszeitkonstanten ohne eigenes
//! Laufzeitsymbol. Keiner der drei stellt die Frage ueberhaupt.
//! `NSAlertSecondButtonReturn` stand hier bis zum 260817 als vierter; die eine
//! Stelle, die ihn nannte, rechnet ihre Antwort seither aus
//! [`abbruchstelle`].
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

pub mod konflikt;
pub mod loeschbestaetigung;
pub mod namenseingabe;
pub mod pfadeingabe;
pub mod stapelumbenennen;
pub mod suche;
pub mod uebersprungen;
pub mod ungesichert;
pub mod zeilennummer;
pub mod zettel;

use std::cell::RefCell;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::{ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAlert, NSAlertFirstButtonReturn, NSAlertStyle, NSButton, NSControl, NSControlStateValueOff,
    NSControlTextEditingDelegate, NSEventModifierFlags, NSModalResponse, NSTextField,
    NSTextFieldDelegate, NSTextView, NSView, NSWindow,
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
/// fuer die Rueckfrage vor dem Raeumen in den Papierkorb ausdruecklich das
/// Gegenteil. Die beiden Kombinationen mit Zusatztaste geben jeder weiteren
/// Antwort einen eigenen Griff; **das Blatt schreibt sie in seinen
/// erlaeuternden Text**, sonst waeren sie unauffindbar.
///
/// **Seit dem 260811 belegt `resources/default-keymap.toml` die nackte
/// Eingabetaste**, naemlich mit `mit_standardprogramm_oeffnen` aus C3 der
/// Runde 4. Bis dahin war sie ab Werk frei, und dieser Absatz sagte deshalb zu,
/// die Tastenentsprechungen kollidierten mit nichts. Die Zusage ist gebrochen,
/// das Verhalten nicht: die Schaltflaechen loesen weiterhin aus, und der Grund
/// ist ein anderer geworden.
///
/// Er lautet: bei stehendem Blatt weist
/// `Anwendungsdelegierter::kommando_ausfuehren` jeden Befehl ab bis auf vier —
/// den Abbruch (`crate::kommandos::operationen::waehrend_blatt_erlaubt`) und
/// die drei der Ausnahmeliste `crate::kommandos::zulaessigkeit::immer_erreichbar`,
/// die die Blattsperre mit aufhebt —, und ein
/// abgewiesener Tastendruck laeuft unveraendert an AppKit weiter, wo die
/// Vorgabeschaltflaeche ihn beantwortet. Keiner der drei zusaetzlich
/// zugelassenen Befehle liegt ab Werk auf einer der Eingabetasten-Kombinationen
/// dieser Aufzaehlung — sie liegen auf `cmd+q`, `shift+cmd+w` und `cmd+n` —,
/// also traegt der Grund die Zusage weiterhin. Die Sperre steht **vor** dem
/// Fokusvorbehalt, also greift sie auch dann, wenn der Fokus in einem
/// Dateifenster steht und der Befehl dort wirken wuerde.
///
/// Die beiden Kombinationen mit Zusatztaste sind ab Werk unbelegt geblieben;
/// fuer sie findet der Ereignisabgriff weiterhin nichts und reicht den
/// Tastendruck unveraendert an AppKit weiter.
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

/// Was das Druecken einer Schaltflaeche anrichtet.
///
/// **Das Feld hat keine Vorgabe, und das ist der Zweck der Aufzaehlung.** Jedes
/// Blatt sagt fuer jede seiner Schaltflaechen, ob sie etwas anrichtet; genau
/// daraus liest [`abbruchstelle`] die ungefaehrliche Stelle. Eine Vorgabe hier
/// waere eine Sicherung, die man beim naechsten Blatt vergessen kann, ohne dass
/// es auffaellt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wirkung {
    /// Sie fuehrt den Vorgang aus, um den das Blatt fragt: loeschen,
    /// ueberschreiben, umbenennen, sichern, verwerfen.
    Ausfuehren,
    /// Sie laesst den Vorgang liegen, um den das Blatt fragt.
    ///
    /// Der Abbruch. Ebenso der einzige Ausgang eines Blattes, das nach keinem
    /// Vorgang fragt: die Abschlussliste der uebersprungenen Eintraege
    /// ("Schliessen"), die Tastaturbelegung und der Notizzettel (beide
    /// "Fertig"). Dort steht keine ausfuehrende Schaltflaeche daneben, aus der
    /// eine verlorene Antwort waehlen koennte, und das Schliessen ist der
    /// ungefaehrliche Ausgang, weil es derselbe ist, den die Escape-Taste nimmt.
    Liegenlassen,
}

/// Eine Schaltflaeche eines Blattes.
#[derive(Debug, Clone, Copy)]
pub struct Schaltflaeche<'a> {
    /// Die Beschriftung.
    pub titel: &'a str,
    /// Die Taste, die sie ausloest.
    pub taste: Taste,
    /// Was ihr Druecken anrichtet.
    pub wirkung: Wirkung,
}

impl<'a> Schaltflaeche<'a> {
    /// Eine Schaltflaeche mit dieser Beschriftung, dieser Taste und dieser
    /// Wirkung.
    pub fn neu(titel: &'a str, taste: Taste, wirkung: Wirkung) -> Self {
        Self {
            titel,
            taste,
            wirkung,
        }
    }
}

/// Die Stelle, auf die eine Antwort faellt, die zu keiner Schaltflaeche gehoert.
///
/// Die eine Antwort auf die Frage "welche Schaltflaeche ist die ungefaehrliche"
/// (Modulkopf), als reine Funktion ueber die angelegte Reihenfolge.
/// [`Blatt::mit_schaltflaechen`] ruft sie einmal je Blatt und legt das Ergebnis
/// ab; die drei Stellen, die es lesen, stehen im Modulkopf.
///
/// # Die Tafel
///
/// | Die Schaltflaechen des Blattes | Ergebnis |
/// |---|---|
/// | die erste mit [`Wirkung::Liegenlassen`] steht an Stelle `s` | `s` |
/// | keine traegt [`Wirkung::Liegenlassen`] | `0` |
/// | keine Schaltflaeche | `0` |
///
/// **Die zweite Zeile ist ein Blatt, das es nicht geben soll**, und
/// [`Blatt::mit_schaltflaechen`] laesst es auffliegen. Ein Blatt,
/// dessen Schaltflaechen alle etwas ausfuehren, hat keinen ungefaehrlichen
/// Ausgang; die Regel nimmt dann die erste Stelle, weil sie die einzige ist,
/// die jedes Blatt hat. Etwas Ungefaehrliches trifft sie dabei nicht, denn es
/// gibt nichts Ungefaehrliches zu treffen.
///
/// **„Laesst es auffliegen" gilt seit dem 260818 fuer jeden Bau und ist
/// gemessen.** Bis dahin stand hier „im Probenbau", und der Satz traf auf
/// keinen Bau zu, den KRK herstellt: die Zusicherung war ein `debug_assert!`
/// und damit im Auslieferungsbau nicht vorhanden, und im Probenbau erreichte
/// sie keine Probe
/// (`issues/260817-1419_*_die-zusicherung-gegen-ein-blatt-ohne-ungefaehrlichen-ausgang-greift-in-keinem-bau.md`).
/// Beides steht jetzt anders, und `ein_blatt_ohne_ungefaehrlichen_ausgang_fliegt_auf`
/// haelt es fest. Dass diese Regel dabei total bleibt und die zweite Zeile
/// ihrer Tafel behaelt, ist Absicht: die Tafel beschreibt, was die reine
/// Funktion tut, und das Blatt, das sie beschreibt, kommt am Bauer nicht mehr
/// vorbei.
///
/// Die dritte Zeile kommt nicht vor: ein `NSAlert` ohne zugefuegte
/// Schaltflaeche legt selbst eine an, und deren Antwort ist
/// `NSAlertFirstButtonReturn`, also Stelle 0. Sie steht in der Tafel, weil eine
/// Tafel mit einer Luecke keine ist.
#[must_use]
pub fn abbruchstelle(schaltflaechen: &[Schaltflaeche<'_>]) -> usize {
    schaltflaechen
        .iter()
        .position(|schaltflaeche| schaltflaeche.wirkung == Wirkung::Liegenlassen)
        .unwrap_or(0)
}

/// Ein stehendes Blatt, das der Aufrufer wieder schliessen kann.
///
/// Der Abbruchbefehl braucht das: `esc` schliesst ein stehendes Blatt ueber
/// seinen Griff, weil ein `NSButton` genau eine Tastenentsprechung traegt und
/// die Rueckfrage vor dem Raeumen in den Papierkorb die Eingabetaste auf
/// "Abbrechen" gelegt hat. Wer den Griff nicht braucht, laesst ihn fallen; das
/// schadet nicht, weil AppKit das Blatt haelt, solange es steht.
pub struct Blattgriff {
    warnung: Retained<NSAlert>,
    fenster: Retained<NSWindow>,
    /// Der Rueckgabewert, den [`Blattgriff::abbrechen`] einsetzt.
    ///
    /// Er gehoert der Schaltflaeche an [`abbruchstelle`]; die Frage nach der
    /// ungefaehrlichen Stelle ist in dieser Datei einmal beantwortet.
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

    /// Derselbe Abbruch als festhaltbarer Ruf.
    ///
    /// **Fuer einen Delegierten des Blattes, der es selbst schliessen muss und
    /// den Griff nicht bekommt.** Der eine Aufrufer ist der
    /// [`Zettelwaechter`](zettel::Zettelwaechter): der Griff geht an den
    /// Anwendungsdelegierten, damit `esc` ueber den Abbruchbefehl dasselbe tut
    /// wie bei jedem anderen Blatt, und der Waechter braucht daneben einen
    /// eigenen Weg fuer die Escape-Taste **in** der Textflaeche.
    ///
    /// **Ein zweiter Schliessweg entsteht damit nicht.** Der Ruf tut Zeile fuer
    /// Zeile, was [`Blattgriff::abbrechen`] tut, und beide muenden in denselben
    /// Abschlussblock von AppKit. Ein zweiter Ruf, nachdem das Blatt schon zu
    /// ist, trifft ein Fenster ohne anhaengendes Blatt und tut nichts.
    pub fn abbruchweg(&self) -> impl Fn() + use<> {
        let warnung = self.warnung.clone();
        let fenster = self.fenster.clone();
        let abbruchcode = self.abbruchcode;
        move || fenster.endSheet_returnCode(&warnung.window(), abbruchcode)
    }
}

/// Ein Blatt mit einer Frage und Schaltflaechen.
pub struct Blatt {
    warnung: Retained<NSAlert>,
    /// Die Rueckgabewerte der Schaltflaechen, in der Reihenfolge, in der sie
    /// angelegt wurden.
    antworten: Vec<NSModalResponse>,
    /// Die Stelle der Schaltflaeche, die alles liegen laesst.
    ///
    /// Aus [`abbruchstelle`] und damit aus der [`Wirkung`] der Schaltflaechen,
    /// nicht aus ihrer Taste.
    abbruchstelle: usize,
    /// Der Delegierte des Eingabefeldes, falls es eines gibt.
    ///
    /// Ein `NSControl` haelt seinen Delegierten schwach; die starke Richtung
    /// laeuft deshalb von hier nach dort.
    waechter: Option<Retained<Eingabewaechter>>,
}

/// Die beiden Schaltflaechen von [`Blatt::neu`], in bindender Reihenfolge.
///
/// **Als reine Funktion herausgezogen, damit der Bauplan der fuenf Blaetter aus
/// [`Blatt::neu`] ohne AppKit und ohne Hauptfaden pruefbar ist.** Dieselbe
/// Bauform wie `super::loeschbestaetigung::schaltflaechen`, und aus demselben
/// Grund: an einem gebauten `NSAlert` ist nicht mehr abzulesen, welche seiner
/// Schaltflaechen alles liegen laesst, an dieser Liste schon
/// (`issues/260817-1419_*_die-zusicherung-gegen-ein-blatt-ohne-ungefaehrlichen-ausgang-greift-in-keinem-bau.md`).
///
/// Die Reihenfolge ist bindend und steht bei [`Blatt::neu`] begruendet: die
/// erste bestaetigt und traegt die Eingabetaste, die zweite bricht ab.
fn standardschaltflaechen(bestaetigen: &str) -> [Schaltflaeche<'_>; 2] {
    [
        Schaltflaeche::neu(bestaetigen, Taste::Eingabe, Wirkung::Ausfuehren),
        Schaltflaeche::neu("Abbrechen", Taste::Escape, Wirkung::Liegenlassen),
    ]
}

impl Blatt {
    /// Ein Blatt mit dieser Frage, einer bestaetigenden und einer abbrechenden
    /// Schaltflaeche.
    ///
    /// Die Reihenfolge ist bindend: die **erste** Schaltflaeche bestaetigt und
    /// traegt die Eingabetaste, die zweite bricht ab und traegt die
    /// Escape-Taste. Beides ist die Mac-Gewohnheit, und C2 verlangt sie
    /// ausdruecklich fuer jedes Textfeld. Auf dieser Reihenfolge ruht der
    /// [`Eingabewaechter`]: er uebersetzt die Eingabetaste des Feldes in die
    /// **erste** Schaltflaeche, und jedes bewachte Blatt kommt von hier. Ein
    /// Feld allein reicht dafuer nicht — das Konfliktblatt traegt eines ohne
    /// Waechter —, ein Aufruf von [`Blatt::waechter_anhaengen`] tut es.
    pub fn neu(mtm: MainThreadMarker, frage: &str, bestaetigen: &str) -> Self {
        Self::mit_schaltflaechen(mtm, frage, &standardschaltflaechen(bestaetigen))
    }

    /// Ein Blatt mit dieser Frage und diesen Schaltflaechen.
    ///
    /// Die erste Schaltflaeche steht rechts und ist die hervorgehobene; welche
    /// die Eingabetaste traegt, entscheidet allein das Feld
    /// [`Schaltflaeche::taste`]. Genau deshalb kann die Rueckfrage vor dem
    /// Loeschen "Abbrechen" vorbelegen, ohne die Reihenfolge zu verdrehen, die
    /// C4 aufzaehlt. Welche Schaltflaeche nichts anrichtet, entscheidet ebenso
    /// allein das Feld [`Schaltflaeche::wirkung`]; siehe [`abbruchstelle`].
    ///
    /// **Mindestens eine Schaltflaeche traegt [`Wirkung::Liegenlassen`].** Ein
    /// Blatt ohne ungefaehrlichen Ausgang kann eine unbekannte Antwort nicht
    /// sicher beantworten; es fliegt hier auf, statt still auf eine
    /// ausfuehrende Schaltflaeche zu fallen.
    ///
    /// **Die Zusicherung ist seit dem 260818 ein `assert!` und kein
    /// `debug_assert!`.** Als `debug_assert!` griff sie in keinem Bau, den KRK
    /// herstellt: `cargo xtask bundle` uebersetzt mit `--profile release`, und
    /// Cargos Vorgabe dafuer ist `debug-assertions = false`; der Probenbau
    /// wiederum erreichte die Zeile nicht, weil keine Probe ein Blatt baute
    /// (`issues/260817-1419_*_die-zusicherung-gegen-ein-blatt-ohne-ungefaehrlichen-ausgang-greift-in-keinem-bau.md`).
    /// Beide Luecken sind geschlossen: die Form gilt in jedem Profil, und
    /// `ein_blatt_ohne_ungefaehrlichen_ausgang_fliegt_auf` erreicht sie.
    ///
    /// **Sie kostet nichts, was zu sparen waere.** Ein Blatt entsteht auf eine
    /// Nutzerhandlung hin, und die Pruefung liest zwei Felder. Ausloesen kann
    /// sie allein ein Bauplan im Quelltext, nie eine Eingabe des Nutzers: die
    /// Schaltflaechen jedes Blattes stehen fest im Baum. Ein Absturz an dieser
    /// Stelle meldet also einen Programmierfehler, und die Lage, die er
    /// abloest, ist ein Blatt, dessen unbekannte Antwort loescht.
    pub fn mit_schaltflaechen(
        mtm: MainThreadMarker,
        frage: &str,
        schaltflaechen: &[Schaltflaeche<'_>],
    ) -> Self {
        assert!(
            schaltflaechen
                .iter()
                .any(|schaltflaeche| schaltflaeche.wirkung == Wirkung::Liegenlassen),
            "das Blatt \"{frage}\" traegt keine Schaltflaeche, die alles liegen laesst"
        );
        let warnung = NSAlert::new(mtm);
        warnung.setMessageText(&NSString::from_str(frage));
        let mut antworten = Vec::with_capacity(schaltflaechen.len());
        for (stelle, schaltflaeche) in schaltflaechen.iter().enumerate() {
            let knopf = warnung.addButtonWithTitle(&NSString::from_str(schaltflaeche.titel));
            // Jede Schaltflaeche bekommt ihre Taste ausdruecklich gesetzt und
            // keine ausgelassen: `NSAlert` gibt der ersten Schaltflaeche von
            // sich aus die Eingabetaste, und ohne das Ueberschreiben traegt sie
            // zwei Blaetter spaeter eine Taste, die niemand ihr zugedacht hat.
            knopf.setKeyEquivalent(schaltflaeche.taste.zeichen());
            knopf.setKeyEquivalentModifierMask(schaltflaeche.taste.zusatztasten());
            antworten.push(antwort_von_stelle(stelle));
        }
        Self {
            warnung,
            antworten,
            abbruchstelle: abbruchstelle(schaltflaechen),
            waechter: None,
        }
    }

    /// Setzt den erlaeuternden Text unter der Frage.
    pub fn erlaeuterung_setzen(&self, text: &str) {
        self.warnung.setInformativeText(&NSString::from_str(text));
    }

    /// Macht das Blatt zur Warnung, mit dem Warnzeichen des Systems.
    ///
    /// Fuer die **laute** Form der Loeschrueckfrage: das Warnzeichen steht,
    /// wenn [`crate::kommandos::loeschwarnung::warngruende`] mindestens einen
    /// Grund liefert, und sonst nicht. Die ruhige Form derselben Rueckfrage
    /// ruft diese Funktion nicht.
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
        let rueckfall = self.abbruchstelle;
        let block = RcBlock::new(move |antwort: NSModalResponse| {
            let _haelt = &waechter;
            // Eine Antwort, die zu keiner angelegten Schaltflaeche gehoert,
            // gilt als die, die alles liegen laesst. Lieber nichts tun als
            // raten, und welche das ist, sagt `abbruchstelle` und nicht die
            // Reihenfolge.
            let stelle = antworten
                .iter()
                .position(|kandidat| *kandidat == antwort)
                .unwrap_or(rueckfall);
            let fuer_alle = warnung
                .suppressionButton()
                .is_some_and(|kaestchen| kaestchen.state() != NSControlStateValueOff);
            fertig(stelle, fuer_alle);
        });
        self.warnung
            .beginSheetModalForWindow_completionHandler(fenster, Some(&block));

        // Der Waechter kann das Blatt erst jetzt beenden: das Fenster, an dem
        // es haengt, steht erst mit diesem Aufruf fest.
        let abbruchcode = antwort_von_stelle(self.abbruchstelle);
        if let Some(waechter) = &self.waechter {
            let blattfenster = self.warnung.window();
            let elternfenster = fenster.retain();
            waechter.antwort_setzen(Box::new(move |bestaetigt| {
                // Bestaetigt heisst die erste Schaltflaeche: einen Waechter
                // haelt nur ein Blatt aus `Blatt::neu`, dessen Reihenfolge die
                // erste als die bestaetigende festlegt. Abgebrochen heisst
                // dieselbe Stelle, die auch der Griff einsetzt; die Frage nach
                // der ungefaehrlichen Schaltflaeche ist einmal beantwortet.
                let antwort = if bestaetigt {
                    NSAlertFirstButtonReturn
                } else {
                    abbruchcode
                };
                elternfenster.endSheet_returnCode(&blattfenster, antwort);
            }));
        }

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

#[cfg(test)]
mod tests {
    use crate::quellbaum::{aufrufstellen, quelldateien};

    use super::*;

    /// Die Tafel von [`abbruchstelle`], Zeile fuer Zeile.
    ///
    /// Ohne AppKit und ohne Hauptfaden: [`Schaltflaeche`] traegt nur eine
    /// Beschriftung, eine Taste und eine Wirkung, und die Regel darueber rechnet
    /// mit nichts sonst. Genau dafuer ist sie von der Rueckrechnung getrennt.
    #[test]
    fn die_tafel_der_liegenlassenden_stelle() {
        let fuehrt_aus = Schaltflaeche::neu("Los", Taste::Eingabe, Wirkung::Ausfuehren);
        let laesst_liegen = Schaltflaeche::neu("Abbrechen", Taste::Escape, Wirkung::Liegenlassen);

        assert_eq!(
            abbruchstelle(&[laesst_liegen, fuehrt_aus]),
            0,
            "die liegenlassende Schaltflaeche steht vorn und wird nicht gefunden"
        );
        assert_eq!(
            abbruchstelle(&[fuehrt_aus, laesst_liegen]),
            1,
            "die liegenlassende Schaltflaeche steht hinten und wird nicht gefunden"
        );
        assert_eq!(
            abbruchstelle(&[fuehrt_aus, fuehrt_aus, laesst_liegen]),
            2,
            "die liegenlassende Schaltflaeche steht an dritter Stelle und wird nicht gefunden"
        );
        assert_eq!(
            abbruchstelle(&[fuehrt_aus, laesst_liegen, laesst_liegen]),
            1,
            "bei zwei liegenlassenden zaehlt die erste"
        );
        assert_eq!(
            abbruchstelle(&[fuehrt_aus, fuehrt_aus]),
            0,
            "ein Blatt ohne liegenlassende Schaltflaeche faellt auf die erste Stelle"
        );
        assert_eq!(abbruchstelle(&[]), 0, "ein Blatt ohne Schaltflaeche");
    }

    /// Die Escape-Taste entscheidet die Rueckfallstelle nicht mehr.
    ///
    /// Die Reihenfolge der Rueckfrage vor dem Loeschen: eine liegenlassende
    /// Schaltflaeche auf der **Eingabetaste** vorn, die ausfuehrende hinten,
    /// keine Escape-Taste im Blatt. Solange die Regel an der Taste hing, fiel
    /// eine unbekannte Antwort hier auf die ausfuehrende Stelle.
    #[test]
    fn ohne_escape_taste_faellt_die_antwort_trotzdem_auf_die_liegenlassende() {
        let schaltflaechen = [
            Schaltflaeche::neu("Abbrechen", Taste::Eingabe, Wirkung::Liegenlassen),
            Schaltflaeche::neu("Räumen", Taste::EingabeMitBefehl, Wirkung::Ausfuehren),
        ];
        assert!(
            schaltflaechen
                .iter()
                .all(|schaltflaeche| schaltflaeche.taste != Taste::Escape),
            "die Probe prueft nicht, was sie pruefen soll: hier traegt eine Schaltflaeche Escape"
        );
        assert_eq!(abbruchstelle(&schaltflaechen), 0);
    }

    /// Jedes Blatt im Baum nennt eine Schaltflaeche, die alles liegen laesst.
    ///
    /// **Gezaehlt wird im Quelltext**, weil die Zusage eine Aussage ueber den
    /// Baum ist: dass es kein Blatt gibt, dessen Schaltflaechen alle etwas
    /// ausfuehren. Am Rueckgabewert einer Funktion ist das nicht abzulesen, und
    /// ein Blatt zu bauen kostet den Hauptfaden, den `libtest` nicht hergibt.
    ///
    /// **Wo die Zaehlung blind ist:** sie prueft je Datei und nicht je
    /// Blatt. Eine Datei mit zwei Blaettern, von denen nur eines seine
    /// liegenlassende Schaltflaeche nennt, kaeme durch. Heute traegt jede Datei
    /// genau ein Blatt.
    ///
    /// **Den Rest deckt die Zusicherung in [`Blatt::mit_schaltflaechen`] ab,
    /// und sie greift seit dem 260818 wirklich** — in jedem Profil, weil sie
    /// ein `assert!` ist, und im Probenbau nachgewiesen von
    /// `ein_blatt_ohne_ungefaehrlichen_ausgang_fliegt_auf`. Bis dahin stand die
    /// Bedingung „sobald das Blatt im Probenbau wirklich aufgeht" hier, und sie
    /// trat nicht ein
    /// (`issues/260817-1419_*_die-zusicherung-gegen-ein-blatt-ohne-ungefaehrlichen-ausgang-greift-in-keinem-bau.md`).
    ///
    /// **Die fuenf Blaetter aus [`Blatt::neu`] sieht diese Zaehlung nicht**, und
    /// sie soll es nicht: ihre Dateien bringen keine Schaltflaechen mit. Deren
    /// gemeinsamer Bauplan ist eigens gemessen, an
    /// `der_bauplan_von_blatt_neu_hat_einen_ungefaehrlichen_ausgang`.
    ///
    /// Beide Nadeln stehen zusammengesetzt da: die Probe liegt in dem Baum, den
    /// sie liest.
    #[test]
    fn jedes_blatt_nennt_seine_liegenlassende_schaltflaeche() {
        let bauer = concat!("mit_schalt", "flaechen");
        let marke = concat!("Wirkung::Liegen", "lassen");
        let mut geprueft = 0;
        for (pfad, inhalt) in quelldateien() {
            if aufrufstellen(&inhalt, bauer) == 0 {
                continue;
            }
            geprueft += 1;
            assert!(
                inhalt.contains(marke),
                "{pfad} baut ein Blatt, nennt aber keine Schaltflaeche, die alles liegen laesst"
            );
        }
        assert!(
            geprueft >= 6,
            "die Probe hat nur {geprueft} Blatt-Bauer gefunden; der Baum traegt mindestens sechs"
        );
    }

    /// Der Bauplan von [`Blatt::neu`] traegt einen ungefaehrlichen Ausgang.
    ///
    /// **Die fuenf Blaetter, die ueber [`Blatt::neu`] entstehen, bringen ihre
    /// Schaltflaechen nicht selbst mit**, also kann die Zaehlprobe darueber sie
    /// nicht sehen: ihre Dateien nennen [`Wirkung::Liegenlassen`] nicht und
    /// muessen es auch nicht. Gemessen wird stattdessen der eine Bauplan, den
    /// alle fuenf teilen, und zwar an derselben reinen Funktion, die
    /// [`Blatt::neu`] einsetzt.
    #[test]
    fn der_bauplan_von_blatt_neu_hat_einen_ungefaehrlichen_ausgang() {
        let schaltflaechen = standardschaltflaechen("Sichern");
        assert_eq!(
            schaltflaechen[abbruchstelle(&schaltflaechen)].wirkung,
            Wirkung::Liegenlassen,
            "der Bauplan von Blatt::neu hat keinen ungefaehrlichen Ausgang"
        );
        assert_eq!(
            abbruchstelle(&schaltflaechen),
            1,
            "die abbrechende Schaltflaeche steht bei Blatt::neu hinten"
        );
    }

    /// Ein Blatt ohne ungefaehrlichen Ausgang fliegt auf.
    ///
    /// **Die Messung der Zusicherung selbst.** Zwei Prosastellen dieser Datei
    /// sagten, [`Blatt::mit_schaltflaechen`] lasse ein Blatt ohne
    /// ungefaehrlichen Ausgang auffliegen, und keine Probe baute je ein Blatt;
    /// die Zeile lief damit in keinem Bau
    /// (`issues/260817-1419_*_die-zusicherung-gegen-ein-blatt-ohne-ungefaehrlichen-ausgang-greift-in-keinem-bau.md`).
    /// Diese Probe ist der Bau, in dem sie laeuft.
    ///
    /// **AppKit wird dabei nicht angefasst, und darauf beruht die Probe.** Die
    /// Zusicherung steht als erste Anweisung des Rumpfes, vor `NSAlert::new`;
    /// die Pruefung stuerzt ab, bevor eine Klasse von AppKit angesprochen ist.
    /// Der Einwand „ein Blatt bauen braucht den Hauptfaden" trifft den Zweig,
    /// den diese Probe gar nicht erreicht. Das `new_unchecked` ist aus
    /// demselben Grund vertretbar wie in `crate::appkit::editor`, und aus einem
    /// staerkeren: der Marker wird weitergereicht und nie eingeloest.
    ///
    /// **Wie sie rot wird, wenn jemand die Zusicherung herausnimmt:** dann
    /// laeuft sie in `NSAlert::new` auf einem Nebenfaden, AppKit wirft eine
    /// Ausnahme von Objective-C, und der Probenlauf bricht mit
    /// „Rust cannot catch foreign exceptions" ab. Ein Fehlschlag ist das, aber
    /// ein harter: der Abbruch nimmt den ganzen Probenlauf mit, statt eine
    /// Zeile zu melden. Wer ihn sieht, sucht hier und nicht bei der Probe, die
    /// zuletzt gemeldet hat.
    #[test]
    #[should_panic(expected = "traegt keine Schaltflaeche, die alles liegen laesst")]
    fn ein_blatt_ohne_ungefaehrlichen_ausgang_fliegt_auf() {
        let ohne_ausgang = [
            Schaltflaeche::neu("Löschen", Taste::Eingabe, Wirkung::Ausfuehren),
            Schaltflaeche::neu("Ersetzen", Taste::EingabeMitBefehl, Wirkung::Ausfuehren),
        ];
        assert!(
            ohne_ausgang
                .iter()
                .all(|schaltflaeche| schaltflaeche.wirkung == Wirkung::Ausfuehren),
            "die Probe prueft nicht, was sie pruefen soll: hier laesst eine Schaltflaeche liegen"
        );
        // SAFETY: Der Marker wird an `mit_schaltflaechen` weitergereicht und
        // dort nie eingeloest: die Zusicherung, die diese Probe messen soll,
        // steht vor dem ersten Aufruf an AppKit und bricht davor ab.
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        let _ = Blatt::mit_schaltflaechen(mtm, "Was soll geschehen?", &ohne_ausgang);
    }
}
