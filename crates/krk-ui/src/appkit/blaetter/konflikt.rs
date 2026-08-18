//! Das Konfliktblatt: was geschieht, wenn am Ziel schon ein Eintrag steht (C4).
//!
//! Vier Moeglichkeiten, wie C4 sie aufzaehlt, und dazu die Wahl "fuer alle
//! weiteren uebernehmen". Der Arbeitsfaden wartet, solange dieses Blatt steht;
//! die Antwort geht ueber den Kanal zurueck, den die Meldung mitgebracht hat.
//!
//! # Warum die Eingabetaste auf "Überspringen" liegt
//!
//! `NSAlert` gibt sie von sich aus der ersten Schaltflaeche, und das waere hier
//! "Überschreiben": ein reflexhaftes Bestaetigen loeschte damit den Eintrag am
//! Ziel. Dieselbe Ueberlegung, die C4 fuer die Loeschrueckfrage ausschreibt
//! ("vorbelegt ist Abbrechen, sodass ein reflexhaftes Bestaetigen mit der
//! Return-Taste nichts loescht"), traegt auch hier. Die
//! Reihenfolge der Schaltflaechen bleibt die des Spec; allein die Taste wandert.
//!
//! # Das Namensfeld steht bereit, ohne den Fokus zu nehmen
//!
//! Der Vorschlag darin ist der freie Name, den der Kern ohnehin fuer die Regel
//! "automatisch umbenennen" bildet. Das Feld ist **nicht** der Ersthelfer: waere
//! es das, gaebe der Ereignisabgriff jede Taste an AppKit weiter, und die
//! Schaltflaechen waeren ohne Maus nicht mehr erreichbar. Wer den Namen aendern
//! will, tabuliert hinein.
//!
//! # Und es traegt seit dem 260818 den Waechter
//!
//! Bis dahin war dies das eine Blatt im Baum mit einem Textfeld **ohne**
//! [`Eingabewaechter`](super::Eingabewaechter). Solange der Nutzer nicht
//! hineinklickt, kostet das nichts: Ersthelfer ist eine Schaltflaeche, und die
//! vier Tastenentsprechungen greifen. Sobald er hineintabuliert, um den Namen
//! fuer "Umbenennen" zu tippen, haelt das Feld den Ersthelferrang, und sein
//! Feldeditor verbraucht die Eingabe- und die Escape-Taste selbst — das Blatt
//! war dann mit keiner von beiden zu beantworten, und der Abbruchbefehl aus
//! `resources/default-keymap.toml` half nicht daneben, weil
//! `kommandos::zulaessigkeit::zulaessig` bei einem Textfeld als Ersthelfer auch
//! ihn abweist
//! (`issues/260817-1241_*_das-konfliktblatt-gibt-seinem-namensfeld-keinen-eingabewaechter.md`).
//!
//! **Der Waechter kennt zwei Antworten, dieses Blatt hat vier**, und das war
//! der Grund, ihn nicht einfach anzuhaengen: er schickte fuer "bestaetigt" fest
//! die **erste** Schaltflaeche, und die ist hier "Überschreiben". Ein Return im
//! Namensfeld haette damit den Eintrag am Ziel geloescht — dieselbe Bewegung,
//! die der Kopf darueber fuer die Vorgabeschaltflaeche ausdruecklich
//! ausschliesst. Beantwortet ist das an [`super::bestaetigungsstelle`]: die
//! Eingabetaste geht an die Schaltflaeche, die sie traegt, und das ist hier
//! "Überspringen". Der Waechter sagt damit im Feld dasselbe, was die
//! Erlaeuterung dem Nutzer ansagt ("Return überspringt"), und die Escape-Taste
//! faellt wie ueberall auf "Abbrechen".
//!
//! Zwei der vier Antworten bleiben im Feld ohne Taste: "Überschreiben" und
//! "Umbenennen" liegen auf Cmd+Return und Opt+Return, und ob der Feldeditor die
//! beiden durchlaesst, ist am laufenden Buendel zu messen und nicht hier zu
//! behaupten. Erreichbar sind sie in jedem Fall, indem der Nutzer das Feld
//! wieder verlaesst oder die Maus nimmt.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTextField` (ueber `NSControl`, `NSView` und `NSResponder`), `NSWindow`
//! und `NSString` stehen seit macOS 10.0 zur Verfuegung, ebenso `alloc`,
//! `initWithFrame:`, `setStringValue:` und `stringValue`. `NSPoint`, `NSRect`
//! und `NSSize` sind blosse Strukturen und tragen keine Verfuegbarkeitsangabe;
//! `MainThreadMarker` gehoert `objc2` und nicht AppKit. Das Buendel zielt auf
//! 15.0 (`.cargo/config.toml`); keine von ihnen ist nach macOS 15
//! hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.
//!
//! Was `NSAlert` selbst betrifft — die Schaltflaechen, ihre
//! Tastenentsprechungen und das Kaestchen —, steht im Kopf von [`Blatt`]:
//! diese Datei spricht es nicht an, sondern reicht Texte und die Beigabe
//! hinein.

use std::path::Path;

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{NSTextField, NSWindow};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString};

use krk_core::operation::{Konfliktantwort, Konfliktentscheid};

use super::{Blatt, Blattgriff, Schaltflaeche, Taste, Wirkung};

/// Die Breite des Namensfeldes in Punkten.
const FELDBREITE: f64 = 420.0;

/// Die Hoehe des Namensfeldes in Punkten.
const FELDHOEHE: f64 = 24.0;

/// Zeigt das Konfliktblatt und meldet die Wahl des Nutzers.
///
/// `vorschlag` ist der freie Name, den "Umbenennen" vorausfuellt. `fertig`
/// laeuft auf dem Hauptfaden und genau einmal.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    quelle: &Path,
    ziel: &Path,
    vorschlag: &str,
    fertig: impl Fn(Konfliktentscheid) + 'static,
) -> Blattgriff {
    let name = ziel.file_name().map_or_else(
        || ziel.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );

    let feld = NSTextField::initWithFrame(
        NSTextField::alloc(mtm),
        NSRect::new(NSPoint::ZERO, NSSize::new(FELDBREITE, FELDHOEHE)),
    );
    feld.setStringValue(&NSString::from_str(vorschlag));

    let mut blatt = Blatt::mit_schaltflaechen(
        mtm,
        &format!("„{name}“ gibt es am Ziel schon"),
        &[
            Schaltflaeche::neu(
                "Überschreiben",
                Taste::EingabeMitBefehl,
                Wirkung::Ausfuehren,
            ),
            Schaltflaeche::neu("Überspringen", Taste::Eingabe, Wirkung::Ausfuehren),
            Schaltflaeche::neu("Umbenennen", Taste::EingabeMitWahl, Wirkung::Ausfuehren),
            Schaltflaeche::neu("Abbrechen", Taste::Escape, Wirkung::Liegenlassen),
        ],
    );
    blatt.erlaeuterung_setzen(&format!(
        "Quelle: {}\nZiel: {}\n\nReturn überspringt, Cmd+Return überschreibt, \
         Opt+Return benennt um, Esc bricht ab.",
        quelle.display(),
        ziel.display()
    ));
    blatt.beigabe_setzen(&feld);
    // Der Waechter, aber **nicht** `textfeld_setzen`: das machte das Feld
    // daneben zum Ersthelfer, und der Kopf dieser Datei sagt, warum es das
    // nicht wird. Gebraucht wird allein die dritte der drei Handlungen.
    blatt.waechter_anhaengen(mtm, &feld);
    blatt.wahl_fuer_alle_zeigen("Für alle weiteren übernehmen");

    let feld: Retained<NSTextField> = feld;
    // Der Block liest das Kaestchen und das Feld erst, wenn der Nutzer
    // geantwortet hat; beide gehoeren dem Blatt, und das Blatt lebt bis dahin.
    let ablesen = AntwortAblesen { feld };
    blatt.zeigen_mit_wahl(fenster, move |stelle, fuer_alle| {
        let antwort = match stelle {
            0 => Konfliktantwort::Ueberschreiben,
            1 => Konfliktantwort::Ueberspringen,
            2 => Konfliktantwort::UmbenennenIn(ablesen.name()),
            _ => Konfliktantwort::Abbrechen,
        };
        fertig(Konfliktentscheid {
            antwort,
            // "Fuer alle weiteren" gilt nicht fuer den Abbruch: der beendet den
            // Vorgang ohnehin, und ein angekreuztes Kaestchen daneben waere eine
            // Regel ohne weiteren Fall.
            fuer_alle_weiteren: fuer_alle && stelle != 3,
        });
    })
}

/// Der Halter des Namensfeldes fuer den Abschlussblock.
struct AntwortAblesen {
    feld: Retained<NSTextField>,
}

impl AntwortAblesen {
    /// Der Name, den der Nutzer stehen gelassen oder getippt hat.
    fn name(&self) -> String {
        self.feld.stringValue().to_string().trim().to_owned()
    }
}
