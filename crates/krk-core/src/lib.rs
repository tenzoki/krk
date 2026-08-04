#![deny(unsafe_code)]
//! Der Kern von KRK: Verzeichnisleser, Ordnermodell, Belegungstabelle,
//! Operationsmaschine und die Ablage in TOML.
//!
//! Der Kern kennt AppKit nicht. Das ist der Grund, aus dem er ohne Fenster
//! testbar ist, und es ist die Grenze, die `krk-ui` von `krk-core` trennt.
//!
//! Die Regel oben lautet `deny` und nicht `forbid`, und der Unterschied ist
//! Absicht: das Modul `verzeichnis::sys` bindet die Systemaufrufe
//! `getattrlistbulk`, `copyfile` und `renamex_np` und traegt dafuer
//! `#[allow(unsafe_code)]`. `forbid` liesse sich an dieser Stelle nicht
//! oeffnen, das ist gerade sein Zweck. Es ist das einzige Modul mit dieser
//! Ausnahme, und mit Schritt 15 ist es das geblieben.

pub mod ablage;
pub mod operation;
pub mod tasten;
pub mod verzeichnis;
pub mod zwischenablage;
