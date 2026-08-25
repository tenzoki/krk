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
