//! Die Ablage: sechs Dateien in zwei Formaten unter
//! `~/Library/Application Support/KRK/`.
//!
//! Vier tragen TOML und gehen ueber [`Zugang::laden`] und [`Zugang::sichern`];
//! die zwei Notizzettel der Runde 9 tragen nackten Text und gehen ueber
//! [`Zugang::text_laden`] und [`Zugang::text_sichern`]. Welche Datei welches
//! Format traegt, sagt [`pfade::Datei::format`], und der Kopf von [`pfade`]
//! sagt, warum die Zettel kein TOML tragen. Die vier Wege stehen nebeneinander
//! und nicht uebereinander: TOML und Text unterscheiden sich im Lesen, im
//! Auslieferungszustand und in dem, was eine beschaedigte Datei bedeutet.
//!
//! Sechs Module, in der Reihenfolge, in der ein Wert sie durchlaeuft:
//!
//! ```text
//! pfade ──> mod (Ablage ──> Zugang: laden, sichern, melden) ──> atomar
//!                   │              ^        ^         ^
//!                sperre            │        │         │
//!                           lesezeichen  sitzung  einstellungen
//! ```
//!
//! [`pfade`] loest den Ordner auf und legt ihn beim ersten Start an.
//! [`sperre`] traegt die beiden Absprachen, die zwei gleichzeitig laufende
//! Instanzen von KRK auseinanderhalten. [`atomar`] schreibt jede Datei ueber
//! eine Nachbardatei und `rename`. [`sitzung`], [`lesezeichen`] und
//! [`einstellungen`] halten drei der vier Inhalte; den vierten, die Belegung aus
//! `keymap.toml`, baut Schritt 11 und legt ihn ueber [`Zugang::laden`] und
//! [`Zugang::sichern`] hier ab. Die Ablage ist deshalb ueber den Inhalt
//! allgemein gehalten: sie kennt Pfad, Format und Fehlerbehandlung, nicht die
//! Felder.
//!
//! # Jeder Weg auf die Platte geht durch die Schreibsperre
//!
//! **[`Zugang`] steht zwischen der Ablage und [`atomar::schreiben`].** Ein
//! `Zugang` ist allein aus [`Ablage::durchgang`] zu bekommen, und der nimmt fuer
//! die Dauer des Durchgangs die Schreibsperre. Was durch ihn geht, wird damit
//! nie ausserhalb von ihr gelesen oder geschrieben; warum das Lesen mit
//! hineingehoert und nicht nur das Schreiben, steht im Kopf von [`sperre`].
//!
//! **Was die Typen halten, und was sie nicht halten.** Sie halten, dass **aus
//! der Ablage heraus** kein Weg an der Sperre vorbeifuehrt: die vier Lade- und
//! Schreibmethoden haengen an einem `Zugang`, und den gibt es nur im Durchgang.
//! Sie versperren die anderen Wege nicht. [`atomar::schreiben`] ist `pub`, weil
//! zwei Schreiber **ausserhalb** des Ablageordners es brauchen — die
//! Markdown-Ausgabe der Tastenbelegung nach `~/Downloads` und das Sichern der
//! Editordatei —, und [`Ablage::pfad`] liefert den Pfad einer der vier Dateien
//! ohne Durchgang, weil etliche Meldungen und Proben ihn zum **Lesen** brauchen.
//! Wer beides zusammennimmt, kann an der Sperre vorbeischreiben; der Uebersetzer
//! haelt ihn nicht auf.
//!
//! Diese eine Luecke bewacht deshalb eine Probe und kein Typ:
//! `nur_benannte_dateien_erreichen_das_atomare_schreiben` in
//! `krk-core/tests/baum.rs` zaehlt, welche Dateien des ganzen Baums
//! [`atomar::schreiben`] ueberhaupt erreichen koennen. Eine sechste laesst sie
//! rot werden. Bis zur Runde 7 stand hier, die Zusage sei „eine Eigenschaft der
//! Typen und keine Verabredung in Kommentaren"; sie war beides nicht, sondern
//! eine unbewachte Aussage ueber den damaligen Baum
//! (`issues/260813-0540_*_kein-schreibweg-an-der-sperre-vorbei-ist-nicht-typgesichert-und-ungeprueft.md`).
//!
//! # Eine der vier Dateien entsteht einmal und wird nie wieder geschrieben
//!
//! `settings.toml` aus Schritt 18c ist die eine von Hand gepflegte Datei, und
//! sie geht als einzige **nicht** ueber [`Ablage::sichern`]: die Anlage beim
//! ersten Start schreibt die eingebettete Auslieferungsfassung woertlich, samt
//! deren Kommentaren, die `serde` nicht kennt. Der Weg dorthin ist derselbe
//! [`atomar::schreiben`], allein die Nutzlast ist eine andere; siehe den Kopf
//! von [`einstellungen`].
//!
//! # Ein beschaedigter Bestand laesst KRK starten
//!
//! [`Ablage::laden`] liefert keinen Fehler, sondern immer einen Wert. Eine
//! fehlende Datei ist der erste Start und keine Meldung wert. Eine nicht
//! lesbare oder beschaedigte Datei fuehrt zum Auslieferungszustand und
//! zu einer [`Ersetzung`], die die Datei benennt. Die Datei auf der Platte
//! bleibt dabei stehen: `keymap.toml` ist laut `### Frage 4` von Hand
//! aenderbar, und ein Tippfehler darin darf die Arbeit des Nutzers nicht
//! loeschen. Ueberschrieben wird sie erst beim naechsten gewoehnlichen
//! Schreibvorgang.
//!
//! # Eine beschaedigte Datei wird zur Seite gelegt
//!
//! Dass sie liegen bleibt, hat den Nutzer bis zur Runde 6 nicht davor bewahrt,
//! seinen Bestand zu verlieren: **der naechste gewoehnliche Schreibvorgang ist
//! genau der Schaden**. Eine Fassung von KRK, die eine aeltere Datei nicht mehr
//! versteht, liest sie als beschaedigt, arbeitet auf dem Auslieferungszustand
//! weiter und schreibt ihn beim Beenden darueber.
//!
//! [`Ablage::laden`] legt den gelesenen Text deshalb unter
//! [`atomar::beiseitepfad`] daneben, bevor der Auslieferungszustand einspringt,
//! und [`Ersetzung::beiseite`] sagt, was dabei herauskam. Vier Regeln tragen
//! den Vorgang, und jede beantwortet eine Frage, die sonst geraten wuerde:
//!
//! - **Nur eine beschaedigte Datei wird gesichert.** Von einer, die sich nicht
//!   lesen liess, gibt es keinen Inhalt, und eine fehlende ist der erste Start.
//!   Seit der Runde 9 zaehlt "zu gross" mit dazu: eine Zetteldatei ueber
//!   `text::datei::EDITORGRENZE` wird nicht geladen, ihr Inhalt geht aber
//!   denselben Weg beiseite. Sie wird dabei aus ihrem offenen Deskriptor
//!   kopiert und steht zu keinem Zeitpunkt vollstaendig im Arbeitsspeicher.
//!   **Kopiert wird hoechstens `EDITORGRENZE`**, also dieselbe Zahl, die ueber
//!   dem Laden steht: von einer sehr grossen Fremddatei liegen danach allein
//!   die ersten 16 MB da, und [`Beiseite::Gekuerzt`] sagt es dem Nutzer. Der
//!   Preis ist angenommen, die Begruendung steht bei
//!   [`Zugang::beiseite_legen`].
//! - **Der Text wird kopiert und die Datei nicht verschoben.** Ein `rename`
//!   waere kuerzer und naehme dem Nutzer die Datei unter der Hand weg, an der er
//!   gerade tippt; siehe den Abschnitt darueber.
//! - **Eine schon dastehende Sicherung bleibt unangetastet.** Was zaehlt, ist
//!   die erste zur Seite gelegte Fassung, nicht die letzte.
//! - **Der Weg dorthin ist [`atomar::schreiben`]**, also derselbe wie fuer jede
//!   andere Datei dieses Moduls. Ein zweiter Schreibweg entsteht nicht.
//!
//! Alle vier TOML-Dateien gehen durch [`Zugang::laden`], und die vier Regeln
//! gelten dort fuer alle vier gleich: das Sichern selbst kennt keine Datei. Die
//! zwei Zettel gehen durch [`Zugang::text_laden`], und die vier Regeln gelten
//! dort unveraendert weiter — [`Zugang::beiseite_legen`] ist dieselbe Funktion
//! und hat mit dem Zettel ihren zweiten Aufrufer bekommen.
//!
//! # Beschaedigt heisst nicht „ungueltiges TOML"
//!
//! **Bis zum 260821 stand ueber diesem Abschnitt eine Zusage, die weiter reichte
//! als der Code.** Der Ladeweg fragte „ist das gueltiges TOML"; die Frage, die
//! er stellen muss, ist „hat die gelesene Datei den Bestand hergegeben, den sie
//! traegt". Zwei Gestalten kamen an der schmaleren Frage vorbei, und beide
//! endeten in einer leeren Liste ohne Meldung und ohne Sicherung, die der
//! naechste gewoehnliche Schreibvorgang festschrieb
//! (`shared/issues/260820-2235_*_eine-bookmarks-toml-die-serde-toleriert-aber-nicht-versteht-wird-still-als-leer-gelesen.md`).
//!
//! Zwei Stellen beantworten die weitere Frage, und keine davon ist ein zweiter
//! Mechanismus: beide muenden in denselben Zweig [`Grund::Beschaedigt`] und
//! damit in [`Zugang::beiseite_legen`].
//!
//! - **Ein oberster Schluessel, den der Leser nicht kennt**, ist ein `Err` und
//!   kein stiller Auslieferungszustand. Das leistet
//!   `#[serde(deny_unknown_fields)]` an der jeweiligen Struktur, und drei der
//!   vier TOML-Dateien tragen es: `Belegungsdatei`, `Einstellungsdatei` und
//!   seit dem 260821 auch [`Lesezeichenliste`].
//! - **Kein einziger oberster Schluessel** heisst je nach Datei etwas anderes,
//!   und deshalb steht die Antwort in [`pfade::Datei::leerbefund`] — einer
//!   vollstaendigen Fallunterscheidung ohne Auffangzweig, wie
//!   [`pfade::Datei::format`] daneben. `bookmarks.toml` traegt dort
//!   [`Leerbefund::Beschaedigt`], weil KRK selbst eine leere Liste als
//!   `eintraege = []` schreibt und die Datei nie ohne obersten Schluessel
//!   hinterlaesst. Die drei uebrigen tragen [`Leerbefund::Vorgabe`]: zwei davon
//!   pflegt der Nutzer von Hand und darf sie leerraeumen, und `session.toml`
//!   ist auf Nachsicht gegenueber einer aelteren Fassung gebaut. Ob die strenge
//!   Lesart auch dorthin gehoert, ist offen
//!   (`shared/decisions/260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`).
//!
//! **Die Zusage deckt weiterhin nicht jede Gestalt des Verlusts.** Eine Datei,
//! die dasteht und sich nicht lesen laesst, traegt [`Grund::NichtLesbar`] und
//! [`Beiseite::Nicht`]: es gibt keinen Inhalt zu sichern, und der naechste
//! Schreibvorgang schreibt trotzdem. Der Datensatz dazu ist
//! `shared/issues/260821-0142_*_eine-nicht-lesbare-ablagedatei-wird-nicht-gesichert-und-vom-naechsten-schreibvorgang-ueberschrieben.md`.
//!
//! # Der Kern gibt nichts aus
//!
//! [`melden`] ist die einzige Stelle im Kern, die aus einer [`Ersetzung`] einen
//! Satz fuer den Nutzer macht, und sie **schreibt ihn nicht, sondern gibt ihn
//! zurueck**. Bis Schritt 11 schrieb sie auf die Standardfehlerausgabe; im
//! Buendel erreichte das niemanden, weil eine ueber den Finder gestartete
//! Anwendung keine hat. Der Nutzer hat am 260804-0830 Moeglichkeit 1 aus
//! `decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md` gewaehlt: die
//! Meldung gehoert in die Statuszeile am Fuss des Dateifensters, und die baut
//! `krk-ui` in Schritt 12. Der Kern schreibt seither auf keinen Kanal mehr, und
//! das Abnahmekriterium von Schritt 12 prueft es mit einem `grep` ueber
//! `crates/krk-core/src` nach dem Namen des Ausgabemakros. Deshalb steht der
//! Name hier nirgends ausgeschrieben: er wuerde die eigene Pruefung brechen.
//!
//! **Die Aufrufrichtung bleibt dabei von oben nach unten.** Der Kern ruft die
//! Oberflaeche nicht an; er liefert einen Wert, und wer ihn geladen hat,
//! entscheidet, wo er ihn hinstellt. Eine zweite Abhaengigkeitsumkehr neben der
//! Papierkorb-Schnittstelle entsteht nicht.

pub mod atomar;
pub mod einstellungen;
pub mod lesezeichen;
pub mod pfade;
pub mod sitzung;
pub mod sperre;

use std::borrow::Cow;
use std::fmt;
use std::fs::{self, File};
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::text::datei::{EDITORGRENZE, Textstand, Unlesbarkeit};

pub use einstellungen::Einstellungen;
pub use lesezeichen::{
    Aenderung, Ausgang, Lesezeichen, Lesezeichenliste, Namenshinweis, Verschiebung, Ziel,
};
pub use pfade::{Ablageort, Datei, Format, Leerbefund, Zettel};
pub use sitzung::{
    Breiten, Dateifenster, Fensterseite, Sichtbarkeit, Sitzung, Sitzungsschreiber,
    Spaltensichtbarkeit, Tab,
};
pub use sperre::{Schreibgriff, Sitzungsrecht};

/// Warum eine Datei durch den Auslieferungszustand ersetzt wurde.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Grund {
    /// Die Datei liegt da, liess sich aber nicht lesen. Traegt die Meldung des
    /// Dateisystems.
    NichtLesbar(String),
    /// Die Datei liess sich lesen, hat ihren Bestand aber nicht hergegeben:
    /// kein gueltiges TOML, nicht die erwartete Gestalt, ein oberster
    /// Schluessel, den der Leser nicht kennt, oder — je nach Datei — kein
    /// einziger oberster Schluessel. Traegt die Meldung des Lesers, oder, wo es
    /// keine gibt, den Satz der Stelle, die den Befund gefasst hat.
    Beschaedigt(String),
    /// Die Datei fehlte und liess sich nicht anlegen. Traegt die Meldung des
    /// Dateisystems.
    ///
    /// Nur `settings.toml` kann ihn tragen. Sie ist die eine Datei, die KRK
    /// beim ersten Start von sich aus anlegt, weil in dieser Runde keine
    /// Ansicht sie schreibt und der Nutzer sonst nichts zu pflegen haette. Bei
    /// den drei uebrigen ist eine fehlende Datei der erste Start und keine
    /// Meldung wert.
    NichtAnlegbar(String),
    /// Die Datei ist groesser als [`EDITORGRENZE`] und wurde deshalb gar nicht
    /// erst gelesen.
    ///
    /// Nur eine Zetteldatei kann ihn tragen: die vier TOML-Dateien schreibt KRK
    /// selbst, und ihr Leseweg kennt keine Grenze. Der Wert steht neben
    /// [`Grund::Beschaedigt`] und nicht darin, weil die beiden verschiedene
    /// Auskuenfte sind — die eine laedt zum Teilen der Datei ein, die andere
    /// nicht. Dieselbe Unterscheidung trifft `text::datei::Abweisung` fuer den
    /// Editor.
    ZuGross {
        /// Die Groesse in Bytes, wie `fstat(2)` sie vor dem Lesen gemeldet hat.
        groesse: u64,
    },
}

impl Grund {
    /// Der Satzteil, der den Grund benennt.
    ///
    /// Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig: ein
    /// fuenfter Grund haelt den Bau an und erzwingt einen fuenften Satzteil.
    fn beschreibung(&self) -> &'static str {
        match self {
            Grund::NichtLesbar(_) => "ist nicht lesbar",
            Grund::Beschaedigt(_) => "ist beschaedigt",
            Grund::NichtAnlegbar(_) => "liess sich nicht anlegen",
            Grund::ZuGross { .. } => "ist zu gross",
        }
    }

    /// Die Einzelheit, die das System oder der Leser gemeldet hat.
    ///
    /// **`Cow` und nicht `&str`, seit [`Grund::ZuGross`] dazugekommen ist.** Er
    /// traegt eine Zahl und keinen Satz, und der Satz entsteht hier statt beim
    /// Erzeugen: sonst stuende die Grenze ein zweites Mal im Baum, an der
    /// Stelle, die den Wert baut. Die vier uebrigen Gruende reichen ihren Text
    /// weiter und kosten weiterhin keine Kopie.
    pub fn einzelheit(&self) -> Cow<'_, str> {
        match self {
            Grund::NichtLesbar(text) | Grund::Beschaedigt(text) | Grund::NichtAnlegbar(text) => {
                Cow::Borrowed(text)
            }
            Grund::ZuGross { groesse } => Cow::Owned(format!(
                "{groesse} Bytes, und die Grenze liegt bei {EDITORGRENZE} Bytes"
            )),
        }
    }
}

/// Was mit dem Inhalt der ersetzten Datei geschehen ist.
///
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig: der Uebersetzer
/// haelt an jeder Stelle an, die sie auseinandernimmt. Die fuenf Werte sind
/// paarweise verschieden und decken jeden Ausgang ab, und genau darauf beruht
/// die Zusage, dass keine Meldung eine Datei verspricht, die es nicht gibt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Beiseite {
    /// Es wurde nichts zur Seite gelegt, und das ist richtig so.
    ///
    /// Der Wert jeder Ersetzung ausser der beschaedigten: von einer Datei, die
    /// sich nicht lesen liess, gibt es keinen Inhalt zu sichern, und eine
    /// fehlende Datei ist der erste Start.
    Nicht,
    /// Der Inhalt liegt jetzt unter diesem Pfad, und zwar ganz.
    Gesichert(PathBuf),
    /// Unter diesem Pfad liegen die ersten [`EDITORGRENZE`] Bytes des Inhalts
    /// und der Rest nicht.
    ///
    /// Der Wert der Runde 9. Die Quelle war laenger als das Budget, das
    /// [`Zugang::beiseite_legen`] fuer eine Sicherung ausgibt; was darueber
    /// hinausging, ist nicht kopiert worden. Der Preis ist benannt und
    /// angenommen, und die Begruendung steht bei [`Zugang::beiseite_legen`].
    ///
    /// **Er ist von [`Beiseite::Gesichert`] getrennt und nicht mit ihm
    /// zusammengelegt**, weil die Meldung an den Nutzer eine andere sein muss:
    /// eine gekuerzte Sicherung sieht auf der Platte aus wie eine
    /// vollstaendige, und sie bleibt beim naechsten Start als
    /// [`Beiseite::SchonVorhanden`] stehen.
    Gekuerzt(PathBuf),
    /// Unter diesem Pfad stand schon eine Sicherung, und sie ist unangetastet
    /// geblieben.
    ///
    /// Der Fall tritt vom zweiten Start an ein, wenn KRK dieselbe Datei erneut
    /// nicht versteht. Die dastehende Fassung ist die aeltere und damit die
    /// wertvollere; siehe [`atomar::beiseitepfad`].
    SchonVorhanden(PathBuf),
    /// Das Zur-Seite-Legen ist selbst gescheitert. Traegt die Meldung des
    /// Dateisystems.
    ///
    /// Der Wert nennt **keinen** Pfad, denn unter ihm liegt nichts.
    Gescheitert(String),
}

/// Eine Datei wurde durch den Auslieferungszustand ersetzt.
///
/// Ein Wert und keine Ausgabe: wer laedt, entscheidet, ob und wie er ihn
/// meldet. Der Weg dorthin ist [`melden`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ersetzung {
    /// Die Datei, um die es geht. Sie steht in jeder Meldung.
    pub datei: PathBuf,
    /// Warum sie ersetzt wurde.
    pub grund: Grund,
    /// Was mit ihrem Inhalt geschehen ist.
    pub beiseite: Beiseite,
}

impl fmt::Display for Ersetzung {
    /// Der Satz sagt zuerst, was der Nutzer tun kann, und danach, was geschehen
    /// ist.
    ///
    /// Die Reihenfolge ist die Antwort vom 260812-1105
    /// (`decisions/260812-1000_*_wie-erfaehrt-der-nutzer-dass-eine-ablagedatei-zur-seite-gelegt-wurde.md`):
    /// die Meldung steht in der Statuszeile und nicht in einem Blatt, sie
    /// erscheint beim Start, und der Nutzer liest ihren Anfang. Die
    /// Wiederherstellbarkeit seines Bestands ist die eigentliche Nachricht; die
    /// Ersetzung ist der Nebensatz.
    ///
    /// Beide Pfade stehen im Satz, sobald es beide gibt. Fuer
    /// [`Beiseite::Nicht`] bleibt der Satz Wort fuer Wort der von vor der
    /// Runde 6, denn dort gibt es keinen zweiten Pfad und nichts hinzuzufuegen.
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datei = self.datei.display();
        let beschreibung = self.grund.beschreibung();
        let einzelheit = self.grund.einzelheit();
        match &self.beiseite {
            Beiseite::Nicht => write!(
                ausgabe,
                "{datei} {beschreibung} und wird durch den Auslieferungszustand ersetzt: \
                 {einzelheit}"
            ),
            Beiseite::Gesichert(pfad) => write!(
                ausgabe,
                "Die bisherige Fassung liegt unter {}; {datei} {beschreibung} und wird durch den \
                 Auslieferungszustand ersetzt: {einzelheit}",
                pfad.display()
            ),
            Beiseite::Gekuerzt(pfad) => write!(
                ausgabe,
                "Die bisherige Fassung liegt gekuerzt unter {}, gesichert sind allein ihre \
                 ersten {EDITORGRENZE} Bytes; {datei} {beschreibung} und wird durch den \
                 Auslieferungszustand ersetzt: {einzelheit}",
                pfad.display()
            ),
            Beiseite::SchonVorhanden(pfad) => write!(
                ausgabe,
                "Die bisherige Fassung liegt seit einem frueheren Start unter {} und bleibt dort; \
                 {datei} {beschreibung} und wird durch den Auslieferungszustand ersetzt: \
                 {einzelheit}",
                pfad.display()
            ),
            Beiseite::Gescheitert(fehler) => write!(
                ausgabe,
                "Der Inhalt liess sich nicht zur Seite legen ({fehler}); {datei} {beschreibung} \
                 und wird durch den Auslieferungszustand ersetzt: {einzelheit}"
            ),
        }
    }
}

/// Der Satz, den der Nutzer zu einer [`Ersetzung`] lesen soll.
///
/// Die eine Stelle im Kern, die eine Ersetzung in Worte fasst. Sie gibt den
/// Text zurueck und schreibt ihn nirgendwohin; siehe den Abschnitt "Der Kern
/// gibt nichts aus" im Modulkopf.
///
/// Der frueher vorangestellte Programmname `krk: ` ist mit Schritt 12
/// entfallen. Er war die Anrede eines Terminals; in einer Statuszeile, die
/// ausschliesslich KRK gehoert, benennt er nichts, und der Satz nennt die
/// betroffene Datei ohnehin selbst.
#[must_use]
pub fn melden(ersetzung: &Ersetzung) -> String {
    ersetzung.to_string()
}

/// Das Ergebnis eines Ladevorgangs: immer ein Wert, dazu die Meldung, falls
/// eine noetig war.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Geladen<T> {
    /// Der gelesene Wert, oder der Auslieferungszustand.
    pub wert: T,
    /// Gesetzt, wenn der Auslieferungszustand eingesprungen ist.
    pub ersetzung: Option<Ersetzung>,
}

impl<T> Geladen<T> {
    /// Ob der Auslieferungszustand eingesprungen ist.
    pub fn ist_ersetzt(&self) -> bool {
        self.ersetzung.is_some()
    }

    /// Der Wert und der Satz, den der Aufrufer dem Nutzer zeigen muss.
    ///
    /// Die Vorgaengerin `gemeldet` hat den Satz selbst geschrieben. Sie ist mit
    /// Schritt 12 entfallen, weil der Kern keinen Ausgabekanal mehr hat: wer
    /// laedt, bekommt den Text und stellt ihn in seine Statuszeile.
    pub fn mit_meldung(self) -> (T, Option<String>) {
        let meldung = self.ersetzung.as_ref().map(melden);
        (self.wert, meldung)
    }
}

/// Der Ablageordner mit den vier Dateien, samt der Schreibsperre darueber.
///
/// **Sie laedt und schreibt nicht selbst.** Wer eine der vier Dateien anfassen
/// will, geht durch [`Ablage::durchgang`] und bekommt einen [`Zugang`]; siehe
/// den Abschnitt „Jeder Weg auf die Platte geht durch die Schreibsperre" im
/// Modulkopf.
///
/// Kein `Clone` und kein `PartialEq`: der Wert haelt einen Deskriptor auf die
/// Sperrdatei, und zwei Ablagen desselben Prozesses auf denselben Ordner
/// schliessen einander am Durchgang aus. Was daraus folgt, steht im Kopf von
/// [`sperre`].
#[derive(Debug)]
pub struct Ablage {
    ort: Ablageort,
    /// Der offen gehaltene Deskriptor, an dem die Schreibsperre haengt.
    ///
    /// Offen gehalten und nicht je Durchgang neu geoeffnet: ein `open` je
    /// Lesezeichenbefehl waere ein Systemaufruf fuer nichts, und der Deskriptor
    /// traegt die Sperre ohnehin erst, wenn [`Schreibgriff::nehmen`] sie nimmt.
    sperrdatei: File,
}

impl Ablage {
    /// Oeffnet die Ablage an einem Ort und legt den Ordner an, falls er fehlt.
    pub fn oeffnen(ort: Ablageort) -> io::Result<Self> {
        ort.anlegen()?;
        let sperrdatei = sperre::sperrdatei_oeffnen(ort.wurzel(), sperre::SCHREIBSPERRE)?;
        Ok(Self { ort, sperrdatei })
    }

    /// Oeffnet die Ablage unter `~/Library/Application Support/KRK/`.
    ///
    /// Der Weg des laufenden Programms. Der erste Start legt den Ordner an.
    pub fn im_benutzerverzeichnis() -> io::Result<Self> {
        Self::oeffnen(Ablageort::im_benutzerverzeichnis()?)
    }

    /// Der Ort, an dem diese Ablage liegt.
    pub fn ort(&self) -> &Ablageort {
        &self.ort
    }

    /// Der Pfad einer der vier Dateien.
    ///
    /// Eine Frage an den Ort und kein Zugriff auf die Platte; sie braucht die
    /// Sperre deshalb nicht.
    pub fn pfad(&self, welche: Datei) -> PathBuf {
        self.ort.datei(welche)
    }

    /// Fuehrt einen vollstaendigen Durchgang aus Lesen, Aendern und Schreiben
    /// unter der Schreibsperre aus.
    ///
    /// Die Sperre wird beim Eintritt genommen, wartend, und beim Verlassen
    /// wieder abgegeben — auch dann, wenn der Rumpf in Panik geraet, denn der
    /// [`Schreibgriff`] gibt sie in seinem `Drop` ab.
    ///
    /// **Der Rumpf ist ein Blatt und wird nicht geschachtelt.** Ein zweiter
    /// Durchgang darin gaebe die Sperre des ersten vorzeitig ab; der Grund steht
    /// im Kopf von [`sperre`]. Der Uebersetzer haelt das nicht.
    ///
    /// Der Fehler kommt vom Nehmen der Sperre und nicht aus dem Rumpf: was der
    /// Rumpf zurueckgibt, entscheidet er selbst, und die vier Lade- und
    /// Schreibwege von [`Zugang`] tragen ihre Antworten je einzeln.
    pub fn durchgang<T>(&self, arbeit: impl FnOnce(&Zugang<'_>) -> T) -> io::Result<T> {
        let _griff = Schreibgriff::nehmen(&self.sperrdatei)?;
        Ok(arbeit(&Zugang { ort: &self.ort }))
    }
}

/// Der eine Weg von der Ablage auf die Platte, und er ist nur unter der
/// Schreibsperre zu bekommen.
///
/// Ein `Zugang` entsteht ausschliesslich in [`Ablage::durchgang`] und lebt
/// genau so lange wie der [`Schreibgriff`] daneben. Damit ist „es gibt keinen
/// Schreibweg an der Sperre vorbei" eine Aussage ueber die Typen und nicht ueber
/// die Aufmerksamkeit des naechsten Lesers.
#[derive(Debug)]
pub struct Zugang<'a> {
    ort: &'a Ablageort,
}

impl Zugang<'_> {
    /// Der Pfad einer der vier Dateien.
    pub fn pfad(&self, welche: Datei) -> PathBuf {
        self.ort.datei(welche)
    }

    /// Liest eine der vier Dateien.
    ///
    /// Scheitert nie: eine fehlende, nicht lesbare oder beschaedigte Datei
    /// fuehrt zum Auslieferungszustand. Nur die letzten beiden Faelle tragen
    /// eine [`Ersetzung`]; eine fehlende Datei ist der erste Start.
    ///
    /// **Zur Seite gelegt wird allein im Zweig [`Grund::Beschaedigt`]**, denn
    /// nur dort gibt es einen gelesenen Text zu sichern. Die beiden uebrigen
    /// Zweige tragen [`Beiseite::Nicht`]; siehe den Modulkopf.
    ///
    /// **Beschaedigt heisst mehr als „kein gueltiges TOML".** Eine dastehende
    /// Datei, aus der kein einziger oberster Schluessel kommt, hat ihren
    /// Bestand nicht hergegeben; ob das ein Schaden ist, entscheidet
    /// [`Datei::leerbefund`] je Datei und nicht diese Stelle. Der gleichnamige
    /// Abschnitt im Modulkopf traegt die Begruendung und die zweite Haelfte
    /// derselben Regel, die an den Strukturen steht.
    ///
    /// **Das Lesen steht mit unter der Sperre und nicht davor.** Ein Aufrufer,
    /// der seine eine Aenderung auf den eben gelesenen Stand anwendet, haette
    /// sonst zwischen Lesen und Schreiben ein Fenster, in dem die andere Instanz
    /// schreibt; die verlorene Aenderung waere dann nur seltener und nicht fort.
    /// Und schon das Laden schreibt: eine beschaedigte Datei wird zur Seite
    /// gelegt.
    pub fn laden<T>(&self, welche: Datei) -> Geladen<T>
    where
        T: DeserializeOwned + Default,
    {
        debug_assert_eq!(
            welche.format(),
            Format::Toml,
            "{} traegt kein TOML; der Weg dorthin ist text_laden",
            welche.dateiname()
        );
        let pfad = self.pfad(welche);
        let text = match fs::read_to_string(&pfad) {
            Ok(text) => text,
            Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => {
                return Geladen {
                    wert: T::default(),
                    ersetzung: None,
                };
            }
            Err(fehler) => {
                return Geladen {
                    wert: T::default(),
                    ersetzung: Some(Ersetzung {
                        datei: pfad,
                        grund: Grund::NichtLesbar(fehler.to_string()),
                        beiseite: Beiseite::Nicht,
                    }),
                };
            }
        };
        if welche.leerbefund() == Leerbefund::Beschaedigt && ohne_obersten_schluessel(&text) {
            let beiseite = self.beiseite_legen(&pfad, &mut text.as_bytes());
            return Geladen {
                wert: T::default(),
                ersetzung: Some(Ersetzung {
                    datei: pfad,
                    grund: Grund::Beschaedigt(String::from(
                        "die Datei traegt keinen einzigen obersten Schluessel, \
                         und KRK schreibt sie nie so",
                    )),
                    beiseite,
                }),
            };
        }
        match toml::from_str(&text) {
            Ok(wert) => Geladen {
                wert,
                ersetzung: None,
            },
            Err(fehler) => {
                let beiseite = self.beiseite_legen(&pfad, &mut text.as_bytes());
                Geladen {
                    wert: T::default(),
                    ersetzung: Some(Ersetzung {
                        datei: pfad,
                        grund: Grund::Beschaedigt(einzeilig(&fehler.to_string())),
                        beiseite,
                    }),
                }
            }
        }
    }

    /// Schreibt eine der vier Dateien, atomar ueber [`atomar::schreiben`].
    ///
    /// **Nicht der Weg zu `settings.toml`.** Die Serialisierung kennt keine
    /// Kommentare, und die von Hand gepflegte Datei besteht zur Haelfte aus
    /// ihnen; ihre Anlage schreibt deshalb die eingebettete
    /// Auslieferungsfassung woertlich. Siehe den Kopf von [`einstellungen`].
    pub fn sichern<T>(&self, welche: Datei, wert: &T) -> io::Result<()>
    where
        T: Serialize,
    {
        debug_assert_eq!(
            welche.format(),
            Format::Toml,
            "{} traegt kein TOML; der Weg dorthin ist text_sichern",
            welche.dateiname()
        );
        let text = toml::to_string(wert).map_err(io::Error::other)?;
        atomar::schreiben(&self.pfad(welche), &mut text.as_bytes())
    }

    /// Liest eine der zwei Zetteldateien als nackten Text (C5 der Runde 9).
    ///
    /// Scheitert nie, wie [`laden`](Self::laden) auch: es kommt immer ein Text
    /// heraus, notfalls ein leerer. Der Befund kommt aus
    /// [`text::datei::lesen`](crate::text::datei::lesen), also aus derselben
    /// Stelle, die der Editor benutzt; ein dritter Weg an das Dateisystem
    /// entsteht nicht, und `EDITORGRENZE` steht weiterhin an genau einer
    /// Stelle.
    ///
    /// **Die vier Ausgaenge und ihre Uebersetzung**, vollstaendig und ohne
    /// Auffangzweig:
    ///
    /// ```text
    ///   Text                     ──> der gelesene Zettel, keine Meldung
    ///   KeinGueltigesZiel, fehlt ──> leerer Zettel, keine Meldung
    ///   KeinGueltigesZiel        ──> leerer Zettel, Meldung, nichts beiseite
    ///   Unlesbar (zu gross)      ──> leerer Zettel, Meldung, beiseitegelegt
    ///   Unlesbar (kein Text)     ──> leerer Zettel, Meldung, beiseitegelegt
    /// ```
    ///
    /// **Die fehlende Datei ist der erste Start und keine Meldung wert.** Das
    /// ist dieselbe Regel, die [`laden`](Self::laden) fuer eine fehlende
    /// TOML-Datei anwendet, und sie steht hier nicht daneben, sondern haengt an
    /// dem einen Feld `fehlt` des Befundes.
    ///
    /// **Beiseitegelegt wird in beiden unlesbaren Faellen, und das ist die
    /// Antwort des Nutzers vom 260814-0005.** Zeigte der Zettel eine unlesbare
    /// Datei als leer an, ohne ihren Inhalt zu sichern, schriebe der naechste
    /// Sicherungsmoment den leeren Stand darueber: ein blosser Blick auf einen
    /// Zettel vernichtete eine Datei. Der Weg ist
    /// [`beiseite_legen`](Self::beiseite_legen) und kein daneben gebauter
    /// zweiter.
    pub fn text_laden(&self, welche: Datei) -> Geladen<String> {
        debug_assert_eq!(
            welche.format(),
            Format::Text,
            "{} traegt TOML; der Weg dorthin ist laden",
            welche.dateiname()
        );
        let pfad = self.pfad(welche);
        match crate::text::datei::lesen(&pfad) {
            Textstand::Text(text) => Geladen {
                wert: text,
                ersetzung: None,
            },
            // Die Datei gibt es nicht: der erste Start eines Zettels.
            Textstand::KeinGueltigesZiel { fehlt: true, .. } => Geladen {
                wert: String::new(),
                ersetzung: None,
            },
            Textstand::KeinGueltigesZiel {
                grund,
                fehlt: false,
            } => Geladen {
                wert: String::new(),
                ersetzung: Some(Ersetzung {
                    datei: pfad,
                    grund: Grund::NichtLesbar(einzeilig(&grund)),
                    // Von einer Datei, die sich nicht oeffnen liess, gibt es
                    // keinen Inhalt zu sichern.
                    beiseite: Beiseite::Nicht,
                }),
            },
            Textstand::Unlesbar { mut datei, grund } => {
                let beiseite = self.beiseite_legen(&pfad, &mut datei);
                let grund = match grund {
                    Unlesbarkeit::ZuGross(groesse) => Grund::ZuGross { groesse },
                    Unlesbarkeit::KeinText => {
                        Grund::Beschaedigt(String::from("keine gueltige UTF-8-Folge"))
                    }
                };
                Geladen {
                    wert: String::new(),
                    ersetzung: Some(Ersetzung {
                        datei: pfad,
                        grund,
                        beiseite,
                    }),
                }
            }
        }
    }

    /// Schreibt eine der zwei Zetteldateien, atomar ueber
    /// [`atomar::schreiben`].
    ///
    /// Geschrieben wird der Text des Zettels und sonst nichts: kein TOML, kein
    /// Kopf, keine Bytefolgenmarke und kein angehaengter Umbruch. Was der
    /// Nutzer im Zettel stehen hat, steht in der Datei, und was in der Datei
    /// steht, kommt beim naechsten Oeffnen unveraendert zurueck.
    pub fn text_sichern(&self, welche: Datei, text: &str) -> io::Result<()> {
        debug_assert_eq!(
            welche.format(),
            Format::Text,
            "{} traegt TOML; der Weg dorthin ist sichern",
            welche.dateiname()
        );
        atomar::schreiben(&self.pfad(welche), &mut text.as_bytes())
    }

    /// Legt den Inhalt einer Datei, die KRK nicht versteht, unter festem Namen
    /// daneben.
    ///
    /// **Die Quelle ist ein Leser und keine Zeichenkette**, und sie hat mit der
    /// Runde 9 ihren zweiten Aufrufer bekommen. Die vier TOML-Dateien reichen
    /// ihren gelesenen Text als `&mut text.as_bytes()` herein; eine Zetteldatei
    /// reicht ihren **offenen Deskriptor** herein, denn ihre zwei unlesbaren
    /// Faelle tragen keinen `&str`: eine ungueltige Bytefolge ist definitions-
    /// gemaess keiner, und eine Datei ueber [`EDITORGRENZE`] darf zu keinem
    /// Zeitpunkt vollstaendig im Arbeitsspeicher stehen. Die vier Regeln unten
    /// gelten fuer beide Aufrufer Wort fuer Wort gleich.
    ///
    /// Die Reihenfolge ist ausgeschrieben, damit sie nicht geraten wird: den
    /// Pfad bilden, fragen, ob dort schon etwas steht, und nur dann schreiben.
    /// Steht schon etwas, bleibt es unangetastet — die aeltere Fassung ist die
    /// wertvollere, siehe [`atomar::beiseitepfad`].
    ///
    /// **Der Text wird kopiert, die Datei nicht verschoben.** Ein `rename` waere
    /// kuerzer und ist falsch: `keymap.toml` und `settings.toml` sind von Hand
    /// aenderbar, und ein Tippfehler darin darf dem Nutzer nicht die Datei unter
    /// der Hand wegnehmen, an der er gerade tippt. Der Modulkopf sagt diese
    /// Zusage seit Schritt 10 der Runde 1 zu.
    ///
    /// **Das Wettrennen zwischen der Frage und dem Schreiben ist benannt und
    /// nicht erreichbar**, und die Begruendung dafuer ist mit der Runde 7 eine
    /// andere geworden. Bis dahin lautete sie „der Vorgang laeuft einmal je
    /// Start in einem Prozess"; mit einer zweiten Instanz von KRK stimmt der
    /// Satz nicht mehr. Unerreichbar ist das Wettrennen jetzt, weil der ganze
    /// Durchgang unter der Schreibsperre laeuft und diese Methode allein an
    /// einem [`Zugang`] haengt. Ein `File::create_new` an dieser Stelle waere
    /// der zweite Schreibweg, den der Datensatz vom 260812-1105 ausschliesst.
    ///
    /// **Kopiert wird hoechstens [`EDITORGRENZE`], und der Preis dieser Regel
    /// gehoert dazu: von einer sehr grossen Fremddatei werden allein die ersten
    /// 16 MB gesichert.** Es ist dieselbe Zahl, die ueber dem Laden steht, und
    /// ausdruecklich keine zweite daneben; so hat der Nutzer am 260814-1010
    /// entschieden. Ohne sie kopierte ein `f2` eine Datei von 40 GB, die unter
    /// dem Namen eines Zettels liegt, in voller Laenge und synchron auf dem
    /// Hauptfaden, mit stehender Oberflaeche, gehaltenem Schreibgriff und einem
    /// Ablageordner, der um dieselben 40 GB waechst
    /// (`issues/260814-0910_*_eine-zetteldatei-ueber-editorgrenze-wird-unbegrenzt-auf-dem-hauptfaden-kopiert.md`).
    /// Der Preis ist der kleinere Verlust: gekuerzt gesichert ist mehr als gar
    /// nicht gesichert, und der Nutzer erfaehrt von der Kuerzung, weil
    /// [`Beiseite::Gekuerzt`] eine eigene Meldung traegt. Verworfen sind eine
    /// zweite, groessere Zahl und ein Umbenennen statt eines Kopierens;
    /// Letzteres verstiesse gegen die Regel darueber.
    ///
    /// **Ob gekuerzt wurde, entscheidet ein einzelnes Byte hinter dem Budget**
    /// und nicht das ausgeschoepfte Budget selbst: eine Datei von genau
    /// [`EDITORGRENZE`] Bytes ist vollstaendig gesichert und wird auch so
    /// gemeldet. Die Frage stellt [`steht_noch_etwas_an`].
    #[must_use]
    fn beiseite_legen(&self, datei: &Path, quelle: &mut impl Read) -> Beiseite {
        let pfad = match atomar::beiseitepfad(datei) {
            Ok(pfad) => pfad,
            Err(fehler) => return Beiseite::Gescheitert(einzeilig(&fehler.to_string())),
        };
        match pfad.try_exists() {
            Ok(true) => return Beiseite::SchonVorhanden(pfad),
            Ok(false) => {}
            Err(fehler) => return Beiseite::Gescheitert(einzeilig(&fehler.to_string())),
        }
        let mut begrenzt = quelle.by_ref().take(EDITORGRENZE);
        if let Err(fehler) = atomar::schreiben(&pfad, &mut begrenzt) {
            return Beiseite::Gescheitert(einzeilig(&fehler.to_string()));
        }
        if begrenzt.limit() > 0 {
            // Die Quelle war vor dem Budget zu Ende; laenger als sie selbst kann
            // die Sicherung nicht sein.
            return Beiseite::Gesichert(pfad);
        }
        if steht_noch_etwas_an(quelle) {
            Beiseite::Gekuerzt(pfad)
        } else {
            Beiseite::Gesichert(pfad)
        }
    }
}

/// Fragt eine Quelle, ob hinter dem ausgeschoepften Budget noch etwas steht.
///
/// Ein Byte genuegt fuer die Antwort, und mehr als eines wird nicht gelesen:
/// die Frage lautet nicht "wie lang ist der Rest", sondern "gibt es einen".
///
/// **Ein Lesefehler wird als "ja" beantwortet**, und das ist die vorsichtige
/// Seite: die Sicherung liegt dann genau auf der Grenze, ihre Vollstaendigkeit
/// ist unbekannt, und eine Meldung, die sie faelschlich als vollstaendig
/// ausgibt, kostet den Nutzer mehr als eine, die faelschlich vor einer Kuerzung
/// warnt. `Interrupted` ist kein solcher Fehler, sondern ein unterbrochener
/// Versuch, und wird wiederholt.
fn steht_noch_etwas_an(quelle: &mut impl Read) -> bool {
    let mut eins = [0_u8; 1];
    loop {
        match quelle.read(&mut eins) {
            Ok(0) => return false,
            Ok(_) => return true,
            Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => return true,
        }
    }
}

/// Ob eine dastehende Datei nicht einen einzigen obersten Schluessel traegt.
///
/// **Die Frage steht am Dokument und nicht am gelesenen Wert.** Eine Struktur,
/// deren Felder alle einen Auslieferungswert haben, nimmt das leere Dokument
/// widerspruchslos an; danach ist der gelesene Wert von einem echten Bestand
/// nicht mehr zu unterscheiden, und genau daran ist der Verlust vorbeigekommen,
/// den
/// `shared/issues/260820-2235_*_eine-bookmarks-toml-die-serde-toleriert-aber-nicht-versteht-wird-still-als-leer-gelesen.md`
/// gemessen hat. Das Dokument dagegen sagt, was in der Datei stand.
///
/// **Ungueltiges TOML beantwortet sie mit „nein"**, und das ist keine
/// Nachlaessigkeit: fuer diesen Fall steht der Zweig darunter, und er traegt
/// die Meldung des Lesers samt Zeile und Spalte, die hier niemand kennt.
fn ohne_obersten_schluessel(text: &str) -> bool {
    toml::from_str::<toml::Table>(text).is_ok_and(|dokument| dokument.is_empty())
}

/// Presst eine mehrzeilige Fehlermeldung in eine Zeile.
///
/// Der TOML-Leser zeichnet die Fundstelle ueber mehrere Zeilen aus. In einer
/// Meldung, die spaeter in eine Statuszeile passen muss, ist das eine Zeile zu
/// viel; die Angaben zu Zeile und Spalte bleiben erhalten.
fn einzeilig(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}
