Die Bildlänge für L1 und L9 stammt aus der ersten Runde, und jede spätere Meldung fällt still weg

---

`rate = rate.or(gemeldete_rate)` (`crates/krk-bench/src/messen.rs:749` und `1042`) behält die
Bildwiederholrate der **ersten** Runde und verwirft die aller weiteren, ohne sie zu vergleichen.
An dieser einen Zahl hängt das gesamte Urteil über L1 und L9: die Bildlänge ist ihr Kehrwert, und
gegen die Bildlänge wird je Einzelwert entschieden, ob eine Eingabe ihr Bild erreicht hat.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Mittel
**Betroffen:** `crates/krk-bench/src/messen.rs`, `crates/krk-bench/src/bericht.rs`

## Das `.or` greift immer, nicht nur ausnahmsweise

`Option::or` behält den linken Wert, sobald er `Some` ist. Sowohl `spannen_messen`
(`messen.rs:880-888`) als auch `sitzung_messen` (`messen.rs:1233-1241`) brechen ab, wenn die
Anwendung **keine** Rate meldet — `rate.is_none()` führt dort je zu einem `io::Error`. Damit ist
`gemeldete_rate` auf dem Erfolgsweg immer `Some`, und das `.or` verwirft folglich in **jeder**
Runde ab der zweiten den gemeldeten Wert. Der Ausdruck liest sich wie „nimm die erste, die
kommt", tut aber „nimm die erste und sieh dir die übrigen nicht an".

## Die Haltung daneben ist die entgegengesetzte

`bildlaenge_bilden` (`messen.rs:662-683`) bricht ab, statt 60 Hz zu unterstellen, und begründet
das ausdrücklich: „Fehlt die Rate, bricht die Auswertung ab, statt 60 Hz zu unterstellen." Eine
Rate, die zwischen den Runden **wechselt**, ist derselbe Fall aus der anderen Richtung — nicht
eine fehlende Angabe, sondern zwei widersprüchliche —, und sie wird nicht abgebrochen, sondern
stillschweigend nach der ersten aufgelöst.

## Was der Bericht daraus macht

`rate_beschreiben` (`messen.rs:2078-2086`) und der Kopf des Abnahmeberichts
(`bericht.rs:236-245`) schreiben **eine** Zahl aus, „gelesen aus NSScreen.maximumFramesPerSecond
am Bildschirm des gemessenen Fensters", ohne Runde. Der Leser hat keine Möglichkeit zu erkennen,
dass die Zahl nur für Runde 1 belegt ist.

## Wie es eintreten kann

`maximumFramesPerSecond` ist eine Eigenschaft des **Bildschirms**, nicht ein Momentanwert. Sie
wechselt nicht von selbst, aber sie wechselt mit dem Bildschirm: der Sitzungslauf startet je
Runde einen neuen `krk`-Prozess (`messen.rs:1226`), und wo dessen Fenster aufgeht, entscheidet
das System. Ein zwischen zwei Runden angeschlossener oder abgezogener Bildschirm, ein Fenster,
das auf dem zweiten Schirm landet — in beiden Fällen werden die Werte der späteren Runden gegen
eine fremde Bildlänge gehalten, und zwar in beiden Richtungen: gegen eine zu große Bildlänge
halten sie zu leicht, gegen eine zu kleine zu schwer.

Ich habe **nicht** nachgewiesen, dass das je eingetreten ist; die Berichte unter `messungen/`
weisen je eine Rate aus und könnten den Fall gar nicht zeigen.

## Denkbarer Weg

Statt `rate.or(…)` die gemeldete Rate je Runde sammeln und vor `bildlaenge_bilden` auf
Übereinstimmung prüfen; bei Abweichung abbrechen mit derselben Begründung wie bei einer fehlenden
Rate. Beide Fundstellen sind zwei Zeilen, `messen.rs:749` und `messen.rs:1042`.
