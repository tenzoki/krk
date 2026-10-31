# Die Quicknote (F10) sichert ihren Inhalt als scratch.txt im krk-Heimordner und trägt die zwei Schaltflächen Strip und Format

## Directive

Wortlaut des Nutzers (261031):

> neues paket: für den F10 Eidtor.
> 1.) persistiere den inhalt immer automatisch in dem krk home, bei den F2 Dateien, als scratch.txt
> 2.) Baue zwei Buttons ein:
> - Strip: entfernt die überflüssigen Leerzeichen und Zeilenbrüche, die beim Kopieren aus diesen Chat reinkommen und - wenn es bash commands etc sind - Probleme machen.
> - Format: Entfernt in Chat Textpassagen die überflüssigen Newline und führenden Spaces, wie z.B in diesem Text: [Beispiel unten unter „Eingabe“]
> Nach Format sollte der Text so aussehen: [Beispiel unten unter „Erwartete Ausgabe“]
> Wichtig ist, dass die intensional umgebrpchenen Zeilen (Überschriften und Bulletpoints) umgebrochen bleiben.
>
> Plan das und setzte beides autonom um

Die Quicknote ist die F10-Fläche aus dem Arbeitspaket `260926-2246-f10-oeffnet-quicknote-mit-fluechtigem-puffer` (Spec `260926-2300_*_spec-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md`); ihr Puffer war bisher flüchtig. Die „F2 Dateien“ sind die Sonderdateien des krk-Heimordners neben `notes.txt`, `tasks.txt` und `secrets.txt` (`krk-core/src/heimordner/mod.rs`, `Sonderdatei`).

Erreicht ist das Ziel, wenn

1. jede Änderung am Inhalt der Quicknote ohne Zutun des Nutzers in `scratch.txt` im krk-Heimordner landet, die Datei unter den F2-Dateien steht und der Inhalt nach einem Neustart von KRK wieder in der Quicknote steht;
2. die Quicknote eine Schaltfläche **Strip** trägt, die aus einem aus dem Chat kopierten Shell-Befehl die überflüssigen Leerzeichen und Zeilenbrüche nimmt, sodass er im Terminal läuft;
3. die Quicknote eine Schaltfläche **Format** trägt, die aus der Eingabe unten die erwartete Ausgabe macht: Fließtext-Zeilen, die nur der Terminalumbruch getrennt hat, werden zu einer Zeile verbunden, die führende Einrückung fällt, und Überschriften und Aufzählungspunkte beginnen weiter auf eigener Zeile.

Mode: autonom auf Wunsch des Nutzers („setzte beides autonom um“).

## Beispiel des Nutzers

Eingabe (so kommt der Text aus dem Chat; zwei Leerzeichen Einrückung je Zeile außer der ersten, Fortsetzungszeilen von Aufzählungspunkten mit vier):

```
Von den 202 registrierten Frameworks sind alle 202 strukturell vollständig, und 194 davon kann der Kernel heute auswählen. Acht schließt die Auswahl bewusst aus,
  weil sie ein Readiness-Tag tragen. Die eigentliche Lücke liegt nicht mehr in den Daten, sondern in der Darstellung: Nur 3 der 104 Canvases werden in ihrer
  Originalgeometrie gezeichnet.

  Was vollständig ist (alle 202)
  - Jedes Framework hat ein Manifest mit Shape, Depth, Topics, Phasen, Phasen- und Workstream-Levels, Produces und Feldern.
  - Alle 2.304 Felder haben einen Typ, ein englisches Label und einen Ausfüll-Hinweis für das Modell. Kein Select-Feld ist ohne Optionen.
  - Jede englische Textdatei hat Name, Beschreibung, Abgrenzung, KI-Hinweise, Outputs und eine Client-Match-Surface (der Text, über den die Auswahl ein Framework zur
    Kundenlage findet; nachgetragen in c3cabfcb5).
  - Alle zwölf Ontologie-Validatoren enden mit Exit-Code 0, auch die Prüfung der Zählungen gegen ontology/uif-stats.yaml.

  Nicht auswählbar (8), und warum
  Die Auswahl verwirft jedes Framework, dessen Readiness-Tag weder fehlt noch ready-for-client-use lautet (codebase/go/pkg/strategy/readiness.go:67):
  - work-in-progress (6): scope-landscape (Stefan braucht eine Whiteboard-Engine), digital-maturity-assessment (nur der Quick Scan wird gebaut), 360-feedback und
    decision-making-options (beide nach deinen Rulings zurückgestellt), cost-of-change (laut Meta Model V23 WIP) und strategy-self-assessment (der Fragebogen fehlt).
  - workshop-ready (2): dlsf-7-step und value-driven-roadmapping. Laut Meta Model V23 sind sie workshop-tauglich, die Auswahl verwirft sie aber genauso.
  - Folge: Der Project Type h2-business-known listet dlsf-7-step und scope-landscape als must. Dort fehlen zur Laufzeit also zwei Pflicht-Frameworks. Das meldet der
    Validator als Warnung.

  Schwächen, die die Nutzung nicht blockieren
  - Canvas-Geometrie: Ein eigenes Layout haben nur BMC, Extended BMC und SWOT. Die übrigen 101 Canvases werden generisch gerendert.
  - Herkunft: 121 Manifeste stammen aus dem generierten UFDS-Import, 81 sind aus Quellen erstellt. Laut Commit 263a0989e wurden die Texte der UFDS-Frameworks gegen ihre
    Quellen abgeglichen. Ob auch die Feldstrukturen der 121 fachlich stimmen, habe ich nicht gemessen.
  - Kleinere Datenpunkte: 55 Canvases haben keine Aliase (Warnung). 81 Warnungen melden typisierte Entitäten, die im Freitext der Inputs/Outputs nicht vorkommen. 7
    Assessments haben Zahlen statt Text als Select-Optionen (offenes Issue 260917-1102). 26 Frameworks haben keine Feldgruppen. Bei 2 fehlt die Altitude, bei 4 die
    Consumes-Liste; beides ist laut Schema zulässig.
  - Sprache: Framework-Texte gibt es nur auf Englisch (ontology/i18n/en/). Eine deutsche Fassung existiert nicht.
```

Erwartete Ausgabe nach Format (so vom Nutzer angegeben, Aufzählungspunkte mit einem führenden Leerzeichen):

```
Von den 202 registrierten Frameworks sind alle 202 strukturell vollständig, und 194 davon kann der Kernel heute auswählen. Acht schließt die Auswahl bewusst aus, weil sie ein Readiness-Tag tragen. Die eigentliche Lücke liegt nicht mehr in den Daten, sondern in der Darstellung: Nur 3 der 104 Canvases werden in ihrer Originalgeometrie gezeichnet.

Was vollständig ist (alle 202)
 - Jedes Framework hat ein Manifest mit Shape, Depth, Topics, Phasen, Phasen- und Workstream-Levels, Produces und Feldern.
 - Alle 2.304 Felder haben einen Typ, ein englisches Label und einen Ausfüll-Hinweis für das Modell. Kein Select-Feld ist ohne Optionen.
 - Jede englische Textdatei hat Name, Beschreibung, Abgrenzung, KI-Hinweise, Outputs und eine Client-Match-Surface (der Text, über den die Auswahl ein Framework zur Kundenlage findet; nachgetragen in c3cabfcb5).
 - Alle zwölf Ontologie-Validatoren enden mit Exit-Code 0, auch die Prüfung der Zählungen gegen ontology/uif-stats.yaml.

Nicht auswählbar (8), und warum
Die Auswahl verwirft jedes Framework, dessen Readiness-Tag weder fehlt noch ready-for-client-use lautet (codebase/go/pkg/strategy/readiness.go:67):
 - work-in-progress (6): scope-landscape (Stefan braucht eine Whiteboard-Engine), digital-maturity-assessment (nur der Quick Scan wird gebaut), 360-feedback und decision-making-options (beide nach deinen Rulings zurückgestellt), cost-of-change (laut Meta Model V23 WIP) und strategy-self-assessment (der Fragebogen fehlt).
 - workshop-ready (2): dlsf-7-step und value-driven-roadmapping. Laut Meta Model V23 sind sie workshop-tauglich, die Auswahl verwirft sie aber genauso.
 - Folge: Der Project Type h2-business-known listet dlsf-7-step und scope-landscape als must. Dort fehlen zur Laufzeit also zwei Pflicht-Frameworks. Das meldet der Validator als Warnung.

Schwächen, die die Nutzung nicht blockieren
 - Canvas-Geometrie: Ein eigenes Layout haben nur BMC, Extended BMC und SWOT. Die übrigen 101 Canvases werden generisch gerendert.
 - Herkunft: 121 Manifeste stammen aus dem generierten UFDS-Import, 81 sind aus Quellen erstellt. Laut Commit 263a0989e wurden die Texte der UFDS-Frameworks gegen ihre Quellen abgeglichen. Ob auch die Feldstrukturen der 121 fachlich stimmen, habe ich nicht gemessen.
 - Kleinere Datenpunkte: 55 Canvases haben keine Aliase (Warnung). 81 Warnungen melden typisierte Entitäten, die im Freitext der Inputs/Outputs nicht vorkommen. 7 Assessments haben Zahlen statt Text als Select-Optionen (offenes Issue 260917-1102). 26 Frameworks haben keine Feldgruppen. Bei 2 fehlt die Altitude, bei 4 die Consumes-Liste; beides ist laut Schema zulässig.
 - Sprache: Framework-Texte gibt es nur auf Englisch (ontology/i18n/en/). Eine deutsche Fassung existiert nicht.
```
