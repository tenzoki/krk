//! Abnahme des SFTP-Gespraechs im Kern (Schritt 1 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`), von
//! Adresse, `ssh`-Aufruf, Vorpruefung des Rechnerschluessels und Zugang
//! (Schritt 2) und des fernen Lesevorgangs (Schritt 3).
//!
//! **Kein Netz, kein `ssh`.** Jede Probe, die spricht, startet
//! `/usr/libexec/sftp-server` unmittelbar als Kindprozess und fuehrt das
//! Gespraech ueber dessen Pipes, im [`Pruefordner`] dieser Kiste. Im Betrieb
//! steht `ssh -s … sftp` an derselben Stelle und reicht die Bytes desselben
//! Servers durch; der Code des Gespraechs ist derselbe. Was diese Proben nicht
//! pruefen, ist ein fremder Server: ob er `RENAME` nach dem Entwurf
//! beantwortet, sagt allein der Test gegen ihn.
//!
//! Jede sprechende Probe laeuft in einem Faden aus `fern::faden`, denn jede
//! Anfrage verlangt das Pfand, das nur dort entsteht; [`im_fernfaden`] reicht
//! einen Absturz darin an die Probe weiter, damit `cargo test` ihn sieht.

mod gemeinsam;

use std::collections::BTreeSet;
use std::fs;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use gemeinsam::{Pruefordner, quelldateien};
use krk_core::fern::kanal::HILFE_VARIABLE;
use krk_core::fern::paket::{
    Anfrage, Antwort, Attribute, Name, PAKETGRENZE, Paket, Paketfehler, oeffnen, paket_lesen,
    status,
};
use krk_core::fern::rechnerschluessel::{
    Fingerabdruck, Sshkonfiguration, anhaengen, gescannt_aus, in_datei_bekannt,
    konfiguration_lesen, suchname,
};
use krk_core::fern::{
    Adressfehler, Anmeldung, BLOCK, Fernfaden, Fernfehler, Fernpfad, Fernziel, Fernzugang,
    Gespraech, Herkunft, Kanalplan, OFFENE_LESUNGEN, Serverkennung, Startpfad, Trennungsgrund,
    Zugangsfehler, Zugangsregister, Zustand, faden, fernziel, ist_einzelner_name, ssh_plan,
};
use krk_core::heimordner::verbindung::{Verbindungsziel, Zielfehler, verbindungsziel};
use krk_core::ort::FernerOrt;
use krk_core::verzeichnis::leser::{Fernname, fern_eintrag, namen_aufnehmen};
use krk_core::verzeichnis::{Abschluss, Eintrag, Lesevorgang, Meldung, STAPELGROESSE, Typ};

/// Der Server des Systems, ohne `ssh` davor.
const SFTP_SERVER: &str = "/usr/libexec/sftp-server";

/// Startet den Server des Systems mit drei Pipes.
fn server_starten() -> Child {
    Command::new(SFTP_SERVER)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("sftp-server laesst sich nicht starten")
}

/// Fuehrt `arbeit` in einem Faden mit Pfand aus und reicht ihr Ergebnis oder
/// ihren Absturz zurueck.
fn im_fernfaden<T: Send + 'static>(arbeit: impl FnOnce(&Fernfaden) -> T + Send + 'static) -> T {
    let lauf = faden("krk-probe-fern".to_owned(), arbeit).expect("kein Faden fuer die Probe");
    match lauf.join() {
        Ok(wert) => wert,
        Err(absturz) => std::panic::resume_unwind(absturz),
    }
}

/// Ein offenes Gespraech mit dem Server des Systems.
fn gespraech(pfand: &Fernfaden) -> Gespraech {
    Gespraech::eroeffnen(pfand, server_starten(), "probe").expect("das Gespraech kommt nicht auf")
}

fn text(pfad: &Path) -> String {
    pfad.to_str().expect("Pruefpfade sind UTF-8").to_owned()
}

// ----------------------------------------------------------------------
// Pakete
// ----------------------------------------------------------------------

/// Die Stelle einer Anfrage in der Deckungsliste. Vollstaendig und ohne
/// Auffangzweig: eine neue Variante haelt den Bau hier an, bis die Probe sie
/// mitnimmt.
fn anfragestelle(anfrage: &Anfrage) -> usize {
    match anfrage {
        Anfrage::Open { .. } => 0,
        Anfrage::Close { .. } => 1,
        Anfrage::Read { .. } => 2,
        Anfrage::Write { .. } => 3,
        Anfrage::Lstat { .. } => 4,
        Anfrage::Setstat { .. } => 5,
        Anfrage::Opendir { .. } => 6,
        Anfrage::Readdir { .. } => 7,
        Anfrage::Remove { .. } => 8,
        Anfrage::Mkdir { .. } => 9,
        Anfrage::Rmdir { .. } => 10,
        Anfrage::Realpath { .. } => 11,
        Anfrage::Stat { .. } => 12,
        Anfrage::Rename { .. } => 13,
        Anfrage::Readlink { .. } => 14,
        Anfrage::Symlink { .. } => 15,
    }
}
const ANFRAGEARTEN: usize = 16;

/// Dasselbe fuer die Antworten.
fn antwortstelle(antwort: &Antwort) -> usize {
    match antwort {
        Antwort::Status { .. } => 0,
        Antwort::Handle(_) => 1,
        Antwort::Data(_) => 2,
        Antwort::Name(_) => 3,
        Antwort::Attrs(_) => 4,
    }
}
const ANTWORTARTEN: usize = 5;

fn volle_attribute() -> Attribute {
    Attribute {
        groesse: Some(0x0123_4567_89ab_cdef),
        besitz: Some((501, 20)),
        rechte: Some(0o100_644),
        zeiten: Some((1_760_000_000, 1_760_000_123)),
        erweitert: vec![
            (b"name@beispiel".to_vec(), b"wert".to_vec()),
            (b"zweiter@beispiel".to_vec(), vec![0, 255, 7]),
        ],
    }
}

/// Jede Anfrage und jede Antwort, dazu `INIT` und `VERSION`, kommt durch
/// Kodierung, [`paket_lesen`] und Zerlegung unveraendert zurueck, einzeln und
/// als Strom hintereinander.
#[test]
fn jedes_paket_uebersteht_den_rundlauf() {
    let h = b"handle-7".to_vec();
    let p = b"/ordner/datei mit leerzeichen".to_vec();
    let anfragen = vec![
        Anfrage::Open {
            pfad: p.clone(),
            kennzeichen: oeffnen::WRITE | oeffnen::CREAT | oeffnen::EXCL,
            attribute: volle_attribute(),
        },
        Anfrage::Close { handle: h.clone() },
        Anfrage::Read {
            handle: h.clone(),
            versatz: u64::MAX - 1,
            laenge: BLOCK,
        },
        Anfrage::Write {
            handle: h.clone(),
            versatz: 3,
            daten: (0..=255).collect(),
        },
        Anfrage::Lstat { pfad: p.clone() },
        Anfrage::Setstat {
            pfad: p.clone(),
            attribute: Attribute {
                rechte: Some(0o755),
                ..Attribute::default()
            },
        },
        Anfrage::Opendir { pfad: p.clone() },
        Anfrage::Readdir { handle: h.clone() },
        Anfrage::Remove { pfad: p.clone() },
        Anfrage::Mkdir {
            pfad: p.clone(),
            attribute: Attribute::default(),
        },
        Anfrage::Rmdir { pfad: p.clone() },
        Anfrage::Realpath {
            pfad: b".".to_vec(),
        },
        Anfrage::Stat { pfad: p.clone() },
        Anfrage::Rename {
            alt: p.clone(),
            neu: b"/neu".to_vec(),
        },
        Anfrage::Readlink { pfad: p.clone() },
        Anfrage::Symlink {
            erste: b"ziel".to_vec(),
            zweite: p.clone(),
        },
    ];
    let antworten = vec![
        Antwort::Status {
            code: status::FAILURE,
            meldung: b"Failure".to_vec(),
            sprache: b"en".to_vec(),
        },
        Antwort::Handle(h.clone()),
        Antwort::Data(vec![0xaa; 1000]),
        Antwort::Name(vec![
            Name {
                name: b".".to_vec(),
                lang: b"drwxr-xr-x ...".to_vec(),
                attribute: volle_attribute(),
            },
            Name {
                name: vec![0xff, 0xfe],
                lang: Vec::new(),
                attribute: Attribute::default(),
            },
        ]),
        Antwort::Attrs(volle_attribute()),
    ];

    let mut gedeckt_anfragen = [false; ANFRAGEARTEN];
    let mut gedeckt_antworten = [false; ANTWORTARTEN];
    let mut pakete = vec![
        Paket::Init {
            fassung: 3,
            erweiterungen: Vec::new(),
        },
        Paket::Version {
            fassung: 3,
            erweiterungen: vec![
                (b"posix-rename@openssh.com".to_vec(), b"1".to_vec()),
                (b"limits@openssh.com".to_vec(), b"1".to_vec()),
            ],
        },
    ];
    for (stelle, anfrage) in anfragen.into_iter().enumerate() {
        gedeckt_anfragen[anfragestelle(&anfrage)] = true;
        pakete.push(Paket::Anfrage {
            kennung: u32::try_from(stelle).expect("klein") * 1000 + 1,
            anfrage,
        });
    }
    for (stelle, antwort) in antworten.into_iter().enumerate() {
        gedeckt_antworten[antwortstelle(&antwort)] = true;
        pakete.push(Paket::Antwort {
            kennung: u32::MAX - u32::try_from(stelle).expect("klein"),
            antwort,
        });
    }
    assert!(
        gedeckt_anfragen.iter().all(|gedeckt| *gedeckt),
        "nicht jede Anfrageart steht in der Probe: {gedeckt_anfragen:?}"
    );
    assert!(
        gedeckt_antworten.iter().all(|gedeckt| *gedeckt),
        "nicht jede Antwortart steht in der Probe: {gedeckt_antworten:?}"
    );

    let mut strom = Vec::new();
    for paket in &pakete {
        let bytes = paket.kodieren();
        let laenge = u32::from_be_bytes(bytes[..4].try_into().expect("vier Bytes"));
        assert_eq!(
            laenge as usize,
            bytes.len() - 4,
            "Laengenangabe von {paket:?}"
        );
        let mut einzeln = &bytes[..];
        let rumpf = paket_lesen(&mut einzeln)
            .expect("lesbar")
            .expect("ein Paket");
        assert_eq!(
            &Paket::zerlegen(&rumpf).expect("zerlegbar"),
            paket,
            "Rundlauf"
        );
        strom.extend_from_slice(&bytes);
    }
    let mut leser = &strom[..];
    for paket in &pakete {
        let rumpf = paket_lesen(&mut leser).expect("lesbar").expect("ein Paket");
        assert_eq!(&Paket::zerlegen(&rumpf).expect("zerlegbar"), paket);
    }
    assert!(
        paket_lesen(&mut leser).expect("sauberes Ende").is_none(),
        "nach dem letzten Paket endet der Strom an einer Paketgrenze"
    );
}

/// Eine Laengenangabe ueber der Grenze wird abgewiesen, bevor ein Byte des
/// Rumpfes gelesen ist: der Strom traegt hier nur ein Byte nach der Laenge,
/// und das Ergebnis ist trotzdem `ZuLang` und kein abgerissener Strom. Genau
/// an der Grenze wird angenommen.
#[test]
fn ein_paket_ueber_der_grenze_wird_abgewiesen() {
    for laenge in [PAKETGRENZE + 1, u32::MAX] {
        let mut bytes = laenge.to_be_bytes().to_vec();
        bytes.push(103);
        match paket_lesen(&mut &bytes[..]) {
            Err(Paketfehler::ZuLang(gemeldet)) => assert_eq!(gemeldet, laenge),
            anderes => panic!("{laenge} Bytes muessen abgewiesen werden, Ergebnis {anderes:?}"),
        }
    }

    let null = 0u32.to_be_bytes();
    assert!(matches!(
        paket_lesen(&mut &null[..]),
        Err(Paketfehler::Leer)
    ));

    // Typbyte, Kennung und Laenge der Daten sind neun Bytes; der Rest fuellt
    // bis genau an die Grenze.
    let an_der_grenze = Paket::Antwort {
        kennung: 1,
        antwort: Antwort::Data(vec![1; PAKETGRENZE as usize - 9]),
    };
    let bytes = an_der_grenze.kodieren();
    assert_eq!(bytes.len(), PAKETGRENZE as usize + 4);
    let rumpf = paket_lesen(&mut &bytes[..])
        .expect("an der Grenze lesbar")
        .expect("ein Paket");
    assert_eq!(Paket::zerlegen(&rumpf).expect("zerlegbar"), an_der_grenze);

    let abgerissen = &bytes[..bytes.len() - 1];
    assert!(matches!(
        paket_lesen(&mut &abgerissen[..]),
        Err(Paketfehler::Abgerissen)
    ));
}

// ----------------------------------------------------------------------
// Das Gespraech mit dem Server des Systems
// ----------------------------------------------------------------------

/// Ein Ordner mit 10 000 Dateien kommt vollstaendig an: 10 002 Namen samt
/// `.` und `..`, wie in der Analyse
/// `261016-1746-sftp-verbindung-als-dateisystem-machbarkeit.md` gemessen.
#[test]
fn das_gespraech_liest_einen_ordner_mit_zehntausend_eintraegen() {
    let ordner = Pruefordner::neu("fern-zehntausend");
    let erwartet: BTreeSet<String> = (0..10_000).map(|i| format!("datei-{i:05}")).collect();
    for name in &erwartet {
        fs::File::create(ordner.unter(name)).expect("Datei laesst sich nicht anlegen");
    }
    let pfad = text(ordner.pfad());

    let (namen, runden) = im_fernfaden(move |pfand| {
        let g = gespraech(pfand);
        let handle = g.opendir(pfand, &pfad).expect("OPENDIR");
        let mut namen = Vec::new();
        let mut runden = 0;
        while let Some(stapel) = g.readdir(pfand, &handle).expect("READDIR") {
            runden += 1;
            namen.extend(
                stapel
                    .into_iter()
                    .map(|name| String::from_utf8(name.name).expect("UTF-8")),
            );
        }
        g.close(pfand, handle).expect("CLOSE");
        (namen, runden)
    });

    assert_eq!(namen.len(), 10_002, "Namen samt . und ..");
    assert!(
        runden > 1,
        "ein Ordner dieser Groesse kommt in mehreren Antworten"
    );
    let gesehen: BTreeSet<String> = namen.into_iter().collect();
    let mut mit_punkten = erwartet;
    mit_punkten.insert(".".to_owned());
    mit_punkten.insert("..".to_owned());
    assert_eq!(gesehen, mit_punkten);
}

/// `RENAME` scheitert am Server des Systems an einem vorhandenen Ziel, fuer
/// eine Datei wie fuer einen Ordner, und laesst beide Seiten unberuehrt; auf
/// einen freien Namen benennt es um. Das belegt Entscheidung 10 des Plans an
/// diesem Server und an keinem anderen.
#[test]
fn rename_weist_ein_vorhandenes_ziel_ab() {
    let ordner = Pruefordner::neu("fern-rename");
    let quelle = ordner.datei("quelle", "alt");
    let ziel = ordner.datei("ziel", "vorhanden");
    let frei = ordner.unter("frei");
    let ordner_a = ordner.ordner("a");
    ordner.datei("a/inhalt", "a");
    let ordner_b = ordner.ordner("b");
    ordner.datei("b/inhalt", "b");
    let (q, z, f, a, b) = (
        text(&quelle),
        text(&ziel),
        text(&frei),
        text(&ordner_a),
        text(&ordner_b),
    );

    let (datei, mappe, umbenannt) = im_fernfaden(move |pfand| {
        let g = gespraech(pfand);
        let datei = g.rename(pfand, &q, &z);
        let mappe = g.rename(pfand, &a, &b);
        let umbenannt = g.rename(pfand, &q, &f);
        (datei, mappe, umbenannt)
    });

    assert!(
        matches!(datei, Err(Fernfehler::Status { .. })),
        "eine Datei auf eine vorhandene: {datei:?}"
    );
    assert!(
        matches!(mappe, Err(Fernfehler::Status { .. })),
        "ein Ordner auf einen vorhandenen: {mappe:?}"
    );
    assert_eq!(fs::read_to_string(&ziel).expect("Ziel steht"), "vorhanden");
    assert_eq!(
        fs::read_to_string(ordner.unter("b/inhalt")).expect("b"),
        "b"
    );
    assert_eq!(
        fs::read_to_string(ordner.unter("a/inhalt")).expect("a"),
        "a"
    );
    assert_eq!(umbenannt, Ok(()));
    assert!(!quelle.exists());
    assert_eq!(fs::read_to_string(&frei).expect("umbenannt"), "alt");
}

/// `OPEN` mit `CREAT|EXCL` scheitert an einem vorhandenen Namen und laesst
/// dessen Inhalt stehen; auf einem freien Namen legt es an, mit den Rechten
/// aus den Attributen.
#[test]
fn ein_exklusives_oeffnen_scheitert_an_einem_vorhandenen_namen() {
    let ordner = Pruefordner::neu("fern-exklusiv");
    let vorhanden = ordner.datei("vorhanden", "bleibt");
    let neu = ordner.unter("neu");
    let (v, n) = (text(&vorhanden), text(&neu));

    let (abgewiesen, angelegt) = im_fernfaden(move |pfand| {
        let g = gespraech(pfand);
        let exklusiv = oeffnen::WRITE | oeffnen::CREAT | oeffnen::EXCL;
        let rechte = Attribute {
            rechte: Some(0o640),
            ..Attribute::default()
        };
        let abgewiesen = g.open(pfand, &v, exklusiv, rechte.clone()).map(|_| ());
        let angelegt = g.open(pfand, &n, exklusiv, rechte).and_then(|handle| {
            g.write(pfand, &handle, 0, b"neu")?;
            g.close(pfand, handle)
        });
        (abgewiesen, angelegt)
    });

    assert!(
        matches!(abgewiesen, Err(Fernfehler::Status { .. })),
        "{abgewiesen:?}"
    );
    assert_eq!(fs::read_to_string(&vorhanden).expect("steht"), "bleibt");
    assert_eq!(angelegt, Ok(()));
    assert_eq!(fs::read_to_string(&neu).expect("angelegt"), "neu");
    use std::os::unix::fs::PermissionsExt;
    let rechte = fs::metadata(&neu).expect("da").permissions().mode() & 0o777;
    assert_eq!(
        rechte & 0o600,
        0o600,
        "mindestens Lesen und Schreiben fuer den Besitzer"
    );
}

/// Eine Datei ueber viele Bloecke kommt mit bis zu sechzehn offenen Lesungen
/// Byte fuer Byte in ihrer Reihenfolge an, auch mit einem kurzen letzten
/// Block.
#[test]
fn sechzehn_offene_lesungen_liefern_die_datei_in_ihrer_reihenfolge() {
    let ordner = Pruefordner::neu("fern-bloecke");
    let laenge = OFFENE_LESUNGEN * BLOCK as usize * 2 + 12_345;
    let inhalt: Vec<u8> = (0..laenge).map(|i| (i * 31 % 251) as u8).collect();
    let datei = ordner.datei("gross", &inhalt);
    let pfad = text(&datei);

    let (gelesen, gezaehlt, bloecke) = im_fernfaden(move |pfand| {
        let g = gespraech(pfand);
        let handle = g
            .open(pfand, &pfad, oeffnen::READ, Attribute::default())
            .expect("OPEN");
        let mut gelesen = Vec::new();
        let mut bloecke = 0;
        let gezaehlt = g
            .lesen_in_bloecken(pfand, &handle, |stueck| {
                bloecke += 1;
                gelesen.extend_from_slice(stueck);
                ControlFlow::Continue(())
            })
            .expect("Lesen");
        g.close(pfand, handle).expect("CLOSE");
        (gelesen, gezaehlt, bloecke)
    });

    assert_eq!(gezaehlt, laenge as u64);
    assert!(
        gelesen == inhalt,
        "die Bytes kommen in ihrer Reihenfolge an"
    );
    assert!(
        bloecke > OFFENE_LESUNGEN,
        "mehr Bloecke als offene Lesungen"
    );
}

/// Ein Abbruch mitten im Lesen laesst die offenen Lesungen fallen, und ihre
/// spaeten Antworten trennen das Gespraech nicht: die naechste Anfrage geht
/// durch.
#[test]
fn ein_abbruch_beim_lesen_laesst_das_gespraech_offen() {
    let ordner = Pruefordner::neu("fern-abbruch");
    let datei = ordner.fuelldatei("gross", OFFENE_LESUNGEN * BLOCK as usize * 4);
    let pfad = text(&datei);

    let (ergebnis, danach, zustand) = im_fernfaden(move |pfand| {
        let g = gespraech(pfand);
        let handle = g
            .open(pfand, &pfad, oeffnen::READ, Attribute::default())
            .expect("OPEN");
        let ergebnis = g.lesen_in_bloecken(pfand, &handle, |_| ControlFlow::Break(()));
        let danach = g.stat(pfand, &pfad).map(|attribute| attribute.groesse);
        g.close(pfand, handle).expect("CLOSE");
        (ergebnis, danach, g.zustand())
    });

    assert_eq!(ergebnis, Err(Fernfehler::Abgebrochen));
    assert_eq!(
        danach,
        Ok(Some((OFFENE_LESUNGEN * BLOCK as usize * 4) as u64))
    );
    assert_eq!(zustand, Zustand::Offen);
}

/// Endet der Server, bekommt jede wartende Anfrage `Getrennt`, und jede
/// spaetere auch.
///
/// Damit die Anfragen wirklich warten, statt schon beantwortet zu sein, oeffnet
/// die erste eine benannte Roehre ohne Schreiber: `sftp-server` haengt darin in
/// `open(2)` und liest die zwei dahinter nicht mehr. Dann beendet die Probe den
/// Server von aussen.
#[test]
fn ein_beendeter_server_beantwortet_jede_wartende_anfrage_mit_getrennt() {
    let ordner = Pruefordner::neu("fern-getrennt");
    let roehre = text(&ordner.roehre("roehre"));
    let wurzel = text(ordner.pfad());

    let (antworten, spaeter, zustand) = im_fernfaden(move |pfand| {
        let g = gespraech(pfand);
        let haengend = g
            .senden(
                pfand,
                Anfrage::Open {
                    pfad: roehre.into_bytes(),
                    kennzeichen: oeffnen::READ,
                    attribute: Attribute::default(),
                },
            )
            .expect("senden");
        let zweite = g
            .senden(
                pfand,
                Anfrage::Stat {
                    pfad: wurzel.clone().into_bytes(),
                },
            )
            .expect("senden");
        let dritte = g
            .senden(
                pfand,
                Anfrage::Realpath {
                    pfad: b".".to_vec(),
                },
            )
            .expect("senden");
        let beendet = Command::new("/bin/kill")
            .arg("-9")
            .arg(g.prozesskennung().to_string())
            .status()
            .expect("kill laesst sich nicht starten");
        assert!(beendet.success(), "kill ist gescheitert");
        let antworten = [
            haengend.warten(pfand),
            zweite.warten(pfand),
            dritte.warten(pfand),
        ];
        let spaeter = g.stat(pfand, &wurzel);
        (antworten, spaeter, g.zustand())
    });

    for antwort in &antworten {
        assert!(
            matches!(antwort, Err(Fernfehler::Getrennt(_))),
            "jede wartende Anfrage: {antwort:?}"
        );
    }
    match spaeter {
        Err(Fernfehler::Getrennt(trennung)) => {
            assert_eq!(trennung.grund, Trennungsgrund::AusgabeEndete);
        }
        anderes => panic!("eine spaetere Anfrage: {anderes:?}"),
    }
    assert!(matches!(zustand, Zustand::Getrennt(_)), "{zustand:?}");
}

/// Ein Kindprozess, der vor `VERSION` endet, laesst die Eroeffnung mit
/// `Getrennt` scheitern, und die Trennung traegt seine Fehlerausgabe und
/// seinen Ausgangscode. So kommt im Betrieb die Begruendung von `ssh` in die
/// Meldung.
#[test]
fn ein_scheiternder_aufbau_nennt_die_fehlerausgabe() {
    let ergebnis = im_fernfaden(|pfand| {
        let kind = Command::new("/bin/sh")
            .arg("-c")
            .arg("echo erste >&2; echo 'Permission denied (password).' >&2; exit 255")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("sh laesst sich nicht starten");
        Gespraech::eroeffnen(pfand, kind, "probe").map(|_| ())
    });
    match ergebnis {
        Err(Fernfehler::Getrennt(trennung)) => {
            assert_eq!(trennung.grund, Trennungsgrund::AusgabeEndete);
            assert_eq!(trennung.ausgang, Some(255));
            assert_eq!(
                trennung.fehlerausgabe,
                vec![
                    "erste".to_owned(),
                    "Permission denied (password).".to_owned()
                ]
            );
        }
        anderes => panic!("die Eroeffnung muss scheitern: {anderes:?}"),
    }
}

/// Ein Enkelprozess, der die Fehlerausgabe offen haelt, haelt die Trennung
/// nicht auf: eine wartende Anfrage bekommt binnen einer Sekunde `Getrennt`,
/// sobald die Ausgabe des Kindes endet (Befund
/// `261017-0004-das-gespraech-meldet-getrennt-erst-nach-dem-join-des-fehlerfadens-und-kann-an-einem-enkelprozess-haengen`).
///
/// Das Kind antwortet auf `INIT` mit `VERSION`, laesst ein `sleep` mit seiner
/// Fehlerausgabe zurueck, liest `INIT` und das erste Byte der Anfrage und
/// endet. Bis zur Behebung wartete die Trennung auf das Ende der
/// Fehlerausgabe, also auf das `sleep`, und die Anfrage blieb fuenf Sekunden
/// ohne Antwort; so haelt im Betrieb ein `ProxyCommand`, der `ssh` ueberlebt,
/// jeden Leser und jeden Vorgang fest.
#[test]
fn ein_enkel_mit_offener_fehlerausgabe_haelt_die_trennung_nicht_auf() {
    // VERSION: Laenge 5, Typ 2, Fassung 3. `INIT` sind neun Bytes, das
    // zehnte gehoert schon der Anfrage.
    const SKRIPT: &str = "printf '\\000\\000\\000\\005\\002\\000\\000\\000\\003'; \
                          sleep 5 >&2 & \
                          head -c 10 >/dev/null; \
                          exit 0";
    let (antwort, dauer) = im_fernfaden(|pfand| {
        let kind = Command::new("/bin/sh")
            .arg("-c")
            .arg(SKRIPT)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("sh laesst sich nicht starten");
        let g = Gespraech::eroeffnen(pfand, kind, "probe").expect("VERSION kommt an");
        let beginn = Instant::now();
        let antwort = g.realpath(pfand, "/");
        (antwort, beginn.elapsed())
    });
    assert!(
        matches!(antwort, Err(Fernfehler::Getrennt(_))),
        "die wartende Anfrage: {antwort:?}"
    );
    assert!(
        dauer < Duration::from_secs(1),
        "die Trennung kam erst nach {dauer:?}"
    );
}

/// `VERSION` nennt Fassung 3 und die Erweiterungen von OpenSSH; `REALPATH`,
/// `MKDIR`, `SETSTAT`, `LSTAT`, `SYMLINK` in der Folge von OpenSSH,
/// `READLINK`, `REMOVE` und `RMDIR` wirken am Server des Systems wie
/// angefragt.
#[test]
fn die_einzelnen_anfragen_wirken_am_server_des_systems() {
    let ordner = Pruefordner::neu("fern-einzeln");
    let wurzel = text(ordner.pfad());
    let w = wurzel.clone();

    let befund = im_fernfaden(move |pfand| {
        let g = gespraech(pfand);
        assert_eq!(g.fassung(), 3);
        assert!(g.spricht_openssh(), "der Server des Systems ist OpenSSH");

        let aufgeloest = g.realpath(pfand, &w).expect("REALPATH");
        let mappe = format!("{w}/mappe");
        g.mkdir(pfand, &mappe, Attribute::default()).expect("MKDIR");
        assert!(
            g.mkdir(pfand, &mappe, Attribute::default()).is_err(),
            "MKDIR auf einen vorhandenen Namen scheitert"
        );
        let datei = format!("{mappe}/datei");
        let handle = g
            .open(
                pfand,
                &datei,
                oeffnen::WRITE | oeffnen::CREAT | oeffnen::EXCL,
                Attribute::default(),
            )
            .expect("OPEN");
        g.write(pfand, &handle, 0, b"abc").expect("WRITE");
        g.close(pfand, handle).expect("CLOSE");
        g.setstat(
            pfand,
            &datei,
            Attribute {
                rechte: Some(0o600),
                zeiten: Some((1_700_000_000, 1_700_000_000)),
                ..Attribute::default()
            },
        )
        .expect("SETSTAT");
        let attribute = g.lstat(pfand, &datei).expect("LSTAT");
        let verweis = format!("{mappe}/verweis");
        g.symlink(pfand, &verweis, "datei").expect("SYMLINK");
        let gelesen = g.readlink(pfand, &verweis).expect("READLINK");
        let verweisattribute = g.lstat(pfand, &verweis).expect("LSTAT");
        let fehlt = g.stat(pfand, &format!("{mappe}/fehlt"));
        g.remove(pfand, &verweis).expect("REMOVE");
        g.remove(pfand, &datei).expect("REMOVE");
        g.rmdir(pfand, &mappe).expect("RMDIR");
        (aufgeloest, attribute, gelesen, verweisattribute, fehlt)
    });

    let (aufgeloest, attribute, gelesen, verweisattribute, fehlt) = befund;
    let erwartet = fs::canonicalize(&wurzel).expect("kanonisch");
    assert_eq!(aufgeloest, erwartet.to_str().expect("UTF-8").as_bytes());
    assert_eq!(attribute.groesse, Some(3));
    assert_eq!(attribute.rechte.map(|r| r & 0o7777), Some(0o600));
    assert_eq!(attribute.geaendert(), Some(1_700_000_000));
    assert!(!attribute.ist_ordner() && !attribute.ist_verknuepfung());
    assert_eq!(gelesen, b"datei");
    assert!(verweisattribute.ist_verknuepfung());
    assert!(
        fehlt
            .as_ref()
            .is_err_and(|fehler| fehler.ist_status(status::NO_SUCH_FILE)),
        "{fehlt:?}"
    );
    assert!(!ordner.unter("mappe").exists());
}

// ----------------------------------------------------------------------
// Pfade auf dem Server
// ----------------------------------------------------------------------

/// Ein `Fernpfad` rechnet rein lexikalisch: bereinigt beim Einlesen, nimmt
/// beim Verbinden nur eine einzelne Komponente, und aufwaerts endet an der
/// Wurzel.
#[test]
fn ein_fernpfad_rechnet_rein_lexikalisch() {
    let pfad = Fernpfad::aus_text("//home/./konto//htdocs/../www/").expect("absolut");
    assert_eq!(pfad.als_text(), "/home/konto/www");
    assert_eq!(pfad.name(), Some("www"));
    assert_eq!(Fernpfad::aus_text("relativ"), None);
    assert_eq!(
        Fernpfad::aus_text("/../..").expect("absolut"),
        Fernpfad::wurzel()
    );

    let darunter = pfad.verbinden("bild.png").expect("ein Name");
    assert_eq!(darunter.als_text(), "/home/konto/www/bild.png");
    for kein_name in ["", ".", "..", "a/b"] {
        assert_eq!(pfad.verbinden(kein_name), None, "{kein_name:?}");
    }
    assert_eq!(
        Fernpfad::wurzel()
            .verbinden("etc")
            .expect("ein Name")
            .als_text(),
        "/etc"
    );

    let mut schritte = vec![darunter.als_text().to_owned()];
    let mut stand = darunter;
    while let Some(oben) = stand.aufwaerts() {
        schritte.push(oben.als_text().to_owned());
        stand = oben;
    }
    assert_eq!(
        schritte,
        [
            "/home/konto/www/bild.png",
            "/home/konto/www",
            "/home/konto",
            "/home",
            "/"
        ]
    );
    assert!(stand.ist_wurzel());
    assert_eq!(stand.name(), None);
}

// ----------------------------------------------------------------------
// Am Quelltext
// ----------------------------------------------------------------------

/// Ein `Fernfaden` entsteht an genau einer Stelle, im Rumpf von `fern::faden`
/// in `krk-core/src/fern/faden.rs`, und kein Merkmal wird fuer ihn von Hand
/// umgesetzt.
///
/// Der Uebersetzer haelt die erste Haelfte schon fuer jede andere Datei, weil
/// das Feld privat ist; diese Probe haelt sie fuer `faden.rs` selbst, wo ein
/// zweiter Bau uebersetzen wuerde. Die zweite Haelfte haelt, dass niemand
/// `Send`, `Sync`, `Clone` oder `Default` fuer das Pfand umsetzt: jede Zeile
/// mit `for Fernfaden` im Code eines Quelltexts unter `crates/` macht sie rot.
///
/// # Was diese Nadel nicht sieht
///
/// Ein `derive` am Pfand, ein Bau ueber ein Makro und ein Pfand, das aus dem
/// Rumpf von `faden` heraus in einen zweiten Faden gereicht wird, etwa ueber
/// `Box::leak`. Das Erste steht unmittelbar ueber der Erklaerung des Pfands
/// und ist beim Lesen von `faden.rs` zu sehen; die anderen beiden waeren ein
/// Umbau dieser einen Datei. Der Kopf von `tests/baum.rs` sagt, warum keine
/// Nadel restlos dicht ist.
#[test]
fn fernfaden_entsteht_an_genau_einer_stelle() {
    let bau = concat!("Fernfaden", " {");
    let umsetzung = concat!("for ", "Fernfaden");
    let mut baustellen = Vec::new();
    let mut umsetzungen = Vec::new();
    for (datei, inhalt) in quelldateien() {
        for (nummer, zeile) in inhalt.lines().enumerate() {
            if zeile.trim_start().starts_with("//") {
                continue;
            }
            for (stelle, _) in zeile.match_indices(bau) {
                let davor = zeile[..stelle].trim_end();
                if !davor.ends_with("struct") && !davor.ends_with("impl") {
                    baustellen.push((datei.clone(), nummer + 1));
                }
            }
            if zeile.contains(umsetzung) {
                umsetzungen.push(format!("{datei}:{}", nummer + 1));
            }
        }
    }
    assert_eq!(
        baustellen.len(),
        1,
        "ein Fernfaden entsteht an genau einer Stelle: {baustellen:?}"
    );
    let (datei, zeile) = &baustellen[0];
    assert_eq!(datei, "krk-core/src/fern/faden.rs");
    let quelle = quelldateien()
        .into_iter()
        .find(|(name, _)| name == datei)
        .map(|(_, inhalt)| inhalt)
        .expect("faden.rs steht im Baum");
    let fadenstart = quelle
        .lines()
        .position(|z| z.starts_with("pub fn faden<"))
        .expect("fern::faden steht in faden.rs");
    assert!(
        *zeile > fadenstart + 1,
        "der Bau steht im Rumpf von fern::faden"
    );
    assert!(
        umsetzungen.is_empty(),
        "fuer das Pfand ist ein Merkmal von Hand umgesetzt: {umsetzungen:?}"
    );
}

// ----------------------------------------------------------------------
// Schritt 2: Adresse, ssh-Aufruf, Rechnerschluessel, Zugang
// ----------------------------------------------------------------------

/// Die drei Zeilen der Tabelle in Entscheidung 7, Port, IPv6 und Konto, und
/// die Abweisungen: leerer Rechner, Leerraum, fuehrendes `-`, `@`, Port
/// ausserhalb von 1 bis 65535.
#[test]
fn eine_sftp_adresse_wird_in_rechner_port_und_startordner_zerlegt() {
    let ziel = |adresse: &str, konto: &str| fernziel(adresse, konto).expect("ein Ziel");
    let absolut = |text: &str| Startpfad::Absolut(Fernpfad::aus_text(text).expect("absolut"));

    assert_eq!(
        ziel(" ssh.strato.de ", " stalmann.org "),
        Fernziel {
            rechner: "ssh.strato.de".into(),
            port: None,
            konto: Some("stalmann.org".into()),
            startpfad: Startpfad::Anmeldeverzeichnis,
        }
    );
    assert_eq!(ziel("ssh.strato.de:22", "").port, Some(22));
    assert_eq!(ziel("ssh.strato.de:22", "").konto, None);
    assert_eq!(
        ziel("ssh.strato.de/~/htdocs", "").startpfad,
        Startpfad::UnterAnmeldeverzeichnis("htdocs".into())
    );
    assert_eq!(
        ziel("ssh.strato.de/htdocs", "").startpfad,
        absolut("/htdocs")
    );
    assert_eq!(
        ziel("ssh.strato.de:2222/var/www/", "").startpfad,
        absolut("/var/www")
    );
    for nur_heim in ["host/", "host/~", "host/~/"] {
        assert_eq!(
            ziel(nur_heim, "").startpfad,
            Startpfad::Anmeldeverzeichnis,
            "{nur_heim}"
        );
    }

    let sechs = ziel("[2001:db8::1]:2222/~/a/b", "kai");
    assert_eq!(sechs.rechner, "2001:db8::1");
    assert_eq!(sechs.port, Some(2222));
    assert_eq!(
        sechs.startpfad,
        Startpfad::UnterAnmeldeverzeichnis("a/b".into())
    );
    assert_eq!(ziel("[::1]", "").rechner, "::1");
    let ohne_klammer = ziel("fe80::1", "");
    assert_eq!(
        (ohne_klammer.rechner.as_str(), ohne_klammer.port),
        ("fe80::1", None)
    );

    assert_eq!(fernziel(":22", ""), Err(Adressfehler::RechnerLeer));
    assert_eq!(fernziel("/htdocs", ""), Err(Adressfehler::RechnerLeer));
    for falsch in ["-oProxyCommand=x", "kai@host", "ho st", "[::1", "[::1]x"] {
        assert!(
            matches!(fernziel(falsch, ""), Err(Adressfehler::RechnerUngueltig(_))),
            "{falsch}"
        );
    }
    for port in ["0", "65536", "x", ""] {
        assert_eq!(
            fernziel(&format!("host:{port}"), ""),
            Err(Adressfehler::PortUngueltig(port.into())),
            "{port}"
        );
    }

    // Derselbe Weg ueber das Verbindungsziel, das Blatt und Klick fragen.
    assert_eq!(
        verbindungsziel("sftp", "ssh.strato.de:22", "stalmann.org"),
        Ok(Verbindungsziel::Sftp(ziel(
            "ssh.strato.de:22",
            "stalmann.org"
        )))
    );
    assert_eq!(
        verbindungsziel("sftp", "host:99999", ""),
        Err(Zielfehler::Fernadresse(Adressfehler::PortUngueltig(
            "99999".into()
        )))
    );
    assert!(
        Zielfehler::Fernadresse(Adressfehler::RechnerUngueltig("kai@host".into()))
            .meldung()
            .contains("kai@host")
    );

    // Die Kennung schreibt den Rechner klein und zeigt Konto und Port.
    let kennung = ziel("SSH.Strato.de:2222", "kai").kennung();
    assert_eq!(kennung.rechner(), "ssh.strato.de");
    assert_eq!(kennung.to_string(), "kai@ssh.strato.de:2222");
    assert_eq!(sechs.kennung().to_string(), "kai@[2001:db8::1]:2222");
}

/// Der Aufruf von `ssh` fuer beide Anmeldungen: absolute Programmpfade, die
/// festen Optionen, `--` unmittelbar vor dem Rechner, `sftp` als letztes
/// Argument. Mit Passwort tragen Argumente und Umgebung allein die Pfade von
/// Socket und Programm, ohne Passwort gar keine Umgebung.
///
/// **Was sie nicht sieht, und warum das genuegt:** ob ein Passwort durch
/// einen anderen Weg in den Aufruf kaeme. `ssh_plan` bekommt keines, die
/// Signatur nimmt allein Ziel und Anmeldung, und die Anmeldung traegt zwei
/// Pfade; die Probe haelt deshalb die ganze Liste fest und nicht nur das
/// Fehlen eines Wortes.
#[test]
fn der_ssh_aufruf_traegt_die_festen_optionen_und_nie_ein_passwort() {
    let ziel = fernziel("ssh.strato.de:22/~/htdocs", "stalmann.org").expect("ein Ziel");
    let feste = [
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
    let erwartet = |anmeldeoptionen: &[&str]| -> Vec<String> {
        let mut argumente = Vec::new();
        for option in feste.iter().chain(anmeldeoptionen) {
            argumente.push("-o".to_owned());
            argumente.push((*option).to_owned());
        }
        argumente.extend(
            [
                "-p",
                "22",
                "-l",
                "stalmann.org",
                "-s",
                "--",
                "ssh.strato.de",
                "sftp",
            ]
            .map(str::to_owned),
        );
        argumente
    };
    let als_text = |plan: &Kanalplan| -> Vec<String> {
        plan.argumente
            .iter()
            .map(|argument| argument.to_str().expect("UTF-8").to_owned())
            .collect()
    };

    let ohne = ssh_plan(&ziel, Anmeldung::OhnePasswort);
    assert_eq!(ohne.programm, Path::new("/usr/bin/ssh"));
    assert_eq!(als_text(&ohne), erwartet(&["BatchMode=yes"]));
    assert!(ohne.umgebung.is_empty());

    let socket = Path::new("/var/folders/xy/T/krk-1-0/s");
    let programm = Path::new("/Applications/KRK.app/Contents/MacOS/krk");
    let mit = ssh_plan(
        &ziel,
        Anmeldung::Passwort {
            hilfe: socket,
            programm,
        },
    );
    assert_eq!(mit.programm, Path::new("/usr/bin/ssh"));
    assert_eq!(
        als_text(&mit),
        erwartet(&[
            "BatchMode=no",
            "PubkeyAuthentication=no",
            "PreferredAuthentications=keyboard-interactive,password",
            "NumberOfPasswordPrompts=1",
        ])
    );
    let umgebung: Vec<(String, String)> = mit
        .umgebung
        .iter()
        .map(|(name, wert)| {
            (
                name.to_str().expect("UTF-8").to_owned(),
                wert.to_str().expect("UTF-8").to_owned(),
            )
        })
        .collect();
    assert_eq!(
        umgebung,
        [
            ("SSH_ASKPASS".to_owned(), programm.display().to_string()),
            ("SSH_ASKPASS_REQUIRE".to_owned(), "force".to_owned()),
            (HILFE_VARIABLE.to_owned(), socket.display().to_string()),
        ]
    );
    for (_, wert) in &umgebung {
        assert!(Path::new(wert).is_absolute() || wert == "force", "{wert}");
    }

    // Ohne Port und Konto fallen `-p` und `-l` weg, und der Rechner steht
    // weiter unmittelbar hinter `--`.
    let knapp = ssh_plan(
        &fernziel("host", "").expect("ein Ziel"),
        Anmeldung::OhnePasswort,
    );
    let knapp = als_text(&knapp);
    assert!(!knapp.iter().any(|a| a == "-p" || a == "-l"));
    assert_eq!(&knapp[knapp.len() - 4..], ["-s", "--", "host", "sftp"]);
}

/// Ein Schluessel fuer die Proben, frisch von `ssh-keygen`, als
/// `<typ> <schluessel>`.
fn probeschluessel(ordner: &Pruefordner) -> String {
    let datei = ordner.unter("schluessel");
    let stand = Command::new("/usr/bin/ssh-keygen")
        .args(["-q", "-t", "ed25519", "-N", "", "-C", "probe", "-f"])
        .arg(&datei)
        .stdin(Stdio::null())
        .status()
        .expect("ssh-keygen laesst sich nicht starten");
    assert!(stand.success());
    let oeffentlich = fs::read_to_string(datei.with_extension("pub")).expect("lesen");
    oeffentlich
        .split_whitespace()
        .take(2)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Ein Rechner, dessen Name in `known_hosts` steht, wird gefunden, unter dem
/// Namen, den `ssh` fuer Port, Alias und Rechner sucht; auch gehasht.
#[test]
fn ein_bekannter_rechner_wird_in_known_hosts_gefunden() {
    let ordner = Pruefordner::neu("fern-bekannt");
    let schluessel = probeschluessel(&ordner);
    let datei = ordner.datei(
        "known_hosts",
        format!("example.test {schluessel}\n[example.test]:2222 {schluessel}\n"),
    );
    let konfiguration = |port: u16, alias: Option<&str>| Sshkonfiguration {
        hostname: "example.test".into(),
        port,
        hostkeyalias: alias.map(str::to_owned),
        dateien: vec![datei.clone()],
        eigene_datei: Some(datei.clone()),
    };
    assert_eq!(suchname(&konfiguration(22, None)), "example.test");
    assert_eq!(suchname(&konfiguration(2222, None)), "[example.test]:2222");
    assert_eq!(
        suchname(&konfiguration(2222, Some("example.test"))),
        "example.test"
    );
    for port in [22, 2222] {
        let name = suchname(&konfiguration(port, None));
        assert_eq!(in_datei_bekannt(&name, &datei).ok(), Some(true), "{name}");
    }

    let gehasht = ordner.datei("gehasht", format!("example.test {schluessel}\n"));
    let stand = Command::new("/usr/bin/ssh-keygen")
        .arg("-H")
        .arg("-f")
        .arg(&gehasht)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("ssh-keygen -H");
    assert!(stand.success());
    assert!(
        !fs::read_to_string(&gehasht)
            .expect("lesen")
            .contains("example.test")
    );
    assert_eq!(in_datei_bekannt("example.test", &gehasht).ok(), Some(true));

    // Die Zeilen von `ssh -G`, wie `ssh` 9.9 sie ausgibt.
    let gelesen = konfiguration_lesen(
        "user kai\nhostname example.test\nport 2222\n\
         globalknownhostsfile /etc/ssh/ssh_known_hosts /etc/ssh/ssh_known_hosts2\n\
         userknownhostsfile ~/.ssh/known_hosts /x/known_hosts2\nhostkeyalias none\n",
        Some(Path::new("/Users/kai")),
    )
    .expect("hostname und port stehen da");
    assert_eq!(
        gelesen,
        Sshkonfiguration {
            hostname: "example.test".into(),
            port: 2222,
            hostkeyalias: None,
            dateien: [
                "/Users/kai/.ssh/known_hosts",
                "/x/known_hosts2",
                "/etc/ssh/ssh_known_hosts",
                "/etc/ssh/ssh_known_hosts2",
            ]
            .map(PathBuf::from)
            .to_vec(),
            eigene_datei: Some(PathBuf::from("/Users/kai/.ssh/known_hosts")),
        }
    );
    assert_eq!(konfiguration_lesen("port 22\n", None), None);

    // `none` als Datei des Nutzers ist kein Ort zum Anhaengen.
    let ohne = konfiguration_lesen(
        "hostname example.test\nport 22\nuserknownhostsfile none\n",
        None,
    )
    .expect("hostname und port stehen da");
    assert_eq!(ohne.eigene_datei, None);
}

/// Ein Rechner, der nicht in `known_hosts` steht, auch nicht unter einem
/// anderen Port, wird nicht gefunden.
#[test]
fn ein_unbekannter_nicht() {
    let ordner = Pruefordner::neu("fern-unbekannt");
    let schluessel = probeschluessel(&ordner);
    let datei = ordner.datei("known_hosts", format!("example.test {schluessel}\n"));
    for name in ["anders.test", "[example.test]:2222"] {
        assert_eq!(in_datei_bekannt(name, &datei).ok(), Some(false), "{name}");
    }
}

/// Zwei feste Schluessel mit den Fingerabdruecken, die `ssh-keygen` 9.9
/// fuer sie nennt; gerechnet am 261016 mit `ssh-keygen -l -E sha256`.
const FESTE_SCHLUESSEL: [(&str, &str, &str); 2] = [
    (
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINCchDA8qQ402glvs5GmSdmMtjRAi1vwvNLCJKg3j3dn",
        "ED25519",
        "SHA256:9LQx2fmiP9B1NG/QxKfQPvLAD5VQ4wpNq3wCleSOvg4",
    ),
    (
        "ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBDPb3CYV5rHrHmBjSqMLy14FUquLLMyNWMuKymQSXQDD601n7OMXEpv90bXiPmHDLy9fNScMUCMUSIWZm8aY9UI=",
        "ECDSA",
        "SHA256:DVC3r3J1DFyUKkJRcndaG7bLbOdUSnZo9fjZWuJlZyc",
    ),
];

/// Eine Konfiguration, wie `ssh -G` sie fuer einen Rechner im Pruefordner
/// liefern koennte, mit `datei` als erster `userknownhostsfile`.
fn scankonfiguration(port: u16, alias: Option<&str>, datei: &Path) -> Sshkonfiguration {
    Sshkonfiguration {
        hostname: "example.test".into(),
        port,
        hostkeyalias: alias.map(str::to_owned),
        dateien: vec![datei.to_owned()],
        eigene_datei: Some(datei.to_owned()),
    }
}

/// Die Fingerabdruecke kommen von `ssh-keygen`, je Zeile in ihrer
/// Reihenfolge, und die Zeilen tragen den Namen, unter dem `ssh` sucht
/// (Akzeptanz von Schritt 8).
///
/// Ohne Netz: die Ausgabe von `ssh-keyscan` steht als Text da, mit einem
/// Kommentar, einer doppelten Zeile, einer Zeile mit vier Feldern, einer mit
/// einem fremden Zeichen im Schluessel und einer, die `ssh-keygen` nicht
/// liest; alle fuenf fallen weg. Daneben haelt ein frisch erzeugter Schluessel
/// die Zuordnung an der Ausgabe von `ssh-keygen -l` fuer seine Datei fest.
#[test]
fn die_fingerabdruecke_kommen_von_ssh_keygen() {
    let ordner = Pruefordner::neu("fern-fingerabdruck");
    let frisch = probeschluessel(&ordner);
    let erwartet_frisch = Command::new("/usr/bin/ssh-keygen")
        .args(["-l", "-E", "sha256", "-f"])
        .arg(ordner.unter("schluessel.pub"))
        .stdin(Stdio::null())
        .output()
        .expect("ssh-keygen -l");
    let erwartet_frisch = String::from_utf8_lossy(&erwartet_frisch.stdout)
        .split_whitespace()
        .nth(1)
        .expect("ein Fingerabdruck")
        .to_owned();
    let datei = ordner.unter("known_hosts");
    let [(ed, _, _), (ec, _, _)] = FESTE_SCHLUESSEL;
    let ausgabe = format!(
        "# example.test:2222 SSH-2.0-OpenSSH_9.9\n\
         [example.test]:2222 {ed}\n\
         [example.test]:2222 {ec}\n\
         [example.test]:2222 {ed}\n\
         [example.test]:2222 {ed} zuviel\n\
         [example.test]:2222 ssh-ed25519 AAAA;rm\n\
         [example.test]:2222 ssh-ed25519 AAAAkeinschluessel\n\
         [example.test]:2222 {frisch}\n"
    );
    let konfiguration = scankonfiguration(2222, None, &datei);
    let gescannt = im_fernfaden(move |pfand| {
        gescannt_aus(pfand, &konfiguration, "[example.test]:2222", &ausgabe)
    })
    .expect("drei Schluessel");
    assert_eq!(gescannt.rechner(), "example.test");
    assert_eq!(gescannt.port(), 2222);
    assert_eq!(gescannt.suchname(), "[example.test]:2222");
    assert_eq!(gescannt.datei(), Some(datei.as_path()));
    let fest = |(_, typ, abdruck): (&str, &str, &str)| Fingerabdruck {
        typ: typ.to_owned(),
        abdruck: abdruck.to_owned(),
    };
    assert_eq!(
        gescannt.fingerabdruecke(),
        [
            fest(FESTE_SCHLUESSEL[0]),
            fest(FESTE_SCHLUESSEL[1]),
            Fingerabdruck {
                typ: "ED25519".to_owned(),
                abdruck: erwartet_frisch,
            },
        ]
    );
    assert_eq!(
        gescannt.zeilen(),
        [
            format!("[example.test]:2222 {ed}"),
            format!("[example.test]:2222 {ec}"),
            format!("[example.test]:2222 {frisch}"),
        ]
    );

    // Mit `HostKeyAlias` steht der Alias vorn, denn unter ihm sucht `ssh`.
    let konfiguration = scankonfiguration(22, Some("kurz"), &datei);
    let zeile = format!("example.test {ed}\n");
    let mit_alias = im_fernfaden(move |pfand| gescannt_aus(pfand, &konfiguration, "kurz", &zeile))
        .expect("ein Schluessel");
    assert_eq!(mit_alias.zeilen(), [format!("kurz {ed}")]);

    // Ohne gueltige Zeile gibt es nichts anzunehmen.
    let konfiguration = scankonfiguration(22, None, &datei);
    let leer = im_fernfaden(move |pfand| {
        gescannt_aus(
            pfand,
            &konfiguration,
            "example.test",
            "# nur ein Kommentar\n",
        )
    });
    assert!(leer.is_err(), "{leer:?}");
}

/// „Annehmen“ legt den fehlenden Ordner mit `0700` und die fehlende Datei
/// mit `0600` an, haengt an eine vorhandene allein die gescannten Zeilen an
/// und laesst deren Rechte und Inhalt stehen (Akzeptanz von Schritt 8).
///
/// Geschrieben wird allein im [`Pruefordner`]; die `known_hosts` des Nutzers
/// erreicht die Probe nie, denn die Datei kommt aus der Konfiguration und
/// nicht aus `ssh -G`.
#[test]
fn das_anhaengen_legt_verzeichnis_und_datei_mit_den_engen_rechten_an_und_schreibt_nur_die_gescannten_zeilen()
 {
    use std::os::unix::fs::PermissionsExt;

    let ordner = Pruefordner::neu("fern-anhaengen");
    let [(ed, _, _), (ec, _, _)] = FESTE_SCHLUESSEL;
    let ausgabe = format!("example.test {ed}\nexample.test {ec}\n");
    let rechte = |pfad: &Path| fs::metadata(pfad).expect("steht da").permissions().mode() & 0o777;

    // Ordner und Datei fehlen.
    let neu = ordner.unter("punktssh").join("known_hosts");
    let konfiguration = scankonfiguration(22, None, &neu);
    let ausgabe_neu = ausgabe.clone();
    im_fernfaden(move |pfand| {
        let gescannt = gescannt_aus(pfand, &konfiguration, "example.test", &ausgabe_neu)
            .expect("zwei Schluessel");
        anhaengen(pfand, &gescannt)
    })
    .expect("angehaengt");
    assert_eq!(rechte(&ordner.unter("punktssh")), 0o700);
    assert_eq!(rechte(&neu), 0o600);
    assert_eq!(
        fs::read_to_string(&neu).expect("lesen"),
        format!("example.test {ed}\nexample.test {ec}\n")
    );
    assert_eq!(
        in_datei_bekannt("example.test", &neu).ok(),
        Some(true),
        "ssh-keygen -F findet den angehaengten Rechner"
    );

    // Eine vorhandene Datei ohne Zeilenende am Schluss, mit weiten Rechten.
    let alt = format!("alt.test {ed}");
    let vorhanden = ordner.datei("known_hosts", &alt);
    fs::set_permissions(&vorhanden, fs::Permissions::from_mode(0o644)).expect("Rechte");
    let konfiguration = scankonfiguration(22, None, &vorhanden);
    im_fernfaden(move |pfand| {
        let gescannt =
            gescannt_aus(pfand, &konfiguration, "example.test", &ausgabe).expect("zwei Schluessel");
        anhaengen(pfand, &gescannt)
    })
    .expect("angehaengt");
    assert_eq!(
        rechte(&vorhanden),
        0o644,
        "eine vorhandene Datei behaelt ihre Rechte"
    );
    assert_eq!(
        fs::read_to_string(&vorhanden).expect("lesen"),
        format!("{alt}\nexample.test {ed}\nexample.test {ec}\n")
    );

    // Ohne Datei des Nutzers wird nichts geschrieben.
    let mut ohne = scankonfiguration(22, None, &neu);
    ohne.eigene_datei = None;
    let zeile = format!("example.test {ed}\n");
    let fehler = im_fernfaden(move |pfand| {
        let gescannt = gescannt_aus(pfand, &ohne, "example.test", &zeile).expect("ein Schluessel");
        anhaengen(pfand, &gescannt)
    })
    .expect_err("keine Datei");
    assert_eq!(fehler.kind(), std::io::ErrorKind::NotFound);
}

/// Ein Rechnername mit einem Zeichen, das `known_hosts` als Muster liest,
/// kommt weder durch das Zerlegen der Adresse noch in eine Zeile zum
/// Anhaengen (Befund `261017-0004-ein-rechnername-mit-komma-oder-stern-…`).
///
/// Ohne die Regel nahm `fernziel` `a.example,*` an, und `gescannt_aus` baute
/// daraus die Zeile `a.example,* <typ> <schluessel>`, die fuer jeden Rechner
/// gilt. Der Suchname wird an `gescannt_aus` eigens geprueft, weil er aus
/// `ssh -G` kommt und damit auch aus `Hostname` oder `HostKeyAlias` der
/// Konfiguration; die Datei im [`Pruefordner`] bleibt dabei unberuehrt.
#[test]
fn ein_rechnername_mit_musterzeichen_kommt_nicht_nach_known_hosts() {
    for falsch in [
        "a.example,*",
        "a*",
        "a?.example",
        "!a.example",
        "a b",
        "-a",
        "[a.example]",
        "[a.example]:22",
        "a]b",
        "a[b",
        "a\u{e4}.example",
    ] {
        assert!(
            matches!(fernziel(falsch, ""), Err(Adressfehler::RechnerUngueltig(_))),
            "{falsch}"
        );
    }
    for gut in [
        "ssh.strato.de",
        "[::1]:2222",
        "10.0.0.1",
        "fe80::1%en0",
        "a_b-c",
    ] {
        assert!(fernziel(gut, "").is_ok(), "{gut}");
    }

    let ordner = Pruefordner::neu("fern-musterzeichen");
    let datei = ordner.unter("known_hosts");
    let [(ed, _, _), _] = FESTE_SCHLUESSEL;
    for falsch in [
        "a.example,*",
        "*",
        "a?.example",
        "!a.example",
        "[a.example,*]:2222",
        "[a.example]:0",
        "[a.example]:x",
        "a.example kurz",
    ] {
        let konfiguration = scankonfiguration(22, Some(falsch), &datei);
        let zeile = format!("a.example {ed}\n");
        let suchname = falsch.to_owned();
        let ergebnis =
            im_fernfaden(move |pfand| gescannt_aus(pfand, &konfiguration, &suchname, &zeile));
        assert!(ergebnis.is_err(), "{falsch}: {ergebnis:?}");
    }
    for gut in [
        "ssh.strato.de",
        "[::1]:2222",
        "10.0.0.1",
        "[example.test]:2222",
    ] {
        let konfiguration = scankonfiguration(22, None, &datei);
        let zeile = format!("example.test {ed}\n");
        let suchname = gut.to_owned();
        let gescannt =
            im_fernfaden(move |pfand| gescannt_aus(pfand, &konfiguration, &suchname, &zeile))
                .unwrap_or_else(|fehler| panic!("{gut}: {fehler}"));
        assert_eq!(gescannt.zeilen(), [format!("{gut} {ed}")]);
    }
    assert!(!datei.exists(), "die Probe hat etwas angehaengt");
}

/// Ein Zugang auf dem Server des Systems, ohne `ssh` davor.
fn serverplan() -> Kanalplan {
    Kanalplan::nur(SFTP_SERVER)
}

/// Ein zweiter Aufbau zum selben Server nimmt den lebenden, offenen Zugang
/// und bereitet nichts vor; ein Aufbau zu einem anderen Server baut einen
/// zweiten; faellt der letzte Halter, baut der naechste Aufruf neu.
#[test]
fn ein_zweiter_aufbau_zum_selben_server_nimmt_den_lebenden_zugang() {
    im_fernfaden(|pfand| {
        let register = Zugangsregister::neu();
        let kennung = Serverkennung::neu("Server.Test", None, Some("kai"));
        let mut vorbereitet = 0;
        let mut holen = |kennung: &Serverkennung| {
            register
                .zugang(pfand, kennung, "Probe", || {
                    vorbereitet += 1;
                    Ok::<_, ()>((serverplan(), ()))
                })
                .expect("der Zugang steht")
        };
        let (erster, herkunft) = holen(&kennung);
        assert_eq!(herkunft, Herkunft::Neu);
        assert_eq!(erster.verbindung(), "Probe");
        let (zweiter, herkunft) = holen(&Serverkennung::neu("server.test", None, Some("kai")));
        assert_eq!(herkunft, Herkunft::Lebend);
        assert!(Arc::ptr_eq(&erster, &zweiter));
        let (anderer, herkunft) = holen(&Serverkennung::neu("server.test", Some(22), Some("kai")));
        assert_eq!(herkunft, Herkunft::Neu);
        assert!(!Arc::ptr_eq(&erster, &anderer));
        drop((erster, zweiter));
        let (dritter, herkunft) = holen(&kennung);
        assert_eq!(herkunft, Herkunft::Neu);
        drop((dritter, anderer));
        assert_eq!(vorbereitet, 3);
    });
}

/// Ein getrennter Zugang bekommt beim naechsten Aufbau ein neues Gespraech:
/// derselbe Zugang, ein anderer Kindprozess, wieder offen. Eine Vorbereitung,
/// die abbricht, laesst ihn getrennt.
#[test]
fn ein_getrennter_zugang_bekommt_ein_neues_gespraech() {
    im_fernfaden(|pfand| {
        let register = Zugangsregister::neu();
        let kennung = Serverkennung::neu("server.test", None, None);
        let (zugang, _) = register
            .zugang(pfand, &kennung, "Probe", || Ok::<_, ()>((serverplan(), ())))
            .expect("der Zugang steht");
        let alter_prozess = zugang.gespraech().prozesskennung();
        let beendet = Command::new("/bin/kill")
            .arg("-9")
            .arg(alter_prozess.to_string())
            .status()
            .expect("kill laesst sich nicht starten");
        assert!(beendet.success());
        // Der Gespraechsfaden setzt den Zustand, sobald er das Ende liest.
        let anfang = std::time::Instant::now();
        while !zugang.getrennt() {
            assert!(
                anfang.elapsed() < std::time::Duration::from_secs(10),
                "die Trennung kommt nicht an"
            );
            std::thread::yield_now();
        }

        let abgebrochen = register.zugang(pfand, &kennung, "Probe", || {
            Err::<(Kanalplan, ()), _>("nein")
        });
        assert!(matches!(
            abgebrochen,
            Err(Zugangsfehler::Vorbereitung("nein"))
        ));
        assert!(zugang.getrennt(), "ohne Vorbereitung kein neues Gespraech");

        let (wieder, herkunft) = register
            .zugang(pfand, &kennung, "Probe", || Ok::<_, ()>((serverplan(), ())))
            .expect("der Zugang steht wieder");
        assert_eq!(herkunft, Herkunft::NeuVerbunden);
        assert!(Arc::ptr_eq(&zugang, &wieder));
        assert!(!zugang.getrennt());
        assert_ne!(zugang.gespraech().prozesskennung(), alter_prozess);
        let anmeldeverzeichnis = zugang.anmeldeverzeichnis();
        assert!(
            zugang
                .gespraech()
                .stat(pfand, anmeldeverzeichnis.als_text())
                .expect("das neue Gespraech antwortet")
                .ist_ordner()
        );
    });
}

/// Der Startordner wird auf dem Server aufgeloest: ein vorhandener Ordner
/// gilt, ein fehlender Pfad und eine Datei fuehren auf das
/// Anmeldeverzeichnis und nennen den verlangten Pfad. Ein fernen Ort
/// vergleicht und zeigt sich ueber Kennung und Pfad.
#[test]
fn der_startordner_wird_auf_dem_server_aufgeloest() {
    let ordner = Pruefordner::neu("fern-start");
    ordner.ordner("htdocs");
    ordner.datei("datei", "x");
    let echt = fs::canonicalize(ordner.pfad()).expect("canonicalize");
    let wurzel = text(&echt);
    let w = wurzel.clone();
    let ausgaenge = im_fernfaden(move |pfand| {
        let wurzel = w;
        let kennung = Serverkennung::neu("server.test", None, Some("kai"));
        let zugang =
            Fernzugang::aufbauen(pfand, kennung, "Probe", &serverplan()).expect("der Zugang steht");
        let heim = zugang.anmeldeverzeichnis();
        let start = |pfad: &str| {
            zugang
                .startordner(
                    pfand,
                    &Startpfad::Absolut(Fernpfad::aus_text(pfad).expect("absolut")),
                )
                .expect("eine Antwort")
        };
        let ort = |pfad: Fernpfad| FernerOrt {
            zugang: Arc::clone(&zugang),
            pfad,
        };
        let gefunden = start(&format!("{wurzel}/htdocs"));
        let fehlt = start(&format!("{wurzel}/fehlt"));
        let datei = start(&format!("{wurzel}/datei"));
        let heimstart = zugang
            .startordner(pfand, &Startpfad::Anmeldeverzeichnis)
            .expect("eine Antwort");
        let gleich = ort(gefunden.pfad.clone()) == ort(gefunden.pfad.clone());
        let ungleich = ort(gefunden.pfad.clone()) == ort(heim.clone());
        let gezeigt = ort(Fernpfad::aus_text("/a/b").expect("absolut")).to_string();
        (
            heim, gefunden, fehlt, datei, heimstart, gleich, ungleich, gezeigt,
        )
    });
    let (heim, gefunden, fehlt, datei, heimstart, gleich, ungleich, gezeigt) = ausgaenge;
    assert_eq!(gefunden.pfad.als_text(), format!("{wurzel}/htdocs"));
    assert_eq!(gefunden.fehlte, None);
    assert_eq!(fehlt.pfad, heim);
    assert_eq!(fehlt.fehlte, Some(format!("{wurzel}/fehlt")));
    assert_eq!(datei.pfad, heim);
    assert_eq!(datei.fehlte, Some(format!("{wurzel}/datei")));
    assert_eq!(heimstart.pfad, heim);
    assert!(gleich && !ungleich);
    assert_eq!(gezeigt, "kai@server.test:/a/b");
}

// ----------------------------------------------------------------------
// Der ferne Lesevorgang (Schritt 3)
// ----------------------------------------------------------------------

/// Ein ferner Ort auf dem Server des Systems, aufgebaut in einem Faden mit
/// Pfand; der Lesevorgang darauf startet danach von jedem Faden aus.
fn ferner_ort_auf(pfad: &str) -> FernerOrt {
    let pfad = pfad.to_owned();
    im_fernfaden(move |pfand| {
        let zugang = Fernzugang::aufbauen(
            pfand,
            Serverkennung::neu("server.test", None, Some("kai")),
            "Probe",
            &serverplan(),
        )
        .expect("der Zugang steht");
        FernerOrt {
            zugang,
            pfad: Fernpfad::aus_text(&pfad).expect("absolut"),
        }
    })
}

/// Liest einen Lesevorgang bis `Fertig` und liefert alle Eintraege, den
/// Abschluss und wie viele `Fertig` kamen, bis der Kanal schloss.
fn auslesen(vorgang: Lesevorgang) -> (Vec<Eintrag>, Abschluss, usize, u64) {
    let mut eintraege = Vec::new();
    let mut abschluss = None;
    let mut fertig = 0;
    while let Ok(meldung) = vorgang.meldungen().recv() {
        match meldung {
            Meldung::Stapel {
                eintraege: stapel, ..
            } => {
                assert!(stapel.len() <= STAPELGROESSE, "ein Stapel ist zu gross");
                eintraege.extend(stapel);
            }
            Meldung::Fertig {
                abschluss: ende, ..
            } => {
                fertig += 1;
                abschluss = Some(ende);
            }
        }
    }
    let uebersprungen = vorgang.uebersprungen();
    vorgang.warten();
    (
        eintraege,
        abschluss.expect("der Lauf meldet keinen Abschluss"),
        fertig,
        uebersprungen,
    )
}

/// Was an einem Eintrag verglichen wird: Name, Groesse, Typ, Versteckt und
/// die Aenderungszeit auf die Sekunde.
fn vergleichbar(eintraege: &[Eintrag]) -> Vec<(String, u64, Typ, bool, u64)> {
    let mut zeilen: Vec<_> = eintraege
        .iter()
        .map(|eintrag| {
            let sekunden = eintrag
                .geaendert
                .duration_since(std::time::UNIX_EPOCH)
                .expect("nach 1970")
                .as_secs();
            (
                eintrag.name.clone(),
                eintrag.groesse,
                eintrag.typ,
                eintrag.versteckt,
                sekunden,
            )
        })
        .collect();
    zeilen.sort_by(|links, rechts| links.0.cmp(&rechts.0));
    zeilen
}

/// Derselbe Ordner liefert ueber den lokalen und den fernen Lesevorgang
/// dieselben Eintraege: Namen, Groessen, Typen, Versteckt-Kennzeichen und
/// Aenderungszeit auf die Sekunde (Entscheidungen 8 und 14 des Plans). Mehr
/// als ein Stapel kommt dabei an, und keiner ist uebersprungen.
#[test]
fn fern_und_lokal_liefern_fuer_denselben_ordner_dieselben_eintraege() {
    let ordner = Pruefordner::neu("fern-gleich");
    for i in 0..1500 {
        ordner.datei(&format!("datei-{i:04}.txt"), "x".repeat(i % 37));
    }
    ordner.datei(".versteckt", "geheim");
    ordner.datei("Ärger mit Umlaut.md", "ä");
    ordner.ordner("unterordner");
    std::os::unix::fs::symlink("datei-0001.txt", ordner.unter("verweis"))
        .expect("Verknuepfung laesst sich nicht anlegen");
    let echt = fs::canonicalize(ordner.pfad()).expect("canonicalize");

    let (lokal, lokal_ende, _, _) = auslesen(Lesevorgang::starten(echt.clone(), 1));
    let ort = ferner_ort_auf(&text(&echt));
    let (fern, fern_ende, fertig, uebersprungen) = auslesen(Lesevorgang::fern_starten(ort, 2));

    assert!(matches!(lokal_ende, Abschluss::Vollstaendig));
    assert!(
        matches!(fern_ende, Abschluss::Vollstaendig),
        "der ferne Lauf endete mit {fern_ende:?}"
    );
    assert_eq!(fertig, 1, "genau ein Fertig");
    assert_eq!(uebersprungen, 0);
    assert_eq!(fern.len(), 1504, "ohne . und ..");
    assert_eq!(vergleichbar(&fern), vergleichbar(&lokal));
    let typ = |name: &str| {
        fern.iter()
            .find(|eintrag| eintrag.name == name)
            .map(|eintrag| (eintrag.typ, eintrag.versteckt))
    };
    assert_eq!(typ("unterordner"), Some((Typ::Ordner, false)));
    assert_eq!(typ("verweis"), Some((Typ::Verknuepfung, false)));
    assert_eq!(typ(".versteckt"), Some((Typ::Datei, true)));
}

/// Ein Name, der kein UTF-8 ist, faellt weg und wird gezaehlt; `.` und `..`
/// fallen weg und werden nicht gezaehlt (Entscheidung 8 des Plans).
///
/// **Am reinen Schritt und nicht am Server**: das Dateisystem des Macs nimmt
/// einen solchen Namen gar nicht erst an (`EILSEQ`), also kann ihn der Server
/// im Pruefordner nicht liefern. Der Lesefaden zaehlt mit genau diesem
/// Schritt; dass die Zahl ihn verlaesst, zeigt die Probe darueber mit ihrer
/// Null.
#[test]
fn ein_name_ohne_utf8_wird_uebersprungen_und_gezaehlt() {
    let name = |bytes: &[u8], rechte: u32| Name {
        name: bytes.to_vec(),
        lang: Vec::new(),
        attribute: Attribute {
            groesse: Some(7),
            rechte: Some(rechte),
            zeiten: Some((0, 1_700_000_000)),
            ..Attribute::default()
        },
    };
    let namen = vec![
        name(b".", 0o040_755),
        name(b"..", 0o040_755),
        name(b"gut.txt", 0o100_644),
        name(b"kaputt-\xff\xfe", 0o100_644),
        name(b".versteckt", 0o100_600),
        name(b"ordner", 0o040_755),
        name(b"verweis", 0o120_777),
        name("nicht-\u{e4}rger".as_bytes(), 0o100_644),
    ];
    let mut gesammelt = Vec::new();
    assert_eq!(namen_aufnehmen(namen, &mut gesammelt), 1);
    let zeilen: Vec<(&str, u64, Typ, bool)> = gesammelt
        .iter()
        .map(|eintrag| {
            (
                eintrag.name.as_str(),
                eintrag.groesse,
                eintrag.typ,
                eintrag.versteckt,
            )
        })
        .collect();
    assert_eq!(
        zeilen,
        [
            ("gut.txt", 7, Typ::Datei, false),
            (".versteckt", 7, Typ::Datei, true),
            ("ordner", 0, Typ::Ordner, false),
            ("verweis", 7, Typ::Verknuepfung, false),
            ("nicht-\u{e4}rger", 7, Typ::Datei, false),
        ]
    );
    assert_eq!(
        gesammelt[0]
            .geaendert
            .duration_since(std::time::UNIX_EPOCH)
            .expect("nach 1970")
            .as_secs(),
        1_700_000_000
    );
    let ohne_angaben = Name {
        name: b"nackt".to_vec(),
        lang: Vec::new(),
        attribute: Attribute::default(),
    };
    let Fernname::Eintrag(nackt) = fern_eintrag(&ohne_angaben) else {
        panic!("ein Name ohne Attribute ist ein Eintrag");
    };
    assert_eq!(
        (nackt.typ, nackt.groesse, nackt.geaendert),
        (Typ::Datei, 0, std::time::UNIX_EPOCH),
        "ohne Angaben: Datei, null Bytes, die Unix-Epoche"
    );
}

/// Die eine Namensregel fuer ferne Namen weist leere Namen, `.`, `..`, Namen
/// mit `/` und Namen mit NUL ab, und der Leser zaehlt, was sie abweist, wie
/// einen Namen ohne UTF-8 (Befund
/// `261017-0004-der-ferne-leser-laesst-namen-mit-schraegstrich-leere-namen-und-nul-in-die-liste`).
///
/// Bis zur Behebung liess der Leser `../../x`, einen leeren Namen und einen
/// mit NUL als Zeile in die Liste. Am reinen Schritt und nicht am Server: das
/// Dateisystem des Macs nimmt keinen dieser Namen an.
#[test]
fn ein_name_mit_schraegstrich_leer_oder_mit_nul_wird_uebersprungen_und_gezaehlt() {
    for abgewiesen in ["", "a/b", "../../x", "/", "a\0b", ".", ".."] {
        assert!(
            !ist_einzelner_name(abgewiesen),
            "{abgewiesen:?} gilt als einzelner Name"
        );
        assert!(
            Fernpfad::wurzel().verbinden(abgewiesen).is_none(),
            "verbinden nimmt {abgewiesen:?} an"
        );
    }
    for angenommen in ["a.b", ".versteckt", "...", "a b", "\u{e4}"] {
        assert!(
            ist_einzelner_name(angenommen),
            "{angenommen:?} gilt nicht als einzelner Name"
        );
    }

    let name = |bytes: &[u8]| Name {
        name: bytes.to_vec(),
        lang: Vec::new(),
        attribute: Attribute::default(),
    };
    let namen = vec![
        name(b"."),
        name(b".."),
        name(b"gut.txt"),
        name(b"../../x"),
        name(b""),
        name(b"a\0b"),
        name(b"kaputt-\xff"),
        name(b"a.b"),
    ];
    for unbrauchbar in [&namen[3], &namen[4], &namen[5]] {
        assert!(
            matches!(fern_eintrag(unbrauchbar), Fernname::KeinEinzelnerName),
            "{:?} ist ein Eintrag",
            unbrauchbar.name
        );
    }
    let mut gesammelt = Vec::new();
    assert_eq!(
        namen_aufnehmen(namen, &mut gesammelt),
        4,
        "drei abgewiesene Namen und einer ohne UTF-8; `.` und `..` zaehlen nicht"
    );
    let gelesen: Vec<&str> = gesammelt.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(gelesen, ["gut.txt", "a.b"]);
}

/// Ein abgebrochener ferner Lesevorgang schickt genau ein `Fertig`, und das
/// sagt `Abgebrochen`; danach schliesst der Kanal.
///
/// Abgebrochen wird nach dem ersten Stapel: der Kanal ist einen Stapel tief,
/// also steht der Leser dann vor einem Ordner, von dem noch Namen ausstehen,
/// und fragt den Abbruch, bevor er sie holt.
#[test]
fn ein_abgebrochener_ferner_lesevorgang_schickt_genau_ein_fertig() {
    let ordner = Pruefordner::neu("fern-abbruch");
    for i in 0..6000 {
        fs::File::create(ordner.unter(&format!("d-{i:05}"))).expect("Datei");
    }
    let ort = ferner_ort_auf(&text(
        &fs::canonicalize(ordner.pfad()).expect("canonicalize"),
    ));
    let vorgang = Lesevorgang::fern_starten(ort, 7);
    match vorgang.meldungen().recv().expect("ein erster Stapel") {
        Meldung::Stapel { generation, .. } => assert_eq!(generation, 7),
        Meldung::Fertig { abschluss, .. } => panic!("fertig vor dem ersten Stapel: {abschluss:?}"),
    }
    vorgang.abbrechen();
    let (eintraege, abschluss, fertig, _) = auslesen(vorgang);
    assert_eq!(fertig, 1, "genau ein Fertig");
    assert!(
        matches!(abschluss, Abschluss::Abgebrochen),
        "der Lauf endete mit {abschluss:?}"
    );
    assert!(
        eintraege.len() < 6000 - STAPELGROESSE,
        "der Abbruch griff nicht"
    );
}

/// Ein ferner Ordner, den es nicht gibt, endet mit `Abschluss::Fern` und
/// einem Statuscode des Servers, ohne einen Stapel.
#[test]
fn ein_fehlender_ferner_ordner_endet_mit_dem_statuscode() {
    let ordner = Pruefordner::neu("fern-fehlt");
    let fehlt = format!(
        "{}/gibt-es-nicht",
        text(&fs::canonicalize(ordner.pfad()).expect("canonicalize"))
    );
    let (eintraege, abschluss, fertig, _) =
        auslesen(Lesevorgang::fern_starten(ferner_ort_auf(&fehlt), 3));
    assert!(eintraege.is_empty());
    assert_eq!(fertig, 1);
    assert!(
        matches!(abschluss, Abschluss::Fern(Fernfehler::Status { .. })),
        "{abschluss:?}"
    );
}

/// Aufwaerts endet an der Wurzel des Servers: aus jedem Pfad fuehrt eine
/// endliche Kette bis `/`, jeder Schritt nennt den verlassenen Namen, und
/// oberhalb von `/` gibt es kein Aufwaerts (Entscheidung 15 des Plans).
#[test]
fn aufwaerts_endet_an_der_wurzel_des_servers() {
    let mut pfad = Fernpfad::aus_text("/home/kai/htdocs/bilder").expect("absolut");
    let mut verlassen = Vec::new();
    while let Some(oben) = pfad.aufwaerts() {
        verlassen.push(
            pfad.name()
                .expect("unter der Wurzel hat jeder Pfad einen Namen")
                .to_owned(),
        );
        pfad = oben;
    }
    assert!(pfad.ist_wurzel());
    assert_eq!(verlassen, ["bilder", "htdocs", "kai", "home"]);
    assert_eq!(Fernpfad::wurzel().aufwaerts(), None);
    assert_eq!(Fernpfad::wurzel().name(), None);
}
