# Wie werden die Spaltenbreiten verteilt, nachdem eine Spalte weggeschaltet wurde?

---
**Domain:** code
**Status:** answered
**Filed by:** orchestrator
**Cross-references:** `shared/issues/260812-0907_*_das-wegschalten-einer-spalte-verteilt-die-breiten-nicht-neu-und-schiebt-datum-aus-dem-bild.md`, `circles/260811-1304-statusleiste-mit-bereichsschaltern/_b_circle.md` (Runde 5), `crates/krk-ui/src/appkit/tabelle.rs:227` (die vier Breitenpaare), `:2237` (die Betriebsart)

---

## Question

Die Spaltenschalter der Runde 5 setzen `setHidden` und sonst nichts. Was mit dem frei werdenden
Platz geschieht, entscheidet heute allein die Betriebsart `FirstColumnOnlyAutoresizingStyle`, und
das Ergebnis hat der Nutzer als Defekt gemeldet: die Namensspalte bekommt zu viel Platz, und die
Spalte Datum steht außerhalb des Bildes.

„Optimal genutzt" ist noch keine Regel. Zu entscheiden ist, welche.

## Options

1. **Die schmalen Spalten behalten ihre natürliche Breite, Name bekommt den Rest.** Größe 80,
   Datum 130, Typ 90 Punkte, unverändert bei jeder Schalterstellung; Name nimmt auf, was übrig
   ist, nach unten begrenzt durch seine Mindestbreite von 100.
   - Pros: Die Absicht hinter der heutigen Betriebsart, also keine zweite Rechenvorschrift.
     Datum ist immer gleich breit und damit an derselben Stelle lesbar, egal was an ist. Die
     Zahlen stehen schon im Baum und sind begründet.
   - Cons: Name wächst weiterhin allein. Bei einem breiten Fenster mit zwei sichtbaren Spalten
     bekommt Name sehr viel Platz.
2. **Alle sichtbaren Spalten wachsen anteilig.** Dieselbe Regel, die seit der Runde 5 die fünf
   Bereiche der Fensterzeile teilt.
   - Pros: Eine Regel an zwei Orten, leicht zu erklären.
   - Cons: Datum bekäme bei zwei sichtbaren Spalten rund 350 Punkte für ein kurzes Datum. Es
     entstünde eine zweite Verhältnisrechnung neben `bereichsbreiten`, und die Spalten tragen
     anders als die Bereiche keine gespeicherten Wünsche.
3. **Anteilig, aber die schmalen Spalten bei ihrer doppelten natürlichen Breite gedeckelt.**
   - Pros: Datum bleibt lesbar breit statt aufgebläht, Name wird nicht übermächtig.
   - Cons: Eine Deckelung ist eine zusätzliche Regel, und die Zahl „das Doppelte" ist gesetzt
     und nicht hergeleitet.

## Constraints

- Eine gezogene Spaltenbreite überlebt heute keinen Neustart: `Sitzung` führt kein Feld dafür.
  Eine Regel, die gezogene Breiten achten will, stößt darauf.
- Die Mindestbreiten aus `tabelle.rs:227` gewinnen gegen jede Verteilung.
- Die Spaltensichtbarkeit gilt für beide Dateifenster gemeinsam; die Breiten werden je Tabelle
  gesetzt.

## Antwort 260812-0910

**Möglichkeit 1, gewählt vom Nutzer.** Die drei schmalen Spalten stehen bei jeder
Schalterstellung auf ihrer natürlichen Breite, Name nimmt den Rest.

Der Nutzer hat die drei Möglichkeiten mit gezeichneten Beispielen bei 1000 Punkten
Fensterbreite vorgelegt bekommen und Möglichkeit 1 gewählt. Der bewusst mitgenannte Preis ist
damit angenommen: Name wächst weiterhin allein, und bei einem breiten Fenster mit zwei
sichtbaren Spalten bekommt es viel Platz. Dafür steht Datum immer an derselben Stelle und in
derselben Breite, und das ist der Teil der Meldung, der wirklich stört.

**Was das für die Behebung heißt:** die Betriebsart bleibt, wie sie ist. Zu bauen ist, dass nach
jedem Schalten die sichtbaren Spalten auf ihre natürliche Breite zurückgesetzt werden und die
Gesamtbreite der Tabelle wieder der sichtbaren Fläche entspricht. Ob das genau die Ursache
trifft, ist vor dem Bauen zu messen — der Defektdatensatz führt die Vermutung ausdrücklich als
solche.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-0910` — Nutzerentscheid vom 260812-0910, vorgelegt mit gezeichneten Beispielen.
Implemented: 260812 — `Dateifenster::spaltenbreiten_verteilen` (`crates/krk-ui/src/appkit/tabelle.rs`), gerufen aus `Anwendungsdelegierter::spaltenanzeige_nachziehen` (`crates/krk-ui/src/appkit/anwendung.rs`). Die Betriebsart `FirstColumnOnlyAutoresizingStyle` steht unverändert.
Deferred:
Superseded by:
