# CLAUDE.md und HowTo.md nennen cmd+v als den Weg in den Filtertext, seit dem 260907 trägt ihn cmd+f

---

Der Nutzerentscheid vom 260907-2009 verlegt das Einfügen in den Filtertext des
Dateifensters von `cmd+v` auf `cmd+f`; `cmd+v` ist für eine spätere
Dateizwischenablage freigezogen und tut im Dateifenster nichts mehr. Zwei
Prosastellen außerhalb von `crates/` nennen weiter `cmd+v`:

- `CLAUDE.md:35`, die Zeile der Runde 21 in der Rundentabelle: „`cmd+v` im
  Dateifenster hängt die Zwischenablage an den Filtertext an, `*` im Filtertext
  ist ein Platzhalter". Die zweite Hälfte des Satzes bleibt richtig.
- `HowTo.md:82`, der Satz, den der Nutzer im Releasepaket liest: „**`cmd+v` hängt
  die Zwischenablage an den Filtertext an**, aber nicht so, wie …".

---

**Filed by:** ontocoder, Kai Stalmann <kai@stalmann.org>

**Cross-references:**
`260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`
(der Entscheid),
`260907-2020_*_das-einfuegen-in-den-filtertext-braucht-eine-siebte-vom-menue-zugestellte-funktion-in-der-belegung.md`
(der Befund zur Belegungszeile),
`260907-2020-k14-cmd-f-fuegt-in-den-filtertext-ein.md` (die Erhebung, die
`HowTo.md:82-88` im Codeanteil führt und die CLAUDE.md-Stelle ausdrücklich noch
nicht abgelegt hat)

Der `coder` hat den Befund am 260907-2020 bewusst nicht geschrieben, weil beide
Stellen bis zur Änderung zutrafen. Mit der Belegungszeile im Baum trifft das nicht
mehr zu, und der Befund steht deshalb jetzt. `HowTo.md` führt der Codeanteil ohnehin
mit; hier steht es, damit die CLAUDE.md-Stelle einen Träger hat, wenn der Durchgang
sie übergeht.

Beide Stellen sind laufender Text und keine Aufzeichnung eines Standes, die
Ortsregel schützt sie also nicht. `activity-log-k1.md` nennt `cmd+v`
ebenfalls und bleibt nach der Ortsregel stehen.

## Abnahme

`CLAUDE.md:35` und `HowTo.md:82` nennen cmd+f als den Weg in den Filtertext, oder
sie nennen beide Kombinationen mit dem Datum des Wechsels.

---
Halb erledigt am 260907-2127, Marker bleibt `_o_`: `HowTo.md` nennt jetzt
`cmd+f`, sagt, dass der Fokus zwischen der Suche im Editor und dem Einfügen in
den Filter entscheidet, dass `cmd+v` im Dateifenster nichts tut und für das
Einfügen einer Datei reserviert ist, und nennt das Datum des Wechsels
(`260907-2127-k15b-der-code-zur-belegungszeile-cmd-f.md`). `CLAUDE.md:35` ist
unberührt: der Auftrag jenes Durchgangs hat die Datei ausdrücklich
ausgenommen. Der Befund bleibt für diese eine Stelle offen, und seine
Abnahmebedingung ist unverändert die beider Stellen.

---
Resolved am 260908-0632: `CLAUDE.md:35` nennt `cmd+v` nicht mehr. Die Zeile heißt
jetzt „Einfügen aus der Zwischenablage in den Filtertext, `*` im Filtertext ist
ein Platzhalter"; die zweite Hälfte des Satzes, die schon richtig war, steht
unverändert.

**Die Abnahmebedingung ist auf einem anderen Weg erfüllt, als sie ihn vorsah, und
das gehört hierher.** Sie verlangte, dass die Stelle `cmd+f` nennt oder beide
Kombinationen mit dem Datum des Wechsels. Der Nutzer hat am 260908 stattdessen
entschieden, dass die Rundentabelle überhaupt keine Tastennamen mehr führt: die
Datei sagt in `CLAUDE.md:86` selbst, dass `resources/default-keymap.toml` die
Tastenzuordnung trägt und nicht sie, und vier Zeilen der Tabelle hielten sich
nicht daran. Die Zeilen der Runden 13, 20, 21 und 22 nennen seitdem die Funktion
statt der Taste. Damit ist die falsche Angabe weg und kann durch keine künftige
Umbelegung wiederkehren — die Bedingung ist der Sache nach übererfüllt und dem
Wortlaut nach umgangen.

`HowTo.md:82` war schon am 260907-2127 nachgezogen und nennt `cmd+f` samt Datum
des Wechsels. Beide Stellen des Befunds sind damit erledigt.
