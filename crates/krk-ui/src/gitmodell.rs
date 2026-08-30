//! Was der Git-Bereich eines Tabs zeigt: der Kopf, der Verlauf, die Auswahl
//! darin und die Zusammenfassung des Status.
//!
//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile, wie in
//! [`crate::fenstermodell`], [`crate::tabs`] und [`crate::vorschaumodell`]
//! daneben. Sie haelt den Stand; die Ansicht dazu ist der Git-Bereich, und der
//! schreibt seine drei Flaechen aus **diesem** Modell und aus keiner zweiten
//! Quelle.
//!
//! # Ein Gitmodell je Tab
//!
//! Es wohnt im [`Tabinhalt`](crate::tabs) neben dem [`Ordnermodell`], und aus
//! demselben Grund: der Git-Bereich folgt dem aktiven Dateifenster (E1 der
//! Runde 23), und dessen sichtbarer Tab bestimmt den Ordner. Ein Modell beim
//! Fenster statt beim Tab muesste bei jedem Tabwechsel neu gefuellt werden und
//! haette den Stand des verlassenen Tabs schon weggeworfen, bevor der Nutzer
//! zurueckwechselt.
//!
//! # Der Kopf ist ein `Option`, und `KeinRepository` ist nicht der Anfang
//!
//! Solange kein Befund da ist, steht **nichts** da (A8): kein Platzhaltertext,
//! kein Fortschrittsanzeiger. [`Kopf::KeinRepository`] waere dafuer die falsche
//! Vorbelegung, denn es ist eine **entschiedene** Antwort und nicht die
//! Abwesenheit einer Antwort — dieselbe Trennung, die
//! [`krk_core::git`] zwischen `Oeffnung::KeinRepository` und
//! `Oeffnung::Unentschieden` zieht. `None` heisst hier deshalb „noch nicht
//! beantwortet", und die Anzeige bleibt in dieser Spanne leer.
//!
//! Dieselbe Regel traegt die Zusammenfassung: sie steht erst, wenn die
//! Markenmeldung da ist, und bis dahin ist sie leer. Branch und Verlauf stehen
//! zu diesem Zeitpunkt schon, denn beide kosten zusammen weniger als eine
//! Statusabfrage ueber einen kleinen Ordner.
//!
//! # Die Woerter kommen aus dem Kern
//!
//! Dieses Modul formt keinen Text selbst. [`krk_core::git::texte`] ist die eine
//! Stelle, an der die Saetze des Git-Bereichs stehen, und zwar dort, weil
//! `krk-ui` kein Bibliotheksziel hat und ein Satz ohne Probe der Satz ist, den
//! die naechste Runde unbemerkt aendert. Die Leseseite hier reicht sie durch
//! und entscheidet allein, **welcher** Satz an welcher Stelle steht.

use krk_core::git::lauf::VERLAUFSSCHRITT;
use krk_core::git::{Commit, Kopf, Marke, texte};

/// Der Stand des Git-Bereichs fuer einen Tab.
#[derive(Debug, Default)]
pub struct Gitmodell {
    /// Worauf HEAD steht, sobald der Lauf es gemeldet hat.
    ///
    /// `None` heisst „noch nicht beantwortet"; der Modulkopf sagt, warum das
    /// nicht dasselbe ist wie [`Kopf::KeinRepository`].
    kopf: Option<Kopf>,
    /// Die geholten Commits, die juengsten zuerst, in der Reihenfolge des
    /// Laufs ueber die Vorfahren von HEAD.
    verlauf: Vec<Commit>,
    /// Die Stelle des ausgewaehlten Commits im Verlauf.
    ///
    /// Ohne Auswahl bleibt die Flaeche der Einzelheiten leer, und es steht kein
    /// Platzhaltertext (C3.5); deshalb waehlt ein eintreffender Verlauf **nicht**
    /// von selbst seine erste Zeile.
    auswahl: Option<usize>,
    /// Ob der letzte Nachschlag weniger als [`VERLAUFSSCHRITT`] Commits
    /// geliefert hat und damit nichts mehr folgt (C4.3).
    erschoepft: bool,
    /// Die Zusammenfassung des Status, sobald die Markenmeldung da war.
    zusammenfassung: Option<String>,
}

impl Gitmodell {
    /// Ein leeres Gitmodell: nichts beantwortet, nichts angezeigt.
    #[must_use]
    pub fn neu() -> Self {
        Self::default()
    }

    /// Setzt das Modell auf den Anfangsstand zurueck (C4.6).
    ///
    /// Der Anlass ist der Ordnerwechsel, und die Zusage dahinter ist, dass die
    /// Nachladehoehe **nicht** ueber zwei Ordner hinweg gehalten wird: der neue
    /// Ordner faengt bei den ersten [`VERLAUFSSCHRITT`] Commits an, gleich wie
    /// weit der Nutzer im vorigen nachgeladen hatte.
    ///
    /// Zurueckgesetzt wird alles und nicht nur der Verlauf. Der Kopf des alten
    /// Ordners stehen zu lassen hiesse, den Branch eines Repositorys anzuzeigen,
    /// in dem der neue Ordner womoeglich gar nicht liegt; und eine
    /// Zusammenfassung ohne ihre Marken zaehlte Eintraege, die es hier nicht
    /// gibt.
    pub fn zuruecksetzen(&mut self) {
        *self = Self::neu();
    }

    /// Traegt ein, worauf HEAD steht.
    pub fn kopf_setzen(&mut self, kopf: Kopf) {
        self.kopf = Some(kopf);
    }

    /// Haengt die gemeldeten Commits an den Verlauf an.
    ///
    /// **Woran das Ende der Liste erkannt wird** (C4.3): die gemeldete Liste
    /// ist kuerzer als [`VERLAUFSSCHRITT`]. Ein eigenes Kennzeichen im Kanal
    /// waere eine zweite Quelle fuer dieselbe Auskunft; der Modulkopf von
    /// [`krk_core::git::lauf`] sagt es an seiner Stelle ebenso.
    pub fn verlauf_anhaengen(&mut self, commits: Vec<Commit>) {
        self.erschoepft = commits.len() < VERLAUFSSCHRITT;
        self.verlauf.extend(commits);
    }

    /// Traegt die Zusammenfassung aus dem Markenbefund ein (A3).
    ///
    /// Die Marken selbst gehen an das [`Ordnermodell`](krk_core::verzeichnis::Ordnermodell),
    /// das sie ueber den Namen den Zeilen zuordnet. Hier bleibt allein der
    /// Satz, den der Git-Bereich darueber schreibt; ihn ein zweites Mal aus
    /// einer gehaltenen Markenliste zu rechnen hiesse, dieselbe Liste an zwei
    /// Stellen zu halten.
    pub fn marken_setzen(&mut self, marken: &[(String, Marke)]) {
        self.zusammenfassung = Some(texte::zusammenfassung(marken));
    }
}

// **Die Leseseite steht in einem eigenen Block, und sie gehoert dem
// Git-Bereich.** `Gitfenster::zeigen` (`crate::appkit`) ist der eine Schreiber
// seiner drei Flaechen und ruft von hier `kopfzeile`, `zusammenfassung`,
// `erschoepft`, `verlaufslaenge`, `verlaufszeile` und `einzelheiten`;
// `letzter_commit` und `erschoepft` ruft daneben `Tabliste::verlauf_nachladen`.
//
// **Drei Stuecke haben nach Schritt 7 keinen Rufer im ausgelieferten Bau:**
// `auswahl`, `auswahl_setzen` und `ausgewaehlter_commit`. Der Git-Bereich haelt
// die Auswahl seiner Liste in seinen eigenen Ivars, weil `zeigen` das Modell
// lesend bekommt und `Tabinhalt::gitmodell` einen Schreiber von aussen
// ausschliesst; welche der beiden Heimaten die richtige ist, ist eine
// Nutzerfrage und als Datensatz gefilt
// (`260831-0120_*_wo-wohnt-die-auswahl-der-verlaufsliste-im-gitfenster-oder-im-gitmodell.md`).
// Bis sie beantwortet ist, traegt dieser Block seine Ausnahme weiter.
//
// Sie steht als `expect` und nicht als `allow`, und das ist ihr Ablaufdatum:
// bekommen die drei ihren Rufer, meldet der Uebersetzer die Erwartung als
// unerfuellt und zwingt zum Entfernen der Zeile. Das `cfg_attr(not(test))`
// davor grenzt sie auf den ausgelieferten Bau ein, denn im Probenbau sind die
// Stellen schon heute gerufen — die Proben unten sind ihre ersten Leser.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "auswahl, auswahl_setzen und ausgewaehlter_commit warten auf 260831-0120"
    )
)]
impl Gitmodell {
    /// Der letzte gehaltene Commit, hinter dem ein Nachschlag ansetzt.
    ///
    /// `None`, solange der Verlauf leer ist: dann gibt es keine Stelle, ab der
    /// nachzuladen waere.
    #[must_use]
    pub fn letzter_commit(&self) -> Option<&Commit> {
        self.verlauf.last()
    }

    /// Ob der Verlauf erschoepft ist und ein Nachschlag nichts mehr braechte
    /// (C4.3).
    #[must_use]
    pub fn erschoepft(&self) -> bool {
        self.erschoepft
    }

    /// Wie viele Zeilen die Verlaufsliste hat.
    #[must_use]
    pub fn verlaufslaenge(&self) -> usize {
        self.verlauf.len()
    }

    /// Die Stelle des ausgewaehlten Commits, falls einer ausgewaehlt ist.
    #[must_use]
    pub fn auswahl(&self) -> Option<usize> {
        self.auswahl
    }

    /// Waehlt den Commit an der genannten Stelle aus.
    ///
    /// Eine Stelle jenseits des Verlaufs raeumt die Auswahl, statt sie auf
    /// einen Commit zu setzen, den es nicht gibt.
    pub fn auswahl_setzen(&mut self, stelle: Option<usize>) {
        self.auswahl = stelle.filter(|stelle| *stelle < self.verlauf.len());
    }

    /// Der ausgewaehlte Commit, falls einer ausgewaehlt ist.
    #[must_use]
    pub fn ausgewaehlter_commit(&self) -> Option<&Commit> {
        self.auswahl.and_then(|stelle| self.verlauf.get(stelle))
    }

    /// Die obere Zeile des Git-Bereichs, oder die leere Zeichenkette, solange
    /// nichts beantwortet ist (A8).
    #[must_use]
    pub fn kopfzeile(&self) -> String {
        self.kopf.as_ref().map(texte::kopfzeile).unwrap_or_default()
    }

    /// Die zweite Zeile des Git-Bereichs.
    ///
    /// **Drei Lagen, und die Reihenfolge ist tragend.** Ein Repository ohne
    /// Commit sagt es hier und nicht in der Kopfzeile (A7): dort steht sein
    /// Branchname, denn den gibt es. Sonst steht die Zusammenfassung des
    /// Status, und solange die Markenmeldung aussteht, steht nichts (A8).
    #[must_use]
    pub fn zusammenfassung(&self) -> &str {
        match self.kopf {
            Some(Kopf::OhneCommit(_)) => texte::OHNE_COMMIT,
            _ => self.zusammenfassung.as_deref().unwrap_or_default(),
        }
    }

    /// Die Zeile der Verlaufsliste an dieser Stelle (A5).
    #[must_use]
    pub fn verlaufszeile(&self, zeile: usize) -> Option<String> {
        self.verlauf.get(zeile).map(texte::verlaufszeile)
    }

    /// Was die Flaeche unter der Liste fuer den Commit an dieser Stelle traegt
    /// (E13, C3.4).
    ///
    /// **Nach der Stelle gefragt und nicht nach der Auswahl**, anders als
    /// [`Self::ausgewaehlter_commit`] daneben: der Git-Bereich haelt die
    /// Auswahl seiner Liste selbst — der Modulkopf von
    /// [`crate::appkit`]`::git` sagt, warum —, und er braucht die Einzelheiten
    /// deshalb zu **jeder** Zeile, nicht zu der einen, die dieses Modell
    /// gewaehlt hat.
    ///
    /// `None` heisst „diese Zeile gibt es nicht", und der Rufer laesst die
    /// Flaeche dann leer (C3.5). Der Text kommt wie die drei anderen aus
    /// [`krk_core::git::texte`] und wird hier nicht geformt.
    #[must_use]
    pub fn einzelheiten(&self, zeile: usize) -> Option<String> {
        self.verlauf.get(zeile).map(texte::einzelheiten)
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    /// Ein Commit, dessen Objektname aus der Nummer entsteht.
    fn commit(nummer: u8) -> Commit {
        let hex = format!("{nummer:02x}").repeat(20);
        Commit {
            id: krk_core::git::ObjectId::from_hex(hex.as_bytes())
                .expect("vierzig Hexziffern sind ein Objektname"),
            kurzbeschreibung: format!("Commit {nummer}"),
            nachricht: format!("Commit {nummer}\n\nmehr dazu"),
            autor: "Wer".to_owned(),
            email: "wer@example.invalid".to_owned(),
            zeit: SystemTime::UNIX_EPOCH,
        }
    }

    /// So viele Commits, wie ein voller Schritt liefert.
    fn voller_schritt() -> Vec<Commit> {
        (0..VERLAUFSSCHRITT)
            .map(|nummer| commit(u8::try_from(nummer % 256).expect("unter 256")))
            .collect()
    }

    /// Vor dem ersten Befund steht nichts da (A8), und `KeinRepository` ist
    /// nicht die Vorbelegung.
    #[test]
    fn ein_frisches_modell_zeigt_nichts() {
        let modell = Gitmodell::neu();
        assert_eq!(modell.kopfzeile(), "");
        assert_eq!(modell.zusammenfassung(), "");
        assert_eq!(modell.verlaufslaenge(), 0);
        assert_eq!(modell.auswahl(), None);
        assert!(!modell.erschoepft());
    }

    /// C4.3: ein voller Schritt laesst offen, ein kuerzerer schliesst ab.
    #[test]
    fn ein_kuerzerer_schwung_erschoepft_den_verlauf() {
        let mut modell = Gitmodell::neu();
        modell.verlauf_anhaengen(voller_schritt());
        assert!(
            !modell.erschoepft(),
            "ein voller Schritt sagt nichts ueber das Ende"
        );
        modell.verlauf_anhaengen(vec![commit(200)]);
        assert!(modell.erschoepft(), "ein kuerzerer Schwung ist der Rest");
        assert_eq!(modell.verlaufslaenge(), VERLAUFSSCHRITT + 1);
    }

    /// C4.6 am Modell: das Zuruecksetzen nimmt Verlauf, Kopf, Auswahl und
    /// Zusammenfassung mit.
    #[test]
    fn das_zuruecksetzen_laesst_nichts_vom_vorigen_ordner_stehen() {
        let mut modell = Gitmodell::neu();
        modell.kopf_setzen(Kopf::Branch("main".to_owned()));
        modell.verlauf_anhaengen(voller_schritt());
        modell.verlauf_anhaengen(voller_schritt());
        modell.auswahl_setzen(Some(3));
        modell.marken_setzen(&[("a.txt".to_owned(), Marke::Geaendert)]);

        modell.zuruecksetzen();

        assert_eq!(modell.verlaufslaenge(), 0, "der Verlauf faengt wieder an");
        assert_eq!(modell.kopfzeile(), "");
        assert_eq!(modell.zusammenfassung(), "");
        assert_eq!(modell.auswahl(), None);
        assert!(!modell.erschoepft());
    }

    /// A7: der ungeborene HEAD nennt seinen Branch oben und „noch kein Commit"
    /// darunter.
    #[test]
    fn ein_repository_ohne_commit_sagt_es_in_der_zweiten_zeile() {
        let mut modell = Gitmodell::neu();
        modell.kopf_setzen(Kopf::OhneCommit("main".to_owned()));
        assert_eq!(modell.kopfzeile(), "main");
        assert_eq!(modell.zusammenfassung(), krk_core::git::texte::OHNE_COMMIT);
    }

    /// Eine Auswahl jenseits des Verlaufs waehlt nichts aus.
    #[test]
    fn eine_auswahl_jenseits_des_verlaufs_bleibt_leer() {
        let mut modell = Gitmodell::neu();
        modell.verlauf_anhaengen(vec![commit(1), commit(2)]);
        modell.auswahl_setzen(Some(5));
        assert_eq!(modell.auswahl(), None);
        assert!(modell.ausgewaehlter_commit().is_none());
        modell.auswahl_setzen(Some(1));
        assert_eq!(
            modell.ausgewaehlter_commit().map(|commit| commit.id),
            Some(commit(2).id)
        );
    }

    /// Die Verlaufszeile kommt aus dem Kern und wird hier nur durchgereicht.
    #[test]
    fn die_verlaufszeile_kommt_aus_dem_kern() {
        let mut modell = Gitmodell::neu();
        modell.verlauf_anhaengen(vec![commit(7)]);
        assert_eq!(
            modell.verlaufszeile(0),
            Some(krk_core::git::texte::verlaufszeile(&commit(7)))
        );
        assert_eq!(modell.verlaufszeile(1), None);
    }
}
