//! Die beiden Enden der Datei: das Einlesen und die Sicherungsform (C2, C4).
//!
//! ```text
//!  Bytes von der Platte
//!         │
//!         └──> einlesen ──> in_gehaltene_form ──> der Stand des Editors
//!                                   ^                      │
//!                                   │                      │
//!                     jeder andere Text, der in            │
//!                     den Stand geraet (S37)               │
//!                                                          v
//!                                            sicherungsform ──> sichern
//!                                                                  │
//!                                                     ablage::atomar
//! ```
//!
//! # Die Zusage, die zwischen den beiden Enden steht
//!
//! **Der gehaltene Stand des Editors ist gueltiges UTF-8 ohne
//! Bytefolgenmarke und mit `\n` als einzigem Zeilenende.**
//!
//! Sie ist eine Eigenschaft, die das **Einlesen** herstellt, und genau deshalb
//! muss das **Sichern** sie nicht mehr herstellen. Beides zusammen ist eine
//! Aussage und nicht zwei: wer [`sicherungsform`] anschaut und dort keine
//! Wandlung von `\r\n` findet, sucht sie eine Zeile zu spaet.
//!
//! Von dieser Zusage leben drei andere Stellen, ohne sie zu wiederholen: der
//! Zeilenindex kennt ein einziges Zeilenende (`text::zeilen`), die Suche
//! normalisiert nichts und sucht buchstaeblich (`text::suche`), und die
//! Textmarken merken sich Zeilennummern, die in beiden Ansichten dasselbe
//! meinen. Jede von ihnen waere sonst eine zweite Meinung darueber, was eine
//! Zeile beendet.
//!
//! **Wer Text in den Stand bringt, der nicht aus [`einlesen`] kommt, fuehrt
//! ihn durch [`in_gehaltene_form`].** Das ist keine Hoeflichkeit, sondern die
//! Bedingung, unter der die drei Stellen oben rechnen duerfen. Der Fall, der
//! ansteht, ist der Ersatztext des Suchen-und-Ersetzens aus C5 (Schritt 37):
//! er kommt aus einem Eingabefeld und kann ein `\r` tragen, wenn er
//! hineinkopiert wurde. Eine eigene Wandlung an jener Stelle waere die zweite
//! Normalisierungsstelle im Programm, und die erste ist diese hier.
//!
//! # Der Preis dieser Wahl, ausgeschrieben
//!
//! KRK schreibt beim Sichern **immer** Unix-Zeilenenden, **immer** einen
//! abschliessenden Umbruch und **nie** eine Bytefolgenmarke, unabhaengig von
//! der Form, die die Datei mitbrachte. Der Nutzer hat das am 260808-0043
//! entschieden und ist damit der Empfehlung des Datensatzes
//! `decisions/260808-0021_*_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md`
//! **nicht** gefolgt; empfohlen war, dass die Datei ihre Form behaelt und der
//! Editor sie sich beim Lesen merkt.
//!
//! Der Preis steht im Datensatz und gehoert hierher, weil er sonst nur dort
//! steht, wo beim naechsten Defekt niemand nachschlaegt:
//!
//! - **Das Sichern aendert Zeilen, die der Nutzer nicht angefasst hat.** Wer
//!   eine Zeile in einer Datei mit Windows-Zeilenenden aendert und sichert,
//!   hat danach eine Aenderung in **jeder** Zeile der Datei.
//! - **Eine fremde Datei aus einem Windows-Projekt kommt veraendert zurueck.**
//!   In einem versionierten Verzeichnis, und KRK bekommt in einer spaeteren
//!   Runde eine Git-Anbindung, ist das der Unterschied zwischen einer lesbaren
//!   Aenderung und einer unbrauchbaren.
//!
//! Das ist angenommen und kein Defekt. Wer diesen Kopf liest, weil ein Nutzer
//! sich ueber genau diese Wirkung beschwert hat, hat den richtigen Ort
//! gefunden und die falsche Erwartung: die Antwort ist nicht ein Sonderfall
//! hier, sondern eine neue Frage an den Nutzer.
//!
//! **Eine Folge, die der Rohansicht gilt:** weil die Wandlung beim Einlesen
//! geschieht, zeigt auch die Rohansicht aus C3 keine Wagenrucklaufzeichen
//! mehr. Nach der Wahl des Nutzers ist die Form der Datei fuer das Sichern
//! ohne Belang, und ein sichtbares `\r` waere ein Zeichen, das beim Sichern
//! ohnehin verschwindet.
//!
//! # Was hier nicht steht
//!
//! **Kein Weg von einem Pfad zu einem Stand.** [`einlesen`] nimmt Bytes und
//! keinen Pfad, obwohl [`sichern`] einen Pfad nimmt, und die Unwucht ist
//! Absicht: die Groessen- und Typpruefung aus C2 muss **vor** dem Lesen
//! laufen, damit eine Datei ueber der Grenze zu keinem Zeitpunkt vollstaendig
//! im Arbeitsspeicher steht. Ein `lesen(pfad)` an dieser Stelle waere die
//! zweite Stelle, die eine Datei oeffnet, und die erste ohne jede Pruefung.
//! Schritt 10 setzt die Pruefung davor und macht daraus den einen Weg, den
//! beide Einstiege aus C2 nehmen.

use std::borrow::Cow;
use std::io;
use std::path::Path;

/// Die Bytefolgenmarke, wie `String::from_utf8` sie liefert: ein Zeichen am
/// Anfang der Zeichenkette und keine drei Bytes mehr.
const BYTEFOLGENMARKE: char = '\u{feff}';

/// Aus den Bytes einer Datei den gehaltenen Stand des Editors.
///
/// `None` heisst: kein gueltiges UTF-8, also keine Textdatei im Sinne von C2.
/// Der Fehler traegt nichts, was der Aufrufer benutzt; welchen Satz der Nutzer
/// zu sehen bekommt, entscheidet der Abweisungsgrund aus Schritt 10 und nicht
/// diese Stelle.
///
/// Gewandelt wird ueber [`String::from_utf8`], denselben Weg, ueber den die
/// Vorschau entscheidet, ob eine Datei Text ist
/// (`krk-ui/src/vorschaumodell.rs`). Zwei Antworten auf die Frage "ist das
/// Text" haetten sonst zwei verschiedene Dateimengen bejaht.
pub fn einlesen(bytes: Vec<u8>) -> Option<String> {
    String::from_utf8(bytes).ok().map(in_gehaltene_form)
}

/// Die **eine** Stelle, die einen Text in die gehaltene Form bringt.
///
/// Sie schneidet eine fuehrende Bytefolgenmarke ab und macht `\r\n` sowie
/// einzelne `\r` zu `\n`. Beides zusammen, weil beides dieselbe Zusage traegt
/// und ein Aufrufer, der nur die Haelfte bekaeme, die andere selbst schreiben
/// muesste.
///
/// Abgeschnitten wird allein die **fuehrende** Marke. Ein `U+FEFF` mitten im
/// Text ist ein Leerzeichen ohne Breite und Umbruchverbot, also ein Zeichen
/// des Nutzers, und bleibt stehen.
///
/// Ein Text, der die Form schon hat, kommt ohne eine einzige Kopie zurueck.
pub fn in_gehaltene_form(text: String) -> String {
    if !text.starts_with(BYTEFOLGENMARKE) && !text.contains('\r') {
        return text;
    }

    let ohne_marke = text.strip_prefix(BYTEFOLGENMARKE).unwrap_or(&text);
    let mut gewandelt = String::with_capacity(ohne_marke.len());
    let mut rest = ohne_marke;
    while let Some(stelle) = rest.find('\r') {
        gewandelt.push_str(&rest[..stelle]);
        gewandelt.push('\n');
        // Das `\n` eines `\r\n` ist damit schon geschrieben und darf nicht
        // ein zweites Mal kommen, sonst wuerde aus jeder Windows-Zeile zwei.
        let danach = &rest[stelle + 1..];
        rest = danach.strip_prefix('\n').unwrap_or(danach);
    }
    gewandelt.push_str(rest);
    gewandelt
}

/// Was von einem Stand auf die Platte geht.
///
/// Genau ein Unterschied zum Stand, und der steht in der Fallunterscheidung
/// unten. Zeilenenden wandelt diese Funktion **nicht**: der Stand traegt
/// keine anderen, siehe den Modulkopf.
///
/// Die drei Faelle sind ueberschneidungsfrei und vollstaendig:
///
/// - **Der leere Stand bleibt leer.** Eine Datei ohne Zeile braucht keinen
///   Zeilenabschluss, und ein angehaengtes `\n` machte aus einer Datei von
///   null Bytes eine von einem.
/// - **Ein Stand, der auf `\n` endet, geht unveraendert hinaus.** Auch einer,
///   der auf mehrere endet: die leeren Zeilen am Dateiende sind Text des
///   Nutzers, und "genau ein abschliessender Umbruch" heisst, dass genau einer
///   **angehaengt** wird, nicht dass hinten aufgeraeumt wird.
/// - **Jeder andere Stand bekommt einen `\n` angehaengt.**
pub fn sicherungsform(stand: &str) -> Cow<'_, str> {
    if stand.is_empty() || stand.ends_with('\n') {
        Cow::Borrowed(stand)
    } else {
        Cow::Owned(format!("{stand}\n"))
    }
}

/// Schreibt den Stand des Editors in die Datei.
///
/// Geschrieben wird ueber [`crate::ablage::atomar`], denselben Weg, den die
/// vier Ablagedateien nehmen: erst vollstaendig in eine Nachbardatei, dann
/// `rename`. Ein Absturz mittendrin laesst die alte Datei stehen, wie sie war,
/// und ein zweiter Schreibweg im Programm entsteht nicht.
///
/// Eine Bytefolgenmarke schreibt diese Stelle nicht. Sie stellt sie auch nicht
/// ab: was am Anfang des Standes steht, ist Text des Nutzers, und der Stand
/// traegt dort keine Marke, weil [`einlesen`] sie abgeschnitten hat.
pub fn sichern(ziel: &Path, stand: &str) -> io::Result<()> {
    crate::ablage::atomar::schreiben(ziel, &sicherungsform(stand))
}
