Ein Verweis im Modulkopf des Verweisziels zeigt auf einen Datensatz, den es unter diesem Namen nie gab

---

`crates/krk-core/src/verzeichnis/verweisziel.rs:95` verweist für die offene Nutzerfrage auf

```
shared/issues/260815-1749_*_der-pfadsprung-meldet-den-ordner-ohne-leserecht-und-der-doppelklick-schweigt.md
```

Diese Datei gibt es nicht, und sie hat nie existiert. Der Datensatz ist im selben Commit
`7fae5ba` als **Entscheid** angelegt worden, unter

```
shared/decisions/260815-1749_o_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md
```

Speicher und Namensteil weichen beide ab. Der Verweis steht im ausgelieferten Code.

---

**Gefunden am:** 260815-1812, Stand `7fae5ba`
**Gefunden von:** reconciler, beim Abgleich der Sitzung 260815-1328
**Schwere:** niedrig. Kein Verhalten, kein Bau, keine Probe hängt daran. Wer der Angabe
folgt, findet nichts und hält die Frage für unbelegt.
**Betroffen:** `crates/krk-core/src/verzeichnis/verweisziel.rs:95`
**Domain:** code

## Wie es entstanden ist

Der Auftrag hat den Datensatz zunächst als Defekt unter `shared/issues/` angelegt und ihn
während der Sitzung nach `shared/decisions/` umgelegt, weil seine Auflösung „entscheiden und
festhalten" ist und nicht „gehen und beheben". Die Umlegung hat den Titel zu einer Frage
umformuliert und damit den Namensteil mitgeändert. Der Verweis im Code trägt beides noch in
der alten Form.

Nachzuprüfen mit:

```sh
grep -rn '260815-1749' crates/ xtask/ CLAUDE.md
find fusion-workbench -name '260815-1749*'
```

## Die Sternform schützt hier nicht, und der Grund ist aufgeschrieben

Der Verweis steht bereits in der Sternform `_*_`, wie der Entscheid
`shared/decisions/260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md`
sie vorschreibt. Sie hält gegen einen Wechsel des Zustandsmarkers und gegen nichts sonst; der
Abschluss von
`shared/issues/260815-1216_c_sieben-verweise-dieser-sitzung-nennen-einen-marker-den-ihr-ziel-nicht-mehr-traegt.md`
sagt das wörtlich: „Wird der Namensteil eines Datensatzes umgeschrieben, zeigt das Zitat
weiter ins Leere, und ohne Prüfung im Bau bemerkt das niemand, bis jemand danach sucht."

Dieser Datensatz ist der erste gemessene Eintritt dieses Restrisikos, und er ist am Tag der
Antwort eingetreten. Ein Wechsel des **Speichers** kommt dazu und ist von der Sternform
ebenso wenig gedeckt: ein Zitat auf `shared/issues/…` bleibt falsch, auch wenn Zeitstempel
und Thema stimmen.

## Zwei weitere Fundstellen derselben Angabe

- `fusion-workbench/shared/issues/260815-1713_c_verweisziel-beantwortet-die-ordnerfrage-mit-open-und-nicht-mit-stat.md:158` —
  Aufzeichnung eines Standes, fällt unter die Ortsregel von `CLAUDE.md`; im Abgleich vom
  260815-1812 dort vermerkt und nicht berichtigt.
- `fusion-workbench/shared/history/260815-1758-coder-verweisziel-fragt-am-pfad-statt-am-deskriptor.md:90` —
  dasselbe, `history/` ist ebenfalls eingefroren.

Zu berichtigen ist allein die Stelle im lebenden Code.

## Was zu tun ist

Den Verweis in `verweisziel.rs:95` auf
`shared/decisions/260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`
setzen. Wer dabei die Wiederkehr abstellen will, hat die Aufgabe schon beschrieben: Punkt 3
der Was-zu-tun-Liste von `260815-1216`, die Prüfung an den Bau oder an eine Probe zu hängen,
und dabei nicht nur den Marker, sondern den ganzen Pfad gegen den Dateibestand aufzulösen.

## Ablage

Gemeinsamer Speicher. Betrifft den Kern und die Directive keiner Runde.

---
Resolved: `crates/krk-core/src/verzeichnis/verweisziel.rs:95` verweist jetzt auf
`shared/decisions/260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`,
also auf Speicher und Namensteil, die die Datei wirklich traegt; die Sternform bleibt (Entscheid
`260815-1145`). Die zwei Fundstellen in `issues/260815-1713_c_…` und `history/260815-1758-…`
sind eingefrorene Aufzeichnungen und bleiben unberuehrt. Die Wiederkehr ist damit nicht
abgestellt — der Vorschlag dazu steht unveraendert als Punkt 3 in `260815-1216`.
