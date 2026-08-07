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
//! # Der Aufschub waehrend eines eigenen Vorgangs
//!
//! Der erste Ausloeser wird ausgesetzt, solange ein eigener Vorgang die
//! gemeldeten Ordner umschreibt — aber nur, wenn seine Operationsart schneller
//! aendert, als ein Lesevorgang fertig wird. Welche Art das ist, entscheidet
//! [`schiebt_auffrischung_auf`] und sonst nichts; [`aufgeschobene_ordner`]
//! legt die Ordner daneben, und [`auffrischung_aufgeschoben`] beantwortet
//! damit den einzelnen gemeldeten Pfad.
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
use krk_core::operation::Art;

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

/// Ob ein Vorgang dieser Art die Auffrischung seiner Ordner aufschiebt.
///
/// **Die eine Stelle, an der "schnell" entschieden wird.** Die
/// Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig: eine
/// sechste Operationsart bricht hier den Bau ab und erzwingt eine bewusste
/// Einordnung, statt still in den Aufschub oder still an ihm vorbei zu laufen.
///
/// **Das Stapel-Umbenennen schiebt auf.** Es schreibt Verzeichniseintraege und
/// sonst nichts; 5.000 davon sind auf dem Referenzgeraet in wenigen hundert
/// Millisekunden durch. Jede einzelne meldet FSEvents, und die naechste Meldung
/// setzt den Lesevorgang neu auf, bevor er seinen ersten Stapel angehaengt hat.
/// Das war der Defekt vom 260805-1337, den [`auffrischung_aufgeschoben`]
/// abfaengt.
///
/// **Die uebrigen vier schieben nicht auf.** Kopieren, Verschieben, Papierkorb
/// und endgueltiges Loeschen melden in gemaechlichem Takt; zwischen zwei
/// Meldungen wird ein Lesevorgang fertig, und der Nutzer sieht einen
/// angezeigten Zielordner sich waehrend des Laufs fuellen statt in einem Schlag
/// am Ende. Bis `fd5e3c5` war das so, danach nicht mehr, und seit der
/// Nutzerentscheidung vom 260806 wieder
/// (`issues/260806-1331_*_der-auffrischungsaufschub-gilt-fuer-alle-fuenf-
/// operationsarten-statt-nur-fuer-die-schnelle.md`).
///
/// **Die Kante ist seit dem 260807 geschlossen, und zwar an der Lesestelle.**
/// Ein Verschieben innerhalb **eines** Datentraegers laeuft ueber `rename(2)`
/// und ist damit so schnell wie ein Stapel-Umbenennen; ueber genuegend
/// Eintraege traegt es dieselbe Meldelawine. Beantwortet ist das nicht durch
/// eine zweite Ausnahme hier, sondern dort, wo die leere Liste entstand:
/// [`krk_core::verzeichnis::Ordnermodell::lesevorgang_beginnen`] ersetzt den
/// Bestand erst mit dem ersten gelieferten Stapel, statt ihn vorab zu leeren.
/// Damit laeuft die Liste bei keiner Operationsart mehr leer, und die
/// Einordnung hier bleibt bei der einen Frage, die sie beantwortet: ob eine
/// Auffrischung waehrend des Vorgangs ueberhaupt lohnt.
pub fn schiebt_auffrischung_auf(art: &Art) -> bool {
    match art {
        Art::UmbenennenImStapel { .. } => true,
        Art::Kopieren { .. }
        | Art::Verschieben { .. }
        | Art::InDenPapierkorb
        | Art::EndgueltigLoeschen => false,
    }
}

/// Die Ordner, deren Auffrischung ein laufender Vorgang aufschiebt.
///
/// Die Ordner des Vorgangs kommen herein und gehen unveraendert wieder hinaus,
/// wenn seine Art nach [`schiebt_auffrischung_auf`] aufschiebt; sonst kommt
/// eine leere Liste zurueck, und [`auffrischung_aufgeschoben`] verneint danach
/// jeden Pfad.
///
/// **Die Aufzaehlung der Ordner selbst gehoert dem Vorgang** und steht in
/// `crate::appkit::anwendung`, wo sie zugleich die Abschlussauffrischung
/// bedient. Hier faellt allein die Entscheidung, ob sie fuer den Aufschub
/// zaehlt; damit steht diese Entscheidung ausserhalb von AppKit und ist ohne
/// Fenster pruefbar.
pub fn aufgeschobene_ordner(art: &Art, ordner_des_vorgangs: Vec<PathBuf>) -> Vec<PathBuf> {
    if schiebt_auffrischung_auf(art) {
        ordner_des_vorgangs
    } else {
        Vec::new()
    }
}

/// Ob die Auffrischung dieses gemeldeten Pfades gerade aufgeschoben ist.
///
/// Die Liste kommt aus [`aufgeschobene_ordner`] und ist in zwei Faellen leer:
/// es laeuft kein eigener Vorgang, oder der laufende schiebt nicht auf. Dann
/// ist die Antwort fuer jeden Pfad "nein", und die Dateisystemwache liest wie
/// ohne Vorgang.
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
pub fn auffrischung_aufgeschoben(pfad: &Path, aufgeschobene_ordner: &[PathBuf]) -> bool {
    aufgeschobene_ordner
        .iter()
        .any(|einer| gleicher_ordner(einer, pfad))
}

/// Holt jeden Tab von einem ausgeworfenen Datenträger herunter (C9).
///
/// **Jeden Tab, nicht nur den sichtbaren.** Bis zum 260805 blieb ein verdeckter
/// Tab auf demselben Datentraeger stehen; wer spaeter auf ihn wechselte, sah
/// eine leere Liste und erfuhr den Grund erst mit dem naechsten Lesevorgang
/// (`issues/260804-1451_*_ein-verdeckter-tab-auf-einem-ausgeworfenen-datentraeger-behaelt-seinen-toten-pfad.md`).
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

    /// Ein Stapel-Umbenennen mit einem Namen; der Inhalt spielt fuer die
    /// Einordnung keine Rolle, die Art tut es.
    fn ein_umbenennen() -> Art {
        Art::UmbenennenImStapel {
            neue_namen: vec!["neu.txt".to_owned()],
        }
    }

    /// Die vier Arten, die nicht aufschieben, in einer Liste; so faellt beim
    /// Hinzukommen einer fuenften auf, dass sie hier fehlt.
    fn die_gemaechlichen() -> [Art; 4] {
        [
            Art::Kopieren {
                ziel: PathBuf::from("/ziel"),
            },
            Art::Verschieben {
                ziel: PathBuf::from("/ziel"),
            },
            Art::InDenPapierkorb,
            Art::EndgueltigLoeschen,
        ]
    }

    /// Die Zuordnung "schnell / nicht schnell" steht an einer Stelle; diese
    /// Pruefung geht sie fuer alle fuenf Operationsarten durch
    /// (`issues/260806-1331_*`).
    #[test]
    fn allein_das_stapel_umbenennen_schiebt_die_auffrischung_auf() {
        assert!(
            schiebt_auffrischung_auf(&ein_umbenennen()),
            "das Stapel-Umbenennen meldet schneller, als ein Lesevorgang fertig wird"
        );
        for art in die_gemaechlichen() {
            assert!(
                !schiebt_auffrischung_auf(&art),
                "{art:?} fuellt seinen angezeigten Ordner waehrend des Laufs"
            );
        }
    }

    /// Die zweite Haelfte derselben Entscheidung: die Ordner eines
    /// gemaechlichen Vorgangs kommen gar nicht erst in die Aufschubliste.
    #[test]
    fn nur_ein_aufschiebender_vorgang_gibt_seine_ordner_in_die_aufschubliste() {
        let ordner = || vec![PathBuf::from("/a"), PathBuf::from("/ziel")];
        assert_eq!(
            aufgeschobene_ordner(&ein_umbenennen(), ordner()),
            ordner(),
            "die Ordner des Stapel-Umbenennens gehen unveraendert durch"
        );
        for art in die_gemaechlichen() {
            assert!(
                aufgeschobene_ordner(&art, ordner()).is_empty(),
                "{art:?} schiebt nichts auf"
            );
        }
    }

    /// Der Aufschub aus dem Defekt vom 260805-1337: der Ordner des laufenden
    /// Stapel-Umbenennens wird erkannt, jeder andere nicht.
    #[test]
    fn der_ordner_eines_aufschiebenden_vorgangs_wird_erkannt() {
        let vorgang = aufgeschobene_ordner(
            &ein_umbenennen(),
            vec![PathBuf::from("/a"), PathBuf::from("/ziel")],
        );
        assert!(auffrischung_aufgeschoben(Path::new("/a"), &vorgang));
        assert!(
            auffrischung_aufgeschoben(Path::new("/a/"), &vorgang),
            "der Schlussstrich macht keinen anderen Ordner"
        );
        assert!(auffrischung_aufgeschoben(Path::new("/ziel"), &vorgang));
        assert!(
            !auffrischung_aufgeschoben(Path::new("/b"), &vorgang),
            "eine fremde Aenderung anderswo frischt weiter auf (C9)"
        );
        assert!(
            !auffrischung_aufgeschoben(Path::new("/a/unterordner"), &vorgang),
            "der Unterordner ist nicht der Ordner des Vorgangs"
        );
    }

    /// Die Gegenrichtung an derselben Kette: eine laufende Kopie in den
    /// angezeigten Zielordner haelt keine Meldung zurueck.
    #[test]
    fn eine_laufende_kopie_haelt_ihren_zielordner_nicht_zurueck() {
        let kopie = Art::Kopieren {
            ziel: PathBuf::from("/ziel"),
        };
        let vorgang =
            aufgeschobene_ordner(&kopie, vec![PathBuf::from("/a"), PathBuf::from("/ziel")]);
        assert!(
            !auffrischung_aufgeschoben(Path::new("/ziel"), &vorgang),
            "der Zielordner fuellt sich waehrend des Laufs"
        );
        assert!(!auffrischung_aufgeschoben(Path::new("/a"), &vorgang));
    }

    #[test]
    fn ohne_laufenden_vorgang_schiebt_nichts_auf() {
        assert!(!auffrischung_aufgeschoben(Path::new("/a"), &[]));
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
