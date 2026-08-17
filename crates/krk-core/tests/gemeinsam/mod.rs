//! Der Pruefordner der Abnahmeproben des Kerns: **die** eine Fassung.
//!
//! Ein Ordner unter dem Temporaerverzeichnis, der Zweck, Prozesskennung und
//! Laufnummer im Namen traegt und sich in `Drop` wieder abraeumt. Prozesskennung
//! und Laufnummer sind kein Zierrat: ohne sie treffen sich zwei gleichzeitige
//! Testlaeufe in demselben Ordner, und ein Fehlschlag daraus benennt nichts, was
//! am Code falsch waere.
//!
//! Der Ordner liegt unter `std::env::temp_dir()` und ausdruecklich **nicht**
//! unter `~/Library/Caches/krk-messplatz`: der Messplatz gehoert der
//! Messstrecke, nicht den Proben.
//!
//! # Warum dieses Modul unter `tests/gemeinsam/` liegt
//!
//! Jede Datei unmittelbar in `tests/` ist ein eigenes Testziel und damit eine
//! eigene Kiste; keine kann von einer anderen etwas einbinden. Ein
//! Unterverzeichnis ist dagegen kein Testziel, sondern ein Modul, das jedes Ziel
//! per `mod gemeinsam;` einzieht. Bis zum 260810 stand diese halbe Seite deshalb
//! sechsmal im Baum, in `ablage.rs`, `belegung.rs`, `navigation.rs`,
//! `operation.rs`, `text.rs` und `verzeichnis.rs`. Der Defekt dazu ist
//! `issues/260810-1330_*_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`.
//!
//! Der Modulkopf von `tests/verzeichnis.rs` sagte bis dahin, ein
//! Pruefordner-Erzeuger sei "bewusst noch nicht" da und komme mit Schritt 3 der
//! Runde 1. Mit dem Abschluss der Runde 2 war der Satz ueberholt; hier steht
//! jetzt, was daraus geworden ist.
//!
//! Diese Fassung ist nicht mit der in `krk-ui/src/pruefordner.rs` oder der in
//! `krk-bench/src/wegwerfordner.rs` zusammenlegbar: beide Kisten haben nur ein
//! Binaerziel, und ein Testziel erreicht den Code eines Binaerziels nicht.
//!
//! # Warum der Starter der Kindproben ebenfalls hier steht
//!
//! [`kind_mit_deskriptorgrenze`] startet dieselbe Testdatei noch einmal, mit
//! abgesenkter Deskriptorgrenze. Etliche Zusagen dieses Vorhabens sind Aussagen
//! ueber den **Vorrat an Deskriptoren** — der Durchlauf haelt einen, gleich wie
//! tief der Baum ist; ein Mangel von aussen laesst einen Auftrag unentschieden;
//! die gedeckelte Zaehlung des Umfangs haelt einen — und keine davon ist unter
//! `cargo test` messbar, weil der Lauf die angehobene Grenze der
//! Anmeldesitzung erbt. Der Starter stand bis zum 260817 in
//! `tests/verzeichnis.rs`; seit `tests/umfang.rs` daneben denselben Bedarf hat,
//! steht er hier, damit es nicht zwei Fassungen davon gibt.
//!
//! # Jedes Ziel nimmt einen anderen Ausschnitt
//!
//! `#![allow(dead_code)]` steht deshalb hier und nicht als Ausnahme je Funktion:
//! die sechs Ziele uebersetzen dieses Modul je einzeln, und was das eine nicht
//! braucht, ist in dessen Uebersetzung ungenutzt. Ohne die Zeile meldete jedes
//! Ziel den Ausschnitt der anderen fuenf als toten Code.

#![allow(dead_code)]

use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Die Laufnummer, die zwei Ordner desselben Zwecks im selben Prozess trennt.
static ZAEHLER: AtomicU64 = AtomicU64::new(0);

/// Ein Ordner unter dem Temporaerverzeichnis, der sich selbst abraeumt.
pub struct Pruefordner {
    pfad: PathBuf,
}

impl Pruefordner {
    /// Ein frisch angelegter, leerer Ordner.
    pub fn neu(zweck: &str) -> Self {
        let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
        let pfad = std::env::temp_dir().join(format!(
            "krk-kern-probe-{zweck}-{}-{laufnummer}",
            std::process::id()
        ));
        abraeumen(&pfad);
        fs::create_dir_all(&pfad).expect("Pruefordner laesst sich nicht anlegen");
        Self { pfad }
    }

    /// Der Ordner selbst.
    pub fn pfad(&self) -> &Path {
        &self.pfad
    }

    /// Ein Pfad im Ordner, ohne dass dort etwas angelegt wird.
    pub fn unter(&self, name: &str) -> PathBuf {
        self.pfad.join(name)
    }

    /// Legt eine Datei mit dem genannten Inhalt an und liefert ihren Pfad.
    ///
    /// Der Inhalt ist `impl AsRef<[u8]>` und nicht `&str`, weil die Proben der
    /// Textrechnung Bytefolgen schreiben, die in Rust-Quelltext als
    /// Zeichenkette nicht mehr das waeren, was auf der Platte stehen soll.
    pub fn datei(&self, name: &str, inhalt: impl AsRef<[u8]>) -> PathBuf {
        let pfad = self.unter(name);
        fs::write(&pfad, inhalt).expect("Datei laesst sich nicht schreiben");
        pfad
    }

    /// Legt eine Datei mit genau dieser Zahl von Bytes an.
    ///
    /// Welches Byte darin steht, ist gleichgueltig; die Zahl ist es nicht. Die
    /// Groessensumme des Markierungsstandes und die Groessenspalte des
    /// Verzeichnislesers werden gegen sie geprueft.
    pub fn fuelldatei(&self, name: &str, bytes: usize) -> PathBuf {
        self.datei(name, vec![b'x'; bytes])
    }

    /// Legt eine Datei der genannten Groesse an, **ohne ein Byte zu schreiben**.
    ///
    /// `set_len` zieht die Datei auf die Laenge und laesst dahinter ein Loch.
    /// Auf APFS kostet das weder Platz noch Zeit, und genau deshalb kann eine
    /// Probe von zwei Gigabyte reden, ohne zwei Gigabyte anzulegen. Wer das Loch
    /// liest, bekommt Nullbytes; die sind gueltiges UTF-8, was den Grenzfall der
    /// Editorgrenze erst brauchbar macht.
    pub fn luecke(&self, name: &str, groesse: u64) -> PathBuf {
        let pfad = self.unter(name);
        let datei = fs::File::create(&pfad).expect("Luecke laesst sich nicht anlegen");
        datei
            .set_len(groesse)
            .expect("Luecke laesst sich nicht ziehen");
        pfad
    }

    /// Legt einen Unterordner an und liefert seinen Pfad.
    pub fn ordner(&self, name: &str) -> PathBuf {
        let pfad = self.unter(name);
        fs::create_dir_all(&pfad).expect("Ordner laesst sich nicht anlegen");
        pfad
    }

    /// Legt eine weiche Verknuepfung auf das genannte Ziel an.
    ///
    /// Das Ziel darf fehlen: eine Verknuepfung ins Leere ist einer der Faelle,
    /// die der Verzeichnisleser und der Editor auseinanderhalten muessen.
    pub fn verknuepfung(&self, name: &str, ziel: impl AsRef<Path>) -> PathBuf {
        let pfad = self.unter(name);
        std::os::unix::fs::symlink(ziel.as_ref(), &pfad)
            .expect("Verknuepfung laesst sich nicht anlegen");
        pfad
    }

    /// Legt eine benannte Roehre an und liefert ihren Pfad.
    ///
    /// Angelegt wird sie ueber `mkfifo(1)` und nicht ueber einen Fremdaufruf:
    /// `mkfifo(2)` waere eine fuenfte Bindung in `verzeichnis::sys`, und dort
    /// steht, was **KRK** braucht. KRK legt keine Roehren an; das tut nur die
    /// Probe, und ein Werkzeug des Systems zu rufen ist dafuer der kleinere
    /// Eingriff.
    pub fn roehre(&self, name: &str) -> PathBuf {
        let pfad = self.unter(name);
        let stand = std::process::Command::new("/usr/bin/mkfifo")
            .arg(&pfad)
            .status()
            .expect("mkfifo laesst sich nicht starten");
        assert!(stand.success(), "mkfifo ist gescheitert: {stand:?}");
        pfad
    }

    /// Legt einen gebundenen Unix-Socket an und liefert seinen Pfad.
    ///
    /// Anders als bei der Roehre braucht es dafuer kein Werkzeug des Systems und
    /// keine weitere Bindung in `verzeichnis::sys`: `UnixListener::bind` steht
    /// in der Standardbibliothek, und der gebundene Eintrag bleibt im
    /// Dateisystem stehen, auch nachdem der Horcher gefallen ist. `abraeumen`
    /// nimmt ihn ohne Zutun mit; `remove_dir_all` kommt an einem Socket vorbei.
    ///
    /// **Der Name bleibt kurz, und das ist keine Geschmacksfrage.** `AF_UNIX`
    /// fasst auf macOS 104 Bytes Pfad. Der Pruefordner liegt unter
    /// `/var/folders/…/T`, also schon 48 Zeichen tief, und traegt Zweck,
    /// Prozesskennung und Laufnummer im Namen; der einzige Rufer kommt damit auf
    /// 92 Bytes und hat ein Dutzend uebrig. Wer den Zwecknamen laenger waehlt
    /// oder den Socket tiefer legt, bekommt keinen Befund ueber das
    /// Verweisziel, sondern „AF_UNIX path too long".
    pub fn socket(&self, name: &str) -> PathBuf {
        let pfad = self.unter(name);
        let horcher = std::os::unix::net::UnixListener::bind(&pfad).unwrap_or_else(|fehler| {
            panic!(
                "Socket {} laesst sich nicht binden: {fehler}",
                pfad.display()
            )
        });
        drop(horcher);
        pfad
    }

    /// Setzt `UF_HIDDEN` auf einen Eintrag im Ordner.
    ///
    /// Das Kennzeichen des Dateisystems ist der zweite Weg, auf dem ein Eintrag
    /// versteckt sein kann; der erste ist der fuehrende Punkt im Namen.
    pub fn verstecken(&self, name: &str) {
        let ergebnis = std::process::Command::new("/usr/bin/chflags")
            .arg("hidden")
            .arg(self.unter(name))
            .status()
            .expect("chflags laesst sich nicht aufrufen");
        assert!(ergebnis.success(), "chflags hidden ist gescheitert");
    }
}

impl Drop for Pruefordner {
    fn drop(&mut self) {
        abraeumen(&self.pfad);
    }
}

/// Raeumt einen Baum ab, notfalls gegen entzogene Rechte.
///
/// Zwei Stufen, und die Reihenfolge ist der Grund fuer beide: `remove_dir_all`
/// raeumt einen Ordner mit 5.000 Eintraegen in einem Zug ab, kommt aber an einem
/// Eintrag mit `0o000` nicht vorbei. Genau so einen legt die Rechtepruefung der
/// Operationsmaschine an. [`entsperren_und_loeschen`] steigt dafuer Eintrag fuer
/// Eintrag hinab und dreht jedem die Rechte zurueck; das kostet einen Aufruf je
/// Eintrag und laeuft deshalb nur, wenn der schnelle Weg gescheitert ist.
fn abraeumen(pfad: &Path) {
    if fs::remove_dir_all(pfad).is_ok() {
        return;
    }
    let _ = entsperren_und_loeschen(pfad);
}

/// Raeumt einen Baum ab und gibt vorher jedem Eintrag wieder Rechte.
fn entsperren_und_loeschen(pfad: &Path) -> io::Result<()> {
    if let Ok(angaben) = fs::symlink_metadata(pfad) {
        if !angaben.is_symlink() {
            let _ = fs::set_permissions(pfad, fs::Permissions::from_mode(0o755));
        }
        if angaben.is_dir()
            && let Ok(eintraege) = fs::read_dir(pfad)
        {
            for eintrag in eintraege.flatten() {
                let _ = entsperren_und_loeschen(&eintrag.path());
            }
        }
    }
    match fs::symlink_metadata(pfad) {
        Ok(angaben) if angaben.is_dir() => fs::remove_dir(pfad),
        Ok(_) => fs::remove_file(pfad),
        Err(fehler) => Err(fehler),
    }
}

// ---------------------------------------------------------------------------
// Der Quellbaum des Vorhabens als Lesestoff fuer die Zaehlproben
// ---------------------------------------------------------------------------

/// Jede `.rs`-Datei unter `crates/`, mit ihrem Pfad unterhalb von `crates/` und
/// ihrem Inhalt, in fester Reihenfolge.
///
/// **Etliche Zusagen dieses Vorhabens sind Aussagen ueber den Baum** und nicht
/// ueber ein Ergebnis: „es gibt genau zwei Dateien mit `#![allow(unsafe_code)]`",
/// „es gibt genau drei Pruefordner-Fassungen". An keinem Rueckgabewert ist
/// abzulesen, dass es keine dritte gibt.
///
/// **Das ist nicht dieselbe Funktion wie `krk_ui::quellbaum::quelldateien`**,
/// und sie ist auch nicht mit ihr zusammenzulegen. Jene ist `pub(crate)` in
/// einer Kiste mit nur einem Binaerziel; ein Testziel erreicht sie nicht, aus
/// demselben Grund, aus dem es drei Pruefordner-Fassungen gibt.
///
/// **Die beiden lesen seit der Runde 7 denselben Umfang**, naemlich alle Kisten
/// unter `crates/`. Bis dahin las jene nur `krk-ui/src`, und der Unterschied
/// stand nirgends nebeneinander: wer eine Zaehlprobe schrieb, waehlte damit
/// unbemerkt ihre Reichweite mit
/// (`issues/260813-0540_*_die-zaehlproben-in-krk-ui-sagen-im-baum-und-lesen-nur-eine-kiste.md`).
/// Wer eine der beiden aendert, aendert die andere mit.
///
/// `CARGO_MANIFEST_DIR` steht beim Uebersetzen fest und zeigt auf
/// `crates/krk-core`; zwei Schritte darueber liegt die Wurzel. Fehlt der Baum,
/// schlaegt die Funktion fehl statt still nichts zu zaehlen — eine leere Liste
/// waere eine Probe, die alles bestaetigt.
pub fn quelldateien() -> Vec<(String, String)> {
    let wurzel = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/krk-core liegt zwei Ebenen unter der Wurzel")
        .join("crates");
    let mut gefunden = Vec::new();
    quellen_einsammeln(&wurzel, &wurzel, &mut gefunden);
    assert!(
        gefunden.len() > 1,
        "unter {} steht kein Quellbaum; die Zaehlproben haetten nichts zu zaehlen",
        wurzel.display()
    );
    gefunden.sort();
    gefunden
}

/// Haengt alle `.rs`-Dateien unter `ordner` an `gefunden`, in die Tiefe.
fn quellen_einsammeln(wurzel: &Path, ordner: &Path, gefunden: &mut Vec<(String, String)>) {
    let eintraege = fs::read_dir(ordner)
        .unwrap_or_else(|fehler| panic!("{} nicht lesbar: {fehler}", ordner.display()));
    for eintrag in eintraege {
        let pfad = eintrag
            .expect("Eintrag des Quellordners nicht lesbar")
            .path();
        if pfad.is_dir() {
            quellen_einsammeln(wurzel, &pfad, gefunden);
        } else if pfad.extension().is_some_and(|endung| endung == "rs") {
            let name = pfad
                .strip_prefix(wurzel)
                .expect("der Pfad kommt aus der Wurzel")
                .to_string_lossy()
                .into_owned();
            let inhalt = fs::read_to_string(&pfad)
                .unwrap_or_else(|fehler| panic!("{} nicht lesbar: {fehler}", pfad.display()));
            gefunden.push((name, inhalt));
        }
    }
}

// ---------------------------------------------------------------------------
// Der Starter der Kindproben unter abgesenkter Deskriptorgrenze
// ---------------------------------------------------------------------------

/// Startet dieselbe Testdatei noch einmal, mit abgesenkter Deskriptorgrenze.
///
/// Der Umweg ueber `/bin/sh` ist der einzige ohne `setrlimit(2)`, und
/// `setrlimit(2)` waere eine sechste Bindung in [`krk_core::verzeichnis::sys`]
/// fuer etwas, das KRK selbst nicht braucht. `$0` ist die Testdatei, `$1` der
/// Name der Kindprobe.
///
/// **`grenze` reist als Argument, weil die beiden Rufer verschiedene Zahlen
/// brauchen.** Die Proben des Durchlaufs messen unter 64, der Zahl, unter der
/// ein aus dem Finder gestartetes Buendel ungefaehr laeuft. Die Zaehlung des
/// Umfangs braucht eine tiefere: ihr Deckel begrenzt die Zahl der geoeffneten
/// Verzeichnisse ohnehin auf 26, und unter 64 liefe deshalb auch ein Abstieg
/// durch, der einen Deskriptor je Ebene haelt. Die Probe wuerde dann nichts
/// messen, und genau davor steht dieser Starter.
///
/// Der Aufrufer prueft im Kind zuerst, wie viele Deskriptoren es ueberhaupt
/// bekommt, und behauptet die abgesenkte Grenze nicht: ohne diese Zusicherung
/// bestuende jede Probe hier auch dann, wenn `ulimit` nicht gegriffen haette.
pub fn kind_mit_deskriptorgrenze(
    grenze: usize,
    name: &str,
    auftrag: &str,
    wert: &Path,
) -> std::process::Output {
    let selbst = std::env::current_exe().expect("die Testdatei kennt ihren Pfad nicht");
    std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg(format!(
            "ulimit -n {grenze} && exec \"$0\" --exact --ignored --nocapture --test-threads 1 \"$1\""
        ))
        .arg(&selbst)
        .arg(name)
        .env(auftrag, wert)
        .output()
        .expect("die Kindprobe laesst sich nicht starten")
}
