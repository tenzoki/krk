//! Was zu tun ist: Quelle, Ziel, Art und Konfliktregel.
//!
//! Ein [`Auftrag`] ist ein Wert ohne Verhalten. Er sagt, was geschehen soll,
//! und nichts darueber, wie oder wann. Ausgefuehrt wird er von
//! [`super::starten`] auf einem eigenen Arbeitsfaden.
//!
//! **Das Ziel steht in der Art und nicht daneben.** Kopieren und Verschieben
//! brauchen einen Zielordner, Papierkorb und Stapelumbenennen nicht, das Packen
//! eine Zieldatei, das Entpacken eine ganze Liste davon und das Duplizieren
//! einen Namen. Ein flaches Feld `ziel` haette bei mehreren Arten keinen Wert,
//! den der Aufrufer sinnvoll fuellen koennte, und jede Auswertung muesste sich
//! darauf verlassen, dass er ihn trotzdem richtig gefuellt hat.

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
    /// Die eine Quelle unter einem anderen Namen in **ihren eigenen** Ordner
    /// duplizieren.
    ///
    /// **Eine eigene Art und kein Kopieren mit anderem Namen.**
    /// [`Art::Kopieren`] traegt die Zusage "die Quellen behalten ihre Namen",
    /// weist Quelle und Ziel im selben Ordner ab und klaert ein vorhandenes
    /// Ziel ueber [`super::ziel_klaeren`], dessen Zweig "ueberschreiben" den
    /// vorhandenen Eintrag endgueltig entfernt. Ein Feld "anderer Name" dort
    /// liesse genau diesen Zweig erreichbar. Diese Art ruft ihn nie; uebertragen
    /// wird ueber denselben Rumpf wie beim Kopieren
    /// (`kopieren::datei_uebertragen`), und wo die Konfliktfrage sitzt, steht
    /// im Kopf von [`super::duplizieren`].
    ///
    /// **Nicht zu ueberschreiben haelt das Dateisystem, und gefragt wird nach
    /// jedem Versuch neu.** Eine Vorabpruefung gibt es nicht: angelegt wird
    /// ausschliessend, und meldet das Dateisystem einen vergebenen Namen, geht
    /// die Frage nach einem anderen Namen ueber den Konfliktkanal
    /// ([`super::Meldung::Konflikt`]) an den Hauptfaden. Jede Antwort, die kein
    /// Name ist, beendet den Vorgang, ohne etwas anzulegen. **Die
    /// [`Konfliktregel`] des Auftrags gilt fuer diese Art deshalb nicht**: sie
    /// regelt Ersetzen und Ueberspringen ueber mehrere Quellen, und diese Art
    /// hat eine Quelle und ersetzt nie.
    Duplizieren {
        /// Der Name des Duplikats, **kein** Pfad.
        ///
        /// Der Name und nicht ein voller Zielpfad, damit "im selben Ordner" aus
        /// der Bauform folgt: das Ziel ist der Pfad der Quelle mit diesem Namen
        /// an der letzten Stelle. Ob er ein Name ist, prueft der Kern mit
        /// [`super::name_pruefen`], bevor er ins Dateisystem geht; ob er frei
        /// ist, beantwortet allein das Dateisystem.
        neuer_name: String,
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
    #[must_use]
    pub fn kopieren(quellen: Vec<PathBuf>, ziel: impl Into<PathBuf>) -> Self {
        Self::neu(quellen, Art::Kopieren { ziel: ziel.into() })
    }

    /// Verschieben in den genannten Ordner.
    #[must_use]
    pub fn verschieben(quellen: Vec<PathBuf>, ziel: impl Into<PathBuf>) -> Self {
        Self::neu(quellen, Art::Verschieben { ziel: ziel.into() })
    }

    /// In den Papierkorb des Systems raeumen.
    #[must_use]
    pub fn in_den_papierkorb(quellen: Vec<PathBuf>) -> Self {
        Self::neu(quellen, Art::InDenPapierkorb)
    }

    /// Im Stapel umbenennen (C4).
    ///
    /// Genommen werden Paare aus altem Pfad und neuem Namen, damit die beiden
    /// Listen gar nicht erst getrennt uebergeben werden koennen. Aufgetrennt
    /// werden sie hier, einmal, und danach laufen sie Stelle fuer Stelle
    /// nebeneinander.
    #[must_use]
    pub fn umbenennen_im_stapel(paare: Vec<(PathBuf, String)>) -> Self {
        let (quellen, neue_namen): (Vec<PathBuf>, Vec<String>) = paare.into_iter().unzip();
        Self::neu(quellen, Art::UmbenennenImStapel { neue_namen })
    }

    /// Die genannten Eintraege in **ein** Archiv packen.
    ///
    /// `ziel` ist der volle Pfad des Archivs und kein Ordner; die Namensbildung
    /// gehoert der Oberflaeche.
    #[must_use]
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
    #[must_use]
    pub fn entpacken(paare: Vec<(PathBuf, PathBuf)>) -> Self {
        let (quellen, ziele): (Vec<PathBuf>, Vec<PathBuf>) = paare.into_iter().unzip();
        Self::neu(quellen, Art::Entpacken { ziele })
    }

    /// Die genannte Datei unter dem neuen Namen in ihren eigenen Ordner
    /// duplizieren.
    ///
    /// Genommen wird **eine** Quelle und keine Liste: die Art traegt einen
    /// Namen, und ein zweiter Eintrag haette keinen.
    ///
    /// **Die Konfliktregel gilt fuer diese Art nicht**, gleich was
    /// [`Auftrag::mit_konfliktregel`] danach setzt. Ein vergebener Name wird
    /// unter jeder Regel erfragt und nie ersetzt, uebersprungen oder von selbst
    /// umbenannt; die Begruendung steht an [`Art::Duplizieren`].
    #[must_use]
    pub fn duplizieren(quelle: PathBuf, neuer_name: impl Into<String>) -> Self {
        Self::neu(
            vec![quelle],
            Art::Duplizieren {
                neuer_name: neuer_name.into(),
            },
        )
    }

    /// Der neue Name der Quelle an dieser Stelle, sofern die Art einen kennt.
    ///
    /// **Die Unterscheidung ist vollstaendig und hat keinen Auffangzweig**, wie
    /// die von [`Auftrag::zielordner`] zwei Bildschirmseiten tiefer. Eine
    /// weitere Art, die wie diese eine Angabe **je Stelle** zu den Quellen
    /// fuehrt, haelt damit den Bau an. Mit `_ => None` uebersetzte sie
    /// anstandslos und meldete je Eintrag "es fehlt der neue Name" in die
    /// Abschlussliste statt in die Fehlerliste des Uebersetzers
    /// (Defekt `260826-1221`).
    ///
    /// **Das Duplizieren traegt einen Namen und keine Liste**, und er gehoert
    /// der Stelle 0: [`Auftrag::duplizieren`] nimmt genau eine Quelle. Ein von
    /// Hand gebauter Auftrag mit einer zweiten Quelle findet an deren Stelle
    /// keinen Namen, und der Lauf meldet es je Eintrag, statt denselben Namen
    /// ein zweites Mal zu vergeben.
    pub(crate) fn neuer_name(&self, stelle: usize) -> Option<&str> {
        match &self.art {
            Art::UmbenennenImStapel { neue_namen } => neue_namen.get(stelle).map(String::as_str),
            Art::Duplizieren { neuer_name } => (stelle == 0).then_some(neuer_name.as_str()),
            Art::Kopieren { .. }
            | Art::Verschieben { .. }
            | Art::InDenPapierkorb
            | Art::Zippen { .. }
            | Art::Entpacken { .. } => None,
        }
    }

    /// Der Zielordner des Archivs an dieser Stelle, sofern die Art einen kennt.
    ///
    /// Vollstaendig und ohne Auffangzweig, aus dem Grund, den
    /// [`Auftrag::neuer_name`] ausschreibt.
    pub(crate) fn entpackziel(&self, stelle: usize) -> Option<&Path> {
        match &self.art {
            Art::Entpacken { ziele } => ziele.get(stelle).map(PathBuf::as_path),
            Art::Kopieren { .. }
            | Art::Verschieben { .. }
            | Art::InDenPapierkorb
            | Art::UmbenennenImStapel { .. }
            | Art::Zippen { .. }
            | Art::Duplizieren { .. } => None,
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
    /// **Allein das Kopieren und das Verschieben haben einen, und `None` ist
    /// bei keiner anderen Art ein vergessener Fall.** Beim Papierkorb liegt das
    /// Ziel ausserhalb des Auftrags. Beim Stapel-Umbenennen bleibt jeder
    /// Eintrag, wo er ist. Beim Packen ist das Ziel eine **Datei** und keine
    /// Ablage fuer weitere Eintraege; wer es hier zurueckgaebe, gaebe einen
    /// Ordnerpfad heraus, der keiner ist. Beim Entpacken hat **jede Quelle**
    /// ihren eigenen Zielordner; einer davon waere eine willkuerliche Wahl, und
    /// die Stelle, die danach fragt, ist [`Auftrag::entpackziel`]. Beim
    /// Duplizieren entsteht das Duplikat im Ordner der Quelle; der Auftrag
    /// traegt dafuer einen Namen und keinen Ordner, und die Stelle, die danach
    /// fragt, ist [`Auftrag::neuer_name`].
    #[must_use]
    pub fn zielordner(&self) -> Option<&PathBuf> {
        match &self.art {
            Art::Kopieren { ziel } | Art::Verschieben { ziel } => Some(ziel),
            Art::InDenPapierkorb
            | Art::UmbenennenImStapel { .. }
            | Art::Zippen { .. }
            | Art::Entpacken { .. }
            | Art::Duplizieren { .. } => None,
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
    fn ein_duplizierauftrag_traegt_eine_quelle_und_ihren_namen_an_der_stelle_null() {
        let auftrag = Auftrag::duplizieren(PathBuf::from("/tmp/a.txt"), "b.txt");

        assert_eq!(auftrag.quellen, vec![PathBuf::from("/tmp/a.txt")]);
        assert_eq!(
            auftrag.art,
            Art::Duplizieren {
                neuer_name: "b.txt".to_owned()
            }
        );
        assert_eq!(auftrag.neuer_name(0), Some("b.txt"));
        assert_eq!(
            auftrag.neuer_name(1),
            None,
            "der Name gehoert der einen Quelle und keiner zweiten"
        );
        assert_eq!(auftrag.entpackziel(0), None);
        assert_eq!(
            auftrag.zielordner(),
            None,
            "das Duplikat entsteht im Ordner der Quelle, und der Auftrag traegt einen Namen"
        );
    }

    #[test]
    fn ohne_angabe_wird_bei_einem_konflikt_gefragt() {
        let auftrag = Auftrag::kopieren(Vec::new(), "/tmp");
        assert_eq!(auftrag.konfliktregel, Konfliktregel::Fragen);
    }
}
