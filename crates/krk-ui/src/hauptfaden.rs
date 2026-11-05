#![deny(unsafe_code)]
// In diesem Bau fehlen alle `#[test]`-Funktionen: Cargo uebersetzt ein Ziel
// mit `harness = false` mit `cfg(test)`, aber ohne `--test`, und der
// Uebersetzer streicht jede `#[test]`-Funktion. Ihre Helfer und die Namen,
// die allein sie hereinholen, liegen deshalb hier brach, und ebenso der ganze
// Betriebscode, den `main.rs` erreicht und diese Wurzel nicht. Die zwei anderen
// Bauten derselben Dateien, das Binaerziel `krk` und sein Pruefbau unter
// `libtest`, warnen weiter, also verdeckt die Erlaubnis dort nichts.
#![allow(dead_code, unused_imports)]
//! Die zweite Kistenwurzel von `krk-ui`: das Pruefziel `hauptfaden`.
//!
//! **Wozu.** `libtest` fuehrt jede Probe auf einem eigenen Faden, und AppKit
//! bindet `NSView` an den Hauptfaden; seit dem SDK 26 setzt es die Bindung
//! durch. Ein Ziel mit `harness = false` haelt sein `main` selbst, also laeuft
//! es auf dem Hauptfaden, und `MainThreadMarker::new()` beantwortet die Frage
//! zur Laufzeit, statt sie vorauszusetzen. Entschieden im Datensatz
//! `261101-1436-wie-kommen-die-appkit-proben-auf-den-hauptfaden.md`.
//!
//! **Dieselben Module wie `main.rs`, in derselben Reihenfolge.** Die Proben
//! bleiben in den Pruefmodulen ihrer Dateien und erreichen dort alles Private;
//! diese Wurzel uebersetzt die Dateien deshalb ein drittes Mal, mit
//! `cfg(test)`. Die `mod`-Zeilen samt ihrer Attributzeile stehen wie in
//! `main.rs`, und `die_zweite_kistenwurzel_fuehrt_die_mod_zeilen_von_main`
//! (`appkit::hauptfaden`) wird rot, sobald die zwei Listen auseinanderlaufen.
//! Was ein Modul tut, steht im Kopf von `main.rs` und nicht hier.
//!
//! **Diese Wurzel nennt keine `objc2`-Kiste**, wie `main.rs`. Den Hauptfaden
//! fragt der Laeufer unter `appkit`, und dort steht auch, welche Proben er
//! faehrt.

use std::process::ExitCode;

mod angezeigtedatei;
mod anmeldehilfe;
mod appkit;
mod archivregister;
mod auffrischung;
mod belegungsausgabe;
mod belegungsmodell;
mod cloudladen;
mod editormodell;
mod fenstermodell;
mod fenstertitel;
#[cfg(test)]
mod fernprobe;
mod filteranzeige;
mod gitmodell;
mod heimgriff;
mod hervorhebung;
mod jsonzeilen;
mod kommandos;
mod leistenmodell;
mod markdown;
mod menuemodell;
mod messmodus;
mod plattenwarte;
#[cfg(test)]
mod pruefordner;
#[cfg(test)]
mod quellbaum;
mod quicknote;
mod quicknotesicherung;
mod spalten;
mod tabs;
mod trenntextvorschau;
mod tresorweg;
mod vorschaumodell;

fn main() -> ExitCode {
    // Der Rueckgabewert 101 ist der, mit dem auch `libtest` einen roten Lauf
    // meldet; `cargo test` liest ihn gleich.
    if appkit::hauptfaden::laufen(std::env::args().skip(1)).ist_gruen() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(101)
    }
}
