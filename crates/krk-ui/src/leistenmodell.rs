//! Das Leistenmodell: was in der Lesezeichen- und Geraeteleiste steht, in
//! welcher Reihenfolge, und welche Zeile ausgewaehlt ist (C5).
//!
//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile. Die
//! Ansicht dazu ist [`crate::appkit::leiste`], die aus den Zeilen hier eine
//! `NSTableView` macht; die Datentraeger, die den unteren Teil fuellen, zaehlt
//! [`crate::appkit::volumes`] auf und reicht sie als gewoehnliche Rust-Werte
//! herein.
//!
//! # Zwei Teile, eine Liste
//!
//! ```text
//! ┌─────────────────────┐
//! │ Lesezeichen         │  Ueberschrift, nicht waehlbar
//! │   Projekte          │  aus bookmarks.toml, Reihenfolge = Reihenfolge
//! │   Sicherung (fehlt) │  ungueltig: der Ordner ist fort
//! │ Geräte und Orte     │  Ueberschrift, nicht waehlbar
//! │   k1                │  das Benutzerverzeichnis
//! │   Macintosh HD      │  aus NSFileManager.mountedVolumeURLs…
//! └─────────────────────┘
//! ```
//!
//! C5 verlangt "trennt Lesezeichen sichtbar von Geraeten und Standardorten".
//! Zwei Tabellen untereinander waeren die andere Antwort; sie haetten zwei
//! Auswahlen, zwei Fokuszustaende und zwei Wege fuer den Auf- und den Ab-Pfeil
//! gehabt. Eine Liste mit zwei Ueberschriften hat eine Auswahl, und die
//! Ueberschriften sind Zeilen, die sie ueberspringt.
//!
//! # Woher die Eintraege kommen
//!
//! Die Lesezeichen kommen aus `bookmarks.toml` und gehoeren dem Nutzer; die
//! Liste selbst wohnt in [`krk_core::ablage::Lesezeichenliste`], weil sie
//! abgelegt wird. Die Geraete kommen bei jedem Aufbau frisch vom System und
//! werden nie abgelegt: was eingehaengt ist, weiss das System besser als eine
//! Datei von gestern.

use std::path::{Path, PathBuf};

use krk_core::ablage::{Lesezeichen, Lesezeichenliste, Verschiebung, Ziel};

/// Die Ueberschrift des oberen Teils.
pub const UEBERSCHRIFT_LESEZEICHEN: &str = "Lesezeichen";

/// Die Ueberschrift des unteren Teils.
pub const UEBERSCHRIFT_GERAETE: &str = "Geräte und Orte";

/// Der Zusatz, mit dem ein ungueltiges Lesezeichen in der Leiste steht (C5).
///
/// Ein Wort und keine Farbe allein: die Leiste faerbt die Zeile zusaetzlich
/// grau, aber eine Farbe ist bei Farbfehlsichtigkeit kein Kennzeichen. Dieselbe
/// Ueberlegung wie bei der Markierung aus C2, die seit S16c neben der Farbe
/// eine fette Schrift traegt.
pub const ZUSATZ_UNGUELTIG: &str = " (fehlt)";

/// Ein Ort im unteren Teil der Leiste: ein Geraet oder ein Standardort.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ort {
    /// Der Name, den der Nutzer im Finder sieht.
    pub name: String,
    /// Der Ordner, den die Auswahl oeffnet.
    pub pfad: PathBuf,
}

impl Ort {
    /// Ein Ort aus Name und Pfad.
    pub fn neu(name: impl Into<String>, pfad: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            pfad: pfad.into(),
        }
    }
}

/// Eine Zeile der Leiste.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zeile {
    /// Eine der beiden Ueberschriften. Nicht waehlbar.
    Ueberschrift(Teil),
    /// Ein Lesezeichen an dieser Stelle der [`Lesezeichenliste`].
    Lesezeichen(usize),
    /// Ein Geraet oder Standardort an dieser Stelle der Ortsliste.
    Ort(usize),
}

impl Zeile {
    /// Ob der Nutzer diese Zeile auswaehlen kann.
    pub fn waehlbar(self) -> bool {
        !matches!(self, Zeile::Ueberschrift(_))
    }
}

/// Welcher der beiden Teile der Leiste.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Teil {
    /// Der obere Teil mit den Lesezeichen des Nutzers.
    Lesezeichen,
    /// Der untere Teil mit den Geraeten und Standardorten.
    Geraete,
}

impl Teil {
    /// Die Beschriftung der Ueberschriftszeile.
    pub fn ueberschrift(self) -> &'static str {
        match self {
            Teil::Lesezeichen => UEBERSCHRIFT_LESEZEICHEN,
            Teil::Geraete => UEBERSCHRIFT_GERAETE,
        }
    }
}

/// Was hinter einer ausgewaehlten Zeile steht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Auswahl {
    /// Der Name, wie er in der Leiste steht.
    pub name: String,
    /// Der Ordner, den sie oeffnet.
    pub ordner: PathBuf,
    /// Ob dieser Ordner noch da ist (C5).
    ///
    /// Immer `true` fuer ein Geraet: die Aufzaehlung nennt nur, was gerade
    /// eingehaengt ist.
    pub gueltig: bool,
}

/// Ein Lesezeichen mit dem Zustand seines Ordners.
///
/// **Die eine Stelle, die die Marke setzt.** Bis zum 260807 taten es zwei:
/// `Leistenmodell::lesezeichen_setzen` beim Aufbau der Eintraege und
/// `Leistenmodell::gueltigkeit_pruefen` beim Nachziehen. Beide riefen
/// [`Lesezeichen::gueltig`], beide lieferten dasselbe, und die erste nannte den
/// Namen der zweiten nicht — wer die Pruefung erweitert haette, haette sie
/// uebersehen
/// (`issues/260807-0012_*_vier-anlaesse-pruefen-die-lesezeichengueltigkeit-auf-drei-verschiedenen-wegen.md`).
/// Seither schreibt allein [`Gemerkt::nachpruefen`] das Feld, und
/// [`Gemerkt::neu`] ruft es. Ein `Gemerkt` mit einer Marke, die seinen Ordner
/// nicht kennt, verlaesst diesen Block nicht.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Gemerkt {
    lesezeichen: Lesezeichen,
    gueltig: bool,
}

impl Gemerkt {
    /// Ein Eintrag mit frisch geprueftem Ordner.
    fn neu(lesezeichen: Lesezeichen) -> Self {
        // Die Vorbelegung ist gleichgueltig: die naechste Zeile setzt die
        // Marke, und dazwischen sieht den Eintrag niemand. Sie steht hier,
        // damit `nachpruefen` die einzige Zeile der Kiste bleibt, die
        // `Lesezeichen::gueltig` ruft.
        let mut gemerkt = Self {
            lesezeichen,
            gueltig: false,
        };
        gemerkt.nachpruefen();
        gemerkt
    }

    /// Prueft den Ordner noch einmal.
    ///
    /// Liefert, ob die Marke sich dabei geaendert hat.
    fn nachpruefen(&mut self) -> bool {
        let jetzt = self.lesezeichen.gueltig();
        std::mem::replace(&mut self.gueltig, jetzt) != jetzt
    }
}

/// Der Inhalt der Leiste und ihre Auswahl.
#[derive(Debug, Default)]
pub struct Leistenmodell {
    lesezeichen: Vec<Gemerkt>,
    orte: Vec<Ort>,
    zeilen: Vec<Zeile>,
    /// Die ausgewaehlte Zeile, oder `None`, solange keine gewaehlt ist.
    ///
    /// Sie zeigt immer auf eine waehlbare Zeile: jede Aenderung an den beiden
    /// Listen zieht sie ueber [`Leistenmodell::auswahl_nachziehen`] nach.
    auswahl: Option<usize>,
}

impl Leistenmodell {
    /// Ein leeres Modell mit den beiden Ueberschriften.
    pub fn neu() -> Self {
        let mut modell = Self::default();
        modell.zeilen_bauen();
        modell
    }

    /// Uebernimmt die Lesezeichen und prueft ihre Ordner (C5).
    ///
    /// Der zweite der vier Anlaesse aus [`Leistenmodell::gueltigkeit_pruefen`];
    /// er laeuft ueber [`Gemerkt::neu`] und damit ueber dieselbe Zeile wie die
    /// drei anderen.
    pub fn lesezeichen_setzen(&mut self, liste: &Lesezeichenliste) {
        self.lesezeichen = liste.eintraege.iter().cloned().map(Gemerkt::neu).collect();
        self.zeilen_bauen();
    }

    /// Uebernimmt die Geraete und Standardorte und prueft die Ordner (C5).
    ///
    /// Der erste der vier Anlaesse: die Ortsliste aendert sich genau dann, wenn
    /// ein Datentraeger gekommen oder gegangen ist, und damit aendert sich, was
    /// ein Lesezeichen darauf wert ist. Die Pruefung steht deshalb hier und
    /// nicht beim Aufrufer. Sie dort zu lassen hiesse, jedem kuenftigen
    /// Aufrufer eine Pflicht mitzugeben, die er vergessen kann, und genau das
    /// war der dritte der drei Wege, die der Befund vom 260807 zusammengelegt
    /// hat.
    ///
    /// Der Rueckgabewert der Pruefung wird hier nicht gebraucht: die Ortsliste
    /// hat sich ohnehin geaendert, die Ansicht zeichnet danach in jedem Fall
    /// neu. Er zaehlt allein am vierten Anlass, wo nichts weiter passiert ist.
    pub fn orte_setzen(&mut self, orte: Vec<Ort>) {
        self.orte = orte;
        self.gueltigkeit_pruefen();
        self.zeilen_bauen();
    }

    /// Prueft die Ordner aller Lesezeichen noch einmal (C5).
    ///
    /// Liefert, ob sich dabei etwas geaendert hat; nur dann muss die Ansicht
    /// neu zeichnen.
    ///
    /// Gerufen an vier Anlaessen: wenn ein Datentraeger gekommen oder gegangen
    /// ist ([`Leistenmodell::orte_setzen`]), wenn die Lesezeichen sich aendern
    /// ([`Leistenmodell::lesezeichen_setzen`] ueber [`Gemerkt::neu`]), wenn eine
    /// Dateioperation aus C4 abgeschlossen ist, und **bevor eine Auswahl
    /// gemeldet wird**. Die letzten beiden kommen von aussen und laufen ueber
    /// `crate::appkit::leiste::Leistenquelle::gueltigkeit_nachziehen`; alle vier
    /// enden in [`Gemerkt::nachpruefen`].
    ///
    /// Der letzte traegt die Zusage aus C5: ein Ordner kann verschwinden, ohne
    /// dass ein Datentraeger es tut, und ohne ihn meldete die Leiste einen
    /// Ordner als gueltig, den es nicht mehr gibt. Der dritte haelt die
    /// angezeigte Marke im haeufigsten dieser Faelle aktuell, dem Loeschen in
    /// KRK selbst; er sitzt in
    /// `crate::appkit::anwendung::Anwendungsdelegierter::vorgang_beenden` und
    /// ist dort begruendet.
    ///
    /// Bei jedem Zeichendurchgang zu fragen waere dagegen ein Systemaufruf je
    /// Zeile und Bild, und die Leiste zeichnet oefter, als der Nutzer in ihr
    /// etwas tut. Loescht ein **fremdes** Programm den Ordner, steht die Marke
    /// deshalb weiterhin bis zur naechsten Auswahl falsch; die Zusage aus C5
    /// haelt auch dann, weil die Auswahl den Grund immer meldet.
    pub fn gueltigkeit_pruefen(&mut self) -> bool {
        let mut geaendert = false;
        for gemerkt in &mut self.lesezeichen {
            // `|=` und nicht `||`: jeder Eintrag wird geprueft, auch wenn ein
            // frueherer sich schon geaendert hat.
            geaendert |= gemerkt.nachpruefen();
        }
        geaendert
    }

    /// Die Lesezeichen, wie sie auf die Platte gehoeren.
    pub fn lesezeichenliste(&self) -> Lesezeichenliste {
        Lesezeichenliste::aus(
            self.lesezeichen
                .iter()
                .map(|gemerkt| gemerkt.lesezeichen.clone())
                .collect(),
        )
    }

    /// Alle Zeilen, von oben nach unten.
    pub fn zeilen(&self) -> &[Zeile] {
        &self.zeilen
    }

    /// Die Zeile an dieser Stelle.
    pub fn zeile(&self, stelle: usize) -> Option<Zeile> {
        self.zeilen.get(stelle).copied()
    }

    /// Die Beschriftung einer Zeile, so wie sie in der Leiste steht.
    pub fn beschriftung(&self, stelle: usize) -> Option<String> {
        match self.zeile(stelle)? {
            Zeile::Ueberschrift(teil) => Some(teil.ueberschrift().to_owned()),
            Zeile::Lesezeichen(stelle) => {
                let gemerkt = self.lesezeichen.get(stelle)?;
                Some(match gemerkt.gueltig {
                    true => gemerkt.lesezeichen.name.clone(),
                    false => format!("{}{ZUSATZ_UNGUELTIG}", gemerkt.lesezeichen.name),
                })
            }
            Zeile::Ort(stelle) => Some(self.orte.get(stelle)?.name.clone()),
        }
    }

    /// Ob die Zeile ein ungueltiges Lesezeichen ist (C5).
    ///
    /// Die Leiste faerbt sie danach; der Zusatz im Text kommt aus
    /// [`Leistenmodell::beschriftung`].
    pub fn ungueltig(&self, stelle: usize) -> bool {
        match self.zeile(stelle) {
            Some(Zeile::Lesezeichen(stelle)) => {
                self.lesezeichen.get(stelle).is_some_and(|g| !g.gueltig)
            }
            _ => false,
        }
    }

    /// Die ausgewaehlte Zeile.
    pub fn auswahl(&self) -> Option<usize> {
        self.auswahl
    }

    /// Waehlt die Zeile an dieser Stelle, falls sie waehlbar ist.
    ///
    /// Liefert, ob sich die Auswahl dadurch geaendert hat. Eine Ueberschrift
    /// laesst die Auswahl stehen: sie ist keine, und der Mausklick auf sie soll
    /// die vorige nicht wegnehmen.
    pub fn waehlen(&mut self, stelle: usize) -> bool {
        if !self.zeile(stelle).is_some_and(Zeile::waehlbar) || self.auswahl == Some(stelle) {
            return false;
        }
        self.auswahl = Some(stelle);
        true
    }

    /// Bewegt die Auswahl um eine waehlbare Zeile nach oben oder unten (C5).
    ///
    /// Ueberschriften werden uebersprungen, am Rand haelt die Auswahl an.
    /// Liefert, ob sie sich bewegt hat. Ohne Auswahl faengt sie bei der ersten
    /// waehlbaren Zeile an, gleich in welche Richtung: die Leiste hat gerade
    /// den Fokus bekommen, und der Nutzer erwartet einen sichtbaren Anfang.
    pub fn auswahl_bewegen(&mut self, schritt: isize) -> bool {
        let Some(jetzt) = self.auswahl else {
            let erste = self.naechste_waehlbare(0, 1);
            self.auswahl = erste;
            return erste.is_some();
        };
        let Some(ziel) = jetzt
            .checked_add_signed(schritt)
            .and_then(|von| self.naechste_waehlbare(von, schritt))
        else {
            return false;
        };
        if ziel == jetzt {
            return false;
        }
        self.auswahl = Some(ziel);
        true
    }

    /// Was hinter der ausgewaehlten Zeile steht.
    ///
    /// **Nur eine Ordnermarke liefert eine Auswahl.** [`Auswahl`] traegt einen
    /// Ordner, und eine Textmarke hat keinen; sie liefert deshalb `None`, und
    /// ihre Auswahl bleibt folgenlos. Das ist ein Platzhalter und keine
    /// Festlegung: **S39** teilt die Auswahl nach der Sorte auf und laesst eine
    /// Textmarke ihre Datei im Editor anspringen. Bis **S38** das Anlegen
    /// bringt, entsteht in KRK ohnehin keine Textmarke.
    pub fn gewaehlt(&self) -> Option<Auswahl> {
        match self.zeile(self.auswahl?)? {
            Zeile::Ueberschrift(_) => None,
            Zeile::Lesezeichen(stelle) => {
                let gemerkt = self.lesezeichen.get(stelle)?;
                let Ziel::Ordner { ordner } = &gemerkt.lesezeichen.ziel else {
                    return None;
                };
                Some(Auswahl {
                    name: gemerkt.lesezeichen.name.clone(),
                    ordner: ordner.clone(),
                    gueltig: gemerkt.gueltig,
                })
            }
            Zeile::Ort(stelle) => {
                let ort = self.orte.get(stelle)?;
                Some(Auswahl {
                    name: ort.name.clone(),
                    ordner: ort.pfad.clone(),
                    gueltig: true,
                })
            }
        }
    }

    /// Die Stelle des ausgewaehlten Lesezeichens in der Lesezeichenliste.
    ///
    /// `None`, wenn die Auswahl auf einer Ueberschrift oder einem Geraet steht.
    /// Die vier Befehle, die ein Lesezeichen aendern, wirken dann nicht; sie
    /// melden das nicht, wie der Wirkungsbereich es auch nicht tut.
    pub fn gewaehltes_lesezeichen(&self) -> Option<usize> {
        match self.zeile(self.auswahl?)? {
            Zeile::Lesezeichen(stelle) => Some(stelle),
            _ => None,
        }
    }

    /// Legt ein Lesezeichen auf einen Ordner an und waehlt es aus (C5).
    ///
    /// Nimmt weiterhin einen Ordner und kein [`Ziel`]: das Anlegen einer
    /// Textmarke kommt in **S38** und legt dort fest, woher Datei, Zeile und
    /// Zeileninhalt kommen. Erst dann ist zu entscheiden, ob diese Funktion das
    /// fertige [`Ziel`] entgegennimmt oder ein zweites Gegenstueck bekommt.
    pub fn anlegen(&mut self, name: &str, ordner: &Path) {
        let mut liste = self.lesezeichenliste();
        let stelle = liste.anlegen(
            name,
            Ziel::Ordner {
                ordner: ordner.to_path_buf(),
            },
        );
        self.lesezeichen_setzen(&liste);
        self.auswahl = self.zeile_des_lesezeichens(stelle);
    }

    /// Benennt das ausgewaehlte Lesezeichen um (C5).
    ///
    /// Liefert, ob sich etwas geaendert hat.
    pub fn umbenennen(&mut self, name: &str) -> bool {
        let Some(stelle) = self.gewaehltes_lesezeichen() else {
            return false;
        };
        let mut liste = self.lesezeichenliste();
        if !liste.umbenennen(stelle, name) {
            return false;
        }
        self.lesezeichen_setzen(&liste);
        self.auswahl = self.zeile_des_lesezeichens(stelle);
        true
    }

    /// Loescht das ausgewaehlte Lesezeichen (C5).
    ///
    /// Die Auswahl bleibt auf derselben Zeile stehen und trifft damit den
    /// nachgerueckten Eintrag; war es der letzte Lesezeichen, ruecken die
    /// Ueberschrift und die Geraete nach, und [`Leistenmodell::auswahl_nachziehen`]
    /// setzt sie auf die naechste waehlbare Zeile.
    pub fn loeschen(&mut self) -> bool {
        let Some(stelle) = self.gewaehltes_lesezeichen() else {
            return false;
        };
        let mut liste = self.lesezeichenliste();
        if !liste.loeschen(stelle) {
            return false;
        }
        self.lesezeichen_setzen(&liste);
        true
    }

    /// Schiebt das ausgewaehlte Lesezeichen einen Platz weiter (C5).
    ///
    /// Die Auswahl wandert mit: sie haengt am Lesezeichen und nicht an der
    /// Zeilennummer, sonst schoebe der zweite Tastendruck den naechsten Eintrag.
    pub fn verschieben(&mut self, richtung: Verschiebung) -> bool {
        let Some(stelle) = self.gewaehltes_lesezeichen() else {
            return false;
        };
        let mut liste = self.lesezeichenliste();
        let Some(ziel) = liste.verschieben(stelle, richtung) else {
            return false;
        };
        self.lesezeichen_setzen(&liste);
        self.auswahl = self.zeile_des_lesezeichens(ziel);
        true
    }

    /// Baut die Zeilenliste aus den beiden Teilen neu auf.
    fn zeilen_bauen(&mut self) {
        let mut zeilen = Vec::with_capacity(self.lesezeichen.len() + self.orte.len() + 2);
        zeilen.push(Zeile::Ueberschrift(Teil::Lesezeichen));
        zeilen.extend((0..self.lesezeichen.len()).map(Zeile::Lesezeichen));
        zeilen.push(Zeile::Ueberschrift(Teil::Geraete));
        zeilen.extend((0..self.orte.len()).map(Zeile::Ort));
        self.zeilen = zeilen;
        self.auswahl_nachziehen();
    }

    /// Haelt die Auswahl auf einer waehlbaren Zeile.
    fn auswahl_nachziehen(&mut self) {
        let Some(jetzt) = self.auswahl else {
            return;
        };
        if self.zeile(jetzt).is_some_and(Zeile::waehlbar) {
            return;
        }
        self.auswahl = self
            .naechste_waehlbare(jetzt, 1)
            .or_else(|| self.naechste_waehlbare(jetzt.saturating_sub(1), -1));
    }

    /// Die naechste waehlbare Zeile ab `von`, in Richtung `schritt`.
    fn naechste_waehlbare(&self, von: usize, schritt: isize) -> Option<usize> {
        let mut stelle = von;
        loop {
            match self.zeile(stelle) {
                Some(zeile) if zeile.waehlbar() => return Some(stelle),
                Some(_) => stelle = stelle.checked_add_signed(schritt)?,
                None => return None,
            }
        }
    }

    /// Die Zeile, in der das Lesezeichen an dieser Stelle steht.
    fn zeile_des_lesezeichens(&self, stelle: usize) -> Option<usize> {
        self.zeilen
            .iter()
            .position(|zeile| *zeile == Zeile::Lesezeichen(stelle))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    /// Ein Ordner unter dem Temporaerverzeichnis, der sich selbst abraeumt.
    ///
    /// Dieselbe Form wie `Pruefordner` in `krk-core/tests/verzeichnis.rs`,
    /// `Wegwerfordner` in `krk-bench/src/fixture.rs` und `Planordner` in
    /// `crate::messmodus`: Zweck, Prozesskennung und Laufnummer im Namen, und
    /// das Abraeumen in `Drop`. Zwei Proben hier standen bis zum 260807 auf
    /// festen Namen; zwei gleichzeitige Testlaeufe haetten denselben Ordner
    /// getroffen, und ein Fehlschlag haette ihn stehen gelassen
    /// (`issues/260807-0800_*_zwei-leistenmodell-proben-benutzen-feste-pruefordnernamen-unter-tmp.md`).
    struct Pruefordner {
        pfad: PathBuf,
    }

    impl Pruefordner {
        /// Ein Name, unter dem noch nichts liegt. Angelegt wird er nicht: die
        /// beiden Proben brauchen den Ordner mal vorhanden und mal fehlend.
        fn neu(zweck: &str) -> Self {
            let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
            let pfad = std::env::temp_dir().join(format!(
                "krk-leiste-test-{zweck}-{}-{laufnummer}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&pfad);
            Self { pfad }
        }

        fn pfad(&self) -> &Path {
            &self.pfad
        }

        fn anlegen(&self) {
            std::fs::create_dir_all(&self.pfad).expect("der Pruefordner laesst sich nicht anlegen");
        }

        fn loeschen(&self) {
            std::fs::remove_dir_all(&self.pfad)
                .expect("der Pruefordner laesst sich nicht loeschen");
        }
    }

    impl Drop for Pruefordner {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.pfad);
        }
    }

    static ZAEHLER: AtomicU64 = AtomicU64::new(0);

    fn modell() -> Leistenmodell {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Eins", "/eins"),
            Lesezeichen::neu("Zwei", "/zwei"),
        ]));
        modell.orte_setzen(vec![
            Ort::neu("Benutzer", "/Users/pruefung"),
            Ort::neu("Macintosh HD", "/"),
        ]);
        modell
    }

    fn beschriftungen(modell: &Leistenmodell) -> Vec<String> {
        (0..modell.zeilen().len())
            .filter_map(|stelle| modell.beschriftung(stelle))
            .collect()
    }

    #[test]
    fn die_leiste_trennt_lesezeichen_sichtbar_von_geraeten() {
        let modell = modell();
        assert_eq!(
            beschriftungen(&modell),
            [
                // Die beiden Pruefpfade gibt es nicht, also traegt jeder von
                // ihnen sein Kennzeichen; geprueft wird hier die Reihenfolge
                // und die Trennung, nicht die Gueltigkeit.
                "Lesezeichen",
                "Eins (fehlt)",
                "Zwei (fehlt)",
                "Geräte und Orte",
                "Benutzer",
                "Macintosh HD"
            ]
        );
        assert!(!modell.zeilen()[0].waehlbar());
        assert!(!modell.zeilen()[3].waehlbar());
    }

    #[test]
    fn die_beiden_ueberschriften_stehen_auch_in_einer_leeren_leiste() {
        let modell = Leistenmodell::neu();
        assert_eq!(beschriftungen(&modell), ["Lesezeichen", "Geräte und Orte"]);
        assert_eq!(modell.auswahl(), None);
        assert_eq!(modell.gewaehlt(), None);
    }

    #[test]
    fn der_auf_und_der_ab_pfeil_ueberspringen_die_ueberschriften() {
        let mut modell = modell();
        // Ohne Auswahl faengt jede Bewegung bei der ersten waehlbaren Zeile an.
        assert!(modell.auswahl_bewegen(1));
        assert_eq!(modell.auswahl(), Some(1));
        assert!(modell.auswahl_bewegen(1));
        assert_eq!(modell.auswahl(), Some(2));
        // Zeile 3 ist die zweite Ueberschrift und wird uebersprungen.
        assert!(modell.auswahl_bewegen(1));
        assert_eq!(modell.auswahl(), Some(4));
        assert!(modell.auswahl_bewegen(-1));
        assert_eq!(modell.auswahl(), Some(2));
    }

    #[test]
    fn am_rand_haelt_die_auswahl_an() {
        let mut modell = modell();
        modell.waehlen(1);
        assert!(
            !modell.auswahl_bewegen(-1),
            "oberhalb steht die Ueberschrift"
        );
        assert_eq!(modell.auswahl(), Some(1));
        modell.waehlen(5);
        assert!(!modell.auswahl_bewegen(1), "darunter steht nichts mehr");
        assert_eq!(modell.auswahl(), Some(5));
    }

    #[test]
    fn eine_ueberschrift_laesst_sich_nicht_waehlen() {
        let mut modell = modell();
        modell.waehlen(1);
        assert!(!modell.waehlen(0));
        assert!(!modell.waehlen(3));
        assert_eq!(modell.auswahl(), Some(1));
    }

    #[test]
    fn die_auswahl_nennt_ordner_und_gueltigkeit() {
        let mut modell = modell();
        modell.waehlen(2);
        assert_eq!(
            modell.gewaehlt(),
            Some(Auswahl {
                name: "Zwei".to_owned(),
                ordner: PathBuf::from("/zwei"),
                gueltig: false,
            }),
            "/zwei gibt es nicht, also ist das Lesezeichen ungueltig"
        );
        modell.waehlen(5);
        assert_eq!(
            modell.gewaehlt(),
            Some(Auswahl {
                name: "Macintosh HD".to_owned(),
                ordner: PathBuf::from("/"),
                gueltig: true,
            })
        );
    }

    /// Der Platzhalter aus [`Leistenmodell::gewaehlt`], festgehalten, damit
    /// niemand die Datei einer Textmarke versehentlich als Ordner ausgibt.
    /// **S39 loest die Probe ab**: dort waehlt eine Textmarke ihre Datei im
    /// Editor an, statt folgenlos zu bleiben.
    #[test]
    fn eine_textmarke_liefert_bis_s39_keine_auswahl() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::textstelle(
            "Stelle",
            "/eins/datei.txt",
            7,
            "eine Zeile",
        )]));
        assert!(modell.waehlen(1));
        assert_eq!(modell.gewaehlt(), None);
    }

    #[test]
    fn ein_ungueltiges_lesezeichen_traegt_seinen_zusatz_und_ein_geraet_nicht() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Temp", std::env::temp_dir()),
            Lesezeichen::neu("Fort", "/gibt-es-nicht-krk"),
        ]));
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Temp"));
        assert!(!modell.ungueltig(1));
        assert_eq!(modell.beschriftung(2).as_deref(), Some("Fort (fehlt)"));
        assert!(modell.ungueltig(2));
    }

    #[test]
    fn ein_neues_lesezeichen_steht_unten_und_ist_ausgewaehlt() {
        let mut modell = modell();
        modell.anlegen("Drei", Path::new("/drei"));
        assert_eq!(modell.auswahl(), Some(3));
        assert_eq!(modell.beschriftung(3).as_deref(), Some("Drei (fehlt)"));
        assert_eq!(modell.lesezeichenliste().zahl(), 3);
    }

    #[test]
    fn die_vier_befehle_wirken_nur_auf_einem_lesezeichen() {
        let mut modell = modell();
        // Die Auswahl steht auf einem Geraet.
        modell.waehlen(4);
        assert!(!modell.umbenennen("Neu"));
        assert!(!modell.loeschen());
        assert!(!modell.verschieben(Verschiebung::Hoch));
        assert_eq!(modell.lesezeichenliste().zahl(), 2);
    }

    #[test]
    fn die_auswahl_wandert_mit_dem_verschobenen_lesezeichen() {
        let mut modell = modell();
        modell.waehlen(2);
        assert!(modell.verschieben(Verschiebung::Hoch));
        assert_eq!(modell.auswahl(), Some(1));
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Zwei (fehlt)"));
        assert!(!modell.verschieben(Verschiebung::Hoch), "oben ist Schluss");
    }

    #[test]
    fn nach_dem_loeschen_des_letzten_lesezeichens_steht_die_auswahl_auf_dem_ersten_geraet() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Eins", "/eins",
        )]));
        modell.orte_setzen(vec![Ort::neu("Benutzer", "/Users/pruefung")]);
        modell.waehlen(1);

        assert!(modell.loeschen());

        assert_eq!(modell.lesezeichenliste().zahl(), 0);
        assert_eq!(
            modell.gewaehlt().map(|auswahl| auswahl.name),
            Some("Benutzer".to_owned()),
            "die Auswahl darf nicht auf der Ueberschrift liegenbleiben"
        );
    }

    #[test]
    fn ein_geraet_mehr_laesst_die_auswahl_stehen() {
        let mut modell = modell();
        modell.waehlen(2);
        modell.orte_setzen(vec![
            Ort::neu("Benutzer", "/Users/pruefung"),
            Ort::neu("Macintosh HD", "/"),
            Ort::neu("Sicherung", "/Volumes/Sicherung"),
        ]);
        assert_eq!(
            modell.gewaehlt().map(|auswahl| auswahl.name),
            Some("Zwei".to_owned()),
            "ein eingehaengter Datentraeger verschiebt die Auswahl des Nutzers nicht"
        );
    }

    #[test]
    fn ein_eingehaengter_datentraeger_macht_sein_lesezeichen_gueltig() {
        let ordner = Pruefordner::neu("gueltigkeit");
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Kommt",
            ordner.pfad(),
        )]));
        assert!(modell.ungueltig(1));

        ordner.anlegen();
        modell.gueltigkeit_pruefen();
        assert!(!modell.ungueltig(1));

        ordner.loeschen();
        modell.gueltigkeit_pruefen();
        assert!(modell.ungueltig(1));
    }

    /// Der vierte Anlass aus C5: eine abgeschlossene Dateioperation hat den
    /// Ordner eines Lesezeichens geloescht.
    ///
    /// Die Stelle, die den Anlass ausloest, sitzt in
    /// `crate::appkit::anwendung::Anwendungsdelegierter::vorgang_beenden` und
    /// ist nur ueber AppKit erreichbar. Pruefbar ohne Fenster ist, was sie dort
    /// ruft, und dabei vor allem der **Rueckgabewert**: an ihm allein haengt das
    /// Neuzeichnen der Leiste. Meldete die Pruefung `false`, bliebe die Zeile
    /// schwarz, obwohl der Ordner fort ist.
    #[test]
    fn nach_einer_dateioperation_meldet_die_pruefung_den_geloeschten_ordner() {
        let ordner = Pruefordner::neu("vorgang-beenden");
        ordner.anlegen();
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Sicherung",
            ordner.pfad(),
        )]));
        assert!(!modell.ungueltig(1), "der Ordner steht noch");

        // Was eine Dateioperation aus C4 tut, wenn sie ihn loescht.
        ordner.loeschen();

        assert!(
            modell.gueltigkeit_pruefen(),
            "ohne die gemeldete Aenderung zeichnete die Leiste nicht neu"
        );
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Sicherung (fehlt)"));
        assert!(
            !modell.gueltigkeit_pruefen(),
            "ein zweiter Anlass ohne Aenderung zeichnet die Leiste nicht noch einmal"
        );
    }

    /// Der zweite Anlass setzt die Marke so, wie der vierte sie pruefen wuerde.
    ///
    /// Die Probe haengt allein am **Rueckgabewert**: findet `gueltigkeit_pruefen`
    /// unmittelbar nach `lesezeichen_setzen` etwas zu aendern, dann haben die
    /// beiden Anlaesse verschiedene Vorstellungen davon, was gueltig heisst.
    /// Bis zum 260807 waren es zwei Codestellen, die dasselbe taten; seither
    /// ist es eine, und diese Probe haelt sie darauf fest.
    #[test]
    fn der_aufbau_und_das_nachziehen_kommen_zum_selben_ergebnis() {
        let vorhanden = Pruefordner::neu("aufbau-vorhanden");
        vorhanden.anlegen();
        let fehlend = Pruefordner::neu("aufbau-fehlend");

        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Da", vorhanden.pfad()),
            Lesezeichen::neu("Fort", fehlend.pfad()),
        ]));
        assert!(!modell.ungueltig(1));
        assert!(modell.ungueltig(2));

        assert!(
            !modell.gueltigkeit_pruefen(),
            "der Aufbau hat die Marken schon richtig gesetzt"
        );
    }

    /// Der erste Anlass: die Ortsliste aendert sich, die Marken ziehen mit.
    ///
    /// Die Pruefung stand bis zum 260807 im AppKit-Aufrufer und nicht hier;
    /// diese Probe kommt ohne Fenster aus und haelt sie im Modell fest.
    #[test]
    fn eine_neue_ortsliste_zieht_die_gueltigkeit_nach() {
        let ordner = Pruefordner::neu("orte-setzen");
        ordner.anlegen();
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Sicherung",
            ordner.pfad(),
        )]));
        assert!(!modell.ungueltig(1), "der Datentraeger ist eingehaengt");

        // Was ein Auswurf hinterlaesst: der Ordner ist fort, und das System
        // meldet die kuerzere Ortsliste.
        ordner.loeschen();
        modell.orte_setzen(vec![Ort::neu("Macintosh HD", "/")]);

        assert!(modell.ungueltig(1));
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Sicherung (fehlt)"));
    }
}
