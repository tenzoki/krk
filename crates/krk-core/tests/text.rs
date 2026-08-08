//! Die Textrechnung des Editors: Zeilenindex, Suche und Ersetzen (C5, C6,
//! Schritt 8).
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
//! **Keine Datei und kein Pruefordner.** Die Textrechnung liegt im Kern, weil
//! sie auf Zeichenketten rechnet; das Einlesen und das Sichern, die das
//! Dateisystem anfassen, sind ein anderer Schritt. Die fuenf Faelle laufen
//! deshalb ohne eine einzige angelegte Datei.

use krk_core::text::{Zeilenindex, Zeilenlage, Zeilensprung, suche};

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
