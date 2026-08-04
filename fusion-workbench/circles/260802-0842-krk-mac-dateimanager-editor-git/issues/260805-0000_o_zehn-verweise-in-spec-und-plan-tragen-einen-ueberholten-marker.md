Zehn Verweise in Spec und Plan tragen einen überholten Marker und zeigen damit auf keine Datei

---

Spec und Plan nennen zehn Datensätze mit dem Marker `_o_` im Pfad, die auf der
Platte inzwischen `_a_`, `_i_` oder `_c_` tragen. Jeder dieser Pfade zeigt damit
ins Leere. In sieben Fällen sagt der umgebende Satz zusätzlich "offen", was
sachlich nicht mehr stimmt.

---

## Der Nachweis

Am 260805-0000 gegen den Dateibestand geprüft. Links der Pfad, wie ihn Spec oder
Plan schreiben, rechts der Marker der Datei:

| Verweis im Text | Marker auf der Platte |
|---|---|
| `decisions/260804-0830_o_was-die-zwischenablage-auswertung-liest.md` | `_i_` |
| `decisions/260804-1122_o_wandern-die-bereichsbreiten-auf-die-links-und-rechts-pfeile.md` | `_a_` |
| `issues/260803-2007_o_s14-bindet-fsevents-ohne-das-framework-coreservices-zu-verlinken.md` | `_c_` |
| `issues/260803-2007_o_s16-nennt-keinen-mechanismus-fuer-die-buendelung-der-fortschrittsmeldungen.md` | `_c_` |
| `issues/260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen.md` | `_c_` |
| `issues/260803-2317_o_der-kopf-der-belegungsdatei-nennt-eine-annahme-als-gemessen.md` | `_c_` |
| `issues/260804-0830_o_s13-nennt-fuer-die-kommando-aufzaehlung-die-falsche-datei.md` | `_c_` |
| `issues/260804-0907_o_kopfkommentar-der-auslieferungsbelegung-nennt-c10-nicht.md` | `_c_` |
| `issues/260804-1814_o_ein-blatt-braucht-360-ms-bis-es-steht-und-l8-sagt-200-ms-zu.md` | `_c_` |
| `issues/260804-1814_o_ein-modales-blatt-widerspricht-der-zusage-dass-die-oberflaeche-bedienbar-bleibt.md` | `_c_` |

Betroffen sind `planning/260802-1036_o_spec-navigator-geruest.md` an drei Stellen
und `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` an dreizehn.

## Warum es zählt

Der Plan hält in `## Angelegte Defekte und Entscheidungen` ausdrücklich fest,
verbindlich sei der Dateibestand und nicht die Aufstellung. Das deckt die
sieben Verweise innerhalb jener Aufstellung ab, aber nicht die neun außerhalb:
sie stehen in Schrittbeschreibungen und in Festlegungen des Specs, wo ein Leser
den Pfad öffnen will und nichts findet. In sieben der zehn Fälle behauptet der
umgebende Satz außerdem einen Zustand, den es nicht mehr gibt, etwa "offen,
bindet S16" für einen geschlossenen Defekt oder "steht als offene Frage" für
eine umgesetzte Entscheidung.

## Was zu tun ist

Nicht bloß den Markerbuchstaben tauschen. In sieben Fällen ist die
umgebende Aussage mit nachzuziehen, und dafür braucht es je einen Blick in den
Datensatz, warum er geschlossen wurde. Das ist ein Abgleichdurchgang und keine
Textkorrektur; er gehört dem `reconciler`.

## Dringlichkeit

Bindet keinen Schritt. Kein Abnahmekriterium hängt an einem dieser Pfade.

---

**Aufgefallen bei:** dem Einarbeiten der sechs Nutzerantworten am 260805-0000,
bei der Schlussprüfung aller Datensatzverweise in Spec und Plan gegen den
Dateibestand. Außerhalb des damaligen Auftrags und deshalb gemeldet statt
behoben; zwei Verweise auf den Cmd+W-Defekt hat jener Durchgang mitgezogen,
weil er die umgebenden Absätze ohnehin neu geschrieben hat.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/history/260805-0000-sieben-nutzerantworten-eingearbeitet.md`
