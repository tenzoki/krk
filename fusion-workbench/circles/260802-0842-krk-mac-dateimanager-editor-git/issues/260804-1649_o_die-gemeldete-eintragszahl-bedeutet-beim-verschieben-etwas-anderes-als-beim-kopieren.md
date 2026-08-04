Die gemeldete Eintragszahl bedeutet beim Verschieben etwas anderes als beim Kopieren

---

Ein Ordner mit 500 Einträgen, kopiert, meldet 501 übertragene Einträge. Derselbe Ordner, innerhalb eines Datenträgers verschoben, meldet 1. Beide Zahlen sind für sich richtig, und C4 verspricht dem Nutzer nur eine.

---

## Woher der Unterschied kommt

Das Kopieren steigt in den Baum ab und zählt jeden Eintrag, den es angefasst hat. Das Verschieben innerhalb eines Datenträgers ist `rename(2)`: ein Systemaufruf hängt den Verzeichniseintrag um, der Inhalt wird nie angefasst. Genau das ist der Grund, warum es schnell ist (gemessen: 200 µs für 200 MB), und genau deshalb gibt es keine 500 Einträge zu zählen.

Die Umsetzung von Schritt 15 hat sich für "was die Operation angefasst hat" entschieden und es in `crates/krk-core/src/operation/fortschritt.rs` am Feld `Bericht::eintraege` festgehalten. Die andere Lesart wäre "wie viele Einträge der Nutzer ausgewählt hat, mal ihre Inhalte", und die verlangte den Vorabdurchlauf, den `### Frage 6` des Plans ausdrücklich ausschließt.

## Wo es auffällt

C4 sagt: "Nach einem Abbruch nennt KRK, wie viele Einträge bereits übertragen wurden." Bricht der Nutzer das Verschieben von zehn Ordnern nach dem dritten ab, liest er "3 Einträge übertragen", obwohl es drei Ordner mit zusammen mehreren tausend Dateien waren. Das ist nicht falsch, aber es ist auch nicht die Auskunft, die er sucht.

Beim Verschmelzen zweier gleichnamiger Ordner und beim Verschieben über eine Datenträgergrenze zählt dieselbe Operation wieder je Eintrag, weil sie dort absteigen muss. Dieselbe Handlung liefert also je nach Datenträger eine andere Zahl.

## Was zu entscheiden ist

Ob die Zahl im Fortschrittsblatt und in der Abbruchmeldung aus S16 "angefasste Einträge" heißen soll oder "erledigte Positionen der Auswahl". Die zweite wäre über alle Fälle hinweg gleich, sagte aber bei einem einzigen großen Ordner nichts aus. Denkbar ist auch, beide zu zeigen: "3 von 10 Positionen, 4.812 Einträge".

Die Entscheidung gehört zu S16, wo das Blatt entsteht; sie steht hier, damit S16 sie nicht übersieht.

**Aufgefallen bei:** der Umsetzung von Schritt 15 am 260804-1649.
