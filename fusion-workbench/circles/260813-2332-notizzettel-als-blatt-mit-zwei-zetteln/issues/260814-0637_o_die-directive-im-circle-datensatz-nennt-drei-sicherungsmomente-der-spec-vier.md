Die Directive im Circle-Datensatz nennt drei Sicherungsmomente, der Spec vier

---

`_t_circle.md` sagt in seinem Abschnitt `## Directive`: „Gesichert wird ohne Zutun des
Nutzers, an drei Punkten — beim Wechsel zwischen den beiden Zetteln, beim Schließen des
Blattes und beim Beenden von KRK." Mit der Nutzerantwort vom 260814-0005 sind es vier:
`shift+cmd+w` sichert, bevor das Fenster schließt. Der Spec ist am 260814-0628 nachgezogen,
der Circle-Datensatz nicht.

---

**Schwere:** niedrig. Kein Bau, kein Verhalten. Die Directive ist die Aussage, gegen die
der Abschluss dieser Runde gelesen wird, und sie steht jetzt hinter dem Spec zurück.

**Warum der Shaper es nicht selbst behoben hat.** Der Abschnitt `## Directive` eines
Circle-Datensatzes ist für den Shaper allein im Modus `portfolio-activation` schreibbar.
Dieser Lauf war ein gewöhnlicher Auftrag am laufenden Circle, also außerhalb dieses Modus.
Die Regel steht in `agents/shaper.md`, Abschnitt `## Scope`.

**Betroffen sind zwei Stellen im Datensatz**, beide am 260814-0637 gelesen:

- `## Directive`, der Satz „an drei Punkten".
- `## Grounding snapshot`, die Überschrift „Drei Sicherungsmomente, und was dabei verloren
  gehen darf".

**Was zu tun ist.** Beide Stellen auf vier ziehen und `shift+cmd+w` mit aufnehmen, mit dem
Wortlaut aus der Directive des Spec. Der Weg dafür ist entweder ein Shaper-Lauf im Modus
`portfolio-activation` oder eine Änderung des Nutzers von Hand.

**Kontext**

- Gefunden beim Nachziehen des Spec am 260814-0628.
- Die Antwort, aus der der vierte Moment folgt, steht in C1 und C4 des Spec
  `planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md`.
