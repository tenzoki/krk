//! Wo die sechs Ablagedateien liegen und wie der Ordner beim ersten Start
//! entsteht.
//!
//! Zwei abgeleitete Fragen stehen daneben, je eine vollstaendige
//! Fallunterscheidung ohne Auffangzweig: [`Datei::leerbefund`] sagt, was eine
//! Datei ohne einen einzigen obersten Schluessel bedeutet, und
//! [`Datei::ersatz`], was an die Stelle einer beschaedigten tritt. Wer eine
//! siebte Ablagedatei aufnimmt, beantwortet beide, sonst haelt der Uebersetzer
//! ihn an.
//!
//! **Eine weitere je Datei beantwortete Frage steht nicht hier**, und wer nur
//! diesen Kopf liest, zaehlt sie nicht mit:
//! [`super::neuerungen::Vergleichsform`] sagt, was bei
//! einer Ablagedatei ein Eintrag ist, den die Auslieferungsfassung fuehren kann
//! und die Nutzerdatei nicht. Sie ist von derselben Bauart und wohnt trotzdem
//! woanders, weil sie eine Aussage ueber den **Inhalt** einer Datei ist und
//! diese Datei den Inhalt nicht kennt.
//!
//! # Ein Format
//!
//! Alle sechs tragen TOML und gehen ueber [`super::Zugang::laden`] und
//! [`super::Zugang::sichern`]. Bis zur krkhome-Arbeit standen daneben die zwei
//! Notizzettel der Runde 9 als nackter Text, und mit ihnen eine Aufzaehlung
//! `Format` samt der Frage `Datei::format`, ueber die sich beide Sorten
//! trennten. Seit die Notizen in `~/krkhome/notes.txt` liegen, liest und
//! schreibt KRK die alten Zetteldateien nicht mehr; ihre Namen stehen allein
//! noch in `heimordner::ALTE_ZETTEL`, fuer die einmalige Uebernahme. Eine
//! Aufzaehlung mit einem Wert entschiede nichts und ist mit ihnen gefallen.
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

/// Was eine dastehende Ablagedatei bedeutet, aus der kein einziger oberster
/// Schluessel kommt.
///
/// **Zwei Werte, vollstaendig und ohne Auffangzweig**, wie [`Ersatz`] daneben.
/// Die Frage ist je Datei zu beantworten und nicht ueber alle zu
/// verallgemeinern: sie haengt daran, ob KRK diese Datei je ohne obersten
/// Schluessel schreibt, und das ist eine Aussage ueber ihren Schreiber und
/// nicht ueber TOML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Leerbefund {
    /// Ein gueltiger Bestand: jedes Feld steht auf seinem Auslieferungswert.
    ///
    /// Der Wert der von Hand gepflegten Dateien. Wer `keymap.toml` bis auf die
    /// Kommentare leerraeumt, meint die Vorgabebelegung und keinen Schaden.
    Vorgabe,
    /// Kein Bestand: die Datei hat nicht hergegeben, was sie traegt.
    ///
    /// Der Wert der Dateien, die KRK selbst schreibt und dabei **immer** mit
    /// mindestens einem obersten Schluessel: eine solche Datei ohne einen
    /// einzigen kann nicht aus KRKs Feder stammen, und sie als ersten Start zu
    /// lesen hiesse, den Bestand beim naechsten Schreibvorgang zu verlieren.
    Beschaedigt,
}

/// Was an die Stelle einer ersetzten Ablagedatei tritt.
///
/// **Zwei Werte, vollstaendig und ohne Auffangzweig**, wie [`Leerbefund`]
/// daneben. Die Frage ist je Datei zu beantworten: sie haengt
/// daran, ob es fuer diese Datei ueberhaupt etwas gibt, das einspringen
/// koennte, und das ist eine Aussage ueber ihren Zweck und nicht ueber ihre
/// Gestalt.
///
/// **Der Wert traegt die Auskunft, weil sie sonst niemand traegt.** Bis zum
/// 260824 stand sie als feststehende Prosa im Formatierer von
/// [`Ersetzung`](super::Ersetzung) und sagte in jedem Zweig „und wird durch den
/// Auslieferungszustand ersetzt". Fuer jede damalige Ablagedatei bis auf eine
/// stimmte das; mit `readers.toml` ist die erste dazugekommen, fuer die es
/// nicht stimmt.
/// [`Grund`](super::Grund) kann sie nicht tragen — derselbe Grund trifft jede
/// Datei, und beschaedigt ist beschaedigt, gleich was danach einspringt.
/// Getragen wird sie deshalb hier, neben den zwei anderen je Datei
/// beantworteten Fragen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ersatz {
    /// Die eingebettete Auslieferungsfassung oder der Vorgabewert tritt an die
    /// Stelle der Datei.
    ///
    /// Der Wert jeder Ablagedatei bis auf [`Datei::Leser`]. Bei den von KRK
    /// geschriebenen Dateien ist es der Vorgabewert der Struktur, bei
    /// `settings.toml` die eingebettete Auslieferungsfassung.
    Auslieferungszustand,
    /// Es tritt nichts an ihre Stelle.
    ///
    /// Der Wert allein von [`Datei::Leser`], und die Begruendung steht unter
    /// „Zweite Abweichung" im Kopf von [`super::leseprofile`]: die
    /// Auslieferungsfassung einzusetzen hiesse, dem Nutzer Profile
    /// unterzuschieben, die er vielleicht gerade herausgenommen hat.
    Nichts,
}

impl Ersatz {
    /// Der Satzteil, der die Auskunft in die Meldung traegt.
    ///
    /// Steht hier und nicht beim Formatierer, damit die Antwort und ihr
    /// Wortlaut an einer Stelle bleiben; [`Grund::beschreibung`](super::Grund)
    /// ist dieselbe Bauform.
    pub(super) const fn satzteil(self) -> &'static str {
        match self {
            Ersatz::Auslieferungszustand => "und wird durch den Auslieferungszustand ersetzt",
            Ersatz::Nichts => "und nichts tritt an ihre Stelle",
        }
    }
}

/// Die sechs Ablagedateien, die KRK unter `Application Support` ablegt.
///
/// Eine Aufzaehlung statt loser Namen: wer alle anfassen muss, laeuft
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
    /// Eine der beiden, die KRK im Betrieb nicht schreibt; die andere ist
    /// [`Datei::Leser`]. Sie entsteht beim ersten Start aus der eingebetteten
    /// Auslieferungsfassung und bleibt danach dem Nutzer ueberlassen.
    Einstellungen,
    /// `readers.toml`: die von Hand gepflegten Leseprofile, aus denen die
    /// Vorschau ihre Zusammenfassung baut (C1 der Runde 16).
    ///
    /// Sie geht denselben Weg wie [`Datei::Einstellungen`]: beim ersten Start
    /// entsteht sie aus einer eingebetteten Auslieferungsfassung, danach
    /// gehoert sie dem Nutzer. Wer sie anlegt und wer ihren Inhalt auswertet,
    /// beschreiben spaetere Schritte; die Ablage kennt von dieser Datei nur den
    /// Namen und den Weg dorthin.
    Leser,
    /// `reported.toml`: fuer welche Fassung von KRK die Neuerungen an den drei
    /// von Hand gepflegten Dateien schon gemeldet sind (Runde 24), siehe
    /// [`super::merker`].
    ///
    /// Die einzige, die keinen Bestand des Nutzers traegt, sondern eine
    /// Auskunft ueber KRKs eigenes Verhalten. Sie steht trotzdem hier und nicht
    /// als Feld einer anderen Datei; die Begruendung steht im Kopf von
    /// [`super::merker`] und der Ausschlag war die zweite Instanz.
    Merker,
}

impl Datei {
    /// Alle sechs, in fester Reihenfolge.
    pub const ALLE: [Datei; 6] = [
        Datei::Belegung,
        Datei::Lesezeichen,
        Datei::Sitzung,
        Datei::Einstellungen,
        Datei::Leser,
        Datei::Merker,
    ];

    /// Der Dateiname unterhalb des Ablageordners.
    ///
    /// `reported.toml` folgt der englischsprachigen Kleinschreibung der
    /// uebrigen; der Name ist der aus der Empfehlung des Datensatzes, den der
    /// Kopf von [`super::merker`] nennt.
    pub const fn dateiname(self) -> &'static str {
        match self {
            Datei::Belegung => "keymap.toml",
            Datei::Lesezeichen => "bookmarks.toml",
            Datei::Sitzung => "session.toml",
            Datei::Einstellungen => "settings.toml",
            Datei::Leser => "readers.toml",
            Datei::Merker => "reported.toml",
        }
    }

    /// Was eine dastehende Datei bedeutet, aus der kein einziger oberster
    /// Schluessel kommt.
    ///
    /// Eine abgeleitete Frage neben [`Datei::ersatz`]: vollstaendig, ohne
    /// Auffangzweig, damit eine siebte Ablagedatei den Bau anhaelt und eine
    /// bewusste Einordnung erzwingt. Wer sie beantwortet, beantwortet sie **je Datei** und leitet
    /// sie nicht von einer anderen ab.
    ///
    /// **`bookmarks.toml` und `session.toml` tragen
    /// [`Leerbefund::Beschaedigt`]**, und beide Antworten sind gemessen und
    /// nicht geraten. Eine leere
    /// [`Lesezeichenliste`](super::Lesezeichenliste) serialisiert zu
    /// `eintraege = []` und damit zu einem obersten Schluessel; eine
    /// `bookmarks.toml` ohne einen einzigen hat KRK nie geschrieben. Fuer
    /// [`Sitzung`](super::Sitzung) traegt die Messung weiter: die aermste
    /// ueberhaupt konstruierbare Sitzung serialisiert zu fuenf obersten
    /// Schluesseln, denn `aktiv` traegt kein `skip_serializing_if`, und die
    /// drei Tische und die Tischfolge
    /// `[[fenster]]` stehen unbedingt daneben. Die Probe dazu ist
    /// `jede_geschriebene_session_toml_traegt_einen_obersten_schluessel` in
    /// `krk-core/tests/ablage.rs`.
    ///
    /// **Die Strenge greift allein diese eine Haelfte, und der Sitzung fehlt
    /// die zweite mit Absicht.** `Sitzung` traegt kein
    /// `#[serde(deny_unknown_fields)]`, und das ist keine Auslassung, sondern
    /// die Bedingung des Nutzerentscheids vom 260907: eine `session.toml`, die
    /// eine **spaetere** Fassung von KRK mit einem neuen obersten Feld
    /// geschrieben hat, darf in einer frueheren die Sitzung nicht kosten. Ein
    /// unbekannter oberster Schluessel und ein fehlender sind hier verschiedene
    /// Befunde: den fehlenden schreibt KRK nie, den unbekannten schreibt es
    /// vielleicht morgen. Gehalten wird die Zusage von der Probe
    /// `eine_session_toml_aus_einer_spaeteren_fassung_behaelt_ihre_sitzung`.
    ///
    /// **`readers.toml` steht mit [`Leerbefund::Vorgabe`] neben
    /// `settings.toml` und `keymap.toml` und nicht neben `bookmarks.toml`**
    /// (C1.4 der Runde 16): sie wird von Hand gepflegt, und wer sie bis auf
    /// ihre Kommentare leerraeumt, meint „keine Profile" und keinen Schaden.
    /// KRK schreibt sie im Betrieb nie, also kann eine Datei ohne obersten
    /// Schluessel hier kein Zeichen fuer einen Schaden sein.
    ///
    /// **Die drei uebrigen tragen [`Leerbefund::Vorgabe`]**, und die Trennung
    /// folgt einem Kriterium: schreibt KRK die Datei selbst, oder pflegt der Nutzer sie von Hand?
    /// `keymap.toml`, `settings.toml` und `readers.toml` aendert der Nutzer von
    /// Hand und darf sie bis auf ihre Kommentare leerraeumen — das heisst dort
    /// „nimm die Vorgabe" und ist kein Schaden. So entschieden am 260907
    /// (`shared/decisions/260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`,
    /// Moeglichkeit 2).
    pub const fn leerbefund(self) -> Leerbefund {
        match self {
            Datei::Lesezeichen | Datei::Sitzung | Datei::Merker => Leerbefund::Beschaedigt,
            Datei::Belegung | Datei::Einstellungen | Datei::Leser => Leerbefund::Vorgabe,
        }
    }

    /// Was an die Stelle dieser Datei tritt, wenn sie ersetzt wird.
    ///
    /// Die abgeleitete Frage neben [`Datei::leerbefund`], und sie steht aus
    /// demselben Grund hier: vollstaendig, ohne Auffangzweig, damit eine
    /// siebte Ablagedatei den Bau anhaelt und eine bewusste Einordnung
    /// erzwingt.
    ///
    /// **`readers.toml` ist die eine mit [`Ersatz::Nichts`]** (C1.6 der
    /// Runde 16). Fuer die fuenf uebrigen gibt es einen Zustand, der einspringt
    /// — ein Vorgabewert oder die eingebettete Auslieferungsfassung —, und die
    /// Meldung darf ihn versprechen. Fuer `readers.toml`
    /// gibt es ihn nicht: eine beschaedigte Datei fuehrt dort zu gar keinem
    /// Profil, und die Begruendung steht unter „Zweite Abweichung" im Kopf von
    /// [`super::leseprofile`].
    pub const fn ersatz(self) -> Ersatz {
        match self {
            Datei::Leser => Ersatz::Nichts,
            Datei::Belegung
            | Datei::Lesezeichen
            | Datei::Sitzung
            | Datei::Einstellungen
            | Datei::Merker => Ersatz::Auslieferungszustand,
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

/// Der Ordner, in dem die sechs Ablagedateien liegen.
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

    /// Der Pfad einer der sechs Ablagedateien.
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
