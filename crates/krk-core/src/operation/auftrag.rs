//! Was zu tun ist: Quelle, Ziel, Art und Konfliktregel.
//!
//! Ein [`Auftrag`] ist ein Wert ohne Verhalten. Er sagt, was geschehen soll,
//! und nichts darueber, wie oder wann. Ausgefuehrt wird er von
//! [`super::starten`] auf einem eigenen Arbeitsfaden.
//!
//! **Das Ziel steht in der Art und nicht daneben.** Kopieren und Verschieben
//! brauchen einen Zielordner, Papierkorb und Stapelumbenennen nicht, das Packen
//! eine Zieldatei und das Entpacken eine ganze Liste davon. Ein flaches Feld
//! `ziel` haette bei mehreren Arten keinen Wert, den der Aufrufer sinnvoll
//! fuellen koennte, und jede Auswertung muesste sich darauf verlassen, dass er
//! ihn trotzdem richtig gefuellt hat.

use std::path::{Path, PathBuf};

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
    /// Jede Quelle ist ein Archiv und wird in **ihren eigenen** neuen Ordner
    /// entpackt.
    ///
    /// **Das Spiegelbild des Packens und nicht seine Umkehrung.** Das Packen
    /// zieht viele Quellen in ein Ziel und laeuft deshalb neben der
    /// Quelle-fuer-Quelle-Schleife; das Entpacken gibt jeder Quelle ihr eigenes
    /// Ziel und laeuft deshalb **in** ihr, wie das Kopieren.
    Entpacken {
        /// Die Zielordner, Stelle fuer Stelle zu [`Auftrag::quellen`].
        ///
        /// **Es ist eine Liste und kein einzelner Pfad**, weil ein Vorgang
        /// mehrere Archive tragen kann: der Nutzer hat am 260824-2120 gewaehlt,
        /// dass Unzip auf die betroffenen Eintraege wirkt und **jedes** Archiv
        /// darin entpackt (`decisions/260825-0727_*_nimmt-unzip-die-betroffenen-
        /// eintraege-oder-allein-die-ausgewaehlte-zeile.md`, Moeglichkeit 3).
        /// Drei markierte Archive ergeben damit drei Zielordner in einem
        /// Vorgang, und der Zielordner-Konflikt wird je Archiv gefragt.
        ///
        /// Zwei Listen und keine Liste aus Paaren, aus demselben Grund wie bei
        /// [`Art::UmbenennenImStapel`]: die Maschine laeuft ueber `quellen` wie
        /// bei jeder anderen Art. Aneinander gebunden werden sie von
        /// [`Auftrag::entpacken`], das die Paare auftrennt.
        ///
        /// Wie ein Zielordner heisst, rechnet die Oberflaeche; der Kern bekommt
        /// die fertigen Pfade.
        ziele: Vec<PathBuf>,
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

    /// Die genannten Archive in je einen eigenen Ordner entpacken.
    ///
    /// Genommen werden Paare aus Archivpfad und Zielordner, damit die beiden
    /// Listen gar nicht erst getrennt uebergeben werden koennen. Aufgetrennt
    /// werden sie hier, einmal, und danach laufen sie Stelle fuer Stelle
    /// nebeneinander.
    ///
    /// Die Zielordner rechnet die Oberflaeche; der Kern legt sie an.
    pub fn entpacken(paare: Vec<(PathBuf, PathBuf)>) -> Self {
        let (quellen, ziele): (Vec<PathBuf>, Vec<PathBuf>) = paare.into_iter().unzip();
        Self::neu(quellen, Art::Entpacken { ziele })
    }

    /// Der neue Name der Quelle an dieser Stelle, sofern die Art einen kennt.
    pub(crate) fn neuer_name(&self, stelle: usize) -> Option<&str> {
        match &self.art {
            Art::UmbenennenImStapel { neue_namen } => neue_namen.get(stelle).map(String::as_str),
            _ => None,
        }
    }

    /// Der Zielordner des Archivs an dieser Stelle, sofern die Art einen kennt.
    pub(crate) fn entpackziel(&self, stelle: usize) -> Option<&Path> {
        match &self.art {
            Art::Entpacken { ziele } => ziele.get(stelle).map(PathBuf::as_path),
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
    /// **Vier Arten haben keinen, und `None` ist bei keiner ein vergessener
    /// Fall.** Beim Papierkorb liegt das Ziel ausserhalb des Auftrags. Beim
    /// Stapel-Umbenennen bleibt jeder Eintrag, wo er ist. Beim Packen ist das
    /// Ziel eine **Datei** und keine Ablage fuer weitere Eintraege; wer es hier
    /// zurueckgaebe, gaebe einen Ordnerpfad heraus, der keiner ist. Beim
    /// Entpacken hat **jede Quelle** ihren eigenen Zielordner; einer davon waere
    /// eine willkuerliche Wahl, und die Stelle, die danach fragt, ist
    /// [`Auftrag::entpackziel`].
    pub fn zielordner(&self) -> Option<&PathBuf> {
        match &self.art {
            Art::Kopieren { ziel } | Art::Verschieben { ziel } => Some(ziel),
            Art::InDenPapierkorb
            | Art::UmbenennenImStapel { .. }
            | Art::Zippen { .. }
            | Art::Entpacken { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn ein_entpackauftrag_traegt_die_ziele_stelle_fuer_stelle_zu_den_archiven() {
        let auftrag = Auftrag::entpacken(vec![
            (PathBuf::from("/tmp/eins.zip"), PathBuf::from("/tmp/eins")),
            (PathBuf::from("/tmp/zwei.zip"), PathBuf::from("/tmp/zwei")),
        ]);

        assert_eq!(
            auftrag.quellen,
            vec![
                PathBuf::from("/tmp/eins.zip"),
                PathBuf::from("/tmp/zwei.zip")
            ]
        );
        assert_eq!(auftrag.entpackziel(0), Some(Path::new("/tmp/eins")));
        assert_eq!(auftrag.entpackziel(1), Some(Path::new("/tmp/zwei")));
        assert_eq!(auftrag.entpackziel(2), None, "jenseits der Liste");
        assert_eq!(
            auftrag.zielordner(),
            None,
            "jedes Archiv hat seinen eigenen Zielordner, und einer davon waere eine willkuerliche Wahl"
        );
    }

    #[test]
    fn eine_andere_art_kennt_kein_entpackziel() {
        let auftrag = Auftrag::zippen(vec![PathBuf::from("/tmp/a")], "/tmp/a.zip");
        assert_eq!(auftrag.entpackziel(0), None);
    }

    #[test]
    fn ohne_angabe_wird_bei_einem_konflikt_gefragt() {
        let auftrag = Auftrag::kopieren(Vec::new(), "/tmp");
        assert_eq!(auftrag.konfliktregel, Konfliktregel::Fragen);
    }
}
