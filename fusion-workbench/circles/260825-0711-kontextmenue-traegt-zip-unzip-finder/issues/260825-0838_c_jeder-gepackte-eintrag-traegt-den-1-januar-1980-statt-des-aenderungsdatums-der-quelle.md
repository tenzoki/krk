Jeder gepackte Eintrag trägt den 1. Januar 1980 statt des Änderungsdatums der Quelle

---

Das Archiv, das `operation::zippen` schreibt, führt für jeden Eintrag den Zeitstempel
`1980-01-01 00:00`. Das Änderungsdatum der Quelle kommt nicht mit, und nach einem Rundweg durch
Zip und Unzip trägt jede Datei dieses Datum.

---

**Gemessen am Arbeitsbaum am 260825-0838, während der Umsetzung von Schritt 2 der Runde 17.**
Neu entstanden mit diesem Schritt; vorher gab es kein Packen.

## Was der Baum trägt

```
$ /usr/bin/unzip -l quelle.zip
  Length      Date    Time    Name
---------  ---------- -----   ----
        0  01-01-1980 00:00   quelle/
        4  01-01-1980 00:00   quelle/oben.txt
        0  01-01-1980 00:00   quelle/unten/
        4  01-01-1980 00:00   quelle/unten/tief.txt
        8  01-01-1980 00:00   quelle/verweis.txt
```

Die Ursache liegt in der Merkmalswahl aus Schritt 1 und nicht in einer vergessenen Zeile.
`SimpleFileOptions::default()` setzt `last_modified_time` auf `DateTime::default_for_write()`, und
diese Funktion hat in `zip 8.6.0` zwei Rümpfe: mit dem Merkmal `time` liefert sie die aktuelle
Uhrzeit, ohne es den festen Wert `DateTime::DEFAULT`, also den 1. Januar 1980
(`zip-8.6.0/src/datetime.rs:183–198`). KRK bindet `zip` mit `default-features = false` ein, und
das Merkmal `time` gehört zum Vorgabesatz.

**Auch mit dem Merkmal `time` wäre der Befund nicht behoben, sondern verschoben.** Jene Fassung
schreibt die Uhrzeit des Packens an jeden Eintrag, nicht das Änderungsdatum der Quelle. Beide
Werte sind falsch; der zweite fällt nur weniger auf.

## Warum das trägt

Der Zeitstempel ist eine Angabe, die der Nutzer in der Dateiliste sieht und nach der er sortiert.
Ein Ordner, der einmal durch ein KRK-Archiv gelaufen ist, trägt danach für jede Datei dasselbe
Datum, und die Sortierung nach Änderungsdatum sagt nichts mehr aus. Das Zip des Finders und
`ditto(1)` erhalten die Zeitstempel; ein Nutzer, der KRKs Archiv mit einem davon vergleicht, sieht
den Unterschied unmittelbar.

Das Kopieren dieses Vorhabens überträgt das Änderungsdatum ausdrücklich
(`kopieren::ordnerangaben_uebernehmen` setzt `FileTimes` nach dem Inhalt). Dass das Packen es
fortwirft, ist deshalb keine bewusste Abweichung, sondern eine, die niemand entschieden hat.

## Warum Schritt 2 sie nicht behoben hat

Der Weg dorthin ist keine Zeile, sondern eine Wahl. Die MS-DOS-Zeitform des Zip-Formats ist eine
**bürgerliche Ortszeit** ohne Zeitzonenangabe, `SystemTime` dagegen eine Zahl von Sekunden seit
1970 in UTC. Die Umrechnung braucht die Zeitzone des Geräts, und die liefert keine der Kisten, die
`zip 8.6` ohne C-Code hereinbringt: `DateTime` nimmt Umwandlungen aus `chrono`, `jiff` und `time`
entgegen, alle drei hinter einem Merkmal. Eine von Hand gerechnete Kalenderumrechnung samt
Zeitzone wäre neuer, ungeprüfter Code an einer Stelle, die der Plan der Runde 17 nicht nennt.

## Vorschlag

Drei Wege, in der Reihenfolge steigender Kosten:

1. **Das Merkmal `time` von `zip` einschalten** und `last_modified_time` je Eintrag aus dem
   `SystemTime` der Quelle setzen. Erst zu prüfen, ob `time` C-Code hereinzieht; nach der
   Erhebung in Schritt 1 tut es das nicht, gezählt ist es aber nicht.
2. **Das erweiterte Zeitfeld 0x5455 mitschreiben** (`FullFileOptions` mit `add_extra_data`). Es
   trägt die Unix-Sekunden unmittelbar und braucht keine Zeitzone; jedes verbreitete
   Entpackwerkzeug liest es und zieht es der MS-DOS-Angabe vor. Die MS-DOS-Angabe bliebe daneben
   falsch stehen.
3. **Beides**, denn 2 allein lässt jedes Werkzeug, das nur die MS-DOS-Angabe liest, beim 1980er
   Datum.

Das Entpacken aus Schritt 3 spiegelt die Frage: es muss den Zeitstempel des Archiveintrags auf die
entpackte Datei setzen, sonst trägt sie die Uhrzeit des Entpackens. Beide Enden gehören in
denselben Zug.

**Schwere:** mittel. Keine Fehlfunktion und kein Datenverlust am Inhalt, aber ein verlorener Wert,
den der Nutzer sieht, und ein sichtbarer Unterschied zu dem Werkzeug, das er bisher benutzt hat.

**Gefunden:** coder, bei der Umsetzung von Schritt 2 der Runde 17 am 260825-0838, über
`/usr/bin/unzip -l` an einem selbst gepackten Archiv

**Betroffen:** `crates/krk-core/src/operation/zippen.rs` (`dateiwahl`, `ordnerwahl`,
`verknuepfung_packen`); `Cargo.toml`, Merkmalswahl von `zip`; für die Gegenrichtung
`crates/krk-core/src/operation/entpacken.rs` aus Schritt 3

**Domain:** code

---

**Nachtrag 260825, Schritt 3.** `crates/krk-core/src/operation/entpacken.rs` steht jetzt im Baum
und setzt **keinen** Zeitstempel: eine entpackte Datei trägt die Uhrzeit des Entpackens. Das ist
die im Absatz „Vorschlag" beschriebene Gegenrichtung, und sie ist absichtlich offen geblieben —
solange der Packlauf jedem Eintrag den 1. Januar 1980 gibt, machte ein Entpacken, das den
Zeitstempel des Eintrags übernähme, aus jeder Datei eine von 1980, und der Befund wäre von
einem verlorenen Wert zu einem falschen geworden. Wer den Datensatz abarbeitet, fängt am
Packende an. Der Modulkopf von `entpacken.rs` verweist auf diesen Datensatz.

**Abgleich 260825-1230 (reconciler), Marker `_o_` bestätigt.** Gegen den Baumstand `ddd41ff`
gelesen: `Cargo.toml:176` bindet `zip` weiterhin mit `default-features = false` und dem einen
Merkmal `deflate-flate2`, das Merkmal `time` steht nicht dabei. Weder `zippen.rs` noch
`entpacken.rs` nennt `last_modified_time`, `FullFileOptions`, `add_extra_data` oder `set_times`;
`dateiwahl` (`zippen.rs:504`) und `ordnerwahl` (`:513`) gehen unverändert über
`SimpleFileOptions::default()`. Beide Enden stehen damit so, wie der Datensatz sie beschreibt, das
Packende wie die im Nachtrag genannte Gegenrichtung. Nichts ist behoben, nichts ist
weitergewandert.

**Nachtrag 260825-1859, Umsetzung. Der Abschnitt „Vorschlag" ist an zwei Stellen widerlegt,
und zwar durch Messung.** Wer ihn spaeter liest, folgt den drei Befunden hier und nicht der
Aufzaehlung darueber.

**Vorschlag 1 traegt nicht.** Das Merkmal `time` von `zip` schaltet `default_for_write()` von
1980 auf `OffsetDateTime::now_utc()` um, also auf die **Uhrzeit des Packens**, und das ist
derselbe falsche Wert, den der Absatz „Auch mit dem Merkmal `time`…" schon nennt. Was es an
Umrechnung hinzufuegt, ist `TryFrom<time::PrimitiveDateTime>`, und eine `PrimitiveDateTime`
ist buergerliche Zeit **ohne** Zone: wer sie hat, hat die Zonenfrage bereits geloest. Gebraucht
wird das Merkmal nicht. `DateTime::from_date_and_time` und `FileOptions::last_modified_time`
tragen **kein** `cfg` und standen mit dem damaligen Merkmalssatz schon zur Verfuegung. Das
Merkmal ist deshalb **nicht** eingeschaltet worden.

**Vorschlag 2 traegt auf macOS nicht.** Das erweiterte Zeitfeld `0x5455` allein genuegt nicht,
weil `ditto(1)` es uebergeht. Gemessen am 260825 an fuenf von Hand gebauten Archiven ueber
dieselben zwei Zeitpunkte, einen aus der Sommerzeit und einen aus der Winterzeit, jedes mit
`/usr/bin/unzip` und `/usr/bin/ditto -x -k` ausgepackt:

| Archiv traegt | `unzip` liefert | `ditto` liefert |
|---|---|---|
| nur das MS-DOS-Feld | :44 (Zweisekundenraster) | :44, im Winter eine Stunde daneben |
| dazu `0x5455` | :45, beide richtig | :44, im Winter eine Stunde daneben |
| dazu `0x5455` und `0x5855` | :45, beide richtig | :45, beide richtig |

Die Stunde ist die Sommerzeitfalle: `ditto` rechnet die MS-DOS-Zeit mit dem **heute**
geltenden Versatz zurueck statt mit dem am Dateidatum geltenden. Es braucht also drei Felder
und nicht zwei, und `0x5855` verlangt das Merkmal `unreserved`, das als `unreserved = []`
deklariert ist und keine Abhaengigkeit einschaltet.

**Dritter Befund: `SimpleFileOptions::default()` entstand an drei Stellen** und deckte damit
jeden Eintragstyp ab — Datei, Ordner, leerer Ordner, Verknuepfung. Der Abschnitt „Betroffen"
nennt sie richtig; die Behebung musste an allen dreien greifen und nicht nur an `dateiwahl`.

---
Resolved: Das Packen setzt das Aenderungsdatum der Quelle an jedem Eintrag, dreifach: im
MS-DOS-Pflichtfeld ueber `verzeichnis::sys::ortszeit` (`localtime_r(3)`, also mit dem Versatz,
der zum Dateidatum galt) und in den zwei Zusatzfeldern `0x5455` und `0x5855`, die
Epochensekunden tragen und keine Zone brauchen. Die drei Wahlbauer heissen jetzt `dateiwahl`,
`ordnerwahl` und `verknuepfungswahl` und gehen alle durch `zeit_uebernehmen`; die Datei fragt
ihre Angaben am offenen Deskriptor, den `datei_packen` ohnehin haelt, die Verknuepfung ueber
`lstat(2)`. Ein Zeitpunkt ausserhalb von 1980 bis 2107 faellt auf `DateTime::DEFAULT` zurueck
und bekommt eine Zeile in der Abschlussliste, statt den Eintrag abzuweisen. Die im Nachtrag
vom 260825 (Schritt 3) genannte Gegenrichtung ist mit erledigt: `operation::entpacken` liest
`0x5455`, hilfsweise `0x5855`, und setzt das Datum auf jede entpackte Datei und, nach dem
Befuellen, auf jeden angelegten Ordner. Das MS-DOS-Feld wird dabei bewusst nicht gelesen, denn
der Weg zurueck braucht `mktime(3)`. Fuenf Proben in `tests/operation.rs` halten das Ergebnis,
darunter der Rundweg und je ein Zeitpunkt aus Sommer- und Winterzeit; am Bauziel gegen
`/usr/bin/unzip` und `/usr/bin/ditto` nachgemessen, beide liefern die Sekunde des Quelldatums.
Offen bleibt die Verknuepfung auf der Entpackseite, als eigener Datensatz
`shared/issues/260825-1859_*_eine-entpackte-verknuepfung-bekommt-ihr-aenderungsdatum-nicht.md`:
ihre Zeit setzte allein `lutimes(2)`, eine siebte Schnittstelle der Systemschicht.
