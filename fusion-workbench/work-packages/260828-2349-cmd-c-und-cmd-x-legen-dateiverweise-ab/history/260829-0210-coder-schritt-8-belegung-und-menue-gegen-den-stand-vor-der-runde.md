# Coder: Schritt 8 der Runde 22, die Belegungs- und Menüausgabe gegen den Stand vor der Runde

**Date:** 2026-08-29
**Status:** Complete
**Plan:** `planning/260829-0006_p_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`, Schritt 8
**Agent:** coder

## Was geändert ist

Nichts im Baum. Geändert sind allein dieser Eintrag und der Planschritt 8 auf `[DONE]`.

## Aufbau

Der Stand vor der Runde ist `83e011c`, der Stand nach Schritt 7 ist `023ee64` (HEAD). Der alte Stand liegt als Kopie aus `git archive 83e011c | tar -x` unter `~/Library/Caches/krk-messplatz/runde22-vorher-83e011c/` und ist dort mit `make menue` gebaut (eigener `target/`, kein Worktree, kein Checkout im Arbeitsbaum). Der neue Stand ist im Arbeitsbaum mit `make menue` gebaut; seine Ausgabe liegt unter `~/Library/Caches/krk-messplatz/runde22-neu-023ee64/`.

## Menühälfte von C1.11: der Diff ist leer

`make menue` schreibt vor dem Menü fünf Zeilen von `cargo xtask bundle`, die den Pfad des Bündels tragen; die unterscheiden sich zwischen beiden Kopien allein im Pfad (`…/krk-messplatz/runde22-vorher-83e011c/target/KRK.app` gegen `…/productive/krk/target/KRK.app`). Ab Zeile 6 steht das Menü, 95 Zeilen, und dort ist der Diff leer:

```
diff menue-alt-menue-nur.txt menue-neu-menue-nur.txt   → exit 0
sha256 alt: 8355633a32004627c465a762adf7ed03ae9d639c21165b06cd73b2161a8d0173
sha256 neu: 8355633a32004627c465a762adf7ed03ae9d639c21165b06cd73b2161a8d0173
```

Die ungekürzten Ausgaben tragen `2a53d0ab…` (alt) und `5fe3d6ef…` (neu); der Unterschied sind die drei Pfadzeilen des Bündelbaus und nichts am Menü.

## Tastenhälfte von C1.11: `make tasten` ist nicht kopflos fahrbar

`--tasten-protokoll` öffnet das Fenster, schreibt je Tastendruck eine Zeile (`ereignisse::protokollieren`) und endet weder an einer Zeitgrenze noch an EOF, sondern allein mit Cmd+Q im Vordergrund. Die Markdown-Ausgabe der Belegung (`belegungsausgabe.rs`) hängt am Menüeintrag und ist ohne Fenster nicht aus dem Binär erreichbar. Ersatz, den der Baum hergibt:

- `resources/default-keymap.toml` ist an beiden Ständen dasselbe Blob `b32b3592dad75c49cf730d2b66b238eae25ebf35`, sha256 `45fa805dfdadd60405d53c91583d466209a9651a9cd6e559e90cdfad09427eb5`.
- `git diff --stat 83e011c HEAD -- resources/default-keymap.toml crates/krk-core/src/tasten crates/krk-ui/src/belegungsausgabe.rs crates/krk-ui/src/belegungsmodell.rs crates/krk-ui/src/menuemodell.rs Cargo.lock` ist leer: weder die Belegungsquelle noch ihre Leser noch die Markdown-Ausgabe sind angefasst. Die Runde ändert acht Dateien, alle unter `crates/krk-ui/src/appkit/` und `crates/krk-ui/src/kommandos/`.
- Das Menü nimmt seine Kürzel aus derselben geladenen Belegung (`starten` in `anwendung.rs`), und der leere Menü-Diff zeigt damit auch die Kürzel unverändert.

Was ein Tastendruck im laufenden Fenster auslöst, prüft damit der Abnahmelauf (Schritt 9, C1.12, C3.7) und nicht dieser Schritt.

## C5.6 und C5.3

- `grep -n 'name = "cc"\|-sys"' Cargo.lock`: drei Treffer, alle `windows-sys` (Zeilen 108, 862, 872). Kein `cc`.
- `cargo clippy --workspace --all-targets -- -D warnings`: exit 0.
- `make check` (Build, Test, Clippy, fmt über den ganzen Arbeitsbereich): exit 0. Die drei Proben aus Schritt 3, die der Eintrag zu Schritt 7 im vollen Lauf rot gesehen hat (`der_zweite_ausgang_legt_verweise_und_namen_ab`, `ein_zweites_ablegen_ersetzt_das_erste`, `eine_verknuepfung_wird_als_verknuepfung_abgelegt`), sind in diesem Lauf grün; ob das die Lage oder ein Zufall der Reihenfolge ist, sagt ein Lauf nicht.

## Verifikation

`make check` — exit 0.
