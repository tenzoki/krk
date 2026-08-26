# Die Probe zur tiefen Suche ohne Filtertext legt seit der Deep-Vorgabe nichts mehr um

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Severity:** Medium
**Affected:** `crates/krk-core/tests/verzeichnis.rs:837-855`, `:359-364` (`geladenes_modell`)
**Tree state:** `4a57028`
**Cross-references:** `shared/issues/260826-1221_o_die-tiefe-suche-ab-werk-nimmt-jede-verknuepfung-beim-ersten-anschlag-aus-der-liste.md` (dieselbe Wurzel, zwei andere Aufbauhelfer); `crates/krk-core/src/verzeichnis/modell.rs:374` (`tief: true`), `:996-998` (`tief_setzen`)

---

## Was ist

Der Datensatz `260826-1221` nennt zwei Aufbauhelfer von `tests/verzeichnis.rs`,
die den Zustand **vor** der Vorgabenänderung herstellen und deshalb an der neuen
Vorgabe vorbeimessen: `gefiltert` (`:708-713`) und `handmodell` (`:1220-1229`),
beide mit `tief_setzen(false)`.

Es gibt einen dritten Aufbauhelfer, `geladenes_modell` (`:359-364`), und er setzt
den Schalter **gar nicht**. Damit trägt jedes über ihn gebaute Modell seit
`20c9833` die neue Vorgabe `tief: true` (`modell.rs:374`). Eine Probe, die den
Schalter danach auf `true` setzt, legt nichts mehr um:

```rust
// tests/verzeichnis.rs:837-855, gekürzt
let mut modell = geladenes_modell(ordner.pfad());       // tief == true, ab Werk
let vorher: Vec<String> = namen(&modell)…;              // schon unter tief == true
…
modell.tief_setzen(true);                               // Nulloperation
assert!(modell.tief(), "das Kennzeichen steht, auch ohne Filtertext");
assert_eq!(namen(&modell), vorher, "…");
```

Die zweitletzte Zeile liest den Wert zurück, den `Ordnermodell::neu` gesetzt hat,
und nicht die Wirkung des Setzers dazwischen. Der Vergleich `namen == vorher` ist
danach ein Vergleich zweier Aufnahmen desselben unveränderten Standes.

## Was die Probe messen wollte

Ihr Doc-Kommentar (`:827-835`) sagt es: C2.4, „steht kein Filtertext, ändert
‚Deep‘ nichts an der Liste", und ausdrücklich „**auch** mit einem Befund
`KeinTreffer` an einem Ordner — ohne Filtertext wird er gar nicht erst gefragt".
Der Übergang aus → ein ist der Gegenstand. Er findet nicht mehr statt.

Der gesetzte Befund (`:846`) bleibt dabei erhalten, weil
`Ordnermodell::schalter_setzen` eine Antwort nur verwirft, wenn sich die Frage
ändert — nachgeprüft an `ein_befund_gilt_nur_zu_seiner_frage`
(`:1150-1157`). Auch von dieser Seite bewegt sich also nichts.

## Was zu tun wäre

Ein Zeichen: `modell.tief_setzen(false)` vor die Aufnahme von `vorher` setzen,
dann wie bisher auf `true`. Die Probe misst dann wieder einen Übergang, und ihre
Zusicherung `assert!(modell.tief())` hält wieder den Setzer statt der Vorgabe.

Ob `geladenes_modell` daneben, wie seine zwei Geschwister, die Vorgabe
ausdrücklich stellen sollte, ist die allgemeinere Frage; sie gehört zu
`260826-1221` und wird hier nicht beantwortet. Die drei Proben, die den Helfer
mit stehendem Filtertext benutzen, sind `:1011`, `:1034` und `:1094`; die ersten
zwei sind von der Vorgabe unberührt, die dritte trägt einen eigenen Datensatz.

**Gefunden:** coderev, Vollbaum-Durchsicht R5, auf die Suche nach demselben
Muster geschickt, das `260826-1221` beschreibt.
