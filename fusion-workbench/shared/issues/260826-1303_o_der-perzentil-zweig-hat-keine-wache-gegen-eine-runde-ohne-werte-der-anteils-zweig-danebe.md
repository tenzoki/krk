Der Perzentil-Zweig hat keine Wache gegen eine Runde ohne Werte, der Anteils-Zweig daneben schon

---

In `Zusage::gehalten_in` steht im Anteils-Zweig ausdrücklich „Eine Runde ohne Werte haelt nicht"
(`messen.rs:597-600`). Der Perzentil-Zweig unmittelbar darüber (`messen.rs:579-583`) hat diese
Wache nicht — und `perzentil(&[], …)` liefert `Duration::ZERO` (`messen.rs:306-309`), was jede
Grenze unterbietet. Dieselbe Eingabe, eine Runde ohne Werte, fällt bei den beiden Abnahmemaßen
also entgegengesetzt aus: verfehlt beim Anteil, **gehalten** beim Perzentil.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Mittel
**Betroffen:** `crates/krk-bench/src/messen.rs`
**Cross-references:** `shared/issues/260826-1302_*_ein-lauf-ohne-runden-besteht-das-gate-…`

## Die fehlende Messung liest sich als die schnellste

Die Rückfallwerte ziehen in dieselbe Richtung. `bestes_perzentil`, `minimum` und `maximum`
(`messen.rs:472`, `491`, `496`) enden alle auf `unwrap_or_default()`, also auf
`Duration::ZERO`. Eine Zusage ohne Werte steht damit in der Zahlentabelle beider Berichte
(`messen.rs:1913-1934`, `bericht.rs:341-362`) mit `0.000 ms` in jeder Spalte und trägt in der
Spalte Urteil „gehalten". Null ist hier nicht der neutrale Wert, sondern der bestmögliche: eine
abwesende Messung ist von einer unendlich schnellen nicht zu unterscheiden.

## Erreichbarkeit, ehrlich benannt

Über die Befehlszeile ist der Fall heute **nicht** erreichbar. `hole` in `eine_runde`
(`messen.rs:822-833`, Zeile 822) und in `eine_gesamtrunde` (`messen.rs:1173-1184`) verlangt genau
`self.wiederholungen` Werte und verwirft die Reihe sonst, und `wiederholungen` kommt aus
`WIEDERHOLUNGEN = 20`. Der Befund ist damit keine gemeldete Fehlmessung, sondern eine
Asymmetrie, die auf ihren ersten Rufer wartet.

Sie wartet allerdings an einer offenen Tür: `Zusage` ist `pub` mit `pub`-Feldern
(`messen.rs:451-460`), `gehalten_in` und `immer_gehalten` sind `pub`, und `perzentil` selbst ist
`pub` (`messen.rs:306`). Wer eine elfte Zusage baut oder eine Größe hinzunimmt, deren Werte die
Anwendung nur unter Bedingungen liefert, bekommt beim Perzentilmaß ein stilles „gehalten" und
beim Anteilsmaß ein „verfehlt".

## Denkbarer Weg

Dieselbe Wache in den Perzentil-Zweig, wortgleich zur bestehenden: eine Runde ohne Werte hält
nicht. Für die Kennzahlen (`bestes_perzentil`, `minimum`, `maximum`) ist der Rückfall auf null
für sich genommen vertretbar, solange das Urteil ihn nicht mehr als „schnell" liest — geprüft
werden sollte trotzdem, ob der Bericht an solchen Stellen `-` schreiben kann, wie er es für die
Spalten „im Bild" und „hoechstwert" schon tut (`messen.rs:1925` und `1929`).
