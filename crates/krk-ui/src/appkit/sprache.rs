//! Die eine Lesung der Sprache, die macOS fuer KRK gewaehlt hat.
//!
//! ```text
//! NSBundle::mainBundle ──> preferredLocalizations ──> erster Eintrag
//!                                                        │
//!                                  Sprache::aus_bezeichner, ohne Eintrag En
//! ```
//!
//! KRK trifft keine eigene Sprachwahl und bietet keine an: welche der drei
//! Sprachen gilt, entscheidet macOS aus der Sprachwahl je Programm
//! (Systemeinstellungen, „Sprache & Region“, „Apps“), geschnitten gegen
//! `CFBundleLocalizations` und `CFBundleDevelopmentRegion` der `Info.plist`
//! und die `.lproj`-Ordner des Buendels. Diese Datei liest die Antwort und
//! sonst nichts; `main` traegt sie ueber `krk_core::sprache::festlegen` in den
//! Kern, einmal je Prozess und vor `starten`.
//!
//! # Was macOS antwortet, gemessen am 261001
//!
//! Gemessen an Scratch-Buendeln, die sich allein in `CFBundleLocalizations`,
//! `CFBundleDevelopmentRegion` und den `.lproj`-Ordnern unterscheiden, mit der
//! Sprachwahl je Buendelkennung ueber `AppleLanguages`. Die Spalte ist der
//! erste Eintrag von `preferredLocalizations` fuer die Liste `de, fr, en`:
//!
//! ```text
//! Sprachwahl          Region en   Region de
//! de-DE               de          de
//! de-CH               de          de
//! fr-CA               fr          fr
//! fr-FR, de-DE        fr          fr
//! en-US, de-DE        en          en
//! en-GB               en          en
//! ja                  en          de
//! pt-BR, es-ES        en          de
//! ```
//!
//! Drei Befunde tragen diese Datei. Der erste Eintrag ist immer eine der
//! angebotenen Sprachen ohne Regionszusatz (`de-CH` kommt als `de`, `fr-CA`
//! als `fr`), die Abbildung auf die Tabelle ist also eine Gleichheit, und
//! `Sprache::aus_bezeichner` schneidet den Zusatz nur fuer den Fall ab, dass
//! eine spaetere macOS-Fassung ihn doch mitgibt. Den Rueckfall fuer eine nicht
//! angebotene Sprache entscheidet `CFBundleDevelopmentRegion`, auch wenn `en`
//! in der Liste steht; die Region steht deshalb auf `en`. Mit `.lproj`-Ordnern
//! fuehrt `preferredLocalizations` die gewaehlte Sprache zweimal, einmal aus
//! dem Schluessel und einmal aus dem Ordner; das ist unschaedlich, gelesen
//! wird der erste Eintrag.
//!
//! **Ein Binaerprogramm ausserhalb eines Buendels bekommt von macOS `en`**
//! (`localizations` ist leer, `preferredLocalizations` ist `["en"]`, waehrend
//! `Locale.preferredLanguages` die Systemsprache nennt), und KRK folgt dem,
//! weil es die Wahl von macOS liest und keine eigene daneben trifft. Jeder
//! vorgesehene Startweg (`make run`, `make tasten`, `make menue`, die
//! Messstrecke) startet das Buendel; wer `target/release/krk` von Hand ruft,
//! sieht die Oberflaeche englisch, und das ist diese Regel und kein Defekt.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSBundle`, `NSArray` und `NSString` stehen seit macOS 10.0 zur
//! Verfuegung, ebenso `mainBundle`, `preferredLocalizations` und
//! `firstObject` (`NSBundle.h`, `NSArray.h`, ohne eigene
//! Verfuegbarkeitsangabe). Das Buendel zielt auf 15.0 (`.cargo/config.toml`);
//! keine Beruehrung dieser Datei ist nach macOS 15 hinzugekommen, und keine
//! braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt
//! keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die
//! Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.

use krk_core::sprache::Sprache;
use objc2_foundation::NSBundle;

/// Die Sprache, die macOS fuer dieses Programm gewaehlt hat.
///
/// Der erste Eintrag von `preferredLocalizations` des Hauptbuendels, ueber
/// [`Sprache::aus_bezeichner`] auf die Tabelle abgebildet; ohne Eintrag
/// [`Sprache::En`]. Den leeren Fall hat die Messung nie gesehen, auch nicht
/// ausserhalb eines Buendels, und die Antwort darauf ist dieselbe wie auf
/// jede nicht angebotene Sprache.
#[must_use]
pub fn vom_system() -> Sprache {
    let erster = NSBundle::mainBundle()
        .preferredLocalizations()
        .firstObject()
        .map(|eintrag| eintrag.to_string());
    aus_erstem_eintrag(erster.as_deref())
}

/// Der Rumpf von [`vom_system`] ohne den Systemaufruf, damit die Probe ihn
/// an beiden Faellen fahren kann.
#[must_use]
fn aus_erstem_eintrag(erster: Option<&str>) -> Sprache {
    erster.map_or(Sprache::En, Sprache::aus_bezeichner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn der_erste_eintrag_wird_auf_die_tabelle_abgebildet() {
        assert_eq!(aus_erstem_eintrag(Some("de")), Sprache::De);
        assert_eq!(aus_erstem_eintrag(Some("fr")), Sprache::Fr);
        assert_eq!(aus_erstem_eintrag(Some("en")), Sprache::En);
        assert_eq!(aus_erstem_eintrag(Some("de-CH")), Sprache::De);
        assert_eq!(aus_erstem_eintrag(Some("ja")), Sprache::En);
    }

    #[test]
    fn ohne_eintrag_gilt_englisch() {
        assert_eq!(aus_erstem_eintrag(None), Sprache::En);
    }
}
