Der Sitzungslauf blieb einmal von drei Malen bei L6 stehen

---

Beim Nachprüfen des Defekts `260806-1235_o_der-sitzungslauf-der-abnahmestrecke-bricht-bei-l5-tab-ab-und-gibt-keine-zahl-mehr-aus.md`
lief die Sitzungsstrecke am 260806 dreimal aus einem Terminalfenster im
Vordergrund, also unter der Bedingung, unter der sie überhaupt misst. Zwei der
drei Läufe kamen vollständig durch. Der **erste** brach ab:

```
krk: die Messung l6 ist nach 10 s nicht am Ziel; seit dem Beginn sind 581
Bildgrenzen eingegangen. Es wird keine Zahl ausgegeben.
```

L1, L7, L5-Tab und L5-Fenster hatten in diesem Lauf ihre zwanzig Werte; erst
L6 blieb stehen. Der zweite, wortgleiche Lauf lieferte für L6 zwanzig Werte,
und der volle `make alle RUNDEN=1` danach nahm L6 mit einem 95. Perzentil von
47,1 ms ab (Zusage 100 ms).

Der Bildtakt lief während der zehn Sekunden weiter (581 Bildgrenzen sind rund
58 je Sekunde); es stand also nicht die Oberfläche.

**Was L6 misst.** Je Wiederholung: den Elternordner in den sichtbaren Tab
lesen, auf ihn warten, den Unterordner am Namen auswählen (`Handlung::Auswaehlen`,
ungemessen), dann `oeffnen` gemessen, bis `ordner_aktiv == unterordner`, der
Lesevorgang beendet ist und Zeilen dastehen
(`crates/krk-ui/src/messmodus.rs`, `sitzungsschritte` und
`sitzungsmessung_fertig`).

`inference:`, nicht gemessen: die wahrscheinlichste Stelle ist die ungemessene
Auswahl. `Tabellenquelle::eintrag_waehlen` liefert `Auswahlversuch::Unbekannt`,
wenn der Name im Modell nicht steht und **kein** Lesevorgang mehr läuft; der
Rückgabewert wird an der Aufrufstelle in `Anwendungsdelegierter::messhandlung`
verworfen. Trifft dieser Fall zu, drückt der nächste Schritt `oeffnen` auf
einen anderen Eintrag oder auf keinen, die Endbedingung steht nie, und die
Geduld läuft ab. Ein zweiter Kandidat ist ein Rennen zwischen dem Warteschritt
auf den Elternordner und der Auswahl.

**Was zu tun bleibt:** feststellen, welcher der beiden Fälle es war, und den
Fehlschlag sichtbar machen statt ihn zu verwerfen — ein abgewiesener
`Auswahlversuch` in einer Vorbereitung der Messstrecke ist ein Fehler der
Strecke und gehört abgebrochen, nicht in eine Zehn-Sekunden-Geduld.

**Aufgefallen bei:** der Gegenmessung zum L5-Tab-Defekt am 260806-1250.
Adressat: `coder`.
