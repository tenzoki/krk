# Gilt die Quelltextzusage auch für das Ziehen einer Auswahl und für die Dienste des Systems?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `shared/decisions/260819-2216_*_was-landet-beim-gerenderten-markdown-in-der-zwischenablage.md`; `shared/decisions/260819-2216_*_welches-kontextmenue-zeigt-die-auswaehlbare-vorschau.md`; `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md` (C2.12); `crates/krk-ui/src/appkit/zwischenablage.rs` (die eine Hülle um `NSPasteboard`)

---

## Question

Eine auswählbare Textansicht gibt ihren markierten Text auf mehr als einem Weg heraus. Neben `cmd+c` und dem Menüeintrag „Kopieren" stehen das **Ziehen der Auswahl mit der Maus** in ein anderes Programm und die **Dienste des Systems**, die im Kontextmenü unter „Dienste" stehen und mit dem markierten Text arbeiten. Der Nutzer hat für die Zwischenablage entschieden, dass bei gerendertem Markdown der Quelltext herausgeht. Zu entscheiden ist, ob dieselbe Zusage für die übrigen Wege gilt.

Die Frage ist nicht bloß Vollständigkeit. Fällt die Antwort auf „nur die Zwischenablage", dann gibt dieselbe Fläche denselben Text auf zwei Wegen verschieden heraus, und der Nutzer sieht dem Weg nicht an, welchen er gerade nimmt.

## Options

1. **Eine Stelle für alle Ausgabewege.** Was die Vorschau aus einer Auswahl herausgibt, entsteht an genau einer Stelle, und jeder Weg fragt sie.
   - Folge: der Nutzer bekommt überall dasselbe. Beim Ziehen und bei einem Dienst gilt dieselbe Zusage wie beim Kopieren.
   - Preis: die Stelle muss so tief liegen, dass alle Wege durch sie gehen. In AppKit ist das die Methode, mit der eine Textansicht ihre Auswahl auf eine Ablage schreibt; ob sie wirklich alle Wege trägt, ist am Bündel nachzusehen und nicht aus der Dokumentation zu erschließen.

2. **Nur die Zwischenablage.** Kopieren gibt den Quelltext heraus, Ziehen und Dienste geben den gerenderten Text.
   - Folge: der Eingriff bleibt auf den Kopierweg beschränkt und ist mit Sicherheit dort, wo er sein soll.
   - Preis: zwei Antworten auf eine Frage, und der Unterschied ist am Bündel nicht zu erkennen. Wer den Text mit der Maus in einen Editor zieht, verliert die Auszeichnungen, die er beim Kopieren bekommen hätte.

3. **Das Ziehen abschalten.** Die Auswahl lässt sich nicht aus der Vorschau ziehen; Dienste bleiben, wie AppKit sie liefert.
   - Folge: ein Weg weniger, über den etwas Falsches herausgehen kann.
   - Preis: eine Bedienung, die der Nutzer von einer Textansicht kennt, fehlt ohne sichtbaren Grund. Für die Dienste löst die Möglichkeit nichts.

## Constraints

- Es entsteht keine zweite Hülle um `NSPasteboard`.
- Für die fünf übrigen Inhalte der Vorschau, in denen Anzeige und Quelle dasselbe sind, ist die Frage gegenstandslos: dort gibt jeder Weg dieselben Zeichen heraus.

## Recommendation

**Wir empfehlen Möglichkeit 1**, mit einem Vorbehalt: ob eine einzige Stelle wirklich alle Wege trägt, ist am laufenden Bündel zu prüfen. Trägt sie es nicht, ist Möglichkeit 2 die ehrlichere Antwort, und dann gehört der Unterschied in die Abnahmekriterien und nicht in eine Fußnote.

## Antwort 260819-2242

**Möglichkeit a.** Eine Stelle für alle Ausgabewege.

Zwischenablage, Ziehen mit der Maus und die System-Dienste liefern denselben Quelltext. Eine Regel, ein Ort im Code. Der Vorbehalt gehört dazu: ob eine Stelle wirklich alle Wege trägt, ist am gebauten Bündel zu prüfen und nicht an einer Probe.

---
Answered: dieser Datensatz, Abschnitt `## Antwort` — Klärungsrunden des Orchestrators mit dem Nutzer am 260819; Sitzungsprotokoll `shared/history/260819-2026-orchestrator-session.md`. Ausformuliert im Spec `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md`.
Implemented:
Deferred:
Superseded by:
