//! Das Modell des Vorschaufensters: seine Tabs, ihr Inhalt und das
//! Halteverhalten (C6, C10).
//!
//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile, wie
//! in `fenstermodell` und `tabs` daneben. Die Ansicht dazu ist
//! [`crate::appkit::vorschau`], die aus einem [`Inhalt`] Text, ein `NSImage`
//! oder die Metadatenzeilen macht.
//!
//! # Das Halteverhalten, aus dem Zustandsdiagramm des Specs
//!
//! ```text
//! [*] ──> Aktiv ──(neue Auswahl im Dateifenster)──> Aktiv, Inhalt ersetzt
//!           │ ▲
//!  (Tabwechsel)│(Rueckkehr, Inhalt unveraendert)
//!           ▼ │
//!         Inaktiv ──(Auswahl wechselt)──> Inaktiv, Inhalt bleibt stehen
//! ```
//!
//! Getragen wird es von einer einzigen Regel: **jede Quelle schreibt in den
//! aktiven Tab und in keinen anderen.** [`Vorschaumodell::datei_anzeigen`] und
//! [`Vorschaumodell::zwischenablage_anzeigen`] nehmen keine Tabstelle
//! entgegen; ein inaktiver Tab ist von keiner der beiden erreichbar und
//! behaelt seinen Inhalt, bis der Nutzer auf ihn zurueckwechselt und dort
//! selbst ueberschreibt. Eine Tab-Sorte mit eigener Regel entsteht nicht, auch
//! nicht fuer die Zwischenablage aus C10.
//!
//! # Die Dreiteilung der Anzeige (C6)
//!
//! Textdateien bis 1 MB erscheinen als reiner Inhalt, die
//! gaengigen Bildformate bis 64 MB als Bild, alles andere, einschliesslich
//! Ordner, als Metadaten mit Name, vollstaendigem Pfad, Groesse,
//! Aenderungsdatum, Rechten und Typ. Eine Textdatei ueber 1 MB faellt auf die
//! Metadaten; das Abnahmekriterium des Schritts laesst beide Wege zu, und die
//! Metadaten sind der ohne zweite Leseregel.
//!
//! **Beide Grenzen sind dieselbe Regel mit zwei Zahlen.** Bis zum 260806 trug
//! allein der Text eine; eine Bilddatei wurde ohne jede Pruefung vollstaendig
//! gelesen, und ein TIFF-Export von mehreren Gigabyte lief damit als Ganzes in
//! den Arbeitsspeicher des Referenzgeraets von 2018
//! (`issues/260806-0834_*_die-vorschau-liest-bilddateien-ohne-groessengrenze-
//! vollstaendig-in-den-speicher.md`). Der Rueckfallweg ist derselbe wie beim
//! Text und war schon da: [`Inhalt::Bild`] fuehrt die Metadaten ohnehin mit,
//! damit die Ansicht bei einer nicht dekodierbaren Datei auf sie zurueckfallen
//! kann. Ueber der Grenze faellt sie eine Stufe frueher darauf zurueck, ohne
//! gelesen zu haben.
//!
//! **Die Bildgrenze gilt auf beiden Wegen in dieselbe Flaeche.** In die Anzeige
//! fuehren zwei Wege, der Dateiweg ueber [`laden`] und der der Zwischenablage
//! aus C10 ueber [`Vorschaumodell::zwischenablage_anzeigen`]. Bis zum 260806
//! trug allein der erste die Grenze; ein kopiertes TIFF ueber 100 MB lief am
//! zweiten vorbei in den Speicher
//! (`issues/260806-1332_*_das-bild-aus-der-zwischenablage-umgeht-beide-
//! groessengrenzen.md`). Gemessen wird jetzt auch dort vor dem Kopieren, an der
//! Laenge des `NSData`, und mit derselben Konstanten. Eine Textgrenze braucht
//! der zweite Weg nicht: was aus der Zwischenablage als Text kommt, ist bereits
//! eine `String` im Speicher, und eine Pruefung danach spart nichts mehr.
//!
//! **Die Rechte erhebt der Arbeitsfaden beim Anzeigen**, mit einem `stat(2)`
//! auf den einen angezeigten Pfad. `Eintrag` aus S2 bleibt so schmal, wie L10
//! es verlangt; das ist Weg 2 aus `issues/260803-2007_*_die-metadatenvorschau-
//! aus-c6-verlangt-rechte-die-der-eintrag-nicht-traegt.md`.
//!
//! # Die Zusammenfassung ist der vierte Weg und die vierte Antwort nicht
//!
//! Seit der Runde 16 steht neben den drei Wegen ein vierter: ein Ordner, den
//! ein Leseprofil aus `readers.toml` an seinem Pfad oder an einer
//! Kennzeichendatei darin erkennt, zeigt statt der Metadatenzeilen die Zeilen
//! seines Profils ([`Inhalt::Zusammenfassung`]). **Er teilt die Dreiteilung
//! nicht in vier, sondern besetzt einen Teil des dritten:** Text und Bild
//! bleiben unberuehrt, und was sich aendert, ist allein, was an der Stelle
//! „alles Uebrige" fuer einen **erkannten** Ordner steht. Auf eine **Datei**
//! greift kein Profil, auch dann nicht, wenn ihr Pfad ein Pfadmuster erfuellt
//! (C2.6). Die Kopfzeile mit Name und vollem Pfad ist die eine Auskunft der
//! Metadaten, die die Ersetzung ueberlebt; sie steckt in der
//! [`Zusammenfassung`](krk_core::leseprofil::Zusammenfassung) selbst
//! (Festlegung A6).
//!
//! **Seit der Runde 19 gibt es fuer einen Ordner ohne Profiltreffer zwei
//! Antworten, und sie unterscheiden sich in ihrer Richtung.** Die
//! Zusammenfassung des erkannten Ordners **ersetzt** die sechs
//! Metadatenangaben; die drei Zaehlzeilen des eingebauten Default-Profils
//! (Dateien, Ordner, Verknuepfungen) **treten unter** sie (C2.1, C2.2). Die
//! sechs Angaben aus C2.5 der Runde 16 stehen weiter unveraendert da, und
//! ihre Anzeige waechst um drei Zeilen. Der Kern sagt in der
//! [`Auskunft`](krk_core::leseprofil::Auskunft), welche der beiden es ist,
//! und [`laden`] verzweigt darueber vollstaendig; die Zeilen reisen an
//! [`Inhalt::Metadaten`] strukturiert mit und werden erst in der Ansicht zu
//! Text, an derselben Stelle wie die sechs Angaben. Eine Verknuepfung
//! bekommt keine Zaehlzeile, und eine Datei erst recht nicht (C1.6, C1.7):
//! beides entscheidet der Kern, nicht dieses Modul.
//!
//! **Sie entsteht auf demselben Arbeitsfaden wie das Lesen einer Textdatei,
//! und aus demselben Grund.** Eine Zusammenfassung kostet bis zu zwoelf
//! Verzeichnisleselaeufe und bis zu vierundzwanzig Dateioeffnungen
//! ([`krk_core::leseprofil::HOECHSTENS_LESELAEUFE`],
//! [`krk_core::leseprofil::HOECHSTENS_OEFFNUNGEN`]) — also mehr Platte als
//! jede einzelne Textdatei dieser Anzeige. Auf dem Hauptfaden gerechnet ginge
//! L7 auf Kosten von L1, und genau das verhindert der Faden aus dem Abschnitt
//! `# Der Arbeitsfaden` seit der Runde 1. Ein zweiter Faden neben ihm entsteht
//! nicht: die Zusammenfassung ist Teil dessen, was `laden` fuer **einen**
//! ausgewaehlten Eintrag liefert, und sie hat kein anderes Ende als er.
//!
//! Daraus folgt C4.7 ohne eine eigene Vorkehrung: [`laden`] ist der eine
//! Aufrufer von [`krk_core::leseprofil::zusammenfassen`] im ausgelieferten
//! Programm, und
//! [`laden`] laeuft allein auf dem Faden, den
//! [`Vorschaumodell::datei_anzeigen`] fuer den ausgewaehlten Eintrag startet.
//! Ein Ordner, den der Nutzer nie auswaehlt, wird damit nie zusammengefasst.
//!
//! # Gelesen wird ueber den Deskriptor und nicht ueber den Pfad
//!
//! Der eine Weg von einem Pfad zu den Bytes ist
//! [`krk_core::text::datei::bis_zur_grenze_lesen`](krk_core::text::datei::bis_zur_grenze_lesen),
//! und er oeffnet ueber
//! [`krk_core::verzeichnis::sys::ohne_warten_oeffnen`](krk_core::verzeichnis::sys::ohne_warten_oeffnen)
//! — dasselbe Stueck, das
//! [`krk_core::text::datei::oeffnen`](krk_core::text::datei::oeffnen) fuer den
//! Editor nimmt. Ein eigener Oeffnungsweg der Vorschau entsteht damit nicht:
//! es ist derselbe Eingang mit einer anderen Grenze, und die beiden Grenzen
//! sind der eine Unterschied zwischen Ansehen und Bearbeiten (siehe
//! [`TEXTGRENZE`]).
//!
//! **Die Huelle wohnt seit der Runde 11 in `krk-core` und nicht mehr hier.**
//! Bis dahin stand ihr Rumpf in dieser Datei, und er stand damit an einer
//! Stelle, die der Inhaltsfilter der Dateiliste nicht erreicht; der Rumpf ist
//! unveraendert umgezogen. Die Vorschau uebersetzt jedes
//! [`Lesehindernis`](krk_core::text::datei::Lesehindernis) in ihre
//! Metadatenanzeige — vier Gruende, eine Antwort, siehe [`laden`] — und ist
//! damit der eine Aufrufer, den die Unterscheidung der vier Gruende nichts
//! angeht.
//!
//! **Zwei Fragen stehen hier nebeneinander, und sie sind verschieden.** Wer sie
//! zusammenzieht, hat entweder eine Verknuepfung, die als ihr Ziel erscheint,
//! oder eine Roehre, die gelesen wird:
//!
//! - **Was die Vorschau anzeigt**, entscheidet [`typ_von`] am `lstat(2)` des
//!   Pfades. Drei Zweige, denn eine Verknuepfung erscheint als sie selbst und
//!   nicht als das, worauf sie zeigt.
//! - **Ob sich etwas lesen laesst**, entscheidet `fstat(2)` am Deskriptor in
//!   der Huelle aus `krk-core`. Eine benannte Roehre, ein Zeichengeraet, ein
//!   Blockgeraet und ein Socket sind fuer [`typ_von`] `Typ::Datei`, melden alle
//!   `st_size == 0` und kaemen damit durch jede Groessenschranke; heraus fallen
//!   sie am Typ des Deskriptors.
//!
//! Bis zum 260810 stand an beiden Lesestellen `std::fs::read(pfad)`, also
//! `File::open` plus `read_to_end`. Auf einer benannten Roehre ohne Schreiber
//! blieb der Faden `krk-vorschau` fuer die Lebensdauer des Programms im `open`
//! stehen, einer je beruehrter Roehre, und auf `/dev/zero` wuchs der Puffer
//! ohne Grenze. Der Defekt dazu ist `260810-1247`; sein Gegenstueck am Editor
//! ist `260809-1652`, und dort steht die ausfuehrliche Begruendung der
//! Reihenfolge.
//!
//! # Der Arbeitsfaden
//!
//! [`Vorschaumodell::datei_anzeigen`] kehrt sofort zurueck: das Lesen der
//! Vorschaudatei laeuft je Anfrage auf einem eigenen Faden, damit L7 nicht auf
//! Kosten von L1 geht. Der Faden schickt genau eine Meldung ueber einen Kanal
//! und endet. Der [`Ladevorgang`] wohnt **im Tab**, so wie der `Lesevorgang`
//! aus [`crate::tabs`] im Tabinhalt wohnt: eine neue Anfrage an denselben Tab
//! laesst den alten Empfaenger fallen, das `send` des ueberholten Fadens
//! scheitert still, und eine Generationspruefung braucht es nicht. Die
//! Zwischenablage liegt im Arbeitsspeicher und braucht keinen Faden.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;
use std::time::SystemTime;

use krk_core::leseprofil::{Auskunft, Profile, Zusammenfassungszeile, zusammenfassen};
use krk_core::text::datei::bis_zur_grenze_lesen;
use krk_core::verzeichnis::Typ;

use crate::editormodell::Dateityp;
use crate::hervorhebung::{self, Darstellungsart, Tafel};
use crate::markdown::{self, Gerendert};

/// Bis zu welcher Groesse eine Textdatei als Inhalt erscheint (C6).
pub const TEXTGRENZE: u64 = 1024 * 1024;

/// Bis zu welcher Groesse eine Bilddatei als Bild erscheint (C6).
///
/// **Warum 64 MB und nicht die Textgrenze.** Die Zahl trennt die gaengigen
/// Bildformate, die C6 mit ihrem Inhalt zusagt, von den Ausreissern, die den
/// Speicher fuellen. Ein Bildschirmfoto eines Retina-Schirms liegt bei wenigen
/// MB, ein Foto aus einer Kamera als JPEG unter 20 MB, ein HEIC darunter; ein
/// TIFF- oder PSD-Export dagegen leicht ueber 100 MB. Mit der Textgrenze von
/// 1 MB fiele ein gewoehnliches Foto aus der Anzeige, und das Abnahmekriterium
/// von C6 waere gebrochen; ohne jede Grenze wird eine beliebig grosse Datei
/// gelesen, und genau das war der Defekt.
pub const BILDGRENZE: u64 = 64 * 1024 * 1024;

/// Die Bildgrenze liegt ueber der Textgrenze, sonst fiele jedes gewoehnliche
/// Foto aus der Anzeige, die C6 zusagt. Beim Uebersetzen geprueft und nicht
/// erst beim Pruefen: die Aussage haengt allein an den beiden Zahlen darueber.
const _: () = assert!(BILDGRENZE > TEXTGRENZE);

/// Der Editor nimmt mehr an als die Vorschau, und genau das war der Grund fuer
/// seine eigene Zahl (`krk_core::text::datei::EDITORGRENZE`, 16 MB).
///
/// **Die Zusicherung steht hier, weil nur hier beide Zahlen benennbar sind.**
/// `krk-core` kennt `krk-ui` nicht — die Abhaengigkeit laeuft allein in die
/// andere Richtung —, und die Zusicherung dort vergleicht deshalb gegen die
/// 1 MB als Zahl statt als Bezug. Sie faengt damit ein **Absenken** von
/// `EDITORGRENZE`; ein **Anheben** von [`TEXTGRENZE`] ueber 16 MB faengt allein
/// diese hier. Zusammen sind es beide Richtungen, und die halbe drueben bleibt
/// stehen: sie schuetzt `krk-core` fuer sich genommen und kostet nichts. Der
/// Defekt, der die fehlende Haelfte gemeldet hat, ist `260809-1610`.
const _: () = assert!(krk_core::text::datei::EDITORGRENZE > TEXTGRENZE);

/// Die Dateiendungen, die als gaengige Bildformate gelten (C6).
///
/// Verglichen ohne Ruecksicht auf Gross- und Kleinschreibung. Die Liste nennt,
/// was `NSImage` auf jedem macOS dieser Runde liest; ein Format, das die
/// Dekodierung dann doch nicht nimmt, faellt in der Ansicht auf die Metadaten
/// zurueck, die jede [`Inhalt::Bild`]-Meldung dafuer mitfuehrt.
const BILDENDUNGEN: [&str; 10] = [
    "png", "jpg", "jpeg", "gif", "tif", "tiff", "heic", "heif", "bmp", "icns",
];

/// Die Metadaten eines Eintrags, wie C6 sie fuer alles Uebrige verlangt.
///
/// Fuenf der sechs Angaben kennt auch `Eintrag` aus S2; die Rechte kommen
/// allein hier vor und werden erst beim Anzeigen erhoben (siehe Modulkopf).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadaten {
    /// Der Name ohne Pfad.
    pub name: String,
    /// Der vollstaendige Pfad.
    pub pfad: PathBuf,
    /// Die Groesse der Daten in Bytes. Fuer einen Ordner ohne Aussage,
    /// die Ansicht zeigt dort `--` wie die Groessenspalte aus C1.
    pub groesse: u64,
    /// Der Zeitpunkt der letzten Aenderung.
    pub geaendert: SystemTime,
    /// Die Zugriffsrechte als Unix-Modus, wie `stat(2)` sie liefert.
    pub rechte: u32,
    /// Ordner, Datei oder symbolische Verknuepfung.
    pub typ: Typ,
}

/// Was ein Vorschau-Tab zeigt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inhalt {
    /// Noch nichts: der Tab ist neu und keine Quelle hat ihn gefuellt.
    ///
    /// Die Ansicht zeigt dafuer einen Satz und keine leere Flaeche, aus
    /// demselben Grund, aus dem C10 das fuer die leere Zwischenablage
    /// verlangt.
    Leer,
    /// Reiner Text: eine Textdatei bis 1 MB oder Text aus der Zwischenablage.
    ///
    /// Markdown aus einer **Datei** steht seit der Runde 6 nicht mehr hier,
    /// sondern in [`Inhalt::Markdown`]. Aus der Zwischenablage kommt es
    /// weiterhin als Text: dort gibt es keinen Pfad, an dem eine Endung
    /// haengt, und `hervorhebung::art` entscheidet die Darstellung nach dem
    /// Pfad.
    Text(String),
    /// Gerendertes Markdown aus einer Datei: der Text ohne seine
    /// Auszeichnungszeichen und die Stellen, die eine Auszeichnung tragen
    /// (C4 der Runde 6).
    ///
    /// **In einem `Box`, weil dieser Wert die uebrigen sonst aufbliese.** Ein
    /// [`Gerendert`] traegt eine `String` und drei Listen; [`Inhalt`] wird bei
    /// jedem Neuzeichnen des aktiven Tabs geklont, und jeder Wert der
    /// Aufzaehlung ist so gross wie der groesste.
    Markdown(Box<Gerendert>),
    /// Ein Bild, als rohe Daten eines Formats, das `NSImage` liest.
    ///
    /// Die Metadaten fahren mit, damit die Ansicht bei einer Datei, deren
    /// Dekodierung scheitert, auf sie zurueckfallen kann, ohne ein zweites
    /// Mal zu lesen. Fuer ein Bild aus der Zwischenablage sind sie leer.
    Bild {
        /// Die Bytes der Bilddatei oder der Zwischenablage.
        ///
        /// **Geteilt und nicht kopiert.** Die Ansicht klont den [`Inhalt`] des
        /// aktiven Tabs bei jedem Neuzeichnen, um die Ausleihe des Modells vor
        /// dem ersten Objective-C-Aufruf zu beenden; ein blosser `Vec<u8>`
        /// legte dabei jedes Mal eine zweite Kopie der ganzen Bilddatei an.
        /// `Arc` macht denselben Klon zu einem Zaehlerschritt. Er und nicht
        /// `Rc`, weil der Arbeitsfaden den Wert baut und durch einen Kanal
        /// schickt.
        daten: Arc<Vec<u8>>,
        /// Die Metadaten der Datei, falls das Bild aus einer kommt.
        metadaten: Option<Metadaten>,
    },
    /// Die Metadaten: alles, was weder Text noch Bild ist, auch Ordner (C6).
    ///
    /// **Seit der Runde 19 fahren die Zaehlzeilen des eingebauten
    /// Default-Profils mit**, und die leere Folge heisst „keine Zaehlzeilen".
    /// Ein Ordner ohne Profiltreffer bekommt die drei Zeilen des
    /// [`defaultprofil`](krk_core::leseprofil::defaultprofil), eine Datei
    /// und eine Verknuepfung bekommen keine (C1.6, C1.7). Ein achter Wert
    /// von [`Inhalt`] waere der teurere Weg und der schlechtere: jede
    /// vollstaendige Fallunterscheidung ueber `Inhalt` — die Nummernspalte,
    /// die Einfaerbung, die Anzeige — muesste eine Frage beantworten, deren
    /// Antwort ausnahmslos „wie bei den Metadaten" lautet. Der Unterschied
    /// besteht an genau einer Stelle, beim Bauen des Textes in der Ansicht,
    /// und dort steht er als Zweiteilung ueber die leere und die nicht leere
    /// Folge.
    Metadaten {
        /// Die sechs Angaben aus C6.
        metadaten: Metadaten,
        /// Die drei Zeilen des eingebauten Default-Profils, oder keine.
        zaehlzeilen: Vec<Zusammenfassungszeile>,
    },
    /// Die Zeilen eines Leseprofils fuer einen **erkannten** Ordner (C4 der
    /// Runde 16).
    ///
    /// **Er ersetzt die Metadaten und tritt nicht neben sie.** Ein Ordner, den
    /// ein Profil aus `readers.toml` an seinem Pfad oder an einer
    /// Kennzeichendatei darin erkennt, zeigt statt Name, Pfad, Groesse,
    /// Aenderungsdatum, Rechten und Typ die Zeilen seines Profils. Ohne
    /// Treffer bleibt es bei [`Inhalt::Metadaten`] mit allen sechs Angaben
    /// (C2.5), und eine **Datei** erreicht diesen Wert nie (C2.6).
    ///
    /// **Die Kopfzeile aus Festlegung A6 steckt in ihm** und nicht daneben:
    /// [`Zusammenfassung::name`](krk_core::leseprofil::Zusammenfassung::name)
    /// und [`Zusammenfassung::pfad`](krk_core::leseprofil::Zusammenfassung::pfad)
    /// sind die eine Auskunft der Metadatenanzeige, die die Ersetzung
    /// ueberlebt. Deshalb faehrt hier kein [`Metadaten`] als Rueckfall mit, wie
    /// es [`Inhalt::Bild`] tut: es gibt keinen zweiten Weg, auf den die Ansicht
    /// zurueckfallen koennte, und die eine Angabe, die sie braucht, hat sie.
    ///
    /// Der Wert wandert **strukturiert** bis in die Ansicht und wird erst dort
    /// zu Text; der Grund steht an seinem Typ im Kern.
    Zusammenfassung(krk_core::leseprofil::Zusammenfassung),
    /// Ein Satz an den Nutzer: die leere Zwischenablage, ein Lesefehler.
    Hinweis(String),
}

/// Was der Arbeitsfaden fuer einen Tab geladen hat.
#[derive(Debug)]
struct Geladen {
    inhalt: Inhalt,
}

/// Ein laufendes Laden einer Vorschaudatei.
///
/// Faellt der Vorgang, faellt sein Empfaenger, und das `send` des Fadens
/// scheitert still; siehe den Modulkopf.
#[derive(Debug)]
pub struct Ladevorgang {
    empfaenger: Receiver<Geladen>,
    /// Welche Datei geladen wird. Beim Eintreffen der Meldung wird sie zum
    /// angezeigten Pfad des Tabs.
    pfad: PathBuf,
}

impl Ladevorgang {
    /// Startet den Arbeitsfaden fuer den genannten Pfad.
    ///
    /// Die Tafel faehrt mit, weil das Rendern von Markdown auf diesem Faden
    /// laeuft und die Farbe eines Verweises aus ihr kommt; siehe [`laden`].
    ///
    /// **Die Profile fahren als `Arc` mit und nicht als Kopie**, aus demselben
    /// Grund, aus dem [`Inhalt::Bild`] seine Bytes teilt: der Satz wird bei
    /// jeder Auswahl an einen neuen Faden gereicht, er traegt je Profil einen
    /// uebersetzten regulaeren Ausdruck, und ein Klon legte jedes Mal eine
    /// zweite Fassung aller Profile an. `Arc` macht denselben Klon zu einem
    /// Zaehlerschritt. Er und nicht `Rc`, weil der Wert diesen Faden erreicht.
    fn starten(pfad: PathBuf, tafel: Tafel, profile: Arc<Profile>) -> Self {
        // Tiefe 1 genuegt: der Faden schickt genau eine Meldung.
        let (sender, empfaenger) = sync_channel(1);
        let fuer_faden = pfad.clone();
        let ergebnis = thread::Builder::new()
            .name("krk-vorschau".to_owned())
            .spawn(move || {
                let _ = SyncSender::send(
                    &sender,
                    Geladen {
                        inhalt: laden(&fuer_faden, tafel, &profile),
                    },
                );
            });
        if let Err(fehler) = ergebnis {
            // Ohne Faden kommt nie eine Meldung; der Kanal ist zu diesem
            // Zeitpunkt schon wieder ohne Sender, und `einziehen` raeumt den
            // Vorgang beim naechsten Takt ab. Der Hinweis hier ist die
            // einzige Spur, die der Fall hinterlaesst.
            eprintln!("krk: der Vorschau-Arbeitsfaden liess sich nicht starten: {fehler}");
        }
        Self { empfaenger, pfad }
    }
}

/// Ein Tab des Vorschaufensters.
#[derive(Debug)]
struct Vorschautab {
    /// Die Beschriftung in der Tableiste.
    titel: String,
    /// Was der Tab zeigt.
    inhalt: Inhalt,
    /// Welche Datei [`Vorschautab::inhalt`] zeigt; `None` fuer alles, was
    /// keine Datei ist, etwa die Zwischenablage oder den leeren Tab.
    ///
    /// Der Pfad wechselt erst, wenn der Arbeitsfaden geliefert hat — genau
    /// wie der Inhalt. Waehrend eines Ladens sagt er also weiterhin, was auf
    /// dem Schirm steht, und die Endbedingung von L7 fragt genau das.
    pfad: Option<PathBuf>,
    /// Das laufende Laden, falls eines laeuft.
    ladevorgang: Option<Ladevorgang>,
}

impl Vorschautab {
    fn leer() -> Self {
        Self {
            titel: "Leer".to_owned(),
            inhalt: Inhalt::Leer,
            pfad: None,
            ladevorgang: None,
        }
    }
}

/// Die Tabs des Vorschaufensters (C6).
///
/// Dieselben Regeln wie die [`Tabliste`](crate::tabs::Tabliste) eines
/// Dateifensters: nie weniger als ein Tab, ein neuer oeffnet hinter dem
/// aktiven, der naechste und der vorige laufen um.
#[derive(Debug)]
pub struct Vorschaumodell {
    tabs: Vec<Vorschautab>,
    aktiv: usize,
}

impl Default for Vorschaumodell {
    fn default() -> Self {
        Self::neu()
    }
}

impl Vorschaumodell {
    /// Ein Vorschaufenster mit einem leeren Tab.
    pub fn neu() -> Self {
        Self {
            tabs: vec![Vorschautab::leer()],
            aktiv: 0,
        }
    }

    /// Wie viele Tabs es gibt. Nie null.
    ///
    /// Heute allein von den Pruefungen gelesen; die Ansicht kommt ueber
    /// [`Vorschaumodell::titel`] an dieselbe Zahl.
    #[cfg(test)]
    pub fn zahl(&self) -> usize {
        self.tabs.len()
    }

    /// Die Stelle des aktiven Tabs.
    pub fn aktive_stelle(&self) -> usize {
        self.aktiv
    }

    /// Was der aktive Tab zeigt.
    pub fn aktiver_inhalt(&self) -> &Inhalt {
        &self.tabs[self.aktiv].inhalt
    }

    /// Die Beschriftungen aller Tabs, in der Reihenfolge der Leiste.
    pub fn titel(&self) -> Vec<String> {
        self.tabs.iter().map(|tab| tab.titel.clone()).collect()
    }

    /// Oeffnet einen neuen, leeren Tab hinter dem aktiven (C1 wie C6).
    pub fn oeffnen(&mut self) {
        let stelle = self.aktiv + 1;
        self.tabs.insert(stelle, Vorschautab::leer());
        self.aktiv = stelle;
    }

    /// Schliesst den aktiven Tab.
    ///
    /// Beim letzten Tab bleibt das Vorschaufenster stehen und zeigt wieder
    /// einen leeren Tab, wie C1 es fuer die Dateifenster verlangt. Liefert,
    /// ob sich etwas geaendert hat.
    pub fn schliessen(&mut self) -> bool {
        if self.tabs.len() == 1 {
            if self.tabs[0].inhalt == Inhalt::Leer && self.tabs[0].ladevorgang.is_none() {
                return false;
            }
            self.tabs[0] = Vorschautab::leer();
            return true;
        }
        self.tabs.remove(self.aktiv);
        if self.aktiv >= self.tabs.len() {
            self.aktiv = self.tabs.len() - 1;
        }
        true
    }

    /// Wechselt zum naechsten Tab und laeuft am Ende auf den ersten um.
    pub fn naechster(&mut self) -> bool {
        self.waehlen((self.aktiv + 1) % self.tabs.len())
    }

    /// Wechselt zum vorigen Tab und laeuft am Anfang auf den letzten um.
    pub fn voriger(&mut self) -> bool {
        self.waehlen((self.aktiv + self.tabs.len() - 1) % self.tabs.len())
    }

    /// Wechselt auf den Tab an der genannten Stelle.
    ///
    /// Eine Stelle ausserhalb der Liste und die des aktiven Tabs werden
    /// uebergangen. Der Inhalt des verlassenen Tabs bleibt unveraendert
    /// stehen; genau das ist das Halteverhalten aus dem Modulkopf.
    pub fn waehlen(&mut self, stelle: usize) -> bool {
        if stelle >= self.tabs.len() || stelle == self.aktiv {
            return false;
        }
        self.aktiv = stelle;
        true
    }

    /// Zeigt den genannten Eintrag im aktiven Tab (C6).
    ///
    /// Kehrt sofort zurueck; das Lesen laeuft auf dem Arbeitsfaden aus dem
    /// Modulkopf. Bis die Meldung eintrifft, steht der bisherige Inhalt, der
    /// Titel wechselt sofort: der Nutzer sieht damit, dass seine Auswahl
    /// angekommen ist, ohne dass eine halbgelesene Anzeige aufblitzt.
    ///
    /// Die Profile gehen an den Faden weiter und werden dort nur gelesen; ein
    /// leerer Satz heisst „keine Profile" und ist kein Fehlerfall, dann zeigt
    /// ein Ordner seine Metadaten. Warum sie als `Arc` und nicht als Kopie
    /// reisen, steht an [`Ladevorgang::starten`].
    pub fn datei_anzeigen(&mut self, pfad: &Path, tafel: Tafel, profile: Arc<Profile>) {
        let tab = &mut self.tabs[self.aktiv];
        tab.titel = titel_von(pfad);
        tab.ladevorgang = Some(Ladevorgang::starten(pfad.to_path_buf(), tafel, profile));
    }

    /// Zeigt den Inhalt der Zwischenablage im aktiven Tab (C10).
    ///
    /// Ohne Arbeitsfaden: die Zwischenablage liegt im Arbeitsspeicher. Ein
    /// noch laufendes Laden des Tabs faellt, denn die Zwischenablage ist die
    /// neuere Quelle.
    pub fn zwischenablage_anzeigen(&mut self, inhalt: Zwischenablageinhalt) {
        let tab = &mut self.tabs[self.aktiv];
        tab.titel = "Zwischenablage".to_owned();
        tab.ladevorgang = None;
        tab.pfad = None;
        tab.inhalt = match inhalt {
            Zwischenablageinhalt::Text(text) => Inhalt::Text(text),
            Zwischenablageinhalt::Bild(daten) => Inhalt::Bild {
                daten: Arc::new(daten),
                metadaten: None,
            },
            Zwischenablageinhalt::BildZuGross(groesse) => Inhalt::Hinweis(zu_gross_text(groesse)),
            Zwischenablageinhalt::Leer => {
                Inhalt::Hinweis("Die Zwischenablage ist leer.".to_owned())
            }
        };
    }

    /// Ob irgendein Tab noch auf seinen Arbeitsfaden wartet.
    pub fn laedt_noch(&self) -> bool {
        self.tabs.iter().any(|tab| tab.ladevorgang.is_some())
    }

    /// Welche Datei der aktive Tab zeigt; `None`, wenn keine Datei.
    ///
    /// Nur zum Ablesen, fuer die Endbedingung von L7 im Messmodus.
    pub fn aktiver_pfad(&self) -> Option<PathBuf> {
        self.tabs[self.aktiv].pfad.clone()
    }

    /// Ob der aktive Tab den rohen Inhalt einer **Datei** zeigt (C10).
    ///
    /// Die eine Stelle, die entscheidet, ob die Nummernspalte in der Vorschau
    /// steht. Sie ist wahr allein fuer [`Inhalt::Text`], und auch dort nur,
    /// wenn der Tab einen Pfad hat: derselbe Wert traegt nach seinem eigenen
    /// Doc-Kommentar auch den Text aus der Zwischenablage, und das dritte
    /// Abnahmekriterium von C10 nimmt ihn ausdruecklich aus. Ein Text ohne
    /// Datei hat keine Dateizeilen, die zu nummerieren waeren.
    ///
    /// **Gerendertes Markdown traegt keine Nummern** (C4, neuntes Kriterium
    /// der Runde 6), und der Grund ist nicht Platz, sondern Wahrheit: die
    /// Zahlen zaehlten die Zeilen des gerenderten Textes, und das sind andere
    /// als die der Datei, die danebensteht. Eine Zahl, die etwas anderes zaehlt
    /// als das, was neben ihr steht, ist eine falsche Auskunft.
    ///
    /// **Die Fallunterscheidung ist vollstaendig und hat keinen
    /// Auffangzweig**, wie die uebrigen dieser Art im Programm: ein siebter
    /// Inhalt haelt den Bau an und erzwingt die Antwort auf die Frage, ob
    /// neben ihm Zeilennummern stehen.
    pub fn zeigt_dateitext(&self) -> bool {
        match self.aktiver_inhalt() {
            Inhalt::Text(_) => self.aktiver_pfad().is_some(),
            // **Eine Zusammenfassung traegt keine Nummern**, aus demselben
            // Grund wie gerendertes Markdown darueber: die Zahlen zaehlten die
            // Zeilen der Zusammenfassung, und daneben steht keine Datei mit
            // diesen Zeilen. Der Tab zeigt einen Ordner, und ein Ordner hat
            // keine Zeilen; die Zeilen daneben sind die des Profils.
            Inhalt::Zusammenfassung(_) => false,
            Inhalt::Leer
            | Inhalt::Markdown(_)
            | Inhalt::Bild { .. }
            | Inhalt::Metadaten { .. }
            | Inhalt::Hinweis(_) => false,
        }
    }

    /// Holt die wartenden Meldungen aller Tabs ab.
    ///
    /// Liefert, ob sich der **aktive** Tab dabei geaendert hat; nur dann muss
    /// die Ansicht neu zeichnen. Ein inaktiver Tab fuellt sich still, wie die
    /// verdeckten Tabs eines Dateifensters.
    pub fn einziehen(&mut self) -> bool {
        let mut aktiver_geaendert = false;
        for (stelle, tab) in self.tabs.iter_mut().enumerate() {
            let Some(vorgang) = tab.ladevorgang.as_ref() else {
                continue;
            };
            let geladener_pfad = vorgang.pfad.clone();
            match vorgang.empfaenger.try_recv() {
                Ok(geladen) => {
                    tab.inhalt = geladen.inhalt;
                    tab.pfad = Some(geladener_pfad);
                    tab.ladevorgang = None;
                    if stelle == self.aktiv {
                        aktiver_geaendert = true;
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                // Der Faden ist ohne Meldung gefallen; darauf zu warten hat
                // keinen Sinn mehr.
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    tab.ladevorgang = None;
                }
            }
        }
        aktiver_geaendert
    }
}

/// Die Beschriftung eines Tabs, der den genannten Pfad zeigt.
///
/// Der letzte Namensteil, fuer die Wurzel der Pfad selbst; dieselbe Regel wie
/// bei den Tabs eines Dateifensters.
fn titel_von(pfad: &Path) -> String {
    match pfad.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => pfad.to_string_lossy().into_owned(),
    }
}

/// Was in der Zwischenablage lag, aus der Sicht der Vorschau (C10).
///
/// Die Dreiteilung aus C10: Text als Text, ein Bild als Bild, und eine leere
/// Zwischenablage sagt das ausdruecklich. Gelesen wird sie in
/// `appkit/zwischenablage.rs`, der einen Huelle um `NSPasteboard`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zwischenablageinhalt {
    /// Text, einschliesslich eines Dateiverweises als `file:`-Zeichenkette.
    Text(String),
    /// Die rohen Daten eines Bildes in einem Format, das `NSImage` liest.
    Bild(Vec<u8>),
    /// Ein Bild ueber [`BILDGRENZE`], mit seiner Groesse in Bytes.
    ///
    /// **Ein eigener Fall und keine Variante von [`Leer`].** Die
    /// Zwischenablage traegt ja ein Bild; nur die Vorschau zeigt es nicht.
    /// Wer beides in einen Fall zoege, koennte dem Nutzer den Unterschied
    /// zwischen "nichts kopiert" und "zu gross" nicht sagen.
    ///
    /// [`Leer`]: Zwischenablageinhalt::Leer
    BildZuGross(u64),
    /// Weder Text noch Bild.
    Leer,
}

/// Der Satz, den ein Bild ueber der Bildgrenze statt seiner selbst zeigt.
///
/// Beide Zahlen kommen aus je einer Quelle: die Groesse aus dem `NSData` der
/// Zwischenablage, die Grenze aus [`BILDGRENZE`]. Eine zweite Zahl im Text
/// entsteht nicht, und wer die Konstante aendert, aendert den Satz mit.
fn zu_gross_text(groesse: u64) -> String {
    let in_mb = |bytes: u64| bytes / (1024 * 1024);
    format!(
        "Das Bild in der Zwischenablage ist {} MB groß. Die Vorschau zeigt Bilder bis {} MB.",
        in_mb(groesse),
        in_mb(BILDGRENZE)
    )
}

/// Liest den Eintrag und ordnet ihn in die Dreiteilung aus C6 ein.
///
/// Innerhalb des Textes entscheidet danach `hervorhebung::art` ueber die drei
/// Wege der Anzeige aus C4 der Runde 6; die Tafel geht dabei allein in die
/// Farbe eines Verweises.
///
/// Laeuft auf dem Arbeitsfaden. Der `lstat(2)` hier ist die eine Stelle, die
/// die Rechte erhebt; siehe den Modulkopf.
///
/// **Jeder Grund, aus dem sich der Inhalt nicht zeigen laesst, endet in den
/// Metadaten.** Zu gross, keine gewoehnliche Datei, nicht lesbar, kein UTF-8:
/// die Metadaten sind fuer alle vier die Antwort, und sie waren es schon fuer
/// die letzten drei. Deshalb steht die Groessenschranke nicht mehr als eigener
/// Zweig hier, sondern in
/// [`krk_core::text::datei::bis_zur_grenze_lesen`](krk_core::text::datei::bis_zur_grenze_lesen)
/// neben den anderen Gruenden; vier Wege zu derselben Antwort brauchen keine
/// vier Verzweigungen.
///
/// **Der eine Aufrufer von [`krk_core::leseprofil::zusammenfassen`] im
/// ausgelieferten Programm** (C4.7). „Im Baum" waere zu weit gegriffen: die
/// Proben in `krk-core/tests` rufen ihn zehnmal, und keiner dieser Rufe laeuft
/// in einem Vorschaufenster. Gezaehlt wird deshalb `crates/krk-ui`, und die
/// Zaehlprobe sagt das ausdruecklich. Weil er hier steht und diese Funktion allein auf dem
/// Arbeitsfaden eines ausgewaehlten Eintrags laeuft, kostet ein Ordner, den der
/// Nutzer nie auswaehlt, keinen Verzeichnisleselauf und keine Dateioeffnung.
fn laden(pfad: &Path, tafel: Tafel, profile: &Profile) -> Inhalt {
    // `symlink_metadata`, damit eine Verknuepfung als sie selbst erscheint
    // und nicht als ihr Ziel: der Leser aus S2 folgt ihr auch nicht.
    let roh = match std::fs::symlink_metadata(pfad) {
        Ok(roh) => roh,
        Err(fehler) => {
            return Inhalt::Hinweis(format!(
                "{} liess sich nicht lesen: {fehler}",
                pfad.display()
            ));
        }
    };
    let metadaten = Metadaten {
        name: titel_von(pfad),
        pfad: pfad.to_path_buf(),
        groesse: roh.len(),
        geaendert: roh.modified().unwrap_or(SystemTime::UNIX_EPOCH),
        rechte: modus_von(&roh),
        typ: typ_von(&roh),
    };
    if metadaten.typ != Typ::Datei {
        // Der Zusammenfassungszweig steht **vor** dem Rueckgabezweig fuer
        // Ordner und Verknuepfungen, und die Reihenfolge traegt beide Zusagen
        // der Runde 16 auf einmal: gefragt wird allein hier drin, also greift
        // kein Profil auf eine Datei (C2.6), und ohne Treffer faellt der Weg
        // in denselben Zweig zurueck, der vor der Runde der einzige war —
        // Metadaten mit allen sechs Angaben und kein zweiter Zweig daneben
        // (C2.5).
        //
        // Seit der Runde 19 hat der Kern drei Ausgaenge, und die Verzweigung
        // ist vollstaendig ohne Auffangzweig: die Zusammenfassung eines
        // erkannten Ordners ersetzt die Metadaten, die Zeilen des
        // Default-Profils treten unter sie (C2.1), und eine Verknuepfung
        // oder ein Ordner, den der Kern nicht lesen kann, behaelt die sechs
        // Angaben allein (C1.7). Welcher Ausgang es ist, entscheidet der Kern
        // und nicht dieser Zweig; deshalb faellt hier nirgends ein Ordner
        // ohne Zaehlzeilen durch, den der Kern zaehlen wollte.
        return match zusammenfassen(profile, pfad) {
            Some(Auskunft::Erkannt(zusammenfassung)) => Inhalt::Zusammenfassung(zusammenfassung),
            Some(Auskunft::Default(zaehlzeilen)) => Inhalt::Metadaten {
                metadaten,
                zaehlzeilen,
            },
            // Ordner und Verknuepfungen erscheinen als Metadaten (C6).
            None => Inhalt::Metadaten {
                metadaten,
                zaehlzeilen: Vec::new(),
            },
        };
    }
    if ist_bildpfad(pfad) {
        // Ueber der Bildgrenze wird nicht gelesen, sondern beschrieben; dieselbe
        // Antwort wie beim Text und aus demselben Grund.
        return match bis_zur_grenze_lesen(pfad, BILDGRENZE) {
            Ok(daten) => Inhalt::Bild {
                daten: Arc::new(daten),
                metadaten: Some(metadaten),
            },
            // Jeder der vier Gruende endet in den Metadaten; welcher es war,
            // fragt die Vorschau nicht.
            Err(_) => Inhalt::Metadaten {
                metadaten,
                zaehlzeilen: Vec::new(),
            },
        };
    }
    // Eine Textdatei ueber 1 MB faellt auf die Metadaten, siehe den Modulkopf.
    match bis_zur_grenze_lesen(pfad, TEXTGRENZE)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
    {
        // Die drei Wege der Anzeige entscheidet die **eine** vorhandene Stelle
        // (C4, fuenfzehntes Kriterium der Runde 6): Markdown wird gerendert,
        // was die Syntaxkiste als Sprache kennt und alles Uebrige bleibt der
        // Text, der dasteht. Eine zweite Endungsliste neben `Dateityp` und
        // `hervorhebung::art` entsteht hier nicht.
        //
        // Eingefaerbt wird der Quelltext **nicht** hier: das gehoert hinter die
        // Endbedingung von L7 und damit in die Ansicht, denn `syntect` ist mit
        // 0,3 MB/s zu langsam, um darauf zu warten.
        Some(text) => match hervorhebung::art(Some(pfad), Dateityp::von_pfad(pfad)) {
            Darstellungsart::Markdown => {
                Inhalt::Markdown(Box::new(markdown::rendern(&text, tafel)))
            }
            Darstellungsart::Code | Darstellungsart::EinfacherText => Inhalt::Text(text),
        },
        // Zu gross, nicht lesbar, keine gewoehnliche Datei oder kein UTF-8, also
        // keine Textdatei im Sinne von C6.
        None => Inhalt::Metadaten {
            metadaten,
            zaehlzeilen: Vec::new(),
        },
    }
}

/// Ob der Pfad auf eines der gaengigen Bildformate endet.
fn ist_bildpfad(pfad: &Path) -> bool {
    pfad.extension()
        .map(|endung| endung.to_string_lossy().to_ascii_lowercase())
        .is_some_and(|endung| BILDENDUNGEN.contains(&endung.as_str()))
}

/// Der Unix-Modus des Eintrags.
#[cfg(unix)]
fn modus_von(roh: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    roh.permissions().mode()
}

/// Die Eintragsart, in der Sprache von S2.
fn typ_von(roh: &std::fs::Metadata) -> Typ {
    let art = roh.file_type();
    if art.is_symlink() {
        Typ::Verknuepfung
    } else if art.is_dir() {
        Typ::Ordner
    } else {
        Typ::Datei
    }
}

/// Die Rechte in der Schreibweise von `ls -l`, ohne das fuehrende Typzeichen.
///
/// Reines Rust und deshalb hier und nicht in der Ansicht: die Zeile ist ohne
/// Fenster pruefbar. Die Sonderbits erscheinen wie bei `ls`: setuid und
/// setgid als `s` beziehungsweise `S` auf der Ausfuehrstelle, das Sticky-Bit
/// als `t` beziehungsweise `T`.
pub fn rechte_text(modus: u32) -> String {
    let mut zeichen = String::with_capacity(9);
    let gruppen = [
        (modus >> 6 & 0o7, modus & 0o4000 != 0, 's'),
        (modus >> 3 & 0o7, modus & 0o2000 != 0, 's'),
        (modus & 0o7, modus & 0o1000 != 0, 't'),
    ];
    for (bits, sonderbit, sonderzeichen) in gruppen {
        zeichen.push(if bits & 0o4 != 0 { 'r' } else { '-' });
        zeichen.push(if bits & 0o2 != 0 { 'w' } else { '-' });
        let ausfuehrbar = bits & 0o1 != 0;
        zeichen.push(match (sonderbit, ausfuehrbar) {
            (true, true) => sonderzeichen,
            (true, false) => sonderzeichen.to_ascii_uppercase(),
            (false, true) => 'x',
            (false, false) => '-',
        });
    }
    zeichen
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::pruefordner::Pruefordner;

    /// Fuellt den aktiven Tab ohne Arbeitsfaden.
    fn text_zeigen(modell: &mut Vorschaumodell, text: &str) {
        modell.zwischenablage_anzeigen(Zwischenablageinhalt::Text(text.to_owned()));
    }

    #[test]
    fn ein_neues_vorschaufenster_hat_einen_leeren_tab() {
        let modell = Vorschaumodell::neu();
        assert_eq!(modell.zahl(), 1);
        assert_eq!(*modell.aktiver_inhalt(), Inhalt::Leer);
    }

    /// Das Halteverhalten aus dem Zustandsdiagramm des Specs: ein inaktiver
    /// Tab behaelt seinen Inhalt, und die Rueckkehr zeigt ihn unveraendert.
    #[test]
    fn ein_inaktiver_tab_behaelt_seinen_inhalt() {
        let mut modell = Vorschaumodell::neu();
        text_zeigen(&mut modell, "erster Inhalt");
        modell.oeffnen();
        text_zeigen(&mut modell, "zweiter Inhalt");
        assert!(modell.waehlen(0));
        assert_eq!(
            *modell.aktiver_inhalt(),
            Inhalt::Text("erster Inhalt".to_owned()),
            "die Rueckkehr zeigt genau den Inhalt beim Verlassen"
        );
        // Eine neue Quelle ersetzt den Inhalt des aktiven Tabs, der inaktive
        // bleibt stehen.
        text_zeigen(&mut modell, "dritter Inhalt");
        assert!(modell.waehlen(1));
        assert_eq!(
            *modell.aktiver_inhalt(),
            Inhalt::Text("zweiter Inhalt".to_owned())
        );
    }

    #[test]
    fn der_naechste_und_der_vorige_tab_laufen_um() {
        let mut modell = Vorschaumodell::neu();
        modell.oeffnen();
        modell.oeffnen();
        assert_eq!(modell.aktive_stelle(), 2);
        assert!(modell.naechster());
        assert_eq!(
            modell.aktive_stelle(),
            0,
            "der letzte laeuft auf den ersten"
        );
        assert!(modell.voriger());
        assert_eq!(
            modell.aktive_stelle(),
            2,
            "der erste laeuft auf den letzten"
        );
    }

    #[test]
    fn das_schliessen_des_letzten_tabs_laesst_das_fenster_stehen() {
        let mut modell = Vorschaumodell::neu();
        text_zeigen(&mut modell, "Inhalt");
        assert!(modell.schliessen());
        assert_eq!(modell.zahl(), 1, "das Vorschaufenster bleibt bestehen");
        assert_eq!(*modell.aktiver_inhalt(), Inhalt::Leer);
        assert!(
            !modell.schliessen(),
            "ein leerer letzter Tab aendert nichts"
        );
    }

    #[test]
    fn das_schliessen_ruecht_die_aktive_stelle_nach() {
        let mut modell = Vorschaumodell::neu();
        modell.oeffnen();
        modell.oeffnen();
        assert_eq!(modell.aktive_stelle(), 2);
        assert!(modell.schliessen());
        assert_eq!(modell.zahl(), 2);
        assert_eq!(modell.aktive_stelle(), 1, "der letzte Tab war aktiv");
    }

    #[test]
    fn die_leere_zwischenablage_sagt_das_ausdruecklich() {
        let mut modell = Vorschaumodell::neu();
        modell.zwischenablage_anzeigen(Zwischenablageinhalt::Leer);
        assert_eq!(
            *modell.aktiver_inhalt(),
            Inhalt::Hinweis("Die Zwischenablage ist leer.".to_owned()),
            "C10: keine leere Flaeche"
        );
        assert_eq!(modell.titel()[0], "Zwischenablage");
    }

    /// Ein Bild ueber der Grenze wird benannt und nicht gezeigt.
    ///
    /// Geprueft wird die Zuordnung im Modell; dass die Groesse vor dem Kopieren
    /// erhoben wird, entscheidet `appkit/zwischenablage.rs`, wo kein Pruefcode
    /// hinkommt.
    #[test]
    fn ein_bild_ueber_der_bildgrenze_erscheint_als_hinweis() {
        let mut modell = Vorschaumodell::neu();
        modell.zwischenablage_anzeigen(Zwischenablageinhalt::BildZuGross(100 * 1024 * 1024));
        assert_eq!(
            *modell.aktiver_inhalt(),
            Inhalt::Hinweis(
                "Das Bild in der Zwischenablage ist 100 MB groß. \
                 Die Vorschau zeigt Bilder bis 64 MB."
                    .to_owned()
            ),
            "C10: der Nutzer erfaehrt, warum sein Bild fehlt"
        );
        assert_eq!(modell.titel()[0], "Zwischenablage");
    }

    #[test]
    fn eine_textdatei_erscheint_mit_ihrem_inhalt() {
        let ordner = Pruefordner::neu("text");
        let pfad = ordner.pfad().join("notiz.txt");
        std::fs::write(&pfad, "Erste Zeile\nZweite").expect("Probendatei");
        assert_eq!(
            laden(&pfad, Tafel::Hell, &Profile::default()),
            Inhalt::Text("Erste Zeile\nZweite".to_owned())
        );
    }

    /// Die drei Wege der Anzeige aus C4 der Runde 6, an drei Dateien.
    ///
    /// Entschieden werden sie von der **einen** vorhandenen Stelle,
    /// `hervorhebung::art`; die Probe misst, dass `laden` sie fragt und keine
    /// zweite Endungsliste fuehrt.
    #[test]
    fn die_drei_wege_der_anzeige_haengen_an_der_endung() {
        let ordner = Pruefordner::neu("wege");

        let markdown = ordner.pfad().join("notiz.md");
        std::fs::write(&markdown, "# Ueberschrift\n").expect("Probendatei");
        let Inhalt::Markdown(gerendert) = laden(&markdown, Tafel::Hell, &Profile::default()) else {
            panic!("eine .md-Datei wird gerendert");
        };
        assert_eq!(
            gerendert.text, "Ueberschrift",
            "das Doppelkreuz ist verschwunden"
        );

        // Quelltext bleibt der Text, der dasteht; eingefaerbt wird er erst in
        // der Ansicht, hinter der Endbedingung von L7.
        let quelltext = ordner.pfad().join("quelle.rs");
        std::fs::write(&quelltext, "fn main() {}\n").expect("Probendatei");
        assert_eq!(
            laden(&quelltext, Tafel::Hell, &Profile::default()),
            Inhalt::Text("fn main() {}\n".to_owned())
        );

        // Das dreizehnte Abnahmekriterium von C4: lokale HTML-Dateien bleiben
        // Quelltext und werden nicht gerendert.
        let html = ordner.pfad().join("seite.html");
        std::fs::write(&html, "<p>Hallo</p>\n").expect("Probendatei");
        assert_eq!(
            laden(&html, Tafel::Hell, &Profile::default()),
            Inhalt::Text("<p>Hallo</p>\n".to_owned())
        );
    }

    #[test]
    fn ein_ordner_erscheint_als_metadaten() {
        let ordner = Pruefordner::neu("ordner");
        let Inhalt::Metadaten { metadaten, .. } =
            laden(ordner.pfad(), Tafel::Hell, &Profile::default())
        else {
            panic!("ein Ordner gehoert in die Metadatenanzeige");
        };
        assert_eq!(metadaten.typ, Typ::Ordner);
        assert_eq!(metadaten.pfad, ordner.pfad());
    }

    /// Die Abnahmelage des Schritts: eine grosse Textdatei blockiert nichts
    /// und faellt auf die Metadaten.
    #[test]
    fn eine_textdatei_ueber_der_grenze_faellt_auf_die_metadaten() {
        let ordner = Pruefordner::neu("gross");
        let pfad = ordner.pfad().join("gross.txt");
        std::fs::write(&pfad, "a".repeat((TEXTGRENZE + 1) as usize)).expect("Probendatei");
        let Inhalt::Metadaten { metadaten, .. } = laden(&pfad, Tafel::Hell, &Profile::default())
        else {
            panic!("ueber der Grenze zeigen die Metadaten");
        };
        assert_eq!(metadaten.groesse, TEXTGRENZE + 1);
        assert_eq!(metadaten.typ, Typ::Datei);
    }

    /// Ein Bild unter der Grenze erscheint mit seinen Bytes. Die Datei ist
    /// kein gueltiges PNG; `laden` entscheidet nach der Endung, und die
    /// Dekodierung ist Sache der Ansicht.
    #[test]
    fn ein_bild_unter_der_grenze_kommt_mit_seinen_bytes() {
        let ordner = Pruefordner::neu("bild-klein");
        let pfad = ordner.pfad().join("bild.png");
        std::fs::write(&pfad, [0x89, b'P', b'N', b'G']).expect("Probendatei");
        let Inhalt::Bild { daten, metadaten } = laden(&pfad, Tafel::Hell, &Profile::default())
        else {
            panic!("unter der Grenze zeigt das Bild");
        };
        assert_eq!(*daten, vec![0x89, b'P', b'N', b'G']);
        assert!(
            metadaten.is_some(),
            "die Metadaten fahren als Rueckfall mit"
        );
    }

    /// Die Abnahmelage der Bildgrenze: eine Bilddatei darueber wird gar nicht
    /// erst gelesen und faellt auf die Metadaten.
    ///
    /// Die Probendatei entsteht ueber `set_len` und belegt deshalb keine
    /// 64 MB auf der Platte; `laden` fragt vor der Grenze allein `stat(2)`,
    /// und der liefert die gesetzte Laenge.
    #[test]
    fn ein_bild_ueber_der_grenze_faellt_auf_die_metadaten() {
        let ordner = Pruefordner::neu("bild-gross");
        let pfad = ordner.pfad().join("gross.tiff");
        let datei = std::fs::File::create(&pfad).expect("Probendatei");
        datei.set_len(BILDGRENZE + 1).expect("Laenge setzen");
        drop(datei);
        let Inhalt::Metadaten { metadaten, .. } = laden(&pfad, Tafel::Hell, &Profile::default())
        else {
            panic!("ueber der Grenze zeigen die Metadaten");
        };
        assert_eq!(metadaten.groesse, BILDGRENZE + 1);
        assert_eq!(metadaten.typ, Typ::Datei);
    }

    #[test]
    fn keine_utf8_datei_faellt_auf_die_metadaten() {
        let ordner = Pruefordner::neu("binaer");
        let pfad = ordner.pfad().join("roh.bin");
        std::fs::write(&pfad, [0xFF, 0xFE, 0x00, 0x42]).expect("Probendatei");
        assert!(matches!(
            laden(&pfad, Tafel::Hell, &Profile::default()),
            Inhalt::Metadaten { .. }
        ));
    }

    #[test]
    fn ein_fehlender_pfad_liefert_einen_hinweis() {
        let pfad = Path::new("/gibt/es/nicht/krk-probe");
        assert!(matches!(
            laden(pfad, Tafel::Hell, &Profile::default()),
            Inhalt::Hinweis(_)
        ));
    }

    /// Ruft [`laden`] auf einem eigenen Faden und gibt die Antwort nur heraus,
    /// wenn sie innerhalb der Schranke kommt.
    ///
    /// **Der Grund ist die Art des Defekts, der hier geprueft wird:** ein
    /// blockierendes `open` liefert kein falsches Ergebnis, sondern gar keines.
    /// Ohne Schranke waere das ein stehender Probelauf, und `cargo test` haette
    /// nichts zu melden als Stillstand. Mit ihr gibt es einen Befund mit Namen.
    /// Dieselbe Bauform wie `oeffnen_mit_zeitschranke` in
    /// `krk-core/tests/text.rs`, wo das Gegenstueck am Editor geprueft wird.
    ///
    /// Der Faden bleibt im Fehlerfall stehen, wo er steht. Er stirbt mit dem
    /// Probelauf, und ein Deskriptor, der nie aufgeht, haelt nichts fest.
    fn laden_mit_zeitschranke(pfad: &Path, schranke: Duration) -> Inhalt {
        let (sender, empfaenger) = std::sync::mpsc::channel();
        let pfad = pfad.to_path_buf();
        thread::spawn(move || {
            let _ = sender.send(laden(&pfad, Tafel::Hell, &Profile::default()));
        });
        empfaenger.recv_timeout(schranke).unwrap_or_else(|_| {
            panic!("laden ist nach {schranke:?} nicht zurueckgekommen; das Oeffnen haengt")
        })
    }

    /// Eine benannte Roehre haelt den Arbeitsfaden der Vorschau nicht an.
    ///
    /// Die Roehre hat keinen Schreiber, und ein `File::open` darauf wartet, bis
    /// jemand hineinschreibt — hier also fuer immer. Genau das tat die Vorschau
    /// bis zum 260810 mit ihrem `std::fs::read(pfad)`, und der Faden
    /// `krk-vorschau` blieb fuer die Lebensdauer des Programms stehen, einer je
    /// beruehrter Roehre. Der Defekt dazu ist `260810-1247`.
    ///
    /// Die groessere der beiden Aussagen ist, dass ueberhaupt eine Antwort
    /// kommt; sie haengt am `O_NONBLOCK` in
    /// `krk_core::verzeichnis::sys::ohne_warten_oeffnen`. Die kleinere ist, dass
    /// die Antwort die Metadaten sind: eine Roehre traegt keinen Inhalt, den die
    /// Vorschau zeigen koennte.
    ///
    /// **Das Zeichengeraet aus demselben Defekt faellt an derselben Zeile
    /// heraus** (`!angaben.is_file()`), und dafuer steht hier absichtlich keine
    /// zweite Probe: `/dev/zero` liefert ohne Ende, ein `read_to_end` darauf
    /// waere vor der Behebung kein Befund, sondern ein volllaufender
    /// Arbeitsspeicher auf dem Geraet dessen, der die Behebung zuruecknimmt.
    #[test]
    fn eine_benannte_roehre_haelt_die_vorschau_nicht_an() {
        let ordner = Pruefordner::neu("roehre");
        let roehre = ordner.roehre("ohne-schreiber");

        let inhalt = laden_mit_zeitschranke(&roehre, Duration::from_secs(5));

        let Inhalt::Metadaten { metadaten, .. } = &inhalt else {
            panic!("die Roehre gehoert in die Metadatenanzeige: {inhalt:?}");
        };
        assert_eq!(metadaten.pfad, roehre);
    }

    #[test]
    fn die_bildendungen_greifen_ohne_ruecksicht_auf_schreibung() {
        assert!(ist_bildpfad(Path::new("/a/bild.PNG")));
        assert!(ist_bildpfad(Path::new("/a/bild.jpeg")));
        assert!(!ist_bildpfad(Path::new("/a/notiz.md")));
        assert!(!ist_bildpfad(Path::new("/a/ohne-endung")));
    }

    #[test]
    fn die_rechte_erscheinen_in_der_schreibweise_von_ls() {
        assert_eq!(rechte_text(0o755), "rwxr-xr-x");
        assert_eq!(rechte_text(0o644), "rw-r--r--");
        assert_eq!(rechte_text(0o000), "---------");
        assert_eq!(rechte_text(0o4755), "rwsr-xr-x");
        assert_eq!(rechte_text(0o4644), "rwSr--r--");
        assert_eq!(rechte_text(0o1777), "rwxrwxrwt");
        assert_eq!(rechte_text(0o1776), "rwxrwxrwT");
    }

    /// Der Weg ueber den Arbeitsfaden: die Meldung kommt an, und sie kommt im
    /// richtigen Tab an, auch wenn der Nutzer inzwischen gewechselt hat.
    #[test]
    fn das_laden_erreicht_den_tab_der_es_bestellt_hat() {
        let ordner = Pruefordner::neu("faden");
        let pfad = ordner.pfad().join("inhalt.txt");
        std::fs::write(&pfad, "aus dem Faden").expect("Probendatei");

        let mut modell = Vorschaumodell::neu();
        modell.datei_anzeigen(&pfad, Tafel::Hell, Arc::default());
        modell.oeffnen();
        // Der bestellende Tab ist jetzt inaktiv; die Meldung gehoert trotzdem
        // ihm.
        while modell.laedt_noch() {
            let aktiver_geaendert = modell.einziehen();
            assert!(
                !aktiver_geaendert,
                "die Meldung gehoert dem inaktiven Tab, nicht dem aktiven"
            );
            std::thread::yield_now();
        }
        assert!(modell.waehlen(0));
        assert_eq!(
            *modell.aktiver_inhalt(),
            Inhalt::Text("aus dem Faden".to_owned())
        );
        assert_eq!(modell.titel()[0], "inhalt.txt");
    }

    /// Setzt Inhalt und Pfad des aktiven Tabs unmittelbar.
    ///
    /// Die beiden gewoehnlichen Wege dorthin, [`Vorschaumodell::datei_anzeigen`]
    /// und [`Vorschaumodell::zwischenablage_anzeigen`], erreichen zusammen nicht
    /// alle sechs Werte von [`Inhalt`]: die Metadaten entstehen nur aus einer
    /// Datei, die keine Textdatei ist, das Bild nur aus einer lesbaren
    /// Bilddatei. Die Probe unten deckt die Fallunterscheidung vollstaendig ab
    /// und setzt deshalb hier an.
    fn tab_setzen(inhalt: Inhalt, pfad: Option<&str>) -> Vorschaumodell {
        let mut modell = Vorschaumodell::neu();
        modell.tabs[0].inhalt = inhalt;
        modell.tabs[0].pfad = pfad.map(PathBuf::from);
        modell
    }

    fn probenmetadaten() -> Metadaten {
        Metadaten {
            name: "probe.txt".to_owned(),
            pfad: PathBuf::from("/tmp/probe.txt"),
            groesse: 12,
            geaendert: SystemTime::UNIX_EPOCH,
            rechte: 0o644,
            typ: Typ::Datei,
        }
    }

    /// Das zweite und das dritte Abnahmekriterium von C10: die Nummern stehen
    /// beim rohen Inhalt einer Textdatei und sonst nirgends in der Vorschau.
    #[test]
    fn allein_der_text_einer_datei_traegt_zeilennummern() {
        assert!(
            tab_setzen(
                Inhalt::Text("eins\nzwei".to_owned()),
                Some("/tmp/probe.txt")
            )
            .zeigt_dateitext(),
            "der rohe Inhalt einer Textdatei"
        );
        assert!(
            !tab_setzen(Inhalt::Text("aus der Zwischenablage".to_owned()), None).zeigt_dateitext(),
            "Text ohne Datei: die Zwischenablage aus C10 der Runde 1"
        );
        assert!(!tab_setzen(Inhalt::Leer, None).zeigt_dateitext());
        assert!(
            !tab_setzen(
                Inhalt::Markdown(Box::new(crate::markdown::rendern("# Titel\n", Tafel::Hell))),
                Some("/tmp/probe.md"),
            )
            .zeigt_dateitext(),
            "C4 der Runde 6: neben gerendertem Markdown steht keine Zahl"
        );
        assert!(
            !tab_setzen(Inhalt::Hinweis("leer".to_owned()), None).zeigt_dateitext(),
            "ein Hinweis hat keine Dateizeilen"
        );
        assert!(
            !tab_setzen(
                Inhalt::Metadaten {
                    metadaten: probenmetadaten(),
                    zaehlzeilen: Vec::new()
                },
                Some("/tmp/probe.bin")
            )
            .zeigt_dateitext(),
            "Metadaten stehen fuer eine Datei, sind aber nicht ihr Inhalt"
        );
        assert!(
            !tab_setzen(
                Inhalt::Bild {
                    daten: Arc::new(vec![0, 1, 2]),
                    metadaten: Some(probenmetadaten()),
                },
                Some("/tmp/probe.png"),
            )
            .zeigt_dateitext(),
            "ein Bild hat keine Zeilen"
        );
        assert!(
            !tab_setzen(
                Inhalt::Zusammenfassung(krk_core::leseprofil::Zusammenfassung::neu(
                    "werkbank".to_owned(),
                    PathBuf::from("/tmp/probe/werkbank"),
                    Vec::new(),
                )),
                Some("/tmp/probe/werkbank"),
            )
            .zeigt_dateitext(),
            "die Zahlen zaehlten die Zeilen der Zusammenfassung, und daneben \
             steht keine Datei mit diesen Zeilen"
        );
    }

    // -----------------------------------------------------------------------
    // Die Zusammenfassung eines erkannten Ordners (Runde 16)
    // -----------------------------------------------------------------------

    /// Ein geprueftes Profilbuendel aus einem TOML-Text, wie die Ablage es
    /// beim Start liefert.
    ///
    /// Die Proben hier bauen ihre Profile aus Text und nicht aus
    /// [`krk_core::leseprofil::Profil::neu`], damit sie denselben Weg nehmen
    /// wie der Nutzer mit seiner `readers.toml`; der Fall der beschaedigten
    /// Datei gehoert der Ablage und wird dort abgenommen.
    fn profile_aus(text: &str) -> Profile {
        let datei: krk_core::leseprofil::datei::Profildatei = toml::from_str(text)
            .unwrap_or_else(|fehler| panic!("der Probentext ist kein lesbares TOML: {fehler}"));
        let (profile, meldungen) = krk_core::leseprofil::datei::pruefen(datei);
        assert!(meldungen.is_empty(), "unerwartete Meldungen: {meldungen:?}");
        profile
    }

    /// C2.5: ein Ordner ohne Treffer zeigt seine Metadaten mit allen sechs
    /// Angaben, unveraendert gegenueber dem Stand vor der Runde 16.
    ///
    /// Geprueft wird gegen ein Profilbuendel, das **nicht leer** ist: ein
    /// leeres liefe an der Erkennung ohnehin vorbei und sagte nichts darueber,
    /// was ein Profil tut, das seinen Ort nicht findet.
    #[test]
    fn ein_ordner_ohne_treffer_zeigt_weiter_alle_sechs_metadatenangaben() {
        let ordner = Pruefordner::neu("ohne-treffer");
        let profile = profile_aus(
            r#"
[[profil]]
name = "Eine Werkbank"
pfad = 'fusion-workbench$'
kennzeichen = '^\.fusion-setup$'

  [[profil.zeile]]
  beschriftung = "Datensaetze"
  zaehlung = { muster = '\.md$' }
"#,
        );

        let Inhalt::Metadaten { metadaten, .. } = laden(ordner.pfad(), Tafel::Hell, &profile)
        else {
            panic!("ohne Treffer bleibt es beim Zweig von vor der Runde");
        };
        assert_eq!(metadaten.name, titel_von(ordner.pfad()));
        assert_eq!(metadaten.pfad, ordner.pfad());
        assert_eq!(metadaten.typ, Typ::Ordner);
        assert_ne!(
            metadaten.geaendert,
            SystemTime::UNIX_EPOCH,
            "das Aenderungsdatum ist erhoben und nicht der Rueckfallwert"
        );
        assert_ne!(metadaten.rechte, 0, "die Rechte sind erhoben");
        // Die Groesse eines Ordners hat nach dem Doc-Kommentar von `Metadaten`
        // keine Aussage; abgelesen wird sie trotzdem und steht damit da.
        let _ = metadaten.groesse;
    }

    /// C1.1, C2.1 und C2.2 der Runde 19: ein Ordner mit leerem Profilsatz
    /// zeigt seine sechs Angaben und darunter die drei Zaehlzeilen.
    ///
    /// Geprueft wird die Uebersetzung der [`Auskunft`] in einen [`Inhalt`]:
    /// die Zeilen reisen strukturiert mit und tragen ihre Beschriftungen in
    /// der Reihenfolge aus Festlegung A1; was sie zaehlen, halten die Proben
    /// im Kern. Die Metadaten daneben bleiben die eines Ordners.
    #[test]
    fn ein_ordner_mit_leerem_profilsatz_traegt_drei_zaehlzeilen_unter_seinen_metadaten() {
        let ordner = Pruefordner::neu("zaehlzeilen");
        ordner.datei("eins.md", "a");
        ordner.datei(".zwei", "b");
        ordner.ordner("drei");

        let Inhalt::Metadaten {
            metadaten,
            zaehlzeilen,
        } = laden(ordner.pfad(), Tafel::Hell, &Profile::default())
        else {
            panic!("ein Ordner ohne Profiltreffer bleibt bei den Metadaten");
        };
        assert_eq!(metadaten.typ, Typ::Ordner);
        assert_eq!(metadaten.pfad, ordner.pfad());
        assert_eq!(
            zaehlzeilen
                .iter()
                .map(|zeile| (zeile.beschriftung(), zeile.wert().clone()))
                .collect::<Vec<_>>(),
            [
                (
                    "Dateien",
                    krk_core::leseprofil::Wert::ZahlMitVersteckten {
                        zahl: 2,
                        versteckt: 1
                    }
                ),
                (
                    "Ordner",
                    krk_core::leseprofil::Wert::ZahlMitVersteckten {
                        zahl: 1,
                        versteckt: 0
                    }
                ),
                (
                    "Verknüpfungen",
                    krk_core::leseprofil::Wert::ZahlMitVersteckten {
                        zahl: 0,
                        versteckt: 0
                    }
                ),
            ],
            "die drei Zaehlzeilen kommen nicht in der Gestalt an, die der Kern liefert"
        );
    }

    /// C1.6 und C1.7 der Runde 19: eine Verknuepfung bekommt keine Zaehlzeile,
    /// auch wenn sie auf einen Ordner zeigt, und eine Datei erst recht nicht.
    ///
    /// Die Verknuepfung zeigt auf denselben Ordner, der in der Probe darueber
    /// drei Zeilen bekommt; dass hier keine dasteht, liegt damit am
    /// ausgewaehlten Eintrag und nicht an seinem Ziel (Festlegung A4).
    #[test]
    fn eine_verknuepfung_und_eine_datei_tragen_keine_zaehlzeile() {
        let ordner = Pruefordner::neu("keine-zaehlzeilen");
        let ziel = ordner.ordner("ziel");
        std::fs::write(ziel.join("eins.md"), "a").expect("Probendatei");
        let verknuepfung = ordner.unter("auf-ziel");
        std::os::unix::fs::symlink(&ziel, &verknuepfung)
            .expect("die Verknuepfung laesst sich nicht anlegen");
        let binaer = ordner.datei("roh.bin", [0xFF, 0xFE, 0x00, 0x42]);

        for (pfad, typ) in [(&verknuepfung, Typ::Verknuepfung), (&binaer, Typ::Datei)] {
            let Inhalt::Metadaten {
                metadaten,
                zaehlzeilen,
            } = laden(pfad, Tafel::Hell, &Profile::default())
            else {
                panic!("{} gehoert in die Metadatenanzeige", pfad.display());
            };
            assert_eq!(metadaten.typ, typ);
            assert!(
                zaehlzeilen.is_empty(),
                "{} traegt Zaehlzeilen: {zaehlzeilen:?}",
                pfad.display()
            );
        }

        // Die Gegenprobe am Ziel: der Ordner selbst bekommt seine drei Zeilen.
        let Inhalt::Metadaten { zaehlzeilen, .. } = laden(&ziel, Tafel::Hell, &Profile::default())
        else {
            panic!("das Ziel der Verknuepfung gehoert in die Metadatenanzeige");
        };
        assert_eq!(zaehlzeilen.len(), 3, "das Ziel bekommt keine drei Zeilen");
    }

    /// Ein Ordner, den ein Profil erkennt, zeigt dessen Zeilen statt der
    /// Metadaten, samt der Kopfzeile aus Festlegung A6.
    #[test]
    fn ein_erkannter_ordner_zeigt_die_zeilen_seines_profils() {
        let ordner = Pruefordner::neu("mit-treffer");
        let werkbank = ordner.ordner("werkbank");
        std::fs::write(werkbank.join("eins.md"), "a").expect("Probendatei");
        std::fs::write(werkbank.join("zwei.md"), "b").expect("Probendatei");
        std::fs::write(werkbank.join("drei.txt"), "c").expect("Probendatei");

        let profile = profile_aus(
            r#"
[[profil]]
name = "Eine Werkbank"
pfad = 'werkbank$'

  [[profil.zeile]]
  beschriftung = "Datensaetze"
  zaehlung = { muster = '\.md$' }
"#,
        );

        let Inhalt::Zusammenfassung(zusammenfassung) = laden(&werkbank, Tafel::Hell, &profile)
        else {
            panic!("ein erkannter Ordner zeigt die Zeilen seines Profils");
        };
        // Die Kopfzeile aus A6: die eine Auskunft der Metadaten, die die
        // Ersetzung ueberlebt.
        assert_eq!(zusammenfassung.name(), "werkbank");
        assert_eq!(zusammenfassung.pfad(), &werkbank);
        assert_eq!(zusammenfassung.zeilen().len(), 1);
        assert_eq!(zusammenfassung.zeilen()[0].beschriftung(), "Datensaetze");
        assert_eq!(
            zusammenfassung.zeilen()[0].wert(),
            &krk_core::leseprofil::Wert::Zahl(2),
            "gezaehlt werden die zwei .md-Dateien und nicht die dritte daneben"
        );
    }

    /// C2.6: kein Profil greift auf eine Datei, auch dann nicht, wenn ihr Pfad
    /// ein Pfadmuster erfuellt.
    ///
    /// Das Muster `'werkbank'` trifft hier auf **jeden** der drei Pfade, denn
    /// alle drei liegen in einem Ordner dieses Namens. Waere die Frage nach dem
    /// Profil vor die Typunterscheidung gerutscht, saehe der Nutzer statt der
    /// Textdatei die Zeilen eines Profils.
    #[test]
    fn eine_datei_unter_einem_treffenden_pfadmuster_zeigt_weiter_ihren_inhalt() {
        let ordner = Pruefordner::neu("datei-unter-muster");
        let werkbank = ordner.ordner("werkbank");
        let text = werkbank.join("notiz.txt");
        std::fs::write(&text, "Erste Zeile\nZweite").expect("Probendatei");
        let bild = werkbank.join("bild.png");
        std::fs::write(&bild, [0x89, b'P', b'N', b'G']).expect("Probendatei");
        let binaer = werkbank.join("roh.bin");
        std::fs::write(&binaer, [0xFF, 0xFE, 0x00, 0x42]).expect("Probendatei");

        let profile = profile_aus(
            r#"
[[profil]]
name = "Eine Werkbank"
pfad = 'werkbank'

  [[profil.zeile]]
  beschriftung = "Datensaetze"
  zaehlung = { }
"#,
        );

        assert_eq!(
            laden(&text, Tafel::Hell, &profile),
            Inhalt::Text("Erste Zeile\nZweite".to_owned()),
            "eine Textdatei bis 1 MB zeigt weiter ihren Inhalt"
        );
        assert!(
            matches!(
                laden(&bild, Tafel::Hell, &profile),
                Inhalt::Bild {
                    metadaten: Some(_),
                    ..
                }
            ),
            "eine Bilddatei bis 64 MB zeigt weiter ihre Bytes"
        );
        assert!(
            matches!(
                laden(&binaer, Tafel::Hell, &profile),
                Inhalt::Metadaten { .. }
            ),
            "alles Uebrige zeigt weiter seine Metadaten"
        );
        // Die Gegenprobe am selben Buendel: der Ordner darum wird erkannt. Ohne
        // sie sagte die Probe darueber nur, dass das Muster nirgends trifft.
        assert!(
            matches!(
                laden(&werkbank, Tafel::Hell, &profile),
                Inhalt::Zusammenfassung(_)
            ),
            "dasselbe Muster trifft den Ordner sehr wohl"
        );
    }

    /// C4.7: die Zusammenfassung entsteht beim Auswaehlen und nicht im Voraus.
    ///
    /// Die Zusage ist eine Aussage ueber den **Baum** und an keinem
    /// Rueckgabewert abzulesen; geprueft wird sie deshalb an zwei Zaehlungen,
    /// die zusammen die Kette schliessen:
    ///
    /// 1. In `crates/krk-ui` wird [`krk_core::leseprofil::zusammenfassen`] an
    ///    genau einer Stelle gerufen, und die steht in dieser Datei. Gezaehlt
    ///    wird allein diese Kiste: `krk-core` erklaert die Funktion und prueft
    ///    sie in eigenen Proben ab, und beides sind keine Rufer der
    ///    Oberflaeche.
    /// 2. In dieser Datei ruft ausserhalb des Pruefmoduls genau eine Stelle
    ///    [`laden`], naemlich der Rumpf des Arbeitsfadens in
    ///    [`Ladevorgang::starten`]. Der Faden entsteht allein in
    ///    [`Vorschaumodell::datei_anzeigen`], also fuer einen ausgewaehlten
    ///    Eintrag.
    ///
    /// Zusammen: ein Ordner, den der Nutzer nie auswaehlt, erreicht
    /// `zusammenfassen` nicht und kostet damit keinen Verzeichnisleselauf und
    /// keine Dateioeffnung.
    ///
    /// **Die verbleibende Blindheit**, wie im Kopf von [`crate::quellbaum`]
    /// ausgeschrieben: ein Aufruf unter anderem Namen, also ein
    /// `use … as anders;`, entginge beiden Zaehlungen.
    #[test]
    fn zusammenfassen_hat_einen_rufer_und_der_haengt_am_arbeitsfaden() {
        let name = concat!("zusammen", "fassen");
        let rufer: Vec<(String, usize)> = crate::quellbaum::quelldateien()
            .into_iter()
            .filter(|(datei, _)| datei.starts_with("krk-ui/"))
            .map(|(datei, inhalt)| (datei, crate::quellbaum::aufrufstellen(&inhalt, name)))
            .filter(|(_, zahl)| *zahl > 0)
            .collect();
        assert_eq!(
            rufer,
            vec![("krk-ui/src/vorschaumodell.rs".to_owned(), 1)],
            "in krk-ui ruft genau eine Stelle {name}, und sie steht in dieser Datei"
        );

        let (_, diese_datei) = crate::quellbaum::quelldateien()
            .into_iter()
            .find(|(datei, _)| datei == "krk-ui/src/vorschaumodell.rs")
            .expect("diese Datei steht im Quellbaum");
        let (ohne_proben, _) = diese_datei
            .split_once("#[cfg(test)]")
            .expect("das Pruefmodul dieser Datei ist mit #[cfg(test)] angemeldet");
        assert_eq!(
            crate::quellbaum::aufrufstellen(ohne_proben, concat!("la", "den")),
            1,
            "ausserhalb des Pruefmoduls ruft genau der Arbeitsfaden laden"
        );
    }
}
