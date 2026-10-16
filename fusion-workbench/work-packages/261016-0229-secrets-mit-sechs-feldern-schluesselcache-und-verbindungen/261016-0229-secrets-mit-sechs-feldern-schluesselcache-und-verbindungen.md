# secrets.txt mit sechs Feldern, zehn Minuten gemerkter Schlüssel und Verbindungen als Lesezeichen

## Directive

Der Auftrag des Nutzers, wörtlich:

> und ein weiteres paket:
> - mit einer änderung der home/secrets funktion: statt zwei spalten wie notes und tasks, braucht secrets: name, protocol, address, account, password, notes
> - einer weiteren erweiterung der secrets funktion dahin, dass der einmal eingegebene key für secrets für 10 min. in memory gecachte bleibt.
> - einer neuen rechte-maustaste funktion verbinden, die name, protocol, address, account, password, und notes abfragt in einem dialogfenster, dann ein neues lesezeichen unter einer neu einzurichtenden gruppe 'Verbindungen' anlegt mit dem eingegebenen namen.
> - einer erweiterung der lesezeichen funktion für die Verbindungen: beim klicken auf eine verbindung wird die verbindung hergestellt, die daten werden aus der secrets gelesen, entweder mit abfrage des keys oder des gecachten keys wenn noch valide.

Erreicht ist das, wenn vier Dinge gelten. Erstens zeigt die Geheimnistabelle des Heimordners (`secrets.txt`) sechs Spalten, Name, Protokoll, Adresse, Konto, Passwort und Notizen, statt der zwei, die Notizen und Aufgaben führen. Zweitens fragt KRK nach einmal eingegebenem Schlüssel zehn Minuten lang nicht erneut; der Schlüssel liegt dabei allein im Arbeitsspeicher und nie auf der Platte. Drittens bietet das Kontextmenü der Dateiliste „Verbinden…“ an: ein Dialog fragt die sechs Felder ab, legt den Eintrag in `secrets.txt` an und ein Lesezeichen mit dem eingegebenen Namen in einer neuen Gruppe „Verbindungen“. Viertens stellt ein Klick auf ein solches Lesezeichen die Verbindung her; die Zugangsdaten kommen aus `secrets.txt`, mit Abfrage des Schlüssels oder, solange er noch gilt, mit dem gemerkten.

Zur Ausführung, wörtlich: „nimm anschließend das andere paket auf, führe es im automode durch, committe alles, mache eine volles release und schließe das paket und die session.“

## Schließungsnotiz

Geschlossen am 261016 als `done`, Commits `ea6dac2..3835e2b` (Bereich `e57b5c7..3835e2b`), selbständig ausgeführt auf das Wort des Nutzers. Alle zehn Schritte des Plans `261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md` sind gebaut; `make check`, `cargo xtask bundle` und `cargo tree` je Mac-Ziel liefen grün. Drei Durchsichten (`261016-0532-reviewer-secrets-sechs-felder-schluesselmerker-verbindungen.md`, `261016-0614-reviewer-nachdurchsicht-der-vier-behebungen-an-verbindungen-und-tilgen.md`, `261016-0638-reviewer-schlussdurchsicht-der-drei-behebungen-vor-der-auslieferung.md`) fanden acht Befunde, alle behoben; die letzte urteilte revise allein wegen einer zerrissenen Tabelle in `HowTo.md`, die `3835e2b` schließt, zusammen mit der Restfrage der zweiten PIN beim Öffnen während eines Schreibgangs. Die drei Entscheidungen des Pakets sind umgesetzt.

Die Haltebedingungen des Plans wurden dem Nutzer unter dem selbständigen Modus nicht vorgelegt und stehen hier wörtlich:

- Jeder der zehn Schritte ist in der Steuerdatei des Plans auf `done`, und `make check` ist auf dem Stand nach Schritt 10 grün.
- `cargo xtask bundle` baut und signiert `target/KRK.app` auf diesem Stand.
- `cargo tree --target <ziel> -e normal,build` zeigt für beide Mac-Ziele weder `cc` noch ein Paket mit einem Namen auf `-sys`.
- Die Proben aus den Akzeptanzen der Schritte 1, 2, 5, 6 und 7 stehen im Baum und laufen in `make check` mit: Ablauf an der Grenze, Formatübergang ohne Verlust, kein Klartext außer dem Namen in `bookmarks.toml`, ein Rufer von `Passwort::preisgeben`, Rufer von `Schluesselmerker::merken`, Chiffrat-Weg mit zwei benannten Rufern.
- Jede neue nutzersichtbare Zeichenkette steht in `de.rs`, `fr.rs` und `en.rs`, und die zwei Nahtproben der Sprachtabelle sind grün.
- `HowTo.md` und `README.md` beschreiben sechs Spalten, den Merker und die Verbindungen; jede Stelle aus „CLAUDE.md-Stellen, die diese Arbeit falsch macht“ ist nachgezogen.
- **Kein Abnahmelauf gegen die zehn Zusagen aus C8 ist geschuldet**, unter drei Bedingungen, deren jede eine Probe hält: der Start liest keine weitere Datei; der Hauptfaden stellt auf dem Verbindungs- und dem Anhängeweg keinen Systemaufruf außer dem einen `stat(2)` aus Entscheidung 14; der Leistenaufbau stellt für eine Verbindung keine Frage an das Dateisystem. Hält eine davon nicht, ist die Arbeit nicht fertig, und die Frage nach dem Abnahmelauf geht an den Nutzer.
- Vorbedingung jeder Auslieferung dieses Stands: alle Klauseln oben sind ja. **Nicht** Vorbedingung, sondern ausdrücklich Nutzerarbeit am laufenden Bündel, die das Paket nicht abwartet: ein echtes Einhängen einer SMB-Freigabe über ein Verbindungs-Lesezeichen, ein Ablauf des Merkers über einen Ruhezustand des Macs und die Sichtprüfung der Tabelle in drei Sprachen. Gebaut ist dann die richtige Aussage über das Paket, abgenommen nicht.

Zur siebten Klausel ist festzuhalten: der Hauptfaden stellt neben dem einen `stat(2)` zwei benannte Ausnahmen, das Anlegen einer fehlenden `secrets.txt` über den Weg von F2 und das Schreiben von `bookmarks.toml` über `lesezeichen_aendern`; die Probe nennt beide beim Namen. Die Frage nach dem Abnahmelauf geht damit an den Nutzer und ist ihm am Ende der Sitzung vorgelegt. Die Zeitprobe `der_abbruch_mitten_in_einer_500_mb_datei_kehrt_binnen_100_ms_zurueck` scheiterte in dieser Sitzung mehrfach unter Last; sie ist von diesem Paket nicht berührt und als `261016-0633-die-zeitprobe-zum-abbruch-einer-500-mb-kopie-scheitert-unter-last-in-allen-fuenf-versuchen.md` erfasst. Nutzerarbeit bleibt die Abnahme am Bündel wie oben genannt.
