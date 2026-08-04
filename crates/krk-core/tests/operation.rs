//! Abnahme der Operationsmaschine (Schritt 15 des Plans, C4).
//!
//! Alle Pruefungen laufen ohne Fenster und ohne AppKit. Der Papierkorb ist eine
//! Attrappe: der echte liegt in `krk-ui/src/appkit/papierkorb.rs`, und dass er
//! dort liegt und hier nicht gebraucht wird, ist die Zusage "der Kern ist ohne
//! Fenster testbar".
//!
//! # Die grossen Pruefdateien
//!
//! Zwei der vier Abnahmepunkte brauchen eine 200-MB- und eine 500-MB-Datei.
//! Beide entstehen unter `/tmp`, auf demselben APFS-Datentraeger wie ihr Ziel,
//! und werden am Ende des jeweiligen Laufs wieder abgeraeumt. In den Quellbaum
//! kommt keine von beiden.
//!
//! Die 200-MB-Datei ist duennbesetzt: das Verschieben haengt einen
//! Verzeichniseintrag um und liest kein einziges Byte, ein Loch ist dafuer so
//! gut wie Daten. Die 500-MB-Datei traegt echte Bytes, denn genau ihr Inhalt
//! ist es, den der Abbruch mitten in der Uebertragung treffen soll.
//!
//! # Warum die beiden Zeitmessungen sich gegenseitig ausschliessen
//!
//! `cargo test` laeuft nebenlaeufig. Zwei Pruefungen, die zugleich hunderte
//! Megabyte durch dasselbe Dateisystem schieben, messen einander mit; die
//! Zahlen waeren dann nicht die des Kerns, sondern die der Testlaufordnung.
//! [`ZEITMESSUNG`] laesst deshalb immer nur eine von beiden laufen.

use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use krk_core::operation::{
    Abschluss, Auftrag, Bericht, Konfliktantwort, Konfliktentscheid, Konfliktregel, Lauf, Meldung,
    OhnePapierkorb, Papierkorb, datei_anlegen, freier_name, ordner_anlegen, starten, umbenennen,
};
use krk_core::verzeichnis::sys::Uebertragungsart;

// ---------------------------------------------------------------------------
// Pruefordner und Hilfsmittel
// ---------------------------------------------------------------------------

static ZAEHLER: AtomicU64 = AtomicU64::new(0);

/// Die beiden Zeitmessungen laufen nacheinander, nicht nebeneinander.
static ZEITMESSUNG: Mutex<()> = Mutex::new(());

/// Ein Ordner unter `/tmp`, der sich selbst wieder abraeumt.
struct Pruefordner {
    pfad: PathBuf,
}

impl Pruefordner {
    fn neu(zweck: &str) -> Self {
        let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
        let mut pfad = std::env::temp_dir();
        pfad.push(format!(
            "krk-operation-{zweck}-{}-{laufnummer}",
            std::process::id()
        ));
        let _ = entsperren_und_loeschen(&pfad);
        fs::create_dir_all(&pfad).expect("Pruefordner laesst sich nicht anlegen");
        Self { pfad }
    }

    fn pfad(&self) -> &Path {
        &self.pfad
    }

    fn unter(&self, name: &str) -> PathBuf {
        self.pfad.join(name)
    }

    fn ordner(&self, name: &str) -> PathBuf {
        let pfad = self.unter(name);
        fs::create_dir_all(&pfad).expect("Ordner laesst sich nicht anlegen");
        pfad
    }

    fn datei(&self, name: &str, inhalt: &str) -> PathBuf {
        let pfad = self.unter(name);
        fs::write(&pfad, inhalt).expect("Datei laesst sich nicht schreiben");
        pfad
    }
}

impl Drop for Pruefordner {
    fn drop(&mut self) {
        let _ = entsperren_und_loeschen(&self.pfad);
    }
}

/// Raeumt einen Baum ab und gibt vorher jedem Eintrag wieder Rechte.
///
/// Die Rechtepruefung legt einen Eintrag mit `0o000` an. Ohne dieses
/// Zurueckdrehen bliebe er liegen, und `/tmp` fuellte sich mit Resten.
fn entsperren_und_loeschen(pfad: &Path) -> std::io::Result<()> {
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

/// Ein Papierkorb, der nichts loescht, sondern nur mitschreibt.
#[derive(Debug, Default)]
struct Papierkorbattrappe {
    geraeumt: Mutex<Vec<PathBuf>>,
}

impl Papierkorb for Papierkorbattrappe {
    fn in_den_papierkorb(&self, pfad: &Path) -> std::io::Result<PathBuf> {
        self.geraeumt
            .lock()
            .expect("Attrappe vergiftet")
            .push(pfad.to_path_buf());
        Ok(PathBuf::from("/Users/pruefer/.Trash").join(pfad.file_name().unwrap_or_default()))
    }
}

/// Faehrt einen Auftrag zu Ende und liefert den Abschlussbericht.
fn durchlaufen(auftrag: Auftrag, papierkorb: Arc<dyn Papierkorb>) -> Bericht {
    let lauf = starten(auftrag, papierkorb);
    let bericht = bericht_abholen(lauf.meldungen());
    lauf.warten();
    bericht
}

/// Faehrt einen Auftrag ohne Papierkorb zu Ende.
fn durchlaufen_ohne_papierkorb(auftrag: Auftrag) -> Bericht {
    durchlaufen(auftrag, Arc::new(OhnePapierkorb))
}

/// Leert den Kanal bis zur Abschlussmeldung.
fn bericht_abholen(meldungen: &Receiver<Meldung>) -> Bericht {
    while let Ok(meldung) = meldungen.recv() {
        if let Meldung::Fertig(bericht) = meldung {
            return bericht;
        }
    }
    panic!("der Lauf hat keine Abschlussmeldung geschickt");
}

/// Zaehlt alle Eintraege unterhalb eines Ordners, den Ordner selbst nicht mit.
fn eintraege_zaehlen(ordner: &Path) -> usize {
    let mut summe = 0;
    for eintrag in fs::read_dir(ordner).expect("Ordner nicht lesbar").flatten() {
        summe += 1;
        let angaben = eintrag.metadata().expect("keine Angaben");
        if angaben.is_dir() {
            summe += eintraege_zaehlen(&eintrag.path());
        }
    }
    summe
}

/// Legt einen Baum mit genau `anzahl` Eintraegen an, darunter verschachtelte
/// Ordner.
///
/// Der Bauplan ist absichtlich einfach und nicht der Pruefordner-Erzeuger aus
/// `krk-bench`: der legt einen **flachen** Ordner an, und geprueft werden soll
/// hier gerade die Verschachtelung.
fn baum_anlegen(wurzel: &Path, anzahl: usize) -> usize {
    fs::create_dir_all(wurzel).expect("Wurzel laesst sich nicht anlegen");
    let mut gelegt = 0;
    let mut ordnernummer = 0;
    let mut aktueller = wurzel.to_path_buf();

    while gelegt < anzahl {
        // Alle neun Dateien ein neuer Unterordner, abwechselnd eine Ebene
        // tiefer und wieder eine Ebene hoeher. Das ergibt einen Baum, der
        // sowohl in die Breite als auch in die Tiefe geht.
        if gelegt % 9 == 8 {
            ordnernummer += 1;
            let unten = ordnernummer % 3 != 0;
            aktueller = if unten {
                aktueller.join(format!("ordner-{ordnernummer:03}"))
            } else {
                wurzel.join(format!("ordner-{ordnernummer:03}"))
            };
            fs::create_dir_all(&aktueller).expect("Unterordner laesst sich nicht anlegen");
        } else {
            let name = format!("datei-{gelegt:04}.txt");
            fs::write(aktueller.join(&name), format!("Inhalt von {name}\n"))
                .expect("Datei laesst sich nicht schreiben");
        }
        gelegt += 1;
    }
    gelegt
}

/// Legt eine duennbesetzte Datei der genannten Groesse an.
fn loch_datei(pfad: &Path, bytes: u64) {
    let datei = File::create(pfad).expect("Pruefdatei laesst sich nicht anlegen");
    datei.set_len(bytes).expect("Groesse nicht setzbar");
}

/// Legt eine Datei mit echten Bytes an.
///
/// Fuer die Abbruchpruefung: eine duennbesetzte Datei koennte das Dateisystem
/// als Loecher weiterreichen, und dann waere die Uebertragung fertig, bevor der
/// Abbruch sie erreicht. Geprueft werden soll aber der Abbruch, nicht die
/// Geschwindigkeit des Dateisystems.
fn volle_datei(pfad: &Path, bytes: u64) {
    let mut datei = File::create(pfad).expect("Pruefdatei laesst sich nicht anlegen");
    let block = vec![b'k'; 4 * 1024 * 1024];
    let mut geschrieben = 0u64;
    while geschrieben < bytes {
        let rest = (bytes - geschrieben).min(block.len() as u64) as usize;
        datei
            .write_all(&block[..rest])
            .expect("Pruefdatei laesst sich nicht fuellen");
        geschrieben += rest as u64;
    }
    datei.sync_all().expect("Pruefdatei nicht auf die Platte");
}

// ---------------------------------------------------------------------------
// Abnahmepunkt 1: ein Baum mit 500 Eintraegen, verschachtelte Ordner
// eingeschlossen
// ---------------------------------------------------------------------------

#[test]
fn ein_baum_mit_500_eintraegen_kommt_vollstaendig_an() {
    let ordner = Pruefordner::neu("baum-500");
    let quelle = ordner.unter("quelle");
    let angelegt = baum_anlegen(&quelle, 500);
    assert_eq!(angelegt, 500);
    let ziel = ordner.ordner("ziel");

    let bericht = durchlaufen_ohne_papierkorb(Auftrag::kopieren(vec![quelle.clone()], &ziel));

    assert_eq!(bericht.abschluss, Abschluss::Fertig);
    assert!(
        bericht.uebersprungen.is_empty(),
        "uebersprungen: {:?}",
        bericht.uebersprungen
    );
    // 500 Eintraege im Baum und der kopierte Wurzelordner selbst.
    assert_eq!(bericht.eintraege, 501);

    let angekommen = ziel.join("quelle");
    assert_eq!(
        eintraege_zaehlen(&angekommen),
        500,
        "am Ziel liegen nicht 500 Eintraege"
    );
    assert!(
        eintraege_zaehlen(&angekommen) == eintraege_zaehlen(&quelle),
        "Quelle und Ziel tragen verschieden viele Eintraege"
    );

    // Und stichprobenartig: der Inhalt ist der Inhalt.
    let erste = angekommen.join("datei-0000.txt");
    assert_eq!(
        fs::read_to_string(&erste).expect("kopierte Datei nicht lesbar"),
        "Inhalt von datei-0000.txt\n"
    );
}

// ---------------------------------------------------------------------------
// Abnahmepunkt 2: Verschieben innerhalb eines Datentraegers haengt nicht an der
// Dateigroesse
// ---------------------------------------------------------------------------

#[test]
fn eine_200_mb_datei_ist_in_unter_50_ms_verschoben() {
    let _reihum = ZEITMESSUNG
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner());
    let ordner = Pruefordner::neu("verschieben-200mb");
    let quelle = ordner.unter("gross.bin");
    loch_datei(&quelle, 200 * 1024 * 1024);
    let ziel = ordner.ordner("ziel");

    let klein = ordner.datei("klein.bin", "ein paar Bytes");
    let ziel_klein = ordner.ordner("ziel-klein");

    let kleine_spanne =
        gemessen(|| durchlaufen_ohne_papierkorb(Auftrag::verschieben(vec![klein], &ziel_klein)));
    let grosse_spanne =
        gemessen(|| durchlaufen_ohne_papierkorb(Auftrag::verschieben(vec![quelle], &ziel)));

    let (kleine_spanne, kleiner_bericht) = kleine_spanne;
    let (grosse_spanne, grosser_bericht) = grosse_spanne;

    assert_eq!(grosser_bericht.abschluss, Abschluss::Fertig);
    assert_eq!(grosser_bericht.eintraege, 1);
    assert_eq!(kleiner_bericht.eintraege, 1);
    assert!(
        ziel.join("gross.bin").exists(),
        "die grosse Datei ist nicht am Ziel angekommen"
    );

    assert!(
        grosse_spanne < Duration::from_millis(50),
        "das Verschieben der 200-MB-Datei hat {grosse_spanne:?} gedauert, \
         erlaubt sind 50 ms (eine kleine Datei brauchte {kleine_spanne:?})"
    );
}

/// Misst, wie lange ein Aufruf gedauert hat.
fn gemessen<T>(arbeit: impl FnOnce() -> T) -> (Duration, T) {
    let beginn = Instant::now();
    let ergebnis = arbeit();
    (beginn.elapsed(), ergebnis)
}

// ---------------------------------------------------------------------------
// Abnahmepunkt 3: Abbruch mitten in einer 500-MB-Datei
// ---------------------------------------------------------------------------

#[test]
fn der_abbruch_mitten_in_einer_500_mb_datei_kehrt_binnen_100_ms_zurueck() {
    let _reihum = ZEITMESSUNG
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner());
    let ordner = Pruefordner::neu("abbruch-500mb");
    let quelle = ordner.unter("riesig.bin");
    let groesse = 500 * 1024 * 1024;
    volle_datei(&quelle, groesse);
    let ziel = ordner.ordner("ziel");

    let auftrag = Auftrag::kopieren(vec![quelle], &ziel)
        // Auf demselben APFS-Datentraeger klont `copyfile(3)` sonst, und ein
        // Klon ist fertig, bevor ein Abbruch ihn erreichen koennte. Geprueft
        // wird hier der Weg, den KRK auf jedem Ziel ohne Klonunterstuetzung
        // geht: ein Datentraeger mehr, ein Netzlaufwerk, ein USB-Stick.
        .mit_uebertragung(Uebertragungsart::ImmerBytes);

    let lauf = starten(auftrag, Arc::new(OhnePapierkorb));
    // Lange genug, dass die Uebertragung wirklich in der Datei steht, und kurz
    // genug, dass von 500 MB noch reichlich uebrig ist.
    std::thread::sleep(Duration::from_millis(40));

    let vor_dem_abbruch = Instant::now();
    lauf.abbrechen();
    let bericht = bericht_abholen(lauf.meldungen());
    let bis_zur_rueckkehr = vor_dem_abbruch.elapsed();
    lauf.warten();

    assert_eq!(
        bericht.abschluss,
        Abschluss::Abgebrochen,
        "der Lauf hat den Abbruch nicht bemerkt"
    );
    assert!(
        bis_zur_rueckkehr < Duration::from_millis(100),
        "der Abbruch kam nach {bis_zur_rueckkehr:?} zurueck, erlaubt sind 100 ms"
    );
    assert!(
        bericht.bytes > 0 && bericht.bytes < groesse,
        "gemeldet sind {} von {groesse} Bytes; der Abbruch lag nicht mitten in der Datei",
        bericht.bytes
    );
    assert_eq!(
        bericht.eintraege, 0,
        "eine abgebrochene Datei ist kein uebertragener Eintrag"
    );
    assert!(
        !ziel.join("riesig.bin").exists(),
        "die halbe Datei ist am Ziel liegen geblieben"
    );
}

#[test]
fn dieselben_500_mb_sind_als_klon_lange_vor_der_frist_fertig() {
    let _reihum = ZEITMESSUNG
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner());
    let ordner = Pruefordner::neu("klon-500mb");
    let quelle = ordner.unter("riesig.bin");
    let groesse = 500 * 1024 * 1024;
    loch_datei(&quelle, groesse);
    let ziel = ordner.ordner("ziel");

    let (spanne, bericht) =
        gemessen(|| durchlaufen_ohne_papierkorb(Auftrag::kopieren(vec![quelle], &ziel)));

    assert_eq!(bericht.abschluss, Abschluss::Fertig);
    assert_eq!(bericht.eintraege, 1);
    assert!(
        spanne < Duration::from_millis(100),
        "der Klon von 500 MB hat {spanne:?} gedauert"
    );
    assert_eq!(
        fs::metadata(ziel.join("riesig.bin"))
            .expect("die Kopie fehlt")
            .len(),
        groesse
    );
}

// ---------------------------------------------------------------------------
// Abnahmepunkt 4: ein Eintrag ohne Leserecht wird uebersprungen, die uebrigen
// laufen durch
// ---------------------------------------------------------------------------

#[test]
fn ein_eintrag_ohne_leserecht_wird_uebersprungen_und_gemeldet() {
    let ordner = Pruefordner::neu("ohne-leserecht");
    let quelle = ordner.ordner("quelle");
    for nummer in 0..5 {
        fs::write(quelle.join(format!("frei-{nummer}.txt")), "lesbar\n")
            .expect("Datei laesst sich nicht schreiben");
    }
    let gesperrt = quelle.join("gesperrt.txt");
    fs::write(&gesperrt, "geheim\n").expect("Datei laesst sich nicht schreiben");
    fs::set_permissions(&gesperrt, fs::Permissions::from_mode(0o000))
        .expect("Rechte lassen sich nicht setzen");

    let ziel = ordner.ordner("ziel");
    let bericht = durchlaufen_ohne_papierkorb(Auftrag::kopieren(vec![quelle], &ziel));

    assert_eq!(bericht.abschluss, Abschluss::Fertig);
    assert_eq!(
        bericht.uebersprungen.len(),
        1,
        "uebersprungen: {:?}",
        bericht.uebersprungen
    );
    let uebersprungen = &bericht.uebersprungen[0];
    assert_eq!(uebersprungen.pfad, gesperrt);
    assert_eq!(uebersprungen.grund, "keine Rechte");

    let angekommen = ziel.join("quelle");
    for nummer in 0..5 {
        assert!(
            angekommen.join(format!("frei-{nummer}.txt")).exists(),
            "frei-{nummer}.txt ist nicht angekommen, obwohl der Stapel weiterlaufen sollte"
        );
    }
    assert!(
        !angekommen.join("gesperrt.txt").exists(),
        "der gesperrte Eintrag ist trotzdem angekommen"
    );
    assert_eq!(bericht.eintraege, 6, "fuenf Dateien und der Ordner");
}

// ---------------------------------------------------------------------------
// Verschieben, Loeschen, Anlegen, Umbenennen
// ---------------------------------------------------------------------------

#[test]
fn ein_ordner_mit_inhalt_wird_verschoben_und_bleibt_nicht_zurueck() {
    let ordner = Pruefordner::neu("verschieben-ordner");
    let quelle = ordner.unter("quelle");
    baum_anlegen(&quelle, 30);
    let ziel = ordner.ordner("ziel");

    let bericht = durchlaufen_ohne_papierkorb(Auftrag::verschieben(vec![quelle.clone()], &ziel));

    assert_eq!(bericht.abschluss, Abschluss::Fertig);
    assert!(!quelle.exists(), "die Quelle steht noch da");
    assert_eq!(eintraege_zaehlen(&ziel.join("quelle")), 30);
}

#[test]
fn der_papierkorb_bekommt_die_auswahl_und_der_kern_kennt_ihn_nur_als_schnittstelle() {
    let ordner = Pruefordner::neu("papierkorb");
    let eins = ordner.datei("eins.txt", "a");
    let zwei = ordner.datei("zwei.txt", "b");
    let attrappe = Arc::new(Papierkorbattrappe::default());

    let bericht = durchlaufen(
        Auftrag::in_den_papierkorb(vec![eins.clone(), zwei.clone()]),
        attrappe.clone(),
    );

    assert_eq!(bericht.abschluss, Abschluss::Fertig);
    assert_eq!(bericht.eintraege, 2);
    let geraeumt = attrappe.geraeumt.lock().expect("Attrappe vergiftet");
    assert_eq!(*geraeumt, vec![eins, zwei]);
}

#[test]
fn ohne_papierkorb_wird_nichts_geloescht_sondern_gemeldet() {
    let ordner = Pruefordner::neu("ohne-papierkorb");
    let datei = ordner.datei("bleibt.txt", "a");

    let bericht = durchlaufen_ohne_papierkorb(Auftrag::in_den_papierkorb(vec![datei.clone()]));

    assert_eq!(bericht.eintraege, 0);
    assert_eq!(bericht.uebersprungen.len(), 1);
    assert!(datei.exists(), "ohne Papierkorb darf nichts verschwinden");
}

#[test]
fn endgueltiges_loeschen_raeumt_einen_ordner_mit_inhalt_ab() {
    let ordner = Pruefordner::neu("endgueltig");
    let baum = ordner.unter("baum");
    baum_anlegen(&baum, 40);

    let bericht = durchlaufen_ohne_papierkorb(Auftrag::endgueltig_loeschen(vec![baum.clone()]));

    assert_eq!(bericht.abschluss, Abschluss::Fertig);
    assert_eq!(bericht.eintraege, 41, "40 Eintraege und der Wurzelordner");
    assert!(!baum.exists(), "der Baum steht noch da");
}

#[test]
fn anlegen_legt_an_und_ueberschreibt_nichts() {
    let ordner = Pruefordner::neu("anlegen");

    let neuer = ordner_anlegen(ordner.pfad(), "Neuer Ordner").expect("Ordner nicht angelegt");
    assert!(neuer.is_dir());
    let zweiter = ordner_anlegen(ordner.pfad(), "Neuer Ordner");
    assert_eq!(
        zweiter.expect_err("das haette scheitern muessen").kind(),
        std::io::ErrorKind::AlreadyExists
    );

    let datei = datei_anlegen(ordner.pfad(), "notiz.md").expect("Datei nicht angelegt");
    assert_eq!(fs::read_to_string(&datei).expect("nicht lesbar"), "");
    fs::write(&datei, "Inhalt").expect("nicht schreibbar");
    assert!(
        datei_anlegen(ordner.pfad(), "notiz.md").is_err(),
        "ein zweites Anlegen haette den Inhalt geleert"
    );
    assert_eq!(fs::read_to_string(&datei).expect("nicht lesbar"), "Inhalt");
}

#[test]
fn anlegen_weist_einen_namen_zurueck_der_keiner_ist() {
    let ordner = Pruefordner::neu("anlegen-namen");
    for name in ["", "   ", "a/b", ".", ".."] {
        assert!(
            ordner_anlegen(ordner.pfad(), name).is_err(),
            "der Name {name:?} haette nicht durchgehen duerfen"
        );
    }
}

#[test]
fn umbenennen_benennt_um_und_ueberschreibt_nichts() {
    let ordner = Pruefordner::neu("umbenennen");
    let alt = ordner.datei("alt.txt", "Inhalt");
    let belegt = ordner.datei("belegt.txt", "fremder Inhalt");

    let neu = umbenennen(&alt, "neu.txt").expect("Umbenennen gescheitert");
    assert!(!alt.exists());
    assert_eq!(fs::read_to_string(&neu).expect("nicht lesbar"), "Inhalt");

    let fehler = umbenennen(&neu, "belegt.txt").expect_err("das haette scheitern muessen");
    assert_eq!(fehler.kind(), std::io::ErrorKind::AlreadyExists);
    assert_eq!(
        fs::read_to_string(&belegt).expect("nicht lesbar"),
        "fremder Inhalt"
    );
}

// ---------------------------------------------------------------------------
// Konflikte
// ---------------------------------------------------------------------------

#[test]
fn die_regel_ueberspringen_laesst_das_vorhandene_stehen() {
    let ordner = Pruefordner::neu("konflikt-ueberspringen");
    let quelle = ordner.datei("bericht.txt", "neu");
    let ziel = ordner.ordner("ziel");
    fs::write(ziel.join("bericht.txt"), "alt").expect("nicht schreibbar");

    let bericht = durchlaufen_ohne_papierkorb(
        Auftrag::kopieren(vec![quelle], &ziel).mit_konfliktregel(Konfliktregel::Ueberspringen),
    );

    assert_eq!(bericht.eintraege, 0);
    assert_eq!(bericht.uebersprungen.len(), 1);
    assert_eq!(
        fs::read_to_string(ziel.join("bericht.txt")).expect("nicht lesbar"),
        "alt"
    );
}

#[test]
fn die_regel_ueberschreiben_ersetzt_das_vorhandene() {
    let ordner = Pruefordner::neu("konflikt-ueberschreiben");
    let quelle = ordner.datei("bericht.txt", "neu");
    let ziel = ordner.ordner("ziel");
    fs::write(ziel.join("bericht.txt"), "alt").expect("nicht schreibbar");

    let bericht = durchlaufen_ohne_papierkorb(
        Auftrag::kopieren(vec![quelle], &ziel).mit_konfliktregel(Konfliktregel::Ueberschreiben),
    );

    assert_eq!(bericht.eintraege, 1);
    assert_eq!(
        fs::read_to_string(ziel.join("bericht.txt")).expect("nicht lesbar"),
        "neu"
    );
}

#[test]
fn die_regel_umbenennen_legt_die_kopie_daneben() {
    let ordner = Pruefordner::neu("konflikt-umbenennen");
    let quelle = ordner.datei("bericht.txt", "neu");
    let ziel = ordner.ordner("ziel");
    fs::write(ziel.join("bericht.txt"), "alt").expect("nicht schreibbar");

    let bericht = durchlaufen_ohne_papierkorb(
        Auftrag::kopieren(vec![quelle], &ziel)
            .mit_konfliktregel(Konfliktregel::AutomatischUmbenennen),
    );

    assert_eq!(bericht.eintraege, 1);
    assert_eq!(
        fs::read_to_string(ziel.join("bericht.txt")).expect("nicht lesbar"),
        "alt"
    );
    assert_eq!(
        fs::read_to_string(ziel.join("bericht Kopie.txt")).expect("die Kopie fehlt"),
        "neu"
    );
    assert_eq!(
        freier_name(&ziel.join("bericht.txt")),
        "bericht Kopie 2.txt"
    );
}

#[test]
fn die_rueckfrage_gilt_auf_wunsch_fuer_alle_weiteren() {
    let ordner = Pruefordner::neu("konflikt-fragen");
    let ziel = ordner.ordner("ziel");
    let mut quellen = Vec::new();
    for nummer in 0..3 {
        let name = format!("datei-{nummer}.txt");
        quellen.push(ordner.datei(&name, "neu"));
        fs::write(ziel.join(&name), "alt").expect("nicht schreibbar");
    }

    let lauf = starten(
        Auftrag::kopieren(quellen, &ziel).mit_konfliktregel(Konfliktregel::Fragen),
        Arc::new(OhnePapierkorb),
    );
    let bericht = fragen_beantworten(&lauf);
    lauf.warten();

    assert_eq!(
        bericht.eintraege, 3,
        "alle drei muessten ueberschrieben sein"
    );
    for nummer in 0..3 {
        assert_eq!(
            fs::read_to_string(ziel.join(format!("datei-{nummer}.txt"))).expect("nicht lesbar"),
            "neu"
        );
    }
}

/// Beantwortet die erste Konfliktfrage mit "ueberschreiben, fuer alle
/// weiteren" und zaehlt, wie oft ueberhaupt gefragt wurde.
fn fragen_beantworten(lauf: &Lauf) -> Bericht {
    let mut gefragt = 0;
    while let Ok(meldung) = lauf.meldungen().recv() {
        match meldung {
            Meldung::Konflikt { antwort, .. } => {
                gefragt += 1;
                assert_eq!(
                    gefragt, 1,
                    "nach 'fuer alle weiteren' darf nicht mehr gefragt werden"
                );
                antwort
                    .send(Konfliktentscheid::fuer_alle(
                        Konfliktantwort::Ueberschreiben,
                    ))
                    .expect("Antwort laesst sich nicht senden");
            }
            Meldung::Fertig(bericht) => return bericht,
            _ => {}
        }
    }
    panic!("der Lauf hat keine Abschlussmeldung geschickt");
}

#[test]
fn eine_unbeantwortete_rueckfrage_gilt_als_abbruch() {
    let ordner = Pruefordner::neu("konflikt-unbeantwortet");
    let quelle = ordner.datei("bericht.txt", "neu");
    let ziel = ordner.ordner("ziel");
    fs::write(ziel.join("bericht.txt"), "alt").expect("nicht schreibbar");

    let lauf = starten(
        Auftrag::kopieren(vec![quelle], &ziel).mit_konfliktregel(Konfliktregel::Fragen),
        Arc::new(OhnePapierkorb),
    );
    let mut abschluss = None;
    while let Ok(meldung) = lauf.meldungen().recv() {
        match meldung {
            // Der Kanal fuer die Antwort wird hier fallen gelassen.
            Meldung::Konflikt { .. } => {}
            Meldung::Fertig(bericht) => {
                abschluss = Some(bericht.abschluss);
                break;
            }
            _ => {}
        }
    }
    lauf.warten();

    assert_eq!(abschluss, Some(Abschluss::Abgebrochen));
    assert_eq!(
        fs::read_to_string(ziel.join("bericht.txt")).expect("nicht lesbar"),
        "alt",
        "ohne Antwort darf nichts ueberschrieben werden"
    );
}

#[test]
fn ein_ordner_auf_einen_gleichnamigen_ordner_ist_kein_konflikt() {
    let ordner = Pruefordner::neu("verschmelzen");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("neu.txt"), "neu").expect("nicht schreibbar");
    let ziel = ordner.ordner("ziel");
    let vorhanden = ziel.join("quelle");
    fs::create_dir(&vorhanden).expect("nicht anlegbar");
    fs::write(vorhanden.join("alt.txt"), "alt").expect("nicht schreibbar");

    let bericht = durchlaufen_ohne_papierkorb(
        // Die Regel "fragen" wuerde hier fragen, wenn es ein Konflikt waere;
        // niemand antwortet, der Lauf braeche also ab.
        Auftrag::kopieren(vec![quelle], &ziel).mit_konfliktregel(Konfliktregel::Fragen),
    );

    assert_eq!(bericht.abschluss, Abschluss::Fertig);
    assert!(vorhanden.join("alt.txt").exists(), "der alte Inhalt fehlt");
    assert!(vorhanden.join("neu.txt").exists(), "der neue Inhalt fehlt");
}

// ---------------------------------------------------------------------------
// Verknuepfungen und Grenzfaelle
// ---------------------------------------------------------------------------

#[test]
fn eine_verknuepfung_wird_kopiert_und_nicht_ihr_ziel() {
    let ordner = Pruefordner::neu("verknuepfung");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("echt.txt"), "Inhalt").expect("nicht schreibbar");
    std::os::unix::fs::symlink("echt.txt", quelle.join("verweis.txt"))
        .expect("Verknuepfung nicht anlegbar");
    let ziel = ordner.ordner("ziel");

    let bericht = durchlaufen_ohne_papierkorb(Auftrag::kopieren(vec![quelle], &ziel));

    assert_eq!(bericht.abschluss, Abschluss::Fertig);
    let verweis = ziel.join("quelle/verweis.txt");
    let angaben = fs::symlink_metadata(&verweis).expect("die Verknuepfung fehlt");
    assert!(
        angaben.is_symlink(),
        "aus dem Verweis ist eine Datei geworden"
    );
    assert_eq!(
        fs::read_link(&verweis).expect("Ziel nicht lesbar"),
        Path::new("echt.txt")
    );
}

#[test]
fn ein_ordner_laesst_sich_nicht_in_sich_selbst_kopieren() {
    let ordner = Pruefordner::neu("in-sich-selbst");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("datei.txt"), "Inhalt").expect("nicht schreibbar");
    let ziel = quelle.join("unten");
    fs::create_dir(&ziel).expect("nicht anlegbar");

    let bericht = durchlaufen_ohne_papierkorb(Auftrag::kopieren(vec![quelle], &ziel));

    assert_eq!(bericht.eintraege, 0);
    assert_eq!(bericht.uebersprungen.len(), 1);
    assert_eq!(
        bericht.uebersprungen[0].grund,
        "das Ziel liegt in der Quelle"
    );
}

#[test]
fn eine_quelle_die_es_nicht_gibt_wird_gemeldet_und_haelt_den_stapel_nicht_auf() {
    let ordner = Pruefordner::neu("fehlende-quelle");
    let da = ordner.datei("da.txt", "Inhalt");
    let weg = ordner.unter("weg.txt");
    let ziel = ordner.ordner("ziel");

    let bericht = durchlaufen_ohne_papierkorb(Auftrag::kopieren(vec![weg.clone(), da], &ziel));

    assert_eq!(bericht.abschluss, Abschluss::Fertig);
    assert_eq!(bericht.eintraege, 1);
    assert_eq!(bericht.uebersprungen.len(), 1);
    assert_eq!(bericht.uebersprungen[0].pfad, weg);
    assert_eq!(bericht.uebersprungen[0].grund, "gibt es nicht mehr");
    assert!(ziel.join("da.txt").exists());
}

#[test]
fn der_fortschritt_meldet_sich_waehrend_einer_grossen_kopie() {
    let ordner = Pruefordner::neu("fortschritt");
    let quelle = ordner.unter("quelle");
    baum_anlegen(&quelle, 200);
    let ziel = ordner.ordner("ziel");

    let lauf = starten(
        Auftrag::kopieren(vec![quelle], &ziel),
        Arc::new(OhnePapierkorb),
    );
    let mut fortschritte = 0;
    let mut letzte_zahl = 0;
    let mut bericht = None;
    while let Ok(meldung) = lauf.meldungen().recv() {
        match meldung {
            Meldung::Fortschritt(stand) => {
                assert!(
                    stand.eintraege >= letzte_zahl,
                    "der Fortschritt ist zurueckgelaufen"
                );
                letzte_zahl = stand.eintraege;
                fortschritte += 1;
            }
            Meldung::Fertig(fertig) => {
                bericht = Some(fertig);
                break;
            }
            _ => {}
        }
    }
    lauf.warten();

    let bericht = bericht.expect("keine Abschlussmeldung");
    assert_eq!(bericht.abschluss, Abschluss::Fertig);
    assert!(fortschritte > 0, "es kam keine einzige Fortschrittsmeldung");
    assert!(
        u64::try_from(fortschritte).expect("Zahl passt nicht") <= bericht.eintraege,
        "die Meldungen sind nicht gebuendelt: {fortschritte} Meldungen fuer {} Eintraege",
        bericht.eintraege
    );
}
