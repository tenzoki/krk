//! Die Flaeche der Quicknote: eine Textflaeche mit eigenem
//! Rueckgaengigverwalter, deren Text allein im Arbeitsspeicher liegt (Plan
//! `260927-0110_*_plan-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md`).
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
//! # Die drei Schaltflaechen
//!
//! Oben in der Rolle stehen „Leeren", „Schließen" und „Kopieren", von links
//! nach rechts (Entscheidung 2 des Plans). **Ein Klick ist ein Kommando** und
//! geht ueber den Knopfmelder, den der Editorbereich beim Bau setzt, durch
//! dieselbe Zulaessigkeit wie die Taste; die Schaltflaechen nehmen den
//! Ersthelferrang nicht an, und ein Klick holt zuerst den Fokus in die
//! Textflaeche, damit die Zulaessigkeit den Fokus im Editor sieht. Keine
//! Aktion heisst `copy:`, `cut:` oder `paste:`: die Zwischenablage erreicht
//! diese Datei nicht, das Kopieren geht beim Anwendungsdelegierten ueber
//! `super::zwischenablage::text_schreiben`.
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
//! `breakUndoCoalescing`, `removeAllActions` am Verwalter, `window` und
//! `makeFirstResponder:`, `sizeToFit` und `setRefusesFirstResponder:` an
//! `NSButton`. **Juenger als seine Klasse ist allein
//! `buttonWithTitle:target:action:`**, seit macOS 10.12 (`NSButton.h:40`,
//! `API_AVAILABLE(macos(10.12))`). Die Wertetypen `NSRect`, `NSPoint`, `NSSize`
//! und die Maske `NSAutoresizingMaskOptions` haben kein eigenes macOS-Alter,
//! `MainThreadMarker` ist ein Rust-Typ der Kiste, und das Makro `ns_string!`
//! baut seine Zeichenkette beim Uebersetzen. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine Beruehrung in dieser Datei liegt darueber.

use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSButton, NSColor, NSScrollView, NSTextDelegate, NSTextView,
    NSTextViewDelegate, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSObject, NSObjectProtocol, NSPoint, NSRange, NSRect, NSSize, NSString,
    NSUndoManager, ns_string,
};

use krk_core::tasten::Kommando;

use crate::editormodell::Ansicht;
use crate::hervorhebung::Darstellungsart;

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
    unsafe impl NSTextDelegate for Quicknote {}

    // SAFETY: `NSTextViewDelegate` stellt keine Bedingungen; die Signatur der
    // einen Methode entspricht der des Protokolls (`NSTextView.h:642`).
    unsafe impl NSTextViewDelegate for Quicknote {
        /// Der eigene Verwalter der Quicknote (Modulkopf, „Warum der
        /// Verwalter ueber den Delegierten kommt").
        #[unsafe(method_id(undoManagerForTextView:))]
        fn verwalter_fuer(&self, _flaeche: &NSTextView) -> Option<Retained<NSUndoManager>> {
            Some(self.ivars().verwalter.clone())
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
        text.setFont(Some(&textmerkmale::grundschrift(
            Ansicht::Roh,
            Darstellungsart::EinfacherText,
        )));
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
        let knoepfe: [(&str, Sel); 3] = [
            ("Leeren", sel!(quicknoteLeeren:)),
            ("Schließen", sel!(quicknoteSchliessen:)),
            ("Kopieren", sel!(quicknoteKopieren:)),
        ];
        let mut links = KNOPFABSTAND;
        for (titel, aktion) in knoepfe {
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

    /// Leert den Puffer als eine Handlung, die `cmd+z` zuruecknimmt (A13 des
    /// Spec).
    ///
    /// Angemeldet ueber `shouldChangeTextInRange:replacementString:` und
    /// `didChangeText`, dieselben zwei Rufe, mit denen die Flaeche das Tippen
    /// anmeldet; `breakUndoCoalescing` davor, damit das Leeren nicht in die
    /// letzte Tipp-Handlung faellt. Ein leerer Puffer bleibt unangetastet.
    pub fn leeren(&self) {
        let text = &self.ivars().text;
        let bereich = NSRange::new(0, text.string().length());
        if bereich.length == 0 {
            return;
        }
        text.breakUndoCoalescing();
        if text.shouldChangeTextInRange_replacementString(bereich, Some(ns_string!(""))) {
            text.replaceCharactersInRange_withString(bereich, ns_string!(""));
            text.didChangeText();
        }
    }

    /// Leert den Puffer nach einem gelungenen Kopieren, ohne Rueckgaengig
    /// (Entscheidung 10 des Plans).
    ///
    /// `setString:` schreibt an der Rueckgaengigverwaltung vorbei, und ein
    /// stehengebliebener Stapel zeigte auf Text, den die Flaeche nicht mehr
    /// traegt; deshalb raeumt `removeAllActions` den eigenen Verwalter danach
    /// ab. Dieselbe Begruendung wie an `Editorbereich::stand_einsetzen`.
    pub fn nach_kopie_leeren(&self) {
        self.ivars().text.setString(ns_string!(""));
        self.ivars().verwalter.removeAllActions();
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

    /// Die drei Aktionen melden die drei Kommandos, und jede ueber
    /// `geklickt`, das zuerst den Fokus holt.
    #[test]
    fn die_drei_aktionen_melden_ihre_kommandos() {
        let quelle = datei("krk-ui/src/appkit/quicknote.rs");
        for (aktion, kommando) in [
            ("leeren_geklickt", "Kommando::QuicknoteLeeren"),
            ("schliessen_geklickt", "Kommando::QuicknoteUmschalten"),
            ("kopieren_geklickt", "Kommando::QuicknoteKopieren"),
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
}
