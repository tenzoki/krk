//! Die Fensterzeile: eine `NSSplitView` mit fuenf Bereichen.
//!
//! ```text
//! ┌───────────┬──────────────────┬──────────────────┬───────────┬──────────┐
//! │ Lesezei-  │ Dateifenster     │ Dateifenster     │ Vorschau  │ Editor   │
//! │ chen (C5) │ links            │ rechts           │ (C6)      │          │
//! └───────────┴──────────────────┴──────────────────┴───────────┴──────────┘
//! ```
//!
//! Die beiden rechten Bereiche teilen sich denselben Platz: der Editor nimmt
//! die Stelle der Vorschau ein, und C1 der Editor-Runde sagt zu, dass beide
//! nie zugleich zu sehen sind. Die Regel dazu wohnt in
//! [`crate::fenstermodell`] und nicht hier; dieses Modul verteilt Breiten und
//! Sichtbarkeit und faellt keine Entscheidung darueber, welcher Bereich steht.
//!
//! **Jeder der fuenf Bereiche steht in einem `NSBox`**, und dessen Rahmen ist
//! die Anzeige aus C9: er sagt, welcher Bereich die Tasten annimmt, und
//! daneben, welches Dateifenster das aktive ist. Die Anzeige haengt am Rahmen
//! und nicht am Inhalt, damit sie auch dann eindeutig ist, wenn beide
//! Dateifenster denselben Ordner zeigen; und sie ist fuer alle fuenf dieselbe
//! Form, weil zwei der fuenf gar keine Auswahl haben, an der sich eine
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
//! Fenster groesser zieht. Im zweiten Fall speist es die Breiten ein, die
//! gerade auf dem Schirm stehen; damit ueberlebt eine mit der Maus verschobene
//! Trennlinie die naechste Fenstergroessenaenderung, ohne dass eine zweite
//! Rechenvorschrift daneben entstuende.
//!
//! **Welche Bereiche stehen, kommt in beiden Faellen aus den Unteransichten**
//! und nie aus dem Modell. Der erste Fall schreibt den Wunsch des Modells
//! vorher hinein und liest ihn dann von dort zurueck. Das ist ein Umweg von
//! einer Zeile und der Preis dafuer, dass die Frage nur eine Antwort hat:
//! [`steht_im`].

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSBox, NSBoxType, NSColor, NSSplitView, NSSplitViewDelegate,
    NSTitlePosition, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSInteger, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
};

use krk_core::ablage::{Breiten, Fensterseite, Sichtbarkeit};

use crate::fenstermodell::Bereich;
use crate::kommandos::fokus::{Fokus, Rahmenrolle, rahmenrolle};

use super::statuszeile;
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

define_class!(
    /// Der Delegierte der Aufteilung: Mindestbreiten und das Auslegen.
    ///
    /// Er haelt nichts. Alles, was er braucht, steht in der `NSSplitView`, die
    /// AppKit ihm bei jedem Aufruf mitgibt: die Rahmen der vier Bereiche und
    /// ihre Sichtbarkeit. Damit gibt es keinen Rueckweg von hier in das
    /// Fenstermodell und keinen Ring.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = ()]
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
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(splitView:resizeSubviewsWithOldSize:))]
        fn neu_auslegen(&self, teiler: &NSSplitView, _alte_groesse: NSSize) {
            let breiten = gemessene_breiten(teiler);
            auslegen(teiler, &breiten);
        }
    }
);

impl AufteilungsDelegierter {
    fn neu(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }
}

/// Die aufgebaute Fensterzeile.
pub struct Aufteilung {
    teiler: Retained<NSSplitView>,
    /// `NSSplitView` haelt seinen Delegierten schwach; hier steht er stark.
    _delegierter: Retained<AufteilungsDelegierter>,
    /// Die Kaesten aller fuenf Bereiche, in der Reihenfolge von
    /// [`Bereich::ALLE`].
    ///
    /// Die Feldbreite steht in der Typangabe: ein sechster Bereich haelt hier
    /// den Bau an, wie bei [`crate::fenstermodell::Bereich::ALLE`] selbst.
    rahmen: [Retained<NSBox>; 5],
}

impl Aufteilung {
    /// Baut die fuenf Bereiche um die beiden Dateifenster, die Leiste, die
    /// Vorschau und den Editor.
    ///
    /// Leiste, Vorschau und Editor kommen fertig herein und werden hier nicht
    /// gebaut: alle drei sind eigene fokussierbare Bereiche mit eigenem Inhalt,
    /// und dieses Modul verteilt Breiten und Sichtbarkeit. Dieselbe
    /// Aufgabenteilung wie bei den beiden Dateifenstern.
    pub fn bauen(
        mtm: MainThreadMarker,
        dateifenster: [&Dateifenster; 2],
        leiste: &NSView,
        vorschau: &NSView,
        editor: &NSView,
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

        // Alle fuenf gehen durch dieselbe Funktion, seit C9 die Anzeige auf
        // alle fuenf ausdehnt. Die beiden Dateifenster bringen ihren Inhalt
        // nicht fertig mit, sondern in drei Stuecken; `dateifensterinhalt`
        // legt sie uebereinander, und eingerahmt wird danach wie bei den
        // uebrigen drei.
        let rahmen = [
            gerahmt(mtm, leiste),
            gerahmt(mtm, &dateifensterinhalt(mtm, dateifenster[0])),
            gerahmt(mtm, &dateifensterinhalt(mtm, dateifenster[1])),
            gerahmt(mtm, vorschau),
            gerahmt(mtm, editor),
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
            _delegierter: delegierter,
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
    pub fn anwenden(&self, breiten: &Breiten, sichtbar: &Sichtbarkeit) {
        for bereich in Bereich::ALLE {
            if let Some(ansicht) = bereichsansicht(&self.teiler, bereich.index()) {
                ansicht.setHidden(!sichtbar_im(sichtbar, bereich));
            }
        }
        auslegen(&self.teiler, breiten);
    }

    /// Die Breiten, die gerade auf dem Schirm stehen.
    ///
    /// Der Weg, auf dem eine mit der Maus verschobene Trennlinie in die Sitzung
    /// kommt: sie steht in den Rahmen der Ansichten und nirgends sonst.
    pub fn gemessene_breiten(&self) -> [f64; 5] {
        let mut breiten = [0.0; 5];
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
    /// welche eine Ansicht den Ersthelferrang traegt, sondern in welchem der
    /// fuenf Teilbaeume er liegt; dafuer muss die Wurzel jedes Bereichs nach
    /// aussen. Sie liegt bereits vor, naemlich als Unteransicht der Aufteilung
    /// an der Stelle [`Bereich::index`], und deshalb entsteht hier keine zweite
    /// Aufzaehlung neben [`Bereich::ALLE`].
    ///
    /// Die fuenf Teilbaeume sind zueinander fremd, weil es die fuenf
    /// Unteransichten einer `NSSplitView` sind; ein Ersthelfer liegt in
    /// hoechstens einem.
    pub fn bereichssicht(&self, bereich: Bereich) -> Option<Retained<NSView>> {
        bereichsansicht(&self.teiler, bereich.index())
    }

    /// Faerbt die Rahmen aller fuenf Bereiche (C9).
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

/// Legt Tableiste, Dateiliste und Statuszeile eines Dateifensters uebereinander.
///
/// Von oben nach unten: die Leiste am Kopf, die Liste dazwischen, die Zeile am
/// Fuss. Die drei Autogroessen halten die Aufteilung fest, wenn der Nutzer die
/// Trennlinie verschiebt: die Leiste haengt oben, die Zeile unten, und die
/// Liste nimmt, was dazwischen frei wird.
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
        NSPoint::new(0.0, statuszeile::HOEHE),
        NSSize::new(breite, hoehe - tableiste::HOEHE - statuszeile::HOEHE),
    ));
    inhalt.addSubview(liste);

    let zeile = dateifenster.statuszeile_sicht();
    zeile.setFrame(NSRect::new(
        NSPoint::new(statuszeile::EINZUG, 0.0),
        NSSize::new(breite - statuszeile::EINZUG, statuszeile::HOEHE),
    ));
    inhalt.addSubview(zeile);

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

/// Das Feld eines [`Bereich`]s in [`Sichtbarkeit`].
///
/// Eine vollstaendige Fallunterscheidung ueber [`Bereich`]: ein neuer Bereich,
/// der hier fehlte, waere dauerhaft unsichtbar, ohne dass der Uebersetzer etwas
/// gesagt haette.
///
/// Eine Abbildung und sonst nichts. Ob ein Bereich im Fenster **steht**,
/// beantwortet [`steht_im`] und nicht diese Funktion; wer sie ueber eine
/// gemessene [`Sichtbarkeit`] fragt, bekommt dieselbe Antwort noch einmal.
fn sichtbar_im(sichtbar: &Sichtbarkeit, bereich: Bereich) -> bool {
    match bereich {
        Bereich::Lesezeichen => sichtbar.lesezeichen,
        Bereich::Links => true,
        Bereich::Rechts => sichtbar.zweites_dateifenster,
        Bereich::Vorschau => sichtbar.vorschau,
        Bereich::Editor => sichtbar.editor,
    }
}

/// Die Breiten, die gerade auf dem Schirm stehen.
///
/// Ein Bereich, dessen Unteransicht die Aufteilung nicht traegt, liefert `None`
/// und behaelt damit seine gespeicherte Breite. Seit Schritt 16 der
/// Editor-Runde stehen alle fuenf Unteransichten; `None` bleibt damit der Fall
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
    }
}

/// Welche Bereiche gerade im Fenster stehen.
fn gemessene_sichtbarkeit(teiler: &NSSplitView) -> Sichtbarkeit {
    Sichtbarkeit {
        lesezeichen: steht_im(teiler, Bereich::Lesezeichen),
        zweites_dateifenster: steht_im(teiler, Bereich::Rechts),
        vorschau: steht_im(teiler, Bereich::Vorschau),
        editor: steht_im(teiler, Bereich::Editor),
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
fn auslegen(teiler: &NSSplitView, breiten: &Breiten) {
    let gesamt = teiler.frame().size;
    let sichtbar = &gemessene_sichtbarkeit(teiler);
    let sichtbare = Bereich::ALLE
        .iter()
        .filter(|bereich| sichtbar_im(sichtbar, **bereich))
        .count();
    let trenner = teiler.dividerThickness() * (sichtbare.saturating_sub(1)) as f64;
    let verfuegbar = (gesamt.width - trenner).max(0.0);
    let zugeteilt = crate::fenstermodell::bereichsbreiten(verfuegbar, breiten, sichtbar);

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
