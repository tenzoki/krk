# Shaper, anticipated-circle: der Dateilistenfilter nimmt Eingaben per Cmd+V

**Datum:** 2026-08-28, 09:24 bis 10:41
**Filed by:** shaper (anticipated-circle mode), Kai Stalmann <kai@stalmann.org>
**Modus:** anticipated-circle, dispatched über `/fusion:direct`, Domain code
**Entwurf:** `shared/backlog/260828-0909_*_dateilistenfilter-nimmt-eingaben-per-paste.md` (ein Absatz: der Filter soll außer einzelnen Zeichen auch Eingaben per Paste annehmen)
**Status:** Complete

## Klärungsrunden 1 und 2 (260828-0924, 260828-0933) und Antworten

1. Dateiverweise in der Ablage, was tut `cmd+v` im Dateifenster? → „Sieht aus wie Name, Datei, Pfad: einfügen."
2. Zeichen, die kein Dateiname trägt? → mehrzeilige Inhalte ablehnen; `\n`, `\t`, `/`, `:` entfernen.
3. Stehender Filtertext: anhängen oder ersetzen? → a) anhängen, wie ein getipptes Zeichen.
4. Was wird bei einem Pfad zum Filtertext? → „1, nur dateiname": der letzte Pfadbestandteil, für Finder-Verweis und Pfadtext gleich.

Dritter Dispatch am 260828-1041 mit allen Antworten und der Weisung, keine weitere Runde zu fahren.

## Festlegungen des Shapers, überstimmbar, im Grounding festgehalten

- Der Doppelpunkt fällt allein beim Einfügen; die Tipp-Regel `traegt_ein_dateiname` bleibt unverändert.
- Zeilenenden am Textende fallen weg, ein inneres Zeilenende macht den Text mehrzeilig und lehnt das Einfügen ab; `\r\n` zählt wie `\n`.
- Mehrere Dateiverweise auf einmal werden wie mehrzeiliger Text abgelehnt; ein einzelner wird eingefügt.
- Ein `http:`-Link wird wie ein Pfad gelesen, ohne eigenen Zweig.
- Ein Einfügen, das nichts einfügt, meldet sich in der Statuszeile nach dem Muster des Zwischenablagesprungs.

## Abgrenzung

Die Runde beantwortet `paste:` am Dateifenster und besetzt damit den seit dem 260805 reservierten Einhängepunkt der Dateizwischenablage, ohne eine zu bauen: `copy:` bleibt unbeantwortet, keine Datei wird durch Einfügen bewegt. Die Folge für eine spätere Dateizwischenablage ist als offener Datensatz gefilet: `circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/decisions/260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`.

## Ergebnis

- Circle `circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/` mit `_a_circle.md`, einem Entscheidungsdatensatz und den sechs Unterordnern.
- Backlogeintrag auf `_c_` gesetzt, `Promoted:`-Zeile angehängt.
- Kein Spec, keine Aktivierung, kein Commit: die Aktivierung ist `/fusion:next`.
