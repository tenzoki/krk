Zwei Zählangaben zu `Inhalt` in `vorschaumodell.rs` sind seit der Runde 16 um eins falsch

---

`Inhalt` (`crates/krk-ui/src/vorschaumodell.rs:231-299`) trägt seit der Runde 16 **sieben**
Werte: `Leer`, `Text`, `Markdown`, `Bild`, `Metadaten`, `Zusammenfassung`, `Hinweis`. Zwei
Stellen derselben Datei zählen noch die sechs von davor:

- `zeigt_dateitext`, Doc-Kommentar `:552-555`: „ein **siebter** Inhalt haelt den Bau an und
  erzwingt die Antwort" — der siebte steht schon in der Fallunterscheidung darunter (`:564`).
  Gemeint ist der achte.
- Der Probenhelfer `tab_setzen`, `:1162-1169`: „erreichen zusammen nicht alle **sechs** Werte von
  [`Inhalt`]" — die Probe darunter (`:1191-1246`) fährt sieben.

Beides sind Kommentare, kein Verhalten; der Fall ist derselbe wie bei den Zahlen, die
`CLAUDE.md` aus genau diesem Grund nicht mehr führt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/vorschaumodell.rs:552-555`, `:1162-1169`
**Baumstand:** `ca8072d`

## Weg

„ein siebter" → „ein achter", „alle sechs Werte" → „alle sieben Werte"; oder beide Sätze ohne
Zahl, wie `Bereich::seite` in `fenstermodell.rs:157-160` es mit „ein sechster Bereich" ebenfalls
riskiert — dort stimmt die Zahl heute noch.

**Resolved:** 260828, Runde 20 Schritt 5 (Coder). Beide Sätze stehen ohne Zahl: `zeigt_dateitext` sagt „ein weiterer Wert von `Inhalt` haelt den Bau an", `tab_setzen` sagt „erreichen zusammen nicht jeden Wert von `Inhalt`". Der achte Wert `Inhalt::Pdf` ist im selben Schritt hinzugekommen, und keine der zwei Stellen trägt seither eine Zahl, die mit dem neunten falsch würde (Constraint 7 des Specs der Runde 20).
