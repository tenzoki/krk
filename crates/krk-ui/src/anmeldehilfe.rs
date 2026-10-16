//! Die Anmeldehilfe: wie das Passwort einer sftp-Verbindung an `ssh` kommt,
//! ohne in einem Befehl, einer Datei, einem Argument oder einer
//! Umgebungsvariable zu stehen (Entscheidung 5 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`, Entscheid
//! `261016-1947-darf-das-passwort-einer-verbindung-krk-ausser-an-netfs-auch-an-ssh-gehen.md`).
//!
//! ```text
//!  KRK, Faden krk-verbinden                 ssh (Kindprozess)
//!  passwort_reichen(passwort, ort)
//!    Ordner 0700, Socket darin
//!    Faden krk-anmeldehilfe: accept, einmal ─┐
//!  ssh_plan: SSH_ASKPASS=<krk>,              │   fragt nach dem Passwort:
//!            SSH_ASKPASS_REQUIRE=force,      │   startet <krk> "<Abfragetext>"
//!            KRK_ANMELDEHILFE=<socket>       │     main: rolle() ──> fragen
//!                                            └──────── abholen(socket)
//!                                                      Passwort + "\n" auf stdout
//!  Anmeldehilfe faellt: Faden endet, Socket und Ordner fallen
//! ```
//!
//! **Zwei Haelften in einer Datei**, weil beide dieselbe Abmachung ueber den
//! Socket teilen: [`passwort_reichen`] laeuft in KRK und gibt das Passwort
//! **hoechstens einmal** heraus, [`rolle`] laeuft im selben Binaerziel, wenn
//! `ssh` es als `SSH_ASKPASS` startet, holt es ab und schreibt es auf die
//! Standardausgabe, von der `ssh` es liest. `main` fragt [`rolle`] als erste
//! Handlung, vor jeder Beruehrung von AppKit und der Sprache; die Probe
//! `main_prueft_die_anmeldehilfe_vor_jeder_andern_handlung` haelt das.
//!
//! # Der zweite Rufer von `Passwort::preisgeben`
//!
//! Der Entscheid erlaubt genau einen weiteren Weg heraus, und das ist der
//! Rumpf von [`passwort_reichen`]. Die Probe
//! `das_passwort_wird_allein_an_netfs_und_ssh_preisgegeben`
//! (`appkit/verbinden.rs`) zaehlt die Nennungen in allen Kisten und verlangt
//! diese eine hier und die eine in `einhaengen`.
//!
//! # Was der Socket haelt und was nicht
//!
//! Der Socket liegt in einem frisch angelegten Ordner mit den Rechten `0700`
//! unter dem Temporaerverzeichnis des Nutzers ([`hilfeort`]); ein anderes Konto
//! dieses Rechners erreicht ihn nicht. **Ein Prozess desselben Kontos kann ihn
//! erreichen**, und wer zuerst fragt, bekommt das Passwort; danach bekommt
//! niemand mehr etwas, auch `ssh` nicht, und die Anmeldung scheitert sichtbar.
//! Wer den Rufer am Socket pruefen wollte, braeuchte `getpeereid(2)`, und das
//! kennt `std` auf diesem System nicht ohne `unsafe`. Ein Prozess desselben
//! Kontos koennte ebenso gut `secrets.txt` lesen, sobald die PIN gemerkt ist.
//!
//! **Getilgt** wird KRKs Abschrift des Passworts (`Zeroizing`), sobald sie
//! gesendet ist oder die Hilfe faellt. Nicht getilgt werden die Bytes im Puffer
//! des Sockets, die Abschrift im Hilfsprozess, der danach endet, und was `ssh`
//! selbst damit tut; `HowTo.md` nennt das unter „Was getilgt wird und was
//! nicht“.
//!
//! Aufgeraeumt wird mit `remove_file` und `remove_dir` und nicht mit dem
//! Abraeumen eines ganzen Baums: der Ordner traegt allein den Socket, und die
//! Zaehlprobe der Pruefordner-Fassungen (`crates/krk-core/tests/baum.rs`) soll
//! diese Datei nicht fuer eine vierte halten.

use std::io::{self, Read, Write};
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread::JoinHandle;

use krk_core::fern::kanal::HILFE_VARIABLE;
use krk_core::heimordner::eintraege::geheimnisse::Passwort;
use zeroize::Zeroizing;

/// Der Name des Sockets in seinem Ordner.
const SOCKETNAME: &str = "s";

/// Die laengste Pfadlaenge eines Unix-Sockets in Bytes: `sun_path` traegt auf
/// macOS 104 Bytes samt der abschliessenden Null.
const SOCKETGRENZE: usize = 103;

/// Wie oft ein vergebener Ordnername zu einem neuen Versuch fuehrt.
const VERSUCHE: u64 = 16;

/// Die Laufnummer, die zwei Hilfen desselben Prozesses trennt.
static ZAEHLER: AtomicU64 = AtomicU64::new(0);

/// Der Ausgang der Hilfsrolle, wenn sie das Passwort geschrieben hat.
const GEGEBEN: i32 = 0;

/// Der Ausgang der Hilfsrolle, wenn sie nichts zu geben hatte; `ssh` gilt die
/// Abfrage dann als abgebrochen.
const NICHTS: i32 = 1;

/// Eine laufende Anmeldehilfe. Faellt sie, endet ihr Faden, und Socket und
/// Ordner fallen.
#[must_use = "fallengelassen schliesst die Hilfe sofort, und ssh bekommt kein Passwort"]
pub struct Anmeldehilfe {
    ordner: PathBuf,
    socket: PathBuf,
    beendet: Arc<AtomicBool>,
    faden: Option<JoinHandle<()>>,
}

impl Anmeldehilfe {
    /// Der Socket, den `ssh` ueber die Hilfsrolle fragt.
    pub fn socket(&self) -> &Path {
        &self.socket
    }
}

impl Drop for Anmeldehilfe {
    fn drop(&mut self) {
        self.beendet.store(true, Ordering::SeqCst);
        // Weckt den Faden, falls er noch auf eine Verbindung wartet; er sieht
        // dann `beendet` und gibt nichts heraus. Steht kein Horcher mehr, weil
        // die eine Verbindung schon kam, scheitert das Verbinden, und das ist
        // recht so.
        let _ = UnixStream::connect(&self.socket);
        if let Some(faden) = self.faden.take() {
            // Ein abgestuerzter Faden hat sein Passwort mit sich getilgt.
            let _ = faden.join();
        }
        // Scheitert eines davon, bleibt ein leerer Ordner mit den Rechten
        // 0700 im Temporaerverzeichnis zurueck; mehr nicht.
        let _ = std::fs::remove_file(&self.socket);
        let _ = std::fs::remove_dir(&self.ordner);
    }
}

/// Der Ort, unter dem [`passwort_reichen`] seinen Ordner anlegt: das
/// Temporaerverzeichnis des Nutzers, oder `~/Library/Caches/KRK`, wenn der
/// Socket darunter laenger wuerde, als ein Unix-Socket sein darf.
#[must_use]
pub fn hilfeort() -> PathBuf {
    let temp = std::env::temp_dir();
    // Der laengste Name, den `ordner_anlegen` bildet, samt Socket.
    let probe = temp.join(format!("krk-{}-{}", std::process::id(), u64::MAX));
    if probe.join(SOCKETNAME).as_os_str().len() <= SOCKETGRENZE {
        return temp;
    }
    std::env::var_os("HOME")
        .map(|heim| PathBuf::from(heim).join("Library/Caches/KRK"))
        .unwrap_or(temp)
}

/// Legt unter `ort` einen Ordner mit den Rechten `0700` und darin einen
/// Socket an und gibt das Passwort an die **erste** Verbindung, die dort
/// ankommt, auf dem Faden `krk-anmeldehilfe`. Jede weitere bekommt nichts.
///
/// **Der zweite Rufer von `Passwort::preisgeben`** (Modulkopf). Ein Fehler
/// heisst, dass Ordner, Socket oder Faden nicht entstanden; das Passwort ist
/// dann schon getilgt, und nichts bleibt liegen.
pub fn passwort_reichen(passwort: Passwort, ort: &Path) -> io::Result<Anmeldehilfe> {
    let ordner = ordner_anlegen(ort)?;
    let socket = ordner.join(SOCKETNAME);
    let horcher = if socket.as_os_str().len() > SOCKETGRENZE {
        Err(io::Error::other(format!(
            "der Socketpfad ist laenger als {SOCKETGRENZE} Bytes: {}",
            socket.display()
        )))
    } else {
        UnixListener::bind(&socket)
    };
    let horcher = match horcher {
        Ok(horcher) => horcher,
        Err(fehler) => {
            let _ = std::fs::remove_dir(&ordner);
            return Err(fehler);
        }
    };
    let beendet = Arc::new(AtomicBool::new(false));
    let fuer_den_faden = Arc::clone(&beendet);
    let klartext = passwort.preisgeben();
    let gestartet = std::thread::Builder::new()
        .name("krk-anmeldehilfe".to_owned())
        .spawn(move || antworten(horcher, &fuer_den_faden, klartext));
    match gestartet {
        Ok(faden) => Ok(Anmeldehilfe {
            ordner,
            socket,
            beendet,
            faden: Some(faden),
        }),
        Err(fehler) => {
            let _ = std::fs::remove_file(&socket);
            let _ = std::fs::remove_dir(&ordner);
            Err(fehler)
        }
    }
}

/// Ein frischer Ordner `krk-<prozess>-<laufnummer>` mit den Rechten `0700`;
/// ein vergebener Name fuehrt zum naechsten, nie zu einem vorhandenen Ordner.
fn ordner_anlegen(ort: &Path) -> io::Result<PathBuf> {
    std::fs::create_dir_all(ort)?;
    let mut letzter = None;
    for _ in 0..VERSUCHE {
        let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
        let ordner = ort.join(format!("krk-{}-{laufnummer}", std::process::id()));
        match std::fs::DirBuilder::new().mode(0o700).create(&ordner) {
            Ok(()) => return Ok(ordner),
            Err(fehler) if fehler.kind() == io::ErrorKind::AlreadyExists => letzter = Some(fehler),
            Err(fehler) => return Err(fehler),
        }
    }
    Err(letzter.unwrap_or_else(|| io::Error::other("kein freier Ordnername")))
}

/// Der Rumpf des Fadens `krk-anmeldehilfe`: genau eine Verbindung annehmen,
/// und ihr das Passwort geben, wenn die Hilfe bis dahin nicht beendet ist.
fn antworten(horcher: UnixListener, beendet: &AtomicBool, klartext: Zeroizing<String>) {
    let Ok((mut strom, _)) = horcher.accept() else {
        return;
    };
    // Ab hier nimmt niemand mehr an: eine zweite Frage scheitert am Verbinden.
    drop(horcher);
    if beendet.load(Ordering::SeqCst) {
        return;
    }
    // Scheitert das Schreiben, bekommt die Hilfsrolle nichts, und `ssh` meldet
    // die Anmeldung als gescheitert; einen zweiten Versuch gibt es nicht.
    let _ = strom.write_all(klartext.as_bytes());
}

/// Holt das Passwort am Socket ab; die Haelfte der Hilfsrolle.
///
/// Ein leerer Text heisst, dass die Hilfe nichts mehr gibt, und ist ein
/// Fehler.
pub fn abholen(socket: &Path) -> io::Result<Zeroizing<String>> {
    let mut strom = UnixStream::connect(socket)?;
    let mut text = Zeroizing::new(String::new());
    strom.read_to_string(&mut text)?;
    if text.is_empty() {
        return Err(io::Error::other("die Anmeldehilfe gibt nichts mehr"));
    }
    Ok(text)
}

/// Die Hilfsrolle des Binaerziels: `None`, wenn [`HILFE_VARIABLE`] nicht
/// gesetzt ist, sonst der Ausgang, mit dem der Prozess endet.
///
/// Gesetzt ist sie allein in der Umgebung eines `ssh`, das KRK selbst
/// gestartet hat; das Argument, mit dem `ssh` die Hilfe ruft, ist der
/// Abfragetext, und den liest sie nicht.
#[must_use]
pub fn rolle() -> Option<i32> {
    let socket = std::env::var_os(HILFE_VARIABLE)?;
    Some(fragen(Path::new(&socket)))
}

/// Holt das Passwort und schreibt es mit Zeilenende auf die Standardausgabe.
fn fragen(socket: &Path) -> i32 {
    let Ok(text) = abholen(socket) else {
        return NICHTS;
    };
    let mut ausgabe = io::stdout().lock();
    let geschrieben = ausgabe
        .write_all(text.as_bytes())
        .and_then(|()| ausgabe.write_all(b"\n"))
        .and_then(|()| ausgabe.flush());
    match geschrieben {
        Ok(()) => GEGEBEN,
        Err(_) => NICHTS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pruefordner::Pruefordner;

    /// Die Hilfe gibt das Passwort an die erste Verbindung und an keine
    /// zweite; der Ordner traegt die Rechte 0700; faellt die Hilfe, sind
    /// Socket und Ordner fort. Ueber einen echten Socket, ohne `ssh`.
    #[test]
    fn die_anmeldehilfe_gibt_hoechstens_einmal_heraus() {
        use std::os::unix::fs::PermissionsExt;

        let ort = Pruefordner::neu("hilfe");
        let hilfe = passwort_reichen(Passwort::neu("geheim-4711".to_owned()), ort.pfad())
            .expect("die Hilfe startet");
        let socket = hilfe.socket().to_path_buf();
        let ordner = socket.parent().expect("der Socket liegt in einem Ordner");
        let rechte = std::fs::metadata(ordner)
            .expect("der Ordner steht")
            .permissions()
            .mode();
        assert_eq!(rechte & 0o777, 0o700);

        let erste = abholen(&socket).expect("die erste Frage bekommt das Passwort");
        assert_eq!(erste.as_str(), "geheim-4711");
        assert!(
            abholen(&socket).is_err(),
            "eine zweite Frage bekommt nichts"
        );

        let ordner = ordner.to_path_buf();
        drop(hilfe);
        assert!(!socket.exists(), "der Socket faellt mit der Hilfe");
        assert!(!ordner.exists(), "der Ordner faellt mit der Hilfe");
    }

    /// Faellt die Hilfe, bevor jemand gefragt hat, endet ihr Faden, und eine
    /// spaete Frage bekommt nichts.
    #[test]
    fn eine_ungefragte_hilfe_gibt_nach_dem_fallen_nichts() {
        let ort = Pruefordner::neu("hilfe-leer");
        let hilfe = passwort_reichen(Passwort::neu("geheim-4711".to_owned()), ort.pfad())
            .expect("die Hilfe startet");
        let socket = hilfe.socket().to_path_buf();
        drop(hilfe);
        assert!(abholen(&socket).is_err());
    }

    /// `main` fragt die Hilfsrolle als erste Handlung, vor der Auswertung der
    /// Befehlszeile, der Sprache und AppKit.
    ///
    /// **Was sie nicht sieht:** ob eine statische Initialisierung vor `main`
    /// laeuft; dieses Binaerziel hat keine.
    #[test]
    fn main_prueft_die_anmeldehilfe_vor_jeder_andern_handlung() {
        let quelle = include_str!("main.rs");
        let rumpf = quelle
            .split_once("\nfn main() {\n")
            .map(|(_, rest)| rest)
            .expect("main steht in main.rs");
        let erste = crate::quellbaum::codezeilen(rumpf)
            .map(str::trim)
            .find(|zeile| !zeile.is_empty())
            .expect("main hat einen Rumpf");
        assert_eq!(
            erste,
            concat!("if let Some(ausgang) = anmeldehilfe::", "rolle() {")
        );
    }
}
