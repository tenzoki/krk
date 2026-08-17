The one place a polarity swap still compiles and passes every probe carries no probe of its own

---
`Anwendungsdelegierter::loeschtexte` (`crates/krk-ui/src/appkit/anwendung.rs:4840-4874`) is where
the five facts are wired into `Loeschziel`. Both first-polarity fields are filled there, and the
wiring is correct. Nothing measures it. The site is reachable from a probe — `loeschtexte` is an
associated function without `&self` and without a `MainThreadMarker`, and `anwendung.rs` already
carries two `#[cfg(test)]` modules — so the gap is a missing probe and not an unreachable one.

---

**Severity:** Medium. No wrong behaviour at today's tree; the field assignments are read and
correct. What is missing is the measurement at the exact site that record
`260817-1623_c_ist-lokal-returns-the-inverse-of-the-field-it-fills.md` described as the one where
a swap "would have compiled, passed every probe and exchanged local for remote". That sentence is
still true of the tree, and the record's own closure says so: "The swap is not uncompilable; it
has lost its occasion."
**Found by:** coderev, review `reviews/260817-1759-coderev-bundle-c-the-loud-confirmation.md`
**Affected:** `crates/krk-ui/src/appkit/anwendung.rs:4840-4874`
**Tree state:** `792995a`
**Domain:** code
**Cross-references:**
`issues/260817-1419_o_die-einzige-sicherung-gegen-den-polaritaetsfehler-ist-prosa-und-ist-warnwuerdig-hat-keinen-aufrufer.md`
(the same subject at type level; its second, stronger way is untouched),
`issues/260817-1623_c_ist-lokal-returns-the-inverse-of-the-field-it-fills.md` (the swap this site
would carry)

## What stands at the tree

Every part of the trigger chain is measured except the wiring:

| piece | where | measured by |
|---|---|---|
| `liegt_auf_netzlaufwerk` answers the trigger, not the resource value | `appkit/volumes.rs:259-289` | four probes, `volumes.rs:468-604` |
| `beruehrt_einen_arbeitsbaum` answers the trigger | `krk-core/.../arbeitsbaum.rs:342-349` | 13 in-file probes plus 11 in `tests/arbeitsbaum.rs` |
| `zaehlen` answers the sixth trigger | `krk-core/.../umfang.rs:217-301` | 2 in-file plus 8 in `tests/umfang.rs` |
| `warngruende` judges the five facts | `kommandos/loeschwarnung.rs:642-706` | 11 probes over the table |
| **the five facts reach the five fields** | `appkit/anwendung.rs:4840-4874` | **nothing** |

The two lines that carry the hazard:

```rust
netzlaufwerk,            // anwendung.rs:4867
arbeitsbaum: beruehrt_arbeitsbaum,   // anwendung.rs:4868
```

Both are correct: `netzlaufwerk` comes from `volumes::liegt_auf_netzlaufwerk` (`:4852`) and
`beruehrt_arbeitsbaum` from `arbeitsbaum::beruehrt_einen_arbeitsbaum` (`:4857`), and both
functions answer the trigger, so no inversion is needed and none is written. Swapping the two
field names, or reintroducing an inversion, compiles and leaves all 24 probes named above green,
because none of them sees this function.

## Why the site is testable, contrary to the usual objection in this crate

`krk-ui` has no library target, so a file under `crates/krk-ui/tests/` would reach nothing. That
argument does not apply here: the probe belongs in a `#[cfg(test)]` module beside the code, which
is what this file already does twice (`anwendung.rs:7090` and `:7278`, 13 probes between them).
And `loeschtexte` needs neither an instance nor the main thread:

```rust
fn loeschtexte(
    textform: Loeschtexte,
    auswahl: &Auswahl,
    quellordner: &Path,
    aufgeloester_ordner: Option<PathBuf>,
) -> (String, String, bool)
```

All four inputs are values the probe can build. The three functions it calls are free functions:
`pfade::benutzerverzeichnis`, `volumes::liegt_auf_netzlaufwerk` (already exercised from probes in
its own file) and `umfang::zaehlen`.

## Direction

Two probes, both cheap, and the first is the one that matters.

1. **A local target does not get announced as a network volume.** Call `loeschtexte` with
   `Loeschtexte::AusDenWarngruenden` over a `Pruefordner` path under the user directory with three
   entries, and assert that the question does **not** contain "von einem Netzlaufwerk" and that
   `laut` is `false`. Swap the two field names and this goes red; that is exactly the swap
   `260817-1623` describes. A counter-probe belongs beside it, otherwise a `loeschtexte` that
   never warns would be green as well: one target that must be loud, and the cheapest is the user
   directory itself, which yields `Warngrund::ImBenutzerordner` from paths alone.
2. **`f8` keeps its own texts until bundle D.** `Loeschtexte::EndgueltigBisBuendelD` returns
   `laut == true` and the wording of `operationen::loeschfrage`, and no warning reason. One
   assertion, and it goes red the day someone folds the two branches together — the deviation the
   commit message of `792995a` names as the reason the enumeration exists.

Both probes go in the `#[cfg(test)]` module of `anwendung.rs`, beside the existing ones.
