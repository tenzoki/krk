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

---
Resolved: Der Nutzerentscheid vom 260806-2345 hat den hier vorgeschlagenen
nächsten Schritt (ein Verschieben von 10.000 Einträgen messen) verworfen und
stattdessen die Ursache angegangen — an der Stelle, die dieser Eintrag selbst
als die richtige benennt: der Lesestelle.

`Ordnermodell::leeren` ist entfallen. An seiner Stelle steht
`Ordnermodell::lesevorgang_beginnen` (`crates/krk-core/src/verzeichnis/modell.rs`),
das die Generation setzt und den Ersatz des Bestands nur **vormerkt**;
eingelöst wird er von `anhaengen` mit dem ersten gelieferten Stapel oder, wenn
der Ordner keinen liefert, von `abschliessen`. `Tabliste::aktiven_neu_lesen`
(`crates/krk-ui/src/tabs.rs`) tauscht den Tab dafür nicht mehr gegen einen
frischen aus, sondern behält sein Ordnermodell.

Damit ist die Kante geschlossen, und zwar für jede Operationsart und für jede
künftige dazu: eine Meldelawine kann die Dateiliste nicht mehr leer laufen
lassen, weil kein Lesevorgang sie mehr vorab leert. Eine zweite Ausnahme in
`schiebt_auffrischung_auf` ist nicht entstanden; die Funktion ist unverändert.

Ob der Aufschub jetzt ganz entfallen kann, ist eine eigene Frage und liegt als
`decisions/260807-0010_o_kann-der-auffrischungsaufschub-entfallen-nachdem-die-lesestelle-nicht-mehr-vorab-leert.md`
beim Nutzer. Kurzform des Befunds: nein, er fängt weiterhin eine Fehlfunktion
ab, nur eine mildere als bei seiner Einführung.

Nachgemessen: `messungen/260807-0002-MacBookPro15-1-ersatz-beim-ersten-stapel-l2-l3-l10.txt`
— L2, L3 und L10 halten in allen fünf Runden, gegen eine zur selben Zeit
gefahrene Basisreihe ohne die Umstellung.
