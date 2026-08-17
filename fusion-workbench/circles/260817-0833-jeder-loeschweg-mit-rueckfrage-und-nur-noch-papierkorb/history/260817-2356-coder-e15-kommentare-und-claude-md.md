# Step 15 — Die Kommentare im Baum, CLAUDE.md und die Norton-Zahl der Belegung

**Agent:** coder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Plan:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, Bündel E, Schritt 15
**Tree state at start:** `8f556ed`
**Status:** Complete

## Was der Auftrag war

Die Prosa des Baums auf den Stand nach Bündel D bringen: keine Kommentar- oder
Modulkopfzeile behauptet noch einen zweiten Löschweg, ein Löschen ohne Rückfrage oder
`f8`/`opt+cmd+delete` als endgültigen Weg. Dazu Zeile 139 von `CLAUDE.md` und, als
angehängter Befund aus der Durchsicht des Bündels D, die vier Stellen in
`resources/default-keymap.toml`, die die Größe der Norton-Reihe mit „sechs" angeben
(`issues/260817-2243_o_the-keymap-head-says-six-norton-functions-…`).

## Die eigene Erhebung gegen die Zahlen des Plans

`grep -rniE "endgueltig|endgültig" --include="*.rs" crates` liefert am 260817-2340
**51 Zeilen in 24 Dateien**. Der Plan nennt 79 Zeilen in 21 Dateien, der Spec 46
Nennungen — beide vom 260817 und beide vor den Bündeln D. Die Schritte 12 bis 14 haben
den Unterschied abgetragen. Von den 51 Zeilen waren 22 nachzuziehen; die übrigen 29 sind
Dateinamen von Entscheidungsdatensätzen, datierte Rückblicke, die seit den Bündeln C und D
richtig dastehen, oder Verwendungen des Wortes in seiner allgemeinen Bedeutung
(„die endgültige Reihenfolge", „die endgültige Antwort").

Nach dem Durchgang liefert dieselbe Suche 34 Zeilen; keine davon behauptet einen zweiten
Löschweg. Ein zweiter Durchgang über
`grep -rniE "ohne rueckfrage|ohne nachfrage|opt\+cmd\+delete|beide loeschbefehle"` hat drei
weitere Stellen gefunden, die das Wort „endgueltig" gar nicht tragen und trotzdem falsch
standen (`loeschbestaetigung.rs` zweimal, `anwendung.rs` einmal).

## Geändert

### `crates/`

| Datei | Stellen | Was |
|---|---|---|
| `krk-core/src/operation/loeschen.rs` | Modulkopf 1–7 | „Zwei Wege" → ein Weg, mit dem überholenden Datensatz statt des überholten |
| `krk-core/src/operation/auftrag.rs` | 8 | „Papierkorb und endgültiges Löschen" → „Papierkorb und Stapelumbenennen"; die zwei Arten ohne `ziel` |
| `krk-core/src/tasten/belegung.rs` | 126, 267 | Wirkungsbereich am Räumen statt am endgültigen Löschen erklärt |
| `krk-core/tests/belegung.rs` | 284–293 | Umschalt+Entf ohne die überholte Begründung; Opt+Cmd+Entf als zweite ab Werk freie Kombination benannt, mit dem Hinweis, dass die Prüfung sie nicht hält |
| `krk-ui/src/belegungsmodell.rs` | 904–906 | Papierkorb ist die eine Zeile des Löschwegs und nicht mehr eine von zweien |
| `krk-ui/src/auffrischung.rs` | 301, 311–312 | „sechste Operationsart" → fünfte, „die übrigen vier" → drei; `Art` trägt vier Werte |
| `krk-ui/src/kommandos/fokus.rs` | 107, 346, 429 | „vor der Rückfrage des endgültigen Löschens" → „bei stehender Löschrückfrage" |
| `krk-ui/src/kommandos/rueckschritt.rs` | 1–2, 16–24, `InDenPapierkorb` | die Begründung der Regel: die Rückfrage ist die zweite Sperre, der falsche Zweig fragt statt zu räumen, und die Unterscheidung bleibt trotzdem nötig |
| `krk-ui/src/kommandos/operationen.rs` | 143 | Rückfrage vor dem Räumen |
| `krk-ui/src/appkit/anwendung.rs` | 602, 4462 | dasselbe; der Doc-Kopf von `papierkorb_oder_zeichen_zurueck` nennt den dritten Ausgang mit seiner Rückfrage |
| `krk-ui/src/appkit/blaetter/mod.rs` | 5–8, 54, 284, 427, 566 | fünfmal die Benennung; `als_warnung` erklärt sich jetzt über die laute Form und nicht über „den einen Vorgang ohne Rückweg" |
| `krk-ui/src/appkit/blaetter/loeschbestaetigung.rs` | 3–4, 44–53 | „welcher der Löschbefehle" → welche der drei Tasten; „seit beide Löschbefehle" → seit der alltägliche Weg |
| `krk-ui/src/appkit/blaetter/konflikt.rs` | 11 | Benennung |
| `krk-ui/src/appkit/blaetter/ungesichert.rs` | 23 | Benennung |
| `krk-ui/src/appkit/blaetter/stapelumbenennen.rs` | 29 | Benennung |
| `krk-ui/src/appkit/hinweis.rs` | 75 | `Critical` wie bei der **lauten** Form; der Vergleich „Vorgang ohne Rückweg" trägt nicht mehr |

### `CLAUDE.md`

Der Absatz bei Zeile 140 sagt jetzt, dass jedem Räumen seit dem 260817 eine Rückfrage
vorausgeht, nennt `f8` neben `cmd+delete` und trägt einen Satz nach, warum die
Fallunterscheidung mit der Rückfrage milder, aber nicht überflüssig geworden ist. Die
übrige Datei ist gegen den Baum nachgezählt und stimmt: `Wirkungsbereich` sieben,
`Bereich` fünf, `Fokus` fünf, `Kommando` ohne Zahl und ohne Git-Variante. Der Absatz über
`kommandos/zulaessigkeit.rs` bleibt unangetastet.

### `resources/default-keymap.toml`

Die Zeilen 9, 170, 640 und 849 sagen „fünf" statt „sechs". Gemessen gegen die Datei: der
Norton-Block `:129`–`:161` trägt sechs `[[funktion]]`-Einträge, von denen `bearbeiten` seit
der Editor-Runde eine einzige Taste führt und im Kommentar bei `:169` ausdrücklich von der
Zwei-Wege-Regel ausgenommen ist. Die Reihe ist damit `vorschau_umschalten`, `kopieren`,
`verschieben`, `ordner_anlegen`, `in_papierkorb`: **fünf Funktionen, fünf Cmd-Kürzel**.

## Nicht geändert, mit Grund

- **`krk-ui/src/appkit/ereignisse.rs:307`** — der Plan nennt die Stelle; sie steht seit
  Bündel D richtig („raeumt seit dem Wegfall des endgueltigen Loeschens ebenso in jeder
  Lage").
- **`krk-ui/src/kommandos/rueckschritt.rs:88`, `belegungsmodell.rs:1182–1184`,
  `anwendung.rs:4481/4816/5691/6446`, `loeschwarnung.rs:167/254`** — datierte Rückblicke,
  in den Schritten 12 bis 14 geschrieben und richtig.
- **`verzeichnis/{arbeitsbaum,loeschzielbefund,umfang}.rs`, `loeschbestaetigung.rs:74`,
  `loeschen.rs:6`, `tests/belegung.rs:291`** — Dateinamen von Entscheidungsdatensätzen.
- **`modell.rs:464`, `zettelmodell.rs:454`, `anwendung.rs:844`, `hinweis.rs:31`,
  `papierkorb.rs:183`, `loeschen.rs:57`** — „endgültig" in seiner allgemeinen Bedeutung,
  ohne Bezug auf einen Löschweg.
- **`tests/belegung.rs:1622`–`:1660`** — die Probe über die zurückgezogene Kennung
  `endgueltig_loeschen`; sie muss die Kennung führen, um sie zu prüfen.
- **`loeschbestaetigung.rs:173`, `:180`** — Prüfkörper, kein Kommentar. Als Befund
  abgelegt: `issues/260817-2355_o_a-probe-fixture-still-labels-the-executing-button-…`.

## Abgelegte Befunde

- `issues/260817-2354_o_opt-cmd-delete-became-free-ex-works-and-the-probe-that-holds-that-list-checks-one.md`
  — die Prüfung der ab Werk freien Kombinationen hält eine von zweien. Der Kommentarblock
  darüber sagt die Lücke jetzt an Ort und Stelle; die Prüfung selbst zu erweitern wäre eine
  Änderung am Prüfkörper und liegt außerhalb dieses Schritts.
- `issues/260817-2355_o_a-probe-fixture-still-labels-the-executing-button-endgueltig-loeschen.md`
- `issues/260817-2243_c_the-keymap-head-says-six-…` — geschlossen, mit `Resolved:` und
  Marker `_o_` → `_c_`.

## Verification

`make check` — Exit 0. Alle vier Abnahmekommandos grün (`cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace
--all-targets -- -D warnings`).
