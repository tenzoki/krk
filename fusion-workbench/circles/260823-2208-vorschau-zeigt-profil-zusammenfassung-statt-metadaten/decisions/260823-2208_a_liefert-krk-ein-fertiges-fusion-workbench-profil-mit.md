# Liefert KRK ein fertiges fusion-workbench-Profil mit, oder schreibt der Nutzer es selbst?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/_*_circle.md`, `krk-core/src/ablage/einstellungen.rs`, `resources/default-settings.toml`

---

## Question

Die Definitionsdatei `readers.toml` entsteht beim ersten Start im Bestandsort, so wie `settings.toml` es heute tut. Offen ist, was darin steht. Der Beispielfall der ganzen Runde ist die fusion-workbench, und ihr Profil umfasst nach der Skizze des Backlogeintrags rund sechzig Zeilen TOML. Liefert KRK dieses Profil mit, trägt sein Bündel die Konventionen eines fremden Projekts; liefert es keines mit, sieht der Nutzer vor eigener Handarbeit kein Ergebnis, und die Abnahme der Runde hängt an einer Datei, die im Baum nicht existiert.

## Options

1. **Mitgeliefert und wirksam** — `resources/default-readers.toml` wird über `include_str!` eingebettet und beim ersten Start wörtlich angelegt, genau wie `settings.toml`.
   - Pros: Der Beispielfall wirkt ohne eine Zeile Handarbeit, und die Runde hat ein vorführbares Ergebnis. Der Weg dorthin existiert bereits in `einstellungen.rs` und wird nicht zweimal gebaut.
   - Cons: KRKs Bündel trägt die Ablagekonventionen von fusion und muss nachziehen, wenn fusion sie ändert. Genau das ist mit dem Umbau auf Circle-Verzeichnisse schon einmal geschehen.
2. **Mitgeliefert, aber auskommentiert** — Die Auslieferungsfassung trägt das Profil vollständig, mit vorangestellten Kommentarzeichen. Der Nutzer nimmt sie weg.
   - Pros: Die Vorlage steht bereit, und ein veraltetes Profil wirkt nicht ungefragt. Die Verantwortung für die Konventionen bleibt beim Nutzer.
   - Cons: Ein Handgriff, den nur findet, wer die Datei öffnet. Nach dem ersten Start zeigt die Vorschau dasselbe wie heute, und der Nutzer sieht der Runde ihr Ergebnis nicht an.
3. **Leere Datei mit kommentiertem Beispiel** — Die Auslieferungsfassung erklärt die Bausteine an einem kurzen Beispiel; das fusion-Profil schreibt der Nutzer selbst.
   - Pros: KRK bleibt frei von den Konventionen eines fremden Projekts, und die Datei veraltet nicht.
   - Cons: Vor dem ersten sichtbaren Ergebnis stehen rund sechzig Zeilen TOML, die der Nutzer aus der fusion-Dokumentation zusammensucht.

## Constraints

`settings.toml` gibt die Form vor: die Auslieferungsfassung ist einkompiliert, wird beim ersten Start wörtlich geschrieben und danach von KRK nie wieder angefasst, damit ihre Kommentarzeilen stehen bleiben. Jede Antwort muss diesen Weg nehmen und keinen zweiten daneben. Ein veraltetes Profil bricht nichts: nach der Antwort des Nutzers vom 260823 bleibt ohne Treffer die heutige Metadatenanzeige stehen.

## Recommendation

Möglichkeit 1. Der Backlogeintrag nennt die fusion-workbench als den Fall, um dessentwillen die Runde überhaupt gefahren wird, und ein veraltetes Profil verschlechtert die Anzeige nicht, sondern fällt auf den heutigen Stand zurück. Das begrenzt den Preis der Mitlieferung auf eine Pflegeaufgabe, die sich am Ergebnis sofort zeigt.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:

---
Answered: circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md:56 — Mitgeliefert und wirksam (Möglichkeit 1); default-readers.toml über include_str! wie settings.toml.
