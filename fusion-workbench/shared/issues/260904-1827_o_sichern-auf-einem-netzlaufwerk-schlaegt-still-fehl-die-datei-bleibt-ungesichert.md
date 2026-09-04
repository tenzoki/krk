Sichern auf einem Netzlaufwerk schlägt still fehl, die Datei bleibt ungesichert

---
Der Nutzer hat eine Datei auf Google Drive im eingebauten Editor geändert und gesichert. Ergebnis: **keine Fehlermeldung, der Editor führt die Datei weiter als ungesichert, und der Stand steht nicht auf der Platte.**

Der Ort ist `~/Library/CloudStorage/GoogleDrive-<konto>`, also eine FileProvider-Einhängung und kein gewöhnliches Dateisystem.

**Warum das doppelt schwer wiegt.** Der Editor hält für diesen Fall den Ausgang `Sicherungsausgang::Gescheitert(String)` bereit, dessen Doc-Kommentar zusagt: „Der Grund gehört in die Statuszeile." Genau das ist nicht geschehen. Dass die Datei weiter als ungesichert dasteht, spricht dafür, dass der Editor den Fehlschlag **kennt** — er hat nicht auf `Gesichert` umgeschaltet — und ihn trotzdem nicht meldet. Ein stiller Fehlschlag ist in diesem Projekt ausdrücklich verboten (`HYG-NO-SILENT-FAIL`), und hier kostet er Nutzerarbeit: wer die Meldung nicht sieht, hält die Datei für gesichert und schließt sie.

**Der Weg, der zu untersuchen ist:** `text::datei::sichern` (`crates/krk-core/src/text/datei.rs:893`) ruft `ablage::atomar::schreiben`, das erst in eine Nachbardatei schreibt und dann `rename(2)` auf das Ziel absetzt. `rename` ist innerhalb eines Dateisystems unteilbar; über eine Dateisystemgrenze scheitert es mit `EXDEV`, und eine FileProvider-Einhängung kann sich in dieser Frage anders verhalten als eine Platte. Ob der Fehler dort entsteht, ob er auf dem Weg nach oben verlorengeht oder ob `schreiben` fälschlich `Ok` liefert, ist offen und gehört gemessen.

**Abnahmetest:** eine Datei unter `~/Library/CloudStorage/GoogleDrive-<konto>` im Editor ändern und sichern. Entweder steht der neue Stand danach auf der Platte und der Editor führt die Datei nicht mehr als ungesichert — oder die Statuszeile nennt den Grund, und die Datei bleibt sichtbar ungesichert. Ein dritter Ausgang, in dem nichts geschieht und nichts gesagt wird, ist der Defekt.

---
**Filed by:** user, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gemeldet am 260904 aus dem laufenden Programm, Fassung v1.6.0. Der Ort ist auf diesem Gerät vorhanden und der Fall damit reproduzierbar.
