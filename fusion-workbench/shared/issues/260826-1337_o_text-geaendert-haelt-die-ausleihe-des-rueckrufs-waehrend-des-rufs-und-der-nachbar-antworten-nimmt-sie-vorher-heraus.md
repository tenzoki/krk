`text_geaendert` hält die Ausleihe des Rückrufs während des Rufs, und der Nachbar `antworten` nimmt sie vorher heraus

---

`Eingabewaechter::text_geaendert` (`crates/krk-ui/src/appkit/blaetter/mod.rs:245-250`) ruft
den hinterlegten Rückruf **innerhalb** der `RefCell`-Ausleihe:

```rust
let aenderung = self.ivars().aenderung.borrow();
if let Some(aenderung) = aenderung.as_ref() {
    aenderung();
}
```

Fünfundzwanzig Zeilen darunter tut `antworten` (`:274-281`) das Gegenteil und sagt warum:
„Die Ausleihe endet vor dem Aufruf: der Antwortweg schliesst das Blatt, und AppKit kann dabei
erneut hierher zurueckrufen." Für `text_geaendert` gilt derselbe Grund: der eine Rückruf, der
heute hinterlegt wird, ist `Vorschauquelle::neu_rechnen` (`stapelumbenennen.rs:430-434`), und
der schreibt über `setStringValue` in die Hinweiszeile und `reloadData` in die Tabelle nach
AppKit hinein. Riefe AppKit dabei `controlTextDidChange:` erneut, oder setzte ein Rückruf über
`textaenderung_melden` (`:735-739`, ein `borrow_mut` auf dieselbe Zelle) einen neuen, bräche
die Probe mit „already borrowed" ab.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/appkit/blaetter/mod.rs`

Heute hält es, weil `neu_rechnen` keine der beiden Bewegungen macht und die Hinweiszeile kein
bewachtes Feld ist. Das ist eine Invariante, die nichts hält — kein Kommentar, keine Probe —,
und die Datei daneben hat für denselben Fall schon einmal die andere Form gewählt.

Denkbarer Weg: dieselbe Form wie `antworten` — den Rückruf klonen oder die Ausleihe in einen
eigenen Block legen, bevor gerufen wird —, und ein Satz dazu, warum.
