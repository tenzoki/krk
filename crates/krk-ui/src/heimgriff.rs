//! Der eine geteilte Wert der Erkennung von `~/krkhome/`.
//!
//! Die Erkennung selbst steht im Kern ([`krk_core::heimordner::Heimordner`])
//! und stellt beim Fragen keinen Systemaufruf. Ihre aufgeloeste Form veraltet
//! aber, wenn der Nutzer den Verweis `~/krkhome` umhaengt, und erneuert wird sie
//! allein bei F2 (`Anwendungsdelegierter::notizordner_oeffnen`). **Damit jeder,
//! der fragt, die Erneuerung sieht, ist genau ein Wert im Umlauf**: der
//! Anwendungsdelegierte baut diesen Griff einmal beim Start und reicht
//! Abschriften des `Rc` weiter, und keiner der Frager haelt eine eigene
//! Abschrift des `Heimordner` ueber einen Aufruf hinaus. Zwei Werte liefen nach
//! dem ersten F2 auseinander, und zwei Stellen gaeben fuer denselben Ordner
//! verschiedene Antworten.
//!
//! **Wer auf einem Arbeitsfaden fragt, bekommt beim Auftrag eine Abschrift des
//! Wertes ([`lesen`]) und nicht den Griff**: ein `Rc<RefCell<…>>` gehoert dem
//! Hauptfaden, und der Uebersetzer haelt das, weil `Rc` nicht `Send` ist.
//!
//! `None` heisst: das System nennt kein Benutzerverzeichnis. Dann gibt es
//! keinen Heimordner, und F2 meldet genau das, statt einen Tab zu oeffnen.

use std::cell::RefCell;
use std::rc::Rc;

use krk_core::heimordner::Heimordner;

/// Der geteilte Griff auf den Heimordner.
pub type Heimgriff = Rc<RefCell<Option<Heimordner>>>;

/// Eine Abschrift des Wertes, wie er gerade steht.
///
/// Eine Abschrift und keine Leihe, damit kein Frager die Zelle ueber einen
/// Ruf hinaus haelt: eine offene Leihe liesse das naechste [`ersetzen`] in
/// einen Laufzeitfehler laufen.
#[must_use = "die Abschrift ist die ganze Wirkung; wer sie fallen laesst, hat nichts gefragt"]
pub fn lesen(griff: &Heimgriff) -> Option<Heimordner> {
    griff.borrow().clone()
}

/// Setzt den Wert, den ab jetzt jeder Frager sieht.
pub fn ersetzen(griff: &Heimgriff, heim: Heimordner) {
    *griff.borrow_mut() = Some(heim);
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /// Ein Ersetzen ist ueber jede Abschrift des Griffs zu sehen, und eine
    /// vorher gelesene Abschrift des Wertes bleibt, wie sie war.
    #[test]
    fn jede_abschrift_des_griffs_sieht_das_ersetzen() {
        let griff: Heimgriff = Rc::new(RefCell::new(None));
        let zweiter = Rc::clone(&griff);
        let vorher = lesen(&zweiter);

        let heim = Heimordner::im_benutzerverzeichnis(Path::new("/Users/probe-ohne-ordner"));
        ersetzen(&griff, heim.clone());

        assert_eq!(vorher, None, "die fruehere Abschrift ist ein eigener Wert");
        assert_eq!(lesen(&zweiter), Some(heim));
    }
}
