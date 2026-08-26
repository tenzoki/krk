`Kommando::kennung` vergleicht über `as u8` und lieferte oberhalb von 256 Varianten still die falsche Kennung

---

Der Vergleich in der `const fn` läuft über `kommando as u8 == self as u8`. Die Aufzählung trägt heute 79 Varianten und wächst nach `CLAUDE.md` mit fast jeder Runde. Ab 257 Varianten schneidet die Umwandlung ab, zwei Kommandos vergleichen sich gleich, und die Funktion liefert eine falsche Kennung statt eines Fehlers. Keine Stelle nennt die Schranke.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Am Baum

`crates/krk-core/src/tasten/belegung.rs:1107-1117`:

```rust
pub const fn kennung(self) -> &'static str {
    let mut stelle = 0;
    while stelle < Self::KENNUNGEN.len() {
        let (kommando, kennung) = Self::KENNUNGEN[stelle];
        if kommando as u8 == self as u8 {
            return kennung;
        }
        stelle += 1;
    }
    panic!("jedes Kommando steht in KENNUNGEN")
}
```

Der Grund für die Umwandlung ist gut: `PartialEq` ist keine `const`-Fähigkeit, und die Funktion soll zur Übersetzungszeit auswertbar bleiben. Der Grund für **`u8`** steht nirgends.

## Warum die Zahl 256 hier nicht weit weg ist

`CLAUDE.md` führt für `Kommando` bewusst keine Zahl, weil sie „mit fast jeder Runde wächst und in dieser Datei viermal in vier Tagen falsch geworden ist". Am 260826 sind es 79, nachgezählt mit `awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs`. Die Aufzählung hat also keine gesetzte Obergrenze, und die einzige, die im Code steht, steht als stiller Nebenwirkung eines Umwandlungsoperators.

Wird sie überschritten, gibt es keinen lauten Fall: `KENNUNGEN` ist nach der Längenangabe vollständig, der Übersetzer hält `wirkungsbereich`, die Proben laufen über `KENNUNGEN` — und `kennung()` liefert für die 257. Variante die Kennung der ersten. Damit bekäme ein Kommando im Menü, in der Belegungsansicht und in der Belegungsausgabe den Namen eines anderen.

## Vorschlag

`u16` statt `u8`, und ein `const _: () = assert!(Kommando::KENNUNGEN.len() < 256);` daneben wäre die Alternative, die die Schranke benennt statt sie zu verschieben. Das Projekt fährt diese Form schon zweimal: `datei.rs:201` und `krk-ui/src/appkit/editor.rs:885`. Der billigste Schnitt ist `as u16` mit einer Zeile Kommentar, warum überhaupt umgewandelt wird.

Nicht erreichbar am heutigen Baum; gemeldet, weil die Schranke unbenannt ist und der Fehlfall still wäre.

Gefunden bei der Vollbaum-Durchsicht R4 an HEAD `004ff72`.
