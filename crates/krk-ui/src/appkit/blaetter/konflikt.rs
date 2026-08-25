//! Das Konfliktblatt: was geschieht, wenn am Ziel schon ein Eintrag steht (C4).
//!
//! **Zwei Gestalten, eine Datei.** Ein Vorgang mit mehreren Zielen bekommt die
//! vier Moeglichkeiten, wie C4 sie aufzaehlt, und dazu die Wahl "fuer alle
//! weiteren uebernehmen". Ein Vorgang, der genau **eine** Zieldatei erzeugt,
//! bekommt drei davon: "Überspringen" faellt weg, weil es dort dasselbe
//! bewirkt wie "Abbrechen", naemlich einen Vorgang, der ohne Ziel endet, und
//! das Kaestchen faellt weg, weil es keinen weiteren Fall gibt, fuer den es
//! gelten koennte. So gewaehlt vom Nutzer am 260824-2120
//! (`circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/decisions/260825-0711_*_welche-antworten-bietet-das-konfliktblatt-bei-genau-einer-zieldatei.md`,
//! Moeglichkeit 2). Welche Gestalt es wird, sagt der Aufrufer im Argument
//! `genau_ein_ziel`; gerechnet hat es
//! [`crate::kommandos::operationen::erzeugt_genau_ein_ziel`] ueber die
//! [`Art`](krk_core::operation::Art) des Vorgangs, und beide Gestalten stehen
//! als eine Angabe in [`schaltflaechen`].
//!
//! Der Arbeitsfaden wartet, solange dieses Blatt steht; die Antwort geht ueber
//! den Kanal zurueck, den die Meldung mitgebracht hat.
//!
//! # Die Eingabetaste liegt in keiner Gestalt auf "Überschreiben"
//!
//! `NSAlert` gaebe sie von sich aus der ersten Schaltflaeche, und das waere
//! hier "Überschreiben": ein reflexhaftes Bestaetigen loeschte damit den
//! Eintrag am Ziel. Dieselbe Ueberlegung, die C4 fuer die Loeschrueckfrage
//! ausschreibt ("vorbelegt ist Abbrechen, sodass ein reflexhaftes Bestaetigen
//! mit der Return-Taste nichts loescht"), traegt auch hier. Die Reihenfolge der
//! Schaltflaechen bleibt die des Spec; allein die Taste wandert.
//!
//! ```text
//!   mehrere Ziele  Überschreiben  Überspringen  Umbenennen  Abbrechen
//!                  Cmd+Return     Return        Opt+Return  Esc
//!
//!   ein Ziel       Überschreiben  Umbenennen    Abbrechen
//!                  Cmd+Return     Opt+Return    Return
//! ```
//!
//! **Faellt "Überspringen" weg, bekommt "Abbrechen" die Eingabetaste**, und
//! damit traegt dieselbe Schaltflaeche beide ungefaehrlichen Wege. Ein neuer
//! Mechanismus entsteht dabei nicht: [`super::bestaetigungsstelle`] rechnet aus
//! dem Feld [`Taste`], [`super::abbruchstelle`] aus dem Feld [`Wirkung`], und
//! in dieser Gestalt fallen beide auf "Abbrechen". Die Escape-Taste steht
//! daneben an keiner Schaltflaeche, weil ein `NSButton` genau eine
//! Tastenentsprechung traegt; sie erreicht das Blatt ueber den Abbruchbefehl
//! aus `resources/default-keymap.toml` und
//! [`Blattgriff::abbrechen`](super::Blattgriff::abbrechen), den der
//! Anwendungsdelegierte fuer das offene Blatt haelt. Die Rueckfrage vor dem
//! Raeumen in den Papierkorb faehrt dieselbe Form seit der Runde 12
//! (`super::loeschbestaetigung`), und diese Gestalt ist die zweite und keine
//! zweite Bauart.
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
//! Tastenentsprechungen greifen. Sobald er hineintabuliert, um den Namen
//! fuer "Umbenennen" zu tippen, haelt das Feld den Ersthelferrang, und sein
//! Feldeditor verbraucht die Eingabe- und die Escape-Taste selbst — das Blatt
//! war dann mit keiner von beiden zu beantworten, und der Abbruchbefehl aus
//! `resources/default-keymap.toml` half nicht daneben, weil
//! `kommandos::zulaessigkeit::zulaessig` bei einem Textfeld als Ersthelfer auch
//! ihn abweist
//! (`issues/260817-1241_*_das-konfliktblatt-gibt-seinem-namensfeld-keinen-eingabewaechter.md`).
//!
//! **Der Waechter kennt zwei Antworten, dieses Blatt hat drei oder vier**, und
//! das war der Grund, ihn nicht einfach anzuhaengen: er schickte fuer
//! "bestaetigt" fest die **erste** Schaltflaeche, und die ist in beiden
//! Gestalten "Überschreiben". Ein Return im Namensfeld haette damit den Eintrag
//! am Ziel geloescht — dieselbe Bewegung, die der Kopf darueber fuer die
//! Vorgabeschaltflaeche ausdruecklich ausschliesst. Beantwortet ist das an
//! [`super::bestaetigungsstelle`]: die Eingabetaste geht an die Schaltflaeche,
//! die sie traegt, und das ist bei mehreren Zielen "Überspringen" und bei einem
//! "Abbrechen". Der Waechter sagt damit im Feld dasselbe, was
//! [`tastenhinweis`] dem Nutzer ansagt, und die Escape-Taste faellt wie ueberall
//! auf "Abbrechen".
//!
//! Zwei Antworten bleiben im Feld ohne Taste: "Überschreiben" und
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

/// Die Schaltflaechen des Blattes, in bindender Reihenfolge.
///
/// **Als reine Funktion herausgezogen**, damit beide Gestalten ohne AppKit und
/// ohne Hauptfaden pruefbar sind, wie es
/// [`super::loeschbestaetigung::schaltflaechen`](super::loeschbestaetigung)
/// seit der Runde 12 vormacht. Sie ist die eine Angabe, gegen die
/// [`antwort`] gehalten wird: die Proben unter `mod tests` lesen aus dieser
/// Liste die Beschriftung an der Stelle, die [`antwort`] auf einen Wert von
/// [`Konfliktantwort`] abbildet, und werden rot, sobald sich eine der beiden
/// Reihenfolgen ohne die andere dreht.
///
/// `genau_ein_ziel` ist die Auskunft aus
/// [`crate::kommandos::operationen::erzeugt_genau_ein_ziel`].
fn schaltflaechen(genau_ein_ziel: bool) -> Vec<Schaltflaeche<'static>> {
    if genau_ein_ziel {
        vec![
            Schaltflaeche::neu(
                "Überschreiben",
                Taste::EingabeMitBefehl,
                Wirkung::Ausfuehren,
            ),
            Schaltflaeche::neu("Umbenennen", Taste::EingabeMitWahl, Wirkung::Ausfuehren),
            // Die Eingabetaste und nicht die Escape-Taste: "Überspringen" hat
            // sie hier nicht mehr, und "Überschreiben" darf sie nicht bekommen.
            // Die Escape-Taste erreicht dieselbe Schaltflaeche ueber den
            // Abbruchbefehl und den Blattgriff, siehe Modulkopf.
            Schaltflaeche::neu("Abbrechen", Taste::Eingabe, Wirkung::Liegenlassen),
        ]
    } else {
        vec![
            Schaltflaeche::neu(
                "Überschreiben",
                Taste::EingabeMitBefehl,
                Wirkung::Ausfuehren,
            ),
            Schaltflaeche::neu("Überspringen", Taste::Eingabe, Wirkung::Ausfuehren),
            Schaltflaeche::neu("Umbenennen", Taste::EingabeMitWahl, Wirkung::Ausfuehren),
            Schaltflaeche::neu("Abbrechen", Taste::Escape, Wirkung::Liegenlassen),
        ]
    }
}

/// Die Antwort zu der Schaltflaeche, die der Nutzer gedrueckt hat.
///
/// `name` ist der Inhalt des Namensfeldes; er wird allein fuer
/// [`Konfliktantwort::UmbenennenIn`] gebraucht.
///
/// # Die Tafeln
///
/// Zwei Gestalten, zwei Tafeln, und jede haelt die Reihenfolge aus
/// [`schaltflaechen`] ein.
///
/// | Stelle | mehrere Ziele | genau ein Ziel |
/// |---|---|---|
/// | `0` | [`Konfliktantwort::Ueberschreiben`] | [`Konfliktantwort::Ueberschreiben`] |
/// | `1` | [`Konfliktantwort::Ueberspringen`] | [`Konfliktantwort::UmbenennenIn`] |
/// | `2` | [`Konfliktantwort::UmbenennenIn`] | [`Konfliktantwort::Abbrechen`] |
/// | `3` | [`Konfliktantwort::Abbrechen`] | — |
/// | jede weitere | [`Konfliktantwort::Abbrechen`] | [`Konfliktantwort::Abbrechen`] |
///
/// **Die letzte Zeile kommt nicht vor, und sie steht trotzdem da.**
/// [`Blatt::zeigen_mit_wahl`] bildet eine Antwort, die zu keiner angelegten
/// Schaltflaeche gehoert, bereits auf [`super::abbruchstelle`] ab; hier trifft
/// sie deshalb nur noch eine Stelle, die es gibt. Der Auffangzweig faellt
/// dennoch auf "Abbrechen" und nicht auf die erste Stelle, aus demselben
/// Grund, den [`super::bestaetigungsstelle`] fuer ihre zweite Tafelzeile
/// ausschreibt: lieber nichts tun als raten. Eine vollstaendige
/// Fallunterscheidung ohne Auffangzweig ist ueber `usize` nicht zu haben.
fn antwort(stelle: usize, genau_ein_ziel: bool, name: &str) -> Konfliktantwort {
    if genau_ein_ziel {
        match stelle {
            0 => Konfliktantwort::Ueberschreiben,
            1 => Konfliktantwort::UmbenennenIn(name.to_owned()),
            _ => Konfliktantwort::Abbrechen,
        }
    } else {
        match stelle {
            0 => Konfliktantwort::Ueberschreiben,
            1 => Konfliktantwort::Ueberspringen,
            2 => Konfliktantwort::UmbenennenIn(name.to_owned()),
            _ => Konfliktantwort::Abbrechen,
        }
    }
}

/// Der Satz, der die Tasten der Schaltflaechen ansagt.
///
/// Er steht je Gestalt genau einmal da und sagt dasselbe, was
/// [`schaltflaechen`] anlegt. Bis zur Runde 17 sagte er in beiden Faellen
/// "Return überspringt"; in der gekuerzten Gestalt gibt es kein Überspringen
/// mehr, und der Satz nennt die Eingabetaste dort beim Abbruch.
fn tastenhinweis(genau_ein_ziel: bool) -> &'static str {
    if genau_ein_ziel {
        "Return und Esc brechen ab, Cmd+Return überschreibt, Opt+Return benennt um."
    } else {
        "Return überspringt, Cmd+Return überschreibt, Opt+Return benennt um, Esc bricht ab."
    }
}

/// Zeigt das Konfliktblatt und meldet die Wahl des Nutzers.
///
/// `vorschlag` ist der freie Name, den "Umbenennen" vorausfuellt.
/// `genau_ein_ziel` waehlt die Gestalt: `true` kuerzt auf Überschreiben,
/// Umbenennen und Abbrechen und laesst das Kaestchen "fuer alle weiteren"
/// weg, siehe Modulkopf. `fertig` laeuft auf dem Hauptfaden und genau einmal.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    quelle: &Path,
    ziel: &Path,
    vorschlag: &str,
    genau_ein_ziel: bool,
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
        &schaltflaechen(genau_ein_ziel),
    );
    blatt.erlaeuterung_setzen(&format!(
        "Quelle: {}\nZiel: {}\n\n{}",
        quelle.display(),
        ziel.display(),
        tastenhinweis(genau_ein_ziel)
    ));
    blatt.beigabe_setzen(&feld);
    // Der Waechter, aber **nicht** `textfeld_setzen`: das machte das Feld
    // daneben zum Ersthelfer, und der Kopf dieser Datei sagt, warum es das
    // nicht wird. Gebraucht wird allein die dritte der drei Handlungen.
    blatt.waechter_anhaengen(mtm, &feld);
    if !genau_ein_ziel {
        blatt.wahl_fuer_alle_zeigen("Für alle weiteren übernehmen");
    }

    let feld: Retained<NSTextField> = feld;
    // Der Block liest das Kaestchen und das Feld erst, wenn der Nutzer
    // geantwortet hat; beide gehoeren dem Blatt, und das Blatt lebt bis dahin.
    let ablesen = AntwortAblesen { feld };
    blatt.zeigen_mit_wahl(fenster, move |stelle, fuer_alle| {
        let antwort = antwort(stelle, genau_ein_ziel, &ablesen.name());
        fertig(Konfliktentscheid {
            // "Fuer alle weiteren" gilt nicht fuer den Abbruch: der beendet den
            // Vorgang ohnehin, und ein angekreuztes Kaestchen daneben waere eine
            // Regel ohne weiteren Fall. Gefragt wird die **Antwort** und nicht
            // die Stelle, weil die Stelle des Abbruchs je Gestalt eine andere
            // ist; ohne Kaestchen ist `fuer_alle` ohnehin immer `false`.
            fuer_alle_weiteren: fuer_alle && antwort != Konfliktantwort::Abbrechen,
            antwort,
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

#[cfg(test)]
mod tests {
    use crate::appkit::blaetter::{abbruchstelle, bestaetigungsstelle};

    use super::{Konfliktantwort, Taste, Wirkung, antwort, schaltflaechen, tastenhinweis};

    /// Der getippte Name, den die Proben durch die Rueckrechnung schicken.
    const NAME: &str = "Projekte 2.zip";

    /// Die Tafel "Stelle → Antwort" in der vollen Gestalt, Zeile fuer Zeile.
    ///
    /// Gehalten wird sie gegen [`schaltflaechen`] und nicht gegen eine zweite
    /// Aufzaehlung im Probenrumpf: die Beschriftung an der Stelle ist der eine
    /// Beleg dafuer, dass die Rueckrechnung dieselbe Reihenfolge liest, die das
    /// Blatt anlegt.
    #[test]
    fn die_tafel_bei_mehreren_zielen() {
        let schaltflaechen = schaltflaechen(false);
        let tafel: [(usize, &str, Konfliktantwort); 4] = [
            (0, "Überschreiben", Konfliktantwort::Ueberschreiben),
            (1, "Überspringen", Konfliktantwort::Ueberspringen),
            (
                2,
                "Umbenennen",
                Konfliktantwort::UmbenennenIn(NAME.to_owned()),
            ),
            (3, "Abbrechen", Konfliktantwort::Abbrechen),
        ];
        assert_eq!(
            schaltflaechen.len(),
            tafel.len(),
            "die Tafel deckt nicht jede Schaltflaeche ab"
        );
        for (stelle, titel, erwartet) in tafel {
            assert_eq!(
                schaltflaechen[stelle].titel, titel,
                "an Stelle {stelle} steht eine andere Schaltflaeche"
            );
            assert_eq!(
                antwort(stelle, false, NAME),
                erwartet,
                "Stelle {stelle} („{titel}“) wird falsch zurueckgerechnet"
            );
        }
        assert_eq!(
            antwort(9, false, NAME),
            Konfliktantwort::Abbrechen,
            "eine unbekannte Stelle richtet etwas an"
        );
    }

    /// Die Tafel "Stelle → Antwort" in der gekuerzten Gestalt.
    ///
    /// Die zweite Gestalt aus der Nutzerentscheidung vom 260824-2120:
    /// "Überspringen" faellt weg, und die drei uebrigen ruecken auf. Ohne diese
    /// Probe liefe die Rueckrechnung der vollen Gestalt ueber drei
    /// Schaltflaechen und machte aus "Umbenennen" ein Überspringen und aus
    /// "Abbrechen" ein Umbenennen.
    #[test]
    fn die_tafel_bei_genau_einem_ziel() {
        let schaltflaechen = schaltflaechen(true);
        let tafel: [(usize, &str, Konfliktantwort); 3] = [
            (0, "Überschreiben", Konfliktantwort::Ueberschreiben),
            (
                1,
                "Umbenennen",
                Konfliktantwort::UmbenennenIn(NAME.to_owned()),
            ),
            (2, "Abbrechen", Konfliktantwort::Abbrechen),
        ];
        assert_eq!(
            schaltflaechen.len(),
            tafel.len(),
            "die gekuerzte Gestalt traegt nicht drei Schaltflaechen"
        );
        for (stelle, titel, erwartet) in tafel {
            assert_eq!(
                schaltflaechen[stelle].titel, titel,
                "an Stelle {stelle} steht eine andere Schaltflaeche"
            );
            assert_eq!(
                antwort(stelle, true, NAME),
                erwartet,
                "Stelle {stelle} („{titel}“) wird falsch zurueckgerechnet"
            );
        }
        assert!(
            !schaltflaechen
                .iter()
                .any(|schaltflaeche| schaltflaeche.titel == "Überspringen"),
            "die gekuerzte Gestalt bietet weiterhin das Überspringen"
        );
        assert_eq!(
            antwort(9, true, NAME),
            Konfliktantwort::Abbrechen,
            "eine unbekannte Stelle richtet etwas an"
        );
    }

    /// Die Eingabetaste liegt in keiner Gestalt auf "Überschreiben".
    ///
    /// Der Sicherheitsgrund aus dem Modulkopf, gemessen an derselben reinen
    /// Funktion, die das Blatt einsetzt: [`bestaetigungsstelle`] rechnet aus
    /// dem Feld [`Taste`], und was sie trifft, darf nichts ueberschreiben.
    #[test]
    fn die_eingabetaste_traegt_in_keiner_gestalt_das_ueberschreiben() {
        for genau_ein_ziel in [false, true] {
            let schaltflaechen = schaltflaechen(genau_ein_ziel);
            let stelle = bestaetigungsstelle(&schaltflaechen);
            assert_ne!(
                schaltflaechen[stelle].titel, "Überschreiben",
                "bei genau_ein_ziel={genau_ein_ziel} loescht ein Return am Ziel"
            );
            assert_eq!(
                schaltflaechen[stelle].taste,
                Taste::Eingabe,
                "bei genau_ein_ziel={genau_ein_ziel} traegt die getroffene Schaltflaeche die Taste nicht"
            );
        }
    }

    /// Beide Gestalten haben einen ungefaehrlichen Ausgang, und er heisst
    /// "Abbrechen".
    ///
    /// In der gekuerzten Gestalt fallen [`abbruchstelle`] und
    /// [`bestaetigungsstelle`] auf dieselbe Schaltflaeche; genau darauf beruht
    /// es, dass die Escape-Taste ueber den Blattgriff und die Eingabetaste ueber
    /// die Tastenentsprechung dasselbe tun.
    #[test]
    fn beide_gestalten_lassen_ueber_abbrechen_liegen() {
        for genau_ein_ziel in [false, true] {
            let schaltflaechen = schaltflaechen(genau_ein_ziel);
            let stelle = abbruchstelle(&schaltflaechen);
            assert_eq!(
                schaltflaechen[stelle].wirkung,
                Wirkung::Liegenlassen,
                "bei genau_ein_ziel={genau_ein_ziel} fuehrt der ungefaehrliche Ausgang etwas aus"
            );
            assert_eq!(
                schaltflaechen[stelle].titel, "Abbrechen",
                "bei genau_ein_ziel={genau_ein_ziel} heisst der ungefaehrliche Ausgang anders"
            );
            assert_eq!(
                antwort(stelle, genau_ein_ziel, NAME),
                Konfliktantwort::Abbrechen,
                "bei genau_ein_ziel={genau_ein_ziel} rechnet die Stelle auf etwas anderes zurueck"
            );
        }
        assert_eq!(
            abbruchstelle(&schaltflaechen(true)),
            bestaetigungsstelle(&schaltflaechen(true)),
            "in der gekuerzten Gestalt tragen Return und Esc nicht dieselbe Schaltflaeche"
        );
    }

    /// Der Tastenhinweis sagt dasselbe wie die angelegten Tasten.
    ///
    /// Er ist der Satz, den der Nutzer liest, und bis zur Runde 17 sagte er in
    /// beiden Faellen "Return überspringt". In der gekuerzten Gestalt gibt es
    /// kein Überspringen; ein Satz, der es dort noch ansagt, schickte den
    /// Nutzer auf eine Schaltflaeche, die nicht dasteht.
    #[test]
    fn der_tastenhinweis_nennt_die_tasten_der_gestalt() {
        assert!(
            tastenhinweis(false).contains("Return überspringt"),
            "die volle Gestalt sagt das Überspringen nicht an"
        );
        assert!(
            !tastenhinweis(true).contains("überspringt"),
            "die gekuerzte Gestalt sagt ein Überspringen an, das sie nicht bietet"
        );
        assert!(
            tastenhinweis(true).contains("Return und Esc brechen ab"),
            "die gekuerzte Gestalt sagt die Eingabetaste nicht beim Abbruch an"
        );
        for hinweis in [tastenhinweis(false), tastenhinweis(true)] {
            assert!(
                hinweis.contains("Cmd+Return überschreibt") && hinweis.contains("Opt+Return"),
                "der Hinweis „{hinweis}“ nennt nicht beide Zusatztasten"
            );
        }
    }
}
