The 25 lives in four places, and the compile-time assertion binds only one pair

---
`crates/krk-ui/src/kommandos/loeschwarnung.rs:499-502` carries
`const _: () = assert!(SCHWELLE == 25, "die Wortlaute des sechsten Ausloesers nennen die 25
ausgeschrieben")`. It halts the build when `SCHWELLE` moves, which is what its own doc comment
claims. It does not bind the two spelled-out wordings **to** `SCHWELLE`: it binds `SCHWELLE` to a
second literal `25`, so a change that moves the constant and the assertion together leaves the two
wordings and the probe that pins them silently at 25.

---

**Severity:** Low. The tripwire works in the direction that matters most today — nobody can move
`SCHWELLE` without the build stopping in this file — and the wordings are pinned by the probe
`jeder_grund_traegt_seinen_wortlaut` (`:1322-1349`, wording literals at `:1338` and `:1342`). The defect is that the plan's own phrasing
promises more than the construct delivers, and a two-sided form is available in this project's
idiom and costs nothing.
**Found by:** coderev, review `reviews/260817-1759-coderev-bundle-c-the-loud-confirmation.md`
**Affected:** `crates/krk-ui/src/kommandos/loeschwarnung.rs:499-502` (the assertion), `:533-534`
(the two wordings), `:695-697` (the discarded payload), `:1338` and `:1342` (the probe's literals);
`crates/krk-core/src/verzeichnis/umfang.rs:164`, `:172`, `:185-188`
**Tree state:** `792995a`
**Domain:** code
**Cross-references:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, execution note
of step 10 ("ein `const _: () = assert!(SCHWELLE == 25, …)` hält die beiden ausgeschriebenen
Wortlaute beim Übersetzen daran"); `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`
C3

## Where the 25 stands, and what holds it

| place | what stands there | what holds it to `SCHWELLE` |
|---|---|---|
| `krk-core/.../umfang.rs:164` | `pub const SCHWELLE: u32 = 25;` | it is the source |
| `krk-core/.../umfang.rs:172` | `const DECKEL: u32 = SCHWELLE + 1;` | an expression, plus the probe `der_deckel_haengt_an_der_schwelle` |
| `loeschwarnung.rs:533-534` | `"mit 25 Einträgen"`, `"mit mehr als 25 Einträgen"` | **a second literal `25` inside `assert!`**, plus a probe that also holds literal strings |
| `loeschwarnung.rs:695-697` | `Umfang::MehrAls(_)` → `MehrAlsDieSchwelle` | nothing reads the payload |

Two consequences.

**First: the assertion is one-sided.** Raise `SCHWELLE` to 30 and the build stops — good. Raise it
to 30 and update the assertion to `SCHWELLE == 30` in the same edit, which is the natural response
to the compiler pointing at that line, and the two wordings still say 25, the probe still expects
25, and `make check` is green over a confirmation that names the wrong number. The construct converts
"the constant moved" into a compiler message; it does not tie the strings to the value. The
precedent its own doc cites is stronger in exactly this respect:
`crates/krk-ui/src/appkit/editor.rs` asserts `STAPELBUDGET == EDITORGRENZE`, two **symbols**, which
cannot drift that way.

**Second: `Umfang::MehrAls(_)` discards a payload the wording depends on.**
`Umfangsgrund::MehrAlsDieSchwelle` renders as "mit mehr als 25 Einträgen", and the number is right
only because `zaehlen` caps at `SCHWELLE + 1` and therefore only ever produces `MehrAls(SCHWELLE)`.
`Umfang` is public in `krk-core` and `MehrAls(u32)` is publicly constructible, so a `MehrAls(10)`
would render "mehr als 25". Nothing binds the payload; the doc comment of `Umfang::MehrAls`
("die Zahl ist [`SCHWELLE`]") is the whole guarantee, and the probes in `tests/umfang.rs` assert
`MehrAls(SCHWELLE)` for the real cases only.

## Direction

One probe closes both halves and needs no new type:

```
assert!(Warngrund::Umfang(Umfangsgrund::GenauDieSchwelle).wortlaut()
            .contains(&SCHWELLE.to_string()));
assert!(Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle).wortlaut()
            .contains(&SCHWELLE.to_string()));
```

That binds the strings **to the constant** rather than to a second literal, and it goes red on the
edit described above. It sits beside `jeder_grund_traegt_seinen_wortlaut` and does not replace it:
that probe holds the exact wording, this one holds the number in it. Keep the `const _: () = assert!`
as well — it fires at compile time and points a person at the right file, which a probe cannot do.

For the payload, the cheap form is to read it: `Umfang::MehrAls(schwelle) if schwelle == SCHWELLE`,
with the mismatching case falling to `Unentscheidbar` rather than to a wording it cannot support.
Whether that branch is worth its line is a judgement, and stating the alternative here is enough:
today the case is unreachable by construction of `zaehlen`, and the construction is the guarantee.

---
Reconciliation 260817-1833 (reconciler, tree state `e313841`): **open, and holds without
re-measuring the code.** This record was filed against `792995a`, and the only commit since is
`e313841`, which touches nothing under `crates/` or `resources/` — it adds this Circle's Bundle C
review and its nine records and nothing else (`git show --stat e313841`). The cited lines are
therefore the lines the review read. `make check` at 260817-1833: exit 0, "alle vier gruen".

---
Resolved: Beide Haelften geschlossen, und die bestehende Uebersetzungszusicherung
ist erweitert statt eine zweite daneben zu setzen.

**Erste Haelfte, die einseitige Zusicherung.** `loeschwarnung.rs` traegt weiter
genau ein `const _: () = assert!`, und es liest jetzt die beiden Wortlaute
selbst:

```rust
const _: () = assert!(
    nennt_die_zahl(Warngrund::Umfang(Umfangsgrund::GenauDieSchwelle).wortlaut(), SCHWELLE)
        && nennt_die_zahl(Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle).wortlaut(), SCHWELLE),
    "…"
);
```

`Warngrund::wortlaut` ist dafuer `const fn` geworden (zur Laufzeit unveraendert),
und `nennt_die_zahl` ist eine `const fn` daneben, die die Dezimalschreibung von
`SCHWELLE` in einem `&str` sucht — `str::contains` ist nicht `const`. **Das
zweite Zahlwort im `assert!` ist damit ganz weg**, es gibt kein Literal mehr,
das jemand mitziehen koennte. Die verbleibende Blindheit steht am Doc-Kommentar
von `nennt_die_zahl`: sie sucht eine Ziffernfolge und kein Wort, ein Wortlaut
„mit 250 Eintraegen" kaeme bei einer `SCHWELLE` von 25 durch.

Nachweis in beide Richtungen, je mit `cargo build -p krk-ui`:

| eingebauter Fehler | Ergebnis |
|---|---|
| `SCHWELLE` auf 30, Wortlaute unveraendert | Bau bricht ab, `E0080`, Meldung der Zusicherung |
| `SCHWELLE` bei 25, ein Wortlaut auf „mit 30 Einträgen" | Bau bricht ab, dieselbe Meldung |

Die alte Form haette den ersten Fall gefangen und waere durch die naheliegende
Antwort darauf — `SCHWELLE == 30` in derselben Zeile nachziehen — sofort wieder
gruen geworden. Die neue hat diese Zeile nicht mehr.

**Zweite Haelfte, der weggeworfene Wert an `Umfang::MehrAls(_)`.**
`warngruende` liest ihn jetzt:

```rust
Umfang::MehrAls(gedeckelt) if gedeckelt >= SCHWELLE => MehrAlsDieSchwelle,
Umfang::MehrAls(_) => Unentscheidbar,
```

„Mehr als `n`" traegt den Wortlaut „mit mehr als 25 Eintraegen" nur, wenn `n`
die Schwelle erreicht; darunter sagt der Wert ueber die Schwelle weder ja noch
nein, und das ist derselbe Fall wie eine ausgefallene Zaehlung. Der Datensatz
nennt die Fallunterscheidung eine Ermessensfrage; gebaut ist sie, weil sie eine
Zeile kostet und die einzige Stelle schliesst, an der die Rueckfrage eine Zahl
behaupten koennte, die niemand gezaehlt hat.

Gemessen wird sie an der **bestehenden** Tafel `der_umfang_loest_ab_der_schwelle_aus`,
die von sechs auf acht Zeilen gewachsen ist: `MehrAls(26)` traegt den Wortlaut,
`MehrAls(10)` fuehrt auf `Unentscheidbar`. Nachweis: den Zweig auf
`Umfang::MehrAls(_)` zurueckgestellt, und die Probe meldet
`MehrAls(10) left: [Umfang(MehrAlsDieSchwelle)] right: [Unentscheidbar]`.

Der vom Datensatz vorgeschlagene eigene Probenrumpf mit
`wortlaut().contains(&SCHWELLE.to_string())` ist **nicht** gebaut: die
Uebersetzungszusicherung prueft dieselbe Sache frueher und zeigt dabei auf die
Datei, was eine Probe nicht kann.
