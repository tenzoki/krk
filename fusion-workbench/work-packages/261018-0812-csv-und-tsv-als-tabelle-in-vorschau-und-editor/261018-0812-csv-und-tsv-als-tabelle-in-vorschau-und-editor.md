# CSV- und TSV-Dateien erscheinen in Vorschau und Editor als Tabelle

## Directive

Der Nutzer, wörtlich: „Füge dem csv/tsv Support hinzu: sowohl preview wie editor sollten einfache tabellen handhaben können. editor mit insert, append, delete, copy markierten bereich. scroll muss horizontal und vertical funktionieren.“

Erreicht ist das, wenn eine `.csv`- oder `.tsv`-Datei in der Vorschau als Tabelle mit Spalten erscheint und der Editor sie als bearbeitbare Tabelle öffnet: Zeilen (und Spalten) einfügen und anhängen, Zeilen (und Spalten) löschen, einen markierten Bereich von Zellen kopieren, Zellen bearbeiten, und beides lässt sich waagerecht und senkrecht scrollen. Gesichert wird wieder als CSV bzw. TSV, ohne dass unveränderte Teile der Datei ihre Form verlieren.

Auf die Frage, ob das Paket selbständig mit den Empfehlungen des Plans durchläuft (CSV als weitere Form der Eintragstabelle mit Rechnung auf dem Text; CSV/TSV behalten beim Sichern Bytefolgenmarke und Zeilenenden; Vorschau als ausgerichteter Text; kein Abnahmelauf; die vorgeschlagenen Tasten), wörtlich: „ja“.

---

## Schlussnotiz (261019)

Geschlossen als `done`, Klasse `completed`, auf das Wort des Nutzers („1“ nach „funktioniert“). Commits 17c9ac81..733067ff, ausgeliefert mit 3.9.0 und 3.9.1. Schlussdurchsicht `reviews/261019-1122-reviewer-csv-und-tsv-schlussdurchsicht-vor-dem-schliessen.md`, Urteil `revise`; ihre Frage zur Sicherungsform hat der Nutzer mit Möglichkeit 1 beantwortet (`decisions/261019-1122-gilt-der-stand-mit-den-zeilenenden-der-datei-als-antwort-auf-die-sicherungsform-einer-csv.md`, umgesetzt in ff2d7b76), der Entscheid 261018-0822 zur Sicherungsform ist damit abgelöst.

Offen im Paket: der Defekt zur Zeitgrenze an 16 MiB (Spalte einfügen 127–130 ms) und die drei niedrigen Defekte der Schlussdurchsicht (Zeilenindex mit `\r` und Textmarken, CR-Regel der Vorschau am ersten Megabyte, Doc-Kommentar des Teilersatzes).

Abbruchbedingungen des Plans, unter `**Mode:** autonomous` nicht vorgelegt und nicht beurteilt, hier wörtlich übernommen:

1. Die Schritte 1 bis 5 und 7 sind abgeschlossen, und für jeden ist seine Akzeptanz am Baum erfüllt, Grundabnahme eingeschlossen.
2. Schritt 6 ist abgeschlossen, oder er ist geschlossen, ohne gebaut zu sein, weil der Entscheid zur Sicherungsform mit Möglichkeit 1 oder 3 beantwortet ist.
3. Der Entscheid zur Architektur ist beantwortet, bevor Schritt 3 beginnt; der Entscheid zur Vorschau, bevor Schritt 2 beginnt; der Entscheid zur Sicherungsform, bevor Schritt 6 beginnt.
4. Keine `Cargo.toml` und `Cargo.lock` hat sich gegenüber `5f29200e` geändert, und `cargo tree` je Mac-Ziel führt weder `cc` noch ein Paket mit einem Namen auf `-sys`.
5. Die Berichte der Schritte 1 bis 3 nennen die gemessenen Zeiten. Liegt die Zellübernahme an 16 MiB über 100 ms, steht ein Defekt dazu im Paket, bevor es schließt.
6. Nutzerarbeit: Der Test am Bündel ist gefahren, und sein Ergebnis steht als Datensatz im Paket. — Der Nutzer meldete am 261019 im Chat „funktioniert“; ein eigener Testdatensatz liegt nicht vor, diese Notiz hält das Wort fest.
7. Der Nutzer hat über den Abnahmelauf L1 bis L10 entschieden (Entscheid `261018-0822-schuldet-das-csv-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md`, Empfehlung angenommen).
8. Ein Auslieferungslauf, der diese Arbeit enthält, setzt voraus, dass der Test am Bündel bestanden ist. — 3.9.0 ging vor dem Wort des Nutzers aus, auf seinen Auftrag „danach release machen“.
