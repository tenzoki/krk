# D14 — The prose counts nobody holds

**Status:** Complete
**Agent:** coder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Source record:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, step 14 (third step of bundle D)
**Tree state before the task:** `82707ef` (steps 12 and 13), `make check` green
**Verification:** `make check` — exit 0 (all four acceptance commands)

> This log is written in English because `CLAUDE.md` declares `**Artifact language:** en`.
> The source files stay German, as does every comment in them.

## The measurement came first

Three counts were taken against the tree before a single number was written:

```
grep -c '^\[\[funktion\]\]' resources/default-keymap.toml            -> 84
Kommando::KENNUNGEN: [(Kommando, &'static str); 78]                  -> 78
grep '^tasten = ' | grep -o '"[^"]*"' | wc -l                        -> 89
```

The split between functions with and without a command was measured too, not subtracted: every
`id` of the key map was set against every kennung of `Kommando::KENNUNGEN` with `comm`. Exactly six
ids appear only in the key map — `text_alles_auswaehlen`, `text_ausschneiden`, `text_einfuegen`,
`text_kopieren`, `text_rueckgaengig`, `text_wiederholen` — and none appears only in the enumeration.
So the six delivered text commands stay six, and every prose sentence that says "sechs" was left
alone.

## Seven places, not eight

The plan named eight lines. Six of them were stale and were pulled along; one more was stale and was
pulled along; the eighth was already right and was not touched.

| Place | old | new | measured with |
|---|---|---|---|
| `belegungsausgabe.rs:45` | alle 85 Funktionen | alle 84 Funktionen | `grep -c '^\[\[funktion\]\]'` |
| `belegungsausgabe.rs:48` | die 79 Funktionen mit `Kommando` | die 78 | `Kommando::KENNUNGEN.len()` |
| `belegungsausgabe.rs:56` | die 79 mit `Kommando` (table row) | die 78 | `Kommando::KENNUNGEN.len()` |
| `belegungsausgabe.rs:256` | 79 der 85 | 78 der 84 | both of the above |
| `belegungsausgabe.rs:730` | alle 85 | alle 84 | `grep -c '^\[\[funktion\]\]'` |
| `belegungsausgabe.rs:731` | die 79 mit Kommando | die 78 | `Kommando::KENNUNGEN.len()` |
| `menue.rs:867` | 79 der 85 | 78 der 84 | both of the above |
| `menue.rs:128` | 84 Funktionen | unchanged | `grep -c '^\[\[funktion\]\]'` says 84 |

**`menue.rs:128` was stale before this session and is correct now by accident.** `git show
82707ef^:crates/krk-ui/src/appkit/menue.rs` shows it already read 84 while the tree carried 85; the
drop of `endgueltig_loeschen` caught up with it. Changing it would have made it wrong. The same
holds for a ninth place the plan does not name, `menue.rs:799` ("als einzige der 84 Funktionen"),
which was 84 before and is 84 now.

So: seven edits, all one-digit substitutions of equal width, no line reflowed.

## What was checked and left alone

- **`belegungsausgabe.rs:758` keeps `assert_eq!(mit_kommando, Kommando::KENNUNGEN.len(), …)`.** That
  is the one place where the relation is held by the compiler and the probes rather than by prose,
  and the acceptance criterion names it. Untouched.
- **Every "sechs" in both files.** The count of functions without a command did not move.
- **The counts of the four justification layers, the five factory-free functions, the nine areas.**
  None of them depends on the number of functions.

## One place outside the scope

`crates/krk-core/src/tasten/belegung.rs:407` carries "die Kennung folgt der Schreibweise der 77
vorhandenen" in the doc comment of `Kommando::TiefeSucheUmschalten`. It is the same class of prose
count. **It was not edited**, both because the step limits itself to two files and because the
number happens to be right now: 78 kennungen minus the one the sentence is about is 77. It was
wrong before step 12 and is right after it, exactly like `menue.rs:128`. Step 15, the broad prose
pass, should read it rather than assume it.

A tree-wide grep for count prose in the same shape (`(alle|der|die|von|ueber) <two-digit>`) over
`crates`, `resources`, `README.md` and `CLAUDE.md` turned up nothing else about functions, commands
or combinations. The remaining hits are the L9 percentages in `krk-bench`, which are a different
subject.

## The tree stays green

`make check` runs `cargo build --workspace`, `cargo test --workspace`, `cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings`, and exits 0. Nothing was committed.

## Files

- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/belegungsausgabe.rs`
- `/Users/k1/Projects/productive/krk/crates/krk-ui/src/appkit/menue.rs`

## Records

- `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, step 14 → `[DONE]`
