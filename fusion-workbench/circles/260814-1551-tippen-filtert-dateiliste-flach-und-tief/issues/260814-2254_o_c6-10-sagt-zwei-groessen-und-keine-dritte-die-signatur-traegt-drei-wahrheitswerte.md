# C6.10 sagt „zwei Größen und keine dritte", die Signatur der Regel trägt drei Wahrheitswerte

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C6.10 und C1.18/C1.20; `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang C, Schritt C1; `crates/krk-ui/src/kommandos/rueckschritt.rs`

---

Der Spec und der Plan zählen dieselbe Regel verschieden, und beide Zahlen stehen wörtlich da.

- **C6.10:** „Die Fallunterscheidung hängt an **zwei Größen und an keiner dritten**: ob ein Filtertext steht, und ob der Anschlag aus einer Tastenwiederholung stammt, die bei stehendem Filtertext begann."
- **Plan, Schritt C1:** „Der Modulkopf schreibt **die drei Größen** aus", und die Signatur, die derselbe Schritt vorschreibt, lautet `rueckschritt(filtertext_steht: bool, wiederholung: bool, merker: bool)`.

**Ein Widerspruch ist es nicht, aber es liest sich als einer.** Die zweite Größe des Spec — „stammt aus einer Wiederholung, die bei stehendem Filtertext begann" — ist aus einem einzelnen Tastenereignis nicht abzulesen: AppKit meldet an `isARepeat` allein, **dass** es eine Wiederholung ist, und nicht, **wobei** sie anfing. Sie zerfällt an der Schnittstelle deshalb in zwei Wahrheitswerte, den Anschlagbefund und den mitgeführten Merker. Zwei Größen der Sache nach, drei Wahrheitswerte in der Signatur.

**Warum das jemanden kosten kann.** Wer C6.10 wörtlich abnimmt, findet drei Parameter, wo „zwei und keine dritte" zugesagt ist, und meldet eine Abweichung, die keine ist. Wer umgekehrt C6.10 wörtlich **baut**, schreibt eine Regel über zwei Wahrheitswerte — und verliert damit genau die Unterscheidung, die C1.20 von C1.18 trennt: ein gehaltener Rückschritt ohne je stehenden Filtertext hörte nach dem ersten Anschlag auf zu räumen. Der Spec nennt diese Lücke im Absatz über dem dritten Bild ausdrücklich als den zweiten Fehler, den das Zeichnen gefunden hat, und C1.20 hält sie fest. Sie käme über den Wortlaut von C6.10 zurück.

**In C1 abgefangen, aber nur dort.** Der Modulkopf von `crates/krk-ui/src/kommandos/rueckschritt.rs` schreibt den Zusammenhang aus: zwei Größen nach C6.10, drei Wahrheitswerte in der Signatur, und warum die zweite nicht in einen Wert passt. Der Spec selbst trägt den Satz nicht.

**Vorschlag.** C6.10 um einen Halbsatz ergänzen, etwa: „…, und die zweite Größe steht in der Umsetzung als zwei Wahrheitswerte da, weil ein einzelnes Tastenereignis nur den Wiederholungsbefund trägt und nicht seinen Anfang." Das ist eine Änderung am Spec und keine am Code; sie gehört dem Nutzer beziehungsweise dem shaper, nicht diesem Schritt.

**Kontext.** Aufgefallen beim Umsetzen von Schritt C1, beim Schreiben des Modulkopfes gegen C6.10. Aus dieser Directive entstanden, deshalb im Circle und nicht im gemeinsamen Speicher.
