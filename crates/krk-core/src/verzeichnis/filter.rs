//! Die zwei Regeln des Filters: welche Zeichen er aufnimmt und wann ein Name
//! passt.
//!
//! ```text
//! Taste ohne Zusatztaste ──> traegt_ein_dateiname ──> Filtertext des Tabs
//!                                                            │
//!                        traegt_die_folge(Name, Filtertext) <┘
//!                              ^                    ^
//!                    modell::sichtbar        durchlauf
//! ```
//!
//! Die Datei traegt beide Regeln, weil beide dasselbe beantworten sollen und es
//! bei zwei Fassungen nicht mehr taeten. Die Zeichenregel hat zwei Aufrufer,
//! den Filter der Dateiliste und die Tippsuche der Belegungsansicht aus der
//! Runde 7; der Vergleich hat zwei, [`super::modell::Ordnermodell::sichtbar`]
//! fuer die angezeigte Zeile und [`super::durchlauf`] fuer den Unterbaum.
//!
//! # Was hier bis zur Runde 10 stand
//!
//! Bis zum 260815 hiess diese Datei `sprungmarke.rs` und trug die Sprungmarke
//! aus C2 der Runde 1: einen Puffer der getippten Anfangsbuchstaben, der nach
//! einer Sekunde von vorn begann, und die Suche nach der ersten Zeile mit
//! diesem Anfang. Die Runde 10 loest sie durch den Filtertext des Tabs ab, der
//! nicht ablaeuft; Puffer, Sekundenregel und Zeilensuche sind damit gefallen.
//! **Es gibt seither im Filter keine Zeitmessung mehr** — weder eine Uhr noch
//! eine Frist —, und keine Zeile dieses Weges liest [`std::time`].
//!
//! # Aufgenommen wird nur, was ein Dateiname tragen kann
//!
//! [`Nachschlag::Sprungmarke`](crate::tasten::Nachschlag::Sprungmarke)
//! antwortet auf **jede** Taste ohne Zusatztaste, die keiner Funktion gehoert,
//! nicht nur auf Buchstaben: der Kern kennt allein den Tastencode und weiss
//! nicht, welches Zeichen darauf liegt. Diese Datei traegt deshalb die eine
//! Regel, die daraus eine Sucheingabe macht, [`traegt_ein_dateiname`].
//!
//! Ohne sie schoebe die seit dem 260804 freie Eingabetaste ein
//! Wagenruecklaufzeichen in den Filtertext, und die Liste zeigte danach nichts
//! mehr. Die Regel ist trotzdem **keine Sonderregel fuer die Eingabetaste**:
//! sie deckt jede unbelegte Funktionstaste ab, deren Zeichen AppKit aus dem
//! privaten Bereich `U+F700` bis `U+F8FF` meldet, und jede andere Taste, die
//! ein Steuerzeichen liefert.
//!
//! Ein abgewiesenes Zeichen laesst den Filtertext unveraendert. Damit uebersteht
//! eine begonnene Suche einen Tastendruck, der keine Suche sein kann.

/// Der erste Tastencode des privaten Bereichs, in dem AppKit die Pfeile und
/// die Funktionstasten meldet (`NSUpArrowFunctionKey` und die uebrigen).
const FUNKTIONSTASTEN_ANFANG: char = '\u{F700}';

/// Das letzte Zeichen dieses Bereichs.
const FUNKTIONSTASTEN_ENDE: char = '\u{F8FF}';

/// Ob ein Dateiname dieses Zeichen tragen kann.
///
/// Zwei Klassen fallen weg. Steuerzeichen, wozu der Wagenruecklauf der
/// Eingabetaste, der Tabulator und die Escape-Taste gehoeren; ein Dateiname
/// traegt sie nicht, und sie im Filtertext zu fuehren hiesse, nach etwas zu
/// suchen, das kein Eintrag heissen kann. Und der Bereich `U+F700` bis
/// `U+F8FF`, in dem AppKit die Pfeile und die Funktionstasten meldet: diese
/// Zeichen sind ein Behelf der Oberflaeche und stehen fuer gar kein
/// Schriftzeichen.
pub fn traegt_ein_dateiname(zeichen: char) -> bool {
    !zeichen.is_control() && !(FUNKTIONSTASTEN_ANFANG..=FUNKTIONSTASTEN_ENDE).contains(&zeichen)
}

/// Ob dieser Name den Filtertext traegt: Teilzeichenfolge an jeder Stelle, ohne
/// Ruecksicht auf die Schreibung, ohne Faltung von Umlauten und Akzenten.
///
/// **Der eine Vergleich.** Seine beiden Rufer sind
/// [`super::modell::Ordnermodell::sichtbar`], das ueber die angezeigte Zeile
/// entscheidet, und [`super::durchlauf`], das denselben Vergleich auf jeden
/// Namen im Unterbaum zieht. Bis zum 260815 stand er in beiden Dateien
/// getrennt; dass eine tiefe Suche etwas anderes faende als eine flache, waere
/// keine Eigenschaft, die jemand haette erklaeren koennen.
///
/// `filter_klein` ist **bereits kleingeschrieben** und wird hier nicht noch
/// einmal umgeschrieben. Das ist der Grund fuer die Asymmetrie der beiden
/// Argumente: der Filtertext wird einmal je Suche umgeschrieben, der Name
/// einmal je Vergleich. Wer einen ungeschriebenen Text hereingibt, findet
/// nichts mit Grossbuchstaben.
///
/// Ein leerer `filter_klein` traegt jeder Name. Wer nicht filtern will, fragt
/// diese Funktion nicht; ihre beiden Rufer haben den Zweig „steht ein
/// Filtertext?" davor.
pub fn traegt_die_folge(name: &str, filter_klein: &str) -> bool {
    name.to_lowercase().contains(filter_klein)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ein_wagenruecklauf_und_eine_funktionstaste_tragen_kein_dateiname() {
        assert!(!traegt_ein_dateiname('\r'));
        assert!(!traegt_ein_dateiname('\n'));
        assert!(!traegt_ein_dateiname('\t'));
        assert!(!traegt_ein_dateiname('\u{1B}'), "die Escape-Taste");
        assert!(!traegt_ein_dateiname('\u{F701}'), "NSDownArrowFunctionKey");
        assert!(!traegt_ein_dateiname('\u{F704}'), "NSF1FunctionKey");
    }

    #[test]
    fn buchstaben_ziffern_und_satzzeichen_tragen_ein_dateiname() {
        for zeichen in ['a', 'Z', '7', '.', '-', ' ', 'ä', '中'] {
            assert!(
                traegt_ein_dateiname(zeichen),
                "{zeichen:?} gilt als nicht tragbar"
            );
        }
    }

    /// C1.2: die Folge zaehlt an jeder Stelle des Namens und nicht nur am
    /// Anfang.
    #[test]
    fn die_folge_zaehlt_an_jeder_stelle_des_namens() {
        assert!(traegt_die_folge("bbbaaaccc.rs", "aaa"), "mittendrin");
        assert!(traegt_die_folge("aaaccc.rs", "aaa"), "am Anfang");
        assert!(traegt_die_folge("cccaaa", "aaa"), "am Ende");
        assert!(!traegt_die_folge("bbbccc.rs", "aaa"));
    }

    /// C1.2: die Schreibung des Namens spielt keine Rolle.
    #[test]
    fn die_schreibung_des_namens_spielt_keine_rolle() {
        assert!(traegt_die_folge("LIESMICH.TXT", "liesmich"));
        assert!(traegt_die_folge("Banane.txt", "nan"));
    }

    /// C1.3: gefaltet wird nichts. `apfel` findet `Äpfel` nicht.
    #[test]
    fn der_vergleich_faltet_keine_umlaute_und_keine_akzente() {
        assert!(!traegt_die_folge("Äpfel.txt", "apfel"));
        assert!(!traegt_die_folge("Cafe.txt", "café"));
        assert!(
            traegt_die_folge("Äpfel.txt", "äpfel"),
            "kleingeschrieben findet der Umlaut sich selbst"
        );
    }

    /// Die Asymmetrie der beiden Argumente, ausgeschrieben: der Filtertext
    /// kommt kleingeschrieben herein, der Name nicht.
    #[test]
    fn ein_grossgeschriebener_filtertext_findet_nichts() {
        assert!(
            !traegt_die_folge("Banane.txt", "Banane"),
            "der Rufer schreibt den Filtertext einmal je Suche klein"
        );
    }

    #[test]
    fn ein_leerer_filtertext_steht_in_jedem_namen() {
        assert!(traegt_die_folge("beliebig.txt", ""));
    }
}
