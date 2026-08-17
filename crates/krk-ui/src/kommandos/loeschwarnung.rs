//! Die Texte der einen Rueckfrage vor dem Raeumen in den Papierkorb (C2, C3).
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Das Blatt selbst baut
//! `crate::appkit::blaetter::loeschbestaetigung`; was darin *steht*, entsteht
//! hier und ist ohne Fenster pruefbar.
//!
//! ```text
//!  auswahl ─┬──> frage_und_erlaeuterung() ──> (Frage, Erlaeuterung)
//!  ordner ──┘
//! ```
//!
//! # Warum die Texte der Loeschfrage eigens dastehen
//!
//! Nach dieser Runde kennt KRK genau einen Loeschweg, und er fragt vorher
//! genau einmal nach. Ein Wortlaut, der an zwei Stellen entstuende, waere zwei
//! Wahrheiten ueber dieselbe Frage; deshalb steht er hier und nicht im Blatt,
//! das ihn zeigt, und nicht in [`super::operationen`], das die Texte aller
//! uebrigen Dateioperationen traegt. `operationen::loeschfrage`, der Wortlaut
//! des endgueltigen Loeschens, faellt mit diesem Loeschweg weg.
//!
//! # Warum der Pfad ungekuerzt dasteht
//!
//! Die Erlaeuterung nennt den vollen Pfad des Ordners, aus dem geraeumt wird,
//! und **nicht** die gekuerzte Form aus
//! `krk_core::ablage::pfade::gekuerzt_fuer_anzeige`, die aus dem
//! Benutzerverzeichnis eine Tilde macht.
//!
//! Der Anlass dieser Runde ist ein Schadensfall: KRK hat am 260817-0344 auf
//! einen einzigen Tastendruck 189 verfolgte Dateien des eigenen Projekts in den
//! Papierkorb geraeumt, ohne Rueckfrage und vier Stunden unbemerkt. Die
//! Erlaeuterung ist die eine Stelle, an der der Nutzer erkennen soll, **welchen**
//! Ordner er gerade leert. Ein `~` an der Stelle des Benutzerverzeichnisses
//! nimmt ihm genau diese Auskunft: es macht zwei verschiedene Orte gleich
//! aussehend und spart dabei die Zeichen, die den Unterschied tragen. Die
//! Kuerzung bleibt der Erfolgsmeldung der Tastenbelegung vorbehalten, wo eine
//! Verwechslung nichts kostet.
//!
//! Gebaut wird der Pfad ueber [`super::operationen::pfadtext`], das dieselbe
//! Entscheidung schon fuer die beiden Pfadkopierer getroffen und begruendet
//! hat. Ein zweiter Pfadformatierer daneben waere die erste Abweichung, die
//! niemand prueft.
//!
//! # Der eine Aufrufer
//!
//! `Anwendungsdelegierter::loeschen_nach_rueckfrage` (`crate::appkit::anwendung`)
//! ist der einzige, und er ist es fuer jeden Loeschbefehl: die beiden Tasten
//! und der Menueeintrag laufen durch denselben Rumpf. Ein zweiter Aufrufer
//! waere ein zweiter Loeschweg, und genau den schafft diese Runde ab. Die
//! Aufruferzaehlung dazu steht in der Form von
//! `die_regel_hat_genau_einen_aufrufer` in [`super::rueckschritt`]; sie kommt
//! mit der Tafel der Ausloeser, weil erst diese die Zusage traegt, dass die
//! Einordnung des Ziels einmal geschieht.
//!
//! **Bis dahin traegt [`frage_und_erlaeuterung`]
//! `#[cfg_attr(not(test), expect(dead_code, ...))]`**, weil ihr Aufrufer noch
//! nicht da ist: er entsteht mit dem dritten Schritt dieser Runde, der den
//! gemeinsamen Rumpf beider Loeschbefehle zieht. Bis dahin erreichen nur die
//! Proben darunter die Funktion, und `krk-ui` ist ein Binaerziel, in dem `pub`
//! allein noch keine Verwendung ist.
//!
//! **`expect` und nicht `allow`, und darin liegt das Ablaufdatum.** Mit dem
//! Aufrufer wird die Erwartung unerfuellt, und der Bau haelt unter
//! `-D warnings` an, bis die Zeilen weg sind. Eine Ausnahme mit Ablaufdatum
//! statt einer, die stehen bleibt und niemandem mehr sagt, warum. Dieselbe
//! Bauform hat [`super::rueckschritt`] in der Runde 10 getragen.

use std::path::Path;

use super::operationen::{Auswahl, ordner_text, pfadtext, zahl};

/// Die beiden Zeilen der Rueckfrage vor dem Raeumen in den Papierkorb (C2).
///
/// Genau einmal je Vorgang, unabhaengig von der Zahl der Eintraege und
/// unabhaengig davon, welche Taste den Befehl ausgeloest hat.
///
/// **Die Frage** nennt in ihrer ersten Zeile, wie viele Eintraege betroffen
/// sind, und bei einem einzelnen steht dort die Einzahl. Sie nennt das Ziel des
/// Vorgangs beim Namen — „in den Papierkorb raeumen" und nicht „loeschen" —,
/// weil der Rueckweg ueber den Papierkorb der Unterschied zum Weg ist, den
/// diese Runde abschafft.
///
/// **Die Erlaeuterung** nennt den vollen Pfad des Ordners, aus dem geraeumt
/// wird, und, falls Ordner unter der Auswahl sind, deren Zahl gesondert. Der
/// Vorgang betrifft genau einen Ordner, also nennt sie genau einen Pfad: jeder
/// Pfad der Auswahl entsteht in [`super::operationen::betroffene`] aus dem
/// angezeigten Ordner und dem Namen einer sichtbaren Zeile, auch bei
/// eingeschalteter tiefer Suche.
///
/// Warum der Pfad ungekuerzt dasteht, sagt der Modulkopf.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "der Aufrufer entsteht mit dem dritten Schritt dieser Runde, siehe Modulkopf"
    )
)]
pub fn frage_und_erlaeuterung(auswahl: &Auswahl, ordner: &Path) -> (String, String) {
    let frage = match auswahl.zahl() {
        1 => "Diesen Eintrag in den Papierkorb räumen?".to_owned(),
        anzahl => format!("Diese {} Einträge in den Papierkorb räumen?", zahl(anzahl)),
    };
    let mut erlaeuterung = format!("Geräumt wird aus {}.", pfadtext(ordner));
    if auswahl.ordner > 0 {
        erlaeuterung.push_str(&format!(
            "\n\nDarunter {}, jeweils mit ihrem gesamten Inhalt.",
            ordner_text(auswahl.ordner)
        ));
    }
    (frage, erlaeuterung)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    /// Eine Auswahl aus `anzahl` Eintraegen, davon `ordner` Ordner.
    ///
    /// Die Pfade stehen nur fuer ihre Zahl; welche es sind, sagt die Frage
    /// nicht, und die Erlaeuterung nennt den Ordner und nicht die Eintraege.
    fn auswahl(anzahl: usize, ordner: usize) -> Auswahl {
        Auswahl {
            pfade: (0..anzahl)
                .map(|nummer| PathBuf::from(format!("/Users/k1/Notizen/eintrag-{nummer}")))
                .collect(),
            ordner,
        }
    }

    /// Ein einzelner Eintrag bekommt die Einzahl (C2).
    ///
    /// Das Abnahmekriterium nennt sie eigens, weil die Rueckfrage auch bei
    /// einem Eintrag erscheint und ein „Diese 1 Einträge" den Nutzer zweimal
    /// hinsehen liesse.
    #[test]
    fn ein_eintrag_steht_in_der_einzahl() {
        let (frage, _) = frage_und_erlaeuterung(&auswahl(1, 0), Path::new("/Users/k1/Notizen"));
        assert_eq!(frage, "Diesen Eintrag in den Papierkorb räumen?");
    }

    /// Mehrere Eintraege nennen ihre Zahl, mit der Tausendertrennung der
    /// Oberflaeche (C2).
    ///
    /// Die 1.234 stehen da, weil sie den Punkt zeigen: die Zahl entsteht ueber
    /// [`super::super::operationen::zahl`] und nicht ueber `{}`, damit ein
    /// grosser Vorgang dieselbe Schreibweise traegt wie die laufende
    /// Vorgangsanzeige daneben.
    #[test]
    fn mehrere_eintraege_nennen_ihre_zahl() {
        let (frage, _) = frage_und_erlaeuterung(&auswahl(2, 0), Path::new("/Users/k1/Notizen"));
        assert_eq!(frage, "Diese 2 Einträge in den Papierkorb räumen?");

        let (viele, _) = frage_und_erlaeuterung(&auswahl(1234, 0), Path::new("/Users/k1/Notizen"));
        assert_eq!(viele, "Diese 1.234 Einträge in den Papierkorb räumen?");
    }

    /// Die Erlaeuterung nennt den vollen Pfad, und zwar ungekuerzt (C2).
    ///
    /// **Die zweite Zusicherung ist die eigentliche.** Der Pfad liegt unter dem
    /// Benutzerverzeichnis, also waere er der Fall, in dem
    /// `gekuerzt_fuer_anzeige` eine Tilde setzte. Steht hier je eine, ist die
    /// Festlegung des Modulkopfes gebrochen, und der Nutzer sieht nicht mehr,
    /// welchen Ordner er leert.
    #[test]
    fn die_erlaeuterung_nennt_den_vollen_pfad() {
        let (_, erlaeuterung) = frage_und_erlaeuterung(
            &auswahl(3, 0),
            Path::new("/Users/k1/Projects/productive/krk/fusion-workbench/shared"),
        );
        assert_eq!(
            erlaeuterung,
            "Geräumt wird aus /Users/k1/Projects/productive/krk/fusion-workbench/shared."
        );
        assert!(
            !erlaeuterung.contains('~'),
            "der Pfad steht gekuerzt in der Erlaeuterung: {erlaeuterung}"
        );
    }

    /// Sind Ordner darunter, nennt die Erlaeuterung ihre Zahl gesondert (C2).
    ///
    /// Der Zusatz sagt, was die Zahl der Eintraege verschweigt: hinter einem
    /// der drei Eintraege haengt ein ganzer Baum. Die Zahl der Eintraege bleibt
    /// davon unberuehrt, ein Ordner zaehlt in der Frage eins.
    #[test]
    fn die_ordner_stehen_gesondert_in_der_erlaeuterung() {
        let (frage, erlaeuterung) =
            frage_und_erlaeuterung(&auswahl(3, 2), Path::new("/Users/k1/Notizen"));
        assert_eq!(frage, "Diese 3 Einträge in den Papierkorb räumen?");
        assert_eq!(
            erlaeuterung,
            "Geräumt wird aus /Users/k1/Notizen.\n\nDarunter 2 Ordner, \
             jeweils mit ihrem gesamten Inhalt."
        );
    }

    /// Ohne Ordner in der Auswahl bleibt der Zusatz weg (C2).
    ///
    /// Die Gegenprobe zur vorigen: der zweite Absatz entsteht nur, wenn er
    /// etwas zu sagen hat.
    #[test]
    fn ohne_ordner_bleibt_die_erlaeuterung_einzeilig() {
        let (_, erlaeuterung) =
            frage_und_erlaeuterung(&auswahl(3, 0), Path::new("/Users/k1/Notizen"));
        assert_eq!(erlaeuterung, "Geräumt wird aus /Users/k1/Notizen.");
    }
}
