# CLAUDE.md angelegt mit Sprachdeklaration

**Agent:** coder
**Zeitstempel:** 260802-1030
**Status:** Complete
**Auslöser:** `shared/issues/260802-0842_o_projektsprache-nicht-deklariert.md`

## Ablage dieses Protokolls

Diese Datei liegt in `shared/history/` und nicht in der Historie des aktiven Circles, obwohl `bin/fusion-paths coder` `OUT_HISTORY` auf `circles/260802-0842-krk-mac-dateimanager-editor-git/history` auflöst. Grund ist die Herkunftsregel: die Arbeit entstand nicht aus der Directive des Circles, sondern aus einem Befund, der daneben gemacht wurde. Der auslösende Issue liegt aus demselben Grund im gemeinsamen Speicher. Sollte der Orchestrator die Ablage anders sehen, ist die Datei ohne Folgeschaden zu verschieben.

## Was gemacht wurde

`/Users/k1/Projects/productive/krk/CLAUDE.md` neu angelegt. Die Datei existierte vorher nicht. Inhalt:

- Die Zeile `**Language:** de` im Format aus `rules/fusion-workbench-conventions.md` `## Project language`.
- Kurzbeschreibung von KRK als nativer macOS-Anwendung zum Navigieren, Bearbeiten und Versionieren lokaler Dateien, mit Verweis auf die vollständige Directive im Circle-Datensatz.
- Die drei Maximen aus `idea.txt`: superschnell, supersimpel, Tastatursteuerung mit zusätzlicher Maus- und Trackpad-Unterstützung.
- Projektstand: kein Quellcode, keine Architektur, kein Build- und kein Testkommando. Die Ordnerübersicht listet nur, was auf der Platte liegt.
- Technologiewahl ausdrücklich offen, Festlegung über einen Entscheidungsdatensatz nach dem Vergleich durch den analyst.
- Die fünf offenen Entscheidungen unter `shared/decisions/` als bindende Grundlage, mit dem Hinweis, dass vier davon den aktiven Circle binden und der fünfte, das Code-SDK, seine eigene Nichtbindung festhält.

Keine weitere Datei geändert, kein Commit gesetzt.

## Verifikation

Vor der Änderung gab `bin/fusion-rules coder` als Stilprofil `./fusion-workbench/stilwerk/chat-voice-en.yaml` aus. Nach der Änderung:

- `bin/fusion-rules coder` → `./fusion-workbench/stilwerk/chat-voice-de.yaml`
- `bin/fusion-rules shaper` → `chat-voice-de.yaml` und `default-voice-de.yaml`

Damit ist der Befund des Issues behoben. Alle Aussagen über den Projektstand in `CLAUDE.md` wurden gegen `git ls-files` (leer, kein Commit im Repository) und eine Dateisuche über das Wurzelverzeichnis geprüft.

## Offen

Der Issue `shared/issues/260802-0842_o_projektsprache-nicht-deklariert.md` bleibt offen. Der Abschluss liegt beim Orchestrator, nicht bei diesem Lauf.
