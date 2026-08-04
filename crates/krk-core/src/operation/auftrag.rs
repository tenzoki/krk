//! Was zu tun ist: Quelle, Ziel, Art und Konfliktregel.
//!
//! Ein [`Auftrag`] ist ein Wert ohne Verhalten. Er sagt, was geschehen soll,
//! und nichts darueber, wie oder wann. Ausgefuehrt wird er von
//! [`super::starten`] auf einem eigenen Arbeitsfaden.
//!
//! **Das Ziel steht in der Art und nicht daneben.** Kopieren und Verschieben
//! brauchen einen Zielordner, Papierkorb und endgueltiges Loeschen nicht. Ein
//! flaches Feld `ziel` haette bei zwei der vier Arten keinen Wert, den der
//! Aufrufer sinnvoll fuellen koennte, und jede Auswertung muesste sich darauf
//! verlassen, dass er ihn trotzdem richtig gefuellt hat.

use std::path::PathBuf;

use crate::verzeichnis::sys::Uebertragungsart;

/// Was mit den Quellen geschehen soll.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Art {
    /// In den genannten Ordner kopieren.
    Kopieren {
        /// Der Zielordner. Die Quellen behalten ihre Namen.
        ziel: PathBuf,
    },
    /// In den genannten Ordner verschieben.
    Verschieben {
        /// Der Zielordner. Die Quellen behalten ihre Namen.
        ziel: PathBuf,
    },
    /// In den Papierkorb des Systems raeumen (C4, Taste Delete).
    InDenPapierkorb,
    /// Endgueltig loeschen, ohne Umweg ueber den Papierkorb (C4, F8).
    ///
    /// Die Rueckfrage davor stellt die Oberflaeche, genau einmal je Vorgang;
    /// der Kern bekommt den Auftrag erst, wenn sie beantwortet ist. Festgelegt
    /// in `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`.
    EndgueltigLoeschen,
}

/// Was geschieht, wenn am Ziel schon ein Eintrag desselben Namens steht.
///
/// Ein Ordner, der auf einen Ordner desselben Namens trifft, ist **kein**
/// Konflikt: sein Inhalt wandert in den vorhandenen Ordner. Ein Konflikt ist
/// erst, wo ein Eintrag einen anderen ueberschreiben wuerde.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Konfliktregel {
    /// Nachfragen. Die Frage geht als [`super::Meldung::Konflikt`] an den
    /// Hauptfaden, der Arbeitsfaden wartet auf die Antwort.
    #[default]
    Fragen,
    /// Den vorhandenen Eintrag ersetzen.
    Ueberschreiben,
    /// Die Quelle auslassen und in der Abschlussliste nennen.
    Ueberspringen,
    /// Einen freien Namen daneben waehlen ("Name Kopie", "Name Kopie 2").
    AutomatischUmbenennen,
    /// Den ganzen Vorgang beenden.
    Abbrechen,
}

/// Ein Auftrag an die Operationsmaschine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Auftrag {
    /// Die Eintraege, auf die der Auftrag wirkt. Ordner mit Inhalt
    /// eingeschlossen.
    pub quellen: Vec<PathBuf>,
    /// Was mit ihnen geschehen soll.
    pub art: Art,
    /// Was bei einem Namenskonflikt gilt.
    pub konfliktregel: Konfliktregel,
    /// Wie eine einzelne Datei uebertragen wird. Die Oberflaeche laesst die
    /// Vorgabe stehen.
    pub uebertragung: Uebertragungsart,
}

impl Auftrag {
    /// Kopieren in den genannten Ordner.
    pub fn kopieren(quellen: Vec<PathBuf>, ziel: impl Into<PathBuf>) -> Self {
        Self::neu(quellen, Art::Kopieren { ziel: ziel.into() })
    }

    /// Verschieben in den genannten Ordner.
    pub fn verschieben(quellen: Vec<PathBuf>, ziel: impl Into<PathBuf>) -> Self {
        Self::neu(quellen, Art::Verschieben { ziel: ziel.into() })
    }

    /// In den Papierkorb des Systems raeumen.
    pub fn in_den_papierkorb(quellen: Vec<PathBuf>) -> Self {
        Self::neu(quellen, Art::InDenPapierkorb)
    }

    /// Endgueltig loeschen.
    pub fn endgueltig_loeschen(quellen: Vec<PathBuf>) -> Self {
        Self::neu(quellen, Art::EndgueltigLoeschen)
    }

    fn neu(quellen: Vec<PathBuf>, art: Art) -> Self {
        Self {
            quellen,
            art,
            konfliktregel: Konfliktregel::default(),
            uebertragung: Uebertragungsart::default(),
        }
    }

    /// Setzt die Konfliktregel.
    #[must_use]
    pub fn mit_konfliktregel(mut self, regel: Konfliktregel) -> Self {
        self.konfliktregel = regel;
        self
    }

    /// Setzt die Uebertragungsart.
    #[must_use]
    pub fn mit_uebertragung(mut self, art: Uebertragungsart) -> Self {
        self.uebertragung = art;
        self
    }

    /// Der Zielordner, sofern die Art einen hat.
    pub fn zielordner(&self) -> Option<&PathBuf> {
        match &self.art {
            Art::Kopieren { ziel } | Art::Verschieben { ziel } => Some(ziel),
            Art::InDenPapierkorb | Art::EndgueltigLoeschen => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn ein_loeschauftrag_hat_keinen_zielordner() {
        let auftrag = Auftrag::endgueltig_loeschen(vec![PathBuf::from("/tmp/a")]);
        assert_eq!(auftrag.zielordner(), None);
    }

    #[test]
    fn ein_kopierauftrag_nennt_seinen_zielordner() {
        let auftrag = Auftrag::kopieren(vec![PathBuf::from("/tmp/a")], "/tmp/b");
        assert_eq!(
            auftrag.zielordner().map(PathBuf::as_path),
            Some(Path::new("/tmp/b"))
        );
    }

    #[test]
    fn ohne_angabe_wird_bei_einem_konflikt_gefragt() {
        let auftrag = Auftrag::kopieren(Vec::new(), "/tmp");
        assert_eq!(auftrag.konfliktregel, Konfliktregel::Fragen);
    }
}
