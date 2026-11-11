//! Die Schaltflaechenreihe ueber einer Textflaeche: eine Reihe aus
//! Beschriftung und Kommando, gebaut fuer die Quicknote
//! (`super::quicknote`) und fuer den Text einer Protokolldatei im Editor
//! (Plan `261111-2248-plan-protokolldatei-traegt-kopieren-singlestrip-format.md`,
//! E5).
//!
//! # Eine Reihe fuer beide Flaechen
//!
//! Bis zu jenem Plan baute die Quicknote ihre fuenf Schaltflaechen selbst, je
//! mit eigenem Selektor. Die drei, die auch vor einer Protokolldatei stehen,
//! sind dieselben Kommandos mit denselben Beschriftungen; damit sie nicht an
//! zwei Stellen gebaut werden, haelt diese Datei den Bau und **beide Listen
//! als Daten**: [`QUICKNOTEKNOEPFE`], die fuenf der Quicknote von links nach
//! rechts, und [`PROTOKOLLKNOEPFE`], die drei des Protokolls, gebildet als
//! Ausschnitt der fuenf und nicht als zweite Aufzaehlung. Beschriftet wird
//! aus der Sprachtabelle; die Liste haelt den Schluessel und nicht den Text,
//! weil eine `const` keinen Tabellentext halten kann.
//!
//! # Ein Klick ist ein Kommando
//!
//! Alle Schaltflaechen einer Reihe teilen sich **einen** Selektor und
//! unterscheiden sich allein in ihrer Marke (`tag`), der Stelle ihres Eintrags
//! in der gehaltenen Liste; eine Marke ohne Eintrag tut nichts
//! ([`kommando_der_marke`]). Das Kommando geht ueber den Melder, den der
//! Besitzer der Reihe setzt, durch dieselbe Zulaessigkeit wie die Taste. Die
//! Schaltflaechen nehmen den Ersthelferrang nicht an, und **ein Klick holt
//! zuerst den Fokus in die Textflaeche und meldet dann das Kommando**
//! ([`Knopfzeile::geklickt`]): die Zulaessigkeit fragt nach dem Fokus, und ein
//! Klick aus einem anderen Bereich heraus hat ihn noch nicht im Editor.
//!
//! Keine Aktion heisst `copy:`, `cut:` oder `paste:`: die Zwischenablage
//! erreicht diese Datei nicht, das Kopieren geht beim Anwendungsdelegierten
//! ueber `super::zwischenablage::text_schreiben`.
//!
//! # Wer wen haelt
//!
//! Die Reihe haelt ihre Ansicht und ihr Fokusziel stark; `NSControl` haelt
//! sein Ziel schwach, also haelt keine Schaltflaeche die Reihe. Der Besitzer
//! haelt die Reihe so lange wie die Ansicht, in der sie steht, sonst zeigte
//! das Ziel der Schaltflaechen ins Leere und ein Klick taete nichts. Der
//! Melder haelt seinen Empfaenger schwach, wie an der Bereichsleiste.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSView`, `NSTextView`, `NSButton` und `NSObject` stehen seit macOS 10.0,
//! ebenso das Protokoll `NSObjectProtocol` und die Methoden `initWithFrame:`,
//! `setAutoresizingMask:`, `addSubview:`, `frame`, `setFrame:` und `window`
//! an `NSView`, `makeFirstResponder:` an `NSWindow`, `sizeToFit`, `tag` und
//! `setTag:` an `NSControl` (`NSControl.h`, ohne eigene
//! Verfuegbarkeitsangabe), `setRefusesFirstResponder:` und, in der Probe,
//! `title` an `NSButton`, `subviews` an `NSView` und `performClick:` an
//! `NSButton`. `NSString` steht seit 10.0. **Juenger als seine Klasse ist
//! allein `buttonWithTitle:target:action:`**, seit macOS 10.12
//! (`NSButton.h:40`, `API_AVAILABLE(macos(10.12))`). Die Wertetypen `NSRect`,
//! `NSPoint`, `NSSize` und die Maske `NSAutoresizingMaskOptions` haben kein
//! eigenes macOS-Alter, und `MainThreadMarker` ist ein Rust-Typ der Kiste.
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine Beruehrung in
//! dieser Datei liegt darueber.

use std::cell::RefCell;

use objc2::rc::Retained;
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{NSAutoresizingMaskOptions, NSButton, NSTextView, NSView};
use objc2_foundation::{
    MainThreadMarker, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString,
};

use krk_core::sprache::{Text, text};
use krk_core::tasten::Kommando;

use super::bereichsleiste::Kommandomelder;

/// Die Hoehe der Schaltflaechenreihe.
pub(super) const KNOPFZEILE: f64 = 30.0;

/// Der Abstand zwischen zwei Schaltflaechen und zum linken Rand.
const KNOPFABSTAND: f64 = 8.0;

/// Ein Eintrag der Reihe: der Schluessel der Beschriftung und das Kommando,
/// das der Klick meldet.
pub(super) type Knopfeintrag = (Text, Kommando);

/// Die fuenf Schaltflaechen der Quicknote, von links nach rechts.
///
/// „Schliessen" meldet `QuicknoteUmschalten`, dasselbe Kommando wie F10 mit
/// dem Fokus in der Quicknote.
pub(super) const QUICKNOTEKNOEPFE: &[Knopfeintrag] = &[
    (Text::QuicknoteLeeren, Kommando::QuicknoteLeeren),
    (Text::BlattSchliessen, Kommando::QuicknoteUmschalten),
    (Text::QuicknoteKopieren, Kommando::QuicknoteKopieren),
    (Text::QuicknoteSinglestrip, Kommando::QuicknoteSinglestrip),
    (Text::QuicknoteFormat, Kommando::QuicknoteFormat),
];

/// Wie viele Eintraege von [`QUICKNOTEKNOEPFE`] vor den dreien des Protokolls
/// stehen: Leeren und Schliessen, die allein der Quicknote gehoeren.
const NUR_QUICKNOTE: usize = 2;

/// Die drei Schaltflaechen vor dem Text einer Protokolldatei: Kopieren,
/// Singlestrip, Format.
///
/// **Ein Ausschnitt der fuenf und keine zweite Liste**, damit Beschriftung
/// und Kommando an beiden Flaechen dieselben Eintraege sind.
pub(super) const PROTOKOLLKNOEPFE: &[Knopfeintrag] = QUICKNOTEKNOEPFE.split_at(NUR_QUICKNOTE).1;

/// Das Kommando des Eintrags, dessen Stelle `marke` nennt, oder `None` fuer
/// eine Marke ohne Eintrag.
fn kommando_der_marke(eintraege: &[Knopfeintrag], marke: isize) -> Option<Kommando> {
    let stelle = usize::try_from(marke).ok()?;
    eintraege.get(stelle).map(|(_, kommando)| *kommando)
}

/// Was die Reihe haelt.
pub struct KnopfzeileIvars {
    /// Die Ansicht der Reihe, [`KNOPFZEILE`] hoch; sie traegt die
    /// Schaltflaechen.
    ansicht: Retained<NSView>,
    /// Die Eintraege, in der Reihenfolge der Schaltflaechen; die Marke einer
    /// Schaltflaeche ist die Stelle ihres Eintrags.
    eintraege: &'static [Knopfeintrag],
    /// Die Textflaeche, in die ein Klick den Fokus holt.
    fokusziel: Retained<NSTextView>,
    /// Die Senke fuer den Klick; `None`, bis der Besitzer sie setzt. Sie haelt
    /// ihren Empfaenger schwach.
    melder: RefCell<Option<Kommandomelder>>,
}

define_class!(
    /// Die Schaltflaechenreihe und das Ziel ihrer Schaltflaechen.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = KnopfzeileIvars]
    pub struct Knopfzeile;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Knopfzeile {}

    impl Knopfzeile {
        /// Der eine Selektor aller Schaltflaechen; welches Kommando gemeint
        /// ist, sagt die Marke des Absenders.
        // SAFETY: Die Signatur ist die einer Handlung von AppKit; der Absender
        // ist eine Schaltflaeche, die diese Datei mit diesem Selektor baut.
        #[unsafe(method(knopfGeklickt:))]
        fn knopf_geklickt(&self, absender: &NSButton) {
            if let Some(kommando) = kommando_der_marke(self.ivars().eintraege, absender.tag()) {
                self.geklickt(kommando);
            }
        }
    }
);

impl Knopfzeile {
    /// Baut die Reihe aus `eintraege` mit ihrer linken unteren Ecke an
    /// `ursprung`, `breite` breit und [`KNOPFZEILE`] hoch.
    ///
    /// Die Ansicht klebt oben und waechst in der Breite mit, wenn die Ansicht
    /// waechst, in die der Besitzer sie legt; eingehaengt und ein- oder
    /// ausgeblendet wird sie vom Besitzer ueber [`Self::ansicht`]. `fokusziel`
    /// ist die Textflaeche, auf die die Kommandos der Reihe wirken.
    pub fn bauen(
        mtm: MainThreadMarker,
        eintraege: &'static [Knopfeintrag],
        fokusziel: &NSTextView,
        ursprung: NSPoint,
        breite: f64,
    ) -> Retained<Self> {
        let ansicht = NSView::initWithFrame(
            NSView::alloc(mtm),
            NSRect::new(ursprung, NSSize::new(breite, KNOPFZEILE)),
        );
        ansicht.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewMinYMargin,
        );
        let this = Self::alloc(mtm).set_ivars(KnopfzeileIvars {
            ansicht,
            eintraege,
            fokusziel: fokusziel.retain(),
            melder: RefCell::new(None),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };
        this.knoepfe_bauen(mtm);
        this
    }

    /// Baut je Eintrag eine Schaltflaeche in die Ansicht, von links nach
    /// rechts, mit der Stelle des Eintrags als Marke.
    fn knoepfe_bauen(&self, mtm: MainThreadMarker) {
        let mut links = KNOPFABSTAND;
        for (marke, (titel, _)) in (0_isize..).zip(self.ivars().eintraege) {
            // SAFETY: `self` beantwortet den Selektor mit der Signatur einer
            // Handlung (siehe `define_class!` oben), und `sel!` liefert einen
            // gueltigen Selektor. `NSControl` haelt sein Ziel schwach.
            let knopf = unsafe {
                NSButton::buttonWithTitle_target_action(
                    &NSString::from_str(text(*titel)),
                    Some(self),
                    Some(sel!(knopfGeklickt:)),
                    mtm,
                )
            };
            knopf.setTag(marke);
            knopf.setRefusesFirstResponder(true);
            knopf.sizeToFit();
            let groesse = knopf.frame().size;
            knopf.setFrame(NSRect::new(
                NSPoint::new(links, (KNOPFZEILE - groesse.height) / 2.0),
                groesse,
            ));
            self.ivars().ansicht.addSubview(&knopf);
            links += groesse.width + KNOPFABSTAND;
        }
    }

    /// Traegt die Senke fuer einen Klick ein.
    pub fn melder_setzen(&self, melder: Kommandomelder) {
        *self.ivars().melder.borrow_mut() = Some(melder);
    }

    /// Die Ansicht der Reihe, die der Besitzer einhaengt und ein- und
    /// ausblendet.
    pub fn ansicht(&self) -> &NSView {
        &self.ivars().ansicht
    }

    /// Ein Klick: zuerst den Fokus in die Textflaeche, dann das Kommando.
    ///
    /// **Die Reihenfolge ist die Aussage**, nach dem Muster des Ankreuzfeldes
    /// der Aufgabentabelle: die Zulaessigkeit fragt nach dem Fokus, und ein
    /// Klick aus einem anderen Bereich heraus hat ihn noch nicht im Editor.
    fn geklickt(&self, kommando: Kommando) {
        let fokusziel = &self.ivars().fokusziel;
        if let Some(fenster) = fokusziel.window() {
            // `let _ =`: lehnt AppKit ab, weist die Zulaessigkeit das Kommando
            // ab, und der Klick tut nichts, wie jede abgewiesene Taste.
            let _ = fenster.makeFirstResponder(Some(fokusziel));
        }
        let melder = self.ivars().melder.borrow();
        if let Some(melder) = melder.as_ref() {
            melder(kommando);
        }
    }
}

#[cfg(test)]
pub(super) use tests::HAUPTFADENPROBEN;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::super::anwendung::quelltextproben::{datei, rumpf};
    use super::super::hauptfaden::{Probe, hauptfadenprobe};
    use super::*;

    /// Die Proben dieses Moduls, die auf dem Hauptfaden laufen.
    ///
    /// Eine Funktion `fn(MainThreadMarker)` dieses Moduls, die hier fehlt, ist
    /// im Bau von `libtest` toter Code und macht `make lint` rot.
    pub(crate) const HAUPTFADENPROBEN: &[Probe] = &[hauptfadenprobe!(
        die_reihe_des_protokolls_traegt_drei_schaltflaechen_und_meldet_ihre_kommandos
    )];

    /// Keine Codezeile dieser Datei nennt die Zwischenablage; das Kopieren
    /// geht beim Anwendungsdelegierten ueber die eine Huelle.
    #[test]
    fn die_knopfzeile_nennt_die_zwischenablage_nicht() {
        let quelle = datei("krk-ui/src/appkit/knopfzeile.rs");
        let (code, _) = quelle
            .split_once(concat!("#[cfg(test)]\nmod ", "tests {"))
            .expect("das Pruefmodul steht am Fuss der Datei");
        let nennt = code
            .lines()
            .filter(|zeile| !zeile.trim_start().starts_with("//"))
            .any(|zeile| zeile.contains(concat!("NSPaste", "board")));
        assert!(
            !nennt,
            "die Knopfzeile spricht die Zwischenablage selbst an"
        );
    }

    /// Die fuenf Schaltflaechen der Quicknote melden ihre fuenf Kommandos in
    /// der Reihenfolge von links nach rechts, jede ueber den einen Selektor
    /// und `geklickt`, das zuerst den Fokus holt.
    #[test]
    fn die_aktionen_melden_ihre_kommandos() {
        assert_eq!(
            QUICKNOTEKNOEPFE,
            [
                (Text::QuicknoteLeeren, Kommando::QuicknoteLeeren),
                (Text::BlattSchliessen, Kommando::QuicknoteUmschalten),
                (Text::QuicknoteKopieren, Kommando::QuicknoteKopieren),
                (Text::QuicknoteSinglestrip, Kommando::QuicknoteSinglestrip),
                (Text::QuicknoteFormat, Kommando::QuicknoteFormat),
            ]
        );
        for (marke, (_, kommando)) in (0_isize..).zip(QUICKNOTEKNOEPFE) {
            assert_eq!(kommando_der_marke(QUICKNOTEKNOEPFE, marke), Some(*kommando));
        }

        let quelle = datei("krk-ui/src/appkit/knopfzeile.rs");
        let aktion = rumpf(&quelle, "knopf_geklickt");
        assert!(
            aktion.contains("kommando_der_marke(self.ivars().eintraege, absender.tag())"),
            "die Aktion liest das Kommando nicht ueber die Marke aus der gehaltenen Liste"
        );
        assert!(
            aktion.contains("self.geklickt(kommando)"),
            "die Aktion geht nicht ueber geklickt"
        );
        let geklickt = rumpf(&quelle, "geklickt");
        let fokus = geklickt
            .find(concat!("makeFirst", "Responder("))
            .expect("der Klick holt den Fokus nicht");
        let melden = geklickt
            .find("melder(kommando)")
            .expect("der Klick meldet kein Kommando");
        assert!(fokus < melden, "der Klick meldet vor dem Fokus");
    }

    /// Eine Marke, zu der die Liste keinen Eintrag fuehrt, meldet nichts.
    #[test]
    fn eine_fremde_marke_tut_nichts() {
        for liste in [QUICKNOTEKNOEPFE, PROTOKOLLKNOEPFE] {
            let laenge = isize::try_from(liste.len()).expect("die Liste ist kurz");
            for marke in [-1, laenge, laenge + 1, isize::MAX, isize::MIN] {
                assert_eq!(kommando_der_marke(liste, marke), None, "Marke {marke}");
            }
        }
    }

    /// Die drei des Protokolls sind Kopieren, Singlestrip und Format in dieser
    /// Reihenfolge, und jeder ist ein Eintrag der fuenf der Quicknote.
    #[test]
    fn die_drei_des_protokolls_sind_eintraege_der_fuenf() {
        let kommandos: Vec<Kommando> = PROTOKOLLKNOEPFE
            .iter()
            .map(|(_, kommando)| *kommando)
            .collect();
        assert_eq!(
            kommandos,
            [
                Kommando::QuicknoteKopieren,
                Kommando::QuicknoteSinglestrip,
                Kommando::QuicknoteFormat,
            ]
        );
        for eintrag in PROTOKOLLKNOEPFE {
            assert!(
                QUICKNOTEKNOEPFE.contains(eintrag),
                "{eintrag:?} steht nicht unter den fuenf der Quicknote"
            );
        }
        assert!(
            !kommandos.contains(&Kommando::QuicknoteLeeren),
            "Leeren gehoert allein der Quicknote"
        );
    }

    /// Eine Reihe aus den dreien des Protokolls traegt drei Schaltflaechen mit
    /// den Titeln der Sprachtabelle, [`KNOPFZEILE`] hoch, und `performClick:`
    /// meldet je das Kommando des Eintrags.
    ///
    /// Das Fokusziel steht in keinem Fenster; den Fokus holt der Klick hier
    /// also nicht, und die Reihenfolge haelt `die_aktionen_melden_ihre_kommandos`
    /// am Quelltext.
    fn die_reihe_des_protokolls_traegt_drei_schaltflaechen_und_meldet_ihre_kommandos(
        mtm: MainThreadMarker,
    ) {
        let flaeche = NSTextView::initWithFrame(
            NSTextView::alloc(mtm),
            NSRect::new(NSPoint::ZERO, NSSize::new(400.0, 200.0)),
        );
        let reihe = Knopfzeile::bauen(
            mtm,
            PROTOKOLLKNOEPFE,
            &flaeche,
            NSPoint::new(0.0, 200.0),
            400.0,
        );
        let gemeldet: Rc<RefCell<Vec<Kommando>>> = Rc::new(RefCell::new(Vec::new()));
        let senke = Rc::clone(&gemeldet);
        reihe.melder_setzen(Box::new(move |kommando| senke.borrow_mut().push(kommando)));

        let rahmen = reihe.ansicht().frame();
        assert_eq!(rahmen.size.height, KNOPFZEILE);
        assert_eq!(rahmen.size.width, 400.0);
        assert_eq!(rahmen.origin.y, 200.0);

        let knoepfe: Vec<Retained<NSButton>> = reihe
            .ansicht()
            .subviews()
            .iter()
            .map(|ansicht| {
                ansicht
                    .downcast::<NSButton>()
                    .expect("in der Reihe steht etwas anderes als eine Schaltflaeche")
            })
            .collect();
        assert_eq!(knoepfe.len(), 3);

        let mut rechts_davor = 0.0;
        for (knopf, (titel, kommando)) in knoepfe.iter().zip(PROTOKOLLKNOEPFE) {
            assert_eq!(knopf.title().to_string(), text(*titel));
            let lage = knopf.frame();
            assert!(
                lage.origin.x >= rechts_davor + KNOPFABSTAND,
                "{titel:?} steht nicht rechts von seinem Vorgaenger"
            );
            rechts_davor = lage.origin.x + lage.size.width;
            // SAFETY: Der Absender darf fehlen; das Ziel der Schaltflaeche ist
            // `reihe`, die bis zum Ende der Probe lebt.
            unsafe { knopf.performClick(None) };
            assert_eq!(gemeldet.borrow().last(), Some(kommando), "{titel:?}");
        }
        assert_eq!(
            *gemeldet.borrow(),
            [
                Kommando::QuicknoteKopieren,
                Kommando::QuicknoteSinglestrip,
                Kommando::QuicknoteFormat,
            ]
        );
    }
}
