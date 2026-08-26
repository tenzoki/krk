must_use fehlt an fast jeder reinen Antwort der Vorgangsmaschine und des Stapelumbenennens

---

Die Marke `#[must_use]` steht in beiden Modulen des Umfangs an vierzehn Stellen, und alle
vierzehn liegen in drei Dateien. In den uebrigen elf steht sie nirgends — auch nicht an
Funktionen, die nichts tun ausser zu antworten, und deren fallen gelassener Wert genau der stille
Ausfall waere, gegen den die Regel dieses Vorhabens sie fordert.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die Regel, gegen die gemessen ist

`CLAUDE.md`: "Ein Rueckgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt
`#[must_use]` — nicht eine Konvention in Kommentaren." So entschieden vom Nutzer am 260811-2140.
Wie sie anderswo im Kern angewandt wird, sagt `verzeichnis/filter.rs:154-157` wortwoertlich:
"`#[must_use]`, weil der Aufruf nichts tut ausser zu antworten: wer den Wert fallen laesst, hat
ihn umsonst geholt, und still." Dasselbe Kriterium an `verzeichnis/umfang.rs:215-218`.

## Wo sie im Umfang steht und wo nicht

Steht (nachgezaehlt mit `grep -rn '#\[must_use' crates/krk-core/src/operation/
crates/krk-core/src/stapelumbenennen/`):

- `operation/zippen.rs` — sieben: der Typ `Packschritt` und die fuenf Wahl-Funktionen
- `operation/mod.rs` — vier: die Typen `Ablauf` und `Zielentscheid`, je mit ausgeschriebener
  Begruendung
- `operation/auftrag.rs` — zwei: `mit_konfliktregel` und `mit_uebertragung`

Steht nicht, in keiner einzigen Zeile: `operation/fortschritt.rs`, `operation/kopieren.rs`,
`operation/verschieben.rs`, `operation/umbenennen.rs`, `operation/loeschen.rs`,
`operation/entpacken.rs`, `operation/anlegen.rs` und alle vier Dateien unter
`stapelumbenennen/`.

## Die vier schaerfsten Faelle

1. **`Vorschau::auszufuehren`** (`stapelumbenennen/vorschau.rs:64-66`) liefert ein `impl
   Iterator`. Ein fallen gelassener Iterator tut buchstaeblich nichts: er wird nie ausgewertet,
   und weder der Uebersetzer noch eine Probe sagt etwas. Es ist zugleich die eine Auskunft
   darueber, was die Ausfuehrung anfasst (Modulkopf `vorschau.rs:13-14`).
2. **`Regel::anwenden`** (`stapelumbenennen/regel.rs:106-117`) rechnet den neuen Namen und
   veraendert nichts. Fallen gelassen bleibt der Eintrag stehen, ohne Meldung.
3. **`kollision::pruefen`** (`stapelumbenennen/kollision.rs:78`) und **`vorschau::vorschau`**
   (`vorschau.rs:74`) sind der ganze Ertrag ihres Aufrufs.
4. **`Abschluss::ist_abgebrochen`** (`operation/fortschritt.rs:64-66`) beantwortet die Frage, an
   der die Oberflaeche entscheidet, ob sie einen Abbruch meldet.

Daneben liegen rund dreissig weitere reine Antworten ohne Marke; die Liste ist mechanisch zu
gewinnen, indem man jede Funktion mit Rueckgabewert und ohne `&mut`-Parameter durchgeht.

## Warum das nicht bloss Kosmetik ist

`CLAUDE.md` haelt fest, dass der Bau die eigentliche Pruefung ist: `unused_must_use` ist erst
unter `-D warnings` ein Fehler, und `make check` faehrt Clippy mit dieser Einstellung. Eine Marke,
die dasteht, faengt den Fehler beim Bau; eine, die fehlt, faengt ihn nie. Die Ungleichheit
zwischen `zippen.rs` mit sieben Marken und `entpacken.rs` mit keiner ist dabei das eigentliche
Zeichen: die Regel wird pro Runde angewandt, in der jemand daran denkt, und nicht pro Datei.

## Umfang

`krk-core`, elf Dateien unter `operation/` und `stapelumbenennen/`. Reine Ergaenzung, kein
Verhaltensunterschied — ausser dort, wo der Bau danach einen echten Ausfall findet.

Also seen: 260826-1221 by coderev — dieselbe Luecke im Verzeichnisbaum: `shared/issues/260826-1221_*_must-use-traegt-sieben-praedikate-des-verzeichnisbaums-und-zwanzig-gleichartige-daneben-nicht.md`. Die zwei Datensaetze sind derselbe Befund an zwei Umfaengen und keine Doppelung.
