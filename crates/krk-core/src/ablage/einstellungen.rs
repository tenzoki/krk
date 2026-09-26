//! `settings.toml`: die erste Ablagedatei, die der Nutzer von Hand pflegt
//! (C11). Die zweite ist `readers.toml` seit der Runde 16, siehe
//! [`super::leseprofile`]; welche Datei welchen Weg geht, sagt
//! [`super::pfade::Datei`].
//!
//! ```text
//! resources/default-settings.toml ──include_str!──> AUSLIEFERUNGSTEXT
//!                                                     │        │
//!                            erster Start ──atomar────┘        │
//!                                                              v
//!      ~/Library/.../KRK/settings.toml ──Ablage──> Einstellungen
//! ```
//!
//! # Warum eine vierte Datei
//!
//! Die drei vorhandenen scheiden aus, jede aus einem eigenen Grund, und die
//! Herleitung steht in `### Frage 4` des Plans. Der kurze Stand: `keymap.toml`
//! setzt der Befehl aus C3 vollstaendig zurueck und naehme die Terminal-Wahl
//! mit; `session.toml` ueberschreibt KRK alle zwei Sekunden und loeschte dabei
//! jeden Kommentar; `bookmarks.toml` haelt Ordnerverweise und wird bei jeder
//! Aenderung geschrieben. Aufgenommen wurde hier ein Wert, der keine
//! Tastenbelegung ist, den KRK im Betrieb nicht selbst schreibt und der in
//! jener Runde keine Oberflaeche hatte.
//!
//! **Die Aufnahmeregel lautet seit Schritt 3.1 des Plans
//! `260926-1506_*_plan-home-menue-und-einstellbarer-ort.md` weiter:** ein Wert
//! darf eine Ansicht haben, die ihn schreibt, wenn ihr Schreibweg allein den
//! Byte-Bereich dieses Werts beruehrt und jedes andere Byte der Datei stehen
//! laesst, Kommentare eingeschlossen. `notizordner` ist der erste solche Wert;
//! sein Schreibweg ist [`notizordner_schreiben`].
//!
//! # Die Datei entsteht einmal und hat danach genau einen Schreibweg
//!
//! [`laden`] legt sie beim ersten Start an, und zwar **woertlich aus
//! [`AUSLIEFERUNGSTEXT`]** und nicht ueber [`Zugang::sichern`]. Der Unterschied
//! ist der ganze Zweck der Datei: `serde` kennt keine Kommentare, und eine
//! Serialisierung von [`Einstellungen`] hinterliesse eine Datei mit einer
//! einzigen Zeile. Die fuenfzig Kommentarzeilen der Auslieferungsfassung sind
//! die Antwort auf den Einwand gegen die Buendelkennung — sie nennen das
//! `mdls`-Kommando, mit dem der Nutzer die Kennung seiner eigenen Anwendung
//! findet. Ohne sie stuende dort ein Wert, den niemand aendern kann.
//!
//! Der Schreibweg selbst ist der aus Schritt 10 und kein zweiter:
//! [`atomar::schreiben`], derselbe Ablageort, dieselbe Behandlung einer
//! beschaedigten Datei. Allein die Nutzlast ist eine andere.
//!
//! **Danach schreibt allein „Ort waehlen…“ die Datei, und dort allein den Wert
//! von `notizordner`** ([`notizordner_schreiben`]). Es liest die Datei unter
//! der Schreibsperre, laesst den Leser den Byte-Bereich des Werts melden
//! (`toml::Spanned`) und ersetzt genau diesen Bereich; fehlt der Schluessel,
//! haengt es ihn samt einer Kommentarzeile ans Ende, mit dem Zeilenende der
//! Datei. Vor dem Schreiben liest es das Ergebnis ein zweites Mal, und es muss
//! allein im Wert von `notizordner` abweichen. Eine beschaedigte Datei
//! schreibt es nicht: der Nutzer berichtigt sie zuerst.
//!
//! **Das Anhaengen am Ende gilt nur, solange die Datei keine Tabelle kennt.**
//! `deny_unknown_fields` laesst heute allein oberste Skalare zu. Kaeme ein
//! Schluessel mit eigener Tabelle hinzu, landete ein angehaengter
//! `notizordner` darin; die zweite Lesung faengt das als
//! [`Schreibhindernis::Intern`] ab, und der Schreibweg muss dann vor der
//! ersten Tabellenueberschrift einfuegen.
//!
//! **Eine verknuepfte `settings.toml` schreibt KRK nicht.**
//! [`atomar::schreiben`] ersetzt das Ziel ueber `rename`, und ein symbolischer
//! Verweis an der Stelle waere danach eine gewoehnliche Datei; KRK verwandelte
//! eine vom Nutzer gepflegte Verknuepfung still in eine Kopie, und die Datei,
//! auf die sie zeigte, bliebe beim alten Wert. [`notizordner_schreiben`] fragt
//! deshalb unter der Sperre `symlink_metadata` und antwortet bei einem Verweis,
//! auch einem verwaisten, mit [`Schreibhindernis::Verweis`], das die Zeile zum
//! Eintragen von Hand mitgibt. `keymap.toml` ist davon unberuehrt; ihr
//! Schreibweg ist [`Zugang::sichern`]. **Die verbleibende Luecke:** zwischen
//! `symlink_metadata` und `rename` kann ein anderes Programm, das die
//! Schreibsperre nicht kennt, einen Verweis an die Stelle legen; die Sperre
//! haelt allein eine zweite Instanz von KRK ab.
//!
//! # Ein fehlendes Feld kommt aus der Auslieferungsfassung
//!
//! Das ist die eine Abweichung von `keymap.toml`, wo die Nutzerdatei die
//! Auslieferungsbelegung **ersetzt**. Dort braucht es das, weil der Nutzer eine
//! Belegung sonst nicht loswerden koennte; eine Terminal-Anwendung laesst sich
//! nicht abwaehlen, ohne die Funktion abzuschalten. Die Abweichung kostet keine
//! Verzweigung: [`Einstellungsdatei`] haelt jedes Feld als `Option`, und
//! [`Einstellungen::aus_datei`] fuellt das leere aus der Auslieferungsfassung.

use std::fs;
use std::io;
use std::sync::LazyLock;

use serde::Deserialize;

use super::{Beiseite, Datei, Ersetzung, Geladen, Grund, Zugang, atomar, einzeilig};

/// Die Auslieferungsfassung der Einstellungen, in das Programm einkompiliert.
///
/// Damit gibt es keinen Start ohne Einstellungen, und die Anlage beim ersten
/// Start braucht keinen Zugriff auf das Buendel.
pub const AUSLIEFERUNGSTEXT: &str = include_str!("../../../../resources/default-settings.toml");

/// Die gelesene Auslieferungsfassung.
///
/// Sie fuellt jedes Feld, das die Nutzerdatei nicht nennt. Gebaut wird sie
/// allein aus dem eingebetteten Text und nie aus sich selbst; eine
/// Ruecksprungmarke auf diesen Wert waehrend seiner eigenen Entstehung gibt es
/// deshalb nicht.
static AUSLIEFERUNG: LazyLock<Einstellungen> = LazyLock::new(|| {
    let datei: Einstellungsdatei = toml::from_str(AUSLIEFERUNGSTEXT)
        .expect("die eingebettete Auslieferungsfassung ist kein gueltiges TOML");
    Einstellungen {
        terminal: datei
            .terminal
            .expect("die eingebettete Auslieferungsfassung nennt keinen Eintrag terminal"),
        notizordner: ortswert(
            datei
                .notizordner
                .as_ref()
                .expect("die eingebettete Auslieferungsfassung nennt keinen Eintrag notizordner"),
        ),
    }
});

/// Die von Hand gepflegten Einstellungen, wie KRK sie im Betrieb liest.
///
/// **Bewusst ohne `Serialize`.** Ein Serialisierungsweg waere der zweite Weg zu
/// dieser Datei, und er schriebe sie ohne ihre Kommentare; siehe den Modulkopf.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Einstellungen {
    /// Die Buendelkennung der Anwendung, die "Ordner im Terminal oeffnen"
    /// ruft (C11), etwa `com.apple.Terminal`.
    ///
    /// Eine Kennung und kein Pfad: `NSWorkspace` kennt genau einen nicht
    /// abgekuendigten Weg von einem Namen zu einer installierten Anwendung, und
    /// der geht ueber die Kennung. Sie ueberlebt zudem das Verschieben und
    /// Umbenennen der Anwendung. Die Herleitung steht in `### Frage 4` des
    /// Plans, die Erklaerung fuer den Nutzer in `resources/default-settings.toml`.
    pub terminal: String,
    /// Der Ort des Notizordners, wie er in der Datei steht, noch ungeprueft
    /// (H2 des Spec `260926-1451_*_spec-home-menue-und-einstellbarer-ort.md`).
    ///
    /// **Ein Wert, und noch kein Ort**: ob der Text einen Notizordner ergibt,
    /// entscheidet `heimordner::ort::ort_lesen`, und das kennt dieses Modul
    /// nicht. Ein Wert, der kein Text ist, macht die Datei **nicht**
    /// beschaedigt, sondern wird [`Ortswert::KeinText`]; siehe dort.
    ///
    /// Nennt die Nutzerdatei ihn nicht, fuellt ihn die Auslieferungsfassung,
    /// wie bei `terminal`; ab Werk ist das der Vorgabeort `~/krkhome`.
    pub notizordner: Ortswert,
}

/// Der Wert von `notizordner`, wie `settings.toml` ihn traegt.
///
/// **Vollstaendig und ohne Auffangzweig.** Die zweite Variante gibt es, weil
/// der Spec „kein Text“ zu den unzulaessigen **Werten** zaehlt und nicht zu den
/// Schaeden der Datei (H2, drittes Kriterium): ein `notizordner = 5` ergibt
/// eine Meldung, die den Wert nennt, und keinen Notizordner, laesst aber
/// `terminal` und die Datei, wie sie sind. Bei `terminal` bleibt ein Wert
/// falschen Typs ein Dateischaden.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ortswert {
    /// Ein Text, noch ungeprueft.
    Text(String),
    /// Ein Wert anderen Typs, in seiner TOML-Schreibweise.
    KeinText(String),
}

/// Der Ortswert zu einem gelesenen TOML-Wert.
fn ortswert(wert: &toml::Spanned<toml::Value>) -> Ortswert {
    match wert.get_ref() {
        toml::Value::String(text) => Ortswert::Text(text.clone()),
        anderer => Ortswert::KeinText(anderer.to_string()),
    }
}

impl Einstellungen {
    /// Die eingebettete Auslieferungsfassung.
    pub fn auslieferung() -> Self {
        AUSLIEFERUNG.clone()
    }

    /// Die gelesene Datei, ergaenzt um jedes Feld, das sie nicht nennt.
    fn aus_datei(datei: &Einstellungsdatei) -> Self {
        Self {
            terminal: datei
                .terminal
                .clone()
                .unwrap_or_else(|| AUSLIEFERUNG.terminal.clone()),
            notizordner: datei
                .notizordner
                .as_ref()
                .map_or_else(|| AUSLIEFERUNG.notizordner.clone(), ortswert),
        }
    }
}

impl Default for Einstellungen {
    fn default() -> Self {
        Self::auslieferung()
    }
}

/// Die Gestalt von `default-settings.toml` und `settings.toml`, unveraendert.
///
/// Der Zwischenschritt zwischen TOML und [`Einstellungen`]: hier fehlt ein
/// Feld noch, statt schon aus der Auslieferungsfassung zu kommen. Denselben
/// Zuschnitt zieht `Belegungsdatei` neben `Belegung`.
///
/// `deny_unknown_fields` wie dort: ein Feld, das KRK nicht kennt, ist in einer
/// von Hand gepflegten Datei fast immer ein Tippfehler, und der Nutzer soll ihn
/// als Meldung sehen statt seine Einstellung stillschweigend zu verlieren.
///
/// **`notizordner` wird als beliebiger TOML-Wert gelesen**, samt seinem
/// Byte-Bereich in der Datei, und erst danach gedeutet ([`Ortswert`]). Ein
/// `Option<String>` wie bei `terminal` machte einen Wert falschen Typs zum
/// Schaden der ganzen Datei. Den Bereich braucht der Schreibweg von „Ort
/// waehlen…“, der allein ihn ersetzt. Ohne `Eq`, weil `toml::Value`
/// Gleitkommazahlen tragen kann.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Einstellungsdatei {
    #[serde(default)]
    terminal: Option<String>,
    #[serde(default)]
    notizordner: Option<toml::Spanned<toml::Value>>,
}

/// Laedt `settings.toml` und legt sie beim ersten Start an.
///
/// Scheitert nie. Die vier Faelle:
///
/// | Auf der Platte | Ergebnis |
/// |---|---|
/// | keine Datei | Vorbelegung, **keine** Meldung, die Datei entsteht |
/// | gueltige Datei | ihr Wert, keine Meldung, nichts wird geschrieben |
/// | Datei ohne `terminal` | Vorbelegung, keine Meldung, nichts wird geschrieben |
/// | kaputte Datei | Vorbelegung, Meldung, die Datei bleibt unveraendert liegen |
///
/// Die kaputte Datei bleibt aus demselben Grund liegen wie eine kaputte
/// `keymap.toml`: sie ist von Hand geschrieben, und ein Tippfehler darf die
/// Arbeit des Nutzers nicht loeschen. Ueberschrieben wird sie auch spaeter
/// nicht: der eine Schreibweg, [`notizordner_schreiben`], weist eine
/// beschaedigte Datei ab.
///
/// Hoechstens eine Meldung kann anfallen: angelegt wird nur, was fehlt, und
/// eine fehlende Datei traegt keine Ersetzung.
pub fn laden(zugang: &Zugang<'_>) -> Geladen<Einstellungen> {
    let roh: Geladen<Einstellungsdatei> = zugang.laden(Datei::Einstellungen);
    let wert = Einstellungen::aus_datei(&roh.wert);
    if roh.ersetzung.is_some() {
        return Geladen {
            wert,
            ersetzung: roh.ersetzung,
        };
    }
    match anlegen_falls_fehlt(zugang) {
        Ok(()) => Geladen {
            wert,
            ersetzung: None,
        },
        Err(fehler) => Geladen {
            wert,
            ersetzung: Some(Ersetzung {
                datei: zugang.pfad(Datei::Einstellungen),
                welche: Datei::Einstellungen,
                grund: Grund::NichtAnlegbar(fehler.to_string()),
                // Eine Datei, die es nicht gibt, hat keinen Inhalt zu sichern.
                beiseite: Beiseite::Nicht,
            }),
        },
    }
}

/// Wie das Schreiben von `notizordner` ausgegangen ist, wenn es kein
/// [`Schreibhindernis`] gab.
///
/// **Vollstaendig und ohne Auffangzweig**; der Rufer entscheidet ueber beide
/// Werte zusammen mit der Frage, ob der Ort wechselt (Tafel in Schritt 3.3 des
/// Plans `260926-1506_*_plan-home-menue-und-einstellbarer-ort.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Schreibausgang {
    /// Der neue Wert steht jetzt in der Datei, und jedes andere Byte ist
    /// geblieben.
    Geschrieben,
    /// Die Datei nannte schon denselben Ort; nichts wurde geschrieben.
    Unveraendert,
}

/// Warum `notizordner` nicht in `settings.toml` geschrieben wurde.
///
/// In jedem Fall ist die Datei danach Byte fuer Byte, was sie vorher war.
/// **Vollstaendig und ohne Auffangzweig**, wie [`Schreibhindernis::meldung`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Schreibhindernis {
    /// `settings.toml` ist ein symbolischer Verweis, auch ein verwaister.
    /// Traegt die Zeile, die der Nutzer von Hand eintragen kann.
    Verweis(String),
    /// Die Datei steht da, gibt ihren Bestand aber nicht her, oder sie traegt
    /// `notizordner` nicht als einen einzelnen Wert. Traegt den Befund.
    Beschaedigt(String),
    /// Die Datei liess sich nicht lesen, oder an ihrer Stelle steht etwas
    /// anderes als eine gewoehnliche Datei. Traegt den Befund.
    NichtLesbar(String),
    /// Das Ergebnis haette mehr geaendert als den Wert von `notizordner`; die
    /// zweite Lesung hat es abgefangen. Traegt den Befund.
    Intern(String),
    /// Das atomare Schreiben ist gescheitert. Traegt die Meldung des Systems.
    NichtGeschrieben(String),
}

impl Schreibhindernis {
    /// Der Satz fuer die Statuszeile.
    #[must_use]
    pub fn meldung(&self) -> String {
        match self {
            Schreibhindernis::Verweis(zeile) => format!(
                "settings.toml ist ein symbolischer Verweis, und KRK ersetzt ihn nicht durch eine Datei; der Ort bleibt, wie er ist. Von Hand in die Zieldatei eintragen: {zeile}"
            ),
            Schreibhindernis::Beschaedigt(befund) => format!(
                "settings.toml ist zuerst von Hand zu berichtigen, KRK schreibt sie so nicht: {befund}"
            ),
            Schreibhindernis::NichtLesbar(befund) => {
                format!("settings.toml ist nicht lesbar, KRK schreibt sie nicht: {befund}")
            }
            Schreibhindernis::Intern(befund) => format!(
                "settings.toml bleibt, wie sie ist: das Ergebnis hätte mehr geändert als den Notizordner ({befund})"
            ),
            Schreibhindernis::NichtGeschrieben(befund) => {
                format!("settings.toml ließ sich nicht schreiben und bleibt, wie sie war: {befund}")
            }
        }
    }
}

/// Die Zeile `notizordner = "…"` fuer einen Wert, gueltig kodiert.
///
/// `toml::Value::to_string` setzt `"` und `\` im Wert ein Escape voran; eine
/// Zusammensetzung von Hand taete das nicht.
fn schluesselzeile(wert: &str) -> String {
    format!("notizordner = {}", toml_text(wert))
}

/// Ein Text in seiner TOML-Schreibweise, samt Anfuehrungszeichen.
fn toml_text(wert: &str) -> String {
    toml::Value::String(wert.to_owned()).to_string()
}

/// Schreibt den Ort des Notizordners in `settings.toml` und aendert dabei kein
/// anderes Byte der Datei.
///
/// **Der eine Schreibweg in diese Datei neben ihrer Anlage**; gerufen von „Ort
/// waehlen…“ (H3 des Spec `260926-1451_*_spec-home-menue-und-einstellbarer-ort.md`).
/// Der Rufer haelt die Schreibsperre, weil es einen [`Zugang`] nur im
/// Durchgang gibt; gelesen wird darunter, damit „derselbe Ort?“ am Wert
/// entschieden wird, der **jetzt** in der Datei steht (S4 der Zweitlesung).
///
/// `derselbe` beantwortet, ob ein vorhandener Text denselben Ort nennt wie
/// `wert`. Der Rufer gibt dafuer seine Leseregel mit, so dass dieses Modul den
/// Heimordner nicht kennen muss; die Abhaengigkeit laeuft von `heimordner`
/// nach `ablage` und nicht umgekehrt.
///
/// Der Ablauf, vollstaendig ueber `symlink_metadata` am Pfad:
///
/// | An der Stelle | Ergebnis |
/// |---|---|
/// | ein symbolischer Verweis, auch ein verwaister | [`Schreibhindernis::Verweis`], nichts wird gelesen |
/// | nichts | die Auslieferungsfassung, mit dem neuen Wert |
/// | eine gewoehnliche Datei | ihre Bytes, weiter unten |
/// | etwas anderes, oder ein Fehler | [`Schreibhindernis::NichtLesbar`] |
///
/// Kein gueltiges UTF-8, ein Fehler des Lesers, ein doppelter Schluessel und
/// ein `notizordner`, der nicht als einzelner Wert dasteht (`notizordner.x =
/// …`, `[notizordner]`), ergeben [`Schreibhindernis::Beschaedigt`]. Nennt der
/// vorhandene Text denselben Ort, ist der Ausgang
/// [`Schreibausgang::Unveraendert`]. Sonst wird genau der Byte-Bereich des
/// Werts ersetzt, oder, fehlt der Schluessel, am Ende angehaengt, mit dem
/// Zeilenende der Datei. **Vor dem Schreiben wird das Ergebnis ein zweites Mal
/// gelesen** und muss `notizordner == wert` und jeden anderen Wert unveraendert
/// ergeben, sonst [`Schreibhindernis::Intern`].
#[must_use = "das Ergebnis sagt, ob die Datei geschrieben ist; wer es fallen laesst, meldet einen Ort, der nicht in der Datei steht"]
pub fn notizordner_schreiben(
    zugang: &Zugang<'_>,
    wert: &str,
    derselbe: impl Fn(&str) -> bool,
) -> Result<Schreibausgang, Schreibhindernis> {
    let pfad = zugang.pfad(Datei::Einstellungen);
    let text = match fs::symlink_metadata(&pfad) {
        Ok(art) if art.file_type().is_symlink() => {
            return Err(Schreibhindernis::Verweis(schluesselzeile(wert)));
        }
        Ok(art) if art.is_file() => {
            let bytes = fs::read(&pfad)
                .map_err(|fehler| Schreibhindernis::NichtLesbar(einzeilig(&fehler.to_string())))?;
            String::from_utf8(bytes).map_err(|_| {
                Schreibhindernis::Beschaedigt(String::from("keine gültige UTF-8-Folge"))
            })?
        }
        Ok(_) => {
            return Err(Schreibhindernis::NichtLesbar(String::from(
                "an ihrer Stelle steht keine gewöhnliche Datei",
            )));
        }
        Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => AUSLIEFERUNGSTEXT.to_owned(),
        Err(fehler) => {
            return Err(Schreibhindernis::NichtLesbar(einzeilig(
                &fehler.to_string(),
            )));
        }
    };

    let datei: Einstellungsdatei = toml::from_str(&text)
        .map_err(|fehler| Schreibhindernis::Beschaedigt(einzeilig(&fehler.to_string())))?;
    let neu = match &datei.notizordner {
        Some(vorhanden) => {
            let bereich = vorhanden.span();
            if !ist_einzelwert(&text[bereich.clone()], vorhanden.get_ref()) {
                return Err(Schreibhindernis::Beschaedigt(String::from(
                    "notizordner steht nicht als einzelner Wert in einer Zeile „notizordner = …“ da",
                )));
            }
            if let toml::Value::String(alt) = vorhanden.get_ref()
                && derselbe(alt)
            {
                return Ok(Schreibausgang::Unveraendert);
            }
            let mut neu = String::with_capacity(text.len() + wert.len());
            neu.push_str(&text[..bereich.start]);
            neu.push_str(&toml_text(wert));
            neu.push_str(&text[bereich.end..]);
            neu
        }
        None => angehaengt(&text, wert),
    };

    pruefen(&text, &neu, wert).map_err(Schreibhindernis::Intern)?;
    atomar::schreiben(&pfad, &mut neu.as_bytes())
        .map_err(|fehler| Schreibhindernis::NichtGeschrieben(einzeilig(&fehler.to_string())))?;
    Ok(Schreibausgang::Geschrieben)
}

/// Ob der Byte-Bereich, den der Leser fuer `notizordner` meldet, den Wert
/// selbst traegt.
///
/// Bei `notizordner.x = 1` und bei `[notizordner]` meldet der Leser den
/// Bereich des **Schluessels** oder der Ueberschrift; ein Ersatz dort machte
/// aus dem Schluessel einen anderen. Gelesen wird der Bereich deshalb als
/// eigener Wert, und er muss denselben Wert ergeben.
fn ist_einzelwert(bereich: &str, wert: &toml::Value) -> bool {
    toml::from_str::<toml::Table>(&format!("wert = {bereich}"))
        .is_ok_and(|tafel| tafel.get("wert") == Some(wert))
}

/// Der Text mit `notizordner` am Ende, fuer eine Datei ohne den Schluessel.
///
/// **Das Anhaengen gilt, solange die Datei keine Tabelle kennt**; siehe den
/// Modulkopf. Jede angehaengte Zeile endet wie die erste Zeile der Datei (O2
/// der Zweitlesung), damit eine Datei mit `\r\n` keine gemischten Zeilenenden
/// bekommt.
fn angehaengt(text: &str, wert: &str) -> String {
    let ende = zeilenende(text);
    let mut neu = String::from(text);
    if !neu.is_empty() {
        if !neu.ends_with('\n') {
            neu.push_str(ende);
        }
        neu.push_str(ende);
    }
    neu.push_str("# Der Ort des Notizordners, gesetzt ueber \"Ort waehlen\" im Menue Home.");
    neu.push_str(ende);
    neu.push_str(&schluesselzeile(wert));
    neu.push_str(ende);
    neu
}

/// `\r\n`, wenn die erste Zeile der Datei so endet, sonst `\n`.
fn zeilenende(text: &str) -> &'static str {
    match text.find('\n') {
        Some(stelle) if text[..stelle].ends_with('\r') => "\r\n",
        _ => "\n",
    }
}

/// Die zweite Lesung: der neue Text laedt als Einstellungsdatei, traegt
/// `notizordner == wert` und sonst genau die Werte des alten Texts.
fn pruefen(alt: &str, neu: &str, wert: &str) -> Result<(), String> {
    toml::from_str::<Einstellungsdatei>(neu).map_err(|fehler| einzeilig(&fehler.to_string()))?;
    let mut vorher: toml::Table =
        toml::from_str(alt).map_err(|fehler| einzeilig(&fehler.to_string()))?;
    let mut nachher: toml::Table =
        toml::from_str(neu).map_err(|fehler| einzeilig(&fehler.to_string()))?;
    let _ = vorher.remove("notizordner");
    if nachher.remove("notizordner") != Some(toml::Value::String(wert.to_owned())) {
        return Err(String::from("der neue Wert steht nicht als notizordner da"));
    }
    if vorher != nachher {
        return Err(String::from(
            "ein anderer Wert der Datei hätte sich geändert",
        ));
    }
    Ok(())
}

/// Schreibt die Auslieferungsfassung woertlich, falls die Datei fehlt.
///
/// Wiederholbar wie [`super::Ablageort::anlegen`] eine Ebene hoeher: eine
/// vorhandene Datei ist kein Fehler und wird nicht angefasst.
fn anlegen_falls_fehlt(zugang: &Zugang<'_>) -> io::Result<()> {
    let pfad = zugang.pfad(Datei::Einstellungen);
    if pfad.try_exists()? {
        return Ok(());
    }
    atomar::schreiben(&pfad, &mut AUSLIEFERUNGSTEXT.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die eingebettete Fassung traegt den Wert, den C11 ab Werk zusagt.
    #[test]
    fn die_auslieferungsfassung_nennt_terminal_app() {
        assert_eq!(Einstellungen::auslieferung().terminal, "com.apple.Terminal");
    }

    /// Sie traegt ihre Kommentare, und die sind der Zweck der Datei.
    #[test]
    fn die_auslieferungsfassung_traegt_ihre_kommentare() {
        assert!(
            AUSLIEFERUNGSTEXT.contains("mdls -name kMDItemCFBundleIdentifier"),
            "ohne das Kommando findet der Nutzer die Kennung seiner Anwendung nicht"
        );
        let kommentarzeilen = AUSLIEFERUNGSTEXT
            .lines()
            .filter(|zeile| zeile.starts_with('#'))
            .count();
        assert!(
            kommentarzeilen > 20,
            "die Auslieferungsfassung traegt nur {kommentarzeilen} Kommentarzeilen"
        );
    }
}
