//! Abnahmeproben fuer `krk_core::terminalskript`: die Wahl zwischen `vim` und
//! `sudo -e`, die Pruefung der Datei und das Skript, das die
//! Terminal-Anwendung ausfuehrt.
//!
//! Das Skript laeuft hier unter `/bin/sh` mit einem Ersatz-`vim` und einem
//! Ersatz-`sudo`, die ihr Arbeitsverzeichnis und ihre Argumente durch NUL
//! getrennt in eine Datei schreiben. Damit ist ohne AppKit und ohne Terminal
//! entschieden, ob der Pfad unverfaelscht bei vim und sudo ankommt. Ein
//! echtes `sudo` startet keine Probe.
//!
//! **Was diese Proben nicht sehen:** eine Datei mit fremdem Eigentuemer und
//! eine Zugriffsliste, denn `cargo test` kann keine root-Datei anlegen. Das
//! deckt die Nutzerabnahme des Arbeitspakets
//! `261029-0915-datei-im-terminal-mit-vim-und-sudo-e-oeffnen`.

mod gemeinsam;

use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use gemeinsam::{Pruefordner, aufrufstellen, betriebscode, quelldateien};
use krk_core::terminalskript::{
    Skriptfehler, Terminalbefehl, befehl_fuer, skripttext, vorbereiten,
};

/// Setzt die Rechte eines Ordners in `Drop` auf `0o755` zurueck, damit der
/// Pruefordner ihn auch nach einem Fehlschlag abraeumen kann.
struct Ordnersperre(PathBuf);

impl Ordnersperre {
    fn sperren(ordner: &Path) -> Self {
        fs::set_permissions(ordner, fs::Permissions::from_mode(0o555))
            .expect("Ordner laesst sich nicht sperren");
        Self(ordner.to_path_buf())
    }
}

impl Drop for Ordnersperre {
    fn drop(&mut self) {
        let _ = fs::set_permissions(&self.0, fs::Permissions::from_mode(0o755));
    }
}

fn rechte_setzen(pfad: &Path, modus: u32) {
    fs::set_permissions(pfad, fs::Permissions::from_mode(modus))
        .expect("Rechte lassen sich nicht setzen");
}

fn skripte_in(verzeichnis: &Path) -> Vec<PathBuf> {
    fs::read_dir(verzeichnis)
        .expect("Skriptverzeichnis nicht lesbar")
        .map(|eintrag| eintrag.expect("Eintrag nicht lesbar").path())
        .collect()
}

#[test]
fn die_wahl_nimmt_vim_wenn_datei_oder_ordner_schreibbar_sind() {
    assert_eq!(befehl_fuer(true, true), Terminalbefehl::Vim);
    assert_eq!(befehl_fuer(true, false), Terminalbefehl::Vim);
    assert_eq!(befehl_fuer(false, true), Terminalbefehl::Vim);
    assert_eq!(befehl_fuer(false, false), Terminalbefehl::SudoBearbeiten);
}

/// Die Wahl an echten Rechten: `access(2)` sieht das fehlende Modusbit an
/// einer eigenen Datei, und der Ordner entscheidet, ob sie trotzdem an vim
/// geht.
#[test]
fn die_wahl_folgt_den_rechten_von_datei_und_ordner() {
    let ordner = Pruefordner::neu("terminal-wahl");
    let skripte = ordner.ordner("skripte");
    let offen = ordner.ordner("offen");
    let gesperrt = ordner.ordner("gesperrt");

    let schreibbar = offen.join("schreibbar.txt");
    fs::write(&schreibbar, "x").unwrap();
    rechte_setzen(&schreibbar, 0o644);
    let schreibgeschuetzt = offen.join("schreibgeschuetzt.txt");
    fs::write(&schreibgeschuetzt, "x").unwrap();
    rechte_setzen(&schreibgeschuetzt, 0o444);
    let im_gesperrten_schreibbar = gesperrt.join("schreibbar.txt");
    fs::write(&im_gesperrten_schreibbar, "x").unwrap();
    rechte_setzen(&im_gesperrten_schreibbar, 0o644);
    let im_gesperrten_geschuetzt = gesperrt.join("schreibgeschuetzt.txt");
    fs::write(&im_gesperrten_geschuetzt, "x").unwrap();
    rechte_setzen(&im_gesperrten_geschuetzt, 0o444);
    let _sperre = Ordnersperre::sperren(&gesperrt);

    let faelle = [
        (&schreibbar, Terminalbefehl::Vim),
        (&schreibgeschuetzt, Terminalbefehl::Vim),
        (&im_gesperrten_schreibbar, Terminalbefehl::Vim),
        (&im_gesperrten_geschuetzt, Terminalbefehl::SudoBearbeiten),
    ];
    for (datei, erwartet) in faelle {
        let vorbereitet = vorbereiten(&skripte, datei)
            .unwrap_or_else(|fehler| panic!("{}: {fehler:?}", datei.display()));
        assert_eq!(vorbereitet.befehl, erwartet, "{}", datei.display());
        fs::remove_file(&vorbereitet.skript).unwrap();
    }
}

/// Legt unter `bin` ein Ersatzprogramm an, das Arbeitsverzeichnis und
/// Argumente durch NUL getrennt in `$KRK_PROBE_AUSGABE` schreibt.
fn ersatzprogramm(bin: &Path, name: &str) {
    let pfad = bin.join(name);
    fs::write(
        &pfad,
        "#!/bin/sh\nprintf '%s\\0' \"$(pwd -P)\" \"$@\" > \"$KRK_PROBE_AUSGABE\"\n",
    )
    .unwrap();
    rechte_setzen(&pfad, 0o755);
}

/// Faehrt das Skript unter `/bin/sh` und liefert Arbeitsverzeichnis und
/// Argumente, die das Ersatzprogramm gesehen hat.
fn skript_fahren(ordner: &Pruefordner, bin: &Path, skript: &Path) -> (PathBuf, Vec<Vec<u8>>) {
    let ausgabe = ordner.unter("ausgabe");
    let _ = fs::remove_file(&ausgabe);
    let lauf = Command::new("/bin/sh")
        .arg(skript)
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
        .env("KRK_PROBE_AUSGABE", &ausgabe)
        .output()
        .expect("/bin/sh laesst sich nicht starten");
    assert!(
        lauf.status.success(),
        "Skript scheitert: {}\n{}",
        String::from_utf8_lossy(&lauf.stdout),
        String::from_utf8_lossy(&lauf.stderr)
    );
    let roh = fs::read(&ausgabe).expect("das Ersatzprogramm hat nichts geschrieben");
    let mut teile: Vec<Vec<u8>> = roh.split(|byte| *byte == 0).map(<[u8]>::to_vec).collect();
    assert_eq!(
        teile.pop().as_deref(),
        Some(&[][..]),
        "Ausgabe endet nicht auf NUL"
    );
    let arbeitsverzeichnis = PathBuf::from(OsStr::from_bytes(&teile.remove(0)));
    (arbeitsverzeichnis, teile)
}

/// Jeder Name kommt unverfaelscht bei vim und bei sudo an, das
/// Arbeitsverzeichnis ist der Ordner der Datei, und das Skript ist nach dem
/// Lauf verschwunden.
#[test]
fn das_skript_reicht_jeden_namen_unverfaelscht_weiter() {
    let ordner = Pruefordner::neu("terminal-lauf");
    let skripte = ordner.ordner("skripte");
    let bin = ordner.ordner("bin");
    ersatzprogramm(&bin, "vim");
    ersatzprogramm(&bin, "sudo");
    let offen = ordner.ordner("offen ordner");
    let gesperrt = ordner.ordner("gesperrt'ordner");

    let namen = [
        "mit leerzeichen.txt",
        "it's.txt",
        "a\"b.txt",
        "$HOME.txt",
        "back`tick.txt",
        "semi;colon.txt",
        "stern*.txt",
        "-fuehrend.txt",
        "zeilen\numbruch.txt",
        "Größe.txt",
    ];
    for name in namen {
        let datei = offen.join(name);
        fs::write(&datei, "x").unwrap();
        rechte_setzen(&datei, 0o644);
        let datei = gesperrt.join(name);
        fs::write(&datei, "x").unwrap();
        rechte_setzen(&datei, 0o444);
    }
    let _sperre = Ordnersperre::sperren(&gesperrt);

    for name in namen {
        for (ordner_der_datei, befehl, erwartet_vorn) in [
            (&offen, Terminalbefehl::Vim, vec![b"--".to_vec()]),
            (
                &gesperrt,
                Terminalbefehl::SudoBearbeiten,
                vec![b"-e".to_vec(), b"--".to_vec()],
            ),
        ] {
            let datei = ordner_der_datei.join(name);
            let vorbereitet = vorbereiten(&skripte, &datei)
                .unwrap_or_else(|fehler| panic!("{name:?}: {fehler:?}"));
            assert_eq!(vorbereitet.befehl, befehl, "{name:?}");

            let (arbeitsverzeichnis, argumente) = skript_fahren(&ordner, &bin, &vorbereitet.skript);
            let mut erwartet = erwartet_vorn.clone();
            erwartet.push(datei.as_os_str().as_bytes().to_vec());
            assert_eq!(argumente, erwartet, "{name:?}");
            assert_eq!(
                arbeitsverzeichnis,
                fs::canonicalize(ordner_der_datei).unwrap(),
                "{name:?}"
            );
            assert!(
                !vorbereitet.skript.exists(),
                "{name:?}: das Skript liegt nach dem Lauf noch da"
            );
        }
    }
}

#[test]
fn das_skript_hat_die_rechte_0700() {
    let ordner = Pruefordner::neu("terminal-rechte");
    let skripte = ordner.ordner("skripte");
    let datei = ordner.datei("datei.txt", "x");
    let vorbereitet = vorbereiten(&skripte, &datei).unwrap();
    let modus = fs::metadata(&vorbereitet.skript)
        .unwrap()
        .permissions()
        .mode();
    assert_eq!(modus & 0o7777, 0o700);
}

#[test]
fn ordner_und_fehlende_datei_legen_kein_skript_an() {
    let ordner = Pruefordner::neu("terminal-fehler");
    let skripte = ordner.ordner("skripte");
    let unterordner = ordner.ordner("unterordner");
    let fehlt = ordner.unter("fehlt.txt");

    assert!(matches!(
        vorbereiten(&skripte, &unterordner),
        Err(Skriptfehler::IstOrdner)
    ));
    assert!(matches!(
        vorbereiten(&skripte, &fehlt),
        Err(Skriptfehler::Fehlt)
    ));
    assert!(skripte_in(&skripte).is_empty());
}

#[test]
fn ein_pfad_ohne_utf8_oder_relativ_wird_ohne_dateisystem_abgewiesen() {
    let krumm = Path::new(OsStr::from_bytes(b"/tmp/krk-\xffkrumm"));
    assert!(matches!(
        skripttext(Terminalbefehl::Vim, krumm),
        Err(Skriptfehler::KeinUtf8)
    ));
    assert!(matches!(
        skripttext(Terminalbefehl::Vim, Path::new("relativ.txt")),
        Err(Skriptfehler::NichtAbsolut)
    ));
    assert!(matches!(
        skripttext(Terminalbefehl::Vim, Path::new("/")),
        Err(Skriptfehler::NichtAbsolut)
    ));

    let ordner = Pruefordner::neu("terminal-pfad");
    let skripte = ordner.ordner("skripte");
    assert!(matches!(
        vorbereiten(&skripte, Path::new("relativ.txt")),
        Err(Skriptfehler::NichtAbsolut)
    ));
    assert!(matches!(
        vorbereiten(&skripte, krumm),
        Err(Skriptfehler::KeinUtf8)
    ));
    assert!(skripte_in(&skripte).is_empty());
}

#[test]
fn der_skripttext_loescht_sich_vor_dem_start() {
    for befehl in [Terminalbefehl::Vim, Terminalbefehl::SudoBearbeiten] {
        let text = skripttext(befehl, Path::new("/a b/it's")).unwrap();
        assert!(text.starts_with("#!/bin/sh\n"), "{text}");
        let loeschen = text.find("rm -f -- \"$0\"").expect("keine Selbstloeschung");
        let start = text.find("exec ").expect("kein exec");
        assert!(loeschen < start, "{text}");
        assert!(text.contains("cd -- '/a b' || exit 1\n"), "{text}");
    }
    assert!(
        skripttext(Terminalbefehl::Vim, Path::new("/a b/it's"))
            .unwrap()
            .ends_with("exec vim -- '/a b/it'\\''s'\n")
    );
    assert!(
        skripttext(Terminalbefehl::SudoBearbeiten, Path::new("/x"))
            .unwrap()
            .ends_with("exec sudo -e -- '/x'\n")
    );
}

/// Jeder Fehler hat einen Satz aus der Sprachtabelle, und die zwei mit Grund
/// tragen ihn.
#[test]
fn jeder_fehler_hat_eine_meldung() {
    let grund = || std::io::Error::other("der Grund");
    for fehler in [
        Skriptfehler::KeinUtf8,
        Skriptfehler::NichtAbsolut,
        Skriptfehler::Fehlt,
        Skriptfehler::IstOrdner,
    ] {
        assert!(!fehler.meldung().is_empty(), "{fehler:?}");
    }
    assert!(
        Skriptfehler::Pruefen(grund())
            .meldung()
            .contains("der Grund")
    );
    assert!(
        Skriptfehler::Schreiben(grund())
            .meldung()
            .contains("der Grund")
    );
}

/// Der Rumpf einer Funktion `pub fn <name>(` als Text, bis zur ersten Zeile,
/// die genau `}` ist.
fn rumpf<'a>(zeilen: &[&'a str], name: &str) -> Vec<&'a str> {
    let kopf = format!("pub fn {name}(");
    let anfang = zeilen
        .iter()
        .position(|zeile| zeile.starts_with(&kopf))
        .unwrap_or_else(|| panic!("{name} steht nicht im Betriebscode"));
    let ende = zeilen[anfang..]
        .iter()
        .position(|zeile| *zeile == "}")
        .expect("der Rumpf endet nicht");
    zeilen[anfang..=anfang + ende].to_vec()
}

/// Die reine Wahl hat im Betriebscode einen Rufer, `vorbereiten`, und die
/// Huelle um `access(2)` genau zwei Rufer, den Umbau eines Archivs und dieses
/// Modul.
///
/// **Was diese Nadel nicht sieht:** einen Aufruf unter einem anderen Namen
/// (`use … as anders;`) und einen Rufer in einem Pruefmodul; der Kopf von
/// `tests/baum.rs` sagt, warum keine Suche im Quelltext restlos dicht ist.
#[test]
fn die_wahl_und_die_rechtefrage_haben_genau_ihre_rufer() {
    let baum = quelldateien();
    let betrieb: Vec<(String, String)> = baum
        .iter()
        .filter(|(name, _)| name.starts_with("krk-core/src/") || name.starts_with("krk-ui/src/"))
        .map(|(name, inhalt)| (name.clone(), betriebscode(inhalt).join("\n")))
        .collect();

    let wahl: Vec<(&str, usize)> = betrieb
        .iter()
        .map(|(name, code)| (name.as_str(), aufrufstellen(code, "befehl_fuer")))
        .filter(|(_, zahl)| *zahl > 0)
        .collect();
    assert_eq!(wahl, [("krk-core/src/terminalskript.rs", 1)]);

    let (_, modul) = baum
        .iter()
        .find(|(name, _)| name == "krk-core/src/terminalskript.rs")
        .expect("terminalskript.rs steht nicht im Baum");
    let zeilen = betriebscode(modul);
    let im_vorbereiten = rumpf(&zeilen, "vorbereiten").join("\n");
    assert_eq!(aufrufstellen(&im_vorbereiten, "befehl_fuer"), 1);

    let rechtefrage: Vec<&str> = betrieb
        .iter()
        .filter(|(_, code)| aufrufstellen(code, "schreiben_erlaubt") > 0)
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(
        rechtefrage,
        [
            "krk-core/src/archiv/umbau.rs",
            "krk-core/src/terminalskript.rs"
        ]
    );
}
