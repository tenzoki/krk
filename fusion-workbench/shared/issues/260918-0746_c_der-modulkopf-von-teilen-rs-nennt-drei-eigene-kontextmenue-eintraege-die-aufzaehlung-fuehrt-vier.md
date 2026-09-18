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

Resolved: Der Kopf nennt keine Zahl mehr. `crates/krk-ui/src/appkit/teilen.rs:43-49` sagt jetzt „erst ihre eigenen Eintraege an — **welche und wie viele, sagt `Kontextbefehl::ALLE` und keine Zahl an dieser Stelle**", mit dem Verweis auf diesen Datensatz und dem Hinweis, dass die Zahl seit der Runde 17 zweimal gestiegen ist; die Form des Menüs steht dort als „Teilen, Trenner, die eigenen Eintraege in der Reihenfolge jener Liste". Das ist die erste der beiden Abnahmemöglichkeiten. Behoben nebenbei beim Bau des Untermenüs „Öffnen mit" (Arbeitspaket `260917-1415-kontextmenue-traegt-open-with`), das die Zahl auf fünf gebracht und den alten Wortlaut ein zweites Mal falsch gemacht hätte. `make check` grün, fünf von fünf (260918).
