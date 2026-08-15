# Vier Verweise im Code nennen einen Marker, den ihr Ziel nicht mehr trägt, drei davon sind neu

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `crates/krk-ui/src/tabs.rs:550`, `:572`, `:1353`; `crates/krk-ui/src/appkit/editor.rs:525`, `:1054`, `:2020`; `crates/krk-ui/src/appkit/anwendung.rs:2835`; `shared/issues/260812-2253_o_sieben-verweise-im-circle-datensatz-der-runde-5-tragen-einen-gestorbenen-marker.md` (derselbe Fehlertyp in der Werkbank); `CLAUDE.md` (`## Bindende Grundlage`, „Jedes Suchmuster dieses Projekts, das `\.md` verlangt, hat einen blinden Fleck")

---

## Der Befund

Der Baum zitiert Entscheidungs- und Defektdatensätze 290-mal. **275 Zitate stehen in
der Sternform** `_*_`, die keinen Markerwechsel mitmacht; **15 nennen einen
ausgeschriebenen Marker**, und **vier davon sind heute falsch**:

| Fundstelle | zitiert | ist heute |
|---|---|---|
| `crates/krk-ui/src/tabs.rs:550`, `:572`, `:1353` | `260814-1830_a_bleibt-der-filtertext-…` | `_i_` |
| `crates/krk-ui/src/appkit/editor.rs:525` | `260807-2147_a_welche-dateien-oeffnet-der-editor-ueberhaupt.md` | `_i_` |
| `crates/krk-ui/src/appkit/anwendung.rs:2835` | `260814-2102_a_gehoert-die-fallunterscheidung-der-rueckschritt-taste-…` | `_i_` |
| `crates/krk-ui/src/appkit/editor.rs:1054`, `:2020` | `260810-0303_o_ein-ersetzen-und-ein-eingefuegtes-crlf-…` | `_c_` |

Nachzuzählen mit:

```sh
grep -rhoE "(decisions|issues)/[0-9]{6}-[0-9]{4}_[a-z*]_" crates/ xtask/ --include='*.rs' \
  | grep -oE "_[a-z*]_$" | sort | uniq -c | sort -rn
```

**Die drei Zitate in `tabs.rs` sind mit `897605e` neu entstanden und binnen derselben
Sitzung falsch geworden.** Sie ersetzten ein `_o_`, das gerade veraltet war, durch ein
`_a_` — und der Markerlauf `_a_` → `_i_` desselben Datensatzes fand am selben
Vormittag statt. Der Wechsel liegt zur Stunde noch unverfolgt im Arbeitsbaum
(`git status`: der `_a_`-Name gelöscht, der `_i_`-Name ungetrackt), das Zitat ist also
falsch, sobald er eingecheckt ist.

**Die Sternform ist in diesem Projekt der Regelfall und nicht eine Vorliebe.** Sie ist
gegen genau diesen Lauf immun: ein Datensatz durchläuft `_o_` → `_a_` → `_i_`, und
jedes ausgeschriebene Zitat auf dem Weg wird zweimal falsch. Der Datensatz
`shared/issues/260812-2253_o_sieben-verweise-im-circle-datensatz-der-runde-5-…` hält
denselben Fehlertyp für die Werkbank fest; für den Code ist er bisher nicht erhoben.

## Der zweite Verweis, der ins Leere zeigt

`crates/krk-ui/src/tabs.rs:1384`, im Doc-Kommentar der neuen Probe
`der_aufstieg_laesst_den_filtertext_stehen_wie_der_einstieg`:

> Die Probe faehrt ihn so, wie `Dateifenster::ordner_aufwaerts` ihn faehrt

`Dateifenster` gibt es (`crates/krk-ui/src/appkit/tabelle.rs:2640`), und es hat kein
`ordner_aufwaerts`. Die Methode gehört `DateifensterQuelle`
(`crates/krk-ui/src/appkit/tabelle.rs:1386`). Beide Typen stehen in derselben Datei,
also führt der Verweis nicht bloß nirgendwohin, sondern an die falsche der beiden
Klassen.

Der Verweis stimmt in der Sache: die Probe rechnet den Aufstieg mit
`krk_core::verzeichnis::aufwaerts` nach und ruft `ordner_setzen` mit dem verlassenen
Ordner als `auswahl` — genau das, was `DateifensterQuelle::ordner_aufwaerts` über
`ordner_lesen` tut. Falsch ist allein der Typname.

## Was zu tun ist

1. Die drei Zitate in `tabs.rs` auf die Sternform `_*_` bringen. Damit sind sie gegen
   den ausstehenden Markerlauf immun.
2. `Dateifenster::ordner_aufwaerts` in `tabs.rs:1384` zu
   `DateifensterQuelle::ordner_aufwaerts` berichtigen.
3. Die vier übrigen ausgeschriebenen Marker der Tabelle ebenfalls auf die Sternform
   bringen, und die elf, die heute noch stimmen, gleich mit: ein Zitat, das nur solange
   richtig ist, bis jemand den Datensatz weiterschiebt, ist eine Verabredung auf Zeit.
   Danach steht der Baum bei 290 Sternformen und keiner Ausnahme.

---
Resolved: Sieben Fundstellen berichtigt statt der vier gemeldeten. Geprüft wurden alle 17 ausgeschriebenen Markerzitate unter `crates/` und `xtask/` gegen den Dateibestand; betroffen waren vier Datensätze auf sieben Stellen, die zehn übrigen stimmten. `spikes/` und `messungen/` blieben unangetastet, weil sie nach der Ortsregel aus `CLAUDE.md` ihren damaligen Marker behalten. Nebenbei ein falscher Typname berichtigt: `Dateifenster::ordner_aufwaerts` heißt `DateifensterQuelle::ordner_aufwaerts`.

**Die Empfehlung dieses Datensatzes, künftig die Sternform `_*_` zu schreiben, ist nicht umgesetzt** und auch nicht verworfen: sie ist eine Verabredung über die Zitierform des ganzen Projekts und keine Frage dieser sieben Stellen. Sie steht als eigene Frage unter `shared/decisions/260815-1145_o_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md`.
