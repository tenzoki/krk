//! Was eine neue Fassung von KRK an den von Hand gepflegten Ablagedateien
//! mitbringt, das die Datei des Nutzers noch nicht fuehrt.
//!
//! ```text
//! AUSLIEFERUNGSTEXT (einkompiliert) ──toml::Table──┐
//!                                                  ├──> Neuerungen ──> Bestand
//! ~/Library/.../KRK/<datei> ──Zugang::laden────────┘
//! ```
//!
//! Der Vergleich braucht weder das Buendel noch das Netz: die
//! Auslieferungsfassung steht ueber `include_str!` einkompiliert da, die
//! Nutzerdatei liegt auf der Platte. Gemeldet werden die Namen in **beide**
//! Richtungen — was die Auslieferungsfassung fuehrt und die Nutzerdatei nicht,
//! und umgekehrt.
//!
//! # Hier wird nichts geschrieben
//!
//! Dieses Modul liest und vergleicht. Keine der verglichenen Dateien wird
//! angefasst, und das ist keine Sparsamkeit, sondern die Zusage der Runde: sie
//! meldet und ergaenzt nicht. Insbesondere nimmt sie [`Ersatz::Nichts`] fuer
//! `readers.toml` nichts zurueck — die Begruendung dort steht unter „Zweite
//! Abweichung" im Kopf von [`super::leseprofile`], und sie gilt unveraendert:
//! die Auslieferungsfassung einzusetzen hiesse, dem Nutzer Profile
//! unterzuschieben, die er vielleicht gerade herausgenommen hat.
//!
//! [`Ersatz::Nichts`]: super::Ersatz::Nichts
//!
//! # Die vierte je Datei beantwortete Frage
//!
//! [`super::pfade`] beantwortet drei Fragen je Ablagedatei, jede als
//! vollstaendige Fallunterscheidung ohne Auffangzweig: Format, Leerbefund,
//! Ersatz. [`Vergleichsform::fuer`] ist die vierte und von derselben Bauart —
//! eine achte Ablagedatei haelt auch hier den Bau an.
//!
//! **Sie steht trotzdem nicht in [`super::pfade`], und das ist keine
//! Nachlaessigkeit.** „Ein Eintrag ist ein `[[profil]]`, benannt durch sein
//! Feld `name`" ist eine Aussage ueber den **Inhalt** einer Datei; die Ablage
//! kennt Pfad, Format und Fehlerbehandlung und nicht den Inhalt, so der Kopf
//! von [`super`]. Die Frage gehoert deshalb in das Modul, das sie braucht.
//!
//! # Verglichen wird ueber `toml::Table`
//!
//! Und nicht ueber die getypten Strukturen. Der Grund ist `settings.toml`:
//! `Einstellungsdatei` haelt lauter `Option`-Felder, und „welchen obersten
//! Schluessel nennt die Datei" waere aus ihr nur ueber eine Feldliste daneben
//! zu beantworten — genau die Falle, die dieser Baum an `Kommando::KENNUNGEN`
//! schon einmal bezahlt hat. Ein `toml::Table` beantwortet die Frage fuer jede
//! verglichene Datei aus ihrem Bestand, ueber einen Weg, und
//! [`Zugang::laden`](super::Zugang::laden) geht dabei durch dieselbe Tuer wie
//! jedes andere Laden, mit derselben Schadensbehandlung.
//!
//! # Was nicht dasteht, wird nicht verglichen
//!
//! Eine Nutzerdatei, die es nicht gibt, liefert keine Neuerung. Auf einer
//! frischen Installation gibt es `keymap.toml` gar nicht; wer sie als leere
//! Datei liest, meldet dem Nutzer jede ausgelieferte Funktion als Neuerung.
//! Der Ausweg ist nicht die genauere Naeherung, sondern die engere Frage:
//! verglichen wird, was dasteht. Dasselbe gilt fuer eine Datei, die ihren
//! Bestand nicht hergegeben hat — was in ihr steht, ist unbekannt, und
//! „fuehrt diesen Eintrag nicht" waere darueber eine Behauptung und keine
//! Auskunft. Beide Faelle behalten ihre Zeile im [`Bestand`] und tragen ihren
//! [`Befund`]; das Blatt auf Abruf nennt den vollen Pfad jeder verglichenen
//! Datei, auch der, die es nicht gibt.
//!
//! # Die Gegenrichtung fuellt sich bauartbedingt nur bei `readers.toml`
//!
//! Ein `[[profil]]` mit einem eigenen Namen ist dort der gewoehnliche Fall.
//! Bei `settings.toml` und `keymap.toml` ist ein Eintrag, den die
//! Auslieferungsfassung nicht kennt, dagegen ein Schaden und keine Abweichung:
//! `Einstellungsdatei` traegt `deny_unknown_fields`, und `Belegung::bauen`
//! weist eine unbekannte Kennung ab. Eine solche Datei gibt ihrem eigentlichen
//! Leser ihren Bestand gar nicht her, KRK arbeitet auf dem
//! Auslieferungszustand weiter — und sie kommt deshalb hier nicht bis zum
//! Vergleich, sondern traegt [`Befund::Ersetzt`]. Welche Datei eigene
//! Eintraege fuehren darf, sagt [`eigene_eintraege_moeglich`] als
//! vollstaendige Fallunterscheidung. Die Zusage „in beide Richtungen" ist
//! damit an einer Datei erfuellt und an zweien leer, mit dem Grund daneben.
//!
//! **Gehalten wird die Begruendung von der Probe
//! `ein_unbekannter_eintrag_macht_settings_und_keymap_beschaedigt`** in
//! `krk-core/tests/ablage.rs`. Ohne sie waere sie eine Behauptung in diesem
//! Kopf, und der erste, der `deny_unknown_fields` entfernt oder die unbekannte
//! Kennung durchlaesst, machte sie still falsch: hier stuende dann eine Datei
//! als beschaedigt da, die ihr Leser laengst annimmt.

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use super::{Datei, Geladen, Zugang, einstellungen, leseprofile, pfade};
use crate::tasten::belegung;

/// Was bei einer Ablagedatei ein Eintrag ist.
///
/// **Vollstaendig ueber [`Datei`] und ohne Auffangzweig**, wie
/// [`Datei::format`], [`Datei::leerbefund`] und [`Datei::ersatz`] daneben; der
/// Modulkopf sagt, warum die Frage hier wohnt und nicht bei jenen dreien.
///
/// Zwei Gestalten reichen fuer jede heute verglichene Datei: `settings.toml`
/// nennt ihre Eintraege als oberste Schluessel, `readers.toml` und
/// `keymap.toml` als Tabellenfolge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vergleichsform {
    /// Die obersten Schluessel der Datei sind ihre Eintraege, und der Name
    /// eines Eintrags ist der Schluessel selbst.
    ///
    /// Die Form von `settings.toml`, die heute genau einen fuehrt: `terminal`.
    ObersteSchluessel,
    /// Eine Tabellenfolge unter einem Tischnamen; jeder Tisch nennt seinen
    /// Namen in einem Schluesselfeld.
    ///
    /// Die Form von `readers.toml` (`[[profil]]` mit `name`) und `keymap.toml`
    /// (`[[funktion]]` mit `id`).
    Tischfolge {
        /// Der Name der Tabellenfolge, ohne die doppelten Klammern.
        tisch: &'static str,
        /// Das Feld eines Tisches, das seinen Namen traegt.
        schluessel: &'static str,
    },
    /// Diese Datei wird nicht verglichen.
    ///
    /// Der Wert jeder Datei, die KRK selbst schreibt, und der zwei Zettel: eine
    /// Nutzerfassung kann dort nicht hinter der Auslieferungsfassung
    /// zurueckliegen, denn es gibt keine ausgelieferte Fassung, hinter der sie
    /// zurueckbliebe.
    Nicht,
}

impl Vergleichsform {
    /// Was bei dieser Ablagedatei ein Eintrag ist.
    pub const fn fuer(welche: Datei) -> Self {
        match welche {
            Datei::Einstellungen => Vergleichsform::ObersteSchluessel,
            Datei::Leser => Vergleichsform::Tischfolge {
                tisch: "profil",
                schluessel: "name",
            },
            Datei::Belegung => Vergleichsform::Tischfolge {
                tisch: "funktion",
                schluessel: "id",
            },
            Datei::Lesezeichen | Datei::Sitzung | Datei::Merker | Datei::Zettel(_) => {
                Vergleichsform::Nicht
            }
        }
    }

    /// Ob diese Form ueberhaupt zu einem Vergleich fuehrt.
    const fn vergleicht(self) -> bool {
        !matches!(self, Vergleichsform::Nicht)
    }

    /// Die Namen der Eintraege, die eine Tabelle in dieser Form fuehrt, in der
    /// Reihenfolge der Datei.
    ///
    /// Ein Tisch ohne sein Schluesselfeld faellt heraus: er traegt keinen
    /// Namen, unter dem er sich melden liesse. Ein solcher Eintrag ist in einer
    /// Nutzerdatei ohnehin ein Schaden, und die Datei kommt dann gar nicht bis
    /// hierher; in der eingebetteten Fassung gibt es ihn nicht.
    fn namen(self, tabelle: &toml::Table) -> Vec<String> {
        match self {
            Vergleichsform::ObersteSchluessel => tabelle.keys().cloned().collect(),
            Vergleichsform::Tischfolge { tisch, schluessel } => tabelle
                .get(tisch)
                .and_then(toml::Value::as_array)
                .map(|folge| {
                    folge
                        .iter()
                        .filter_map(|eintrag| eintrag.as_table()?.get(schluessel)?.as_str())
                        .map(str::to_owned)
                        .collect()
                })
                .unwrap_or_default(),
            // Erreicht wird dieser Zweig nicht: [`erheben`] ueberspringt eine
            // Datei ohne Vergleich, bevor es nach Namen fragt. Er steht da,
            // weil die Fallunterscheidung vollstaendig ist.
            Vergleichsform::Nicht => Vec::new(),
        }
    }
}

/// Was der Vergleich an einer Ablagedatei vorgefunden hat.
///
/// **Drei Werte, vollstaendig und ohne Auffangzweig.** Die zwei letzten sind
/// keine Fehler, sondern Auskuenfte: sie sagen, warum die zwei Namenslisten
/// daneben leer sind, und unterscheiden „kein Unterschied" von „nicht
/// verglichen". Ohne sie hiesse beides dasselbe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Befund {
    /// Die Nutzerdatei stand da und hat ihren Bestand hergegeben; die zwei
    /// Namenslisten tragen das Ergebnis.
    Verglichen,
    /// Die Nutzerdatei liegt nicht auf der Platte.
    ///
    /// Der gewoehnliche Fall bei `keymap.toml`: KRK legt sie nie an, und wer
    /// seine Belegung nie geaendert hat, hat sie nicht.
    Fehlt,
    /// Die Nutzerdatei liegt da, hat ihren Bestand aber nicht hergegeben.
    ///
    /// Zwei Wege fuehren hierher. Der eine ist das Laden selbst: es traegt eine
    /// [`Ersetzung`](super::Ersetzung), und die hat der Nutzer beim Start schon
    /// gelesen; dieses Modul meldet sie kein zweites Mal. Der andere ist ein
    /// eigener Eintrag in einer Datei, die keine fuehren darf — siehe
    /// [`eigene_eintraege_moeglich`] und den Modulkopf.
    Ersetzt,
}

/// Der Unterschied zwischen der Auslieferungsfassung einer Ablagedatei und der
/// Datei des Nutzers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Neuerungen {
    /// Welche Ablagedatei.
    pub welche: Datei,
    /// Ihr voller Pfad, so wie das Blatt auf Abruf ihn nennt.
    ///
    /// Voll und nicht gekuerzt: der Nutzer soll die Datei aufmachen koennen.
    /// Die gekuerzte Form nimmt allein die Startzeile, ueber
    /// [`pfade::gekuerzt_fuer_anzeige`].
    pub pfad: PathBuf,
    /// Was der Vergleich an dieser Datei vorgefunden hat.
    pub befund: Befund,
    /// Die Namen, die allein die Auslieferungsfassung fuehrt.
    ///
    /// Das ist die Hinrichtung: was diese Fassung von KRK mitbringt und die
    /// Datei des Nutzers noch nicht kennt.
    pub nur_ausgeliefert: Vec<String>,
    /// Die Namen, die allein die Nutzerdatei fuehrt.
    ///
    /// Die Gegenrichtung. Sie fuellt sich bauartbedingt nur bei
    /// `readers.toml`; der Modulkopf sagt, warum.
    pub nur_beim_nutzer: Vec<String>,
}

impl Neuerungen {
    /// Ob diese Datei ueberhaupt einen Unterschied traegt, in welcher Richtung
    /// auch immer.
    #[must_use]
    pub fn traegt_unterschied(&self) -> bool {
        !self.nur_ausgeliefert.is_empty() || !self.nur_beim_nutzer.is_empty()
    }
}

/// Der Unterschied ueber alle Ablagedateien, die einen tragen koennen.
///
/// Eine Zeile je Datei mit einer [`Vergleichsform`], in der Reihenfolge von
/// [`Datei::ALLE`] — auch fuer die, die nicht dasteht, denn das Blatt auf Abruf
/// nennt ihren Pfad trotzdem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bestand {
    ordner: PathBuf,
    dateien: Vec<Neuerungen>,
}

impl Bestand {
    /// Der Ablageordner, in dem die verglichenen Dateien liegen.
    #[must_use]
    pub fn ordner(&self) -> &Path {
        &self.ordner
    }

    /// Die Zeilen, in der Reihenfolge von [`Datei::ALLE`].
    #[must_use]
    pub fn dateien(&self) -> &[Neuerungen] {
        &self.dateien
    }

    /// Die Zeile einer bestimmten Ablagedatei, falls sie verglichen wird.
    #[must_use]
    pub fn fuer(&self, welche: Datei) -> Option<&Neuerungen> {
        self.dateien.iter().find(|zeile| zeile.welche == welche)
    }

    /// Ob irgendeine Datei einen Unterschied traegt.
    #[must_use]
    pub fn traegt_unterschied(&self) -> bool {
        self.dateien.iter().any(Neuerungen::traegt_unterschied)
    }
}

/// Erhebt den Unterschied fuer jede Ablagedatei, die einen tragen kann.
///
/// Scheitert nie, wie [`Zugang::laden`](super::Zugang::laden) darunter: eine
/// fehlende oder beschaedigte Datei liefert eine Zeile mit ihrem [`Befund`] und
/// ohne Namen.
///
/// **Die [`Ersetzung`](super::Ersetzung) aus dem Laden faellt hier absichtlich
/// weg.** Jede verglichene Datei ist im selben Start schon von ihrem
/// eigentlichen Leser geladen worden — `keymap.toml` von
/// [`belegung::laden`], `settings.toml` von
/// [`einstellungen::laden`], `readers.toml` von
/// [`leseprofile::laden`] —, und deren Meldung ist
/// die, die der Nutzer liest. Eine zweite, wortgleiche daneben waere keine
/// zweite Auskunft.
#[must_use]
pub fn erheben(zugang: &Zugang<'_>) -> Bestand {
    let mut dateien = Vec::new();
    for welche in Datei::ALLE {
        let form = Vergleichsform::fuer(welche);
        if !form.vergleicht() {
            continue;
        }
        let Some(ausgeliefert) = auslieferung(welche) else {
            // Eine Datei mit Vergleichsform, aber ohne eingebettete Fassung
            // liesse sich nicht vergleichen. Der Uebersetzer haelt das Paar
            // nicht; die Probe
            // `jede_verglichene_ablagedatei_hat_eine_eingebettete_fassung`
            // haelt es.
            debug_assert!(
                false,
                "{} traegt eine Vergleichsform, aber keine eingebettete Auslieferungsfassung",
                welche.dateiname()
            );
            continue;
        };
        dateien.push(eine_datei(zugang, welche, form, ausgeliefert));
    }
    Bestand {
        ordner: zugang.ort.wurzel().to_path_buf(),
        dateien,
    }
}

/// Der Unterschied an einer einzelnen Ablagedatei.
fn eine_datei(
    zugang: &Zugang<'_>,
    welche: Datei,
    form: Vergleichsform,
    ausgeliefert: &toml::Table,
) -> Neuerungen {
    let pfad = zugang.pfad(welche);
    let ohne_namen = |befund| Neuerungen {
        welche,
        pfad: zugang.pfad(welche),
        befund,
        nur_ausgeliefert: Vec::new(),
        nur_beim_nutzer: Vec::new(),
    };
    // **Gefragt wird vor dem Laden und nicht danach.** Eine fehlende Datei und
    // eine gueltige Datei ohne einen einzigen obersten Schluessel sind fuer
    // `Zugang::laden` dasselbe Ergebnis, und hier sind sie zweierlei: die eine
    // wird nicht verglichen, die andere fuehrt keinen Eintrag. Ein Fehler beim
    // Fragen ist keine Antwort „sie fehlt": die Datei liegt dann da und ist
    // nur nicht zu erreichen, und das Laden darunter sagt es genauer.
    if pfad.try_exists().is_ok_and(|steht_da| !steht_da) {
        return ohne_namen(Befund::Fehlt);
    }
    let geladen: Geladen<toml::Table> = zugang.laden(welche);
    if geladen.ersetzung.is_some() {
        return ohne_namen(Befund::Ersetzt);
    }
    let ausgelieferte_namen = form.namen(ausgeliefert);
    let eigene_namen = form.namen(&geladen.wert);
    let nur_beim_nutzer = ohne(&eigene_namen, &ausgelieferte_namen);
    // **Ein eigener Eintrag, wo keiner sein darf, ist ein Schaden und keine
    // Abweichung.** Der eigentliche Leser dieser Datei weist sie ab, KRK
    // arbeitet auf dem Auslieferungszustand weiter, und was in ihr steht, ist
    // fuer den laufenden Betrieb ohne Belang. Beide Richtungen waeren dann eine
    // Aussage ueber eine Datei, mit der niemand arbeitet; der Modulkopf traegt
    // die Begruendung, und eine Probe haelt sie.
    if !nur_beim_nutzer.is_empty() && !eigene_eintraege_moeglich(welche) {
        return ohne_namen(Befund::Ersetzt);
    }
    Neuerungen {
        welche,
        pfad,
        befund: Befund::Verglichen,
        nur_ausgeliefert: ohne(&ausgelieferte_namen, &eigene_namen),
        nur_beim_nutzer,
    }
}

/// Ob die Nutzerfassung dieser Ablagedatei Eintraege fuehren darf, die die
/// Auslieferungsfassung nicht kennt.
///
/// **Die dritte vollstaendige Fallunterscheidung dieses Moduls**, und sie
/// beantwortet eine andere Frage als die zwei darueber: nicht, was ein Eintrag
/// ist, und nicht, woran er gemessen wird, sondern ob ein unbekannter ueberhaupt
/// zulaessig ist.
///
/// Allein `readers.toml` sagt ja: ein `[[profil]]` mit einem eigenen Namen ist
/// dort der gewoehnliche Fall und der Zweck der Datei. `settings.toml` traegt
/// an `Einstellungsdatei` ein `deny_unknown_fields`, und `keymap.toml` laeuft
/// durch `Belegung::bauen`, das eine unbekannte Kennung als
/// `Belegungsfehler::UnbekannteFunktion` abweist. Die uebrigen werden gar nicht
/// verglichen; ihre Antwort steuert nichts und steht da, weil die
/// Fallunterscheidung vollstaendig ist.
const fn eigene_eintraege_moeglich(welche: Datei) -> bool {
    match welche {
        Datei::Leser => true,
        Datei::Belegung
        | Datei::Einstellungen
        | Datei::Lesezeichen
        | Datei::Sitzung
        | Datei::Merker
        | Datei::Zettel(_) => false,
    }
}

/// Die Namen aus `diese`, die in `jene` nicht vorkommen, in ihrer Reihenfolge
/// und jeder hoechstens einmal.
fn ohne(diese: &[String], jene: &[String]) -> Vec<String> {
    let mut heraus: Vec<String> = Vec::new();
    for name in diese {
        if !jene.contains(name) && !heraus.contains(name) {
            heraus.push(name.clone());
        }
    }
    heraus
}

/// Die eingebettete Auslieferungsfassung der Belegung, zerlegt.
static AUSGELIEFERTE_BELEGUNG: LazyLock<toml::Table> =
    LazyLock::new(|| eingebettet(belegung::AUSLIEFERUNGSTEXT, "die Auslieferungsbelegung"));

/// Die eingebettete Auslieferungsfassung der Einstellungen, zerlegt.
static AUSGELIEFERTE_EINSTELLUNGEN: LazyLock<toml::Table> = LazyLock::new(|| {
    eingebettet(
        einstellungen::AUSLIEFERUNGSTEXT,
        "die Auslieferungsfassung der Einstellungen",
    )
});

/// Die eingebettete Auslieferungsfassung der Leseprofile, zerlegt.
static AUSGELIEFERTE_LESEPROFILE: LazyLock<toml::Table> = LazyLock::new(|| {
    eingebettet(
        leseprofile::AUSLIEFERUNGSTEXT,
        "die Auslieferungsfassung der Leseprofile",
    )
});

/// Zerlegt einen eingebetteten Auslieferungstext.
///
/// Bricht ab, wenn er kein gueltiges TOML ist — dieselbe Bauform wie die drei
/// vorhandenen `LazyLock`-Werte neben den Konstanten. Ein Text, der schon beim
/// Uebersetzen im Programm steht, ist entweder gueltig oder der Bau ist kaputt;
/// dazwischen gibt es nichts zu behandeln.
fn eingebettet(text: &str, was: &str) -> toml::Table {
    toml::from_str(text).unwrap_or_else(|fehler| panic!("{was} ist kein gueltiges TOML: {fehler}"))
}

/// Die eingebettete Auslieferungsfassung, gegen die eine Ablagedatei gehalten
/// wird.
///
/// **Die zweite vollstaendige Fallunterscheidung dieses Moduls, und sie gehoert
/// zur ersten.** Eine Datei mit einer [`Vergleichsform`] ohne eingebettete
/// Fassung liesse sich nicht vergleichen, und eine mit
/// [`Vergleichsform::Nicht`] braucht keine. Der Uebersetzer haelt beide
/// Haelften je fuer sich vollstaendig, ihre Paarung haelt er nicht; die haelt
/// die Probe `jede_verglichene_ablagedatei_hat_eine_eingebettete_fassung` in
/// `krk-core/tests/ablage.rs`.
fn auslieferung(welche: Datei) -> Option<&'static toml::Table> {
    match welche {
        Datei::Belegung => Some(&AUSGELIEFERTE_BELEGUNG),
        Datei::Einstellungen => Some(&AUSGELIEFERTE_EINSTELLUNGEN),
        Datei::Leser => Some(&AUSGELIEFERTE_LESEPROFILE),
        Datei::Lesezeichen | Datei::Sitzung | Datei::Merker | Datei::Zettel(_) => None,
    }
}

/// Die eine Zeile, die der Nutzer beim ersten Start einer neuen Fassung liest —
/// oder `None`, wenn es nichts zu melden gibt.
///
/// **Genannt werden allein die Dateien mit einem Unterschied.** Die Zeile steht
/// in der Statuszeile, ist einzeilig und kuerzt rechts; drei Nullen darin
/// verdraengten die Auskunft, um die es geht. Je Datei mit Unterschied bleibt
/// eine Zahl damit erfuellt.
///
/// **Gezaehlt wird die Hinrichtung.** Die Zeile sagt, was diese Fassung
/// mitbringt; ein eigenes Profil des Nutzers ist keine Neuerung dieser Fassung
/// und gehoert nicht in einen Satz, der sie ankuendigt. Die Gegenrichtung
/// zeigt [`blatttext`].
///
/// Der Ordner steht in KRKs Meldungsform, also mit `~` statt des
/// Benutzerverzeichnisses ([`pfade::gekuerzt_fuer_anzeige`]). Das
/// Benutzerverzeichnis kommt als Argument herein und wird hier nicht erfragt —
/// dieselbe Erwaegung wie dort: so ist die Funktion ohne Zugriff auf das echte
/// Benutzerverzeichnis pruefbar.
#[must_use]
pub fn startzeile(bestand: &Bestand, benutzerverzeichnis: Option<&Path>) -> Option<String> {
    let teile: Vec<String> = bestand
        .dateien
        .iter()
        .filter(|zeile| !zeile.nur_ausgeliefert.is_empty())
        .map(|zeile| {
            let zahl = zeile.nur_ausgeliefert.len();
            let wort = if zahl == 1 { "Eintrag" } else { "Einträge" };
            format!("{zahl} {wort} in {}", zeile.welche.dateiname())
        })
        .collect();
    if teile.is_empty() {
        return None;
    }
    let ordner = pfade::gekuerzt_fuer_anzeige(&bestand.ordner, benutzerverzeichnis);
    Some(format!(
        "Neu in dieser Fassung: {}. Ihre Dateien liegen unter {ordner}.",
        teile.join(", ")
    ))
}

/// Der Text des Blattes auf Abruf: je verglichener Datei ihr voller Pfad und
/// der Unterschied in beide Richtungen.
///
/// **Jede verglichene Datei bekommt ihren Absatz, auch die ohne Unterschied und
/// die, die es nicht gibt.** Das Blatt beantwortet die Frage „was ist mit
/// meinen Dateien", und „an dieser ist nichts" ist darauf eine Antwort. Der
/// volle Pfad steht dabei in jedem Absatz: er ist der Grund, aus dem der Nutzer
/// das Blatt aufmacht.
///
/// Der Schlusssatz sagt, worauf sich der Text bezieht: auf den Stand vom Start.
/// KRK liest diese Dateien im Betrieb nicht neu, und ein Blatt, das die Platte
/// neu laese, zeigte einen Bestand, mit dem die laufende Anwendung gar nicht
/// arbeitet.
#[must_use]
pub fn blatttext(bestand: &Bestand) -> String {
    let mut text = String::new();
    for zeile in &bestand.dateien {
        text.push_str(zeile.welche.dateiname());
        text.push('\n');
        text.push_str(&zeile.pfad.display().to_string());
        text.push('\n');
        match zeile.befund {
            Befund::Fehlt => {
                text.push_str(
                    "Diese Datei liegt nicht in Ihrer Ablage; verglichen wird nur, was dasteht.\n",
                );
            }
            Befund::Ersetzt => {
                // Woertlich das Wort, das `Grund::beschreibung` dem Nutzer
                // schon in der Statuszeile hinschreibt: eine Sache, eine
                // Formulierung.
                text.push_str("Diese Datei ist beschädigt und wird deshalb nicht verglichen.\n");
            }
            Befund::Verglichen => {
                text.push_str(&namenszeile(
                    "Neu in dieser Fassung",
                    &zeile.nur_ausgeliefert,
                ));
                text.push_str(&namenszeile("Nur in Ihrer Datei", &zeile.nur_beim_nutzer));
            }
        }
        text.push('\n');
    }
    text.push_str("Gezeigt ist der Stand vom Start; KRK liest diese Dateien im Betrieb nicht neu.");
    text
}

/// Eine Zeile des Blattes: eine Ueberschrift und die Namen dahinter.
///
/// Eine leere Liste bekommt einen Gedankenstrich und faellt nicht weg: der
/// Nutzer soll sehen, dass die Richtung gefragt und leer war, statt zu raten,
/// ob sie geprueft wurde. Eine lange Liste geht durch [`gekuerzt`]:
/// `keymap.toml` kann jede ausgelieferte Funktion nennen.
fn namenszeile(ueberschrift: &str, namen: &[String]) -> String {
    if namen.is_empty() {
        return format!("{ueberschrift}: —\n");
    }
    format!("{ueberschrift}: {}\n", gekuerzt(namen.to_vec()).join(", "))
}

// ----------------------------------------------------------------------
// Der eine Kuerzer fuer lange Namenslisten
// ----------------------------------------------------------------------

/// Hoechstens so viele Glieder einer Liste stehen einzeln da.
///
/// Ein Wert fuer jede Liste, die KRK zeigt. Eine Kopie ueber einen Ordner
/// ohne Leserechte kann
/// Tausende uebersprungene Eintraege erzeugen, und `keymap.toml` nennt jede
/// ausgelieferte Funktion; eine Liste, die den Schirm ueberragt, ist keine
/// Auskunft mehr. Der Rest wird gezaehlt.
pub const HOECHSTENS_EINZELN: usize = 12;

/// Die eine Schreibweise fuer eine Zahl in KRKs Oberflaeche.
///
/// Tausenderpunkte. Sie stand bis zum 260910 als `zahl` in
/// `krk_ui::kommandos::operationen` und ist mit [`gekuerzt`] hierher gezogen,
/// weil dieser Kuerzer sie braucht und der Kern die Oberflaeche nicht rufen
/// kann. Der alte Ort holt sie ueber `pub(crate) use` zurueck; seine Rufer
/// haben davon nichts gemerkt, und eine zweite Fassung ist nicht entstanden.
#[must_use]
pub fn zahl(wert: usize) -> String {
    let ziffern = wert.to_string();
    let mut aus = String::with_capacity(ziffern.len() + ziffern.len() / 3);
    for (stelle, ziffer) in ziffern.chars().enumerate() {
        if stelle > 0 && (ziffern.len() - stelle).is_multiple_of(3) {
            aus.push('.');
        }
        aus.push(ziffer);
    }
    aus
}

/// Kuerzt eine lange Liste auf [`HOECHSTENS_EINZELN`] Glieder und beziffert
/// den Rest in einem weiteren Glied.
///
/// **Der Wortlaut „… und N weitere" steht hier und sonst nirgends.** Zwei
/// Rufer teilen ihn: die Abschlussliste der uebersprungenen Eintraege in
/// `krk_ui::kommandos::operationen` und die Namenszeile des Blattes nebenan.
/// Wie viele Rufer es sind, haelt die Probe
/// `der_kuerzer_langer_namenslisten_hat_genau_zwei_rufer` in
/// `krk-core/tests/baum.rs`; eine dritte Fassung des Wortlauts faellt dort
/// auf, statt still danebenzustehen.
///
/// Eine Liste bis zur Grenze kommt unveraendert zurueck, auch die leere.
#[must_use]
pub fn gekuerzt(mut glieder: Vec<String>) -> Vec<String> {
    let ganz = glieder.len();
    if ganz > HOECHSTENS_EINZELN {
        glieder.truncate(HOECHSTENS_EINZELN);
        glieder.push(format!("… und {} weitere", zahl(ganz - HOECHSTENS_EINZELN)));
    }
    glieder
}
