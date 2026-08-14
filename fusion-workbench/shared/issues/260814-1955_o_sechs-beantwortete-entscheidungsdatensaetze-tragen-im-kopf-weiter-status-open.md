Sechs beantwortete Entscheidungsdatensätze tragen im Kopf weiter `**Status:** open`

---

Der Marker im Dateinamen und das Feld `**Status:**` im Kopf desselben Datensatzes widersprechen einander. Am 260814-1950 trugen sechs der sieben mit `_a_` markierten Datensätze des Circles `260814-1551-tippen-filtert-dateiliste-flach-und-tief` im Kopf weiter `open`, obwohl jeder eine ausgefüllte `Answered:`-Zeile hat. Der Shaper hat die sechs Felder beim Nachbessern des Spec auf `answered` gesetzt; der Mechanismus, der die Abweichung erzeugt, besteht fort.

---

**Gefunden am:** 260814-1950, beim Nachbessern des Spec auf die zweite Bewertung
**Gemeldet von:** shaper
**Herkunft:** neben der Arbeit am Circle `260814-1551-tippen-filtert-dateiliste-flach-und-tief` gefunden; die Ursache liegt im Ablauf, mit dem Datensätze beantwortet werden, und nicht in dessen Directive. Deshalb im gemeinsamen Speicher.

## Was der Befund war

| Datensatz | Marker | Kopffeld vor der Berichtigung |
|---|---|---|
| `260814-1552_a_passt-der-filter-auf-den-namensanfang-…` | `_a_` | `open` |
| `260814-1552_a_steigt-die-tiefe-suche-in-symbolische-verknuepfungen-hinab` | `_a_` | `open` |
| `260814-1552_a_was-geschieht-mit-einer-markierung-…` | `_a_` | `open` |
| `260814-1552_a_welche-tastenkombination-schaltet-die-tiefe-suche` | `_a_` | `open` |
| `260814-1552_a_wie-kommt-der-nutzer-von-einem-tiefen-treffer-…` | `_a_` | `open` |
| `260814-1852_a_raeumt-ein-gehaltener-rueckschritt-…` | `_a_` | `open` |
| `260814-1830_a_wie-nimmt-der-nutzer-ein-einzelnes-zeichen-…` | `_a_` | `answered` — richtig |

Einer von sieben war richtig gesetzt. Das ist kein einzelnes Versehen, sondern die Regel mit einer Ausnahme.

## Warum es zählt, obwohl der Marker verbindlich ist

`fusion-workbench-conventions.md` führt `**Status:** open | answered | implemented | deferred | superseded` als Kopffeld jedes Entscheidungsdatensatzes, und `CLAUDE.md` sagt zugleich, verbindlich sei der Dateibestand. Beides zusammen macht das Feld nicht harmlos, sondern zu einer zweiten Quelle, die falsch antwortet: wer einen einzelnen Datensatz öffnet, liest zuerst den Kopf und nicht den Dateinamen. Dieses Projekt hat den Fall schon einmal bezahlt — `260812-2253_o_sieben-verweise-im-circle-datensatz-der-runde-5-tragen-einen-gestorbenen-marker.md` ist derselbe Riss zwischen zitiertem und tatsächlichem Stand.

Dazu kommt ein zweiter, kleinerer Befund am selben Ort: `260814-1852_a_raeumt-ein-gehaltener-rueckschritt-…` trägt zwei `Answered:`-Blöcke, den leeren aus der Vorlage und den ausgefüllten darunter. Der leere ist beim Beantworten nicht ersetzt, sondern ein zweiter angehängt worden. Ein Suchmuster auf `^Answered:$` findet damit einen unbeantworteten Datensatz, den es nicht gibt.

## Was zu tun bleibt

Die sechs Felder sind gesetzt. Offen ist die Ursache: wer einen Datensatz beantwortet, benennt heute die Datei um und schreibt die `Answered:`-Zeile, fasst das Kopffeld aber nicht an. Solange der Schritt einzeln neben der Umbenennung steht, wird er wieder ausgelassen — dieses Projekt hat genau diese Gestalt schon mehrfach als übersprungen gemessen. Die Berichtigung gehört deshalb in denselben Vorgang wie die Umbenennung und nicht daneben.
