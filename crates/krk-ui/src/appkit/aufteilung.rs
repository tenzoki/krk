//! Die Fensterzeile: eine `NSSplitView` mit vier Bereichen.
//!
//! ```text
//! ┌───────────┬──────────────────┬──────────────────┬───────────┐
//! │ Lesezei-  │ Dateifenster     │ Dateifenster     │ Vorschau  │
//! │ chen (C5) │ links            │ rechts           │ (C6)      │
//! └───────────┴──────────────────┴──────────────────┴───────────┘
//! ```
//!
//! Jedes Dateifenster steht in einem `NSBox`, und dessen Rahmen ist die
//! Markierung des aktiven Dateifensters aus C1: der aktive traegt die
//! Akzentfarbe des Systems, der andere die Farbe einer gewoehnlichen
//! Trennlinie. Die Markierung haengt am Rahmen und nicht am Inhalt, damit sie
//! auch dann eindeutig ist, wenn beide Dateifenster denselben Ordner zeigen.
//!
//! Die Lesezeichenleiste steht seit Schritt 18 als eigener Bereich darin und
//! kommt fertig von [`super::leiste`] herein; das Vorschaufenster kommt seit
//! Schritt 19 ebenso fertig von [`super::vorschau`]. Breite und Sichtbarkeit
//! beider gehoeren zu C7 und damit hierher.
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

use super::statuszeile;
use super::tabelle::Dateifenster;
use super::tableiste;

/// Die Breite des Rahmens, der das aktive Dateifenster markiert.
const RAHMENBREITE: f64 = 2.0;

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
            let sichtbar = gemessene_sichtbarkeit(teiler);
            auslegen(teiler, &breiten, &sichtbar);
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
    /// Die Rahmen der beiden Dateifenster, in der Reihenfolge links, rechts.
    rahmen: [Retained<NSBox>; 2],
}

impl Aufteilung {
    /// Baut die vier Bereiche um die beiden Dateifenster, die Leiste und die
    /// Vorschau.
    ///
    /// Leiste und Vorschau kommen fertig herein und werden hier nicht gebaut:
    /// beide sind eigene fokussierbare Bereiche mit eigenem Inhalt, und dieses
    /// Modul verteilt Breiten und Sichtbarkeit. Dieselbe Aufgabenteilung wie
    /// bei den beiden Dateifenstern.
    pub fn bauen(
        mtm: MainThreadMarker,
        dateifenster: [&Dateifenster; 2],
        leiste: &NSView,
        vorschau: &NSView,
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

        let rahmen = [
            gerahmtes_dateifenster(mtm, dateifenster[0]),
            gerahmtes_dateifenster(mtm, dateifenster[1]),
        ];
        // Die Reihenfolge ist die von `Bereich::ALLE` und die einzige, in der
        // die Rechenvorschrift der Breiten die Bereiche wiederfindet.
        teiler.addSubview(leiste);
        teiler.addSubview(&rahmen[0]);
        teiler.addSubview(&rahmen[1]);
        teiler.addSubview(vorschau);

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
    pub fn anwenden(&self, breiten: &Breiten, sichtbar: &Sichtbarkeit) {
        for bereich in Bereich::ALLE {
            if let Some(ansicht) = bereichsansicht(&self.teiler, bereich.index()) {
                ansicht.setHidden(!sichtbar_im(sichtbar, bereich));
            }
        }
        auslegen(&self.teiler, breiten, sichtbar);
    }

    /// Die Breiten, die gerade auf dem Schirm stehen.
    ///
    /// Der Weg, auf dem eine mit der Maus verschobene Trennlinie in die Sitzung
    /// kommt: sie steht in den Rahmen der Ansichten und nirgends sonst.
    pub fn gemessene_breiten(&self) -> [f64; 4] {
        let mut breiten = [0.0; 4];
        for bereich in Bereich::ALLE {
            if let Some(ansicht) = bereichsansicht(&self.teiler, bereich.index()) {
                breiten[bereich.index()] = ansicht.frame().size.width;
            }
        }
        breiten
    }

    /// Markiert das aktive Dateifenster (C1).
    ///
    /// Die Markierung ist ein farbiger Rahmen und kein Unterschied im Inhalt:
    /// nur so bleibt sie eindeutig, wenn beide Dateifenster denselben Ordner
    /// zeigen.
    pub fn aktives_markieren(&self, seite: Fensterseite) {
        for kandidat in Fensterseite::ALLE {
            let kasten = &self.rahmen[kandidat.index()];
            let farbe = if kandidat == seite {
                NSColor::controlAccentColor()
            } else {
                NSColor::separatorColor()
            };
            kasten.setBorderColor(&farbe);
        }
    }
}

/// Setzt Tableiste, Dateiliste und Statuszeile in einen Rahmen.
///
/// Von oben nach unten: die Leiste am Kopf, die Liste dazwischen, die Zeile am
/// Fuss. Die drei Autogroessen halten die Aufteilung fest, wenn der Nutzer die
/// Trennlinie verschiebt: die Leiste haengt oben, die Zeile unten, und die
/// Liste nimmt, was dazwischen frei wird.
fn gerahmtes_dateifenster(mtm: MainThreadMarker, dateifenster: &Dateifenster) -> Retained<NSBox> {
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

    let kasten = NSBox::initWithFrame(NSBox::alloc(mtm), NSRect::new(NSPoint::ZERO, AUFBAUGROESSE));
    kasten.setBoxType(NSBoxType::Custom);
    kasten.setTitlePosition(NSTitlePosition::NoTitle);
    kasten.setBorderWidth(RAHMENBREITE);
    kasten.setBorderColor(&NSColor::separatorColor());
    kasten.setFillColor(&NSColor::clearColor());
    kasten.setContentViewMargins(NSSize::ZERO);
    kasten.setContentView(Some(&inhalt));
    kasten
}

/// Die Ansicht eines Bereichs, falls die Aufteilung sie schon traegt.
fn bereichsansicht(teiler: &NSSplitView, stelle: usize) -> Option<Retained<NSView>> {
    let ansichten = teiler.subviews();
    (stelle < ansichten.len()).then(|| ansichten.objectAtIndex(stelle))
}

/// Ob der Bereich nach der genannten Sichtbarkeit steht.
fn sichtbar_im(sichtbar: &Sichtbarkeit, bereich: Bereich) -> bool {
    match bereich {
        Bereich::Lesezeichen => sichtbar.lesezeichen,
        Bereich::Links => true,
        Bereich::Rechts => sichtbar.zweites_dateifenster,
        Bereich::Vorschau => sichtbar.vorschau,
    }
}

/// Die Breiten, die gerade auf dem Schirm stehen.
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
    }
}

/// Welche Bereiche gerade sichtbar sind.
fn gemessene_sichtbarkeit(teiler: &NSSplitView) -> Sichtbarkeit {
    let steht =
        |stelle: usize| bereichsansicht(teiler, stelle).is_some_and(|ansicht| !ansicht.isHidden());
    Sichtbarkeit {
        lesezeichen: steht(Bereich::Lesezeichen.index()),
        zweites_dateifenster: steht(Bereich::Rechts.index()),
        vorschau: steht(Bereich::Vorschau.index()),
    }
}

/// Setzt die Rahmen der vier Bereiche nach der einen Rechenvorschrift.
fn auslegen(teiler: &NSSplitView, breiten: &Breiten, sichtbar: &Sichtbarkeit) {
    let gesamt = teiler.frame().size;
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
        if bereichsansicht(teiler, bereich.index()).is_some_and(|a| !a.isHidden()) {
            lage += bereich.mindestbreite() + teiler.dividerThickness();
        }
    }
    (lage - teiler.dividerThickness()).max(0.0)
}

/// Die groesste Lage, auf die sich die genannte Trennlinie ziehen laesst.
fn grenze_rechts(teiler: &NSSplitView, trennlinie: usize) -> f64 {
    let mut noetig = 0.0;
    for bereich in Bereich::ALLE.into_iter().skip(trennlinie + 1) {
        if bereichsansicht(teiler, bereich.index()).is_some_and(|a| !a.isHidden()) {
            noetig += bereich.mindestbreite() + teiler.dividerThickness();
        }
    }
    (teiler.frame().size.width - noetig).max(0.0)
}
