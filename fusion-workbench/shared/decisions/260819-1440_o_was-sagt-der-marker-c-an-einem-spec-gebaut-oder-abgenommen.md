# Was sagt der Marker `_c_` an einem Spec — „gebaut und belegt" oder „abgenommen"?

---
**Domain:** code
**Status:** open
**Filed by:** reconciler
**Cross-references:** `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/planning/260811-1552_o_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md:4`; `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md`, `## Reconciliation Log` vom 260814-1247; `shared/planning/260818-1510_c_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md`, `## Abgleich 260819-0057`; `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/_c_circle.md`, `## Closure note`; `rules/fusion-workbench-conventions.md`, `### Planning files`

---

## Question

Sieben Specs und eine Abnahmeanleitung tragen im Dateinamen `_o_`, obwohl ihre Runde geschlossen ist. Zwei Specs geschlossener Runden tragen `_c_`. Beide Gruppen sind mit Begründung so gesetzt worden, und die zwei Begründungen widersprechen einander. Der Widerspruch ist bisher nirgends als solcher aufgeschrieben; jeder Abgleich hat für sich entschieden, und die Wahl fiel nach dem Datum.

**Die zwei Lesarten, jede im Baum ausgeschrieben:**

**Lesart A — der Marker folgt den Abnahmekriterien.** Ausgeschrieben in der Kopfzeile des Specs der Runde 4: „Der Marker bleibt `_o_`, bis die Abnahmekriterien eingelöst sind." Bekräftigt vom zweiten Abgleich der Runde 9, der den Marker gemessen stehen lässt: der Abnahmelauf war gefahren, deckte aber nur 8 von 29 Bündelkriterien, also bleibt `_o_`. Nach dieser Lesart sind die sieben offenen Marker kein Rückstand, sondern eine Aussage.

**Lesart B — der Marker folgt der belegten Bauarbeit.** Ausgeschrieben in der Schließungsnotiz des Specs der Runde 13: „Der Marker `_c_` sagt hier ‚die Runde ist gebaut und ihre Schritte sind belegt' und nicht ‚abgenommen'." Nach dieser Lesart sind die sieben offenen Marker ein Rückstand von sieben Runden.

**Die Konvention entscheidet die Frage nicht.** `rules/fusion-workbench-conventions.md`, Abschnitt `### Planning files`, sagt: „When all steps are `[DONE]`: set `**Status:** Complete` … and rename the filename marker to `_c_`." Die Regel ist für **Schritte** geschrieben, und ein Spec hat keine; er hat Abnahmekriterien. Für Pläne trägt sie und ist im ganzen Baum befolgt — alle Pläne geschlossener Runden stehen auf `_c_`. Für Specs schweigt sie.

**Warum die Frage jetzt gestellt wird und nicht früher.** Bis zur Runde 11 fiel die Antwort mit dem Circle-Marker zusammen und war deshalb nie sichtbar. Die Runden 12 und 13 haben das getrennt: die Runde 12 hat kohärent geschlossen **ohne** gefahrenen Abnahmelauf, und ihr Circle-Datensatz schreibt ausdrücklich aus, dass `_c_` dort nicht „abgenommen" heißt („Wer künftig `_c_` als ‚vom Nutzer abgenommen' liest, liest diese Runde falsch"). Damit ist der Circle-Marker als Ersatzauskunft weg, und die Frage steht für den Spec-Marker allein.

## Der Bestand am 260819-1440

| Runde | Circle | Spec | Abnahmelauf gefahren |
|---|---|---|---|
| 1 | `_b_` | `_c_` | ja, bis auf die Zeitzusagen aus C8 |
| 2 | `_b_` | `_o_` | nein |
| 3 | `_b_` | `_o_` (dazu die Abnahmeanleitung `_o_`) | nein |
| 4 | `_b_` | `_o_` | nein |
| 5 | `_b_` | kein Spec, nur ein Plan (`_c_`) | nein |
| 6 | `_b_` | kein Spec, nur ein Plan (`_c_`) | nein |
| 7 | `_b_` | `_o_` | nein |
| 8 | `_c_` | `_c_` | ja |
| 9 | `_b_` | `_o_` | ja, deckt 8 von 29 Bündelkriterien |
| 10 | `_b_` | `_o_` | nein |
| 11 | `_b_` | `_o_` | nein |
| 12 | `_c_` | `_c_` | **nein** |
| 13 | `_c_` | `_c_` | ja |

Die Runden 5 und 6 fallen heraus: sie haben keinen Spec, sondern führen ihre Abnahmekriterien im Plan, und für Pläne ist die Regel eindeutig.

**Die Runden 7 und 11 sind der wunde Punkt beider Lesarten.** Ihre Specs (`shared/planning/260813-0053_o_…`, `shared/planning/260816-1310_o_…`) tragen als einzige **keinen** `## Reconciliation Log`. Sie sind nie beurteilt worden; ihr `_o_` ist nicht gesetzt, sondern stehen geblieben.

## Options

1. **Lesart A festschreiben: der Spec-Marker folgt den Abnahmekriterien.** Ein Spec geht auf `_c_`, wenn seine Kriterien eingelöst sind, und bleibt sonst `_o_`, gleich wie die Runde geschlossen hat. Die Runden 12 und 13 werden nachträglich beurteilt; die Runde 12 fiele dabei auf `_o_` zurück.
   - Pro: Der Marker trägt die einzige Auskunft, die dieses Projekt sonst nirgends mechanisch führt, nämlich ob der Abnahmelauf gefahren ist. Sieben Marker bleiben, wie sie stehen, und zwei Dateien werden angefasst.
   - Kontra: `_o_` heißt in der Konvention „offen, Anfangszustand", und ein Spec, dessen Runde vor sechs Wochen geschlossen hat, ist in keinem üblichen Sinn im Anfangszustand. Jede Zählung offener Planungsarbeit über den Marker meldet sieben Posten, an denen niemand arbeitet und niemand arbeiten wird. Ein Rückfall von `_c_` auf `_o_` ist daneben eine Rückwärtsbewegung, die die Konvention für Defekte ausdrücklich nicht kennt.
   - Was sie verbaut: den Marker als Auskunft über die Bauarbeit. Wer wissen will, ob ein Spec gebaut ist, muss dann immer den Plan danebenlegen.

2. **Lesart B festschreiben: der Spec-Marker folgt der belegten Bauarbeit.** Ein Spec geht auf `_c_`, sobald alle Schritte seines Plans am Baum belegt sind. Die sieben offenen Marker werden in einem Durchgang nachgezogen.
   - Pro: Ein Marker, ein Kriterium, und dasselbe Kriterium wie beim Plan. Die Zählung offener Planungsarbeit meldet dann, woran wirklich gearbeitet wird. Es ist die Lesart der beiden jüngsten Runden.
   - Kontra: Die Abnahme verliert ihren einzigen mechanischen Träger. Dieses Projekt unterscheidet „gebaut" und „abgenommen" an jeder Stelle sorgfältig, und nach dieser Lesart steht die Unterscheidung nur noch in Prosa. Sieben Dateien werden angefasst, darunter eine, deren Kopfzeile die Gegenregel ausschreibt und die dabei zu berichtigen wäre.
   - Was sie verbaut: die Auskunft „dieser Spec wartet auf den Nutzer", solange kein zweiter Träger dafür entsteht.

3. **Beide Auskünfte trennen: der Marker folgt der Bauarbeit, die Abnahme bekommt eine eigene Kopfzeile.** Wie Möglichkeit 2, dazu eine Pflichtzeile im Spec-Kopf, etwa `**Abnahme:** nicht gefahren | gefahren am <Datum>, <n> von <m> Kriterien`.
   - Pro: Löst den Grund des Streits statt der Wahl. Beide Auskünfte stehen weiter zur Verfügung, jede an ihrer eigenen Stelle, und keine überlädt die andere. Die Zeile ist daneben genauer, als ein Marker je sein könnte: die Runde 9 hat einen gefahrenen Lauf, der 8 von 29 Kriterien deckt, und kein Marker kann das sagen.
   - Kontra: Die teuerste der drei. Dreizehn Spec-Köpfe bekommen eine Zeile, und eine Kopfzeile ohne Prüfung läuft auseinander — dieses Projekt führt dafür einen offenen Datensatz (`shared/issues/260814-1955_*_sechs-beantwortete-entscheidungsdatensaetze-tragen-im-kopf-weiter-status-open.md`), und die Gestalt ist heute in 19 von 27 Defektdatensätzen gemessen.
   - Was sie verbaut: nichts Erkennbares; sie schließt die anderen beiden ein.

## Constraints

- **Kein Marker wird ohne Beleg bewegt.** Welche Lesart auch gilt: für jeden der sieben Specs muss die gewählte Bedingung einzeln am Baum nachgelesen sein, bevor der Name sich ändert.
- **Umbenennungen erzeugen tote Zeiger.** Der Baum führt heute 14 tote Zeiger (`shared/issues/260818-0807_*_…`) und 62 ausgeschriebene Marker in Zitaten (`shared/issues/260817-1130_*_…`). Ein Durchgang über sieben Dateien muss die Zitate mitziehen oder auf die Sternform stellen.
- **Die Kopfzeile `**Status:**` zieht mit.** Drei Specs trugen bis zum Abgleich vom 260819-1440 im Kopf „Entwurf" bei einem Dateimarker `_c_`; das ist dort berichtigt und darf nicht neu entstehen.

## Recommendation

**Möglichkeit 3**, und wenn ihr Preis zu hoch ist, Möglichkeit 2. Der Streit entsteht nicht daraus, dass jemand falsch gewählt hätte, sondern daraus, dass ein Marker mit vier Werten zwei unabhängige Fragen beantworten soll: „ist es gebaut?" und „ist es abgenommen?". Solange beide an ihm hängen, liefert jede Wahl eine richtige und eine falsche Auskunft, und der nächste Abgleich wählt wieder neu. Möglichkeit 2 ist die billigere Hälfte davon und für sich brauchbar, weil dieses Projekt die Abnahme ohnehin je Runde in der Schließungsnotiz des Circle-Datensatzes ausschreibt.

**Was dieser Abgleich getan hat, solange die Frage offen ist:** nichts umbenannt. Ein Durchgang über die sieben in eine der beiden Richtungen wäre die stille Festlegung auf eine Lesart, und der Nutzer hat sie nicht getroffen.

---
Answered:
Implemented:
Deferred:
Superseded by:
