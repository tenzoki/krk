//! Namen: pruefen, einen freien finden, einen Eintrag umbenennen.
//!
//! Drei Dinge, die alle an einem Dateinamen haengen, und deshalb an einer
//! Stelle:
//!
//! ```text
//! name_pruefen  ──> anlegen.rs (Ordner und Datei anlegen)
//!               ──> umbenennen (hier)
//!               ──> spaeter: umbenennen im Stapel (S17)
//! freier_name   ──> fortschritt.rs (Konfliktregel "automatisch umbenennen")
//! umbenennen    ──> spaeter: umbenennen im Stapel (S17), je Eintrag
//! ```
//!
//! Der Stapel aus S17 fuehrt [`umbenennen`] je Eintrag aus; ein zweiter
//! Umbenennungsweg daneben entsteht nicht.

use std::io;
use std::path::{Path, PathBuf};

use crate::verzeichnis::sys::im_datentraeger_verschieben;

/// Was an einem Namen nicht stimmt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namensfehler {
    /// Der Name ist leer oder besteht nur aus Leerzeichen.
    Leer,
    /// Der Name enthaelt einen Schraegstrich und benennt damit einen Pfad.
    Schraegstrich,
    /// Der Name enthaelt ein Nullbyte.
    Nullbyte,
    /// Der Name ist `.` oder `..` und benennt damit keinen neuen Eintrag.
    Punktname,
}

impl Namensfehler {
    /// Der Grund im Klartext, so wie ihn die Oberflaeche zeigt.
    pub fn grund(self) -> &'static str {
        match self {
            Namensfehler::Leer => "der Name ist leer",
            Namensfehler::Schraegstrich => "ein Name darf keinen Schraegstrich enthalten",
            Namensfehler::Nullbyte => "ein Name darf kein Nullbyte enthalten",
            Namensfehler::Punktname => "'.' und '..' sind keine Namen",
        }
    }
}

impl From<Namensfehler> for io::Error {
    fn from(fehler: Namensfehler) -> Self {
        io::Error::new(io::ErrorKind::InvalidInput, fehler.grund())
    }
}

/// Prueft einen Namen, bevor er ins Dateisystem geht.
///
/// Die Pruefung ist bewusst knapp: sie faengt ab, was kein Name ist, und nicht,
/// was ein ungewoehnlicher Name ist. Ein Name mit Doppelpunkt, ein Name mit
/// Zeilenumbruch und ein Name aus 250 Zeichen sind unter macOS zulaessig, und
/// ein Dateimanager, der sie verbietet, kann Ordner nicht mehr abbilden, die
/// eine andere Anwendung angelegt hat.
pub fn name_pruefen(name: &str) -> Result<(), Namensfehler> {
    if name.trim().is_empty() {
        return Err(Namensfehler::Leer);
    }
    if name.contains('/') {
        return Err(Namensfehler::Schraegstrich);
    }
    if name.contains('\0') {
        return Err(Namensfehler::Nullbyte);
    }
    if name == "." || name == ".." {
        return Err(Namensfehler::Punktname);
    }
    Ok(())
}

/// Benennt einen Eintrag um, im selben Ordner.
///
/// Ein vorhandener Eintrag desselben Namens wird **nicht** ueberschrieben: der
/// Aufruf scheitert mit [`io::ErrorKind::AlreadyExists`]. Wer ueberschreiben
/// will, raeumt das Ziel vorher weg.
pub fn umbenennen(pfad: &Path, neuer_name: &str) -> io::Result<PathBuf> {
    name_pruefen(neuer_name)?;
    let ziel = pfad.with_file_name(neuer_name);
    if ziel == pfad {
        return Ok(ziel);
    }
    im_datentraeger_verschieben(pfad, &ziel, true)?;
    Ok(ziel)
}

/// Wie oft ein freier Name hoechstens gesucht wird, bevor der Versuch
/// aufgegeben wird.
///
/// Die Grenze ist keine Vorsichtsmassnahme gegen einen erwarteten Fall, sondern
/// gegen eine Endlosschleife, falls das Dateisystem jeden Namen als vorhanden
/// meldet. Wer 1.000 Kopien desselben Namens in einem Ordner hat, hat ein
/// anderes Problem als den Namen der 1.001.
const HOECHSTE_KOPIE: u32 = 1_000;

/// Findet einen freien Namen neben einem belegten Ziel.
///
/// `bericht.txt` wird zu `bericht Kopie.txt`, dann `bericht Kopie 2.txt`. Die
/// Endung bleibt hinten, damit die Kopie einer Textdatei eine Textdatei bleibt.
pub fn freier_name(ziel: &Path) -> String {
    let Some(name) = ziel.file_name().and_then(|teil| teil.to_str()) else {
        return "Kopie".to_owned();
    };
    let (stamm, endung) = namen_teilen(name);

    for nummer in 1..=HOECHSTE_KOPIE {
        let vorschlag = if nummer == 1 {
            format!("{stamm} Kopie{endung}")
        } else {
            format!("{stamm} Kopie {nummer}{endung}")
        };
        if !ziel.with_file_name(&vorschlag).exists() {
            return vorschlag;
        }
    }
    format!("{stamm} Kopie {HOECHSTE_KOPIE}{endung}")
}

/// Teilt einen Namen in Stamm und Endung, einschliesslich des Punktes.
///
/// **Die eine Stelle, die diese Frage beantwortet.** Bis zum 260805 zog
/// `crate::stapelumbenennen::regel` fuer die fortlaufende Nummerierung dieselbe
/// Trennung ein zweites Mal, ueber `Path::file_stem` und `Path::extension`
/// (`issues/260804-2040_c_die-trennung-von-stamm-und-endung-steht-an-zwei-stellen.md`).
/// Beide lieferten dasselbe, und genau das ist die Lage, in der eine Abweichung
/// spaeter unbemerkt entsteht.
///
/// Geblieben ist diese Rechnung und nicht der Weg ueber `Path`, weil sie
/// geliehene Ausschnitte liefert: `Path::extension` streicht den Punkt, und ihn
/// wieder anzusetzen kostete je Aufruf eine `String`. Dass die beiden auf jedem
/// Namen uebereinstimmen, den KRK ueberhaupt antreffen kann, haelt die Pruefung
/// `die_trennung_stimmt_mit_der_trennung_von_path_ueberein` fest.
///
/// Ein fuehrender Punkt zaehlt nicht als Endung: `.gitignore` ist ein Stamm
/// ohne Endung und nicht eine Endung ohne Stamm.
pub fn namen_teilen(name: &str) -> (&str, &str) {
    match name.rfind('.') {
        Some(stelle) if stelle > 0 => name.split_at(stelle),
        _ => (name, ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ein_leerer_name_geht_nicht_durch() {
        assert_eq!(name_pruefen(""), Err(Namensfehler::Leer));
        assert_eq!(name_pruefen("   "), Err(Namensfehler::Leer));
    }

    #[test]
    fn ein_name_mit_schraegstrich_geht_nicht_durch() {
        assert_eq!(name_pruefen("a/b"), Err(Namensfehler::Schraegstrich));
    }

    #[test]
    fn punkt_und_doppelpunkt_sind_keine_namen() {
        assert_eq!(name_pruefen("."), Err(Namensfehler::Punktname));
        assert_eq!(name_pruefen(".."), Err(Namensfehler::Punktname));
    }

    #[test]
    fn ein_gewoehnlicher_name_geht_durch() {
        assert_eq!(name_pruefen("Bericht 2026.md"), Ok(()));
        assert_eq!(name_pruefen(".gitignore"), Ok(()));
    }

    #[test]
    fn die_endung_bleibt_hinten() {
        assert_eq!(namen_teilen("bericht.txt"), ("bericht", ".txt"));
        assert_eq!(namen_teilen("archiv.tar.gz"), ("archiv.tar", ".gz"));
        assert_eq!(namen_teilen("liesmich"), ("liesmich", ""));
        assert_eq!(namen_teilen(".gitignore"), (".gitignore", ""));
    }

    /// Die Rechtfertigung dafuer, dass [`namen_teilen`] die zweite Trennung
    /// abgeloest hat und nicht umgekehrt.
    ///
    /// Die abgeloeste ging ueber `Path::file_stem` und `Path::extension`, mit
    /// der Begruendung, so zu trennen wie die Standardbibliothek. Diese
    /// Pruefung haelt fest, dass genau das weiterhin geschieht: sie rechnet
    /// beide Wege nebeneinander und vergleicht sie.
    ///
    /// `..` steht bewusst nicht in der Liste. Dort weichen die beiden ab
    /// (`namen_teilen` liefert `(".", ".")`, `Path` liefert `("..", "")`), und
    /// `..` ist kein Name, den KRK antreffen kann: [`name_pruefen`] weist ihn
    /// als [`Namensfehler::Punktname`] ab, bevor er irgendwo ankommt. Die
    /// Pruefung darueber haelt das fest.
    #[test]
    fn die_trennung_stimmt_mit_der_trennung_von_path_ueberein() {
        for name in [
            "bericht.txt",
            "archiv.tar.gz",
            ".gitignore",
            "liesmich",
            "datei.",
            "..foo",
            ".",
            "a.b.",
            ".x.y",
            "",
        ] {
            let pfad = Path::new(name);
            let stamm = pfad
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or(name);
            let endung = pfad
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .map_or_else(String::new, |endung| format!(".{endung}"));
            assert_eq!(
                namen_teilen(name),
                (stamm, endung.as_str()),
                "bei \"{name}\" trennt namen_teilen anders als Path"
            );
        }
    }

    #[test]
    fn der_erste_freie_name_traegt_kein_zaehlwerk() {
        // In einem Ordner, den es nicht gibt, ist jeder Name frei.
        let vorschlag = freier_name(Path::new("/tmp/krk-gibt-es-nicht-4711/bericht.txt"));
        assert_eq!(vorschlag, "bericht Kopie.txt");
    }
}
