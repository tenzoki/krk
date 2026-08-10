Der Verweis in die Gegenrichtung fehlt der Nummernspalte noch
---
`nummernspalte.rs:89-93` sagt, dass der Zugriff auf `layoutManager` AppKit auf den aelteren `NSLayoutManager` zurueckfallen laesst, und nennt als eingekauften Nutzen die voruebergehenden Merkmale der Einfaerbung. Seit `260810-1243` haengt an demselben Rueckfall ein Zweites: der Rueckweg des Rueckgaengig aus der Textflaeche in das `Editormodell`. Der Datensatz verlangt den Verweis in beide Richtungen; die Editorseite steht, die Nummernspaltenseite nicht.
---
**Schwere:** Niedrig
**Gefunden:** bei der Behebung von `260810-1243`
**Betroffen:** `crates/krk-ui/src/appkit/nummernspalte.rs` (nur Modulkopf)
**Zusammenhang:** `issues/260810-1243_*_dass-ein-cmd-z-ueberhaupt-im-modell-ankommt-haengt-an-textkit-1-und-das-steht-nirgends-als-tragend.md`

## Warum es ein eigener Datensatz ist

Die Behebung von `260810-1243` war auf `crates/krk-ui/src/appkit/editor.rs`
begrenzt; `nummernspalte.rs` lag ausserhalb der Dateigrenze der Aufgabe. Der
Verweis ist damit das eine Stueck des Vorschlags, das nicht ausgefuehrt ist.

## Was fehlt

Der Modulkopf von `nummernspalte.rs` fuehrt den Rueckfall heute so:

```text
Der Zugriff auf `layoutManager` laesst AppKit auf den aelteren
`NSLayoutManager` statt auf `NSTextLayoutManager` zurueckfallen. Der
Rueckfall ist von diesem Plan bereits eingekauft: die Einfaerbung der
Formatansicht legt ihre voruebergehenden Merkmale in denselben Verwalter.
```

Was dazugehoert: dass der Editor denselben Rueckfall seit dem 260810-1243
**ausdruecklich** in `textflaeche_bauen` herstellt, weil `textDidChange:` bei
einem `undo` nur auf TextKit 1 feuert, und dass eine Probe
(`appkit::editor::tests::die_gebaute_flaeche_steht_auf_textkit_1`) es haelt. Ein
Verweis auf jene Zeile genuegt; die Messung selbst steht im Modulkopf von
`editor.rs` und soll nicht an zwei Stellen stehen.

## Warum es nicht tragend ist

Vor der Behebung waere es tragend gewesen: der Rueckfall entstand allein als
Nebenwirkung, und wer die Nummernspalte auf `NSTextLayoutManager` nachzog, nahm
ihn mit. Seither stellt `textflaeche_bauen` ihn selbst her, und die Probe faellt
aus, wenn jemand die Zeile entfernt. Ein Nachziehen der Nummernspalte allein
bricht heute nichts.

Es bleibt trotzdem ein Befund: wer die Nummernspalte umbaut, liest ihren
Modulkopf und nicht den des Editors, und ein Verweis dort spart ihm den Umweg
ueber eine fehlgeschlagene Probe.

---
Resolved: Nachgeprueft, dass der Verweis nicht mehr tragend ist, und deshalb
steht im Modulkopf der Nummernspalte jetzt genau das, statt eines Verweises auf
eine Voraussetzung, die keine mehr ist.

**Die Nachpruefung, an drei Stellen im Code:**

- `appkit/editor.rs:3060` — `textflaeche_bauen` fasst `layoutManager` mit einer
  eigenen Zeile an, unmittelbar hinter `setAllowsUndo(true)`, und die 16 Zeilen
  Kommentar darueber nennen den Grund samt Messung.
- `appkit/editor.rs:4629` — `die_gebaute_flaeche_steht_auf_textkit_1` fragt zuerst
  `textLayoutManager().is_none()`, dann `layoutManager().is_some()`, und faellt
  aus, sobald jene Zeile fehlt. Der Datensatz `260810-1243` hat die Gegenprobe
  gefahren.
- `appkit/editor.rs:58-76` — der Modulkopf traegt die Messung und den Hinweis, dass
  sie gemessen und nicht von Apple zugesagt ist.

Der Rueckfall auf TextKit 1 haengt damit an einer Zeile in `textflaeche_bauen` und
nicht mehr an der Nummernspalte. Ein Nachziehen dieser Datei auf
`NSTextLayoutManager` nimmt dem Editor sein Rueckgaengig nicht mehr weg.

**Was eingetragen ist** (`appkit/nummernspalte.rs`, ein Absatz hinter dem
bestehenden zum Rueckfall): dass das Rueckgaengig des Editors bis zum 260810-1243
mit an diesem Zugriff hing, ohne dass eine Zeile es sagte; dass `textDidChange:`
bei einem `undo` nur auf TextKit 1 feuert; dass `textflaeche_bauen` den Rueckfall
seither selbst herstellt und die genannte Probe ihn haelt; und dass beim
Nachziehen dieser Datei allein die Einfaerbung der Formatansicht zu klaeren
bleibt, denn die legt ihre voruebergehenden Merkmale weiter in den aelteren
Verwalter. Die Messung selbst steht nicht hier, sondern bleibt im Modulkopf von
`editor.rs`, wie der Datensatz es verlangt.

**Die Zeile `Betroffen:` nennt den richtigen Pfad**
(`crates/krk-ui/src/appkit/nummernspalte.rs`); die Aufgabenstellung nannte ihn
ohne das `appkit/`, und eine Datei dieses Namens gibt es unter `src/` nicht.

Verification: `cargo build --workspace` exit 0, `cargo test --workspace` exit 0,
`cargo clippy --workspace --all-targets` exit 0,
`cargo fmt -p krk-ui -- --check` exit 0, `cargo fmt -p krk-core -- --check` exit 0.
