//! Die Bildfolge eines Profils: welche Fotos, aus welchen Ordnern, in welcher
//! Reihenfolge.
//!
//! Die Arbeit ist in zwei Stufen geschnitten, und der Schnitt traegt die
//! Zusage, dass das erste Foto eines Jahresordners schnell erscheint:
//!
//! ```text
//! ausgewaehlter Ordner
//!   └─ verzeichnis_erheben ──> Bildverzeichnis   (Verzeichnisleselaeufe, keine Datei)
//!        └─ je Gruppe: gruppe_ordnen ──> Vec<Foto>   (ein Datum je Foto)
//! ```
//!
//! **Die Erhebung oeffnet keine Datei.** Sie liest die Ordner der Folge, nimmt
//! daraus die Fotos und kennt danach die Gruppen und die Gesamtzahl. Das
//! erste Foto haengt allein an der ersten Gruppe, und erst deren Ordnung
//! liest Aufnahmedaten. Wer die Stufen zusammenlegt, laesst das erste Foto auf
//! die Daten aller Monate warten.
//!
//! # Gruppen und Reihenfolge (C2 des Spec der Bildfolge)
//!
//! Eine Gruppe ist ein Ordner, aus dem Fotos stammen: ohne Platzhalter der
//! eine Ordner der Ortsangabe (ein Monat), mit `*` jeder Unterordner (ein
//! Jahr). **Gruppen laufen nach Namen, Fotos innerhalb einer Gruppe nach
//! Aufnahmedatum**; ohne lesbares Datum gilt das Aenderungsdatum, bei gleichem
//! Zeitpunkt entscheidet der Name. Beide Namensvergleiche nehmen den
//! Kollationsschluessel der Namensspalte (`Eintrag::sortierschluessel`), also
//! dieselbe Ordnung, die der Nutzer in der Dateiliste sieht.
//!
//! Ein Ordner ohne Fotos fehlt in der Gruppenliste (C2.5). Fotos direkt im
//! Jahresordner und in tieferen Ordnern gehoeren nicht dazu (C2.6): gelesen
//! wird allein die eine Ebene je Gruppe.
//!
//! # Was ein Foto ist
//!
//! Ein Eintrag vom Typ Datei, nie eine Verknuepfung und nie ein Ordner, mit
//! einer der Endungen aus [`crate::bild::ENDUNGEN`], ohne Ruecksicht auf die
//! Schreibung (C2.1). **Ein versteckter Eintrag, dessen Name mit einem Punkt
//! beginnt, zaehlt nicht** (Nutzerentscheid vom 260929): auf Datentraegern mit
//! fremdem Dateisystem liegt neben jedem Foto eine AppleDouble-Datei
//! `._IMG_0970.jpg`, die sonst als Foto mit ihren Metadaten in der Folge
//! stuende.
//!
//! # Grenzen und Kuerzung
//!
//! Gezaehlt wird im [`Bildhaushalt`] gegen [`HOECHSTENS_FOTOS`],
//! [`HOECHSTENS_BILDGRUPPEN`] und [`HOECHSTENS_EINTRAEGE_JE_BILDORDNER`].
//! **Gekuerzt wird in der Reihenfolge der Folge** (Entscheidung 4 des Plans):
//! Gruppen kommen nach Namen hinein, solange Platz ist; die Gruppe, an der die
//! Grenze faellt, wird ganz geordnet und traegt nur ihre fruehesten Fotos bei
//! ([`Gruppe::beitrag`]). Ist die Grenze erreicht, werden die uebrigen Ordner
//! nicht mehr gelesen, und die Folge gilt als gekuerzt, auch wenn darin keine
//! Fotos mehr laegen. Eine Gruppe, deren Lesung an der Eintragsgrenze abbricht,
//! traegt bei, was gelesen ist, und macht die Folge ebenfalls gekuerzt.
//!
//! **Die Kuerzung traegt ihren Grund** ([`Kuerzung`], je [`Grenze`] ein
//! Merkmal): die Statuszeile nennt die Grenze, die gegriffen hat, und nicht
//! immer die Fotogrenze. Eine Folge von 900 Fotos, die an der Ordner- oder an
//! der Eintragsgrenze gekuerzt ist, hiesse sonst „nach 7.500 Fotos gekuerzt“
//! (`issues/260929-1646_*_der-zaehler-nennt-jede-gekuerzte-folge-*.md` im
//! Arbeitspaket `260929-1415-vorschau-blaettert-fotos-nach-aufnahmedatum`).
//!
//! # Abbruch
//!
//! [`Bildverzeichnis::gruppe_ordnen`] fragt vor jedem Foto `weiter` und hoert
//! auf, sobald es nein sagt (C5.4). Die Vorschau haengt daran eine Marke, die
//! ein Auswahlwechsel setzt.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::bild::{Aufnahmezeit, Datumsleser, ist_fotoname};
use crate::verzeichnis::leser;
use crate::verzeichnis::{Eintrag, Typ};

use super::bausteine::innerhalb;
use super::{
    Bildfolgeangabe, Bildhaushalt, HOECHSTENS_BILDGRUPPEN, HOECHSTENS_EINTRAEGE_JE_BILDORDNER,
    HOECHSTENS_FOTOS,
};

/// Ein Foto, wie die Erhebung es kennt: aus dem Verzeichnisleselauf und ohne
/// Oeffnung.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fotoeintrag {
    name: String,
    schluessel: Box<[u8]>,
    geaendert: SystemTime,
}

impl Fotoeintrag {
    /// Der Name ohne Pfad.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Das Aenderungsdatum aus dem Leselauf.
    pub fn geaendert(&self) -> SystemTime {
        self.geaendert
    }
}

/// Ein Ordner, aus dem Fotos der Folge stammen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gruppe {
    ordner: PathBuf,
    fotos: Vec<Fotoeintrag>,
    beitrag: usize,
}

impl Gruppe {
    /// Der Ordner, in der Schreibweise des ausgewaehlten Pfades und nicht
    /// aufgeloest: ein Sprung dorthin fuehrt die Dateiliste an den Ort, den
    /// der Nutzer kennt.
    pub fn ordner(&self) -> &Path {
        &self.ordner
    }

    /// Die Fotos des Ordners in Lesereihenfolge, noch nicht geordnet.
    pub fn fotos(&self) -> &[Fotoeintrag] {
        &self.fotos
    }

    /// Wie viele davon in die Folge kommen. Kleiner als die Zahl der Fotos
    /// allein in der Gruppe, an der die Folge gekuerzt wird.
    pub fn beitrag(&self) -> usize {
        self.beitrag
    }
}

/// Ein geordnetes Foto der Folge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Foto {
    pfad: PathBuf,
    zeit: Option<Aufnahmezeit>,
    aus_bilddaten: bool,
}

impl Foto {
    /// Der volle Pfad, in der Schreibweise des Gruppenordners.
    pub fn pfad(&self) -> &Path {
        &self.pfad
    }

    /// Der Zeitpunkt, nach dem es eingeordnet ist. `None` allein dann, wenn
    /// weder die Bilddaten noch das Aenderungsdatum einen Kalendertag tragen;
    /// ein solches Foto steht vor den uebrigen.
    pub fn zeit(&self) -> Option<Aufnahmezeit> {
        self.zeit
    }

    /// Ob der Zeitpunkt aus den Bilddaten stammt und nicht aus dem
    /// Aenderungsdatum.
    pub fn aus_bilddaten(&self) -> bool {
        self.aus_bilddaten
    }
}

/// Eine Grenze, an der eine Bildfolge gekuerzt werden kann.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grenze {
    /// [`HOECHSTENS_FOTOS`]: die Folge ist voll, oder die Gruppe an der Grenze
    /// traegt nur ihre fruehesten Fotos bei.
    Fotos,
    /// [`HOECHSTENS_BILDGRUPPEN`]: weitere Ordner werden nicht mehr gelesen,
    /// ob als Gruppen gezaehlt oder als Leselaeufe, die auch ein Ordner ohne
    /// Fotos kostet.
    Ordner,
    /// [`HOECHSTENS_EINTRAEGE_JE_BILDORDNER`]: ein Leselauf ist abgeschnitten,
    /// und was darin fehlt, haengt von der Lesereihenfolge des Verzeichnisses
    /// ab.
    Eintraege,
}

impl Grenze {
    /// Die Zahl, bei der die Grenze greift.
    #[must_use]
    pub fn hoechstens(self) -> usize {
        match self {
            Self::Fotos => HOECHSTENS_FOTOS,
            Self::Ordner => HOECHSTENS_BILDGRUPPEN,
            Self::Eintraege => HOECHSTENS_EINTRAEGE_JE_BILDORDNER,
        }
    }
}

/// Welche Grenzen eine Bildfolge gekuerzt haben; ohne eine ist sie
/// vollstaendig. Es koennen mehrere zugleich gegriffen haben.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Kuerzung {
    fotos: bool,
    ordner: bool,
    eintraege: bool,
}

impl Kuerzung {
    /// Dieselbe Kuerzung, zusaetzlich an der genannten Grenze.
    #[must_use]
    pub fn mit(mut self, grenze: Grenze) -> Self {
        match grenze {
            Grenze::Fotos => self.fotos = true,
            Grenze::Ordner => self.ordner = true,
            Grenze::Eintraege => self.eintraege = true,
        }
        self
    }

    /// Ob irgendeine Grenze gegriffen hat.
    #[must_use]
    pub fn ist_gekuerzt(self) -> bool {
        self.fotos || self.ordner || self.eintraege
    }

    /// Die Grenzen, die gegriffen haben, in fester Reihenfolge: Fotos,
    /// Ordner, Eintraege.
    #[must_use]
    pub fn grenzen(self) -> Vec<Grenze> {
        [
            (self.fotos, Grenze::Fotos),
            (self.ordner, Grenze::Ordner),
            (self.eintraege, Grenze::Eintraege),
        ]
        .into_iter()
        .filter_map(|(gegriffen, grenze)| gegriffen.then_some(grenze))
        .collect()
    }
}

/// Was die Erhebung ueber eine Bildfolge weiss, bevor ein Datum gelesen ist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bildverzeichnis {
    gruppen: Vec<Gruppe>,
    gesamt: usize,
    kuerzung: Kuerzung,
    haushalt: Bildhaushalt,
}

impl Bildverzeichnis {
    /// Die Gruppen nach Namen, jede mit mindestens einem Foto.
    pub fn gruppen(&self) -> &[Gruppe] {
        &self.gruppen
    }

    /// Wie viele Fotos die Folge traegt, hoechstens [`HOECHSTENS_FOTOS`].
    pub fn gesamt(&self) -> usize {
        self.gesamt
    }

    /// Ob eine Grenze Fotos oder Ordner aus der Folge gelassen hat.
    pub fn ist_gekuerzt(&self) -> bool {
        self.kuerzung.ist_gekuerzt()
    }

    /// Welche Grenzen die Folge gekuerzt haben.
    pub fn kuerzung(&self) -> Kuerzung {
        self.kuerzung
    }

    /// Was die Erhebung verbraucht hat.
    pub fn haushalt(&self) -> Bildhaushalt {
        self.haushalt
    }

    /// Die Gruppe und die Stelle darin, an der das Foto mit der Stelle
    /// `stelle` der ganzen Folge steht, von null gezaehlt.
    #[must_use]
    pub fn ort_der_stelle(&self, stelle: usize) -> Option<(usize, usize)> {
        let mut vorher = 0;
        for (index, gruppe) in self.gruppen.iter().enumerate() {
            if stelle < vorher + gruppe.beitrag {
                return Some((index, stelle - vorher));
            }
            vorher += gruppe.beitrag;
        }
        None
    }

    /// Ordnet eine Gruppe nach Aufnahmedatum und liefert ihren Beitrag zur
    /// Folge.
    ///
    /// Je Foto einmal `leser`, davor jedes Mal `weiter`; sagt `weiter` nein,
    /// hoert die Ordnung auf und liefert `None`, ohne ein weiteres Datum zu
    /// lesen (C5.4). Ein Foto ohne Datum aus den Bilddaten bekommt sein
    /// Aenderungsdatum und bricht nichts ab (C5.5). `None` heisst auch: es
    /// gibt keine Gruppe mit diesem Index.
    #[must_use = "die geordnete Gruppe ist das Ergebnis der Datumslesungen"]
    pub fn gruppe_ordnen(
        &self,
        index: usize,
        leser: &Datumsleser<'_>,
        weiter: &dyn Fn() -> bool,
    ) -> Option<Vec<Foto>> {
        let gruppe = self.gruppen.get(index)?;
        let mut bewertet: Vec<(&Fotoeintrag, Foto)> = Vec::with_capacity(gruppe.fotos.len());
        for eintrag in &gruppe.fotos {
            if !weiter() {
                return None;
            }
            let pfad = gruppe.ordner.join(&eintrag.name);
            let (zeit, aus_bilddaten) = match leser(&pfad) {
                Some(zeit) => (Some(zeit), true),
                None => (Aufnahmezeit::aus_zeitpunkt(eintrag.geaendert), false),
            };
            bewertet.push((
                eintrag,
                Foto {
                    pfad,
                    zeit,
                    aus_bilddaten,
                },
            ));
        }
        bewertet.sort_by(|(links_eintrag, links), (rechts_eintrag, rechts)| {
            links
                .zeit
                .cmp(&rechts.zeit)
                .then_with(|| links_eintrag.schluessel.cmp(&rechts_eintrag.schluessel))
                .then_with(|| links_eintrag.name.cmp(&rechts_eintrag.name))
        });
        bewertet.truncate(gruppe.beitrag);
        Some(bewertet.into_iter().map(|(_, foto)| foto).collect())
    }
}

/// Ob ein Eintrag als Foto zaehlt; die Regel steht im Modulkopf.
fn ist_foto(eintrag: &Eintrag) -> bool {
    eintrag.typ == Typ::Datei && !eintrag.name.starts_with('.') && ist_fotoname(&eintrag.name)
}

/// Erhebt die Gruppen einer Bildfolge, ohne eine Datei zu oeffnen.
///
/// `ausgewaehlt` ist der Pfad, den der Nutzer ausgewaehlt hat, `wurzel` derselbe
/// Ordner aufgeloest. Gelesen wird am aufgeloesten Pfad und gegen ihn als
/// Schranke gehalten, dieselbe wie fuer die Ortsangabe eines Bausteins (C3.13
/// der Runde 16); ein Unterordner, der aufgeloest ausserhalb liegt, wird
/// uebergangen. Der Platzhalter greift allein Eintraege vom Typ Ordner und
/// damit keine Verknuepfung. Die Pfade der Gruppen tragen dagegen die
/// Schreibweise des ausgewaehlten Pfades.
///
/// `None` heisst: der Ort der Angabe laesst sich nicht aufloesen, liegt
/// ausserhalb oder laesst sich nicht lesen. Ein Verzeichnis ohne ein einziges
/// Foto ist dagegen eine Antwort mit null Gruppen, und der Rufer zeigt dann
/// die Zeilen des Profils (C1.5).
#[must_use = "das Verzeichnis ist das Ergebnis der Leselaeufe"]
pub fn verzeichnis_erheben(
    angabe: &Bildfolgeangabe,
    ausgewaehlt: &Path,
    wurzel: &Path,
) -> Option<Bildverzeichnis> {
    let ort = angabe.ort();
    let mut haushalt = Bildhaushalt::neu();
    let mut kuerzung = Kuerzung::default();

    let mut anzeige = ausgewaehlt.to_path_buf();
    let mut lesepfad = wurzel.to_path_buf();
    for teil in ort.teile() {
        anzeige.push(teil);
        lesepfad.push(teil);
    }
    let lesepfad = innerhalb(wurzel, &lesepfad)?;

    // Die Ordner der Folge in ihrer Reihenfolge: je einer mit dem Pfad fuer
    // die Anzeige und dem fuer das Lesen.
    let ordner: Vec<(PathBuf, PathBuf)> = match ort.hinter_dem_platzhalter() {
        None => vec![(anzeige, lesepfad)],
        Some(hinter) => {
            if !haushalt.leselauf_nehmen() {
                return None;
            }
            let stand =
                leser::lesen_hoechstens(&lesepfad, HOECHSTENS_EINTRAEGE_JE_BILDORDNER).ok()?;
            if stand.abgeschnitten {
                kuerzung = kuerzung.mit(Grenze::Eintraege);
            }
            let mut unter: Vec<&Eintrag> = stand
                .eintraege
                .iter()
                .filter(|eintrag| eintrag.typ == Typ::Ordner)
                .collect();
            unter.sort_by(|links, rechts| {
                links
                    .sortierschluessel
                    .cmp(&rechts.sortierschluessel)
                    .then_with(|| links.name.cmp(&rechts.name))
            });
            unter
                .into_iter()
                .map(|eintrag| {
                    let mut zur_anzeige = anzeige.join(&eintrag.name);
                    let mut zum_lesen = lesepfad.join(&eintrag.name);
                    for teil in hinter {
                        zur_anzeige.push(teil);
                        zum_lesen.push(teil);
                    }
                    (zur_anzeige, zum_lesen)
                })
                .collect()
        }
    };

    let mut gruppen = Vec::new();
    for (zur_anzeige, zum_lesen) in ordner {
        if let Some(voll) = volle_grenzen(&haushalt, kuerzung) {
            kuerzung = voll;
            break;
        }
        let Some(zum_lesen) = innerhalb(wurzel, &zum_lesen) else {
            continue;
        };
        if !haushalt.leselauf_nehmen() {
            kuerzung = kuerzung.mit(Grenze::Ordner);
            break;
        }
        let Ok(stand) = leser::lesen_hoechstens(&zum_lesen, HOECHSTENS_EINTRAEGE_JE_BILDORDNER)
        else {
            continue;
        };
        if stand.abgeschnitten {
            kuerzung = kuerzung.mit(Grenze::Eintraege);
        }
        let fotos: Vec<Fotoeintrag> = stand
            .eintraege
            .into_iter()
            .filter(ist_foto)
            .map(|eintrag| Fotoeintrag {
                name: eintrag.name,
                schluessel: eintrag.sortierschluessel,
                geaendert: eintrag.geaendert,
            })
            .collect();
        if fotos.is_empty() {
            continue;
        }
        let Some(beitrag) = haushalt.gruppe_nehmen(fotos.len()) else {
            // `gruppe_nehmen` sagt nein aus denselben zwei Gruenden, die
            // `volle_grenzen` oben schon fragt; ein Nein ohne einen davon gibt
            // es nicht, und die Folge gilt dann an beiden als gekuerzt.
            kuerzung = volle_grenzen(&haushalt, kuerzung)
                .unwrap_or_else(|| kuerzung.mit(Grenze::Fotos).mit(Grenze::Ordner));
            break;
        };
        if beitrag < fotos.len() {
            kuerzung = kuerzung.mit(Grenze::Fotos);
        }
        gruppen.push(Gruppe {
            ordner: zur_anzeige,
            fotos,
            beitrag,
        });
    }

    Some(Bildverzeichnis {
        gesamt: haushalt.fotos(),
        gruppen,
        kuerzung,
        haushalt,
    })
}

/// Die Kuerzung um die Grenzen erweitert, die eine weitere Gruppe schon
/// ausschliessen; `None`, solange Platz ist.
///
/// Dieselben zwei Fragen wie in [`Bildhaushalt::gruppe_nehmen`], hier getrennt,
/// damit die Kuerzung die Grenze nennt, die gegriffen hat.
fn volle_grenzen(haushalt: &Bildhaushalt, mut kuerzung: Kuerzung) -> Option<Kuerzung> {
    let ordner = haushalt.gruppen() >= HOECHSTENS_BILDGRUPPEN;
    let fotos = haushalt.fotos() >= HOECHSTENS_FOTOS;
    if fotos {
        kuerzung = kuerzung.mit(Grenze::Fotos);
    }
    if ordner {
        kuerzung = kuerzung.mit(Grenze::Ordner);
    }
    (ordner || fotos).then_some(kuerzung)
}
