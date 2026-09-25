Ein Doc-Kommentar in `tests/belegung.rs` hängt an der falschen Funktion

---

S12 hat zwei Hilfsfunktionen in `crates/krk-core/tests/belegung.rs` eingefügt,
`geladene_belegung` und `belegung_sichern`. Sie sind **in** den vorhandenen Doc-Kommentar von
`ablage_mit` hineingeschoben worden, ohne Leerzeile dazwischen. Das Ergebnis
(`crates/krk-core/tests/belegung.rs:26-39`):

```rust
/// Eine Ablage im genannten Pruefordner, mit dem gegebenen Inhalt von
/// `keymap.toml`.
///
/// Eine freie Funktion und keine Methode am [`Pruefordner`]: … Nur diese
/// Datei braucht ihn, also steht er hier.
/// Laedt die Belegung so, wie der Betrieb es tut: unter der Schreibsperre.
///
/// Seit der Runde 7 fuehrt jeder Weg auf die Platte durch einen [`Zugang`] …
fn geladene_belegung(ablage: &Ablage) -> …
```

Der Kommentar über `ablage_mit` beschreibt jetzt `geladene_belegung`, und `ablage_mit`
(`:53-58`) steht ohne Dokumentation da.

---

**Schwere:** gering. Prüfcode, kein Verhalten. `rustdoc` läuft über Integrationsproben nicht,
also fällt es nur beim Lesen auf.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-core/tests/belegung.rs:26-58`

**Domain:** code

## Vorschlag

Die zwei neuen Funktionen samt ihren Kommentaren hinter `ablage_mit` stellen, sodass der alte
Kommentar wieder an seiner Funktion hängt.

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813. `ablage_mit` steht jetzt unmittelbar unter seinem eigenen Doc-Kommentar, und `geladene_belegung` und `belegung_sichern` folgen dahinter, jede mit ihrer eigenen Erklaerung. Der Kommentar von `ablage_mit` ist bei der Gelegenheit um den Absatz gewachsen, den die Behebung von `kein-schreibweg-an-der-sperre-vorbei` verlangt: die Funktion schreibt ihren Anfangsinhalt seither unter der Schreibsperre und holt den Pfad aus dem `Zugang`.
