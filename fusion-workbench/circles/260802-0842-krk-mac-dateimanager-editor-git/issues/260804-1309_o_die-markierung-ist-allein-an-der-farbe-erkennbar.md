Die Markierung aus C2 ist allein an der Farbe erkennbar

---

Schritt 13 macht die Mehrfachauswahl aus C2 sichtbar, indem er die Zellen eines markierten Eintrags orange einfärbt (`crates/krk-ui/src/appkit/tabelle.rs`, `zellenansicht`). Das ist das einzige Kennzeichen: Schrift, Hintergrund, Zeilenhöhe und Text bleiben gleich.

Ein Nutzer mit einer Rot-Grün-Schwäche unterscheidet Orange und die Beschriftungsfarbe auf dem dunklen Hintergrund schlecht; bei einer Blau-Gelb-Schwäche verschwindet der Unterschied fast ganz. Für ihn zeigt eine markierte Liste dasselbe Bild wie eine unmarkierte, und die vier Markierungsbefehle wirken folgenlos.

---

## Warum es zählt

Die Markierung ist kein Schmuck: die Dateioperationen aus C4 wirken auf sie. Wer nicht sieht, was markiert ist, kopiert oder löscht im Blindflug. Die Norton-Reihe, das Vorbild der Anwendung, färbt markierte Einträge ebenfalls, setzt aber zusätzlich die Zahl der markierten Einträge und ihre Gesamtgröße in eine Zeile am Fuß.

## Was zu tun ist

Nicht in diesem Schritt: die Statuszeile aus C1 trägt heute allein Fehlermeldungen, und was sie sonst noch zeigt, ist eine Festlegung und keine Nebenwirkung. Der Modulkopf von `crates/krk-ui/src/appkit/statuszeile.rs` hält ausdrücklich fest, dass Lesefortschritt und Eintragszahl in einer späteren Runde **in dieselbe Zeile** kommen und nicht in eine zweite daneben; die Zahl der markierten Einträge gehört in dieselbe Frage.

Vorgeschlagen: ein zweites Kennzeichen neben der Farbe. Zwei Kandidaten, beide klein:

- Die Zahl der markierten Einträge in der Statuszeile des Dateifensters, zusammen mit der Frage, was diese Zeile sonst noch trägt.
- Ein Zeichen in der Namensspalte vor dem Namen, so wie es Dateimanager mit Textoberfläche halten.

Welches, entscheidet der Nutzer; beide berühren das Aussehen der Liste, und keines folgt aus dem Spec.

---

Herkunft: gefunden bei der Umsetzung von Schritt 13 am 260804-1309, beim Nachweis des Abnahmekriteriums 7 aus C2 im laufenden Bündel.
