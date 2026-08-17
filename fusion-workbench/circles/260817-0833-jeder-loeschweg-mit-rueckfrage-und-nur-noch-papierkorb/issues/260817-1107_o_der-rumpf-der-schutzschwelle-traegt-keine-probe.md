# Der Rumpf, der die Schutzschwelle trägt, ist von keiner Probe gedeckt

**Datum:** 260817-1107
**Gefunden von:** coderev, Durchsicht `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`, Befund 2
**Schwere:** Mittel
**Betrifft:** `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/kommandos/loeschwarnung.rs`
**Baumstand:** `472eb81`

## Der Befund

`loeschen_nach_rueckfrage` und `loeschauftrag_stellen` (`anwendung.rs:4603-4697`) sind die
Mechanik, um derentwillen diese Runde läuft. Vier Eigenschaften tragen sie, und keine ist
geprüft:

1. der laufende Vorgang wird **vor** dem Blatt gemeldet und nicht nach der Bestätigung,
2. die leere Auswahl kommt gar nicht bis zum Blatt,
3. ein Abbruch stellt keinen Auftrag,
4. der bestätigte Auftrag trägt die Auswahl, die im Blatt stand.

`kommandos/loeschwarnung.rs` trägt fünf Proben, aber allein über die beiden Texte. `krk-ui`
hat kein Bibliotheksziel, und ein Blatt lässt sich unter `libtest` nicht bedienen; der heutige
Zuschnitt ist ohne Umbau nicht prüfbar. Abgenommen ist er ebenfalls nicht, denn der
Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit.

## Richtung

Die vierstufige Reihenfolge ist eine Regel über vier Wahrheitswerte (Vorgang läuft, Auswahl
leer, Papierkorb vorhanden, bestätigt) und keine AppKit-Sache. Als reine Funktion neben
`kommandos::rueckschritt` und `kommandos::loeschwarnung` wäre sie mit einer ausgeschriebenen
Tafel prüfbar, so wie dieses Projekt seine übrigen Regeln hält.

Bündel B setzt mit der Papierkorbprüfung eine fünfte Stufe in dieselbe Kette. Wenn die Regel
umzieht, dann dort, nicht danach: ein Umzug nach Bündel B änderte dieselbe Stelle zweimal.
