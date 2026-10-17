//! Das Inhaltsverzeichnis eines Archivs: was darin steht, ohne es zu
//! entpacken, und die Leser fuer tar, tar.gz und gz.
//!
//! Jedes Format liefert seine Eintraege in der Reihenfolge, in der es sie
//! fuehrt, an einen [`Sammler`], und erst [`Sammler::abschliessen`] macht
//! daraus ein [`Inhaltsverzeichnis`]. **Die Regeln ueber alle Eintraege
//! zugleich stehen deshalb an einer Stelle und nicht je Format:**
//!
//! - Ein Name, den [`Innenpfad::aus_archivname`] abweist, kommt nicht hinein
//!   und zaehlt in [`Inhaltsverzeichnis::ausgelassen`].
//! - **Doppelte Namen:** der spaetere gilt, wie beim Entpacken auf die
//!   Platte, und der fruehere zaehlt. Ist ein Name zugleich Ordner und etwas
//!   anderes, gilt der Ordner, gleich in welcher Folge.
//! - **Der Weg ueber eine Verknuepfung:** ein Eintrag, dessen Weg durch einen
//!   Namen fuehrt, der im selben Archiv eine Verknuepfung ist
//!   (`hinaus -> /etc`, dann `hinaus/passwd`), faellt weg und zaehlt. Das ist
//!   der zweite Weg aus einem Zielordner heraus, den `operation::entpacken`
//!   mit `kette_anlegen` versperrt; im Fenster kommt er gar nicht erst vor.
//! - **Ordner ohne eigenen Eintrag** (`a/b/c.txt` ohne `a/` und `a/b/`)
//!   entstehen als erschlossene Ordner ([`Fundstelle::Erschlossen`]) mit dem
//!   Aenderungsdatum der Archivdatei. Eine Datei, deren Name zugleich ein
//!   solcher Ordner sein muesste, faellt weg und zaehlt.
//!
//! # Abbruch
//!
//! Gefragt wird je Eintrag und, fuer die gepackten Stroeme, je MiB entpackter
//! Bytes ([`Abbruchleser`]). Ein abgebrochener Aufbau liefert
//! [`Verzeichnisfehler::Abgebrochen`] und kein halbes Verzeichnis: der
//! [`Sammler`] wird dann verworfen, und nur ein abgeschlossenes Verzeichnis
//! erreicht den Zwischenspeicher des Zugangs.
//!
//! # Was ein Eintrag ueber seine Lesbarkeit sagt
//!
//! [`Lesbarkeit`] haelt fest, was beim Herauskopieren nicht herauskommt
//! (verschluesselt, ein Verfahren, das `deflate-flate2` nicht entpackt, ein
//! harter Verweis oder eine Sonderdatei im tar). Gezeigt wird ein solcher
//! Eintrag trotzdem: er steht im Archiv, und ihn zu verschweigen waere die
//! falsche Auskunft (E3 und E11 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`).

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use flate2::read::MultiGzDecoder;
use tar::EntryType;

use crate::verzeichnis::Typ;

use super::HOECHSTENS_EINTRAEGE;
use super::innenpfad::Innenpfad;
use super::zugang::Dateikennung;

/// Warum kein Inhaltsverzeichnis entstanden ist.
#[derive(Debug)]
pub enum Verzeichnisfehler {
    /// Die Abbruchmarke war gesetzt.
    Abgebrochen,
    /// Die Datei liess sich nicht oeffnen oder lesen; der Grund ist der des
    /// Systems.
    Lesen(io::Error),
    /// Unter dem Namen steht keine gewoehnliche Datei, etwa ein Ordner, eine
    /// Roehre oder ein Geraet.
    KeineDatei,
    /// Die Datei ist kein Archiv des Formats, das ihre Endung verspricht. Der
    /// Grund ist der Wortlaut der Kiste, aus dem Grund, aus dem
    /// `operation::entpacken` ihn weitergibt: er sagt genauer als eine eigene
    /// Formulierung, woran das Lesen gescheitert ist.
    Ungueltig(String),
    /// Das Archiv fuehrt mehr als [`HOECHSTENS_EINTRAEGE`] Eintraege.
    ZuGross,
}

impl Verzeichnisfehler {
    /// Ordnet einen Fehler beim Lesen eines Stroms zu.
    ///
    /// Ein Fehler mit einer Nummer des Systems ist ein Lesefehler an der Datei;
    /// jeder andere kommt aus dem Zerlegen des Formats und heisst, dass die
    /// Datei kein gueltiges Archiv ist. Eine gesetzte Abbruchmarke geht vor,
    /// denn der [`Abbruchleser`] meldet den Abbruch selbst als Fehler.
    pub(crate) fn aus_strom(fehler: io::Error, abbruch: &AtomicBool) -> Self {
        if abbruch.load(Ordering::Acquire) {
            Verzeichnisfehler::Abgebrochen
        } else if fehler.raw_os_error().is_some() {
            Verzeichnisfehler::Lesen(fehler)
        } else {
            Verzeichnisfehler::Ungueltig(fehler.to_string())
        }
    }
}

/// Wo die Daten eines Eintrags im Archiv stehen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fundstelle {
    /// Der Platz im Zentralverzeichnis eines Zip, wie `ZipArchive::by_index`
    /// ihn nimmt.
    Zip(usize),
    /// Der Versatz des Kopfs im entpackten tar-Strom. Die Eintraege eines
    /// Stroms stehen in dieser Folge, und ein Lesezeiger, der nur vorwaerts
    /// laeuft, erreicht sie in ihr ohne Neustart.
    Tar {
        /// Versatz in Bytes vom Anfang des (entpackten) Stroms.
        versatz: u64,
    },
    /// Der ganze entpackte Strom einer gz-Datei.
    GzGanz,
    /// Ein Ordner ohne eigenen Eintrag, erschlossen aus den Namen darunter.
    /// Er hat keine Daten und keinen Kopf; der Umbau braucht das Kennzeichen,
    /// um ihn stehen zu lassen, wenn sein letzter Eintrag wegfaellt.
    Erschlossen,
}

/// Ob ein Eintrag beim Herauskopieren herauskommt, und wenn nicht, warum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lesbarkeit {
    /// Der Eintrag laesst sich lesen.
    Lesbar,
    /// Der Eintrag ist verschluesselt.
    Verschluesselt,
    /// Der Eintrag ist mit einem Verfahren gepackt, das KRK nicht entpackt;
    /// der Text ist der Name des Verfahrens.
    Verfahren(String),
    /// Ein harter Verweis im tar.
    HarterVerweis,
    /// Eine Geraetedatei, eine benannte Roehre oder ein unbekannter Typ.
    Sonderdatei,
}

/// Ein Eintrag des Inhaltsverzeichnisses.
#[derive(Debug, Clone)]
pub struct Archiveintrag {
    /// Der Pfad im Archiv.
    pub innen: Innenpfad,
    /// Ordner, Datei oder Verknuepfung.
    pub typ: Typ,
    /// Die entpackte Groesse in Bytes; bei einem Ordner null.
    pub groesse: u64,
    /// Das Aenderungsdatum. Ein Zip-Eintrag ohne Zeitfeld traegt die
    /// Unix-Epoche, wie ein ferner Eintrag ohne Angabe; das MS-DOS-Feld wird
    /// aus dem Grund im Kopf von `operation::entpacken` nicht gelesen.
    pub geaendert: SystemTime,
    /// Die Rechte (`& 0o7777`), sofern das Archiv sie fuehrt.
    pub rechte: Option<u32>,
    /// Wo die Daten stehen.
    pub fundstelle: Fundstelle,
    /// Ob der Eintrag herauskommt.
    pub lesbarkeit: Lesbarkeit,
}

impl Archiveintrag {
    /// Ob der Eintrag ein Ordner ohne eigenen Eintrag im Archiv ist.
    #[must_use]
    pub fn ist_erschlossen(&self) -> bool {
        self.fundstelle == Fundstelle::Erschlossen
    }
}

/// Was in einem Archiv steht, mit der Kennung der Datei beim Lesen.
#[derive(Debug)]
pub struct Inhaltsverzeichnis {
    kennung: Dateikennung,
    eintraege: Vec<Archiveintrag>,
    stellen: HashMap<Innenpfad, usize>,
    kinder: HashMap<Innenpfad, Vec<usize>>,
    ausgelassen: u64,
    verschluesselt: bool,
}

impl Inhaltsverzeichnis {
    /// Die Kennung der Archivdatei, aus der dieses Verzeichnis gelesen ist.
    #[must_use]
    pub fn kennung(&self) -> Dateikennung {
        self.kennung
    }

    /// Alle Eintraege, erschlossene Ordner eingeschlossen, ohne die Wurzel.
    #[must_use]
    pub fn eintraege(&self) -> &[Archiveintrag] {
        &self.eintraege
    }

    /// Der Eintrag unter `innen`, oder `None`. Die Wurzel ist kein Eintrag.
    #[must_use]
    pub fn eintrag(&self, innen: &Innenpfad) -> Option<&Archiveintrag> {
        self.stellen
            .get(innen)
            .map(|&stelle| &self.eintraege[stelle])
    }

    /// Die Eintraege unmittelbar unter `ordner`, oder `None`, wenn `ordner`
    /// weder die Wurzel noch ein Ordner des Archivs ist.
    #[must_use]
    pub fn kinder(&self, ordner: &Innenpfad) -> Option<impl Iterator<Item = &Archiveintrag>> {
        self.kinder
            .get(ordner)
            .map(|stellen| stellen.iter().map(|&stelle| &self.eintraege[stelle]))
    }

    /// Wie viele Eintraege des Archivs nicht im Verzeichnis stehen: unsichere
    /// Namen, Namen, die kein UTF-8 sind, fruehere von doppelten Namen und
    /// Eintraege, die einem Ordner weichen mussten.
    #[must_use]
    pub fn ausgelassen(&self) -> u64 {
        self.ausgelassen
    }

    /// Ob ein Eintrag des Archivs verschluesselt ist. Ein solches Archiv
    /// schreibt KRK nicht (E13 des Plans).
    #[must_use]
    pub fn verschluesselt(&self) -> bool {
        self.verschluesselt
    }
}

/// Sammelt die Eintraege eines Formats in Lesefolge und schliesst sie zu
/// einem [`Inhaltsverzeichnis`] ab.
#[derive(Debug, Default)]
pub(crate) struct Sammler {
    zeilen: Vec<Archiveintrag>,
    ausgelassen: u64,
}

impl Sammler {
    /// Nimmt einen Eintrag auf. Ein Eintrag an der Wurzel (`./`) nennt den
    /// Ordner, in dem das Fenster ohnehin steht, und faellt ohne Zaehlung weg.
    pub(crate) fn aufnehmen(&mut self, eintrag: Archiveintrag) -> Result<(), Verzeichnisfehler> {
        if eintrag.innen.ist_wurzel() {
            return Ok(());
        }
        if self.zeilen.len() >= HOECHSTENS_EINTRAEGE {
            return Err(Verzeichnisfehler::ZuGross);
        }
        self.zeilen.push(eintrag);
        Ok(())
    }

    /// Zaehlt einen Eintrag, der nicht ins Verzeichnis kommt.
    pub(crate) fn auslassen(&mut self) {
        self.ausgelassen += 1;
    }

    /// Wendet die Regeln aus dem Kopf dieses Moduls an und baut das
    /// Verzeichnis.
    pub(crate) fn abschliessen(
        self,
        kennung: Dateikennung,
        archivzeit: SystemTime,
    ) -> Inhaltsverzeichnis {
        let Sammler {
            zeilen,
            mut ausgelassen,
        } = self;

        // Doppelte Namen: der spaetere gilt, ein Ordner geht vor.
        let mut plaetze: Vec<Option<Archiveintrag>> = Vec::with_capacity(zeilen.len());
        let mut stelle_von: HashMap<Innenpfad, usize> = HashMap::with_capacity(zeilen.len());
        for zeile in zeilen {
            match stelle_von.get(&zeile.innen) {
                Some(&stelle) => {
                    ausgelassen += 1;
                    let bisher_ordner = plaetze[stelle]
                        .as_ref()
                        .is_some_and(|bisher| matches!(bisher.typ, Typ::Ordner));
                    let neu_ordner = matches!(zeile.typ, Typ::Ordner);
                    if !bisher_ordner || neu_ordner {
                        plaetze[stelle] = Some(zeile);
                    }
                }
                None => {
                    stelle_von.insert(zeile.innen.clone(), plaetze.len());
                    plaetze.push(Some(zeile));
                }
            }
        }
        drop(stelle_von);

        // Der Weg ueber eine Verknuepfung.
        let verknuepfungen: HashSet<Innenpfad> = plaetze
            .iter()
            .flatten()
            .filter(|eintrag| matches!(eintrag.typ, Typ::Verknuepfung))
            .map(|eintrag| eintrag.innen.clone())
            .collect();
        if !verknuepfungen.is_empty() {
            for platz in &mut plaetze {
                let durch_verknuepfung = platz.as_ref().is_some_and(|eintrag| {
                    vorfahren(&eintrag.innen).any(|vorfahr| verknuepfungen.contains(&vorfahr))
                });
                if durch_verknuepfung {
                    *platz = None;
                    ausgelassen += 1;
                }
            }
        }

        // Jeder Vorfahr eines Eintrags muss ein Ordner sein.
        let mut noetige_ordner: HashSet<Innenpfad> = HashSet::new();
        for eintrag in plaetze.iter().flatten() {
            for vorfahr in vorfahren(&eintrag.innen) {
                if !noetige_ordner.insert(vorfahr) {
                    // Wer diesen Vorfahren schon hat, hat auch dessen Vorfahren.
                    break;
                }
            }
        }
        for platz in &mut plaetze {
            let weicht = platz.as_ref().is_some_and(|eintrag| {
                !matches!(eintrag.typ, Typ::Ordner) && noetige_ordner.contains(&eintrag.innen)
            });
            if weicht {
                *platz = None;
                ausgelassen += 1;
            }
        }

        let mut eintraege: Vec<Archiveintrag> = plaetze.into_iter().flatten().collect();
        let mut stellen: HashMap<Innenpfad, usize> = eintraege
            .iter()
            .enumerate()
            .map(|(stelle, eintrag)| (eintrag.innen.clone(), stelle))
            .collect();

        // Erschlossene Ordner, in einer festen Folge, damit zwei Laeufe ueber
        // dasselbe Archiv dasselbe Verzeichnis ergeben.
        let mut fehlende: Vec<Innenpfad> = noetige_ordner
            .into_iter()
            .filter(|ordner| !stellen.contains_key(ordner))
            .collect();
        fehlende.sort();
        for ordner in fehlende {
            stellen.insert(ordner.clone(), eintraege.len());
            eintraege.push(Archiveintrag {
                innen: ordner,
                typ: Typ::Ordner,
                groesse: 0,
                geaendert: archivzeit,
                rechte: None,
                fundstelle: Fundstelle::Erschlossen,
                lesbarkeit: Lesbarkeit::Lesbar,
            });
        }

        let mut kinder: HashMap<Innenpfad, Vec<usize>> = HashMap::new();
        kinder.insert(Innenpfad::wurzel(), Vec::new());
        for eintrag in &eintraege {
            if matches!(eintrag.typ, Typ::Ordner) {
                kinder.entry(eintrag.innen.clone()).or_default();
            }
        }
        for (stelle, eintrag) in eintraege.iter().enumerate() {
            let eltern = eintrag.innen.aufwaerts().unwrap_or_default();
            kinder.entry(eltern).or_default().push(stelle);
        }

        let verschluesselt = eintraege
            .iter()
            .any(|eintrag| eintrag.lesbarkeit == Lesbarkeit::Verschluesselt);

        Inhaltsverzeichnis {
            kennung,
            eintraege,
            stellen,
            kinder,
            ausgelassen,
            verschluesselt,
        }
    }
}

/// Die echten Vorfahren eines Pfads ohne die Wurzel, vom naechsten an.
fn vorfahren(innen: &Innenpfad) -> impl Iterator<Item = Innenpfad> {
    std::iter::successors(innen.aufwaerts(), Innenpfad::aufwaerts)
        .take_while(|vorfahr| !vorfahr.ist_wurzel())
}

/// Wie viele entpackte Bytes zwischen zwei Abbruchfragen liegen.
const ABBRUCHTAKT: u64 = 1 << 20;

/// Ein Leser, der je [`ABBRUCHTAKT`] gelesener Bytes die Abbruchmarke fragt
/// und bei gesetzter Marke mit einem Fehler endet.
///
/// Der Fehler traegt keine Nummer des Systems; wer ihn bekommt, fragt die
/// Marke ueber [`Verzeichnisfehler::aus_strom`] und meldet den Abbruch statt
/// eines ungueltigen Archivs.
pub(crate) struct Abbruchleser<'a, R> {
    innen: R,
    abbruch: &'a AtomicBool,
    seit_frage: u64,
}

impl<'a, R> Abbruchleser<'a, R> {
    pub(crate) fn neu(innen: R, abbruch: &'a AtomicBool) -> Self {
        Abbruchleser {
            innen,
            abbruch,
            seit_frage: 0,
        }
    }
}

impl<R: Read> Read for Abbruchleser<'_, R> {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        let gelesen = self.innen.read(puffer)?;
        self.seit_frage += gelesen as u64;
        if self.seit_frage >= ABBRUCHTAKT {
            self.seit_frage = 0;
            if self.abbruch.load(Ordering::Acquire) {
                return Err(io::Error::other("abgebrochen"));
            }
        }
        Ok(gelesen)
    }
}

impl<R: Seek> Seek for Abbruchleser<'_, R> {
    fn seek(&mut self, stelle: SeekFrom) -> io::Result<u64> {
        self.innen.seek(stelle)
    }
}

/// Fragt die Abbruchmarke zwischen zwei Eintraegen.
pub(crate) fn abbruch_fragen(abbruch: &AtomicBool) -> Result<(), Verzeichnisfehler> {
    if abbruch.load(Ordering::Acquire) {
        Err(Verzeichnisfehler::Abgebrochen)
    } else {
        Ok(())
    }
}

/// Prueft die zwei Bytes, mit denen jeder gzip-Strom beginnt, und stellt die
/// Datei danach an den Anfang zurueck.
///
/// `MultiGzDecoder` liefert fuer eine leere Datei einen leeren Strom und
/// keinen Fehler; ohne diese Frage hiesse eine leere `.tar.gz` ein leeres
/// Archiv statt keines.
fn gzip_kopf_pruefen(datei: &mut File) -> Result<(), Verzeichnisfehler> {
    let mut kopf = [0u8; 2];
    let gelesen = datei.read(&mut kopf).map_err(Verzeichnisfehler::Lesen)?;
    if gelesen < 2 || kopf != [0x1f, 0x8b] {
        return Err(Verzeichnisfehler::Ungueltig(
            "invalid gzip header".to_owned(),
        ));
    }
    datei
        .seek(SeekFrom::Start(0))
        .map_err(Verzeichnisfehler::Lesen)?;
    Ok(())
}

/// Liest das Verzeichnis eines tar-Archivs ohne Packung.
///
/// Die Daten werden ueberlesen und nicht gelesen: `entries_with_seek` springt
/// ueber sie, also kostet ein tar mit einer grossen Datei einen Sprung und
/// kein Lesen.
pub(crate) fn tar_lesen(
    datei: File,
    sammler: &mut Sammler,
    abbruch: &AtomicBool,
) -> Result<(), Verzeichnisfehler> {
    let mut archiv = tar::Archive::new(Abbruchleser::neu(BufReader::new(datei), abbruch));
    let eintraege = archiv
        .entries_with_seek()
        .map_err(|fehler| Verzeichnisfehler::aus_strom(fehler, abbruch))?;
    tar_eintraege_sammeln(eintraege, sammler, abbruch)
}

/// Liest das Verzeichnis eines gepackten tar-Stroms.
///
/// Der ganze Strom wird einmal entpackt und verworfen; anders ist das Ende
/// eines Eintrags in einem gzip-Strom nicht zu finden.
pub(crate) fn tar_gz_lesen(
    mut datei: File,
    sammler: &mut Sammler,
    abbruch: &AtomicBool,
) -> Result<(), Verzeichnisfehler> {
    gzip_kopf_pruefen(&mut datei)?;
    let strom = MultiGzDecoder::new(BufReader::new(datei));
    let mut archiv = tar::Archive::new(Abbruchleser::neu(strom, abbruch));
    let eintraege = archiv
        .entries()
        .map_err(|fehler| Verzeichnisfehler::aus_strom(fehler, abbruch))?;
    tar_eintraege_sammeln(eintraege, sammler, abbruch)
}

/// Der gemeinsame Teil von [`tar_lesen`] und [`tar_gz_lesen`].
fn tar_eintraege_sammeln<R: Read>(
    eintraege: tar::Entries<'_, R>,
    sammler: &mut Sammler,
    abbruch: &AtomicBool,
) -> Result<(), Verzeichnisfehler> {
    for eintrag in eintraege {
        abbruch_fragen(abbruch)?;
        let eintrag = eintrag.map_err(|fehler| Verzeichnisfehler::aus_strom(fehler, abbruch))?;
        let kopf = eintrag.header();
        let (typ, lesbarkeit) = match kopf.entry_type() {
            EntryType::Directory => (Typ::Ordner, Lesbarkeit::Lesbar),
            EntryType::Symlink => (Typ::Verknuepfung, Lesbarkeit::Lesbar),
            EntryType::Regular | EntryType::Continuous | EntryType::GNUSparse => {
                (Typ::Datei, Lesbarkeit::Lesbar)
            }
            EntryType::Link => (Typ::Datei, Lesbarkeit::HarterVerweis),
            EntryType::Char
            | EntryType::Block
            | EntryType::Fifo
            | EntryType::__Nonexhaustive(_) => (Typ::Datei, Lesbarkeit::Sonderdatei),
            // Koepfe, die einen anderen Eintrag beschreiben, und keine
            // Eintraege: lange Namen und die Erweiterungen nach PAX liest die
            // Kiste selbst in den naechsten Eintrag hinein, ein globaler
            // Kopf gilt fuer das ganze Archiv.
            EntryType::GNULongName
            | EntryType::GNULongLink
            | EntryType::XHeader
            | EntryType::XGlobalHeader => continue,
        };
        let name = eintrag.path_bytes();
        let Some(innen) = std::str::from_utf8(&name)
            .ok()
            .and_then(Innenpfad::aus_archivname)
        else {
            sammler.auslassen();
            continue;
        };
        let geaendert = kopf.mtime().map_or(UNIX_EPOCH, |sekunden| {
            UNIX_EPOCH + Duration::from_secs(sekunden)
        });
        let groesse = match typ {
            Typ::Ordner => 0,
            Typ::Datei | Typ::Verknuepfung => eintrag.size(),
        };
        sammler.aufnehmen(Archiveintrag {
            innen,
            typ,
            groesse,
            geaendert,
            rechte: kopf.mode().ok().map(|rechte| rechte & 0o7777),
            fundstelle: Fundstelle::Tar {
                versatz: eintrag.raw_header_position(),
            },
            lesbarkeit,
        })?;
    }
    Ok(())
}

/// Liest eine gzip-Datei, die kein tar traegt: genau ein Eintrag.
///
/// Der Name ist der Dateiname ohne die letzte Endung `.gz`
/// (`bericht.txt.gz` zeigt `bericht.txt`), wie `gunzip` ohne `-N` ihn
/// erzeugt; das Feld `FNAME` im Kopf liest KRK nicht. Hiesse der Rest leer
/// oder waere er kein einzelner Name, heisst der Eintrag `inhalt`.
///
/// **Die Groesse wird gezaehlt und nicht gelesen**: das Feld `ISIZE` am Ende
/// zaehlt nur das letzte Glied und nur modulo 2^32. Gezaehlt wird mit
/// `MultiGzDecoder`, also ueber alle Glieder.
pub(crate) fn gz_lesen(
    mut datei: File,
    dateiname: &str,
    angaben: &std::fs::Metadata,
    sammler: &mut Sammler,
    abbruch: &AtomicBool,
) -> Result<(), Verzeichnisfehler> {
    use std::os::unix::fs::PermissionsExt;

    gzip_kopf_pruefen(&mut datei)?;
    let mut strom = Abbruchleser::neu(MultiGzDecoder::new(BufReader::new(datei)), abbruch);
    let mut puffer = vec![0u8; 64 * 1024];
    let mut groesse: u64 = 0;
    loop {
        match strom.read(&mut puffer) {
            Ok(0) => break,
            Ok(gelesen) => groesse += gelesen as u64,
            Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => {}
            Err(fehler) => return Err(Verzeichnisfehler::aus_strom(fehler, abbruch)),
        }
    }
    abbruch_fragen(abbruch)?;

    let stamm = dateiname
        .len()
        .checked_sub(3)
        .filter(|&schnitt| dateiname.is_char_boundary(schnitt))
        .map_or(dateiname, |schnitt| &dateiname[..schnitt]);
    let innen = Innenpfad::wurzel()
        .verbinden(stamm)
        .or_else(|| Innenpfad::wurzel().verbinden("inhalt"))
        .unwrap_or_default();
    sammler.aufnehmen(Archiveintrag {
        innen,
        typ: Typ::Datei,
        groesse,
        geaendert: angaben.modified().unwrap_or(UNIX_EPOCH),
        rechte: Some(angaben.permissions().mode() & 0o7777),
        fundstelle: Fundstelle::GzGanz,
        lesbarkeit: Lesbarkeit::Lesbar,
    })
}
