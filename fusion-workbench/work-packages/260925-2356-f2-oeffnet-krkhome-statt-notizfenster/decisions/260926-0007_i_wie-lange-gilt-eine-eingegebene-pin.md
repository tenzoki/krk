# Wie lange gilt eine eingegebene PIN, und kann der Nutzer sie ändern?

---
**Domain:** code
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260925-2356-f2-oeffnet-krkhome-statt-notizfenster.md, 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0007_*_wie-wird-secrets-txt-verschluesselt-und-was-schuetzt-eine-vierstellige-pin.md

---

## Question

Die Directive sagt: „zum editieren muss die pin eingegeben werden". Offen ist, wie lange die Eingabe gilt, also wann KRK den entschlüsselten Inhalt wieder aus dem Speicher und vom Schirm nimmt, und ob die PIN nach dem Anlegen je geändert werden kann. Ohne Antwort baut der Planer entweder eine Abfrage bei jedem Tastendruck oder eine, die bis zum Beenden gilt, und beides ist eine Verhaltensentscheidung des Nutzers.

## Options

1. **Die PIN gilt, solange `.secrets.txt` im Editor offen ist.** Schließt der Nutzer den Editor, wechselt er dort auf eine andere Datei oder beendet er KRK, ist der Inhalt wieder verschlossen, und das nächste Öffnen fragt erneut.
   - Pros: einfach zu verstehen; der Klartext steht nur so lange da, wie der Nutzer ihn sichtbar vor sich hat.
   - Cons: wer die Datei offen lässt und den Mac verlässt, lässt sie offen.
2. **Wie 1, zusätzlich schließt KRK nach einer Ruhezeit ohne Eingabe** (Vorgabe etwa fünf Minuten) und beim Sperren des Bildschirms.
   - Pros: schützt gegen die offen gelassene Datei.
   - Cons: ungesicherte Änderungen brauchen dann eine Regel; der Spec würde verlangen, dass vor dem Verschließen gesichert wird. Eine zusätzliche Zahl, die vielleicht einstellbar sein soll.
3. **Die PIN gilt bis zum Beenden von KRK.**
   - Pros: bequem; nur eine Abfrage je Sitzung.
   - Cons: der entschlüsselte Inhalt bleibt die ganze Sitzung im Speicher, und jeder, der an den Mac tritt, öffnet die Datei ohne Abfrage.

**Zur zweiten Hälfte der Frage, der Änderung der PIN**, stehen zwei Antworten: (a) ein eigener Befehl „PIN ändern" in dieser Arbeit, der die alte PIN verlangt und die Datei neu verschlüsselt; (b) keine Änderung in dieser Arbeit, der Weg ist, eine neue Datei anzulegen und den Inhalt hinüberzukopieren. Ohne (a) bleibt eine einmal verratene PIN für diese Datei gültig.

## Constraints

- Beim Verschließen darf kein ungesicherter Stand still verloren gehen; das Projekt sagt für jeden Weg aus einem Bearbeitungsstand heraus zu, dass nichts Getipptes ohne Meldung verschwindet (Runde 9, C4).
- Die PIN-Abfrage ist ein Blatt am Hauptfenster; ein Blatt über einem anderen Blatt geht in AppKit nicht.

## Recommendation

Wir empfehlen Möglichkeit 1 zusammen mit (a). Möglichkeit 1 hält die Regel an einem Ereignis fest, das der Nutzer selbst auslöst und sieht, und kommt ohne eine weitere Zahl aus; die Ruhezeit aus Möglichkeit 2 kann eine spätere Arbeit nachreichen. Die Änderung der PIN gehört in diese Arbeit, weil ohne sie eine verratene PIN nur durch Handarbeit mit Klartext in der Zwischenablage zu ersetzen ist, also genau dem Weg, den die Verschlüsselung vermeiden soll.

---
Answered: dieser Datensatz `## Options` und 260926-0017-zweitlesung-spec-f2-krkhome.md — Möglichkeit 1 mit dem Befehl PIN ändern; `.secrets.txt` wird nicht in der Sitzung gemerkt; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: 9e40d5c — Schlüssel und PIN gelten, solange die Datei offen ist; PIN ändern in d3a9983; nie in der Sitzung (8d6e0d0)
