Die Zählproben in `krk-ui` sagen „im Baum" und lesen nur eine Kiste

---

`krk_ui::quellbaum::quelldateien` liest `crates/krk-ui/src` und sonst nichts
(`crates/krk-ui/src/quellbaum.rs:51`). Jede Probe, die darauf aufsetzt, spricht in ihrem
Doc-Kommentar und in ihrer Fehlermeldung trotzdem vom **Baum**:

- `die_zulaessigkeitsregel_ist_genau_einmal_erklaert` — „`fn zulaessig` kommt im Baum genau
  einmal vor" (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:145-172`)
- `die_frage_nach_dem_ersthelfer_steht_an_genau_einer_stelle` — „im ganzen Baum"
  (`crates/krk-ui/src/appkit/ereignisse.rs:682-731`)
- `keine_ansicht_ueberschreibt_keydown` (`ereignisse.rs:733-756`)
- `der_eigene_buendelort_wird_an_genau_einer_stelle_bestimmt`
  (`crates/krk-ui/src/appkit/weitereinstanz.rs:129-142`)
- die zwei Zählproben in `crates/krk-ui/src/appkit/menue.rs:1061-1150`

Der Kopf von `quellbaum.rs` sagt es selbst richtig („Der Quellbaum **dieser Kiste**"), die
Proben darüber nicht. Zum Vergleich: `crates/krk-core/tests/gemeinsam/mod.rs:264` liest alle
Kisten unter `crates/`, und die Proben in `crates/krk-core/tests/baum.rs` dürfen deshalb vom
Baum sprechen.

**Was das kostet.** Für `isKindOfClass` und `keyDown:` ist die Einschränkung harmlos: `krk-core`
und `krk-bench` dürfen `objc2` nicht kennen, und `xtask/src/release.rs` prüft die Grenze. Für
`fn zulaessig(` ist sie es nicht — eine zweite Fassung der Zulässigkeitsregel in `krk-core`
bliebe unsichtbar, und C2.16 sagt „an genau einer Stelle" ohne Kistengrenze zu.

---

**Schwere:** mittel. Kein Fehlverhalten heute; eine Zusage, die weiter reicht als ihre
Prüfung, und ein Leser, der die Fehlermeldung wörtlich nimmt, hält den ganzen Baum für
abgedeckt.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/quellbaum.rs:38-61` und die sechs Proben darüber

**Domain:** code

## Vorschlag

Zwei Wege, und der zweite ist der billigere.

1. Die Zusage, die wirklich kistenübergreifend gilt — `fn zulaessig` genau einmal —, nach
   `crates/krk-core/tests/baum.rs` ziehen, wo der Leser alle Kisten sieht. Die übrigen bleiben,
   wo sie sind.
2. Die Doc-Kommentare und Fehlermeldungen auf „in `krk-ui`" umschreiben und im Kopf von
   `quellbaum.rs` einen Satz ergänzen, dass eine Zusage über alle Kisten nach `tests/baum.rs`
   gehört.
