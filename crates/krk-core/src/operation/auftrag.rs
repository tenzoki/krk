//! Was zu tun ist: Quelle, Ziel, Art und Konfliktregel.
//!
//! Ein [`Auftrag`] ist ein Wert ohne Verhalten. Er sagt, was geschehen soll,
//! und nichts darueber, wie oder wann. Ausgefuehrt wird er von
//! [`super::starten`] auf einem eigenen Arbeitsfaden.
//!
//! **Das Ziel steht in der Art und nicht daneben.** Kopieren und Verschieben
//! brauchen einen Zielordner, Papierkorb und Stapelumbenennen nicht. Ein
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
    /// Im Stapel umbenennen, jeder Eintrag in seinem eigenen Ordner (C4).
    ///
    /// Die Regel, aus der die neuen Namen entstehen, rechnet die Oberflaeche;
    /// der Kern bekommt die fertige Liste. Er prueft sie nicht ein zweites Mal
    /// auf Kollisionen: was der Nutzer in der Vorschau gesehen hat, ist der
    /// Auftrag, und ein Name, den das Dateisystem inzwischen vergeben hat,
    /// scheitert an ebendiesem und landet in der Abschlussliste.
    UmbenennenImStapel {
        /// Die neuen Namen, Stelle fuer Stelle zu [`Auftrag::quellen`].
        ///
        /// Zwei Listen und keine Liste aus Paaren, weil die Maschine ueber
        /// `quellen` laeuft wie bei jeder anderen Art. Aneinander gebunden
        /// werden sie von [`Auftrag::umbenennen_im_stapel`], das die Paare
        /// auftrennt; ein Aufrufer kann sie deshalb nicht gegeneinander
        /// verschieben.
        neue_namen: Vec<String>,
    },
    /// Die Quellen in **ein** Archiv packen.
    ///
    /// Die einzige Art, die nicht Quelle fuer Quelle abgearbeitet wird: ihr
    /// Ziel gehoert dem ganzen Lauf und nicht der einzelnen Quelle. Wo die
    /// Verzweigung sitzt und warum, steht bei [`super::zippen`].
    Zippen {
        /// Der volle Pfad des Archivs, **nicht** sein Ordner. Ein Lauf erzeugt
        /// genau eine Zieldatei, und sie steht damit hier vollstaendig da.
        ///
        /// Wie das Archiv heisst, rechnet die Oberflaeche; der Kern bekommt den
        /// fertigen Pfad. Ein Name, den das Dateisystem inzwischen vergeben hat,
        /// loest die Konfliktfrage aus, bevor ein Byte geschrieben wird.
        ziel: PathBuf,
    },
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

    /// Im Stapel umbenennen (C4).
    ///
    /// Genommen werden Paare aus altem Pfad und neuem Namen, damit die beiden
    /// Listen gar nicht erst getrennt uebergeben werden koennen. Aufgetrennt
    /// werden sie hier, einmal, und danach laufen sie Stelle fuer Stelle
    /// nebeneinander.
    pub fn umbenennen_im_stapel(paare: Vec<(PathBuf, String)>) -> Self {
        let (quellen, neue_namen): (Vec<PathBuf>, Vec<String>) = paare.into_iter().unzip();
        Self::neu(quellen, Art::UmbenennenImStapel { neue_namen })
    }

    /// Die genannten Eintraege in **ein** Archiv packen.
    ///
    /// `ziel` ist der volle Pfad des Archivs und kein Ordner; die Namensbildung
    /// gehoert der Oberflaeche.
    pub fn zippen(quellen: Vec<PathBuf>, ziel: impl Into<PathBuf>) -> Self {
        Self::neu(quellen, Art::Zippen { ziel: ziel.into() })
    }

    /// Der neue Name der Quelle an dieser Stelle, sofern die Art einen kennt.
    pub(crate) fn neuer_name(&self, stelle: usize) -> Option<&str> {
        match &self.art {
            Art::UmbenennenImStapel { neue_namen } => neue_namen.get(stelle).map(String::as_str),
            _ => None,
        }
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
    ///
    /// **Drei Arten haben keinen, und `None` ist bei keiner ein vergessener
    /// Fall.** Beim Papierkorb liegt das Ziel ausserhalb des Auftrags. Beim
    /// Stapel-Umbenennen bleibt jeder Eintrag, wo er ist. Beim Packen ist das
    /// Ziel eine **Datei** und keine Ablage fuer weitere Eintraege; wer es hier
    /// zurueckgaebe, gaebe einen Ordnerpfad heraus, der keiner ist.
    pub fn zielordner(&self) -> Option<&PathBuf> {
        match &self.art {
            Art::Kopieren { ziel } | Art::Verschieben { ziel } => Some(ziel),
            Art::InDenPapierkorb | Art::UmbenennenImStapel { .. } | Art::Zippen { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn ein_loeschauftrag_hat_keinen_zielordner() {
        let auftrag = Auftrag::in_den_papierkorb(vec![PathBuf::from("/tmp/a")]);
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
    fn ein_stapel_umbenennen_traegt_die_namen_stelle_fuer_stelle_zu_den_quellen() {
        let auftrag = Auftrag::umbenennen_im_stapel(vec![
            (PathBuf::from("/tmp/a.txt"), "eins.txt".to_owned()),
            (PathBuf::from("/tmp/b.txt"), "zwei.txt".to_owned()),
        ]);

        assert_eq!(
            auftrag.quellen,
            vec![PathBuf::from("/tmp/a.txt"), PathBuf::from("/tmp/b.txt")]
        );
        assert_eq!(auftrag.neuer_name(0), Some("eins.txt"));
        assert_eq!(auftrag.neuer_name(1), Some("zwei.txt"));
        assert_eq!(auftrag.neuer_name(2), None, "jenseits der Liste");
        assert_eq!(
            auftrag.zielordner(),
            None,
            "jeder Eintrag bleibt, wo er ist"
        );
    }

    #[test]
    fn eine_andere_art_kennt_keinen_neuen_namen() {
        let auftrag = Auftrag::kopieren(vec![PathBuf::from("/tmp/a")], "/tmp/b");
        assert_eq!(auftrag.neuer_name(0), None);
    }

    #[test]
    fn ein_packauftrag_hat_keinen_zielordner_sondern_eine_zieldatei() {
        let auftrag = Auftrag::zippen(vec![PathBuf::from("/tmp/a")], "/tmp/a.zip");
        assert_eq!(auftrag.zielordner(), None);
        assert_eq!(
            auftrag.art,
            Art::Zippen {
                ziel: PathBuf::from("/tmp/a.zip")
            }
        );
    }

    #[test]
    fn ohne_angabe_wird_bei_einem_konflikt_gefragt() {
        let auftrag = Auftrag::kopieren(Vec::new(), "/tmp");
        assert_eq!(auftrag.konfliktregel, Konfliktregel::Fragen);
    }
}
