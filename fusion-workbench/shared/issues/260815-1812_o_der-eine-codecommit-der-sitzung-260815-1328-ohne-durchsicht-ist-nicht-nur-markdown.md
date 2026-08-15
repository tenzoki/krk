Der eine ungedeckte Commit der Sitzung 260815-1328 fasst Code an und nicht nur Markdown der Werkbank

---

`bin/fusion-review-coverage` meldet für die Sitzung 260815-1328 sieben von zwölf Commits als
von keiner Durchsicht gedeckt. Die Begründung, mit der sie ungedeckt geblieben sind, lautet:
sie fassen allein Markdown der Werkbank an. Für sechs der sieben stimmt das. Für `7fae5ba`
nicht — der Commit ändert `crates/krk-core/src/verzeichnis/verweisziel.rs` und
`crates/krk-core/tests/verzeichnis.rs`, also ausgelieferten Code und die Proben dazu.

---

**Gefunden am:** 260815-1812, Stand `7fae5ba`
**Gefunden von:** reconciler, beim Abgleich der Sitzung 260815-1328
**Schwere:** mittel. Kein Fehlverhalten am Code, keine gebrochene Probe. Der Schaden ist die
Lücke im Verfahren: die Sitzung hat ihre eigenen Fehler zweimal über Durchsichten gefunden
und den letzten Codewurf ohne diesen Schritt abgelegt.
**Betroffen:** Verfahren, nicht Code. Der ungedeckte Wurf ist `7fae5ba`.
**Domain:** code

## Der Befund

```
$ "$FUSION_PLUGIN_ROOT/bin/fusion-review-coverage"
anchor=workbench-root  since=838432c  head=HEAD
commits=12  reviews=2  unusable=0  uncovered=7  verdict=uncovered
```

Die sieben ungedeckten, mit ihrem Dateibestand aus `git show --name-only`:

| Commit | Fasst an | Nur Werkbank-Markdown |
|---|---|---|
| `7fae5ba` | `crates/krk-core/src/verzeichnis/verweisziel.rs`, `crates/krk-core/tests/verzeichnis.rs`, dazu vier Werkbankdateien | **nein** |
| `e37a1e3` | fünf Dateien unter `fusion-workbench/shared/` | ja |
| `a2670db` | sechs Dateien unter `fusion-workbench/shared/` | ja |
| `cd0b5b7` | fünfzehn Dateien unter `fusion-workbench/` | ja |
| `f280c42` | zwei Dateien unter `fusion-workbench/shared/issues/` | ja |
| `39060d4` | eine Datei unter `fusion-workbench/shared/consult/` | ja |
| `223a333` | zwei Dateien unter `fusion-workbench/shared/issues/` | ja |

Nachzuzählen mit:

```sh
for c in 7fae5ba e37a1e3 a2670db cd0b5b7 f280c42 39060d4 223a333; do
  echo "== $c"; git show --name-only --format= $c
done
```

## Warum gerade dieser Wurf die Durchsicht gebraucht hätte

`7fae5ba` ist die Behebung des einzigen Befunds der Schwere **hoch** dieser Sitzung
(`shared/issues/260815-1713_*_verweisziel-beantwortet-die-ordnerfrage-mit-open-und-nicht-mit-stat.md`).
Er wechselt den Mechanismus einer Auflösung von `open(2)` auf `stat(2)`, also die Systemfrage
selbst, und ändert damit das Verhalten in drei Dateisystemzuständen, die vorher falsch
eingeordnet waren. Die beiden Durchsichten dieser Sitzung haben je zwei Befunde in genau der
Sorte Code gefunden, die dieser Wurf noch einmal anfasst:
`shared/reviews/260815-1450-coderev-…` fünf Befunde,
`shared/reviews/260815-1720-coderev-…` vier. Beide Male stammten mehrere Befunde aus dem Code
des jeweils vorigen Wurfs derselben Sitzung.

Der Abgleich hat zwei Abweichungen an `7fae5ba` gefunden, die einer Durchsicht aufgefallen
wären: ein Verweis im Modulkopf zeigt auf einen Datensatz, den es unter diesem Namen nie gab
(`shared/issues/260815-1812_*_ein-verweis-im-modulkopf-des-verweisziels-zeigt-auf-einen-datensatz-der-nie-so-hiess.md`),
und der Datensatz `260815-1752` zählt seine eigenen Fundstellen falsch.

## Abgrenzung

Der Befund betrifft nicht die Regel, mit der Werkbank-Markdown ungedeckt bleiben darf — die
trägt, und sechs der sieben Commits fallen richtig darunter. Er betrifft die eine Zuordnung,
die daneben lag, und die Frage, ob die Zuordnung von Hand geschieht oder aus dem
Dateibestand des Commits abgeleitet wird. `git show --name-only` beantwortet sie in einer
Zeile.

## Ablage

Gemeinsamer Speicher. Betrifft das Verfahren dieses Projekts und die Directive keiner Runde.
