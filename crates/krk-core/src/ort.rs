//! Ein Ort im Dateifenster: ein lokaler Ordner oder einer auf einem Server.
//!
//! **Ob ein Ort entfernt ist, entscheidet der Wert, nie ein Systemaufruf**
//! (Entscheidung 1 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`, Entscheid
//! `261016-1956-welche-form-hat-ein-ort-im-dateifenster-und-wie-haelt-der-bau-fernaufrufe-vom-hauptfaden-fern.md`,
//! Moeglichkeit 1). Jede Naht, an der ein Tab, ein Auftrag oder die
//! Auffrischung einen Ordner nennt, verzweigt ueber [`Ort`] vollstaendig und
//! ohne Auffangzweig, und der lokale Zweig ruft den Code, der ohne ferne Orte
//! laeuft.
//!
//! **Gleichheit geht ueber Serverkennung und Pfad, nie ueber den Zeiger** auf
//! den Zugang: zwei ferne Orte auf demselben Server unter demselben Pfad sind
//! derselbe Ort, auch wenn einer davon aus einem Zugang stammt, der inzwischen
//! neu verbunden ist.

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use crate::fern::adresse::Serverkennung;
use crate::fern::pfad::Fernpfad;
use crate::fern::zugang::Fernzugang;

/// Ein Ordner, lokal oder auf einem Server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ort {
    /// Ein lokaler Ordner.
    Lokal(PathBuf),
    /// Ein Ordner auf einem Server.
    Fern(FernerOrt),
}

impl Ort {
    /// Der lokale Pfad, oder `None` fuer einen fernen Ort.
    #[must_use]
    pub fn lokal(&self) -> Option<&Path> {
        match self {
            Ort::Lokal(pfad) => Some(pfad),
            Ort::Fern(_) => None,
        }
    }

    /// Der ferne Ort, oder `None` fuer einen lokalen.
    #[must_use]
    pub fn fern(&self) -> Option<&FernerOrt> {
        match self {
            Ort::Lokal(_) => None,
            Ort::Fern(ort) => Some(ort),
        }
    }
}

/// Ein Ordner auf einem Server: der Zugang und der Pfad dort.
#[derive(Clone)]
pub struct FernerOrt {
    /// Der Zugang zum Server, geteilt.
    pub zugang: Arc<Fernzugang>,
    /// Der Pfad auf dem Server.
    pub pfad: Fernpfad,
}

impl FernerOrt {
    /// Die Kennung des Servers.
    #[must_use]
    pub fn kennung(&self) -> &Serverkennung {
        self.zugang.kennung()
    }
}

impl PartialEq for FernerOrt {
    fn eq(&self, anderer: &Self) -> bool {
        self.kennung() == anderer.kennung() && self.pfad == anderer.pfad
    }
}

impl Eq for FernerOrt {}

impl fmt::Debug for FernerOrt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FernerOrt")
            .field("kennung", self.kennung())
            .field("pfad", &self.pfad)
            .finish()
    }
}

impl fmt::Display for FernerOrt {
    /// `konto@rechner:/pfad`, mit Port `konto@rechner:port:/pfad`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.kennung(), self.pfad)
    }
}
