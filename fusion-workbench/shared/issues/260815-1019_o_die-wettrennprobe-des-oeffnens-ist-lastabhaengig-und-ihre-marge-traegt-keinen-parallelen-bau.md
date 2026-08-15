Die Wettrennprobe des Öffnens ist lastabhängig, und ihre Marge trägt keinen parallelen Bau

---

`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` (`crates/krk-core/tests/text.rs:792`) läuft auf einem unbelasteten Gerät in rund 2,7 s durch und reißt unter Last die feste Schranke von 15 s (`recv_timeout`). Dazwischen liegt kein Plateau: gemessen sind 2,7 s ohne Last, rund 13 s unter mittlerer Last und der Ausfall unter der Last eines parallelen `make check`. Damit fällt jeder Abnahmelauf aus, der neben einem zweiten Bau läuft — genau die Lage, die eine Sitzung mit parallel arbeitenden Agenten herstellt.

---

**Gefunden am:** 260815, Stand `f8297b6`
**Gefunden von:** coder; zweimal nachgemessen und zweimal berichtigt vom orchestrator
**Herkunft:** neben der Aufgabe T2 des Circles `260814-1551-tippen-filtert-dateiliste-flach-und-tief` gefunden, von jener Aufgabe strukturell unabhängig: `krk-core` führt `krk-ui` nicht unter seinen Abhängigkeiten. Die Probe gehört zum Editor der Runde 2 und nicht zum Filter, deshalb steht der Datensatz im gemeinsamen Speicher.

## Der Befund

| Lage | Läufe | Ergebnis | Dauer |
|---|---|---|---|
| während ein Agent parallel `make check` fuhr | 6 | alle ausgefallen | je 15,0 s |
| unbelastetes Gerät, `debug` | 6 | alle bestanden | 2,7 bis 3,5 s |
| unbelastetes Gerät, im vollen `cargo test --workspace` | 1 | bestanden | — |
| 12 Lastprozesse auf 16 Kernen, `debug` | 3 | alle bestanden | 9,1 / 11,3 / 12,9 s |

**Die Verteilung ist zweigipflig, nicht knapp.** Ein bestandener Lauf braucht ein Fünftel der Schranke; ein gescheiterter erreicht sie nie. Unter steigender Last wandert die Dauer stetig nach oben, von 2,7 s über 12,9 s bis über die Schranke. Die Ursache ist damit die Marge und nicht ein Sprung im Verhalten.

## Zwei Fehlschlüsse auf dem Weg hierher, beide festgehalten

**Der erste war die Lesart „liegt dicht an der Schranke".** Sie stützte sich auf einen berichteten `release`-Lauf von 4,66 s neben Ausfällen bei 15 s. Zwischen 4,66 s und 15 s liegt der Faktor drei; das ist keine Nähe zu einer Grenze, sondern genau die Zweigipfligkeit, die erst die Messung unter Last erklärt hat.

**Der zweite war meiner und wog schwerer: „fällt sechsmal von sechs, also deterministisch".** Als Beleg diente, dass jeder Ausfall exakt bei 15,0 s lag und nicht knapp darunter. Dieser Beleg trägt nichts: **die Dauer eines Ausfalls ist per Bau die Schranke selbst**, denn gemessen wird ein `recv_timeout(15 s)`. Ein Zeitablauf sieht immer gleich lang aus, gleich woran er liegt. Aus der Gleichförmigkeit einer Zahl, die eine Konstante ist, wurde auf Determinismus geschlossen, und daraufhin ein Bericht als nicht reproduzierbar verworfen, der in der Sache recht hatte. Die sechs Ausfälle lagen sämtlich im selben Zeitfenster, in dem ein zweiter Agent das Gerät auslastete; die Läufe waren nicht unabhängig, sondern sechsmal dieselbe Lage.

## Was daraus für den Abnahmelauf folgt

`make check` ist auf diesem Gerät **nicht verlässlich grün**, sobald ein zweiter Bau daneben läuft. Ein rotes Ergebnis dieser einen Probe belegt dann nichts über den Baum. Wer eine Abnahme fährt, fährt sie ohne zweiten Bau daneben, oder er nimmt diese Probe ausdrücklich aus und sagt es im Bericht.

## Was die Probe weiterhin nicht zeigt

Die Meldung des Ausfallzweigs behauptet eine Ursache, die die Probe nicht misst: „das Oeffnen haengt an der benannten Roehre". Sie sieht allein, dass der Kanal in 15 s nichts geliefert hat, und kann einen Zeitablauf nicht von einem hängenden Öffnen unterscheiden. Nach der Lastmessung ist die Marge die naheliegende Erklärung; ausgeschlossen ist die andere damit nicht, denn kein Lauf hat je den Zustand der beiden Fäden im Ausfall festgehalten.

## Was zu tun ist

1. **Den Ausfallzweig um die beiden Zähler `gelaufen` und `getauscht` ergänzen.** Ohne sie ist im Ausfall nicht zu sehen, wie weit die Fäden gekommen sind, und die verbleibende Frage bleibt unentscheidbar. Ohne Abwägung richtig.
2. **Die Meldung berichtigen:** sie darf sagen, dass in der gesetzten Zeit nichts zurückkam, und die benannte Röhre als **eine** mögliche Ursache nennen. Wer die heutige Meldung liest, sucht am `O_NONBLOCK`-Pfad in `crates/krk-core/src/verzeichnis/sys.rs`, und dort ist nach heutigem Stand nichts.
3. **Erst danach über die Marge entscheiden.** Die Abwägung gehört dem Nutzer: eine höhere Schranke macht den Ausfall seltener und den Abnahmelauf im Fehlerfall länger; weniger Durchläufe schwächen die Aussage über das Wettrennen; ein `#[ignore]` nähme die Probe aus dem Abnahmelauf. Eine vierte Möglichkeit ist, die Schranke an der gemessenen Dauer des Laufs zu bemessen statt an einer festen Zahl.
