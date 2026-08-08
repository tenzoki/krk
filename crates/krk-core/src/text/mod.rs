//! Die Textrechnung des Editors: Zeilenindex, Suche und Ersetzen (C5, C6).
//!
//! ```text
//!  Stand des Editors (String)
//!         │
//!         ├──> zeilen::Zeilenindex ──> Versatz je Zeile, Zeile je Versatz
//!         │                            (Zeilensprung aus C5, Textmarke aus C6)
//!         │
//!         └──> suche::alle ──> Treffer als Byteversatzbereiche
//!                   │
//!                   ├──> erster_ab / naechster / voriger ──> welcher Treffer
//!                   │
//!                   └──> einen_ersetzen / alle_ersetzen ──> neuer Stand
//! ```
//!
//! # Warum das hier steht und nicht in `krk-ui`
//!
//! **Keine Zeile AppKit.** Das ist Rechnung auf einer Zeichenkette: aus einem
//! Text Versaetze, aus Versaetzen ein neuer Text. Nichts davon braucht ein
//! Fenster, und `cargo test -p krk-core` erreicht es hier ohne eines. Es ist
//! derselbe Schnitt, den `stapelumbenennen` neben `operation::umbenennen`
//! zieht: dort rechnet die eine Seite die neuen Namen aus, waehrend die andere
//! das Dateisystem anfasst.
//!
//! Die Ansicht dazu ist `krk-ui`'s `appkit/editor.rs` mit seiner `NSTextView`,
//! und was der Editor ueber die geoeffnete Datei weiss, haelt
//! `krk-ui`'s `editormodell.rs`. Beide rechnen nicht selbst nach, was hier
//! steht.
//!
//! **Kein Zustand und kein Dateisystem.** Jede Funktion dieses Verzeichnisses
//! bekommt den Text als Zeichenkette und liefert Versaetze oder eine neue
//! Zeichenkette. Sie liest keine Datei und schreibt keine. Daraus faellt das
//! neunte Abnahmekriterium von C5 von selbst an: die Suche geht ueber den
//! gehaltenen Stand des Editors und nicht ueber die Datei auf der Platte, weil
//! sie einen Pfad gar nicht entgegennehmen kann.
//!
//! # Die Versaetze sind Byteversaetze in gueltigem UTF-8
//!
//! Jeder `usize`, den dieses Verzeichnis liefert oder entgegennimmt, zaehlt
//! **Bytes** und keine Zeichen, und jede Grenze liegt auf einer
//! **Zeichengrenze**. Beides gilt zusammen und ist keine Nebensache: ein
//! Versatz mitten in einer Mehrbytefolge fuehrt beim Uebertragen in die
//! `NSTextView` an eine falsche Stelle, und in Rust laesst er zudem jeden
//! Zugriff `&text[..versatz]` in Panik enden.
//!
//! Die Zusage haelt, weil jeder Versatz aus dem Text selbst entsteht: die
//! Zeilenanfaenge stehen hinter einem `\n`, die Treffer kommen aus
//! [`str::match_indices`], und beide Quellen liefern nur Zeichengrenzen. Wer
//! einen [`suche::Treffer`] von Hand baut, statt ihn aus [`suche::alle`] zu
//! nehmen, gibt die Zusage auf.
//!
//! Zeilennummern sind davon unberuehrt: sie zaehlen ab 1, weil der Nutzer sie
//! eingibt und liest.

pub mod suche;
pub mod zeilen;

// Die Typen stehen hier, die Funktionen nicht: `text::alle` und
// `text::naechster` sagten am Aufrufort nicht mehr, wovon sie handeln.
// `suche::alle` und `suche::naechster` sagen es.
pub use suche::{Ersetzung, Sammelersetzung, Treffer};
pub use zeilen::{Zeilenindex, Zeilenlage, Zeilensprung};
