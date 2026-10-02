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

---
Resolved: Die erste Hälfte der Abnahme ist erfüllt und mit ihr mehr, als der
Datensatz gemessen hat. Die zweite bleibt offen und hat einen anderen Ort.

**Die 51 sind 0.** Der Lauf aus dem Abschnitt „Gemessen" liefert am heutigen
Baum 0 statt 51:

```
export PATH="$HOME/.cargo/bin:$PATH"
touch crates/krk-ui/src/main.rs
cargo doc -p krk-ui --no-deps --document-private-items 2>&1 | grep -c 'unresolved link'
-> 0
```

**Der Datensatz misst dabei nur eine der vier Arten.** `krk-ui` trug 85
Doc-Warnungen und nicht 51: 51 unaufgelöste Verweise, 30 überflüssige
Verweisziele und 4 mehrdeutige Namen (Funktion zugleich Modul). Die 51 sind
richtig gezählt für das, was das Kommando zählt; die anderen 34 hätten
`RUSTDOCFLAGS="-D warnings"` genauso rot gehalten. Alle 85 sind geräumt:
`RUSTDOCFLAGS="-D warnings" cargo doc -p krk-ui --no-deps` gibt Exit 0 und keine
Warnung aus.

Die vierte Art, „public documentation for … links to private item", kommt in
`krk-ui` nicht vor und konnte es nicht: die Kiste hat kein Bibliotheksziel und
keine öffentliche Fläche, an der rustdoc die Meldung stellen könnte. Ihre
`crate::appkit::…`-Verweise meldet rustdoc stattdessen als `unresolved link`,
und zwar auch unter `--document-private-items` — nachgemessen —, denn
`crate::appkit::fsevents` ist von `crate::auffrischung` aus sichtbarkeitshalber
kein gültiger Pfad. `#![allow(rustdoc::private_intra_doc_links)]` hätte in
dieser Kiste keine einzige Meldung gedeckt. Sie sind zu Fließtext geworden, was
der Empfehlung von
`shared/decisions/260905-2336_*_wird-ein-privates-element-oeffentlich-oder-der-verweis-darauf-zu-fliesstext.md`
für genau diese Gruppe entspricht (dort Option 2 für die Modulverweise);
`appkit/mod.rs` schreibt kistenübergreifende Verweise seit jeher so.

**Die zweite Hälfte ist nicht beantwortet und steht weiter zur Entscheidung.**
Ob `cargo doc` als fünftes Kommando in `make check` kommt, gehört dem Nutzer und
liegt in demselben Entscheid `260905-2336`, Abschnitt „Randbedingungen" und
„Empfehlung". Der Datensatz hier wird geschlossen, weil sein Defekt nicht mehr
besteht; die Frage bleibt an ihrem Ort offen. Ohne sie fällt die Räumung beim
nächsten Umbenennen wieder auseinander, und das ist ausdrücklich nicht behoben.
