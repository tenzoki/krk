# Coder — Schritt 11: der Bau, die Zählungen und die zwei Ausgaben gegen den Stand vor der Runde

**Datum:** 260829-1220
**Circle:** 260828-1041-dateilistenfilter-nimmt-eingaben-per-paste
**Plan:** `planning/260829-1102_p_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, Schritt 11
**Status:** Complete

## Geänderte Dateien

Nichts im Baum. Geändert sind allein dieser Eintrag und der Planschritt 11 auf `[DONE]`. Nicht committet.

## Aufbau

Der Stand vor der Runde ist `c6c86cb`, der Stand nach Schritt 10 ist `415ef6f` (HEAD; 35 Dateien, +2720/−242). Beide Stände liegen als Kopie aus `git archive <commit> | tar -x` unter `~/Library/Caches/krk-messplatz/runde21-vorher-c6c86cb/` und `~/Library/Caches/krk-messplatz/runde21-neu-415ef6f/` und sind dort je mit `make menue` gebaut (eigener `target/`, kein Worktree, kein Stash, kein Checkout im Arbeitsbaum). Beide Kopien, weil im Arbeitsbaum zugleich `make check` lief und ein zweiter `cargo` denselben `target/` gehalten hätte.

## C4.4: `make check`

`make check` — exit 0 (Build, Test, Clippy, fmt über den ganzen Arbeitsbereich). 23 Probensätze, keiner mit `failed`. Die vierte Stufe ist `cargo clippy --workspace --all-targets -- -D warnings`, `Finished` ohne Warnung; das Makefile meldet „alle vier gruen".

## C1.9, Schritt 4: die Belegungsdatei trägt allein Kommentarzeilen

- `git diff -I '^\s*#' c6c86cb HEAD -- resources/default-keymap.toml`: leer (0 Zeilen).
- `git diff --stat`: 23 Einfügungen, 11 Löschungen, alle Kommentarzeilen; die Zahl der geänderten Nicht-Kommentar-Zeilen im Diff ist 0.
- Blob alt `b32b3592dad75c49cf730d2b66b238eae25ebf35`, Blob neu `ca8943cd99ca381ec50012dd51d6384a48073d62`.
- Tastenhälfte (`grep -v '^#'` beidseits): `diff` leer, sha256 beidseits `b9295a029b81259a97541fd51c4dfee6dbdaafde3ef37549186f8772eeef8702`.

## C7.4: keine neue Kiste

- `git diff c6c86cb HEAD -- Cargo.lock Cargo.toml`: leer.
- `grep -n 'name = "cc"\|-sys"' Cargo.lock`: drei Treffer, alle `windows-sys` (Zeilen 108, 862, 872). Kein `cc`.
- sha256 `Cargo.lock` `4e93c8984fd08c64c5274e4a75c6a3247adae10ca4d4ffc690273b7249065328`, `Cargo.toml` `bc0eb81948dcfba3777d399683fedafc1a06bf2fd6c3619081b177127e5b0cf1`.

## Constraint 2 und A12: die Aufzählungen

- `awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs | grep -c ','`: 167 auf `c6c86cb`, 167 auf HEAD.
- `Kontextbefehl` (`crates/krk-ui/src/kommandos/kontextmenue.rs`): drei Werte, `Zippen`, `Entpacken`, `ImFinderZeigen`.

## C1.9: `make menue` alt gegen neu

`make menue` schreibt vor dem Menü fünf Zeilen von `cargo xtask bundle`, die sich allein im Bündelpfad der jeweiligen Kopie unterscheiden. Ab Zeile 6 steht das Menü, 95 Zeilen, und dort ist der Diff leer:

```
diff menue-alt-nur.txt menue-neu-nur.txt   → exit 0
sha256 alt: 8355633a32004627c465a762adf7ed03ae9d639c21165b06cd73b2161a8d0173
sha256 neu: 8355633a32004627c465a762adf7ed03ae9d639c21165b06cd73b2161a8d0173
```

Das ist dieselbe Prüfsumme, die der Eintrag der Runde 22 (`circles/260828-2349-…/history/260829-0210-…`) für das Menü hält. Die ungekürzten Ausgaben tragen `75d1187d…` (alt) und `b093282b…` (neu); der Unterschied sind die Pfadzeilen des Bündelbaus. Beide `make menue` exit 0.

`make tasten` ist wie in der Runde 22 nicht kopflos fahrbar (endet allein mit Cmd+Q im Vordergrund); an seine Stelle tritt der leere Diff der Belegungsquelle ohne Kommentare, siehe oben.

## C4.1: `NSPasteboard` außerhalb der Hülle

`grep -rn NSPasteboard crates/krk-ui/src` ohne `appkit/zwischenablage.rs`: 30 Stellen auf `c6c86cb`, 30 auf HEAD, in denselben sechs Dateien (`abwurf.rs`, `betrachter.rs`, `mod.rs`, `teilen.rs`, `vorschau.rs`, `kommandos/operationen.rs`). Ohne Zeilennummern verglichen sind die Zeilen zeichengleich; verschoben sind allein `betrachter.rs:320→321`, `:715→716` und `operationen.rs:1132→1135` durch Prosa davor.

## Constraint 5: die zehn Zusagen

`grep -oE '"L[0-9]+"' crates/krk-bench/src/messen.rs | sort -u`: `L1` bis `L10`, beidseits dieselben zehn.

## Verifikation

`make check` — exit 0.
