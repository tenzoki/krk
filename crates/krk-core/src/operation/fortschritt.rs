//! Fortschritt, Abbruch, uebersprungene Eintraege und die Rueckfrage bei einem
//! Konflikt.
//!
//! Der Hauptfaden fuehrt keine Dateisystem-Arbeit aus. Was hier steht, ist die
//! eine Verbindung zwischen ihm und dem Arbeitsfaden: ein [`AtomicBool`] in die
//! eine Richtung, ein Kanal in die andere.
//!
//! ```text
//!   Hauptfaden                            Arbeitsfaden
//!   ──────────                            ────────────
//!   Lauf::abbrechen      ──AtomicBool──>  Steuerung::abgebrochen
//!   Abbruchgriff::abbrechen ──┘ (dasselbe Kennzeichen)
//!   Lauf::meldungen      <───Kanal─────   Steuerung::{fortschritt, ueberspringen}
//!   Konfliktentscheid    ──Kanal──────>   Steuerung::konflikt_loesen (wartet)
//!                                         Steuerung::namen_erfragen  (wartet)
//! ```
//!
//! Die zwei Frager der letzten Zeile stellen dieselbe [`Meldung::Konflikt`]
//! ueber denselben privaten Weg und lesen die Antwort verschieden:
//! [`Steuerung::konflikt_loesen`] fragt unter der Konfliktregel des Laufs,
//! was mit einem vorhandenen Ziel geschieht; [`Steuerung::namen_erfragen`]
//! fragt ohne Regel nach einem anderen Namen und kennt keine Antwort, die
//! ersetzt.
//!
//! [`Abbruchgriff`] gibt es, weil der [`Lauf`] nicht zwischen zwei Faeden
//! geteilt werden kann; wer ihn einem Faden gibt und trotzdem abbrechen koennen
//! muss, nimmt den Griff. Die Begruendung steht dort.
//!
//! # Warum der Fortschritt getaktet gemeldet wird und die uebrigen Meldungen
//! nicht
//!
//! Eine Fortschrittsmeldung ist ohne Wert, sobald die naechste da ist: sie sagt
//! einen Zwischenstand, den der folgende ersetzt. Eine uebersprungene Position
//! und der Abschluss sind das Gegenteil, sie kommen genau einmal. Deshalb geht
//! der Fortschritt hoechstens alle [`MELDEABSTAND`] los und die uebrigen
//! Meldungen immer.
//!
//! Ohne diesen Takt haengt die Zahl der Meldungen an der Zahl der Eintraege:
//! eine Kopie von 100.000 Dateien schoebe 100.000 Meldungen in den Kanal,
//! obwohl der Hauptfaden hoechstens sechzig Bilder je Sekunde zeichnet. Mit dem
//! Takt haengt sie an der Laufzeit, und der Kanal bleibt klein, ohne dass
//! jemand ihn leeren muss.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, SyncSender, sync_channel};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use super::auftrag::Konfliktregel;
use super::umbenennen::freier_name;

/// Der kleinste Abstand zwischen zwei Fortschrittsmeldungen.
///
/// Acht Millisekunden sind eine halbe Bildlaenge bei 120 Hz. Wer schneller
/// meldet, meldet in ein Bild hinein, das ohnehin nicht neu gezeichnet wird.
pub const MELDEABSTAND: Duration = Duration::from_millis(8);

/// Wie eine Operation geendet hat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Abschluss {
    /// Alle Eintraege sind abgearbeitet. Einzelne koennen uebersprungen worden
    /// sein; das bricht den Stapel nicht ab.
    Fertig,
    /// Der Nutzer hat abgebrochen.
    Abgebrochen,
}

/// Ein Eintrag, an dem die Operation gescheitert ist.
///
/// Eine gescheiterte Einzelposition bricht den Stapel nicht ab (C4). Sie
/// sammelt Eintrag und Grund, und die uebrigen laufen durch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uebersprungen {
    /// Der Eintrag, um den es geht.
    pub pfad: PathBuf,
    /// Warum er nicht bearbeitet wurde, im Klartext.
    pub grund: String,
}

/// Ein Zwischenstand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fortschritt {
    /// Wie viele Eintraege fertig sind.
    pub eintraege: u64,
    /// Wie viele Bytes uebertragen sind, ueber alle Eintraege.
    pub bytes: u64,
    /// Der Eintrag, an dem gerade gearbeitet wird.
    pub eintrag: PathBuf,
}

/// Was der Nutzer bei einem Namenskonflikt gewaehlt hat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Konfliktantwort {
    /// Den vorhandenen Eintrag ersetzen.
    Ueberschreiben,
    /// Die Quelle auslassen.
    Ueberspringen,
    /// Unter dem genannten Namen ablegen. Der Name ist ein Name, kein Pfad.
    UmbenennenIn(String),
    /// Den ganzen Vorgang beenden.
    Abbrechen,
}

/// Die Antwort samt der Wahl "fuer alle weiteren uebernehmen" (C4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Konfliktentscheid {
    /// Was mit diesem Eintrag geschieht.
    pub antwort: Konfliktantwort,
    /// Ob dieselbe Wahl fuer jeden weiteren Konflikt dieses Vorgangs gilt.
    pub fuer_alle_weiteren: bool,
}

impl Konfliktentscheid {
    /// Eine Antwort, die nur fuer diesen einen Eintrag gilt.
    #[must_use]
    pub fn einmal(antwort: Konfliktantwort) -> Self {
        Self {
            antwort,
            fuer_alle_weiteren: false,
        }
    }

    /// Eine Antwort, die fuer jeden weiteren Konflikt dieses Vorgangs gilt.
    #[must_use]
    pub fn fuer_alle(antwort: Konfliktantwort) -> Self {
        Self {
            antwort,
            fuer_alle_weiteren: true,
        }
    }
}

/// Der Abschlussbericht einer Operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bericht {
    /// Wie der Vorgang geendet hat.
    pub abschluss: Abschluss,
    /// Wie viele Eintraege uebertragen wurden.
    pub eintraege: u64,
    /// Wie viele Bytes uebertragen wurden.
    ///
    /// Gemeint ist der Inhalt und nicht der Plattenverkehr: ein Klon bewegt
    /// keine Bytes, die Datei ist trotzdem angekommen, und der Nutzer will die
    /// Groesse seiner Datei sehen und nicht eine Null.
    pub bytes: u64,
    /// Die uebersprungenen Eintraege mit ihrem Grund.
    pub uebersprungen: Vec<Uebersprungen>,
}

/// Was der Arbeitsfaden an den Hauptfaden schickt.
#[derive(Debug)]
pub enum Meldung {
    /// Ein Zwischenstand, hoechstens alle [`MELDEABSTAND`].
    Fortschritt(Fortschritt),
    /// Ein Namenskonflikt. Der Arbeitsfaden wartet auf die Antwort ueber den
    /// mitgeschickten Kanal.
    Konflikt {
        /// Der Eintrag, der uebertragen werden soll.
        quelle: PathBuf,
        /// Der Eintrag, der schon da ist.
        ziel: PathBuf,
        /// Der Weg zurueck. Wird der Kanal fallen gelassen, gilt das als
        /// Abbruch: lieber nichts tun als ungefragt ueberschreiben.
        antwort: SyncSender<Konfliktentscheid>,
    },
    /// Ein Eintrag ist ausgelassen worden.
    Uebersprungen(Uebersprungen),
    /// Der Vorgang ist zu Ende. Kommt genau einmal und immer zuletzt.
    Fertig(Bericht),
}

/// Der Griff an das Abbruchkennzeichen eines Laufs, fuer sich allein.
///
/// **Der Grund, aus dem es ihn gibt.** Ein [`Lauf`] haelt drei Dinge zusammen:
/// das Abbruchkennzeichen, den Empfaenger des Meldekanals und den Faden. Ein
/// `Receiver` ist `Send`, aber nicht `Sync`, der Lauf laesst sich also an einen
/// anderen Faden **geben**, nicht zwischen zweien **teilen**. Wer in `recv`
/// wartet, muss ihn haben, und das darf nicht der Hauptfaden sein.
///
/// Ohne diesen Griff blieb dem Hauptfaden nur ein zweites Kennzeichen, das der
/// wartende Faden nach jeder Meldung abfragte und weiterreichte. Bei einer
/// Operation, die ueber Sekunden nichts meldet, wirkte der Abbruch entsprechend
/// spaet (`issues/260804-1816_*_der-abbruchwunsch-erreicht-den-lauf-erst-mit-der-naechsten-meldung.md`).
/// Das Kennzeichen selbst ist ein `AtomicBool` und damit sehr wohl teilbar; nur
/// herausgegeben wurde es nie.
///
/// Ein Griff ist kein zweiter Weg zum Abbruch, sondern derselbe: er zeigt auf
/// dasselbe Kennzeichen wie [`Lauf::abbrechen`], und der Arbeitsfaden liest
/// weiter allein dieses eine.
#[derive(Debug, Clone)]
pub struct Abbruchgriff {
    kennzeichen: Arc<AtomicBool>,
}

impl Abbruchgriff {
    /// Bricht den Vorgang ab, zu dem dieser Griff gehoert.
    pub fn abbrechen(&self) {
        self.kennzeichen.store(true, Ordering::Relaxed);
    }
}

/// Ein laufender Vorgang auf einem eigenen Arbeitsfaden.
///
/// **`#[must_use]` steht am Typ und nicht an [`super::starten`]**, das ihn
/// heute als einziges liefert: der Griff traegt den Meldungskanal und den
/// Abbruch, und ein fallen gelassener Lauf beendet den Vorgang, ohne dass
/// jemand davon erfaehrt. Ein zweiter Erzeuger erbt die Marke damit von
/// selbst.
#[must_use = "der Lauf ist der einzige Griff an den Vorgang; fallen gelassen endet er"]
pub struct Lauf {
    abbruch: Arc<AtomicBool>,
    meldungen: Receiver<Meldung>,
    faden: Option<JoinHandle<()>>,
}

impl Lauf {
    /// Ein Lauf um einen Arbeitsfaden, oder um keinen.
    ///
    /// **`None` ist keine Bequemlichkeit, sondern der Fadenmangel.** Scheitert
    /// `thread::Builder::spawn` in [`super::starten`], weil dem Prozess die
    /// Faeden ausgegangen sind, entsteht ein Lauf ohne Faden und mit
    /// geschlossenem Kanal. Er meldet nichts, [`Lauf::warten`] hat nichts zu
    /// erwarten, und der Rufer liest die Lage aus dem geschlossenen Kanal
    /// (`shared/issues/260826-1221_*_zwei-fadenstarts-des-verzeichnisbaums-brechen-mit-panik-ab-waehrend-derselbe-mangel-am-deskriptor-sorgfaeltig-behandelt-ist.md`).
    pub(crate) fn neu(
        abbruch: Arc<AtomicBool>,
        meldungen: Receiver<Meldung>,
        faden: Option<JoinHandle<()>>,
    ) -> Self {
        Self {
            abbruch,
            meldungen,
            faden,
        }
    }

    /// Der Kanal, aus dem der Hauptfaden die Meldungen holt.
    #[must_use]
    pub fn meldungen(&self) -> &Receiver<Meldung> {
        &self.meldungen
    }

    /// Ein Griff an das Abbruchkennzeichen, der den Lauf verlassen darf.
    ///
    /// Fuer den Aufrufer, der den Lauf einem anderen Faden gibt und trotzdem
    /// abbrechen koennen muss; siehe [`Abbruchgriff`]. Beliebig oft abrufbar,
    /// jeder Griff zeigt auf dasselbe Kennzeichen.
    #[must_use]
    pub fn abbruchgriff(&self) -> Abbruchgriff {
        Abbruchgriff {
            kennzeichen: Arc::clone(&self.abbruch),
        }
    }

    /// Bricht den Vorgang ab.
    ///
    /// Der Arbeitsfaden bemerkt den Abbruch zwischen zwei Eintraegen und,
    /// waehrend eine einzelne Datei uebertragen wird, im Statusrueckruf von
    /// `copyfile(3)`.
    pub fn abbrechen(&self) {
        self.abbruch.store(true, Ordering::Relaxed);
    }

    /// Wartet, bis der Arbeitsfaden geendet hat.
    ///
    /// Nur fuer Aufrufer, die den Kanal schon leergeraeumt haben.
    pub fn warten(mut self) {
        if let Some(faden) = self.faden.take() {
            let _ = faden.join();
        }
    }
}

impl Drop for Lauf {
    /// Fordert den Abbruch an, wartet aber nicht auf den Faden.
    ///
    /// Warten hiesse, dass das Schliessen eines Fensters auf eine Kopie von
    /// 50 GB wartet. Der Faden endet von selbst, sobald er das
    /// Abbruchkennzeichen bemerkt.
    fn drop(&mut self) {
        self.abbrechen();
    }
}

/// Der Zaehl- und Meldestand einer laufenden Operation.
///
/// Alle Module der Operationsmaschine rechnen ueber diesen einen Wert: er
/// zaehlt Eintraege und Bytes, kennt das Abbruchkennzeichen, taktet die
/// Fortschrittsmeldungen und haelt die Konfliktregel, die sich mit "fuer alle
/// weiteren uebernehmen" waehrend des Laufs aendern kann.
pub struct Steuerung {
    abbruch: Arc<AtomicBool>,
    sender: Option<Sender<Meldung>>,
    regel: Konfliktregel,
    eintraege: u64,
    bytes: u64,
    uebersprungen: Vec<Uebersprungen>,
    zuletzt_gemeldet: Option<Instant>,
}

impl Steuerung {
    /// Eine Steuerung mit Kanal und Abbruchkennzeichen.
    pub(crate) fn neu(
        abbruch: Arc<AtomicBool>,
        sender: Option<Sender<Meldung>>,
        regel: Konfliktregel,
    ) -> Self {
        Self {
            abbruch,
            sender,
            regel,
            eintraege: 0,
            bytes: 0,
            uebersprungen: Vec::new(),
            zuletzt_gemeldet: None,
        }
    }

    /// Wahr, sobald der Abbruch angefordert ist.
    pub(crate) fn abgebrochen(&self) -> bool {
        self.abbruch.load(Ordering::Relaxed)
    }

    /// Meldet einen Zwischenstand innerhalb eines Eintrags, getaktet.
    pub(crate) fn zwischenstand(&mut self, eintrag: &Path, bytes_im_eintrag: u64) {
        let jetzt = Instant::now();
        if let Some(zuletzt) = self.zuletzt_gemeldet
            && jetzt.duration_since(zuletzt) < MELDEABSTAND
        {
            return;
        }
        self.zuletzt_gemeldet = Some(jetzt);
        self.senden(Meldung::Fortschritt(Fortschritt {
            eintraege: self.eintraege,
            bytes: self.bytes + bytes_im_eintrag,
            eintrag: eintrag.to_path_buf(),
        }));
    }

    /// Verbucht einen fertigen Eintrag und meldet den Stand, getaktet.
    pub(crate) fn eintrag_fertig(&mut self, eintrag: &Path, bytes: u64) {
        self.eintraege += 1;
        self.bytes += bytes;
        self.zuletzt_gemeldet = None;
        self.zwischenstand(eintrag, 0);
    }

    /// Verbucht Bytes, die vor einem Abbruch schon geflossen sind.
    ///
    /// Der Eintrag selbst zaehlt nicht als uebertragen: er ist halb da und
    /// wird weggeraeumt. Die Bytes zaehlen trotzdem, weil C4 nach der bis dahin
    /// uebertragenen Zahl fragt.
    pub(crate) fn teilstueck(&mut self, bytes: u64) {
        self.bytes += bytes;
    }

    /// Verbucht einen ausgelassenen Eintrag und meldet ihn sofort.
    ///
    /// **`grund` ist ein Tabellenwert**, also `sprache::text` oder
    /// `sprache::satz` ueber einen Schluessel aus `Text`, oder der Wortlaut,
    /// den das System oder die Zip-Kiste zu einem Fehler liefert
    /// (`super::grund`, `ZipError::to_string`); ein Stringliteral und ein
    /// `format!` stehen an keiner Rufstelle. Die Senkenprobe
    /// `keine_senke_der_oberflaeche_bekommt_ein_literal` in
    /// `crates/krk-core/tests/baum.rs` haelt das an den Argumenten jedes
    /// Rufers. Der Nutzer liest den Grund in der Abschlussliste, und in der
    /// Sprache, die macOS gewaehlt hat, kommt er allein aus der Tabelle an.
    pub(crate) fn ueberspringen(&mut self, pfad: &Path, grund: impl Into<String>) {
        let eintrag = Uebersprungen {
            pfad: pfad.to_path_buf(),
            grund: grund.into(),
        };
        self.uebersprungen.push(eintrag.clone());
        self.senden(Meldung::Uebersprungen(eintrag));
    }

    /// Wie viele Eintraege bis jetzt uebersprungen worden sind.
    ///
    /// Ein Stand zum Vormerken: wer eine Teilarbeit anstoesst und danach
    /// wissen will, ob sie etwas uebersprungen hat, merkt sich diesen Wert
    /// vorher und fragt danach [`Steuerung::uebersprungen_seit`].
    #[must_use]
    pub(crate) fn uebersprungen_stand(&self) -> usize {
        self.uebersprungen.len()
    }

    /// Die Eintraege, die seit dem vorgemerkten Stand uebersprungen worden sind.
    ///
    /// Leer heisst: seither ist jeder Eintrag angekommen.
    #[must_use]
    pub(crate) fn uebersprungen_seit(&self, stand: usize) -> &[Uebersprungen] {
        &self.uebersprungen[stand.min(self.uebersprungen.len())..]
    }

    /// Loest einen Namenskonflikt auf, notfalls durch Nachfragen.
    ///
    /// Wartet der Vorgang auf eine Antwort und kommt keine, gilt das als
    /// Abbruch. Ein Kanal ohne Gegenueber ist kein Grund, ungefragt zu
    /// ueberschreiben.
    pub(crate) fn konflikt_loesen(&mut self, quelle: &Path, ziel: &Path) -> Konfliktantwort {
        match self.regel {
            Konfliktregel::Ueberschreiben => Konfliktantwort::Ueberschreiben,
            Konfliktregel::Ueberspringen => Konfliktantwort::Ueberspringen,
            Konfliktregel::AutomatischUmbenennen => {
                Konfliktantwort::UmbenennenIn(freier_name(ziel))
            }
            Konfliktregel::Abbrechen => Konfliktantwort::Abbrechen,
            Konfliktregel::Fragen => self.nachfragen(quelle, ziel),
        }
    }

    /// Fragt nach einem anderen Namen, weil der gewuenschte vergeben ist.
    ///
    /// Der Frager des Duplizierens (`duplizieren::eintrag_duplizieren`).
    /// `quelle` ist die Datei, die dupliziert wird, `ziel` der Pfad, an dem das
    /// Dateisystem eben einen vorhandenen Eintrag gemeldet hat; beide reisen in
    /// der [`Meldung::Konflikt`] zum Hauptfaden.
    ///
    /// **Die Frage geht denselben Weg wie [`Steuerung::konflikt_loesen`] unter
    /// [`Konfliktregel::Fragen`], und die Regel des Laufs liest sie nicht.**
    /// Die Regel sagt, was mit einem vorhandenen Ziel geschieht: ersetzen,
    /// ueberspringen, von selbst umbenennen. Keine dieser Antworten gibt es
    /// beim Duplizieren, und ein Lauf, der sie der Regel entnaehme, ersetzte
    /// unter `Ueberschreiben` einen Eintrag, nach dem niemand gefragt wurde.
    ///
    /// **Nur ein Name ist eine Antwort.** [`Konfliktantwort::UmbenennenIn`]
    /// wird `Some(name)`; jede andere Antwort, ein fehlender Sender und ein
    /// fallen gelassener Antwortkanal werden `None`, und der Rufer beendet den
    /// Vorgang, ohne etwas anzulegen. Ob der Name ein Name ist und ob er frei
    /// ist, prueft diese Funktion nicht: das erste tut der Rufer ueber
    /// `name_pruefen`, das zweite beantwortet das Dateisystem beim naechsten
    /// Versuch.
    ///
    /// `#[must_use]`: ein fallen gelassenes `None` hiesse, nach einer
    /// ausgebliebenen Antwort weiterzumachen.
    #[must_use]
    pub(crate) fn namen_erfragen(&mut self, quelle: &Path, ziel: &Path) -> Option<String> {
        match self.nachfragen(quelle, ziel) {
            Konfliktantwort::UmbenennenIn(name) => Some(name),
            Konfliktantwort::Ueberschreiben
            | Konfliktantwort::Ueberspringen
            | Konfliktantwort::Abbrechen => None,
        }
    }

    fn nachfragen(&mut self, quelle: &Path, ziel: &Path) -> Konfliktantwort {
        let Some(sender) = &self.sender else {
            return Konfliktantwort::Abbrechen;
        };
        let (hin, zurueck) = sync_channel(1);
        let frage = Meldung::Konflikt {
            quelle: quelle.to_path_buf(),
            ziel: ziel.to_path_buf(),
            antwort: hin,
        };
        if sender.send(frage).is_err() {
            return Konfliktantwort::Abbrechen;
        }
        let Ok(entscheid) = zurueck.recv() else {
            return Konfliktantwort::Abbrechen;
        };
        if entscheid.fuer_alle_weiteren {
            self.regel = match entscheid.antwort {
                Konfliktantwort::Ueberschreiben => Konfliktregel::Ueberschreiben,
                Konfliktantwort::Ueberspringen => Konfliktregel::Ueberspringen,
                // Ein von Hand getippter Name gilt fuer einen Eintrag und laesst
                // sich nicht auf die weiteren uebertragen. "Fuer alle weiteren"
                // heisst hier deshalb: such jedes Mal selbst einen freien Namen.
                Konfliktantwort::UmbenennenIn(_) => Konfliktregel::AutomatischUmbenennen,
                Konfliktantwort::Abbrechen => Konfliktregel::Abbrechen,
            };
        }
        entscheid.antwort
    }

    /// Schliesst die Buchfuehrung ab.
    pub(crate) fn bericht(self, abschluss: Abschluss) -> Bericht {
        Bericht {
            abschluss,
            eintraege: self.eintraege,
            bytes: self.bytes,
            uebersprungen: self.uebersprungen,
        }
    }

    fn senden(&self, meldung: Meldung) {
        if let Some(sender) = &self.sender {
            // Ein verschwundener Empfaenger ist kein Fehler: das Fenster ist zu,
            // und der Faden endet ohnehin gleich am Abbruchkennzeichen.
            let _ = sender.send(meldung);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    fn steuerung_mit_kanal(regel: Konfliktregel) -> (Steuerung, Receiver<Meldung>) {
        let (sender, empfaenger) = channel();
        let steuerung = Steuerung::neu(Arc::new(AtomicBool::new(false)), Some(sender), regel);
        (steuerung, empfaenger)
    }

    #[test]
    fn ein_uebersprungener_eintrag_steht_im_bericht_und_im_kanal() {
        let (mut steuerung, empfaenger) = steuerung_mit_kanal(Konfliktregel::Fragen);
        steuerung.ueberspringen(Path::new("/tmp/a"), "keine Leserechte");
        let bericht = steuerung.bericht(Abschluss::Fertig);

        assert_eq!(bericht.uebersprungen.len(), 1);
        assert_eq!(bericht.uebersprungen[0].grund, "keine Leserechte");
        assert!(matches!(empfaenger.recv(), Ok(Meldung::Uebersprungen(_))));
    }

    #[test]
    fn ohne_gegenueber_gilt_ein_konflikt_als_abbruch() {
        let mut steuerung = Steuerung::neu(
            Arc::new(AtomicBool::new(false)),
            None,
            Konfliktregel::Fragen,
        );
        let antwort = steuerung.konflikt_loesen(Path::new("/tmp/a"), Path::new("/tmp/b"));
        assert_eq!(antwort, Konfliktantwort::Abbrechen);
    }

    /// Beantwortet die naechste Konfliktfrage des Kanals mit `entscheid`.
    fn einmal_antworten(
        empfaenger: Receiver<Meldung>,
        entscheid: Konfliktentscheid,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            let Ok(Meldung::Konflikt { antwort, .. }) = empfaenger.recv() else {
                panic!("keine Konfliktfrage angekommen");
            };
            antwort
                .send(entscheid)
                .expect("Antwort laesst sich nicht senden");
        })
    }

    /// Die Namensfrage nimmt allein einen Namen als Antwort, und sie wird unter
    /// jeder Konfliktregel gestellt.
    ///
    /// **Die Regel des Laufs steht in der Tafel, damit abzulesen ist, dass sie
    /// nicht gelesen wird**: unter `Ueberschreiben` antwortete
    /// [`Steuerung::konflikt_loesen`] ohne zu fragen, und der Beantworter hier
    /// bliebe ohne Frage stehen.
    #[test]
    fn die_namensfrage_wird_unter_jeder_regel_gestellt_und_nimmt_allein_einen_namen() {
        let tafel = [
            (
                Konfliktregel::Fragen,
                Konfliktantwort::UmbenennenIn("anders.txt".to_owned()),
                Some("anders.txt"),
            ),
            (
                Konfliktregel::Ueberschreiben,
                Konfliktantwort::UmbenennenIn("anders.txt".to_owned()),
                Some("anders.txt"),
            ),
            (
                Konfliktregel::AutomatischUmbenennen,
                Konfliktantwort::Abbrechen,
                None,
            ),
            (
                Konfliktregel::Ueberspringen,
                Konfliktantwort::Ueberschreiben,
                None,
            ),
            (
                Konfliktregel::Abbrechen,
                Konfliktantwort::Ueberspringen,
                None,
            ),
        ];
        for (regel, antwort, erwartet) in tafel {
            let (mut steuerung, empfaenger) = steuerung_mit_kanal(regel);
            let beantworter =
                einmal_antworten(empfaenger, Konfliktentscheid::einmal(antwort.clone()));

            let name = steuerung.namen_erfragen(Path::new("/tmp/a"), Path::new("/tmp/b"));
            beantworter.join().expect("Beantworter gescheitert");

            assert_eq!(
                name.as_deref(),
                erwartet,
                "unter {regel:?} mit der Antwort {antwort:?}"
            );
        }
    }

    #[test]
    fn ohne_gegenueber_gibt_die_namensfrage_keinen_namen() {
        let mut steuerung = Steuerung::neu(
            Arc::new(AtomicBool::new(false)),
            None,
            Konfliktregel::AutomatischUmbenennen,
        );
        assert_eq!(
            steuerung.namen_erfragen(Path::new("/tmp/a"), Path::new("/tmp/b")),
            None
        );
    }

    #[test]
    fn fuer_alle_weiteren_aendert_die_regel_des_laufs() {
        let (mut steuerung, empfaenger) = steuerung_mit_kanal(Konfliktregel::Fragen);
        let beantworter = std::thread::spawn(move || {
            let Ok(Meldung::Konflikt { antwort, .. }) = empfaenger.recv() else {
                panic!("keine Konfliktfrage angekommen");
            };
            antwort
                .send(Konfliktentscheid::fuer_alle(Konfliktantwort::Ueberspringen))
                .expect("Antwort laesst sich nicht senden");
        });

        let erste = steuerung.konflikt_loesen(Path::new("/tmp/a"), Path::new("/tmp/b"));
        beantworter.join().expect("Beantworter gescheitert");
        assert_eq!(erste, Konfliktantwort::Ueberspringen);

        // Die zweite Frage stellt der Lauf nicht mehr; niemand hoert mehr zu,
        // und trotzdem kommt dieselbe Antwort.
        let zweite = steuerung.konflikt_loesen(Path::new("/tmp/c"), Path::new("/tmp/d"));
        assert_eq!(zweite, Konfliktantwort::Ueberspringen);
    }

    #[test]
    fn der_fortschritt_wird_getaktet_und_der_erste_kommt_sofort() {
        let (mut steuerung, empfaenger) = steuerung_mit_kanal(Konfliktregel::Fragen);
        for _ in 0..100 {
            steuerung.zwischenstand(Path::new("/tmp/a"), 1);
        }
        let gezaehlt = empfaenger.try_iter().count();
        assert_eq!(
            gezaehlt, 1,
            "hundert Meldungen in einem Wimpernschlag muessen zu einer werden"
        );
    }

    #[test]
    fn ein_fertiger_eintrag_meldet_immer() {
        let (mut steuerung, empfaenger) = steuerung_mit_kanal(Konfliktregel::Fragen);
        steuerung.eintrag_fertig(Path::new("/tmp/a"), 10);
        steuerung.eintrag_fertig(Path::new("/tmp/b"), 20);
        let gezaehlt = empfaenger.try_iter().count();
        assert_eq!(
            gezaehlt, 2,
            "ein fertiger Eintrag ist keine Zwischenmeldung"
        );

        let bericht = steuerung.bericht(Abschluss::Fertig);
        assert_eq!(bericht.eintraege, 2);
        assert_eq!(bericht.bytes, 30);
    }
}
