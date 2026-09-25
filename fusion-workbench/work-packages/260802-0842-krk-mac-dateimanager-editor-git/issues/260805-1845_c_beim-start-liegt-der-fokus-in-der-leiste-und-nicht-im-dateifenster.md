Beim Start liegt der Eingabefokus in der Leiste und nicht im Dateifenster

---

Nach dem Start des Bündels steht der Ersthelfer des Fensters auf der Tabelle der
Lesezeichen- und Geräteleiste, nicht auf einer der beiden Dateilisten.
`Anwendungsdelegierter::fokus` liefert deshalb `Fokus::Leiste`, und **jeder
Befehl mit `Wirkungsbereich::Dateifenster` wirkt bis zum ersten Fokuswechsel
nicht** — stumm, wie der Wirkungsbereich es vorsieht. Betroffen sind unter
anderem `oeffnen`, `ordner_aufwaerts`, `in_papierkorb`, `endgueltig_loeschen`,
`kopieren`, `verschieben` und seit S18c `terminal_oeffnen`. Erst `shift+cmd+d`
oder ein Klick in eine Dateiliste macht sie erreichbar.

---

Gemessen am 260805-1845 im laufenden Bündel, mit der vorübergehenden Sonde aus
Schritt 18c. Zwei Läufe, beide mit wiederhergestellter `session.toml`:

- ohne vorherigen Tastendruck: `fokus=Leiste`, `ctrl+o` löst nichts aus, die
  Statuszeile bleibt leer, keine Terminal-Anwendung startet;
- nach `shift+cmd+d`: `fokus=Dateifenster`, `ctrl+o` öffnet Terminal.app im
  angezeigten Ordner.

Der zweite Lauf ist zugleich der Nachweis, dass der Fokusvorbehalt aus C5
greift; der erste zeigt, dass der Ausgangszustand der falsche ist.

**Warum das ein Defekt und keine Auslegung ist.** C1 macht die beiden
Dateifenster zur Mitte der Anwendung, und C2 legt die Tastaturnavigation auf
sie. Ein Nutzer, der KRK startet und die Pfeiltasten drückt, bewegt heute die
Auswahl in der Leiste. Die Leiste ist nach C5 ein Nebenbereich, den der Nutzer
mit `shift+cmd+l` ausdrücklich anspricht.

Die Ursache liegt vermutlich in der Reihenfolge der Ansichten in der
`NSSplitView` aus S12: die Leiste ist der linke der vier Bereiche und damit die
erste Ansicht der Schlüsselansichtskette, und `oberflaeche_aufbauen` setzt
keinen Ersthelfer, bevor es `makeKeyAndOrderFront` ruft. Ein
`fenster.makeFirstResponder(...)` auf die Liste des aktiven Dateifensters wäre
die naheliegende Behebung; geprüft ist sie nicht.

Gefunden bei der Abnahme von Schritt 18c (C11), nicht von ihm verursacht: der
Befehl `terminal_oeffnen` trägt denselben Wirkungsbereich wie die zwölf
Befehle, die schon vorher betroffen waren.

---
Resolved: `oberflaeche_aufbauen` setzt den Eingabefokus als letzte Zeile des
Aufbaus auf `kommandos::fokus::BEIM_START` (`Fokus::Dateifenster`), über
dieselbe eine Stelle `fokus_setzen`, die auch die beiden Fokusbefehle aus C5
gehen. Der Aufruf steht **nach** `makeKeyAndOrderFront`, weil AppKit beim
ersten Anzeigen sonst die erste Ansicht der Schlüsselansichtskette einsetzt und
die Zeile davor wirkungslos wäre.

Der Fokus wird nicht gespeichert: C7 zählt Tabs, Ordner, Auswahl, Breiten,
Sichtbarkeit und Sortierung auf, und der Fokus gehört nicht dazu. Der
Startzustand ist damit immer derselbe. Aus der Sitzung kommt allein, **welches**
der beiden Dateifenster den Fokus bekommt.

Am laufenden Bündel geprüft, ohne vorher eine Taste zu drücken, die den Fokus
setzt: `fokus=Dateifenster`, `right` steigt in den ausgewählten Ordner ein,
`ctrl+o` öffnet Terminal.app darin. Die Gegenprobe mit abgeschalteter Zeile
liefert `fokus=Leiste` und weder Einstieg noch Terminal. Einzelheiten in
`history/260805-1901-fokus-beim-start-in-das-dateifenster.md`.
