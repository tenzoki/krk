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
//!
//! **Eine SQLite-Datenbank ist das fuenfte Format** (`.sqlite`, `.sqlite3`,
//! `.db`, `.db3`), allein lesbar, gelesen ueber `archiv::sqlite`
//! (Entscheid
//! `261023-1340-welche-dateien-gelten-als-sqlite-datenbank.md`, Moeglichkeit 2,
//! im Arbeitspaket
//! `261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen`).
//! Auch hier entscheidet allein die Endung: eine `.db`, die keine Datenbank
//! ist (`Thumbs.db`, Berkeley DB), wird betreten und meldet dort, dass sie
//! keine lesbare SQLite-Datenbank ist (`Text::DatenbankUngueltig` mit dem
//! Grund `Text::SqliteKeineDatenbank`).
//!
//! **Welche Meldung ein Befehl bekommt, der im Format nicht wirkt, folgt dem
//! Format und nicht dem Rufer**: [`Archivformat::nicht_verfuegbar_text`] und
//! [`Archivformat::nur_lesbar_text`] antworten je ohne Auffangzweig, und jeder
//! Melder fragt sie, statt einen Text fuer „Archiv“ fest zu nennen. Dasselbe
//! gilt fuer den ganzen Satz der Statuszeile
//! ([`Archivformat::allein_lesbar_satz`]) und fuer die Meldung eines Lesers,
//! der die Datei nicht lesen kann ([`Archivformat::ungueltig_text`]).

use std::ffi::OsStr;

use crate::sprache::Text;

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
    /// `.sqlite`, `.sqlite3`, `.db` und `.db3`: eine SQLite-Datenbank, deren
    /// Tabellen und Sichten an der Wurzel stehen. Allein lesbar.
    Sqlite,
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
        } else if [".sqlite", ".sqlite3", ".db", ".db3"]
            .iter()
            .any(|endung| name.ends_with(endung))
        {
            Some(Archivformat::Sqlite)
        } else {
            None
        }
    }

    /// Ob KRK in ein Archiv dieses Formats schreiben darf.
    ///
    /// Eine gzip-Datei fasst genau einen Strom, und jedes Hineinkopieren,
    /// Umbenennen, Anlegen oder Loeschen machte daraus etwas anderes als die
    /// Datei, die der Nutzer betreten hat (E6 des Plans). Eine Datenbank
    /// oeffnet KRK allein lesend (Modulkopf von `archiv::sqlite`).
    #[must_use]
    pub fn beschreibbar(self) -> bool {
        match self {
            Archivformat::Zip | Archivformat::Tar | Archivformat::TarGz => true,
            Archivformat::Gz | Archivformat::Sqlite => false,
        }
    }

    /// Der Satz fuer einen Befehl, der allein an einem lokalen Ordner wirkt,
    /// an einem Ort in diesem Format (`ort::nicht_verfuegbar_text`).
    #[must_use]
    pub fn nicht_verfuegbar_text(self) -> Text {
        match self {
            Archivformat::Zip | Archivformat::Tar | Archivformat::TarGz | Archivformat::Gz => {
                Text::ImArchivNichtVerfuegbar
            }
            Archivformat::Sqlite => Text::InDatenbankNichtVerfuegbar,
        }
    }

    /// Der Grund fuer einen Eintrag, der in ein Archiv dieses Formats
    /// geschrieben oder aus ihm entfernt werden soll, obwohl es nicht
    /// [`beschreibbar`](Self::beschreibbar) ist. Fuer ein beschreibbares
    /// Format antwortet die Funktion trotzdem, mit dem Text des Archivs, weil
    /// die Unterscheidung vollstaendig ist; gemeldet wird er dort nie.
    #[must_use]
    pub fn nur_lesbar_text(self) -> Text {
        match self {
            Archivformat::Zip | Archivformat::Tar | Archivformat::TarGz | Archivformat::Gz => {
                Text::ArchivNurLesbar
            }
            Archivformat::Sqlite => Text::DatenbankNurLesbar,
        }
    }

    /// Der ganze Satz der Statuszeile, wenn ein Befehl in einen Ort dieses
    /// Formats schreiben soll, den KRK allein lesen darf; gross begonnen und
    /// mit Punkt. [`nur_lesbar_text`](Self::nur_lesbar_text) ist dagegen der
    /// Grund derselben Lage in der Abschlussliste, ein Satzteil. Fuer ein
    /// beschreibbares Format antwortet die Funktion mit dem Satz des Archivs,
    /// aus dem Grund von `nur_lesbar_text`.
    #[must_use]
    pub fn allein_lesbar_satz(self) -> Text {
        match self {
            Archivformat::Zip | Archivformat::Tar | Archivformat::TarGz | Archivformat::Gz => {
                Text::ArchivAlleinLesbar
            }
            Archivformat::Sqlite => Text::DatenbankAlleinLesbar,
        }
    }

    /// Der Grund in der Abschlussliste, wenn sich die Datei seit dem Lesen
    /// ihres Inhaltsverzeichnisses geaendert hat (`Lesefehler::VonAussenVeraendert`),
    /// in einer Datenbank etwa, weil ein Schreiber waehrend des Lesens
    /// uebergeben hat.
    #[must_use]
    pub fn beim_lesen_veraendert_text(self) -> Text {
        match self {
            Archivformat::Zip | Archivformat::Tar | Archivformat::TarGz | Archivformat::Gz => {
                Text::ArchivBeimLesenVeraendert
            }
            Archivformat::Sqlite => Text::DatenbankBeimLesenVeraendert,
        }
    }

    /// Der Grund in der Abschlussliste, wenn ein Eintrag nicht mehr dort
    /// steht, wo das Inhaltsverzeichnis ihn nennt (`Lesefehler::Fehlt`), in
    /// einer Datenbank also eine Tabelle oder Sicht, die es nicht mehr gibt.
    #[must_use]
    pub fn eintrag_fehlt_text(self) -> Text {
        match self {
            Archivformat::Zip | Archivformat::Tar | Archivformat::TarGz | Archivformat::Gz => {
                Text::ArchivEintragFehlt
            }
            Archivformat::Sqlite => Text::DatenbankEintragFehlt,
        }
    }

    /// Die Meldung, wenn der Leser des Formats die Datei nicht lesen kann,
    /// etwa eine `.db`, die keine SQLite-Datenbank ist. Die Vorlage des
    /// Archivs nennt `{datei}`, `{format}` und `{grund}`, die der Datenbank
    /// allein `{datei}` und `{grund}`; der Rufer reicht alle drei.
    #[must_use]
    pub fn ungueltig_text(self) -> Text {
        match self {
            Archivformat::Zip | Archivformat::Tar | Archivformat::TarGz | Archivformat::Gz => {
                Text::ArchivUngueltig
            }
            Archivformat::Sqlite => Text::DatenbankUngueltig,
        }
    }
}
