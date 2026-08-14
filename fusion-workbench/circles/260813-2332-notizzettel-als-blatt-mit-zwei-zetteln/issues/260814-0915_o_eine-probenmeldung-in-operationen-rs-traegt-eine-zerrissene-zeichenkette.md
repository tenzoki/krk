Eine Probenmeldung in operationen.rs trägt vierzehn Leerzeichen mitten im Satz

---

`waehrend_eines_blattes_bleibt_es_bei_dem_einen_abbruch`
(`crates/krk-ui/src/kommandos/operationen.rs:1301`) trägt in seiner zweiten Zusicherung:

```rust
assert!(
    !waehrend_blatt_erlaubt(Kommando::Notizzettel),
    "der Notizzettelbefehl steht in der Ausnahme; der Zettel schliesst              mit esc und nicht mit der Taste, mit der er kommt"
);
```

Zwischen „schliesst" und „mit esc" stehen vierzehn Leerzeichen. Es sieht aus wie eine
Zeichenkette, die über zwei Zeilen umgebrochen war und beim Zusammenziehen die Einrückung
mitgenommen hat.

---

**Schwere:** niedrig. Kein Bau, kein Verhalten; die Zeile ist über 100 Zeichen lang und
`cargo fmt` bricht Zeichenketten nicht um, deshalb ist sie durch `make check` gelaufen.

**Was zu tun ist.** Die Zeichenkette in der Form der Nachbarn schreiben, also mit `\` am
Zeilenende:

```rust
"der Notizzettelbefehl steht in der Ausnahme; der Zettel schliesst mit esc und \
 nicht mit der Taste, mit der er kommt"
```

**Kontext**

- Gefunden bei der Durchsicht von Turn 1, `reviews/260814-0908-coderev-turn-1-notizzettel.md`.
- Der Satz erscheint nur im Fehlschlagsfall dieser Probe, also genau dann, wenn jemand
  `waehrend_blatt_erlaubt` um den Notizzettel erweitert hat — der Augenblick, in dem er
  lesbar sein soll.
