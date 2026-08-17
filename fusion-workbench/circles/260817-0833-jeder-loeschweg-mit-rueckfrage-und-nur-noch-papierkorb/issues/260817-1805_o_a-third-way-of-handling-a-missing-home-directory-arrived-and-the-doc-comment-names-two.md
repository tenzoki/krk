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
