# Abgleich — 260829-1223 — Runde 21, Abschluss

**Circle:** `260828-1041-dateilistenfilter-nimmt-eingaben-per-paste` (`_t_`)
**Domain:** code — **Anker:** `79d507a` → HEAD `8d64859`, Turn 1 (`fusion-events turns`, scope=checkout)
**Verifikation am Arbeitsbaum:** `make check` exit 0 — 23 Probensätze, 1733 Proben grün, `clippy -D warnings` und `fmt --check` ohne Ausgabe.

## Bestand

- Pläne: 1 gelesen, 1 geändert — `planning/260829-1102_*_plan-…` `_p_` → `_c_`, Status Complete, Reconciliation Log mit Belegtabelle je Schritt.
- Specs: 1 gelesen, 1 geändert — `planning/260829-1052_*_spec-…` `_o_` → `_c_`, Statuszeile nach dem Muster der Runde 22.
- Defekte des Circles: 4 gelesen, 4 bleiben offen mit Vermerk; 1 neu gefilt (unten). `shared/issues/260816-2144_o_…` (Leertaste) bleibt offen, wie der Plan es ausschreibt.
- Entscheidungen: `decisions/260828-1041_o_…` gelesen, bleibt `_o_` mit Vermerk (keine Vorbedingung; Spec A6). Unter `shared/decisions/` 24 aktive (`_o_`/`_a_`), keine berührt.
- Durchsicht: `reviews/260829-1218-coderev-…` annotiert, Befunde unverändert.
- Backlog: `shared/backlog/260829-0842_c_…` trägt `Promoted:` auf diesen Circle und den Spec; `_c_` ist richtig, nichts zu tun.

## Befunde

1. **Alle zwölf `[DONE]` halten.** Je Schritt Datei:Zeile im Reconciliation Log des Plans; Commits `f4ba58d`, `1b0939a`, `3722c89`, `415ef6f`, `097abc2`, `8d64859`.
2. **`## Where this Circle stops`:** alle Klauseln halten bis auf den Wortlaut einer: `grep -rn 'regex' Cargo.lock` ist nicht leer (12 Treffer über `syntect`, auf `c6c86cb` wie auf HEAD). Die Absicht hält (Diff von `Cargo.lock`/`Cargo.toml` leer). Gefilt: `issues/260829-1223_o_die-abschlussklausel-des-plans-verlangt-ein-leeres-grep-nach-regex-in-cargo-lock-und-die-datei-traegt-es-seit-syntect.md`. Nebenbefund ohne Datensatz: `grep -n objc2 crates/krk-core/src/zwischenablage.rs` trifft die Kommentarzeile `:12`, die das Fehlen der Kiste ausspricht; die Klausel meint Code, und Code trifft sie nicht.
3. **Offene Defekte, je am Baum nachgelesen:** `260829-1201` — Spec-Prosa C6.6, Probe `tests/verzeichnis.rs:2427` hält den tatsächlichen Stand; `260829-1215` — `filtertext_aus` (`zwischenablage.rs:163`) und `text_anhaengen` (`modell.rs:972`) tragen keine Grenze und keinen Doc-Satz dazu; `260829-1216` — `zwischenablage.rs` prüft nach dem Abschneiden allein `contains('\n')`; `260829-1217` — `grep -c die_zeichenregel_hat_zwei_rufer CLAUDE.md` liefert 1, und der Absatz nennt den Vergleich weiter als Teilzeichenfolge.
4. **Circle-Datensatz (nur gemeldet, nicht geändert):** `**Active spec/plan:**` zeigt nach der Umbenennung auf `…_p_plan-…`; richtig ist `…_c_plan-…`. Der Orchestrator setzt es. `## Turn log` steht, uncommittet, und stimmt mit den Commits überein (S1 `f4ba58d`, S2 `1b0939a`, S3–S8+S10 `3722c89`, S9 `415ef6f`, S11 `097abc2`; die vier Defekte sind die unter `issues/`).
5. **CLAUDE.md (nur gemeldet, Kurator):** die zwei Aussagen aus `260829-1217` stehen noch; dazu nennt der Absatz zur Zwischenablage-Hülle „seit der Runde 14 zusätzlich eine hereingereichte Ablage" und schweigt über den dritten Leser `einfuegequelle` — keine falsche Aussage, nur eine Lücke.
6. **History-Einträge gegen die Commits:** die zehn Coder-/Ontocoder-Einträge sind in `415ef6f` und `097abc2` committet; die Sitzungshistory `260829-1047-orchestrator-session.md` trägt `**Status:** In Arbeit` und endet am Snapshot — der Orchestrator schreibt den Abschluss.

## Misfiled — should be a decision

Keiner. `260829-1215` nennt den Nutzer als ersten Executor (Festlegung Grenze oder ausdrücklich keine), ist aber als Defekt mit zwei benannten Abnahmewegen tragfähig und bleibt, wo er ist.
