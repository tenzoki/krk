//! Die Aufbereitung einer CSV- oder TSV-Datei fuer die Vorschau: ein
//! ausgerichteter Text mit einer Kopfzeile, einer Zeile mit den
//! Spaltenbuchstaben, einer Trennlinie und links der Nummer jeder Zeile.
//!
//! ```text
//! Semikolon · 2 Zeilen · 3 Spalten
//!  │ A     │ B     │ C
//! ─┼───────┼───────┼──────
//! 1│ Name  │ Preis │ Menge
//! 2│ Äpfel │ 1,50  │ 3
//! ```
//!
//! Plan `261018-0822_*_plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md`
//! im Arbeitspaket `261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor`,
//! Schritt 2 und Entscheidung E17; der Entscheid zur Vorschau hat Moeglichkeit
//! 1 gewaehlt, ausgerichteten Text in der vorhandenen Textflaeche.
//!
//! **Keine Zeile AppKit**, wie [`crate::jsonzeilen`] daneben: die Datei rechnet
//! einen Text aus einem Text, und gerufen wird sie allein auf dem Arbeitsfaden
//! der Vorschau (`vorschaumodell::laden` und `archiveintrag_laden`), also nie
//! auf dem Hauptfaden.
//!
//! **Zerlegt wird ueber den einen Leser des Kerns**, [`trenntext::indexieren`]
//! und [`Satzindex::felder`]; einen zweiten Leser hat diese Datei nicht.
//!
//! # Was die Ausrichtung nicht kann
//!
//! Die Breite einer Spalte ist die groesste Anzeigebreite ihrer Zellen, hoechstens
//! [`HOECHSTBREITE`]; eine laengere Zelle endet in „…“. Ein Zeichen zaehlt eins,
//! ein ostasiatisches Breitzeichen zwei, nach der kleinen festen Tafel
//! [`BREITZEICHEN`]. **Zeichen ohne eigene Breite kennt die Tafel nicht**: ein
//! kombinierendes Zeichen (etwa das U+0308 eines zerlegten „ä“) zaehlt eins wie
//! jedes andere, und eine Zelle, die eines traegt, steht um eine Stelle zu kurz
//! aufgefuellt da. Eine vollstaendige Breitenrechnung braeuchte die
//! Unicode-Tabellen einer fremden Kiste (E20), und die Vorschau zeigt; sie
//! rechnet nicht mit den Werten.
//!
//! Ein Zeilenumbruch in einer Zelle erscheint als `⏎` (`\r\n` als eines), ein
//! Tabulator als `⇥`, damit jede Zeile der Datei genau eine Zeile der Anzeige
//! bleibt.

use krk_core::sprache::{Text, Zahlwort, anzahl, satz, text};
use krk_core::trenntext::{self, Dialekt, Satzindex, Trennzeichen};

/// Die groesste Anzeigebreite einer Spalte; eine laengere Zelle wird gekuerzt.
pub const HOECHSTBREITE: usize = 40;

/// Die Bereiche der ostasiatischen Breitzeichen, die zwei Stellen einnehmen:
/// Hangul-Anlaute, die CJK-Zeichen und -Satzzeichen, Hiragana, Katakana,
/// Hangul-Silben, die Vollbreitformen und die Bildzeichen der Emoji-Bloecke.
///
/// **Klein und fest, und mit Absicht nicht vollstaendig**; der Modulkopf sagt,
/// was ihr fehlt.
const BREITZEICHEN: [(u32, u32); 15] = [
    (0x1100, 0x115F),
    (0x2E80, 0x303E),
    (0x3041, 0x33FF),
    (0x3400, 0x4DBF),
    (0x4E00, 0x9FFF),
    (0xA000, 0xA4CF),
    (0xAC00, 0xD7A3),
    (0xF900, 0xFAFF),
    (0xFE30, 0xFE4F),
    (0xFF00, 0xFF60),
    (0xFFE0, 0xFFE6),
    (0x1F300, 0x1F64F),
    (0x1F900, 0x1F9FF),
    (0x20000, 0x2FFFD),
    (0x30000, 0x3FFFD),
];

/// Die Anzeigebreite eines Zeichens: zwei fuer ein Breitzeichen, sonst eins.
#[must_use]
pub fn zeichenbreite(zeichen: char) -> usize {
    let wert = u32::from(zeichen);
    if BREITZEICHEN
        .iter()
        .any(|&(von, bis)| (von..=bis).contains(&wert))
    {
        2
    } else {
        1
    }
}

/// Bereitet `quelle` im `dialekt` als ausgerichteten Text auf, oder `None`,
/// wenn die Datei als Text erscheinen soll (E17): ein Anfuehrungszeichen
/// schliesst nicht, sie traegt mehr als [`trenntext::HOECHSTENS_SPALTEN`]
/// Spalten, oder sie hat keine einzige Zeile.
///
/// `gekuerzt_bei` ist die Lesegrenze in MB, wenn `quelle` nur der Anfang der
/// Datei ist; dann sagt es die Kopfzeile. Dass `quelle` am Ende eines
/// vollstaendigen Satzes aufhoert, sorgt der Rufer
/// ([`trenntext::vollstaendig_bis`]).
#[must_use]
pub fn aufbereiten(quelle: &str, dialekt: &Dialekt, gekuerzt_bei: Option<u64>) -> Option<String> {
    let index = trenntext::indexieren(quelle, dialekt);
    if index.unterminiert().is_some()
        || index.spaltenzahl() > trenntext::HOECHSTENS_SPALTEN
        || index.zeilenzahl() == 0
    {
        return None;
    }
    Some(ausrichten(quelle, &index, gekuerzt_bei))
}

/// Eine Zelle, wie sie dasteht: ihr Text und dessen Anzeigebreite.
struct Anzeigezelle {
    text: String,
    breite: usize,
}

/// Baut den ausgerichteten Text aus einem gueltigen Index.
fn ausrichten(quelle: &str, index: &Satzindex, gekuerzt_bei: Option<u64>) -> String {
    let spalten = index.spaltenzahl();
    let buchstaben: Vec<String> = (0..spalten).map(spaltenbuchstabe).collect();
    let mut breiten: Vec<usize> = buchstaben.iter().map(|b| b.chars().count()).collect();

    // Erst alle Zellen, dann die Breiten: die Breite einer Spalte haengt an
    // ihrer laengsten Zelle, und die kann in der letzten Zeile stehen.
    let zeilen: Vec<Vec<Anzeigezelle>> = (0..index.zeilenzahl())
        .map(|satz| {
            index
                .felder(quelle, satz)
                .unwrap_or_default()
                .iter()
                .map(|feld| anzeigezelle(&feld.wert))
                .collect()
        })
        .collect();
    for zeile in &zeilen {
        for (spalte, zelle) in zeile.iter().enumerate() {
            breiten[spalte] = breiten[spalte].max(zelle.breite);
        }
    }

    let nummernbreite = index.zeilenzahl().to_string().len();
    let mut aus = String::with_capacity(quelle.len() * 2);
    aus.push_str(&kopfzeile(index, gekuerzt_bei));
    aus.push('\n');

    let kopf: Vec<Anzeigezelle> = buchstaben
        .into_iter()
        .map(|text| Anzeigezelle {
            breite: text.chars().count(),
            text,
        })
        .collect();
    zeile_schreiben(&mut aus, "", nummernbreite, &kopf, &breiten);

    aus.push_str(&"─".repeat(nummernbreite));
    for (spalte, breite) in breiten.iter().enumerate() {
        aus.push('┼');
        // Zwei Stellen Abstand um jede Zelle, bei der letzten nur davor.
        let abstand = if spalte + 1 == spalten { 1 } else { 2 };
        aus.push_str(&"─".repeat(breite + abstand));
    }
    aus.push('\n');

    for (satz, zeile) in zeilen.iter().enumerate() {
        zeile_schreiben(
            &mut aus,
            &(satz + 1).to_string(),
            nummernbreite,
            zeile,
            &breiten,
        );
    }
    aus
}

/// Eine Zeile der Anzeige: die Nummer rechtsbuendig, dann je Spalte ein Strich,
/// ein Leerzeichen und die aufgefuellte Zelle. Die letzte Spalte wird nicht
/// aufgefuellt, damit keine Zeile Leerraum ohne Inhalt nachzieht; eine Zelle
/// jenseits des Satzendes (E7) ist leer.
fn zeile_schreiben(
    aus: &mut String,
    nummer: &str,
    nummernbreite: usize,
    zellen: &[Anzeigezelle],
    breiten: &[usize],
) {
    aus.push_str(&" ".repeat(nummernbreite.saturating_sub(nummer.len())));
    aus.push_str(nummer);
    for (spalte, &breite) in breiten.iter().enumerate() {
        aus.push('│');
        aus.push(' ');
        let (text, belegt) = zellen
            .get(spalte)
            .map_or(("", 0), |zelle| (zelle.text.as_str(), zelle.breite));
        aus.push_str(text);
        if spalte + 1 < breiten.len() {
            aus.push_str(&" ".repeat(breite - belegt + 1));
        }
    }
    aus.push('\n');
}

/// Die erste Zeile: Trennzeichen, Zeilen- und Spaltenzahl, bei Kuerzung mit
/// dem Hinweis.
fn kopfzeile(index: &Satzindex, gekuerzt_bei: Option<u64>) -> String {
    let trennzeichen = trennzeichenname(index.trennzeichen());
    let zeilen = anzahl(Zahlwort::TrenntextZeilen, index.zeilenzahl() as u64, &[]);
    let spalten = anzahl(Zahlwort::TrenntextSpalten, index.spaltenzahl() as u64, &[]);
    match gekuerzt_bei {
        None => satz(
            Text::TrenntextKopfzeile,
            &[
                ("trennzeichen", &trennzeichen),
                ("zeilen", &zeilen),
                ("spalten", &spalten),
            ],
        ),
        Some(grenze) => satz(
            Text::TrenntextKopfzeileGekuerzt,
            &[
                ("trennzeichen", &trennzeichen),
                ("zeilen", &zeilen),
                ("spalten", &spalten),
                ("grenze", &grenze),
            ],
        ),
    }
}

/// Der Name eines Trennzeichens in der geltenden Sprache.
#[must_use]
pub fn trennzeichenname(trennzeichen: Trennzeichen) -> &'static str {
    match trennzeichen {
        Trennzeichen::Komma => text(Text::TrennzeichenKomma),
        Trennzeichen::Semikolon => text(Text::TrennzeichenSemikolon),
        Trennzeichen::Tabulator => text(Text::TrennzeichenTabulator),
    }
}

/// Der Buchstabe der Spalte `spalte`, von null gezaehlt: A bis Z, dann AA, AB
/// und so fort, wie in einer Tabellenkalkulation (E6).
#[must_use]
pub fn spaltenbuchstabe(spalte: usize) -> String {
    let mut rest = spalte + 1;
    let mut umgekehrt = Vec::new();
    while rest > 0 {
        rest -= 1;
        umgekehrt.push(char::from(b'A' + (rest % 26) as u8));
        rest /= 26;
    }
    umgekehrt.into_iter().rev().collect()
}

/// Ein Wert, wie ihn die Anzeige zeigt: Umbrueche und Tabulatoren als Zeichen,
/// ueber [`HOECHSTBREITE`] gekuerzt mit „…“.
fn anzeigezelle(wert: &str) -> Anzeigezelle {
    let mut zeichen: Vec<char> = Vec::with_capacity(wert.len());
    let mut lauf = wert.chars().peekable();
    while let Some(c) = lauf.next() {
        match c {
            '\r' => {
                // `\r\n` ist ein Umbruch und kein zweiter.
                if lauf.peek() == Some(&'\n') {
                    let _ = lauf.next();
                }
                zeichen.push('⏎');
            }
            '\n' => zeichen.push('⏎'),
            '\t' => zeichen.push('⇥'),
            anderes => zeichen.push(anderes),
        }
    }
    let gesamt: usize = zeichen.iter().map(|&c| zeichenbreite(c)).sum();
    if gesamt <= HOECHSTBREITE {
        return Anzeigezelle {
            text: zeichen.into_iter().collect(),
            breite: gesamt,
        };
    }
    // Platz fuer die Auslassung lassen; ein Breitzeichen, das nicht mehr ganz
    // passt, faellt mit weg, und die Zelle ist dann eine Stelle schmaler.
    let mut text = String::new();
    let mut breite = 0;
    for c in zeichen {
        let b = zeichenbreite(c);
        if breite + b > HOECHSTBREITE - 1 {
            break;
        }
        text.push(c);
        breite += b;
    }
    text.push('…');
    Anzeigezelle {
        text,
        breite: breite + 1,
    }
}

#[cfg(test)]
mod tests {
    use krk_core::trenntext::{Art, dialekt_erkennen};

    use super::*;

    fn aufbereitet(art: Art, quelle: &str) -> Option<String> {
        aufbereiten(quelle, &dialekt_erkennen(art, quelle), None)
    }

    /// Die genaue Erwartung fuer eine kleine Datei mit Semikolon,
    /// Dezimalkomma und Umlauten.
    #[test]
    fn eine_csv_erscheint_mit_kopfzeile_buchstaben_trennlinie_und_nummern() {
        let quelle = "Name;Preis;Menge\nÄpfel;1,50;3\n";
        let kopf = satz(
            Text::TrenntextKopfzeile,
            &[
                ("trennzeichen", &text(Text::TrennzeichenSemikolon)),
                ("zeilen", &anzahl(Zahlwort::TrenntextZeilen, 2, &[])),
                ("spalten", &anzahl(Zahlwort::TrenntextSpalten, 3, &[])),
            ],
        );
        let erwartet = format!(
            "{kopf}\n \
             │ A     │ B     │ C\n\
             ─┼───────┼───────┼──────\n\
             1│ Name  │ Preis │ Menge\n\
             2│ Äpfel │ 1,50  │ 3\n"
        );
        assert_eq!(
            aufbereitet(Art::Csv, quelle).as_deref(),
            Some(erwartet.as_str())
        );
        assert_eq!(kopf, "Semikolon · 2 Zeilen · 3 Spalten");
    }

    /// Ein kuerzerer Satz zeigt leere Zellen, die Nummernspalte waechst mit
    /// der Zeilenzahl, und eine TSV liest Tabulatoren.
    #[test]
    fn kurze_saetze_und_viele_zeilen_bleiben_ausgerichtet() {
        let mut quelle = String::from("a\tb\tc\n");
        for _ in 0..10 {
            quelle.push_str("x\n");
        }
        let text = aufbereitet(Art::Tsv, &quelle).expect("Tabelle");
        let zeilen: Vec<&str> = text.lines().collect();
        assert_eq!(zeilen[1], "  │ A │ B │ C");
        assert_eq!(zeilen[2], "──┼───┼───┼──");
        assert_eq!(zeilen[3], " 1│ a │ b │ c");
        assert_eq!(zeilen[4], " 2│ x │   │ ");
        assert_eq!(zeilen[13], "11│ x │   │ ");
    }

    #[test]
    fn eine_lange_zelle_endet_in_auslassung_und_umbrueche_erscheinen_als_zeichen() {
        let lang = "x".repeat(HOECHSTBREITE + 5);
        let quelle = format!("\"zwei\r\nzeilen\"\t\"ein\ntab\"\t{lang}\n\"a\tb\"\n");
        let text = aufbereitet(Art::Tsv, &quelle).expect("Tabelle");
        let zeilen: Vec<&str> = text.lines().collect();
        let gekuerzt = format!("{}…", "x".repeat(HOECHSTBREITE - 1));
        assert_eq!(zeilen[3], format!("1│ zwei⏎zeilen │ ein⏎tab │ {gekuerzt}"));
        assert_eq!(
            zeilen[4],
            format!("2│ a⇥b{}│{}│ ", " ".repeat(9), " ".repeat(9))
        );
        assert_eq!(anzeigezelle(&lang).breite, HOECHSTBREITE);
        assert_eq!(text.lines().count(), 5, "jeder Satz ist eine Zeile");
    }

    #[test]
    fn breitzeichen_zaehlen_doppelt() {
        assert_eq!(zeichenbreite('漢'), 2);
        assert_eq!(zeichenbreite('한'), 2);
        assert_eq!(zeichenbreite('Ä'), 1);
        assert_eq!(zeichenbreite('a'), 1);
        let text = aufbereitet(Art::Csv, "漢字,b\nab,c\n").expect("Tabelle");
        let zeilen: Vec<&str> = text.lines().collect();
        assert_eq!(zeilen[3], "1│ 漢字 │ b");
        assert_eq!(zeilen[4], "2│ ab   │ c");
        // Ein Breitzeichen an der Grenze faellt mit weg, statt sie zu sprengen.
        let breit = "漢".repeat(HOECHSTBREITE);
        let zelle = anzeigezelle(&breit);
        assert!(zelle.breite <= HOECHSTBREITE);
        assert!(zelle.text.ends_with('…'));
    }

    #[test]
    fn eine_unterminierte_eine_leere_und_eine_zu_breite_csv_erscheinen_als_text() {
        assert_eq!(aufbereitet(Art::Csv, "a,\"offen\nb,c\n"), None);
        assert_eq!(aufbereitet(Art::Csv, ""), None);
        assert_eq!(aufbereitet(Art::Csv, "\u{feff}"), None);
        let breit = ",".repeat(trenntext::HOECHSTENS_SPALTEN);
        assert_eq!(aufbereitet(Art::Csv, &format!("{breit}\n")), None);
        let gerade = ",".repeat(trenntext::HOECHSTENS_SPALTEN - 1);
        assert!(aufbereitet(Art::Csv, &format!("{gerade}\n")).is_some());
    }

    /// Die `sep=`-Zeile ist Vorspann und keine Zeile der Tabelle; eine
    /// Bytefolgenmarke erscheint nicht.
    #[test]
    fn vorspann_und_marke_erscheinen_nicht_als_zeile() {
        let text = aufbereitet(Art::Csv, "\u{feff}sep=;\na;b\n").expect("Tabelle");
        assert!(!text.contains("sep="));
        assert!(!text.contains('\u{feff}'));
        assert!(text.ends_with("1│ a │ b\n"));
    }

    #[test]
    fn die_spaltenbuchstaben_zaehlen_wie_eine_tabellenkalkulation() {
        assert_eq!(spaltenbuchstabe(0), "A");
        assert_eq!(spaltenbuchstabe(25), "Z");
        assert_eq!(spaltenbuchstabe(26), "AA");
        assert_eq!(spaltenbuchstabe(27), "AB");
        assert_eq!(spaltenbuchstabe(701), "ZZ");
        assert_eq!(spaltenbuchstabe(702), "AAA");
    }

    /// Die gekuerzte Kopfzeile nennt die Grenze.
    #[test]
    fn die_gekuerzte_kopfzeile_nennt_die_grenze() {
        let quelle = "a,b\n";
        let text =
            aufbereiten(quelle, &dialekt_erkennen(Art::Csv, quelle), Some(1)).expect("Tabelle");
        let erste = text.lines().next().expect("Kopfzeile");
        assert!(
            erste.starts_with("Komma · eine Zeile · 2 Spalten"),
            "{erste}"
        );
        assert!(erste.contains("1 MB"), "{erste}");
    }
}
