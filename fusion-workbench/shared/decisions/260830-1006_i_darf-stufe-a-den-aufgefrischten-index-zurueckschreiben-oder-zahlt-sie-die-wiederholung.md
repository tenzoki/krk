# Darf Stufe A den aufgefrischten Index zurückschreiben, oder zahlt sie die Wiederholung?

---
**Domain:** code
**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md` (Frage 4); `crates/krk-bench/src/messen.rs:1140` (L3), `:1213` (L10)

---

## Question

Die Stufe A liest und schreibt nichts ins Repository. `git status` schreibt trotzdem: es frischt den Stat-Zwischenspeicher im Index auf, damit der nächste Lauf die unveränderten Dateien nicht ein zweites Mal einlesen muss. `gix` bietet dasselbe an: der Statusstrom meldet die betroffenen Einträge als `EntryStatus::NeedsUpdate`, und `Outcome::write_changes()` schreibt sie zurück.

Wer nicht zurückschreibt, zahlt die Auffrischung bei jeder Abfrage erneut, und in KRK heißt das: bei jedem Ordnerwechsel. Wer zurückschreibt, ändert eine Datei im Repository des Nutzers, ohne dass der Nutzer einen schreibenden Befehl gegeben hätte, in einem Vorhaben, dessen Stufe A ausdrücklich nur liest.

Zu entscheiden ist die Frage vor dem Plan, weil sie den Umfang der Stufe A berührt und weil `write_changes` einen Schreibweg samt Sperre und Fehlerbehandlung nach sich zieht, den die Runde sonst gar nicht braucht.

## Options

1. **Nicht zurückschreiben, Stufe A bleibt schreibfrei** — `NeedsUpdate` wird gelesen und verworfen.
   - Pros: die Stufe hält ihre Zusage buchstäblich; kein Schreibweg, keine Sperre, kein Konflikt mit einem gleichzeitig laufenden `git`; die Runde bleibt klein.
   - Cons: die Auffrischung fällt bei jedem Ordnerwechsel neu an. Wie hoch der Posten ist, ist **ungemessen**: in den vier Prüfbäumen war `NeedsUpdate` null, weil `git status` unmittelbar davor gelaufen war. Er entsteht, wenn Zeitstempel angefasst wurden, also gerade nach einem Auschecken, einem Baulauf oder einer Kopieroperation von KRK selbst.
2. **Zurückschreiben, wenn der Nutzer es einschaltet** — eine Einstellung in der Ablage, ab Werk aus.
   - Pros: wer die Kosten spürt, kann sie abstellen; die Voreinstellung bleibt schreibfrei.
   - Cons: eine Einstellung für einen Posten, dessen Größe niemand gemessen hat; die Ablage bekommt eine achte Datei oder einen Schlüssel mehr, und die Frage, wie sich zwei Instanzen von KRK dabei verhalten, hängt an `shared/decisions/260813-0053_*_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`.
3. **Immer zurückschreiben** — KRK verhält sich wie `git status`.
   - Pros: der günstigste Dauerbetrieb; das Verhalten ist das, das jeder Nutzer von `git` kennt.
   - Cons: die Stufe A ist dann nicht mehr schreibfrei, und der `.git/index` eines fremden Repositorys wird angefasst, weil der Nutzer einen Ordner betreten hat. Bei einem Repository, das einem anderen Benutzer gehört, schlägt der Schreibversuch fehl und braucht eine eigene Fehlerbehandlung.

## Constraints

- Der Schreibweg braucht die Sperre, die `git` dafür nimmt; ein halb geschriebener Index ist ein beschädigtes Repository.
- Die Antwort ändert keine der zehn Zeitzusagen: der teuerste gemessene Status liegt bei 155 ms je Ordner mit 100 000 Einträgen, und das Budget von L10 für das vollständige Lesen beträgt 4 000 ms.
- Vor einer Antwort für Möglichkeit 2 oder 3 ist der Posten zu messen: ein Baum, dessen Zeitstempel frisch angefasst sind, gegen denselben Baum nach einem Lauf mit Rückschreiben.

## Recommendation

Wir empfehlen Möglichkeit 1 für die Stufe A, mit ausdrücklicher Wiedervorlage, sobald der Posten gemessen ist. Eine Einstellung oder einen Schreibweg für Kosten zu bauen, deren Größe niemand kennt, ist die Reihenfolge verkehrt herum. Die Messung gehört in dieselbe Runde, die den Status baut, denn dann steht die Messstrecke schon da.

---
Implemented: shared/history/260830-0950-orchestrator-session.md:150 — Möglichkeit 1: Stufe A bleibt schreibfrei. Der Posten ist in `messungen/260831-0855-needsupdate.txt` beziffert (Faktor 1,7 bis 9,5 gegenüber der Statusabfrage) und in dieser Höhe angenommen; `grep -rn 'write_changes(' crates/` liefert null Aufrufstellen.
