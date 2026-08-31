Der Nachschlag des Verlaufs setzt am letzten Commit an und verliert jeden Nebenzweig

---
`Gitleser::verlauf` (`crates/krk-core/src/git/leser.rs:194-220`) beginnt den Lauf bei `ab`, dem letzten schon angezeigten Commit:

```rust
let anfang = match ab { Some(id) => id, None => … };
let ueberspringen = usize::from(ab.is_some());
let lauf = self.repo.rev_walk([anfang]).all().ok()?;
for stand in lauf.skip(ueberspringen).take(zahl) { … }
```

Ein Lauf ab `anfang` liefert allein die **Vorfahren von `anfang`**. Der letzte Eintrag eines Schwungs beherrscht den Graphen aber nicht: bei `gix::revision::walk::Sorting::BreadthFirst`, der Vorbelegung, stehen mehrere Zweige nebeneinander in der Warteschlange, und der fünfzigste Commit kann auf einem davon liegen. Jeder Commit, der zum Zeitpunkt des Schwungendes noch in der Warteschlange stand und kein Vorfahre von `ab` ist, kommt danach **nie mehr** in die Liste.

Am Beispiel aus dem Kopf von `gix-0.87.1/src/revision/walk.rs:25-31` (`1-2-4-7` und `1-3-5-6`, `8` ist die Zusammenführung von `7` und `6`) mit einem Schritt von drei: der erste Schwung liefert `8, 6, 7`, `ab` ist `7`, der zweite Schwung läuft über die Vorfahren von `7` und liefert `4, 2, 1`. `5` und `3` fehlen dauerhaft.

KRKs eigenes Repository ist linear, und der Abnahmelauf aus Schritt 17 sähe den Befund dort nicht.

**Abnahmetest:** ein Prüfrepository mit einer Zusammenführung zweier Zweige und mehr Commits als `VERLAUFSSCHRITT`; die Vereinigung aller Schwünge trägt jeden Commit des Repositorys genau einmal. Eine Probe dieser Form steht heute nicht in `crates/krk-core/tests/git.rs`: `der_erste_aufruf_liefert_fuenfzig_commits` und `drei_commits_liefern_drei_und_melden_das_ende` messen beide an einer linearen Kette.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23, beim Lesen von `Gitleser::verlauf` gegen C4.2 und C4.3. Betrifft die Verlaufsliste des Git-Bereichs in jedem Repository mit Zusammenführungen.
Verwandt: `260831-1444_*_der-verlauf-laeuft-in-graphenreihenfolge-und-nicht-nach-commit-zeit.md` (dieselbe Funktion, andere Hälfte).
