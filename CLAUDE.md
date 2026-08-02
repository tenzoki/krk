# KRK

**Language:** de

## Worum es geht

KRK ist eine native macOS-Anwendung zum Navigieren, Bearbeiten und Versionieren lokaler Dateien, in der Tradition von ForkLift und Norton Commander. Die Oberfläche besteht aus einer Lesezeichen- und Geräteleiste links, zwei Dateifenstern mit je mehreren Tabs in der Mitte und einem Vorschaufenster mit eigenen Tabs rechts. Der Editor öffnet Text, Code und Markdown in einer Rohansicht und einer Formatansicht. Git ist eingebaut, beschränkt auf hinzufügen, committen, verwerfen sowie ältere Versionen ansehen und auschecken.

Die vollständige Directive steht im Circle-Datensatz `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md`, Abschnitt `## Directive`. Dieser Abschnitt hier ist die Kurzfassung, nicht die verbindliche Formulierung.

## Maximen

Aus `idea.txt`:

- Superschnell
- Supersimpel
- Steuerung über die Tastatur, ergänzt um Maus- und Trackpad-Unterstützung

Die Maxime "superschnell" trägt in dieser Form noch keine Abnahmekriterien. Der Aktivierungs-Spec muss sie in messbare Zusagen übersetzen.

## Projektstand

Es gibt noch keinen Quellcode und keine Architektur. Im Projektwurzelverzeichnis liegen:

```
krk/
├── CLAUDE.md            # diese Datei
├── idea.txt             # der ursprüngliche Entwurf, Quelle der Directive
├── .gitignore
├── .claude/             # lokale Claude-Code-Einstellungen (nicht versioniert)
└── fusion-workbench/    # Circles, Entscheidungen, Issues, Historie
```

Kein Build-Verzeichnis, keine Projektdatei, keine Abhängigkeitsdeklaration, keine Tests. Es gibt daher auch **kein Build-Kommando und kein Testkommando** — beides entsteht erst mit der ersten Implementierung und wird dann hier nachgetragen.

## Technologiewahl

Offen. Sprache, UI-Toolkit und Git-Anbindung sind nicht festgelegt. Die Festlegung erfolgt über einen Entscheidungsdatensatz unter `fusion-workbench/shared/decisions/`, sobald der analyst die Kandidaten verglichen hat. Bis dahin gilt keine Vorfestlegung, auch keine implizite: kein Agent wählt ein Toolkit nebenbei im Zuge einer anderen Aufgabe.

## Bindende Grundlage: fünf offene Entscheidungen

Diese fünf Entscheidungsdatensätze sind gestellt und noch nicht beantwortet. Sie sind die aktuell bindende Grundlage für jede Planung und jede Implementierung. Alle liegen unter `fusion-workbench/shared/decisions/`:

| Datei | Frage |
|---|---|
| `260802-0842_o_f-tasten-unter-macos-systembelegung.md` | Wie erreicht KRK die Tasten F3 bis F8, die macOS ab Werk selbst belegt? |
| `260802-0842_o_loeschen-papierkorb-oder-endgueltig.md` | Löscht Shift+Delete in den Papierkorb oder endgültig, und fragt KRK vorher nach? |
| `260802-0842_o_git-verwerfen-bedeutung.md` | Bedeutet "revert" aus dem Entwurf: Änderungen der Datei verwerfen oder einen Commit zurücknehmen? |
| `260802-0842_o_editor-formatansicht-je-dateityp.md` | Was zeigt die Formatansicht bei Text, bei Code und bei Markdown? |
| `260802-0842_o_code-sdk-fuer-ki-integration.md` | Welches Code-SDK trägt die spätere KI-Anbindung? |

Die ersten vier binden den aktiven Circle. Der fünfte, das Code-SDK, hält seine eigene Nichtbindung fest: die KI-Anbindung liegt ausdrücklich außerhalb des aktiven Circles.

Ausdrücklich außerhalb des aktiven Circles liegen außerdem: integrierter Browser, Datei- und Ordnervergleich, Suchen und Ersetzen über mehrere Dateien, Zugriff über Server-Protokolle sowie Git jenseits der oben genannten vier Operationen. Die Abgrenzung im Einzelnen steht im Circle-Datensatz.

## Sprache

Die Zeile `**Language:** de` oben deklariert Deutsch als Projektsprache. Sie steuert, welche Stilprofile unter `fusion-workbench/stilwerk/` gelten: `bin/fusion-rules` gibt daraufhin `chat-voice-de.yaml` und, für Langform-Agenten, `default-voice-de.yaml` aus. Ohne die Zeile fällt die Auflösung still auf `en` zurück. Das Format ist in `rules/fusion-workbench-conventions.md`, Abschnitt `## Project language`, festgelegt — Zeile nicht umformulieren, nicht verschieben in einen anderen Abschnitt und nicht entfernen.

Prosa in diesem Projekt ist deutsch. Bezeichner im Code, Commit-Messages und maschinenlesbare Artefakte folgen den üblichen englischen Konventionen.
