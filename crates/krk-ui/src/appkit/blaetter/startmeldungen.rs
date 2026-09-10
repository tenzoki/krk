//! Die Sammlung der Startmeldungen, sobald es mehr als eine ist.
//!
//! `oberflaeche_aufbauen` sammelt jede Meldung des Starts in einem `Vec` und
//! stellte sie bis zum 260910 in einer Schleife in die eine Zeile des aktiven
//! Dateifensters. Die Zeile haelt genau **eine** Fenstermeldung
//! ([`crate::appkit::statuszeile::Rang::Fenstermeldung`]); von n Meldungen sah
//! der Nutzer die n-te. Gemeldet war das als
//! `shared/issues/260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`.
//!
//! **Der Kanal wechselt, nicht die Auswahl.** n Saetze in ein Fach zu legen
//! verliert n-1 davon, gleich in welcher Reihenfolge; was sich aendern muss,
//! ist der Weg. Der Nutzer hat am 260910 entschieden: genau eine Meldung geht
//! unveraendert in die Statuszeile, ab der zweiten faehrt ein Blatt herunter,
//! das alle auffuehrt
//! (`260910-0818_*_wie-erreichen-n-startmeldungen-den-nutzer-wenn-die-eine-zeile-nur-eine-traegt.md`,
//! Moeglichkeit 2). Die beiden verworfenen Wege stehen dort mit ihren
//! Gruenden: die Saetze zu verbinden tauscht das Ueberschreiben gegen das
//! Abschneiden, und eine Warteschlange mit Verweildauer verliert jede Meldung,
//! die vor dem ersten Blick des Nutzers abgelaufen ist.
//!
//! **Der Fall „genau eine Meldung" bleibt Wort fuer Wort, was er war.** Das
//! fordert der Defekt ausdruecklich, und [`auskunft`] loest es ein: sie reicht
//! den Text unveraendert heraus, und der Aufrufer ruft damit dasselbe
//! `meldung_zeigen` wie zuvor.
//!
//! **Es ist eine begrenzte Abweichung von der Antwort vom 260804-0830**
//! (`decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md`,
//! Moeglichkeit 1: die laufenden Fehler traegt die Statuszeile). Sie greift
//! allein den Start und allein ab der zweiten Meldung; die Antwort von damals
//! hat fuer **einen** laufenden Fehler entschieden, und die Lage mit mehreren
//! zugleich stand dort nicht zur Wahl. Ein dritter Anzeigeweg entsteht nicht:
//! das Blatt ist dasselbe Bauteil wie die Abschlussliste der uebersprungenen
//! Eintraege ([`super::uebersprungen`]).
//!
//! **Ein Start mit mehr als einer Meldung ist selten**, und darauf beruht die
//! Wahl: er heisst, dass an der Ablage mehreres zugleich nicht stimmt. Genau
//! dann ist die Sperre, die ein stehendes Blatt ueber die Tastenbefehle legt,
//! angemessen — der Nutzer bestaetigt, bevor er arbeitet. Bei einer Meldung
//! tritt sie nicht ein.
//!
//! **Die Entscheidung steht als reine Funktion da und nicht im Rumpf des
//! Aufrufers**, damit alle vier Faelle ohne AppKit und ohne Hauptfaden pruefbar
//! sind: keine Meldung, eine, zwei, drei.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! Eine einzige AppKit-Klasse, `NSWindow`, und die Datei reicht sie nur weiter;
//! sie steht seit macOS 10.0 zur Verfuegung. `MainThreadMarker` gehoert `objc2`
//! und nicht AppKit. Das Buendel zielt auf 15.0 (`.cargo/config.toml`), und
//! nichts hier ist nach macOS 15 hinzugekommen; `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und die Nennung ist die Gegenmassnahme.
//! Alles, was `NSAlert` betrifft, steht im Kopf von [`Blatt`].

use objc2_app_kit::NSWindow;
use objc2_foundation::MainThreadMarker;

use super::{Blatt, Blattgriff, Schaltflaeche, Taste, Wirkung};

/// Was aus den gesammelten Startmeldungen wird.
///
/// Eine **vollstaendige Fallunterscheidung ohne Auffangzweig** beim Aufrufer:
/// ein vierter Weg, eine Meldung zu zeigen, haelt damit den Bau an, statt still
/// zu entstehen.
#[derive(Debug, PartialEq, Eq)]
pub enum Auskunft<'a> {
    /// Keine Meldung: kein Blatt, keine Zeile.
    Nichts,
    /// Genau eine Meldung: unveraendert in die Statuszeile, wie bisher.
    Zeile(&'a str),
    /// Mehr als eine: ein Blatt mit Kopfzeile und Liste.
    Blatt {
        /// Die Kopfzeile des Blattes.
        frage: String,
        /// Die Meldungen, eine je Zeile.
        liste: String,
    },
}

/// Welchen Weg die gesammelten Startmeldungen nehmen.
///
/// Rein und ohne AppKit, damit die vier Faelle messbar sind. Die Grenze liegt
/// bei zwei und nicht bei einer beliebigen Zahl: eine Meldung passt in die
/// Zeile, zwei passen nicht.
#[must_use]
pub fn auskunft(meldungen: &[String]) -> Auskunft<'_> {
    match meldungen {
        [] => Auskunft::Nichts,
        [einzige] => Auskunft::Zeile(einzige),
        mehrere => Auskunft::Blatt {
            frage: format!("Beim Start gab es {} Meldungen", mehrere.len()),
            liste: mehrere.join("\n"),
        },
    }
}

/// Die eine Schaltflaeche der Startmeldungen.
///
/// **Als reine Funktion herausgezogen**, aus demselben Grund wie bei
/// [`super::uebersprungen`]: an einem gebauten `NSAlert` ist nicht mehr
/// abzulesen, welche seiner Schaltflaechen alles liegen laesst, an dieser Liste
/// schon.
///
/// Sie laesst liegen, obwohl sie die einzige ist: das Blatt fragt nach nichts,
/// sondern meldet, und das Schliessen ist derselbe Ausgang, den die
/// Escape-Taste naehme.
#[must_use]
fn schaltflaechen() -> [Schaltflaeche<'static>; 1] {
    [Schaltflaeche::neu(
        "Schließen",
        Taste::Eingabe,
        Wirkung::Liegenlassen,
    )]
}

/// Zeigt die Startmeldungen am Fenster.
///
/// `frage` und `liste` kommen aus [`auskunft`].
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    liste: &str,
    fertig: impl Fn() + 'static,
) -> Blattgriff {
    let blatt = Blatt::mit_schaltflaechen(mtm, frage, &schaltflaechen());
    blatt.erlaeuterung_setzen(liste);
    blatt.zeigen_mit_wahl(fenster, move |_stelle, _fuer_alle| fertig())
}

#[cfg(test)]
mod tests {
    use crate::appkit::blaetter::abbruchstelle;

    use super::{Auskunft, Taste, Wirkung, auskunft, schaltflaechen};

    /// Ohne Startmeldung geschieht nichts.
    ///
    /// Kein Blatt und keine Zeile: ein Blatt, das nach einem sauberen Start
    /// „nichts zu melden" meldete, waere ein Tastendruck ohne Auskunft —
    /// dieselbe Ueberlegung wie bei der Abschlussliste der uebersprungenen
    /// Eintraege.
    #[test]
    fn ohne_meldung_geschieht_nichts() {
        assert_eq!(auskunft(&[]), Auskunft::Nichts);
    }

    /// Bei genau einer Meldung bleibt es Wort fuer Wort beim heutigen Weg.
    ///
    /// Das fordert der Defekt
    /// `260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`
    /// ausdruecklich: der Text geht **unveraendert** in die Statuszeile, ohne
    /// Kopfzeile, ohne Zusatz und ohne Blatt.
    #[test]
    fn eine_meldung_geht_unveraendert_in_die_zeile() {
        let meldungen = ["die Lesezeichen ließen sich nicht laden".to_owned()];
        assert_eq!(
            auskunft(&meldungen),
            Auskunft::Zeile("die Lesezeichen ließen sich nicht laden")
        );
    }

    /// Bei zwei Meldungen erreicht jede den Nutzer.
    #[test]
    fn zwei_meldungen_stehen_beide_im_blatt() {
        let meldungen = ["erste Meldung".to_owned(), "zweite Meldung".to_owned()];
        let Auskunft::Blatt { frage, liste } = auskunft(&meldungen) else {
            panic!("zwei Startmeldungen fahren kein Blatt herunter");
        };
        assert_eq!(frage, "Beim Start gab es 2 Meldungen");
        for meldung in &meldungen {
            assert!(
                liste.lines().any(|zeile| zeile == meldung),
                "die Meldung {meldung:?} steht nicht im Blatt: {liste:?}"
            );
        }
    }

    /// Bei drei Meldungen erreicht jede den Nutzer.
    ///
    /// Die zweite Zahl steht daneben, weil die Grenze bei zwei liegt und eine
    /// Fallunterscheidung, die allein bei zwei richtig rechnete, hier nicht
    /// auffiele.
    #[test]
    fn drei_meldungen_stehen_alle_drei_im_blatt() {
        let meldungen = [
            "die session.toml ist beschädigt".to_owned(),
            "die settings.toml ist beschädigt".to_owned(),
            "ohne Sitzungsrecht".to_owned(),
        ];
        let Auskunft::Blatt { frage, liste } = auskunft(&meldungen) else {
            panic!("drei Startmeldungen fahren kein Blatt herunter");
        };
        assert_eq!(frage, "Beim Start gab es 3 Meldungen");
        assert_eq!(
            liste.lines().count(),
            3,
            "das Blatt fuehrt nicht genau drei Zeilen: {liste:?}"
        );
        for meldung in &meldungen {
            assert!(
                liste.lines().any(|zeile| zeile == meldung),
                "die Meldung {meldung:?} steht nicht im Blatt: {liste:?}"
            );
        }
    }

    /// Der Bauplan traegt genau eine Schaltflaeche, und sie laesst liegen.
    ///
    /// Dieselbe Probe wie bei [`super::super::uebersprungen`], aus demselben
    /// Grund: eine zweite Schaltflaeche waere eine Antwort auf eine Frage, die
    /// dieses Blatt nicht stellt.
    #[test]
    fn der_bauplan_traegt_die_eine_schliessende_schaltflaeche() {
        let schaltflaechen = schaltflaechen();
        assert_eq!(
            schaltflaechen.len(),
            1,
            "das Blatt der Startmeldungen bietet mehr als das Schließen an"
        );
        assert_eq!(schaltflaechen[0].titel, "Schließen");
        assert_eq!(schaltflaechen[0].taste, Taste::Eingabe);
        assert_eq!(
            schaltflaechen[0].wirkung,
            Wirkung::Liegenlassen,
            "das Schließen der Startmeldungen fuehrt etwas aus"
        );
        assert_eq!(
            abbruchstelle(&schaltflaechen),
            0,
            "der ungefaehrliche Ausgang liegt nicht auf der einen Schaltflaeche"
        );
    }
}
