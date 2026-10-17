# Zip- und gz-Dateien lassen sich im Dateifenster wie ein Ordner betreten und durchblättern

## Directive

Der Nutzer, wörtlich: „es müsste noch ein paket geben, bei dem es darum ging, zip files genauso wie das dateisystem zu navigieren. - wenn ich jetzt auf eine zip gehe und dann pfeil rechts drücke oder anschließend pfeil links, stürtst die app ab, inhalte werden nicht angezeigt. - außer zip brauche ich auch gzip (gz) dateien“. Ein solches Paket gab es nicht; der Absturz wird getrennt als Defekt behoben.

Erreicht ist das, wenn das Betreten einer `.zip`-Datei im Dateifenster (Pfeil rechts, Doppelklick, Return wie bei einem Ordner) ihren Inhalt als Ordnerliste zeigt, der Nutzer darin in Unterordner hinein und mit Pfeil links wieder hinaus bis in den Ordner der Zip-Datei navigieren kann, und Einträge daraus in ein lokales oder entferntes Dateifenster kopiert werden können wie aus einem Ordner. Eine `.gz`-Datei (auch `.tar.gz`/`.tgz`, soweit ohne C-Bibliothek machbar) lässt sich ebenso betreten. Auf die Frage, ob Hineinkopieren, Umbenennen und Löschen gleich mitkommen, wörtlich: „auch schreiben“. Erreicht ist das Paket deshalb erst, wenn auch Kopieren und Verschieben in ein Archiv hinein, Umbenennen, neuer Ordner und Löschen darin funktionieren, ohne dass ein Fehlschlag das Archiv beschädigt.

Zur Ausführung, auf die Frage „Soll ich dieses Paket wieder selbständig durchziehen …?“, wörtlich: „ja“.

Auf drei Fragen nach dem Plan, wörtlich: „1:nicht in krk, 2:nur lesen, 3:nicht messen“. Also: `return` auf einem Archiv bleibt „Mit dem Standardprogramm öffnen“; eine einzelne `.gz` bleibt nur lesbar; kein Abnahmelauf gegen die zehn Zusagen.

## Stand am 261017 abends

Gebaut sind alle acht Schritte des Plans `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md` (`9d0afb5b..55a87679`), dazu drei Behebungsrunden nach zwei Durchsichten (`b79eebd2`, `c9651443`, `46192d60`); alle 15 Befunde sind geschlossen, `make check` und Bündelbau grün. Das Paket bleibt offen (`claimed`), bis der Nutzer am Bündel mit eigenem Zip, `.tar.gz` und `.gz` getestet hat; der Plan macht eine Auslieferung davon abhängig. Ein tar, an das mit `tar -r` angehängt wurde, ist wegen möglicher verdeckter Einträge nur lesbar (`261017-1923-darf-jeder-umbau-die-verdeckten-eintraege-eines-archivs-verwerfen.md`).
