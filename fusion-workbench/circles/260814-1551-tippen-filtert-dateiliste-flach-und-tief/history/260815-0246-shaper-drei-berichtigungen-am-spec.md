# Shaper — fünf Berichtigungen am Spec der Filter-Runde

**Datum:** 260815-0246
**Agent:** shaper (in-Circle-Klärung, Turn 2, Reparaturrunde)
**Circle:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/`
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`
**Baum:** `195791a`, unverändert gelassen

---

## Der Auftrag und was daraus wurde

Drei Berichtigungen waren beauftragt, zwei weitere gleichen Typs waren mitzunehmen. Alle
fünf sagen dasselbe: der Spec behauptet etwas anderes, als der Baum tut. **In allen fünf
hat der Baum recht behalten**, und keine Zeile Code ist geändert worden.

| Datensatz | Was er sagt | Entscheidung |
|---|---|---|
| `260815-0233` | Das zweite Bild zeigt einen Abstieg mit Rückkehr, den es nicht mehr gibt | Bild neu gezeichnet, Kreiszählung ersetzt, C3.8/C3.10/C3.13 nachgezogen, C3.15 neu |
| `260815-0211` C1.11 | „meldet nichts", und jeder Operationsbefehl meldet „es ist nichts ausgewählt" | Kriterium an den Baum gezogen |
| `260815-0211` Ersatzzeile | greift beim Tippen und nicht beim Umschalten von „Deep" | Ungleichheit als Entscheidung festgehalten, neues C2.14 |
| `260814-2320` | C5.5 verlangt den Markdown-Eintrag, C5.7 schließt ihn aus | Vorschlag des Datensatzes wörtlich übernommen |
| `260814-2254` | C6.10 nennt zwei Größen, die Signatur trägt drei | Halbsatz aus dem Vorschlag ergänzt |

Alle fünf Datensätze sind mit `Resolved:`-Zeile auf `_c_` umbenannt.

---

## Das zweite Bild, und wie die Zahlen entstanden sind

Der Umbau des Durchlaufs (`195791a`) hat die Kante „zurück zum übergeordneten Ordner"
abgeschafft. Ein Ordner wird jetzt ganz gelesen, seine Unterordner wandern als Pfad auf
einen Stapel, und der nächste wird erst geöffnet, wenn der laufende zu Ende ist.

**Vier Änderungen am Bild**, über die drei Stellen des Datensatzes hinaus:

1. `in ihn absteigen` heißt `seinen Pfad vormerken` und führt nicht mehr zur
   Verknüpfungsprüfung zurück, sondern zurück auf die Einträge des Stapels.
2. Die Rückkehrkante ist weg. An ihre Stelle tritt der Knoten
   `ist noch ein Ordner vorgemerkt?` als Schleifenkopf.
3. **Die Frage nach dem Abbruch steht als eigener Knoten da**, unmittelbar vor
   `nächsten Stapel holen`. Vorher trug eine gepunktete Kante von `STAPEL` nach `ENDE` den
   Abbruch, ohne ihn als Entscheidung zu zeigen; jetzt ist C3.4 am Bild ablesbar.
4. **Ein vierter Ausgang `nicht entschieden`**, gespeist aus dem Abbruch und aus
   `fehlt dem Prozess ein Deskriptor?`. Er trägt keinen Befund an das erste Bild.

**Die Zahlen sind am geschriebenen Bild nachgezählt und nicht geschätzt.** Ein kleiner
Parser über die drei Mermaid-Blöcke des Spec hat Knoten, Kanten und einfache Kreise
gezählt; er hat auch die Zahlen der Bilder 1 und 3 bestätigt, die seit dem 260814
dastehen.

| | vorher | jetzt |
|---|---|---|
| Knoten | 19 | **22** |
| Kanten | 27 | **31** |
| einfache Kreise | 7 | **5** |

Die fünf Kreise stehen im Spec einzeln in einer Tabelle: zwei über die Einträge eines
Stapels, einer über die Stapel eines Ordners, zwei über die vorgemerkten Ordner. Keiner
läuft mehr über einen zweiten offenen Leser, und genau daran hängt die Zusage, dass die
Tiefe eines Baumes keine Grenze ist.

Eine Quelle, zwei Senken, größter Ausgangsgrad 2, größter Eingangsgrad 3, kein Knoten ohne
Weg zu einem Endpunkt.

## Was beim Prüfen der Kriterien C3.1 bis C3.15 gegen das Bild aufgefallen ist

Der Auftrag verlangte, jedes Kriterium der Fähigkeit C3 Pfad für Pfad gegen das neue Bild
zu lesen. Zwei Stellen sind dabei aufgefallen, und beide stehen jetzt im Spec:

**Der Kreis über `fehlt dem Prozess ein Deskriptor? — nein` trägt keine Abbruchfrage.** Ein
Ordner, der sich nicht öffnen lässt, hält keinen Stapel und passiert deshalb keine
Stapelgrenze. C3.4 ist über eine Kette solcher Ordner der Sache nach leer und nicht
verletzt; der Absatz zum Abbruch sagt es jetzt, damit ein Leser, der C3.4 gegen das Bild
prüft, den Kreis nicht für eine Lücke hält.

**Die Verknüpfungsregel steht an zwei Knoten und nicht an einem.** Der Auftrag selbst wird
oben geprüft; eine tiefer liegende Verknüpfung fällt an
`ist es ein Ordner und keine Verknüpfung?` heraus, weil `Typ::Verknuepfung` neben
`Typ::Ordner` steht. Ihr Name wird einen Knoten davor geprüft wie jeder andere, eine
Verknüpfung mit passendem Namen ist also ein Treffer (C3.9).

## Die inhaltliche Frage des Datensatzes: C3.10 oder C3.13?

Der Datensatz stellte sie ausdrücklich: bekommt C3.10 einen Satz über den
Deskriptormangel, oder gehört der Fall als vierter Weg zu „nicht entschieden"? **Der Baum
entscheidet ihn als „nicht entschieden", und der Spec folgt ihm**, aus einem Grund, der
über die Übereinstimmung hinausgeht: C3.10 handelt von Gründen, die am Pfad liegen und
dauerhaft gelten, ein Deskriptormangel liegt am Prozess und gilt für einen Augenblick.
Beide in ein Kriterium zu legen hieße, zwei verschiedene Fragen mit einer Regel zu
beantworten.

Umgesetzt in drei Schritten: C3.10 grenzt sich jetzt ausdrücklich auf Gründe am Pfad ein,
C3.13 nennt die zwei Ursachen von „nicht entschieden", und das neue C3.15 trägt die Regel
mit ihrer Begründung.

**C3.8 ist dabei mitgezogen worden.** Die Zusage „keine Tiefengrenze" hält an einem Baum
von zweihundert Ebenen nur, wenn der Prozess dabei nicht ohne Deskriptor dasteht; das
Kriterium sagt das jetzt und zieht die Kindprobe unter 64 Deskriptoren als Nachweis mit.

## Zwei Entscheidungen, die anders hätten ausfallen können

**C1.11: das Kriterium an den Baum, nicht den Baum an das Kriterium.** Die Meldung „es ist
nichts ausgewählt" ist in der Lage, in der die Liste leer vor dem Nutzer steht, seine
einzige Auskunft darüber, warum sein Tastendruck nichts getan hat. Sie zu entfernen hieße,
den einen Zweig in `auftrag_stellen` zu ändern, durch den alle vier Operationsbefehle
laufen — also sichtbares Verhalten außerhalb dieses Specs.

**C2.14: die vier Wege bleiben ungleich, und das ist jetzt eine Entscheidung.** Beim
Tippen fällt eine Zeile endgültig weg, beim Umschalten von „Deep" nur so lange, bis der
Befund für ihren Ordner eintrifft. Die Ersatzzeile setzte die Auswahl auf die erste
sichtbare Zeile und verlöre den Platz des Nutzers dauerhaft, obwohl der Eintrag gleich
darauf wiederkommt. Der Preis der gewählten Richtung steht in C2.14 benannt.

## Die Zahl der Abnahmekriterien

**75 → 77.** C2 von dreizehn auf vierzehn, C3 von vierzehn auf fünfzehn; die übrigen vier
Fähigkeiten unverändert. Beide neuen Kriterien tragen allein **(Probe)**.

**Die zehn Bündelkriterien sind unverändert**, und damit auch der Durchgang, den der
Nutzer am Bündel fährt. Die Abnahmeliste `history/260815-0400-abnahmeliste-g2.md` ist
nachgezogen: Kopf, Zahlentabelle, sieben Zeilen der Kriterientafel und der Abschnitt der
Befunde, der jetzt elf statt acht führt.

## Eine Lücke, die ein Coder schließen müsste

**C3.15 ist zur Hälfte gemessen.** Gemessen ist, dass der Durchlauf keinen eigenen
Deskriptormangel erzeugt. Ungemessen ist, dass ein von außen herbeigeführter Mangel zu
keinem Befund führt: `krk_core::verzeichnis::sys::ist_deskriptormangel` hat keine Probe,
und der Zweig `Err(fehler) if ist_deskriptormangel(&fehler) => return None` wird von keiner
Prüfung erreicht. Das Muster dafür steht daneben — die Kindprobe mit `ulimit -n 64` in
`crates/krk-core/tests/verzeichnis.rs`. Die Kennzeichnung des Kriteriums sagt die Lücke
aus, statt eine Probe zu behaupten, die es nicht gibt.

## Was nicht angefasst wurde

- **Der Arbeitsbaum.** `crates/` und `resources/` sind unverändert; committen ist Sache des
  Nutzers.
- **Die zehn Zeitzusagen aus C8 der Runde 1.** Keine ist angefasst, keine elfte gesetzt.
- **Die vier offenen Entscheidungsdatensätze** dieses Circles. Keine der fünf
  Berichtigungen berührt sie.
