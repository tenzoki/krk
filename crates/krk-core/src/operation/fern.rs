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
//! ueber `mkdir`, eine Verknuepfung ueber `symlink`. Scheitert das, fragt
//! `LSTAT` am Ziel, ob dort ein Eintrag steht, denn SFTP v3 kennt keinen
//! eigenen Code fuer „vorhanden“. Steht einer da, geht die Konfliktfrage ueber
//! [`Steuerung::konflikt_loesen_mit`] an dasselbe Blatt wie lokal;
//! „Ueberschreiben“ entfernt das Ziel endgueltig, wie `ziel_klaeren` es lokal
//! tut. Ordner auf Ordner ist kein Konflikt.
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
//!   am Pfad und nicht an der Naemlichkeit: ueber SFTP v3 gibt es weder Geraet
//!   noch Inode. Ein Ziel, das ueber eine Verknuepfung auf dem Server in der
//!   Quelle liegt, sieht es nicht.
//! - Erweiterte Attribute und Ressourcengabeln gehen auf dem Weg zum Server
//!   verloren; mit gehen allein Rechte und Aenderungszeit.
//! - Eine Verknuepfung legt es auf einem Server allein an, wenn er sich als
//!   OpenSSH zu erkennen gibt ([`Gespraech::spricht_openssh`]); sonst steht
//!   sie im Bericht.

use std::collections::VecDeque;
use std::ffi::OsStr;
use std::fs::{self, File, FileTimes, OpenOptions, Permissions};
use std::io::{self, Read, Write};
use std::ops::ControlFlow;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::fern::paket::{Attribute, Name, S_IFDIR, S_IFLNK, S_IFMT, S_IFREG, oeffnen, status};
use crate::fern::{
    self as fernzugriff, Ausstehend, BLOCK, Fernfaden, Fernfehler, Fernpfad, Gespraech, Handle,
    OFFENE_SCHREIBUNGEN, Serverkennung,
};
use crate::ort::Ort;
use crate::sprache::{Text, Zahlwort, anzahl, satz, text};
use crate::verzeichnis::sys::ohne_warten_oeffnen;
use crate::verzeichnis::{Typ, lesen};

use super::auftrag::Konfliktregel;
use super::fortschritt::{Abschluss, Bericht, Lauf, Meldung, Steuerung, Uebersprungen};
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
fn ausfuehren(pfand: &Fernfaden, auftrag: &Fernauftrag, steuerung: &mut Steuerung) -> Abschluss {
    let zielordner = auftrag.zielordner().map(Stelle::von);
    for quelle in &auftrag.quellen {
        if steuerung.abgebrochen() {
            return Abschluss::Abgebrochen;
        }
        let quelle = Stelle::von(quelle);
        if einen_abarbeiten(pfand, &auftrag.art, &quelle, zielordner.as_ref(), steuerung)
            == Ablauf::Abgebrochen
        {
            return Abschluss::Abgebrochen;
        }
    }
    if steuerung.abgebrochen() {
        Abschluss::Abgebrochen
    } else {
        Abschluss::Fertig
    }
}

/// Eine Quelle, nach der Art verzweigt. Vollstaendig ueber [`Fernart`] und
/// ohne Auffangzweig.
fn einen_abarbeiten(
    pfand: &Fernfaden,
    art: &Fernart,
    quelle: &Stelle,
    zielordner: Option<&Stelle>,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let verschieben = match art {
        Fernart::Kopieren { .. } => false,
        Fernart::Verschieben { .. } => true,
        Fernart::Umbenennen { neuer_name } => {
            return eintrag_umbenennen(pfand, quelle, neuer_name, steuerung);
        }
        Fernart::Anlegen { name, typ } => {
            return eintrag_anlegen(pfand, quelle, name, *typ, steuerung);
        }
        Fernart::Loeschen => return eintrag_loeschen(pfand, quelle, steuerung),
    };
    // `zielordner` steht genau dann, wenn die Art einen hat
    // ([`Fernauftrag::zielordner`]); der Fall ist trotzdem behandelt, wie die
    // fehlenden Angaben je Stelle in `super::einen_abarbeiten`.
    let Some(zielordner) = zielordner else {
        steuerung.ueberspringen(&quelle.bezeichnung(), text(Text::VorgangZielordnerFehlt));
        return Ablauf::Weiter;
    };
    einen_uebertragen(pfand, verschieben, quelle, zielordner, steuerung)
}

/// Eine Quelle kopieren oder verschieben: Angaben holen, Ziel rechnen, die
/// zwei Faelle abweisen, in denen es keines gibt, und dann verzweigen.
fn einen_uebertragen(
    pfand: &Fernfaden,
    verschieben: bool,
    quelle: &Stelle,
    zielordner: &Stelle,
    steuerung: &mut Steuerung,
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
    if quelle.derselbe_eintrag(&ziel) {
        steuerung.ueberspringen(
            &bezeichnung,
            text(Text::VorgangQuelleUndZielDerselbeEintrag),
        );
        return Ablauf::Weiter;
    }
    if angaben.typ == Typ::Ordner && zielordner.liegt_in(quelle) {
        steuerung.ueberspringen(&bezeichnung, text(Text::VorgangZielLiegtInDerQuelle));
        return Ablauf::Weiter;
    }
    if verschieben {
        eintrag_verschieben(pfand, quelle, &angaben, ziel, steuerung)
    } else {
        eintrag_kopieren(pfand, quelle, &angaben, ziel, steuerung)
    }
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
fn eintrag_loeschen(pfand: &Fernfaden, quelle: &Stelle, steuerung: &mut Steuerung) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    let angaben = match quelle.angaben(pfand) {
        Ok(angaben) => angaben,
        Err(fehler) => {
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return Ablauf::Weiter;
        }
    };
    match quelle.baum_entfernen(pfand) {
        Ok(()) => steuerung.eintrag_fertig(&bezeichnung, angaben.groesse),
        Err(fehler) => steuerung.ueberspringen(&bezeichnung, grund(&fehler)),
    }
    Ablauf::Weiter
}

/// Der Grund, wenn Umbenennen oder Anlegen scheitert.
///
/// **SFTP v3 kennt keinen Code fuer „vorhanden“**: der Server des Systems
/// antwortet auf ein vergebenes Ziel mit dem allgemeinen `FAILURE`. Gefragt
/// wird deshalb danach `LSTAT` am Ziel, wie beim Kopieren; steht dort ein
/// Eintrag, ist der Grund der von [`super::grund`] fuer `AlreadyExists`, also
/// genau der, den ein lokales Scheitern an einem vergebenen Namen traegt. Die
/// zwei Codes, die etwas anderes sagen, „gibt es nicht“ und „keine Rechte“,
/// behalten ihren eigenen Grund. Lokal sagt das System es selbst.
fn namensgrund(pfand: &Fernfaden, ziel: Option<&Stelle>, fehler: &Fehlschlag) -> String {
    let vergeben = match fehler {
        Fehlschlag::Fern(fern) => {
            !fern.ist_status(status::NO_SUCH_FILE)
                && !fern.ist_status(status::PERMISSION_DENIED)
                && ziel.is_some_and(|ziel| ziel.angaben(pfand).is_ok())
        }
        Fehlschlag::Lokal(_)
        | Fehlschlag::NameOhneUtf8
        | Fehlschlag::KeineGewoehnlicheDatei
        | Fehlschlag::KeineVerknuepfung
        | Fehlschlag::KeinEintrag
        | Fehlschlag::Abgebrochen => false,
    };
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
    /// Das Ziel ist entfernt; derselbe Name wird noch einmal angelegt.
    Ersetzt,
    /// Unter einem anderen Namen weiter.
    Nach(Stelle),
    /// Diese Quelle auslassen; sie steht schon im Bericht.
    Ueberspringen,
    /// Den ganzen Vorgang beenden.
    Abbrechen,
}

/// Kopiert einen Eintrag an sein Ziel, samt Konfliktbehandlung.
///
/// **Die Schleife endet**: nach „Ueberschreiben“ wird genau einmal neu
/// angelegt, und ein zweiter Konflikt danach laesst die Quelle aus; nach
/// „Umbenennen“ geht es nur unter einem anderen Namen weiter.
fn eintrag_kopieren(
    pfand: &Fernfaden,
    quelle: &Stelle,
    angaben: &Angaben,
    ziel: Stelle,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
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
    let mut ersetzt = false;
    loop {
        let fehler = match anlegen(pfand, &ziel, &anlage) {
            Ok(Angelegt::Datei(schreibziel)) => {
                return datei_fuellen(pfand, quelle, angaben, &ziel, schreibziel, steuerung);
            }
            Ok(Angelegt::Ordner) => {
                return ordner_fuellen(pfand, quelle, angaben, &ziel, steuerung);
            }
            Ok(Angelegt::Verknuepfung) => {
                steuerung.eintrag_fertig(&bezeichnung, 0);
                return Ablauf::Weiter;
            }
            Err(fehler) => fehler,
        };
        // Zwei Gruende, die kein vorhandenes Ziel meinen: der Server kann die
        // Verknuepfung nicht anlegen, oder ihr Verweis ist kein UTF-8.
        if fehler.ist_kein_konflikt() {
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return Ablauf::Weiter;
        }
        let Ok(vorhanden) = ziel.angaben(pfand) else {
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return Ablauf::Weiter;
        };
        if angaben.typ == Typ::Ordner && vorhanden.typ == Typ::Ordner {
            return ordner_fuellen(pfand, quelle, angaben, &ziel, steuerung);
        }
        if ersetzt {
            steuerung.ueberspringen(&bezeichnung, text(Text::VorgangAmZielStehtEintrag));
            return Ablauf::Weiter;
        }
        match konflikt_klaeren(pfand, &bezeichnung, &ziel, steuerung) {
            Klaerung::Ersetzt => ersetzt = true,
            Klaerung::Nach(neues) => {
                if neues.bezeichnung() == ziel.bezeichnung() {
                    steuerung.ueberspringen(&bezeichnung, text(Text::VorgangAmZielStehtEintrag));
                    return Ablauf::Weiter;
                }
                ziel = neues;
                ersetzt = false;
            }
            Klaerung::Ueberspringen => return Ablauf::Weiter,
            Klaerung::Abbrechen => return Ablauf::Abgebrochen,
        }
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

/// Fragt, was mit einem vorhandenen Ziel geschieht, und fuehrt die Antwort
/// aus, soweit sie am Ziel wirkt.
///
/// Dieselben vier Antworten wie lokal in `super::ziel_klaeren`, und
/// „Ueberschreiben“ entfernt endgueltig, bei einem Ordner samt Inhalt; das
/// Konfliktblatt sagt das schon heute (`operationen::ersetzungsweg` in der
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
        super::Konfliktantwort::Ueberschreiben => match ziel.baum_entfernen(pfand) {
            Ok(()) => Klaerung::Ersetzt,
            Err(fehler) => {
                steuerung.ueberspringen(
                    quelle,
                    satz(Text::VorgangZielNichtErsetzt, &[("grund", &grund(&fehler))]),
                );
                Klaerung::Ueberspringen
            }
        },
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
/// und Aenderungszeit.
///
/// **Ein abgebrochenes oder gescheitertes Ziel wird entfernt**: die halbe
/// Datei ist kein Ergebnis, sondern ein Rest, wie in
/// `kopieren::datei_uebertragen`. Das Ziel hat dieser Lauf selbst angelegt,
/// ausschliessend; entfernt wird also nie ein Eintrag, der vorher dastand.
fn datei_fuellen(
    pfand: &Fernfaden,
    quelle: &Stelle,
    angaben: &Angaben,
    ziel: &Stelle,
    schreibziel: Schreibziel,
    steuerung: &mut Steuerung,
) -> Ablauf {
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
        Ok(()) => {}
        Err(Fehlschlag::Abgebrochen) => {
            steuerung.teilstueck(bytes);
            halbes_ziel_wegraeumen(pfand, ziel, steuerung);
            return Ablauf::Abgebrochen;
        }
        Err(fehler) => {
            halbes_ziel_wegraeumen(pfand, ziel, steuerung);
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return Ablauf::Weiter;
        }
    }

    if let Err(fehler) = ziel.angaben_setzen(pfand, angaben) {
        steuerung.ueberspringen(
            &ziel.bezeichnung(),
            satz(
                Text::VorgangDateiangabenNichtKopiert,
                &[("grund", &grund(&fehler))],
            ),
        );
    }
    steuerung.eintrag_fertig(&bezeichnung, bytes);
    Ablauf::Weiter
}

/// Entfernt die halbe Zieldatei; scheitert das, steht es im Bericht.
fn halbes_ziel_wegraeumen(pfand: &Fernfaden, ziel: &Stelle, steuerung: &mut Steuerung) {
    if let Err(fehler) = ziel.datei_entfernen(pfand)
        && !fehler.ist_nicht_da()
    {
        steuerung.ueberspringen(
            &ziel.bezeichnung(),
            satz(
                Text::VorgangNachAbbruchNichtWeggeraeumt,
                &[("grund", &grund(&fehler))],
            ),
        );
    }
}

/// Kopiert den Inhalt eines Ordners in einen Ordner am Ziel, der eben
/// angelegt wurde oder schon dastand, dann Rechte und Aenderungszeit.
fn ordner_fuellen(
    pfand: &Fernfaden,
    quelle: &Stelle,
    angaben: &Angaben,
    ziel: &Stelle,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    let Some(kinder) = kinder_lesen(pfand, quelle, steuerung) else {
        return Ablauf::Weiter;
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
        let Some(unterangaben) = unterangaben_holen(pfand, &unterquelle, unterangaben, steuerung)
        else {
            continue;
        };
        if eintrag_kopieren(pfand, &unterquelle, &unterangaben, unterziel, steuerung)
            == Ablauf::Abgebrochen
        {
            return Ablauf::Abgebrochen;
        }
    }

    if let Err(fehler) = ziel.angaben_setzen(pfand, angaben) {
        steuerung.ueberspringen(
            &ziel.bezeichnung(),
            satz(
                Text::VorgangOrdnerangabenNichtKopiert,
                &[("grund", &grund(&fehler))],
            ),
        );
    }
    steuerung.eintrag_fertig(&bezeichnung, 0);
    Ablauf::Weiter
}

/// Die Kinder eines Ordners als Stellen, mit den Angaben, die das Lesen schon
/// mitbrachte. `None` heisst: der Ordner liess sich nicht lesen, und das
/// steht im Bericht. Namen ohne UTF-8 kommen gezaehlt in den Bericht, weil
/// sich kein Pfad auf dem Server zu ihnen bauen laesst.
fn kinder_lesen(
    pfand: &Fernfaden,
    quelle: &Stelle,
    steuerung: &mut Steuerung,
) -> Option<Vec<(Stelle, Option<Angaben>)>> {
    let bezeichnung = quelle.bezeichnung();
    let liste = match quelle.liste(pfand) {
        Ok(liste) => liste,
        Err(fehler) => {
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return None;
        }
    };
    if liste.ohne_utf8 > 0 {
        steuerung.ueberspringen(
            &bezeichnung,
            anzahl(Zahlwort::FernNamenUebersprungen, liste.ohne_utf8, &[]),
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
    Some(kinder)
}

/// Die Angaben eines Kindes: die mitgebrachten, sonst ein eigenes `LSTAT`.
fn unterangaben_holen(
    pfand: &Fernfaden,
    kind: &Stelle,
    mitgebracht: Option<Angaben>,
    steuerung: &mut Steuerung,
) -> Option<Angaben> {
    if let Some(angaben) = mitgebracht {
        return Some(angaben);
    }
    match kind.angaben(pfand) {
        Ok(angaben) => Some(angaben),
        Err(fehler) => {
            steuerung.ueberspringen(&kind.bezeichnung(), grund(&fehler));
            None
        }
    }
}

// ----------------------------------------------------------------------
// Verschieben
// ----------------------------------------------------------------------

/// Verschiebt einen Eintrag an sein Ziel, samt Konfliktbehandlung.
///
/// **Auf demselben Server ist es ein `RENAME`** und nicht
/// `posix-rename@openssh.com` (Entscheidung 10); `RENAME` scheitert an einem
/// vorhandenen Ziel, und dann geht dieselbe Konfliktfrage wie beim Kopieren.
/// Ordner auf Ordner wird Kind fuer Kind zusammengefuehrt. **Ueber eine
/// Grenze**, zwischen Geraet und Server oder zwischen zwei Servern, wird
/// kopiert und danach die Quelle entfernt, allein wenn nichts uebersprungen
/// wurde: die Regel von `verschieben::ueber_datentraeger`.
fn eintrag_verschieben(
    pfand: &Fernfaden,
    quelle: &Stelle,
    angaben: &Angaben,
    ziel: Stelle,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    let mut ziel = ziel;
    let mut ersetzt = false;
    loop {
        let fehler = match quelle.umbenennen(pfand, &ziel) {
            None => return ueber_die_grenze(pfand, quelle, angaben, ziel, steuerung),
            Some(Ok(())) => {
                steuerung.eintrag_fertig(&bezeichnung, angaben.groesse);
                return Ablauf::Weiter;
            }
            Some(Err(fehler)) => fehler,
        };
        let Ok(vorhanden) = ziel.angaben(pfand) else {
            steuerung.ueberspringen(&bezeichnung, grund(&fehler));
            return Ablauf::Weiter;
        };
        if angaben.typ == Typ::Ordner && vorhanden.typ == Typ::Ordner {
            return verschmelzen(pfand, quelle, &ziel, steuerung);
        }
        if ersetzt {
            steuerung.ueberspringen(&bezeichnung, text(Text::VorgangAmZielStehtEintrag));
            return Ablauf::Weiter;
        }
        match konflikt_klaeren(pfand, &bezeichnung, &ziel, steuerung) {
            Klaerung::Ersetzt => ersetzt = true,
            Klaerung::Nach(neues) => {
                if neues.bezeichnung() == ziel.bezeichnung() {
                    steuerung.ueberspringen(&bezeichnung, text(Text::VorgangAmZielStehtEintrag));
                    return Ablauf::Weiter;
                }
                ziel = neues;
                ersetzt = false;
            }
            Klaerung::Ueberspringen => return Ablauf::Weiter,
            Klaerung::Abbrechen => return Ablauf::Abgebrochen,
        }
    }
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
    let Some(kinder) = kinder_lesen(pfand, quelle, steuerung) else {
        return Ablauf::Weiter;
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
        let Some(unterangaben) = unterangaben_holen(pfand, &unterquelle, unterangaben, steuerung)
        else {
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
/// wenn seit dem Beginn dieses Eintrags nichts uebersprungen wurde.
///
/// Der Zeuge ist derselbe wie in `verschieben::ueber_datentraeger`: jeder Weg
/// in diesem Modul, der ohne Ankunft am Ziel endet, ruft
/// `Steuerung::ueberspringen`.
fn ueber_die_grenze(
    pfand: &Fernfaden,
    quelle: &Stelle,
    angaben: &Angaben,
    ziel: Stelle,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let bezeichnung = quelle.bezeichnung();
    let stand = steuerung.uebersprungen_stand();
    if eintrag_kopieren(pfand, quelle, angaben, ziel, steuerung) == Ablauf::Abgebrochen {
        return Ablauf::Abgebrochen;
    }
    let seither = steuerung.uebersprungen_seit(stand);
    if !seither.is_empty() {
        let selbst_genannt = seither.iter().any(|eintrag| eintrag.pfad == bezeichnung);
        if !selbst_genannt {
            steuerung.ueberspringen(&bezeichnung, text(Text::VorgangNichtVollstaendigKopiert));
        }
        return Ablauf::Weiter;
    }
    if let Err(fehler) = quelle.baum_entfernen(pfand) {
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
    /// Der Nutzer hat abgebrochen.
    Abgebrochen,
}

impl Fehlschlag {
    /// Ob der Eintrag fehlt, an dem gefragt wurde.
    fn ist_nicht_da(&self) -> bool {
        match self {
            Fehlschlag::Lokal(fehler) => fehler.kind() == io::ErrorKind::NotFound,
            Fehlschlag::Fern(fehler) => fehler.ist_status(status::NO_SUCH_FILE),
            Fehlschlag::NameOhneUtf8
            | Fehlschlag::KeineGewoehnlicheDatei
            | Fehlschlag::KeineVerknuepfung
            | Fehlschlag::KeinEintrag
            | Fehlschlag::Abgebrochen => false,
        }
    }

    /// Ob das Scheitern beim Anlegen sicher kein vorhandenes Ziel meint.
    fn ist_kein_konflikt(&self) -> bool {
        match self {
            Fehlschlag::NameOhneUtf8
            | Fehlschlag::KeineGewoehnlicheDatei
            | Fehlschlag::KeineVerknuepfung
            | Fehlschlag::KeinEintrag
            | Fehlschlag::Abgebrochen => true,
            Fehlschlag::Lokal(_) | Fehlschlag::Fern(_) => false,
        }
    }
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
        // Ein Abbruch endet den Vorgang und kommt nicht in die Abschlussliste;
        // der Zweig steht, weil die Unterscheidung vollstaendig ist.
        Fehlschlag::Abgebrochen => Fernfehler::Abgebrochen.to_string(),
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
    ohne_utf8: u64,
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
    /// ein spaeterer Wiederaufbau erreicht diesen Lauf nicht mehr.
    fn von(ort: &Ort) -> Self {
        match ort {
            Ort::Lokal(pfad) => Stelle::Lokal(pfad.clone()),
            Ort::Fern(fern) => Stelle::Fern {
                gespraech: fern.zugang.gespraech(),
                kennung: fern.kennung().clone(),
                pfad: fern.pfad.clone(),
            },
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
    /// Naemlichkeit wie `super::zielpfad`, fern am Pfad auf demselben Server.
    fn derselbe_eintrag(&self, andere: &Stelle) -> bool {
        match (self, andere) {
            (Stelle::Lokal(eine), Stelle::Lokal(andere)) => {
                benennen_denselben_eintrag(eine, andere)
            }
            (
                Stelle::Fern { kennung, pfad, .. },
                Stelle::Fern {
                    kennung: andere_kennung,
                    pfad: anderer_pfad,
                    ..
                },
            ) => kennung == andere_kennung && pfad == anderer_pfad,
            (Stelle::Lokal(_), Stelle::Fern { .. }) | (Stelle::Fern { .. }, Stelle::Lokal(_)) => {
                false
            }
        }
    }

    /// Ob dieser Ordner der Ordner `quelle` ist oder unter ihm liegt.
    fn liegt_in(&self, quelle: &Stelle) -> bool {
        match (self, quelle) {
            (Stelle::Lokal(ordner), Stelle::Lokal(quelle)) => liegt_im_ordner(ordner, quelle),
            (
                Stelle::Fern { kennung, pfad, .. },
                Stelle::Fern {
                    kennung: quellkennung,
                    pfad: quellpfad,
                    ..
                },
            ) => {
                kennung == quellkennung
                    && (quellpfad.ist_wurzel()
                        || pfad == quellpfad
                        || pfad
                            .als_text()
                            .strip_prefix(quellpfad.als_text())
                            .is_some_and(|rest| rest.starts_with('/')))
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
    /// Angaben gleich mitbringen.
    fn liste(&self, pfand: &Fernfaden) -> Result<Liste, Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => {
                let eintraege = lesen(pfad).map_err(Fehlschlag::Lokal)?;
                Ok(Liste {
                    kinder: eintraege
                        .into_iter()
                        .map(|eintrag| (eintrag.name, None))
                        .collect(),
                    ohne_utf8: 0,
                })
            }
            Stelle::Fern {
                gespraech, pfad, ..
            } => {
                let namen = fern_namen(pfand, gespraech, pfad).map_err(Fehlschlag::Fern)?;
                let mut liste = Liste {
                    kinder: Vec::with_capacity(namen.len()),
                    ohne_utf8: 0,
                };
                for eintrag in namen {
                    let Ok(name) = String::from_utf8(eintrag.name) else {
                        liste.ohne_utf8 += 1;
                        continue;
                    };
                    if name == "." || name == ".." {
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

    /// Entfernt den Eintrag endgueltig, einen Ordner samt Inhalt.
    fn baum_entfernen(&self, pfand: &Fernfaden) -> Result<(), Fehlschlag> {
        match self {
            Stelle::Lokal(pfad) => loeschen::baum_entfernen(pfad).map_err(Fehlschlag::Lokal),
            Stelle::Fern {
                gespraech, pfad, ..
            } => fern_baum_entfernen(pfand, gespraech, pfad).map_err(Fehlschlag::Fern),
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
fn fern_namen(
    pfand: &Fernfaden,
    gespraech: &Gespraech,
    pfad: &Fernpfad,
) -> Result<Vec<Name>, Fernfehler> {
    let handle = gespraech.opendir(pfand, pfad.als_text())?;
    let mut namen = Vec::new();
    let gelesen = loop {
        match gespraech.readdir(pfand, &handle) {
            Ok(Some(stapel)) => namen.extend(stapel),
            Ok(None) => break Ok(()),
            Err(fehler) => break Err(fehler),
        }
    };
    // Ein gescheitertes Schliessen nach dem Lesen aendert nichts an den
    // gelesenen Namen.
    let _ = gespraech.close(pfand, handle);
    gelesen.map(|()| namen)
}

/// Entfernt einen Eintrag auf dem Server endgueltig, einen Ordner samt
/// Inhalt, und haelt am ersten Fehler an, wie `loeschen::baum_entfernen`.
fn fern_baum_entfernen(
    pfand: &Fernfaden,
    gespraech: &Gespraech,
    pfad: &Fernpfad,
) -> Result<(), Fernfehler> {
    let angaben = gespraech.lstat(pfand, pfad.als_text())?;
    if !angaben.ist_ordner() {
        return gespraech.remove(pfand, pfad.als_text());
    }
    for eintrag in fern_namen(pfand, gespraech, pfad)? {
        // Ein Name ohne UTF-8 laesst sich nicht ansteuern; er bleibt stehen,
        // und das `RMDIR` darunter scheitert dann mit seinem Grund.
        let Ok(name) = String::from_utf8(eintrag.name) else {
            continue;
        };
        let Some(kind) = pfad.verbinden(&name) else {
            continue;
        };
        fern_baum_entfernen(pfand, gespraech, &kind)?;
    }
    gespraech.rmdir(pfand, pfad.als_text())
}
