# Plan-Nachzug: beantwortete L4-Frage und fehlender xtask-Alias

**Agent:** planner
**Zeitpunkt:** 260802-1746
**Status:** Complete
**Bearbeitete Dateien:**
- `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Marker bleibt `_o_`)
- `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1755_o_...` → `_c_plan-legt-den-cargo-xtask-alias-in-keinem-schritt-an.md`

## Auftrag

Zwei Auslassungen im Plan nachziehen, beide ohne Entwurfsfrage. Erstens die seit dem 260802-1735 beantwortete Frage zu L4, die der Plan an fünf Stellen noch als offen führte, samt der neuen Messbedingung aus C8. Zweitens die Beschreibung von S1, die den Abschnitt `[alias]` der `.cargo/config.toml` übergeht, obwohl S5 mit `cargo xtask bundle` abnimmt.

## Gelesene Grundlagen

- Entscheidungsdatensatz `decisions/260802-1428_a_was-l4-mit-wiederhergestellten-tabs-meint.md`, einschließlich der `Answered:`-Zeile vom 260802-1735: Möglichkeit 1, ausdrücklich auch für den Tabwechsel aus L5.
- Spec `planning/260802-1036_o_spec-navigator-geruest.md`, Abschnitt C8: Messbedingungen (Prüfordner A und B, Sitzungslage), Zeilen L4 und L5 der Zusagentabelle, Absatz "Was L4 und L5 als abgeschlossen zählt".
- Defekt `issues/260802-1755_o_plan-legt-den-cargo-xtask-alias-in-keinem-schritt-an.md`.
- Ist-Stand von `.cargo/config.toml` (nur gelesen): der Alias steht in Zeile 9.

## Was geändert wurde

**Die fünf Stellen zur L4-Frage.** S8 hält jetzt fest, dass die Lesart entschieden ist und S8 sie unverändert misst, weil am Durchstich weder Tabs noch eine wiederhergestellte Sitzung existieren; die Abnahme gegen die Prüfsitzung leistet S22. S21 trägt die Messvorschrift für L4 und L5 samt der Prüfsitzung und der Staffelung L5, L2, L3/L10 für einen noch ungelesenen Zieltab. S22 stellt die Prüfsitzung über `session.toml` her und schreibt sie vor jedem Kaltlauf zurück. Die Zeile der Risikotabelle nennt die Entscheidung statt der offenen Frage. Der Eintrag unter "Angelegte Defekte und Entscheidungen" ist auf "beantwortet" gezogen, mit dem neuen Pfad `_a_`.

**Was die Messbedingung an S3 ändert.** S3 erzeugt drei Prüfordner statt zweier Größen: A und B mit je 10.000 Einträgen an verschiedenen Pfaden und einen mit 100.000 Einträgen. Der Unterbefehl `fixture` bleibt unverändert und erzeugt einen Ordner je Aufruf; unterschieden werden die drei über den Startwert (1, 2, 3). Ein Mehrfachmodus entsteht nicht. Das Abnahmekriterium prüft zusätzlich, dass Startwert 2 eine andere Liste liefert als Startwert 1. Mitgezogen: die Tabelle der Messstrecken in Frage 5 (L5 als eigene Zeile, L4 mit der entschiedenen Lesart), der Absatz über die Erzeugung der Prüfordner, der Berichtskopf (Pfad und Startwert je Ordner) und das Abnahmekommando von S21, das jetzt `--ordner-a` und `--ordner-b` statt `--ordner10k` nennt.

**Eine Auslassung, die beim Nachziehen auffiel: S12.** Die Zusage L5 hält nur, wenn ein Tab im Hintergrund bereits gelesen ist, wenn der Nutzer ihn ansteuert. Kein Schritt sagte bisher, wann das geschieht. S12 trägt jetzt die Lesereihenfolge beim Start, erst der sichtbare Tab jedes Fensters, danach die verdeckten, dazu ein Abnahmekriterium, das den ausbleibenden zweiten Lesevorgang prüft. Die Randbedingungstabelle nennt S12 zusätzlich bei den zehn Zeitzusagen.

**Der xtask-Alias.** S1 beschreibt für `.cargo/config.toml` jetzt beide Abschnitte, `[env]` und `[alias]`, mit der Begründung, dass S5, S21 und S23 unter dem Namen `cargo xtask` abnehmen. Das Abnahmekriterium verlangt den Alias im Diff und einen erfolgreichen Aufruf. Am Dateibestand ändert sich nichts, der `coder` hat den Alias bei S1 bereits gesetzt. Der Defekt ist mit `Resolved:`-Abschnitt geschlossen und auf `_c_` umbenannt.

## Grenzen eingehalten

`crates/`, `spikes/`, der Spec und der Circle-Datensatz sind unberührt. Die `[DONE]`-Markierung an S1 steht unverändert. Die Beschreibung von S2 ist nicht angefasst; der L4-Nachtrag berührt S2 auch inhaltlich nicht, weil der gestückelte Lesevorgang die entschiedene Lesart ohne Zusatzaufwand trägt. Der Plan bleibt auf Marker `_o_`. Nichts committet.
