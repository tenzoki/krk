Die Leiste soll auch die Spalten Größe, Datum und Typ wegschalten

---

Der Nutzer hat am 260811-1732 nachgetragen: die geplante Leiste am unteren Rand bekommt neben den
Schaltern für die fünf Bereiche auch Schalter für die **Spalten der Dateilisten** — Größe, Datum
und Typ, je an/aus.

Das erweitert die Directive dieses vorgesehenen Circles und ist deshalb hier abgelegt, damit es
bei der Aktivierung gefunden wird.

---

**Schwere:** — (kein Defekt, eine Erweiterung des Zuschnitts)
**Gefunden:** Nutzer
**Betroffen:** dieser Circle, vor seiner Aktivierung
**Domain:** code

## Der Bestand, am 260811-1732 geprüft

`Spalte` (`crates/krk-ui/src/appkit/tabelle.rs:130`) trägt vier Werte: `Name`, `Groesse`,
`Geaendert`, `Typ`. Der Nutzer nennt drei davon — **`Name` bleibt also immer stehen**, was
plausibel ist, aber nicht ausgesprochen wurde. Wer den Circle aktiviert, klärt das mit.

„Typ" heißt in KRK die **Dateiendung**, nicht die Art des Eintrags; der Kommentar bei
`tabelle.rs:139-140` schreibt das aus, und die Sortierung nach Typ ordnet nach der Endung. Der
Schalter trägt damit einen Namen, den der Nutzer anders lesen könnte als die Spalte ihn meint.

## Was das für den Zuschnitt des Circles heißt

Die Leiste trägt danach **zwei Sorten** von Schaltern: fünf für die Bereiche der Fensterzeile und
drei für die Spalten der Dateilisten. Das ist mehr als eine längere Liste — die beiden Sorten
verhalten sich verschieden:

- Ein **Bereichsschalter** ändert die Aufteilung des Fensters und löst die proportionale
  Neuverteilung aus, um die es der Directive geht.
- Ein **Spaltenschalter** ändert den Inhalt beider Dateifenster und die Aufteilung **nicht**. Er
  berührt die Breitenregel gar nicht.

**Zu klären ist deshalb bei der Aktivierung:**

- Gelten die Spaltenschalter für **beide** Dateifenster gemeinsam oder je Seite? Die Breiten
  werden je Bereich geführt, die Spalten heute nicht.
- Überstehen sie einen Neustart? C7 der Runde 1 verlangt das für Tabs, Ordner, Auswahl, Breiten,
  Sichtbarkeit und Sortierung — Spaltensichtbarkeit steht dort **nicht**, `Sitzung` müsste also
  wachsen.
- Was geschieht mit der **Sortierung**, wenn die Spalte, nach der sortiert ist, weggeschaltet
  wird? Das ist die einzige Stelle, an der ein Spaltenschalter mehr tut als etwas zu verbergen.
- Bekommen die Schalter Tastenbefehle? Bei den Bereichsschaltern verlangt die Directive
  Tastatur **und** Maus.

## Zusammenhang

Der Circle führt bereits sieben offene Fragen, deren erste den Umfang der Runde festlegt. Dieser
Nachtrag vergrößert den Umfang und gehört bei jener Frage mitgedacht: eine Runde, die
Breitenregel **und** Spaltensichtbarkeit umbaut, ist deutlich mehr als eine, die nur die
Aufteilung anfasst. Ob beides in eine Runde gehört, ist selbst eine Frage.

Daneben liegt der gemeldete Defekt
`shared/issues/260811-1245_*_die-breite-des-vorschaufensters-faellt-beim-navigieren-in-der-dateiliste-zurueck.md`,
den die siebte Frage dieses Circles ohnehin führt.
