# Acht offene Defektdatensätze tragen eine leere `Resolved:`-Zeile und antworten jeder Suche als geschlossen

---
**Domain:** code
**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/issues/260818-0710_*_forty-three-closure-notes-are-written-in-a-form-no-resolved-sweep-finds.md` (derselbe Mechanismus, die andere Richtung); `shared/issues/260820-2056_*_dreissig-entscheidungsdatensaetze-tragen-eine-leere-vorlagenzeile-vor-der-gefuellten.md` (dieselbe Vorlagenlücke im Entscheidungsspeicher); `rules/fusion-workbench-conventions.md` `## Inline State Tracking`

---

## Was ist

Acht Defektdatensätze mit dem Marker `_o_` tragen im Rumpf eine Zeile `Resolved:` ohne jeden
Text dahinter, eingefasst in die zwei `---`-Trenner der Vorlage. Wer den Speicher mit
`grep -l '^Resolved:'` abfragt, bekommt sie als erledigt zurück, obwohl an keinem etwas behoben
ist.

## Gemessen am 260826-1024, Baumstand `c95f28b`

```
for f in shared/issues/*_o_*.md circles/*/issues/*_o_*.md; do
    grep -qE '^\**Resolved:\**[[:space:]]*$' "$f" && echo "$f"
done
```

| | Datensatz |
|---|---|
| 1 | `shared/issues/260823-1433_o_kommando-ausfuehren-liefert-nicht-immer-true-…` |
| 2 | `shared/issues/260823-1436_o_die-wettrennprobe-des-oeffnens-braucht-allein-neun-sekunden-…` |
| 3 | `shared/issues/260823-1439_o_drei-zeilenzitate-im-quelltext-zeigen-ins-leere-…` |
| 4 | `shared/issues/260823-1442_o_der-modulkopf-der-rundwegproben-nennt-eine-abwehr-…` |
| 5 | `shared/issues/260823-1445_o_die-neue-regel-verweist-jeden-rufer-an-sich-selbst-…` |
| 6 | `shared/issues/260823-1651_o_die-auslieferung-ist-der-letzte-commit-einer-sitzung-…` |
| 7 | `shared/issues/260824-1745_o_ein-commit-des-orchestrators-nimmt-die-git-mv-umbenennungen-…` |
| 8 | `shared/issues/260824-1758_o_die-zeitstempel-in-dateinamen-laufen-der-uhr-voraus-…` |

Ein neunter ist mit diesem Abgleich weggefallen:
`260823-1649` hat seine Zeile ausgefüllt bekommen und steht jetzt auf `_c_`.

Alle acht stammen aus zwei Tagen, dem 260823 und dem 260824, und aus zwei Läufen. Über 194
offene Datensätze sind es acht; die übrigen 186 tragen die Zeile nicht.

**Fünf geschlossene Datensätze tragen dieselbe leere Zeile**, und zwei Gestalten sind darunter
zu unterscheiden. Bei vieren steht der Text der Notiz auf der **nächsten** Zeile statt hinter
dem Doppelpunkt; die Notiz ist da, nur nicht dort, wo eine Suche sie liest. Beim fünften
(`shared/issues/260823-1650_c_die-releaseseite-der-1-0-0-schweigt-…`) steht die leere Zeile
zusätzlich zu einer richtigen weiter unten. Die vier gehören der Sache nach zu
`260818-0710_*` und sind hier nur genannt, damit die Erhebung vollständig ist.

## Warum das zählt

`260818-0710_*` beschreibt die eine Richtung des Fehlers: geschlossene Datensätze, deren Notiz
eine Suche nach `^Resolved:` **nicht** findet. Dies ist die andere: offene Datensätze, die eine
solche Suche **findet**, obwohl sie nichts zu melden haben. Die zweite Richtung ist die
teurere. Ein übersehener Abschluss kostet eine Nachprüfung; ein vorgetäuschter kostet die
Nachprüfung nicht, weil niemand sie ansetzt.

Dazu kommt, dass die Zahlen von `260818-0710_*` selbst betroffen sind: seine Erhebung zählt
385 Datensätze mit `^Resolved:` als der Konvention entsprechend, und vier der fünf oben stehen
in diesem Eimer, obwohl ihre Notiz erst in der Folgezeile beginnt.

## Was zu tun wäre

An den acht offenen die leere Zeile samt ihren zwei Trennern streichen. Kein Marker bewegt sich
dabei, keine Aussage geht verloren: die Zeile trägt keine. An den vier geschlossenen die Notiz
hinter den Doppelpunkt ziehen, was zu `260818-0710_*` gehört und dort mitzunehmen ist.

Der Abgleich hat es nicht selbst getan, weil das Streichen einer Zeile im Rumpf eines fremden
Datensatzes das Bearbeiten seiner Beschreibung wäre und nicht das Setzen einer Markierung. Beim
einen, den dieser Abgleich ohnehin geschlossen hat (`260823-1649`), ist die Zeile ausgefüllt
statt gestrichen worden, weil dort eine Notiz hingehörte.

**Schwere:** mittel. Kein Verhalten der Anwendung. Acht falsche Auskünfte in dem Speicher, den
`CLAUDE.md` als verbindlich benennt, und zwar in der Richtung, in der ein Fehlbefund unbemerkt
bleibt.

**Gefunden:** reconciler, Schlussabgleich der Sitzung `260825-1659` gegen `e5ec81a..c95f28b`.
