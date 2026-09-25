# 260818-0340 — ontocoder: die Pfade und die Blattsperre in der Belegungsdatei

**Status:** Complete
**Baumstand bei Beginn:** `48bb57f`
**Datei:** `resources/default-keymap.toml` (die eine)
**Datensätze:** `issues/260817-2243_*_two-decision-paths-in-the-keymap-head-…` (geschlossen),
`issues/260817-1419_*_ein-vierter-traeger-der-verkuerzten-blattsperre-…` (bleibt offen)

## Auftrag

Wiederaufnahme eines abgebrochenen Laufs. Der erste der drei Befunde an dieser Datei ist als
`48bb57f` festgeschrieben; die beiden übrigen sind hier gefahren.

## Was getan wurde

**1. Jeder Pfad steht ganz auf einer Zeile (`260817-2243`).** Der Datensatz nennt zwei
umgebrochene Pfade, `:12-13` und `:66-67`. Die Prüfung über die ganze Datei fand **22**, zwei
davon über drei Kommentarzeilen. Jeder wurde zusammengezogen und sein Absatz auf die in der
Datei vorherrschende Breite von 78 Zeichen neu umbrochen, wobei der Pfad selbst als
unteilbares Wort auf einer Zeile bleibt.

**2. Zehn Pfade zeigten auf nichts (zusätzlicher Befund).** Zitate der Form `decisions/…` und
`issues/…` ohne Circle lösen nach der Pfadregel in `CLAUDE.md` gegen die **Runde 2** auf,
`circles/260807-2116-eingebauter-editor-mit-textmarken`. Keiner der zehn liegt dort: neun
gehören der Runde 1 (`circles/260802-0842-krk-mac-dateimanager-editor-git`), einer der Runde 4
(`circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`). Alle zehn tragen jetzt ihren
Circle, so wie die Mehrheit der Pfade dieser Datei es schon tat.

**3. Die Blattsperre nennt vier Kommandos (`260817-1419`).** Die Stelle sagte „jeden Befehl
ausser dem Abbruch". Sie nennt jetzt den Abbruch über
`kommandos::operationen::waehrend_blatt_erlaubt` und die drei der Ausnahmeliste
`kommandos::zulaessigkeit::immer_erreichbar`. Beleg ist die Probe
`zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch` (seit `441da86`), die die
tatsächlich durchgelassenen Kommandos zählt; der Absatz nennt sie jetzt selbst. Die Tasten der
drei sind in derselben Datei nachgezählt und nicht aus einem Kommentar übernommen: `cmd+q`
(`beenden`), `shift+cmd+w` (`fenster_schliessen`), `cmd+n` (`fenster_einblenden`).

## Prüfung

```
$ grep -nE '`[^`]*/$' resources/default-keymap.toml
(kein Treffer)

$ python3 <jeden Pfad in Rückwärtsanführungszeichen über seinen Marker-Glob auflösen>
paths checked: 33  unresolved: 0

$ make check
alle vier gruen   (Exit 0)
```

## Offen geblieben

`CLAUDE.md:124` trägt dieselbe verkürzte Blattsperre. Die Datei gehört nicht zum Auftrag dieses
Executors, und `260817-1419` bleibt deshalb offen.
