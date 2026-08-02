# Meint L4 mit "wiederhergestellten Tabs" die vollständig gelesenen Ordner oder die bedienbare erste Bildschirmseite?

---
**Domain:** code
**Status:** answered
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

## Nachtrag des Shapers 260802-1445: was der Frage als Autor von C8 noch fehlt

Die drei Möglichkeiten tragen, und die Empfehlung des Planners bleibt unwidersprochen. Drei Punkte kommen hinzu, die aus der Herkunft der Zusagen folgen und die Frage schärfen, ohne sie zu beantworten.

**Erstens: C8 nennt keine Sitzungslage, auf der L4 gemessen wird, und das trifft alle drei Möglichkeiten.** Die Messbedingungen legen Referenzgerät, Cache-Zustand, Prüfordner und zwanzig Wiederholungen fest. L4 ist die einzige der zehn Zusagen, deren Messung an einem Zustand hängt, den ein vorheriger Lauf hinterlassen hat: an den Tabs, die wiederhergestellt werden. Selbst unter Möglichkeit 1 wächst die Startlast mit der Zahl der sichtbaren Tabs, weil jeder von ihnen eine erste Bildschirmseite braucht. Die Antwort auf diese Frage muss deshalb einen Satz für die Messbedingungen mitliefern, etwa: zwei Dateifenster mit je einem Tab auf dem Prüfordner mit 10.000 Einträgen. Ohne einen solchen Satz bleibt L4 auch unter der milderen Lesart nicht wiederholbar, und der Grund, aus dem der Planner die Frage gestellt hat, besteht fort.

**Zweitens: die Frage reicht über L4 hinaus bis L5, und beide hängen an derselben Antwort.** C1 lässt beliebig viele Tabs je Dateifenster zu, sichtbar ist höchstens einer je Fenster. Möglichkeit 1 spricht von "jedem sichtbaren Tab" und stellt damit stillschweigend die Anschlussfrage, wann ein Tab im Hintergrund gelesen wird. Wird er erst beim Hinwechseln gelesen, gerät L5 in dieselbe Klemme wie L4: L5 sagt 50 ms für den Tabwechsel zu, und ein Ordner mit 100.000 Einträgen ist in 50 ms nicht vollständig gelesen. Die Antwort sollte deshalb in einem Zug festhalten, ob die gewählte Lesart auch für den Tabwechsel gilt. Diese Ausweitung ist bewusst in diesen Datensatz gezogen und nicht als zweite Frage danebengestellt: es ist derselbe Begriff, an derselben Stufengrenze, und zwei Datensätze würden zwei Antworten erlauben, die sich widersprechen können.

**Drittens: was die Antwort am Spec ändert, ist eng begrenzt.** Betroffen sind die Zeile L4 in der Tabelle von C8, ein Satz unter den Messbedingungen desselben Abschnitts und, falls Punkt zwei mitbeantwortet wird, die Zeile L5. C1 bleibt unberührt; die Wiederherstellung der Tabs samt Ordner und Auswahl ist unter jeder Lesart zugesagt. Keine der zehn Zahlen ändert sich.

**Zur Empfehlung.** Der Shaper schließt sich Möglichkeit 1 an, aus einem Grund, der im Aufbau von C8 liegt. Der Abschnitt trennt "sichtbar und bedienbar" von "vollständig gelesen" bereits einmal, in L2 gegen L3. Möglichkeit 1 wendet dieselbe Trennung auf den Kaltstart und, mit Punkt zwei, auf den Tabwechsel an und kommt damit auf eine Regel für drei Fälle. Möglichkeit 3 führt eine Schwelle ein, die der Spec sonst nirgends kennt, und Möglichkeit 2 macht eine Zusage vom Inhalt der letzten Sitzung abhängig. Die Abwägung stützt sich auf die Systematik des Specs, nicht auf eine Messung; die Entscheidung liegt beim Nutzer.

---
Answered: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`:249 — **Möglichkeit 1, und sie gilt auch für den Tabwechsel aus L5.** Nutzerentscheidung am 260802-1735.

Der Wortlaut der Entscheidung: der Kaltstart ist abgeschlossen, wenn Fenster, Tabs, Leisten und die erste Bildschirmseite jedes sichtbaren Tabs stehen und die Tastatur reagiert; das vollständige Lesen läuft danach weiter und fällt unter L3 beziehungsweise L10. Punkt zwei des Shaper-Nachtrags ist mitentschieden: dieselbe Lesart gilt für L5, ausdrücklich als eine Regel für beide Fälle. Keine der zehn Zahlen aus C8 ändert sich.

Eingearbeitet in den Spec an vier Stellen, alle in C8: Zeile L4 (`:249`), Zeile L5 (`:250`), der Absatz "Was L4 und L5 als abgeschlossen zählt" (`:257`) und die Messbedingungen (`:238` und `:239`).

Zu Punkt eins des Nachtrags, der Sitzungslage. Der Nutzer hat die Formulierung dem Shaper überlassen. Gewählt ist eine Prüfsitzung aus zwei Dateifenstern mit je zwei Tabs: im ersten Fenster ist der Tab auf Prüfordner A sichtbar und der auf B im Hintergrund, im zweiten Fenster umgekehrt, die Auswahl jeweils auf dem ersten Eintrag, Lesezeichenleiste und Vorschau eingeblendet. A und B sind zwei nach demselben Verfahren erzeugte Ordner mit je 10.000 Einträgen an verschiedenen Pfaden. Die im Nachtrag vorgeschlagene Lage mit je einem Tab auf demselben Ordner ist damit an zwei Punkten erweitert, und beide Erweiterungen haben denselben Grund, die Wiederholbarkeit der Messung. Zwei verschiedene Ordner verhindern, dass der zweite Lesevorgang eines Kaltstarts aus dem Cache des Systems bedient und L4 zur Hälfte warm gemessen wird. Der zweite Tab je Fenster gibt L5 einen Zieltab, ohne den der Tabwechsel in derselben Sitzung nicht messbar wäre; ohne ihn bräuchte L5 eine eigene Sitzungslage, und die zweite Lage wäre die Sonderregel, die die Maxime "supersimpel" ausschließt.

Eine Folgerung, die aus der Entscheidung und den vorhandenen Zahlen folgt und deshalb keine eigene Frage geworden ist: wechselt der Nutzer auf einen Tab, dessen Ordner KRK noch nicht gelesen hat, sagt L5 den Wechsel selbst zu, während die erste Bildschirmseite des Zielordners unter L2 mit 100 ms fällt und das vollständige Lesen unter L3 beziehungsweise L10. Anders ginge es nicht, weil L2 für genau diese erste Bildschirmseite 100 ms veranschlagt und die 50 ms aus L5 sie nicht unterbieten können. Der Fall bleibt der Ausnahmefall, weil KRK nach dem Erreichen der bedienbaren Oberfläche weiterliest. Der Absatz `:257` im Spec schreibt das aus.
Implemented:
Deferred:
Superseded by:
