//! Abnahme von Schritt 5 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`: der Umbau
//! eines Archivs im Kern, ohne Oberflaeche und ohne Fernauftrag.
//!
//! Jede Probe baut ihre Archive selbst in einem Pruefordner, mit den Kisten,
//! mit denen KRK sie liest (`gemeinsam::archivbau`), und faehrt den Umbau
//! ueber [`umbau::starten`], also auf einem Faden mit Pfand und mit
//! derselben Konfliktfrage ueber den Meldekanal wie jeder andere Vorgang.
//! Die Gegenprobe laeuft ueber die Werkzeuge des Systems (`/usr/bin/unzip`,
//! `/usr/bin/tar`), damit KRK sich nicht nur selbst bestaetigt.
//!
//! **Der volle Datentraeger steht nicht hier**, sondern im Pruefmodul von
//! `src/archiv/umbau.rs`: er braucht den privaten Weg mit Schreibgrenze, und
//! der Betrieb soll keinen oeffentlichen dafuer tragen. Die Proben hier
//! stehen in einer eigenen Datei und nicht in `tests/archiv.rs`, weil
//! Schritt 2 des Plans jene Datei zur selben Zeit erweitert.

mod gemeinsam;

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::time::{Duration, UNIX_EPOCH};

use flate2::read::MultiGzDecoder;
use gemeinsam::archivbau::{gzip_glied, tar_gz_packen, tar_packen, zip_packen};
use gemeinsam::{Pruefordner, aufrufstellen, betriebscode, quelldateien};
use krk_core::archiv::umbau::{
    self, Datenquelle, Geoeffnet, Leer, Neuart, Neueintrag, Umbauplan, Umbauschritt,
};
use krk_core::archiv::{Archivformat, Archivzugang, Inhaltsverzeichnis, Innenpfad, Lesbarkeit};
use krk_core::fern::{Fernfaden, faden};
use krk_core::operation::Konfliktregel;
use krk_core::operation::fortschritt::{
    Abschluss, Bericht, Konfliktantwort, Konfliktentscheid, Meldung,
};
use krk_core::sprache::{Text, Zahlwort, anzahl, satz, text};
use krk_core::verzeichnis::Typ;
use zip::ZipWriter;
use zip::write::FullFileOptions;

/// Wie lange eine Probe hoechstens auf die naechste Meldung wartet.
const GEDULD: Duration = Duration::from_secs(120);

/// Packt einen Baum in ein Archiv.
type Packer = fn(&Path, &Path);

/// Die drei beschreibbaren Formate: Endung und Packer.
fn formen() -> [(&'static str, Packer); 3] {
    [
        ("zip", zip_packen),
        ("tar", tar_packen),
        ("tgz", tar_gz_packen),
    ]
}

fn innen(text: &str) -> Innenpfad {
    Innenpfad::aus_archivname(text).expect("ein sicherer Name")
}

fn zugang(pfad: &Path) -> Arc<Archivzugang> {
    let format = Archivformat::aus_namen(pfad.file_name().expect("Name"))
        .expect("die Probe gibt ihrem Archiv eine Endung aus der Tafel");
    Arc::new(Archivzugang::neu(pfad.to_path_buf(), format))
}

/// Das Verzeichnis eines Archivs, frisch gelesen in einem Faden mit Pfand.
fn verzeichnis(pfad: &Path) -> Arc<Inhaltsverzeichnis> {
    let zugang = zugang(pfad);
    faden("krk-umbauprobe".into(), move |pfand| {
        zugang.verzeichnis(pfand, &AtomicBool::new(false))
    })
    .expect("Faden")
    .join()
    .expect("keine Panik")
    .expect("das Archiv laesst sich lesen")
}

fn pfade(verzeichnis: &Inhaltsverzeichnis) -> Vec<String> {
    let mut pfade: Vec<String> = verzeichnis
        .eintraege()
        .iter()
        .map(|eintrag| eintrag.innen.als_text().to_owned())
        .collect();
    pfade.sort();
    pfade
}

/// Baut einen Plan in einem Faden mit Pfand, weil `Neueintrag::lokal` die
/// Platte fragt.
fn plan_bauen(bauen: impl FnOnce(&Fernfaden) -> Umbauplan + Send + 'static) -> Umbauplan {
    faden("krk-umbauplan".into(), bauen)
        .expect("Faden")
        .join()
        .expect("keine Panik")
}

fn lokal(pfand: &Fernfaden, pfad: &Path) -> Neueintrag {
    Neueintrag::lokal(pfand, pfad).expect("die lokale Quelle laesst sich lesen")
}

/// Was eine Fahrt gemeldet hat.
struct Fahrt {
    bericht: Bericht,
    fragen: usize,
    /// Fortschrittsmeldungen bis einschliesslich der letzten Konfliktfrage.
    fortschritt_vor_letzter_frage: usize,
}

/// Faehrt einen Umbau und beantwortet die Konfliktfragen der Reihe nach.
fn fahren(
    archiv: &Path,
    plan: Umbauplan,
    regel: Konfliktregel,
    antworten: Vec<Konfliktentscheid>,
) -> Fahrt {
    let lauf = umbau::starten(zugang(archiv), plan, regel);
    let mut antworten = antworten.into_iter();
    let mut fragen = 0;
    let mut fortschritt = 0;
    let mut fortschritt_vor_letzter_frage = 0;
    loop {
        match lauf
            .meldungen()
            .recv_timeout(GEDULD)
            .expect("der Umbau meldet sich")
        {
            Meldung::Konflikt { antwort, .. } => {
                fragen += 1;
                fortschritt_vor_letzter_frage = fortschritt;
                let entscheid = antworten
                    .next()
                    .unwrap_or_else(|| Konfliktentscheid::einmal(Konfliktantwort::Abbrechen));
                antwort.send(entscheid).expect("die Antwort kommt an");
            }
            Meldung::Fortschritt(_) => fortschritt += 1,
            Meldung::Uebersprungen(_) => {}
            Meldung::Fertig(bericht) => {
                lauf.warten();
                return Fahrt {
                    bericht,
                    fragen,
                    fortschritt_vor_letzter_frage,
                };
            }
        }
    }
}

/// Ob im Ordner ein Zwischenname liegt.
fn zwischennamen(ordner: &Path) -> Vec<String> {
    fs::read_dir(ordner)
        .expect("Ordner lesbar")
        .map(|eintrag| {
            eintrag
                .expect("Eintrag")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.starts_with(umbau_vorsatz()))
        .collect()
}

fn umbau_vorsatz() -> &'static str {
    krk_core::archiv::ersetzen::ZWISCHENVORSATZ
}

/// Die Gruenden des Berichts zu einem Pfad.
fn gruende(bericht: &Bericht, pfad: &Path) -> Vec<String> {
    bericht
        .uebersprungen
        .iter()
        .filter(|zeile| zeile.pfad == pfad)
        .map(|zeile| zeile.grund.clone())
        .collect()
}

fn unveraendert_weil(grund: Text) -> String {
    satz(Text::ArchivUnveraendert, &[("grund", &text(grund))])
}

/// Prueft ein Archiv mit dem Werkzeug des Systems.
fn system_liest(archiv: &Path, endung: &str) {
    let stand = match endung {
        "zip" => Command::new("/usr/bin/unzip")
            .arg("-tqq")
            .arg(archiv)
            .status(),
        "tar" => Command::new("/usr/bin/tar")
            .arg("-tf")
            .arg(archiv)
            .output()
            .map(|aus| aus.status),
        _ => Command::new("/usr/bin/tar")
            .arg("-tzf")
            .arg(archiv)
            .output()
            .map(|aus| aus.status),
    }
    .expect("das Werkzeug des Systems laesst sich starten");
    assert!(
        stand.success(),
        "das System liest {} nicht",
        archiv.display()
    );
}

/// Packt ein Archiv mit `/usr/bin/tar` aus; libarchive liest auch Zip.
fn system_auspacken(archiv: &Path, ziel: &Path) {
    fs::create_dir_all(ziel).expect("Ziel anlegen");
    let stand = Command::new("/usr/bin/tar")
        .arg("-xf")
        .arg(archiv)
        .arg("-C")
        .arg(ziel)
        .status()
        .expect("tar laesst sich starten");
    assert!(
        stand.success(),
        "das System packt {} nicht aus",
        archiv.display()
    );
}

/// Die Bytes jedes tar-Eintrags samt Vorkoepfen und Daten, nach Namen.
fn tar_bloecke(archiv: &Path) -> HashMap<String, Vec<u8>> {
    let mut daten = Vec::new();
    let datei = File::open(archiv).expect("Archiv oeffnen");
    if archiv.extension().is_some_and(|endung| endung == "tgz") {
        MultiGzDecoder::new(datei)
            .read_to_end(&mut daten)
            .expect("entpacken");
    } else {
        let mut datei = datei;
        datei.read_to_end(&mut daten).expect("lesen");
    }
    let mut bloecke = HashMap::new();
    let mut leser = tar::Archive::new(Cursor::new(&daten));
    let mut anfang = 0u64;
    for eintrag in leser.entries().expect("tar lesbar") {
        let eintrag = eintrag.expect("Eintrag lesbar");
        let ende = eintrag.raw_file_position() + eintrag.size().div_ceil(512) * 512;
        let name = String::from_utf8_lossy(&eintrag.path_bytes()).into_owned();
        let von = usize::try_from(anfang).expect("klein");
        let bis = usize::try_from(ende).expect("klein");
        bloecke.insert(name, daten[von..bis].to_vec());
        anfang = ende;
    }
    bloecke
}

/// Der Baum, aus dem die meisten Proben ihr Archiv packen.
fn grundbaum(ordner: &Pruefordner) -> PathBuf {
    ordner.ordner("baum");
    ordner.ordner("baum/a");
    ordner.ordner("baum/a/b");
    ordner.datei("baum/a/x.txt", "x-inhalt");
    ordner.datei("baum/a/b/y.txt", "y-inhalt");
    ordner.datei("baum/c.txt", "c-inhalt");
    ordner.verknuepfung("baum/verweis", "c.txt");
    ordner.unter("baum")
}

/// Hinzufuegen, Umbenennen und Entfernen ergeben je Format genau das
/// geplante Archiv; das System liest es, und ein unveraenderter tar-Eintrag
/// geht Byte fuer Byte hinueber.
#[test]
fn hinzufuegen_umbenennen_und_entfernen_ergeben_das_geplante_archiv() {
    for (endung, packen) in formen() {
        let ordner = Pruefordner::neu(&format!("umbau-plan-{endung}"));
        let baum = grundbaum(&ordner);
        let archiv = ordner.unter(&format!("archiv.{endung}"));
        packen(&baum, &archiv);
        ordner.ordner("neu");
        ordner.ordner("neu/tief");
        ordner.datei("neu/n.txt", "n-inhalt");
        ordner.datei("neu/tief/t.txt", "t-inhalt");
        ordner.datei("einzeln.txt", "einzeln-inhalt");
        let vorher_zeit = verzeichnis(&archiv)
            .eintrag(&innen("a/x.txt"))
            .expect("a/x.txt")
            .geaendert;

        let neu = ordner.unter("neu");
        let einzeln = ordner.unter("einzeln.txt");
        let plan = plan_bauen(move |pfand| {
            Umbauplan::neu()
                .mit(Umbauschritt::Hinzufuegen {
                    ordner: Innenpfad::wurzel(),
                    eintrag: lokal(pfand, &einzeln),
                })
                .mit(Umbauschritt::Hinzufuegen {
                    ordner: innen("a"),
                    eintrag: lokal(pfand, &neu),
                })
                .mit(Umbauschritt::Umhaengen {
                    von: innen("c.txt"),
                    nach: innen("a/c2.txt"),
                })
                .mit(Umbauschritt::Entfernen(innen("a/b")))
        });
        let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());

        assert_eq!(fahrt.bericht.abschluss, Abschluss::Fertig, "{endung}");
        assert!(
            fahrt.bericht.uebersprungen.is_empty(),
            "{endung}: {:?}",
            fahrt.bericht.uebersprungen
        );
        assert_eq!(fahrt.bericht.eintraege, 4, "{endung}");
        assert_eq!(fahrt.fragen, 0, "{endung}");
        let nachher = verzeichnis(&archiv);
        assert_eq!(
            pfade(&nachher),
            [
                "a",
                "a/c2.txt",
                "a/neu",
                "a/neu/n.txt",
                "a/neu/tief",
                "a/neu/tief/t.txt",
                "a/x.txt",
                "einzeln.txt",
                "verweis",
            ],
            "{endung}"
        );
        assert_eq!(
            nachher
                .eintrag(&innen("a/x.txt"))
                .expect("a/x.txt")
                .geaendert,
            vorher_zeit,
            "{endung}: die Zeit eines unveraenderten Eintrags bleibt"
        );
        let einzeln_angaben = fs::metadata(ordner.unter("einzeln.txt")).expect("Angaben");
        let einzeln_eintrag = nachher.eintrag(&innen("einzeln.txt")).expect("einzeln.txt");
        assert_eq!(
            einzeln_eintrag.rechte,
            Some(einzeln_angaben.permissions().mode() & 0o7777),
            "{endung}"
        );
        assert_eq!(
            einzeln_eintrag
                .geaendert
                .duration_since(UNIX_EPOCH)
                .expect("nach 1970")
                .as_secs(),
            einzeln_angaben
                .modified()
                .expect("Zeit")
                .duration_since(UNIX_EPOCH)
                .expect("nach 1970")
                .as_secs(),
            "{endung}"
        );
        assert_eq!(
            nachher.eintrag(&innen("verweis")).expect("verweis").typ,
            Typ::Verknuepfung,
            "{endung}"
        );

        system_liest(&archiv, endung);
        let aus = ordner.unter("aus");
        system_auspacken(&archiv, &aus);
        for (pfad, inhalt) in [
            ("a/c2.txt", "c-inhalt"),
            ("a/x.txt", "x-inhalt"),
            ("a/neu/n.txt", "n-inhalt"),
            ("a/neu/tief/t.txt", "t-inhalt"),
            ("einzeln.txt", "einzeln-inhalt"),
        ] {
            assert_eq!(
                fs::read_to_string(aus.join(pfad)).expect("ausgepackt"),
                inhalt,
                "{endung}: {pfad}"
            );
        }
        assert_eq!(
            fs::read_link(aus.join("verweis")).expect("eine Verknuepfung"),
            Path::new("c.txt"),
            "{endung}"
        );
        assert!(!aus.join("a/b").exists(), "{endung}: a/b ist entfernt");
        assert!(zwischennamen(ordner.pfad()).is_empty(), "{endung}");
    }
}

/// Ein unveraenderter tar-Eintrag geht samt Kopf Byte fuer Byte hinueber,
/// auch neben einem umbenannten.
#[test]
fn ein_unveraenderter_tar_eintrag_geht_byte_fuer_byte_hinueber() {
    for (endung, packen) in formen().into_iter().skip(1) {
        let ordner = Pruefordner::neu(&format!("umbau-tarroh-{endung}"));
        let baum = grundbaum(&ordner);
        let archiv = ordner.unter(&format!("archiv.{endung}"));
        packen(&baum, &archiv);
        let vorher = tar_bloecke(&archiv);

        let plan = Umbauplan::neu().mit(Umbauschritt::Umhaengen {
            von: innen("c.txt"),
            nach: innen("a/c2.txt"),
        });
        let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());
        assert_eq!(fahrt.bericht.abschluss, Abschluss::Fertig);

        let nachher = tar_bloecke(&archiv);
        let unveraendert: Vec<&String> = vorher
            .keys()
            .filter(|name| name.as_str() != "c.txt")
            .collect();
        assert_eq!(unveraendert.len(), 5, "{endung}: {:?}", vorher.keys());
        for name in unveraendert {
            assert_eq!(
                nachher.get(name),
                vorher.get(name),
                "{endung}: {name} ist nicht Byte fuer Byte geblieben"
            );
        }
        system_liest(&archiv, endung);
    }
}

/// Setzt das Verfahren eines Zip-Eintrags auf eines, das KRK nicht entpackt,
/// im lokalen Kopf und im zentralen Satz.
fn verfahren_setzen(archiv: &Path, name: &str, verfahren: u16) {
    let (lokal, zentral) = {
        let mut kiste =
            zip::ZipArchive::new(File::open(archiv).expect("Zip oeffnen")).expect("Zip lesbar");
        let stelle = kiste.index_for_name(name).expect("Eintrag");
        let eintrag = kiste.by_index_raw(stelle).expect("Eintrag");
        (eintrag.header_start(), eintrag.central_header_start())
    };
    let mut datei = OpenOptions::new()
        .write(true)
        .open(archiv)
        .expect("Zip zum Patchen oeffnen");
    for stelle in [lokal + 8, zentral + 10] {
        datei.seek(SeekFrom::Start(stelle)).expect("springen");
        datei.write_all(&verfahren.to_le_bytes()).expect("patchen");
    }
}

/// Was ein Zip-Eintrag roh ist: Pruefsumme, Verfahren, gepackte Bytes,
/// Zusatzfelder und Typ.
fn zip_roh(archiv: &Path, name: &str) -> (u32, String, Vec<u8>, Vec<u8>, bool, Option<u32>) {
    let mut kiste =
        zip::ZipArchive::new(File::open(archiv).expect("Zip oeffnen")).expect("Zip lesbar");
    let stelle = kiste.index_for_name(name).expect("Eintrag");
    let mut eintrag = kiste.by_index_raw(stelle).expect("Eintrag");
    let mut gepackt = Vec::new();
    eintrag.read_to_end(&mut gepackt).expect("roh lesbar");
    (
        eintrag.crc32(),
        format!("{:?}", eintrag.compression()),
        gepackt,
        eintrag.extra_data().unwrap_or_default().to_vec(),
        eintrag.is_symlink(),
        eintrag.unix_mode(),
    )
}

/// Ein unveraenderter Zip-Eintrag wird roh hinuebergetragen und nicht neu
/// gepackt: Pruefsumme, Verfahren, gepackte Bytes, Zusatzfelder (also die
/// Zeitfelder) und das Typkennzeichen einer Verknuepfung bleiben, auch fuer
/// einen Eintrag mit einem Verfahren, das KRK nicht entpackt.
#[test]
fn ein_unveraenderter_zip_eintrag_wird_roh_und_nicht_neu_gepackt() {
    let ordner = Pruefordner::neu("umbau-ziproh");
    let baum = grundbaum(&ordner);
    ordner.datei("baum/fremd.bin", vec![7u8; 3000]);
    ordner.datei("baum/weg.txt", "weg");
    let archiv = ordner.unter("archiv.zip");
    zip_packen(&baum, &archiv);
    verfahren_setzen(&archiv, "fremd.bin", 12);
    let vorher_verzeichnis = verzeichnis(&archiv);
    assert_eq!(
        vorher_verzeichnis
            .eintrag(&innen("fremd.bin"))
            .expect("fremd.bin")
            .lesbarkeit,
        Lesbarkeit::Verfahren("bzip2".to_owned())
    );
    let namen = ["a/x.txt", "a/b/y.txt", "c.txt", "verweis", "fremd.bin"];
    let vorher: Vec<_> = namen.iter().map(|name| zip_roh(&archiv, name)).collect();

    let plan = Umbauplan::neu()
        .mit(Umbauschritt::Entfernen(innen("weg.txt")))
        .mit(Umbauschritt::Hinzufuegen {
            ordner: innen("a"),
            eintrag: Neueintrag {
                name: "leer.txt".to_owned(),
                herkunft: PathBuf::from("/probe/leer.txt"),
                rechte: 0o600,
                geaendert: UNIX_EPOCH + Duration::from_secs(1_700_000_000),
                art: Neuart::Datei(Box::new(Leer)),
            },
        });
    let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());
    assert_eq!(fahrt.bericht.abschluss, Abschluss::Fertig);
    assert!(
        fahrt.bericht.uebersprungen.is_empty(),
        "{:?}",
        fahrt.bericht.uebersprungen
    );

    for (name, vorher) in namen.iter().zip(&vorher) {
        assert_eq!(
            &zip_roh(&archiv, name),
            vorher,
            "{name} ist nicht roh geblieben"
        );
    }
    let nachher = verzeichnis(&archiv);
    assert!(nachher.eintrag(&innen("weg.txt")).is_none());
    let leer = nachher.eintrag(&innen("a/leer.txt")).expect("a/leer.txt");
    assert_eq!(leer.rechte, Some(0o600));
    assert_eq!(
        leer.geaendert,
        UNIX_EPOCH + Duration::from_secs(1_700_000_000)
    );
}

/// Daten, die nach dem ersten MiB anhalten, bis die Probe sie weiterlaufen
/// laesst; die Probe bricht dazwischen ab.
struct Bremse {
    angehalten: SyncSender<()>,
    weiter: Receiver<()>,
    groesse: u64,
}

struct Bremsleser<'a> {
    bremse: &'a Bremse,
    gelesen: u64,
    angehalten: bool,
}

impl Read for Bremsleser<'_> {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        if !self.angehalten && self.gelesen >= 1 << 20 {
            self.angehalten = true;
            let _ = self.bremse.angehalten.send(());
            let _ = self.bremse.weiter.recv();
        }
        let rest = self.bremse.groesse - self.gelesen;
        let stueck = puffer
            .len()
            .min(usize::try_from(rest).unwrap_or(usize::MAX));
        for (stelle, byte) in puffer[..stueck].iter_mut().enumerate() {
            *byte = (self.gelesen as usize + stelle).wrapping_mul(2_654_435_761) as u8;
        }
        self.gelesen += stueck as u64;
        Ok(stueck)
    }
}

impl Datenquelle for Bremse {
    fn oeffnen<'a>(&'a mut self, _pfand: &'a Fernfaden) -> io::Result<Geoeffnet<'a>> {
        Ok(Geoeffnet {
            groesse: self.groesse,
            leser: Box::new(Bremsleser {
                bremse: self,
                gelesen: 0,
                angehalten: false,
            }),
        })
    }
}

/// Daten, die nach einigen Bytes mit einem Fehler enden.
struct Kaputt;

impl Datenquelle for Kaputt {
    fn oeffnen<'a>(&'a mut self, _pfand: &'a Fernfaden) -> io::Result<Geoeffnet<'a>> {
        let anfang = io::repeat(b'k').take(100_000);
        let fehler = FehlerNachAnfang;
        Ok(Geoeffnet {
            groesse: 1 << 20,
            leser: Box::new(anfang.chain(fehler)),
        })
    }
}

struct FehlerNachAnfang;

impl Read for FehlerNachAnfang {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::other("die Quelle ist kaputt"))
    }
}

fn neu_mit(name: &str, art: Neuart) -> Umbauschritt {
    Umbauschritt::Hinzufuegen {
        ordner: Innenpfad::wurzel(),
        eintrag: Neueintrag {
            name: name.to_owned(),
            herkunft: PathBuf::from(format!("/probe/{name}")),
            rechte: 0o644,
            geaendert: UNIX_EPOCH,
            art,
        },
    }
}

/// Ein Abbruch mitten in einem grossen Eintrag und ein Fehler beim Lesen
/// einer Quelle lassen jedes Format Byte fuer Byte stehen, und kein
/// Zwischenname bleibt zurueck. Der volle Datentraeger steht im Pruefmodul
/// von `src/archiv/umbau.rs` (Kopf dieser Datei).
#[test]
fn ein_abgebrochener_oder_gescheiterter_umbau_laesst_das_archiv_byte_fuer_byte_stehen() {
    for (endung, packen) in formen() {
        let ordner = Pruefordner::neu(&format!("umbau-abbruch-{endung}"));
        let baum = grundbaum(&ordner);
        let archiv = ordner.unter(&format!("archiv.{endung}"));
        packen(&baum, &archiv);
        let vorher = fs::read(&archiv).expect("vorher");

        // Abbruch mitten in einem Eintrag von 64 MiB.
        let (angehalten_sender, angehalten) = sync_channel(1);
        let (weiter, weiter_empfaenger) = sync_channel(1);
        let plan = Umbauplan::neu()
            .mit(Umbauschritt::Entfernen(innen("c.txt")))
            .mit(neu_mit(
                "gross.bin",
                Neuart::Datei(Box::new(Bremse {
                    angehalten: angehalten_sender,
                    weiter: weiter_empfaenger,
                    groesse: 64 << 20,
                })),
            ));
        let lauf = umbau::starten(zugang(&archiv), plan, Konfliktregel::Fragen);
        angehalten
            .recv_timeout(GEDULD)
            .expect("die Quelle haelt an");
        lauf.abbrechen();
        weiter.send(()).expect("weiter");
        let bericht = loop {
            if let Meldung::Fertig(bericht) =
                lauf.meldungen().recv_timeout(GEDULD).expect("Meldung")
            {
                break bericht;
            }
        };
        assert_eq!(bericht.abschluss, Abschluss::Abgebrochen, "{endung}");
        assert_eq!(bericht.eintraege, 0, "{endung}");
        assert_eq!(
            gruende(&bericht, &archiv),
            [text(Text::ArchivUnveraendertNachAbbruch)],
            "{endung}"
        );
        assert_eq!(
            fs::read(&archiv).expect("nachher"),
            vorher,
            "{endung}: Abbruch"
        );
        assert!(zwischennamen(ordner.pfad()).is_empty(), "{endung}: Abbruch");

        // Ein Fehler beim Lesen einer Quelle.
        let plan = Umbauplan::neu()
            .mit(Umbauschritt::Umhaengen {
                von: innen("c.txt"),
                nach: innen("c3.txt"),
            })
            .mit(neu_mit("kaputt.bin", Neuart::Datei(Box::new(Kaputt))));
        let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());
        assert_eq!(fahrt.bericht.abschluss, Abschluss::Fertig, "{endung}");
        assert_eq!(fahrt.bericht.eintraege, 0, "{endung}");
        assert_eq!(
            gruende(&fahrt.bericht, Path::new("/probe/kaputt.bin")),
            ["die Quelle ist kaputt"],
            "{endung}"
        );
        assert_eq!(
            gruende(&fahrt.bericht, &archiv),
            [satz(
                Text::ArchivUnveraendert,
                &[("grund", &"die Quelle ist kaputt")]
            )],
            "{endung}"
        );
        assert_eq!(
            fs::read(&archiv).expect("nachher"),
            vorher,
            "{endung}: Fehler"
        );
        assert!(zwischennamen(ordner.pfad()).is_empty(), "{endung}: Fehler");
    }
}

/// Eine Quelle, die beim Oeffnen das Archiv von aussen aendert, wie ein
/// anderes Programm es zwischen Beginn und Ende des Umbaus taete.
struct Stoerer {
    archiv: PathBuf,
}

impl Datenquelle for Stoerer {
    fn oeffnen<'a>(&'a mut self, _pfand: &'a Fernfaden) -> io::Result<Geoeffnet<'a>> {
        let mut datei = OpenOptions::new().append(true).open(&self.archiv)?;
        datei.write_all(b"!")?;
        Ok(Geoeffnet {
            leser: Box::new(io::empty()),
            groesse: 0,
        })
    }
}

/// Ein Archiv, das sich waehrend des Umbaus von aussen aendert, wird nicht
/// ersetzt: die fremde Aenderung bleibt, und der Bericht sagt es.
#[test]
fn ein_von_aussen_veraendertes_archiv_wird_nicht_ersetzt() {
    for (endung, packen) in formen() {
        let ordner = Pruefordner::neu(&format!("umbau-aussen-{endung}"));
        let baum = grundbaum(&ordner);
        let archiv = ordner.unter(&format!("archiv.{endung}"));
        packen(&baum, &archiv);
        let mut erwartet = fs::read(&archiv).expect("vorher");
        erwartet.push(b'!');

        let plan = Umbauplan::neu().mit(neu_mit(
            "stoerung.txt",
            Neuart::Datei(Box::new(Stoerer {
                archiv: archiv.clone(),
            })),
        ));
        let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());
        assert_eq!(
            gruende(&fahrt.bericht, &archiv),
            [unveraendert_weil(Text::ArchivVonAussenVeraendert)],
            "{endung}"
        );
        assert_eq!(fs::read(&archiv).expect("nachher"), erwartet, "{endung}");
        assert!(zwischennamen(ordner.pfad()).is_empty(), "{endung}");
    }
}

/// Eine Verknuepfung auf ein Archiv bleibt eine Verknuepfung; geschrieben
/// wird an ihrem Ziel.
#[test]
fn eine_verknuepfung_auf_ein_archiv_bleibt_eine_verknuepfung() {
    for (endung, packen) in formen() {
        let ordner = Pruefordner::neu(&format!("umbau-verweis-{endung}"));
        let baum = grundbaum(&ordner);
        let echt = ordner.unter(&format!("echt.{endung}"));
        packen(&baum, &echt);
        let verweis = ordner.verknuepfung(&format!("verweis.{endung}"), format!("echt.{endung}"));

        let plan = Umbauplan::neu().mit(Umbauschritt::Entfernen(innen("c.txt")));
        let fahrt = fahren(&verweis, plan, Konfliktregel::Fragen, Vec::new());
        assert_eq!(fahrt.bericht.abschluss, Abschluss::Fertig, "{endung}");
        assert!(fahrt.bericht.uebersprungen.is_empty(), "{endung}");

        assert!(
            fs::symlink_metadata(&verweis)
                .expect("Verweis")
                .file_type()
                .is_symlink(),
            "{endung}: der Verweis ist keine Verknuepfung mehr"
        );
        assert!(
            verzeichnis(&echt).eintrag(&innen("c.txt")).is_none(),
            "{endung}"
        );
        assert!(zwischennamen(ordner.pfad()).is_empty(), "{endung}");
    }
}

/// Baut ein Archiv mit genau einer Datei `d/e/f.txt` und keinem
/// Ordnereintrag.
fn ohne_ordnereintraege(archiv: &Path, endung: &str) {
    match endung {
        "zip" => {
            let mut schreiber = ZipWriter::new(File::create(archiv).expect("Zip"));
            schreiber
                .start_file("d/e/f.txt", FullFileOptions::default())
                .expect("Eintrag");
            schreiber.write_all(b"f").expect("Inhalt");
            schreiber.finish().expect("Zip abschliessen");
        }
        _ => {
            let mut daten = Vec::new();
            {
                let mut bauer = tar::Builder::new(&mut daten);
                let mut kopf = tar::Header::new_ustar();
                kopf.set_size(1);
                kopf.set_mode(0o644);
                bauer
                    .append_data(&mut kopf, "d/e/f.txt", &b"f"[..])
                    .expect("Eintrag");
                bauer.finish().expect("tar abschliessen");
            }
            if endung == "tgz" {
                daten = gzip_glied(&daten);
            }
            fs::write(archiv, daten).expect("Archiv schreiben");
        }
    }
}

/// Faellt der letzte Eintrag eines erschlossenen Ordners weg, bleibt der
/// Ordner stehen, wie er es in einem Dateisystem taete.
#[test]
fn ein_erschlossener_ordner_bleibt_nach_dem_entfernen_seines_letzten_eintrags_stehen() {
    for (endung, _) in formen() {
        let ordner = Pruefordner::neu(&format!("umbau-erschlossen-{endung}"));
        let archiv = ordner.unter(&format!("archiv.{endung}"));
        ohne_ordnereintraege(&archiv, endung);
        let vorher = verzeichnis(&archiv);
        assert!(
            vorher
                .eintrag(&innen("d/e"))
                .expect("d/e")
                .ist_erschlossen()
        );

        let plan = Umbauplan::neu().mit(Umbauschritt::Entfernen(innen("d/e/f.txt")));
        let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());
        assert_eq!(fahrt.bericht.abschluss, Abschluss::Fertig, "{endung}");

        let nachher = verzeichnis(&archiv);
        assert_eq!(pfade(&nachher), ["d", "d/e"], "{endung}");
        let e = nachher.eintrag(&innen("d/e")).expect("d/e");
        assert_eq!(e.typ, Typ::Ordner, "{endung}");
        assert!(
            !e.ist_erschlossen(),
            "{endung}: d/e traegt jetzt einen eigenen Eintrag"
        );
        assert_eq!(
            nachher.kinder(&innen("d/e")).expect("Ordner").count(),
            0,
            "{endung}"
        );
        system_liest(&archiv, endung);
    }
}

/// Ein verdeckter Eintrag faellt beim Umbau weg und wird gezaehlt, ein
/// Eintrag mit unsicherem Namen bleibt Byte fuer Byte (Modulkopf von
/// `archiv::umbau`, „Was im Fenster nicht steht“): sonst kaeme nach dem
/// Entfernen von `dup.txt` dessen frueherer Eintrag wieder hervor.
#[test]
fn ein_verdeckter_eintrag_faellt_weg_und_ein_unsicherer_bleibt() {
    for endung in ["tar", "tgz"] {
        let ordner = Pruefordner::neu(&format!("umbau-verdeckt-{endung}"));
        let archiv = ordner.unter(&format!("archiv.{endung}"));
        let mut daten = Vec::new();
        {
            let mut bauer = tar::Builder::new(&mut daten);
            for (name, inhalt) in [
                ("dup.txt", &b"frueher"[..]),
                ("x.txt", &b"x"[..]),
                ("../boese", &b"boese"[..]),
                ("dup.txt", &b"spaeter"[..]),
            ] {
                let mut kopf = tar::Header::new_ustar();
                kopf.as_old_mut().name[..name.len()].copy_from_slice(name.as_bytes());
                kopf.set_size(inhalt.len() as u64);
                kopf.set_mode(0o644);
                kopf.set_cksum();
                bauer.append(&kopf, inhalt).expect("Eintrag");
            }
            bauer.finish().expect("tar abschliessen");
        }
        if endung == "tgz" {
            daten = gzip_glied(&daten);
        }
        fs::write(&archiv, daten).expect("Archiv schreiben");
        let vorher = verzeichnis(&archiv);
        assert_eq!(
            vorher.ausgelassen(),
            2,
            "{endung}: der fruehere dup.txt und ../boese"
        );
        let boese_vorher = tar_bloecke(&archiv).remove("../boese").expect("../boese");

        let plan = Umbauplan::neu().mit(Umbauschritt::Entfernen(innen("dup.txt")));
        let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());
        assert_eq!(fahrt.bericht.abschluss, Abschluss::Fertig, "{endung}");
        assert_eq!(
            gruende(&fahrt.bericht, &archiv),
            [anzahl(Zahlwort::ArchivVerdeckteWeggefallen, 1, &[])],
            "{endung}"
        );

        let nachher = verzeichnis(&archiv);
        assert_eq!(pfade(&nachher), ["x.txt"], "{endung}");
        assert_eq!(nachher.ausgelassen(), 1, "{endung}: allein ../boese");
        assert_eq!(
            tar_bloecke(&archiv).remove("../boese"),
            Some(boese_vorher),
            "{endung}"
        );
    }
}

/// Der Umbau behaelt die Rechte der Archivdatei und den Kommentar eines Zip.
#[test]
fn der_umbau_behaelt_rechte_der_archivdatei_und_den_zipkommentar() {
    for (endung, packen) in formen() {
        let ordner = Pruefordner::neu(&format!("umbau-rechte-{endung}"));
        let baum = grundbaum(&ordner);
        let archiv = ordner.unter(&format!("archiv.{endung}"));
        packen(&baum, &archiv);
        let kommentar = "ein Kommentar über das Archiv".as_bytes();
        if endung == "zip" {
            // Den Kommentar schreibt die Kiste, damit die Probe nicht selbst am
            // Ende des Archivs rechnet.
            let ohne = ordner.unter("ohne.zip");
            fs::rename(&archiv, &ohne).expect("beiseite");
            let mut quelle = zip::ZipArchive::new(File::open(&ohne).expect("Zip")).expect("lesbar");
            let mut schreiber = ZipWriter::new(File::create(&archiv).expect("Zip"));
            schreiber
                .set_raw_comment(kommentar.into())
                .expect("Kommentar");
            for stelle in 0..quelle.len() {
                schreiber
                    .raw_copy_file(quelle.by_index_raw(stelle).expect("Eintrag"))
                    .expect("Kopie");
            }
            schreiber.finish().expect("Zip abschliessen");
        }
        fs::set_permissions(&archiv, fs::Permissions::from_mode(0o640)).expect("Rechte");

        let plan = Umbauplan::neu().mit(Umbauschritt::Entfernen(innen("a/x.txt")));
        let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());
        assert_eq!(fahrt.bericht.abschluss, Abschluss::Fertig, "{endung}");
        assert!(fahrt.bericht.uebersprungen.is_empty(), "{endung}");

        assert_eq!(
            fs::metadata(&archiv).expect("Angaben").permissions().mode() & 0o7777,
            0o640,
            "{endung}"
        );
        if endung == "zip" {
            let kiste = zip::ZipArchive::new(File::open(&archiv).expect("Zip")).expect("lesbar");
            assert_eq!(kiste.comment(), kommentar);
        }
        assert!(
            verzeichnis(&archiv).eintrag(&innen("a/x.txt")).is_none(),
            "{endung}"
        );
    }
}

/// Ein Archiv mit verschluesseltem Eintrag wird nicht umgebaut.
#[test]
fn ein_archiv_mit_verschluesseltem_eintrag_wird_nicht_umgebaut() {
    let ordner = Pruefordner::neu("umbau-geheim");
    ordner.datei("geheim.txt", b"streng geheim");
    ordner.datei("offen.txt", b"offen");
    for argumente in [
        &["-q", "-P", "kennwort", "geheim.zip", "geheim.txt"][..],
        &["-q", "geheim.zip", "offen.txt"][..],
    ] {
        let stand = Command::new("/usr/bin/zip")
            .current_dir(ordner.pfad())
            .args(argumente)
            .status()
            .expect("/usr/bin/zip laesst sich starten");
        assert!(stand.success());
    }
    let archiv = ordner.unter("geheim.zip");
    let vorher = fs::read(&archiv).expect("vorher");

    let plan = Umbauplan::neu().mit(Umbauschritt::Entfernen(innen("offen.txt")));
    let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());
    assert_eq!(
        gruende(&fahrt.bericht, &archiv),
        [unveraendert_weil(Text::ArchivVerschluesseltNurLesbar)]
    );
    assert_eq!(fahrt.fragen, 0);
    assert_eq!(fs::read(&archiv).expect("nachher"), vorher);
    assert!(zwischennamen(ordner.pfad()).is_empty());
}

/// Eine gz-Datei kennt keinen Umbau und bleibt, wie sie ist.
#[test]
fn eine_gz_datei_wird_nicht_umgebaut() {
    let ordner = Pruefordner::neu("umbau-gz");
    let archiv = ordner.datei("bericht.txt.gz", gzip_glied(b"ein Bericht"));
    let vorher = fs::read(&archiv).expect("vorher");

    let plan = Umbauplan::neu().mit(Umbauschritt::Entfernen(innen("bericht.txt")));
    let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());
    assert_eq!(
        gruende(&fahrt.bericht, &archiv),
        [unveraendert_weil(Text::ArchivNurLesbar)]
    );
    assert_eq!(fs::read(&archiv).expect("nachher"), vorher);
}

/// Ein Umbau ueber die Grenze von 65 535 Eintraegen schreibt die ZIP64-Form
/// und bleibt fuer KRK und das System lesbar.
#[test]
fn ein_umbau_ueber_die_zip64_grenze_bleibt_lesbar() {
    let ordner = Pruefordner::neu("umbau-zip64");
    let archiv = ordner.unter("viele.zip");
    let zahl = 65_535;
    {
        let mut schreiber = ZipWriter::new(std::io::BufWriter::new(
            File::create(&archiv).expect("Zip anlegen"),
        ));
        let wahl = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        for nummer in 0..zahl {
            schreiber
                .start_file(format!("d/{nummer}"), wahl)
                .expect("Eintrag");
        }
        schreiber.finish().expect("Zip abschliessen");
    }

    let plan = Umbauplan::neu()
        .mit(neu_mit("eins.txt", Neuart::Datei(Box::new(Leer))))
        .mit(neu_mit("zwei.txt", Neuart::Datei(Box::new(Leer))));
    let fahrt = fahren(&archiv, plan, Konfliktregel::Fragen, Vec::new());
    assert_eq!(fahrt.bericht.abschluss, Abschluss::Fertig);
    assert!(
        fahrt.bericht.uebersprungen.is_empty(),
        "{:?}",
        fahrt.bericht.uebersprungen
    );

    let nachher = verzeichnis(&archiv);
    assert_eq!(
        nachher.eintraege().len(),
        zahl + 3,
        "die Dateien, `d` und zwei neue"
    );
    assert_eq!(nachher.kinder(&innen("d")).expect("d").count(), zahl);
    system_liest(&archiv, "zip");
}

/// Jede Konfliktfrage kommt vor dem ersten geschriebenen Byte, und jede
/// Antwort wirkt: Ueberschreiben, Umbenennen in einen freien Namen,
/// Ueberspringen, und Ordner auf Ordner fuehrt ohne Frage zusammen.
#[test]
fn die_konfliktfragen_kommen_vor_dem_ersten_byte_und_jede_antwort_wirkt() {
    for (endung, packen) in formen() {
        let ordner = Pruefordner::neu(&format!("umbau-konflikt-{endung}"));
        ordner.ordner("baum");
        ordner.ordner("baum/o");
        ordner.datei("baum/a.txt", "alt-a");
        ordner.datei("baum/b.txt", "alt-b");
        ordner.datei("baum/o/k.txt", "alt-k");
        let archiv = ordner.unter(&format!("archiv.{endung}"));
        packen(&ordner.unter("baum"), &archiv);
        ordner.ordner("quelle");
        ordner.ordner("quelle/o");
        ordner.datei("quelle/a.txt", "neu-a");
        ordner.datei("quelle/b.txt", "neu-b");
        ordner.datei("quelle/o/k.txt", "neu-k");
        ordner.datei("quelle/o/m.txt", "neu-m");

        let quelle = ordner.unter("quelle");
        let plan = plan_bauen(move |pfand| {
            ["a.txt", "b.txt", "o"]
                .into_iter()
                .fold(Umbauplan::neu(), |plan, name| {
                    plan.mit(Umbauschritt::Hinzufuegen {
                        ordner: Innenpfad::wurzel(),
                        eintrag: lokal(pfand, &quelle.join(name)),
                    })
                })
        });
        let fahrt = fahren(
            &archiv,
            plan,
            Konfliktregel::Fragen,
            vec![
                Konfliktentscheid::einmal(Konfliktantwort::Ueberschreiben),
                Konfliktentscheid::einmal(Konfliktantwort::UmbenennenIn("b2.txt".to_owned())),
                Konfliktentscheid::einmal(Konfliktantwort::Ueberspringen),
            ],
        );
        assert_eq!(fahrt.fragen, 3, "{endung}: a.txt, b.txt und o/k.txt");
        assert_eq!(
            fahrt.fortschritt_vor_letzter_frage, 0,
            "{endung}: vor der letzten Frage ist kein Byte geschrieben"
        );
        assert_eq!(fahrt.bericht.abschluss, Abschluss::Fertig, "{endung}");
        assert_eq!(fahrt.bericht.eintraege, 3, "{endung}");

        let aus = ordner.unter("aus");
        system_auspacken(&archiv, &aus);
        for (pfad, inhalt) in [
            ("a.txt", "neu-a"),
            ("b.txt", "alt-b"),
            ("b2.txt", "neu-b"),
            ("o/k.txt", "alt-k"),
            ("o/m.txt", "neu-m"),
        ] {
            assert_eq!(
                fs::read_to_string(aus.join(pfad)).expect("ausgepackt"),
                inhalt,
                "{endung}: {pfad}"
            );
        }
        assert_eq!(
            pfade(&verzeichnis(&archiv)),
            ["a.txt", "b.txt", "b2.txt", "o", "o/k.txt", "o/m.txt"],
            "{endung}"
        );
    }
}

/// `archiv::ersetzen` ist der einzige Schreiber an einer Archivdatei: in den
/// Codezeilen unter `krk-core/src/archiv/` oeffnet allein `ersetzen.rs` eine
/// Datei zum Schreiben, benennt um oder entfernt, und `ersetzen(` hat genau
/// einen Rufer, den Umbau.
///
/// # Was diese Probe nicht sieht
///
/// Einen Schreiber ueber einen Helfer ausserhalb von `archiv/`, etwa ein
/// Aufruf von `ablage::atomar` mit dem Pfad eines Archivs; einen Aufruf
/// unter einem anderen Namen (`use … as anders;`); und ein Schreiben ueber
/// einen Deskriptor, den ein Rufer von aussen hereinreicht. Das Schreiben in
/// die Datei unter dem Zwischennamen, die `ersetzen` dem Umbau reicht, sieht
/// sie mit Absicht nicht: jene Datei ist das neue Archiv und nicht das alte.
#[test]
fn archiv_ersetzen_ist_der_einzige_schreiber_an_einer_archivdatei() {
    let nadeln = [
        concat!("Open", "Options"),
        concat!("File::", "create"),
        concat!("fs::", "write"),
        concat!("fs::", "copy"),
        concat!("ren", "ame("),
        concat!("remove_", "file"),
        concat!("remove_", "dir"),
        concat!("hard_", "link"),
    ];
    let archivdateien: Vec<(String, String)> = quelldateien()
        .into_iter()
        .filter(|(name, _)| name.starts_with("krk-core/src/archiv/"))
        .collect();
    assert!(
        archivdateien
            .iter()
            .any(|(name, _)| name == "krk-core/src/archiv/ersetzen.rs"),
        "ersetzen.rs steht nicht mehr im Baum"
    );
    let mut schreiber: Vec<String> = Vec::new();
    for (name, inhalt) in &archivdateien {
        let schreibt = betriebscode(inhalt)
            .into_iter()
            .filter(|zeile| !zeile.trim_start().starts_with("//"))
            .any(|zeile| nadeln.iter().any(|nadel| zeile.contains(nadel)));
        if schreibt {
            schreiber.push(name.clone());
        }
    }
    assert_eq!(
        schreiber,
        ["krk-core/src/archiv/ersetzen.rs"],
        "eine weitere Datei unter archiv/ schreibt an einer Datei"
    );

    // Gezaehlt wird unter `archiv/`, wo `ersetzen` sichtbar ist; ausserhalb
    // ist es `pub(crate)` und muesste ueber den Modulpfad gerufen werden.
    let mut rufer: Vec<(String, usize)> = archivdateien
        .iter()
        .map(|(name, inhalt)| {
            let code = betriebscode(inhalt).join("\n");
            (name.clone(), aufrufstellen(&code, "ersetzen"))
        })
        .filter(|(_, zahl)| *zahl > 0)
        .collect();
    rufer.sort();
    assert_eq!(
        rufer,
        [("krk-core/src/archiv/umbau.rs".to_owned(), 1)],
        "ersetzen hat einen anderen Rufer als den Umbau"
    );
    let modulpfad = concat!("ersetzen::", "ersetzen(");
    let fremde: Vec<String> = quelldateien()
        .into_iter()
        .filter(|(name, inhalt)| {
            !name.starts_with("krk-core/src/archiv/")
                && betriebscode(inhalt)
                    .iter()
                    .any(|zeile| zeile.contains(modulpfad))
        })
        .map(|(name, _)| name)
        .collect();
    assert!(
        fremde.is_empty(),
        "ersetzen wird ausserhalb von archiv/ gerufen: {fremde:?}"
    );
}
