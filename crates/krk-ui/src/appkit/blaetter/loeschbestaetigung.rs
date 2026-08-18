//! Die eine Rueckfrage vor dem Raeumen in den Papierkorb (C2, C3).
//!
//! Genau einmal je Vorgang, unabhaengig von der Zahl der betroffenen Eintraege
//! und unabhaengig davon, welche der drei Tasten oder ob der Menueeintrag ihn
//! ausgeloest hat. Die
//! beiden Texte rechnet
//! [`crate::kommandos::loeschwarnung::frage_und_erlaeuterung`]; hier steht
//! allein, was AppKit betrifft.
//!
//! # Ruhig und laut sind dasselbe Blatt
//!
//! Ein zweites Blatt entsteht nicht: zwei Blaetter waeren zwei Wahrheiten ueber
//! dieselbe Frage. Die laute Form unterscheidet sich von der ruhigen in genau
//! drei Dingen, und nur eines davon steht in dieser Datei. Der **erste**
//! Warngrund in der Frage und die **uebrigen** Gruende als eigener Absatz der
//! Erlaeuterung kommen als fertiger Text herein; das Warnzeichen setzt
//! [`Blatt::als_warnung`], und zwar nur bei `laut`. Die Erlaeuterung gewinnt
//! damit **keinen Satz ueber die Folgen**, sondern die Gruende, die in der
//! Frage keinen Platz hatten; gebaut wird der Absatz in
//! [`crate::kommandos::loeschwarnung::frage_und_erlaeuterung`], und gemessen
//! von dessen Probe `die_frage_nennt_den_ersten_grund_und_die_erlaeuterung_die_uebrigen`.
//! Die Schaltflaechen, ihre Reihenfolge und ihre Tasten sind in beiden Formen
//! dieselben (C3).
//!
//! ```text
//!  frage, erlaeuterung ─┬──> ruhig: Frage, Erlaeuterung, zwei Schaltflaechen
//!  laut ────────────────┘    laut:  dasselbe, dazu das Warnzeichen
//! ```
//!
//! **Die Beschriftung der zweiten Schaltflaeche kommt als Argument herein**,
//! weil der Wortlaut des Vorgangs [`crate::kommandos::loeschwarnung`] gehoert
//! und nicht dieser Datei. "Abbrechen" bleibt als einziger Wortlaut hier
//! stehen, denn es benennt keinen Vorgang, sondern die Sicherheitseigenschaft
//! des Blattes, und es haengt untrennbar an der Tastenzuordnung darunter.
//!
//! # Vorbelegt ist Abbrechen
//!
//! C4 verlangt es woertlich: "Vorbelegt ist Abbrechen, sodass ein reflexhaftes
//! Bestaetigen mit der Return-Taste nichts loescht." Das ist der Grund, aus dem
//! die Huelle die Taste je Schaltflaeche entgegennimmt: `NSAlert` gaebe die
//! Eingabetaste sonst der ersten, und die erste soll hier "Abbrechen" sein,
//! damit sie zugleich die hervorgehobene ist.
//!
//! Dieselbe Forderung steht als Abnahmekriterium in C2 dieser Runde, und sie
//! gilt dort fuer **beide** Formen: die ruhige Rueckfrage vor dem alltaeglichen
//! Raeumen ist genauso vorbelegt wie die laute.
//!
//! **Der Hinweissatz darunter nennt keinen Vorgang.** Er lautet "Return und
//! Esc brechen ab. Zum Bestätigen Cmd+Return." und sagte bis zum 260817 "Zum
//! Löschen Cmd+Return". Das ging, solange das Blatt allein das endgueltige
//! Loeschen bediente; seit der alltaegliche Weg in den Papierkorb durch
//! dasselbe Blatt geht, waere "Löschen" fuer ihn schlicht das falsche Wort. Es
//! ist dasselbe Wort, das [`crate::kommandos::loeschwarnung`] in der Frage
//! ausdruecklich vermeidet, weil der Rueckweg ueber den Papierkorb der
//! Unterschied zu dem Weg ist, den diese Runde abgeschafft hat; ein Satz, der
//! es zwei Zeilen darunter doch benutzt, nimmt der Unterscheidung ihre Wirkung.
//! Den Vorgang benennt die zweite Schaltflaeche, und sie tut es in beiden
//! Formen richtig. Der Satz benennt deshalb allein die Taste, und er ist mit
//! Buendel D richtig geblieben, wo nur noch der eine Loeschweg uebrig ist.
//!
//! **Auch eine Antwort, die zu keiner der beiden Schaltflaechen gehoert, raeumt
//! nichts.** Dieses Blatt ist das eine im Baum, dessen **letzte** Schaltflaeche
//! die ausfuehrende ist; solange die Huelle eine unbekannte Antwort auf die
//! letzte Stelle abbildete, ergab sie hier den Loeschauftrag
//! (`issues/260817-1106_*`). Die Huelle fragt seit dem 260817 die
//! [`Wirkung`](super::Wirkung) der Schaltflaechen und nicht ihre Reihenfolge;
//! [`schaltflaechen`] traegt sie, und die Probe darunter liest nach, dass die
//! ungefaehrliche Stelle nicht die ausfuehrende ist.
//!
//! Der zweite Weg zum Abbruch, die Escape-Taste, laeuft nicht ueber eine
//! Tastenentsprechung dieses Blattes, sondern ueber den Befehl `abbrechen` aus
//! `resources/default-keymap.toml`: der Ereignisabgriff sieht die Taste vor dem
//! Blatt, und der Anwendungsdelegierte schliesst das offene Blatt. Eine zweite
//! Tastenentsprechung waere hier auch gar nicht moeglich, weil ein `NSButton`
//! genau eine traegt.
//!
//! Der Weg dahin ist bindend: das Raeumen faengt erst an, wenn diese Frage mit
//! Ja beantwortet ist. Der Kern bekommt seinen Auftrag danach, siehe
//! `shared/decisions/260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`.
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

/// Die Stelle der Schaltflaeche, die den Vorgang ausloest.
///
/// Sie steht als Konstante da, weil zwei Stellen sie brauchen: der Rueckruf,
/// der aus der gedrueckten Stelle ein `bool` macht, und die Probe darunter, die
/// nachliest, dass eine unbekannte Antwort **nicht** hier landet.
const AUSFUEHRENDE_STELLE: usize = 1;

/// Die beiden Schaltflaechen der Rueckfrage, in bindender Reihenfolge.
///
/// **Als reine Funktion herausgezogen**, damit die Reihenfolge ohne AppKit und
/// ohne Hauptfaden pruefbar ist: [`Schaltflaeche`] traegt nur eine
/// Beschriftung, eine Taste und eine [`Wirkung`]. Der Bauplan des Blattes ist
/// die eine Angabe, an der die Zusage "eine unbekannte Antwort loescht nichts"
/// haengt (`issues/260817-1106_*`).
fn schaltflaechen(vorgang: &str) -> [Schaltflaeche<'_>; 2] {
    [
        Schaltflaeche::neu("Abbrechen", Taste::Eingabe, Wirkung::Liegenlassen),
        Schaltflaeche::neu(vorgang, Taste::EingabeMitBefehl, Wirkung::Ausfuehren),
    ]
}

/// Zeigt die Rueckfrage und meldet, ob der Nutzer den Vorgang bestaetigt hat.
///
/// `schaltflaeche` ist die Beschriftung der **zweiten** Schaltflaeche, also
/// derjenigen, die den Vorgang ausloest; die erste bleibt "Abbrechen" und
/// traegt die Eingabetaste. Angelegt wird beides in [`schaltflaechen`]. `laut` setzt das Warnzeichen des Systems und sonst
/// nichts: Text, Reihenfolge und Tasten sind in beiden Formen dieselben.
///
/// `fertig` laeuft auf dem Hauptfaden und genau einmal. `false` heisst
/// abgebrochen, und dann geschieht nichts.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    erlaeuterung: &str,
    schaltflaeche: &str,
    laut: bool,
    fertig: impl Fn(bool) + 'static,
) -> Blattgriff {
    let blatt = Blatt::mit_schaltflaechen(mtm, frage, &schaltflaechen(schaltflaeche));
    blatt.erlaeuterung_setzen(&format!(
        "{erlaeuterung}\n\nReturn und Esc brechen ab. Zum Bestätigen Cmd+Return."
    ));
    if laut {
        blatt.als_warnung();
    }
    blatt.zeigen_mit_wahl(fenster, move |stelle, _fuer_alle| {
        fertig(stelle == AUSFUEHRENDE_STELLE)
    })
}

#[cfg(test)]
mod tests {
    use crate::appkit::blaetter::abbruchstelle;

    use super::{AUSFUEHRENDE_STELLE, Wirkung, schaltflaechen};

    /// Eine unbekannte Antwort von `NSAlert` raeumt nichts.
    ///
    /// Die Zusage, um derentwillen die Rueckfallstelle aus der [`Wirkung`]
    /// kommt und nicht aus der Reihenfolge: dieses Blatt ist das eine im Baum,
    /// dessen **letzte** Schaltflaeche die ausfuehrende ist. Solange die Regel
    /// die letzte nahm, ergab eine unbekannte Antwort hier
    /// `bestaetigt == true` und damit den Loeschauftrag
    /// (`issues/260817-1106_*`).
    #[test]
    fn eine_unbekannte_antwort_stellt_keinen_auftrag() {
        let schaltflaechen = schaltflaechen("In den Papierkorb");
        let stelle = abbruchstelle(&schaltflaechen);
        assert_ne!(
            stelle, AUSFUEHRENDE_STELLE,
            "eine unbekannte Antwort faellt auf die ausfuehrende Schaltflaeche"
        );
        assert_eq!(stelle, 0, "die abbrechende Schaltflaeche steht vorn");
    }

    /// Die ausfuehrende Stelle und der Bauplan sagen dasselbe.
    ///
    /// Ohne diese Probe koennte die Reihenfolge in [`schaltflaechen`] sich
    /// drehen, ohne dass [`AUSFUEHRENDE_STELLE`] mitgeht; der Rueckruf machte
    /// dann aus "Abbrechen" ein `true`.
    #[test]
    fn die_ausfuehrende_stelle_zeigt_auf_die_ausfuehrende_schaltflaeche() {
        let schaltflaechen = schaltflaechen("Endgültig löschen");
        assert_eq!(
            schaltflaechen[AUSFUEHRENDE_STELLE].wirkung,
            Wirkung::Ausfuehren
        );
        assert_eq!(
            schaltflaechen[AUSFUEHRENDE_STELLE].titel,
            "Endgültig löschen"
        );
    }
}
