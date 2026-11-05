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
//!     Frist um, Frage n offen, n die juengste ──> Satz steht, solange der
//!                      │                          Befehl gilt
//!     Antwort auf die juengste Frage ──────────> Satz faellt
//!     Befehl ueberholt ────────────────────────> Satz faellt, auch ohne Antwort
//! ```
//!
//! **Die Nummer zaehlt je Befehl und nicht ueber alle fuenf.** Ein Pfadsprung
//! auf einer gesunden Platte beantwortet nicht das Loeschen, das auf einer
//! getrennten haengt. Nachgefragt wird allein die juengste Frage eines Befehls,
//! weil ihre Antwort die einzige ist, die noch etwas bewirken kann. **Der
//! Pfadsprung zaehlt je Dateifenster**, denn seine Antwort gilt je Dateifenster:
//! ein Sprung rechts beantwortet nicht den, der links haengt.
//!
//! **Der Satz ueberlebt seinen Befehl nicht** (Nutzerentscheid zum Defekt
//! `261105-2000-der-wartesatz-zaehlt-eigene-nummern-und-bleibt-nach-einem-ueberholten-befehl-stehen.md`).
//! Ein Faden auf einer getrennten Freigabe kommt vielleicht nie zurueck, und auf
//! seine Antwort wartet dann niemand mehr. Ob ein Befehl noch gilt, entscheidet
//! weiter allein der Befehl an seinem eigenen Stand, und zwar mit derselben
//! Frage, die auch seine Antwort besteht:
//!
//! ```text
//! Loeschen      ein neuerer Loeschbefehl, an welchem Ort auch immer; ein
//!               Tabwechsel nicht, denn das Blatt geht danach noch auf
//! F2            ein neuerer F2, ein anderes Dateifenster, ein anderer Tab oder
//!               Ort im Tab, ein anderer Ersthelfer, ein Blatt, ein anderer
//!               Notizordner; die Saetze ueber Angelegtes kommen trotzdem
//! F4 Geheimnis  ein neues Oeffnen oder Neuladen, eine andere gehaltene Datei,
//!               die Quicknote
//! Pfadsprung    in seinem Dateifenster: ein neuer Sprung, ein anderer Tab, ein
//!               anderer Ort im Tab; das andere Dateifenster nicht
//! Lesezeichen   eine neue Auswahl, eine andere gewaehlte Zeile, ein anderes
//!               Dateifenster, ein anderer Tab oder Ort im Tab, ein Blatt
//! ```
//!
//! Dieses Modul kennt diese Staende nicht und bekommt allein das Urteil
//! ([`Plattenwarte::geltung`]); die Fragen haelt `appkit::nebenfaden`. **Gilt ein
//! Befehl wieder**, etwa wenn das Blatt zu ist, das ihn ueberholt hatte, steht
//! der Satz wieder, denn dann wirkt auch seine Antwort wieder.
//!
//! **Eine Antwort beendet das Warten, auch wenn der Befehl sie verwirft**;
//! `beantwortet` kennt die Staende der Befehle gar nicht.
//!
//! **Beim Gedrueckthalten bleibt der Satz stehen.** Eine neue Frage nimmt einen
//! stehenden Satz nicht weg, denn gewartet wird weiter, und erst die Antwort auf
//! die juengste raeumt ihn; die Zeile flackert also nicht. Das gilt auch dann,
//! wenn der neue Befehl den alten dabei selbst ueberholt. War der Satz dagegen
//! schon gefallen, weil der Befehl ueberholt war, laeuft fuer die neue Frage die
//! Frist von vorn. Solange noch kein
//! Satz steht und die Taste schneller wiederholt, als die Frist laeuft, ist bei
//! jedem Ablauf schon eine neuere Frage da, und der Satz kommt eine Frist nach
//! dem letzten Druck.

use std::time::Duration;

use krk_core::ablage::Fensterseite;

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
    /// Der Sprung auf einen eingegebenen Pfad im linken Dateifenster.
    PfadsprungLinks,
    /// Derselbe im rechten: ein Sprung gilt je Dateifenster, also auch sein Stand.
    PfadsprungRechts,
    /// Die Auswahl eines Lesezeichens in der Leiste.
    Lesezeichen,
}

impl Plattenbefehl {
    /// Der Pfadsprung des genannten Dateifensters.
    #[must_use]
    pub const fn pfadsprung(seite: Fensterseite) -> Self {
        match seite {
            Fensterseite::Links => Plattenbefehl::PfadsprungLinks,
            Fensterseite::Rechts => Plattenbefehl::PfadsprungRechts,
        }
    }

    /// Wie viele es gibt: die Laenge von [`Plattenwarte`]s Tafel.
    const ANZAHL: usize = 6;

    /// Die Stelle in der Tafel. Vollstaendig und ohne Auffangzweig: ein weiterer
    /// Befehl haelt hier den Bau an.
    const fn stelle(self) -> usize {
        match self {
            Plattenbefehl::Loeschen => 0,
            Plattenbefehl::Notizordner => 1,
            Plattenbefehl::Geheimnis => 2,
            Plattenbefehl::PfadsprungLinks => 3,
            Plattenbefehl::PfadsprungRechts => 4,
            Plattenbefehl::Lesezeichen => 5,
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

/// Ob der Satz eines Befehls in der Zeile steht: seine Frist ist um, und er gilt
/// noch. Die eine Stelle, die das entscheidet.
#[must_use]
pub const fn satz_steht(frist_um: bool, ueberholt: bool) -> bool {
    frist_um && !ueberholt
}

/// Der Stand eines Befehls.
#[derive(Debug, Clone, Copy, Default)]
struct Fragestand {
    /// Die Nummer der juengsten Frage.
    juengste: u64,
    /// Ob die juengste Frage noch unbeantwortet ist.
    offen: bool,
    /// Ob die Frist abgelaufen ist, ohne dass eine Antwort auf die juengste
    /// Frage kam. In der Zeile steht der Satz erst nach [`satz_steht`].
    gemeldet: bool,
    /// Ob der Befehl beim letzten Nachsehen nicht mehr galt.
    ueberholt: bool,
}

impl Fragestand {
    /// Ob der Satz dieses Befehls in der Zeile steht.
    const fn steht(&self) -> bool {
        satz_steht(self.gemeldet, self.ueberholt)
    }
}

/// Was die fuenf Befehle gerade von der Platte erwarten.
#[derive(Debug, Default)]
pub struct Plattenwarte {
    staende: [Fragestand; Plattenbefehl::ANZAHL],
}

impl Plattenwarte {
    /// Eine neue Frage: vergibt ihre Nummer. Ein stehender Satz bleibt stehen; hat
    /// ein Ueberholen ihn schon genommen, beginnt die Frist von vorn.
    pub fn gestellt(&mut self, befehl: Plattenbefehl) -> u64 {
        let stand = &mut self.staende[befehl.stelle()];
        stand.juengste += 1;
        stand.offen = true;
        stand.gemeldet = stand.steht();
        stand.ueberholt = false;
        stand.juengste
    }

    /// Die Frist der Frage `nummer` ist um. `true` heisst: der Satz steht jetzt
    /// neu, und die Zeile muss nachziehen. Bei einem ueberholten Befehl steht er
    /// nicht, kommt aber, falls der Befehl wieder gilt.
    pub fn frist_um(&mut self, befehl: Plattenbefehl, nummer: u64) -> bool {
        let stand = &mut self.staende[befehl.stelle()];
        if stand.gemeldet || !satz_faellig(stand.offen, nummer == stand.juengste) {
            return false;
        }
        stand.gemeldet = true;
        stand.steht()
    }

    /// Das Urteil des Befehls ueber seine Frage `nummer`: gilt sie noch? `true`
    /// heisst: der Satz ist dadurch gefallen oder wieder da, und die Zeile muss
    /// nachziehen. Das Urteil ueber eine aeltere Frage aendert nichts.
    pub fn geltung(&mut self, befehl: Plattenbefehl, nummer: u64, gilt: bool) -> bool {
        let stand = &mut self.staende[befehl.stelle()];
        if nummer != stand.juengste {
            return false;
        }
        let vorher = stand.steht();
        stand.ueberholt = !gilt;
        vorher != stand.steht()
    }

    /// Die Frage `nummer` ist beantwortet. `true` heisst: der Satz ist jetzt
    /// gefallen, und die Zeile muss nachziehen. Die Antwort auf eine aeltere
    /// Frage aendert nichts, denn auf die juengste wird weiter gewartet.
    pub fn beantwortet(&mut self, befehl: Plattenbefehl, nummer: u64) -> bool {
        let stand = &mut self.staende[befehl.stelle()];
        if nummer != stand.juengste {
            return false;
        }
        let vorher = stand.steht();
        stand.offen = false;
        stand.gemeldet = false;
        stand.ueberholt = false;
        vorher
    }

    /// Ob irgendein Befehl gerade wartet und das sagen darf.
    #[must_use]
    pub fn meldet(&self) -> bool {
        self.staende.iter().any(Fragestand::steht)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLE: [Plattenbefehl; Plattenbefehl::ANZAHL] = [
        Plattenbefehl::Loeschen,
        Plattenbefehl::Notizordner,
        Plattenbefehl::Geheimnis,
        Plattenbefehl::PfadsprungLinks,
        Plattenbefehl::PfadsprungRechts,
        Plattenbefehl::Lesezeichen,
    ];

    /// Der Sprung im linken Dateifenster; die Proben nennen ihn oft.
    const LINKS: Plattenbefehl = Plattenbefehl::pfadsprung(Fensterseite::Links);
    /// Der Sprung im rechten Dateifenster.
    const RECHTS: Plattenbefehl = Plattenbefehl::pfadsprung(Fensterseite::Rechts);

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

    /// Die Tafel, ob der Satz in der Zeile steht, zeilenweise.
    #[test]
    fn der_satz_steht_nur_nach_der_frist_und_solange_der_befehl_gilt() {
        for (frist_um, ueberholt, erwartet) in [
            (true, false, true),
            (true, true, false),
            (false, false, false),
            (false, true, false),
        ] {
            assert_eq!(
                satz_steht(frist_um, ueberholt),
                erwartet,
                "Frist um {frist_um}, ueberholt {ueberholt}"
            );
        }
    }

    /// Der Kern des Entscheids: der Befehl ist ueberholt, die Frage haengt weiter,
    /// und es kommt keine Antwort. Bei jedem Befehl faellt der Satz trotzdem.
    #[test]
    fn ein_ueberholter_befehl_meldet_nichts_auch_ohne_antwort() {
        for befehl in ALLE {
            let mut warte = Plattenwarte::default();
            let nummer = warte.gestellt(befehl);
            assert!(warte.frist_um(befehl, nummer));
            assert!(
                warte.geltung(befehl, nummer, false),
                "die Zeile zieht nicht nach"
            );
            assert!(!warte.meldet(), "{befehl:?} meldet nach dem Ueberholen");
        }
    }

    /// Ueberholt, bevor die Frist um ist: der Ablauf setzt nichts in die Zeile.
    #[test]
    fn die_frist_eines_ueberholten_befehls_setzt_keinen_satz() {
        let mut warte = Plattenwarte::default();
        let nummer = warte.gestellt(Plattenbefehl::Lesezeichen);
        assert!(!warte.geltung(Plattenbefehl::Lesezeichen, nummer, false));
        assert!(!warte.frist_um(Plattenbefehl::Lesezeichen, nummer));
        assert!(!warte.meldet());
    }

    /// Gilt der Befehl wieder, wirkt auch seine Antwort wieder, und der Satz ist
    /// wieder wahr; die verspaetete Antwort raeumt ihn dann.
    #[test]
    fn gilt_der_befehl_wieder_steht_auch_der_satz_wieder() {
        let mut warte = Plattenwarte::default();
        let nummer = warte.gestellt(Plattenbefehl::Lesezeichen);
        assert!(warte.frist_um(Plattenbefehl::Lesezeichen, nummer));
        assert!(warte.geltung(Plattenbefehl::Lesezeichen, nummer, false));
        assert!(warte.geltung(Plattenbefehl::Lesezeichen, nummer, true));
        assert!(warte.meldet());
        assert!(warte.beantwortet(Plattenbefehl::Lesezeichen, nummer));
        assert!(!warte.meldet());
    }

    /// Eine verspaetete Antwort auf einen ueberholten Befehl aendert die Zeile
    /// nicht mehr: der Satz ist schon weg.
    #[test]
    fn die_antwort_eines_ueberholten_befehls_zieht_die_zeile_nicht_nach() {
        let mut warte = Plattenwarte::default();
        let nummer = warte.gestellt(Plattenbefehl::Notizordner);
        assert!(warte.frist_um(Plattenbefehl::Notizordner, nummer));
        assert!(warte.geltung(Plattenbefehl::Notizordner, nummer, false));
        assert!(!warte.beantwortet(Plattenbefehl::Notizordner, nummer));
        assert!(!warte.meldet());
    }

    /// Das Urteil ueber eine aeltere Frage trifft die juengste nicht.
    #[test]
    fn das_urteil_ueber_eine_aeltere_frage_aendert_nichts() {
        let mut warte = Plattenwarte::default();
        let erste = warte.gestellt(Plattenbefehl::Loeschen);
        let zweite = warte.gestellt(Plattenbefehl::Loeschen);
        assert!(warte.frist_um(Plattenbefehl::Loeschen, zweite));
        assert!(!warte.geltung(Plattenbefehl::Loeschen, erste, false));
        assert!(warte.meldet());
    }

    /// Derselbe Befehl noch einmal auf derselben haengenden Stelle: der Satz
    /// bleibt, auch wenn der neue Befehl den alten selbst ueberholt hat, und die
    /// Zeile flackert nicht.
    #[test]
    fn ein_wiederholter_befehl_zeigt_den_satz_weiter() {
        for befehl in ALLE {
            let mut warte = Plattenwarte::default();
            let erste = warte.gestellt(befehl);
            assert!(warte.frist_um(befehl, erste));
            let zweite = warte.gestellt(befehl);
            assert!(warte.meldet(), "{befehl:?}: die Wiederholung nahm den Satz");
            assert!(!warte.geltung(befehl, zweite, true), "die Zeile flackert");
            assert!(warte.meldet());
            assert!(
                !warte.geltung(befehl, erste, false),
                "die alte Frage zaehlt"
            );
            assert!(warte.meldet());
        }
    }

    /// War der Satz schon gefallen, ist die neue Frage eine neue Lage: ihre Frist
    /// laeuft von vorn, und eine schnelle Antwort zeigt gar nichts.
    #[test]
    fn nach_dem_ueberholen_beginnt_die_frist_der_naechsten_frage_von_vorn() {
        let mut warte = Plattenwarte::default();
        let erste = warte.gestellt(Plattenbefehl::Lesezeichen);
        assert!(warte.frist_um(Plattenbefehl::Lesezeichen, erste));
        assert!(warte.geltung(Plattenbefehl::Lesezeichen, erste, false));
        let zweite = warte.gestellt(Plattenbefehl::Lesezeichen);
        assert!(!warte.meldet(), "der gefallene Satz kam ohne Frist zurueck");
        assert!(warte.frist_um(Plattenbefehl::Lesezeichen, zweite));
        assert!(warte.meldet());
    }

    /// Die zwei Dateifenster teilen sich keinen Stand: die Antwort auf den Sprung
    /// rechts nimmt den Satz des haengenden Sprungs links nicht, und ihr
    /// Ueberholen auch nicht.
    #[test]
    fn die_antwort_auf_den_sprung_im_anderen_dateifenster_nimmt_den_satz_nicht() {
        let mut warte = Plattenwarte::default();
        let links = warte.gestellt(LINKS);
        assert!(warte.frist_um(LINKS, links));
        let rechts = warte.gestellt(RECHTS);
        assert!(!warte.beantwortet(RECHTS, rechts));
        assert!(warte.meldet(), "die Antwort rechts nahm den Satz von links");
        let rechts = warte.gestellt(RECHTS);
        assert!(!warte.geltung(RECHTS, rechts, false));
        assert!(
            warte.meldet(),
            "das Ueberholen rechts nahm den Satz von links"
        );
        assert!(warte.beantwortet(LINKS, links));
        assert!(!warte.meldet());
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
        let erste = warte.gestellt(LINKS);
        let zweite = warte.gestellt(LINKS);
        assert!(!warte.frist_um(LINKS, erste));
        assert!(!warte.meldet());
        assert!(warte.frist_um(LINKS, zweite));
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
        let sprung = warte.gestellt(LINKS);
        assert!(!warte.beantwortet(LINKS, sprung));
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
