# Analyst: die drei Workbench-Aufzeichnungen tragen ihren Nachtrag

**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Schritt:** 15 des Plans `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`
**Kriterien:** C9.3, C10.2
**Defekt:** `260830-1106_*_der-entscheid-zur-c-freiheits-zusage-nennt-fuenf-prosastellen-im-baum-stehen-sechs.md`, mit diesem Schritt geschlossen
**Stand:** `9566973`, Arbeitsbaum unterhalb von `crates/` unberührt

## Die Form ist die Sache

Der Schritt berichtigt drei Aufzeichnungen und fasst keinen Quelltext an. Nach der Ortsregel
aus `CLAUDE.md` behalten Aufzeichnungen eines Standes ihren damaligen Wortlaut; eine Zahl
darin wird angehängt berichtigt und nicht überschrieben. Kein bestehender Satz der drei
Dateien ist geändert, jeder Nachtrag steht am Ende seiner Datei.

## Die Erhebungsvorschrift ist selbst gefahren

```sh
grep -rn --exclude-dir=fusion-workbench --include='*.md' --include='*.toml' --include='*.rs' 'Namen auf `-sys`' .
```

Sieben Treffer am 260831-1321: `Cargo.toml:93` (`regex`), `:153` (`zip`), `:279`
(`objc2-pdf-kit`), `:361` (`syntect` und `two-face`), `:515` (`gix`), `CLAUDE.md:89` und
`crates/krk-core/src/verzeichnis/sys.rs:75`. Das ist dieselbe Menge wie in Schritt 13, mit
einer Abweichung in der Zeilenlage: `CLAUDE.md` liegt zwei Zeilen tiefer als dort notiert,
weil Schritt 14 die Rundentabelle um die Zeile 23 erweitert hat. Die Stelle selbst ist
dieselbe. Die Zeilennummern sind der Grund, die Vorschrift zu fahren statt sie abzuschreiben.

`grep -rn 'write_changes(' crates/` bleibt ohne Fundstelle, Exit 1. Das ist die Prüfung, die
die Schreibfreiheit der Stufe A trägt, und der Nachtrag zum dritten Datensatz zitiert sie.

## Die drei Nachträge

- `260830-1006_*_wie-lautet-die-c-freiheits-zusage-wenn-linux-raw-sys-in-cargo-lock-steht.md`,
  hinter der `Answered:`-Zeile: die Zahl „fünf Prosastellen" steht dreimal im Datensatz und
  bleibt dreimal stehen. Der Nachtrag nennt die Erhebungsvorschrift und ihre sieben Treffer
  und schreibt den eigentlichen Befund aus: die Erhebung des Entscheids suchte nach dem
  Wortlaut der alten Zusage, und die sechste Stelle führte die Zusage ohne diesen Wortlaut.
  Die neue Vorschrift ist nur vollständig, solange jede Stelle die Wendung „Namen auf
  `-sys`" führt.
- `260830-0950-orchestrator-session.md`, als eigener Abschnitt am Ende: derselbe Nachtrag,
  bezogen auf den Satz „Fünf Prosastellen sind nachzuziehen" im Abschnitt über die zwei vor
  dem Plan beantworteten Entscheidungen.
- `260830-1006_*_darf-stufe-a-den-aufgefrischten-index-zurueckschreiben-oder-zahlt-sie-die-wiederholung.md`,
  hinter der `Implemented:`-Zeile: der in Schritt 10 gemessene Posten als Tabelle über die
  drei Bäume, mit dem Verhältnis 1,7 bis 9,5 gegenüber der Statusabfrage und den 1 369 gegen
  147 ms bei hunderttausend Einträgen. Dazu der Nebenbefund des Messberichts: `gix` 0.87.1
  fängt den `NeedsUpdate`-Posten im eigenen Statusiterator ab, der Zweig im Gitleser ist
  unerreichbar, und die Schreibfreiheit trägt allein der fehlende Aufruf von
  `write_changes`.

## Die Marker

- Der Defekt `260830-1106_*` geht von `_o_` auf `_c_`. Seine `Resolved:`-Zeile trägt jetzt
  zwei Hälften: die des Quellbaums aus Schritt 13 und die der Workbench aus diesem Schritt.
- Der dritte Datensatz bleibt `_i_`. Der Plan beschreibt ihn als offen in der Wiedervorlage;
  der Nutzer hat am 260831 Möglichkeit 1 gewählt, und die Antwort steht in seiner
  `Implemented:`-Zeile. Der Nachtrag trägt allein die Zahl nach.
- Der erste Datensatz bleibt `_a_`, unverändert. **Das ist ein Befund und keine
  Erledigung:** die Antwort ist seit Schritt 13 (`ad7c2f2`) an allen sieben Stellen im Baum
  realisiert, der Marker sagt aber weiter „beantwortet, nicht umgesetzt". Der Übergang nach
  `_i_` ist ein Endzustand und nicht rückholbar, dieser Schritt hat ihn nicht zugeteilt
  bekommen, und ein Endzustand ohne Auftrag gesetzt ist teurer als einer, der eine Runde
  später gesetzt wird. Der Übergang liegt beim Orchestrator.

## Abnahme

`make check` ist hier keine Prüfung: der Schritt ändert keine Zeile unter `crates/`,
`Cargo.toml`, `resources/` oder `xtask/`. Geprüft wurde stattdessen:

- die Erhebungsvorschrift gefahren, sieben Treffer, Exit 0;
- `grep -rn 'write_changes(' crates/` ohne Fundstelle, Exit 1;
- `git status --porcelain` nach dem Schritt: geändert sind allein die drei Aufzeichnungen,
  die Planzeile 387 und diese Datei, dazu die Umbenennung des Defekts;
- `git diff` gegen `9566973` über die drei Aufzeichnungen: ausschließlich Zeilen hinzugefügt,
  keine geändert, keine gelöscht.

Kein Kommando über den ganzen Baum ist abgesetzt worden, kein Commit.
