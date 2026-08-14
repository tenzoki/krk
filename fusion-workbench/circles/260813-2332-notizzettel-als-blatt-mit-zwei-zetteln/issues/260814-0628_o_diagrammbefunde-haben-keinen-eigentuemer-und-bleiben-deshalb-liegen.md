Diagrammbefunde haben keinen Eigentümer und bleiben deshalb liegen

---

Die Diagrammprüfung schließt regelmäßig mit dem Satz, ihre Befunde seien „an Ort und Stelle
zu beheben". Der Satz benennt keinen, der es tut. In den Runden 8 und 9 ist derselbe Befund
dreimal erhoben und zweimal nicht behoben worden; behoben wurde er beim dritten Mal nur,
weil der Nutzer eigens einen Shaper-Lauf dafür angesetzt hat. Der Mechanismus, der ihn ohne
diesen Eingriff aufgegriffen hätte, besteht nicht.

---

**Schwere:** niedrig. Kein Bau, kein Verhalten. Betroffen ist die Verlässlichkeit der
Spec- und Plandokumente als Beleg.

**Die drei Erhebungen**

| Wann | Wo | Befund | Behoben |
|---|---|---|---|
| 260813-1049 | Spec der Runde 8, Bild 2 | B2: der Teilgraph `BILLIG` zeichnet den Abbruch an einem von vier Knoten, abbrechen können alle vier | nein |
| 260813-1124 | Plan der Runde 8, neu gezeichnetes Bild | F1: derselbe Punkt am neuen Bild, die Prüfung sagt ausdrücklich „B2 besteht fort" | nein |
| 260814-0000 | Spec der Runde 9, Bild 1 | N1 und N2: zwei Entscheidungsrauten mit je einem Ausgang, und der ungezeichnete Zweig widerspricht Bild 2 desselben Dokuments | ja, mit dem Nachtrag vom 260814-0628 |

**Was das von dem Datensatz unterscheidet, der schon offen ist.** Der Datensatz
`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/issues/260813-1345_o_die-diagrammbefunde-am-spec-sind-nie-behoben-worden-und-das-stationsbild-zeigt-jetzt-sechs-wo-der-baum-sieben-traegt.md`
hält den Zustand von zwei Dokumenten der Runde 8 fest: dort stehen drei Stellen unberichtigt.
Er ist eine Bestandsaufnahme und keine Aussage über den Weg, auf dem ein Befund zu seinem
Bearbeiter kommt. Dieser Datensatz hier hält den Weg fest, und er hält ihn in einem Circle
fest, der noch läuft. Der andere liegt im Speicher einer geschlossenen Runde, die nicht
wieder fährt.

**Die Ursache, soweit sie belegt ist.** Die Diagrammprüfung ist beratend und schreibt keine
Datensätze; ihr Prompt schließt beides ausdrücklich aus. Der Spruch geht an das Nutzer-Tor,
und das Sitzungsprotokoll der Runde 8 hat den Satz „an Ort und Stelle zu beheben" als
Erledigungszusage übernommen, ohne ihn einem Schritt zuzuweisen. Ein Plan entsteht danach
aus dem Spec und nicht aus der Prüfung, also greift auch der Plan sie nicht auf. Damit gibt
es keine Stelle, an der ein nicht behobener Befund auffällt.

**Was zu tun wäre, in zwei Stufen.** Die kleinere: das Nutzer-Tor entscheidet je Befund
ausdrücklich zwischen „berichtigen" und „verwerfen", und der Spruch der Prüfung wird nicht
als erledigt verbucht, solange keine der beiden Entscheidungen gefallen ist. Die größere:
die Befunde werden Schritte des Plans, mit den Kosten, die jeder Planschritt hat. Welche
der beiden richtig ist, entscheidet der Nutzer; dieser Datensatz entscheidet es nicht.

**Ein Hinweis zum Ort.** Das Muster reicht über diesen Circle hinaus und gehörte der Sache
nach in `shared/issues/`, weil dort der einzige Speicher liegt, der über die Runden hinweg
lebendig bleibt. Der Nutzer hat den aktiven Circle als Ort vorgegeben, und der Shaper folgt
dem. Genau die Eigenschaft, die den Datensatz vom 260813-1345 unsichtbar gemacht hat, das
Liegen im Speicher einer abgeschlossenen Runde, wird diesen hier treffen, sobald die neunte
Runde schließt. Ein Umzug nach `shared/issues/` vor dem Rundenabschluss nähme das weg.

**Kontext**

- Gefunden beim Nachziehen des Spec der Runde 9 am 260814-0628, ausgelöst durch die
  Schlussbemerkung der Prüfung `reviews/260814-0000-conceptrev-spec-notizzettel-als-blatt-mit-zwei-zetteln.md`.
- Der Befund an diesem Spec selbst ist behoben; siehe den Abschnitt „Was der Nachtrag vom
  260814 geändert hat" in `planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md`.
