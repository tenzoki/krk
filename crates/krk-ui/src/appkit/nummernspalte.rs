//! Die Zeilennummern neben dem Text: **eine** `NSRulerView` in der senkrechten
//! Linealstelle einer `NSScrollView`, im Editor und in der Vorschau (C10).
//!
//! ```text
//! ┌────┬─────────────────────────┐
//! │ 12 │ NSScrollView            │  die Spalte ist Schwesteransicht des
//! │ 13 │   NSTextView            │  Textbehaelters und nicht sein Inhalt
//! └────┴─────────────────────────┘
//! ```
//!
//! **Eine Klasse fuer beide Flaechen.** Editor und Vorschau tragen dieselbe
//! Bauart, eine `NSTextView` in einer `NSScrollView` mit Umlauf an der
//! Behaelterbreite; [`super::editor`] und [`super::vorschau`] haengen deshalb
//! dieselbe Klasse ein, statt zwei aehnliche zu halten. Der Spec sagt das unter
//! C10 zu, und zwei Spalten waeren zwei Zaehlungen fuer dieselbe Frage.
//!
//! **Die Spalte gehoert nicht zum Text.** Sie ist eine Schwesteransicht des
//! Textbehaelters und nicht sein Inhalt: sie laesst sich nicht mitauswaehlen,
//! geht beim Kopieren nicht mit und kann beim Sichern nicht in die Datei
//! geraten. Das letzte Abnahmekriterium von C10 faellt daraus an und ist nicht
//! eigens gebaut.
//!
//! # Die Zaehlung kommt aus dem Kern, und eine zweite entsteht hier nicht
//!
//! Gezaehlt wird in [`Zeilenindex`] (`krk_core::text::zeilen`). Diese Datei
//! rechnet keine Zeile selbst: sie haelt einen Index, fragt ihn nach dem Anfang
//! jeder Zeile und nach der Zeilenzahl, und sucht darin. Was der Index nicht
//! liefert, ist die Stelle auf dem Schirm; die weiss allein der Layoutverwalter
//! der Textflaeche, und beim Umlauf am Fensterrand ist er die einzige Stelle,
//! die sie weiss. Zwei Fragen an die beiden, die je eine Haelfte kennen.
//!
//! **Zwei Koordinaten, ein Anfang.** Der Index rechnet in Byteversaetzen eines
//! UTF-8-Textes, AppKit in UTF-16-Einheiten. [`super::koordinaten`] wechselt
//! die Koordinate und nichts weiter; welche Stellen Zeilenanfang sind und wie
//! viele es gibt, sagt weiterhin allein der Index. Ohne den Wechsel truege jede
//! Zeile hinter dem ersten Umlaut eine falsche Nummer.
//!
//! Die Umrechnung stand bis zum 260810 hier und war privat. Sie ist mit dem
//! Zeilensprung und der Suche aus C5 in ein eigenes Modul gewandert, weil beide
//! sie brauchen und ein zweiter Rechenweg fuer dieselbe Frage entstanden waere;
//! [`anfaenge_in_utf16`] ist seither die Zeile, die den Index befragt, und die
//! Rechnung selbst steht nebenan.
//!
//! **Genau eine Nummer je Dateizeile.** Gezeichnet wird nur dort, wo der Anfang
//! eines Zeilenkastens des Layoutverwalters zugleich ein Zeilenanfang der Datei
//! ist. Eine umgelaufene Zeile bekommt damit ihre Nummer neben der ersten
//! Bildschirmzeile und ihre Fortsetzungen keine; das vierte Abnahmekriterium
//! von C10 faellt aus einer Suche in den Zeilenanfaengen an und nicht aus einer
//! Sonderregel fuer den Umbruch.
//!
//! **Ein Zeilenende, und das ist eine Zusage von anderswo.** Der Index kennt
//! `\n` und sonst nichts, weil der gehaltene Stand des Editors durch
//! `krk_core::text::datei::in_gehaltene_form` gelaufen ist. Die Vorschau zeigt
//! den Dateiinhalt dagegen ungewandelt; eine Datei mit `\r\n` zaehlt trotzdem
//! richtig, weil der Layoutverwalter das Paar als einen Umbruch auslegt und
//! sein Zeilenkasten hinter demselben `\n` beginnt, hinter dem der Index seine
//! Zeile beginnen laesst. Eine Datei mit einzelnen `\r` traegt weniger Zeilen,
//! als der Layoutverwalter Kaesten zeichnet, und die Spalte zeigt dann genau
//! die Zaehlung des Index. Das ist Absicht: der Zeilensprung aus C5 und die
//! Textmarke aus C6 rechnen in derselben Zaehlung, und eine Anzeige mit einer
//! zweiten Meinung ueber Zeilenenden fuehrte den Nutzer an die falsche Stelle.
//!
//! # Wann neu gezeichnet und wann neu gezaehlt wird
//!
//! Der Index wird als **ueberholt** gekennzeichnet, wenn sich der Text aendert,
//! und beim naechsten Zeichnen neu gebaut. Damit faellt je gezeichnetem Bild
//! hoechstens ein Neuaufbau an und nicht je Anschlag. Zwei Beobachter setzen
//! ihn an:
//!
//! ```text
//! NSTextStorageDidProcessEditingNotification ──> ueberholt + neu zeichnen
//! NSViewBoundsDidChangeNotification (Klemme)  ──> nur neu zeichnen
//! ```
//!
//! Der zweite kennzeichnet nichts: beim Blaettern hat sich der Text nicht
//! geaendert. Der Weg ueber den **Textspeicher** und nicht ueber den
//! Delegierten der Flaeche ist Absicht: eine `NSTextView` hat einen
//! Delegierten, den der Editor fuer sich braucht, ihr Textspeicher aber
//! beliebig viele Beobachter, und die Vorschau hat gar keinen Delegierten.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSRulerView`, `NSLayoutManager`, `NSTextContainer`, `NSTextStorage` und
//! `NSClipView` stehen seit macOS 10.0 zur Verfuegung; das Buendel zielt auf
//! 15.0 (`.cargo/config.toml`). Keine von ihnen ist nach macOS 15
//! hinzugekommen, und deshalb braucht keine der Beruehrungen in dieser Datei
//! eine Verfuegbarkeitspruefung zur Laufzeit.
//!
//! Der Zugriff auf `layoutManager` laesst AppKit auf den aelteren
//! `NSLayoutManager` statt auf `NSTextLayoutManager` zurueckfallen. Der
//! Rueckfall ist von diesem Plan bereits eingekauft: die Einfaerbung der
//! Formatansicht legt ihre voruebergehenden Merkmale in denselben Verwalter.
//! Beide sprechen denselben an; ein zweiter Textfluss entsteht nicht.
//!
//! **Das Rueckgaengig des Editors haengt nicht mehr mit daran.** Bis zum
//! 260810-1243 tat es das, ohne dass eine Zeile es sagte: `textDidChange:` ist
//! der eine Rueckweg aus der Textflaeche in das `Editormodell`, es feuert bei
//! einem `undo` nur auf TextKit 1, und den Rueckfall dorthin loeste die
//! Nummernspalte nebenbei mit aus. Seither stellt `textflaeche_bauen` in
//! [`super::editor`] ihn mit einer eigenen Zeile und ihrem Grund selbst her, und
//! die Probe `appkit::editor::tests::die_gebaute_flaeche_steht_auf_textkit_1`
//! faellt aus, sobald jemand diese Zeile wegnimmt. Die Messung steht im
//! Modulkopf von [`super::editor`] und hier absichtlich nicht ein zweites Mal.
//!
//! Wer diese Datei auf `NSTextLayoutManager` nachzieht, nimmt dem Editor damit
//! also **nicht** mehr sein Rueckgaengig weg. Zu klaeren bleibt allein die
//! Einfaerbung aus dem Absatz darueber, denn die legt ihre voruebergehenden
//! Merkmale weiter in den aelteren Verwalter.

use std::cell::{Cell, RefCell};

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSColor, NSFont, NSFontAttributeName, NSForegroundColorAttributeName, NSRulerOrientation,
    NSRulerView, NSScrollView, NSStringDrawing, NSTextStorageDidProcessEditingNotification,
    NSTextView, NSViewBoundsDidChangeNotification,
};
use objc2_foundation::{
    MainThreadMarker, NSDictionary, NSNotification, NSNotificationCenter, NSObject,
    NSObjectNSDelayedPerforming, NSObjectProtocol, NSPoint, NSRange, NSRect, NSString,
};

use krk_core::text::Zeilenindex;

use super::koordinaten;

/// Der Abstand der Nummer zum linken und zum rechten Rand der Spalte.
const RAND: f64 = 5.0;

/// Die Zahl der Stellen, fuer die die Spalte mindestens Platz haelt.
///
/// Zwei, damit eine Datei mit neun Zeilen keine spuerbar schmalere Spalte hat
/// als dieselbe Datei nach der zehnten Zeile. Darueber waechst sie mit der
/// Stellenzahl, so dass auch eine sechsstellige Nummer vollstaendig steht; das
/// zehnte Abnahmekriterium von C10 verlangt es.
const MINDESTSTELLEN: usize = 2;

/// Was die Nummernspalte haelt.
pub struct NummernspalteIvars {
    /// Die Textflaeche, deren Zeilen gezaehlt werden.
    ///
    /// Stark gehalten und trotzdem ohne Ring: die Bildlaufansicht haelt die
    /// Spalte und ueber ihre Klemme die Flaeche, und die Flaeche haelt keine
    /// der beiden.
    flaeche: Retained<NSTextView>,
    /// Die Zaehlung aus dem Kern. Sie sagt, wie viele Zeilen der Text hat und
    /// wo jede anfaengt.
    index: RefCell<Zeilenindex>,
    /// Dieselben Zeilenanfaenge in AppKits Koordinate, aufsteigend.
    ///
    /// Keine zweite Zaehlung, sondern der Koordinatenwechsel aus dem Modulkopf:
    /// die Stelle eines Wertes in dieser Liste **ist** die um eins verminderte
    /// Zeilennummer, weil die Liste aus dem Index entsteht.
    anfaenge: RefCell<Vec<usize>>,
    /// Ob [`Self::index`] und [`Self::anfaenge`] noch zum Text der Flaeche
    /// passen.
    ueberholt: Cell<bool>,
    /// Schrift und Farbe, mit denen eine Nummer gezeichnet wird.
    ///
    /// Einmal gebaut und nicht je Nummer: `secondaryLabelColor` ist eine
    /// Systemfarbe und loest sich beim Zeichnen gegen das gerade geltende
    /// Erscheinungsbild auf, in Hell wie in Dunkel.
    merkmale: Retained<NSDictionary<NSString, AnyObject>>,
}

define_class!(
    /// Die Nummernspalte neben einer Textflaeche (C10).
    // SAFETY:
    // - Die Oberklasse NSRulerView stellt an Unterklassen keine Bedingungen
    //   ausser der, `drawHashMarksAndLabelsInRect:` in den Koordinaten der
    //   Spalte zu zeichnen; genau das tut `zeichnen`.
    // - Die Klasse implementiert `Drop`: er meldet die beiden Beobachter wieder
    //   ab, ruft keine ueberschriebene Methode und haelt das Objekt nicht ueber
    //   die Lebensdauer des Aufrufs hinaus fest.
    #[unsafe(super = NSRulerView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = NummernspalteIvars]
    pub struct Nummernspalte;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Nummernspalte {}

    impl Nummernspalte {
        /// Zeichnet die Nummern des sichtbaren Ausschnitts.
        // SAFETY: Die Signatur entspricht der von NSRulerView.
        #[unsafe(method(drawHashMarksAndLabelsInRect:))]
        fn zeichnen(&self, _bereich: NSRect) {
            self.nummern_zeichnen();
        }

        /// Der Text hat sich geaendert: die Zaehlung ist ueberholt.
        // SAFETY: Die Signatur passt zu der einer Meldungsannahme.
        #[unsafe(method(textGeaendert:))]
        fn text_geaendert(&self, _meldung: &NSNotification) {
            self.ivars().ueberholt.set(true);
            self.neu_zeichnen();
        }

        /// Es wurde geblaettert: dieselbe Zaehlung, ein neues Bild.
        // SAFETY: Die Signatur passt zu der einer Meldungsannahme.
        #[unsafe(method(sichtVerschoben:))]
        fn sicht_verschoben(&self, _meldung: &NSNotification) {
            self.neu_zeichnen();
        }

        /// Setzt die Breite der Spalte auf die Stellenzahl der groessten
        /// Nummer.
        ///
        /// **Sie laeuft nach dem Zeichnen und nicht darin**, und das ist der
        /// Grund, aus dem sie eine eigene Methode ist: `setRuleThickness:`
        /// legt die Bildlaufansicht neu aus, und eine Auslegung mitten in
        /// einem Zeichendurchgang aenderte die Geometrie, in der gerade
        /// gezeichnet wird. Angestossen wird sie in [`Self::index_erneuern`]
        /// ueber die Laufschleife.
        // SAFETY: Die Signatur ist die einer parameterlosen Methode ohne
        // Rueckgabe, wie `performSelector:withObject:afterDelay:` sie ruft.
        #[unsafe(method(dickeNachziehen))]
        fn dicke_nachziehen(&self) {
            let noetig = self.noetige_dicke();
            if (self.ruleThickness() - noetig).abs() >= 0.5 {
                self.setRuleThickness(noetig);
                self.neu_zeichnen();
            }
        }
    }
);

impl Nummernspalte {
    /// Baut die Spalte und haengt sie in die senkrechte Linealstelle der
    /// Bildlaufansicht.
    ///
    /// **Der eine Weg, auf dem eine Nummernspalte entsteht.** Beide Flaechen
    /// gehen durch ihn: [`super::editor`] beim Bau der Textflaeche und
    /// [`super::vorschau`] beim Bau der Textanzeige. Ob die Spalte danach steht,
    /// entscheidet die Bildlaufansicht ueber `setRulersVisible`; der Editor
    /// zeigt sie immer, die Vorschau nur beim rohen Inhalt einer Textdatei.
    pub fn einhaengen(mtm: MainThreadMarker, rolle: &NSScrollView, flaeche: &NSTextView) {
        let schrift = NSFont::userFixedPitchFontOfSize(NSFont::smallSystemFontSize())
            .unwrap_or_else(|| NSFont::systemFontOfSize(NSFont::smallSystemFontSize()));
        let farbe = NSColor::secondaryLabelColor();
        // SAFETY: Zwei Fremdsymbole von AppKit, die Merkmalsnamen fuer Schrift
        // und Farbe. Sie werden gelesen und nicht geschrieben.
        let schluessel = unsafe { [NSFontAttributeName, NSForegroundColorAttributeName] };
        let werte: [&AnyObject; 2] = [&schrift, &farbe];
        let merkmale = NSDictionary::from_slices(&schluessel, &werte);

        let this = Self::alloc(mtm).set_ivars(NummernspalteIvars {
            flaeche: flaeche.retain(),
            index: RefCell::new(Zeilenindex::neu("")),
            anfaenge: RefCell::new(Vec::new()),
            // Ueberholt von Anfang an: gezaehlt wird beim ersten Zeichnen und
            // nicht hier, damit es genau eine Stelle gibt, die zaehlt.
            ueberholt: Cell::new(true),
            merkmale,
        });
        // SAFETY: `initWithScrollView:orientation:` von NSRulerView hat die
        // hier angenommene Signatur.
        let spalte: Retained<Self> = unsafe {
            msg_send![
                super(this),
                initWithScrollView: rolle,
                orientation: NSRulerOrientation::VerticalRuler,
            ]
        };

        rolle.setHasVerticalRuler(true);
        rolle.setVerticalRulerView(Some(&spalte));
        spalte.setClientView(Some(flaeche));
        spalte.setRuleThickness(spalte.noetige_dicke());
        rolle.setRulersVisible(true);

        let zentrale = NSNotificationCenter::defaultCenter();
        let klemme = rolle.contentView();
        // Ohne diese Zeile verschickt die Klemme beim Blaettern nichts, und die
        // Spalte bliebe beim zuletzt gezeichneten Ausschnitt stehen.
        klemme.setPostsBoundsChangedNotifications(true);
        // SAFETY: `spalte` ist von der Klasse, die die beiden Selektoren mit
        // der Signatur einer Meldungsannahme beantwortet. Beide Beobachter
        // werden in `Drop` wieder abgemeldet, ueberleben die Zentrale also
        // nicht. Dieselbe Form wie die Datentraegerbeobachtung in
        // `super::volumes`.
        unsafe {
            if let Some(speicher) = flaeche.textStorage() {
                zentrale.addObserver_selector_name_object(
                    &spalte,
                    sel!(textGeaendert:),
                    Some(NSTextStorageDidProcessEditingNotification),
                    Some(&speicher),
                );
            }
            zentrale.addObserver_selector_name_object(
                &spalte,
                sel!(sichtVerschoben:),
                Some(NSViewBoundsDidChangeNotification),
                Some(&klemme),
            );
        }
    }

    /// Fordert ein neues Bild an, ohne die Zaehlung fuer ueberholt zu erklaeren.
    ///
    /// **Fuer jeden, der Umbruchbreite oder Schrift der Flaeche aendert.** Ein
    /// solcher Wechsel aendert die Zeilenkaesten des Layoutverwalters, ohne
    /// dass der Textspeicher eine Meldung verschickt; ohne diesen Ruf zeigte die
    /// Spalte danach die Nummern des zuletzt gezeichneten Umbruchs.
    pub fn neu_zeichnen(&self) {
        self.setNeedsDisplay(true);
    }

    /// Baut Zaehlung und Zeilenanfaenge aus dem Text der Flaeche neu.
    fn index_erneuern(&self) {
        let ivars = self.ivars();
        let text = ivars.flaeche.string().to_string();
        let index = Zeilenindex::neu(&text);
        let anfaenge = anfaenge_in_utf16(&text, &index);
        *ivars.index.borrow_mut() = index;
        *ivars.anfaenge.borrow_mut() = anfaenge;
        ivars.ueberholt.set(false);

        if (self.ruleThickness() - self.noetige_dicke()).abs() >= 0.5 {
            // Ueber die Laufschleife und nicht sofort, siehe
            // `dickeNachziehen`. Der Standardmodus genuegt: die Zeilenzahl
            // aendert sich beim Tippen und nicht beim Blaettern, und getippt
            // wird nicht waehrend einer Mausverfolgung.
            let selbst: &NSObject = self;
            // SAFETY: `dickeNachziehen` ist eine Methode dieser Klasse ohne
            // Parameter, und `None` ist das dazu passende Argument.
            unsafe {
                selbst.performSelector_withObject_afterDelay(sel!(dickeNachziehen), None, 0.0)
            };
        }
    }

    /// Die Breite, die die groesste Nummer dieses Textes braucht.
    fn noetige_dicke(&self) -> f64 {
        let stellen = self
            .ivars()
            .index
            .borrow()
            .zeilenzahl()
            .to_string()
            .len()
            .max(MINDESTSTELLEN);
        let probe = NSString::from_str(&"0".repeat(stellen));
        // SAFETY: Die Merkmale sind Schrift und Farbe und damit gueltige
        // Merkmale einer Zeichenkette.
        let groesse = unsafe { probe.sizeWithAttributes(Some(&self.ivars().merkmale)) };
        (groesse.width + 2.0 * RAND).ceil()
    }

    /// Zeichnet die Nummern der sichtbaren Zeilen.
    ///
    /// Der Durchgang laeuft ueber die **Zeilenkaesten** des Layoutverwalters
    /// und nicht ueber die Zeilen der Datei: nur er weiss, wo eine Zeile
    /// gezeichnet steht und wo sie umgelaufen ist. Welche Nummer ein Kasten
    /// traegt, sagt die Suche in den Zeilenanfaengen, und ein Kasten ohne
    /// Treffer ist die Fortsetzung einer umgelaufenen Zeile und bekommt keine.
    fn nummern_zeichnen(&self) {
        let ivars = self.ivars();
        let Some(rolle) = self.scrollView() else {
            return;
        };
        // SAFETY: Beide Eigenschaften der Textflaeche werden nur gelesen; die
        // Flaeche bringt Verwalter und Behaelter selbst mit.
        let (verwalter, behaelter) =
            unsafe { (ivars.flaeche.layoutManager(), ivars.flaeche.textContainer()) };
        let (Some(verwalter), Some(behaelter)) = (verwalter, behaelter) else {
            return;
        };

        if ivars.ueberholt.get() {
            self.index_erneuern();
        }
        let anfaenge = ivars.anfaenge.borrow();

        // Der sichtbare Ausschnitt, aus den Koordinaten der Flaeche in die des
        // Textbehaelters gerueckt.
        let sichtbar = rolle.contentView().bounds();
        let ursprung = ivars.flaeche.textContainerOrigin();
        let ausschnitt = NSRect::new(
            NSPoint::new(
                sichtbar.origin.x - ursprung.x,
                sichtbar.origin.y - ursprung.y,
            ),
            sichtbar.size,
        );
        let glyphenbereich =
            verwalter.glyphRangeForBoundingRect_inTextContainer(ausschnitt, &behaelter);

        // Von der Hoehe im Textbehaelter auf die Hoehe in der Spalte: die
        // Spalte beginnt am oberen Rand des sichtbaren Ausschnitts.
        let hoehenversatz = ursprung.y - sichtbar.origin.y;
        let dicke = self.ruleThickness();

        let mut glyphe = glyphenbereich.location;
        let ende = glyphenbereich.location + glyphenbereich.length;
        while glyphe < ende {
            let mut wirkung = NSRange::new(0, 0);
            // SAFETY: `wirkung` zeigt auf einen gueltigen NSRange auf dem
            // Stapel, und `glyphe` liegt im gelieferten Glyphenbereich.
            let kasten = unsafe {
                verwalter.lineFragmentRectForGlyphAtIndex_effectiveRange(glyphe, &mut wirkung)
            };
            if wirkung.length == 0 {
                // Ohne diese Zeile liefe die Schleife bei einem leeren Kasten
                // endlos; sie ist die Abbruchbedingung und keine Vorsicht.
                break;
            }
            // Von der Glyphe auf das Zeichen: die Zeilenanfaenge stehen in
            // Zeichenstellen, weil der Zeilenindex sie so liefert.
            let zeichenstelle = verwalter.characterIndexForGlyphAtIndex(wirkung.location);
            if let Ok(gefunden) = anfaenge.binary_search(&zeichenstelle) {
                self.nummer_zeichnen(gefunden + 1, kasten, hoehenversatz, dicke);
            }
            glyphe = wirkung.location + wirkung.length;
        }

        // Die leere letzte Zeile eines Textes, der auf einem Umbruch endet.
        // Der Layoutverwalter fuehrt sie als zusaetzlichen Kasten ausserhalb
        // des Glyphenbereichs, und der Index kennt sie als letzte Zeile; ohne
        // diesen Zweig stuende die Schreibmarke am Dateiende neben keiner
        // Nummer. Der leere Text geht denselben Weg und traegt die 1.
        if verwalter.extraLineFragmentTextContainer().is_some() {
            self.nummer_zeichnen(
                anfaenge.len(),
                verwalter.extraLineFragmentRect(),
                hoehenversatz,
                dicke,
            );
        }
    }

    /// Zeichnet eine Nummer rechtsbuendig neben ihren Zeilenkasten.
    fn nummer_zeichnen(&self, nummer: usize, kasten: NSRect, hoehenversatz: f64, dicke: f64) {
        let text = NSString::from_str(&nummer.to_string());
        let merkmale = &self.ivars().merkmale;
        // SAFETY: Die Merkmale sind Schrift und Farbe und damit gueltige
        // Merkmale einer Zeichenkette. Gezeichnet wird in den Zeichenbereich,
        // den AppKit fuer diesen Durchgang aufgespannt hat.
        unsafe {
            let groesse = text.sizeWithAttributes(Some(merkmale));
            let stelle = NSPoint::new(
                dicke - RAND - groesse.width,
                kasten.origin.y + hoehenversatz + (kasten.size.height - groesse.height) / 2.0,
            );
            text.drawAtPoint_withAttributes(stelle, Some(merkmale));
        }
    }
}

impl Drop for Nummernspalte {
    /// Meldet die beiden Beobachter aus [`Nummernspalte::einhaengen`] wieder ab.
    fn drop(&mut self) {
        let zentrale = NSNotificationCenter::defaultCenter();
        let selbst: &AnyObject = &*self;
        // SAFETY: `selbst` ist der Beobachter, den `einhaengen` fuer die beiden
        // Meldungen angemeldet hat. Ohne Gegenstand nimmt die Zentrale ihn fuer
        // die genannte Meldung heraus, gleich bei welchem Absender er
        // eingetragen war. Keiner der beiden Aufrufe haelt `selbst` fest oder
        // ruft eine ueberschriebene Methode.
        unsafe {
            zentrale.removeObserver_name_object(
                selbst,
                Some(NSTextStorageDidProcessEditingNotification),
                None,
            );
            zentrale.removeObserver_name_object(
                selbst,
                Some(NSViewBoundsDidChangeNotification),
                None,
            );
        }
    }
}

/// Laesst die Spalte einer Bildlaufansicht neu zeichnen, falls sie eine traegt.
///
/// **Der Weg von aussen**, und der einzige. [`Nummernspalte::neu_zeichnen`]
/// verlangt die Spalte selbst, und die haelt niemand ausser der
/// Bildlaufansicht: [`Nummernspalte::einhaengen`] gibt sie nicht heraus, damit
/// es keinen zweiten Halter gibt. Wer sie braucht, hat die Bildlaufansicht zur
/// Hand und bekommt sie hier.
///
/// **Der Aufrufer ist, wer Umbruchbreite oder Schrift der Flaeche aendert.** Das
/// ist seit S33 der Ansichtswechsel des Editors: er aendert beides, und der
/// Textspeicher verschickt dabei keine Meldung, an der die Spalte den Wechsel
/// bemerken koennte. Ohne diesen Ruf zeigte die Formatansicht die Nummern des
/// zuletzt gezeichneten Umbruchs, und das fuenfte Abnahmekriterium von C10
/// waere gebrochen.
///
/// Traegt die Ansicht keine Spalte oder eine fremde, geschieht nichts. Der Fall
/// ist im Programm nicht erreichbar — beide Flaechen haengen die eine Klasse
/// ein —, und es gibt nichts zu melden: eine Ansicht ohne Zeilennummern hat
/// keine, die falsch stehen koennten.
pub fn spalte_neu_zeichnen(rolle: &NSScrollView) {
    let Some(spalte) = rolle.verticalRulerView() else {
        return;
    };
    let spalte: &AnyObject = &spalte;
    if let Some(spalte) = spalte.downcast_ref::<Nummernspalte>() {
        spalte.neu_zeichnen();
    }
}

/// Dieselben Zeilenanfaenge wie im [`Zeilenindex`], in AppKits Koordinate.
///
/// **Ein Koordinatenwechsel und keine zweite Zaehlung.** Welche Stellen
/// Zeilenanfang sind, sagt allein der Index; gerechnet wird in
/// [`super::koordinaten::in_utf16`], der einen Stelle des Programms, die
/// zwischen den beiden Koordinaten wechselt. Der Text ist derselbe, ueber den
/// der Index gelaufen ist, und jeder Zeilenanfang liegt deshalb auf einer
/// Zeichengrenze; die Anfaenge kommen aufsteigend, wie die Rechnung es
/// verlangt.
fn anfaenge_in_utf16(text: &str, index: &Zeilenindex) -> Vec<usize> {
    let byteanfaenge: Vec<usize> = (1..=index.zeilenzahl())
        .map(|nummer| index.anfang_der_zeile(nummer).versatz)
        .collect();
    koordinaten::in_utf16(text, &byteanfaenge)
}

/// Die Zaehlung der Zeilenanfaenge braucht kein Fenster; deshalb steht ihre
/// Pruefung hier und nicht unter `Nutzerarbeit`. Der Koordinatenwechsel selbst
/// ist in [`super::koordinaten`] geprueft.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn der_leere_text_hat_einen_zeilenanfang_und_er_liegt_bei_null() {
        let text = "";
        let anfaenge = anfaenge_in_utf16(text, &Zeilenindex::neu(text));
        assert_eq!(anfaenge, vec![0]);
    }

    #[test]
    fn ohne_zeichen_ausserhalb_von_ascii_sind_beide_koordinaten_gleich() {
        let text = "eins\nzwei\ndrei";
        let index = Zeilenindex::neu(text);
        let anfaenge = anfaenge_in_utf16(text, &index);
        assert_eq!(anfaenge, vec![0, 5, 10]);
        assert_eq!(anfaenge.len(), index.zeilenzahl());
    }

    /// Der Grund, aus dem der Wechsel ueberhaupt stattfindet: ein Umlaut kostet
    /// zwei Bytes und eine UTF-16-Einheit, ein Bildzeichen vier Bytes und zwei.
    #[test]
    fn umlaute_und_bildzeichen_verschieben_die_beiden_koordinaten_gegeneinander() {
        let text = "Äpfel\n🍎🍎\nEnde";
        let index = Zeilenindex::neu(text);
        // In Bytes: 6 fuer "Äpfel\n", danach 9 fuer die beiden Bildzeichen und
        // den Umbruch.
        assert_eq!(index.anfang_der_zeile(2).versatz, 7);
        assert_eq!(index.anfang_der_zeile(3).versatz, 16);
        // In UTF-16: fuenf Einheiten fuer "Äpfel", eine fuer den Umbruch, dann
        // zwei je Bildzeichen und wieder eine fuer den Umbruch.
        assert_eq!(anfaenge_in_utf16(text, &index), vec![0, 6, 11]);
    }

    /// Die leere letzte Zeile nach einem abschliessenden Umbruch: der Index
    /// kennt sie, und sie liegt am Textende.
    #[test]
    fn die_leere_letzte_zeile_liegt_am_textende() {
        let text = "eins\n";
        let index = Zeilenindex::neu(text);
        let anfaenge = anfaenge_in_utf16(text, &index);
        assert_eq!(index.zeilenzahl(), 2);
        assert_eq!(anfaenge, vec![0, 5]);
    }
}
