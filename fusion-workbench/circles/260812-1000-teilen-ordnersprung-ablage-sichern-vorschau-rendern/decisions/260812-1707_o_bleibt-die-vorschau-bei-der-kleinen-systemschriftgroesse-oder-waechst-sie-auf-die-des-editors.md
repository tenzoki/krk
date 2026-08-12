# Bleibt die Vorschau bei der kleinen Systemschriftgröße oder wächst sie auf die des Editors?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator
**Cross-references:** `crates/krk-ui/src/appkit/vorschau.rs` (`text_zeigen`); `crates/krk-ui/src/appkit/textmerkmale.rs` (`grundschrift`); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md` (Schritt 9)

---

## Question

Schritt 9 hat die Vorschau an `textmerkmale::grundschrift` angeschlossen, und damit hat sich
ihre Schriftgröße geändert, ohne dass jemand das entschieden hätte.

Vorher setzte `text_zeigen` die Schrift selbst:
`userFixedPitchFontOfSize(smallSystemFontSize())`, also die feste Schreibmaschinenschrift in
der kleinen Systemgröße, unter macOS 11 Punkt. Jetzt kommt sie aus
`grundschrift(Ansicht::Roh, …)` und damit aus `systemFontSize()`, unter macOS 13 Punkt. Die
Schriftart bleibt dieselbe, die Größe wächst um zwei Punkte.

**Der Anschluss selbst ist richtig und nicht die Frage.** Er ist die Folge davon, dass
`text_zeigen` die Merkmale einer zuvor angezeigten Markdown-Datei zurücknehmen muss; sonst
trüge ein Hinweistext die Überschriften der Datei davor. Und eine eigene Schriftwahl neben
`grundschrift` wäre die zweite Wahrheit, die Schritt 7 gerade beseitigt hat: die Fläche stünde
in einer anderen Schrift da als ihr Inhalt.

Die Frage ist allein die Größe. Sie ist am laufenden Bündel zu beurteilen und nicht am Code.

## Options

1. **So lassen.** Vorschau und Editor zeigen Text in derselben Größe.
   - Pros: eine Größe für beide Textflächen, keine Ausnahme, keine Zeile Code. Dieselbe
     Datei sieht in Vorschau und Editor gleich aus, was den Wechsel zwischen beiden ruhiger
     macht.
   - Cons: die Vorschau ist die schmalere Fläche und zeigt bei größerer Schrift weniger. Wer
     sie zum schnellen Hineinsehen benutzt, sieht künftig weniger Zeilen auf einmal.

2. **Die kleine Größe für die Vorschau zurückholen**, über einen Parameter an `grundschrift`
   oder eine zweite `Ansicht`.
   - Pros: die Vorschau bleibt, wie sie war; niemand muss sich umgewöhnen.
   - Cons: `grundschrift` bekommt eine Unterscheidung, die es heute nicht hat, und die Frage
     „welche Größe gilt wo" bekommt zwei Antworten statt einer.

3. **Beide Flächen auf die kleine Größe.** Auch der Editor zeigt künftig 11 Punkt.
   - Pros: eine Größe, und die kleinere; auf beiden Flächen passt mehr Text.
   - Cons: ändert den Editor, den in dieser Runde niemand anfassen wollte, und die Runde 2
     hat seine Größe nicht zufällig gewählt.

## Constraints

- Es bleibt bei **einer** Stelle, die über die Grundschrift entscheidet. Eine zweite
  Schriftwahl neben `grundschrift` ist in keiner Möglichkeit statthaft.
- Die Schriftart selbst steht nicht zur Debatte: C6 der Runde 1 verlangt für die Rohansicht
  die feste Schreibmaschinenschrift des Nutzers.

## Recommendation

Keine. Die Frage ist eine Geschmacksfrage über eine sichtbare Fläche und gehört an das
laufende Bündel. Sie hält keinen Planschritt auf; die Schritte 10 und 11 fassen die
Schriftgröße der Vorschau nicht an.

---
Answered:
Implemented:
Deferred:
Superseded by:
