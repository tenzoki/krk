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

---
Abgleich 260819-1440 (reconciler, Baumstand `77dcd48`): **offen, und die Erhebung ist wiederholt.** Über 205 lebende Dateien und 1487 Zitate: **alle vierzehn Zeiger außerhalb der Runde stehen unverändert**, in denselben fünf Dateien — `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260814-0656_c_plan-notizzettel-….md:402`, `circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_c_bereichsleiste-….md:275`, `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/_b_circle.md:70,71`, `circles/260816-1321-…/planning/260816-1359_c_plan-inhaltsfilter-….md:18,18,20,365,380,479` und `circles/260816-2255-…/planning/260816-2307_o_plan-befehle-….md:110,209,424,436`.

**Die zwei Zeiger in der Sitzungsstandsdatei sind gegenstandslos geworden:** `fusion-workbench/agentstate.yaml` besteht nicht mehr, weil der Orchestrator sie beim sauberen Sitzungsende löscht — genau der Verlauf, den dieser Datensatz vorhergesagt hat. **Kein neuer toter Zeiger ist dazugekommen.** Die eine zusätzliche Meldung der Erhebung, `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/planning/260813-0205_c_plan-suche-….md:625`, ist ein Berichtigungsvermerk, der zitiert, was die Zeile früher sagte, und fällt damit unter die Ausnahme „der Marker ist die Aussage selbst".

**Ein Hinweis für die nächste Erhebung:** die ersten zwei der vierzehn Zeiger stehen in der Kurzform mit Auslassung und entgehen einem einfachen Suchmuster über den vollen Namensteil. Das ist dieselbe Falle, die `shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md` beschreibt.

---
**Zur Hälfte behoben am 260906, und der Marker bleibt deshalb auf `_o_`.**

**Die Zeiger sind auf die Sternform gezogen.** Sechzehn statt der genannten vierzehn, in denselben fünf Dateien; die Erhebung ist über alle 73 lebenden Werkbankdateien wiederholt, mit Auflösung nach Zeitstempel **und** Namensteil und mit den Kurzformen über ihr Präfix.

**Zwei Abweichungen zur Tabelle dieses Datensatzes, beide nachgemessen:**

- Zwei Stellen kommen hinzu, die die Tabelle nicht führt: `260816-1321-…/_b_circle.md:68` (`260816-1310_a_welche-vorhandene-groessengrenze-…`, steht `_i_`) und `260816-2255-…/planning/260816-2307_*_plan-befehle-…md:267` (`260816-2307_o_der-doc-kommentar-von-ablage-pfad-…`, steht `_c_`). Beide sind seit dem 260819 gestorben.
- Die zweite der zwei Kurzformen ist **kein** toter Zeiger: `260813-2332-…/planning/260814-0656_*_plan-notizzettel-….md:394` zitiert `shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-…`, und dieser Datensatz steht unverändert auf `_o_`. Tot war allein die Stelle an `:402`.

Ein siebzehnter Ersatz betrifft keinen toten Zeiger: die vierte Zeile derselben Aufzählung im `_b_circle.md` der Runde 11 ist mitgezogen, weil drei Sterne neben einem ausgeschriebenen Buchstaben in derselben Liste schlechter lesbar sind als vier Sterne.

**`**Active spec/plan:**` der Runde 16 ist auf den Buchstaben nachgezogen, den die Datei trägt** (`_o_` → `_p_`, `circles/260823-2208-…/_b_circle.md:6`). Das Feld ist ein wörtlich gelesener Pfad und bleibt beim Buchstaben, wie `260818-0753_*_die-ausnahme-fuer-maschinell-gelesene-kopffelder-steht-nur-in-einem-geschlossenen-datensatz.md` es beschreibt; tot war es trotzdem. Dass Plan und Spec jener geschlossenen Runde auf `_p_` und `_o_` stehen, ist ein eigener Befund und abgelegt.

**Die Vorbedingung, auf die der `## Fix` wartet, ist nicht eingetreten und war für diese sechzehn auch nicht nötig.** `260818-0201_*_does-a-cross-references-line-…` fragt nach der `**Cross-references:**`-Zeile in einem eingefrorenen Speicher. Keine der sechzehn Stellen ist eine; alle stehen in Plänen, Spec-Dateien und Circle-Datensätzen, und für die gilt die Antwort vom 260815-1230 unmittelbar — sie nennt „die Circle-Datensätze und die Spec- und Plandateien unter `planning/`" in ihrem Geltungsbereich.

**Offen bleibt der breite Fix, und er ist jetzt genauer beschreibbar.** Über die lebenden Werkbankdateien stehen **110 Zitate mit ausgeschriebenem Marker in 34 Dateien** (gemessen am 260906-0210); sieben davon zeigen ins Leere, und die sieben sind sämtlich die Klasse „der Marker ist die Aussage": sechs Zeilen der Tafel `## Datensätze` im Plan der Runde 14, deren Spalten „Marker heute" und „Marker danach" heißen, und ein Berichtigungsvermerk im Plan der Runde 7. Eine Probe, die die Auflösung erzwingt, muss diese Klasse mechanisch erkennen können, und dafür fehlen zwei Antworten: die Ausnahme für wörtlich gelesene Kopffelder (`260818-0753_*_…`, offen) und die Gestalt, in der eine Aussage über ein Zitat geschrieben wird (`260818-0201_*_…`, offen). Solange beide offen sind, wäre die Probe entweder rot an sieben richtigen Stellen oder trüge eine Ausnahmeliste, die niemand pflegt.
