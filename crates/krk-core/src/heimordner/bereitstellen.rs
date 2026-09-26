//! Den Heimordner und seine Eintragsdateien anlegen, und beim ersten Mal die
//! alten Zettel uebernehmen.
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
//! einem Aufrufer gelingt. **Gelingt es, und nur dann, werden die alten Zettel
//! uebernommen** (`260926-0007_*_was-geschieht-mit-den-zwei-zetteln-des-bisherigen-notizblatts.md`,
//! Moeglichkeit 1). Ein spaeteres Neuanlegen von `notes.txt` uebernimmt nichts,
//! und ein Ordner, den es schon gab, bekommt nichts uebernommen.
//!
//! Aus demselben Grund entsteht jede Eintragsdatei mit exklusivem Oeffnen
//! (`create_new`, also `O_CREAT | O_EXCL`): steht die Datei schon da, scheitert
//! das Oeffnen, und die Datei bleibt Byte fuer Byte, wie sie war. **Eine
//! vorhandene Datei wird nie ueberschrieben**, auch nicht, wenn sie zwischen
//! einer Pruefung und dem Anlegen entstanden ist; eine solche Pruefung gibt es
//! hier gar nicht (`260926-0007_*_wann-entsteht-secrets-txt-und-was-geschieht-mit-fehlenden-dateien.md`).
//!
//! **Der eine Wettlauf, der bleibt:** legt eine zweite Instanz zwischen dem
//! Anlegen des Ordners und dem von `notes.txt` eine `notes.txt` an, scheitert
//! die Uebernahme an ihr. Die Meldung sagt dann, dass die Zettel nicht
//! uebernommen wurden und unveraendert im Ablageordner liegen. Still verloren
//! geht nichts, und die zweite Instanz hat nichts uebernommen, weil ihr
//! `mkdir(2)` gescheitert ist.
//!
//! # Was hier nicht entsteht
//!
//! `.secrets.txt` entsteht nicht in dieser Stufe, sondern mit C7 in Stufe 5:
//! eine leere Datei ohne die Regeln aus C7 liesse sich im Editor oeffnen und mit
//! Klartext sichern. Angelegt wird genau, was [`Sonderdatei::ALLE`] fuehrt.
//!
//! # Die alten Zettel
//!
//! Gelesen werden `note-1.txt` und `note-2.txt` aus dem Ablageordner, ueber den
//! einen Leseweg fuer Textdateien ([`text::datei::lesen`]). Die zwei Namen
//! stehen hier als Konstanten und nicht in der Aufzaehlung der Ablagedateien,
//! weil KRK die Dateien nach der Uebernahme weder liest noch schreibt. **Dieser
//! Weg schreibt nichts in den Ablageordner und legt dort nichts beiseite**; die
//! alten Dateien bleiben als Rueckfall liegen.
//!
//! Ein fehlender Zettel und einer aus nichts als Leerraum ergeben keine Notiz.
//! Ein lesbarer ergibt `## Zettel 1` beziehungsweise `## Zettel 2` mit seinem
//! Text darunter, und ein fehlender Schlussumbruch wird ergaenzt, damit die
//! naechste Themenzeile am Zeilenanfang steht. **Ein Zettel mit einer
//! Themenzeile wird nicht uebernommen**: sie eroeffnete eine eigene Notiz, und
//! sie still einzuruecken aenderte Text des Nutzers. Ebenso wenig ein Zettel,
//! der sich nicht als Text lesen laesst. Beide bekommen eine Meldung, und der
//! andere Zettel wird trotzdem uebernommen.

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use super::eintraege::ist_themenzeile;
use super::{Heimordner, ORDNERNAME, Sonderdatei};
use crate::text::datei::{self, Textstand, Unlesbarkeit};

/// Ein Zettel des Notizblatts der Runde 9: seine Datei im Ablageordner und das
/// Thema, unter dem er zur Notiz wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlterZettel {
    /// Der Dateiname im Ablageordner.
    pub datei: &'static str,
    /// Das Thema der Notiz, ohne `## `.
    pub thema: &'static str,
}

/// Die zwei alten Zettel, in der Reihenfolge, in der sie Notizen werden.
pub const ALTE_ZETTEL: [AlterZettel; 2] = [
    AlterZettel {
        datei: "note-1.txt",
        thema: "Zettel 1",
    },
    AlterZettel {
        datei: "note-2.txt",
        thema: "Zettel 2",
    },
];

/// Was aus einem alten Zettel geworden ist.
///
/// **Vollstaendig und ohne Auffangzweig**, damit ein weiterer Ausgang die
/// Meldungen anhaelt, bis er einen Satz hat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Zettelbefund {
    /// Er traegt Text und ergibt eine Notiz. Ob sie in `notes.txt` angekommen
    /// ist, sagt [`Uebernahme::ausgang`].
    Notiz,
    /// Er traegt nichts ausser Leerraum.
    Leer,
    /// Seine Datei gibt es nicht.
    Fehlt,
    /// Er traegt eine Zeile, die mit `## ` beginnt, und wird nicht uebernommen.
    Themenzeile,
    /// Er liess sich nicht als Text lesen; der Grund in einem Satzteil.
    Unlesbar(String),
}

/// Ob die Notizen aus den alten Zetteln in `notes.txt` angekommen sind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Uebernahmeausgang {
    /// `notes.txt` ist mit den Notizen neu entstanden.
    Geschrieben,
    /// Kein Zettel ergab eine Notiz; `notes.txt` entstand, wenn ueberhaupt, leer.
    NichtsZuUebernehmen,
    /// `notes.txt` stand schon, als sie angelegt werden sollte: der Wettlauf
    /// aus dem Modulkopf. Nichts ist geschrieben.
    NotizenStandenSchon,
    /// Das Anlegen oder Schreiben von `notes.txt` ist gescheitert; eine eben
    /// selbst angelegte Datei ist wieder entfernt. Der Grund in einem Satzteil.
    Gescheitert(String),
}

/// Die einmalige Uebernahme der alten Zettel, angestossen allein dadurch, dass
/// derselbe Aufruf den Ordner angelegt hat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uebernahme {
    /// Je alter Zettel sein Befund, in der Reihenfolge von [`ALTE_ZETTEL`].
    pub zettel: [(AlterZettel, Zettelbefund); 2],
    /// Ob die Notizen angekommen sind.
    pub ausgang: Uebernahmeausgang,
}

/// Was ein Aufruf von [`bereitstellen`] getan hat.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "traegt die Meldungen der Uebernahme; fallengelassen verschwindet ein abgewiesener Zettel ohne ein Wort"]
pub struct Bereitstellung {
    /// Ob dieser Aufruf den Ordner angelegt hat.
    pub ordner_angelegt: bool,
    /// Die Eintragsdateien, die dieser Aufruf angelegt hat.
    pub angelegt: Vec<Sonderdatei>,
    /// Die Eintragsdateien, die fehlten und sich nicht anlegen liessen, je mit
    /// dem Grund des Systems.
    pub nicht_angelegt: Vec<(Sonderdatei, String)>,
    /// Die Uebernahme, genau dann vorhanden, wenn `ordner_angelegt` gilt.
    pub uebernahme: Option<Uebernahme>,
}

/// Warum F2 keinen Tab oeffnet.
///
/// **Vollstaendig und ohne Auffangzweig.** Ein Hindernis betrifft den Ordner;
/// eine einzelne Datei, die sich nicht anlegen laesst, haelt den Tab nicht auf
/// und steht in [`Bereitstellung::nicht_angelegt`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hindernis {
    /// Das System nennt kein Benutzerverzeichnis. Erzeugt vom Rufer, der den
    /// [`Heimordner`] gar nicht erst bauen kann.
    KeinBenutzerverzeichnis,
    /// An der Stelle von `~/krkhome` steht etwas, das kein Ordner ist; es
    /// bleibt unveraendert.
    KeinOrdner,
    /// An der Stelle steht etwas, dessen Ziel sich nicht erreichen laesst,
    /// etwa ein Verweis ins Leere. Der Grund des Systems in einem Satzteil.
    Unerreichbar(String),
    /// Der Ordner fehlt und laesst sich nicht anlegen. Der Grund des Systems.
    NichtAnlegbar(String),
}

impl Hindernis {
    /// Der Satz fuer die Statuszeile.
    pub fn meldung(&self) -> String {
        let ordner = anzeigename();
        match self {
            Hindernis::KeinBenutzerverzeichnis => {
                format!("Das System nennt kein Benutzerverzeichnis, also gibt es kein {ordner}")
            }
            Hindernis::KeinOrdner => {
                format!("{ordner} ist kein Ordner; KRK legt dort nichts an und öffnet keinen Tab")
            }
            Hindernis::Unerreichbar(grund) => {
                format!("{ordner} ist nicht erreichbar: {grund}")
            }
            Hindernis::NichtAnlegbar(grund) => {
                format!("{ordner} lässt sich nicht anlegen: {grund}")
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
        if let Some(uebernahme) = &self.uebernahme {
            saetze.extend(uebernahme.meldungen());
        }
        for (sorte, grund) in &self.nicht_angelegt {
            saetze.push(format!(
                "{} lässt sich nicht anlegen: {grund}",
                sorte.dateiname()
            ));
        }
        saetze
    }
}

impl Uebernahme {
    fn meldungen(&self) -> Vec<String> {
        let notizen = Sonderdatei::Notizen.dateiname();
        let mut saetze = Vec::new();
        let tragen_notiz: Vec<&AlterZettel> = self
            .zettel
            .iter()
            .filter(|(_, befund)| *befund == Zettelbefund::Notiz)
            .map(|(zettel, _)| zettel)
            .collect();
        match &self.ausgang {
            Uebernahmeausgang::Geschrieben => {
                let themen: Vec<&str> = tragen_notiz.iter().map(|zettel| zettel.thema).collect();
                let als = if themen.len() == 1 { "Notiz" } else { "Notizen" };
                saetze.push(format!(
                    "{} als {als} in {notizen} übernommen",
                    themen.join(" und ")
                ));
            }
            Uebernahmeausgang::NichtsZuUebernehmen => {}
            Uebernahmeausgang::NotizenStandenSchon => saetze.push(format!(
                "Die alten Zettel sind nicht übernommen, weil {notizen} schon stand; {} liegen unverändert im Ablageordner",
                dateien(&tragen_notiz)
            )),
            Uebernahmeausgang::Gescheitert(grund) => saetze.push(format!(
                "Die alten Zettel sind nicht übernommen ({grund}); {} liegen unverändert im Ablageordner",
                dateien(&tragen_notiz)
            )),
        }
        for (zettel, befund) in &self.zettel {
            match befund {
                Zettelbefund::Notiz | Zettelbefund::Leer | Zettelbefund::Fehlt => {}
                Zettelbefund::Themenzeile => saetze.push(format!(
                    "{} ist nicht übernommen, weil er eine Zeile mit „## “ trägt; {} liegt unverändert im Ablageordner",
                    zettel.thema, zettel.datei
                )),
                Zettelbefund::Unlesbar(grund) => saetze.push(format!(
                    "{} ist nicht übernommen ({grund}); {} liegt unverändert im Ablageordner",
                    zettel.thema, zettel.datei
                )),
            }
        }
        saetze
    }
}

/// Die Dateinamen der genannten Zettel, mit „und“ verbunden.
fn dateien(zettel: &[&AlterZettel]) -> String {
    zettel
        .iter()
        .map(|zettel| zettel.datei)
        .collect::<Vec<_>>()
        .join(" und ")
}

/// Der Ordner, wie ihn die Statuszeile nennt.
fn anzeigename() -> String {
    format!("~/{ORDNERNAME}")
}

/// Legt den Heimordner an, falls er fehlt, darin jede fehlende Eintragsdatei,
/// und uebernimmt die alten Zettel, wenn dieser Aufruf den Ordner angelegt hat.
///
/// `ablageordner` ist der Ordner, in dem `note-1.txt` und `note-2.txt` liegen.
/// Angelegt wird an der geschriebenen Form des Heimordners; ist sie ein Verweis
/// auf einen Ordner, entstehen die Dateien in dessen Ziel. **Allein fuer F2
/// gedacht**; der Modulkopf sagt, warum der Start diesen Weg nicht erreicht.
#[must_use = "ein Hindernis heisst: kein Tab, und die Meldungen gehoeren in die Statuszeile"]
pub fn bereitstellen(heim: &Heimordner, ablageordner: &Path) -> Result<Bereitstellung, Hindernis> {
    anlegen_mit_vorlauf(heim, ablageordner, &mut |_| {})
}

/// [`bereitstellen`] mit einem Einhaengepunkt, der unmittelbar vor jedem
/// exklusiven Oeffnen mit dem Pfad der Datei gerufen wird.
///
/// Der Vorlauf ist allein fuer die Rennprobe im Pruefmodul da: sie legt die
/// Datei darin selbst an, also nach jeder denkbaren Pruefung und vor dem
/// Anlegen, und findet sie danach unveraendert.
fn anlegen_mit_vorlauf(
    heim: &Heimordner,
    ablageordner: &Path,
    vorlauf: &mut dyn FnMut(&Path),
) -> Result<Bereitstellung, Hindernis> {
    let ordner = heim.geschrieben.as_path();
    let ordner_angelegt = match fs::create_dir(ordner) {
        Ok(()) => true,
        Err(fehler) if fehler.kind() == io::ErrorKind::AlreadyExists => {
            // Es steht etwas da. `metadata` folgt einem Verweis, und genau das
            // ist die Frage: ist dort, wohin der Name fuehrt, ein Ordner?
            match fs::metadata(ordner) {
                Ok(angaben) if angaben.is_dir() => false,
                Ok(_) => return Err(Hindernis::KeinOrdner),
                Err(fehler) => return Err(Hindernis::Unerreichbar(fehler.to_string())),
            }
        }
        Err(fehler) => return Err(Hindernis::NichtAnlegbar(fehler.to_string())),
    };

    // Gelesen wird erst nach dem gelungenen `mkdir(2)`: vorher ist nicht
    // entschieden, ob dieser Aufruf ueberhaupt uebernimmt.
    let befunde = ordner_angelegt.then(|| zettel_lesen(ablageordner));

    let mut bereitstellung = Bereitstellung {
        ordner_angelegt,
        angelegt: Vec::new(),
        nicht_angelegt: Vec::new(),
        uebernahme: None,
    };
    for sorte in Sonderdatei::ALLE {
        let pfad = ordner.join(sorte.dateiname());
        let inhalt = match (sorte, &befunde) {
            (Sonderdatei::Notizen, Some((_, notizen))) => notizen.as_str(),
            (Sonderdatei::Notizen | Sonderdatei::Aufgaben, _) => "",
        };
        vorlauf(&pfad);
        let ausgang = exklusiv_anlegen(&pfad, inhalt);
        match &ausgang {
            Anlegeausgang::Angelegt => bereitstellung.angelegt.push(sorte),
            Anlegeausgang::StandSchon => {}
            Anlegeausgang::Gescheitert(grund) => {
                bereitstellung.nicht_angelegt.push((sorte, grund.clone()));
            }
        }
        if let (Sonderdatei::Notizen, Some((zettel, notizen))) = (sorte, &befunde) {
            bereitstellung.uebernahme = Some(Uebernahme {
                zettel: zettel.clone(),
                ausgang: uebernahmeausgang(notizen, ausgang),
            });
        }
    }
    Ok(bereitstellung)
}

/// Was aus dem Anlegen von `notes.txt` fuer die Uebernahme folgt.
fn uebernahmeausgang(notizen: &str, ausgang: Anlegeausgang) -> Uebernahmeausgang {
    if notizen.is_empty() {
        return Uebernahmeausgang::NichtsZuUebernehmen;
    }
    match ausgang {
        Anlegeausgang::Angelegt => Uebernahmeausgang::Geschrieben,
        Anlegeausgang::StandSchon => Uebernahmeausgang::NotizenStandenSchon,
        Anlegeausgang::Gescheitert(grund) => Uebernahmeausgang::Gescheitert(grund),
    }
}

/// Wie das exklusive Anlegen einer Datei ausgegangen ist.
enum Anlegeausgang {
    Angelegt,
    /// Die Datei stand schon und ist nicht angefasst.
    StandSchon,
    /// Der Grund des Systems; eine eben angelegte Datei ist wieder entfernt.
    Gescheitert(String),
}

/// Legt die Datei exklusiv an und schreibt den Inhalt hinein.
///
/// Scheitert das Schreiben mittendrin, wird die eben **selbst** angelegte
/// Datei wieder entfernt: sie gehoert diesem Aufruf, weil `create_new` gelungen
/// ist, und ein halber Stand darin saehe aus wie ein ganzer.
fn exklusiv_anlegen(pfad: &Path, inhalt: &str) -> Anlegeausgang {
    let mut datei = match OpenOptions::new().write(true).create_new(true).open(pfad) {
        Ok(datei) => datei,
        Err(fehler) if fehler.kind() == io::ErrorKind::AlreadyExists => {
            return Anlegeausgang::StandSchon;
        }
        Err(fehler) => return Anlegeausgang::Gescheitert(fehler.to_string()),
    };
    match datei.write_all(inhalt.as_bytes()) {
        Ok(()) => Anlegeausgang::Angelegt,
        Err(fehler) => {
            drop(datei);
            let grund = match fs::remove_file(pfad) {
                Ok(()) => fehler.to_string(),
                Err(entfernen) => format!(
                    "{fehler}; die angefangene Datei lässt sich nicht entfernen: {entfernen}"
                ),
            };
            Anlegeausgang::Gescheitert(grund)
        }
    }
}

/// Liest beide alten Zettel und setzt die Notizen zusammen, die aus ihnen
/// werden. Der Text ist leer, wenn kein Zettel eine Notiz ergibt.
fn zettel_lesen(ablageordner: &Path) -> ([(AlterZettel, Zettelbefund); 2], String) {
    let mut notizen = String::new();
    let befunde = ALTE_ZETTEL.map(|zettel| {
        let befund = match datei::lesen(&ablageordner.join(zettel.datei)) {
            Textstand::Text(text) => {
                if text.trim().is_empty() {
                    Zettelbefund::Leer
                } else if text.lines().any(ist_themenzeile) {
                    Zettelbefund::Themenzeile
                } else {
                    notizen.push_str("## ");
                    notizen.push_str(zettel.thema);
                    notizen.push('\n');
                    notizen.push_str(&text);
                    if !text.ends_with('\n') {
                        notizen.push('\n');
                    }
                    Zettelbefund::Notiz
                }
            }
            Textstand::Unlesbar { grund, .. } => Zettelbefund::Unlesbar(match grund {
                Unlesbarkeit::ZuGross(groesse) => format!("mit {groesse} Bytes zu groß"),
                Unlesbarkeit::KeinText => "kein lesbarer Text".to_owned(),
            }),
            Textstand::KeinGueltigesZiel { fehlt: true, .. } => Zettelbefund::Fehlt,
            Textstand::KeinGueltigesZiel { grund, .. } => Zettelbefund::Unlesbar(grund),
        };
        (zettel, befund)
    });
    (befunde, notizen)
}

#[cfg(test)]
mod proben {
    use std::path::PathBuf;

    use super::*;

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
        fs::create_dir_all(pfad.join("ablage")).expect("der Pruefordner laesst sich nicht anlegen");
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

        let bereitstellung = anlegen_mit_vorlauf(&heim, &zuhause.join("ablage"), &mut |pfad| {
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

    /// Der Wettlauf aus dem Modulkopf: der Ordner ist von diesem Aufruf
    /// angelegt, `notes.txt` entsteht davor aber anderswo. Die Uebernahme
    /// scheitert laut, und nichts ist ueberschrieben.
    #[test]
    fn ein_verlorenes_rennen_um_die_notizen_wird_gemeldet() {
        let zuhause = pruefpfad("rennen");
        let ablage = zuhause.join("ablage");
        fs::write(ablage.join("note-1.txt"), "alter Text\n").expect("note-1.txt");
        let heim = Heimordner::im_benutzerverzeichnis(&zuhause);

        let bereitstellung = anlegen_mit_vorlauf(&heim, &ablage, &mut |pfad| {
            if pfad.ends_with(Sonderdatei::Notizen.dateiname()) {
                fs::write(pfad, b"").expect("die fremde Datei laesst sich nicht schreiben");
            }
        })
        .expect("ein Hindernis, wo keines ist");

        assert!(bereitstellung.ordner_angelegt);
        let notizen = zuhause
            .join(ORDNERNAME)
            .join(Sonderdatei::Notizen.dateiname());
        assert_eq!(fs::read(&notizen).expect("notes.txt fehlt"), b"");
        assert_eq!(
            bereitstellung.uebernahme.as_ref().map(|u| &u.ausgang),
            Some(&Uebernahmeausgang::NotizenStandenSchon)
        );
        let meldungen = bereitstellung.meldungen().join("\n");
        assert!(
            meldungen.contains("nicht übernommen") && meldungen.contains("note-1.txt"),
            "{meldungen}"
        );
        assert_eq!(
            fs::read(ablage.join("note-1.txt")).expect("note-1.txt fehlt"),
            b"alter Text\n"
        );
        abraeumen(&zuhause);
    }
}
