//! Der Ort des Notizordners als Text: lesen, pruefen und zurueckschreiben.
//!
//! Der Ort kommt aus `settings.toml` (Schluessel `notizordner`, H2 des Spec
//! `260926-1451_*_spec-home-menue-und-einstellbarer-ort.md`), ab Werk
//! `~/` und [`ORDNERNAME`]. **[`ort_lesen`] ist die eine Pruefung**, und sie
//! hat zwei Frager: den Start, der den Wert von Hand prueft, und ab Stufe 3
//! „Ort waehlen…“, das genau den Text prueft, den es danach in die Datei
//! schreibt. [`schreibform`] ist ihr Gegenstueck und macht aus einem Ort den
//! Text, den [`ort_lesen`] wieder zu demselben Ort liest.
//!
//! # Ohne Systemaufruf
//!
//! **Keine Funktion dieses Moduls fragt das Dateisystem.** Geprueft wird der
//! Text: `~/` wird gegen das hereingereichte Benutzerverzeichnis gesetzt, und
//! danach wird der Pfad **lexikalisch** bereinigt, also ohne nachzusehen, ob
//! unterwegs ein Verweis steht. Das ist der Preis dafuer, dass der Start einen
//! eingestellten Ort nie beruehrt: ein nicht eingehaengtes oder haengendes
//! Laufwerk haelt ihn nicht an.
//!
//! # Der Ablageordner von KRK ist ausgeschlossen
//!
//! Ein Ort im Ablageordner (`~/Library/Application Support/KRK/`) oder darunter
//! wird abgewiesen, weil Werkzeuge, die eine Anwendung samt ihren Stuetzdateien
//! entfernen, diesen Ordner mitnehmen. **Verglichen wird Bestandteil fuer
//! Bestandteil und ohne Ruecksicht auf Gross- und Kleinschreibung**: das Volume
//! des Benutzerverzeichnisses unterscheidet sie gewoehnlich nicht, und
//! `~/library/application support/krk` ist dort derselbe Ordner. Ein Nachbar
//! wie `KRK-alt` faellt nicht darunter, weil ganze Bestandteile verglichen
//! werden und keine Zeichenfolgen. **Hingenommen ist die Gegenseite**: auf einem
//! Volume, das Gross und Klein unterscheidet, weist die Regel einen Ordner ab,
//! der nicht der Ablageordner ist.
//!
//! **Was die Pruefung nicht sieht**: einen Ort, der erst ueber einen Verweis
//! im Ablageordner landet. Das zu erkennen braeuchte `canonicalize`, also einen
//! Systemaufruf am Ort; beim Start gibt es ihn nicht. „Ort waehlen…“ prueft
//! ab Stufe 3 zusaetzlich die kanonische Form.

use std::path::{Component, Path, PathBuf};

use super::{Heimordner, ORDNERNAME, lexikalisch_bereinigt};

/// Der Notizordner, der gilt, oder der Grund, warum keiner gilt.
///
/// **Kein Ersatzort**: ein `Err` heisst, F2 nennt den Grund, legt nichts an
/// und oeffnet keinen Tab. Ein stiller Rueckfall auf den Vorgabeort legte
/// Dateien an einem Ort an, den der Nutzer gerade verlassen wollte (H2 des
/// Spec, „Decisions made“).
pub type Notizort = Result<Heimordner, Ortsfehler>;

/// Warum ein Text keinen Notizordner ergibt.
///
/// **Vollstaendig und ohne Auffangzweig**, damit ein weiterer Fall den Bau an
/// [`Ortsfehler::meldung`] anhaelt, bis er einen Satz hat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ortsfehler {
    /// Das System nennt kein Benutzerverzeichnis; `~/` laesst sich nicht
    /// aufloesen, und der Vorgabeort ist nicht zu bauen.
    KeinBenutzerverzeichnis,
    /// Der Wert ist leer oder traegt nichts als Leerraum.
    Leer,
    /// Der Wert beginnt weder mit `~/` noch mit `/`; `~` allein gehoert dazu.
    /// Der Wert, wie er dasteht.
    NichtAbsolut(String),
    /// Der Wert beginnt mit `~name`, nennt also das Benutzerverzeichnis eines
    /// anderen Benutzers. Der Wert, wie er dasteht.
    FremdesBenutzerverzeichnis(String),
    /// Der Wert ist kein Text, etwa eine Zahl. Seine TOML-Schreibweise.
    ///
    /// Erzeugt erst dort, wo `settings.toml` gedeutet wird; [`ort_lesen`]
    /// bekommt schon einen Text.
    KeinText(String),
    /// Der Ort liegt im Ablageordner von KRK oder darunter. Der Wert, wie er
    /// dasteht.
    ImAblageordner(String),
}

impl Ortsfehler {
    /// Der Satz fuer die Statuszeile. Nennt den Wert, wo es einen gibt.
    pub fn meldung(&self) -> String {
        match self {
            Ortsfehler::KeinBenutzerverzeichnis => {
                "Das System nennt kein Benutzerverzeichnis, also gibt es keinen Notizordner"
                    .to_owned()
            }
            Ortsfehler::Leer => format!(
                "Der Notizordner in settings.toml ist leer; gültig ist ein Ort, der mit „~/“ oder „/“ beginnt, ab Werk „~/{ORDNERNAME}“"
            ),
            Ortsfehler::NichtAbsolut(wert) => format!(
                "Der Notizordner „{wert}“ in settings.toml beginnt weder mit „~/“ noch mit „/“"
            ),
            Ortsfehler::FremdesBenutzerverzeichnis(wert) => format!(
                "Der Notizordner „{wert}“ in settings.toml nennt ein fremdes Benutzerverzeichnis; gültig ist „~/“ für das eigene oder ein Pfad ab „/“"
            ),
            Ortsfehler::KeinText(wert) => format!(
                "Der Notizordner in settings.toml ist kein Text, sondern {wert}; gültig ist ein Ort in Anführungszeichen, etwa \"~/{ORDNERNAME}\""
            ),
            Ortsfehler::ImAblageordner(wert) => format!(
                "Der Notizordner „{wert}“ liegt im Ablageordner von KRK; ein Werkzeug, das KRK entfernt, nähme ihn mit, also gilt er nicht"
            ),
        }
    }
}

/// Liest einen Ortstext zu einem Pfad, oder zum Grund, warum er keiner ist.
///
/// - `~/rest` wird gegen `benutzerverzeichnis` gesetzt; ohne eines ist das
///   [`Ortsfehler::KeinBenutzerverzeichnis`]. `~/` allein nennt das
///   Benutzerverzeichnis selbst.
/// - `/…` gilt, wie es dasteht.
/// - Beides wird danach lexikalisch bereinigt: `.` faellt, `..` hebt den
///   Bestandteil davor auf, doppelte und schliessende Trennstriche fallen.
///
/// Abgewiesen werden der leere Text, jeder relative Text, `~` allein, `~name`
/// und `~name/…` sowie ein Ort im Ablageordner oder darunter, sofern der Rufer
/// einen `ablageordner` nennt. **Kein Systemaufruf**; der Modulkopf sagt,
/// warum, und was die Pruefung deshalb nicht sieht.
#[must_use = "ein Ortsfehler heisst: es gilt kein Notizordner, und die Meldung gehoert in die Statuszeile"]
pub fn ort_lesen(
    text: &str,
    benutzerverzeichnis: Option<&Path>,
    ablageordner: Option<&Path>,
) -> Result<PathBuf, Ortsfehler> {
    if text.trim().is_empty() {
        return Err(Ortsfehler::Leer);
    }
    let ungeprueft = if let Some(rest) = text.strip_prefix("~/") {
        let zuhause = benutzerverzeichnis.ok_or(Ortsfehler::KeinBenutzerverzeichnis)?;
        zuhause.join(rest)
    } else if text == "~" {
        return Err(Ortsfehler::NichtAbsolut(text.to_owned()));
    } else if text.starts_with('~') {
        return Err(Ortsfehler::FremdesBenutzerverzeichnis(text.to_owned()));
    } else if text.starts_with('/') {
        PathBuf::from(text)
    } else {
        return Err(Ortsfehler::NichtAbsolut(text.to_owned()));
    };
    let ort = lexikalisch_bereinigt(&ungeprueft);
    if let Some(ablage) = ablageordner
        && liegt_darin_ohne_schreibung(&ort, &lexikalisch_bereinigt(ablage))
    {
        return Err(Ortsfehler::ImAblageordner(text.to_owned()));
    }
    Ok(ort)
}

/// Der Text, den [`ort_lesen`] wieder zu `ort` liest, oder `None`, wenn der
/// Pfad kein gueltiges UTF-8 ist.
///
/// Ein Ort **echt unterhalb** des Benutzerverzeichnisses steht als `~/rest`,
/// jeder andere absolut. **Das Benutzerverzeichnis selbst steht absolut**, weil
/// `~` allein keine zulaessige Leseform ist; `~/` waere eine, sieht aber aus
/// wie ein abgeschnittener Pfad.
#[must_use]
pub fn schreibform(ort: &Path, benutzerverzeichnis: Option<&Path>) -> Option<String> {
    if let Some(zuhause) = benutzerverzeichnis
        && let Ok(rest) = ort.strip_prefix(zuhause)
        && !rest.as_os_str().is_empty()
    {
        return rest.to_str().map(|rest| format!("~/{rest}"));
    }
    ort.to_str().map(str::to_owned)
}

/// Ob `ort` der Ordner `ordner` ist oder darunter liegt, Bestandteil fuer
/// Bestandteil und ohne Ruecksicht auf Gross- und Kleinschreibung.
fn liegt_darin_ohne_schreibung(ort: &Path, ordner: &Path) -> bool {
    let mut teile = ort.components();
    ordner.components().all(|oben| {
        teile
            .next()
            .is_some_and(|unten| gleich_ohne_schreibung(oben, unten))
    })
}

/// Zwei Pfadbestandteile ohne Ruecksicht auf Gross- und Kleinschreibung.
fn gleich_ohne_schreibung(links: Component<'_>, rechts: Component<'_>) -> bool {
    let klein = |teil: Component<'_>| teil.as_os_str().to_string_lossy().to_lowercase();
    klein(links) == klein(rechts)
}
