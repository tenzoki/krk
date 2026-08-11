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

---

## Behebung 260811-2155 (Commit `b2a6c2e`)

**Entschieden vom Nutzer am 260811-2140: `#[must_use]`.** Der Übersetzer erzwingt die Behandlung;
die Unterscheidung „nackt heißt, `Unbekannt` kann hier nicht eintreten" wandert vollständig in die
Kommentare — ein nackter Aufruf baut nicht mehr.

**Der Datensatz legt das als offene Abwägung ohne Empfehlung vor. Sie war keine mehr.** Dieses
Projekt hat dieselbe Frage bereits am Defekt `260810-0423` beantwortet: `EditorModell::bearbeiten`
(`crates/krk-ui/src/editormodell.rs:940`) trägt seither ein `#[must_use]`, und sein Doc-Kommentar
schreibt genau diese Abwägung aus, bis hin zu dem Satz, dass ein `let _ =` davor ausdrücklich
sagt, man brauche die Meldung nicht.

**Damit standen zwei entgegengesetzte Bedeutungen von `let _ =` im selben Crate** — beim Editor
„ich brauche den Wert nicht", am `Auswahlversuch` „`Unbekannt` kann eintreten und wird bewusst
verworfen". Diese Unstimmigkeit aufzulösen war der eigentliche Gewinn, und sie fehlt in der
Abwägung des Datensatzes.

**Das Attribut hat sofort eine sechste Aufrufstelle gefunden, die kein Datensatz führte:** eine
Probe in `crates/krk-ui/src/tabs.rs` (`eine_zweite_auffrischung_laesst_den_vorgemerkten_namen_stehen`),
die über `Tabliste::auswahl_auf_namen` geht statt über `eintrag_waehlen`. Der Datensatz zählt nur
die Aufrufer von `eintrag_waehlen`; das Attribut sitzt am **Typ** und trifft damit beide Wege. Die
Probe verwirft den Wert zu Recht und sagt es jetzt. `cargo build` und `cargo test` liefen dabei
grün — `unused_must_use` ist erst unter `-D warnings` ein Fehler.

Die fünf im Reconciliation-Vermerk geführten Aufrufstellen lagen so, wie er sie führt; nur die
Zeilennummern sind abgewandert.

**Ein zweiter Fall derselben Form ist mitgekommen:** `Einzug` (`tabs.rs`, geliefert von
`Tabliste::einziehen`). Fällt der Wert still, bleibt die `NSTableView` mit dem alten Bestand
stehen, während das Modell den neuen führt, und kein zweiter Weg meldet das nach. Er hat dasselbe
Attribut bekommen. Hier fand der Bau nichts — `einziehen` hat genau einen Aufrufer
(`appkit/tabelle.rs:1784`), und der bindet den Wert.

**Nicht angefasst und ausdrücklich als harmloser eingeordnet:** die `-> bool`-Rückgaben von
`Fensteraufteilung::umschalten`, `einblenden` und `aktiv_setzen`. Sie melden bloß, ob sich etwas
geändert hat.

`Verification: make check — exit 0`

---
Abgleichsvermerk 260811-2157 (`reconciler`): **die Behebung traegt in jedem geprueften Punkt.**
`#[must_use]` steht an `Auswahlversuch` (`crates/krk-ui/src/tabs.rs:270`) mit der Begruendung „war
der Versuch Unbekannt, steht der Name nicht in der gelesenen Liste", und der Doc-Kommentar darueber
(`:250`) schreibt aus, dass es Erzwingung und keine Bitte ist. Der zweite Fall ist ebenfalls da:
`Einzug` traegt es bei `tabs.rs:297`. Die Pfadangabe, die der vorige Abgleich als falsch gemeldet
hatte, ist damit auch praktisch berichtigt — das Attribut sitzt in `tabs.rs` und nicht in
`tabelle.rs`, wie die Zeile `**Betroffen:**` behauptet hatte.

`make check` laeuft gruen (Ausgang 0, 795 Proben in 16 Zielen, 0 gescheitert), und `clippy` faehrt
mit `-D warnings`, womit `unused_must_use` ein Fehler ist. Die Erzwingung ist also gemessen und
nicht behauptet.

**Was `CLAUDE.md` dazu weiterhin sagt, und was nicht mehr stimmt:** der Absatz „Am `Auswahlversuch`
unterscheidet die Schreibweise des Aufrufs, und nichts erzwingt sie" (`CLAUDE.md:98`) beschreibt die
Lage vor `b2a6c2e`, samt dem Satz „`Auswahlversuch` traegt kein `#[must_use]`". Er ist seit dem
260811-2155 falsch. `CLAUDE.md` ist in dieser Sitzung nicht nachgezogen worden.
