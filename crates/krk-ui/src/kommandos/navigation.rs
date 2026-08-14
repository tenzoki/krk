//! Die Auswahl durch die Liste bewegen (C2).
//!
//! Vier Befehle, eine Rechnung: Pfeil auf und ab um einen Eintrag, Bild auf und
//! ab um eine Bildschirmseite, und je ein Befehl an den Anfang und an das Ende
//! der Liste. Alle vier laufen ueber [`zielzeile`]; eine zweite Stelle, die
//! eine Zeilennummer begrenzt, entsteht nicht.
//!
//! Die Rechnung steht hier und nicht in der Ansicht, weil sie am Rand der Liste
//! entscheidet und genau dort die Fehler sitzen: um eins zu weit, um eins zu
//! kurz, oder der Umlauf, den C2 nicht will.
//!
//! Aus demselben Grund steht [`ersatzzeile`] daneben: sie beantwortet, welche
//! Zeile die Auswahl bekommt, wenn ihre eigene weggefallen ist (C1.11 der
//! Filter-Runde). Auch das ist eine Randentscheidung — die leere Liste, die
//! Auswahl, die es nie gab, und die, die nur gerade nicht zu sehen ist —, und
//! auch sie braucht keine `NSTableView`, um geprueft zu werden.

/// Wohin sich die Auswahl bewegt (C2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bewegung {
    /// Um so viele Zeilen, negativ nach oben.
    ///
    /// Traegt die Pfeiltasten mit `±1` und die Seitentasten mit der Zahl der
    /// sichtbaren Zeilen, die allein die Tabelle kennt.
    Um(isize),
    /// An den Anfang der Liste.
    Anfang,
    /// An das Ende der Liste.
    Ende,
}

/// Die Zeile, auf die die Auswahl durch diese Bewegung rueckt.
///
/// `jetzt` ist die ausgewaehlte Zeile, negativ, solange nichts ausgewaehlt ist;
/// das ist der Wert, den `NSTableView.selectedRow` liefert. `None` heisst: die
/// Liste ist leer, und es gibt nichts auszuwaehlen.
///
/// **Am Rand bleibt die Auswahl stehen, statt umzulaufen.** Ein Umlauf von der
/// letzten auf die erste Zeile setzte den Nutzer in einem Ordner mit 100.000
/// Eintraegen an eine Stelle, die er nicht angesteuert hat.
///
/// **Ohne bestehende Auswahl faengt sie an dem Rand an, aus dem die Bewegung
/// kommt.** Pfeil ab setzt auf die erste Zeile, Pfeil auf auf die letzte. Der
/// erste Tastendruck in einer frisch gelesenen Liste tut damit das, was er
/// aussieht.
pub fn zielzeile(bewegung: Bewegung, jetzt: isize, zeilen: usize) -> Option<usize> {
    let letzte = isize::try_from(zeilen.checked_sub(1)?).ok()?;
    let ziel = match bewegung {
        Bewegung::Anfang => 0,
        Bewegung::Ende => letzte,
        Bewegung::Um(schritte) if jetzt < 0 => {
            if schritte < 0 {
                letzte
            } else {
                0
            }
        }
        Bewegung::Um(schritte) => jetzt.saturating_add(schritte).clamp(0, letzte),
    };
    usize::try_from(ziel).ok()
}

/// Die Zeile, auf die die Auswahl rueckt, nachdem sich die Sicht verkuerzt hat
/// (C1.11).
///
/// `None` heisst: hier ist nichts umzusetzen, die bestehende Auswahl gilt
/// weiter. `Some(zeile)` heisst: diese Zeile ist neu auszuwaehlen.
///
/// Die drei Eingaben sind genau die, die der Fall braucht, und keine mehr.
/// `hatte_auswahl` sagt, ob das Modell ueberhaupt einen Eintrag als
/// ausgewaehlt fuehrt; `zeile_jetzt` ist die Zeile dieses Eintrags in der neuen
/// Sicht, `None`, sobald der Filter ihn ausblendet; `zeilen` ist die Zahl der
/// Zeilen, die die neue Sicht traegt.
///
/// **Ohne bestehende Auswahl entsteht keine.** Ein Filter, der die Auswahl auf
/// die erste Zeile setzt, wo vorher keine stand, waere ein Sprung, und C1.1
/// sagt zu, dass die Auswahl nicht springt. Weggefallen ist eine Zeile nur
/// dann, wenn es sie vorher gab.
///
/// **Zeigt die Sicht keine Zeile, bleibt die Auswahl leer.** Ein Befehl, der
/// eine braeuchte, tut dann nichts und meldet nichts; das ist das bestehende
/// Verhalten von `crate::kommandos::operationen::betroffene` und keine Regel
/// dieser Stelle.
///
/// **Die erste Zeile und nicht die naechstgelegene.** Welche Zeile der
/// weggefallenen am naechsten liegt, waere eine Rechnung ueber eine Ordnung,
/// die der Filter gerade zerrissen hat: die Eintraege dazwischen sind weg, und
/// „daneben" hiesse in der alten Sicht etwas anderes als in der neuen.
pub fn ersatzzeile(
    hatte_auswahl: bool,
    zeile_jetzt: Option<usize>,
    zeilen: usize,
) -> Option<usize> {
    if zeile_jetzt.is_some() || !hatte_auswahl || zeilen == 0 {
        return None;
    }
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eine_leere_liste_hat_keine_zielzeile() {
        for bewegung in [
            Bewegung::Um(1),
            Bewegung::Um(-1),
            Bewegung::Anfang,
            Bewegung::Ende,
        ] {
            assert_eq!(zielzeile(bewegung, -1, 0), None);
            assert_eq!(zielzeile(bewegung, 0, 0), None);
        }
    }

    #[test]
    fn ohne_auswahl_faengt_die_bewegung_an_ihrem_rand_an() {
        assert_eq!(zielzeile(Bewegung::Um(1), -1, 10), Some(0));
        assert_eq!(zielzeile(Bewegung::Um(-1), -1, 10), Some(9));
        assert_eq!(
            zielzeile(Bewegung::Um(20), -1, 10),
            Some(0),
            "auch eine Bildschirmseite abwaerts faengt oben an"
        );
    }

    #[test]
    fn am_rand_bleibt_die_auswahl_stehen() {
        assert_eq!(zielzeile(Bewegung::Um(-1), 0, 10), Some(0));
        assert_eq!(zielzeile(Bewegung::Um(1), 9, 10), Some(9));
        assert_eq!(zielzeile(Bewegung::Um(-40), 3, 10), Some(0));
        assert_eq!(zielzeile(Bewegung::Um(40), 3, 10), Some(9));
    }

    #[test]
    fn anfang_und_ende_treffen_die_raender() {
        assert_eq!(zielzeile(Bewegung::Anfang, 5, 10), Some(0));
        assert_eq!(zielzeile(Bewegung::Ende, 5, 10), Some(9));
        assert_eq!(
            zielzeile(Bewegung::Anfang, -1, 10),
            Some(0),
            "auch ohne bestehende Auswahl"
        );
        assert_eq!(zielzeile(Bewegung::Ende, -1, 10), Some(9));
    }

    /// Die Saettigung greift, bevor der Ueberlauf kommt.
    #[test]
    fn ein_riesiger_schritt_laeuft_nicht_ueber() {
        assert_eq!(zielzeile(Bewegung::Um(isize::MAX), 5, 10), Some(9));
        assert_eq!(zielzeile(Bewegung::Um(isize::MIN), 5, 10), Some(0));
    }

    #[test]
    fn eine_liste_mit_einer_zeile_bewegt_sich_nicht() {
        for bewegung in [
            Bewegung::Um(1),
            Bewegung::Um(-1),
            Bewegung::Anfang,
            Bewegung::Ende,
        ] {
            assert_eq!(zielzeile(bewegung, 0, 1), Some(0));
        }
    }

    /// Steht die Auswahl noch in der Sicht, ist nichts umzusetzen.
    #[test]
    fn eine_sichtbare_auswahl_bleibt_stehen() {
        assert_eq!(ersatzzeile(true, Some(0), 10), None);
        assert_eq!(ersatzzeile(true, Some(7), 10), None);
        assert_eq!(
            ersatzzeile(true, Some(3), 4),
            None,
            "auch die letzte Zeile der neuen Sicht"
        );
    }

    /// Der tragende Fall aus C1.11: die Zeile faellt weg, die Auswahl rueckt
    /// nach oben.
    #[test]
    fn eine_weggefallene_auswahl_geht_auf_die_erste_zeile() {
        assert_eq!(ersatzzeile(true, None, 10), Some(0));
        assert_eq!(ersatzzeile(true, None, 1), Some(0));
    }

    /// Ohne bestehende Auswahl entsteht keine: der Filter springt nicht (C1.1).
    #[test]
    fn ohne_bestehende_auswahl_entsteht_keine() {
        assert_eq!(ersatzzeile(false, None, 10), None);
        assert_eq!(
            ersatzzeile(false, None, 0),
            None,
            "und in der leeren Sicht erst recht nicht"
        );
    }

    /// Zeigt die Sicht keine Zeile, bleibt die Auswahl leer (C1.11, zweiter
    /// Satz).
    #[test]
    fn eine_leere_sicht_bekommt_keine_auswahl() {
        assert_eq!(ersatzzeile(true, None, 0), None);
    }
}
