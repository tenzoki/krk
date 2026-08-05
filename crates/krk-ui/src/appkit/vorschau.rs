//! Das Vorschaufenster: Tableiste, Text- und Bildanzeige, angebunden an das
//! Modell aus [`crate::vorschaumodell`] (C6, C10).
//!
//! ```text
//! ┌──────────────────────────────┐
//! │ Tableiste (appkit::tableiste)│  ein Abschnitt je Vorschau-Tab
//! ├──────────────────────────────┤
//! │ Inhaltsflaeche               │  nimmt Klick und Fokus entgegen
//! │   NSScrollView + NSTextView  │  Text, Metadaten, Hinweise
//! │   NSImageView                │  Bilder; je einer von beiden sichtbar
//! └──────────────────────────────┘
//! ```
//!
//! **Dieselbe Tableiste wie am Dateifenster, ein zweites Mal.** C6 verlangt
//! fuer die Vorschau-Tabs "dieselben Befehle zum Oeffnen, Schliessen und
//! Wechseln wie in C1"; die Leiste dazu ist [`super::tableiste::Tableiste`]
//! aus S12, und eine zweite Leistensorte daneben entsteht nicht.
//!
//! **Was hier steht und was im Modell.** Die Tabs, ihr Inhalt, das
//! Halteverhalten und das Lesen der Vorschaudatei auf dem Arbeitsfaden wohnen
//! in [`Vorschaumodell`] und damit ausserhalb von `appkit/`. Diese Datei setzt
//! den [`Inhalt`] des aktiven Tabs in die Ansichten um, weil `NSImage`,
//! `NSTextView` und die beiden Formatierer AppKit sind, und trifft keine
//! Entscheidung darueber, was ein Tab zeigt.
//!
//! **Wie eine Meldung des Arbeitsfadens den Hauptfaden erreicht.** Wie beim
//! Dateifenster: ein Zeitgeber auf dem Hauptfaden raeumt die Kanaele leer,
//! solange ein Tab laedt, und endet, sobald keiner mehr laedt. Derselbe Takt
//! wie der Einzugstakt aus [`super::tabelle`].
//!
//! **Die Inhaltsflaeche nimmt den Eingabefokus.** Ein Klick in den Inhalt
//! macht sie zum Ersthelfer, und damit bedienen die vier Tabbefehle aus C1 die
//! Vorschau-Tabs ([`Wirkungsbereich::Tabbereich`](krk_core::tasten::Wirkungsbereich)).
//! Die Textanzeige ist dafuer nicht auswaehlbar: eine auswaehlbare naehme den
//! Fokus als Textsystem, und der Ereignisabgriff reichte jede Taste an AppKit
//! weiter, statt die Tabbefehle auszufuehren. Einen Tastenbefehl, der den
//! Fokus hierher setzt, gibt es in dieser Runde nicht; die offene Frage dazu
//! liegt im Entscheidungsspeicher.

use std::cell::RefCell;
use std::path::Path;

use objc2::rc::Retained;
use objc2::{AnyThread, DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSEvent, NSFont, NSImage, NSImageScaling, NSImageView, NSScrollView,
    NSTextView, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSByteCountFormatter, NSByteCountFormatterCountStyle, NSData, NSDate,
    NSDateFormatter, NSDateFormatterStyle, NSObject, NSObjectProtocol, NSPoint, NSRect, NSRunLoop,
    NSRunLoopCommonModes, NSSize, NSString, NSTimeInterval, NSTimer,
};

use krk_core::tasten::Kommando;
use krk_core::verzeichnis::Typ;

use crate::vorschaumodell::{Inhalt, Metadaten, Vorschaumodell, Zwischenablageinhalt, rechte_text};

use super::tabelle::typ_beschriften;
use super::tableiste::{self, Tableiste};

/// Die Groesse, mit der die Ansichten entstehen, bevor die Aufteilung sie
/// auslegt.
const AUFBAUGROESSE: NSSize = NSSize::new(260.0, 400.0);

/// Der Takt, in dem der Hauptfaden die Meldungen der Arbeitsfaeden abholt.
///
/// Dieselbe Zahl wie der Einzugstakt des Dateifensters, aus demselben Grund:
/// haeufiger zu fragen braechte nichts, weil nicht oefter gezeichnet wird.
const LADETAKT: NSTimeInterval = 1.0 / 60.0;

/// Was ein leerer Tab sagt, statt eine leere Flaeche zu zeigen.
const LEERTEXT: &str = "Kein Inhalt. Die Auswahl im Dateifenster füllt diesen Tab.";

define_class!(
    /// Die Flaeche unter der Tableiste: sie nimmt Klick und Fokus entgegen.
    ///
    /// Ein eigener Ersthelfer, damit [`Fokus::Vorschau`](crate::kommandos::fokus::Fokus)
    /// ueberhaupt eintreten kann: die Textanzeige darin ist nicht auswaehlbar
    /// und lehnt den Fokus ab, die Bildanzeige ebenso. Ihr Klick faellt durch
    /// die Antwortkette hierher.
    // SAFETY:
    // - Die Oberklasse NSView stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = ()]
    pub struct Inhaltsflaeche;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Inhaltsflaeche {}

    impl Inhaltsflaeche {
        /// Die Flaeche nimmt den Eingabefokus an.
        // SAFETY: Die Signatur entspricht der Eigenschaft von NSResponder.
        #[unsafe(method(acceptsFirstResponder))]
        fn nimmt_ersthelferrang(&self) -> bool {
            true
        }

        /// Ein Klick in den Inhalt holt den Fokus in die Vorschau.
        // SAFETY: Die Signatur entspricht der von NSResponder.
        #[unsafe(method(mouseDown:))]
        fn maus_gedrueckt(&self, _ereignis: &NSEvent) {
            if let Some(fenster) = self.window() {
                fenster.makeFirstResponder(Some(self));
            }
        }
    }
);

impl Inhaltsflaeche {
    /// Eine Flaeche mit dem genannten Rahmen.
    fn neu(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: `initWithFrame:` von NSView hat die hier angenommene
        // Signatur.
        unsafe { msg_send![super(this), initWithFrame: rahmen] }
    }
}

/// Was das Vorschaufenster haelt.
pub struct VorschaufensterIvars {
    /// Der Bereich, der in die Aufteilung gehaengt wird.
    bereich: Retained<NSView>,
    /// Die fokussierbare Flaeche unter der Tableiste.
    inhaltsflaeche: Retained<Inhaltsflaeche>,
    /// Die Bildlaufansicht um die Textanzeige.
    textrolle: Retained<NSScrollView>,
    /// Die Textanzeige: Text, Metadaten und Hinweise.
    text: Retained<NSTextView>,
    /// Die Bildanzeige (C6: Bilder ueber `NSImage`).
    bild: Retained<NSImageView>,
    /// Die Leiste am Kopf. Sie kommt nach dem Objekt zur Welt, weil ihr
    /// Rueckruf es braucht; dieselbe Reihenfolge wie beim Dateifenster.
    tableiste: RefCell<Option<Tableiste>>,
    /// Die Tabs mit ihrem Inhalt und dem Halteverhalten.
    modell: RefCell<Vorschaumodell>,
    /// Der Zeitgeber, der die Meldungen der Arbeitsfaeden abholt.
    ///
    /// Er haelt das Objekt als Ziel fest, und das Objekt haelt ihn; der Ring
    /// bricht mit `invalidate`, wie beim Einzugstakt des Dateifensters.
    takt: RefCell<Option<Retained<NSTimer>>>,
    /// Der Formatierer fuer das Aenderungsdatum der Metadaten.
    datumsformat: Retained<NSDateFormatter>,
    /// Der Formatierer fuer die Groesse der Metadaten.
    groessenformat: Retained<NSByteCountFormatter>,
}

define_class!(
    /// Das Vorschaufenster (C6).
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = VorschaufensterIvars]
    pub struct Vorschaufenster;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Vorschaufenster {}

    impl Vorschaufenster {
        /// Der Rueckruf des Zeitgebers.
        // SAFETY: Die Signatur passt zu der, die NSTimer aufruft.
        #[unsafe(method(ladenEinziehen:))]
        fn laden_einziehen(&self, _zeitgeber: &NSTimer) {
            self.einziehen();
        }
    }
);

impl Vorschaufenster {
    /// Baut das Vorschaufenster mit einem leeren Tab.
    pub fn bauen(mtm: MainThreadMarker) -> Retained<Self> {
        let rahmen = NSRect::new(NSPoint::ZERO, AUFBAUGROESSE);
        let bereich = NSView::initWithFrame(NSView::alloc(mtm), rahmen);
        bereich.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        let inhaltsrahmen = NSRect::new(
            NSPoint::ZERO,
            NSSize::new(AUFBAUGROESSE.width, AUFBAUGROESSE.height - tableiste::HOEHE),
        );
        let inhaltsflaeche = Inhaltsflaeche::neu(mtm, inhaltsrahmen);
        inhaltsflaeche.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        let fuellend = NSRect::new(NSPoint::ZERO, inhaltsrahmen.size);
        let (textrolle, text) = textanzeige(mtm, fuellend);
        inhaltsflaeche.addSubview(&textrolle);

        let bild = NSImageView::initWithFrame(NSImageView::alloc(mtm), fuellend);
        bild.setImageScaling(NSImageScaling::ScaleProportionallyDown);
        bild.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );
        bild.setHidden(true);
        inhaltsflaeche.addSubview(&bild);

        bereich.addSubview(&inhaltsflaeche);

        let datumsformat = NSDateFormatter::new();
        datumsformat.setDateStyle(NSDateFormatterStyle::ShortStyle);
        datumsformat.setTimeStyle(NSDateFormatterStyle::ShortStyle);
        let groessenformat = NSByteCountFormatter::new();
        groessenformat.setCountStyle(NSByteCountFormatterCountStyle::File);

        let this = Self::alloc(mtm).set_ivars(VorschaufensterIvars {
            bereich,
            inhaltsflaeche,
            textrolle,
            text,
            bild,
            tableiste: RefCell::new(None),
            modell: RefCell::new(Vorschaumodell::neu()),
            takt: RefCell::new(None),
            datumsformat,
            groessenformat,
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };

        // Die Tableiste zuletzt: ihr Rueckruf braucht das Objekt. Er haelt es
        // **schwach**, sonst schloesse sich der Ring Vorschau → Tableiste →
        // Ziel → Rueckruf → Vorschau; dieselbe Form wie beim Dateifenster.
        let schwach = objc2::rc::Weak::from_retained(&this.retain());
        let leiste = Tableiste::bauen(this.mtm(), move |stelle| {
            if let Some(vorschau) = schwach.load() {
                vorschau.tab_waehlen(stelle);
            }
        });
        let leistensicht = leiste.sicht().retain();
        leistensicht.setFrame(NSRect::new(
            NSPoint::new(0.0, AUFBAUGROESSE.height - tableiste::HOEHE),
            NSSize::new(AUFBAUGROESSE.width, tableiste::HOEHE),
        ));
        this.ivars().bereich.addSubview(&leistensicht);
        *this.ivars().tableiste.borrow_mut() = Some(leiste);

        this.anzeigen();
        this
    }

    /// Die Ansicht, die in die Aufteilung gehaengt wird.
    pub fn sicht(&self) -> &NSView {
        &self.ivars().bereich
    }

    /// Die Flaeche, die den Eingabefokus traegt.
    ///
    /// Fuer die Fokusabfrage und den Fokuswechsel des Anwendungsdelegierten;
    /// sie wird sonst nirgends nach aussen gereicht.
    pub fn fokusansicht(&self) -> &NSView {
        &self.ivars().inhaltsflaeche
    }

    /// Zeigt den genannten Eintrag im aktiven Tab (C6).
    ///
    /// Kehrt sofort zurueck; gelesen wird auf dem Arbeitsfaden des Modells,
    /// und der Zeitgeber holt die Meldung ab.
    pub fn datei_anzeigen(&self, pfad: &Path) {
        self.ivars().modell.borrow_mut().datei_anzeigen(pfad);
        self.anzeigen();
        self.takt_starten();
    }

    /// Welche Datei der aktive Tab zeigt; `None`, wenn keine Datei.
    ///
    /// Nur zum Ablesen, fuer die Endbedingung von L7 im Messmodus.
    pub fn angezeigter_pfad(&self) -> Option<std::path::PathBuf> {
        self.ivars().modell.borrow().aktiver_pfad()
    }

    /// Ob ein Vorschau-Tab noch auf seinen Arbeitsfaden wartet.
    ///
    /// Nur zum Ablesen, fuer dieselbe Endbedingung.
    pub fn laedt_noch(&self) -> bool {
        self.ivars().modell.borrow().laedt_noch()
    }

    /// Zeigt den Inhalt der Zwischenablage im aktiven Tab (C10).
    pub fn zwischenablage_anzeigen(&self, inhalt: Zwischenablageinhalt) {
        self.ivars()
            .modell
            .borrow_mut()
            .zwischenablage_anzeigen(inhalt);
        self.anzeigen();
    }

    /// Fuehrt einen der vier Tabbefehle aus C1 auf den Vorschau-Tabs aus (C6).
    ///
    /// Alles andere geht zurueck an den Aufrufer: die Vorschau traegt keine
    /// Auswahl und keine Liste, und ein hier nicht ausgefuehrtes Kommando
    /// laeuft wie ein unbelegtes weiter.
    pub fn kommando_ausfuehren(&self, kommando: Kommando) -> bool {
        {
            let mut modell = self.ivars().modell.borrow_mut();
            match kommando {
                Kommando::TabNeu => {
                    modell.oeffnen();
                }
                Kommando::TabSchliessen => {
                    modell.schliessen();
                }
                Kommando::TabNaechster => {
                    modell.naechster();
                }
                Kommando::TabVoriger => {
                    modell.voriger();
                }
                _ => return false,
            }
        }
        self.anzeigen();
        true
    }

    /// Wechselt auf den Tab an der genannten Stelle (Klick in der Tableiste).
    fn tab_waehlen(&self, stelle: usize) {
        let gewechselt = self.ivars().modell.borrow_mut().waehlen(stelle);
        if gewechselt {
            self.anzeigen();
        } else {
            // Die Leiste hat die Wahl schon optisch umgesetzt; sie wird aus
            // dem Modell zurueckgeschrieben, damit beide dasselbe sagen.
            self.tableiste_nachziehen();
        }
    }

    /// Holt die Meldungen der Arbeitsfaeden ab.
    fn einziehen(&self) {
        let aktiver_geaendert = self.ivars().modell.borrow_mut().einziehen();
        if aktiver_geaendert {
            self.anzeigen();
        }
        let laedt_noch = self.ivars().modell.borrow().laedt_noch();
        if !laedt_noch {
            self.takt_beenden();
        }
    }

    /// Schreibt den aktiven Tab in die Ansichten.
    ///
    /// Die eine Stelle, die aus einem [`Inhalt`] Anzeige macht. Die Ausleihe
    /// des Modells endet, bevor der erste Objective-C-Aufruf faellt; deshalb
    /// der Umweg ueber den geklonten Inhalt.
    fn anzeigen(&self) {
        let (titel, aktiv, inhalt) = {
            let modell = self.ivars().modell.borrow();
            (
                modell.titel(),
                modell.aktive_stelle(),
                modell.aktiver_inhalt().clone(),
            )
        };

        match inhalt {
            Inhalt::Leer => self.text_zeigen(LEERTEXT),
            Inhalt::Text(text) => self.text_zeigen(&text),
            Inhalt::Hinweis(hinweis) => self.text_zeigen(&hinweis),
            Inhalt::Metadaten(metadaten) => {
                let zeilen = self.metadaten_text(&metadaten);
                self.text_zeigen(&zeilen);
            }
            Inhalt::Bild { daten, metadaten } => self.bild_zeigen(&daten, metadaten.as_ref()),
        }

        let leiste = self.ivars().tableiste.borrow();
        if let Some(leiste) = leiste.as_ref() {
            leiste.setzen(&titel, aktiv);
        }
    }

    /// Schreibt Beschriftungen und aktive Stelle in die Tableiste.
    fn tableiste_nachziehen(&self) {
        let (titel, aktiv) = {
            let modell = self.ivars().modell.borrow();
            (modell.titel(), modell.aktive_stelle())
        };
        let leiste = self.ivars().tableiste.borrow();
        if let Some(leiste) = leiste.as_ref() {
            leiste.setzen(&titel, aktiv);
        }
    }

    /// Stellt Text in die Textanzeige und blendet die Bildanzeige aus.
    fn text_zeigen(&self, text: &str) {
        let ivars = self.ivars();
        ivars.text.setString(&NSString::from_str(text));
        ivars.textrolle.setHidden(false);
        ivars.bild.setHidden(true);
    }

    /// Stellt ein Bild in die Bildanzeige, oder faellt auf die Metadaten
    /// zurueck, wenn `NSImage` die Daten nicht liest.
    fn bild_zeigen(&self, daten: &[u8], metadaten: Option<&Metadaten>) {
        let bild = NSImage::initWithData(NSImage::alloc(), &NSData::with_bytes(daten));
        match bild {
            Some(bild) => {
                let ivars = self.ivars();
                ivars.bild.setImage(Some(&bild));
                ivars.bild.setHidden(false);
                ivars.textrolle.setHidden(true);
            }
            None => match metadaten {
                Some(metadaten) => {
                    let zeilen = self.metadaten_text(metadaten);
                    self.text_zeigen(&zeilen);
                }
                None => {
                    self.text_zeigen("Das Bild aus der Zwischenablage ließ sich nicht darstellen.")
                }
            },
        }
    }

    /// Die sechs Metadatenzeilen aus C6.
    fn metadaten_text(&self, metadaten: &Metadaten) -> String {
        // Ein Ordner hat keine eigene Groesse; dieselbe Antwort wie die
        // Groessenspalte aus C1.
        let groesse = if metadaten.typ == Typ::Ordner {
            "--".to_owned()
        } else {
            let bytes = i64::try_from(metadaten.groesse).unwrap_or(i64::MAX);
            self.ivars()
                .groessenformat
                .stringFromByteCount(bytes)
                .to_string()
        };
        let geaendert = match metadaten.geaendert.duration_since(std::time::UNIX_EPOCH) {
            Ok(seit_epoche) => {
                let datum = NSDate::dateWithTimeIntervalSince1970(seit_epoche.as_secs_f64());
                self.ivars().datumsformat.stringFromDate(&datum).to_string()
            }
            // Ein Zeitpunkt vor 1970: moeglich, aber keine eigene Darstellung
            // wert, wie in der Datumsspalte aus C1.
            Err(_) => String::new(),
        };
        format!(
            "Name: {}\nPfad: {}\nGröße: {}\nGeändert: {}\nRechte: {}\nTyp: {}",
            metadaten.name,
            metadaten.pfad.display(),
            groesse,
            geaendert,
            rechte_text(metadaten.rechte),
            typ_beschriften(metadaten.typ),
        )
    }

    /// Haengt den Zeitgeber in die Laufschleife, falls er noch nicht laeuft.
    fn takt_starten(&self) {
        if self.ivars().takt.borrow().is_some() {
            return;
        }
        // SAFETY: `self` ist das Ziel und beantwortet `ladenEinziehen:` mit
        // der erwarteten Signatur. Der Zeitgeber wird unten in die
        // Laufschleife gehaengt; `NSRunLoopCommonModes` ist ein Fremdsymbol
        // von Foundation. Dieselbe Form wie der Einzugstakt des
        // Dateifensters.
        let zeitgeber = unsafe {
            let zeitgeber = NSTimer::timerWithTimeInterval_target_selector_userInfo_repeats(
                LADETAKT,
                self,
                sel!(ladenEinziehen:),
                None,
                true,
            );
            NSRunLoop::currentRunLoop().addTimer_forMode(&zeitgeber, NSRunLoopCommonModes);
            zeitgeber
        };
        *self.ivars().takt.borrow_mut() = Some(zeitgeber);
    }

    /// Nimmt den Zeitgeber aus der Laufschleife und loest den Ring auf.
    fn takt_beenden(&self) {
        if let Some(zeitgeber) = self.ivars().takt.borrow_mut().take() {
            zeitgeber.invalidate();
        }
    }
}

/// Baut die Textanzeige: eine nicht auswaehlbare `NSTextView` in einer
/// Bildlaufansicht.
///
/// Nicht auswaehlbar aus dem Grund im Modulkopf: eine auswaehlbare naehme als
/// Textsystem den Fokus, und der Ereignisabgriff reichte jede Taste weiter.
/// Die Schrift ist die feste Schreibmaschinenschrift des Nutzers, weil C6 die
/// Anzeige als **rohen** Inhalt ohne Formatierung zusagt.
fn textanzeige(
    mtm: MainThreadMarker,
    rahmen: NSRect,
) -> (Retained<NSScrollView>, Retained<NSTextView>) {
    let rolle = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), rahmen);
    rolle.setHasVerticalScroller(true);
    rolle.setAutohidesScrollers(true);
    rolle.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
    );

    let text = NSTextView::initWithFrame(NSTextView::alloc(mtm), rahmen);
    text.setEditable(false);
    text.setSelectable(false);
    text.setVerticallyResizable(true);
    text.setHorizontallyResizable(false);
    text.setMinSize(NSSize::ZERO);
    text.setMaxSize(NSSize::new(f64::MAX, f64::MAX));
    text.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
    if let Some(schrift) = NSFont::userFixedPitchFontOfSize(NSFont::smallSystemFontSize()) {
        text.setFont(Some(&schrift));
    }
    rolle.setDocumentView(Some(&text));
    (rolle, text)
}
