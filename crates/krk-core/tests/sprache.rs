//! Abnahme der Sprachtabelle: was der Uebersetzer an den drei Tabellen nicht
//! haelt, haelt diese Datei.
//!
//! Der Uebersetzer haelt, dass jeder Schluessel in jeder Sprache einen
//! Eintrag hat; er haelt nicht, ob der Eintrag leer, ohne seine Platzhalter
//! oder in der falschen Typografie geschrieben ist. Die Proben hier laufen
//! ueber `Text::ALLE` und `Zahlwort::ALLE` und fragen jede Sprache ueber die
//! Formen mit ausdruecklicher Sprache, `Sprache::text` und
//! `Sprache::zahlwort`; die Mechanik des Einsetzens an freien Vorlagen prueft
//! das Pruefmodul von `krk_core::sprache` selbst.
//!
//! **Keine Probe dieser Datei ruft `festlegen`.** Die Sprache ist ein Wert
//! des Prozesses, und `cargo test` faehrt jede Probe eines Testziels in
//! einem Prozess; eine Probe, die `festlegen` riefe, baende jede andere an
//! ihren Wert. `geltende()` ist deshalb in jeder Probe Deutsch, und die
//! letzte Probe haelt das fest.
//!
//! Was keine Probe hier kann: eine Uebersetzung auf Richtigkeit pruefen. Das
//! ist die Durchsicht des Nutzers, Flaeche fuer Flaeche in der laufenden
//! Anwendung, und bis dahin tragen `fr.rs` und `en.rs` ihre Kopfzeile.

use std::collections::BTreeSet;

use krk_core::sprache::{self, Sprache, Text, Zahlwort};
use krk_core::tasten::{Funktionsschluessel, Kommando, Zugestellt};

/// Jeder Eintrag einer Sprache mit seinem Namen: die `Text`-Eintraege, je
/// `Zahlwort` beide Formen, und seit Schritt 5 des Plans die Namen der
/// Funktionen der Belegung ueber beide `KENNUNGEN`, damit die
/// Typografieproben auch sie sehen (das franzoesische „Rendez-vous :“ etwa
/// braucht sein geschuetztes Leerzeichen wie jeder andere Eintrag).
fn eintraege(sprache: Sprache) -> Vec<(String, &'static str)> {
    let mut alle = Vec::new();
    for schluessel in Text::ALLE {
        alle.push((format!("{schluessel:?}"), sprache.text(schluessel)));
    }
    for schluessel in Zahlwort::ALLE {
        let (einzahl, mehrzahl) = sprache.zahlwort(schluessel);
        alle.push((format!("{schluessel:?} (Einzahl)"), einzahl));
        alle.push((format!("{schluessel:?} (Mehrzahl)"), mehrzahl));
    }
    for (kommando, _) in Kommando::KENNUNGEN {
        alle.push((
            format!("{kommando:?}"),
            sprache.funktionsname(Funktionsschluessel::Kommando(kommando)),
        ));
    }
    for (zugestellt, _) in Zugestellt::KENNUNGEN {
        alle.push((
            format!("{zugestellt:?}"),
            sprache.funktionsname(Funktionsschluessel::Zugestellt(zugestellt)),
        ));
    }
    alle
}

/// Die Namen der Platzhalter `{…}` eines Eintrags.
fn platzhalter(eintrag: &str) -> BTreeSet<String> {
    let mut namen = BTreeSet::new();
    let mut rest = eintrag;
    while let Some(anfang) = rest.find('{') {
        let nach = &rest[anfang + 1..];
        let ende = nach
            .find('}')
            .unwrap_or_else(|| panic!("der Eintrag `{eintrag}` schliesst einen Platzhalter nicht"));
        namen.insert(nach[..ende].to_owned());
        rest = &nach[ende + 1..];
    }
    namen
}

/// C4: zu jedem Schluessel ein nicht leerer Text, in jeder Sprache, bei
/// einem Zahlwort in beiden Formen.
#[test]
fn kein_eintrag_ist_leer() {
    for sprache in Sprache::ALLE {
        for (name, eintrag) in eintraege(sprache) {
            assert!(
                !eintrag.trim().is_empty(),
                "{name} ist in {} leer",
                sprache.kennung()
            );
        }
    }
}

/// C4: die Platzhalter eines Eintrags sind in allen drei Sprachen dieselben;
/// die Reihenfolge darf abweichen, die Menge nicht.
#[test]
fn die_platzhalter_sind_in_allen_sprachen_gleich() {
    for schluessel in Text::ALLE {
        let deutsch = platzhalter(Sprache::De.text(schluessel));
        for sprache in [Sprache::Fr, Sprache::En] {
            assert_eq!(
                platzhalter(sprache.text(schluessel)),
                deutsch,
                "{schluessel:?} traegt in {} andere Platzhalter als in de",
                sprache.kennung()
            );
        }
    }
}

/// C4 fuer die Zahlwoerter: die Platzhalter beider Formen zusammen, ohne
/// `n`, sind in allen drei Sprachen dieselben. `n` bleibt aussen vor, weil
/// die Einzahl es auslassen darf.
#[test]
fn die_platzhalter_der_zahlwoerter_sind_ohne_n_in_allen_sprachen_gleich() {
    let ohne_n = |sprache: Sprache, schluessel: Zahlwort| {
        let (einzahl, mehrzahl) = sprache.zahlwort(schluessel);
        let mut namen = platzhalter(einzahl);
        namen.extend(platzhalter(mehrzahl));
        namen.remove("n");
        namen
    };
    for schluessel in Zahlwort::ALLE {
        let deutsch = ohne_n(Sprache::De, schluessel);
        for sprache in [Sprache::Fr, Sprache::En] {
            assert_eq!(
                ohne_n(sprache, schluessel),
                deutsch,
                "{schluessel:?} traegt in {} andere Platzhalter als in de",
                sprache.kennung()
            );
        }
    }
}

/// Kein Eintrag traegt ein ASCII-Anfuehrungszeichen: Deutsch schreibt „ “,
/// Franzoesisch « », Englisch “ ”.
#[test]
fn kein_eintrag_traegt_ein_ascii_anfuehrungszeichen() {
    for sprache in Sprache::ALLE {
        for (name, eintrag) in eintraege(sprache) {
            assert!(
                !eintrag.contains('"'),
                "{name} traegt in {} ein ASCII-Anfuehrungszeichen: {eintrag}",
                sprache.kennung()
            );
        }
    }
}

/// Deutsche Eintraege tragen keine Guillemets.
#[test]
fn deutsche_eintraege_tragen_keine_guillemets() {
    for (name, eintrag) in eintraege(Sprache::De) {
        assert!(
            !eintrag.contains('«') && !eintrag.contains('»'),
            "{name} traegt in de Guillemets: {eintrag}"
        );
    }
}

/// C4 fuer Franzoesisch: keine deutschen Anfuehrungszeichen, Guillemets mit
/// U+00A0 innen, und vor `:`, `;`, `!` und `?` ein geschuetztes Leerzeichen
/// (U+00A0 oder U+202F), nie ein gewoehnliches und nie keines.
///
/// **Ein `:` innerhalb einer Schreibweise ist kein Satzzeichen.** `HH:MM`
/// und `09:30` tragen den Doppelpunkt als Trenner einer Uhrzeit, und ein
/// geschuetztes Leerzeichen davor machte die Schreibweise falsch, die der
/// Nutzer in `appointments.md` tippen soll. Die Probe unterscheidet die
/// beiden Faelle an dem Zeichen danach: folgt auf das Zeichen unmittelbar
/// ein Buchstabe oder eine Ziffer, ist es ein Trenner und bleibt ungeprueft;
/// sonst ist es ein Satzzeichen. Der Plan nannte diese Unterscheidung nicht;
/// sie ist eine Eigenschaft des Zeichens und keine Liste von Ausnahmen.
#[test]
fn franzoesische_eintraege_tragen_die_franzoesische_typografie() {
    for (name, eintrag) in eintraege(Sprache::Fr) {
        assert!(
            !eintrag.contains('„') && !eintrag.contains('“'),
            "{name} traegt in fr deutsche Anfuehrungszeichen: {eintrag}"
        );
        let zeichen: Vec<char> = eintrag.chars().collect();
        for (stelle, &aktuell) in zeichen.iter().enumerate() {
            let davor = stelle.checked_sub(1).map(|s| zeichen[s]);
            let danach = zeichen.get(stelle + 1).copied();
            match aktuell {
                '«' => assert_eq!(
                    danach,
                    Some('\u{a0}'),
                    "{name}: nach « steht kein U+00A0: {eintrag}"
                ),
                '»' => assert_eq!(
                    davor,
                    Some('\u{a0}'),
                    "{name}: vor » steht kein U+00A0: {eintrag}"
                ),
                ':' | ';' | '!' | '?' => {
                    if danach.is_some_and(char::is_alphanumeric) {
                        continue;
                    }
                    let Some(davor) = davor else {
                        continue;
                    };
                    assert!(
                        davor != ' ',
                        "{name}: vor `{aktuell}` steht ein gewoehnliches Leerzeichen: {eintrag}"
                    );
                    assert!(
                        !davor.is_alphanumeric(),
                        "{name}: vor `{aktuell}` fehlt das geschuetzte Leerzeichen: {eintrag}"
                    );
                    assert!(
                        davor == '\u{a0}' || davor == '\u{202f}' || !davor.is_whitespace(),
                        "{name}: vor `{aktuell}` steht ein anderer Leerraum als U+00A0 oder \
                         U+202F: {eintrag}"
                    );
                }
                _ => {}
            }
        }
    }
}

/// C4 fuer Englisch: weder deutsche Anfuehrungszeichen noch Guillemets.
#[test]
fn englische_eintraege_tragen_keine_deutschen_oder_franzoesischen_anfuehrungszeichen() {
    for (name, eintrag) in eintraege(Sprache::En) {
        assert!(
            !eintrag.contains('„') && !eintrag.contains('«') && !eintrag.contains('»'),
            "{name} traegt in en fremde Anfuehrungszeichen: {eintrag}"
        );
    }
}

/// `Sprache::aus_bezeichner` liest den Teil vor `-` oder `_` ohne Ruecksicht
/// auf Gross- und Kleinschreibung und antwortet fuer alles andere Englisch.
#[test]
fn der_bezeichner_wird_am_stamm_erkannt_und_sonst_englisch() {
    for bezeichner in ["de", "de-CH", "de_AT", "DE"] {
        assert_eq!(
            Sprache::aus_bezeichner(bezeichner),
            Sprache::De,
            "{bezeichner}"
        );
    }
    for bezeichner in ["fr", "fr-CA"] {
        assert_eq!(
            Sprache::aus_bezeichner(bezeichner),
            Sprache::Fr,
            "{bezeichner}"
        );
    }
    for bezeichner in ["en", "en-GB", "ja", "pt-BR", ""] {
        assert_eq!(
            Sprache::aus_bezeichner(bezeichner),
            Sprache::En,
            "{bezeichner}"
        );
    }
}

/// Die Kennung jeder Sprache ist der Stamm, den `aus_bezeichner` erkennt.
#[test]
fn die_kennung_fuehrt_zur_sprache_zurueck() {
    for sprache in Sprache::ALLE {
        assert_eq!(Sprache::aus_bezeichner(sprache.kennung()), sprache);
    }
}

/// Deutsch und Englisch setzen allein die 1 in die Einzahl, Franzoesisch
/// auch die 0.
#[test]
fn die_mehrzahlregel_je_sprache() {
    for sprache in [Sprache::De, Sprache::En] {
        assert!(sprache.mehrzahl(0), "{}", sprache.kennung());
        assert!(!sprache.mehrzahl(1), "{}", sprache.kennung());
        assert!(sprache.mehrzahl(2), "{}", sprache.kennung());
    }
    assert!(!Sprache::Fr.mehrzahl(0));
    assert!(!Sprache::Fr.mehrzahl(1));
    assert!(Sprache::Fr.mehrzahl(2));
}

/// Bei null sagt keine Mengenangabe „eins“: in jeder Sprache und fuer jedes
/// `Zahlwort` traegt `anzahl(…, 0, …)` die Ziffer 0 oder ist die Mehrzahl.
///
/// Gehalten wird eine Eigenschaft der Ausgabe und keine Liste der Zahlwoerter,
/// die nie mit null gerufen wuerden: das waere eine Behauptung ueber Rufer,
/// die keine Probe entscheiden kann. Die uebrigen Platzhalter bekommen einen
/// Wert ohne Ziffer, damit die 0 allein aus `{n}` kommen kann. Ein Zahlwort,
/// das in keiner Form `{n}` traegt (`HeimZettelUebernommen`), besteht ueber
/// den zweiten Zweig. Die Regel steht in `Sprache::form`.
#[test]
fn bei_null_sagt_keine_mengenangabe_eins() {
    for sprache in Sprache::ALLE {
        for schluessel in Zahlwort::ALLE {
            let (einzahl, mehrzahl) = sprache.zahlwort(schluessel);
            let mut namen = platzhalter(einzahl);
            namen.extend(platzhalter(mehrzahl));
            namen.remove("n");
            let werte: Vec<(&str, &dyn std::fmt::Display)> = namen
                .iter()
                .map(|name| (name.as_str(), &"x" as &dyn std::fmt::Display))
                .collect();
            let aus = sprache.anzahl(schluessel, 0, &werte);
            let als_mehrzahl = namen
                .iter()
                .fold(mehrzahl.replace("{n}", "0"), |text, name| {
                    text.replace(&format!("{{{name}}}"), "x")
                });
            assert!(
                aus.contains('0') || aus == als_mehrzahl,
                "{schluessel:?} sagt in {} bei null `{aus}`",
                sprache.kennung()
            );
        }
    }
}

/// Tausender je Sprache: Punkt, schmales geschuetztes Leerzeichen, Komma.
#[test]
fn die_tausender_je_sprache() {
    assert_eq!(Sprache::De.zahl(1_234_567), "1.234.567");
    assert_eq!(Sprache::Fr.zahl(1_234_567), "1\u{202f}234\u{202f}567");
    assert_eq!(Sprache::En.zahl(1_234_567), "1,234,567");
    for sprache in Sprache::ALLE {
        assert_eq!(sprache.zahl(0), "0");
        assert_eq!(sprache.zahl(999), "999");
    }
}

/// Datenmengen je Sprache: Dezimaltrenner und Einheit aus der Tabelle, unter
/// 1.000 Bytes das Zahlwort in Einzahl oder Mehrzahl.
#[test]
fn die_datenmenge_je_sprache() {
    assert_eq!(Sprache::De.menge(1_500), "1,5 kB");
    assert_eq!(Sprache::Fr.menge(1_500), "1,5 ko");
    assert_eq!(Sprache::En.menge(1_500), "1.5 kB");
    for sprache in Sprache::ALLE {
        let (einzahl, mehrzahl) = sprache.zahlwort(Zahlwort::Byte);
        assert_eq!(
            sprache.menge(1),
            einzahl.replace("{n}", "1"),
            "{}",
            sprache.kennung()
        );
        assert_eq!(
            sprache.menge(2),
            mehrzahl.replace("{n}", "2"),
            "{}",
            sprache.kennung()
        );
    }
}

/// `anzahl` nimmt fuer 1 die Einzahl und fuer eine groessere Zahl die
/// Mehrzahl mit gruppierter Zahl; `satz` ohne Platzhalter ist der Eintrag.
#[test]
fn anzahl_und_satz_an_der_tabelle() {
    assert_eq!(Sprache::De.anzahl(Zahlwort::Byte, 1, &[]), "1 Byte");
    assert_eq!(
        Sprache::De.anzahl(Zahlwort::Byte, 2_000, &[]),
        "2.000 Bytes"
    );
    assert_eq!(Sprache::Fr.anzahl(Zahlwort::Byte, 0, &[]), "0 octet");
    assert_eq!(Sprache::Fr.anzahl(Zahlwort::Ordner, 0, &[]), "0 dossiers");
    assert_eq!(Sprache::Fr.anzahl(Zahlwort::Eintraege, 0, &[]), "0 entrées");
    assert_eq!(
        Sprache::Fr.anzahl(Zahlwort::Eintraege, 1, &[]),
        "une entrée"
    );
    assert_eq!(Sprache::En.anzahl(Zahlwort::Byte, 0, &[]), "0 bytes");
    for sprache in Sprache::ALLE {
        assert_eq!(
            sprache.satz(Text::NameLeer, &[]),
            sprache.text(Text::NameLeer),
            "{}",
            sprache.kennung()
        );
    }
}

/// Ohne `festlegen` gilt Deutsch, und die Formen ohne Sprache antworten wie
/// die deutschen mit Sprache.
#[test]
fn ohne_festlegen_antworten_die_formen_ohne_sprache_deutsch() {
    assert_eq!(sprache::geltende(), Sprache::De);
    for schluessel in Text::ALLE {
        assert_eq!(sprache::text(schluessel), Sprache::De.text(schluessel));
    }
    assert_eq!(sprache::zahl(1_234_567), "1.234.567");
    assert_eq!(sprache::menge(1_500), "1,5 kB");
    assert_eq!(sprache::anzahl(Zahlwort::Byte, 3, &[]), "3 Bytes");
    assert_eq!(sprache::satz(Text::NameLeer, &[]), "der Name ist leer");
}
