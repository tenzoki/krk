//! Das Fenstermodell: welches Dateifenster das aktive ist, welche Bereiche
//! sichtbar sind, wie breit sie stehen und in welcher Reihenfolge beim Start
//! gelesen wird.
//!
//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile. Die
//! Ansicht dazu ist [`crate::appkit::aufteilung`], die aus den Zahlen hier
//! Rahmen fuer die vier Bereiche einer `NSSplitView` macht.
//!
//! # Die vier Bereiche
//!
//! ```text
//! ┌───────────┬──────────────────┬──────────────────┬───────────┐
//! │ Lesezei-  │ linkes           │ rechtes          │ Vorschau  │
//! │ chen (C5) │ Dateifenster     │ Dateifenster     │ (C6)      │
//! └───────────┴──────────────────┴──────────────────┴───────────┘
//!    fest          beweglich          beweglich         fest
//! ```
//!
//! "Fest" und "beweglich" beziehen sich auf das Verteilen des Platzes: die
//! beiden Randbereiche behalten ihre Breite, die beiden Dateifenster teilen
//! sich, was uebrig bleibt. Das ist die Aufteilung, die C7 verlangt ("Die
//! verbleibenden Bereiche nutzen den frei gewordenen Platz"), und sie macht
//! zugleich die Zusage darunter einfach: eine gespeicherte Breite gilt auch
//! fuer einen ausgeblendeten Bereich, also steht sie beim Wiedereinblenden
//! wieder da.
//!
//! # Was das linke Dateifenster von den anderen unterscheidet
//!
//! Es laesst sich nicht ausblenden. C7 sichert zu, dass mindestens ein
//! Dateifenster sichtbar bleibt, und [`Sichtbarkeit`] traegt deshalb gar kein
//! Feld dafuer. [`Fenstermodell::umschalten`] weist einen Befehl auf
//! [`Bereich::Links`] trotzdem ausdruecklich ab, statt sich auf die fehlende
//! Belegung zu verlassen: die Zusage gehoert an die Stelle, die sie einloest.

use krk_core::ablage::{
    Breiten, Dateifenster as Fensterzustand, Fensterseite, Sichtbarkeit, Sitzung,
};

use crate::tabs::Tabuebersicht;

/// Um wie viele Punkte ein Tastenbefehl einen Bereich breiter oder schmaler
/// macht.
///
/// Zwei Zeilenhoehen der Dateiliste. Ein kleinerer Schritt braeuchte zu viele
/// Anschlaege, ein groesserer spraenge ueber die gesuchte Breite hinweg.
pub const BREITENSCHRITT: f64 = 40.0;

/// Einer der vier Bereiche der Fensterzeile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bereich {
    /// Die Lesezeichen- und Geraeteleiste ganz links (C5).
    Lesezeichen,
    /// Das linke Dateifenster.
    Links,
    /// Das rechte Dateifenster.
    Rechts,
    /// Das Vorschaufenster ganz rechts (C6).
    Vorschau,
}

impl Bereich {
    /// Alle vier, von links nach rechts.
    pub const ALLE: [Bereich; 4] = [
        Bereich::Lesezeichen,
        Bereich::Links,
        Bereich::Rechts,
        Bereich::Vorschau,
    ];

    /// Die Stelle des Bereichs in der Fensterzeile.
    pub const fn index(self) -> usize {
        match self {
            Bereich::Lesezeichen => 0,
            Bereich::Links => 1,
            Bereich::Rechts => 2,
            Bereich::Vorschau => 3,
        }
    }

    /// Der Bereich eines Dateifensters.
    pub const fn von_seite(seite: Fensterseite) -> Self {
        match seite {
            Fensterseite::Links => Bereich::Links,
            Fensterseite::Rechts => Bereich::Rechts,
        }
    }

    /// Die Breite, unter die sich der Bereich nicht ziehen laesst.
    ///
    /// Ein Dateifenster braucht mehr, weil vier Spalten hineinpassen muessen;
    /// die beiden Randbereiche tragen je eine Liste mit einer Spalte.
    pub const fn mindestbreite(self) -> f64 {
        match self {
            Bereich::Lesezeichen => 120.0,
            Bereich::Links | Bereich::Rechts => 240.0,
            Bereich::Vorschau => 160.0,
        }
    }

    /// Die Breite, mit der der Bereich beim allerersten Start aufgeht.
    pub const fn anfangsbreite(self) -> f64 {
        match self {
            Bereich::Lesezeichen => 180.0,
            Bereich::Links | Bereich::Rechts => 420.0,
            Bereich::Vorschau => 260.0,
        }
    }

    /// Ob dieser Bereich sich den frei werdenden Platz teilt.
    ///
    /// Die beiden Dateifenster tun es, die beiden Randbereiche nicht.
    const fn ist_beweglich(self) -> bool {
        matches!(self, Bereich::Links | Bereich::Rechts)
    }
}

/// Das gehaltene Fenstermodell.
///
/// Es traegt, was nicht zu den Tabs gehoert: das aktive Dateifenster, die
/// Sichtbarkeit der vier Bereiche und ihre Breiten. Die Tabs selbst haelt
/// [`Tabliste`], je eine Liste je Dateifenster.
#[derive(Debug)]
pub struct Fenstermodell {
    aktiv: Fensterseite,
    breiten: Breiten,
    sichtbar: Sichtbarkeit,
}

impl Fenstermodell {
    /// Das Modell aus einer geladenen Sitzung.
    ///
    /// **Ein ausgeblendetes Dateifenster kann nicht das aktive sein**, und die
    /// Zusicherung wird hier hergestellt und nicht nur unterstellt.
    /// [`Fenstermodell::umschalten`] haelt sie fuer jede Umschaltung zur
    /// Laufzeit, aber `session.toml` kommt nicht von dort: die Datei ist nach
    /// C7 zum Lesen und Aendern von Hand gedacht, und `aktiv = "rechts"` neben
    /// `zweites_dateifenster = false` ist ein Paar, das `serde` anstandslos
    /// einliest. Ohne diese Zeilen faende der Nutzer nach dem Start seine
    /// Auswahl, seinen Eingabefokus und jede Dateioperation in einem
    /// Dateifenster, das er nicht sieht.
    pub fn aus_sitzung(sitzung: &Sitzung) -> Self {
        let mut modell = Self {
            aktiv: sitzung.aktiv,
            breiten: sitzung.breiten,
            sichtbar: sitzung.sichtbar,
        };
        if !modell.sichtbar(Bereich::von_seite(modell.aktiv)) {
            // Das linke ist immer sichtbar, siehe den Modulkopf.
            modell.aktiv = Fensterseite::Links;
        }
        modell
    }

    /// Die Sitzung, wie sie in `session.toml` gehoert.
    ///
    /// Die Tabs kommen von aussen dazu, weil sie in
    /// [`Tabliste`](crate::tabs::Tabliste) wohnen und nicht hier. Damit gibt es
    /// genau einen Ort je Angabe und keine zweite Wahrheit ueber die Tabs.
    pub fn sitzung(&self, fenster: [Fensterzustand; 2]) -> Sitzung {
        Sitzung {
            aktiv: self.aktiv,
            breiten: self.breiten,
            sichtbar: self.sichtbar,
            fenster,
        }
    }

    /// Welches Dateifenster gerade das aktive ist.
    ///
    /// Bei Dateioperationen ist es die Quelle und das andere das Ziel (C1).
    pub fn aktiv(&self) -> Fensterseite {
        self.aktiv
    }

    /// Macht das genannte Dateifenster zum aktiven, falls es sichtbar ist.
    ///
    /// Liefert, ob sich dadurch etwas geaendert hat.
    pub fn aktiv_setzen(&mut self, seite: Fensterseite) -> bool {
        if seite == self.aktiv || !self.sichtbar(Bereich::von_seite(seite)) {
            return false;
        }
        self.aktiv = seite;
        true
    }

    /// Wechselt das aktive Dateifenster (C1).
    ///
    /// Ist das andere ausgeblendet, geschieht nichts: ein aktives Dateifenster,
    /// das niemand sieht, waere ein Fenster ohne sichtbare Auswahl.
    pub fn fenster_wechseln(&mut self) -> bool {
        self.aktiv_setzen(self.aktiv.andere())
    }

    /// Ob der Bereich sichtbar ist.
    pub fn sichtbar(&self, bereich: Bereich) -> bool {
        match bereich {
            Bereich::Lesezeichen => self.sichtbar.lesezeichen,
            Bereich::Links => true,
            Bereich::Rechts => self.sichtbar.zweites_dateifenster,
            Bereich::Vorschau => self.sichtbar.vorschau,
        }
    }

    /// Die Sichtbarkeit aller vier Bereiche, von links nach rechts.
    pub fn sichtbarkeit(&self) -> Sichtbarkeit {
        self.sichtbar
    }

    /// Blendet einen Bereich aus oder wieder ein (C7).
    ///
    /// Liefert `false` fuer den einen Befehl, den C7 ausdruecklich verwirft:
    /// das letzte sichtbare Dateifenster auszublenden. Der Aufrufer meldet
    /// nichts; die Zusage lautet "wird ohne Fehlermeldung ignoriert".
    ///
    /// War der ausgeblendete Bereich das aktive Dateifenster, wandert die
    /// Aktivitaet auf das andere.
    pub fn umschalten(&mut self, bereich: Bereich) -> bool {
        match bereich {
            Bereich::Lesezeichen => {
                self.sichtbar.lesezeichen = !self.sichtbar.lesezeichen;
                true
            }
            Bereich::Vorschau => {
                self.sichtbar.vorschau = !self.sichtbar.vorschau;
                true
            }
            Bereich::Rechts => {
                self.sichtbar.zweites_dateifenster = !self.sichtbar.zweites_dateifenster;
                if !self.sichtbar.zweites_dateifenster && self.aktiv == Fensterseite::Rechts {
                    self.aktiv = Fensterseite::Links;
                }
                true
            }
            // Das letzte sichtbare Dateifenster. Kein ausgeliefertes Kuerzel
            // fuehrt heute hierher; die Abweisung steht trotzdem hier und nicht
            // in der Belegungsdatei, weil eine spaetere Belegung sie sonst
            // umgehen koennte.
            Bereich::Links => false,
        }
    }

    /// Holt einen ausgeblendeten Bereich hervor und blendet nie einen aus.
    ///
    /// Liefert, ob sich dadurch etwas geaendert hat; ein schon sichtbarer
    /// Bereich liefert `false` und bleibt unangetastet.
    ///
    /// **Die eine Stelle der Asymmetrie**, und sie steht neben
    /// [`Fenstermodell::umschalten`] statt bei den Befehlen, die sie brauchen.
    /// Drei tun das heute: `shift+f3` aus C10 zeigt den Inhalt der
    /// Zwischenablage in der Vorschau, und seit dem Nutzerentscheid vom 260807
    /// holen auch die Fokusbefehle ihren Bereich hervor
    /// (`decisions/260805-1730_*_holt-der-fokusbefehl-eine-ausgeblendete-leiste-hervor.md`).
    /// Der gemeinsame Grund: wer einen Bereich verlangt, verlangt damit, ihn
    /// zu sehen; ausblenden tut keiner von ihnen, dafuer bleiben die Befehle
    /// aus C7. Es entsteht deshalb keine zweite Wahrheit ueber die
    /// Sichtbarkeit, sondern dieselbe Asymmetrie an einer Stelle.
    pub fn einblenden(&mut self, bereich: Bereich) -> bool {
        if self.sichtbar(bereich) {
            return false;
        }
        self.umschalten(bereich)
    }

    /// Die gespeicherten Breiten.
    pub fn breiten(&self) -> Breiten {
        self.breiten
    }

    /// Aendert die Breite eines Bereichs um den genannten Betrag.
    ///
    /// Der Weg der beiden Tastenbefehle aus C7. Ein Randbereich waechst
    /// unmittelbar; bei einem Dateifenster **verschiebt der Befehl die
    /// Trennlinie**, das heisst das andere Dateifenster gibt genau so viel ab,
    /// wie dieses bekommt. Der Unterschied ist nicht kosmetisch: die beiden
    /// Dateifenster teilen sich ihren Platz nach dem Verhaeltnis ihrer Breiten
    /// (siehe [`bereichsbreiten`]), und eine Zahl, die nur den Zaehler erhoeht,
    /// verschoebe die Linie um einen Bruchteil des Schritts. Gemessen am
    /// 260804 im laufenden Buendel: 40 Punkte Zuwachs bewegten die Linie um 13.
    ///
    /// Am Mindestmass hoert der Schritt auf, statt es zu unterschreiten.
    pub fn breite_aendern(&mut self, bereich: Bereich, betrag: f64) {
        if bereich.ist_beweglich() {
            let anderer = match bereich {
                Bereich::Links => Bereich::Rechts,
                _ => Bereich::Links,
            };
            if !self.sichtbar(anderer) {
                // Ein einziges sichtbares Dateifenster nimmt ohnehin die ganze
                // Breite; es gibt keine Trennlinie, die sich verschieben liesse.
                return;
            }
            let hier = self.breite_oder_anfang(bereich);
            let dort = self.breite_oder_anfang(anderer);
            let betrag = betrag
                .min(dort - anderer.mindestbreite())
                .max(bereich.mindestbreite() - hier);
            self.breite_setzen(bereich, hier + betrag);
            self.breite_setzen(anderer, dort - betrag);
            return;
        }
        let jetzt = self.breite_oder_anfang(bereich);
        self.breite_setzen(bereich, (jetzt + betrag).max(bereich.mindestbreite()));
    }

    /// Uebernimmt die Breiten, die gerade wirklich auf dem Schirm stehen.
    ///
    /// Gerufen, bevor die Sitzung geschrieben wird und bevor ein Tastenbefehl
    /// eine Breite aendert: eine mit der Maus verschobene Trennlinie steht in
    /// den Rahmen der Ansichten und nirgends sonst. Ein ausgeblendeter Bereich
    /// behaelt seine gespeicherte Breite, weil C7 verlangt, dass das
    /// Wiedereinblenden sie herstellt.
    ///
    /// **Die beiden Dateifenster bleiben unangetastet, solange nur eines von
    /// ihnen sichtbar ist.** Das sichtbare traegt dann den Platz des anderen
    /// mit, und diese Zahl zu uebernehmen hiesse, das Verhaeltnis der beiden
    /// zueinander mit einem Wert zu ueberschreiben, in dem das andere gar nicht
    /// vorkommt. Am 260804 im laufenden Buendel gemessen: das zweite
    /// Dateifenster kam nach dem Wiedereinblenden auf 269 Punkten statt auf
    /// seinen 406 zurueck, und das verfehlte die Zusage aus C7.
    pub fn breiten_uebernehmen(&mut self, gemessen: [f64; 4]) {
        let beide_dateifenster = self.sichtbar(Bereich::Links) && self.sichtbar(Bereich::Rechts);
        for bereich in Bereich::ALLE {
            if bereich.ist_beweglich() && !beide_dateifenster {
                continue;
            }
            if self.sichtbar(bereich) && gemessen[bereich.index()] > 0.0 {
                self.breite_setzen(bereich, gemessen[bereich.index()]);
            }
        }
    }

    /// Die gespeicherte Breite eines Bereichs, sonst seine Anfangsbreite.
    fn breite_oder_anfang(&self, bereich: Bereich) -> f64 {
        self.breite(bereich)
            .unwrap_or_else(|| bereich.anfangsbreite())
    }

    /// In welcher Reihenfolge die Tabs beim Start gelesen werden.
    ///
    /// Zuerst der sichtbare Tab jedes sichtbaren Dateifensters, danach alles
    /// uebrige. Die Reihenfolge folgt aus C8: L4 endet, sobald die sichtbaren
    /// Tabs ihre erste Bildschirmseite zeigen.
    pub fn lesereihenfolge(&self, fenster: [Tabuebersicht; 2]) -> Vec<(Fensterseite, usize)> {
        let mut reihenfolge = Vec::new();
        for seite in Fensterseite::ALLE {
            if self.sichtbar(Bereich::von_seite(seite)) {
                reihenfolge.push((seite, fenster[seite.index()].sichtbar));
            }
        }
        for seite in Fensterseite::ALLE {
            for stelle in 0..fenster[seite.index()].zahl {
                if !reihenfolge.contains(&(seite, stelle)) {
                    reihenfolge.push((seite, stelle));
                }
            }
        }
        reihenfolge
    }

    /// Die gespeicherte Breite eines Bereichs, falls es eine gibt.
    fn breite(&self, bereich: Bereich) -> Option<f64> {
        match bereich {
            Bereich::Lesezeichen => self.breiten.lesezeichen,
            Bereich::Links => self.breiten.links,
            Bereich::Rechts => self.breiten.rechts,
            Bereich::Vorschau => self.breiten.vorschau,
        }
    }

    fn breite_setzen(&mut self, bereich: Bereich, breite: f64) {
        let feld = match bereich {
            Bereich::Lesezeichen => &mut self.breiten.lesezeichen,
            Bereich::Links => &mut self.breiten.links,
            Bereich::Rechts => &mut self.breiten.rechts,
            Bereich::Vorschau => &mut self.breiten.vorschau,
        };
        *feld = Some(breite);
    }
}

/// Verteilt den verfuegbaren Platz auf die vier Bereiche.
///
/// Ein ausgeblendeter Bereich bekommt `0.0`; seine gespeicherte Breite bleibt
/// unangetastet und steht beim Wiedereinblenden wieder zur Verfuegung.
///
/// Die Regel in drei Saetzen. Die beiden Randbereiche bekommen ihre
/// gespeicherte Breite, hoechstens aber so viel, dass fuer die Dateifenster
/// deren Mindestbreite bleibt. Was danach uebrig ist, teilen die sichtbaren
/// Dateifenster im Verhaeltnis ihrer gespeicherten Breiten. Ist nur eines
/// sichtbar, bekommt es alles.
///
/// Dass die Dateifenster ueber ein **Verhaeltnis** und nicht ueber ihre
/// absolute Zahl gehen, ist die Antwort auf zwei Fragen zugleich: eine
/// Fenstervergroesserung kommt dort an, wo der Nutzer sie braucht, und die
/// beiden Tastenbefehle aus C7 verschieben die Trennlinie, statt eine Breite zu
/// setzen, die der naechste Bildaufbau wieder einkassiert.
pub fn bereichsbreiten(verfuegbar: f64, breiten: &Breiten, sichtbar: &Sichtbarkeit) -> [f64; 4] {
    let modell = Fenstermodell {
        aktiv: Fensterseite::Links,
        breiten: *breiten,
        sichtbar: *sichtbar,
    };
    let mut ergebnis = [0.0_f64; 4];

    // Was die sichtbaren Dateifenster mindestens brauchen.
    let mindestens_dateifenster: f64 = Bereich::ALLE
        .iter()
        .filter(|bereich| bereich.ist_beweglich() && modell.sichtbar(**bereich))
        .map(|bereich| bereich.mindestbreite())
        .sum();

    let mut rest = verfuegbar;
    for bereich in [Bereich::Lesezeichen, Bereich::Vorschau] {
        if !modell.sichtbar(bereich) {
            continue;
        }
        let gewuenscht = modell
            .breite(bereich)
            .unwrap_or_else(|| bereich.anfangsbreite())
            .max(bereich.mindestbreite());
        // Nie so viel, dass die Dateifenster unter ihr Mindestmass fallen.
        let hoechstens = (rest - mindestens_dateifenster).max(0.0);
        let breite = gewuenscht.min(hoechstens);
        ergebnis[bereich.index()] = breite;
        rest -= breite;
    }

    let beweglich: Vec<Bereich> = Bereich::ALLE
        .into_iter()
        .filter(|bereich| bereich.ist_beweglich() && modell.sichtbar(*bereich))
        .collect();
    let rest = rest.max(0.0);
    match beweglich.as_slice() {
        [eines] => ergebnis[eines.index()] = rest,
        [links, rechts] => {
            let wunsch_links = modell
                .breite(*links)
                .unwrap_or_else(|| links.anfangsbreite())
                .max(1.0);
            let wunsch_rechts = modell
                .breite(*rechts)
                .unwrap_or_else(|| rechts.anfangsbreite())
                .max(1.0);
            let anteil = wunsch_links / (wunsch_links + wunsch_rechts);
            let mut breite_links = rest * anteil;
            // Das Mindestmass gewinnt gegen das Verhaeltnis. Beide zugleich
            // sind nur zu halten, solange das Fenster breit genug ist, und
            // dafuer sorgt seine eigene Mindestgroesse.
            breite_links = breite_links
                .max(links.mindestbreite())
                .min((rest - rechts.mindestbreite()).max(links.mindestbreite()));
            ergebnis[links.index()] = breite_links;
            ergebnis[rechts.index()] = (rest - breite_links).max(0.0);
        }
        // Ohne sichtbares Dateifenster gibt es nichts zu verteilen. Der Fall
        // tritt nicht ein: `Fenstermodell::umschalten` weist ihn ab.
        _ => {}
    }
    ergebnis
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modell() -> Fenstermodell {
        Fenstermodell::aus_sitzung(&Sitzung::default())
    }

    #[test]
    fn der_auslieferungszustand_zeigt_alle_vier_bereiche() {
        let modell = modell();
        for bereich in Bereich::ALLE {
            assert!(modell.sichtbar(bereich), "{bereich:?} ist ausgeblendet");
        }
        assert_eq!(modell.aktiv(), Fensterseite::Links);
    }

    /// Eine von Hand geaenderte `session.toml` macht kein ausgeblendetes
    /// Dateifenster zum aktiven.
    ///
    /// Gefunden am 260805 neben dem Fokusdefekt: `umschalten` haelt die
    /// Zusicherung, `aus_sitzung` hielt sie nicht.
    #[test]
    fn ein_ausgeblendetes_dateifenster_kommt_nicht_als_aktives_aus_der_sitzung() {
        let mut sitzung = Sitzung {
            aktiv: Fensterseite::Rechts,
            ..Sitzung::default()
        };
        sitzung.sichtbar.zweites_dateifenster = false;
        let modell = Fenstermodell::aus_sitzung(&sitzung);
        assert_eq!(
            modell.aktiv(),
            Fensterseite::Links,
            "das aktive Dateifenster waere unsichtbar"
        );

        // Die Gegenprobe: ist es sichtbar, bleibt es das aktive.
        let sitzung = Sitzung {
            aktiv: Fensterseite::Rechts,
            ..Sitzung::default()
        };
        assert_eq!(
            Fenstermodell::aus_sitzung(&sitzung).aktiv(),
            Fensterseite::Rechts
        );
    }

    #[test]
    fn das_letzte_dateifenster_laesst_sich_nicht_ausblenden() {
        let mut modell = modell();
        assert!(
            !modell.umschalten(Bereich::Links),
            "C7 verwirft diesen Befehl"
        );
        assert!(modell.sichtbar(Bereich::Links));
    }

    #[test]
    fn das_zweite_dateifenster_geht_aus_und_wieder_ein() {
        let mut modell = modell();
        assert!(modell.umschalten(Bereich::Rechts));
        assert!(!modell.sichtbar(Bereich::Rechts));
        assert!(modell.umschalten(Bereich::Rechts));
        assert!(modell.sichtbar(Bereich::Rechts));
    }

    /// Das Einblenden holt hervor, was ausgeblendet ist, und laesst alles
    /// andere stehen.
    ///
    /// Die Zusage hinter dem Nutzerentscheid vom 260807: `shift+cmd+l` auf
    /// eine ausgeblendete Leiste blendet sie ein, `shift+cmd+l` auf eine
    /// sichtbare laesst sie sichtbar. Ohne die zweite Haelfte waere aus dem
    /// Fokusbefehl ein zweites Umschalten geworden, und zwei Befehle sagten
    /// widersprechendes ueber denselben Bereich.
    #[test]
    fn das_einblenden_holt_hervor_und_blendet_nie_aus() {
        for bereich in [Bereich::Lesezeichen, Bereich::Vorschau, Bereich::Rechts] {
            let mut modell = modell();
            assert!(
                modell.sichtbar(bereich),
                "die Probe beginnt mit sichtbarem Bereich"
            );
            assert!(
                !modell.einblenden(bereich),
                "ein sichtbarer Bereich aendert sich nicht"
            );
            assert!(modell.sichtbar(bereich), "und bleibt sichtbar");

            modell.umschalten(bereich);
            assert!(!modell.sichtbar(bereich));
            assert!(modell.einblenden(bereich), "der ausgeblendete kommt hervor");
            assert!(modell.sichtbar(bereich));
        }
    }

    /// Das linke Dateifenster ist nie ausgeblendet, und deshalb hat der
    /// Fokusbefehl dorthin nichts hervorzuholen.
    #[test]
    fn das_letzte_dateifenster_ist_immer_schon_eingeblendet() {
        let mut modell = modell();
        assert!(modell.sichtbar(Bereich::Links));
        assert!(!modell.einblenden(Bereich::Links));
        assert!(modell.sichtbar(Bereich::Links));
    }

    #[test]
    fn das_ausblenden_holt_die_aktivitaet_zurueck_nach_links() {
        let mut modell = modell();
        assert!(modell.fenster_wechseln());
        assert_eq!(modell.aktiv(), Fensterseite::Rechts);
        modell.umschalten(Bereich::Rechts);
        assert_eq!(
            modell.aktiv(),
            Fensterseite::Links,
            "ein ausgeblendetes Dateifenster kann nicht das aktive sein"
        );
        assert!(
            !modell.fenster_wechseln(),
            "und der Wechsel dorthin geschieht nicht"
        );
    }

    #[test]
    fn eine_ausgeblendete_breite_bleibt_erhalten() {
        let mut modell = modell();
        modell.breite_setzen(Bereich::Lesezeichen, 200.0);
        modell.umschalten(Bereich::Lesezeichen);
        let breiten = bereichsbreiten(1200.0, &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Lesezeichen.index()], 0.0);
        assert_eq!(
            modell.breiten().lesezeichen,
            Some(200.0),
            "die gespeicherte Breite ueberlebt das Ausblenden"
        );

        modell.umschalten(Bereich::Lesezeichen);
        let breiten = bereichsbreiten(1200.0, &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Lesezeichen.index()], 200.0);
    }

    #[test]
    fn der_frei_gewordene_platz_geht_an_die_dateifenster() {
        let modell = modell();
        let voll = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        let summe_voll: f64 = voll.iter().sum();
        assert!((summe_voll - 1400.0).abs() < 0.001, "{voll:?}");

        let mut ohne_vorschau = modell;
        ohne_vorschau.umschalten(Bereich::Vorschau);
        let jetzt = bereichsbreiten(
            1400.0,
            &ohne_vorschau.breiten(),
            &ohne_vorschau.sichtbarkeit(),
        );
        assert_eq!(jetzt[Bereich::Vorschau.index()], 0.0);
        assert!(
            jetzt[Bereich::Links.index()] > voll[Bereich::Links.index()],
            "das linke Dateifenster hat nichts vom frei gewordenen Platz bekommen"
        );
        assert!(jetzt[Bereich::Rechts.index()] > voll[Bereich::Rechts.index()]);
        let summe: f64 = jetzt.iter().sum();
        assert!((summe - 1400.0).abs() < 0.001, "{jetzt:?}");
    }

    #[test]
    fn ein_einziges_dateifenster_nimmt_die_ganze_breite() {
        let mut modell = modell();
        modell.umschalten(Bereich::Rechts);
        modell.umschalten(Bereich::Lesezeichen);
        modell.umschalten(Bereich::Vorschau);
        let breiten = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten, [0.0, 1400.0, 0.0, 0.0]);
    }

    #[test]
    fn kein_bereich_faellt_unter_sein_mindestmass() {
        let modell = modell();
        // Enger, als die vier Mindestbreiten zusammen erlauben.
        let breiten = bereichsbreiten(500.0, &modell.breiten(), &modell.sichtbarkeit());
        assert!(
            breiten[Bereich::Links.index()] >= Bereich::Links.mindestbreite(),
            "{breiten:?}"
        );
        assert!(
            breiten[Bereich::Rechts.index()] >= Bereich::Rechts.mindestbreite(),
            "{breiten:?}"
        );
    }

    /// Der Tastenbefehl bewegt die Trennlinie um genau einen Schritt.
    ///
    /// Die Zahl im Modell und die Zahl auf dem Schirm muessen dieselbe sein.
    /// Bevor `breite_aendern` das andere Dateifenster mitzog, waren es 13
    /// Punkte statt 40, gemessen am 260804 im laufenden Buendel.
    #[test]
    fn der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt() {
        let mut modell = modell();
        modell.breiten_uebernehmen(bereichsbreiten(
            1400.0,
            &modell.breiten(),
            &modell.sichtbarkeit(),
        ));
        let vorher = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());

        modell.breite_aendern(Bereich::Links, BREITENSCHRITT);
        let nachher = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        assert!(
            (nachher[Bereich::Links.index()] - vorher[Bereich::Links.index()] - BREITENSCHRITT)
                .abs()
                < 0.001,
            "vorher {vorher:?}, nachher {nachher:?}"
        );
        assert!(
            (vorher[Bereich::Rechts.index()] - nachher[Bereich::Rechts.index()] - BREITENSCHRITT)
                .abs()
                < 0.001,
            "das andere Dateifenster gibt nicht ab, was dieses bekommt"
        );

        modell.breite_aendern(Bereich::Links, -BREITENSCHRITT);
        let zurueck = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        assert!((zurueck[Bereich::Links.index()] - vorher[Bereich::Links.index()]).abs() < 0.001);
    }

    /// Das wiedereingeblendete Dateifenster kommt auf seiner alten Breite
    /// zurueck.
    ///
    /// Der Fall, der die Zusage aus C7 am 260804 im laufenden Buendel verfehlt
    /// hat: das sichtbare Dateifenster traegt den Platz des ausgeblendeten mit,
    /// und diese Zahl darf nicht als sein Wunsch in das Modell zurueckfliessen.
    #[test]
    fn das_wiedereingeblendete_dateifenster_hat_wieder_seine_alte_breite() {
        let mut modell = modell();
        modell.breiten_uebernehmen(bereichsbreiten(
            1400.0,
            &modell.breiten(),
            &modell.sichtbarkeit(),
        ));
        let vorher = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());

        modell.umschalten(Bereich::Rechts);
        // Der Bildaufbau schreibt die gemessenen Breiten zurueck, so wie es der
        // Sitzungsabgleich und jeder Breitenbefehl tun.
        let alleine = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        modell.breiten_uebernehmen(alleine);
        assert!(
            alleine[Bereich::Links.index()] > vorher[Bereich::Links.index()],
            "das linke Dateifenster hat den Platz nicht uebernommen"
        );

        modell.umschalten(Bereich::Rechts);
        let nachher = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        assert!(
            (nachher[Bereich::Rechts.index()] - vorher[Bereich::Rechts.index()]).abs() < 0.001,
            "vorher {vorher:?}, nachher {nachher:?}"
        );
        assert!((nachher[Bereich::Links.index()] - vorher[Bereich::Links.index()]).abs() < 0.001);
    }

    #[test]
    fn die_lesereihenfolge_nimmt_die_sichtbaren_tabs_zuerst() {
        let modell = modell();
        let links = Tabuebersicht {
            zahl: 2,
            sichtbar: 0,
        };
        let rechts = Tabuebersicht {
            zahl: 2,
            sichtbar: 1,
        };
        assert_eq!(
            modell.lesereihenfolge([links, rechts]),
            [
                (Fensterseite::Links, 0),
                (Fensterseite::Rechts, 1),
                (Fensterseite::Links, 1),
                (Fensterseite::Rechts, 0),
            ],
            "das ist die Pruefsitzung aus C8: je zwei Tabs, je einer sichtbar"
        );
    }

    #[test]
    fn ein_ausgeblendetes_dateifenster_liest_erst_in_der_zweiten_stufe() {
        let mut modell = modell();
        modell.umschalten(Bereich::Rechts);
        let eines = Tabuebersicht {
            zahl: 1,
            sichtbar: 0,
        };
        assert_eq!(
            modell.lesereihenfolge([eines, eines]),
            [(Fensterseite::Links, 0), (Fensterseite::Rechts, 0)]
        );
    }
}
