//! Die zwei Absprachen ueber dem Ablageordner, und es sind genau zwei.
//!
//! Sobald KRK ein zweites Mal laeuft, greifen zwei Prozesse auf dieselben vier
//! Dateien unter `~/Library/Application Support/KRK/` zu. Zwei Dinge sind dabei
//! auseinanderzuhalten, und bis zum 260813 trugen sie ein Wort:
//!
//! ```text
//! Schreibgriff    kurzlebig   je Lesen-Aendern-Schreiben genommen und gleich
//!                             abgegeben; schuetzt die eine Nachbardatei
//! Sitzungsrecht   langlebig   einmal beim Start versucht, bis zum Prozessende
//!                             gehalten; beantwortet, wer die Sitzung schreibt
//! ```
//!
//! **Ein Mechanismus kann beides nicht leisten.** Hielte Instanz 1 ihn vom Start
//! bis zum Ende, kaeme keine zweite je zum Schreiben. Gaebe jeder Schreibvorgang
//! ihn wieder ab, hielte ihn nach dem ersten Schreiben niemand, und „wer ihn
//! haelt" beantwortete die Frage nach der Sitzung nicht mehr. Der Datensatz dazu
//! ist
//! `shared/decisions/260813-0053_*_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`,
//! Moeglichkeit 1.
//!
//! # Warum `flock` und keine Marke im Dateisystem
//!
//! Beide Sperren muessen frei werden, wenn ein Prozess **abstuerzt**. Eine Marke
//! ueber `OpenOptions::create_new` oder ueber `renamex_np` mit `RENAME_EXCL`
//! ueberlebt den Absturz und sperrte danach jede weitere Instanz fuer immer aus
//! dem Sitzungsschreiben aus; beide Mittel liegen im Baum bereit und reichen
//! fuer diese eine Anforderung trotzdem nicht. Der Kern gibt eine
//! `flock`-Sperre dagegen von sich aus frei, sobald der letzte Deskriptor auf
//! die offene Datei geschlossen wird, und das tut er auch nach einem `SIGKILL`.
//!
//! Der Preis ist ein fuenfter Fremdaufruf, und er faellt in
//! [`crate::verzeichnis::sys`] an, der einen Datei des Kerns mit
//! `#![allow(unsafe_code)]`. Eine zweite Datei mit dieser Ausnahme entsteht
//! nicht.
//!
//! # Zwei Dateien, und die Sperre gilt dem Ordner
//!
//! Die Griffe liegen auf [`SCHREIBSPERRE`] und [`SITZUNGSRECHT`] im
//! Ablageordner und **nicht** auf den vier Nutzdateien. Der Grund ist die
//! Nachbardatei: `atomar::schreiben` ersetzt die Zieldatei ueber ein `rename`,
//! und ein Griff auf die Zieldatei selbst haenge danach an einem Deskriptor, den
//! kein Name mehr nennt. Eine eigene Sperrdatei wird nie umbenannt und nie
//! ersetzt.
//!
//! # Was der Uebersetzer nicht haelt
//!
//! **Ein `flock` haengt an der offenen Datei und nicht am Prozess.** Daraus
//! folgen zwei Regeln, die keine Typangabe erzwingt und die deshalb hier stehen:
//!
//! - **Ein Durchgang wird nicht geschachtelt.** Ein zweiter [`Schreibgriff`] auf
//!   **denselben** Deskriptor blockiert nicht, sondern gibt die Sperre einfach
//!   erneut aus; der `Drop` des inneren gaebe danach die Sperre des aeusseren
//!   ab, und der Rest des aeusseren Durchgangs liefe ungeschuetzt.
//! - **Zwei [`crate::ablage::Ablage`]-Werte eines Prozesses duerfen nicht
//!   zugleich einen Durchgang fahren.** Sie halten zwei Deskriptoren auf
//!   dieselbe Sperrdatei, und zwei Deskriptoren blockieren einander so wie zwei
//!   Prozesse. Heute ist das erfuellt, weil `tasten::belegung::fuer_den_betrieb`
//!   seine Ablage verwirft, bevor die Oberflaeche die bleibende oeffnet.
//!
//! **Eine Verklemmung zwischen den beiden Sperren gibt es nicht.** Das
//! Sitzungsrecht wird beim Start genommen und nie, waehrend ein Schreibgriff
//! gehalten wird. Die Reihenfolge ist damit fest und ohne Ring.
//!
//! **Zwei Wege nehmen das Recht, und beide beim Start.** Der gewoehnliche Start
//! haelt es bis zum Prozessende; der Sitzungslauf des Messmodus
//! (`--messmodus <plan.toml>`) nimmt es, bevor er die Pruefsitzung schreibt, und
//! gibt es danach wieder ab. Beide gehen dieselbe Reihenfolge, und ein Prozess
//! haelt nie zwei davon zugleich: der Messlauf kehrt aus `sitzung_laden`
//! zurueck, bevor der gewoehnliche Weg dort ueberhaupt beginnt.

use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

use super::pfade::Ablageort;
use crate::verzeichnis::sys::{self, Sperrversuch};

/// Die Datei, an der die Schreibsperre haengt.
pub const SCHREIBSPERRE: &str = "schreiben.lock";

/// Die Datei, an der das Sitzungsrecht haengt.
pub const SITZUNGSRECHT: &str = "sitzungsrecht.lock";

/// Oeffnet eine der beiden Sperrdateien und legt sie an, falls sie fehlt.
///
/// Der Inhalt bleibt leer und wird von niemandem gelesen: was zaehlt, ist der
/// Deskriptor. `truncate` steht deshalb auf `false` — eine Datei ohne Inhalt
/// abzuschneiden waere Arbeit fuer nichts, und ein fremder Inhalt darin ginge
/// KRK nichts an.
pub(super) fn sperrdatei_oeffnen(ordner: &Path, name: &str) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(ordner.join(name))
}

/// Die gehaltene Schreibsperre ueber dem Ablageordner.
///
/// Sie umfasst einen **vollstaendigen** Durchgang aus Lesen, Aendern und
/// Schreiben und nicht nur das Schreiben: laege das Lesen ausserhalb, waere die
/// verlorene Aenderung nur seltener und nicht fort.
///
/// **`#[must_use]`, und der Grund ist nicht die Gewohnheit.** Ein
/// fallengelassener Griff gibt die Sperre unverzueglich wieder ab; der Durchgang
/// danach liefe ungeschuetzt weiter, und niemand saehe es. Der Wert gehoert
/// deshalb in eine Bindung, die so lange lebt wie der Durchgang. Genommen wird
/// er allein von [`crate::ablage::Ablage::durchgang`], das genau das tut.
#[must_use = "der Griff gibt die Sperre sofort wieder ab, wenn er fallengelassen wird"]
#[derive(Debug)]
pub struct Schreibgriff<'a> {
    datei: &'a File,
}

impl<'a> Schreibgriff<'a> {
    /// Nimmt die Sperre und **wartet**, bis sie frei ist.
    ///
    /// Ein Durchgang wird nicht abgewiesen, sondern kommt an die Reihe: er
    /// dauert einen Lese- und einen Schreibvorgang auf eine kleine TOML-Datei,
    /// und ein Wartender hier ist ein Nutzer, der eben ein Lesezeichen angelegt
    /// hat.
    pub(super) fn nehmen(datei: &'a File) -> io::Result<Self> {
        sys::sperre_nehmen(datei)?;
        Ok(Self { datei })
    }
}

impl Drop for Schreibgriff<'_> {
    /// Gibt die Sperre ab.
    ///
    /// Ein Fehlschlag bleibt hier unbeantwortet, und das ist die einzige
    /// moegliche Antwort: `Drop` kann nichts zurueckgeben. Scheitern kann der
    /// Aufruf nur an einem Deskriptor, der nicht mehr taugt, und dann gibt das
    /// Prozessende die Sperre ohnehin frei.
    fn drop(&mut self) {
        let _ = sys::sperre_abgeben(self.datei);
    }
}

/// Wer die Sitzung schreibt: das langlebige Merkmal, einmal beim Start vergeben.
///
/// **Die Frage, welche gespeicherte Sitzung zu welchem Prozess gehoert, ist aus
/// den Eingaben eines Prozesses nicht zu beantworten** — ein Prozess traegt
/// ueber einen Neustart hinweg keine Naemlichkeit, und jede Naeherung darueber
/// waere eine geratene Antwort. Dieser Typ beantwortet sie deshalb nicht,
/// sondern ersetzt sie durch eine entscheidbare: haelt dieser Prozess das
/// Sitzungsrecht.
///
/// **Gehalten wird es bis zum Ende des Prozesses**, und der Wert gehoert
/// entsprechend lange gehalten: mit ihm faellt der Deskriptor, und mit dem
/// Deskriptor die Sperre. Daher `#[must_use]`.
///
/// **Ein zweiter Versuch findet nicht statt.** Wer beim Start kein Recht bekam,
/// schreibt bis zu seinem Ende keine Sitzung, auch wenn die erste Instanz
/// vorher endet; eine wandernde Zustaendigkeit waere eine zweite Regel und ein
/// Wettlauf mehr. Eine Instanz, die **nach** dem Ende der ersten startet,
/// bekommt das Recht dagegen wie jede erste, und das ist keine Wanderung,
/// sondern die gewoehnliche Vergabe beim Start.
#[must_use = "faellt das Recht weg, faellt die Sperre mit, und eine zweite Instanz haelt sich fuer die erste"]
#[derive(Debug)]
pub struct Sitzungsrecht {
    /// Der Deskriptor, solange das Recht gehalten wird.
    ///
    /// `None` heisst „eine andere Instanz haelt es". Ein `Option` und kein
    /// zweiter Typ: der Aufrufer hat in beiden Faellen einen Wert in der Hand
    /// und fragt ihn, statt zwei Wege zu bauen.
    griff: Option<File>,
}

impl Sitzungsrecht {
    /// Versucht das Recht **ohne zu warten**.
    ///
    /// Scheitert nicht daran, dass ein anderer es haelt: das ist der erwartete
    /// Ausgang der zweiten Instanz und liefert ein Recht, das
    /// [`Sitzungsrecht::gehalten`] verneint. Ein Fehler kommt allein von einer
    /// Sperrdatei, die sich nicht oeffnen laesst.
    pub fn nehmen(ort: &Ablageort) -> io::Result<Self> {
        let datei = sperrdatei_oeffnen(ort.wurzel(), SITZUNGSRECHT)?;
        match sys::sperre_versuchen(&datei)? {
            Sperrversuch::Genommen => Ok(Self { griff: Some(datei) }),
            Sperrversuch::Belegt => Ok(Self { griff: None }),
        }
    }

    /// Ein Recht, das niemand hat.
    ///
    /// Der Wert fuer jeden Weg, auf dem es gar keinen Ablageordner gibt. Er
    /// verneint [`Sitzungsrecht::gehalten`] und laesst den Aufrufer damit
    /// denselben Zweig gehen wie die zweite Instanz, statt ihm einen dritten
    /// Fall aufzumachen.
    pub fn ohne() -> Self {
        Self { griff: None }
    }

    /// Ob dieser Prozess die Sitzung schreibt.
    #[must_use]
    pub fn gehalten(&self) -> bool {
        self.griff.is_some()
    }
}
