//! Verzeichnisleser und Ordnermodell.
//!
//! Acht Module, in der Reihenfolge, in der die Daten sie durchlaufen:
//!
//! ```text
//! sys ──> leser ──> eintrag ──> modell <── sortierung
//!  │                     ^         ^ ^
//!  │               kollation       │ │
//!  └──> durchlauf ─────────────────┘ │
//!             ^                      │
//!             └──── filter ──────────┘
//! ```
//!
//! [`sys`] ist die einzige Stelle im Kern mit einem Fremdaufruf und bindet
//! `getattrlistbulk(2)` fuer das Lesen, seit Schritt 15 `copyfile(3)` und
//! `renamex_np(2)` fuer die Operationsmaschine und seit dem Defekt
//! `260809-1652` `fcntl(2)` fuer `ohne_warten_oeffnen`, den gemeinsamen Eingang
//! von `text::datei::oeffnen` und, seit dem Defekt `260810-1247`, vom Leseweg
//! der Vorschau in `krk-ui`, und seit der Runde 7 `flock(2)` fuer die beiden
//! Sperren der Ablage. Das sind fuenf Schnittstellen und neun gebundene
//! Funktionen, denn `copyfile(3)` braucht seine vier
//! `copyfile_state_*`-Helfer. [`leser`] macht aus der ersten der fuenf
//! Schnittstellen den gestueckelten Lesevorgang auf einem Arbeitsfaden. [`eintrag`] beschreibt, was ein Eintrag traegt, und
//! laesst sich von [`kollation`] die beiden Sortierschluessel bauen.
//! [`modell`] haelt Eintraege und Sichtreihenfolge getrennt, und [`sortierung`]
//! liefert die acht Ordnungen.
//!
//! [`filter`] steht als einziges Modul **unter** zwei anderen und nicht in der
//! Kette: es traegt die zwei Regeln des Filters aus der Runde 10, welche
//! Zeichen aufgenommen werden und wann ein Name den Filtertext traegt, und
//! beide Regeln stehen dort je einmal. Der Vergleich hat zwei Rufer, [`modell`]
//! fuer die angezeigte Zeile und [`durchlauf`] fuer jeden Namen im Unterbaum;
//! zwei Fassungen davon hiessen, dass eine tiefe Suche etwas anderes faende als
//! eine flache. Bis zum 260815 hiess das Modul `sprungmarke` und trug die
//! Sprungmarke aus C2 der Runde 1, die die Runde 10 abgeloest hat.
//!
//! [`durchlauf`] steht neben [`leser`] und nicht unter ihm: er liest ueber
//! dieselbe Huelle `sys::Schwungleser` und auf derselben Bauart, beantwortet
//! aber eine andere Frage. Der Leser liefert den Bestand eines Ordners, der
//! Durchlauf je Ordner des angezeigten Ordners einen Wahrheitswert ueber
//! seinen ganzen Unterbaum. Er ist die fuenfte Eingabe des Pruefschritts in
//! [`modell`], und die einzige, die von aussen kommt.
//!
//! Der Kern kennt AppKit nicht; alles hier ist ohne Fenster testbar.

use std::path::{Path, PathBuf};

pub mod durchlauf;
pub mod eintrag;
pub mod filter;
pub mod kollation;
pub mod leser;
pub mod modell;
pub mod sortierung;
pub mod sys;

pub use durchlauf::{Auftrag, Befundmeldung, Durchlauf};
pub use eintrag::{Eintrag, Typ};
pub use leser::{Abschluss, Lesevorgang, Meldung, STAPELGROESSE, lesen};
pub use modell::{Markierungsstand, Ordnermodell};
pub use sortierung::{Richtung, Schluessel, Sortierung};

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
