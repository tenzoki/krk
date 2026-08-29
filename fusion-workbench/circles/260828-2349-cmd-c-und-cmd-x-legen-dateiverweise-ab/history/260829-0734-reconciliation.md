# Abgleich — 260829-0734

**Circle:** 260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab (aktiv, `_t_`)
**Anker:** `4bd0084` → HEAD `35b95b3`; Turns laut `fusion-events turns`: 1 (scope=checkout)
**Domain:** code

## Zahlen

- Pläne gelesen: 8 (Plan und Spec dieses Circles; sechs `_o_`/`_p_`-Einträge unter `shared/planning/`, keiner seit dem Anker angefasst und keiner Gegenstand der Runde — Statuszeilen gelesen, nicht neu verifiziert). Aktualisiert: 2 — Plan `_p_` → `_c_` mit Reconciliation Log und Belegtabelle, Spec `_o_` → `_c_` mit Statuszeile und Log (Vorgehen der Runde 20; Lesart des Markers offen, `shared/decisions/260819-1440_o_…`).
- Entscheidungen gelesen: 25 (1 im Circle, 24 `_a_`/`_o_` unter `shared/decisions/`). Aktualisiert: 1 — `260829-0053_a_` bleibt `_a_` mit Vermerk (die Antwort ändert keinen Code, `_i_` verspräche einen Commit, den es nicht gibt).
- Defekte gelesen: 6 (vier `_o_` im Circle; `shared/issues/260826-1302_c_` und `260826-1423_c_` als seit dem Anker angefasst, beide schon am 260828-1044 bestätigt). Aktualisiert: 4 mit Abgleichsvermerk; kein Marker geändert.
- Reviews: 1, annotiert.
- Neu gefilet: 0.

## Befunde

- Alle neun `[DONE]` halten; Belegtabelle im Reconciliation Log des Plans. `cargo test --workspace` grün auf `35b95b3` (krk-ui 851 passed), `clippy -D warnings` exit 0, `fmt --check` exit 0.
- Jede Klausel unter `## Where this Circle stops` mit Beleg (im Log des Plans); `NSPasteboard` außerhalb der Hülle allein in `abwurf.rs::sorten` und `vorschau.rs::auswahl_ablegen`, nach dem präzisierten Constraint erlaubt.
- Abweichung ohne Handlungsbedarf: Plan nennt `public.url`, die Messung der Durchsicht allein `public.file-url`.
- `issues/260829-0006_o_…`, Punkt 2, ist durch `3764fb6` im Baum überholt (`fileURLWithPath:` steht seither im Untergrenzen-Abschnitt); der Defekt bleibt wegen der Punkte 1 und 3 offen, Vermerk angehängt.
- Circle-Datensatz: `## Turn log` steht (uncommittet); **`**Active spec/plan:**` zeigt auf `…_p_plan-…`, nach der Umbenennung veraltet** — Orchestrator. `**Filed by:** shaper (anticipated-circle mode)` und `**Claim:**` stehen.
- Sitzungs-History: trägt nach dem Snapshot keinen Turn-Abschnitt und `**Status:** In Arbeit`; Sache des Orchestrators beim Abschluss. `## Coherence` angehängt.
- History-Einträge (acht Coder-Einträge, `023ee64`/`38aa652`) decken sich mit den Commits `4455af7`, `dfde98c`, `3764fb6`, `1644ada`; der Eintrag zu Schritt 7 nennt drei rote Proben im vollen Lauf, die Schritt 8 grün sah — als `issues/260829-0041_o_…` gefilet.
- Die Auslieferung 1.2.2 (`701412c`, `9facb1e`) liegt zwischen Anker und erstem Codecommit der Runde, ändert allein `Cargo.toml`/`Cargo.lock`/Ereignislog und ist kein Gegenstand.
- CLAUDE.md, nur gemeldet, nicht geändert (Kurator): Zeile 79 „Es gibt genau eine Hülle um `NSPasteboard`" beschreibt sie als Ziel allein für Text — seit `3764fb6` schreibt sie daneben Dateiverweise (`dateiverweise_schreiben`, `writeObjects:`), der Absatz kennt den zweiten Ausgang nicht; die Rundentabelle endet bei 18 und führt weder 19 (`260827-0310`), 20 (`260827-2028`), 21 (`260828-1041`, vorgesehen) noch 22 (dieser Circle); der Absatz „Seit der Runde 17 führt ein zweiter Weg in die Anwendung hinein, und er hat keine Taste" kennt den dritten nicht: `copy:`/`cut:` erreichen den Anwendungsdelegierten über die Antwortkette, ohne `Kommando` und ohne Belegungszeile, gehalten von `Anspruch` in `zulaessigkeit.rs:247` und der Zählprobe in `betrachter.rs:731`.
- `shared/backlog/260828-2345_c_…` (Ursprung dieser Runde) steht auf `_c_`, vom Playmaker; stimmig.

## Nicht angefasst

Code, `Cargo.*`, `resources/`, CLAUDE.md, der Circle-Datensatz (Marker `_t_`, Kopffelder und Turn log gehören dem Orchestrator). Die zwei Umbenennungen unter `planning/` sind per `git mv` im Index vorgemerkt, nicht committet.
