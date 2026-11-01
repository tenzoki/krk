# Die Markdown-Vorschau rendert Tabellen mit richtig ausgerichteten Spalten

## Directive

Wortlaut des Nutzers (261031):

> neues paket: der makrdown viewer kann tabellen nicht korrekt rendern, finde eine lösung, so dass die spalten richtig aliniert sind.

Erreicht ist das Ziel, wenn eine Markdown-Tabelle in der gerenderten Vorschau so erscheint, dass die Zellen jeder Spalte untereinander stehen, gleich wie lang ihr Inhalt in den einzelnen Zeilen ist.

## Pause (261101)

Pausiert auf Wunsch des Nutzers (Antwort „1“, 261101). Das Paket wartet auf ein grünes `make check` durch das Paket `261101-1320-appkit-proben-laufen-auf-dem-hauptfaden`. Danach stehen noch aus: die Abnahme je Prüfdatei, die Beobachtung bei 160 pt (zu `260812-1000_*_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md`) und die Antwort auf die übrigen Haltebedingungen des Plans. Die Schlussprüfung `261101-0924-reviewer-schlusspruefung-markdown-tabellen.md` urteilt revise; ihre drei Befunde sind mit `194c643a` behoben, der außerhalb des geprüften Bereichs liegt.

## Abschluss (261101)

Geschlossen als `done` (`completed`), Bereich `190dbeaa..194c643a`. Der Nutzer hat am 261101 nach einem frischen `make bundle` (Stand `32a47fe5`) die Abnahmedateien `tabellen-abnahme.md`, `tabellen-gross.md` und `tabellen-abschnitte.md` angesehen und alle neun Haltebedingungen des Plans als erfüllt bestätigt; die 1-MB-Tabelle öffnet sich ohne sichtbares Hängen (`(condition did not arise: one clause)`).

**Beobachtung bei 160 pt** (zu `260812-1000_*_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md`): nach Auskunft des Nutzers ragt bei 160 Punkten Vorschaubreite auch in Tabellen mit drei oder mehr Spalten kein Wort über den Rand seiner Zelle. An der Mindestbreite ändert dieses Paket nichts.

Prüfung: die Schlussprüfung `reviews/261101-0924-reviewer-schlusspruefung-markdown-tabellen.md` (revise, drei Defekte, mit `194c643a` behoben) und die Nachprüfung `reviews/261101-2225-reviewer-nachpruefung-markdown-tabellen.md` über `f70c6202..194c643a` (revise, ein Low-Befund in der Prosa). Offen bleibt als Nacharbeit `issues/261101-2225-howto-und-modulkopf-nennen-den-strich-davor-auch-fuer-die-erste-spalte-einer-tabelle-ohne-aeussere-striche.md`. Keine Auslieferung gehört zu diesem Paket; der Stand liegt nach `v3.14.0`.
