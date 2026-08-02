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
