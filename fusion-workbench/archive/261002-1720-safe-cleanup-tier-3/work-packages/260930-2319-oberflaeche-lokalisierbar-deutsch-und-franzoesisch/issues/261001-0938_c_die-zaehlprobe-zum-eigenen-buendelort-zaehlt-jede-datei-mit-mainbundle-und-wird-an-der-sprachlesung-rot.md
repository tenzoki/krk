Die Zählprobe zum eigenen Bündelort zählt jede Datei mit `mainBundle()` und wird an der Sprachlesung rot
---
`der_eigene_buendelort_wird_an_genau_einer_stelle_bestimmt` (`crates/krk-ui/src/appkit/weitereinstanz.rs`, Prüfmodul) hält laut Doc-Kommentar, dass es im Baum keine zweite Antwort auf die Frage nach dem **eigenen Ort** gibt, also nach `bundleURL`. Ihre Nadel ist aber `mainBundle()`, und die zählt jede Datei, die das Hauptbündel aus irgendeinem Grund anspricht. Schritt 2 des Plans `261001-0850_*_plan-oberflaeche-folgt-der-systemsprache-deutsch-franzoesisch-englisch.md` legt mit `appkit/sprache.rs` den zweiten Leser des Hauptbündels an, der `preferredLocalizations` fragt und den Ort nicht, und die Probe wird rot, obwohl ihre Aussage weiter wahr ist; der Modulkopf von `appkit/mod.rs` nennt denselben Ausdruck in Prosa und zählt mit:

```
assertion `left == right` failed: der eigene Buendelort wird an mehr als einer Stelle bestimmt
  left: ["krk-ui/src/appkit/mod.rs", "krk-ui/src/appkit/sprache.rs", "krk-ui/src/appkit/weitereinstanz.rs"]
 right: ["krk-ui/src/appkit/weitereinstanz.rs"]
```

Der Schritt nennt diese Probe nicht, und die Vorgabe des Ausführenden sagt, dass eine rote Probe, die der Schritt nicht nennt, ein Stopp ist und keine Probe ihre Erwartung ändert; die Nadel ist deshalb nicht angefasst, und `make check` bleibt am Stand dieses Schrittes rot (`cargo test -p krk-ui`, exit 101).

Abnahme: die Nadel misst die Frage, die der Doc-Kommentar stellt, also `mainBundle().bundleURL()` (als `concat!`, damit die Probe sich nicht selbst zählt), die Erwartung bleibt `["krk-ui/src/appkit/weitereinstanz.rs"]`, die Probe ist mit `appkit/sprache.rs` im Baum grün, und `make check` endet mit 0. Eine Nadel, die weiter `mainBundle()` zählt und `sprache.rs` als zweite erlaubte Datei einträgt, wäre die Aufzählung, die der Doc-Kommentar der Probe gerade vermeidet.
---
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>
Gefunden beim Lauf von `make check` nach Schritt 2 am 261001-0938; Arbeitspaket `260930-2319-oberflaeche-lokalisierbar-deutsch-und-franzoesisch`.
---
Resolved: 261001, code-implementer. Die Nadel der Probe `der_eigene_buendelort_wird_an_genau_einer_stelle_bestimmt` (`crates/krk-ui/src/appkit/weitereinstanz.rs`) ist von `mainBundle()` auf `bundleURL()` verengt (als `concat!`, damit die Probe sich nicht selbst zaehlt); die Erwartung bleibt allein `krk-ui/src/appkit/weitereinstanz.rs`. `bundleURL()` statt `mainBundle().bundleURL()`, damit auch eine Form zaehlt, die das Hauptbuendel erst in eine Variable legt. `bundlePath` und `executableURL` stehen nirgends im Baum (`grep -rn` ueber `crates` und `xtask`), die Nadelmenge bleibt deshalb bei dem einen Ausdruck; der Doc-Kommentar der Probe sagt, dass sie eine zweite Stelle ueber einen dieser Ausdruecke nicht saehe. Keine andere Datei angefasst. `make check` endet mit 0.
