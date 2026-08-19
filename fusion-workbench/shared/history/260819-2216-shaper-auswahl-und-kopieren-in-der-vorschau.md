# Shaper — Auswahl und Kopieren in der Vorschau

**Date:** 2026-08-19
**Status:** Complete
**Agent:** shaper (dispatched, zweite Klärungsrunde)
**Baum:** `6be1e81`

## Auftrag

Der Orchestrator hat den Shaper am 260819-1843 auf das Thema „Auswahl und Kopieren in der Vorschau" angesetzt (Ereignis `shaper_start` in `orchestrator-events.jsonl`). Die erste Klärungsrunde hat vier Fragen an den Nutzer gegeben; dieser Lauf ist die zweite Runde und hat die Antworten entgegengenommen, den Spec geschrieben und die Datensätze abgelegt.

## Die vier Antworten des Nutzers vom 260819-2210

1. Auswählbar wird alles, was die Textfläche zeigt; Bilder nicht.
2. Beim gerenderten Markdown landet der **Quelltext mit Auszeichnungszeichen** in der Zwischenablage. Der Nutzer hat damit gegen die Empfehlung des Shapers entschieden und die Kosten ausdrücklich angenommen.
3. Pfeil hoch und Pfeil runter bleiben, wie sie sind.
4. Das Kontextmenü der Vorschau übernimmt AppKits Menü unverändert.

## Was geschrieben wurde

- **Spec:** `shared/planning/260819-2216_o_spec-auswahl-und-kopieren-in-der-vorschau.md`, vier Fähigkeiten mit 39 Abnahmekriterien, davon 14 mit Bündelanteil.
- **Sieben Entscheidungsdatensätze** unter `shared/decisions/`, alle mit dem Zeitstempel `260819-2216` und dem Marker `_o_`. Vier halten die Antworten des Nutzers fest, drei stellen neue Fragen.
- Kein Circle. Der Circle entsteht nach der Abnahme des Specs durch den Nutzer; der Shaper hat weder einen angelegt noch einen bestehenden angefasst.
- Der überholte Datensatz der Runde 6 ist **nicht** umbenannt worden. Der überholende Datensatz schreibt aus, welche Zeile der Orchestrator dort anzuhängen hat und dass allein die zweite der beiden Fragen jenes Dokuments überholt ist.

## Die drei offenen Fragen

1. **Welche Auszeichnungszeichen fahren an den Rändern einer Auswahl mit?** Die Abbildung vom gerenderten Text auf die Quelle ist an den Rändern nicht eindeutig, und diese Folge stand in der Optionsbeschreibung der ersten Runde nicht. Empfohlen: eine berührte Auszeichnung fährt ganz mit.
2. **Gilt die Quelltextzusage auch für das Ziehen einer Auswahl und für die Dienste?** Empfohlen: eine Stelle für alle Ausgabewege, mit einem Vorbehalt, der am Bündel zu prüfen ist.
3. **Schuldet diese Runde einen Abnahmelauf gegen L7?** Empfohlen: kein Lauf in dieser Runde, L7 als Gegenstand der späteren Messrunde.

## Was am Baum erhoben wurde

Neun Feststellungen stehen im Spec unter „Ausgangslage". Vier davon widersprechen dem, was man ohne sie annähme, und zwei tragen den Zuschnitt der Runde:

- **`Fokus::Vorschau` bleibt die Antwort**, auch wenn der Ersthelferrang in der Textanzeige steht, weil `bereich_des_ersthelfers` über `isDescendantOf:` fragt. Der Fokusrahmen und der Fenstertitel brauchen keine Zeile.
- **Kopieren ist in diesem Baum kein Befehl von KRK.** `text_kopieren` und `text_alles_auswaehlen` sind vom Menü gehalten, `Belegung::nachschlag` überspringt sie, und `validateMenuItem:` antwortet für fremde Aktionen `true`. Die Runde braucht deshalb keinen Belegungseintrag und keine `Kommando`-Variante.
- Nebenbefund, der in kein Kriterium gehört, aber in einen Datensatz: **Bild-auf, Bild-ab, Pos1 und Ende blättern nach dieser Runde in der Vorschau**, weil sie mit dem Fokus dort unzulässig sind und an AppKit weiterlaufen. Die Pfeiltasten bleiben wirkungslos. Die Ungleichheit ist im Datensatz zu den Pfeiltasten benannt.

## Voice-Profile

`fusion-rules` hat `chat-voice-de.yaml` und `default-voice-de.yaml` ausgegeben. Die Artefaktsprache ist seit dem Commit `6be1e81` nicht mehr eigens deklariert, und `**Language:** de` steuert damit beide Flächen.
