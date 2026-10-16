//! Der gemerkte Schluessel von `secrets.txt`: zehn Minuten ab der Eingabe,
//! danach getilgt.
//!
//! Wer `secrets.txt` mit seiner PIN geoeffnet hat, soll in den naechsten zehn
//! Minuten nicht erneut gefragt werden, wenn er sie wieder oeffnet oder eine
//! Verbindung herstellt
//! (`261016-0306_*_wie-lange-gilt-ein-eingegebener-schluessel-fuer-secrets-txt-und-was-wird-gemerkt.md`,
//! Moeglichkeit 1). **Gemerkt wird der abgeleitete [`Schluessel`] und nicht die
//! PIN**: ein Oeffnen mit ihm kostet keine Ableitung, und die PIN steht nach
//! dem Ableiten nirgends mehr im Merker.
//!
//! # Rein und ohne Uhr
//!
//! [`Schluesselmerker`] liest keine Uhr. Jede Frage bekommt `jetzt` vom Rufer,
//! damit die Grenze von zehn Minuten ohne Warten pruefbar ist; der Rufer liest
//! die Uhr an genau einer Stelle, [`crate::verzeichnis::sys::wachzeit`].
//!
//! **Die Uhr ist `CLOCK_MONOTONIC` und nicht `std::time::Instant`**, und das
//! ist am Quelltext der Toolchain nachgelesen und nicht vermutet: `Instant`
//! liest in Rust 1.97.1 auf jedem Apple-Ziel `CLOCK_UPTIME_RAW`
//! (`library/std/src/sys/time/unix.rs`, Konstante `Instant::CLOCK_ID`, am
//! 261016 im Quelltext der installierten Toolchain gelesen, Komponente
//! `rust-src`), und diese Uhr steht nach der Handbuchseite, die der Kommentar
//! dort zitiert, im Ruhezustand still („does not increment while the system is
//! asleep“). Mit `Instant` gemessen, gaelte ein Schluessel nach einer Nacht im
//! Ruhezustand weiter, solange der Mac zusammen keine zehn Minuten wach war.
//! `CLOCK_MONOTONIC` laeuft nach `man clock_gettime` dieses Systems im
//! Ruhezustand weiter, und eine verstellte Wanduhr beruehrt sie nicht.
//!
//! # Die Frist
//!
//! - **Sie zaehlt ab dem Merken und verlaengert sich nicht.** [`Schluesselmerker::gueltiger`]
//!   fragt und merkt nicht neu; nur [`Schluesselmerker::merken`] setzt die
//!   Frist, und das ruft der Rufer allein nach einer geprueften Eingabe
//!   (Oeffnen mit PIN, Festlegen, Aendern), nie nach einem Oeffnen mit dem
//!   gemerkten Schluessel.
//! - **Sie endet bei [`GUELTIGKEIT`] genau**: nach 599 Sekunden gilt der
//!   Schluessel, nach 600 nicht mehr.
//! - **Eine Wachzeit vor dem Merken zaehlt als abgelaufen.** `CLOCK_MONOTONIC`
//!   geht nicht rueckwaerts; meldet die Uhr trotzdem einen frueheren Wert, etwa
//!   die Null, mit der [`crate::verzeichnis::sys::wachzeit`] einen Fehler des
//!   Systems beantwortet, faellt der Schluessel, statt laenger zu gelten.
//!
//! # Tilgen
//!
//! Ein Schluessel, der den Merker verlaesst, faellt: beim Ablauf (in
//! [`Schluesselmerker::gueltiger`]),
//! beim Ersetzen durch [`Schluesselmerker::merken`] und bei
//! [`Schluesselmerker::verwerfen`]. In jedem Fall tilgt `Drop` von
//! [`Schluessel`] seine 32 Byte; was das nicht erreicht, sagt der Modulkopf
//! von [`super::tresor`].
//!
//! # Was der Merker nicht weiss
//!
//! Er kennt den Pfad der Datei nicht. Passt der gemerkte Schluessel nicht zur
//! Datei, die geoeffnet werden soll (anderer Notizordner, PIN von einer
//! zweiten Instanz geaendert, ein veraendertes Byte), antwortet
//! [`super::tresor::oeffnen_mittels`] wie bei einer falschen PIN, und der Rufer
//! verwirft den Schluessel und fragt nach der PIN. Eine Regel deckt damit
//! jeden Wechsel hinter dem Ruecken des Merkers.

use std::time::Duration;

use super::tresor::Schluessel;

/// Wie lange ein gemerkter Schluessel gilt, ab dem Merken: zehn Minuten. Eine
/// Einstellung dafuer gibt es nicht.
pub const GUELTIGKEIT: Duration = Duration::from_secs(600);

/// Der eine gemerkte Schluessel des Prozesses, oder keiner.
///
/// `Debug` zeigt allein, ob und seit wann ein Schluessel gemerkt ist, und
/// weder Schluesselbytes noch Salz.
#[derive(Default)]
pub struct Schluesselmerker {
    /// Der Schluessel und die Wachzeit, zu der er gemerkt wurde.
    eintrag: Option<(Schluessel, Duration)>,
}

impl std::fmt::Debug for Schluesselmerker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schluesselmerker")
            .field("gemerkt_seit", &self.eintrag.as_ref().map(|(_, seit)| seit))
            .finish()
    }
}

impl Schluesselmerker {
    /// Ein leerer Merker.
    pub fn neu() -> Self {
        Self::default()
    }

    /// Merkt `schluessel` ab der Wachzeit `jetzt`. Ein vorher gemerkter
    /// Schluessel faellt und wird getilgt; die Frist beginnt neu.
    ///
    /// Gerufen allein nach einer geprueften Eingabe; dass ein Oeffnen mit dem
    /// gemerkten Schluessel nicht neu merkt, haelt der Rufer.
    pub fn merken(&mut self, schluessel: Schluessel, jetzt: Duration) {
        self.eintrag = Some((schluessel, jetzt));
    }

    /// Der gemerkte Schluessel, wenn er zur Wachzeit `jetzt` noch gilt.
    ///
    /// Ist die Frist um, faellt er hier und wird getilgt, und die Antwort ist
    /// `None`. Die Frage verlaengert die Frist nicht.
    pub fn gueltiger(&mut self, jetzt: Duration) -> Option<&Schluessel> {
        if self.restdauer(jetzt).is_none() {
            self.verwerfen();
            return None;
        }
        self.eintrag.as_ref().map(|(schluessel, _)| schluessel)
    }

    /// Wie lange der gemerkte Schluessel zur Wachzeit `jetzt` noch gilt;
    /// `None`, wenn keiner gemerkt ist oder seine Frist um ist. Tilgt nicht;
    /// fuer den Wecker, der sich auf den Rest stellt und beim Feuern ueber
    /// [`Self::gueltiger`] leert.
    #[must_use]
    pub fn restdauer(&self, jetzt: Duration) -> Option<Duration> {
        let (_, seit) = self.eintrag.as_ref()?;
        let vergangen = jetzt.checked_sub(*seit)?;
        GUELTIGKEIT
            .checked_sub(vergangen)
            .filter(|rest| !rest.is_zero())
    }

    /// Laesst den gemerkten Schluessel fallen und tilgt ihn; ohne Schluessel
    /// tut es nichts.
    pub fn verwerfen(&mut self) {
        self.eintrag = None;
    }
}
