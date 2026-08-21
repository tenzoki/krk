//! Wo die sechs Ablagedateien liegen, in welchen zwei Formaten sie stehen, und
//! wie der Ordner beim ersten Start entsteht.
//!
//! # Zwei Formate, und warum die Zettel kein TOML tragen
//!
//! Vier Dateien tragen TOML und gehen ueber [`super::Zugang::laden`] und
//! [`super::Zugang::sichern`]; die zwei Zetteldateien der Runde 9 tragen den
//! Text des Zettels und sonst nichts. [`Datei::format`] sagt, welche welche
//! ist, und wer beide Sorten verschieden behandeln muss, fragt diese abgeleitete
//! Frage statt eine zweite Liste neben [`Datei::ALLE`] zu fuehren: eine Liste
//! kann auseinanderlaufen, eine vollstaendige Fallunterscheidung nicht.
//!
//! **Der Nutzer hat zwei einzelne Zetteldateien gewaehlt und ausdruecklich
//! keine gemeinsame.** Aus dieser Wahl folgt die Form des Inhalts: eine Datei je
//! Zettel ist nur dann eine Verbesserung gegenueber einer gemeinsamen, wenn sie
//! fuer sich lesbar ist und in einem beliebigen Textprogramm aufgeht. Ein
//! TOML-Rahmen um den Text naehme genau das zurueck und brachte daneben die
//! Frage nach der Behandlung von Sonderzeichen mit, die eine Textdatei nicht
//! kennt.
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

use serde::{Deserialize, Serialize};

/// Welcher der beiden Notizzettel gemeint ist (C2 der Runde 9).
///
/// Das Blatt fuehrt genau zwei, gleichrangig nebeneinander. **Ein eigener Typ
/// statt einer Zahl, damit ein Index nicht versehentlich zu drei Zetteln
/// wird**; dieselbe Erwaegung und dieselbe Bauform wie bei
/// [`Fensterseite`](super::Fensterseite). Damit ist "das Blatt fuehrt genau
/// zwei Zettel" eine Aussage ueber einen Typ und nicht ueber eine Zeile Code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Zettel {
    /// Der Zettel, der ohne weiteres Zutun offen ist.
    #[default]
    Erster,
    /// Der zweite Zettel.
    Zweiter,
}

impl Zettel {
    /// Beide Zettel, der erste zuerst.
    pub const ALLE: [Zettel; 2] = [Zettel::Erster, Zettel::Zweiter];

    /// Die Stelle im Feld der beiden Zettel, etwa fuer die Tabs des Blattes.
    pub const fn index(self) -> usize {
        match self {
            Zettel::Erster => 0,
            Zettel::Zweiter => 1,
        }
    }

    /// Der jeweils andere Zettel. Beim Tabwechsel ist er das Ziel.
    pub const fn andere(self) -> Self {
        match self {
            Zettel::Erster => Zettel::Zweiter,
            Zettel::Zweiter => Zettel::Erster,
        }
    }
}

/// In welchem Format eine Ablagedatei steht.
///
/// **Zwei Werte, vollstaendig und ohne Auffangzweig.** Eine dritte Sorte haelt
/// den Bau an, und zwar an jeder Stelle, die heute nach dem Format fragt: den
/// zwei Gegenproben in [`super::Zugang`] und den Rundlaeufen der Proben.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    /// TOML, gelesen und geschrieben ueber `serde`.
    Toml,
    /// Nackter Text, ohne Kopf und ohne Rahmen.
    Text,
}

/// Was eine dastehende Ablagedatei bedeutet, aus der kein einziger oberster
/// Schluessel kommt.
///
/// **Zwei Werte, vollstaendig und ohne Auffangzweig**, wie [`Format`] daneben.
/// Die Frage ist je Datei zu beantworten und nicht ueber alle zu
/// verallgemeinern: sie haengt daran, ob KRK diese Datei je ohne obersten
/// Schluessel schreibt, und das ist eine Aussage ueber ihren Schreiber und
/// nicht ueber TOML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Leerbefund {
    /// Ein gueltiger Bestand: jedes Feld steht auf seinem Auslieferungswert.
    ///
    /// Der Wert der von Hand gepflegten Dateien und der zwei Zettel. Wer
    /// `keymap.toml` bis auf die Kommentare leerraeumt, meint die
    /// Vorgabebelegung und keinen Schaden; ein leerer Notizzettel ist ein
    /// leerer Notizzettel.
    Vorgabe,
    /// Kein Bestand: die Datei hat nicht hergegeben, was sie traegt.
    ///
    /// Der Wert der Dateien, die KRK selbst schreibt und dabei **immer** mit
    /// mindestens einem obersten Schluessel: eine solche Datei ohne einen
    /// einzigen kann nicht aus KRKs Feder stammen, und sie als ersten Start zu
    /// lesen hiesse, den Bestand beim naechsten Schreibvorgang zu verlieren.
    Beschaedigt,
}

/// Die sechs Dateien, die KRK unter `Application Support` ablegt.
///
/// Eine Aufzaehlung statt sechs loser Namen: wer alle anfassen muss, laeuft
/// ueber [`Datei::ALLE`] und kann keine vergessen. Eine Ablagedatei, die in
/// keiner Aufzaehlung steht, gibt es nicht.
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
    /// `note-1.txt` und `note-2.txt`: die zwei Notizzettel aus C5 der Runde 9.
    ///
    /// **Eine Variante mit Nutzlast und nicht zwei nebeneinander.** Welcher
    /// Zettel gemeint ist, sagt [`Zettel`]; damit traegt die Ablageaufzaehlung
    /// die Zusage "genau zwei Zettel" mit, statt sie ein zweites Mal zu
    /// behaupten.
    ///
    /// Die einzigen beiden, die kein TOML tragen; siehe [`Datei::format`] und
    /// den Modulkopf.
    Zettel(Zettel),
}

impl Datei {
    /// Alle sechs, in fester Reihenfolge: die vier TOML-Dateien, danach die
    /// zwei Zettel.
    pub const ALLE: [Datei; 6] = [
        Datei::Belegung,
        Datei::Lesezeichen,
        Datei::Sitzung,
        Datei::Einstellungen,
        Datei::Zettel(Zettel::Erster),
        Datei::Zettel(Zettel::Zweiter),
    ];

    /// Der Dateiname unterhalb des Ablageordners.
    ///
    /// Die zwei Zettelnamen folgen der englischsprachigen Kleinschreibung der
    /// vier bestehenden; der Bindestrich mit Ziffer ist die knappste Form, zwei
    /// Dateien derselben Art zu unterscheiden.
    pub const fn dateiname(self) -> &'static str {
        match self {
            Datei::Belegung => "keymap.toml",
            Datei::Lesezeichen => "bookmarks.toml",
            Datei::Sitzung => "session.toml",
            Datei::Einstellungen => "settings.toml",
            Datei::Zettel(Zettel::Erster) => "note-1.txt",
            Datei::Zettel(Zettel::Zweiter) => "note-2.txt",
        }
    }

    /// In welchem Format diese Datei steht.
    ///
    /// Die abgeleitete Frage, ueber die sich die beiden Sorten trennen lassen,
    /// ohne eine zweite Liste neben [`Datei::ALLE`] zu fuehren. Die
    /// Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig: eine
    /// siebte Ablagedatei haelt den Bau hier an und erzwingt eine bewusste
    /// Einordnung.
    pub const fn format(self) -> Format {
        match self {
            Datei::Belegung | Datei::Lesezeichen | Datei::Sitzung | Datei::Einstellungen => {
                Format::Toml
            }
            Datei::Zettel(_) => Format::Text,
        }
    }

    /// Was eine dastehende Datei bedeutet, aus der kein einziger oberster
    /// Schluessel kommt.
    ///
    /// Die zweite abgeleitete Frage neben [`Datei::format`], und sie steht aus
    /// demselben Grund hier: vollstaendig, ohne Auffangzweig, damit eine
    /// siebte Ablagedatei den Bau anhaelt und eine bewusste Einordnung
    /// erzwingt. Wer sie beantwortet, beantwortet sie **je Datei** und leitet
    /// sie nicht von einer anderen ab.
    ///
    /// **`bookmarks.toml` ist die eine mit [`Leerbefund::Beschaedigt`]**, und
    /// die Antwort ist gemessen und nicht geraten: eine leere
    /// [`Lesezeichenliste`](super::Lesezeichenliste) serialisiert zu
    /// `eintraege = []` und damit zu einem obersten Schluessel. Eine
    /// `bookmarks.toml` ohne einen einzigen hat KRK nie geschrieben.
    ///
    /// **Die drei uebrigen TOML-Dateien und die zwei Zettel tragen
    /// [`Leerbefund::Vorgabe`]**, und zwar vorlaeufig: `settings.toml` und
    /// `keymap.toml` aendert der Nutzer von Hand und darf sie leerraeumen,
    /// `session.toml` traegt an jeder Struktur `#[serde(default)]` und ist auf
    /// Nachsicht gegenueber einer aelteren Fassung gebaut. Ob die strenge
    /// Lesart auf `session.toml` und `keymap.toml` gehoert, ist die offene
    /// Frage
    /// `shared/decisions/260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`;
    /// bis zu ihrer Antwort steht hier die Fassung, die nichts am Verhalten
    /// aendert.
    pub const fn leerbefund(self) -> Leerbefund {
        match self {
            Datei::Lesezeichen => Leerbefund::Beschaedigt,
            Datei::Belegung | Datei::Sitzung | Datei::Einstellungen | Datei::Zettel(_) => {
                Leerbefund::Vorgabe
            }
        }
    }
}

/// Die drei Namensteile zwischen Benutzerverzeichnis und Ablageordner.
const UNTERPFAD: [&str; 3] = ["Library", "Application Support", "KRK"];

/// Das Benutzerverzeichnis, falls das System eines nennt.
///
/// Die eine Stelle im Kern, die danach fragt. **Ein `None` ist keine Aussage
/// ueber den Ordner, sondern eine ueber KRKs Kenntnis von ihm.** Der Baum geht
/// damit auf drei Weisen um, und welche gilt, entscheidet der Aufrufer an
/// seiner Aufgabe:
///
/// - **Scheitern**, wo es ohne Wurzel nichts zu tun gibt:
///   [`Ablageort::im_benutzerverzeichnis`] gibt einen Fehler zurueck.
/// - **Auf `/` ausweichen**, wo eine Flaeche einen Ordner zeigen muss:
///   `super::sitzung::standardordner` tut das, weil ein Dateifenster
///   trotzdem etwas anzeigen muss.
/// - **Unentschieden bleiben**, wo aus dem Wert eine Aussage ueber ein Ziel
///   wuerde: `Anwendungsdelegierter::loeschtexte` in `krk-ui` reicht das `None`
///   als offene Frage weiter, und die Rueckfrage vor dem Raeumen in den
///   Papierkorb wird darauf laut.
///
/// **Die dritte Weise ist die, die ein neuer Aufrufer am ehesten verfehlt.** Ein
/// `/` als Ausweichwert erfindet einen Pfad: aus „KRK kennt den Benutzerordner
/// nicht" wird die Aussage „der Ordner liegt darin". Wo an dem Wert eine
/// Warnung, eine Erlaubnis oder ein Loeschbefehl haengt, ist das falsch; nur wo
/// er eine Anzeige fuellt, ist es zulaessig. Eine Zahl der Aufrufer steht hier
/// bewusst nicht — sie altert mit jedem neuen, die Regel darueber nicht.
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

/// Der Ordner, in dem die sechs Dateien liegen.
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

    /// Der Pfad einer der sechs Dateien.
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
