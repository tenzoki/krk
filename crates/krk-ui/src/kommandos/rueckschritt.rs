//! Was ein Druck auf die nackte Rueckschritt-Taste bedeutet: ein Zeichen des
//! Filtertextes zurueck, gar nichts, oder eine Datei in den Papierkorb (C6.10).
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Die drei Wahrheitswerte liest der
//! Anwendungsdelegierte — zwei aus dem Anschlag und dem Modell des sichtbaren
//! Tabs, einen aus seinem eigenen Merker —, die Regel selbst steht hier und ist
//! ohne Fenster pruefbar.
//!
//! ```text
//!  filtertext_steht ─┐
//!  wiederholung ─────┼──> rueckschritt() ──> (Rueckschritt, Merker danach)
//!  merker ───────────┘
//! ```
//!
//! # Warum diese Regel eigens dasteht
//!
//! Sie ist die eine Fallunterscheidung dieser Runde, deren falscher Zweig
//! Dateien wegraeumt. `delete` traegt in `resources/default-keymap.toml` die
//! Funktion `in_papierkorb`, und das Raeumen laeuft ohne Rueckfrage. Ohne die
//! Unterscheidung raeumte die Berichtigung eines Vertippers Dateien weg
//! (C1.15). Deshalb steht sie als reine Funktion mit einer ausgeschriebenen
//! Tafel da und nicht als Bedingung im Ausfuehrungszweig: dort waere sie an
//! keiner Probe zu fassen.
//!
//! # Zwei Groessen, drei Wahrheitswerte
//!
//! C6.10 nennt **zwei** Groessen, an denen die Unterscheidung haengt: ob ein
//! Filtertext steht, und ob der Anschlag aus einer Tastenwiederholung stammt,
//! **die bei stehendem Filtertext begann**. Die zweite ist aus einem einzelnen
//! Anschlag nicht abzulesen, denn AppKit meldet an einem Tastenereignis allein,
//! ob es eine Wiederholung ist, und nicht, wobei die Wiederholung anfing. Sie
//! zerfaellt deshalb an der Schnittstelle in zwei Wahrheitswerte:
//!
//! 1. **`filtertext_steht`** — ob das Ordnermodell des sichtbaren Tabs des
//!    aktiven Dateifensters in diesem Augenblick einen Filtertext fuehrt.
//! 2. **`wiederholung`** — ob dieser Anschlag aus einer Tastenwiederholung
//!    stammt. Der Anwendungsdelegierte liest ihn als `isARepeat` am Ereignis.
//! 3. **`merker`** — ob die laufende Wiederholung bei stehendem Filtertext
//!    begonnen hat. Er wohnt beim Anwendungsdelegierten, wird von dieser
//!    Funktion fortgeschrieben und von jeder anderen Eingabe zurueckgesetzt.
//!
//! **Der Zusatz „die bei stehendem Filtertext begann" ist nicht schmueckend,
//! sondern die Grenze zwischen C1.18 und C1.20.** Ohne ihn hoerte auch ein
//! gehaltener Rueckschritt **ohne** jeden Filtertext nach dem ersten Anschlag
//! auf zu raeumen, und das aenderte das heutige Verhalten, das C1.16
//! ausdruecklich unangetastet laesst. Genau darum sind es drei Wahrheitswerte
//! und nicht zwei: `wiederholung` allein trennt die dritte Zeile der Tafel
//! nicht von der vierten.
//!
//! # Woran die Regel nicht haengt
//!
//! Drei Groessen liegen nahe und stehen bewusst **nicht** in der Signatur
//! (C6.10):
//!
//! - **Ob der Filtertext Treffer hat.** Ein Filtertext ohne Treffer schuetzt
//!   genauso wie einer mit Treffern; sonst raeumte ausgerechnet der Vertipper,
//!   der die Liste leert, eine Datei weg.
//! - **Ob eine Auswahl besteht.** `kommandos::operationen::betroffene` wird
//!   fuer die beiden ersten Ausgaenge gar nicht erst befragt, und weder Auswahl
//!   noch Markierung werden angefasst (C6.9).
//! - **Ob „Deep" an ist.** Das Ankreuzfeld dehnt die Suche auf den Unterbaum
//!   aus und sagt nichts darueber, was die Rueckschritt-Taste bedeutet.
//!
//! # Wo die Regel steht und wen sie nicht trifft
//!
//! Sie steht **hinter** [`zulaessigkeit::zulaessig`](super::zulaessigkeit::zulaessig)
//! und nicht darin. Das haelt dreierlei auseinander, was sonst zusammenfiele:
//! die Tafel aus 280 Faellen der Zulaessigkeitsregel behaelt ihre Bedeutung;
//! der Menueeintrag „In den Papierkorb raeumen" laeuft ohne Anschlag durch
//! denselben Ausfuehrungszweig und bekommt die Antwort deshalb nicht ab
//! (C1.19, C6.11); und beim Umbenennen in der Dateiliste haelt der Feldeditor
//! den Ersthelferrang, `zulaessig` sagt nein, und die Taste geht unveraendert
//! an AppKit, ohne dass hier ein Zweig dafuer stuende.
//!
//! Die uebrigen Loeschwege erreichen diese Funktion nie: `cmd+delete` traegt
//! eine Zusatztaste und geht am Fragezweig vorbei (C1.17), `f8` und
//! `opt+cmd+delete` tragen `Kommando::EndgueltigLoeschen`, und `ctrl+delete`
//! wirkt in der Lesezeichenleiste, also in einem anderen Wirkungsbereich
//! (C6.11).
//!
//! # Der eine Aufrufer
//!
//! `Anwendungsdelegierter::papierkorb_oder_zeichen_zurueck`
//! (`crate::appkit::anwendung`) ist der einzige, und die Probe
//! `die_regel_hat_genau_einen_aufrufer` haelt die Zahl fest. Der Weg dorthin
//! traegt der Ereignisabgriff: `ereignisse::Anschlag` nimmt neben dem
//! `Tastendruck` das Wiederholungsbit auf, `Eingabe::Kommando` traegt ihn bis
//! in die Senke, und `kommando_ausfuehren` reicht ihn als `Option<Anschlag>`
//! weiter — `None` heisst „es gab keinen Tastendruck".
//!
//! **Bis zum Setzen dieses Aufrufers trugen die beiden Stuecke unten
//! `#[cfg_attr(not(test), expect(dead_code, ...))]`**, und die Erwartung war
//! genau darauf gestellt: mit dem Aufrufer wurde sie unerfuellt, und der Bau
//! hielt unter `-D warnings` an, bis die Zeilen weg waren. Das ist der Grund,
//! aus dem dort `expect` und nicht `allow` stand — eine Ausnahme mit
//! Ablaufdatum statt einer, die stehen bleibt und niemandem mehr sagt, warum.

/// Was der Druck auf die nackte Rueckschritt-Taste ausloest.
///
/// Die drei Werte sind die drei Senken des Unterbaums aus dem dritten Bild des
/// Spec. Sie stehen als Aufzaehlung und nicht als zwei Wahrheitswerte da, weil
/// „nichts" kein Sonderfall von „raeumen" ist, sondern ein eigener Ausgang:
/// kein Auftrag, keine Meldung, und erst ein neuer Druck raeumt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rueckschritt {
    /// Das letzte Zeichen des Filtertextes faellt weg, und die Liste waechst um
    /// die Eintraege, die damit wieder passen (C1.14).
    ZeichenZurueck,
    /// Nichts geschieht: kein Auftrag, keine Meldung. Die Tastenwiederholung
    /// hat den Filtertext geleert und traegt nicht ueber diese Grenze (C1.18).
    Nichts,
    /// Der Eintrag geht in den Papierkorb, ohne Rueckfrage, wie bisher
    /// (C1.16, C1.20).
    InDenPapierkorb,
}

/// Die eine Regel der Rueckschritt-Taste, und zurueck kommt der Merker fuer den
/// naechsten Anschlag.
///
/// Der Rumpf ist diese Tafel, und sie steht ausgeschrieben und nicht gerechnet:
///
/// | `filtertext_steht` | `wiederholung` | `merker` | Ausgang | Merker danach |
/// |---|---|---|---|---|
/// | ja | gleichgueltig | gleichgueltig | [`Rueckschritt::ZeichenZurueck`] | ja |
/// | nein | nein | gleichgueltig | [`Rueckschritt::InDenPapierkorb`] | nein |
/// | nein | ja | ja | [`Rueckschritt::Nichts`] | ja |
/// | nein | ja | nein | [`Rueckschritt::InDenPapierkorb`] | nein |
///
/// **Die vier Zeilen decken alle acht Wahrheitskombinationen ab**, und die
/// Fallunterscheidung ist damit ueberschneidungsfrei und vollstaendig; einen
/// Auffangzweig gibt es nicht, und der Uebersetzer haelt die Vollstaendigkeit.
/// Die Probe `die_tafel_aus_acht_faellen_geht_auf` schreibt alle acht aus, aus demselben Grund, aus dem die Tafel in
/// [`super::zulaessigkeit`] ausgeschrieben dasteht: eine gerechnete Erwartung
/// waere die Umsetzung ein zweites Mal.
///
/// **Der Merker danach ist in jeder Zeile derselbe Wert wie der Ausgang ihn
/// braucht**, und er ist nicht `filtertext_steht` in Verkleidung: in der
/// dritten Zeile steht kein Filtertext mehr, und der Merker bleibt trotzdem
/// gesetzt — genau daran haengt C1.18.
///
/// `#[must_use]`, weil das stille Fallenlassen des Rueckgabewerts unbemerkt
/// bliebe: verlorenginge dabei der Merker, und der naechste Anschlag einer
/// gehaltenen Taste raeumte.
#[must_use]
pub fn rueckschritt(
    filtertext_steht: bool,
    wiederholung: bool,
    merker: bool,
) -> (Rueckschritt, bool) {
    match (filtertext_steht, wiederholung, merker) {
        // Ein Vertipper wird berichtigt: das letzte Zeichen faellt weg, und der
        // Merker haelt fest, dass eine von hier weiterlaufende Wiederholung bei
        // stehendem Filtertext begonnen hat (C1.14, C1.15, C6.9).
        (true, _, _) => (Rueckschritt::ZeichenZurueck, true),
        // Ein frischer Druck ohne Filtertext raeumt, wie er es vor dieser Runde
        // tat. Er beendet zugleich jede vorige Wiederholung, also faellt der
        // Merker (C1.16).
        (false, false, _) => (Rueckschritt::InDenPapierkorb, false),
        // Die gehaltene Taste hat den Filtertext geleert: sie traegt nicht ueber
        // diese Grenze, und der Merker haelt die Sperre fuer die weiteren
        // Anschlaege derselben Wiederholung (C1.18).
        (false, true, true) => (Rueckschritt::Nichts, true),
        // Eine Wiederholung, die nie einen Filtertext gesehen hat, raeumt
        // weiter, so oft sie wiederholt (C1.20).
        (false, true, false) => (Rueckschritt::InDenPapierkorb, false),
    }
}

#[cfg(test)]
mod tests {
    use crate::quellbaum::{aufrufstellen, quelldateien};

    use super::*;

    /// Genau eine Stelle im Baum ruft die Regel (C6.10).
    ///
    /// **Eine Aufruferzaehlung in der Form von
    /// `beide_frager_rufen_die_eine_regel` in [`super::super::zulaessigkeit`]**,
    /// und sie steht hier aus demselben Grund: die Zusage handelt davon, dass es
    /// diese Fallunterscheidung einmal gibt. Ein zweiter Aufrufer waere entweder
    /// ein zweiter Weg zum Papierkorb oder ein zweiter Merker, und beides ist
    /// genau der Zustand, den diese Runde vermeidet.
    ///
    /// Der eine Aufrufer ist `Anwendungsdelegierter::papierkorb_oder_zeichen_zurueck`
    /// in `crate::appkit::anwendung`. Rot wird die Probe, wenn ein zweiter
    /// hinzukommt; die richtige Antwort darauf ist die Frage, warum es ihn gibt,
    /// und nicht die Zahl hier.
    ///
    /// **Diese Datei bleibt aussen vor**, wie bei der Vorlage: die Tafel der
    /// Proben darunter ruft die Regel achtmal, und das sind keine Aufrufer im
    /// Sinne der Zusage. Was eine Aufruferzaehlung leistet und was nicht, steht
    /// in [`crate::quellbaum`].
    ///
    /// Die Nadel steht zusammengesetzt da, weil die Probe in dem Baum liegt, den
    /// sie liest.
    #[test]
    fn die_regel_hat_genau_einen_aufrufer() {
        let zuhause = "krk-ui/src/kommandos/rueckschritt.rs";
        let name = concat!("rueck", "schritt");
        let aufrufe: usize = quelldateien()
            .iter()
            .filter(|(datei, _)| datei != zuhause)
            .map(|(_, inhalt)| aufrufstellen(inhalt, name))
            .sum();
        assert_eq!(
            aufrufe, 1,
            "die Regel der Rueckschritt-Taste hat nicht genau einen Aufrufer"
        );
    }

    /// Die ganze Regel auf einen Blick: zwei Filtertextstaende mal zwei
    /// Wiederholungsbefunde mal zwei Merkerstaende, also acht Faelle.
    ///
    /// Die Tafel steht in der Form der Tafel aus [`super::super::zulaessigkeit`]
    /// und schreibt aus, was die vier Zeilen der Dokumentation von
    /// [`rueckschritt`] mit „gleichgueltig" zusammenfassen. Sie zeigt, dass
    /// keine Kombination fehlt und keine zweimal beantwortet wird; die Proben
    /// darunter zeigen einzelne Felder mit ihrer Begruendung.
    #[test]
    fn die_tafel_aus_acht_faellen_geht_auf() {
        // filtertext_steht, wiederholung, merker, Ausgang, Merker danach.
        const TAFEL: [(bool, bool, bool, Rueckschritt, bool); 8] = [
            (true, true, true, Rueckschritt::ZeichenZurueck, true),
            (true, true, false, Rueckschritt::ZeichenZurueck, true),
            (true, false, true, Rueckschritt::ZeichenZurueck, true),
            (true, false, false, Rueckschritt::ZeichenZurueck, true),
            (false, true, true, Rueckschritt::Nichts, true),
            (false, true, false, Rueckschritt::InDenPapierkorb, false),
            (false, false, true, Rueckschritt::InDenPapierkorb, false),
            (false, false, false, Rueckschritt::InDenPapierkorb, false),
        ];

        for (filtertext_steht, wiederholung, merker, ausgang, danach) in TAFEL {
            assert_eq!(
                rueckschritt(filtertext_steht, wiederholung, merker),
                (ausgang, danach),
                "filtertext_steht={filtertext_steht}, wiederholung={wiederholung}, \
                 merker={merker}"
            );
        }
    }

    /// Der erste Weg des dritten Bildes: einen Vertipper berichtigen
    /// (C1.14, C1.15, C6.9).
    ///
    /// **Der Merker ist hier gleichgueltig und die Wiederholung auch**, und das
    /// ist die Aussage der Probe: ein stehender Filtertext entscheidet allein,
    /// ohne dass die beiden anderen Werte gefragt wuerden. Beide sind deshalb
    /// ueber alle vier Kombinationen durchgefahren.
    #[test]
    fn ein_stehender_filtertext_nimmt_ein_zeichen_zurueck() {
        for wiederholung in [false, true] {
            for merker in [false, true] {
                let (ausgang, _) = rueckschritt(true, wiederholung, merker);
                assert_eq!(
                    ausgang,
                    Rueckschritt::ZeichenZurueck,
                    "ein stehender Filtertext nimmt kein Zeichen zurueck: \
                     wiederholung={wiederholung}, merker={merker}"
                );
                assert_ne!(
                    ausgang,
                    Rueckschritt::InDenPapierkorb,
                    "ein stehender Filtertext erreicht den Papierkorb: \
                     wiederholung={wiederholung}, merker={merker}"
                );
            }
        }
    }

    /// Der zweite Weg: eine Datei wegraeumen (C1.16).
    ///
    /// Ohne Filtertext und ohne Wiederholung wirkt die Taste unveraendert. Der
    /// Merker steht in beiden Staenden da, weil ein frischer Druck jede vorige
    /// Wiederholung beendet und ihn deshalb nicht mehr befragen darf.
    #[test]
    fn ohne_filtertext_raeumt_ein_frischer_druck() {
        for merker in [false, true] {
            assert_eq!(
                rueckschritt(false, false, merker),
                (Rueckschritt::InDenPapierkorb, false),
                "ein frischer Druck ohne Filtertext raeumt nicht: merker={merker}"
            );
        }
    }

    /// Der dritte Weg: die Taste zu lange halten (C1.18).
    ///
    /// Die Wiederholung hat den Filtertext geleert; der naechste Anschlag
    /// derselben Wiederholung erreicht den Papierkorb nicht, und der Merker
    /// bleibt gesetzt, damit auch der uebernaechste ihn nicht erreicht.
    #[test]
    fn eine_wiederholung_bei_stehendem_filtertext_traegt_nicht_ueber_die_grenze() {
        assert_eq!(
            rueckschritt(false, true, true),
            (Rueckschritt::Nichts, true),
            "die Wiederholung traegt ueber die Grenze"
        );
    }

    /// Der vierte Weg: mehrere Dateien am Stueck wegraeumen (C1.20).
    ///
    /// **Diese Probe ist der Grund fuer den dritten Wahrheitswert.** Ohne ihn
    /// fiele dieser Fall mit dem dritten Weg zusammen, und ein gehaltener
    /// Rueckschritt ohne jeden Filtertext hoerte nach dem ersten Anschlag auf zu
    /// raeumen — eine Aenderung des Verhaltens, das C1.16 unangetastet laesst.
    #[test]
    fn eine_wiederholung_ohne_je_stehenden_filtertext_raeumt_weiter() {
        let (ausgang, danach) = rueckschritt(false, true, false);
        assert_eq!(
            (ausgang, danach),
            (Rueckschritt::InDenPapierkorb, false),
            "die Wiederholung ohne Filtertext raeumt nicht weiter"
        );
        // Und der naechste Anschlag derselben Wiederholung raeumt wieder: der
        // Merker, den der vorige zurueckgab, sperrt nicht.
        assert_eq!(
            rueckschritt(false, true, danach),
            (Rueckschritt::InDenPapierkorb, false),
            "der zweite Anschlag derselben Wiederholung raeumt nicht"
        );
    }

    /// Der Merker ist nicht `filtertext_steht` in Verkleidung.
    ///
    /// Der Fall, an dem es sich zeigt, ist die dritte Zeile der Tafel: es steht
    /// kein Filtertext mehr, und der Merker ist trotzdem gesetzt. Wer beide
    /// Groessen fuer dieselbe haelt, baut die Sperre aus C1.18 nicht.
    #[test]
    fn der_merker_ueberlebt_das_leerwerden_des_filtertextes() {
        // Der letzte Anschlag, der noch ein Zeichen findet.
        let (ausgang, merker) = rueckschritt(true, true, false);
        assert_eq!(ausgang, Rueckschritt::ZeichenZurueck);
        assert!(merker, "der letzte Anschlag setzt den Merker nicht");

        // Derselbe gehaltene Rueckschritt, einen Anschlag spaeter: der
        // Filtertext ist leer, der Merker traegt die Sperre.
        assert_eq!(
            rueckschritt(false, true, merker),
            (Rueckschritt::Nichts, true),
            "der Merker traegt die Sperre nicht ueber das Leerwerden hinweg"
        );
    }
}
