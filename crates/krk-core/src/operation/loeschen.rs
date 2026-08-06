//! Loeschen: in den Papierkorb des Systems und endgueltig.
//!
//! Zwei Wege, wie es der Nutzer am 260802-1105 entschieden hat
//! (`shared/decisions/260802-0842_*_loeschen-papierkorb-oder-endgueltig.md`):
//! Delete raeumt in den Papierkorb, F8 loescht endgueltig. Die Rueckfrage vor
//! dem endgueltigen Loeschen stellt die Oberflaeche, genau einmal je Vorgang;
//! der Kern bekommt den Auftrag erst danach.
//!
//! # Die eine Abhaengigkeitsumkehr des Entwurfs
//!
//! Der Papierkorb ist `NSFileManager.trashItemAtURL:` und damit ein
//! AppKit-Aufruf. `krk-core` kennt AppKit nicht. Deshalb steht hier die
//! **Schnittstelle** [`Papierkorb`], und die Huelle um den Aufruf steht in
//! `krk-ui/src/appkit/papierkorb.rs` und wird hereingereicht:
//!
//! ```text
//!   krk-core::operation::loeschen        krk-ui::appkit::papierkorb
//!   ─────────────────────────────        ──────────────────────────
//!   trait Papierkorb            <────────  impl Papierkorb for Systempapierkorb
//!        ^                                        (NSFileManager)
//!        └── die Maschine ruft
//! ```
//!
//! Der Aufruf laeuft damit von unten nach oben, die Uebersetzungsabhaengigkeit
//! weiterhin von oben nach unten: `krk-core` nennt keine `objc2`-Kiste. Ein
//! eigener Rueckgaengig-Speicher entsteht nicht; der Rueckweg ist der
//! Papierkorb des Systems (C4).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::verzeichnis::{Typ, lesen};

use super::fortschritt::Steuerung;
use super::{Ablauf, Quelle, grund};

/// Der Papierkorb des Systems, aus der Sicht des Kerns.
///
/// Die einzige Schnittstelle, die `krk-core` sich hereinreichen laesst. Sie
/// nimmt einen Pfad und liefert den Ort, an dem der Eintrag im Papierkorb
/// gelandet ist.
pub trait Papierkorb: Send + Sync {
    /// Raeumt den Eintrag in den Papierkorb des Systems.
    ///
    /// Der Rueckgabewert ist der neue Ort. Er wird heute nicht angezeigt; er
    /// steht in der Signatur, weil das System ihn ohnehin liefert und eine
    /// Meldung "liegt jetzt unter …" ohne ihn nicht moeglich waere.
    fn in_den_papierkorb(&self, pfad: &Path) -> io::Result<PathBuf>;
}

/// Ein Papierkorb, den es nicht gibt.
///
/// Fuer jeden Aufrufer ohne Oberflaeche: fuer die Tests des Kerns und fuer
/// Auftraege, die den Papierkorb gar nicht brauchen. Er scheitert, statt
/// stillschweigend endgueltig zu loeschen. Ein Papierkorb, der bei fehlender
/// Anbindung zur Loeschung wird, waere der schlimmste denkbare Rueckfallweg.
#[derive(Debug, Clone, Copy, Default)]
pub struct OhnePapierkorb;

impl Papierkorb for OhnePapierkorb {
    fn in_den_papierkorb(&self, _pfad: &Path) -> io::Result<PathBuf> {
        Err(io::Error::other(
            "kein Papierkorb eingehaengt; es wurde nichts geloescht",
        ))
    }
}

/// Raeumt einen Eintrag in den Papierkorb.
///
/// Ordner mit Inhalt eingeschlossen: das System nimmt den ganzen Baum in einem
/// Zug, und ein eigener Abstieg waere hier weder noetig noch richtig.
pub(crate) fn in_den_papierkorb(
    quelle: &Quelle<'_>,
    papierkorb: &dyn Papierkorb,
    steuerung: &mut Steuerung,
) -> Ablauf {
    match papierkorb.in_den_papierkorb(quelle.pfad) {
        Ok(_) => {
            steuerung.eintrag_fertig(quelle.pfad, quelle.groesse);
            Ablauf::Weiter
        }
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            Ablauf::Weiter
        }
    }
}

/// Loescht einen Eintrag endgueltig, rekursiv und mit Bericht.
///
/// Der Abstieg laeuft ueber den Leser aus Schritt 2, aus demselben Grund wie
/// beim Kopieren: er ist die eine Auskunft darueber, was in einem Ordner steht.
/// `fs::remove_dir_all` waere die zweite und koennte weder abbrechen noch eine
/// einzelne gescheiterte Position ueberspringen.
pub(crate) fn endgueltig_loeschen(quelle: &Quelle<'_>, steuerung: &mut Steuerung) -> Ablauf {
    if quelle.typ != Typ::Ordner {
        return match fs::remove_file(quelle.pfad) {
            Ok(()) => {
                steuerung.eintrag_fertig(quelle.pfad, quelle.groesse);
                Ablauf::Weiter
            }
            Err(fehler) => {
                steuerung.ueberspringen(quelle.pfad, grund(&fehler));
                Ablauf::Weiter
            }
        };
    }

    let eintraege = match lesen(quelle.pfad) {
        Ok(eintraege) => eintraege,
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            return Ablauf::Weiter;
        }
    };

    for eintrag in eintraege {
        if steuerung.abgebrochen() {
            return Ablauf::Abgebrochen;
        }
        let unterpfad = quelle.pfad.join(&eintrag.name);
        let kind = Quelle {
            pfad: &unterpfad,
            typ: eintrag.typ,
            groesse: eintrag.groesse,
        };
        if endgueltig_loeschen(&kind, steuerung) == Ablauf::Abgebrochen {
            return Ablauf::Abgebrochen;
        }
    }

    match fs::remove_dir(quelle.pfad) {
        Ok(()) => {
            steuerung.eintrag_fertig(quelle.pfad, 0);
            Ablauf::Weiter
        }
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            Ablauf::Weiter
        }
    }
}

/// Raeumt einen Baum weg, ohne Bericht und ohne Abbruch.
///
/// Fuer die beiden Stellen, an denen ein Loeschen kein Auftrag des Nutzers ist,
/// sondern ein Schritt in einem anderen: das Ersetzen eines vorhandenen Ziels
/// und das Verschieben ueber Datentraegergrenzen hinweg. Beide Aufrufer wollen
/// nicht zaehlen, sondern wissen, ob es geklappt hat, und melden einen Fehler
/// als Grund an ihrem eigenen Eintrag.
///
/// Scheitert beim ersten Fehler, statt weiterzumachen: ein halb weggeraeumtes
/// Ziel, in das dann kopiert wird, waere aus zwei Baeumen einer.
pub(crate) fn baum_entfernen(pfad: &Path) -> io::Result<()> {
    let angaben = fs::symlink_metadata(pfad)?;
    if !angaben.is_dir() {
        return fs::remove_file(pfad);
    }
    for eintrag in lesen(pfad)? {
        baum_entfernen(&pfad.join(&eintrag.name))?;
    }
    fs::remove_dir(pfad)
}
