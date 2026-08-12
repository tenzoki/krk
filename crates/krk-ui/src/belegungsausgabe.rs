//! Die geltende Tastenbelegung als Markdown-Datei im Downloads-Ordner
//! (Runde 3).
//!
//! **Dieses Modul spricht keine AppKit-Schnittstelle an**, und das ist der
//! Grund, aus dem es neben `appkit` liegt und nicht darin: alles, was die Datei
//! ausmacht — ihr Inhalt, die Reihenfolge, die drei Spalten, das Ueberschreiben
//! und die beiden Fehlerfaelle — ist damit ohne Fenster und ohne Hauptfaden
//! pruefbar. Unter `appkit/` bleiben zwei kurze Stuecke: der Menueeintrag in
//! `appkit::menue` und die Methode am Anwendungsdelegierten, die ihn
//! beantwortet.
//!
//! ```text
//!  Menueeintrag "Tastenbelegung als Markdown sichern" (ohne Kuerzel)
//!        │  Antwortkette, kein Ziel gesetzt
//!        ▼
//!  Anwendungsdelegierter::tastenbelegungSichern:
//!        │  leiht ivars().belegung — die Belegung des Betriebs
//!        ▼
//!  ausgeben ──> markdown ──> atomar::schreiben ──> ~/Downloads/KRK-Tastenbelegung.md
//!        │
//!        └────> Ausgang::meldung ──> Statuszeile, oberster Rang
//! ```
//!
//! # Die Belegung kommt aus dem Delegierten und nicht von der Platte
//!
//! [`ausgeben`] nimmt die Belegung als Argument. Der eine Aufrufer reicht ihm
//! `ivars().belegung`, den Wert, der im Betrieb gilt. **Ein Aufruf von
//! `belegung::fuer_den_betrieb()` in dieser Datei waere ein Defekt**: er laese
//! `keymap.toml` erneut von der Platte, waere ein zweiter Ladeweg neben dem
//! einen, und er antwortete in einem Fall nachweislich falsch — scheitert das
//! Sichern beim Verlassen der Belegungsansicht, gilt die neue Belegung im
//! Programm, waehrend die Datei die alte traegt, und KRK sagt es dem Nutzer in
//! dieser Lage ausdruecklich.
//!
//! Daraus faellt zugleich der Fall "offene Belegungsansicht" ohne einen
//! einzigen Zweig an: die Ansicht arbeitet auf einer Kopie, der Wert in den
//! Ivars bleibt bis zum Verlassen unberuehrt, und die Ausgabe schreibt deshalb
//! den **gesicherten** Stand, ohne zu fragen, ob ein Blatt steht. Dass die
//! Datei dann sichtbar vom Schirm abweichen kann, ist der Preis, den der Nutzer
//! am 260811-0110 angenommen hat; eine zusaetzliche Meldung darueber verlangt
//! C4 ausdruecklich nicht.
//!
//! # Die vier Begruendungslagen der dritten Spalte
//!
//! **Gezaehlt wird ueber alle 79 Funktionen, und die Ziffer einer Lage heisst
//! ueberall dasselbe**: im Modulkopf, an den Zweigen von [`wirkung`] und in der
//! Probe `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`. Die
//! erste Lage traegt die 73 Funktionen mit [`Kommando`], die zweite bis vierte
//! verteilen die sechs zugestellten Textbefehle unter sich.
//!
//! Die Spalte "Wirkt in" hat damit **vier verschiedene Quellen**, und
//! [`wirkung`] haelt sie auseinander, statt sie zu mitteln:
//!
//! | Lage | Funktionen | Zelle | woher die Aussage kommt |
//! |---|---|---|---|
//! | 1 | die 73 mit [`Kommando`] | [`Wirkungsbereich::beschriftung`] | aus der Belegung **entscheidbar**, ohne Naeherung |
//! | 2 | `text_ausschneiden`, `text_kopieren`, `text_einfuegen` | "Textfelder und Editor" | in S1 am Laufzeitsystem **gemessen**, zuzueglich eines `inference:`-Schrittes ueber den Feldeditor (siehe unten) |
//! | 3 | `text_alles_auswaehlen` | leer | S1 hat die Ableitung **gebrochen** |
//! | 4 | `text_rueckgaengig`, `text_wiederholen` | "Editor" | **Nutzerentscheid** vom 260811-0935, am Code belegt |
//!
//! **Die zweite Lage ist zur Haelfte gemessen und zur Haelfte erschlossen.**
//! Gemessen hat S1 eine Aussage ueber Klassen: `cut:`, `copy:` und `paste:`
//! haengen an `NSText`, und `NSTextField` beantwortet keinen von ihnen. Der
//! Schritt von dort zu "Textfelder" ist ein zweiter und **nicht** gemessen —
//! `inference:` der Feldeditor eines `NSTextField` ist eine `NSTextView` und
//! bringt `NSText` mit. Das ist eine zugesagte Eigenschaft von AppKit, aber
//! `AnyClass::responds_to` hat sie nicht geprueft, denn es legt keine Instanz
//! an und fragt nichts ueber den Ersthelfer. Die Haelfte "Editor" derselben
//! Zelle braucht diesen zweiten Schritt nicht: gemessen ist, dass
//! `NSTextView` die drei beantwortet, und die Textflaeche des Editors **ist**
//! eine `NSTextView` (`super::appkit::editor`).
//!
//! Die Einzelheiten stehen an den Zweigen von [`wirkung`]. Was sie gemeinsam
//! haben: keine Zelle behauptet mehr, als ihre Quelle hergibt. Eine leere Zelle
//! ist eine ehrliche Auskunft, eine falsche ist es nicht.
//!
//! **Daneben steht eine Zelle, die keine Begruendungslage ist**, sondern das
//! Eingestaendnis, dass keine der obigen greift: [`NICHT_EINGEORDNET`]. Sie
//! trifft eine Kennung, die ein Kommando traegt und der eine von Hand
//! geschriebene `keymap.toml` trotzdem einen Zusteller gibt; der Weg dorthin
//! steht am Auffangzweig von [`wirkung`] ausgeschrieben. Sie ist ausdruecklich
//! **nicht** leer, weil die leere Zelle in dieser Datei schon vergeben ist:
//! "hier ist nichts entschieden" und "hier hat niemand nachgesehen" duerfen
//! nicht in derselben Zelle zusammenfallen.
//!
//! # Warum die Datei unteilbar geschrieben wird
//!
//! Ueber [`atomar::schreiben`], denselben Weg, den `krk_core::text::datei`
//! beim Sichern des Editors und die vier Ablagedateien gehen. Ein zweiter
//! Schreibweg im Programm entsteht damit nicht, und C2 bekommt, was es
//! verlangt: eine halb geschriebene Datei bleibt in keinem Fall zurueck. Der
//! Preis ist eine kurzlebige Nachbardatei `KRK-Tastenbelegung.md.neu` im
//! Downloads-Ordner; sie traegt einen festen Namen ohne Laufnummer, sodass ein
//! Absturz hoechstens eine einzige liegenlaesst, und der naechste Aufruf
//! ueberschreibt sie.
//!
//! # Warum der Fehlerfall am Ergebnis haengt und nicht an einer Vorabpruefung
//!
//! Eine Pruefung des Zielordners **vor** dem Schreiben waere die falsche
//! Bauform: zwischen Pruefung und Schreiben liegt ein Fenster, in dem sich die
//! Antwort aendert. Dieses Projekt hat die Lehre schon gezogen — die Typfrage
//! vor dem Oeffnen einer Textdatei steht am Deskriptor und nicht am Pfad. Hier
//! steht sie am Rueckgabewert: `fs::File::create` liefert fuer einen fehlenden
//! Ordner `NotFound` und fuer einen verwehrten Zugriff `PermissionDenied`, und
//! [`ausgeben`] liest die beiden ab, statt sie vorherzusagen. Ein vom
//! Mechanismus fuer Transparenz, Zustimmung und Kontrolle abgelehnter Zugriff
//! kommt als `EPERM` und damit ebenfalls als `PermissionDenied` an; die
//! Gegenprobe am gebauten Buendel steht in S4 und ist Nutzerarbeit.
//!
//! **KRK legt den Zielordner nicht an.** Ein fehlender Downloads-Ordner ist
//! eine Auskunft ueber das Geraet und keine Aufgabe fuer einen Dateimanager,
//! der dorthin nur schreiben soll.

use std::io;
use std::path::{Path, PathBuf};

use krk_core::ablage::{atomar, pfade};
use krk_core::tasten::{Belegung, Funktion};

use crate::belegungsmodell::{nach_bereichen, tastenliste};

/// Der Name der Ausgabedatei. Fest, nicht einstellbar.
pub const DATEINAME: &str = "KRK-Tastenbelegung.md";

/// Der Ordner unter dem Benutzerverzeichnis, in den geschrieben wird. Fest,
/// nicht einstellbar.
pub const ZIELORDNER: &str = "Downloads";

/// Die Ueberschrift, mit der die Datei beginnt.
///
/// **Genau eine, und kein Vorspann.** Kein Erzeugungszeitpunkt und keine
/// Versionsangabe: eine Datei ohne Zeitstempel ist zwischen zwei Laeufen
/// byteweise vergleichbar, und wer sie versioniert, bekommt einen leeren Diff,
/// wenn sich an der Belegung nichts geaendert hat (Nutzerentscheid vom
/// 260811-0115).
const UEBERSCHRIFT: &str = "# Tastenbelegung von KRK";

/// Die Kopfzeile jeder Tabelle.
const TABELLENKOPF: &str = "| Funktion | Kombinationen | Wirkt in |";

/// Die Trennzeile unter der Kopfzeile.
const TABELLENTRENNER: &str = "|---|---|---|";

/// Die Tastenbelegung als Markdown-Text.
///
/// Eine Ueberschrift, danach je besetztem [`Funktionsbereich`] ein Abschnitt
/// mit einer Pipe-Tabelle aus drei Spalten. Der Text endet mit `\n`;
/// geschrieben wird er als UTF-8 ohne Bytefolgenmarke, also in derselben Form,
/// die der Editor beim Sichern schreibt.
///
/// **Keine Zahl ist verdrahtet**, weder die der Funktionen noch die der
/// Bereiche: gezaehlt wird, was die Belegung fuehrt. Eine spaetere Runde, die
/// die Belegung erweitert, aendert diese Funktion nicht.
///
/// Aufgenommen wird eine Funktion nur, wenn sie mindestens eine Kombination
/// traegt; ein Abschnitt, dessen Funktionen saemtlich unbelegt sind, entfaellt
/// ganz statt mit einer leeren Tabelle zu erscheinen. Der Umfang ist der
/// Nutzerentscheid vom 260811-0110, gegen die Empfehlung des Datensatzes, und
/// sein Preis steht dort: eine versehentlich unbelegte Funktion verschwindet
/// aus der Datei, statt darin als unbelegt zu erscheinen.
///
/// Gliederung und Reihenfolge kommen aus
/// [`nach_bereichen`](crate::belegungsmodell::nach_bereichen), die
/// Schreibweise der Kombinationen aus
/// [`tastenliste`](crate::belegungsmodell::tastenliste) und damit aus
/// `anzeige`. Beide sind mit der Bildschirmansicht geteilt und nicht
/// abgeschrieben; eine zweite Aufbereitung schliesst die Directive aus.
///
/// [`Funktionsbereich`]: crate::belegungsmodell::Funktionsbereich
pub fn markdown(belegung: &Belegung) -> String {
    let mut text = String::from(UEBERSCHRIFT);
    text.push('\n');

    for (bereich, stellen) in nach_bereichen(belegung) {
        let belegte: Vec<&Funktion> = stellen
            .iter()
            .filter_map(|stelle| belegung.funktionen().get(*stelle))
            .filter(|funktion| !funktion.tasten().is_empty())
            .collect();
        if belegte.is_empty() {
            continue;
        }

        text.push('\n');
        text.push_str("## ");
        text.push_str(bereich.name());
        text.push_str("\n\n");
        text.push_str(TABELLENKOPF);
        text.push('\n');
        text.push_str(TABELLENTRENNER);
        text.push('\n');
        for funktion in belegte {
            text.push_str(&zeile(funktion));
            text.push('\n');
        }
    }
    text
}

/// Eine Tabellenzeile: Name, Kombinationen, Wirkungsangabe.
fn zeile(funktion: &Funktion) -> String {
    format!(
        "| {} | {} | {} |",
        maskiert(funktion.name()),
        maskiert(&tastenliste(funktion)),
        maskiert(wirkung(funktion))
    )
}

/// Der Text einer Zelle, mit maskiertem senkrechten Strich.
///
/// Der Name einer Funktion kommt aus der Belegungsdatei und damit
/// moeglicherweise aus der `keymap.toml` des Nutzers. Ein Name mit einem
/// senkrechten Strich zerbraeche die Tabelle; die Maskierung ist eine Zeile und
/// eine Probe.
fn maskiert(text: &str) -> String {
    text.replace('|', "\\|")
}

/// Die dritte Zelle einer Funktion, ueber die [`wirkung`] nichts weiss.
///
/// **Der Zweck dieses Wertes ist, nicht die leere Zeichenkette zu sein.** Die
/// leere Zelle ist in dieser Datei bereits vergeben: `text_alles_auswaehlen`
/// bleibt leer, weil die Messung aus S1 die Ableitung gebrochen hat, und das
/// ist ein Ergebnis. Faellt der Auffangzweig von [`wirkung`] auf denselben
/// Ausgang, sind zwei verschiedene Aussagen in der Ausgabe nicht mehr
/// unterscheidbar, und der Leser der Datei hat keinen Weg, das zu merken.
///
/// **Der Wortlaut ist mit Bedacht keine Ortsangabe.** Die uebrigen Zellen
/// dieser Spalte nennen Orte — "Editor", "Textfelder und Editor", die sieben
/// Beschriftungen von [`Wirkungsbereich`](krk_core::tasten::Wirkungsbereich).
/// Dieser hier nennt stattdessen KRK selbst und steht in Klammern, damit ihn
/// niemand fuer eine Aussage darueber haelt, **wo** der Befehl wirkt: er sagt
/// allein, dass KRK die Funktion nicht einordnen konnte. "Eingeordnet" ist
/// dabei dasselbe Wort, das `belegungsmodell::bereich` fuer diese Rechnung
/// fuehrt.
const NICHT_EINGEORDNET: &str = "(von KRK nicht eingeordnet)";

/// Die dritte Spalte einer Zeile: wo der Befehl wirkt.
///
/// **Die Fallunterscheidung ist ueberschneidungsfrei und vollstaendig**: eine
/// Funktion traegt entweder ein [`Kommando`](krk_core::tasten::Kommando) oder
/// nicht, und beide Zweige liefern eine Antwort. Gefragt wird ueber
/// [`Funktion::kommando`] und nicht ueber `Kommando::aus_kennung`, weil das die
/// Zustellerregel mitfuehrt: was das Hauptmenue zustellt, hat nie ein Kommando,
/// und die Zusage haengt damit nicht daran, dass `Kommando::KENNUNGEN` die
/// sechs Textbefehle zufaellig nicht nennt.
///
/// **Der rechte Zweig entscheidet je Befehl und nicht fuer die Gruppe.** Das
/// ist der Kern dieser Funktion: die sechs zugestellten Textbefehle verteilen
/// sich auf die **zweite bis vierte** der vier Begruendungslagen, und ein
/// Alles-oder-nichts ueber die sechs waere fuer zwei dieser drei Lagen falsch.
/// Der Modulkopf stellt alle vier als Tabelle daneben und legt dort die
/// Zaehlung fest, der die Zweige hier folgen.
fn wirkung(funktion: &Funktion) -> &'static str {
    // Erste Lage: aus der Belegung entscheidbar, ohne Naeherung. 73 der 79
    // Funktionen tragen ein Kommando, `Kommando::wirkungsbereich` ist eine
    // totale Funktion darueber, und `Wirkungsbereich::beschriftung` ist eine
    // zweite, deren Vollstaendigkeit der Uebersetzer erzwingt. Hier ist nichts
    // gemessen und nichts entschieden — hier wird abgelesen.
    if let Some(kommando) = funktion.kommando() {
        return kommando.wirkungsbereich().beschriftung();
    }

    match funktion.kennung() {
        // Zweite Lage: **gemessen, und der Schritt zu "Textfelder" daraus
        // erschlossen.** Gemessen hat S1 am Objective-C-Laufzeitsystem, welche
        // Klasse diese drei Selektoren beantwortet, und die Antwort ist
        // `NSText` — nicht `NSTextView`, und `NSTextField` beantwortet keinen
        // von ihnen. Das ist eine Aussage ueber Klassen. Fuer den Editor
        // reicht sie aus: seine Textflaeche ist eine `NSTextView`
        // (`super::appkit::editor`).
        //
        // Fuer "Textfelder" kommt ein zweiter Schritt hinzu, und der ist
        // **nicht** gemessen: `inference:` erreicht wird der **Feldeditor**
        // des Textfeldes, der eine `NSTextView` ist und `NSText` mitbringt.
        // Das ist eine zugesagte Eigenschaft von AppKit, aber
        // `AnyClass::responds_to` hat sie nicht geprueft — es legt keine
        // Instanz an und fragt nichts ueber den Ersthelfer. Wer die Kette
        // messen will, braucht eine Instanz und damit den Hauptfaden.
        //
        // Die Tabelle der Messung steht im Modulkopf von
        // `super::appkit::menue`, die Probe daneben unter `mod tests`.
        "text_ausschneiden" | "text_kopieren" | "text_einfuegen" => "Textfelder und Editor",

        // Dritte Lage: **die Messung hat die Ableitung gebrochen, und die
        // Zelle bleibt deshalb leer.** `NSTableView` beantwortet `selectAll:`
        // aus einer eigenen Methode, und die Lesezeichen- und Geraeteleiste
        // ist eine `NSTableView`: mit dem Fokus dort weist der stumme
        // Fokusvorbehalt `alle_markieren` ab, der Tastendruck geht unveraendert
        // an AppKit und erreicht diesen Menueeintrag. "Textfelder und Editor"
        // waere fuer diesen einen der sechs eine falsche Zusicherung.
        //
        // **Die leere Zelle ist ein Ergebnis und kein Versaeumnis.** Der
        // Datensatz
        // `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/issues/260811-0930_*_die-ableitung-textfelder-und-editor-bricht-fuer-alles-auswaehlen-die-leiste-beantwortet-selectall-selbst.md`
        // haelt die Messung fest, damit sie hier nicht spaeter
        // "vervollstaendigt" wird. Was sie ausdruecklich **nicht** entschieden
        // hat: ob der in der Leiste bedienbare Eintrag dort auch etwas
        // bewirkt. Das braucht eine Instanz und damit den Hauptfaden. Wer die
        // Zelle fuellen will, misst das zuerst.
        "text_alles_auswaehlen" => "",

        // Vierte Lage: **ein Nutzerentscheid, am Code belegt und
        // ausdruecklich nicht aus S1 abgeleitet.** S1 konnte hier nichts
        // entscheiden: `undo:` und `redo:` stehen an `NSWindow` und nicht an
        // der Textklasse, `responds_to` liefert `false` fuer einen
        // weitergeleiteten Selektor, und ein `false` an dieser Stelle belegt
        // nicht, dass niemand antwortet. Der Beleg kommt von woanders — die
        // `NSTextView` des Editors bringt ihren Rueckgaengigverwalter mit und
        // benutzt ihn, sobald `setAllowsUndo(true)` gesetzt ist, und genau das
        // geschieht in `super::appkit::editor`. Der Nutzer hat daraufhin am
        // 260811-0935 "Editor" gesetzt.
        "text_rueckgaengig" | "text_wiederholen" => "Editor",

        // **Erreichbar, und deshalb traegt der Zweig eine eigene Auskunft.**
        //
        // Bis zum 260811 stand hier die Begruendung, der Zweig sei
        // unerreichbar, weil [`markdown`] zuvor durch `nach_bereichen` laufe
        // und das laut abbreche, sobald `belegungsmodell::bereich` eine
        // Kennung nicht einordnen koenne. **Die Begruendung traegt nicht**:
        // die beiden Stellen stellen zwei verschiedene Fragen. `bereich`
        // (`belegungsmodell.rs`) fragt ueber `Kommando::aus_kennung` und sieht
        // `gehalten_von` **nicht**; diese Funktion fragt drei Zeilen weiter
        // oben ueber `Funktion::kommando` und sieht es.
        //
        // Der Weg hierher, am 260811-0955 gegen `krk-core` gemessen: eine von
        // Hand geschriebene `keymap.toml` gibt einer Kennung **mit** Kommando
        // einen Zusteller, etwa `kopieren` ein `gehalten_von = "menue"`.
        // `Belegung::vom_nutzer` nimmt sie an — `Belegung::bauen` prueft
        // allein die Kennung gegen den Wortschatz der Auslieferungsbelegung
        // und uebernimmt `gehalten_von` unveraendert, und `konflikte`
        // vergleicht nur innerhalb desselben Zustellers. `bereich("kopieren")`
        // ordnet die Funktion dann ueber ihr Kommando ein, `nach_bereichen`
        // bricht **nicht** ab, und `funktion.kommando()` steht hier auf `None`:
        // `kommando()=None gehalten_von=Some("menue")
        // aus_kennung=Some(Kopieren)`. Die Probe
        // `eine_kennung_mit_kommando_und_zusteller_landet_im_auffangzweig`
        // haelt genau diesen Fall fest.
        //
        // **Was hier nicht steht und nicht stehen darf: ein `panic!`.** Es
        // braechte KRK an einer vom Nutzer von Hand geschriebenen, formal
        // zulaessigen `keymap.toml` zum Absturz. Der `match` laeuft ueber
        // `&str`; ein Auffangzweig ist hier ohnehin unvermeidlich, und die
        // Projektregel "vollstaendig ohne Auffangzweig" greift auf ihn nicht.
        // Es zaehlt allein, was er tut — und das ist: eine Auskunft geben, die
        // von der bewusst leeren Zelle darueber unterscheidbar ist.
        //
        // **Die Ungleichheit der beiden Fallunterscheidungen bleibt bestehen.**
        // Sie zu schliessen — `wirkung` fragte dieselbe Frage wie `bereich` —
        // haenge die Zusage des Doc-Kommentars oben wieder daran, dass
        // `Kommando::KENNUNGEN` die sechs Textbefehle nicht nennt. Der
        // Datensatz
        // `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/issues/260811-0955_*_der-auffangzweig-in-wirkung-ist-erreichbar-bereich-und-wirkung-fragen-nicht-dasselbe.md`
        // legt beide Wege vor; gebaut ist der zweite, und die Ungleichheit
        // bleibt dort erfasst.
        _ => NICHT_EINGEORDNET,
    }
}

/// Was aus einem Aufruf der Ausgabe geworden ist.
///
/// **Die Werte tragen den ungekuerzten Pfad**; gekuerzt wird erst beim Melden.
/// Ein Wert, der einen Pfad haelt, haelt ihn brauchbar, nicht huebsch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ausgang {
    /// Die Datei steht unter diesem Pfad. Neu entstanden oder eine vorhandene
    /// ersetzt — beides derselbe Wert, und beides dieselbe Meldung.
    Geschrieben(PathBuf),
    /// Das System nennt kein Benutzerverzeichnis; es gibt keinen Zielordner.
    KeinBenutzerverzeichnis,
    /// Der Zielordner fehlt. KRK legt ihn nicht an.
    OrdnerFehlt(PathBuf),
    /// Der Zugriff auf den Zielordner ist abgelehnt. Auch der Fall, in dem der
    /// Nutzer die Rueckfrage des Systems nach dem Downloads-Ordner verneint.
    ZugriffAbgelehnt(PathBuf),
    /// Ein anderer Fehler beim Schreiben, mit seinem Wortlaut.
    Fehlgeschlagen(PathBuf, String),
}

impl Ausgang {
    /// Die Meldung fuer die Statuszeile, mit dem Benutzerverzeichnis des
    /// Systems.
    ///
    /// Der Weg des Alltags; die eine Aufrufstelle ist der Anwendungsdelegierte.
    /// [`Ausgang::meldung_mit`] daneben ist dieselbe Rechnung mit einem
    /// uebergebenen Benutzerverzeichnis und damit die pruefbare Form.
    pub fn meldung(&self) -> String {
        self.meldung_mit(pfade::benutzerverzeichnis().as_deref())
    }

    /// Die Meldung, gegen ein uebergebenes Benutzerverzeichnis.
    ///
    /// **Vollstaendige Fallunterscheidung ohne Auffangzweig**: ein sechster
    /// Ausgang braucht hier eine Zeile, bevor er uebersetzt.
    ///
    /// **Jeden Pfad, den diese Funktion schreibt, schickt sie zuvor durch
    /// [`pfade::gekuerzt_fuer_anzeige`]** — nicht nur den der Erfolgsmeldung.
    /// Eine Form je Meldung waere die dritte Form fuer denselben Pfad im selben
    /// Programm. Am Ziel der Runde 3 lautet die Erfolgsmeldung damit
    /// "Tastenbelegung geschrieben: ~/Downloads/KRK-Tastenbelegung.md"
    /// (Nutzerentscheid vom 260811-0900, gegen die Empfehlung des Plans).
    pub fn meldung_mit(&self, benutzerverzeichnis: Option<&Path>) -> String {
        let kurz = |pfad: &PathBuf| pfade::gekuerzt_fuer_anzeige(pfad, benutzerverzeichnis);
        match self {
            Ausgang::Geschrieben(pfad) => {
                format!("Tastenbelegung geschrieben: {}", kurz(pfad))
            }
            Ausgang::KeinBenutzerverzeichnis => {
                "die Tastenbelegung ließ sich nicht schreiben: das System nennt kein \
                 Benutzerverzeichnis"
                    .to_owned()
            }
            Ausgang::OrdnerFehlt(pfad) => format!(
                "die Tastenbelegung ließ sich nicht schreiben: der Ordner zu {} fehlt",
                kurz(pfad)
            ),
            Ausgang::ZugriffAbgelehnt(pfad) => format!(
                "die Tastenbelegung ließ sich nicht schreiben: der Zugriff auf {} ist abgelehnt",
                kurz(pfad)
            ),
            Ausgang::Fehlgeschlagen(pfad, grund) => format!(
                "die Tastenbelegung ließ sich nicht nach {} schreiben: {grund}",
                kurz(pfad)
            ),
        }
    }
}

/// Schreibt die Belegung nach `~/Downloads/KRK-Tastenbelegung.md`.
///
/// Der eine Aufrufer ist der Anwendungsdelegierte, und er reicht die Belegung
/// des Betriebs herein; siehe den Modulkopf, warum sie nicht hier geholt wird.
///
/// Eine vorhandene Datei desselben Namens wird ueberschrieben, ohne Rueckfrage
/// und ohne gesonderten Hinweis, auch wenn sie nicht von KRK stammt. Der
/// Downloads-Ordner gehoert dem Nutzer, und KRK ist dort nicht der einzige
/// Schreiber; der Preis ist benannt und am 260811-0110 angenommen. Der
/// Gegenwert ist der stabile Pfad, den ein Git-Repository wiedersehen will.
pub fn ausgeben(belegung: &Belegung) -> Ausgang {
    let Some(zuhause) = pfade::benutzerverzeichnis() else {
        return Ausgang::KeinBenutzerverzeichnis;
    };
    in_ordner_schreiben(belegung, &zuhause.join(ZIELORDNER))
}

/// Dieselbe Arbeit gegen einen genannten Zielordner.
///
/// Der Zielordner kommt als Argument herein, damit die Proben das Schreiben
/// pruefen koennen, ohne im echten Downloads-Ordner des Nutzers eine Datei
/// anzulegen. Dieselbe Erwaegung, aus der sich `Ablageort` auf einen beliebigen
/// Ordner setzen laesst: keine Testhintertuer, sondern die Bedingung der
/// Pruefbarkeit.
fn in_ordner_schreiben(belegung: &Belegung, zielordner: &Path) -> Ausgang {
    let ziel = zielordner.join(DATEINAME);
    match atomar::schreiben(&ziel, &markdown(belegung)) {
        Ok(()) => Ausgang::Geschrieben(ziel),
        // Am Rueckgabewert unterschieden und nicht an einer Vorabpruefung,
        // siehe den Modulkopf. `io::ErrorKind` ist `#[non_exhaustive]` und
        // laesst keine vollstaendige Fallunterscheidung zu; der Auffangzweig
        // steht hier deshalb an einer fremden Aufzaehlung und nicht an einer
        // dieses Projekts, und er verliert nichts: `Fehlgeschlagen` traegt den
        // Wortlaut des Fehlers mit.
        Err(fehler) => match fehler.kind() {
            io::ErrorKind::NotFound => Ausgang::OrdnerFehlt(ziel),
            io::ErrorKind::PermissionDenied => Ausgang::ZugriffAbgelehnt(ziel),
            _ => Ausgang::Fehlgeschlagen(ziel, fehler.to_string()),
        },
    }
}

#[cfg(test)]
mod tests {
    use krk_core::tasten::{Belegungsdatei, Kommando, Wirkungsbereich};

    use crate::belegungsmodell::Funktionsbereich;
    use crate::pruefordner::Pruefordner;

    use super::*;

    /// Eine Belegung aus einem Stueck `keymap.toml`.
    ///
    /// Jede Funktion, die der Text nicht nennt, tritt unbelegt hinzu — das tut
    /// `Belegung::vom_nutzer` von sich aus, und genau darauf beruhen die Proben
    /// des Umfangs: was hier nicht steht, traegt keine Kombination und gehoert
    /// deshalb nicht in die Datei.
    fn belegung_aus(keymap: &str) -> Belegung {
        let datei: Belegungsdatei =
            toml::from_str(keymap).expect("die Pruefbelegung laesst sich nicht lesen");
        Belegung::vom_nutzer(&datei).expect("die Pruefbelegung ist widerspruechlich")
    }

    /// Die Zeilen der Datei, die eine Funktion tragen: alles, was mit `| `
    /// beginnt und nicht die Kopf- oder die Trennzeile ist.
    fn funktionszeilen(text: &str) -> Vec<&str> {
        text.lines()
            .filter(|zeile| zeile.starts_with("| "))
            .filter(|zeile| **zeile != *TABELLENKOPF)
            .collect()
    }

    /// Der Inhalt der Zellen einer Funktionszeile.
    fn zellen(zeile: &str) -> Vec<&str> {
        let innen = zeile
            .strip_prefix('|')
            .and_then(|rest| rest.strip_suffix('|'))
            .expect("eine Funktionszeile steht zwischen zwei Strichen");
        innen.split(" | ").map(str::trim).collect()
    }

    // -----------------------------------------------------------------------
    // Umfang und Gliederung
    // -----------------------------------------------------------------------

    /// Jede belegte Funktion der Auslieferungsbelegung steht in der Datei, und
    /// keine unbelegte.
    ///
    /// **Die Zusage steht in den beiden ersten Teilen**: die Datei fuehrt so
    /// viele Zeilen, wie es belegte Funktionen gibt, und jede belegte findet
    /// sich darin. Der dritte Teil sagt daneben, welche Funktionen ab Werk
    /// unbelegt sind, und nennt sie beim Namen. Bis zum 260812 war die Antwort
    /// darauf "keine"; seither sind es die drei Spaltenschalter, die nach der
    /// Nutzerantwort vom 260812-0306 ohne Kombination ausgeliefert werden
    /// (`circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/
    /// 260812-0306_*_bekommen-die-spaltenschalter-tastenbefehle.md`). Die
    /// Aufzaehlung steht hier ausgeschrieben statt als Zahl: eine Zahl sagte
    /// nicht, **welche** Funktion aus der Datei faellt, und genau das ist die
    /// Auskunft, die ein Leser dieser Probe braucht.
    #[test]
    fn jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte() {
        let belegung = Belegung::auslieferung();
        let text = markdown(&belegung);

        let erwartet: Vec<&str> = belegung
            .funktionen()
            .iter()
            .filter(|funktion| !funktion.tasten().is_empty())
            .map(|funktion| funktion.name())
            .collect();
        let gefunden: Vec<String> = funktionszeilen(&text)
            .iter()
            .map(|zeile| zellen(zeile)[0].to_owned())
            .collect();

        assert_eq!(
            gefunden.len(),
            erwartet.len(),
            "die Datei fuehrt nicht genau die belegten Funktionen"
        );
        for name in &erwartet {
            assert!(
                gefunden.iter().any(|zeile| zeile == name),
                "die Funktion {name} fehlt in der Datei"
            );
        }

        // Ab Werk sind genau die drei Spaltenschalter unbelegt; jede andere
        // Funktion steht in der Datei.
        let unbelegt: Vec<&str> = belegung
            .funktionen()
            .iter()
            .filter(|funktion| funktion.tasten().is_empty())
            .map(Funktion::kennung)
            .collect();
        assert_eq!(
            unbelegt,
            [
                "spalte_groesse_umschalten",
                "spalte_datum_umschalten",
                "spalte_typ_umschalten",
            ],
            "ab Werk sind andere Funktionen unbelegt als die drei Spaltenschalter"
        );
    }

    /// Eine unbelegte Funktion faellt aus der Datei, ohne leere Zelle.
    #[test]
    fn eine_funktion_ohne_kombination_erscheint_nicht() {
        // `kopieren` traegt eine Kombination, `verschieben` nennt die Datei
        // nicht und tritt damit unbelegt hinzu.
        let belegung = belegung_aus(
            r#"
            [[funktion]]
            id = "kopieren"
            name = "In das andere Fenster kopieren"
            tasten = ["f5"]
            "#,
        );
        let text = markdown(&belegung);

        assert!(text.contains("In das andere Fenster kopieren"));
        assert!(
            !text.contains("verschieben"),
            "eine unbelegte Funktion steht in der Datei"
        );
        assert_eq!(funktionszeilen(&text).len(), 1);
    }

    /// Die Abschnitte stehen in der Reihenfolge von `Funktionsbereich::ALLE`
    /// und tragen den Text aus `Funktionsbereich::name()`.
    #[test]
    fn die_abschnitte_stehen_in_der_reihenfolge_der_funktionsbereiche() {
        let text = markdown(&Belegung::auslieferung());
        let ueberschriften: Vec<&str> = text
            .lines()
            .filter_map(|zeile| zeile.strip_prefix("## "))
            .collect();
        let erwartet: Vec<&str> = Funktionsbereich::ALLE
            .iter()
            .map(|bereich| bereich.name())
            .collect();
        assert_eq!(
            ueberschriften, erwartet,
            "ab Werk ist jeder Bereich besetzt, also stehen alle neun in ihrer Reihenfolge"
        );
    }

    /// Innerhalb eines Abschnitts bleibt die Reihenfolge der Belegungsdatei
    /// erhalten; eine eigene Sortierung entsteht nicht.
    #[test]
    fn innerhalb_eines_abschnitts_bleibt_die_reihenfolge_der_datei() {
        let belegung = Belegung::auslieferung();
        let text = markdown(&belegung);
        let gefunden: Vec<String> = funktionszeilen(&text)
            .iter()
            .map(|zeile| zellen(zeile)[0].to_owned())
            .collect();

        // Die unbelegten fallen aus der Erwartung, wie sie aus der Datei
        // fallen: `nach_bereichen` ordnet **jede** Funktion einem Abschnitt zu,
        // die Ausgabe schreibt nur die mit einer Kombination. Ab Werk sind das
        // seit dem 260812 die drei Spaltenschalter; welche es genau sind, sagt
        // `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`, diese
        // Probe misst allein die Reihenfolge.
        let erwartet: Vec<&str> = nach_bereichen(&belegung)
            .into_iter()
            .flat_map(|(_, stellen)| stellen)
            .map(|stelle| &belegung.funktionen()[stelle])
            .filter(|funktion| !funktion.tasten().is_empty())
            .map(Funktion::name)
            .collect();

        assert_eq!(gefunden, erwartet);
    }

    /// Ein Bereich, dessen Funktionen saemtlich unbelegt sind, erzeugt keinen
    /// Abschnitt mit leerer Tabelle — er entfaellt ganz.
    #[test]
    fn ein_unbelegter_bereich_erzeugt_keinen_abschnitt() {
        let belegung = belegung_aus(
            r#"
            [[funktion]]
            id = "kopieren"
            name = "In das andere Fenster kopieren"
            tasten = ["f5"]
            "#,
        );
        let text = markdown(&belegung);
        let ueberschriften: Vec<&str> = text
            .lines()
            .filter_map(|zeile| zeile.strip_prefix("## "))
            .collect();
        assert_eq!(
            ueberschriften,
            [Funktionsbereich::Dateioperationen.name()],
            "nur der eine besetzte Bereich bekommt einen Abschnitt"
        );
        assert_eq!(
            text.matches(TABELLENKOPF).count(),
            1,
            "es steht genau eine Tabelle da, und keine leere daneben"
        );
    }

    // -----------------------------------------------------------------------
    // Die drei Spalten
    // -----------------------------------------------------------------------

    /// Jede Zeile traegt drei Spalten, und eine Funktion mit zwei
    /// Kombinationen steht in **einer** Zeile mit beiden darin, getrennt durch
    /// Komma und Leerzeichen.
    #[test]
    fn eine_funktion_mit_zwei_kombinationen_steht_in_einer_zeile() {
        let text = markdown(&Belegung::auslieferung());

        for zeile in funktionszeilen(&text) {
            assert_eq!(
                zellen(zeile).len(),
                3,
                "die Zeile {zeile} traegt nicht drei Spalten"
            );
        }

        let treffer: Vec<&str> = funktionszeilen(&text)
            .into_iter()
            .filter(|zeile| zellen(zeile)[0] == "In das andere Fenster kopieren")
            .collect();
        assert_eq!(treffer.len(), 1, "die Funktion steht in genau einer Zeile");
        assert_eq!(
            zellen(treffer[0])[1],
            "F5, Shift+Cmd+K",
            "beide Kombinationen stehen in derselben Zelle, in der Schreibweise von anzeige()"
        );
    }

    /// **Die dritte Spalte, ueber ihre vier Begruendungslagen.**
    ///
    /// Die Zaehlung ist die des Modulkopfs und laeuft ueber alle 79
    /// Funktionen: die erste Lage traegt die 73 mit Kommando, die zweite bis
    /// vierte die sechs zugestellten Textbefehle. Die Probe haelt jede der
    /// vier einzeln fest, weil ein Alles-oder-nichts ueber die sechs fuer zwei
    /// der drei sie betreffenden Lagen falsch waere. Aendert eine der vier
    /// Quellen ihre Antwort, schlaegt genau der betroffene Abschnitt fehl.
    #[test]
    fn die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander() {
        let belegung = Belegung::auslieferung();

        // Erste Lage, aus der Belegung entscheidbar: jede Funktion mit
        // Kommando traegt die Beschriftung ihres Wirkungsbereichs, und keine
        // andere Quelle mischt sich ein.
        let mut mit_kommando = 0;
        for funktion in belegung.funktionen() {
            let Some(kommando) = funktion.kommando() else {
                continue;
            };
            mit_kommando += 1;
            assert_eq!(
                wirkung(funktion),
                kommando.wirkungsbereich().beschriftung(),
                "{} traegt nicht die Beschriftung ihres Wirkungsbereichs",
                funktion.kennung()
            );
        }
        assert_eq!(
            mit_kommando,
            Kommando::KENNUNGEN.len(),
            "jedes Kommando der Aufzaehlung steht in der Auslieferungsbelegung"
        );

        let wirkung_von = |kennung: &str| {
            wirkung(
                belegung
                    .funktion(kennung)
                    .unwrap_or_else(|| panic!("{kennung} steht nicht in der Belegung")),
            )
        };

        // Zweite Lage, in S1 gemessen: die drei Zwischenablage-Befehle haengen
        // an `NSText`, und die Textflaeche des Editors ist eine `NSTextView`.
        // `inference:` Der Weg zu den Textfeldern fuehrt ueber deren
        // Feldeditor, der ebenfalls eine `NSTextView` ist; gemessen ist dieser
        // Schritt nicht, siehe den Zweig von `wirkung`.
        for kennung in ["text_ausschneiden", "text_kopieren", "text_einfuegen"] {
            assert_eq!(
                wirkung_von(kennung),
                "Textfelder und Editor",
                "{kennung} traegt nicht die aus der Messung von S1 hergeleitete Beschriftung"
            );
        }

        // Dritte Lage: S1 hat die Ableitung gebrochen, weil
        // `NSTableView` `selectAll:` selbst beantwortet und die Leiste eine
        // ist. Die Zelle bleibt **leer**, und das ist ein Ergebnis und kein
        // Versaeumnis; der Datensatz steht am Zweig von `wirkung`.
        assert_eq!(
            wirkung_von("text_alles_auswaehlen"),
            "",
            "die Zelle ist absichtlich leer — wer sie fuellt, misst zuerst, \
             ob der Eintrag in der Leiste etwas bewirkt"
        );

        // Vierte Lage: ein Nutzerentscheid vom 260811-0935,
        // am Code ueber `setAllowsUndo(true)` belegt und ausdruecklich nicht
        // aus S1 abgeleitet — `responds_to` kann fuer `undo:` und `redo:`
        // nichts entscheiden.
        for kennung in ["text_rueckgaengig", "text_wiederholen"] {
            assert_eq!(
                wirkung_von(kennung),
                Wirkungsbereich::Editor.beschriftung(),
                "{kennung} traegt nicht die vom Nutzer gesetzte Beschriftung"
            );
        }
    }

    /// Die Zelle von `text_alles_auswaehlen` steht auch in der fertigen Datei
    /// leer da, und die Tabelle bleibt dabei heil.
    #[test]
    fn die_leere_zelle_zerbricht_die_tabelle_nicht() {
        let text = markdown(&Belegung::auslieferung());
        let zeile = funktionszeilen(&text)
            .into_iter()
            .find(|zeile| zellen(zeile)[0] == "Alles auswählen")
            .expect("die Zeile steht in der Datei");
        assert_eq!(zellen(zeile), ["Alles auswählen", "Cmd+A", ""]);
    }

    /// **In der Auslieferungsbelegung** wird jede Kennung ohne Kommando vom
    /// Menue zugestellt und steht in einem der sechs Zweige von [`wirkung`].
    ///
    /// **Der Umfang dieser Probe ist die ausgelieferte Datei und nichts
    /// sonst.** Sie laeuft ueber `Belegung::auslieferung()`; fuer diese eine
    /// Belegung faengt sie eine Funktion, die weder ein Kommando noch einen der
    /// sechs Zweige traegt, bevor sie in der Markdown-Datei eine Zelle ohne
    /// Auskunft erzeugt. Ein neuer `[[funktion]]`-Block in
    /// `resources/default-keymap.toml` schlaegt hier also fehl, solange
    /// `wirkung` ihn nicht kennt.
    ///
    /// **Ueber eine Belegung des Nutzers sagt sie nichts.** Eine von Hand
    /// geschriebene `keymap.toml` kann einer Kennung mit Kommando einen
    /// Zusteller geben und damit den Auffangzweig von [`wirkung`] erreichen;
    /// der Kommentar dort schreibt den Weg aus, und
    /// `eine_kennung_mit_kommando_und_zusteller_landet_im_auffangzweig` misst
    /// ihn. Nach dem Vorbild von `jede_kennung_hat_einen_funktionsbereich`.
    #[test]
    fn jede_kennung_ohne_kommando_wird_vom_menue_zugestellt() {
        for funktion in Belegung::auslieferung().funktionen() {
            if funktion.kommando().is_some() {
                continue;
            }
            assert_eq!(
                funktion.gehalten_von(),
                Some("menue"),
                "{} traegt weder ein Kommando noch einen Zusteller",
                funktion.kennung()
            );
            assert!(
                matches!(
                    funktion.kennung(),
                    "text_ausschneiden"
                        | "text_kopieren"
                        | "text_einfuegen"
                        | "text_alles_auswaehlen"
                        | "text_rueckgaengig"
                        | "text_wiederholen"
                ),
                "{} ist zugestellt, aber wirkung() kennt sie nicht",
                funktion.kennung()
            );
        }
    }

    /// **Der Auffangzweig von [`wirkung`] ist erreichbar, und seine Zelle ist
    /// von der bewusst leeren unterscheidbar.**
    ///
    /// Der Fall, den der Kommentar am Zweig ausschreibt, hier gemessen statt
    /// behauptet: eine `keymap.toml` des Nutzers gibt `kopieren` einen
    /// Zusteller. `Belegung::vom_nutzer` nimmt sie an, `bereich` ordnet die
    /// Funktion ueber ihr Kommando ein — [`markdown`] bricht also **nicht** ab
    /// —, und `Funktion::kommando` liefert hier trotzdem `None`. Die Probe
    /// haelt beide Haelften fest: dass der Zweig greift, und dass die fertige
    /// Datei danach zwei verschiedene Sachverhalte auseinanderhaelt.
    #[test]
    fn eine_kennung_mit_kommando_und_zusteller_landet_im_auffangzweig() {
        let belegung = belegung_aus(
            r#"
            [[funktion]]
            id = "kopieren"
            name = "In das andere Fenster kopieren"
            tasten = ["f5"]
            gehalten_von = "menue"
            "#,
        );
        let funktion = belegung
            .funktion("kopieren")
            .expect("die Pruefbelegung fuehrt die Funktion");

        // Die beiden Fragen, und dass sie auseinanderfallen: daran haengt der
        // ganze Fall.
        assert_eq!(
            funktion.kommando(),
            None,
            "der Zusteller nimmt der Funktion ihr Kommando"
        );
        assert_eq!(
            Kommando::aus_kennung(funktion.kennung()),
            Some(Kommando::Kopieren),
            "ueber diesen Weg fragt `bereich`, und deshalb ordnet es die Funktion ein"
        );

        assert_eq!(wirkung(funktion), NICHT_EINGEORDNET);
        assert_ne!(
            wirkung(funktion),
            "",
            "die leere Zelle gehoert `text_alles_auswaehlen`; \
             die beiden Sachverhalte duerfen in der Datei nicht zusammenfallen"
        );

        let text = markdown(&belegung);
        let zeile = funktionszeilen(&text)
            .into_iter()
            .find(|zeile| zellen(zeile)[0] == "In das andere Fenster kopieren")
            .expect("die Zeile steht in der Datei");
        assert_eq!(
            zellen(zeile),
            ["In das andere Fenster kopieren", "F5", NICHT_EINGEORDNET]
        );
    }

    // -----------------------------------------------------------------------
    // Form der Datei
    // -----------------------------------------------------------------------

    /// Ein Name mit einem senkrechten Strich zerbricht die Tabelle nicht.
    #[test]
    fn ein_name_mit_senkrechtem_strich_zerbricht_die_tabelle_nicht() {
        let belegung = belegung_aus(
            r#"
            [[funktion]]
            id = "kopieren"
            name = "Kopieren | mit Strich"
            tasten = ["f5"]
            "#,
        );
        let text = markdown(&belegung);
        let zeilen = funktionszeilen(&text);
        assert_eq!(zeilen.len(), 1);
        assert!(
            zeilen[0].contains("Kopieren \\| mit Strich"),
            "der Strich ist nicht maskiert: {}",
            zeilen[0]
        );
        assert_eq!(
            zellen(zeilen[0]).len(),
            3,
            "die Zeile traegt trotz des Strichs drei Spalten: {}",
            zeilen[0]
        );
    }

    /// Genau eine Ueberschrift, kein Zeitstempel, keine Versionsangabe — und
    /// deshalb sind zwei Laeufe ueber dieselbe Belegung byteweise gleich.
    #[test]
    fn der_kopf_traegt_keinen_zeitstempel_und_zwei_laeufe_sind_gleich() {
        let belegung = Belegung::auslieferung();
        let erster = markdown(&belegung);
        let zweiter = markdown(&belegung);
        assert_eq!(erster, zweiter, "zwei Laeufe liefern verschiedene Bytes");

        let ueberschriften: Vec<&str> = erster
            .lines()
            .filter(|zeile| zeile.starts_with("# "))
            .collect();
        assert_eq!(ueberschriften, [UEBERSCHRIFT]);
        assert!(erster.starts_with("# Tastenbelegung von KRK\n"));
        assert!(
            erster.ends_with('\n'),
            "der Text endet mit einem Zeilenende"
        );

        // Kein Vorspann: unter der Ueberschrift folgt sofort der erste
        // Abschnitt, und zwischen beiden steht allein eine Leerzeile.
        let mut zeilen = erster.lines();
        assert_eq!(zeilen.next(), Some(UEBERSCHRIFT));
        assert_eq!(zeilen.next(), Some(""));
        assert!(
            zeilen.next().is_some_and(|zeile| zeile.starts_with("## ")),
            "zwischen Ueberschrift und erstem Abschnitt steht ein Vorspann"
        );
    }

    // -----------------------------------------------------------------------
    // Das Schreiben
    // -----------------------------------------------------------------------

    /// Ein zweiter Aufruf ersetzt die vorhandene Datei; danach liegt genau eine
    /// Datei dieses Namens im Ordner, mit dem Inhalt des zweiten Aufrufs. Eine
    /// fremde Datei desselben Namens wird ohne Rueckfrage ueberschrieben.
    #[test]
    fn ein_zweiter_aufruf_ersetzt_die_vorhandene_datei() {
        let ordner = Pruefordner::neu("belegungsausgabe-ersetzen");
        let ziel = ordner.datei(DATEINAME, "etwas Fremdes, das nicht von KRK stammt\n");

        let belegung = Belegung::auslieferung();
        assert_eq!(
            in_ordner_schreiben(&belegung, ordner.pfad()),
            Ausgang::Geschrieben(ziel.clone())
        );
        assert_eq!(
            in_ordner_schreiben(&belegung, ordner.pfad()),
            Ausgang::Geschrieben(ziel.clone())
        );

        assert_eq!(
            std::fs::read_to_string(&ziel).expect("die Datei steht nicht da"),
            markdown(&belegung)
        );

        let mit_diesem_namen = std::fs::read_dir(ordner.pfad())
            .expect("der Pruefordner laesst sich nicht lesen")
            .filter_map(Result::ok)
            .filter(|eintrag| eintrag.file_name() == DATEINAME)
            .count();
        assert_eq!(mit_diesem_namen, 1);

        // Die Nachbardatei des atomaren Schreibens bleibt nicht liegen.
        assert!(
            !ordner.unter(&format!("{DATEINAME}.neu")).exists(),
            "die Nachbardatei ist nach dem Umbenennen fort"
        );
    }

    /// Ein fehlender Ordner und ein Ordner ohne Schreibrecht sind
    /// unterscheidbar, keiner von beiden laesst eine Datei zurueck, und ihre
    /// Meldungen sind zwei verschiedene.
    #[test]
    fn ein_fehlender_ordner_und_ein_abgelehnter_zugriff_sind_unterscheidbar() {
        use std::os::unix::fs::PermissionsExt;

        let belegung = Belegung::auslieferung();

        let fehlt = Pruefordner::nur_name("belegungsausgabe-ohne-ordner");
        let ausgang_fehlt = in_ordner_schreiben(&belegung, fehlt.pfad());
        assert_eq!(
            ausgang_fehlt,
            Ausgang::OrdnerFehlt(fehlt.unter(DATEINAME)),
            "ein fehlender Ordner kommt als NotFound zurueck"
        );
        assert!(!fehlt.pfad().exists(), "KRK legt den Zielordner nicht an");

        let verschlossen = Pruefordner::neu("belegungsausgabe-ohne-recht");
        std::fs::set_permissions(verschlossen.pfad(), std::fs::Permissions::from_mode(0o500))
            .expect("die Rechte lassen sich nicht entziehen");
        let ausgang_abgelehnt = in_ordner_schreiben(&belegung, verschlossen.pfad());
        // Unter root greifen Zugriffsrechte nicht; die Probe belegt dann
        // nichts und bricht erkennbar ab, statt still durchzugehen.
        std::fs::set_permissions(verschlossen.pfad(), std::fs::Permissions::from_mode(0o700))
            .expect("die Rechte lassen sich nicht zuruecksetzen");
        assert_eq!(
            ausgang_abgelehnt,
            Ausgang::ZugriffAbgelehnt(verschlossen.unter(DATEINAME)),
            "ein Ordner ohne Schreibrecht kommt als PermissionDenied zurueck — \
             laeuft der Lauf unter root?"
        );
        assert!(
            !verschlossen.unter(DATEINAME).exists(),
            "es ist keine Datei entstanden"
        );
        assert!(
            !verschlossen.unter(&format!("{DATEINAME}.neu")).exists(),
            "es ist auch keine halbe Datei entstanden"
        );

        let zuhause = Path::new("/Users/kai");
        assert_ne!(
            ausgang_fehlt.meldung_mit(Some(zuhause)),
            ausgang_abgelehnt.meldung_mit(Some(zuhause)),
            "die beiden Faelle melden dasselbe"
        );
    }

    // -----------------------------------------------------------------------
    // Die Meldung
    // -----------------------------------------------------------------------

    /// Die Erfolgsmeldung traegt den Pfad mit Tilde, und die drei Meldungen mit
    /// Pfad aus den Fehlerfaellen tragen ihn in derselben Form.
    ///
    /// Der Wortlaut ist der Nutzerentscheid vom 260811-0900, gegen die
    /// Empfehlung des Plans. Eine Meldung fuer beide Faelle: ob die Datei neu
    /// entstanden ist oder eine vorhandene ersetzt hat, unterscheidet sie
    /// nicht — `Ausgang::Geschrieben` kennt den Unterschied gar nicht.
    #[test]
    fn die_meldungen_tragen_den_pfad_mit_tilde() {
        let zuhause = Path::new("/Users/kai");
        let ziel = zuhause.join(ZIELORDNER).join(DATEINAME);

        assert_eq!(
            Ausgang::Geschrieben(ziel.clone()).meldung_mit(Some(zuhause)),
            "Tastenbelegung geschrieben: ~/Downloads/KRK-Tastenbelegung.md"
        );

        for ausgang in [
            Ausgang::OrdnerFehlt(ziel.clone()),
            Ausgang::ZugriffAbgelehnt(ziel.clone()),
            Ausgang::Fehlgeschlagen(ziel.clone(), "die Platte ist voll".to_owned()),
        ] {
            let meldung = ausgang.meldung_mit(Some(zuhause));
            assert!(
                meldung.contains("~/Downloads/KRK-Tastenbelegung.md"),
                "die Meldung traegt den Pfad nicht in der gekuerzten Form: {meldung}"
            );
            assert!(
                !meldung.contains("/Users/kai/"),
                "die Meldung traegt den Pfad zusaetzlich ausgeschrieben: {meldung}"
            );
        }

        // Der eine Ausgang ohne Pfad meldet trotzdem einen Grund; kommentarlos
        // nichts zu tun ist in keinem Fall zulaessig.
        assert!(
            Ausgang::KeinBenutzerverzeichnis
                .meldung_mit(Some(zuhause))
                .contains("Benutzerverzeichnis")
        );
    }
}
