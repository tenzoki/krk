# Zip- und gz-Dateien lassen sich im Dateifenster wie ein Ordner betreten und durchblättern

## Directive

Der Nutzer, wörtlich: „es müsste noch ein paket geben, bei dem es darum ging, zip files genauso wie das dateisystem zu navigieren. - wenn ich jetzt auf eine zip gehe und dann pfeil rechts drücke oder anschließend pfeil links, stürtst die app ab, inhalte werden nicht angezeigt. - außer zip brauche ich auch gzip (gz) dateien“. Ein solches Paket gab es nicht; der Absturz wird getrennt als Defekt behoben.

Erreicht ist das, wenn das Betreten einer `.zip`-Datei im Dateifenster (Pfeil rechts, Doppelklick, Return wie bei einem Ordner) ihren Inhalt als Ordnerliste zeigt, der Nutzer darin in Unterordner hinein und mit Pfeil links wieder hinaus bis in den Ordner der Zip-Datei navigieren kann, und Einträge daraus in ein lokales oder entferntes Dateifenster kopiert werden können wie aus einem Ordner. Eine `.gz`-Datei (auch `.tar.gz`/`.tgz`, soweit ohne C-Bibliothek machbar) lässt sich ebenso betreten. Auf die Frage, ob Hineinkopieren, Umbenennen und Löschen gleich mitkommen, wörtlich: „auch schreiben“. Erreicht ist das Paket deshalb erst, wenn auch Kopieren und Verschieben in ein Archiv hinein, Umbenennen, neuer Ordner und Löschen darin funktionieren, ohne dass ein Fehlschlag das Archiv beschädigt.

Zur Ausführung, auf die Frage „Soll ich dieses Paket wieder selbständig durchziehen …?“, wörtlich: „ja“.

Auf drei Fragen nach dem Plan, wörtlich: „1:nicht in krk, 2:nur lesen, 3:nicht messen“. Also: `return` auf einem Archiv bleibt „Mit dem Standardprogramm öffnen“; eine einzelne `.gz` bleibt nur lesbar; kein Abnahmelauf gegen die zehn Zusagen.

## Stand am 261017 abends

Gebaut sind alle acht Schritte des Plans `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md` (`9d0afb5b..55a87679`), dazu drei Behebungsrunden nach zwei Durchsichten (`b79eebd2`, `c9651443`, `46192d60`); alle 15 Befunde sind geschlossen, `make check` und Bündelbau grün. Das Paket bleibt offen (`claimed`), bis der Nutzer am Bündel mit eigenem Zip, `.tar.gz` und `.gz` getestet hat; der Plan macht eine Auslieferung davon abhängig. Ein tar, an das mit `tar -r` angehängt wurde, ist wegen möglicher verdeckter Einträge nur lesbar (`261017-1923-darf-jeder-umbau-die-verdeckten-eintraege-eines-archivs-verwerfen.md`).

## Schließungsnotiz

Geschlossen am 261018 als `done` auf das Wort des Nutzers („ok, dann kannst du das abschließen“). Commits `9d0afb5b..46192d60`, dazu die Vorschau im Archiv `e7397d56`; ausgeliefert in 3.6.0, 3.7.0 und 3.8.0. Zwei Durchsichten, alle 15 Befunde geschlossen; die letzte (`261017-2202-reviewer-nachdurchsicht-archivpaket-8c8579af-bis-c9651443.md`) urteilte revise, ihre zwei Befunde behebt `46192d60` ohne erneute Durchsicht. Die Abschlussmeldung bindet keinen Beleg, weil das Narrativ nach dem Beleg ergänzt wurde. Einen ausdrücklichen Testbericht des Nutzers zu den Archiven gibt es nicht. Auf den Abnahmelauf L1 bis L10 hat der Nutzer verzichtet (`261017-1105-schuldet-das-zip-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md`).

Die Haltebedingungen des Plans wurden unter dem selbständigen Modus nicht vorgelegt und stehen hier wörtlich:


- Die Schritte 1 bis 8 sind abgeschlossen, und für jeden ist seine Akzeptanz am Baum erfüllt, Grundabnahme eingeschlossen.
- Der Defekt zum Absturz bei Pfeil rechts und links auf einer Zip-Datei ist behoben und committet, bevor Schritt 2 beginnt.
- Der Entscheid `261017-1036-liest-und-schreibt-krk-tar-ueber-die-kiste-tar-oder-ueber-einen-eigenen-leser.md` ist beantwortet, bevor Schritt 1 beginnt, und `261017-1036-wird-ein-archiv-ein-dritter-fall-von-ort-und-schreibt-krk-hinein-ueber-einen-umbau-des-ganzen-archivs.md` ist beantwortet, bevor Schritt 2 beginnt.
- `cargo tree` je Mac-Ziel führt nach Schritt 1 weder `cc` noch ein Paket mit einem Namen auf `-sys`, und `Cargo.lock` führt gegenüber dem Stand vor Schritt 1 allein `tar` als neues Paket. Trifft das nicht zu, ist der Entscheid zu tar mit dem Beleg neu vorgelegt und beantwortet, bevor ein weiterer Schritt beginnt.
- `ein_abgebrochener_oder_gescheiterter_umbau_laesst_das_archiv_byte_fuer_byte_stehen` ist für Zip, Tar und TarGz grün.
- **Nutzerarbeit: Der Test am Bündel ist gefahren, und sein Ergebnis steht als Datensatz im Paket.** Geprüft werden dabei mit einem eigenen Zip, einem `.tar.gz` und einer `.gz`-Datei:
  - Betreten mit Pfeil rechts und Doppelklick, Navigation hinein und mit Pfeil links hinaus bis zur gewählten Archivdatei;
  - Herauskopieren in ein lokales Fenster und, falls die Verbindung zur Hand ist, in ein fernes;
  - Hineinkopieren und Verschieben hinein aus dem anderen Fenster und aus dem Finder, die Konfliktfrage und ein Abbruch mitten in einer großen Datei, danach ist das Archiv unverändert;
  - Umbenennen, neuer Ordner und endgültiges Löschen samt Rückfrage;
  - dass `/usr/bin/unzip` oder das Archivprogramm des Systems das geänderte Archiv öffnet.

  Ja oder nein: Der Datensatz liegt vor, und jede gescheiterte Prüfung hat einen Defekt im Paket.
- Der Nutzer hat über den Abnahmelauf L1 bis L10 aus C8 entschieden: Der Lauf ist am Bündel im Vordergrund gefahren und alle zehn halten, oder ein Entscheid in diesem Paket verzichtet ausdrücklich darauf (E19). Diese Bedingung hält keinen Schritt an, nur das Schließen des Pakets.
- Ein Auslieferungslauf, der diese Arbeit enthält, setzt voraus, dass der Test am Bündel bestanden ist.

