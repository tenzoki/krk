Die Auswahl des Nutzers überlebt das Sortieren am Ende eines Lesevorgangs nicht

---

Die Auswahl im Dateifenster hängt allein an der Zeilennummer der `NSTableView`.
Wenn `Ordnermodell::abschliessen` am Ende des Lesevorgangs die Sichtreihenfolge
sortiert, steht unter derselben Zeilennummer ein anderer Eintrag. Der Nutzer
hatte "zzz.txt" ausgewählt und hat nach dem Sortieren "Applications" ausgewählt,
ohne dass er etwas getan hat.

---

## Wo es passiert

`crates/krk-ui/src/appkit/tabelle.rs:328-339`, der Takt des Zeitgebers:

```rust
fn einziehen(&self) {
    let (angehaengt, fertig) = self.stapel_uebernehmen();
    if fertig {
        self.einzug_beenden();
        *self.ivars().lesevorgang.borrow_mut() = None;
        // Erst jetzt steht die Sortierung. Die bisher angezeigten Zeilen
        // standen in Lesereihenfolge, also muss die Tabelle sie neu holen.
        self.ivars().tabelle.reloadData();
    } else if angehaengt {
```

`stapel_uebernehmen` ruft `modell.abschliessen()` (`tabelle.rs:365`), und
`abschliessen` baut die Sichtreihenfolge von Grund auf neu und sortiert sie
(`crates/krk-core/src/verzeichnis/modell.rs:80-82`, `166-181`). Die
Zeilennummern zeigen danach auf andere Einträge. `einziehen` sichert die Auswahl
vorher nicht und stellt sie nachher nicht wieder her.

`auswahl_verschieben` (`tabelle.rs:273-292`) liest und setzt die Auswahl
ausschließlich über `selectedRow()` und `selectRowIndexes_byExtendingSelection`.
An keiner Stelle in `tabelle.rs` steht ein Aufruf von
`Ordnermodell::eintragsindex` oder `Ordnermodell::zeile_von`.

## Warum das Fenster für den Fehler wirklich offensteht

Es ist kein Randfall, sondern die Zusage L2. Sie verlangt die erste
Bildschirmseite "sichtbar **und bedienbar**", also ausdrücklich bevor die
Sortierung steht. Die Spanne zwischen erstem Stapel und Abschluss ist gemessen:
auf dem Prüfordner mit 100.000 Einträgen erster Stapel nach 35 ms, vollständig
gelesen und sortiert nach 690 ms
(`history/260803-1244-fenster-menue-und-echte-dateiliste.md`, Abschnitt
"Zur Durchblätterprüfung"). Über eine halbe Sekunde lang bewegt sich die Auswahl
über eine Liste in Lesereihenfolge, die gleich umsortiert wird.

Der Ausgang ist in beiden denkbaren Fällen falsch: hält `reloadData` die
Zeilennummer, springt die Auswahl auf einen anderen Eintrag; verwirft sie sie,
verliert der Nutzer seine Auswahl ganz. Für den Befund ist die Unterscheidung
nicht nötig.

## Warum das ein Defekt und keine Auslassung des Zuschnitts ist

Der Kern hält die Lösung bereit und sagt ausdrücklich, wozu:

```rust
/// Der Eintragsindex zur genannten Zeile.
///
/// Die Auswahl haengt an diesem Index und nicht an der Zeilennummer; nur
/// deshalb ueberlebt sie einen Sortierwechsel.
pub fn eintragsindex(&self, zeile: usize) -> Option<u32>
```

(`crates/krk-core/src/verzeichnis/modell.rs:143-149`, dazu `zeile_von` in
`150-156`.)

Der Plan sagt dasselbe in `### Frage 2`: "die Auswahl des Nutzers bleibt über
einen Sortierwechsel hinweg stabil, weil sie am Eintragsindex hängt und nicht an
der Zeilennummer". Beide Funktionen sind seit Schritt 2 da und werden von
niemandem gerufen.

## Warum es jetzt zu beheben ist und nicht in S13

Die Auswahl ist ab hier ein Muster. S12 bringt zwei Dateifenster mit je mehreren
Tabs, S13 die vollständige Tastaturnavigation aus C2, S14 die Bereichsauswahl.
Jeder dieser Schritte setzt auf der Frage auf, wo die Auswahl wohnt. Wenn sie in
der `NSTableView` wohnt, ist sie an dieser Stelle dreimal zu reparieren statt
einmal.

Dieselbe Frage kommt außerdem unabhängig vom Lesevorgang zurück, sobald C2 das
Umschalten der Sortierung über den Spaltenkopf bringt: `sortierung_setzen`
(`modell.rs:90-93`) ruft denselben `sicht_neu_aufbauen`.

## Was zu tun ist

Die Auswahl in der `DateifensterQuelle` als Eintragsindex führen, nicht als
Zeilennummer. Konkret:

- `QuelleIvars` bekommt ein Feld für den ausgewählten Eintragsindex.
- `auswahl_verschieben` rechnet weiter in Zeilennummern (das ist die richtige
  Einheit für eine Bewegung), schreibt am Ende aber `eintragsindex(ziel)` in
  dieses Feld.
- `einziehen` liest das Feld vor `reloadData`, holt danach über `zeile_von` die
  neue Zeilennummer und setzt die Auswahl neu.

Ein Kandidat für eine Prüfung ohne Fenster liegt damit im Kern: `zeile_von` nach
`abschliessen` liefert für denselben Eintragsindex die neue Zeile.

**Aufgefallen bei:** der Prüfung von Schritt 6 und 7,
`circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md`.
