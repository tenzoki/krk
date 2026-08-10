Planordner in messmodus ist die dreizehnte Fassung und kann jetzt auf die gemeinsame aufsetzen

---

`Planordner` in `crates/krk-ui/src/messmodus.rs:1685` legt denselben
selbstabraeumenden Ordner an wie die zwoelf Fassungen aus dem Defekt
`260810-1330`: Prozesskennung und Laufnummer im Namen, `remove_dir_all` in
`Drop`. Er stand nicht in der Aufzaehlung jenes Datensatzes und blieb bei dessen
Behebung deshalb unangetastet. Seit dem 260810 gibt es in derselben Kiste
`crate::pruefordner::Pruefordner`, auf dem er aufsetzen koennte.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Behebung des Defekts `260810-1330`
**Betroffen:** `crates/krk-ui/src/messmodus.rs`
**Domain:** code
**Zusammenhang:** `issues/260810-1330_*_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`

## Was er ausser dem Ordner noch tut

`Planordner` ist keine reine Kopie: er legt im Wurzelordner die vier
Unterordner `a`, `b`, `a-l6` und `ziel` an und liefert daraus mit `plan()` einen
`Messplan`. Das ist Fachlogik der Messstrecke und gehoert nicht in den
gemeinsamen Pruefordner. Zusammenlegbar ist nur der Ordner darunter.

Die Form, die dabei herauskommt:

```rust
struct Planordner {
    wurzel: Pruefordner,
}

impl Planordner {
    fn neu(zweck: &str) -> Self {
        let wurzel = Pruefordner::neu(zweck);
        for unter in ["a", "b", "a-l6", "ziel"] {
            wurzel.ordner(unter);
        }
        Self { wurzel }
    }
}
```

Damit faellt der eigene `AtomicU64`, der eigene `Drop` und der eigene
Namensbau weg. Der gemeinsame Pruefordner braucht dafuer ein `ordner(name)`,
das er heute noch nicht hat; die Fassung in `krk-core/tests/gemeinsam/mod.rs`
fuehrt es, die in `krk-ui/src/pruefordner.rs` nicht, weil bisher keine Probe in
`krk-ui` einen Unterordner ueber den Pruefordner angelegt hat.

## Fehlszenario

Kein Fehlverhalten zur Laufzeit; das ist der Grund fuer die niedrige Schwere.
Was es kostet, ist dasselbe wie bei `260810-1330`: wer eine Eigenschaft der
Bauform aendert, etwa den Ort unter dem Temporaerverzeichnis, aendert sie in
`krk-ui` jetzt an zwei Stellen statt an einer und uebersieht die zweite.

## Warum nicht mit 260810-1330 behoben

Die Dateigrenze jener Aufgabe nannte die zwoelf Dateien des Datensatzes, und
`messmodus.rs` war keine davon. Der Datensatz hatte den Planordner nicht
gezaehlt, obwohl der Modulkopf des `leistenmodell` ihn seit dem 260807
ausdruecklich als Verwandte nannte.

## Zustaendigkeit

`coder`.
