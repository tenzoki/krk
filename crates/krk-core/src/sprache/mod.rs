//! Die Sprache der Oberflaeche: ein Wert je Prozess, drei Tabellen, und die
//! eine Stelle, an der ein nutzersichtbarer Text entsteht.
//!
//! KRK spricht Deutsch, Franzoesisch und Englisch; welche Sprache gilt,
//! entscheidet macOS aus der Sprachwahl je Programm, und `krk-ui` traegt die
//! Antwort beim Start ueber [`festlegen`] hierher. Der Kern stellt keinen
//! Systemaufruf und liest keine Datei: er bekommt die Sprache als Wert, und
//! sonst nichts (Spec
//! `261001-0735_*_spec-oberflaeche-lokalisierbar-deutsch-und-franzoesisch.md`,
//! `## Constraints`).
//!
//! # Die Regel
//!
//! **Ein Text, den ein Mensch durch KRKs Fenster, Blaetter, Menues oder die
//! Statuszeile liest, ist ein Eintrag aller drei Tabellen unter einem
//! ASCII-Schluessel, und er entsteht nirgends sonst.** Die Schluessel sind die
//! Aufzaehlungen [`Text`] und [`Zahlwort`] in [`schluessel`]; die Eintraege
//! stehen je Sprache in `tabelle/de.rs`, `tabelle/fr.rs` und `tabelle/en.rs`
//! als vollstaendige Fallunterscheidung ohne Auffangzweig. Daraus folgt vom
//! Uebersetzer und nicht von einer Probe: ein neuer Schluessel ohne
//! franzoesischen oder englischen Eintrag uebersetzt nicht, ein Eintrag ohne
//! Schluessel ebenso wenig, und ein Schluessel, zu dem eine Sprache keinen
//! Eintrag hat, ist kein Zustand, den das Programm erreichen kann. Was der
//! Uebersetzer nicht haelt, halten die Proben in
//! `crates/krk-core/tests/sprache.rs`: keinen leeren Eintrag, dieselben
//! Platzhalter in allen drei Sprachen, die Typografie je Sprache.
//!
//! Im Einzelnen:
//!
//! - **Die deutsche Tabelle traegt Umlaute; Kommentare und Bezeichner tragen
//!   die Umschrift.** Das ist die Umlautregel vom 260907
//!   (`shared/decisions/260826-1225_*_welche-schreibweise-gilt-fuer-nutzersichtbare-deutsche-meldungen-umlaut-oder-umschrift.md`),
//!   und die Tabellen sind seit dieser Arbeit die eine Stelle im Betriebscode,
//!   an der ein Umlaut in einem Stringliteral steht. Ein Schluessel ist ein
//!   Bezeichner und damit ASCII.
//! - **Ein Tabellentext traegt keine geschweifte Klammer ausser als
//!   Platzhalter.** `{name}` ist ein Platzhalter, den [`satz`] und [`anzahl`]
//!   einsetzen; die Reihenfolge der Platzhalter darf je Sprache verschieden
//!   sein, ihre Menge nicht. Ein [`Zahlwort`] traegt zwei Formen, Einzahl und
//!   Mehrzahl, und `{n}` steht fuer die gruppierte Zahl; die Einzahl darf
//!   `{n}` auslassen. Welche Form welche Zahl bekommt, sagt
//!   [`Sprache::mehrzahl`] je Sprache.
//! - **Kein `LazyLock` und kein `static` haelt einen Tabellentext.** Ein
//!   Text, der vor [`festlegen`] eingefroren wuerde, stuende in der
//!   Quellsprache, gleich was macOS gewaehlt hat. Gehalten wird der
//!   Schluessel, und der Text entsteht, wenn er gebraucht wird.
//! - **Terminalausgaben sind nicht Gegenstand.** Die `eprintln!`-Zeilen, der
//!   Messmodus, `--tasten-protokoll` und `--menue-protokoll` tragen die
//!   Umschrift und bleiben, wie die offene Entscheidung
//!   `shared/decisions/260907-0826_*_gilt-die-umlautregel-auch-fuer-die-terminalausgabe-von-xtask-krk-bench-und-messmodus.md`
//!   sie findet.
//!
//! # Die Sprache ist ein Wert des Prozesses
//!
//! Die Sprache wird genau einmal gesetzt, in `main` von `krk-ui`, vor dem
//! Laden der Belegung und vor dem Bau des Hauptmenues, und danach nie wieder;
//! macOS waehlt sie je Programm beim Start, und einen Wechsel zur Laufzeit
//! gibt es nicht. Sie ist deshalb kein Parameter: `fmt::Display` und
//! `From<Namensfehler> for io::Error` haben keinen Platz fuer einen, und
//! jede Stelle, die einen Kernfehler zu Text macht, muesste sonst einen Wert
//! halten, den sie heute nicht hat. [`geltende`] antwortet [`Sprache::De`],
//! solange nichts gesetzt ist, weil die Quelltabelle Deutsch ist und
//! `cargo test` nichts setzt; der Preis steht in `tests/sprache.rs`: eine
//! Probe kann die Sprache nicht umschalten, und die Eigenschaften der drei
//! Sprachen werden deshalb ueber die Formen mit ausdruecklicher Sprache
//! ([`Sprache::text`], [`Sprache::zahlwort`]) an den Tabellen geprueft.
//!
//! # Zahlen
//!
//! [`Sprache::zahl`] gruppiert Tausender (Deutsch Punkt, Franzoesisch ein
//! schmales geschuetztes Leerzeichen, Englisch Komma), [`Sprache::dezimal`]
//! trennt Dezimalen (Deutsch und Franzoesisch Komma, Englisch Punkt), und
//! [`Sprache::menge`] schreibt eine Bytemenge mit den Einheiten aus der
//! Tabelle. [`zahl`] stand bis zu dieser Arbeit in `ablage::neuerungen`,
//! [`menge`] in `krk_ui::kommandos::operationen`; beide sind Zahlformatierung
//! je Sprache und gehoeren deshalb hierher. Die Rufer greifen sie unter
//! denselben Namen.

use std::fmt;
use std::sync::OnceLock;

pub mod schluessel;
mod tabelle;

pub use schluessel::{Text, Zahlwort};

/// Die Sprachen, die KRK spricht.
///
/// Die Reihenfolge ist die von [`Sprache::ALLE`] und sagt nichts ueber einen
/// Vorrang: welche gilt, entscheidet macOS, und der Rueckfall fuer eine
/// nicht angebotene Sprache ist [`Sprache::En`], festgelegt am Buendel ueber
/// `CFBundleDevelopmentRegion` und hier ueber [`Sprache::aus_bezeichner`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sprache {
    /// Deutsch, die Quellsprache der Tabellen.
    De,
    /// Franzoesisch.
    Fr,
    /// Englisch, der Rueckfall.
    En,
}

/// Der eine Wert des Prozesses; siehe den Modulkopf.
static GELTENDE: OnceLock<Sprache> = OnceLock::new();

impl Sprache {
    /// Alle Sprachen, in der Reihenfolge der Aufzaehlung.
    ///
    /// Die Probe `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung`
    /// in `crates/krk-core/tests/baum.rs` haelt die Liste gegen die
    /// Aufzaehlung.
    pub const ALLE: [Sprache; 3] = [Sprache::De, Sprache::Fr, Sprache::En];

    /// Die Kennung, unter der macOS die Sprache nennt: `de`, `fr`, `en`.
    #[must_use]
    pub const fn kennung(self) -> &'static str {
        match self {
            Sprache::De => "de",
            Sprache::Fr => "fr",
            Sprache::En => "en",
        }
    }

    /// Die Sprache zu einem Bezeichner, wie `preferredLocalizations` ihn
    /// liefert.
    ///
    /// Gelesen wird der Teil vor dem ersten `-` oder `_`, ohne Ruecksicht auf
    /// Gross- und Kleinschreibung: `de-CH` und `de_AT` sind Deutsch, `fr-CA`
    /// ist Franzoesisch. Jeder andere Bezeichner, auch der leere, ist
    /// Englisch, der Rueckfall des Buendels.
    #[must_use]
    pub fn aus_bezeichner(bezeichner: &str) -> Sprache {
        let stamm = bezeichner
            .split(['-', '_'])
            .next()
            .expect("ein split liefert immer ein erstes Stueck");
        if stamm.eq_ignore_ascii_case(Sprache::De.kennung()) {
            Sprache::De
        } else if stamm.eq_ignore_ascii_case(Sprache::Fr.kennung()) {
            Sprache::Fr
        } else {
            Sprache::En
        }
    }

    /// Ob eine Zahl die Mehrzahlform bekommt.
    ///
    /// Deutsch und Englisch: alles ausser 1. Franzoesisch: alles ueber 1, die
    /// Null steht dort in der Einzahl.
    #[must_use]
    pub const fn mehrzahl(self, n: u64) -> bool {
        match self {
            Sprache::De | Sprache::En => n != 1,
            Sprache::Fr => n > 1,
        }
    }

    /// Das Zeichen zwischen zwei Tausendergruppen.
    const fn tausendertrenner(self) -> char {
        match self {
            Sprache::De => '.',
            Sprache::Fr => '\u{202F}',
            Sprache::En => ',',
        }
    }

    /// Das Zeichen zwischen Ganzen und Zehnteln.
    const fn dezimaltrenner(self) -> char {
        match self {
            Sprache::De | Sprache::Fr => ',',
            Sprache::En => '.',
        }
    }

    /// Eine ganze Zahl mit gruppierten Tausendern.
    #[must_use]
    pub fn zahl(self, wert: u64) -> String {
        gruppiert(&wert.to_string(), self.tausendertrenner())
    }

    /// Eine Zahl mit einer Nachkommastelle.
    ///
    /// Die Ganzen werden nicht gruppiert: der eine Rufer, [`Sprache::menge`],
    /// liefert hoechstens dreistellige Ganze, solange eine Einheit darueber
    /// steht.
    #[must_use]
    pub fn dezimal(self, ganze: u64, zehntel: u64) -> String {
        format!("{ganze}{}{zehntel}", self.dezimaltrenner())
    }

    /// Eine Datenmenge in der Schreibweise, die der Nutzer im Blatt liest.
    ///
    /// Dezimalpraefixe, wie der Finder sie zeigt, mit einer Nachkommastelle
    /// und der Einheit aus der Tabelle; unter 1.000 Bytes die Zahl mit dem
    /// [`Zahlwort::Byte`]. Die Tabelle im Dateifenster formatiert ueber
    /// `NSByteCountFormatter` und bleibt dabei; sie beschriftet eine Zelle
    /// fester Breite, und diese Zeile beschriftet einen Satz.
    #[must_use]
    pub fn menge(self, bytes: u64) -> String {
        const EINHEITEN: [(u64, Text); 4] = [
            (1_000_000_000_000, Text::EinheitTerabyte),
            (1_000_000_000, Text::EinheitGigabyte),
            (1_000_000, Text::EinheitMegabyte),
            (1_000, Text::EinheitKilobyte),
        ];
        for (teiler, einheit) in EINHEITEN {
            if bytes >= teiler {
                let ganze = bytes / teiler;
                let zehntel = (bytes % teiler) * 10 / teiler;
                return format!("{} {}", self.dezimal(ganze, zehntel), self.text(einheit));
            }
        }
        self.anzahl(Zahlwort::Byte, bytes, &[])
    }

    /// Der Eintrag zu einem Schluessel in dieser Sprache.
    #[must_use]
    pub const fn text(self, schluessel: Text) -> &'static str {
        match self {
            Sprache::De => tabelle::de::text(schluessel),
            Sprache::Fr => tabelle::fr::text(schluessel),
            Sprache::En => tabelle::en::text(schluessel),
        }
    }

    /// Einzahl und Mehrzahl zu einem Zahlwort in dieser Sprache.
    #[must_use]
    pub const fn zahlwort(self, schluessel: Zahlwort) -> (&'static str, &'static str) {
        match self {
            Sprache::De => tabelle::de::zahlwort(schluessel),
            Sprache::Fr => tabelle::fr::zahlwort(schluessel),
            Sprache::En => tabelle::en::zahlwort(schluessel),
        }
    }

    /// Ein Satz mit eingesetzten Platzhaltern in dieser Sprache.
    #[must_use]
    pub fn satz(self, schluessel: Text, werte: &[(&str, &dyn fmt::Display)]) -> String {
        eingesetzt(self.text(schluessel), werte)
    }

    /// Eine Mengenangabe in dieser Sprache: die Form nach
    /// [`Sprache::mehrzahl`], `{n}` als gruppierte Zahl, die uebrigen
    /// Platzhalter aus `werte`.
    #[must_use]
    pub fn anzahl(
        self,
        schluessel: Zahlwort,
        n: u64,
        werte: &[(&str, &dyn fmt::Display)],
    ) -> String {
        let (einzahl, mehrzahl) = self.zahlwort(schluessel);
        let vorlage = if self.mehrzahl(n) { mehrzahl } else { einzahl };
        let zahl = self.zahl(n);
        let mut alle: Vec<(&str, &dyn fmt::Display)> = Vec::with_capacity(werte.len() + 1);
        alle.push(("n", &zahl));
        alle.extend_from_slice(werte);
        eingesetzt(vorlage, &alle)
    }
}

/// Setzt die Sprache des Prozesses, genau einmal.
///
/// Der Rufer ist `main` in `krk-ui`, vor `starten`. Ein zweiter Aufruf
/// aendert nichts und antwortet `Err` mit dem Wert, der schon steht; den Fall
/// gibt es nicht, und der Rufer schliesst ihn aus, statt ihn zu behandeln.
pub fn festlegen(sprache: Sprache) -> Result<(), Sprache> {
    festlegen_in(&GELTENDE, sprache)
}

/// Der Rumpf von [`festlegen`] an einer hereingereichten Zelle, damit die
/// Probe ihn an einer eigenen Zelle fahren kann, ohne den Wert des Prozesses
/// zu setzen.
fn festlegen_in(zelle: &OnceLock<Sprache>, sprache: Sprache) -> Result<(), Sprache> {
    zelle.set(sprache).map_err(|_| {
        *zelle
            .get()
            .expect("set schlaegt allein fehl, wenn die Zelle schon gefuellt ist")
    })
}

/// Die Sprache des Prozesses.
///
/// [`Sprache::De`], solange [`festlegen`] nicht gerufen ist: die Quelltabelle
/// ist Deutsch, und `cargo test` setzt nichts. Jede Probe eines Testziels
/// sieht deshalb Deutsch, und keine ruft `festlegen`, weil sie damit jede
/// andere Probe desselben Ziels an ihren Wert baende.
#[must_use]
pub fn geltende() -> Sprache {
    GELTENDE.get().copied().unwrap_or(Sprache::De)
}

/// Der Eintrag zu einem Schluessel in der geltenden Sprache.
#[must_use]
pub fn text(schluessel: Text) -> &'static str {
    geltende().text(schluessel)
}

/// Ein Satz mit eingesetzten Platzhaltern in der geltenden Sprache.
///
/// `werte` nennt je Platzhalter seinen Namen ohne Klammern und seinen Wert;
/// die Reihenfolge ist gleichgueltig. Ein Platzhalter der Vorlage ohne Wert
/// bleibt stehen und haelt unter `debug_assertions` an; ein Wert ohne
/// Platzhalter ist zulaessig, weil eine Sprache einen Wert auslassen darf.
#[must_use]
pub fn satz(schluessel: Text, werte: &[(&str, &dyn fmt::Display)]) -> String {
    geltende().satz(schluessel, werte)
}

/// Eine Mengenangabe in der geltenden Sprache; siehe [`Sprache::anzahl`].
#[must_use]
pub fn anzahl(schluessel: Zahlwort, n: u64, werte: &[(&str, &dyn fmt::Display)]) -> String {
    geltende().anzahl(schluessel, n, werte)
}

/// Die eine Schreibweise fuer eine ganze Zahl in KRKs Oberflaeche, in der
/// geltenden Sprache.
///
/// Sie stand bis zum 260910 in `krk_ui::kommandos::operationen`, bis zu
/// dieser Arbeit in `ablage::neuerungen`, und traegt dieselbe Signatur wie
/// dort; der alte Ort in `krk-ui` holt sie ueber `pub(crate) use` zurueck.
#[must_use]
pub fn zahl(wert: usize) -> String {
    gruppiert(&wert.to_string(), geltende().tausendertrenner())
}

/// Eine Datenmenge in der geltenden Sprache; siehe [`Sprache::menge`].
#[must_use]
pub fn menge(bytes: u64) -> String {
    geltende().menge(bytes)
}

/// Eine Ziffernfolge mit einem Trenner vor jeder Dreiergruppe.
fn gruppiert(ziffern: &str, trenner: char) -> String {
    let mut aus = String::with_capacity(ziffern.len() + ziffern.len() / 3 * trenner.len_utf8());
    for (stelle, ziffer) in ziffern.chars().enumerate() {
        if stelle > 0 && (ziffern.len() - stelle).is_multiple_of(3) {
            aus.push(trenner);
        }
        aus.push(ziffer);
    }
    aus
}

/// Setzt die Platzhalter `{name}` einer Vorlage ein.
///
/// Die Vorlage wird einmal von links nach rechts gelesen; ein eingesetzter
/// Wert wird nicht ein zweites Mal gelesen, also kann ein Dateiname mit
/// geschweiften Klammern keinen Platzhalter vortaeuschen. Ein Platzhalter
/// ohne Wert bleibt stehen und haelt unter `debug_assertions` an, weil er
/// ein Schluessel ist, dessen Rufer einen Wert vergessen hat.
fn eingesetzt(vorlage: &str, werte: &[(&str, &dyn fmt::Display)]) -> String {
    let mut aus = String::with_capacity(vorlage.len() + 16);
    let mut rest = vorlage;
    while let Some(anfang) = rest.find('{') {
        aus.push_str(&rest[..anfang]);
        let nach = &rest[anfang + 1..];
        let Some(ende) = nach.find('}') else {
            debug_assert!(
                false,
                "die Vorlage `{vorlage}` oeffnet einen Platzhalter und schliesst ihn nicht"
            );
            aus.push_str(&rest[anfang..]);
            return aus;
        };
        let name = &nach[..ende];
        match werte.iter().find(|(gesucht, _)| *gesucht == name) {
            Some((_, wert)) => aus.push_str(&wert.to_string()),
            None => {
                debug_assert!(
                    false,
                    "die Vorlage `{vorlage}` traegt den Platzhalter {{{name}}} ohne Wert"
                );
                aus.push_str(&rest[anfang..=anfang + 1 + ende]);
            }
        }
        rest = &nach[ende + 1..];
    }
    aus.push_str(rest);
    aus
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die Mechanik des Einsetzens an freien Vorlagen: zwei Platzhalter in
    /// umgekehrter Reihenfolge, ein Wert ohne Platzhalter, ein Wert mit
    /// geschweiften Klammern. Die Tabelleneintraege dieses Schrittes tragen
    /// keinen Platzhalter; die Mechanik wird deshalb hier an Vorlagen
    /// geprueft und nicht in `tests/sprache.rs` an einem Schluessel.
    #[test]
    fn das_einsetzen_folgt_den_namen_und_nicht_der_reihenfolge() {
        let name = "bericht.txt";
        let zahl = 3;
        assert_eq!(
            eingesetzt("{zahl} mal {name}", &[("name", &name), ("zahl", &zahl)]),
            "3 mal bericht.txt"
        );
        assert_eq!(
            eingesetzt("ohne Platzhalter", &[("name", &name)]),
            "ohne Platzhalter"
        );
        let klammern = "{zahl}.txt";
        assert_eq!(
            eingesetzt("{name} und {zahl}", &[("name", &klammern), ("zahl", &zahl)]),
            "{zahl}.txt und 3"
        );
    }

    /// Ein Platzhalter ohne Wert haelt im Pruefbau an.
    #[test]
    #[should_panic(expected = "ohne Wert")]
    fn ein_platzhalter_ohne_wert_haelt_an() {
        let _ = eingesetzt("{name}", &[]);
    }

    /// `anzahl` nimmt fuer 1 die Einzahl, auch eine ohne `{n}`, und fuer 2
    /// die Mehrzahl mit gruppierter Zahl. Gefahren an der Mechanik, weil
    /// kein Zahlwort dieses Schrittes eine Einzahl ohne `{n}` traegt.
    #[test]
    fn die_einzahl_darf_n_auslassen_und_die_mehrzahl_traegt_die_zahl() {
        let (einzahl, mehrzahl) = ("Einen Eintrag umbenennen", "{n} Einträge umbenennen");
        let mit = |sprache: Sprache, n: u64| {
            let vorlage = if sprache.mehrzahl(n) {
                mehrzahl
            } else {
                einzahl
            };
            let zahl = sprache.zahl(n);
            eingesetzt(vorlage, &[("n", &zahl)])
        };
        assert_eq!(mit(Sprache::De, 1), "Einen Eintrag umbenennen");
        assert_eq!(mit(Sprache::De, 2000), "2.000 Einträge umbenennen");
        assert_eq!(mit(Sprache::Fr, 0), "Einen Eintrag umbenennen");
        assert_eq!(mit(Sprache::En, 0), "0 Einträge umbenennen");
    }

    /// Der zweite Aufruf von `festlegen` aendert nichts und nennt den ersten
    /// Wert. Gefahren an einer eigenen Zelle, damit der Wert des Prozesses
    /// in keinem Testziel gesetzt wird.
    #[test]
    fn der_zweite_aufruf_von_festlegen_nennt_den_ersten_wert() {
        let zelle = OnceLock::new();
        assert_eq!(festlegen_in(&zelle, Sprache::Fr), Ok(()));
        assert_eq!(festlegen_in(&zelle, Sprache::En), Err(Sprache::Fr));
        assert_eq!(zelle.get(), Some(&Sprache::Fr));
    }

    /// `geltende()` ist Deutsch, solange nichts gesetzt ist; und in diesem
    /// Ziel setzt nichts.
    #[test]
    fn ohne_festlegen_gilt_deutsch() {
        assert_eq!(geltende(), Sprache::De);
        assert_eq!(zahl(1_234_567), "1.234.567");
    }
}
