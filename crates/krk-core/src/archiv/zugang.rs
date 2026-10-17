//! Der Zugang zu einer Archivdatei: Pfad, Format und das zwischengespeicherte
//! Inhaltsverzeichnis.
//!
//! **Ein Zugang entsteht ohne Systemaufruf** ([`Archivzugang::neu`]); er
//! haelt den Pfad und das Format aus der Endung und hat die Datei noch nicht
//! geoeffnet. Das ist die Voraussetzung dafuer, dass der Hauptfaden beim
//! Einstieg in ein Archiv nichts tut, was warten kann (E5 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`).
//!
//! **Jeder Zugriff auf die Datei verlangt das Pfand [`Fernfaden`]**, das
//! allein in einem Arbeitsfaden entsteht. Damit haelt der Uebersetzer den
//! Hauptfaden von jedem Lesen eines Archivs fern, mit demselben Mechanismus
//! wie beim Server und ohne Quelltextprobe (E9).
//!
//! # Zwischenspeicher und Kennung
//!
//! Das Verzeichnis bleibt im Zugang, zusammen mit der [`Dateikennung`] der
//! Datei beim Lesen: Geraet, Inode, Groesse und Aenderungszeit in
//! Nanosekunden. Jeder Zugriff fragt die Kennung mit einem `stat` neu. Ist sie
//! gleich, gilt das gespeicherte Verzeichnis, und das Betreten eines
//! Unterordners in einem grossen tar.gz kostet keinen zweiten Durchlauf; weicht
//! sie ab, entsteht das Verzeichnis neu.
//!
//! **Was die Kennung nicht sieht:** eine Aenderung, die Groesse und
//! Aenderungszeit auf die Nanosekunde wiederherstellt, ohne den Inode zu
//! wechseln. Dafuer muesste ein Programm die Zeit ausdruecklich zuruecksetzen.
//!
//! Gebaut wird ohne gehaltene Sperre: zwei Faeden, die zugleich ein
//! veraltetes Verzeichnis finden, bauen es beide, und der spaetere ersetzt das
//! fruehere. Eine Sperre ueber den Aufbau liesse den zweiten Faden auf einen
//! Abbruch warten, den nur der erste sieht.

use std::fs::{self, Metadata};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::UNIX_EPOCH;

use crate::fern::Fernfaden;
use crate::verzeichnis::sys::ohne_warten_oeffnen;

use super::format::Archivformat;
use super::verzeichnis::{
    Inhaltsverzeichnis, Sammler, Verzeichnisfehler, abbruch_fragen, gz_lesen, tar_gz_lesen,
    tar_lesen,
};
use super::zipform;

/// Was eine Archivdatei ausmacht, solange niemand sie aendert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dateikennung {
    geraet: u64,
    inode: u64,
    groesse: u64,
    geaendert_ns: i128,
}

impl Dateikennung {
    /// Die Kennung aus den Angaben einer Datei, wie `stat(2)` oder `fstat(2)`
    /// sie liefern.
    #[must_use]
    pub fn aus_angaben(angaben: &Metadata) -> Self {
        Dateikennung {
            geraet: angaben.dev(),
            inode: angaben.ino(),
            groesse: angaben.size(),
            geaendert_ns: i128::from(angaben.mtime()) * 1_000_000_000
                + i128::from(angaben.mtime_nsec()),
        }
    }
}

/// Der geteilte Zugang zu einer Archivdatei.
///
/// Zwei Tabs auf demselben Archiv teilen einen Zugang und damit ein
/// Inhaltsverzeichnis; wer ihn teilt, regelt das Register der Oberflaeche.
#[derive(Debug)]
pub struct Archivzugang {
    datei: PathBuf,
    format: Archivformat,
    verzeichnis: Mutex<Option<Arc<Inhaltsverzeichnis>>>,
}

impl Archivzugang {
    /// Ein Zugang zu `datei` im genannten Format. Stellt keinen Systemaufruf.
    #[must_use]
    pub fn neu(datei: PathBuf, format: Archivformat) -> Self {
        Archivzugang {
            datei,
            format,
            verzeichnis: Mutex::new(None),
        }
    }

    /// Der lokale Pfad der Archivdatei.
    #[must_use]
    pub fn datei(&self) -> &Path {
        &self.datei
    }

    /// Das Format aus der Endung.
    #[must_use]
    pub fn format(&self) -> Archivformat {
        self.format
    }

    /// Das Inhaltsverzeichnis, aus dem Zwischenspeicher, wenn die Datei
    /// unveraendert ist, sonst neu gelesen.
    ///
    /// `abbruch` wird je Eintrag und je MiB entpackter Bytes gefragt; ein
    /// abgebrochener Aufbau laesst den Zwischenspeicher, wie er war.
    ///
    /// # Errors
    ///
    /// [`Verzeichnisfehler`]: abgebrochen, nicht lesbar, keine gewoehnliche
    /// Datei, kein Archiv des Formats oder zu viele Eintraege.
    pub fn verzeichnis(
        &self,
        _pfand: &Fernfaden,
        abbruch: &AtomicBool,
    ) -> Result<Arc<Inhaltsverzeichnis>, Verzeichnisfehler> {
        abbruch_fragen(abbruch)?;
        let vorher = fs::metadata(&self.datei).map_err(Verzeichnisfehler::Lesen)?;
        if let Some(gespeichert) = self.gespeichert(Dateikennung::aus_angaben(&vorher)) {
            return Ok(gespeichert);
        }

        // Geoeffnet wird ohne Warten, und der Typ wird am Deskriptor gefragt:
        // unter einem Namen auf `.zip` kann eine benannte Roehre stehen, und
        // ein gewoehnliches Oeffnen haengte dort fuer immer.
        let datei = ohne_warten_oeffnen(&self.datei).map_err(Verzeichnisfehler::Lesen)?;
        let angaben = datei.metadata().map_err(Verzeichnisfehler::Lesen)?;
        if !angaben.is_file() {
            return Err(Verzeichnisfehler::KeineDatei);
        }
        let kennung = Dateikennung::aus_angaben(&angaben);
        if let Some(gespeichert) = self.gespeichert(kennung) {
            return Ok(gespeichert);
        }

        let mut sammler = Sammler::default();
        match self.format {
            Archivformat::Zip => zipform::verzeichnis_lesen(datei, &mut sammler, abbruch)?,
            Archivformat::Tar => tar_lesen(datei, &mut sammler, abbruch)?,
            Archivformat::TarGz => tar_gz_lesen(datei, &mut sammler, abbruch)?,
            Archivformat::Gz => {
                let dateiname = self
                    .datei
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default();
                gz_lesen(datei, dateiname, &angaben, &mut sammler, abbruch)?;
            }
        }
        abbruch_fragen(abbruch)?;
        // Erschlossene Ordner tragen die Zeit der Archivdatei, gefragt am
        // Deskriptor und nicht ein zweites Mal am Pfad.
        let archivzeit = angaben.modified().unwrap_or(UNIX_EPOCH);
        let verzeichnis = Arc::new(sammler.abschliessen(kennung, archivzeit));
        *self.sperre() = Some(Arc::clone(&verzeichnis));
        Ok(verzeichnis)
    }

    /// Das zuletzt gelesene Verzeichnis, ohne die Kennung zu fragen, oder
    /// `None`, wenn noch keines vollstaendig gelesen ist.
    ///
    /// Stellt keinen Systemaufruf und darf deshalb auch vom Hauptfaden
    /// gefragt werden; ob die Datei sich seither geaendert hat, sagt erst
    /// [`Archivzugang::verzeichnis`].
    #[must_use]
    pub fn gespeichertes(&self) -> Option<Arc<Inhaltsverzeichnis>> {
        self.sperre().as_ref().map(Arc::clone)
    }

    /// Das gespeicherte Verzeichnis, sofern es zur Kennung passt.
    fn gespeichert(&self, kennung: Dateikennung) -> Option<Arc<Inhaltsverzeichnis>> {
        self.sperre()
            .as_ref()
            .filter(|verzeichnis| verzeichnis.kennung() == kennung)
            .map(Arc::clone)
    }

    /// Die Sperre um den Zwischenspeicher. Eine vergiftete Sperre heisst, dass
    /// ein Faden mitten im Ersetzen eines `Arc` in Panik geriet; der Wert
    /// darin ist trotzdem ein ganzes Verzeichnis oder keines.
    fn sperre(&self) -> std::sync::MutexGuard<'_, Option<Arc<Inhaltsverzeichnis>>> {
        self.verzeichnis
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}
