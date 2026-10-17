//! Welche Dateien KRK als Archiv betritt, entschieden allein am Namen.
//!
//! **Die Endung entscheidet, und sonst nichts.** Gefragt wird auf dem
//! Hauptfaden, beim Einstieg in eine Zeile; eine Pruefung der ersten Bytes
//! waere dort ein `open` und ein `read` je Einstieg und hielte an einem
//! haengenden Netzlaufwerk die Ereignisschleife an. Ob die Datei wirklich ein
//! solches Archiv ist, sagt erst der Leser des Formats auf dem Arbeitsfaden
//! ([`super::Archivzugang::verzeichnis`]); eine Datei mit falscher Endung wird
//! also betreten und meldet dort ihren Grund, statt still als anderes Format
//! gelesen zu werden (E1 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`).
//!
//! Die Liste ist geschlossen und steht an genau einer Stelle,
//! [`Archivformat::aus_namen`]. Abkoemmlinge des Zip-Formats wie `.jar`,
//! `.docx` oder `.epub` betritt KRK nicht, und `.tar.bz2`, `.tar.xz`, `.7z`
//! und `.rar` fehlen, weil keine Kiste ohne C sie heute im Baum liest.

use std::ffi::OsStr;

/// Ein Archivformat, das KRK lesen kann.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Archivformat {
    /// `.zip`
    Zip,
    /// `.tar`
    Tar,
    /// `.tar.gz` und `.tgz`
    TarGz,
    /// `.gz`, sofern nicht `.tar.gz`: genau eine gepackte Datei.
    Gz,
}

impl Archivformat {
    /// Das Format zu einem Dateinamen, oder `None`, wenn der Name keine der
    /// Endungen traegt.
    ///
    /// Gross- und Kleinschreibung zaehlen nicht. `.tar.gz` steht vor `.gz`,
    /// damit ein gepackter tar-Strom nicht als einzelne Datei gilt. Ein Name,
    /// der kein UTF-8 ist, ist kein Archiv: jede der Endungen ist ASCII, und
    /// ein solcher Name laesst sich im Fenster ohnehin nur ersetzt zeigen.
    /// Stellt keinen Systemaufruf.
    #[must_use]
    pub fn aus_namen(name: &OsStr) -> Option<Self> {
        let name = name.to_str()?.to_ascii_lowercase();
        if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
            Some(Archivformat::TarGz)
        } else if name.ends_with(".tar") {
            Some(Archivformat::Tar)
        } else if name.ends_with(".zip") {
            Some(Archivformat::Zip)
        } else if name.ends_with(".gz") {
            Some(Archivformat::Gz)
        } else {
            None
        }
    }

    /// Ob KRK in ein Archiv dieses Formats schreiben darf.
    ///
    /// Eine gzip-Datei fasst genau einen Strom, und jedes Hineinkopieren,
    /// Umbenennen, Anlegen oder Loeschen machte daraus etwas anderes als die
    /// Datei, die der Nutzer betreten hat (E6 des Plans).
    #[must_use]
    pub fn beschreibbar(self) -> bool {
        match self {
            Archivformat::Zip | Archivformat::Tar | Archivformat::TarGz => true,
            Archivformat::Gz => false,
        }
    }
}
