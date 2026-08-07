# Kann der Auffrischungsaufschub entfallen, nachdem die Lesestelle nicht mehr vorab leert?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:**
`issues/260805-1337_*_die-dateiliste-ist-waehrend-eines-stapel-umbenennens-im-angezeigten-ordner-leer.md`,
`issues/260806-1331_*_der-auffrischungsaufschub-gilt-fuer-alle-fuenf-operationsarten-statt-nur-fuer-die-schnelle.md`,
`issues/260806-1445_c_ein-schnelles-verschieben-koennte-dieselbe-meldelawine-ausloesen-wie-das-stapel-umbenennen.md`,
`crates/krk-ui/src/auffrischung.rs` (`schiebt_auffrischung_auf`),
`crates/krk-core/src/verzeichnis/modell.rs` (`lesevorgang_beginnen`)

---

## Question

Der Aufschub der Auffrischung waehrend eines eigenen Vorgangs
(`schiebt_auffrischung_auf`, heute wahr allein fuer das Stapel-Umbenennen) ist
am 260806 eingezogen worden, um genau einen Defekt abzufangen: die leere
Dateiliste waehrend eines Stapel-Umbenennens im angezeigten Ordner
(`260805-1337`).

Am 260807 ist die Ursache dieses Defekts an der Lesestelle behoben. Ein
Lesevorgang leert sein Ordnermodell nicht mehr vorab, sondern ersetzt den
Bestand erst mit dem ersten gelieferten Stapel
(`Ordnermodell::lesevorgang_beginnen`). Die leere Liste kann damit nicht mehr
entstehen — auch nicht ohne den Aufschub.

Damit steht die Frage, ob der Aufschub sein Daseinsrecht verloren hat. Sie muss
jetzt beantwortet werden, weil `schiebt_auffrischung_auf` eine
Fallunterscheidung ueber alle fuenf Operationsarten haelt, die jede neue
Operationsart zu einer bewussten Einordnung zwingt. Eine Sonderregel ohne Grund
ist genau das, was die Maxime "supersimpel" ausschliesst; eine Sonderregel mit
Grund gehoert dagegen erklaert an ihren Platz.

## Befund des Coders

**Der Aufschub faengt nach der Umstellung nicht mehr dieselbe Fehlfunktion ab,
sondern eine mildere.** Gemessen und gerechnet:

- FSEvents sammelt 300 ms, bevor es meldet (`SAMMELVERZOEGERUNG` in
  `crates/krk-ui/src/appkit/fsevents.rs`). Zwei Meldungen desselben laufenden
  Vorgangs liegen also mindestens 300 ms auseinander.
- Ein vollstaendiger Lesevorgang braucht auf dem Referenzgeraet 43 ms fuer
  10.000 Eintraege und 492 ms fuer 100.000 (`messungen/260807-0002-...`).

Daraus folgen zwei Faelle:

1. **Ordner bis rund 60.000 Eintraege.** Der Lesevorgang wird zwischen zwei
   Meldungen fertig. Ohne den Aufschub saehe der Nutzer eine vollstaendige,
   sortierte Liste, die sich rund dreimal je Sekunde erneuert. Unruhig, aber
   nicht falsch.
2. **Ordner darueber.** Der Lesevorgang wird nicht fertig, bevor die naechste
   Meldung ihn neu aufsetzt. Die Liste zeigte dann dauerhaft nur den Anfang des
   Ordners in Lesereihenfolge, also unsortiert und unvollstaendig, und kaeme
   fuer die ganze Laufzeit des Vorgangs nicht mehr in ihren sortierten Zustand.

Fall 2 ist nicht die leere Liste des Ursprungsdefekts, aber er ist eine
Fehlfunktion. Der Aufschub verhindert ihn weiterhin.

## Options

1. **Den Aufschub stehen lassen, seine Begruendung nachziehen.** Der Kommentar
   an `schiebt_auffrischung_auf` beschriebe dann nicht mehr die leere Liste,
   sondern den unvollstaendigen, unsortierten Stand aus Fall 2.
   - Pro: Fall 2 bleibt abgefangen. Kein Verhalten aendert sich, also keine
     neue Messung noetig.
   - Contra: Die Sonderregel bleibt, und sie greift auch dort, wo sie nichts
     mehr abfaengt (Ordner unter der Schwelle).
2. **Den Aufschub entfernen.** `schiebt_auffrischung_auf`,
   `aufgeschobene_ordner` und `auffrischung_aufgeschoben` fielen weg, samt der
   Fallunterscheidung ueber die Operationsarten.
   - Pro: Eine Regel weniger, und die Auffrischung liefe fuer alle fuenf
     Operationsarten gleich. Der Nutzer saehe seinen Ordner sich waehrend eines
     Stapel-Umbenennens fuellen statt erst am Ende.
   - Contra: Fall 2 kaeme zurueck. Bei 100.000 Eintraegen ist das ein sichtbar
     falscher Stand ueber die ganze Laufzeit.
3. **Den Aufschub an die Ordnergroesse haengen** statt an die Operationsart.
   - Pro: Er griffe genau dort, wo er noetig ist.
   - Contra: Eine Schwelle ist ein neuer Regelparameter, der gemessen, begruendet
     und gepflegt werden will, und er haengt am Geraet. Das ist mehr Sonderregel
     als heute, nicht weniger.

## Constraints

- Die zehn Zeitzusagen aus C8 stellen keine Zusage ueber die Auffrischung. Kein
  Ausgang dieser Frage ist an einer Zusage abnehmbar; entschieden wird sie am
  Erlebnis.
- Der zweite Ausloeser aus S16 frischt den Ordner beim Abschluss des Vorgangs
  ohnehin auf. Jeder Ausgang endet also mit einem richtigen Stand, die Frage
  betrifft allein, was der Nutzer waehrend des Laufs sieht.

## Recommendation

Option 1. Der Aufschub faengt weiterhin eine Fehlfunktion ab, nur eine andere
als bei seiner Einfuehrung, und sein Preis ist gering: er wirkt allein waehrend
eines eigenen Stapel-Umbenennens und allein auf dessen eigene Ordner. Option 2
tauscht eine reale Fehlfunktion gegen eine Zeile weniger Code ein. Option 3
kostet mehr Regel als sie einspart.

---
Answered:
Implemented:
Deferred:
Superseded by:
