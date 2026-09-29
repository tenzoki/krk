# Wie teilen zwei Funktionen desselben Zustellers eine Kombination, wenn die eine nur wirkt, solange die Vorschau eine Bildfolge zeigt?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md (C3, C4, vierter Punkt unter `## Stops when`); 260929-1423_*_plan-vorschau-blaettert-fotos-nach-aufnahmedatum.md (Schritt 2 prüft diese Frage, Schritt 7 baut die Antwort); 260926-2308_*_duerfen-zwei-funktionen-desselben-zustellers-eine-kombination-tragen-wenn-ihre-wirkungsbereiche-einander-ausschliessen.md (die erste Art des Teilens)

---

## Question

Der Spec verlangt, dass Cmd+Pfeil hoch neben „In den übergeordneten Ordner“ auch „Voriges Bild“ trägt und Return neben „Mit dem Standardprogramm öffnen“ auch den Sprung zum Foto. Beide Paare liegen auf derselben Seite (`Seite::Ausserhalb`) und haben denselben Zusteller. Die heutige Regel in `begegnen` (`crates/krk-core/src/tasten/belegung.rs`) lässt ein Teilen allein zu, wenn die Wirkungsbereiche einander ausschließen, und meldet beide Paare als Konflikt. Die Auslieferungsbelegung bräche beim Laden ab. Zu entscheiden ist, in welcher Form die zweite Art des Teilens in die Regel kommt. Die Antwort bindet über diese Arbeit hinaus: jeder spätere Befehl, dessen Bedeutung am Inhalt einer Fläche hängt, nimmt denselben Weg.

## Options

1. **Verengung neben Ausschluss, an der einen Stelle.** Ein neuer Wirkungsbereich `Bildfolge` („Fokus im Dateifenster, und die Vorschau zeigt eine Bildfolge“) verengt `Dateifenster`: wo er zulässig ist, ist `Dateifenster` es auch. Die eine Regelfunktion lässt zwei Funktionen eine Kombination teilen, wenn ihre Bereiche einander ausschließen oder der eine den anderen verengt, und sie lässt höchstens zwei Funktionen desselben Zustellers auf einer Kombination zu. `Belegung::nachschlag` liefert bei einer Verengung die engere Funktion zuerst; `zulaessigkeit::waehlen` bleibt unverändert und nimmt damit die engere, wo sie zulässig ist, sonst die weitere.
   - Pros: eine Regelstelle für beide Arten; `waehlen`, der Ereignisabgriff und das Hauptmenü ändern nichts; eine Probe kann über jede Lage halten, dass die Verengung nie ohne ihren weiteren Bereich zulässig ist.
   - Cons: die Regel wird von paarweise zu mengenweise, weil drei paarweise verträgliche Funktionen (Editor, Dateifenster, Bildfolge) sonst eine Kombination teilen könnten und `nachschlag` die dritte still überginge. Die Reihenfolge eines geteilten Nachschlags hängt dann nicht mehr allein an der Datei.
2. **Eine zweite Regel neben `begegnen`.** Die Verengung bekommt eine eigene Prüfung in `konflikte` und `zuweisen`.
   - Pros: die bestehende Regel bleibt Wort für Wort.
   - Cons: genau der Fall, den der Spec als Haltepunkt nennt; zwei Regeln, die dieselbe Frage beantworten, laufen auseinander.
3. **Blättern auf Umschalt+Cmd+Pfeil hoch/runter.** Die Kombinationen sind frei, kein Teilen ist nötig; Return bräuchte trotzdem eine eigene Antwort.
   - Pros: die Konfliktregel bleibt unberührt.
   - Cons: weicht von der Nutzerwahl 2A und 3A ab und löst Return nicht.

## Constraints

- Zwei andere Befehle des Dateifensters auf derselben Kombination bleiben ein Konflikt (C3 des Spec, letztes Kriterium).
- Pfeil links führt in jeder Lage in den übergeordneten Ordner; „In den übergeordneten Ordner“ bleibt deshalb ein Befehl des Bereichs `Dateifenster` und wird nicht selbst verengt.
- Die Frage „zeigt die Vorschau eine Bildfolge?“ steht in `zulaessigkeit::Lage` und wird wie `form_passt` gefragt, nicht im Kern, der die Oberfläche nicht kennt.

## Recommendation

Wir empfehlen Möglichkeit 1, vorbehaltlich des Urteils aus Schritt 2 des Plans. Sie hält die Regel an einer Stelle und lässt die zwei Zusteller der Wahl (Ereignisabgriff und Menü) unberührt. Die Mengenform ist keine zweite Regel: sie ist dieselbe Frage, gestellt an die Funktionen, die die Kombination schon tragen, statt an eine davon.
