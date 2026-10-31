Singlestrip klebt nach einer `\`-Fortsetzung mit folgender Leerzeile zwei Wörter ohne Leerzeichen zusammen
---
Endet eine Zeile ohne Leerraum auf `\` und folgt eine Leerzeile, setzt Singlestrip zwischen die Wörter kein Leerzeichen: aus `foo\` + Leerzeile + `bar` wird `foobar`. Die Leerzeile ist ein Umbruch und muss ein Leerzeichen ergeben, wie es der Kommentar an der Stelle selbst annimmt.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Domain:** code
**Schwere:** Low
**Cross-references:** `261031-1344-reviewer-durchsicht-singlestrip-und-format-korrektur.md`

**Wo:** `crates/krk-core/src/text/glaetten.rs`, `befehl`. Der Zweig für eine Leerzeile außerhalb überspringt sie mit dem Kommentar „der Trenner davor steht schon“. Nach `Trenner::Fortsetzung` steht aber nur dann ein Leerzeichen, wenn vor dem `\` oder am Anfang der Folgezeile Leerraum stand. `nachher_leer` liest dafür die Leerzeile und nicht die Zeile danach.

**Gemessen** am Stand `780d6870`, `glaetten.rs` per `#[path]` in einem Wegwerfprogramm:

```
"foo\\\n\n  bar\n"   -> "foobar"     (erwartet "foo bar")
"foo \\\n\n  bar\n"  -> "foo bar"
"ls\n\n  -la\n"      -> "ls -la"
```

Für die Shell beendet die Leerzeile den Befehl, `foo` und `bar` sind also zwei Wörter. Unter der Annahme „ein Befehl“ muss deshalb ein Leerzeichen dazwischen stehen. `foobar` ist ein drittes Wort, das niemand geschrieben hat.

**Abnahme:**
- `befehl("foo\\\n\n  bar")` ergibt `foo bar`.
- Eine Probe in `crates/krk-core/tests/glaetten.rs` hält das fest.
- `singlestrip_entfernt_backslash_fortsetzungen_ohne_die_bedeutung_zu_aendern` und `singlestrip_ist_idempotent` bleiben grün.

---
Resolved: 261031, fixed in `29af436a`: eine Leerzeile nach einer Fortsetzung setzt ein Leerzeichen.
