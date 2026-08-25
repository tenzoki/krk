# Zeitstempel beim Packen und Entpacken

**Datum:** 2026-08-25
**Agent:** coder
**Status:** Complete
**Auftrag:** Schritt 3, Strang 1 des Plans
`shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` — „Ein gepackter
Eintrag trägt das Änderungsdatum seiner Quelle, ein entpackter das des Archivs"

## Was gebaut ist

**`crates/krk-core/src/operation/zippen.rs`.** Die drei Stellen, an denen die Eintragswahl
entsteht, heißen jetzt `dateiwahl`, `ordnerwahl` und `verknuepfungswahl` und gehen alle durch
den neuen `zeit_uebernehmen`. Er trägt das Änderungsdatum dreifach ein: im MS-DOS-Pflichtfeld
über `archivzeitpunkt`, das `verzeichnis::sys::ortszeit` aus Schritt 2 aufruft, und in den zwei
Zusatzfeldern `FELD_ERWEITERTE_ZEIT` (`0x5455`) und `FELD_INFOZIP_UNIX` (`0x5855`), die
Epochensekunden tragen und keine Zeitzone brauchen. `rechte_uebernehmen` nimmt statt eines
Pfades die schon eingeholten Angaben entgegen; für eine Datei sind das die vom **offenen
Deskriptor**, den `datei_packen` ohnehin hält, für eine Verknüpfung die aus `lstat(2)`. Damit
fällt ein Systemaufruf je Datei weg.

Ein Zeitpunkt außerhalb von 1980 bis 2107 fällt auf `DateTime::DEFAULT` zurück und erzeugt genau
eine Zeile in der Abschlussliste; abgewiesen wird der Eintrag nicht.

**`crates/krk-core/src/operation/entpacken.rs`.** `eintragszeit` liest das Datum eines
Archiveintrags, zuerst aus `0x5455` über die Zerlegung der Kiste, hilfsweise aus `0x5855` über
`infozip_unix_zeit`, das die Rohbytes abschreitet, weil die Kiste dieses Feld nicht zerlegt. Das
MS-DOS-Feld wird bewusst nicht gelesen: der Weg zurück bräuchte `mktime(3)`, und eine Umrechnung
mit dem heute geltenden Versatz wäre gerade der Fehler, den `ditto(1)` macht. `zeit_setzen`
schreibt das Datum auf jede entpackte Datei; für die Ordner sammelt der neue `Ordnernachtrag`
Rechte **und** Datum, und `ordnerangaben_nachtragen` setzt beide nach dem Befüllen, tiefste
Ebene zuerst.

**`Cargo.toml`.** `zip` trägt jetzt zwei Merkmale statt einem: `deflate-flate2` und
`unreserved`. Das zweite ist als `unreserved = []` deklariert, schaltet keine Abhängigkeit ein
und lässt allein das Zusatzfeld `0x5855` zu. `Cargo.lock` ist unverändert.

**`crates/krk-core/tests/operation.rs`.** Fünf Proben und fünf Helfer dazu
(`zeitpunkt`, `datum_setzen`, `datum_lesen`, `archivzeit`, `archivzusatzfelder`,
`archivepochenzeit`): das MS-DOS-Feld trägt die Ortszeit je Zeitpunkt (Sommer und Winter),
jeder der vier Eintragstypen trägt beide Zusatzfelder, der Rundweg erhält das Datum auf die
Sekunde, ein Archiv mit allein `0x5855` gibt sein Datum her, und ein Zeitpunkt vor 1980 erzeugt
genau eine Zeile.

## Was gemessen ist

Der Plan berichtigt den Defektdatensatz an zwei Stellen, und beide Berichtigungen sind hier
nachgemessen worden, bevor eine Zeile Code entstand.

Fünf von Hand gebaute Archive über dieselben zwei Zeitpunkte, einen aus der Sommerzeit
(4. Juli 2026 14:30:45) und einen aus der Winterzeit (15. Januar 2026 10:30:45), jedes mit
`/usr/bin/unzip` und mit `/usr/bin/ditto -x -k` ausgepackt:

| Archiv trägt | `unzip` liefert | `ditto` liefert |
|---|---|---|
| nur das MS-DOS-Feld | :44 | :44, im Winter eine Stunde daneben |
| dazu `0x5455` | :45, beide richtig | :44, im Winter eine Stunde daneben |
| dazu `0x5455` und `0x5855` (12 Byte, mit Kennungen) | :45, beide richtig | :45, beide richtig, Gruppe umgesetzt |
| dazu `0x5455` und `0x5855` (8 Byte, ohne Kennungen) | :45, beide richtig | :45, beide richtig |
| nur `0x5855` | :45, beide richtig | :45, beide richtig |

Gewählt ist die vierte Zeile: die kurze Form ohne Benutzer- und Gruppenkennung. Die lange Form
liefert dasselbe Datum und setzt beim Auspacken zusätzlich die Gruppe um, und dieses Vorhaben
packt Zeitstempel und keine Eigentumsverhältnisse.

Danach am fertigen Bau nachgemessen, an einem von KRK gepackten Archiv mit Datei, Unterordner,
tiefer Datei und Verknüpfung: `/usr/bin/unzip` und `/usr/bin/ditto -x -k` legen beide
`sommer.txt` auf den 4. Juli 14:30:45 und `winter.txt` auf den 15. Januar 10:30:45 ab, also auf
die Sekunde und ohne Stundenversatz.

## Was offen bleibt

Zwei Datensätze sind entstanden, beide im gemeinsamen Speicher:

- `shared/issues/260825-1859_*_eine-entpackte-verknuepfung-bekommt-ihr-aenderungsdatum-nicht.md`
  — `File::set_times` folgt der Verknüpfung, die Zeit am Verweis selbst setzte allein
  `lutimes(2)`, eine siebte Schnittstelle der Systemschicht. `operation::kopieren` hat dieselbe
  Lücke. Damit ist eine Hälfte des dritten Abnahmekriteriums dieses Schritts nicht erfüllt, und
  der Plan trägt den Nachtrag dazu.
- `shared/issues/260825-1859_*_claude-md-nennt-fuer-zip-das-eine-merkmal-deflate-flate2-es-sind-zwei.md`
  — `CLAUDE.md:82` steht nicht in der Dateiliste dieses Schritts.

Nutzerarbeit bleibt, was `cargo test` nicht sehen kann: ein Doppelklick auf ein von KRK
gepacktes Archiv im Finder, dessen Archivierungsfunktion ein drittes Werkzeug neben `unzip` und
`ditto` ist.

Der Defektdatensatz
`circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0838_*_jeder-gepackte-eintrag-traegt-den-1-januar-1980-*`
trägt seinen Nachtrag mit den drei Messbefunden, seine `Resolved:`-Zeile und den Marker `_c_`.

## Abnahme

`make check` — Rückgabewert 0. Kein Commit; die Änderungen stehen im Arbeitsbaum.
