Eine Textmarke in `.secrets.txt` schreibt eine Klartextzeile in die Lesezeichendatei

---

Seit Schritt 5.4b öffnet sich `.secrets.txt` mit der PIN im Editor. Steht der Fokus dort, legt der Lesezeichenbefehl eine Textmarke an: `Anwendungsdelegierter::anlegeziel` (`crates/krk-ui/src/appkit/anwendung.rs`) baut `Ziel::Textstelle { datei, zeile, zeileninhalt }` aus `Editorbereich::schreibmarkenzeile`, und `zeileninhalt` ist die Zeile des gehaltenen Klartexts. Die Lesezeichenliste geht als Ablagedatei auf die Platte (`krk_core::ablage::lesezeichen`, Feldtrio `datei`, `zeile`, `zeileninhalt`). Damit steht eine Zeile der Geheimnisse im Klartext unter `~/Library/Application Support/KRK/`, und C7.8 des Spec sagt zu: „Klartext der Geheimnisse gelangt an keine Stelle auf der Platte“.

---
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>

**Am Code gelesen, nicht am Bündel ausgelöst.** Der Weg ist in der Rohansicht sicher offen, weil dort die Textfläche mit Schreibmarke steht; ob die Formatansicht (Tabelle) über `schreibmarkenzeile` ebenfalls eine Zeile liefert, ist nicht geprüft. Vor 5.4b war der Weg nicht erreichbar, weil `.secrets.txt` sich gar nicht öffnen ließ.

**Warum nicht mitbehoben.** Der Auftrag war das PIN-Blatt und der Weg in den Editor; die Lesezeichenablage liegt außerhalb der Dateien des Schritts. Zwei naheliegende Antworten, zwischen denen nicht entschieden ist: `anlegeziel` weist für `.secrets.txt` im erkannten Ordner ab (Satz in die Statuszeile), oder die Textmarke entsteht ohne `zeileninhalt` für diese eine Datei. Die erste ist die kleinere und hält die Zusage ohne neue Form im Kern. Nutzerprüfung nach der Behebung: `.secrets.txt` öffnen, in der Rohansicht eine Textmarke setzen, `grep` über die Lesezeichendatei nach dem Text der Zeile.
