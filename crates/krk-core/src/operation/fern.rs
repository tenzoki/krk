//! Kopieren und Verschieben, wenn eine Seite auf einem Server liegt
//! (Entscheidung 9 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`), dazu
//! Umbenennen, Anlegen und endgueltiges Loeschen an einem fernen Ort
//! (Entscheidungen 10 bis 12).
//!
//! ```text
//!  Fernauftrag ──> starten ──> fern::faden("krk-operation") ──> ausfuehren
//!                                    (mit &Fernfaden)              │
//!                                                     je Quelle: einen_abarbeiten
//!                                                                  ├─> eintrag_kopieren
//!                                                                  │     anlegen (exklusiv) ──> fuellen
//!                                                                  │       └─ scheitert: LSTAT am Ziel
//!                                                                  │            Ordner auf Ordner: zusammenfuehren
//!                                                                  │            sonst: konflikt_klaeren
//!                                                                  └─> eintrag_verschieben
//!                                                                        derselbe Server: RENAME
//!                                                                          FAILURE, Ziel frei: wie sonst
//!                                                                        sonst: kopieren, dann Quelle entfernen
//!                                                     je Quelle: eintrag_umbenennen  (RENAME)
//!                                                                eintrag_anlegen     (MKDIR, OPEN CREAT|EXCL)
//!                                                                eintrag_loeschen    (REMOVE, RMDIR, READDIR)
//!  Hauptfaden <── Lauf, Meldung (unveraendert) <── Steuerung <──────────┘
//! ```
//!
//! **Umbenennen und Anlegen laufen ueber diese Maschine und nicht synchron
//! auf dem Hauptfaden wie lokal** (Entscheidungen 10 und 11): jede Anfrage an
//! einen Server braucht das Pfand eines Arbeitsfadens. Damit gilt fuer sie
//! auch die Regel „ein Vorgang zur Zeit“. Ob ein Name frei ist, beantwortet
//! das Anlegen oder das `RENAME` selbst; scheitert es und steht am Ziel ein
//! Eintrag, ist der Grund derselbe wie lokal bei `AlreadyExists`
//! ([`super::grund`]), und die Oberflaeche liest ihn dort zurueck
//! ([`super::fehlerart`]).
//!
//! **Lokal bleibt, wie es ist.** Ein Vorgang, dessen Quelle und Ziel lokal
//! sind, ist ein [`super::Auftrag`] und geht nie hierher; dieses Modul steht
//! neben der lokalen Maschine und nicht in ihr. Was beide teilen, sind die
//! [`Steuerung`] mit Fortschritt, Abbruch und Konfliktfrage, der Bericht und
//! die Meldungen an den Hauptfaden.
//!
//! **Seit den Archiven traegt die Maschine jeden Vorgang mit einer Seite, die
//! kein gewoehnlicher lokaler Ordner ist** (E10 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`); der Name
//! `Fern…` ist geblieben. Aus einem Archiv wird herauskopiert, und dafuer
//! **trennt das Modul Lesen und Schreiben am Typ** (E11): eine [`Quelle`] ist
//! eine [`Stelle`] oder ein Eintrag in einem Archiv und traegt allein
//! Lesehandlungen; die Schreibhandlungen stehen allein an [`Stelle`], und jedes
//! Ziel ist eine. Gelesen wird ein Archiveintrag ueber den
//! `archiv::Archivleser`, den alle Quellen eines Archivs im Vorgang teilen, in
//! einem tar-Strom in der Folge der Fundstellen, damit ein Baum einen
//! Durchlauf kostet. Fuer das Ziel gelten Zielklaerung, Konfliktfrage und
//! Tausch unveraendert. **Was in ein Archiv schreibt, schreibt diese Maschine
//! nicht selbst**: Hineinkopieren, Hineinverschieben, Umbenennen, Anlegen und
//! Loeschen im Archiv gehen an einen Umbau des ganzen Archivs (E13), und
//! Verschieben heraus kopiert hier und entfernt danach mit einem Umbau des
//! Quellarchivs; die Weiche steht in [`ausfuehren`], der Umbau im Kindmodul
//! `archiv`.
//!
//! **Eine Schleife fuer jede Richtung.** Lokal nach fern, fern nach lokal und
//! fern nach fern, auf demselben Server wie zwischen zweien, laufen durch
//! dieselbe Uebertragung zwischen einer Lesequelle und einem Schreibziel, von
//! denen jedes eine lokale Datei oder ein Handle auf dem Server ist
//! ([`Stelle`]). Gelesen und geschrieben wird in Bloecken zu
//! [`crate::fern::BLOCK`] Bytes mit bis zu sechzehn offenen Anfragen; nach
//! jedem Block kommen der Fortschritt und die Frage nach dem Abbruch. Die
//! Erweiterung `copy-data` wird nicht benutzt, sie waere ein zweiter Kopierweg
//! neben dieser Schleife.
//!
//! **Das Anlegen ist die eine Pruefung, ob ein Name frei ist.** Eine Datei
//! entsteht ausschliessend (lokal `create_new`, fern `CREAT|EXCL`), ein Ordner
//! ueber `mkdir`, eine Verknuepfung ueber `symlink`. **Ob ein Scheitern ein
//! vergebener Name ist, beantwortet eine Regel** ([`vorhandenes_ziel`]): lokal
//! allein `AlreadyExists`, fern jeder Statuscode ausser „gibt es nicht“ und
//! „keine Rechte“, und in beiden Faellen nur, wenn `LSTAT` am Ziel danach
//! einen Eintrag findet; SFTP v3 kennt keinen eigenen Code fuer „vorhanden“,
//! und `FAILURE` bleibt mehrdeutig. Jedes andere Scheitern ist ein Fehler des
//! Eintrags und steht mit seinem Grund im Bericht. Ist der Name vergeben, geht
//! die Konfliktfrage ueber [`Steuerung::konflikt_loesen_mit`] an dasselbe
//! Blatt wie lokal. Ordner auf Ordner ist kein Konflikt.
//!
//! **„Ueberschreiben“ entfernt das Ziel erst, wenn sein Ersatz vollstaendig
//! an seinem Platz steht.** Der Ersatz entsteht unter einem Zwischennamen im
//! Ordner des Ziels (`.krk-ersatz-<prozess>-<n>`): beim Kopieren als
//! vollstaendige Kopie, beim Verschieben auf demselben Server als `RENAME` der
//! Quelle dorthin. Kommt er nicht vollstaendig an, faellt er weg, und das Ziel
//! bleibt, wie es war. Dann tauscht [`tauschen`] in drei Schritten, von denen
//! die ersten zwei je ein `RENAME` sind und das Ziel nicht anfassen:
//!
//! 1. Das Ziel geht unter einen eigenen Zwischennamen beiseite
//!    (`.krk-alt-<prozess>-<n>`). Scheitert das, steht es unveraendert, und
//!    der Ersatz faellt weg.
//! 2. Der Ersatz bekommt den Namen des Ziels. Scheitert das, kommt das Ziel
//!    unter seinen Namen zurueck, und der Ersatz faellt weg; scheitert auch
//!    das Zurueckholen, nennt der Bericht beide Zwischennamen.
//! 3. Erst jetzt wird das beiseitegelegte Ziel endgueltig entfernt, ein Ordner
//!    samt Inhalt. Scheitert das an einem Kind, steht der Ersatz schon am
//!    Ziel, und der Bericht nennt den Zwischennamen, unter dem der Rest des
//!    alten Ziels liegt.
//!
//! Ein `RENAME` eines Ordners braucht allein das Schreibrecht im Elternordner
//! und keines in seinem Inhalt; ein Ziel, das sich nicht ganz entfernen
//! laesst, wird so nie zur Haelfte entfernt, bevor sein Ersatz steht. Ein
//! `RENAME` ueber ein vorhandenes Ziel hinweg kennt SFTP v3 nicht, und
//! `posix-rename@openssh.com` bleibt nach Entscheidung 10 ungenutzt; zwischen
//! dem ersten und dem zweiten Schritt traegt der Name des Ziels deshalb einen
//! Augenblick lang keinen Eintrag, waehrend Ziel und Ersatz beide vollstaendig
//! daneben liegen.
//!
//! **Ein `RENAME`, dessen Antwort mit der Verbindung verloren geht, gilt als
//! vielleicht geschehen** ([`Fehlschlag::laesst_den_ausgang_offen`]): die
//! Anfrage kann den Server erreicht haben. Bleibt das Beiseitelegen des Ziels
//! ohne Antwort, nennt der Bericht unter dem Namen des Ziels den
//! Zwischennamen, unter dem es dann liegt; bleibt beim Verschieben auf
//! demselben Server das `RENAME` der Quelle auf ihren Zwischennamen oder
//! zurueck ohne Antwort, nennt er den Zwischennamen der Quelle. Die Schritte
//! danach, Ersatz auf den Namen und Zurueckholen, nennen beide Zwischennamen
//! ohnehin.
//!
//! **Liegt mit dem Ziel auch die Quelle beiseite, wird nicht getauscht.**
//! Nach dem ersten Schritt fragt [`tauschen`] beim Kopieren und beim
//! Verschieben ueber eine Grenze, ob die Quelle noch unter ihrem Namen steht
//! (`LSTAT`, lokal `symlink_metadata`). Fehlt sie, ist sie das Ziel selbst,
//! erreicht ueber einen zweiten Weg, oder sie liegt in ihm; das Ziel kommt
//! zurueck, der Ersatz faellt weg, und der Eintrag steht im Bericht. Ohne
//! diese Frage entfernte das Verschieben danach die Quelle, unter deren Namen
//! nun der Ersatz stuende, und die Datei waere an keinem Ort mehr. Laesst sich
//! die Quelle nicht fragen, wird ebenso nicht getauscht. **Die Frage traegt
//! nur, wo die Antwort nicht aus einem Cache kommt**: auf einem Netzlaufwerk
//! (NFS, SMB) kann der Client einen Namen bejahen, den der Server eben ueber
//! sftp umbenannt hat. Deshalb ersetzt ein Verschieben ueber eine Grenze nicht,
//! wenn die lokale Seite kein `MNT_LOCAL` traegt ([`ersetzen_erlaubt`], die
//! Abfrage in `verzeichnis::sys::auf_lokalem_dateisystem`), und der Bericht
//! nennt den Grund. Das Kopieren ersetzt dort weiter: es entfernt danach
//! nichts, und der Ersatz ist eine vollstaendige Kopie.
//!
//! **Ein Hinweis ist keine Luecke.** Lehnt der Server allein das Setzen von
//! Rechten und Aenderungszeit ab, steht der Inhalt ganz da, und getauscht wird
//! wie beim Kopieren ohne Konflikt; der Hinweis kommt erst nach dem Tausch in
//! den Bericht, unter dem Namen des Ziels und nicht unter dem Zwischennamen
//! ([`Hinweisweg`]). Als fertig verbucht wird der Eintrag erst, wenn der Ersatz
//! am Ziel steht. Ebenso ein Hinweis ist der Rest eines alten Ziels, das sich
//! nach dem Tausch nicht ganz entfernen liess. Beim Verschieben ueber eine
//! Grenze haelt allein eine Luecke die Quelle auf ([`ueber_die_grenze`]); nach
//! Hinweisen allein wird sie entfernt, und der Bericht nennt den Hinweis unter
//! dem Namen des Ziels und die Quelle nicht. **Ein Ordner, in dem ein Kind
//! nicht ankam, steht dagegen als kopiert und als in der Quelle geblieben
//! da**: angelegt und gefuellt ist er am Ziel, soweit es ging, und die Quelle
//! bleibt mit dem Grund „nicht vollstaendig kopiert“, weil ein Teil ihres
//! Inhalts nur dort liegt.
//!
//! **Was der Bericht nennt, ist eine Bezeichnung und fuer einen fernen Eintrag
//! kein Pfad auf dieser Platte.** [`Uebersprungen::pfad`], der Fortschritt und
//! die Konfliktfrage tragen weiter einen `PathBuf`; fuer einen Eintrag auf
//! einem Server steht darin `konto@rechner:/pfad`, dieselbe Schreibweise wie
//! im Fenstertitel. Die Oberflaeche liest daran den Namen und zeigt den Rest;
//! auf die Platte geht sie mit keinem dieser Werte. Der Plan sah dafuer den
//! Typ `Ort` in den Meldungen vor; die Abweichung haelt die Meldungen der
//! lokalen Maschine und jede Probe darauf unveraendert.
//!
//! # Was dieses Modul nicht haelt
//!
//! - Ob Quelle und Ziel auf einem Server derselbe Eintrag sind, entscheidet es
//!   nicht an der Naemlichkeit, denn ueber SFTP v3 gibt es weder Geraet noch
//!   Inode, sondern am Pfad: gleicher Pfad, oder gleicher Name unter zwei
//!   Elternordnern, die `REALPATH` auf dem Server zum selben Ordner aufloest
//!   ([`Stelle::derselbe_eintrag`]). Damit sieht es eine Verknuepfung oberhalb
//!   des Eintrags, etwa `~/htdocs` auf `/var/www`, nicht aber zwei harte
//!   Verweise auf dieselbe Datei. Laesst sich `REALPATH` nicht fragen, wird
//!   nicht ueberschrieben. Ob ein Zielordner in der Quelle liegt, fragt
//!   [`Stelle::liegt_in`] ebenso ueber `REALPATH`, und ohne Antwort am Pfad.
//! - Zwei Kennungen, die denselben Server meinen (`rechner` und
//!   `rechner:22`), gelten als zwei Server; zwischen ihnen wird kopiert und
//!   nicht umbenannt, und `REALPATH` wird nicht gefragt. Dasselbe gilt fuer
//!   eine lokale Seite und eine ferne, die denselben Speicher erreichen (ein
//!   NAS, eingehaengt und zugleich ueber sftp geoeffnet). Vor die
//!   Konfliktfrage kommt ein solcher Eintrag deshalb; „Ueberschreiben“ verliert
//!   ihn aber nicht. Liegt die lokale Seite auf einem Netzlaufwerk, wie beim
//!   eingehaengten NAS, ersetzt das Verschieben dort gar nicht, und das
//!   Kopieren verliert dort nichts, weil es danach nichts entfernt und der
//!   Ersatz eine vollstaendige Kopie ist; auf einem lokalen Dateisystem fragt
//!   [`tauschen`] nach dem Beiseitelegen des Ziels nach der Quelle (oben).
//!   **Die Grenze dieser Regel:** sie erkennt allein
//!   eine Quelle, die mit dem Ziel ihren Namen verliert. Zwei harte Verweise
//!   auf dieselbe Datei sieht sie nicht; dort bleibt nach dem Tausch der
//!   Ersatz am Ziel und der zweite Verweis an der Quelle, und das Verschieben
//!   entfernt allein den zweiten Verweis, also keine einzige Kopie.
//! - Erweiterte Attribute und Ressourcengabeln gehen auf dem Weg zum Server
//!   verloren; mit gehen allein Rechte und Aenderungszeit.
//! - Eine Verknuepfung legt es auf einem Server allein an, wenn er sich als
//!   OpenSSH zu erkennen gibt ([`Gespraech::spricht_openssh`]); sonst steht
//!   sie im Bericht.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::ffi::OsStr;
use std::fs::{self, File, FileTimes, OpenOptions, Permissions};
use std::io::{self, Read, Write};
use std::ops::ControlFlow;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::archiv::lesen::lesbarkeit_grund;
use crate::archiv::{Archiveintrag, Archivleser, Innenpfad, Lesbarkeit, Lesefehler};
use crate::fern::paket::{Attribute, Name, S_IFDIR, S_IFLNK, S_IFMT, S_IFREG, oeffnen, status};
use crate::fern::{
    self as fernzugriff, Ausstehend, BLOCK, Fernfaden, Fernfehler, Fernpfad, Gespraech, Handle,
    OFFENE_SCHREIBUNGEN, Serverkennung, ist_einzelner_name,
};
use crate::ort::Ort;
use crate::sprache::{Text, Zahlwort, anzahl, satz, text};
use crate::verzeichnis::sys::{auf_lokalem_dateisystem, ohne_warten_oeffnen};
use crate::verzeichnis::{Typ, lesen};

use super::auftrag::Konfliktregel;
use super::fortschritt::{Abschluss, Bericht, Lauf, Meldung, Steuerung, Uebersprungen};

// Ein Kind dieses Moduls und keines von `operation`: der Umbau eines Archivs
// braucht die Stellen, Angaben und Gruende dieser Maschine, und die bleiben
// privat. Die Datei steht neben dieser, wie der Plan der Archive sie nennt.
#[path = "archiv.rs"]
mod archiv;
use super::umbenennen::{freier_name, freier_name_unter, name_pruefen, umbenennen};
use super::{Ablauf, benennen_denselben_eintrag, liegt_im_ordner, loeschen};

/// Was mit den Quellen eines [`Fernauftrag`] geschehen soll.
///
/// Die Arten, die ein Vorgang mit einer fernen Seite kennt. Zippen, Entpacken,
/// Duplizieren, Stapelumbenennen und der Papierkorb stehen nicht darin und
/// sind fuer einen fernen Ort damit nicht baubar (Entscheidung 2 des Plans).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fernart {
    /// In den genannten Ordner kopieren. Die Quellen behalten ihre Namen.
    Kopieren {
        /// Der Zielordner.
        ziel: Ort,
    },
    /// In den genannten Ordner verschieben. Die Quellen behalten ihre Namen.
    Verschieben {
        /// Der Zielordner.
        ziel: Ort,
    },
    /// Die eine Quelle in ihrem Ordner umbenennen, ueber `RENAME` und nie
    /// ueber `posix-rename@openssh.com` (Entscheidung 10): ein vorhandener
    /// Name laesst es scheitern, statt still ersetzt zu werden.
    Umbenennen {
        /// Der neue Name, **kein** Pfad.
        neuer_name: String,
    },
    /// In der einen Quelle, einem Ordner, einen Eintrag anlegen (Entscheidung
    /// 11). Ein vorhandener Name laesst es scheitern.
    Anlegen {
        /// Der Name des neuen Eintrags, **kein** Pfad.
        name: String,
        /// Ordner oder leere Datei.
        typ: Anlegetyp,
    },
    /// Jede Quelle endgueltig entfernen, einen Ordner samt Inhalt
    /// (Entscheidung 12). Ein Server hat keinen Papierkorb; gestellt wird der
    /// Auftrag allein nach der Rueckfrage, die „endgueltig“ ausspricht.
    Loeschen,
}

/// Was [`Fernart::Anlegen`] anlegt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anlegetyp {
    /// Ein leerer Ordner, ueber `MKDIR`.
    Ordner,
    /// Eine leere Datei, ueber `OPEN` mit `CREAT|EXCL` und `CLOSE`.
    Datei,
}

/// Ein Auftrag an die Maschine, bei dem Quelle oder Ziel auf einem Server
/// liegt.
///
/// **Die Quellen sind Eintraege und kein Ordner mit Namen.** Der Plan schlug
/// einen Quellordner mit einer Namensliste vor, weil die Auswahl eines
/// Fensters aus einem Ordner stammt; ein Abwurf aus dem Finder bringt dagegen
/// Pfade aus beliebigen Ordnern mit, und der geht ebenfalls hierher
/// (Entscheidung 9). Jede Quelle ist deshalb ein [`Ort`], der einen Eintrag
/// benennt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fernauftrag {
    /// Die Eintraege, auf die der Auftrag wirkt, Ordner samt Inhalt.
    pub quellen: Vec<Ort>,
    /// Was mit ihnen geschehen soll.
    pub art: Fernart,
    /// Was bei einem Namenskonflikt gilt.
    pub konfliktregel: Konfliktregel,
}

impl Fernauftrag {
    /// Kopieren in den genannten Ordner.
    #[must_use]
    pub fn kopieren(quellen: Vec<Ort>, ziel: Ort) -> Self {
        Self::neu(quellen, Fernart::Kopieren { ziel })
    }

    /// Verschieben in den genannten Ordner.
    #[must_use]
    pub fn verschieben(quellen: Vec<Ort>, ziel: Ort) -> Self {
        Self::neu(quellen, Fernart::Verschieben { ziel })
    }

    /// Die eine Quelle in ihrem Ordner umbenennen.
    #[must_use]
    pub fn umbenennen(quelle: Ort, neuer_name: impl Into<String>) -> Self {
        Self::neu(
            vec![quelle],
            Fernart::Umbenennen {
                neuer_name: neuer_name.into(),
            },
        )
    }

    /// Im genannten Ordner einen Eintrag anlegen.
    #[must_use]
    pub fn anlegen(ordner: Ort, name: impl Into<String>, typ: Anlegetyp) -> Self {
        Self::neu(
            vec![ordner],
            Fernart::Anlegen {
                name: name.into(),
                typ,
            },
        )
    }

    /// Die genannten Eintraege endgueltig entfernen.
    #[must_use]
    pub fn loeschen(quellen: Vec<Ort>) -> Self {
        Self::neu(quellen, Fernart::Loeschen)
    }

    fn neu(quellen: Vec<Ort>, art: Fernart) -> Self {
        Self {
            quellen,
            art,
            konfliktregel: Konfliktregel::default(),
        }
    }

    /// Setzt die Konfliktregel.
    #[must_use]
    pub fn mit_konfliktregel(mut self, regel: Konfliktregel) -> Self {
        self.konfliktregel = regel;
        self
    }

    /// Der Zielordner, sofern die Art einen hat. Vollstaendig ueber
    /// [`Fernart`] und ohne Auffangzweig.
    ///
    /// Allein Kopieren und Verschieben haben einen. Das Umbenennen bleibt im
    /// Ordner seiner Quelle, das Anlegen entsteht in der Quelle selbst, und
    /// das Loeschen hat kein Ziel; `None` ist bei keiner der drei ein
    /// vergessener Fall, wie bei `Auftrag::zielordner`.
    #[must_use]
    pub fn zielordner(&self) -> Option<&Ort> {
        match &self.art {
            Fernart::Kopieren { ziel } | Fernart::Verschieben { ziel } => Some(ziel),
            Fernart::Umbenennen { .. } | Fernart::Anlegen { .. } | Fernart::Loeschen => None,
        }
    }
}

/// Die Bezeichnung eines Orts im Bericht: lokal sein Pfad, fern
/// `konto@rechner:/pfad`.
#[must_use]
pub fn bezeichnung(ort: &Ort) -> PathBuf {
    match ort {
        Ort::Lokal(pfad) => pfad.clone(),
        Ort::Fern(fern) => PathBuf::from(fern.to_string()),
        Ort::Archiv(archiv) => PathBuf::from(archiv.to_string()),
    }
}

/// Startet einen Fernauftrag auf einem eigenen Arbeitsfaden und kehrt sofort
/// zurueck.
///
/// Der Faden entsteht ueber [`crate::fern::faden()`] und bekommt damit das
/// Pfand, ohne das keine Anfrage an einen Server gestellt werden kann; der
/// Hauptfaden haelt allein den [`Lauf`]. Ein gescheiterter Fadenstart meldet
/// jede Quelle als uebersprungen, wie `super::starten` es lokal tut.
pub(crate) fn starten(auftrag: Fernauftrag) -> Lauf {
    let abbruch = Arc::new(AtomicBool::new(false));
    let faden_abbruch = Arc::clone(&abbruch);
    let (sender, empfaenger) = channel();
    let abschlusssender = sender.clone();
    let notsender = sender.clone();
    let auftrag = Arc::new(auftrag);
    let fuer_faden = Arc::clone(&auftrag);

    let faden = fernzugriff::faden("krk-operation".to_owned(), move |pfand| {
        let mut steuerung = Steuerung::neu(faden_abbruch, Some(sender), fuer_faden.konfliktregel);
        let abschluss = ausfuehren(pfand, &fuer_faden, &mut steuerung);
        let _ = abschlusssender.send(Meldung::Fertig(steuerung.bericht(abschluss)));
    });

    let faden = match faden {
        Ok(faden) => Some(faden),
        Err(fehler) => {
            let _ = notsender.send(Meldung::Fertig(Bericht {
                abschluss: Abschluss::Fertig,
                eintraege: 0,
                bytes: 0,
                uebersprungen: auftrag
                    .quellen
                    .iter()
                    .map(|ort| Uebersprungen {
                        pfad: bezeichnung(ort),
                        grund: satz(
                            Text::VorgangKeinArbeitsfaden,
                            &[("grund", &super::grund(&fehler))],
                        ),
                    })
                    .collect(),
            }));
            None
        }
    };

    Lauf::neu(abbruch, empfaenger, faden)
}

/// Arbeitet einen Fernauftrag Quelle fuer Quelle ab.
///
/// Der Zielordner wird einmal vor der Schleife zur [`Stelle`], damit jede
/// Quelle dasselbe Gespraech an ihm nimmt.
///
/// **Ein Ort in einem Archiv ist eine Lesequelle und keine Stelle** (E11 des
/// Plans `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`): aus
/// einem Archiv wird herauskopiert, in ein lokales oder fernes Ziel, ueber
/// [`Quelle::Archiv`]. Je Archiv entsteht ein [`Archivleser`], den jede
/// Quelle darin teilt; die Quellen eines tar oder tar.gz kommen in der Folge
/// ihrer Fundstelle dran ([`Archivleser::lesefolge`]), damit der Strom
/// einmal durchlaufen wird.
///
/// **Was in ein Archiv schreibt, geht an einen Umbau** (E10 und E13 des
/// Plans, Modul `archiv` neben dieser Datei), und die Weiche steht hier, am
/// Wert von Ziel und Quelle:
///
/// ```text
///  Ziel im Archiv (Kopieren, Verschieben)    ──> archiv::hinein, ein Umbau
///  Quelle im Archiv, Umbenennen, Anlegen,
///  Loeschen                                   ──> archiv::im_archiv_abarbeiten
///  Quelle im Archiv, Verschieben, Ziel nicht   ──> diese Maschine kopiert, dann
///  im Archiv                                       archiv::aus_archiven_entfernen
///  sonst                                      ──> diese Maschine, wie bisher
/// ```
///
/// Aus einem Archiv, das KRK allein lesen darf, verschiebt sie nichts: die
/// Quelle meldet den Text ihres Formats (`Archivformat::nur_lesbar_text`),
/// bevor am Ziel etwas entsteht.
fn ausfuehren(pfand: &Fernfaden, auftrag: &Fernauftrag, steuerung: &mut Steuerung) -> Abschluss {
    let verschieben = match &auftrag.art {
        Fernart::Kopieren { ziel } | Fernart::Verschieben { ziel } => {
            if let Some(ziel) = ziel.archiv() {
                let verschieben = matches!(auftrag.art, Fernart::Verschieben { .. });
                return archiv::hinein(pfand, &auftrag.quellen, ziel, verschieben, steuerung);
            }
            matches!(auftrag.art, Fernart::Verschieben { .. })
        }
        Fernart::Umbenennen { .. } | Fernart::Anlegen { .. } | Fernart::Loeschen => false,
    };
    let schreibt_an_der_quelle = match &auftrag.art {
        Fernart::Kopieren { .. } | Fernart::Verschieben { .. } => false,
        Fernart::Umbenennen { .. } | Fernart::Anlegen { .. } | Fernart::Loeschen => true,
    };
    let zielordner = auftrag.zielordner().and_then(Stelle::von);
    let mut leser: Vec<Archivzugriff> = Vec::new();
    let mut im_archiv: Vec<crate::ort::Archivort> = Vec::new();
    let mut quellen: Vec<(PathBuf, Result<Quelle, String>)> =
        Vec::with_capacity(auftrag.quellen.len());
    for ort in &auftrag.quellen {
        if steuerung.abgebrochen() {
            return Abschluss::Abgebrochen;
        }
        let quelle = match ort {
            Ort::Lokal(_) | Ort::Fern(_) => match Stelle::von(ort) {
                Some(stelle) => Ok(Quelle::Stelle(stelle)),
                None => Err(text(Text::VorgangPfadBenenntKeinenEintrag).to_owned()),
            },
            Ort::Archiv(archivort) if schreibt_an_der_quelle => {
                im_archiv.push(archivort.clone());
                continue;
            }
            Ort::Archiv(archivort) if verschieben && !archivort.format().beschreibbar() => {
                Err(text(archivort.format().nur_lesbar_text()).to_owned())
            }
            Ort::Archiv(archivort) => {
                match archivzugriff(pfand, &mut leser, archivort, steuerung) {
                    Ok(geteilt) => Ok(Quelle::Archiv(Archivstelle {
                        leser: geteilt,
                        innen: archivort.innen.clone(),
                    })),
                    Err(Lesefehler::Abgebrochen) => return Abschluss::Abgebrochen,
                    Err(fehler) => Err(fehler.grund()),
                }
            }
        };
        quellen.push((bezeichnung(ort), quelle));
    }
    in_lesefolge_bringen(&mut quellen);
    // Die Eintraege aus Archiven, die beim Verschieben vollstaendig am Ziel
    // stehen; entfernt werden sie erst danach, ueber einen Umbau je Archiv,
    // und auch nach einem Abbruch steht jeder davon im Bericht.
    let mut nachher: Vec<archiv::Entfernposten> = Vec::new();
    let mut abgebrochen = false;
    for (bezeichnung, quelle) in quellen {
        if steuerung.abgebrochen() {
            abgebrochen = true;
            break;
        }
        let quelle = match quelle {
            Ok(quelle) => quelle,
            Err(grund) => {
                steuerung.ueberspringen(&bezeichnung, grund);
                continue;
            }
        };
        if einen_abarbeiten(
            pfand,
            &auftrag.art,
            &quelle,
            zielordner.as_ref(),
            steuerung,
            &mut nachher,
        ) == Ablauf::Abgebrochen
        {
            abgebrochen = true;
            break;
        }
    }
    // Die Leser der Quellarchive fallen, bevor ein Umbau ihre Dateien
    // ersetzt.
    drop(leser);
    let mut ersetzte = archiv::aus_archiven_entfernen(pfand, nachher, steuerung);
    if abgebrochen {
        return Abschluss::Abgebrochen;
    }
    if !im_archiv.is_empty() {
        ersetzte.extend(archiv::im_archiv_abarbeiten(
            pfand,
            &auftrag.art,
            im_archiv,
            steuerung,
        ));
    }
    // Ohne ersetztes Archiv wie bisher: ein gesetzter Abbruch ist ein
    // Abbruch. Mit einem ersetzten Archiv kam er zu spaet (E13).
    crate::archiv::umbau::abschluss_nach_ersetzen(steuerung, &ersetzte)
}

/// Ein Archiv dieses Vorgangs: der Pfad seiner Datei und sein Leser, oder
/// der Grund, aus dem er sich nicht oeffnen liess.
type Archivzugriff = (PathBuf, Result<Rc<RefCell<Archivleser>>, String>);

/// Der geteilte Leser des Archivs, in dem `ort` liegt, beim ersten Mal
/// geoeffnet; ein gescheitertes Oeffnen wird je Archiv einmal versucht.
///
/// Gleich sind zwei Archive ueber den Pfad ihrer Datei, wie
/// [`crate::ort::Archivort`] es haelt.
fn archivzugriff(
    pfand: &Fernfaden,
    leser: &mut Vec<Archivzugriff>,
    ort: &crate::ort::Archivort,
    steuerung: &Steuerung,
) -> Result<Rc<RefCell<Archivleser>>, Lesefehler> {
    if let Some((_, vorhanden)) = leser.iter().find(|(datei, _)| datei == ort.datei()) {
        return vorhanden
            .as_ref()
            .map(Rc::clone)
            .map_err(|grund| Lesefehler::Unlesbar(grund.clone()));
    }
    let geoeffnet = match Archivleser::oeffnen(
        pfand,
        Arc::clone(&ort.zugang),
        steuerung.abbruchkennzeichen(),
    ) {
        Ok(neu) => Ok(Rc::new(RefCell::new(neu))),
        Err(Lesefehler::Abgebrochen) => return Err(Lesefehler::Abgebrochen),
        Err(fehler) => Err(fehler.grund()),
    };
    leser.push((ort.datei().to_path_buf(), geoeffnet.clone()));
    geoeffnet.map_err(Lesefehler::Unlesbar)
}

/// Ordnet die Quellen in die Lesefolge ihres Archivs, wenn alle aus einem
/// tar-Strom kommen; sonst bleibt die Folge des Auftrags.
///
/// Der Lesezeiger eines tar laeuft nur vorwaerts ([`Archivleser`]), und
/// Quellen in der Folge ihrer Fundstelle kosten damit einen Durchlauf statt
/// einen je Quelle. Ein Zip liest wahlfrei, und seine Quellen behalten wie
/// jede lokale oder ferne die Folge, in der sie kamen.
fn in_lesefolge_bringen(quellen: &mut [(PathBuf, Result<Quelle, String>)]) {
    let mut gemeinsam: Option<&Rc<RefCell<Archivleser>>> = None;
    for (_, quelle) in quellen.iter() {
        let Ok(Quelle::Archiv(stelle)) = quelle else {
            return;
        };
        match gemeinsam {
            Some(leser) if !Rc::ptr_eq(leser, &stelle.leser) => return,
            Some(_) | None => gemeinsam = Some(&stelle.leser),
        }
    }
    let Some(leser) = gemeinsam.map(Rc::clone) else {
        return;
    };
    let leser = leser.borrow();
    if !leser.liest_strom() {
        return;
    }
    quellen.sort_by_cached_key(|(_, quelle)| match quelle {
        Ok(Quelle::Archiv(stelle)) => leser.lesefolge(&stelle.innen),
        Ok(Quelle::Stelle(_)) | Err(_) => u64::MAX,
    });
}

/// Eine Quelle, nach der Art verzweigt. Vollstaendig ueber [`Fernart`] und
/// ohne Auffangzweig.
///
/// Umbenennen, Anlegen und Loeschen schreiben an der Quelle selbst und nehmen
/// deshalb allein eine [`Stelle`]; eine Quelle im Archiv kommt dafuer nicht
/// hierher, sondern an `archiv::im_archiv_abarbeiten` ([`ausfuehren`]). Kaeme
/// sie doch, meldete sie den Text ihres Formats
/// (`Archivformat::nicht_verfuegbar_text`).
///
/// `nachher` sammelt die Eintraege aus Archiven, die ein Verschieben
/// vollstaendig kopiert hat und die erst danach entfernt werden.
fn einen_abarbeiten(
    pfand: &Fernfaden,
    art: &Fernart,
    quelle: &Quelle,
    zielordner: Option<&Stelle>,
    steuerung: &mut Steuerung,
    nachher: &mut Vec<archiv::Entfernposten>,
) -> Ablauf {
    let verschieben = match art {
        Fernart::Kopieren { .. } => false,
        Fernart::Verschieben { .. } => true,
        Fernart::Umbenennen { .. } | Fernart::Anlegen { .. } | Fernart::Loeschen => {
            let stelle = match quelle {
                Quelle::Stelle(stelle) => stelle,
                Quelle::Archiv(archivstelle) => {
                    let grund = archivstelle
                        .leser
                        .borrow()
                        .zugang()
                        .format()
                        .nicht_verfuegbar_text();
                    steuerung.ueberspringen(&quelle.bezeichnung(), text(grund));
                    return Ablauf::Weiter;
                }
            };
            return an_der_stelle_abarbeiten(pfand, art, stelle, steuerung);
        }
    };
    // `zielordner` steht genau dann, wenn die Art einen hat
    // ([`Fernauftrag::zielordner`]); der Fall ist trotzdem behandelt, wie die
    // fehlenden Angaben je Stelle in `super::einen_abarbeiten`.
    let Some(zielordner) = zielordner else {
        steuerung.ueberspringen(&quelle.bezeichnung(), text(Text::VorgangZielordnerFehlt));
        return Ablauf::Weiter;
    };
    einen_uebertragen(pfand, verschieben, quelle, zielordner, steuerung, nachher)
}

/// Umbenennen, Anlegen oder Loeschen an einer Stelle. Kopieren und
/// Verschieben kommen nicht hierher; ihr Zweig steht, weil die
/// Unterscheidung vollstaendig ist, und meldet den fehlenden Zielordner wie
/// [`einen_abarbeiten`].
fn an_der_stelle_abarbeiten(
    pfand: &Fernfaden,
    art: &Fernart,
    quelle: &Stelle,
    steuerung: &mut Steuerung,
) -> Ablauf {
    match art {
        Fernart::Umbenennen { neuer_name } => {
            eintrag_umbenennen(pfand, quelle, neuer_name, steuerung)
        }
        Fernart::Anlegen { name, typ } => eintrag_anlegen(pfand, quelle, name, *typ, steuerung),
        Fernart::Loeschen => eintrag_loeschen(pfand, quelle, steuerung),
        Fernart::Kopieren { .. } | Fernart::Verschieben { .. } => {
            steuerung.ueberspringen(&quelle.bezeichnung(), text(Text::VorgangZielordnerFehlt));
            Ablauf::Weiter
        }
    }
}

/// Eine Quelle kopieren oder verschieben: Angaben holen, Ziel rechnen, die
/// zwei Faelle abweisen, in denen es keines gibt, und dann verzweigen.
fn einen_uebertragen(
    pfand: &Fernfaden,
    verschieben: bool,
    quelle: &Quelle,
    zielordner: &Stelle,
    steuerung: &mut Steuerung,
    nachher: &mut Vec<archiv::Entfernposten>,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    let angaben = match quelle.angaben(pfand) {
        Ok(angaben) => angaben,
        Err(fehler) => {
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return Ablauf::Weiter;
        }
    };
    let name = match quelle.name() {
        Ok(name) => name,
        Err(schluessel) => {
            steuerung.ueberspringen(&bezeichnung, text(schluessel));
            return Ablauf::Weiter;
        }
    };
    let Some(ziel) = zielordner.kind(&name) else {
        steuerung.ueberspringen(&bezeichnung, text(Text::VorgangFernNameOhneUtf8));
        return Ablauf::Weiter;
    };
    // Ein unklarer Befund laesst den Vorgang weiterlaufen: er endet am Ziel
    // allein dann in einem Entfernen, wenn „Ueberschreiben“ kommt, und dort
    // haelt ihn [`ersetzen_erlaubt`] an.
    match quelle.derselbe_eintrag(pfand, &ziel) {
        Gleichheit::Ja => {
            steuerung.ueberspringen(
                &bezeichnung,
                text(Text::VorgangQuelleUndZielDerselbeEintrag),
            );
            return Ablauf::Weiter;
        }
        Gleichheit::Nein | Gleichheit::Unklar(_) => {}
    }
    if angaben.typ == Typ::Ordner && quelle.enthaelt(pfand, zielordner) {
        steuerung.ueberspringen(&bezeichnung, text(Text::VorgangZielLiegtInDerQuelle));
        return Ablauf::Weiter;
    }
    match (verschieben, quelle) {
        (true, Quelle::Stelle(stelle)) => {
            eintrag_verschieben(pfand, stelle, &angaben, ziel, steuerung)
        }
        // Aus einem Archiv wird kopiert, und entfernt wird die Quelle erst
        // nach dem Vorgang, mit einem Umbau des Archivs ([`ausfuehren`]).
        (true, Quelle::Archiv(stelle)) => {
            aus_dem_archiv_verschieben(pfand, stelle, &angaben, ziel, steuerung, nachher)
        }
        (false, _) => eintrag_kopieren(
            pfand,
            quelle,
            &angaben,
            ziel,
            Zweck::Kopieren,
            steuerung,
            &mut Hinweisweg::bericht(),
        ),
    }
}

/// Verschiebt einen Eintrag aus einem Archiv an ein lokales oder fernes Ziel:
/// kopieren wie [`ueber_die_grenze`] und, allein wenn seit dem Beginn dieses
/// Eintrags keine Luecke in den Bericht kam, ihn in `nachher` vormerken.
///
/// **Entfernt wird hier nichts.** Ein Archiv laesst sich allein ganz neu
/// schreiben; jeder Eintrag einzeln hiesse ein Umbau je Eintrag. Das
/// Entfernen aller vorgemerkten Eintraege eines Archivs ist deshalb ein Umbau
/// nach dem letzten Eintrag des Vorgangs (`archiv::aus_archiven_entfernen`).
fn aus_dem_archiv_verschieben(
    pfand: &Fernfaden,
    stelle: &Archivstelle,
    angaben: &Angaben,
    ziel: Stelle,
    steuerung: &mut Steuerung,
    nachher: &mut Vec<archiv::Entfernposten>,
) -> Ablauf {
    let bezeichnung = stelle.bezeichnung();
    let stand = steuerung.uebersprungen_stand();
    let mut weg = Hinweisweg::bericht();
    if eintrag_kopieren(
        pfand,
        &Quelle::Archiv(stelle.clone()),
        angaben,
        ziel,
        Zweck::Verschieben,
        steuerung,
        &mut weg,
    ) == Ablauf::Abgebrochen
    {
        return Ablauf::Abgebrochen;
    }
    let seither = steuerung.uebersprungen_seit(stand);
    if seither.len() != weg.gemeldet() {
        let selbst_genannt = seither.iter().any(|eintrag| eintrag.pfad == bezeichnung);
        if !selbst_genannt {
            steuerung.ueberspringen(&bezeichnung, text(Text::VorgangNichtVollstaendigKopiert));
        }
        return Ablauf::Weiter;
    }
    let (zugang, kennung) = {
        let leser = stelle.leser.borrow();
        (Arc::clone(leser.zugang()), leser.verzeichnis().kennung())
    };
    nachher.push(archiv::Entfernposten::neu(
        zugang,
        stelle.innen.clone(),
        bezeichnung,
        kennung,
    ));
    Ablauf::Weiter
}

// ----------------------------------------------------------------------
// Umbenennen, Anlegen, Loeschen (Entscheidungen 10 bis 12)
// ----------------------------------------------------------------------

/// Benennt die Quelle in ihrem Ordner um.
///
/// **Ob der neue Name frei ist, beantwortet das `RENAME` selbst**: der Entwurf
/// von SFTP v3 laesst es an einem vorhandenen Ziel scheitern, und der Server
/// des Systems haelt das (`rename_weist_ein_vorhandenes_ziel_ab` in
/// `tests/fern.rs`). Eine Vorabfrage gibt es nicht, wie lokal in
/// [`super::umbenennen()`]. Eine lokale Quelle geht ueber genau diesen lokalen
/// Weg; die Oberflaeche stellt einen solchen Auftrag nicht, er ist aber
/// baubar und tut dann dasselbe wie lokal.
fn eintrag_umbenennen(
    pfand: &Fernfaden,
    quelle: &Stelle,
    neuer_name: &str,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    if let Err(fehler) = name_pruefen(neuer_name) {
        steuerung.ueberspringen(&bezeichnung, fehler.grund());
        return Ablauf::Weiter;
    }
    match quelle.umbenennen_in(pfand, neuer_name) {
        Ok(()) => steuerung.eintrag_fertig(&bezeichnung, 0),
        Err(fehler) => {
            let ziel = quelle.geschwister(neuer_name);
            steuerung.ueberspringen(&bezeichnung, namensgrund(pfand, ziel.as_ref(), &fehler));
        }
    }
    Ablauf::Weiter
}

/// Legt im Ordner `ordner` einen Eintrag namens `name` an, ausschliessend.
///
/// Ein vorhandener Name laesst das Anlegen selbst scheitern, wie lokal in
/// [`super::ordner_anlegen`] und [`super::datei_anlegen`]; der Bericht nennt
/// den neuen Eintrag, gelungen wie gescheitert.
fn eintrag_anlegen(
    pfand: &Fernfaden,
    ordner: &Stelle,
    name: &str,
    typ: Anlegetyp,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let Some(ziel) = ordner.kind(name) else {
        steuerung.ueberspringen(
            &ordner.bezeichnung().join(name),
            namensgrund_ohne_ziel(name),
        );
        return Ablauf::Weiter;
    };
    let bezeichnung = ziel.bezeichnung();
    if let Err(fehler) = name_pruefen(name) {
        steuerung.ueberspringen(&bezeichnung, fehler.grund());
        return Ablauf::Weiter;
    }
    let angelegt = match typ {
        Anlegetyp::Ordner => ziel.ordner_anlegen(pfand),
        Anlegetyp::Datei => ziel
            .datei_anlegen(pfand)
            .and_then(|schreibziel| schreibziel.schliessen(pfand)),
    };
    match angelegt {
        Ok(()) => steuerung.eintrag_fertig(&bezeichnung, 0),
        Err(fehler) => {
            steuerung.ueberspringen(&bezeichnung, namensgrund(pfand, Some(&ziel), &fehler));
        }
    }
    Ablauf::Weiter
}

/// Der Grund, wenn ein Name keine einzelne Komponente ist und sich deshalb
/// kein Pfad zu ihm bauen laesst: der der Namenspruefung, sonst der
/// allgemeine.
fn namensgrund_ohne_ziel(name: &str) -> String {
    match name_pruefen(name) {
        Err(fehler) => fehler.grund().to_owned(),
        Ok(()) => text(Text::VorgangPfadBenenntKeinenEintrag).to_owned(),
    }
}

/// Entfernt die Quelle endgueltig, einen Ordner samt Inhalt.
///
/// Haelt am ersten Fehler an, wie [`super::loeschen::baum_entfernen`]: was
/// stehen blieb, steht mit seinem Grund im Bericht, und die naechste Quelle
/// kommt dran. Die Angaben vorweg liefern die Groesse fuer den Bericht und
/// melden eine Quelle, die es nicht mehr gibt, mit ihrem Grund.
///
/// **Der Abbruch wirkt mitten im Baum**: gefragt wird vor jedem `READDIR` und
/// vor jedem Eintrag ([`fern_baum_entfernen`]). Was bis dahin entfernt ist,
/// bleibt entfernt; der Rest steht, und der Vorgang endet als abgebrochen.
fn eintrag_loeschen(pfand: &Fernfaden, quelle: &Stelle, steuerung: &mut Steuerung) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    let angaben = match quelle.angaben(pfand) {
        Ok(angaben) => angaben,
        Err(fehler) => {
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return Ablauf::Weiter;
        }
    };
    let entfernt = quelle.baum_entfernen(pfand, &|| steuerung.abgebrochen());
    match entfernt {
        Ok(()) => steuerung.eintrag_fertig(&bezeichnung, angaben.groesse),
        Err(Fehlschlag::Abgebrochen) => return Ablauf::Abgebrochen,
        Err(fehler) => steuerung.ueberspringen(&bezeichnung, grund(&fehler)),
    }
    Ablauf::Weiter
}

/// Der Grund, wenn Umbenennen oder Anlegen scheitert.
///
/// **SFTP v3 kennt keinen Code fuer „vorhanden“**: der Server des Systems
/// antwortet auf ein vergebenes Ziel mit dem allgemeinen `FAILURE`. Ob der
/// Name vergeben ist, sagt deshalb dieselbe Regel wie beim Kopieren,
/// [`vorhandenes_ziel`]; ist er es, ist der Grund der von [`super::grund`] fuer
/// `AlreadyExists`, also genau der, den ein lokales Scheitern an einem
/// vergebenen Namen traegt. Jedes andere Scheitern behaelt seinen eigenen
/// Grund.
fn namensgrund(pfand: &Fernfaden, ziel: Option<&Stelle>, fehler: &Fehlschlag) -> String {
    let vergeben = ziel.is_some_and(|ziel| vorhandenes_ziel(pfand, fehler, ziel).is_some());
    if vergeben {
        super::grund(&io::Error::from(io::ErrorKind::AlreadyExists))
    } else {
        grund(fehler)
    }
}

// ----------------------------------------------------------------------
// Kopieren
// ----------------------------------------------------------------------

/// Was am Ziel anzulegen ist.
enum Anlage {
    Datei,
    Ordner,
    /// Eine Verknuepfung mit diesem Verweis, als Bytes.
    Verknuepfung(Vec<u8>),
}

/// Was das Anlegen am Ziel hinterlassen hat.
enum Angelegt {
    Datei(Schreibziel),
    Ordner,
    Verknuepfung,
}

/// Wie ein Konflikt am Ziel ausging.
enum Klaerung {
    /// Das Ziel soll ersetzt werden; entfernt ist es noch nicht (Modulkopf,
    /// „Ueberschreiben“).
    Ersetzen,
    /// Unter einem anderen Namen weiter.
    Nach(Stelle),
    /// Diese Quelle auslassen; sie steht schon im Bericht.
    Ueberspringen,
    /// Den ganzen Vorgang beenden.
    Abbrechen,
}

/// Wohin ein Hinweis zu einem Eintrag geht, der vollstaendig angekommen ist:
/// Rechte oder Aenderungszeit liessen sich nicht setzen, oder vom alten Ziel
/// eines Tauschs blieb ein Rest.
///
/// Ein Hinweis ist keine Luecke. Er zaehlt nicht zur Vollstaendigkeit eines
/// Ersatzes (Modulkopf, „Ueberschreiben“), und er haelt das Entfernen der
/// Quelle beim Verschieben ueber eine Grenze nicht auf ([`ueber_die_grenze`]
/// zaehlt ihn deshalb). Er soll den Namen tragen, unter dem der Eintrag am
/// Ende steht. Ein Ersatz liegt beim Fuellen noch unter seinem Zwischennamen;
/// deshalb haelt [`kopie_ersetzt_ziel`] die Hinweise bis zum Tausch zurueck.
enum Hinweisweg<'a> {
    /// Gleich in den Bericht: der Eintrag steht schon unter seinem Namen.
    /// `hinweise` zaehlt, wie viele Eintraege des Berichts Hinweise sind.
    Bericht { hinweise: usize },
    /// Zurueckgehalten, als Bezeichnung und Grund, bis der Ersatz seinen
    /// Namen traegt.
    Ersatz(&'a mut Vec<(PathBuf, String)>),
}

impl Hinweisweg<'_> {
    /// Der Weg in den Bericht, noch ohne Hinweis.
    fn bericht() -> Self {
        Hinweisweg::Bericht { hinweise: 0 }
    }

    fn melden(&mut self, steuerung: &mut Steuerung, pfad: PathBuf, grund: String) {
        match self {
            Hinweisweg::Bericht { hinweise } => {
                *hinweise += 1;
                steuerung.ueberspringen(&pfad, grund);
            }
            Hinweisweg::Ersatz(zurueckgehalten) => zurueckgehalten.push((pfad, grund)),
        }
    }

    /// Wie viele Hinweise dieser Weg in den Bericht gegeben hat; ein
    /// zurueckhaltender Weg hat keinen gegeben.
    fn gemeldet(&self) -> usize {
        match self {
            Hinweisweg::Bericht { hinweise } => *hinweise,
            Hinweisweg::Ersatz(_) => 0,
        }
    }
}

/// Wozu ein Eintrag kopiert wird: als Kopie, oder als erste Haelfte eines
/// Verschiebens ueber eine Grenze, nach der die Quelle entfernt wird.
///
/// Die Unterscheidung braucht allein [`ersetzen_erlaubt`]: beim Verschieben
/// haengt am Tausch, ob danach die einzige Kopie entfernt wird (Modulkopf,
/// „Was dieses Modul nicht haelt“).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Zweck {
    Kopieren,
    Verschieben,
}

/// Setzt Rechte und Aenderungszeit am eben gefuellten `ziel`; scheitert das,
/// geht der Hinweis mit `schluessel` ueber `weg`.
fn angaben_uebernehmen(
    pfand: &Fernfaden,
    ziel: &Stelle,
    angaben: &Angaben,
    schluessel: Text,
    steuerung: &mut Steuerung,
    weg: &mut Hinweisweg<'_>,
) {
    if let Err(fehler) = ziel.angaben_setzen(pfand, angaben) {
        weg.melden(
            steuerung,
            ziel.bezeichnung(),
            satz(schluessel, &[("grund", &grund(&fehler))]),
        );
    }
}

/// Kopiert einen Eintrag an sein Ziel, samt Konfliktbehandlung.
///
/// **Die Schleife endet**: „Ueberschreiben“ geht an [`kopie_ersetzt_ziel`]
/// und kehrt nicht zurueck; nach „Umbenennen“ geht es nur unter einem anderen
/// Namen weiter.
fn eintrag_kopieren(
    pfand: &Fernfaden,
    quelle: &Quelle,
    angaben: &Angaben,
    ziel: Stelle,
    zweck: Zweck,
    steuerung: &mut Steuerung,
    weg: &mut Hinweisweg<'_>,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    // Was aus einem Archiv nicht herauskommt, steht mit seinem Grund im
    // Bericht, bevor am Ziel etwas entsteht.
    if let Some(warum) = quelle.kommt_nicht_heraus() {
        steuerung.ueberspringen(&bezeichnung, warum);
        return Ablauf::Weiter;
    }
    let anlage = match angaben.typ {
        Typ::Datei if !angaben.gewoehnlich => {
            steuerung.ueberspringen(&bezeichnung, text(Text::VorgangKeineGewoehnlicheDatei));
            return Ablauf::Weiter;
        }
        Typ::Datei => Anlage::Datei,
        Typ::Ordner => Anlage::Ordner,
        Typ::Verknuepfung => match quelle.verknuepfung_lesen(pfand) {
            Ok(verweis) => Anlage::Verknuepfung(verweis),
            Err(fehler) => {
                steuerung.ueberspringen(&bezeichnung, grund(&fehler));
                return Ablauf::Weiter;
            }
        },
    };

    let mut ziel = ziel;
    loop {
        let fehler = match anlegen(pfand, &ziel, &anlage) {
            Ok(Angelegt::Datei(schreibziel)) => {
                return datei_fuellen(pfand, quelle, angaben, &ziel, schreibziel, steuerung, weg);
            }
            Ok(Angelegt::Ordner) => {
                return ordner_fuellen(pfand, quelle, angaben, &ziel, zweck, steuerung, weg);
            }
            Ok(Angelegt::Verknuepfung) => {
                steuerung.eintrag_fertig(&bezeichnung, 0);
                return Ablauf::Weiter;
            }
            Err(fehler) => fehler,
        };
        // Allein ein vergebener Name ist ein Konflikt; jedes andere
        // Scheitern ist ein Fehler dieses Eintrags.
        let Some(vorhanden) = vorhandenes_ziel(pfand, &fehler, &ziel) else {
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return Ablauf::Weiter;
        };
        if angaben.typ == Typ::Ordner && vorhanden.typ == Typ::Ordner {
            return ordner_fuellen(pfand, quelle, angaben, &ziel, zweck, steuerung, weg);
        }
        match konflikt_klaeren(pfand, &bezeichnung, &ziel, steuerung) {
            Klaerung::Ersetzen => {
                if !ersetzen_erlaubt(pfand, quelle, &ziel, zweck, steuerung) {
                    return Ablauf::Weiter;
                }
                return kopie_ersetzt_ziel(pfand, quelle, angaben, &anlage, &ziel, steuerung, weg);
            }
            Klaerung::Nach(neues) => {
                if neues.bezeichnung() == ziel.bezeichnung() {
                    steuerung.ueberspringen(&bezeichnung, text(Text::VorgangAmZielStehtEintrag));
                    return Ablauf::Weiter;
                }
                ziel = neues;
            }
            Klaerung::Ueberspringen => return Ablauf::Weiter,
            Klaerung::Abbrechen => return Ablauf::Abgebrochen,
        }
    }
}

/// Wie viele Zwischennamen das Ueberschreiben versucht, bevor es aufgibt.
const ZWISCHENVERSUCHE: u32 = 16;

/// Der Zwischenname des Ersatzes beim `versuch`-ten Anlauf, im Ordner des
/// Ziels (Modulkopf, „Ueberschreiben“). Er traegt die Prozesskennung und
/// keinen Teil des Zielnamens, damit er nie laenger wird als ein Name, den
/// der Server annimmt.
fn zwischenname(versuch: u32) -> String {
    format!(".krk-ersatz-{}-{versuch}", std::process::id())
}

/// Ob „Ueberschreiben“ das Ziel entfernen darf.
///
/// **Nicht, wenn Quelle und Ziel derselbe Eintrag sind, und nicht, wenn es
/// sich nicht feststellen laesst**: das Entfernen traefe dann die Quelle.
/// Beides steht im Bericht, und das Ziel bleibt. Gefragt wird auf dem
/// Arbeitsfaden, fern ueber `REALPATH` ([`Stelle::derselbe_eintrag`]).
///
/// **Beim Verschieben ueber eine Grenze auch nicht, wenn die lokale Seite auf
/// einem Netzlaufwerk liegt** ([`netzbefund`]): dort beantwortet ein Client
/// mit Namenscache die Frage des Tauschs nach der Quelle womoeglich mit einem
/// Eintrag, den der Server eben umbenannt hat, und das Entfernen der Quelle
/// danach traefe den Ersatz (Modulkopf, „Was dieses Modul nicht haelt“).
fn ersetzen_erlaubt(
    pfand: &Fernfaden,
    quelle: &Quelle,
    ziel: &Stelle,
    zweck: Zweck,
    steuerung: &mut Steuerung,
) -> bool {
    match netzbefund(zweck, || lokale_seite_liegt_lokal(quelle, ziel)) {
        Netzbefund::Erlaubt => {}
        Netzbefund::Netzlaufwerk => {
            steuerung.ueberspringen(
                &quelle.bezeichnung(),
                text(Text::VorgangNetzlaufwerkNichtErsetzt),
            );
            return false;
        }
        Netzbefund::Unklar(fehler) => {
            steuerung.ueberspringen(
                &quelle.bezeichnung(),
                satz(
                    Text::VorgangZielNichtErsetzt,
                    &[("grund", &grund(&Fehlschlag::Lokal(fehler)))],
                ),
            );
            return false;
        }
    }
    match quelle.derselbe_eintrag(pfand, ziel) {
        Gleichheit::Nein => true,
        Gleichheit::Ja => {
            steuerung.ueberspringen(
                &quelle.bezeichnung(),
                text(Text::VorgangQuelleUndZielDerselbeEintrag),
            );
            false
        }
        Gleichheit::Unklar(fehler) => {
            steuerung.ueberspringen(
                &quelle.bezeichnung(),
                satz(Text::VorgangZielNichtErsetzt, &[("grund", &grund(&fehler))]),
            );
            false
        }
    }
}

/// Was das Dateisystem der lokalen Seite fuer ein Ersetzen heisst.
#[derive(Debug)]
enum Netzbefund {
    /// Ersetzt werden darf, soweit es an dieser Frage haengt.
    Erlaubt,
    /// Die lokale Seite liegt auf einem Netzlaufwerk; beim Verschieben ueber
    /// eine Grenze wird nicht ersetzt.
    Netzlaufwerk,
    /// Das Dateisystem liess sich nicht fragen, aus diesem Grund; ersetzt wird
    /// ebenso nicht.
    Unklar(io::Error),
}

/// Die Regel, ob ein Tausch an einem Netzlaufwerk haengen bleibt, ohne
/// Systemaufruf: beim Kopieren nie, denn dort wird danach nichts entfernt,
/// und der Ersatz ist eine vollstaendige Kopie. Beim Verschieben fragt sie
/// `lokale_seite`, und erst dort: `None` heisst, keine Seite liegt auf diesem
/// Geraet, `Some(Ok(nein))` heisst Netzlaufwerk.
fn netzbefund(zweck: Zweck, lokale_seite: impl FnOnce() -> Option<io::Result<bool>>) -> Netzbefund {
    if zweck == Zweck::Kopieren {
        return Netzbefund::Erlaubt;
    }
    match lokale_seite() {
        None | Some(Ok(true)) => Netzbefund::Erlaubt,
        Some(Ok(false)) => Netzbefund::Netzlaufwerk,
        Some(Err(fehler)) => Netzbefund::Unklar(fehler),
    }
}

/// Ob jede lokale Seite von `quelle` und `ziel` auf einem Dateisystem dieses
/// Geraets liegt, ueber `statfs(2)` ([`auf_lokalem_dateisystem`]); `None`,
/// wenn keine lokal ist.
///
/// Gefragt werden der Eintrag und sein Ordner: der Eintrag kann selbst der
/// Einhaengepunkt eines Netzlaufwerks sein, und sein Ordner ist der Ort, an
/// dem `LSTAT` den Namen nachschlaegt. Ein Systemaufruf je Frage, also allein
/// auf dem Arbeitsfaden und allein bei „Ueberschreiben“.
///
/// Eine Quelle im Archiv hat keine lokale Seite in diesem Sinn: entfernt
/// wird sie erst nach dem Vorgang mit einem Umbau ihres Archivs, und nach
/// ihrem Namen fragt der Tausch an der Archivdatei.
fn lokale_seite_liegt_lokal(quelle: &Quelle, ziel: &Stelle) -> Option<io::Result<bool>> {
    let mut gefragt = false;
    for stelle in [quelle.als_stelle(), Some(ziel)].into_iter().flatten() {
        let Stelle::Lokal(pfad) = stelle else {
            continue;
        };
        gefragt = true;
        for frage in [pfad.parent(), Some(pfad.as_path())].into_iter().flatten() {
            match auf_lokalem_dateisystem(frage) {
                Ok(true) => {}
                anderes => return Some(anderes),
            }
        }
    }
    gefragt.then_some(Ok(true))
}

/// Legt den Ersatz fuer `ziel` unter einem freien Zwischennamen daneben an,
/// ausschliessend; ein vergebener Zwischenname fuehrt zum naechsten.
fn zwischen_anlegen(
    pfand: &Fernfaden,
    ziel: &Stelle,
    anlage: &Anlage,
) -> Result<(Stelle, Angelegt), Fehlschlag> {
    let mut letzter = Fehlschlag::KeinEintrag;
    for versuch in 0..ZWISCHENVERSUCHE {
        let zwischen = ziel
            .geschwister(&zwischenname(versuch))
            .ok_or(Fehlschlag::KeinEintrag)?;
        match anlegen(pfand, &zwischen, anlage) {
            Ok(angelegt) => return Ok((zwischen, angelegt)),
            Err(fehler) if vorhandenes_ziel(pfand, &fehler, &zwischen).is_some() => {
                letzter = fehler;
            }
            Err(fehler) => return Err(fehler),
        }
    }
    Err(letzter)
}

/// Ersetzt das vorhandene `ziel` durch eine Kopie der Quelle, ohne dass das
/// Ziel je verschwindet, ohne ersetzt zu werden (Modulkopf, „Ueberschreiben“).
///
/// Die Kopie entsteht vollstaendig unter einem Zwischennamen; dass sie
/// vollstaendig ist, bezeugt wie in [`ueber_die_grenze`], dass seit ihrem
/// Beginn nichts uebersprungen wurde. Ein Hinweis auf Rechte und Zeit zaehlt
/// dabei nicht mit: er geht nicht in den Bericht, sondern wartet ueber
/// [`Hinweisweg::Ersatz`] auf den Tausch. Fehlt etwas, faellt die Kopie weg,
/// und das Ziel bleibt. Erst dann tauscht [`tauschen`], und erst nach dem
/// Tausch ist der Eintrag fertig.
///
/// `weg` ist der Weg des Rufers: ueber ihn gehen die zurueckgehaltenen
/// Hinweise nach dem Tausch, unter dem Namen des Ziels, und der Rest eines
/// alten Ziels, das sich nicht ganz entfernen liess.
///
/// Ob ersetzt werden darf, hat der Rufer schon gefragt ([`ersetzen_erlaubt`]).
fn kopie_ersetzt_ziel(
    pfand: &Fernfaden,
    quelle: &Quelle,
    angaben: &Angaben,
    anlage: &Anlage,
    ziel: &Stelle,
    steuerung: &mut Steuerung,
    weg: &mut Hinweisweg<'_>,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    let (zwischen, angelegt) = match zwischen_anlegen(pfand, ziel, anlage) {
        Ok(paar) => paar,
        Err(fehler) => {
            steuerung.ueberspringen(
                &bezeichnung,
                satz(Text::VorgangZielNichtErsetzt, &[("grund", &grund(&fehler))]),
            );
            return Ablauf::Weiter;
        }
    };
    let stand = steuerung.uebersprungen_stand();
    let mut zurueckgehalten = Vec::new();
    let gefuellt = {
        let mut ersatzweg = Hinweisweg::Ersatz(&mut zurueckgehalten);
        match angelegt {
            Angelegt::Datei(schreibziel) => {
                match datei_uebertragen(pfand, quelle, &zwischen, schreibziel, steuerung) {
                    Ok(bytes) => {
                        angaben_uebernehmen(
                            pfand,
                            &zwischen,
                            angaben,
                            Text::VorgangDateiangabenNichtKopiert,
                            steuerung,
                            &mut ersatzweg,
                        );
                        Ok(bytes)
                    }
                    Err(ablauf) => Err(ablauf),
                }
            }
            Angelegt::Ordner => {
                // Der Ersatz ist ein frisch angelegter Ordner; ein Konflikt
                // darin entsteht allein, wenn der Server zwei Namen der Quelle
                // als einen liest. Dann gilt die strengere Regel des
                // Verschiebens, weil dieser Weg nicht weiss, wozu er kopiert.
                match ordnerinhalt_fuellen(
                    pfand,
                    quelle,
                    &zwischen,
                    Zweck::Verschieben,
                    steuerung,
                    &mut ersatzweg,
                ) {
                    Ok(()) => {
                        angaben_uebernehmen(
                            pfand,
                            &zwischen,
                            angaben,
                            Text::VorgangOrdnerangabenNichtKopiert,
                            steuerung,
                            &mut ersatzweg,
                        );
                        Ok(0)
                    }
                    Err(ablauf) => Err(ablauf),
                }
            }
            Angelegt::Verknuepfung => Ok(0),
        }
    };
    let bytes = match gefuellt {
        Err(Ablauf::Abgebrochen) => {
            rest_wegraeumen(pfand, &zwischen, steuerung);
            return Ablauf::Abgebrochen;
        }
        Ok(bytes) if steuerung.uebersprungen_seit(stand).is_empty() => bytes,
        Ok(_) | Err(Ablauf::Weiter) => {
            rest_wegraeumen(pfand, &zwischen, steuerung);
            steuerung.ueberspringen(&bezeichnung, text(Text::VorgangErsatzUnvollstaendig));
            return Ablauf::Weiter;
        }
    };
    match tauschen(pfand, &zwischen, ziel, Some(quelle)) {
        Tausch::Getauscht => {
            steuerung.eintrag_fertig(&bezeichnung, bytes);
            hinweise_nachtragen(zurueckgehalten, &zwischen, ziel, steuerung, weg);
        }
        Tausch::AltesBlieb { alt, fehler } => {
            steuerung.eintrag_fertig(&bezeichnung, bytes);
            hinweise_nachtragen(zurueckgehalten, &zwischen, ziel, steuerung, weg);
            altes_ziel_melden(ziel, &alt, &fehler, steuerung, weg);
        }
        Tausch::ZielBlieb(fehler) => {
            rest_wegraeumen(pfand, &zwischen, steuerung);
            steuerung.ueberspringen(
                &bezeichnung,
                satz(Text::VorgangZielNichtErsetzt, &[("grund", &grund(&fehler))]),
            );
        }
        Tausch::ZielVielleichtBeiseite { alt, fehler } => {
            rest_wegraeumen(pfand, &zwischen, steuerung);
            steuerung.ueberspringen(
                &bezeichnung,
                satz(Text::VorgangZielNichtErsetzt, &[("grund", &grund(&fehler))]),
            );
            ziel_vielleicht_beiseite_melden(ziel, &alt, &fehler, steuerung);
        }
        Tausch::QuelleBeiseite => {
            rest_wegraeumen(pfand, &zwischen, steuerung);
            steuerung.ueberspringen(&bezeichnung, text(Text::VorgangQuelleIstTeilDesZiels));
        }
        Tausch::BeideBeiseite { alt, fehler } => {
            beide_beiseite_melden(&bezeichnung, &zwischen, &alt, &fehler, steuerung);
        }
    }
    Ablauf::Weiter
}

/// Gibt die zurueckgehaltenen Hinweise eines Ersatzes, der nun am Ziel
/// steht, ueber `weg` weiter, mit dem Namen des Ziels statt des
/// Zwischennamens.
fn hinweise_nachtragen(
    hinweise: Vec<(PathBuf, String)>,
    zwischen: &Stelle,
    ziel: &Stelle,
    steuerung: &mut Steuerung,
    weg: &mut Hinweisweg<'_>,
) {
    let vorher = zwischen.bezeichnung();
    let nachher = ziel.bezeichnung();
    for (pfad, grund) in hinweise {
        let pfad = match pfad.strip_prefix(&vorher) {
            Ok(rest) if rest.as_os_str().is_empty() => nachher.clone(),
            Ok(rest) => nachher.join(rest),
            Err(_) => pfad,
        };
        weg.melden(steuerung, pfad, grund);
    }
}

/// Meldet den Rest eines alten Ziels, der sich nach dem Tausch nicht
/// entfernen liess: unter dem Namen des Ziels, mit dem Zwischennamen, unter
/// dem er liegt. Der Ersatz steht vollstaendig am Ziel; der Rest ist deshalb
/// ein Hinweis und keine Luecke, und er geht ueber `weg`.
fn altes_ziel_melden(
    ziel: &Stelle,
    alt: &Stelle,
    fehler: &Fehlschlag,
    steuerung: &mut Steuerung,
    weg: &mut Hinweisweg<'_>,
) {
    weg.melden(
        steuerung,
        ziel.bezeichnung(),
        satz(
            Text::VorgangAltesZielBlieb,
            &[
                ("name", &alt.bezeichnung().display()),
                ("grund", &grund(fehler)),
            ],
        ),
    );
}

/// Meldet ein Ziel, dessen `RENAME` beiseite ohne Antwort blieb: es liegt
/// unter seinem Namen oder unter `alt`, und welches, weiss nur der Server.
fn ziel_vielleicht_beiseite_melden(
    ziel: &Stelle,
    alt: &Stelle,
    fehler: &Fehlschlag,
    steuerung: &mut Steuerung,
) {
    steuerung.ueberspringen(
        &ziel.bezeichnung(),
        satz(
            Text::VorgangZielVielleichtBeiseite,
            &[
                ("alt", &alt.bezeichnung().display()),
                ("grund", &grund(fehler)),
            ],
        ),
    );
}

/// Meldet einen Tausch, nach dem weder Ziel noch Ersatz unter dem Namen des
/// Ziels steht: beide liegen unter ihren Zwischennamen.
fn beide_beiseite_melden(
    bezeichnung: &Path,
    zwischen: &Stelle,
    alt: &Stelle,
    fehler: &Fehlschlag,
    steuerung: &mut Steuerung,
) {
    steuerung.ueberspringen(
        bezeichnung,
        satz(
            Text::VorgangErsatzUnterZwischennamen,
            &[
                ("alt", &alt.bezeichnung().display()),
                ("name", &zwischen.bezeichnung().display()),
                ("grund", &grund(fehler)),
            ],
        ),
    );
}

/// Wie der Tausch von Ersatz und Ziel ausging (Modulkopf, „Ueberschreiben“).
enum Tausch {
    /// Der Ersatz traegt den Namen des Ziels, und das alte Ziel ist fort.
    Getauscht,
    /// Der Ersatz traegt den Namen des Ziels; vom beiseitegelegten alten Ziel
    /// liess sich nicht alles entfernen, und der Rest liegt unter `alt`.
    AltesBlieb { alt: Stelle, fehler: Fehlschlag },
    /// Das Ziel steht unveraendert unter seinem Namen: es liess sich nicht
    /// beiseitelegen, oder es kam zurueck, weil der Ersatz seinen Namen nicht
    /// bekam oder die Quelle sich nicht fragen liess. Der Ersatz liegt unter
    /// seinem Zwischennamen.
    ZielBlieb(Fehlschlag),
    /// Das `RENAME`, das das Ziel beiseitelegen sollte, blieb ohne Antwort
    /// ([`Fehlschlag::laesst_den_ausgang_offen`]): das Ziel liegt unter seinem
    /// Namen oder unter `alt`. Der Ersatz liegt unter seinem Zwischennamen.
    ZielVielleichtBeiseite { alt: Stelle, fehler: Fehlschlag },
    /// Mit dem Ziel lag auch die Quelle beiseite; das Ziel steht wieder
    /// unveraendert unter seinem Namen, der Ersatz unter seinem Zwischennamen.
    QuelleBeiseite,
    /// Das Ziel liegt unter `alt`, der Ersatz unter seinem Zwischennamen:
    /// zurueckholen liess sich das Ziel nicht, aus dem Grund `fehler`.
    BeideBeiseite { alt: Stelle, fehler: Fehlschlag },
}

/// Der Zwischenname des beiseitegelegten Ziels beim `versuch`-ten Anlauf,
/// gebaut wie [`zwischenname`].
fn altname(versuch: u32) -> String {
    format!(".krk-alt-{}-{versuch}", std::process::id())
}

/// Wie das Beiseitelegen des Ziels ausging.
enum Beiseite {
    /// Das Ziel liegt unter diesem Zwischennamen.
    Gelegt(Stelle),
    /// Das Ziel steht unveraendert unter seinem Namen.
    Nicht(Fehlschlag),
    /// Das `RENAME` blieb ohne Antwort
    /// ([`Fehlschlag::laesst_den_ausgang_offen`]): das Ziel liegt unter
    /// seinem Namen oder unter `alt`.
    Vielleicht { alt: Stelle, fehler: Fehlschlag },
}

/// Legt `ziel` per `RENAME` unter einem freien Zwischennamen daneben
/// beiseite; ein vergebener Zwischenname fuehrt zum naechsten.
fn beiseite_legen(pfand: &Fernfaden, ziel: &Stelle) -> Beiseite {
    let mut letzter = Fehlschlag::KeinEintrag;
    for versuch in 0..ZWISCHENVERSUCHE {
        let name = altname(versuch);
        let Some(alt) = ziel.geschwister(&name) else {
            return Beiseite::Nicht(Fehlschlag::KeinEintrag);
        };
        match ziel.umbenennen_in(pfand, &name) {
            Ok(()) => return Beiseite::Gelegt(alt),
            Err(fehler) if fehler.laesst_den_ausgang_offen() => {
                return Beiseite::Vielleicht { alt, fehler };
            }
            Err(fehler) if vorhandenes_ziel(pfand, &fehler, &alt).is_some() => {
                letzter = fehler;
            }
            Err(fehler) => return Beiseite::Nicht(fehler),
        }
    }
    Beiseite::Nicht(letzter)
}

/// Setzt den vollstaendigen Ersatz `zwischen` an die Stelle von `ziel`, in den
/// drei Schritten des Modulkopfs: Ziel beiseite, Ersatz auf den Namen, altes
/// Ziel entfernen.
///
/// `quelle` ist die Quelle, nach der nach dem ersten Schritt gefragt wird, oder
/// `None`, wenn sie selbst der Ersatz ist (Verschieben auf demselben Server).
fn tauschen(
    pfand: &Fernfaden,
    zwischen: &Stelle,
    ziel: &Stelle,
    quelle: Option<&Quelle>,
) -> Tausch {
    let name = match ziel.name() {
        Ok(name) => name,
        Err(_) => return Tausch::ZielBlieb(Fehlschlag::KeinEintrag),
    };
    let alt = match beiseite_legen(pfand, ziel) {
        Beiseite::Gelegt(alt) => alt,
        Beiseite::Vielleicht { alt, fehler } => {
            return Tausch::ZielVielleichtBeiseite { alt, fehler };
        }
        Beiseite::Nicht(fehler) => return Tausch::ZielBlieb(fehler),
    };
    let zurueckholen = |anlass: Tausch| match alt.umbenennen_in(pfand, &name) {
        Ok(()) => anlass,
        Err(fehler) => Tausch::BeideBeiseite {
            alt: alt.clone(),
            fehler,
        },
    };
    if let Some(quelle) = quelle
        && let Err(fehler) = quelle.angaben(pfand)
    {
        return zurueckholen(if fehler.ist_nicht_da() {
            Tausch::QuelleBeiseite
        } else {
            Tausch::ZielBlieb(fehler)
        });
    }
    if let Err(fehler) = zwischen.umbenennen_in(pfand, &name) {
        return zurueckholen(Tausch::ZielBlieb(fehler));
    }
    // Ohne Abbruchfrage: der Ersatz steht schon am Ziel, und der Rest des
    // alten Ziels gehoert weg, auch wenn der Nutzer jetzt abbricht.
    match alt.baum_entfernen(pfand, &nie_abgebrochen) {
        Ok(()) => Tausch::Getauscht,
        Err(fehler) => Tausch::AltesBlieb { alt, fehler },
    }
}

/// Entfernt einen Ersatz, der nicht an die Stelle des Ziels kommt; scheitert
/// das, steht es im Bericht.
///
/// Ohne Abbruchfrage: gerufen wird es auch nach einem Abbruch, und der Rest
/// soll trotzdem fort.
fn rest_wegraeumen(pfand: &Fernfaden, zwischen: &Stelle, steuerung: &mut Steuerung) {
    if let Err(fehler) = zwischen.baum_entfernen(pfand, &nie_abgebrochen)
        && !fehler.ist_nicht_da()
    {
        steuerung.ueberspringen(
            &zwischen.bezeichnung(),
            satz(
                Text::VorgangErsatzNichtWeggeraeumt,
                &[("grund", &grund(&fehler))],
            ),
        );
    }
}

/// Legt am Ziel an, ausschliessend: ein vorhandener Name laesst es scheitern.
fn anlegen(pfand: &Fernfaden, ziel: &Stelle, anlage: &Anlage) -> Result<Angelegt, Fehlschlag> {
    match anlage {
        Anlage::Datei => ziel.datei_anlegen(pfand).map(Angelegt::Datei),
        Anlage::Ordner => ziel.ordner_anlegen(pfand).map(|()| Angelegt::Ordner),
        Anlage::Verknuepfung(verweis) => ziel
            .verknuepfung_anlegen(pfand, verweis)
            .map(|()| Angelegt::Verknuepfung),
    }
}

/// Fragt, was mit einem vorhandenen Ziel geschieht.
///
/// Dieselben vier Antworten wie lokal in `super::ziel_klaeren`.
/// „Ueberschreiben“ entfernt hier noch nichts: der Rufer legt erst den
/// Ersatz unter einem Zwischennamen an und entfernt das Ziel, bei einem
/// Ordner samt Inhalt, erst danach endgueltig (Modulkopf); das Konfliktblatt
/// sagt das Endgueltige schon heute (`operationen::ersetzungsweg` in der
/// Oberflaeche).
fn konflikt_klaeren(
    pfand: &Fernfaden,
    quelle: &Path,
    ziel: &Stelle,
    steuerung: &mut Steuerung,
) -> Klaerung {
    let ziel_bezeichnung = ziel.bezeichnung();
    let antwort =
        steuerung.konflikt_loesen_mit(quelle, &ziel_bezeichnung, || ziel.freier_name(pfand));
    match antwort {
        super::Konfliktantwort::Ueberschreiben => Klaerung::Ersetzen,
        super::Konfliktantwort::Ueberspringen => {
            steuerung.ueberspringen(quelle, text(Text::VorgangAmZielStehtEintrag));
            Klaerung::Ueberspringen
        }
        super::Konfliktantwort::UmbenennenIn(name) => match name_pruefen(&name) {
            Ok(()) => match ziel.geschwister(&name) {
                Some(neues) => Klaerung::Nach(neues),
                None => {
                    steuerung.ueberspringen(quelle, text(Text::VorgangFernNameOhneUtf8));
                    Klaerung::Ueberspringen
                }
            },
            Err(fehler) => {
                steuerung.ueberspringen(quelle, fehler.grund());
                Klaerung::Ueberspringen
            }
        },
        super::Konfliktantwort::Abbrechen => Klaerung::Abbrechen,
    }
}

/// Uebertraegt den Inhalt einer Datei in das eben angelegte Ziel, dann Rechte
/// und Aenderungszeit, und verbucht den Eintrag als fertig.
fn datei_fuellen(
    pfand: &Fernfaden,
    quelle: &Quelle,
    angaben: &Angaben,
    ziel: &Stelle,
    schreibziel: Schreibziel,
    steuerung: &mut Steuerung,
    weg: &mut Hinweisweg<'_>,
) -> Ablauf {
    let bytes = match datei_uebertragen(pfand, quelle, ziel, schreibziel, steuerung) {
        Ok(bytes) => bytes,
        Err(ablauf) => return ablauf,
    };
    angaben_uebernehmen(
        pfand,
        ziel,
        angaben,
        Text::VorgangDateiangabenNichtKopiert,
        steuerung,
        weg,
    );
    steuerung.eintrag_fertig(&quelle.bezeichnung(), bytes);
    Ablauf::Weiter
}

/// Uebertraegt den Inhalt einer Datei in das eben angelegte Ziel und gibt die
/// Zahl der Bytes zurueck; `Err` traegt, wie der Rufer endet, und was
/// scheiterte, steht schon im Bericht.
///
/// **Ein abgebrochenes oder gescheitertes Ziel wird entfernt**: die halbe
/// Datei ist kein Ergebnis, sondern ein Rest, wie in
/// `kopieren::datei_uebertragen`. Das Ziel hat dieser Lauf selbst angelegt,
/// ausschliessend; entfernt wird also nie ein Eintrag, der vorher dastand.
fn datei_uebertragen(
    pfand: &Fernfaden,
    quelle: &Quelle,
    ziel: &Stelle,
    schreibziel: Schreibziel,
    steuerung: &mut Steuerung,
) -> Result<u64, Ablauf> {
    let bezeichnung = quelle.bezeichnung();
    let mut schreibziel = schreibziel;
    let mut bytes: u64 = 0;
    let mut schreibfehler = None;
    let gelesen = quelle.lesen(pfand, |daten| {
        if let Err(fehler) = schreibziel.schreiben(pfand, daten) {
            schreibfehler = Some(fehler);
            return ControlFlow::Break(());
        }
        bytes += daten.len() as u64;
        steuerung.zwischenstand(&bezeichnung, bytes);
        if steuerung.abgebrochen() {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    let geschlossen = schreibziel.schliessen(pfand);
    let ausgang = match (gelesen, schreibfehler) {
        (_, Some(fehler)) | (Err(fehler), None) => Err(fehler),
        (Ok(_), None) => geschlossen,
    };

    match ausgang {
        Ok(()) => Ok(bytes),
        Err(Fehlschlag::Abgebrochen) => {
            steuerung.teilstueck(bytes);
            halbes_ziel_wegraeumen(pfand, ziel, steuerung);
            Err(Ablauf::Abgebrochen)
        }
        Err(fehler) => {
            halbes_ziel_wegraeumen(pfand, ziel, steuerung);
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            Err(Ablauf::Weiter)
        }
    }
}

/// Entfernt die halbe Zieldatei; scheitert das, steht es im Bericht.
fn halbes_ziel_wegraeumen(pfand: &Fernfaden, ziel: &Stelle, steuerung: &mut Steuerung) {
    if let Err(fehler) = ziel.datei_entfernen(pfand)
        && !fehler.ist_nicht_da()
    {
        steuerung.ueberspringen(
            &ziel.bezeichnung(),
            satz(
                Text::VorgangHalbeDateiNichtWeggeraeumt,
                &[("grund", &grund(&fehler))],
            ),
        );
    }
}

/// Kopiert den Inhalt eines Ordners in einen Ordner am Ziel, der eben
/// angelegt wurde oder schon dastand, dann Rechte und Aenderungszeit, und
/// verbucht den Ordner als fertig.
fn ordner_fuellen(
    pfand: &Fernfaden,
    quelle: &Quelle,
    angaben: &Angaben,
    ziel: &Stelle,
    zweck: Zweck,
    steuerung: &mut Steuerung,
    weg: &mut Hinweisweg<'_>,
) -> Ablauf {
    if let Err(ablauf) = ordnerinhalt_fuellen(pfand, quelle, ziel, zweck, steuerung, weg) {
        return ablauf;
    }
    angaben_uebernehmen(
        pfand,
        ziel,
        angaben,
        Text::VorgangOrdnerangabenNichtKopiert,
        steuerung,
        weg,
    );
    steuerung.eintrag_fertig(&quelle.bezeichnung(), 0);
    Ablauf::Weiter
}

/// Kopiert die Kinder des Ordners `quelle` in den Ordner `ziel`, jedes samt
/// Konfliktbehandlung. `Err` traegt, wie der Rufer endet: der Ordner liess
/// sich nicht lesen (steht im Bericht), oder der Nutzer brach ab.
fn ordnerinhalt_fuellen(
    pfand: &Fernfaden,
    quelle: &Quelle,
    ziel: &Stelle,
    zweck: Zweck,
    steuerung: &mut Steuerung,
    weg: &mut Hinweisweg<'_>,
) -> Result<(), Ablauf> {
    let kinder = quellkinder_lesen(pfand, quelle, steuerung)?;
    for (unterquelle, unterangaben) in kinder {
        if steuerung.abgebrochen() {
            return Err(Ablauf::Abgebrochen);
        }
        let Some(unterziel) = unterquelle.name().ok().and_then(|name| ziel.kind(&name)) else {
            steuerung.ueberspringen(
                &unterquelle.bezeichnung(),
                text(Text::VorgangFernNameOhneUtf8),
            );
            continue;
        };
        let Some(unterangaben) = unterangaben_holen(
            unterangaben,
            &unterquelle.bezeichnung(),
            || unterquelle.angaben(pfand),
            steuerung,
        ) else {
            continue;
        };
        if eintrag_kopieren(
            pfand,
            &unterquelle,
            &unterangaben,
            unterziel,
            zweck,
            steuerung,
            weg,
        ) == Ablauf::Abgebrochen
        {
            return Err(Ablauf::Abgebrochen);
        }
    }
    Ok(())
}

/// Die Kinder eines Ordners als Stellen, mit den Angaben, die das Lesen schon
/// mitbrachte. Namen ohne UTF-8 und Namen, die die Namensregel abweist, kommen
/// gezaehlt in den Bericht, weil sich kein Pfad auf dem Server zu ihnen bauen
/// laesst.
///
/// `Err` traegt, wie der Rufer endet: [`Ablauf::Weiter`], wenn der Ordner sich
/// nicht lesen liess, und das steht im Bericht; [`Ablauf::Abgebrochen`], wenn
/// der Nutzer waehrend des Lesens abbrach. **Der Abbruch muss als Abbruch
/// zurueck**: ein Verschieben ueber eine Grenze entfernt seine Quelle, sobald
/// seit ihrem Beginn nichts uebersprungen ist, und ein still beendeter
/// Ordner liesse sie mit halb kopiertem Inhalt entfernen.
fn kinder_lesen(
    pfand: &Fernfaden,
    quelle: &Stelle,
    steuerung: &mut Steuerung,
) -> Result<Vec<(Stelle, Option<Angaben>)>, Ablauf> {
    let bezeichnung = quelle.bezeichnung();
    let liste = match quelle.liste(pfand, &|| steuerung.abgebrochen()) {
        Ok(liste) => liste,
        Err(Fehlschlag::Abgebrochen) => return Err(Ablauf::Abgebrochen),
        Err(fehler) => {
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return Err(Ablauf::Weiter);
        }
    };
    if liste.unansteuerbar > 0 {
        steuerung.ueberspringen(
            &bezeichnung,
            anzahl(Zahlwort::FernNamenUebersprungen, liste.unansteuerbar, &[]),
        );
    }
    let mut kinder = Vec::with_capacity(liste.kinder.len());
    for (name, angaben) in liste.kinder {
        match quelle.kind(&name) {
            Some(kind) => kinder.push((kind, angaben)),
            None => steuerung.ueberspringen(
                &bezeichnung.join(&name),
                text(Text::VorgangPfadBenenntKeinenEintrag),
            ),
        }
    }
    Ok(kinder)
}

/// Die Angaben eines Kindes: die mitgebrachten, sonst ein eigenes `LSTAT`
/// ueber `holen`; scheitert das, steht es unter `bezeichnung` im Bericht.
fn unterangaben_holen(
    mitgebracht: Option<Angaben>,
    bezeichnung: &Path,
    holen: impl FnOnce() -> Result<Angaben, Fehlschlag>,
    steuerung: &mut Steuerung,
) -> Option<Angaben> {
    if let Some(angaben) = mitgebracht {
        return Some(angaben);
    }
    match holen() {
        Ok(angaben) => Some(angaben),
        Err(fehler) => {
            steuerung.ueberspringen(bezeichnung, grund(&fehler));
            None
        }
    }
}

/// Die Kinder einer Lesequelle, wie [`kinder_lesen`] sie fuer eine Stelle
/// liefert. Im Archiv kommen sie aus dem Inhaltsverzeichnis, mit ihren
/// Angaben und, in einem tar-Strom, in der Folge ihrer Fundstelle
/// ([`Archivleser::lesefolge`]): ein Ordnerbaum kostet so einen Durchlauf.
fn quellkinder_lesen(
    pfand: &Fernfaden,
    quelle: &Quelle,
    steuerung: &mut Steuerung,
) -> Result<Vec<(Quelle, Option<Angaben>)>, Ablauf> {
    match quelle {
        Quelle::Stelle(stelle) => Ok(kinder_lesen(pfand, stelle, steuerung)?
            .into_iter()
            .map(|(kind, angaben)| (Quelle::Stelle(kind), angaben))
            .collect()),
        Quelle::Archiv(stelle) => match stelle.kinder() {
            Some(kinder) => Ok(kinder),
            None => {
                steuerung.ueberspringen(&stelle.bezeichnung(), text(Text::ArchivEintragFehlt));
                Err(Ablauf::Weiter)
            }
        },
    }
}

// ----------------------------------------------------------------------
// Verschieben
// ----------------------------------------------------------------------

/// Verschiebt einen Eintrag an sein Ziel, samt Konfliktbehandlung.
///
/// **Auf demselben Server ist es ein `RENAME`** und nicht
/// `posix-rename@openssh.com` (Entscheidung 10); `RENAME` scheitert an einem
/// vorhandenen Ziel, und dann geht dieselbe Konfliktfrage wie beim Kopieren,
/// gestellt nach derselben Regel ([`vorhandenes_ziel`]). „Ueberschreiben“ geht
/// an [`umbenennen_ersetzt_ziel`].
/// Ordner auf Ordner wird Kind fuer Kind zusammengefuehrt. **Ueber eine
/// Grenze**, zwischen Geraet und Server oder zwischen zwei Servern, wird
/// kopiert und danach die Quelle entfernt, allein wenn nichts uebersprungen
/// wurde: die Regel von `verschieben::ueber_datentraeger`.
///
/// **Eine Grenze kann auch auf dem Server liegen**: zwischen zwei
/// Dateisystemen dort scheitert `RENAME` an `EXDEV`, und SFTP v3 meldet das
/// als `FAILURE`. Steht am Ziel dann nichts, ist es kein Konflikt, und der Weg
/// ist derselbe wie ueber eine Grenze zwischen Servern
/// ([`Fehlschlag::kann_eine_grenze_des_servers_heissen`]), wie lokal
/// `verschieben` bei `EXDEV` auf `ueber_datentraeger` ausweicht.
/// Jedes andere Scheitern, etwa „keine Rechte“, bleibt ein Fehler des
/// Eintrags: ein Kopieren daneben haette dort eine Kopie hinterlassen, wo der
/// Nutzer ein Verschieben verlangt hat.
fn eintrag_verschieben(
    pfand: &Fernfaden,
    quelle: &Stelle,
    angaben: &Angaben,
    ziel: Stelle,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    let mut ziel = ziel;
    loop {
        let fehler = match quelle.umbenennen(pfand, &ziel) {
            None => return ueber_die_grenze(pfand, quelle, angaben, ziel, steuerung),
            Some(Ok(())) => {
                steuerung.eintrag_fertig(&bezeichnung, angaben.groesse);
                return Ablauf::Weiter;
            }
            Some(Err(fehler)) => fehler,
        };
        let Some(vorhanden) = vorhandenes_ziel(pfand, &fehler, &ziel) else {
            if fehler.kann_eine_grenze_des_servers_heissen() {
                return ueber_die_grenze(pfand, quelle, angaben, ziel, steuerung);
            }
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return Ablauf::Weiter;
        };
        if angaben.typ == Typ::Ordner && vorhanden.typ == Typ::Ordner {
            return verschmelzen(pfand, quelle, &ziel, steuerung);
        }
        match konflikt_klaeren(pfand, &bezeichnung, &ziel, steuerung) {
            Klaerung::Ersetzen => {
                return umbenennen_ersetzt_ziel(pfand, quelle, angaben, &ziel, steuerung);
            }
            Klaerung::Nach(neues) => {
                if neues.bezeichnung() == ziel.bezeichnung() {
                    steuerung.ueberspringen(&bezeichnung, text(Text::VorgangAmZielStehtEintrag));
                    return Ablauf::Weiter;
                }
                ziel = neues;
            }
            Klaerung::Ueberspringen => return Ablauf::Weiter,
            Klaerung::Abbrechen => return Ablauf::Abgebrochen,
        }
    }
}

/// Ersetzt das vorhandene `ziel` auf demselben Server durch die Quelle, ohne
/// dass das Ziel je verschwindet, ohne ersetzt zu werden (Modulkopf,
/// „Ueberschreiben“).
///
/// Die Quelle geht zuerst per `RENAME` unter einen Zwischennamen neben das
/// Ziel; scheitert das, ist nichts geschehen. Danach tauscht [`tauschen`];
/// steht das Ziel danach unveraendert, weil es sich nicht beiseitelegen liess
/// oder zurueckkam, kehrt die Quelle an ihren Platz zurueck.
fn umbenennen_ersetzt_ziel(
    pfand: &Fernfaden,
    quelle: &Stelle,
    angaben: &Angaben,
    ziel: &Stelle,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    if !ersetzen_erlaubt(
        pfand,
        &Quelle::Stelle(quelle.clone()),
        ziel,
        Zweck::Verschieben,
        steuerung,
    ) {
        return Ablauf::Weiter;
    }
    let mut zwischen = None;
    let mut vielleicht = None;
    let mut letzter = Fehlschlag::KeinEintrag;
    for versuch in 0..ZWISCHENVERSUCHE {
        let Some(kandidat) = ziel.geschwister(&zwischenname(versuch)) else {
            break;
        };
        match quelle.umbenennen(pfand, &kandidat) {
            Some(Ok(())) => {
                zwischen = Some(kandidat);
                break;
            }
            // Ohne Antwort kann die Quelle schon unter dem Zwischennamen
            // liegen; der Bericht nennt ihn.
            Some(Err(fehler)) if fehler.laesst_den_ausgang_offen() => {
                vielleicht = Some(kandidat);
                letzter = fehler;
                break;
            }
            Some(Err(fehler)) if vorhandenes_ziel(pfand, &fehler, &kandidat).is_some() => {
                letzter = fehler;
            }
            Some(Err(fehler)) => {
                letzter = fehler;
                break;
            }
            // Zwei Stellen auf verschiedenen Servern kommen nie hierher; der
            // Fall ist behandelt, weil die Unterscheidung vollstaendig ist.
            None => break,
        }
    }
    let Some(zwischen) = zwischen else {
        let grund_text = match vielleicht {
            Some(kandidat) => satz(
                Text::VorgangQuelleVielleichtUnterZwischennamen,
                &[
                    ("name", &kandidat.bezeichnung().display()),
                    ("grund", &grund(&letzter)),
                ],
            ),
            None => satz(
                Text::VorgangZielNichtErsetzt,
                &[("grund", &grund(&letzter))],
            ),
        };
        steuerung.ueberspringen(&bezeichnung, grund_text);
        return Ablauf::Weiter;
    };
    // Die Quelle ist hier selbst der Ersatz; nach ihr zu fragen hiesse,
    // nach dem Zwischennamen zu fragen, den der Tausch eben vergibt.
    let (fehler, gruende) = match tauschen(pfand, &zwischen, ziel, None) {
        Tausch::Getauscht => {
            steuerung.eintrag_fertig(&bezeichnung, angaben.groesse);
            return Ablauf::Weiter;
        }
        Tausch::AltesBlieb { alt, fehler } => {
            steuerung.eintrag_fertig(&bezeichnung, angaben.groesse);
            altes_ziel_melden(ziel, &alt, &fehler, steuerung, &mut Hinweisweg::bericht());
            return Ablauf::Weiter;
        }
        Tausch::BeideBeiseite { alt, fehler } => {
            beide_beiseite_melden(&bezeichnung, &zwischen, &alt, &fehler, steuerung);
            return Ablauf::Weiter;
        }
        Tausch::ZielBlieb(fehler) => {
            let zurueck = satz(Text::VorgangZielNichtErsetzt, &[("grund", &grund(&fehler))]);
            (fehler, zurueck)
        }
        Tausch::ZielVielleichtBeiseite { alt, fehler } => {
            ziel_vielleicht_beiseite_melden(ziel, &alt, &fehler, steuerung);
            let zurueck = satz(Text::VorgangZielNichtErsetzt, &[("grund", &grund(&fehler))]);
            (fehler, zurueck)
        }
        Tausch::QuelleBeiseite => (
            Fehlschlag::KeinEintrag,
            text(Text::VorgangQuelleIstTeilDesZiels).to_owned(),
        ),
    };
    // Das Ziel steht unveraendert, oder ob es beiseite liegt, ist offen; die
    // Quelle kehrt an ihren Platz zurueck.
    match zwischen.umbenennen(pfand, quelle) {
        Some(Ok(())) => steuerung.ueberspringen(&bezeichnung, gruende),
        Some(Err(zurueck)) if zurueck.laesst_den_ausgang_offen() => steuerung.ueberspringen(
            &bezeichnung,
            satz(
                Text::VorgangQuelleVielleichtUnterZwischennamen,
                &[
                    ("name", &zwischen.bezeichnung().display()),
                    ("grund", &grund(&zurueck)),
                ],
            ),
        ),
        Some(Err(zurueck)) => steuerung.ueberspringen(
            &bezeichnung,
            satz(
                Text::VorgangQuelleUnterZwischennamen,
                &[
                    ("name", &zwischen.bezeichnung().display()),
                    ("grund", &grund(&zurueck)),
                ],
            ),
        ),
        None => steuerung.ueberspringen(
            &bezeichnung,
            satz(
                Text::VorgangQuelleUnterZwischennamen,
                &[
                    ("name", &zwischen.bezeichnung().display()),
                    ("grund", &grund(&fehler)),
                ],
            ),
        ),
    }
    Ablauf::Weiter
}

/// Verschiebt den Inhalt eines Ordners in einen gleichnamigen am Ziel auf
/// demselben Server, Kind fuer Kind, und entfernt danach den leeren
/// Quellordner.
fn verschmelzen(
    pfand: &Fernfaden,
    quelle: &Stelle,
    ziel: &Stelle,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    let kinder = match kinder_lesen(pfand, quelle, steuerung) {
        Ok(kinder) => kinder,
        Err(ablauf) => return ablauf,
    };
    for (unterquelle, unterangaben) in kinder {
        if steuerung.abgebrochen() {
            return Ablauf::Abgebrochen;
        }
        let Some(unterziel) = unterquelle.name().ok().and_then(|name| ziel.kind(&name)) else {
            steuerung.ueberspringen(
                &unterquelle.bezeichnung(),
                text(Text::VorgangFernNameOhneUtf8),
            );
            continue;
        };
        let Some(unterangaben) = unterangaben_holen(
            unterangaben,
            &unterquelle.bezeichnung(),
            || unterquelle.angaben(pfand),
            steuerung,
        ) else {
            continue;
        };
        if eintrag_verschieben(pfand, &unterquelle, &unterangaben, unterziel, steuerung)
            == Ablauf::Abgebrochen
        {
            return Ablauf::Abgebrochen;
        }
    }
    // Ist der Ordner nicht leer, ist ein Kind uebersprungen worden, und dann
    // gehoert er dem Nutzer weiter.
    match quelle.ordner_entfernen(pfand) {
        Ok(()) => steuerung.eintrag_fertig(&bezeichnung, 0),
        Err(fehler) => steuerung.ueberspringen(
            &bezeichnung,
            satz(
                Text::VorgangOrdnerSelbstBlieb,
                &[("grund", &grund(&fehler))],
            ),
        ),
    }
    Ablauf::Weiter
}

/// Verschiebt ueber eine Grenze: kopieren, dann die Quelle entfernen, allein
/// wenn seit dem Beginn dieses Eintrags keine Luecke in den Bericht kam.
///
/// Der Zeuge ist derselbe wie in `verschieben::ueber_datentraeger`: jeder Weg
/// in diesem Modul, der ohne Ankunft am Ziel endet, ruft
/// `Steuerung::ueberspringen`. **Nicht jeder Eintrag seither ist eine
/// Luecke**: ein Hinweis zu einem Eintrag, der vollstaendig angekommen ist,
/// geht ueber denselben Bericht ([`Hinweisweg`]), und der Weg zaehlt ihn. Eine
/// Luecke gibt es, sobald seither mehr Eintraege im Bericht stehen, als der
/// Weg Hinweise gegeben hat; wer einen Eintrag ohne den Weg meldet, meldet
/// damit eine Luecke, und die Quelle bleibt.
fn ueber_die_grenze(
    pfand: &Fernfaden,
    quelle: &Stelle,
    angaben: &Angaben,
    ziel: Stelle,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    let stand = steuerung.uebersprungen_stand();
    let mut weg = Hinweisweg::bericht();
    if eintrag_kopieren(
        pfand,
        &Quelle::Stelle(quelle.clone()),
        angaben,
        ziel,
        Zweck::Verschieben,
        steuerung,
        &mut weg,
    ) == Ablauf::Abgebrochen
    {
        return Ablauf::Abgebrochen;
    }
    let seither = steuerung.uebersprungen_seit(stand);
    if seither.len() != weg.gemeldet() {
        let selbst_genannt = seither.iter().any(|eintrag| eintrag.pfad == bezeichnung);
        if !selbst_genannt {
            steuerung.ueberspringen(&bezeichnung, text(Text::VorgangNichtVollstaendigKopiert));
        }
        return Ablauf::Weiter;
    }
    // Ohne Abbruchfrage: die Kopie ist vollstaendig, und eine halb entfernte
    // Quelle haelt niemand mehr fuer verschoben oder fuer unberuehrt.
    if let Err(fehler) = quelle.baum_entfernen(pfand, &nie_abgebrochen) {
        steuerung.ueberspringen(
            &bezeichnung,
            satz(
                Text::VorgangKopiertAberInQuelleGeblieben,
                &[("grund", &grund(&fehler))],
            ),
        );
    }
    Ablauf::Weiter
}

// ----------------------------------------------------------------------
// Stelle, Angaben, Schreibziel
// ----------------------------------------------------------------------

/// Warum ein Schritt an einem Eintrag scheiterte.
enum Fehlschlag {
    /// Ein Systemaufruf auf diesem Geraet.
    Lokal(io::Error),
    /// Eine Anfrage an den Server.
    Fern(Fernfehler),
    /// Ein Name oder Verweis, der kein UTF-8 ist, soll auf einen Server.
    NameOhneUtf8,
    /// Am Pfad steht keine gewoehnliche Datei mehr.
    KeineGewoehnlicheDatei,
    /// Der Server legt keine Verknuepfung an.
    KeineVerknuepfung,
    /// Der Pfad benennt keinen Eintrag, etwa die Wurzel des Servers.
    KeinEintrag,
    /// Ein ferner Ordner liefert mehr Namen als diese Grenze
    /// ([`HOECHSTENS_NAMEN`]).
    ZuVieleNamen(usize),
    /// Der Nutzer hat abgebrochen.
    Abgebrochen,
    /// Ein Eintrag eines Archivs liess sich nicht lesen.
    Archiv(Lesefehler),
}

impl Fehlschlag {
    /// Der Fehlschlag zu einem Lesefehler im Archiv; ein Abbruch bleibt ein
    /// Abbruch, damit die Wege dieses Moduls ihn als solchen sehen.
    fn aus_archiv(fehler: Lesefehler) -> Self {
        match fehler {
            Lesefehler::Abgebrochen => Fehlschlag::Abgebrochen,
            andere => Fehlschlag::Archiv(andere),
        }
    }

    /// Ob der Eintrag fehlt, an dem gefragt wurde.
    fn ist_nicht_da(&self) -> bool {
        match self {
            Fehlschlag::Lokal(fehler) => fehler.kind() == io::ErrorKind::NotFound,
            Fehlschlag::Fern(fehler) => fehler.ist_status(status::NO_SUCH_FILE),
            Fehlschlag::Archiv(fehler) => fehler.ist_nicht_da(),
            Fehlschlag::NameOhneUtf8
            | Fehlschlag::KeineGewoehnlicheDatei
            | Fehlschlag::KeineVerknuepfung
            | Fehlschlag::KeinEintrag
            | Fehlschlag::ZuVieleNamen(_)
            | Fehlschlag::Abgebrochen => false,
        }
    }

    /// Ob das Scheitern beim Anlegen oder Umbenennen ein vergebener Name sein
    /// kann: lokal allein `AlreadyExists`, fern jeder Statuscode ausser „gibt
    /// es nicht“ und „keine Rechte“. SFTP v3 kennt keinen Code fuer
    /// „vorhanden“; die Regel schliesst aus, was sicher etwas anderes heisst,
    /// und `FAILURE` bleibt mehrdeutig. Eine Trennung, ein Protokollfehler und
    /// jeder Grund ohne Systemaufruf sind nie ein vergebener Name.
    fn kann_vergeben_heissen(&self) -> bool {
        match self {
            Fehlschlag::Lokal(fehler) => fehler.kind() == io::ErrorKind::AlreadyExists,
            Fehlschlag::Fern(Fernfehler::Status { code, .. }) => {
                *code != status::NO_SUCH_FILE && *code != status::PERMISSION_DENIED
            }
            Fehlschlag::Fern(
                Fernfehler::Getrennt(_)
                | Fernfehler::Protokoll(_)
                | Fernfehler::NichtAngeboten(_)
                | Fernfehler::Abgebrochen,
            )
            | Fehlschlag::NameOhneUtf8
            | Fehlschlag::KeineGewoehnlicheDatei
            | Fehlschlag::KeineVerknuepfung
            | Fehlschlag::KeinEintrag
            | Fehlschlag::ZuVieleNamen(_)
            | Fehlschlag::Abgebrochen
            | Fehlschlag::Archiv(_) => false,
        }
    }

    /// Ob eine Anfrage, die so scheiterte, beim Server trotzdem geschehen sein
    /// kann: allein bei einer Trennung, denn dann kann die Anfrage den Server
    /// erreicht haben, und ihre Antwort ging mit der Verbindung verloren. Fuer
    /// ein `RENAME` des Ueberschreibens heisst das: der Eintrag liegt unter
    /// seinem alten oder unter seinem neuen Namen, und der Bericht nennt beide
    /// (Modulkopf, „Ueberschreiben“).
    fn laesst_den_ausgang_offen(&self) -> bool {
        matches!(self, Fehlschlag::Fern(Fernfehler::Getrennt(_)))
    }

    /// Ob ein gescheitertes `RENAME` auf demselben Server eine Grenze
    /// zwischen zwei Dateisystemen dort sein kann: allein `FAILURE`, denn
    /// OpenSSH meldet `EXDEV` so, und SFTP v3 kennt keinen eigenen Code dafuer.
    ///
    /// Gefragt wird sie allein, nachdem [`vorhandenes_ziel`] nein gesagt hat;
    /// ein `FAILURE` an einem vergebenen Namen ist ein Konflikt und kommt
    /// nicht hierher. Ist der Grund ein anderer, scheitert das Kopieren
    /// danach mit seinem eigenen Grund, und die Quelle bleibt, weil
    /// [`ueber_die_grenze`] sie allein ohne Uebersprungenes entfernt.
    fn kann_eine_grenze_des_servers_heissen(&self) -> bool {
        matches!(
            self,
            Fehlschlag::Fern(Fernfehler::Status {
                code: status::FAILURE,
                ..
            })
        )
    }
}

/// Die eine Regel, ob ein Scheitern an `ziel` ein vergebener Name ist: das
/// Scheitern kann es heissen ([`Fehlschlag::kann_vergeben_heissen`]), und
/// `LSTAT` findet am Ziel danach einen Eintrag. Dann die Angaben dieses
/// Eintrags, sonst `None`, und der Fehler gehoert dem Eintrag selbst.
///
/// Gefragt wird sie beim Kopieren, beim Verschieben, beim Umbenennen und
/// Anlegen ([`namensgrund`]) und fuer die Zwischennamen des Ueberschreibens;
/// eine zweite Antwort auf dieselbe Frage gibt es in diesem Modul nicht.
fn vorhandenes_ziel(pfand: &Fernfaden, fehler: &Fehlschlag, ziel: &Stelle) -> Option<Angaben> {
    if !fehler.kann_vergeben_heissen() {
        return None;
    }
    ziel.angaben(pfand).ok()
}

/// Ob zwei Stellen denselben Eintrag benennen.
enum Gleichheit {
    Ja,
    Nein,
    /// Es liess sich nicht feststellen, aus diesem Grund.
    Unklar(Fehlschlag),
}

/// Der Grund, den die Abschlussliste zeigt.
///
/// Lokal der von `super::grund`. Fern die zwei haeufigen Statuscodes und die
/// Trennung aus der Sprachtabelle, sonst der Wortlaut des Servers oder des
/// Gespraechs, wie lokal der des Systems.
fn grund(fehler: &Fehlschlag) -> String {
    match fehler {
        Fehlschlag::Lokal(fehler) => super::grund(fehler),
        Fehlschlag::Fern(fehler) => fern_grund(fehler),
        Fehlschlag::NameOhneUtf8 => text(Text::VorgangFernNameOhneUtf8).to_owned(),
        Fehlschlag::KeineGewoehnlicheDatei => text(Text::VorgangKeineGewoehnlicheDatei).to_owned(),
        Fehlschlag::KeineVerknuepfung => text(Text::VorgangFernKeineVerknuepfung).to_owned(),
        Fehlschlag::KeinEintrag => text(Text::VorgangPfadBenenntKeinenEintrag).to_owned(),
        Fehlschlag::ZuVieleNamen(grenze) => {
            satz(Text::VorgangFernOrdnerZuVieleNamen, &[("zahl", grenze)])
        }
        // Ein Abbruch endet den Vorgang und kommt nicht in die Abschlussliste;
        // der Zweig steht, weil die Unterscheidung vollstaendig ist.
        Fehlschlag::Abgebrochen => Fernfehler::Abgebrochen.to_string(),
        Fehlschlag::Archiv(fehler) => fehler.grund(),
    }
}

/// Der Grund zu einer Antwort des Servers oder des Gespraechs.
fn fern_grund(fehler: &Fernfehler) -> String {
    match fehler {
        Fernfehler::Status {
            code: status::NO_SUCH_FILE,
            ..
        } => text(Text::VorgangGibtEsNichtMehr).to_owned(),
        Fernfehler::Status {
            code: status::PERMISSION_DENIED,
            ..
        } => text(Text::VorgangKeineRechte).to_owned(),
        Fernfehler::Status { meldung, .. } if !meldung.is_empty() => meldung.clone(),
        Fernfehler::Getrennt(_) => text(Text::VorgangFernGetrennt).to_owned(),
        Fernfehler::Status { .. }
        | Fernfehler::Protokoll(_)
        | Fernfehler::NichtAngeboten(_)
        | Fernfehler::Abgebrochen => fehler.to_string(),
    }
}

/// Was ueber einen Eintrag bekannt ist, ohne einer Verknuepfung zu folgen.
#[derive(Clone, Copy)]
struct Angaben {
    typ: Typ,
    /// Ob er eine gewoehnliche Datei ist; ohne Typangabe vom Server: ja.
    gewoehnlich: bool,
    /// Die Groesse, Ordner und Verknuepfungen 0.
    groesse: u64,
    /// Die Rechte ohne Typbits.
    rechte: Option<u32>,
    geaendert: Option<SystemTime>,
    zugriff: Option<SystemTime>,
}

impl Angaben {
    fn aus_metadata(angaben: &fs::Metadata) -> Self {
        let typ = if angaben.is_symlink() {
            Typ::Verknuepfung
        } else if angaben.is_dir() {
            Typ::Ordner
        } else {
            Typ::Datei
        };
        Angaben {
            typ,
            gewoehnlich: angaben.file_type().is_file(),
            groesse: if typ == Typ::Datei { angaben.len() } else { 0 },
            rechte: Some(angaben.mode() & 0o7777),
            geaendert: angaben.modified().ok(),
            zugriff: angaben.accessed().ok(),
        }
    }

    fn aus_attributen(attribute: &Attribute) -> Self {
        let art = attribute.rechte.map(|rechte| rechte & S_IFMT);
        let typ = match art {
            Some(S_IFDIR) => Typ::Ordner,
            Some(S_IFLNK) => Typ::Verknuepfung,
            Some(_) | None => Typ::Datei,
        };
        Angaben {
            typ,
            gewoehnlich: art.is_none_or(|art| art == S_IFREG),
            groesse: if typ == Typ::Datei {
                attribute.groesse.unwrap_or(0)
            } else {
                0
            },
            rechte: attribute.rechte.map(|rechte| rechte & 0o7777),
            geaendert: attribute.zeiten.map(|(_, geaendert)| zeitpunkt(geaendert)),
            zugriff: attribute.zeiten.map(|(zugriff, _)| zeitpunkt(zugriff)),
        }
    }
}

impl Angaben {
    /// Die Angaben eines Eintrags im Archiv, aus dem Inhaltsverzeichnis.
    ///
    /// Eine Sonderdatei im tar ist keine gewoehnliche Datei; was sonst nicht
    /// herauskommt, sagt [`Quelle::kommt_nicht_heraus`] vor dem Anlegen. Eine
    /// Zugriffszeit fuehrt kein Archiv, also gilt die Aenderungszeit fuer
    /// beide, wie bei einer Quelle ohne Zugriffszeit.
    ///
    /// Die Groesse ist die in Bytes, `Archiveintrag::datenlaenge`, und nicht
    /// `groesse`, das in einer Datenbank Zeilen zaehlt; steht sie vor dem
    /// Lesen nicht fest, wie bei einer Tabelle, ist sie 0.
    fn aus_archiveintrag(eintrag: &Archiveintrag) -> Self {
        Angaben {
            typ: eintrag.typ,
            gewoehnlich: eintrag.lesbarkeit != Lesbarkeit::Sonderdatei,
            groesse: if eintrag.typ == Typ::Datei {
                eintrag.datenlaenge().unwrap_or(0)
            } else {
                0
            },
            rechte: eintrag.rechte,
            geaendert: Some(eintrag.geaendert),
            zugriff: None,
        }
    }
}

fn zeitpunkt(sekunden: u32) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(u64::from(sekunden))
}

/// Sekunden seit der Unix-Epoche, wie SFTP v3 sie traegt: vor der Epoche 0,
/// nach 2106 die groesste Zahl.
fn sekunden(zeit: SystemTime) -> u32 {
    zeit.duration_since(UNIX_EPOCH)
        .map(|spanne| u32::try_from(spanne.as_secs()).unwrap_or(u32::MAX))
        .unwrap_or(0)
}

/// Die Kinder eines Ordners: Name und, wo das Lesen sie mitbrachte, Angaben.
struct Liste {
    kinder: Vec<(String, Option<Angaben>)>,
    /// Wie viele Namen kein UTF-8 sind oder die Namensregel nicht bestehen.
    unansteuerbar: u64,
}

/// Wie viele Namen ein ferner Ordner in Kopieren, Verschieben und Loeschen
/// hoechstens liefern darf, bevor der Vorgang ihn mit
/// [`Fehlschlag::ZuVieleNamen`] auslaesst.
///
/// Ein solcher Ordner wird ganz gelesen, bevor der erste Eintrag drankommt;
/// die Grenze haelt einen Server, der nie `EOF` sendet, davon ab, den
/// Speicher von KRK unbegrenzt wachsen zu lassen. Eine Million Namen sind
/// mit Attributen einige hundert Megabyte, weit ueber jedem Ordner, den man
/// von Hand kopiert. Das Dateifenster liest in Stapeln und kennt keine Grenze.
const HOECHSTENS_NAMEN: usize = 1_000_000;

/// Die Frage, ob der Nutzer abgebrochen hat, fuer die Wege, die einen fernen
/// Ordner lesen oder einen Baum entfernen.
///
/// Eine Frage und kein Verweis auf die [`Steuerung`], weil zwei Wege sie
/// bewusst nie bejahen: das Wegraeumen nach einem Abbruch und das Entfernen
/// eines alten Ziels, dessen Ersatz schon an seinem Platz steht
/// ([`nie_abgebrochen`]).
type Abbruchfrage<'a> = &'a dyn Fn() -> bool;

/// Die Abbruchfrage der Wege, die zu Ende laufen sollen, auch nach einem
/// Abbruch.
fn nie_abgebrochen() -> bool {
    false
}

/// Ein Eintrag, lokal oder auf einem Server, mit dem Gespraech, ueber das er
/// erreicht wird.
#[derive(Clone)]
enum Stelle {
    Lokal(PathBuf),
    Fern {
        gespraech: Arc<Gespraech>,
        kennung: Serverkennung,
        pfad: Fernpfad,
    },
}

impl Stelle {
    /// Die Stelle eines Orts. Fern nimmt sie das Gespraech, das jetzt gilt;
    /// ein spaeterer Wiederaufbau erreicht diesen Lauf nicht mehr. Ein Ort in
    /// einem Archiv hat keine Stelle (siehe [`ausfuehren`]).
    fn von(ort: &Ort) -> Option<Self> {
        match ort {
            Ort::Lokal(pfad) => Some(Stelle::Lokal(pfad.clone())),
            Ort::Fern(fern) => Some(Stelle::Fern {
                gespraech: fern.zugang.gespraech(),
                kennung: fern.kennung().clone(),
                pfad: fern.pfad.clone(),
            }),
            Ort::Archiv(_) => None,
        }
    }

    fn bezeichnung(&self) -> PathBuf {
        match self {
            Stelle::Lokal(pfad) => pfad.clone(),
            Stelle::Fern { kennung, pfad, .. } => PathBuf::from(format!("{kennung}:{pfad}")),
        }
    }

    /// Der letzte Namensteil, oder der Schluessel des Grundes, warum es
    /// keinen gibt.
    fn name(&self) -> Result<String, Text> {
        match self {
            Stelle::Lokal(pfad) => match pfad.file_name() {
                None => Err(Text::VorgangPfadBenenntKeinenEintrag),
                Some(name) => name
                    .to_str()
                    .map(str::to_owned)
                    .ok_or(Text::VorgangFernNameOhneUtf8),
            },
            Stelle::Fern { pfad, .. } => pfad
                .name()
                .map(str::to_owned)
                .ok_or(Text::VorgangPfadBenenntKeinenEintrag),
        }
    }

    /// Der Eintrag `name` in diesem Ordner. Fern `None`, wenn `name` keine
    /// einzelne Komponente ist.
    fn kind(&self, name: &str) -> Option<Stelle> {
        match self {
            Stelle::Lokal(pfad) => Some(Stelle::Lokal(pfad.join(name))),
            Stelle::Fern {
                gespraech,
                kennung,
                pfad,
            } => pfad.verbinden(name).map(|pfad| Stelle::Fern {
                gespraech: Arc::clone(gespraech),
                kennung: kennung.clone(),
                pfad,
            }),
        }
    }

    /// Der Eintrag `name` neben diesem.
    fn geschwister(&self, name: &str) -> Option<Stelle> {
        match self {
            Stelle::Lokal(pfad) => Some(Stelle::Lokal(pfad.with_file_name(name))),
            Stelle::Fern {
                gespraech,
                kennung,
                pfad,
            } => pfad
                .aufwaerts()
                .and_then(|oben| oben.verbinden(name))
                .map(|pfad| Stelle::Fern {
                    gespraech: Arc::clone(gespraech),
                    kennung: kennung.clone(),
                    pfad,
                }),
        }
    }

    /// Ob zwei Stellen denselben Eintrag benennen: lokal an der
    /// Naemlichkeit wie `super::zielpfad`, fern auf demselben Server am Pfad
    /// und, wo die Pfade verschieden sind, am Namen unter den zwei
    /// Elternordnern, die `REALPATH` auf dem Server aufloest (Modulkopf, „Was
    /// dieses Modul nicht haelt“).
    ///
    /// Aufgeloest werden die Elternordner und nicht die Eintraege selbst: der
    /// Eintrag ist, was `LSTAT` sieht, und eine Verknuepfung an seiner Stelle
    /// ist ein eigener Eintrag, kein Weg zu ihrem Ziel. Laesst sich einer der
    /// beiden Ordner nicht aufloesen, ist die Antwort [`Gleichheit::Unklar`].
    fn derselbe_eintrag(&self, pfand: &Fernfaden, andere: &Stelle) -> Gleichheit {
        match (self, andere) {
            (Stelle::Lokal(eine), Stelle::Lokal(andere)) => {
                if benennen_denselben_eintrag(eine, andere) {
                    Gleichheit::Ja
                } else {
                    Gleichheit::Nein
                }
            }
            (
                Stelle::Fern {
                    gespraech,
                    kennung,
                    pfad,
                },
                Stelle::Fern {
                    kennung: andere_kennung,
                    pfad: anderer_pfad,
                    ..
                },
            ) => {
                if kennung != andere_kennung {
                    return Gleichheit::Nein;
                }
                if pfad == anderer_pfad {
                    return Gleichheit::Ja;
                }
                let (Some(oben), Some(anderes_oben)) = (pfad.aufwaerts(), anderer_pfad.aufwaerts())
                else {
                    return Gleichheit::Nein;
                };
                if pfad.name() != anderer_pfad.name() {
                    return Gleichheit::Nein;
                }
                match (
                    gespraech.realpath(pfand, oben.als_text()),
                    gespraech.realpath(pfand, anderes_oben.als_text()),
                ) {
                    (Ok(eins), Ok(zwei)) if eins == zwei => Gleichheit::Ja,
                    (Ok(_), Ok(_)) => Gleichheit::Nein,
                    (Err(fehler), _) | (_, Err(fehler)) => {
                        Gleichheit::Unklar(Fehlschlag::Fern(fehler))
                    }
                }
            }
            (Stelle::Lokal(_), Stelle::Fern { .. }) | (Stelle::Fern { .. }, Stelle::Lokal(_)) => {
                Gleichheit::Nein
            }
        }
    }

    /// Ob dieser Ordner der Ordner `quelle` ist oder unter ihm liegt.
    ///
    /// Fern erst am Pfad und dann, wenn der Pfad nein sagt, an den zwei
    /// Pfaden, die `REALPATH` auf dem Server aufloest; so faellt auch ein
    /// Zielordner auf, der ueber eine Verknuepfung in der Quelle liegt, und
    /// das Kopieren rekursiert nicht in sich selbst. Ohne Antwort von
    /// `REALPATH` gilt die Antwort am Pfad.
    fn liegt_in(&self, pfand: &Fernfaden, quelle: &Stelle) -> bool {
        match (self, quelle) {
            (Stelle::Lokal(ordner), Stelle::Lokal(quelle)) => liegt_im_ordner(ordner, quelle),
            (
                Stelle::Fern {
                    gespraech,
                    kennung,
                    pfad,
                },
                Stelle::Fern {
                    kennung: quellkennung,
                    pfad: quellpfad,
                    ..
                },
            ) => {
                if kennung != quellkennung {
                    return false;
                }
                let darunter = |ordner: &[u8], quelle: &[u8]| {
                    quelle == b"/"
                        || ordner == quelle
                        || ordner
                            .strip_prefix(quelle)
                            .is_some_and(|rest| rest.first() == Some(&b'/'))
                };
                if darunter(pfad.als_text().as_bytes(), quellpfad.als_text().as_bytes()) {
                    return true;
                }
                match (
                    gespraech.realpath(pfand, pfad.als_text()),
                    gespraech.realpath(pfand, quellpfad.als_text()),
                ) {
                    (Ok(ordner), Ok(quelle)) => darunter(&ordner, &quelle),
                    (Err(_), _) | (_, Err(_)) => false,
                }
            }
            (Stelle::Lokal(_), Stelle::Fern { .. }) | (Stelle::Fern { .. }, Stelle::Lokal(_)) => {
                false
            }
        }
    }

    fn angaben(&self, pfand: &Fernfaden) -> Result<Angaben, Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => fs::symlink_metadata(pfad)
                .map(|angaben| Angaben::aus_metadata(&angaben))
                .map_err(Fehlschlag::Lokal),
            Stelle::Fern {
                gespraech, pfad, ..
            } => gespraech
                .lstat(pfand, pfad.als_text())
                .map(|attribute| Angaben::aus_attributen(&attribute))
                .map_err(Fehlschlag::Fern),
        }
    }

    /// Die Kinder dieses Ordners. Lokal ueber den Leser aus Schritt 2 der
    /// Runde 1, fern ueber `OPENDIR` und `READDIR`, deren Attribute die
    /// Angaben gleich mitbringen; fern fragt es `abbruch` vor jedem `READDIR`
    /// ([`fern_namen`]).
    fn liste(&self, pfand: &Fernfaden, abbruch: Abbruchfrage<'_>) -> Result<Liste, Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => {
                let eintraege = lesen(pfad).map_err(Fehlschlag::Lokal)?;
                Ok(Liste {
                    kinder: eintraege
                        .into_iter()
                        .map(|eintrag| (eintrag.name, None))
                        .collect(),
                    unansteuerbar: 0,
                })
            }
            Stelle::Fern {
                gespraech, pfad, ..
            } => {
                let namen = fern_namen(pfand, gespraech, pfad, abbruch, HOECHSTENS_NAMEN)?;
                let mut liste = Liste {
                    kinder: Vec::with_capacity(namen.len()),
                    unansteuerbar: 0,
                };
                for eintrag in namen {
                    let Ok(name) = String::from_utf8(eintrag.name) else {
                        liste.unansteuerbar += 1;
                        continue;
                    };
                    if name == "." || name == ".." {
                        continue;
                    }
                    // Dieselbe Regel wie der Leser des Dateifensters
                    // (`verzeichnis::leser::fern_eintrag`): ein Name, zu dem
                    // sich kein Pfad bauen laesst, zaehlt wie einer ohne UTF-8.
                    if !ist_einzelner_name(&name) {
                        liste.unansteuerbar += 1;
                        continue;
                    }
                    let angaben = eintrag
                        .attribute
                        .rechte
                        .is_some()
                        .then(|| Angaben::aus_attributen(&eintrag.attribute));
                    liste.kinder.push((name, angaben));
                }
                Ok(liste)
            }
        }
    }

    fn ordner_anlegen(&self, pfand: &Fernfaden) -> Result<(), Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => fs::create_dir(pfad).map_err(Fehlschlag::Lokal),
            Stelle::Fern {
                gespraech, pfad, ..
            } => gespraech
                .mkdir(pfand, pfad.als_text(), Attribute::default())
                .map_err(Fehlschlag::Fern),
        }
    }

    /// Legt eine Datei ausschliessend an und oeffnet sie zum Schreiben.
    fn datei_anlegen(&self, pfand: &Fernfaden) -> Result<Schreibziel, Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(pfad)
                .map(Schreibziel::Lokal)
                .map_err(Fehlschlag::Lokal),
            Stelle::Fern {
                gespraech, pfad, ..
            } => gespraech
                .open(
                    pfand,
                    pfad.als_text(),
                    oeffnen::WRITE | oeffnen::CREAT | oeffnen::EXCL,
                    Attribute::default(),
                )
                .map(|handle| Schreibziel::Fern {
                    gespraech: Arc::clone(gespraech),
                    handle,
                    offen: VecDeque::new(),
                    versatz: 0,
                })
                .map_err(Fehlschlag::Fern),
        }
    }

    fn verknuepfung_lesen(&self, pfand: &Fernfaden) -> Result<Vec<u8>, Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => fs::read_link(pfad)
                .map(|verweis| verweis.as_os_str().as_bytes().to_vec())
                .map_err(Fehlschlag::Lokal),
            Stelle::Fern {
                gespraech, pfad, ..
            } => gespraech
                .readlink(pfand, pfad.als_text())
                .map_err(Fehlschlag::Fern),
        }
    }

    fn verknuepfung_anlegen(&self, pfand: &Fernfaden, verweis: &[u8]) -> Result<(), Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => std::os::unix::fs::symlink(OsStr::from_bytes(verweis), pfad)
                .map_err(Fehlschlag::Lokal),
            Stelle::Fern {
                gespraech, pfad, ..
            } => {
                let verweis = std::str::from_utf8(verweis).map_err(|_| Fehlschlag::NameOhneUtf8)?;
                gespraech
                    .symlink(pfand, pfad.als_text(), verweis)
                    .map_err(|fehler| match fehler {
                        Fernfehler::NichtAngeboten(_) => Fehlschlag::KeineVerknuepfung,
                        andere => Fehlschlag::Fern(andere),
                    })
            }
        }
    }

    fn datei_entfernen(&self, pfand: &Fernfaden) -> Result<(), Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => fs::remove_file(pfad).map_err(Fehlschlag::Lokal),
            Stelle::Fern {
                gespraech, pfad, ..
            } => gespraech
                .remove(pfand, pfad.als_text())
                .map_err(Fehlschlag::Fern),
        }
    }

    fn ordner_entfernen(&self, pfand: &Fernfaden) -> Result<(), Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => fs::remove_dir(pfad).map_err(Fehlschlag::Lokal),
            Stelle::Fern {
                gespraech, pfad, ..
            } => gespraech
                .rmdir(pfand, pfad.als_text())
                .map_err(Fehlschlag::Fern),
        }
    }

    /// Entfernt den Eintrag endgueltig, einen Ordner samt Inhalt. Fern fragt
    /// es `abbruch` vor jedem Eintrag ([`fern_baum_entfernen`]); lokal geht es
    /// ueber `loeschen::baum_entfernen`, das keinen Abbruch kennt.
    fn baum_entfernen(
        &self,
        pfand: &Fernfaden,
        abbruch: Abbruchfrage<'_>,
    ) -> Result<(), Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => loeschen::baum_entfernen(pfad).map_err(Fehlschlag::Lokal),
            Stelle::Fern {
                gespraech, pfad, ..
            } => fern_baum_entfernen(pfand, gespraech, pfad, abbruch),
        }
    }

    /// Benennt auf demselben Server um; `None`, wenn die zwei Stellen nicht
    /// auf demselben Server liegen.
    fn umbenennen(&self, pfand: &Fernfaden, ziel: &Stelle) -> Option<Result<(), Fehlschlag>> {
        match (self, ziel) {
            (
                Stelle::Fern {
                    gespraech,
                    kennung,
                    pfad,
                },
                Stelle::Fern {
                    kennung: zielkennung,
                    pfad: zielpfad,
                    ..
                },
            ) if kennung == zielkennung => Some(
                gespraech
                    .rename(pfand, pfad.als_text(), zielpfad.als_text())
                    .map_err(Fehlschlag::Fern),
            ),
            (Stelle::Fern { .. }, Stelle::Fern { .. })
            | (Stelle::Lokal(_), _)
            | (Stelle::Fern { .. }, Stelle::Lokal(_)) => None,
        }
    }

    /// Benennt diesen Eintrag in seinem Ordner um: lokal ueber
    /// [`super::umbenennen()`], das ein vorhandenes Ziel abweist, fern ueber
    /// `RENAME`, das es ebenso abweist (Entscheidung 10).
    fn umbenennen_in(&self, pfand: &Fernfaden, neuer_name: &str) -> Result<(), Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => umbenennen(pfad, neuer_name)
                .map(|_neuer_pfad| ())
                .map_err(Fehlschlag::Lokal),
            Stelle::Fern {
                gespraech, pfad, ..
            } => {
                let zielpfad = pfad
                    .aufwaerts()
                    .and_then(|oben| oben.verbinden(neuer_name))
                    .ok_or(Fehlschlag::KeinEintrag)?;
                gespraech
                    .rename(pfand, pfad.als_text(), zielpfad.als_text())
                    .map_err(Fehlschlag::Fern)
            }
        }
    }

    /// Ein freier Name neben diesem Eintrag, fuer die Regel
    /// „automatisch umbenennen“. Lokal ist es [`freier_name`]; fern fragt
    /// `LSTAT` je Vorschlag.
    fn freier_name(&self, pfand: &Fernfaden) -> String {
        match self {
            Stelle::Lokal(pfad) => freier_name(pfad),
            Stelle::Fern { .. } => freier_name_unter(&self.bezeichnung(), |vorschlag| {
                self.geschwister(vorschlag).is_some_and(|nachbar| {
                    nachbar
                        .angaben(pfand)
                        .is_err_and(|fehler| fehler.ist_nicht_da())
                })
            }),
        }
    }

    /// Setzt Rechte und Aenderungszeit. Eine Verknuepfung bekommt keine.
    ///
    /// Lokal erst die Zeit, dann die Rechte, wie `kopieren` es fuer Ordner
    /// tut: Rechte ohne Schreibrecht duerfen das Setzen der Zeit nicht
    /// verhindern. Fern beides in einem `SETSTAT`.
    fn angaben_setzen(&self, pfand: &Fernfaden, angaben: &Angaben) -> Result<(), Fehlschlag> {
        if angaben.typ == Typ::Verknuepfung {
            return Ok(());
        }
        match self {
            Stelle::Lokal(pfad) => {
                if let Some(geaendert) = angaben.geaendert {
                    let zeiten = FileTimes::new()
                        .set_modified(geaendert)
                        .set_accessed(angaben.zugriff.unwrap_or(geaendert));
                    File::open(pfad)
                        .and_then(|datei| datei.set_times(zeiten))
                        .map_err(Fehlschlag::Lokal)?;
                }
                if let Some(rechte) = angaben.rechte {
                    fs::set_permissions(pfad, Permissions::from_mode(rechte))
                        .map_err(Fehlschlag::Lokal)?;
                }
                Ok(())
            }
            Stelle::Fern {
                gespraech, pfad, ..
            } => {
                let attribute = Attribute {
                    rechte: angaben.rechte,
                    zeiten: angaben.geaendert.map(|geaendert| {
                        (
                            sekunden(angaben.zugriff.unwrap_or(geaendert)),
                            sekunden(geaendert),
                        )
                    }),
                    ..Attribute::default()
                };
                gespraech
                    .setstat(pfand, pfad.als_text(), attribute)
                    .map_err(Fehlschlag::Fern)
            }
        }
    }

    /// Liest die Datei vom Anfang bis zum Ende in Bloecken und reicht jeden
    /// an `empfang`; `Break` dort bricht ab, und das Ergebnis ist
    /// [`Fehlschlag::Abgebrochen`].
    ///
    /// Lokal geht das Oeffnen ueber `ohne_warten_oeffnen`, damit eine
    /// benannte Roehre an diesem Pfad den Faden nicht anhaelt, und die Frage
    /// nach der gewoehnlichen Datei steht am offenen Deskriptor.
    fn lesen(
        &self,
        pfand: &Fernfaden,
        mut empfang: impl FnMut(&[u8]) -> ControlFlow<()>,
    ) -> Result<u64, Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => {
                let mut datei = ohne_warten_oeffnen(pfad).map_err(Fehlschlag::Lokal)?;
                if !datei.metadata().map_err(Fehlschlag::Lokal)?.is_file() {
                    return Err(Fehlschlag::KeineGewoehnlicheDatei);
                }
                let mut puffer = vec![0u8; BLOCK as usize];
                let mut gelesen: u64 = 0;
                loop {
                    let zahl = match datei.read(&mut puffer) {
                        Ok(0) => return Ok(gelesen),
                        Ok(zahl) => zahl,
                        Err(fehler) if fehler.kind() == io::ErrorKind::Interrupted => continue,
                        Err(fehler) => return Err(Fehlschlag::Lokal(fehler)),
                    };
                    gelesen += zahl as u64;
                    if empfang(&puffer[..zahl]).is_break() {
                        return Err(Fehlschlag::Abgebrochen);
                    }
                }
            }
            Stelle::Fern {
                gespraech, pfad, ..
            } => {
                let handle = gespraech
                    .open(pfand, pfad.als_text(), oeffnen::READ, Attribute::default())
                    .map_err(Fehlschlag::Fern)?;
                let ergebnis = gespraech.lesen_in_bloecken(pfand, &handle, empfang);
                // Ein gescheitertes Schliessen eines gelesenen Handles aendert
                // nichts an dem, was angekommen ist.
                let _ = gespraech.close(pfand, handle);
                ergebnis.map_err(|fehler| match fehler {
                    Fernfehler::Abgebrochen => Fehlschlag::Abgebrochen,
                    andere => Fehlschlag::Fern(andere),
                })
            }
        }
    }
}

// ----------------------------------------------------------------------
// Lesequelle: eine Stelle oder ein Eintrag in einem Archiv (E11)
// ----------------------------------------------------------------------

/// Woraus kopiert wird: eine [`Stelle`], lokal oder fern, oder ein Eintrag in
/// einem Archiv.
///
/// **Lesen und Schreiben sind am Typ getrennt** (E11 des Plans
/// `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`): diese
/// Aufzaehlung traegt allein Lesehandlungen, die Schreibhandlungen stehen
/// allein an [`Stelle`]. Die Kopierwege nehmen ihre Quelle als `&Quelle` und
/// ihr Ziel als `&Stelle`, also laesst sich eine Schreibhandlung an einem
/// Archiveintrag gar nicht erst schreiben. Wer an einer Quelle schreiben
/// will, holt sich ueber [`Quelle::als_stelle`] die Stelle und bekommt fuer
/// ein Archiv keine.
#[derive(Clone)]
enum Quelle {
    Stelle(Stelle),
    Archiv(Archivstelle),
}

/// Ein Eintrag in einem Archiv, mit dem Leser, den alle Eintraege dieses
/// Archivs im Vorgang teilen.
///
/// Geteilt ueber `Rc` und nicht `Arc`: der Vorgang laeuft auf einem Faden,
/// und der Leser haelt seinen Strom zwischen zwei Eintraegen offen.
#[derive(Clone)]
struct Archivstelle {
    leser: Rc<RefCell<Archivleser>>,
    innen: Innenpfad,
}

impl Archivstelle {
    /// `/pfad/zum/archiv.zip:/innen/pfad`, die Schreibweise des
    /// Fenstertitels und von [`bezeichnung`].
    fn bezeichnung(&self) -> PathBuf {
        PathBuf::from(format!(
            "{}:{}",
            self.leser.borrow().datei().display(),
            self.innen
        ))
    }

    /// Die Kinder dieses Ordners samt Angaben, in der Lesefolge eines
    /// tar-Stroms, oder `None`, wenn er im Verzeichnis kein Ordner ist.
    fn kinder(&self) -> Option<Vec<(Quelle, Option<Angaben>)>> {
        let leser = self.leser.borrow();
        let mut kinder: Vec<(Quelle, Option<Angaben>)> = leser
            .verzeichnis()
            .kinder(&self.innen)?
            .map(|eintrag| {
                (
                    Quelle::Archiv(Archivstelle {
                        leser: Rc::clone(&self.leser),
                        innen: eintrag.innen.clone(),
                    }),
                    Some(Angaben::aus_archiveintrag(eintrag)),
                )
            })
            .collect();
        if leser.liest_strom() {
            kinder.sort_by_cached_key(|(kind, _)| match kind {
                Quelle::Archiv(stelle) => leser.lesefolge(&stelle.innen),
                Quelle::Stelle(_) => u64::MAX,
            });
        }
        Some(kinder)
    }
}

impl Quelle {
    /// Die Stelle, sofern die Quelle keine im Archiv ist.
    fn als_stelle(&self) -> Option<&Stelle> {
        match self {
            Quelle::Stelle(stelle) => Some(stelle),
            Quelle::Archiv(_) => None,
        }
    }

    fn bezeichnung(&self) -> PathBuf {
        match self {
            Quelle::Stelle(stelle) => stelle.bezeichnung(),
            Quelle::Archiv(stelle) => stelle.bezeichnung(),
        }
    }

    /// Der Name am Ziel, oder der Schluessel des Grundes, warum es keinen
    /// gibt. Eine Stelle bringt ihren letzten Namensteil mit; ein Eintrag im
    /// Archiv seinen `Archiveintrag::ausfuhrname`, die eine Stelle, an der
    /// eine Tabelle zu `<name>.tsv` und eine Sicht zu `<name>.sql` wird. Der
    /// Name darin ist eine einzelne Komponente nach der einen Namensregel
    /// (`Innenpfad::aus_archivname`); ein Name mit `..` oder `/` kommt also
    /// nie bis ans Ziel.
    fn name(&self) -> Result<String, Text> {
        match self {
            Quelle::Stelle(stelle) => stelle.name(),
            Quelle::Archiv(stelle) => stelle
                .leser
                .borrow()
                .verzeichnis()
                .eintrag(&stelle.innen)
                .and_then(Archiveintrag::ausfuhrname)
                .ok_or(Text::VorgangPfadBenenntKeinenEintrag),
        }
    }

    /// Die Angaben der Quelle. Im Archiv fragt es zuerst, ob die Archivdatei
    /// noch unter ihrem Namen steht: der Tausch beim Ueberschreiben fragt so
    /// nach der Quelle, und liegt das Archiv im beiseitegelegten Ziel, fehlt
    /// sie wie eine lokale ([`tauschen`]).
    fn angaben(&self, pfand: &Fernfaden) -> Result<Angaben, Fehlschlag> {
        match self {
            Quelle::Stelle(stelle) => stelle.angaben(pfand),
            Quelle::Archiv(stelle) => {
                let leser = stelle.leser.borrow();
                fs::metadata(leser.datei()).map_err(Fehlschlag::Lokal)?;
                leser
                    .verzeichnis()
                    .eintrag(&stelle.innen)
                    .map(Angaben::aus_archiveintrag)
                    .ok_or(Fehlschlag::Archiv(Lesefehler::Fehlt))
            }
        }
    }

    /// Warum diese Quelle nicht herauskommt, oder `None`. Eine Stelle kommt
    /// immer; ein Archiveintrag nicht, wenn er verschluesselt, mit einem
    /// unbekannten Verfahren gepackt, ein harter Verweis oder eine
    /// Sonderdatei ist ([`lesbarkeit_grund`]).
    fn kommt_nicht_heraus(&self) -> Option<String> {
        match self {
            Quelle::Stelle(_) => None,
            Quelle::Archiv(stelle) => {
                let leser = stelle.leser.borrow();
                let eintrag = leser.verzeichnis().eintrag(&stelle.innen)?;
                lesbarkeit_grund(&eintrag.lesbarkeit)
            }
        }
    }

    /// Ob Quelle und `ziel` denselben Eintrag benennen. Ein Eintrag im Archiv
    /// ist nie derselbe wie eine Stelle: ein Ziel liegt immer ausserhalb des
    /// Archivs.
    fn derselbe_eintrag(&self, pfand: &Fernfaden, ziel: &Stelle) -> Gleichheit {
        match self {
            Quelle::Stelle(stelle) => stelle.derselbe_eintrag(pfand, ziel),
            Quelle::Archiv(_) => Gleichheit::Nein,
        }
    }

    /// Ob der Ordner `ordner` diese Quelle ist oder in ihr liegt; ein Ordner
    /// ausserhalb eines Archivs liegt nie in einem Eintrag darin.
    fn enthaelt(&self, pfand: &Fernfaden, ordner: &Stelle) -> bool {
        match self {
            Quelle::Stelle(stelle) => ordner.liegt_in(pfand, stelle),
            Quelle::Archiv(_) => false,
        }
    }

    fn verknuepfung_lesen(&self, pfand: &Fernfaden) -> Result<Vec<u8>, Fehlschlag> {
        match self {
            Quelle::Stelle(stelle) => stelle.verknuepfung_lesen(pfand),
            Quelle::Archiv(stelle) => stelle
                .leser
                .borrow_mut()
                .verweis_lesen(pfand, &stelle.innen)
                .map_err(Fehlschlag::aus_archiv),
        }
    }

    /// Liest die Datei in Bloecken, wie [`Stelle::lesen`]; im Archiv ueber
    /// den geteilten [`Archivleser`].
    ///
    /// **Kein Block ist groesser als [`BLOCK`]**, auch aus dem Archiv, dessen
    /// Leser in groesseren Stuecken liest: ein fernes Ziel schickt jeden
    /// Block als ein `WRITE`, und ein Paket ueber der Grenze des Servers
    /// beendet die Verbindung.
    fn lesen(
        &self,
        pfand: &Fernfaden,
        mut empfang: impl FnMut(&[u8]) -> ControlFlow<()>,
    ) -> Result<u64, Fehlschlag> {
        match self {
            Quelle::Stelle(stelle) => stelle.lesen(pfand, empfang),
            Quelle::Archiv(stelle) => stelle
                .leser
                .borrow_mut()
                .lesen(pfand, &stelle.innen, |daten| {
                    for stueck in daten.chunks(BLOCK as usize) {
                        empfang(stueck)?;
                    }
                    ControlFlow::Continue(())
                })
                .map_err(Fehlschlag::aus_archiv),
        }
    }
}

/// Ein zum Schreiben geoeffnetes Ziel.
enum Schreibziel {
    Lokal(File),
    /// Ein Handle auf dem Server mit bis zu [`OFFENE_SCHREIBUNGEN`] offenen
    /// `WRITE`.
    Fern {
        gespraech: Arc<Gespraech>,
        handle: Handle,
        offen: VecDeque<Ausstehend>,
        versatz: u64,
    },
}

impl Schreibziel {
    /// Haengt `daten` an. Fern wartet es erst dann auf die aelteste Antwort,
    /// wenn [`OFFENE_SCHREIBUNGEN`] offen sind.
    fn schreiben(&mut self, pfand: &Fernfaden, daten: &[u8]) -> Result<(), Fehlschlag> {
        match self {
            Schreibziel::Lokal(datei) => datei.write_all(daten).map_err(Fehlschlag::Lokal),
            Schreibziel::Fern {
                gespraech,
                handle,
                offen,
                versatz,
            } => {
                if offen.len() >= OFFENE_SCHREIBUNGEN
                    && let Some(aelteste) = offen.pop_front()
                {
                    aelteste.bestaetigen(pfand).map_err(Fehlschlag::Fern)?;
                }
                let ausstehend = gespraech
                    .write_senden(pfand, handle, *versatz, daten)
                    .map_err(Fehlschlag::Fern)?;
                offen.push_back(ausstehend);
                *versatz += daten.len() as u64;
                Ok(())
            }
        }
    }

    /// Wartet jede offene Antwort ab und schliesst. Der erste Fehler gilt.
    fn schliessen(self, pfand: &Fernfaden) -> Result<(), Fehlschlag> {
        match self {
            Schreibziel::Lokal(datei) => {
                drop(datei);
                Ok(())
            }
            Schreibziel::Fern {
                gespraech,
                handle,
                offen,
                ..
            } => {
                let mut erster = None;
                for ausstehend in offen {
                    if let Err(fehler) = ausstehend.bestaetigen(pfand)
                        && erster.is_none()
                    {
                        erster = Some(fehler);
                    }
                }
                let geschlossen = gespraech.close(pfand, handle);
                match (erster, geschlossen) {
                    (Some(fehler), _) | (None, Err(fehler)) => Err(Fehlschlag::Fern(fehler)),
                    (None, Ok(())) => Ok(()),
                }
            }
        }
    }
}

/// Alle Namen eines fernen Ordners: `OPENDIR`, `READDIR` bis zum Ende,
/// `CLOSE`.
///
/// **Vor jedem `READDIR` fragt es `abbruch`**, wie der Leser des
/// Dateifensters und wie die Uebertragung zwischen zwei Bloecken; ein Abbruch
/// wirkt damit nach hoechstens einer Antwort und nicht erst nach dem ganzen
/// Ordner. Mehr als `grenze` Namen nimmt es nicht an und endet mit
/// [`Fehlschlag::ZuVieleNamen`] ([`HOECHSTENS_NAMEN`]). Das Handle faellt in
/// jedem Fall.
fn fern_namen(
    pfand: &Fernfaden,
    gespraech: &Gespraech,
    pfad: &Fernpfad,
    abbruch: Abbruchfrage<'_>,
    grenze: usize,
) -> Result<Vec<Name>, Fehlschlag> {
    let handle = gespraech
        .opendir(pfand, pfad.als_text())
        .map_err(Fehlschlag::Fern)?;
    let mut namen = Vec::new();
    let gelesen = loop {
        if abbruch() {
            break Err(Fehlschlag::Abgebrochen);
        }
        match gespraech.readdir(pfand, &handle) {
            Ok(Some(stapel)) => {
                if namen.len().saturating_add(stapel.len()) > grenze {
                    break Err(Fehlschlag::ZuVieleNamen(grenze));
                }
                namen.extend(stapel);
            }
            Ok(None) => break Ok(()),
            Err(fehler) => break Err(Fehlschlag::Fern(fehler)),
        }
    };
    // Ein gescheitertes Schliessen nach dem Lesen aendert nichts an den
    // gelesenen Namen.
    let _ = gespraech.close(pfand, handle);
    gelesen.map(|()| namen)
}

/// Entfernt einen Eintrag auf dem Server endgueltig, einen Ordner samt
/// Inhalt, und haelt am ersten Fehler an, wie `loeschen::baum_entfernen`.
///
/// Vor jedem Eintrag fragt es `abbruch`, und das Lesen jedes Ordners fragt
/// ihn vor jedem `READDIR` ([`fern_namen`]).
fn fern_baum_entfernen(
    pfand: &Fernfaden,
    gespraech: &Gespraech,
    pfad: &Fernpfad,
    abbruch: Abbruchfrage<'_>,
) -> Result<(), Fehlschlag> {
    if abbruch() {
        return Err(Fehlschlag::Abgebrochen);
    }
    let angaben = gespraech
        .lstat(pfand, pfad.als_text())
        .map_err(Fehlschlag::Fern)?;
    if !angaben.ist_ordner() {
        return gespraech
            .remove(pfand, pfad.als_text())
            .map_err(Fehlschlag::Fern);
    }
    for eintrag in fern_namen(pfand, gespraech, pfad, abbruch, HOECHSTENS_NAMEN)? {
        // Ein Name ohne UTF-8 oder einer, den die Namensregel abweist, laesst
        // sich nicht ansteuern; er bleibt stehen, und das `RMDIR` darunter
        // scheitert dann mit seinem Grund.
        let Ok(name) = String::from_utf8(eintrag.name) else {
            continue;
        };
        let Some(kind) = pfad.verbinden(&name) else {
            continue;
        };
        fern_baum_entfernen(pfand, gespraech, &kind, abbruch)?;
    }
    gespraech
        .rmdir(pfand, pfad.als_text())
        .map_err(Fehlschlag::Fern)
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::process::{Command, Stdio};

    use super::*;
    use crate::fern::faden;
    use crate::operation::fortschritt::Steuerung;

    /// Ein frischer Ordner unter dem Temporaerverzeichnis, mit Prozesskennung
    /// und Probennamen.
    ///
    /// Bewusst keine weitere Pruefordner-Fassung mit `Drop`, aus dem Grund,
    /// den `operation/verschieben.rs` an seiner gleichgebauten Hilfe nennt:
    /// abgeraeumt wird von Hand mit [`abraeumen`] am Ende jeder Probe, und
    /// ein Rest einer abgebrochenen Probe faellt beim naechsten Anlegen.
    fn pruefpfad(probe: &str) -> PathBuf {
        let pfad = fs::canonicalize(std::env::temp_dir())
            .expect("das Temporaerverzeichnis laesst sich aufloesen")
            .join(format!("krk-fern-operation-{probe}-{}", std::process::id()));
        abraeumen(&pfad);
        fs::create_dir_all(&pfad).expect("der Pruefordner laesst sich nicht anlegen");
        pfad
    }

    fn abraeumen(pfad: &Path) {
        let _ = fs::remove_dir_all(pfad);
    }

    /// Fuehrt `arbeit` mit einem Gespraech zum Server des Systems aus, auf
    /// einem Faden mit Pfand.
    fn mit_gespraech<T: Send + 'static>(
        arbeit: impl FnOnce(&Fernfaden, &Arc<Gespraech>) -> T + Send + 'static,
    ) -> T {
        mit_gespraech_ueber(Command::new("/usr/libexec/sftp-server"), arbeit)
    }

    /// Ein Vermittler zwischen Gespraech und Server des Systems, in Perl, das
    /// macOS mitbringt: er reicht jedes Paket weiter und jede Antwort zurueck,
    /// eine nach der anderen. Beim ersten `RENAME` (Pakettyp 18), dessen Paket
    /// das Muster aus seinem Argument traegt, wartet er die Antwort des
    /// Servers ab, gibt sie **nicht** weiter, beendet den Server und endet.
    /// Das `RENAME` ist damit geschehen, und das Gespraech sieht allein die
    /// Trennung.
    ///
    /// Die Fehlerausgabe geht vor dem Start des Servers nach `/dev/null`,
    /// damit kein Enkelprozess die Fehlerausgabe des Gespraechs offen haelt.
    /// Jede Zeile ist eingerueckt: eine Zeile, die allein aus `}` besteht,
    /// beendete fuer die Baumproben (`tests/gemeinsam`, `betriebscode`) das
    /// Pruefmodul.
    const TRENNENDER_VERMITTLER: &str = r#"
  use strict; use warnings; use IPC::Open2;
  my $muster = shift @ARGV;
  open(STDERR, '>', '/dev/null') or die;
  my $pid = open2(my $vom, my $zum, '/usr/libexec/sftp-server');
  binmode($_) for (\*STDIN, \*STDOUT, $vom, $zum);
  sub lies { my ($fh, $n) = @_; my $p = '';
    while (length($p) < $n) { my $r = sysread($fh, $p, $n - length($p), length($p)); return undef unless $r; }
    return $p; }
  sub paket { my ($fh) = @_; my $kopf = lies($fh, 4); return undef unless defined $kopf;
    my $rumpf = lies($fh, unpack('N', $kopf)); return undef unless defined $rumpf; return $kopf . $rumpf; }
  sub schreiben { my ($fh, $d) = @_;
    while (length $d) { my $w = syswrite($fh, $d); exit 1 unless $w; substr($d, 0, $w) = ''; } }
  while (defined(my $anfrage = paket(\*STDIN))) {
    schreiben($zum, $anfrage);
    my $antwort = paket($vom);
    last unless defined $antwort;
    last if ord(substr($anfrage, 4, 1)) == 18 && index($anfrage, $muster) >= 0;
    schreiben(\*STDOUT, $antwort);
  }
  kill 'KILL', $pid; waitpid($pid, 0);
"#;

    /// Wie [`mit_gespraech`], ueber [`TRENNENDER_VERMITTLER`] mit `muster`.
    fn mit_trennendem_gespraech<T: Send + 'static>(
        muster: &str,
        arbeit: impl FnOnce(&Fernfaden, &Arc<Gespraech>) -> T + Send + 'static,
    ) -> T {
        let mut befehl = Command::new("/usr/bin/perl");
        befehl.arg("-e").arg(TRENNENDER_VERMITTLER).arg(muster);
        mit_gespraech_ueber(befehl, arbeit)
    }

    /// Fuehrt `arbeit` mit einem Gespraech ueber den Kindprozess `befehl` aus,
    /// auf einem Faden mit Pfand.
    fn mit_gespraech_ueber<T: Send + 'static>(
        mut befehl: Command,
        arbeit: impl FnOnce(&Fernfaden, &Arc<Gespraech>) -> T + Send + 'static,
    ) -> T {
        faden("krk-probe-fern".to_owned(), move |pfand| {
            let kind = befehl
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("sftp-server laesst sich nicht starten");
            let gespraech =
                Gespraech::eroeffnen(pfand, kind, "probe").expect("das Gespraech kommt nicht auf");
            arbeit(pfand, &Arc::new(gespraech))
        })
        .expect("kein Faden fuer die Probe")
        .join()
        .unwrap_or_else(|absturz| std::panic::resume_unwind(absturz))
    }

    fn fernpfad(pfad: &Path) -> Fernpfad {
        Fernpfad::aus_text(pfad.to_str().expect("Pruefpfade sind UTF-8")).expect("absolut")
    }

    /// Ein Abbruch waehrend des Lesens eines fernen Ordners wirkt vor dem
    /// naechsten `READDIR` und nicht erst nach dem ganzen Ordner (Befund
    /// `261017-0004-ferne-ordnerlisten-in-kopieren-verschieben-und-loeschen-fragen-keinen-abbruch-und-haben-keine-obergrenze`).
    ///
    /// Der Ordner traegt 10 000 Eintraege, der Server des Systems liefert
    /// hoechstens einige hundert je `READDIR`. Die Abbruchfrage sagt beim
    /// zweiten Mal ja; gezaehlt wird, wie oft sie gefragt wurde, und das ist
    /// die Zahl der Anfragen, die das Lesen bis dahin stellte. Bis zur
    /// Behebung gab es diese Frage nicht, und das Lesen lief bis `EOF`.
    #[test]
    fn ein_abbruch_beim_lesen_eines_fernen_ordners_wirkt_vor_dem_naechsten_readdir() {
        let ordner = pruefpfad("abbruch-liste");
        for i in 0..10_000 {
            File::create(ordner.join(format!("d-{i:05}"))).expect("Datei");
        }
        let pfad = fernpfad(&ordner);
        let (ergebnis, gefragt) = mit_gespraech(move |pfand, gespraech| {
            let gefragt = Cell::new(0_u32);
            let abbruch = || {
                gefragt.set(gefragt.get() + 1);
                gefragt.get() >= 2
            };
            let ergebnis = fern_namen(pfand, gespraech, &pfad, &abbruch, HOECHSTENS_NAMEN)
                .map(|namen| namen.len());
            (ergebnis, gefragt.get())
        });
        abraeumen(&ordner);
        assert!(
            matches!(ergebnis, Err(Fehlschlag::Abgebrochen)),
            "das Lesen endete mit {:?}",
            ergebnis.as_ref().map_err(grund)
        );
        assert_eq!(
            gefragt, 2,
            "nach einem READDIR gefragt, beim zweiten abgebrochen"
        );
    }

    /// Mehr Namen als die Grenze nimmt das Lesen nicht an, und der Grund
    /// nennt sie.
    #[test]
    fn ein_ferner_ordner_ueber_der_grenze_wird_mit_grund_ausgelassen() {
        let ordner = pruefpfad("grenze");
        for i in 0..300 {
            File::create(ordner.join(format!("d-{i:03}"))).expect("Datei");
        }
        let pfad = fernpfad(&ordner);
        let ergebnis = mit_gespraech(move |pfand, gespraech| {
            let unter =
                fern_namen(pfand, gespraech, &pfad, &nie_abgebrochen, 400).map(|namen| namen.len());
            let ueber =
                fern_namen(pfand, gespraech, &pfad, &nie_abgebrochen, 50).map(|namen| namen.len());
            (unter, ueber)
        });
        abraeumen(&ordner);
        let (unter, ueber) = ergebnis;
        assert_eq!(unter.ok(), Some(302), "300 Dateien, dazu `.` und `..`");
        match ueber {
            Err(fehler @ Fehlschlag::ZuVieleNamen(50)) => {
                assert!(grund(&fehler).contains("50"), "{}", grund(&fehler));
            }
            anderes => panic!("ueber der Grenze: {:?}", anderes.map_err(|f| grund(&f))),
        }
    }

    /// Das endgueltige Loeschen eines fernen Baums fragt den Abbruch je
    /// Eintrag. Gefragt wird vor dem Ordner, vor jedem `READDIR` (zwei bei zwanzig
    /// Namen) und vor jedem Kind; beim sechsten Mal sagt die Frage ja, also nach
    /// dem ersten Kind, und der Rest steht noch da.
    #[test]
    fn das_entfernen_eines_fernen_baums_haelt_beim_abbruch_an() {
        let ordner = pruefpfad("abbruch-baum");
        let baum = ordner.join("baum");
        fs::create_dir(&baum).expect("Ordner");
        for i in 0..20 {
            File::create(baum.join(format!("d-{i:02}"))).expect("Datei");
        }
        let pfad = fernpfad(&baum);
        let ergebnis = mit_gespraech(move |pfand, gespraech| {
            let gefragt = Cell::new(0_u32);
            let abbruch = || {
                gefragt.set(gefragt.get() + 1);
                gefragt.get() > 5
            };
            fern_baum_entfernen(pfand, gespraech, &pfad, &abbruch).map_err(|f| grund(&f))
        });
        let uebrig = fs::read_dir(&baum).map(Iterator::count).unwrap_or(0);
        abraeumen(&ordner);
        assert!(
            matches!(ergebnis, Err(ref text) if text == &grund(&Fehlschlag::Abgebrochen)),
            "{ergebnis:?}"
        );
        assert!(
            uebrig > 0 && uebrig < 20,
            "{uebrig} von 20 Eintraegen stehen noch"
        );
    }

    /// Ein Abbruch waehrend des Lesens der fernen Quelle kommt als Abbruch
    /// zurueck, und ein Verschieben ueber eine Grenze laesst die Quelle
    /// stehen.
    ///
    /// Haelt die Rueckgabe von `kinder_lesen`: endete das Lesen still, saehe
    /// [`ueber_die_grenze`] nichts Uebersprungenes und entfernte die Quelle,
    /// deren Inhalt nie angekommen ist.
    #[test]
    fn ein_abbruch_beim_lesen_der_quelle_laesst_sie_beim_verschieben_stehen() {
        let ordner = pruefpfad("abbruch-verschieben");
        let quelle = ordner.join("quelle");
        let ziel = ordner.join("ziel");
        fs::create_dir(&quelle).expect("Ordner");
        fs::create_dir(&ziel).expect("Ordner");
        fs::write(quelle.join("a.txt"), b"a").expect("Datei");
        let (q, z) = (quelle.clone(), ziel.clone());
        let ablauf = mit_gespraech(move |pfand, gespraech| {
            let quelle = Stelle::Fern {
                gespraech: Arc::clone(gespraech),
                kennung: Serverkennung::neu("rechner", None, Some("kai")),
                pfad: fernpfad(&q),
            };
            let Ok(angaben) = quelle.angaben(pfand) else {
                panic!("die Quelle steht nicht da");
            };
            let mut steuerung = Steuerung::neu(
                Arc::new(AtomicBool::new(true)),
                None,
                Konfliktregel::default(),
            );
            let ablauf = ueber_die_grenze(
                pfand,
                &quelle,
                &angaben,
                Stelle::Lokal(z.join("quelle")),
                &mut steuerung,
            );
            (ablauf, steuerung.uebersprungen_stand())
        });
        let steht = quelle.join("a.txt").exists();
        abraeumen(&ordner);
        assert!(ablauf.0 == Ablauf::Abgebrochen, "der Lauf ging weiter");
        assert_eq!(ablauf.1, 0, "ein Abbruch ist kein uebersprungener Eintrag");
        assert!(steht, "die Quelle ist trotz Abbruch entfernt");
    }

    /// Die Regel, welches Scheitern ein vergebener Name sein kann, Fall fuer
    /// Fall (Befund
    /// `261017-0004-jedes-scheitern-beim-fernen-anlegen-gilt-als-konflikt-…`).
    ///
    /// Bis zur Behebung galt jedes lokale und jedes ferne Scheitern als
    /// moeglicher Konflikt, also auch „keine Rechte“ und eine Trennung; mit
    /// einem Ziel, das `LSTAT` fand, kam dann die Konfliktfrage mit falschem
    /// Grund. Gegen den Server des Systems laesst sich das nicht stellen: der
    /// meldet an einem vorhandenen Namen immer `FAILURE`, auch in einem Ordner
    /// ohne Schreibrecht, weil `open(2)`, `mkdir(2)` und `link(2)` dort
    /// `EEXIST` vor `EACCES` setzen. Die Tafel steht deshalb hier.
    #[test]
    fn allein_ein_moeglicher_vergebener_name_ist_ein_konflikt() {
        let fern = |code: u32| {
            Fehlschlag::Fern(Fernfehler::Status {
                code,
                meldung: String::new(),
            })
        };
        let tafel = [
            (Fehlschlag::Lokal(io::ErrorKind::AlreadyExists.into()), true),
            (
                Fehlschlag::Lokal(io::ErrorKind::PermissionDenied.into()),
                false,
            ),
            (Fehlschlag::Lokal(io::ErrorKind::NotFound.into()), false),
            (Fehlschlag::Lokal(io::ErrorKind::StorageFull.into()), false),
            (fern(status::FAILURE), true),
            (fern(status::OP_UNSUPPORTED), true),
            (fern(status::NO_SUCH_FILE), false),
            (fern(status::PERMISSION_DENIED), false),
            (
                Fehlschlag::Fern(Fernfehler::Protokoll(String::new())),
                false,
            ),
            (
                Fehlschlag::Fern(Fernfehler::NichtAngeboten("symlink")),
                false,
            ),
            (Fehlschlag::Fern(Fernfehler::Abgebrochen), false),
            (Fehlschlag::NameOhneUtf8, false),
            (Fehlschlag::KeineGewoehnlicheDatei, false),
            (Fehlschlag::KeineVerknuepfung, false),
            (Fehlschlag::KeinEintrag, false),
            (Fehlschlag::ZuVieleNamen(1), false),
            (Fehlschlag::Abgebrochen, false),
        ];
        for (fehler, erwartet) in tafel {
            assert_eq!(
                fehler.kann_vergeben_heissen(),
                erwartet,
                "{}",
                grund(&fehler)
            );
        }
    }

    /// Die Regel, ob ein Tausch am Dateisystem der lokalen Seite haengen
    /// bleibt, Fall fuer Fall (Befund
    /// `261017-0145-die-frage-nach-der-quelle-im-tausch-liest-auf-einem-netzlaufwerk-…`).
    ///
    /// Ein Netzlaufwerk laesst sich ohne Netz nicht einhaengen; die Tafel
    /// steht deshalb hier, und die Frage am System haelt die Probe in
    /// `verzeichnis/sys.rs` gegen `/`. Beim Kopieren wird die lokale Seite
    /// nicht einmal gefragt.
    #[test]
    fn verschieben_ersetzt_nicht_ueber_ein_netzlaufwerk_und_kopieren_fragt_nicht() {
        let kopieren = netzbefund(Zweck::Kopieren, || {
            panic!("beim Kopieren wird das Dateisystem nicht gefragt")
        });
        assert!(matches!(kopieren, Netzbefund::Erlaubt), "{kopieren:?}");

        let tafel: [(Option<io::Result<bool>>, &str); 4] = [
            (None, "erlaubt"),
            (Some(Ok(true)), "erlaubt"),
            (Some(Ok(false)), "netzlaufwerk"),
            (Some(Err(io::ErrorKind::PermissionDenied.into())), "unklar"),
        ];
        for (lokale_seite, erwartet) in tafel {
            let befund = netzbefund(Zweck::Verschieben, || lokale_seite);
            let ist = match befund {
                Netzbefund::Erlaubt => "erlaubt",
                Netzbefund::Netzlaufwerk => "netzlaufwerk",
                Netzbefund::Unklar(_) => "unklar",
            };
            assert_eq!(ist, erwartet);
        }

        // Zwei lokale Seiten auf der Platte dieses Geraets: gefragt, und lokal.
        let ordner = pruefpfad("netzbefund");
        let befund = lokale_seite_liegt_lokal(
            &Quelle::Stelle(Stelle::Lokal(ordner.join("a"))),
            &Stelle::Lokal(ordner.clone()),
        );
        abraeumen(&ordner);
        assert!(
            matches!(befund, Some(Err(_))),
            "a gibt es nicht: {befund:?}"
        );
        let befund = lokale_seite_liegt_lokal(
            &Quelle::Stelle(Stelle::Lokal(PathBuf::from("/"))),
            &Stelle::Lokal(std::env::temp_dir()),
        );
        assert!(matches!(befund, Some(Ok(true))), "{befund:?}");
    }

    /// Die Texte des Berichts, als Paare aus Pfad und Grund.
    fn bericht(steuerung: &Steuerung) -> Vec<(PathBuf, String)> {
        steuerung
            .uebersprungen_seit(0)
            .iter()
            .map(|eintrag| (eintrag.pfad.clone(), eintrag.grund.clone()))
            .collect()
    }

    /// Eine Stelle auf dem Server unter der Kennung der Proben.
    fn fernstelle(gespraech: &Arc<Gespraech>, pfad: &Path) -> Stelle {
        Stelle::Fern {
            gespraech: Arc::clone(gespraech),
            kennung: Serverkennung::neu("rechner", None, Some("kai")),
            pfad: fernpfad(pfad),
        }
    }

    /// Endet das `RENAME`, das das Ziel beiseitelegt, mit der Trennung,
    /// nennt der Bericht beide Namen, unter denen das Ziel liegen kann (Befund
    /// `261017-0145-ein-rename-dessen-antwort-mit-der-trennung-verloren-geht-…`).
    ///
    /// Der Vermittler laesst das `RENAME` auf `.krk-alt-…` beim Server
    /// geschehen und trennt vor der Antwort. Bis zur Behebung las der Tausch
    /// die Trennung als „nicht geschehen“: der Bericht sagte „das Ziel liess
    /// sich nicht ersetzen“, und der Name, unter dem das Ziel nun lag, stand
    /// nirgends.
    #[test]
    fn ein_beiseitelegen_ohne_antwort_nennt_beide_namen_des_ziels() {
        let ordner = pruefpfad("getrennt-beiseite");
        let quelle = ordner.join("quelle.txt");
        fs::write(&quelle, b"neu").expect("quelle.txt");
        let server = ordner.join("server");
        fs::create_dir(&server).expect("server");
        fs::write(server.join("a.txt"), b"alt").expect("a.txt");
        let (q, z) = (quelle.clone(), server.join("a.txt"));
        let (ablauf, gemeldet, zielname) =
            mit_trennendem_gespraech(".krk-alt-", move |pfand, g| {
                let quelle = Quelle::Stelle(Stelle::Lokal(q));
                let ziel = fernstelle(g, &z);
                let Ok(angaben) = quelle.angaben(pfand) else {
                    panic!("die Quelle steht nicht da");
                };
                let mut steuerung = Steuerung::neu(
                    Arc::new(AtomicBool::new(false)),
                    None,
                    Konfliktregel::default(),
                );
                let ablauf = kopie_ersetzt_ziel(
                    pfand,
                    &quelle,
                    &angaben,
                    &Anlage::Datei,
                    &ziel,
                    &mut steuerung,
                    &mut Hinweisweg::bericht(),
                );
                (ablauf, bericht(&steuerung), ziel.bezeichnung())
            });
        let alt = altname(0);
        let beiseite = fs::read(server.join(&alt)).ok();
        let am_namen = server.join("a.txt").exists();
        abraeumen(&ordner);

        assert!(ablauf == Ablauf::Weiter);
        assert_eq!(
            beiseite.as_deref(),
            Some(&b"alt"[..]),
            "der Vermittler hat das RENAME nicht geschehen lassen"
        );
        assert!(!am_namen);
        let zum_ziel: Vec<&String> = gemeldet
            .iter()
            .filter(|(pfad, _)| *pfad == zielname)
            .map(|(_, grund)| grund)
            .collect();
        assert_eq!(zum_ziel.len(), 1, "{gemeldet:?}");
        assert!(
            zum_ziel[0].contains(&alt),
            "der Bericht nennt den Zwischennamen des Ziels nicht: {gemeldet:?}"
        );
    }

    /// Endet das `RENAME`, das beim Verschieben auf demselben Server die
    /// Quelle auf ihren Zwischennamen legt, mit der Trennung, nennt der
    /// Bericht beide Namen, unter denen die Quelle liegen kann (Befund wie
    /// oben).
    ///
    /// Bis zur Behebung brach die Schleife ab, und der Bericht sagte „das Ziel
    /// liess sich nicht ersetzen“; die Quelle lag dann unter `.krk-ersatz-…`,
    /// und ihr Name stand nirgends.
    #[test]
    fn ein_umbenennen_der_quelle_ohne_antwort_nennt_beide_namen_der_quelle() {
        let ordner = pruefpfad("getrennt-quelle");
        let hier = ordner.join("hier");
        fs::create_dir(&hier).expect("hier");
        fs::write(hier.join("a.txt"), b"wandernd").expect("hier/a.txt");
        fs::write(ordner.join("a.txt"), b"alt").expect("a.txt");
        let (q, z) = (hier.join("a.txt"), ordner.join("a.txt"));
        let (gemeldet, quellname) = mit_trennendem_gespraech(".krk-ersatz-", move |pfand, g| {
            let quelle = fernstelle(g, &q);
            let ziel = fernstelle(g, &z);
            let Ok(angaben) = quelle.angaben(pfand) else {
                panic!("die Quelle steht nicht da");
            };
            let mut steuerung = Steuerung::neu(
                Arc::new(AtomicBool::new(false)),
                None,
                Konfliktregel::default(),
            );
            let _ = umbenennen_ersetzt_ziel(pfand, &quelle, &angaben, &ziel, &mut steuerung);
            (bericht(&steuerung), quelle.bezeichnung())
        });
        let zwischen = zwischenname(0);
        let verlegt = fs::read(ordner.join(&zwischen)).ok();
        let ziel = fs::read(ordner.join("a.txt")).ok();
        abraeumen(&ordner);

        assert_eq!(
            verlegt.as_deref(),
            Some(&b"wandernd"[..]),
            "der Vermittler hat das RENAME nicht geschehen lassen"
        );
        assert_eq!(ziel.as_deref(), Some(&b"alt"[..]), "das Ziel ist angefasst");
        assert_eq!(gemeldet.len(), 1, "{gemeldet:?}");
        let (pfad, grund_text) = &gemeldet[0];
        assert_eq!(pfad, &quellname);
        assert!(
            grund_text.contains(&zwischen),
            "der Bericht nennt den Zwischennamen der Quelle nicht: {grund_text}"
        );
    }

    /// Packt einen Ordnerbaum als tar.gz, Ordner vor ihrem Inhalt, in der
    /// Folge, in der die Probe die Namen nennt.
    fn tgz_packen(wurzel: &Path, namen: &[&str], archiv: &Path) {
        let gepackt = flate2::write::GzEncoder::new(
            File::create(archiv).expect("tgz anlegen"),
            flate2::Compression::default(),
        );
        let mut bauer = tar::Builder::new(gepackt);
        for name in namen {
            bauer
                .append_path_with_name(wurzel.join(name), name)
                .expect("Eintrag packen");
        }
        bauer
            .into_inner()
            .expect("tar abschliessen")
            .finish()
            .expect("gzip abschliessen");
    }

    /// Ein Ordnerbaum aus einem tar.gz kostet einen Durchlauf durch den
    /// Strom: die Maschine nimmt die Quellen und die Kinder jedes Ordners in
    /// der Folge ihrer Fundstelle (`in_lesefolge_bringen`,
    /// `quellkinder_lesen`), und der Lesezeiger beginnt nie neu, auch wenn die
    /// Auswahl die Namen umgekehrt nennt. Zwei Quellen in absichtlich
    /// umgekehrter Folge, an der Ordnung vorbei, kosten genau einen Neustart.
    ///
    /// **Was sie nicht sieht:** ein tar, dessen Unterbaeume nicht
    /// zusammenhaengend im Strom liegen; dort sind Neustarts unvermeidlich,
    /// und die Probe misst keinen solchen.
    #[test]
    fn ein_baum_aus_einem_tgz_kostet_einen_durchlauf() {
        let ordner = pruefpfad("tgz-durchlauf");
        let quelle = ordner.join("quelle");
        for unter in ["a/b", "c"] {
            fs::create_dir_all(quelle.join(unter)).expect("Ordner");
        }
        for (name, inhalt) in [
            ("a/x.txt", "x"),
            ("a/b/y.txt", "y"),
            ("c/z.txt", "z"),
            ("d.txt", "d"),
        ] {
            fs::write(quelle.join(name), inhalt).expect("Datei");
        }
        let archiv = ordner.join("baum.tgz");
        tgz_packen(
            &quelle,
            &["a", "a/b", "a/b/y.txt", "a/x.txt", "c", "c/z.txt", "d.txt"],
            &archiv,
        );
        let ziel = ordner.join("ziel");
        let umgekehrt = ordner.join("umgekehrt");
        fs::create_dir(&ziel).expect("ziel");
        fs::create_dir(&umgekehrt).expect("umgekehrt");

        let zugang = Arc::new(crate::archiv::Archivzugang::neu(
            archiv,
            crate::archiv::Archivformat::TarGz,
        ));
        let (geordnet, ungeordnet, gemeldet) = {
            let (ziel, umgekehrt) = (ziel.clone(), umgekehrt.clone());
            faden("krk-probe-tgz".to_owned(), move |pfand| {
                let abbruch = Arc::new(AtomicBool::new(false));
                let mut steuerung =
                    Steuerung::neu(Arc::clone(&abbruch), None, Konfliktregel::default());
                let kopieren =
                    |namen: &[&str], ordnen: bool, nach: &Path, steuerung: &mut Steuerung| {
                        let leser = Rc::new(RefCell::new(
                            Archivleser::oeffnen(pfand, Arc::clone(&zugang), Arc::clone(&abbruch))
                                .expect("das Verzeichnis entsteht"),
                        ));
                        let mut quellen: Vec<(PathBuf, Result<Quelle, String>)> = namen
                            .iter()
                            .map(|name| {
                                let stelle = Archivstelle {
                                    leser: Rc::clone(&leser),
                                    innen: Innenpfad::aus_archivname(name).expect("Name"),
                                };
                                (stelle.bezeichnung(), Ok(Quelle::Archiv(stelle)))
                            })
                            .collect();
                        if ordnen {
                            in_lesefolge_bringen(&mut quellen);
                        }
                        let art = Fernart::Kopieren {
                            ziel: Ort::Lokal(nach.to_path_buf()),
                        };
                        let zielordner = Stelle::Lokal(nach.to_path_buf());
                        for (_, quelle) in quellen {
                            let quelle = quelle.expect("eine Quelle");
                            let ablauf = einen_abarbeiten(
                                pfand,
                                &art,
                                &quelle,
                                Some(&zielordner),
                                steuerung,
                                &mut Vec::new(),
                            );
                            assert!(ablauf == Ablauf::Weiter);
                        }
                        leser.borrow().neustarts()
                    };
                let geordnet = kopieren(&["d.txt", "c", "a"], true, &ziel, &mut steuerung);
                let ungeordnet = kopieren(&["c", "a"], false, &umgekehrt, &mut steuerung);
                (geordnet, ungeordnet, bericht(&steuerung))
            })
            .expect("Faden")
            .join()
            .expect("die Probe ist abgestuerzt")
        };
        let angekommen: Vec<Option<Vec<u8>>> = ["a/x.txt", "a/b/y.txt", "c/z.txt", "d.txt"]
            .iter()
            .map(|name| fs::read(ziel.join(name)).ok())
            .collect();
        let umgekehrt_angekommen = fs::read(umgekehrt.join("a/b/y.txt")).ok();
        abraeumen(&ordner);

        assert!(gemeldet.is_empty(), "{gemeldet:?}");
        assert_eq!(
            angekommen,
            [
                Some(b"x".to_vec()),
                Some(b"y".to_vec()),
                Some(b"z".to_vec()),
                Some(b"d".to_vec())
            ]
        );
        assert_eq!(umgekehrt_angekommen.as_deref(), Some(&b"y"[..]));
        assert_eq!(geordnet, 0, "der geordnete Baum hat den Strom neu begonnen");
        assert_eq!(ungeordnet, 1, "zwei Quellen in umgekehrter Folge");
    }
}
