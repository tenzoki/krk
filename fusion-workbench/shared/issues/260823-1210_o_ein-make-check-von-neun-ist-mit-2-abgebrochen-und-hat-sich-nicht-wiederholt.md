Ein `make check` von neun ist mit 2 abgebrochen und hat sich nicht wiederholt

---

Am 260823 gegen 12:05 hat ein `make check` den Rückgabewert 2 geliefert. Acht weitere Läufe
unmittelbar davor und danach, am selben Baumstand, sind grün. **Die Ausgabe des roten Laufs
ist nicht erhalten**: er lief nach `/dev/null`, weil allein der Rückgabewert gebraucht war.
Damit ist nicht entscheidbar, welches der vier Kommandos abgebrochen ist.

---

**Beobachtet, nicht diagnostiziert.**

## Was gemessen ist

| Lauf | Kommando | Rückgabewert | Ausgabe erhalten |
|---|---|---|---|
| 1 | `make check` | 0 | ja, „alle vier gruen" |
| 2 | `make check` | 0 | ja, „alle vier gruen" |
| 3 | `make check` | **2** | **nein** |
| 4 | `make check` | 0 | ja, „alle vier gruen" |
| 5 | `make check` | 0 | ja, keine Zeile mit `FAILED`, `panicked` oder `error` |
| 6–8 | `make check` | 0 | ja |
| 9–12 | `cargo test --workspace` | 0 | ja |

Zwischen Lauf 2 und Lauf 3 lag genau eine Änderung: der Umbruch einer einzelnen
Kommentarzeile in `crates/krk-ui/src/kommandos/mod.rs`. Lauf 4 lief unmittelbar danach ohne
weitere Änderung und war grün, also erklärt die Änderung den Abbruch nicht.

## Warum das hier steht

`make check` ist das Abnahmekommando dieses Baums, und ein Lauf, der ohne erkennbaren Grund
rot wird, macht jede Abnahme, die sich auf ihn stützt, um so viel unsicherer. Zwei
Kandidaten liegen nahe und sind hier **nicht** geprüft:

1. **Der Messplanwächter.** `Messplanwaechter::neu` (`krk-bench/src/messen.rs`) räumt beim
   Anlegen jede fremde `krk-messplan-*.toml` im Temporärverzeichnis ab, und die Probe
   `der_messplan_traegt_die_pruefsitzung_…` ruft `plan_schreiben`, also räumt auch
   `cargo test` dort ab. Vorausgesetzt ist, dass nie zwei Läufe zugleich darauf greifen
   (`shared/issues/260810-1925_*`). Ob zwei Prüffäden desselben Laufs das können, ist hier
   nicht nachgesehen.
2. **Etwas außerhalb dieses Baums**, etwa ein abgebrochener Übersetzerlauf.

## Was zu tun wäre

Den nächsten roten Lauf **mit** seiner Ausgabe festhalten. Ohne sie ist der Befund nicht
weiter zu treiben; deshalb steht hier keine Vermutung als Ursache.

**Schwere:** niedrig, solange er sich nicht wiederholt.

**Gefunden:** coder, beim Abschluss der acht Befunde vom 260823, Baumstand `471d801` plus
den Änderungen jener Arbeit

**Domain:** code
