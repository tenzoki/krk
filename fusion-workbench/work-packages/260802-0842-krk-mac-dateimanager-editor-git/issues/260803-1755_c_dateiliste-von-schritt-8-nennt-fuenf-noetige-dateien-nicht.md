Die Dateiliste von Schritt 8 nennt fünf nötige Dateien nicht

---

Schritt 8 des Plans nennt neun Dateien. Gebraucht wurden vierzehn. Die fünf fehlenden sind keine Bequemlichkeit, sondern jede für sich eine Voraussetzung dafür, dass der Schritt überhaupt eine Zahl liefert. Dieselbe Auslassung hat Schritt 7 schon einmal getroffen, Defekt `260803-1309_o_dateiliste-von-schritt-7-nennt-fuenf-noetige-dateien-nicht.md`; die Nachzugsrunde vom 260802-1859 hat 35 einbindende Dateien ergänzt, aber nur die einbindenden.

---

| Datei | Wofür |
|---|---|
| `crates/krk-ui/src/appkit/tabelle.rs` | Drei nur lesende Zugriffe auf die Datenquelle: `zeilen()`, `liest_noch()`, `auswahlzeile()`. Ohne sie kann der Messmodus an einer Bildgrenze nicht feststellen, ob die erste Bildschirmseite steht, ob der Lesevorgang fertig ist oder ob die Auswahl umgesprungen ist — also für keine der vier in der Anwendung gemessenen Zusagen das Ende der Spanne bestimmen. |
| `crates/krk-ui/src/appkit/ereignisse.rs` | Das synthetische Tastenereignis für L1 (`pfeil_ab_senden`). Der Plan weist die synthetischen Ereignisse in S21 ausdrücklich diesem Modul zu ("Die synthetischen Ereignisse gehören zum Ereignisabgriff"), verlangt aber schon in S8 die Messung von L1 über einen Tastendruck. Ohne diese Datei ist L1 in S8 nur mit einem Menschen an der Tastatur messbar, und dann nicht zwanzigmal reproduzierbar. |
| `crates/krk-bench/src/bericht.rs` | Drei Hilfsfunktionen (`befehl_ausgabe`, `betriebssystem`, `bauart`) mussten von privat auf sichtbar gestellt werden, damit der neue Bericht den Bedingungskopf nicht ein zweites Mal nachbaut. Der Plan nennt diese Datei erst in S21. |
| `crates/krk-bench/src/main.rs` | Der Unterbefehl `durchstich`. Der Plan nennt für S8 nur `messen.rs`; ein Unterbefehl, den die Befehlszeile nicht kennt, ist nicht aufrufbar. |
| `Cargo.lock` | Mechanisch, durch die neue Abhängigkeit `objc2-quartz-core`. Die Datei ist versioniert. |

Die Regel, an der die Liste hängt, steht seit dem Nachzug vom 260802-1859 im Plan: ein Schritt muss auch die Datei anfassen, die sein neues Modul oder seine neue Abhängigkeit einbindet. Sie deckt die einbindenden Dateien ab und nicht die, an denen ein neues Modul ablesen muss, was in einem alten steht. Ein Nachzug über alle Schritte lohnt unter der erweiterten Regel: **auch die Datei, aus der ein Schritt etwas lesen oder auslösen muss, gehört in seine Liste.**

---
Resolved: Die Dateiliste von S8 nennt jetzt alle vierzehn Dateien; die fünf fehlenden tragen den Vermerk `(lesend)` beziehungsweise `(erweitert)`, und die `Cargo.lock` steht mit ihrem mechanischen Grund dabei. Die erweiterte Regel steht als eigener Absatz im Kopf von `## Implementierungsschritte` des Plans `260802-1428_o_plan-navigator-geruest-runde-1.md` und bindet damit jeden künftigen Schritt. Der Nachzug über die Listen von S9 bis S23 ist nicht Teil dieser Behebung und steht als eigener Defekt: `260803-1819_o_dateilisten-von-s9-bis-s23-noch-nicht-unter-der-erweiterten-regel-durchgegangen.md`. Behoben am 260803-1819 vom `planner`.
