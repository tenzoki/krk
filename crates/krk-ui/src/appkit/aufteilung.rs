//! Die Fensterzeile: eine `NSSplitView` mit sechs Bereichen.
//!
//! ```text
//! ┌───────────┬──────────────┬──────────────┬──────────┬────────┬────────┐
//! │ Lesezei-  │ Dateifenster │ Dateifenster │ Vorschau │ Editor │ Git    │
//! │ chen (C5) │ links        │ rechts       │ (C6)     │        │        │
//! └───────────┴──────────────┴──────────────┴──────────┴────────┴────────┘
//! ```
//!
//! Die drei rechten Bereiche teilen sich denselben Platz: Editor und
//! Git-Bereich nehmen die Stelle der Vorschau ein, und C1 der Editor-Runde wie
//! C1 der Git-Runde sagen zu, dass nie zwei von ihnen zugleich zu sehen sind.
//! Die Regel dazu wohnt in [`crate::fenstermodell`] und nicht hier; dieses
//! Modul verteilt Breiten und Sichtbarkeit und faellt keine Entscheidung
//! darueber, welcher Bereich steht.
//!
//! **Jeder der sechs Bereiche steht in einem `NSBox`**, und dessen Rahmen ist
//! die Anzeige aus C9: er sagt, welcher Bereich die Tasten annimmt, und
//! daneben, welches Dateifenster das aktive ist. Die Anzeige haengt am Rahmen
//! und nicht am Inhalt, damit sie auch dann eindeutig ist, wenn beide
//! Dateifenster denselben Ordner zeigen; und sie ist fuer alle sechs dieselbe
//! Form, weil nicht jeder von ihnen eine Auswahl hat, an der sich eine
//! Auswahlfarbe zeigen liesse.
//!
//! Bis zum 260809 trugen allein die beiden Dateifenster einen Kasten, und die
//! Frage "traegt dieser Bereich einen Rahmen?" hatte zwei Antworten. Jetzt hat
//! sie eine, und was die Kaesten unterscheidet, ist allein ihre Farbe:
//! [`crate::kommandos::fokus::rahmenrolle`] entscheidet sie, ausserhalb von
//! `appkit` und ohne Fenster pruefbar, und [`rahmenfarbe`] setzt jede Rolle in
//! eine Systemfarbe um. Der Preis ist benannt und klein: zwei Punkte Rahmen
//! nehmen jedem der drei Randbereiche vier Punkte Inhaltsbreite, und die
//! Mindestbreiten aus [`Bereich::mindestbreite`] sind an der Flaeche gerechnet
//! und nicht am Inhalt.
//!
//! Die Lesezeichenleiste steht seit Schritt 18 als eigener Bereich darin und
//! kommt fertig von [`super::leiste`] herein; das Vorschaufenster kommt seit
//! Schritt 19 ebenso fertig von [`super::vorschau`], der Editor seit Schritt 16
//! der Editor-Runde von [`super::editor`]. Breite und Sichtbarkeit aller drei
//! gehoeren zu C7 und damit hierher.
//!
//! # Wo die Breiten herkommen
//!
//! Die Regel steht **einmal**, in [`crate::fenstermodell::bereichsbreiten`],
//! und sie ist reines Rust ohne AppKit. Dieses Modul ruft sie an zwei Stellen:
//! wenn das Fenstermodell eine Breite oder eine Sichtbarkeit geaendert hat, und
//! wenn AppKit die Bereiche neu auslegen laesst, etwa weil der Nutzer das
//! Fenster groesser zieht.
//!
//! **Womit gerechnet wird, steht im Delegierten und nicht in den Rahmen der
//! Unteransichten**; der Grund steht an [`AufteilungsIvars::wuensche`]. Der
//! erste Fall traegt den Wunsch des Fenstermodells dort ein, der zweite
//! uebernimmt eine mit der Maus verschobene Trennlinie. Damit ueberlebt eine
//! solche Ziehbewegung die naechste Fenstergroessenaenderung, ohne dass eine
//! zweite Rechenvorschrift daneben entstuende — und ohne dass ein Zug am
//! Fensterrand die Aufteilung des Nutzers durch das Verhaeltnis der
//! Mindestbreiten ersetzte.
//!
//! **Welche Bereiche stehen, kommt in beiden Faellen aus den Unteransichten**
//! und nie aus dem Modell. Der erste Fall schreibt den Wunsch des Modells
//! vorher hinein und liest ihn dann von dort zurueck. Das ist ein Umweg von
//! einer Zeile und der Preis dafuer, dass die Frage nur eine Antwort hat:
//! [`steht_im`].
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSSplitView`, `NSBox`, `NSView`, `NSColor` und `NSObject` stehen seit macOS
//! 10.0 zur Verfuegung, ebenso das Protokoll `NSSplitViewDelegate` mit den drei
//! hier bedienten Methoden `splitView:constrainMinCoordinate:ofSubviewAt:`,
//! `splitView:constrainMaxCoordinate:ofSubviewAt:` und
//! `splitView:resizeSubviewsWithOldSize:`. Sechs Beruehrungen sind juenger als
//! ihre Klasse: die Sorte `NSBoxCustom` und die drei Merkmale, die allein sie
//! zeichnen laesst — `borderWidth`, `borderColor` und `fillColor` — seit 10.5,
//! und die beiden Semantikfarben `controlAccentColor` und `separatorColor` seit
//! 10.14. Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen
//! ist nach macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei
//! braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt
//! keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die
//! Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.

use std::cell::Cell;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSBox, NSBoxType, NSColor, NSSplitView, NSSplitViewDelegate,
    NSTitlePosition, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSInteger, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
};

use krk_core::ablage::{Breiten, Fensterseite, Sichtbarkeit};

use crate::fenstermodell::{Bereich, Zeilenmass, sichtbar_in, wuensche_nachfuehren};
use crate::kommandos::fokus::{Fokus, Rahmenrolle, rahmenrolle};

use super::tabelle::Dateifenster;
use super::tableiste;

/// Die Breite des Rahmens, der die Anzeige aus C9 traegt.
const RAHMENBREITE: f64 = 2.0;

/// Wie stark die Akzentfarbe zurueckgenommen wird, wo sie nicht voll gilt.
///
/// Zwei Faelle nehmen sie: das aktive Dateifenster ohne Fokus, und jeder
/// Bereich, solange das Fenster im Hintergrund steht. Beide sollen sichtbar
/// bleiben und dem Bereich mit dem Fokus nicht den Rang ablaufen; das achte
/// Abnahmekriterium von C9 verlangt fuer den Hintergrund ausdruecklich
/// zuruecktreten und nicht verschwinden.
const ZURUECKGETRETEN: f64 = 0.4;

/// Die Groesse, mit der ein Bereich entsteht, bevor die Aufteilung ihn auslegt.
const AUFBAUGROESSE: NSSize = NSSize::new(400.0, 400.0);

/// Was der Delegierte der Aufteilung haelt.
pub struct AufteilungsIvars {
    /// Die Wuensche, aus denen die Zeile zuletzt ausgelegt wurde.
    ///
    /// **Die eine Stelle, an der die Wuensche auf der AppKit-Seite stehen.**
    /// Bis zum 260812 standen sie in den Rahmen der Unteransichten selbst: das
    /// Auslegen las sie von dort und schrieb sie dorthin zurueck. Der Rahmen
    /// ist dafuer aber kein verlustfreier Speicher — haengt ein Bereich an
    /// seinem Mindestmass, traegt sein Rahmen die Deckelung und nicht mehr den
    /// Wunsch —, und ein Zug am Fensterrand und zurueck loeschte damit die
    /// Aufteilung des Nutzers.
    ///
    /// Geschrieben wird das Feld an genau zwei Stellen, und beide sind eine
    /// Antwort auf "wer hat den Wunsch geaendert": [`Aufteilung::anwenden`]
    /// traegt den des Fenstermodells ein, und das Auslegen nach einer
    /// Groessenaenderung uebernimmt eine mit der Maus verschobene Trennlinie.
    /// Welche der beiden Zahlen gilt, entscheidet
    /// [`wuensche_nachfuehren`](crate::fenstermodell::wuensche_nachfuehren) und
    /// nicht diese Datei.
    ///
    /// **Kein Rueckweg in das Fenstermodell und kein Ring.** Der Delegierte
    /// haelt einen Wert und keine Sicht auf das Modell; das Modell erfaehrt von
    /// einer Ziehbewegung weiterhin nur, wenn jemand nachmisst.
    pub wuensche: Cell<Breiten>,
}

define_class!(
    /// Der Delegierte der Aufteilung: Mindestbreiten und das Auslegen.
    ///
    /// Er haelt die Wuensche, aus denen die Zeile ausgelegt wird (siehe
    /// [`AufteilungsIvars::wuensche`]). Alles uebrige steht in der
    /// `NSSplitView`, die AppKit ihm bei jedem Aufruf mitgibt: die Rahmen der
    /// fuenf Bereiche und ihre Sichtbarkeit.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = AufteilungsIvars]
    pub struct AufteilungsDelegierter;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for AufteilungsDelegierter {}

    // SAFETY: `NSSplitViewDelegate` stellt keine Bedingungen.
    unsafe impl NSSplitViewDelegate for AufteilungsDelegierter {
        /// Wie weit sich eine Trennlinie nach links ziehen laesst.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(splitView:constrainMinCoordinate:ofSubviewAt:))]
        fn mindestlage(&self, teiler: &NSSplitView, _vorschlag: f64, trennlinie: NSInteger) -> f64 {
            let Ok(trennlinie) = usize::try_from(trennlinie) else {
                return 0.0;
            };
            grenze_links(teiler, trennlinie)
        }

        /// Wie weit sich eine Trennlinie nach rechts ziehen laesst.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(splitView:constrainMaxCoordinate:ofSubviewAt:))]
        fn hoechstlage(&self, teiler: &NSSplitView, _vorschlag: f64, trennlinie: NSInteger) -> f64 {
            let Ok(trennlinie) = usize::try_from(trennlinie) else {
                return 0.0;
            };
            grenze_rechts(teiler, trennlinie)
        }

        /// AppKit laesst die Bereiche neu auslegen.
        ///
        /// **Die gemessenen Breiten werden nicht mehr blind als Wuensche
        /// eingespeist.** Sie tragen einen Wunsch nur dann, wenn jemand eine
        /// Trennlinie mit der Maus verschoben hat; sonst stehen dort die Zahlen,
        /// die das letzte Auslegen selbst hingeschrieben hat, und die
        /// zurueckzulesen ist nur ohne Deckelung neutral. Die Entscheidung
        /// faellt in
        /// [`wuensche_nachfuehren`](crate::fenstermodell::wuensche_nachfuehren).
        ///
        /// **`alte_groesse` ist dafuer tragend**: die gemessenen Breiten sind
        /// unter der **alten** Zeilenbreite entstanden, und nur an ihr gemessen
        /// laesst sich sagen, ob sie von der Regel stammen. Die Trennerbreite
        /// aendert sich dabei nicht und kommt wie immer aus [`zeilenmass`].
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(splitView:resizeSubviewsWithOldSize:))]
        fn neu_auslegen(&self, teiler: &NSSplitView, alte_groesse: NSSize) {
            let altes_mass = Zeilenmass {
                gesamt: alte_groesse.width,
                trennerbreite: zeilenmass(teiler).trennerbreite,
            };
            let wuensche = wuensche_nachfuehren(
                self.ivars().wuensche.get(),
                gemessene_breiten(teiler),
                altes_mass,
                &gemessene_sichtbarkeit(teiler),
            );
            self.ivars().wuensche.set(wuensche);
            auslegen(teiler, &wuensche);
        }
    }
);

impl AufteilungsDelegierter {
    fn neu(mtm: MainThreadMarker) -> Retained<Self> {
        // Ohne gesetzte Breite: bis das Fenstermodell seine erste sagt, gelten
        // die Anfangsbreiten aus `Bereich::anfangsbreite`.
        let this = Self::alloc(mtm).set_ivars(AufteilungsIvars {
            wuensche: Cell::new(Breiten::default()),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Merkt sich die Wuensche, aus denen gerade ausgelegt wird.
    fn wuensche_merken(&self, breiten: &Breiten) {
        self.ivars().wuensche.set(*breiten);
    }
}

/// Die aufgebaute Fensterzeile.
pub struct Aufteilung {
    teiler: Retained<NSSplitView>,
    /// `NSSplitView` haelt seinen Delegierten schwach; hier steht er stark.
    ///
    /// Er haelt daneben die Wuensche, aus denen ausgelegt wird, und deshalb
    /// spricht [`Aufteilung::anwenden`] ihn an.
    delegierter: Retained<AufteilungsDelegierter>,
    /// Die Kaesten aller sechs Bereiche, in der Reihenfolge von
    /// [`Bereich::ALLE`].
    ///
    /// # Was die Feldbreite haelt, und was sie nicht haelt
    ///
    /// **Sie haelt nichts.** Das Feld entsteht in [`Aufteilung::bauen`] aus
    /// einem Literal mit sechs Gliedern und nicht aus `Bereich::ALLE.map(…)`;
    /// die Zahl `6` steht damit im Quelltext und folgt nicht aus der
    /// Aufzaehlung. Ein siebter Bereich, den jemand hinzufuegte, ohne dieses
    /// Literal zu erweitern, uebersetzte anstandslos und liefe beim Start auf
    /// `index out of bounds`, sobald [`Aufteilung::rahmen_setzen`] ueber
    /// `Bereich::ALLE` griffe. Anders als bei
    /// `Bereichsleiste::bereichsschalter`, dessen Feld ueber
    /// `Bereich::ALLE.map(…)` entsteht und dessen Laenge deshalb der Bau
    /// haelt.
    rahmen: [Retained<NSBox>; 6],
}

impl Aufteilung {
    /// Baut die sechs Bereiche um die beiden Dateifenster, die Leiste, die
    /// Vorschau, den Editor und den Git-Bereich.
    ///
    /// Leiste, Vorschau, Editor und Git-Bereich kommen fertig herein und werden
    /// hier nicht gebaut: alle vier sind eigene fokussierbare Bereiche mit
    /// eigenem Inhalt, und dieses Modul verteilt Breiten und Sichtbarkeit.
    /// Dieselbe Aufgabenteilung wie bei den beiden Dateifenstern.
    pub fn bauen(
        mtm: MainThreadMarker,
        dateifenster: [&Dateifenster; 2],
        leiste: &NSView,
        vorschau: &NSView,
        editor: &NSView,
        git: &NSView,
    ) -> Self {
        let teiler = NSSplitView::initWithFrame(
            NSSplitView::alloc(mtm),
            NSRect::new(NSPoint::ZERO, AUFBAUGROESSE),
        );
        teiler.setVertical(true);
        teiler.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        // Alle sechs gehen durch dieselbe Funktion, seit C9 die Anzeige auf
        // alle Bereiche ausdehnt. Die beiden Dateifenster bringen ihren Inhalt
        // nicht fertig mit, sondern in drei Stuecken; `dateifensterinhalt`
        // legt sie uebereinander, und eingerahmt wird danach wie bei den
        // uebrigen vier.
        //
        // **Die Reihenfolge dieses Literals ist die von `Bereich::ALLE`, und
        // der Uebersetzer haelt das nicht**; die Warnung dazu steht an
        // `Aufteilung::rahmen`.
        let rahmen = [
            gerahmt(mtm, leiste),
            gerahmt(mtm, &dateifensterinhalt(mtm, dateifenster[0])),
            gerahmt(mtm, &dateifensterinhalt(mtm, dateifenster[1])),
            gerahmt(mtm, vorschau),
            gerahmt(mtm, editor),
            gerahmt(mtm, git),
        ];
        // Die Reihenfolge ist die von `Bereich::ALLE` und die einzige, in der
        // die Rechenvorschrift der Breiten die Bereiche wiederfindet.
        for bereich in Bereich::ALLE {
            teiler.addSubview(&rahmen[bereich.index()]);
        }

        let delegierter = AufteilungsDelegierter::neu(mtm);
        // `NSSplitView.setDelegate:` ist eine sichere Bindung; unsicher ist
        // allein, den Delegierten fallen zu lassen, solange die Aufteilung
        // steht. `NSSplitView.delegate` ist eine schwache Eigenschaft ("This is
        // a weak property",
        // `objc2-app-kit-0.3.2/src/generated/NSSplitView.rs:129-137`), deshalb
        // haelt `Aufteilung` ihn selbst fest.
        teiler.setDelegate(Some(ProtocolObject::from_ref(&*delegierter)));

        Self {
            teiler,
            delegierter,
            rahmen,
        }
    }

    /// Die Ansicht, die in das Fenster gehaengt wird.
    pub fn sicht(&self) -> &NSView {
        &self.teiler
    }

    /// Blendet die Bereiche ein und aus und legt sie neu aus.
    ///
    /// Die gewuenschte Sichtbarkeit wird hier **in die Ansichten geschrieben**
    /// und nicht an [`auslegen`] weitergereicht. Das ist der Grund, aus dem
    /// beide Wege in die Aufteilung dasselbe Bild ergeben; er steht bei
    /// [`steht_im`] ausgeschrieben.
    ///
    /// **Die Breiten des Modells werden dabei zum gehaltenen Wunsch.** Das
    /// naechste Auslegen nach einer Groessenaenderung rechnet aus ihnen weiter,
    /// statt die Zahlen zurueckzulesen, die diese Zeile eben in die Rahmen
    /// geschrieben hat; der Grund steht an [`AufteilungsIvars::wuensche`].
    pub fn anwenden(&self, breiten: &Breiten, sichtbar: &Sichtbarkeit) {
        for bereich in Bereich::ALLE {
            if let Some(ansicht) = bereichsansicht(&self.teiler, bereich.index()) {
                ansicht.setHidden(!sichtbar_in(sichtbar, bereich));
            }
        }
        self.delegierter.wuensche_merken(breiten);
        auslegen(&self.teiler, breiten);
    }

    /// Das Mass der Fensterzeile: ihre Breite und die einer Trennlinie.
    ///
    /// **Der Weg, auf dem die Geometrie der Zeile in das Fenstermodell kommt.**
    /// Es kennt AppKit nicht und kann die beiden Zahlen nicht erfragen; jeder
    /// Aufruf, dessen Antwort an ihnen haengt, bekommt sie deshalb als Wert
    /// mitgegeben — die Abweisung an den Mindestbreiten in
    /// [`Fenstermodell::umschalten`](crate::fenstermodell::Fenstermodell::umschalten)
    /// und der Massstab in
    /// [`Fenstermodell::breite_aendern`](crate::fenstermodell::Fenstermodell::breite_aendern).
    ///
    /// Gelesen wird ueber [`zeilenmass`], also durch dieselbe eine Stelle, aus
    /// der auch [`auslegen`] die beiden Zahlen nimmt.
    pub fn zeilenmass(&self) -> Zeilenmass {
        zeilenmass(&self.teiler)
    }

    /// Die Breiten, die gerade auf dem Schirm stehen.
    ///
    /// Der Weg, auf dem eine mit der Maus verschobene Trennlinie in die Sitzung
    /// kommt: sie steht in den Rahmen der Ansichten und nirgends sonst.
    ///
    /// # Was die Feldbreite haelt, und was sie nicht haelt
    ///
    /// **Sie haelt nichts.** `[0.0; 6]` ist eine Zahl im Quelltext, und die
    /// Schleife darunter greift ueber [`Bereich::index`] hinein. Ein siebter
    /// Bereich, der diese Zahl nicht erhoehte, uebersetzte und liefe zur
    /// Laufzeit auf `index out of bounds`. Auch die Gegenseite haelt nichts:
    /// `Fenstermodell::breiten_uebernehmen` nimmt `[f64; 6]`, und beide Seiten
    /// blieben stumm bei fuenf.
    pub fn gemessene_breiten(&self) -> [f64; 6] {
        let mut breiten = [0.0; 6];
        for bereich in Bereich::ALLE {
            if let Some(ansicht) = bereichsansicht(&self.teiler, bereich.index()) {
                breiten[bereich.index()] = ansicht.frame().size.width;
            }
        }
        breiten
    }

    /// Die Wurzelansicht eines Bereichs, falls die Aufteilung sie schon traegt.
    ///
    /// **Der Preis der Enthaltensfrage aus S43, und er faellt genau einmal
    /// an.** `Anwendungsdelegierter::fokus` fragt seit dem 260809 nicht mehr,
    /// welche eine Ansicht den Ersthelferrang traegt, sondern in welchem
    /// Teilbaum er liegt; dafuer muss die Wurzel jedes Bereichs nach
    /// aussen. Sie liegt bereits vor, naemlich als Unteransicht der Aufteilung
    /// an der Stelle [`Bereich::index`], und deshalb entsteht hier keine zweite
    /// Aufzaehlung neben [`Bereich::ALLE`].
    ///
    /// Die Teilbaeume sind zueinander fremd, weil es die Unteransichten einer
    /// `NSSplitView` sind; ein Ersthelfer liegt in hoechstens einem.
    pub fn bereichssicht(&self, bereich: Bereich) -> Option<Retained<NSView>> {
        bereichsansicht(&self.teiler, bereich.index())
    }

    /// Faerbt die Rahmen aller sechs Bereiche (C9).
    ///
    /// **Ein Schreiber, drei Angaben, keine Entscheidung.** Welche Rolle ein
    /// Bereich traegt, rechnet [`rahmenrolle`] ausserhalb von `appkit`; welche
    /// Farbe eine Rolle bekommt, sagt [`rahmenfarbe`]. Diese Funktion setzt sie
    /// und faellt selbst keine Fallunterscheidung.
    ///
    /// Sie loest `aktives_markieren` ab, das bis zum 260809 zwei Kaesten nach
    /// der aktiven Seite einfaerbte. Der Unterschied ist nicht die Zahl der
    /// Kaesten, sondern die Frage: gefaerbt wird jetzt nach dem Fokus, und das
    /// aktive Dateifenster steht daneben.
    ///
    /// `im_vordergrund` ist `isKeyWindow` des Hauptfensters. Steht es im
    /// Hintergrund, tritt auch die volle Akzentfarbe zurueck, statt zu
    /// verschwinden; das achte Abnahmekriterium von C9 verlangt genau das, und
    /// macOS haelt es fuer jede Auswahl so.
    pub fn rahmen_setzen(&self, fokus: Fokus, aktiv: Fensterseite, im_vordergrund: bool) {
        for bereich in Bereich::ALLE {
            let farbe = rahmenfarbe(rahmenrolle(bereich, fokus, aktiv), im_vordergrund);
            self.rahmen[bereich.index()].setBorderColor(&farbe);
        }
    }
}

/// Die Systemfarbe zu einer Rahmenrolle (C9).
///
/// **Drei Systemfarben und keine eigene Tafel.** Dass die Anzeige dem
/// Erscheinungsbild von Hell und Dunkel ohne Zutun folgt, faellt daraus an;
/// dieselbe Begruendung wie in [`super::leiste`] und [`super::tableiste`], wo
/// steht, warum KRK das Erscheinungsbild nicht selbst nachbaut.
///
/// Die zurueckgetretene Fassung ist dieselbe Akzentfarbe mit verringerter
/// Deckkraft und keine zweite Farbe: nur so bleibt erkennbar, dass beide
/// dasselbe meinen.
fn rahmenfarbe(rolle: Rahmenrolle, im_vordergrund: bool) -> Retained<NSColor> {
    match rolle {
        Rahmenrolle::Fokussiert if im_vordergrund => NSColor::controlAccentColor(),
        // Beide Faelle nehmen dieselbe Farbe zurueckgenommen: das aktive
        // Dateifenster ohne Fokus, und der fokussierte Bereich eines Fensters
        // im Hintergrund.
        Rahmenrolle::Fokussiert | Rahmenrolle::AktivOhneFokus => {
            NSColor::controlAccentColor().colorWithAlphaComponent(ZURUECKGETRETEN)
        }
        Rahmenrolle::Ruhig => NSColor::separatorColor(),
    }
}

/// Setzt eine fertige Ansicht in einen Kasten mit farbigem Rahmen.
///
/// **Die eine Stelle, an der ein Bereich seinen Rahmen bekommt**, seit C9 alle
/// fuenf einen tragen. Die Farbe bleibt hier die einer gewoehnlichen
/// Trennlinie; wer sie setzt, ist [`Aufteilung::rahmen_setzen`], und zwar beim
/// ersten Nachzug des Aufbaus.
fn gerahmt(mtm: MainThreadMarker, inhalt: &NSView) -> Retained<NSBox> {
    let kasten = NSBox::initWithFrame(NSBox::alloc(mtm), NSRect::new(NSPoint::ZERO, AUFBAUGROESSE));
    kasten.setBoxType(NSBoxType::Custom);
    kasten.setTitlePosition(NSTitlePosition::NoTitle);
    kasten.setBorderWidth(RAHMENBREITE);
    kasten.setBorderColor(&NSColor::separatorColor());
    kasten.setFillColor(&NSColor::clearColor());
    kasten.setContentViewMargins(NSSize::ZERO);
    kasten.setContentView(Some(inhalt));
    // Keine Autogroesse am Kasten: die Rahmen der fuenf Unteransichten setzt
    // `auslegen` bei jeder Groessenaenderung selbst, und der Kasten legt seine
    // Inhaltsansicht danach von sich aus neu aus. Dieselbe Wahl wie bei den
    // beiden Dateifenster-Kaesten bis zum 260809.
    kasten
}

/// Legt Tableiste und Dateiliste eines Dateifensters uebereinander.
///
/// Von oben nach unten: die Leiste am Kopf, die Liste darunter bis zum Fuss.
/// Die Autogroessen halten die Aufteilung fest, wenn der Nutzer die Trennlinie
/// verschiebt: die Leiste haengt oben, und die Liste nimmt, was darunter frei
/// wird.
///
/// **Die Statuszeile steht seit der Runde 6 nicht mehr darin**, und die
/// Dateiliste verliert dabei keine Hoehe (C5.4). Vorher mass sie
/// `H − Bereichsleiste − Tableiste − eigene Statuszeile`, jetzt misst sie
/// `H − Bereichsleiste − Statuszeile − Tableiste`: die Zeile ist eine Ansicht
/// weiter oben eingehaengt, in `super::fenster::fensterinhalt`, und was sie der
/// Fensterzeile nimmt, gibt sie diesem Dateifenster zurueck. Was die drei
/// Bereiche ohne eigene Zeile dabei verlieren, holt `MINDESTGROESSE` dort
/// nach.
///
/// **Das Einrahmen steht nicht mehr darin.** Bis zum 260809 hiess diese
/// Funktion `gerahmtes_dateifenster` und tat beides; seit alle fuenf Bereiche
/// einen Kasten tragen, ist das Einrahmen [`gerahmt`] und gilt fuer alle. Die
/// drei Randbereiche kommen fertig herein und brauchen diese Haelfte nicht.
fn dateifensterinhalt(mtm: MainThreadMarker, dateifenster: &Dateifenster) -> Retained<NSView> {
    let inhalt = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::ZERO, AUFBAUGROESSE),
    );
    inhalt.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
    );

    let hoehe = AUFBAUGROESSE.height;
    let breite = AUFBAUGROESSE.width;

    let leiste = dateifenster.tableiste_sicht();
    leiste.setFrame(NSRect::new(
        NSPoint::new(0.0, hoehe - tableiste::HOEHE),
        NSSize::new(breite, tableiste::HOEHE),
    ));
    inhalt.addSubview(&leiste);

    let liste = dateifenster.sicht();
    liste.setFrame(NSRect::new(
        NSPoint::ZERO,
        NSSize::new(breite, hoehe - tableiste::HOEHE),
    ));
    inhalt.addSubview(liste);

    inhalt
}

/// Die Ansicht eines Bereichs, falls die Aufteilung sie schon traegt.
fn bereichsansicht(teiler: &NSSplitView, stelle: usize) -> Option<Retained<NSView>> {
    let ansichten = teiler.subviews();
    (stelle < ansichten.len()).then(|| ansichten.objectAtIndex(stelle))
}

/// Ob die Aufteilung den Bereich traegt **und** zeigt.
///
/// **Die eine Stelle, an der dieses Modul beantwortet, welche Bereiche im
/// Fenster stehen.** Vier Aufrufer haengen daran: [`gemessene_sichtbarkeit`],
/// [`auslegen`] und die beiden Grenzen der Trennlinien. Bis zum 260809 stand
/// derselbe Ausdruck dreimal ausgeschrieben da, und [`auslegen`] fragte als
/// vierte statt der Ansichten das Modell — mit einer anderen Antwort, siehe den
/// Kommentar dort.
///
/// Der `is_some`-Teil ist keine Vorsichtsmassnahme, sondern die Aussage selbst:
/// ein Bereich, dessen Unteransicht die Aufteilung nicht traegt, steht nicht im
/// Fenster, gleich was das Modell ueber ihn sagt. Er traf bis Schritt 16 der
/// Editor-Runde den Editor; seit dessen fuenfter Unteransicht trifft er keinen
/// Bereich mehr, und die Antwort haengt fuer alle fuenf allein an `isHidden`.
fn steht_im(teiler: &NSSplitView, bereich: Bereich) -> bool {
    bereichsansicht(teiler, bereich.index()).is_some_and(|ansicht| !ansicht.isHidden())
}

/// Die Breiten, die gerade auf dem Schirm stehen.
///
/// Ein Bereich, dessen Unteransicht die Aufteilung nicht traegt, liefert `None`
/// und behaelt damit seine gespeicherte Breite. Seit Schritt 16 der
/// Editor-Runde stehen alle Unteransichten; `None` bleibt damit der Fall
/// eines ausgeblendeten Bereichs, dessen Rahmen die Breite 0 traegt.
fn gemessene_breiten(teiler: &NSSplitView) -> Breiten {
    let breite = |stelle: usize| {
        bereichsansicht(teiler, stelle).and_then(|ansicht| {
            let breite = ansicht.frame().size.width;
            (breite > 0.0).then_some(breite)
        })
    };
    Breiten {
        lesezeichen: breite(Bereich::Lesezeichen.index()),
        links: breite(Bereich::Links.index()),
        rechts: breite(Bereich::Rechts.index()),
        vorschau: breite(Bereich::Vorschau.index()),
        editor: breite(Bereich::Editor.index()),
        git: breite(Bereich::Git.index()),
    }
}

/// Welche Bereiche gerade im Fenster stehen.
fn gemessene_sichtbarkeit(teiler: &NSSplitView) -> Sichtbarkeit {
    Sichtbarkeit {
        erstes_dateifenster: steht_im(teiler, Bereich::Links),
        lesezeichen: steht_im(teiler, Bereich::Lesezeichen),
        zweites_dateifenster: steht_im(teiler, Bereich::Rechts),
        vorschau: steht_im(teiler, Bereich::Vorschau),
        editor: steht_im(teiler, Bereich::Editor),
        git: steht_im(teiler, Bereich::Git),
    }
}

/// Setzt die Rahmen der Bereiche nach der einen Rechenvorschrift.
///
/// **Die Sichtbarkeit kommt aus den Ansichten und nicht aus dem Modell**, und
/// zwar auf beiden Wegen hierher: [`Aufteilung::anwenden`] schreibt den Wunsch
/// des Modells vorher in die Ansichten, AppKit ruft ueber `neu_auslegen` ohne
/// Modell an. Vorher nahm der erste Weg die Modellsicht und der zweite die
/// gemessene entgegen, und dieselbe Fensterzeile lag je nach Ausloeser anders.
///
/// Der Unterschied der beiden Antworten war genau ein Bereich: der Editor, den
/// das Modell seit Schritt 13 fuehrt und die Aufteilung erst ab Schritt 16
/// traegt. Ihn mitzuzaehlen zog einen Trenner zu viel ab und gab ihm seine
/// Anfangsbreite von 460 Punkten, die anschliessend niemand setzte — die vier
/// wirklichen Bereiche bekamen sie nicht.
///
/// **Seit die fuenfte Unteransicht haengt, faellt der Unterschied weg**, und
/// zwar ohne eine Zeile in dieser Funktion: [`steht_im`] fuehrt den Editor
/// jetzt als stehend, sobald er nicht ausgeblendet ist, und Zaehler wie
/// Zuteilung nehmen ihn von selbst auf. Das war der Zweck der Umstellung vom
/// 260809 und nicht ihr Nebenprodukt.
///
/// Damit sagen Zaehler, Zuteilung und Schleife dasselbe: die Schleife
/// ueberspringt eine fehlende Unteransicht nicht mehr als Ausnahme, sondern
/// findet dort ohnehin die Breite 0.
///
/// **Gezaehlt wird seit der Bereichsleisten-Runde nicht mehr hier.** Diese
/// Funktion reicht mit [`Zeilenmass`] allein die Geometrie der Zeile weiter,
/// also die beiden Zahlen, die nur AppKit kennt; wie viele Trennlinien zwischen
/// den sichtbaren Bereichen liegen, rechnet
/// [`bereichsbreiten`](crate::fenstermodell::bereichsbreiten) aus derselben
/// [`Sichtbarkeit`], aus der es auch die Zuteilung rechnet. Vorher stand die
/// Rechnung hier und die Zuteilung dort, und beide mussten dieselbe Menge
/// meinen.
fn auslegen(teiler: &NSSplitView, breiten: &Breiten) {
    let gesamt = teiler.frame().size;
    let sichtbar = &gemessene_sichtbarkeit(teiler);
    let zugeteilt = crate::fenstermodell::bereichsbreiten(zeilenmass(teiler), breiten, sichtbar);

    let mut links = 0.0;
    for bereich in Bereich::ALLE {
        let Some(ansicht) = bereichsansicht(teiler, bereich.index()) else {
            continue;
        };
        let breite = zugeteilt[bereich.index()];
        ansicht.setFrame(NSRect::new(
            NSPoint::new(links, 0.0),
            NSSize::new(breite, gesamt.height),
        ));
        if breite > 0.0 {
            links += breite + teiler.dividerThickness();
        }
    }
}

/// Die beiden Zahlen der Fensterzeile, die nur AppKit kennt.
///
/// **Die eine Stelle, an der sie gelesen werden.** Zwei Aufrufer haengen daran:
/// [`auslegen`], das die Zeile auslegt, und [`Aufteilung::zeilenmass`], das das
/// Mass nach aussen gibt, damit das Fenstermodell mit derselben Geometrie
/// rechnet wie die Anzeige. Zweimal ausgeschrieben koennten die beiden Wege
/// verschiedene Zeilen meinen.
fn zeilenmass(teiler: &NSSplitView) -> Zeilenmass {
    Zeilenmass {
        gesamt: teiler.frame().size.width,
        trennerbreite: teiler.dividerThickness(),
    }
}

/// Die kleinste Lage, auf die sich die genannte Trennlinie ziehen laesst.
fn grenze_links(teiler: &NSSplitView, trennlinie: usize) -> f64 {
    let mut lage = 0.0;
    for bereich in Bereich::ALLE.into_iter().take(trennlinie + 1) {
        if steht_im(teiler, bereich) {
            lage += bereich.mindestbreite() + teiler.dividerThickness();
        }
    }
    (lage - teiler.dividerThickness()).max(0.0)
}

/// Die groesste Lage, auf die sich die genannte Trennlinie ziehen laesst.
fn grenze_rechts(teiler: &NSSplitView, trennlinie: usize) -> f64 {
    let mut noetig = 0.0;
    for bereich in Bereich::ALLE.into_iter().skip(trennlinie + 1) {
        if steht_im(teiler, bereich) {
            noetig += bereich.mindestbreite() + teiler.dividerThickness();
        }
    }
    (teiler.frame().size.width - noetig).max(0.0)
}
