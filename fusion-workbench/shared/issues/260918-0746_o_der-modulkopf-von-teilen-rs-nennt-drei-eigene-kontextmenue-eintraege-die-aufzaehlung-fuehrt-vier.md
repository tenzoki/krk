Der Modulkopf von `teilen.rs` nennt drei eigene Kontextmenü-Einträge, die Aufzählung führt vier

---

`crates/krk-ui/src/appkit/teilen.rs:44` sagt, die Tabelle hänge in `menuNeedsUpdate:` „erst ihre drei eigenen Eintraege an — Zip, Unzip und Finder". `Kontextbefehl` führt seit dem 260907 vier Varianten: `Zippen`, `Entpacken`, `ImFinderOeffnen`, `ImFinderAnzeigen`. Die beiden Finder-Wege sind zu zweien geworden, der Kopf nennt sie weiter als einen.

---

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>

Gefunden als Nebenbefund beim Umbau des Kontextmenüs am 260918 (`260917-2216_*_das-kontextmenue-der-dateiliste-schliesst-sich-nach-einer-zehntel-sekunde-von-selbst.md`); dort steht er in einem geschlossenen Datensatz und wäre damit nicht wiederzufinden.

Erhoben mit `awk '/pub enum Kontextbefehl/,/^}/' crates/krk-ui/src/kommandos/kontextmenue.rs` gegen `grep -n 'drei eigene' crates/krk-ui/src/appkit/teilen.rs`.

**Warum die Zahl im Kopf steht und nicht bloß Zierat ist.** Der Absatz beschreibt die Reihenfolge, in der das Menü entsteht: die eigenen Einträge zuerst, der Teilen-Eintrag danach über `eintrag_anfuegen`. Wer die Reihenfolge nachrechnet, rechnet mit der falschen Zahl.

**Abnahme**

Der Kopf nennt keine Zahl mehr, sondern verweist auf `Kontextbefehl::ALLE` als die Auskunft, so wie `CLAUDE.md` es für dieselbe Aufzählung schon hält; oder er nennt vier und die beiden Finder-Wege einzeln. `make check` bleibt grün.
