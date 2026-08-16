//! Die drei Regeln des Filters: welche Zeichen er aufnimmt, wann ein Name
//! passt, und ab welcher Laenge der Filtertext auch Inhalte meint.
//!
//! ```text
//! Taste ohne Zusatztaste ──> traegt_ein_dateiname ──> Filtertext des Tabs
//!                                                            │
//!                        traegt_die_folge(Name, Filtertext) <┤
//!                              ^                    ^        │
//!                    modell::sichtbar        durchlauf       │
//!                                                            │
//!                        inhaltsschwelle(tief) <─ Zeichenzahl┘
//!                              ^
//!                    modell::inhalt_wirkt
//! ```
//!
//! Die Datei traegt alle drei Regeln, weil jede an mehreren Stellen dieselbe
//! Antwort geben soll und es bei zwei Fassungen nicht mehr taete. Die
//! Zeichenregel hat zwei Aufrufer, den Filter der Dateiliste und die Tippsuche
//! der Belegungsansicht aus der Runde 7; der Vergleich hat zwei,
//! [`super::modell::Ordnermodell::sichtbar`] fuer die angezeigte Zeile und
//! [`super::durchlauf`] fuer den Unterbaum. Die Schwelle hat einen,
//! [`super::modell::Ordnermodell::inhalt_wirkt`], und der ist seinerseits die
//! eine Stelle, die alle Frager nach dem Inhaltsfilter bedient.
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
//! [`Nachschlag::Tippen`](crate::tasten::Nachschlag::Tippen) antwortet auf
//! **jede** Taste, die keiner Funktion gehoert und keine Befehlstaste haelt,
//! nicht nur auf Buchstaben: der Kern kennt allein den Tastencode und weiss
//! nicht, welches Zeichen darauf liegt. Diese Datei traegt deshalb die eine
//! Regel, die daraus eine Sucheingabe macht, [`traegt_ein_dateiname`].
//!
//! **Die beiden Fragen bleiben getrennt.** Welcher **Tastendruck** ueberhaupt
//! ankommt, entscheidet der Nachschlag; welches **Zeichen** aufgenommen wird,
//! entscheidet [`traegt_ein_dateiname`]. Seit dem 260816 laesst der Nachschlag
//! auch `shift` und `opt` durch, und damit kommt hier mehr an als zuvor — ein
//! Grossbuchstabe, `_`, `@`, `|`, `~`, `\`. Die Regel hier hat sich dafuer
//! nicht geaendert: sie liess diese Zeichen immer schon durch.
//!
//! Ohne sie schoebe die seit dem 260804 freie Eingabetaste ein
//! Wagenruecklaufzeichen in den Filtertext, und die Liste zeigte danach nichts
//! mehr. Die Regel ist trotzdem **keine Sonderregel fuer die Eingabetaste**:
//! sie deckt jede unbelegte Funktionstaste ab, deren Zeichen AppKit aus dem
//! privaten Bereich `U+F700` bis `U+F8FF` meldet, jede andere Taste, die ein
//! Steuerzeichen liefert, und den Schraegstrich.
//!
//! **Der Schraegstrich faellt weg, seit die Namensspalte ihn zeigt.** Seit dem
//! 260815 haengt die Dateiliste an jeden Ordner ein Ordnerzeichen an, und das
//! ist der Schraegstrich (`krk-ui/src/appkit/tabelle.rs`, `ORDNERZEICHEN`). Er
//! ist dort Anzeige und nie Name: der Vergleich unten liest weiter
//! `eintrag.name`. Wer `Bilder/` liest und `bilder/` tippt, bekaeme also eine
//! leere Liste, wenn die Zeichenregel den Schraegstrich aufnaehme. Sie nimmt
//! ihn nicht auf, und der Grund ist derselbe wie beim Wagenruecklauf: kein
//! Name kann ihn tragen.
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
/// Drei Klassen fallen weg. Steuerzeichen, wozu der Wagenruecklauf der
/// Eingabetaste, der Tabulator und die Escape-Taste gehoeren; ein Dateiname
/// traegt sie nicht, und sie im Filtertext zu fuehren hiesse, nach etwas zu
/// suchen, das kein Eintrag heissen kann. Der Bereich `U+F700` bis `U+F8FF`,
/// in dem AppKit die Pfeile und die Funktionstasten meldet: diese Zeichen sind
/// ein Behelf der Oberflaeche und stehen fuer gar kein Schriftzeichen. Und der
/// Schraegstrich, der die Bestandteile eines Pfades trennt und deshalb in
/// keinem Namen vorkommen kann, den ein Dateisystem hergibt;
/// [`crate::operation::name_pruefen`] weist ihn aus demselben
/// Grund ab.
pub fn traegt_ein_dateiname(zeichen: char) -> bool {
    !zeichen.is_control()
        && !(FUNKTIONSTASTEN_ANFANG..=FUNKTIONSTASTEN_ENDE).contains(&zeichen)
        && zeichen != '/'
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

/// Ab wie vielen getippten **Zeichen** der Filter auch den Inhalt einer Datei
/// liest: fuenf bei eingeschalteter tiefer Suche, sonst drei.
///
/// **Die Staffelung ist hergeleitet und nicht gesetzt.** Ein flacher
/// Inhaltsfilter liest die Dateien des angezeigten Ordners, ein tiefer die
/// Dateien seines ganzen Unterbaums, und das sind je nach Ort um
/// Groessenordnungen mehr. Zwei Zeichen bezeichnen wenig und treffen
/// entsprechend viel; die Zahl der zu lesenden Dateien waechst also genau
/// dort, wo die Eingabe am wenigsten aussagt. Die hoehere Schwelle der tiefen
/// Suche gleicht das aus.
///
/// **Gezaehlt werden Zeichen und keine Bytes.** Ein getipptes `äöü` sind drei
/// Zeichen und sechs Bytes; die Staffelung spricht von Zeichen, und der eine
/// Rufer zaehlt deshalb mit `chars().count()`.
///
/// **Ein Rufer, und der ist selbst die eine Stelle:**
/// [`super::modell::Ordnermodell::inhalt_wirkt`]. Wer wissen will, ob der
/// Inhaltsfilter wirkt, fragt dort und rechnet die Schwelle nicht nach. Ein
/// zweiter Rechenweg waere die Gelegenheit, an zwei Stellen verschieden zu
/// antworten, und genau die schliesst diese Runde aus.
///
/// **Die Schwelle wird bei jeder Bewertung neu gefragt und nicht beim Start
/// gemerkt.** Daraus folgt ein Fall, der benannt gehoert: wer bei vier Zeichen
/// ohne tiefe Suche Inhaltstreffer vor sich hat und die tiefe Suche
/// einschaltet, verliert sie, weil die Schwelle auf fuenf steigt. Ein fuenftes
/// Zeichen holt sie zurueck. Eine Ausnahme fuer den Umschaltmoment waere ein
/// Sonderfall ohne Gegenstueck.
///
/// `#[must_use]`, weil der Aufruf nichts tut ausser zu antworten: wer den Wert
/// fallen laesst, hat ihn umsonst geholt, und still.
#[must_use]
pub fn inhaltsschwelle(tief: bool) -> usize {
    if tief { 5 } else { 3 }
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

    /// Der Schraegstrich trennt die Bestandteile eines Pfades; kein Name
    /// traegt ihn, und die Namensspalte zeigt ihn seit dem 260815 als
    /// Ordnerzeichen. Naehme der Filter ihn auf, liefe ein getipptes `bilder/`
    /// gegen den Namen `Bilder` und faende nichts.
    #[test]
    fn ein_schraegstrich_traegt_kein_dateiname() {
        assert!(!traegt_ein_dateiname('/'));
    }

    /// Was die Zeichenregel aufnimmt, muss die Namenspruefung des Umbenennens
    /// durchlassen. **Die Umkehrung gilt nicht** und soll nicht gelten: ein
    /// Name mit Zeilenumbruch ist unter macOS zulaessig, taugt aber als
    /// Filtereingabe nicht. Genau in der einen Richtung stand der Widerspruch,
    /// den `shared/issues/260815-2208_*_der-filter-nimmt-den-schraegstrich-auf`
    /// gemeldet hat.
    #[test]
    fn was_die_zeichenregel_aufnimmt_traegt_auch_ein_name() {
        for zeichen in ['/', '\0', 'a', 'Z', '7', '.', '-', ' ', 'ä', '中'] {
            if !traegt_ein_dateiname(zeichen) {
                continue;
            }
            let name = format!("datei{zeichen}name");
            assert!(
                crate::operation::name_pruefen(&name).is_ok(),
                "{zeichen:?} wird aufgenommen, aber {name:?} ist kein Name"
            );
        }
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

    /// Die Staffelung, ausgeschrieben: drei Zeichen flach, fuenf tief.
    #[test]
    fn die_inhaltsschwelle_steht_bei_drei_und_bei_fuenf() {
        assert_eq!(inhaltsschwelle(false), 3, "ohne tiefe Suche");
        assert_eq!(inhaltsschwelle(true), 5, "mit tiefer Suche");
    }
}
