# Was geschieht mit den zwei Zetteln des bisherigen Notizblatts, wenn das Blatt entfällt?

---
**Domain:** code
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260925-2356-f2-oeffnet-krkhome-statt-notizfenster.md, 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260813-2348_*_spec-notizzettel-als-blatt-mit-zwei-zetteln.md

---

## Question

Das Notizblatt der Runde 9 hält seinen Text in zwei Dateien, `note-1.txt` und `note-2.txt` unter `~/Library/Application Support/KRK/`; beide stehen in der Aufzählung der Ablagedateien (`Datei::Zettel` in `crates/krk-core/src/ablage/pfade.rs`), und `session.toml` merkt sich, welcher zuletzt offen war. Mit dem Wegfall des Blattes liest und schreibt KRK diese Dateien nicht mehr. Die Directive sagt nicht, ob ihr Inhalt in die neuen Notizen wandert. Auf diesem Gerät können die Dateien Text tragen, den der Nutzer seit dem 260814 geschrieben hat; ein stiller Wegfall wäre Datenverlust ohne Meldung.

## Options

1. **Einmalige Übernahme in `notes.txt`, die alten Dateien bleiben unangetastet liegen.** Beim Anlegen von `~/krkhome/` wird jeder nicht leere Zettel zu einer Notiz mit dem Thema „Zettel 1" beziehungsweise „Zettel 2". Die zwei alten Dateien bleiben, wo sie sind; KRK fasst sie danach nie wieder an.
   - Pros: nichts geht verloren, und der Nutzer findet seinen alten Text dort, wo er ab jetzt Notizen sucht. Die alten Dateien bleiben als Rückfall, falls die Übernahme etwas falsch macht.
   - Cons: zwei Dateien liegen danach ohne Leser im Ablageordner; wer später aufräumt, muss wissen, dass sie gefahrlos entfernt werden können. Existiert `~/krkhome/notes.txt` beim ersten Aufruf schon (etwa von Hand angelegt), braucht die Übernahme eine Regel: der Spec schlägt vor, dann nicht zu übernehmen und nichts zu überschreiben.
2. **Keine Übernahme; die alten Dateien bleiben liegen, und die Anleitung nennt sie.** KRK legt `notes.txt` leer an. `HowTo.md` sagt, wo der alte Text liegt.
   - Pros: kein Übernahmecode, keine Frage nach dem Format der Übernahme.
   - Cons: der Nutzer muss den alten Text selbst hinüberkopieren; wer die Anleitung nicht liest, hält den Text für verloren.
3. **Übernahme und Verschieben der alten Dateien nach `~/krkhome/`.** Die Zettel wandern als Dateien in den neuen Ordner (etwa `~/krkhome/alte-zettel/`).
   - Pros: nach dem Wechsel liegt alles an einem Ort.
   - Cons: KRK verschiebt Nutzerdaten aus dem Ablageordner, was es bisher nie tut; das Projekt kopiert beim Beiseitelegen ausdrücklich und verschiebt nicht (Runde 9, C5, Festlegungen). Ein Fehler zwischen Kopie und Entfernen hinterlässt einen halben Zustand.

## Constraints

- Keine Möglichkeit darf Text der alten Zettel ohne Meldung verlieren.
- Eine `session.toml` mit dem Feld für den zuletzt offenen Zettel muss lesbar bleiben (die Sitzung trägt keine Sperre gegen unbekannte Felder; das Feld wird ignoriert).
- Die Tastenkombinationen `f2` und `cmd+k` tragen heute beide das Notizblatt. Der Spec setzt als Vorgabe, dass beide auf den neuen Befehl übergehen; das ist keine Frage dieses Datensatzes.

## Recommendation

Wir empfehlen Möglichkeit 1. Sie ist die einzige, bei der der Nutzer nichts tun muss und trotzdem nichts verliert, und sie verschiebt keine Dateien. Die zwei liegengebliebenen Dateien sind der kleinere Preis; sie lassen sich in einer späteren Runde entfernen, sobald die Übernahme sich bewährt hat.

---
Answered: dieser Datensatz `## Options` und 260926-0017-zweitlesung-spec-f2-krkhome.md — Möglichkeit 1, einmalige Übernahme, ausgelöst allein durch das Anlegen des Ordners und nicht der Datei; das alte Sitzungsfeld darf entfallen; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: be0e5b2 — einmalige Übernahme beim Anlegen des Ordners, alte Dateien unangetastet
