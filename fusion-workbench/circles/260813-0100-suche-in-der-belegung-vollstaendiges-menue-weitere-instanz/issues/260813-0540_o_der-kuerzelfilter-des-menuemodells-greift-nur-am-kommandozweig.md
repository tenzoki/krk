Der Kürzelfilter des Menümodells greift nur am Kommandozweig

---

`menuemodell::eintrag` gibt einer Funktion mit Kommando ihr Kürzel nur dann, wenn kein
Zusteller es beansprucht (`crates/krk-ui/src/menuemodell.rs:243-250`):

```rust
Some(kommando) => Eintrag::Befehl {
    kombination: kombination.filter(|k| !zugestellt.contains(k)),
    …
},
```

Der dritte Zweig derselben Fallunterscheidung, eine benannte Funktion **ohne** Kommando und
ohne Zusteller, bekommt seine Kombination ungefiltert (`:256-261`). Trüge eine solche Funktion
dieselbe Kombination wie ein Zusteller, stünden zwei Einträge mit demselben Kürzel in der
Leiste, und AppKit nähme sie dem später stehenden still weg — genau der Fall, gegen den
`zugestellte_kuerzel` gebaut ist.

**Heute ist der Fall unerreichbar und abgesichert.** Die Auslieferungsbelegung führt keine
unbelegte Funktion mehr; `ein_eintrag_zeigt_die_erste_kombination_oder_keine`
(`crates/krk-ui/src/menuemodell.rs:544-603`) stellt das ausdrücklich fest, und
`keine_zwei_eintraege_tragen_dieselbe_kombination` (`:665-701`) würde den Fall in jedem Fall
rot melden. Der Befund ist eine Ungleichbehandlung im Code, kein Fehlverhalten.

---

**Schwere:** gering. Zwei Zweige derselben Fallunterscheidung behandeln dieselbe Frage
verschieden, ohne dass ein Satz sagt, warum.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/menuemodell.rs:236-262`

**Domain:** code

## Vorschlag

Den Filter einmal vor die Fallunterscheidung ziehen, damit er für beide `Eintrag::Befehl`-Zweige
gilt:

```rust
let kombination = funktion.tasten().first().copied();
let eigenes = kombination.filter(|k| !zugestellt.contains(k));
```

Der Textbefehlszweig behält dabei die ungefilterte Kombination, denn er **ist** der Zusteller.
Das ist eine Zeile weniger als heute und schließt den Zweig mit ein.
