Der Doc-Kommentar des Giteinzugs sagt, die wartende Markenmeldung falle mit dem Lauf

---
`gitmeldungen_einziehen` (`crates/krk-ui/src/tabs.rs:1379-1384`) schreibt:

> **Die zurueckgehaltene Meldung haelt den Lauf nicht am Leben.** Der Kanal ist drei tief, der Arbeitsfaden blockiert also an keiner der drei Meldungen und endet auch dann, wenn niemand sie holt; das Feld faellt mit dem Lauf weg, sobald der Kanal schliesst …

Der Absatz handelt von `wartende_marken`, und auf dieses Feld gelesen ist der Satz falsch. Beim geschlossenen Kanal fällt allein der Halter (`tabs.rs:1407-1409`):

```rust
if kanal_zu { tab.gitlauf = None; }
```

`tab.wartende_marken` bleibt stehen — und muss es: der Kanal schließt in einem großen Ordner regelmäßig, **bevor** der Bestand gelesen ist, und die Meldung wird erst danach eingetragen (`tabs.rs:1421-1437`). Fiele sie mit dem Kanalschluss, wäre der Befund für immer weg; das ist genau die Alternative, die der Doc-Kommentar an `Tabinhalt::wartende_marken` (`tabs.rs:136-141`) als verworfen ausschreibt. Die zwei Stellen, an denen das Feld wirklich fällt, sind `gitlauf_nachziehen_an` (`tabs.rs:1201`) und `abbrechen` (`tabs.rs:1239`).

**Der Satz ist daneben mehrdeutig**, und das ist der zweite Teil des Befunds: „das Feld" kann auch `tab.gitlauf` meinen, von dem der Absatz darüber spricht („das Feld stehen zu lassen hielte den Einzugstakt fuer immer am Laufen", `:1372-1373`). Auf jenes gelesen stimmt er. Ein Satz, der je nach Bezugswort das eine oder sein Gegenteil sagt, taugt an dieser Stelle nicht: er beschreibt die Zusage, an der C7.4 hängt.

**Abnahmetest:** der Satz nennt sein Bezugswort und die Stelle, an der `wartende_marken` fällt, und nicht den Kanalschluss.

**Resolved:** 260831. Der letzte Absatz des Doc-Kommentars von `gitmeldungen_einziehen` (`crates/krk-ui/src/tabs.rs`) nennt sein Bezugswort ausdrücklich und sagt das Gegenteil des alten Satzes: „**Der Kanalschluss nimmt `wartende_marken` nicht mit**", er räume allein `tab.gitlauf` weg. Dazu der Grund, den der Datensatz nennt: in einem großen Ordner schließt der Kanal regelmäßig, bevor der Bestand gelesen ist, und fiele der Befund mit ihm, wäre er für immer weg — genau die Alternative, die das Feld `wartende_marken` als verworfen ausschreibt. Die zwei Stellen, an denen das Feld wirklich fällt, stehen jetzt namentlich da: `Tabliste::gitlauf_nachziehen_an` und `Tabliste::abbrechen`. Die Aussage über den drei tiefen Kanal bleibt, weil sie stimmt und die Überschrift des Absatzes trägt; sie steht jetzt als eigener Satz und teilt sich kein Semikolon mehr mit der falschen Hälfte. Die Mehrdeutigkeit, die der zweite Teil des Befunds nennt, ist damit weg: `tab.gitlauf` und `wartende_marken` stehen beide beim Namen.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23 beim Prüfen des Einzugstakts gegen A8 und C7.4. Der Code ist richtig, die Beschreibung nicht eindeutig.
