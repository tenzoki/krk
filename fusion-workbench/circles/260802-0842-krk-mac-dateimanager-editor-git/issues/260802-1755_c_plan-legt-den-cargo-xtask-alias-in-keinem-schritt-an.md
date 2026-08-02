Der Plan legt den `cargo xtask`-Alias in keinem Schritt an, nimmt ihn aber ab

---

Schritt S5 des Plans `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` nimmt mit dem Kommando `cargo xtask bundle` ab. `cargo xtask` ist kein eingebautes Cargo-Kommando, sondern ein Alias, den ein Projekt selbst im Abschnitt `[alias]` seiner `.cargo/config.toml` setzt.

Kein Schritt des Plans legt diesen Alias an:

- S1 nennt `.cargo/config.toml` unter seinen Dateien, beschreibt für sie aber ausschließlich den Abschnitt `[env]` mit `MACOSX_DEPLOYMENT_TARGET`.
- S5 nennt als Dateien nur `xtask/src/*` und `README.md`.

Ohne den Alias schlägt das Abnahmekriterium von S5 fehl, und `xtask` wäre ein Workspace-Mitglied, das über seinen vorgesehenen Namen nicht erreichbar ist.

---

**Bereits behoben, ohne Entscheidungsbedarf.** Der `coder` hat den Alias bei der Umsetzung von S1 ergänzt, weil er zum Bauzuschnitt gehört und die Auslassung sonst erst in S5 aufgefallen wäre. Stand in `.cargo/config.toml`:

```toml
[alias]
xtask = "run --package xtask --"
```

Verifiziert am 260802-1755: `grep -n 'xtask' .cargo/config.toml` findet den Alias in Zeile 9.

**Warum der Defekt trotzdem abgelegt ist.** Der Plan beschreibt weiterhin für `.cargo/config.toml` nur den Abschnitt `[env]`. Ein späterer Leser, der den Plan gegen den Dateibestand hält, findet einen Unterschied ohne Erklärung. Der Plan gehört dem `planner`; der `coder` durfte ihn nicht ändern, und der Orchestrator ändert keine Plandokumente.

**Was zu tun ist:** Der `planner` ergänzt in S1 die Beschreibung von `.cargo/config.toml` um den Abschnitt `[alias]`. Am Dateibestand ändert sich dadurch nichts.

**Aufgefallen bei:** der Umsetzung von S1 durch den `coder`, Protokoll `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1741-cargo-workspace-und-bauzuschnitt.md`. Der `coder` konnte den Defekt nicht selbst ablegen, weil ihm `fusion-workbench/` für diesen Schritt gesperrt war; er hat ihn stattdessen gemeldet.

---
Resolved: Der `planner` hat S1 im Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` um den Abschnitt `[alias]` ergänzt. Die Beschreibung von `.cargo/config.toml` nennt jetzt beide Abschnitte, `[env]` mit `MACOSX_DEPLOYMENT_TARGET = "15.0"` und `[alias]` mit `xtask = "run --package xtask --"`, samt der Begründung, dass S5, S21 und S23 unter diesem Namen abnehmen. Das Abnahmekriterium von S1 verlangt den Alias jetzt ebenfalls im Diff und einen Aufruf `cargo xtask`, der das Bauwerkzeug erreicht. Am Dateibestand ändert sich nichts: `.cargo/config.toml` trägt den Alias seit der Umsetzung von S1 in Zeile 9, geprüft am 260802-1746 mit `cat .cargo/config.toml`. Plan und Dateibestand stimmen damit überein.
