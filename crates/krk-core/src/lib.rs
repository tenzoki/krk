#![deny(unsafe_code)]
// Die Modulkoepfe dieses Projekts erklaeren die Mechanik, und die Mechanik wohnt
// haeufig in einer privaten Funktion. Rustdoc kann einen solchen Verweis in der
// **oeffentlichen** Dokumentation nicht aufloesen und bricht daran ab. Die
// Verweise bleiben trotzdem Verweise: sie werden weder zu Fliesstext, noch wird
// ihr Ziel dafuer oeffentlich. Wer diese Koepfe liest, liest sie mit
// `--document-private-items` oder im Quelltext, und dort loesen sie auf.
//
// **Diese Zeile stellt keine Namenspruefung still.**
// `rustdoc::private_intra_doc_links` und `rustdoc::broken_intra_doc_links` sind
// zwei Pruefer, und nur der erste steht hier. Der zweite bleibt scharf: ein
// Verweis auf einen Namen, den es nicht gibt, laesst
// `RUSTDOCFLAGS="-D warnings" cargo doc` weiterhin abbrechen. Nachgemessen am
// 260906-0034 an einer Wegwerfkiste, drei Laeufe: privater Verweis ohne diese
// Zeile Exit 101, mit ihr Exit 0, und ein kaputter Verweis daneben trotz ihrer
// Exit 101 unter `-D rustdoc::broken-intra-doc-links`.
//
// Wie viele Verweisstellen die Zeile deckt, zaehlt nach ihrem Entfernen
//   RUSTDOCFLAGS="-D warnings" cargo doc -p krk-core --no-deps 2>&1 \
//     | grep -c 'links to private item'
// und keine Zahl an dieser Stelle.
//
// Entscheid:
// `260905-2336_*_wird-ein-privates-element-oeffentlich-oder-der-verweis-darauf-zu-fliesstext.md`
// (Option 3 fuer die privaten Elemente; die vier Verweise auf die privaten
// Module `zippen` und `entpacken` fuehrt rustdoc als `unresolved link`, sie
// deckt diese Zeile nicht und sie sind in `verzeichnis/sys.rs` zu Fliesstext
// geworden).
#![allow(rustdoc::private_intra_doc_links)]
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
//! `getattrlistbulk`, `copyfile`, `renamex_np`, `fcntl`, `flock` und
//! `localtime_r` und traegt dafuer `#[allow(unsafe_code)]`. Das sind sechs
//! Schnittstellen und zehn gebundene Funktionen, denn `copyfile(3)` braucht
//! seine vier `copyfile_state_*`-Helfer. `forbid` liesse sich an dieser Stelle
//! nicht oeffnen, das ist gerade sein Zweck. Es ist das einzige Modul mit
//! dieser Ausnahme, und es ist das geblieben, als Schritt 15 `copyfile` und
//! `renamex_np`, der Defekt `260809-1652` `fcntl`, die Runde 7 `flock` und die
//! Runde 18 `localtime_r` hinzugebracht haben.

pub mod ablage;
pub mod git;
pub mod leseprofil;
pub mod operation;
pub mod stapelumbenennen;
pub mod tasten;
pub mod text;
pub mod verzeichnis;
pub mod zwischenablage;
