Drei Prosastellen in tabelle.rs zaehlen fuenf Raenge und eine Quelle ohne Feld; die Zeile hat sechs und zwei

---

Seit der Runde 10 hat die Statuszeile sechs Raenge (`statuszeile::Quellen`, sechs Felder) und zwei
gerechnete ohne eigenes Feld (Filterstand, Markierungsstand). Drei Doc-Kommentare in `tabelle.rs` zaehlen
noch den Stand davor; ein vierter in derselben Datei zaehlt richtig.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/appkit/tabelle.rs:788-789`: "wenn eine der **fuenf** Meldungsquellen dieses
  Dateifensters sich geaendert hat" — `:3126` sagt fuer denselben Rueckruf "eine der **sechs** Quellen".
- `:857`: "Der oberste der **fuenf** Raenge".
- `:3222-3223`: "**Die einzige Quelle der Zeile ohne eigenes Feld**, und das ist der Entwurf" — `:3178`
  sagt "Die Eingaben der **beiden** Raenge ohne eigenes Feld", und `gerechnete_raenge` (`:3196-3217`)
  rechnet beide.
- `statuszeile.rs:274-286`: `Quellen` mit Rang 1 bis Rang 6.

## Umfang

`krk-ui`, `appkit/tabelle.rs`, Doc-Kommentare.

---
Abgleich 260829-1252, am Baum `b9d9cbc`: **gilt weiter.** `crates/krk-ui/src/appkit/tabelle.rs:864` sagt weiter „der oberste der fuenf Raenge"; `Rang::ALLE` trägt seit der Runde 20 sieben Werte (`statuszeile.rs:275`), also ist der Abstand gewachsen. Die Stelle `:3227` („eine der sechs Quellen") stimmt weiter, weil `Quellen` unverändert sechs Felder hat.
