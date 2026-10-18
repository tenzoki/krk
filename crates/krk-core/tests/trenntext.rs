//! Abnahme des Kerns `krk_core::trenntext`, Schritt 1 des Plans
//! `261018-0822_*_plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md` im
//! Arbeitspaket `261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor`.
//!
//! Ohne Fenster und ohne Datei: jede Probe erzeugt ihren Text im Speicher.
//! Die Proben zu den Handlungen vergleichen Bytes ausserhalb des beruehrten
//! Bereichs und nicht nur Zellwerte; die zufaelligen Tabellen kommen aus einem
//! eigenen Zufallsgenerator mit festem Startwert, damit ein Fehler wiederholbar
//! bleibt.

use std::ffi::OsStr;
use std::time::Instant;

use krk_core::heimordner::eintraege::Richtung;
use krk_core::trenntext::{
    Art, Dialekt, Feldform, HOECHSTENS_SPALTEN, Herkunft, LetzteSpalte, Satzindex, Trennzeichen,
    Zellbereich, bereich_als_tsv, bereich_einsetzen, dialekt_erkennen, feld_kodieren, indexieren,
    spalte_einfuegen, spalten_loeschen, tsv_lesen, vollstaendig_bis, zeile_anhaengen,
    zeilen_einfuegen, zeilen_loeschen, zeilen_verschieben, zelle_setzen,
};

fn dialekt(trennzeichen: Trennzeichen) -> Dialekt {
    Dialekt {
        trennzeichen,
        herkunft: Herkunft::Gleichmass,
        sep_zeile: false,
    }
}

fn komma() -> Dialekt {
    dialekt(Trennzeichen::Komma)
}

/// Die Werte aller Zellen, je Satz.
fn werte(text: &str, index: &Satzindex) -> Vec<Vec<String>> {
    (0..index.zeilenzahl())
        .map(|satz| {
            index
                .felder(text, satz)
                .expect("der Satz steht im Index")
                .into_iter()
                .map(|feld| feld.wert.into_owned())
                .collect()
        })
        .collect()
}

fn lesen(text: &str, dialekt: &Dialekt) -> Vec<Vec<String>> {
    let index = indexieren(text, dialekt);
    werte(text, &index)
}

fn zeilen(zeilen: &[&[&str]]) -> Vec<Vec<String>> {
    zeilen
        .iter()
        .map(|zeile| zeile.iter().map(|wert| (*wert).to_owned()).collect())
        .collect()
}

/// Je Satz sein Inhalt und sein Zeilenende, wie sie im Text stehen.
fn roh_saetze(text: &str, index: &Satzindex) -> Vec<(String, String)> {
    let saetze = index.saetze();
    saetze
        .iter()
        .enumerate()
        .map(|(stelle, bereich)| {
            let naechster = saetze.get(stelle + 1).map_or(text.len(), |s| s.start);
            (
                text[bereich.clone()].to_owned(),
                text[bereich.end..naechster].to_owned(),
            )
        })
        .collect()
}

/// Je Feld eines Satzes seine rohen Bytes.
fn roh_felder(text: &str, index: &Satzindex, satz: usize) -> Vec<String> {
    index
        .felder(text, satz)
        .expect("der Satz steht im Index")
        .into_iter()
        .map(|feld| text[feld.roh].to_owned())
        .collect()
}

// ---------------------------------------------------------------- E1

#[test]
fn jede_endung_ergibt_ihre_art_und_keine_andere() {
    let tafel: [(&str, Option<Art>); 13] = [
        ("a.csv", Some(Art::Csv)),
        ("a.CSV", Some(Art::Csv)),
        ("Liste.Csv", Some(Art::Csv)),
        ("a.tsv", Some(Art::Tsv)),
        ("a.TSV", Some(Art::Tsv)),
        ("a.tab", Some(Art::Tsv)),
        ("a.Tab", Some(Art::Tsv)),
        ("a.csv.gz", None),
        ("a.txt", None),
        ("a.psv", None),
        ("csv", None),
        (".csv", None),
        ("a.csvx", None),
    ];
    for (name, erwartet) in tafel {
        assert_eq!(Art::aus_namen(OsStr::new(name)), erwartet, "{name}");
    }
}

// ---------------------------------------------------------------- E2

#[test]
fn das_trennzeichen_folgt_sep_zeile_gleichmass_und_reihenfolge() {
    // Eine deutsche Excel-Datei: Semikolon, Dezimalkomma.
    let excel = "Name;Preis;Menge\nÄpfel;1,50;3\nBirnen;2,25;10\nKiwi;0,5;1\n";
    let gewaehlt = dialekt_erkennen(Art::Csv, excel);
    assert_eq!(gewaehlt.trennzeichen, Trennzeichen::Semikolon);
    assert_eq!(gewaehlt.herkunft, Herkunft::Gleichmass);
    assert!(!gewaehlt.sep_zeile);

    // Dieselbe mit Bytefolgenmarke.
    let mit_marke = format!("\u{feff}{excel}");
    assert_eq!(
        dialekt_erkennen(Art::Csv, &mit_marke).trennzeichen,
        Trennzeichen::Semikolon
    );

    // Komma mit gequoteten Kommas und Semikolons darin.
    let gequotet = "a,b,c\n\"x, y\",\"1;2\",3\n\"p;q;r;s\",2,3\n";
    let gewaehlt = dialekt_erkennen(Art::Csv, gequotet);
    assert_eq!(gewaehlt.trennzeichen, Trennzeichen::Komma);
    assert_eq!(gewaehlt.herkunft, Herkunft::Gleichmass);

    // Gleichstand: beide tragen je Satz zwei Felder, Komma kommt zuerst.
    let gleich = "a,b;c\nd,e;f\n";
    assert_eq!(
        dialekt_erkennen(Art::Csv, gleich).trennzeichen,
        Trennzeichen::Komma
    );
    // Gleichstand zwischen Semikolon und Tabulator: Semikolon.
    let gleich = "a;b\tc\nd;e\tf\n";
    assert_eq!(
        dialekt_erkennen(Art::Csv, gleich).trennzeichen,
        Trennzeichen::Semikolon
    );

    // Tabulator, wenn allein er gleichmaessig teilt.
    let tab = "a\tb\tc\n1\t2\t3\n";
    assert_eq!(
        dialekt_erkennen(Art::Csv, tab).trennzeichen,
        Trennzeichen::Tabulator
    );

    // Eine Spalte: keine Regel greift, es gilt Komma.
    let eine = "Name\nÄpfel\nBirnen\n";
    let gewaehlt = dialekt_erkennen(Art::Csv, eine);
    assert_eq!(gewaehlt.trennzeichen, Trennzeichen::Komma);
    assert_eq!(gewaehlt.herkunft, Herkunft::Vorgabe);
    assert_eq!(dialekt_erkennen(Art::Csv, "").herkunft, Herkunft::Vorgabe);

    // `sep=;` gilt vor dem Gleichmass und ist Vorspann.
    let sep = "sep=;\na,b,c\nd,e,f\n";
    let gewaehlt = dialekt_erkennen(Art::Csv, sep);
    assert_eq!(gewaehlt.trennzeichen, Trennzeichen::Semikolon);
    assert_eq!(gewaehlt.herkunft, Herkunft::SepZeile);
    assert!(gewaehlt.sep_zeile);
    let index = indexieren(sep, &gewaehlt);
    assert_eq!(&sep[index.vorspann()], "sep=;\n");
    assert_eq!(
        werte(sep, &index),
        zeilen(&[&["a,b,c"], &["d,e,f"]]),
        "die sep-Zeile ist keine Zeile der Tabelle"
    );
    // Mit Marke und `\r\n` steht beides im Vorspann.
    let sep_crlf = "\u{feff}sep=\t\r\na\tb\r\n";
    let gewaehlt = dialekt_erkennen(Art::Csv, sep_crlf);
    assert_eq!(gewaehlt.trennzeichen, Trennzeichen::Tabulator);
    let index = indexieren(sep_crlf, &gewaehlt);
    assert_eq!(&sep_crlf[index.vorspann()], "\u{feff}sep=\t\r\n");
    assert_eq!(werte(sep_crlf, &index), zeilen(&[&["a", "b"]]));
    // `sep=` mit einem anderen Zeichen oder mit Text dahinter ist keine.
    assert_eq!(
        dialekt_erkennen(Art::Csv, "sep=|\na|b\n").herkunft,
        Herkunft::Vorgabe
    );
    assert_ne!(
        dialekt_erkennen(Art::Csv, "sep=;x\na,b\n").herkunft,
        Herkunft::SepZeile
    );

    // Eine `.tsv` fragt nichts davon und liest keine sep-Zeile.
    let gewaehlt = dialekt_erkennen(Art::Tsv, "sep=;\na;b\n");
    assert_eq!(gewaehlt, Dialekt::tsv());
    assert_eq!(gewaehlt.herkunft, Herkunft::Endung);
    let index = indexieren("sep=;\na;b\n", &gewaehlt);
    assert_eq!(index.zeilenzahl(), 2);

    // Der Wechsel geht reihum und behaelt den Vorspann.
    let gewechselt = dialekt_erkennen(Art::Csv, sep).gewechselt();
    assert_eq!(gewechselt.trennzeichen, Trennzeichen::Tabulator);
    assert_eq!(gewechselt.herkunft, Herkunft::Gewechselt);
    assert!(gewechselt.sep_zeile);
    assert_eq!(
        gewechselt.gewechselt().trennzeichen,
        Trennzeichen::Komma,
        "nach dem Tabulator kommt wieder das Komma"
    );
}

#[test]
fn eine_sep_zeile_entsteht_nicht_nachtraeglich_durch_eine_handlung() {
    // Die Datei begann ohne sep-Zeile; eine Zeile `sep=;`, die an den Anfang
    // wandert, bleibt eine Zeile.
    let text = "a\nsep=;\n";
    let gewaehlt = dialekt_erkennen(Art::Csv, text);
    assert!(!gewaehlt.sep_zeile);
    let index = indexieren(text, &gewaehlt);
    let stand = zeilen_verschieben(text, &index, &Zellbereich::zelle(1, 0), Richtung::Hoch)
        .expect("die zweite Zeile wandert nach oben");
    assert_eq!(stand.text, "sep=;\na\n");
    assert_eq!(indexieren(&stand.text, &gewaehlt).zeilenzahl(), 2);
}

// ---------------------------------------------------------------- E3

#[test]
fn rfc_4180_felder_werden_gelesen_wie_sie_geschrieben_sind() {
    // `""` im Feld.
    assert_eq!(
        lesen("\"er sagte \"\"hi\"\"\",b\n", &komma()),
        zeilen(&[&["er sagte \"hi\"", "b"]])
    );
    // Umbruch im Feld, auch als `\r\n`.
    assert_eq!(
        lesen("\"zwei\nzeilen\",x\n\"drei\r\nzeilen\",y\n", &komma()),
        zeilen(&[&["zwei\nzeilen", "x"], &["drei\r\nzeilen", "y"]])
    );
    // `\r\n` als Zeilenende, und `\n` daneben.
    assert_eq!(
        lesen("a,b\r\nc,d\ne,f\r\n", &komma()),
        zeilen(&[&["a", "b"], &["c", "d"], &["e", "f"]])
    );
    // Ein einzelnes `\r` ist ein Zeichen des Feldes.
    assert_eq!(
        lesen("a\rb,c\nd,e\r", &komma()),
        zeilen(&[&["a\rb", "c"], &["d", "e\r"]])
    );
    // Leere Felder und ein abschliessendes Trennzeichen.
    assert_eq!(
        lesen(",,\na,\n,b\n", &komma()),
        zeilen(&[&["", "", ""], &["a", ""], &["", "b"]])
    );
    // Eine Leerzeile in der Mitte ist ein Satz mit einem leeren Feld.
    assert_eq!(
        lesen("a,b\n\nc,d\n", &komma()),
        zeilen(&[&["a", "b"], &[""], &["c", "d"]])
    );
    // Dateiende ohne Umbruch, und das letzte Zeilenende eroeffnet keinen Satz.
    assert_eq!(
        lesen("a,b\nc,d", &komma()),
        zeilen(&[&["a", "b"], &["c", "d"]])
    );
    assert_eq!(lesen("a,b\n", &komma()).len(), 1);
    assert_eq!(lesen("a,b\r\n", &komma()).len(), 1);
    assert!(lesen("", &komma()).is_empty());
    assert_eq!(lesen("\n", &komma()), zeilen(&[&[""]]));
    // Gequotete leere Felder und Umlaute.
    assert_eq!(
        lesen("\"\",\"Äpfel\",ß\n", &komma()),
        zeilen(&[&["", "Äpfel", "ß"]])
    );
    // Das Semikolon trennt, das Komma nicht.
    assert_eq!(
        lesen("Äpfel;1,50\n", &dialekt(Trennzeichen::Semikolon)),
        zeilen(&[&["Äpfel", "1,50"]])
    );

    // Die Bytefolgenmarke ist Vorspann und kein Teil der ersten Zelle.
    let marke = "\u{feff}a,b\nc,d\n";
    let index = indexieren(marke, &komma());
    assert_eq!(&marke[index.vorspann()], "\u{feff}");
    assert_eq!(werte(marke, &index), zeilen(&[&["a", "b"], &["c", "d"]]));
    assert_eq!(indexieren("\u{feff}", &komma()).zeilenzahl(), 0);

    // Formen und Bereiche.
    let text = "glatt,\"gequotet\",x\"y\n";
    let index = indexieren(text, &komma());
    let felder = index.felder(text, 0).expect("Satz 0");
    let formen: Vec<Feldform> = felder.iter().map(|feld| feld.form).collect();
    assert_eq!(
        formen,
        [Feldform::Glatt, Feldform::Gequotet, Feldform::Nachsichtig]
    );
    assert_eq!(&text[felder[1].roh.clone()], "\"gequotet\"");
    assert_eq!(index.spaltenzahl(), 3);
    assert!(index.felder(text, 1).is_none());

    // Spaltenzahl ist die groesste Feldzahl.
    let index = indexieren("a\nb,c,d\ne,f\n", &komma());
    assert_eq!(index.spaltenzahl(), 3);
    assert_eq!(indexieren("", &komma()).spaltenzahl(), 1);
}

#[test]
fn nachsichtige_felder_bleiben_stehen_bis_ihre_zelle_geaendert_wird() {
    let text = "ab\"c,\"de\"f,\"g\"\"h\",x\n2,3,4,5\n";
    let index = indexieren(text, &komma());
    let felder = index.felder(text, 0).expect("Satz 0");
    assert_eq!(felder[0].form, Feldform::Nachsichtig);
    assert_eq!(
        felder[0].wert, "ab\"c",
        "ein `\"` mitten im Feld ist ein Zeichen"
    );
    assert_eq!(felder[1].form, Feldform::Nachsichtig);
    assert_eq!(
        felder[1].wert, "def",
        "der Text hinter dem `\"` wird angehaengt"
    );
    assert_eq!(felder[2].form, Feldform::Gequotet);
    assert_eq!(felder[2].wert, "g\"h");

    // Eine andere Zelle des Satzes zu aendern laesst die nachsichtigen stehen.
    let stand = zelle_setzen(text, &index, 0, 3, "y").expect("x wird y");
    assert_eq!(stand.text, "ab\"c,\"de\"f,\"g\"\"h\",y\n2,3,4,5\n");

    // Erst die eigene Aenderung schreibt kanonisch.
    let stand = zelle_setzen(text, &index, 0, 1, "de\"f").expect("die Zelle aendert sich");
    assert_eq!(stand.text, "ab\"c,\"de\"\"f\",\"g\"\"h\",x\n2,3,4,5\n");
    let stand = zelle_setzen(text, &index, 0, 0, "abc").expect("die Zelle aendert sich");
    assert_eq!(stand.text, "abc,\"de\"f,\"g\"\"h\",x\n2,3,4,5\n");

    // Derselbe Wert ist keine Handlung, auch nicht an einem nachsichtigen Feld.
    assert!(zelle_setzen(text, &index, 0, 1, "def").is_none());
}

#[test]
fn ein_unterminiertes_anfuehrungszeichen_wird_mit_seinem_satz_gemeldet() {
    let text = "a,b\nc,d\n\"offen,e\nf,g\n";
    let index = indexieren(text, &komma());
    assert_eq!(index.unterminiert(), Some(2));
    assert_eq!(index.zeilenzahl(), 3, "der offene Satz reicht bis zum Ende");
    assert_eq!(index.saetze()[2].end, text.len());

    // Ohne offenes Zeichen keine Meldung, auch bei `"` mitten im Feld.
    assert_eq!(indexieren("a\"b,c\n", &komma()).unterminiert(), None);
    assert_eq!(indexieren("\"a", &komma()).unterminiert(), Some(0));

    // Keine Handlung rechnet auf einem solchen Index.
    let alles = Zellbereich {
        zeilen: 0..1,
        spalten: 0..1,
    };
    assert!(zelle_setzen(text, &index, 0, 0, "z").is_none());
    assert!(zeile_anhaengen(text, &index).is_none());
    assert!(zeilen_einfuegen(text, &index, &alles).is_none());
    assert!(zeilen_loeschen(text, &index, &alles).is_none());
    assert!(spalte_einfuegen(text, &index, 0, &alles).is_none());
    assert_eq!(spalten_loeschen(text, &index, &alles), Ok(None));
    assert!(bereich_einsetzen(text, &index, 0, 0, &[vec!["z".to_owned()]]).is_none());

    // TSV aus der Zwischenablage mit offenem `"` wird roh gelesen.
    assert_eq!(
        tsv_lesen("\"a\tb\nc\td\n"),
        zeilen(&[&["\"a", "b"], &["c", "d"]])
    );
}

// ---------------------------------------------------------------- E5

#[test]
fn kodieren_und_lesen_kehren_einander_um() {
    let werte_liste: [&str; 14] = [
        "",
        "glatt",
        "Äpfel und Birnen",
        ",",
        ";",
        "\t",
        "\"",
        "\"\"",
        "a\nb",
        "a\r\nb",
        "a\rb",
        "  Leerzeichen  ",
        "\"am Anfang",
        "1,5;2\t3\"4\n5",
    ];
    for trennzeichen in Trennzeichen::ALLE {
        let trenn = (trennzeichen.byte() as char).to_string();
        // Je Wert ein Satz aus zwei Feldern, damit auch das leere Feld steht.
        let text: String = werte_liste
            .iter()
            .map(|wert| format!("{}{trenn}x\n", feld_kodieren(wert, trennzeichen)))
            .collect();
        let gelesen = lesen(&text, &dialekt(trennzeichen));
        assert_eq!(gelesen.len(), werte_liste.len(), "{trennzeichen:?}");
        for (wert, satz) in werte_liste.iter().zip(&gelesen) {
            assert_eq!(
                satz,
                &vec![(*wert).to_owned(), "x".to_owned()],
                "{trennzeichen:?}"
            );
        }
    }
    // Quotiert wird allein, was es braucht.
    assert_eq!(feld_kodieren("1,5", Trennzeichen::Semikolon), "1,5");
    assert_eq!(feld_kodieren("1,5", Trennzeichen::Komma), "\"1,5\"");
    assert_eq!(feld_kodieren("a\"b", Trennzeichen::Tabulator), "\"a\"\"b\"");
    assert_eq!(feld_kodieren("", Trennzeichen::Komma), "");
}

// ---------------------------------------------------------------- E7

#[test]
fn eine_zelle_jenseits_des_satzendes_fuellt_mit_trennzeichen_auf_und_ein_leerer_wert_aendert_nichts()
 {
    let text = "a,b,c\nd\ne,f\n";
    let index = indexieren(text, &komma());
    let stand = zelle_setzen(text, &index, 1, 2, "X").expect("die Zelle wird gesetzt");
    assert_eq!(stand.text, "a,b,c\nd,,X\ne,f\n");
    assert_eq!(stand.auswahl, Zellbereich::zelle(1, 2));
    // Auch hinter der Spaltenzahl.
    let stand = zelle_setzen(text, &index, 2, 4, "Y").expect("die Zelle wird gesetzt");
    assert_eq!(stand.text, "a,b,c\nd\ne,f,,,Y\n");
    // Ein leerer Wert hinter dem Ende aendert nichts.
    assert!(zelle_setzen(text, &index, 1, 2, "").is_none());
    assert!(zelle_setzen(text, &index, 1, 1, "").is_none());
    // Eine Zeile, die es nicht gibt, legt `zelle_setzen` nicht an.
    assert!(zelle_setzen(text, &index, 3, 0, "Z").is_none());
    // Am Ende ohne Zeilenende.
    let ohne = "a,b\nc";
    let index = indexieren(ohne, &komma());
    assert_eq!(
        zelle_setzen(ohne, &index, 1, 1, "d").expect("gesetzt").text,
        "a,b\nc,d"
    );
    // Ein letzter Satz, der leer wird, behaelt seine Zeile.
    let stand = zelle_setzen(ohne, &index, 1, 0, "").expect("c wird leer");
    assert_eq!(stand.text, "a,b\n\n");
    assert_eq!(indexieren(&stand.text, &komma()).zeilenzahl(), 2);
}

#[test]
fn spalte_einfuegen_anhaengen_und_loeschen_folgen_der_regel_der_fehlenden_felder() {
    // Saetze mit drei, einem, zwei und einem leeren Feld.
    let text = "a,b,c\nd\ne,f\n\n";
    let index = indexieren(text, &komma());
    let alle = Zellbereich {
        zeilen: 0..4,
        spalten: 0..1,
    };
    assert_eq!(index.spaltenzahl(), 3);

    // Einfuegen an 0: jeder Satz bekommt vorn ein leeres Feld.
    let stand = spalte_einfuegen(text, &index, 0, &alle).expect("eingefuegt");
    assert_eq!(stand.text, ",a,b,c\n,d\n,e,f\n,\n");
    assert_eq!(stand.auswahl.spalten, 0..1);
    // Einfuegen an 1: jeder Satz mit mindestens einem Feld, also jeder.
    let stand = spalte_einfuegen(text, &index, 1, &alle).expect("eingefuegt");
    assert_eq!(stand.text, "a,,b,c\nd,\ne,,f\n,\n");
    // Einfuegen an 2: nur Saetze mit mindestens zwei Feldern.
    let stand = spalte_einfuegen(text, &index, 2, &alle).expect("eingefuegt");
    assert_eq!(stand.text, "a,b,,c\nd\ne,f,\n\n");
    // Anhaengen ist Einfuegen an der Spaltenzahl: nur die laengsten Saetze.
    let stand = spalte_einfuegen(text, &index, 3, &alle).expect("angehaengt");
    assert_eq!(stand.text, "a,b,c,\nd\ne,f\n\n");
    assert_eq!(indexieren(&stand.text, &komma()).spaltenzahl(), 4);
    assert_eq!(stand.auswahl.spalten, 3..4);
    // Hinter der Spaltenzahl gibt es keine Stelle.
    assert!(spalte_einfuegen(text, &index, 4, &alle).is_none());

    // Loeschen an 0: wer mehr als null Felder hat, verliert das erste samt
    // dem folgenden Trennzeichen; ein Satz aus einem Feld wird leer.
    let bereich = |spalten: std::ops::Range<usize>| Zellbereich {
        zeilen: 0..1,
        spalten,
    };
    let stand = spalten_loeschen(text, &index, &bereich(0..1))
        .expect("nicht die letzte")
        .expect("geloescht");
    assert_eq!(stand.text, "b,c\n\nf\n\n");
    // Loeschen an 1: das zweite Feld samt folgendem, beim letzten Feld samt
    // vorigem Trennzeichen; kuerzere Saetze bleiben.
    let stand = spalten_loeschen(text, &index, &bereich(1..2))
        .expect("nicht die letzte")
        .expect("geloescht");
    assert_eq!(stand.text, "a,c\nd\ne\n\n");
    // Loeschen an 2.
    let stand = spalten_loeschen(text, &index, &bereich(2..3))
        .expect("nicht die letzte")
        .expect("geloescht");
    assert_eq!(stand.text, "a,b\nd\ne,f\n\n");
    assert_eq!(stand.auswahl.spalten, 1..2, "die vorige Spalte rueckt nach");
    // Zwei Spalten auf einmal.
    let stand = spalten_loeschen(text, &index, &bereich(0..2))
        .expect("nicht die letzte")
        .expect("geloescht");
    assert_eq!(stand.text, "c\n\n\n\n");
    let stand = spalten_loeschen(text, &index, &bereich(1..3))
        .expect("nicht die letzte")
        .expect("geloescht");
    assert_eq!(stand.text, "a\nd\ne\n\n");

    // Ein Satz aus einem Feld am Ende ohne Zeilenende bleibt eine Zeile.
    let ohne = "a,b\nc";
    let index = indexieren(ohne, &komma());
    let stand = spalten_loeschen(ohne, &index, &bereich(0..1))
        .expect("nicht die letzte")
        .expect("geloescht");
    assert_eq!(stand.text, "b\n\n");
    assert_eq!(indexieren(&stand.text, &komma()).zeilenzahl(), 2);
}

#[test]
fn die_letzte_spalte_wird_nicht_geloescht() {
    let text = "a\nb\n";
    let index = indexieren(text, &komma());
    assert_eq!(
        spalten_loeschen(text, &index, &Zellbereich::zelle(0, 0)),
        Err(LetzteSpalte)
    );
    let text = "a,b\nc\n";
    let index = indexieren(text, &komma());
    let alles = Zellbereich {
        zeilen: 0..2,
        spalten: 0..2,
    };
    assert_eq!(spalten_loeschen(text, &index, &alles), Err(LetzteSpalte));
    // Ueber die Spaltenzahl hinaus gewaehlt zaehlt wie bis zu ihr.
    let mehr = Zellbereich {
        zeilen: 0..1,
        spalten: 0..9,
    };
    assert_eq!(spalten_loeschen(text, &index, &mehr), Err(LetzteSpalte));
    // Eine leere Auswahl tut nichts.
    let leer = Zellbereich {
        zeilen: 0..1,
        spalten: 1..1,
    };
    assert_eq!(spalten_loeschen(text, &index, &leer), Ok(None));
}

#[test]
fn eine_neue_zeile_traegt_spaltenzahl_weniger_eins_trennzeichen() {
    let text = "a,b,c\nd\n";
    let index = indexieren(text, &komma());
    let stand = zeile_anhaengen(text, &index).expect("angehaengt");
    assert_eq!(stand.text, "a,b,c\nd\n,,\n");
    assert_eq!(stand.auswahl, Zellbereich::zelle(2, 0));

    // Ohne Zeilenende am Schluss kommt es vor der neuen Zeile dazu.
    let ohne = "a;b\nc;d";
    let index = indexieren(ohne, &dialekt(Trennzeichen::Semikolon));
    assert_eq!(
        zeile_anhaengen(ohne, &index).expect("angehaengt").text,
        "a;b\nc;d\n;\n"
    );
    // Ein Text mit `\r\n` bekommt `\r\n`.
    let crlf = "a\tb\r\nc\td";
    let index = indexieren(crlf, &dialekt(Trennzeichen::Tabulator));
    assert_eq!(
        zeile_anhaengen(crlf, &index).expect("angehaengt").text,
        "a\tb\r\nc\td\r\n\t\r\n"
    );
    // Ein leerer Text, nur eine Marke, nur eine sep-Zeile.
    let index = indexieren("", &komma());
    assert_eq!(zeile_anhaengen("", &index).expect("angehaengt").text, "\n");
    let marke = "\u{feff}";
    let index = indexieren(marke, &komma());
    assert_eq!(
        zeile_anhaengen(marke, &index).expect("angehaengt").text,
        "\u{feff}\n"
    );
    let sep = "sep=;";
    let gewaehlt = dialekt_erkennen(Art::Csv, sep);
    let index = indexieren(sep, &gewaehlt);
    assert_eq!(index.zeilenzahl(), 0);
    assert_eq!(
        zeile_anhaengen(sep, &index).expect("angehaengt").text,
        "sep=;\n\n"
    );

    // Einfuegen ueber der Auswahl: so viele wie sie Zeilen umfasst, nach der
    // sep-Zeile und nie davor.
    let text = "sep=;\na;b;c\nd;e;f\n";
    let gewaehlt = dialekt_erkennen(Art::Csv, text);
    let index = indexieren(text, &gewaehlt);
    let auswahl = Zellbereich {
        zeilen: 0..2,
        spalten: 1..2,
    };
    let stand = zeilen_einfuegen(text, &index, &auswahl).expect("eingefuegt");
    assert_eq!(stand.text, "sep=;\n;;\n;;\na;b;c\nd;e;f\n");
    assert_eq!(stand.auswahl, auswahl);
}

#[test]
fn zeilen_loeschen_und_verschieben_halten_das_dateiende() {
    let ohne = "a\nb\nc";
    let index = indexieren(ohne, &komma());
    let zeile = |z: usize| Zellbereich::zelle(z, 0);
    assert_eq!(
        zeilen_loeschen(ohne, &index, &zeile(2)).expect("weg").text,
        "a\nb"
    );
    assert_eq!(
        zeilen_loeschen(ohne, &index, &zeile(1)).expect("weg").text,
        "a\nc"
    );
    assert_eq!(
        zeilen_loeschen(ohne, &index, &zeile(0)).expect("weg").text,
        "b\nc"
    );
    let alle = Zellbereich {
        zeilen: 0..3,
        spalten: 0..1,
    };
    let stand = zeilen_loeschen(ohne, &index, &alle).expect("weg");
    assert_eq!(stand.text, "");
    assert_eq!(stand.auswahl.zeilen, 0..0);
    assert!(zeilen_loeschen(ohne, &index, &zeile(3)).is_none());

    // Ein leerer voriger Satz behaelt sein Zeilenende.
    let leer = "a\n\nc";
    let index = indexieren(leer, &komma());
    let stand = zeilen_loeschen(leer, &index, &zeile(2)).expect("weg");
    assert_eq!(stand.text, "a\n\n");
    assert_eq!(indexieren(&stand.text, &komma()).zeilenzahl(), 2);
    assert_eq!(stand.auswahl, zeile(1));

    // Verschieben: der letzte Satz bekommt ein Zeilenende, der neue letzte
    // verliert seins.
    let index = indexieren(ohne, &komma());
    let stand = zeilen_verschieben(ohne, &index, &zeile(2), Richtung::Hoch).expect("hoch");
    assert_eq!(stand.text, "a\nc\nb");
    assert_eq!(stand.auswahl, zeile(1));
    let stand = zeilen_verschieben(ohne, &index, &zeile(1), Richtung::Runter).expect("runter");
    assert_eq!(stand.text, "a\nc\nb");
    assert_eq!(stand.auswahl, zeile(2));
    assert!(zeilen_verschieben(ohne, &index, &zeile(0), Richtung::Hoch).is_none());
    assert!(zeilen_verschieben(ohne, &index, &zeile(2), Richtung::Runter).is_none());
    // Als Block.
    let block = Zellbereich {
        zeilen: 1..3,
        spalten: 0..1,
    };
    let stand = zeilen_verschieben(ohne, &index, &block, Richtung::Hoch).expect("hoch");
    assert_eq!(stand.text, "b\nc\na");
    assert_eq!(stand.auswahl.zeilen, 0..2);
    // Ein leerer Satz, der ans Ende ohne Zeilenende wandert, behaelt seine Zeile.
    let leer = "\na";
    let index = indexieren(leer, &komma());
    let stand = zeilen_verschieben(leer, &index, &zeile(0), Richtung::Runter).expect("runter");
    assert_eq!(stand.text, "a\n\n");
    assert_eq!(indexieren(&stand.text, &komma()).zeilenzahl(), 2);
    // CRLF-Zeilenenden bleiben an ihren Stellen.
    let crlf = "a\r\nb\r\nc\r\n";
    let index = indexieren(crlf, &komma());
    let stand = zeilen_verschieben(crlf, &index, &zeile(0), Richtung::Runter).expect("runter");
    assert_eq!(stand.text, "b\r\na\r\nc\r\n");
}

// ---------------------------------------------------------------- E11, E12

#[test]
fn ein_bereich_als_tsv_und_zurueck_ergibt_dieselben_zellen() {
    let text =
        "Name;Notiz;Preis\nÄpfel;\"rot\tund\ngrün\";1,50\nBirnen;\"sagt \"\"hi\"\"\"\nKiwi\n";
    let index = indexieren(text, &dialekt(Trennzeichen::Semikolon));
    let bereich = Zellbereich {
        zeilen: 0..4,
        spalten: 0..3,
    };
    let tsv = bereich_als_tsv(text, &index, &bereich);
    assert_eq!(
        tsv,
        "Name\tNotiz\tPreis\nÄpfel\t\"rot\tund\ngrün\"\t1,50\nBirnen\t\"sagt \"\"hi\"\"\"\t\nKiwi\t\t"
    );
    let zurueck = tsv_lesen(&tsv);
    let erwartet = zeilen(&[
        &["Name", "Notiz", "Preis"],
        &["Äpfel", "rot\tund\ngrün", "1,50"],
        &["Birnen", "sagt \"hi\"", ""],
        &["Kiwi", "", ""],
    ]);
    assert_eq!(zurueck, erwartet);

    // Eine einzelne Zelle ist ihr blosser Wert.
    assert_eq!(
        bereich_als_tsv(text, &index, &Zellbereich::zelle(1, 2)),
        "1,50"
    );
    // Zeilen hinter der letzten fallen weg.
    let ueber = Zellbereich {
        zeilen: 3..9,
        spalten: 0..1,
    };
    assert_eq!(bereich_als_tsv(text, &index, &ueber), "Kiwi");
    // Ein abschliessendes Zeilenende eroeffnet beim Lesen keine Zeile, `\r\n`
    // aus Excel ebenso wenig.
    assert_eq!(tsv_lesen("a\tb\n"), zeilen(&[&["a", "b"]]));
    assert_eq!(
        tsv_lesen("a\tb\r\nc\td\r\n"),
        zeilen(&[&["a", "b"], &["c", "d"]])
    );
}

#[test]
fn ein_block_ueberschreibt_haengt_an_und_fuellt_auf_in_einer_handlung() {
    let text = "a,b\nc\n";
    let index = indexieren(text, &komma());
    let block = tsv_lesen("1\t2\t3\n4\t5\t6\n7\t8\t9");
    let stand = bereich_einsetzen(text, &index, 1, 1, &block).expect("eingesetzt");
    assert_eq!(stand.text, "a,b\nc,1,2,3\n,4,5,6\n,7,8,9\n");
    assert_eq!(
        stand.auswahl,
        Zellbereich {
            zeilen: 1..4,
            spalten: 1..4,
        }
    );
    // Ab der Zeilenzahl haengt er allein an, dahinter gibt es keine Stelle.
    let stand = bereich_einsetzen(text, &index, 2, 0, &tsv_lesen("x,y")).expect("angehaengt");
    assert_eq!(stand.text, "a,b\nc\n\"x,y\",\n");
    assert!(bereich_einsetzen(text, &index, 3, 0, &block).is_none());
    // Derselbe Block noch einmal ist keine Handlung.
    let gleich = tsv_lesen("a\tb");
    assert!(bereich_einsetzen(text, &index, 0, 0, &gleich).is_none());
    assert!(bereich_einsetzen(text, &index, 0, 0, &[]).is_none());
}

// ---------------------------------------------------------------- E17

#[test]
fn die_kuerzung_endet_am_letzten_vollstaendigen_satz() {
    let text = "a,b\nc,\"zwei\nzeilen\"\ne,f\n";
    let bytes = text.as_bytes();
    // Grenze mitten im gequoteten Umbruch: die Tabelle endet nach Satz 0.
    let mitten = text.find("zeilen").expect("steht im Text");
    assert_eq!(vollstaendig_bis(&bytes[..mitten], Trennzeichen::Komma), 4);
    // Grenze direkt nach dem Umbruch im Feld: immer noch Satz 0.
    let nach_umbruch = text.find("zwei\n").expect("steht im Text") + 5;
    assert_eq!(
        vollstaendig_bis(&bytes[..nach_umbruch], Trennzeichen::Komma),
        4
    );
    // Grenze hinter dem Zeilenende von Satz 1.
    let nach_satz = text.find("e,f").expect("steht im Text");
    assert_eq!(
        vollstaendig_bis(&bytes[..nach_satz], Trennzeichen::Komma),
        nach_satz
    );
    // Ganz und ohne vollstaendigen Satz.
    assert_eq!(vollstaendig_bis(bytes, Trennzeichen::Komma), text.len());
    assert_eq!(vollstaendig_bis(b"abc", Trennzeichen::Komma), 0);
    assert_eq!(vollstaendig_bis(b"", Trennzeichen::Komma), 0);
    // Ein Schnitt mitten in einem Zeichen: das Ergebnis liegt dahinter nicht.
    let umlaut = "a\nÄpfel".as_bytes();
    assert_eq!(vollstaendig_bis(&umlaut[..3], Trennzeichen::Komma), 2);
    // `\r\n` gilt als Zeilenende.
    assert_eq!(vollstaendig_bis(b"a\r\nb", Trennzeichen::Komma), 3);
}

// ---------------------------------------------------------------- E9

#[test]
fn die_spaltengrenze_ist_eine_konstante_und_der_leser_kennt_sie_nicht() {
    let breit: String = (0..=HOECHSTENS_SPALTEN)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let index = indexieren(&breit, &komma());
    assert_eq!(index.spaltenzahl(), HOECHSTENS_SPALTEN + 1);
}

// ---------------------------------------------------------------- E5, Zufall

/// Ein eigener Zufallsgenerator mit festem Startwert (xorshift64*), damit die
/// Probe keine Kiste braucht und ein Fehler wiederholbar bleibt.
struct Zufall(u64);

impl Zufall {
    fn naechste(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn bis(&mut self, grenze: usize) -> usize {
        (self.naechste() % grenze as u64) as usize
    }
}

/// Eine Tabelle mit ungleich langen Saetzen, glatten, gequoteten und
/// nachsichtigen Feldern, gemischten Zeilenenden und einem Dateiende mit oder
/// ohne Zeilenende.
fn zufallstabelle(zufall: &mut Zufall) -> String {
    const GLATT: [&str; 6] = ["a", "bc", "Äpfel", "1.5", "x y", ""];
    const SCHWIERIG: [&str; 6] = [
        "a,b",
        "er sagte \"hi\"",
        "zwei\nzeilen",
        "cr\rin",
        "\"",
        "1,\"2\"\n3",
    ];
    const NACHSICHTIG: [&str; 3] = ["ab\"c", "\"ab\"c", "x\"\"y"];
    let saetze = 1 + zufall.bis(12);
    let mut text = String::new();
    for satz in 0..saetze {
        let felder = 1 + zufall.bis(5);
        let mut teile = Vec::new();
        for _ in 0..felder {
            let feld = match zufall.bis(4) {
                0 | 1 => GLATT[zufall.bis(GLATT.len())].to_owned(),
                2 => feld_kodieren(SCHWIERIG[zufall.bis(SCHWIERIG.len())], Trennzeichen::Komma)
                    .into_owned(),
                _ => NACHSICHTIG[zufall.bis(NACHSICHTIG.len())].to_owned(),
            };
            teile.push(feld);
        }
        let inhalt = teile.join(",");
        text.push_str(&inhalt);
        let letzter = satz + 1 == saetze;
        // Ein leerer letzter Satz braucht sein Zeilenende, sonst waere er
        // keiner.
        if letzter && !inhalt.is_empty() && zufall.bis(2) == 0 {
            continue;
        }
        text.push_str(if zufall.bis(3) == 0 { "\r\n" } else { "\n" });
    }
    text
}

fn zufallsbereich(zufall: &mut Zufall, zeilen: usize, spalten: usize) -> Zellbereich {
    let von = zufall.bis(zeilen);
    let bis = von + 1 + zufall.bis(zeilen - von);
    let links = zufall.bis(spalten);
    let rechts = links + 1 + zufall.bis(spalten - links);
    Zellbereich {
        zeilen: von..bis,
        spalten: links..rechts,
    }
}

#[test]
fn jede_handlung_aendert_nur_die_bytes_der_beruehrten_saetze() {
    let mut zufall = Zufall(0x5eed_c5f0_2610_1800);
    for runde in 0..400 {
        let text = zufallstabelle(&mut zufall);
        let index = indexieren(&text, &komma());
        assert!(index.unterminiert().is_none(), "Runde {runde}: {text:?}");
        let alt = roh_saetze(&text, &index);
        let n = index.zeilenzahl();
        let spaltenzahl = index.spaltenzahl();
        let auswahl = zufallsbereich(&mut zufall, n, spaltenzahl);
        let kontext = format!("Runde {runde}, Auswahl {auswahl:?}, Text {text:?}");

        // Zelle setzen: jeder andere Satz und jedes andere Feld bleibt.
        let zeile = zufall.bis(n);
        let spalte = zufall.bis(spaltenzahl + 2);
        if let Some(stand) = zelle_setzen(&text, &index, zeile, spalte, "NEU,\"1\"") {
            let neu_index = indexieren(&stand.text, &komma());
            let neu = roh_saetze(&stand.text, &neu_index);
            assert_eq!(neu.len(), n, "{kontext}");
            for satz in 0..n {
                if satz != zeile {
                    assert_eq!(neu[satz], alt[satz], "zelle_setzen, Satz {satz}: {kontext}");
                } else {
                    assert_eq!(neu[satz].1, alt[satz].1, "Zeilenende: {kontext}");
                }
            }
            let alte_felder = roh_felder(&text, &index, zeile);
            let neue_felder = roh_felder(&stand.text, &neu_index, zeile);
            for (stelle, roh) in alte_felder.iter().enumerate() {
                if stelle != spalte {
                    assert_eq!(&neue_felder[stelle], roh, "Feld {stelle}: {kontext}");
                }
            }
            let wert = &neu_index.felder(&stand.text, zeile).expect("Satz")[spalte].wert;
            assert_eq!(wert, "NEU,\"1\"", "{kontext}");
        }

        // Zeilen einfuegen: davor und danach bleibt jeder Satz.
        let stand = zeilen_einfuegen(&text, &index, &auswahl).expect("einfuegen geht immer");
        let neu = roh_saetze(&stand.text, &indexieren(&stand.text, &komma()));
        let k = auswahl.zeilen.len();
        assert_eq!(neu.len(), n + k, "{kontext}");
        assert_eq!(
            neu[..auswahl.zeilen.start],
            alt[..auswahl.zeilen.start],
            "{kontext}"
        );
        assert_eq!(
            neu[auswahl.zeilen.end..],
            alt[auswahl.zeilen.start..],
            "{kontext}"
        );
        for leer in &neu[auswahl.zeilen.clone()] {
            assert_eq!(leer.0, ",".repeat(spaltenzahl - 1), "{kontext}");
        }

        // Anhaengen: alles bleibt, das letzte Zeilenende darf dazukommen.
        let stand = zeile_anhaengen(&text, &index).expect("anhaengen geht immer");
        let neu = roh_saetze(&stand.text, &indexieren(&stand.text, &komma()));
        assert_eq!(neu.len(), n + 1, "{kontext}");
        assert_eq!(neu[..n - 1], alt[..n - 1], "{kontext}");
        assert_eq!(neu[n - 1].0, alt[n - 1].0, "{kontext}");
        assert!(stand.text.starts_with(&text), "{kontext}");

        // Zeilen loeschen: die uebrigen bleiben, das Dateiende auch.
        let stand = zeilen_loeschen(&text, &index, &auswahl).expect("loeschen");
        let neu = roh_saetze(&stand.text, &indexieren(&stand.text, &komma()));
        let mut erwartet = alt.clone();
        erwartet.drain(auswahl.zeilen.clone());
        assert_eq!(neu.len(), erwartet.len(), "{kontext}");
        for (stelle, (neu_satz, alt_satz)) in neu.iter().zip(&erwartet).enumerate() {
            assert_eq!(neu_satz.0, alt_satz.0, "loeschen, Satz {stelle}: {kontext}");
            if stelle + 1 < neu.len() {
                assert_eq!(neu_satz.1, alt_satz.1, "loeschen, Ende {stelle}: {kontext}");
            }
        }
        if !erwartet.is_empty() && !erwartet.last().expect("nicht leer").0.is_empty() {
            assert_eq!(
                stand.text.ends_with('\n'),
                text.ends_with('\n'),
                "das Dateiende: {kontext}"
            );
        }

        // Verschieben: die Inhalte wandern, die Zeilenenden bleiben an ihrem Platz.
        for richtung in [Richtung::Hoch, Richtung::Runter] {
            let Some(stand) = zeilen_verschieben(&text, &index, &auswahl, richtung) else {
                continue;
            };
            let neu = roh_saetze(&stand.text, &indexieren(&stand.text, &komma()));
            assert_eq!(neu.len(), n, "{kontext}");
            let mut folge: Vec<usize> = (0..n).collect();
            let Zellbereich { zeilen, .. } = &auswahl;
            match richtung {
                Richtung::Hoch => folge[zeilen.start - 1..zeilen.end].rotate_left(1),
                Richtung::Runter => folge[zeilen.start..zeilen.end + 1].rotate_right(1),
            }
            for (platz, herkunft) in folge.into_iter().enumerate() {
                assert_eq!(
                    neu[platz].0, alt[herkunft].0,
                    "verschieben {richtung:?}: {kontext}"
                );
                if !(alt[platz].1.is_empty() && alt[herkunft].0.is_empty()) {
                    assert_eq!(neu[platz].1, alt[platz].1, "Ende {platz}: {kontext}");
                }
            }
        }

        // Spalte einfuegen: jeder Satz mit mindestens `an` Feldern bekommt ein
        // leeres Feld, alle anderen Felder bleiben roh.
        let an = zufall.bis(spaltenzahl + 1);
        let stand = spalte_einfuegen(&text, &index, an, &auswahl).expect("einfuegen");
        let neu_index = indexieren(&stand.text, &komma());
        let neu = roh_saetze(&stand.text, &neu_index);
        assert_eq!(neu.len(), n, "{kontext}");
        for satz in 0..n {
            assert_eq!(neu[satz].1, alt[satz].1, "{kontext}");
            let mut erwartet = roh_felder(&text, &index, satz);
            if erwartet.len() >= an {
                erwartet.insert(an, String::new());
            }
            assert_eq!(
                roh_felder(&stand.text, &neu_index, satz),
                erwartet,
                "spalte_einfuegen an {an}, Satz {satz}: {kontext}"
            );
        }

        // Spalten loeschen: die Felder der Auswahl gehen, alle anderen bleiben roh.
        let ergebnis = spalten_loeschen(&text, &index, &auswahl);
        if auswahl.spalten.start == 0 && auswahl.spalten.end >= spaltenzahl {
            assert_eq!(ergebnis, Err(LetzteSpalte), "{kontext}");
        } else if let Some(stand) = ergebnis.expect("nicht die letzte Spalte") {
            let neu_index = indexieren(&stand.text, &komma());
            let neu = roh_saetze(&stand.text, &neu_index);
            assert_eq!(neu.len(), n, "{kontext}");
            for satz in 0..n {
                let mut erwartet = roh_felder(&text, &index, satz);
                let bis = auswahl.spalten.end.min(erwartet.len());
                if erwartet.len() > auswahl.spalten.start {
                    erwartet.drain(auswahl.spalten.start..bis);
                }
                if erwartet.is_empty() {
                    erwartet.push(String::new());
                }
                // Ein leer gewordener letzter Satz bekommt ein Zeilenende,
                // jeder andere behaelt seins.
                if !(neu[satz].0.is_empty() && alt[satz].1.is_empty()) {
                    assert_eq!(neu[satz].1, alt[satz].1, "{kontext}");
                }
                assert_eq!(
                    roh_felder(&stand.text, &neu_index, satz),
                    erwartet,
                    "spalten_loeschen, Satz {satz}: {kontext}"
                );
            }
        }

        // Ein Block: die beruehrten Saetze behalten jedes Feld ausserhalb.
        let block = vec![vec!["P".to_owned(), String::new()], vec!["Q\tR".to_owned()]];
        let zeile = zufall.bis(n + 1);
        let spalte = zufall.bis(spaltenzahl);
        if let Some(stand) = bereich_einsetzen(&text, &index, zeile, spalte, &block) {
            let neu_index = indexieren(&stand.text, &komma());
            let neu = roh_saetze(&stand.text, &neu_index);
            assert_eq!(neu.len(), n.max(zeile + block.len()), "{kontext}");
            for satz in 0..n {
                if !(zeile..zeile + block.len()).contains(&satz) {
                    assert_eq!(neu[satz].0, alt[satz].0, "bereich, Satz {satz}: {kontext}");
                    // Haengt der Block Zeilen an, bekommt ein letzter Satz ohne
                    // Zeilenende eins; jedes andere bleibt.
                    let angehaengt = zeile + block.len() > n && satz + 1 == n;
                    if !(angehaengt && alt[satz].1.is_empty()) {
                        assert_eq!(neu[satz].1, alt[satz].1, "bereich, Ende {satz}: {kontext}");
                    }
                    continue;
                }
                let alte = roh_felder(&text, &index, satz);
                let neue = roh_felder(&stand.text, &neu_index, satz);
                for (stelle, roh) in alte.iter().enumerate() {
                    if !(spalte..spalte + 2).contains(&stelle) {
                        assert_eq!(&neue[stelle], roh, "bereich, Feld {stelle}: {kontext}");
                    }
                }
                let werte = &werte(&stand.text, &neu_index)[satz];
                assert_eq!(werte[spalte], block[satz - zeile][0], "{kontext}");
            }
        }
    }
}

#[test]
fn eine_zelle_aendert_nur_ihr_feld_auch_mit_marke_und_crlf() {
    let text = "\u{feff}sep=;\r\nName;Notiz\r\n\"Äpfel\";\"rot\r\nund grün\"\r\nBirnen;x\r\n";
    let gewaehlt = dialekt_erkennen(Art::Csv, text);
    assert_eq!(gewaehlt.trennzeichen, Trennzeichen::Semikolon);
    let index = indexieren(text, &gewaehlt);
    assert_eq!(index.zeilenzahl(), 3);
    let stand = zelle_setzen(text, &index, 2, 1, "y; z").expect("gesetzt");
    assert_eq!(
        stand.text,
        "\u{feff}sep=;\r\nName;Notiz\r\n\"Äpfel\";\"rot\r\nund grün\"\r\nBirnen;\"y; z\"\r\n"
    );
    let stand = zeile_anhaengen(text, &index).expect("angehaengt");
    assert!(stand.text.ends_with("Birnen;x\r\n;\r\n"));
    assert!(stand.text.starts_with("\u{feff}sep=;\r\n"));
}

// ---------------------------------------------------------------- Messung

/// Die Messprobe aus Schritt 1: Zeit und Speicher des Index an einer erzeugten
/// Datei von 16 MiB.
///
/// `cargo test --release -p krk-core --test trenntext -- --ignored --nocapture`
#[test]
#[ignore = "Messung, kein Abnahmekriterium"]
fn messung_index_einer_16_mib_datei() {
    const ZIEL: usize = 16 * 1024 * 1024;
    let mut text = String::with_capacity(ZIEL + 256);
    let mut nummer = 0usize;
    while text.len() < ZIEL {
        text.push_str(&format!(
            "{nummer},Äpfel {nummer},\"mit, Komma\",\"zwei\nzeilen\",{},x\"y,,ende\n",
            nummer * 7
        ));
        nummer += 1;
    }
    let dialekt_beginn = Instant::now();
    let gewaehlt = dialekt_erkennen(Art::Csv, &text);
    let dialekt_dauer = dialekt_beginn.elapsed();
    assert_eq!(gewaehlt.trennzeichen, Trennzeichen::Komma);

    let mut dauern = Vec::new();
    let mut index = None;
    for _ in 0..5 {
        let beginn = Instant::now();
        let gelesen = indexieren(&text, &gewaehlt);
        dauern.push(beginn.elapsed());
        index = Some(gelesen);
    }
    let index = index.expect("fuenf Laeufe");
    dauern.sort();
    let speicher = std::mem::size_of_val(index.saetze());

    let beginn = Instant::now();
    let stand = zelle_setzen(&text, &index, index.zeilenzahl() / 2, 1, "neu").expect("gesetzt");
    let zelle_dauer = beginn.elapsed();
    let beginn = Instant::now();
    let spalte = spalte_einfuegen(&text, &index, 1, &Zellbereich::zelle(0, 0)).expect("Spalte");
    let spalte_dauer = beginn.elapsed();
    assert!(stand.text.len() > ZIEL && spalte.text.len() > ZIEL);

    println!(
        "trenntext: {} Bytes, {} Saetze, {} Spalten",
        text.len(),
        index.zeilenzahl(),
        index.spaltenzahl()
    );
    println!("  dialekt_erkennen: {dialekt_dauer:?}");
    println!(
        "  indexieren: min {:?}, median {:?}, max {:?}",
        dauern[0], dauern[2], dauern[4]
    );
    println!(
        "  Index im Speicher: {} Bytes ({} KiB)",
        speicher,
        speicher / 1024
    );
    println!("  zelle_setzen samt Spleiss: {zelle_dauer:?}");
    println!("  spalte_einfuegen: {spalte_dauer:?}");
}
