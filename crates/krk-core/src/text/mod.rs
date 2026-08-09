//! Die Textrechnung des Editors: Einlesen, Zeilenindex, Suche und Ersetzen,
//! Sichern (C2, C4, C5, C6).
//!
//! ```text
//!  Bytes ──> datei::einlesen ──┐
//!                              │
//!                              v
//!                     Stand des Editors (String)
//!                              │
//!         ┌────────────────────┼────────────────────┐
//!         │                    │                    │
//!         │                    │                    v
//!         │                    │           datei::sichern ──> Platte
//!         │                    │
//!         │                    └──> suche::alle ──> Treffer als
//!         │                              │          Byteversatzbereiche
//!         │                              │
//!         │                              ├──> erster_ab / naechster / voriger
//!         │                              │
//!         │                              └──> einen_ersetzen / alle_ersetzen
//!         │                                        ──> neuer Stand
//!         v
//!  zeilen::Zeilenindex ──> Versatz je Zeile, Zeile je Versatz
//!                          (Zeilensprung aus C5, Textmarke aus C6)
//! ```
//!
//! # Der Stand traegt `\n` und keine Bytefolgenmarke
//!
//! Die Zusage, ohne die [`zeilen`] und [`suche`] anders aussehen muessten,
//! steht in [`datei`] und wird hier nicht zum zweiten Mal formuliert. Kurz:
//! **[`datei::einlesen`] stellt sie her, alle uebrigen Module dieses
//! Verzeichnisses rechnen darauf, und wer Text von anderswo in den Stand
//! bringt, fuehrt ihn durch [`datei::in_gehaltene_form`].** Der Preis dieser
//! Wahl und der Datensatz, aus dem sie stammt, stehen ebenfalls dort.
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
//! **Kein Zustand, und das Dateisystem an genau einer Stelle.** [`zeilen`] und
//! [`suche`] bekommen den Text als Zeichenkette und liefern Versaetze oder eine
//! neue Zeichenkette; sie lesen keine Datei und schreiben keine. Daraus faellt
//! das neunte Abnahmekriterium von C5 von selbst an: die Suche geht ueber den
//! gehaltenen Stand des Editors und nicht ueber die Datei auf der Platte, weil
//! sie einen Pfad gar nicht entgegennehmen kann.
//!
//! [`datei`] ist die Ausnahme und der Grund, aus dem die Regel sich so genau
//! aufschreiben laesst: die beiden Enden, an denen Bytes hereinkommen und
//! hinausgehen, liegen dort und nirgends sonst. Gehalten wird auch dort
//! nichts; was der Editor ueber die geoeffnete Datei weiss, steht in
//! `krk-ui`'s `editormodell.rs`.
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

pub mod datei;
pub mod marke;
pub mod suche;
pub mod zeilen;

// Die Typen stehen hier, die Funktionen nicht: `text::alle` und
// `text::naechster` sagten am Aufrufort nicht mehr, wovon sie handeln.
// `suche::alle` und `suche::naechster` sagen es. `datei::einlesen`,
// `datei::sichern` und `datei::in_gehaltene_form` folgen derselben Regel und
// bleiben deshalb unter ihrem Modulnamen.
pub use marke::{Fund, Markensprung};
pub use suche::{Ersetzung, Sammelersetzung, Treffer};
pub use zeilen::{Zeilenindex, Zeilenlage, Zeilensprung};
