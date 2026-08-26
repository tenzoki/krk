Ein Lauf ohne Runden besteht das Gate: alle zehn Zusagen gelten „in allen 0 Runden" als gehalten

---

`Zusage::gehalten_in` liefert bei einer leeren Rundenliste `Some((0, 0))`, `immer_gehalten`
vergleicht `0 == 0` und sagt `Some(true)`, und `bestanden()` sagt daraufhin für **jede** der zehn
Zusagen: gehalten. Der Bericht schreibt dann „alle zehn Zusagen halten ihr Mass in jeder Runde"
über null Messungen und `urteil()` je Zeile „gehalten in allen 0 Runden". Die Wache dagegen steht
in `main.rs` und nicht in den Funktionen, die `pub` sind und das Urteil fällen.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Mittel
**Betroffen:** `crates/krk-bench/src/messen.rs`

## Der Weg, Zeile für Zeile

- `Durchstich::fahren` (`messen.rs:743`) und `Gesamtlauf::fahren` (`messen.rs:1018`) sind beide
  `pub` und prüfen `self.runden` an keiner Stelle. Bei `runden == 0` läuft die Schleife
  (`messen.rs:746` bzw. `1039`) nicht, `rohrunden` bleibt leer, und `sammeln` liefert
  für jede Zusage ein leeres `Vec<Vec<Duration>>`.
- `gehalten_in`, Perzentil-Zweig (`messen.rs:579-583`): `perzentile` ist leer, `gehalten` ist 0,
  zurück kommt `Some((0, 0))`.
- `immer_gehalten` (`messen.rs:622-625`): `gehalten == runden`, also `Some(true)`.
- `bestanden` (`messen.rs:734-738` und `1009-1013`, beide `pub fn bestanden`): `Some(true) != Some(false)` → `true`.
- `urteil` (`messen.rs:1986-1994`): `Some((0, 0))` trifft den Zweig `gehalten == runden` und
  schreibt „gehalten in allen 0 Runden".

Der Anteils-Zweig verhält sich genauso: `.count()` über eine leere Rundenliste ist 0, und
`self.runden.len()` ist ebenfalls 0.

## Die Wache liegt in der falschen Schicht

`main.rs:277` und `main.rs:353` fangen `--runden 0` je Unterbefehl ab, und über die
Befehlszeile ist der Fall damit heute nicht erreichbar. Das ist aber genau das Muster, das
dieselbe Datei eine Bildschirmseite höher anders löst: `Messreihe::fahren` prüft
`wiederholungen == 0` **in sich selbst** (`messen.rs:144-149`) und liefert
`ErrorKind::InvalidInput`, obwohl auch dort ein Aufrufer in `main.rs` steht. Drei `fahren`, zwei
Haltungen; der Rufer muss wissen, welche gilt.

## Warum das nicht nur formal ist

Ein bestandenes Gate ist der Beleg, auf den sich `CLAUDE.md` und die Berichte unter `messungen/`
berufen. Ein Bericht, der „alle zehn Zusagen halten" sagt und in seiner Tabelle überall
`0.000 ms` trägt, ist von einem echten Lauf nur an der Spalte „Urteil" zu unterscheiden — und die
sagt dasselbe. Wer den Rückgabewert aus einem Skript liest, sieht 0.

## Denkbarer Weg

`self.runden == 0` in beiden `fahren` abweisen, in derselben Form wie `Messreihe::fahren` es tut.
Die Wachen in `main.rs` können bleiben; sie liefern die bessere Meldung. Der zweite Halbsatz
gehört zu `260826-1303_*_der-perzentil-zweig-hat-keine-wache-gegen-eine-runde-ohne-werte`.
