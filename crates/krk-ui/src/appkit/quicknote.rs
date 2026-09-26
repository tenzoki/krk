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
//! eigene Verfuegbarkeitsangabe. Die Wertetypen `NSRect`, `NSPoint`, `NSSize`
//! und die Maske `NSAutoresizingMaskOptions` haben kein eigenes macOS-Alter,
//! `MainThreadMarker` ist ein Rust-Typ der Kiste. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine Beruehrung in dieser Datei liegt darueber.

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSColor, NSScrollView, NSTextDelegate, NSTextView,
    NSTextViewDelegate, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSUndoManager,
};

use crate::editormodell::Ansicht;
use crate::hervorhebung::Darstellungsart;

use super::{textautomatik, textmerkmale};

/// Was die Quicknote haelt.
pub struct QuicknoteIvars {
    /// Die Ansicht, die der Editorbereich ein- und ausblendet.
    rolle: Retained<NSView>,
    /// Die Textflaeche; ihr Textspeicher ist der Puffer.
    text: Retained<NSTextView>,
    /// Der eigene Verwalter, den `undoManagerForTextView:` liefert.
    verwalter: Retained<NSUndoManager>,
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

        let innen = NSRect::new(NSPoint::ZERO, rahmen.size);
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
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };
        this.ivars()
            .text
            .setDelegate(Some(ProtocolObject::from_ref(&*this)));
        this
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
