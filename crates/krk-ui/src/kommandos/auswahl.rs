//! Die Mehrfachauswahl aus C2, soweit sie mehr ist als Zustand.
//!
//! Drei der vier Markierungsbefehle sind reiner Zustand des Ordnermodells und
//! stehen dort: alles markieren, jede Markierung aufheben, die Markierung
//! umkehren. Der vierte ist eine Verbindung aus zweien, "markieren **und**
//! weiterruecken", und braucht dafuer die Zeilen. Er steht hier.

use krk_core::verzeichnis::Ordnermodell;

use super::navigation::{Bewegung, zielzeile};

/// Markiert den Eintrag der genannten Zeile und rueckt weiter (C2).
///
/// Liefert die Zeile, auf die die Auswahl danach steht. In der letzten Zeile
/// bleibt sie stehen, wie bei jeder anderen Bewegung auch: der Eintrag ist
/// markiert, und ein Umlauf an den Listenanfang begaenne eine zweite Runde, in
/// der jeder Tastendruck die Markierung wieder abraeumte.
pub fn markieren_und_weiter(modell: &mut Ordnermodell, zeile: usize) -> Option<usize> {
    let eintrag = modell.eintragsindex(zeile)?;
    modell.markierung_umschalten(eintrag);
    let jetzt = isize::try_from(zeile).ok()?;
    zielzeile(Bewegung::Um(1), jetzt, modell.zeilenzahl())
}
