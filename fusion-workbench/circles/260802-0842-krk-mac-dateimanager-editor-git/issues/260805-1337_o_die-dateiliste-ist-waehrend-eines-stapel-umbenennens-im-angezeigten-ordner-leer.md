Die Dateiliste ist während eines Stapel-Umbenennens im angezeigten Ordner leer

---

Läuft ein Stapel-Umbenennen über 5.000 Einträge, zeigt das Dateifenster, in dem es läuft,
für die ganze Laufzeit eine **leere Liste**. Die Statuszeile zeigt den Fortschritt richtig,
das Fenster nimmt Tastendrücke an, und nach dem Ende steht die vollständige Liste wieder da
— aber während des Vorgangs gibt es nichts zu sehen und nichts auszuwählen.

---

Am 260805-1330 am laufenden Bündel gesehen, zwei voneinander unabhängige Bildschirmfotos
während desselben Laufs, Prüfordner `/tmp/krk-s17c-gross` mit 5.000 Dateien. Auf beiden ist
die Liste leer, während die Statuszeile `Umbenennen: 3.452 Einträge … · Esc bricht ab`
trägt.

**Vermutete Ursache (inference, nicht gemessen):** die Dateisystembeobachtung aus C9. Jede
Umbenennung ändert den Ordner, den dieses Dateifenster gerade zeigt; FSEvents meldet das,
`auffrischung::ordner_neu_lesen` startet einen neuen Lesevorgang, und der leert das
Ordnermodell, bevor er den ersten Stapel anhängt. Bei 5.000 Umbenennungen in wenigen
Sekunden setzt die nächste Meldung den Lesevorgang neu auf, bevor er fertig ist, und die
Liste kommt nicht mehr zum Füllen.

**Warum das vorher nicht auffiel.** Bis S17c lief das Stapel-Umbenennen als Schleife auf
dem Hauptfaden. Der stand während der ganzen Operation, also konnte kein Lesevorgang
dazwischenkommen; die Liste war erst nach dem letzten Eintrag wieder dran. Beim Kopieren
und Verschieben tritt der Fall nicht auf, weil dort der **Zielordner** sich ändert und der
steht im anderen Dateifenster. Der Defekt ist damit eine Folge von S17c und keine
Regression an dessen Zusagen: C4 sagt Bedienbarkeit zu, und die Oberfläche ist bedienbar.

Zwei Richtungen, die eine Auflösung nehmen könnte, beide mit Wirkung über diesen Schritt
hinaus und deshalb hier als Frage und nicht als Reparatur:

1. Die Auffrischung während eines eigenen laufenden Vorgangs aussetzen und einmal am Ende
   nachholen. Der Abschluss ruft `ordner_neu_lesen` ohnehin schon.
2. Die Auffrischung entprellen, also mehrere Meldungen innerhalb einer kurzen Spanne zu
   einem Lesevorgang zusammenfassen. Das hülfe auch bei fremden Änderungen im Stapel.
