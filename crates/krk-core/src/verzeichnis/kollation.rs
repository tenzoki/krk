//! Der sprachsensitive Vergleich, als Bytefolge.
//!
//! Ein Dateimanager mit deutschsprachiger Oberflaeche muss `Aepfel` zwischen
//! `Apfel` und `Baeume` einordnen und nicht hinter `Zebra`. Der Unterschied
//! ist keine Feinheit der Darstellung: eine Ordnung nach Unicode-Position
//! stellt jeden Namen mit Umlaut an eine Stelle, an der ihn niemand sucht.
//! Der Finder ordnet sprachsensitiv, und KRK tut es seit dem Nutzerentscheid
//! vom 260806 auch (`decisions/260802-1810_*_sortierung-ohne-sprachsensitive-kollation.md`).
//!
//! # Warum ein Schluessel und kein Vergleich
//!
//! Die uebliche Form einer Kollation ist eine Vergleichsfunktion: sie nimmt
//! zwei Zeichenketten und liefert ihre Reihenfolge. Diese Form passt hier
//! nicht. Ein Sortierlauf ueber 100.000 Eintraege ruft die Vergleichsfunktion
//! rund 1,7 Millionen Mal; die Kollation liefe damit siebzehnmal je Eintrag
//! statt einmal.
//!
//! [`icu_collator`] kann den Vergleich stattdessen als **Bytefolge**
//! ausschreiben: zwei so gebaute Schluessel bytweise verglichen ergeben
//! dieselbe Reihenfolge wie der sprachsensitive Vergleich der Namen. Damit
//! bleibt der Zuschnitt aus Schritt 2 unveraendert — der Schluessel entsteht
//! einmal beim Lesen in [`super::Eintrag::neu`], und das Sortieren vergleicht
//! nur noch Bytes. Es gibt genau einen Sortierweg, keinen schnellen neben
//! einem richtigen.
//!
//! # Welche Ordnung
//!
//! Die Wurzelordnung von CLDR, ohne Anpassung an eine einzelne Sprache. Fuer
//! Deutsch ist das die erwartete Ordnung: die CLDR-Anpassung `de` aendert an
//! der Wurzel nichts, sie unterscheidet sich erst in der Sonderform
//! `de-u-co-phonebk` (Telefonbuch, `ae` fuer `ä`).
//!
//! Der Nutzer bekommt damit **eine** Ordnung, unabhaengig von seinen
//! Systemeinstellungen. Das ist eine Festlegung und keine Auslassung: in
//! Schwedisch etwa steht `ä` hinter `z`, und wer der Systemsprache folgen
//! wollte, muesste die Ordnung von aussen hereinreichen — die Systemsprache
//! liegt in Foundation und damit in `krk-ui`. Das waere ein eigener Entscheid;
//! dieser Datensatz verlangt ihn nicht.

use std::sync::OnceLock;

use icu_collator::options::CollatorOptions;
use icu_collator::{Collator, CollatorBorrowed, CollatorPreferences};

/// Der Kollator, einmal je Prozess.
///
/// Ihn je Eintrag neu zu bauen kostete bei 100.000 Eintraegen rund 20 ms fuer
/// nichts; er traegt keinen veraenderlichen Zustand und laesst sich deshalb
/// teilen.
static KOLLATOR: OnceLock<CollatorBorrowed<'static>> = OnceLock::new();

fn kollator() -> &'static CollatorBorrowed<'static> {
    KOLLATOR.get_or_init(|| {
        Collator::try_new(CollatorPreferences::default(), CollatorOptions::default())
            .expect("die CLDR-Wurzelordnung ist ueber `compiled_data` mit einkompiliert")
    })
}

/// Baut den Kollationsschluessel eines Textes.
///
/// Zwei Schluessel bytweise verglichen ergeben dieselbe Reihenfolge wie der
/// sprachsensitive Vergleich der beiden Texte. Die Ordnung unterscheidet auch
/// Gross- und Kleinschreibung, aber erst nachrangig: `datei` und `Datei`
/// stehen beieinander und in fester Reihenfolge.
///
/// Kanonisch gleiche Namen ergeben denselben Schluessel. Das ist auf einem Mac
/// kein Randfall: `ä` kommt als ein Zeichen und als `a` mit
/// Kombinationszeichen vor, je nachdem, welches Programm die Datei angelegt
/// hat. Beide Schreibweisen sind derselbe Name und sortieren an derselben
/// Stelle.
///
/// Der leere Text ergibt den leeren Schluessel, und der belegt keinen Speicher.
/// Ohne diesen Fall traegt jede Datei ohne Endung eine Zuteilung von vier Bytes
/// fuer nichts.
pub fn schluessel(text: &str) -> Box<[u8]> {
    if text.is_empty() {
        return Box::default();
    }
    // Der Schluessel faellt in der Regel kuerzer aus als der Text, weil ein
    // Zeichen mit ein bis zwei Bytes je Ebene auskommt. Die Zugabe deckt die
    // drei Ebenentrenner.
    let mut bytes = Vec::with_capacity(text.len() + 4);
    // Der Fehlertyp von `Vec<u8>` als Senke ist unbewohnt: eine Zuteilung
    // schlaegt nicht fehl, sie geraet in Panik. Es gibt hier also keinen
    // Fehlerfall, den dieses Modul verschlucken koennte.
    let Ok(()) = kollator().write_sort_key_to(text, &mut bytes);
    bytes.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn umlaute_stehen_beim_grundbuchstaben_und_nicht_hinter_z() {
        assert!(schluessel("Apfel") < schluessel("Äpfel"));
        assert!(schluessel("Äpfel") < schluessel("Bäume"));
        assert!(schluessel("Bäume") < schluessel("Zebra"));
    }

    #[test]
    fn die_schreibung_entscheidet_erst_nachrangig() {
        // Gross und klein stehen beieinander ...
        assert!(schluessel("datei") < schluessel("egal"));
        assert!(schluessel("Datei") < schluessel("egal"));
        // ... aber unterscheidbar, damit die Ordnung total bleibt.
        assert_ne!(schluessel("Datei"), schluessel("datei"));
    }

    #[test]
    fn kanonisch_gleiche_namen_ergeben_denselben_schluessel() {
        // Einmal als ein Zeichen, einmal als `a` mit Kombinationszeichen.
        assert_eq!(schluessel("\u{e4}"), schluessel("a\u{308}"));
    }

    #[test]
    fn der_leere_text_ergibt_den_leeren_schluessel() {
        assert!(schluessel("").is_empty());
    }

    #[test]
    fn der_leere_schluessel_steht_vor_jedem_anderen() {
        // Darauf beruht, dass Dateien ohne Endung in der Sortierung nach Typ
        // vor allen anderen stehen.
        assert!(schluessel("") < schluessel("a"));
        assert!(schluessel("") < schluessel("-"));
        assert!(schluessel("") < schluessel(" "));
    }

    #[test]
    fn kein_schluessel_traegt_ein_nullbyte() {
        // Der Sortierschluessel aus Schritt 2 trennte seine beiden Teile mit
        // einem Nullbyte. Dass ein Kollationsschluessel keines enthaelt, ist
        // die Eigenschaft, die einen solchen Aufbau ueberhaupt zuliesse.
        for probe in ["Äpfel-x.txt", "a", "ÄÖÜäöüß", "名前", "\u{1F600}"] {
            assert!(!schluessel(probe).contains(&0), "{probe}");
        }
    }
}
