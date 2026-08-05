//! Verzeichnisleser und Ordnermodell.
//!
//! Sechs Module, in der Reihenfolge, in der die Daten sie durchlaufen:
//!
//! ```text
//! sys  ──> leser ──> eintrag ──> modell <── sortierung
//!                                  ^
//!                                  └── sprungmarke
//! ```
//!
//! [`sys`] ist die einzige Stelle im Kern mit einem Fremdaufruf und bindet
//! `getattrlistbulk(2)` fuer das Lesen sowie, seit Schritt 15, `copyfile(3)`
//! und `renamex_np(2)` fuer die Operationsmaschine. [`leser`] macht aus dem
//! ersten der drei Aufrufe den gestueckelten Lesevorgang auf
//! einem Arbeitsfaden. [`eintrag`] beschreibt, was ein Eintrag traegt.
//! [`modell`] haelt Eintraege und Sichtreihenfolge getrennt, [`sortierung`]
//! liefert die acht Ordnungen, und [`sprungmarke`] findet einen Eintrag ueber
//! die getippten Anfangsbuchstaben (C2).
//!
//! Der Kern kennt AppKit nicht; alles hier ist ohne Fenster testbar.

use std::path::{Path, PathBuf};

pub mod eintrag;
pub mod leser;
pub mod modell;
pub mod sortierung;
pub mod sprungmarke;
pub mod sys;

pub use eintrag::{Eintrag, Typ};
pub use leser::{Abschluss, Lesevorgang, Meldung, STAPELGROESSE, lesen};
pub use modell::{Markierungsstand, Ordnermodell};
pub use sortierung::{Richtung, Schluessel, Sortierung};
pub use sprungmarke::Sprungmarke;

/// Der uebergeordnete Ordner und der Name des verlassenen (C2).
///
/// C2 verlangt beim Aufstieg, dass die Auswahl auf dem Ordner steht, aus dem
/// der Nutzer gerade kam. Der Name dafuer ist reine Pfadarithmetik und steht
/// deshalb im Kern und nicht in der Oberflaeche: er ist ohne Fenster pruefbar,
/// und `krk-ui` haengt allein die Navigation daran.
///
/// `None` fuer die Wurzel, die keinen uebergeordneten Ordner hat. Ein Aufstieg
/// von `/` ist damit kein Sonderfall mit eigener Meldung, sondern schlicht
/// keine Bewegung.
pub fn aufwaerts(ordner: &Path) -> Option<(PathBuf, String)> {
    let name = ordner.file_name()?.to_string_lossy().into_owned();
    let eltern = ordner.parent()?;
    Some((eltern.to_path_buf(), name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn der_aufstieg_nennt_den_verlassenen_ordner() {
        let (eltern, name) = aufwaerts(Path::new("/Users/k1/Projekte"))
            .expect("ein Ordner unterhalb der Wurzel hat einen uebergeordneten");
        assert_eq!(eltern, Path::new("/Users/k1"));
        assert_eq!(name, "Projekte");
    }

    #[test]
    fn der_aufstieg_aus_der_wurzel_fuehrt_nirgendwohin() {
        assert_eq!(aufwaerts(Path::new("/")), None);
    }

    #[test]
    fn ein_abschliessender_schraegstrich_aendert_nichts() {
        let (eltern, name) =
            aufwaerts(Path::new("/Users/k1/")).expect("derselbe Ordner, anders geschrieben");
        assert_eq!(eltern, Path::new("/Users"));
        assert_eq!(name, "k1");
    }
}
