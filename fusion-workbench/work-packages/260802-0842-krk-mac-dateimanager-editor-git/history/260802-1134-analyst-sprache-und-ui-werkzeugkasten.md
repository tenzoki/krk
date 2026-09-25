# Sitzung: Vergleich von Sprache und UI-Werkzeugkasten für KRK

**Datum:** 2026-08-02 11:34
**Agent:** analyst
**Status:** Complete
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Auftrag:** Vergleichende Analyse der Kandidaten für Programmiersprache und UI-Werkzeugkasten, gemessen an C3 und C8 des Aktivierungs-Specs, auf dem vom Nutzer neu benannten Intel-Referenzgerät.

## Was gemacht wurde

Gelesen wurden `CLAUDE.md`, `idea.txt`, der vollständige Spec `260802-1036_o_spec-navigator-geruest.md`, der Circle-Datensatz `_t_circle.md`, der Leistungs-Entscheidungsdatensatz `260802-1036_o_leistungszusagen-navigator.md` sowie die beantwortete F-Tasten-Entscheidung `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`. Die Historie und die offenen Defekte des Circles wurden gesichtet.

Die Recherche lief über Websuche und direktes Abrufen der Quellen. Ein Versuch, sie auf parallele Unteragenten zu verteilen, schlug fehl: der Agententyp `general-purpose` steht in diesem Projekt nicht zur Verfügung, die Fusion-Agenten sind es alle. Die Recherche wurde daraufhin direkt geführt.

Verglichen wurden drei Kandidaten (Swift mit AppKit, Swift mit SwiftUI einschließlich Mischform, Rust mit AppKit über `objc2`) auf acht Achsen. Sechs weitere Kandidaten wurden mit Begründung ausgeschlossen: Electron, Tauri, Qt, Flutter, Objective-C und GPUI.

## Ergebnis

**Empfohlen wird Swift mit AppKit**, mit zwei ausdrücklich benannten Bedingungen, unter denen die Empfehlung kippen würde.

Die Empfehlung entscheidet sich an der Zusage L3, also 10.000 Einträge in 400 ms. Belege dafür, dass SwiftUI diese Zusage auf macOS nicht trägt, stammen aus vier unabhängigen Quellen, darunter zwei Fäden im Apple-Entwicklerforum. Der schärfste Einzelbeleg ist ein Bericht über 13 Sekunden Hängung bei rund 1.000 Zeilen einer SwiftUI-`Table` auf einem Mac Studio mit M2 Max, also auf erheblich schnellerer Hardware als das Referenzgerät und bei einem Zehntel der zugesagten Einträge.

## Neue Befunde, die den Plan binden

Drei Ergebnisse gehen über die eigentliche Werkzeugfrage hinaus und sind im Entscheidungsdatensatz unter `## Constraints` festgehalten:

1. **KRK muss außerhalb der App-Sandbox ausgeliefert werden.** C9 verlangt Zugriff auf jeden lokalen Pfad einschließlich `/Volumes`; in der Sandbox gibt es für den Schreibtisch keine passende Berechtigung.
2. **Das Referenzgerät erhält macOS 26 Tahoe nicht.** Tahoe unterstützt nur vier Intel-Modelle, und `MacBookPro15,1` von 2018 ist keines davon. Das minimale Zielsystem von KRK liegt damit dauerhaft bei macOS 15, solange dieses Gerät die Abnahme trägt.
3. **Für Swift und Rust gibt es kein offizielles Anthropic-SDK.** Beide sprechen die Programmierschnittstelle über rohes HTTP an. Das Claude Agent SDK existiert nur für Python und TypeScript. Der Punkt betrifft den offenen Datensatz zur KI-Anbindung, nicht diesen Circle, und differenziert die Kandidaten nicht.

## Was ungeprüft blieb

- Die Annahme aus C3, dass Fn+F3 bis Fn+F8 als gewöhnliche Tastenereignisse ankommen, ließ sich nicht belegen. Sie ist werkzeugunabhängig und muss vor der ersten Implementierung an einem Zehnzeiler geprüft werden.
- Zu den Zusagen L1 und L4 existiert für keinen Kandidaten eine veröffentlichte Vergleichsmessung auf einem Intel-Mac. Die Aussagen der Analyse zu diesen beiden sind als Schlussfolgerung aus Mechanismen gekennzeichnet, nicht als Messung.

## Erzeugte Artefakte

| Pfad | Was |
|---|---|
| `circles/260802-0842-krk-mac-dateimanager-editor-git/analyses/260802-1134-sprache-und-ui-werkzeugkasten.md` | Der Analysebericht mit neun Befunden, zwei Mermaid-Diagrammen und vollständiger Quellenliste |
| `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1134_o_sprache-und-ui-werkzeugkasten.md` | Der offene Entscheidungsdatensatz mit drei Möglichkeiten, sechs Randbedingungen und Empfehlung |
| `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1134-analyst-sprache-und-ui-werkzeugkasten.md` | Dieser Eintrag |

Es wurde kein Defekt angelegt: die Analyse hat keinen Fehler in bestehenden Dokumenten gefunden.

## Empfohlener nächster Schritt

Der Nutzer beantwortet den Entscheidungsdatensatz. Danach kann der Planner den Implementierungsplan für Runde 1 schreiben; die Punkte unter `## Offen für den Planner` im Spec sind damit bis auf die Messautomatisierung entscheidbar.

Nicht committet, wie beauftragt. Der Orchestrator committet.
