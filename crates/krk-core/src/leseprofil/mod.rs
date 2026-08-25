//! Die Leseprofile: was an einem Ort liegt, ohne dass der Nutzer ihn betritt.
//!
//! Ein Profil erkennt seinen Ort ueber ein Pfadmuster oder ueber eine
//! Kennzeichendatei darin und beschreibt aus vier Bausteinen die
//! Zusammenfassung, die das Vorschaufenster dort zeigt. Die Gestalt der Datei,
//! die der Nutzer dafuer von Hand pflegt, und der Pruefschritt dahinter stehen
//! in [`datei`]; hier stehen die **geprueften** Werte, mit denen die Auswertung
//! danach arbeitet, und die Werte, die sie liefert. Welches Profil ein
//! ausgewaehlter Ordner bekommt, entscheidet [`erkennung`] in zwei
//! Durchgaengen; was seine Zeilen dort ergeben, rechnet [`bausteine`] innerhalb
//! des [`Haushalt`]s, dessen Zahlen weiter unten als Konstanten stehen.
//!
//! ```text
//! readers.toml ──serde──> datei::Profildatei ──datei::pruefen──> Profile
//!                                                                   │
//!                            ausgewaehlter Ordner ──erkennung───────┘
//!                                                   │
//!                                    bausteine::zusammenfassen
//!                                                   │
//!                                                   v
//!                                     Zusammenfassung ──als_text──> Vorschau
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
//! # Warum die Datei keine Sortenkennung traegt und der Pruefschritt trotzdem
//! zaehlt
//!
//! [`Baustein`] traegt vier Werte, und in `readers.toml` steht dafuer **keine
//! Sortenkennung**, sondern allein der Tisch `zaehlung`, `juengste`, `feld`
//! oder `vorhandensein` neben der Beschriftung. Die Datei bleibt damit von
//! Hand lesbar, und der Nutzer pflegt kein `baustein = "zaehlung"` mit, das
//! dasselbe ein zweites Mal sagte.
//!
//! **Die Wahl unter den vieren trifft [`datei::Zeilendatei::zerlegen`] und
//! nicht `serde`.** Bis zum 260824 stand hier eine unmarkierte Auswahl hinter
//! `#[serde(flatten)]`, nach der Vorlage von
//! [`crate::ablage::lesezeichen::Ziel`], und sie hat zwei Dinge nicht geleistet
//! (`issues/260824-1216_*_zwei-bausteintische-…`,
//! `issues/260824-1217_*_ein-tippfehler-in-einem-bausteintisch-…`): zwei Tische
//! in einer Zeile nahm sie schweigend an und liess den unteren fallen, und die
//! Meldung eines verschriebenen Schluessels verwarf sie unterwegs, weil sie
//! allein sagen konnte, dass keine Variante gepasst hat. Beides sind Aussagen
//! ueber die Zeile, die dem Nutzer gehoeren, und deshalb steht die Auswahl
//! jetzt als vier benannte Felder da, deren Zahl der Pruefschritt zaehlt. Die
//! Vorlage bleibt bei ihrer Form: ihre zwei Varianten unterscheiden sich an je
//! einem Pflichtfeld und tragen keinen Tisch, an dem eine Meldung verloren
//! gehen koennte.
//!
//! **Der Vorbehalt der Vorlage ist damit erledigt und nicht bloss umgangen.**
//! `#[serde(flatten)]` zwang den Deserialisierer ueber einen
//! zwischenspeichernden Weg, und ob `toml` die Verbindung aus `flatten` und
//! `untagged` traegt, war am Papier nicht zu entscheiden; die Rundreise ueber
//! alle vier Bausteine
//! (`tests/leseprofil.rs::eine_rundreise_ueber_alle_vier_bausteine_liefert_die_erwarteten_werte`)
//! nimmt jetzt vier gewoehnliche `Option`-Felder ab und keine Verbindung
//! zweier Sonderwege mehr.
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

pub mod bausteine;
pub mod datei;
pub mod erkennung;

pub use bausteine::{zusammenfassen, zusammenfassen_gezaehlt};

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

/// Wie viele Eintraege ein einzelner Leselauf liefert, hoechstens (C6.5,
/// Festlegung A5).
///
/// Der groesste Speicher der Werkbank, an der die Runde gemessen hat, traegt
/// 157 Eintraege; die Grenze laesst Raum fuer das Zehnfache und kappt keine
/// Zaehlung des Beispielfalls. Was eine abgeschnittene Lesung noch sagen darf,
/// steht bei [`Wert::UeberGrenze`].
///
/// **Seit der Runde 18 begrenzt sie einen Ort und nicht ein Verzeichnis.** Eine
/// Ortsangabe mit Platzhalter ([`Ortsangabe::hinter_dem_platzhalter`]) legt die
/// Eintraege mehrerer Verzeichnisse zu **einem** Lesestand zusammen, und diese
/// Zahl deckelt die Sammlung. Das ist die Einheit, die die Arbeit dort noch
/// begrenzt, wo [`HOECHSTENS_LESELAEUFE`] es nicht mehr kann: die Zahl der
/// Unterordner eines Ordners waechst mit dem Bestand, die Zahl der gelesenen
/// Eintraege ist gedeckelt.
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
/// Festlegung A7 haelt die Zahl vier fest und nennt als ihren Preis
/// ausdruecklich, dass der Zustand eines Circles sich auf mehrere
/// Vorhandensein-Zeilen verteilt, statt einen Baustein fuer Dateinamen zu
/// bekommen. **Wie viele Zeilen das sind, steht hier nicht**: es sind die
/// Zeilen, die `resources/default-readers.toml` im Circle-Profil fuehrt, und
/// eine dritte Stelle fuer diese Zahl waere eine dritte, die veraltet — bis
/// zum 260824 stand hier „drei", waehrend die Auslieferungsfassung seit
/// Schritt 14 vier traegt.
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

/// Das Stueck einer Ortsangabe, das fuer „jeder Unterordner hier" steht.
///
/// **Nicht zu verwechseln mit [`PLATZHALTER`]**, das weiter unten steht: jenes
/// ist das Zeichenpaar, das die Anzeige an die Stelle eines fehlenden Wertes
/// setzt, dieses das Stueck, das in `readers.toml` einen Namen offen laesst.
pub const PLATZHALTERSTUECK: &str = "*";

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
///
/// # Der eine Platzhalter
///
/// Genau eines der Stuecke darf [`PLATZHALTERSTUECK`] sein und laesst damit
/// einen Namen offen: `*` meint jeden Unterordner des erkannten Ordners,
/// `*/issues` den Speicher `issues` in jedem von ihnen. Die Auswertung legt
/// die Eintraege aller getroffenen Ordner zu **einem** Lesestand zusammen und
/// bucht dafuer **einen** Leselauf; begrenzt wird die Sammlung durch
/// [`HOECHSTENS_EINTRAEGE`].
///
/// **Warum hoechstens einer**: so ist die Form der Kosten aus dem Profil
/// abzulesen — ein Lauf ueber den Ordner vor dem Platzhalter, dann einer je
/// Treffer. Ein zweiter Platzhalter vervielfachte sie, und wie oft, stuende
/// erst am Bestand fest. Zwei oder mehr werden deshalb beim Laden abgewiesen
/// ([`Ortsmangel::MehrerePlatzhalter`]).
///
/// **Was er greift und was nicht**: allein Eintraege vom Typ
/// [`crate::verzeichnis::Typ::Ordner`], und damit keine Verknuepfung. Daran
/// haelt C3.13 durch Bauart statt durch eine zusaetzliche Pruefung — ein
/// wirklicher Unterordner eines Ordners innerhalb der Schranke liegt innerhalb
/// der Schranke. Es ist derselbe Grund, aus dem
/// [`crate::verzeichnis::durchlauf`] nicht in eine Verknuepfung absteigt.
///
/// **Wer ihn nicht annimmt**: [`Baustein::Juengste`] und [`Baustein::Feld`].
/// Beide **lesen** Dateien und brauchen dafuer deren Pfad, den ein
/// zusammengelegter Lesestand nicht mehr traegt; die Grenze liegt auf der
/// Naht, die der Modulkopf von [`bausteine`] ohnehin zieht. Abgewiesen wird
/// beim Laden, und die Zeile behaelt ihre Beschriftung.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ortsangabe {
    /// Die Stuecke vor dem Platzhalter; ohne Platzhalter alle.
    vor: Vec<String>,
    /// Die Stuecke hinter dem Platzhalter. `None` heisst: es steht keiner da.
    nach: Option<Vec<String>>,
}

impl Ortsangabe {
    /// Der erkannte Ordner selbst, ohne Unterordner.
    pub fn wurzel() -> Self {
        Self {
            vor: Vec::new(),
            nach: None,
        }
    }

    /// Prueft eine Ortsangabe aus der Datei, textlich (C3.13, erste Haelfte).
    ///
    /// Abgewiesen wird, was schon am Text aus dem erkannten Ordner
    /// herausfuehrt, ihn gar nicht erst benennt oder mehr verlangt, als die
    /// eine Sammlung hergibt: ein absoluter Pfad, ein leeres Stueck (`a//b`,
    /// `planning/`), die Stuecke `.` und `..` und ein zweiter Platzhalter. Was
    /// uebrig bleibt, ist eine Folge gewoehnlicher Namensbestandteile mit
    /// hoechstens einem offenen Namen darin — und ob **die** aufgeloest im
    /// Ordner bleibt, entscheidet erst die zweite Haelfte der Pruefung beim
    /// Auswerten.
    ///
    /// Ein Stueck, in dem ein `*` bloss **vorkommt**, ist ein gewoehnlicher
    /// Name: offen ist der Name nur, wenn das ganze Stueck aus dem Stern
    /// besteht. Ein Ordner, der `*` heisst, ist damit als Ort nicht mehr
    /// benennbar, und das ist der Preis dieser Form.
    pub fn aus_angabe(angabe: &str) -> Result<Self, Ortsmangel> {
        if angabe.starts_with('/') {
            return Err(Ortsmangel::Absolut);
        }
        let mut vor = Vec::new();
        let mut nach: Option<Vec<String>> = None;
        for stueck in angabe.split('/') {
            match stueck {
                "" => return Err(Ortsmangel::LeeresStueck),
                "." | ".." => return Err(Ortsmangel::Punktstueck),
                PLATZHALTERSTUECK if nach.is_some() => {
                    return Err(Ortsmangel::MehrerePlatzhalter);
                }
                PLATZHALTERSTUECK => nach = Some(Vec::new()),
                name => match nach.as_mut() {
                    Some(hinter) => hinter.push(name.to_owned()),
                    None => vor.push(name.to_owned()),
                },
            }
        }
        Ok(Self { vor, nach })
    }

    /// Die Namensbestandteile bis zum Platzhalter, von oben nach unten.
    ///
    /// Ohne Platzhalter sind es alle. Leer heisst: der erkannte Ordner selbst,
    /// und bei einer Angabe wie `*` heisst es, dass der Platzhalter unmittelbar
    /// in ihm greift.
    pub fn teile(&self) -> &[String] {
        &self.vor
    }

    /// Die Namensbestandteile hinter dem Platzhalter.
    ///
    /// `None` heisst: die Angabe traegt keinen Platzhalter. `Some` mit einer
    /// leeren Folge heisst: sie traegt einen, und dahinter steht nichts mehr —
    /// die Angabe `*`.
    pub fn hinter_dem_platzhalter(&self) -> Option<&[String]> {
        self.nach.as_deref()
    }

    /// Ob die Angabe einen Platzhalter traegt.
    pub fn traegt_platzhalter(&self) -> bool {
        self.nach.is_some()
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
    /// Sie traegt zwei oder mehr Platzhalter. Einer laesst die Kosten aus dem
    /// Profil ablesen, ein zweiter vervielfachte sie um eine Zahl, die erst am
    /// Bestand feststuende.
    MehrerePlatzhalter,
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
            Ortsmangel::MehrerePlatzhalter => {
                "traegt mehr als einen Platzhalter * und damit Kosten, die erst am Bestand \
                 feststuenden"
            }
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

    /// Die Zusammenfassung als anzuzeigender Text (C4.2, C4.3).
    ///
    /// Eine reine Funktion, und die **eine** Stelle, an der aus den Werten
    /// Zeilen werden. `Name:` und `Pfad:` stehen oben wie in der
    /// Metadatenanzeige (Festlegung A6); darunter steht je Profilzeile eine
    /// Zeile aus Beschriftung und Wert.
    ///
    /// **Wann ein Wert unter seine Beschriftung rutscht**, ist eine
    /// vollstaendige und ueberschneidungsfreie Unterscheidung mit zwei Fragen:
    /// [`Wert::Titel`] steht immer darunter, weil C4.3 einen Block aus bis zu N
    /// Zeilen verlangt, und jeder andere Wert genau dann, wenn er selbst mehr
    /// als eine Zeile traegt. Das zweite ist kein Sonderfall, sondern die Folge
    /// von C3.9: der Feldbaustein greift einen ganzen Absatz, und einer der
    /// Circle-Datensaetze dieser Werkbank traegt seine Directive auf vier
    /// Zeilen. Hinter der Beschriftung stehend liefe er in die naechste
    /// Beschriftung hinein.
    ///
    /// Der Weg an die Flaeche geht danach durch `text_zeigen` wie jeder andere
    /// Text der Vorschau; damit gilt die Auswaehlbarkeit aus der Runde 14
    /// unveraendert weiter (C4.6).
    #[must_use = "der Text ist das Ergebnis; wer ihn fallen laesst, zeigt nichts an"]
    pub fn als_text(&self) -> String {
        let mut ausgabe = format!("Name: {}\nPfad: {}", self.name, self.pfad.display());
        for zeile in &self.zeilen {
            let wert = zeile.wert().als_text();
            if matches!(zeile.wert(), Wert::Titel(_)) || wert.contains('\n') {
                ausgabe.push_str(&format!("\n{}:", zeile.beschriftung()));
                for teilzeile in wert.lines() {
                    ausgabe.push_str(&format!("\n{EINRUECKUNG}{teilzeile}"));
                }
            } else {
                ausgabe.push_str(&format!("\n{}: {wert}", zeile.beschriftung()));
            }
        }
        ausgabe
    }
}

/// Was an der Stelle eines Wertes steht, ueber den nichts zu sagen ist (C3.12).
///
/// Kein neues Zeichen: die Metadatenanzeige schreibt es seit der Runde 1 in die
/// Groessenzeile eines Ordners, und es heisst dort schon „darueber ist nichts
/// zu sagen".
pub const PLATZHALTER: &str = "--";

/// Womit die Zeilen eines Blocks unter ihrer Beschriftung einruecken.
const EINRUECKUNG: &str = "    ";

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
    /// Die Lesung wurde bei [`HOECHSTENS_EINTRAEGE`] abgebrochen; die Zahl ist,
    /// was **innerhalb** der gelesenen Eintraege getroffen hat.
    ///
    /// **Es wird nur gesagt, was die Teillesung entscheidet.** Sie entscheidet
    /// zweierlei, und der Satz der Anzeige traegt beides: dass es mindestens so
    /// viele sind wie die getroffenen, und dass die Lesung abgebrochen wurde.
    ///
    /// **Die Zahl ist die der Treffer und nicht die der Grenze**, und darin
    /// liegt der Unterschied zu einem Satz, der allein die Grenze naehme: ein
    /// Muster, auf das in 2.101 Eintraegen genau einer passt, ergaebe „ueber
    /// 2.000 offene Defekte" und damit eine falsche Aussage. „Mindestens 1" ist
    /// wahr — und „ueber 1" waere es nicht, denn ein weiterer Treffer hinter
    /// dem Abbruch ist moeglich und nicht gesichert.
    ///
    /// **Die Zahl allein traegt die zweite Auskunft nicht.** Bei einem Wert
    /// nahe der Grenze erraet der Nutzer den Abbruch noch, bei „1" nicht mehr;
    /// deshalb nennt [`Wert::als_text`] den Abbruch ausdruecklich, und die
    /// Grenze darin kommt aus [`HOECHSTENS_EINTRAEGE`] und steht nicht ein
    /// zweites Mal im Text.
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

impl Wert {
    /// Der Wert als Text, ohne seine Beschriftung.
    ///
    /// Die Fallunterscheidung ist vollstaendig und hat keinen Auffangzweig.
    /// Der Satz zu [`Wert::UeberGrenze`] traegt zwei Zahlen aus je einer
    /// Quelle: die Treffer aus dem Wert und die Grenze aus
    /// [`HOECHSTENS_EINTRAEGE`]. Keine von beiden steht ein zweites Mal im
    /// Text; dieselbe Regel, nach der `vorschaumodell::zu_gross_text` seine
    /// Grenze aus der Konstanten bildet.
    #[must_use]
    pub fn als_text(&self) -> String {
        match self {
            Wert::Zahl(zahl) => zahl.to_string(),
            Wert::UeberGrenze(gezaehlt) => {
                format!(
                    "mindestens {gezaehlt} (Lesung bei {HOECHSTENS_EINTRAEGE} Einträgen abgebrochen)"
                )
            }
            Wert::Vorhanden(true) => "ja".to_owned(),
            Wert::Vorhanden(false) => "nein".to_owned(),
            Wert::Text(text) => text.clone(),
            Wert::Titel(titel) => titel.join("\n"),
            Wert::Nicht => PLATZHALTER.to_owned(),
        }
    }
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

    /// Bucht `wie_viele` Dateioeffnungen und sagt, ob sie noch im Haushalt
    /// lagen.
    ///
    /// **Ganz oder gar nicht**, und darin liegt der Grund fuer die Zahl im
    /// Argument: der Baustein „juengste N" braucht N Oeffnungen fuer **eine**
    /// Antwort, und passt die letzte nicht mehr hinein, ist die halbe Antwort
    /// keine. Einzeln gebucht haette er die ersten Oeffnungen verbraucht und
    /// den Wert am Ende doch fallen lassen, und die verbrauchten fehlten den
    /// Zeilen darunter.
    ///
    /// `false` heisst wie bei [`Haushalt::leselauf_nehmen`]: es hat nicht
    /// stattgefunden, und der Zaehler bleibt stehen.
    #[must_use = "wer nicht hinsieht, oeffnet ueber die Grenze hinaus"]
    pub fn oeffnungen_nehmen(&mut self, wie_viele: u32) -> bool {
        let Some(danach) = self.oeffnungen.checked_add(wie_viele) else {
            return false;
        };
        if danach > HOECHSTENS_OEFFNUNGEN {
            return false;
        }
        self.oeffnungen = danach;
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
        assert!(!ort.traegt_platzhalter());
        assert!(Ortsangabe::wurzel().teile().is_empty());
        assert!(Ortsangabe::wurzel().hinter_dem_platzhalter().is_none());
    }

    /// Der Platzhalter zerlegt die Angabe in ein Stueck davor und eines
    /// dahinter, und beide Stuecke duerfen leer sein.
    #[test]
    fn ein_platzhalter_zerlegt_die_ortsangabe_in_zwei_haelften() {
        let jeder = Ortsangabe::aus_angabe("*").expect("ein Platzhalter allein ist zulaessig");
        assert!(jeder.teile().is_empty(), "vor dem Stern steht nichts");
        assert_eq!(
            jeder.hinter_dem_platzhalter(),
            Some(&[][..]),
            "hinter dem Stern steht nichts, und das ist etwas anderes als kein Stern"
        );

        let defekte = Ortsangabe::aus_angabe("*/issues").expect("die Angabe ist zulaessig");
        assert!(defekte.teile().is_empty());
        assert_eq!(
            defekte.hinter_dem_platzhalter(),
            Some(&["issues".to_owned()][..])
        );

        let mittendrin =
            Ortsangabe::aus_angabe("circles/*/planning").expect("die Angabe ist zulaessig");
        assert_eq!(mittendrin.teile(), ["circles"]);
        assert_eq!(
            mittendrin.hinter_dem_platzhalter(),
            Some(&["planning".to_owned()][..])
        );
    }

    /// Ein Stern **im** Stueck ist ein gewoehnlicher Name, ein zweites Stueck
    /// aus einem Stern ist ein Mangel.
    #[test]
    fn ein_zweiter_platzhalter_wird_abgewiesen_und_ein_stern_im_namen_nicht() {
        for angabe in ["*/*", "circles/*/issues/*", "*/*/planning"] {
            assert_eq!(
                Ortsangabe::aus_angabe(angabe),
                Err(Ortsmangel::MehrerePlatzhalter),
                "die Angabe {angabe:?} kommt durch"
            );
        }

        let sternchen = Ortsangabe::aus_angabe("a*b/c").expect("ein Stern im Namen ist ein Name");
        assert_eq!(sternchen.teile(), ["a*b", "c"]);
        assert!(!sternchen.traegt_platzhalter());
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
