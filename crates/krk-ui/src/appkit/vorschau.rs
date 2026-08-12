//! Das Vorschaufenster: Tableiste, Text- und Bildanzeige, angebunden an das
//! Modell aus [`crate::vorschaumodell`] (C6, C10).
//!
//! ```text
//! ┌──────────────────────────────┐
//! │ Tableiste (appkit::tableiste)│  ein Abschnitt je Vorschau-Tab
//! ├──────────────────────────────┤
//! │ Inhaltsflaeche               │  nimmt Klick und Fokus entgegen
//! │   NSScrollView + NSTextView  │  Text, Metadaten, Hinweise
//! │     + Nummernspalte          │  nur beim rohen Inhalt einer Datei (C10)
//! │   NSImageView                │  Bilder; je einer von beiden sichtbar
//! └──────────────────────────────┘
//! ```
//!
//! **Die Nummernspalte ist dieselbe Klasse wie im Editor.**
//! [`super::nummernspalte`] haelt sie, und C10 sagt eine Anzeige fuer beide
//! Flaechen zu und nicht zwei aehnliche. Ob sie steht, entscheidet
//! [`Vorschaumodell::zeigt_dateitext`] und sonst nichts: sie steht beim rohen
//! Inhalt einer Textdatei und weder bei einem Bild noch bei Metadaten, einem
//! Hinweis, einem leeren Tab oder dem Text aus der Zwischenablage.
//!
//! **Die Vorschau stammt aus der Runde 1 und wird mit der Nummernspalte zum
//! ersten Mal seit ihrem Abschluss erweitert.** Der Nutzer hat sie am
//! 260809-2035 ausdruecklich hereingeholt; die Ausklammerung der Restarbeit
//! jener Runde gilt den Messreihen und nicht jeder Beruehrung. Eine davon ist
//! benannt und wird nicht verschwiegen: **L7** misst die Vorschau einer
//! Textdatei, und die Spalte haengt in genau dieser Flaeche. Eine Zahl steht
//! hier nicht; der Spec uebergibt L7 an die spaetere Messrunde.
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
//! Vorschau-Tabs. Drei von ihnen tragen dafuer
//! [`Wirkungsbereich::Tabbereich`](krk_core::tasten::Wirkungsbereich); der
//! vierte, `tab_schliessen`, traegt seit C4 der Runde 4
//! [`Wirkungsbereich::Ueberall`](krk_core::tasten::Wirkungsbereich) und
//! erreicht diese Tabs ueber die Verzweigung nach dem Fokus im
//! Anwendungsdelegierten.
//!
//! Die Textanzeige ist dafuer nicht auswaehlbar: eine auswaehlbare naehme den
//! Fokus als Textsystem, und der Ereignisabgriff reichte jede Taste an AppKit
//! weiter, statt die Tabbefehle auszufuehren. Einen Tastenbefehl, der den
//! Fokus hierher setzt, gibt es in dieser Runde nicht; die offene Frage dazu
//! liegt im Entscheidungsspeicher.
//!
//! **Das Kontextmenue haengt an allen drei Ansichten, und diese Datei baut es
//! nicht.** Seit C1 der Runde 6 ist das Vorschaufenster der Delegierte seiner
//! Textanzeige und der seines Menues; es beantwortet allein, welche Datei der
//! aktive Tab zeigt, und laesst [`super::teilen::eintrag_anfuegen`] den
//! Eintrag setzen. Warum drei und nicht eine, und warum auf zwei
//! Anschlussarten, steht am Aufbau weiter unten und im Kopf jenes Moduls.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSView`, `NSScrollView`, `NSTextView`, `NSImageView`, `NSImage`, `NSFont`,
//! `NSEvent`, `NSTimer`, `NSRunLoop`, `NSDate`, `NSDateFormatter`, `NSData`
//! und `NSString` stehen seit macOS 10.0 zur Verfuegung, seit C1 der Runde 6
//! ebenso `NSMenu`, die Eigenschaft `menu` von `NSResponder`
//! (`NSResponder.h:111`), `NSMenu`s Setzer `delegate` (`NSMenu.h:156`) und die
//! drei angenommenen Protokolle `NSMenuDelegate` (`NSMenu.h:269`) samt
//! `menuNeedsUpdate:` (`:271`), `NSTextDelegate` (`NSText.h:200`) und
//! `NSTextViewDelegate` (`NSTextView.h:576`). Einzig `NSByteCountFormatter` ist juenger
//! als seine Nachbarn und steht seit 10.8 (`NSByteCountFormatter.h:38`). Das
//! Buendel zielt auf 15.0 (`.cargo/config.toml`).
//!
//! **Drei Beruehrungen tragen daneben eine eigene Angabe**:
//! `NSRunLoopCommonModes` steht seit 10.5 (`NSRunLoop.h:14`) und die
//! Delegiertenmethode `textView:menu:forEvent:atIndex:` ebenfalls seit 10.5
//! (`NSTextView.h:628`); `NSMenu`s `removeAllItems` steht seit 10.6
//! (`NSMenu.h:112`). Alles uebrige —
//! `setRulersVisible:`, `setImageScaling:`, `initWithData:`,
//! `userFixedPitchFontOfSize:`, `smallSystemFontSize`, `addTimer:forMode:`,
//! `dateWithTimeIntervalSince1970:` und der fuenfteilige Zeitgeberaufruf
//! `timerWithTimeInterval:target:selector:userInfo:repeats:` — traegt im Kopf
//! des Systems keine Verfuegbarkeitsangabe und steht damit seit 10.0; ebenso
//! drei der vier angesprochenen Aufzaehlungen — `NSAutoresizingMaskOptions`,
//! `NSDateFormatterStyle` und `NSByteCountFormatterCountStyle` —, deren Werte
//! ebenfalls keine eigene Angabe tragen. Die vierte, `NSImageScaling`, traegt
//! an ihrer schliessenden Klammer `API_AVAILABLE(macos(10.5))`
//! (`NSCell.h`); ihre Werte tragen keine.
//!
//! Keine von ihnen ist nach macOS 15 hinzugekommen, und keine Beruehrung in
//! dieser Datei braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit.
//! `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer
//! haelt die Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{AnyThread, DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSEvent, NSFont, NSImage, NSImageScaling, NSImageView, NSMenu,
    NSMenuDelegate, NSScrollView, NSTextDelegate, NSTextView, NSTextViewDelegate, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSByteCountFormatter, NSByteCountFormatterCountStyle, NSData, NSDate,
    NSDateFormatter, NSDateFormatterStyle, NSObject, NSObjectProtocol, NSPoint, NSRect, NSRunLoop,
    NSRunLoopCommonModes, NSSize, NSString, NSTimeInterval, NSTimer, NSUInteger,
};

use krk_core::tasten::Kommando;
use krk_core::verzeichnis::Typ;

use crate::vorschaumodell::{Inhalt, Metadaten, Vorschaumodell, Zwischenablageinhalt, rechte_text};

use super::nummernspalte::Nummernspalte;
use super::tabelle::typ_beschriften;
use super::tableiste::{self, Tableiste};
use super::teilen;

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

    // SAFETY: `NSTextDelegate` stellt keine Bedingungen. Er steht hier allein,
    // weil `NSTextViewDelegate` ihn voraussetzt; keine seiner Methoden wird
    // beantwortet. Die Textanzeige der Vorschau ist nicht bearbeitbar, es gibt
    // also keine Aenderung zu melden.
    unsafe impl NSTextDelegate for Vorschaufenster {}

    // SAFETY: `NSTextViewDelegate` stellt keine Bedingungen. Die Textflaeche
    // haelt ihren Delegierten schwach ("This is a weak property",
    // `objc2-app-kit-0.3.2/src/generated/NSTextView.rs:1258-1263`), und das
    // Vorschaufenster haelt die Flaeche stark; ein Ring entsteht deshalb nicht.
    // Dieselbe Anbindung wie im Editor.
    unsafe impl NSTextViewDelegate for Vorschaufenster {
        /// Haengt den Teilen-Eintrag in das Kontextmenue der Textanzeige
        /// (C1 der Runde 6, sechstes Kriterium).
        ///
        /// **Derselbe Weg wie im Editor, und aus demselben Grund**: eine
        /// `NSTextView` baut ihr Kontextmenue selbst, und dieser Haken
        /// **ergaenzt** es, statt es zu ersetzen. Was AppKit einer nicht
        /// auswaehlbaren Anzeige gibt, ist wenig bis nichts; es bleibt
        /// trotzdem stehen. Die zweite Anschlussart, `setMenu:`, nehmen die
        /// Bildansicht und die Inhaltsflaeche, weil sie kein eigenes Menue
        /// bauen — beide stehen im Kopf von [`super::teilen`] nebeneinander.
        // SAFETY: Die Signatur entspricht der des Protokolls
        // (`NSTextView.h:628`).
        #[unsafe(method_id(textView:menu:forEvent:atIndex:))]
        fn kontextmenue(
            &self,
            _flaeche: &NSTextView,
            menue: &NSMenu,
            _ereignis: &NSEvent,
            _stelle: NSUInteger,
        ) -> Option<Retained<NSMenu>> {
            teilen::eintrag_anfuegen(menue, &self.teilbare_pfade(), self.mtm());
            Some(menue.retain())
        }
    }

    // SAFETY: `NSMenuDelegate` stellt keine Bedingungen. Das Menue haelt seinen
    // Delegierten **schwach** (`NSMenu.h:156`, "This is a weak property" in
    // `objc2-app-kit-0.3.2/src/generated/NSMenu.rs:356-361`), die beiden
    // Ansichten halten das Menue stark, und das Vorschaufenster haelt die
    // Ansichten. Der Ring bleibt an der Kante Menue → Delegierter offen.
    unsafe impl NSMenuDelegate for Vorschaufenster {
        /// Baut das Kontextmenue der Bildansicht und der Inhaltsflaeche, bei
        /// jedem Rechtsklick neu (C1 der Runde 6, sechstes Kriterium).
        ///
        /// **Ein Menue fuer beide Ansichten, und eine Methode fuer beide.**
        /// Welche der beiden angeklickt wurde, aendert nichts an der Antwort:
        /// geteilt wird die Datei des aktiven Tabs, ob sie gerade als Bild
        /// oder als Text dasteht. Eine Verzweigung nach der Ansicht waere eine
        /// zweite Regel ohne zweite Frage.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(menuNeedsUpdate:))]
        fn menue_auffrischen(&self, menue: &NSMenu) {
            let pfade = self.teilbare_pfade();
            menue.removeAllItems();
            teilen::eintrag_anfuegen(menue, &pfade, self.mtm());
        }
    }

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

        // Das Kontextmenue aus C1 der Runde 6, an allen drei Ansichten. Es
        // steht hier und nicht in `bauen`s erster Haelfte, weil es das Objekt
        // erst ab dem `init` weiter oben gibt; dieselbe Reihenfolge wie beim
        // Rueckruf der Tableiste darueber und beim Delegierten des Editors.
        //
        // **Alle drei bekommen es, und nicht die eine, auf der der Klick nach
        // unserer Vermutung landet.** Wo ein Rechtsklick in der Vorschau
        // ankommt, haengt am Inhalt: auf der Textanzeige, auf der Bildansicht
        // oder auf der Inhaltsflaeche dahinter. Ob eine Ansicht ohne eigenes
        // Menue die rechte Maustaste an ihre Uebergeordnete weiterreicht, ist
        // eine Zusage von AppKit, die wir nicht gelesen haben, und eine
        // Flaeche ohne Menue waere der stille Fehlschlag, den C1 ausschliesst.
        //
        // Die Textanzeige geht ihren eigenen Weg, `textView:menu:forEvent:atIndex:`
        // weiter oben; die beiden anderen teilen sich **ein** Menue. Ein
        // zweites daneben traege denselben einen Eintrag und braeuchte
        // denselben Delegierten, waere also eine Wiederholung ohne Unterschied.
        this.ivars()
            .text
            .setDelegate(Some(ProtocolObject::from_ref(&*this)));
        let kontextmenue = NSMenu::new(mtm);
        kontextmenue.setDelegate(Some(ProtocolObject::from_ref(&*this)));
        // SAFETY: `setMenu:` ist als Setzer einer `strong`-Eigenschaft unsicher
        // gebunden und verlangt nichts weiter, als dass das Menue eines ist.
        // Beide Ansichten halten es danach; dasselbe Objekt zweimal zu setzen
        // ist zulaessig, weil ein Kontextmenue kein Untermenue ist und keinen
        // Elternteil hat.
        unsafe {
            this.ivars().inhaltsflaeche.setMenu(Some(&kontextmenue));
            this.ivars().bild.setMenu(Some(&kontextmenue));
        }

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
    /// Nur zum Ablesen. Drei fragen danach: die Endbedingung von L7 im
    /// Messmodus, [`crate::angezeigtedatei::welche`] ueber den
    /// Anwendungsdelegierten, und das Kontextmenue dieser Datei ueber
    /// [`Self::teilbare_pfade`].
    pub fn angezeigter_pfad(&self) -> Option<std::path::PathBuf> {
        self.ivars().modell.borrow().aktiver_pfad()
    }

    /// Was ein Rechtsklick in der Vorschau zu teilen findet (C1 der Runde 6).
    ///
    /// Keine oder eine Datei, nie mehr: die Vorschau zeigt einen Tab, und der
    /// zeigt hoechstens eine Datei. Zeigt er etwas anderes — Metadaten, einen
    /// Hinweis, den Inhalt der Zwischenablage, gar nichts —, bleibt die Liste
    /// leer, und [`teilen::eintrag_anfuegen`] setzt dann keinen Eintrag.
    ///
    /// **Die Sichtbarkeit der Vorschau wird hier nicht gefragt.** Das Menue
    /// geht nur auf, wo der Nutzer hinklickt, und geklickt hat er in die
    /// sichtbare Vorschau; [`crate::angezeigtedatei::welche`] beantwortete
    /// eine Frage, die der Klick schon beantwortet hat. Die Ausleihe des
    /// Modells endet mit dieser Zeile, vor jedem Objective-C-Aufruf.
    fn teilbare_pfade(&self) -> Vec<PathBuf> {
        self.angezeigter_pfad().into_iter().collect()
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
    ///
    /// **Der Klon kopiert keine Bilddatei.** [`Inhalt::Bild`] haelt seine
    /// Bytes seit dem 260806 in einem `Arc`, und der Klon hier ist fuer sie
    /// ein Zaehlerschritt; vorher entstand bei jedem Neuzeichnen eine zweite
    /// vollstaendige Kopie. Die Begruendung steht am Feld selbst.
    fn anzeigen(&self) {
        let (titel, aktiv, inhalt, zeigt_nummern) = {
            let modell = self.ivars().modell.borrow();
            (
                modell.titel(),
                modell.aktive_stelle(),
                modell.aktiver_inhalt().clone(),
                modell.zeigt_dateitext(),
            )
        };

        // Die Nummernspalte aus C10, an derselben Stelle geschaltet, an der
        // Textrolle und Bildansicht sich gegenseitig verbergen. Entschieden
        // wird nichts hier: `zeigt_dateitext` ist die eine Stelle, die die
        // Frage beantwortet. Nur beim Wechsel gesetzt, weil `setRulersVisible:`
        // die Bildlaufansicht neu auslegt.
        let rolle = &self.ivars().textrolle;
        if rolle.rulersVisible() != zeigt_nummern {
            rolle.setRulersVisible(zeigt_nummern);
        }

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
    // **Dieselbe Klasse, die der Editor einhaengt** (C10), und keine zweite
    // Spalte daneben. Ob sie steht, entscheidet `Vorschaufenster::anzeigen`
    // ueber `setRulersVisible`; hier entsteht sie nur.
    Nummernspalte::einhaengen(mtm, &rolle, &text);
    (rolle, text)
}
