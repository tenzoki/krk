# Durchsicht der Zeigerreparatur (Buendel G)

**Reviewed-range:** `1cef661..0494604`
**Not-opened:** none
**Sender:** coderev
**Datum:** 260818-0754

## Zusammenfassung

Die Reparatur haelt. Alle 22 gemeldeten Zeiger und die zehn selbst gefundenen loesen auf, die
drei absichtlichen Ausnahmen sind richtig gesetzt, keine Aussage ist durch einen Stern geloescht
worden, und `make check` laeuft gruen. Zwei Befunde bleiben, beide klein und beide ausserhalb
dessen, was die zwei Commits angefasst haben.

**Der Bereich aendert keine ausfuehrbare Zeile.** Die sechs Dateien unter `crates/` sind
ausschliesslich Modulkoepfe und Doc-Kommentare; `git diff` zeigt keine Zeile Code.

## Zahlen

| | |
|---|---|
| Zitate in `crates/`, `xtask/`, `resources/` aufgeloest | 428 |
| Zitate in den 46 nicht eingefrorenen Werkbankdateien aufgeloest | 1016 |
| tote Zeiger danach in `crates/`, `xtask/`, `resources/` | 0 |
| tote Zeiger danach in der Werkbank-Prosa | 1 |
| Befunde | 1 mittel, 1 gering |

## Die Pruefung

Jedes Vorkommen der Form `YYMMDD-HHMM_x_<slug>` ist gegen den Dateibestand aufgeloest worden,
**Zeitstempel und Namensteil**, nicht nur der Marker. Abgekuerzte Schreibweisen (`…`, `...`,
`-*`) sind ueber ihr Praefix aufgeloest, mehrdeutige Praefixe einzeln nachgelesen. Eingefroren
nach der Ortsregel in `CLAUDE.md` `## Bindende Grundlage` und damit ausgenommen: `history/`,
`reviews/`, `analyses/`, `issues/`, `decisions/`, `archive/`.

Das ist der Punkt, auf den es ankam: **zwei der reparierten Stellen trugen schon die Sternform
und zeigten trotzdem ins Leere**, `crates/krk-ui/src/appkit/textautomatik.rs:98` und `plan:261`.
Eine Markerpruefung haette beide durchgelassen. Der Befund, den diese Durchsicht hinzufuegt,
ist von derselben Art.

## Was geprueft und richtig befunden ist

**Die drei absichtlichen Ausnahmen halten.** `plan:555` und `:556` messen, in welchem Commit
ein `_c_`-Pfad zuerst steht (`git log --diff-filter=A --no-renames`); dort ist der Marker die
Aussage, und ein Stern loeschte den Inhalt. Das Kopffeld `**Active spec/plan:**` in
`_t_circle.md:7` wird von drei Verbrauchern woertlich als Pfad gelesen
(`agents/orchestrator.md:275` und `:407`, `skills/next/SKILL.md:232`, `skills/migrate/SKILL.md:99`),
und beide darin genannten Pfade loesen heute auf. Beide Begruendungen sind einzeln nachgelesen
und nicht uebernommen.

**Eine vierte Stelle behaelt den Buchstaben zu Recht und wird im Abschlussvermerk nicht genannt:**
`plan:585` fuehrt in der Abgleichszeile zu Schritt 16 `260802-0842_s_…` und `260817-0536_i_…`.
Der Satz behauptet gerade, dass die fuenf Dateien diese Marker tragen — auch hier ist der Marker
die Aussage. Die Reparatur hat sie richtig stehen lassen; nur die Zaehlung „drei Stellen" im
Vermerk ist damit eng.

**Keine Aussage ist zerstoert worden.** Die zwei Grenzfaelle sind einzeln geprueft. `plan:585`
im Absatz „Vier Entscheidungsdatensaetze bleiben auf `_a_`" ist gesternt, aber die Aussage steht
im Prosatext daneben und ist datiert (260817-1833), also unbeschaedigt. `_t_circle.md:30` sagt
weiter „Der Datensatz traegt `_i_`", waehrend `:54` den neuen Stand `_s_` nennt; das ist kein
Widerspruch, sondern die Trennung zwischen `## Grounding snapshot` (auf `b8e198e` datiert) und
`## Dependencies` (heutiger Stand). Die Reparatur hat genau die richtige der beiden Stellen
nachgezogen.

**Die zwei Musterzitate treffen genau die vier Datensaetze.** `plan:303` und `:316` lauten
`shared/decisions/260817-0536_*_*.md` beziehungsweise `shared/decisions/260817-0536_*_…`. Der
Pfadanteil beschraenkt sie auf `shared/decisions/`, wo genau vier Dateien mit diesem Zeitstempel
liegen; der Spec derselben Kennung liegt in `shared/planning/` und wird nicht mitgefangen.

**`make check` — Exit 0**, alle vier Abnahmekommandos, 21 Probenziele gruen, keine Warnung. Die
Wettrennprobe `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` ist durchgelaufen; der
aufgenommene Ausfall aus `shared/issues/260816-0055_*_…` ist in diesem Lauf nicht aufgetreten.

**`plan:310` ist nicht gefildet.** Der Satz zeichnet einen Stand zur Planzeit im Feld `Changes`
eines `[DONE]`-Schritts auf. Die Einordnung des ausfuehrenden Agenten trifft zu.

## Befunde

**1 — Ein Zitat im Circle-Datensatz des Web-Betrachters nennt einen Namensteil, den es nie gab.**
Mittel. `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md:438`
zitiert `260812-2133_*_merkzeichen-einloesen-kostet-bei-tiefer-verschachtelung-mehr-als-der-rest-der-zerlegung.md`;
der Datensatz heisst seit seiner ersten Fassung `…das-zweieinhalbfache-und-verfehlt-l7-frueher.md`.
Das Zitat traegt bereits die Sternform. Geschrieben am 260812 in `0e09377`, also fuenf Tage vor
dieser Sitzung und ausserhalb des Bereichs. Der Ort macht ihn teuer: ein `_a_circle.md` wird bei
der Aktivierung als bindende Grundlage gelesen. **Es ist der dritte Beleg desselben Fehlertyps in
zwei Tagen und damit das dritte Argument fuer den breiten Fix, den `260818-0710` unter `## Fix`
verlangt.** Datensatz:
`shared/issues/260818-0752_*_ein-zitat-im-circle-datensatz-des-web-betrachters-nennt-einen-namensteil-den-es-nie-gab.md`.
Er liegt im gemeinsamen Speicher und nicht in diesem Circle, weil er nicht aus dieser Directive
entstanden ist, sondern neben ihr gefunden wurde (Herkunftsregel).

**2 — Die Ausnahme fuer maschinell gelesene Kopffelder steht nur in einem geschlossenen
Datensatz.** Gering. Die Begruendung fuer `**Active spec/plan:**` stimmt, aber sie ist eine
**zweite** Ausnahmeklasse neben der, die `shared/decisions/260815-1145_*_…:54` fuehrt: nach
dessen Probe („ein Zeiger auf eine Datei verliert nichts") gehoerte das Feld in die Sternform.
Dieselbe Sitzung hat den Plankopf `**Spec:**`, ein Kopffeld derselben Bauart, gesternt — heute
folgenlos, weil kein Verbraucher ihn liest, aber die Unterscheidung steht nirgends geschrieben.
Ein spaeterer Durchgang, der die Regel woertlich liest, sternt `_t_circle.md:7` und bricht einen
woertlichen Dateizugriff, still. Datensatz:
`issues/260818-0753_*_die-ausnahme-fuer-maschinell-gelesene-kopffelder-steht-nur-in-einem-geschlossenen-datensatz.md`.

## Reihenfolge

Kein Auslieferungshindernis. Beide Befunde sind Aufraeumarbeit. Befund 1 ist eine Zeile und
gehoert dem Nutzer oder dem Orchestrator, der den Datensatz besitzt. Befund 2 ist eine Zeile in
`CLAUDE.md` und gehoert an das naechste Gate zur Pflege der normativen Flaechen.
