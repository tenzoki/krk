//! Ein Ort im Dateifenster: ein lokaler Ordner, einer auf einem Server oder
//! einer in einem Archiv.
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
//!
//! # Ein Ordner in einem Archiv
//!
//! [`Ort::Archiv`] ist der dritte Fall (E2 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`). Er traegt den
//! geteilten [`Archivzugang`] und den [`Innenpfad`] darin; gleich sind zwei
//! Archivorte ueber den Pfad der Archivdatei und den Innenpfad, nie ueber den
//! Zeiger auf den Zugang. Was ein Archivort ist und ob in ihn geschrieben
//! werden darf, sagt [`Ort::art`] am Wert und ohne Systemaufruf; die Datei
//! liest erst ein Arbeitsfaden mit dem Pfand `Fernfaden`.

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::archiv::{Archivformat, Archivzugang, Innenpfad};
pub use crate::fern::adresse::Serverkennung;
use crate::fern::pfad::Fernpfad;
use crate::fern::zugang::Fernzugang;
use crate::sprache::Text;

/// Ein Ordner, lokal, auf einem Server oder in einem Archiv.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ort {
    /// Ein lokaler Ordner.
    Lokal(PathBuf),
    /// Ein Ordner auf einem Server.
    Fern(FernerOrt),
    /// Ein Ordner in einer lokalen Archivdatei.
    Archiv(Archivort),
}

/// Welche Art Ort ein [`Ort`] ist, ohne seinen Inhalt.
///
/// Am Wert gerechnet und ohne Systemaufruf; die Zulaessigkeit fragt sie je
/// Tastendruck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ortsart {
    /// Ein lokaler Ordner.
    Lokal,
    /// Ein Ordner auf einem Server.
    Fern,
    /// Ein Ordner in einem Archiv; `beschreibbar` allein aus dem Format
    /// ([`Archivformat::beschreibbar`]).
    Archiv {
        /// Ob KRK in ein Archiv dieses Formats schreiben darf.
        beschreibbar: bool,
    },
}

impl Ort {
    /// Der lokale Pfad, oder `None` fuer einen fernen Ort oder einen im
    /// Archiv.
    #[must_use]
    pub fn lokal(&self) -> Option<&Path> {
        match self {
            Ort::Lokal(pfad) => Some(pfad),
            Ort::Fern(_) | Ort::Archiv(_) => None,
        }
    }

    /// Der ferne Ort, oder `None` fuer einen lokalen oder einen im Archiv.
    #[must_use]
    pub fn fern(&self) -> Option<&FernerOrt> {
        match self {
            Ort::Lokal(_) | Ort::Archiv(_) => None,
            Ort::Fern(ort) => Some(ort),
        }
    }

    /// Der Ort im Archiv, oder `None` fuer jeden anderen.
    #[must_use]
    pub fn archiv(&self) -> Option<&Archivort> {
        match self {
            Ort::Lokal(_) | Ort::Fern(_) => None,
            Ort::Archiv(ort) => Some(ort),
        }
    }

    /// Die Art des Orts, am Wert und ohne Systemaufruf.
    #[must_use]
    pub fn art(&self) -> Ortsart {
        match self {
            Ort::Lokal(_) => Ortsart::Lokal,
            Ort::Fern(_) => Ortsart::Fern,
            Ort::Archiv(ort) => Ortsart::Archiv {
                beschreibbar: ort.zugang.format().beschreibbar(),
            },
        }
    }
}

/// Der Satz, den ein Befehl meldet, der nur an einem lokalen Ordner wirkt,
/// oder `None` an einem lokalen Ordner, wo er wirkt.
///
/// **Eine Fallunterscheidung ersetzt den festen Text** an jedem Rufer, der
/// fuer einen nicht lokalen Ort `None` aus `lokaler_ordner()` bekommt: fern
/// wie bisher [`Text::AmFernenOrtNichtVerfuegbar`], im Archiv
/// [`Text::ImArchivNichtVerfuegbar`] (E2 des Plans).
#[must_use]
pub fn nicht_verfuegbar_text(ort: &Ort) -> Option<Text> {
    match ort {
        Ort::Lokal(_) => None,
        Ort::Fern(_) => Some(Text::AmFernenOrtNichtVerfuegbar),
        Ort::Archiv(_) => Some(Text::ImArchivNichtVerfuegbar),
    }
}

/// Ein Ordner in einem Archiv: der geteilte Zugang zur Archivdatei und der
/// Pfad darin.
#[derive(Clone)]
pub struct Archivort {
    /// Der Zugang zur Archivdatei, geteilt mit jedem Tab auf demselben Archiv.
    pub zugang: Arc<Archivzugang>,
    /// Der Pfad im Archiv; die Wurzel ist leer.
    pub innen: Innenpfad,
}

impl Archivort {
    /// Die Wurzel des Archivs hinter `zugang`.
    #[must_use]
    pub fn wurzel(zugang: Arc<Archivzugang>) -> Self {
        Archivort {
            zugang,
            innen: Innenpfad::wurzel(),
        }
    }

    /// Der lokale Pfad der Archivdatei.
    #[must_use]
    pub fn datei(&self) -> &Path {
        self.zugang.datei()
    }

    /// Das Format der Archivdatei.
    #[must_use]
    pub fn format(&self) -> Archivformat {
        self.zugang.format()
    }

    /// Der Ort `name` unter diesem, oder `None`, wenn `name` keine einzelne
    /// Komponente ist. Rein lexikalisch.
    #[must_use]
    pub fn kind(&self, name: &str) -> Option<Self> {
        self.innen.verbinden(name).map(|innen| Archivort {
            zugang: Arc::clone(&self.zugang),
            innen,
        })
    }

    /// Der uebergeordnete Ort im Archiv und der Name des verlassenen, oder
    /// `None` an der Wurzel. Rein lexikalisch.
    #[must_use]
    pub fn aufwaerts(&self) -> Option<(Self, String)> {
        let name = self.innen.name()?.to_owned();
        let innen = self.innen.aufwaerts()?;
        Some((
            Archivort {
                zugang: Arc::clone(&self.zugang),
                innen,
            },
            name,
        ))
    }
}

impl PartialEq for Archivort {
    fn eq(&self, anderer: &Self) -> bool {
        self.datei() == anderer.datei() && self.innen == anderer.innen
    }
}

impl Eq for Archivort {}

impl fmt::Debug for Archivort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Archivort")
            .field("datei", &self.datei())
            .field("innen", &self.innen)
            .finish()
    }
}

impl fmt::Display for Archivort {
    /// `/pfad/zum/archiv.zip:/innen/pfad`, die Wurzel als
    /// `/pfad/zum/archiv.zip:/`, in der Schreibweise des fernen Orts mit dem
    /// Doppelpunkt (E16 des Plans).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.datei().display(), self.innen)
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
