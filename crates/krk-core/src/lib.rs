#![deny(unsafe_code)]
//! Der Kern von KRK: Verzeichnisleser, Ordnermodell, Belegungstabelle,
//! Operationsmaschine, das Regelmodell fuer das Umbenennen im Stapel, die
//! Textrechnung des Editors, die Leseprofile des Vorschaufensters und die
//! Ablage in TOML.
//!
//! Die Leseprofile aus der Runde 16 sind die juengste Schicht: `leseprofil`
//! liest die von Hand gepflegte `readers.toml`, erkennt an einem Ordner sein
//! Profil und rechnet daraus die Zusammenfassung, die das Vorschaufenster an
//! die Stelle der Metadaten setzt. Sie liegt hier und nicht in `krk-ui`, weil
//! ihre abzaehlbaren Grenzen ohne Fenster zu belegen sind und `krk-ui` kein
//! Bibliotheksziel hat; die Herleitung steht im Kopf jenes Moduls.
//!
//! Der Kern kennt AppKit nicht. Das ist der Grund, aus dem er ohne Fenster
//! testbar ist, und es ist die Grenze, die `krk-ui` von `krk-core` trennt.
//!
//! Die Regel oben lautet `deny` und nicht `forbid`, und der Unterschied ist
//! Absicht: das Modul `verzeichnis::sys` bindet die Systemaufrufe
//! `getattrlistbulk`, `copyfile`, `renamex_np`, `fcntl` und `flock` und traegt
//! dafuer `#[allow(unsafe_code)]`. Das sind fuenf Schnittstellen und neun
//! gebundene Funktionen, denn `copyfile(3)` braucht seine vier
//! `copyfile_state_*`-Helfer. `forbid` liesse sich an dieser Stelle nicht
//! oeffnen, das ist gerade sein Zweck. Es ist das einzige Modul mit dieser
//! Ausnahme, und es ist das geblieben, als Schritt 15 `copyfile` und
//! `renamex_np`, der Defekt `260809-1652` `fcntl` und die Runde 7 `flock`
//! hinzugebracht haben.

pub mod ablage;
pub mod leseprofil;
pub mod operation;
pub mod stapelumbenennen;
pub mod tasten;
pub mod text;
pub mod verzeichnis;
pub mod zwischenablage;
