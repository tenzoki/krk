# Meint L4 mit "wiederhergestellten Tabs" die vollständig gelesenen Ordner oder die bedienbare erste Bildschirmseite?

---
**Domain:** code
**Status:** open
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitte C1 und C8, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` Schritte S8 und S12, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_a_leistungszusagen-navigator.md`

---

## Question

Zwei Zusagen aus C8 widersprechen sich unter der einen Lesart und vertragen sich unter der anderen, und der Spec sagt nicht, welche gilt.

L4 sagt zu: "Kaltstart bis zur bedienbaren Oberfläche mit wiederhergestellten Tabs: 1000 ms". L10 sagt zu: "Ordner mit 100.000 Einträgen: erste Bildschirmseite wie L2, vollständig 4 s warm". C1 verlangt, dass nach Beenden und erneutem Start beide Fenster wieder dieselben Tabs mit denselben Ordnern und derselben Auswahl zeigen.

Zeigt ein wiederhergestellter Tab auf einen Ordner mit 100.000 Einträgen, dann verlangt L4 in 1000 ms, was L10 mit 4 s veranschlagt, und zwar warm. Kalt liegt es noch höher. Unter der Lesart "vollständig gelesen" ist L4 damit unerfüllbar, sobald ein einziger wiederhergestellter Tab groß ist.

Die Frage muss vor der Abnahme beantwortet sein, weil L4 sonst je nach Zustand der letzten Sitzung mal hält und mal nicht, ohne dass sich am Programm etwas geändert hätte. Sie muss nicht vor der Implementierung beantwortet sein: der Plan hält beide Lesarten offen, siehe `## Constraints`.

## Options

1. **L4 meint die bedienbare Oberfläche mit den Tabs an ihren Ordnern und der ersten Bildschirmseite** — der Kaltstart ist abgeschlossen, wenn Fenster, Tabs, Leisten und die erste Bildschirmseite jedes sichtbaren Tabs stehen und die Tastatur reagiert. Das vollständige Lesen der Ordner läuft danach weiter und fällt unter L3 beziehungsweise L10.
   - Pro: deckt sich mit der Zweistufigkeit, die der Spec in L2 gegen L3 ohnehin schon zieht. L4 bleibt unabhängig davon, worauf der letzte Tab zeigte, und ist damit als Zusage überhaupt wiederholbar messbar.
   - Contra: die Bildlaufleiste eines großen Tabs steht beim Erreichen der 1000 ms noch nicht auf ihrer endgültigen Höhe. Wer "bedienbar" streng liest, könnte das als unfertig empfinden.

2. **L4 meint die vollständig gelesenen Ordner aller wiederhergestellten Tabs** — der Kaltstart ist erst abgeschlossen, wenn jeder wiederhergestellte Tab vollständig gelesen und sortiert ist.
   - Pro: die schärfere Lesart, und "bedienbare Oberfläche" heißt dann ohne Einschränkung bedienbar.
   - Contra: unerfüllbar, sobald ein wiederhergestellter Tab groß ist, und zwar aus Gründen, die L10 selbst zugesteht. Die Zusage wäre damit vom Inhalt der letzten Sitzung abhängig und als Abnahmekriterium wertlos.

3. **L4 gilt nur für wiederhergestellte Tabs unterhalb einer Grenze, etwa 10.000 Einträge** — oberhalb greift L3 beziehungsweise L10.
   - Pro: nennt eine Zahl statt eines Begriffs.
   - Contra: eine zusätzliche Schwelle mit eigener Fallunterscheidung, die der Spec sonst nirgends führt. Die Maxime "supersimpel" wirkt hier als Ausschlussgrund, und Möglichkeit 1 leistet dasselbe ohne die Schwelle.

## Constraints

- Die zehn Zahlen aus C8 selbst stehen fest und sind nicht Gegenstand dieser Frage. Zur Debatte steht die Bedeutung eines Begriffs in L4, nicht sein Wert.
- Die Antwort muss eine Messvorschrift ergeben, die auf dem Referenzgerät wiederholbar ist und nicht vom Inhalt der letzten Sitzung abhängt.
- C1 bleibt unberührt: die Wiederherstellung der Tabs samt Ordner und Auswahl ist zugesagt, gleich welche Lesart gilt. Nur der Zeitpunkt, ab dem der Kaltstart als abgeschlossen gilt, steht zur Frage.
- **Der Plan ist von der Antwort nicht blockiert.** Der gestückelte Lesevorgang aus Schritt S2 macht ein Dateifenster nach dem ersten Stapel bedienbar, unabhängig von der Ordnergröße. Möglichkeit 1 ist damit ohne Zusatzaufwand erfüllt; Möglichkeit 2 wäre eine Verschärfung des Abnahmekriteriums, keine Änderung am Entwurf.

## Recommendation

**Wir empfehlen Möglichkeit 1.**

Der Spec zieht die Trennung zwischen "sichtbar und bedienbar" und "vollständig gelesen" bereits an anderer Stelle, und zwar ausdrücklich: L2 sagt für denselben Ordner mit 10.000 Einträgen 100 ms für die erste Bildschirmseite zu und L3 im selben Atemzug 400 ms für das vollständige Lesen. Möglichkeit 1 wendet dieselbe Unterscheidung auf den Kaltstart an, statt eine zweite einzuführen.

Der entscheidende Punkt ist die Messbarkeit. Eine Zusage, deren Einhaltung davon abhängt, worauf der Nutzer beim letzten Beenden zufällig stand, lässt sich nicht abnehmen. C8 verlangt zwanzig Wiederholungen und ein 95. Perzentil, und beides setzt voraus, dass zwanzig Läufe dasselbe messen.

Die Abwägung ist eine Empfehlung auf Basis der Systematik des Specs, keine geprüfte Aussage. Die Entscheidung liegt beim Nutzer.

---
Answered:
Implemented:
Deferred:
Superseded by:
