`ist_lokal` returns the inverse of the `Loeschziel` field it fills, and nothing but prose stops the swap

---

Step 9 of this round's plan prescribes `volumes::ist_lokal(pfad) -> Loeschzielbefund`, where `Ja`
means "local" and therefore *harmless*. Step 10 gives `Loeschziel` a field `netzlaufwerk` where
`Ja` means "is a network volume" and therefore *warns*. The two run in opposite directions, both
sides carry the same type, and the compiler cannot see the difference. There is no three-valued
negation on `Loeschzielbefund` either, so the caller in step 11 has to write the inversion by hand.

---

**Severity:** Medium, rising to High the moment step 11 is written. Nothing is wrong on today's
tree: `ist_lokal` has no caller yet, and the deliberate `expect(dead_code)` on it makes that
visible. The defect is the trap laid for the next step. A caller that writes
`netzlaufwerk: volumes::ist_lokal(&ordner)` compiles, passes every existing probe, and inverts
trigger 3 of C3: a local disk is announced as a network volume in the loud confirmation, and an
actual network volume goes quiet. Worse, the mistake hides behind a promise that still looks kept —
`Unentschieden` is a fixed point of the inversion, so "undecided counts as loud" continues to hold
while the *stated reason* is wrong for the two decided cases.
**Found by:** coder, while implementing step 9 (task T8)
**Affected:** `crates/krk-ui/src/appkit/volumes.rs` (`ist_lokal`),
`crates/krk-core/src/verzeichnis/loeschzielbefund.rs` (no `nicht`/`umgekehrt`),
plan `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md` steps 9, 10 and 11
**Tree state:** `3fcd375` plus the uncommitted step 9
**Domain:** code

## What the tree says

Spec C3, trigger 3, reads: "der Datenträger des Ordners ist kein lokaler" — the trigger is the
*negation* of the question `ist_lokal` asks. The two polarities of `Loeschzielbefund` are written
out in its module header, and this function lands on the second one (`Ja` is the harmless answer,
`Unentschieden` groups with `Nein`), exactly like `papierkorb::fuehrt_einen_papierkorb`. The field
that consumes it lands on the first (`Ja` warns, `Unentschieden` groups with `Ja`).

```text
  ist_lokal        Ja = local        harmless      polarity 2
  netzlaufwerk     Ja = network      warns         polarity 1
                   └── one type, two directions, no compiler check
```

`Loeschzielbefund` offers `ist_warnwuerdig` and `oder`. Neither inverts, so step 11 must spell out
a three-arm `match` at the call site — a second place where the polarity has to be got right, in a
file that carries none of the reasoning.

## What step 9 did about it, and what it could not do

Three guards are in place, and all three are local to `volumes.rs`:

1. The module header states the polarity, the inversion the caller owes, and why the swap is
   invisible.
2. The doc comment on `ist_lokal` names each of the three outcomes and which of them warn.
3. A counting probe, `hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt`, asserts that this file
   never calls `ist_warnwuerdig` — the first of the two directions asked for in
   `260817-1419_o_die-einzige-sicherung-gegen-den-polaritaetsfehler-ist-prosa-und-ist-warnwuerdig-hat-keinen-aufrufer.md`.

None of them reaches the call site in `appkit/anwendung.rs`, and none of them turns the swap into a
compile error. Renaming the function was not step 9's to do: the plan prescribes `ist_lokal`, and
silently reversing a prescribed name would have hidden the question rather than raised it.

## Direction

Three ways, and the first is the cheapest by a wide margin.

1. **Name the function after the trigger and return the trigger's answer.**
   `liegt_auf_netzlaufwerk(pfad) -> Loeschzielbefund`, with `Ja` for a non-local volume. The
   inversion then happens once, inside the function, beside the header that explains it; the field
   name and the function name agree; `ist_warnwuerdig` becomes the *correct* question for this
   value, so the habit that the older record fears stops being a hazard here. Cost: one rename
   inside one file plus the two probe names, before any caller exists. It contradicts the plan's
   prescribed name, which is why it is filed rather than done.
2. **Add a three-valued inversion to the type.** `Loeschzielbefund::umgekehrt()` with a written-out
   three-row table, `Unentschieden` mapping to itself. Keeps the plan's name and gives the caller
   one named operation instead of an ad-hoc `match`. It does not make the swap uncompilable: a
   caller can still omit the call.
3. **Two types for two questions**, the second direction of
   `260817-1419_o_die-einzige-sicherung-gegen-den-polaritaetsfehler-…`. This is the only option that
   makes the mistake a compile error, and it settles both records at once. Cost is the highest: a
   second type plus a conversion at every checkpoint of bundle C.

Whichever is chosen, it should land **before** step 11, since step 11 is the first caller and the
inversion is its work.

---

Resolved: 260817-1722 (coder, T9) — **way 1, chosen by the user on 260817-1640.** The function
is `volumes::liegt_auf_netzlaufwerk(pfad) -> Loeschzielbefund` and returns the *trigger's*
answer: `Ja` for a non-local volume, `Nein` for a local one, `Unentschieden` unchanged. Name,
return value and the field that consumes it (`Loeschziel.netzlaufwerk`, polarity 1) now point
the same way, and the one inversion happens inside the function, next to the module header that
explains it:

```rust
// **Die eine Umkehrung.** `NSURLVolumeIsLocalKey` antwortet „lokal", der
// Ausloeser aus C3 fragt „kein lokaler". …
if wert.boolValue() {
    Loeschzielbefund::Nein
} else {
    Loeschzielbefund::Ja
}
```

The queried resource value is still `NSURLVolumeIsLocalKey`, so the availability section of the
module header is untouched. What changed: the name, that one inversion, the module header's
polarity section, and four probes — `das_benutzerverzeichnis_liegt_nicht_auf_einem_netzlaufwerk`
now expects `Nein` and `der Einhaengepunkt der /home-Automatik` expects `Ja`. Nothing outside
`volumes.rs` referenced the old name, so no other file moved with it.

**The three rejected ways, as the user rejected them.** `Loeschzielbefund::umgekehrt()` — keeps
the plan's name and gives the caller one named operation, but a caller can still omit the call,
so the swap stays compilable. Two types per polarity — the only option that makes the mistake a
compile error, and the most expensive: a second type plus a conversion at every checkpoint of
bundle C. The inversion by hand at the call site — a three-arm `match` in
`appkit/anwendung.rs`, a file that carries none of the reasoning.

**What this does not fix, stated so nobody reads it as fixed.** The swap is not uncompilable; it
has lost its occasion. Two questions of opposite polarity still share one type, and the only
guard against confusing them remains prose plus counting probes. That is the second way of
`260817-1419_o_die-einzige-sicherung-gegen-den-polaritaetsfehler-…`, and it is untouched.

**One consequence for the counting probe named in this record.**
`hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt` in `volumes.rs` did lose its stated subject:
after the rename `ist_warnwuerdig` is the *correct* question for this value, so the count no
longer guards against a wrong answer. It stays, with a rewritten doc comment and a different,
unchanged promise — this module *answers* the trigger and does not *judge* it; whether the
confirmation goes loud, and which of the six reasons it names, is decided once in
`kommandos::loeschwarnung::warngruende`. Inverting the probe was rejected: an assertion that the
file *does* ask would turn a module boundary into an obligation to cross it. Dropping it was
rejected because the boundary is real and nothing else holds it.

Verification: `make check` — exit 0.
