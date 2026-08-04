//! Verschieben: `rename(2)` innerhalb eines Datentraegers, sonst kopieren und
//! loeschen.
//!
//! ```text
//! eintrag_verschieben ──> ziel_klaeren (Konflikt)
//!                     ──> Ordner auf Ordner ──> verschmelzen, Kind fuer Kind
//!                     ──> sonst ──> renamex_np ──> fertig (ein Systemaufruf)
//!                                            └──> EXDEV ──> kopieren + loeschen
//! ```
//!
//! **Innerhalb eines Datentraegers ist ein Verschieben ein Systemaufruf, und
//! zwar genau einer, gleich wie gross die Datei ist.** `rename(2)` haengt einen
//! Verzeichniseintrag um; die Daten bleiben liegen. Deshalb dauert das
//! Verschieben einer 200-MB-Datei nicht laenger als das einer leeren.
//!
//! Ueber Datentraegergrenzen hinweg gibt es diesen Weg nicht. Kopieren mit
//! anschliessendem Loeschen ist dort kein Rueckfallweg im Sinne der Maxime
//! "supersimpel", sondern die einzige Art, wie ein Verschieben zwischen zwei
//! Datentraegern ueberhaupt geht.

use std::path::Path;

use crate::verzeichnis::sys::{EXDEV, Uebertragungsart, im_datentraeger_verschieben};
use crate::verzeichnis::{Typ, lesen};

use super::fortschritt::Steuerung;
use super::{Ablauf, Quelle, Zielentscheid, grund, kopieren, loeschen, ziel_klaeren};

/// Verschiebt einen Eintrag an sein Ziel, samt Konfliktbehandlung.
pub(crate) fn eintrag_verschieben(
    quelle: &Quelle<'_>,
    ziel: &Path,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let ziel = match ziel_klaeren(quelle, ziel, steuerung) {
        Zielentscheid::Nach(geklaertes_ziel) => geklaertes_ziel,
        Zielentscheid::Ueberspringen => return Ablauf::Weiter,
        Zielentscheid::Abbrechen => return Ablauf::Abgebrochen,
    };

    // Ein Ordner auf einen gleichnamigen Ordner ist kein Konflikt, sondern ein
    // Verschmelzen. `rename(2)` kann das nicht: es scheitert an einem Ziel, in
    // dem etwas liegt. Also Kind fuer Kind.
    if quelle.typ == Typ::Ordner && ziel.is_dir() {
        return verschmelzen(quelle, &ziel, art, steuerung);
    }

    match im_datentraeger_verschieben(quelle.pfad, &ziel, true) {
        Ok(()) => {
            steuerung.eintrag_fertig(quelle.pfad, quelle.groesse);
            Ablauf::Weiter
        }
        Err(fehler) if fehler.raw_os_error() == Some(EXDEV) => {
            ueber_datentraeger(quelle, &ziel, art, steuerung)
        }
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            Ablauf::Weiter
        }
    }
}

/// Verschiebt den Inhalt eines Ordners in einen gleichnamigen am Ziel.
fn verschmelzen(
    quelle: &Quelle<'_>,
    ziel: &Path,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> Ablauf {
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
        if eintrag_verschieben(&kind, &unterziel, art, steuerung) == Ablauf::Abgebrochen {
            return Ablauf::Abgebrochen;
        }
    }

    // Der leere Ordner bleibt sonst als Rest stehen. Ist er nicht leer, ist ein
    // Kind uebersprungen worden, und dann gehoert er dem Nutzer weiter.
    match std::fs::remove_dir(quelle.pfad) {
        Ok(()) => steuerung.eintrag_fertig(quelle.pfad, 0),
        Err(fehler) => steuerung.ueberspringen(
            quelle.pfad,
            format!(
                "Inhalt verschoben, der Ordner selbst blieb: {}",
                grund(&fehler)
            ),
        ),
    }
    Ablauf::Weiter
}

/// Verschiebt ueber eine Datentraegergrenze hinweg: kopieren, dann loeschen.
fn ueber_datentraeger(
    quelle: &Quelle<'_>,
    ziel: &Path,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> Ablauf {
    if kopieren::kopieren_nach(quelle, ziel, art, steuerung) == Ablauf::Abgebrochen {
        return Ablauf::Abgebrochen;
    }
    // Geloescht wird nur, was auch angekommen ist. Ist beim Kopieren etwas
    // uebersprungen worden, steht es noch in der Quelle, und `baum_entfernen`
    // scheitert daran; der Grund kommt in die Abschlussliste.
    if let Err(fehler) = loeschen::baum_entfernen(quelle.pfad) {
        steuerung.ueberspringen(
            quelle.pfad,
            format!("kopiert, aber in der Quelle geblieben: {}", grund(&fehler)),
        );
    }
    Ablauf::Weiter
}
