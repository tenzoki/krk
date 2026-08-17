//! Die Nachfrage vor einem Anlass, der den ungesicherten Stand des Editors
//! verlieren wuerde (C4 der Editor-Runde).
//!
//! Drei Wahlmoeglichkeiten, wie der Nutzer sie am 260807-2139 festgelegt hat:
//! sichern, verwerfen, abbrechen. Das dritte Abnahmekriterium von C4 nennt sie
//! namentlich und in dieser Reihenfolge.
//!
//! # Dieses Blatt rechnet nichts und kennt keinen der drei Anlaesse
//!
//! Es fragt und antwortet. Welche Handlung auf die Antwort folgt, traegt die
//! Schliessung, die der Aufrufer mitgibt — dasselbe Muster, das jeder
//! Blattaufrufer der Runde 1 faehrt. Ein Feld, das eine noch nicht ausgefuehrte
//! Absicht ueber den Rueckruf hinaus haelt, entsteht damit nicht: die Absicht
//! reist in der Schliessung mit und faellt mit ihr.
//!
//! Deshalb kommt die Antwort auch als [`Antwort`] heraus und nicht als die
//! Stelle der gedrueckten Schaltflaeche. Ein `usize` zwaenge jeden Aufrufer,
//! die Reihenfolge der Schaltflaechen ein zweites Mal zu kennen, und die erste
//! Abweichung zwischen zwei von ihnen faende keine Pruefung.
//!
//! # Warum die Eingabetaste auf "Sichern" liegt
//!
//! Anders als bei der Rueckfrage vor dem endgueltigen Loeschen ist die
//! bewahrende Antwort hier zugleich die erste: ein reflexhaftes Bestaetigen mit
//! der Eingabetaste schreibt die Datei und verliert nichts. Verwerfen kostet
//! deshalb die Zusatztaste, und der erlaeuternde Text nennt alle drei Wege, wie
//! das Konfliktblatt es tut. Ohne diese Zeile waeren sie unauffindbar: ein
//! `NSButton` traegt genau eine Tastenentsprechung, und der Tabulator erreicht
//! die Schaltflaechen nur bei eingeschalteter vollstaendiger
//! Tastaturnavigation.
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

use super::{Blatt, Blattgriff, Schaltflaeche, Taste, Wirkung};

/// Was der Nutzer auf die Nachfrage geantwortet hat (C4).
///
/// **Drei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig.**
/// Sie sind die drei Wahlmoeglichkeiten der Festlegung vom 260807-2139; ein
/// vierter Wert haelt bei jedem Aufrufer den Bau an und erzwingt die Antwort
/// darauf, was er bedeutet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Antwort {
    /// Zuerst schreiben, dann den Anlass ausfuehren.
    ///
    /// Scheitert das Schreiben, unterbleibt der Anlass; das neunte
    /// Abnahmekriterium von C4 verlangt es, und der Aufrufer sieht es am
    /// Ausgang des Sicherns.
    Sichern,
    /// Den Anlass ausfuehren; der ungesicherte Stand faellt dabei.
    Verwerfen,
    /// Der Anlass unterbleibt, und der Stand bleibt stehen.
    Abbrechen,
}

/// Zeigt die Nachfrage und meldet die Wahl des Nutzers.
///
/// `datei` ist die Datei, deren Stand auf dem Spiel steht, also die, die der
/// Editor haelt — und nicht die, die er aufnehmen soll. Ihr Name steht in der
/// Frage, weil ein Blatt ohne Namen den Nutzer suchen laesst.
///
/// `fertig` laeuft auf dem Hauptfaden und genau einmal.
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
        &format!("„{name}“ hat ungesicherte Änderungen"),
        &[
            Schaltflaeche::neu("Sichern", Taste::Eingabe, Wirkung::Ausfuehren),
            Schaltflaeche::neu("Verwerfen", Taste::EingabeMitBefehl, Wirkung::Ausfuehren),
            Schaltflaeche::neu("Abbrechen", Taste::Escape, Wirkung::Liegenlassen),
        ],
    );
    blatt.erlaeuterung_setzen(&format!(
        "{}\n\nReturn sichert, Cmd+Return verwirft die Änderungen, Esc bricht ab.",
        datei.display()
    ));

    blatt.zeigen_mit_wahl(fenster, move |stelle, _fuer_alle| {
        // Der Auffangzweig faengt die abbrechende Stelle und nichts sonst: eine
        // Antwort, die zu keiner Schaltflaeche gehoert, kommt hier als
        // `Wirkung::Liegenlassen`-Stelle an, also als die 2. Welche das ist,
        // entscheidet `blaetter::abbruchstelle` einmal fuer jedes Blatt.
        let antwort = match stelle {
            0 => Antwort::Sichern,
            1 => Antwort::Verwerfen,
            _ => Antwort::Abbrechen,
        };
        fertig(antwort);
    })
}
