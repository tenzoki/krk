//! Die Gestalt von `readers.toml` und der eine Pruefschritt dahinter.
//!
//! Hier steht die Datei so, wie sie dasteht: Zeichenketten, wo spaeter ein
//! [`Regex`] steht, und Schluessel, die fehlen duerfen. [`pruefen`] macht
//! daraus die geprueften Werte aus dem Elternmodul und die Meldungen ueber
//! das, was dabei liegen blieb.
//!
//! ```text
//! readers.toml ──toml::from_str──> Profildatei ──pruefen──> (Profile, Meldungen)
//! ```
//!
//! Denselben Zuschnitt zieht `ablage::einstellungen` zwischen
//! `Einstellungsdatei` und `Einstellungen`, und aus demselben Grund: die
//! gelesene Gestalt und der gepruefte Wert sind zwei verschiedene Dinge, und
//! ein Typ fuer beide haette an jeder Lesestelle die Frage offen gelassen, ob
//! das Muster schon uebersetzt ist.
//!
//! # Der eine Ort, an dem ein Muster zu einem Ausdruck wird
//!
//! [`pruefen`] uebersetzt jedes Muster **einmal** und behaelt die uebersetzte
//! Fassung. Das ist keine Sparsamkeit, sondern die Bedingung dafuer, dass die
//! Meldung ueber ein unuebersetzbares Muster einmal beim Start erscheint und
//! nicht bei jeder Auswahl eines Ordners; der Modulkopf des Elternmoduls
//! schreibt es unter „Warum jede Pruefung beim Laden laeuft" aus. Eine zweite
//! Uebersetzungsstelle daneben waere zugleich eine zweite Meldungsstelle.
//!
//! # Wo `deny_unknown_fields` steht und wo nicht
//!
//! An [`Profildatei`], an [`Zeilendatei`] und an jedem der vier
//! Bausteintische; allein [`Profilblock`] traegt ihn nicht, und das kostet
//! nichts: ein verschriebenes `pfad` laesst das Profil ohne Pfadmuster und
//! ohne Kennzeichen zurueck, und genau das weist [`pruefen`] mit einer Meldung
//! ab.
//!
//! **An der Zeile stand er bis zum 260824 nicht**, weil ihre Bausteinhaelfte
//! ueber `#[serde(flatten)]` in eine unmarkierte Auswahl lief und die beiden
//! Angaben einander ausschliessen. Der Preis dafuer war doppelt: ein
//! zusaetzlicher Schluessel neben der Beschriftung fiel nicht auf, und zwei
//! Bausteintische in einer Zeile wurden schweigend angenommen, wobei der obere
//! gewann und der untere wegfiel (`issues/260824-1216_*_zwei-bausteintische-…`).
//! Beides ist mit den vier benannten Feldern weg, und die Datei sieht dabei
//! aus wie zuvor.
//!
//! # Was abgewiesen wird, und wie weit
//!
//! **Drei** Reichweiten, und der Unterschied ist die Antwort darauf, was ohne
//! das abgewiesene Stueck noch Sinn ergibt:
//!
//! - **Die ganze Datei faellt weg**, wenn `serde` sie nicht in diese Gestalt
//!   bringt: ein unbekannter Schluessel an einer der sechs Stellen mit
//!   `deny_unknown_fields`, eine Zahl ausserhalb ihres Bereichs, ein
//!   Tischname, den es nicht gibt. Die Datei gilt dann nach C1.6 als
//!   beschaedigt, wird beiseitegelegt, und KRK arbeitet ohne jedes Profil
//!   weiter. **Das ist die weiteste der drei Reichweiten, und ein Buchstaben-
//!   dreher in einem Bausteintisch faellt in sie und nicht in die kleinste.**
//!   Die Meldung stammt von `serde` und nicht von hier; sie nennt seit dem
//!   260824 den Schluessel und die erwarteten Namen, denn ohne die unmarkierte
//!   Auswahl darueber wird die Meldung des Tisches nicht mehr verworfen
//!   (`issues/260824-1217_*_ein-tippfehler-in-einem-bausteintisch-…`).
//! - **Das ganze Profil faellt weg**, wenn eines seiner beiden
//!   Erkennungsmuster sich nicht uebersetzen laesst (C2.7) oder wenn es
//!   keines von beiden nennt. Ein Profil, das seinen Ort nicht erkennt, ist
//!   nicht halb brauchbar, sondern gar nicht. Die uebrigen Profile bleiben
//!   unberuehrt.
//! - **Die Zeile behaelt ihre Beschriftung und verliert ihren Baustein**, wenn
//!   sie nicht genau einen der vier Bausteintische nennt (C3), wenn ein Muster
//!   darin sich nicht uebersetzen laesst, wenn das Feldmuster nicht genau eine
//!   Fanggruppe traegt (C3.10) oder wenn die Ortsangabe schon am Text aus dem
//!   erkannten Ordner herausfuehrt (C3.13, erste Haelfte). Die Zeile steht dann
//!   in jeder Zusammenfassung mit ihrem Platzhalter, und die uebrigen Zeilen
//!   bleiben unberuehrt (C3.12).
//!
//! Die zweite und die dritte Reichweite melden aus [`pruefen`], und **jede
//! Meldung nennt den Profilnamen, bei einer Zeile deren Beschriftung, und den
//! Grund**. Die erste kann das nicht: dort ist noch kein Profil gelesen, und
//! die Zeilennummer der Meldung ist alles, was `serde` an dieser Stelle hat.
//!
//! Eine Zahl ueber [`HOECHSTENS_JUENGSTE`] wird **gekappt und nicht
//! abgewiesen** (C6.3): sie ist keine falsche Angabe, sondern eine, die mehr
//! verlangt, als die Zusammenfassung hergibt.

use regex::Regex;
use serde::Deserialize;

use super::{Baustein, HOECHSTENS_JUENGSTE, Ortsangabe, Profil, Profile, Zeile};

// ---------------------------------------------------------------------------
// Die Gestalt der Datei
// ---------------------------------------------------------------------------

/// Die Gestalt von `readers.toml` und `default-readers.toml`, unveraendert.
///
/// **Bewusst ohne `Serialize`.** KRK schreibt diese Datei nach ihrer Anlage
/// beim ersten Start nie wieder; ein Serialisierungsweg waere der zweite Weg
/// zu ihr, und er hinterliesse sie ohne ihre Kommentarzeilen. Dieselbe Wahl
/// trifft `ablage::einstellungen::Einstellungsdatei`.
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Profildatei {
    /// Die Profilbloecke in der Reihenfolge der Datei.
    ///
    /// `default` und nicht Pflicht: eine Datei ohne einen einzigen Block ist
    /// gueltig und heisst „keine Profile" (C1.5).
    pub profil: Vec<Profilblock>,
}

/// Ein `[[profil]]`-Block, wie er in der Datei steht.
#[derive(Debug, Deserialize)]
pub struct Profilblock {
    /// Der Name, nur fuer Meldungen.
    pub name: String,
    /// Das Muster auf dem vollen Pfad des ausgewaehlten Ordners.
    pub pfad: Option<String>,
    /// Das Muster auf den Namen der Eintraege im ausgewaehlten Ordner.
    pub kennzeichen: Option<String>,
    /// Die Zeilen. Ein Profil ohne Zeilen ist zulaessig und zeigt allein die
    /// Kopfzeile aus Name und Pfad.
    #[serde(default)]
    pub zeile: Vec<Zeilendatei>,
}

/// Eine `[[profil.zeile]]`, wie sie in der Datei steht.
///
/// Die vier Bausteintische stehen **neben** der Beschriftung und nicht
/// geschachtelt darunter; die Gestalt der Datei ist damit dieselbe, die sie
/// vor dem 260824 hatte. Genau einer von ihnen muss dastehen, und diese Frage
/// beantwortet [`Zeilendatei::zerlegen`] und nicht `serde`.
///
/// **Vier benannte Felder statt einer unmarkierten Auswahl hinter
/// `#[serde(flatten)]`**, und der Grund steht im Modulkopf unter „Wo
/// `deny_unknown_fields` steht und wo nicht": die unmarkierte Auswahl konnte
/// weder zwei Tische in einer Zeile bemerken noch die Meldung eines
/// verschriebenen Schluessels durchlassen. Der Preis ist ein Feld mehr in
/// dieser Struktur und keines in der Datei; die Vorlage
/// `ablage::lesezeichen::Ziel` bleibt bei ihrer Form, weil ihre zwei Varianten
/// sich an je einem Pflichtfeld unterscheiden und keinen Tisch tragen.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Zeilendatei {
    /// Die Beschriftung, wie sie in der Zusammenfassung links steht.
    pub beschriftung: String,
    /// B1: `zaehlung = { … }`.
    pub zaehlung: Option<Zaehlungsdatei>,
    /// B2: `juengste = { … }`.
    pub juengste: Option<Juengstedatei>,
    /// B3: `feld = { … }`.
    pub feld: Option<Felddatei>,
    /// B4: `vorhandensein = { … }`.
    pub vorhandensein: Option<Vorhandenseindatei>,
}

impl Zeilendatei {
    /// Zerlegt die Zeile in ihre Beschriftung und ihren einen Bausteintisch.
    ///
    /// Die Beschriftung kommt in beiden Ausgaengen heraus, denn eine
    /// abgewiesene Zeile behaelt sie (C3.12), und die Meldung nennt sie.
    ///
    /// Keiner und zwei sind beide ein Grund und keine stille Wahl: C3 sagt
    /// „genau ein Baustein", und ohne diese Stelle traefe die Entscheidung die
    /// Reihenfolge der Felder.
    fn zerlegen(self) -> (String, Result<Bausteindatei, String>) {
        let Zeilendatei {
            beschriftung,
            zaehlung,
            juengste,
            feld,
            vorhandensein,
        } = self;
        let mut genannt: Vec<(&'static str, Bausteindatei)> = [
            zaehlung.map(|tisch| ("zaehlung", Bausteindatei::Zaehlung(tisch))),
            juengste.map(|tisch| ("juengste", Bausteindatei::Juengste(tisch))),
            feld.map(|tisch| ("feld", Bausteindatei::Feld(tisch))),
            vorhandensein.map(|tisch| ("vorhandensein", Bausteindatei::Vorhandensein(tisch))),
        ]
        .into_iter()
        .flatten()
        .collect();

        if genannt.len() > 1 {
            let namen: Vec<&str> = genannt.iter().map(|(name, _)| *name).collect();
            return (
                beschriftung,
                Err(format!(
                    "sie nennt {} Bausteine ({}) und nicht genau einen",
                    genannt.len(),
                    namen.join(", ")
                )),
            );
        }
        match genannt.pop() {
            Some((_, tisch)) => (beschriftung, Ok(tisch)),
            None => (
                beschriftung,
                Err(format!(
                    "sie nennt keinen der vier Bausteine ({BAUSTEINNAMEN})"
                )),
            ),
        }
    }
}

/// Die Namen der vier Bausteintische, wie sie in der Datei stehen.
///
/// Eine Meldung ueber eine Zeile ohne Baustein zaehlt sie auf: der Nutzer hat
/// an dieser Stelle offenbar keinen von ihnen vor Augen.
const BAUSTEINNAMEN: &str = "zaehlung, juengste, feld, vorhandensein";

/// Der eine Bausteintisch einer Zeile, ausgewaehlt aus den vieren.
///
/// **Kein `Deserialize`.** Der Wert entsteht in [`Zeilendatei::zerlegen`] und
/// nicht beim Lesen; genau darin liegt der Unterschied zur unmarkierten
/// Auswahl, die bis zum 260824 hier stand.
#[derive(Debug)]
pub enum Bausteindatei {
    /// B1: `zaehlung = { … }`.
    Zaehlung(Zaehlungsdatei),
    /// B2: `juengste = { … }`.
    Juengste(Juengstedatei),
    /// B3: `feld = { … }`.
    Feld(Felddatei),
    /// B4: `vorhandensein = { … }`.
    Vorhandensein(Vorhandenseindatei),
}

/// Der Tisch `zaehlung`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Zaehlungsdatei {
    /// Der Unterordner, oder der erkannte Ordner selbst.
    pub ordner: Option<String>,
    /// Das Muster auf dem Eintragsnamen, oder alle Eintraege.
    pub muster: Option<String>,
}

/// Der Tisch `juengste`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Juengstedatei {
    /// Der Unterordner, oder der erkannte Ordner selbst.
    pub ordner: Option<String>,
    /// Das Muster auf dem Eintragsnamen, oder alle Eintraege.
    pub muster: Option<String>,
    /// Wie viele. Ueber [`HOECHSTENS_JUENGSTE`] wird gekappt.
    ///
    /// `u64` und nicht `u8`, damit eine ueberhoehte Zahl in der Datei die
    /// Kappung erreicht, statt schon am Zahlenbereich zu scheitern und die
    /// ganze Datei als beschaedigt dastehen zu lassen.
    pub anzahl: u64,
}

/// Der Tisch `feld`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Felddatei {
    /// Der Unterordner, oder der erkannte Ordner selbst.
    pub ordner: Option<String>,
    /// Das Muster auf dem Dateinamen.
    pub datei: String,
    /// Das Muster auf dem Inhalt, mit genau einer Fanggruppe.
    pub feldmuster: String,
}

/// Der Tisch `vorhandensein`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Vorhandenseindatei {
    /// Der Unterordner, oder der erkannte Ordner selbst.
    pub ordner: Option<String>,
    /// Das Muster auf dem Eintragsnamen.
    pub muster: String,
}

// ---------------------------------------------------------------------------
// Der Pruefschritt
// ---------------------------------------------------------------------------

/// Macht aus der gelesenen Datei die geprueften Profile und die Meldungen.
///
/// Was abgewiesen wird und wie weit, steht im Modulkopf. Jede Meldung nennt
/// den Profilnamen, bei einer Zeile deren Beschriftung, und den Grund; sie
/// geht denselben Weg wie die Meldungen der uebrigen Ablagedateien und
/// erscheint einmal beim Start in der Statuszeile.
#[must_use = "die zweite Haelfte des Paares sind die Meldungen; wer sie fallen \
              laesst, verschweigt dem Nutzer, welches Profil und welche Zeile \
              seiner readers.toml abgewiesen wurden"]
pub fn pruefen(datei: Profildatei) -> (Profile, Vec<String>) {
    let mut geprueft = Vec::new();
    let mut meldungen = Vec::new();

    for block in datei.profil {
        let Profilblock {
            name,
            pfad,
            kennzeichen,
            zeile,
        } = block;

        let pfad = match erkennungsmuster(pfad.as_deref(), "das Pfadmuster") {
            Ok(muster) => muster,
            Err(grund) => {
                meldungen.push(profilmeldung(&name, &grund));
                continue;
            }
        };
        let kennzeichen = match erkennungsmuster(kennzeichen.as_deref(), "die Kennzeichendatei") {
            Ok(muster) => muster,
            Err(grund) => {
                meldungen.push(profilmeldung(&name, &grund));
                continue;
            }
        };
        if pfad.is_none() && kennzeichen.is_none() {
            meldungen.push(profilmeldung(
                &name,
                "es nennt weder ein Pfadmuster noch eine Kennzeichendatei und koennte damit nie \
                 treffen",
            ));
            continue;
        }

        let zeilen = zeile
            .into_iter()
            .map(|zeile| zeile_pruefen(&name, zeile, &mut meldungen))
            .collect();
        geprueft.push(Profil::neu(name, pfad, kennzeichen, zeilen));
    }

    (Profile::aus(geprueft), meldungen)
}

/// Prueft eine einzelne Zeile. Eine abgewiesene behaelt ihre Beschriftung.
fn zeile_pruefen(profil: &str, zeile: Zeilendatei, meldungen: &mut Vec<String>) -> Zeile {
    let (beschriftung, tisch) = zeile.zerlegen();
    match tisch.and_then(baustein_pruefen) {
        Ok(baustein) => Zeile::neu(beschriftung, Some(baustein)),
        Err(grund) => {
            meldungen.push(zeilenmeldung(profil, &beschriftung, &grund));
            Zeile::neu(beschriftung, None)
        }
    }
}

/// Prueft einen einzelnen Baustein.
///
/// Die Fallunterscheidung ueber die vier Bausteine ist vollstaendig und hat
/// keinen Auffangzweig.
fn baustein_pruefen(baustein: Bausteindatei) -> Result<Baustein, String> {
    match baustein {
        Bausteindatei::Zaehlung(zaehlung) => Ok(Baustein::Zaehlung {
            ort: ortsangabe(zaehlung.ordner.as_deref())?,
            muster: wahlfreies_muster(zaehlung.muster.as_deref())?,
        }),
        Bausteindatei::Juengste(juengste) => Ok(Baustein::Juengste {
            ort: ortsangabe(juengste.ordner.as_deref())?,
            muster: wahlfreies_muster(juengste.muster.as_deref())?,
            anzahl: gekappte_anzahl(juengste.anzahl),
        }),
        Bausteindatei::Feld(feld) => Ok(Baustein::Feld {
            ort: ortsangabe(feld.ordner.as_deref())?,
            datei: muster(&feld.datei)?,
            feldmuster: feldmuster(&feld.feldmuster)?,
        }),
        Bausteindatei::Vorhandensein(vorhandensein) => Ok(Baustein::Vorhandensein {
            ort: ortsangabe(vorhandensein.ordner.as_deref())?,
            muster: muster(&vorhandensein.muster)?,
        }),
    }
}

/// Uebersetzt ein Muster und liefert im Fehlerfall die einzeilige Meldung des
/// Uebersetzers.
///
/// **Die eine Stelle, an der ein Muster zu einem [`Regex`] wird.** Jeder
/// Aufrufer darunter setzt seinen Satz davor und uebersetzt nicht selbst.
fn uebersetzen(text: &str) -> Result<Regex, String> {
    Regex::new(text).map_err(|fehler| einzeilig(&fehler))
}

/// Uebersetzt eines der beiden Erkennungsmuster eines Profils (C2.7).
///
/// `was` benennt, welches von beiden gemeint ist; die zwei Aufrufer sind die
/// zwei Muster, und ein dritter Satz entsteht nicht.
fn erkennungsmuster(text: Option<&str>, was: &str) -> Result<Option<Regex>, String> {
    match text {
        None => Ok(None),
        Some(text) => uebersetzen(text)
            .map(Some)
            .map_err(|grund| format!("{was} {text:?} laesst sich nicht uebersetzen: {grund}")),
    }
}

/// Uebersetzt ein Muster, das in einem Baustein dastehen muss.
fn muster(text: &str) -> Result<Regex, String> {
    uebersetzen(text)
        .map_err(|grund| format!("das Muster {text:?} laesst sich nicht uebersetzen: {grund}"))
}

/// Uebersetzt ein Muster, das in einem Baustein fehlen darf.
fn wahlfreies_muster(text: Option<&str>) -> Result<Option<Regex>, String> {
    match text {
        None => Ok(None),
        Some(text) => muster(text).map(Some),
    }
}

/// Uebersetzt ein Feldmuster und haelt es gegen C3.10: genau eine Fanggruppe.
///
/// [`Regex::captures_len`] zaehlt die Gruppe 0 mit, also den ganzen Treffer;
/// gefordert ist deshalb der Wert 2. Nicht fangende Gruppen `(?:…)` zaehlen
/// nicht mit, und genau darum sind sie der Ausweg fuer ein Muster, das eine
/// Alternative gruppieren will, ohne sie zu fangen.
fn feldmuster(text: &str) -> Result<Regex, String> {
    let ausdruck = muster(text)?;
    let gruppen = ausdruck.captures_len() - 1;
    if gruppen == 1 {
        return Ok(ausdruck);
    }
    Err(format!(
        "das Feldmuster {text:?} traegt {gruppen} Fanggruppen und nicht genau eine"
    ))
}

/// Prueft eine Ortsangabe, die fehlen darf. Fehlt sie, ist der erkannte
/// Ordner selbst gemeint.
fn ortsangabe(angabe: Option<&str>) -> Result<Ortsangabe, String> {
    match angabe {
        None => Ok(Ortsangabe::wurzel()),
        Some(text) => Ortsangabe::aus_angabe(text)
            .map_err(|mangel| format!("die Ortsangabe {text:?} {}", mangel.grund())),
    }
}

/// Kappt eine ueberhoehte Zahl auf [`HOECHSTENS_JUENGSTE`] (C6.3).
///
/// Ohne Meldung: die Zahl ist nicht falsch, sie verlangt nur mehr, als die
/// Zusammenfassung hergibt.
fn gekappte_anzahl(anzahl: u64) -> u8 {
    u8::try_from(anzahl)
        .unwrap_or(HOECHSTENS_JUENGSTE)
        .min(HOECHSTENS_JUENGSTE)
}

/// Die Meldung des Uebersetzers, auf eine Zeile gebracht.
///
/// `regex` beschreibt einen Fehler ueber mehrere Zeilen samt einer Zeile aus
/// Leerzeichen und einem Dach darunter. Die Statuszeile ist eine Zeile; die
/// Umbrueche fielen dort ohnehin weg und liessen den Text mit
/// zusammengelaufenen Woertern zurueck.
fn einzeilig(fehler: &regex::Error) -> String {
    fehler
        .to_string()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Eine Meldung ueber ein ganzes Profil.
fn profilmeldung(profil: &str, grund: &str) -> String {
    format!("Profil \u{201e}{profil}\u{201c}: {grund}")
}

/// Eine Meldung ueber eine einzelne Zeile eines Profils.
fn zeilenmeldung(profil: &str, beschriftung: &str, grund: &str) -> String {
    format!("Profil \u{201e}{profil}\u{201c}, Zeile \u{201e}{beschriftung}\u{201c}: {grund}")
}
