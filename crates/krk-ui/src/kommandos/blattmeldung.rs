//! Was die Blattsperre sagt, wenn sie einen Tastenbefehl abweist.
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Die drei Eingaben liest der
//! Anwendungsdelegierte — zwei aus der [`Lage`](super::zulaessigkeit::Lage) und
//! dem nachgeschlagenen Kommando, eine aus dem Anschlag des Ereignisabgriffs —,
//! die Regel selbst steht hier und ist ohne Fenster pruefbar.
//!
//! ```text
//!  blatt_steht ─┐
//!  kommando ────┼──> blattmeldung() ──> Some(Satz) | None
//!  druck ───────┘
//! ```
//!
//! # Warum es diese Regel gibt
//!
//! `Anwendungsdelegierter::kommando_ausfuehren` hat bis zum 260904 jede
//! Abweisung mit einem nackten `return false` beantwortet: kein Wort, keine
//! Wirkung. Bei einem Navigationsbefehl faellt das nicht auf, bei `cmd+s`
//! kostet es Arbeit — der Nutzer haelt die Datei fuer gesichert und schliesst
//! sie. Genau das ist am 260904 geschehen
//! (`shared/issues/260904-1827_*_sichern-auf-einem-netzlaufwerk-schlaegt-still-fehl-*`,
//! aufgeklaert in `shared/history/260904-1905-bugfix-sichern-auf-google-drive.md`:
//! der Schreibweg traegt dreimal gemessen kein `errno`, und der beobachtete
//! Ausgang ist mit keinem der drei Ausgaenge von `Editormodell::sichern`
//! vereinbar, sondern allein damit, dass `sichern` gar nicht gelaufen ist).
//! `HYG-NO-SILENT-FAIL` verbietet den stillen Fehlschlag; diese Regel loest ihn
//! fuer die eine Sperre ein, die ihn nachweislich erzeugt hat.
//!
//! # Warum allein die Blattsperre und nicht alle vier Bestandteile
//!
//! [`zulaessigkeit`](super::zulaessigkeit) fragt vier Dinge, und drei davon
//! weisen im Sekundentakt ab, ohne dass etwas verlorenginge:
//!
//! - **Der Fokusvorbehalt** trennt die Bereiche. `up` und `down` liegen in
//!   `resources/default-keymap.toml` ohne Zusatztaste und laufen bei jedem
//!   Pfeildruck im Editor durch ihn hindurch. Eine Meldung dort stuende
//!   dauernd in der Zeile und verdraengte die Meldungen, auf die es ankommt.
//! - **Der Ersthelferbefund** weist ab, waehrend der Nutzer in der Dateiliste
//!   umbenennt. Dass die Tasten dann dem Feldeditor gehoeren, ist der Zweck und
//!   kein Hindernis.
//! - **Das fremde Schluesselfenster** heisst, dass KRK gar nicht vorn steht.
//!   Eine Meldung in eine Zeile zu schreiben, die hinter einem anderen Fenster
//!   liegt, hilft niemandem.
//!
//! Die Blattsperre ist die vierte und die einzige, die den Nutzer ueber einen
//! **ausgefuehrten** Befehl taeuschen kann: das Blatt steht sichtbar da, der
//! Befehl kommt nicht durch, und nichts unterscheidet das fuer ihn von einem
//! Befehl, der gewirkt hat. Der Schnitt ist damit an der Wirkung gezogen und
//! nicht an einer Liste von Kommandos, die mit jeder Runde waechst.
//!
//! # Warum nicht jeder Anschlag meldet
//!
//! Steht ein Blatt, gehoert die Tastatur ihm. Ein Anschlag, den AppKit zum
//! Bedienen des Blattes braucht, ist deshalb kein abgewiesener Befehl, sondern
//! ein angekommener Griff: `tab` rueckt die Tastaturansteuerung weiter, `space`
//! loest die gewaehlte Schaltflaeche aus, `return` die Vorgabeschaltflaeche,
//! `esc` den Abbruch, und der Pfeilblock samt `pageup`, `pagedown`, `home` und
//! `end` bewegt die Auswahl in einer Liste im Blatt — die Vorschau des
//! Stapelumbenennens ist eine. Genau diese Gruppe fuehrt
//! `krk_core::tasten::parser::TASTEN` schon heute unter der Ueberschrift „der
//! Pfeilblock, vollstaendig, und die uebrige Bewegung im Blatt".
//!
//! **Gefragt ist die Stelle auf der Tastatur und nicht die Zusatztaste.**
//! `cmd+up` faellt damit ebenso heraus wie `up`, obwohl es kein Blatt bedient.
//! Der Preis ist genannt und klein: die Befehle des Pfeilblocks bewegen eine
//! Auswahl, und eine nicht bewegte Auswahl kostet keine Arbeit. Eine zweite
//! Bedingung an der Maske waere die erste Ausnahme in einer Regel, die ohne
//! Ausnahmen auskommt.
//!
//! # Der Weg des Hauptmenues bekommt die Antwort nicht ab
//!
//! `druck` ist `None`, wenn kein Tastendruck vorlag; die Regel schweigt dann.
//! Das ist dieselbe Vorkehrung wie in [`rueckschritt`](super::rueckschritt),
//! und sie traegt hier zwei Faelle: den Menueeintrag, der ueber `krkKommando:`
//! durch denselben Ausfuehrungszweig laeuft, und den Melder der Bereichsleiste.
//! **Ein ausgegrauter Menueeintrag ist die Antwort des Menues**, und ein Satz
//! in der Statuszeile daneben waere dieselbe Auskunft ein zweites Mal.
//!
//! Die Regel steht aus demselben Grund **hinter**
//! [`zulaessigkeit::zulaessig`](super::zulaessigkeit::zulaessig) und nicht
//! darin: eine Antwort dort traefe beide Frager zugleich, den Ereignisabgriff
//! und die Ausgrauung ueber `validateMenuItem:`.
//!
//! # Der eine Aufrufer
//!
//! `Anwendungsdelegierter::kommando_ausfuehren` (`crate::appkit::anwendung`)
//! ist der einzige, und die Probe [`die_regel_hat_genau_einen_aufrufer`] haelt
//! die Zahl fest. Ein zweiter waere ein zweiter Weg, dieselbe Meldung zu
//! setzen, und die erste Abweichung zwischen beiden waere ein Fehler ohne
//! Pruefung.
//!
//! [`die_regel_hat_genau_einen_aufrufer`]: tests::die_regel_hat_genau_einen_aufrufer

use krk_core::tasten::{Kommando, Tastendruck, code_von_pflicht};

use super::operationen;
use super::zulaessigkeit;

/// Die Stellen auf der Tastatur, mit denen AppKit ein Blatt bedient.
///
/// **Eine Eigenschaft von AppKit und keine von KRK**, und deshalb waechst diese
/// Liste nicht mit den Kommandos dieses Baums. Jeder Eintrag steht mit seinem
/// Grund da:
///
/// | Taste | Was AppKit im Blatt damit tut |
/// |---|---|
/// | `tab` | rueckt die Tastaturansteuerung eine Stelle weiter |
/// | `space` | loest die angesteuerte Schaltflaeche aus |
/// | `return` | loest die Vorgabeschaltflaeche aus |
/// | `esc` | loest die Abbruchschaltflaeche aus |
/// | `up`, `down`, `left`, `right` | Ansteuerung und Auswahl in einer Liste |
/// | `pageup`, `pagedown`, `home`, `end` | Blaettern in einer Liste im Blatt |
///
/// Die Namen gehen durch [`code_von_pflicht`], damit ein Tippfehler die
/// Uebersetzung anhaelt und keine tote Taste hinterlaesst.
const BEDIENT_EIN_BLATT: [u16; 12] = [
    code_von_pflicht("tab"),
    code_von_pflicht("space"),
    code_von_pflicht("return"),
    code_von_pflicht("esc"),
    code_von_pflicht("up"),
    code_von_pflicht("down"),
    code_von_pflicht("left"),
    code_von_pflicht("right"),
    code_von_pflicht("pageup"),
    code_von_pflicht("pagedown"),
    code_von_pflicht("home"),
    code_von_pflicht("end"),
];

/// Der Satz, mit dem eine abgewiesene Taste in der Statuszeile steht.
///
/// **Er sagt zuerst, dass nichts geschehen ist, und danach warum.** Die
/// Reihenfolge ist die Auskunft: wer nur den Anfang liest, weiss schon, dass
/// seine Datei nicht gesichert ist.
///
/// **Er nennt den Befehl nicht.** Welche Taste gedrueckt wurde, weiss der
/// Nutzer; ein Name dafuer musste aus der Belegung geholt werden und machte aus
/// einer reinen Funktion eine mit Bestand.
///
/// Er steht hier und nicht bei den uebrigen Befehlsantworten in
/// [`operationen`](super::operationen), weil Regel und Satz hier zusammen eine
/// Sache sind — dieselbe Bauart wie bei
/// [`loeschwarnung`](super::loeschwarnung), das seine Stufenfolge und die Texte
/// daraus ebenfalls in einem Modul haelt.
fn satz() -> String {
    "nicht ausgeführt: über dem Fenster steht ein Blatt".to_owned()
}

/// Ob diese Abweisung gemeldet wird, und mit welchem Satz.
///
/// Der Rumpf ist eine Konjunktion aus vier Wahrheitswerten, und sie ist damit
/// ueberschneidungsfrei und vollstaendig ohne Tafel: gemeldet wird genau, was
/// alle vier erfuellt, geschwiegen bei allem uebrigen.
///
/// 1. **Es gab einen Tastendruck** (`druck.is_some()`). Der Menueweg und der
///    Melder der Bereichsleiste geben `None` und bekommen nichts.
/// 2. **Es steht ein Blatt.** Die drei uebrigen Bestandteile der
///    Zulaessigkeitsregel melden nicht; der Modulkopf sagt, warum.
/// 3. **Die Blattsperre haelt diesen Befehl auf.** Gefragt sind
///    [`operationen::waehrend_blatt_erlaubt`] und
///    [`zulaessigkeit::immer_erreichbar`], also die zwei Regeln, die den
///    Durchlass schon heute entscheiden, und keine dritte Fassung daneben. Die
///    vier Kommandos, die waehrend eines Blattes durchkommen, werden gar nicht
///    erst abgewiesen und brauchen deshalb keinen Satz; die Zeile hier haelt
///    das auch dann noch, wenn ein fuenftes hinzukaeme.
/// 4. **Der Anschlag bedient kein Blatt.** Die zwoelf Stellen aus
///    [`BEDIENT_EIN_BLATT`] gehoeren dem Blatt und sind kein abgewiesener
///    Befehl.
///
/// **Der Aufrufer prueft die Zulaessigkeit vorher und nicht diese Funktion.**
/// Sie wird allein im Abweisungszweig gerufen; `blatt_steht` ohne Abweisung
/// gibt es dort nicht.
///
/// `#[must_use]`, weil das stille Fallenlassen des Rueckgabewerts genau den
/// Defekt wiederherstellte, gegen den die Regel gebaut ist: der Satz entstuende
/// und niemand saehe ihn.
#[must_use]
pub fn blattmeldung(
    blatt_steht: bool,
    kommando: Kommando,
    druck: Option<Tastendruck>,
) -> Option<String> {
    let druck = druck?;
    let sperre_haelt_auf = !operationen::waehrend_blatt_erlaubt(kommando)
        && !zulaessigkeit::immer_erreichbar(kommando);
    let bedient_das_blatt = BEDIENT_EIN_BLATT.contains(&druck.code);

    (blatt_steht && sperre_haelt_auf && !bedient_das_blatt).then(satz)
}

#[cfg(test)]
mod tests {
    use krk_core::tasten::normalisierung::ModMaske;

    use crate::quellbaum::{aufrufstellen, quelldateien};

    use super::*;

    /// Ein Anschlag auf die genannte Stelle, ohne Zusatztaste.
    fn nackt(name: &str) -> Option<Tastendruck> {
        Some(Tastendruck::neu(code_von_pflicht(name), ModMaske::LEER))
    }

    /// Ein Anschlag auf die genannte Stelle mit der Befehlstaste.
    fn mit_cmd(name: &str) -> Option<Tastendruck> {
        Some(Tastendruck::neu(code_von_pflicht(name), ModMaske::BEFEHL))
    }

    /// Der gemessene Fall vom 260904: `cmd+s` vor einem stehenden Blatt.
    ///
    /// **Die Probe der ganzen Runde.** Ohne sie stand der Nutzer vor einer
    /// ungesicherten Datei, die er fuer gesichert hielt.
    #[test]
    fn ein_abgewiesenes_sichern_meldet_sich() {
        let meldung = blattmeldung(true, Kommando::EditorSichern, mit_cmd("s"));
        assert_eq!(
            meldung.as_deref(),
            Some("nicht ausgeführt: über dem Fenster steht ein Blatt"),
            "ein abgewiesenes cmd+s bleibt stumm"
        );
    }

    /// Ohne Blatt sagt die Regel nichts, gleich welcher Befehl abgewiesen wurde.
    ///
    /// Die drei uebrigen Bestandteile der Zulaessigkeitsregel weisen im
    /// Sekundentakt ab; der Modulkopf schreibt aus, warum keiner von ihnen
    /// meldet.
    #[test]
    fn ohne_blatt_bleibt_die_regel_stumm() {
        for kommando in [
            Kommando::EditorSichern,
            Kommando::AuswahlHoch,
            Kommando::Kopieren,
        ] {
            assert_eq!(
                blattmeldung(false, kommando, mit_cmd("s")),
                None,
                "{kommando:?} meldet ohne Blatt"
            );
        }
    }

    /// Die zwoelf Stellen, mit denen AppKit ein Blatt bedient, melden nicht.
    ///
    /// Sie stehen hier mit ihren Namen und nicht als Zahl: `up` und `down`
    /// laufen bei jedem Anschlag in einem Blatt mit Liste durch diese Regel,
    /// und ein Satz je Anschlag waere die Zeile voller Rauschen.
    #[test]
    fn was_das_blatt_bedient_meldet_nicht() {
        for name in [
            "tab", "space", "return", "esc", "up", "down", "left", "right", "pageup", "pagedown",
            "home", "end",
        ] {
            assert_eq!(
                blattmeldung(true, Kommando::AuswahlHoch, nackt(name)),
                None,
                "`{name}` meldet, obwohl es das Blatt bedient"
            );
        }
    }

    /// Die Stelle entscheidet und nicht die Zusatztaste.
    ///
    /// `cmd+up` ist `Kommando::OrdnerAufwaerts` und bedient kein Blatt; es
    /// schweigt trotzdem. Der Preis steht im Modulkopf: eine nicht bewegte
    /// Auswahl kostet keine Arbeit, eine zweite Bedingung an der Maske waere
    /// die erste Ausnahme in einer Regel ohne Ausnahmen.
    #[test]
    fn die_zusatztaste_hebt_die_stelle_nicht_auf() {
        assert_eq!(
            blattmeldung(true, Kommando::OrdnerAufwaerts, mit_cmd("up")),
            None,
            "cmd+up meldet, obwohl seine Stelle das Blatt bedient"
        );
    }

    /// Ohne Tastendruck schweigt die Regel: der Menueweg bekommt sie nicht ab.
    ///
    /// Ein ausgegrauter Menueeintrag ist die Antwort des Menues, und ein Satz
    /// in der Statuszeile daneben waere dieselbe Auskunft ein zweites Mal.
    #[test]
    fn ohne_anschlag_bleibt_die_regel_stumm() {
        assert_eq!(
            blattmeldung(true, Kommando::EditorSichern, None),
            None,
            "der Menueweg bekommt eine Meldung"
        );
    }

    /// Die vier Kommandos, die waehrend eines Blattes durchkommen, melden nicht.
    ///
    /// Sie werden gar nicht erst abgewiesen; die Zeile in [`blattmeldung`]
    /// haelt das unabhaengig davon, ob der Aufrufer sie je erreicht. Welche vier
    /// es sind, schreibt
    /// `zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch`
    /// aus, und diese Probe zaehlt sie nicht nach: sie prueft die Wirkung des
    /// Durchlasses und nicht seine Zahl.
    #[test]
    fn was_durch_das_blatt_kommt_meldet_nicht() {
        for kommando in [
            Kommando::Abbrechen,
            Kommando::Beenden,
            Kommando::FensterSchliessen,
            Kommando::FensterEinblenden,
        ] {
            assert_eq!(
                blattmeldung(true, kommando, mit_cmd("q")),
                None,
                "{kommando:?} meldet, obwohl es durch das Blatt kommt"
            );
        }
    }

    /// Ein Befehl mit Zusatztaste, der kein Blatt bedient, meldet.
    ///
    /// Die Gegenprobe zu [`was_das_blatt_bedient_meldet_nicht`]: die Regel
    /// schweigt nicht ueberall.
    #[test]
    fn ein_befehl_mit_zusatztaste_meldet() {
        for (kommando, taste) in [
            (Kommando::TabNeu, "t"),
            (Kommando::EintragspfadKopieren, "c"),
            (Kommando::InPapierkorb, "delete"),
        ] {
            assert!(
                blattmeldung(true, kommando, mit_cmd(taste)).is_some(),
                "{kommando:?} auf cmd+{taste} bleibt stumm"
            );
        }
    }

    /// Genau eine Stelle im Baum ruft die Regel.
    ///
    /// **Eine Aufruferzaehlung in der Form von
    /// `die_regel_hat_genau_einen_aufrufer` in [`super::super::rueckschritt`]**,
    /// und sie steht hier aus demselben Grund: ein zweiter Aufrufer waere ein
    /// zweiter Weg, dieselbe Meldung zu setzen, und die erste Abweichung
    /// zwischen beiden waere ein Fehler ohne Pruefung. Der eine Aufrufer ist
    /// `Anwendungsdelegierter::kommando_ausfuehren` in
    /// `crate::appkit::anwendung`.
    ///
    /// **Diese Datei bleibt aussen vor**, wie bei der Vorlage: die Proben
    /// darueber rufen die Regel vielfach, und das sind keine Aufrufer im Sinne
    /// der Zusage. Die Nadel steht zusammengesetzt da, weil die Probe in dem
    /// Baum liegt, den sie liest.
    #[test]
    fn die_regel_hat_genau_einen_aufrufer() {
        let zuhause = "krk-ui/src/kommandos/blattmeldung.rs";
        let name = concat!("blatt", "meldung");
        let aufrufe: usize = quelldateien()
            .iter()
            .filter(|(datei, _)| datei != zuhause)
            .map(|(_, inhalt)| aufrufstellen(inhalt, name))
            .sum();
        assert_eq!(
            aufrufe, 1,
            "die Regel der Blattmeldung hat nicht genau einen Aufrufer"
        );
    }
}
