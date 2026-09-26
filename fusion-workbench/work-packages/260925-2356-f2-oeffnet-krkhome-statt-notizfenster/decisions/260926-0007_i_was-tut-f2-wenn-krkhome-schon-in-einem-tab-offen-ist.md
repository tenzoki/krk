# Was tut F2, wenn ~/krkhome schon in einem Tab offen ist?

---
**Domain:** code
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260925-2356-f2-oeffnet-krkhome-statt-notizfenster.md, 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md

---

## Question

Die Directive sagt, F2 öffne „einen neuen Dateilisten-Tab". Wer F2 mehrmals drückt, bekäme bei wörtlicher Lesung jedes Mal einen weiteren Tab mit demselben Ordner. Die Tabs werden in `session.toml` gemerkt, also sammeln sie sich über Neustarts an. Offen ist außerdem, in welchem der zwei Dateifenster der Tab entsteht und was F2 tut, wenn der Fokus im Editor oder in der Vorschau steht.

## Options

1. **F2 springt zu einem vorhandenen Tab mit `~/krkhome/` im aktiven Dateifenster und öffnet nur dann einen neuen, wenn dort keiner steht.** Der Fokus geht in das Dateifenster.
   - Pros: F2 wird zu einem festen Weg „zu meinen Notizen", beliebig oft drückbar, ohne dass sich Tabs ansammeln.
   - Cons: ein Tab, der `~/krkhome/` zeigt, weil der Nutzer von Hand hineingegangen ist, zählt dann ebenfalls als vorhanden; das ist gewollt, sollte aber bekannt sein.
2. **F2 öffnet immer einen neuen Tab**, wie die Directive wörtlich sagt.
   - Pros: keine Suchregel; genau die Directive.
   - Cons: Tabs sammeln sich an und werden über die Sitzung weitergetragen.
3. **F2 wechselt zwischen dem krkhome-Tab und dem vorigen Tab hin und her.**
   - Pros: ein Tastendruck hin, einer zurück, wie `cmd+e` zwischen Dateifenster und Editor.
   - Cons: die Bedeutung von F2 hängt vom Zustand ab; wer F2 drückt, um Notizen zu sehen, und schon dort ist, wird weggeschickt.

## Constraints

- `f2` und `cmd+k` sind heute beide an das Notizblatt gebunden; der Spec setzt als Vorgabe, dass beide auf den neuen Befehl übergehen.
- Der Befehl wirkt wie bisher aus jedem Bereich heraus, und wie bisher nicht, solange ein Blatt steht.

## Recommendation

Wir empfehlen Möglichkeit 1, im aktiven Dateifenster. Sie ist die einzige, bei der F2 immer dasselbe Ergebnis hat, nämlich „der Notizordner ist vor mir", und sie lässt keine Tabs anwachsen. Die Abweichung vom Wortlaut „neuen Tab" ist gering: beim ersten Druck entsteht der neue Tab, danach wird er wiedergefunden.

---
Answered: dieser Datensatz `## Options` — Möglichkeit 1, F2 springt zu einem vorhandenen Tab im aktiven Dateifenster; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: 4c2ca49 — F2 springt zu einem vorhandenen Tab im aktiven Dateifenster
