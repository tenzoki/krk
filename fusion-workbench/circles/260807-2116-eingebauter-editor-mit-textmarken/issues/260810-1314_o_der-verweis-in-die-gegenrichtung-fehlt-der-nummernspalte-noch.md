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
