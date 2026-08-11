//! Wo die vier Ablagedateien liegen, und wie der Ordner beim ersten Start
//! entsteht.
//!
//! Der Ort ist `~/Library/Application Support/KRK/`, so wie `### Frage 4` des
//! Plans ihn festlegt. Aufgeloest wird er ueber das Benutzerverzeichnis, das
//! [`benutzerverzeichnis`] als einzige Stelle im Kern ermittelt.
//!
//! [`Ablageort`] traegt die Wurzel und nichts weiter. Dass er sich auch auf
//! einen beliebigen Ordner setzen laesst, ist keine Testhintertuer, sondern die
//! Bedingung dafuer, dass die Ablage ueberhaupt ohne Zugriff auf das echte
//! Benutzerverzeichnis pruefbar ist.
//!
//! # Wie KRK dem Nutzer einen Pfad hinschreibt
//!
//! [`gekuerzt_fuer_anzeige`] ist **KRKs Form fuer Meldungen**: das
//! Benutzerverzeichnis erscheint als `~`. Sie steht neben
//! [`benutzerverzeichnis`], weil sie genau dieses eine Verzeichnis abzieht und
//! diese Datei nach ihrem eigenen Modulkopf die einzige Stelle im Kern ist, die
//! danach fragt.
//!
//! **Der Fenstertitel benutzt sie bewusst nicht.** `krk-ui/src/fenstertitel.rs`
//! schreibt den Pfad aus, auf Verlangen des Nutzers vom 260809. KRK traegt
//! damit zwei Formen fuer denselben Pfad an zwei Flaechen desselben Fensters,
//! und das ist gesehen und angenommen: der Nutzer hat die Kuerzung am
//! 260811-0900 gegen die Empfehlung des Plans gewaehlt
//! (`circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260811-0838_*_schreibt-krk-einen-pfad-fuer-den-nutzer-je-gekuerzt.md`).
//! Wer die Ungleichheit aufloesen will, hebt eine der beiden Entscheidungen
//! auf; eine Angleichung im Vorbeigehen ist keine.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Die vier Dateien, die KRK unter `Application Support` ablegt.
///
/// Eine Aufzaehlung statt vier loser Namen: wer alle anfassen muss, laeuft
/// ueber [`Datei::ALLE`] und kann keine vergessen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Datei {
    /// `keymap.toml`: die vollstaendige Belegung des Nutzers.
    ///
    /// Den Inhalt beschreibt Schritt 11, nicht dieser Schritt. Die Ablage
    /// kennt von dieser Datei nur den Namen und den Weg dorthin.
    Belegung,
    /// `bookmarks.toml`: die Lesezeichen, siehe [`super::lesezeichen`].
    Lesezeichen,
    /// `session.toml`: der Sitzungszustand, siehe [`super::sitzung`].
    Sitzung,
    /// `settings.toml`: die von Hand gepflegten Einstellungen (C11), siehe
    /// [`super::einstellungen`].
    ///
    /// Die einzige der vier, die KRK im Betrieb nicht schreibt. Sie entsteht
    /// beim ersten Start aus der eingebetteten Auslieferungsfassung und bleibt
    /// danach dem Nutzer ueberlassen.
    Einstellungen,
}

impl Datei {
    /// Alle vier, in fester Reihenfolge.
    pub const ALLE: [Datei; 4] = [
        Datei::Belegung,
        Datei::Lesezeichen,
        Datei::Sitzung,
        Datei::Einstellungen,
    ];

    /// Der Dateiname unterhalb des Ablageordners.
    pub const fn dateiname(self) -> &'static str {
        match self {
            Datei::Belegung => "keymap.toml",
            Datei::Lesezeichen => "bookmarks.toml",
            Datei::Sitzung => "session.toml",
            Datei::Einstellungen => "settings.toml",
        }
    }
}

/// Die drei Namensteile zwischen Benutzerverzeichnis und Ablageordner.
const UNTERPFAD: [&str; 3] = ["Library", "Application Support", "KRK"];

/// Das Benutzerverzeichnis, falls das System eines nennt.
///
/// Die eine Stelle im Kern, die danach fragt. Zwei Aufrufer haengen daran und
/// gehen mit einem fehlenden Benutzerverzeichnis verschieden um:
/// [`Ablageort::im_benutzerverzeichnis`] scheitert, weil es ohne Wurzel nichts
/// abzulegen gibt, und der Auslieferungszustand der Sitzung weicht auf `/` aus,
/// weil ein Dateifenster einen Ordner zeigen muss.
pub fn benutzerverzeichnis() -> Option<PathBuf> {
    std::env::home_dir()
}

/// Der Pfad in der Form, in der KRK ihn dem Nutzer meldet: `~` statt des
/// Benutzerverzeichnisses.
///
/// **Das Benutzerverzeichnis kommt als Argument herein** und wird hier nicht
/// erfragt. Damit ist die Funktion ohne Zugriff auf das echte
/// Benutzerverzeichnis pruefbar — dieselbe Erwaegung, aus der sich
/// [`Ablageort`] auf einen beliebigen Ordner setzen laesst.
///
/// Vier Faelle, und die Funktion ist ueber sie total:
///
/// - Der Pfad liegt unter dem Benutzerverzeichnis: `~/` und der Rest.
/// - Der Pfad **ist** das Benutzerverzeichnis: `~`.
/// - Der Pfad liegt nicht darunter: ausgeschrieben, unveraendert.
/// - Es wird kein Benutzerverzeichnis uebergeben: ausgeschrieben. Kein Fehler
///   und kein `Option` im Rueckgabewert — ein Pfad ohne etwas zu kuerzen ist
///   kein Scheitern, sondern ein Pfad.
///
/// **Der Vergleich laeuft ueber [`Path::strip_prefix`] und nicht ueber eine
/// Zeichenkette.** `strip_prefix` vergleicht Pfadbestandteile; ein Vergleich
/// auf Bytes machte aus `/Users/kai-alt/Downloads` gegen das
/// Benutzerverzeichnis `/Users/kai` die Antwort `~-alt/Downloads`. Dieser Fall
/// steht als eigene Zusicherung in `krk-core/tests/ablage.rs`.
///
/// Die dritte und die vierte Zeile koennen in der Runde 3 nicht eintreten: das
/// Ziel der Tastenbelegung ist fest der Downloads-Ordner. Gebaut sind sie
/// trotzdem jetzt, weil eine Funktion, die einen Fall nicht kennt, ihn beim
/// ersten Auftreten falsch beantwortet — und der erste Auftritt waere die
/// Runde, die den Zielordner einstellbar macht.
///
/// Ausgeschrieben wird ueber `display()`, also in derselben Form, die
/// `fenstertitel::titel` fuer den Titelbalken erzeugt. Die beiden Flaechen
/// unterscheiden sich damit in genau einer Sache, der Kuerzung.
pub fn gekuerzt_fuer_anzeige(pfad: &Path, benutzerverzeichnis: Option<&Path>) -> String {
    let Some(zuhause) = benutzerverzeichnis else {
        return pfad.display().to_string();
    };
    match pfad.strip_prefix(zuhause) {
        Ok(rest) if rest.as_os_str().is_empty() => "~".to_owned(),
        Ok(rest) => format!("~/{}", rest.display()),
        Err(_) => pfad.display().to_string(),
    }
}

/// Der Ordner, in dem die vier Dateien liegen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ablageort {
    wurzel: PathBuf,
}

impl Ablageort {
    /// Der Ort unter dem Benutzerverzeichnis:
    /// `~/Library/Application Support/KRK/`.
    ///
    /// Legt nichts an; das tut [`Ablageort::anlegen`].
    pub fn im_benutzerverzeichnis() -> io::Result<Self> {
        let Some(zuhause) = benutzerverzeichnis() else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "das System nennt kein Benutzerverzeichnis",
            ));
        };
        let mut wurzel = zuhause;
        for teil in UNTERPFAD {
            wurzel.push(teil);
        }
        Ok(Self { wurzel })
    }

    /// Der Ort an einer frei gewaehlten Wurzel.
    pub fn an(wurzel: impl Into<PathBuf>) -> Self {
        Self {
            wurzel: wurzel.into(),
        }
    }

    /// Der Ablageordner selbst.
    pub fn wurzel(&self) -> &Path {
        &self.wurzel
    }

    /// Der Pfad einer der vier Dateien.
    pub fn datei(&self, welche: Datei) -> PathBuf {
        self.wurzel.join(welche.dateiname())
    }

    /// Legt den Ablageordner an, falls er noch nicht steht.
    ///
    /// Der Aufruf ist wiederholbar: ein vorhandener Ordner ist kein Fehler.
    /// Das ist die Anlage beim ersten Start, und sie kostet danach einen
    /// Systemaufruf je Programmstart.
    pub fn anlegen(&self) -> io::Result<()> {
        fs::create_dir_all(&self.wurzel)
    }
}
