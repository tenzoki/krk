# Das Kontextmenü der Dateiliste trägt „Auswerfen“ für ein DMG oder ein anderes Volume

## Directive

Der Auftrag des Nutzers, wörtlich: „mache eine neues arbeitspaket mit einer aufgaben im rechte-maustaste-menu in der dateiliste: - auswerfen: eines dmg oder eines anderen volumes“. Erreicht ist das, wenn ein Rechtsklick in der Dateiliste auf ein eingehängtes Volume (ein geöffnetes DMG, ein USB-Stick, eine Netzfreigabe oder ein anderes auswerfbares Laufwerk) einen Eintrag „Auswerfen“ anbietet, der das Volume auswirft, und der Eintrag bei einem gewöhnlichen Ordner oder einer Datei nicht wirkt. Ein Fehlschlag, etwa weil eine Datei auf dem Volume noch offen ist, wird in der Statuszeile gemeldet.

Zur Ausführung, wörtlich: „wenn beide pakete angelegt sind, übernimm das erste und führe es im automode aus. committe danach, pushe und schließe das paket.“

## Schließungsnotiz

Geschlossen am 261016 als `done`, Commit `bb2f861` (Bereich `bb2f861~1..bb2f861`). „Auswerfen“ steht im Kontextmenü der Dateiliste; `make check` und `cargo xtask bundle` liefen grün, die Durchsicht `261016-0253-reviewer-auswerfen-im-kontextmenue.md` urteilt accept. Kein Plan gebunden, also keine Haltebedingungen zu verlesen. Offen bleiben zwei kleine Befunde, `261016-0253-ein-zweiter-klick-auf-auswerfen-waehrend-des-laufenden-auswurfs-schickt-einen-zweiten-auftrag.md` und `261016-0253-der-modulkopf-von-appkit-nennt-finder-das-eine-modul-ohne-rueckgabe-auswurf-ist-ein-zweites.md`, und die Abnahme am Bündel mit einem echten DMG oder Stick, die Nutzerarbeit ist; ungeklärt ist dabei, ob macOS bei einem Auswurf über `NSFileManager` die Ankündigung des Aushängens schickt.
