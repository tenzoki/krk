Vierzehn tote Zeiger stehen außerhalb dieser Runde, und zwei in der Sitzungsstandsdatei

---

Der breite Fix, den `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260818-0710_*_step-16-killed-22-pointers-in-living-text-and-five-of-them-are-in-crates.md`
unter `## Fix` verlangt — eine Prüfung, die jedes Werkbank-Zitat des Baums gegen den
Dateibestand auflöst —, ist jetzt einmal gefahren. Sie findet sechzehn tote Zeiger, die keine
Reparatur dieser Runde erreicht hat: vierzehn in den Plänen und Circle-Datensätzen von fünf
anderen Runden, zwei in `fusion-workbench/agentstate.yaml`. Die zwei letzten hat diese Sitzung
selbst erzeugt.

---

**Schwere:** gering für den Baum, mittel als Beleg
**Gefunden von:** reconciler, zweiter Abgleich der Sitzung 260817-2131 (260818-0807)
**Betroffen:** die unten genannten sechs Dateien
**Domain:** code

## Gemessen, an `9ac41ea`

205 lebende Dateien gelesen (`crates/`, `xtask/`, `resources/`, `CLAUDE.md`, `README.md`, dazu
jede Werkbankdatei außerhalb der eingefrorenen Speicher `history/`, `reviews/`, `analyses/`,
`issues/`, `decisions/`, `archive/`, `messungen/`, `spikes/`). 1465 Zitate der Form
`YYMMDD-HHMM_x_<slug>` gegen den Dateibestand aufgelöst, **Zeitstempel und Namensteil**, nicht
nur der Marker; abgekürzte Schreibweisen über ihr Präfix, Zitate über einen Zeilenumbruch
zusammengesetzt. 22 Stellen aufgefallen, davon fünf zu Recht beim Buchstaben (dort ist der
Marker die Aussage) oder in einer auflösenden Kurzform. Bleiben siebzehn.

**Eine der siebzehn ist schon gefilt** —
`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md:438`, siehe
`shared/issues/260818-0752_*_ein-zitat-im-circle-datensatz-des-web-betrachters-nennt-einen-namensteil-den-es-nie-gab.md`.
**Keine der siebzehn ist eine der 22, die Schritt 16 getötet hat**; die Reparatur der Runde 12
hält vollständig, und unter `crates/`, `xtask/` und `resources/` steht kein toter Zeiger mehr.

### Vierzehn in fünf anderen Runden

| Datei | Zeile | zitiert | steht |
|---|---|---|---|
| `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260814-0656_*_plan-notizzettel-als-blatt-mit-zwei-zetteln.md` | 402 | `decisions/260814-0656_o_…` | `_i_` |
| `circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_*_bereichsleiste-und-proportionale-breitenregel.md` | 275 | `issues/260812-0548_o_…` | `_c_` |
| `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/_b_circle.md` | 70 | `shared/decisions/260816-1310_o_was-zeigt-die-eine-statuszeile-…` | `_i_` |
| `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/_b_circle.md` | 71 | `shared/decisions/260816-1310_o_sieht-der-nutzer-ob-eine-zeile-…` | `_i_` |
| `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/planning/260816-1359_*_plan-inhaltsfilter-der-dateiliste.md` | 18 | `decisions/260816-1359_a_welche-aussage-schreibt-die-dateizelle-…` | `_i_` |
| dieselbe | 18 | `decisions/260816-1359_a_in-welcher-reihenfolge-stehen-die-satzteile-…` | `_i_` |
| dieselbe | 20 | `decisions/260816-1359_o_beendet-ein-tabwechsel-den-durchlauf-…` | `_i_` |
| dieselbe | 365 | `decisions/260816-1359_a_welche-aussage-schreibt-die-dateizelle-…` | `_i_` |
| dieselbe | 380 | `decisions/260816-1359_a_in-welcher-reihenfolge-stehen-die-satzteile-…` | `_i_` |
| dieselbe | 479 | `…/decisions/260816-1359_o_beendet-ein-tabwechsel-den-durchlauf-des-verlassenen-tabs-jetzt-wo-er-dateien-liest.md` | `_i_` |
| `circles/260816-2255-befehle-absetzen-und-makros-speichern/planning/260816-2307_*_plan-befehle-absetzen-und-makros-speichern.md` | 110 | `decisions/260816-2307_o_stirbt-die-prozessgruppe-…` | `_a_` |
| dieselbe | 209 | `decisions/260816-2307_o_stirbt-die-prozessgruppe-…` | `_a_` |
| dieselbe | 424 | `decisions/260816-2307_o_stirbt-die-prozessgruppe-…` | `_a_` |
| dieselbe | 436 | `…/decisions/260816-2307_o_stirbt-die-prozessgruppe-auch-am-normalen-ende-des-laufs.md` | `_a_` |

Alle vierzehn sind derselbe Fehlertyp wie die 22: ein ausgeschriebener Marker, dessen Ziel
seinen Zustand danach gewechselt hat. Vier davon (`260816-2255`) stehen im Plan eines
zurückgestellten Circles, der aufgenommen werden soll; die anderen zehn in geschlossenen
Runden.

### Zwei in `fusion-workbench/agentstate.yaml`

Zeile 22 (`source_file:`) und Zeile 65 (`plan_file:`) nennen den Plan dieser Runde als
`…/planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`. Der erste Abgleich dieser
Sitzung hat ihn in Commit `1cef661` auf `_c_` gestellt; unter dem zitierten Namen liegt keine
Datei mehr.

**Das ist derselbe Vorgang wie bei Schritt 16, eine Ebene tiefer:** ein Markerwechsel bewegt
eine Datei, und die Zitate darauf bleiben stehen. Hier ist der Umbenenner der Abgleich selbst.
Die Felder sind maschinell gelesene Pfade, keine Prosa — dieselbe Klasse, für die
`circles/260817-0833-…/issues/260818-0753_*_die-ausnahme-fuer-maschinell-gelesene-kopffelder-steht-nur-in-einem-geschlossenen-datensatz.md`
die fehlende Regel meldet, und dort darf die Sternform gerade nicht stehen.

**Der Schaden ist begrenzt und die Lehre nicht.** `agentstate.yaml` wird beim sauberen
Sitzungsende gelöscht; die zwei toten Zeiger verschwinden dann mit ihr. Bricht die Sitzung
vorher ab, liest die Wiederaufnahme einen Pfad, unter dem nichts liegt.

## Warum das der vierte Beleg für den breiten Fix ist

`260818-0710` nennt unter `## Fix` zwei Wege: den schmalen `sed` über sechs Dateien und die
Prüfung, die jedes Werkbank-Zitat des Baums auflöst. Die Runde hat den schmalen Weg viermal
gefahren und dabei viermal etwas gefunden, das der Auftrag nicht nannte — fünf weitere unter
`crates/`, neun weitere in Spec und Plan, eine im Circle-Datensatz, und jetzt diese sechzehn.
Drei Fälle davon zeigten mit richtiger Sternform ins Leere, weil der Namensteil falsch war
(`textautomatik.rs:98`, `plan:261`, `_a_circle.md:438`); eine Prüfung über den Marker allein
findet keinen davon.

Die Erhebung dieses Datensatzes ist rund achtzig Zeilen Python und läuft in unter einer
Minute über den ganzen Baum. Sie prüft Zeitstempel **und** Namensteil, setzt umbrochene Zitate
zusammen und löst Kurzformen über ihr Präfix auf — die drei Eigenschaften, an denen die
bisherigen Erhebungen einzeln gescheitert sind.

## Fix

Vierzehn Zeilen in sechs Dateien auf die Sternform, sobald `shared/decisions/260818-0201_*_does-a-cross-references-line-between-records-write-the-marker-in-the-star-form.md`
beantwortet ist; die Frage steht für Werkbank-Prosa offen, und ein zweites Mal umzustellen wäre
die Arbeit doppelt. Die zwei in `agentstate.yaml` bleiben beim Buchstaben und brauchen keinen
Fix, sondern einen Schreiber, der sie beim Umbenennen mitzieht.

Der breite Fix bleibt, was `260818-0710` sagt: ein `xtask`-Ziel oder eine Probe, die die
Auflösung bei jedem `make check` fährt. Ohne sie ist der Baum nach jeder Runde wieder in dem
Zustand, den diese Runde zweimal repariert hat.
