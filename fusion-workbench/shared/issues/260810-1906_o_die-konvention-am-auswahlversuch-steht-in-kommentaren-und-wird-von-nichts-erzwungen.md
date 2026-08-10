Die Konvention am Auswahlversuch steht in Kommentaren und wird von nichts erzwungen

---

Turn 2 der Sitzung 260810-1647 hat eine Konvention eingeführt: ein nacktes
`eintrag_waehlen(…)` heißt „`Auswahlversuch::Unbekannt` kann hier nicht eintreten", ein
begründetes `let _ = …` heißt „kann eintreten und wird bewusst verworfen". Sie hält heute an
allen fünf Aufrufstellen. Sie steht aber nur in zwei Aufrufkommentaren und in einem
Defektdatensatz, **nicht** im Doc-Kommentar von `eintrag_waehlen` selbst
(`crates/krk-ui/src/appkit/tabelle.rs:1062-1074`), und `Auswahlversuch` trägt kein
`#[must_use]`. Ein sechster Aufrufer bricht sie unbemerkt.

---

**Schwere:** Niedrig
**Gefunden:** coderev, bei der Durchsicht von Turn 2
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs` (`Auswahlversuch`, `eintrag_waehlen`)
**Domain:** code

## Warum das mehr ist als Kosmetik

Dieses Projekt setzt bewusst auf Vollständigkeit, die der Übersetzer erzwingt: `CLAUDE.md` hält
unter „Was man nicht sieht" fest, dass etliche Fallunterscheidungen keinen Auffangzweig haben,
damit eine neue Variante den Bau anhält. Die Konvention am `Auswahlversuch` ist der genaue
Gegenfall — eine Regel, die allein davon lebt, dass der nächste Leser die richtigen zwei
Kommentare findet.

Der Defekt, den Turn 1 und Turn 2 dieser Sitzung abgearbeitet haben, ist genau daraus entstanden:
drei Aufrufer warfen den Rückgabewert weg, und niemand merkte es, bis jemand danach suchte. Die
Konvention beschreibt jetzt, was gilt; erzwungen wird sie so wenig wie vorher.

## Denkbarer Weg

`#[must_use]` an `Auswahlversuch`. Damit meldet der Übersetzer jeden Aufruf, dessen Wert
stillschweigend fällt, und ein `let _ =` wird zur ausdrücklichen Abweisung statt zur Gewohnheit.
Alle fünf heutigen Aufrufstellen behandeln den Wert bereits: zwei werten ihn aus
(`tabelle.rs:1057`, `anwendung.rs:4274`), zwei verwerfen ihn begründet (`anwendung.rs:2709`,
`:3233`), einer nimmt ihn nackt (`anwendung.rs:2733`). Der nackte müsste dann ein `let _ =` mit
seiner bestehenden Begründung werden — womit die Konvention allerdings ihre Aussagekraft
verliert, weil dann jede Stelle gleich aussieht.

**Das ist die eigentliche Frage und deshalb steht hier keine Empfehlung:** `#[must_use]` und die
Konvention „nackt heißt kann nicht eintreten" schließen einander aus. Entweder der Übersetzer
erzwingt die Behandlung und die Unterscheidung wandert vollständig in den Kommentar, oder die
Unterscheidung bleibt im Programmtext sichtbar und wird von nichts erzwungen. Ein Absatz im
Doc-Kommentar von `eintrag_waehlen` wäre der kleinste Schritt und schlösse die Lücke nicht,
sondern machte sie nur auffindbar.

## Dringlichkeit

Gering. Heute ist nichts falsch; es geht um die sechste Aufrufstelle, die es noch nicht gibt.
