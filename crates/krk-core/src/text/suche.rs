//! Suchen und Ersetzen in der geoeffneten Datei (C5).
//!
//! ```text
//!  alle(text, gesucht) ──> Vec<Treffer>
//!            │
//!            ├──> erster_ab(treffer, versatz)   der erste ab der Schreibmarke
//!            ├──> naechster(treffer, versatz)   der darauffolgende
//!            └──> voriger(treffer, versatz)     der davorliegende
//!
//!  einen_ersetzen(text, gesucht, ersatz, treffer) ──> Ersetzung
//!  alle_ersetzen(text, gesucht, ersatz)           ──> Sammelersetzung
//! ```
//!
//! # Gesucht wird buchstaeblich und ueber den ganzen Text
//!
//! Gross- und Kleinschreibung, regulaere Ausdruecke und die Suchrichtung sind
//! nach dem Spec **nicht** festgelegt und kommen nicht hinzu. Der Spec sagt
//! zu, dass gesucht und ersetzt wird, und nicht, mit welchen Schaltern; jeder
//! Schalter waere ein Bedienelement und ein Abnahmekriterium mehr. Diese Datei
//! kennt deshalb keinen einzigen.
//!
//! Die Treffer ueberlappen nicht: `aa` kommt in `aaa` einmal vor und nicht
//! zweimal. Das ist die Zaehlweise von [`str::match_indices`], auf der die
//! Suche steht, und es ist dieselbe, die das Ersetzen braucht, weil zwei
//! ueberlappende Treffer sich nicht beide ersetzen liessen.
//!
//! # Ein leerer Suchtext hat null Treffer
//!
//! Er ist kein Sonderfall des Ersetzens, sondern einer der Suche, und wird
//! deshalb genau einmal behandelt, naemlich in [`alle`]. Alles Uebrige folgt
//! daraus: ohne Treffer aendert das Ersetzen nichts und zaehlt 0.
//!
//! **Das ist zugleich der Grund, aus dem hier nicht [`str::replace`] steht.**
//! Es setzt seinen Ersatz bei leerem Suchtext an jede Zeichengrenze:
//! `"abc".replace("", "-")` liefert `"-a-b-c-"`. Das Abnahmekriterium
//! verlangt das Gegenteil, und ein Ersetzen, das auf einer anderen
//! Trefferzaehlung stuende als die Suche daneben, waere die zweite Wahrheit
//! darueber, was ein Treffer ist.
//!
//! # Das Ersetzen schreibt nichts
//!
//! Es liefert eine neue Zeichenkette. Ein Ersetzen ist damit eine ungesicherte
//! Aenderung im Sinne von C4, wie das achte Abnahmekriterium von C5 es
//! verlangt, und es ist keine Zusage, die jemand einhalten muss: dieses Modul
//! kann gar nicht schreiben.

/// Ein Fund als Byteversatzbereich, `ende` ausschliesslich.
///
/// Beide Grenzen liegen auf Zeichengrenzen, solange der Treffer aus [`alle`]
/// stammt. Der Modulkopf von [`crate::text`] sagt, warum das die Bedingung
/// jeder Verwendung ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Treffer {
    /// Wo der Fund beginnt.
    pub anfang: usize,
    /// Wo er endet, ausschliesslich.
    pub ende: usize,
}

/// Was ein einzelnes Ersetzen hinterlaesst.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ersetzung {
    /// Der neue Stand.
    pub stand: String,
    /// Der naechste Treffer **im neuen Stand**, oder `None`, wenn keiner mehr
    /// folgt.
    pub naechster: Option<Treffer>,
}

/// Was ein Ersetzen aller Treffer in einem Zug hinterlaesst.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sammelersetzung {
    /// Der neue Stand.
    pub stand: String,
    /// Wie viele Treffer ersetzt wurden. C5 verlangt, dass der Editor die Zahl
    /// danach nennt.
    pub zahl: usize,
}

/// Alle Treffer der Zeichenfolge `gesucht` im Text, in Textreihenfolge.
///
/// Ein leerer Suchtext liefert keinen Treffer.
pub fn alle(text: &str, gesucht: &str) -> Vec<Treffer> {
    if gesucht.is_empty() {
        return Vec::new();
    }
    text.match_indices(gesucht)
        .map(|(anfang, stueck)| Treffer {
            anfang,
            ende: anfang + stueck.len(),
        })
        .collect()
}

/// Der erste Treffer, der bei `versatz` oder dahinter beginnt.
///
/// Das ist die Wahl beim Start einer Suche: der Treffer unter der Schreibmarke
/// zaehlt mit, sonst uebergaenge eine frisch begonnene Suche genau die Stelle,
/// an der der Nutzer steht. Liegt hinter `versatz` keiner mehr, laeuft die
/// Suche um und liefert den ersten des Textes. Ohne Treffer `None`.
pub fn erster_ab(treffer: &[Treffer], versatz: usize) -> Option<usize> {
    let ab = treffer.partition_point(|kandidat| kandidat.anfang < versatz);
    umlaufen(treffer, ab)
}

/// Der erste Treffer **hinter** `versatz`.
///
/// Das ist die Wahl beim Weitergehen: der Treffer, auf dem die Schreibmarke
/// gerade steht, zaehlt nicht mit, sonst bewegte sich der Befehl nicht. Hinter
/// dem letzten geht es beim ersten weiter. Ohne Treffer `None`.
pub fn naechster(treffer: &[Treffer], versatz: usize) -> Option<usize> {
    let ab = treffer.partition_point(|kandidat| kandidat.anfang <= versatz);
    umlaufen(treffer, ab)
}

/// Der letzte Treffer **vor** `versatz`.
///
/// Vor dem ersten geht es beim letzten weiter. Ohne Treffer `None`.
pub fn voriger(treffer: &[Treffer], versatz: usize) -> Option<usize> {
    let davor = treffer.partition_point(|kandidat| kandidat.anfang < versatz);
    // Einen zurueck, im Ring gerechnet: `umlaufen` nimmt keine Stelle vor der
    // ersten, deshalb steht der Schritt zurueck als Schritt um `len - 1` nach
    // vorn. Auf der leeren Liste ist der Summand 0 und `umlaufen` antwortet
    // `None`, wie es das fuer alle drei tut.
    umlaufen(treffer, davor + treffer.len().saturating_sub(1))
}

/// Die drei Auswahlfunktionen laufen um, und das ist die eine Stelle, an der
/// sie es tun: die Stelle wird im **Ring** der Trefferliste gerechnet, und die
/// leere Liste hat keine.
///
/// Umlaufen statt anhalten, weil die Zaehlung, die C5 zusagt ("der wievielte
/// gerade angesteuert ist"), eine Runde durch die Trefferliste beschreibt.
///
/// Der Ring und nicht ein "sonst der erste": beide Richtungen laufen um, und
/// nur die Restrechnung traegt beide. Wer sie zu `if stelle < len` verkuerzt,
/// nimmt [`voriger`] seinen Umlauf und zwingt ihn, sich einen eigenen zu
/// schreiben — genau das war der Defekt `260808-1413`.
fn umlaufen(treffer: &[Treffer], stelle: usize) -> Option<usize> {
    if treffer.is_empty() {
        return None;
    }
    Some(stelle % treffer.len())
}

/// Ersetzt den angesteuerten Treffer und nennt den naechsten.
///
/// `treffer` stammt aus [`alle`] ueber denselben `text`; ein von Hand gebauter
/// Bereich mit einer Grenze mitten in einer Mehrbytefolge laesst den Zugriff
/// in Panik enden.
///
/// **Der naechste Treffer wird im neuen Stand gesucht und laeuft nicht um.**
/// Beides folgt aus derselben Ueberlegung: der Ersatztext kann den Suchtext
/// enthalten, und ein Umlauf schickte den Nutzer zurueck in genau das, was er
/// eben eingesetzt hat. Gesucht wird deshalb ab dem Ende des eingesetzten
/// Textes; hinter dem letzten Treffer ist der Durchgang zu Ende, und der
/// Aufrufer meldet das.
pub fn einen_ersetzen(text: &str, gesucht: &str, ersatz: &str, treffer: Treffer) -> Ersetzung {
    let mut stand =
        String::with_capacity(text.len() - (treffer.ende - treffer.anfang) + ersatz.len());
    stand.push_str(&text[..treffer.anfang]);
    stand.push_str(ersatz);
    stand.push_str(&text[treffer.ende..]);

    let hinter = treffer.anfang + ersatz.len();
    let naechster = alle(&stand, gesucht)
        .into_iter()
        .find(|kandidat| kandidat.anfang >= hinter);
    Ersetzung { stand, naechster }
}

/// Ersetzt alle Treffer in einem Zug und nennt ihre Zahl.
///
/// **Der Lauf endet auch dann, wenn der Ersatztext den Suchtext enthaelt.** Er
/// steht auf der Trefferliste des **alten** Standes, die einmal gebildet wird;
/// was der Ersatz an neuen Treffern erzeugt, kann er deshalb nicht mehr
/// erreichen. Ein Lauf, der nach jedem Ersatz erneut suchte, ersetzte `foo`
/// durch `foofoo` bis zum Speicherende.
pub fn alle_ersetzen(text: &str, gesucht: &str, ersatz: &str) -> Sammelersetzung {
    let treffer = alle(text, gesucht);
    if treffer.is_empty() {
        return Sammelersetzung {
            stand: text.to_owned(),
            zahl: 0,
        };
    }

    let ersetzte_bytes: usize = treffer
        .iter()
        .map(|kandidat| kandidat.ende - kandidat.anfang)
        .sum();
    let mut stand =
        String::with_capacity(text.len() - ersetzte_bytes + treffer.len() * ersatz.len());
    let mut gelesen = 0;
    for kandidat in &treffer {
        stand.push_str(&text[gelesen..kandidat.anfang]);
        stand.push_str(ersatz);
        gelesen = kandidat.ende;
    }
    stand.push_str(&text[gelesen..]);

    Sammelersetzung {
        stand,
        zahl: treffer.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn treffer_ueberlappen_nicht() {
        assert_eq!(alle("aaa", "aa"), [Treffer { anfang: 0, ende: 2 }]);
    }

    #[test]
    fn ein_leerer_suchtext_hat_keine_treffer() {
        assert!(alle("abc", "").is_empty());
    }

    #[test]
    fn der_erste_ab_einem_versatz_zaehlt_den_treffer_unter_der_schreibmarke_mit() {
        let liste = alle("ab ab ab", "ab");
        assert_eq!(erster_ab(&liste, 3), Some(1));
        assert_eq!(naechster(&liste, 3), Some(2), "das Weitergehen nicht");
    }

    #[test]
    fn die_auswahl_laeuft_in_beide_richtungen_um() {
        let liste = alle("ab ab ab", "ab");
        assert_eq!(naechster(&liste, 6), Some(0), "hinter dem letzten");
        assert_eq!(voriger(&liste, 0), Some(2), "vor dem ersten");
        assert_eq!(voriger(&liste, 6), Some(1));
    }

    /// Der einzige Treffer ist der scharfe Fall der Ringrechnung: jede der drei
    /// Richtungen kommt auf ihn zurueck, und `voriger` rechnet dabei mit dem
    /// Summanden 0.
    #[test]
    fn ein_einziger_treffer_wird_aus_jeder_richtung_wieder_erreicht() {
        let liste = alle("ab", "ab");
        assert_eq!(erster_ab(&liste, 0), Some(0));
        assert_eq!(naechster(&liste, 0), Some(0), "hinter dem einzigen");
        assert_eq!(voriger(&liste, 0), Some(0), "vor dem einzigen");
    }

    #[test]
    fn ohne_treffer_waehlt_keine_der_drei_etwas_aus() {
        let liste: Vec<Treffer> = Vec::new();
        assert_eq!(erster_ab(&liste, 0), None);
        assert_eq!(naechster(&liste, 0), None);
        assert_eq!(voriger(&liste, 0), None);
    }

    #[test]
    fn ein_einzelnes_ersetzen_nennt_den_naechsten_im_neuen_stand() {
        let text = "foo bar foo";
        let liste = alle(text, "foo");
        let ergebnis = einen_ersetzen(text, "foo", "baz", liste[0]);
        assert_eq!(ergebnis.stand, "baz bar foo");
        assert_eq!(
            ergebnis.naechster,
            Some(Treffer {
                anfang: 8,
                ende: 11
            })
        );
    }

    #[test]
    fn ein_einzelnes_ersetzen_geht_nicht_in_den_eigenen_ersatz() {
        let text = "foo";
        let liste = alle(text, "foo");
        let ergebnis = einen_ersetzen(text, "foo", "foofoo", liste[0]);
        assert_eq!(ergebnis.stand, "foofoo");
        assert_eq!(ergebnis.naechster, None);
    }

    #[test]
    fn hinter_dem_letzten_treffer_ist_der_durchgang_zu_ende() {
        let text = "foo bar";
        let liste = alle(text, "foo");
        let ergebnis = einen_ersetzen(text, "foo", "baz", liste[0]);
        assert_eq!(ergebnis.naechster, None, "kein Umlauf beim Ersetzen");
    }
}
