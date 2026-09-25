Die Belegungsprüfung nimmt Cmd+Q als Beispiel für eine unbelegte Kombination

---

`crates/krk-core/tests/belegung.rs:626` prüft, dass eine unbelegte Kombination **mit** Zusatztaste nicht auf die Sprungmarke durchfällt, und wählt dafür `cmd+q` als Beispiel:

```rust
#[test]
fn eine_unbelegte_kombination_mit_zusatztaste_faellt_nicht_auf_die_sprungmarke() {
    // Die Sprungmarke tippt Anfangsbuchstaben. Cmd+Q ist kein Anfangsbuchstabe,
    // sondern ein Kuerzel des Systems, und muss weitergehen duerfen.
    let belegung = Belegung::auslieferung();
    assert_eq!(
        belegung.nachschlag(kombi("cmd+q").tastendruck()),
        Nachschlag::Unbelegt
    );
}
```

Seit dem 260805-0820 trägt `resources/default-keymap.toml` die Funktion `beenden` auf `cmd+q`, und damit ist das Beispiel keines mehr. Der Lauf meldet:

```
$ cargo test -p krk-core --test belegung
---- eine_unbelegte_kombination_mit_zusatztaste_faellt_nicht_auf_die_sprungmarke stdout ----
  left: Funktion(Funktion { kennung: "beenden", ... })
 right: Unbelegt
test result: FAILED. 31 passed; 1 failed
```

Alle übrigen 31 Prüfungen dieser Datei und alle drei anderen Testprogramme des Arbeitsbereichs bleiben grün; `cargo test --workspace` meldet genau diesen einen Fehlschlag.

---

## Warum es zählt

Der Eintrag `beenden` ist der sechste Menükürzel-Eintrag, den C3 verlangt, und der Defekt `260805-0753_c_cmd-q-loest-etwas-aus-und-steht-in-keiner-tastenliste.md` verlangt ihn ausdrücklich auf `cmd+q`. Die Kombination ist damit belegt und bleibt es. Der Testlauf ist seit S13c grün und ist es seit dieser Änderung nicht mehr.

## Was die Prüfung zusagt, und was ihr fehlt

Die Zusage ist unberührt: eine Kombination mit Zusatztaste, die keiner Funktion gehört, fällt nicht auf die Sprungmarke aus C2 durch, sondern gilt als unbelegt und geht weiter. Was fehlt, ist allein ein Beispiel, das ab Werk frei ist. `cmd+q` war eines und ist keines mehr.

## Was zu tun ist

Eine ab Werk freie Kombination mit Zusatztaste an die Stelle setzen und den Kommentar darüber mitziehen, der heute Cmd+Q begründet. Frei sind unter anderem `shift+cmd+q`, `opt+cmd+q` und `ctrl+j`; nachgesehen am 260805-0820 am vollständigen Eintrag über alle 63 ausgelieferten Kombinationen.

`opt+cmd+q` ist als Beispiel ungeeignet: dort liegt die Zweitform "Quit and Keep Windows", die `260805-0753_o_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md` behandelt, und ein Beispiel, das genau eine Runde später wieder umzieht, ist keines.

Die Prüfung ist Code und gehört dem `coder`; `resources/default-keymap.toml` gehört dem `ontocoder`. Zusammen mit `260805-0753_o_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md` zu behandeln, weil beide denselben Menüeintrag betreffen und derselbe Schritt sie erreicht.

---

Herkunft: gefunden beim Nachtragen des Eintrags `beenden` in `resources/default-keymap.toml` am 260805-0820, beim Abnahmelauf `cargo test -p krk-core --test belegung`.

---
Resolved: Die Prüfung nennt kein Beispiel mehr. `crates/krk-core/tests/belegung.rs` trägt sie jetzt als `keine_unbelegte_kombination_mit_zusatztaste_faellt_auf_die_sprungmarke`: sie sammelt jede Kombination, die eine Funktion der Auslieferungsbelegung führt, geht über alle 61 Tasten der Tabelle mal die 15 nicht leeren Zusatztastenmasken und prüft die Zusage an jeder Kombination, die dabei frei bleibt.

**Warum kein Ersatzbeispiel.** Der Defekt nennt `shift+cmd+q` und `ctrl+j` als geprüft frei. Beide wären dieselbe Falle eine Runde später. Beim letzten Mal, bei `cmd+arrowleft`, ging die Absicherung über den Namen: `arrowleft` steht nicht in der Tastentabelle und darf nie hinein, weil die Schreibweise die Taste `left` nennt, und damit kann die Belegung die Kombination gar nicht führen. Hier greift das nicht, jede Kombination der Tabelle darf belegt werden. Die Absicherung geht deshalb über die Herkunft des Beispiels statt über seine Wahl: die Prüfung liest die freien Kombinationen aus der Belegung, statt eine hinzuschreiben. Ein Nachtrag in `resources/default-keymap.toml` nimmt ihr einen Fall und lässt die übrigen stehen.

Die Masken sind aus `ModMaske::BENANNT` gerechnet und nicht aufgezählt, damit eine fünfte Zusatztaste die Liste nicht still unvollständig macht. Eine Schlusszeile `assert!(geprueft > 0, …)` fängt den Fall ab, dass die Auslieferungsbelegung eines Tages jede Kombination mit Zusatztaste vergibt: dann fällt die Prüfung mit dieser Begründung aus, statt still nichts mehr zu prüfen.

Geprüft am 260805-0841: `cargo test -p krk-core --test belegung` meldet 32 Prüfungen, davon 0 gescheitert. Die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` enden alle mit 0.
