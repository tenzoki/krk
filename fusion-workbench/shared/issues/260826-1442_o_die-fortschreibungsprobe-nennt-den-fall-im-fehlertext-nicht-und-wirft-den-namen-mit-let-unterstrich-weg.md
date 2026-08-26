Die Fortschreibungsprobe nennt den Fall im Fehlertext nicht und wirft den Namen mit `let _ =` weg
---
`ein_fortgeschriebener_durchgang_gleicht_dem_vollen` führt vierzehn benannte Fälle, reicht aber den Dateinamen `"a.rs"` als `name` an den Helfer und den Fallnamen in `let _ = name;`. Schlägt einer der 28 Läufe fehl, sagt die Meldung „a.rs“ und den ganzen Quelltext, nicht „ein geöffneter Blockkommentar“.
---
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>

## Am Baum

`crates/krk-ui/src/hervorhebung.rs:1891-1896`:

```rust
for (name, nachher) in faelle {
    fortschreiben_gleicht_vollem_durchgang(quelle, &nachher, "a.rs", Dateityp::Sonstiges);
    fortschreiben_gleicht_vollem_durchgang(&nachher, quelle, "a.rs", Dateityp::Sonstiges);
    let _ = name;
}
```

Der Helfer `:1810-1842` formatiert `{name}` als Dateinamen. Das `let _ =` steht hier nicht für „ich brauche den Wert nicht“ im Sinn der `#[must_use]`-Regel aus `CLAUDE.md`, sondern unterdrückt die Warnung über eine unbenutzte Bindung.

## Vorschlag

Dem Helfer einen fünften Parameter `fall: &str` geben und ihn in die drei Meldungen schreiben; das `let _ =` fällt damit weg.
