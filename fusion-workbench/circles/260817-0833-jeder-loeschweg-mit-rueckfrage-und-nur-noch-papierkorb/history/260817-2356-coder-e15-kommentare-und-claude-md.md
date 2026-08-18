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

---
**Addendum 260818-0201 (analyst) — the sentence "Die übrige Datei ist gegen den Baum nachgezählt
und stimmt" claims a wider scope than was read, and is corrected here rather than in the body.**

This log records a state and keeps its wording; what follows narrows the claim and says what a
re-measurement at `ae665e5` actually found. The finding that prompted it is
`issues/260818-0029_*_the-record-claims-the-rest-of-claude-md-was-counted-against-the-tree.md`.

**What the step measured, and what holds.** The four enumerations named in the body, re-run:

| claim in `CLAUDE.md` | command | result |
|---|---|---|
| `Wirkungsbereich` seven | `awk '/^pub enum Wirkungsbereich/,/^}/' crates/krk-core/src/tasten/belegung.rs \| grep -cE '^    [A-ZÄÖÜ][A-Za-z]*,'` | `7` |
| `Bereich` five | same shape over `crates/krk-ui/src/fenstermodell.rs` | `5` |
| `Fokus` five | same shape over `crates/krk-ui/src/kommandos/fokus.rs` | `5` |
| `Kommando` carries no Git variant | `awk '/^pub enum Kommando/,/^}/' … \| grep -ciE 'git'` | `0` |

Four for four, and the edited Rückschritt paragraph holds too: `delete` reaches
`loeschen_nach_rueckfrage` through `papierkorb_oder_zeichen_zurueck` and `in_den_papierkorb`, the
rule has one caller, and `die_regel_hat_genau_einen_aufrufer` holds the count.

**Five further statements were checked for this addendum, and all five hold.** They were not part
of the step and are recorded so that the next reader knows which ground has been walked:

- `#![allow(unsafe_code)]` stands in exactly two files, `krk-core/src/verzeichnis/sys.rs:88` and
  `krk-ui/src/appkit/mod.rs:1`. The five further grep hits are prose about the rule, not sites.
- The `# Ab welchem macOS…` section stands in 38 of the 40 files under `crates/krk-ui/src/appkit/`,
  and the two without it are `koordinaten.rs` and `mod.rs` — the two `CLAUDE.md` names.
- `rust-toolchain.toml` pins `1.97.1` and both Mac architectures.
- `Cargo.lock` carries no `cc` and no `-sys` package but `windows-sys`.
- There is one hull around `NSPasteboard`, `crates/krk-ui/src/appkit/zwischenablage.rs`; the other
  files matching the name only discuss it.

**What does not hold, and it is two statements, both in the sections the phrase "die übrige Datei"
covers.**

| statement | measured at `ae665e5` |
|---|---|
| `:24` region, "**Zehn Runden sind gefahren.**" with a ten-row table | `ls fusion-workbench/circles/*/*_circle.md` gives fourteen records: ten `_b_`, one `_c_`, one `_t_`, one `_a_` (never run) and one `_d_` (deferred). Twelve have been driven. Filed as `shared/issues/260816-2138_*_claude-md-nennt-zehn-gefahrene-runden-es-sind-elf.md`, itself now one round out of date. |
| `:39`, "liegt als `v0.4.1` aus" | `Cargo.toml:13` reads `0.5.1`, and `git tag` shows `v0.4.3`, `v0.4.4`, `v0.5.0`, `v0.5.1` standing after `v0.4.1`. Filed as `shared/issues/260818-0028_*_claude-md-says-the-bundle-ships-as-v0-4-1-and-four-tags-have-been-set-since.md`. |

Two further sentences drift with the round count rather than independently, and are named so that
whoever corrects the count corrects them in the same pass: `:39` "Was die Runden 2 bis 10
hinzugefügt haben", and `:78` "er liegt vor den Runden 5 bis 10 — keine der sechs ist gegen die
zehn Zusagen gemessen". Both are arithmetic on the same wrong ten.

**Nothing was changed in `CLAUDE.md` under this addendum.** Both defects have records of their own
in the shared store, which is where they belong: neither arose from this Circle's Directive, and
`CLAUDE.md` is outside the scope of the task that wrote this addendum.

**The lesson worth keeping, and it is about the sentence rather than the file.** "Die übrige Datei
ist nachgezählt" states a check over a surface nobody read. A claim that names its scope — these
four enumerations, by these four commands — survives the next reader, because the next reader can
see where it stops. The wider form is worse than no claim at all: it stops the next pass from
looking.
