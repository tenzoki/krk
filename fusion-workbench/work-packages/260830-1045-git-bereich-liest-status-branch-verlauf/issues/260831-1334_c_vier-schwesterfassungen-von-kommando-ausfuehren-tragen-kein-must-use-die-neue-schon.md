Vier Schwesterfassungen von `kommando_ausfuehren` tragen kein `#[must_use]`, die neue schon

---
Schritt 8 der Runde 23 hat `Gitfenster::kommando_ausfuehren` angelegt und mit der Marke versehen, die dieses Projekt für einen so gearteten Rückgabewert verlangt:

```rust
#[must_use = "ein nicht ausgefuehrtes Kommando laeuft weiter"]
pub fn kommando_ausfuehren(&self, kommando: Kommando) -> bool   // appkit/git.rs:654
```

**Die vier Schwesterfassungen desselben Zuschnitts tragen sie nicht.** Alle vier liefern dasselbe `bool` mit derselben Bedeutung, und keine steht unter der Marke:

- `crates/krk-ui/src/appkit/vorschau.rs:1185`
- `crates/krk-ui/src/appkit/leiste.rs:345`
- `crates/krk-ui/src/appkit/tabelle.rs:1735`
- `crates/krk-ui/src/appkit/anwendung.rs:3373` (der Delegierte, privat)

Das ist nicht bloß Ungleichmaß: **drei Aufrufstellen lassen die Antwort heute nackt fallen**, und der Bau sieht es nicht.

- `appkit/anwendung.rs:1390` — im Melder der Bereichsleiste, `selbst.kommando_ausfuehren(kommando, None);`
- `appkit/anwendung.rs:8272` — `Handlung::Listenanfaenge`, `…quelle().kommando_ausfuehren(Kommando::Listenanfang);`
- `appkit/anwendung.rs:8300` — `Handlung::AlleMarkieren`, `…quelle().kommando_ausfuehren(Kommando::AlleMarkieren);`

CLAUDE.md hält für diese Lage eine Regel und ihre Form: „`let _ =` davor heißt überall dasselbe: „ich brauche den Wert nicht", und ein nackter Aufruf baut nicht mehr". An den drei Stellen baut er.

**Was der Befund nicht ist.** Die Runde 23 hat ihn nicht verursacht; alle vier Fassungen und alle drei Aufrufstellen stehen so schon in `d1fbaac`. Sichtbar wird er, weil die fünfte Fassung dieser Runde die Marke trägt und damit die vier daneben als Ausnahme ausweist. Der Vergleichsfall an derselben Datei fällt anders aus und ist deshalb kein Beleg gegen die Marke: `Tabliste::gitlauf_nachziehen_an` steht ohne Marke, und `Tabliste::durchlauf_nachziehen_an` daneben auch — die zwei privaten Helfer sind untereinander gleich.

**Abnahme:** entweder tragen die vier Fassungen die Marke und die drei Aufrufstellen ein `let _ =`, oder es steht an einer Stelle, warum dieser Rückgabewert die Ausnahme von der Regel ist. `make check` bleibt in beiden Fällen grün; unter `-D warnings` ist `unused_must_use` ein Fehler, deshalb müssen die drei Stellen mit der Marke zusammen fallen.

---
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Domain:** code — `crates/krk-ui/src/appkit/`.
Gefunden in Schritt 16 der Runde 23, bei der Durchsicht der neuen Rückgabewerte nach C8.9. Die Durchsicht selbst geht auf: jeder neue Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, trägt die Marke; die Zahl der Stellen unter `crates/*/src` ist von 139 auf 169 gestiegen.

---
Resolved: Behoben, und am `2fa1d0e` waren von den vier Fassungen schon drei nachgezogen. Nachgemessen: `vorschau.rs:1188`, `leiste.rs:347` und `tabelle.rs:1750` tragen die Marke bereits mit eigener Begruendung; offen war allein `Anwendungsdelegierter::kommando_ausfuehren` (`crates/krk-ui/src/appkit/anwendung.rs`), das sie jetzt mit demselben Text traegt wie `Gitfenster::kommando_ausfuehren`: "ein nicht ausgefuehrtes Kommando laeuft weiter". Von den drei nackten Aufrufstellen waren zwei ebenfalls schon nachgezogen (`Handlung::Listenanfaenge` und `Handlung::AlleMarkieren` im Messmodus tragen `let _ =` mit Begruendung), `:889` ebenso; die verbliebene im Melder der Bereichsleiste traegt jetzt `let _ =` und einen Satz dazu, dass ein Klick auf einen Schalter kein Tastendruck ist, der weiterliefe. `cargo clippy -p krk-ui --all-targets -- -D warnings` steht auf Exit 0, also ist keine Stelle uebersehen. Der Vergleichsfall `gitlauf_nachziehen_an`/`durchlauf_nachziehen_an` ist unberuehrt geblieben, wie der Datensatz es begruendet. Geprueft: cargo test -p krk-ui 908 gruen, clippy/fmt/doc je Exit 0.
