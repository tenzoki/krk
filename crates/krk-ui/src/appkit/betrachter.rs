//! Der PDF-Betrachter der Vorschau: die Klasse [`Pdfbetrachter`], eine
//! Unterklasse von `PDFView` (Runde 20, C1.1, C1.2, C3.9, C5.2, C5.7, C5.8).
//!
//! **Ein Betrachter, ein Dokument, und die Vorschau haelt beides.** Die Klasse
//! wohnt in der Inhaltsflaeche des Vorschaufensters als dritte Ansicht neben
//! Textanzeige und Bildanzeige und wird dort erst gebaut, wenn das erste PDF
//! zu zeigen ist (Z2); das Einhaengen ist Sache von `super::vorschau`, nicht
//! dieses Moduls. Was hier steht, ist alles, was der Betrachter **selbst**
//! beantwortet: welche Bytes sein Dokument traegt, wie es ausgelegt ist, wie
//! weit es zoomt, was beim Kopieren, beim Rechtsklick und beim Klick auf
//! einen Verweis geschieht, und auf welcher Seite es steht.
//!
//! ```text
//! vorschau ──> Pdfbetrachter ──> PDFView (PDFKit)
//!                            ──> zwischenablage   (copy:, Verweis nach draussen)
//!                            ──> teilen           (der eine Menuebauer)
//!                            <── Rueckverweis, schwach: die Datei des aktiven Tabs
//!                            <── Seitenmelder: „die Seite hat gewechselt"
//! ```
//!
//! # Was gedeutet wird, und wo
//!
//! Das Dokument entsteht **aus den gelesenen Bytes** ueber
//! `PDFDocument::initWithData:` und nicht aus dem Pfad (Entscheidung 9 des
//! Plans): der Weg vom Pfad zu den Bytes ist `bis_zur_grenze_lesen` im Modell,
//! und ein `initWithURL:` oeffnete die Datei ein zweites Mal, an der Huelle
//! vorbei, ohne Groessengrenze und an einer benannten Roehre wieder
//! blockierend. Gedeutet wird auf dem Hauptfaden, weil ein `PDFDocument` ein
//! AppKit-Wert ist, der den Kanal vom Arbeitsfaden nicht ueberschreiten darf;
//! die Seiten zeichnet PDFKit auf eigenen Faeden.
//!
//! Die Antwort der Deutung ist [`Deutung`], vollstaendig und ohne
//! Auffangzweig: `Gesetzt`, `Beschaedigt` (PDFKit liefert kein Dokument) oder
//! `Gesperrt` (ein Dokument mit Kennwort). Was die Vorschau in den zwei
//! Rueckfaellen zeigt, entscheidet sie selbst (A9); dieses Modul zeigt nichts
//! und meldet nichts.
//!
//! # Der Merkposten
//!
//! [`Pdfbetrachter::dokument_setzen`] merkt sich, welche Bytes das gesetzte
//! Dokument traegt, und vergleicht ueber `Arc::ptr_eq` und nicht ueber den
//! Inhalt: dieselben Bytes wiederzusehen heisst, dass der Tab derselbe ist,
//! und dann bleibt das Dokument samt Zoom und Ausschnitt stehen (C1.7). Andere
//! Bytes ersetzen es und kommen in Ausgangsgroesse, denn A3 sagt ausdruecklich,
//! dass der Zoom nicht gemerkt wird.
//!
//! # Der Zoom
//!
//! Die Schrittweite gehoert PDFKit (`zoomIn:`, `zoomOut:`), die Grenzen
//! gehoeren KRK: [`ZOOM_MIN`] und [`ZOOM_MAX`] gehen ueber `setMinScaleFactor:`
//! und `setMaxScaleFactor:` an die Ansicht, und `canZoomIn`/`canZoomOut`
//! antworten an der Grenze `false` (C3.9, A2). Eine eigene Schrittweite ueber
//! `setScaleFactor:` waere eine zweite Zoomregel neben der der Trackpad-Geste,
//! die PDFKit selbst fuehrt (A4); mit `zoomIn:` teilen sich Taste und Geste
//! dieselbe Maschine. Die Ausgangsgroesse ist `autoScales`, also die Seite in
//! die Breite eingepasst, und sie folgt einer Groessenaenderung der Ansicht,
//! solange der Schalter steht (A1, C3.12).
//!
//! # Das Kopieren geht durch die eine Huelle
//!
//! `PDFView` beantwortet `copy:` selbst und legt dabei seine Auswahl ab. Die
//! Ueberschreibung hier nimmt ihm das ab und reicht den Text an
//! [`zwischenablage::text_schreiben`]; **keine Codezeile dieser Datei spricht
//! `NSPasteboard` an**, und die Zaehlprobe
//! `nspasteboard_steht_nicht_im_betrachter_und_copy_genau_einmal` haelt das
//! (C5.2, Constraint 3). Jeder der Wege aus C5.2 und C5.3 endet an dieser
//! einen Stelle — `cmd+c` ueber den Menueeintrag mit Ziel `nil`, der Eintrag
//! „Kopieren" des Hauptmenues, der Eintrag im Kontextmenue von `PDFView` —,
//! **und das ist Erschliessung aus dem Verhalten von Vorschau.app und der
//! Bindung `PDFView::copy:`, am Buendel abzunehmen wie die fuenf Wege der
//! Runde 14.** Ein Ziehen der Auswahl aus dem Betrachter heraus geht nicht
//! ueber diese Stelle und legt ab, was PDFKit ablegt; C5 nennt das Ziehen
//! nicht.
//!
//! # Der Rueckverweis und der Melder
//!
//! Der Rueckverweis auf das Vorschaufenster ist **schwach**, nach dem Muster
//! von `Inhaltsflaeche` in `super::vorschau`, sonst schloesse sich der Ring
//! Vorschaufenster → Betrachter → Rueckverweis → Vorschaufenster. Gebraucht
//! wird er fuer eine Frage: welche Datei der aktive Tab zeigt, damit das
//! Kontextmenue den Teilen-Eintrag bekommt (C5.8).
//!
//! Der Seitenwechsel geht nicht ueber den Rueckverweis, sondern ueber einen
//! **Melder**, wie `Hauptfenster::melder_setzen` und die uebrigen Melder
//! dieses Projekts: der Betrachter meldet „die Seite hat gewechselt", und wer
//! zuhoert, fragt [`Pdfbetrachter::seitenstand`] und schreibt die Statuszeile.
//! So kennt dieses Modul keine Methode des Vorschaufensters, die es fuer den
//! Seitenzaehler braeuchte; der Schritt, der den Betrachter einhaengt, traegt
//! den Melder ein.
//!
//! # Verweise
//!
//! Ein Klick auf einen Verweis nach draussen geht an den Systembrowser (A8,
//! C5.7), ueber [`zwischenablage::im_browser_oeffnen`] und damit ueber
//! dieselbe Stelle wie der Sprung aus der Zwischenablage. **Nur `http:` und
//! `https:` erreichen den Aufruf**, aus dem Grund, der an jener Funktion
//! steht (C9 der Runde 1): ein `smb:` oder `ftp:` aus einem fremden PDF baute
//! ueber das System die Serververbindung auf, die C9 ausschliesst. Ein
//! Verweis mit anderem Schema tut nichts. Verweise innerhalb der Datei
//! behandelt PDFKit vor der Delegiertenmethode selbst und blaettert dorthin.
//!
//! **Der Delegierte ist ein eigenes Objekt, [`Verweisdelegierter`], und nicht
//! die Ansicht selbst.** `PDFView` beantwortet mehrere Selektoren, die genau
//! so heissen wie die Methoden seines Delegierten, etwa
//! `PDFViewWillChangeScaleFactor:toScale:`, und reicht darin an den
//! Delegierten weiter, sobald der auf den Selektor antwortet. Eine `PDFView`,
//! die ihr eigener Delegierter ist, antwortet auf jeden davon — sie erbt die
//! Fassung, die weiterreicht — und ruft sich beim ersten Zoom selbst, bis der
//! Stapel ueberlaeuft (Absturzbericht 260828-0912). Das Objekt haelt keinen
//! Rueckverweis, denn es braucht keinen: seine eine Antwort geht an die
//! Zwischenablage und nicht an den Betrachter. Die Ansicht haelt es stark im
//! ivar, weil `PDFView` seinen Delegierten schwach haelt.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! Alle Angaben sind am SDK gelesen (`PDFKit.framework/Headers`), nicht aus
//! dem Plan uebernommen. PDFKit schreibt seine Angaben ueber
//! `PDFKIT_AVAILABLE(mac, ios)` und `PDFKIT_CLASS_AVAILABLE`, beide auf
//! `NS_AVAILABLE` abgebildet (`PDFKitPlatform.h:18-25`); `PDFKitPlatformView`
//! ist auf dem Mac `NSView` (`PDFKitPlatform.h:46`).
//!
//! **Die vier Klassen stehen seit 10.4:** `PDFView` (`PDFView.h:63-65`),
//! `PDFDocument` (`PDFDocument.h:129-130`), `PDFPage` (`PDFPage.h:49-50`) und
//! `PDFSelection` (`PDFSelection.h:28-29`). Was diese Datei an ihnen ruft und
//! keine eigene Angabe traegt, steht damit ebenfalls seit 10.4: an `PDFView`
//! die Eigenschaften `document` (`:79`), `currentPage` (`:107`),
//! `displayMode` (`:130`) samt der Aufzaehlung `PDFDisplayMode`
//! (`PDFKIT_ENUM_AVAILABLE(10_4, 11_0)`, `:24`), `displaysPageBreaks` (`:139`),
//! `delegate` (`:178`, **schwach** gehalten), `autoScales` (`:199`),
//! `canZoomIn` (`:208`) und `canZoomOut` (`:211`), die Aktionen `zoomIn:`
//! (`:207`), `zoomOut:` (`:210`) und `copy:` (`:276`), die Eigenschaft
//! `currentSelection` (`:235`) und die Meldung
//! `PDFViewPageChangedNotification` (`:53`, ausdruecklich `10_4`); an
//! `PDFDocument` `initWithData:` (`:139`), `isLocked` (`:164`), `pageCount`
//! (`:231`) und `indexForPage:` (`:238`); an `PDFSelection` `string` (`:52`).
//!
//! **Drei Stuecke sind juenger, und 10.13 ist die hoechste Angabe:**
//! `displayDirection` (`PDFView.h:134`, `PDFKIT_AVAILABLE(10_13, 11_0)`) samt
//! seiner Aufzaehlung `PDFDisplayDirection` (`:34`, `PDFKIT_ENUM_AVAILABLE(10_13,
//! 11_0)`), `minScaleFactor` (`:194`) und `maxScaleFactor` (`:195`), beide
//! `10_13`. Dazwischen liegt das Protokoll `PDFViewDelegate` (`:364`, ohne
//! eigene Angabe) mit der einen beantworteten Methode
//! `PDFViewWillClickOnLink:withURL:` (`:369`, `PDFKIT_AVAILABLE(10_5, 11_0)`),
//! beantwortet von [`Verweisdelegierter`], einer Unterklasse von `NSObject`
//! (`NSObject.h`, seit 10.0) mit dessen `init`.
//! `pageBreakMargins` und `scaleFactorForSizeToFit`, die der Plan als
//! 10.13-Stuecke nennt, ruft diese Datei nicht.
//!
//! **Aus AppKit und Foundation:** `NSView`s `initWithFrame:` (`NSView.h:83`)
//! und `menuForEvent:` (`NSView.h:291`), `NSMenu` (`NSMenu.h:56`) und `NSEvent`
//! (`NSEvent.h:317`), `NSNotificationCenter` (`NSNotification.h:37`) mit
//! `addObserver:selector:name:object:` (`:41`) und
//! `removeObserver:name:object:` (`:48`), `NSData`s `initWithBytes:length:`
//! (`NSData.h:113`, ueber `NSData::with_bytes` der Kiste) und an `NSURL`
//! `absoluteString` (`NSURL.h:108`) und `scheme` (`:115`): keines traegt eine
//! Verfuegbarkeitsangabe, alle stehen seit 10.0.
//!
//! Keine von ihnen ist nach macOS 15 hinzugekommen, und keine Beruehrung in
//! dieser Datei braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit.
//! `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer
//! haelt die Untergrenze nicht; die Nennung hier ist die Gegenmassnahme. Das
//! Buendel zielt auf 15.0 (`.cargo/config.toml`).

use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;

use objc2::rc::{Retained, Weak};
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{AnyThread, DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{NSEvent, NSMenu};
use objc2_core_foundation::CGFloat;
use objc2_foundation::{
    MainThreadMarker, NSData, NSNotification, NSNotificationCenter, NSObject, NSObjectProtocol,
    NSRect, NSURL,
};
use objc2_pdf_kit::{
    PDFDisplayDirection, PDFDisplayMode, PDFDocument, PDFView, PDFViewDelegate,
    PDFViewPageChangedNotification,
};

use super::teilen;
use super::vorschau::Vorschaufenster;
use super::zwischenablage;

/// Die Untergrenze des Zooms, als Faktor auf die natuerliche Seitengroesse
/// (C3.9, A2).
///
/// Ein Viertel: darunter ist eine Seite auf jedem Bildschirm dieses Projekts
/// ein Fleck ohne lesbare Zeile, und die Rolle aus C1.2 verliert ihren Sinn.
/// PDFKit haelt die Grenze selbst, sobald sie ueber `setMinScaleFactor:`
/// gesetzt ist: `canZoomOut` antwortet dann `false`, und die Trackpad-Geste
/// bleibt an derselben Stelle stehen wie die Taste.
pub const ZOOM_MIN: CGFloat = 0.25;

/// Die Obergrenze des Zooms, derselbe Faktor (C3.9, A2).
///
/// Das Achtfache: genug, um eine Fussnote auf einer A4-Seite bildschirmfuellend
/// zu lesen, und weit unter dem, wo PDFKit beim Zeichnen einer einzelnen Seite
/// mehr Speicher braeuchte als das ganze Dokument. Dieselbe Bauart wie die
/// Untergrenze, ueber `setMaxScaleFactor:` und `canZoomIn`.
pub const ZOOM_MAX: CGFloat = 8.0;

// Die zwei Grenzen schliessen die Ausgangsgroesse ein: beim Uebersetzen
// gehalten und nicht in einer Probe, nach dem Muster von `STAPELBUDGET` im
// Editor, denn eine Probe ueber zwei Konstanten prueft nichts, was der
// Uebersetzer nicht schon weiss.
const _: () = assert!(ZOOM_MIN > 0.0 && ZOOM_MIN < 1.0 && 1.0 < ZOOM_MAX);

/// Die drei Zoombefehle der Runde 20, so wie der Anwendungsdelegierte sie an
/// die Vorschau reicht (C3.1, A1, A2, A6).
///
/// Vollstaendig verzweigt in [`Pdfbetrachter::zoomen`]; ein vierter Wert haelt
/// den Bau dort an, statt still nichts zu tun.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zoom {
    /// `cmd+plus`: einen Schritt von PDFKit hinein, bis [`ZOOM_MAX`].
    Groesser,
    /// `cmd+minus`: einen Schritt von PDFKit hinaus, bis [`ZOOM_MIN`].
    Kleiner,
    /// `cmd+0`: die Seite in die Breite eingepasst, wie beim ersten Anzeigen.
    Ausgangsgroesse,
}

/// Was aus den Bytes geworden ist, die [`Pdfbetrachter::dokument_setzen`]
/// bekommen hat (C2.3 bis C2.5, A9).
///
/// Vollstaendig und ohne Auffangzweig beim Rufer: die Vorschau verzweigt ueber
/// alle drei Werte, und ein vierter haelt den Bau an. Die zwei Rueckfaelle
/// tragen keinen Text und keine Meldung — was der Nutzer statt des Dokuments
/// sieht, entscheidet die Vorschau, nicht der Betrachter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deutung {
    /// Das Dokument steht in der Ansicht, in Ausgangsgroesse oder, bei
    /// denselben Bytes wie zuvor, unveraendert (C1.7).
    Gesetzt,
    /// PDFKit hat aus den Bytes kein Dokument gemacht: abgeschnitten,
    /// umbenannter Text, kein PDF (C2.3, C2.4).
    Beschaedigt,
    /// Ein Dokument mit Kennwort. KRK fragt nicht danach (C2.5).
    Gesperrt,
}

/// Was der Betrachter neben seiner `PDFView` haelt.
pub struct PdfbetrachterIvars {
    /// Der schwache Rueckverweis auf das Vorschaufenster, fuer die eine Frage
    /// des Kontextmenues: welche Datei zeigt der aktive Tab.
    vorschau: RefCell<Option<Weak<Vorschaufenster>>>,
    /// Welche Bytes das gesetzte Dokument traegt; `None`, solange keines
    /// gesetzt ist. Verglichen wird ueber `Arc::ptr_eq`, siehe Modulkopf.
    bytes: RefCell<Option<Arc<Vec<u8>>>>,
    /// Der Melder, den [`Pdfbetrachter::seitenmelder_setzen`] eintraegt.
    seitenmelder: RefCell<Option<Box<dyn Fn()>>>,
    /// Der Delegierte der Ansicht, stark gehalten, weil `PDFView` ihn nur
    /// schwach haelt (`PDFView.h:178`); lebt so lange wie die Ansicht.
    delegierter: Retained<Verweisdelegierter>,
}

define_class!(
    /// Der Delegierte des Betrachters: beantwortet allein den Klick auf einen
    /// Verweis nach draussen (A8, C5.7).
    ///
    /// Ein eigenes Objekt und nicht die Ansicht; warum, steht im Modulkopf
    /// unter „Verweise". Es hat keine ivars und keinen Rueckverweis.
    // SAFETY:
    // - Die Oberklasse NSObject stellt an eine Unterklasse keine Bedingung;
    //   der Erzeuger ruft ihr `init`.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    pub struct Verweisdelegierter;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Verweisdelegierter {}

    // SAFETY: `PDFViewDelegate` stellt keine Bedingungen. Die Ansicht haelt
    // ihren Delegierten schwach (`PDFView.h:178`, `weak`), der Betrachter
    // haelt ihn stark; das Objekt haelt nichts zurueck, ein Ring entsteht
    // nicht.
    unsafe impl PDFViewDelegate for Verweisdelegierter {
        /// Ein Klick auf einen Verweis nach draussen (A8, C5.7).
        ///
        /// PDFKit ruft diese Methode fuer einen Verweis mit Adresse und
        /// oeffnet ihn danach **nicht** selbst; Verweise innerhalb der Datei
        /// erreichen sie nicht, die blaettert PDFKit vorher. Nur `http:` und
        /// `https:` gehen weiter, aus dem Grund im Modulkopf; alles andere tut
        /// nichts. Ob der Browser die Adresse angenommen hat, wird nicht
        /// gemeldet: der Betrachter hat keine Statuszeile, und ein Klick, auf
        /// den nichts folgt, ist dem Nutzer dieselbe Auskunft.
        // SAFETY: Die Signatur entspricht der des Protokolls
        // (`PDFView.h:369`): zwei Objektargumente, keine Rueckgabe.
        #[unsafe(method(PDFViewWillClickOnLink:withURL:))]
        fn verweis_geklickt(&self, _absender: &PDFView, adresse: &NSURL) {
            if !ist_webadresse(adresse) {
                return;
            }
            let Some(text) = adresse.absoluteString() else {
                return;
            };
            let _ = zwischenablage::im_browser_oeffnen(&text.to_string());
        }
    }
);

impl Verweisdelegierter {
    /// Ein Delegierter ohne Zustand.
    fn neu(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }
}

define_class!(
    /// Der PDF-Betrachter der Vorschau (Runde 20).
    ///
    /// Eine `PDFView`, die drei Dinge anders macht als die nackte Klasse: sie
    /// legt ihr Kopieren ueber die eine Huelle um `NSPasteboard` ab, sie
    /// haengt den Teilen-Eintrag in ihr Kontextmenue, und sie gibt einen
    /// Verweis nach draussen an den Systembrowser. Alles Uebrige — Rolle,
    /// Blaettern, Auswahl ueber Seitengrenzen, Trackpad-Geste — ist PDFKit
    /// und bleibt es. Der Modulkopf sagt, warum jedes der drei so gebaut ist.
    // SAFETY:
    // - Die Oberklasse PDFView stellt an eine Unterklasse keine Bedingung,
    //   die diese Klasse verletzen koennte: sie ruft den bezeichneten Erzeuger
    //   `initWithFrame:` der Oberklasse, sie ist nicht ihr eigener Delegierter
    //   (siehe `Verweisdelegierter`), und ihre zwei Ueberschreibungen
    //   reichen, was sie nicht selbst beantworten, unveraendert an die
    //   Oberklasse weiter (`menuForEvent:`) oder ersetzen die Antwort ganz
    //   (`copy:`, dessen Oberklassenfassung allein die Ablage schriebe, die
    //   hier durch die Huelle geht).
    // - Die Klasse implementiert `Drop`: er meldet den einen Beobachter wieder
    //   ab, ruft keine ueberschriebene Methode und haelt das Objekt nicht ueber
    //   die Lebensdauer des Aufrufs hinaus fest. Dieselbe Form wie
    //   `Nummernspalte`.
    #[unsafe(super = PDFView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = PdfbetrachterIvars]
    pub struct Pdfbetrachter;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Pdfbetrachter {}

    impl Pdfbetrachter {
        /// Der eine Ausgang jeder Auswahl aus dem Betrachter (C5.2, C5.5).
        ///
        /// Leere oder fehlende Auswahl: nichts geschieht, die Zwischenablage
        /// bleibt, wie sie war (C5.5). Sonst geht der reine Text der Auswahl
        /// an [`zwischenablage::text_schreiben`]. Der Rueckgabewert faellt mit
        /// `let _ =`: die zwei Pfadkopierer im Dateifenster melden ein
        /// Scheitern in der Statuszeile, weil sie eine haben; der Betrachter
        /// hat keine, und die Textanzeige der Vorschau meldet ihres seit der
        /// Runde 14 ebenso wenig.
        ///
        /// Die Oberklasse wird **nicht** gerufen: ihr `copy:` schriebe die
        /// Ablage ein zweites Mal, an der Huelle vorbei.
        // SAFETY: Die Signatur entspricht der Aktion von PDFView
        // (`PDFView.h:276`): ein optionales Objektargument, keine Rueckgabe.
        #[unsafe(method(copy:))]
        fn kopieren(&self, _absender: Option<&AnyObject>) {
            // SAFETY: Beide Aufrufe sind Leser ohne Vorbedingung; die Auswahl
            // gehoert der Ansicht, und `string` liefert eine Kopie.
            let text = unsafe { self.currentSelection() }
                .and_then(|auswahl| unsafe { auswahl.string() })
                .map(|text| text.to_string());
            if let Some(text) = text
                && !text.is_empty()
            {
                let _ = zwischenablage::text_schreiben(&text);
            }
        }

        /// Haengt den Teilen-Eintrag in das Kontextmenue der Ansicht (C5.8).
        ///
        /// Die Oberklasse baut ihr Menue selbst, und dieser Haken **ergaenzt**
        /// es, statt es zu ersetzen — dieselbe Wahl wie die Runde 14 fuer die
        /// Textanzeige: was PDFKit einer Auswahl gibt, bleibt stehen.
        /// [`teilen::eintrag_anfuegen`] bleibt die eine Stelle, die den
        /// Eintrag setzt; ohne teilbare Datei setzt sie keinen, und ohne
        /// Menue der Oberklasse gibt es dann auch keines.
        // SAFETY: Die Signatur entspricht der von NSView (`NSView.h:291`):
        // ein Objektargument, ein optionales Menue zurueck.
        #[unsafe(method_id(menuForEvent:))]
        fn kontextmenue(&self, ereignis: &NSEvent) -> Option<Retained<NSMenu>> {
            // SAFETY: Die Oberklasse beantwortet dieselbe Nachricht mit
            // demselben Argument und liefert ein optionales Menue.
            let bestand: Option<Retained<NSMenu>> =
                unsafe { msg_send![super(self), menuForEvent: ereignis] };
            let pfade = self.teilbare_pfade();
            // Kein `return` in diesem Rumpf: `method_id` huellt den Wert.
            let menue = match bestand {
                Some(menue) => Some(menue),
                None if pfade.is_empty() => None,
                None => Some(NSMenu::new(self.mtm())),
            };
            if let Some(menue) = &menue {
                teilen::eintrag_anfuegen(menue, &pfade, self.mtm());
            }
            menue
        }

        /// PDFKit meldet: eine andere Seite ist die aktuelle (C4.2).
        // SAFETY: Die Signatur passt zu der einer Meldungsannahme.
        #[unsafe(method(seiteGewechselt:))]
        fn seite_gewechselt(&self, _meldung: &NSNotification) {
            self.seiten_melden();
        }
    }
);

impl Pdfbetrachter {
    /// Ein Betrachter mit dem genannten Rahmen, noch ohne Dokument, ohne
    /// Rueckverweis und ohne Melder.
    ///
    /// Der Delegierte, ein [`Verweisdelegierter`], wird hier gebaut und
    /// gesetzt und nicht je Dokument: er gehoert der Ansicht, und ein Setzen
    /// bei jedem `dokument_setzen` truege dieselbe Zeile an einer zweiten
    /// Stelle. Der Beobachter fuer den Seitenwechsel
    /// wird ebenfalls hier angemeldet und in `Drop` wieder abgemeldet.
    pub fn neu(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let delegierter = Verweisdelegierter::neu(mtm);
        let this = Self::alloc(mtm).set_ivars(PdfbetrachterIvars {
            vorschau: RefCell::new(None),
            bytes: RefCell::new(None),
            seitenmelder: RefCell::new(None),
            delegierter,
        });
        // SAFETY: `initWithFrame:` von PDFView hat die hier angenommene
        // Signatur (`NSView.h:83`, bezeichneter Erzeuger).
        let betrachter: Retained<Self> = unsafe { msg_send![super(this), initWithFrame: rahmen] };

        // SAFETY: Der Delegierte wird schwach gehalten (`PDFView.h:178`), der
        // ivar haelt ihn stark ueber die Lebensdauer der Ansicht, und seine
        // Klasse erfuellt das Protokoll. Die Ansicht selbst darf es nicht
        // sein; warum, steht im Modulkopf unter „Verweise".
        unsafe {
            betrachter.setDelegate(Some(ProtocolObject::from_ref(
                &*betrachter.ivars().delegierter,
            )));
        }

        // SAFETY: `betrachter` ist von der Klasse, die den Selektor mit der
        // Signatur einer Meldungsannahme beantwortet. Der Beobachter wird in
        // `Drop` wieder abgemeldet, ueberlebt die Zentrale also nicht.
        // Dieselbe Form wie in `Nummernspalte::einhaengen`.
        unsafe {
            NSNotificationCenter::defaultCenter().addObserver_selector_name_object(
                &betrachter,
                sel!(seiteGewechselt:),
                Some(PDFViewPageChangedNotification),
                Some(&betrachter),
            );
        }
        betrachter
    }

    /// Traegt den Rueckverweis nach, sobald es das Vorschaufenster gibt.
    pub fn ziel_setzen(&self, vorschau: &Vorschaufenster) {
        *self.ivars().vorschau.borrow_mut() = Some(Weak::from_retained(&vorschau.retain()));
    }

    /// Traegt den Melder ein, der jeden Seitenwechsel weitergibt (C4.2).
    ///
    /// Gerufen von dem, der den Betrachter einhaengt, mit einem Rueckruf, der
    /// sein Ziel schwach haelt. Derselbe Zuschnitt wie
    /// `Hauptfenster::melder_setzen` und die uebrigen Melder dieses Projekts.
    pub fn seitenmelder_setzen(&self, melden: Box<dyn Fn()>) {
        *self.ivars().seitenmelder.borrow_mut() = Some(melden);
    }

    /// Gibt den Seitenwechsel weiter, falls jemand zuhoert.
    ///
    /// Die Ausleihe steht waehrend des Rufs und ist lesend; der einzige
    /// schreibende Zugriff auf dieselbe Zelle ist
    /// [`Self::seitenmelder_setzen`] beim Einhaengen.
    fn seiten_melden(&self) {
        let melden = self.ivars().seitenmelder.borrow();
        if let Some(melden) = melden.as_ref() {
            melden();
        }
    }

    /// Setzt das Dokument aus den gelesenen Bytes, oder laesst es stehen
    /// (C1.1, C1.2, C1.7, C2.3 bis C2.5, A1, A2).
    ///
    /// **Dieselben Bytes wie zuvor: nichts geschieht, `Gesetzt`.** Verglichen
    /// wird ueber `Arc::ptr_eq`, siehe Modulkopf; Zoom und Ausschnitt bleiben.
    /// Andere Bytes gehen an `PDFDocument::initWithData:`; liefert PDFKit kein
    /// Dokument, ist die Antwort `Beschaedigt`, traegt es ein Kennwort,
    /// `Gesperrt`, und in beiden Faellen bleibt das vorige Dokument samt
    /// Merkposten stehen — die Vorschau blendet den Betrachter dann aus, und
    /// kommen die vorigen Bytes zurueck, steht ihr Dokument noch.
    ///
    /// Bei `Gesetzt` wird die Auslegung jedes Mal neu gesetzt: fortlaufende
    /// Rolle, senkrecht, mit Seitenabstand (C1.2), die zwei Zoomgrenzen und
    /// `autoScales`, also die Ausgangsgroesse (A1, A3). Ein `zoomIn:` nimmt
    /// den Schalter zurueck, `cmd+0` setzt ihn wieder.
    #[must_use]
    pub fn dokument_setzen(&self, daten: &Arc<Vec<u8>>) -> Deutung {
        let dieselben = self
            .ivars()
            .bytes
            .borrow()
            .as_ref()
            .is_some_and(|bisher| Arc::ptr_eq(bisher, daten));
        if dieselben {
            return Deutung::Gesetzt;
        }

        let rohdaten = NSData::with_bytes(daten);
        // SAFETY: `initWithData:` ist der bezeichnete Erzeuger von PDFDocument
        // (`PDFDocument.h:139`) und liefert `nil`, wenn die Bytes kein
        // Dokument ergeben; `isLocked` ist ein Leser ohne Vorbedingung.
        let Some(dokument) =
            (unsafe { PDFDocument::initWithData(PDFDocument::alloc(), &rohdaten) })
        else {
            return Deutung::Beschaedigt;
        };
        if unsafe { dokument.isLocked() } {
            return Deutung::Gesperrt;
        }

        // SAFETY: Sieben Setzer von PDFView ohne Vorbedingung; die Werte sind
        // die der Aufzaehlungen des SDK und zwei Gleitkommazahlen innerhalb
        // des Bereichs, den `scaleFactor` annimmt.
        unsafe {
            self.setDocument(Some(&dokument));
            self.setDisplayMode(PDFDisplayMode::SinglePageContinuous);
            self.setDisplayDirection(PDFDisplayDirection::Vertical);
            self.setDisplaysPageBreaks(true);
            self.setMinScaleFactor(ZOOM_MIN);
            self.setMaxScaleFactor(ZOOM_MAX);
            self.setAutoScales(true);
        }
        *self.ivars().bytes.borrow_mut() = Some(Arc::clone(daten));
        Deutung::Gesetzt
    }

    /// Fuehrt einen der drei Zoombefehle aus (C3.1, C3.9, A1, A2, A6).
    ///
    /// Liefert, ob sich etwas geaendert hat: an der Grenze antwortet PDFKit
    /// mit `canZoomIn`/`canZoomOut` `false`, und dann geschieht nichts und
    /// wird nichts gemeldet (C3.9). Die Ausgangsgroesse ist immer erreichbar.
    /// Der Rueckgabewert traegt `#[must_use]`, weil ein Rufer, der ihn fallen
    /// laesst, die Grenze nicht von einem Schritt unterscheiden kann.
    #[must_use]
    pub fn zoomen(&self, zoom: Zoom) -> bool {
        // SAFETY: Leser und Aktionen von PDFView ohne Vorbedingung; `None` ist
        // das zulaessige Argument einer Aktion ohne Absender.
        unsafe {
            match zoom {
                Zoom::Groesser => {
                    if !self.canZoomIn() {
                        return false;
                    }
                    self.zoomIn(None);
                }
                Zoom::Kleiner => {
                    if !self.canZoomOut() {
                        return false;
                    }
                    self.zoomOut(None);
                }
                Zoom::Ausgangsgroesse => self.setAutoScales(true),
            }
        }
        true
    }

    /// Auf welcher Seite der Betrachter steht, und wie viele es sind, beide
    /// ab eins (A5, C4.1, C4.3).
    ///
    /// `None` ohne Dokument, ohne aktuelle Seite oder wenn PDFKit fuer die
    /// aktuelle Seite keinen Index im Dokument nennt (`NSNotFound`, also ein
    /// Wert ausserhalb der Seitenzahl). Welche Seite die aktuelle ist,
    /// entscheidet `currentPage` von PDFKit; ob das die Seite mit der meisten
    /// Flaeche ist, wie A5 es sagt, ist am Buendel zu sehen.
    pub fn seitenstand(&self) -> Option<(usize, usize)> {
        // SAFETY: Vier Leser ohne Vorbedingung; `seite` stammt aus demselben
        // Dokument, dem sie zum Nachschlagen gereicht wird.
        unsafe {
            let dokument = self.document()?;
            let seite = self.currentPage()?;
            let gesamt = dokument.pageCount();
            let index = dokument.indexForPage(&seite);
            if index >= gesamt {
                return None;
            }
            Some((index + 1, gesamt))
        }
    }

    /// Was ein Rechtsklick in den Betrachter zu teilen findet (C5.8).
    ///
    /// Keine oder eine Datei, wie an den zwei anderen Ansichten der Vorschau:
    /// der aktive Tab zeigt hoechstens eine. Ohne Rueckverweis oder mit einem
    /// Vorschaufenster, das nicht mehr steht, bleibt die Liste leer.
    fn teilbare_pfade(&self) -> Vec<PathBuf> {
        let vorschau = self.ivars().vorschau.borrow().as_ref().and_then(Weak::load);
        vorschau
            .and_then(|vorschau| vorschau.angezeigter_pfad())
            .into_iter()
            .collect()
    }
}

impl Drop for Pdfbetrachter {
    /// Meldet den Beobachter aus [`Pdfbetrachter::neu`] wieder ab.
    fn drop(&mut self) {
        let selbst: &AnyObject = &*self;
        // SAFETY: `selbst` ist der Beobachter, den `neu` fuer die Meldung
        // angemeldet hat. Ohne Gegenstand nimmt die Zentrale ihn fuer die
        // genannte Meldung heraus, gleich bei welchem Absender er eingetragen
        // war. Der Aufruf haelt `selbst` nicht fest und ruft keine
        // ueberschriebene Methode.
        unsafe {
            NSNotificationCenter::defaultCenter().removeObserver_name_object(
                selbst,
                Some(PDFViewPageChangedNotification),
                None,
            );
        }
    }
}

/// Ob eine Adresse `http:` oder `https:` traegt, ohne Ruecksicht auf die
/// Schreibung des Schemas.
///
/// Die eine Grenze vor [`zwischenablage::im_browser_oeffnen`] auf dem Weg
/// aus einem PDF; warum sie noetig ist, steht im Modulkopf unter „Verweise".
fn ist_webadresse(adresse: &NSURL) -> bool {
    adresse
        .scheme()
        .is_some_and(|schema| ist_webschema(&schema.to_string()))
}

/// Die reine Regel hinter [`ist_webadresse`], ohne Fenster pruefbar.
fn ist_webschema(schema: &str) -> bool {
    schema.eq_ignore_ascii_case("http") || schema.eq_ignore_ascii_case("https")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quellbaum::quelldateien;

    /// Der Pfad dieser Datei, wie [`quelldateien`] ihn fuehrt.
    const DIESE_DATEI: &str = "krk-ui/src/appkit/betrachter.rs";

    /// Der Inhalt dieser Datei aus dem Quellbaum.
    fn quelltext() -> String {
        quelldateien()
            .into_iter()
            .find(|(datei, _)| datei == DIESE_DATEI)
            .map(|(_, inhalt)| inhalt)
            .expect("der Betrachter liegt im Quellbaum")
    }

    /// Die Varianten einer Aufzaehlung ohne Daten, aus dem Quelltext gelesen —
    /// nach der Lesart von `varianten_der_aufzaehlung` in
    /// `krk-core/tests/gemeinsam`, die diese Kiste nicht erreicht.
    fn varianten(inhalt: &str, name: &str) -> Vec<String> {
        let kopf = format!("pub enum {name} {{");
        inhalt
            .lines()
            .skip_while(|zeile| *zeile != kopf)
            .skip(1)
            .take_while(|zeile| *zeile != "}")
            .map(str::trim)
            .filter(|zeile| {
                !zeile.is_empty() && !zeile.starts_with("//") && !zeile.starts_with("#[")
            })
            .map(|zeile| zeile.trim_end_matches(',').to_owned())
            .collect()
    }

    /// Die Codezeilen dieser Datei, also ohne Kommentarzeilen.
    fn codezeilen(inhalt: &str) -> impl Iterator<Item = &str> {
        inhalt
            .lines()
            .filter(|zeile| !zeile.trim_start().starts_with("//"))
    }

    /// `Zoom` traegt genau die drei Befehle, die die Belegung fuehrt, und
    /// `Deutung` genau die drei Antworten, ueber die die Vorschau verzweigt.
    ///
    /// Der Uebersetzer haelt die Vollstaendigkeit der Verzweigungen; was er
    /// nicht haelt, ist die Zahl selbst. Diese Probe schreibt sie aus, damit
    /// ein vierter Wert nicht still dazukommt, sondern hier rot wird und
    /// bewusst eingeordnet werden muss.
    #[test]
    fn zoom_und_deutung_tragen_je_genau_drei_werte() {
        let inhalt = quelltext();
        assert_eq!(
            varianten(&inhalt, "Zoom"),
            ["Groesser", "Kleiner", "Ausgangsgroesse"],
            "Zoom traegt nicht genau die drei Befehle der Runde 20"
        );
        assert_eq!(
            varianten(&inhalt, "Deutung"),
            ["Gesetzt", "Beschaedigt", "Gesperrt"],
            "Deutung traegt nicht genau die drei Antworten aus A9"
        );
        let auffangzweige = codezeilen(&inhalt)
            .filter(|zeile| zeile.trim_start().starts_with("_ =>"))
            .count();
        assert_eq!(
            auffangzweige, 0,
            "eine Verzweigung in dieser Datei traegt einen Auffangzweig"
        );
    }

    /// Keine Codezeile dieser Datei nennt `NSPasteboard`, und `copy:` ist im
    /// ganzen Baum genau einmal ueberschrieben, naemlich hier (C5.2,
    /// Constraint 3).
    ///
    /// Gezaehlt werden Codezeilen: der Modulkopf **nennt** die Klasse, um zu
    /// sagen, dass er sie nicht anspricht, und das ist keine Beruehrung. Die
    /// Nadeln stehen zusammengesetzt da: die Probe liegt in dem Baum, den sie
    /// liest, und als ein Stueck geschrieben faende sie sich selbst.
    #[test]
    fn nspasteboard_steht_nicht_im_betrachter_und_copy_genau_einmal() {
        let huelle = concat!("NSPaste", "board");
        let ueberschreibung = concat!("unsafe(method(co", "py:))");

        let dateien = quelldateien();
        let (_, inhalt) = dateien
            .iter()
            .find(|(datei, _)| datei == DIESE_DATEI)
            .expect("der Betrachter liegt im Quellbaum");
        let nennungen = codezeilen(inhalt)
            .filter(|zeile| zeile.contains(huelle))
            .count();
        assert_eq!(
            nennungen, 0,
            "eine Codezeile des Betrachters nennt `{huelle}`; die eine Huelle ist `zwischenablage.rs`"
        );

        let stellen: Vec<(String, usize)> = dateien
            .iter()
            .map(|(datei, inhalt)| {
                let zahl = codezeilen(inhalt)
                    .filter(|zeile| zeile.contains(ueberschreibung))
                    .count();
                (datei.clone(), zahl)
            })
            .filter(|(_, zahl)| *zahl > 0)
            .collect();
        assert_eq!(
            stellen,
            vec![(DIESE_DATEI.to_owned(), 1)],
            "`copy:` ist nicht allein im Betrachter und dort genau einmal ueberschrieben"
        );
    }

    /// Nur `http` und `https` gehen an den Systembrowser, gleich wie
    /// geschrieben (A8, C9 der Runde 1).
    #[test]
    fn allein_http_und_https_sind_webschemata() {
        for schema in ["http", "https", "HTTP", "Https"] {
            assert!(ist_webschema(schema), "{schema} ist ein Webschema");
        }
        for schema in ["smb", "ftp", "mailto", "file", "", "httpx"] {
            assert!(!ist_webschema(schema), "{schema} ist keines");
        }
    }
}
