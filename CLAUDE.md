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

Die Maxime "superschnell" trägt in dieser Form keine Abnahmekriterien. Der Spec der ersten Runde übersetzt sie in Abschnitt `### C8: Messbare Geschwindigkeit` in Zeitzusagen: `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`. Das Referenzgerät, auf dem diese Zusagen gemessen werden, ist im Datensatz `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_a_leistungszusagen-navigator.md` festgehalten.

## Projektstand

Geprüft am 260802-1130. Es gibt weiterhin keinen Quellcode und keine Architektur. Im Projektwurzelverzeichnis liegen:

```
krk/
├── CLAUDE.md            # diese Datei
├── idea.txt             # der ursprüngliche Entwurf, Quelle der Directive
├── .gitignore
├── .claude/             # lokale Claude-Code-Einstellungen (nicht versioniert)
└── fusion-workbench/    # Circles, Entscheidungen, Issues, Historie
```

Kein Build-Verzeichnis, keine Projektdatei, keine Abhängigkeitsdeklaration, keine Tests. Es gibt daher auch **kein Build-Kommando und kein Testkommando** — beides entsteht erst mit der ersten Implementierung und wird dann hier nachgetragen.

Was inzwischen vorliegt, sind zwei Dokumente im aktiven Circle:

| Datei | Was drinsteht |
|---|---|
| `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` | Spec der ersten Runde (Navigator-Gerüst), Fähigkeiten C1 bis C9. Entwurf, noch nicht abgenommen. |
| `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260802-1118-conceptrev-spec-navigator-geruest.md` | Konzeptprüfung der Diagramme dieses Specs, Verdikt "acceptable". |

Analysen gibt es noch keine: `fusion-workbench/shared/analyses/` und `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/analyses/` sind beide leer. Der Technologievergleich, der unten als Voraussetzung genannt ist, hat also noch nicht stattgefunden.

## Technologiewahl

Offen. Sprache, UI-Toolkit und Git-Anbindung sind nicht festgelegt. Die Festlegung erfolgt über einen Entscheidungsdatensatz unter `fusion-workbench/shared/decisions/`, sobald der analyst die Kandidaten verglichen hat. Bis dahin gilt keine Vorfestlegung, auch keine implizite: kein Agent wählt ein Toolkit nebenbei im Zuge einer anderen Aufgabe.

## Bindende Grundlage: die Entscheidungsdatensätze

Die Entscheidungsdatensätze sind die bindende Grundlage für jede Planung und jede Implementierung. **Verbindlich ist der Dateibestand, nicht diese Tabelle.** Den Stand trägt der Marker im Dateinamen: `_o_` offen, `_a_` beantwortet, `_i_` umgesetzt, `_d_` zurückgestellt, `_s_` überholt. Wer den aktuellen Stand braucht, listet beide Speicher auf, nicht nur einen:

- `fusion-workbench/shared/decisions/` — projektweite Fragen
- `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/` — Fragen des aktiven Circles

Die folgende Aufstellung gibt den am 260802-1130 geprüften Stand wieder. Weicht sie vom Dateibestand ab, gilt der Dateibestand.

**Beantwortet.** Die Antwort selbst steht im jeweiligen Datensatz in der Zeile `Answered:` und ausformuliert im Spec. Sie wird hier nicht wiederholt, damit sie nicht an zwei Stellen auseinanderläuft.

| Datei | Frage |
|---|---|
| `fusion-workbench/shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md` | Wie erreicht KRK die Tasten F3 bis F8, die macOS ab Werk selbst belegt? |
| `fusion-workbench/shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md` | Löscht Shift+Delete in den Papierkorb oder endgültig, und fragt KRK vorher nach? |
| `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_a_umbenennen-im-stapel-umfang.md` | Wie weit reicht "im Stapel umbenennen" in der ersten Runde? |
| `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_a_leistungszusagen-navigator.md` | Auf welchem Referenzgerät gelten die zehn Zeitzusagen aus Abschnitt C8 des Specs? |

**Offen.** Diese Fragen sind gestellt und noch nicht beantwortet:

| Datei | Frage |
|---|---|
| `fusion-workbench/shared/decisions/260802-0842_o_git-verwerfen-bedeutung.md` | Bedeutet "revert" aus dem Entwurf: Änderungen der Datei verwerfen oder einen Commit zurücknehmen? |
| `fusion-workbench/shared/decisions/260802-0842_o_editor-formatansicht-je-dateityp.md` | Was zeigt die Formatansicht bei Text, bei Code und bei Markdown? |
| `fusion-workbench/shared/decisions/260802-0842_o_code-sdk-fuer-ki-integration.md` | Welches Code-SDK trägt die spätere KI-Anbindung? |

Die ersten beiden offenen Fragen binden den aktiven Circle. Die dritte, das Code-SDK, hält ihre eigene Nichtbindung fest: die KI-Anbindung liegt ausdrücklich außerhalb des aktiven Circles.

Eine beantwortete Entscheidung ist damit noch nicht erledigt: `_a_` heißt, die Antwort ist festgehalten, aber noch nicht in Code umgesetzt. Erst die Umsetzung zieht den Marker auf `_i_`.

Ausdrücklich außerhalb des aktiven Circles liegen außerdem: integrierter Browser, Datei- und Ordnervergleich, Suchen und Ersetzen über mehrere Dateien, Zugriff über Server-Protokolle sowie Git jenseits der oben genannten vier Operationen. Die Abgrenzung im Einzelnen steht im Circle-Datensatz.

## Sprache

Die Zeile `**Language:** de` oben deklariert Deutsch als Projektsprache. Sie steuert, welche Stilprofile unter `fusion-workbench/stilwerk/` gelten: `$FUSION_PLUGIN_ROOT/bin/fusion-rules` gibt daraufhin `fusion-workbench/stilwerk/chat-voice-de.yaml` und, für Langform-Agenten, `fusion-workbench/stilwerk/default-voice-de.yaml` aus. Ohne die Zeile fällt die Auflösung still auf `en` zurück. Das Format ist in `$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md`, Abschnitt `## Project language`, festgelegt — Zeile nicht umformulieren, nicht verschieben in einen anderen Abschnitt und nicht entfernen.

Prosa in diesem Projekt ist deutsch. Bezeichner im Code, Commit-Messages und maschinenlesbare Artefakte folgen den üblichen englischen Konventionen.
