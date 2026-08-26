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
///
/// Geloescht wird nur, was auch angekommen ist, und der Zeuge dafuer ist der
/// Zaehlstand der uebersprungenen Eintraege: jeder Weg in `kopieren.rs` und in
/// [`ziel_klaeren`], der ohne Ankunft am Ziel endet, ruft
/// `Steuerung::ueberspringen`, und `kopieren_nach` liefert fuer ein
/// gescheitertes wie fuer ein gegluecktes Kopieren `Ablauf::Weiter`. Ein
/// Rueckgabewert allein sagt also nichts ueber die Ankunft, der Zaehlstand
/// sagt es vollstaendig. Wer in `kopieren.rs` einen Fehlerzweig ohne
/// `ueberspringen` anlegt, bricht diese Zusage; die Proben unten halten sie
/// fuer den Fall der Datei und den des Ordners mit gescheitertem Kind.
fn ueber_datentraeger(
    quelle: &Quelle<'_>,
    ziel: &Path,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let stand = steuerung.uebersprungen_stand();
    if kopieren::kopieren_nach(quelle, ziel, art, steuerung) == Ablauf::Abgebrochen {
        return Ablauf::Abgebrochen;
    }

    let seither = steuerung.uebersprungen_seit(stand);
    if !seither.is_empty() {
        // Etwas ist nicht angekommen; die Quelle gehoert dem Nutzer weiter.
        // Ist sie selbst schon genannt (Datei, Verknuepfung, `create_dir`),
        // steht der Grund da. Sonst ist ein Kind oder eine Ordnerangabe
        // gescheitert, und der Ordner bekommt seine eigene Zeile.
        let selbst_genannt = seither.iter().any(|eintrag| eintrag.pfad == quelle.pfad);
        if !selbst_genannt {
            steuerung.ueberspringen(
                quelle.pfad,
                "nicht vollstaendig kopiert, in der Quelle geblieben",
            );
        }
        return Ablauf::Weiter;
    }

    if let Err(fehler) = loeschen::baum_entfernen(quelle.pfad) {
        steuerung.ueberspringen(
            quelle.pfad,
            format!("kopiert, aber in der Quelle geblieben: {}", grund(&fehler)),
        );
    }
    Ablauf::Weiter
}

#[cfg(test)]
mod tests {
    //! Die Proben rufen [`ueber_datentraeger`] direkt und brauchen deshalb
    //! keinen zweiten Datentraeger: was hier gemessen wird, ist allein die
    //! Regel, wann die Quelle nach dem Kopieren geloescht wird, und nicht der
    //! `EXDEV`-Abzweig davor (`shared/issues/260826-1221_*_ein-gescheitertes-kopieren-ueber-die-datentraegergrenze-loescht-die-quelle-trotzdem.md`).

    use std::fs;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use super::*;
    use crate::operation::auftrag::Konfliktregel;
    use crate::operation::fortschritt::Abschluss;

    /// Ein frischer Ordner unter dem Temporaerverzeichnis, mit Prozesskennung
    /// und Probennamen, damit die drei Proben nebeneinander laufen koennen.
    ///
    /// Bewusst keine vierte Pruefordner-Fassung mit `Drop`: die eine Fassung
    /// dieser Kiste liegt in `tests/gemeinsam/mod.rs`, und ein Pruefmodul der
    /// Bibliothek erreicht sie nicht. Abgeraeumt wird deshalb von Hand mit
    /// [`abraeumen`] am Ende jeder Probe; ein Rest einer abgebrochenen Probe
    /// faellt beim naechsten Anlegen.
    fn pruefpfad(probe: &str) -> PathBuf {
        let pfad =
            std::env::temp_dir().join(format!("krk-verschieben-{probe}-{}", std::process::id()));
        abraeumen(&pfad);
        fs::create_dir_all(&pfad).expect("der Pruefordner laesst sich nicht anlegen");
        pfad
    }

    fn abraeumen(pfad: &Path) {
        let _ = fs::remove_dir_all(pfad);
    }

    fn steuerung(regel: Konfliktregel) -> Steuerung {
        Steuerung::neu(Arc::new(AtomicBool::new(false)), None, regel)
    }

    #[test]
    fn eine_datei_die_nicht_ankommt_bleibt_in_der_quelle() {
        let ordner = pruefpfad("datei");
        let quelle = ordner.join("quelle.txt");
        fs::write(&quelle, b"inhalt").expect("die Quelle laesst sich nicht schreiben");
        let ziel = ordner.join("fehlt").join("ziel.txt");
        let mut steuerung = steuerung(Konfliktregel::Ueberspringen);

        let ablauf = ueber_datentraeger(
            &Quelle {
                pfad: &quelle,
                typ: Typ::Datei,
                groesse: 6,
            },
            &ziel,
            Uebertragungsart::ImmerBytes,
            &mut steuerung,
        );

        assert_eq!(ablauf, Ablauf::Weiter);
        assert!(
            quelle.exists(),
            "die Quelle ist weg, obwohl nichts angekommen ist"
        );
        let bericht = steuerung.bericht(Abschluss::Fertig);
        assert_eq!(
            bericht.uebersprungen.len(),
            1,
            "{:?}",
            bericht.uebersprungen
        );
        assert_eq!(bericht.uebersprungen[0].pfad, quelle);
        abraeumen(&ordner);
    }

    #[test]
    fn ein_ordner_mit_einem_uebersprungenen_kind_bleibt_in_der_quelle() {
        let ordner = pruefpfad("ordner");
        let quelle = ordner.join("quelle");
        fs::create_dir(&quelle).expect("der Quellordner laesst sich nicht anlegen");
        fs::write(quelle.join("a.txt"), b"a").expect("a.txt");
        fs::write(quelle.join("b.txt"), b"b").expect("b.txt");
        let ziel = ordner.join("ziel");
        // Am Ziel steht `b.txt` schon, und zwar als Ordner: ein Konflikt, den
        // die Regel "ueberspringen" ohne Nachfrage entscheidet.
        fs::create_dir_all(ziel.join("b.txt")).expect("der Stoerer laesst sich nicht anlegen");
        let mut steuerung = steuerung(Konfliktregel::Ueberspringen);

        let ablauf = ueber_datentraeger(
            &Quelle {
                pfad: &quelle,
                typ: Typ::Ordner,
                groesse: 0,
            },
            &ziel,
            Uebertragungsart::ImmerBytes,
            &mut steuerung,
        );

        assert_eq!(ablauf, Ablauf::Weiter);
        assert!(quelle.join("a.txt").is_file(), "a.txt fehlt in der Quelle");
        assert!(quelle.join("b.txt").is_file(), "b.txt fehlt in der Quelle");
        let bericht = steuerung.bericht(Abschluss::Fertig);
        let pfade: Vec<&Path> = bericht
            .uebersprungen
            .iter()
            .map(|u| u.pfad.as_path())
            .collect();
        assert!(
            pfade.contains(&quelle.join("b.txt").as_path()),
            "das Kind fehlt im Bericht: {pfade:?}"
        );
        assert!(
            pfade.contains(&quelle.as_path()),
            "der Ordner fehlt im Bericht: {pfade:?}"
        );
        abraeumen(&ordner);
    }

    #[test]
    fn ein_angekommener_eintrag_verlaesst_die_quelle() {
        let ordner = pruefpfad("gegenprobe");
        let quelle = ordner.join("quelle.txt");
        fs::write(&quelle, b"inhalt").expect("die Quelle laesst sich nicht schreiben");
        let ziel = ordner.join("ziel.txt");
        let mut steuerung = steuerung(Konfliktregel::Ueberspringen);

        let ablauf = ueber_datentraeger(
            &Quelle {
                pfad: &quelle,
                typ: Typ::Datei,
                groesse: 6,
            },
            &ziel,
            Uebertragungsart::ImmerBytes,
            &mut steuerung,
        );

        assert_eq!(ablauf, Ablauf::Weiter);
        assert!(!quelle.exists(), "die Quelle steht noch");
        assert!(ziel.is_file(), "das Ziel fehlt");
        let bericht = steuerung.bericht(Abschluss::Fertig);
        assert!(
            bericht.uebersprungen.is_empty(),
            "{:?}",
            bericht.uebersprungen
        );
        abraeumen(&ordner);
    }
}
