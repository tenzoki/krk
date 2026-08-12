Eine Ablagedatei mit ungültigem UTF-8 wird nicht zur Seite gelegt, obwohl sie Inhalt trägt

---

Schritt 1 legt allein im Zweig `Grund::Beschaedigt` zur Seite. Die Begründung
dafür, dass `Grund::NichtLesbar` es nicht tut, steht dreimal im Baum und ist in
einem Fall falsch:

- `crates/krk-core/src/ablage/mod.rs`, Modulkopf: „Von einer, die sich nicht
  lesen liess, gibt es keinen Inhalt."
- `crates/krk-core/src/ablage/mod.rs`, `Beiseite::Nicht`: „von einer Datei, die
  sich nicht lesen liess, gibt es keinen Inhalt zu sichern."
- Kriterium C3.5 des Plans vom 260812-1145, wörtlich derselbe Satz.

`Ablage::laden` (`crates/krk-core/src/ablage/mod.rs:323-340`) liest mit
`fs::read_to_string`. Diese Funktion scheitert nicht nur an einem
Zugriffsfehler, sondern auch mit `io::ErrorKind::InvalidData`, wenn die Bytes
kein gültiges UTF-8 sind. Beides fällt in denselben Zweig:

```rust
Err(fehler) => {
    return Geladen {
        wert: T::default(),
        ersetzung: Some(Ersetzung {
            datei: pfad,
            grund: Grund::NichtLesbar(fehler.to_string()),
            beiseite: Beiseite::Nicht,
        }),
    };
}
```

Bei `InvalidData` steht die Datei da, ist vollständig, trägt die Arbeit des
Nutzers und wird trotzdem nicht gesichert. Danach greift genau der Schaden, den
die Runde abstellen will: KRK arbeitet auf dem Auslieferungszustand weiter und
schreibt ihn beim nächsten gewöhnlichen Schreibvorgang darüber. Für
`bookmarks.toml` und `session.toml` ist das jedes Beenden.

---

**Wie eine Ablagedatei dazu kommt**

Der Weg ist nicht hypothetisch, und er trifft genau die beiden Dateien, um
derentwillen die Regel „kopieren statt verschieben" überhaupt besteht:
`keymap.toml` und `settings.toml` sind von Hand änderbar, und das sagt der
Modulkopf seit Schritt 10 der Runde 1 selbst. Ein Editor, der beim Sichern auf
Latin-1 oder auf eine andere Einbyte-Kodierung fällt, macht aus einem Umlaut in
einem Lesezeichennamen oder in einem Kommentar eine Bytefolge, die kein UTF-8
ist. KRK schreibt so etwas nie selbst; der Nutzer kann es.

**Was zu tun ist**

Die Prüfung „gibt es einen Inhalt zu sichern" gehört an das, was sie behauptet:
an das Vorhandensein von Bytes, nicht an das Gelingen einer
Zeichenkettenumwandlung. Zwei Wege stehen offen, und beide sind größer als eine
Zeile, weshalb sie hier nicht gegangen werden:

1. `Ablage::laden` liest mit `fs::read` statt `fs::read_to_string` und wandelt
   danach selbst um. Der `InvalidData`-Fall bekommt damit die Bytes und kann
   sie sichern — nur nimmt `atomar::schreiben` heute ein `&str`, also bräuchte
   der Weg eine Byte-Fassung daneben, und ein zweiter Schreibweg ist durch den
   Datensatz vom 260812-1105 ausgeschlossen.
2. `Grund` bekommt einen vierten Wert für „steht da, ist aber kein UTF-8", und
   nur dieser Zweig sichert. Das ändert eine vollständige Fallunterscheidung
   des Kerns und hält den Bau an mehreren Stellen an.

**Kontext**

- Das ist der **dritte** Weg an der Sicherung vorbei. Der zweite steht als
  `issues/260812-1204_o_eine-semantisch-widerspruechliche-keymap-toml-wird-nicht-zur-seite-gelegt.md`
  und liegt eine Ebene höher, in `belegung::laden`. Dieser hier liegt eine
  Ebene tiefer, im Lesen selbst. Beide teilen die Ursache: die Sicherung hängt
  an einem einzigen Zweig von `Ablage::laden`, und beide Nachbarzweige tragen
  eine Begründung, die für ihren Regelfall stimmt und für einen Sonderfall
  nicht.
- Der Datensatz
  `decisions/260812-1000_a_wie-heisst-die-zur-seite-gelegte-ablagedatei-und-was-geschieht-beim-zweiten-mal.md`
  bindet weiter: ein fester Name, eine Sicherung, die nicht überschrieben wird.
- Die Probe `eine_fehlende_und_eine_nicht_lesbare_datei_werden_nicht_zur_seite_gelegt`
  (`crates/krk-core/tests/ablage.rs`) prüft den Zweig mit einem **Ordner** an
  der Stelle der Datei. Das ist der Fall, für den die Begründung stimmt, und
  deshalb hat die Probe den anderen nicht gefunden. Eine Behebung braucht eine
  zweite Probe mit einer Datei aus ungültigen Bytes, etwa `b"name = \"\xe4\"\n"`.
- Gefunden bei der Durchsicht von Turn 1 der Runde 6; nicht behoben.
