Der Größenformatierer schreibt "Zero KB" auf Englisch

---

`NSByteCountFormatter` liefert für null Bytes den Text "Zero KB" statt einer deutschen
Entsprechung. Die Zahlen selbst kommen in deutscher Schreibweise ("10 KB"), allein das
Wort für null ist englisch. Sichtbar an zwei Stellen:

- in der Größenspalte des Dateifensters bei jeder leeren Datei (seit S12),
- seit S16c zusätzlich mitten in einem deutschen Satz der Statuszeile:
  `3 markiert, davon 3 Ordner, Zero KB`.

---

Am 260805-1130 im laufenden Bündel gesehen, Bildschirmfoto während der Abnahme von S16c.

Der Grund ist die Lokalisierung: die Zahl formatiert Foundation nach der Spracheinstellung
des Nutzers, das Wort "Zero" kommt dagegen aus einer Zeichenkettentabelle, die Foundation
nach den Sprachen des **Bündels** wählt. `resources/Info.plist` nennt keine, also fällt
Foundation auf Englisch zurück.

Zwei Wege stehen zur Wahl, und beide berühren Dateien, die S16c nicht anfassen darf:

1. `CFBundleLocalizations` in `resources/Info.plist` um `de` erweitern. Dann kommt das
   Wort aus der deutschen Tabelle des Systems. Wirkt auf jede Foundation-Ausgabe des
   Programms und nicht nur auf diese.
2. `setAllowsNonnumericFormatting(false)` am gemeinsamen Formatierer in
   `crates/krk-ui/src/appkit/tabelle.rs`. Dann steht dort "0 bytes" statt "Zero KB" —
   ebenfalls englisch, aber unauffälliger. Ändert zugleich die Größenspalte aus S12.

Beide sind Entwurfsentscheidungen mit Wirkung über diesen Schritt hinaus; deshalb steht
hier ein Defekt und keine Reparatur. Der Formatierer selbst ist richtig gewählt: der Plan
verlangt für den Markierungsstand ausdrücklich denselben, der die Größenspalte
beschriftet, und eine zweite Rechnung daneben wäre eine zweite Wahrheit.
