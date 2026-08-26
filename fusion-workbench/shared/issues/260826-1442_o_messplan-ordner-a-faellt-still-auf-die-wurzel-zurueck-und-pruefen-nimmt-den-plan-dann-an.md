`Messplan::ordner_a` fällt still auf die Wurzel zurück, und `pruefen` nimmt den Plan dann an
---
Hat das linke Dateifenster des Messplans keinen aktiven Tab, liefert `ordner_a()` den Pfad `/`. `pruefen` fragt danach nur, ob `/` ein Verzeichnis ist und auf demselben Gerät liegt wie das Kopierziel; beides stimmt für ein Kopierziel auf der Systemplatte. Die Sitzungsstrecke liefe dann mit L1, L7, L6 und L8 über die Wurzel und meldete Zahlen über den falschen Ordner.
---
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>

## Am Baum

`crates/krk-ui/src/messmodus.rs:240-245`:

```rust
pub fn ordner_a(&self) -> &Path {
    self.sitzung.fenster[0]
        .aktiver_tab()
        .map(|tab| tab.ordner.as_path())
        .unwrap_or_else(|| Path::new("/"))
}
```

`:255-293` (`pruefen`) hält `/` für gültig. Dieselbe Bauart an `:812-817` (`RechtsZeigt(... .unwrap_or_default())`, also der leere Pfad) und `:797-806` (`eltern` → `/`, `unterordner_name` → `""`).

Heute schreibt allein `krk-bench` den Plan, und dessen Prüfsitzung trägt zwei Tabs je Fenster. Der Fall trifft einen von Hand geschriebenen oder beim Bearbeiten gekürzten Plan. Der Doc-Kommentar zu `aus_argumenten` (`:155-159`) will, dass ein Fehler in der Planangabe „als ‚Messplan nicht lesbar‘ auffällt statt still“; der Rückfall auf `/` widerspricht dem.

## Vorschlag

`ordner_a` liefert `Result<&Path, String>` oder `pruefen` weist eine Sitzung ohne aktiven Tab in einem der zwei Fenster ab, mit Meldung.
