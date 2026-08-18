A third way of handling a missing home directory arrived, and the doc comment names two

---
`crates/krk-core/src/ablage/pfade.rs:189-193` describes `benutzerverzeichnis()` as "Die eine Stelle
im Kern, die danach fragt. **Zwei Aufrufer** haengen daran und gehen mit einem fehlenden
Benutzerverzeichnis **verschieden** um", then names two. `792995a` added a third handling, and it is
the safety-relevant one: `Anwendungsdelegierter::loeschtexte` (`appkit/anwendung.rs:4849`) treats
`None` as "the question is open" and lets it become `Warngrund::Unentscheidbar`, so the confirmation
goes loud.

---

**Severity:** Low. No behaviour is wrong; the new handling is the right one and its own doc comment
(`anwendung.rs:4807-4816`) explains it at length. The defect is that the enumeration exists in order
to answer "what does a missing home directory mean here", it enumerates the divergent handlings, and
the divergent handling this bundle added is not in it.
**Found by:** coderev, review `reviews/260817-1759-coderev-bundle-c-the-loud-confirmation.md`
**Affected:** `crates/krk-core/src/ablage/pfade.rs:189-193`
**Tree state:** `792995a`
**Domain:** code
**Cross-references:**
`shared/issues/260816-2307_o_der-doc-kommentar-von-ablage-pfad-nennt-vier-dateien-die-aufzaehlung-fuehrt-sechs.md`
(a different stale count in the same module),
`shared/issues/260815-1448_o_die-neun-berichtigten-zahlen-stehen-weiter-unverankert-und-die-benannte-ursache-traegt-keinen-datensatz.md`

## The callers, counted

`grep -rn "benutzerverzeichnis()" crates/`, dropping the definition itself and the probe files:

| caller | what a `None` means there |
|---|---|
| `ablage/pfade.rs:253` `Ablageort::im_benutzerverzeichnis` | fails — named |
| `ablage/sitzung.rs:421` | falls back to `/` — named |
| `krk-ui/src/belegungsausgabe.rs:389` | passed on as `Option` to `meldung_mit` — **not named** |
| `krk-ui/src/belegungsausgabe.rs:441` | early return — **not named** |
| `krk-ui/src/appkit/anwendung.rs:7004` | falls back to `/` (same as `sitzung.rs`) | 
| `krk-ui/src/appkit/anwendung.rs:4849` `loeschtexte` | undecidable, and undecidable is loud — **not named, and added by `792995a`** |

Two readings of the sentence are possible and it settles neither. Read as scoped to the core it is
accurate: inside `krk-core` there are exactly the two named callers. Read as a statement about the
tree it is false, and it has been false since round 3 brought `belegungsausgabe.rs`. That ambiguity
is the same shape as `shared/issues/260815-1448_o_…`: the reach of the count and the reach of the
claim do not stand next to each other.

## Why this one is worth the edit rather than the usual count correction

Because of what the third handling is. `sitzung.rs` and `anwendung.rs:7004` substitute `/` so that a
file pane always shows a folder; `loeschtexte` must **not** do that, and its doc comment says why in
so many words: "ein `/` an dieser Stelle machte aus 'KRK kennt den Benutzerordner nicht' die Aussage
'der Ordner liegt darin'". That is a reasoning the next caller of `benutzerverzeichnis()` needs
before it picks a fallback, and the place a next caller looks is this doc comment.

## Direction

Either state the reach ("zwei Aufrufer im Kern") and leave the tree-wide callers out, or list the
three distinct handlings — fail, substitute `/`, stay undecided — and cite one caller for each. The
second is more useful, and the third entry is the one worth a clause: a fallback that invents a path
turns "not known" into an assertion, and the delete path cannot afford that.

---
Reconciliation 260817-1833 (reconciler, tree state `e313841`): **open, and holds without
re-measuring the code.** This record was filed against `792995a`, and the only commit since is
`e313841`, which touches nothing under `crates/` or `resources/` — it adds this Circle's Bundle C
review and its nine records and nothing else (`git show --stat e313841`). The cited lines are
therefore the lines the review read. `make check` at 260817-1833: exit 0, "alle vier gruen".

---
Resolved 260818 (coder, tree state `ae665e5`): the enumeration lists handlings instead of counting
callers, and the third handling is in it.

`crates/krk-core/src/ablage/pfade.rs:187-209`. The sentence "Zwei Aufrufer haengen daran" is gone.
The doc comment now opens with what a `None` means — a statement about KRK's knowledge, not about
the folder — and lists the three ways the tree handles it, one caller each:

- **Scheitern**: `Ablageort::im_benutzerverzeichnis` returns an error.
- **Auf `/` ausweichen**: `super::sitzung::standardordner`, because a file pane has to show
  something.
- **Unentschieden bleiben**: `Anwendungsdelegierter::loeschtexte` in `krk-ui` passes the `None` on
  as an open question, and the confirmation before emptying into the trash goes loud on it.

A closing paragraph carries the reasoning this record calls the load-bearing part: a `/` fallback
invents a path and turns "KRK does not know the home folder" into "the folder lies inside it", which
is wrong wherever a warning, a permission or a delete command hangs on the value. It ends by saying
outright that no caller count stands there, and why: the count ages with every new caller, the rule
above it does not.

Checked with `grep -rn "benutzerverzeichnis()" crates/`. Beyond the definition it returns six
production call sites — `pfade.rs:269`, `ablage/sitzung.rs:421`, `belegungsausgabe.rs:389` and
`:441`, `appkit/anwendung.rs:4790` (in `loeschtexte`) and `:6935` (the free wrapper that substitutes
`/`) — plus two inside `#[test]` bodies (`volumes.rs:483`, `papierkorb.rs:213`), which panic and are
not a handling of the production question. Every one of the six falls into one of the three listed
kinds, so the enumeration is complete without naming a number. `make check` exit 0.
