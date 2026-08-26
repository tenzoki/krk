Zwei Verzweigungen ueber Art tragen einen Auffangzweig und halten den Bau nicht an

---

`Auftrag::neuer_name` und `Auftrag::entpackziel` sind die einzigen zwei Stellen im ganzen Baum,
die ueber `Art` mit `_ => None` verzweigen. Eine siebte Operationsart, die wie diese beiden eine
zweite Angabe Stelle fuer Stelle zu den Quellen fuehrt, uebersetzt damit anstandslos und liefert
still `None` — und der Rufer meldet je Eintrag "es fehlt der neue Name". Genau diese Falle
schliesst `CLAUDE.md` fuer jede andere Verzweigung ueber `Art` ausdruecklich.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

`crates/krk-core/src/operation/auftrag.rs:178-191`:

```rust
pub(crate) fn neuer_name(&self, stelle: usize) -> Option<&str> {
    match &self.art {
        Art::UmbenennenImStapel { neue_namen } => neue_namen.get(stelle).map(String::as_str),
        _ => None,
    }
}

pub(crate) fn entpackziel(&self, stelle: usize) -> Option<&Path> {
    match &self.art {
        Art::Entpacken { ziele } => ziele.get(stelle).map(PathBuf::as_path),
        _ => None,
    }
}
```

## Dass es die einzigen zwei sind, ist nachgezaehlt

`grep -rn "Art::" crates/ xtask/ --include='*.rs'` gegen `grep -rn "_ => "` in beiden Modulen des
Umfangs. Jede andere Verzweigung ueber `Art` ist vollstaendig und ohne Auffangzweig:

- `operation/mod.rs:189-196` — `ausfuehren`
- `operation/mod.rs:240-283` — `einen_abarbeiten`, mit einem eigens erlaeuterten toten Zweig
  `Art::Zippen` (`mod.rs:274-282`), der ausdruecklich dasteht, damit die Unterscheidung
  vollstaendig bleibt
- `operation/auftrag.rs:226-234` — `zielordner`, mit einem Doc-Kommentar, der jede der vier
  `None`-Arten einzeln begruendet
- `krk-ui/src/auffrischung.rs:332-341` — `schiebt_auffrischung_auf`
- `krk-ui/src/kommandos/operationen.rs:433-438` und `:484-489`
- `krk-ui/src/appkit/anwendung.rs:522-532` und `:6605-6647`

Die zwei mit Auffangzweig sind die zwei, die eine **Angabe je Stelle** liefern — also gerade die,
bei denen ein stilles `None` je Eintrag eine Zeile in der Abschlussliste erzeugt statt eines
Uebersetzungsfehlers.

## Was der stille Ausfall kostet

`einen_abarbeiten` (`mod.rs:254-273`) behandelt den `None`-Fall bewusst und meldet ihn:
"es fehlt der neue Name" beziehungsweise "es fehlt der Zielordner". Der eigene Kommentar dort
begruendet das damit, dass die beiden Listen aus denselben Paaren entstehen und gleich lang sind
— was fuer die heutigen zwei Arten stimmt. Fuer eine achte Art stimmte es nicht mehr: sie liefe
durch, meldete je Eintrag einen erfundenen Grund und truege den Fehler in die Abschlussliste
statt in die Fehlerliste des Uebersetzers.

## Was zu tun waere

Beide Auffangzweige durch die Aufzaehlung der uebrigen fuenf Arten ersetzen, so wie
`zielordner` es zwei Bildschirmseiten tiefer schon tut. Das ist je Funktion eine Zeile mehr und
kostet nichts.

## Umfang

`krk-core`, `operation/auftrag.rs`.
