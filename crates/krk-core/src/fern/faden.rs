//! Das Pfand, ohne das keine Anfrage an einen Server gestellt werden kann.
//!
//! Jede Anfrage, die auf die Gegenseite wartet, nimmt ein `&Fernfaden`. Ein
//! [`Fernfaden`] entsteht an genau einer Stelle, in [`faden()`], und zwar in
//! einem frisch gestarteten Faden, dem es als Argument hineingereicht wird. Es
//! traegt `PhantomData<*const ()>` und ist damit weder `Send` noch `Sync`:
//! weder das Pfand noch ein Verweis darauf kann den Faden verlassen, in dem es
//! entstanden ist, und ein Abschluss fuer `DispatchQueue::main()` verlangt
//! `Send`. **Eine Anfrage vom Hauptfaden aus uebersetzt deshalb nicht.** Das
//! ist die Antwort auf die Frage, an der L1 und L9 aus C8 haengen, sobald ein
//! Ordner auf einem Server liegt (Entscheid
//! `261016-1956-welche-form-hat-ein-ort-im-dateifenster-und-wie-haelt-der-bau-fernaufrufe-vom-hauptfaden-fern.md`,
//! Moeglichkeit 1).
//!
//! **Das Pfand haelt seit dem Archivpaket mehr als den Netzweg**: jeden
//! Zugriff, dessen Dauer die Groesse einer fremden Datei bestimmt. So verlangt
//! [`crate::archiv::Archivzugang::verzeichnis`] es ebenfalls, denn das
//! Inhaltsverzeichnis eines grossen tar.gz entsteht erst nach einem Durchlauf
//! durch den ganzen Strom (E9 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`). Der Name
//! bleibt der aus dem sftp-Paket.
//!
//! ```
//! use krk_core::fern::{Fernfaden, faden};
//!
//! fn anfragen(_: &Fernfaden) -> u32 { 7 }
//!
//! let lauf = faden("krk-beispiel".into(), |pfand| anfragen(pfand)).unwrap();
//! assert_eq!(lauf.join().unwrap(), 7);
//! ```
//!
//! Das Pfand laesst sich nicht in einen Abschluss bewegen, der `Send` verlangt,
//! etwa den eines weiteren Fadens oder der Hauptschlange:
//!
//! ```compile_fail,E0277
//! use krk_core::fern::{Fernfaden, faden};
//!
//! fn braucht_send<T: Send>(_: T) {}
//!
//! let _ = faden("krk-beispiel".into(), |pfand: &Fernfaden| {
//!     braucht_send(pfand);
//! });
//! ```
//!
//! # Was das Pfand nicht haelt
//!
//! Es haelt den Hauptfaden fern, nicht jede Wartezeit: ein Arbeitsfaden, der
//! mit seinem Pfand auf eine Antwort wartet, wartet so lange, wie die
//! Gegenseite braucht. Dagegen stehen die Zeitgrenzen von `ssh`
//! (`ConnectTimeout`, `ServerAliveInterval`) und die Abbruchfragen der Rufer,
//! nicht das Pfand. Und es haelt nur Anfragen, die ueber
//! [`super::Gespraech`] gehen; ein Weg, der die Pipes eines `ssh` an diesem
//! Modul vorbei selbst liest, braeuchte kein Pfand. Dass das Pfand nur hier
//! entsteht, haelt die Probe `fernfaden_entsteht_an_genau_einer_stelle` in
//! `tests/fern.rs`; der Uebersetzer haelt es fuer jede Datei ausser dieser,
//! weil das Feld privat ist.

use std::io;
use std::marker::PhantomData;
use std::thread::{self, JoinHandle};

/// Das Pfand eines Arbeitsfadens, der Anfragen an einen Server stellen darf.
///
/// Weder `Send` noch `Sync`, weder `Clone` noch `Default`: es gibt genau so
/// viele, wie [`faden()`] Faeden gestartet hat.
pub struct Fernfaden {
    _nicht_send: PhantomData<*const ()>,
}

/// Startet einen Faden mit dem Namen `name` und reicht ihm ein
/// [`Fernfaden`] hinein.
///
/// Der Rueckgabewert ist der des Fadenstarts: ein Fehler heisst, dass das
/// System keinen Faden hergibt, und der Rufer meldet ihn, statt zu warten.
pub fn faden<T, F>(name: String, arbeit: F) -> io::Result<JoinHandle<T>>
where
    T: Send + 'static,
    F: FnOnce(&Fernfaden) -> T + Send + 'static,
{
    thread::Builder::new().name(name).spawn(move || {
        let pfand = Fernfaden {
            _nicht_send: PhantomData,
        };
        arbeit(&pfand)
    })
}
