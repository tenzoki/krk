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
}
