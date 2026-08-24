Die Probe zum verschriebenen Schlüssel nennt vier Eingaben und prüft drei

---

`ein_verschriebener_schluessel_nennt_sich_in_der_meldung`
(`crates/krk-core/tests/leseprofil.rs:474-505`) sagt in ihrem Doc-Kommentar: „Diese Probe hält
fest, dass **jede der vier Eingaben** ihren eigenen Schlüssel nennt." Die Schleife darunter
trägt drei Paare.

---

```
crates/krk-core/tests/leseprofil.rs:489   ("  zaehlung = { mustre = 'y' }\n", "mustre"),
crates/krk-core/tests/leseprofil.rs:490   ("  zaehlungg = { }\n", "zaehlungg"),
crates/krk-core/tests/leseprofil.rs:491-494  ("  zaehlung = { }\n  beschreibung = \"zu viel\"\n", "beschreibung"),
```

Drei Eingaben: ein verschriebenes Feld **im** Tisch, ein verschriebener Tischname, ein
zusätzlicher Schlüssel **neben** der Beschriftung.

Die Zahl ist entweder eine, die von einem gestrichenen vierten Fall übrig blieb, oder sie war
von Anfang an falsch. Beide Male ist sie heute die Aussage, an der ein Leser die Abdeckung
abliest, und sie stimmt nicht.

**Ein vierter Fall wäre naheliegend und fehlt tatsächlich:** ein unbekannter Schlüssel auf der
obersten Ebene, also neben `[[profil]]`. Er fällt an `Profildatei`s `deny_unknown_fields`
(`crates/krk-core/src/leseprofil/datei.rs:98`), und der Modulkopf zählt ihn unter den sechs
Stellen mit dieser Angabe ausdrücklich mit. Ob ein vierter Fall dazukommt oder die Zahl auf
drei geht, ist die Entscheidung; die Prosa und die Schleife müssen danach dasselbe sagen.

**Daneben, ohne eigenen Datensatz:** der `vorspann` derselben Probe
(`crates/krk-core/tests/leseprofil.rs:487`) trägt zwischen `\n\n` und `[[profil.zeile]]`
zweiundzwanzig Leerzeichen. TOML kümmert das nicht, gemeint war es offenbar nicht, und beim
nächsten Anfassen der Zeile gehört es weg.

**Schwere:** niedrig. Die drei geprüften Fälle halten, was sie prüfen; falsch ist allein die
Zahl darüber.

**Gefunden:** coderev, bei der Durchsicht der Bündel C, D und E am 260824-1646.

**Betroffen:** `crates/krk-core/tests/leseprofil.rs` (Doc-Kommentar Zeile 482-484, Schleife 488-495)

**Domain:** code

---
Resolved:
