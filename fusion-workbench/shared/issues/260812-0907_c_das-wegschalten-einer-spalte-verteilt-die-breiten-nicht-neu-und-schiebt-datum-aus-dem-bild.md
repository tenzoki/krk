Das Wegschalten einer Spalte verteilt die Breiten nicht neu und schiebt Datum aus dem Bild

---

Der Nutzer hat am 260812-0900 gemeldet: schaltet er über die Bereichsleiste die Spalten Größe
und Typ weg, bekommt die Namensspalte zu viel Platz, und die Spalte Datum steht trotz des frei
gewordenen Raums außerhalb des sichtbaren Bereichs. Der gewonnene Platz wird also nicht auf die
bleibenden Spalten verteilt, sondern von einer allein aufgenommen.

---

**Schwere:** mittel (sichtbare Fehlbedienbarkeit einer Fähigkeit, die gerade erst gebaut wurde;
kein Datenverlust)
**Gefunden:** Nutzer, am laufenden Bündel
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs`
**Domain:** code
**Cross-references:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/_b_circle.md` (Runde 5,
beschränkt abgeschlossen — die Spaltenschalter stammen von dort),
`circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260812-0306_i_gelten-die-spaltenschalter-fuer-beide-dateifenster-gemeinsam.md`

## Was am Baum steht, gelesen am 260812-0907

**Die Tabelle steht auf `FirstColumnOnlyAutoresizingStyle`** (`tabelle.rs:2237`). In dieser
Betriebsart darf beim Ändern der Gesamtbreite **allein die erste Spalte** wachsen oder
schrumpfen; alle übrigen behalten ihre Punktzahl. Die erste Spalte ist `Spalte::Name`.

**Die vier Spalten tragen feste Anfangs- und Mindestbreiten** (`tabelle.rs:227`):

| Spalte | Anfangsbreite | Mindestbreite |
|---|---|---|
| Name | 240 | 100 |
| Größe | 80 | 60 |
| Geändert | 130 | 100 |
| Typ | 90 | 60 |

Gesetzt werden sie in `spaltenkopf` (`tabelle.rs:2394-2398`) über `setWidth` und `setMinWidth`.
**Eine Höchstbreite setzt niemand**, also ist die Namensspalte nach oben unbeschränkt.

**Das Wegschalten setzt allein `setHidden`** (`spaltenanzeige_nachziehen`, aus der Runde 5). Es
fasst keine Breite an. Was danach geschieht, entscheidet AppKit über die Betriebsart oben.

## Die Vermutung, ausdrücklich als solche benannt

`inference:` Unter `FirstColumnOnlyAutoresizingStyle` nimmt die Namensspalte den frei werdenden
Platz vollständig auf, statt dass die Summe der sichtbaren Spalten schrumpft. Bleibt die
Gesamtbreite der Tabelle dabei größer als die sichtbare Fläche des Bildlaufs, steht die letzte
sichtbare Spalte außerhalb — das wäre der zweite Teil der Meldung. Gemessen ist keines von
beidem; das gehört an den Anfang der Behebung.

## Was zu entscheiden ist, bevor gebaut wird

„Optimal genutzt" ist noch keine Regel. Drei Kandidaten, die der Behebende gegeneinander
abwägen muss:

1. **Die festen Spalten behalten ihre Anfangsbreite, Name bekommt den Rest** — nach unten
   begrenzt durch seine Mindestbreite von 100. Das ist die Absicht hinter
   `FirstColumnOnlyAutoresizingStyle` und verlangt nur, dass beim Wegschalten die Breiten der
   sichtbaren Spalten auf ihre Anfangswerte zurückgesetzt und die Gesamtbreite auf die sichtbare
   Fläche gebracht wird.
2. **Alle sichtbaren Spalten wachsen anteilig**, wie es die Bereiche der Fensterzeile seit der
   Runde 5 tun. Das wäre dieselbe Regel an zwei Orten und damit leicht zu erklären — kostet aber
   eine zweite Verhältnisrechnung neben `bereichsbreiten`, und die Spalten tragen anders als die
   Bereiche keine gespeicherten Wünsche.
3. **Nur die zuletzt sichtbare Spalte nimmt den Rest auf** (`LastColumnOnlyAutoresizingStyle`).
   Billig, aber der Rest landet dann je nach Schalterstellung bei einer anderen Spalte, und bei
   ausgeschalteten Größe, Datum und Typ wieder bei Name.

Eine gezogene Spaltenbreite überlebt heute keinen Neustart — `Sitzung` führt kein Feld dafür.
Wer eine Regel wählt, die gezogene Breiten achten soll, stößt darauf; wer die Anfangsbreiten
wiederherstellt, nicht.

## Messung am 260812-0930, vor der Behebung

Gemessen an einer `NSTableView` ohne Fenster, mit denselben vier Breitenpaaren und derselben
Betriebsart wie im Baum (kopfloses Programm gegen `objc2-app-kit` 0.3.2, außerhalb des
Projektbaums; KRK selbst lässt sich nicht starten, der Abnahmelauf ist Nutzerarbeit).

**Die erste Vermutung hält.** `setHidden:` hält die Gesamtbreite der Tabelle fest und schlägt die
frei werdenden Punkte samt einem Zellenabstand von 17 Punkten der Namensspalte zu. Bei 700 Punkten
Sichtfläche: Name 337 → 434 (Größe weg) → 541 (Typ zusätzlich weg). Keine andere Spalte wird
angefasst.

**Die zweite Vermutung hält in ihrer Formulierung nicht, und die Messung sagt es genauer.** Die
Gesamtbreite bleibt nicht „größer als die Sichtfläche", sie **ändert sich überhaupt nicht**. Vier
Spalten in ihrer natürlichen Breite brauchen 603 Punkte; ein Dateifenster, das schmaler ist, und
zwei nebeneinander in einem gewöhnlichen Fenster sind es, zeigt die Tabelle deshalb schon vor
jedem Schalten mit Überstand. Das Wegschalten von Größe und Typ macht 204 Punkte frei, die genau
diesen Überstand auflösen würden; sie gehen stattdessen an Name, die Tabelle bleibt 603 breit, und
Datum steht weiter außerhalb. **Der frei werdende Platz erreicht die Sichtfläche nie.**

**Was AppKit von sich aus richtig macht.** Bei einer Änderung der Sichtfläche trifft
`FirstColumnOnlyAutoresizingStyle` die vom Nutzer gewählte Regel schon heute: 900 Punkte → Name
537, 500 → 137, 400 → Mindestbreite 100, die drei schmalen unangetastet. Kaputt ist allein der Weg
über `setHidden:`.

## Behebung am 260812

`Dateifenster::spaltenbreiten_verteilen` (`crates/krk-ui/src/appkit/tabelle.rs`) setzt die
sichtbaren Spalten auf ihre natürliche Breite, misst den rechten Rand der letzten sichtbaren
Spalte über `rectOfColumn:` und gibt der Namensspalte, was bis zur Sichtfläche fehlt, nach unten
begrenzt durch ihre Mindestbreite von 100. Gerufen wird sie einmal je Dateifenster in
`Anwendungsdelegierter::spaltenanzeige_nachziehen`, also für beide Fenster und zu beiden
Anlässen (Aufbau und Schalter). Die Betriebsart bleibt unverändert.

Gegengeprobt am selben kopflosen Aufbau: in jeder geprüften Schalterstellung sitzt der rechte Rand
der letzten sichtbaren Spalte danach genau auf der Sichtfläche. Die eine Ausnahme ist die
Sichtfläche, in die alle vier Spalten auch mit Name auf seiner Mindestbreite nicht mehr
hineinpassen (unter rund 463 Punkten); dann bleibt der waagerechte Schieber, und so steht es im
Entscheid.

---
Resolved: 260812 — `Dateifenster::spaltenbreiten_verteilen` in
`crates/krk-ui/src/appkit/tabelle.rs`, gerufen aus
`Anwendungsdelegierter::spaltenanzeige_nachziehen` (`crates/krk-ui/src/appkit/anwendung.rs`).
Regel nach `shared/decisions/260812-0910_*_wie-werden-die-spaltenbreiten-nach-dem-wegschalten-verteilt.md`,
Möglichkeit 1. `make check` und `make bundle` grün. **Am laufenden Bündel ungesehen**: der
Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit.
