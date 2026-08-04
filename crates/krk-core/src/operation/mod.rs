//! Die Operationsmaschine: Kopieren, Verschieben, Loeschen, Anlegen,
//! Umbenennen (C4).
//!
//! ```text
//!            Auftrag ──> starten ──> Arbeitsfaden ──> ausfuehren
//!                          │                             │
//!                          │                             ├─> kopieren
//!  Hauptfaden <── Lauf ────┘                             ├─> verschieben
//!    Meldung  <── Kanal <── Steuerung <──────────────────┴─> loeschen
//!    abbrechen ─> AtomicBool ─┘                                 │
//!                                                     Papierkorb (injiziert)
//!
//!            anlegen, umbenennen: ohne Faden, sofort fertig
//! ```
//!
//! # Der Hauptfaden fuehrt keine Dateisystem-Arbeit aus
//!
//! Kein Sonderfall, keine Ausnahme. Jeder Auftrag laeuft auf einem eigenen
//! Arbeitsfaden, der Fortschritt und uebersprungene Eintraege ueber einen Kanal
//! meldet und ueber ein [`AtomicBool`](std::sync::atomic::AtomicBool)
//! abgebrochen wird. Damit haelt die Zusage L9 ("keine Eingabe wartet laenger
//! als 16 ms waehrend einer Stapeloperation") strukturell und nicht durch
//! Sorgfalt.
//!
//! # Drei Dinge, die dieses Modul bewusst nicht selbst tut
//!
//! **Es zaehlt nicht vor.** Wie viele Eintraege ein Ordnerbaum traegt, weiss
//! die Maschine erst, wenn sie ihn abgearbeitet hat. Ein Durchlauf vorweg
//! koennte die 200 ms aus L8 allein aufbrauchen; das Fortschrittsblatt aus S16
//! erscheint deshalb nach einer Zeitspanne und nicht nach einer Schwelle.
//!
//! **Es steigt nicht selbst durch Verzeichnisse.** Der Abstieg laeuft ueber
//! [`crate::verzeichnis::lesen`] aus Schritt 2. Der Leser ist die eine Auskunft
//! darueber, was in einem Ordner steht.
//!
//! **Es kennt AppKit nicht.** Der Papierkorb kommt als Schnittstelle herein,
//! siehe [`loeschen`].
//!
//! # Eine gescheiterte Einzelposition bricht den Stapel nicht ab
//!
//! Sie sammelt Eintrag und Grund und kommt in die Abschlussliste (C4). Nur zwei
//! Dinge beenden einen Vorgang vorzeitig: der Abbruchbefehl des Nutzers und
//! seine Antwort "abbrechen" auf eine Konfliktfrage.

pub mod anlegen;
pub mod auftrag;
pub mod fortschritt;
mod kopieren;
pub mod loeschen;
pub mod umbenennen;
mod verschieben;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;
use std::thread;

use crate::verzeichnis::Typ;

pub use anlegen::{datei_anlegen, ordner_anlegen};
pub use auftrag::{Art, Auftrag, Konfliktregel};
pub use fortschritt::{
    Abschluss, Bericht, Fortschritt, Konfliktantwort, Konfliktentscheid, Lauf, MELDEABSTAND,
    Meldung, Uebersprungen,
};
pub use loeschen::{OhnePapierkorb, Papierkorb};
pub use umbenennen::{Namensfehler, freier_name, name_pruefen, umbenennen};

use fortschritt::Steuerung;
use umbenennen::name_pruefen as namen_pruefen;

/// Ein Eintrag, so wie die Maschine ihn anfasst.
///
/// Der Typ und die Groesse stehen daneben, weil der Aufrufer sie schon hat: im
/// Baum aus dem Leser, an der Oberflaeche aus einem einzelnen `stat`. Wer sie
/// hier noch einmal erfragte, fragte das Dateisystem zweimal dasselbe.
pub(crate) struct Quelle<'a> {
    /// Wo der Eintrag liegt.
    pub pfad: &'a Path,
    /// Ordner, Datei oder Verknuepfung. Entscheidet ueber den Abstieg.
    pub typ: Typ,
    /// Die Groesse in Bytes. Ordner und Verknuepfungen tragen 0.
    pub groesse: u64,
}

/// Ob nach einem Eintrag weitergemacht wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Ablauf {
    /// Weiter mit dem naechsten Eintrag.
    Weiter,
    /// Der ganze Vorgang endet.
    Abgebrochen,
}

/// Wohin ein Eintrag geht, nachdem ein Konflikt geklaert ist.
pub(crate) enum Zielentscheid {
    /// An diesen Pfad. Er kann vom urspruenglichen abweichen, wenn der Nutzer
    /// umbenennen gewaehlt hat.
    Nach(PathBuf),
    /// Diesen Eintrag auslassen. Er steht bereits in der Abschlussliste.
    Ueberspringen,
    /// Den ganzen Vorgang beenden.
    Abbrechen,
}

/// Startet einen Auftrag auf einem eigenen Arbeitsfaden und kehrt sofort
/// zurueck.
///
/// Der [`Lauf`] ist der Griff daran: er traegt den Kanal mit den Meldungen und
/// den Abbruch. Wird er fallen gelassen, endet der Vorgang.
pub fn starten(auftrag: Auftrag, papierkorb: Arc<dyn Papierkorb>) -> Lauf {
    let abbruch = Arc::new(AtomicBool::new(false));
    let faden_abbruch = Arc::clone(&abbruch);
    let (sender, empfaenger) = channel();
    let abschlusssender = sender.clone();

    let faden = thread::Builder::new()
        .name("krk-operation".to_owned())
        .spawn(move || {
            let mut steuerung = Steuerung::neu(faden_abbruch, Some(sender), auftrag.konfliktregel);
            let abschluss = ausfuehren(&auftrag, papierkorb.as_ref(), &mut steuerung);
            let _ = abschlusssender.send(Meldung::Fertig(steuerung.bericht(abschluss)));
        })
        .expect("Arbeitsfaden fuer eine Dateioperation laesst sich nicht starten");

    Lauf::neu(abbruch, empfaenger, faden)
}

/// Arbeitet einen Auftrag Quelle fuer Quelle ab.
fn ausfuehren(
    auftrag: &Auftrag,
    papierkorb: &dyn Papierkorb,
    steuerung: &mut Steuerung,
) -> Abschluss {
    for pfad in &auftrag.quellen {
        if steuerung.abgebrochen() {
            return Abschluss::Abgebrochen;
        }
        if einen_abarbeiten(auftrag, pfad, papierkorb, steuerung) == Ablauf::Abgebrochen {
            return Abschluss::Abgebrochen;
        }
    }
    if steuerung.abgebrochen() {
        Abschluss::Abgebrochen
    } else {
        Abschluss::Fertig
    }
}

fn einen_abarbeiten(
    auftrag: &Auftrag,
    pfad: &Path,
    papierkorb: &dyn Papierkorb,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let (typ, groesse) = match typ_und_groesse(pfad) {
        Ok(werte) => werte,
        Err(fehler) => {
            steuerung.ueberspringen(pfad, grund(&fehler));
            return Ablauf::Weiter;
        }
    };
    let quelle = Quelle { pfad, typ, groesse };

    match &auftrag.art {
        Art::Kopieren { ziel } => match zielpfad(&quelle, ziel, steuerung) {
            Some(ziel) => {
                kopieren::eintrag_kopieren(&quelle, &ziel, auftrag.uebertragung, steuerung)
            }
            None => Ablauf::Weiter,
        },
        Art::Verschieben { ziel } => match zielpfad(&quelle, ziel, steuerung) {
            Some(ziel) => {
                verschieben::eintrag_verschieben(&quelle, &ziel, auftrag.uebertragung, steuerung)
            }
            None => Ablauf::Weiter,
        },
        Art::InDenPapierkorb => loeschen::in_den_papierkorb(&quelle, papierkorb, steuerung),
        Art::EndgueltigLoeschen => loeschen::endgueltig_loeschen(&quelle, steuerung),
    }
}

/// Rechnet den Zielpfad einer Quelle aus und weist die drei Faelle ab, in denen
/// es keinen gibt.
///
/// Der dritte ist der gefaehrliche: ein Ordner, der in sich selbst kopiert
/// wird, waechst waehrend des Kopierens weiter, und der Abstieg fuellt den
/// Datentraeger. Die Pruefung steht hier oben und nicht im Abstieg, weil der
/// Abstieg sie sonst bei jedem Eintrag wiederholen muesste.
fn zielpfad(quelle: &Quelle<'_>, zielordner: &Path, steuerung: &mut Steuerung) -> Option<PathBuf> {
    let Some(name) = quelle.pfad.file_name() else {
        steuerung.ueberspringen(quelle.pfad, "der Pfad benennt keinen Eintrag");
        return None;
    };
    let ziel = zielordner.join(name);
    if ziel == quelle.pfad {
        steuerung.ueberspringen(quelle.pfad, "Quelle und Ziel sind derselbe Eintrag");
        return None;
    }
    if quelle.typ == Typ::Ordner && ziel.starts_with(quelle.pfad) {
        steuerung.ueberspringen(quelle.pfad, "das Ziel liegt in der Quelle");
        return None;
    }
    Some(ziel)
}

/// Klaert, wohin ein Eintrag geht, wenn am Ziel schon etwas steht.
///
/// Ein Ordner auf einen gleichnamigen Ordner ist **kein** Konflikt: sein Inhalt
/// wandert in den vorhandenen. Andernfalls entscheidet die Konfliktregel des
/// Auftrags, notfalls durch Nachfragen beim Nutzer.
pub(crate) fn ziel_klaeren(
    quelle: &Quelle<'_>,
    ziel: &Path,
    steuerung: &mut Steuerung,
) -> Zielentscheid {
    let Ok(vorhanden) = fs::symlink_metadata(ziel) else {
        return Zielentscheid::Nach(ziel.to_path_buf());
    };
    if quelle.typ == Typ::Ordner && vorhanden.is_dir() {
        return Zielentscheid::Nach(ziel.to_path_buf());
    }

    match steuerung.konflikt_loesen(quelle.pfad, ziel) {
        Konfliktantwort::Ueberschreiben => match loeschen::baum_entfernen(ziel) {
            Ok(()) => Zielentscheid::Nach(ziel.to_path_buf()),
            Err(fehler) => {
                steuerung.ueberspringen(
                    quelle.pfad,
                    format!("das Ziel liess sich nicht ersetzen: {}", grund(&fehler)),
                );
                Zielentscheid::Ueberspringen
            }
        },
        Konfliktantwort::Ueberspringen => {
            steuerung.ueberspringen(quelle.pfad, "am Ziel steht schon ein Eintrag");
            Zielentscheid::Ueberspringen
        }
        Konfliktantwort::UmbenennenIn(name) => match namen_pruefen(&name) {
            Ok(()) => Zielentscheid::Nach(ziel.with_file_name(name)),
            Err(fehler) => {
                steuerung.ueberspringen(quelle.pfad, fehler.grund());
                Zielentscheid::Ueberspringen
            }
        },
        Konfliktantwort::Abbrechen => Zielentscheid::Abbrechen,
    }
}

/// Fragt Typ und Groesse eines Eintrags ab, ohne einer Verknuepfung zu folgen.
fn typ_und_groesse(pfad: &Path) -> io::Result<(Typ, u64)> {
    let angaben = fs::symlink_metadata(pfad)?;
    let typ = if angaben.is_symlink() {
        Typ::Verknuepfung
    } else if angaben.is_dir() {
        Typ::Ordner
    } else {
        Typ::Datei
    };
    let groesse = if typ == Typ::Datei { angaben.len() } else { 0 };
    Ok((typ, groesse))
}

/// Uebersetzt einen Systemfehler in den Grund, den die Abschlussliste zeigt.
///
/// Die vier haeufigen Faelle stehen auf Deutsch da, weil der Nutzer sie liest.
/// Alles andere behaelt den Wortlaut des Systems: eine erfundene Uebersetzung
/// waere ungenauer als das Original.
pub(crate) fn grund(fehler: &io::Error) -> String {
    match fehler.kind() {
        io::ErrorKind::PermissionDenied => "keine Rechte".to_owned(),
        io::ErrorKind::NotFound => "gibt es nicht mehr".to_owned(),
        io::ErrorKind::AlreadyExists => "am Ziel steht schon ein Eintrag".to_owned(),
        io::ErrorKind::StorageFull => "kein Platz mehr auf dem Datentraeger".to_owned(),
        _ => fehler.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ein_ordner_kann_nicht_in_sich_selbst_kopiert_werden() {
        let (mut steuerung, _empfaenger) = pruefsteuerung();
        let pfad = Path::new("/tmp/krk-ordner");
        let quelle = Quelle {
            pfad,
            typ: Typ::Ordner,
            groesse: 0,
        };
        let ziel = zielpfad(&quelle, Path::new("/tmp/krk-ordner/unten"), &mut steuerung);
        assert!(ziel.is_none(), "das Ziel laege in der Quelle");

        let bericht = steuerung.bericht(Abschluss::Fertig);
        assert_eq!(bericht.uebersprungen.len(), 1);
        assert_eq!(
            bericht.uebersprungen[0].grund,
            "das Ziel liegt in der Quelle"
        );
    }

    #[test]
    fn eine_quelle_kann_nicht_auf_sich_selbst_kopiert_werden() {
        let (mut steuerung, _empfaenger) = pruefsteuerung();
        let pfad = Path::new("/tmp/krk-ordner/datei.txt");
        let quelle = Quelle {
            pfad,
            typ: Typ::Datei,
            groesse: 7,
        };
        let ziel = zielpfad(&quelle, Path::new("/tmp/krk-ordner"), &mut steuerung);
        assert!(ziel.is_none());
    }

    #[test]
    fn die_haeufigen_gruende_stehen_auf_deutsch_da() {
        let fehler = io::Error::from(io::ErrorKind::PermissionDenied);
        assert_eq!(grund(&fehler), "keine Rechte");
    }

    fn pruefsteuerung() -> (Steuerung, std::sync::mpsc::Receiver<Meldung>) {
        let (sender, empfaenger) = channel();
        (
            Steuerung::neu(
                Arc::new(AtomicBool::new(false)),
                Some(sender),
                Konfliktregel::Fragen,
            ),
            empfaenger,
        )
    }
}
