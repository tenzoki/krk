Der Messplanwächter entsteht erst nach dem Schreiben und deckt das Schreiben selbst nicht

---

`plan_schreiben` (`crates/krk-bench/src/messen.rs:1594-1596`) legt die Datei an und baut den
Wächter erst danach:

```rust
let pfad = std::env::temp_dir().join(format!("krk-messplan-{}.toml", std::process::id()));
std::fs::write(&pfad, text)?;
Ok(Messplanwaechter { pfad })
```

`std::fs::write` ist `File::create` und `write_all`. Scheitert das Schreiben, nachdem das
Anlegen gelungen ist — kein Platz mehr, Kontingent voll, E/A-Fehler —, dann steht die
angelegte, leere oder halbe Datei auf der Platte, und das `?` kehrt zurück, bevor es einen
Wächter gibt. Genau die Lücke, die `ed5c896` schließen wollte, bleibt für diesen einen
Ausgang offen.

---

**Schwere:** Niedrig
**Gefunden:** coderev, Durchsicht des Codeanteils von Turn 1
(`shared/history/260810-1647-orchestrator-session.md`)
**Betroffen:** `crates/krk-bench/src/messen.rs`
**Domain:** code

## Was der Wächter sonst deckt

Geprüft und in Ordnung: der Erfolgsweg, jedes `?` innerhalb der Rundenschleife, das `?` von
`Sitzungssicherung::anlegen` eine Zeile darunter (`messen.rs:1034`), das `?` von
`bildlaenge_bilden` hinter der Schleife (`messen.rs:1053`) und die Panik-Abwicklung — der
Arbeitsbereich setzt kein `panic = "abort"`, das ist in keiner `Cargo.toml` gesetzt. Kein
`std::mem::forget` und kein `ManuallyDrop` steht im Baum
(`grep -rn 'mem::forget\|ManuallyDrop' crates/krk-bench/src/` ist leer).

Die Lebensdauer hält: `plan` wird bei `messen.rs:1029` gebunden und lebt bis zum Ende von
`fahren`; die einzige Leserin ist die Anwendung im Unterprozess, und `warten_bis`
(`messen.rs:1684-1697`) wartet auf ihr Ende oder bringt sie mit `kill` um. Nach dem Fall des
Wächters liest niemand mehr.

`process::exit` in `signalwache_starten` (`messen.rs:1367`) ist bekannt und liegt als
`shared/issues/260810-1745_o_der-messplanwaechter-greift-bei-strg-c-nicht-weil-process-exit-kein-drop-laeuft.md`.

## Denkbarer Weg

Den Wächter vor dem Schreiben bauen, so wie `Wegwerfordner::neu`
(`crates/krk-bench/src/wegwerfordner.rs:39-48`) seinen Namen vor dem Anlegen festhält:

```rust
let waechter = Messplanwaechter { pfad: … };
std::fs::write(waechter.pfad(), text)?;
Ok(waechter)
```

Dann fällt der Wächter auch dann, wenn das Schreiben in der Mitte scheitert, und der
`Drop`-Aufruf auf eine nie angelegte Datei ist folgenlos — `remove_file` liefert dort
`NotFound`, und der Rückgabewert wird ohnehin verworfen.

## Nebenbei, ungeprüft

Der Dateiname trägt allein die Prozesskennung. Zwei Aufrufe von `plan_schreiben` **im selben
Prozess** benennen dieselbe Datei, und der `Drop` des einen Wächters räumt die Datei des
anderen weg. Heute gibt es genau einen Aufruf in `fahren` und einen in der Probe
`der_messplan_traegt_die_pruefsitzung_in_der_serialisierung_der_sitzung`
(`messen.rs:2593`), also fällt das nicht an; `libtest` fährt Proben aber nebenläufig, und
eine zweite Probe, die `plan_schreiben` ruft, liefe in diese Kante. `Wegwerfordner` hat den
Fall über einen `AtomicU64`-Zähler neben der Prozesskennung schon beantwortet
(`wegwerfordner.rs:29`, `wegwerfordner.rs:41-45`). Kein Defekt heute, aber die Kante ist mit
dem Wächter näher gerückt, weil der Zeitpunkt des Abräumens nicht mehr im Programmtext
steht.

## Dringlichkeit

Gering. Der Fall verlangt ein Dateisystem, das beim Schreiben einer wenige Kilobyte großen
Datei scheitert, nachdem es sie angelegt hat. Der Schaden ist eine liegengebliebene Datei im
Temporärverzeichnis. Kein Abnahmekriterium und keine der zehn Zeitzusagen aus C8 sind
berührt.

---
Resolved: `Messplanwaechter::neu()` legt jetzt allein den Namen fest, und `plan_schreiben`
schreibt danach ueber `waechter.pfad()`. Damit deckt der Waechter das Schreiben selbst mit ab:
scheitert es, nachdem die Datei angelegt ist, faellt der Waechter auf dem `?`-Weg und raeumt.

Die Form ist die von `Wegwerfordner::neu` (`crates/krk-bench/src/wegwerfordner.rs`), es ist
keine zweite entstanden. Der Zaehlerteil von `Wegwerfordner` ist bewusst nicht uebernommen: er
ist die im Datensatz genannte ungeprueft-Kante und heute kein Defekt.
Abgenommen mit `make check`, exit 0.

Geschlossen in der Sitzung `shared/history/260810-1647-orchestrator-session.md`, Turn 2.
