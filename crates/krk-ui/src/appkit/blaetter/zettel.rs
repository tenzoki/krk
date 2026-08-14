//! Das zehnte Blatt: der Notizzettel der Runde 9, zwei Zettel als anklickbare
//! Tabs ueber einer bearbeitbaren Textflaeche.
//!
//! ```text
//!  ┌ Notizzettel ───────────────────────────┐
//!  │  [ Zettel 1 │ Zettel 2 ]               │  NSSegmentedControl, ein Klick
//!  │  ┌──────────────────────────────────┐  │  wechselt und sichert
//!  │  │ NSTextView, bearbeitbar          │  │
//!  │  │ keine Nummernspalte, keine       │  │  NSScrollView
//!  │  │ Hervorhebung, keine Suche        │  │
//!  │  └──────────────────────────────────┘  │
//!  └                              [ Fertig ] ┘  Esc schliesst ebenfalls
//! ```
//!
//! # Warum die Textflaeche in `ersthelfer_gehoert_appkit` **nicht** angemeldet wird
//!
//! **`CLAUDE.md` sagt fuer eine zweite bedienbare Textflaeche das Gegenteil, und
//! fuer diese Flaeche zeigt der Satz in die falsche Richtung.** Dort steht: „Wer
//! eine zweite bedienbare Textflaeche baut, meldet sie dort an, sonst gehoeren
//! ihre Tasten AppKit und kein Befehl von KRK wirkt darin." Genau das ist hier
//! **erwuenscht**, und zwar wegen `Esc`.
//!
//! Die Kette ist diese: `kommandos::zulaessigkeit::zulaessig` laesst bei
//! stehendem Blatt allein durch, was
//! `kommandos::operationen::waehrend_blatt_erlaubt` nennt, und das ist
//! `Kommando::Abbrechen` — **aber** nur, solange der Ersthelfer nicht AppKit
//! gehoert. Bleibt die Flaeche des Zettels unangemeldet, gehoert ihr Ersthelfer
//! AppKit, `Abbrechen` ist damit unzulaessig, der Tastendruck laeuft unveraendert
//! weiter, die Flaeche macht daraus `cancelOperation:`, und der
//! [`Zettelwaechter`] schliesst das Blatt. Eine Anmeldung kehrte beides um:
//! `Abbrechen` wuerde zulaessig, KRK schluckte `Esc` fuer den Abbruch einer
//! Dateioperation, und der Zettel bliebe stehen.
//!
//! Der Editor braucht die Anmeldung aus dem umgekehrten Grund: er **will** KRKs
//! Befehle mit dem Fokus in sich selbst, denn er ist ein Bereich der
//! Fensterzeile und kein Blatt. Der Zettel ist ein Blatt und braucht von KRKs
//! Befehlen keinen.
//!
//! **Daran haengt auch der Rangrueckgabe nach einem Tabklick.** Haelt der
//! Tabschalter den Ersthelferrang — moeglich unter eingeschalteter
//! vollstaendiger Tastaturbedienung —, gehoert der Ersthelfer nicht mehr AppKit,
//! `Abbrechen` wird zulaessig, KRK schluckt `Esc`, und der Zettel bleibt stehen.
//! [`Zettelwaechter::tab_gewechselt`] gibt den Rang deshalb ausdruecklich an die
//! Textflaeche zurueck; C2 sagt es als eigenes Abnahmekriterium zu.
//!
//! # Warum der Waechter ein eigener Typ ist und kein Schalter am bestehenden
//!
//! Der [`Eingabewaechter`](super::Eingabewaechter) der neun uebrigen Blaetter
//! faengt **zwei** Tasten ab, die Eingabe- und die Escape-Taste, und beendet mit
//! beiden das Blatt. Fuer den Zettel gilt nur die Haelfte davon: `Esc` schliesst,
//! die Eingabetaste setzt eine Zeile (C3). Ein Schalter am bestehenden Waechter
//! waeren zwei Wahrheiten darueber, was die Eingabetaste in einem Blatt tut —
//! genau das, was dessen Modulkopf ausdruecklich vermeidet.
//!
//! **Dazu kommt, dass es zwei verschiedene Protokolle sind.** Der bestehende
//! Waechter ist ein `NSControlTextEditingDelegate` und beantwortet
//! `control:textView:doCommandBySelector:`; diese Methode ruft der Feldeditor
//! eines `NSControl`. Eine freistehende `NSTextView` hat keinen Feldeditor und
//! ruft `textView:doCommandBySelector:` an ihrem `NSTextViewDelegate`. Zwei
//! Signaturen, zwei Protokolle; ein gemeinsamer Typ muesste beide tragen und die
//! Regel dann doch nach Herkunft aufteilen.
//!
//! **Die Entscheidung selbst steht als reine Funktion**, [`uebernimmt`], und
//! nicht im Rumpf der Objective-C-Methode. Damit ist sie ohne Fenster und ohne
//! Hauptfaden pruefbar: die Probe faehrt sie an einem `Sel` und haelt beide
//! Haelften fest. Der Baum bringt so keine weitere Behauptung des Hauptfadens
//! hinzu (`issues/260810-1001_*`, Datensatz `decisions/260810-1044_*`).
//!
//! # Was der Zettel nicht hat
//!
//! Keine Nummernspalte, keine Syntaxhervorhebung, keine Suche, kein Ersetzen,
//! keine Textmarken (C3). Der Zettel ist kein zweiter Editor, sondern eine
//! Flaeche fuer das, was zwischendurch anfaellt; jede dieser Faehigkeiten waere
//! ein Bedienelement und ein Abnahmekriterium mehr. Eine Zaehlprobe unter
//! `mod tests` haelt fest, dass diese Datei die drei Namen nicht nennt.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSSegmentedControl`, `NSScrollView`, `NSTextView`, `NSView`, `NSWindow`,
//! `NSObject`, `NSString`, `NSArray` und `NSUndoManager` stehen seit macOS 10.0
//! zur Verfuegung, ebenso das Protokoll `NSTextViewDelegate` samt der hier
//! beantworteten Methode `textView:doCommandBySelector:`, die Aufzaehlung
//! `NSSegmentSwitchTracking` und die Zugriffe `selectedSegment`,
//! `setSelectedSegment:`, `setDocumentView:`, `setHasVerticalScroller:`,
//! `setAutohidesScrollers:`, `setString:`, `setEditable:`, `setSelectable:`,
//! `setAllowsUndo:`, `makeFirstResponder:`, `undoManager` und
//! `removeAllActions` (`NSSegmentedControl.h`, `NSScrollView.h`,
//! `NSTextView.h`, `NSResponder.h`, `NSWindow.h`, `NSUndoManager.h`).
//!
//! **Zwei Beruehrungen sind juenger als 10.0**, und beide liegen weit unter dem
//! Zielsystem: `setSegmentStyle:` seit 10.5 und
//! `segmentedControlWithLabels:trackingMode:target:action:` seit 10.12
//! (`NSSegmentedControl.h`).
//!
//! Was der Zettel an Einstellungen seiner Textflaeche setzt, geht durch
//! [`super::super::textautomatik`]; die Untergrenzen dazu nennt dessen Modulkopf
//! und nicht dieser. Wie das Blatt aufgeht und wieder zugeht, geht durch
//! [`super::Blatt`]; auch dafuer gilt der Kopf von [`super`].
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::runtime::{ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSScrollView, NSSegmentStyle, NSSegmentSwitchTracking,
    NSSegmentedControl, NSTextDelegate, NSTextView, NSTextViewDelegate, NSView, NSWindow,
};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString,
};

use krk_core::ablage::pfade::Zettel;

use super::super::textautomatik;
use super::{Blatt, Blattgriff, Schaltflaeche, Taste};

/// Die Breite der Beigabe in Punkten. Sie bestimmt zugleich die Breite des
/// Blattes: `NSAlert` waechst mit seiner Beigabe.
const BREITE: f64 = 460.0;

/// Die Hoehe der Bildlaufansicht mit der Textflaeche.
const TEXTHOEHE: f64 = 300.0;

/// Die Hoehe des Tabschalters.
const SCHALTERHOEHE: f64 = 24.0;

/// Der senkrechte Abstand zwischen Tabschalter und Textflaeche.
const ABSTAND: f64 = 8.0;

/// Die Beschriftungen der beiden Tabs, in der Reihenfolge von [`Zettel::ALLE`].
///
/// **Die schlichte Nummerierung, wie der Spec sie als Vorbelegung nennt.**
/// Benannte Zettel waeren eine eigene Faehigkeit mit eigener Eingabe und stehen
/// ausdruecklich ausserhalb dieser Runde.
const BESCHRIFTUNGEN: [&str; Zettel::ALLE.len()] = ["Zettel 1", "Zettel 2"];

/// Was beim Klick auf einen Tab zu tun ist.
///
/// Zurueck kommt der Text des Ziels, oder `None`, wenn der Klick dem bereits
/// offenen Tab galt.
type Tabwechselweg = Box<dyn Fn(Zettel) -> Option<String>>;

/// Was der Waechter haelt.
pub struct ZettelwaechterIvars {
    /// Die Textflaeche des Zettels.
    ///
    /// Stark gehalten, und die Gegenrichtung ist schwach: eine `NSTextView`
    /// haelt ihren Delegierten schwach, ein Ring entsteht deshalb nicht.
    flaeche: Retained<NSTextView>,
    /// Der Tabschalter, damit ein abgewiesener Wechsel die Anzeige nicht
    /// stehen laesst, wo sie nicht hingehoert.
    schalter: Retained<NSSegmentedControl>,
    /// Was beim Klick auf einen Tab zu tun ist: den verlassenen Zettel sichern
    /// und den Text des Ziels liefern. `None` heisst „nichts zu tun" — der
    /// Klick galt dem bereits offenen Tab.
    ///
    /// Wahlfrei, weil der Waechter vor dem Rueckruf zur Welt kommt: er ist das
    /// Ziel des Tabschalters, und der wird gebaut, bevor der Aufrufer seine
    /// Antwortwege setzt.
    tabklick: RefCell<Option<Tabwechselweg>>,
    /// Wie das Blatt zu schliessen ist.
    ///
    /// Wahlfrei aus demselben Grund wie beim
    /// [`Eingabewaechter`](super::Eingabewaechter): das Fenster, an dem das
    /// Blatt haengt, steht erst mit [`Blatt::zeigen_mit_wahl`] fest.
    schliessen: RefCell<Option<Box<dyn Fn()>>>,
}

define_class!(
    /// Der Delegierte der Textflaeche des Zettels und das Ziel seines
    /// Tabschalters.
    ///
    /// **Ein Empfaenger und nicht zwei.** Er beantwortet die eine Taste, die
    /// das Blatt schliesst, und den einen Klick, der den Zettel wechselt;
    /// beides gehoert zu diesem Blatt und zu keinem anderen. Dieselbe Form wie
    /// die `Belegungsquelle` der Belegungsansicht, die Datenquelle, Delegierter
    /// und Ziel ihrer Schaltflaechen in einem ist.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = ZettelwaechterIvars]
    pub struct Zettelwaechter;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Zettelwaechter {}

    // SAFETY: `NSTextDelegate` stellt keine Bedingungen. Er steht hier, weil
    // `NSTextViewDelegate` ihn voraussetzt; beantwortet wird aus ihm nichts —
    // der Zettel braucht keine Aenderungsmeldung, sein Stand wird an den vier
    // Sicherungsmomenten aus der Flaeche gelesen.
    unsafe impl NSTextDelegate for Zettelwaechter {}

    // SAFETY: `NSTextViewDelegate` hat nur wahlfreie Methoden.
    unsafe impl NSTextViewDelegate for Zettelwaechter {
        /// Die Textflaeche fragt, ob jemand anders diesen Befehl uebernimmt.
        ///
        /// **Die halbe Regel des [`Eingabewaechter`](super::Eingabewaechter):**
        /// `cancelOperation:` schliesst das Blatt, `insertNewline:` bleibt bei
        /// der Flaeche und setzt eine Zeile. Was uebernommen wird, entscheidet
        /// [`uebernimmt`] und nicht dieser Rumpf.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(textView:doCommandBySelector:))]
        fn befehl_umleiten(&self, _sicht: &NSTextView, befehl: Sel) -> objc2::runtime::Bool {
            if !uebernimmt(befehl) {
                return objc2::runtime::Bool::NO;
            }
            // Die Ausleihe endet vor dem Aufruf: das Schliessen laeuft durch
            // AppKit, und AppKit kann dabei erneut hierher zurueckrufen.
            let schliessen = self.ivars().schliessen.borrow_mut().take();
            if let Some(schliessen) = schliessen {
                schliessen();
            }
            objc2::runtime::Bool::YES
        }
    }

    impl Zettelwaechter {
        /// Der Nutzer hat auf einen Tab geklickt.
        ///
        /// Drei Schritte, und die Reihenfolge ist bindend: der Rueckruf sichert
        /// den verlassenen Zettel und liefert den Text des Ziels, danach steht
        /// dieser Text in der Flaeche, und danach hat die Flaeche den
        /// Ersthelferrang zurueck.
        ///
        /// **Der Rang muss zurueck**, und der Grund steht im Modulkopf: haelt
        /// der Tabschalter ihn, wird `Kommando::Abbrechen` zulaessig, KRK
        /// schluckt `Esc`, und der Zettel liesse sich mit der Tastatur nicht
        /// mehr schliessen.
        ///
        /// **Der Rueckgaengigverlauf faellt beim Wechsel.** `setString:`
        /// schreibt an der Rueckgaengigverwaltung vorbei; ein stehen
        /// gebliebener Stapel zeigte danach auf den Text des **anderen**
        /// Zettels, und ein `cmd+z` schriebe ihn in den offenen hinein. Es ist
        /// derselbe Grund, aus dem `Editorbereich::stand_einsetzen` den Verlauf
        /// beim Dateiwechsel fallen laesst.
        // SAFETY: Die Signatur ist die einer gewoehnlichen Aktion: ein
        // Argument, der Absender.
        #[unsafe(method(tabGewechselt:))]
        fn tab_gewechselt(&self, _absender: Option<&NSSegmentedControl>) {
            let ivars = self.ivars();
            let Some(ziel) = zettel_an_stelle(ivars.schalter.selectedSegment()) else {
                return;
            };
            let text = {
                // Die Ausleihe laeuft durch den Aufruf, und das traegt: der Weg
                // ist gesetzt, solange das Blatt steht, und veraendert wird er
                // allein beim Aufbau in `zeigen`. Der Rueckruf geht in den
                // Anwendungsdelegierten und von dort in die Ablage; er kommt
                // nicht hierher zurueck.
                let tabklick = ivars.tabklick.borrow();
                let Some(tabklick) = tabklick.as_ref() else {
                    return;
                };
                tabklick(ziel)
            };
            // `None` heisst: der Klick galt dem offenen Tab. Dann bleibt der
            // Text stehen, und mit ihm der Rueckgaengigverlauf und die Auswahl
            // des Nutzers — ein Neusetzen desselben Textes verloere beides.
            if let Some(text) = text {
                ivars.flaeche.setString(&NSString::from_str(&text));
                if let Some(verwalter) = ivars.flaeche.undoManager() {
                    verwalter.removeAllActions();
                }
            }
            // Der Rang geht in jedem Fall zurueck: geklickt hat der Nutzer auf
            // den Schalter, und der haelt ihn sonst.
            if let Some(fenster) = ivars.flaeche.window() {
                fenster.makeFirstResponder(Some(&ivars.flaeche));
            }
        }
    }
);

impl Zettelwaechter {
    /// Einen Waechter zu dieser Flaeche und diesem Schalter, ohne Antwortwege.
    fn neu(
        mtm: MainThreadMarker,
        flaeche: Retained<NSTextView>,
        schalter: Retained<NSSegmentedControl>,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(ZettelwaechterIvars {
            flaeche,
            schalter,
            tabklick: RefCell::new(None),
            schliessen: RefCell::new(None),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }
}

/// Ob der Waechter diesen Befehl der Textflaeche abnimmt.
///
/// **Genau einer, und der zweite ausdruecklich nicht.** `cancelOperation:` ist
/// die Escape-Taste und schliesst das Blatt; `insertNewline:` ist die
/// Eingabetaste und bleibt bei der Flaeche, weil C3 zusagt, dass sie eine neue
/// Zeile setzt. Der [`Eingabewaechter`](super::Eingabewaechter) der neun
/// uebrigen Blaetter nimmt beide; der Unterschied ist die ganze Begruendung
/// dafuer, dass der Zettel einen eigenen Waechter hat.
///
/// Alles Uebrige, darunter jede Bewegung der Schreibmarke, bleibt bei der
/// Flaeche.
fn uebernimmt(befehl: Sel) -> bool {
    befehl == sel!(cancelOperation:)
}

/// Welcher Zettel an dieser Stelle des Tabschalters steht.
///
/// `None` fuer eine Stelle, die es nicht gibt — `NSSegmentedControl` liefert
/// `-1`, solange keiner ausgewaehlt ist. Lieber nichts tun als raten.
fn zettel_an_stelle(stelle: isize) -> Option<Zettel> {
    usize::try_from(stelle)
        .ok()
        .and_then(|stelle| Zettel::ALLE.get(stelle).copied())
}

/// Zeigt den Notizzettel als Blatt am Fenster.
///
/// Kehrt sofort zurueck. `tabklick` laeuft auf dem Hauptfaden, sobald der Nutzer
/// einen Tab anklickt: es sichert den verlassenen Zettel und liefert den Text
/// des Ziels, oder `None`, wenn der Klick dem bereits offenen Tab galt.
/// `abschluss` laeuft genau einmal, wenn das Blatt zugeht —
/// gleich ob ueber die Escape-Taste, ueber die Schaltflaeche oder ueber den
/// zurueckgegebenen [`Blattgriff`]. **Alle drei Wege muenden in denselben
/// Abschlussblock von AppKit**, und deshalb haengt das Sichern dort und nicht am
/// Waechter.
///
/// Zurueck kommen die Textflaeche und der Griff. Die Flaeche geht an den
/// Aufrufer, weil er ihren Stand an den Sicherungsmomenten braucht, die
/// ausserhalb des Blattes liegen: `shift+cmd+w` und das Beenden treffen den
/// Zettel, waehrend er steht.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    offener: Zettel,
    text: &str,
    tabklick: impl Fn(Zettel) -> Option<String> + 'static,
    abschluss: impl Fn() + 'static,
) -> (Retained<NSTextView>, Blattgriff) {
    let (bildlauf, flaeche) = textflaeche_bauen(mtm, textrahmen());
    flaeche.setString(&NSString::from_str(text));

    let beschriftungen: Vec<Retained<NSString>> = BESCHRIFTUNGEN
        .iter()
        .map(|titel| NSString::from_str(titel))
        .collect();
    let beschriftungen = NSArray::from_retained_slice(&beschriftungen);
    // Ziel und Aktion bleiben hier leer und werden gesetzt, sobald der Waechter
    // steht: er haelt den Schalter, und der Schalter haelt sein Ziel schwach.
    // SAFETY: Der Erzeuger verlangt ein Ziel vom richtigen Typ und einen
    // gueltigen Selektor; `None` ist fuer beide der zulaessige Wert.
    let schalter = unsafe {
        NSSegmentedControl::segmentedControlWithLabels_trackingMode_target_action(
            &beschriftungen,
            NSSegmentSwitchTracking::SelectOne,
            None,
            None,
            mtm,
        )
    };
    schalter.setSegmentStyle(NSSegmentStyle::Automatic);
    schalter.setSelectedSegment(
        isize::try_from(offener.index())
            .expect("die Stelle eines von zwei Zetteln passt in ein isize"),
    );
    schalter.setFrame(NSRect::new(
        NSPoint::new(0.0, TEXTHOEHE + ABSTAND),
        NSSize::new(BREITE, SCHALTERHOEHE),
    ));

    let waechter = Zettelwaechter::neu(mtm, flaeche.clone(), schalter.clone());
    // Der Waechter beantwortet `NSTextViewDelegate`, das er oben implementiert.
    // Ueber die Lebensdauer verlangt die Bindung nichts: die Flaeche haelt den
    // Delegierten schwach, und der Abschlussblock haelt ihn stark, solange das
    // Blatt steht.
    flaeche.setDelegate(Some(ProtocolObject::from_ref(&*waechter)));
    // SAFETY: Der Waechter beantwortet `tabGewechselt:` mit der ueblichen
    // Aktionssignatur, und `sel!` liefert einen gueltigen Selektor.
    unsafe {
        schalter.setTarget(Some(&*waechter));
        schalter.setAction(Some(sel!(tabGewechselt:)));
    }
    *waechter.ivars().tabklick.borrow_mut() = Some(Box::new(tabklick));

    let beigabe = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(
            NSPoint::ZERO,
            NSSize::new(BREITE, TEXTHOEHE + ABSTAND + SCHALTERHOEHE),
        ),
    );
    beigabe.addSubview(&schalter);
    beigabe.addSubview(&bildlauf);

    // Eine einzige Schaltflaeche, und sie traegt die Escape-Taste und nicht die
    // Eingabetaste. Die Eingabetaste an einer Schaltflaeche liefe ueber
    // `performKeyEquivalent:` und schloesse das Blatt, bevor die Flaeche sie
    // saehe; C3 sagt zu, dass sie eine neue Zeile setzt.
    let blatt = Blatt::mit_schaltflaechen(
        mtm,
        "Notizzettel",
        &[Schaltflaeche::neu("Fertig", Taste::Escape)],
    );
    // Der Satz nennt die Taste und die Zusage aus C4 und zaehlt die vier
    // Sicherungsmomente **nicht** auf: welcher Weg heraus wann schreibt, ist
    // eine Frage an den Code und keine, die der Nutzer beantworten muss.
    blatt.erlaeuterung_setzen(
        "Esc schließt den Zettel. Was du tippst, wird gesichert, ohne dass du etwas tun musst.",
    );
    blatt.beigabe_setzen(&beigabe);
    blatt.ersthelfer_setzen(&flaeche);

    let waechter_im_block = waechter.clone();
    let griff = blatt.zeigen_mit_wahl(fenster, move |_stelle, _fuer_alle| {
        let _haelt = &waechter_im_block;
        abschluss();
    });

    // Der Waechter kann das Blatt erst jetzt schliessen: das Fenster, an dem es
    // haengt, steht erst mit dem Zeigen fest. Derselbe Weg wie beim
    // `Eingabewaechter`, und er endet im selben Abschlussblock.
    *waechter.ivars().schliessen.borrow_mut() = Some(Box::new(griff.abbruchweg()));

    (flaeche, griff)
}

/// Der Rahmen, in dem Bildlaufansicht und Textflaeche gebaut werden.
fn textrahmen() -> NSRect {
    NSRect::new(NSPoint::ZERO, NSSize::new(BREITE, TEXTHOEHE))
}

/// Baut die Textflaeche des Zettels: eine bearbeitbare `NSTextView` in einer
/// Bildlaufansicht.
///
/// **Nicht dieselbe Funktion wie im Editor, und der Unterschied ist die Liste
/// dessen, was hier fehlt**: keine Nummernspalte, keine feste
/// Schreibmaschinenschrift, kein Zugriff auf den Layoutverwalter. Gemeinsam ist
/// beiden Flaechen allein, was [`textautomatik::automatiken_abschalten`] setzt,
/// und genau das steht deshalb dort und hier nicht.
///
/// Rueckgaengig steht **an**, ueber die Rueckgaengigverwaltung von AppKit und
/// ohne Budget in Bytes: der Zettel haelt, was der Nutzer hineinschreibt, und
/// nicht eine fremde Datei. Der Verlauf endet mit dem Blatt (C3).
fn textflaeche_bauen(
    mtm: MainThreadMarker,
    rahmen: NSRect,
) -> (Retained<NSScrollView>, Retained<NSTextView>) {
    let bildlauf = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), rahmen);
    bildlauf.setHasVerticalScroller(true);
    bildlauf.setAutohidesScrollers(true);

    let flaeche = NSTextView::initWithFrame(NSTextView::alloc(mtm), rahmen);
    flaeche.setEditable(true);
    flaeche.setSelectable(true);
    textautomatik::automatiken_abschalten(&flaeche);
    flaeche.setAllowsUndo(true);
    flaeche.setVerticallyResizable(true);
    flaeche.setHorizontallyResizable(false);
    flaeche.setMinSize(NSSize::ZERO);
    flaeche.setMaxSize(NSSize::new(f64::MAX, f64::MAX));
    flaeche.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
    bildlauf.setDocumentView(Some(&flaeche));
    (bildlauf, flaeche)
}

#[cfg(test)]
mod tests {
    use objc2::sel;

    use super::{uebernimmt, zettel_an_stelle};
    use crate::quellbaum::quelldateien;
    use krk_core::ablage::pfade::Zettel;

    /// Der Waechter faengt die Escape-Taste ab und die Eingabetaste **nicht**
    /// (C3).
    ///
    /// Beide Haelften stehen hier, weil beide zugesagt sind: ohne die erste
    /// liesse sich der Zettel mit der Tastatur nicht schliessen, mit der zweiten
    /// schloesse ihn jede neue Zeile.
    ///
    /// **Ohne Fenster und ohne Hauptfaden.** Gefahren wird die reine Funktion
    /// und nicht die Objective-C-Methode; ein `Sel` entsteht ohne AppKit. Der
    /// Zettel bringt damit keine weitere Behauptung des Hauptfadens in diesen
    /// Baum. Was die Probe nicht sagt: ob AppKit die Methode ueberhaupt ruft —
    /// das entscheidet der Lauf am Buendel.
    #[test]
    fn der_waechter_nimmt_die_escape_taste_und_nicht_die_eingabetaste() {
        assert!(
            uebernimmt(sel!(cancelOperation:)),
            "die Escape-Taste muss das Blatt schließen"
        );
        assert!(
            !uebernimmt(sel!(insertNewline:)),
            "die Eingabetaste muss eine Zeile setzen und nicht das Blatt schließen"
        );
        assert!(
            !uebernimmt(sel!(moveLeft:)),
            "jede Bewegung der Schreibmarke bleibt bei der Fläche"
        );
    }

    /// Die zwei Stellen des Tabschalters sind die zwei Zettel, und eine dritte
    /// gibt es nicht.
    ///
    /// `NSSegmentedControl` liefert `-1`, solange keine Stelle ausgewählt ist;
    /// daraus wird kein Zettel und kein Wechsel.
    #[test]
    fn nur_die_zwei_stellen_des_schalters_sind_zettel() {
        assert_eq!(zettel_an_stelle(0), Some(Zettel::Erster));
        assert_eq!(zettel_an_stelle(1), Some(Zettel::Zweiter));
        assert_eq!(zettel_an_stelle(2), None);
        assert_eq!(zettel_an_stelle(-1), None);
    }

    /// Es gibt genau so viele Beschriftungen wie Zettel.
    ///
    /// Der Uebersetzer haelt die Laenge des Feldes; diese Probe haelt fest, dass
    /// keine der beiden leer ist — ein leerer Tab waere mit der Maus nicht zu
    /// treffen.
    #[test]
    fn jeder_zettel_traegt_eine_beschriftung() {
        for beschriftung in super::BESCHRIFTUNGEN {
            assert!(!beschriftung.is_empty(), "ein Tab ohne Beschriftung");
        }
    }

    /// Im Zettel gibt es keine Suche, kein Ersetzen, keine Zeilennummern und
    /// keine Syntaxhervorhebung (C3).
    ///
    /// **Gezaehlt wird am Quelltext dieser Datei**, wie in [`crate::quellbaum`]
    /// beschrieben; die Nadeln stehen zusammengesetzt da, weil die Probe in dem
    /// Baum liegt, den sie liest.
    ///
    /// Was die Probe nicht sieht: eine dieser Faehigkeiten, die ueber einen
    /// anderen Namen hereinkaeme — etwa ueber eine Hilfsfunktion in einer
    /// dritten Datei, die ihrerseits die Nummernspalte einhaengt. Der Kopf von
    /// `crates/krk-core/tests/baum.rs` sagt, warum keine Nadel restlos dicht
    /// ist.
    #[test]
    // Der Name meidet die vier Nadeln: die Probe liest die Datei, in der sie
    // steht, und ein Name mit einer davon faende sich selbst.
    fn im_zettel_steht_keine_der_vier_ausgeschlossenen_faehigkeiten() {
        let nadeln = [
            concat!("Nummern", "spalte"),
            concat!("hervor", "hebung"),
            concat!("such", "e"),
            concat!("text", "merkmale"),
        ];
        let (_, inhalt) = quelldateien()
            .into_iter()
            .find(|(name, _)| name == "krk-ui/src/appkit/blaetter/zettel.rs")
            .expect("die Datei des Zettels steht im Quellbaum");
        // Gelesen werden Codezeilen und keine Prosa: der Modulkopf zaehlt die
        // ausgeschlossenen Faehigkeiten beim Namen auf, und genau das soll er.
        let code: String = inhalt
            .lines()
            .filter(|zeile| !zeile.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for nadel in nadeln {
            assert!(
                !code.contains(nadel),
                "der Zettel ruft »{nadel}«; C3 schließt die Fähigkeit aus"
            );
        }
    }
}
