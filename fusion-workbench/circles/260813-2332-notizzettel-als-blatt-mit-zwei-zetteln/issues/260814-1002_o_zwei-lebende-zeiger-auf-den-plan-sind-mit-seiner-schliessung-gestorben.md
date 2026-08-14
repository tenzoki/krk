Zwei lebende Zeiger auf den Plan sind mit seiner Schließung gestorben

---

Der Abgleich vom 260814-1002 hat den Plan von `_o_` auf `_c_` gezogen, weil alle sechzehn
Schritte auf `[DONE]` stehen und am Baum bestätigt sind. Zwei Zeiger auf die Datei sind damit
tot, und beide gehören nicht dem Abgleich:

| Datei | Stelle | Was dort steht |
|---|---|---|
| `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/_t_circle.md` | Kopfzeile `**Active spec/plan:**` | `…/planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md` |
| `fusion-workbench/agentstate.yaml` | `plan_context.plan_file` und `current_task.source_file` | derselbe Pfad mit `_o_` |

Richtig ist beides mal `260814-0656_c_plan-notizzettel-als-blatt-mit-zwei-zetteln.md`.

---

**Schwere:** niedrig, aber vor dem Rundenabschluss zu erledigen. Beide Dateien sind lebende
Zeiger und keine Aufzeichnungen eines Standes: der Circle-Datensatz sagt, welcher Plan gerade
gilt, und `agentstate.yaml` ist die Datei, aus der eine unterbrochene Sitzung wieder aufsetzt.
Wer nach einem Neustart daraus aufsetzt, greift auf einen Dateinamen, den es nicht gibt.

**Warum der Abgleich sie nicht selbst gezogen hat.** Keine der beiden steht in der
Schreibliste des `reconciler` (`agents/reconciler.md`, Abschnitt `## Scope`): erlaubt sind
Pläne, Defekte, Entscheide, Durchsichten, das eigene Abgleichprotokoll und der Abschnitt
`## Coherence` im Sitzungsprotokoll des Orchestrators. Der Circle-Datensatz und der
Sitzungszustand gehören dem Orchestrator.

**Nicht betroffen sind die Verweise in `history/`, `reviews/`, `decisions/` und
`shared/issues/`.** Sie behalten ihren damaligen Marker, wie die Ortsregel in `CLAUDE.md` es
für Aufzeichnungen eines Standes vorsieht. Am 260814-1002 nachgezählt: zwölf Verweise in Vollform über elf
Dateien, dazu **zwei in Kurzform**, die der Suche nach dem vollen Dateinamen entgehen.

**Der bekannte blinde Fleck hat auch hier zugeschlagen, und zwar zuerst bei diesem Abgleich
selbst.** Die zwei geschlossenen Defektdatensätze der Durchsicht schreiben
``planning/260814-0656_o_plan-…`` mit einer Ellipse statt des vollen Namens
(`260814-0908_c_…:letzter Absatz`, `260814-0909_c_…:letzter Absatz`). Ein `grep` auf
`260814-0656_o_plan-notizzettel` findet sie nicht; erst ein Muster auf `260814-0656_o_plan-`
tut es. `CLAUDE.md` führt genau das als Eigenschaft dieses Projekts, und der Anlass ist
`shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`.
Beide liegen in `issues/` und behalten ihren damaligen Marker; zu ziehen ist an ihnen nichts.

**Was zu tun ist.** Der Orchestrator zieht beide Stellen beim Abschluss von Turn 2 nach.

**Kontext**

- Gefunden beim Abgleich der Runde 9, `history/260814-1002-reconciliation.md`.
- Gemessen mit `grep -rln '260814-0656_o_plan-notizzettel' fusion-workbench/` am Stand `79dab20`.
