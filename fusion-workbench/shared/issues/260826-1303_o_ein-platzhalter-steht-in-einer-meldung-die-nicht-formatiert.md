# Ein Platzhalter steht in einer Meldung, die nicht formatiert

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Severity:** Low
**Affected:** `crates/krk-core/tests/leseprofil.rs:664-665`
**Tree state:** `4a57028`

---

## Was ist

```rust
// tests/leseprofil.rs:662-666
for wert in ["titelchen", "Datum", ""] {
    let text = format!("{vorspann}  juengste = {{ anzahl = 1, zeigt = \"{wert}\" }}\n");
    let fehler = toml::from_str::<Profildatei>(&text)
        .expect_err("der Wert {wert:?} kommt durch, obwohl es ihn nicht gibt");
```

`Result::expect_err` nimmt eine Zeichenkette und keine Formatvorlage. Der
Platzhalter `{wert:?}` reist wörtlich in die Abbruchmeldung; wer die Probe fallen
sieht, liest `der Wert {wert:?} kommt durch` und erfährt gerade nicht, welcher
der drei Durchgänge es war.

Es ist die einzige Stelle dieser Art in den drei größten Probendateien des Kerns:

```sh
$ grep -n 'expect(".*{.*}\|expect_err(".*{' crates/krk-core/tests/{leseprofil,ablage,verzeichnis}.rs
crates/krk-core/tests/leseprofil.rs:665:            .expect_err("der Wert {wert:?} kommt durch, obwohl es ihn nicht gibt");
```

Die Nachbarn in derselben Schleife machen es richtig: `:668-671` geht über
`assert!` mit Formatvorlage und nennt `{wert:?}` dort mit Wert.

## Was zu tun wäre

```rust
let fehler = toml::from_str::<Profildatei>(&text)
    .unwrap_or_else(|_| panic!("der Wert {wert:?} kommt durch, obwohl es ihn nicht gibt"));
```

Ein `unwrap_or_else` mit `panic!` ist der übliche Weg; er steht in diesem Baum
schon mehrfach so, etwa `tests/verzeichnis.rs:717-718` und
`tests/gemeinsam/mod.rs:291-292`. `clippy` fängt die Lage nicht: die Regel
`literal_string_with_formatting_args` ist in der Vorgabegruppe nicht scharf.

**Gefunden:** coderev, Vollbaum-Durchsicht R5 der drei größten Probendateien des
Kerns.
