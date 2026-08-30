//! Was `cmd+e` bedeutet: hin in den Editor oder zurueck in die Dateiliste.
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Wo der Fokus steht, liest
//! `Anwendungsdelegierter::fokus`; die Regel, was daraus folgt, steht hier und
//! ist ohne Fenster pruefbar.
//!
//! ```text
//!  Fokus ──> rundweg() ──> Some(Rundweg) ──> einer von drei Ruempfen
//!                     └──> None (Leiste, Blatt): der Befehl kommt nicht her
//! ```
//!
//! # Warum diese Regel eigens dasteht
//!
//! Sie ist die zweite Fallunterscheidung dieses Baums, die ein Befehl nach
//! seinem Fokus trifft; die erste ist [`super::rueckschritt`]. Dort geht es um
//! einen Loeschbefehl, hier um nichts Zerstoerendes, und der Preis ist deshalb
//! geringer. Die Form ist trotzdem dieselbe: eine reine Funktion mit einer
//! ausgeschriebenen Tafel und genau einem Rufer, und nicht eine Bedingung im
//! Ausfuehrungszweig, wo keine Probe sie faende. Der Nutzer hat sie am
//! 260823-0942 so verlangt
//! (`shared/decisions/260820-1034_*_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md`).
//!
//! # Eine Groesse, fuenf Werte
//!
//! Die Regel haengt an nichts als am [`Fokus`]. Drei seiner fuenf Werte tragen
//! einen Ausgang, zwei tragen keinen:
//!
//! - **[`Fokus::Dateifenster`]** — der ausgewaehlte Eintrag der Liste geht in
//!   den Editor. Es ist derselbe Rumpf wie bei `f4`, und das ist die Zusage:
//!   ein zweiter Weg auf dieselbe Handlung entsteht nicht.
//! - **[`Fokus::Vorschau`]** — die angezeigte Datei geht in den Editor. Die
//!   Richtung, die `cmd+e` seit dem 260807-2139 traegt, unveraendert.
//! - **[`Fokus::Editor`]** — der Editor wird geschlossen. **Geschlossen und
//!   nicht ausgeblendet**: der Rueckweg gibt die Datei frei und loest die
//!   Nachfrage aus C4 aus. Der Nutzer hat die Wahl mit diesem Preis vorgelegt
//!   bekommen und so getroffen; wer sie umdreht, dreht eine bewusste Wahl um
//!   und kein Versehen.
//! - **[`Fokus::Leiste`], [`Fokus::Git`] und [`Fokus::Anderswo`]** — kein
//!   Ausgang. In der
//!   Lesezeichen- und Geraeteleiste gibt es keine Datei, die der Befehl meinte,
//!   und `Anderswo` heisst ein stehendes Blatt oder ein Textfeld.
//!
//! # Woran die Regel nicht haengt
//!
//! Drei Groessen liegen nahe und stehen bewusst **nicht** in der Signatur:
//!
//! - **Ob der Editor sichtbar ist.** Der Rueckweg wird mit dem Fokus im Editor
//!   gedrueckt, und dann steht er auf dem Schirm; der Hinweg blendet ihn ein.
//! - **Ob eine Datei ausgewaehlt ist oder die Vorschau eine zeigt.** Beide
//!   Hinwege beantworten das selbst und melden es in der Statuszeile. Der
//!   Wirkungsbereich entscheidet, ob eine Taste durchkommt, und nicht, ob sie
//!   etwas findet — derselbe Satz wie beim Ordnersprung in
//!   `Kommando::wirkungsbereich`.
//! - **Ob der Editor einen ungesicherten Stand haelt.** Danach fragt
//!   `Anwendungsdelegierter::anlass_beginnen`, hinter dieser Regel und nicht
//!   in ihr.
//!
//! # Die beiden ausgangslosen Werte sind heute unerreichbar, und das bleibt
//! nicht zugesagt
//!
//! `Kommando::EditorRundweg` traegt
//! [`Wirkungsbereich::Dateibereiche`](krk_core::tasten::Wirkungsbereich), und
//! [`super::fokus::wirkt`] weist die Leiste und das Blatt schon vor dieser Regel
//! ab. [`rundweg`] antwortet trotzdem fuer alle fuenf Werte, aus demselben Grund,
//! aus dem [`super::fokus::rahmenrolle`] auch fuer `Anderswo` antwortet: erst
//! damit kann die Tafel darunter die Frage ueberhaupt stellen. Wer den
//! Wirkungsbereich spaeter weitet, findet hier eine Antwort vor statt einer
//! Luecke.
//!
//! # Der eine Aufrufer
//!
//! `Anwendungsdelegierter::editor_rundweg` (`crate::appkit::anwendung`) ist der
//! einzige, und die Probe `die_regel_hat_genau_einen_aufrufer` haelt die Zahl
//! fest. Ein zweiter waere ein zweiter Weg in den Editor, und genau den
//! vermeidet dieser Befehl.

use super::fokus::Fokus;

/// Welchen Weg `cmd+e` von hier aus nimmt.
///
/// Die drei Werte stehen als Aufzaehlung und nicht als zwei Wahrheitswerte da:
/// die beiden Hinwege sind nicht dieselbe Handlung mit einer anderen Quelle,
/// sondern zwei Ruempfe beim Anwendungsdelegierten, und der Rueckweg ist ein
/// dritter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rundweg {
    /// Hinweg aus der Dateiliste: den ausgewaehlten Eintrag im Editor oeffnen
    /// und den Fokus hineinlegen, wie `f4` es tut.
    AusDerDateiliste,
    /// Hinweg aus der Vorschau: die dort angezeigte Datei im Editor oeffnen.
    AusDerVorschau,
    /// Rueckweg: den Editor schliessen. Die Vorschau kommt zurueck, der Fokus
    /// geht in die Dateiliste.
    ZurueckInDieDateiliste,
}

/// Die eine Regel des Rundwegs.
///
/// Der Rumpf ist diese Tafel, und sie steht ausgeschrieben und nicht gerechnet:
///
/// | [`Fokus`] | Ausgang |
/// |---|---|
/// | [`Fokus::Dateifenster`] | [`Rundweg::AusDerDateiliste`] |
/// | [`Fokus::Vorschau`] | [`Rundweg::AusDerVorschau`] |
/// | [`Fokus::Editor`] | [`Rundweg::ZurueckInDieDateiliste`] |
/// | [`Fokus::Leiste`] | `None` |
/// | [`Fokus::Git`] | `None` |
/// | [`Fokus::Anderswo`] | `None` |
///
/// **Die Zeilen decken jeden Fokuswert ab**, die
/// Fallunterscheidung ist ueberschneidungsfrei und vollstaendig, und einen
/// Auffangzweig gibt es nicht; ein siebter Fokuswert haelt den Bau an. Die
/// Probe `die_tafel_aus_sechs_faellen_geht_auf` schreibt alle aus, aus
/// demselben Grund, aus dem die Tafeln in [`super::zulaessigkeit`] und
/// [`super::rueckschritt`] ausgeschrieben dastehen: eine gerechnete Erwartung
/// waere die Umsetzung ein zweites Mal.
///
/// `None` heisst "von hier aus fuehrt kein Rundweg" und nicht "hier ist nichts
/// zu tun". Der Aufrufer antwortet darauf mit `false`, und das heisst allein,
/// dass kein Nachzug der Aufteilung und keine vorgemerkte Sitzung anfaellt: den
/// Tastendruck verbraucht `Anwendungsdelegierter::kommando_ausfuehren` in jedem
/// Fall, weil es seit der Runde 7 immer `true` liefert.
///
/// `#[must_use]`, weil das stille Fallenlassen des Rueckgabewerts unbemerkt
/// bliebe: `cmd+e` taete dann gar nichts, und keine Meldung sagte warum.
#[must_use]
pub fn rundweg(fokus: Fokus) -> Option<Rundweg> {
    match fokus {
        // Derselbe Rumpf wie `f4`. Der Befehl ist damit in der Dateiliste kein
        // zweiter Weg auf dieselbe Handlung, sondern dieselbe Handlung unter
        // einer zweiten Taste.
        Fokus::Dateifenster => Some(Rundweg::AusDerDateiliste),
        // Die Richtung, die `cmd+e` seit dem 260807-2139 traegt.
        Fokus::Vorschau => Some(Rundweg::AusDerVorschau),
        // Der Rueckweg. Er schliesst und blendet nicht aus; die Begruendung
        // steht im Modulkopf.
        Fokus::Editor => Some(Rundweg::ZurueckInDieDateiliste),
        // In der Leiste und im Git-Bereich gibt es keine Datei, die der Befehl
        // meinte — ein Commit ist keine —, und `Anderswo` ist ein stehendes
        // Blatt oder ein Textfeld. Alle drei erreicht der Befehl heute gar
        // nicht erst; warum sie hier trotzdem eine Antwort bekommen, steht im
        // Modulkopf.
        Fokus::Leiste | Fokus::Git | Fokus::Anderswo => None,
    }
}

#[cfg(test)]
mod tests {
    use krk_core::tasten::{Kommando, Wirkungsbereich};

    use crate::kommandos::fokus::wirkt;
    use crate::quellbaum::{aufrufstellen, quelldateien};

    use super::*;

    /// Die Aufzaehlung der Pruefungen ist die des Programms.
    ///
    /// Dieselbe Begruendung wie in [`super::super::fokus`] und
    /// [`super::super::zulaessigkeit`]: eine zweite Liste derselben fuenf Werte
    /// pruefte womoeglich eine andere Menge als die, ueber die das Programm
    /// laeuft.
    const JEDER_FOKUS: [Fokus; 6] = Fokus::ALLE;

    /// Genau eine Stelle im Baum ruft die Regel.
    ///
    /// **Eine Aufruferzaehlung in der Form von
    /// `die_regel_hat_genau_einen_aufrufer` in [`super::super::rueckschritt`]**,
    /// und sie steht hier aus demselben Grund: die Zusage handelt davon, dass es
    /// diese Fallunterscheidung einmal gibt. Ein zweiter Aufrufer waere ein
    /// zweiter Weg in den Editor, und genau den vermeidet dieser Befehl.
    ///
    /// Der eine Aufrufer ist `Anwendungsdelegierter::editor_rundweg` in
    /// `crate::appkit::anwendung`. Rot wird die Probe, wenn ein zweiter
    /// hinzukommt; die richtige Antwort darauf ist die Frage, warum es ihn gibt,
    /// und nicht die Zahl hier.
    ///
    /// **Diese Datei bleibt aussen vor**, wie bei der Vorlage: die Tafel der
    /// Proben darunter ruft die Regel mehrfach, und das sind keine Aufrufer im
    /// Sinne der Zusage.
    ///
    /// Die Nadel steht zusammengesetzt da, weil die Probe in dem Baum liegt, den
    /// sie liest.
    #[test]
    fn die_regel_hat_genau_einen_aufrufer() {
        let zuhause = "krk-ui/src/kommandos/rundweg.rs";
        let name = concat!("rund", "weg");
        let aufrufe: usize = quelldateien()
            .iter()
            .filter(|(datei, _)| datei != zuhause)
            .map(|(_, inhalt)| aufrufstellen(inhalt, name))
            .sum();
        assert_eq!(
            aufrufe, 1,
            "die Regel des Rundwegs hat nicht genau einen Aufrufer"
        );
    }

    /// Die ganze Regel auf einen Blick: sechs Fokuswerte, sechs Ausgaenge.
    ///
    /// Die Tafel steht in der Form der Tafeln aus [`super::super::fokus`] und
    /// [`super::super::rueckschritt`]. Sie zeigt, dass keine Zeile fehlt und
    /// keine zweimal beantwortet wird; die Proben darunter zeigen einzelne
    /// Zeilen mit ihrer Begruendung.
    #[test]
    fn die_tafel_aus_sechs_faellen_geht_auf() {
        const TAFEL: [(Fokus, Option<Rundweg>); 6] = [
            (Fokus::Dateifenster, Some(Rundweg::AusDerDateiliste)),
            (Fokus::Leiste, None),
            (Fokus::Vorschau, Some(Rundweg::AusDerVorschau)),
            (Fokus::Editor, Some(Rundweg::ZurueckInDieDateiliste)),
            (Fokus::Git, None),
            (Fokus::Anderswo, None),
        ];

        assert_eq!(
            TAFEL.len(),
            Fokus::ALLE.len(),
            "die Tafel nennt nicht jeden Fokuswert"
        );
        for (fokus, ausgang) in TAFEL {
            assert_eq!(rundweg(fokus), ausgang, "fokus={fokus:?}");
        }
    }

    /// Der Hinweg aus der Dateiliste, die Zeile, die der 260823-0942
    /// hinzugefuegt hat.
    ///
    /// Sie ist der Grund der ganzen Aenderung: der Fokus bleibt nach `f3` in
    /// der Dateiliste, und ein Umschalter, der die Vorschau als Ausgangspunkt
    /// naehme, verfehlte den haeufigen Fall.
    #[test]
    fn aus_der_dateiliste_fuehrt_der_weg_in_den_editor() {
        assert_eq!(
            rundweg(Fokus::Dateifenster),
            Some(Rundweg::AusDerDateiliste),
            "cmd+e in der Dateiliste oeffnet nicht im Editor"
        );
    }

    /// Der Hinweg aus der Vorschau, die Zeile, die seit dem 260807-2139 steht.
    ///
    /// Der Nutzer hat sie nicht entschieden, sondern beibehalten; wer sie
    /// streichen will, fragt ihn.
    #[test]
    fn aus_der_vorschau_bleibt_der_weg_unveraendert() {
        assert_eq!(
            rundweg(Fokus::Vorschau),
            Some(Rundweg::AusDerVorschau),
            "cmd+e in der Vorschau tut nicht mehr, was es seit dem 260807 tut"
        );
    }

    /// Der Rueckweg, und er ist ein eigener Ausgang und nicht der Hinweg
    /// rueckwaerts.
    ///
    /// **Die Probe haelt fest, dass der Rueckweg nicht mit einem der beiden
    /// Hinwege zusammenfaellt.** Faellt er es doch, oeffnete `cmd+e` im Editor
    /// eine Datei, statt ihn zu schliessen, und der Rundweg haette kein Ende.
    #[test]
    fn aus_dem_editor_fuehrt_der_weg_zurueck_und_nicht_noch_einmal_hinein() {
        let ausgang = rundweg(Fokus::Editor);
        assert_eq!(
            ausgang,
            Some(Rundweg::ZurueckInDieDateiliste),
            "cmd+e im Editor schliesst ihn nicht"
        );
        assert_ne!(ausgang, Some(Rundweg::AusDerDateiliste));
        assert_ne!(ausgang, Some(Rundweg::AusDerVorschau));
    }

    /// Die drei Werte ohne Ausgang.
    ///
    /// In der Leiste und im Git-Bereich gibt es keine Datei, die der Befehl
    /// meinte; `Anderswo` heisst ein stehendes Blatt oder ein Textfeld, und
    /// dort wirkt kein Befehl, der einen Bereich braucht.
    #[test]
    fn die_leiste_der_git_bereich_und_das_blatt_tragen_keinen_rundweg() {
        assert_eq!(rundweg(Fokus::Leiste), None);
        assert_eq!(rundweg(Fokus::Git), None);
        assert_eq!(rundweg(Fokus::Anderswo), None);
    }

    /// Jeder Fokuswert, den der Wirkungsbereich durchlaesst, traegt einen
    /// Ausgang — und jeder, den er abweist, traegt keinen.
    ///
    /// **Die Probe haelt die beiden Regeln aneinander**, die zusammen
    /// entscheiden, was `cmd+e` tut: [`Kommando::wirkungsbereich`] sagt, ob die
    /// Taste durchkommt, und [`rundweg`] sagt, was sie dann tut. Liefen sie
    /// auseinander, gaebe es entweder einen Bereich, in dem der Befehl
    /// durchkommt und nichts findet — die Gestalt, die gerade als Defekt
    /// gemeldet war —, oder einen Ausgang, den keine Taste je erreicht.
    ///
    /// Sie ist damit die Stelle, die eine spaetere Weitung des
    /// Wirkungsbereichs auffaellig macht, ohne dass jemand daran denken muss.
    #[test]
    fn der_wirkungsbereich_und_die_regel_lassen_dieselben_bereiche_durch() {
        let bereich = Kommando::EditorRundweg.wirkungsbereich();
        assert_eq!(
            bereich,
            Wirkungsbereich::Dateibereiche,
            "der Rundweg traegt nicht mehr seinen eigenen Wirkungsbereich"
        );

        for fokus in JEDER_FOKUS {
            assert_eq!(
                wirkt(bereich, fokus),
                rundweg(fokus).is_some(),
                "der Wirkungsbereich und die Regel sind sich ueber {fokus:?} nicht einig"
            );
        }
    }
}
