//! Der Kanal zu einem Server: der Aufruf von `/usr/bin/ssh -s … sftp`.
//!
//! [`ssh_plan`] ist eine **reine Funktion** und rechnet aus Ziel und Anmeldung
//! Programm, Argumente und Umgebung des Kindprozesses aus; [`Kanalplan::starten`]
//! startet ihn mit drei Pipes. Getrennt sind die beiden, damit der Aufruf ohne
//! Netz pruefbar ist und die Proben denselben Start mit einem anderen Plan
//! fahren koennen, etwa auf `/usr/libexec/sftp-server` (Entscheidung 5 des
//! Plans `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`).
//!
//! ```text
//!  /usr/bin/ssh -o StrictHostKeyChecking=yes -o ConnectTimeout=10
//!    -o ServerAliveInterval=15 -o ServerAliveCountMax=3
//!    -o ForwardAgent=no -o ForwardX11=no -o ClearAllForwardings=yes
//!    -o PermitLocalCommand=no -o RemoteCommand=none -o RequestTTY=no
//!    [mit Passwort:  -o BatchMode=no -o PubkeyAuthentication=no
//!                    -o PreferredAuthentications=keyboard-interactive,password
//!                    -o NumberOfPasswordPrompts=1]
//!    [ohne Passwort: -o BatchMode=yes]
//!    [-p <port>] [-l <konto>] -s -- <rechner> sftp
//! ```
//!
//! **`RemoteCommand=none` und `RequestTTY=no` stehen ueber dem Plan hinaus da**,
//! aus demselben Grund wie die vier Optionen gegen Weiterleitungen: setzt
//! `~/.ssh/config` fuer den Rechner einen `RemoteCommand`, liefe er statt des
//! Teilsystems, und `sftp(1)` setzt beide aus genau diesem Grund ebenso.
//!
//! # Das Passwort steht nirgends im Aufruf
//!
//! Mit Passwort traegt die Umgebung des Kindes `SSH_ASKPASS` (das Binaerziel
//! `krk` selbst), `SSH_ASKPASS_REQUIRE=force` und [`HILFE_VARIABLE`] mit dem
//! Pfad eines Sockets; `ssh` startet das Programm, wenn es nach dem Passwort
//! fragt, und das Programm holt es sich am Socket ab (`krk-ui/src/anmeldehilfe.rs`).
//! **Kein Argument und kein Wert der Umgebung traegt das Passwort**, und
//! [`ssh_plan`] bekommt es gar nicht zu sehen. Die Probe
//! `der_ssh_aufruf_traegt_die_festen_optionen_und_nie_ein_passwort` haelt die
//! Form.
//!
//! **Warum jede Abfrage eine Passwortabfrage ist**, ohne dass KRK den
//! Abfragetext von OpenSSH liest, der keine zugesicherte Schnittstelle ist:
//! mit Passwort ist die Anmeldung ueber Schluessel aus, also fragt `ssh` nach
//! keiner Passphrase; `StrictHostKeyChecking=yes` stellt keine Frage zum
//! Rechnerschluessel; ohne Passwort gilt `BatchMode=yes`, und `ssh` fragt gar
//! nicht. Fragt `ssh` ein zweites Mal, etwa nach einem zweiten Faktor, hat die
//! Hilfe nichts mehr zu geben, und die Anmeldung scheitert.
//!
//! `~/.ssh/config` gilt weiter; die Optionen auf der Befehlszeile gehen ihr
//! vor. Der Rechner steht hinter `--`, und [`super::adresse::fernziel`] weist
//! einen Rechner mit fuehrendem `-` ohnehin ab.

use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use super::adresse::Fernziel;

/// Das `ssh` des Systems, mit vollem Pfad: ein `ssh` aus dem `PATH` des
/// Nutzers waere ein anderes Programm mit anderen Optionen.
pub const SSH: &str = "/usr/bin/ssh";

/// Die Umgebungsvariable, die dem Binaerziel `krk` sagt, dass es als
/// Anmeldehilfe laeuft, und an welchem Socket das Passwort liegt.
pub const HILFE_VARIABLE: &str = "KRK_ANMELDEHILFE";

/// Wie sich `ssh` anmeldet.
#[derive(Debug, Clone, Copy)]
pub enum Anmeldung<'a> {
    /// Mit dem Passwort des Eintrags, ueber die Anmeldehilfe.
    Passwort {
        /// Der Socket, an dem die Hilfe das Passwort abholt.
        hilfe: &'a Path,
        /// Das Programm, das `ssh` als `SSH_ASKPASS` startet: das Binaerziel
        /// `krk`.
        programm: &'a Path,
    },
    /// Ohne Passwort: Schluessel und Agent, oder gar nicht.
    OhnePasswort,
}

/// Was gestartet wird: Programm, Argumente und die Umgebung, die zu der des
/// Elternprozesses hinzukommt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kanalplan {
    /// Das Programm mit vollem Pfad.
    pub programm: PathBuf,
    /// Die Argumente, ohne das Programm.
    pub argumente: Vec<OsString>,
    /// Zusaetzliche Umgebungsvariablen.
    pub umgebung: Vec<(OsString, OsString)>,
}

impl Kanalplan {
    /// Ein Plan ohne Argumente und ohne Umgebung, etwa fuer
    /// `/usr/libexec/sftp-server` in den Proben.
    #[must_use]
    pub fn nur(programm: impl Into<PathBuf>) -> Self {
        Kanalplan {
            programm: programm.into(),
            argumente: Vec::new(),
            umgebung: Vec::new(),
        }
    }

    /// Startet den Kindprozess mit drei Pipes.
    ///
    /// Gewartet wird allein auf den Start, nicht auf die Gegenseite: was `ssh`
    /// danach tut, liest das Gespraech.
    pub fn starten(&self) -> io::Result<Child> {
        Command::new(&self.programm)
            .args(&self.argumente)
            .envs(self.umgebung.iter().map(|(name, wert)| (name, wert)))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    }
}

/// Die festen Optionen, die fuer jede Anmeldung gelten.
const FESTE_OPTIONEN: [&str; 10] = [
    "StrictHostKeyChecking=yes",
    "ConnectTimeout=10",
    "ServerAliveInterval=15",
    "ServerAliveCountMax=3",
    "ForwardAgent=no",
    "ForwardX11=no",
    "ClearAllForwardings=yes",
    "PermitLocalCommand=no",
    "RemoteCommand=none",
    "RequestTTY=no",
];

/// Die Optionen der Anmeldung mit Passwort.
const MIT_PASSWORT: [&str; 4] = [
    "BatchMode=no",
    "PubkeyAuthentication=no",
    "PreferredAuthentications=keyboard-interactive,password",
    "NumberOfPasswordPrompts=1",
];

/// Die Option der Anmeldung ohne Passwort.
const OHNE_PASSWORT: [&str; 1] = ["BatchMode=yes"];

/// Der Aufruf von `ssh` fuer ein Ziel und eine Anmeldung (Modulkopf).
#[must_use]
pub fn ssh_plan(ziel: &Fernziel, anmeldung: Anmeldung<'_>) -> Kanalplan {
    let mut argumente: Vec<OsString> = Vec::new();
    let anmeldeoptionen: &[&str] = match anmeldung {
        Anmeldung::Passwort { .. } => &MIT_PASSWORT,
        Anmeldung::OhnePasswort => &OHNE_PASSWORT,
    };
    for option in FESTE_OPTIONEN.iter().chain(anmeldeoptionen) {
        argumente.push("-o".into());
        argumente.push((*option).into());
    }
    if let Some(port) = ziel.port {
        argumente.push("-p".into());
        argumente.push(port.to_string().into());
    }
    if let Some(konto) = &ziel.konto {
        argumente.push("-l".into());
        argumente.push(konto.into());
    }
    argumente.push("-s".into());
    argumente.push("--".into());
    argumente.push(ziel.rechner.as_str().into());
    argumente.push("sftp".into());

    let umgebung = match anmeldung {
        Anmeldung::Passwort { hilfe, programm } => vec![
            ("SSH_ASKPASS".into(), programm.into()),
            ("SSH_ASKPASS_REQUIRE".into(), "force".into()),
            (HILFE_VARIABLE.into(), hilfe.into()),
        ],
        Anmeldung::OhnePasswort => Vec::new(),
    };
    Kanalplan {
        programm: PathBuf::from(SSH),
        argumente,
        umgebung,
    }
}
