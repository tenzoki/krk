Die Directive-Zeile sagt freie Funktionstasten zu, die Messung zeigt das als unerfüllbar

---

Der Circle-Datensatz `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md` sagt im Abschnitt `## Directive` zu: "ausgeliefert wird eine Mac-typische Vorbelegung, die die Norton-Reihe auf Fn+F3 bis Fn+F8 legt und die nackten Funktionstasten frei lässt." Dieselbe Formulierung steht im Abschnitt `## Grounding snapshot` bei der Aufzählung der beantworteten Entscheidungen.

Die Messung vom 260802-1137 auf dem Abnahmegerät belegt, dass KRK diese Zusage nicht einlösen kann. Fn+F3 und die nackte F3 erzeugen dasselbe Tastenereignis, Tastencode 99 mit gesetztem Modifikator `function`. Wer das eine belegt, belegt das andere mit. "Die nackten Funktionstasten frei lassen" ist damit keine Wahlmöglichkeit, sondern eine Zusage ohne technische Grundlage.

Zweitens nennt die Directive-Zeile den zweiten Weg über die Cmd-Kürzel nicht, den der Nutzer am 260802-1409 bestellt hat: jede Funktion der Norton-Reihe trägt ab Werk zusätzlich ein Mac-typisches Cmd-Kürzel.

---

**Beleg:** `spikes/fn-tasten/messung-A.txt`, Ereignisse #03 bis #05 und #09 bis #11. Ausformuliert im Spec `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`, Abschnitt C3, Unterabschnitt "Was KRK technisch belegt".

**Vorschlag für die korrigierte Zeile**, zur Entscheidung durch den Nutzer, nicht vom Shaper vorweggenommen: "Jede Tastenbelegung ist frei konfigurierbar; ausgeliefert wird eine Mac-typische Vorbelegung, die jede Funktion der Norton-Reihe auf zwei Wegen erreichbar macht, über die Funktionstaste F3 bis F8 und über ein Cmd-Kürzel. Die Taste Delete räumt in den Papierkorb, F8 löscht endgültig und fragt dabei einmal je Vorgang nach."

**Warum der Shaper es nicht selbst behoben hat:** der Auftrag dieser Runde untersagte ausdrücklich jede Änderung am Circle-Datensatz. Der Vorgang entspricht dem bereits geschlossenen Defekt `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1105_c_directive-zeile-widerspricht-loeschantwort.md`, bei dem der Nutzer am Spec-Gate die Korrektur der Directive-Zeile gewählt hat.

**Aufgefallen bei:** der Einarbeitung des Messergebnisses in C3, Sitzung `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1409-shaper-fn-tasten-messung-und-cmd-kuerzel.md`.

---
Resolved: Der Nutzer hat am 260802-1423 die Freigabe für den Circle-Datensatz erteilt, begrenzt auf diesen einen Satz. Die Zeile im Abschnitt `## Directive` lautet jetzt: "Jede Tastenbelegung ist frei konfigurierbar; ausgeliefert wird eine Mac-typische Vorbelegung, die jede Funktion der Norton-Reihe auf zwei Wegen erreichbar macht, über die Funktionstasten F3 bis F8 und über ein Cmd-Kürzel." Die unerfüllbare Zusage der freien nackten Funktionstasten ist entfallen, der zweite Weg über die Cmd-Kürzel steht darin, und die Schreibweise "Fn+" ist aus dem Satz verschwunden. Die Einzelheiten bleiben in C3 des Specs.

Zwei Stellen desselben Datensatzes tragen die überholte Fassung weiter und lagen außerhalb der Freigabe dieser Runde: die Schreibweise "Fn+F8" im folgenden Satz der Directive und die Zusammenfassung des F-Tasten-Entscheids im Abschnitt `## Grounding snapshot`. Beide sind gesondert gemeldet als `260802-1425_o_circle-datensatz-wiederholt-die-ueberholte-fn-zusage-an-zwei-stellen.md`. Der Abschnitt `## Abgleich mit der Circle-Directive` im Spec meldet die Abweichung ebenfalls noch als offen, gemeldet als `260802-1425_o_spec-meldet-die-directive-abweichung-noch-als-offen.md`.
