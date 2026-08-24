//! Die beiden Enden der Datei: das Oeffnen samt Pruefung, das Einlesen und die
//! Sicherungsform (C2, C4).
//!
//! ```text
//!  ein Pfad
//!     │
//!     └──> lesen ──> Textstand ──> oeffnen ──> Abweisung (kein gueltiges
//!            │           │                       Ziel, zu gross, kein Text)
//!            │           └───────> Zugang::text_laden ──> der Notizzettel
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
//! **Die Wandlung traegt zwei Namen und ist eine.** [`gehaltene_form`] nimmt
//! einen geliehenen Text und gibt ihn geliehen zurueck, solange nichts zu
//! wandeln ist; [`in_gehaltene_form`] nimmt ihn uebernommen und gibt ihn
//! uebernommen zurueck. Die Regeln stehen in der ersten, die zweite ist eine
//! Zeile darueber, und welche von beiden ein Aufrufer nimmt, entscheidet allein,
//! ob er den Text ohnehin besitzt.
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
//! **[`lesen`] ist die einzige Stelle im Programm, die einen Pfad daraufhin
//! ansieht, ob eine Textdatei dahintersteht.** Beide Einstiege des Editors aus
//! C2, F4 und das Menue, kommen ueber [`oeffnen`] dort an, der Sprung auf eine
//! Textmarke aus C6 ebenfalls, und seit der Runde 9 auch der Notizzettel ueber
//! `ablage::Zugang::text_laden`. Genau das meint C2 mit "beide Einstiege legen
//! dieselbe Pruefung an"; ein zweiter Leseweg daneben waere die zweite Wahrheit
//! darueber, welche Datei KRK als Text annimmt, und die erste Abweichung
//! zwischen beiden faende keine Pruefung. Es ist derselbe Zuschnitt, den
//! `krk-ui`s `kommandos::pfadeingabe` fuer den Pfad zieht.
//!
//! **Die zwei Aufrufer uebersetzen denselben Befund verschieden, und das ist
//! der Grund fuer die Trennung.** Der Editor weist ab und wirft die Bytes weg;
//! der Notizzettel legt sie beiseite und arbeitet mit einem leeren Zettel
//! weiter. [`Textstand::Unlesbar`] traegt den offenen Deskriptor deshalb mit,
//! und [`Abweisung`] tut es nicht.
//!
//! **Daneben stehen zwei weitere Fragen, und beide gehen durch dieselbe Tuer.**
//! Keine von ihnen ist ein zweiter Leseweg im Sinne des Absatzes darueber; es
//! ist derselbe Eingang, dreimal verschieden befragt:
//!
//! - **[`bis_zur_grenze_lesen`]:** "gib mir die Bytes, aber hoechstens so
//!   viele, sonst gar nichts". Ueber der Grenze weist es ab, und der Aufrufer
//!   weiss ueber den Inhalt der Datei nichts.
//! - **[`anlesen`]:** "gib mir die ersten N Bytes und sage nichts ueber das,
//!   was dahinter steht". Es weist wegen der Groesse nie ab und kann
//!   [`Lesehindernis::ZuGross`] deshalb gar nicht liefern.
//!
//! **Der Unterschied zwischen den beiden ist keine Feinheit, sondern der Grund
//! fuer die zweite Fassung.** Wer eine Ueberschrift, ein Feld oder die ersten
//! Zeilen einer Datei braucht, ist mit einer Abweisung nicht bedient: gesucht
//! ist ein Wert am Dateianfang, und eine Datei kann im Ganzen weit ueber der
//! Grenze liegen und ihn trotzdem in ihren ersten hundert Bytes tragen. Unter
//! einer Grenze von 64 KB liefert [`bis_zur_grenze_lesen`] fuer sie nichts und
//! [`anlesen`] die Ueberschrift. Der Unterschied zu [`lesen`] steht an den
//! Huellen selbst.
//!
//! [`einlesen`] nimmt weiterhin Bytes und keinen Pfad. Die Unwucht gegenueber
//! [`sichern`] ist Absicht und jetzt erst recht: die Groessenpruefung laeuft
//! **vor** dem Lesen, damit eine Datei ueber der Grenze zu keinem Zeitpunkt
//! vollstaendig im Arbeitsspeicher steht (sechstes Abnahmekriterium von C2).
//! Wer die Bytes schon hat, hat die Grenze schon ueberschritten.

use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::{Read, Seek};
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
        /// Die Groesse in Bytes, wie `fstat(2)` sie vor dem Lesen gemeldet hat.
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

/// Warum eine Datei nicht als Text angenommen wird, samt der Angabe, die dazu
/// gehoert.
///
/// **Zwei Werte, vollstaendig und ohne Auffangzweig.** Sie sind dieselbe
/// Unterscheidung, die [`Abweisung`] fuer den Editor trifft — "zu gross" laedt
/// zum Teilen der Datei ein, "kein Text" nicht —, und sie steht hier ein
/// zweites Mal, weil [`lesen`] den Befund erhebt und die Uebersetzung in eine
/// Meldung nicht mehr kennt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unlesbarkeit {
    /// Ueber [`EDITORGRENZE`], also gar nicht erst gelesen. Traegt die Groesse
    /// in Bytes, wie sie zum Zeitpunkt der Feststellung galt.
    ZuGross(u64),
    /// Gelesen, aber kein gueltiges UTF-8.
    KeinText,
}

/// Was hinter einem Pfad steht, gemessen an dem, was KRK als Textdatei annimmt.
///
/// **Der eine Befund, und zwei Uebersetzungen leben davon.** Der Editor macht
/// daraus eine [`Abweisung`] ([`oeffnen`]), der Notizzettel einen leeren Zettel
/// samt beiseitegelegtem Inhalt (`ablage::Zugang::text_laden`). Vor der Runde 9
/// stand die Pruefung allein in [`oeffnen`] und warf dabei genau das weg, was
/// das Beiseitelegen braucht: die Bytes und den offenen Deskriptor. Ein zweiter
/// Leser daneben waere die zweite Wahrheit darueber, was eine Textdatei ist.
///
/// **Drei Werte und vier Ausgaenge**, ueberschneidungsfrei und vollstaendig;
/// beide Uebersetzungen halten den Bau an, wenn ein fuenfter dazukommt.
///
/// Kein `PartialEq` und kein `Clone`: [`Textstand::Unlesbar`] haelt einen
/// offenen Deskriptor, und der ist weder vergleichbar noch beliebig
/// vervielfaeltigbar.
#[derive(Debug)]
pub enum Textstand {
    /// Gueltiges UTF-8 unter der Grenze, schon in der gehaltenen Form.
    Text(String),
    /// Eine gewoehnliche Datei, die KRK nicht als Text annimmt.
    ///
    /// **Der Deskriptor steht am Anfang**, und das ist die Zusage, von der der
    /// Aufrufer lebt: er kopiert den Inhalt weiter, ohne den Pfad ein zweites
    /// Mal aufzuloesen. Wer hier einen Wert erzeugt, stellt ihn zurueck; siehe
    /// [`lesen`].
    Unlesbar {
        /// Die offene Datei, zurueckgespult.
        datei: File,
        /// Woran es lag.
        grund: Unlesbarkeit,
    },
    /// Nichts, was ein Texteditor oeffnen koennte: ein Ordner, ein fehlender
    /// Pfad, ein fehlendes Leserecht, alles, was keine gewoehnliche Datei ist.
    KeinGueltigesZiel {
        /// Woran es lag, in einem Satzteil: der Systemfehler oder die Art.
        grund: String,
        /// Ob der Pfad schlicht nichts benennt.
        ///
        /// **Ein Feld und kein fuenfter Ausgang**, und der Unterschied ist
        /// tragend. Fuer den Editor ist eine fehlende Datei dasselbe wie ein
        /// Ordner: beide werden abgewiesen, mit demselben Wert und demselben
        /// Satz. Der Notizzettel trennt sie, weil eine fehlende Zetteldatei der
        /// erste Start ist und keine Meldung wert — dieselbe Regel, die
        /// `ablage::Zugang::laden` fuer eine fehlende TOML-Datei anwendet. Ein
        /// eigener Wert daneben machte aus vier Ausgaengen fuenf und zwaenge
        /// den Editor zu einer Unterscheidung, die er nicht trifft.
        fehlt: bool,
    },
}

/// Die **eine** Groessen- und Typpruefung vor dem Oeffnen (C2, C6).
///
/// Liefert den [`Textstand`] hinter einem Pfad: den fertigen Stand, oder einen
/// benannten Grund, aus dem nichts geoeffnet wird. Warum es nur diese eine
/// Stelle gibt, steht im Modulkopf; wer eine [`Abweisung`] und keinen Befund
/// braucht, nimmt [`oeffnen`].
///
/// # Die Reihenfolge ist bindend
///
/// 1. **Geoeffnet wird zuerst, und zwar ohne zu warten**
///    ([`verzeichnis::sys::ohne_warten_oeffnen`](crate::verzeichnis::sys::ohne_warten_oeffnen)).
///    Das ist der eine Aufruf, der den **Namen** anfasst; alles danach fragt den
///    Deskriptor. Ein `File::open` auf eine benannte Roehre haengt, bis jemand
///    hineinschreibt, und deshalb traegt dieser Aufruf `O_NONBLOCK`, das er vor
///    der Rueckgabe wieder abnimmt. Warum das dort steht und nicht hier, sagt
///    der Modulkopf jener Funktion.
/// 2. **Gefragt wird der Deskriptor** (`fstat` ueber `File::metadata`) **und
///    nicht der Name.** Geoeffnet wird ohne `O_NOFOLLOW`, eine Verknuepfung wird
///    also nach dem behandelt, worauf sie zeigt: als Ziel des Oeffnens ist eine
///    Verknuepfung auf eine Textdatei dieselbe Textdatei. In der Dateiliste
///    meldet der Leser sie weiter als Verknuepfung, und die beiden Fragen sind
///    verschieden. Dieselbe Wahl und derselbe Grund wie in `krk-ui`s
///    `kommandos::pfadeingabe::pruefen`.
/// 3. **Alles, was keine gewoehnliche Datei ist, faellt hier heraus**, der
///    Ordner voran. Ein Ordner und eine benannte Roehre lassen sich beide
///    oeffnen; heraus fallen sie an ihrem Typ, und weil Schritt 1 nicht wartet,
///    fallen sie heraus, statt die Anwendung ohne Meldung anzuhalten.
/// 4. **Die Groesse wird vor dem Lesen geprueft**, so wie die Vorschau es fuer
///    ihre beiden Grenzen tut. Eine Protokolldatei von zwei Gigabyte darf nicht
///    erst eingelesen und dann abgewiesen werden; sie steht damit zu keinem
///    Zeitpunkt vollstaendig im Arbeitsspeicher.
/// 5. **Erst danach werden die Bytes gelesen und gewandelt.** Scheitert die
///    Wandlung, wird abgewiesen und nicht mit Ersatzzeichen geoeffnet.
///
/// # Die Grenze wird eingehalten und nicht nur vorhergesagt
///
/// Schritt 4 fragt `fstat(2)`, und zwischen `fstat` und `read` kann eine Datei
/// wachsen. Deshalb liest Schritt 5 hoechstens [`EDITORGRENZE`] `+ 1` Bytes und
/// weist ab, sobald das eine Byte zuviel ankommt. Der Unterschied ist nicht
/// akademisch: ohne diese Schranke waere "die Datei steht nie vollstaendig im
/// Speicher" eine Vorhersage aus einer alten Auskunft, mit ihr ist es eine
/// Eigenschaft der Bauart. Eine wachsende Protokolldatei ist genau der Fall,
/// fuer den ein Nutzer den Editor aufmacht.
///
/// # Geprueft wird der Deskriptor und nicht der Pfad
///
/// Bis zum 260810 stand es umgekehrt: `stat(2)` auf den Pfad, danach ein zweites
/// Oeffnen desselben Pfades. Zwischen beiden Aufrufen lag ein Fenster, in dem der
/// Pfad auf etwas anderes zeigen konnte, und ein Austausch gegen eine benannte
/// Roehre hielt das zweite Oeffnen an: der Arbeitsfaden des Ladevorgangs endete
/// nie, und der Editor oeffnete kommentarlos nichts. Der Defekt dazu ist
/// `260809-1652`.
///
/// **Das Fenster ist zu, und zwar weil es keinen zweiten Aufruf mehr gibt**, der
/// den Namen aufloest. Zwei Aussagen gehoeren dazu, und die zweite ist die, die
/// man leicht zu viel liest:
///
/// - **Typpruefung, Groessenpruefung und Lesen betreffen ein und dasselbe Ding.**
///   Was der Editor annimmt oder abweist, ist der Gegenstand hinter dem
///   Deskriptor, und der wechselt seine Art nicht mehr, gleichgueltig was mit dem
///   Namen geschieht. **Wachsen** faengt weiterhin die Schranke aus dem Absatz
///   darueber, denn sie zaehlt gelesene Bytes und keine Auskunft.
/// - **Welche** Datei das ist, sagt diese Reihenfolge nicht zu. Wird der Pfad
///   ausgetauscht, **bevor** Schritt 1 laeuft, oeffnet der Editor das neue Ding
///   und beurteilt es richtig. Das ist keine Luecke, sondern die Grenze jeder
///   Schnittstelle, die einen Namen annimmt: wer eine bestimmte Datei meint, muss
///   einen Deskriptor uebergeben und keinen Pfad.
///
/// # Ein unlesbarer Befund traegt seinen Deskriptor zurueckgespult
///
/// [`Textstand::Unlesbar`] reicht die offene Datei weiter, damit der Aufrufer
/// den Inhalt beiseitelegen kann, ohne den Pfad ein zweites Mal aufzuloesen.
/// **Zurueckgespult wird vor jeder solchen Rueckkehr und nicht nur dort, wo es
/// noetig scheint**: im Fall "zwischen `fstat` und `read` gewachsen" sind
/// bereits [`EDITORGRENZE`] `+ 1` Bytes gelesen, und der Aufrufer kopierte
/// sonst einen Rumpf.
///
/// Scheitert das Zurueckspulen, kommt kein `Unlesbar` zurueck, sondern
/// [`Textstand::KeinGueltigesZiel`] mit der Meldung des Systems. Ein Deskriptor
/// an unbekannter Stelle ist schlimmer als eine Meldung: er ergaebe eine
/// abgeschnittene Sicherung, die aussieht wie eine vollstaendige. Auf einer
/// gewoehnlichen Datei — und eine andere kommt bis hierher nicht — ist der Fall
/// nicht zu erreichen.
pub fn lesen(pfad: &Path) -> Textstand {
    let kein_ziel = |grund: String, fehlt: bool| Textstand::KeinGueltigesZiel { grund, fehlt };

    let mut datei = match crate::verzeichnis::sys::ohne_warten_oeffnen(pfad) {
        Ok(datei) => datei,
        Err(fehler) => {
            let fehlt = fehler.kind() == io::ErrorKind::NotFound;
            return kein_ziel(fehler.to_string(), fehlt);
        }
    };
    let angaben = match datei.metadata() {
        Ok(angaben) => angaben,
        Err(fehler) => return kein_ziel(fehler.to_string(), false),
    };

    if !angaben.is_file() {
        return kein_ziel(
            String::from(if angaben.is_dir() {
                "ein Ordner hat keinen Text, den der Editor zeigen könnte"
            } else {
                "das ist keine gewöhnliche Datei"
            }),
            false,
        );
    }

    if angaben.len() > EDITORGRENZE {
        return unlesbar(datei, Unlesbarkeit::ZuGross(angaben.len()));
    }

    let mut bytes = Vec::with_capacity(angaben.len() as usize);
    if let Err(fehler) = datei
        .by_ref()
        .take(EDITORGRENZE + 1)
        .read_to_end(&mut bytes)
    {
        return kein_ziel(fehler.to_string(), false);
    }
    if bytes.len() as u64 > EDITORGRENZE {
        // Die Datei ist zwischen `fstat` und `read` gewachsen. Gemeldet wird die
        // Groesse von jetzt und nicht die von vorhin, denn die alte war nie
        // wahr; laesst sie sich nicht mehr erheben, steht die untere Schranke
        // da, die wir sicher wissen.
        let groesse = datei
            .metadata()
            .map(|angaben| angaben.len())
            .unwrap_or(bytes.len() as u64);
        return unlesbar(datei, Unlesbarkeit::ZuGross(groesse));
    }

    match einlesen(bytes) {
        Some(text) => Textstand::Text(text),
        None => unlesbar(datei, Unlesbarkeit::KeinText),
    }
}

/// Der Befund, der einen zurueckgespulten Deskriptor mitgibt.
///
/// Die **eine** Stelle, an der [`Textstand::Unlesbar`] entsteht; ein zweiter
/// Bauplatz daneben koennte das Zurueckspulen vergessen. Warum ueberhaupt
/// zurueckgespult wird und was ein Scheitern bedeutet, steht an [`lesen`].
fn unlesbar(mut datei: File, grund: Unlesbarkeit) -> Textstand {
    match datei.rewind() {
        Ok(()) => Textstand::Unlesbar { datei, grund },
        Err(fehler) => Textstand::KeinGueltigesZiel {
            grund: fehler.to_string(),
            fehlt: false,
        },
    }
}

/// Die Uebersetzung des Befundes in die Sprache des Editors (C2, C6).
///
/// Signatur und Rueckgabewerte sind unveraendert das, was sie vor der Runde 9
/// waren; der Editor sieht von der Zerlegung in [`lesen`] nichts. Die
/// Reihenfolge der Pruefungen, die Schranke gegen eine wachsende Datei und die
/// Begruendung, warum am Deskriptor geprueft wird und nicht am Pfad, stehen
/// jetzt an [`lesen`] und werden hier nicht wiederholt.
///
/// **Der Deskriptor aus [`Textstand::Unlesbar`] wird hier fallengelassen.** Der
/// Editor oeffnet nichts, was er nicht als Text lesen kann, und braucht die
/// Bytes deshalb nicht; wer sie braucht, ist der Notizzettel, und der geht
/// ueber [`lesen`].
pub fn oeffnen(pfad: &Path) -> Result<String, Abweisung> {
    match lesen(pfad) {
        Textstand::Text(stand) => Ok(stand),
        Textstand::Unlesbar {
            grund: Unlesbarkeit::ZuGross(groesse),
            ..
        } => Err(Abweisung::ZuGross {
            pfad: pfad.to_path_buf(),
            groesse,
        }),
        Textstand::Unlesbar {
            grund: Unlesbarkeit::KeinText,
            ..
        } => Err(Abweisung::NichtAlsTextLesbar {
            pfad: pfad.to_path_buf(),
        }),
        // Die fehlende Datei ist fuer den Editor kein eigener Fall: sie hat so
        // wenig Text zu zeigen wie ein Ordner. Der Notizzettel trennt sie,
        // siehe das Feld `fehlt`.
        Textstand::KeinGueltigesZiel { grund, .. } => Err(Abweisung::KeinGueltigesZiel {
            pfad: pfad.to_path_buf(),
            grund,
        }),
    }
}

/// Warum das begrenzte Lesen keine Bytes geliefert hat.
///
/// **Vier Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig.**
/// Ueberschneidungsfrei sind sie durch die Reihenfolge der Pruefungen in
/// [`bis_zur_grenze_lesen`]: das Oeffnen scheitert vor jeder Frage an den
/// Deskriptor, die Typfrage steht vor der Groessenfrage, und was danach
/// schiefgeht, ist ein Lesefehler. Ein fuenfter Grund haelt jede Uebersetzung
/// an, die diese Werte auseinanderlegt.
///
/// **[`Deskriptormangel`](Self::Deskriptormangel) wird hier getrennt und nicht
/// beim Aufrufer**, denn allein diese Stelle haelt den `io::Error` in der Hand;
/// die Regel dafuer ist die eine vorhandene
/// [`verzeichnis::sys::ist_deskriptormangel`](crate::verzeichnis::sys::ist_deskriptormangel).
/// Der Unterschied ist tragend und nicht bloss genauer: `EMFILE` und `ENFILE`
/// sagen etwas ueber den Prozess und nichts ueber die Datei. Wer sie mit den
/// uebrigen Fehlern zusammenzoege, entschiede negativ, wo nichts entschieden
/// ist — derselbe Fehlgriff, den der Durchlauf ueber den Unterbaum seit der
/// Runde 10 vermeidet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lesehindernis {
    /// Ueber der uebergebenen Grenze. Gelesen wurde die Datei damit nicht, und
    /// der Aufrufer weiss ueber ihren Inhalt nichts.
    ZuGross,
    /// Keine gewoehnliche Datei: ein Ordner, eine benannte Roehre, ein Socket,
    /// ein Zeichen- oder Blockgeraet.
    KeineDatei,
    /// Der Vorrat an Deskriptoren ist erschoepft (`EMFILE`, `ENFILE`). Eine
    /// Lage des Prozesses und kein Befund ueber die Datei.
    Deskriptormangel,
    /// Jeder andere Fehler des Systems: der fehlende Pfad, das fehlende
    /// Leserecht, ein Fehler beim Lesen selbst.
    Fehler,
}

/// Liest hoechstens `grenze` Bytes hinter einem Pfad, **ohne bei einer
/// benannten Roehre zu warten** (C1, C6).
///
/// Die zweite der drei Fragen aus dem Modulkopf: [`lesen`] beantwortet "ist das
/// eine Textdatei fuer den Editor", diese Huelle beantwortet "gib mir die Bytes,
/// aber hoechstens so viele", und [`anlesen`] beantwortet "gib mir die ersten N
/// Bytes". Alle drei gehen durch dieselbe eine Tuer,
/// [`verzeichnis::sys::ohne_warten_oeffnen`](crate::verzeichnis::sys::ohne_warten_oeffnen),
/// und eine zweite Tuer entsteht daneben nicht.
///
/// # Die Grenze reist als Argument und wohnt nicht hier
///
/// [`EDITORGRENZE`] gilt fuer [`lesen`] und fuer sonst nichts. Wer diese Huelle
/// ruft, bringt seine eigene Zahl mit: die Vorschau ihre `TEXTGRENZE` und ihre
/// `BILDGRENZE`, der Inhaltsfilter der Dateiliste die Zahl, die er von der
/// Vorschau erbt. Alle drei wohnen in `krk-ui`, und `krk-core` kennt `krk-ui`
/// nicht.
///
/// # Warum das nicht [`lesen`] ist
///
/// [`lesen`] gibt den offenen, zurueckgespulten Deskriptor zurueck
/// ([`Textstand::Unlesbar`]), damit der Notizzettel den Inhalt beiseitelegen
/// kann, ohne den Pfad ein zweites Mal aufzuloesen. **Diese Huelle gibt ihn
/// nicht zurueck**, und ihre Aufrufer brauchen ihn auch nicht: sie haben die
/// Bytes oder einen Grund. Die zweite Fassung ist deshalb kein Versehen und
/// keine Doppelung derselben Frage, sondern die andere Frage; ein Umbau von
/// [`lesen`] auf diese Form kostete die Zusage des Notizzettels und der
/// Sicherungsform und braechte den Aufrufern dieser Huelle nichts.
///
/// # Die Reihenfolge ist dieselbe wie bei [`lesen`]
///
/// Oeffnen ohne zu warten, `fstat(2)` am Deskriptor, Typ, dann Groesse, dann
/// erst lesen. Sie steht ausfuehrlich an [`lesen`] und wird hier nicht ein
/// zweites Mal begruendet; der eine Aufruf, der den **Namen** anfasst, ist das
/// Oeffnen, und alles danach fragt den Deskriptor.
///
/// # Die Grenze wird eingehalten und nicht nur vorhergesagt
///
/// Zwischen `fstat` und `read` kann eine Datei wachsen, und `/dev/zero` liefert
/// ohne Ende, ohne je eine Groesse zu melden. Gelesen werden deshalb hoechstens
/// `grenze + 1` Bytes, und das eine Byte zuviel entscheidet: kommt es an, ist
/// die Datei ueber der Grenze und die Antwort
/// [`Lesehindernis::ZuGross`]. Ohne diese Schranke waere "es wird nie mehr als
/// die Grenze gelesen" eine Vorhersage aus einer alten Auskunft, mit ihr ist es
/// eine Eigenschaft der Bauart.
pub fn bis_zur_grenze_lesen(pfad: &Path, grenze: u64) -> Result<Vec<u8>, Lesehindernis> {
    let datei = match crate::verzeichnis::sys::ohne_warten_oeffnen(pfad) {
        Ok(datei) => datei,
        Err(fehler) => {
            return Err(if crate::verzeichnis::sys::ist_deskriptormangel(&fehler) {
                Lesehindernis::Deskriptormangel
            } else {
                Lesehindernis::Fehler
            });
        }
    };
    let angaben = datei.metadata().map_err(|_| Lesehindernis::Fehler)?;
    if !angaben.is_file() {
        return Err(Lesehindernis::KeineDatei);
    }
    if angaben.len() > grenze {
        return Err(Lesehindernis::ZuGross);
    }

    let mut bytes = Vec::with_capacity(angaben.len() as usize);
    datei
        .take(grenze + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Lesehindernis::Fehler)?;
    if bytes.len() as u64 > grenze {
        // Die Datei ist zwischen `fstat` und `read` gewachsen.
        return Err(Lesehindernis::ZuGross);
    }
    Ok(bytes)
}

/// Liest die ersten `hoechstens` Bytes hinter einem Pfad, **ohne bei einer
/// benannten Roehre zu warten** und **ohne wegen der Groesse abzuweisen**.
///
/// Die dritte der drei Fragen an dieselbe eine Tuer,
/// [`verzeichnis::sys::ohne_warten_oeffnen`](crate::verzeichnis::sys::ohne_warten_oeffnen):
/// "gib mir die ersten N Bytes und sage nichts ueber das, was dahinter steht".
/// Die Reihenfolge — oeffnen ohne zu warten, `fstat(2)` am Deskriptor, Typ,
/// dann lesen — ist dieselbe wie bei [`lesen`] und dort ausfuehrlich begruendet;
/// der eine Aufruf, der den **Namen** anfasst, ist das Oeffnen, und alles danach
/// fragt den Deskriptor. Wer die Pruefung an den Pfad zurueckzoege, haette das
/// Fenster aus dem Defekt `260809-1652` wieder und bliebe an einer benannten
/// Roehre haengen.
///
/// # Der eine Unterschied zu [`bis_zur_grenze_lesen`]
///
/// Jene Huelle weist eine Datei ueber ihrer Grenze ab; diese liest sie an.
/// [`Lesehindernis::ZuGross`] kann deshalb aus [`anlesen`] nicht kommen, und
/// eine Groessenpruefung vor dem Lesen gibt es hier nicht: gelesen wird ueber
/// `take`, also stehen zu keinem Zeitpunkt mehr als `hoechstens` Bytes im
/// Arbeitsspeicher, gleich wie gross die Datei ist. Das ist dieselbe Zusage wie
/// drueben, nur nicht ueber eine Auskunft von `fstat` hergestellt, sondern ueber
/// die Zahl gelesener Bytes; auch eine wachsende Datei und `/dev/zero` fallen
/// darunter.
///
/// # Warum es die dritte Fassung ueberhaupt gibt
///
/// Die Profil-Zusammenfassung der Vorschau bildet ihre Werte aus den gelesenen
/// Bytes: die Ueberschrift einer Datei, ein Feld aus ihrem Kopf. Eine Abweisung
/// nuetzt ihr nichts, denn was sie sucht, steht am Anfang, und die Groesse der
/// ganzen Datei sagt darueber nichts. Fuer eine Datei weit ueber der Grenze,
/// deren gesuchter Wert in ihren ersten hundert Bytes steht, liefert
/// [`bis_zur_grenze_lesen`] [`Lesehindernis::ZuGross`] und diese Huelle die
/// Ueberschrift. Die Grenze reist wie bei der zweiten Fassung als Argument und
/// wohnt nicht hier.
///
/// **Der gemessene Fall, einmal und mit Datum:** am 260824 war der groesste
/// Circle-Datensatz der Werkbank dieses Projekts 119.614 Bytes gross und trug
/// seine Zeile `## Directive` bei Byte 222. Die Zahl steht hier als Beleg von
/// damals und nicht als Zusage: der Datensatz liegt ausserhalb des Quellbaums,
/// keine Probe liest ihn, und ein Archivlauf verschiebt ihn. Dieselbe Form
/// tragen die Kostenangaben in der Wurzel-`Cargo.toml`.
pub fn anlesen(pfad: &Path, hoechstens: u64) -> Result<Vec<u8>, Lesehindernis> {
    let datei = match crate::verzeichnis::sys::ohne_warten_oeffnen(pfad) {
        Ok(datei) => datei,
        Err(fehler) => {
            return Err(if crate::verzeichnis::sys::ist_deskriptormangel(&fehler) {
                Lesehindernis::Deskriptormangel
            } else {
                Lesehindernis::Fehler
            });
        }
    };
    let angaben = datei.metadata().map_err(|_| Lesehindernis::Fehler)?;
    if !angaben.is_file() {
        return Err(Lesehindernis::KeineDatei);
    }

    // Vorgemerkt wird die kleinere der beiden Zahlen: die Datei kann groesser
    // sein als der Deckel, und dann waere ihre Groesse als Vorrat genau das,
    // was diese Huelle nicht in den Speicher holen soll.
    let mut bytes = Vec::with_capacity(angaben.len().min(hoechstens) as usize);
    datei
        .take(hoechstens)
        .read_to_end(&mut bytes)
        .map_err(|_| Lesehindernis::Fehler)?;
    Ok(bytes)
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
/// # Zwei Formen, eine Regel
///
/// **Hier stehen die Regeln, und [`in_gehaltene_form`] ist die Fassung
/// darueber, die einen Text uebernimmt.** Es sind zwei Namen und nicht zwei
/// Wandlungen: die zweite ruft die erste und gibt in ihrem kurzen Weg die
/// uebernommene Zeichenkette zurueck, statt sie noch einmal anzulegen. Wer eine
/// Regel aendert, aendert sie an dieser Stelle und nirgends sonst.
///
/// Ein Text, der die Form schon hat, kommt hier **geliehen** zurueck und kostet
/// keine Kopie. Genau das braucht [`versatz_nach_der_wandlung`], das den Rest
/// hinter einer Stelle wandelt und ihn im Regelfall nur liest; bis zum 260810
/// nahm es dafuer eine uebernommene Zeichenkette und legte den Rest eines
/// 16-MB-Textes auch dann an, wenn die Wandlung ihn unveraendert zurueckgab.
/// Der Defekt dazu ist `260810-0424`.
pub fn gehaltene_form(text: &str) -> Cow<'_, str> {
    if ist_in_gehaltener_form(text) {
        return Cow::Borrowed(text);
    }

    let ohne_marke = text.strip_prefix(BYTEFOLGENMARKE).unwrap_or(text);
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
    Cow::Owned(gewandelt)
}

/// Bringt einen uebernommenen Text in die gehaltene Form.
///
/// Die Regeln stehen in [`gehaltene_form`]; diese Fassung ist der Eingang fuer
/// die Aufrufer, die den Text ohnehin besitzen und ihn hergeben.
///
/// Ein Text, der die Form schon hat, kommt unveraendert und ohne eine einzige
/// Kopie zurueck. Die Fallunterscheidung unten ist genau dafuer da: eine
/// geliehene Antwort heisst "nichts zu wandeln", und dann geht die uebernommene
/// Zeichenkette zurueck, statt aus der Leihe abgeschrieben zu werden.
pub fn in_gehaltene_form(text: String) -> String {
    match gehaltene_form(&text) {
        Cow::Borrowed(_) => text,
        Cow::Owned(gewandelt) => gewandelt,
    }
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
/// dagegen bei jeder kuenftigen Regel von [`gehaltene_form`] nachgezogen
/// werden, und die erste vergessene Nachziehung faende keine Pruefung.
///
/// # Gewandelt wird geliehen, und im Regelfall kostet das keine Kopie
///
/// Der Rest hinter `versatz` geht durch [`gehaltene_form`] und damit **ohne**
/// eine eigene Zeichenkette. Gebraucht wird von ihm allein die **Laenge** des
/// Ergebnisses, und im Regelfall dieser Rechnung — ein eingefuegtes `\r\n` vorn,
/// die Schreibmarke dahinter, der ganze Rest schon in gehaltener Form — ist das
/// die Laenge des Restes selbst. Bis zum 260810 stand hier `rest.to_owned()`,
/// also eine Kopie bis zur Groesse der ganzen Datei fuer eine Zahl, die
/// unveraendert daneben lag. Der Defekt dazu ist `260810-0424`, die Zaehlung
/// steht in `tests/textkopien.rs`.
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
    nachher.len().saturating_sub(gehaltene_form(rest).len())
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
    let form = sicherungsform(stand);
    crate::ablage::atomar::schreiben(ziel, &mut form.as_bytes())
}
