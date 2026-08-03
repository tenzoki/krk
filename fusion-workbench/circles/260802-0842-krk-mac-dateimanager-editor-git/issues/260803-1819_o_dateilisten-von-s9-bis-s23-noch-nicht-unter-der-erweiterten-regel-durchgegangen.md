Die Dateilisten von S9 bis S23 sind unter der erweiterten Regel noch nicht durchgegangen

---

Der Nachzug vom 260803-1819 hat die Dateilisten-Regel im Kopf von `## Implementierungsschritte` erweitert: ein Schritt nennt auch die vorhandene Datei, aus der er liest oder über die er auslöst, nicht nur die, die sein neues Modul einbindet. Angewandt ist die erweiterte Regel bisher allein auf S8. Die Listen der Schritte S9 bis S23 stehen weiter auf dem Stand der engeren Regel vom 260802-1859.

---

Der Anlass war der Defekt `260803-1755_c_dateiliste-von-schritt-8-nennt-fuenf-noetige-dateien-nicht.md`. Er nennt am Ende selbst, dass ein Nachzug über alle Schritte lohnt, und dasselbe Muster hat schon S7 getroffen, Defekt `260803-1309_o_dateiliste-von-schritt-7-nennt-fuenf-noetige-dateien-nicht.md`. Zwei aufeinanderfolgende Umsetzungen mit derselben Auslassung sind ein Muster und kein Einzelfall.

Der Nachzug ist bewusst nicht in derselben Bearbeitung erfolgt. Die Bearbeitung vom 260803-1819 hatte die Nutzerentscheidung zum Abnahmemaß von L1 und L9 einzuarbeiten; fünfzehn Dateilisten spekulativ zu erweitern hätte die Änderung unüberschaubar gemacht und Aussagen über Schritte erzeugt, deren Umsetzung noch nicht begonnen hat.

Was zu tun ist: die Listen von S9 bis S23 einmal unter der erweiterten Regel durchgehen und je Schritt fragen, an welcher vorhandenen Datei er etwas ablesen oder auslösen muss. Die Kandidaten mit der höchsten Trefferwahrscheinlichkeit sind die Schritte, die eine bestehende Ansicht abfragen oder ein Ereignis in eine bestehende Schlange stellen: S13, S16, S17, S19 und S21.
