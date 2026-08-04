//! Kopieren, ueber `copyfile(3)` und den Verzeichnisleser aus Schritt 2.
//!
//! ```text
//! eintrag_kopieren ──> ziel_klaeren (Konflikt)
//!                  ──> kopieren_nach ──> Typ::Datei        ──> sys::datei_kopieren
//!                                    ──> Typ::Ordner       ──> verzeichnis::lesen ──┐
//!                                    ──> Typ::Verknuepfung ──> fs::symlink          │
//!                                    <──────────────────────── je Eintrag ──────────┘
//! ```
//!
//! **Der Abstieg laeuft ueber den vorhandenen Leser.** `copyfile(3)` kennt ein
//! `COPYFILE_RECURSIVE` und koennte einen Ordner selbst durchlaufen. Genau das
//! tut es hier nicht: der Leser aus Schritt 2 ist die eine Auskunft darueber,
//! was in einem Ordner steht, und ein zweiter Durchlauf daneben waere eine
//! zweite. Ausserdem meldet der eigene Abstieg je Eintrag, laesst sich zwischen
//! zwei Eintraegen abbrechen und kann eine gescheiterte Einzelposition
//! ueberspringen; `COPYFILE_RECURSIVE` kann keines der drei.
//!
//! [`Typ`] entscheidet, was mit einem Eintrag geschieht. Einer Verknuepfung
//! folgt KRK nicht: kopiert wird die Verknuepfung, nicht ihr Ziel. Wer einem
//! Verweis folgte, kopierte einen Ordner doppelt, sobald er auf sich selbst
//! zeigt.

use std::fs::{self, File, FileTimes};
use std::io;
use std::path::Path;

use crate::verzeichnis::sys::{Uebertragungsart, Weiter, datei_kopieren as sys_datei_kopieren};
use crate::verzeichnis::{Typ, lesen};

use super::fortschritt::Steuerung;
use super::{Ablauf, Quelle, Zielentscheid, grund, ziel_klaeren};

/// Kopiert einen Eintrag an sein Ziel, samt Konfliktbehandlung.
pub(crate) fn eintrag_kopieren(
    quelle: &Quelle<'_>,
    ziel: &Path,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> Ablauf {
    match ziel_klaeren(quelle, ziel, steuerung) {
        Zielentscheid::Nach(geklaertes_ziel) => {
            kopieren_nach(quelle, &geklaertes_ziel, art, steuerung)
        }
        Zielentscheid::Ueberspringen => Ablauf::Weiter,
        Zielentscheid::Abbrechen => Ablauf::Abgebrochen,
    }
}

/// Kopiert einen Eintrag an ein bereits geklaertes Ziel.
///
/// Getrennt von [`eintrag_kopieren`], weil das Verschieben ueber
/// Datentraegergrenzen hinweg denselben Weg braucht, seinen Konflikt aber schon
/// geklaert hat.
pub(crate) fn kopieren_nach(
    quelle: &Quelle<'_>,
    ziel: &Path,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> Ablauf {
    match quelle.typ {
        Typ::Datei => datei(quelle, ziel, art, steuerung),
        Typ::Ordner => ordner(quelle, ziel, art, steuerung),
        Typ::Verknuepfung => verknuepfung(quelle, ziel, steuerung),
    }
}

/// Kopiert eine einzelne Datei.
fn datei(
    quelle: &Quelle<'_>,
    ziel: &Path,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let pfad = quelle.pfad;
    let ergebnis = {
        let mut melden = |bytes: u64| {
            steuerung.zwischenstand(pfad, bytes);
            if steuerung.abgebrochen() {
                Weiter::Abbrechen
            } else {
                Weiter::Weitermachen
            }
        };
        sys_datei_kopieren(pfad, ziel, art, &mut melden)
    };

    match ergebnis {
        Ok(kopie) if kopie.abgebrochen => {
            steuerung.teilstueck(kopie.bytes);
            // Die halbe Datei am Ziel ist kein Ergebnis, sondern ein Rest. Wer
            // sie stehen liesse, hinterliesse dem Nutzer eine Datei, die
            // aussieht wie seine und es nicht ist.
            if let Err(fehler) = fs::remove_file(ziel)
                && fehler.kind() != io::ErrorKind::NotFound
            {
                steuerung.ueberspringen(
                    ziel,
                    format!("nach dem Abbruch nicht weggeraeumt: {}", grund(&fehler)),
                );
            }
            Ablauf::Abgebrochen
        }
        Ok(kopie) => {
            // Ein Klon bewegt keine Bytes. Uebertragen ist der Inhalt der Datei
            // trotzdem, und die Zahl im Fortschritt meint den Inhalt.
            let bytes = if kopie.geklont {
                quelle.groesse
            } else {
                kopie.bytes
            };
            steuerung.eintrag_fertig(pfad, bytes);
            Ablauf::Weiter
        }
        Err(fehler) => {
            steuerung.ueberspringen(pfad, grund(&fehler));
            Ablauf::Weiter
        }
    }
}

/// Kopiert einen Ordner samt Inhalt, Eintrag fuer Eintrag.
fn ordner(
    quelle: &Quelle<'_>,
    ziel: &Path,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> Ablauf {
    if let Err(fehler) = fs::create_dir(ziel)
        && fehler.kind() != io::ErrorKind::AlreadyExists
    {
        steuerung.ueberspringen(quelle.pfad, grund(&fehler));
        return Ablauf::Weiter;
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
        let unterquelle = quelle.pfad.join(&eintrag.name);
        let unterziel = ziel.join(&eintrag.name);
        let kind = Quelle {
            pfad: &unterquelle,
            typ: eintrag.typ,
            groesse: eintrag.groesse,
        };
        if eintrag_kopieren(&kind, &unterziel, art, steuerung) == Ablauf::Abgebrochen {
            return Ablauf::Abgebrochen;
        }
    }

    if let Err(fehler) = ordnerangaben_uebernehmen(quelle.pfad, ziel) {
        steuerung.ueberspringen(
            ziel,
            format!(
                "Inhalt kopiert, Rechte und Datum des Ordners nicht: {}",
                grund(&fehler)
            ),
        );
    }
    steuerung.eintrag_fertig(quelle.pfad, 0);
    Ablauf::Weiter
}

/// Kopiert eine symbolische Verknuepfung, nicht ihr Ziel.
fn verknuepfung(quelle: &Quelle<'_>, ziel: &Path, steuerung: &mut Steuerung) -> Ablauf {
    let ergebnis =
        fs::read_link(quelle.pfad).and_then(|verweis| std::os::unix::fs::symlink(verweis, ziel));
    match ergebnis {
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

/// Uebertraegt Rechte und Aenderungsdatum eines Ordners.
///
/// Erst **nach** dem Inhalt: ein Ordner, dessen Rechte vorher gesetzt werden,
/// laesst sich unter Umstaenden nicht mehr befuellen.
fn ordnerangaben_uebernehmen(quelle: &Path, ziel: &Path) -> io::Result<()> {
    let angaben = fs::metadata(quelle)?;
    let zeiten = FileTimes::new()
        .set_modified(angaben.modified()?)
        .set_accessed(angaben.accessed()?);
    File::open(ziel)?.set_times(zeiten)?;
    fs::set_permissions(ziel, angaben.permissions())
}
