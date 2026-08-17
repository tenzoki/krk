The Norton block header promises a Cmd shortcut for every one of its six entries, and one of them has none

---
`resources/default-keymap.toml:129-131` heads the Norton block with "die Norton-Reihe, je zwei
Wege" and the sentence "Jede dieser Funktionen ist ueber die Funktionstaste und ueber ein
Cmd-Kuerzel erreichbar." The block holds six `[[funktion]]` entries; `bearbeiten` carries `["f4"]`
and no Cmd shortcut, and the comment beside it at `:169-171` says so explicitly.

---

**Severity:** Low. Nothing depends on it at run time, and the contradiction predates this round.
It is filed now because step 15 changed four other statements in this file from "sechs" to "fünf"
and left this one, so the file now says "die fünf Norton-Funktionen" four times while the header
of the block those four sentences point at claims that all of **these** — six entries — have two
ways. Before the change the two readings at least agreed with each other while both being wrong
about `bearbeiten`.

**Found by:** coderev, review `reviews/260818-0024-coderev-bundle-e-the-prose-and-the-records.md`
**Affected:** `resources/default-keymap.toml:129-131`
**Cross-references:** `issues/260817-2243_c_the-keymap-head-says-six-norton-functions-in-four-places-and-there-are-five.md`
(closed by step 15; its `## Direction` addressed the four counts and did not reach the block header)
**Tree state:** `da716c1`
**Domain:** data

## Measured

```
$ awk 'NR>=129 && NR<=131' resources/default-keymap.toml
# ── C3 und C4: die Norton-Reihe, je zwei Wege ────────────────────────────────
# Jede dieser Funktionen ist ueber die Funktionstaste und ueber ein
# Cmd-Kuerzel erreichbar. Beide Wege stehen in derselben Zeile.
```

The six entries between that header and the next section header at `:176`:

| id | tasten | Cmd shortcut |
|---|---|---|
| `vorschau_umschalten` | `["f3", "cmd+y"]` | yes |
| `kopieren` | `["f5", "shift+cmd+k"]` | yes |
| `verschieben` | `["f6", "shift+cmd+v"]` | yes |
| `ordner_anlegen` | `["f7", "shift+cmd+n"]` | yes |
| `in_papierkorb` | `["delete", "cmd+delete", "f8"]` | yes |
| `bearbeiten` | `["f4"]` | **no** |

`:169-171`, inside the block, states the exception: "Es bleibt bei der einen Taste. Die
Zwei-Wege-Regel aus C3 gilt den fuenf Funktionen der Norton-Reihe ganz oben; `bearbeiten` gehoert
zu den spaeteren Funktionen, die je eine tragen."

The round-1 spec, rewritten in step 17, got this right: its entry at
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md:12`
says the block "fünf Funktionen mit je zwei Wegen führt und `bearbeiten` auf F4 im Kommentar
daneben ausdrücklich von der Zwei-Wege-Regel ausgenommen ist". Only the key map itself still
claims otherwise at its block header.

## Direction

Two words in the header carry it: say that the block holds six entries and that the two-ways rule
covers the first five, or move `bearbeiten` out of the block. The second is the larger change and
touches the display order — `belegungsmodell::nach_bereichen` returns a group in file order and
three consumers show it that way (`resources/default-keymap.toml:37-43`) — so the header wording
is the cheaper of the two and the one that keeps the file consistent with the round-1 spec.
