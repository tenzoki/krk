//! Was KRK ueber die beiden Notizzettel der Runde 9 weiss: welcher offen ist,
//! was beim Oeffnen aus der Datei kam und was seither in der Flaeche steht.
//!
//! ```text
//!   f2 / cmd+k ──> text_laden ──> oeffnen(zettel, gelesen)
//!                                     │
//!   Tippen ─────> bearbeiten(stand) ──┤
//!                                     │
//!   Sicherungsmoment ──> zu_sichern() ┴─> Some((zettel, text)) ──> text_sichern
//!                                                                      │
//!                                     gesichert(zettel) <──────────────┘
//!
//!   Tabklick ──> wechseln(ziel) ──> Derselbe               nichts geschieht
//!                                ──> GewechseltUngeaendert nur nachladen
//!                                ──> GewechseltZuSichern   sichern, dann nachladen
//! ```
//!
//! **Ohne AppKit und ohne Fenster.** Das Modul liegt neben `appkit/` und nicht
//! darin, aus demselben Grund wie [`crate::editormodell`]: was der Zettel haelt,
//! ist eine Frage ueber Zeichenketten, und sie ist damit ohne Hauptfaden
//! pruefbar. Die Textflaeche, das Blatt und die zwei Tabs stehen in
//! `appkit/blaetter/zettel.rs` und setzen den Stand nur um.
//!
//! **Je Zettel zwei Zeichenketten, und der Unterschied zwischen ihnen ist die
//! ganze Sicherungsregel.** `gelesen` ist, was beim Oeffnen aus der Datei kam;
//! `gehalten` ist, was der Nutzer seither daraus gemacht hat. Sind beide gleich,
//! gibt es nichts zu schreiben — das ist die Zusage aus C4 („Ist der Text des
//! Zettels derselbe, der beim Oeffnen gelesen wurde, schreibt KRK nicht") und
//! zugleich die aus C2 („Ein Wechsel auf den bereits offenen Tab schreibt
//! nichts").
//!
//! **Beide Zettel stehen im Modell, nicht nur der offene.** Der Grund ist der
//! Tabwechsel: [`Zettelmodell::wechseln`] gibt den offenen Zettel weiter, und der
//! **verlassene** ist danach derjenige, der noch zu sichern ist. Ein Modell, das
//! allein den offenen kennte, haette ihn in demselben Augenblick vergessen, in
//! dem er gebraucht wird.
//!
//! **Zwei und nicht n.** Wie viele Zettel es gibt, sagt
//! [`krk_core::ablage::pfade::Zettel`] und nicht dieses Modul: das Feld hat die
//! Laenge der Aufzaehlung, und eine dritte Wahl gaebe es nur, wenn sie dort
//! entstuende. Das erste Abnahmekriterium von C2 ist damit eine Aussage ueber
//! einen Typ und nicht ueber eine Zeile Code.

use krk_core::ablage::pfade::Zettel;

/// Was ein Klick auf den anderen Tab bedeutet.
///
/// **Vollstaendig und ohne Auffangzweig.** Die drei Faelle unterscheiden sich in
/// dem, was der Aufrufer danach zu tun hat, und ein vierter haelt den Bau an.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wechsel {
    /// Das Ziel ist der bereits offene Zettel.
    ///
    /// Nichts geschieht: nicht geschrieben, nicht nachgeladen, der Text der
    /// Flaeche bleibt stehen. C2 sagt es ausdruecklich zu.
    Derselbe,
    /// Gewechselt, und der verlassene Zettel steht wie seine Datei.
    ///
    /// Der Aufrufer laedt den neuen Zettel nach und schreibt nichts.
    GewechseltUngeaendert,
    /// Gewechselt, und der verlassene Zettel weicht von seiner Datei ab.
    ///
    /// Der Aufrufer sichert ihn — [`Zettelmodell::zu_sichern`] nennt ihn samt
    /// Text — und laedt danach den neuen nach.
    GewechseltZuSichern,
}

/// Der Stand eines einzelnen Zettels.
#[derive(Debug, Default, Clone)]
struct Zettelstand {
    /// Was beim letzten Oeffnen oder Sichern in der Datei stand.
    gelesen: String,
    /// Was der Nutzer seither daraus gemacht hat.
    gehalten: String,
}

impl Zettelstand {
    /// Ob dieser Zettel von seiner Datei abweicht.
    fn weicht_ab(&self) -> bool {
        self.gehalten != self.gelesen
    }
}

/// Welcher Zettel offen ist und was beide tragen.
#[derive(Debug, Default, Clone)]
pub struct Zettelmodell {
    offener: Zettel,
    staende: [Zettelstand; Zettel::ALLE.len()],
}

impl Zettelmodell {
    /// Welcher Zettel gerade offen ist.
    pub fn offener(&self) -> Zettel {
        self.offener
    }

    /// Nimmt den frisch aus der Datei gelesenen Stand eines Zettels auf und
    /// macht ihn zum offenen.
    ///
    /// **Gelesen und gehalten sind danach dasselbe**, und genau das heisst
    /// „nichts zu sichern": ein Zettel, der eben erst gelesen wurde, weicht von
    /// seiner Datei nicht ab. Wer diese Gleichsetzung ausliesse, schriebe beim
    /// naechsten Sicherungsmoment einen unveraenderten Text zurueck.
    ///
    /// **Der Aufrufer liest bei jedem Oeffnen neu**, auch beim Wechsel auf den
    /// anderen Tab; C4 sagt es zu. Das Modell haelt deshalb keinen Stand von
    /// gestern fest, sondern bekommt ihn gereicht.
    pub fn oeffnen(&mut self, zettel: Zettel, gelesen: String) {
        self.offener = zettel;
        let stand = &mut self.staende[zettel.index()];
        stand.gehalten.clone_from(&gelesen);
        stand.gelesen = gelesen;
    }

    /// Nimmt den Stand der Textflaeche fuer den **offenen** Zettel auf und
    /// meldet, ob er jetzt von seiner Datei abweicht.
    ///
    /// # Der Rueckgabewert sagt, ob es etwas zu sichern gibt
    ///
    /// `true` heisst: der offene Zettel traegt andere Zeichen als seine Datei,
    /// und der naechste Sicherungsmoment schreibt. `false` heisst: er steht wie
    /// seine Datei, und geschrieben wird nichts.
    ///
    /// **Der Wert laesst sich nicht still fallenlassen**, und das ist eine
    /// Erzwingung und keine Bitte. Dieses Programm gibt jedem Rueckgabewert ein
    /// `#[must_use]`, dessen stilles Fallenlassen unbemerkt bliebe; so
    /// entschieden vom Nutzer am 260811-2140, in der Sache schon am Defekt
    /// `260810-0423`. Hier bliebe es unbemerkt, weil ein vergessenes Sichern
    /// nirgends auffaellt: der Zettel steht weiter auf dem Schirm, das Blatt
    /// schliesst, und der Verlust zeigt sich erst beim naechsten Oeffnen.
    ///
    /// **`let _ =` davor heisst „ich brauche den Wert nicht"**, und das ist in
    /// diesem Baum die einzige Lesart. Wer gleich danach
    /// [`zu_sichern`](Self::zu_sichern) fragt, braucht ihn wirklich nicht: die
    /// Frage beantwortet dasselbe und nennt dazu den Zettel und seinen Text.
    #[must_use = "sagt, ob der offene Zettel von seiner Datei abweicht"]
    pub fn bearbeiten(&mut self, stand: String) -> bool {
        let zettel = self.offener;
        self.staende[zettel.index()].gehalten = stand;
        self.staende[zettel.index()].weicht_ab()
    }

    /// Wechselt auf `ziel` und sagt, was mit dem verlassenen Zettel zu tun ist.
    ///
    /// **Der verlassene ist nach dem Wechsel `ziel.andere()`**, und das ist keine
    /// Vermutung, sondern die Folge davon, dass es genau zwei Zettel gibt.
    /// [`zu_sichern`](Self::zu_sichern) nennt ihn ohnehin beim Namen; der
    /// Rueckgabewert hier sagt, ob ueberhaupt zu sichern ist, **bevor** der
    /// Aufrufer einen Durchgang durch die Ablage nimmt.
    pub fn wechseln(&mut self, ziel: Zettel) -> Wechsel {
        if ziel == self.offener {
            return Wechsel::Derselbe;
        }
        let verlassen = self.offener;
        self.offener = ziel;
        if self.staende[verlassen.index()].weicht_ab() {
            Wechsel::GewechseltZuSichern
        } else {
            Wechsel::GewechseltUngeaendert
        }
    }

    /// Welcher Zettel zu sichern ist und mit welchem Text, falls einer es ist.
    ///
    /// **Gefragt sind beide und nicht nur der offene.** Nach einem Tabwechsel ist
    /// der zu sichernde gerade der **verlassene**; eine Frage allein nach dem
    /// offenen ginge an ihm vorbei.
    ///
    /// **Unter dem Ablauf dieser Runde weicht hoechstens einer ab**, weil jeder
    /// Wechsel den verlassenen sofort sichert. Weichen doch beide ab — das
    /// geschieht, wenn ein Schreibvorgang gescheitert ist und der Nutzer danach
    /// den anderen Zettel bearbeitet hat —, liefert diese Frage den in der
    /// Reihenfolge von [`Zettel::ALLE`] ersten, und der naechste
    /// Sicherungsmoment nimmt den zweiten. Was sie nicht tut: melden, dass es
    /// zwei sind.
    pub fn zu_sichern(&self) -> Option<(Zettel, &str)> {
        Zettel::ALLE.into_iter().find_map(|zettel| {
            let stand = &self.staende[zettel.index()];
            stand
                .weicht_ab()
                .then_some((zettel, stand.gehalten.as_str()))
        })
    }

    /// Vermerkt, dass der Text dieses Zettels jetzt in seiner Datei steht.
    ///
    /// Der gehaltene Stand wird zum gelesenen; danach gibt es an diesem Zettel
    /// nichts mehr zu sichern, bis der Nutzer ihn wieder anfasst. Aufzurufen
    /// **nach** einem gelungenen Schreibvorgang und nicht davor: eine
    /// gescheiterte Sicherung laesst den Stand stehen und wird beim naechsten
    /// Moment erneut versucht.
    pub fn gesichert(&mut self, zettel: Zettel) {
        let stand = &mut self.staende[zettel.index()];
        stand.gelesen.clone_from(&stand.gehalten);
    }
}

#[cfg(test)]
mod tests {
    use super::{Wechsel, Zettelmodell};
    use krk_core::ablage::pfade::Zettel;

    /// Ohne Zutun steht der erste Zettel offen, und es gibt nichts zu sichern.
    #[test]
    fn ein_frisches_modell_steht_auf_dem_ersten_zettel_und_ist_sauber() {
        let modell = Zettelmodell::default();
        assert_eq!(modell.offener(), Zettel::Erster);
        assert_eq!(modell.zu_sichern(), None);
    }

    /// Ein eben gelesener Zettel weicht von seiner Datei nicht ab.
    ///
    /// Die Gegenprobe zu der Zusage aus C4: ohne diese Gleichsetzung schriebe
    /// der erste Sicherungsmoment einen unveraenderten Text zurueck.
    #[test]
    fn ein_gelesener_zettel_hat_nichts_zu_sichern() {
        let mut modell = Zettelmodell::default();
        modell.oeffnen(Zettel::Erster, "aus der Datei".to_owned());
        assert_eq!(modell.zu_sichern(), None);
    }

    /// Getippter Text macht den offenen Zettel zu sicherndem.
    #[test]
    fn bearbeiten_meldet_die_abweichung_und_nennt_den_zettel() {
        let mut modell = Zettelmodell::default();
        modell.oeffnen(Zettel::Zweiter, "alt".to_owned());
        assert!(modell.bearbeiten("neu".to_owned()));
        assert_eq!(modell.zu_sichern(), Some((Zettel::Zweiter, "neu")));
    }

    /// Wer den gelesenen Text wieder herstellt, hat nichts zu sichern.
    ///
    /// Die Frage ist der Vergleich mit der Datei und nicht, ob der Nutzer getippt
    /// hat; ein zurueckgenommenes Zeichen ist keine Aenderung mehr.
    #[test]
    fn der_zurueckgetippte_stand_ist_wieder_sauber() {
        let mut modell = Zettelmodell::default();
        modell.oeffnen(Zettel::Erster, "alt".to_owned());
        assert!(modell.bearbeiten("altx".to_owned()));
        assert!(!modell.bearbeiten("alt".to_owned()));
        assert_eq!(modell.zu_sichern(), None);
    }

    /// Nach dem Sichern steht der gehaltene Stand als der gelesene.
    #[test]
    fn gesichert_nimmt_dem_zettel_die_abweichung() {
        let mut modell = Zettelmodell::default();
        modell.oeffnen(Zettel::Erster, "alt".to_owned());
        assert!(modell.bearbeiten("neu".to_owned()));
        modell.gesichert(Zettel::Erster);
        assert_eq!(modell.zu_sichern(), None);
    }

    /// Der Wechsel auf den bereits offenen Tab schreibt nichts (C2).
    ///
    /// Auch dann nicht, wenn der offene Zettel abweicht: der Klick fuehrt nicht
    /// aus ihm heraus.
    #[test]
    fn ein_wechsel_auf_den_offenen_tab_ist_derselbe() {
        let mut modell = Zettelmodell::default();
        modell.oeffnen(Zettel::Erster, "alt".to_owned());
        assert!(modell.bearbeiten("neu".to_owned()));
        assert_eq!(modell.wechseln(Zettel::Erster), Wechsel::Derselbe);
        assert_eq!(modell.offener(), Zettel::Erster);
    }

    /// Der Tabwechsel sichert den verlassenen Zettel (C2).
    ///
    /// Die Zusage ohne Fenster gemessen: nach dem Wechsel nennt `zu_sichern` den
    /// **verlassenen** Zettel und seinen Text, nicht den neu offenen.
    #[test]
    fn ein_wechsel_laesst_den_verlassenen_zettel_zu_sichern() {
        let mut modell = Zettelmodell::default();
        modell.oeffnen(Zettel::Erster, "alt".to_owned());
        assert!(modell.bearbeiten("getippt".to_owned()));
        assert_eq!(
            modell.wechseln(Zettel::Zweiter),
            Wechsel::GewechseltZuSichern
        );
        assert_eq!(modell.offener(), Zettel::Zweiter);
        assert_eq!(modell.zu_sichern(), Some((Zettel::Erster, "getippt")));
    }

    /// Ein Wechsel ohne Aenderung hat nichts zu sichern.
    #[test]
    fn ein_wechsel_ohne_aenderung_schreibt_nichts() {
        let mut modell = Zettelmodell::default();
        modell.oeffnen(Zettel::Erster, "alt".to_owned());
        assert_eq!(
            modell.wechseln(Zettel::Zweiter),
            Wechsel::GewechseltUngeaendert
        );
        assert_eq!(modell.zu_sichern(), None);
    }

    /// Der Text des einen Zettels landet nicht im anderen.
    ///
    /// Der Fall, den ein Modell mit nur einem Stand stillschweigend falsch
    /// beantwortete: nach Hin- und Rueckwechsel traegt jeder Zettel wieder das
    /// Seine.
    #[test]
    fn die_beiden_zettel_halten_ihre_texte_auseinander() {
        let mut modell = Zettelmodell::default();
        modell.oeffnen(Zettel::Erster, "eins".to_owned());
        assert!(modell.bearbeiten("eins geaendert".to_owned()));
        assert_eq!(
            modell.wechseln(Zettel::Zweiter),
            Wechsel::GewechseltZuSichern
        );
        modell.gesichert(Zettel::Erster);
        modell.oeffnen(Zettel::Zweiter, "zwei".to_owned());
        assert!(modell.bearbeiten("zwei geaendert".to_owned()));
        assert_eq!(
            modell.zu_sichern(),
            Some((Zettel::Zweiter, "zwei geaendert"))
        );
    }
}
