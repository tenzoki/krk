`Fenstermodell::lesereihenfolge` liefert eine zweite Stufe, die ihr einziger Rufer verwirft

---

`Fenstermodell::lesereihenfolge` (`crates/krk-ui/src/fenstermodell.rs:957-972`) baut zwei
Stufen: erst den sichtbaren Tab jedes sichtbaren Dateifensters, „danach alles uebrige" — jede
weitere Tabstelle beider Seiten. Der eine Rufer, `Anwendungsdelegierter::lesevorgaenge_starten`
(`crates/krk-ui/src/appkit/anwendung.rs:2879-2890`), filtert die Liste sofort wieder auf
`stelle == uebersicht[seite.index()].sichtbar` und ruft für jeden Treffer
`sichtbaren_lesen()`. Die zweite Stufe erreicht damit keinen Aufruf; sie lebt wirklich in
`Tabliste::nachzuegler_starten` (`tabs.rs:792-799`), angestossen vom Einzugstakt
(`appkit/tabelle.rs:3414-3415`), und das steht so auch im Doc-Kommentar des Rufers.

Übrig bleibt eine Funktion, deren Rückgabewert zu mehr als der Hälfte tot ist, ein Feld
`Tabuebersicht::zahl` (`tabs.rs:382-387`), das ausserhalb dieser toten Hälfte niemand liest, und
zwei Proben (`fenstermodell.rs:2628-2662`), die eine Reihenfolge mit vier Einträgen messen, von
der das Programm zwei benutzt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/fenstermodell.rs` (`lesereihenfolge`), `crates/krk-ui/src/tabs.rs`
(`Tabuebersicht`), `crates/krk-ui/src/appkit/anwendung.rs` (`lesevorgaenge_starten`)
**Baumstand:** `ca8072d`

## Warum es der Übersetzer nicht meldet

Der Wert wird gelesen, nur nicht für das, was er trägt: die Schleife über `reihenfolge` läuft
über alle Einträge und lässt die der zweiten Stufe am `if` vorbei. Für `unused` ist das eine
Verwendung.

## Was die Probe misst

`die_lesereihenfolge_nimmt_die_sichtbaren_tabs_zuerst` (`:2628`) nennt sich nach der Prüfsitzung
aus C8 und vergleicht vier Einträge. Die Aussage, an der L4 hängt, ist allein die Reihenfolge der
ersten beiden; der Rest der Zusicherung misst Code, den kein Weg erreicht.

## Weg

`lesereihenfolge` auf die erste Stufe kürzen und `Vec<Fensterseite>` liefern — welches
Dateifenster sichtbar ist, weiss allein dieses Modell, welcher Tab dort sichtbar ist, weiss die
`Tabliste` ohnehin. `Tabuebersicht` fällt dann samt `zahl`, und `lesevorgaenge_starten` braucht
das `if` nicht mehr. Die zwei Proben schrumpfen auf die Aussage, die sie tragen.
