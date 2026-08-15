Die Wettrennprobe des Öffnens reißt die 15-Sekunden-Schranke in beiden Profilen

---

`cargo test --workspace` bricht mit `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` (`crates/krk-core/tests/text.rs:792`) ab: „die Durchlaeufe sind nach 15 Sekunden nicht fertig geworden; das Oeffnen haengt an der benannten Roehre". Die Meldung nennt eine Ursache, die die Probe nicht gemessen hat. Sechs Läufe am 260815 sind sechsmal ausgefallen, in `debug` wie in `release` und auch am Stand vor dieser Sitzung; ein Durchlauf ist nicht beobachtet.

---

**Gefunden am:** 260815, Stand `f8297b6`
**Gefunden von:** coder, nachgemessen und berichtigt vom orchestrator
**Herkunft:** neben der Aufgabe T2 des Circles `260814-1551-tippen-filtert-dateiliste-flach-und-tief` gefunden. Der Ausfall ist von jener Aufgabe unabhängig, und zwar strukturell und nicht nur beobachtet: `krk-core` führt `krk-ui` nicht unter seinen Abhängigkeiten, also kann eine Änderung an `crates/krk-ui/src/tabs.rs` eine Probe in `crates/krk-core/tests/text.rs` nicht erreichen. Die Probe gehört zum Editor der Runde 2 und nicht zum Filter, deshalb steht der Datensatz im gemeinsamen Speicher.

## Der Befund

Gemessen vom orchestrator am 260815 gegen 10:30, auf dem Referenzgerät:

| Lauf | Profil | Ergebnis | Dauer |
|---|---|---|---|
| `cargo test --workspace`, Stand `f8297b6` + T2 | `debug` | ausgefallen | 15,14 s |
| allein gefahren, dreimal | `debug` | dreimal ausgefallen | 15,02 / 15,04 / 15,02 s |
| allein gefahren | `release` | ausgefallen | 15,03 s |
| allein gefahren, Arbeitsbaum auf `c3fcdef` | `debug` | ausgefallen | 15,03 s |

**Sechs von sechs, kein einziger Durchlauf.** Die Dauer liegt jedes Mal auf der Schranke selbst und nicht knapp darunter, was für ein Wackeln an einer Grenze spräche.

**Zwei Messaussagen des ersten Datensatzes haben sich nicht bestätigt** und stehen hier, damit niemand sie später aus einer älteren Fassung wieder aufnimmt: „im Profil `release` läuft dieselbe Probe in 4,66 s durch" und „am 260815 zweimal ausgefallen und einmal durchgelaufen". Beides ist bei der Nachmessung nicht eingetreten. Woher der berichtete grüne Lauf kam, ist offen; er könnte an einer anderen Last des Geräts gelegen haben.

## Die Kette

Der Lesefaden läuft, bis **beides** erreicht ist: `DURCHLAEUFE = 20_000` eigene Durchläufe und `MINDESTENS_GETAUSCHT = 2_000` Tausche des zweiten Fadens. Die Notbremse `HOECHSTENS_DURCHLAEUFE = 10 * DURCHLAEUFE` liegt bei 200.000. Beides zusammen steht gegen eine feste Schranke von 15 s (`recv_timeout`).

Der Doc-Kommentar der Probe hält die Kopplung an den Tauscher fest und begründet sie damit, dass im Profil `release` der Lesefaden dem Tauscher davonläuft. Der umgekehrte Fall, dass die Kopplung die Zeitschranke reißt, steht dort nicht.

## Was nicht gemessen ist

**Ob die Probe das Richtige anzeigt, ist offen.** Sie kann eine überschrittene Zeit nicht von einem hängenden Öffnen unterscheiden: sie sieht allein, dass der Kanal in 15 s nichts geliefert hat. Damit steht beides nebeneinander und keines ist belegt — der Zuschnitt ist zu eng, **oder** das Öffnen hängt tatsächlich an der benannten Röhre, und die Probe fängt genau den Defekt, für den sie geschrieben wurde. Die Deutung des ersten Datensatzes, es liege am Zuschnitt, ist eine Erschließung und keine Messung; sie wird durch den deterministischen Ausfall in `release` eher geschwächt als gestützt.

Wer das entscheidet, misst zuerst, wie weit die beiden Fäden in den 15 s kommen: `gelaufen` und `getauscht` sind im Ausfallzweig nicht ausgegeben, und ohne diese zwei Zahlen ist die Frage nicht entscheidbar. Ein Lauf unter `truss` oder `dtruss` am `open`-Aufruf beantwortet die zweite Hälfte.

## Was zu tun ist

1. Den Ausfallzweig um die beiden Zähler ergänzen, damit der nächste Lauf die Frage überhaupt beantworten kann. Das ist ohne Abwägung richtig.
2. Die Meldung berichtigen: sie darf sagen, dass in der gesetzten Zeit nichts zurückkam, und die benannte Röhre als **eine** mögliche Ursache nennen. Wer die heutige Meldung liest, sucht am `O_NONBLOCK`-Pfad in `crates/krk-core/src/verzeichnis/sys.rs`.
3. Erst danach über den Zuschnitt entscheiden. Eine kleinere Zahl von Durchläufen schwächt die Aussage über das Wettrennen, eine größere Zeitschranke macht `cargo test` langsamer, und ein `#[ignore]` nähme die Probe aus dem Abnahmelauf. Diese Abwägung gehört dem Nutzer, sobald Punkt 1 die Zahlen geliefert hat.

**Solange dieser Datensatz offen ist, ist `make check` auf diesem Gerät rot**, und jede Abnahme muss diese eine Probe ausdrücklich ausnehmen, statt das rote Ergebnis zu übergehen.
