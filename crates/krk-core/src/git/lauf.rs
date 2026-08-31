//! Der Gitlauf: die vier Auskuenfte des Lesers auf einem eigenen Faden.
//!
//! [`leser::Gitleser`] ist synchron, und `marken()` kostet an einem Baum mit
//! 100 000 Eintraegen gemessen 12 bis 164 ms — ein Vielfaches eines Bildes. Auf
//! dem Hauptfaden gerufen hielte es den Zeichendurchgang an. Dieses Modul ist
//! die eine Stelle, die den Leser trotzdem ruft: ein Arbeitsfaden je Lauf, ein
//! Kanal zurueck, ein Abbruchkennzeichen, ein `Drop`, der es setzt. **Wer den
//! Gitbefund braucht, nimmt diesen Weg und keinen zweiten** (C7.1 der
//! Runde 23); die Zaehlprobe
//! `keine_statusabfrage_steht_ausserhalb_des_gitmoduls` in
//! `crates/krk-core/tests/git.rs` haelt es fest.
//!
//! ```text
//! abgebrochen?          ─ ja ──> nichts
//!            │ nein
//! oeffnen?              ─ KeinRepository ─> Kopf(KeinRepository), Ende
//!            │ Offen    └ Unentschieden ──> nichts
//! abgebrochen?          ─ ja ──> nichts
//!            │ nein
//! Kopf?                 ─ Some ─> Kopf(…)      (nur bei Gitfrage::Ganz)
//!            │          └ None ─> nichts
//! abgebrochen?          ─ ja ──> nichts
//!            │ nein
//! Verlauf?              ─ Some ─> Verlauf(…)
//!            │          └ None ─> nichts
//! abgebrochen?          ─ ja ──> nichts
//!            │ nein
//! Marken?               ─ Some ─> Marken(…)    (nur bei Gitfrage::Ganz)
//!   je Posten des Stroms  └ None ─> nichts
//!   erneut: abgebrochen?    (darunter: der Abbruch mitten im Strom)
//! ```
//!
//! # Die Bauform ist die des Durchlaufs, und zwei Dinge sind anders
//!
//! Gebaut ist der Lauf Zeile fuer Zeile nach
//! [`verzeichnis::durchlauf::Durchlauf`](crate::verzeichnis::durchlauf::Durchlauf):
//! [`Gitlauf::starten`] kehrt sofort zurueck, [`Gitlauf::meldungen`] gibt den
//! Kanal heraus, [`Gitlauf::abbrechen`] setzt das Kennzeichen, und `Drop` ruft
//! es. Eine zweite Bauart fuer dieselbe Sache — eine Auskunft von der Platte,
//! die nebenlaeufig entsteht und die Anzeige nachtraegt — entstuende sonst
//! neben der ersten. Zwei Unterschiede gehoeren aber hierher, weil sie sonst
//! als Nachlaessigkeit gelesen wuerden:
//!
//! **Die Kanaltiefe ist die Zahl der Antworten und kein Rueckstaumass.** Beim
//! Durchlauf haelt der Kanal einen Strom von Befunden, und seine Tiefe sagt,
//! wie weit der Arbeitsfaden dem Einzugstakt vorauslaufen darf. Hier gibt es je
//! Lauf hoechstens drei Meldungen, und ein Faden, der nach der dritten
//! blockierte, haette ohnehin nichts mehr zu tun. `sync_channel(3)` heisst
//! deshalb „so viele Antworten gibt es" und nicht „so weit darf er voraus".
//!
//! **Die Marken kommen in einem Stueck und nicht Eintrag fuer Eintrag.** Zwei
//! Gruende, und beide sind Zusagen des Specs. A8 verlangt, dass die
//! Markenspalte leer bleibt, bis der Befund da ist; eine fortschreitend
//! gefuellte Spalte waere genau das Flackern, das der Entscheid zum leeren
//! Ordner abgelehnt hat. Und die Zuordnung ueber den **Namen** braucht ein
//! Nachschlagewerk ueber den Bestand: es je eintreffendem Schwung aufzubauen
//! hiesse, bei hunderttausend Eintraegen mehrfach hunderttausend Namen zu
//! hashen, auf dem Hauptfaden. Einmal je Lauf ist einmal.
//!
//! # Was ausbleibt, heisst „nicht entschieden"
//!
//! Dieselbe Regel wie beim Durchlauf, und dieselbe wie im Kopf von
//! [`crate::git`]: ein geschlossener Kanal ohne [`Gitmeldung::Marken`] heisst
//! **nicht** „dieser Ordner hat keine Marken", sondern „der Befund steht
//! aus". Der Lauf meldet nur, was er entschieden hat; jedes `None` des Lesers
//! und jeder Abbruch enden ohne Meldung. Der Rufer laesst die Spalte dann leer,
//! wie sie vorher war, und schreibt nichts.
//!
//! Die eine entschiedene Verneinung ist [`Kopf::KeinRepository`], und sie kommt
//! aus [`leser::Oeffnung::KeinRepository`]. Sie steht am Anfang jedes Laufs und
//! nicht nur beim ganzen: **jeder** Lauf muss oeffnen, also beantwortet jeder
//! die Frage, ob dieser Ordner ueberhaupt in einem Repository liegt. Ein
//! Nachschlag auf einen Ordner, dessen Repository inzwischen weg ist, meldet
//! deshalb `Kopf(KeinRepository)` statt zu schweigen; eine Ausnahme davon waere
//! eine zweite Regel fuer dieselbe Frage.
//!
//! # Geprueft wird vor jeder Einheit, die dauern kann
//!
//! Die Abbruchzusage haengt nicht an der Kanaltiefe, sondern an den Einheiten
//! des Laufs — die Regel steht so im Kopf des Durchlaufs. Hier sind es das
//! Oeffnen, jede der drei Auskuenfte und, **innerhalb** der Marken, jeder
//! Posten des Statusstroms. Das Oeffnen zaehlt mit, weil `gix::discover` im
//! positiven Fall gemessen 346 bis 900 µs kostet und je Ebene bis zur Wurzel
//! nach `.git` sieht; ein Lauf, der erst danach nach dem Abbruch fragte, liefe
//! an einem tiefen Pfad ueber die Zusage hinaus.
//!
//! **Die letzte Einheit liegt als einzige nicht in dieser Datei**, und sie ist
//! die teuerste: [`Gitleser::marken`] kostet gemessen 12 bis 164 ms und lief
//! bis zum 260831 nach dem Eintritt in jedem Fall zu Ende, gleich ob der Lauf
//! aufgegeben war. Das Kennzeichen reist deshalb bis dorthin mit; der Datensatz
//! ist
//! `260831-1444_*_ein-abgebrochener-gitlauf-laeuft-weiter-und-a10-gilt-nur-dem-halter-und-nicht-dem-faden.md`.
//!
//! **Was das haelt und was nicht.** Ein aufgegebener Lauf bricht beim naechsten
//! Posten ab, statt den ganzen Status zu lesen. Nebeneinander laufen koennen
//! zwei Faeden trotzdem, und zwar so lange, wie der aeltere fuer einen Posten
//! und den Aufbau seines Stroms braucht: `Drop` wartet ausdruecklich nicht, und
//! ein Warten waere genau die Bildzeit, die dieser Lauf vermeidet.
//!
//! # Kein `warten`
//!
//! Wie beim Durchlauf: es gibt keinen Rufer, der auf den Abschluss wartet.
//! Dass der Faden geendet hat, sagt der geschlossene Kanal.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;

use super::leser::{Gitleser, Oeffnung};
use super::{Commit, Kopf, Marke};

/// Wie viele Commits ein Lauf holt, beim ersten Mal und bei jedem Nachschlag.
///
/// Fuenfzig (E12, C4.1 der Runde 23), und die Zahl steht hier, weil dieses
/// Modul der einzige Rufer von [`Gitleser::verlauf`] ist: eine zweite Zahl beim
/// Anzeigenden liefe mit dieser auseinander, sobald eine von beiden sich
/// aendert. Gemessen kosten fuenfzig Commits 2,5 bis 3,2 ms, also weniger als
/// ein Fuenftelbild; die Messung steht bei
/// [`leser::OBJEKTSPEICHER`](super::leser), und sie ist am 260831 mit der
/// Sortierung neu genommen worden (davor 3,9 ms).
///
/// **Woran der Rufer erkennt, dass nichts mehr folgt** (C4.3): die gemeldete
/// Liste ist kuerzer als diese Zahl. Ein eigenes Kennzeichen daneben waere eine
/// zweite Quelle fuer dieselbe Auskunft; [`Gitleser::verlauf`] sagt es an
/// seiner Stelle ebenso.
pub const VERLAUFSSCHRITT: usize = 50;

/// Wonach ein Lauf fragt.
///
/// **Zwei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig.**
/// [`Gitfrage::Ganz`] ist der Ordnerwechsel und die Auffrischung: alles, was
/// der Git-Bereich und die Markenspalte zeigen, entsteht neu.
/// [`Gitfrage::WeitererVerlauf`] ist das Nachladen aus E12: der Kopf steht
/// schon, die Marken stehen schon, gefragt sind allein die naechsten
/// [`VERLAUFSSCHRITT`] Commits hinter den schon angezeigten.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Gitfrage {
    /// Kopf, Verlauf und Marken, in dieser Reihenfolge.
    Ganz,
    /// Allein die naechsten Commits, hinter den schon angezeigten.
    WeitererVerlauf {
        /// Wie viele Commits schon dastehen. Sie kommen nicht noch einmal mit.
        ///
        /// **Die Zahl und nicht der letzte Commit**: ein Lauf, der beim
        /// zuletzt angezeigten Commit ansetzte, lieferte allein dessen
        /// Vorfahren und verloere jeden Nebenzweig, der beim Schwungende noch
        /// in der Warteschlange stand. [`Gitleser::verlauf`] schreibt es aus.
        bereits: usize,
    },
}

/// Was ein Lauf ueber den Kanal meldet.
///
/// **Drei Meldungsarten und keine zwei Vektoren**, vollstaendig und ohne
/// Auffangzweig. Je Lauf hoechstens drei Meldungen, bei
/// [`Gitfrage::WeitererVerlauf`] hoechstens eine; weniger heisst, dass der Rest
/// nicht entschieden ist.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Gitmeldung {
    /// Worauf HEAD steht, oder dass dieser Ordner in keinem Repository liegt.
    Kopf(Kopf),
    /// Die naechsten Commits, hoechstens [`VERLAUFSSCHRITT`] viele.
    Verlauf(Vec<Commit>),
    /// Der Status des angezeigten Ordners, vollstaendig, in einem Stueck.
    Marken(Vec<(String, Marke)>),
}

/// Ein laufender Gitlauf auf einem eigenen Faden.
#[derive(Debug)]
pub struct Gitlauf {
    abbruch: Arc<AtomicBool>,
    meldungen: Receiver<Gitmeldung>,
}

impl Gitlauf {
    /// Startet den Lauf und kehrt sofort zurueck.
    ///
    /// `ordner` ist der **angezeigte** Ordner und nicht die Wurzel des
    /// Repositorys; [`Gitleser::oeffnen`] sucht von dort aufwaerts, und
    /// [`Gitleser::marken`] beschraenkt den Status auf ihn (C7.7).
    ///
    /// `generation` benennt allein den Arbeitsfaden (`krk-gitlauf-<n>`), damit
    /// ein Fadenprotokoll lesbar bleibt. **Den Meldungen liegt sie nicht bei**,
    /// wie beim Durchlauf: jeder Tab haelt seinen eigenen Lauf und liest allein
    /// aus dessen Kanal. Wer die Marken ins Ordnermodell traegt, haelt die
    /// Generation trotzdem gegen — dort traegt der Befund einen **Namen**, den
    /// auch ein neuer Ordner fuehren kann, waehrend der Durchlauf einen
    /// Eintragsindex traegt, den das Modell am Bestandsende von selbst
    /// verwirft.
    #[must_use = "ein sofort fallengelassener Lauf bricht sich selbst ab und meldet nichts"]
    pub fn starten(ordner: PathBuf, frage: Gitfrage, generation: u64) -> Self {
        let abbruch = Arc::new(AtomicBool::new(false));
        let (sender, meldungen) = sync_channel(KANALTIEFE);
        let faden_abbruch = Arc::clone(&abbruch);
        thread::Builder::new()
            .name(format!("krk-gitlauf-{generation}"))
            .spawn(move || {
                gitlauffaden(&ordner, &frage, &faden_abbruch, &sender);
            })
            .expect("Arbeitsfaden fuer den Gitlauf laesst sich nicht starten");
        Self { abbruch, meldungen }
    }

    /// Der Kanal, aus dem der Hauptfaden die Meldungen holt.
    ///
    /// Er schliesst, wenn der Arbeitsfaden geendet hat. **Ein geschlossener
    /// Kanal ohne [`Gitmeldung::Marken`] heisst nicht, dass dieser Ordner keine
    /// Marken traegt: er heisst, dass der Befund nicht entschieden ist.**
    /// Dasselbe gilt fuer die beiden anderen Meldungsarten.
    pub fn meldungen(&self) -> &Receiver<Gitmeldung> {
        &self.meldungen
    }

    /// Bricht den Lauf ab.
    ///
    /// Der Arbeitsfaden bemerkt es vor der naechsten Einheit — der Modulkopf
    /// zaehlt sie auf — und meldet nichts mehr. Bereits gesendete Meldungen
    /// bleiben gueltig.
    pub fn abbrechen(&self) {
        self.abbruch.store(true, Ordering::Relaxed);
    }
}

impl Drop for Gitlauf {
    /// Fordert den Abbruch an, wartet aber nicht auf den Faden.
    ///
    /// Warten hiesse, dass ein Ordnerwechsel auf den Statuslauf des vorigen
    /// Ordners wartet — genau die Bildzeit, die dieser Lauf vermeiden soll. Der
    /// Faden endet von selbst: entweder bemerkt er das Abbruchkennzeichen, oder
    /// sein naechstes Senden scheitert, weil der Empfaenger mit dem [`Gitlauf`]
    /// gefallen ist. **Bemerkt wird es auch mitten im Status**, seit
    /// [`Gitleser::marken`] das Kennzeichen entgegennimmt; vorher lief die
    /// teuerste Einheit nach ihrem Eintritt in jedem Fall zu Ende.
    fn drop(&mut self) {
        self.abbrechen();
    }
}

/// Die Tiefe des Kanals: die Zahl der Antworten je Lauf.
///
/// Drei und nicht `STAPELGROESSE`. Der Modulkopf schreibt aus, warum das eine
/// andere Aussage ist als die Tiefe beim Durchlauf.
const KANALTIEFE: usize = 3;

/// Holt die Auskuenfte in der Reihenfolge ihrer Kosten und meldet jede einzeln.
///
/// **Die Reihenfolge ist Kopf, Verlauf, Marken**, und sie ist zweimal begruendet
/// und nicht einmal: sie ist die ihrer gemessenen Kosten — der Kopf steht nach
/// unter einer Millisekunde, der Verlauf nach knapp vier, die Marken nach zwoelf
/// bis hundertvierundsechzig —, und sie ist die, die A8 verlangt: Branch und
/// Verlauf stehen schon, waehrend die Markenspalte noch leer ist.
///
/// Endet ohne weitere Meldung, sobald der Abbruch greift, eine Auskunft
/// unentschieden bleibt oder der Empfaenger verschwunden ist.
fn gitlauffaden(
    ordner: &Path,
    frage: &Gitfrage,
    abbruch: &AtomicBool,
    sender: &SyncSender<Gitmeldung>,
) {
    if abbruch.load(Ordering::Relaxed) {
        return;
    }
    let leser = match Gitleser::oeffnen(ordner) {
        Oeffnung::Offen(leser) => leser,
        // Die eine entschiedene Verneinung, und sie gilt jeder Frage: auch ein
        // Nachschlag muss oeffnen, also beantwortet auch er sie.
        Oeffnung::KeinRepository => {
            let _ = sender.send(Gitmeldung::Kopf(Kopf::KeinRepository));
            return;
        }
        // Ein Deskriptormangel des eigenen Prozesses ist keine Auskunft ueber
        // diesen Ordner (C7.8).
        Oeffnung::Unentschieden => return,
    };

    // Vollstaendig und ohne Auffangzweig: der ganze Lauf holt alle drei, der
    // Nachschlag allein den Verlauf.
    let (kopf_holen, marken_holen, bereits) = match frage {
        Gitfrage::Ganz => (true, true, 0),
        Gitfrage::WeitererVerlauf { bereits } => (false, false, *bereits),
    };

    if kopf_holen {
        if abbruch.load(Ordering::Relaxed) {
            return;
        }
        let Some(kopf) = leser.kopf() else {
            return;
        };
        if sender.send(Gitmeldung::Kopf(kopf)).is_err() {
            return;
        }
    }

    if abbruch.load(Ordering::Relaxed) {
        return;
    }
    let Some(verlauf) = leser.verlauf(bereits, VERLAUFSSCHRITT) else {
        return;
    };
    if sender.send(Gitmeldung::Verlauf(verlauf)).is_err() {
        return;
    }

    if marken_holen {
        if abbruch.load(Ordering::Relaxed) {
            return;
        }
        let Some(marken) = leser.marken(ordner, abbruch) else {
            return;
        };
        let _ = sender.send(Gitmeldung::Marken(marken));
    }
}
