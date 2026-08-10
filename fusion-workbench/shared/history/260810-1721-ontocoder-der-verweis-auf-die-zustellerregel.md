# Der Verweis auf die Zustellerregel nennt jetzt den Circle der Runde 1 (T4)

**Agent:** ontocoder
**Status:** Complete
**Quellen:**
- `shared/issues/260810-0805_*_ein-verweis-nennt-den-falschen-circle-und-die-zustellerregel-liegt-woanders.md`
- `tasklist.md`, Abschnitt `### 4. Ein Verweis nennt den falschen Circle`

**Zum Stilprofil:** `fusion-rules ontocoder` gab allein `fusion-workbench/stilwerk/chat-voice-de.yaml` aus, kein `default-voice-de.yaml`. Für diesen Bericht gilt deshalb kein Langform-Schreibprofil; das Fehlen ist hier vermerkt, wie `rules/agent-setup.md` es verlangt.

---

## Die Änderung

Eine Zeile in einer Datei, `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_o_wie-wird-die-ausgabe-der-belegung-ausgeloest.md`, Zeile 7, Feld `**Cross-references:**`. Der vierte Verweis der Zeile führte

```
circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260805-0713_i_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md
```

und führt jetzt

```
circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0713_i_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md
```

Ersetzt ist allein das Verzeichnis. Marker (`_i_`), Dateiname und die anschließende Klammer „die Zustellerregel, zitiert wo sie liegt" stehen unverändert. Die drei übrigen Verweise derselben Zeile sind nicht angefasst.

## Gegenprüfung vor der Änderung

`find fusion-workbench -name '260805-0713*'` liefert genau einen Treffer:

```
fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0713_i_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md
```

Der Datensatz liegt also im Circle der Runde 1, wie der Defektdatensatz es sagt. Eine zweite Fassung im Circle der Runde 2 gibt es nicht.

## Prüfung nach der Änderung

Ein Skript hat aus allen fünf Datensätzen dieses Circles jeden in Backticks gesetzten `.md`-Pfad der `**Cross-references:**`-Zeile ausgelesen und gegen den Dateibestand gehalten: 14 Verweise, 14 vorhanden, keiner fehlt, Rückgabewert 0. Damit ist der Umfangsbefund des `taskplanner` vom 260810-1707 unabhängig bestätigt: die übrigen vier Datensätze tragen keinen toten Verweis, und der zweite fremde Verweis derselben Zeile 7, auf `260805-0000_i_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`, nennt den Circle richtig.

`grep -rn '260807-2116-eingebauter-editor-mit-textmarken/decisions/260805-0713' fusion-workbench --include='*.md'` findet den falschen Pfad noch an zwei Stellen: in `tasklist.md` und im Defektdatensatz selbst. Beide zitieren ihn absichtlich als den zu behebenden Fehler und sind deshalb nicht angefasst.

`git diff --stat` der Datei: eine Zeile geändert, `1 insertion(+), 1 deletion(-)`.

## Nicht angefasst

Keine zweite Datei. Insbesondere nicht die Circle-Datensätze (`_?_circle.md`), nicht `portfolio.md` und nicht `CLAUDE.md` — daran arbeitet T2 parallel. Der Defektdatensatz bleibt auf `_p_`; den Abschluss mit `Resolved:`-Vermerk setzt der Nutzer. Nicht committet.
