Eine verworfene Leistenauswahl lässt die Zeile gewählt, und ein zweiter Klick auf sie löst nichts aus
---
Schwere: Mittel. Seit dem Umbau der Auswahl auf den Faden. Fällt die Antwort auf ein Lesezeichen still, steht die Zeile als gewählt da, ohne dass der Klick je gewirkt hat, und derselbe Klick lässt sich nicht wiederholen.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `crates/krk-ui/src/appkit/anwendung.rs:3204`: `leistenauswahl_einziehen` vollzieht nur bei `dieselbe && gefragt == selbst.leistenauswahlstand()` und hat keinen anderen Zweig. Der Stand trägt Nummer, Seite, Tab und Blatt (`:3180`). `crates/krk-ui/src/leistenmodell.rs:714` bis `:716`: `waehlen` liefert `false`, wenn die Zeile schon gewählt ist. `crates/krk-ui/src/appkit/leiste.rs:381`: nur auf `true` folgt `auswahl_melden`. `leiste.rs:398`: `zeile_geklickt` wirkt allein für eine Verbindung. Die Tastatur bewegt die Auswahl nur auf eine andere Zeile (`leiste.rs:700`).

**Fehlerbild, abgeleitet und nicht im Programm gefahren.** Der Nutzer klickt ein Lesezeichen auf einer langsamen oder gerade aufwachenden Freigabe. Bis die Antwort kommt, wechselt er Tab oder Dateifenster, oder ein Blatt öffnet sich. Die Antwort fällt ohne Meldung. Die Zeile bleibt markiert, der Ordner erscheint nicht, und ein weiterer Klick auf dieselbe Zeile tut nichts. Wiederholen geht nur über eine andere Zeile, die ihrerseits navigiert. Löschen, F2 und die Pfadeingabe lassen sich dagegen neu auslösen.

Dass ein Klick auf die gewählte Zeile nichts tut, gab es schon vorher. Neu ist die Lage, in der der erste Klick nie gewirkt hat.

**Richtung.** Ich schlage einen Merker im Delegierten oder im Modell vor: er sagt, dass die Auswahl der Zeile verworfen wurde oder noch aussteht, und `zeile_geklickt` führt in dem Fall das Lesezeichen aus. Die Alternative ist, die Zeile beim Verwerfen abzuwählen. Das nimmt dem Nutzer allerdings die Rückmeldung, worauf er geklickt hat.

**Abnahme.** Eine reine Probe am neuen Wert zeigt: Nach einer verworfenen Antwort löst derselbe Klick die Auswahl erneut aus, nach einer vollzogenen nicht. Die Lage an einer aufwachenden Freigabe prüft der Nutzer im Lauf.
