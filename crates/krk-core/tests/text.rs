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
//! Er kommt aus `tests/gemeinsam/`, traegt Prozesskennung und Laufnummer und
//! raeumt sich in `Drop` selbst ab.
//!
//! **Fall 10 ist am 260810 auf drei Proben verteilt worden**, weil `oeffnen`
//! seither zuerst oeffnet und danach am Deskriptor prueft (Defekt
//! `260809-1652`). Der eine Byte Unterschied bleibt bei Fall 10 selbst, die
//! benannte Roehre und der Wechsel des Pfades unter der Pruefung kommen daneben,
//! und den Satz "ohne gelesen zu werden" traegt jetzt die Zeitmessung an zwei
//! Gigabyte. Jede der Proben sagt in ihrem Kopf, welchen Teil sie haelt.

use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Duration;

use krk_core::text::datei::{Textstand, Unlesbarkeit};
use krk_core::text::{Abweisung, Zeilenindex, Zeilenlage, Zeilensprung, datei, suche};

mod gemeinsam;
use gemeinsam::Pruefordner;

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

/// Die Frage, an der die Wandlung ihren kurzen Weg nimmt, ist von aussen
/// erreichbar und beantwortet dieselbe Sache wie die Wandlung selbst.
///
/// Der Editor fragt sie, um zu erfahren, ob seine Textflaeche nachzuziehen ist;
/// die Zusicherung unten haelt beide aneinander, damit die Pruefung nicht gegen
/// eine andere Bedingung laeuft als die Wandlung.
#[test]
fn die_frage_nach_der_gehaltenen_form_und_die_wandlung_sagen_dasselbe() {
    for text in [
        "",
        "eins\nzwei\n",
        "Äpfel 🍎\n",
        "eins\r\nzwei",
        "alter Mac\rmit einzelnem Ruecklauf",
        "\u{feff}mit fuehrender Marke\n",
        "ohne fuehrende\u{feff}Marke\n",
    ] {
        let gewandelt = datei::in_gehaltene_form(text.to_owned());
        assert_eq!(
            datei::ist_in_gehaltener_form(text),
            gewandelt == text,
            "die Frage und die Wandlung sind sich uneins ueber {text:?}"
        );
        assert!(
            datei::ist_in_gehaltener_form(&gewandelt),
            "das Ergebnis der Wandlung ist in gehaltener Form: {gewandelt:?}"
        );
    }
}

/// Der Defekt 260810-0215, an dem Stueck gemessen, das ihn behebt.
///
/// Die Textflaeche des Editors traegt ein eingefuegtes `\r\n` zeichengetreu,
/// der gehaltene Stand traegt danach ein `\n`. Wer die Schreibmarke der Flaeche
/// unbesehen in den Stand traegt, landet von der eingefuegten Stelle an je `\r`
/// ein Zeichen zu weit hinten; diese Rechnung setzt sie dorthin, wo sie stand.
#[test]
fn eine_stelle_wandert_mit_der_wandlung_in_die_gehaltene_form() {
    let flaeche = "erste\r\nzweite\r\ndritte";
    let stand = datei::in_gehaltene_form(flaeche.to_owned());
    assert_eq!(stand, "erste\nzweite\ndritte");

    // Die Schreibmarke steht hinter „zweite“. Unverrechnet zeigte dieselbe Zahl
    // im Stand schon in die dritte Zeile — das ist der Defekt.
    let hinter_zweite = 13;
    assert_eq!(&flaeche[..hinter_zweite], "erste\r\nzweite");
    assert_eq!(&stand[..hinter_zweite], "erste\nzweite\n");

    let gerechnet = datei::versatz_nach_der_wandlung(flaeche, hinter_zweite, &stand);
    assert_eq!(&stand[..gerechnet], "erste\nzweite");

    // Jede Zeichengrenze des ungewandelten Textes landet auf einer Grenze des
    // gewandelten, keine ueberholt eine davor, und was hinter der Stelle steht,
    // ist auf beiden Seiten dasselbe.
    let mut vorige = 0;
    for stelle in (0..=flaeche.len()).filter(|s| flaeche.is_char_boundary(*s)) {
        let ziel = datei::versatz_nach_der_wandlung(flaeche, stelle, &stand);
        assert!(
            stand.is_char_boundary(ziel),
            "die Stelle {stelle} landet neben einer Zeichengrenze"
        );
        assert!(ziel >= vorige, "die Stelle {stelle} ueberholt die davor");
        vorige = ziel;
        assert_eq!(
            &stand[ziel..],
            datei::in_gehaltene_form(flaeche[stelle..].to_owned()),
            "hinter der Stelle {stelle} stehen zwei verschiedene Texte"
        );
    }

    // Die fuehrende Bytefolgenmarke faellt weg und verschiebt alles dahinter.
    let mit_marke = "\u{feff}Äpfel\r\n🍎";
    let ohne_marke = datei::in_gehaltene_form(mit_marke.to_owned());
    assert_eq!(ohne_marke, "Äpfel\n🍎");
    assert_eq!(
        datei::versatz_nach_der_wandlung(mit_marke, 0, &ohne_marke),
        0
    );
    assert_eq!(
        datei::versatz_nach_der_wandlung(mit_marke, mit_marke.len(), &ohne_marke),
        ohne_marke.len()
    );
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
//     nachweislich, ohne gelesen zu werden — auf drei Proben verteilt, siehe
//     den Modulkopf
// 11  Eine Datei mit ungueltiger UTF-8-Folge wird abgewiesen und nicht mit
//     Ersatzzeichen geliefert
// 12  Die drei Abweisungsgruende liefern drei verschiedene Meldetexte

/// Ruft [`datei::oeffnen`] auf einem eigenen Faden und gibt die Antwort nur
/// heraus, wenn sie innerhalb der Schranke kommt.
///
/// **Der Grund ist die Art des Defekts, der hier geprueft wird:** ein
/// blockierendes `open` liefert kein falsches Ergebnis, sondern gar keines. Ohne
/// Schranke waere das ein stehender Probelauf, und `cargo test` haette nichts zu
/// melden als Stillstand. Mit ihr gibt es einen Befund mit Namen.
///
/// Der Faden bleibt im Fehlerfall stehen, wo er steht. Er stirbt mit dem
/// Probelauf, und ein Deskriptor, der nie aufgeht, haelt nichts fest.
fn oeffnen_mit_zeitschranke(pfad: &Path, schranke: Duration) -> Result<String, Abweisung> {
    let (sender, empfaenger) = mpsc::channel();
    let pfad = pfad.to_path_buf();
    std::thread::spawn(move || {
        let _ = sender.send(datei::oeffnen(&pfad));
    });
    empfaenger.recv_timeout(schranke).unwrap_or_else(|_| {
        panic!("oeffnen ist nach {schranke:?} nicht zurueckgekommen; das Oeffnen haengt")
    })
}

/// Fall 8: ein Ordner wird abgewiesen.
///
/// Er ist der eine Fall, den der Datensatz namentlich als sicher abgewiesen
/// nennt, und er braucht keine eigene Regel: er ist keine gewoehnliche Datei,
/// und daran scheitert er.
#[test]
fn ein_ordner_wird_abgewiesen() {
    let ordner = Pruefordner::neu("oeffnen-ordner");
    let unterordner = ordner.ordner("ein-ordner");

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
    let unterordner = ordner.ordner("ziel-ordner");

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

    let ins_leere = ordner.verknuepfung("ins-leere", ordner.unter("gibt-es-nicht"));
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
/// # Der Nachweis ist am 260810 neu geschnitten worden
///
/// Bis dahin hing er an den Rechten: zwei Loecher, gleich angelegt, beide mit
/// Rechten 000, um genau ein Byte verschieden. Das ueber der Grenze kam als
/// `ZuGross` zurueck, das auf der Grenze als Lesefehler, und der Unterschied
/// zeigte, dass die Groessenpruefung vor dem Oeffnen lag.
///
/// Genau davor liegt sie nicht mehr. Seit `oeffnen` **zuerst oeffnet** und danach
/// am Deskriptor prueft, scheitern beide gesperrten Dateien schon am Oeffnen, und
/// die Rechte trennen nichts mehr. Ein Recht, das ein Lesen ohne ein Oeffnen
/// verbietet, gibt es auch nicht: POSIX prueft das Leserecht beim `open`. Der
/// alte Schnitt war damit nicht ungenau, sondern gegenstandslos.
///
/// # Was diese Probe jetzt belegt, und was die Nachbarin belegt
///
/// Hier steht der deterministische Teil: **der eine Byte Unterschied
/// entscheidet, und unterhalb der Grenze wird wirklich gelesen.**
///
/// ```text
///   EDITORGRENZE + 1 Bytes  ──> ZuGross, und der Wert traegt die Groesse
///   EDITORGRENZE     Bytes  ──> angenommen, 16 MB Stand, also gelesen
/// ```
///
/// Die zweite Zeile ist die tragende: ohne sie koennte die erste auch von einer
/// Funktion kommen, die ueberhaupt nichts liest.
///
/// **Dass im Fall `ZuGross` keine Bytes flossen, belegt sie nicht**, und das ist
/// keine Nachlaessigkeit, sondern eine Eigenschaft der Bauart: die Schranke
/// `take(EDITORGRENZE + 1)` liefert auch dann `ZuGross`, wenn erst gelesen und
/// dann geprueft wuerde. Im **Ergebnis** sind die beiden Reihenfolgen nicht zu
/// unterscheiden, allein im **Aufwand**. Diesen Teil traegt
/// `zwei_gigabyte_werden_ohne_arbeitsspeicher_abgewiesen` weiter unten. Die
/// beiden Proben zusammen sind der Nachweis von S10, und keine von beiden ist es
/// allein.
#[test]
fn eine_datei_ueber_der_grenze_wird_abgewiesen_ohne_gelesen_zu_werden() {
    let ordner = Pruefordner::neu("oeffnen-grenze");

    let ueber_der_grenze = ordner.luecke("zu-gross.log", datei::EDITORGRENZE + 1);
    let auf_der_grenze = ordner.luecke("gerade-noch.log", datei::EDITORGRENZE);

    assert_eq!(
        datei::oeffnen(&ueber_der_grenze),
        Err(Abweisung::ZuGross {
            pfad: ueber_der_grenze.clone(),
            groesse: datei::EDITORGRENZE + 1,
        }),
        "die zu grosse Datei kam nicht als zu gross zurueck"
    );

    let stand = datei::oeffnen(&auf_der_grenze)
        .expect("die Datei auf der Grenze gehoert gelesen, sonst belegt die Probe nichts");
    assert_eq!(
        stand.len() as u64,
        datei::EDITORGRENZE,
        "die Datei auf der Grenze wurde nicht vollstaendig gelesen"
    );
}

/// Fall 10, zweiter Teil: eine benannte Roehre haelt das Oeffnen nicht an.
///
/// Die Roehre hat keinen Schreiber, und ein `File::open` darauf wartet, bis
/// jemand hineinschreibt — hier also fuer immer. Dass die Probe mit einem Befund
/// endet und nicht mit Stillstand, besorgt [`oeffnen_mit_zeitschranke`].
///
/// **Zwei Aussagen, und die zweite ist die kleinere.** Abgewiesen wird die Roehre
/// mit dem Satz "das ist keine gewoehnliche Datei", und dass dieser Satz auf eine
/// Roehre wirklich greift, war bis zum 260810 ungeprueft: `tests/text.rs` deckte
/// den Ordner ab und die Verknuepfung, aber kein Ding, das sich oeffnen laesst
/// und doch keine Datei ist. Dass die Antwort ueberhaupt kommt, ist die groessere
/// Aussage, und sie haengt am `O_NONBLOCK` in
/// `verzeichnis::sys::ohne_warten_oeffnen`. Wer es dort herausnimmt, sieht diese
/// Probe stehen bleiben, statt einen Zweig weniger zu treffen.
#[test]
fn eine_benannte_roehre_wird_abgewiesen_und_haelt_das_oeffnen_nicht_an() {
    let ordner = Pruefordner::neu("oeffnen-roehre");
    let roehre = ordner.roehre("ohne-schreiber");

    let ergebnis = oeffnen_mit_zeitschranke(&roehre, Duration::from_secs(5));

    assert!(
        matches!(&ergebnis, Err(Abweisung::KeinGueltigesZiel { .. })),
        "die Roehre kam nicht als kein gueltiges Ziel zurueck: {ergebnis:?}"
    );
    let meldung = ergebnis.unwrap_err().meldung();
    assert!(
        meldung.contains("keine gewöhnliche Datei"),
        "die Meldung nennt den Grund nicht: {meldung}"
    );
}

/// Eine gesperrte Datei scheitert am Oeffnen, und der Systemfehler kommt mit.
///
/// Diese Probe steht hier, weil der alte Schnitt von Fall 10 sie beilaeufig
/// mitbelegt hat: dort waren beide Dateien mit Rechten 000 angelegt. Nach dem
/// Umbau auf den Deskriptor liegt das fehlende Leserecht **vor** jeder Pruefung,
/// und dass es dann als `KeinGueltigesZiel` mit dem Grund des Systems ankommt und
/// nicht verschluckt wird, gehoert weiter geprueft.
#[test]
fn eine_gesperrte_datei_kommt_mit_dem_systemfehler_zurueck() {
    use std::os::unix::fs::PermissionsExt;

    let ordner = Pruefordner::neu("oeffnen-gesperrt");
    let gesperrt = ordner.datei("verschlossen.txt", b"eins\n");
    fs::set_permissions(&gesperrt, fs::Permissions::from_mode(0o000))
        .expect("Rechte lassen sich nicht setzen");

    if fs::read(&gesperrt).is_ok() {
        // Unter root liest sich auch eine gesperrte Datei. Dann sagt die Probe
        // nichts aus, und eine Probe, die nichts aussagt, behauptet hier auch
        // nichts.
        eprintln!("uebersprungen: die Rechtesperre wirkt auf dieser Kennung nicht");
        return;
    }

    let ergebnis = datei::oeffnen(&gesperrt);
    let Err(Abweisung::KeinGueltigesZiel { grund, .. }) = &ergebnis else {
        panic!("die gesperrte Datei kam nicht als kein gueltiges Ziel zurueck: {ergebnis:?}");
    };
    assert!(
        !grund.is_empty(),
        "der Grund ist leer, der Systemfehler ist unterwegs verlorengegangen"
    );
}

/// Fall 10, dritter Teil: der Pfad wechselt seine Art, waehrend geoeffnet wird.
///
/// Das ist der Defekt `260809-1652` in seiner eigentlichen Gestalt. Die alte
/// Fassung fragte `stat(2)` auf den Pfad und oeffnete danach denselben Pfad ein
/// zweites Mal; wurde er in dieser Spanne gegen eine benannte Roehre getauscht,
/// hing das Oeffnen. Ein Faden tauscht hier genau das, waehrend ein zweiter
/// [`datei::oeffnen`] ruft, und zwar tausendfach.
///
/// # Der Tausch ist nachgemessen und nicht nur gemeint
///
/// Die alte Fassung von `oeffnen` faellt an dieser Probe wirklich aus: mit der
/// alten Reihenfolge eingesetzt bleibt der Lesefaden in `File::open` an der
/// Roehre stehen und die Zeitschranke unten schlaegt zu (am 260810 gemessen, ein
/// Lauf). Das ist der Grund, aus dem der Tauscher so aussieht, wie er aussieht,
/// und nicht einfacher:
///
/// - **Der umkaempfte Pfad ist zu jedem Zeitpunkt vorhanden**, einmal als Datei
///   und einmal als Roehre. Ein Tauscher, der wegbenennt und zuruecklegt, laesst
///   den Pfad die meiste Zeit **fehlen**; der Lesefaden sieht dann grosse Teile
///   der Zeit gar nichts, und das Fenster zwischen Pruefung und Oeffnen wird nie
///   getroffen. So gebaut lief die Probe auch unter der alten Fassung durch und
///   belegte nichts.
/// - Gelegt wird ueber eine **harte Verknuepfung auf eine Vorlage** und ein
///   `rename` darueber. `rename` ersetzt in einem Zug, und die Vorlage bleibt
///   liegen, statt mitzuwandern.
/// - **`rename` auf denselben Inode tut nichts und meldet Erfolg** (so schreibt
///   POSIX es vor). Deshalb wechseln Datei und Roehre streng ab, und deshalb
///   raeumt die Schleife ihre Zwischenverknuepfung vorher weg: bliebe sie
///   liegen, scheiterte die naechste Verknuepfung, und der Tauscher hoerte nach
///   einem Tausch auf.
///
/// Genau das faengt die Zaehlung unten ab. **Ein Tauscher, der still aufhoert,
/// laesst die Probe nicht durchlaufen, sondern ausfallen** — sonst waere das die
/// Sorte Probe, die gruen ist, weil sie nichts mehr tut.
///
/// # Der Lesefaden wartet auf den Tauscher und nicht auf eine Zahl
///
/// Die Schleife unten laeuft, bis **beides** erreicht ist: die Zahl der
/// Durchlaeufe und die Zahl der Tausche. Eine feste Zahl von Durchlaeufen allein
/// genuegt nicht, und das ist gemessen und nicht befuerchtet: im Profil `release`
/// ist der Lesefaden so viel schneller, dass er seine 20.000 Durchlaeufe hinter
/// sich hat, bevor der Tauscher tausend Tausche geschafft hat (am 260810
/// beobachtet: 994). Die Probe waere dann im einen Profil aussagekraeftig und im
/// anderen nicht. Mit der Kopplung an den Zaehler richtet sie sich nach dem
/// langsameren der beiden, was auch immer das auf dem Geraet ist.
///
/// Die Obergrenze daneben ist die Notbremse: haelt der Tauscher an, laeuft der
/// Lesefaden nicht ewig, sondern kommt zurueck und laesst die Zaehlung ausfallen.
///
/// # Was die Probe zusagt und was nicht
///
/// - **Unter der heutigen Bauart kann sie nicht ausfallen.** Es gibt nur noch
///   einen Aufruf, der den Namen aufloest, also kein Fenster, in dem er sich
///   aendern koennte. Jede Antwort ist entweder der Inhalt der Datei oder eine
///   Abweisung mit Grund, und beides kommt sofort.
/// - **Unter der alten Bauart faellt sie mit hoher Wahrscheinlichkeit aus, nicht
///   mit Sicherheit.** Ob ein Tausch in das schmale Fenster faellt, entscheidet
///   der Ablaufplaner; erzwingen laesst sich ein Wettrennen von aussen nicht.
///   Deshalb stehen beide Zahlen hoch.
///
/// `NichtAlsTextLesbar` und `ZuGross` kommen nicht vor: die Vorlage traegt fuenf
/// Bytes gueltiges UTF-8, und die Roehre faellt am Typ heraus, bevor irgendetwas
/// gelesen wird.
#[test]
fn ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an() {
    // Wie oft mindestens geoeffnet und wie oft mindestens getauscht wird; die
    // Schleife laeuft, bis beides erreicht ist. Die dritte Zahl ist die
    // Notbremse fuer den Fall, dass der Tauscher stehen bleibt.
    const DURCHLAEUFE: usize = 20_000;
    const MINDESTENS_GETAUSCHT: u64 = 2_000;
    const HOECHSTENS_DURCHLAEUFE: usize = 10 * DURCHLAEUFE;

    let ordner = Pruefordner::neu("oeffnen-wechsel");
    let vorlage_datei = ordner.datei("vorlage.txt", b"eins\n");
    let vorlage_roehre = ordner.roehre("vorlage.roehre");
    let zwischen = ordner.unter("zwischen");
    let umkaempft = ordner.unter("umkaempft");
    fs::hard_link(&vorlage_roehre, &umkaempft).expect("der Anfangsstand laesst sich nicht legen");

    let schluss = Arc::new(AtomicBool::new(false));
    let getauscht = Arc::new(AtomicU64::new(0));
    let tauscher = {
        let schluss = Arc::clone(&schluss);
        let getauscht = Arc::clone(&getauscht);
        let umkaempft = umkaempft.clone();
        std::thread::spawn(move || {
            while !schluss.load(Ordering::Relaxed) {
                for vorlage in [&vorlage_datei, &vorlage_roehre] {
                    let _ = fs::remove_file(&zwischen);
                    if fs::hard_link(vorlage, &zwischen).is_err()
                        || fs::rename(&zwischen, &umkaempft).is_err()
                    {
                        // Der Ordner ist abgeraeumt oder der Lesefaden fertig.
                        // Wie weit es gekommen ist, steht im Zaehler.
                        return;
                    }
                    getauscht.fetch_add(1, Ordering::Relaxed);
                }
            }
        })
    };

    let (sender, empfaenger) = mpsc::channel();
    let leserpfad = umkaempft.clone();
    let mitgezaehlt = Arc::clone(&getauscht);
    std::thread::spawn(move || {
        let mut unerwartet: Vec<String> = Vec::new();
        let mut gelaufen = 0usize;
        while gelaufen < DURCHLAEUFE
            || (mitgezaehlt.load(Ordering::Relaxed) < MINDESTENS_GETAUSCHT
                && gelaufen < HOECHSTENS_DURCHLAEUFE)
        {
            gelaufen += 1;
            match datei::oeffnen(&leserpfad) {
                Ok(_) | Err(Abweisung::KeinGueltigesZiel { .. }) => {}
                andere => unerwartet.push(format!("{andere:?}")),
            }
        }
        let _ = sender.send((gelaufen, unerwartet));
    });

    let ergebnis = empfaenger.recv_timeout(Duration::from_secs(15));
    schluss.store(true, Ordering::Relaxed);
    let _ = tauscher.join();
    let gelaufene_tausche = getauscht.load(Ordering::Relaxed);

    match ergebnis {
        Ok((gelaufen, unerwartet)) => assert!(
            unerwartet.is_empty(),
            "{} von {gelaufen} Durchlaeufen kamen mit einer unerwarteten Antwort zurueck: {unerwartet:?}",
            unerwartet.len()
        ),
        Err(_) => panic!(
            "die Durchlaeufe sind nach 15 Sekunden nicht fertig geworden; \
             das Oeffnen haengt an der benannten Roehre"
        ),
    }
    assert!(
        gelaufene_tausche >= MINDESTENS_GETAUSCHT,
        "der Tauscher ist nach {gelaufene_tausche} Tauschen stehen geblieben; \
         unter {MINDESTENS_GETAUSCHT} belegt die Probe kein Wettrennen mehr"
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
/// Zwei Gigabyte als Loch: das ist die Protokolldatei aus dem Datensatz.
///
/// **Diese Probe traegt seit dem 260810 den Nachweis "ohne gelesen zu werden".**
/// Fall 10 hat ihn abgegeben, weil er dort an den Rechten hing und der Umbau auf
/// den Deskriptor sie gegenstandslos gemacht hat; im **Ergebnis** liefern "vor
/// dem Lesen geprueft" und "nach dem Lesen geprueft" beide `ZuGross`, denn die
/// Schranke `take(EDITORGRENZE + 1)` faengt auch den zweiten Fall. Beobachtbar
/// ist allein der **Aufwand**, und deshalb ist die Zeit hier kein Beiwerk,
/// sondern der Beleg.
///
/// # Gemessen wird gegen die Maschine und nicht gegen eine Zahl
///
/// ```text
///   16 MB Loch, vollstaendig gelesen  ──> die Messlatte
///    2 GB Loch, abgewiesen            ──> muss darunter bleiben, mit Luft
/// ```
///
/// Zwei Gigabyte sind das 128-fache von 16 MB. Wer sie liest, braucht mindestens
/// das 128-fache der Messlatte, dazu zwei Gigabyte Arbeitsspeicher. Die Schranke
/// unten laesst das Achtfache zu; ein gelesenes Gigabytepaar waere damit um den
/// Faktor 16 ausgeschlossen, und die Rechnung lebt nicht von der Geschwindigkeit
/// des Geraets. Die Fassung bis zum 260810 stand bei einer halben Sekunde
/// absolut, also bei einer Zahl, die auf einem langsamen Geraet zu knapp und auf
/// einem schnellen zu weit ist.
#[test]
fn zwei_gigabyte_werden_ohne_arbeitsspeicher_abgewiesen() {
    use std::time::Instant;

    let ordner = Pruefordner::neu("oeffnen-riesig");
    let messlatte = ordner.luecke("messlatte.log", datei::EDITORGRENZE);
    let riesig = ordner.luecke("protokoll.log", 2 * 1024 * 1024 * 1024);

    let begonnen = Instant::now();
    let stand = datei::oeffnen(&messlatte).expect("das Loch auf der Grenze gehoert gelesen");
    let gelesen = begonnen.elapsed();
    assert_eq!(stand.len() as u64, datei::EDITORGRENZE);
    drop(stand);

    let begonnen = Instant::now();
    let ergebnis = datei::oeffnen(&riesig);
    let abgewiesen = begonnen.elapsed();

    assert_eq!(
        ergebnis,
        Err(Abweisung::ZuGross {
            pfad: riesig.clone(),
            groesse: 2 * 1024 * 1024 * 1024,
        })
    );
    assert!(
        abgewiesen < gelesen * 8,
        "die Abweisung der zwei Gigabyte hat {abgewiesen:?} gebraucht, \
         das vollstaendige Lesen von 16 MB nur {gelesen:?}; das riecht nach gelesenen Bytes"
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
    let unterordner = ordner.ordner("ein-ordner");
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

/// Der eine Befund geht ueber alle vier Ausgaenge, und der unlesbare traegt
/// seinen Deskriptor zurueckgespult.
///
/// **Der Befund ist die Stelle, an der `oeffnen` und der Notizzettel sich
/// treffen** (Runde 9). Der Editor uebersetzt ihn in eine [`Abweisung`] und
/// wirft die Bytes weg; `ablage::Zugang::text_laden` legt sie beiseite. Was
/// dieser zweite Aufrufer braucht und die `Abweisung` nicht traegt, ist der
/// offene Deskriptor — und zwar am Anfang.
///
/// **Zurueckgespult wird auch dort, wo es nicht noetig scheint.** Der Fall "zu
/// gross" kehrt in `lesen` zurueck, bevor gelesen wird, und stuende ohnehin am
/// Anfang; der Fall "kein Text" steht hinter einem `read_to_end` und stuende es
/// nicht. Die Probe fragt beide, weil eine Regel, die nur an einer Stelle gilt,
/// beim naechsten Umbau an der anderen vergessen wird.
///
/// Der fuenfte Ausgang, den es nicht gibt: eine fehlende Datei ist kein eigener
/// Wert, sondern das Feld `fehlt` an `KeinGueltigesZiel`. Die Probe haelt beide
/// Haelften fest, denn allein daran haengt die Zusage, dass ein fehlender
/// Zettel keine Meldung nach sich zieht.
#[test]
fn der_befund_deckt_alle_vier_ausgaenge_und_spult_zurueck() {
    let ordner = Pruefordner::neu("befund");

    // 1. Gueltiges UTF-8 unter der Grenze.
    let text = ordner.datei("zettel.txt", b"erste\nzweite");
    match datei::lesen(&text) {
        Textstand::Text(stand) => assert_eq!(stand, "erste\nzweite"),
        anderes => panic!("die Textdatei kam als {anderes:?} zurueck"),
    }

    // 2. Kein gueltiges UTF-8. Der Deskriptor steht am Anfang, obwohl vorher
    //    gelesen wurde, und liefert die Bytes vollstaendig ein zweites Mal.
    let roh: &[u8] = b"noch lesbar\n\xff\xfe und ab hier nicht mehr";
    let binaer = ordner.datei("bild.png", roh);
    match datei::lesen(&binaer) {
        Textstand::Unlesbar {
            mut datei,
            grund: Unlesbarkeit::KeinText,
        } => {
            assert_eq!(
                datei.stream_position().expect("keine Stelle"),
                0,
                "der Deskriptor steht nicht am Anfang"
            );
            let mut bytes = Vec::new();
            datei.read_to_end(&mut bytes).expect("lesen gescheitert");
            assert_eq!(bytes, roh, "aus dem Deskriptor kam ein Rumpf");
        }
        anderes => panic!("die Binaerdatei kam als {anderes:?} zurueck"),
    }

    // 3. Ueber der Grenze. Gelesen wird sie nicht, zurueckgespult trotzdem.
    let gross = ordner.luecke("zu-gross.log", datei::EDITORGRENZE + 1);
    match datei::lesen(&gross) {
        Textstand::Unlesbar {
            mut datei,
            grund: Unlesbarkeit::ZuGross(groesse),
        } => {
            assert_eq!(groesse, datei::EDITORGRENZE + 1);
            assert_eq!(
                datei.stream_position().expect("keine Stelle"),
                0,
                "der Deskriptor steht nicht am Anfang"
            );
            assert_eq!(
                datei.seek(SeekFrom::End(0)).expect("kein Ende"),
                datei::EDITORGRENZE + 1,
                "der Deskriptor zeigt nicht auf die ganze Datei"
            );
        }
        anderes => panic!("die zu grosse Datei kam als {anderes:?} zurueck"),
    }

    // 4. Kein gueltiges Ziel, in beiden Haelften: ein Ordner steht da, eine
    //    fehlende Datei steht nicht da.
    let unterordner = ordner.ordner("ein-ordner");
    match datei::lesen(&unterordner) {
        Textstand::KeinGueltigesZiel { grund, fehlt } => {
            assert!(!fehlt, "ein Ordner gilt als fehlend");
            assert!(!grund.is_empty(), "der Grund ist leer");
        }
        anderes => panic!("der Ordner kam als {anderes:?} zurueck"),
    }
    match datei::lesen(&ordner.unter("gibt-es-nicht.txt")) {
        Textstand::KeinGueltigesZiel { fehlt, .. } => {
            assert!(fehlt, "die fehlende Datei gilt nicht als fehlend");
        }
        anderes => panic!("die fehlende Datei kam als {anderes:?} zurueck"),
    }
}
