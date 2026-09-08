messung_unmoeglich fängt eine neue Messgröße still auf, statt den Bau anzuhalten
---
`messung_unmoeglich` (`crates/krk-ui/src/messmodus.rs`) endet auf `_ => None`. Eine neue Variante von `Sitzungsgroesse` bekommt damit stillschweigend „keine Vorbedingung“ zugeteilt, obwohl jede der vier ausgeschriebenen Größen eine hat. Der Übersetzer sagt nichts, keine Probe sagt etwas, und die Strecke misst im Fehlerfall eine Zahl, statt ohne Zahl abzubrechen — genau das, was der Doc-Kommentar der Funktion zusichert.
---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

Gefunden beim Einbau des Ordnersprungs am 260907: `Sitzungsgroesse::L7Ordner` ist hinzugekommen, und der Bau lief grün durch, ohne die Frage nach der Vorbedingung zu stellen. Die Antwort war in diesem Fall dieselbe wie bei L1 und L7 (eine leere Liste kann keine Auswahl bewegen), und sie ist von Hand nachgetragen worden; gefragt hat niemand.

Das steht gegen die Bauregel dieses Projekts, die CLAUDE.md unter „Etliche Fallunterscheidungen sind vollständig und haben keinen Auffangzweig“ ausschreibt und die `sitzungsmessung_fertig` in derselben Datei einhält: dort hält der Übersetzer die neue Variante an und erzwingt eine bewusste Einordnung. Zwei Fallunterscheidungen über dieselbe Aufzählung, zwei Meter — und der eine misst genau die Frage, deren stille Antwort einen falschen Messwert erzeugt.

Der Auffangzweig deckt heute `L5Tab`, `L5Fenster` und `L6`. Eine vollständige Fassung schreibt diese drei mit `=> None` aus und begründet in einer Zeile, warum keine von ihnen eine Vorbedingung hat.

**Abnahme:** `messung_unmoeglich` trägt keinen `_`-Zweig mehr, und eine neu hinzugefügte Variante von `Sitzungsgroesse` lässt `cargo build -p krk-ui` fehlschlagen.

---
Resolved: Behoben. `messung_unmoeglich` (`crates/krk-ui/src/messmodus.rs`) traegt keinen `_`-Zweig mehr: die drei Groessen ohne Vorbedingung, `L5Tab`, `L5Fenster` und `L6`, stehen ausgeschrieben auf `=> None`, und ein Kommentar daneben sagt in einem Satz, warum keine von ihnen eine hat (sie messen einen Wechsel und keine Bewegung innerhalb eines Bestands). Die Abnahme des Datensatzes ist damit erfuellt: eine neue Variante von `Sitzungsgroesse` haelt `cargo build -p krk-ui` an, wie `sitzungsmessung_fertig` in derselben Datei es schon tat. Geprueft: cargo test -p krk-ui Exit 0.
