//! Ob `ssh` den Rechnerschluessel eines Servers schon kennt, gefragt bevor
//! KRK verbindet, und wie ein unbekannter angenommen wird (Entscheidung 6 des
//! Plans `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`,
//! Schritte 2 und 8).
//!
//! ```text
//!  vorpruefen(ziel)                     Faden krk-verbinden
//!    ssh -G [-p port] [-l konto] -- rechner
//!      ──> hostname, port, hostkeyalias, userknownhostsfile, globalknownhostsfile
//!    suchname: hostkeyalias | hostname (Port 22) | [hostname]:port
//!    je vorhandene Datei: ssh-keygen -F <suchname> -f <datei>, Ausgang 0 = bekannt
//!
//!  unbekannt:
//!  scannen(pfand, konfiguration, suchname)
//!    ssh-keyscan -T 10 -p <port> -- <hostname>   ──> <name> <typ> <schluessel>
//!    je Zeile: ssh-keygen -l -E sha256 -f -       ──> SHA256:… (TYP)
//!      ──> Gescannt: Rechner, Port, Zeilen, Fingerabdruecke, Zieldatei
//!  Blatt im Hauptfaden: Annehmen | Abbrechen
//!  anhaengen(pfand, gescannt)                     Faden krk-verbinden
//!    erste userknownhostsfile, Ordner 0700, Datei 0600, nur anhaengen
//! ```
//!
//! **Verbunden wird immer mit `StrictHostKeyChecking=yes`**, und `ssh` scheitert
//! an einem unbekannten Schluessel, ohne zu fragen. Die Vorpruefung sagt
//! vorher, ob genau das geschehen wird, damit KRK den Rechner nennen und seine
//! Schluessel zur Annahme vorlegen kann, statt eine Zeile der Fehlerausgabe von
//! `ssh` weiterzureichen. `accept-new` setzt KRK nie: es naehme einen
//! Schluessel an, den der Nutzer nie gesehen hat.
//!
//! **Gelesen werden dokumentierte Ausgaben und kein Fliesstext:** die
//! Zeilen von `ssh -G` sind Schluesselwort und Wert (`man ssh`, `-G`),
//! `ssh-keygen -F` antwortet mit seinem Ausgang (`man ssh-keygen`, `-F`), eine
//! Zeile von `ssh-keyscan` ist eine Zeile im Format von `known_hosts`
//! (`man ssh-keyscan`, OUTPUT FORMAT), und `ssh-keygen -l` nennt je Schluessel
//! Bits, Fingerabdruck, Kommentar und in Klammern den Typ. Gehashte Eintraege
//! in `known_hosts` findet `ssh-keygen` selbst; KRK rechnet keinen Hash und
//! keinen Fingerabdruck.
//!
//! # Was angehaengt wird, ist genau das, was das Blatt zeigt
//!
//! [`Gescannt`] haelt die Zeilen, aus denen die Fingerabdruecke gerechnet
//! sind, und seine Felder sind privat: zwischen dem Blatt und dem Anhaengen
//! kann niemand eine Zeile tauschen. Jede Zeile ist `<suchname> <typ>
//! <schluessel>`, also das Format, das `ssh-keyscan` liefert, mit dem Namen,
//! unter dem `ssh` sucht, an der ersten Stelle; ohne `HostKeyAlias` ist das
//! woertlich die Zeile von `ssh-keyscan`. Danach verbindet KRK wieder mit
//! `StrictHostKeyChecking=yes`, und `ssh` prueft gegen genau die gezeigten
//! Schluessel: ein Rechner, der zwischen Scan und Verbindung einen anderen
//! vorzeigt, scheitert.
//!
//! **Der Name an der ersten Stelle nennt genau einen Rechner.** `known_hosts`
//! liest das erste Feld als Liste von Mustern: `,` trennt, `*` und `?` sind
//! Platzhalter, `!` verneint. Ein Suchname mit einem dieser Zeichen schriebe
//! eine Zeile, die den angenommenen Schluessel fuer fremde Rechner gelten
//! liesse, und zwar fuer jeden spaeteren `ssh`-Aufruf des Nutzers, nicht nur
//! in KRK. [`gescannt_aus`] weist deshalb jeden Suchnamen ab, der nicht
//! `rechner` oder `[rechner]:port` mit einem Rechner nach
//! [`super::adresse::rechnername_gueltig`] ist, und [`anhaengen`] haelt jede
//! Zeile vor dem Schreiben noch einmal dagegen. Die Adresse des Eintrags hat
//! dieselbe Regel schon beim Zerlegen bestanden; geprueft wird hier trotzdem,
//! weil der Suchname aus `ssh -G` kommt und damit auch aus einem `Hostname`
//! oder `HostKeyAlias` der Konfiguration.
//!
//! **[`anhaengen`] ist der eine Schreiber an `known_hosts`**, gehalten von der
//! Probe `known_hosts_hat_genau_einen_schreiber` (`tests/baum.rs`). Er
//! haengt an und schreibt die Datei nie neu; fehlen Ordner oder Datei, legt er
//! sie mit den Rechten an, die `ssh` fuer sie erwartet (`0700`, `0600`).
//! **Scannen und Anhaengen verlangen das Pfand [`Fernfaden`]**: beide warten
//! auf das Netz oder die Platte und erreichen damit den Hauptfaden nicht.
//!
//! # Was die Vorpruefung nicht sieht
//!
//! Einen Eintrag mit `@revoked` oder `@cert-authority`: `ssh-keygen -F` findet
//! ihn, `ssh` nimmt den Schluessel damit trotzdem nicht an, und die Anmeldung
//! scheitert dann mit der Zeile von `ssh`. Und einen geaenderten Schluessel:
//! der Name steht in der Datei, die Vorpruefung sagt „bekannt“, und `ssh`
//! scheitert an der Abweichung; auch das meldet die Zeile von `ssh`. KRK bietet
//! kein Ueberschreiben an.
//!
//! # Was der Scan nicht sieht
//!
//! Einen Rechner, den `ssh` nur ueber `ProxyJump` oder `ProxyCommand` aus
//! `~/.ssh/config` erreicht: `ssh-keyscan` kennt beides nicht und fragt den
//! Rechner unmittelbar. Dann scheitert der Scan, und KRK meldet es mit der
//! Zeile von `ssh-keyscan`; der Schluessel bleibt eine Handlung des Nutzers in
//! `known_hosts`.

use std::fs::{DirBuilder, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::adresse::{Fernziel, rechnername_gueltig};
use super::faden::Fernfaden;
use super::kanal::SSH;

/// `ssh-keygen` des Systems, mit vollem Pfad.
pub const SSH_KEYGEN: &str = "/usr/bin/ssh-keygen";

/// `ssh-keyscan` des Systems, mit vollem Pfad.
pub const SSH_KEYSCAN: &str = "/usr/bin/ssh-keyscan";

/// Wie lange `ssh-keyscan` auf den Rechner wartet, in Sekunden; derselbe Wert
/// wie `ConnectTimeout` beim Verbinden.
pub const SCANFRIST: u32 = 10;

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
    /// Die erste `userknownhostsfile`, wenn sie ein absoluter Pfad ist: die
    /// Datei, an die „Annehmen“ anhaengt. `None` bei `none` oder ohne Angabe.
    pub eigene_datei: Option<PathBuf>,
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
        /// Was `ssh -G` ausgerechnet hat; [`scannen`] fragt damit denselben
        /// Rechner am selben Port, den `ssh` fragen wird.
        konfiguration: Sshkonfiguration,
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
    Ok(Schluesselstand::Unbekannt {
        suchname,
        konfiguration,
    })
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
    let eigene_datei = eigene.first().filter(|datei| datei.is_absolute()).cloned();
    eigene.extend(globale);
    Some(Sshkonfiguration {
        hostname: hostname?,
        port: port?,
        hostkeyalias,
        dateien: eigene,
        eigene_datei,
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

/// Ob `suchname` als erstes Feld einer Zeile in `known_hosts` genau einen
/// Rechner nennt: `rechner` oder `[rechner]:port`, der Rechner nach
/// [`rechnername_gueltig`], der Port eine Zahl von 1 bis 65535 (Modulkopf).
#[must_use]
pub fn suchname_gueltig(suchname: &str) -> bool {
    let Some(rest) = suchname.strip_prefix('[') else {
        return rechnername_gueltig(suchname);
    };
    let Some((rechner, port)) = rest.split_once("]:") else {
        return false;
    };
    rechnername_gueltig(rechner)
        && port.chars().all(|zeichen| zeichen.is_ascii_digit())
        && port.parse::<u16>().is_ok_and(|port| port > 0)
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

/// Ein Fingerabdruck, wie `ssh-keygen -l -E sha256` ihn nennt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fingerabdruck {
    /// Der Typ in der Schreibung von `ssh-keygen`, etwa `ED25519` oder `RSA`.
    pub typ: String,
    /// Der Abdruck samt Verfahren, `SHA256:…`.
    pub abdruck: String,
}

/// Die Schluessel eines unbekannten Rechners, so wie `ssh-keyscan` sie
/// geholt hat, mit ihren Fingerabdruecken und der Datei, an die „Annehmen“
/// sie anhaengt.
///
/// **Die Felder sind privat**, und es entsteht allein in [`gescannt_aus`]:
/// was [`anhaengen`] schreibt, sind genau die Zeilen, deren Fingerabdruecke
/// das Blatt zeigt (Modulkopf).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gescannt {
    rechner: String,
    port: u16,
    suchname: String,
    zeilen: Vec<String>,
    abdruecke: Vec<Fingerabdruck>,
    datei: Option<PathBuf>,
}

impl Gescannt {
    /// Der Rechner, den `ssh-keyscan` gefragt hat (`hostname` aus `ssh -G`).
    #[must_use]
    pub fn rechner(&self) -> &str {
        &self.rechner
    }

    /// Der Port, an dem gefragt wurde.
    #[must_use]
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Der Name, unter dem `ssh` den Schluessel sucht und unter dem er
    /// eingetragen wird.
    #[must_use]
    pub fn suchname(&self) -> &str {
        &self.suchname
    }

    /// Die Zeilen, die [`anhaengen`] schreibt, je ohne Zeilenende.
    #[must_use]
    pub fn zeilen(&self) -> &[String] {
        &self.zeilen
    }

    /// Je Zeile ihr Fingerabdruck, in derselben Reihenfolge.
    #[must_use]
    pub fn fingerabdruecke(&self) -> &[Fingerabdruck] {
        &self.abdruecke
    }

    /// Die Datei, an die angehaengt wird; `None`, wenn `ssh -G` keine
    /// absolute `userknownhostsfile` nennt.
    #[must_use]
    pub fn datei(&self) -> Option<&Path> {
        self.datei.as_deref()
    }
}

/// Holt die Schluessel des Rechners mit `ssh-keyscan` und rechnet ihre
/// Fingerabdruecke; laeuft auf dem Faden `krk-verbinden`.
///
/// Gefragt werden `hostname` und `port` aus `ssh -G`, also derselbe Rechner,
/// den `ssh` danach fragt. Ein Fehler traegt die letzte Zeile der
/// Fehlerausgabe von `ssh-keyscan`, die kein Kommentar ist, oder seinen
/// Ausgang; er geht unuebersetzt in die Statuszeile, wie die Zeilen von `ssh`.
pub fn scannen(
    pfand: &Fernfaden,
    konfiguration: &Sshkonfiguration,
    suchname: &str,
) -> Result<Gescannt, String> {
    let ausgabe = Command::new(SSH_KEYSCAN)
        .arg("-T")
        .arg(SCANFRIST.to_string())
        .arg("-p")
        .arg(konfiguration.port.to_string())
        .arg("--")
        .arg(&konfiguration.hostname)
        .stdin(Stdio::null())
        .output()
        .map_err(|fehler| format!("{SSH_KEYSCAN}: {fehler}"))?;
    let gescannt = gescannt_aus(
        pfand,
        konfiguration,
        suchname,
        &String::from_utf8_lossy(&ausgabe.stdout),
    );
    match gescannt {
        Ok(gescannt) => Ok(gescannt),
        Err(grund) => {
            let fehlerausgabe = String::from_utf8_lossy(&ausgabe.stderr);
            let zeile = fehlerausgabe
                .lines()
                .map(str::trim)
                .rfind(|zeile| !zeile.is_empty() && !zeile.starts_with('#'));
            Err(match zeile {
                Some(zeile) => format!("{SSH_KEYSCAN}: {zeile}"),
                None if !ausgabe.status.success() => {
                    format!("{SSH_KEYSCAN}: exit {:?}", ausgabe.status.code())
                }
                None => grund,
            })
        }
    }
}

/// Baut aus der Ausgabe von `ssh-keyscan` die Zeilen zum Anhaengen und
/// rechnet je Zeile den Fingerabdruck mit `ssh-keygen -l`; der Teil von
/// [`scannen`], der kein Netz braucht.
///
/// Gelesen wird jede Zeile, die kein Kommentar ist und aus genau drei
/// Feldern besteht: Name, Typ und Schluessel. Typ und Schluessel muessen aus
/// den Zeichen bestehen, die `known_hosts` dort traegt, sonst faellt die
/// Zeile weg; der Name wird durch `suchname` ersetzt. Eine Zeile, deren
/// Fingerabdruck `ssh-keygen` nicht rechnen kann, faellt ebenso weg. Ohne
/// eine gueltige Zeile ist das ein Fehler, und ebenso ein `suchname`, der nach
/// [`suchname_gueltig`] mehr als einen Rechner nennen koennte (Modulkopf).
pub fn gescannt_aus(
    _pfand: &Fernfaden,
    konfiguration: &Sshkonfiguration,
    suchname: &str,
    keyscan_ausgabe: &str,
) -> Result<Gescannt, String> {
    if !suchname_gueltig(suchname) {
        return Err(format!(
            "{SSH_KEYSCAN}: {suchname:?} is not a single host name for known_hosts"
        ));
    }
    let mut zeilen = Vec::new();
    let mut abdruecke = Vec::new();
    for zeile in keyscan_ausgabe.lines() {
        let Some((typ, schluessel)) = schluesselzeile(zeile) else {
            continue;
        };
        let zeile = format!("{suchname} {typ} {schluessel}");
        let Some(abdruck) = fingerabdruck(&zeile)? else {
            continue;
        };
        if zeilen.contains(&zeile) {
            continue;
        }
        zeilen.push(zeile);
        abdruecke.push(abdruck);
    }
    if zeilen.is_empty() {
        return Err(format!(
            "{SSH_KEYSCAN}: no keys from {}:{}",
            konfiguration.hostname, konfiguration.port
        ));
    }
    Ok(Gescannt {
        rechner: konfiguration.hostname.clone(),
        port: konfiguration.port,
        suchname: suchname.to_owned(),
        zeilen,
        abdruecke,
        datei: konfiguration.eigene_datei.clone(),
    })
}

/// Typ und Schluessel einer Zeile von `ssh-keyscan`, oder `None` fuer einen
/// Kommentar, eine leere Zeile und jede Zeile, die nicht aus genau drei
/// Feldern mit den erlaubten Zeichen besteht.
fn schluesselzeile(zeile: &str) -> Option<(&str, &str)> {
    let zeile = zeile.trim();
    if zeile.starts_with('#') {
        return None;
    }
    let mut felder = zeile.split_whitespace();
    let (_name, typ, schluessel) = (felder.next()?, felder.next()?, felder.next()?);
    if felder.next().is_some() {
        return None;
    }
    let typ_gueltig = typ
        .chars()
        .all(|zeichen| zeichen.is_ascii_alphanumeric() || "-@.".contains(zeichen));
    let schluessel_gueltig = schluessel
        .chars()
        .all(|zeichen| zeichen.is_ascii_alphanumeric() || "+/=".contains(zeichen));
    (typ_gueltig && schluessel_gueltig).then_some((typ, schluessel))
}

/// Der Fingerabdruck einer Zeile im Format von `known_hosts`, gerechnet von
/// `ssh-keygen -l -E sha256 -f -`; `Ok(None)`, wenn `ssh-keygen` die Zeile
/// nicht als Schluessel liest, ein Fehler, wenn es sich nicht starten laesst.
///
/// Die Ausgabe ist `<bits> SHA256:<abdruck> <kommentar> (<TYP>)`; gelesen
/// werden das zweite Feld und das letzte ohne Klammern.
fn fingerabdruck(zeile: &str) -> Result<Option<Fingerabdruck>, String> {
    let fehler = |fehler: io::Error| format!("{SSH_KEYGEN} -l: {fehler}");
    let mut kind = Command::new(SSH_KEYGEN)
        .args(["-l", "-E", "sha256", "-f", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(fehler)?;
    if let Some(mut eingabe) = kind.stdin.take() {
        eingabe
            .write_all(format!("{zeile}\n").as_bytes())
            .map_err(fehler)?;
    }
    let ausgabe = kind.wait_with_output().map_err(fehler)?;
    if !ausgabe.status.success() {
        return Ok(None);
    }
    let text = String::from_utf8_lossy(&ausgabe.stdout);
    let Some(zeile) = text.lines().next() else {
        return Ok(None);
    };
    let felder: Vec<&str> = zeile.split_whitespace().collect();
    let (Some(abdruck), Some(typ)) = (felder.get(1), felder.last()) else {
        return Ok(None);
    };
    let Some(typ) = typ.strip_prefix('(').and_then(|typ| typ.strip_suffix(')')) else {
        return Ok(None);
    };
    if felder.len() < 3 || !abdruck.starts_with("SHA256:") {
        return Ok(None);
    }
    Ok(Some(Fingerabdruck {
        typ: typ.to_owned(),
        abdruck: (*abdruck).to_owned(),
    }))
}

/// Haengt die Zeilen von `gescannt` an seine Datei an; laeuft auf dem Faden
/// `krk-verbinden`, nachdem der Nutzer „Annehmen“ gewaehlt hat.
///
/// **Der eine Schreiber an `known_hosts`** (Modulkopf). Fehlt der Ordner,
/// entsteht er mit `0700`, fehlt die Datei, entsteht sie mit `0600`; eine
/// vorhandene behaelt ihre Rechte und ihren Inhalt, und die Zeilen kommen in
/// einem Schreibaufruf ans Ende. Endet die Datei nicht auf einen
/// Zeilenwechsel, geht einer voraus, damit die erste neue Zeile nicht an die
/// letzte alte geraet. Ohne Datei (`userknownhostsfile none`) ist das ein
/// Fehler der Art `NotFound`.
///
/// **Vor dem ersten Byte wird jede Zeile geprueft**: ihr erstes Feld ist
/// genau der Suchname, und der besteht [`suchname_gueltig`]; sonst ist das ein
/// Fehler der Art `InvalidData`, und geschrieben wird nichts. [`gescannt_aus`]
/// laesst keine andere Zeile entstehen, und die Felder von [`Gescannt`] sind
/// privat; die Pruefung steht trotzdem hier, an dem einen Schreiber, damit sie
/// nicht an einem Bau haengt, der einmal anders entsteht.
pub fn anhaengen(_pfand: &Fernfaden, gescannt: &Gescannt) -> io::Result<()> {
    let erwartet = gescannt.suchname.as_str();
    let fremde_zeile = gescannt
        .zeilen
        .iter()
        .any(|zeile| zeile.contains('\n') || zeile.split(' ').next() != Some(erwartet));
    if !suchname_gueltig(erwartet) || fremde_zeile {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{erwartet:?} is not a single host name for known_hosts"),
        ));
    }
    let Some(datei) = gescannt.datei.as_deref() else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "ssh -G names no userknownhostsfile",
        ));
    };
    if let Some(ordner) = datei.parent()
        && !ordner.as_os_str().is_empty()
    {
        DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(ordner)?;
    }
    let mut offen = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .mode(0o600)
        .open(datei)?;
    let mut block = String::new();
    if offen.metadata()?.len() > 0 {
        let mut letztes = [0_u8; 1];
        offen.seek(SeekFrom::End(-1))?;
        offen.read_exact(&mut letztes)?;
        if letztes[0] != b'\n' {
            block.push('\n');
        }
    }
    for zeile in &gescannt.zeilen {
        block.push_str(zeile);
        block.push('\n');
    }
    offen.write_all(block.as_bytes())?;
    offen.flush()
}
