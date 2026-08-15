//! Worauf ein Name zeigt, gefragt am Deskriptor (Defekt `260814-1612`).
//!
//! ```text
//! ein Pfad ──> sys::ohne_warten_oeffnen ──> metadata am Deskriptor
//!                                                     │
//!                             ┌───────────────────────┼───────────────┐
//!                             v                       v               v
//!                          Ordner                 KeinOrdner    Unerreichbar
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
//! bei einem Doppelklick und nicht bei der Anzeige.
//!
//! # Gefragt wird der Deskriptor und nicht der Name
//!
//! [`bestimmen`] nimmt keine eigene Huelle um `open` oder `stat`, sondern
//! [`super::sys::ohne_warten_oeffnen`], die im Baum eingefuehrte Form: oeffnen
//! mit `O_NONBLOCK`, danach `metadata()` am offenen Deskriptor, `O_NONBLOCK`
//! wieder abnehmen. Damit fallen zwei Dinge weg, die eine Pruefung am Pfad
//! haette — das Fenster zwischen Pruefung und Oeffnen, und das Blockieren an
//! einer benannten Roehre. Geoeffnet wird ohne `O_NOFOLLOW`, also steht am
//! Deskriptor das **Ziel** der Verknuepfung; genau das ist hier die Frage.
//!
//! Dieses Modul ist damit der dritte Aufrufer jener Huelle, neben
//! `text::datei::lesen` und dem Leseweg der Vorschau in `krk-ui`. Wie bei jenen
//! beiden bleibt die Frage "was ist ein gueltiges Ziel" hier und nicht in der
//! Huelle: der Editor sucht eine gewoehnliche Datei, dieses Modul ein
//! Verzeichnis.

use std::io;
use std::path::Path;

use super::sys;

/// Was hinter einem Namen steht, nachdem eine Verknuepfung aufgeloest ist.
///
/// **Drei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig.**
/// Entweder es gibt einen Deskriptor, dann ist das Ding dahinter ein
/// Verzeichnis oder es ist keines; oder es gibt keinen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verweisziel {
    /// Ein Verzeichnis. Der Aufrufer darf hineingehen.
    Ordner,
    /// Etwas, das kein Verzeichnis ist: eine gewoehnliche Datei, eine
    /// Geraetedatei, eine benannte Roehre. Der Aufrufer behandelt den Namen wie
    /// eine Datei.
    KeinOrdner,
    /// Nicht aufloesbar. Der Name zeigt ins Leere, im Ring, oder das Oeffnen
    /// war nicht erlaubt.
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

/// Was hinter diesem Namen steht, gefragt an einem offenen Deskriptor.
///
/// Ein Systemaufrufpaar, und es faellt nur an, wo jemand die Antwort braucht;
/// warum das so zugeschnitten ist, steht im Modulkopf.
///
/// `#[must_use]`, weil der Aufruf sonst allein einen Deskriptor oeffnet und
/// wieder schliesst. Ein stilles Fallenlassen bliebe unbemerkt und waere genau
/// die Art Fehler, gegen die dieses Projekt die Angabe setzt.
#[must_use]
pub fn bestimmen(pfad: &Path) -> Verweisziel {
    let datei = match sys::ohne_warten_oeffnen(pfad) {
        Ok(datei) => datei,
        Err(fehler) => return unerreichbar(&fehler),
    };
    match datei.metadata() {
        Ok(angaben) if angaben.is_dir() => Verweisziel::Ordner,
        Ok(_) => Verweisziel::KeinOrdner,
        Err(fehler) => unerreichbar(&fehler),
    }
}

/// Der eine Bauplatz von [`Verweisziel::Unerreichbar`].
///
/// Beide Fehlschlaege von [`bestimmen`] tragen dieselbe Antwort; sie zweimal zu
/// schreiben hiesse, zwei Schreibweisen fuer denselben Grund zu haben.
fn unerreichbar(fehler: &io::Error) -> Verweisziel {
    Verweisziel::Unerreichbar {
        grund: fehler.to_string(),
    }
}
