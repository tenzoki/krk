Ein doppeltes Kommentarzeichen in `tabelle.rs` entwertet den Absatz zu `clickedRow`

---

`crates/krk-ui/src/appkit/tabelle.rs:199` beginnt mit `//! //!`. Der Absatz zur Verfuegbarkeit von
`clickedRow` steht damit als **Text** in der Moduldokumentation und nicht als deren Fortsetzung; die
Auszeichnung des ersten Satzes bleibt roh stehen. Entstanden beim Einfuegen der fuenf neuen
Untergrenzen-Angaben der Runde 17 in `a34a3f0`.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die Zeile

```
//! //! **`clickedRow` steht seit 10.0** (`NSTableView.h:276`, am SDK gelesen: die
```

Es ist die einzige Stelle dieser Art im ganzen Baum; nachgezaehlt am 260825 mit
`grep -rn '^\s*//!\s*//!' crates/`.

## Warum es zaehlt

Der Abschnitt "Ab welchem macOS die angesprochenen Klassen stehen" ist in diesem Vorhaben eine
Gewohnheit ohne Werkzeug: `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer
haelt die Untergrenze nicht. Der Abschnitt ist deshalb das einzige, was die Angabe traegt, und eine
Zeile, die ihn optisch zerreisst, trifft genau die Stelle, an der der naechste Leser nachschlaegt.
`cargo fmt --all --check` und `cargo clippy -- -D warnings` sehen es nicht (beide am 260825 hier
gefahren, Exit 0): eine Zeile Doku ist fuer beide gueltiger Text.

## Vorschlag

Das zweite `//!` streichen.

## Umfang

`krk-ui`, `appkit/tabelle.rs`, nur Prosa.

---
Resolved: Das zweite `//!` in `crates/krk-ui/src/appkit/tabelle.rs` ist gestrichen; der Absatz zu
`clickedRow` steht wieder als Fortsetzung der Moduldokumentation, und die Auszeichnung des ersten
Satzes greift. `grep -rn '^\s*//!\s*//!' crates/` gibt danach nichts mehr aus.
