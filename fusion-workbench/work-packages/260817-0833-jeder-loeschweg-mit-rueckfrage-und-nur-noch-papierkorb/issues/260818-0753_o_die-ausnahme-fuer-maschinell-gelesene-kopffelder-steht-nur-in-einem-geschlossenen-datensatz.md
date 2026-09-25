Die Ausnahme fuer maschinell gelesene Kopffelder steht nur in einem geschlossenen Datensatz

---

Die Zeigerreparatur des 260818 laesst `**Active spec/plan:**` in
`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/_t_circle.md:7`
bewusst beim ausgeschriebenen Marker, weil das Feld ein maschinell gelesener Pfad ist. Die
Begruendung stimmt und ist geprueft. Sie steht aber nur im Abschlussvermerk eines geschlossenen
Befundes und in keiner bindenden Regel — und der bindende Datensatz `260815-1145` kennt eine
andere Ausnahmeprobe, die dieses Feld nicht deckt. Der naechste Durchgang, der die Regel
woertlich liest, sternt das Feld und bricht damit einen woertlichen Dateizugriff.

---

**Severity:** Low
**Found by:** coderev, Durchsicht der Zeigerreparatur 260818-0752
**Domain:** code

## Was geprueft ist

**Die Begruendung traegt.** `rules/circle-records.md:77` und `:106` definieren
`**Active spec/plan:**` als werkbank-relativen Pfad, nicht als Dateinamen. Drei Verbraucher
lesen ihn als Pfad: `agents/orchestrator.md:275` und `:407` schreiben ihn, `skills/next/SKILL.md:232`
liest ihn beim Aktivieren, `skills/migrate/SKILL.md:99` schreibt ihn um und nennt ausdruecklich,
dass ein Fehler dort still degradiert. Beide Pfade der Zeile 7 loesen heute auf: der Plan traegt
`_c_`, der Spec traegt `_o_`.

## Die Luecke

**Die Ausnahmeprobe des bindenden Datensatzes trifft dieses Feld nicht.**
`shared/decisions/260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md:54`
formuliert sie so: „Die Probe dafuer ist, was der Stern kostet: ein Zeiger auf eine Datei
verliert nichts, eine Aussage ueber einen Zustand verliert ihren Inhalt." `**Active spec/plan:**`
ist nach dieser Probe ein **Zeiger** und gehoert damit in die Sternform. Dass er trotzdem den
Buchstaben behaelt, ist eine zweite, neue Ausnahmeklasse: nicht „der Marker ist die Aussage",
sondern „der Pfad wird woertlich benutzt".

**Dieselbe Sitzung hat die beiden Klassen gegenlaeufig behandelt.** Der Plankopf
`**Spec:**` (`planning/260817-0856_*_plan-absicherung-jedes-loeschwegs.md:5`) ist ein Kopffeld
derselben Bauart mit einem Pfad darin und wurde gesternt. Das bricht heute nichts — ein `grep`
ueber `agents/`, `skills/`, `rules/` und `hooks/` findet fuer `**Spec:**` nur die Definition in
`agents/planner.md:104` und keinen Verbraucher —, aber die Unterscheidung zwischen den beiden
Feldern steht nirgends geschrieben. Sie ist rekonstruierbar und wurde rekonstruiert; ein
spaeterer Durchgang muss sie erneut rekonstruieren.

## Warum das zaehlt

Der Fehler waere still: ein gesterntes `**Active spec/plan:**` bricht keinen Bau und keine Probe,
sondern laesst `/fusion:next` und die Wiederaufnahme des Orchestrators ins Leere greifen. Genau
diese Klasse benennt `skills/migrate/SKILL.md:99` als das, was `HYG-NO-SILENT-FAIL` verbietet.

Die Erhebung dieser Durchsicht bestaetigt, dass es heute genau eine solche Stelle im lebenden Text
gibt: `_t_circle.md:7` mit zwei Pfaden. Die Ausnahme ist also klein und leicht zu notieren.

## Empfehlung

Eine Zeile in `CLAUDE.md` unter `## Bindende Grundlage`, neben der Ortsregel und der bestehenden
Ausnahme: **Kopffelder, deren Wert woertlich als Pfad gelesen wird, behalten den Buchstaben** —
heute `**Active spec/plan:**` und `**Active session history:**` in einem Circle-Datensatz. Die
Alternative waere ein eigener Entscheidungsdatensatz; er waere teurer als die Sache ist, denn die
Antwort steht schon fest und ist nur nicht aufgeschrieben.

Falls dagegen entschieden wird, gehoert die Begruendung in den Datensatz `260815-1145` als dritte
benannte Grenze der Antwort.
