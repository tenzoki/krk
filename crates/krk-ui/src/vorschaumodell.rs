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
//! Textdateien bis 1 MB und Markdown erscheinen als reiner Inhalt, die
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

use krk_core::verzeichnis::Typ;

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
    /// Reiner Text: eine Textdatei bis 1 MB, Markdown, oder Text aus der
    /// Zwischenablage.
    Text(String),
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
    Metadaten(Metadaten),
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
    fn starten(pfad: PathBuf) -> Self {
        // Tiefe 1 genuegt: der Faden schickt genau eine Meldung.
        let (sender, empfaenger) = sync_channel(1);
        let fuer_faden = pfad.clone();
        let ergebnis = thread::Builder::new()
            .name("krk-vorschau".to_owned())
            .spawn(move || {
                let _ = SyncSender::send(
                    &sender,
                    Geladen {
                        inhalt: laden(&fuer_faden),
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
    pub fn datei_anzeigen(&mut self, pfad: &Path) {
        let tab = &mut self.tabs[self.aktiv];
        tab.titel = titel_von(pfad);
        tab.ladevorgang = Some(Ladevorgang::starten(pfad.to_path_buf()));
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
    /// **Die Fallunterscheidung ist vollstaendig und hat keinen
    /// Auffangzweig**, wie die uebrigen dieser Art im Programm: ein sechster
    /// Inhalt haelt den Bau an und erzwingt die Antwort auf die Frage, ob
    /// neben ihm Zeilennummern stehen.
    pub fn zeigt_dateitext(&self) -> bool {
        match self.aktiver_inhalt() {
            Inhalt::Text(_) => self.aktiver_pfad().is_some(),
            Inhalt::Leer | Inhalt::Bild { .. } | Inhalt::Metadaten(_) | Inhalt::Hinweis(_) => false,
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
/// Laeuft auf dem Arbeitsfaden. Der `stat(2)` hier ist die eine Stelle, die
/// die Rechte erhebt; siehe den Modulkopf.
fn laden(pfad: &Path) -> Inhalt {
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
        // Ordner und Verknuepfungen erscheinen als Metadaten (C6).
        return Inhalt::Metadaten(metadaten);
    }
    if ist_bildpfad(pfad) {
        // Ueber der Grenze wird nicht gelesen, sondern beschrieben; dieselbe
        // Antwort wie beim Text und aus demselben Grund. Die Pruefung steht
        // **vor** dem Lesen: danach waere die Datei schon im Speicher, und
        // genau das ist der Fall, den die Grenze verhindert.
        if metadaten.groesse > BILDGRENZE {
            return Inhalt::Metadaten(metadaten);
        }
        return match std::fs::read(pfad) {
            Ok(daten) => Inhalt::Bild {
                daten: Arc::new(daten),
                metadaten: Some(metadaten),
            },
            Err(_) => Inhalt::Metadaten(metadaten),
        };
    }
    if metadaten.groesse > TEXTGRENZE {
        // Eine Textdatei ueber 1 MB faellt auf die Metadaten, siehe den
        // Modulkopf.
        return Inhalt::Metadaten(metadaten);
    }
    match std::fs::read(pfad) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(text) => Inhalt::Text(text),
            // Kein UTF-8, also keine Textdatei im Sinne von C6.
            Err(_) => Inhalt::Metadaten(metadaten),
        },
        Err(_) => Inhalt::Metadaten(metadaten),
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
    use super::*;

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
        let ordner = std::env::temp_dir().join("krk-vorschau-probe-text");
        std::fs::create_dir_all(&ordner).expect("Probenordner");
        let pfad = ordner.join("notiz.md");
        std::fs::write(&pfad, "# Ueberschrift\nZeile").expect("Probendatei");
        assert_eq!(
            laden(&pfad),
            Inhalt::Text("# Ueberschrift\nZeile".to_owned())
        );
    }

    #[test]
    fn ein_ordner_erscheint_als_metadaten() {
        let ordner = std::env::temp_dir().join("krk-vorschau-probe-ordner");
        std::fs::create_dir_all(&ordner).expect("Probenordner");
        let Inhalt::Metadaten(metadaten) = laden(&ordner) else {
            panic!("ein Ordner gehoert in die Metadatenanzeige");
        };
        assert_eq!(metadaten.typ, Typ::Ordner);
        assert_eq!(metadaten.pfad, ordner);
    }

    /// Die Abnahmelage des Schritts: eine grosse Textdatei blockiert nichts
    /// und faellt auf die Metadaten.
    #[test]
    fn eine_textdatei_ueber_der_grenze_faellt_auf_die_metadaten() {
        let ordner = std::env::temp_dir().join("krk-vorschau-probe-gross");
        std::fs::create_dir_all(&ordner).expect("Probenordner");
        let pfad = ordner.join("gross.txt");
        std::fs::write(&pfad, "a".repeat((TEXTGRENZE + 1) as usize)).expect("Probendatei");
        let Inhalt::Metadaten(metadaten) = laden(&pfad) else {
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
        let ordner = std::env::temp_dir().join("krk-vorschau-probe-bild-klein");
        std::fs::create_dir_all(&ordner).expect("Probenordner");
        let pfad = ordner.join("bild.png");
        std::fs::write(&pfad, [0x89, b'P', b'N', b'G']).expect("Probendatei");
        let Inhalt::Bild { daten, metadaten } = laden(&pfad) else {
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
        let ordner = std::env::temp_dir().join("krk-vorschau-probe-bild-gross");
        std::fs::create_dir_all(&ordner).expect("Probenordner");
        let pfad = ordner.join("gross.tiff");
        let datei = std::fs::File::create(&pfad).expect("Probendatei");
        datei.set_len(BILDGRENZE + 1).expect("Laenge setzen");
        drop(datei);
        let Inhalt::Metadaten(metadaten) = laden(&pfad) else {
            panic!("ueber der Grenze zeigen die Metadaten");
        };
        assert_eq!(metadaten.groesse, BILDGRENZE + 1);
        assert_eq!(metadaten.typ, Typ::Datei);
    }

    #[test]
    fn keine_utf8_datei_faellt_auf_die_metadaten() {
        let ordner = std::env::temp_dir().join("krk-vorschau-probe-binaer");
        std::fs::create_dir_all(&ordner).expect("Probenordner");
        let pfad = ordner.join("roh.bin");
        std::fs::write(&pfad, [0xFF, 0xFE, 0x00, 0x42]).expect("Probendatei");
        assert!(matches!(laden(&pfad), Inhalt::Metadaten(_)));
    }

    #[test]
    fn ein_fehlender_pfad_liefert_einen_hinweis() {
        let pfad = Path::new("/gibt/es/nicht/krk-probe");
        assert!(matches!(laden(pfad), Inhalt::Hinweis(_)));
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
        let ordner = std::env::temp_dir().join("krk-vorschau-probe-faden");
        std::fs::create_dir_all(&ordner).expect("Probenordner");
        let pfad = ordner.join("inhalt.txt");
        std::fs::write(&pfad, "aus dem Faden").expect("Probendatei");

        let mut modell = Vorschaumodell::neu();
        modell.datei_anzeigen(&pfad);
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
    /// alle fuenf Werte von [`Inhalt`]: die Metadaten entstehen nur aus einer
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
            !tab_setzen(Inhalt::Hinweis("leer".to_owned()), None).zeigt_dateitext(),
            "ein Hinweis hat keine Dateizeilen"
        );
        assert!(
            !tab_setzen(Inhalt::Metadaten(probenmetadaten()), Some("/tmp/probe.bin"))
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
    }
}
