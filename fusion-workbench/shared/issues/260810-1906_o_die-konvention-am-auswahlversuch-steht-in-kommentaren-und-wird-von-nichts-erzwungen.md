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

---

## Reconciliation-Vermerk 260810-1907 (Abschluss-Abgleich der Sitzung 260810-1647)

**Der Befund trägt in jedem geprüften Punkt.** `Auswahlversuch` trägt kein `#[must_use]`
(`grep must_use` liefert in beiden beteiligten Dateien null Treffer). Der Doc-Kommentar von
`eintrag_waehlen` nennt die Konvention nicht. Die fünf Aufrufstellen verteilen sich so, wie
der Datensatz sie führt: auswertend `tabelle.rs:1057` (`eintrag_anspringen`) und
`anwendung.rs:4274` (`messhandlung`, Zweig `Handlung::Auswaehlen`), begründet verworfen
`anwendung.rs:2709` und `:3230`, nackt `anwendung.rs:2733`. Auch die Feststellung, dass
`#[must_use]` und die Konvention „nackt heißt kann nicht eintreten" einander ausschließen,
hält.

**Eine Pfadangabe trifft nicht.** Die Zeile `**Betroffen:**` nennt
`crates/krk-ui/src/appkit/tabelle.rs (Auswahlversuch, eintrag_waehlen)`. Dort steht allein
`eintrag_waehlen`. Die Aufzählung `Auswahlversuch` ist in `crates/krk-ui/src/tabs.rs:249`
deklariert, ihr Doc-Kommentar beginnt bei `:239`. Der vorgeschlagene Weg — `#[must_use]` an
`Auswahlversuch` — gehörte damit nach `crates/krk-ui/src/tabs.rs:248`, nicht nach
`tabelle.rs`. Wer den Datensatz anfasst, fasst zwei Dateien an und nicht eine.

Nebenbei: die Zeilenangabe `tabelle.rs:1062-1074` für den Doc-Kommentar von
`eintrag_waehlen` ist um eine Zeile abgewandert; er steht heute in `:1063-1075`, die Funktion
in `:1076`.

Nachgetragen von `reconciler`, `shared/history/260810-1907-reconciliation.md`.
