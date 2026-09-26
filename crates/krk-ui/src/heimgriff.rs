//! Der eine geteilte Wert der Erkennung des Notizordners.
//!
//! Die Erkennung selbst steht im Kern ([`krk_core::heimordner::Heimordner`])
//! und stellt beim Fragen keinen Systemaufruf. Welcher Ort gilt, steht erst
//! fest, wenn `settings.toml` gelesen ist, und ihre aufgeloeste Form veraltet,
//! wenn der Nutzer einen Verweis am Ort umhaengt. **Damit jeder, der fragt,
//! denselben Stand sieht, ist genau ein Wert im Umlauf**: der
//! Anwendungsdelegierte baut diesen Griff einmal und reicht Abschriften des
//! `Rc` weiter. Zwei Werte liefen auseinander, und zwei Stellen gaeben fuer
//! denselben Ordner verschiedene Antworten.
//!
//! # Wann der Wert ersetzt wird
//!
//! [`ersetzen`] ist der eine Schreiber, und er wird an drei Stellen gerufen:
//!
//! - **beim Start**, in `oberflaeche_aufbauen` unmittelbar nach
//!   `sitzung_laden` und vor der ersten Tabliste. Bis dahin haelt der Griff
//!   den Platzhalter aus [`ungelesen`], den kein Frager zu sehen bekommt;
//! - **bei F2**, mit der ueber `canonicalize` erneuerten aufgeloesten Form;
//! - ab Stufe 3 **bei „Ort waehlen…“**.
//!
//! # Abschriften, die ueber einen Aufruf hinaus leben
//!
//! Wer auf einem Arbeitsfaden fragt, bekommt beim Auftrag eine Abschrift des
//! Wertes ([`lesen`]) und nicht den Griff: ein `Rc<RefCell<…>>` gehoert dem
//! Hauptfaden, und der Uebersetzer haelt das, weil `Rc` nicht `Send` ist.
//! Solche Abschriften leben an vier Stellen ueber einen Aufruf hinaus: im
//! Ladeauftrag der Vorschau, im Einfaerbelauf der Vorschau, in der Eigenschaft
//! „ohne Inhaltsauftrag“ jedes Ordnermodells (gesetzt je Lesevorgang in
//! `Tabliste::lesen_starten`) und im Dateityp, den das Editormodell beim
//! Uebernehmen einer gelesenen Datei festlegt. Ein Wechsel des Orts erreicht
//! sie erst mit dem naechsten Auftrag.
//!
//! # Zwei Fragen, zwei Leser
//!
//! [`lage`] liefert den [`Notizort`] samt Fehler; ihn fragen F2 und „Ort
//! waehlen…“, die den Grund nennen muessen, wenn kein Ort gilt. [`lesen`]
//! liefert den Ordner, an dem die Regeln des Notizordners gelten, und ihn
//! fragen alle uebrigen. **Gilt kein Ort, sind das zwei verschiedene
//! Antworten**: F2 legt nichts an, die Regeln fuer `secrets.txt` gelten aber
//! am Schutzort weiter ([`krk_core::heimordner::ort::schutzort`] sagt, welcher
//! das ist und warum). Ohne diese Trennung schaltete eine beschaedigte
//! `settings.toml` still den Schutz der Geheimnisdatei ab.

use std::cell::RefCell;
use std::rc::Rc;

use krk_core::heimordner::Heimordner;
use krk_core::heimordner::ort::{Notizort, Ortsfehler};

/// Was der Griff haelt: der geltende Ort oder sein Fehler, und fuer den Fehler
/// den Ordner, an dem die Schutzregeln weiter gelten.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notizlage {
    /// Der geltende Ort, oder warum keiner gilt.
    ort: Notizort,
    /// Allein fuer `Err` gelesen: der Ordner, dessen Regeln trotzdem gelten.
    schutzort: Option<Heimordner>,
}

impl Notizlage {
    /// Ein geltender Ort.
    pub fn gilt(heim: Heimordner) -> Self {
        Self {
            ort: Ok(heim),
            schutzort: None,
        }
    }

    /// Kein Ort, mit dem Ordner, dessen Schutzregeln weiter gelten.
    pub fn ohne_ort(fehler: Ortsfehler, schutzort: Option<Heimordner>) -> Self {
        Self {
            ort: Err(fehler),
            schutzort,
        }
    }

    /// Aus einem Notizort; `schutzort` wird allein bei einem Fehler gehalten.
    pub fn aus(ort: Notizort, schutzort: Option<Heimordner>) -> Self {
        match ort {
            Ok(heim) => Self::gilt(heim),
            Err(fehler) => Self::ohne_ort(fehler, schutzort),
        }
    }
}

impl Default for Notizlage {
    /// Der Platzhalter aus [`ungelesen`]: kein Ort und kein Schutzort.
    fn default() -> Self {
        Self::ohne_ort(
            Ortsfehler::EinstellungenUngelesen("der Start hat sie noch nicht gelesen".to_owned()),
            None,
        )
    }
}

/// Der geteilte Griff auf den Notizordner.
pub type Heimgriff = Rc<RefCell<Notizlage>>;

/// Der Platzhalter, bevor `settings.toml` gelesen ist: kein Ort und kein
/// Schutzort.
///
/// Der Anwendungsdelegierte baut damit seinen Griff in `neu`, und
/// `oberflaeche_aufbauen` ersetzt ihn, bevor der erste Frager ihn bekommt.
/// Pruefmodule nehmen ihn fuer einen Griff ohne Notizordner.
#[must_use]
pub fn ungelesen() -> Heimgriff {
    Rc::new(RefCell::new(Notizlage::default()))
}

/// Ein Griff auf einen geltenden Ort; fuer Pruefmodule.
#[cfg(test)]
#[must_use]
pub fn mit(heim: Heimordner) -> Heimgriff {
    Rc::new(RefCell::new(Notizlage::gilt(heim)))
}

/// Eine Abschrift des Ordners, an dem die Regeln des Notizordners gelten: der
/// geltende Ort, sonst der Schutzort, sonst `None`.
///
/// Eine Abschrift und keine Leihe, damit kein Frager die Zelle ueber einen
/// Ruf hinaus haelt: eine offene Leihe liesse das naechste [`ersetzen`] in
/// einen Laufzeitfehler laufen.
#[must_use = "die Abschrift ist die ganze Wirkung; wer sie fallen laesst, hat nichts gefragt"]
pub fn lesen(griff: &Heimgriff) -> Option<Heimordner> {
    let lage = griff.borrow();
    match &lage.ort {
        Ok(heim) => Some(heim.clone()),
        Err(_) => lage.schutzort.clone(),
    }
}

/// Eine Abschrift des geltenden Orts samt Fehler; fuer F2 und „Ort waehlen…“.
///
/// Der Schutzort kommt hier nicht heraus: wer anlegen will, darf ihn nicht
/// sehen.
#[must_use = "die Abschrift ist die ganze Wirkung; wer sie fallen laesst, hat nichts gefragt"]
pub fn lage(griff: &Heimgriff) -> Notizort {
    griff.borrow().ort.clone()
}

/// Setzt den Wert, den ab jetzt jeder Frager sieht.
pub fn ersetzen(griff: &Heimgriff, lage: Notizlage) {
    *griff.borrow_mut() = lage;
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /// Ein Ersetzen ist ueber jede Abschrift des Griffs zu sehen, und eine
    /// vorher gelesene Abschrift des Wertes bleibt, wie sie war.
    #[test]
    fn jede_abschrift_des_griffs_sieht_das_ersetzen() {
        let griff = ungelesen();
        let zweiter = Rc::clone(&griff);
        let vorher = lesen(&zweiter);

        let heim = Heimordner::im_benutzerverzeichnis(Path::new("/Users/probe-ohne-ordner"));
        ersetzen(&griff, Notizlage::gilt(heim.clone()));

        assert_eq!(vorher, None, "die fruehere Abschrift ist ein eigener Wert");
        assert_eq!(lesen(&zweiter), Some(heim.clone()));
        assert_eq!(lage(&zweiter), Ok(heim));
    }

    /// Gilt kein Ort, sieht F2 den Fehler und nie den Schutzort; jeder andere
    /// Frager sieht den Schutzort.
    #[test]
    fn ohne_ort_sieht_f2_den_fehler_und_jeder_andere_den_schutzort() {
        let schutz = Heimordner::im_benutzerverzeichnis(Path::new("/Users/probe-ohne-ordner"));
        let fehler = Ortsfehler::EinstellungenBeschaedigt("ist beschädigt".to_owned());
        let griff = ungelesen();
        ersetzen(
            &griff,
            Notizlage::ohne_ort(fehler.clone(), Some(schutz.clone())),
        );

        assert_eq!(lage(&griff), Err(fehler));
        assert_eq!(lesen(&griff), Some(schutz));
    }

    /// Ein geltender Ort verwirft einen mitgegebenen Schutzort: `aus` haelt
    /// ihn allein fuer den Fehler.
    #[test]
    fn ein_geltender_ort_haelt_keinen_schutzort() {
        let heim = Heimordner::am_ort(Path::new("/Volumes/X/notizen").to_path_buf(), None);
        let schutz = Heimordner::im_benutzerverzeichnis(Path::new("/Users/probe-ohne-ordner"));
        assert_eq!(
            Notizlage::aus(Ok(heim.clone()), Some(schutz)),
            Notizlage::gilt(heim)
        );
    }
}
