Die Dateiliste von Schritt 17 nennt `anwendung.rs` und `tabs.rs` nicht, obwohl beide gebraucht werden

---

Der Plan zu S17 (`planning/260802-1428_o_plan-navigator-geruest-runde-1.md`,
Abschnitt `#### 17.`) führt seine Dateiliste als bindend und vollständig. Zwei
Dateien fehlen darin, und ohne beide lässt sich der Schritt nicht bauen, ohne
eine Zusage des Plans selbst zu brechen.

---

**`crates/krk-ui/src/appkit/anwendung.rs`.** Der Plan verlangt, dass der neu
angelegte Eintrag über `auffrischung::ordner_neu_lesen` aus S14 erscheint und
"nicht über einen zweiten Auffrischungsweg". `ordner_neu_lesen` nimmt eine
`Dateifenstersicht` entgegen, und die setzt allein der Anwendungsdelegierte um:
er ist die einzige Stelle, die beide Dateifenster hält. Aus
`appkit/tabelle.rs` wäre nur `DateifensterQuelle::neu_lesen` erreichbar, und
das ist genau der zweite Weg, den der Plan ausschließt — es frischt das andere
Dateifenster nicht mit auf. Die drei neuen Befehle liegen deshalb dort, wo
schon Kopieren, Verschieben, Löschen und Abbrechen liegen: in
`Anwendungsdelegierter::kommando_ausfuehren`.

**`crates/krk-ui/src/tabs.rs`.** Der Plan verlangt, dass die Auswahl nach dem
Anlegen über "den vorhandenen Sprung auf einen Namen" auf den neuen Eintrag
geht und "ein zweiter Weg, eine Zeile anhand ihres Namens auszuwählen", nicht
entsteht. Der Lesevorgang aus `ordner_neu_lesen` ist gestückelt und läuft noch,
wenn der Befehl zurückkehrt; der neue Eintrag steht zu diesem Zeitpunkt in
keinem Modell. Getragen wird der Name deshalb von der `wunschauswahl` des Tabs,
demselben Feld, das die Sitzungswiederherstellung, der Aufstieg aus C2, der
Sprung aus der Zwischenablage (C10) und die Auffrischung aus C9 schon benutzen.
Es ist privat, und `Tabliste` hatte keinen Setzer dafür. Hinzugekommen ist
genau einer, `Tabliste::wunschauswahl_setzen`, neunzehn Zeilen mit Begründung.

Beide Eingriffe sind gemacht und im Sitzungsbericht
`history/260804-2040-s17-stapelumbenennen-anlegen-namenseingabe.md`
ausgeschrieben. Der Defekt betrifft die **Plandatei**: ihre Dateiliste ist an
dieser Stelle unvollständig und sollte um die beiden Einträge samt Begründung
ergänzt werden, damit die nächste Runde sich nicht dieselbe Frage stellt.
