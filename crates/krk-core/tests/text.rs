//! Die Textrechnung des Editors: Zeilenindex, Suche und Ersetzen (C5, C6,
//! Schritt 8), das Einlesen und die Sicherungsform (C2, C4, Schritt 9) und die
//! eine Groessen- und Typpruefung vor dem Oeffnen (C2, Schritt 10).
//!
//! Die fuenf Faelle, die das Abnahmekriterium von S8 namentlich nennt:
//!
//! ```text
//! 1  Zeilenindex ueber 10.000 Zeilen gegen einen Durchlauf von Hand
//! 2  Zeilennummer 0 und eine ueber der Zeilenzahl, je mit Kennzeichen
//! 3  Suche in einem Text mit Umlauten und Emojis, Treffer auf Zeichengrenzen
//! 4  Ersetzen aller Treffer, wobei der Ersatztext den Suchtext enthaelt
//! 5  Ersetzen mit leerem Suchtext
//! ```
//!
//! Die beiden Faelle, die das Abnahmekriterium von S9 nennt:
//!
//! ```text
//! 6  Eine Pruefdatei mit CRLF, ohne abschliessenden Umbruch und mit
//!    Bytefolgenmarke: gewandelt beim Lesen, in der Zielform geschrieben
//! 7  Eine Datei, die die Zielform schon hat: Rundreise byteweise unveraendert
//! ```
//!
//! Die fuenf Faelle, die das Abnahmekriterium von S10 nennt:
//!
//! ```text
//!  8  Ein Ordner wird abgewiesen
//!  9  Eine Verknuepfung gilt nach ihrem Ziel
//! 10  Eine Datei von EDITORGRENZE + 1 Bytes wird abgewiesen, nachweislich
//!     ohne gelesen zu werden
//! 11  Eine ungueltige UTF-8-Folge wird abgewiesen, nicht ersetzt
//! 12  Die drei Abweisungsgruende liefern drei verschiedene Meldetexte
//! ```
//!
//! **Die Faelle 1 bis 5 legen keine einzige Datei an**, weil die Textrechnung
//! auf Zeichenketten steht; die Faelle ab 6 pruefen die beiden Enden, an denen
//! Bytes hereinkommen und hinausgehen, und brauchen dafuer einen Pruefordner.
//! Er traegt Prozesskennung und Laufnummer und raeumt sich in `Drop` selbst
//! ab, so wie in `tests/verzeichnis.rs`.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use krk_core::text::{Abweisung, Zeilenindex, Zeilenlage, Zeilensprung, datei, suche};

/// Der Durchlauf von Hand: Byte fuer Byte, ohne die Rechnung des Index.
///
/// Er ist absichtlich anders gebaut als [`Zeilenindex::neu`], das ueber
/// `match_indices` geht. Eine Probe, die dieselbe Rechnung zweimal aufschriebe,
/// pruefte nichts.
fn anfaenge_von_hand(text: &str) -> Vec<usize> {
    let mut anfaenge = vec![0];
    for (stelle, byte) in text.bytes().enumerate() {
        if byte == b'\n' {
            anfaenge.push(stelle + 1);
        }
    }
    anfaenge
}

/// Fall 1: der Zeilenindex einer Datei mit 10.000 Zeilen.
///
/// Die Zeilen tragen Mehrbytezeichen, damit ein Index, der Zeichen statt Bytes
/// zaehlte, hier auffiele und nicht erst in der `NSTextView`.
#[test]
fn zehntausend_zeilen_liefern_dieselben_versaetze_wie_ein_durchlauf_von_hand() {
    let text: String = (1..=10_000)
        .map(|nummer| format!("Zeile {nummer}: Äpfel, Birnen und 🍎\n"))
        .collect();
    let index = Zeilenindex::neu(&text);
    let von_hand = anfaenge_von_hand(&text);

    // 10.000 Umbrueche oeffnen eine leere letzte Zeile: 10.001 Zeilen.
    assert_eq!(index.zeilenzahl(), 10_001);
    assert_eq!(index.zeilenzahl(), von_hand.len());

    for (stelle, erwartet) in von_hand.iter().enumerate() {
        let nummer = stelle + 1;
        assert_eq!(
            index.anfang_der_zeile(nummer),
            Zeilensprung {
                versatz: *erwartet,
                lage: Zeilenlage::Getroffen,
            },
            "Zeile {nummer}"
        );
        assert_eq!(
            index.zeile_am_versatz(*erwartet),
            nummer,
            "der Rueckweg aus Versatz {erwartet}"
        );
        assert!(
            text.is_char_boundary(*erwartet),
            "Versatz {erwartet} liegt nicht auf einer Zeichengrenze"
        );
    }
}

/// Fall 2: die Zeilennummer 0 und eine ueber der Zeilenzahl.
///
/// Beide liefern ein Ergebnis samt Kennzeichen. Das zweite Abnahmekriterium
/// von C5 verlangt fuer die zu grosse Nummer den Sprung an das Dateiende und
/// eine Meldung; die Meldung braucht das Kennzeichen, und deshalb steht es hier
/// und nicht nur der Versatz.
#[test]
fn die_null_und_eine_zu_grosse_nummer_liefern_je_ein_kennzeichen() {
    let text = "eins\nzwei\ndrei";
    let index = Zeilenindex::neu(text);
    assert_eq!(index.zeilenzahl(), 3);

    assert_eq!(
        index.anfang_der_zeile(0),
        Zeilensprung {
            versatz: 0,
            lage: Zeilenlage::VorDerErsten,
        }
    );
    assert_eq!(
        index.anfang_der_zeile(4),
        Zeilensprung {
            versatz: text.len(),
            lage: Zeilenlage::HinterDerLetzten,
        }
    );
    assert_eq!(
        index.anfang_der_zeile(3),
        Zeilensprung {
            versatz: 10,
            lage: Zeilenlage::Getroffen,
        },
        "die letzte Zeile selbst ist getroffen"
    );

    // Dieselbe Regel, mit abschliessendem Umbruch: das Dateiende ist dann der
    // Anfang der leeren letzten Zeile.
    let mit_umbruch = Zeilenindex::neu("eins\nzwei\n");
    assert_eq!(
        mit_umbruch.anfang_der_zeile(99),
        Zeilensprung {
            versatz: 10,
            lage: Zeilenlage::HinterDerLetzten,
        }
    );
}

/// Fall 3: eine Suche in einem Text mit Umlauten und Emojis.
///
/// Gepruefte Zusage ist nicht nur, dass gefunden wird, sondern **wo**: jede
/// Grenze liegt auf einer Zeichengrenze, und der ausgeschnittene Bereich ist
/// buchstaeblich der Suchtext.
#[test]
fn treffer_in_umlauten_und_emojis_liegen_auf_zeichengrenzen() {
    let text = "Äpfel 🍎 und Birnen\nnoch mehr Äpfel 🍎 im Korb\n";

    for gesucht in ["Äpfel", "🍎", "Birnen"] {
        let treffer = suche::alle(text, gesucht);
        assert!(!treffer.is_empty(), "{gesucht} nicht gefunden");
        for fund in &treffer {
            assert!(
                text.is_char_boundary(fund.anfang) && text.is_char_boundary(fund.ende),
                "{gesucht}: {fund:?} liegt nicht auf Zeichengrenzen"
            );
            assert_eq!(&text[fund.anfang..fund.ende], gesucht);
        }
    }

    assert_eq!(suche::alle(text, "Äpfel").len(), 2);
    assert_eq!(suche::alle(text, "🍎").len(), 2);

    // Der Zeilenindex und die Suche meinen dieselben Versaetze: der zweite
    // Treffer steht in der zweiten Zeile.
    let index = Zeilenindex::neu(text);
    let zweiter = suche::alle(text, "Äpfel")[1];
    assert_eq!(index.zeile_am_versatz(zweiter.anfang), 2);

    // Ein Bytepaar mitten in einem Mehrbytezeichen ist kein Treffer: gesucht
    // wird ueber Zeichen und nicht ueber Bytes.
    assert!(suche::alle(text, "pfel 🍎 und").len() == 1);
}

/// Fall 4: ein Ersetzen ueber alle Treffer, bei dem der Ersatztext den
/// Suchtext enthaelt.
///
/// Der Lauf muss enden. Er tut es, weil er auf der Trefferliste des alten
/// Standes steht; ein Lauf, der nach jedem Ersatz erneut suchte, liefe hier
/// bis zum Speicherende.
#[test]
fn ein_ersatz_der_den_suchtext_enthaelt_endet_und_zaehlt_richtig() {
    let ergebnis = suche::alle_ersetzen("foo bar foo baz foo", "foo", "foofoo");
    assert_eq!(ergebnis.zahl, 3);
    assert_eq!(ergebnis.stand, "foofoo bar foofoo baz foofoo");

    // Derselbe Fall beim einzelnen Ersetzen: der naechste Treffer liegt hinter
    // dem eingesetzten Text und nicht darin.
    let text = "foo foo";
    let erster = suche::alle(text, "foo")[0];
    let einzeln = suche::einen_ersetzen(text, "foo", "xfoox", erster);
    assert_eq!(einzeln.stand, "xfoox foo");
    assert_eq!(einzeln.naechster.map(|fund| fund.anfang), Some(6));

    // Ein Ersatztext, der kuerzer ist als der Suchtext, laesst den Stand
    // schrumpfen; die Zahl bleibt die Zahl der alten Treffer.
    let kuerzer = suche::alle_ersetzen("aaaa", "aa", "a");
    assert_eq!(kuerzer.zahl, 2);
    assert_eq!(kuerzer.stand, "aa");
}

/// Fall 5: ein Ersetzen mit leerem Suchtext.
///
/// Null Treffer, und der Stand kommt unveraendert zurueck. `str::replace`
/// setzte den Ersatz hier an jede Zeichengrenze; der Modulkopf von
/// `text::suche` haelt fest, warum diese Datei ihn deshalb nicht benutzt.
#[test]
fn ein_leerer_suchtext_liefert_null_treffer_und_aendert_nichts() {
    let text = "Äpfel 🍎 und Birnen";

    assert!(suche::alle(text, "").is_empty());

    let ergebnis = suche::alle_ersetzen(text, "", "-");
    assert_eq!(ergebnis.zahl, 0);
    assert_eq!(ergebnis.stand, text);

    // Auch der leere Text mit leerem Suchtext bleibt, was er ist.
    let leer = suche::alle_ersetzen("", "", "-");
    assert_eq!(leer.zahl, 0);
    assert_eq!(leer.stand, "");
}

// ---------------------------------------------------------------------------
// Schritt 9: das Einlesen und die Sicherungsform
// ---------------------------------------------------------------------------

static ZAEHLER: AtomicU64 = AtomicU64::new(0);

/// Ein Ordner unter dem Temporaerverzeichnis, der sich selbst wieder abraeumt.
///
/// Nicht der Messplatz unter `~/Library/Caches/krk-messplatz`: der gehoert der
/// Messstrecke, nicht den Proben.
struct Pruefordner {
    pfad: PathBuf,
}

impl Pruefordner {
    fn neu(zweck: &str) -> Self {
        let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
        let mut pfad = std::env::temp_dir();
        pfad.push(format!(
            "krk-test-{zweck}-{}-{laufnummer}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&pfad);
        fs::create_dir_all(&pfad).expect("Pruefordner laesst sich nicht anlegen");
        Self { pfad }
    }

    /// Legt eine Datei aus rohen Bytes an und liefert ihren Pfad.
    ///
    /// Bewusst `&[u8]` und nicht `&str`: die Proben schreiben Bytefolgen, die
    /// in Rust-Quelltext als Zeichenkette nicht mehr das waeren, was auf der
    /// Platte stehen soll.
    fn datei(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let pfad = self.pfad.join(name);
        fs::write(&pfad, bytes).expect("Datei laesst sich nicht schreiben");
        pfad
    }
}

impl Drop for Pruefordner {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.pfad);
    }
}

fn bytes_von(pfad: &Path) -> Vec<u8> {
    fs::read(pfad).expect("Datei laesst sich nicht lesen")
}

/// Fall 6: die Pruefdatei, die alle drei Abweichungen zugleich traegt.
///
/// CRLF, kein abschliessender Umbruch, eine Bytefolgenmarke. Gesichert wird
/// **ohne jede Aenderung** am Stand, weil der Preis der Entscheidung vom
/// 260808-0043 genau dort anfaellt: das Sichern aendert Zeilen, die der Nutzer
/// nicht angefasst hat. Diese Probe haelt fest, dass es so gewollt ist.
#[test]
fn die_drei_abweichungen_verschwinden_beim_lesen_und_kommen_nicht_zurueck() {
    let ordner = Pruefordner::neu("sicherungsform");
    let mut roh = Vec::from("\u{feff}".as_bytes());
    roh.extend_from_slice(b"erste\r\nzweite\r\ndritte ohne Umbruch");
    let pfad = ordner.datei("windows.txt", &roh);

    // Was hereinkommt, traegt alle drei Abweichungen.
    assert_eq!(&roh[..3], b"\xef\xbb\xbf", "die Probe braucht die Marke");
    assert!(roh.contains(&0x0D), "die Probe braucht CRLF");
    assert_ne!(*roh.last().unwrap(), 0x0A, "die Probe braucht kein Ende");

    let stand = datei::einlesen(bytes_von(&pfad)).expect("die Datei ist Text");

    // Nach dem Einlesen: kein Wagenruecklauf, keine fuehrende Marke.
    assert!(!stand.contains('\r'), "der Stand traegt noch ein \\r");
    assert!(
        !stand.starts_with('\u{feff}'),
        "der Stand traegt noch die Marke"
    );
    assert_eq!(stand, "erste\nzweite\ndritte ohne Umbruch");

    // Der Zeilenindex sieht drei Zeilen und nicht sechs halbe.
    assert_eq!(Zeilenindex::neu(&stand).zeilenzahl(), 3);

    datei::sichern(&pfad, &stand).expect("das Sichern scheitert nicht");
    let geschrieben = bytes_von(&pfad);

    assert!(
        !geschrieben.contains(&0x0D),
        "auf der Platte steht ein 0x0D"
    );
    assert_eq!(
        geschrieben.last(),
        Some(&0x0A),
        "die Datei endet nicht auf einem Umbruch"
    );
    assert_ne!(
        &geschrieben[geschrieben.len() - 2..],
        b"\n\n",
        "es ist mehr als genau ein abschliessender Umbruch"
    );
    assert_ne!(
        &geschrieben[..3],
        b"\xef\xbb\xbf",
        "auf der Platte steht wieder eine Marke"
    );
    assert_eq!(
        geschrieben.as_slice(),
        b"erste\nzweite\ndritte ohne Umbruch\n"
    );

    // Und die Nachbardatei aus `ablage::atomar` liegt nicht herum.
    assert!(
        !pfad.with_file_name("windows.txt.neu").exists(),
        "die Nachbardatei ist liegengeblieben"
    );
}

/// Fall 7: eine Datei, die die Zielform schon hat, ueberlebt die Rundreise
/// byteweise.
///
/// Das ist die Gegenprobe zu Fall 6: die Wandlung greift **nur** dort, wo
/// etwas zu wandeln ist. Der leere Stand steht mit dabei, weil er der einzige
/// Fall ist, in dem kein abschliessender Umbruch angehaengt wird.
#[test]
fn die_zielform_ueberlebt_die_rundreise_byteweise() {
    let ordner = Pruefordner::neu("rundreise");

    for (name, inhalt) in [
        ("gewoehnlich.txt", "eins\nzwei\nÄpfel 🍎\n".as_bytes()),
        ("leerzeile-am-ende.txt", b"eins\n\n\n".as_slice()),
        ("leer.txt", b"".as_slice()),
        ("eine-zeile.txt", b"nur eine\n".as_slice()),
    ] {
        let pfad = ordner.datei(name, inhalt);
        let stand = datei::einlesen(bytes_von(&pfad)).expect("die Datei ist Text");
        datei::sichern(&pfad, &stand).expect("das Sichern scheitert nicht");
        assert_eq!(
            bytes_von(&pfad).as_slice(),
            inhalt,
            "{name} kam veraendert zurueck"
        );
    }
}

/// Der einzelne Wagenruecklauf, wie ihn alte Mac-Dateien tragen, wird
/// ebenfalls zu `\n` und nicht zu nichts.
///
/// Er ist der Fall, in dem eine Wandlung, die nur `\r\n` kennt, den ganzen
/// Text in eine einzige Zeile zoege.
#[test]
fn ein_einzelner_wagenruecklauf_wird_zu_einem_umbruch() {
    let stand = datei::einlesen(Vec::from(b"eins\rzwei\rdrei".as_slice())).expect("das ist Text");
    assert_eq!(stand, "eins\nzwei\ndrei");
    assert_eq!(Zeilenindex::neu(&stand).zeilenzahl(), 3);

    // Gemischt in einer Datei: jede der drei Formen liefert genau einen
    // Umbruch, und aus `\r\n` werden nicht zwei.
    let gemischt = datei::einlesen(Vec::from(b"a\r\nb\rc\nd".as_slice())).expect("das ist Text");
    assert_eq!(gemischt, "a\nb\nc\nd");
}

/// `in_gehaltene_form` ist die eine Stelle, und sie ist von aussen erreichbar.
///
/// Der Fall, der ansteht, ist der Ersatztext des Suchen-und-Ersetzens aus C5
/// (Schritt 37): er kommt aus einem Eingabefeld und kann ein `\r` tragen.
/// Wer ihn ungewandelt in den Stand setzt, bricht die Zusage; wer ihn hier
/// hindurchfuehrt, braucht keine eigene Wandlung zu schreiben.
#[test]
fn ein_ersatztext_geht_durch_dieselbe_stelle_wie_das_eingelesene() {
    let ersatz = datei::in_gehaltene_form(String::from("erste\r\nzweite"));
    assert_eq!(ersatz, "erste\nzweite");

    let stand = datei::einlesen(Vec::from(b"vorher X nachher\n".as_slice())).expect("das ist Text");
    let ergebnis = suche::alle_ersetzen(&stand, "X", &ersatz);
    assert_eq!(ergebnis.zahl, 1);
    assert!(
        !ergebnis.stand.contains('\r'),
        "der neue Stand traegt ein \\r"
    );
    assert_eq!(ergebnis.stand, "vorher erste\nzweite nachher\n");

    // Ein Text, der die Form schon hat, kommt woertlich zurueck.
    let unveraendert = datei::in_gehaltene_form(String::from("Äpfel 🍎\n"));
    assert_eq!(unveraendert, "Äpfel 🍎\n");
}

/// Abgeschnitten wird allein die **fuehrende** Marke.
///
/// Ein `U+FEFF` mitten im Text ist ein Zeichen des Nutzers und kein Rahmen.
#[test]
fn eine_marke_mitten_im_text_bleibt_stehen() {
    let mut roh = Vec::from("\u{feff}".as_bytes());
    roh.extend_from_slice("eins\u{feff}zwei\n".as_bytes());
    let stand = datei::einlesen(roh).expect("das ist Text");
    assert_eq!(stand, "eins\u{feff}zwei\n");
}

/// Ungueltiges UTF-8 liefert keinen Stand und keine Ersatzzeichen.
///
/// Welchen Satz der Nutzer dazu liest, entscheidet der Abweisungsgrund aus
/// Schritt 10; hier steht allein, dass nichts durchkommt.
#[test]
fn ungueltiges_utf8_liefert_keinen_stand() {
    assert!(datei::einlesen(Vec::from(b"gueltig \xff\xfe kaputt".as_slice())).is_none());
    assert!(datei::einlesen(Vec::from(b"\x80".as_slice())).is_none());
    assert!(datei::einlesen(Vec::new()).is_some(), "leer ist gueltig");
}

/// Die Sicherungsform in ihren drei Faellen, ohne eine einzige Datei.
///
/// Sie sind ueberschneidungsfrei und vollstaendig: leer, endet auf `\n`,
/// alles Uebrige.
#[test]
fn die_sicherungsform_haengt_genau_einen_umbruch_an_und_raeumt_hinten_nicht_auf() {
    assert_eq!(datei::sicherungsform(""), "");
    assert_eq!(datei::sicherungsform("ohne"), "ohne\n");
    assert_eq!(datei::sicherungsform("mit\n"), "mit\n");
    assert_eq!(
        datei::sicherungsform("zwei leere Zeilen\n\n\n"),
        "zwei leere Zeilen\n\n\n",
        "die leeren Zeilen am Ende sind Text des Nutzers"
    );
}

// ---------------------------------------------------------------------------
// Schritt 10: die eine Groessen- und Typpruefung vor dem Oeffnen
// ---------------------------------------------------------------------------

// Die fuenf Faelle, die das Abnahmekriterium von S10 namentlich nennt:
//
//  8  Ein Ordner wird abgewiesen
//  9  Eine Verknuepfung gilt nach ihrem Ziel: auf eine Textdatei angenommen,
//     auf einen Ordner abgewiesen
// 10  Eine Datei von EDITORGRENZE + 1 Bytes wird abgewiesen, und zwar
//     nachweislich, ohne gelesen zu werden
// 11  Eine Datei mit ungueltiger UTF-8-Folge wird abgewiesen und nicht mit
//     Ersatzzeichen geliefert
// 12  Die drei Abweisungsgruende liefern drei verschiedene Meldetexte

impl Pruefordner {
    /// Legt eine Datei der genannten Groesse an, **ohne ein Byte zu
    /// schreiben**.
    ///
    /// `set_len` zieht die Datei auf die Laenge und laesst dahinter ein Loch.
    /// Auf APFS kostet das weder Platz noch Zeit, und genau deshalb kann eine
    /// Probe hier von zwei Gigabyte reden, ohne zwei Gigabyte anzulegen. Wer
    /// das Loch liest, bekommt Nullbytes; die sind gueltiges UTF-8, was den
    /// Grenzfall unten erst brauchbar macht.
    fn luecke(&self, name: &str, groesse: u64) -> PathBuf {
        let pfad = self.pfad.join(name);
        let datei = fs::File::create(&pfad).expect("Luecke laesst sich nicht anlegen");
        datei
            .set_len(groesse)
            .expect("Luecke laesst sich nicht ziehen");
        pfad
    }

    /// Legt einen Unterordner an und liefert seinen Pfad.
    fn unterordner(&self, name: &str) -> PathBuf {
        let pfad = self.pfad.join(name);
        fs::create_dir(&pfad).expect("Unterordner laesst sich nicht anlegen");
        pfad
    }

    /// Legt eine weiche Verknuepfung auf das genannte Ziel an.
    fn verknuepfung(&self, name: &str, ziel: &Path) -> PathBuf {
        let pfad = self.pfad.join(name);
        std::os::unix::fs::symlink(ziel, &pfad).expect("Verknuepfung laesst sich nicht anlegen");
        pfad
    }
}

/// Nimmt der Datei jedes Recht, damit ein Lesevorgang an ihr scheitern **muss**.
fn sperren(pfad: &Path) {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(pfad, fs::Permissions::from_mode(0o000))
        .expect("Rechte lassen sich nicht setzen");
}

/// Ob die Sperre auf dieser Kennung ueberhaupt wirkt.
///
/// Gefragt wird nicht die Benutzerkennung, sondern die Wirkung: root liest eine
/// gesperrte Datei trotzdem, und unter root sagt der Nachweis unten nichts aus.
fn sperre_wirkt(gesperrt: &Path) -> bool {
    fs::read(gesperrt).is_err()
}

/// Fall 8: ein Ordner wird abgewiesen.
///
/// Er ist der eine Fall, den der Datensatz namentlich als sicher abgewiesen
/// nennt, und er braucht keine eigene Regel: er ist keine gewoehnliche Datei,
/// und daran scheitert er.
#[test]
fn ein_ordner_wird_abgewiesen() {
    let ordner = Pruefordner::neu("oeffnen-ordner");
    let unterordner = ordner.unterordner("ein-ordner");

    let ergebnis = datei::oeffnen(&unterordner);
    assert!(
        matches!(ergebnis, Err(Abweisung::KeinGueltigesZiel { .. })),
        "der Ordner kam nicht als kein gueltiges Ziel zurueck: {ergebnis:?}"
    );
    assert!(
        ergebnis.unwrap_err().meldung().contains("Ordner"),
        "die Meldung nennt den Ordner nicht"
    );
}

/// Fall 9: eine Verknuepfung gilt nach dem, worauf sie zeigt.
///
/// Das ist die Wahl `metadata` statt `symlink_metadata`, und sie ist an drei
/// Zielen gepruefft: Textdatei, Ordner, ins Leere.
#[test]
fn eine_verknuepfung_gilt_nach_ihrem_ziel() {
    let ordner = Pruefordner::neu("oeffnen-verknuepfung");
    let textdatei = ordner.datei("ziel.txt", b"eins\nzwei\n");
    let unterordner = ordner.unterordner("ziel-ordner");

    let auf_text = ordner.verknuepfung("auf-text", &textdatei);
    assert_eq!(
        datei::oeffnen(&auf_text),
        Ok(String::from("eins\nzwei\n")),
        "die Verknuepfung auf eine Textdatei wurde nicht angenommen"
    );

    let auf_ordner = ordner.verknuepfung("auf-ordner", &unterordner);
    assert!(
        matches!(
            datei::oeffnen(&auf_ordner),
            Err(Abweisung::KeinGueltigesZiel { .. })
        ),
        "die Verknuepfung auf einen Ordner wurde nicht abgewiesen"
    );

    let ins_leere = ordner.verknuepfung("ins-leere", &ordner.pfad.join("gibt-es-nicht"));
    assert!(
        matches!(
            datei::oeffnen(&ins_leere),
            Err(Abweisung::KeinGueltigesZiel { .. })
        ),
        "die Verknuepfung ins Leere wurde nicht abgewiesen"
    );
}

/// Fall 10: eine Datei ueber der Grenze wird abgewiesen, **ohne gelesen zu
/// werden**.
///
/// Der Nachweis steht nicht an der Laufzeit, sondern an den Rechten, und er ist
/// damit deterministisch. Zwei Dateien, gleich angelegt, gleich gesperrt, und
/// um genau ein Byte verschieden:
///
/// ```text
///   EDITORGRENZE + 1 Bytes, Rechte 000  ──> ZuGross
///   EDITORGRENZE     Bytes, Rechte 000  ──> KeinGueltigesZiel (Lesefehler)
/// ```
///
/// Die zweite Zeile ist die tragende: sie zeigt, dass unterhalb der Grenze
/// **wirklich** geoeffnet und gelesen wird. Kaeme die Groessenpruefung erst
/// nach dem Lesen, muesste die erste Zeile denselben Lesefehler melden wie die
/// zweite. Sie tut es nicht, also lag die Pruefung davor.
#[test]
fn eine_datei_ueber_der_grenze_wird_abgewiesen_ohne_gelesen_zu_werden() {
    let ordner = Pruefordner::neu("oeffnen-grenze");

    let ueber_der_grenze = ordner.luecke("zu-gross.log", datei::EDITORGRENZE + 1);
    let auf_der_grenze = ordner.luecke("gerade-noch.log", datei::EDITORGRENZE);
    sperren(&ueber_der_grenze);
    sperren(&auf_der_grenze);

    if !sperre_wirkt(&auf_der_grenze) {
        // Unter root liest sich auch eine gesperrte Datei. Dann sagt der
        // Nachweis nichts aus, und eine Probe, die nichts aussagt, behauptet
        // hier auch nichts.
        eprintln!("uebersprungen: die Rechtesperre wirkt auf dieser Kennung nicht");
        return;
    }

    assert_eq!(
        datei::oeffnen(&ueber_der_grenze),
        Err(Abweisung::ZuGross {
            pfad: ueber_der_grenze.clone(),
            groesse: datei::EDITORGRENZE + 1,
        }),
        "die zu grosse Datei kam nicht als zu gross zurueck"
    );
    assert!(
        matches!(
            datei::oeffnen(&auf_der_grenze),
            Err(Abweisung::KeinGueltigesZiel { .. })
        ),
        "die Datei auf der Grenze wurde nicht gelesen, damit belegt die Probe nichts"
    );
}

/// Der Grenzfall selbst: genau [`datei::EDITORGRENZE`] Bytes werden angenommen.
///
/// Die Grenze ist ein `>` und kein `>=`, und das ist keine Kleinigkeit: eine
/// Datei von genau 16 MB ist die, an der ein Nutzer den Unterschied merkt.
/// Gelesen wird aus einem Loch, also aus Nullbytes; die sind gueltiges UTF-8.
#[test]
fn genau_auf_der_grenze_wird_angenommen() {
    let ordner = Pruefordner::neu("oeffnen-grenzfall");
    let auf_der_grenze = ordner.luecke("genau.log", datei::EDITORGRENZE);

    let stand = datei::oeffnen(&auf_der_grenze).expect("genau auf der Grenze gehoert angenommen");
    assert_eq!(stand.len() as u64, datei::EDITORGRENZE);
}

/// Eine Datei weit ueber der Grenze kostet weder Zeit noch Speicher.
///
/// Zwei Gigabyte als Loch: das ist die Protokolldatei aus dem Datensatz. Die
/// Probe haelt fest, dass die Antwort aus einem `stat(2)` kommt und nicht aus
/// zwei Gigabyte im Arbeitsspeicher — die Zeitschranke ist grosszuegig, weil
/// sie nicht messen, sondern nur den Unterschied zwischen Mikrosekunden und
/// Sekunden treffen soll.
#[test]
fn zwei_gigabyte_werden_ohne_arbeitsspeicher_abgewiesen() {
    use std::time::Instant;

    let ordner = Pruefordner::neu("oeffnen-riesig");
    let riesig = ordner.luecke("protokoll.log", 2 * 1024 * 1024 * 1024);

    let begonnen = Instant::now();
    let ergebnis = datei::oeffnen(&riesig);
    let gedauert = begonnen.elapsed();

    assert_eq!(
        ergebnis,
        Err(Abweisung::ZuGross {
            pfad: riesig.clone(),
            groesse: 2 * 1024 * 1024 * 1024,
        })
    );
    assert!(
        gedauert.as_millis() < 500,
        "die Abweisung hat {} ms gebraucht, das riecht nach gelesenen Bytes",
        gedauert.as_millis()
    );
}

/// Fall 11: eine ungueltige UTF-8-Folge wird abgewiesen und nicht ersetzt.
///
/// Hier haengt die bindende Zusage des Datensatzes: kein Weg darf eine Datei
/// beim Sichern veraendern, die der Editor nicht vollstaendig und verlustfrei
/// als Text gelesen hat. Die Probe haelt beides fest — dass abgewiesen wird,
/// und dass kein Ersatzzeichen durchkommt.
#[test]
fn ungueltiges_utf8_wird_abgewiesen_und_nicht_ersetzt() {
    let ordner = Pruefordner::neu("oeffnen-binaer");
    let binaer = ordner.datei("bild.png", b"\x89PNG\r\n\x1a\n\x00\x00\xff\xfe kaputt");

    let ergebnis = datei::oeffnen(&binaer);
    assert_eq!(
        ergebnis,
        Err(Abweisung::NichtAlsTextLesbar {
            pfad: binaer.clone()
        })
    );
    assert!(
        !ergebnis.unwrap_err().meldung().contains('\u{fffd}'),
        "die Meldung traegt ein Ersatzzeichen"
    );
}

/// Fall 12: die drei Gruende liefern drei verschiedene Meldetexte.
///
/// Das neunte Abnahmekriterium von C2 verlangt, "zu gross" von "nicht als Text
/// lesbar" zu unterscheiden. Die Probe prueft die Unterschiedlichkeit und nicht
/// den Wortlaut: der Wortlaut darf sich aendern, die Unterscheidbarkeit nicht.
#[test]
fn die_drei_gruende_liefern_drei_verschiedene_meldetexte() {
    let ordner = Pruefordner::neu("oeffnen-meldungen");
    let unterordner = ordner.unterordner("ein-ordner");
    let zu_gross = ordner.luecke("zu-gross.log", datei::EDITORGRENZE + 1);
    let binaer = ordner.datei("binaer.bin", b"\xff\xfe");

    let meldungen: Vec<String> = [&unterordner, &zu_gross, &binaer]
        .into_iter()
        .map(|pfad| {
            datei::oeffnen(pfad)
                .expect_err("alle drei gehoeren abgewiesen")
                .meldung()
        })
        .collect();

    assert_eq!(meldungen.len(), 3);
    for (stelle, meldung) in meldungen.iter().enumerate() {
        assert!(!meldung.is_empty(), "die {stelle}. Meldung ist leer");
        assert!(
            meldungen.iter().filter(|andere| *andere == meldung).count() == 1,
            "die {stelle}. Meldung ist nicht von den anderen zu unterscheiden: {meldung}"
        );
    }
}

/// Der eine Weg traegt die Wandlung aus Schritt 9 mit.
///
/// `oeffnen` ist kein zweiter Leseweg neben `einlesen`, sondern die Pruefung
/// davor. Die Probe haelt fest, dass eine Datei mit Bytefolgenmarke und CRLF
/// ueber `oeffnen` denselben Stand liefert wie ueber `einlesen`; liefen die
/// beiden auseinander, gaebe es zwei Meinungen darueber, was eine Zeile endet.
#[test]
fn oeffnen_liefert_denselben_stand_wie_einlesen() {
    let ordner = Pruefordner::neu("oeffnen-wandlung");
    let mut roh = Vec::from("\u{feff}".as_bytes());
    roh.extend_from_slice(b"erste\r\nzweite\r\ndritte ohne Umbruch");
    let pfad = ordner.datei("windows.txt", &roh);

    let ueber_oeffnen = datei::oeffnen(&pfad).expect("die Datei ist Text");
    let ueber_einlesen = datei::einlesen(bytes_von(&pfad)).expect("die Datei ist Text");

    assert_eq!(ueber_oeffnen, ueber_einlesen);
    assert_eq!(ueber_oeffnen, "erste\nzweite\ndritte ohne Umbruch");
}
