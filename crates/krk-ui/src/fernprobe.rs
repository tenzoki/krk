//! Ein ferner Ort fuer die Proben dieser Kiste, ohne Netz und ohne `ssh`.
//!
//! Wie die Proben des Kerns in `crates/krk-core/tests/fern.rs` startet
//! [`ferner_ort`] `/usr/libexec/sftp-server` unmittelbar als Kindprozess und
//! baut darueber einen echten [`Fernzugang`] auf: derselbe Code wie gegen einen
//! Server, nur ohne `ssh` davor. Gebaut wird in einem Faden aus
//! `krk_core::fern::faden`, denn der Aufbau verlangt das Pfand, das nur dort
//! entsteht; die Probe selbst bekommt den fertigen Ort und stellt keine
//! Anfrage, solange sie ihn nicht einem Lesevorgang gibt.
//!
//! Kennung und Pfad sind frei waehlbar: der Pfad muss auf dem Server nicht
//! stehen, solange die Probe nur am Wert rechnet. Der Kindprozess endet, sobald
//! der letzte Halter des Zugangs faellt.

use std::sync::Arc;

use krk_core::fern::{Fernpfad, Fernzugang, Kanalplan, Serverkennung, faden};
use krk_core::ort::FernerOrt;

/// Der Server des Systems, ohne `ssh` davor.
const SFTP_SERVER: &str = "/usr/libexec/sftp-server";

/// Ein Zugang zum Server des Systems unter der Kennung `kai@rechner[:port]`.
pub fn zugang(rechner: &str, port: Option<u16>) -> Arc<Fernzugang> {
    let kennung = Serverkennung::neu(rechner, port, Some("kai"));
    faden("krk-probe-fern".to_owned(), move |pfand| {
        Fernzugang::aufbauen(pfand, kennung, "Probe", &Kanalplan::nur(SFTP_SERVER))
    })
    .expect("kein Faden fuer die Probe")
    .join()
    .expect("der Aufbau ist abgestuerzt")
    .expect("sftp-server laesst sich nicht ansprechen")
}

/// Ein ferner Ort `kai@rechner[:port]:pfad` auf einem frischen Zugang.
pub fn ferner_ort(rechner: &str, port: Option<u16>, pfad: &str) -> FernerOrt {
    FernerOrt {
        zugang: zugang(rechner, port),
        pfad: Fernpfad::aus_text(pfad).expect("der Probenpfad ist absolut"),
    }
}
