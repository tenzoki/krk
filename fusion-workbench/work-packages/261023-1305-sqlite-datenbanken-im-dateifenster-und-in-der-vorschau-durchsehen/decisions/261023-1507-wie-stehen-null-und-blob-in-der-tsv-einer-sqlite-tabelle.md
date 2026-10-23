# Wie stehen NULL und BLOB in der TSV einer SQLite-Tabelle, die Vorschau und Ausfuhr teilen?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, 261023-1340-was-tut-kopieren-aus-einer-sqlite-datenbank.md, 261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen.md

---

## Question

Der Plan liest eine Tabelle über einen einzigen Leser, den `Archivleser` des Archivwegs, und dieser liefert die Zeilen als TSV-Bytes, kodiert über `trenntext::feld_kodieren`. Dieselben Bytes zeigt die Vorschau, und dieselben Bytes schreibt das Kopieren als `<tabelle>.tsv` (Entscheid `261023-1340-was-tut-kopieren-aus-einer-sqlite-datenbank.md`, Möglichkeit 2). Zwei SQLite-Werte haben in TSV keine eigene Form: `NULL` unterscheidet sich vom leeren Text nicht, und ein BLOB ist kein Text. Der Analysebericht hat für die Vorschau einen Platzhalter mit Länge und eine sichtbare Form für `NULL` empfohlen, ohne die Ausfuhr zu kennen, die erst der Nutzer danach gewählt hat. Die Frage muss vor dem Schritt beantwortet sein, der den Tabellenleser schreibt, denn sie legt das Dateiformat jeder ausgeführten Tabelle fest.

## Options

1. **Eine Form, verlustfrei: `NULL` als leeres Feld, ein BLOB als SQL-Literal `x'…'` in Hexadezimal.**
   - Pros: ein Leser, eine Form, kein Schalter im `Archivleser`. Die Ausfuhr verliert kein Byte eines BLOB. Leeres Feld für `NULL` ist die Form, die Tabellenkalkulationen beim Einlesen erwarten.
   - Cons: `NULL` und der leere Text sehen in Vorschau und Datei gleich aus. Ein großer BLOB verdoppelt seine Länge und füllt die Lesegrenze der Vorschau (1 MiB) schneller; die Vorschau zeigt von ihm die ersten 40 Zeichen. Ein Text, der wörtlich `x'00'` lautet, ist von einem BLOB nicht zu unterscheiden.
2. **Eine Form, verlustbehaftet: `NULL` als leeres Feld, ein BLOB als Platzhalter `BLOB, <n> Bytes` in der Sprache der Oberfläche.**
   - Pros: ein Leser, eine Form; die Vorschau zeigt, was der Analysebericht empfahl.
   - Cons: die Ausfuhr enthält statt der Daten einen Text, der wie Daten aussieht, und wechselt mit der Sprache der Oberfläche. Wer die TSV wieder einliest, verliert jeden BLOB, ohne dass eine Meldung es sagt.
3. **Zwei Formen: die Vorschau zeigt `NULL` sichtbar und BLOBs als Platzhalter, die Ausfuhr schreibt Möglichkeit 1.**
   - Pros: jede Fläche bekommt die für sie beste Form.
   - Cons: der `Archivleser` bekommt einen Schalter, den allein das SQLite-Format liest, und zwei Rufer (Vorschau, Vorgangsmaschine) müssen ihn setzen; Vorschau und ausgeführte Datei zeigen für dieselbe Zelle verschiedene Bytes. Die sichtbare `NULL`-Form der Vorschau ist ein Zeichen, das auch als Text in einer Zelle stehen kann.

## Constraints

- Ein Leser für die Daten einer Tabelle (Planentscheidung, Abschnitt `## Approach` des Plans `261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md`); jede Kodierung geht über `trenntext::feld_kodieren`.
- Ein nutzersichtbarer Text steht in allen drei Sprachtabellen; ein Platzhalter in der Ausfuhr wäre damit sprachabhängig.
- Die Vorschau kürzt eine Zelle beim Ausrichten auf `trenntextvorschau::HOECHSTBREITE` Zeichen und begrenzt das Gelesene auf `TEXTGRENZE`.

## Recommendation

Möglichkeit 1. Sie hält den einen Leser ohne Schalter, und die Ausfuhr bleibt verlustfrei, was für eine Datei, die der Nutzer weitergibt, schwerer wiegt als die Unterscheidung von `NULL` und leerem Text in der Vorschau. Die Vorschau zeigt Tabellen zum Durchsehen; wer `NULL` genau braucht, öffnet die Datenbank im Standardprogramm (`return`, Entscheid `261023-1340-betritt-der-doppelklick-eine-sqlite-datenbank-und-oeffnet-return-sie-weiter-extern.md`).

## Antwort des Nutzers (261023)

Im Chat vorgelegt waren zwei Formen: a) `NULL` als leeres Feld, BLOB als Hex `x'…'` (verlustfrei), und b) beide als Platzhalter, etwa `NULL` und `[BLOB 1234 Bytes]`, lesbarer in der Vorschau, verlustbehaftet in der Ausfuhr. Wortlaut der Antwort, die zugleich den Plan freigibt:

> 2

Gewählt ist b), die verlustbehaftete Form. Sie liegt nahe bei Möglichkeit 2 dieses Datensatzes, weicht aber in einem Punkt ab: `NULL` steht nicht als leeres Feld, sondern als sichtbarer Platzhalter `NULL`. Vorschau und Ausfuhr teilen weiter dieselben Bytes, ein Leser ohne Schalter.

Nicht im Chat genannt und deshalb hier festgehalten: der BLOB-Platzhalter ist ein nutzersichtbarer Text und damit nach der Sprachregel sprachabhängig, also auch in der ausgeführten Datei; und eine Zelle, die wörtlich `NULL` enthält, ist von einem echten `NULL` nicht zu unterscheiden.

---
Answered: Möglichkeit b) aus dem Chat — NULL als Platzhalter `NULL`, BLOB als Platzhalter mit Länge, in Vorschau und TSV-Ausfuhr gleich, verlustbehaftet; ruled by user, Kai Stalmann <kai@stalmann.org>
