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

---
Abgleich 260819-1440 (reconciler, Baumstand `77dcd48`): **der eine ungedeckte Wurf ist nachgedeckt, die Verfahrensfrage steht, und die Gestalt ist heute wieder aufgetreten.**

**`7fae5ba` ist gedeckt**, und zwar 32 Minuten nach der Ablage dieses Datensatzes: `shared/reviews/260815-1844-coderev-verweisziel-fragt-am-namen-nachgemessen.md` trägt `Reviewed-range: e37a1e3..60a8ca5` und nennt den Commit in seiner Zusammenfassung ausdrücklich. Der Einzelfall der Überschrift ist damit erledigt.

**Die Frage der Abgrenzung ist es nicht.** Ob die Zuordnung „fasst nur Werkbank-Markdown an" von Hand geschieht oder aus `git show --name-only` abgeleitet wird, ist am Baum nicht entscheidbar und auch nicht entschieden. Sie ist ihrer Art nach eine Entscheidung und kein Defekt; der Vorschlag steht im Abgleichsprotokoll `shared/history/260819-1440-reconciliation.md`.

**Also seen: 260819-1440 by reconciler** — `76ceb68` fasst `crates/krk-ui/src/appkit/anwendung.rs` und `crates/krk-ui/src/appkit/tabelle.rs` an, also ausgelieferten Code, und keine Durchsicht deckt ihn: die jüngste Durchsichtsdatei des ganzen Baums ist `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/reviews/260818-2340-coderev-round-13-turn-2-abwurf-aus-fremden-apps.md`, und sie liegt vor dem Commit. Der Wurf ist außerhalb jedes Turns gefahren, nach dem `session_end` der Sitzung 260818-1117, weshalb kein Durchsichtsschritt für ihn vorgesehen war.

---
Umgelegt am 260819 vom Defekt- in den Entscheidungsspeicher, auf Befund des Abgleichs 260819-1440: der Text sagt selbst, dass zu entscheiden und nicht zu beheben ist. Der Datensatz behaelt vorerst die Gestalt eines Defektberichts; die Abschnitte Options, Constraints und Recommendation fehlen ihm und sind nachzutragen, wenn die Frage vorgelegt wird.
