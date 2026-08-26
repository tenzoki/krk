Der Sammler der Zählproben in `release.rs` verschluckt Lesefehler und liest die Werkbank mit
---
`sammeln` kehrt bei einem unlesbaren Verzeichnis still zurück; die zwei Zählproben darüber liefen dann mit weniger Dateien grün. Ausgeschlossen sind nur `target` und `.git`.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/release.rs` (Prüfmodul)

## Befund

`sammeln` (`release.rs:1304-1322`): `let Ok(eintraege) = fs::read_dir(ordner) else { return; };` und `for eintrag in eintraege.flatten()`. Beide Formen lassen einen Lesefehler stumm. Die Abnehmer sind Zählproben: `xtask_ruft_git_an_genau_einer_stelle` (`:1086-1099`, C3.13) und `allein_release_fragt_nach_tag_und_arbeitsbaum` (`:1108-1131`, C3.12). Eine Zählprobe, die weniger liest, als da ist, hält weniger, als sie behauptet — dieselbe Gestalt wie `260826-1302_o_eine-vierte-pruefordner-fassung-…`, wo eine Zählprobe eine Kiste nicht sieht.

Die Ausschlussliste `:1312` kennt `target` und `.git`. `fusion-workbench/` und `spikes/` werden gelesen. Die Nachbarprobe `der_quellbaum_nennt_die_alte_stationszahl_nicht_mehr` (`:1272-1279`) begründet, warum die Werkbank draußen bleibt, und beschränkt sich deshalb selbst auf `xtask/`; `rust_dateien` teilt die Begründung nicht.

## Abhilfe

Lesefehler in `sammeln` mit `expect` zum Ausfall machen; die Ausschlussliste um `fusion-workbench` und `spikes` erweitern und die Begründung von `:1272-1279` an `sammeln` ziehen.

**Schwere:** Low.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, L8
