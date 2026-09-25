# Stapelbudget fuer den Rueckgaengigstapel des Editors

**Status:** Complete
**Agent:** coder
**Datum:** 260810-1341
**Auftrag:** Behebe `issues/260810-1314_o_ein-wiederholtes-sammelersetzen-legt-je-ruf-einen-bereich-in-dateigroesse-in-den-stapel.md`
**Dateigrenze:** ausschliesslich `crates/krk-ui/src/appkit/editor.rs`

## Was getan wurde

Der Rueckgaengigstapel traegt jetzt ein Budget in **Bytes**. Drei Stuecke, alle in
`crates/krk-ui/src/appkit/editor.rs`:

- `STAPELBUDGET` — die Summe, die die angemeldeten Handlungen zusammen halten
  duerfen. Die Zahl ist `krk_core::text::datei::EDITORGRENZE`, also die
  Dateigrenze des Editors aus C2; eine Zusicherung beim Bauen haelt die
  Umrechnung `u64` → `usize` fest.
- `Stapellast` — die Huelle im angemeldeten Block. Sie traegt die Bytes ihres
  Punktes an einem `Rc<Cell<usize>>` an und in ihrem `Drop` ab, damit der Zaehler
  auf jedem der vier Wege stimmt, auf denen ein `NSUndoManager` eine Handlung
  fallen laesst.
- `verlauf_fuer_umbau` — die Regel. Passt der Punkt neben das, was schon im
  Stapel steht, wird er als `Verlauf::Traegt` angemeldet; passt er nicht, geht er
  als `Verlauf::TraegtNurDiese` durch die eine Schreibstelle und steht danach
  allein im Stapel. Sie steht als freie Funktion, damit sie ohne Fenster pruefbar
  ist; die gleichnamige Methode reicht ihr allein den Zaehler herein.

`treffer_ersetzen` und `alle_treffer_ersetzen` gehen durch diese eine Stelle.
`umkehren` **nicht**: ein `cmd+z` nimmt eine Handlung vom einen Stapel und legt
eine von derselben Groesse auf den anderen, die Summe bleibt also stehen, und ein
Budget, das dort zugriffe, koennte einem Nutzer, der `cmd+z` und `shift+cmd+z`
gegeneinander laufen laesst, den Verlauf nehmen.

## Die Messung

`der_stapel_haelt_hoechstens_das_budget_und_die_letzte_handlung`, an der
Editorgrenze und mit den Staenden, die `ctrl+cmd+r` herstellt:

```text
  je Ruf                                   16 777 214 B   gemessen, in allen Rufen gleich
  3 Rufe, ohne Budget (bis 260810-1341)    50 331 642 B
  3 Rufe, mit Budget                       16 777 214 B   zweimal geraeumt
  100 Rufe, ohne Budget                 1 677 721 400 B   100 × die gemessene Zahl
  100 Rufe, mit Budget                     16 777 214 B   unabhaengig von der Zahl der Rufe
```

`ein_gewoehnlicher_umbau_bleibt_neben_dem_verlauf_und_erst_der_volle_stapel_wird_geraeumt`
haelt die andere Richtung fest: ein Punkt von drei Bytes raeumt bei leerem Zaehler
nichts, und ein voller Stapel raeumt auch fuer drei Bytes.

## Drei Wege, ein genommener

Der Datensatz nannte drei, keinen davon empfohlen.

**Weg 2, eine Schranke in Bytes, ist genommen — und seine Ablehnung im Datensatz
war falsch.** Sie verlangt keinen eigenen Stapel neben dem des `NSUndoManager`,
sondern einen Zaehler und den Raeumungsweg, den diese Datei schon hatte. Der
Modulkopf bleibt, wie er ist.

**Weg 1, mehrere Bereiche je Handlung, ist nicht genommen, und der Grund ist eine
Rechnung.** Eine Liste der Stellen kostet je Stelle einen Versatz; an einer Datei
von 16 MB liegt der Umschlag bei einem Treffer je acht Bytes, und darunter ist die
Liste teurer als der eine Bereich. Genau im Fall des Datensatzes — ein haeufiger
Buchstabe — liegt der Abstand darunter. Die Liste loeste den Fall also nicht,
sondern verschoebe ihn.

**Weg 3, nichts tun, ist nicht mehr der Stand.** „Ein Sammelersetzen, das den
ganzen Text aendert, muss den ganzen Text aufheben" ist richtig fuer **ein**
Sammelersetzen. Der Defekt war die Summe ohne Grenze, und die ist begrenzbar, ohne
einem einzigen die Ruecknahme zu nehmen.

## Was nicht angefasst wurde

- Keine Zeile ausserhalb `crates/krk-ui/src/appkit/editor.rs`.
- Die vier Instanzproben mit `MainThreadMarker::new_unchecked` sind unberuehrt,
  und es ist auch keine fuenfte dazugekommen.
- `setLevelsOfUndo` steht weiter nirgends.
- Keine der zehn Zeitzusagen aus C8 der Runde 1 ist beruehrt: hinzugekommen sind
  je Ersetzen eine Addition und ein Vergleich auf `usize`, und keine Zusage liegt
  auf dem Ersetzungsweg.

## Neu gefundener Defekt

`issues/260810-1341_o_die-freigabe-des-angemeldeten-rueckgaengig-blocks-ist-geschlossen-und-nicht-gemessen.md`
— dass der `NSUndoManager` den angemeldeten Block mit der Handlung wieder
freigibt, ist geschlossen und nicht gemessen. Die Messung braeuchte eine fuenfte
Probe mit `MainThreadMarker::new_unchecked`, und darueber steht die offene Frage
`260810-1044`. Die Schranke haengt an der Annahme nicht: traefe sie nicht zu,
hielte der Stapel eine Handlung statt „Budget plus eine".

## Abnahme

```text
cargo build --workspace                     exit 0
cargo test --workspace                      exit 0   753 Proben
cargo clippy --workspace --all-targets      exit 0
cargo fmt -p krk-ui -- --check              exit 0
```
