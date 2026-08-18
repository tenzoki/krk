A `test`-typed commit carries an unmentioned split of a production branch

---

`441da86` is typed `test(ui): sechs behauptete Eigenschaften sind jetzt gemessen`. Besides the
probes and the extended compile-time assertion it describes, it splits one arm of `warngruende`
into two with a new guard and a new outcome. The commit message does not mention it.

---

**Severity:** Low. Traceability only — the new branch is right, it is justified in a code
comment, it is measured by a probe, and no reachable input changes behaviour. Filed because
somebody reading the log for "what changed the delete path" would skip a `test(ui)` commit, and
this Circle exists because of a delete path that changed without anybody noticing.
**Found by:** coderev, review `reviews/260818-0410-coderev-bundle-f-die-messungen-und-der-waechter.md`
**Affected:** `crates/krk-ui/src/kommandos/loeschwarnung.rs:766-780`, commit `441da86`
**Tree state:** `a4d8211`
**Domain:** code

## What changed

Before `441da86`:

```rust
Umfang::MehrAls(_) => {
    gruende.push(Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle));
}
```

After:

```rust
Umfang::MehrAls(gedeckelt) if gedeckelt >= SCHWELLE => {
    gruende.push(Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle));
}
Umfang::MehrAls(_) => gruende.push(Warngrund::Unentscheidbar),
```

One arm became two, and the second produces a different `Warngrund` than the arm it was split
out of. That is production logic in the function that decides what the delete confirmation says
and whether it is loud.

**Checked, and reachable behaviour is unchanged:** `umfang::zaehlen` caps at `SCHWELLE + 1` and
emits `MehrAls` only with the value `SCHWELLE`, so the new guard is true for every value the
counter produces. The branch is defensive, and it is defensible: `Umfang` is `pub` with a `pub`
`MehrAls(u32)` variant, so `MehrAls(10)` is constructible from outside, and mapping it to
`MehrAlsDieSchwelle` would let the question claim a number nobody counted. The code comment at
`:766-771` says exactly that.

**Checked, and it is measured:** `der_umfang_loest_ab_der_schwelle_aus` gained the three cases
`MehrAls(25)`, `MehrAls(26)` and `MehrAls(10)`. Verified by mutation: weakening the guard to
`gedeckelt >= 1` turns that probe red with `MehrAls(10) left: [Umfang(MehrAlsDieSchwelle)] right:
[Unentscheidbar]`; reverted, green again.

## What the commit message says about it

Nothing. It describes the two wiring probes, the extended `const` assertion, the `assert!`
upgrade and the two decision records. The `MehrAls` split appears in no paragraph, and the
Conventional-Commits type is `test`.

## Direction

Nothing to change in the code. The record exists so the change is findable: whoever writes the
Circle's Artifact or its closure note carries the split into the list of what this Circle did to
`warngruende`, rather than leaving it to be discovered by reading a `test`-typed diff. For future
runs: a commit that changes a match arm in a production function is not `test`, whatever else it
carries.
