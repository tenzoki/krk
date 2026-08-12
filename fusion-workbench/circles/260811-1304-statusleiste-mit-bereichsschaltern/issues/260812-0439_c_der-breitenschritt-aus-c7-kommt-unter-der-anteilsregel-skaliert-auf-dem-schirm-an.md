Der Breitenschritt aus C7 kommt unter der Anteilsregel skaliert auf dem Schirm an

---

Kriterium C4.9 des Plans `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md` sagt zu:
„Die beiden Breitenbefehle aus C7 verschieben die Trennlinie weiterhin um genau einen Schritt von
40 Punkten." Nach Schritt 1 hält die Zusage **nur bei einer Fensterbreite**, nämlich dort, wo die
Summe der gespeicherten Breiten der sichtbaren Bereiche die verfügbare Breite trifft. Sonst kommt
der Schritt um den Faktor `verfügbare Breite / gespeicherte Summe` vergrößert oder verkleinert an.

---

**Schwere:** mittel (eine Zusage des Plans, die nach Schritt 1 nicht mehr allgemein hält)
**Gefunden:** coder, bei der Umsetzung von Schritt 1
**Betroffen:** `crates/krk-ui/src/fenstermodell.rs`, `Fenstermodell::breite_aendern`
**Domain:** code

## Gemessen, nicht geschlossen

Eine Wegwerfprobe am Baum vom 260812-0439, mit dem Stand nach Schritt 1 und dem
Auslieferungszustand der Sichtbarkeit (Lesezeichenleiste, beide Dateifenster, Vorschau). Vor jeder
Messung läuft `breiten_uebernehmen` mit den gerade ausgelegten Breiten, so wie es
`Anwendungsdelegierter::kommando_ausfuehren` vor jedem Befehl tut:

| Fensterbreite | Schritt auf dem Schirm | gespeicherte Summe |
|---|---|---|
| 1280 | 40,00 | 1280 |
| 1400 | 43,75 | 1280 |
| 1920 | 60,00 | 1280 |

Die Probe ist nach der Messung wieder entfernt worden; sie steht nicht im Baum.

## Warum das aus Schritt 1 folgt und kein Versehen ist

Die beiden Zusagen C4.7 und C4.9 ziehen gegeneinander, und Schritt 1 entscheidet die Sache
zugunsten von C4.7:

- **C4.7** verlangt, dass das Vergrößern des Fensters keine gespeicherte Breite ändert. Der Plan
  löst das mit der Rückrechnung in `breiten_uebernehmen`: die gemessenen Punktzahlen werden auf die
  **gespeicherte** Summe zurückgerechnet. Diese Summe ist damit eine Konstante, die das Fenster
  nicht mitzieht.
- **C4.9** verlangt einen Schritt von 40 Punkten **auf dem Schirm**. `breite_aendern` rechnet aber
  in gespeicherten Punkten, und zwischen beiden Maßstäben steht genau jener Faktor.

Vor dieser Runde fiel der Faktor von selbst auf 1 zurück, weil `breiten_uebernehmen` die gemessenen
Zahlen **roh** übernahm: die gespeicherte Summe war danach immer die verfügbare Breite. Genau das
gibt Schritt 1 auf, und zwar mit Grund — ohne die Rückrechnung verlöre ein ausgeblendeter Bereich
seinen Anteil (C4.8), weil seine eingefrorene Zahl gegen die wachsenden Zahlen der sichtbaren
zusammenschrumpfte.

## Wie der Baum heute damit steht

Nichts ist offen geblieben und nichts ist grün gerechnet worden:

- Die Probe `der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt` misst jetzt bei 1280
  Punkten statt bei 1400. Dort trifft die gespeicherte Summe die verfügbare Breite, und die Zusage
  ist prüfbar. Ihr Kommentar sagt ausdrücklich, dass die Zahl nicht beliebig ist, und verweist
  hierher.
- Der Kommentar an `Fenstermodell::breite_aendern` nennt den Maßstabsunterschied und verweist
  ebenfalls hierher.

## Zwei Wege, und beide gehören nicht in Schritt 1

1. **Den Schritt umrechnen.** `breite_aendern` bekommt das `Zeilenmass` und rechnet
   `BREITENSCHRITT` mit `gespeicherte Summe / verfügbare Breite` in gespeicherte Punkte um. Damit
   hält C4.9 über jede Fensterbreite. Dasselbe gilt dann für die Mindestbreiten, gegen die
   `breite_aendern` heute in gespeicherten Punkten deckelt: auch sie stehen im falschen Maßstab.
   Kosten: `breite_aendern` bekommt einen Parameter, und seine Aufrufer in
   `crates/krk-ui/src/appkit/anwendung.rs` müssen das Maß holen — dieselbe Durchreichung, die
   Schritt 2 für `umschalten` ohnehin baut. Der Weg ist deshalb **nach** Schritt 2 billig und davor
   teuer.
2. **C4.9 umformulieren.** Der Schritt gilt dann als Anteilsschritt und nicht als Punktschritt.
   Das ist die kleinere Änderung am Code (keine) und die größere an der Zusage: der Nutzer sähe
   eine Trennlinie, die bei breitem Fenster weiter springt als bei schmalem. Ob das stört, ist am
   laufenden Bündel zu sehen und nicht am Baum zu entscheiden.

Der erste Weg ist die Empfehlung. Er gehört als eigener Schritt hinter Schritt 2 des Plans und
nicht in Schritt 1, dessen Zuschnitt `breite_aendern` und `anwendung.rs` ausdrücklich nicht nennt.

## Zusammenhang

Der Plan führt unter `## Risiken und Gegenmaßnahmen` bereits einen benachbarten Fall („Die
gedeckelte Breite eines Bereichs wird beim nächsten Nachlesen sein neuer Wunsch") und nimmt ihn
an. Dieser hier ist ein anderer: dort geht es um den Wunsch, der sich ändert, hier um den Maßstab,
in dem ein Befehl rechnet.

---
Resolved: Weg 1 der beiden benannten, gebaut in Schritt 2 des Plans, wie es der Datensatz
empfohlen hat. `Fenstermodell::breite_aendern` nimmt jetzt das `Zeilenmass` als dritten Parameter
und rechnet den Schritt ueber die neue Funktion `Fenstermodell::massstab` (`gespeicherte Summe der
sichtbaren / verfuegbare Breite`) in gespeicherte Punkte um; **dieselbe Umrechnung gilt fuer die
beiden Mindestbreiten**, gegen die der Schritt deckelt, denn auch sie standen im falschen Massstab.
Der Weg des Masses ist der, den Schritt 2 fuer `umschalten` ohnehin gebaut hat:
`Aufteilung::zeilenmass` liest die beiden Zahlen aus der `NSSplitView`,
`Anwendungsdelegierter::zeilenmass` reicht sie durch. Die Behebung kostete damit keinen eigenen
Durchgang durch die Aufrufer.

Die Probe `der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt` misst wieder ueber
mehrere Fensterbreiten, naemlich ueber die drei aus der Messtabelle oben (1280, 1400, 1920), und
verlangt an jeder genau 40 Punkte auf dem Schirm — hin und zurueck. Gegengeprobt: mit einem fest auf
1 gesetzten Massstab faellt sie bei 1400 mit 43,75 Punkten, also mit genau der Zahl, die oben steht.

Der Massstab gilt genau, solange kein sichtbarer Bereich an seinem Mindestmass haengt; dort ist die
Abbildung zwischen gespeicherten Punkten und Punkten auf dem Schirm nicht linear. Das ist am
Kommentar von `massstab` benannt und nicht behandelt: eine Sonderregel dafuer waere ein zweiter
Rechenweg neben `bereichsbreiten`.

Abgenommen mit `make check`, exit 0. Behoben in Schritt 2 des Plans
`planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`, Protokoll
`history/260812-0512-coder-schritt-2-abweisung-an-den-mindestbreiten.md`.
