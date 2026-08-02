#![deny(unsafe_code)]
//! Der Kern von KRK: Verzeichnisleser, Ordnermodell, Belegungstabelle,
//! Operationsmaschine und die Ablage in TOML.
//!
//! Der Kern kennt AppKit nicht. Das ist der Grund, aus dem er ohne Fenster
//! testbar ist, und es ist die Grenze, die `krk-ui` von `krk-core` trennt.
//!
//! Die Regel oben lautet `deny` und nicht `forbid`, und der Unterschied ist
//! Absicht: das spaetere Modul `verzeichnis::sys` bindet die beiden
//! Systemaufrufe `getattrlistbulk` und `copyfile` und traegt dafuer
//! `#[allow(unsafe_code)]`. `forbid` liesse sich an dieser Stelle nicht
//! oeffnen, das ist gerade sein Zweck.
