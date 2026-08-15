//! Worauf ein Name zeigt, gefragt am Pfad (Defekt `260814-1612`, Befund
//! `260815-1713`).
//!
//! ```text
//! ein Pfad ──> std::fs::metadata  (ein stat(2) am Namen)
//!                      │
//!      ┌───────────────┼───────────────┐
//!      v               v               v
//!   Ordner        KeinOrdner      Unerreichbar
//! ```
//!
//! # Wozu es dieses Modul gibt
//!
//! Der Verzeichnisleser folgt einer Verknuepfung nicht: er meldet sie als
//! [`Typ::Verknuepfung`](super::Typ::Verknuepfung), gleichgueltig worauf sie
//! zeigt. Das ist richtig fuer die Liste und falsch fuer den Einstieg — der
//! Nutzer erwartet, dass eine Verknuepfung auf einen Ordner sich beim
//! Hineingehen wie dieser Ordner verhaelt. Aufgeloest wird deshalb **allein im
//! Einstiegsweg** und nicht beim Lesen.
//!
//! **Der Lesevorgang bekommt dafuer keinen zusaetzlichen Systemaufruf.** Ein
//! `stat` je Verknuepfung bei jedem Lesen aendert die Rechnung, an der die
//! Zeitzusagen L3 und L10 haengen; der Sortierschluessel entsteht einmal beim
//! Lesen, und dort kommt nichts hinzu. Der eine Aufruf dieses Moduls faellt
//! erst an, wenn jemand tatsaechlich in eine Verknuepfung einsteigen will, also
//! bei einem Doppelklick und nicht bei der Anzeige. Diese Zusage haengt am
//! Aufrufer und nicht an der Form des Aufrufs; sie galt vor dem 260815 und
//! gilt danach unveraendert.
//!
//! # Gefragt wird der Name und nicht ein Deskriptor
//!
//! [`bestimmen`] fragt `std::fs::metadata`, also ein `stat(2)` am Pfad. Bis zum
//! 260815 nahm es stattdessen [`super::sys::ohne_warten_oeffnen`] — oeffnen mit
//! `O_NONBLOCK`, `metadata()` am offenen Deskriptor, `O_NONBLOCK` wieder
//! abnehmen —, weil das die im Baum eingefuehrte Form ist. Eingefuehrt ist sie
//! zu Recht, und hier war sie trotzdem falsch. Der Unterschied, an dem das
//! haengt, ist einer, den der naechste Leser sonst genauso uebersieht:
//!
//! **Wer den Deskriptor danach benutzt, oeffnet. Wer nur fragt, was hinter dem
//! Namen steht, fragt am Namen.**
//!
//! Der Editor (`text::datei::oeffnen`) und der Leseweg der Vorschau in
//! `krk-ui` lesen aus genau dem Deskriptor, den sie geprueft haben. Sie kaufen
//! mit ihm zwei Dinge, und beide bekommt nur, wer ihn behaelt: es gibt kein
//! Fenster zwischen Pruefung und Benutzung, weil beides dasselbe Objekt trifft
//! und nicht denselben Namen; und `open` haengt an einer benannten Roehre ohne
//! Schreiber nicht fest, weil `O_NONBLOCK` gesetzt ist.
//!
//! Dieses Modul benutzt seinen Deskriptor nicht. Es gibt ihn am Ende der
//! Funktion sofort wieder ab, und der Aufrufer oeffnet danach den **Pfad** ein
//! zweites Mal (`krk-ui`, `tabelle::in_zeile_einsteigen` ruft `ordner_lesen`
//! mit dem Namen). Das Fenster zwischen Pruefung und Benutzung besteht damit
//! unveraendert fort; der Deskriptor kauft es nicht weg. Und `stat(2)` wartet
//! an einer Roehre nie, weil es sie nicht anfasst. Von den zwei Gewinnen ist
//! hier keiner ein Gewinn.
//!
//! Der Preis war dagegen echt, denn `open(2)` beantwortet eine andere Frage als
//! die gestellte: nicht "was steht hinter diesem Namen", sondern "darf ich das
//! aufmachen". Am Referenzgeraet gemessen (Befund
//! `shared/issues/260815-1713_*_verweisziel-beantwortet-die-ordnerfrage-mit-open-und-nicht-mit-stat.md`)
//! kam eine Verknuepfung auf eine Datei ohne Leserecht als
//! [`Verweisziel::Unerreichbar`] statt als [`Verweisziel::KeinOrdner`], ein
//! Verzeichnis mit Modus `0111` ebenso statt als [`Verweisziel::Ordner`], und
//! ein Unix-Socket scheiterte mit `EOPNOTSUPP`. Jeder dieser drei Eintraege ist
//! da, und von jedem laesst sich sagen, ob er ein Verzeichnis ist. Nur `open`
//! kann es nicht sagen.
//!
//! Dazu eine Nebenwirkung, die eine blosse Frage nicht haben darf: `open`
//! **oeffnet**. Bei einer Verknuepfung auf eine serielle Schnittstelle unter
//! `/dev/cu.*` hat schon das Oeffnen eine Wirkung am Geraet, und ein
//! Doppelklick in einem Dateimanager soll sie nicht ausloesen. `stat(2)` hat
//! keine.
//!
//! Der Wechsel ist damit **kein Rueckfall hinter die Bauform des Editors**,
//! sondern dieselbe Regel, richtig herum angewandt. Wer sie umdreht und hier
//! wieder einen Deskriptor holt, holt die drei Fehlfaelle und die
//! Geraetewirkung mit zurueck.
//!
//! # Dieselbe Frage steht im Baum schon einmal, mit demselben Aufruf
//!
//! `krk-ui`s `kommandos::pfadeingabe::pruefen` beantwortet fuer den Pfadsprung
//! dieselbe Frage — fuehrt dieser Name, Verknuepfungen gefolgt, auf ein
//! Verzeichnis? — und fragt ebenfalls `std::fs::metadata`. Ein Unterschied
//! zwischen beiden waere die zweite Wahrheit darueber, was KRK fuer einen
//! Ordner haelt, und die erste Abweichung faende keine Pruefung.
//!
//! Ein Unterschied bleibt und ist gewollt: `pfadeingabe::pruefen` prueft fuer
//! ein Verzeichnis zusaetzlich das Leserecht ueber `read_dir` und meldet, wenn
//! es fehlt. Dieses Modul prueft es nicht. Ein Doppelklick auf einen
//! gewoehnlichen [`Typ::Ordner`](super::Typ::Ordner) ohne Leserecht ist heute
//! wortlos, und eine Verknuepfung darauf verhaelt sich jetzt genauso: eine
//! Regel statt zweier. Dass der Pfadsprung an derselben Stelle meldet und der
//! Doppelklick schweigt, ist eine aeltere Ungleichheit und eine Frage an den
//! Nutzer
//! (`shared/issues/260815-1749_*_der-pfadsprung-meldet-den-ordner-ohne-leserecht-und-der-doppelklick-schweigt.md`).

use std::path::Path;

/// Was hinter einem Namen steht, nachdem eine Verknuepfung aufgeloest ist.
///
/// **Drei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig —
/// und zwar fuer die Ausgaenge des Verfahrens wie fuer die Zustaende, die die
/// Werte benennen.** `stat(2)` loest den Namen auf oder es loest ihn nicht auf.
/// Loest es ihn auf, ist das Ding dahinter ein Verzeichnis oder es ist keines.
/// Loest es ihn nicht auf, steht hinter dem Namen nichts, was von hier aus
/// erreichbar waere. Kein Zustand faellt in zwei Werte, und keiner faellt
/// durch.
///
/// **Bis zum 260815 galt der Satz nur fuer die Ausgaenge.** Das Verfahren
/// fragte `open(2)`, und eine gewoehnliche Datei ohne Leserecht ist genau das,
/// was [`Verweisziel::KeinOrdner`] beschreibt — und kam trotzdem als
/// [`Verweisziel::Unerreichbar`] zurueck. Zwei Werte beschrieben denselben
/// Zustand; das ist der Fall, den `rules/critical-stance.md` §4 als nicht
/// ueberschneidungsfreien Schnitt benennt (Befund `260815-1713`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verweisziel {
    /// Ein Verzeichnis. Der Aufrufer darf hineingehen.
    ///
    /// Ob es sich auch **lesen** laesst, sagt dieser Wert nicht; siehe den
    /// letzten Abschnitt des Modulkopfs.
    Ordner,
    /// Etwas, das kein Verzeichnis ist: eine gewoehnliche Datei, eine
    /// Geraetedatei, eine benannte Roehre, ein Socket. Der Aufrufer behandelt
    /// den Namen wie eine Datei.
    ///
    /// Auch dann, wenn das Ziel selbst kein Leserecht traegt. Ob es sich
    /// oeffnen laesst, entscheidet das Programm, das es oeffnet, und nicht der
    /// Einstiegsweg.
    KeinOrdner,
    /// Der Name loest sich nicht auf: hinter ihm steht nichts, was von hier aus
    /// erreichbar waere. Er zeigt ins Leere, im Ring, oder eine Stufe des
    /// Pfades laesst sich nicht durchschreiten.
    ///
    /// **"Ohne Recht" heisst hier ohne Recht am Pfad und nicht ohne Recht am
    /// Ziel.** Was das Ziel selbst erlaubt, fragt dieser Wert nicht und kann er
    /// nicht fragen: eine Datei ohne Leserecht ist ein
    /// [`Self::KeinOrdner`], ein Verzeichnis mit Modus `0111` ein
    /// [`Self::Ordner`].
    ///
    /// **Ein Fall fuer eine Meldung und nicht fuers stille Verschlucken.** Eine
    /// Verknuepfung, deren Ziel geloescht wurde, sieht in der Liste aus wie
    /// jede andere; ohne Meldung bliebe der Doppelklick darauf wirkungslos, und
    /// der Nutzer suchte den Fehler bei sich.
    Unerreichbar {
        /// Woran es lag, in einem Satzteil: die Meldung des Systems. Der Satz
        /// darum herum gehoert dem Aufrufer, der auch den Pfad hat.
        grund: String,
    },
}

/// Was hinter diesem Namen steht, gefragt am Pfad.
///
/// Ein einziger Systemaufruf, und er faellt nur an, wo jemand die Antwort
/// braucht; warum am Pfad und nicht an einem Deskriptor, steht im Modulkopf.
///
/// `metadata` folgt einer Verknuepfung, `symlink_metadata` taete es nicht.
/// Genau das ist hier die Frage: nicht ob der Name eine Verknuepfung ist — das
/// weiss der Aufrufer aus der Liste —, sondern worauf sie zeigt.
///
/// `#[must_use]`, weil der Aufruf **nichts** tut ausser zu antworten. Er
/// oeffnet nichts, aendert nichts und hinterlaesst nichts; wer den Wert fallen
/// laesst, hat den ganzen Aufruf umsonst gemacht, und still.
#[must_use]
pub fn bestimmen(pfad: &Path) -> Verweisziel {
    match std::fs::metadata(pfad) {
        Ok(angaben) if angaben.is_dir() => Verweisziel::Ordner,
        Ok(_) => Verweisziel::KeinOrdner,
        Err(fehler) => Verweisziel::Unerreichbar {
            grund: fehler.to_string(),
        },
    }
}
