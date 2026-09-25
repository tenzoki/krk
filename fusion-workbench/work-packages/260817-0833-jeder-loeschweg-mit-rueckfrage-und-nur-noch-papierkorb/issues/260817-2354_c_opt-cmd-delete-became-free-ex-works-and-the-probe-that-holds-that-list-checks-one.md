Opt+Cmd+Entf became free ex works and the probe that holds that list checks one

---
`die_ab_werk_freien_kombinationen_kommen_nicht_vor` (`crates/krk-core/tests/belegung.rs:277`)
asserts that a combination a reader would expect to be bound appears in no key list. Since
`82707ef` there are **two** such combinations — Umschalt+Entf and Opt+Cmd+Entf — and the
probe asserts only the first. The head of `resources/default-keymap.toml` names both at
`:62`-`:67`.

---

**Severity:** Low. Nothing is broken today; `opt+cmd+delete` really is unbound, and the
keymap head says so. What is missing is the assertion that keeps it that way. The head
gives a reason for holding it free that outlives this round: in the Finder the combination
means "delete immediately", and KRK no longer has that meaning
(`shared/decisions/260817-0536_*_bekommt-f8-den-papierkorb-nachdem-das-endgueltige-loeschen-weggefallen-ist.md`).
A later round can hand it out without any probe objecting.

**Found by:** coder, step 15 of `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`
**Affected:** `crates/krk-core/tests/belegung.rs:277`-`:335`
**Tree state:** working tree after step 15, on `8f556ed`
**Domain:** code

## Why step 15 did not fix it

Step 15 is scoped to prose — comments, module heads, doc comments. Adding a second
combination to the assertion changes the probe body, and with two entries the loop the
comment describes comes back (Clippy rejects `single_element_loop` for one). That is a code
change and belongs to a step that is allowed to make one.

The comment block above the assertion was corrected in step 15 and now states the gap in
place: it says that the keymap head carries two ex-works free combinations and this probe
one, and that whoever adds the second brings the loop back with it.

## Direction

Restore the loop over a two-element list and assert both combinations. The comment block
above already carries the reasoning for each; the paragraph naming the gap comes out with
the fix.

---
Resolved: `die_ab_werk_freien_kombinationen_kommen_nicht_vor`
(`crates/krk-core/tests/belegung.rs`) traegt wieder eine Liste und laeuft ueber
zwei Eintraege, `shift+delete` und `opt+cmd+delete`. Die Pruefung und der
Kopfkommentar von `resources/default-keymap.toml` (`:62`-`:67`) nennen damit
dieselben Kombinationen.

Der Kommentarblock ueber der Zusicherung ist nachgezogen: der Absatz, der die
Luecke benannte, ist heraus, der Absatz zur fehlenden Schleife sagt jetzt,
warum sie zwischen dem 260811 und dem 260818 weg war und seit wann sie wieder
dasteht, und Opt+Cmd+Entf traegt seinen Grund fuers Freibleiben an Ort und
Stelle statt eines Verweises auf die Luecke. Clippys `single_element_loop`
greift bei zwei Eintraegen nicht mehr; `make check` laeuft durch.

Nachweis, dass der zweite Eintrag wirklich geprueft wird: die Liste
versuchsweise auf `["shift+delete", "f8"]` gestellt, und die Probe meldet
`f8 ist ab Werk belegt` (Fehlschlag), weil `f8` in der Auslieferungsbelegung
steht. Die Zusicherung greift also auf beide Eintraege und nicht nur auf den
ersten. Der naheliegendere Nachweis waere gewesen, `opt+cmd+delete` wieder in
eine Tastenliste zu schreiben; dafuer haette `resources/default-keymap.toml`
angefasst werden muessen, und die Datei gehoerte nicht zu dieser Aufgabe.
