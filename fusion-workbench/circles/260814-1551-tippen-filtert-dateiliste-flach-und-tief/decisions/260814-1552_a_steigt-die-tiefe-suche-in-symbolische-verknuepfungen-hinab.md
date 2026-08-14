# Steigt die tiefe Suche in symbolische Verknüpfungen hinab?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `crates/krk-core/src/verzeichnis/sys.rs:341` (`Typ::Verknuepfung`); `crates/krk-core/src/verzeichnis/leser.rs` (der gestückelte Lesevorgang)

---

## Question

Die tiefe Suche bekommt weder eine Tiefengrenze noch einen Deckel auf die Trefferzahl; so hat der Nutzer es entschieden. Damit hängt die Frage, ob der Durchlauf endet, allein am Unterbaum. Ein Unterbaum mit symbolischen Verknüpfungen ist kein Baum: eine Verknüpfung, die auf einen ihrer eigenen Vorfahren zeigt, schließt einen Kreis, und ein Durchlauf, der ihr folgt, läuft ohne Abbruch weiter. Der Nutzer sähe eine Liste, die endlos wächst und dieselben Dateien mehrfach führt. Der Baum unterscheidet Verknüpfungen bereits beim Lesen, es gibt für sie also einen Zweig; welchen Weg er nimmt, ist nicht entschieden.

Die Frage ist keine reine Umsetzungsfrage. Sie bestimmt, was der Nutzer findet: ein Heimatverzeichnis mit einer Verknüpfung auf ein eingehängtes Laufwerk enthält für ihn dessen Inhalt, für einen Durchlauf ohne Nachverfolgung nicht.

## Options

1. **Verknüpfungen werden nicht verfolgt.** Der Durchlauf steigt nur in echte Ordner hinab; eine Verknüpfung auf einen Ordner erscheint als Treffer, wenn ihr Name passt, wird aber nicht geöffnet.
   - Pro: der Durchlauf endet immer, ohne dass irgendetwas mitgezählt werden muss. Jede Datei erscheint höchstens einmal. Es ist das Verhalten, das `find` ohne Zusatzangabe zeigt.
   - Kontra: der Nutzer findet nichts hinter einer Verknüpfung, auch dort nicht, wo sie sein üblicher Weg zu den Dateien ist.
2. **Verknüpfungen werden verfolgt, und besuchte Ordner werden gemerkt.** Der Durchlauf hält jeden schon besuchten Ordner fest und betritt ihn nicht ein zweites Mal.
   - Pro: der Nutzer findet, was er über die Verknüpfung ohnehin sieht, und der Kreis ist geschlossen.
   - Kontra: der Durchlauf muss über seine ganze Laufzeit eine Menge besuchter Ordner halten, was bei einem großen Unterbaum Speicher kostet, den die heutige Stapelbauart bewusst nicht braucht. Dieselbe Datei kann über zwei Wege zweimal in der Liste stehen.
3. **Verknüpfungen werden verfolgt, aber nur nach unten.** Der Durchlauf folgt einer Verknüpfung, deren Ziel unterhalb des Ausgangsordners liegt, und keiner, die hinausführt.
   - Pro: braucht keine Menge besuchter Ordner, sondern nur einen Pfadvergleich je Verknüpfung.
   - Kontra: schließt den Kreis nicht. Zwei Verknüpfungen innerhalb des Unterbaums, die aufeinander zeigen, laufen weiter im Kreis. Die Regel ist damit weder überschneidungsfrei noch vollständig gegen den Fall, für den sie da ist.

## Constraints

- Es gibt keine Tiefengrenze und keinen Deckel; ein Abbruch nach einer Zahl ist deshalb keine der Möglichkeiten.
- Der Nutzer kann jeden laufenden Durchlauf mit einem Tastendruck anhalten. Das begrenzt den Schaden eines Kreises, hebt die Frage aber nicht auf: ein Durchlauf, der von selbst nie endet, ist eine Zusage, die die Runde nicht halten will.
- Der Lesevorgang hält heute keinen Zustand über den laufenden Ordner hinaus. Möglichkeit 2 führt einen ein.
- Die Antwort gilt gleichermaßen für Verknüpfungen auf Ordner außerhalb des Heimatverzeichnisses und für eingehängte Laufwerke.

## Recommendation

Möglichkeit 1. Sie ist die einzige der drei, bei der der Durchlauf ohne einen mitgeführten Zustand endet, und sie entspricht dem, was der Nutzer von den Werkzeugen des Systems gewohnt ist. Möglichkeit 3 sieht billiger aus als Möglichkeit 2 und löst die Frage nicht, für die sie gebaut wäre.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: Nutzer am 260814-1610 — Moeglichkeit 1, wie empfohlen: die tiefe Suche steigt nicht in symbolische Verknuepfungen hinab. Der Nutzer haelt daneben fest, dass Verknuepfungen ausserhalb der Suche begehbar sein sollen und es heute nicht sind. Das ist ein eigener Defekt und nicht Gegenstand dieser Runde; abgelegt als `shared/issues/260814-1612_o_eine-verknuepfung-auf-einen-ordner-laesst-sich-nicht-betreten.md`.
