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

**Resolved:** 260831. `Gitleser::verlauf` nimmt jetzt `bereits: usize` statt `ab: Option<ObjectId>`
und läuft in jedem Schwung von HEAD los, wie `git log --skip`: `rev_walk` gibt jeden erreichbaren
Commit genau einmal aus, also zerlegen die Schwünge den Verlauf in Stücke, statt ihn am letzten
angezeigten Commit zu beschneiden. `Gitfrage::WeitererVerlauf` trägt dieselbe Zahl,
`Tabliste::verlauf_nachladen` nimmt sie aus `Gitmodell::verlaufslaenge`, und `Gitmodell::letzter_commit`
fällt damit als Rufer weg und ist gestrichen.

**Gemessen und nicht gelesen.** Das Prüfrepository der neuen Probe
`die_vereinigung_aller_schwuenge_traegt_jeden_commit_genau_einmal`
(`crates/krk-core/tests/git.rs`) trägt zwei Zweige von je dreißig Commits und eine Zusammenführung
darüber, also 62 Commits gegen einen `VERLAUFSSCHRITT` von 50. Gegen den alten Stand sah die
Blätterschleife **56 von 62**; gegen den neuen trägt die Vereinigung aller Schwünge jeden Commit
aus `git rev-list HEAD` genau einmal. Der Sollstand kommt von `git` und nicht aus einer Zahl in der
Probe.

**Zum Nachbardatensatz** `260831-1444_*_der-verlauf-laeuft-in-graphenreihenfolge-und-nicht-nach-commit-zeit.md`:
der Weg verbaut ihn nicht, er macht ihn erst gefahrlos. Ein `.sorting(…)` am Lauf gilt jetzt jedem
Schwung gleich, weil jeder Schwung derselbe Lauf von HEAD aus ist; mit dem Ansatz am letzten
angezeigten Commit hätte jede Sortierung außer der Graphenreihenfolge den Verlust noch vergrößert.
Der Doc-Kommentar von `verlauf` nennt die Stelle namentlich.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23, beim Lesen von `Gitleser::verlauf` gegen C4.2 und C4.3. Betrifft die Verlaufsliste des Git-Bereichs in jedem Repository mit Zusammenführungen.
Verwandt: `260831-1444_*_der-verlauf-laeuft-in-graphenreihenfolge-und-nicht-nach-commit-zeit.md` (dieselbe Funktion, andere Hälfte).
