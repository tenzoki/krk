Der Messmodus schreibt die Sitzung ohne Sitzungsrecht

---

C3.9 sagt zu: „**Die Sitzung schreibt genau die Instanz, die beim Start das Sitzungsrecht
bekommen hat.**" Im Betrieb steht die Regel an einem fehlenden Wert und nicht an einer
Abfrage: `sitzung_laden` baut den `Sitzungsschreiber` nur, wenn das Recht gehalten wird
(`crates/krk-ui/src/appkit/anwendung.rs:1233-1236`), und der Doc-Kommentar von
`Sitzungsschreiber` sagt es ausdrücklich (`crates/krk-core/src/ablage/sitzung.rs:424-428`).

`Messplan::herstellen` umgeht das: es öffnet eine eigene `Ablage`, baut einen
`Sitzungsschreiber::neu()` ohne jede Frage nach dem Sitzungsrecht und schreibt
`session.toml` (`crates/krk-ui/src/messmodus.rs:300-325`). Läuft daneben eine gewöhnliche
Instanz, überschreibt der Messlauf deren Sitzung, und die Halterin des Rechts merkt es nicht.

**Unbedenklich ist der Fall an der Datei.** Der Schreibvorgang läuft seit dieser Runde unter
der Schreibsperre, und der Kommentar an der Stelle begründet das richtig: ein Messlauf ist
kein Sonderfall des Schreibwegs. Es geht allein um C3.9.

---

**Schwere:** gering. Der Messlauf ist Nutzerarbeit im Vordergrund und läuft nicht neben dem
gewöhnlichen Betrieb; die Zusage C3.9 gilt trotzdem nicht ausnahmslos, und das steht nirgends.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/messmodus.rs:300-325`,
`crates/krk-core/src/ablage/sitzung.rs:424-428`

**Domain:** code

## Vorschlag

Der billigste Weg ist ein Satz und kein Bau: den Doc-Kommentar von `Sitzungsschreiber` um die
eine Ausnahme ergänzen, damit „er entsteht nur, wenn dieser Prozess das Sitzungsrecht hält"
nicht als ausnahmslos gelesen wird, und den Grund dazuschreiben — der Messlauf stellt eine
Prüfsitzung her und ist der einzige Schreiber, der sie **setzen** und nicht fortschreiben
soll. Wer es strenger will, nimmt das Recht auch dort und bricht ab, wenn es fehlt; das ist
für einen Messlauf die richtige Antwort, weil eine Zahl auf fremder Lage keine Zahl ist.

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813, und zwar auf dem strengeren der beiden vorgeschlagenen Wege — nicht mit einem Satz, sondern am Typ.

**`Sitzungsschreiber::neu` und `::mit_takt` verlangen jetzt ein `&Sitzungsrecht` und liefern `Option<Self>`** (`crates/krk-core/src/ablage/sitzung.rs`). Ohne gehaltenes Recht gibt es keinen Schreiber, und das ist keine Abmachung mehr, an der jemand vorbeilaufen kann: der Uebersetzer verlangt das Argument. Genau daran war der Messmodus vorbeigelaufen. Der Doc-Kommentar sagt die Regel jetzt so, wie sie gilt, und nennt diesen Datensatz als Anlass.

**`Messplan::herstellen` nimmt das Recht und bricht ohne es ab** (`crates/krk-ui/src/messmodus.rs`). Die Meldung nennt die Lage und den Ausweg: eine andere Instanz haelt das Recht und schreibt `session.toml`, der Messlauf stellt die Pruefsitzung nicht her, solange sie laeuft. Das ist die im Datensatz genannte richtige Antwort — eine Zahl auf fremder Lage waere keine. Das Recht wird fuer die Dauer des Schreibens gehalten und danach abgegeben; die Reihenfolge Recht-dann-Schreibgriff ist dieselbe wie beim gewoehnlichen Start, eine Verklemmung entsteht also nicht.

**Zwei weitere Stellen sind nachgezogen.** Der Modulkopf von `crates/krk-core/src/ablage/sperre.rs` fuehrt jetzt beide Nehmer des Rechts und haelt fest, dass ein Prozess nie zwei zugleich haelt. Der Doc-Kommentar am Feld `sitzungsrecht` des Anwendungsdelegierten sagt nicht mehr, die Regel stehe an einem fehlenden Wert, sondern nennt den Uebersetzer.

**Eine Probe haelt es:** `ohne_sitzungsrecht_entsteht_kein_sitzungsschreiber` (`crates/krk-core/tests/ablage.rs`) prueft alle drei Faelle — der erste Halter bekommt einen Schreiber, der zweite keinen, und ein Recht, das niemand haelt, auch keinen.
