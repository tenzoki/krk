Ein Zitat im Circle-Datensatz des Web-Betrachters nennt einen Namensteil, den es nie gab

---

`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md:438` zitiert
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-2133_*_merkzeichen-einloesen-kostet-bei-tiefer-verschachtelung-mehr-als-der-rest-der-zerlegung.md`.
Diesen Dateinamen hat es nie gegeben. Der Datensatz heisst seit seiner ersten Fassung
`260812-2133_o_merkzeichen-einloesen-kostet-bei-tiefer-verschachtelung-das-zweieinhalbfache-und-verfehlt-l7-frueher.md`.
Das Zitat steht bereits in der Sternform; falsch ist der Namensteil.

---

**Severity:** Medium
**Found by:** coderev, Durchsicht der Zeigerreparatur 260818-0752
**Domain:** code

## Der Beleg

Der Zeitstempel `260812-2133` hat im ganzen Baum genau einen Traeger, und der Name hat nie
gewechselt:

```sh
git log --all --pretty=format: --name-only --diff-filter=A | grep '260812-2133' | sort -u
# fusion-workbench/circles/260812-1000-…/issues/260812-2133_o_merkzeichen-einloesen-kostet-bei-tiefer-verschachtelung-das-zweieinhalbfache-und-verfehlt-l7-frueher.md
```

Geschrieben wurde die Zeile am 260812 in Commit `0e09377` („das Portfolio nach dem Abschluss der
sechsten Runde"), also fuenf Tage vor dieser Sitzung. Sie ist von der Zeigerreparatur des
260818 nicht beruehrt: die betraf `crates/`, den Spec, den Plan und den Circle-Datensatz der
zwoelften Runde.

## Warum das mehr ist als ein Tippfehler

**Es ist der dritte Beleg desselben Fehlertyps in zwei Tagen**, und der erste ausserhalb der
zwoelften Runde. Die beiden anderen hat die Reparatur selbst gefunden und behoben:
`crates/krk-ui/src/appkit/textautomatik.rs:98` und `plan:261` trugen ebenfalls schon die
Sternform und einen falschen Namensteil
(`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260818-0710_*_step-16-killed-22-pointers-in-living-text-and-five-of-them-are-in-crates.md`).
Der Datensatz `260815-1145` hat den Fall vorhergesagt: „die Sternform haelt gegen einen
Markerwechsel und gegen nichts sonst"
(`shared/decisions/260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md:50`).

**Der Ort macht ihn teuer.** Ein `_a_circle.md` ist ein vorgesehener Circle: sein Abschnitt
`## Grounding snapshot` wird bei der Aktivierung als bindende Grundlage gelesen. Die Zeile
begruendet dort, dass L7 seit der sechsten Runde schon ab rund 12 kB verfehlt wird statt ab
19 kB. Wer den Beleg nachschlagen will, findet nichts und kann nicht unterscheiden, ob der
Datensatz geloescht wurde oder ob der Zeiger falsch ist.

## Erhebung

Alle 1016 Zitate der Form `YYMMDD-HHMM_x_<slug>` in den 46 nicht eingefrorenen Werkbankdateien
(alles ausser `history/`, `reviews/`, `analyses/`, `issues/`, `decisions/`, `archive/` nach der
Ortsregel in `CLAUDE.md` `## Bindende Grundlage`) sind gegen den Dateibestand aufgeloest worden,
Zeitstempel und Namensteil, abgekuerzte Formen ueber ein Praefix. Tote Zeiger: genau dieser eine.
Dieselbe Erhebung ueber `crates/`, `xtask/` und `resources/` (428 Zitate) meldet nach der
Reparatur keinen.

## Empfehlung

Die eine Zeile berichtigen. Der Datensatz traegt weiter `_o_`, also bleibt die Aussage der
Umgebung richtig.

Der breite Fix bleibt offen und ist derselbe, den `260818-0710` unter `## Fix` verlangt: eine
Pruefung, die jedes Zitat gegen den Dateibestand aufloest, Namensteil eingeschlossen. Der
Fehlertyp hat jetzt drei Belege, und keiner davon waere ueber den Marker allein gefunden worden.
