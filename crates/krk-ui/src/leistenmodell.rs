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
//! ┌───────────────────────┐
//! │ Lesezeichen           │  Ueberschrift, nicht waehlbar
//! │  📁 Projekte          │  aus bookmarks.toml, Reihenfolge = Reihenfolge
//! │  📄 Die Lesestelle    │  eine Textmarke aus C6, dieselbe Liste
//! │  📁 Sicherung (fehlt) │  ungueltig: der Ordner ist fort
//! │ Geräte und Orte       │  Ueberschrift, nicht waehlbar
//! │  📁 k1                │  das Benutzerverzeichnis
//! │  📁 Macintosh HD      │  aus NSFileManager.mountedVolumeURLs…
//! └───────────────────────┘
//! ```
//!
//! Das Sinnbild vor der Beschriftung kommt aus [`Leistenmodell::sinnbild`] und
//! sagt, **was die Zeile oeffnet**: einen Ordner oder eine Stelle in einer
//! Datei. Welches Systemsinnbild das ist, entscheidet die Ansicht; hier steht
//! die Sorte und kein Bildname.
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

use krk_core::ablage::{Lesezeichen, Lesezeichenliste, Ziel};

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

/// Woran der Nutzer die Sorte einer Zeile erkennt (C6).
///
/// **Eine Regel ueber alle waehlbaren Zeilen, keine Sonderregel fuer
/// Textmarken:** das Sinnbild sagt, was die Zeile oeffnet. Ein Geraet oeffnet
/// einen Ordner und traegt deshalb dasselbe Sinnbild wie ein
/// Ordner-Lesezeichen; eine Ueberschrift oeffnet nichts und traegt keines.
/// Zwei Regeln nebeneinander — eine fuer die Lesezeichen, eine fuer die
/// Geraete — haetten zwei Ausnahmen, sobald der untere Teil einmal etwas
/// anderes als Ordner fuehrt.
///
/// **Ein Sinnbild und keine Farbe.** Eine Farbe allein waere bei
/// Farbfehlsichtigkeit kein Kennzeichen; dieselbe Ueberlegung, die C2 und C5
/// zu zwei Kennzeichen gefuehrt hat und die auch hinter [`ZUSATZ_UNGUELTIG`]
/// steht.
///
/// Welches Systemsinnbild daraus wird, entscheidet `crate::appkit::leiste`:
/// hier steht keine Zeile AppKit und deshalb auch kein Bildname.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sinnbild {
    /// Die Zeile oeffnet einen Ordner: ein Ordner-Lesezeichen (C5), ein Geraet
    /// oder ein Standardort.
    Ordner,
    /// Die Zeile oeffnet eine Stelle in einer Datei (C6).
    Textstelle,
}

/// Was hinter einer ausgewaehlten Zeile steht.
///
/// **Die Sorte steht im [`Ziel`] und nicht daneben** (C6). Ein zweiter
/// Sortenwert hier waere die zweite Wahrheit darueber, was eine Zeile oeffnet,
/// und liefe von dem weg, was in `bookmarks.toml` steht; [`Leistenmodell::sinnbild`]
/// liest dieselbe Eigenschaft fuer die Anzeige. Ein Geraet und ein Standardort
/// tragen [`Ziel::Ordner`], denn genau das oeffnen sie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Auswahl {
    /// Der Name, wie er in der Leiste steht.
    pub name: String,
    /// Was sie oeffnet: einen Ordner (C5) oder eine Stelle in einer Datei (C6).
    pub ziel: Ziel,
    /// Ob das Ziel noch da ist (C5, C6).
    ///
    /// **Ungueltig heisst allein, dass Ordner oder Datei fehlen.** Ob der
    /// gemerkte Zeileninhalt einer Textmarke noch auf seiner Nummer steht,
    /// entscheidet sich beim Sprung und nur dort; die Begruendung steht im
    /// Modulkopf von [`krk_core::ablage::lesezeichen`].
    ///
    /// Immer `true` fuer ein Geraet: die Aufzaehlung nennt nur, was gerade
    /// eingehaengt ist.
    pub gueltig: bool,
}

impl Auswahl {
    /// Der Pfad, an dem das Ziel liegt: der Ordner oder die Datei.
    ///
    /// **Die eine Stelle, die beide Sorten auf einen Pfad bringt.** Der
    /// Aufrufer braucht ihn fuer den Satz ueber ein fehlendes Ziel, und der
    /// lautet fuer beide Sorten gleich: was fehlt, ist ein Eintrag im
    /// Dateisystem, und welcher Art er waere, sagt der Satz nicht. Wer die
    /// Sorte braucht, sieht auf [`Self::ziel`] und bekommt dort eine
    /// vollstaendige Fallunterscheidung.
    pub fn pfad(&self) -> &Path {
        match &self.ziel {
            Ziel::Ordner { ordner } => ordner,
            Ziel::Textstelle { datei, .. } => datei,
        }
    }
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

    /// Das Sinnbild, das vor der Beschriftung steht (C6).
    ///
    /// `None` fuer eine Ueberschrift: sie oeffnet nichts. Die
    /// Fallunterscheidung ueber [`Ziel`] ist vollstaendig und hat keinen
    /// Auffangzweig; eine dritte Sorte hielte den Bau an und erzwaenge die
    /// Antwort, woran der Nutzer sie erkennt.
    ///
    /// **Sie stellt keine Frage an das Dateisystem.** Die Sorte steht im
    /// Eintrag; ob sein Ziel noch da ist, beantwortet
    /// [`Leistenmodell::ungueltig`] aus der Marke, die
    /// [`Gemerkt::nachpruefen`] gesetzt hat.
    pub fn sinnbild(&self, stelle: usize) -> Option<Sinnbild> {
        match self.zeile(stelle)? {
            Zeile::Ueberschrift(_) => None,
            Zeile::Lesezeichen(stelle) => {
                Some(match self.lesezeichen.get(stelle)?.lesezeichen.ziel {
                    Ziel::Ordner { .. } => Sinnbild::Ordner,
                    Ziel::Textstelle { .. } => Sinnbild::Textstelle,
                })
            }
            Zeile::Ort(stelle) => {
                self.orte.get(stelle)?;
                Some(Sinnbild::Ordner)
            }
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
    /// **Beide Sorten liefern eine Auswahl** (C6). Die Fallunterscheidung
    /// steht nicht hier, sondern beim Aufrufer: dieses Modell sagt, **was** die
    /// Zeile bezeichnet, und `crate::appkit::anwendung` entscheidet, was daraus
    /// folgt — ein Ordner im aktiven Dateifenster oder eine Datei im Editor.
    ///
    /// Bis S39 lieferte eine Textmarke hier `None`, weil [`Auswahl`] einen
    /// Ordner trug und eine Textmarke keinen hat; ihre Auswahl blieb damit
    /// folgenlos. Der Platzhalter ist gefallen, seit `Auswahl` das [`Ziel`]
    /// selbst traegt.
    ///
    /// `None` bleibt fuer eine Ueberschrift: sie oeffnet nichts, wie das
    /// fehlende Sinnbild in [`Self::sinnbild`] es schon sagt.
    pub fn gewaehlt(&self) -> Option<Auswahl> {
        match self.zeile(self.auswahl?)? {
            Zeile::Ueberschrift(_) => None,
            Zeile::Lesezeichen(stelle) => {
                let gemerkt = self.lesezeichen.get(stelle)?;
                Some(Auswahl {
                    name: gemerkt.lesezeichen.name.clone(),
                    ziel: gemerkt.lesezeichen.ziel.clone(),
                    gueltig: gemerkt.gueltig,
                })
            }
            Zeile::Ort(stelle) => {
                let ort = self.orte.get(stelle)?;
                Some(Auswahl {
                    name: ort.name.clone(),
                    ziel: Ziel::Ordner {
                        ordner: ort.pfad.clone(),
                    },
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

    /// Das ausgewaehlte Lesezeichen als Wert, falls eines ausgewaehlt ist (C5).
    ///
    /// **Der Eintrag und nicht seine Stelle**, und darauf beruht seit der
    /// Runde 7 die Zusage aus C3.8. Die drei Befehle, die ein vorhandenes
    /// Lesezeichen aendern, nennen ihr Ziel damit als Wert; eine Stelle waere
    /// eine Zahl in **dieser** Liste, und geschrieben wird auf die frisch von
    /// der Platte gelesene, in der an derselben Stelle ein anderes stehen kann.
    /// Der Modulkopf von [`krk_core::ablage::lesezeichen`] schreibt es aus.
    pub fn gewaehltes_lesezeichen_wert(&self) -> Option<Lesezeichen> {
        let stelle = self.gewaehltes_lesezeichen()?;
        self.lesezeichen
            .get(stelle)
            .map(|gemerkt| gemerkt.lesezeichen.clone())
    }

    /// Uebernimmt eine gelesene Lesezeichenliste und stellt die Auswahl (C5).
    ///
    /// Der eine Weg, auf dem eine geaenderte Liste in die Leiste kommt. Bis zur
    /// Runde 7 rechneten hier vier Methoden die neue Liste selbst aus und
    /// schrieben sie danach; seither entsteht sie unter der Schreibsperre aus
    /// der Datei, und die Leiste uebernimmt das Ergebnis.
    ///
    /// `stelle` ist der betroffene Eintrag: die Auswahl haengt am Lesezeichen
    /// und nicht an der Zeilennummer, sonst schoebe der zweite Tastendruck den
    /// naechsten Eintrag.
    ///
    /// **Zwei Faelle lassen die Auswahl, wo sie ist**, und in beiden hat
    /// `lesezeichen_setzen` sie ueber `auswahl_nachziehen` schon auf eine
    /// waehlbare Zeile gestellt:
    ///
    /// - `None`, also kein betroffener Eintrag.
    /// - Eine Stelle, die die neue Liste nicht mehr fuehrt. Das ist der Fall
    ///   nach dem Loeschen des letzten Lesezeichens: die Zeile darunter ist die
    ///   Ueberschrift der Geraete, und `auswahl_nachziehen` geht von dort auf
    ///   das erste Geraet. Nach dem Loeschen eines Eintrags in der Mitte gibt es
    ///   die Stelle dagegen weiter, und sie trifft den nachgerueckten Eintrag —
    ///   dieselbe Antwort, die die Auswahl in der Dateiliste gibt.
    pub fn uebernehmen(&mut self, liste: &Lesezeichenliste, stelle: Option<usize>) {
        self.lesezeichen_setzen(liste);
        if let Some(zeile) = stelle.and_then(|stelle| self.zeile_des_lesezeichens(stelle)) {
            self.auswahl = Some(zeile);
        }
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
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use krk_core::ablage::{Aenderung, Ausgang, Verschiebung};

    use super::*;
    use crate::pruefordner::Pruefordner;

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

    fn sinnbilder(modell: &Leistenmodell) -> Vec<Option<Sinnbild>> {
        (0..modell.zeilen().len())
            .map(|stelle| modell.sinnbild(stelle))
            .collect()
    }

    /// Fuehrt eine Aenderung an den Lesezeichen so aus, wie der Delegierte es
    /// tut: rechnen und das Ergebnis samt betroffener Stelle uebernehmen.
    ///
    /// Seit der Runde 7 rechnet das Modell nicht mehr selbst; die geaenderte
    /// Liste kommt unter der Schreibsperre von der Platte. Die Proben nehmen
    /// denselben Weg, nur ohne Datei, und pruefen damit weiter, was hier zu
    /// pruefen ist: wo die Auswahl danach steht.
    fn aendern(modell: &mut Leistenmodell, aenderung: &Aenderung) -> Ausgang {
        let mut liste = modell.lesezeichenliste();
        let ausgang = liste.anwenden(aenderung);
        let stelle = match ausgang {
            Ausgang::Geaendert(stelle) => Some(stelle),
            Ausgang::Unveraendert | Ausgang::Verschwunden => None,
        };
        modell.uebernehmen(&liste, stelle);
        ausgang
    }

    /// Aendert das ausgewaehlte Lesezeichen, oder meldet, dass keines gewaehlt
    /// ist.
    ///
    /// Die drei Befehle, die ein vorhandenes Lesezeichen treffen, nennen ihr
    /// Ziel als Eintrag. Steht die Auswahl auf einer Ueberschrift oder einem
    /// Geraet, gibt es keinen, und der Befehl wirkt nicht.
    fn gewaehltes_aendern(
        modell: &mut Leistenmodell,
        bauen: impl FnOnce(Lesezeichen) -> Aenderung,
    ) -> Option<Ausgang> {
        let welches = modell.gewaehltes_lesezeichen_wert()?;
        Some(aendern(modell, &bauen(welches)))
    }

    /// Ein Ordnerziel aus einem Pfad, damit die Proben kurz bleiben.
    ///
    /// Dasselbe Gegenstueck wie in `krk_core::ablage::lesezeichen`.
    fn ordnerziel(pfad: impl Into<PathBuf>) -> Ziel {
        Ziel::Ordner {
            ordner: pfad.into(),
        }
    }

    /// Ein Textstellenziel, das auf eine Datei zeigt, die es nicht gibt.
    fn textziel(datei: impl Into<PathBuf>, zeile: u32, zeileninhalt: &str) -> Ziel {
        Ziel::Textstelle {
            datei: datei.into(),
            zeile,
            zeileninhalt: zeileninhalt.to_owned(),
        }
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
                ziel: Ziel::Ordner {
                    ordner: PathBuf::from("/zwei")
                },
                gueltig: false,
            }),
            "/zwei gibt es nicht, also ist das Lesezeichen ungueltig"
        );
        modell.waehlen(5);
        assert_eq!(
            modell.gewaehlt(),
            Some(Auswahl {
                name: "Macintosh HD".to_owned(),
                ziel: Ziel::Ordner {
                    ordner: PathBuf::from("/")
                },
                gueltig: true,
            })
        );
    }

    /// Die Ablösung des Platzhalters aus [`Leistenmodell::gewaehlt`] (S39): eine
    /// Textmarke liefert eine Auswahl, und die traegt ihr [`Ziel`] mitsamt
    /// Zeilennummer und gemerktem Inhalt. Bis S39 lieferte sie `None` und blieb
    /// damit folgenlos.
    #[test]
    fn eine_textmarke_liefert_ihre_stelle_und_nicht_nichts() {
        let ordner = Pruefordner::nur_name("markenauswahl");
        ordner.anlegen();
        let datei = ordner.datei("quelle.rs", "eine Zeile\n");

        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::textstelle(
            "Stelle",
            &datei,
            7,
            "eine Zeile",
        )]));
        assert!(modell.waehlen(1));
        assert_eq!(
            modell.gewaehlt(),
            Some(Auswahl {
                name: "Stelle".to_owned(),
                ziel: Ziel::Textstelle {
                    datei: datei.clone(),
                    zeile: 7,
                    zeileninhalt: "eine Zeile".to_owned(),
                },
                gueltig: true,
            })
        );
        assert_eq!(
            modell.gewaehlt().as_ref().map(Auswahl::pfad),
            Some(datei.as_path()),
            "der Pfad einer Textmarke ist ihre Datei"
        );
    }

    /// Das zehnte Abnahmekriterium von C6: fehlt die Datei, ist die Auswahl
    /// ungueltig und nennt den Pfad, den der Aufrufer meldet — dieselbe Auskunft
    /// wie bei einer Ordnermarke auf einen verschwundenen Ordner.
    #[test]
    fn eine_textmarke_auf_eine_fehlende_datei_liefert_eine_ungueltige_auswahl() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::textstelle(
            "Fort",
            "/gibt/es/nicht/datei.txt",
            3,
            "eine Zeile",
        )]));
        assert!(modell.waehlen(1));
        let auswahl = modell
            .gewaehlt()
            .expect("die Textmarke liefert eine Auswahl");
        assert!(!auswahl.gueltig);
        assert_eq!(auswahl.pfad(), Path::new("/gibt/es/nicht/datei.txt"));
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
        assert_eq!(
            aendern(
                &mut modell,
                &Aenderung::Anlegen {
                    name: "Drei".to_owned(),
                    ziel: ordnerziel("/drei"),
                },
            ),
            Ausgang::Geaendert(2),
            "ein neues Lesezeichen haengt unten an"
        );
        assert_eq!(modell.auswahl(), Some(3));
        assert_eq!(modell.beschriftung(3).as_deref(), Some("Drei (fehlt)"));
        assert_eq!(modell.lesezeichenliste().zahl(), 3);
    }

    /// Beide Sorten gehen durch dieselbe Tuer und stehen in einer Ordnung (C6).
    ///
    /// Die Probe zu [`Aenderung::Anlegen`] nach S38: der Vorgang traegt das
    /// fertige [`Ziel`], und die angelegte Textmarke haengt unten an wie eine
    /// Ordnermarke — keine eigene Ordnung, keine Sortierung nach Sorte.
    #[test]
    fn eine_angelegte_textmarke_haengt_unten_an_und_ist_ausgewaehlt() {
        let mut modell = modell();
        assert_eq!(
            aendern(
                &mut modell,
                &Aenderung::Anlegen {
                    name: "Die Lesestelle".to_owned(),
                    ziel: textziel("/eins/leser.rs", 118, "let x = 1;"),
                },
            ),
            Ausgang::Geaendert(2),
            "ein neues Lesezeichen haengt unten an"
        );
        assert_eq!(modell.auswahl(), Some(3));
        assert_eq!(modell.sinnbild(3), Some(Sinnbild::Textstelle));
        assert_eq!(
            modell.beschriftung(3).as_deref(),
            Some("Die Lesestelle (fehlt)")
        );
        assert_eq!(modell.lesezeichenliste().zahl(), 3);
    }

    #[test]
    fn die_vier_befehle_wirken_nur_auf_einem_lesezeichen() {
        let mut modell = modell();
        // Die Auswahl steht auf einem Geraet: es gibt kein Ziel, und die drei
        // Befehle, die eines brauchen, kommen gar nicht erst zum Rechnen.
        modell.waehlen(4);
        assert!(modell.gewaehltes_lesezeichen_wert().is_none());
        assert!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Umbenennen {
                welches,
                name: "Neu".to_owned(),
            })
            .is_none()
        );
        assert!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Loeschen { welches }).is_none()
        );
        assert!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Verschieben {
                welches,
                richtung: Verschiebung::Hoch,
            })
            .is_none()
        );
        assert_eq!(modell.lesezeichenliste().zahl(), 2);
    }

    /// Eine Liste, eine Ordnung, zwei Sinnbilder (C6).
    ///
    /// Die Reihenfolge ist die des Anlegens; nach Sorte sortiert wird nicht,
    /// und eine getrennte Ordnung fuer Textmarken gibt es nicht. Der Modulkopf
    /// von `krk_core::ablage::lesezeichen` begruendet es damit, dass zwei
    /// Ordnungen zwei Wahrheiten waeren.
    #[test]
    fn eine_gemischte_liste_behaelt_ihre_reihenfolge_und_zeigt_beide_sorten() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Projekte", "/p"),
            Lesezeichen::textstelle("Die Lesestelle", "/p/leser.rs", 118, "let x = 1;"),
            Lesezeichen::neu("Sicherung", "/s"),
        ]));
        modell.orte_setzen(vec![Ort::neu("Macintosh HD", "/")]);

        assert_eq!(
            beschriftungen(&modell),
            [
                "Lesezeichen",
                "Projekte (fehlt)",
                "Die Lesestelle (fehlt)",
                "Sicherung (fehlt)",
                "Geräte und Orte",
                "Macintosh HD",
            ]
        );
        assert_eq!(
            sinnbilder(&modell),
            [
                None,
                Some(Sinnbild::Ordner),
                Some(Sinnbild::Textstelle),
                Some(Sinnbild::Ordner),
                None,
                Some(Sinnbild::Ordner),
            ],
            "eine Ueberschrift oeffnet nichts, ein Geraet einen Ordner"
        );
    }

    /// Ungueltig heisst fuer eine Textmarke allein, dass die Datei fehlt (C6).
    ///
    /// Der tragende Teil der Antwort vom 260808-0017: eine Marke, deren
    /// gemerkter Zeileninhalt nicht mehr auf der gemerkten Nummer steht, bleibt
    /// gueltig und bleibt ohne Kennzeichen. Ob der Inhalt noch stimmt,
    /// entscheidet sich beim Sprung und nur dort.
    #[test]
    fn eine_textmarke_ist_ungueltig_wenn_die_datei_fehlt_und_sonst_nie() {
        let ordner = Pruefordner::nur_name("textmarke-gueltigkeit");
        ordner.anlegen();
        let datei = ordner.datei("leser.rs", "fn eins() {}\nfn zwei() {}\n");

        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::textstelle("Verschoben", &datei, 2, "fn ganz_anders() {}"),
            Lesezeichen::textstelle("Fort", ordner.pfad().join("fehlt.rs"), 1, "fn eins() {}"),
        ]));

        assert!(
            !modell.ungueltig(1),
            "der gemerkte Inhalt steht nicht mehr dort, die Datei aber schon"
        );
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Verschoben"));
        assert!(modell.ungueltig(2));
        assert_eq!(modell.beschriftung(2).as_deref(), Some("Fort (fehlt)"));
    }

    /// Die vier Lesezeichenbefehle aus C5 wirken auf eine Textmarke wie auf
    /// eine Ordnermarke (C6).
    ///
    /// Ohne eigenen Bau: [`Lesezeichenliste`] fragt an keiner Stelle nach der
    /// Sorte, und diese Probe haelt es an der Leiste fest.
    #[test]
    fn die_vier_lesezeichenbefehle_wirken_auf_eine_textmarke() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Projekte", "/p"),
            Lesezeichen::textstelle("Stelle", "/p/leser.rs", 118, "let x = 1;"),
        ]));

        assert_eq!(
            aendern(
                &mut modell,
                &Aenderung::Anlegen {
                    name: "Zweite Stelle".to_owned(),
                    ziel: textziel("/p/schreiber.rs", 7, "fn sieben() {}"),
                },
            ),
            Ausgang::Geaendert(2),
            "ein neues Lesezeichen haengt unten an"
        );
        assert_eq!(modell.auswahl(), Some(3));

        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Umbenennen {
                welches,
                name: "Die zweite Stelle".to_owned(),
            }),
            Some(Ausgang::Geaendert(2))
        );
        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Verschieben {
                welches,
                richtung: Verschiebung::Hoch,
            }),
            Some(Ausgang::Geaendert(1))
        );
        assert_eq!(modell.auswahl(), Some(2), "die Auswahl wandert mit");
        assert_eq!(
            modell.beschriftung(2).as_deref(),
            Some("Die zweite Stelle (fehlt)")
        );
        assert_eq!(modell.sinnbild(2), Some(Sinnbild::Textstelle));

        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Loeschen { welches }),
            Some(Ausgang::Geaendert(1))
        );
        assert_eq!(modell.lesezeichenliste().zahl(), 2);
    }

    /// Das elfte Abnahmekriterium von C6: eine Textmarke kostet in der Leiste
    /// nicht mehr als eine Ordnermarke.
    ///
    /// **Was diese Probe misst und was nicht.** Die Zahl der Systemaufrufe ist
    /// von innerhalb des Prozesses nicht zu zaehlen. Messbar sind zwei
    /// Aussagen, und beide zusammen tragen die Zusage:
    ///
    /// - **Je Marke eine eigene Frage.** Zehn Marken auf fuenf vorhandene und
    ///   fuenf fehlende Dateien ergeben fuenf gueltige, und eine Datei, die
    ///   danach verschwindet, aendert genau ihre Marke. Eine gemeinsame oder
    ///   eine gemerkte Antwort fiele hier auf.
    /// - **Kein Lesevorgang.** Die letzte Marke zeigt auf eine Datei ohne
    ///   Leserecht. Sie bleibt gueltig; wer sie oeffnete, bekaeme `EACCES` und
    ///   muesste sie fuer ungueltig erklaeren. Unter `root` sagt diese Haelfte
    ///   weniger, weil dort auch das Oeffnen gelingt — falsch anschlagen kann
    ///   sie deshalb nicht.
    #[test]
    fn zehn_textmarken_kosten_je_eine_frage_und_keinen_lesevorgang() {
        let ordner = Pruefordner::nur_name("zehn-textmarken");
        ordner.anlegen();
        let mut marken = Vec::new();
        let mut vorhandene = Vec::new();
        for nummer in 0..10u32 {
            let name = format!("zeile-{nummer}.txt");
            let pfad = match nummer % 2 == 0 {
                true => {
                    let pfad = ordner.datei(&name, "eine Zeile\n");
                    vorhandene.push(pfad.clone());
                    pfad
                }
                false => ordner.pfad().join(&name),
            };
            marken.push(Lesezeichen::textstelle(
                format!("Marke {nummer}"),
                pfad,
                1,
                "eine Zeile",
            ));
        }
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(marken));
        assert_eq!(gueltige_marken(&modell, 10), 5);

        std::fs::remove_file(&vorhandene[0]).expect("die Pruefdatei laesst sich nicht loeschen");
        assert!(
            modell.gueltigkeit_pruefen(),
            "ohne die gemeldete Aenderung zeichnete die Leiste nicht neu"
        );
        assert!(modell.ungueltig(1));
        assert_eq!(gueltige_marken(&modell, 10), 4);

        let verschlossen = ordner.datei("verschlossen.txt", "eine Zeile\n");
        std::fs::set_permissions(&verschlossen, std::fs::Permissions::from_mode(0o000))
            .expect("das Leserecht laesst sich nicht nehmen");
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::textstelle(
            "Verschlossen",
            &verschlossen,
            1,
            "eine Zeile",
        )]));
        assert!(
            !modell.ungueltig(1),
            "die Pruefung fragt nach der Datei und liest sie nicht"
        );
    }

    /// Wie viele der ersten `zahl` Lesezeichenzeilen gueltig sind.
    fn gueltige_marken(modell: &Leistenmodell, zahl: usize) -> usize {
        (1..=zahl)
            .filter(|stelle| !modell.ungueltig(*stelle))
            .count()
    }

    #[test]
    fn die_auswahl_wandert_mit_dem_verschobenen_lesezeichen() {
        let mut modell = modell();
        modell.waehlen(2);
        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Verschieben {
                welches,
                richtung: Verschiebung::Hoch,
            }),
            Some(Ausgang::Geaendert(0))
        );
        assert_eq!(modell.auswahl(), Some(1));
        assert_eq!(modell.beschriftung(1).as_deref(), Some("Zwei (fehlt)"));
        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Verschieben {
                welches,
                richtung: Verschiebung::Hoch,
            }),
            Some(Ausgang::Unveraendert),
            "oben ist Schluss"
        );
    }

    #[test]
    fn nach_dem_loeschen_des_letzten_lesezeichens_steht_die_auswahl_auf_dem_ersten_geraet() {
        let mut modell = Leistenmodell::neu();
        modell.lesezeichen_setzen(&Lesezeichenliste::aus(vec![Lesezeichen::neu(
            "Eins", "/eins",
        )]));
        modell.orte_setzen(vec![Ort::neu("Benutzer", "/Users/pruefung")]);
        modell.waehlen(1);

        assert_eq!(
            gewaehltes_aendern(&mut modell, |welches| Aenderung::Loeschen { welches }),
            Some(Ausgang::Geaendert(0))
        );

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
        let ordner = Pruefordner::nur_name("gueltigkeit");
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
        let ordner = Pruefordner::nur_name("vorgang-beenden");
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
        let vorhanden = Pruefordner::nur_name("aufbau-vorhanden");
        vorhanden.anlegen();
        let fehlend = Pruefordner::nur_name("aufbau-fehlend");

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
        let ordner = Pruefordner::nur_name("orte-setzen");
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
