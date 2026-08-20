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

---
Abgleich 260819-1440 (reconciler, Baumstand `77dcd48`): **offen, und die Zahl in der Überschrift dieses Datensatzes ist selbst überholt.** `CLAUDE.md:12` sagt unverändert „Zehn Runden sind gefahren", und die zwei abgeleiteten Spannen bei `:39` („Runden 2 bis 10") und `:78` („Runden 5 bis 10") stehen ebenso. Nachgezählt am Dateibestand: `ls fusion-workbench/circles/*/_*_circle.md` liefert 15 Datensätze, davon 10 beschränkt geschlossen, **3** kohärent geschlossen (`260813-0939`, `260817-0833`, `260818-1615`), einer vorgesehen (`260804-0933`, nie gefahren) und einer zurückgestellt (`260816-2255`, nie gefahren). **Gefahren sind 13**, nicht elf. Die Prosa in `CLAUDE.md` ist damit zum vierten Mal veraltet.

**Zwei weitere Aussagen derselben Datei hängen an dieser Zahl** und sind mit ihr falsch geworden: `CLAUDE.md` nennt die Runde `260813-0939` als „bisher einzige" kohärent geschlossene, und es sind drei. Der Datensatz bleibt `_o_` für den Durchgang des Kurators; dieser Abgleich fasst `CLAUDE.md` nicht an.

---
Resolved: Der Kuratorenlauf `260819-1500` hat mit seinem Eintrag L01 die Zahl aus `CLAUDE.md`
genommen und durch das Kommando `ls fusion-workbench/circles/*/_*_circle.md` ersetzt (Commit
`5886d04`); der Lauf `260820-1119` hat mit L03 die Rundentabelle auf vierzehn Zeilen und mit L04
die Zahl der kohärent geschlossenen Runden von einer auf drei gezogen (Commit `7da3098`).
**Am Baumstand `f5300f4` einzeln nachgelesen, alle vier Behauptungen dieses Datensatzes:**
`grep -c 'Wie viele Runden gefahren sind und wie jede geschlossen hat, sagt der Dateibestand' CLAUDE.md`
liefert 1, die fette Zeile „Zehn Runden sind gefahren" steht nirgends mehr; die abgeleitete Spanne
„die Runden 2 bis 10" heißt jetzt „Was die Runden ab der zweiten hinzugefügt haben"; die zweite
Spanne „er liegt vor den Runden 5 bis 10 — keine der sechs" heißt jetzt „er liegt vor jeder Runde,
die seither geschlossen hat"; und die Aussage über die „bisher einzige" kohärent geschlossene Runde
nennt jetzt drei. Damit ist keine der beanstandeten Stellen mehr im Baum, und keine trägt eine Zahl,
die ein sechstes Mal veralten könnte — genau die Richtung, die dieser Datensatz selbst empfohlen hat.
