`HowTo.md` zerreißt die Tabelle „Was dabei schiefgehen kann“ mit einem eingeschobenen Absatz
---
`0272a86` hat den Absatz „Diese drei Lagen prüft KRK zweimal …“ zwischen die vorletzte und die letzte Zeile der Tabelle im Abschnitt „Eine Verbindung anlegen“ gesetzt. Die letzte Zeile, „Der Eintrag steht, das Lesezeichen ließ sich nicht anlegen“, steht damit ohne Tabellenkopf direkt unter dem Absatz und wird als dessen Fortsetzung gesetzt, also als Fließtext mit Pipes. `HowTo.md` reist im Releasepaket mit.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** 261016-0614-verbinden-prueft-die-schreibgangsperre-nur-vor-dem-blatt-und-sieht-einen-laufenden-ladevorgang-des-editors-nicht.md

## Defekt

Auf `0272a86`, `HowTo.md`, Abschnitt „Eine Verbindung anlegen“:

```
| `secrets.txt` wird im Editor gerade geöffnet | „Verbinden…“ weist ab: erst warten, bis sie offen ist. |

Diese drei Lagen prüft KRK zweimal: bevor das Blatt aufgeht und noch einmal
nach „Anlegen“ und der PIN, unmittelbar vor dem Schreiben. Ist der Editor in
der Zwischenzeit in eine davon geraten, schreibt KRK nichts und nennt den
Grund in der Statuszeile; die Felder des Blattes sind dann verloren.
| Der Eintrag steht, das Lesezeichen ließ sich nicht anlegen | Die Statuszeile sagt genau das. |
```

Die Leerzeile beendet die Tabelle. Eine Pipe-Zeile direkt nach einem Absatz ohne Leerzeile ist in CommonMark und GFM eine Fortsetzungszeile des Absatzes und keine Tabelle. Die Lage „Lesezeichen fehlt“ verschwindet damit aus der Tabelle.

Am Rand: „Diese drei Lagen“ meint die drei Zeilen zur Schreibgangsperre (ungesichert, PIN nie gesichert, wird geöffnet). Über einer Tabelle mit sieben Lagen steht das nicht da. Steht der Absatz unter der vollständigen Tabelle, hilft eine Nennung der drei.

## Richtung der Behebung

Die Zeile „Der Eintrag steht, das Lesezeichen ließ sich nicht anlegen“ zurück in die Tabelle, direkt nach „wird im Editor gerade geöffnet“. Den Absatz danach, mit einer Leerzeile davor. Im Absatz die drei Lagen nennen statt „diese drei“.

## Akzeptanz

- Die Tabelle im Abschnitt „Eine Verbindung anlegen“ trägt unter Kopf und Trenner alle sieben Lagen ohne Unterbrechung, die letzte „Der Eintrag steht, das Lesezeichen ließ sich nicht anlegen“.
- Zwischen dem Absatz zur zweiten Prüfung und jeder Tabellenzeile steht eine Leerzeile.

---

Resolved: In `HowTo.md` unter „Eine Verbindung anlegen“ steht die Zeile „Der Eintrag steht, das Lesezeichen ließ sich nicht anlegen“ wieder als letzte Zeile der geschlossenen Tabelle, und der Absatz über die zweifache Prüfung folgt dahinter, nun als „Die drei Lagen des Editors aus der Tabelle …“, weil hinter der Tabelle sieben Zeilen stehen. Eine Suche nach Tabellenzeilen ohne Kopf und Trennzeile über `HowTo.md` und `README.md` findet keine weitere zerrissene Tabelle. Nicht committet.
