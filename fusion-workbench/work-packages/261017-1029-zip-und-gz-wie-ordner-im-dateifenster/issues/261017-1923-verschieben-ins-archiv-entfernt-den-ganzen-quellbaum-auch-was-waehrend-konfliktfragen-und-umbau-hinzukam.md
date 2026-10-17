Verschieben ins Archiv entfernt den ganzen Quellbaum, auch was während Konfliktfragen und Umbau hinzukam
---
Beim Verschieben eines lokalen oder fernen Ordners in ein Archiv wird der Baum zuerst als Wert zusammengestellt. Danach folgen alle Konfliktfragen und das Schreiben des ganzen Archivs. Zuletzt entfernt `baum_entfernen` den Quellordner rekursiv, samt allem, was inzwischen darin steht. Eine Datei, die während dieser Zeit in den Quellordner kommt, etwa ein laufender Download oder ein Sichern aus einem Editor, geht verloren, ohne im Archiv zu stehen. Der Bau nennt die Grenze im Modulkopf. Das Fenster ist hier aber breiter als bei `ueber_die_grenze`, auf das sich der Kopf beruft: es schließt jede Wartezeit des Nutzers an der Konfliktfrage ein und das Neuschreiben eines Archivs beliebiger Größe.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (Datenverlust, nur bei gleichzeitigem Schreiben in den Quellordner; Fenster von Minutenlänge)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-core/src/operation/archiv.rs`, Modulkopf, „Was dieses Modul nicht hält“: „was zwischen dem Zusammenstellen und dem Entfernen in einen Quellordner kam, fällt mit“.
- `hinein`: `Neueintrag::lokal(pfand, pfad)` stellt den Baum vor `umbau::umbauen` zusammen. Erst danach steht `stelle.baum_entfernen(pfand, &nie_abgebrochen)`.
- `crates/krk-core/src/archiv/umbau.rs`, `Neueintrag::lokal_lesen`: liest `read_dir` einmal, zur Zeit der Zusammenstellung.
- `crates/krk-core/src/archiv/umbau.rs`, `Aufstellung::ziel_klaeren`: die Konfliktfragen warten über `Steuerung::konflikt_loesen_mit` auf den Nutzer. Das geschieht nach der Zusammenstellung und vor dem Entfernen.
- Dieselbe Rekursion trifft eine Datei, die nach dem Schreiben ihrer Daten ins neue Archiv und vor dem Entfernen geändert wird. Ihre neue Fassung steht dann nirgends.

**Behebung (Richtung)**

Entfernt wird allein, was zusammengestellt wurde. Jede Datei wird einzeln entfernt, möglichst nur, wenn Gerät, Inode, Größe und Änderungszeit noch die beim Lesen sind. Danach wird jeder Ordner mit `rmdir` entfernt, und das scheitert an einem Ordner, in den etwas hinzukam. Was stehen bleibt, kommt mit Grund in den Bericht. Dieselbe Regel passt auch für `ueber_die_grenze` und schlösse die Grenze dort mit.

**Abnahme**

Eine Probe verschiebt einen lokalen Ordner in ein Archiv und legt an der Konfliktfrage eines Eintrags eine neue Datei in den Quellordner. Danach steht die neue Datei weiter im Quellordner, der Ordner selbst bleibt stehen, und der Bericht nennt ihn.

---
Resolved: fuer lokale Quellen. `crates/krk-core/src/archiv/umbau.rs`: `Neueintrag::lokal_mit_spur` liefert je Eintrag eine `Lokalspur` (Pfad, Ordner ja/nein, Geraet, Inode, Groesse, Aenderungszeit beim Zusammenstellen). `crates/krk-core/src/operation/archiv.rs`, `lokal_entfernen` (Herkunft `Lokal`) entfernt danach Kinder vor ihrem Ordner, eine Datei nur bei unveraenderter Kennung, einen Ordner mit `rmdir(2)`; was bleibt, nennt der Bericht (`Text::VorgangQuelleSeitDemLesenVeraendert` neu in allen drei Tabellen). Eine ferne Quelle wird weiter als Baum entfernt, so steht es im Modulkopf unter „Was dieses Modul nicht haelt“. Probe `verschieben_ins_archiv_laesst_stehen_was_waehrend_der_konfliktfrage_hinzukam` in `crates/krk-core/tests/archiv_operation.rs` (alle drei Formate; ohne Behebung rot).
