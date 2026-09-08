Die neue `atomar`-Probe behauptet eine Vollstaendigkeit, die Rust nicht hergibt

---

`nur_benannte_dateien_erreichen_das_atomare_schreiben` (`crates/krk-core/tests/baum.rs:177-206`)
ist in Turn 2 als der eine Fall gebaut worden, in dem die Vollständigkeit erreichbar ist. Ihr
Doc-Kommentar sagt es in einem Satz (`:164-170`):

> Es gibt in Rust genau zwei Wege an eine fremde Funktion: den Pfad an der Aufrufstelle oder
> ein `use`, das sie in den Geltungsbereich holt. Beide nennen das Modul, also enthaelt jede
> Datei, die `schreiben` ueberhaupt erreichen kann, eine der drei Zeichenketten
> `atomar::schreiben`, `atomar::{` oder `atomar::*`. Ein anderer Weg besteht nicht.

**Der Schluss von „beide nennen das Modul" auf „also steht dort `atomar::`" hält nicht.** Ein
`use` darf das Modul umbenennen, und dann steht sein Name nirgends in der geforderten Form.
Zwei Gegenbeispiele, beide mit `rustc --edition 2024` übersetzt:

```rust
// Weg 3: das Modul unter anderem Namen einbinden.
use krk_core::ablage::atomar as werkzeug;
werkzeug::schreiben(&pfad, &text)?;      // keine der drei Zeichenketten in dieser Datei
```

```rust
// Weg 4: eine Wiederausfuhr, die das Modul gar nicht nennt.
// in crates/krk-core/src/ablage/mod.rs — steht bereits auf der Liste der fuenf:
pub use atomar::schreiben;
// in jeder anderen Datei:
krk_core::ablage::schreiben(&pfad, &text)?;
```

Beide Male ist `atomar::schreiben` erreicht, und die Probe zählt die Datei nicht mit. Weg 4
ist der unangenehmere: die eine Zeile, die ihn öffnet, steht in einer Datei, die auf der
Liste bereits erlaubt ist, und macht die Probe damit für den ganzen Baum blind, ohne sie rot
werden zu lassen.

**Das ist genau der Fehlschluss, den derselbe Commit abstellen wollte.** Der Kopf von
`crates/krk-ui/src/quellbaum.rs:56-71` zieht drei Folgerungen, und die dritte lautet: die
verbleibende Blindheit am Doc-Kommentar benennen. Benannt ist an dieser Probe allein der über
zwei Zeilen umbrochene Pfad (`tests/baum.rs:172-176`). Die zwei Wege oben stehen nirgends, und
davor steht der Satz „Ein anderer Weg besteht nicht", der das Nachsehen erübrigen soll.

---

**Schwere:** mittel. Die Probe läuft heute richtig grün, und keiner der zwei Wege liegt im
Baum. Der Schaden liegt in der Zusage: wer den Satz liest, hält die Lücke für dicht und prüft
sie nicht nach.

**Gefunden:** coderev, Durchsicht von `a34bf17..dff167a` am 260813-0715

**Betroffen:** `crates/krk-core/tests/baum.rs:164-176`

**Domain:** code

## Vorschlag

Die Nadelliste um `atomar as` erweitern — das fängt Weg 3 und kostet eine Zeile. Für Weg 4
gibt es keine Nadel, denn die Wiederausfuhr macht den Namen `atomar` überflüssig; er gehört
als benannte Blindheit an den Doc-Kommentar, zusammen mit dem Satz, dass eine `pub use` auf
`atomar::schreiben` in einer der fünf erlaubten Dateien die Probe für den ganzen Baum blind
macht.

Der Satz „Ein anderer Weg besteht nicht" gehört gestrichen. Was stimmt, ist schwächer und
trägt trotzdem: die drei Zeichenketten decken jede Schreibweise ab, die dieser Baum heute
kennt, und zwei weitere sind möglich und benannt.

---

## Abgleich 260908, und die Behebung

**Der Befund besteht am heutigen Baum unveraendert.** Der Satz „Ein anderer Weg besteht
nicht" stand bis heute am Doc-Kommentar von
`nur_benannte_dateien_erreichen_das_atomare_schreiben`, die Nadelliste fuehrte drei
Zeichenketten, und Weg 4 war nirgends benannt.

Gebaut ist der Vorschlag, beide Haelften:

- **Weg 3** faengt jetzt eine vierte Nadel, `atomar as`. Sie steht in derselben
  `concat!`-Form wie die drei anderen, damit die Probe sich nicht selbst zaehlt.
- **Weg 4** hat einen eigenen Abschnitt am Doc-Kommentar, „Der eine Weg, den keine Nadel
  sehen kann". Er schreibt aus, dass eine `pub use atomar::schreiben;` in einer der ohnehin
  erlaubten Dateien die Probe fuer den **ganzen Baum** blind macht, ohne sie rot werden zu
  lassen, und dass dagegen keine Nadel hilft.
- Der Satz „Ein anderer Weg besteht nicht" ist gestrichen. An seiner Stelle steht die
  schwaechere und tragende Aussage: die Nadeln fangen jede Datei, die `schreiben` **unter
  diesem Namen** erreicht.

Resolved: 260908, `crates/krk-core/tests/baum.rs` — vierte Nadel `atomar as`, der Abschnitt
zur Wiederausfuhr als benannte Blindheit, der Vollstaendigkeitssatz gestrichen.
