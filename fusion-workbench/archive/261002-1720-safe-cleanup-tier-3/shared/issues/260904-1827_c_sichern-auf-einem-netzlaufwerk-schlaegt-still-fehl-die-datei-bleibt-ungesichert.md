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

---
Resolved: Das Netzlaufwerk war es nicht. Der Schreibweg trägt, dreimal gefahren, kein
`errno` (`260904-1905-bugfix-sichern-auf-google-drive.md`, drei Läufe
gegen `~/Library/CloudStorage/GoogleDrive-…`, darunter der Ordner des Nutzerfalls);
`nachbarpfad`, `File::create`, `rename(2)` und `text::datei::sichern` gehen dort alle
vier durch, und der neue Inhalt stand danach unverändert auf der Platte. Der beobachtete
Ausgang — kein Wort, kein Schreiben — ist mit keinem der drei Ausgänge von
`Editormodell::sichern` vereinbar, denn alle drei enden in der Statuszeile. Er ist allein
damit vereinbar, dass `sichern` gar nicht gelaufen ist, und der Nutzer hat am 260904
bestätigt, was das bedeutet: **es stand ein Blatt offen.** Der Zulässigkeitsvorbehalt am
Kopf von `Anwendungsdelegierter::kommando_ausfuehren` hat `cmd+s` abgewiesen und dazu
nichts gesagt. Behoben ist deshalb nicht der Schreibweg, sondern das Schweigen der
Blattsperre: `crates/krk-ui/src/kommandos/blattmeldung.rs` ist die eine Regel, die
entscheidet, ob eine Abweisung einen Satz bekommt, und `kommando_ausfuehren` stellt ihn
als Befehlsantwort in die Statuszeile. Ein abgewiesenes `cmd+s` meldet seitdem „nicht
ausgeführt: über dem Fenster steht ein Blatt". Was ein Blatt bedient — `tab`, `space`,
`return`, `esc` und der Pfeilblock samt `pageup`, `pagedown`, `home` und `end` — schweigt
weiter, und der Menüweg ebenso, weil dort die Ausgrauung die Antwort ist.
Offen bleibt daneben `260904-2047_*_wohin-geht-die-blattmeldung-wenn-das-blatt-die-statuszeile-verdeckt.md`:
beim Anfangsmaß des Fensters ist die Statuszeile neben jedem Blatt dieses Baums sichtbar,
bei einem klein gezogenen Fenster nicht.
