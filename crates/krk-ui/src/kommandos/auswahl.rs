//! Die Mehrfachauswahl aus C2, soweit sie mehr ist als Zustand.
//!
//! Drei der vier Markierungsbefehle sind reiner Zustand des Ordnermodells und
//! stehen dort: alles markieren, jede Markierung aufheben, die Markierung
//! umkehren. Der vierte ist eine Verbindung aus zweien, "markieren **und**
//! weiterruecken", und braucht dafuer die Zeilen. Er steht hier.
//!
//! Dazu die Wendung, mit der die Statuszeile den Markierungsstand nennt. Sie
//! steht hier und nicht in [`super::operationen`], weil sie zu C2 gehoert und
//! nicht zu C4; die beiden Bausteine, aus denen sie sich zusammensetzt, leiht
//! sie sich dort, statt sie ein zweites Mal zu schreiben.

use krk_core::verzeichnis::{Markierungsstand, Ordnermodell};

use super::navigation::{Bewegung, zielzeile};
use super::operationen::{ordner_text, zahl};

/// Markiert den Eintrag der genannten Zeile und rueckt weiter (C2).
///
/// Liefert die Zeile, auf die die Auswahl danach steht. In der letzten Zeile
/// bleibt sie stehen, wie bei jeder anderen Bewegung auch: der Eintrag ist
/// markiert, und ein Umlauf an den Listenanfang begaenne eine zweite Runde, in
/// der jeder Tastendruck die Markierung wieder abraeumte.
pub fn markieren_und_weiter(modell: &mut Ordnermodell, zeile: usize) -> Option<usize> {
    let eintrag = modell.eintragsindex(zeile)?;
    modell.markierung_umschalten(eintrag);
    let jetzt = isize::try_from(zeile).ok()?;
    zielzeile(Bewegung::Um(1), jetzt, modell.zeilenzahl())
}

/// Der fuenfte Rang der Statuszeile: was im sichtbaren Tab markiert ist (C2).
///
/// `None`, wenn nichts markiert ist; dann bleibt der Rang stumm und die Zeile
/// leer, statt eine Null anzuzeigen, die in den meisten Augenblicken dastuende.
///
/// Die Groesse kommt fertig formatiert herein und wird hier nicht gerechnet:
/// sie beschriftet derselbe `NSByteCountFormatter`, der die Groessenspalte des
/// Dateifensters beschriftet, und der steht in `appkit/`. Diese Funktion bleibt
/// dadurch ohne AppKit und pruefbar.
///
/// Die Ordnerzahl steht auch dann da, wenn sie null ist. Sie wegzulassen waere
/// eine zweite Wortform mit einer eigenen Bedingung, und der Nutzer muesste aus
/// ihrer Abwesenheit schliessen, dass kein Ordner markiert ist.
pub fn markierungsstand_text(stand: Markierungsstand, groesse: &str) -> Option<String> {
    if stand.ist_leer() {
        return None;
    }
    Some(format!(
        "{} markiert, davon {}, {groesse}",
        zahl(stand.zahl),
        ordner_text(stand.ordner)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stand(zahl: usize, ordner: usize, groesse: u64) -> Markierungsstand {
        Markierungsstand {
            zahl,
            ordner,
            groesse,
        }
    }

    #[test]
    fn ohne_markierung_bleibt_der_rang_stumm() {
        assert_eq!(markierungsstand_text(stand(0, 0, 0), "0 Bytes"), None);
    }

    #[test]
    fn der_text_nennt_zahl_ordnerzahl_und_groesse() {
        assert_eq!(
            markierungsstand_text(stand(12, 3, 4_200_000), "4,2 MB").as_deref(),
            Some("12 markiert, davon 3 Ordner, 4,2 MB")
        );
    }

    #[test]
    fn die_ordnerzahl_steht_auch_dann_da_wenn_sie_null_ist() {
        assert_eq!(
            markierungsstand_text(stand(1, 0, 12), "12 Bytes").as_deref(),
            Some("1 markiert, davon 0 Ordner, 12 Bytes")
        );
    }

    #[test]
    fn grosse_zahlen_bekommen_dieselben_tausenderpunkte_wie_ein_vorgang() {
        assert_eq!(
            markierungsstand_text(stand(5_000, 1, 0), "1,2 GB").as_deref(),
            Some("5.000 markiert, davon ein Ordner, 1,2 GB")
        );
    }
}
