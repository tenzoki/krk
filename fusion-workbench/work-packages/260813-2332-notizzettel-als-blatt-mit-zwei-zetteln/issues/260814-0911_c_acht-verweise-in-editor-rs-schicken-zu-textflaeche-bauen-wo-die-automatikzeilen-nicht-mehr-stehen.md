Acht Verweise in editor.rs schicken zu textflaeche_bauen, wo die Automatikzeilen nicht mehr stehen

---

Schritt 9 hat die neun Zeilen, die die Textautomatiken abschalten, aus
`editor::textflaeche_bauen` in `appkit/textautomatik::automatiken_abschalten` gezogen. Die
Prosa in `crates/krk-ui/src/appkit/editor.rs` ist an acht Stellen nicht mitgezogen und
schickt den nächsten Leser weiterhin an die alte Stelle:

| Zeile | Aussage |
|---|---|
| 316 | „Sie baut die Flaeche mit `textflaeche_bauen`, liest jede der neun zurueck" |
| 4231 | `Einordnung::Abgeschaltet`: „`textflaeche_bauen` schaltet sie ab" |
| 4253 | „die einzelnen Zeilen in `textflaeche_bauen` sind die erste" |
| 4269 | „`textflaeche_bauen` setzt sie deshalb **nicht**" |
| 4361 | „Keine braucht eine eigene Zeile in `textflaeche_bauen`" |
| 4423 | „zwei tragen daraufhin einen Aus-Wert und je eine Zeile in `textflaeche_bauen`" |
| 4854 | „Wer eine zehnte Einstellung als `Abgeschaltet` einträgt, ohne die Zeile in `textflaeche_bauen` zu schreiben" |
| 5253, 5260 | „wäre die Zeile in `textflaeche_bauen` überflüssig" / „nimmt die Zeile in `textflaeche_bauen` nichts fort" |

---

**Schwere:** mittel. Kein Bau, kein Verhalten — aber eine Anweisung, deren Befolgung genau
den Zustand wiederherstellte, den diese Runde beseitigt hat.

**Die Stelle bei 4854 ist die teure.** Sie ist eine Handlungsanweisung an den nächsten
Bauer: wer eine zehnte Einstellung einträgt, soll „die Zeile in `textflaeche_bauen`"
schreiben. Wer das täte, gäbe die Einstellung dem Editor und **nicht** dem Notizzettel —
zwei Wahrheiten darüber, was „abgeschaltet" heißt, also die Lage, die der Entscheid
`decisions/260814-0656_a_wird-die-abschaltung-der-textautomatiken-bauanhaltend.md` gerade
als den stillen Schaden benennt. Die Probe
`die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus` fiele daraufhin an der Fläche
des Zettels, aber erst nach der falschen Arbeit.

**Nicht betroffen** sind die Stellen, an denen `textflaeche_bauen` als **Erzeuger der
Fläche** gemeint ist (`:201`, `:343`, `:398`, `:4885`, `:4943`, `:5016`, `:5168`): dort
stimmt der Verweis weiter, denn die Fläche kommt nach wie vor von dort.

**Was zu tun ist.** Die acht Stellen auf `textautomatik::automatiken_abschalten` ziehen.
Wo der Satz beide Seiten meint — „`textflaeche_bauen` schaltet sie ab" —, ist die
genauere Fassung „`automatiken_abschalten`, das `textflaeche_bauen` und die Fläche des
Zettels rufen".

**Kontext**

- Gefunden bei der Durchsicht von Turn 1, `reviews/260814-0908-coderev-turn-1-notizzettel.md`.
- Zeilennummern am Stand `dd2643e` gezählt.

---
Resolved: Die acht Stellen zeigen jetzt auf
`textautomatik::automatiken_abschalten`. Nachgezogen in
`crates/krk-ui/src/appkit/editor.rs`, in der Reihenfolge des Befundes:

| vorher | nachher |
|---|---|
| 316 | die neun Zeilen stehen in `super::textautomatik::automatiken_abschalten`, das `textflaeche_bauen` und die Fläche des Zettels rufen; die Probe baut beide Flächen |
| 4231 | „`automatiken_abschalten` schaltet sie ab, das `textflaeche_bauen` und die Fläche des Zettels rufen" |
| 4253 | „die einzelnen Zeilen in `automatiken_abschalten` sind die erste" |
| 4269 | „`automatiken_abschalten` setzt sie deshalb **nicht**" |
| 4361 | „Keine braucht eine eigene Zeile in `textautomatik::automatiken_abschalten`" |
| 4423 | „je eine Zeile in `textautomatik::automatiken_abschalten`" |
| 4854 | die Handlungsanweisung nennt `automatiken_abschalten` und sagt dazu, warum die Zeile nicht in `textflaeche_bauen` gehört |
| 5253, 5260 | beide Probenmeldungen nennen `automatiken_abschalten` |

**Die teure Stelle bei 4854 trägt jetzt ihre Begründung mit.** Der bloße Austausch des
Namens hätte den nächsten Bauer richtig geschickt und ihm nicht gesagt, warum; die Stelle
schreibt deshalb aus, dass eine Zeile in `textflaeche_bauen` allein dem Editor gehörte und
der Zettel ohne sie dastünde — die zwei Wahrheiten darüber, was „abgeschaltet" heißt, die
der Entscheid `decisions/260814-0656_*_wird-die-abschaltung-der-textautomatiken-bauanhaltend.md`
als den stillen Schaden benennt.

**Die sieben Stellen, an denen `textflaeche_bauen` als Erzeuger der Fläche gemeint ist,
sind unangetastet geblieben** (`:201`, `:343`, `:398`, `:4885`, `:4943`, `:5016`, `:5168`),
wie der Datensatz es festhält: dort stimmt der Verweis weiter.

Verhalten ändert sich nicht; die Änderung ist Prosa. `make check` am 260814 gefahren,
Rückgabewert 0, „alle vier gruen" — die Doc-Verweise in eckigen Klammern lösen auf, also
hält auch `cargo doc` sie.

---

**Abgleich 260814-1247: nachgezählt und bestätigt.** Die acht Verweise zeigen jetzt auf
`textautomatik::automatiken_abschalten`; gesehen an `crates/krk-ui/src/appkit/editor.rs:316`,
`:4233`, `:4257`, `:4273`, `:4365`, `:4427`, `:4829` und `:4859`. Die sieben Stellen, an denen
`textflaeche_bauen` als Erzeuger der Fläche gemeint ist, stehen unverändert. `make check`
Rückgabewert 0.
