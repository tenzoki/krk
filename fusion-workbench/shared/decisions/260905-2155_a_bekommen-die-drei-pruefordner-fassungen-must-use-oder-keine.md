# Bekommen die drei Prüfordner-Fassungen `#[must_use]`, oder keine von ihnen?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260826-1305_*_krk-bench-traegt-ein-einziges-must-use-und-der-sitzungswaechter-ist-nicht-das-eine.md` (offen; dort steht die Frage als Einschränkung im Abhilfeteil), `260826-1302_*_eine-vierte-pruefordner-fassung-steht-in-xtask-und-die-zaehlprobe-c4-6-kann-sie-nicht-sehen.md` (offen; dieselbe Dreiergruppe, andere Frage)

---

## Frage

`Wegwerfordner` (`crates/krk-bench/src/wegwerfordner.rs`) ist ein RAII-Wächter: sein `Drop` räumt
den Ordner und den Steckbrief daneben ab. `Wegwerfordner::neu("zweck");` als nackte Anweisung
übersetzt, legt einen Namen fest, den niemand mehr hält, und räumt sofort wieder ab. Das ist
genau der Fall, den die `#[must_use]`-Regel des Projekts meint (entschieden 260811-2140, in
`CLAUDE.md` unter „Was man nicht sieht").

Gesetzt werden kann es hier nicht allein. Die Kiste führt genau eine der drei anerkannten
Prüfordner-Fassungen; die beiden Schwestern sind `krk_ui::pruefordner::Pruefordner`
(`crates/krk-ui/src/pruefordner.rs`) und der `Pruefordner` in
`crates/krk-core/tests/gemeinsam/mod.rs`. Keine der drei trägt heute `#[must_use]`. Wer es an
einer setzt, erzeugt eine vierte Abweichung zwischen drei Fassungen, die es ausdrücklich nur
dreimal geben soll — oder er setzt es an allen dreien und greift dafür in drei Kisten.

Die Frage muss vor dem nächsten `#[must_use]`-Durchgang beantwortet sein: die Behebungsrunde vom
260905 hat den `Sitzungswaechter` und die zwei `bestanden`-Urteile in `krk-bench` bekommen und den
`Wegwerfordner` genau deshalb ausgelassen.

## Optionen

1. **Alle drei bekommen es, in einem Durchgang.** Eine Zeile je Fassung, drei Kisten berührt.
   - Pro: die Regel gilt dann für den Wächter wie für jede andere reine Antwort des Baums, und die
     drei Fassungen bleiben zeichengleich, was ihre Vergleichbarkeit ausmacht.
   - Contra: ein Durchgang über drei Kisten für drei Zeilen; und es ist nicht gemessen, ob ein
     Rufer je einen dieser Werte fallen gelassen hat.
2. **Keine bekommt es, und der Verzicht steht am Modulkopf.** Der Wächter wird immer gebunden,
   weil er sonst nichts täte; das ist an der Aufrufstelle offensichtlich.
   - Pro: kein Eingriff, und die drei Fassungen bleiben gleich.
   - Contra: „offensichtlich" ist genau die Begründung, die die Regel von 260811-2140 verworfen
     hat. Ein `Wegwerfordner::neu("x");` ohne Bindung sieht aus wie ein Anlegen.
3. **Nur `krk-bench` bekommt es, mit einer Notiz an allen dreien.** Die Kiste, deren Wächter
   heute den teuersten Nebeneffekt hat, geht voran.
   - Pro: kleinster Eingriff.
   - Contra: erzeugt genau die vierte Abweichung, die `260826-1305` benennt; die nächste Durchsicht
     liest sie als Defekt.

## Randbedingungen

- Die Zahl der anerkannten Fassungen bleibt drei; die Zählprobe
  `genau_drei_pruefordner_fassungen_stehen_im_baum` (`crates/krk-core/tests/baum.rs`) hält sie.
- Ob die vierte Fassung in `xtask/src/release.rs` (`Wegwerfwurzel`) anerkannt wird oder fällt, ist
  eine eigene offene Frage und hier nicht mitentschieden.
- Kein Rufer im Baum lässt heute einen dieser Werte fallen; der Befund ist die fehlende
  Absicherung, nicht ein eingetretener Fehler.

## Empfehlung

Möglichkeit 1, zusammen mit dem nächsten Durchgang, der ohnehin alle drei Kisten anfasst. Sie ist
die einzige, die die Regel des Projekts einhält, ohne die Gleichheit der drei Fassungen
aufzugeben. Möglichkeit 3 wäre der billigste Eingriff und die teuerste Folge.

---
Answered: 260905-2008-orchestrator-session.md `## Fuenf weitere Entscheidungen am 260907-0823 beantwortet` — alle drei bekommen sie, in einem Durchgang ueber die drei Kisten; nur die Fassung im Messwerkzeug zu bemarken waere der billigste Eingriff und die teuerste Folge, weil die drei zeichengleich bleiben sollen; ruled by user, Kai Stalmann <kai@stalmann.org>.
