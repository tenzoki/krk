//! Den Heimordner und seine Eintragsdateien anlegen.
//!
//! [`bereitstellen`] ist der Weg, den F2 vor dem Oeffnen des Tabs geht (C2 und
//! C3 des Spec `260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`).
//! **Beim Start laeuft er nicht**, auch nicht fuer einen wiederhergestellten Tab
//! auf den Ordner; das haelt eine Probe ueber seine Rufer in `krk-ui`.
//!
//! # Wer anlegt, entscheidet der Systemaufruf, der anlegt
//!
//! Ob **dieser** Aufruf den Ordner angelegt hat, laesst sich aus einer
//! vorherigen Existenzpruefung nicht beantworten: eine zweite KRK-Instanz kann
//! dazwischen anlegen. Die Antwort gibt deshalb `mkdir(2)` selbst, das genau
//! einem Aufrufer gelingt, und sie steht in [`Bereitstellung::ordner_angelegt`].
//!
//! # Die Ablage beruehrt dieser Weg nicht mehr
//!
//! Bis zum 261111 uebernahm ein Aufruf, der den Vorgabeort `~/krkhome` selbst
//! anlegte, die Zettel `note-1.txt` und `note-2.txt` aus dem Ablageordner als
//! Notizen nach `notes.txt`, einmal, gehalten vom Merker
//! [`zettel_uebernommen`](crate::ablage::merker::Merker::zettel_uebernommen) in
//! `reported.toml`; dafuer nahm er den Ablageordner und einen Zugang zur Ablage
//! und lief am Vorgabeort unter deren Schreibsperre. **Mit `notes.txt` ist die
//! Uebernahme gefallen** (C3.7 des Spec
//! `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`, E7 des
//! Plans `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`):
//! dieser Weg liest die zwei Zettel nicht, nimmt weder Ablageordner noch
//! Zugang, liest und schreibt `reported.toml` nicht, und die Zettel bleiben
//! unveraendert im Ablageordner liegen. Das Feld des Merkers bleibt in der
//! Datei stehen; warum, sagt sein Doc-Kommentar.
//!
//! # Eine vorhandene Datei wird nie ueberschrieben
//!
//! Jede Eintragsdatei entsteht mit exklusivem Oeffnen
//! (`create_new`, also `O_CREAT | O_EXCL`): steht die Datei schon da, scheitert
//! das Oeffnen, und die Datei bleibt Byte fuer Byte, wie sie war. **Eine
//! vorhandene Datei wird nie ueberschrieben**, auch nicht, wenn sie zwischen
//! einer Pruefung und dem Anlegen entstanden ist; eine solche Pruefung gibt es
//! hier gar nicht (`260926-0007_*_wann-entsteht-secrets-txt-und-was-geschieht-mit-fehlenden-dateien.md`).
//!
//! # `secrets.txt` entsteht leer und ohne PIN
//!
//! Angelegt wird genau, was [`Sonderdatei::ALLE`] fuehrt, und seit Schritt 5.2
//! gehoert `secrets.txt` dazu, ohne dass dieser Weg eine Zeile dafuer traegt:
//! sie entsteht ueber dasselbe exklusive Oeffnen **mit null Bytes**, und eine
//! PIN fragt F2 dabei nie. Eine leere Datei traegt noch keinen Kopf; die PIN
//! legt das erste Oeffnen im Editor fest
//! (`260926-0007_*_wann-entsteht-secrets-txt-und-was-geschieht-mit-fehlenden-dateien.md`).
//! Bis Stufe 1 entstand sie hier nicht, weil eine leere Datei ohne die Regeln
//! aus C7 sich im Editor oeffnen und mit Klartext sichern liesse; diese Regeln
//! bringt Stufe 5 mit.
//!
//! **`notes.txt` und `appointments.md` legt dieser Weg seit dem 261111 nicht
//! mehr an**: sie stehen nicht mehr in [`Sonderdatei::ALLE`], und weil allein
//! die Namen jener Liste hier einen Pfad bekommen, fasst die Bereitstellung
//! eine vorhandene `notes.txt` oder `appointments.md` auch nicht an.
//!
//! # `minutes/` entsteht nach den Dateien und haelt den Tab nicht auf
//!
//! Nach den Eintragsdateien legt derselbe Aufruf den Unterordner
//! [`PROTOKOLLORDNER`] an, wenn er fehlt, auch in einem Notizordner, den es
//! schon gibt (C4.1 des Spec
//! `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`). Die
//! Frage, ob er fehlt, beantwortet wie beim Notizordner selbst `mkdir(2)` und
//! keine Pruefung davor; ein vorhandener Ordner wird nicht geoeffnet und nicht
//! gelesen, sein Inhalt bleibt also, wie er ist. **Der Ausgang ist ein Feld
//! und kein Hindernis der Funktion** ([`Bereitstellung::protokollordner`]):
//! steht an der Stelle eine gewoehnliche Datei namens `minutes`, bleibt sie
//! unveraendert, F2 oeffnet den Notizordner trotzdem, und die Statuszeile
//! nennt den Grund (E8 des Plans
//! `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`).
//!
//! # Eine `.secrets.txt` von vorher wird zu `secrets.txt`
//!
//! Bis zum 260926 hiess die Datei `.secrets.txt`, mit Punkt
//! (`260926-1308_*_heisst-die-geheimnisdatei-secrets-txt-ohne-punkt.md`).
//! Steht eine solche im Heimordner und `secrets.txt` noch nicht, benennt F2 sie
//! um, bevor es `secrets.txt` anlegen wuerde. **Umbenannt wird ueber einen
//! harten Verweis und nicht ueber `rename(2)`**: `link(2)` scheitert, wenn der
//! neue Name schon steht, und ersetzt nie; erst danach faellt der alte Name.
//! `rename(2)` ersetzte ein vorhandenes `secrets.txt` still, und eine
//! Existenzpruefung davor liesse den Wettlauf offen, den der Abschnitt
//! darueber schliesst. Der Inhalt wird dabei nicht gelesen und nicht
//! geschrieben; es bleibt dieselbe Inode, Byte fuer Byte.
//!
//! **Stehen beide, bleiben beide**, und die Statuszeile sagt es. Von da an gilt
//! `secrets.txt`; `.secrets.txt` ist fuer KRK eine gewoehnliche versteckte
//! Datei. Scheitert der Verweis aus einem anderen Grund, etwa auf einem
//! Dateisystem ohne harte Verweise, bleibt `.secrets.txt` unberuehrt, es
//! entsteht **keine** leere `secrets.txt` daneben, und die Statuszeile nennt
//! den Grund; das naechste F2 versucht es wieder.

use std::fmt;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::Path;

use super::{Heimordner, PROTOKOLLORDNER, Sonderdatei};
use crate::sprache::{Text, satz};

/// Der Name der Geheimnisdatei bis zum 260926, allein fuer das Umbenennen.
///
/// Der gueltige Name steht an [`Sonderdatei::dateiname`] und nirgends sonst;
/// dieser hier wird nie angelegt und nie gelesen, nur umbenannt.
pub const ALTER_GEHEIMNISNAME: &str = ".secrets.txt";

/// Was aus einer Datei unter ihrem frueheren Namen geworden ist, der Ausgang
/// von [`ohne_ersetzen_umbenennen`].
///
/// Der Typ hat zwei Nutzer: `.secrets.txt` wird bei F2 zu `secrets.txt`, und
/// die Sicherung der Quicknote macht aus der frueheren Quicknote-Datei
/// ([`super::ALTE_QUICKNOTEDATEI`]) `quicknote.txt`. Bis zum 261111 hiess er
/// `AlteGeheimnisse` und meinte allein den ersten.
///
/// **Vollstaendig und ohne Auffangzweig**, damit ein weiterer Ausgang die
/// Meldungen anhaelt, bis er einen Satz hat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Umbenennung {
    /// Die Datei traegt jetzt den neuen Namen, dieselbe Inode, der alte Name
    /// ist weg.
    Umbenannt,
    /// Der neue Name stand schon; beide Dateien bleiben, wie sie sind.
    BeideStehen,
    /// Der harte Verweis ist gelungen, der alte Name laesst sich aber nicht
    /// entfernen: beide Namen nennen dieselbe Datei. Der Grund des Systems.
    AlterNameBleibt(String),
    /// Der harte Verweis ist gescheitert; die Datei unter dem alten Namen ist
    /// unberuehrt, und der neue Name ist nicht angelegt. Der Grund des
    /// Systems.
    Gescheitert(String),
}

impl Umbenennung {
    /// Der Satz fuer die Statuszeile; `alt` und `neu` sind die zwei
    /// Dateinamen, `ort` der Ordner, wie [`Heimordner::anzeigename`] ihn nennt.
    ///
    /// Der Wortlaut steht in der Sprachtabelle (`crate::sprache`) und nennt
    /// keine Datei selbst, damit beide Nutzer des Typs dieselben vier Saetze
    /// tragen.
    pub fn meldung(&self, alt: &str, neu: &str, ort: &str) -> String {
        let namen: [(&str, &dyn fmt::Display); 2] = [("alt", &alt), ("neu", &neu)];
        match self {
            Umbenennung::Umbenannt => satz(Text::HeimUmbenannt, &namen),
            Umbenennung::BeideStehen => satz(
                Text::HeimUmbenennungBeideStehen,
                &[("alt", &alt), ("neu", &neu), ("ort", &ort)],
            ),
            Umbenennung::AlterNameBleibt(grund) => satz(
                Text::HeimUmbenennungAlterNameBleibt,
                &[("alt", &alt), ("neu", &neu), ("grund", grund)],
            ),
            Umbenennung::Gescheitert(grund) => satz(
                Text::HeimNichtUmbenannt,
                &[("alt", &alt), ("neu", &neu), ("grund", grund)],
            ),
        }
    }
}

/// Was ein Aufruf von [`bereitstellen`] getan hat.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "traegt die Meldungen des Anlegens; fallengelassen verschwindet eine Datei, die sich nicht anlegen liess, ohne ein Wort"]
pub struct Bereitstellung {
    /// Ob dieser Aufruf den Ordner angelegt hat.
    pub ordner_angelegt: bool,
    /// Die Eintragsdateien, die dieser Aufruf angelegt hat.
    pub angelegt: Vec<Sonderdatei>,
    /// Die Eintragsdateien, die fehlten und sich nicht anlegen liessen, je mit
    /// dem Grund des Systems.
    pub nicht_angelegt: Vec<(Sonderdatei, String)>,
    /// Was aus einer `.secrets.txt` von vorher geworden ist; `None`, wenn im
    /// Heimordner keine stand. Bis zum 261111 hiess das Feld
    /// `alte_geheimnisse`.
    pub umbenennung: Option<Umbenennung>,
    /// Ob [`PROTOKOLLORDNER`] im Notizordner steht: `Ok`, wenn dieser Aufruf
    /// ihn angelegt oder als Ordner vorgefunden hat, sonst der Grund in den
    /// Werten von [`Hindernis`], gelesen am Unterordner statt am Notizordner.
    ///
    /// **Ein `Err` hier haelt F2 nicht auf**; es ist das Hindernis allein fuer
    /// den, der in den Unterordner will.
    pub protokollordner: Result<(), Hindernis>,
    /// Der Ort, wie die Statuszeile ihn nennt ([`Heimordner::anzeigename`]),
    /// fuer die Saetze, die den Ordner nennen.
    pub ort: String,
}

/// Warum F2 keinen Tab oeffnet.
///
/// **Vollstaendig und ohne Auffangzweig.** Ein Hindernis betrifft den Ordner;
/// eine einzelne Datei, die sich nicht anlegen laesst, haelt den Tab nicht auf
/// und steht in [`Bereitstellung::nicht_angelegt`]. **Dieselben vier Werte
/// beschreiben seit dem 261111 den Unterordner [`PROTOKOLLORDNER`]**, im Feld
/// [`Bereitstellung::protokollordner`]; dort halten sie F2 nicht auf, wohl
/// aber „Protokoll anlegen“, das ohne `minutes/` kein Ziel hat. Der Satz von
/// [`Hindernis::KeinOrdner`] sagt deshalb „öffnet dort keinen Tab“ und nicht
/// „öffnet keinen Tab“: bei F2 geht der Tab auf den Notizordner trotzdem auf,
/// und der Satz muss fuer beide Befehle wahr sein. Dass es gar keinen Ort
/// gibt, etwa ohne Benutzerverzeichnis, ist kein Hindernis, sondern ein
/// [`super::ort::Ortsfehler`]: dann gibt es keinen [`Heimordner`], den dieser
/// Weg bekommen koennte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hindernis {
    /// An der Stelle des Orts steht etwas, das kein Ordner ist; es bleibt
    /// unveraendert.
    KeinOrdner,
    /// An der Stelle steht etwas, dessen Ziel sich nicht erreichen laesst,
    /// etwa ein Verweis ins Leere. Der Grund des Systems in einem Satzteil.
    Unerreichbar(String),
    /// Der Ordner fehlt und laesst sich nicht anlegen. Der Grund des Systems.
    NichtAnlegbar(String),
    /// Der Ordner fehlt, und der Ordner darueber fehlt ebenso, etwa weil das
    /// Laufwerk nicht eingehaengt ist. Angelegt wird allein die letzte
    /// Pfadstufe, also nichts.
    ObererOrdnerFehlt,
}

impl Hindernis {
    /// Der Satz fuer die Statuszeile; `ordner` ist der Ort, wie ihn
    /// [`Heimordner::anzeigename`] nennt.
    ///
    /// Der Wortlaut steht in der Sprachtabelle (`crate::sprache`), hier je
    /// Wert sein Schluessel.
    pub fn meldung(&self, ordner: &str) -> String {
        match self {
            Hindernis::KeinOrdner => satz(Text::HeimKeinOrdner, &[("ordner", &ordner)]),
            Hindernis::Unerreichbar(grund) => satz(
                Text::HeimNichtErreichbar,
                &[("ordner", &ordner), ("grund", grund)],
            ),
            Hindernis::NichtAnlegbar(grund) => satz(
                Text::HeimNichtAnlegbar,
                &[("name", &ordner), ("grund", grund)],
            ),
            Hindernis::ObererOrdnerFehlt => {
                satz(Text::HeimObererOrdnerFehlt, &[("ordner", &ordner)])
            }
        }
    }
}

impl Bereitstellung {
    /// Die Saetze fuer die Statuszeile, in der Reihenfolge, in der sie
    /// geschehen sind. Leer, wenn es nichts zu sagen gibt: das stille Anlegen
    /// einer fehlenden Datei ist keine Meldung wert.
    pub fn meldungen(&self) -> Vec<String> {
        let mut saetze = Vec::new();
        for (sorte, grund) in &self.nicht_angelegt {
            saetze.push(satz(
                Text::HeimNichtAnlegbar,
                &[("name", &sorte.dateiname()), ("grund", grund)],
            ));
        }
        if let Some(umbenennung) = &self.umbenennung {
            saetze.push(umbenennung.meldung(
                ALTER_GEHEIMNISNAME,
                Sonderdatei::Geheimnisse.dateiname(),
                &self.ort,
            ));
        }
        if let Err(hindernis) = &self.protokollordner {
            // Der Unterordner so, wie die Statuszeile den Notizordner nennt,
            // mit dem Namen dahinter; `join` setzt den Trennstrich auch dann
            // richtig, wenn der Ort die Wurzel ist.
            let unterordner = Path::new(&self.ort).join(PROTOKOLLORDNER);
            saetze.push(hindernis.meldung(&unterordner.display().to_string()));
        }
        saetze
    }
}

/// Legt den Heimordner an, falls er fehlt, darin jede fehlende
/// Eintragsdatei und danach den Unterordner [`PROTOKOLLORDNER`].
///
/// Angelegt wird an der geschriebenen Form des Heimordners; ist sie ein Verweis
/// auf einen Ordner, entstehen die Dateien in dessen Ziel. **Allein fuer F2
/// gedacht**; der Modulkopf sagt, warum der Start diesen Weg nicht erreicht.
/// Die Ablage beruehrt der Aufruf nicht: er nimmt weder Ablageordner noch
/// Zugang (Modulkopf, „Die Ablage beruehrt dieser Weg nicht mehr“).
#[must_use = "ein Hindernis heisst: kein Tab, und die Meldungen gehoeren in die Statuszeile"]
pub fn bereitstellen(heim: &Heimordner) -> Result<Bereitstellung, Hindernis> {
    anlegen_mit_vorlauf(heim, &mut |_| {})
}

/// [`bereitstellen`] mit einem Einhaengepunkt, der unmittelbar vor jedem
/// exklusiven Oeffnen mit dem Pfad der Datei gerufen wird.
///
/// Der Vorlauf ist allein fuer die Rennprobe im Pruefmodul da: sie legt die
/// Datei darin selbst an, also nach jeder denkbaren Pruefung und vor dem
/// Anlegen, und findet sie danach unveraendert.
fn anlegen_mit_vorlauf(
    heim: &Heimordner,
    vorlauf: &mut dyn FnMut(&Path),
) -> Result<Bereitstellung, Hindernis> {
    let ordner = heim.geschrieben.as_path();
    let ordner_angelegt = ordner_anlegen(ordner)?;

    let mut bereitstellung = Bereitstellung {
        ordner_angelegt,
        angelegt: Vec::new(),
        nicht_angelegt: Vec::new(),
        umbenennung: None,
        // Gesetzt nach den Dateien, unten.
        protokollordner: Ok(()),
        ort: heim.anzeigename().to_owned(),
    };
    for sorte in Sonderdatei::ALLE {
        let pfad = ordner.join(sorte.dateiname());
        if sorte == Sonderdatei::Geheimnisse {
            bereitstellung.umbenennung =
                ohne_ersetzen_umbenennen(&ordner.join(ALTER_GEHEIMNISNAME), &pfad);
            match &bereitstellung.umbenennung {
                // Es gab keine alte Datei, oder `secrets.txt` stand schon: dann
                // legt der gewoehnliche Weg an oder findet sie vor.
                None | Some(Umbenennung::BeideStehen) => {}
                // Sie heisst jetzt `secrets.txt`, oder eine Datei unter altem
                // Namen soll nicht eine leere neue neben sich bekommen.
                Some(
                    Umbenennung::Umbenannt
                    | Umbenennung::AlterNameBleibt(_)
                    | Umbenennung::Gescheitert(_),
                ) => continue,
            }
        }
        // Jede Eintragsdatei entsteht leer: `secrets.txt` mit null Bytes und
        // ohne Kopf, die PIN legt erst das erste Oeffnen im Editor fest.
        vorlauf(&pfad);
        let ausgang = exklusiv_anlegen(&pfad);
        match ausgang {
            Anlegeausgang::Angelegt => bereitstellung.angelegt.push(sorte),
            Anlegeausgang::StandSchon => {}
            Anlegeausgang::Gescheitert(grund) => {
                bereitstellung.nicht_angelegt.push((sorte, grund));
            }
        }
    }
    // Nach den Dateien, und ohne `?`: ein Hindernis am Unterordner ist ein
    // Feld und haelt den Tab auf den Notizordner nicht auf.
    bereitstellung.protokollordner = ordner_anlegen(&heim.protokollordner()).map(|_| ());
    Ok(bereitstellung)
}

/// Legt die letzte Pfadstufe als Ordner an, falls sie fehlt; `true`, wenn
/// dieser Aufruf sie angelegt hat, `false`, wenn dort schon ein Ordner stand.
///
/// Die eine Regel fuer den Notizordner und fuer [`PROTOKOLLORDNER`] darin. Ob
/// der Ordner fehlt, beantwortet `mkdir(2)` selbst (Modulkopf, „Wer anlegt,
/// entscheidet der Systemaufruf, der anlegt“); ein vorhandener Ordner wird
/// nicht geoeffnet, und was sonst an der Stelle steht, bleibt unveraendert.
fn ordner_anlegen(ordner: &Path) -> Result<bool, Hindernis> {
    match fs::create_dir(ordner) {
        Ok(()) => Ok(true),
        Err(fehler) if fehler.kind() == io::ErrorKind::AlreadyExists => {
            // Es steht etwas da. `metadata` folgt einem Verweis, und genau das
            // ist die Frage: ist dort, wohin der Name fuehrt, ein Ordner?
            match fs::metadata(ordner) {
                Ok(angaben) if angaben.is_dir() => Ok(false),
                Ok(_) => Err(Hindernis::KeinOrdner),
                Err(fehler) => Err(Hindernis::Unerreichbar(fehler.to_string())),
            }
        }
        // Angelegt wird allein die letzte Pfadstufe; fehlt die darueber, ist
        // das ein eigenes Hindernis und kein `create_dir_all`.
        Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => {
            Err(Hindernis::ObererOrdnerFehlt)
        }
        Err(fehler) => Err(Hindernis::NichtAnlegbar(fehler.to_string())),
    }
}

/// Gibt der Datei `alt` den Namen `neu`, **ohne je eine vorhandene Datei zu
/// ersetzen**: erst ein harter Verweis, der an einem vorhandenen `neu`
/// scheitert, dann faellt `alt`. `None`, wenn es `alt` nicht gibt.
///
/// Meldet `link(2)` ein vorhandenes Ziel, wird `alt` noch einmal am Eintrag
/// gefragt: fehlen beide Seiten nicht, stehen beide; fehlt `alt`, gab es nichts
/// umzubenennen, gleich welchen Fehler das System zuerst nennt.
///
/// **Vor dem Verweis steht keine Pruefung, ob `neu` frei ist, und `rename(2)`
/// steht hier nicht**: die eine Frage, ob der neue Name frei ist, beantwortet
/// `link(2)` selbst, also gibt es kein Fenster zwischen Pruefung und
/// Umbenennung, in dem eine entstehende Datei ersetzt wuerde. Die Probe
/// `ohne_ersetzen_umbenennen_prueft_nicht_vor_dem_verweis_und_benennt_nicht_um`
/// (`crates/krk-core/tests/heimordner.rs`) haelt beides am Rumpf.
///
/// Oeffentlich seit dem 261111, weil neben F2 (`.secrets.txt`) die Sicherung
/// der Quicknote dieselbe Regel braucht (E3 des Plans
/// `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`). Die
/// Funktion stellt Systemaufrufe und gehoert auf keinen Hauptfadenweg.
pub fn ohne_ersetzen_umbenennen(alt: &Path, neu: &Path) -> Option<Umbenennung> {
    match fs::hard_link(alt, neu) {
        Ok(()) => Some(match fs::remove_file(alt) {
            Ok(()) => Umbenennung::Umbenannt,
            Err(fehler) => Umbenennung::AlterNameBleibt(fehler.to_string()),
        }),
        Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => None,
        Err(fehler) if fehler.kind() == io::ErrorKind::AlreadyExists => {
            match fs::symlink_metadata(alt) {
                Ok(_) => Some(Umbenennung::BeideStehen),
                Err(pruefung) if pruefung.kind() == io::ErrorKind::NotFound => None,
                Err(pruefung) => Some(Umbenennung::Gescheitert(pruefung.to_string())),
            }
        }
        Err(fehler) => Some(Umbenennung::Gescheitert(fehler.to_string())),
    }
}

/// Wie das exklusive Anlegen einer Datei ausgegangen ist.
enum Anlegeausgang {
    Angelegt,
    /// Die Datei stand schon und ist nicht angefasst.
    StandSchon,
    /// Der Grund des Systems.
    Gescheitert(String),
}

/// Legt die Datei exklusiv und leer an.
///
/// `create_new` ist `O_CREAT | O_EXCL`: steht die Datei schon da, scheitert
/// das Oeffnen, und sie bleibt Byte fuer Byte, wie sie war. Geschrieben wird
/// nichts hinein; bis zum 261111 trug `notes.txt` hier die uebernommenen
/// Zettel, und mit ihr ist der Schreibgang samt dem Wegraeumen einer halb
/// geschriebenen Datei gefallen.
fn exklusiv_anlegen(pfad: &Path) -> Anlegeausgang {
    match OpenOptions::new().write(true).create_new(true).open(pfad) {
        Ok(_) => Anlegeausgang::Angelegt,
        Err(fehler) if fehler.kind() == io::ErrorKind::AlreadyExists => Anlegeausgang::StandSchon,
        Err(fehler) => Anlegeausgang::Gescheitert(fehler.to_string()),
    }
}

#[cfg(test)]
mod proben {
    use std::path::PathBuf;

    use super::*;
    use crate::heimordner::ORDNERNAME;

    /// Ein frischer Ordner unter dem Temporaerverzeichnis, der die Rolle des
    /// Benutzerverzeichnisses spielt, mit Prozesskennung und Probennamen.
    ///
    /// Bewusst keine weitere Pruefordner-Fassung mit `Drop`: die eine Fassung
    /// dieser Kiste liegt in `tests/gemeinsam/mod.rs`, und ein Pruefmodul der
    /// Bibliothek erreicht sie nicht. Abgeraeumt wird von Hand am Ende jeder
    /// Probe; ein Rest einer abgebrochenen Probe faellt beim naechsten Anlegen.
    /// Dieselbe Bauart wie im Pruefmodul von `operation/verschieben.rs`.
    fn pruefpfad(probe: &str) -> PathBuf {
        let pfad =
            std::env::temp_dir().join(format!("krk-heim-bereit-{probe}-{}", std::process::id()));
        abraeumen(&pfad);
        fs::create_dir_all(&pfad).expect("der Pruefordner laesst sich nicht anlegen");
        pfad
    }

    fn abraeumen(pfad: &Path) {
        let _ = fs::remove_dir_all(pfad);
    }

    /// C2.1: Die Datei entsteht im Vorlauf, also nach jeder Pruefung und
    /// unmittelbar vor dem exklusiven Oeffnen, und bleibt Byte fuer Byte.
    #[test]
    fn eine_im_vorlauf_angelegte_datei_bleibt_unveraendert() {
        let zuhause = pruefpfad("vorlauf");
        let heim = Heimordner::im_benutzerverzeichnis(&zuhause);
        fs::create_dir(zuhause.join(ORDNERNAME)).expect("Heimordner");
        let fremd = b"von der anderen Instanz\n";

        let bereitstellung = anlegen_mit_vorlauf(&heim, &mut |pfad| {
            fs::write(pfad, fremd).expect("die fremde Datei laesst sich nicht schreiben");
        })
        .expect("ein Hindernis, wo keines ist");

        for sorte in Sonderdatei::ALLE {
            let pfad = zuhause.join(ORDNERNAME).join(sorte.dateiname());
            assert_eq!(
                fs::read(&pfad).expect("die Datei fehlt"),
                fremd,
                "{} ist ueberschrieben",
                sorte.dateiname()
            );
        }
        assert!(bereitstellung.angelegt.is_empty(), "{bereitstellung:?}");
        assert!(
            bereitstellung.nicht_angelegt.is_empty(),
            "{bereitstellung:?}"
        );
        abraeumen(&zuhause);
    }
}
