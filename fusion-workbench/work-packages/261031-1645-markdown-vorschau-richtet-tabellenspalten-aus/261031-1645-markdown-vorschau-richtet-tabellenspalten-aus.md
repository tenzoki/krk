# Die Markdown-Vorschau rendert Tabellen mit richtig ausgerichteten Spalten

## Directive

Wortlaut des Nutzers (261031):

> neues paket: der makrdown viewer kann tabellen nicht korrekt rendern, finde eine lösung, so dass die spalten richtig aliniert sind.

Erreicht ist das Ziel, wenn eine Markdown-Tabelle in der gerenderten Vorschau so erscheint, dass die Zellen jeder Spalte untereinander stehen, gleich wie lang ihr Inhalt in den einzelnen Zeilen ist.

## Pause (261101)

Pausiert auf Wunsch des Nutzers (Antwort „1“, 261101). Das Paket wartet auf ein grünes `make check` durch das Paket `261101-1320-appkit-proben-laufen-auf-dem-hauptfaden`. Danach stehen noch aus: die Abnahme je Prüfdatei, die Beobachtung bei 160 pt (zu `260812-1000_*_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md`) und die Antwort auf die übrigen Haltebedingungen des Plans. Die Schlussprüfung `261101-0924-reviewer-schlusspruefung-markdown-tabellen.md` urteilt revise; ihre drei Befunde sind mit `194c643a` behoben, der außerhalb des geprüften Bereichs liegt.
