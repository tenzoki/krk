//! Das Blatt nach einem Sichern, das an einer Aenderung von aussen
//! abgewiesen wurde (C4 der Editor-Runde, Nutzerentscheid vom 261004).
//!
//! Drei Wahlmoeglichkeiten, in dieser Reihenfolge: neu laden, trotzdem
//! ueberschreiben, abbrechen. Bis zum 261004 stand an dieser Stelle allein ein
//! Satz in der Statuszeile, der mit dem naechsten Tastendruck verschwand, und
//! „Sichern“ in der Nachfrage aus C4 fuehrte immer wieder auf dieselbe Nachfrage
//! zurueck, weil kein Weg aus der Lage herausfuehrte.
//!
//! # Dieses Blatt rechnet nichts und kennt den Anlass nicht
//!
//! Dasselbe Muster wie [`super::ungesichert`]: es fragt und antwortet, und was
//! auf die Antwort folgt, traegt die Schliessung des Aufrufers. Ob nach der
//! Antwort noch ein Anlass aus C4 wartet (eine andere Datei, das Schliessen,
//! das Beenden) oder ob ein blosses `cmd+s` hierher gefuehrt hat, weiss allein
//! diese Schliessung.
//!
//! # Warum die Eingabetaste auf "Abbrechen" liegt
//!
//! Anders als bei der Nachfrage aus C4 gibt es hier keine bewahrende Antwort
//! unter den ausfuehrenden: „Neu laden“ verwirft, was im Editor steht, und
//! „Trotzdem überschreiben“ verwirft, was ein anderes Programm auf die Platte
//! geschrieben hat. Ein reflexhaftes Bestaetigen soll keines von beiden
//! verlieren, also liegt die Eingabetaste auf „Abbrechen“, wie bei der
//! Rueckfrage vor dem Raeumen in den Papierkorb. Die beiden ausfuehrenden
//! Antworten kosten je eine Zusatztaste, und der erlaeuternde Text nennt alle
//! Wege; `esc` geht ueber den Abbruchbefehl und den [`Blattgriff`].
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! Eine einzige AppKit-Klasse, `NSWindow`, und die Datei reicht sie nur weiter;
//! sie steht seit macOS 10.0 zur Verfuegung. `MainThreadMarker` gehoert `objc2`
//! und nicht AppKit. Das Buendel zielt auf 15.0 (`.cargo/config.toml`), und
//! nichts hier ist nach macOS 15 hinzugekommen; `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und die Nennung ist die Gegenmassnahme.
//! Alles, was `NSAlert` betrifft, steht im Kopf von [`Blatt`].

use std::path::Path;

use objc2_app_kit::NSWindow;
use objc2_foundation::MainThreadMarker;

use krk_core::sprache::{Text, satz, text};

use super::{Blatt, Blattgriff, Schaltflaeche, Taste, Wirkung};

/// Was der Nutzer auf die Aenderung von aussen geantwortet hat.
///
/// **Drei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig**;
/// ein vierter haelt beim Aufrufer den Bau an.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Antwort {
    /// Den Stand des Editors verwerfen und die Fassung der Platte laden; ein
    /// wartender Anlass laeuft danach, denn nichts Ungesichertes bleibt.
    NeuLaden,
    /// Den Stand des Editors ueber die Fassung der Platte schreiben; ein
    /// wartender Anlass laeuft, wenn das Schreiben gelingt.
    Ueberschreiben,
    /// Nichts schreiben und nichts laden; ein wartender Anlass unterbleibt.
    Abbrechen,
}

/// Die Stelle jeder Antwort im Bauplan; die eine Rueckrechnung von der
/// gedrueckten Stelle auf [`Antwort`].
///
/// Eine Stelle, die zu keiner Schaltflaeche gehoert, faellt auf
/// [`Antwort::Abbrechen`]: das ist die Stelle, die [`super::abbruchstelle`]
/// fuer dieses Blatt liefert, und ein unbekannter Ausgang verliert so nichts.
#[must_use]
fn antwort_an(stelle: usize) -> Antwort {
    match stelle {
        0 => Antwort::NeuLaden,
        1 => Antwort::Ueberschreiben,
        _ => Antwort::Abbrechen,
    }
}

/// Die drei Schaltflaechen, in bindender Reihenfolge; als reine Funktion,
/// damit der Bauplan ohne AppKit pruefbar ist.
#[must_use]
fn schaltflaechen() -> [Schaltflaeche<'static>; 3] {
    [
        Schaltflaeche::neu(
            text(Text::FremdaenderungNeuLaden),
            Taste::EingabeMitWahl,
            Wirkung::Ausfuehren,
        ),
        Schaltflaeche::neu(
            text(Text::FremdaenderungUeberschreiben),
            Taste::EingabeMitBefehl,
            Wirkung::Ausfuehren,
        ),
        Schaltflaeche::neu(
            text(Text::BlattAbbrechen),
            Taste::Eingabe,
            Wirkung::Liegenlassen,
        ),
    ]
}

/// Zeigt das Blatt und meldet die Wahl des Nutzers.
///
/// `datei` ist die Datei, die der Editor haelt und die sich ausserhalb von KRK
/// geaendert hat. `fertig` laeuft auf dem Hauptfaden und genau einmal.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    datei: &Path,
    fertig: impl Fn(Antwort) + 'static,
) -> Blattgriff {
    let name = datei.file_name().map_or_else(
        || datei.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    let blatt = Blatt::mit_schaltflaechen(
        mtm,
        &satz(Text::FremdaenderungFrage, &[("name", &name)]),
        &schaltflaechen(),
    );
    blatt.erlaeuterung_setzen(&satz(
        Text::FremdaenderungErlaeuterung,
        &[("pfad", &datei.display())],
    ));
    blatt.zeigen_mit_wahl(fenster, move |stelle, _fuer_alle| {
        fertig(antwort_an(stelle));
    })
}

#[cfg(test)]
mod tests {
    use crate::appkit::blaetter::{abbruchstelle, bestaetigungsstelle, wahlstelle};

    use super::{Antwort, Taste, Wirkung, antwort_an, schaltflaechen};

    /// Die drei Schaltflaechen stehen in der Reihenfolge des Nutzerentscheids,
    /// und die Rueckrechnung liest dieselbe Reihenfolge.
    #[test]
    fn der_bauplan_zaehlt_neu_laden_ueberschreiben_abbrechen() {
        let schaltflaechen = schaltflaechen();
        let tafel: [(&str, Taste, Wirkung, Antwort); 3] = [
            (
                "Neu laden",
                Taste::EingabeMitWahl,
                Wirkung::Ausfuehren,
                Antwort::NeuLaden,
            ),
            (
                "Trotzdem überschreiben",
                Taste::EingabeMitBefehl,
                Wirkung::Ausfuehren,
                Antwort::Ueberschreiben,
            ),
            (
                "Abbrechen",
                Taste::Eingabe,
                Wirkung::Liegenlassen,
                Antwort::Abbrechen,
            ),
        ];
        assert_eq!(schaltflaechen.len(), tafel.len());
        for (stelle, (titel, taste, wirkung, antwort)) in tafel.into_iter().enumerate() {
            assert_eq!(schaltflaechen[stelle].titel, titel, "Stelle {stelle}");
            assert_eq!(schaltflaechen[stelle].taste, taste, "„{titel}“");
            assert_eq!(schaltflaechen[stelle].wirkung, wirkung, "„{titel}“");
            assert_eq!(antwort_an(stelle), antwort, "„{titel}“");
        }
    }

    /// Weder die Eingabetaste noch eine unbekannte Antwort verliert etwas:
    /// beide fallen auf „Abbrechen“, und die Wahltaste mit Eingabe gehoert
    /// dem Neuladen.
    #[test]
    fn die_eingabetaste_und_eine_unbekannte_antwort_brechen_ab() {
        let schaltflaechen = schaltflaechen();
        let liegen = abbruchstelle(&schaltflaechen);
        assert_eq!(antwort_an(liegen), Antwort::Abbrechen);
        assert_eq!(
            antwort_an(bestaetigungsstelle(&schaltflaechen)),
            Antwort::Abbrechen
        );
        assert_eq!(antwort_an(usize::MAX), Antwort::Abbrechen);
        assert_eq!(
            wahlstelle(&schaltflaechen).map(antwort_an),
            Some(Antwort::NeuLaden)
        );
    }
}
