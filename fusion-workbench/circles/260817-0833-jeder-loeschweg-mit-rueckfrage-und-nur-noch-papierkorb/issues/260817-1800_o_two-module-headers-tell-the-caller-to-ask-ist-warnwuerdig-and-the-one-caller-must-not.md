Two module headers tell the caller to ask `ist_warnwuerdig`, and the one caller must not

---
`crates/krk-core/src/verzeichnis/arbeitsbaum.rs:95-96` states as fact: "Der Aufrufer fragt
[`Loeschzielbefund::ist_warnwuerdig`], nicht auf `Ja` selbst." `crates/krk-ui/src/appkit/volumes.rs:248-251`
says it is "die richtige Frage" and is asked "dort, wo die Rangfolge aus C3 steht". The one caller
of both values is `loeschwarnung::warngruende`, and it deliberately does not ask it and must not:
`ist_warnwuerdig` merges `Ja` and `Unentschieden`, and those two produce **different** entries in
its list.

---

**Severity:** Medium. Nothing is wrong at today's tree. The defect is that two module headers
direct the next reader into the error a C3 acceptance criterion forbids, at exactly the spot where
the review of bundle B recorded that the only safeguard is prose. Following
`arbeitsbaum.rs:95-96` literally would collapse the two answers and make KRK name "aus einem
Git-Arbeitsbaum" for a target it could not classify — a claim with no measurement behind it.
**Found by:** coderev, review `reviews/260817-1759-coderev-bundle-c-the-loud-confirmation.md`
**Affected:** `crates/krk-core/src/verzeichnis/arbeitsbaum.rs:91-99`,
`crates/krk-ui/src/appkit/volumes.rs:70-76` and `:246-251`. The three shorter notes at
`arbeitsbaum.rs:220-221`, `:283-284` and `:333-334` are **not** affected: they state the polarity
only and name no asker.
**Tree state:** `792995a`
**Domain:** code
**Cross-references:**
`issues/260817-1419_o_die-einzige-sicherung-gegen-den-polaritaetsfehler-ist-prosa-und-ist-warnwuerdig-hat-keinen-aufrufer.md`
(its progress note of 260817-1722 establishes *why* `warngruende` does not call it; this record is
the two files that still say the opposite),
`shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md` C3

## The two statements, and what the tree does

`arbeitsbaum.rs:91-99`:

> **Auf der ersten**, und alle drei auf derselben: `Loeschzielbefund::Ja` ist der Warngrund, und
> `Loeschzielbefund::Unentschieden` gehoert zu ihm. **Der Aufrufer fragt
> `Loeschzielbefund::ist_warnwuerdig`, nicht auf `Ja` selbst.**

`volumes.rs:246-251`:

> `Loeschzielbefund::ist_warnwuerdig` ist damit fuer diesen Wert die richtige Frage. **Gestellt
> wird sie nicht hier, sondern dort, wo die Rangfolge aus C3 steht**; der Modulkopf sagt, warum.

Where the ranking stands is `warngruende` (`kommandos/loeschwarnung.rs:642-706`), and it writes
all three answers out for both fields:

```rust
match ziel.netzlaufwerk {
    Loeschzielbefund::Ja => gruende.push(Warngrund::Netzlaufwerk),
    Loeschzielbefund::Unentschieden => gruende.push(Warngrund::Unentscheidbar),
    Loeschzielbefund::Nein => {}
}
```

`ist_warnwuerdig` has no production call site anywhere in the tree. Counted with
`grep -rn "ist_warnwuerdig" crates/`: 27 hits, of which 6 are calls and all 6 sit in probes
(`krk-core/tests/arbeitsbaum.rs:100`, `:305`, `arbeitsbaum.rs:573`, and three in
`loeschzielbefund.rs`'s own probe module). Every other hit is a doc comment.

The reason is written out twice and is design, not oversight: `loeschwarnung.rs:153-158` and the
progress note of `260817-1419_o_…-polaritaetsfehler-…` both say that `Ja` and `Unentschieden`
must stay apart here because they lead to different reasons, which is C3's criterion "nennt als
Grund, dass das Ziel sich nicht einordnen ließ". The probe
`ein_unentschiedener_eingang_nennt_seinen_ausloeser_nicht_mit` (`loeschwarnung.rs:1475`) holds it,
and its own doc comment says it would go red if someone merged the three answers with
`ist_warnwuerdig`.

So the design is right and measured. Two files describe the design it replaced.

## Which of the two is worse

`arbeitsbaum.rs` states it flatly ("Der Aufrufer fragt …"), as a fact about the tree. `volumes.rs`
is milder: the value really does
sit on polarity 1, so calling `ist_warnwuerdig` the right question for it is true in the abstract;
what is false is the second half, that the question is asked where the ranking stands.

## Direction

Correct both sentences to say what holds, and say why, because the "why" is the load-bearing part:

- for a value on the first polarity `ist_warnwuerdig` **would** be sound, and
- the one consumer still writes all three answers out, because it has to name *which* reason, and
  a merged question cannot.

`arbeitsbaum.rs` is the place to carry the reasoning, since it makes the flat claim; `volumes.rs`
already carries an adjacent paragraph on its counting probe and needs only its second half fixed.
Neither change touches behaviour. Whether the value's polarity should be carried by the type at
all remains the second way of `260817-1419`, untouched.
