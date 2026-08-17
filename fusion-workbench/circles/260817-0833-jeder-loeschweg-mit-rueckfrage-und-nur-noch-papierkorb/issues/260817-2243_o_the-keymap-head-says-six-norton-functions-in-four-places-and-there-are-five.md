The keymap head says "six" Norton functions in four places and there are five

---
`resources/default-keymap.toml` states the size of the Norton row four times, and all four say
six. After `82707ef` removed the `endgueltig_loeschen` entry the block holds **five** functions
with five Cmd shortcuts. No step of the remaining plan reaches the file, so nothing downstream
will correct the number.

---

**Severity:** Medium. No behaviour depends on it, but this is the file the project names as the
single source of the shipped key bindings, and its head is where a later round reads how large
the Norton row is. Four statements of one fact, all off by one, is the pattern this project has
filed against itself before (`shared/issues/260812-2253_*`, `shared/issues/260812-1438_*`).
**Found by:** coderev, review `reviews/260817-2243-coderev-bundle-d-the-removal.md`
**Affected:** `resources/default-keymap.toml:9`, `:170`, `:640`, `:849`
**Tree state:** `f7a85c1`
**Domain:** data

## Measured

The Norton block runs from the section header at `:129` to the `bearbeiten` entry at `:161`, and
the same file excludes `bearbeiten` from the two-ways rule in its own comment at `:170`:

```
$ sed -n '129,161p' resources/default-keymap.toml | grep '^id = '
id = "vorschau_umschalten"     f3, cmd+y
id = "kopieren"                f5, shift+cmd+k
id = "verschieben"             f6, shift+cmd+v
id = "ordner_anlegen"          f7, shift+cmd+n
id = "in_papierkorb"           delete, cmd+delete, f8
```

Five functions, five Cmd shortcuts. Before `82707ef` the block also held
`endgueltig_loeschen` (`f8`, `opt+cmd+delete`), which made it six and six.

The four statements:

| Line | Text | Is |
|---|---|---|
| 9 | "Die sechs Norton-Funktionen und ihre Cmd-Kuerzel stehen dort in der Tabelle" | five |
| 170 | "Die Zwei-Wege-Regel aus C3 gilt den sechs Funktionen der Norton-Reihe ganz oben" | five |
| 640 | "gilt fuer die sechs Funktionen der Norton-Reihe ganz oben und nicht fuer jede Funktion" | five |
| 849 | "anders als bei den sechs Cmd-Kuerzeln der Norton-Reihe oben" | five |

## Why nothing downstream catches it

Step 13 of the plan named exactly two head changes for the `ontocoder` — the count
"85 Funktionen mit zusammen 90 Kombinationen" and the binding decision reference — and both
were made. It said nothing about these four.

Step 15 sweeps the comments of the tree, and its scope is
`grep -rniE "endgueltig|endgültig" --include="*.rs" crates`. That search reaches neither
`resources/` nor a `.toml` file, and none of the four lines carries the word anyway.

Step 17 rewrites the round-1 spec, where the table "Die ausgelieferten Cmd-Kuerzel" loses its
`Endgültig löschen | F8 | Cmd+Opt+Delete` row and drops to five rows. That makes line 9 point at
a five-row table while saying six, and it does not touch this file.

## Direction

Four occurrences, one fact. Set all four to five in one edit, and keep line 9's wording pointing
at the round-1 table so that step 17 and this file end up saying the same number. Line 849 is
about the Cmd shortcuts rather than the functions; both counts are five, so the same number
serves.
