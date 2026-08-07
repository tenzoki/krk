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

---
Übergeben: Die Prüfung hat den Defekt weiter gefasst als seine Beschreibung — nicht nur die Null, sondern jede Byte-Angabe erscheint auf Englisch. Weitergeführt als issues/260806-1215_o_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md, dessen Auflösung beim ontocoder liegt. Dieser Eintrag geht darin auf.

---

**Abgleich 260806-1647: der Marker `_c_` steht, das beschriebene Verhalten steht aber auch noch.** "Zero KB" erscheint unverändert; behoben ist hier nichts, der Eintrag ist in den weiteren übergegangen. Das deckt `_c_` in seiner zweiten Lesart ("resolved, **or** closed") ab, und der Nachfolger `issues/260806-1215_*_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md` trägt `_o_`, sodass die Sache nicht verschwindet. Für den Leser, der nur die Markerliste überfliegt, bleibt es trotzdem eine Falle: die Zeile `Übergeben:` steht an der Stelle, an der die Konvention `Resolved:` erwartet. Der Marker bleibt, weil ein Umzug auf `_d_` die Sache als zurückgestellt ausgäbe, was sie nicht ist. Gemeldet wird der Punkt nicht als eigener Defekt; er steht im Abgleichsbericht `history/260806-1647-reconciliation.md`.

**Nachtrag 260807-0745: das beschriebene Verhalten steht nicht mehr.** Der
Halbsatz oben, "Zero KB erscheint unverändert", ist seit dem 260807 überholt.
Der Nachfolger `issues/260806-1215_*_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md`
ist umgesetzt: `resources/Info.plist` führt jetzt `CFBundleLocalizations` mit
`de` vor `en`, und am gebauten Bündel gemessen steht `0 KB`, `1 Byte`,
`512 Byte`. Der Marker `_c_` dieses Eintrags trägt damit rückwirkend auch seine
erste Lesart. Behoben wurde es an der Bündelbeschreibung und nicht hier, weil
der Weg über `setAllowsNonnumericFormatting` gemessen und verworfen ist.
