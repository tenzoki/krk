//! Abnahme des SFTP-Gespraechs im Kern (Schritt 1 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`).
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
use std::path::Path;
use std::process::{Child, Command, Stdio};

use gemeinsam::{Pruefordner, quelldateien};
use krk_core::fern::paket::{
    Anfrage, Antwort, Attribute, Name, PAKETGRENZE, Paket, Paketfehler, oeffnen, paket_lesen,
    status,
};
use krk_core::fern::{
    BLOCK, Fernfaden, Fernfehler, Fernpfad, Gespraech, OFFENE_LESUNGEN, Trennungsgrund, Zustand,
    faden,
};

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
