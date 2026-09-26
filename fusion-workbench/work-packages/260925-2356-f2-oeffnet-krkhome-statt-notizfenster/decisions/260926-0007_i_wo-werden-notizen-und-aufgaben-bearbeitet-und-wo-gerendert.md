# Wo werden Notizen und Aufgaben bearbeitet, wo gerendert, und wie verhalten sich die Sondereditoren zum bestehenden Editor?

---
**Domain:** code
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260925-2356-f2-oeffnet-krkhome-statt-notizfenster.md, 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0007_*_in-welchem-format-stehen-notes-txt-und-tasks-txt.md

---

## Question

Die Directive verlangt für `notes.txt` einen „speziellen Editor für tabellenartige Einträge und gerenderte Darstellung", für `tasks.txt` einen „speziellen Editor für Operationen und gerenderte Darstellung" und für `.secrets.txt` denselben Editor wie für die Notizen. KRK hat heute zwei Flächen für den Inhalt einer Datei: die Vorschau (dritter Bereich, nur lesend, rendert Markdown seit der Runde 6) und den Editor (fünfter Bereich, mit Roh- und Formatansicht, `ctrl+cmd+e` wechselt). Offen ist, auf welche dieser Flächen die Sondereditoren und die gerenderte Darstellung kommen, und damit auch, wo der Klick auf das Erledigt-Kästchen einer Aufgabe wirkt. Das bestimmt den Zuschnitt von Stufe 2 bis 4 und ob die Vorschau zum ersten Mal Dateien schreibt.

## Options

1. **Die Vorschau zeigt die gerenderte Darstellung und schreibt nichts; alle Operationen geschehen im Editor, dessen Formatansicht für diese drei Dateien die Sonderform ist.** Mit F4 oder `cmd+e` geht der Nutzer von der ausgewählten Datei in den Editor; die Formatansicht zeigt dort die Notizen als Tabelle aus Thema und Notiz beziehungsweise die Aufgaben als Liste mit Kästchen und den Befehlen Hinzufügen, Verschieben, Löschen, Abhaken. Die Rohansicht zeigt wie bei jeder Datei den Text.
   - Pros: kein neuer Bereich, keine neue Schreibstelle neben dem Editor; Sichern, die Rückfrage bei ungesichertem Stand und Rückgängig des Editors gelten unverändert. Die Vorschau bleibt, was sie seit der Runde 1 ist: lesend. Die Rohansicht bleibt als Ausweg, wenn die Sonderform eine Datei nicht versteht.
   - Cons: ein Kästchen abhaken braucht den Schritt in den Editor. Die Formatansicht wird für drei Pfade zu etwas grundsätzlich anderem als für jede andere Datei.
2. **Wie 1, aber das Abhaken geht auch in der Vorschau.** Ein Klick auf ein Kästchen in der gerenderten Darstellung ändert `tasks.txt` sofort.
   - Pros: die häufigste Handlung an einer Aufgabenliste geht mit einem Klick.
   - Cons: die Vorschau schreibt zum ersten Mal eine Datei, und damit braucht sie eine Regel für den Fall, dass dieselbe Datei im Editor mit ungesichertem Stand offen ist. Die Tastatursteuerung, eine Maxime des Projekts, verlangt dann einen Tastenweg zum Abhaken auch in der Vorschau.
3. **Ein eigener Bereich oder ein eigenes Blatt für die drei Dateien.**
   - Pros: die Sonderform muss sich nicht in die Ansichtenlogik des Editors einfügen.
   - Cons: ein Blatt ist genau das blockierende Fenster, das die Directive abschafft. Ein siebter Bereich der Fensterzeile erweitert drei Aufzählungen, an denen die Runde 23 neun Stellen von Hand nachziehen musste.

## Constraints

- Die Directive schafft das blockierende Fenster ab; eine Lösung als Blatt widerspricht ihr.
- Jede Möglichkeit muss vollständig über die Tastatur bedienbar sein und die Maus zusätzlich zulassen (Maxime aus `idea.txt`).
- Für `.secrets.txt` zeigt die Vorschau nach der Directive nichts oder einen Hinweis; das gilt in jeder Möglichkeit.

## Recommendation

Wir empfehlen Möglichkeit 1. Sie ordnet das Neue in die bestehende Teilung ein, in der die Vorschau zeigt und der Editor schreibt, und sie bekommt Sichern, Rückfragen und Rückgängig geschenkt, statt sie ein zweites Mal zu bauen. Möglichkeit 2 lässt sich später als eigene Arbeit nachreichen, wenn sich der Schritt in den Editor im Alltag als lästig erweist; dann mit einer ausdrücklichen Regel für die offene Datei im Editor.

---
Answered: dieser Datensatz `## Options` und 260926-0017-zweitlesung-spec-f2-krkhome.md — Möglichkeit 1, jede Tabellenhandlung wird als Textänderung am Editorstand ausgeführt, damit Sichern, Rückfrage und Rückgängig greifen; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: e6cc96b — Tabellenhandlungen als Textänderung am Editorstand über den Umbauweg; Vorschau liest nur (e98fd5d)
