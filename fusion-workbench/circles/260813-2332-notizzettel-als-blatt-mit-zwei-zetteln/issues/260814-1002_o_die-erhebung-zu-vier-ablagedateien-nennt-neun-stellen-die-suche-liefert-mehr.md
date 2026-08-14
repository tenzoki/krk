Die Erhebung zu „vier Ablagedateien" nennt neun Stellen; ihr eigenes Suchmuster liefert mehr

---

`260814-0912_o_neun-stellen-sprechen-weiter-von-vier-ablagedateien-es-sind-sechs.md` nennt sein
Suchmuster selbst: „das Suchmuster war `vier Dateien` und `vier Ablagedateien` über `crates/`".
Dasselbe Muster liefert am 260814-1002 siebzehn Fundstellen, nicht neun plus die zwei
ausdrücklich richtigen.

Nicht in der Tabelle des Datensatzes stehen:

| Stelle | Was dort steht | Einordnung |
|---|---|---|
| `krk-core/src/ablage/mod.rs:59` | Überschrift „Eine der vier Dateien entsteht einmal und wird nie wieder geschrieben" | meint `settings.toml` unter den vier TOML-Dateien; wie `:447` und `:512` genauer mit „TOML" |
| `krk-core/tests/ablage.rs:52` | „Laedt eine der vier Dateien so, wie der Betrieb es tut" | Hilfsfunktion für den TOML-Weg; richtig, gewinnt mit „TOML" |
| `krk-core/tests/ablage.rs:69` | „Schreibt eine der vier Dateien unter der Schreibsperre" | dieselbe Sorte |
| `krk-core/tests/ablage.rs:94` | „Die vier Ablagedateien, die TOML tragen" | ausdrücklich richtig, sagt „die TOML tragen" schon |
| `krk-core/tests/ablage.rs:122` | „Damit die Zusage ‚alle vier Dateien' trotzdem an vier Dateien geprueft wird" | richtig im TOML-Zusammenhang |
| `krk-core/tests/ablage.rs:1018` | „Der Pfad, unter dem die Sicherung einer der vier Dateien zu erwarten ist" | die Beiseitesicherung betrifft jetzt auch die zwei Zettel |
| `krk-core/tests/ablage.rs:1043` | „Alle vier Dateien werden gesichert, und das Original bleibt liegen" | dieselbe Sorte |

Zwei davon sind Befunde derselben Art wie die neun (`:1018`, `:1043`, dazu `mod.rs:59` als
Genauigkeitsgewinn), die übrigen sind im TOML-Zusammenhang richtig.

---

**Schwere:** niedrig. Kein Bau, kein Verhalten. Der Datensatz vom 260814-0912 bleibt richtig in
allem, was er sagt; er ist unvollständig in dem, was er zu zählen verspricht.

**Es ist der Fall, den `CLAUDE.md` als Eigenschaft dieses Projekts führt.** Dort steht: „Wer eine
Erhebung fährt, erweitert das Muster, bevor er zählt", und der Anlass war eine Erhebung, die
fünfmal dieselben acht Stellen nicht gesehen hat
(`shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`).
Hier ist der blinde Fleck ein anderer und einfacherer: das Muster ist nicht zu eng, sondern die
Auswertung hat `crates/krk-core/tests/` weggelassen, ohne es zu sagen.

**Was zu tun ist.** Entweder die Tabelle des Datensatzes vom 260814-0912 um die drei Stellen
erweitern, die dieselbe Sorte sind, und den Titel auf zwölf ziehen; oder in seinem
Kontext-Abschnitt schreiben, dass die Erhebung `src/` und nicht `tests/` erfasst hat. Das Zweite
ist eine Zeile und macht die Zahl im Titel wahr.

**Kontext**

- Gefunden beim Abgleich der Runde 9, `history/260814-1002-reconciliation.md`.
- Gemessen mit `grep -rn 'vier Dateien\|vier Ablagedateien\|vier Lade- und Schreibmethoden' crates/`
  am Stand `79dab20`, siebzehn Zeilen.
