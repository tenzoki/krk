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
//! **[`ort_lesen`], [`schreibform`], [`ortswechsel`] und [`startzeile`] fragen
//! das Dateisystem nicht.** [`notizort`] und [`schutzort`] bauen am Ende einen
//! [`Heimordner`] ueber [`Heimordner::am_ort`], und der stellt fuer einen Ort
//! unmittelbar im Benutzerverzeichnis zwei Aufrufe an dessen Eintrag, nie am
//! Ziel eines Verweises; fuer jeden anderen Ort keinen. Geprueft wird der
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
use crate::ablage::einstellungen::Ortswert;
use crate::ablage::{Grund, pfade};

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
    /// `settings.toml` war da und liess sich nicht lesen oder nicht deuten
    /// ([`Grund::Beschaedigt`], [`Grund::NichtLesbar`]). Traegt den Satzteil
    /// des Grunds, etwa „ist beschädigt“.
    ///
    /// Die Einzelheit nennt die Startmeldung des Laders schon; dieser Fehler
    /// nennt den Weg hinaus.
    EinstellungenBeschaedigt(String),
    /// Der Start ist nicht bis zum Lesen von `settings.toml` gekommen, weil
    /// sich der Ablageordner nicht oeffnen oder seine Schreibsperre nicht
    /// nehmen liess. Traegt die Ursache.
    EinstellungenUngelesen(String),
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
            Ortsfehler::EinstellungenBeschaedigt(satzteil) => format!(
                "settings.toml {satzteil}, also gilt kein Notizordner, und F2 legt nichts an: settings.toml berichtigen und KRK neu starten"
            ),
            Ortsfehler::EinstellungenUngelesen(ursache) => format!(
                "KRK konnte settings.toml beim Start nicht lesen ({ursache}), also gilt kein Notizordner, und F2 legt nichts an: KRK neu starten"
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

/// Der Notizordner, der nach dem Lesen von `settings.toml` gilt, oder der
/// Grund, warum keiner gilt.
///
/// `wert` ist [`crate::ablage::Einstellungen::notizordner`], `schaden` der
/// Grund, aus dem der Lader die Datei ersetzt hat, falls er es getan hat.
/// **Vollstaendig ueber `schaden`**, ohne Auffangzweig:
///
/// - kein Schaden: der Wert der Datei, geprueft von [`ort_lesen`];
/// - [`Grund::NichtAnlegbar`]: die Datei fehlte, also hat niemand einen Ort
///   eingestellt, und `wert` ist der Auslieferungswert, der Vorgabeort;
/// - [`Grund::Beschaedigt`] und [`Grund::NichtLesbar`]: **kein Ort**, siehe
///   den Zweig.
///
/// Gebaut wird der Wert ueber [`Heimordner::am_ort`]; der Modulkopf sagt, was
/// das am Dateisystem kostet.
#[must_use = "der Notizort ist die ganze Wirkung; er gehoert in den Griff"]
pub fn notizort(
    wert: &Ortswert,
    schaden: Option<&Grund>,
    benutzerverzeichnis: Option<&Path>,
    ablageordner: Option<&Path>,
) -> Notizort {
    match schaden {
        None | Some(Grund::NichtAnlegbar(_)) => {}
        // OFFENE NUTZERFRAGE: dieser eine Zweig haengt an der Antwort auf
        // `260926-1506_*_welcher-notizordner-gilt-wenn-settings-toml-beim-start-beschaedigt-ist.md`.
        // Gebaut ist die empfohlene Moeglichkeit 1 in der Schaerfung der
        // Zweitlesung: bei einer beschaedigten oder unlesbaren Datei weiss KRK
        // nicht, ob der Nutzer einen anderen Ort eingestellt hat, und ein
        // Ersatzort legte bei F2 Dateien dort an, wo er nicht mehr notiert.
        // Eine andere Antwort aendert allein diesen Zweig, seine Probe in
        // `tests/heimordner.rs` und den Satz in `HowTo.md`.
        Some(grund @ (Grund::Beschaedigt(_) | Grund::NichtLesbar(_))) => {
            return Err(Ortsfehler::EinstellungenBeschaedigt(
                grund.beschreibung().to_owned(),
            ));
        }
    }
    let text = match wert {
        Ortswert::Text(text) => text,
        Ortswert::KeinText(schreibweise) => {
            return Err(Ortsfehler::KeinText(schreibweise.clone()));
        }
    };
    let ort = ort_lesen(text, benutzerverzeichnis, ablageordner)?;
    Ok(Heimordner::am_ort(ort, benutzerverzeichnis))
}

/// Der gemerkte Ort als bereinigter Pfad.
///
/// `session.toml` traegt den geschriebenen, absoluten Pfad; eine von Hand
/// geschriebene Form `~/…` wird wie in `settings.toml` gelesen, damit
/// `~/krkhome` und `/<zuhause>/krkhome` derselbe Ort sind. Was [`ort_lesen`]
/// abweist, gilt lexikalisch bereinigt, wie es dasteht: der gemerkte Ort ist
/// eine Erinnerung und wird nicht geprueft.
fn gemerkt_gelesen(gemerkt: &Path, benutzerverzeichnis: Option<&Path>) -> PathBuf {
    gemerkt
        .to_str()
        .and_then(|text| ort_lesen(text, benutzerverzeichnis, None).ok())
        .unwrap_or_else(|| lexikalisch_bereinigt(gemerkt))
}

/// Der Satz ueber einen Ortswechsel: welcher Ort jetzt gilt, dass am alten
/// alles liegen bleibt und F2 zum neuen fuehrt.
///
/// Beide Orte in der Anzeigeform. Stufe 3 („Ort waehlen…“) nimmt denselben
/// Satz.
#[must_use]
pub fn wechselsatz(neu: &str, alt: &str) -> String {
    format!(
        "Der Notizordner ist jetzt „{neu}“; am alten Ort „{alt}“ bleibt alles liegen, und F2 führt zum neuen"
    )
}

/// Der Wechselsatz, wenn der geltende Ort ein anderer ist als der gemerkte,
/// sonst `None`.
///
/// Kein gemerkter Ort, derselbe Ort (auch in anderer Schreibweise) und ein
/// ungueltiger geltender Ort ergeben keinen Satz: im letzten Fall gilt gar
/// kein Ort, und dessen Grund meldet [`startzeile`] oder der Lader.
/// Verglichen wird als Text, **ohne Systemaufruf**; am alten Ort wird nicht
/// nachgesehen, ob dort etwas liegt.
#[must_use]
pub fn ortswechsel(
    gemerkt: Option<&Path>,
    geltend: &Notizort,
    benutzerverzeichnis: Option<&Path>,
) -> Option<String> {
    let gemerkt = gemerkt?;
    let neu = geltend.as_ref().ok()?;
    let alt = gemerkt_gelesen(gemerkt, benutzerverzeichnis);
    if alt == neu.geschrieben() {
        return None;
    }
    let alt = pfade::gekuerzt_fuer_anzeige(&alt, benutzerverzeichnis);
    Some(wechselsatz(neu.anzeigename(), &alt))
}

/// Hoechstens eine Startmeldung zum Notizordner.
///
/// **Vollstaendig ueber den Fehler**, ohne Auffangzweig:
///
/// - ein gueltiger Ort: der Wechselsatz aus [`ortswechsel`], oder nichts;
/// - [`Ortsfehler::EinstellungenBeschaedigt`] und
///   [`Ortsfehler::EinstellungenUngelesen`]: nichts, weil der Lader
///   beziehungsweise der fruehe Ausgang des Starts die Ursache schon in die
///   Statuszeile stellt und F2 sie wiederholt;
/// - jeder andere Fehler: seine Meldung, die den Wert nennt.
#[must_use]
pub fn startzeile(
    geltend: &Notizort,
    gemerkt: Option<&Path>,
    benutzerverzeichnis: Option<&Path>,
) -> Option<String> {
    let fehler = match geltend {
        Ok(_) => return ortswechsel(gemerkt, geltend, benutzerverzeichnis),
        Err(fehler) => fehler,
    };
    match fehler {
        Ortsfehler::EinstellungenBeschaedigt(_) | Ortsfehler::EinstellungenUngelesen(_) => None,
        Ortsfehler::KeinBenutzerverzeichnis
        | Ortsfehler::Leer
        | Ortsfehler::NichtAbsolut(_)
        | Ortsfehler::FremdesBenutzerverzeichnis(_)
        | Ortsfehler::KeinText(_)
        | Ortsfehler::ImAblageordner(_) => Some(fehler.meldung()),
    }
}

/// Der Pfad, den die Sitzung als zuletzt geltenden Ort merkt.
///
/// Der geschriebene Pfad eines gueltigen Orts; gilt keiner, bleibt der bisher
/// gemerkte stehen, damit nach dem Berichtigen keine Wechselmeldung kommt, wenn
/// der Ort derselbe geblieben ist.
#[must_use]
pub fn zu_merken(geltend: &Notizort, gemerkt: Option<&Path>) -> Option<PathBuf> {
    match geltend {
        Ok(heim) => Some(heim.geschrieben().to_path_buf()),
        Err(_) => gemerkt.map(Path::to_path_buf),
    }
}

/// Der Ordner, dessen Schutzregeln weiter gelten, wenn **kein** Notizordner
/// gilt: der zuletzt geltende aus der Sitzung, ohne einen gemerkten der
/// Vorgabeort.
///
/// **Eine Entscheidung zur sicheren Seite**, getroffen beim Bau von Schritt
/// 2.4 des Plans `260926-1506_*_plan-home-menue-und-einstellbarer-ort.md`.
/// Ohne Ort fragten Vorschau, Editor, Inhaltsfilter, Sitzung und
/// Tastenprotokoll keinen Heimordner mehr, und eine `secrets.txt` am Ort, der
/// bis gestern galt, verloere still jede Regel: eine leere ginge im Editor
/// ueber den Klartextweg, und der Inhaltsfilter laese sie. **F2 und „Ort
/// waehlen…“ sehen diesen Ordner nicht**; sie lesen den Fehler und legen
/// nichts an. Der Preis: die Regeln fuer `notes.txt` und `tasks.txt` gelten
/// an diesem Ordner ebenfalls weiter, denn die Erkennung trennt sie nicht von
/// denen fuer `secrets.txt`.
#[must_use]
pub fn schutzort(gemerkt: Option<&Path>, benutzerverzeichnis: Option<&Path>) -> Option<Heimordner> {
    match gemerkt {
        Some(gemerkt) => Some(Heimordner::am_ort(
            gemerkt_gelesen(gemerkt, benutzerverzeichnis),
            benutzerverzeichnis,
        )),
        None => benutzerverzeichnis.map(Heimordner::im_benutzerverzeichnis),
    }
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
