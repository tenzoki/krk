//! Kopieren, ueber `copyfile(3)` und den Verzeichnisleser aus Schritt 2.
//!
//! ```text
//! eintrag_kopieren ──> ziel_klaeren (Konflikt)
//!                  ──> kopieren_nach ──> Typ::Datei        ──> datei ──> datei_uebertragen
//!                                    ──> Typ::Ordner       ──> verzeichnis::lesen ──┐
//!                                    ──> Typ::Verknuepfung ──> fs::symlink          │
//!                                    <──────────────────────── je Eintrag ──────────┘
//!
//! duplizieren::eintrag_duplizieren ──────────────────────────> datei_uebertragen
//!                                                                     │
//!                                                            sys::datei_kopieren
//! ```
//!
//! **Die Uebertragung einer Datei steht einmal da und hat zwei Rufer.**
//! [`datei_uebertragen`] traegt Fortschritt, Abbruch und das Wegraeumen der
//! halben Zieldatei und gibt einen Fehler von `copyfile(3)` an seinen Rufer
//! zurueck, statt ihn zu verbuchen. Das Kopieren verbucht ihn als
//! uebersprungenen Eintrag; das Duplizieren liest an ihm ab, ob der Name
//! vergeben war, und fragt dann nach einem anderen. Einen zweiten Kopierweg
//! daneben gibt es nicht.
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

use crate::sprache::{Text, satz};
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
///
/// Die Arbeit selbst ist [`datei_uebertragen`]; hier kommt dazu, was das
/// Kopieren mit einem Fehler von `copyfile(3)` tut: es verbucht ihn als
/// uebersprungenen Eintrag, und der Stapel laeuft weiter.
fn datei(
    quelle: &Quelle<'_>,
    ziel: &Path,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> Ablauf {
    match datei_uebertragen(quelle, ziel, art, steuerung) {
        Ok(ablauf) => ablauf,
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            Ablauf::Weiter
        }
    }
}

/// Uebertraegt eine einzelne Datei an ein Ziel, das es noch nicht geben darf.
///
/// **Der eine Rumpf fuer das Kopieren und das Duplizieren.** Er meldet den
/// Fortschritt, reicht den Abbruch in den Statusrueckruf von `copyfile(3)`,
/// raeumt nach einem Abbruch die halbe Zieldatei weg und verbucht die fertige
/// Datei.
///
/// **Einen Fehler von `copyfile(3)` verbucht er nicht, er gibt ihn zurueck.**
/// Was ein Fehler bedeutet, weiss der Rufer: [`datei`] ueberspringt den Eintrag
/// mit seinem Grund, `duplizieren::eintrag_duplizieren` fragt bei
/// [`io::ErrorKind::AlreadyExists`] nach einem anderen Namen. Bis zum
/// Duplizieren stand die Verbuchung hier mit im Rumpf, und ob der Grund "am
/// Ziel steht schon ein Eintrag" war, liess sich danach nur noch am Text des
/// Berichts ablesen.
///
/// **Angelegt wird ausschliessend.** `sys::datei_kopieren` setzt
/// `COPYFILE_EXCL` in jeder [`Uebertragungsart`]; ein vorhandenes Ziel laesst
/// den Aufruf scheitern, bevor ein Byte geschrieben ist. Die halbe Zieldatei,
/// die nach einem Abbruch faellt, hat deshalb dieser Aufruf selbst angelegt und
/// nie ein anderer.
pub(crate) fn datei_uebertragen(
    quelle: &Quelle<'_>,
    ziel: &Path,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> io::Result<Ablauf> {
    let pfad = quelle.pfad;
    let kopie = {
        let mut melden = |bytes: u64| {
            steuerung.zwischenstand(pfad, bytes);
            if steuerung.abgebrochen() {
                Weiter::Abbrechen
            } else {
                Weiter::Weitermachen
            }
        };
        sys_datei_kopieren(pfad, ziel, art, &mut melden)
    }?;

    if kopie.abgebrochen {
        steuerung.teilstueck(kopie.bytes);
        // Die halbe Datei am Ziel ist kein Ergebnis, sondern ein Rest. Wer
        // sie stehen liesse, hinterliesse dem Nutzer eine Datei, die
        // aussieht wie seine und es nicht ist.
        if let Err(fehler) = fs::remove_file(ziel)
            && fehler.kind() != io::ErrorKind::NotFound
        {
            steuerung.ueberspringen(
                ziel,
                satz(
                    Text::VorgangNachAbbruchNichtWeggeraeumt,
                    &[("grund", &grund(&fehler))],
                ),
            );
        }
        return Ok(Ablauf::Abgebrochen);
    }

    // Ein Klon bewegt keine Bytes. Uebertragen ist der Inhalt der Datei
    // trotzdem, und die Zahl im Fortschritt meint den Inhalt.
    let bytes = if kopie.geklont {
        quelle.groesse
    } else {
        kopie.bytes
    };
    steuerung.eintrag_fertig(pfad, bytes);
    Ok(Ablauf::Weiter)
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
            satz(
                Text::VorgangOrdnerangabenNichtKopiert,
                &[("grund", &grund(&fehler))],
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
