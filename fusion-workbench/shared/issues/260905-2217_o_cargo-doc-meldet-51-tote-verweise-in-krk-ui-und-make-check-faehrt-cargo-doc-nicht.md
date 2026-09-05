`cargo doc` meldet 51 tote Verweise in `krk-ui`, und kein Abnahmekommando faehrt `cargo doc`

---

`cargo doc -p krk-ui --no-deps --document-private-items` meldet 51 unaufloesbare
Doc-Verweise. Keines der vier Abnahmekommandos aus `CLAUDE.md` faehrt `cargo doc`, also faengt
sie nichts: weder `cargo build`, noch `cargo test`, noch `cargo clippy --all-targets -- -D
warnings`, noch `cargo fmt --check`. Ein Verweis in eckigen Klammern, der ins Leere zeigt,
sieht im Quelltext wie ein gepruefter aus und fuehrt den naechsten Leser auf einen Namen, den
es nicht gibt.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** niedrig

## Gemessen

```
export PATH="$HOME/.cargo/bin:$PATH"
touch crates/krk-ui/src/main.rs
cargo doc -p krk-ui --no-deps --document-private-items 2>&1 | grep -c 'unresolved link'
-> 51
```

Am Baumstand von HEAD `28c4a47` plus den Aenderungen dieser Sitzung; die Zahl stand vor der
Sitzung ebenso auf 51 und ist durch sie weder gestiegen noch gefallen. Sie ist hier gestempelt
und nicht behauptet: wer sie braucht, faehrt das Kommando.

Die Verweise fallen in erkennbare Gruppen. Ein Teil zeigt auf Proben unter `mod tests`
(`tests::die_gebaute_flaeche_steht_auf_textkit_1` und Geschwister); ein Teil auf private
Nachbarmodule ueber den vollen Pfad (`crate::appkit::fsevents`, `::volumes`, `::anwendung`);
ein Teil auf Namen, die es nicht mehr gibt (`Self::sitzung_laden`, `Self::offenes_blatt`,
`super::editor::textflaeche_bauen`, `DateifensterQuelle::auswahl_verschieben`); ein Teil auf
Fremdnamen ohne Import (`io::ErrorKind::AlreadyExists`, `NSFontDescriptor`, `Kommando`).

## Wie er gefunden wurde

Beim Beheben von
`260826-1327_*_zwei-tote-verweise-ein-umlaut-im-probennamen-und-zweimal-messt-im-pruefmodul-des-editors.md`,
das zwei dieser Verweise namentlich fuehrt und den Satz enthaelt „`cargo doc` meldet den toten
Verweis; `make check` faehrt `cargo doc` nicht". Die zwei sind behoben; der Satz daneben
beschreibt eine Luecke im Abnahmelauf, die keinen eigenen Datensatz hat und um 49 weitere
Stellen groesser ist als jener Datensatz vermuten laesst.

## Abnahme

Zwei Haelften, und die zweite haengt an einer Nutzerfrage.

1. Die 51 Stellen sind berichtigt oder als Nicht-Verweis geschrieben (Backticks statt eckiger
   Klammern), und der Lauf oben liefert 0.
2. Ob `cargo doc` in die Abnahmekette kommt — als fuenftes Kommando in `make check`, mit
   `RUSTDOCFLAGS="-D warnings"` — ist eine Nutzerentscheidung mit Kosten: der Lauf dauert
   laenger als die vier heutigen zusammen, und ohne ihn faellt die erste Haelfte beim
   naechsten Umbenennen wieder auseinander. Ohne die zweite Haelfte ist die erste eine
   Berichtigung ohne Verankerung, genau die schwaechste der drei Moeglichkeiten, die
   `CLAUDE.md` fuer Zahlen in Prosa beschreibt.
