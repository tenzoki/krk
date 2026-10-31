//! Die Flaeche der Quicknote: eine Textflaeche mit eigenem
//! Rueckgaengigverwalter (Plan
//! `260927-0110_*_plan-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md`),
//! deren Text seit dem Plan
//! `261031-1034-plan-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md`
//! in `scratch.txt` gesichert wird. **Diese Datei schreibt nichts auf die
//! Platte**: sie meldet jede Aenderung dem Editorbereich, und der gibt den
//! Auftrag an `crate::quicknotesicherung`, den einen Schreiber.
//!
//! # Warum eine eigene Flaeche
//!
//! **Die Quicknote nutzt nicht die Textflaeche des Editors mit anderem
//! Inhalt** (Entscheidung 1 des Plans). Sonst muesste jeder Wechsel den Stand
//! der Datei aus der Flaeche und zurueck kopieren, und Rueckgaengigstapel,
//! Schreibmarke und Einfaerbung liefen dabei durch dieselbe Flaeche. Die eigene
//! Flaeche liegt deckungsgleich ueber der des Editors, der
//! `crate::appkit::editor::Editorbereich` blendet sie mit `setHidden:` ein und
//! aus, und die Datei darunter bleibt unberuehrt.
//!
//! **Der Puffer ist der Textspeicher dieser Flaeche und keine Zeichenkette
//! daneben**, dieselbe Regel „ein Textspeicher und kein zweiter Textbestand",
//! die der Modulkopf des Editors aufstellt. Die Flaeche entsteht beim ersten F10
//! und lebt danach so lange wie der Editorbereich, also so lange wie der
//! Prozess.
//!
//! # Wie eine Aenderung zur Sicherung kommt
//!
//! **Jede Aenderung des Textes meldet sich ueber `textDidChange:`** am
//! Delegierten, und der ruft den Aenderungsmelder, den der Editorbereich beim
//! Bau setzt: Tippen, Einfuegen, Ziehen, Leeren, Strip, Format und auch
//! Rueckgaengig und Wiederholen, denn die Textflaeche meldet jede Handlung
//! ihres Verwalters ueber dieselbe Benachrichtigung, solange ihr Fenster das
//! Schluesselfenster ist, und anders erreicht `cmd+z` sie nicht (gemessen am
//! 261031, `messungen/261031-1300-quicknote-textdidchange.txt`; ohne
//! Schluesselfenster meldet ein Rueckgaengig sich nicht, und das ist die Lage
//! unter `libtest`). **Zwei Wege schreiben an
//! ihr vorbei**, beide ueber `setString:`: [`Quicknote::nach_kopie_leeren`],
//! das deshalb den Melder selbst ruft, und
//! [`Quicknote::einsetzen_ohne_rueckgaengig`], das den gelesenen Inhalt von
//! `scratch.txt` einsetzt und gerade **nicht** melden soll, denn was von der
//! Platte kommt, muss nicht zurueck auf sie.
//!
//! **Bis der Inhalt gelesen ist, ist die Flaeche nicht bearbeitbar**
//! ([`Quicknote::bearbeitbar_setzen`]): eine nicht bearbeitbare `NSTextView`
//! nimmt weder Tippen noch `shouldChangeTextInRange:replacementString:` an,
//! also entsteht vor dem Einlesen keine Aenderung, die den Inhalt der Datei
//! ueberschreiben koennte.
//!
//! # Warum der Verwalter ueber den Delegierten kommt
//!
//! `undo:` beantwortet in der Antwortkette allein `NSWindow`, und es nimmt dabei
//! den Verwalter des Ersthelfers (gemessen am 260810, Doc-Kommentar von
//! `rueckgaengigstapel_leeren` im Editor). Eine gewoehnliche `NSTextView` fragt
//! fuer ihren Verwalter zuerst ihren Delegierten ueber
//! `undoManagerForTextView:`; diese Klasse beantwortet die Frage mit einem
//! eigenen Verwalter, und das Tippen der Quicknote landet damit nicht im Stapel
//! der Datei darunter. **Eine Unterklasse braucht es dafuer nicht**: die
//! Ausnahme davon ist der Feldeditor, der `undo:` selbst beantworten muss
//! (`Zelleneditor` in `super::eintragsansicht`), und die Quicknote hat keinen.
//!
//! **Auch der leere eigene Stapel greift nicht zur Datei durch**, gemessen am
//! 260927 (`messungen/260927-0232-quicknote-rueckgaengig.txt`): direkt nach
//! dem Oeffnen, nach `setString:` plus `removeAllActions` (der Weg von
//! [`Quicknote::nach_kopie_leeren`]) und nach dem Zuruecknehmen der letzten
//! Tipp-Handlung sind „Rueckgaengig“ und, bei leerem Wiederholstapel,
//! „Wiederholen“ grau, und das Tastenaequivalent loest nichts aus. An der
//! Menuepruefung vorbei nimmt `NSWindow.undo:` den Verwalter des Ersthelfers,
//! also diesen hier, und tut nichts; der Verwalter des Fensters bleibt in
//! jedem Zustand unberuehrt. Beim Feldeditor der Messung vom 260926 war es
//! umgekehrt, und das ist der Grund, aus dem der `Zelleneditor` `undo:` selbst
//! beantwortet und diese Flaeche nicht.
//!
//! # Die fuenf Schaltflaechen
//!
//! Oben in der Rolle stehen Leeren, Schliessen und Kopieren, von links nach
//! rechts (Entscheidung 2 des Plans), und rechts daneben Strip und Format
//! (Entscheidung 13 des Plans der Sicherung), beschriftet aus der
//! Sprachtabelle; „Strip“ und „Format“ sind Namen und in jeder Sprache
//! dieselben. Strip und Format wirken auf die Auswahl, sonst auf den ganzen
//! Text, und gehen ueber [`Quicknote::ersetzen`], denselben einen
//! Ersetzungsweg wie Leeren; was sie rechnen, steht in
//! `krk_core::text::glaetten`.
//! **Ein Klick ist ein Kommando** und
//! geht ueber den Knopfmelder, den der Editorbereich beim Bau setzt, durch
//! dieselbe Zulaessigkeit wie die Taste; die Schaltflaechen nehmen den
//! Ersthelferrang nicht an, und ein Klick holt zuerst den Fokus in die
//! Textflaeche, damit die Zulaessigkeit den Fokus im Editor sieht. Keine
//! Aktion heisst `copy:`, `cut:` oder `paste:`: die Zwischenablage erreicht
//! diese Datei nicht, das Kopieren geht beim Anwendungsdelegierten ueber
//! `super::zwischenablage::text_schreiben`.
//!
//! # Die Grenze des Puffers
//!
//! **Jede Aenderung des Textes geht durch
//! `textView:shouldChangeTextInRange:replacementString:`**, Tippen,
//! Einfuegen, Ziehen und Dienste gleichermassen; dort fragt diese Klasse
//! `crate::quicknote::aenderung_passt`, ob das Ergebnis innerhalb von
//! `QUICKNOTEGRENZE` bleibt. Liegt es darueber, unterbleibt die Aenderung
//! ganz, und der Grenzmelder, den der Editorbereich beim Bau setzt, sagt es
//! in der Statuszeile. Die Schranke vorab ist die UTF-16-Laenge mal drei, die
//! Obergrenze, die `maximumLengthOfBytesUsingEncoding:` fuer UTF-8 nennt,
//! hier mit `saturating_mul` gerechnet statt erfragt, weil die Methode einen
//! Ueberlauf als 0 meldete; die genaue Laenge rechnet
//! `lengthOfBytesUsingEncoding:` erst, wenn die Schranke nicht passt.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSView`, `NSScrollView`, `NSTextView`, `NSColor`, `NSObject` und
//! `NSUndoManager` stehen seit macOS 10.0, ebenso die Protokolle
//! `NSObjectProtocol`, `NSTextDelegate` und `NSTextViewDelegate` samt
//! `undoManagerForTextView:` (`NSTextView.h:642`) und die Methoden
//! `initWithFrame:`, `setEditable:`, `setSelectable:`, `setRichText:`,
//! `setImportsGraphics:` (`NSTextView.h:417`), `setAllowsUndo:`,
//! `setTextColor:`, `setBackgroundColor:`, `setDelegate:`, `setHidden:`,
//! `addSubview:` und `setDocumentView:` sowie die Farben `textColor` und
//! `textBackgroundColor` (`NSColor.h:217-218`). Keine davon traegt im SDK eine
//! eigene Verfuegbarkeitsangabe. Ebenso seit 10.0: `NSString`, `NSRange`, die
//! Textmethoden `string`, `setString:`, `replaceCharactersInRange:withString:`,
//! `shouldChangeTextInRange:replacementString:`, `didChangeText` und
//! `breakUndoCoalescing`, `length`, `substringWithRange:` und
//! `lengthOfBytesUsingEncoding:` mit `NSUTF8StringEncoding` (`NSString.h:294`),
//! die Delegiertenmethode `textView:shouldChangeTextInRange:replacementString:`
//! (`NSTextView.h:620`), `removeAllActions` am Verwalter, `window` und
//! `makeFirstResponder:`, `sizeToFit` und `setRefusesFirstResponder:` an
//! `NSButton`. Ebenso seit 10.0 und ohne eigene Angabe: die
//! Delegiertenmethode `textDidChange:` (`NSText.h`, Protokoll
//! `NSTextDelegate`) mit ihrem Argument `NSNotification`, `selectedRange` und
//! `setSelectedRange:` an `NSText` und `isEditable`. **Juenger als seine Klasse ist allein
//! `buttonWithTitle:target:action:`**, seit macOS 10.12 (`NSButton.h:40`,
//! `API_AVAILABLE(macos(10.12))`). Die Wertetypen `NSRect`, `NSPoint`, `NSSize`
//! und die Maske `NSAutoresizingMaskOptions` haben kein eigenes macOS-Alter,
//! `MainThreadMarker` ist ein Rust-Typ der Kiste, und das Makro `ns_string!`
//! baut seine Zeichenkette beim Uebersetzen. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine Beruehrung in dieser Datei liegt darueber.

use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, Bool, ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSButton, NSColor, NSScrollView, NSTextDelegate, NSTextView,
    NSTextViewDelegate, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRange, NSRect, NSSize,
    NSString, NSUTF8StringEncoding, NSUndoManager, ns_string,
};

use krk_core::sprache::{Text, text};
use krk_core::tasten::Kommando;

use crate::editormodell::Ansicht;
use crate::hervorhebung::Darstellungsart;
use crate::quicknote;

use super::bereichsleiste::Kommandomelder;
use super::{textautomatik, textmerkmale};

/// Die Hoehe der Schaltflaechenreihe oben in der Rolle.
const KNOPFZEILE: f64 = 30.0;

/// Der Abstand zwischen zwei Schaltflaechen und zum linken Rand.
const KNOPFABSTAND: f64 = 8.0;

/// Was die Quicknote haelt.
pub struct QuicknoteIvars {
    /// Die Ansicht, die der Editorbereich ein- und ausblendet.
    rolle: Retained<NSView>,
    /// Die Textflaeche; ihr Textspeicher ist der Puffer.
    text: Retained<NSTextView>,
    /// Der eigene Verwalter, den `undoManagerForTextView:` liefert.
    verwalter: Retained<NSUndoManager>,
    /// Die Senke fuer den Klick auf eine Schaltflaeche; `None`, bis der
    /// Editorbereich sie setzt. Sie haelt den Editorbereich schwach.
    knopfmelder: RefCell<Option<Kommandomelder>>,
    /// Die Senke fuer eine Aenderung, die die Grenze abgewiesen hat; `None`,
    /// bis der Editorbereich sie setzt. Sie haelt den Editorbereich schwach.
    grenzmelder: RefCell<Option<Box<dyn Fn()>>>,
    /// Die Senke fuer jede Aenderung des Textes, ueber `textDidChange:` und
    /// aus [`Quicknote::nach_kopie_leeren`]; `None`, bis der Editorbereich sie
    /// setzt. Sie haelt den Editorbereich schwach.
    aenderungsmelder: RefCell<Option<Box<dyn Fn()>>>,
}

define_class!(
    /// Die Quicknote und der Delegierte ihrer Textflaeche.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = QuicknoteIvars]
    pub struct Quicknote;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Quicknote {}

    // SAFETY: `NSTextDelegate` stellt keine Bedingungen. Die Textflaeche haelt
    // ihren Delegierten schwach, die Quicknote haelt die Flaeche stark; ein
    // Ring entsteht nicht.
    unsafe impl NSTextDelegate for Quicknote {
        /// Jede Aenderung des Textes, auch Rueckgaengig und Wiederholen
        /// (Modulkopf, „Wie eine Aenderung zur Sicherung kommt").
        // SAFETY: Die Signatur entspricht der des Protokolls (`NSText.h`).
        #[unsafe(method(textDidChange:))]
        fn text_geaendert(&self, _meldung: &NSNotification) {
            self.aenderung_melden();
        }
    }

    // SAFETY: `NSTextViewDelegate` stellt keine Bedingungen; die Signatur der
    // einen Methode entspricht der des Protokolls (`NSTextView.h:642`).
    unsafe impl NSTextViewDelegate for Quicknote {
        /// Der eigene Verwalter der Quicknote (Modulkopf, „Warum der
        /// Verwalter ueber den Delegierten kommt").
        #[unsafe(method_id(undoManagerForTextView:))]
        fn verwalter_fuer(&self, _flaeche: &NSTextView) -> Option<Retained<NSUndoManager>> {
            Some(self.ivars().verwalter.clone())
        }

        /// Laesst eine Aenderung allein zu, wenn der Puffer danach innerhalb
        /// der Grenze bleibt (Modulkopf, „Die Grenze des Puffers").
        ///
        /// `None` als Ersatz ist eine reine Aenderung der Merkmale und passt
        /// immer.
        // SAFETY: Die Signatur entspricht der des Protokolls
        // (`NSTextView.h:620`).
        #[unsafe(method(textView:shouldChangeTextInRange:replacementString:))]
        fn aenderung_pruefen(
            &self,
            flaeche: &NSTextView,
            bereich: NSRange,
            ersatz: Option<&NSString>,
        ) -> Bool {
            let Some(ersatz) = ersatz else {
                return Bool::YES;
            };
            let text = flaeche.string();
            let schranke = text
                .length()
                .saturating_add(ersatz.length())
                .saturating_mul(3);
            let passt = quicknote::aenderung_passt(schranke, || {
                let ohne = text
                    .lengthOfBytesUsingEncoding(NSUTF8StringEncoding)
                    .saturating_sub(
                        text.substringWithRange(bereich)
                            .lengthOfBytesUsingEncoding(NSUTF8StringEncoding),
                    );
                ohne.saturating_add(ersatz.lengthOfBytesUsingEncoding(NSUTF8StringEncoding))
            });
            if !passt {
                let melder = self.ivars().grenzmelder.borrow();
                if let Some(melder) = melder.as_ref() {
                    melder();
                }
            }
            Bool::new(passt)
        }
    }

    impl Quicknote {
        /// Die Schaltflaeche „Leeren".
        // SAFETY: Die Signatur ist die uebliche einer Aktion.
        #[unsafe(method(quicknoteLeeren:))]
        fn leeren_geklickt(&self, _absender: Option<&AnyObject>) {
            self.geklickt(Kommando::QuicknoteLeeren);
        }

        /// Die Schaltflaeche „Schließen": dasselbe Kommando wie F10 mit dem
        /// Fokus in der Quicknote.
        // SAFETY: Die Signatur ist die uebliche einer Aktion.
        #[unsafe(method(quicknoteSchliessen:))]
        fn schliessen_geklickt(&self, _absender: Option<&AnyObject>) {
            self.geklickt(Kommando::QuicknoteUmschalten);
        }

        /// Die Schaltflaeche „Kopieren".
        // SAFETY: Die Signatur ist die uebliche einer Aktion.
        #[unsafe(method(quicknoteKopieren:))]
        fn kopieren_geklickt(&self, _absender: Option<&AnyObject>) {
            self.geklickt(Kommando::QuicknoteKopieren);
        }

        /// Die Schaltflaeche „Strip".
        // SAFETY: Die Signatur ist die uebliche einer Aktion.
        #[unsafe(method(quicknoteStrip:))]
        fn strip_geklickt(&self, _absender: Option<&AnyObject>) {
            self.geklickt(Kommando::QuicknoteStrip);
        }

        /// Die Schaltflaeche „Format".
        // SAFETY: Die Signatur ist die uebliche einer Aktion.
        #[unsafe(method(quicknoteFormat:))]
        fn format_geklickt(&self, _absender: Option<&AnyObject>) {
            self.geklickt(Kommando::QuicknoteFormat);
        }
    }
);

impl Quicknote {
    /// Baut die Rolle mit der Textflaeche darin, im genannten Rahmen und
    /// ausgeblendet.
    ///
    /// Die Groessenregeln sind die der Textflaeche des Editors
    /// (`textflaeche_bauen` in `super::editor`), ohne Nummernspalte: eine
    /// Notiz ohne Datei hat keine Zeilen, auf die sich eine Nummer bezoege.
    pub fn bauen(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let rolle = NSView::initWithFrame(NSView::alloc(mtm), rahmen);
        rolle.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );
        rolle.setHidden(true);

        let innen = NSRect::new(
            NSPoint::ZERO,
            NSSize::new(
                rahmen.size.width,
                (rahmen.size.height - KNOPFZEILE).max(0.0),
            ),
        );
        let bildlauf = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), innen);
        bildlauf.setHasVerticalScroller(true);
        bildlauf.setAutohidesScrollers(true);
        bildlauf.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        let text = NSTextView::initWithFrame(NSTextView::alloc(mtm), innen);
        text.setEditable(true);
        text.setSelectable(true);
        text.setRichText(false);
        text.setImportsGraphics(false);
        // Dieselbe Regel wie an jeder bearbeitbaren Flaeche: das System
        // korrigiert beim Tippen nichts still.
        textautomatik::automatiken_abschalten(&text);
        text.setAllowsUndo(true);
        text.setVerticallyResizable(true);
        text.setHorizontallyResizable(false);
        text.setMinSize(NSSize::ZERO);
        text.setMaxSize(NSSize::new(f64::MAX, f64::MAX));
        text.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
        // Schrift und Grundabsatz als Vorgabe: der Puffer ist anfangs leer,
        // und ohne Vorgabe tabbte getippter Text auf die 28-Punkt-Stopps des
        // Systems (siehe `textmerkmale::grund_vorgeben`).
        textmerkmale::grund_vorgeben(&text, Ansicht::Roh, Darstellungsart::EinfacherText);
        // Die dynamischen Systemfarben ziehen den Wechsel des
        // Erscheinungsbildes selbst nach; ein eigener Nachzug entsteht nicht.
        text.setTextColor(Some(&NSColor::textColor()));
        text.setBackgroundColor(&NSColor::textBackgroundColor());
        bildlauf.setDocumentView(Some(&text));
        rolle.addSubview(&bildlauf);

        let this = Self::alloc(mtm).set_ivars(QuicknoteIvars {
            rolle,
            text,
            verwalter: NSUndoManager::new(mtm),
            knopfmelder: RefCell::new(None),
            grenzmelder: RefCell::new(None),
            aenderungsmelder: RefCell::new(None),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };
        this.ivars()
            .text
            .setDelegate(Some(ProtocolObject::from_ref(&*this)));
        this.knoepfe_bauen(mtm, rahmen);
        this
    }

    /// Baut die Schaltflaechenreihe oben in der Rolle.
    ///
    /// `NSControl` haelt sein Ziel schwach; die Quicknote lebt so lange wie
    /// der Editorbereich, der sie haelt.
    fn knoepfe_bauen(&self, mtm: MainThreadMarker, rahmen: NSRect) {
        // Von links nach rechts; welches Kommando ein Selektor meldet, steht
        // an seiner Methode oben.
        let knoepfe: [(Text, Sel); 5] = [
            (Text::QuicknoteLeeren, sel!(quicknoteLeeren:)),
            (Text::BlattSchliessen, sel!(quicknoteSchliessen:)),
            (Text::QuicknoteKopieren, sel!(quicknoteKopieren:)),
            (Text::QuicknoteStrip, sel!(quicknoteStrip:)),
            (Text::QuicknoteFormat, sel!(quicknoteFormat:)),
        ];
        let mut links = KNOPFABSTAND;
        for (titel, aktion) in knoepfe {
            let titel = text(titel);
            // SAFETY: `self` beantwortet den Selektor mit der ueblichen
            // Aktionssignatur (siehe `define_class!` oben), und `sel!` liefert
            // einen gueltigen Selektor.
            let knopf = unsafe {
                NSButton::buttonWithTitle_target_action(
                    &NSString::from_str(titel),
                    Some(self),
                    Some(aktion),
                    mtm,
                )
            };
            knopf.setRefusesFirstResponder(true);
            knopf.sizeToFit();
            let groesse = knopf.frame().size;
            knopf.setFrame(NSRect::new(
                NSPoint::new(
                    links,
                    rahmen.size.height - KNOPFZEILE + (KNOPFZEILE - groesse.height) / 2.0,
                ),
                groesse,
            ));
            // Die Reihe klebt oben, wenn die Rolle waechst.
            knopf.setAutoresizingMask(NSAutoresizingMaskOptions::ViewMinYMargin);
            self.ivars().rolle.addSubview(&knopf);
            links += groesse.width + KNOPFABSTAND;
        }
    }

    /// Traegt die Senke fuer eine abgewiesene Aenderung ein.
    pub fn grenzmelder_setzen(&self, melder: Box<dyn Fn()>) {
        *self.ivars().grenzmelder.borrow_mut() = Some(melder);
    }

    /// Traegt die Senke fuer jede Aenderung des Textes ein (Modulkopf, „Wie
    /// eine Aenderung zur Sicherung kommt").
    pub fn aenderungsmelder_setzen(&self, melder: Box<dyn Fn()>) {
        *self.ivars().aenderungsmelder.borrow_mut() = Some(melder);
    }

    /// Ruft den Aenderungsmelder, falls einer gesetzt ist.
    fn aenderung_melden(&self) {
        let melder = self.ivars().aenderungsmelder.borrow();
        if let Some(melder) = melder.as_ref() {
            melder();
        }
    }

    /// Traegt die Senke fuer einen Klick ein.
    pub fn knopfmelder_setzen(&self, melder: Kommandomelder) {
        *self.ivars().knopfmelder.borrow_mut() = Some(melder);
    }

    /// Ein Klick: zuerst den Fokus in die Textflaeche, dann das Kommando.
    ///
    /// **Die Reihenfolge ist die Aussage**, nach dem Muster des Ankreuzfeldes
    /// der Aufgabentabelle: die Zulaessigkeit fragt nach dem Fokus, und ein
    /// Klick aus einem anderen Bereich heraus hat ihn noch nicht im Editor.
    fn geklickt(&self, kommando: Kommando) {
        if let Some(fenster) = self.ivars().text.window() {
            // `let _ =`: lehnt AppKit ab, weist die Zulaessigkeit das Kommando
            // ab, und der Klick tut nichts, wie jede abgewiesene Taste.
            let _ = fenster.makeFirstResponder(Some(&self.ivars().text));
        }
        let melder = self.ivars().knopfmelder.borrow();
        if let Some(melder) = melder.as_ref() {
            melder(kommando);
        }
    }

    /// Der ganze Text des Puffers.
    #[must_use]
    pub fn text(&self) -> String {
        self.ivars().text.string().to_string()
    }

    /// Die Auswahl und ihr Text, oder bei leerer Auswahl der ganze Text samt
    /// seinem Bereich (Entscheidung 12 des Plans der Sicherung). Der Bereich
    /// zaehlt in UTF-16, wie `NSString`.
    #[must_use]
    pub fn auswahl_oder_alles(&self) -> (NSRange, String) {
        let flaeche = &self.ivars().text;
        let text = flaeche.string();
        let auswahl = flaeche.selectedRange();
        if auswahl.length == 0 {
            (NSRange::new(0, text.length()), text.to_string())
        } else {
            (auswahl, text.substringWithRange(auswahl).to_string())
        }
    }

    /// Ersetzt `bereich` durch `ersatz` als eine Handlung, die `cmd+z`
    /// zuruecknimmt, und waehlt danach den Ersatz aus; `false`, wenn die
    /// Flaeche die Aenderung abgewiesen hat (ueber der Grenze, oder vor dem
    /// Einlesen nicht bearbeitbar).
    ///
    /// **Der eine Ersetzungsweg der Quicknote**, fuer Leeren, Strip und
    /// Format. Angemeldet ueber `shouldChangeTextInRange:replacementString:`
    /// und `didChangeText`, dieselben zwei Rufe, mit denen die Flaeche das
    /// Tippen anmeldet; die Grenzpruefung des Delegierten gilt damit auch hier,
    /// und `didChangeText` loest `textDidChange:` und damit die Vormerkung der
    /// Sicherung aus. `breakUndoCoalescing` davor, damit die Ersetzung nicht in
    /// die letzte Tipp-Handlung faellt.
    #[must_use = "eine abgewiesene Ersetzung hat am Text nichts geaendert"]
    pub fn ersetzen(&self, bereich: NSRange, ersatz: &str) -> bool {
        let flaeche = &self.ivars().text;
        let ersatz = NSString::from_str(ersatz);
        flaeche.breakUndoCoalescing();
        if !flaeche.shouldChangeTextInRange_replacementString(bereich, Some(&ersatz)) {
            return false;
        }
        flaeche.replaceCharactersInRange_withString(bereich, &ersatz);
        flaeche.didChangeText();
        flaeche.setSelectedRange(NSRange::new(bereich.location, ersatz.length()));
        true
    }

    /// Leert den Puffer als eine Handlung, die `cmd+z` zuruecknimmt (A13 des
    /// Spec), ueber [`Self::ersetzen`]. Ein leerer Puffer bleibt unangetastet.
    pub fn leeren(&self) {
        let laenge = self.ivars().text.string().length();
        if laenge == 0 {
            return;
        }
        // `let _ =`: eine abgewiesene Ersetzung laesst den Text stehen, und
        // Leeren hat dafuer keine eigene Meldung.
        let _ = self.ersetzen(NSRange::new(0, laenge), "");
    }

    /// Setzt den gelesenen Inhalt von `scratch.txt` ein, **ohne Eintrag im
    /// Rueckgaengig und ohne Aenderungsmeldung** (Entscheidung 3 des Plans der
    /// Sicherung).
    ///
    /// `setString:` loest `textDidChange:` nicht aus, also merkt die Sicherung
    /// nichts vor; `removeAllActions` danach aus dem Grund an
    /// [`Self::nach_kopie_leeren`].
    pub fn einsetzen_ohne_rueckgaengig(&self, text: &str) {
        self.ivars().text.setString(&NSString::from_str(text));
        self.ivars().verwalter.removeAllActions();
    }

    /// Schaltet die Textflaeche bearbeitbar oder nicht; vor dem Einlesen ist
    /// sie es nicht (Modulkopf).
    pub fn bearbeitbar_setzen(&self, bearbeitbar: bool) {
        self.ivars().text.setEditable(bearbeitbar);
    }

    /// Leert den Puffer nach einem gelungenen Kopieren, ohne Rueckgaengig
    /// (Entscheidung 10 des Plans).
    ///
    /// `setString:` schreibt an der Rueckgaengigverwaltung vorbei, und ein
    /// stehengebliebener Stapel zeigte auf Text, den die Flaeche nicht mehr
    /// traegt; deshalb raeumt `removeAllActions` den eigenen Verwalter danach
    /// ab. Dieselbe Begruendung wie an `Editorbereich::stand_einsetzen`.
    ///
    /// **Es meldet die Aenderung selbst**, weil `setString:` an
    /// `textDidChange:` vorbeischreibt und die Sicherung sonst den Text vor dem
    /// Kopieren in `scratch.txt` liesse (Entscheidung 4 des Plans der
    /// Sicherung).
    pub fn nach_kopie_leeren(&self) {
        self.ivars().text.setString(ns_string!(""));
        self.ivars().verwalter.removeAllActions();
        self.aenderung_melden();
    }

    /// Die Ansicht, die der Editorbereich ein- und ausblendet.
    pub fn rolle(&self) -> &NSView {
        &self.ivars().rolle
    }

    /// Die Textflaeche, die den Ersthelfer nimmt und an der die
    /// Naemlichkeitsfrage des Anwendungsdelegierten haengt.
    pub fn textflaeche(&self) -> &NSTextView {
        &self.ivars().text
    }

    /// Der eigene Verwalter, fuer die Probe im Editor.
    #[cfg(test)]
    pub fn verwalter(&self) -> &NSUndoManager {
        &self.ivars().verwalter
    }
}

#[cfg(test)]
mod tests {
    use super::super::anwendung::quelltextproben::{datei, rumpf};

    /// Keine Codezeile dieser Datei nennt die Zwischenablage; das Kopieren
    /// geht beim Anwendungsdelegierten ueber die eine Huelle.
    #[test]
    fn die_quicknote_nennt_die_zwischenablage_nicht() {
        let quelle = datei("krk-ui/src/appkit/quicknote.rs");
        let (code, _) = quelle
            .split_once(concat!("#[cfg(test)]\nmod ", "tests {"))
            .expect("das Pruefmodul steht am Fuss der Datei");
        let nennt = code
            .lines()
            .filter(|zeile| !zeile.trim_start().starts_with("//"))
            .any(|zeile| zeile.contains(concat!("NSPaste", "board")));
        assert!(!nennt, "die Quicknote spricht die Zwischenablage selbst an");
    }

    /// Die fuenf Aktionen melden ihre fuenf Kommandos, und jede ueber
    /// `geklickt`, das zuerst den Fokus holt.
    #[test]
    fn die_aktionen_melden_ihre_kommandos() {
        let quelle = datei("krk-ui/src/appkit/quicknote.rs");
        for (aktion, kommando) in [
            ("leeren_geklickt", "Kommando::QuicknoteLeeren"),
            ("schliessen_geklickt", "Kommando::QuicknoteUmschalten"),
            ("kopieren_geklickt", "Kommando::QuicknoteKopieren"),
            ("strip_geklickt", "Kommando::QuicknoteStrip"),
            ("format_geklickt", "Kommando::QuicknoteFormat"),
        ] {
            let rumpf = rumpf(&quelle, aktion);
            assert!(
                rumpf.contains(&format!("self.geklickt({kommando})")),
                "{aktion} meldet nicht {kommando}"
            );
        }
        let geklickt = rumpf(&quelle, "geklickt");
        let fokus = geklickt
            .find(concat!("makeFirst", "Responder("))
            .expect("der Klick holt den Fokus nicht");
        let melden = geklickt
            .find("melder(kommando)")
            .expect("der Klick meldet kein Kommando");
        assert!(fokus < melden, "der Klick meldet vor dem Fokus");
    }

    // Die vier Proben darunter lesen den Quelltext und nicht eine Flaeche:
    // eine `NSTextView`, deren Text sich aendert, laesst sich unter `libtest`
    // nicht fahren (Kommentar ueber den Proben des Umbauwegs im Editor; am
    // 261031 wieder gemessen, der Prueflauf haengt). Was AppKit dabei tut, ist
    // im laufenden Programm gemessen
    // (`messungen/261031-1300-quicknote-textdidchange.txt`).

    /// Strip, Format und Leeren sind eine Handlung, die `cmd+z` zuruecknimmt,
    /// ueber den einen Ersetzungsweg `ersetzen` (Entscheidung 12 des Plans der
    /// Sicherung), und einen zweiten gibt es in dieser Datei nicht.
    #[test]
    fn strip_und_format_sind_eine_zuruecknehmbare_handlung() {
        let quelle = datei("krk-ui/src/appkit/quicknote.rs");
        let ersetzen = rumpf(&quelle, "ersetzen");
        let folge = [
            "breakUndoCoalescing()",
            "shouldChangeTextInRange_replacementString(",
            "replaceCharactersInRange_withString(",
            "didChangeText()",
            "setSelectedRange(",
        ];
        let stellen: Vec<usize> = folge
            .iter()
            .map(|nadel| {
                ersetzen
                    .find(nadel)
                    .unwrap_or_else(|| panic!("ersetzen ruft {nadel} nicht"))
            })
            .collect();
        assert!(
            stellen.windows(2).all(|paar| paar[0] < paar[1]),
            "ersetzen ruft {folge:?} nicht in dieser Reihenfolge"
        );
        assert!(
            rumpf(&quelle, "leeren").contains("self.ersetzen("),
            "Leeren geht nicht ueber ersetzen"
        );
        let (code, _) = quelle
            .split_once(concat!("#[cfg(test)]\nmod ", "tests {"))
            .expect("das Pruefmodul steht am Fuss der Datei");
        let ersetzungen = crate::quellbaum::codezeilen(code)
            .filter(|zeile| zeile.contains(concat!("replaceCharacters", "InRange")))
            .count();
        assert_eq!(
            ersetzungen, 1,
            "neben ersetzen steht ein zweiter Ersetzungsweg"
        );
    }

    /// Eine Umformung wirkt auf die Auswahl und ohne Auswahl auf den ganzen
    /// Text, und danach ist das Ergebnis ausgewaehlt (Entscheidung 12).
    #[test]
    fn eine_umformung_wirkt_auf_die_auswahl_sonst_auf_alles() {
        let quelle = datei("krk-ui/src/appkit/quicknote.rs");
        let auswahl = rumpf(&quelle, "auswahl_oder_alles");
        assert!(auswahl.contains("flaeche.selectedRange()"));
        assert!(auswahl.contains("if auswahl.length == 0 {"));
        assert!(auswahl.contains("NSRange::new(0, text.length())"));
        assert!(auswahl.contains("text.substringWithRange(auswahl)"));
        assert!(
            rumpf(&quelle, "ersetzen")
                .contains("setSelectedRange(NSRange::new(bereich.location, ersatz.length()))"),
            "nach der Ersetzung ist das Ergebnis nicht ausgewaehlt"
        );
    }

    /// Der gelesene Inhalt steht ohne Rueckgaengig und ohne Vormerkung; das
    /// Leeren nach dem Kopieren meldet sich selbst, und jede andere Aenderung
    /// kommt ueber `textDidChange:` (Entscheidungen 3 und 4 des Plans der
    /// Sicherung).
    #[test]
    fn das_eingelesene_steht_ohne_rueckgaengig_und_ohne_vormerkung() {
        let quelle = datei("krk-ui/src/appkit/quicknote.rs");
        let einsetzen = rumpf(&quelle, "einsetzen_ohne_rueckgaengig");
        assert!(einsetzen.contains("setString("));
        assert!(einsetzen.contains("removeAllActions()"));
        assert!(
            !einsetzen.contains("aenderung_melden"),
            "das Einsetzen des Gelesenen merkt eine Sicherung vor"
        );
        assert!(
            rumpf(&quelle, "nach_kopie_leeren").contains("self.aenderung_melden()"),
            "das Leeren nach dem Kopieren meldet sich nicht"
        );
        assert!(
            rumpf(&quelle, "text_geaendert").contains("self.aenderung_melden()"),
            "textDidChange: meldet die Aenderung nicht"
        );
    }
}
