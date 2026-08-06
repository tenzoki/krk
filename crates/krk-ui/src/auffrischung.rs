//! Der eine Auffrischungspfad: welche Dateifenster ein Pfad angeht, und was
//! mit ihnen geschieht.
//!
//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile und
//! kein `unsafe`. Sie haelt die Entscheidung, die Ansicht dazu sind
//! [`crate::appkit::fsevents`] fuer die Beobachtung des Dateisystems und
//! [`crate::appkit::volumes`] fuer die der Datentraeger.
//!
//! # Eine Funktion, zwei Ausloeser
//!
//! ```text
//!  FSEvents-Rueckruf ─────┐
//!                         ├──> ordner_neu_lesen(pfad) ──> Dateifenstersicht::neu_lesen
//!  Abschluss einer   ─────┘                                (der gestueckelte Leser aus S2)
//!  Dateioperation (S16)
//! ```
//!
//! [`ordner_neu_lesen`] ist der **einzige** Weg, auf dem ein Dateifenster
//! seinen Ordner noch einmal liest. Seit S16 steht auch der zweite Ausloeser:
//! der ueber den Fortschrittskanal gemeldete Abschluss einer Dateioperation
//! ruft dieselbe Funktion, in
//! `crate::appkit::anwendung::Anwendungsdelegierter::vorgang_beenden`, einmal
//! fuer den Quellordner und einmal fuer den Zielordner. Ein eigener Weg fuer
//! die selbst verursachte Aenderung waere die Sonderregel mit eigenem
//! Rueckfallweg, die die Maxime "supersimpel" ausschliesst; die erste
//! Abweichung zwischen den beiden Wegen waere ein Fehler ohne Pruefung.
//!
//! Beide Ausloeser liegen in `krk-ui`. `krk-core` ruft die Funktion nicht: die
//! Operationsmaschine aus S15 meldet ihren Abschluss nach oben und weiss
//! nicht, welche Ordner auf dem Schirm stehen.
//!
//! **Zwei Aufrufe sind kein zweiter Weg.** Quelle und Ziel sind zwei Pfade, und
//! `ordner_neu_lesen` nimmt einen. Beim Loeschen und beim Papierkorb gibt es
//! nur den Quellordner, dann bleibt es bei einem Aufruf.
//!
//! # Warum die Pfade nicht bloss verglichen werden
//!
//! FSEvents meldet den Pfad in aufgeloester Form: unter `/tmp` angelegte
//! Dateien kommen als `/private/tmp/...` zurueck, weil `/tmp` eine
//! Verknuepfung ist. Das Dateifenster zeigt dagegen den Pfad, den der Nutzer
//! angesteuert hat. Ein reiner Zeichenvergleich liesse jede Auffrischung unter
//! einem verknuepften Pfad still ausfallen, und "still" ist der Teil, der
//! nicht hinnehmbar ist. [`gleicher_ordner`] vergleicht deshalb erst die
//! geschriebene Form und danach die aufgeloeste. Das sind zwei Vergleiche
//! einer Frage und keine zwei Regeln: der zweite beantwortet dieselbe Frage
//! genauer, wenn der erste sie verneint.

use std::path::{Component, Path, PathBuf};

use krk_core::ablage::Fensterseite;

/// Was der Auffrischungspfad von den beiden Dateifenstern braucht.
///
/// Die Umsetzung steht am Anwendungsdelegierten in
/// [`crate::appkit::anwendung`] und ist dort in jeder Methode eine Zeile. Der
/// Umweg ueber diese Schnittstelle ist der Grund, aus dem die Entscheidung
/// darueber, welches Dateifenster ein Pfad angeht, ohne Fenster pruefbar ist.
pub trait Dateifenstersicht {
    /// Der Ordner, den der sichtbare Tab dieses Dateifensters gerade zeigt.
    fn ordner(&self, seite: Fensterseite) -> PathBuf;

    /// Die Ordner **aller** Tabs dieses Dateifensters, in der Reihenfolge der
    /// Leiste.
    ///
    /// Der Auswurf aus C9 trifft den Tab und nicht das Fenster, deshalb gibt es
    /// diese Frage neben [`Dateifenstersicht::ordner`]. Fuer die Auffrischung
    /// waere sie falsch: was in einem verdeckten Tab steht, sieht niemand, und
    /// ihn bei jeder fremden Aenderung neu zu lesen waere Arbeit fuer einen
    /// leeren Schirm.
    fn tabordner(&self, seite: Fensterseite) -> Vec<PathBuf>;

    /// Die Stelle des sichtbaren Tabs in [`Dateifenstersicht::tabordner`].
    fn sichtbarer_tab(&self, seite: Fensterseite) -> usize;

    /// Ob dieses Dateifenster auf dem Schirm steht (C7).
    fn sichtbar(&self, seite: Fensterseite) -> bool;

    /// Liest den sichtbaren Tab dieses Dateifensters noch einmal.
    ///
    /// Auswahl und Bildlaufposition ueberstehen den Vorgang, soweit die
    /// Eintraege noch existieren.
    fn neu_lesen(&self, seite: Fensterseite);

    /// Laesst den Tab an der genannten Stelle einen anderen Ordner zeigen.
    ///
    /// Ob dabei sofort gelesen wird, entscheidet der Empfaenger: der sichtbare
    /// Tab muss, ein verdeckter liest erst, wenn der Nutzer auf ihn wechselt.
    fn tab_wechseln(&self, seite: Fensterseite, stelle: usize, ziel: &Path);

    /// Stellt einen Text in die Statuszeile dieses Dateifensters (C1).
    fn melden(&self, seite: Fensterseite, text: &str);
}

/// Die Ordner, die gerade auf dem Schirm stehen. Hoechstens zwei.
///
/// Das ist die Liste, die der `FSEventStream` beobachtet. Ein ausgeblendetes
/// Dateifenster kommt nicht vor: was niemand sieht, braucht keine
/// Auffrischung, und C7 laesst das zweite Dateifenster ausblenden.
/// Doppelnennungen fallen weg, weil beide Dateifenster denselben Ordner zeigen
/// duerfen.
pub fn sichtbare_ordner(sicht: &impl Dateifenstersicht) -> Vec<PathBuf> {
    let mut ordner: Vec<PathBuf> = Vec::with_capacity(2);
    for seite in Fensterseite::ALLE {
        if !sicht.sichtbar(seite) {
            continue;
        }
        let dieser = sicht.ordner(seite);
        if !ordner.iter().any(|schon| gleicher_ordner(schon, &dieser)) {
            ordner.push(dieser);
        }
    }
    ordner
}

/// Liest jedes Dateifenster neu, das diesen Ordner zeigt.
///
/// **Der einzige Auffrischungspfad.** Liefert die Zahl der aufgefrischten
/// Dateifenster; null heisst, dass der gemeldete Pfad auf keinem Schirm steht.
/// Der Fall ist gewoehnlich und kein Fehler: FSEvents beobachtet einen Ordner
/// samt allem darunter und meldet auch Aenderungen in einem Unterordner, den
/// kein Dateifenster zeigt.
///
/// Ein ausgeblendetes Dateifenster wird mitgenommen, obwohl der Strom seinen
/// Ordner nicht beobachtet: der zweite Ausloeser aus S16 kann es treffen, und
/// ein Fenster, das beim Wiedereinblenden einen ueberholten Stand zeigt, waere
/// genau die Luecke, die C4 ausschliesst.
pub fn ordner_neu_lesen(sicht: &impl Dateifenstersicht, pfad: &Path) -> usize {
    let mut aufgefrischt = 0;
    for seite in Fensterseite::ALLE {
        if gleicher_ordner(&sicht.ordner(seite), pfad) {
            sicht.neu_lesen(seite);
            aufgefrischt += 1;
        }
    }
    aufgefrischt
}

/// Ob ein gemeldeter Pfad einen Ordner benennt, den ein eigener laufender
/// Vorgang gerade umschreibt.
///
/// **Wozu die Frage gestellt wird.** Ein Stapel-Umbenennen aus C4 laeuft seit
/// S17c auf einem Arbeitsfaden und aendert dabei denselben Ordner, den das
/// Dateifenster zeigt. Jede Umbenennung meldet FSEvents, jede Meldung startete
/// bis zum 260806 einen neuen Lesevorgang, und ein Lesevorgang leert sein
/// Ordnermodell, bevor er den ersten Stapel anhaengt. Bei 5.000 Umbenennungen
/// in wenigen Sekunden setzte die naechste Meldung den Lesevorgang neu auf,
/// bevor er fertig war, und die Liste kam fuer die ganze Laufzeit nicht mehr
/// zum Fuellen
/// (`issues/260805-1337_*_die-dateiliste-ist-waehrend-eines-stapel-umbenennens-
/// im-angezeigten-ordner-leer.md`).
///
/// **Was der Nutzer stattdessen sieht.** Die Liste bleibt auf dem Stand vor dem
/// Vorgang stehen, statt leer zu sein, und der Abschluss frischt sie einmal
/// auf. Der zweite Ausloeser aus S16 ruft [`ordner_neu_lesen`] ohnehin schon,
/// und zwar fuer genau diese Ordner; ein eigener Nachhol-Weg entsteht deshalb
/// nicht.
///
/// **Der Aufschub gilt allein fuer die Ordner des Vorgangs.** Eine fremde
/// Aenderung anderswo frischt weiter ohne Zutun auf, wie C9 es zusagt. Eine
/// fremde Aenderung **in** diesen Ordnern geht nicht verloren, sie erscheint
/// eine Auffrischung spaeter, naemlich mit der des Abschlusses.
pub fn gehoert_zu_vorgang(pfad: &Path, ordner_des_vorgangs: &[PathBuf]) -> bool {
    ordner_des_vorgangs
        .iter()
        .any(|einer| gleicher_ordner(einer, pfad))
}

/// Holt jeden Tab von einem ausgeworfenen Datenträger herunter (C9).
///
/// **Jeden Tab, nicht nur den sichtbaren.** Bis zum 260805 blieb ein verdeckter
/// Tab auf demselben Datentraeger stehen; wer spaeter auf ihn wechselte, sah
/// eine leere Liste und erfuhr den Grund erst mit dem naechsten Lesevorgang
/// (`issues/260804-1451_c_ein-verdeckter-tab-auf-einem-ausgeworfenen-datentraeger-behaelt-seinen-toten-pfad.md`).
/// Ein Pfad, den es nicht mehr gibt, ist in einem verdeckten Tab so tot wie in
/// einem sichtbaren.
///
/// **Die Meldung bleibt eine je Dateifenster.** Sie gehoert der Statuszeile,
/// und die gehoert dem Fenster und nicht dem Tab; C9 formuliert die Zusage
/// ebenso. Sie sagt deshalb, was umgezogen ist: der sichtbare Tab, verdeckte,
/// oder beides. Eine Meldung, die "das Dateifenster zeigt jetzt X" behauptet,
/// waehrend allein ein verdeckter Tab umgezogen ist, waere schlicht falsch.
///
/// Erst der Wechsel, dann die Meldung: der Wechsel loescht die Statuszeile des
/// Dateifensters, weil er ihr die Meldung des neuen Ordners gibt, und eine
/// vorher gesetzte Meldung waere danach weg.
///
/// Liefert die Zahl der betroffenen Dateifenster.
pub fn datentraeger_verloren(
    sicht: &impl Dateifenstersicht,
    datentraeger: &Path,
    name: &str,
    ausweichziel: &Path,
) -> usize {
    let mut betroffen = 0;
    for seite in Fensterseite::ALLE {
        let treffer: Vec<usize> = sicht
            .tabordner(seite)
            .iter()
            .enumerate()
            .filter(|(_, ordner)| liegt_auf(ordner, datentraeger))
            .map(|(stelle, _)| stelle)
            .collect();
        if treffer.is_empty() {
            continue;
        }
        let sichtbarer = sicht.sichtbarer_tab(seite);
        let sichtbar_dabei = treffer.contains(&sichtbarer);
        let verdeckt = treffer.len() - usize::from(sichtbar_dabei);
        for stelle in treffer {
            sicht.tab_wechseln(seite, stelle, ausweichziel);
        }
        sicht.melden(
            seite,
            &auswurfmeldung(name, ausweichziel, sichtbar_dabei, verdeckt),
        );
        betroffen += 1;
    }
    betroffen
}

/// Der Satz, den die Statuszeile nach einem Auswurf traegt.
///
/// Vier Faelle, und jeder sagt genau, was umgezogen ist. Der fuenfte, "nichts
/// umgezogen", kommt nicht vor: [`datentraeger_verloren`] ruft erst, wenn
/// mindestens ein Tab getroffen ist.
fn auswurfmeldung(name: &str, ziel: &Path, sichtbar: bool, verdeckt: usize) -> String {
    let ziel = ziel.display();
    match (sichtbar, verdeckt) {
        (true, 0) => format!("{name} wurde ausgeworfen; das Dateifenster zeigt jetzt {ziel}"),
        (true, 1) => format!(
            "{name} wurde ausgeworfen; das Dateifenster und ein verdeckter Tab zeigen jetzt {ziel}"
        ),
        (true, zahl) => format!(
            "{name} wurde ausgeworfen; das Dateifenster und {zahl} verdeckte Tabs zeigen jetzt \
             {ziel}"
        ),
        (false, 1) => {
            format!("{name} wurde ausgeworfen; ein verdeckter Tab zeigt jetzt {ziel}")
        }
        (false, zahl) => {
            format!("{name} wurde ausgeworfen; {zahl} verdeckte Tabs zeigen jetzt {ziel}")
        }
    }
}

/// Ob zwei Pfade denselben Ordner benennen.
///
/// Zuerst die geschriebene Form ohne Schlussstrich, dann die vom Dateisystem
/// aufgeloeste. Der zweite Vergleich faellt aus, sobald einer der beiden Pfade
/// nicht mehr existiert; dann ist die Antwort die des ersten, und das ist
/// richtig: ein ausgeworfener Datentraeger loest nichts mehr auf.
fn gleicher_ordner(einer: &Path, anderer: &Path) -> bool {
    if ohne_schlussstrich(einer) == ohne_schlussstrich(anderer) {
        return true;
    }
    match (std::fs::canonicalize(einer), std::fs::canonicalize(anderer)) {
        (Ok(einer), Ok(anderer)) => einer == anderer,
        _ => false,
    }
}

/// Ob dieser Ordner auf dem genannten Datentraeger liegt.
///
/// Der Datentraeger selbst zaehlt mit: ein Dateifenster auf `/Volumes/Sicherung`
/// verliert seinen Ordner genauso wie eines auf `/Volumes/Sicherung/Fotos`.
/// Der Vergleich laeuft ueber Pfadbestandteile und nicht ueber Zeichen, damit
/// `/Volumes/Sicherung2` nicht als Teil von `/Volumes/Sicherung` gilt.
fn liegt_auf(ordner: &Path, datentraeger: &Path) -> bool {
    ohne_schlussstrich(ordner).starts_with(ohne_schlussstrich(datentraeger))
}

/// Derselbe Pfad ohne Schlussstrich und ohne `.`-Bestandteile.
///
/// FSEvents meldet Ordner mal mit und mal ohne Schlussstrich; `Path` sieht
/// darin zwei verschiedene Zeichenketten, aber denselben Ordner.
fn ohne_schlussstrich(pfad: &Path) -> PathBuf {
    pfad.components()
        .filter(|teil| !matches!(teil, Component::CurDir))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    /// Ein Paar Dateifenster ohne Fenster darum.
    ///
    /// Jedes Dateifenster traegt eine Tabliste. Im Regelfall hat sie genau
    /// einen Tab, und dann liest sich jede Pruefung wie vor S14;
    /// [`Probe::mit_tabs`] gibt einer Seite mehrere.
    struct Probe {
        tabs: [Vec<PathBuf>; 2],
        sichtbarer_tab: [usize; 2],
        sichtbar: [bool; 2],
        /// Was die Probe erlebt hat, in der Reihenfolge des Geschehens.
        protokoll: RefCell<Vec<String>>,
    }

    impl Probe {
        fn neu(links: &str, rechts: &str) -> Self {
            Self {
                tabs: [vec![PathBuf::from(links)], vec![PathBuf::from(rechts)]],
                sichtbarer_tab: [0, 0],
                sichtbar: [true, true],
                protokoll: RefCell::new(Vec::new()),
            }
        }

        /// Gibt der linken Seite mehrere Tabs; `sichtbar` nennt den sichtbaren.
        fn mit_tabs(mut self, ordner: &[&str], sichtbar: usize) -> Self {
            self.tabs[0] = ordner.iter().map(PathBuf::from).collect();
            self.sichtbarer_tab[0] = sichtbar;
            self
        }

        fn ohne_rechtes(mut self) -> Self {
            self.sichtbar[1] = false;
            self
        }

        fn protokoll(&self) -> Vec<String> {
            self.protokoll.borrow().clone()
        }

        fn notieren(&self, zeile: String) {
            self.protokoll.borrow_mut().push(zeile);
        }
    }

    impl Dateifenstersicht for Probe {
        fn ordner(&self, seite: Fensterseite) -> PathBuf {
            self.tabs[seite.index()][self.sichtbarer_tab[seite.index()]].clone()
        }

        fn tabordner(&self, seite: Fensterseite) -> Vec<PathBuf> {
            self.tabs[seite.index()].clone()
        }

        fn sichtbarer_tab(&self, seite: Fensterseite) -> usize {
            self.sichtbarer_tab[seite.index()]
        }

        fn sichtbar(&self, seite: Fensterseite) -> bool {
            self.sichtbar[seite.index()]
        }

        fn neu_lesen(&self, seite: Fensterseite) {
            self.notieren(format!("neu_lesen {}", seite.index()));
        }

        fn tab_wechseln(&self, seite: Fensterseite, stelle: usize, ziel: &Path) {
            self.notieren(format!(
                "wechseln {} tab {stelle} {}",
                seite.index(),
                ziel.display()
            ));
        }

        fn melden(&self, seite: Fensterseite, text: &str) {
            self.notieren(format!("melden {} {text}", seite.index()));
        }
    }

    #[test]
    fn nur_das_dateifenster_mit_diesem_ordner_liest_neu() {
        let probe = Probe::neu("/a", "/b");
        assert_eq!(ordner_neu_lesen(&probe, Path::new("/a")), 1);
        assert_eq!(probe.protokoll(), ["neu_lesen 0"]);
    }

    #[test]
    fn zeigen_beide_denselben_ordner_lesen_beide_neu() {
        let probe = Probe::neu("/a", "/a");
        assert_eq!(ordner_neu_lesen(&probe, Path::new("/a")), 2);
        assert_eq!(probe.protokoll(), ["neu_lesen 0", "neu_lesen 1"]);
    }

    #[test]
    fn ein_ordner_den_niemand_zeigt_frischt_nichts_auf() {
        let probe = Probe::neu("/a", "/b");
        assert_eq!(ordner_neu_lesen(&probe, Path::new("/a/unterordner")), 0);
        assert!(probe.protokoll().is_empty());
    }

    #[test]
    fn der_schlussstrich_macht_keinen_anderen_ordner() {
        let probe = Probe::neu("/a", "/b");
        assert_eq!(ordner_neu_lesen(&probe, Path::new("/a/")), 1);
    }

    #[test]
    fn ein_verknuepfter_pfad_findet_sein_dateifenster_wieder() {
        // `/tmp` ist auf macOS eine Verknuepfung auf `/private/tmp`, und genau
        // so meldet FSEvents jede Aenderung darunter.
        if !Path::new("/tmp").exists() {
            return;
        }
        let probe = Probe::neu("/tmp", "/b");
        assert_eq!(ordner_neu_lesen(&probe, Path::new("/private/tmp")), 1);
    }

    #[test]
    fn beobachtet_werden_hoechstens_zwei_ordner_und_keiner_doppelt() {
        assert_eq!(
            sichtbare_ordner(&Probe::neu("/a", "/b")),
            [PathBuf::from("/a"), PathBuf::from("/b")]
        );
        assert_eq!(
            sichtbare_ordner(&Probe::neu("/a", "/a")),
            [PathBuf::from("/a")],
            "derselbe Ordner in beiden Fenstern ist ein Pfad"
        );
        assert_eq!(
            sichtbare_ordner(&Probe::neu("/a", "/b").ohne_rechtes()),
            [PathBuf::from("/a")],
            "ein ausgeblendetes Dateifenster wird nicht beobachtet"
        );
    }

    #[test]
    fn ein_ausgeblendetes_dateifenster_frischt_trotzdem_auf() {
        let probe = Probe::neu("/a", "/a").ohne_rechtes();
        assert_eq!(ordner_neu_lesen(&probe, Path::new("/a")), 2);
    }

    /// Der Aufschub aus dem Defekt vom 260805-1337: der Ordner des laufenden
    /// Vorgangs wird erkannt, jeder andere nicht.
    #[test]
    fn der_ordner_eines_laufenden_vorgangs_wird_erkannt() {
        let vorgang = [PathBuf::from("/a"), PathBuf::from("/ziel")];
        assert!(gehoert_zu_vorgang(Path::new("/a"), &vorgang));
        assert!(
            gehoert_zu_vorgang(Path::new("/a/"), &vorgang),
            "der Schlussstrich macht keinen anderen Ordner"
        );
        assert!(gehoert_zu_vorgang(Path::new("/ziel"), &vorgang));
        assert!(
            !gehoert_zu_vorgang(Path::new("/b"), &vorgang),
            "eine fremde Aenderung anderswo frischt weiter auf (C9)"
        );
        assert!(
            !gehoert_zu_vorgang(Path::new("/a/unterordner"), &vorgang),
            "der Unterordner ist nicht der Ordner des Vorgangs"
        );
    }

    #[test]
    fn ohne_laufenden_vorgang_schiebt_nichts_auf() {
        assert!(!gehoert_zu_vorgang(Path::new("/a"), &[]));
    }

    #[test]
    fn ein_ausgeworfener_datentraeger_holt_das_dateifenster_herunter() {
        let probe = Probe::neu("/Volumes/Pruef/Fotos", "/b");
        let betroffen = datentraeger_verloren(
            &probe,
            Path::new("/Volumes/Pruef"),
            "Pruef",
            Path::new("/Users/k"),
        );
        assert_eq!(betroffen, 1);
        assert_eq!(
            probe.protokoll(),
            [
                "wechseln 0 tab 0 /Users/k".to_owned(),
                "melden 0 Pruef wurde ausgeworfen; das Dateifenster zeigt jetzt /Users/k"
                    .to_owned(),
            ],
            "erst der Wechsel, dann die Meldung"
        );
    }

    /// Der Fall, der den Defekt vom 260804-1451 traegt: der sichtbare Tab liegt
    /// woanders, ein verdeckter auf dem ausgeworfenen Datentraeger.
    #[test]
    fn ein_verdeckter_tab_auf_dem_datentraeger_zieht_mit_um() {
        let probe = Probe::neu("/a", "/b").mit_tabs(&["/a", "/Volumes/Pruef/Fotos"], 0);
        let betroffen = datentraeger_verloren(
            &probe,
            Path::new("/Volumes/Pruef"),
            "Pruef",
            Path::new("/Users/k"),
        );
        assert_eq!(betroffen, 1);
        assert_eq!(
            probe.protokoll(),
            [
                "wechseln 0 tab 1 /Users/k".to_owned(),
                "melden 0 Pruef wurde ausgeworfen; ein verdeckter Tab zeigt jetzt /Users/k"
                    .to_owned(),
            ],
            "der verdeckte Tab zieht um, und die Meldung behauptet nichts ueber den sichtbaren"
        );
    }

    #[test]
    fn sichtbarer_und_verdeckte_tabs_ziehen_zusammen_um() {
        let probe = Probe::neu("/a", "/b").mit_tabs(
            &[
                "/Volumes/Pruef",
                "/a",
                "/Volumes/Pruef/Fotos",
                "/Volumes/Pruef/Musik",
            ],
            0,
        );
        assert_eq!(
            datentraeger_verloren(
                &probe,
                Path::new("/Volumes/Pruef"),
                "Pruef",
                Path::new("/Users/k")
            ),
            1
        );
        assert_eq!(
            probe.protokoll(),
            [
                "wechseln 0 tab 0 /Users/k".to_owned(),
                "wechseln 0 tab 2 /Users/k".to_owned(),
                "wechseln 0 tab 3 /Users/k".to_owned(),
                "melden 0 Pruef wurde ausgeworfen; das Dateifenster und 2 verdeckte Tabs zeigen \
                 jetzt /Users/k"
                    .to_owned(),
            ],
            "`/a` bleibt stehen, und die Meldung zaehlt die verdeckten"
        );
    }

    /// Ohne diese Zusage bekaeme ein Dateifenster, dessen Tabs alle woanders
    /// liegen, eine Meldung ueber einen Auswurf, der es nicht betrifft.
    #[test]
    fn ein_dateifenster_ohne_getroffenen_tab_bekommt_keine_meldung() {
        let probe = Probe::neu("/a", "/b").mit_tabs(&["/a", "/c"], 1);
        assert_eq!(
            datentraeger_verloren(
                &probe,
                Path::new("/Volumes/Pruef"),
                "Pruef",
                Path::new("/Users/k")
            ),
            0
        );
        assert!(probe.protokoll().is_empty());
    }

    #[test]
    fn der_datentraeger_selbst_zaehlt_mit() {
        let probe = Probe::neu("/Volumes/Pruef", "/b");
        assert_eq!(
            datentraeger_verloren(
                &probe,
                Path::new("/Volumes/Pruef"),
                "Pruef",
                Path::new("/Users/k")
            ),
            1
        );
    }

    #[test]
    fn ein_namensvetter_des_datentraegers_bleibt_stehen() {
        let probe = Probe::neu("/Volumes/Pruef2", "/Volumes/Pruefung");
        assert_eq!(
            datentraeger_verloren(
                &probe,
                Path::new("/Volumes/Pruef"),
                "Pruef",
                Path::new("/Users/k")
            ),
            0
        );
        assert!(probe.protokoll().is_empty());
    }
}
