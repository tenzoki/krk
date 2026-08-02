//! Verzeichnisleser und Ordnermodell.
//!
//! Fuenf Module, in der Reihenfolge, in der die Daten sie durchlaufen:
//!
//! ```text
//! sys  ──> leser ──> eintrag ──> modell <── sortierung
//! ```
//!
//! [`sys`] bindet `getattrlistbulk(2)` und ist die einzige Stelle im Kern mit
//! einem Fremdaufruf. [`leser`] macht daraus den gestueckelten Lesevorgang auf
//! einem Arbeitsfaden. [`eintrag`] beschreibt, was ein Eintrag traegt.
//! [`modell`] haelt Eintraege und Sichtreihenfolge getrennt, [`sortierung`]
//! liefert die acht Ordnungen.
//!
//! Der Kern kennt AppKit nicht; alles hier ist ohne Fenster testbar.

pub mod eintrag;
pub mod leser;
pub mod modell;
pub mod sortierung;
pub mod sys;

pub use eintrag::{Eintrag, Typ};
pub use leser::{Abschluss, Lesevorgang, Meldung, STAPELGROESSE, lesen};
pub use modell::Ordnermodell;
pub use sortierung::{Richtung, Schluessel, Sortierung};
