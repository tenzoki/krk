//! Die drei Regeln des Filters: welche Zeichen er aufnimmt, wann ein Name
//! passt, und ab welcher Laenge der Filtertext auch Inhalte meint.
//!
//! ```text
//! Taste ohne Zusatztaste ──> traegt_ein_dateiname ──> Filtertext des Tabs
//! Text aus der Zwischenablage ─┘ (zwischenablage::filtertext_aus)   │
//!                                                                    │
//!                                        Muster::aus(Filtertext) <───┤
//!                                              │                     │
//!                        traegt_die_folge(Text, &Muster)             │
//!                            ^             ^         ^               │
//!                 modell::zeilengrund_von durchlauf inhalt           │
//!                                                                    │
//!                        inhaltsschwelle(tief) <─ Zeichenzahl ohne `*`┘
//!                              ^
//!                    modell::inhalt_wirkt
//! ```
//!
//! Die Datei traegt alle drei Regeln, weil jede an mehreren Stellen dieselbe
//! Antwort geben soll und es bei zwei Fassungen nicht mehr taete. Die
//! Zeichenregel hat drei Aufrufer, den Filter der Dateiliste, die Tippsuche
//! der Belegungsansicht aus der Runde 7 und seit der Runde 21 die Reinigung
//! des eingefuegten Textes, [`crate::zwischenablage::filtertext_aus`]; der
//! Vergleich hat drei, den Pruefschritt des Ordnermodells fuer die angezeigte
//! Zeile, [`super::durchlauf`] fuer den Unterbaum und seit der Runde 11
//! [`super::inhalt`] fuer den gelesenen Text einer Datei. Die Schwelle hat
//! einen, [`super::modell::Ordnermodell::inhalt_wirkt`], und der ist
//! seinerseits die eine Stelle, die alle Frager nach dem Inhaltsfilter
//! bedient.
//!
//! # Der Vergleich ist seit der Runde 21 ein Musterabgleich
//!
//! Der Filtertext kennt **genau ein Sonderzeichen**, das `*`, und es steht fuer
//! eine beliebige, auch leere Zeichenfolge; mehrere `*` sind erlaubt, zwei
//! nebeneinander bedeuten dasselbe wie eines. Kein `?`, keine Zeichenklassen,
//! kein Entkommen: ein `*` ist immer der Platzhalter, und ein Name mit
//! woertlichem `*` wird ueber seine anderen Zeichen gefunden. **Der Vergleich
//! bleibt an beiden Enden ungebunden**: eine Teilfolge bleibt eine Teilfolge,
//! `abc` trifft an jeder Stelle des Namens wie vor der Runde, und ein `*` am
//! Anfang oder am Ende des Filtertexts verankert nichts. Der Filtertext wird
//! **einmal je Aenderung** in [`Muster`] zerlegt und kleingeschrieben, der
//! Vergleich laeuft je Eintrag ohne Rueckverfolgung: jedes Stueck wird ab dem
//! Ende des vorigen genau einmal gesucht. Fuer die Schwelle zaehlt ein `*`
//! nicht mit; siehe [`inhaltsschwelle`]. Die Tippsuche der Belegungsansicht
//! teilt mit dem Filter allein die Zeichenregel und kennt den Platzhalter
//! nicht.
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

/// Der zerlegte, kleingeschriebene Filtertext, wie der Vergleich ihn braucht.
///
/// **Entsteht einmal je Aenderung des Filtertexts und nicht einmal je
/// Vergleich**: der Text wird kleingeschrieben und an jedem `*` geteilt, und
/// diese Stuecke reisen als Wert in den Pruefschritt, in den Durchlauf und an
/// den Inhaltsbefund. Bei 100.000 Eintraegen waere alles andere 100.000
/// Zerlegungen desselben kurzen Texts. Die Asymmetrie der beiden Argumente
/// von [`traegt_die_folge`] haelt damit der Typ und nicht die Disziplin des
/// Rufers: wer ein `Muster` in der Hand hat, hat einen kleingeschriebenen
/// Text.
///
/// Die Stueckliste ist **nie leer**: ein leerer Filtertext ergibt ein leeres
/// Stueck, und das steht in jedem Namen. Ein `*` am Anfang, am Ende oder
/// neben einem zweiten ergibt ebenfalls ein leeres Stueck; `find("")` trifft
/// sofort und verschiebt nichts, und genau darum verankert der Platzhalter
/// nichts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Muster {
    /// Die an `*` getrennten, kleingeschriebenen Stuecke des Filtertexts.
    stuecke: Vec<String>,
}

impl Muster {
    /// Zerlegt den Filtertext einmal: kleinschreiben, an `*` teilen.
    #[must_use]
    pub fn aus(filtertext: &str) -> Self {
        Self {
            stuecke: filtertext
                .to_lowercase()
                .split('*')
                .map(str::to_owned)
                .collect(),
        }
    }
}

/// Ob dieser Name das Muster traegt: jedes Stueck des Musters in Reihenfolge
/// und ohne Ueberlappung, das erste an beliebiger Stelle, jedes weitere hinter
/// dem Ende des vorigen; ohne Ruecksicht auf die Schreibung, ohne Faltung von
/// Umlauten und Akzenten. Fuer einen Filtertext ohne `*` ist das genau eine
/// Suche, also `contains`, wie vor der Runde 21.
///
/// **Der eine Vergleich, und seine Rufer stehen alle im Kern.** Der
/// Pruefschritt des Ordnermodells entscheidet ueber die angezeigte Zeile,
/// [`super::durchlauf`] zieht denselben Vergleich auf jeden Namen im Unterbaum,
/// und seit der Runde 11 legt ihn [`super::inhalt`] an den gelesenen Text einer
/// Datei. Bis zum 260815 stand er in den ersten beiden
/// Dateien getrennt; dass eine tiefe Suche etwas anderes faende als eine
/// flache, waere keine Eigenschaft, die jemand haette erklaeren koennen.
///
/// **Das Argument heisst `name`, weil die ersten beiden Rufer Namen
/// vergleichen.** Der dritte gibt den gelesenen Text einer Datei herein, und
/// die Regel ist dieselbe — genau darum steht sie hier einmal und nicht je
/// Gegenstand einmal. Beim Inhalt darf ein `*` deshalb ueber Zeilenenden
/// hinweg treffen; eine Regel „nur innerhalb einer Zeile" waere ein zweiter
/// Vergleich.
///
/// **Ohne Rueckverfolgung, und trotzdem vollstaendig.** Gibt es fuer
/// `s1*s2*…*sn` eine Zerlegung mit Stellen `p1 < p2 < …`, dann liegt die
/// erste Fundstelle `q1` von `s1` bei `q1 <= p1`, und `s2` steht ab
/// `p1 + |s1| >= q1 + |s1|` weiterhin im Rest; Induktion ueber die Stuecke.
/// Die gierige erste Fundstelle verliert also nie eine Zerlegung, die eine
/// spaetere gefunden haette.
///
/// Das Muster ist **bereits kleingeschrieben und zerlegt** ([`Muster::aus`]);
/// der Name wird hier einmal je Vergleich umgeschrieben. Das ist die
/// Asymmetrie der beiden Argumente: der Filtertext einmal je Aenderung, der
/// Name einmal je Vergleich.
///
/// Ein leeres Muster traegt jeder Name. Wer nicht filtern will, fragt
/// diese Funktion nicht: der Pruefschritt und der Durchlauf haben den Zweig
/// „steht ein Filtertext?" davor, und der Inhaltsbefund kommt gar nicht erst
/// zustande, weil [`inhaltsschwelle`] ohne Filtertext nicht erreicht ist.
pub fn traegt_die_folge(name: &str, muster: &Muster) -> bool {
    let name = name.to_lowercase();
    let mut ab = 0;
    for stueck in &muster.stuecke {
        match name[ab..].find(stueck.as_str()) {
            Some(stelle) => ab += stelle + stueck.len(),
            None => return false,
        }
    }
    true
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
/// **Gezaehlt werden Zeichen und keine Bytes, und das `*` zaehlt nicht.** Ein
/// getipptes `äöü` sind drei Zeichen und sechs Bytes; die Staffelung spricht
/// von Zeichen, und der eine Rufer zaehlt deshalb mit `chars()`. Seit der
/// Runde 21 laesst er dabei jedes `*` aus: der Platzhalter sagt nichts ueber
/// den Gegenstand aus, `ab*` bezeichnet weniger als `abc` und nicht mehr, und
/// die Schwelle schuetzt genau davor, bei einer wenig sagenden Eingabe viele
/// Dateien zu lesen. `ab*cd` sind vier Zeichen, `*****` sind null, und ein
/// Filtertext aus lauter `*` liest nie eine Datei.
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

    /// Kurzform fuer die Proben des Vergleichs.
    fn trifft(name: &str, filtertext: &str) -> bool {
        traegt_die_folge(name, &Muster::aus(filtertext))
    }

    /// C1.2: die Folge zaehlt an jeder Stelle des Namens und nicht nur am
    /// Anfang.
    #[test]
    fn die_folge_zaehlt_an_jeder_stelle_des_namens() {
        assert!(trifft("bbbaaaccc.rs", "aaa"), "mittendrin");
        assert!(trifft("aaaccc.rs", "aaa"), "am Anfang");
        assert!(trifft("cccaaa", "aaa"), "am Ende");
        assert!(!trifft("bbbccc.rs", "aaa"));
    }

    /// C1.2: die Schreibung des Namens spielt keine Rolle.
    #[test]
    fn die_schreibung_des_namens_spielt_keine_rolle() {
        assert!(trifft("LIESMICH.TXT", "liesmich"));
        assert!(trifft("Banane.txt", "nan"));
    }

    /// C1.3: gefaltet wird nichts. `apfel` findet `Äpfel` nicht.
    #[test]
    fn der_vergleich_faltet_keine_umlaute_und_keine_akzente() {
        assert!(!trifft("Äpfel.txt", "apfel"));
        assert!(!trifft("Cafe.txt", "café"));
        assert!(
            trifft("Äpfel.txt", "äpfel"),
            "kleingeschrieben findet der Umlaut sich selbst"
        );
    }

    /// Die Asymmetrie der beiden Argumente, seit der Runde 21 vom Typ
    /// gehalten: `Muster::aus` schreibt einmal klein, und kein Rufer kann dem
    /// Vergleich einen ungeschriebenen Text reichen.
    #[test]
    fn das_muster_schreibt_einmal_klein() {
        assert!(trifft("Banane.txt", "Banane"));
        assert_eq!(Muster::aus("BaNaNe"), Muster::aus("banane"));
    }

    #[test]
    fn ein_leerer_filtertext_steht_in_jedem_namen() {
        assert!(trifft("beliebig.txt", ""));
    }

    /// C5.2 (B1): `*` steht fuer eine beliebige, auch leere Folge, und die
    /// Reihenfolge der Stuecke zaehlt.
    #[test]
    fn ein_stern_steht_fuer_eine_beliebige_auch_leere_folge() {
        assert!(trifft("ab", "a*b"), "leer");
        assert!(trifft("a-b", "a*b"));
        assert!(trifft("a-lange-folge-b", "a*b"));
        assert!(!trifft("ba", "a*b"), "die Reihenfolge zaehlt");
    }

    /// C5.3 (B1): zwei `*` nebeneinander sind eines, und lauter `*` treffen
    /// jeden Namen.
    #[test]
    fn zwei_sterne_sind_einer_und_lauter_sterne_treffen_jeden_namen() {
        for name in ["ab", "a-b", "a-lange-folge-b", "ba"] {
            assert_eq!(trifft(name, "a**b"), trifft(name, "a*b"), "{name}");
        }
        for name in ["", "x", "beliebig.txt", "Äpfel.txt"] {
            assert!(trifft(name, "*"), "{name:?} gegen `*`");
            assert!(trifft(name, "***"), "{name:?} gegen `***`");
        }
    }

    /// C5.4 (B2): der Platzhalter verankert nichts. `*abc`, `abc*` und `*abc*`
    /// treffen genau, was `abc` trifft.
    #[test]
    fn ein_stern_am_rand_verankert_nichts() {
        for name in ["abc", "xabc", "abcx", "xabcx", "axbc"] {
            let erwartet = trifft(name, "abc");
            for filtertext in ["*abc", "abc*", "*abc*"] {
                assert_eq!(
                    trifft(name, filtertext),
                    erwartet,
                    "{name} gegen {filtertext}"
                );
            }
        }
        assert!(
            trifft("xabcx", "abc"),
            "die Teilfolge bleibt eine Teilfolge"
        );
        assert!(!trifft("axbc", "abc"));
    }

    /// C5.5 (B3): `?` und `[` sind gewoehnliche Zeichen, und ein woertliches
    /// `*` im Namen laesst sich nicht gezielt suchen.
    #[test]
    fn es_gibt_kein_zweites_sonderzeichen_und_kein_entkommen() {
        assert!(trifft("a?b.txt", "a?b"));
        assert!(!trifft("axb.txt", "a?b"), "`?` steht fuer kein Zeichen");
        assert!(trifft("a[b.txt", "a[b"));
        assert!(!trifft("ab.txt", "a[b"));
        assert!(
            trifft("a*b.txt", "a*b"),
            "das woertliche `*` trifft der Platzhalter"
        );
        assert!(
            trifft("axb.txt", "a*b"),
            "und kein Filtertext trifft es allein"
        );
        assert!(
            trifft("a\\*b.txt", "a\\*b"),
            "der Rueckstrich ist ein Zeichen, kein Entkommen"
        );
        assert!(trifft("a\\xb.txt", "a\\*b"));
    }

    /// C5.6 (B4): Schreibung egal, Faltung keine, auch mit Platzhalter.
    #[test]
    fn die_schreibung_bleibt_und_gefaltet_wird_nichts() {
        assert!(trifft("Äpfel.txt", "Ä*.txt"));
        assert!(trifft("äpfel.txt", "Ä*.txt"));
        assert!(!trifft("Äpfel.txt", "a*.txt"));
        assert!(!trifft("äpfel.txt", "a*.txt"));
        assert!(trifft("ab", "A*B"));
    }

    /// C7.3 (B7): jedes Stueck wird ab dem Ende des vorigen genau einmal
    /// gesucht, und das genuegt.
    #[test]
    fn der_vergleich_sucht_jedes_stueck_genau_einmal_ab_dem_ende_des_vorigen() {
        assert!(trifft("aaa", "a*a*a"));
        assert!(!trifft("aa", "a*a*a"), "zwei `a` tragen drei Stuecke nicht");
        assert!(trifft("a-a-a", "a*a*a"));
        assert!(trifft("aab", "aa*b"), "ohne Ueberlappung");
        assert!(!trifft("ab", "aa*b"));
    }

    /// C5.7 (B3): die Zeichenregel nimmt das `*`, vor und nach der Runde.
    #[test]
    fn traegt_ein_dateiname_nimmt_den_stern() {
        assert!(traegt_ein_dateiname('*'));
    }

    /// C5.1 (B1), Vergleichshaelfte: der Markerfall aus dem Backlog-Eintrag.
    #[test]
    fn der_marker_zwischen_zwei_unterstrichen_trifft_jeden_marker_und_keinen_fehlenden() {
        let filtertext = "260503-1144_*_f1";
        assert!(trifft("260503-1144_d_f1-zitadel.md", filtertext));
        assert!(trifft("260503-1144_c_f1-zitadel.md", filtertext));
        assert!(
            !trifft("260503-1144-f1-zitadel.md", filtertext),
            "ohne Marker fehlen die zwei Unterstriche"
        );
    }

    /// Die Staffelung, ausgeschrieben: drei Zeichen flach, fuenf tief.
    #[test]
    fn die_inhaltsschwelle_steht_bei_drei_und_bei_fuenf() {
        assert_eq!(inhaltsschwelle(false), 3, "ohne tiefe Suche");
        assert_eq!(inhaltsschwelle(true), 5, "mit tiefer Suche");
    }
}
