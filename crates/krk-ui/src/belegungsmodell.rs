//! Der Inhalt der Belegungsansicht aus C3, ohne AppKit.
//!
//! **Die Ansicht fuehrt keine eigene Tabelle.** Dieses Modul haelt waehrend
//! einer offenen Belegungsansicht genau eine [`Belegung`] — die Arbeitskopie —
//! und reicht jede Frage an sie weiter: die Zeilen sind ihre Funktionen, die
//! Zuweisung geht ueber [`Belegung::zuweisen`], das Zuruecksetzen ueber
//! [`Belegung::zuruecksetzen`], und die Konfliktmeldung kommt woertlich aus
//! `krk_core::tasten::konflikt`. Was hier dazukommt, ist allein die
//! Anzeigeform und das Kennzeichen, ob sich etwas geaendert hat.
//!
//! ```text
//! Kommando::BelegungAnsehen ──> Belegungsmodell (Arbeitskopie der Belegung)
//!                                    │  zuweisen / zuruecksetzen
//!                                    ▼
//!            beim Verlassen: in_belegung ──> Belegung::sichern (keymap.toml)
//! ```
//!
//! # Eine Zeile je Funktion
//!
//! C3 verlangt: genau eine Zeile je Funktion, alle Kombinationen dieser
//! Funktion in dieser einen Zeile. Das ist hier keine Rechenleistung, sondern
//! die Gestalt der Belegung selbst: [`Belegung::funktionen`] fuehrt jede
//! Funktion genau einmal, mit allen ihren Kombinationen. Das Modell zaehlt
//! sie ab und erfindet keine zweite Ordnung.
//!
//! # Die Beschriftung geht ueber die Tastentabelle
//!
//! Eine Kombination schreibt sich ueber ihre [`fmt::Display`]-Form
//! (`shift+cmd+k`), und die kennt allein die Namen aus
//! `krk_core::tasten::parser::TASTEN`. Die Anzeigeform [`anzeige`] setzt
//! darauf nur Grossbuchstaben an den Teilanfang: `Shift+Cmd+K`, `F3`. Eine
//! zweite Namensliste entsteht nicht, und "Fn+" kann an keiner Stelle
//! erscheinen, weil die Schreibweise fn nicht kennt (C3, S7).

use krk_core::tasten::{Belegung, Kombination, Tastendruck};

/// Was aus dem Versuch geworden ist, der ausgewaehlten Funktion die gedrueckte
/// Kombination zu geben.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zuweisung {
    /// Die Funktion traegt die Kombination jetzt. Beide Namen stehen in der
    /// Anzeigeform fuer die Bestaetigung.
    Zugewiesen {
        /// Die Beschriftung der Funktion.
        funktion: String,
        /// Die Kombination in der Anzeigeform.
        kombination: String,
    },
    /// Die gedrueckte Taste hat in der Schreibweise keinen Namen (Satzzeichen,
    /// Zehnerblock) und liesse sich nicht wieder aus `keymap.toml` einlesen.
    OhneNamen,
    /// Die Belegung hat die Zuweisung abgewiesen; der Text nennt den Grund
    /// und kommt woertlich aus dem Kern, samt der anderen Funktion bei einem
    /// Konflikt.
    Abgelehnt(String),
}

/// Die Arbeitskopie der Belegung, solange die Ansicht offen ist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Belegungsmodell {
    belegung: Belegung,
    /// Ob eine Zuweisung oder ein Zuruecksetzen gelungen ist. Nur dann wird
    /// beim Verlassen gesichert; eine unveraenderte Ansicht schreibt nichts.
    geaendert: bool,
}

impl Belegungsmodell {
    /// Ein Modell ueber der uebergebenen Belegung, ohne Aenderung.
    pub fn neu(belegung: Belegung) -> Self {
        Self {
            belegung,
            geaendert: false,
        }
    }

    /// Wie viele Zeilen die Ansicht fuehrt: eine je Funktion.
    pub fn zeilen(&self) -> usize {
        self.belegung.funktionen().len()
    }

    /// Die Beschriftung der Funktion an dieser Stelle, fuer die Spalte
    /// "Funktion".
    ///
    /// Eine reservierte Funktion traegt den Vorbehalt im Text, wie C3 es fuer
    /// den F4-Eintrag verlangt; eine vom Hauptmenue zugestellte den Zusteller,
    /// damit die beiden Cmd+A-Zeilen unterscheidbar sind.
    pub fn funktionstext(&self, stelle: usize) -> Option<String> {
        let funktion = self.belegung.funktionen().get(stelle)?;
        let mut text = funktion.name().to_owned();
        if let Some(wofuer) = funktion.reserviert_fuer() {
            let wofuer = match wofuer {
                "editor" => "den Editor",
                andere => andere,
            };
            text.push_str(&format!(" (reserviert für {wofuer})"));
        }
        if let Some(zusteller) = funktion.gehalten_von() {
            let zusteller = match zusteller {
                "menue" => "Kürzel des Menüs",
                andere => andere,
            };
            text.push_str(&format!(" ({zusteller})"));
        }
        Some(text)
    }

    /// Alle Kombinationen der Funktion an dieser Stelle, in der Anzeigeform,
    /// fuer die Spalte "Belegung".
    pub fn tastentext(&self, stelle: usize) -> Option<String> {
        let funktion = self.belegung.funktionen().get(stelle)?;
        Some(
            funktion
                .tasten()
                .iter()
                .map(anzeige)
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    /// Der blosse Name der Funktion an dieser Stelle, fuer die Aufforderung
    /// waehrend der Aufnahme.
    pub fn name(&self, stelle: usize) -> Option<&str> {
        self.belegung
            .funktionen()
            .get(stelle)
            .map(|funktion| funktion.name())
    }

    /// Gibt der Funktion an dieser Stelle die gedrueckte Kombination.
    ///
    /// Der Tastendruck kommt normalisiert herein; zwei Druecke, die sich
    /// allein durch gehaltenes fn unterscheiden, sind hier schon derselbe
    /// (S7), und eine fn-Kombination ist deshalb nicht anlegbar.
    pub fn zuweisen(&mut self, stelle: usize, druck: Tastendruck) -> Zuweisung {
        let Some(kombination) = Kombination::aus_tastendruck(druck) else {
            return Zuweisung::OhneNamen;
        };
        let Some(funktion) = self.belegung.funktionen().get(stelle) else {
            return Zuweisung::Abgelehnt("es ist keine Funktion ausgewählt".to_owned());
        };
        let kennung = funktion.kennung().to_owned();
        let name = funktion.name().to_owned();
        match self.belegung.zuweisen(&kennung, kombination) {
            Ok(()) => {
                self.geaendert = true;
                Zuweisung::Zugewiesen {
                    funktion: name,
                    kombination: anzeige(&kombination),
                }
            }
            Err(fehler) => Zuweisung::Abgelehnt(fehler.to_string()),
        }
    }

    /// Setzt die Arbeitskopie auf den Auslieferungszustand zurueck (C3).
    pub fn zuruecksetzen(&mut self) {
        self.belegung.zuruecksetzen();
        self.geaendert = true;
    }

    /// Ob beim Verlassen zu sichern ist.
    pub fn geaendert(&self) -> bool {
        self.geaendert
    }

    /// Die Arbeitskopie, fuer das Sichern und den weiteren Betrieb.
    pub fn in_belegung(self) -> Belegung {
        self.belegung
    }
}

impl Default for Belegungsmodell {
    /// Die Auslieferungsbelegung, unveraendert.
    ///
    /// Der Wert, den `RefCell::take` beim Abschliessen der Ansicht
    /// zuruecklaesst; er wird nie angezeigt und nie gesichert.
    fn default() -> Self {
        Self::neu(Belegung::auslieferung())
    }
}

/// Die Anzeigeform einer Kombination: die Schreibweise mit grossem
/// Teilanfang.
///
/// `shift+cmd+k` wird zu `Shift+Cmd+K`, `f3` zu `F3`. Mehr geschieht nicht:
/// die Namen kommen aus der einen Tastentabelle des Kerns, und eine
/// Uebersetzungsliste daneben waere die zweite Namensliste, die der Plan
/// ausschliesst.
pub fn anzeige(kombination: &Kombination) -> String {
    kombination
        .to_string()
        .split('+')
        .map(teilanfang_gross)
        .collect::<Vec<String>>()
        .join("+")
}

/// Der Teil mit grossem ersten Buchstaben.
fn teilanfang_gross(teil: &str) -> String {
    let mut zeichen = teil.chars();
    match zeichen.next() {
        Some(erstes) => erstes.to_ascii_uppercase().to_string() + zeichen.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use krk_core::tasten::normalisierung::{ModMaske, roh};
    use krk_core::tasten::{Kommando, code_von_pflicht};

    use super::*;

    /// Das Modell zaehlt die Funktionen der Belegung ab: eine Zeile je
    /// Funktion, und Papierkorb und endgueltiges Loeschen sind zwei davon.
    #[test]
    fn eine_zeile_je_funktion() {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        assert_eq!(modell.zeilen(), Belegung::auslieferung().funktionen().len());

        let namen: Vec<String> = (0..modell.zeilen())
            .map(|stelle| {
                modell
                    .funktionstext(stelle)
                    .expect("jede Zeile hat einen Text")
            })
            .collect();
        let mut sortiert = namen.clone();
        sortiert.sort();
        sortiert.dedup();
        assert_eq!(namen.len(), sortiert.len(), "eine Funktion steht zweimal");

        let belegung = Belegung::auslieferung();
        assert!(belegung.funktion("in_papierkorb").is_some());
        assert!(belegung.funktion("endgueltig_loeschen").is_some());
    }

    /// Kein Zeilentext der Ansicht schreibt "Fn+" vor eine Kombination, und
    /// die Funktionstasten erscheinen als F1 bis F12 (C3).
    #[test]
    fn keine_zeile_traegt_fn_und_die_funktionstasten_heissen_f1_bis_f12() {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let mut gross_f_gesehen = false;
        for stelle in 0..modell.zeilen() {
            let tasten = modell.tastentext(stelle).expect("jede Zeile hat Tasten");
            let funktion = modell.funktionstext(stelle).expect("und einen Text");
            for text in [&tasten, &funktion] {
                assert!(!text.contains("Fn+"), "{text} traegt Fn+");
                assert!(!text.contains("fn+"), "{text} traegt fn+");
            }
            // Kein kleines f vor einer Ziffer: f3 erscheint als F3.
            assert!(
                !tasten.split(['+', ',', ' ']).any(|teil| {
                    teil.starts_with('f')
                        && teil[1..].chars().all(|z| z.is_ascii_digit())
                        && !teil[1..].is_empty()
                }),
                "{tasten} schreibt eine Funktionstaste klein"
            );
            if tasten.contains('F') {
                gross_f_gesehen = true;
            }
        }
        assert!(gross_f_gesehen, "keine Zeile zeigt eine Funktionstaste");
    }

    /// Der F4-Eintrag erscheint als fuer den Editor reserviert (C3).
    #[test]
    fn der_f4_eintrag_ist_als_reserviert_gekennzeichnet() {
        let modell = Belegungsmodell::neu(Belegung::auslieferung());
        let belegung = Belegung::auslieferung();
        let stelle = belegung
            .funktionen()
            .iter()
            .position(|funktion| funktion.kennung() == "bearbeiten")
            .expect("die Auslieferungsbelegung kennt die Funktion bearbeiten");
        let text = modell
            .funktionstext(stelle)
            .expect("die Zeile hat einen Text");
        assert!(
            text.contains("reserviert für den Editor"),
            "{text} nennt den Vorbehalt nicht"
        );
        assert_eq!(modell.tastentext(stelle).as_deref(), Some(""));
    }

    /// Die Zuweisung durch Druecken: eine freie Kombination landet in der
    /// Zeile ihrer Funktion.
    #[test]
    fn eine_freie_kombination_wird_zugewiesen() {
        let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
        let stelle = zeile_von("kopieren");
        let druck = Tastendruck::neu(code_von_pflicht("f9"), ModMaske::LEER);
        assert_eq!(
            modell.zuweisen(stelle, druck),
            Zuweisung::Zugewiesen {
                funktion: "In das andere Fenster kopieren".to_owned(),
                kombination: "F9".to_owned(),
            }
        );
        assert!(modell.geaendert());
        let tasten = modell.tastentext(stelle).expect("die Zeile hat Tasten");
        assert!(tasten.contains("F9"), "{tasten} traegt F9 nicht");
    }

    /// Eine vergebene Kombination wird abgewiesen, und die Meldung nennt die
    /// andere Funktion — sie kommt woertlich aus dem Kern (C3).
    #[test]
    fn eine_vergebene_kombination_meldet_die_andere_funktion() {
        let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
        let stelle = zeile_von("kopieren");
        // f8 gehoert dem endgueltigen Loeschen.
        let druck = Tastendruck::neu(code_von_pflicht("f8"), ModMaske::LEER);
        let Zuweisung::Abgelehnt(meldung) = modell.zuweisen(stelle, druck) else {
            panic!("f8 ist vergeben und darf nicht zugewiesen werden");
        };
        assert!(
            meldung.contains("endgueltig_loeschen"),
            "{meldung} nennt die andere Funktion nicht"
        );
        assert!(!modell.geaendert());
    }

    /// Eine Taste ohne Namen in der Schreibweise ergibt keine Zeile, sondern
    /// die Auskunft [`Zuweisung::OhneNamen`] (C3, S11b).
    #[test]
    fn eine_taste_ohne_namen_wird_gemeldet_statt_geschrieben() {
        let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
        // Tastencode 10 traegt auf einer deutschen Tastatur die Taste links
        // neben der 1; die Schreibweise kennt keinen Namen dafuer.
        let druck = Tastendruck::neu(10, ModMaske::LEER);
        assert_eq!(modell.zuweisen(0, druck), Zuweisung::OhneNamen);
        assert!(!modell.geaendert());
    }

    /// Zwei Druecke, die sich allein durch gehaltenes fn unterscheiden, sind
    /// dieselbe Kombination: eine fn-Belegung ist nicht anlegbar (C3, S7).
    #[test]
    fn fn_unterscheidet_keine_kombination() {
        let mit_fn = Tastendruck::aus_ereignis(code_von_pflicht("f9"), roh::FUNKTION);
        let ohne = Tastendruck::aus_ereignis(code_von_pflicht("f9"), 0);
        assert_eq!(mit_fn, ohne);
        assert_eq!(
            Kombination::aus_tastendruck(mit_fn),
            Kombination::aus_tastendruck(ohne)
        );
    }

    /// Das Zuruecksetzen stellt die Auslieferungsbelegung wieder her (C3).
    #[test]
    fn das_zuruecksetzen_stellt_die_auslieferung_wieder_her() {
        let mut modell = Belegungsmodell::neu(Belegung::auslieferung());
        let stelle = zeile_von("kopieren");
        let druck = Tastendruck::neu(code_von_pflicht("f9"), ModMaske::LEER);
        assert!(matches!(
            modell.zuweisen(stelle, druck),
            Zuweisung::Zugewiesen { .. }
        ));
        modell.zuruecksetzen();
        assert!(modell.geaendert());
        assert_eq!(modell.in_belegung(), Belegung::auslieferung());
    }

    /// Die Kennung der Ansicht fuehrt seit diesem Schritt zu einem Kommando.
    #[test]
    fn belegung_ansehen_ist_ein_kommando() {
        assert_eq!(
            Kommando::aus_kennung("belegung_ansehen"),
            Some(Kommando::BelegungAnsehen)
        );
    }

    /// Die Zeile der genannten Funktion in der Auslieferungsbelegung.
    fn zeile_von(kennung: &str) -> usize {
        Belegung::auslieferung()
            .funktionen()
            .iter()
            .position(|funktion| funktion.kennung() == kennung)
            .unwrap_or_else(|| panic!("die Auslieferungsbelegung kennt {kennung} nicht"))
    }
}
