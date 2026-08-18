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

---
Also seen: 260817-1833 by reconciler — die Zahl ist inzwischen zwölf, `CLAUDE.md` sagt weiter
zehn. Gemessen über `ls fusion-workbench/circles/*/*_circle.md`: vierzehn Circles, davon einer
`_a_` (anticipated, nie gefahren) und einer `_d_` (`260816-2255-befehle-absetzen-und-makros-speichern`,
am 260817 zugunsten der laufenden Runde zurückgestellt), also zwölf gefahrene. Der
Circle-Datensatz der laufenden Runde nennt sich selbst „die zwölfte gefahrene Runde", und
`crates/krk-ui/src/kommandos/mod.rs:26` schreibt „Runde 12" aus. Die Tabelle in `CLAUDE.md`
führt zehn Zeilen. Damit ist die Zahl seit dem Filing dieses Datensatzes ein zweites Mal
veraltet, was der Datensatz selbst als Muster benennt.

---
Also seen: 260819-0057 by reconciler — es sind jetzt **dreizehn**, und `CLAUDE.md` sagt weiter
zehn. Gezählt über `ls fusion-workbench/circles/*/*_circle.md`: fünfzehn Circle-Datensätze,
davon zehn `_b_`, zwei `_c_`, einer `_t_` (die laufende Runde 13), einer `_a_` (nie gefahren)
und einer `_d_`. Gefahren sind damit die zehn beschränkt geschlossenen, die zwei kohärent
geschlossenen und die laufende: dreizehn. Die Tabelle in `CLAUDE.md` führt zehn Zeilen, und die
zwei Spannen, die aus derselben Zahl rechnen („die Runden 2 bis 10", „er liegt vor den Runden 5
bis 10 — keine der sechs"), enden jetzt drei Runden zu früh. Damit ist die Zahl seit dem Filing
zum dritten Mal veraltet; der Datensatz benennt genau dieses Muster.
