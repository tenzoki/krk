//! Die von Hand gepflegten Ablagedateien auf den Zustand eines ersten
//! Starts zuruecksetzen, nachdem ihre alten Fassungen beiseitegelegt sind
//! (C2 des Spec `260929-0759_*_spec-werkseinstellungen-zuruecksetzen-und-neu-einlesen.md`).
//!
//! ```text
//! lage ──> beiseite (link) ──> vorbereitet (Nachbardateien) ──> vollzogen
//!   │            │                     │
//!   └── Abbruch <┴──── Rueckbau <──────┘
//! ```
//!
//! Welche Dateien es sind, sagt [`Werkszustand::fuer`] und keine Liste
//! daneben: `readers.toml` steht danach woertlich als ihre eingebettete
//! Auslieferungsfassung da, `settings.toml` als ihre Auslieferungsfassung mit
//! dem Wert von `notizordner` aus der alten Datei, `keymap.toml` fehlt, und
//! jede andere Ablagedatei bleibt unberuehrt.
//!
//! # Der Notizordner bleibt
//!
//! Der Nutzer hat am 260929 entschieden: „Der Notizordner bleibt immer, wie er
//! ist. Der Rest von settings.toml geht auf Werkseinstellung.“ (Spec, Abschnitt
//! „Änderung 260929“). Der Wert von `notizordner` kommt deshalb in seiner alten
//! Schreibweise in die neue Fassung, auch wenn er kein Text ist, und zwar ueber
//! dieselbe Ersetzung, ueber die „Ort waehlen…“ ihn schreibt
//! (`einstellungen::wert_einsetzen`). **Eine beschaedigte `settings.toml` bricht
//! ab, bevor etwas geschieht**: aus ihr laesst sich kein Wert uebernehmen, und
//! die Auslieferungsfassung an ihre Stelle zu schreiben, setzte beim naechsten
//! Start `~/krkhome` in Kraft, also genau den Wechsel, den der Befehl nie
//! ausloesen soll. „Beschaedigt“ heisst dabei dasselbe wie bei
//! `einstellungen::notizordner_schreiben`, denn beide fragen
//! `einstellungen::ortsstelle`. Keine Datei im Notizordner wird angefasst; dieses
//! Modul kennt nur den Ablageordner.
//!
//! # Vier Stufen unter einem Zugang
//!
//! [`zuruecksetzen`] haengt an einem [`Zugang`] und laeuft damit ganz unter der
//! Schreibsperre. Die Stufen, und jede vor der letzten laesst sich vollstaendig
//! zuruecknehmen:
//!
//! 1. **Die Lage.** Jede beruehrte Datei wird ueber `symlink_metadata`
//!    gefragt ([`lage`]). Ein symbolischer Verweis, auch ein verwaister, bricht
//!    ab: `rename` ersetzte ihn still durch eine gewoehnliche Datei, und die
//!    Datei, auf die er zeigt, bliebe beim alten Stand. Dieselbe Regel haelt
//!    `einstellungen::notizordner_schreiben`. Etwas anderes als eine
//!    gewoehnliche Datei bricht ebenso ab. Dieselbe Stufe stellt die neuen
//!    Fassungen fest und liest dafuer die alte `settings.toml`; ist sie
//!    beschaedigt oder nicht lesbar, bricht sie hier ab.
//! 2. **Beiseite.** Jede vorhandene Datei bekommt einen zweiten Namen,
//!    `<name>.<JJMMTT-HHMM>`, ueber `link(2)`.
//! 3. **Vorbereitet.** Die neuen Fassungen aus der Lage, fuer `settings.toml`
//!    und `readers.toml`, werden ueber [`atomar::vorbereiten`] vollstaendig in
//!    ihre Nachbardateien geschrieben; die Ziele sind dabei noch alt.
//! 4. **Vollzogen.** Beide Nachbardateien werden umbenannt, und `keymap.toml`
//!    wird entfernt.
//!
//! Scheitert Stufe 2 oder 3, entfernt der Vorgang die Sicherungsnamen, die er
//! in diesem Lauf angelegt hat, und die Nachbardateien fallen in ihrem `Drop`.
//! Keine der beruehrten Dateien ist dann geaendert (C2.7).
//!
//! # Warum ein harter Verweis und keine Kopie
//!
//! **Der alte Inhalt bleibt damit Byte fuer Byte derselbe Inode**, gleich wie
//! gross die Datei ist, und `link(2)` ueberschreibt nie: steht der Name schon,
//! scheitert es mit `EEXIST`. Ob ein Sicherungsname frei ist, entscheidet also
//! das Dateisystem selbst, und vorhergesagt wird nichts. Eine Kopie haette
//! entweder ein Groessenlimit gebraucht, wie `Zugang::beiseite_legen` es fuer
//! eine beschaedigte Datei traegt, oder einen zweiten Schreibweg an
//! [`atomar`] vorbei, und `EEXIST` waere dann eine Frage vor dem Schreiben
//! statt einer Antwort des Schreibens.
//!
//! Der Preis ist benannt: bis zum `rename` der letzten Stufe teilen Original
//! und Sicherung einen Inode. Scheitert dort ein `rename`, sind beide
//! dieselbe Datei, und ein Editor, der an Ort und Stelle schreibt, aenderte
//! beide. Der Ausgang nennt die Datei dann ausdruecklich als nicht
//! zurueckgesetzt.
//!
//! # Der Sicherungsname
//!
//! Der Stempel kommt aus `verzeichnis::sys::ortszeit` ueber den Zeitpunkt, den
//! der Rufer uebergibt; die Uhr liest dieses Modul nicht, damit der Vorgang mit
//! fester Zeit pruefbar ist. Alle Sicherungen eines Vorgangs tragen denselben
//! Stempel. In derselben Minute folgen `-2`, `-3` und so fort (C2.5); nach der
//! Nummer [`HOECHSTE_NUMMER`] bricht der Vorgang ab, statt unbegrenzt zu
//! zaehlen. Liefert die Ortszeit nichts, bricht er ab, bevor etwas geschieht.
//!
//! Die Sicherungen stehen nicht in [`Datei::ALLE`], KRK liest sie nie, und
//! niemand raeumt sie auf; sie stehen neben den `.beschaedigt`-Kopien aus
//! `Zugang::beiseite_legen`, mit einem anderen Namen und aus einem anderen
//! Grund.
//!
//! # Ein Fehler in der letzten Stufe wird gemeldet und nicht zurueckgebaut
//!
//! Scheitert dort ein `rename` oder das Entfernen, stehen die Sicherungen
//! schon (C2.3), und [`Zurueckgesetzt`] nennt je Datei, ob sie zurueckgesetzt
//! ist. Ein Rueckbau waere eine weitere Folge von Schreibvorgaengen mit eigenen
//! Fehlerfaellen, gegen einen Fall, der nach einer gelungenen, synchronisierten
//! Nachbardatei im selben Ordner allein bei einem fremden Eingriff eintritt.
//!
//! # Warum `keymap.toml` entfernt und nicht geschrieben wird
//!
//! „Werkseinstellungen" heisst hier der Zustand nach einem ersten Start, und
//! nach einem ersten Start gibt es keine `keymap.toml`. Eine geschriebene
//! Kopie der Auslieferungsbelegung braechte jede kuenftige neue Funktion ohne
//! Tastenkombination an, genau den offenen Defekt
//! `260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`
//! (Spec, C2, vierte Entscheidung).
//!
//! # Die verbleibende Luecke
//!
//! Die Sperre haelt allein eine zweite Instanz von KRK ab. Ein anderes
//! Programm, das sie nicht kennt, kann zwischen der Lage und dem `rename`
//! einen Verweis an die Stelle legen; dieselbe benannte Luecke steht im Kopf
//! von [`super::einstellungen`].

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::einstellungen::{self, Schreibhindernis};
use super::{Datei, Zugang, atomar, einzeilig, leseprofile};
use crate::verzeichnis::sys::ortszeit;

/// Die hoechste Nummer, die ein Sicherungsname in derselben Minute bekommt.
pub const HOECHSTE_NUMMER: u32 = 99;

/// Was das Zuruecksetzen an einer Ablagedatei tut.
///
/// **Vollstaendig ueber [`Datei`] und ohne Auffangzweig**, wie
/// [`Datei::leerbefund`] und [`Datei::ersatz`]: eine siebte Ablagedatei haelt
/// den Bau an [`Werkszustand::fuer`] an und erzwingt eine Einordnung.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Werkszustand {
    /// Die Datei steht danach woertlich mit diesem Text da.
    Wortlaut(&'static str),
    /// Die Datei steht danach als eingebettete Auslieferungsfassung von
    /// `settings.toml` da, in die der Wert von `notizordner` aus der alten
    /// Datei uebernommen ist (`einstellungen::auslieferung_mit_notizordner`).
    MitNotizordner,
    /// Die Datei fehlt danach.
    Fehlt,
    /// Das Zuruecksetzen fasst die Datei nicht an.
    Unberuehrt,
}

impl Werkszustand {
    /// Was das Zuruecksetzen an dieser Ablagedatei tut.
    ///
    /// Die beruehrten Dateien sind genau die, die
    /// [`super::neuerungen::Vergleichsform`] vergleicht; die Probe
    /// `die_zurueckgesetzten_dateien_sind_die_verglichenen` haelt beide Mengen
    /// gegeneinander.
    pub const fn fuer(welche: Datei) -> Self {
        match welche {
            Datei::Einstellungen => Werkszustand::MitNotizordner,
            Datei::Leser => Werkszustand::Wortlaut(leseprofile::AUSLIEFERUNGSTEXT),
            Datei::Belegung => Werkszustand::Fehlt,
            Datei::Lesezeichen | Datei::Sitzung | Datei::Merker => Werkszustand::Unberuehrt,
        }
    }

    /// Ob das Zuruecksetzen die Datei anfasst.
    pub const fn beruehrt(self) -> bool {
        !matches!(self, Werkszustand::Unberuehrt)
    }
}

/// Die Ablagedateien, die das Zuruecksetzen anfasst, in der Reihenfolge von
/// [`Datei::ALLE`].
fn beruehrte() -> impl Iterator<Item = Datei> {
    Datei::ALLE
        .into_iter()
        .filter(|&welche| Werkszustand::fuer(welche).beruehrt())
}

/// Warum das Zuruecksetzen nichts geaendert hat.
///
/// In jedem Fall sind die beruehrten Dateien danach, was sie vorher waren.
/// **Vollstaendig und ohne Auffangzweig**, wie [`Werkshindernis::meldung`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Werkshindernis {
    /// Die Datei ist ein symbolischer Verweis, auch ein verwaister.
    Verweis(Datei),
    /// An der Stelle der Datei steht etwas anderes als eine gewoehnliche Datei.
    KeineDatei(Datei),
    /// Die Frage nach der Datei ist gescheitert. Traegt die Meldung des
    /// Systems.
    NichtLesbar(Datei, String),
    /// Der Zeitpunkt laesst sich nicht in Ortszeit umrechnen.
    KeineOrtszeit,
    /// Jeder Sicherungsname bis zur Nummer [`HOECHSTE_NUMMER`] ist vergeben.
    KeinFreierName(Datei),
    /// Der harte Verweis auf die alte Fassung ist gescheitert. Traegt die
    /// Meldung des Systems.
    NichtBeiseitegelegt(Datei, String),
    /// Die Nachbardatei der neuen Fassung ist gescheitert. Traegt die Meldung
    /// des Systems.
    NichtVorbereitet(Datei, String),
    /// Aus der alten `settings.toml` laesst sich der Wert von `notizordner`
    /// nicht uebernehmen, weil sie beschaedigt ist. Traegt den Befund, den
    /// auch „Ort waehlen…“ gaebe.
    Einstellungen(Schreibhindernis),
    /// Das Hindernis ist eingetreten, und danach liess sich mindestens ein
    /// Sicherungsname dieses Laufs nicht wieder entfernen.
    ///
    /// Die beruehrten Dateien sind trotzdem unveraendert; unter den genannten Pfaden
    /// liegt ihr unveraenderter Inhalt ein zweites Mal.
    NichtZurueckgebaut {
        /// Das Hindernis, das den Rueckbau ausgeloest hat.
        hindernis: Box<Werkshindernis>,
        /// Die Sicherungsnamen, die stehen geblieben sind.
        liegen: Vec<PathBuf>,
    },
}

impl Werkshindernis {
    /// Der Satz fuer die Statuszeile: die Datei und der Grund, und dass nichts
    /// zurueckgesetzt ist.
    #[must_use]
    pub fn meldung(&self) -> String {
        match self {
            Werkshindernis::Verweis(welche) => format!(
                "{} ist ein symbolischer Verweis, und KRK ersetzt ihn nicht durch eine Datei; nichts ist zurückgesetzt.",
                welche.dateiname()
            ),
            Werkshindernis::KeineDatei(welche) => format!(
                "An der Stelle von {} steht keine gewöhnliche Datei; nichts ist zurückgesetzt.",
                welche.dateiname()
            ),
            Werkshindernis::NichtLesbar(welche, befund) => format!(
                "{} lässt sich nicht befragen ({befund}); nichts ist zurückgesetzt.",
                welche.dateiname()
            ),
            Werkshindernis::KeineOrtszeit => String::from(
                "Die Uhr dieses Geräts ergibt keinen Zeitstempel für die Sicherungen; nichts ist zurückgesetzt.",
            ),
            Werkshindernis::KeinFreierName(welche) => format!(
                "Für {} ist in dieser Minute kein freier Sicherungsname mehr übrig; nichts ist zurückgesetzt.",
                welche.dateiname()
            ),
            Werkshindernis::NichtBeiseitegelegt(welche, befund) => format!(
                "{} ließ sich nicht beiseitelegen ({befund}); nichts ist zurückgesetzt.",
                welche.dateiname()
            ),
            Werkshindernis::NichtVorbereitet(welche, befund) => format!(
                "Die neue Fassung von {} ließ sich nicht schreiben ({befund}); nichts ist zurückgesetzt.",
                welche.dateiname()
            ),
            Werkshindernis::Einstellungen(befund) => {
                format!("{}. Nichts ist zurückgesetzt.", befund.meldung())
            }
            Werkshindernis::NichtZurueckgebaut { hindernis, liegen } => format!(
                "{} Liegen geblieben ist eine zweite Kopie des unveränderten Inhalts unter {}.",
                hindernis.meldung(),
                pfadliste(liegen)
            ),
        }
    }
}

/// Die Pfade, durch Kommas getrennt, voll ausgeschrieben.
fn pfadliste(pfade: &[PathBuf]) -> String {
    pfade
        .iter()
        .map(|pfad| pfad.display().to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

/// Ob eine beruehrte Ablagedatei vor dem Vorgang dasteht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stand {
    /// Eine gewoehnliche Datei steht da.
    Steht,
    /// An ihrer Stelle steht nichts.
    Fehlt,
}

/// Die Lage der beruehrten Ablagedateien, wie [`lage`] sie gefunden hat,
/// samt ihren neuen Fassungen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lage {
    staende: Vec<(Datei, Stand)>,
    /// Je Datei, die danach dasteht, ihr neuer Text.
    neue_fassungen: Vec<(Datei, String)>,
}

impl Lage {
    /// Ob diese Ablagedatei dasteht.
    ///
    /// Eine Datei, die das Zuruecksetzen nicht anfasst, hat keine Lage und
    /// wird mit `false` beantwortet.
    #[must_use]
    pub fn steht(&self, welche: Datei) -> bool {
        self.staende
            .iter()
            .any(|&(datei, stand)| datei == welche && stand == Stand::Steht)
    }
}

/// Fragt jede beruehrte Ablagedatei, ob sie steht, fehlt oder das
/// Zuruecksetzen verhindert, und stellt ihre neue Fassung fest.
///
/// **Ohne Sperre aufrufbar**, weil sie nur fragt und liest; die Oberflaeche
/// fragt so vor der Rueckfrage, was diese sagen muss, und bricht bei einem
/// Verweis oder einer beschaedigten `settings.toml` ab, bevor sie aufgeht.
/// [`zuruecksetzen`] fragt unter der Sperre ein zweites Mal selbst. Jede
/// Datei wird dabei hoechstens einmal gelesen, und gelesen wird allein die
/// alte `settings.toml`.
pub fn lage(pfad: impl Fn(Datei) -> PathBuf) -> Result<Lage, Werkshindernis> {
    let mut staende = Vec::new();
    let mut neue_fassungen = Vec::new();
    for welche in beruehrte() {
        let stelle = pfad(welche);
        let stand = match fs::symlink_metadata(&stelle) {
            Ok(art) if art.file_type().is_symlink() => {
                return Err(Werkshindernis::Verweis(welche));
            }
            Ok(art) if art.is_file() => Stand::Steht,
            Ok(_) => return Err(Werkshindernis::KeineDatei(welche)),
            Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => Stand::Fehlt,
            Err(fehler) => {
                return Err(Werkshindernis::NichtLesbar(
                    welche,
                    einzeilig(&fehler.to_string()),
                ));
            }
        };
        staende.push((welche, stand));
        if let Some(fassung) = neue_fassung(welche, &stelle, stand)? {
            neue_fassungen.push((welche, fassung));
        }
    }
    Ok(Lage {
        staende,
        neue_fassungen,
    })
}

/// Der Text, den eine beruehrte Datei nach dem Zuruecksetzen traegt; `None`,
/// wenn sie danach fehlt.
fn neue_fassung(
    welche: Datei,
    stelle: &Path,
    stand: Stand,
) -> Result<Option<String>, Werkshindernis> {
    match Werkszustand::fuer(welche) {
        Werkszustand::Wortlaut(text) => Ok(Some(text.to_owned())),
        Werkszustand::MitNotizordner => {
            let alt = match stand {
                Stand::Steht => {
                    let bytes = fs::read(stelle).map_err(|fehler| {
                        Werkshindernis::NichtLesbar(welche, einzeilig(&fehler.to_string()))
                    })?;
                    Some(einstellungen::als_text(bytes).map_err(Werkshindernis::Einstellungen)?)
                }
                Stand::Fehlt => None,
            };
            einstellungen::auslieferung_mit_notizordner(alt.as_deref())
                .map(Some)
                .map_err(Werkshindernis::Einstellungen)
        }
        Werkszustand::Fehlt | Werkszustand::Unberuehrt => Ok(None),
    }
}

/// Was aus einer beruehrten Ablagedatei geworden ist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dateiausgang {
    /// Welche Ablagedatei.
    pub welche: Datei,
    /// Der Pfad, unter dem ihre alte Fassung liegt; `None`, wenn sie vorher
    /// nicht dastand.
    pub sicherung: Option<PathBuf>,
    /// Ob die letzte Stufe an ihr gelungen ist; sonst die Meldung des Systems.
    pub vollzug: Result<(), String>,
}

/// Der Ausgang eines Zuruecksetzens, je beruehrter Ablagedatei.
#[must_use = "der Ausgang nennt die Sicherungen und jede Datei, die nicht zurueckgesetzt ist"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zurueckgesetzt {
    /// Je beruehrter Ablagedatei ein Ausgang, in der Reihenfolge von
    /// [`Datei::ALLE`].
    pub dateien: Vec<Dateiausgang>,
}

impl Zurueckgesetzt {
    /// Ob jede beruehrte Datei zurueckgesetzt ist.
    #[must_use]
    pub fn vollstaendig(&self) -> bool {
        self.dateien.iter().all(|ausgang| ausgang.vollzug.is_ok())
    }

    /// Der Satz fuer die Statuszeile: die vollen Pfade der Sicherungen, jede
    /// Datei ohne Sicherung und jede, die nicht zurueckgesetzt ist (C2.4,
    /// C2.8).
    #[must_use]
    pub fn meldung(&self) -> String {
        let mut saetze = vec![if self.vollstaendig() {
            String::from("Auf Werkseinstellungen zurückgesetzt.")
        } else {
            String::from("Nur teilweise auf Werkseinstellungen zurückgesetzt.")
        }];
        let gesichert: Vec<PathBuf> = self
            .dateien
            .iter()
            .filter_map(|ausgang| ausgang.sicherung.clone())
            .collect();
        if !gesichert.is_empty() {
            saetze.push(format!("Beiseitegelegt: {}.", pfadliste(&gesichert)));
        }
        for ausgang in &self.dateien {
            let name = ausgang.welche.dateiname();
            if ausgang.sicherung.is_none() {
                saetze.push(format!(
                    "{name} stand nicht da, für sie ist nichts beiseitegelegt."
                ));
            }
            if let Err(fehler) = &ausgang.vollzug {
                saetze.push(format!("{name} ist nicht zurückgesetzt: {fehler}."));
            }
        }
        saetze.join(" ")
    }
}

/// Der Stempel `JJMMTT-HHMM` zu einem Zeitpunkt in Ortszeit.
fn stempel(zeitpunkt: SystemTime) -> Option<String> {
    let zeit = ortszeit(zeitpunkt)?;
    Some(format!(
        "{:02}{:02}{:02}-{:02}{:02}",
        zeit.jahr.rem_euclid(100),
        zeit.monat,
        zeit.tag,
        zeit.stunde,
        zeit.minute
    ))
}

/// Der Sicherungsname zu einer Datei, einem Stempel und einer Nummer; die
/// Nummer 1 traegt keine Endung.
fn sicherungspfad(datei: &Path, welche: Datei, stempel: &str, nummer: u32) -> PathBuf {
    let name = if nummer == 1 {
        format!("{}.{stempel}", welche.dateiname())
    } else {
        format!("{}.{stempel}-{nummer}", welche.dateiname())
    };
    datei.with_file_name(name)
}

/// Legt die Datei ueber einen harten Verweis unter dem ersten freien
/// Sicherungsnamen beiseite.
fn beiseitelegen(datei: &Path, welche: Datei, stempel: &str) -> Result<PathBuf, Werkshindernis> {
    for nummer in 1..=HOECHSTE_NUMMER {
        let ziel = sicherungspfad(datei, welche, stempel, nummer);
        match fs::hard_link(datei, &ziel) {
            Ok(()) => return Ok(ziel),
            Err(fehler) if fehler.kind() == io::ErrorKind::AlreadyExists => {}
            Err(fehler) => {
                return Err(Werkshindernis::NichtBeiseitegelegt(
                    welche,
                    einzeilig(&fehler.to_string()),
                ));
            }
        }
    }
    Err(Werkshindernis::KeinFreierName(welche))
}

/// Entfernt die Sicherungsnamen dieses Laufs wieder und reicht das Hindernis
/// weiter, bei einem gescheiterten Entfernen samt den stehen gebliebenen
/// Namen.
fn rueckbau(angelegt: &[(Datei, PathBuf)], hindernis: Werkshindernis) -> Werkshindernis {
    let liegen: Vec<PathBuf> = angelegt
        .iter()
        .filter(|(_, pfad)| fs::remove_file(pfad).is_err())
        .map(|(_, pfad)| pfad.clone())
        .collect();
    if liegen.is_empty() {
        hindernis
    } else {
        Werkshindernis::NichtZurueckgebaut {
            hindernis: Box::new(hindernis),
            liegen,
        }
    }
}

/// Legt die alten Fassungen der von Hand gepflegten Ablagedateien
/// beiseite und stellt den Zustand eines ersten Starts her.
///
/// Die vier Stufen, der Rueckbau und der Ausgang stehen im Modulkopf. Der
/// Zeitpunkt kommt vom Rufer und ergibt den Stempel der Sicherungen.
///
/// Ein `Err` heisst: keine beruehrte Datei ist geaendert. Ein `Ok` nennt je
/// Datei die Sicherung und ob die letzte Stufe an ihr gelungen ist.
#[must_use = "ein Hindernis oder eine nicht zurueckgesetzte Datei faellt sonst niemandem auf"]
pub fn zuruecksetzen(
    zugang: &Zugang<'_>,
    zeitpunkt: SystemTime,
) -> Result<Zurueckgesetzt, Werkshindernis> {
    let stempel = stempel(zeitpunkt).ok_or(Werkshindernis::KeineOrtszeit)?;
    let lage = lage(|welche| zugang.pfad(welche))?;

    // Stufe 2: jede vorhandene Datei bekommt ihren Sicherungsnamen.
    let mut angelegt: Vec<(Datei, PathBuf)> = Vec::new();
    for welche in beruehrte() {
        if !lage.steht(welche) {
            continue;
        }
        match beiseitelegen(&zugang.pfad(welche), welche, &stempel) {
            Ok(sicherung) => angelegt.push((welche, sicherung)),
            Err(hindernis) => return Err(rueckbau(&angelegt, hindernis)),
        }
    }

    // Stufe 3: die neuen Fassungen stehen vollstaendig in ihren
    // Nachbardateien, die Ziele sind noch alt.
    let mut vorbereitet: Vec<(Datei, atomar::Nachbardatei)> = Vec::new();
    for (welche, text) in &lage.neue_fassungen {
        let welche = *welche;
        match atomar::vorbereiten(&zugang.pfad(welche), &mut text.as_bytes()) {
            Ok(nachbar) => vorbereitet.push((welche, nachbar)),
            Err(fehler) => {
                // Die schon vorbereiteten Nachbardateien fallen mit
                // `vorbereitet` und raeumen sich dabei selbst ab.
                drop(vorbereitet);
                let hindernis =
                    Werkshindernis::NichtVorbereitet(welche, einzeilig(&fehler.to_string()));
                return Err(rueckbau(&angelegt, hindernis));
            }
        }
    }

    // Stufe 4: umbenennen und entfernen. Ab hier wird nichts zurueckgebaut.
    let mut dateien = Vec::new();
    for welche in beruehrte() {
        let vollzug = match Werkszustand::fuer(welche) {
            Werkszustand::Wortlaut(_) | Werkszustand::MitNotizordner => {
                let stelle = vorbereitet
                    .iter()
                    .position(|(datei, _)| *datei == welche)
                    .expect("jede Datei mit neuer Fassung ist in Stufe 3 vorbereitet");
                let (_, nachbar) = vorbereitet.swap_remove(stelle);
                nachbar.umbenennen()
            }
            Werkszustand::Fehlt => {
                if lage.steht(welche) {
                    match fs::remove_file(zugang.pfad(welche)) {
                        Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => Ok(()),
                        anderes => anderes,
                    }
                } else {
                    Ok(())
                }
            }
            Werkszustand::Unberuehrt => continue,
        };
        let sicherung = angelegt
            .iter()
            .find(|(datei, _)| *datei == welche)
            .map(|(_, pfad)| pfad.clone());
        dateien.push(Dateiausgang {
            welche,
            sicherung,
            vollzug: vollzug.map_err(|fehler| einzeilig(&fehler.to_string())),
        });
    }
    Ok(Zurueckgesetzt { dateien })
}
