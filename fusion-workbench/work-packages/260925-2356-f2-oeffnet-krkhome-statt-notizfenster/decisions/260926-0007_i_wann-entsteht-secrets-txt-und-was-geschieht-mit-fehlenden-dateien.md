# Wann entsteht .secrets.txt, und was geschieht, wenn eine der drei Dateien später fehlt?

---
**Domain:** code
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260925-2356-f2-oeffnet-krkhome-statt-notizfenster.md, 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0007_*_wie-wird-secrets-txt-verschluesselt-und-was-schuetzt-eine-vierstellige-pin.md

---

## Question

Die Directive sagt: „beim ersten Aufruf wird der Ordner angelegt und … folgende Dateien angelegt", und für `.secrets.txt`: „beim Erzeugen wird eine 4-stellige PIN abgefragt". Wörtlich gelesen fragt der allererste Druck auf F2 also eine PIN ab, bevor der Nutzer überhaupt einen Tab sieht; das ist ein Blatt, also genau die Art Unterbrechung, die die Directive für F2 abschafft. Außerdem sagt die Directive nichts über spätere Aufrufe: löscht der Nutzer `tasks.txt`, bleibt offen, ob F2 sie neu anlegt.

## Options

1. **Der erste Aufruf legt `notes.txt` und `tasks.txt` an; `.secrets.txt` entsteht erst, wenn der Nutzer sie zum ersten Mal öffnen will.** Bis dahin steht in der Liste ein Eintrag für sie, der beim Öffnen die PIN für die neue Datei abfragt. Jeder spätere F2-Aufruf legt eine fehlende `notes.txt` oder `tasks.txt` wieder leer an und überschreibt nie eine vorhandene.
   - Pros: F2 unterbricht nie mit einer Abfrage; die PIN wird dann gefragt, wenn der Nutzer mit Geheimnissen etwas tun will. Die zwei Klartextdateien sind immer da.
   - Cons: „immer gelistet" braucht für die noch nicht angelegte Datei einen Platzhalter in der Liste, also einen Eintrag, der keine Datei ist. Das weicht von der Directive ab, die alle drei beim ersten Aufruf anlegt.
2. **Der erste Aufruf legt alle drei an und fragt dabei die PIN ab.** Bricht der Nutzer die Abfrage ab, entstehen nur die zwei Klartextdateien, und der nächste F2-Aufruf fragt erneut. Spätere Aufrufe legen fehlende Dateien wie in 1 neu an, `.secrets.txt` wieder mit Abfrage.
   - Pros: genau die Directive; kein Platzhalter in der Liste.
   - Cons: der erste F2-Druck zeigt ein Blatt. Wer die Abfrage abbricht, bekommt sie bei jedem F2 wieder, bis er eine PIN setzt.
3. **Nur der allererste Aufruf legt an; danach legt KRK nichts mehr neu an.**
   - Pros: KRK erschafft nie eine Datei, die der Nutzer absichtlich entfernt hat.
   - Cons: eine versehentlich gelöschte `tasks.txt` bleibt weg, bis der Nutzer sie von Hand anlegt, und die Sondereditoren haben dann keine Datei.

## Constraints

- KRK überschreibt nie eine vorhandene Datei in `~/krkhome/` beim Anlegen.
- Eine neu angelegte `.secrets.txt` enthält nie Klartext, auch nicht leer als Zwischenstand.
- Lässt sich `~/krkhome/` nicht anlegen (etwa weil an der Stelle eine Datei gleichen Namens liegt oder das Schreibrecht fehlt), nennt die Statuszeile den Grund, und es öffnet sich kein Tab.

## Recommendation

Wir empfehlen Möglichkeit 2 mit einer Einschränkung: bricht der Nutzer die Abfrage ab, fragt KRK erst wieder, wenn er `.secrets.txt` öffnen will, und nicht bei jedem F2. Damit hält sich die Arbeit an den Wortlaut der Directive, und der Platzhalter aus Möglichkeit 1 entfällt; die einmalige Abfrage beim allerersten F2 ist ein bewusster Schritt, kein wiederkehrendes Hindernis. Möglichkeit 1 ist die richtige Wahl, falls der Nutzer auch diesen einen Moment der Unterbrechung nicht will.

---
Answered: 260926-0017-zweitlesung-spec-f2-krkhome.md — keine der drei Möglichkeiten: F2 legt `.secrets.txt` leer mit null Bytes an und fragt nie nach einer PIN, das erste Öffnen legt die PIN fest; fehlende Dateien legt F2 mit `create_new` neu an und überschreibt nie; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: 6f50611 — .secrets.txt entsteht mit null Bytes ohne PIN, fehlende Dateien mit create_new (be0e5b2)
