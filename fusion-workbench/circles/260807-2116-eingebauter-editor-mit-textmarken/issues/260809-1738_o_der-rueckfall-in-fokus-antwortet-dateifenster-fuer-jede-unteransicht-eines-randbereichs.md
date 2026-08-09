# Der Rückfall in `fokus()` antwortet `Dateifenster` für jede Unteransicht eines Randbereichs

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coder, beim Bau von S17
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs` (`Anwendungsdelegierter::fokus`, `Anwendungsdelegierter::fokusansicht`)
**Cross-references:** `issues/260809-1640_c_der-fokus-kennt-den-editor-nicht-obwohl-der-abgriff-ihn-seit-s4-durchlaesst.md`, `crates/krk-ui/src/kommandos/fokus.rs` (`Fokus`), S17

---

## Der Befund

`Anwendungsdelegierter::fokus` beantwortet die Frage nach dem Fokus über die
**Nämlichkeit** des Ersthelfers: er wird gegen genau eine Ansicht je Fokuswert
gehalten, die `fokusansicht` nennt — die Tabelle der Leiste, die Inhaltsfläche
der Vorschau, die Textfläche des Editors, die Liste des aktiven Dateifensters.
Trifft keine davon zu, lautet die Antwort `Fokus::Dateifenster`.

Damit ist ein Ersthelfer, der **innerhalb** eines Randbereichs sitzt, aber nicht
dessen eine genannte Ansicht ist, für KRK ein Dateifenster. Beispiele, die keine
Erfindung sind, sondern aus dem Aufbau folgen:

- Die Vorschau hängt in `fokusansicht` an ihrer Inhaltsfläche. Zieht eine
  Unteransicht darin den Ersthelferrang an sich (eine Vorschau-Ansicht des
  Systems bringt eigene Unteransichten mit), sagt `fokus()` `Dateifenster`, und
  `delete` wirft die im Dateifenster ausgewählte Datei in den Papierkorb,
  während der Nutzer in die Vorschau geklickt hat.
- Dasselbe gilt für die Bildlaufansicht um die Tabelle der Leiste und für die
  Bildlaufansicht um die Textfläche des Editors.

S17 hat das nicht verursacht und nicht beseitigt: der Rückfall stand seit der
Runde 1 und lautete schon damals `Dateifenster`. S17 hat ihm nur die zweite
Aufgabe genommen, die er bis dahin mit trug — bis zum 260809 fing er auch die
Bereiche ab, die in der Abfrage schlicht fehlten, und das war der Defekt
`260809-1640`.

## Warum der Rückfall trotzdem stehen bleibt

`Fokus::Anderswo` an dieser Stelle hieße: ein Ersthelfer, den keine der fünf
Ansichten trifft, lässt **keinen** Befehl des Dateifensters mehr wirken. Genau
diesen Zustand hat der Defekt vom 260805-1845 schon einmal hergestellt (beim
Start stand der Ersthelfer auf der Leiste, und kein Dateibefehl wirkte bis zum
ersten `shift+cmd+d`). Der Rückfall ist deshalb die vorsichtigere der beiden
Antworten, solange die Frage so gestellt ist, wie sie gestellt ist.

## Der andere Schnitt

Die Frage lässt sich anders schneiden, und dann hat sie eine Antwort ohne
Rückfall: nicht "**ist** der Ersthelfer diese Ansicht", sondern "**liegt** er in
dieser Ansicht" (`isDescendantOf:` über die Wurzelansicht jedes Bereichs). Die
vier Teilbäume sind disjunkt, ihre Vereinigung ist der Inhalt des Fensters, und
alles außerhalb ist `Fokus::Anderswo` — eine vollständige und überschneidungs-
freie Fallunterscheidung ohne Auffangzweig.

Der Schnitt ist nicht umsonst zu haben, und die Gegenrechnung gehört dazu:

1. Der Feldeditor eines Textfeldes im Dateifenster ist eine **Unteransicht**
   des Dateifensters. Mit der Namensfrage ist er `Anderswo`; mit der
   Enthaltensfrage wäre er `Dateifenster`, und die Pfadeingabe aus C2 bekäme
   jeden Dateibefehl ab. Der Ereignisabgriff fängt ihn heute vorher ab
   (`ersthelfer_gehoert_appkit`), aber damit hinge die Richtigkeit von
   `fokus()` an einer Prüfung in einer anderen Datei.
2. Die Wurzelansicht jedes Bereichs müsste nach außen gereicht werden; heute
   geht allein die eine fokustragende Ansicht nach außen, mit ausdrücklicher
   Begründung an `Editorbereich::textflaeche`.

## Wie weit es heute reicht

Nicht gemessen. Ob eine der drei Randbereichs-Unteransichten den Ersthelferrang
im laufenden Bündel tatsächlich an sich zieht, ist von einem Agenten nicht
prüfbar; es braucht einen Lauf mit `--tasten-protokoll` und einen Klick in die
Vorschau. Der Befund steht deshalb als Sachlage im Aufbau und nicht als
beobachtetes Fehlverhalten.

## Vorschlag

Erst messen, dann schneiden. Ein Lauf am laufenden Bündel, der nach einem Klick
in die Vorschau und in den Editor den gemeldeten Fokus zeigt, entscheidet, ob
der Fall überhaupt eintritt. Tritt er ein, ist der Enthaltensschnitt die
Antwort, zusammen mit einer Festlegung, wohin der Feldeditor eines Blattes und
der der Pfadeingabe gehören.
