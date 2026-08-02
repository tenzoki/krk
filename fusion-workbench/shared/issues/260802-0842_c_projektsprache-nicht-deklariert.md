Projekt deklariert keine Sprache, die deutschen Stilprofile bleiben ungenutzt
---
Im Projektwurzelverzeichnis `/Users/k1/Projects/productive/krk/` existiert kein `CLAUDE.md` und damit keine Zeile `**Language:** de`. Nach `rules/fusion-workbench-conventions.md` `## Project language` fällt die Auflösung deshalb still auf `en` zurück. `bin/fusion-rules shaper` hat entsprechend `fusion-workbench/stilwerk/chat-voice-en.yaml` und `fusion-workbench/stilwerk/default-voice-en.yaml` ausgegeben, obwohl im selben Verzeichnis `chat-voice-de.yaml` und `default-voice-de.yaml` liegen und sowohl der Nutzer als auch das Ausgangsmaterial `idea.txt` deutschsprachig sind.
---
Aufgefallen beim Setup der Shaper-Sitzung am 260802-0842, die den Circle `260802-0842-krk-mac-dateimanager-editor-git` angelegt hat. Der Shaper hat für diese Sitzung ersatzweise die deutschen Profile gelesen und angewendet, weil Dialog und Quellmaterial deutsch sind. Ohne die Deklaration wiederholt jeder folgende Agent denselben Fehlgriff und schreibt gegen das englische Profil.

Behebung: `CLAUDE.md` im Projektwurzelverzeichnis anlegen und die Zeile `**Language:** de` aufnehmen. Der Befund gehört nicht zur Directive des Circles, sondern wurde daneben gefunden, deshalb liegt er im gemeinsamen Speicher und nicht im Circle.

---
Resolved: `CLAUDE.md` angelegt mit `**Language:** de` in Zeile 3, dazu Kurzbeschreibung, Maximen, überprüfbarer Projektstand, offene Technologiewahl und die fünf offenen Entscheidungen. Auflösung nachgewiesen: `bin/fusion-rules orchestrator` gibt jetzt `chat-voice-de.yaml` und `default-voice-de.yaml` aus statt der englischen Varianten. Bearbeitet von coder, Protokoll `shared/history/260802-1030-claude-md-sprachdeklaration.md`.
