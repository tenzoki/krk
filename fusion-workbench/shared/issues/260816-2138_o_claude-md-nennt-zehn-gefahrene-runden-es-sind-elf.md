CLAUDE.md nennt zehn gefahrene Runden, es sind elf

---

Der Abschnitt `## Worum es geht` behauptet in fetter Schrift "**Zehn Runden sind gefahren.**"
und führt darunter eine Tabelle mit zehn Zeilen. Am 260816 sind es elf: die Runde
`circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content` ist am selben Tag beschränkt
geschlossen worden und fehlt in der Tabelle. Nachzuzählen mit
`ls fusion-workbench/circles/*/_[bc]_circle.md` — elf Datensätze, zehn mit `_b_`, einer mit `_c_`.

Betroffen sind daneben zwei Stellen, die aus derselben Zahl rechnen: der Abschnitt
`## Projektstand` sagt "Was die Runden 2 bis 10 hinzugefügt haben", und der Satz über den
Abnahmelauf sagt "er liegt vor den Runden 5 bis 10 — keine der sechs ist gegen die zehn
Zusagen gemessen". Beide Spannen enden eine Runde zu früh.

---

Gefunden vom Shaper am 260816 beim Aufsetzen der zwölften Runde (Befehlslauf und Makros).
Er hat die Zahl gegen den Dateibestand geprüft, weil sein Auftrag von "elfter Runde" sprach.

Der Befund gehört in den gemeinsamen Speicher und nicht in den Circle der zwölften Runde:
er ist neben der Directive gefunden worden, nicht aus ihr entstanden.

Die Datei trägt an drei weiteren Stellen ausdrücklich die Regel, dass der Dateibestand
verbindlich ist und nicht die Aufzählung. Die Zeile "Zehn Runden sind gefahren" sagt das
im selben Absatz und nennt trotzdem eine Zahl. Wer den Befund behebt, sollte prüfen, ob
die Zahl dort überhaupt stehen bleiben soll.
