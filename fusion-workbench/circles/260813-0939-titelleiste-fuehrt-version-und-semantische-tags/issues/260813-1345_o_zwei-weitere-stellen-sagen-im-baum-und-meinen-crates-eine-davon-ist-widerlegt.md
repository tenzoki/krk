Zwei weitere Stellen sagen „im Baum" und meinen `crates/`, und eine davon ist widerlegt

---

Der Datensatz `260813-1258_o_die-versionszahlprobe-sagt-baum-und-liest-nur-crates.md` nennt eine
Stelle: den Doc-Kommentar von `die_versionszahl_steht_in_keiner_quelldatei`
(`crates/krk-ui/src/appkit/titelzusatz.rs:298-311`). Dieselbe Verwechslung steht zweimal weiter
oben in derselben Datei, und in einem der beiden Fälle ist die Aussage nicht bloß zu weit
gefasst, sondern falsch.

---

**Schwere:** niedrig. Kein Verhalten, keine Probe wird rot.

**1. `titelzusatz.rs:130` behauptet mehr, als der Baum trägt:**

```
/// **Die einzige Stelle im Baum, die Name und Version zusammensetzt.**
```

Vier Stellen in `crates/krk-bench/` setzen einen Namen mit derselben Version zusammen:
`bericht.rs:109-111` und `:286-288` sowie `messen.rs:1862-1864` schreiben `"krk-bench {}"`,
`fixture.rs:542` gibt `version = env!("CARGO_PKG_VERSION")` in eine Tafel. Sie liegen **unter
`crates/`** und damit im Blickfeld von `quellbaum::quelldateien()`; die Probe findet sie nur
deshalb nicht, weil ihre Nadel `concat!("\"KRK \", env!(", …)` auf den Anwendungsnamen
festgelegt ist (`:266-268`). Gemeint ist „die einzige Stelle, die **KRKs** Namen mit der Version
zusammensetzt", und genau das verlangt C5.4 auch. Dagesteht die weitere Aussage.

**2. `titelzusatz.rs:239` und der Probenname bei `:266`** tragen dieselbe Formulierung: „Genau
eine Stelle im Baum setzt Namen und Version zusammen (C5.4)" und
`nur_eine_stelle_im_baum_setzt_namen_und_version_zusammen`. Hier ist die Reichweite `crates/`
und nicht der Baum, genau wie beim schon erfassten Fall.

**Eine fünfte wörtliche Fundstelle der Zahl ausserhalb `crates/`**, die der erfasste Datensatz
nicht führt: `xtask/src/bundle.rs:587`, `let vorlage = "<string>0.1.0</string>";` im Prüfstoff
von `ohne_platzhalter_bricht_die_ersetzung_ab`. Der Datensatz zählt allein die Konstanten in
`release.rs` auf. Wer den Doc-Kommentar dort berichtigt und dabei sagen will, wo die Zahl
ausserhalb `crates/` steht, nennt diese Stelle mit.

**Was zu tun ist**

Beide Formulierungen auf ihre wirkliche Reichweite bringen, in einem Zug mit dem schon
erfassten Fall — es ist dieselbe Datei und dieselbe Ursache. Punkt 1 heisst danach „die einzige
Stelle im Baum, die **KRKs** Namen und die Version zusammensetzt", Punkt 2 „unter `crates/`".
Die Proben selbst bleiben, wie sie sind; sie messen richtig.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Muttersatz: `260813-1258_o_die-versionszahlprobe-sagt-baum-und-liest-nur-crates.md`, offen.
- Dieselbe Verwechslung von „im Baum" mit „in dieser einen Kiste" ist am 260813-0540 schon
  einmal als Defekt aufgetreten und hat `quelldateien` von `krk-ui/src` auf `crates/` gezogen.
