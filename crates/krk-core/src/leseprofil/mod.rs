//! Die Leseprofile: was an einem Ort liegt, ohne dass der Nutzer ihn betritt.
//!
//! Ein Profil erkennt seinen Ort ueber ein Pfadmuster oder ueber eine
//! Kennzeichendatei darin und beschreibt aus vier Bausteinen die
//! Zusammenfassung, die das Vorschaufenster dort zeigt. Die Gestalt der Datei,
//! die der Nutzer dafuer von Hand pflegt, und der Pruefschritt dahinter stehen
//! in [`datei`]; hier stehen die **geprueften** Werte, mit denen die Auswertung
//! danach arbeitet, und die Werte, die sie liefert. Welches Profil ein
//! ausgewaehlter Ordner bekommt, entscheidet [`erkennung`] in zwei
//! Durchgaengen.
//!
//! ```text
//! readers.toml ──serde──> datei::Profildatei ──datei::pruefen──> Profile
//!                                                                   │
//!                            ausgewaehlter Ordner ──erkennung───────┘
//!                                                   │
//!                                                   v
//!                                            Zusammenfassung
//! ```
//!
//! # Warum die Auswertung im Kern liegt und nicht in `krk-ui`
//!
//! C6.8 verlangt, dass die abzaehlbaren Grenzen dieser Runde durch Proben
//! belegt sind, die **ohne Fenster** laufen. `krk-ui` hat kein
//! Bibliotheksziel, sondern allein das Binaerziel `krk`: eine Datei unter
//! `crates/krk-ui/tests/` ist deshalb eine eigene Kiste und erreicht nichts
//! aus jener Kiste, ob `pub` oder nicht. Im Kern erreicht eine Probe alles,
//! und die Zaehlung der Leselaeufe und Oeffnungen ist damit nachrechenbar
//! statt behauptet.
//!
//! Dazu kommt, dass die Lesemaschinerie, die eine Zusammenfassung braucht,
//! ohnehin hier steht: [`crate::verzeichnis::leser`] fuer die Eintraege eines
//! Ordners und [`crate::text::datei`] fuer die Bytes einer Datei. Eine
//! Auswertung in `krk-ui` haette beide ueber die Kistengrenze gerufen und
//! ihren Haushalt dort gefuehrt, wo ihn keine Probe ohne Fenster nachzaehlen
//! kann.
//!
//! # Warum die Bausteinauswahl unmarkiert ist
//!
//! [`Baustein`] traegt vier Werte, und in `readers.toml` steht dafuer **keine
//! Sortenkennung**, sondern allein der Tisch `zaehlung`, `juengste`, `feld`
//! oder `vorhandensein` neben der Beschriftung. Die Vorlage ist
//! [`crate::ablage::lesezeichen::Ziel`], die unmarkierte Auswahl der
//! Lesezeichen: dieselbe Form, ueber `#[serde(flatten)]` neben das
//! gemeinsame Feld gelegt, aus demselben Grund. Die Datei bleibt von Hand
//! lesbar, und der Nutzer pflegt kein `baustein = "zaehlung"` mit, das
//! dasselbe ein zweites Mal sagte.
//!
//! **Der Vorbehalt der Vorlage gilt hier weiter.** `#[serde(flatten)]` zwingt
//! den Deserialisierer ueber einen zwischenspeichernden Weg, und ob `toml` die
//! Verbindung aus `flatten` und `untagged` traegt, ist am Papier nicht zu
//! entscheiden; dort nimmt ihn eine Rundreise ueber beide Sorten ab
//! (`tests/ablage.rs::eine_rundreise_ueber_beide_sorten_liefert_dieselbe_datei`),
//! hier eine ueber alle vier Bausteine
//! (`tests/leseprofil.rs::eine_rundreise_ueber_alle_vier_bausteine_liefert_die_erwarteten_werte`).
//! Die Vorlage traegt zwei Varianten, diese Auswahl vier; die Rundreise ist
//! deshalb die erste Probe, die laufen muss. Sollte sie eines Tages fallen,
//! ist der Ausweg benannt und nicht zu suchen: die Zeile bekommt ein Feld
//! `baustein = "zaehlung"` als ausgeschriebene Sortenkennung und einen von
//! Hand geschriebenen Pruefschritt, der genau eine Bausteinangabe fordert. Der
//! Preis waere eine Zeile mehr je Profilzeile in der Datei.
//!
//! # Warum jede Pruefung beim Laden laeuft und nicht beim Anzeigen
//!
//! Ein nicht uebersetzbares Muster, ein Feldmuster mit zwei Fanggruppen und
//! eine Ortsangabe, die aus dem erkannten Ordner herausfuehrt, sind Aussagen
//! ueber die **Datei** und nicht ueber den angezeigten Ort. Sie fallen deshalb
//! dort an, wo die Datei gelesen wird, und ihre Meldung erscheint **einmal**
//! beim Start in der Statuszeile. Beim Anzeigen fielen dieselben Befunde so
//! oft an, wie der Nutzer einen Ordner auswaehlt, und eine Meldung, die auf
//! jede Auswahl folgt, wird weggesehen statt gelesen.
//!
//! Daraus folgt der Zuschnitt der Auswertung: **sie kennt keinen
//! Meldungskanal.** Sie liefert Werte und Platzhalter, und die Statuszeile
//! hoert von ihr nichts. Ein abgewiesenes Profil steht nach [`datei::pruefen`]
//! nicht mehr in der Liste; eine abgewiesene Zeile steht darin und traegt
//! keinen Baustein ([`Zeile::baustein`] antwortet `None`), zeigt also ihre
//! Beschriftung und den Platzhalter [`Wert::Nicht`] — jedes Mal und ohne
//! weitere Meldung.

use std::path::PathBuf;

use regex::Regex;

pub mod datei;
pub mod erkennung;

// ---------------------------------------------------------------------------
// Die Zahlen des Haushalts
// ---------------------------------------------------------------------------

/// Wie viele Verzeichnisleselaeufe eine Zusammenfassung kostet, hoechstens
/// (C6.4).
///
/// Der Erkennungslauf zaehlt mit: er ist ein Leselauf dieser Zusammenfassung
/// und keiner daneben.
pub const HOECHSTENS_LESELAEUFE: u32 = 12;

/// Wie viele Dateioeffnungen eine Zusammenfassung kostet, hoechstens (C6.4).
///
/// Eine Datei, die zwei Bausteine desselben Profils lesen, wird zweimal
/// geoeffnet. Das ist gewollt: so ist die Zahl der Oeffnungen aus dem Profil
/// ablesbar, naemlich eine je Feldbaustein und N je Baustein „juengste N".
pub const HOECHSTENS_OEFFNUNGEN: u32 = 24;

/// Wie viele Eintraege ein einzelner Verzeichnisleselauf liefert, hoechstens
/// (C6.5, Festlegung A5).
///
/// Der groesste Speicher der Werkbank, an der die Runde gemessen hat, traegt
/// 157 Eintraege; die Grenze laesst Raum fuer das Zehnfache und kappt keine
/// Zaehlung des Beispielfalls. Was eine abgeschnittene Lesung noch sagen darf,
/// steht bei [`Wert::UeberGrenze`].
pub const HOECHSTENS_EINTRAEGE: usize = 2_000;

/// Wie viele Bytes ein Baustein aus einer Datei liest, hoechstens (C6.6).
pub const HOECHSTENS_BYTES: u64 = 64 * 1024;

/// Wie viele Eintraege der Baustein „juengste N" liefert, hoechstens (C6.3).
///
/// Eine groessere Zahl in der Datei wird auf diesen Wert **gekappt** und nicht
/// abgewiesen: sie ist keine falsche Angabe, sondern eine, die mehr verlangt,
/// als die Zusammenfassung hergibt.
pub const HOECHSTENS_JUENGSTE: u8 = 10;

// ---------------------------------------------------------------------------
// Die geprueften Profile
// ---------------------------------------------------------------------------

/// Die geprueften Profile, in der Reihenfolge der Datei.
///
/// Die Reihenfolge ist tragend und kein Zufall der Datenhaltung: die Erkennung
/// nimmt das erste Profil mit Treffer, und der Nutzer ordnet seine Profile,
/// indem er die Bloecke in `readers.toml` verschiebt (C2.2).
///
/// Der Auslieferungswert ist die leere Liste. Er heisst „keine Profile" und
/// ist kein Fehlerfall: eine `readers.toml` ohne einen einzigen Block ist
/// gueltig (C1.5), und im Messmodus liest KRK die Ablage gar nicht erst.
#[derive(Debug, Clone, Default)]
pub struct Profile {
    profile: Vec<Profil>,
}

impl Profile {
    /// Die Liste aus geprueften Profilen. Der eine Weg dorthin ist
    /// [`datei::pruefen`].
    pub fn aus(profile: Vec<Profil>) -> Self {
        Self { profile }
    }

    /// Wie viele Profile die Liste fuehrt.
    pub fn zahl(&self) -> usize {
        self.profile.len()
    }

    /// Die Profile in der Reihenfolge der Datei.
    pub fn iter(&self) -> std::slice::Iter<'_, Profil> {
        self.profile.iter()
    }
}

/// Ein einzelnes Profil: woran es seinen Ort erkennt und was es dort zeigt.
///
/// Mindestens eines von [`Profil::pfad`] und [`Profil::kennzeichen`] steht;
/// ein Profil ohne beides koennte nie treffen und wird beim Laden abgewiesen.
#[derive(Debug, Clone)]
pub struct Profil {
    name: String,
    pfad: Option<Regex>,
    kennzeichen: Option<Regex>,
    zeilen: Vec<Zeile>,
}

impl Profil {
    /// Ein geprueftes Profil aus seinen Bestandteilen.
    pub fn neu(
        name: String,
        pfad: Option<Regex>,
        kennzeichen: Option<Regex>,
        zeilen: Vec<Zeile>,
    ) -> Self {
        Self {
            name,
            pfad,
            kennzeichen,
            zeilen,
        }
    }

    /// Der Name aus der Datei.
    ///
    /// **Nur fuer Meldungen.** Er erkennt nichts und wird nicht angezeigt; die
    /// Kopfzeile der Zusammenfassung traegt den Namen des Ordners und nicht
    /// den des Profils (Festlegung A6).
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Das Muster auf dem **vollen Pfad** des ausgewaehlten Ordners.
    pub fn pfad(&self) -> Option<&Regex> {
        self.pfad.as_ref()
    }

    /// Das Muster auf den **Namen der Eintraege** im ausgewaehlten Ordner.
    pub fn kennzeichen(&self) -> Option<&Regex> {
        self.kennzeichen.as_ref()
    }

    /// Die Zeilen der Zusammenfassung, in der Reihenfolge der Datei.
    pub fn zeilen(&self) -> &[Zeile] {
        &self.zeilen
    }
}

/// Eine Zeile der Zusammenfassung: eine Beschriftung und ein Baustein.
#[derive(Debug, Clone)]
pub struct Zeile {
    beschriftung: String,
    baustein: Option<Baustein>,
}

impl Zeile {
    /// Eine gepruefte Zeile. `None` als Baustein heisst: beim Laden abgewiesen.
    pub fn neu(beschriftung: String, baustein: Option<Baustein>) -> Self {
        Self {
            beschriftung,
            baustein,
        }
    }

    /// Die Beschriftung, wie sie in der Zusammenfassung links steht.
    pub fn beschriftung(&self) -> &str {
        &self.beschriftung
    }

    /// Der Baustein, oder `None`, wenn er beim Laden abgewiesen wurde.
    ///
    /// `None` ist kein Fehler mehr, sondern ein Zustand: die Zeile zeigt ihre
    /// Beschriftung und [`Wert::Nicht`]. Gemeldet wurde die Abweisung einmal
    /// beim Start, siehe den Modulkopf.
    pub fn baustein(&self) -> Option<&Baustein> {
        self.baustein.as_ref()
    }
}

/// Der feste Bausteinsatz aus C3: vier Bausteine und kein fuenfter.
///
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig. Ein fuenfter
/// Baustein haelt jeden Rechner an und erzwingt eine bewusste Einordnung;
/// Festlegung A7 haelt die Zahl vier fest und nennt sie ausdruecklich als das,
/// was den Zustand eines Circles auf drei Vorhandensein-Zeilen verteilt,
/// statt einen Baustein fuer Dateinamen aufzunehmen.
///
/// Zwei der vier sehen auf **Namen** und zwei lesen **Dateien**, und daran
/// haengt der Umgang mit Verknuepfungen: [`Baustein::Zaehlung`] und
/// [`Baustein::Vorhandensein`] sehen auf die Namen aller Eintraege, gleich
/// welchen Typs, [`Baustein::Juengste`] und [`Baustein::Feld`] nehmen allein
/// Eintraege vom Typ Datei.
#[derive(Debug, Clone)]
pub enum Baustein {
    /// B1: die Zahl der Eintraege, deren Name das Muster erfuellt.
    ///
    /// Ohne Muster zaehlt sie alle. Sie laeuft flach ueber eine Ebene und
    /// nicht ueber den Unterbaum (Festlegung A2, C3.2).
    Zaehlung {
        /// Wo gezaehlt wird.
        ort: Ortsangabe,
        /// Das Muster auf dem Eintragsnamen, oder alle Eintraege.
        muster: Option<Regex>,
    },
    /// B2: die N Eintraege mit dem juengsten Aenderungsdatum, je mit Titel.
    Juengste {
        /// Wo gesucht wird.
        ort: Ortsangabe,
        /// Das Muster auf dem Eintragsnamen, oder alle Eintraege.
        muster: Option<Regex>,
        /// Wie viele, hoechstens [`HOECHSTENS_JUENGSTE`].
        anzahl: u8,
    },
    /// B3: die erste Fanggruppe des ersten Treffers im Inhalt einer Datei.
    Feld {
        /// In welchem Ordner die Datei gesucht wird.
        ort: Ortsangabe,
        /// Das Muster auf dem Dateinamen.
        datei: Regex,
        /// Das Muster auf dem Inhalt, mit genau einer Fanggruppe (C3.10).
        feldmuster: Regex,
    },
    /// B4: ob ein Eintrag das Muster erfuellt.
    Vorhandensein {
        /// Wo gesucht wird.
        ort: Ortsangabe,
        /// Das Muster auf dem Eintragsnamen.
        muster: Regex,
    },
}

/// Wo ein Baustein arbeitet, relativ zum erkannten Ordner.
///
/// Leer heisst: der erkannte Ordner selbst. Sonst eine Folge gewoehnlicher
/// Namensbestandteile — beim Laden geprueft ([`Ortsangabe::aus_angabe`]) und
/// beim Auswerten gegen den aufgeloesten erkannten Ordner gehalten.
///
/// **Die zwei Pruefungen ersetzen einander nicht.** Die textliche greift beim
/// Laden und kostet keinen Systemaufruf, entscheidet aber nicht, ob eine
/// Verknuepfung im Weg aus dem Ordner herausfuehrt; die aufgeloeste
/// entscheidet das und kostet dafuer einen Aufruf je Auswertung. C3.13
/// verlangt beide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ortsangabe {
    teile: Vec<String>,
}

impl Ortsangabe {
    /// Der erkannte Ordner selbst, ohne Unterordner.
    pub fn wurzel() -> Self {
        Self { teile: Vec::new() }
    }

    /// Prueft eine Ortsangabe aus der Datei, textlich (C3.13, erste Haelfte).
    ///
    /// Abgewiesen wird, was schon am Text aus dem erkannten Ordner
    /// herausfuehrt oder ihn gar nicht erst benennt: ein absoluter Pfad, ein
    /// leeres Stueck (`a//b`, `planning/`) und die Stuecke `.` und `..`. Was
    /// uebrig bleibt, ist eine Folge gewoehnlicher Namensbestandteile — und
    /// ob **die** aufgeloest im Ordner bleibt, entscheidet erst die zweite
    /// Haelfte der Pruefung beim Auswerten.
    pub fn aus_angabe(angabe: &str) -> Result<Self, Ortsmangel> {
        if angabe.starts_with('/') {
            return Err(Ortsmangel::Absolut);
        }
        let mut teile = Vec::new();
        for stueck in angabe.split('/') {
            match stueck {
                "" => return Err(Ortsmangel::LeeresStueck),
                "." | ".." => return Err(Ortsmangel::Punktstueck),
                name => teile.push(name.to_owned()),
            }
        }
        Ok(Self { teile })
    }

    /// Die Namensbestandteile, von oben nach unten. Leer heisst: der erkannte
    /// Ordner selbst.
    pub fn teile(&self) -> &[String] {
        &self.teile
    }
}

/// Warum eine Ortsangabe schon am Text abgewiesen wird.
///
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ortsmangel {
    /// Sie beginnt mit einem Schraegstrich und benennt damit keinen
    /// Unterordner des erkannten Ordners, sondern einen Ort im Dateisystem.
    Absolut,
    /// Sie traegt ein leeres Stueck, etwa durch einen doppelten oder einen
    /// abschliessenden Schraegstrich.
    LeeresStueck,
    /// Sie traegt `.` oder `..`. Das zweite fuehrt aus dem Ordner heraus, das
    /// erste sagt nichts und stuende nur da, um wie ein Pfad auszusehen.
    Punktstueck,
}

impl Ortsmangel {
    /// Der Satzteil, der den Mangel benennt, so wie ihn die Statuszeile zeigt.
    ///
    /// Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig.
    pub fn grund(self) -> &'static str {
        match self {
            Ortsmangel::Absolut => "ist ein absoluter Pfad",
            Ortsmangel::LeeresStueck => "traegt ein leeres Stueck",
            Ortsmangel::Punktstueck => "traegt ein Stueck . oder ..",
        }
    }
}

// ---------------------------------------------------------------------------
// Was die Auswertung liefert
// ---------------------------------------------------------------------------

/// Was das Vorschaufenster fuer einen erkannten Ordner zeigt.
///
/// Der Wert wandert **strukturiert** bis in die Ansicht und wird erst dort zu
/// Text: die Abnahmekriterien C3.1 bis C3.12 pruefen Werte und keine
/// Zeilenumbrueche, und gegen eine fertige Zeichenkette waeren sie bruechig.
///
/// [`Zusammenfassung::name`] und [`Zusammenfassung::pfad`] sind die eine
/// Auskunft der Metadatenanzeige, die die Ersetzung ueberlebt (Festlegung A6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zusammenfassung {
    name: String,
    pfad: PathBuf,
    zeilen: Vec<Zusammenfassungszeile>,
}

impl Zusammenfassung {
    /// Eine fertige Zusammenfassung aus ihren Bestandteilen.
    pub fn neu(name: String, pfad: PathBuf, zeilen: Vec<Zusammenfassungszeile>) -> Self {
        Self { name, pfad, zeilen }
    }

    /// Der Name des Ordners, ueber den sie spricht.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Der volle Pfad dieses Ordners.
    pub fn pfad(&self) -> &PathBuf {
        &self.pfad
    }

    /// Die Zeilen in der Reihenfolge des Profils.
    pub fn zeilen(&self) -> &[Zusammenfassungszeile] {
        &self.zeilen
    }
}

/// Eine Zeile der fertigen Zusammenfassung.
///
/// Die Beschriftung steht auch dann da, wenn der Baustein nichts gefunden hat
/// oder beim Laden abgewiesen wurde; dann traegt der Wert [`Wert::Nicht`]
/// (C3.12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zusammenfassungszeile {
    beschriftung: String,
    wert: Wert,
}

impl Zusammenfassungszeile {
    /// Eine Zeile aus Beschriftung und Wert.
    pub fn neu(beschriftung: String, wert: Wert) -> Self {
        Self { beschriftung, wert }
    }

    /// Die Beschriftung aus dem Profil.
    pub fn beschriftung(&self) -> &str {
        &self.beschriftung
    }

    /// Was der Baustein ergeben hat.
    pub fn wert(&self) -> &Wert {
        &self.wert
    }
}

/// Was ein Baustein ergeben hat.
///
/// Eine vollstaendige Fallunterscheidung ohne Auffangzweig: ein siebter Wert
/// haelt die Anzeige an und erzwingt die Antwort darauf, wie er dasteht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Wert {
    /// Eine Zahl, genau gezaehlt.
    Zahl(u64),
    /// Die Lesung hat die Grenze erreicht, die der Wert traegt; gezaehlt sind
    /// mehr Eintraege, als die Zahl sagt.
    ///
    /// **Es wird nur gesagt, was die Teillesung entscheidet.** Eine
    /// abgeschnittene Liste kann sagen, dass es mehr als die Grenze sind, und
    /// sonst nichts. Der Satz der Anzeige entsteht aus dieser Zahl und nicht
    /// aus einer zweiten im Text.
    UeberGrenze(u64),
    /// Ja oder nein.
    ///
    /// Ein Nichtfund in einer abgeschnittenen Liste ist **kein** Nein und
    /// liefert [`Wert::Nicht`]; derselbe Rueckgriff, den
    /// `verzeichnis::sys::ist_deskriptormangel` seit der Runde 10 im Durchlauf
    /// traegt.
    Vorhanden(bool),
    /// Ein aus einer Datei gezogenes Feld.
    Text(String),
    /// Bis zu N Titel, in der Reihenfolge des Aenderungsdatums.
    Titel(Vec<String>),
    /// Der Platzhalter aus C3.12: hier ist nichts zu sagen.
    ///
    /// Er steht fuer drei verschiedene Lagen, und das ist Absicht — der Nutzer
    /// sieht in allen dreien dasselbe, naemlich dass diese Zeile ihm nichts
    /// beantwortet: der Baustein hat nichts gefunden, er wurde beim Laden
    /// abgewiesen, oder die Lesung reichte fuer eine Antwort nicht aus.
    Nicht,
}

// ---------------------------------------------------------------------------
// Der Haushalt
// ---------------------------------------------------------------------------

/// Was eine einzelne Zusammenfassung schon verbraucht hat.
///
/// Die Zahlen, gegen die gezaehlt wird, stehen als Konstanten oben in diesem
/// Modul und nirgends sonst. Verbraucht wird in Dateireihenfolge; ist eine der
/// beiden Grenzen erreicht, setzen die uebrigen Bausteine ihren Platzhalter,
/// statt dass die Zusammenfassung abbricht.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Haushalt {
    leselaeufe: u32,
    oeffnungen: u32,
}

impl Haushalt {
    /// Ein unverbrauchter Haushalt.
    pub fn neu() -> Self {
        Self::default()
    }

    /// Bucht einen Verzeichnisleselauf und sagt, ob er noch im Haushalt lag.
    ///
    /// `false` heisst: er lag nicht mehr darin und hat auch nicht
    /// stattgefunden. Der Zaehler bleibt dann stehen, damit die Zahl am Ende
    /// die tatsaechlichen Laeufe nennt und nicht die versuchten.
    #[must_use = "wer nicht hinsieht, liest ueber die Grenze hinaus"]
    pub fn leselauf_nehmen(&mut self) -> bool {
        if self.leselaeufe >= HOECHSTENS_LESELAEUFE {
            return false;
        }
        self.leselaeufe += 1;
        true
    }

    /// Bucht eine Dateioeffnung und sagt, ob sie noch im Haushalt lag.
    #[must_use = "wer nicht hinsieht, oeffnet ueber die Grenze hinaus"]
    pub fn oeffnung_nehmen(&mut self) -> bool {
        if self.oeffnungen >= HOECHSTENS_OEFFNUNGEN {
            return false;
        }
        self.oeffnungen += 1;
        true
    }

    /// Wie viele Verzeichnisleselaeufe tatsaechlich stattgefunden haben.
    pub fn leselaeufe(self) -> u32 {
        self.leselaeufe
    }

    /// Wie viele Dateien tatsaechlich geoeffnet wurden.
    pub fn oeffnungen(self) -> u32 {
        self.oeffnungen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Eine leere Angabe gibt es nicht: sie kommt als fehlender Schluessel und
    /// wird zu [`Ortsangabe::wurzel`]. Ein leerer **Text** ist dagegen ein
    /// leeres Stueck und wird abgewiesen.
    #[test]
    fn eine_ortsangabe_traegt_gewoehnliche_namensbestandteile() {
        let ort = Ortsangabe::aus_angabe("planning/entwuerfe").expect("die Angabe ist gewoehnlich");
        assert_eq!(ort.teile(), ["planning", "entwuerfe"]);
        assert!(Ortsangabe::wurzel().teile().is_empty());
    }

    #[test]
    fn eine_ortsangabe_die_herausfuehrt_wird_schon_am_text_abgewiesen() {
        let abgewiesen = [
            ("/etc", Ortsmangel::Absolut),
            ("/", Ortsmangel::Absolut),
            ("", Ortsmangel::LeeresStueck),
            ("planning//entwuerfe", Ortsmangel::LeeresStueck),
            ("planning/", Ortsmangel::LeeresStueck),
            ("..", Ortsmangel::Punktstueck),
            ("planning/../../weg", Ortsmangel::Punktstueck),
            (".", Ortsmangel::Punktstueck),
        ];
        for (angabe, mangel) in abgewiesen {
            assert_eq!(
                Ortsangabe::aus_angabe(angabe),
                Err(mangel),
                "die Angabe {angabe:?} kommt durch"
            );
        }
    }

    #[test]
    fn der_haushalt_zaehlt_die_tatsaechlichen_laeufe_und_nicht_die_versuchten() {
        let mut haushalt = Haushalt::neu();
        for _ in 0..HOECHSTENS_LESELAEUFE {
            assert!(haushalt.leselauf_nehmen(), "der Lauf liegt im Haushalt");
        }
        assert!(!haushalt.leselauf_nehmen(), "der Haushalt ist erschoepft");
        assert_eq!(haushalt.leselaeufe(), HOECHSTENS_LESELAEUFE);
        assert_eq!(haushalt.oeffnungen(), 0, "die Laeufe kosten keine Oeffnung");
    }
}
