# Reconciliation — Sitzung 260817-2131, zweiter Durchgang nach dem Rebalance-Gate

**Datum:** 260818-0807
**Status:** Complete
**Domain:** code
**Bereich:** `cdde9da..9ac41ea`, 20 Commits, vier Turns
**Baumstand:** `9ac41ea`, Arbeitsbaum mit zwei ungesicherten Werkbankdateien (`_t_circle.md`, `orchestrator-events.jsonl`)
**Verification:** `make check` — Exit 0

Der erste Abgleich dieser Sitzung (`history/260818-0712-reconciliation.md`) meldete
`review-needed` mit drei Driftpunkten. Der Nutzer hat am Rebalance-Gate „Artefakt überarbeiten"
gewählt und dabei die Zeigerreparatur benannt; Turn 4 hat sie in `adf638b`, `0494604` und
`9ac41ea` gefahren. Dieser Durchgang prüft die drei Punkte gegen den Baum nach, statt den
Berichten zu glauben.

## Zum Abnahmekommando

`make check` ist zweimal gefahren. Der erste Lauf um 0757 fiel aus, an der Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` (`crates/krk-core/tests/text.rs:870`,
Abbruch nach 15,07 s an der Notbremse). Der zweite Lauf um 0805 lief durch, Exit 0, alle vier
Abnahmekommandos grün. Die Probe allein gefahren: Exit 0 in 13,72 s.

**Das ist kein Rückschritt dieser Sitzung, sondern der aufgenommene Ausfall aus
`shared/issues/260816-0055_*_die-wettrennprobe-ein-wechsel-der-art-unter-dem-oeffnen-faellt-gelegentlich-aus.md`**,
offen seit dem 260816. Der Datensatz misst dort vier Ausfälle in fünf Läufen am
Arbeitsbaum und zwei in drei Läufen an einem unveränderten HEAD **vor** dieser Runde; jeder
Ausfall endet bei 15,0 s an derselben Notbremse. Die Laufzeit von 13,72 s im geglückten Lauf
liegt in derselben Streuung. Der Prüfling ist nicht betroffen, das Abnahmekommando ist es.

## Was geprüft wurde

| Speicher | Gelesen | Geändert |
|---|---|---|
| Pläne (Circle + gemeinsam) | 5 | 1 (zweiter Abgleichseintrag angehängt) |
| Defektdatensätze | 31 im Circle, 30 im gemeinsamen Speicher | 2 angehängt, 2 neu gefilt |
| Entscheidungsdatensätze | 3 im Circle, 29 offen über alle Speicher | 0 |
| Durchsichten (Circle) | 7, davon 4 in dieser Sitzung | 0 |
| Circle-Datensatz | 1 | 0 (ein Befund gefilt statt geschrieben) |
| Ereignisprotokoll | 1113–1184, die Sitzung | 0 |

## Die eigene Auflösung der Zeiger

Nicht der Marker allein, sondern **Zeitstempel und Namensteil** gegen den Dateibestand. 205
lebende Dateien gelesen: `crates/`, `xtask/`, `resources/`, `CLAUDE.md`, `README.md`, dazu jede
Werkbankdatei außerhalb der eingefrorenen Speicher nach der Ortsregel in `CLAUDE.md`
(`history/`, `reviews/`, `analyses/`, `issues/`, `decisions/`, `archive/`, `messungen/`,
`spikes/`). 1465 Zitate der Form `YYMMDD-HHMM_x_<slug>` aufgelöst, Kurzformen über ihr Präfix,
umbrochene Zitate über Zeilengrenzen zusammengesetzt.

Ergebnis: 22 Stellen aufgefallen, davon fünf zu Recht stehend (an drei ist der Marker die
Aussage selbst, zwei sind auflösende Kurzformen). Bleiben siebzehn tote Zeiger — **keiner davon
einer der 22, die Schritt 16 getötet hat.**

## Driftpunkt 1 — die 22 toten Zeiger: behoben

**Behoben, und die Reparatur reicht weiter als der Auftrag.** Die beiden Commits stellen 38
Zitate auf die Sternform: `adf638b` zehn unter `crates/`, `0494604` 28 in Spec, Plan und
Circle-Datensatz. Jedes entfernte Token trägt einen ausgeschriebenen Marker oder einen falschen
Namensteil, jedes hinzugefügte die Sternform, und die Zahlen stimmen je Datei überein.

Unter `crates/`, `xtask/` und `resources/` steht kein toter Zeiger mehr; im Plan, im Spec und im
Circle-Datensatz auch nicht. Das ist unabhängig gemessen und nicht aus der Durchsicht übernommen.

**Die Reparatur hat nichts kaputtgemacht.** Die vier Stellen, an denen der Marker die Aussage
ist, tragen ihn weiter:

| Stelle | was sie behauptet |
|---|---|
| `plan:553`–`:556` | in welchem Commit der `_c_`-Pfad eines Datensatzes zuerst steht |
| `plan:585` | dass die fünf bewegten Dateien `_s_` und `_i_` im Namen tragen |
| `_t_circle.md:7` | maschinell gelesenes Kopffeld `**Active spec/plan:**`, wörtlich als Pfad gelesen |
| `plan:5` | Kopffeld `**Spec:**` — hier **richtig** gesternt, weil kein Verbraucher ihn wörtlich liest |

Beide im Kopffeld genannten Pfade lösen heute auf. Der Kopfeintrag `**Spec:**` steht in der
Sternform und ist damit die Gegenprobe zur vierten Zeile: dieselbe Bauart, andere Behandlung,
und der Unterschied ist begründet — er steht aber nirgends geschrieben, was
`issues/260818-0753_*_die-ausnahme-fuer-maschinell-gelesene-kopffelder-…` meldet.

## Driftpunkt 2 — die 43 Abschlussvermerke: unverändert, bewusst

Nachgemessen an `9ac41ea`: **43 von 429** geschlossenen Defektdatensätzen entgehen einer
`^Resolved:`-Suche, gegenüber 43 von 428 im ersten Durchgang. Der eine hinzugekommene ist
`issues/260818-0710_*_step-16-killed-22-pointers-…`, und er trägt seinen Vermerk in der
Konvention. Turn 4 hat die Quote also nicht verschlechtert.

Nicht behoben, und das ist richtig: der Nutzer hat am Gate die Zeigerreparatur benannt und nicht
diese. Die Nachmessung ist an
`shared/issues/260818-0710_*_forty-three-closure-notes-are-written-in-a-form-no-resolved-sweep-finds.md`
angehängt; der Datensatz bleibt offen.

## Driftpunkt 3 — die fehlenden Commit-Ereignisse: unverändert, und die engere Fassung fällt

**Jetzt vier von 20 statt drei von 16.** Ohne `commit`-Ereignis sind `8f556ed`, `f79f964`,
`b0eee2c` und neu `1cef661`, der Commit des ersten Abgleichs; für ihn steht auf derselben
Sekunde ein `reconciliation`-Ereignis, aber kein `commit`.

**Die Fassung „das Ablegen einer Durchsicht emittiert kein Commit-Ereignis", die der erste
Durchgang als engeren Befund vorschlug, hält nicht.** `e843d90` und `9ac41ea` legen ebenfalls
eine Durchsicht ab und tragen beide ihr Ereignis. Die Lücke sitzt in den Turns 1 und 2 und im
Nachlauf, nicht an einer Commit-Art. Daneben geht die Paarung der Aufgabenereignisse nicht auf:
15 `task_start` gegen 16 `task_done`, wobei `F-7` und `R-3` ein `task_done` ohne `task_start`
tragen und `F-5` mit `task_error` endet. Als zweiter `Also seen:` an
`shared/issues/260810-1945_*_der-orchestrator-hat-in-drei-turns-keine-aufgabenereignisse-emittiert.md`
angehängt.

## Der Turn-Log des Circle-Datensatzes

**Die vier Commit-Bereiche stimmen, jeder gegen das Ereignisprotokoll gelesen.**

| Turn | Turn-Log | Ereignisprotokoll |
|---|---|---|
| 1 | `82707ef`..`f7a85c1`, dazu `8f556ed` | zwei `commit`, Durchsicht `8f556ed` (ohne Ereignis) |
| 2 | `522cf51`..`da716c1`, dazu `f79f964` | drei `commit`, Durchsicht `f79f964` (ohne Ereignis) |
| 3 | `ae665e5`..`a4d8211`, dazu die Durchsicht | acht Commits plus `e843d90`; `turn_end` sagt „9 commits" |
| 4 | `adf638b`..`0494604`, dazu die Durchsicht | `turn_end` sagt „3 commits adf638b 0494604 9ac41ea" |

**Die Kohärenz-Befunde stimmen ebenfalls**: viermal `verdict=ok` im Protokoll, viermal
„Kohärenz-Befund `ok`" im Turn-Log. Die Nebenzahlen von Turn 3 (30 geschlossen, 6 Datensätze aus
der Durchsicht, zwei mittel, vier neue Entscheidungsfragen, ein Verbindungsabbruch mit
gerettetem Teil `48bb57f`) und von Turn 4 (zwei Datensätze, einer mittel, beide außerhalb des
Bereichs) decken sich Wort für Wort mit `review_done`, `coherence_review` und `turn_end`.

**Eine Zahl stimmt nicht.** Der Eintrag zu Turn 4 sagt „dazu zehn weitere, die zwei eigene
Erhebungen mitbrachten"; gestellt sind sechzehn weitere. Gefilt als
`issues/260818-0807_*_der-turn-log-nennt-zehn-weitere-zeiger-die-zwei-commits-haben-sechzehn-gestellt.md`;
der Datensatz gehört dem Orchestrator, der die Datei besitzt.

Die Formulierung „Drei Stellen behalten den Buchstaben mit Absicht" im selben Eintrag zählt
Absätze und nicht Zeilen und ist damit eng, aber nicht falsch — die Tabelle oben zeigt vier
Zeilen in drei Absätzen plus das Kopffeld.

## Die acht offenen Befunde: es sind sieben im Circle

Im Speicher des Circles stehen **sieben** offene Datensätze: `260818-0410` bis `260818-0415`
aus der Durchsicht des Bündels F, dazu `260818-0753` aus der Durchsicht des Bündels G. Der
achte, den die Aufgabe vermutet, ist `shared/issues/260818-0752_*_ein-zitat-im-circle-datensatz-des-web-betrachters-…`
und liegt nach der Herkunftsregel im gemeinsamen Speicher, weil die Durchsicht ihn neben der
Directive und nicht aus ihr gefunden hat. Über beide Speicher gezählt sind es acht.

Alle sieben sind bewusst offen und für eine spätere Sitzung; keiner ist ein
Auslieferungshindernis, zwei sind mittel. Keiner ist eine Entscheidungsfrage im Defektgewand.

## Die offenen Entscheidungsfragen: es sind vier

Diese Sitzung hat vier Entscheidungsdatensätze angelegt, nicht fünf, gezählt mit
`git diff --diff-filter=A cdde9da..HEAD`:

| Datensatz | Speicher | löst auf | widerspricht der Directive |
|---|---|---|---|
| `260818-0249_*_bekommen-die-zwei-polaritaeten-des-loeschzielbefunds-zwei-typen` | Circle | 3/3 Zitate | nein — Verschärfung derselben Sicherung |
| `260818-0250_*_verlangt-der-blattbauer-die-liegenlassende-schaltflaeche-am-typ` | Circle | 2/2 | nein — stützt „Abbrechen vorbelegt" |
| `260818-0512_*_wie-lautet-die-frage-wenn-der-umfang-der-genannte-grund-ist-…` | Circle | 2/2 | nein — Wortlaut der Frage, deren Pflicht die Directive setzt |
| `260818-0201_*_does-a-cross-references-line-between-records-write-the-marker-…` | gemeinsam | 7/7 | nein — Werkbank-Schreibweise, berührt die Directive nicht |

Jedes Zitat in jedem der vier ist gegen den Dateibestand aufgelöst worden: 14 Zitate, 0 tot.
Das Ereignisprotokoll nennt für Turn 3 „4 new open decisions" und für Turn 4 „no decision
touched"; beides deckt sich.

## Neue Abweichungen

1. **Der Turn-Log nennt zehn weitere Zeiger, gestellt sind sechzehn.** Gefilt als
   `issues/260818-0807_*_der-turn-log-nennt-zehn-weitere-zeiger-…`.
2. **Vierzehn tote Zeiger stehen in lebendem Text außerhalb dieser Runde**, in den Plänen und
   Circle-Datensätzen der Runden 5, 9, des Inhaltsfilter-Circles und des zurückgestellten
   Circles „Befehle absetzen". **Dazu zwei in `fusion-workbench/agentstate.yaml`**, die diese
   Sitzung selbst erzeugt hat: die Zeilen 22 und 65 nennen den Plan noch als `_o_`, den der
   erste Abgleich in `1cef661` auf `_c_` gestellt hat. Gefilt als
   `shared/issues/260818-0807_*_vierzehn-tote-zeiger-stehen-ausserhalb-dieser-runde-und-zwei-in-der-sitzungsstandsdatei.md`,
   im gemeinsamen Speicher nach der Herkunftsregel. Es ist der vierte Beleg für den breiten Fix,
   den `260818-0710` unter `## Fix` verlangt.
3. **Der Ausfall von `make check` im ersten Lauf**, oben behandelt. Kein neuer Datensatz: er
   steht seit dem 260816 offen.

## Nicht angefasst

**Der Spec bleibt auf `_o_`, und der Grund hat sich gegenüber dem ersten Durchgang geändert.**
`shared/planning/260817-0536_*_spec-absicherung-jedes-loeschwegs.md` ist mit dem vollständigen
Plan inhaltlich erfüllt. Ihn jetzt umzubenennen bräche aber `_t_circle.md:7`, wo das Kopffeld
`**Active spec/plan:**` seinen Pfad wörtlich mit dem Buchstaben führt und von vier Verbrauchern
als Pfad gelesen wird — genau der Vorgang, den Schritt 16 an fünf anderen Dateien ausgelöst hat.
Die Umbenennung gehört an den Abschluss des Circles, wo derselbe Schreiber beide Dateien in der
Hand hat.

**Der Circle-Datensatz ist nicht geschrieben.** Der Zahlenbefund ist gefilt statt eingetragen;
die Datei gehört dem Orchestrator.

**Nichts ist neu misfiled.** Kein offener Defekt in einem der beiden Speicher ist eine
Entscheidungsfrage im Defektgewand. Der eine Fall dieser Runde, `260817-1720_c`, ist im ersten
Durchgang behandelt und als `decisions/260818-0512_*_…` am richtigen Ort.

## Zum Abnahmelauf: die Entscheidung des ersten Durchgangs bleibt

Der Abnahmelauf der zehn Zeitzusagen aus C8 ist **nicht** in das Verdikt gefaltet. Der Grund ist
unverändert und trägt weiter: die Directive dieser Runde sagt über die zehn Zusagen nichts, und
was unerreichbar ist, ist die Abnahme durch den Nutzer und nicht die Directive. Alles, was die
Directive verlangt, ist gebaut, grün und einzeln gegen den Baum gelesen.

Der Lauf verlangt KRK im Vordergrund und ist damit Nutzerarbeit (`CLAUDE.md`, „Was man nicht
sieht"); zuletzt gefahren am 260810, sechs Runden zurück. Das ist die Bedingung, unter der zehn
der elf bisher gefahrenen Runden dieses Projekts beschränkt geschlossen haben, nachgezählt mit
`ls fusion-workbench/circles/*/_b_circle.md`. Wo diese Runde landet, entscheidet der
Orchestrator beim Abschluss und nicht dieser Abgleich.

## Verdikt

`coherent`. Der Befund steht im Sitzungsprotokoll
`history/260817-2131-orchestrator-session.md` unter `## Coherence` und ersetzt dort den des
ersten Durchgangs.
