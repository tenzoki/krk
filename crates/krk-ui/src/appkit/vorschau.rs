//! Das Vorschaufenster: Tableiste, Text- und Bildanzeige, angebunden an das
//! Modell aus [`crate::vorschaumodell`] (C6, C10).
//!
//! ```text
//! ┌──────────────────────────────┐
//! │ Tableiste (appkit::tableiste)│  ein Abschnitt je Vorschau-Tab
//! ├──────────────────────────────┤
//! │ Inhaltsflaeche               │  nimmt Klick und Fokus entgegen
//! │   NSScrollView + Vorschautext│  Text, Metadaten, Hinweise; auswaehlbar
//! │     + Nummernspalte          │  nur beim rohen Inhalt einer Datei (C10)
//! │   NSImageView                │  Bilder; je einer von beiden sichtbar
//! └──────────────────────────────┘
//! ```
//!
//! **Die Nummernspalte ist dieselbe Klasse wie im Editor.**
//! [`super::nummernspalte`] haelt sie, und C10 sagt eine Anzeige fuer beide
//! Flaechen zu und nicht zwei aehnliche. Ob sie steht, entscheidet
//! [`Vorschaumodell::zeigt_dateitext`] und sonst nichts: sie steht beim rohen
//! Inhalt einer Textdatei und weder bei einem Bild noch bei Metadaten, einer
//! Zusammenfassung, einem Hinweis, einem leeren Tab oder dem Text aus der
//! Zwischenablage.
//!
//! # Die Zusammenfassung eines erkannten Ordners
//!
//! Seit der Runde 16 tritt sie an die Stelle der Metadaten, sobald ein
//! Leseprofil den ausgewaehlten Ordner erkennt (C4.1, C4.2). **Sie nimmt
//! denselben Weg an die Flaeche wie Metadaten und Hinweise**:
//! [`als_text`](krk_core::leseprofil::Zusammenfassung::als_text) macht aus
//! ihren Werten Zeilen, und [`Vorschaufenster::text_zeigen`] stellt sie hin;
//! ein zweiter Ausgabeweg entsteht fuer sie nicht. Eine Nummernspalte traegt
//! sie nicht, und der Grund steht im Absatz darueber: die Zahlen zaehlten die
//! Zeilen der Zusammenfassung, und daneben steht keine Datei mit diesen
//! Zeilen. **Ihre Auswaehlbarkeit gilt aus der Runde 14 unveraendert weiter**
//! (C4.6) — sie faellt aus dem Weg heraus, den sie nimmt, und nicht aus einer
//! Regel, die fuer sie geschrieben waere; der Zweig in
//! [`Vorschaufenster::anzeigen`] schreibt aus, warum.
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
//! Dateifenster: ein Zeitgeber auf dem Hauptfaden raeumt die Kanaele leer und
//! endet, sobald keiner von ihnen mehr etwas zu liefern hat. Derselbe Takt
//! wie der Einzugstakt aus [`super::tabelle`]. Seit C4 der Runde 6 sind es
//! **zwei** Kanaele — das Laden und die Einfaerbung — und weiterhin **ein**
//! Zeitgeber.
//!
//! # Der Einfaerbungsvorgang wohnt hier und nicht im Modell
//!
//! ```text
//!   Arbeitsfaden krk-vorschau        │  Arbeitsfaden krk-einfaerbung
//!   lesen, Markdown rendern          │  syntect, 0,3 MB/s (gemessen)
//!            │                       │           │
//!            v                       │           v
//!   Vorschaumodell::laedt_noch  ──> Text steht ──> Farben ziehen nach
//!            ^                       │
//!            └── Endbedingung L7 ────┘
//! ```
//!
//! **[`Vorschaumodell::laedt_noch`] weiss von der Einfaerbung nichts, und das
//! ist die tragende Zusage dieses Schnitts.** Es beantwortet weiter allein
//! "wartet ein Tab auf seinen Text", und daran haengt die Endbedingung von
//! **L7**, einer der zehn Zeitzusagen aus C8 der Runde 1. Ein
//! Einfaerbungsvorgang im Modell liesse L7 auf `syntect` warten und machte aus
//! rund 100 ms bei 1 MB ueber drei Sekunden. Deshalb halten
//! [`VorschaufensterIvars::einfaerbung`] und
//! [`VorschaufensterIvars::einfaerbungsstand`] den Vorgang hier, in der
//! Ansicht, und `crate::vorschaumodell` nennt weder das eine noch das andere.
//!
//! **Der Text erscheint sofort, die Farben ziehen sichtbar nach** (C4, elftes
//! Kriterium der Runde 6). Das ist keine Einschraenkung, sondern die Form, in
//! der die Zusage einzuhalten ist.
//!
//! **Ein Verweis bekommt Farbe und Unterstreichung, aber keine Klickwirkung
//! und keinen Zeigefinger** (C4, siebtes Kriterium). Beides kaeme von
//! `NSLinkAttributeName`, und dieses Merkmal wird ausdruecklich **nicht**
//! gesetzt: welche Quellen eine Adresse setzen duerfen, ist die erste offene
//! Frage des Circles `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`,
//! und sie hier nebenbei zu beantworten naehme jenem Circle seine
//! Klaerungsrunde. Farbe und Unterstreichung kommen als voruebergehende
//! Merkmale des Layoutverwalters, und die tragen keine Wirkung.
//!
//! # Warum die Vorschau beide Werte von `Ansicht` benutzt
//!
//! Sie kennt keinen Ansichtswechsel — es gibt keinen Befehl dafuer und keine
//! Wahl des Nutzers. Sie uebergibt [`Ansicht`] trotzdem in beiden Werten, und
//! zwar als Aussage ueber den **Inhalt**: was dasteht, wie es gelesen wurde,
//! bekommt die Besetzung der Rohansicht ([`Vorschaufenster::text_zeigen`]);
//! was ausgezeichnet oder eingefaerbt wird, die der Formatansicht
//! ([`Vorschaufenster::formatierung_anwenden`]). Fuer Quelltext sind beide
//! dieselbe Schrift, weil [`textmerkmale::grundschrift`] dort nicht
//! unterscheidet — deshalb springt die Anzeige nicht, wenn die Farben
//! nachziehen.
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
//! # Die Textanzeige ist auswaehlbar, und das loest eine Zusage ab
//!
//! **Bis zur Runde 14 stand hier `setSelectable(false)`.** Die Runde 6 hatte
//! das ausdruecklich zugesagt (C4, achtes Kriterium: „die beiden Schalter
//! bleiben, wo sie stehen"). Der Nutzer hat diese Zusage am 260819
//! **ersetzt und nicht ergaenzt**: der Text der Vorschau soll zu markieren und
//! zu kopieren sein (C1.1 der Runde 14). Der Schalter ist damit gefallen, und
//! diese Datei setzt `setSelectable(true)`.
//!
//! Der Grund, aus dem er stand, ist dabei nicht widerlegt, sondern bezahlt:
//! eine auswaehlbare Flaeche nimmt den Fokus als Textsystem, und
//! [`super::ereignisse::ersthelfer_gehoert_appkit`] reicht dann jede Taste an
//! AppKit weiter, statt die vier Tabbefehle auszufuehren. Die Gegenmassnahme
//! ist dieselbe, mit der der Editor seit der Runde 2 lebt: der
//! Anwendungsdelegierte kennt die eigenen Textflaechen von KRK **namentlich**
//! und nimmt sie von der Weitergabe aus (C1.7 der Runde 14). Ohne diese
//! Anmeldung waere die Auswahl mit den Tabbefehlen erkauft.
//!
//! **`setEditable(false)` bleibt stehen — aus einem anderen Grund als dem
//! gefallenen Schalter, und deshalb faellt es nicht mit ihm.** Die
//! Nichtauswaehlbarkeit war ein **Mittel** gegen den Fokus, und ein Mittel
//! wird hinfaellig, sobald ein besseres danebensteht. Die
//! Nichtbearbeitbarkeit ist keines, sondern eine Aussage darueber, was die
//! Vorschau **ist**: sie zeigt und bearbeitet nicht (C1.4 der Runde 14, und
//! unveraendert seit C6 der Runde 1). Wer beide Zeilen fuer dieselbe Sache
//! haelt, nimmt mit dem einen Schalter den anderen mit und macht aus der
//! Vorschau einen zweiten Editor.
//!
//! Einen Tastenbefehl, der den Fokus hierher setzt, gibt es weiterhin nicht;
//! die offene Frage dazu liegt im Entscheidungsspeicher.
//!
//! **Was die Auswahl beim Kopieren hergibt, entscheidet eine einzige Stelle**:
//! [`Vorschautext::auswahl_ablegen`], die Ueberschreibung von
//! `writeSelectionToPasteboard:types:`. Bei gerendertem Markdown geht der
//! **Quelltext** mit seinen Auszeichnungszeichen heraus (C2.2 der Runde 14),
//! sonst der Text, wie er dasteht (C2.1). Kopieren ist dabei kein Befehl von
//! KRK: der Menueeintrag traegt `copy:` und Ziel `nil`, und die Antwortkette
//! entscheidet, wer ihn beantwortet — seit dieser Runde auch diese Flaeche.
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
//! `NSView`, `NSScrollView`, `NSTextView`, `NSImageView`, `NSImage`,
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
//! **Textspeicher und Layoutverwalter fasst diese Datei nicht selbst an.** Sie
//! kommen ueber [`super::textmerkmale`], und die Angaben stehen im Kopf jener
//! Datei; genannt seien sie hier trotzdem, weil die Beruehrung ueber den
//! Aufruf von [`textmerkmale::anwenden`], [`textmerkmale::zuruecksetzen`] und
//! [`textmerkmale::grundschrift`] wirklich stattfindet: `NSFont` steht seit
//! macOS 10.0 (`NSFont.h:24`), `NSTextStorage` seit macOS 10.0
//! (`NSTextStorage.h:37`), **`NSLayoutManager` seit macOS 10.7**
//! (`NSLayoutManager.h:65`, am SDK gelesen — nicht seit 10.0, wie zwei aeltere
//! Modulkoepfe dieses Verzeichnisses es nannten).
//!
//! **Vier Beruehrungen tragen daneben eine eigene Angabe**:
//! `NSRunLoopCommonModes` steht seit 10.5 (`NSRunLoop.h:14`) und die
//! Delegiertenmethode `textView:menu:forEvent:atIndex:` ebenfalls seit 10.5
//! (`NSTextView.h:628`); `NSMenu`s `removeAllItems` steht seit 10.6
//! (`NSMenu.h:112`), und `viewDidChangeEffectiveAppearance` — die Meldung, an
//! der seit C4 der Runde 6 die Farbtafel haengt — seit 10.14
//! (`NSView.h:378`). Alles uebrige —
//! `setRulersVisible:`, `setImageScaling:`, `initWithData:`, `setFont:`,
//! `addTimer:forMode:`,
//! `dateWithTimeIntervalSince1970:` und der fuenfteilige Zeitgeberaufruf
//! `timerWithTimeInterval:target:selector:userInfo:repeats:` — traegt im Kopf
//! des Systems keine Verfuegbarkeitsangabe und steht damit seit 10.0; ebenso
//! drei der vier angesprochenen Aufzaehlungen — `NSAutoresizingMaskOptions`,
//! `NSDateFormatterStyle` und `NSByteCountFormatterCountStyle` —, deren Werte
//! ebenfalls keine eigene Angabe tragen. Die vierte, `NSImageScaling`, traegt
//! an ihrer schliessenden Klammer `API_AVAILABLE(macos(10.5))`
//! (`NSCell.h`); ihre Werte tragen keine.
//!
//! **Die Runde 14 spricht keine juengere Klasse an, und das ist am SDK
//! nachgelesen und nicht geschlossen.** [`Vorschautext`] ist eine Unterklasse
//! von `NSTextView` (`NSTextView.h:76`), erzeugt ueber dessen `initWithFrame:`
//! (`NSTextView.h:86`); die beiden Schalter `setEditable:` und
//! `setSelectable:` sind die Setzer der Eigenschaften `editable` und
//! `selectable` von `NSText` (`NSText.h:89-90`). Keine der vier Stellen traegt
//! im Kopf des Systems eine Verfuegbarkeitsangabe und steht damit seit 10.0.
//! Genannt seien sie trotzdem: `setSelectable:` wechselt in dieser Runde sein
//! Argument, und `initWithFrame:` wird ab jetzt ueber `super` und nicht mehr
//! am fertigen `NSTextView` gerufen. Dazu kommt der **Leser** einer
//! Eigenschaft, deren Setzer diese Datei schon lange ruft: `isHidden` ist der
//! Getter von `hidden` aus `NSView` (`NSView.h:92`), traegt dort ebenfalls
//! keine Verfuegbarkeitsangabe und steht damit seit 10.0.
//! [`Vorschaufenster::fokusansicht`] fragt ihn seit der Runde 14.
//!
//! **Die Abfangstelle des Kopierens bringt vier weitere Beruehrungen mit, und
//! keine davon ist juenger.** `writeSelectionToPasteboard:types:` steht in der
//! Kategorie `NSTextView (NSPasteboard)` (`NSTextView.h:258`) an
//! `NSTextView.h:277`; weder die Kategorie noch die Methode traegt eine
//! Verfuegbarkeitsangabe, beide stehen damit seit 10.0. `selectedRange` ist
//! die Eigenschaft von `NSText` (`NSText.h:100`), ebenfalls ohne Angabe und
//! damit seit 10.0. `NSPasteboard` steht seit 10.0 (`NSPasteboard.h:157`), der
//! Typaliasname `NSPasteboardType` ebenso (`NSPasteboard.h:23`), und `NSArray`
//! seit 10.0 (`NSArray.h:17`). **Die einzelnen Sortennamen** wie
//! `NSPasteboardTypeString` tragen dagegen `API_AVAILABLE(macos(10.6))`; diese
//! Datei nennt keinen davon, sie reicht die Ablage an
//! [`super::zwischenablage::text_auf_ablage_schreiben`] weiter, und die Angabe
//! steht im Kopf jener Datei.
//!
//! Keine von ihnen ist nach macOS 15 hinzugekommen, und keine Beruehrung in
//! dieser Datei braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit.
//! `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer
//! haelt die Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.

use std::cell::{Cell, OnceCell, RefCell};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use objc2::rc::{Retained, Weak};
use objc2::runtime::ProtocolObject;
use objc2::{AnyThread, DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSEvent, NSImage, NSImageScaling, NSImageView, NSMenu,
    NSMenuDelegate, NSPasteboard, NSPasteboardType, NSScrollView, NSTextDelegate, NSTextView,
    NSTextViewDelegate, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSByteCountFormatter, NSByteCountFormatterCountStyle, NSData,
    NSDate, NSDateFormatter, NSDateFormatterStyle, NSObject, NSObjectProtocol, NSPoint, NSRect,
    NSRunLoop, NSRunLoopCommonModes, NSSize, NSString, NSTimeInterval, NSTimer, NSUInteger,
};

use krk_core::leseprofil::{Profile, Zusammenfassungszeile, zeilen_als_text};
use krk_core::tasten::Kommando;
use krk_core::verzeichnis::Typ;

use crate::editormodell::{Ansicht, Dateityp};
use crate::hervorhebung::{
    self, Abholung, Darstellungsart, Einfaerbungsstand, Einfaerbungsvorgang, Formatierung, Tafel,
};
use crate::markdown::Quellbezug;
use crate::vorschaumodell::{Inhalt, Metadaten, Vorschaumodell, Zwischenablageinhalt, rechte_text};

use super::nummernspalte::{self, Nummernspalte};
use super::tabelle::typ_beschriften;
use super::tableiste::{self, Tableiste};
use super::teilen;
use super::textmerkmale;
use super::zwischenablage;

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
    /// Die Flaeche unter der Tableiste: sie nimmt Klick und Fokus entgegen —
    /// und die Meldung ueber den Wechsel des Erscheinungsbildes (C4 der
    /// Runde 6).
    ///
    /// Ein eigener Ersthelfer, damit [`Fokus::Vorschau`](crate::kommandos::fokus::Fokus)
    /// ueberhaupt eintreten kann: die Bildanzeige lehnt den Fokus ab, und ihr
    /// Klick faellt durch die Antwortkette hierher.
    ///
    /// **Fuer die Textanzeige gilt das seit der Runde 14 nicht mehr.**
    /// [`Vorschautext`] ist auswaehlbar (C1.1) und nimmt den Ersthelferrang
    /// selbst, sobald jemand hineinklickt; der Modulkopf sagt, welche Zusage
    /// damit abgeloest ist. Diese Flaeche bleibt trotzdem, und aus zwei
    /// Gruenden: sie traegt die Meldung ueber das Erscheinungsbild (unten),
    /// und sie faengt den Klick auf ein Bild und auf den leeren Rand.
    ///
    /// **Die zweite Aufgabe kam mit der Einfaerbung.**
    /// `viewDidChangeEffectiveAppearance` ist die eine Stelle, die AppKit fuer
    /// die Frage "hat das System auf Dunkel umgestellt" vorsieht, und sie ist
    /// eine Methode einer Ansicht; das [`Vorschaufenster`] ist keine, sondern
    /// ein `NSObject`. Genommen wird diese Flaeche und keine zweite daneben:
    /// sie ist ohnehin da, und das wirksame Erscheinungsbild ist fuer jede
    /// Ansicht derselben Kette dasselbe. Dieselbe Bauart traegt
    /// [`Editorsicht`](super::editor) im Editor.
    ///
    /// Die Rueckverbindung ist **schwach**, sonst schloesse sich der Ring
    /// Vorschaufenster → Flaeche → Rueckverweis → Vorschaufenster.
    // SAFETY:
    // - Die Oberklasse NSView stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = RefCell<Option<Weak<Vorschaufenster>>>]
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

        /// Das System hat auf Hell oder Dunkel umgestellt (C4 der Runde 6).
        // SAFETY: Die Signatur entspricht der von NSView.
        #[unsafe(method(viewDidChangeEffectiveAppearance))]
        fn erscheinung_gewechselt(&self) {
            // SAFETY: Die Oberklasse beantwortet dieselbe Nachricht ohne
            // Argument und ohne Rueckgabe. Sie zuerst, weil AppKit hinter
            // dieser Methode die Erscheinung der Unteransichten nachzieht und
            // KRK danach eine bereits umgestellte Flaeche vorfindet.
            let _: () = unsafe { msg_send![super(self), viewDidChangeEffectiveAppearance] };
            let vorschau = self.ivars().borrow().as_ref().and_then(Weak::load);
            if let Some(vorschau) = vorschau {
                vorschau.erscheinung_nachziehen();
            }
        }
    }
);

impl Inhaltsflaeche {
    /// Eine Flaeche mit dem genannten Rahmen, noch ohne Rueckverweis.
    fn neu(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(RefCell::new(None));
        // SAFETY: `initWithFrame:` von NSView hat die hier angenommene
        // Signatur.
        unsafe { msg_send![super(this), initWithFrame: rahmen] }
    }

    /// Traegt den Rueckverweis nach, sobald es das Vorschaufenster gibt.
    fn ziel_setzen(&self, vorschau: &Vorschaufenster) {
        *self.ivars().borrow_mut() = Some(Weak::from_retained(&vorschau.retain()));
    }
}

define_class!(
    /// Die Textanzeige der Vorschau: eine `NSTextView`, die weiss, aus welcher
    /// Quelle ihr Text gerendert wurde (C1.1 und C2 der Runde 14).
    ///
    /// **Warum eine eigene Klasse und nicht die nackte `NSTextView` von
    /// vorher.** Sobald die Flaeche auswaehlbar ist, fuehren `copy:`, der
    /// Eintrag des Kontextmenues, die Dienste des Systems und das Ziehen einer
    /// Auswahl in AppKit an einer Stelle zusammen:
    /// `writeSelectionToPasteboard:types:`. Nur eine Unterklasse kommt an
    /// diese Stelle heran; ein Delegiertenweg oder ein Abfangen vor der
    /// Antwortkette erreichte jeweils nur einen Teil der Wege. Wie weit diese
    /// Zusammenfuehrung wirklich traegt, steht am Doc-Kommentar der
    /// Ueberschreibung selbst: dort ist getrennt, was Apples Beschreibung sagt
    /// und was an diesem Baum gemessen ist.
    ///
    /// Ihre eine Ueberschreibung ist [`Vorschautext::auswahl_ablegen`]; sonst
    /// ist die Klasse in ihrem Verhalten eine `NSTextView` und nichts weiter.
    ///
    /// **Der Merkposten haelt einen [`Arc`] und keine Kopie.** Der
    /// [`Quellbezug`] entsteht im Durchgang, der rendert, und liegt bereits im
    /// [`Inhalt`] des Tabs; ihn hier ein zweites Mal aufzubauen hiesse, die
    /// Quelle der Datei ein zweites Mal im Speicher zu halten. `None` heisst
    /// „was hier steht, ist kein gerendertes Markdown" — roher Text,
    /// eingefaerbter Quelltext, Metadaten, ein Hinweis, ein leerer Tab oder
    /// der Text aus der Zwischenablage.
    // SAFETY:
    // - Die Oberklasse NSTextView stellt an eine Unterklasse keine Bedingung,
    //   die diese Klasse verletzen koennte: sie ruft den bezeichneten Erzeuger
    //   der Oberklasse, und ihre eine Ueberschreibung reicht jeden Fall, den
    //   sie nicht selbst beantwortet, unveraendert an die Oberklasse weiter.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSTextView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = RefCell<Option<Arc<Quellbezug>>>]
    pub struct Vorschautext;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Vorschautext {}

    impl Vorschautext {
        /// Der eine Ausgang jeder Auswahl aus der Vorschau (C2.1, C2.2, C2.12
        /// der Runde 14).
        ///
        /// **Zwei Zweige, und der Merkposten entscheidet.** Liegt kein
        /// [`Quellbezug`] bei, steht hier kein gerendertes Markdown, und die
        /// Oberklasse legt Zeichen fuer Zeichen ab, was markiert war — roher
        /// Text, eingefaerbter Quelltext, Metadaten, ein Hinweis, ein leerer
        /// Tab und der Text aus der Zwischenablage laufen genau so heraus, wie
        /// sie dastehen (C2.1). Liegt einer bei, geht der **Quelltext** mit
        /// seinen Auszeichnungszeichen heraus und nicht das Gerenderte
        /// (C2.2). Der Nutzer hat das am 260819-2210 so entschieden, gegen die
        /// Empfehlung des Datensatzes
        /// `shared/decisions/260819-2216_*_was-landet-beim-gerenderten-markdown-in-der-zwischenablage.md`.
        ///
        /// **Die Grenzen der Auswahl brauchen keine Umrechnung.** `NSRange`
        /// zaehlt UTF-16-Einheiten, und das sind genau die Koordinaten, in
        /// denen der Quellbezug seine Textbereiche fuehrt; gerechnet wird in
        /// [`Quellbezug::quelltext`] und nicht hier.
        ///
        /// # Was Erschliessung ist und was gemessen
        ///
        /// **Erschliessung:** dass diese Methode der gemeinsame Ausgang aller
        /// fuenf Wege ist — `copy:`, der Eintrag des Hauptmenues, der Eintrag
        /// des Kontextmenues, die Dienste des Systems und das Ziehen einer
        /// Auswahl mit der Maus. So steht es in Apples Beschreibung
        /// (`NSTextView.h:258-277`, „Declares all the types to the pasteboard
        /// then calls writeSelectionToPasteboard:type: for each type"), und
        /// dieselbe Signatur traegt das Dienste-Protokoll
        /// `NSServicesMenuRequestor` (`NSApplication.h:539`). **An diesem Baum
        /// ist keiner der fuenf Wege gemessen**, denn dafuer braucht es KRK im
        /// Vordergrund, und das ist Nutzerarbeit.
        ///
        /// **Gemessen** ist allein, dass es bei **einer** Abfangstelle bleibt:
        /// die Zaehlprobe
        /// `die_abfangstelle_steht_im_baum_genau_einmal` liest den Baum.
        ///
        /// Der Nutzer nimmt die Wege am laufenden Buendel ab. Traegt einer von
        /// ihnen nicht, gehoert der Befund in den Datensatz
        /// `shared/decisions/260819-2216_*_gilt-die-quelltextzusage-auch-fuer-das-ziehen-einer-auswahl-und-die-dienste.md`,
        /// der fuer diesen Fall seine Moeglichkeit 2 bereithaelt. Ein zweiter
        /// Entwurf steht deshalb **nicht** vorsorglich daneben.
        // SAFETY: Die Signatur entspricht der von NSTextView
        // (`NSTextView.h:277`): zwei Objektargumente, ein Wahrheitswert
        // zurueck.
        #[unsafe(method(writeSelectionToPasteboard:types:))]
        fn auswahl_ablegen(
            &self,
            ablage: &NSPasteboard,
            sorten: &NSArray<NSPasteboardType>,
        ) -> bool {
            let Some(bezug) = self.quellbezug() else {
                // SAFETY: Die Oberklasse beantwortet dieselbe Nachricht mit
                // denselben zwei Argumenten und liefert einen Wahrheitswert.
                return unsafe {
                    msg_send![super(self), writeSelectionToPasteboard: ablage, types: sorten]
                };
            };
            let auswahl = self.selectedRange();
            let quelltext = bezug.quelltext(auswahl.location..auswahl.end());
            zwischenablage::text_auf_ablage_schreiben(ablage, quelltext)
        }
    }
);

impl Vorschautext {
    /// Eine Textanzeige mit dem genannten Rahmen, noch ohne Quellbezug.
    ///
    /// Ueber `initWithFrame:` und nicht ueber `initWithFrame:textContainer:`,
    /// weil diese Fassung das Textnetz — Textspeicher, Layoutverwalter und
    /// Behaelter — selbst aufspannt (`NSTextView.h:86`). Genau darauf greifen
    /// [`textmerkmale`] und [`Nummernspalte`] zu.
    fn neu(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(RefCell::new(None));
        // SAFETY: `initWithFrame:` von NSTextView hat die hier angenommene
        // Signatur.
        unsafe { msg_send![super(this), initWithFrame: rahmen] }
    }

    /// Legt den Quellbezug des Textes ab, der jetzt dasteht.
    ///
    /// `None` nimmt ihn zurueck. Wer den Text der Flaeche ersetzt, ohne diese
    /// Methode zu rufen, laesst den Bezug des **vorigen** Inhalts stehen, und
    /// eine Auswahl lieferte danach Quelltext aus einer anderen Datei.
    ///
    /// **Zwei Rufer und keiner mehr**, beide in dieser Datei:
    /// [`Vorschaufenster::text_zeigen`] nimmt den Bezug des vorigen Inhalts
    /// zurueck, und der Markdown-Zweig von [`Vorschaufenster::anzeigen`] legt
    /// den des neuen ab. Ein dritter Ort waere eine zweite Meinung darueber,
    /// wann ein Quellbezug gilt.
    fn quellbezug_setzen(&self, bezug: Option<Arc<Quellbezug>>) {
        *self.ivars().borrow_mut() = bezug;
    }

    /// Der Quellbezug des Textes, der jetzt dasteht, falls einer beiliegt.
    ///
    /// **Ein Rufer und keiner mehr**: [`Vorschautext::auswahl_ablegen`]. `None`
    /// heisst dort „das hier ist kein gerendertes Markdown", und die Antwort
    /// entscheidet, welcher der beiden Zweige der Ueberschreibung greift.
    #[must_use]
    fn quellbezug(&self) -> Option<Arc<Quellbezug>> {
        self.ivars().borrow().clone()
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
    ///
    /// Seit der Runde 14 ein [`Vorschautext`] und keine nackte `NSTextView`.
    /// Jede Beruehrung hier laeuft weiter ueber die Ableitung auf die
    /// Oberklasse — [`Nummernspalte::einhaengen`] und [`textmerkmale`]
    /// eingeschlossen —; die Unterklasse traegt allein den Quellbezug.
    text: Retained<Vorschautext>,
    /// Die Bildanzeige (C6: Bilder ueber `NSImage`).
    bild: Retained<NSImageView>,
    /// Die Leiste am Kopf. Sie kommt nach dem Objekt zur Welt, weil ihr
    /// Rueckruf es braucht; dieselbe Reihenfolge wie beim Dateifenster.
    tableiste: RefCell<Option<Tableiste>>,
    /// Die Tabs mit ihrem Inhalt und dem Halteverhalten.
    modell: RefCell<Vorschaumodell>,
    /// Der Zeitgeber, der die Meldungen **beider** Arbeitsfaeden abholt: die
    /// des Ladens aus dem Modell und die der Einfaerbung.
    ///
    /// Er haelt das Objekt als Ziel fest, und das Objekt haelt ihn; der Ring
    /// bricht mit `invalidate`, wie beim Einzugstakt des Dateifensters. Ein
    /// zweiter Zeitgeber neben ihm entsteht nicht: beide Kanaele werden im
    /// selben Takt gefragt, und geendet wird, wenn keiner von beiden mehr
    /// etwas zu liefern hat.
    takt: RefCell<Option<Retained<NSTimer>>>,
    /// Das laufende Einfaerben des angezeigten Quelltextes (C4 der Runde 6).
    ///
    /// **Er wohnt hier und nicht im [`Vorschaumodell`]**, und daran haengt die
    /// Endbedingung von L7; der Modulkopf schreibt es aus.
    ///
    /// **Hoechstens einer**: die Vorschau zeigt einen aktiven Tab, und ein
    /// zweiter Lauf faerbte denselben Text ein zweites Mal ein. Ein Tabwechsel
    /// oder ein neuer Inhalt waehrend eines Laufs startet deshalb keinen
    /// zweiten, sondern setzt [`Self::einfaerbung_erneut`]; was der
    /// ueberholte Lauf liefert, wird verworfen.
    einfaerbung: RefCell<Option<Einfaerbungsvorgang>>,
    /// Der aufgehobene Stand des letzten fertigen Einfaerbungslaufs.
    ///
    /// Die Vorlage, aus der [`crate::hervorhebung::fortschreiben`] den naechsten
    /// Lauf fortschreibt. Waehrend ein Lauf laeuft, steht hier `None`: die
    /// Vorlage ist dann im Arbeitsfaden und kommt mit dem Ergebnis zurueck.
    ///
    /// **Er wird ueber den Tabwechsel hinweg gehalten und nicht geleert.** Ein
    /// Stand, der zu einem anderen Text gehoert, ist keine falsche Vorlage,
    /// sondern nur eine schlechtere: `fortschreiben` vergleicht Schluessel und
    /// Text und rechnet dann von vorn. Zeigt der Nutzer denselben Tab wieder
    /// an, ist der Text derselbe, und der Lauf kostet nichts.
    einfaerbungsstand: RefCell<Option<Einfaerbungsstand>>,
    /// Ob der laufende Lauf ueberholt ist und nach seiner Rueckkehr sofort ein
    /// neuer zu starten ist.
    ///
    /// **Das ist die Schranke gegen den Faedenstau**, und sie ist derselbe
    /// Handgriff wie im Editor. Wer mit den Pfeiltasten durch einen Ordner
    /// geht, loest je Eintrag eine Anfrage aus; ohne die Marke stuende je
    /// Anfrage ein Faden, und ein fallengelassener Faden hoert nicht auf zu
    /// rechnen — er rechnet zu Ende und scheitert erst am `send`. Bei
    /// Dateien nahe der Textgrenze sind das Sekunden Rechenzeit je Schritt,
    /// und sie liefen gegen das Lesen des naechsten Eintrags, also gegen L7.
    ///
    /// Mit der Marke lebt zu jedem Zeitpunkt hoechstens ein Faden, und
    /// eingefaerbt wird der letzte Stand statt jedes Zwischenstandes. Sie
    /// traegt beide Anlaesse — einen neuen Inhalt und eine gewechselte
    /// Farbtafel —, denn beide verlangen dasselbe.
    einfaerbung_erneut: Cell<bool>,
    /// Welche der beiden Farbtafeln gerade gilt.
    ///
    /// Sie geht an zwei Stellen ein: in das Rendern von Markdown auf dem
    /// Arbeitsfaden des Modells (die Farbe eines Verweises) und in jeden
    /// Einfaerbungslauf. Gewaehlt wird sie von
    /// [`textmerkmale::tafel_der_erscheinung`], der einen Zuordnung im
    /// Programm; nachgezogen wird sie von [`Vorschaufenster::erscheinung_nachziehen`].
    tafel: Cell<Tafel>,
    /// Die geprueften Leseprofile, mit denen ein ausgewaehlter Ordner
    /// erkannt wird (Runde 16).
    ///
    /// **Sie stehen nach dem Aufbau der Oberflaeche fest und wechseln
    /// danach nicht mehr** (C4.5); deshalb eine [`OnceCell`] und keine
    /// [`RefCell`]. Der eine Schreiber ist
    /// [`Vorschaufenster::profile_setzen`], und dessen Doc-Kommentar sagt,
    /// warum die Profile hier wohnen und nicht im [`Vorschaumodell`].
    ///
    /// Leer heisst „keine Profile" und ist kein Fehlerfall: im Messmodus
    /// liest KRK die Ablage gar nicht erst, und dann zeigt jeder Ordner
    /// seine Metadaten.
    profile: OnceCell<Arc<Profile>>,
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

        // Die Tafel steht vor dem ersten Anzeigen fest: schon der erste
        // gerenderte Markdown-Text faerbt seine Verweise mit ihr.
        let tafel = textmerkmale::tafel_der_erscheinung(&bereich);

        let this = Self::alloc(mtm).set_ivars(VorschaufensterIvars {
            bereich,
            inhaltsflaeche,
            textrolle,
            text,
            bild,
            tableiste: RefCell::new(None),
            modell: RefCell::new(Vorschaumodell::neu()),
            takt: RefCell::new(None),
            einfaerbung: RefCell::new(None),
            einfaerbungsstand: RefCell::new(None),
            einfaerbung_erneut: Cell::new(false),
            tafel: Cell::new(tafel),
            profile: OnceCell::new(),
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

        // Der Rueckverweis fuer die Meldung ueber den Wechsel des
        // Erscheinungsbildes, aus demselben Grund an dieser Stelle wie der
        // Rueckruf der Tableiste darueber: es gibt das Objekt erst ab dem
        // `init`.
        this.ivars().inhaltsflaeche.ziel_setzen(&this);

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
    ///
    /// **Seit der Runde 14 beantwortet sie eine Frage mit**: welche der beiden
    /// Anzeigen steht gerade. Steht die Bildlaufansicht, ist die Antwort die
    /// Textanzeige; sonst bleibt es bei der Inhaltsflaeche, und damit beim
    /// heutigen Verhalten fuer ein Bild. Wer den Fokus ueber einen Befehl holt
    /// statt mit der Maus, bekommt den Ersthelferrang so an die Stelle, an der
    /// `cmd+a` und `cmd+c` wirken (C1.8 der Runde 14).
    ///
    /// **Die Fallunterscheidung fragt danach, welche Anzeige steht, und nicht
    /// danach, was der Tab zeigt.** Der Grund ist, was der Anwendungsdelegierte
    /// mit der Antwort tut: `Anwendungsdelegierter::fokusansicht` liefert die
    /// Ansicht nicht nur als Ersthelfer, sondern seit C1 der Runde 6 auch als
    /// **Anker** fuer den Freigabedialog — ein solcher Dialog haengt sich an
    /// eine Flaeche und an deren Rechteck. Eine ausgeblendete Ansicht taugt
    /// fuer keines von beidem: sie nimmt den Rang nicht an, und ein Anker ohne
    /// sichtbares Rechteck setzt den Dialog ins Nichts.
    ///
    /// Die zwei Zweige sind vollstaendig und ueberschneidungsfrei, weil genau
    /// eine der beiden Anzeigen sichtbar ist: [`Self::text_zeigen`] und
    /// [`Self::bild_zeigen`] setzen die beiden Schalter immer gegenlaeufig.
    ///
    /// **Es bleibt bei einer Zuordnung von Fokuswert auf Ansicht**, und diese
    /// Verzweigung steht **innerhalb** davon. Eine zweite Zuordnung daneben
    /// waeren zwei Wahrheiten darueber, welche Flaeche zu [`Fokus::Vorschau`]
    /// gehoert; die Begruendung im Langen steht an
    /// `Anwendungsdelegierter::fokusansicht`.
    ///
    /// [`Fokus::Vorschau`]: crate::kommandos::fokus::Fokus::Vorschau
    pub fn fokusansicht(&self) -> &NSView {
        if self.ivars().textrolle.isHidden() {
            &self.ivars().inhaltsflaeche
        } else {
            &self.ivars().text
        }
    }

    /// Die Textanzeige, an der die Naemlichkeitsfrage des Fokusvorbehalts
    /// haengt.
    ///
    /// Sie geht allein zum Vergleichen nach aussen, wortgleich zu
    /// [`Editorbereich::textflaeche`](super::editor::Editorbereich::textflaeche):
    /// der Anwendungsdelegierte haelt sie in
    /// `Anwendungsdelegierter::ist_eigene_textflaeche` gegen den Ersthelfer des
    /// Schluesselfensters. Seit sie auswaehlbar ist, nimmt sie den
    /// Ersthelferrang, und ohne diesen Vergleich fiele sie unter den
    /// Fokusvorbehalt: mit dem Fokus in der Vorschau wirkte kein einziger
    /// Tastenbefehl von KRK mehr, die vier Tabbefehle aus C1 der Runde 2
    /// eingeschlossen.
    ///
    /// **Der engere Typ der Unterklasse geht dabei nicht mit hinaus.**
    /// Verglichen wird ein Objektzeiger, und dafuer traegt `NSTextView` alles,
    /// was gebraucht wird; wer den [`Quellbezug`] setzen oder lesen will, geht
    /// ueber die Wege innerhalb dieser Datei.
    pub fn textflaeche(&self) -> &NSTextView {
        &self.ivars().text
    }

    /// Uebergibt der Vorschau die geprueften Leseprofile (Runde 16).
    ///
    /// **Der eine Schreiber des Merkfeldes, und er hat einen Rufer:**
    /// `Anwendungsdelegierter::oberflaeche_aufbauen` ruft ihn einmal, nachdem
    /// die Bereiche stehen, mit dem Satz, den `sitzung_laden` gelesen hat. Ein
    /// zweiter Rufer waere ein zweiter Zeitpunkt, zu dem die Vorschau andere
    /// Profile bekaeme, und C4.5 sagt das Gegenteil zu: es gilt der Stand des
    /// Starts, und eine geaenderte `readers.toml` erreicht KRK erst mit dem
    /// naechsten Start.
    ///
    /// **Warum die Profile hier wohnen und nicht im [`Vorschaumodell`].** Das
    /// Modell haelt, was ein Tab zeigt; die Profile sind Bestand der
    /// Anwendung und gelten fuer jeden Tab und fuer jeden, der noch aufgeht.
    /// Im Modell laegen sie an der Sache vorbei — es muesste sie beim
    /// Anlegen jedes Tabs mitbekommen und haette dann eine Auskunft zu
    /// halten, die mit dem angezeigten Tab nichts zu tun hat.
    ///
    /// **Ein zweiter Aufruf schreibt nicht**, und der Rueckgabewert faellt
    /// hier mit `let _ =` weg: dieselbe Form wie an den uebrigen
    /// `OnceCell`-Feldern des Programms, und sie heisst wie ueberall „ich
    /// brauche den Wert nicht". Dass es beim einen Aufruf bleibt, misst die
    /// Probe `die_profile_haben_genau_einen_schreiber_und_einen_rufer`.
    pub fn profile_setzen(&self, profile: Arc<Profile>) {
        let _ = self.ivars().profile.set(profile);
    }

    /// Zeigt den genannten Eintrag im aktiven Tab (C6).
    ///
    /// Kehrt sofort zurueck; gelesen wird auf dem Arbeitsfaden des Modells,
    /// und der Zeitgeber holt die Meldung ab.
    pub fn datei_anzeigen(&self, pfad: &Path) {
        // Die Tafel geht allein in die Farbe eines Verweises im gerenderten
        // Markdown; das Rendern laeuft auf dem Arbeitsfaden des Modells und
        // braucht sie deshalb jetzt. Gefragt wird nicht hier, sondern beim
        // Aufbau und bei jedem Wechsel des Erscheinungsbildes — die eine
        // Zuordnung dazu steht in [`textmerkmale::tafel_der_erscheinung`].
        // Die Profile kommen aus dem Merkfeld, das `profile_setzen` einmal
        // beim Aufbau der Oberflaeche fuellt. Ist es leer, heisst das „keine
        // Profile" und ist kein Fehlerfall: im Messmodus liest KRK die Ablage
        // gar nicht erst, und dann zeigt auch ein erkennbarer Ordner seine
        // Metadaten. Der leere Satz nimmt denselben Weg wie ein voller; eine
        // Verzweigung danach stuende sonst hier und im Modell ein zweites Mal.
        let profile = self.ivars().profile.get().cloned().unwrap_or_default();
        self.ivars()
            .modell
            .borrow_mut()
            .datei_anzeigen(pfad, self.ivars().tafel.get(), profile);
        // **Nur die Leiste und nicht die ganze Anzeige.** Geaendert hat sich
        // allein die Beschriftung des Tabs; Inhalt und Pfad wechseln erst,
        // wenn der Arbeitsfaden geliefert hat, und bis dahin steht der
        // bisherige Text da. Ein voller Durchgang durch [`Self::anzeigen`]
        // setzte ihn ein zweites Mal, naehme ihm dabei seine Farben und
        // forderte sie sofort wieder an — bei jedem Schritt durch eine
        // Dateiliste ein sichtbares Flackern und ein Faden fuer nichts.
        self.tableiste_nachziehen();
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

    /// Holt die Meldungen **beider** Arbeitsfaeden ab.
    ///
    /// Erst das Laden, dann die Einfaerbung, und die Reihenfolge ist keine
    /// Wahl: ein eben eingezogener Inhalt geht durch [`Self::anzeigen`] und
    /// vermerkt dabei, dass ein laufender Einfaerbungsvorgang ueberholt ist.
    /// Andersherum truege die Flaeche fuer einen Augenblick die Farben des
    /// vorigen Textes.
    ///
    /// Der Takt endet, wenn keiner der beiden Kanaele mehr etwas zu liefern
    /// hat. Eine Einfaerbung haelt ihn also am Leben, ohne dass
    /// [`Vorschaumodell::laedt_noch`] davon etwas wuesste — genau die
    /// Trennung, an der die Endbedingung von L7 haengt.
    fn einziehen(&self) {
        let aktiver_geaendert = self.ivars().modell.borrow_mut().einziehen();
        if aktiver_geaendert {
            self.anzeigen();
        }
        self.einfaerbung_einziehen();
        let laedt_noch = self.ivars().modell.borrow().laedt_noch();
        if !laedt_noch && self.ivars().einfaerbung.borrow().is_none() {
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
            // Der gerenderte Text und seine Auszeichnungen in einem Zug: die
            // Umsetzung ist die des Editors, und eine zweite daneben entsteht
            // nicht (C4, erstes und zweites Kriterium der Runde 6).
            Inhalt::Markdown(gerendert) => {
                self.text_zeigen(&gerendert.text);
                self.formatierung_anwenden(&gerendert.formatierung);
                // Der eine Ort, an dem ein Quellbezug gesetzt wird, und er
                // liegt neben dem einen Ort, an dem die Formatierung gesetzt
                // wird: beide sind Auskuenfte desselben Durchgangs ueber
                // denselben Text. `text_zeigen` hat den vorigen Bezug eben
                // zurueckgenommen, wie es die vorigen Merkmale zurueckgenommen
                // hat; die Reihenfolge ist deshalb dieselbe.
                //
                // **Der `Arc` wandert und wird nicht geklont.** `inhalt` ist
                // der Klon des aktiven Tabs und gehoert dieser Funktion; der
                // Quellbezug im Modell bleibt davon unberuehrt.
                self.ivars()
                    .text
                    .quellbezug_setzen(Some(gerendert.quellbezug));
            }
            Inhalt::Hinweis(hinweis) => self.text_zeigen(&hinweis),
            Inhalt::Metadaten {
                metadaten,
                zaehlzeilen,
            } => {
                let zeilen = self.metadaten_text(&metadaten, &zaehlzeilen);
                self.text_zeigen(&zeilen);
            }
            Inhalt::Bild { daten, metadaten } => self.bild_zeigen(&daten, metadaten.as_ref()),
            // Derselbe Weg wie Metadaten und Hinweise, und mehr geschieht
            // hier nicht (C4.2, C4.3): `als_text` ist die eine Stelle, an der
            // aus den Werten Zeilen werden, und sie steht in `krk-core`.
            //
            // **Daraus faellt C4.6 heraus, ohne dass eine Regel dazukommt.**
            // `text_zeigen` nimmt den Quellbezug des vorigen Inhalts zurueck;
            // ohne Quellbezug reicht `Vorschautext::auswahl_ablegen` an die
            // Oberklasse durch, und was markiert ist, geht Zeichen fuer
            // Zeichen heraus — wie bei jedem anderen Text der Vorschau seit
            // der Runde 14. Eine eigene Abfangstelle fuer die Zusammenfassung
            // waere eine zweite Meinung darueber, was eine Auswahl hergibt.
            Inhalt::Zusammenfassung(zusammenfassung) => {
                self.text_zeigen(&zusammenfassung.als_text());
            }
        }

        // Erst nachdem der Text steht: der Vorgang faerbt genau diese Zeichen
        // ein, und was hier eben gesetzt worden ist, hat den Text davor
        // ersetzt.
        self.einfaerbung_nachfuehren();

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
    ///
    /// **Und nimmt die Merkmale des vorigen Inhalts wieder heraus.** Was hier
    /// hereinkommt, steht so da, wie es gelesen wurde; ohne die Ruecknahme
    /// truege ein Hinweis nach einer Markdown-Datei deren Ueberschriften. Die
    /// Ruecknahme ist [`textmerkmale::zuruecksetzen`], die eine Stelle im
    /// Programm, die das tut — ein zweites Leeren daneben waere die zweite
    /// Meinung darueber, was zurueckzunehmen ist.
    ///
    /// Die Besetzung ist die der **Rohansicht**, und der Grund steht im
    /// Modulkopf unter "Warum die Vorschau beide Werte von `Ansicht`
    /// benutzt". Die [`Darstellungsart`] geht dabei in nichts ein: die
    /// Rohansicht bekommt ihre feste Schrift unabhaengig von ihr.
    ///
    /// **Und nimmt den Quellbezug des vorigen Inhalts mit zurueck**, an
    /// derselben Stelle und aus demselben Grund (C1.13 der Runde 14). Ohne die
    /// Ruecknahme lieferte eine Auswahl im rohen Text einer Datei den Quelltext
    /// der Markdown-Datei, die vorher dastand. Setzen und Loeschen haben damit
    /// je genau einen Ort: gesetzt wird im Markdown-Zweig von
    /// [`Self::anzeigen`], geloescht hier.
    ///
    /// **Daraus faellt die ganze Zusage C1.13 heraus, ohne dass eine Regel
    /// dafuer entsteht.** Jeder Inhaltswechsel laeuft ueber diese Funktion —
    /// ein Tabwechsel, eine andere Datei, ein neuer Lesevorgang —, also faellt
    /// der Quellbezug mit ihm. Und weil `setString:` den Textspeicher **ganz**
    /// ersetzt, laesst AppKit die sichtbare Auswahl von sich aus fallen; eine
    /// Auswahl je Tab zu merken, waere die vom Nutzer nicht gewaehlte
    /// Moeglichkeit gewesen.
    fn text_zeigen(&self, text: &str) {
        let ivars = self.ivars();
        ivars.text.setString(&NSString::from_str(text));
        ivars.text.quellbezug_setzen(None);
        textmerkmale::zuruecksetzen(&ivars.text, Ansicht::Roh, Darstellungsart::EinfacherText);
        ivars.textrolle.setHidden(false);
        ivars.bild.setHidden(true);
    }

    /// Traegt eine fertige Formatierung in die Textanzeige (C4 der Runde 6).
    ///
    /// **Dieselbe Umsetzung wie im Editor**, und der Aufruf ist die ganze
    /// Beteiligung dieser Datei daran: welche Stelle welches Merkmal traegt,
    /// rechnet [`crate::hervorhebung`] und [`crate::markdown`] aus, und
    /// [`textmerkmale::anwenden`] setzt es. Zwei Wege dorthin fuehren hier
    /// zusammen — das gerenderte Markdown und die eingefaerbte Quelltextdatei.
    ///
    /// **Die Darstellungsart kommt aus der Lieferung und wird nicht ein
    /// zweites Mal gefragt.** Eine [`Formatierung`] nennt die Besetzung, aus
    /// der sie entstanden ist; eine zweite Frage an
    /// [`crate::hervorhebung::art`] koennte anders
    /// ausfallen als die, die diese Listen erzeugt hat.
    ///
    /// **Nachgezogen wird nur, wenn gesetzt wurde**, wie im Editor: hat der
    /// Guertel in [`textmerkmale::anwenden`] die Lieferung abgewiesen, ist an
    /// der Flaeche nichts geschehen. Die Nummernspalte steht in der Vorschau
    /// allein neben dem rohen Text einer Datei — also genau im
    /// Quelltextfall —, und die Auszeichnungen aendern die Zeilenkaesten.
    fn formatierung_anwenden(&self, formatierung: &Formatierung) {
        if textmerkmale::anwenden(
            &self.ivars().text,
            formatierung,
            formatierung.art,
            Ansicht::Format,
        ) {
            nummernspalte::spalte_neu_zeichnen(&self.ivars().textrolle);
        }
    }

    /// Fordert die Einfaerbung des Textes an, der jetzt dasteht (C4 der
    /// Runde 6).
    ///
    /// **Die eine Stelle, die einen Vorgang startet**, und sie hat zwei
    /// Anlaesse: ein neuer Inhalt in der Flaeche und ein gewechseltes
    /// Erscheinungsbild. Beide verlangen dasselbe, naemlich den Lauf ueber den
    /// Text, der jetzt dasteht. Ob ueberhaupt anzufordern ist, beantwortet
    /// [`einzufaerben`] und sonst nichts.
    ///
    /// **Hoechstens ein Faden zur Zeit**, dieselbe Bauart wie im Editor und
    /// ohne Anfragenummer. Laeuft schon einer, wird kein zweiter gestartet,
    /// sondern nur vermerkt, dass sein Ergebnis ueberholt sein wird; er wird
    /// nach seiner Rueckkehr verworfen und sofort wiederholt. Der Plan sagt an
    /// dieser Stelle "laesst einen laufenden Vorgang fallen"; fallengelassen
    /// wird hier das **Ergebnis** und nicht der Empfaenger, und der Grund
    /// steht an [`VorschaufensterIvars::einfaerbung_erneut`]: ein
    /// fallengelassener Faden hoert nicht auf zu rechnen.
    fn einfaerbung_nachfuehren(&self) {
        if self.ivars().einfaerbung.borrow().is_some() {
            self.ivars().einfaerbung_erneut.set(true);
            return;
        }
        let angaben = {
            let modell = self.ivars().modell.borrow();
            let pfad = modell.aktiver_pfad();
            einzufaerben(modell.aktiver_inhalt(), pfad.as_deref())
                .map(|(text, pfad)| (text.to_owned(), pfad.to_path_buf()))
        };
        let Some((stand, pfad)) = angaben else {
            // Nichts einzufaerben: ein etwaiger Vermerk gehoerte zu einem
            // Inhalt, den die Flaeche nicht mehr zeigt.
            self.ivars().einfaerbung_erneut.set(false);
            return;
        };
        // Die Vorlage wandert in den Lauf hinein und kommt mit dem Ergebnis
        // zurueck; waehrenddessen haelt sie niemand hier.
        let vorlage = self.ivars().einfaerbungsstand.borrow_mut().take();
        let typ = Dateityp::von_pfad(&pfad);
        let vorgang =
            Einfaerbungsvorgang::starten(vorlage, stand, Some(pfad), typ, self.ivars().tafel.get());
        *self.ivars().einfaerbung.borrow_mut() = Some(vorgang);
        self.ivars().einfaerbung_erneut.set(false);
        self.takt_starten();
    }

    /// Holt die Meldung des Einfaerbungsfadens ab (C4 der Runde 6).
    ///
    /// **Ein ueberholtes Ergebnis wird nicht angewendet, sondern verworfen und
    /// sofort neu angefordert.** Es waere nicht nur veraltet: seine Bereiche
    /// zeigten in einen Text, der inzwischen kuerzer sein kann, und ein
    /// `NSRange` hinter dem Text beantwortet AppKit mit einer
    /// Objective-C-Ausnahme, die in Rust nicht zu fangen ist. Der Guertel in
    /// [`textmerkmale::anwenden`] faengt denselben Fall ein zweites Mal ab.
    ///
    /// **Verworfen wird allein die Formatierung, nicht der aufgehobene
    /// Stand.** Der Stand beschreibt einen Text, der wirklich gerechnet worden
    /// ist, und ist damit auch fuer den ueberholten Fall die bessere Vorlage
    /// als keine: der naechste Lauf schreibt von ihm fort oder erkennt am
    /// Schluessel, dass er von vorn rechnen muss.
    ///
    /// **Angewendet wird aus einer eigenen Bindung und nicht aus der Zelle
    /// heraus**, wie bei jeder anderen Ausleihe in dieser Datei: der Weg
    /// fuehrt in das Textsystem, und ein Rueckweg von dort naehme eine zweite
    /// Ausleihe derselben Zelle.
    fn einfaerbung_einziehen(&self) {
        let abholung = {
            let vorgang = self.ivars().einfaerbung.borrow();
            match vorgang.as_ref() {
                Some(vorgang) => vorgang.abholen(),
                None => return,
            }
        };
        match abholung {
            Abholung::Laeuft => {}
            // Der Faden ist ohne Meldung gefallen; darauf zu warten hat keinen
            // Sinn mehr. Mit ihm faellt die Vorlage, und der naechste Lauf
            // rechnet von vorn: langsamer, aber nicht falsch.
            Abholung::Weggefallen => {
                *self.ivars().einfaerbung.borrow_mut() = None;
                self.ivars().einfaerbung_erneut.set(false);
            }
            Abholung::Fertig(stand) => {
                *self.ivars().einfaerbung.borrow_mut() = None;
                let stand = *stand;
                let ueberholt = self.ivars().einfaerbung_erneut.replace(false);
                if !ueberholt {
                    self.formatierung_anwenden(stand.formatierung());
                }
                *self.ivars().einfaerbungsstand.borrow_mut() = Some(stand);
                if ueberholt {
                    self.einfaerbung_nachfuehren();
                }
            }
        }
    }

    /// Zieht die Farbtafel auf das gewechselte Erscheinungsbild nach (C4 der
    /// Runde 6).
    ///
    /// Gerufen von [`Inhaltsflaeche`], der einen Stelle, an der AppKit den
    /// Wechsel meldet. Hat sich die Tafel nicht geaendert, geschieht nichts:
    /// die Meldung kommt auch bei Wechseln, die Hell und Dunkel nicht
    /// betreffen.
    ///
    /// **Gerendertes Markdown zieht damit noch nicht nach.** Die Farbe eines
    /// Verweises entsteht beim Rendern auf dem Arbeitsfaden des Modells und
    /// steht im Inhalt des Tabs; sie neu zu setzen hiesse, die Datei ein
    /// zweites Mal zu lesen, und zwar in jedem Tab. Ein Markdown-Tab traegt
    /// bis zu seiner naechsten Anzeige also die Verweisfarbe der Tafel, mit
    /// der er gerendert wurde. Alles Uebrige an ihm — Schrift, Einzug,
    /// Fliesstextfarbe — kommt aus dem System und wechselt mit.
    fn erscheinung_nachziehen(&self) {
        let neue = textmerkmale::tafel_der_erscheinung(&self.ivars().inhaltsflaeche);
        if neue == self.ivars().tafel.get() {
            return;
        }
        self.ivars().tafel.set(neue);
        // Der aufgehobene Stand traegt die alte Tafel in seinem Schluessel;
        // `fortschreiben` erkennt das und rechnet von vorn. Ihn hier
        // wegzuwerfen waere dieselbe Entscheidung an einer zweiten Stelle.
        self.einfaerbung_nachfuehren();
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
                    let zeilen = self.metadaten_text(metadaten, &[]);
                    self.text_zeigen(&zeilen);
                }
                None => {
                    self.text_zeigen("Das Bild aus der Zwischenablage ließ sich nicht darstellen.")
                }
            },
        }
    }

    /// Die sechs Metadatenzeilen aus C6, und seit der Runde 19 darunter die
    /// Zaehlzeilen des Default-Profils, falls welche mitkommen (C2.1, C2.2).
    ///
    /// Die drei Zaehlzeilen entstehen hier und nicht im Kern, weil nur hier
    /// beide Haelften vorliegen: Groesse und Aenderungsdatum brauchen die
    /// Formatierer von AppKit, die Zaehlwerte den Leselauf des Kerns. Aus
    /// Werten Zeilen macht dabei allein
    /// [`zeilen_als_text`](krk_core::leseprofil::zeilen_als_text), dieselbe
    /// Stelle, die auch die Zusammenfassung ruft; eine leere Folge haengt
    /// nichts an, und so bleiben die sechs Zeilen einer Datei, wie sie waren.
    fn metadaten_text(
        &self,
        metadaten: &Metadaten,
        zaehlzeilen: &[Zusammenfassungszeile],
    ) -> String {
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
            "Name: {}\nPfad: {}\nGröße: {}\nGeändert: {}\nRechte: {}\nTyp: {}{}",
            metadaten.name,
            metadaten.pfad.display(),
            groesse,
            geaendert,
            rechte_text(metadaten.rechte),
            typ_beschriften(metadaten.typ),
            zeilen_als_text(zaehlzeilen),
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

/// Was der aktive Tab dem Einfaerbungsfaden gibt; `None`, wenn nichts
/// einzufaerben ist (C4, zehntes Kriterium der Runde 6).
///
/// **Die eine Bedingung, und sie ist genau [`Darstellungsart::Code`].** Fuer
/// [`Darstellungsart::EinfacherText`] gaebe es nichts einzufaerben — die Kiste
/// kennt keine Sprache dazu und lieferte einen Lauf ohne ein einziges Stueck.
/// Fuer [`Darstellungsart::Markdown`] ist der Weg ein anderer: dort steht die
/// Formatierung schon im [`Inhalt`], gerechnet auf dem Arbeitsfaden des
/// Modells.
///
/// **Ohne Pfad wird nicht eingefaerbt.** [`Inhalt::Text`] traegt auch den Text
/// aus der Zwischenablage, und an dem haengt keine Endung, an der eine Sprache
/// zu erkennen waere. Dieselbe Unterscheidung, die
/// [`Vorschaumodell::zeigt_dateitext`] fuer die Nummernspalte trifft.
///
/// **Eine Zusammenfassung ist kein Quelltext.** Ihre Zeilen gibt ein Leseprofil
/// vor, und der Pfad daneben zeigt auf einen Ordner; es gibt nichts, woran eine
/// Sprache haengt, und `syntect` haette an ihr nichts zu faerben.
///
/// **Eine reine Fallunterscheidung ohne Auffangzweig**, wie die uebrigen dieser
/// Art im Programm: ein achter [`Inhalt`] haelt den Bau an und erzwingt die
/// Antwort auf die Frage, ob er eingefaerbt wird.
///
/// Keine Groessenschranke: eingefaerbt wird jede Datei, die die Vorschau
/// ueberhaupt als Text zeigt, und was sie als Text zeigt, entscheidet
/// `TEXTGRENZE` und sonst nichts (C4, zwoelftes Kriterium).
fn einzufaerben<'a>(inhalt: &'a Inhalt, pfad: Option<&'a Path>) -> Option<(&'a str, &'a Path)> {
    match inhalt {
        Inhalt::Text(text) => match pfad {
            Some(pfad)
                if hervorhebung::art(Some(pfad), Dateityp::von_pfad(pfad))
                    == Darstellungsart::Code =>
            {
                Some((text.as_str(), pfad))
            }
            Some(_) | None => None,
        },
        // Der eigene Zweig haelt den Grund an der Stelle, an der er gilt:
        // eine Zusammenfassung traegt keine Sprache, an der ein Einfaerbungs-
        // lauf ansetzen koennte. In der Sammelliste darunter stuende er nicht,
        // und der Leser muesste ihn sich aus dem Doc-Kommentar zusammensuchen.
        Inhalt::Zusammenfassung(_) => None,
        Inhalt::Leer
        | Inhalt::Markdown(_)
        | Inhalt::Bild { .. }
        | Inhalt::Metadaten { .. }
        | Inhalt::Hinweis(_) => None,
    }
}

/// Baut die Textanzeige: einen auswaehlbaren, nicht bearbeitbaren
/// [`Vorschautext`] in einer Bildlaufansicht.
///
/// **Die beiden Schalter sagen nicht mehr dasselbe, und das ist der Kern
/// dieser Stelle.**
///
/// `setSelectable(true)` ist neu und loest eine abgenommene Zusage ab: C4,
/// achtes Kriterium der Runde 6, sagte „die beiden Schalter bleiben, wo sie
/// stehen", und der Nutzer hat das am 260819 **ersetzt und nicht ergaenzt**
/// (C1.1 der Runde 14). Der Grund der alten Zusage — eine auswaehlbare Flaeche
/// nimmt den Fokus als Textsystem — gilt unveraendert; er ist nur nicht mehr
/// mit dem Schalter zu bezahlen, sondern mit der Anmeldung der Flaeche beim
/// Anwendungsdelegierten (C1.7). Der Modulkopf fuehrt das aus.
///
/// `setEditable(false)` bleibt davon **unberuehrt**. Es steht hier nicht als
/// Mittel gegen den Fokus, sondern weil die Vorschau zeigt und nicht bearbeitet
/// (C1.4); keine Ueberlegung, die den anderen Schalter hat fallen lassen,
/// trifft diesen. Die Flaeche ist damit auch keine „bearbeitbare
/// Textflaeche" im Sinne von [`super::textautomatik`] und schaltet dessen
/// Automatiken nicht ab: was der Nutzer nicht tippen kann, veraendert keine
/// Automatik.
///
/// Die Schrift ist die der Rohansicht, also die feste Schreibmaschinenschrift
/// des Nutzers, weil C6 die Anzeige als **rohen** Inhalt zusagt. Sie kommt
/// dabei aus [`textmerkmale::grundschrift`] und nicht aus einer eigenen Wahl:
/// [`Vorschaufenster::text_zeigen`] setzt dieselbe ueber den ganzen
/// Textspeicher, und zwei Rechnungen daneben waeren die erste Gelegenheit, dass
/// die Flaeche in einer anderen Schrift dasteht als ihr Inhalt.
fn textanzeige(
    mtm: MainThreadMarker,
    rahmen: NSRect,
) -> (Retained<NSScrollView>, Retained<Vorschautext>) {
    let rolle = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), rahmen);
    rolle.setHasVerticalScroller(true);
    rolle.setAutohidesScrollers(true);
    rolle.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
    );

    let text = Vorschautext::neu(mtm, rahmen);
    text.setEditable(false);
    text.setSelectable(true);
    text.setVerticallyResizable(true);
    text.setHorizontallyResizable(false);
    text.setMinSize(NSSize::ZERO);
    text.setMaxSize(NSSize::new(f64::MAX, f64::MAX));
    text.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
    text.setFont(Some(&textmerkmale::grundschrift(
        Ansicht::Roh,
        Darstellungsart::EinfacherText,
    )));
    rolle.setDocumentView(Some(&text));
    // **Dieselbe Klasse, die der Editor einhaengt** (C10), und keine zweite
    // Spalte daneben. Ob sie steht, entscheidet `Vorschaufenster::anzeigen`
    // ueber `setRulersVisible`; hier entsteht sie nur.
    Nummernspalte::einhaengen(mtm, &rolle, &text);
    (rolle, text)
}

/// Was ohne Fenster zu pruefen ist: die Bedingung, unter der eingefaerbt wird,
/// und der Ort, an dem der Vorgang wohnt.
///
/// Alles Uebrige an dieser Datei haengt an einer Instanz und steht deshalb als
/// Kriterium am Buendel; diese Runde baut keine neue Probe, die den Hauptfaden
/// behauptet.
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::SystemTime;

    use crate::quellbaum::{aufrufstellen, quelldateien};

    use super::*;

    /// Metadaten mit dem genannten Pfad, sonst nichtssagend.
    fn metadaten(pfad: &Path) -> Metadaten {
        Metadaten {
            name: "beispiel".to_owned(),
            pfad: pfad.to_path_buf(),
            groesse: 0,
            geaendert: SystemTime::UNIX_EPOCH,
            rechte: 0o644,
            typ: Typ::Datei,
        }
    }

    /// Eingefaerbt wird genau [`Darstellungsart::Code`] und sonst nichts
    /// (C4, zehntes und zwoelftes Kriterium der Runde 6).
    ///
    /// **Die Probe zur Anforderungsbedingung, als reine Fallunterscheidung.**
    /// Sie misst die eine Frage, die [`einzufaerben`] beantwortet: aus welchem
    /// Zustand der Vorschau ein Einfaerbungsfaden entsteht. Sechs der sieben
    /// Werte von [`Inhalt`] kommen hier vor; der siebte, die Zusammenfassung,
    /// hat seit der Runde 16 die Probe
    /// [`eine_zusammenfassung_wird_nicht_eingefaerbt`] daneben. Ein achter
    /// faellt an der vollstaendigen Fallunterscheidung in [`einzufaerben`]
    /// auf und nicht erst am Bild.
    #[test]
    fn eingefaerbt_wird_genau_darstellungsart_code() {
        let quelltext = PathBuf::from("/tmp/beispiel.rs");
        let unbekannt = PathBuf::from("/tmp/beispiel.krk-gibt-es-nicht");
        let markdown = PathBuf::from("/tmp/beispiel.md");

        // Der eine Fall: eine Datei, deren Sprache die Kiste kennt. Text und
        // Pfad gehen an den Faden, wie sie dastehen.
        let quelle = Inhalt::Text("fn main() {}\n".to_owned());
        let (text, pfad) = einzufaerben(&quelle, Some(&quelltext))
            .expect("eine .rs-Datei ist Quelltext und wird eingefaerbt");
        assert_eq!(text, "fn main() {}\n");
        assert_eq!(pfad, quelltext);

        // Einfacher Text: die Kiste kennt keine Sprache, es gaebe nichts
        // einzufaerben.
        assert!(
            einzufaerben(&Inhalt::Text("nur Text\n".to_owned()), Some(&unbekannt)).is_none(),
            "eine unbekannte Endung ist einfacher Text und kein Quelltext"
        );

        // Eine Markdown-Endung, deren Inhalt als roher Text dasteht: der Weg
        // fuer Markdown ist ein anderer, und `art` sagt es.
        assert!(
            einzufaerben(&Inhalt::Text("# Titel\n".to_owned()), Some(&markdown)).is_none(),
            "Markdown geht ueber Inhalt::Markdown und nicht ueber die Einfaerbung"
        );

        // Text ohne Pfad: die Zwischenablage. Keine Endung, keine Sprache.
        assert!(
            einzufaerben(&Inhalt::Text("aus der Ablage".to_owned()), None).is_none(),
            "der Text der Zwischenablage traegt keinen Pfad und wird nicht eingefaerbt"
        );

        // Die uebrigen fuenf Werte von `Inhalt`, jeder mit dem Pfad einer
        // Quelltextdatei daneben: an ihnen liegt es und nicht am Pfad.
        let uebrige = [
            Inhalt::Leer,
            Inhalt::Markdown(Box::new(crate::markdown::rendern("# Titel\n", Tafel::Hell))),
            Inhalt::Bild {
                daten: Arc::new(Vec::new()),
                metadaten: Some(metadaten(&quelltext)),
            },
            Inhalt::Metadaten {
                metadaten: metadaten(&quelltext),
                zaehlzeilen: Vec::new(),
            },
            Inhalt::Hinweis("etwas ging nicht".to_owned()),
        ];
        for inhalt in &uebrige {
            assert!(
                einzufaerben(inhalt, Some(&quelltext)).is_none(),
                "{inhalt:?} zeigt keinen rohen Dateitext und wird nicht eingefaerbt"
            );
        }
    }

    /// Eine Zusammenfassung wird nicht eingefaerbt (Runde 16).
    ///
    /// **Die Probe zum siebten Wert von [`Inhalt`]**, nach dem Vorbild der
    /// Probe darueber und aus demselben Grund daneben und nicht in ihr: der
    /// Grund ist ein eigener. Eine Zusammenfassung zeigt Text, aber keinen
    /// Dateiinhalt; ihre Zeilen gibt ein Leseprofil vor, und keine Endung
    /// sagt eine Sprache dazu.
    ///
    /// **Der zweite Durchgang legt einen Quelltextpfad daneben.** An ihm
    /// haengt es nicht: `einzufaerben` entscheidet am Inhalt, und ohne diese
    /// Haelfte liesse die Probe zu, dass die Antwort bloss am Ordnerpfad
    /// haengt.
    #[test]
    fn eine_zusammenfassung_wird_nicht_eingefaerbt() {
        let ordner = PathBuf::from("/tmp/probe/werkbank");
        let quelltext = PathBuf::from("/tmp/beispiel.rs");
        let zusammenfassung = Inhalt::Zusammenfassung(krk_core::leseprofil::Zusammenfassung::neu(
            "werkbank".to_owned(),
            ordner.clone(),
            vec![krk_core::leseprofil::Zusammenfassungszeile::neu(
                "Runden".to_owned(),
                krk_core::leseprofil::Wert::Zahl(16),
            )],
        ));

        assert!(
            einzufaerben(&zusammenfassung, Some(&ordner)).is_none(),
            "eine Zusammenfassung traegt keine Sprache; syntect haette an ihr \
             nichts zu faerben"
        );
        assert!(
            einzufaerben(&zusammenfassung, Some(&quelltext)).is_none(),
            "am Inhalt liegt es und nicht am Pfad daneben"
        );
    }

    /// Das Merkfeld der Profile hat genau einen Schreiber, und `profile_setzen`
    /// genau einen Rufer, und der steht beim Anwendungsdelegierten (C4.5).
    ///
    /// **Beide Haelften stehen als „genau einmal" da.** Zugesagt ist erstens,
    /// dass [`Vorschaufenster::profile_setzen`] der eine Schreiber des
    /// Merkfeldes ist, und zweitens, dass genau eine Stelle im Baum ihn ruft,
    /// naemlich `oberflaeche_aufbauen` in `appkit/anwendung.rs`. Ein zweiter
    /// Schreiber wie ein zweiter Rufer waere ein zweiter Zeitpunkt, zu dem die
    /// Vorschau andere Profile bekaeme, und an keinem Rueckgabewert waere
    /// einer von beiden abzulesen. Gezaehlt wird deshalb im Baum.
    ///
    /// Die zweite Haelfte stand bis zum Schritt 11 der Runde 16 als obere
    /// Schranke da, weil es den Rufer noch nicht gab; seit jener Schritt ihn
    /// gebaut hat, ist sie eine Gleichheit und faengt damit auch den Fall, dass
    /// ihn jemand wieder ausbaut.
    ///
    /// # Was diese Probe nicht sieht
    ///
    /// **Sie sagt nichts darueber, ob die Profile ueberhaupt gelesen wurden.**
    /// Ein Rufer, der einen leeren Satz uebergibt, besteht sie muehelos; dass
    /// `readers.toml` beim Start einmal gelesen wird, misst die Zaehlprobe
    /// ueber `leseprofile::laden` beim Anwendungsdelegierten und nicht diese.
    ///
    /// Daneben gelten die Grenzen aus dem Kopf von [`crate::quellbaum`]:
    /// [`aufrufstellen`] zaehlt jede Empfaengerform und jeden Pfad, aber
    /// keinen Aufruf unter einem anderen Namen.
    #[test]
    fn die_profile_haben_genau_einen_schreiber_und_einen_rufer() {
        // Beide Nadeln stehen zusammengesetzt da: die Probe liegt in dem Baum,
        // den sie liest, und als ein Stueck geschrieben faende jede sich
        // selbst.
        let schreiber = concat!("profile", ".set");
        let rufer = concat!("profile_", "setzen");
        let vorschau = "krk-ui/src/appkit/vorschau.rs";
        let anwendung = "krk-ui/src/appkit/anwendung.rs";

        let dateien = quelldateien();
        let zaehlen = |nadel: &str| -> Vec<(String, usize)> {
            dateien
                .iter()
                .map(|(datei, inhalt)| (datei.clone(), aufrufstellen(inhalt, nadel)))
                .filter(|(_, zahl)| *zahl > 0)
                .collect()
        };

        assert_eq!(
            zaehlen(schreiber),
            vec![(vorschau.to_owned(), 1)],
            "`{schreiber}` steht nicht genau einmal, und zwar in der Vorschau; \
             das Merkfeld hat einen Schreiber, und der heisst `{rufer}`"
        );

        assert_eq!(
            zaehlen(rufer),
            vec![(anwendung.to_owned(), 1)],
            "`{rufer}` wird nicht genau einmal und beim Anwendungsdelegierten \
             gerufen; die Profile gehen einmal beim Aufbau der Oberflaeche \
             herein und wechseln danach nicht mehr"
        );
    }

    /// Das Vorschaumodell weiss von der Einfaerbung nichts (C4, elftes
    /// Kriterium der Runde 6).
    ///
    /// **Eine Probe ueber die Modulgrenze und keine ueber ein Ergebnis.** Die
    /// Zusage lautet, dass [`Vorschaumodell::laedt_noch`] nicht auf `syntect`
    /// wartet, und sie haengt daran, **wo** der Vorgang wohnt: an keinem
    /// Rueckgabewert des Modells ist abzulesen, dass es ihn nicht kennt. Also
    /// wird der Baum gelesen — dieselbe Art der Abnahme, mit der
    /// [`super::teilen`] seine Zaehlproben fuehrt.
    ///
    /// Die zweite Haelfte ist der Riegel gegen eine Probe, die alles
    /// bestaetigt: verschwaende die Einfaerbung ganz aus der Vorschau, waere
    /// die erste Haelfte weiterhin wahr.
    #[test]
    fn das_vorschaumodell_weiss_von_der_einfaerbung_nichts() {
        let lesen = |unterpfad: &str| {
            let pfad = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join(unterpfad);
            std::fs::read_to_string(&pfad)
                .unwrap_or_else(|fehler| panic!("{} nicht lesbar: {fehler}", pfad.display()))
        };

        let modell = lesen("vorschaumodell.rs");
        for name in ["Einfaerbungsvorgang", "Einfaerbungsstand", "fortschreiben"] {
            assert!(
                !modell.contains(name),
                "vorschaumodell.rs nennt {name}; damit haengt die Endbedingung von L7 an syntect"
            );
        }

        let ansicht = lesen("appkit/vorschau.rs");
        for name in ["Einfaerbungsvorgang", "Einfaerbungsstand"] {
            assert!(
                ansicht.contains(name),
                "appkit/vorschau.rs nennt {name} nicht; die Probe darueber bestaetigte dann nichts"
            );
        }
    }

    /// Die beiden Schalter stehen je an genau einer Stelle im Baum, und dort
    /// (C1.1 und C1.4 der Runde 14).
    ///
    /// **Die eine Haelfte ist die abgeloeste Zusage.** C4, achtes Kriterium
    /// der Runde 6, sagte zu, dass die Textanzeige der Vorschau nicht
    /// auswaehlbar bleibt; der Nutzer hat das am 260819 ersetzt. Diese Probe
    /// haelt fest, dass die Zeile wirklich weg ist und nicht bloss ein
    /// zweites Mal daneben steht — an einer gebauten Flaeche ist das nicht zu
    /// messen, weil `krk-ui` kein Bibliotheksziel hat und eine Probe, die
    /// dafuer den Hauptfaden behauptet, der bekannte Defekt `260810-1001`
    /// waere. Die sichtbare Haelfte, dass sich der Text mit der Maus markieren
    /// laesst, steht als Kriterium am Buendel und ist Nutzerarbeit.
    ///
    /// **Die andere Haelfte ist die Zusage, die stehen bleibt.**
    /// `setEditable(false)` faellt nicht mit dem anderen Schalter, und der
    /// Doc-Kommentar von [`super::textanzeige`] sagt, warum. Ohne diese
    /// Haelfte waere die Probe einseitig: sie liesse zu, dass jemand beide
    /// Zeilen fuer dasselbe haelt und die Vorschau nebenbei bearbeitbar macht.
    ///
    /// # Was diese Nadel nicht sieht
    ///
    /// **Sie zaehlt Codezeilen und nicht Aufrufe.** Ein Aufruf, der ueber zwei
    /// Zeilen umbricht, der ueber `setValue:forKey:` geht oder der den Wert
    /// aus einer Variablen nimmt, entgeht ihr vollstaendig. Der Kopf von
    /// [`crate::quellbaum`] sagt, warum keine Suche im Quelltext restlos dicht
    /// ist.
    ///
    /// **Und sie unterscheidet `NSTextView` nicht von `NSTextField`.** Die
    /// eine verbliebene Fundstelle von `setSelectable(false)` sitzt an der
    /// Meldungszeile des Belegungsblattes, und das ist ein `NSTextField` und
    /// keine Textanzeige; sie steht deshalb als erwartete Stelle in der
    /// Erwartung und nicht als Fehlschlag. Der Plan der Runde erwartete an
    /// dieser Stelle „kommt im Baum nicht mehr vor"; das war am Baum nicht
    /// nachgesehen, und die Probe schreibt die Lage aus, statt eine
    /// Erwartung zu setzen, die von Anfang an rot waere.
    #[test]
    fn die_zwei_schalter_stehen_je_an_genau_einer_stelle_und_dort() {
        // Beide Nadeln stehen zusammengesetzt da: die Probe liegt in dem Baum,
        // den sie liest, und als ein Stueck geschrieben faende jede sich
        // selbst.
        let nicht_auswaehlbar = concat!("setSelectable(", "false)");
        let nicht_bearbeitbar = concat!("setEditable(", "false)");

        let stellen = |nadel: &str| -> Vec<(String, usize)> {
            quelldateien()
                .into_iter()
                .map(|(name, inhalt)| {
                    let zahl = inhalt
                        .lines()
                        .filter(|zeile| !zeile.trim_start().starts_with("//"))
                        .filter(|zeile| zeile.contains(nadel))
                        .count();
                    (name, zahl)
                })
                .filter(|(_, zahl)| *zahl > 0)
                .collect()
        };

        assert_eq!(
            stellen(nicht_auswaehlbar),
            vec![("krk-ui/src/appkit/belegungsansicht.rs".to_owned(), 1)],
            "`{nicht_auswaehlbar}` steht nicht mehr allein an der Meldungszeile \
             des Belegungsblattes; die Textanzeige der Vorschau ist seit der \
             Runde 14 auswaehlbar"
        );
        assert_eq!(
            stellen(nicht_bearbeitbar),
            vec![("krk-ui/src/appkit/vorschau.rs".to_owned(), 1)],
            "`{nicht_bearbeitbar}` steht nicht mehr genau einmal, und zwar an \
             der Textanzeige der Vorschau; die Vorschau zeigt und bearbeitet \
             nicht"
        );
    }

    /// Der Quellbezug wird an genau zwei Stellen gesetzt, und beide liegen
    /// hier (C1.13 der Runde 14).
    ///
    /// **Eine Aufruferzaehlung, und sie steht hier zu Recht.** Der Kopf von
    /// [`crate::quellbaum`] laesst eine solche Zaehlung nur dort zu, wo ein
    /// Abnahmekriterium die Zahl selbst zusagt, und genau das tut C1.13: die
    /// Auswahl faellt mit jedem Inhaltswechsel, weil der Quellbezug an einer
    /// Stelle gesetzt und an einer zurueckgenommen wird. Ein dritter Rufer
    /// waere eine zweite Meinung darueber, wann ein Quellbezug gilt, und genau
    /// den soll diese Probe rot werden lassen.
    ///
    /// **Die zwei Haelften werden einzeln geprueft.** Die blosse Zahl zwei
    /// liesse zu, dass beide Rufer setzen und keiner zuruecknimmt; dann truege
    /// eine Auswahl im rohen Text der naechsten Datei den Quelltext der
    /// vorigen Markdown-Datei. Gesucht wird deshalb je eine Zeile mit `None`
    /// und je eine mit `Some(`.
    ///
    /// # Was diese Nadeln nicht sehen
    ///
    /// [`aufrufstellen`] zaehlt jede Empfaengerform und jeden Pfad, aber
    /// keinen Aufruf unter einem anderen Namen. Die zwei Nadeln der zweiten
    /// Haelfte zaehlen daneben **Codezeilen**: ein Aufruf, der zwischen dem
    /// Namen und seinem Argument umbricht, entginge ihnen. Beide Grenzen
    /// stehen im Kopf von [`crate::quellbaum`].
    #[test]
    fn der_quellbezug_wird_an_genau_zwei_stellen_gesetzt() {
        // Alle drei Nadeln stehen zusammengesetzt da: die Probe liegt in dem
        // Baum, den sie liest, und als ein Stueck geschrieben faende jede sich
        // selbst.
        let name = concat!("quellbezug_", "setzen");
        let ruecknahme = concat!("quellbezug_", "setzen(None)");
        let setzung = concat!("quellbezug_", "setzen(Some(");
        let vorschau = "krk-ui/src/appkit/vorschau.rs";

        let dateien = quelldateien();
        let stellen: Vec<(String, usize)> = dateien
            .iter()
            .map(|(datei, inhalt)| (datei.clone(), aufrufstellen(inhalt, name)))
            .filter(|(_, zahl)| *zahl > 0)
            .collect();
        assert_eq!(
            stellen,
            vec![(vorschau.to_owned(), 2)],
            "`{name}` wird nicht an genau zwei Stellen gerufen, und zwar in \
             der Vorschau; das Setzen und das Loeschen haben je genau einen Ort"
        );

        let (_, inhalt) = dateien
            .iter()
            .find(|(datei, _)| datei == vorschau)
            .expect("die Vorschau liegt im Quellbaum");
        let zeilen = |nadel: &str| -> usize {
            inhalt
                .lines()
                .filter(|zeile| !zeile.trim_start().starts_with("//"))
                .filter(|zeile| zeile.contains(nadel))
                .count()
        };
        assert_eq!(
            zeilen(ruecknahme),
            1,
            "`{ruecknahme}` steht nicht genau einmal; ohne die Ruecknahme in \
             `text_zeigen` truege der naechste Inhalt den Quellbezug des vorigen"
        );
        assert_eq!(
            zeilen(setzung),
            1,
            "`{setzung}` steht nicht genau einmal; gesetzt wird allein im \
             Markdown-Zweig von `anzeigen`"
        );
    }

    /// Die Abfangstelle des Kopierens steht im Baum genau einmal (C2.12 der
    /// Runde 14).
    ///
    /// **Zugesagt ist eine Stelle fuer alle Ausgabewege**, und das ist die
    /// eine **Ueberschreibung** von `writeSelectionToPasteboard:types:`. Eine
    /// zweite waere eine zweite Meinung darueber, was eine Auswahl aus KRK
    /// hergibt — und weil `copy:`, der Menueeintrag, das Kontextmenue, die
    /// Dienste und das Ziehen alle hier zusammenlaufen, waere sie zugleich
    /// eine Stelle, an der ein Teil der Wege einen anderen Text ablegte als
    /// der Rest.
    ///
    /// **Warum die Erwartung zwei Fundstellen nennt und nicht eine.** Die
    /// blosse Zeichenfolge steht im Programmtext zweimal, und beide Male zu
    /// Recht: einmal als Bezeichnung der Ueberschreibung im
    /// `#[unsafe(method(...))]`, einmal in der Weitergabe an die Oberklasse
    /// fuer den Fall ohne Quellbezug. Ein `msg_send!` an `super` kann den
    /// Selektor nicht anders nennen. Die Probe schreibt die Lage deshalb aus
    /// und prueft die beiden Haelften einzeln, statt eine Zahl zu erwarten,
    /// die am Baum von Anfang an rot waere; denselben Fehlgriff haben die
    /// Proben `die_zwei_schalter_stehen_je_an_genau_einer_stelle_und_dort`
    /// und `die_zuordnung_auf_eine_ansicht_steht_in_der_vorschau_genau_einmal`
    /// darueber schon einmal abgewehrt.
    ///
    /// # Was diese Nadeln nicht sehen
    ///
    /// Sie zaehlen Codezeilen. Eine Ueberschreibung, deren Attribut ueber zwei
    /// Zeilen umbricht, entginge ihnen, und ob dieselbe Sache anderswo unter
    /// einem anderen Selektor noch einmal abgefangen ist — etwa ueber
    /// `writeSelectionToPasteboard:type:` im Singular oder ueber
    /// `writablePasteboardTypes` —, entscheidet keine Suche im Quelltext. Der
    /// Kopf von [`crate::quellbaum`] sagt, was daraus folgt. **Dass die eine
    /// Stelle wirklich alle fuenf Ausgabewege traegt, misst diese Probe
    /// nicht**; das ist Erschliessung aus Apples Beschreibung und wird am
    /// laufenden Buendel abgenommen.
    #[test]
    fn die_abfangstelle_steht_im_baum_genau_einmal() {
        // Alle drei Nadeln stehen zusammengesetzt da: die Probe liegt in dem
        // Baum, den sie liest, und als ein Stueck geschrieben faende jede sich
        // selbst.
        let nadel = concat!("writeSelectionTo", "Pasteboard");
        let ueberschreibung = concat!("unsafe(method(writeSelectionTo", "Pasteboard:types:))");
        let weitergabe = concat!("super(self), writeSelectionTo", "Pasteboard:");
        let vorschau = "krk-ui/src/appkit/vorschau.rs";

        let dateien = quelldateien();
        let stellen: Vec<(String, usize)> = dateien
            .iter()
            .map(|(datei, inhalt)| {
                let zahl = inhalt
                    .lines()
                    .filter(|zeile| !zeile.trim_start().starts_with("//"))
                    .filter(|zeile| zeile.contains(nadel))
                    .count();
                (datei.clone(), zahl)
            })
            .filter(|(_, zahl)| *zahl > 0)
            .collect();
        assert_eq!(
            stellen,
            vec![(vorschau.to_owned(), 2)],
            "`{nadel}` steht nicht allein in der Vorschau und dort auf genau \
             zwei Zeilen; das Kopieren wird an genau einer Stelle abgefangen"
        );

        let (_, inhalt) = dateien
            .iter()
            .find(|(datei, _)| datei == vorschau)
            .expect("die Vorschau liegt im Quellbaum");
        let zeilen = |teil: &str| -> usize {
            inhalt
                .lines()
                .filter(|zeile| !zeile.trim_start().starts_with("//"))
                .filter(|zeile| zeile.contains(teil))
                .count()
        };
        assert_eq!(
            zeilen(ueberschreibung),
            1,
            "`{ueberschreibung}` steht nicht genau einmal; die Ueberschreibung \
             ist der eine Ausgang jeder Auswahl aus der Vorschau"
        );
        assert_eq!(
            zeilen(weitergabe),
            1,
            "`{weitergabe}` steht nicht genau einmal; ohne die Weitergabe an \
             die Oberklasse legte der Zweig ohne Quellbezug nichts ab"
        );
    }

    /// Die Zuordnung von einem Fokuswert auf eine Ansicht steht in der
    /// Vorschau genau einmal (C1.8 der Runde 14).
    ///
    /// **Erklaerungen und keine Aufrufer**, und der Unterschied ist hier der
    /// tragende: zugesagt ist, dass es bei **einer** Zuordnung bleibt. Wer die
    /// Verzweigung nach der sichtbaren Anzeige nicht in
    /// [`Vorschaufenster::fokusansicht`] legte, sondern als zweite Auskunft
    /// daneben, haette zwei Wahrheiten darueber, welche Flaeche zu
    /// `Fokus::Vorschau` gehoert — und keine Aufruferzahl saehe das.
    ///
    /// **Die zweite Fundstelle ist erwartet und kein Fehlschlag.**
    /// `Anwendungsdelegierter::fokusansicht` traegt denselben Namen und
    /// beantwortet die andere Haelfte derselben Frage: welcher Fokuswert
    /// welchem Bereich gehoert. Die Probe schreibt beide Stellen aus, statt
    /// eine Zahl zu erwarten, die die Lage nicht trifft; der Plan der Runde 14
    /// spricht an dieser Stelle nur von der Vorschau, und eine Erwartung ohne
    /// den Delegierten waere von Anfang an rot. Denselben Fehlgriff hat die
    /// Probe `die_zwei_schalter_stehen_je_an_genau_einer_stelle_und_dort`
    /// darueber schon einmal abgewehrt.
    ///
    /// # Was diese Nadel nicht sieht
    ///
    /// Sie zaehlt Codezeilen mit `fn` und dem Namen. Eine zweite Zuordnung
    /// unter einem **anderen** Namen entginge ihr vollstaendig; keine Suche im
    /// Quelltext entscheidet, ob dieselbe Sache anderswo noch einmal gebaut
    /// ist. Der Kopf von [`crate::quellbaum`] sagt, was daraus folgt.
    #[test]
    fn die_zuordnung_auf_eine_ansicht_steht_in_der_vorschau_genau_einmal() {
        // Zusammengesetzt, wie oben: als ein Stueck geschrieben faende die
        // Nadel sich selbst.
        let erklaerung = concat!("fn ", "fokusansicht");

        let stellen: Vec<(String, usize)> = quelldateien()
            .into_iter()
            .map(|(datei, inhalt)| {
                let zahl = inhalt
                    .lines()
                    .filter(|zeile| !zeile.trim_start().starts_with("//"))
                    .filter(|zeile| zeile.contains(erklaerung))
                    .count();
                (datei, zahl)
            })
            .filter(|(_, zahl)| *zahl > 0)
            .collect();

        assert_eq!(
            stellen,
            vec![
                ("krk-ui/src/appkit/anwendung.rs".to_owned(), 1),
                ("krk-ui/src/appkit/vorschau.rs".to_owned(), 1),
            ],
            "`{erklaerung}` steht nicht genau je einmal beim \
             Anwendungsdelegierten und in der Vorschau; die Verzweigung nach \
             der sichtbaren Anzeige gehoert in die eine Zuordnung und nicht \
             neben sie"
        );
    }
}
