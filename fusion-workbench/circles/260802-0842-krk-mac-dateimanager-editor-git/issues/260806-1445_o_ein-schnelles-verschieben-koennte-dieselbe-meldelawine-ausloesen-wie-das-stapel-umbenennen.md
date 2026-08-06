Ein Verschieben innerhalb eines Datenträgers könnte dieselbe Meldelawine auslösen wie das Stapel-Umbenennen

---

Der Aufschub der Auffrischung gilt seit dem Nutzerentscheid vom 260806 nur für
das Umbenennen. Ein Verschieben **innerhalb eines Datenträgers** läuft aber
über `rename(2)` und ist damit genauso schnell; über genügend Einträge könnte
es dieselbe Meldelawine tragen, die den Defekt
`issues/260805-1337_*_die-dateiliste-ist-waehrend-eines-stapel-umbenennens-im-angezeigten-ordner-leer.md`
verursacht hat.

---

**Beobachtet ist das nicht.** Der Eintrag hält einen Vorbehalt fest, keine
Fehlfunktion.

**Warum trotzdem ein eigener Eintrag.** Der Nutzerentscheid nennt das
Verschieben ausdrücklich als nicht aufschiebend, und genau so ist es
umgesetzt. Der Vorbehalt hängt an einer Bedingung, die der Entscheid nicht vor
sich hatte: die Geschwindigkeit eines Verschiebens hängt davon ab, ob Quelle
und Ziel auf demselben Datenträger liegen, und das weiß erst die Laufzeit.
Wird der Vorbehalt zur Beobachtung, ist er ein Fall für die Lesestelle und
nicht für eine zweite Ausnahme in der Einordnung `schiebt_auffrischung_auf`
(Weg 2 des Ursprungsdefekts). Der Kommentar an der Funktion hält das fest.

**Nächster Schritt, wenn jemand ihn geht:** ein Verschieben von 10.000
Einträgen innerhalb eines APFS-Datenträgers im angezeigten Ordner fahren und
sehen, ob die Liste dabei leer läuft. Erst diese Messung entscheidet, ob es
eine Änderung braucht; eine Reparatur auf Verdacht wäre genau die
Sonderregel, die die Maxime supersimpel ausschließt.

**Dringlichkeit.** Gering, unbeobachtet.

**Aufgefallen bei:** der Begrenzung des Auffrischungsaufschubs auf schnelle
Vorgänge, Turn 23 der Sitzung 260806-1140.
