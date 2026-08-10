//! Die beiden Enden der Datei: das Oeffnen samt Pruefung, das Einlesen und die
//! Sicherungsform (C2, C4).
//!
//! ```text
//!  ein Pfad
//!     │
//!     └──> oeffnen ──> Abweisung (kein gueltiges Ziel, zu gross, kein Text)
//!            │
//!            │  die Bytes, und zwar erst nach der Groessenpruefung
//!            v
//!         einlesen ──> in_gehaltene_form ──> der Stand des Editors
//!                              ^                      │
//!                              │                      │
//!                der Stand aus der Textflaeche        │
//!                und der Ersatztext aus C5            │
//!                                                     v
//!                                       sicherungsform ──> sichern
//!                                                             │
//!                                                ablage::atomar
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
//! Bedingung, unter der die drei Stellen oben rechnen duerfen. Es sind **zwei**
//! Faelle, und beide sind gebaut:
//!
//! - **Der Stand, den die `NSTextView` des Editors zurueckgibt** (Schritt 9).
//!   Das ist der groessere der beiden: eine `NSTextView` bewahrt eingefuegten
//!   Text zeichengetreu auf, also bringt ein Einfuegen aus einer Windows-Quelle
//!   `\r\n` mit.
//! - **Der Ersatztext des Suchen-und-Ersetzens aus C5** (Schritt 37). Er kommt
//!   aus einem Eingabefeld und kann ein `\r` tragen, wenn er hineinkopiert
//!   wurde.
//!
//! Beide liegen in `krk-ui/src/editormodell.rs`, der erste in `bearbeiten`, der
//! zweite in `ersetzung_vorbereiten`; dessen Modulkopf fuehrt sie aus, statt
//! dass dieser sie ein zweites Mal beschreibt. Eine eigene Wandlung an einer
//! der beiden Stellen waere die zweite Normalisierungsstelle im Programm, und
//! die erste ist diese hier.
//!
//! # Wer aus einem Textbestand liest, muss ihn nachziehen
//!
//! Die Wandlung geschieht **auf dem Weg** in den Stand, und der Bestand, aus
//! dem sie las, bleibt dabei stehen, wie er war. Wer einen solchen Bestand
//! fuehrt — die `NSTextView` des Editors ist der eine Fall —, hat danach zwei
//! Texte, die sich um die gewandelten Zeichen unterscheiden, und jede Stelle
//! hinter der ersten Wandlung zeigt in den beiden auf Verschiedenes.
//!
//! Zwei Stuecke stehen dafuer bereit, und beide halten sich an die eine
//! Wandlung, statt ihre Regeln zu wiederholen: [`ist_in_gehaltener_form`] sagt
//! im Voraus, ob ueberhaupt gewandelt wird, und [`versatz_nach_der_wandlung`]
//! sagt, wohin eine Stelle dabei wandert. Der Defekt, der beides verlangt hat,
//! ist `260810-0215`.
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
//! # Der eine Weg von einem Pfad zu einem Stand
//!
//! **[`oeffnen`] ist die einzige Stelle im Programm, die eine Datei fuer den
//! Editor liest.** Beide Einstiege aus C2, F4 und das Menue, rufen sie, und der
//! Sprung auf eine Textmarke aus C6 ruft sie ebenfalls. Genau das meint C2 mit
//! "beide Einstiege legen dieselbe Pruefung an"; ein zweiter Leseweg daneben
//! waere die zweite Wahrheit darueber, welche Datei der Editor annimmt, und die
//! erste Abweichung zwischen beiden faende keine Pruefung. Es ist derselbe
//! Zuschnitt, den `krk-ui`s `kommandos::pfadeingabe` fuer den Pfad zieht.
//!
//! [`einlesen`] nimmt weiterhin Bytes und keinen Pfad. Die Unwucht gegenueber
//! [`sichern`] ist Absicht und jetzt erst recht: die Groessenpruefung laeuft
//! **vor** dem Lesen, damit eine Datei ueber der Grenze zu keinem Zeitpunkt
//! vollstaendig im Arbeitsspeicher steht (sechstes Abnahmekriterium von C2).
//! Wer die Bytes schon hat, hat die Grenze schon ueberschritten.

use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Die Bytefolgenmarke, wie `String::from_utf8` sie liefert: ein Zeichen am
/// Anfang der Zeichenkette und keine drei Bytes mehr.
const BYTEFOLGENMARKE: char = '\u{feff}';

/// Bis zu welcher Groesse der Editor eine Datei vollstaendig einliest (C2).
///
/// Drei Aussagen ueber diese Zahl, und alle drei gehoeren hierher:
///
/// - **Der Nutzer hat sie am 260808-0017 gewaehlt**, im Datensatz
///   `decisions/260807-2147_*_welche-dateien-oeffnet-der-editor-ueberhaupt.md`,
///   und damit gegen die Moeglichkeit, die Grenze der Vorschau zu erben.
/// - **Sie ist die zweite Zahl neben `TEXTGRENZE`**, den 1 MB der Vorschau in
///   `krk-ui/src/vorschaumodell.rs`. Zwei Zahlen fuer dieselbe Frage sind
///   angenommen und kein Versehen: beide tragen **dieselbe Regel**, naemlich
///   eine Obergrenze fuer das vollstaendige Einlesen in den Arbeitsspeicher.
///   Verschieden ist allein, wie viel die jeweilige Handlung rechtfertigt, denn
///   Ansehen ist nicht Bearbeiten.
/// - `speculation:` **Sie ist ein Vorschlag und keine gemessene Groesse.** Eine
///   Messung, ab welcher Dateigroesse das Oeffnen im Editor spuerbar wird, gibt
///   es nicht. Sie liegt weit unter dem, was das Referenzgeraet verkraftet, und
///   weit ueber den Dateien, die man von Hand bearbeitet.
pub const EDITORGRENZE: u64 = 16 * 1024 * 1024;

/// Der Editor nimmt mehr an als die Vorschau; genau das war der Grund fuer die
/// zweite Zahl. Beim Uebersetzen geprueft und nicht erst beim Pruefen, in
/// derselben Form wie `BILDGRENZE > TEXTGRENZE` in `vorschaumodell.rs`.
///
/// Verglichen wird gegen die 1 MB der Vorschau **als Zahl und nicht als
/// Bezug**: `krk-core` kennt `krk-ui` nicht, und die Abhaengigkeit laeuft nur
/// in die andere Richtung.
///
/// **Diese Zusicherung ist die eine Haelfte, und die andere steht drueben.**
/// Sie faengt ein Absenken von [`EDITORGRENZE`] unter die Vorschaugrenze. Ein
/// **Anheben** der Vorschaugrenze ueber 16 MB faengt sie nicht, weil jene Zahl
/// in der anderen Kiste steht; das faengt die Gegenrichtung an `TEXTGRENZE`
/// selbst (`krk-ui/src/vorschaumodell.rs`, wo beide Zahlen benennbar sind).
/// Zusammen sind es beide Richtungen. Der Defekt, der die fehlende Haelfte
/// gemeldet hat, ist `260809-1610`.
const _: () = assert!(EDITORGRENZE > 1024 * 1024);

/// Warum der Editor eine Datei nicht oeffnet (C2).
///
/// **Drei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig.**
/// Sie sind verschieden, weil das neunte Abnahmekriterium von C2 verlangt, "zu
/// gross" von "nicht als Text lesbar" zu unterscheiden: der Nutzer soll wissen,
/// ob seine Datei zu gross ist oder gar kein Text, denn die eine Antwort laedt
/// zum Teilen der Datei ein und die andere nicht.
///
/// Jeder Wert traegt den Pfad, weil sein [`meldung`](Self::meldung) in die
/// Statuszeile aus C1 geht und dort allein steht. Eine zweite Meldeflaeche
/// entsteht nicht; das hat der Nutzer im Datensatz mitentschieden.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Abweisung {
    /// Nichts, was ein Texteditor oeffnen koennte.
    ///
    /// Der Ordner ist der Fall, den der Datensatz namentlich nennt: er hat
    /// keinen Inhalt, den ein Texteditor zeigen koennte, und braucht deshalb
    /// keine eigene Regel. Dieselbe Antwort bekommen der fehlende Pfad, das
    /// fehlende Leserecht und alles, was keine gewoehnliche Datei ist.
    KeinGueltigesZiel {
        /// Der Pfad, wie der Aufrufer ihn uebergeben hat.
        pfad: PathBuf,
        /// Woran es lag, in einem Satzteil: der Systemfehler oder die Art.
        grund: String,
    },
    /// Ueber [`EDITORGRENZE`], also gar nicht erst gelesen.
    ZuGross {
        /// Der Pfad, wie der Aufrufer ihn uebergeben hat.
        pfad: PathBuf,
        /// Die Groesse in Bytes, wie `stat(2)` sie vor dem Lesen gemeldet hat.
        groesse: u64,
    },
    /// Gelesen, aber kein gueltiges UTF-8.
    ///
    /// **Das ist der Wert, an dem die bindende Zusage des Datensatzes haengt:**
    /// kein Weg darf eine Datei beim Sichern veraendern, die der Editor nicht
    /// vollstaendig und verlustfrei als Text gelesen hat. Wer hier abweist,
    /// statt mit Ersatzzeichen zu oeffnen, haelt sie ein.
    NichtAlsTextLesbar {
        /// Der Pfad, wie der Aufrufer ihn uebergeben hat.
        pfad: PathBuf,
    },
}

impl Abweisung {
    /// Der Satz fuer die Statuszeile aus C1.
    ///
    /// Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig: ein
    /// vierter Grund haelt den Bau an und erzwingt einen vierten Satz.
    ///
    /// **Die Byteangaben stehen roh und nicht in MB.** Der menschenlesbare
    /// Groessensatz des Programms ist `menge` in
    /// `krk-ui/src/kommandos/operationen.rs`, und der liegt in der anderen
    /// Kiste. Ihn hier nachzubauen hiesse, zwei Schreibweisen fuer dieselbe
    /// Groesse zu haben; die Ansicht kann stattdessen aus den Feldern des
    /// Wertes ihren eigenen Satz bauen, wenn sie einen schoeneren will.
    pub fn meldung(&self) -> String {
        match self {
            Abweisung::KeinGueltigesZiel { pfad, grund } => {
                format!(
                    "{} lässt sich nicht im Editor öffnen: {grund}",
                    pfad.display()
                )
            }
            Abweisung::ZuGross { pfad, groesse } => format!(
                "{} ist mit {groesse} Bytes zu groß für den Editor; die Grenze liegt bei {EDITORGRENZE} Bytes",
                pfad.display()
            ),
            Abweisung::NichtAlsTextLesbar { pfad } => {
                format!(
                    "{} ist keine Textdatei und wird nicht geöffnet",
                    pfad.display()
                )
            }
        }
    }
}

/// Die **eine** Groessen- und Typpruefung vor dem Oeffnen (C2, C6).
///
/// Liefert den fertigen Stand des Editors oder einen benannten Grund, aus dem
/// nichts geoeffnet wird. Warum es nur diese eine Stelle gibt, steht im
/// Modulkopf.
///
/// # Die Reihenfolge ist bindend
///
/// 1. **`metadata` und nicht `symlink_metadata`**, damit eine Verknuepfung nach
///    dem behandelt wird, worauf sie zeigt. Eine Verknuepfung auf eine
///    Textdatei ist als Ziel des Oeffnens dieselbe Textdatei; in der Dateiliste
///    meldet der Leser sie weiter als Verknuepfung, und die beiden Fragen sind
///    verschieden. Dieselbe Wahl und derselbe Grund wie in
///    `krk-ui`s `kommandos::pfadeingabe::pruefen`.
/// 2. **Alles, was keine gewoehnliche Datei ist, faellt hier heraus**, der
///    Ordner voran. Diese Frage steht **vor** dem Oeffnen und nicht erst vor
///    dem Lesen, weil ein `File::open` auf eine benannte Roehre so lange
///    haengt, bis jemand hineinschreibt; das waere eine angehaltene Anwendung
///    ohne Meldung. Wovor diese Lage schuetzt und wovor nicht, steht unten
///    unter "Geprueft wird der Pfad und nicht der Deskriptor".
/// 3. **Die Groesse wird vor dem Lesen geprueft**, so wie die Vorschau es fuer
///    ihre beiden Grenzen tut. Eine Protokolldatei von zwei Gigabyte darf nicht
///    erst eingelesen und dann abgewiesen werden; sie steht damit zu keinem
///    Zeitpunkt vollstaendig im Arbeitsspeicher.
/// 4. **Erst danach werden die Bytes gelesen und gewandelt.** Scheitert die
///    Wandlung, wird abgewiesen und nicht mit Ersatzzeichen geoeffnet.
///
/// # Die Grenze wird eingehalten und nicht nur vorhergesagt
///
/// Schritt 3 fragt `stat(2)`, und zwischen `stat` und `read` kann eine Datei
/// wachsen. Deshalb liest Schritt 4 hoechstens [`EDITORGRENZE`] `+ 1` Bytes und
/// weist ab, sobald das eine Byte zuviel ankommt. Der Unterschied ist nicht
/// akademisch: ohne diese Schranke waere "die Datei steht nie vollstaendig im
/// Speicher" eine Vorhersage aus einer alten Auskunft, mit ihr ist es eine
/// Eigenschaft der Bauart. Eine wachsende Protokolldatei ist genau der Fall,
/// fuer den ein Nutzer den Editor aufmacht.
///
/// # Geprueft wird der Pfad und nicht der Deskriptor
///
/// Schritt 2 und 3 fragen `stat(2)` auf den **Pfad**, und Schritt 4 oeffnet
/// denselben **Pfad** ein zweites Mal. Zwischen beiden Aufrufen liegt ein
/// Fenster, in dem der Pfad auf etwas anderes zeigen kann. Die Reihenfolge oben
/// ist damit eine Pruefung des gewoehnlichen Betriebs und **keine Eigenschaft
/// der Bauart**; wer den vorigen Absatz gelesen hat, unterscheide beides:
///
/// - **Wachsen** faengt die Schranke, weil sie gelesene Bytes zaehlt und keine
///   Auskunft. Das gilt auch, wenn der Pfad in der Spanne auf eine **groessere**
///   Datei zeigt: gelesen wird hoechstens ein Byte ueber der Grenze, und dann
///   steht `ZuGross` da.
/// - **Ein Austausch gegen eine benannte Roehre** faengt sie nicht: dann haengt
///   das `File::open` doch, der Arbeitsfaden des Ladevorgangs endet nie, und der
///   Editor oeffnet kommentarlos nichts. Der Fall braucht ein Wettrennen und ist
///   selten, aber nicht bloss theoretisch.
///
/// Die Fassung, die auch das zu einer Eigenschaft der Bauart machte, prueft den
/// **Deskriptor**: nicht blockierend oeffnen, dann `fstat` darauf, dann Typ und
/// Groesse. Sie ist mit einem Merkmal an `OpenOptions` nicht getan, denn
/// `O_NONBLOCK` gehoert vor dem Lesen wieder abgeschaltet — POSIX laesst seine
/// Wirkung auf gewoehnliche Dateien offen, und `speculation:` auf einem
/// Netzlaufwerk koennte ein Lesen sonst mit `EAGAIN` scheitern —, und das
/// Abschalten ist ein `fcntl`, also eine vierte Bindung in `verzeichnis::sys`.
/// Sie zieht ausserdem den Nachweis "ohne gelesen zu werden" in
/// `tests/text.rs` mit, der heute an den Rechten haengt und dann am Oeffnen
/// scheiterte. Ob der Aufwand die Seltenheit des Falls rechtfertigt, ist offen
/// und steht in
/// `issues/260809-1652_*_die-typpruefung-steht-auf-dem-pfad-und-nicht-auf-dem-deskriptor.md`.
pub fn oeffnen(pfad: &Path) -> Result<String, Abweisung> {
    let kein_ziel = |grund: String| Abweisung::KeinGueltigesZiel {
        pfad: pfad.to_path_buf(),
        grund,
    };

    let angaben = std::fs::metadata(pfad).map_err(|fehler| kein_ziel(fehler.to_string()))?;

    if !angaben.is_file() {
        return Err(kein_ziel(String::from(if angaben.is_dir() {
            "ein Ordner hat keinen Text, den der Editor zeigen könnte"
        } else {
            "das ist keine gewöhnliche Datei"
        })));
    }

    if angaben.len() > EDITORGRENZE {
        return Err(Abweisung::ZuGross {
            pfad: pfad.to_path_buf(),
            groesse: angaben.len(),
        });
    }

    let mut datei = File::open(pfad).map_err(|fehler| kein_ziel(fehler.to_string()))?;
    let mut bytes = Vec::with_capacity(angaben.len() as usize);
    datei
        .by_ref()
        .take(EDITORGRENZE + 1)
        .read_to_end(&mut bytes)
        .map_err(|fehler| kein_ziel(fehler.to_string()))?;
    if bytes.len() as u64 > EDITORGRENZE {
        // Die Datei ist zwischen `stat` und `read` gewachsen. Gemeldet wird die
        // Groesse von jetzt und nicht die von vorhin, denn die alte war nie
        // wahr; laesst sie sich nicht mehr erheben, steht die untere Schranke
        // da, die wir sicher wissen.
        let groesse = datei
            .metadata()
            .map(|angaben| angaben.len())
            .unwrap_or(bytes.len() as u64);
        return Err(Abweisung::ZuGross {
            pfad: pfad.to_path_buf(),
            groesse,
        });
    }

    einlesen(bytes).ok_or(Abweisung::NichtAlsTextLesbar {
        pfad: pfad.to_path_buf(),
    })
}

/// Aus den Bytes einer Datei den gehaltenen Stand des Editors.
///
/// `None` heisst: kein gueltiges UTF-8, also keine Textdatei im Sinne von C2.
/// Der Fehler traegt nichts, was der Aufrufer benutzt; welchen Satz der Nutzer
/// zu sehen bekommt, entscheidet [`Abweisung::NichtAlsTextLesbar`] und nicht
/// diese Stelle.
///
/// Wer einen Pfad hat und keine Bytes, nimmt [`oeffnen`]: dort steht die
/// Groessen- und Typpruefung davor.
///
/// Gewandelt wird ueber [`String::from_utf8`], denselben Weg, ueber den die
/// Vorschau entscheidet, ob eine Datei Text ist
/// (`krk-ui/src/vorschaumodell.rs`). Zwei Antworten auf die Frage "ist das
/// Text" haetten sonst zwei verschiedene Dateimengen bejaht.
pub fn einlesen(bytes: Vec<u8>) -> Option<String> {
    String::from_utf8(bytes).ok().map(in_gehaltene_form)
}

/// Ob ein Text die gehaltene Form schon hat.
///
/// **Die eine Stelle, die diese Frage beantwortet.** [`in_gehaltene_form`]
/// nimmt an ihr ihren kurzen Weg, und der Editor fragt sie, um zu erfahren, ob
/// seine Textflaeche Zeichen traegt, die der gehaltene Stand nicht traegt;
/// siehe den Modulkopf. Zwei Formulierungen derselben Frage waeren die erste
/// Gelegenheit, sie verschieden zu schreiben, und die Wandlung liefe dann gegen
/// eine andere Bedingung als die Pruefung.
pub fn ist_in_gehaltener_form(text: &str) -> bool {
    !text.starts_with(BYTEFOLGENMARKE) && !text.contains('\r')
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
    if ist_in_gehaltener_form(&text) {
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

/// Wohin ein Byteversatz wandert, wenn sein Text durch [`in_gehaltene_form`]
/// geht.
///
/// `vorher` ist der ungewandelte Text, `versatz` eine Stelle darin auf einer
/// Zeichengrenze, `nachher` das Ergebnis der Wandlung. Zurueck kommt die
/// entsprechende Stelle in `nachher`, ebenfalls als Byteversatz.
///
/// Gebraucht wird sie an einer Stelle: der Editor richtet seine Textflaeche auf
/// den gehaltenen Stand, nachdem ein eingefuegtes `\r\n` beide
/// auseinandergebracht hat, und die Schreibmarke des Nutzers soll dabei stehen
/// bleiben, wo sie stand.
///
/// # Gerechnet wird vom Ende her
///
/// Das ist der Grund, aus dem diese Rechnung **keine Regel der Wandlung
/// wiederholt.** Was hinter `versatz` steht, wandelt sich unabhaengig von
/// allem davor; wer den Rest wandelt und seine Laenge von der des Ergebnisses
/// abzieht, bekommt die gesuchte Stelle, ohne zu wissen, welche Zeichen
/// unterwegs wegfallen. Eine Zaehlung der weggefallenen Zeichen muesste
/// dagegen bei jeder kuenftigen Regel von [`in_gehaltene_form`] nachgezogen
/// werden, und die erste vergessene Nachziehung faende keine Pruefung.
///
/// **Ein Fall geht dabei um ein Zeichen daneben, und er steht hier statt
/// verschwiegen zu werden:** steht genau an `versatz` eine Bytefolgenmarke, so
/// schneidet die Wandlung des Restes sie als **seine** fuehrende ab, waehrend
/// die des ganzen Textes sie als Zeichen des Nutzers stehen laesst. Die Stelle
/// liegt dann um dieses eine Zeichen zu weit hinten. Erreichbar ist der Fall
/// allein mit zwei Marken in einem Text, von denen die erste ganz vorn steht;
/// ein Sonderfall dafuer waere genau die Regelwiederholung, die der Absatz
/// darueber vermeidet.
pub fn versatz_nach_der_wandlung(vorher: &str, versatz: usize, nachher: &str) -> usize {
    // Ein Versatz hinter dem Text oder neben einer Zeichengrenze hat keinen
    // Rest; die Antwort ist dann das Ende des gewandelten Textes.
    let Some(rest) = vorher.get(versatz..) else {
        return nachher.len();
    };
    nachher
        .len()
        .saturating_sub(in_gehaltene_form(rest.to_owned()).len())
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
