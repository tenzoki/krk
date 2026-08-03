C3 nennt F6 "Verschieben und Umbenennen", die Belegungstabelle desselben Abschnitts nur "Verschieben"

---

Der Spec `planning/260802-1036_o_spec-navigator-geruest.md` beschreibt F6 an zwei Stellen von C3 verschieden. Das Abnahmekriterium in Zeile 120 lautet: "Die Norton-Zuordnung der Auslieferungsbelegung lautet: F3 Vorschau anzeigen, F5 Kopieren, **F6 Verschieben und Umbenennen**, F7 Ordner anlegen, F8 endgültig löschen." Die Tabelle "Die ausgelieferten Cmd-Kürzel" in Zeile 136 führt für dieselbe Taste allein "Verschieben in das andere Fenster | F6 | Cmd+Shift+V".

Beide Lesarten zugleich sind nicht umsetzbar. C4 verlangt das Umbenennen als eigene Operation ("Umbenennen: ein Tastenbefehl benennt den ausgewählten Eintrag um, direkt in der Liste", Zeile 165) und daneben das Umbenennen im Stapel (Zeile 177). Läge das Umbenennen mit auf F6, trügen zwei verschiedene Funktionen dieselbe Kombination, was das Abnahmekriterium in Zeile 119 ausdrücklich ausschließt: "Die Auslieferungsbelegung ist in sich konfliktfrei: keine Kombination ist zwei verschiedenen Funktionen zugewiesen."

---

Herkunft: gefunden beim Schreiben von `resources/default-keymap.toml` (Plan Schritt 9, `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`).

Wie die Datei den Widerspruch vorläufig auflöst: F6 und `shift+cmd+v` tragen allein das Verschieben, so wie es die Cmd-Kürzel-Tabelle und das Abnahmekriterium von Schritt 9 wörtlich vorschreiben. Das Umbenennen hat eine eigene Zeile mit `shift+f6` und `shift+cmd+u` bekommen; die Umschalttaste vor F6 ist die Norton- und Total-Commander-Form für das Umbenennen und hält die Nähe zu F6, ohne die Kombination zu teilen.

Was zu entscheiden ist: ob die Formulierung in Zeile 120 auf "F6 Verschieben" gezogen wird (dann beschreibt sie nur noch, was die Tabelle sagt, und die Datei stimmt bereits) oder ob der Nutzer für das Umbenennen eine andere Kombination will als `shift+f6`. Der zweite Fall ändert eine Zeile in `resources/default-keymap.toml`.
