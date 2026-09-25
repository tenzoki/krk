# Das Leistenmodell zieht auf die neue Lesezeichenschnittstelle nach

---
**Agent:** coder
**Status:** Complete
**Anlass:** Nacharbeit zu S11 — `cargo build --workspace` brach an zwei Aufrufstellen in `crates/krk-ui/src/leistenmodell.rs` ab
**Umfang:** ausschließlich `crates/krk-ui/src/leistenmodell.rs`

---

## Was gebaut wurde

S11 hat `krk_core::ablage::Lesezeichen` von einem Feld `ordner` auf ein Feld
`ziel: Ziel` umgestellt und `Lesezeichenliste::anlegen` von `&Path` auf `Ziel`.
Zwei Stellen im Leistenmodell haben die alte Form gelesen und sind nachgezogen,
ohne dass sich am Verhalten des Ordnerfalls etwas ändert.

**`Leistenmodell::gewaehlt` (`leistenmodell.rs:345`)** liest den Pfad jetzt aus
`Ziel::Ordner` statt aus dem entfallenen Feld `ordner`. Eine Textmarke liefert
`None`. Das ist die einfachste ehrliche Form, die der heutige Typ zulässt:
`Auswahl` trägt ein Pflichtfeld `ordner`, und eine Textmarke hat keinen Ordner.
Die Datei einer Textmarke dort einzutragen hieße, dem Aufrufer einen Ordner zu
melden, den er lesen würde; `gueltig: false` zu melden hieße, die Statuszeile
"fehlt" sagen zu lassen, obwohl die Datei da ist. `None` ist ein Ausgang, den
`gewaehlt` schon vorher hatte, und alle Aufrufer behandeln ihn.

**`Leistenmodell::anlegen` (`leistenmodell.rs:392`)** verpackt den Pfad in
`Ziel::Ordner` und behält seine Signatur mit `&Path`. Keine zweite Sorte wird
vorgebaut.

Dazu eine Probe, `eine_textmarke_liefert_bis_s39_keine_auswahl`, die den
Platzhalter festhält, damit niemand später die Datei einer Textmarke als Ordner
ausgibt.

## Was S38 und S39 ablösen müssen

Drei Stellen tragen den Vorbehalt im Kommentar:

- **S39** teilt `gewaehlt` (oder seinen Aufrufer) nach der Sorte auf. Bis dahin
  bleibt die Auswahl einer Textmarke folgenlos, und der Name einer Textmarke
  erreicht die Vorbelegung des Umbenennungsblattes nicht
  (`appkit/leiste.rs::gewaehlter_lesezeichenname` geht über `gewaehlt`).
- **S39** löst außerdem die Probe `eine_textmarke_liefert_bis_s39_keine_auswahl`
  ab.
- **S38** entscheidet, ob `anlegen` das fertige `Ziel` entgegennimmt oder ein
  zweites Gegenstück bekommt. Vorher ist nicht zu entscheiden, woher Datei,
  Zeile und Zeileninhalt kommen.

Bis S38 das Anlegen bringt, entsteht in KRK überhaupt keine Textmarke; eine von
Hand in `bookmarks.toml` eingetragene ist der einzige Weg, den Platzhalter zu
erreichen.

## Abnahme

Alle vier Kommandos aus `make check` laufen grün:

- `cargo build --workspace` — fertig ohne Fehler
- `cargo test --workspace` — 0 Fehlschläge über alle Kisten, 1 übersprungene
  Probe wie zuvor
- `cargo clippy --workspace --all-targets` — keine Warnung
- `cargo fmt --all --check` — keine Ausgabe

## Geänderte Dateien

- `crates/krk-ui/src/leistenmodell.rs`
