# Bleibt der Vorspann eines Containers die eine Lücke in der Deckungszusage von C4.3?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator
**Cross-references:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_c_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md` (C4.3); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1920_*_die-deckungszusage-gilt-nicht-innerhalb-eines-elements-das-zeichen-geliefert-hat.md` (geschlossen); `crates/krk-ui/src/markdown.rs` (Modulkopf, Abschnitt „Wo die Deckung endet"; Probe `im_vorspann_eines_elements_endet_die_deckung`)

---

## Question

C4.3 sagt zu: „Alles außerhalb dieses Umfangs erscheint als der Quelltext, der dasteht." Nach
drei Reparaturen dieser Runde gilt das für jedes Byte der Quelle mit **einer** benannten
Ausnahme: den **Vorspann** eines Containers, also alles bis zum ersten darin gelesenen Byte,
dort wo sein Merkzeichen steht.

Praktisch heißt das: eine Verweisdefinition, die sich in den Vorspann eines Listenpunkts
verirrt, fällt heraus.

```
Quelle : "- [ref]: https://example.com\n\n  Text\n"
Ausgabe: "• Text"
```

Gegenüber dem Stand vor der Reparatur ist das kein Verlust, sondern ein Rest: dort war der
**ganze** Container ungedeckt, jetzt nur noch sein Vorspann.

**Die Grenze steht an drei Stellen im Baum** und ist damit keine stille Schwäche mehr: im
Modulkopf von `markdown.rs` unter „Wo die Deckung endet", im Doc-Kommentar von `luecke_bis`,
und als Probe `im_vorspann_eines_elements_endet_die_deckung`, die beide Ausgaben festschreibt.

Zu klären ist deshalb nicht, ob der Code richtig ist, sondern **ob die Zusage nachgezogen
wird.** Ein Abnahmekriterium, das weiter reicht als der Baum, ist genau der Fehler, den die
Durchsichten dieser Runde zweimal gefunden haben.

## Options

1. **Die Zusage nachziehen.** C4.3 bekommt den Zusatz „mit Ausnahme des Vorspanns eines
   Containers".
   - Pros: Zusage und Baum stimmen überein, und die Ausnahme ist gemessen, dokumentiert und
     durch eine Probe festgehalten. Kostet keine Zeile Code.
   - Cons: ein Abnahmekriterium der laufenden Runde wird nachträglich enger. Der Fall ist
     obendrein selten: eine Verweisdefinition im Vorspann eines Listenpunkts ist kein
     Markdown, das jemand absichtlich schreibt.

2. **Die Lücke schließen.** Der Vorspann wird mitgelesen.
   - Pros: C4.3 gilt dann wörtlich, ohne Ausnahme.
   - Cons: der Vorspann enthält das Merkzeichen des Containers, das die Vorschau gerade
     **nicht** zeigen soll — es wird ja durch das gerenderte ersetzt. Ihn auszugeben hieße,
     Quelltext und Rendering derselben Stelle nebeneinanderzustellen. Der Aufwand liegt darin,
     im Vorspann das Merkzeichen von allem anderen zu trennen, und das ist wieder eine
     Fallunterscheidung über Markdown-Syntax, also die Sorte Regel, die diese Runde gerade
     abgeschafft hat.

3. **So lassen und nichts nachziehen.** Der Baum trägt die Grenze, C4.3 trägt sie nicht.
   - Pros: keine Arbeit.
   - Cons: das ist der Zustand, den zwei Durchsichten dieser Runde als Befund abgelegt haben.
     Eine Zusage, die weiter reicht als der Code, wird von jemandem geglaubt.

## Constraints

- Die Auffangregel bleibt **mechanisch**: sie fragt nach dem Stand in der Quelle und nicht nach
  Ereignisarten. Eine Lösung, die eine Liste bekannter Markdown-Fälle einführt, ist in keiner
  Möglichkeit statthaft — genau die ist mit `a9e1149` abgeschafft worden.

## Recommendation

Möglichkeit 1. Der Baum ist an dieser Stelle sauberer als die Zusage, und die Ausnahme ist so
schmal und so gut dokumentiert, dass sie eine Nachführung der Zusage verdient statt eines
weiteren Umbaus. Möglichkeit 2 kostet eine Syntax-Fallunterscheidung für einen Fall, den
niemand absichtlich schreibt.

---
Answered:
Implemented:
Deferred:
Superseded by:
