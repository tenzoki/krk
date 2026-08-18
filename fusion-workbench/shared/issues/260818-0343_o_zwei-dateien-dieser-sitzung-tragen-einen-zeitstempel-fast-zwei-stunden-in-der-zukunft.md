Zwei Dateien dieser Sitzung tragen einen Zeitstempel fast zwei Stunden in der Zukunft
---
Gemessen am 260818-0343 gegen die Geräteuhr. Zwei Dateien der Sitzung `260817-2131` tragen einen Namensstempel, den keine Uhr geliefert hat:

- `circles/260817-0833-…/decisions/260818-0512_o_wie-lautet-die-frage-wenn-der-umfang-der-genannte-grund-ist-und-die-zahl-doppelt-dasteht.md`
- `circles/260817-0833-…/history/260818-0530-coder-sechs-befunde-der-buendel-c-und-d.md`

Beide stammen aus demselben Lauf, dem `coder`-Stapel, der als Commit `285b58f` festgeschrieben ist. Die Uhr stand zum Zeitpunkt jenes Commits bei etwa `0319`. Die sechzehn übrigen Dateien der Sitzung sind stimmig und laufen monoton; der Fehler ist einer von achtzehn Läufen und keine durchgehende Eigenschaft.

Gefunden vom `ontocoder`, der den Sprung bemerkte, weil sein eigener Verlaufseintrag (`260818-0340`) vor einer bereits abgelegten Datei zu liegen kam.
---
**Die Regel steht und wurde nicht befolgt.** `rules/fusion-workbench-conventions.md`, Abschnitt `## Timestamps`: „Always obtain `YYMMDD-HHMM` from `date +%y%m%d-%H%M`. LLMs have no clock — never guess or estimate the time." Ein geschätzter Stempel ist genau das, was die Regel verbietet, und er sieht von einem echten nicht zu unterscheiden aus.

**Was es kostet.** Der Stempel ist der Sortierschlüssel jedes Speichers der Werkbank und die Grundlage jeder Frage der Form „was kam zuerst". Zwei Dateien in der Zukunft kehren die Reihenfolge gegen alles um, was danach noch in derselben Stunde entsteht — die Entscheidungsfrage `0512` liest sich, als sei sie nach dem Abschluss der Sitzung gestellt worden, und sie war es nicht.

**Warum es eine Wiederholung ist.** Derselbe Befund lag in dieser Sitzung schon vor und ist mit Commit `59ddcbe` geschlossen worden: `circles/260817-0833-…/issues/260817-1807_*_two-history-filenames-and-four-closure-notes-carry-timestamps-that-no-clock-produced.md`, dort vier Protokolldateien um 4 bis 27 Minuten voraus. Die Behebung war ein Nachtrag je Datei. **Sie hat die Wiederholung nicht verhindert**, und das ist die eigentliche Auskunft dieses Datensatzes: der Fehler entsteht neu in jedem Lauf, der einen Namen bildet, ohne die Uhr zu fragen, und eine Korrektur an den betroffenen Dateien erreicht den nächsten Lauf nicht.

Der Sprung ist hier größer als beim ersten Mal, 113 Minuten gegen 27, was gegen die Lesart einer Ungenauigkeit und für eine Schätzung spricht.

**Zur Behandlung.** Zwei Wege sind denkbar und verschieden teuer: den Nachtrag je Datei wie beim ersten Mal, oder eine Stelle, die den Stempel liefert, statt ihn jedem Lauf zu überlassen. Welcher richtig ist, entscheidet dieser Datensatz nicht. Beim Umbenennen ist zu beachten, was der erste Befund schon festhielt: ein Dateiname ist ein Zeiger, und `260818-0530` wird von der Werkbank bereits zitiert.

**Domain:** code
**Filed by:** orchestrator
**Related:** `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260817-1807_*_two-history-filenames-and-four-closure-notes-carry-timestamps-that-no-clock-produced.md` (dieselbe Klasse, in dieser Sitzung geschlossen)
