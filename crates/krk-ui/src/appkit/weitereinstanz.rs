//! Die eine Beruehrung mit dem System, die C3 der Runde 7 braucht.
//!
//! ```text
//! NSBundle::mainBundle ──> bundleURL ──> endet auf .app?
//!                                            │  ja
//!                                            ▼
//!               NSWorkspaceOpenConfiguration (createsNewApplicationInstance)
//!                                            │
//!                    openApplicationAtURL:configuration:… ──> LaunchServices
//! ```
//!
//! Die eine Frage dieses Moduls: **wo steckt das laufende KRK, und wie startet
//! man es ein zweites Mal.** Ein eigenes Modul und kein Zusatz zu den
//! Nachbarn, aus demselben Grund, aus dem [`super::standardprogramm`] und
//! [`super::terminal`] getrennt sind: jeder von ihnen beantwortet genau eine
//! Frage, und keiner von beiden stellt diese. `terminal.rs` loest eine
//! **fremde** Buendelkennung aus `settings.toml` auf; hier geht es um das
//! eigene Buendel, und danach fragt im ganzen Baum sonst niemand.
//!
//! # Warum das Merkmal `createsNewApplicationInstance` noetig ist
//!
//! Ohne es aktiviert LaunchServices die schon laufende Instanz und bringt ihr
//! Fenster nach vorn, statt eine zweite zu starten — das ist die gewoehnliche
//! Antwort des Systems auf „oeffne diese Anwendung", und sie ist hier genau
//! die falsche. Das Merkmal ist der eine Schalter, der sie umdreht.
//!
//! # Warum der eigene Ort erfragt und nicht geschrieben wird
//!
//! Gestartet wird das Buendel, in dem die laufende Instanz steckt (C3.5). Ein
//! Pfad im Programmtext waere eine Behauptung darueber, wo KRK liegt, und sie
//! waere falsch, sobald der Nutzer das Buendel verschiebt oder eine zweite
//! Fassung daneben legt. `NSBundle::mainBundle` weiss es.
//!
//! # Ohne Buendel wird nichts gestartet
//!
//! Beim Entwicklungslauf ueber `cargo run` liegt das Programm nicht in einem
//! `.app`, sondern unter `target/debug/`. `bundleURL` liefert dann den Ordner
//! des Programms, und ein Start darauf ergaebe nichts oder etwas Falsches.
//! [`starten`] prueft deshalb die Endung und meldet den Fall, statt ihn zu
//! versuchen (C3.6).
//!
//! # Der Rueckrufparameter bleibt leer
//!
//! Aus demselben Grund wie bei [`super::terminal`]: ein Block auf einer
//! beliebigen Schlange, der auf den Hauptfaden zurueckspringt, waere der zweite
//! asynchrone Weg neben dem Vermittlerfaden der Dateioperationen. Was der
//! Nutzer vom Ergebnis merkt, ist das zweite Fenster; bleibt es aus, sagt das
//! Systemprotokoll mehr, als KRK hier melden koennte.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSBundle`, `NSString` und `NSURL` stehen seit macOS 10.0 zur Verfuegung,
//! ebenso `mainBundle` und `pathExtension`; `bundleURL` seit 10.6.
//! **`NSWorkspaceOpenConfiguration` ist die juengste Klasse dieser Datei und
//! steht seit macOS 10.15** (`NSWorkspace.h`, `API_AVAILABLE(macos(10.15))`
//! ueber dem `@interface`), ihre Bauform `configuration` und die Eigenschaft
//! `createsNewApplicationInstance` mit ihr. `NSWorkspace` und
//! `sharedWorkspace` stehen seit 10.0,
//! `openApplicationAtURL:configuration:completionHandler:` seit 10.15. Das
//! Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use objc2::rc::Retained;
use objc2_app_kit::{NSWorkspace, NSWorkspaceOpenConfiguration};
use objc2_foundation::{NSBundle, NSURL};

/// Der Satz, den KRK meldet, wenn es nicht aus einem Buendel laeuft (C3.6).
///
/// Er steht als Konstante da und nicht als Zeichenkette im Rumpf, damit die
/// Probe ihn nennen kann, ohne ihn abzuschreiben: [`starten`] selbst laesst
/// sich ohne laufendes Buendel nicht pruefen.
pub const OHNE_BUENDEL: &str =
    "KRK laeuft nicht aus einem Buendel; eine weitere Instanz startet nur das gebaute KRK.app";

/// Der Ort des Buendels, in dem die laufende Instanz steckt.
///
/// `None`, wenn der Ort keine `.app`-Endung traegt. Die eine Stelle des
/// Programms, die den eigenen Buendelort bestimmt.
///
/// **Liefert die `NSURL` und keinen `PathBuf`.** Bis zur Runde 7 baute sie
/// einen Pfad, den niemand benutzte: [`starten`] brauchte von ihr allein die
/// Ja-Nein-Antwort und fragte `NSBundle::mainBundle().bundleURL()` fuer den
/// Start selbst ein zweites Mal. Dabei fiel ein Nebenausgang falsch aus — ohne
/// uebersetzbaren Pfad meldete KRK „laeuft nicht aus einem Buendel", obwohl es
/// das tat
/// (`issues/260813-0540_*_weitereinstanz-fragt-den-buendelort-zweimal-und-wirft-die-antwort-weg.md`).
/// Jetzt entscheidet allein die Endung, und der Ort wird einmal bestimmt.
fn eigenes_buendel() -> Option<Retained<NSURL>> {
    let adresse = NSBundle::mainBundle().bundleURL();
    let endung = adresse.pathExtension()?;
    if endung.to_string() != "app" {
        return None;
    }
    Some(adresse)
}

/// Startet eine weitere Instanz von KRK (C3.1, C3.5, C3.6).
///
/// Liefert `None`, wenn der Start angestossen wurde, und den Satz fuer die
/// Statuszeile, wenn nichts gestartet worden ist. Was danach geschieht,
/// entscheidet LaunchServices; der Rueckruf bleibt leer, siehe den Modulkopf.
pub fn starten() -> Option<&'static str> {
    let Some(adresse) = eigenes_buendel() else {
        return Some(OHNE_BUENDEL);
    };
    let einstellung = NSWorkspaceOpenConfiguration::configuration();
    // Der eine Schalter, der LaunchServices davon abhaelt, statt einer zweiten
    // Instanz die laufende nach vorn zu holen.
    einstellung.setCreatesNewApplicationInstance(true);
    NSWorkspace::sharedWorkspace().openApplicationAtURL_configuration_completionHandler(
        &adresse,
        &einstellung,
        None,
    );
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Der Pfad kommt vom System und nicht aus dem Programmtext (C3.5).
    ///
    /// **Was diese Probe zeigt und was nicht.** Unter `cargo test` laeuft KRK
    /// nicht aus einem Buendel, und [`eigenes_buendel`] liefert deshalb `None`;
    /// dass der Ort im Buendel richtig herauskommt, sieht der Nutzer am
    /// laufenden `KRK.app`. Nachweisbar ist hier die andere Haelfte, und sie
    /// ist die, an der ein Fehler unbemerkt bliebe: es gibt im Baum keine
    /// zweite Antwort auf die Frage nach dem eigenen Ort. Gezaehlt werden
    /// Erklaerungen und nicht Aufrufer; die Unterscheidung steht in
    /// [`crate::quellbaum`].
    #[test]
    fn der_eigene_buendelort_wird_an_genau_einer_stelle_bestimmt() {
        let nadel = concat!("mainBundle", "()");
        let dateien: Vec<String> = crate::quellbaum::quelldateien()
            .into_iter()
            .filter(|(_, inhalt)| inhalt.contains(nadel))
            .map(|(name, _)| name)
            .collect();
        assert_eq!(
            dateien,
            vec!["krk-ui/src/appkit/weitereinstanz.rs".to_owned()],
            "der eigene Buendelort wird an mehr als einer Stelle bestimmt"
        );
    }

    /// Ohne Buendel wird nichts gestartet, und der Satz sagt es (C3.6).
    ///
    /// Der Probenlauf ist genau dieser Fall: das Programm liegt unter
    /// `target/debug/deps/` und in keinem `.app`.
    #[test]
    fn ohne_buendel_gibt_es_keinen_ort_und_einen_satz() {
        assert!(
            eigenes_buendel().is_none(),
            "der Probenlauf steckt unerwartet in einem Buendel"
        );
        assert!(OHNE_BUENDEL.contains("Buendel"));
    }
}
