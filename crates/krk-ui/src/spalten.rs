//! Die vier Spalten des Dateifensters als reine Aufzaehlung: welche es gibt,
//! in welcher Reihenfolge sie stehen, wie sie heissen und in welcher der Nutzer
//! schreiben darf.
//!
//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile, und
//! das ist ihr Zweck: [`Spalte`] hat seit der Bereichsleisten-Runde zwei
//! Leser, die Tabelle und die Bereichsleiste, und der zweite braucht die
//! Aufzaehlung, nicht die Tabelle. Was AppKit-Typen nennt, bleibt drueben in
//! [`crate::appkit::tabelle`] und steht dort als freie Funktion ueber dieser
//! Aufzaehlung: die Kennung, die Ueberschrift, die beiden Breiten, die
//! Ausrichtung und der Weg von einer Kennung zurueck zur Spalte. Dasselbe
//! Muster tragen `aufteilung::sichtbar_im` und `aufteilung::rahmenfarbe`, die
//! ueber [`crate::fenstermodell::Bereich`] rechnen, ohne dass die Aufzaehlung
//! unter `appkit/` liegen muesste.
//!
//! # Vollstaendige Fallunterscheidung ohne Auffangzweig
//!
//! [`Spalte`] traegt vier Werte und behaelt sie. Jede Fallunterscheidung
//! darueber ist ausgeschrieben, keine hat einen Auffangzweig, und das ist
//! Absicht: eine fuenfte Spalte haelt den Bau an und erzwingt fuer jede der
//! Stellen eine bewusste Antwort — Kennung, Ueberschrift, Beschriftung,
//! Breiten, Ausrichtung, Zellentext und die Frage, ob man in ihr schreiben
//! darf. Ein `_ =>` irgendwo darunter machte aus der neuen Spalte still eine
//! linksbuendige, unbeschreibbare Namenlose.
//!
//! Die Reihenfolge in [`Spalte::ALLE`] ist die Reihenfolge im Fenster. Zwei
//! Stellen leiten daraus etwas ab, statt es hinzuschreiben: die Stelle der
//! Namensspalte fuer `editColumn:row:withEvent:select:` und, ab der
//! Bereichsleiste, die Nummer, unter der ein Schalter seine Spalte nennt.

/// Eine der vier Spalten des Dateifensters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spalte {
    /// Der Name des Eintrags.
    Name,
    /// Die Groesse der Daten.
    Groesse,
    /// Der Zeitpunkt der letzten Aenderung.
    Geaendert,
    /// Die Dateiendung.
    ///
    /// "Typ" heisst in KRK die Dateiendung: die Spalte zeigt sie, die
    /// Sortierung nach Typ ordnet nach ihr
    /// ([`krk_core::verzeichnis::Schluessel::Typ`]), und die Tastenfunktion
    /// "Nach Typ sortieren" loest dieselbe Ordnung aus. Die Eintragsart selbst
    /// (Ordner, Datei, Verknuepfung) steht in der Metadatenanzeige der
    /// Vorschau, nicht in der Tabelle.
    ///
    /// Zwei Entscheide tragen das, und sie tragen verschiedene Haelften.
    /// Ueber den **Schluessel der Sortierung** entscheidet
    /// `decisions/260802-1810_*_sortierung-ohne-sprachsensitive-kollation.md`
    /// (Nutzerentscheid vom 260806): nach Typ zu ordnen heisst, nach der
    /// Endung zu ordnen. Ueber den **Inhalt dieser Zelle** sagt er nichts;
    /// den entscheidet der Nutzer am 260806-2300 in
    /// `issues/260806-1723_*_die-spalte-typ-zeigt-die-eintragsart-sortiert-aber-nach-der-endung.md`,
    /// Abschnitt "ein fuenfter Weg": die Ueberschrift bleibt "Typ", die Zelle
    /// zeigt die Endung.
    Typ,
}

impl Spalte {
    /// Alle vier Spalten in der Reihenfolge, in der sie im Fenster stehen.
    pub const ALLE: [Spalte; 4] = [
        Spalte::Name,
        Spalte::Groesse,
        Spalte::Geaendert,
        Spalte::Typ,
    ];

    /// Der kurze Name der Spalte, wie ihn ein Schalter der Bereichsleiste
    /// traegt.
    ///
    /// **Kurz, weil die Leiste 18 Punkte hoch ist** und neun Schalter
    /// nebeneinander traegt. Drei der vier Namen sind zugleich die
    /// Ueberschrift der Spalte in der Tabelle, und
    /// `appkit::tabelle::titel` leitet sie von hier ab, statt sie ein zweites
    /// Mal hinzuschreiben.
    ///
    /// [`Spalte::Geaendert`] ist die Ausnahme: der Schalter heisst "Datum",
    /// die Spaltenueberschrift bleibt "Änderungsdatum". Das ist gewollt und
    /// kein Versehen — "Datum" ist der Name, den der Nutzer dem Schalter
    /// gegeben hat, und ueber der Spalte stuende er zu knapp, weil dort auch
    /// die Uhrzeit steht.
    pub fn beschriftung(self) -> &'static str {
        match self {
            Spalte::Name => "Name",
            Spalte::Groesse => "Größe",
            Spalte::Geaendert => "Datum",
            Spalte::Typ => "Typ",
        }
    }

    /// Ob der Nutzer in dieser Spalte schreiben darf (C4).
    ///
    /// Allein der Name: die drei uebrigen Spalten zeigen, was das Dateisystem
    /// ueber den Eintrag sagt, und keine davon laesst sich durch Hinschreiben
    /// aendern.
    ///
    /// **Ausgeschrieben und nicht `matches!`**, wie
    /// `appkit::tabelle::ausrichtung` es haelt: ein `matches!` ist ein `match`
    /// mit einem `_ => false` darunter und gaebe einer fuenften Spalte still
    /// "nicht beschreibbar", statt den Bau anzuhalten. Genau das schliesst der
    /// Modulkopf aus, und die Probe daneben faengt es nicht ab — eine
    /// fuenfte, still unbeschreibbare Spalte laesst ihre Gleichheit
    /// unberuehrt.
    pub const fn beschreibbar(self) -> bool {
        match self {
            Spalte::Name => true,
            Spalte::Groesse | Spalte::Geaendert | Spalte::Typ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jede_spalte_hat_eine_eigene_beschriftung() {
        for (stelle, spalte) in Spalte::ALLE.into_iter().enumerate() {
            assert!(!spalte.beschriftung().is_empty());
            for andere in Spalte::ALLE.into_iter().skip(stelle + 1) {
                assert_ne!(spalte.beschriftung(), andere.beschriftung());
            }
        }
    }

    /// Welche der vier Spalten beschreibbar ist — und nur das.
    ///
    /// **Dass eine fuenfte Spalte hier eine Antwort erzwingt, haelt der
    /// Uebersetzer und nicht diese Probe.** [`Spalte::beschreibbar`] ist ein
    /// ausgeschriebenes `match`; eine neue Variante haelt den Bau an. Diese
    /// Probe liefe auch mit einem `_ => false` gruen, und genau daran hing der
    /// Befund vom 260812-0727.
    #[test]
    fn genau_die_namensspalte_ist_beschreibbar() {
        let beschreibbare: Vec<Spalte> = Spalte::ALLE
            .into_iter()
            .filter(|spalte| spalte.beschreibbar())
            .collect();
        assert_eq!(beschreibbare, vec![Spalte::Name]);
    }
}
