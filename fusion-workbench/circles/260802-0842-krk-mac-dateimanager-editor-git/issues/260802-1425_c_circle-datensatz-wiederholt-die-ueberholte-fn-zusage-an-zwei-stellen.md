Der Circle-Datensatz wiederholt die überholte Fn-Zusage an zwei weiteren Stellen

---

Die Directive-Zeile zur Tastenbelegung ist am 260802-1423 korrigiert worden, siehe den geschlossenen Defekt `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1417_c_directive-zeile-sagt-freie-funktionstasten-zu.md`. Zwei weitere Stellen im selben Datensatz `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md` tragen die überholte Fassung weiter:

1. Abschnitt `## Directive`, der auf die korrigierte Zeile folgende Satz: "Die Taste Delete räumt in den Papierkorb, Fn+F8 löscht endgültig und fragt dabei einmal je Vorgang nach." Die Schreibweise "Fn+F8" behauptet eine Unterscheidung, die KRK nach der Messung vom 260802-1338 nicht treffen kann: Fn+F8 und ein nacktes F8 erzeugen dasselbe Tastenereignis. Der Spec schreibt in C3 als Abnahmekriterium vor, dass die Belegungsansicht an keiner Stelle "Fn+" vor eine Kombination schreibt; die Directive selbst tut es weiterhin. Zusätzlich fehlt hier das ab Werk mitgelieferte Cmd-Kürzel Cmd+Opt+Delete.

2. Abschnitt `## Grounding snapshot`, Unterabschnitt "Offene Entscheidungen", Aufzählung "Beantwortet am 260802-1105": der Eintrag zu `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md` fasst dessen Antwort noch als "ausgeliefert wird ausschließlich die Fn-Kombination, Fn+F3 bis Fn+F8. Die nackten Funktionstasten bleiben frei" zusammen. Der Entscheidungsdatensatz selbst trägt seit dem 260802-1409 einen Nachtrag mit dem zweiten Weg über die Cmd-Kürzel; die Zusammenfassung im Grounding gibt den Stand vor diesem Nachtrag wieder.

---

**Was zu tun ist:** beide Stellen auf den Stand von C3 des Specs ziehen. Für Stelle 1 genügt es, "Fn+F8" durch "F8" zu ersetzen und das Cmd-Kürzel zu ergänzen. Für Stelle 2 ist die Zusammenfassung um den Nachtrag zu erweitern, ohne die ursprüngliche Antwort zu tilgen, weil das Grounding auch die Entstehung trägt.

**Warum der Shaper es nicht in derselben Bearbeitung behoben hat:** der Auftrag vom 260802-1423 hat die Freigabe für den Circle-Datensatz ausdrücklich auf einen einzigen Satz im Abschnitt `## Directive` und auf die Kopffelder samt Turn-Log begrenzt. Beide hier genannten Stellen liegen außerhalb dieser Freigabe. Sie sind fachlich unstrittig und in einem Zug zu beheben, sobald der Nutzer die Freigabe erteilt.

**Beleg für den Sachstand:** `spikes/fn-tasten/messung-A.txt`, Ereignisse #03 bis #05, und die korrigierte Auswertung in `spikes/fn-tasten/messung-A-neuauswertung.txt`. Ausformuliert im Spec `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`, Abschnitt C3.

**Aufgefallen bei:** der Behebung der beiden Defekte vom 260802-1417, Sitzung `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1423-shaper-circle-datensatz-korrektur.md`.

---
Resolved: Der Nutzer hat am 260802-1445 die Freigabe für beide Stellen erteilt, der Shaper hat sie gezogen.

Stelle 1, Abschnitt `## Directive`: der Satz lautet jetzt "Die Taste Delete und Cmd+Delete räumen in den Papierkorb, F8 und Cmd+Opt+Delete löschen endgültig und fragen dabei einmal je Vorgang nach." Die Schreibweise "Fn+F8" ist damit auch aus diesem Satz verschwunden. Über den gemeldeten Umfang hinaus nennt der Satz jetzt auch Cmd+Delete für den Papierkorb, weil die Cmd-Tabelle in C3 beide Wege führt und ein Satz, der nur eines der beiden Kürzel nennt, eine Asymmetrie behauptet, die es nicht gibt.

Stelle 2, Abschnitt `## Grounding snapshot`: die Zusammenfassung des F-Tasten-Entscheids führt weiterhin die Antwort des Nutzers vom 260802-1105 im Wortlaut und stellt den Nachtrag vom 260802-1409 daneben, mit der Tastencode-Sicht, dem Entfallen der freien nackten Funktionstasten und dem zweiten Weg über die Cmd-Kürzel. Die ursprüngliche Antwort ist wie gefordert nicht getilgt, weil das Grounding auch die Entstehung trägt.

**Zwei weitere Fälle sind beim Nachziehen aufgefallen** und gesondert abgelegt, weil sie außerhalb der Freigabe lagen. Der Unterabschnitt "Beantwortete Fragen aus der Klärungsrunde" sagt "Löschen ist ausdrücklich auf Shift+Delete vorbelegt", während C3 Shift+Delete seit dem 260802-1105 ab Werk unbelegt lässt. In derselben Aufzählung wie Stelle 2 trägt der Nachbareintrag zum Löschentscheid weiterhin die Schreibweise "Fn+F8". Siehe `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1445_o_grounding-snapshot-traegt-den-loeschstand-an-zwei-stellen-ueberholt.md`.
