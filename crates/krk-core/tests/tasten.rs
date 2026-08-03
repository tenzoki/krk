//! Die Normalisierung der Modifikatoren.
//!
//! Alles hier laeuft ohne Fenster und ohne AppKit. Das ist der Grund, aus dem
//! die Normalisierung im Kern wohnt und nicht im Ereignisabgriff: sie ist die
//! Regel, an der die Abnahme von C3 haengt, und sie waere hinter einem Fenster
//! nur von Hand pruefbar.
//!
//! Die rohen Bitwerte kommen aus `krk_core::tasten::normalisierung::roh` und
//! stehen hier nicht ein zweites Mal. Sie sind Teil der binaeren Schnittstelle
//! von AppKit; sie hier als Zahlen zu wiederholen hiesse, zwei Wahrheiten zu
//! fuehren.
//!
//! **Der Nachschlag steht nicht mehr hier.** Bis Schritt 10 prueften diese
//! Zeilen die fest verdrahtete Tabelle aus Schritt 7 mit; Schritt 11 hat sie
//! abgeloest, und was an ihre Stelle getreten ist, prueft
//! `crates/krk-core/tests/belegung.rs`.

use krk_core::tasten::normalisierung::roh;
use krk_core::tasten::{ModMaske, Tastendruck, code_von_pflicht, normalisieren};

/// Der gemessene Tastencode von F3, aus `spikes/fn-tasten/messung-A.txt`.
///
/// Die Zahl steht nicht hier. Sie kommt aus der einen Tastentabelle des Kerns,
/// die sie zugleich als gemessen ausweist; sie abzuschreiben hiesse, sie an
/// zwei Stellen zu fuehren.
const F3: u16 = code_von_pflicht("f3");

/// Der Tastencode von `k`, fuer den Nachweis, dass zwei Bits zugleich halten.
const TASTE_K: u16 = code_von_pflicht("k");

/// Der Tastencode von Pfeil ab.
const PFEIL_AB: u16 = code_von_pflicht("down");

#[test]
fn f3_mit_und_ohne_function_ergibt_dieselbe_nachschlagemaske() {
    // C3 verlangt, dass fn keine Zusatztaste einer Belegung ist: der
    // Nachschlag darf die beiden Faelle nicht unterscheiden. Gemessen ist am
    // 260802-1137 nur der eine von ihnen, F3 mit gehaltener fn
    // (`spikes/fn-tasten/messung-A.txt:17-19`); der andere ist am
    // Referenzgeraet nicht messbar. Der Modulkopf von
    // `krk-core/src/tasten/normalisierung.rs` schreibt beides aus. Diese
    // Pruefung haengt nicht daran, welches Ereignis ein nacktes F3 erzeugt:
    // sie deckt beide ab.
    let mit_function = Tastendruck::aus_ereignis(F3, roh::FUNKTION);
    let ohne_function = Tastendruck::aus_ereignis(F3, 0);

    assert_eq!(mit_function.maske, ohne_function.maske);
    assert_eq!(mit_function, ohne_function);
    assert_eq!(mit_function.maske, ModMaske::LEER);
}

#[test]
fn cmd_shift_k_behaelt_beide_bits() {
    let druck = Tastendruck::aus_ereignis(TASTE_K, roh::BEFEHL | roh::UMSCHALT);

    assert!(druck.maske.enthaelt(ModMaske::BEFEHL));
    assert!(druck.maske.enthaelt(ModMaske::UMSCHALT));
    assert!(!druck.maske.enthaelt(ModMaske::STEUERUNG));
    assert!(!druck.maske.enthaelt(ModMaske::WAHL));
    assert_eq!(druck.maske, ModMaske::BEFEHL | ModMaske::UMSCHALT);
    assert_eq!(druck.maske.to_string(), "shift+cmd");
}

#[test]
fn jedes_der_vier_bits_kommt_einzeln_durch() {
    let erwartet = [
        (roh::BEFEHL, ModMaske::BEFEHL),
        (roh::STEUERUNG, ModMaske::STEUERUNG),
        (roh::WAHL, ModMaske::WAHL),
        (roh::UMSCHALT, ModMaske::UMSCHALT),
    ];
    for (rohes_bit, maske) in erwartet {
        assert_eq!(normalisieren(rohes_bit), maske);
    }
}

#[test]
fn function_feststelltaste_zehnerblock_und_hilfe_fallen_weg() {
    let geloescht = [
        roh::FUNKTION,
        roh::FESTSTELLTASTE,
        roh::ZEHNERBLOCK,
        roh::HILFE,
    ];
    for rohes_bit in geloescht {
        assert_eq!(normalisieren(rohes_bit), ModMaske::LEER);
        // Auch neben einem behaltenen Bit darf das geloeschte nichts hinzufuegen.
        assert_eq!(normalisieren(rohes_bit | roh::BEFEHL), ModMaske::BEFEHL);
    }
    let alle_geloescht = geloescht.into_iter().fold(0, |sammel, bit| sammel | bit);
    assert_eq!(normalisieren(alle_geloescht), ModMaske::LEER);
}

#[test]
fn ein_pfeil_mit_gesetztem_function_und_zehnerblock_bleibt_ein_nackter_pfeil() {
    // AppKit setzt bei den Pfeiltasten beide Bits. Kaeme eines davon in die
    // Maske, faende der Nachschlag das Kommando nicht.
    let roh_wie_appkit = roh::FUNKTION | roh::ZEHNERBLOCK;
    let druck = Tastendruck::aus_ereignis(PFEIL_AB, roh_wie_appkit);

    assert_eq!(druck.maske, ModMaske::LEER);
}
