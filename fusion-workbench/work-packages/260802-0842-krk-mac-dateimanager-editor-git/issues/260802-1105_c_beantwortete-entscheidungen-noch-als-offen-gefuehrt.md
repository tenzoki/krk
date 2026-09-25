Zwei Dokumente führen beantwortete Entscheidungen weiterhin als offen

---

Seit dem 260802-1105 tragen zwei Entscheidungsdatensätze den Marker `_a_` (beantwortet) und damit einen neuen Dateinamen:

- `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`
- `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`

Zwei Dokumente nennen sie weiterhin unter dem alten Namen mit dem Marker `_o_` (offen) und behaupten dazu ausdrücklich, sie seien unbeantwortet:

1. `CLAUDE.md`, Abschnitt `## Bindende Grundlage: fünf offene Entscheidungen`. Der Einleitungssatz lautet "Diese fünf Entscheidungsdatensätze sind gestellt und noch nicht beantwortet"; die Tabelle darunter führt beide Zeilen mit dem alten Pfad.
2. `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md`, Abschnitt `## Grounding snapshot`, Unterabschnitt "Offene Entscheidungen". Der Einleitungssatz lautet "Fünf Fragen sind gestellt und noch nicht beantwortet"; die Aufzählung führt beide mit dem alten Pfad.

Ein Agent, der eines der beiden Dokumente als Einstieg liest, hält vier Fragen für offen, von denen zwei entschieden sind, und folgt zwei Pfaden, die ins Leere zeigen.

---

**Was zu tun ist:** in beiden Dokumenten die Zahl auf drei offene Entscheidungen korrigieren, die beiden beantworteten Einträge als beantwortet kennzeichnen oder entfernen und die Pfade auf den Marker `_a_` ziehen. Die Antworten selbst gehören in keines der beiden Dokumente; sie stehen im Spec und in den beiden Datensätzen.

**Warum das nicht der Shaper behebt:** der Shaper schreibt Spec-Dokumente, Entscheidungsdatensätze, Defekte und seine Historie. `CLAUDE.md` liegt außerhalb. Den Circle-Datensatz darf er nur im portfolio-activation-Modus bearbeiten, und auch dort nur die Abschnitte `## Directive` und `## Grounding snapshot`; die laufende Runde ist eine in-Circle-Klärung.

**Nicht betroffen:** `fusion-workbench/portfolio.md` nennt dieselben alten Pfade, wird aber vom Playmaker bei jedem Lauf vollständig neu erzeugt und heilt sich damit selbst. Die Historiendateien nennen sie ebenfalls; sie halten den Stand ihres Zeitpunkts fest und werden nicht nachträglich umgeschrieben.

**Verursacht durch:** die Klärungsrunde vom 260802-1105 im Circle `260802-0842-krk-mac-dateimanager-editor-git`, festgehalten in `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1036-shaper-spec-navigator-geruest.md`.

---

## Teil 1 erledigt am 260802-1127: der Circle-Datensatz

Der Shaper hat den Unterabschnitt "Offene Entscheidungen" im Abschnitt `## Grounding snapshot` von `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md` neu gefasst. Der Orchestrator hat den Zugriff auf den Datensatz für diese Runde ausdrücklich freigegeben; die oben notierte Modus-Grenze galt für den regulären in-Circle-Klärungsmodus.

Der Unterabschnitt trennt jetzt drei noch offene von zwei beantworteten Fragen, nennt jede mit dem Pfad, den die Datei heute trägt, und vermerkt zusätzlich die beiden Entscheidungsdatensätze im Circle selbst, die ebenfalls beide beantwortet sind. Die Antworten stehen weiterhin nicht im Datensatz, sondern im Spec und in den Datensätzen, wie oben gefordert.

Ein Punkt kam nach dem Filing hinzu: `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_o_leistungszusagen-navigator.md` trägt seit dem 260802-1127 den Marker `_a_` und heißt jetzt `260802-1036_a_leistungszusagen-navigator.md`. Der Circle-Datensatz nennt bereits den neuen Pfad.

**Noch offen: Teil 2, `CLAUDE.md`.** Der Abschnitt `## Bindende Grundlage: fünf offene Entscheidungen` ist unverändert. Ein anderer Agent bearbeitet ihn. Dieser Defekt bleibt deshalb offen; er wird geschlossen, wenn beide Teile vorliegen.

**Hinweis für Teil 2:** der Abschnittstitel in `CLAUDE.md` nennt die Zahl fünf und ist damit selbst Teil des Fehlers. Drei der fünf Fragen im geteilten Speicher sind offen, und keine davon bindet die Runde 1.

---
Resolved: Beide Teile liegen vor. Teil 1 (Circle-Datensatz) am 260802-1127 durch den Shaper, siehe Abschnitt oben. Teil 2 (`CLAUDE.md`) am 260802-1130 durch den Coder: der Abschnitt heißt jetzt "Bindende Grundlage: die Entscheidungsdatensätze" ohne Zahl im Titel, führt den Dateibestand als verbindlich und die Tabelle ausdrücklich als datierte Momentaufnahme, nennt beide Speicher und trennt beantwortete von offenen Fragen. Der Circle-eigene Entscheidungsspeicher fehlte in `CLAUDE.md` bis dahin vollständig. Abnahme des Coders: 20 Pfadangaben geprüft, 20 vorhanden. Protokolle: `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1127-shaper-directive-korrektur-und-referenzgeraet.md` und `.../history/260802-1130-coder-claude-md-entscheidungsstand.md`.

Nachtrag, kein neuer Defekt: der Coder hat um 1130 festgehalten, es gebe noch keine Analysen. Der parallel laufende Analyst hat um 1134 `analyses/260802-1134-sprache-und-ui-werkzeugkasten.md` abgelegt. Die Aussage in `CLAUDE.md` ist damit seit 1134 überholt. Sie wird mit der Technologiefestlegung ohnehin fortgeschrieben, weil dann auch der Abschnitt `## Technologiewahl` fällt.
