Anlegen und Umbenennen im lokalen Ordner stellen ihren Systemaufruf auf dem Hauptfaden
---
`Anwendungsdelegierter::anlegen_ausfuehren` ruft für einen lokalen Ordner `operation::ordner_anlegen` oder `operation::datei_anlegen` unmittelbar, und `umbenennen_ausfuehren` ruft `operation::umbenennen` ebenso (`crates/krk-ui/src/appkit/anwendung.rs`). Beide laufen im Abschlussblock eines Blattes oder einer Zelle, also auf dem Hauptfaden. Liegt der Ordner auf einer hängenden Freigabe, steht die Ereignisschleife, bis das System antwortet.
---
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>

Gefunden beim Planen von `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`. Der Plan baut auf diesem Weg auf: „Leere Datei anlegen“ in `minutes/` und der neue Befehl auf `cmd+m` enden beide in `anlegen_ausfuehren`. Der Plan fügt dort keine Frage an die Platte hinzu, nimmt die vorhandene aber auch nicht weg; ein Notizordner auf einer hängenden Freigabe trifft sie nach `cmd+m` mit dem Bestätigen des Namens.

Die Regel, gegen die der Weg steht: `.claude/rules/coding-was-man-nicht-sieht.md`, Absatz „Eine Dateiadresse entsteht im Betriebscode der Oberfläche allein über `fileURLWithPath:isDirectory:` …“. Der Absatz nennt die Reste, die weiter auf dem Hauptfaden fragen, mit Namen; diese zwei stehen nicht darunter. Das Paket `261102-1304-dateisystemfragen-vom-hauptfaden-nehmen` führt sie in keinem seiner Defekte (durchsucht am 261111 über die Namen `datei_anlegen`, `ordner_anlegen` und `anlegen_ausfuehren`).

Beleg: `crates/krk-ui/src/appkit/anwendung.rs`, Rumpf von `anlegen_ausfuehren` (Zweig `Dateiort::Lokal`) und Rumpf von `umbenennen_ausfuehren`. Der ferne Zweig daneben (`fern_anlegen`, `fern_umbenennen`) geht über einen Arbeitsfaden und ist nicht betroffen.

Abnahme: der lokale Zweig beider Funktionen stellt seinen Systemaufruf über `nebenfaden::fragen_lassen` oder über die Operationsmaschine, die Antwort wird gegen den Stand der Frage gehalten, und eine Quelltextprobe nach dem Muster von `f2_stellt_auf_dem_hauptfaden_keinen_systemaufruf` hält die zwei Rümpfe. Oder der Nutzer entscheidet, dass die zwei als benannte Reste in der Regeldatei stehen.
