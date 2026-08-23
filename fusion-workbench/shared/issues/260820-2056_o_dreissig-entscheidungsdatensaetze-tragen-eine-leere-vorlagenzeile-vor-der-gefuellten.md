Dreißig Entscheidungsdatensätze tragen eine leere Vorlagenzeile vor der gefüllten, und ein `grep` liest die leere

---

Die Vorlage des Entscheidungsdatensatzes endet auf einen Block aus vier Schlüsseln — `Answered:`,
`Implemented:`, `Deferred:`, `Superseded by:` —, die beim Anlegen leer bleiben. Die Regel zum
Fortschreiben verlangt daneben, den Vermerk **unten anzuhängen**. Beides zusammen erzeugt in
derselben Datei zwei Zeilen desselben Schlüssels: oben die leere Vorlagenzeile, unten die gefüllte.
Wer den Stand mit `grep -m1 'Answered:'` oder `grep -m1 'Superseded by:'` abfragt — und das ist die
naheliegende Form —, bekommt die leere und schließt auf „nicht beantwortet" beziehungsweise „nicht
überholt".

---

**Schwere:** gering für den Inhalt, mittel für jede maschinelle Auswertung. Kein Datensatz ist
falsch; jeder trägt seine Auskunft, nur nicht an der Stelle, an der die einfachste Abfrage sie sucht.
**Gefunden von:** curator, zweimal als Kandidat K04 vorgelegt (Läufe `260819-1500` und `260820-1119`)
und beide Male nicht aufgenommen; hier vom reconciler auf den ganzen Bestand erhoben und abgelegt.
**Domain:** code

## Gemessen, an `f5300f4`

Über alle 158 Entscheidungsdatensätze in `shared/decisions/` und `circles/*/decisions/`, ohne das
Archiv. Gezählt ist der Fall „ein Schlüssel steht mindestens zweimal, die **erste** Fundstelle ist
leer, und eine spätere ist gefüllt".

**30 Dateien, 46 Schlüsselfälle.** Die Verteilung über die vier Schlüssel: `Answered` und
`Implemented` tragen den Löwenanteil, `Deferred` zweimal, `Superseded by` zweimal.

Die zwei Fälle mit `Superseded by` sind die, die der Kurator als K04 vorgelegt hat:

- `circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_s_ist-die-neue-leiste-die-statuszeile-aus-c1-oder-eine-zweite-flaeche.md` — Zeile 73 leer, Zeile 74 gefüllt
- `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/decisions/260813-0939_s_wer-setzt-den-ersten-tag-v0-1-0-und-wann.md` — Zeile 41 leer, Zeile 47 gefüllt

**Der Kurator hat richtig gemessen und zu eng.** Er hat den Schlüssel `Superseded by` geprüft und
zwei Treffer gemeldet; über alle vier Schlüssel sind es 46 in 30 Dateien, also mehr als zwanzigmal so
viele. Sein Grund, nicht zu handeln, bleibt richtig: ein Formfehler ist keiner seiner zwei
Änderungsgründe, und er hat ihn deshalb an einen Abgleich verwiesen.

## Das ist keine Nachlässigkeit einzelner Datensätze, sondern eine Kollision zweier Regeln

`rules/fusion-workbench-conventions.md` schreibt beides:

- `## Decision Record Template` setzt den Block der vier Schlüssel an das **Ende** des Rumpfs, mit
  Platzhaltern der Form `Answered: <set when status moves to _a_>`.
- `### Decision files` unter `## Inline State Tracking` sagt, der Vermerk werde als eigener Block
  mit `---` davor **angehängt**.

Wer beides befolgt, erzeugt den Befund. **Dieser Abgleich hat ihn selbst achtmal reproduziert**, beim
Heben von acht Datensätzen auf `_i_` — der Stand nach diesem Durchgang ist 32 Dateien und 54
Schlüsselfälle. Das ist der Beleg dafür, dass die Gestalt aus der Regel folgt und nicht aus
Unachtsamkeit: ein Durchgang, der die Regel wörtlich befolgt, vergrößert den Bestand.

## Was zu tun wäre

Zwei Wege, und dieser Datensatz wählt keinen:

1. **Am Bestand.** Die leere Vorlagenzeile aus den 30 Dateien nehmen, wo eine gefüllte Zeile
   desselben Schlüssels darunter steht. Mechanisch, prüfbar, und es fasst 30 Dateien an — darunter
   terminale (`_i_`, `_s_`, `_d_`), was in diesem Projekt Aufzeichnungen sind. Wer es fährt, prüft
   vorher, ob die Ortsregel aus `CLAUDE.md` dem entgegensteht.
2. **An der Regel.** Die Kollision in `rules/fusion-workbench-conventions.md` auflösen, also
   entweder den Vorlagenblock streichen oder das Anhängen durch ein Ersetzen der Vorlagenzeile
   ersetzen. Das ist eine Frage an fusion und nicht an dieses Projekt, und sie beendet den Zufluss,
   während Weg 1 nur den Bestand räumt.

**Bis dahin gilt für jede Abfrage in diesem Baum:** nicht `grep -m1`, sondern die **letzte** Fundstelle
lesen, etwa `grep 'Answered:' <datei> | tail -1`. Denselben blinden Fleck beschreibt
`shared/issues/260818-0710_*_forty-three-closure-notes-are-written-in-a-form-no-resolved-sweep-finds.md`
für die Schließungsnotizen des Defektspeichers, mit anderer Ursache und gleicher Wirkung.

---
Also seen: 260823-1336 by reconciler — der Bestand liegt heute bei 28 statt dreißig, und die
Differenz ist kein Fortschritt: zwei der genannten Datensätze sind am 260820 ins Archiv
gewandert, korrigiert wurde keiner. Neu hinzugekommen ist einer aus der Sitzung `260823-0442`,
`shared/decisions/260823-1137_*_holt-der-rueckweg-von-cmd-e-die-vorschau-*`. Erhoben über alle
161 Datensätze in `shared/decisions/` und `circles/*/decisions/` mit der Bedingung „erste leere
Vorlagenzeile steht vor der ersten gefüllten".
