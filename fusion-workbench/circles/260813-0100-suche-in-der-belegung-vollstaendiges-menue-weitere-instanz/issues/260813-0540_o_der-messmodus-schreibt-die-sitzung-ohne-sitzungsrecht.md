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
