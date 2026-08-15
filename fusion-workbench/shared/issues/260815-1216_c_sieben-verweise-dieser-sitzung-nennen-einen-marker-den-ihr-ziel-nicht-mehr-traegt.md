Sieben Verweise dieser Sitzung nennen einen Marker, den ihr Ziel nicht mehr trägt

---

Die Sitzung vom 260815-0912 hat den Defekt `shared/issues/260815-1047_c_vier-verweise-im-code-nennen-einen-marker-den-ihr-ziel-nicht-mehr-traegt-drei-davon-sind-neu.md` geschlossen und dabei sieben Fundstellen unter `crates/` berichtigt. Am Stand `9a2d0e0` stehen neun Verweise falsch, und **sieben davon hat dieselbe Sitzung geschrieben**. Fünf waren in dem Commit falsch, der sie schrieb: `9a2d0e0` hat den Entscheidungsdatensatz von `_a_` auf `_i_` gezogen und die beiden Defektdatensätze als `_c_` und `_d_` angelegt, und derselbe Commit schrieb fünf Zitate auf den jeweils alten Marker.

---

**Gefunden am:** 260815-1216, Stand `9a2d0e0`
**Gefunden von:** reconciler, beim Abgleich der Sitzung 260815-0912
**Herkunft:** kein Circle war aktiv; der Befund betrifft den Circle `260814-1551-tippen-filtert-dateiliste-flach-und-tief` und den Baum gleichermaßen und liegt deshalb im gemeinsamen Speicher.

## Der Befund

| Fundstelle | zitiert | ist heute | geschrieben in |
|---|---|---|---|
| `crates/krk-ui/src/tabs.rs:564` | `260815-1047_o_c1-9-und-der-doc-kommentar-…` | `_c_` | `9a2d0e0` |
| `crates/krk-ui/src/tabs.rs:582` | `260815-1047_o_die-bedingung-der-moeglichkeit-2-…` | `_d_` | `9a2d0e0` |
| `…/planning/260814-1830_o_spec-…:308` (C1.9) | `260814-1830_a_bleibt-der-filtertext-…` | `_i_` | `9a2d0e0` |
| `…/planning/260814-1830_o_spec-…:308` (C1.9) | `260815-1047_o_c1-9-und-der-doc-kommentar-…` | `_c_` | `9a2d0e0` |
| `…/planning/260814-1830_o_spec-…:488` (Fragentabelle) | `260814-1830_a_bleibt-der-filtertext-…` | `_i_` | `f8297b6` |
| `…/planning/260814-2102_c_plan-…:578` (Ankreuzliste) | `260814-1830_a_bleibt-der-filtertext-…` | `_i_` | `f8297b6` |
| `…/_b_circle.md:89` (Closure-Notiz) | `260815-1047_o_die-bedingung-der-moeglichkeit-2-…` | `_d_` | `9a2d0e0` |

Dazu zwei Fundstellen aus der Runde 10 selbst, die schon vorher falsch standen:

| Fundstelle | zitiert | ist heute |
|---|---|---|
| `…/planning/260814-2102_c_plan-…:585` | `260814-2102_a_gehoert-die-fallunterscheidung-…` | `_i_` |
| `…/planning/260814-2102_c_plan-…:587` | `260814-2102_o_der-pruefschritt-fuer-die-sichtbarkeit-…` | `_c_` |

`…` steht für `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/`.

Nachzuzählen mit dem Muster, das Code und Werkbank gleichermaßen liest und den Fund gegen den Dateibestand auflöst:

```sh
grep -rnoE "[0-9]{6}-[0-9]{4}_[a-z]_[a-z0-9-]{6,}" \
  crates/ xtask/ \
  fusion-workbench/circles/*/planning fusion-workbench/circles/*/_*_circle.md \
  fusion-workbench/shared/planning
```

Jeder Fund wird danach über `find fusion-workbench -name '<zeitstempel>_?_<thema>*.md'` aufgelöst und der gefundene Marker mit dem zitierten verglichen.

## Warum die Berichtigung vom 260815 den Rückfall nicht verhindert hat

Der geschlossene Datensatz hat sein Muster ausdrücklich auf `crates/` und `xtask/` beschränkt und die Werkbank nicht mitgelesen; fünf der neun Fundstellen liegen in der Werkbank. Und er hat die Fundstellen zu einem Zeitpunkt gezählt, zu dem die beiden neuen Zitate in `tabs.rs` noch nicht geschrieben waren: sie entstanden im selben Commit, der die zitierten Defektdatensätze als `_c_` und `_d_` anlegte. **Eine Prüfung, die einmal von Hand läuft, kann eine Änderung desselben Commits nicht sehen.** Das ist der Punkt, an dem die Frage `shared/decisions/260815-1145_o_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md` ansetzt; sie hält in ihren `## Constraints` schon fest, dass jede Antwort ohne eine Prüfung eine Verabredung bleibt.

## Die Ortsregel begrenzt den Befund

`CLAUDE.md` legt unter `## Bindende Grundlage` fest, dass Aufzeichnungen eines Standes ihren damaligen Marker behalten: `history/`, `reviews/`, `analyses/`, `issues/`, `decisions/`, `messungen/` und `spikes/`. Zitate dort sind kein Befund und sind hier nicht gezählt. Gezählt sind allein die lebenden Flächen: der Code, die Plandateien und die Circle-Datensätze.

## Was zu tun ist

1. **Die neun Fundstellen berichtigen** — oder, falls die Frage `260815-1145` auf die Sternform fällt, sie gleich in diese Form bringen. Die Reihenfolge ist umgekehrt sinnvoll: erst die Frage beantworten, dann einmal umschreiben, statt zweimal.
2. **Das Muster auf die Werkbank ausdehnen.** Die Beschränkung auf `crates/` und `xtask/` hat fünf der neun Fundstellen nicht gesehen.
3. **Die Prüfung an den Bau oder an eine Probe hängen.** Ohne sie ist der nächste Rückfall eine Frage der Zeit; er ist am 260815 in einer Sitzung zweimal eingetreten.

---
Resolved: Behoben mit `e49412a` (Zitierform-Entscheid
`shared/decisions/260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md`,
Möglichkeit 2). Am 260815-1400 gegen den Baum gelesen: alle sechs von diesem Datensatz
namentlich genannten Fundstellen tragen die Sternform, `crates/krk-ui/src/tabs.rs:564`
und `:582` ebenso wie die drei Stellen in Spec und Plan der Runde 10.

**Erhebung über den ganzen lebenden Text**, also `crates/`, `xtask/`, `CLAUDE.md`, die
Circle-Datensätze und die Dateien unter `planning/`, mit einem Muster ohne die Endung
`\.md` und damit ohne den blinden Fleck aus `CLAUDE.md`: zwei Fundstellen mit
ausgeschriebenem Marker bleiben, und beide sind die Ausnahmen, die der Entscheid selbst
benennt — Sätze über Zitate, in denen der Marker die Aussage ist und kein Zeiger
(`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_*_plan-*.md:1670`
und `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/planning/260813-0205_*_plan-*.md:625`).
Die eingefrorenen Orte der Ortsregel sind unangetastet.

**Die Grenze der Antwort steht im Entscheid und gilt weiter:** die Sternform hält gegen
einen Markerwechsel und gegen nichts sonst. Wird der Namensteil eines Datensatzes
umgeschrieben, zeigt das Zitat weiter ins Leere, und ohne Prüfung im Bau bemerkt das
niemand, bis jemand danach sucht.
