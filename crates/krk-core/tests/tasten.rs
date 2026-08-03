//! Die Normalisierung der Modifikatoren und die verdrahtete Tastenzuordnung.
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

use krk_core::tasten::normalisierung::roh;
use krk_core::tasten::{Kommando, ModMaske, Tastendruck, code, kommando, normalisieren};

/// Der gemessene Tastencode von F3, aus `spikes/fn-tasten/messung-A.txt`.
const F3: u16 = 99;

/// Der Tastencode von `k`, fuer den Nachweis, dass zwei Bits zugleich halten.
const TASTE_K: u16 = 40;

#[test]
fn f3_mit_und_ohne_function_ergibt_dieselbe_nachschlagemaske() {
    // Die Messung vom 260802-1137 zeigt F3 immer mit gesetztem `function`,
    // gleich ob der Nutzer fn gehalten hat oder nicht. Der Nachschlag darf die
    // beiden deshalb nicht unterscheiden.
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
    assert_eq!(druck.maske.to_string(), "command+shift");
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
    let druck = Tastendruck::aus_ereignis(code::PFEIL_AB, roh_wie_appkit);

    assert_eq!(druck.maske, ModMaske::LEER);
    assert_eq!(kommando(druck), Some(Kommando::AuswahlRunter));
}

#[test]
fn die_fuenf_verdrahteten_tasten_liefern_ihr_kommando() {
    let erwartet = [
        (code::PFEIL_AUF, Kommando::AuswahlHoch),
        (code::PFEIL_AB, Kommando::AuswahlRunter),
        (code::BILD_AUF, Kommando::SeiteHoch),
        (code::BILD_AB, Kommando::SeiteRunter),
        (code::RETURN, Kommando::Oeffnen),
    ];
    for (taste, erwartetes) in erwartet {
        let druck = Tastendruck::neu(taste, ModMaske::LEER);
        assert_eq!(kommando(druck), Some(erwartetes));
    }
}

#[test]
fn eine_unbelegte_taste_liefert_kein_kommando() {
    assert_eq!(kommando(Tastendruck::neu(TASTE_K, ModMaske::LEER)), None);
    assert_eq!(kommando(Tastendruck::neu(F3, ModMaske::LEER)), None);
}

#[test]
fn eine_gehaltene_zusatztaste_nimmt_der_verdrahteten_taste_ihr_kommando() {
    // Umschalt+Pfeil ab gehoert spaeter der Bereichsauswahl aus C2 und darf
    // nicht schon jetzt wie ein nacktes Pfeil ab wirken.
    for (zusatz, _) in ModMaske::BENANNT {
        let druck = Tastendruck::neu(code::PFEIL_AB, zusatz);
        assert_eq!(kommando(druck), None, "Zusatztaste {zusatz} schlug durch");
    }
}
