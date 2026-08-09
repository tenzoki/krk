//! Die Textflaeche des eingebauten Editors: eine `NSTextView` in einer
//! `NSScrollView`, angebunden an das Modell aus [`crate::editormodell`]
//! (C1 bis C6).
//!
//! ```text
//! ┌──────────────────────────────┐
//! │ • lies.md                    │  der Kopf: Dateiname und Abweichungszeichen
//! ├──────────────────────────────┤
//! │ NSScrollView                 │  der fuenfte Bereich der Fensterzeile
//! │  ┌────┬─────────────────────┐│
//! │  │ 12 │ NSTextView          ││  editierbar, ein Textspeicher
//! │  └────┴─────────────────────┘│  links die Nummernspalte aus C10
//! └──────────────────────────────┘
//! ```
//!
//! # Der Kreis, den diese Datei schliesst
//!
//! ```text
//!   F4 ──> datei_oeffnen ──> Editormodell::oeffnen ──┐
//!                                                    │ Arbeitsfaden
//!   Einzugstakt (1/60 s) ──> Editormodell::einziehen <┘
//!            │
//!            ├─ Geoeffnet ──> stand_einsetzen ──> NSTextView
//!            └─ jeder Ausgang ──> melden ──> Anwendungsdelegierter
//!                                                 │ Zurueckgehalten: Blatt
//!   zurueckgehaltenes_uebernehmen  <───────────────┤ (sichern / verwerfen)
//!   zurueckgehaltenes_fallenlassen <───────────────┘ (abbrechen)
//!
//!   opt+cmd+e ──> Blatt ──> schliessen ──> stand_einsetzen, kopf_nachziehen
//!
//!   Tippen ──> textDidChange: ──> Editormodell::bearbeiten ──> kopf_nachziehen
//!
//!   cmd+s ──> sichern ──> Editormodell::sichern ──┬─ gelungen ─> kopf_nachziehen
//!                                                 └─ jeder Ausgang ─> nach oben
//! ```
//!
//! **Der untere Pfeil ist der Rueckweg, und ohne ihn ist das Modell blind.**
//! Bis S26 hatte [`Editormodell::bearbeiten`] keinen Aufrufer: das Getippte
//! stand allein in der `NSTextView`, `hat_ungesicherten_stand` blieb `false`,
//! und ein Sichern schriebe den Plattenstand zurueck und meldete Erfolg
//! (`issues/260809-2148_*_s25-sichern-schriebe-den-plattenstand-weil-die-rueckschreibung-erst-s26-baut.md`).
//! `textDidChange:` ist die eine Stelle, die AppKit dafuer vorsieht.
//!
//! **`setString:` loest den Rueckweg nicht aus.** Eine `NSTextView` meldet ihrem
//! Delegierten allein die Aenderungen des Nutzers; ein programmatisch gesetzter
//! Text laeuft an `didChangeText` vorbei. Darauf ruht, dass eine frisch
//! geoeffnete Datei nicht sofort als geaendert gilt — sichtbar wird ein Bruch
//! dieser Annahme sofort, naemlich als Abweichungszeichen am Kopf einer eben
//! geoeffneten Datei.
//!
//! **Die Nummernspalte ist nicht hier gebaut, sondern eingehaengt.**
//! [`super::nummernspalte`] haelt sie, und die Vorschau haengt dieselbe Klasse
//! ein; C10 sagt eine Anzeige fuer beide Flaechen zu und nicht zwei aehnliche.
//! Im Editor steht sie immer: der Spec laesst sie nicht abschalten.
//!
//! **Was hier steht und was im Modell.** Welche Datei der Editor haelt, ihr
//! Stand, ob er von der Datei abweicht, die Ansichtswahl, der Dateityp und der
//! Suchlauf wohnen in [`Editormodell`] und damit ausserhalb von `appkit/`.
//! Diese Datei setzt den gehaltenen Stand in die Textflaeche um und rechnet
//! ihn nicht nach. Derselbe Schnitt wie [`super::vorschau`] neben
//! [`crate::vorschaumodell`] und [`super::tabelle`] neben [`crate::tabs`].
//!
//! **Die Textflaeche geht als Objekt nach aussen, nicht als Klasse.**
//! [`Editorbereich::textflaeche`] ist die eine Zugriffsfunktion darauf, und sie
//! beantwortet die Naemlichkeitsfrage des Fokusvorbehalts: der Ereignisabgriff
//! reicht jeden Tastendruck an AppKit weiter, sobald der Ersthelfer eine
//! `NSTextView` **ist**, und diese eine ist die Ausnahme davon. Gefragt wird
//! nach der Naemlichkeit und nicht nach der Art, denn der Feldeditor eines
//! Textfeldes ist ebenfalls eine `NSTextView`, und eine Frage nach der Art
//! trennte die beiden nicht. Der Vergleich selbst steht beim
//! Anwendungsdelegierten, der die Flaeche haelt; `super::ereignisse` kennt den
//! Editor nicht.
//!
//! **Ein Textspeicher und kein zweiter Textbestand.** Die `NSTextView` bringt
//! ihren `NSTextStorage`, ihren `NSLayoutManager` und ihren `NSTextContainer`
//! selbst mit; KRK baut keinen davon von Hand und haelt daneben keine zweite
//! Zeichenkette. Der Stand steht im Modell, die Darstellung in der Flaeche, und
//! [`Editorbereich::stand_einsetzen`] ist die eine Stelle, die den Text der
//! Flaeche ersetzt.
//!
//! **Der Kopf ist die zweite Anzeige neben der Statuszeile, und er ist eine
//! andere Art von Aussage.** Die Statuszeile traegt Antworten auf Befehle; der
//! Kopf traegt einen Zustand, naemlich welche Datei der Editor haelt und ob ihr
//! Stand von der Platte abweicht. Das zweite Abnahmekriterium von C4 verlangt
//! ausdruecklich, dass der Nutzer den ungesicherten Stand **ohne** Hinsehen auf
//! die Statuszeile bemerkt; eine Meldung dort waere die falsche Form, weil sie
//! mit dem naechsten Befehl verschwaende. Eine zweite Meldeflaeche entsteht
//! damit nicht: der Kopf beantwortet keine Frage und meldet kein Ereignis.
//!
//! **Was der Editor zu melden hat, geht als Wert nach oben und nicht als
//! fertige Zeile an eine Flaeche.** [`Editormeldung`] benennt es; wohin es
//! geht, weiss diese Datei nicht. Der Anwendungsdelegierte nimmt den Wert und
//! stellt ihn in die **eine** Meldeflaeche des Fensters aus C1 der Runde 1, auf
//! den obersten ihrer fuenf Raenge. Eine zweite Meldeflaeche neben ihr entsteht
//! nicht: die Uebergabe an diese Runde sagt das zu, und C1 wiederholt es unter
//! "Der Editor bekommt keine eigene Meldezeile".
//!
//! **Reiner Text.** `setRichText(false)` und die vier abgeschalteten
//! Ersetzungen halten fest, was der Nutzer tippt: eine Zeichenkette, die beim
//! Sichern Zeichen fuer Zeichen wieder in der Datei steht. Eine typografische
//! Ersetzung von Anfuehrungszeichen oder Bindestrichen aendert Programmtext
//! still, und die Zusage aus C4 lautet, dass der gesicherte Stand der getippte
//! ist.
//!
//! **Die Formatansicht aus C3 widerspricht dem nicht, und der Grund ist nicht,
//! wo ihre Merkmale liegen.** Sie setzt Farbe und Unterstreichung als
//! voruebergehende Merkmale des Layoutverwalters und die Markdown-Auszeichnung
//! als Merkmale des Textspeichers; warum sie geteilt werden muss, steht im
//! Modulkopf von [`crate::hervorhebung`]. In die Datei geraet weder das eine
//! noch das andere, weil der Sicherungsweg
//! [`Editormodell::stand`](crate::editormodell::Editormodell::stand) schreibt
//! und der aus `NSTextView::string` kommt — den **Zeichen** der Flaeche. Kein
//! Merkmal wird auf diesem Weg auch nur gelesen.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSScrollView`, `NSTextView`, `NSTextStorage`, `NSLayoutManager`,
//! `NSTextContainer`, `NSTextField` und `NSTimer` stehen seit macOS 10.0 zur
//! Verfuegung; das Buendel zielt auf 15.0 (`.cargo/config.toml`). Keine von
//! ihnen ist nach macOS 15 hinzugekommen, und deshalb braucht keine der
//! Beruehrungen in dieser Datei eine Verfuegbarkeitspruefung zur Laufzeit.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use objc2::rc::{Retained, Weak};
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAppearanceCustomization, NSAppearanceNameAqua, NSAppearanceNameDarkAqua,
    NSAutoresizingMaskOptions, NSColor, NSFont, NSFontAttributeName,
    NSForegroundColorAttributeName, NSMutableParagraphStyle, NSParagraphStyleAttributeName,
    NSScrollView, NSTextAlignment, NSTextDelegate, NSTextField, NSTextView, NSTextViewDelegate,
    NSUnderlineStyle, NSUnderlineStyleAttributeName, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSDictionary, NSNotification, NSNumber, NSObject, NSObjectProtocol,
    NSPoint, NSRange, NSRect, NSRunLoop, NSRunLoopCommonModes, NSSize, NSString, NSTimeInterval,
    NSTimer, ns_string,
};

use krk_core::text::{Abweisung, Fund, Markensprung};

use crate::editormodell::{Ansicht, Editormodell, Ladeausgang, Sicherungsausgang};
use crate::hervorhebung::{
    Abholung, Auszeichnung, Darstellungsart, Einfaerbungsvorgang, Farbe, Formatierung, Tafel,
};

use super::nummernspalte::{self, Nummernspalte};
use super::statuszeile;

/// Was der Editor dem Nutzer zu sagen hat (C1, C2, C6).
///
/// **Ein Wert und keine Zeichenkette am Meldeort.** Jede Meldung des Editors
/// ist die Antwort auf einen Tastenbefehl, und jede geht denselben einen Weg
/// nach oben; der Wortlaut steht deshalb hier an einer Stelle und nicht bei den
/// sechs Befehlen, die ihn ausloesen. Wer eine siebte Meldung braucht, setzt
/// eine Variante dazu und bekommt vom Uebersetzer die fehlende Zeile in
/// [`Self::text`] angezeigt.
///
/// **Die Aufzaehlung ist vollstaendig und hat keinen Auffangzweig**, wie die
/// drei uebrigen dieser Art im Programm. Sie ist heute kurz, weil erst zwei der
/// sechs Ausloeser gebaut sind; die vier uebrigen kommen mit ihren Schritten und
/// tragen ihre Variante bei:
///
/// ```text
///  gebaut    Abweisung beim Oeffnen        krk_core::text::datei::oeffnen (S10)
///  gebaut    Markenstelle geaendert        krk_core::text::marke (S12)
///  gebaut    gelungenes Sichern            krk_core::text::datei::sichern (S9)
///  gebaut    gescheitertes Sichern         dieselbe Stelle (S25)
///  offen     Zeilennummer ueber der Zahl   S35
///  offen     Suche ohne Treffer            S36
///  offen     Zahl der ersetzten Treffer    S37
/// ```
///
/// **Das gelungene Sichern meldet sich, obwohl der Kopf es schon zeigt.** Die
/// beiden sagen Verschiedenes: der Kopf traegt den Zustand, naemlich dass nichts
/// mehr abweicht, und die Statuszeile die Antwort auf den Tastendruck, naemlich
/// dass eben geschrieben wurde. Wer `cmd+s` an einer unveraenderten Datei
/// drueckt, sieht am Kopf nichts geschehen und braucht trotzdem eine Antwort;
/// kommentarlos nichts zu tun ist in keinem Fall zulaessig.
///
/// **Kommentarlos nichts zu tun ist in keinem Fall zulaessig**; das steht so im
/// zehnten Abnahmekriterium von C2 und im achten von C6, und dieser Wert ist
/// die Form, in der ein Befehl seinen Grund abgibt.
///
/// **Der erste Ausloeser steht seit S22**: F4 weist eine Datei ab und gibt den
/// Grund ueber [`Self::Abgewiesen`] nach oben. Bis dahin trugen dieser Wert und
/// sein Rumpf je ein `#[allow(dead_code)]`; beide sind mit dem Ausloeser
/// gefallen. Zwei Zeilen sind geblieben, an
/// [`Self::MarkenstelleGeaendert`] und an [`Self::markenstelle`]: deren
/// Ausloeser ist der Sprung auf eine Textmarke und nicht F4. Die Ankuendigung
/// aus S21, S22 loese beide ab, war fuer die Haelfte richtig.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Editormeldung {
    /// Der Editor nimmt die Datei nicht an (C2).
    ///
    /// Die drei Gruende bleiben unterschieden, weil das zehnte
    /// Abnahmekriterium von C2 es verlangt und weil der Datensatz
    /// `decisions/260807-2147_a_welche-dateien-oeffnet-der-editor-ueberhaupt.md`
    /// es ausdruecklich fordert. Unterschieden werden sie in
    /// [`Abweisung::meldung`] und hier nicht ein zweites Mal.
    Abgewiesen(Abweisung),
    /// Die Textmarke ist gesprungen, aber ihre gemerkte Stelle war fort (C6).
    ///
    /// Der gemerkte Zeileninhalt stand weder auf der gemerkten Nummer noch in
    /// den `krk_core::text::marke::NAHFENSTER` Zeilen darum. Die Marke fuehrt
    /// **trotzdem** an die gemerkte Nummer; gemeldet wird, dass die Stelle
    /// sich geaendert hat, statt kommentarlos irgendwohin zu fuehren.
    // **Diese Zeile faellt mit dem Sprung auf eine Textmarke**, dem Ausloeser
    // dieser Variante. S21 hat S22 fuer beide Meldungen angekuendigt; das war
    // fuer die Abweisung richtig und fuer die Marke zu frueh — F4 weist Dateien
    // ab und laesst keine Marke springen. Bis dahin fasst die Pruefung
    // `allein_die_nicht_wiedergefundene_markenstelle_meldet_sich` am Dateiende
    // jeden Zweig an, tot ist also nichts. Ohne die Zeile stuende der
    // Arbeitsbereich rot, weil `make lint` mit `-D warnings` faehrt.
    #[allow(dead_code)]
    MarkenstelleGeaendert {
        /// Die gemerkte Zeilennummer, ab 1 gezaehlt, an die die Marke gefuehrt
        /// hat.
        zeile: u32,
    },
    /// Der Stand steht in der Datei (C4).
    Gesichert {
        /// Die geschriebene Datei. Der volle Pfad, wie bei jeder anderen
        /// Meldung ueber eine Datei; der Kopf des Editorbereichs nennt daneben
        /// den blossen Namen.
        pfad: PathBuf,
    },
    /// Es wurde nicht geschrieben, und der Stand des Editors steht unveraendert
    /// da (C4).
    ///
    /// **Der Satz kommt fertig aus dem Modell** und wird hier nicht ein zweites
    /// Mal gebaut: [`crate::editormodell::Sicherungsausgang::Gescheitert`]
    /// traegt ihn, weil dort entschieden wird, woran es lag — am Schreiben
    /// selbst oder an einer Datei, die sich von aussen geaendert hat.
    SichernGescheitert {
        /// Der Grund, wie das Modell ihn formuliert hat.
        grund: String,
    },
}

impl Editormeldung {
    /// Die Meldung des Markensprungs, falls er eine hat (C6).
    ///
    /// **Die Fallunterscheidung ueber den Fund steht hier und nicht beim
    /// Aufrufer.** Ein getroffener und ein verschobener Sprung melden nichts,
    /// weil beide an der richtigen Stelle landen; allein der dritte Fall
    /// meldet. Ein vierter Fund haelt den Bau an und erzwingt die Antwort auf
    /// die Frage, ob er zu melden ist.
    ///
    /// **Das beantwortet die eine Haelfte der Auskunft und nicht beide.**
    /// `krk_core::text::Markensprung` traegt zwei verschiedene Auskuenfte: ob
    /// der gemerkte Inhalt wiedergefunden wurde ([`Markensprung::fund`]) und ob
    /// die angesteuerte Nummer im Text ueberhaupt vorkommt
    /// (`Markensprung::sprung.lage`). Diese Funktion beantwortet die erste. Die
    /// zweite ist die Meldung der Zeilenlage aus C5, die mit S35 kommt; wie die
    /// beiden sich einen Rang teilen, wenn sie zusammentreffen, fuehrt
    /// `issues/260809-1631_o_ein-markensprung-kann-zwei-meldungen-zugleich-haben-und-die-zeile-traegt-eine.md`.
    // Dieselbe Zeile und derselbe Grund wie an `MarkenstelleGeaendert`, deren
    // einziger Erzeuger diese Funktion ist.
    #[allow(dead_code)]
    pub fn markenstelle(sprung: &Markensprung) -> Option<Self> {
        match sprung.fund {
            Fund::Getroffen | Fund::Verschoben => None,
            Fund::NichtGefunden => Some(Self::MarkenstelleGeaendert {
                zeile: sprung.zeile,
            }),
        }
    }

    /// Der Satz, der dem Nutzer gezeigt wird.
    ///
    /// Vollstaendig und ohne Auffangzweig: eine neue Variante haelt den Bau an
    /// und erzwingt ihren Satz, statt still einen fremden zu bekommen.
    pub fn text(&self) -> String {
        match self {
            Self::Abgewiesen(abweisung) => abweisung.meldung(),
            Self::MarkenstelleGeaendert { zeile } => {
                format!("die gemerkte Stelle hat sich geändert; die Marke führt auf Zeile {zeile}")
            }
            Self::Gesichert { pfad } => format!("{} gesichert", pfad.display()),
            Self::SichernGescheitert { grund } => grund.clone(),
        }
    }
}

/// Die Groesse, mit der die Flaeche entsteht, bevor die Aufteilung sie auslegt.
///
/// Die Breite ist die Anfangsbreite des Bereichs aus
/// [`crate::fenstermodell::Bereich::anfangsbreite`]; sie gilt nur bis zum
/// ersten Auslegen und ist danach ohne Bedeutung.
const AUFBAUGROESSE: NSSize = NSSize::new(460.0, 400.0);

/// Der Takt, in dem der Hauptfaden die Meldung des Arbeitsfadens abholt.
///
/// Dieselbe Zahl wie der Einzugstakt der Vorschau und des Dateifensters, aus
/// demselben Grund: haeufiger zu fragen braechte nichts, weil nicht oefter
/// gezeichnet wird.
const LADETAKT: NSTimeInterval = 1.0 / 60.0;

/// Das Zeichen, das einen ungesicherten Stand am Kopf anzeigt (C4).
///
/// **Vor dem Namen und nicht dahinter.** Der Kopf ist so breit wie der
/// Editorbereich, und der laesst sich bis auf 320 Punkte schmal ziehen; ein
/// langer Dateiname wird dann rechts gekuerzt, und ein Zeichen am Ende ginge
/// mit. Vorn steht es an einer festen Stelle und bleibt in jeder Breite
/// sichtbar.
const ABWEICHUNGSZEICHEN: &str = "•";

/// Um wie viele Punkte die Formatansicht ihre Grundschrift ueber die der
/// Rohansicht hebt (C3).
///
/// C3 verlangt fuer einfachen Text "eine lesbare Schriftgroesse" und der Plan
/// "eine gegenueber der Rohansicht lesbarere". Beides nennt keine Zahl, und
/// diese ist gewaehlt und nicht abgeleitet: zwei Punkte sind der kleinste
/// Schritt, den man nebeneinandergehalten sieht, und der groesste, der die Zahl
/// der Zeilen im Bild nicht spuerbar aendert.
///
/// **Code bekommt den Zuschlag nicht.** Quelltext wird in der Groesse gelesen,
/// in der er geschrieben wurde, und der sichtbare Unterschied zur Rohansicht ist
/// bei ihm die Einfaerbung und der Umbruch.
const LESEZUSCHLAG: f64 = 2.0;

/// Um welchen Faktor eine Markdown-Ueberschrift ihre Grundschrift ueberschreitet,
/// nach Stufen von 1 bis 6.
///
/// Absteigend, weil `#` mehr wiegt als `######`. Die Zahlen sind gewaehlt und
/// nicht abgeleitet; sie halten die sechste Stufe noch merklich ueber dem
/// Fliesstext, damit keine Ueberschrift aussieht wie keine.
const UEBERSCHRIFTSFAKTOREN: [f64; 6] = [1.7, 1.5, 1.3, 1.2, 1.1, 1.05];

/// Der Einzug einer Markdown-Listenzeile in Punkten (C3).
///
/// Er rueckt den ganzen Absatz ein, das Aufzaehlungszeichen eingeschlossen; das
/// Zeichen selbst bleibt stehen, wie der Datensatz vom 260808-0140 es verlangt.
const LISTENEINZUG: f64 = 20.0;

define_class!(
    /// Die Ansicht, in der Kopf und Textflaeche haengen — und die Stelle, an
    /// der KRK den Wechsel des Erscheinungsbildes bemerkt (S34).
    ///
    /// **Sie traegt genau eine Aufgabe ueber die einer `NSView` hinaus.**
    /// `viewDidChangeEffectiveAppearance` ist die eine Stelle, die AppKit fuer
    /// die Frage "hat das System auf Dunkel umgestellt" vorsieht, und sie ist
    /// eine Methode einer Ansicht. Der [`Editorbereich`] ist keine Ansicht,
    /// sondern ein `NSObject`, also braucht die Meldung eine Ansicht, die sie
    /// annimmt und weiterreicht.
    ///
    /// Die Rueckverbindung ist **schwach**, sonst schloesse sich der Ring
    /// Editorbereich → Ansicht → Rueckverweis → Editorbereich. Dieselbe Form
    /// wie der Rueckruf der Tableiste in [`super::vorschau`].
    // SAFETY:
    // - Die Oberklasse NSView stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = RefCell<Option<Weak<Editorbereich>>>]
    pub struct Editorsicht;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Editorsicht {}

    impl Editorsicht {
        /// Das System hat auf Hell oder Dunkel umgestellt (S34).
        // SAFETY: Die Signatur entspricht der von NSView.
        #[unsafe(method(viewDidChangeEffectiveAppearance))]
        fn erscheinung_gewechselt(&self) {
            // SAFETY: Die Oberklasse beantwortet dieselbe Nachricht ohne
            // Argument und ohne Rueckgabe. Sie zuerst, weil AppKit hinter
            // dieser Methode die Erscheinung der Unteransichten nachzieht und
            // KRK danach eine bereits umgestellte Flaeche vorfindet.
            let _: () = unsafe { msg_send![super(self), viewDidChangeEffectiveAppearance] };
            let editor = self.ivars().borrow().as_ref().and_then(Weak::load);
            if let Some(editor) = editor {
                editor.erscheinung_nachziehen();
            }
        }
    }
);

impl Editorsicht {
    /// Eine Ansicht mit dem genannten Rahmen, noch ohne Rueckverweis.
    fn neu(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(RefCell::new(None));
        // SAFETY: `initWithFrame:` von NSView hat die hier angenommene
        // Signatur.
        unsafe { msg_send![super(this), initWithFrame: rahmen] }
    }

    /// Traegt den Rueckverweis nach, sobald es den Editorbereich gibt.
    fn ziel_setzen(&self, editor: &Editorbereich) {
        *self.ivars().borrow_mut() = Some(Weak::from_retained(&editor.retain()));
    }
}

/// Die Senke, an die jeder [`Ladeausgang`] geht.
///
/// Ein eigener Name, weil der Typ an drei Stellen steht — Feld, Setzer und
/// Aufrufstelle — und ausgeschrieben an jeder von ihnen dieselbe Zeile waere.
pub type Ausgangsmelder = Box<dyn Fn(Ladeausgang)>;

/// Was der Editorbereich haelt.
pub struct EditorIvars {
    /// Die Ansicht, die in die Aufteilung gehaengt wird: Kopf und Bildlauf
    /// darin.
    ///
    /// Eine [`Editorsicht`] und keine blosse `NSView`, weil an ihr die eine
    /// Meldung haengt, mit der AppKit den Wechsel des Erscheinungsbildes
    /// anzeigt (S34).
    bereich: Retained<Editorsicht>,
    /// Der Kopf mit dem Dateinamen und dem Abweichungszeichen (C4).
    kopf: Retained<NSTextField>,
    /// Die Textflaeche selbst, editierbar und mit einem Textspeicher.
    ///
    /// Die Bildlaufansicht um sie herum steht hier **nicht**: sie haengt in
    /// [`Self::bereich`], der sie festhaelt, und niemand hier spricht sie an.
    /// Wer sie braucht — S33, um nach einem Ansichtswechsel die Nummernspalte
    /// neu zeichnen zu lassen —, bekommt sie ueber `enclosingScrollView`.
    text: Retained<NSTextView>,
    /// Der Stand des Editors, ohne AppKit.
    modell: RefCell<Editormodell>,
    /// Der Zeitgeber, der die Meldung des Arbeitsfadens abholt.
    ///
    /// Er haelt das Objekt als Ziel fest, und das Objekt haelt ihn; der Ring
    /// bricht mit `invalidate`, wie beim Einzugstakt der Vorschau.
    takt: RefCell<Option<Retained<NSTimer>>>,
    /// Die Senke, an die jeder [`Ladeausgang`] geht.
    ///
    /// Sie haelt den Anwendungsdelegierten **schwach**; die Begruendung steht
    /// an [`Editorbereich::melder_setzen`]. `None` heisst: der Aufbau ist noch
    /// nicht so weit, und dann gibt es auch niemanden, der etwas anfinge.
    melden: RefCell<Option<Ausgangsmelder>>,
    /// Das laufende Einfaerben, falls eines laeuft (C3).
    ///
    /// Hoechstens eines. Der Editor haelt hoechstens eine Datei und zeigt
    /// hoechstens eine Ansicht; ein zweiter Lauf daneben faerbte denselben Text
    /// ein zweites Mal ein. Fallengelassen wird der Vorgang beim Wechsel in die
    /// Rohansicht und beim Schliessen: sein Empfaenger faellt mit, und das
    /// `send` des ueberholten Fadens scheitert still.
    einfaerbung: RefCell<Option<Einfaerbungsvorgang>>,
    /// Ob der laufende Lauf ueberholt ist und nach seiner Rueckkehr sofort ein
    /// neuer zu starten ist (C3).
    ///
    /// **Das ist die ganze Zusammenfassung schneller Anfragen.** Wer tippt,
    /// stellt je Anschlag eine Anfrage; laeuft schon eine, wird nicht eine
    /// zweite gestartet, sondern diese Marke gesetzt. Damit lebt zu jedem
    /// Zeitpunkt hoechstens ein Faden, und der letzte Stand wird genau einmal
    /// eingefaerbt, statt jeder Zwischenstand einmal.
    ///
    /// Sie traegt beide Anlaesse: einen geaenderten Text und eine gewechselte
    /// Farbtafel. Beide verlangen dasselbe, naemlich einen neuen Lauf, und eine
    /// zweite Marke daneben unterschiede etwas, das dieselbe Antwort hat.
    einfaerbung_erneut: Cell<bool>,
    /// Welche der beiden Farbtafeln gerade gilt (S34).
    tafel: Cell<Tafel>,
}

define_class!(
    /// Der Editorbereich (C1 bis C6).
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = EditorIvars]
    pub struct Editorbereich;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Editorbereich {}

    // SAFETY: `NSTextDelegate` stellt keine Bedingungen. Die Textflaeche haelt
    // ihren Delegierten schwach ("This is a weak property",
    // `objc2-app-kit-0.3.2/src/generated/NSTextView.rs:1258-1263`), und der
    // Editorbereich haelt die Flaeche stark; ein Ring entsteht deshalb nicht,
    // und der Delegierte lebt so lange wie die Flaeche.
    unsafe impl NSTextDelegate for Editorbereich {
        /// Der Nutzer hat getippt, eingefuegt oder geloescht (C4).
        ///
        /// **Der Rueckweg aus der Flaeche ins Modell**, und die eine Stelle,
        /// die ihn geht; siehe den Modulkopf.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(textDidChange:))]
        fn text_geaendert(&self, _meldung: &NSNotification) {
            self.text_zurueckschreiben();
        }
    }

    // SAFETY: `NSTextViewDelegate` stellt keine Bedingungen. Er steht hier,
    // weil `NSTextView::setDelegate:` genau diesen Protokolltyp verlangt; die
    // eine benutzte Methode, `textDidChange:`, kommt aus dem Obertyp
    // `NSTextDelegate`.
    unsafe impl NSTextViewDelegate for Editorbereich {}

    impl Editorbereich {
        /// Der Rueckruf des Zeitgebers.
        // SAFETY: Die Signatur passt zu der, die NSTimer aufruft.
        #[unsafe(method(ladenEinziehen:))]
        fn laden_einziehen(&self, _zeitgeber: &NSTimer) {
            self.einziehen();
        }
    }
);

impl Editorbereich {
    /// Baut Kopf und Textflaeche mit einem Modell, das noch keine Datei haelt.
    pub fn bauen(mtm: MainThreadMarker) -> Retained<Self> {
        let bereich = Editorsicht::neu(mtm, NSRect::new(NSPoint::ZERO, AUFBAUGROESSE));
        bereich.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        // Der Bildlauf fuellt alles unter dem Kopf und waechst mit; der Kopf
        // klebt oben und waechst nur in der Breite. Dieselbe Aufteilung wie
        // Tableiste und Inhaltsflaeche in `super::vorschau`.
        let (rolle, text) = textflaeche_bauen(
            mtm,
            NSRect::new(
                NSPoint::ZERO,
                NSSize::new(
                    AUFBAUGROESSE.width,
                    AUFBAUGROESSE.height - statuszeile::HOEHE,
                ),
            ),
        );
        bereich.addSubview(&rolle);

        let kopf = kopf_bauen(mtm);
        kopf.setFrame(NSRect::new(
            NSPoint::new(
                statuszeile::EINZUG,
                AUFBAUGROESSE.height - statuszeile::HOEHE,
            ),
            NSSize::new(
                AUFBAUGROESSE.width - statuszeile::EINZUG,
                statuszeile::HOEHE,
            ),
        ));
        bereich.addSubview(&kopf);

        let tafel = tafel_der_erscheinung(&bereich);
        let this = Self::alloc(mtm).set_ivars(EditorIvars {
            bereich,
            kopf,
            text,
            modell: RefCell::new(Editormodell::neu()),
            takt: RefCell::new(None),
            melden: RefCell::new(None),
            einfaerbung: RefCell::new(None),
            einfaerbung_erneut: Cell::new(false),
            tafel: Cell::new(tafel),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };

        // Der Rueckweg aus der Flaeche ins Modell (C4). Er steht hier und nicht
        // in `textflaeche_bauen`, weil es das Objekt erst ab dieser Zeile gibt.
        this.ivars()
            .text
            .setDelegate(Some(ProtocolObject::from_ref(&*this)));
        // Derselbe Grund an der Ansicht: der Wechsel des Erscheinungsbildes
        // laeuft ueber sie hierher, und "hierher" gibt es erst ab dieser Zeile.
        this.ivars().bereich.ziel_setzen(&this);

        // Die Flaeche zeigt von der ersten Zeichnung an den Stand des Modells
        // und nicht irgendeinen. Beim Aufbau ist er leer, weil der Editor keine
        // Datei haelt; die Zeile steht trotzdem hier, damit es genau einen Weg
        // vom Modell in die Flaeche gibt und keinen Anfangszustand daneben. Der
        // Kopf und die Ansicht folgen derselben Regel.
        this.stand_einsetzen();
        this.kopf_nachziehen();
        this.darstellung_nachziehen();
        this
    }

    /// Die Ansicht, die in die Aufteilung gehaengt wird.
    ///
    /// Der ganze Bereich mit Kopf und Bildlauf, nicht die Bildlaufansicht
    /// allein: die Fokusabfrage aus S43 fragt nach dem Enthaltensein in dieser
    /// Ansicht, und die Textflaeche liegt darin.
    pub fn sicht(&self) -> &NSView {
        &self.ivars().bereich
    }

    /// Traegt die Senke ein, die jeden [`Ladeausgang`] bekommt.
    ///
    /// Gerufen vom Aufbau der Oberflaeche, mit einem Rueckruf, der den
    /// Anwendungsdelegierten **schwach** haelt: sonst schloesse sich der Ring
    /// Delegierter → Editorbereich → Rueckruf → Delegierter. Derselbe Zuschnitt
    /// wie `Hauptfenster::melder_setzen` und die uebrigen Melder dieses
    /// Projekts.
    ///
    /// **Warum der Ausgang ueberhaupt einen Rueckweg braucht.** Seit S24 liest
    /// der Editor auf einem Arbeitsfaden; wann eine Datei steht oder abgewiesen
    /// ist, weiss der Befehl, der sie angefordert hat, zu seiner eigenen Zeit
    /// nicht mehr. Der eine Ausgangstyp geht deshalb nicht mehr als Rueckgabe
    /// an den Aufrufer, sondern durch diese Senke — und zwar **jeder** Ausgang,
    /// auch der sofort feststehende [`Ladeausgang::SchonOffen`], damit es eine
    /// Behandlung gibt und nicht zwei.
    pub fn melder_setzen(&self, melden: Ausgangsmelder) {
        *self.ivars().melden.borrow_mut() = Some(melden);
    }

    /// Die Textflaeche, an der die Naemlichkeitsfrage des Fokusvorbehalts
    /// haengt.
    ///
    /// Sie geht allein zum Vergleichen nach aussen: der Anwendungsdelegierte
    /// haelt sie gegen den Ersthelfer des Fensters, so wie er es fuer die Liste
    /// der Leiste und die Inhaltsflaeche der Vorschau seit der Runde 1 tut.
    /// Wer sie zum Schreiben braucht, geht ueber das Modell und
    /// [`Self::stand_einsetzen`].
    pub fn textflaeche(&self) -> &NSTextView {
        &self.ivars().text
    }

    /// Ob der Editor eine Datei haelt (C1, C2).
    ///
    /// Der Fokusbefehl aus C1 fragt danach: einen ausgeblendeten Editor ohne
    /// Datei holt er nicht hervor. Die Frage geht an das Modell und wird hier
    /// nicht aus der Textflaeche beantwortet — ein leerer Text ist keine
    /// fehlende Datei.
    pub fn haelt_datei(&self) -> bool {
        self.ivars().modell.borrow().haelt_datei()
    }

    /// Ob der Editor Aenderungen haelt, die nicht in der Datei stehen (C4).
    ///
    /// Die Frage der vier Anlaesse aus C4: der Anwendungsdelegierte stellt sie,
    /// bevor er einen Anlass ausfuehrt, der den Stand verloere. Sie geht an das
    /// Modell und wird hier nicht aus der Textflaeche beantwortet.
    pub fn hat_ungesicherten_stand(&self) -> bool {
        self.ivars().modell.borrow().hat_ungesicherten_stand()
    }

    /// Die Datei, die der Editor haelt, falls er eine haelt (C11).
    ///
    /// Der Fenstertitel fragt danach: steht der Fokus im Editor, zeigt der
    /// Titel den vollen Pfad dieser Datei, auch dann, wenn das aktive
    /// Dateifenster einen anderen Ordner zeigt. Die Frage geht wie
    /// [`Self::haelt_datei`] an das Modell und wird hier nicht ein zweites Mal
    /// beantwortet.
    ///
    /// Der Pfad wird abgeschrieben und nicht ausgeliehen: die Ausleihe des
    /// Modells endet mit dieser Zeile, und der Aufrufer traegt den Wert durch
    /// AppKit-Aufrufe, die hierher zuruecklaufen koennen.
    pub fn pfad(&self) -> Option<PathBuf> {
        self.ivars().modell.borrow().pfad().map(Path::to_path_buf)
    }

    /// Nimmt die genannte Datei auf und zeigt ihren Stand (C2).
    ///
    /// **Der eine Weg, auf dem eine Datei in den Editor kommt.** Beide
    /// Einstiege aus C2 gehen ueber ihn und legen damit dieselbe Pruefung an,
    /// wie es das neunte Abnahmekriterium von C2 verlangt; der Sprung auf eine
    /// Textmarke aus C6 kommt spaeter dazu.
    ///
    /// **Sie kehrt sofort zurueck und nennt keinen Ausgang.** Gelesen und
    /// geprueft wird auf dem Arbeitsfaden des Modells, und die Antwort holt
    /// [`Self::einziehen`] ab; wer wissen will, wie es ausgegangen ist, haengt
    /// sich ueber [`Self::melder_setzen`] ein. Steht der Ausgang schon fest,
    /// weil der Editor die Datei bereits haelt, geht er durch dieselbe Senke,
    /// nur eben sofort.
    ///
    /// **Was der Nutzer davon sieht:** F4 auf eine grosse Datei blendet den
    /// Editor nicht sogleich ein, sondern erst, wenn sie gelesen ist. Das ist
    /// die Reihenfolge, die das elfte Abnahmekriterium von C2 verlangt — erst
    /// die Pruefung, dann die Flaeche —, und sie bleibt mit dem Arbeitsfaden
    /// erhalten, weil auch die Pruefung dort laeuft. Der Gegenwert steht in
    /// S24: waehrend des Lesens bleiben die beiden Dateifenster bedienbar.
    ///
    /// Entschieden wird nichts hier: die Pruefung steht in
    /// `krk_core::text::datei::oeffnen` und ist ueber [`Editormodell::oeffnen`]
    /// erreichbar. Bei [`Ladeausgang::Abgewiesen`] bleibt der bisherige Stand
    /// vollstaendig stehen, und der Grund geht als Wert nach oben; wohin er
    /// dort kommt, weiss diese Datei nicht (siehe den Modulkopf).
    pub fn datei_oeffnen(&self, pfad: &Path) {
        let sofort = self.ivars().modell.borrow_mut().oeffnen(pfad);
        match sofort {
            Some(ausgang) => self.melden(ausgang),
            None => self.takt_starten(),
        }
    }

    /// Schreibt den gehaltenen Stand in die Datei (C4).
    ///
    /// **Geschrieben wird im Modell und hier nicht ein zweites Mal.** Diese
    /// Funktion reicht den Befehl hinein und den Ausgang heraus; die
    /// Sicherungsform, die Stempelpruefung und der atomare Schreibweg stehen in
    /// [`Editormodell::sichern`] und darunter in `krk_core::text::datei`.
    ///
    /// **Was sie beitraegt, ist der Kopf.** Nach einem gelungenen Sichern
    /// meldet das Modell keine Abweichung mehr, und ohne diesen Ruf truege der
    /// Kopf sein Zeichen weiter, obwohl nichts mehr abweicht. Nach einem
    /// gescheiterten bleibt der Kopf, wie er ist, weil auch die Abweichung
    /// bleibt.
    ///
    /// **Der Stand kommt nicht aus der Textflaeche.** Er steht im Modell, weil
    /// `textDidChange:` ihn bei jeder Aenderung dorthin zurueckschreibt (siehe
    /// den Modulkopf). Ihn hier ein zweites Mal aus der Flaeche zu holen waere
    /// der zweite Rueckweg, und der eine bestehende waere damit nicht mehr die
    /// Wahrheit ueber den Stand des Editors.
    ///
    /// Die Ausleihe des Modells endet vor dem Ruf an den Kopf, wie ueberall in
    /// dieser Datei.
    pub fn sichern(&self) -> Sicherungsausgang {
        let ausgang = self.ivars().modell.borrow_mut().sichern();
        if matches!(ausgang, Sicherungsausgang::Gesichert(_)) {
            self.kopf_nachziehen();
        }
        ausgang
    }

    /// Nimmt die zurueckgehaltene Datei jetzt auf (C4).
    ///
    /// Der Weg zurueck aus der Nachfrage, wenn der Nutzer mit "sichern" oder
    /// "verwerfen" geantwortet hat. Was danach zu tun ist, ist genau das, was
    /// [`Self::einziehen`] fuer [`Ladeausgang::Geoeffnet`] tut, und deshalb
    /// steht es hier in derselben Form: Stand in die Flaeche, Kopf nachziehen,
    /// Ausgang durch dieselbe Senke. Eine zweite Behandlung desselben Wertes
    /// entsteht damit nicht — der Anwendungsdelegierte sieht `Geoeffnet` und
    /// holt Fokus und Titel nach, ohne diesen Weg vom gewoehnlichen zu
    /// unterscheiden.
    ///
    /// Wartete nichts, geschieht nichts. Der Fall ist im Ablauf nicht
    /// erreichbar, weil allein der Rueckruf der Nachfrage hierher fuehrt;
    /// stillschweigend nichts zu tun ist trotzdem richtig, denn es gibt keine
    /// Datei, ueber die etwas zu melden waere.
    pub fn zurueckgehaltenes_uebernehmen(&self) {
        let ausgang = self
            .ivars()
            .modell
            .borrow_mut()
            .zurueckgehaltenes_uebernehmen();
        let Some(ausgang) = ausgang else {
            return;
        };
        if ausgang == Ladeausgang::Geoeffnet {
            self.stand_einsetzen();
            self.kopf_nachziehen();
            self.darstellung_nachziehen();
        }
        self.melden(ausgang);
    }

    /// Laesst die zurueckgehaltene Datei fallen (C4).
    ///
    /// Der Weg zurueck aus der Nachfrage, wenn der Nutzer abgebrochen hat oder
    /// das Sichern gescheitert ist. Die Flaeche wird dabei nicht angefasst: sie
    /// traegt unveraendert den Stand, den der Nutzer behalten wollte.
    pub fn zurueckgehaltenes_fallenlassen(&self) {
        self.ivars()
            .modell
            .borrow_mut()
            .zurueckgehaltenes_fallenlassen();
    }

    /// Gibt die gehaltene Datei auf und leert die Flaeche (C1, C4).
    ///
    /// Gerufen, wenn der Editor geschlossen wird — nach der Nachfrage aus C4,
    /// die dem Anwendungsdelegierten gehoert. Der Stand faellt im Modell, und
    /// die beiden Anzeigen ziehen ueber dieselben zwei Stellen nach wie nach
    /// jedem anderen Wechsel des Gehaltenen.
    pub fn schliessen(&self) {
        self.ivars().modell.borrow_mut().schliessen();
        self.stand_einsetzen();
        self.kopf_nachziehen();
        // Ohne Datei gibt es keine Sprache und nichts einzufaerben; der Ruf
        // raeumt die gesetzten Merkmale ab und laesst einen laufenden
        // Einfaerbungsfaden fallen.
        self.darstellung_nachziehen();
    }

    /// Holt die Meldung des Arbeitsfadens ab (C2).
    ///
    /// **Der Vergleich nennt [`Ladeausgang::Geoeffnet`] namentlich und darf
    /// nicht auf "nicht abgewiesen" gelockert werden.** Das ist die Haelfte der
    /// Behebung vom 260809, die in dieser Datei steht: bei
    /// [`Ladeausgang::SchonOffen`] hat das Modell nicht gelesen, und die Flaeche
    /// traegt das, was der Nutzer getippt und noch nicht gesichert hat; ein Ruf
    /// von [`Self::stand_einsetzen`] schriebe den Plattenstand darueber, und
    /// genau so ging die Aenderung des Nutzers verloren
    /// (`issues/260809-2029_*_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`).
    /// Dass jener Ausgang seit S24 gar nicht mehr hier ankommt, weil das Modell
    /// ihn entscheidet, bevor ein Faden startet, macht die Namensnennung nicht
    /// ueberfluessig: sie ist die Stelle, an der ein spaeter dazukommender
    /// Ausgang auffaellt, statt still mitzulaufen.
    ///
    /// **Der Takt endet, sobald nichts mehr laeuft**, und zwar auf beiden
    /// Wegen: nach einer eingetroffenen Meldung und nach einem Faden, der ohne
    /// Meldung gefallen ist. Der zweite Fall hinterlaesst allein die Zeile auf
    /// der Standardfehlerausgabe, die `Ladevorgang::starten` schreibt; dasselbe
    /// gilt seit der Runde 1 fuer die Vorschau.
    ///
    /// **Ein Takt fuer zwei Arbeitsfaeden.** Seit S33 laeuft neben dem Lesen das
    /// Einfaerben auf einem eigenen Faden, und beide werden hier abgeholt. Ein
    /// zweiter Zeitgeber daneben fragte im selben Sechzigstel dieselbe
    /// Laufschleife ein zweites Mal; er braechte kein Bild frueher, weil in
    /// einem Bild nur einmal gezeichnet wird.
    fn einziehen(&self) {
        self.ladeausgang_einziehen();
        self.einfaerbung_einziehen();
        if !self.ivars().modell.borrow().laedt_noch() && self.ivars().einfaerbung.borrow().is_none()
        {
            self.takt_beenden();
        }
    }

    /// Holt die Meldung des Lesefadens ab (C2).
    fn ladeausgang_einziehen(&self) {
        let eingetroffen = self.ivars().modell.borrow_mut().einziehen();
        let Some(ausgang) = eingetroffen else {
            return;
        };
        if ausgang == Ladeausgang::Geoeffnet {
            self.stand_einsetzen();
            self.kopf_nachziehen();
            // Die neue Datei kann eine andere Besetzung der Formatansicht
            // verlangen als die vorige: Schrift, Umbruch und Einfaerbung
            // haengen am Dateityp und an der Sprache, die die Kiste kennt.
            self.darstellung_nachziehen();
        }
        self.melden(ausgang);
    }

    /// Gibt den Ausgang an die Senke weiter, falls jemand zuhoert.
    ///
    /// Die Ausleihe steht waehrend des Rufs, wie bei `Hauptfenster::melden`.
    /// Sie ist lesend, und der einzige schreibende Zugriff auf dieselbe Zelle
    /// ist [`Self::melder_setzen`] beim Aufbau; ein Ruf, der ueber AppKit
    /// hierher zuruecklaeuft, nimmt eine zweite Leseausleihe und keine
    /// schreibende.
    fn melden(&self, ausgang: Ladeausgang) {
        let melden = self.ivars().melden.borrow();
        if let Some(melden) = melden.as_ref() {
            melden(ausgang);
        }
    }

    /// Haengt den Zeitgeber in die Laufschleife, falls er noch nicht laeuft.
    fn takt_starten(&self) {
        if self.ivars().takt.borrow().is_some() {
            return;
        }
        // SAFETY: `self` ist das Ziel und beantwortet `ladenEinziehen:` mit der
        // erwarteten Signatur. Der Zeitgeber wird unten in die Laufschleife
        // gehaengt; `NSRunLoopCommonModes` ist ein Fremdsymbol von Foundation.
        // Dieselbe Form wie der Einzugstakt der Vorschau.
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

    /// Schreibt zurueck, was der Nutzer in die Flaeche getippt hat (C4).
    ///
    /// **Der Rueckweg**, und die eine Stelle, an der der Stand der Flaeche zum
    /// Stand des Modells wird. Er nimmt den ganzen Text und nicht die geaenderte
    /// Stelle; der Grund und der Preis stehen an [`Editormodell::bearbeiten`],
    /// das ihn dabei durch `krk_core::text::datei::in_gehaltene_form` fuehrt.
    /// Eine `NSTextView` bewahrt eingefuegten Text zeichengetreu auf, also
    /// kommt ein `\r\n` aus einer Windows-Quelle hier an und darf nicht weiter.
    ///
    /// **Der Kopf wird nur beim Uebergang nachgezogen.** Die Abweichungsmarke
    /// geht von falsch nach wahr und bleibt dort bis zum naechsten Sichern; sie
    /// bei jedem Anschlag neu in ein `NSTextField` zu schreiben hiesse, je
    /// Tastendruck ein Auslegen anzustossen, das nichts aendert.
    ///
    /// Die Ausleihe des Modells endet vor dem Ruf an den Kopf, wie ueberall in
    /// dieser Datei.
    fn text_zurueckschreiben(&self) {
        let stand = self.ivars().text.string().to_string();
        let war_abweichend = {
            let mut modell = self.ivars().modell.borrow_mut();
            let vorher = modell.hat_ungesicherten_stand();
            modell.bearbeiten(stand);
            vorher
        };
        if !war_abweichend {
            self.kopf_nachziehen();
        }
        // Die Einfaerbung gehoert zu dem Stand, aus dem sie gebildet wurde. Wer
        // ein Anfuehrungszeichen tippt, macht aus dem Rest der Datei eine
        // Zeichenkette, und ohne diesen Ruf bliebe die alte Farbe stehen. Die
        // Anfrage kostet nichts, solange schon eine laeuft; siehe
        // [`Self::einfaerbung_anfordern`].
        self.einfaerbung_anfordern();
    }

    /// Schreibt Dateiname und Abweichungszeichen in den Kopf (C4).
    ///
    /// **Die eine Stelle, die den Kopf beschreibt.** Sie wird gerufen, wo sich
    /// eine der beiden Angaben aendern kann: beim Aufbau, nach einem gelungenen
    /// Oeffnen, beim Uebergang in den ungesicherten Stand, nach einem
    /// gelungenen Sichern und seit S28 nach dem Schliessen.
    ///
    /// Was dort steht, entscheidet [`kopfzeile`] ohne AppKit und ist deshalb
    /// ohne Fenster pruefbar.
    fn kopf_nachziehen(&self) {
        let zeile = {
            let modell = self.ivars().modell.borrow();
            kopfzeile(modell.pfad(), modell.hat_ungesicherten_stand())
        };
        self.ivars()
            .kopf
            .setStringValue(&NSString::from_str(&zeile));
    }

    /// Schreibt den gehaltenen Stand in die Textflaeche.
    ///
    /// **Die eine Stelle, die den Text der Flaeche ersetzt.** Ein
    /// Ansichtswechsel geht nicht ueber sie, sondern ueber die
    /// voruebergehenden Merkmale des Layoutverwalters; deshalb kann er nichts
    /// verlieren.
    ///
    /// Die Ausleihe des Modells endet, bevor der Aufruf in das Textsystem
    /// faellt. `NSString::from_str` steht noch darin, und das ist kein Bruch
    /// der Regel: es kopiert Bytes und ruft nichts zurueck, waehrend
    /// `setString:` den Layoutverwalter, die Delegierten und damit
    /// moeglicherweise KRK selbst erreicht. Den Stand stattdessen zu klonen
    /// hiesse, eine Datei von 16 MB zweimal zu kopieren statt einmal.
    ///
    /// **Hier fehlt noch das Leeren des Rueckgaengigstapels.** `setString:`
    /// schreibt an der Rueckgaengigverwaltung vorbei und laesst einen bereits
    /// gefuellten Stapel stehen, der auf den Text der vorigen Datei zeigt;
    /// seit `textflaeche_bauen` `allowsUndo` einschaltet, kann so ein Stapel
    /// entstehen. **Der Fall ist erreichbar**, seit ein Dateiwechsel im Editor
    /// steht; der offene Defekt dazu ist
    /// `issues/260809-1727_o_ein-dateiwechsel-laesst-den-rueckgaengigstapel-der-vorigen-datei-stehen.md`.
    /// Bis S26 stand hier, der Fall sei unerreichbar, weil der einzige Aufrufer
    /// [`Self::bauen`] sei; das war schon seit S22 nicht mehr wahr, denn das
    /// Oeffnen ruft ebenfalls hierher.
    fn stand_einsetzen(&self) {
        let stand = {
            let modell = self.ivars().modell.borrow();
            NSString::from_str(modell.stand())
        };
        self.ivars().text.setString(&stand);
    }

    // ------------------------------------------------------------------
    // Die beiden Ansichten (C3)
    // ------------------------------------------------------------------

    /// Wechselt zwischen Rohansicht und Formatansicht (C3).
    ///
    /// **Der ganze Wechsel ist ein Ruf ins Modell und ein Nachziehen der
    /// Darstellung.** Der Textspeicher wird dabei nicht angefasst, und deshalb
    /// kann der Wechsel nichts verlieren: es gibt keinen zweiten Textbestand, in
    /// den etwas verlorengehen koennte, und
    /// [`Editormodell::ansicht_umschalten`] fasst weder den Stand noch die
    /// Abweichungsmarke an. Das zehnte Abnahmekriterium von C3 ist damit eine
    /// Eigenschaft der Bauart und keine Zusage der Sorgfalt.
    ///
    /// **Die Schreibmarke bleibt, wo sie ist**, und zwar ohne eigenen Bau: sie
    /// haengt an Zeichenstellen des Textspeichers, und der bleibt Zeichen fuer
    /// Zeichen derselbe. Das elfte Abnahmekriterium von C3 faellt daraus an.
    pub fn ansicht_umschalten(&self) {
        self.ivars().modell.borrow_mut().ansicht_umschalten();
        self.darstellung_nachziehen();
    }

    /// Setzt Grundschrift, Umbruch und Merkmale auf die gewaehlte Ansicht (C3).
    ///
    /// **Die eine Stelle, an der die Darstellung entsteht**, und sie kennt vier
    /// Aufrufer: den Aufbau, ein gelungenes Oeffnen, das Schliessen und den
    /// Ansichtswechsel. Alle vier stellen dieselbe Frage — welche Ansicht,
    /// welche Datei —, und eine zweite Stelle daneben waere die erste
    /// Gelegenheit, sie verschieden zu beantworten.
    ///
    /// Die drei Sachen, die sich aendern, stehen in `### Frage 7` des Plans: die
    /// Schrift, der Umbruch und die Merkmale. Sie werden hier in dieser
    /// Reihenfolge gesetzt, und die Reihenfolge zaehlt: `setFont:` und
    /// `setTextColor:` einer `NSTextView` schreiben ueber den **ganzen**
    /// Textspeicher, also muessen sie vor den Auszeichnungen stehen, die
    /// einzelne Stellen davon ueberschreiben.
    ///
    /// **Die Einfaerbung kommt nicht von hier, sondern spaeter.** Sie laeuft auf
    /// einem Arbeitsfaden (0,3 MB/s, gemessen; siehe `crate::hervorhebung`), und
    /// diese Funktion fordert sie nur an. Bis sie eintrifft, steht der Text in
    /// der Grundfarbe da — dieselbe Spanne, die schon beim Lesen einer grossen
    /// Datei vergeht, und aus demselben Grund.
    fn darstellung_nachziehen(&self) {
        let (ansicht, art) = {
            let modell = self.ivars().modell.borrow();
            (
                modell.ansicht(),
                crate::hervorhebung::art(modell.pfad(), modell.typ()),
            )
        };

        self.grundschrift_setzen(ansicht, art);
        self.umbruch_setzen(ansicht == Ansicht::Format);
        self.merkmale_zuruecksetzen();

        match ansicht {
            Ansicht::Format => self.einfaerbung_anfordern(),
            // Die Rohansicht zeigt die Zeichen ohne Einfaerbung; ein laufender
            // Faden hat nichts mehr abzuliefern und faellt mit seinem
            // Empfaenger.
            Ansicht::Roh => {
                *self.ivars().einfaerbung.borrow_mut() = None;
                self.ivars().einfaerbung_erneut.set(false);
            }
        }

        self.nummernspalte_nachziehen();
    }

    /// Setzt die Grundschrift der Flaeche und damit auch die des naechsten
    /// Anschlags (C3).
    ///
    /// **Eine Regel und keine drei.** Fest geschrieben wird, was Zeichen fuer
    /// Zeichen gelesen wird: die Rohansicht immer, und die Formatansicht bei
    /// Code. Alles Uebrige — einfacher Text und Markdown — bekommt die
    /// Systemschrift mit dem [`LESEZUSCHLAG`]. Das ist die "lesbare
    /// Schriftgroesse", die C3 fuer einfachen Text zusagt, und zugleich die
    /// Grundschrift, ueber der die Markdown-Ueberschriften ihre Stufen haben.
    ///
    /// `setFont:` schreibt ueber den ganzen Textspeicher **und** setzt die
    /// Merkmale des naechsten Anschlags. Beides ist gewollt: ohne das zweite
    /// truege ein neu getipptes Zeichen die Schrift der vorigen Ansicht.
    fn grundschrift_setzen(&self, ansicht: Ansicht, art: Darstellungsart) {
        let (fest, groesse) = match (ansicht, art) {
            (Ansicht::Roh, _) | (Ansicht::Format, Darstellungsart::Code) => {
                (true, NSFont::systemFontSize())
            }
            (Ansicht::Format, Darstellungsart::EinfacherText | Darstellungsart::Markdown) => {
                (false, NSFont::systemFontSize() + LESEZUSCHLAG)
            }
        };
        let schrift = if fest {
            feste_schrift(groesse)
        } else {
            NSFont::systemFontOfSize(groesse)
        };
        self.ivars().text.setFont(Some(&schrift));
        // Die Systemfarbe und nicht die der Tafel: sie loest sich in Hell wie in
        // Dunkel gegen den Grund der Flaeche auf, und der Grund bleibt nach S34
        // die Systemfarbe. Aus der Tafel kommen allein die Vordergrundfarben
        // der Wortarten, und die setzt die Einfaerbung darueber.
        self.ivars().text.setTextColor(Some(&NSColor::textColor()));
    }

    /// Schaltet den Umbruch am Fensterrand ein oder aus (C3).
    ///
    /// Die Rohansicht zeigt die Zeichen der Datei, also auch ihre Zeilenlaengen:
    /// ohne Umbruch und mit einem waagerechten Schieber. Die Formatansicht
    /// bricht am Fensterrand um, wie C3 es fuer einfachen Text ausdruecklich
    /// zusagt und wie es fuer die beiden anderen Besetzungen ebenso gilt.
    ///
    /// **Der Rahmen der Flaeche wird beim Einschalten zurueckgesetzt.** In der
    /// Rohansicht waechst sie mit der laengsten Zeile; bliebe sie so breit,
    /// laege der Umbruchrand ausserhalb des Sichtbaren, und der Umbruch griffe
    /// erst beim naechsten Auslegen aus einem anderen Anlass.
    fn umbruch_setzen(&self, umbruch: bool) {
        let text = &self.ivars().text;
        let Some(rolle) = text.enclosingScrollView() else {
            return;
        };
        let breite = rolle.contentSize().width;
        rolle.setHasHorizontalScroller(!umbruch);
        text.setHorizontallyResizable(!umbruch);
        // SAFETY: Der Behaelter wird von der Flaeche selbst mitgebracht und hier
        // nur eingestellt; kein fremdes Objekt wird gehalten.
        if let Some(behaelter) = unsafe { text.textContainer() } {
            behaelter.setWidthTracksTextView(umbruch);
            behaelter.setContainerSize(if umbruch {
                NSSize::new(breite, f64::MAX)
            } else {
                NSSize::new(f64::MAX, f64::MAX)
            });
        }
        if umbruch {
            let hoehe = text.frame().size.height;
            text.setFrameSize(NSSize::new(breite, hoehe));
        }
    }

    /// Nimmt jede gesetzte Auszeichnung wieder heraus.
    ///
    /// **Beide Listen**, denn beide werden gesetzt: die voruebergehenden
    /// Merkmale im Layoutverwalter und der Absatzeinzug im Textspeicher. Schrift
    /// und Farbe brauchen hier nichts, weil `setFont:` und `setTextColor:` in
    /// [`Self::grundschrift_setzen`] den ganzen Speicher ueberschreiben; der
    /// Einzug ist das einzige gesetzte Merkmal, das keines von beiden erreicht.
    fn merkmale_zuruecksetzen(&self) {
        let text = &self.ivars().text;
        // SAFETY: Speicher und Verwalter bringt die Flaeche selbst mit und wird
        // hier nur beschrieben; die Bereiche decken genau den vorhandenen Text.
        unsafe {
            if let Some(speicher) = text.textStorage() {
                let ganz = NSRange::new(0, speicher.length());
                speicher.removeAttribute_range(NSParagraphStyleAttributeName, ganz);
                if let Some(verwalter) = text.layoutManager() {
                    let leer: Retained<NSDictionary<NSString, AnyObject>> = NSDictionary::new();
                    verwalter.setTemporaryAttributes_forCharacterRange(&leer, ganz);
                }
            }
        }
    }

    /// Fordert eine Einfaerbung des gehaltenen Standes an (C3).
    ///
    /// **Hoechstens ein Faden zur Zeit.** Laeuft schon einer, wird kein zweiter
    /// gestartet, sondern nur vermerkt, dass sein Ergebnis ueberholt sein wird;
    /// er wird dann nach seiner Rueckkehr sofort wiederholt. Damit kostet ein
    /// Tastendruck waehrend eines laufenden Laufs nichts, und der Nutzer bekommt
    /// die Einfaerbung des letzten Standes statt der jedes Zwischenstandes.
    ///
    /// **Die Rohansicht fordert nie an**, und die Abfrage steht hier und nicht
    /// bei den drei Aufrufern. Sie zeigt die Zeichen der Datei ohne
    /// Einfaerbung; eine Anfrage von dort brachte eine Lieferung zurueck, und
    /// [`Self::formatierung_anwenden`] faerbte die Rohansicht ein. Der Weg ist
    /// erreichbar, seit [`Self::text_zurueckschreiben`] bei jedem Anschlag
    /// anfordert — dort wird nicht nach der Ansicht gefragt, sondern gemeldet,
    /// dass sich der Text geaendert hat.
    ///
    /// Ohne gehaltene Datei geschieht ebenso nichts: es gibt keinen Pfad, an dem
    /// die Kiste eine Sprache erkennen koennte, und nichts einzufaerben.
    fn einfaerbung_anfordern(&self) {
        if self.ivars().einfaerbung.borrow().is_some() {
            self.ivars().einfaerbung_erneut.set(true);
            return;
        }
        let (stand, pfad, typ) = {
            let modell = self.ivars().modell.borrow();
            if !modell.haelt_datei() || modell.ansicht() != Ansicht::Format {
                return;
            }
            (
                modell.stand().to_owned(),
                modell.pfad().map(Path::to_path_buf),
                modell.typ(),
            )
        };
        let vorgang = Einfaerbungsvorgang::starten(stand, pfad, typ, self.ivars().tafel.get());
        *self.ivars().einfaerbung.borrow_mut() = Some(vorgang);
        self.ivars().einfaerbung_erneut.set(false);
        self.takt_starten();
    }

    /// Holt die Meldung des Einfaerbungsfadens ab (C3).
    ///
    /// **Ein ueberholtes Ergebnis wird nicht angewendet, sondern fallengelassen
    /// und sofort neu angefordert.** Es waere nicht nur veraltet: seine Bereiche
    /// zeigten in einen Text, der inzwischen kuerzer sein kann, und ein
    /// `NSRange` hinter dem Text beantwortet AppKit mit einer
    /// Objective-C-Ausnahme. Die ist in Rust nicht zu fangen und beendet das
    /// Programm.
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
            // Sinn mehr. Derselbe Zweig und derselbe Grund wie beim Lesevorgang.
            Abholung::Weggefallen => {
                *self.ivars().einfaerbung.borrow_mut() = None;
                self.ivars().einfaerbung_erneut.set(false);
            }
            Abholung::Fertig(formatierung) => {
                *self.ivars().einfaerbung.borrow_mut() = None;
                if self.ivars().einfaerbung_erneut.replace(false) {
                    self.einfaerbung_anfordern();
                } else {
                    self.formatierung_anwenden(&formatierung);
                }
            }
        }
    }

    /// Traegt eine fertige Formatierung in die Flaeche (C3).
    ///
    /// **Zwei Listen und zwei Orte**, und der Grund steht im Modulkopf von
    /// `crate::hervorhebung`: der Layoutverwalter beachtet als voruebergehendes
    /// Merkmal allein, was die Auslegung nicht aendert. Farbe und
    /// Unterstreichung gehen deshalb dorthin, Schriftgroesse, Schriftschnitt,
    /// feste Schrift und Einzug in den Textspeicher. In die **Datei** geraet
    /// weder das eine noch das andere: gesichert wird
    /// [`Editormodell::stand`], und der kommt aus den Zeichen der Flaeche und
    /// nicht aus ihren Merkmalen.
    ///
    /// **Der Guertel vorweg.** Stimmt die Laenge nicht mehr, gehoert die
    /// Lieferung zu einem anderen Stand, und jeder Bereich dahinter waere ein
    /// Programmabbruch statt eines falschen Bildes. Erreichbar ist der Fall
    /// nicht, weil ein ueberholtes Ergebnis schon in
    /// [`Self::einfaerbung_einziehen`] fallengelassen wird; er steht hier, weil
    /// der Preis eines Irrtums an dieser Stelle das Programm ist.
    fn formatierung_anwenden(&self, formatierung: &Formatierung) {
        let text = &self.ivars().text;
        // SAFETY: Speicher und Verwalter bringt die Flaeche selbst mit.
        let (speicher, verwalter) = unsafe { (text.textStorage(), text.layoutManager()) };
        let (Some(speicher), Some(verwalter)) = (speicher, verwalter) else {
            return;
        };
        if speicher.length() != formatierung.laenge {
            return;
        }
        let ganz = NSRange::new(0, formatierung.laenge);

        // Die Merkmale des Textspeichers: was auf die Auslegung wirkt.
        let grundgroesse = NSFont::systemFontSize() + LESEZUSCHLAG;
        speicher.beginEditing();
        for stelle in &formatierung.auszeichnungen {
            let bereich = NSRange::new(stelle.anfang, stelle.laenge);
            let merkmale = match stelle.art {
                Auszeichnung::Ueberschrift { stufe } => {
                    let faktor = UEBERSCHRIFTSFAKTOREN[usize::from(stufe.clamp(1, 6)) - 1];
                    schriftmerkmal(&NSFont::boldSystemFontOfSize(grundgroesse * faktor))
                }
                Auszeichnung::FesteSchrift => schriftmerkmal(&feste_schrift(grundgroesse)),
                Auszeichnung::Listenzeile => einzugsmerkmal(),
            };
            // SAFETY: Der Bereich liegt im Text; die Laenge ist oben geprueft,
            // und die Stellen der Formatierung sind aufsteigend und
            // ueberschneidungsfrei.
            unsafe { speicher.addAttributes_range(&merkmale, bereich) };
        }
        speicher.endEditing();

        // Die voruebergehenden Merkmale: was die Auslegung nicht anfasst.
        let strich = NSNumber::numberWithInteger(NSUnderlineStyle::Single.0);
        let mut farben: HashMap<Farbe, Retained<NSColor>> = HashMap::new();
        // SAFETY: Dieselbe Pruefung deckt beide Schleifen; der Verwalter gehoert
        // dieser Flaeche.
        unsafe {
            verwalter.setTemporaryAttributes_forCharacterRange(&NSDictionary::new(), ganz);
            for stueck in &formatierung.einfaerbungen {
                let bereich = NSRange::new(stueck.anfang, stueck.laenge);
                let farbe = farben
                    .entry(stueck.farbe)
                    .or_insert_with(|| nsfarbe(stueck.farbe));
                verwalter.addTemporaryAttribute_value_forCharacterRange(
                    NSForegroundColorAttributeName,
                    farbe,
                    bereich,
                );
                if stueck.unterstrichen {
                    verwalter.addTemporaryAttribute_value_forCharacterRange(
                        NSUnderlineStyleAttributeName,
                        &strich,
                        bereich,
                    );
                }
            }
        }

        // Die Auszeichnungen haben die Zeilenkaesten geaendert; die Nummern
        // stehen sonst neben dem zuletzt gezeichneten Umbruch.
        self.nummernspalte_nachziehen();
    }

    /// Zieht die Farbtafel auf das gewechselte Erscheinungsbild nach (S34).
    ///
    /// Gerufen von [`Editorsicht`], der einen Stelle, an der AppKit den Wechsel
    /// meldet. Hat sich die Tafel nicht geaendert, geschieht nichts: die Meldung
    /// kommt auch bei Wechseln, die Hell und Dunkel nicht betreffen, und ein
    /// Einfaerbungslauf ueber eine Datei von 16 MB ist kein Preis fuer nichts.
    ///
    /// Ob ueberhaupt einzufaerben ist, fragt
    /// [`Self::einfaerbung_anfordern`] und nicht diese Stelle; die Antwort
    /// steht dort einmal.
    fn erscheinung_nachziehen(&self) {
        let neue = tafel_der_erscheinung(&self.ivars().bereich);
        if neue == self.ivars().tafel.get() {
            return;
        }
        self.ivars().tafel.set(neue);
        self.einfaerbung_anfordern();
    }

    /// Laesst die Nummernspalte neu zeichnen.
    ///
    /// Umbruch und Schrift aendern die Zeilenkaesten des Layoutverwalters, ohne
    /// dass der Textspeicher eine Meldung verschickt, an der die Spalte es
    /// bemerken koennte; ohne diesen Ruf zeigte die Formatansicht die Nummern
    /// des zuletzt gezeichneten Umbruchs, und das fuenfte Abnahmekriterium von
    /// C10 waere gebrochen. Der Vermerk stammt aus S46.
    fn nummernspalte_nachziehen(&self) {
        if let Some(rolle) = self.ivars().text.enclosingScrollView() {
            nummernspalte::spalte_neu_zeichnen(&rolle);
        }
    }
}

/// Welche Farbtafel zum wirksamen Erscheinungsbild dieser Ansicht passt (S34).
///
/// **Die eine Zuordnung**, und sie ist eine Zeile und keine Tabelle:
/// `bestMatchFromAppearancesWithNames:` ist die Stelle, die AppKit fuer diese
/// Frage vorsieht, und sie beantwortet auch die Erscheinungsbilder mit erhoehtem
/// Kontrast, indem sie sie auf eines der beiden genannten abbildet.
///
/// Alles, was nicht das dunkle Erscheinungsbild ist, bekommt die helle Tafel.
/// Die Fallunterscheidung ist damit trennscharf und vollstaendig, ohne dass KRK
/// eine Liste der Erscheinungsbilder fuehrte, die das System kennt.
fn tafel_der_erscheinung(sicht: &NSView) -> Tafel {
    // SAFETY: Zwei Fremdsymbole von AppKit, die Namen der beiden
    // Erscheinungsbilder. Sie werden gelesen und nicht geschrieben.
    let (hell, dunkel) = unsafe { (NSAppearanceNameAqua, NSAppearanceNameDarkAqua) };
    let namen = NSArray::from_slice(&[hell, dunkel]);
    match sicht
        .effectiveAppearance()
        .bestMatchFromAppearancesWithNames(&namen)
    {
        Some(name) if *name == *dunkel => Tafel::Dunkel,
        _ => Tafel::Hell,
    }
}

/// Die feste Schreibmaschinenschrift des Nutzers, hilfsweise die Systemschrift.
///
/// Dieselbe Wahl und derselbe Rueckfall wie in `super::nummernspalte`. Ein
/// System ohne feste Schrift gibt es nicht; der Rueckfall steht da, weil die
/// Schnittstelle ihn zulaesst und ein Editor ohne Schrift keine Antwort ist.
fn feste_schrift(groesse: f64) -> Retained<NSFont> {
    NSFont::userFixedPitchFontOfSize(groesse).unwrap_or_else(|| NSFont::systemFontOfSize(groesse))
}

/// Ein Merkmalsverzeichnis mit genau einer Schrift darin.
fn schriftmerkmal(schrift: &NSFont) -> Retained<NSDictionary<NSString, AnyObject>> {
    // SAFETY: Ein Fremdsymbol von AppKit, der Merkmalsname der Schrift. Es wird
    // gelesen und nicht geschrieben.
    let schluessel = unsafe { [NSFontAttributeName] };
    let werte: [&AnyObject; 1] = [schrift];
    NSDictionary::from_slices(&schluessel, &werte)
}

/// Ein Merkmalsverzeichnis mit dem Einzug einer Listenzeile darin (C3).
fn einzugsmerkmal() -> Retained<NSDictionary<NSString, AnyObject>> {
    let stil = NSMutableParagraphStyle::new();
    // Beide, damit die erste Zeile mit dem Aufzaehlungszeichen genauso weit
    // einrueckt wie ihre Fortsetzung nach einem Umbruch; sonst haengt das
    // Zeichen als einziges am linken Rand.
    stil.setFirstLineHeadIndent(LISTENEINZUG);
    stil.setHeadIndent(LISTENEINZUG);
    // SAFETY: Ein Fremdsymbol von AppKit, der Merkmalsname des Absatzstils.
    let schluessel = unsafe { [NSParagraphStyleAttributeName] };
    let werte: [&AnyObject; 1] = [&stil];
    NSDictionary::from_slices(&schluessel, &werte)
}

/// Eine Farbe der Tafel als `NSColor`.
///
/// Im sRGB-Farbraum, weil die Tafeln ihre Werte darin angeben. Ohne Angabe des
/// Farbraums nimmt AppKit den kalibrierten, und dieselbe Zahl saehe dann anders
/// aus als in jedem anderen Programm, das dieselbe Tafel zeigt.
fn nsfarbe(farbe: Farbe) -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(
        f64::from(farbe.rot) / 255.0,
        f64::from(farbe.gruen) / 255.0,
        f64::from(farbe.blau) / 255.0,
        1.0,
    )
}

/// Was am Kopf des Editorbereichs steht (C4).
///
/// **Eine reine Funktion, damit die Anzeige des ungesicherten Standes ohne
/// Fenster abzunehmen ist.** Sie bekommt die beiden Angaben und gibt die Zeile;
/// woher die Angaben kommen und wohin die Zeile geht, steht in
/// [`Editorbereich::kopf_nachziehen`].
///
/// **Der Name und nicht der Pfad.** Der volle Pfad steht seit S48 im
/// Fenstertitel, solange der Fokus im Editor steht; ihn hier zu wiederholen
/// braechte zwei Anzeigen derselben Angabe und liesse in einem schmalen Editor
/// den Namen als erstes wegfallen. Ein Pfad ohne letzten Bestandteil ist auf
/// dem Mac kein Ziel des Editors; kaeme trotzdem einer, steht er ganz da, statt
/// dass der Kopf leer bliebe.
///
/// Ohne Datei bleibt der Kopf leer: der Editor zeigt dann nichts, was einen
/// Namen haette, und ein Platzhalter waere ein Wort ueber ein Nichts.
fn kopfzeile(pfad: Option<&Path>, ungesichert: bool) -> String {
    let Some(pfad) = pfad else {
        return String::new();
    };
    let name = pfad.file_name().map_or_else(
        || pfad.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    if ungesichert {
        format!("{ABWEICHUNGSZEICHEN} {name}")
    } else {
        name
    }
}

/// Baut den Kopf: eine einzeilige Beschriftung ueber der Textflaeche.
///
/// **Dieselben beiden Masse wie die Statuszeile der Dateifenster**, Hoehe und
/// Einzug, und aus demselben Grund: es ist dieselbe Form, naemlich eine Zeile
/// in der kleinen Systemschrift am Rand eines Bereichs. Zwei eigene Zahlen
/// daneben waeren zwei Antworten auf dieselbe Frage, und der Nutzer saehe zwei
/// verschieden hohe Streifen nebeneinander.
///
/// Die Farbe ist die zurueckgenommene Beschriftungsfarbe, wie bei der
/// Statuszeile ohne Meldung: der Kopf ist eine Angabe und keine Warnung. Das
/// Abweichungszeichen traegt die Aussage, nicht die Farbe; damit haengt sie
/// nicht am Farbsehen.
fn kopf_bauen(mtm: MainThreadMarker) -> Retained<NSTextField> {
    let kopf = NSTextField::labelWithString(ns_string!(""), mtm);
    kopf.setFont(Some(&NSFont::systemFontOfSize(
        NSFont::smallSystemFontSize(),
    )));
    kopf.setTextColor(Some(&NSColor::secondaryLabelColor()));
    kopf.setAlignment(NSTextAlignment::Left);
    kopf.setMaximumNumberOfLines(1);
    // Am oberen Rand festgemacht, in der Breite mitwachsend: der Abstand nach
    // unten ist beweglich, der nach oben nicht.
    kopf.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewMinYMargin,
    );
    kopf
}

/// Baut die Textflaeche: eine editierbare `NSTextView` in einer
/// Bildlaufansicht.
///
/// Editierbar und auswaehlbar, anders als die Textanzeige der Vorschau
/// (`super::vorschau`), die beides ablehnt, damit sie den Fokus nicht als
/// Textsystem nimmt. Der Editor **will** ihn nehmen, und der Fokusvorbehalt
/// laesst ihn ueber die Naemlichkeitsfrage aus dem Modulkopf durch.
///
/// Die Schrift ist die feste Schreibmaschinenschrift des Nutzers in
/// Systemgroesse; die Mindestbreite des Bereichs (320 Punkte) ist an ihr
/// gerechnet. Welche Schrift die Formatansicht aus C3 setzt, entscheidet ein
/// spaeterer Schritt.
fn textflaeche_bauen(
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
    text.setEditable(true);
    text.setSelectable(true);
    // Reiner Text, und die vier Ersetzungen aus: der gesicherte Stand ist der
    // getippte. Der Grund steht im Modulkopf.
    text.setRichText(false);
    text.setAutomaticQuoteSubstitutionEnabled(false);
    text.setAutomaticDashSubstitutionEnabled(false);
    text.setAutomaticTextReplacementEnabled(false);
    text.setAutomaticSpellingCorrectionEnabled(false);
    // Ohne diese Zeile traegt die Textansicht keine einzige
    // Rueckgaengig-Handlung, und die beiden Menueeintraege aus S7 finden am
    // Ende der Antwortkette einen leeren Verwalter vor. `allowsUndo` steht bei
    // einer programmatisch erzeugten `NSTextView` ab Werk auf `NO`; die
    // Menueseite derselben Sache steht in `super::menue`.
    text.setAllowsUndo(true);
    text.setVerticallyResizable(true);
    text.setHorizontallyResizable(false);
    text.setMinSize(NSSize::ZERO);
    text.setMaxSize(NSSize::new(f64::MAX, f64::MAX));
    text.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
    if let Some(schrift) = NSFont::userFixedPitchFontOfSize(NSFont::systemFontSize()) {
        text.setFont(Some(&schrift));
    }
    rolle.setDocumentView(Some(&text));
    // Die Nummernspalte aus C10, dieselbe Klasse, die die Vorschau einhaengt.
    // Sie steht im Editor immer: der Spec laesst sie nicht abschalten, und der
    // Editor zeigt ausschliesslich den Inhalt einer Datei.
    Nummernspalte::einhaengen(mtm, &rolle, &text);
    (rolle, text)
}

/// Die Meldungen sind reine Werte und brauchen kein Fenster; deshalb stehen die
/// Pruefungen hier und nicht unter `Nutzerarbeit`.
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use krk_core::text::marke::wiederfinden;

    use super::*;

    fn pfad() -> PathBuf {
        PathBuf::from("/tmp/probe.txt")
    }

    /// Das zehnte Abnahmekriterium von C2: jede Abweisung nennt ihren Grund,
    /// und "zu gross" ist von "nicht als Text lesbar" zu unterscheiden.
    #[test]
    fn die_drei_abweisungsgruende_tragen_drei_verschiedene_saetze() {
        let saetze = [
            Editormeldung::Abgewiesen(Abweisung::KeinGueltigesZiel {
                pfad: pfad(),
                grund: "ein Ordner".into(),
            })
            .text(),
            Editormeldung::Abgewiesen(Abweisung::ZuGross {
                pfad: pfad(),
                groesse: 20 * 1024 * 1024,
            })
            .text(),
            Editormeldung::Abgewiesen(Abweisung::NichtAlsTextLesbar { pfad: pfad() }).text(),
        ];
        for satz in &saetze {
            assert!(
                !satz.is_empty(),
                "kommentarlos nichts zu sagen ist unzulässig"
            );
        }
        assert_ne!(saetze[0], saetze[1]);
        assert_ne!(saetze[1], saetze[2]);
        assert_ne!(saetze[0], saetze[2]);
    }

    /// Das achte Abnahmekriterium von C6: nur der Fehlschlag meldet, und er
    /// meldet, dass die Stelle sich geaendert hat.
    #[test]
    fn allein_die_nicht_wiedergefundene_markenstelle_meldet_sich() {
        let text = "eins\nzwei\ndrei\n";
        // Der gemerkte Inhalt steht auf der gemerkten Nummer.
        assert_eq!(
            Editormeldung::markenstelle(&wiederfinden(text, 2, "zwei")),
            None
        );
        // Er steht daneben und wird im Fenster wiedergefunden.
        assert_eq!(
            Editormeldung::markenstelle(&wiederfinden(text, 1, "drei")),
            None
        );
        // Er ist fort: die Marke fuehrt trotzdem, und der Sprung meldet sich.
        let sprung = wiederfinden(text, 2, "vier");
        assert_eq!(sprung.fund, Fund::NichtGefunden);
        let meldung = Editormeldung::markenstelle(&sprung)
            .expect("ein nicht wiedergefundener Inhalt meldet sich");
        assert_eq!(meldung, Editormeldung::MarkenstelleGeaendert { zeile: 2 });
        assert!(
            meldung.text().contains('2'),
            "die Meldung nennt die Zeile, an die sie geführt hat"
        );
    }

    /// C4: beide Ausgaenge des Sicherns melden sich, und sie melden
    /// Verschiedenes.
    ///
    /// Der Grund des Fehlschlags kommt fertig aus dem Modell; geprueft wird
    /// hier, dass die Meldung ihn unveraendert weitergibt, statt einen zweiten
    /// Satz daneben zu bauen.
    #[test]
    fn das_sichern_meldet_gelingen_und_fehlschlag_verschieden() {
        let gelungen = Editormeldung::Gesichert { pfad: pfad() }.text();
        assert!(
            gelungen.contains("probe.txt"),
            "die Meldung nennt die geschriebene Datei: {gelungen}"
        );

        let grund = "/tmp/probe.txt ließ sich nicht sichern: Permission denied";
        let gescheitert = Editormeldung::SichernGescheitert {
            grund: grund.to_owned(),
        }
        .text();
        assert_eq!(
            gescheitert, grund,
            "der Grund des Modells geht unverändert durch"
        );
        assert_ne!(gelungen, gescheitert);
    }

    /// Das zweite Abnahmekriterium von C4, an der Stelle gemessen, an der der
    /// Satz entsteht: der Kopf traegt den Namen, und ein ungesicherter Stand
    /// setzt ein Zeichen davor.
    #[test]
    fn der_kopf_zeigt_den_namen_und_bei_abweichung_ein_zeichen() {
        let pfad = PathBuf::from("/tmp/tief/lies.md");

        assert_eq!(kopfzeile(Some(&pfad), false), "lies.md");
        let abweichend = kopfzeile(Some(&pfad), true);
        assert_ne!(
            abweichend, "lies.md",
            "ein ungesicherter Stand ist am Kopf zu sehen"
        );
        assert!(abweichend.contains("lies.md"), "der Name bleibt lesbar");
        assert!(
            abweichend.starts_with(ABWEICHUNGSZEICHEN),
            "das Zeichen steht vorn, wo eine Kürzung von rechts es nicht erreicht: {abweichend}"
        );
    }

    /// Der Kopf nennt den Namen und nicht den Pfad; den vollen Pfad traegt der
    /// Fenstertitel aus C11.
    #[test]
    fn der_kopf_nennt_den_namen_und_nicht_den_pfad() {
        let pfad = PathBuf::from("/Users/jemand/Projekte/krk/lies.md");
        assert_eq!(kopfzeile(Some(&pfad), false), "lies.md");
    }

    /// Ohne gehaltene Datei bleibt der Kopf leer.
    #[test]
    fn ohne_datei_bleibt_der_kopf_leer() {
        assert_eq!(kopfzeile(None, false), "");
        assert_eq!(
            kopfzeile(None, true),
            "",
            "ohne Datei gibt es auch nichts, was abweichen könnte"
        );
    }
}
