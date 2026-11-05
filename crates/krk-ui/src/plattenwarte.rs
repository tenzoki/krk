//! Wann die Statuszeile sagt, dass ein Befehl auf die Platte wartet.
//!
//! Fuenf Befehle stellen ihre Frage an die Platte auf einem eigenen Faden und
//! tun bis zur Antwort nichts Sichtbares: Loeschen, F2, F4 auf `secrets.txt`,
//! der Pfadsprung und die Auswahl eines Lesezeichens. Auf einer gesunden Platte
//! faellt das nicht auf; auf einer langsamen oder getrennten sieht der Nutzer
//! nach dem Befehl nichts und drueckt noch einmal
//! (`work-packages/261102-1304-dateisystemfragen-vom-hauptfaden-nehmen/issues/261105-1826-keine-laufende-plattenfrage-meldet-sich-und-jeder-weitere-befehl-startet-einen-faden.md`).
//! Der Nutzer hat entschieden: nach einer kurzen Frist ein Satz in der
//! Statuszeile und nicht Stille.
//!
//! Dieses Modul haelt allein die Regel, ohne AppKit, ohne Uhr und ohne
//! Systemaufruf. Den Zeitgeber und den Faden haelt
//! `appkit::nebenfaden::befehl_fragen_lassen`, die Stelle in der Rangfolge
//! `appkit::statuszeile::Rang::Plattenwarte`.
//!
//! # Die Regel
//!
//! ```text
//! Frage gestellt ──> Nummer n
//!                      │
//!     Frist um, Frage n offen, n die juengste ──> Satz steht
//!                      │
//!     Antwort auf die juengste Frage ──────────> Satz faellt
//! ```
//!
//! **Die Nummer zaehlt je Befehl und nicht ueber alle fuenf.** Ein Pfadsprung
//! auf einer gesunden Platte beantwortet nicht das Loeschen, das auf einer
//! getrennten haengt. Nachgefragt wird allein die juengste Frage eines Befehls,
//! weil ihre Antwort die einzige ist, auf die der Nutzer noch wartet.
//!
//! **Ob die Antwort noch gilt, spielt keine Rolle.** Auch eine Antwort, die der
//! Befehl danach verwirft, weil der Nutzer den Tab gewechselt hat, beendet das
//! Warten; `beantwortet` kennt die Staende der Befehle gar nicht.
//!
//! **Beim Gedrueckthalten bleibt der Satz stehen.** Eine neue Frage nimmt einen
//! stehenden Satz nicht weg, denn gewartet wird weiter, und erst die Antwort auf
//! die juengste raeumt ihn; die Zeile flackert also nicht. Solange noch kein
//! Satz steht und die Taste schneller wiederholt, als die Frist laeuft, ist bei
//! jedem Ablauf schon eine neuere Frage da, und der Satz kommt eine Frist nach
//! dem letzten Druck.

use std::time::Duration;

/// Wie lange eine Frage unbeantwortet sein muss, bevor die Statuszeile etwas
/// sagt: eine Sekunde.
///
/// **Der Wert ist gesetzt und nicht gemessen.** Ziel ist, dass die Frist auf
/// einer gesunden Platte nie ausloest. Die Fragen sind klein: das Zaehlen des
/// Loeschumfangs oeffnet hoechstens `umfang::SCHWELLE + 1` Verzeichnisse, die
/// anderen stellen wenige Fragen nach einem Pfad. Eine Sekunde liegt weit ueber
/// dem, was das auf einer lokalen Platte kosten sollte, und ist zugleich die
/// Spanne, nach der ein Nutzer anfaengt, sich zu fragen, ob sein Befehl
/// angekommen ist. Eine Platte, die erst anlaufen muss, oder eine langsame
/// Freigabe loest den Satz aus, und dann stimmt er auch.
pub const FRIST: Duration = Duration::from_secs(1);

/// Die Befehle, die sich melden, wenn ihre Frage an die Platte haengt.
///
/// **Die Fragen im Hintergrund stehen nicht darin** (Geraeteaufzaehlung, Marken
/// der Lesezeichen, Schreibrecht beim Abwurf): sie hat kein Nutzer gestellt, also
/// wartet auch keiner auf sie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plattenbefehl {
    /// Der Loeschbefund vor der Rueckfrage.
    Loeschen,
    /// F2: den Notizordner bereitstellen und aufloesen.
    Notizordner,
    /// F4 auf `secrets.txt`: die Groesse der Datei.
    Geheimnis,
    /// Der Sprung auf einen eingegebenen Pfad, in beiden Dateifenstern.
    Pfadsprung,
    /// Die Auswahl eines Lesezeichens in der Leiste.
    Lesezeichen,
}

impl Plattenbefehl {
    /// Wie viele es gibt: die Laenge von [`Plattenwarte`]s Tafel.
    const ANZAHL: usize = 5;

    /// Die Stelle in der Tafel. Vollstaendig und ohne Auffangzweig: ein weiterer
    /// Befehl haelt hier den Bau an.
    const fn stelle(self) -> usize {
        match self {
            Plattenbefehl::Loeschen => 0,
            Plattenbefehl::Notizordner => 1,
            Plattenbefehl::Geheimnis => 2,
            Plattenbefehl::Pfadsprung => 3,
            Plattenbefehl::Lesezeichen => 4,
        }
    }
}

/// Ist die Frist einer Frage um, sagt dieser Wert, ob jetzt der Satz faellig ist.
///
/// Die Frist selbst ist hier kein Eingang: dass sie um ist, ist das Ereignis, an
/// dem [`Plattenwarte::frist_um`] gerufen wird.
#[must_use]
pub const fn satz_faellig(frage_offen: bool, nummer_ist_die_juengste: bool) -> bool {
    frage_offen && nummer_ist_die_juengste
}

/// Der Stand eines Befehls.
#[derive(Debug, Clone, Copy, Default)]
struct Fragestand {
    /// Die Nummer der juengsten Frage.
    juengste: u64,
    /// Ob die juengste Frage noch unbeantwortet ist.
    offen: bool,
    /// Ob der Satz fuer diesen Befehl steht.
    gemeldet: bool,
}

/// Was die fuenf Befehle gerade von der Platte erwarten.
#[derive(Debug, Default)]
pub struct Plattenwarte {
    staende: [Fragestand; Plattenbefehl::ANZAHL],
}

impl Plattenwarte {
    /// Eine neue Frage: vergibt ihre Nummer. Ein stehender Satz bleibt stehen.
    pub fn gestellt(&mut self, befehl: Plattenbefehl) -> u64 {
        let stand = &mut self.staende[befehl.stelle()];
        stand.juengste += 1;
        stand.offen = true;
        stand.juengste
    }

    /// Die Frist der Frage `nummer` ist um. `true` heisst: der Satz steht jetzt
    /// neu, und die Zeile muss nachziehen.
    pub fn frist_um(&mut self, befehl: Plattenbefehl, nummer: u64) -> bool {
        let stand = &mut self.staende[befehl.stelle()];
        if stand.gemeldet || !satz_faellig(stand.offen, nummer == stand.juengste) {
            return false;
        }
        stand.gemeldet = true;
        true
    }

    /// Die Frage `nummer` ist beantwortet. `true` heisst: der Satz ist jetzt
    /// gefallen, und die Zeile muss nachziehen. Die Antwort auf eine aeltere
    /// Frage aendert nichts, denn auf die juengste wird weiter gewartet.
    pub fn beantwortet(&mut self, befehl: Plattenbefehl, nummer: u64) -> bool {
        let stand = &mut self.staende[befehl.stelle()];
        if nummer != stand.juengste {
            return false;
        }
        stand.offen = false;
        std::mem::take(&mut stand.gemeldet)
    }

    /// Ob irgendein Befehl gerade wartet und das sagen darf.
    #[must_use]
    pub fn meldet(&self) -> bool {
        self.staende.iter().any(|stand| stand.gemeldet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLE: [Plattenbefehl; Plattenbefehl::ANZAHL] = [
        Plattenbefehl::Loeschen,
        Plattenbefehl::Notizordner,
        Plattenbefehl::Geheimnis,
        Plattenbefehl::Pfadsprung,
        Plattenbefehl::Lesezeichen,
    ];

    /// Die Frist bleibt kurz genug, dass der Nutzer nicht noch einmal drueckt,
    /// und lang genug fuer eine gesunde Platte. Beide Grenzen sind gesetzt und
    /// nicht gemessen, wie der Wert selbst.
    #[test]
    fn die_frist_liegt_zwischen_einer_halben_und_zwei_sekunden() {
        assert_eq!(FRIST, Duration::from_secs(1));
        assert!(FRIST >= Duration::from_millis(500) && FRIST <= Duration::from_secs(2));
    }

    /// Die Tafel der Entscheidung, zeilenweise.
    #[test]
    fn der_satz_ist_nur_bei_offener_juengster_frage_faellig() {
        for (offen, juengste, erwartet) in [
            (true, true, true),
            (true, false, false),
            (false, true, false),
            (false, false, false),
        ] {
            assert_eq!(
                satz_faellig(offen, juengste),
                erwartet,
                "offen {offen}, juengste {juengste}"
            );
        }
    }

    /// Eine Frage allein zeigt nichts: vor der Frist steht kein Satz.
    #[test]
    fn vor_der_frist_steht_kein_satz() {
        let mut warte = Plattenwarte::default();
        for befehl in ALLE {
            warte.gestellt(befehl);
        }
        assert!(!warte.meldet());
    }

    /// Nach der Frist steht er, und die Antwort nimmt ihn, bei jedem Befehl.
    #[test]
    fn die_antwort_raeumt_den_satz_den_die_frist_gesetzt_hat() {
        for befehl in ALLE {
            let mut warte = Plattenwarte::default();
            let nummer = warte.gestellt(befehl);
            assert!(warte.frist_um(befehl, nummer));
            assert!(warte.meldet());
            assert!(warte.beantwortet(befehl, nummer));
            assert!(!warte.meldet());
        }
    }

    /// Eine schnelle Antwort: die Frist laeuft ins Leere, und die Zeile zieht
    /// nichts nach.
    #[test]
    fn eine_beantwortete_frage_meldet_sich_nach_der_frist_nicht_mehr() {
        let mut warte = Plattenwarte::default();
        let nummer = warte.gestellt(Plattenbefehl::Loeschen);
        assert!(!warte.beantwortet(Plattenbefehl::Loeschen, nummer));
        assert!(!warte.frist_um(Plattenbefehl::Loeschen, nummer));
        assert!(!warte.meldet());
    }

    /// Die Frist einer ueberholten Frage zeigt nichts, die der juengsten schon.
    #[test]
    fn allein_die_frist_der_juengsten_frage_setzt_den_satz() {
        let mut warte = Plattenwarte::default();
        let erste = warte.gestellt(Plattenbefehl::Pfadsprung);
        let zweite = warte.gestellt(Plattenbefehl::Pfadsprung);
        assert!(!warte.frist_um(Plattenbefehl::Pfadsprung, erste));
        assert!(!warte.meldet());
        assert!(warte.frist_um(Plattenbefehl::Pfadsprung, zweite));
        assert!(warte.meldet());
    }

    /// Das Gedrueckthalten: eine neue Frage nimmt den Satz nicht weg, die Antwort
    /// der alten auch nicht, erst die der juengsten.
    #[test]
    fn beim_gedrueckthalten_faellt_der_satz_erst_mit_der_juengsten_antwort() {
        let mut warte = Plattenwarte::default();
        let erste = warte.gestellt(Plattenbefehl::Loeschen);
        assert!(warte.frist_um(Plattenbefehl::Loeschen, erste));
        let zweite = warte.gestellt(Plattenbefehl::Loeschen);
        assert!(warte.meldet(), "die neue Frage hat den Satz genommen");
        assert!(
            !warte.frist_um(Plattenbefehl::Loeschen, zweite),
            "zweimal gesetzt"
        );
        assert!(!warte.beantwortet(Plattenbefehl::Loeschen, erste));
        assert!(warte.meldet(), "die alte Antwort hat den Satz genommen");
        assert!(warte.beantwortet(Plattenbefehl::Loeschen, zweite));
        assert!(!warte.meldet());
    }

    /// Eine Antwort raeumt allein ihren Befehl; der andere wartet weiter.
    #[test]
    fn die_antwort_eines_befehls_raeumt_nicht_den_satz_eines_anderen() {
        let mut warte = Plattenwarte::default();
        let loeschen = warte.gestellt(Plattenbefehl::Loeschen);
        assert!(warte.frist_um(Plattenbefehl::Loeschen, loeschen));
        let sprung = warte.gestellt(Plattenbefehl::Pfadsprung);
        assert!(!warte.beantwortet(Plattenbefehl::Pfadsprung, sprung));
        assert!(warte.meldet());
        assert!(warte.beantwortet(Plattenbefehl::Loeschen, loeschen));
        assert!(!warte.meldet());
    }

    /// Die Tafel und `stelle` passen zueinander: keine zwei Befehle teilen sich
    /// einen Stand.
    #[test]
    fn jeder_befehl_hat_seinen_eigenen_stand() {
        let mut stellen: Vec<usize> = ALLE.iter().map(|befehl| befehl.stelle()).collect();
        stellen.sort_unstable();
        assert_eq!(stellen, (0..Plattenbefehl::ANZAHL).collect::<Vec<_>>());
    }
}
