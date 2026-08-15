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

---
Abgleich 260815-1812 (reconciler): **Der Bestand ist erledigt, die Ursache steht unverändert,
und ein zweiter Datensatz hat inzwischen dieselbe Sache anders geschlossen.**

**Der Bestand.** Der Prüflauf über alle Entscheidungsdatensätze läuft am 260815-1812 ohne
eine einzige Abweichung durch — 137 Datensätze in `shared/decisions` und
`circles/*/decisions`, Marker und Kopffeld stimmen überall überein. Die sieben Datensätze der
Tabelle oben sind darunter. Gefahren ist die Berichtigung nicht von diesem Datensatz,
sondern von `cd0b5b7` über
`shared/issues/260815-1216_c_vierzehn-entscheidungsdatensaetze-tragen-im-rumpf-einen-anderen-stand-als-im-dateinamen.md`,
das denselben Befund projektweit erhoben hat.

**Die beiden Datensätze beschreiben eine Sache und schließen verschieden.** `260815-1216`
ist als `_c_` geschlossen mit der Feststellung, Weg 2 — die Kopfzeile aus der Vorlage nehmen —
gehöre `fusion` und sei von hier aus nicht gangbar, „der Rest ist als bekannte Lage
hinzunehmen". Dieser Datensatz bleibt offen und verlangt genau das, was jener als nicht
gangbar abgelegt hat. Zu entscheiden ist deshalb nicht mehr die Sache, sondern welcher der
beiden Abschlüsse gilt; solange das offen ist, bleibt der Marker hier `_o_`.

**Der Nebenbefund ist größer als die eine Datei, die er nennt.** Am 260815-1812 über alle
137 Entscheidungsdatensätze gezählt: **49** tragen eine leere Zeile `Answered:` ohne Inhalt
dahinter, und **25** tragen zwei `Answered:`-Blöcke, den leeren und einen ausgefüllten
darunter — also genau die Gestalt, die dieser Datensatz an
`260814-1852_a_raeumt-ein-gehaltener-rueckschritt-…` beschreibt. Alle 25 mit doppeltem Block
sind unter den 49. Ein Suchmuster auf einen leeren `Answered:`-Block meldet damit 49
angeblich unbeantwortete Datensätze; unbeantwortet sind 24 (`_o_`), und die beiden Mengen
decken sich nicht. Nachzuzählen mit:

```sh
cd fusion-workbench
for f in shared/decisions/*.md circles/*/decisions/*.md; do
  printf '%s %s %s\n' "$(grep -cE '^Answered: *$' "$f")" "$(grep -c '^Answered:' "$f")" "$f"
done
```
