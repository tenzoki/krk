# Shaper, anticipated-circle: Cmd+C und Cmd+X in der Dateiliste legen Dateiverweise ab

**Datum:** 2026-08-28, 23:45 bis 23:55
**Filed by:** shaper (anticipated-circle mode), Kai Stalmann <kai@stalmann.org>
**Modus:** anticipated-circle, dispatched über `/fusion:direct`, Domain code, ohne Klärungsrunde auf Weisung des Nutzers („autonom fertigstellen")
**Entwurf:** `shared/backlog/260828-2345_*_cmd-c-und-cmd-x-kopieren-dateien-fuer-andere-apps.md` (ein Absatz: `cmd+c`/`cmd+x` kopieren bzw. schneiden die Markierung oder den Eintrag unter der Zeilenmarke für andere Apps aus)
**Status:** Complete

## Klärungsrunden

Keine. Der Baum entscheidet jede Frage bis auf eine (was `cmd+x` tut); die ist im Grounding mit beiden Lesarten ausgeschrieben und nach dem Baum entschieden, überstimmbar am Spec-Tor.

## Festlegungen des Shapers, überstimmbar, im Grounding festgehalten

- `copy:` und `cut:` werden am Dateifenster über die Antwortkette beantwortet; kein `Kommando`, keine Belegungszeile, kein Menüeintrag (Reservierung vom 260805).
- Betroffen ist `betroffene()`: Markierung vor Zeilenmarke, Ordner wie Dateien, Sichtreihenfolge; leerer Ordner meldet in der Statuszeile wie der Pfadkopierer.
- Abgelegt werden Datei-`NSURL` über die eine Hülle `zwischenablage.rs`; daneben die Namen als Text, ein Name je Zeile (Finder-Konvention). Der Entscheid vom 260811 (nur Text) gilt den Pfadkopierern weiter.
- `cmd+x` legt dieselben Verweise ab und meldet in der Statuszeile, dass das Verschieben beim Ziel liegt; kein Abblenden der Zeilen (kein dritter Zellenzustand, Runde 11).
- Die Zählprobe zu `copy:` in `betrachter.rs` zieht nach; das Kontextmenü bekommt keinen fünften Eintrag; `paste:` bleibt beim Filter-Circle `260828-1041`.

## Ergebnis

Circle `circles/260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab/` mit `_a_circle.md` und sechs Unterordnern. Backlog-Eintrag auf `_c_` gesetzt mit `Promoted:`-Zeile. Nichts committet.
