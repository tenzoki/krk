Der Grounding snapshot trägt den Löschstand an zwei weiteren Stellen überholt

---

Der Abschnitt `## Grounding snapshot` in `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md` gibt die Löschbelegung an zwei Stellen so wieder, wie sie vor den Antworten des Nutzers vom 260802-1105 und vor der Messung vom 260802-1409 stand. Beide Stellen liegen außerhalb der Freigabe vom 260802-1445, die zwei andere Stellen desselben Datensatzes nannte.

**Stelle 1, Unterabschnitt "Beantwortete Fragen aus der Klärungsrunde", Absatz Bedienmodell:**

> Löschen ist ausdrücklich auf Shift+Delete vorbelegt. Damit ist Löschen ab Werk auf zwei Wegen erreichbar, über F8 aus der Norton-Reihe und über Shift+Delete; beides ist gewollt und kein Konflikt.

Der Spec `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` sagt in C3 das Gegenteil: "Shift+Delete ist ab Werk unbelegt. Der Nutzer kann die Kombination frei belegen, KRK liefert sie nicht vorbelegt aus." Der Stand des Specs geht auf die Antwort des Nutzers vom 260802-1105 zurück, festgehalten in `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`: Delete räumt in den Papierkorb, F8 löscht endgültig. Shift+Delete kommt darin nicht mehr vor.

Derselbe Absatz nennt zusätzlich "die Norton-Belegung auf F3 bis F8", was seit der Korrektur vom 260802-1423 wieder richtig ist und nicht angefasst werden muss.

**Stelle 2, Unterabschnitt "Offene Entscheidungen", Aufzählung "Beantwortet am 260802-1105":**

> `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md` — die Taste Delete räumt in den Papierkorb, Fn+F8 löscht endgültig.

Die Schreibweise "Fn+F8" behauptet dieselbe Unterscheidung, die die Messung ausschließt: Fn+F8 und ein nacktes F8 erzeugen dasselbe Tastenereignis. Der Nachbareintrag derselben Aufzählung, die Zusammenfassung des F-Tasten-Entscheids, ist am 260802-1445 genau deshalb nachgezogen worden; dieser hier stand nicht in der Freigabe und blieb stehen. Zusätzlich fehlt auch hier das ab Werk mitgelieferte Cmd-Kürzel Cmd+Opt+Delete.

---

**Was zu tun ist:** beide Stellen auf den Stand von C3 ziehen. Für Stelle 2 genügt es, "Fn+F8" durch "F8" zu ersetzen und Cmd+Opt+Delete zu ergänzen. Für Stelle 1 gilt, was schon für die Zusammenfassung des F-Tasten-Entscheids galt: die ursprüngliche Aussage nicht tilgen, sondern den späteren Stand danebenstellen, weil der Absatz eine Klärungsrunde festhält und das Grounding auch die Entstehung trägt.

**Warum der Shaper es nicht in derselben Bearbeitung behoben hat:** die Freigabe vom 260802-1445 nannte drei Stellen, zwei davon im Circle-Datensatz, und diese beiden waren keine davon. Sie sind fachlich unstrittig und in einem Zug zu beheben, sobald der Nutzer die Freigabe erteilt.

**Beleg für den Sachstand:** `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`, Antwort des Nutzers vom 260802-1105. Ausformuliert im Spec, Abschnitt C3, Abnahmekriterien zu Delete, F8 und Shift+Delete, und in C4 unter "Getroffene Festlegungen". Für die Nichtunterscheidbarkeit von Fn+F8 und F8: `spikes/fn-tasten/messung-A.txt`, Ereignisse #03 bis #05, und `spikes/fn-tasten/messung-A-neuauswertung.txt`.

**Aufgefallen bei:** dem Nachziehen der drei Reststellen vom 260802-1425 und 260802-1428, Sitzung `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1445-shaper-restellen-fn-und-c8.md`.

---
Resolved: Beide Stellen im Abschnitt `## Grounding snapshot` von `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md` stehen auf dem Stand von C3. Stelle 1, der Absatz "Bedienmodell", behält die ursprüngliche Aussage und trägt darunter den späteren Stand: Shift+Delete ist ab Werk unbelegt, Delete und Cmd+Delete räumen in den Papierkorb, F8 und Cmd+Opt+Delete löschen endgültig. Stelle 2, der Eintrag zum Löschentscheid, schreibt jetzt F8 statt "Fn+F8" und nennt beide Cmd-Kürzel.

In derselben Bearbeitung sind drei weitere Stellen desselben Abschnitts nachgezogen worden, die dieser Defekt nicht nannte und die der Nutzer mit der Freigabe vom 260802-1735 mit abgedeckt hat: die Ausgangslage führte Sprache und UI-Werkzeugkasten noch als offen, obwohl sie seit dem 260802-1150 entschieden sind; die Liste der Entscheidungsdatensätze im Circle nannte zwei von inzwischen fünf; und die Forderung an den Aktivierungs-Spec, die Maxime "superschnell" in messbare Zusagen zu überführen, ist mit C8 erfüllt. Alle drei folgen dem Muster von Stelle 1: die ursprüngliche Aussage bleibt stehen, der spätere Stand tritt daneben.

Sitzung: `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1735-shaper-l4-entscheid-und-grounding.md`.
