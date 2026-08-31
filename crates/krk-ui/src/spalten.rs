//! Die fuenf Spalten des Dateifensters als reine Aufzaehlung: welche es gibt,
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
//! [`Spalte`] traegt fuenf Werte. Jede Fallunterscheidung darueber ist
//! ausgeschrieben, keine hat einen Auffangzweig, und das ist Absicht: eine
//! sechste Spalte haelt den Bau an und erzwingt fuer jede der Stellen eine
//! bewusste Antwort — Kennung, Ueberschrift, Beschriftung, Breiten,
//! Ausrichtung, Zellentext und die Frage, ob man in ihr schreiben darf. Ein
//! `_ =>` irgendwo darunter machte aus der neuen Spalte still eine
//! linksbuendige, unbeschreibbare Namenlose. Die Git-Runde ist der Beleg
//! dafuer, dass die Vorkehrung traegt: der Uebersetzer hat diese sieben
//! Stellen einzeln genannt, dazu `spalte_sichtbar_in` und `spalte_umschalten`
//! (`crate::fenstermodell`), `kommando_der_spalte`
//! (`crate::appkit::bereichsleiste`) und ein Strukturliteral in
//! `crates/krk-core/tests/ablage.rs`, und jede hat ihre Antwort von Hand
//! bekommen. Wer die Zahl der gehaltenen Stellen heute braucht, zaehlt sie mit
//! `grep -rn 'Spalte::Marke =>' crates/krk-ui/src`, dem letzten Wert der
//! Aufzaehlung, statt sie hier abzulesen: sie waechst mit jeder
//! Fallunterscheidung, die jemand hinzufuegt.
//!
//! # Die eine Stelle, die der Uebersetzer nicht haelt: [`Spalte::ALLE`]
//!
//! **Und sie ist die, die entscheidet, ob die neue Spalte ueberhaupt
//! erscheint.** Wer eine sechste Variante anlegt und die Fallunterscheidungen
//! darueber alle beantwortet hat, hat eine uebersetzbare Datei; [`Spalte::ALLE`]
//! steht dabei weiter auf `[Spalte; 5]`, und die neue Spalte hat danach weder
//! eine Zelle in der Tabelle noch ein Ankreuzfeld in der Bereichsleiste, weil
//! beide ueber diese Liste reihen. Der Bau meldet nichts.
//!
//! **Keine Probe faengt diesen Fall**, und die naheliegende faengt den
//! umgekehrten: `tabs::tests::die_dateiliste_bleibt_flach_und_hat_fuenf_spalten`
//! haelt `Spalte::ALLE.len()` gegen eine hingeschriebene Zahl und wird rot,
//! wenn jemand die Liste erweitert, ohne die Probe nachzuziehen — nicht, wenn
//! jemand die Aufzaehlung erweitert, ohne die Liste nachzuziehen. Eine Probe,
//! die die Varianten aus dem **Quelltext** der Aufzaehlung liest, wie
//! `Kommando::KENNUNGEN` und `Marke::ALLE` sie haben
//! (`crates/krk-core/tests/belegung.rs`, `crates/krk-core/tests/git.rs`), kann
//! [`Spalte::ALLE`] nicht bekommen: die stehen unter `crates/krk-core/tests/`,
//! und `krk-ui` hat kein Bibliotheksziel. Welche Bauform die Vollstaendigkeit
//! einer `ALLE`-Liste kuenftig haelt, ist eine offene Nutzerfrage
//! (`shared/decisions/260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`);
//! bis dahin ist die Vorkehrung das Lesen dieses Absatzes.
//!
//! Die Reihenfolge in [`Spalte::ALLE`] ist die Reihenfolge im Fenster. Zwei
//! Stellen leiten daraus etwas ab, statt es hinzuschreiben: die Stelle der
//! Namensspalte fuer `editColumn:row:withEvent:select:` und, ab der
//! Bereichsleiste, die Nummer, unter der ein Schalter seine Spalte nennt.

/// Eine der fuenf Spalten des Dateifensters.
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
    /// Die Git-Marke des Eintrags: ein Buchstabe oder nichts.
    ///
    /// Fuenf Zustaende, fuenf Buchstaben (E11 der Git-Runde): `M` geaendert,
    /// `S` vorgemerkt, `N` neu, `K` in Konflikt, `U` umbenannt. **Ein
    /// unveraenderter Eintrag traegt kein sechstes Zeichen**, sondern eine
    /// leere Zelle (A11): eine Marke fuer den Normalfall fuellte die Spalte in
    /// jedem Repository mit einem Zeichen, das nichts sagt.
    ///
    /// **Die leere Zelle ist deshalb kein Zwischenstand des Baus, sondern der
    /// eine von zwei Zielzustaenden.** Ein Ordner, der in keinem Repository
    /// liegt, laesst sie dauerhaft leer, und die Spalte wird trotzdem nicht
    /// eingezogen (E5, C6.3); den zweiten Fall — der Ordner liegt in einem
    /// Repository und der Statuslauf hat einen Buchstaben geliefert — schreibt
    /// `appkit::tabelle::Tabellenquelle::beschriften` seit Schritt 6 der
    /// Git-Runde aus dem Ordnermodell.
    ///
    /// **Ihr Ankreuzfeld traegt seit Schritt 8 der Git-Runde
    /// [`Kommando::SpalteMarkeUmschalten`]**(krk_core::tasten::Kommando) und
    /// steht als viertes in der Reihe der Spaltenschalter; ab Werk steht die
    /// Spalte (A13).
    ///
    /// **Nach ihr wird nicht sortiert** (A12): [`krk_core::verzeichnis::Schluessel`]
    /// bleibt bei vier Werten. Die Sortierung dieses Projekts laeuft ueber
    /// Schluessel, die beim Lesen entstehen; ein Schluessel, der auf einen
    /// nachgetragenen Befund wartete, ordnete die Liste beim Eintreffen des
    /// Befunds neu.
    Marke,
}

impl Spalte {
    /// Alle fuenf Spalten in der Reihenfolge, in der sie im Fenster stehen.
    pub const ALLE: [Spalte; 5] = [
        Spalte::Name,
        Spalte::Groesse,
        Spalte::Geaendert,
        Spalte::Typ,
        Spalte::Marke,
    ];

    /// Der kurze Name der Spalte, wie ihn ein Schalter der Bereichsleiste
    /// traegt.
    ///
    /// **Kurz, weil die Leiste 18 Punkte hoch ist** und ihre Schalter
    /// nebeneinander traegt. Alle Namen ausser dem von [`Spalte::Geaendert`]
    /// sind zugleich die Ueberschrift der Spalte in der Tabelle, und
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
            Spalte::Marke => "Marke",
        }
    }

    /// Ob der Nutzer in dieser Spalte schreiben darf (C4).
    ///
    /// Allein der Name: die uebrigen Spalten zeigen, was das Dateisystem
    /// beziehungsweise das Repository ueber den Eintrag sagt, und keine davon
    /// laesst sich durch Hinschreiben aendern. Fuer [`Spalte::Marke`] ist das
    /// kein Nebenbefund, sondern die Grenze der Stufe A der Git-Runde: sie
    /// liest und schreibt nicht (E8).
    ///
    /// **Ausgeschrieben und nicht `matches!`**, wie
    /// `appkit::tabelle::ausrichtung` es haelt: ein `matches!` ist ein `match`
    /// mit einem `_ => false` darunter und gaebe einer neuen Spalte still
    /// "nicht beschreibbar", statt den Bau anzuhalten. Genau das schliesst der
    /// Modulkopf aus, und die Probe daneben faengt es nicht ab — eine neue,
    /// still unbeschreibbare Spalte laesst ihre Gleichheit unberuehrt.
    pub const fn beschreibbar(self) -> bool {
        match self {
            Spalte::Name => true,
            Spalte::Groesse | Spalte::Geaendert | Spalte::Typ | Spalte::Marke => false,
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

    /// Welche Spalte beschreibbar ist — und nur das.
    ///
    /// **Dass eine neue Spalte hier eine Antwort erzwingt, haelt der
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
