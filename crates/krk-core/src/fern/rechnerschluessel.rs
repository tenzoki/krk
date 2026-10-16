//! Ob `ssh` den Rechnerschluessel eines Servers schon kennt, gefragt bevor
//! KRK verbindet (Entscheidung 6 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`).
//!
//! ```text
//!  vorpruefen(ziel)                     Faden krk-verbinden
//!    ssh -G [-p port] [-l konto] -- rechner
//!      ──> hostname, port, hostkeyalias, userknownhostsfile, globalknownhostsfile
//!    suchname: hostkeyalias | hostname (Port 22) | [hostname]:port
//!    je vorhandene Datei: ssh-keygen -F <suchname> -f <datei>, Ausgang 0 = bekannt
//! ```
//!
//! **Verbunden wird immer mit `StrictHostKeyChecking=yes`**, und `ssh` scheitert
//! an einem unbekannten Schluessel, ohne zu fragen. Die Vorpruefung sagt
//! vorher, ob genau das geschehen wird, damit KRK den Rechner nennen kann,
//! statt eine Zeile der Fehlerausgabe von `ssh` weiterzureichen. Bis das Blatt
//! fuer einen unbekannten Schluessel steht (Schritt 8 des Plans), endet ein
//! unbekannter Rechner mit einer Meldung, die ihn nennt.
//!
//! **Gelesen werden zwei dokumentierte Ausgaben und kein Fliesstext:** die
//! Zeilen von `ssh -G` sind Schluesselwort und Wert (`man ssh`, `-G`), und
//! `ssh-keygen -F` antwortet mit seinem Ausgang (`man ssh-keygen`, `-F`).
//! Gehashte Eintraege in `known_hosts` findet `ssh-keygen` selbst; KRK
//! rechnet keinen Hash.
//!
//! # Was die Vorpruefung nicht sieht
//!
//! Einen Eintrag mit `@revoked` oder `@cert-authority`: `ssh-keygen -F` findet
//! ihn, `ssh` nimmt den Schluessel damit trotzdem nicht an, und die Anmeldung
//! scheitert dann mit der Zeile von `ssh`. Und einen geaenderten Schluessel:
//! der Name steht in der Datei, die Vorpruefung sagt „bekannt“, und `ssh`
//! scheitert an der Abweichung; auch das meldet die Zeile von `ssh`. KRK bietet
//! kein Ueberschreiben an.

use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::adresse::Fernziel;
use super::kanal::SSH;

/// `ssh-keygen` des Systems, mit vollem Pfad.
pub const SSH_KEYGEN: &str = "/usr/bin/ssh-keygen";

/// Was `ssh -G` fuer einen Rechner ausrechnet, soweit die Vorpruefung es
/// braucht.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Sshkonfiguration {
    /// Der Rechner, mit dem `ssh` spricht, nach `~/.ssh/config`.
    pub hostname: String,
    /// Der Port, nach `~/.ssh/config`.
    pub port: u16,
    /// `HostKeyAlias`, wenn gesetzt.
    pub hostkeyalias: Option<String>,
    /// Die Dateien, in denen `ssh` bekannte Schluessel sucht, die des Nutzers
    /// zuerst.
    pub dateien: Vec<PathBuf>,
}

/// Wie es um den Schluessel eines Rechners steht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Schluesselstand {
    /// Eine der Dateien fuehrt ihn.
    Bekannt,
    /// Keine fuehrt ihn; `ssh` wuerde an ihm scheitern.
    Unbekannt {
        /// Der Name, unter dem gesucht wurde: der Alias, der Rechner oder
        /// `[rechner]:port`.
        suchname: String,
    },
}

/// Fragt `ssh -G` und `ssh-keygen -F`, ob der Rechner des Ziels bekannt ist.
///
/// Laeuft auf einem Arbeitsfaden: es startet Prozesse und wartet auf sie,
/// auch wenn keiner davon ins Netz geht. Ein Fehler heisst, dass eine der zwei
/// Fragen nicht beantwortet wurde; der Text nennt, welche.
pub fn vorpruefen(ziel: &Fernziel) -> Result<Schluesselstand, String> {
    let konfiguration = konfiguration_erfragen(ziel)?;
    let suchname = suchname(&konfiguration);
    for datei in &konfiguration.dateien {
        if !datei.is_file() {
            continue;
        }
        match in_datei_bekannt(&suchname, datei) {
            Ok(true) => return Ok(Schluesselstand::Bekannt),
            Ok(false) => {}
            Err(fehler) => return Err(format!("{SSH_KEYGEN} -F: {fehler}")),
        }
    }
    Ok(Schluesselstand::Unbekannt { suchname })
}

/// `ssh -G` mit Port, Konto und Rechner des Ziels.
fn konfiguration_erfragen(ziel: &Fernziel) -> Result<Sshkonfiguration, String> {
    let mut befehl = Command::new(SSH);
    befehl.arg("-G");
    if let Some(port) = ziel.port {
        befehl.arg("-p").arg(port.to_string());
    }
    if let Some(konto) = &ziel.konto {
        befehl.arg("-l").arg(konto);
    }
    befehl.arg("--").arg(&ziel.rechner);
    let ausgabe = befehl
        .stdin(Stdio::null())
        .output()
        .map_err(|fehler| format!("{SSH} -G: {fehler}"))?;
    if !ausgabe.status.success() {
        let zeile = String::from_utf8_lossy(&ausgabe.stderr)
            .lines()
            .map(str::trim)
            .rfind(|zeile| !zeile.is_empty())
            .unwrap_or_default()
            .to_owned();
        return Err(format!("{SSH} -G: {zeile}"));
    }
    let heim = std::env::var_os("HOME").map(PathBuf::from);
    konfiguration_lesen(&String::from_utf8_lossy(&ausgabe.stdout), heim.as_deref())
        .ok_or_else(|| format!("{SSH} -G: ohne hostname oder port"))
}

/// Liest die Zeilen von `ssh -G`: je Zeile ein Schluesselwort in
/// Kleinschreibung, ein Leerzeichen und der Wert. `None`, wenn `hostname`
/// oder `port` fehlt oder der Port keine Zahl ist.
///
/// Ein Dateiname mit `~/` davor wird gegen `heim` aufgeloest; `ssh` 9.9 gibt
/// ihn schon aufgeloest aus, eine aeltere Fassung tat das nicht. `none` als
/// `hostkeyalias` heisst: keiner.
#[must_use]
pub fn konfiguration_lesen(text: &str, heim: Option<&Path>) -> Option<Sshkonfiguration> {
    let mut hostname = None;
    let mut port = None;
    let mut hostkeyalias = None;
    let mut eigene = Vec::new();
    let mut globale = Vec::new();
    let aufloesen = |name: &str| -> PathBuf {
        match (name.strip_prefix("~/"), heim) {
            (Some(rest), Some(heim)) => heim.join(rest),
            _ => PathBuf::from(name),
        }
    };
    for zeile in text.lines() {
        let Some((wort, wert)) = zeile.split_once(' ') else {
            continue;
        };
        match wort {
            "hostname" => hostname = Some(wert.to_owned()),
            "port" => port = wert.parse::<u16>().ok(),
            "hostkeyalias" if wert != "none" => hostkeyalias = Some(wert.to_owned()),
            "userknownhostsfile" => {
                eigene = wert.split_whitespace().map(aufloesen).collect();
            }
            "globalknownhostsfile" => {
                globale = wert.split_whitespace().map(aufloesen).collect();
            }
            _ => {}
        }
    }
    eigene.extend(globale);
    Some(Sshkonfiguration {
        hostname: hostname?,
        port: port?,
        hostkeyalias,
        dateien: eigene,
    })
}

/// Der Name, unter dem `ssh` den Schluessel in `known_hosts` sucht: der
/// Alias, wenn einer gesetzt ist, sonst der Rechner bei Port 22 und
/// `[rechner]:port` bei jedem anderen.
#[must_use]
pub fn suchname(konfiguration: &Sshkonfiguration) -> String {
    match (&konfiguration.hostkeyalias, konfiguration.port) {
        (Some(alias), _) => alias.clone(),
        (None, 22) => konfiguration.hostname.clone(),
        (None, port) => format!("[{}]:{port}", konfiguration.hostname),
    }
}

/// Ob `ssh-keygen -F <suchname> -f <datei>` einen Eintrag findet: Ausgang 0
/// heisst ja, 1 nein, jeder andere ist ein Fehler.
pub fn in_datei_bekannt(suchname: &str, datei: &Path) -> io::Result<bool> {
    let stand = Command::new(SSH_KEYGEN)
        .arg("-F")
        .arg(suchname)
        .arg("-f")
        .arg(datei)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    match stand.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        anders => Err(io::Error::other(format!("Ausgang {anders:?}"))),
    }
}
