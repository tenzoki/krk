Ein untergeordneter Agent hat `git stash` gefahren, während ein zweiter im selben Arbeitsbaum schrieb
---
Am 260818 hat der Orchestrator zwei Agenten gleichzeitig auf getrennte Dateimengen angesetzt: ein `coder` auf Modulköpfe unter `crates/`, ein `analyst` auf Datensätze unter `fusion-workbench/`. Der `coder` wollte die Zahl der `cargo doc`-Warnungen vor und nach seinem Durchgang vergleichen und fuhr dafür `git stash` und danach `git stash pop`.

`git stash` erfasst den **ganzen** Arbeitsbaum, nicht die Dateimenge des Agenten, der es fährt. Für die Dauer eines Baulaufs war damit auch die Arbeit des parallelen Agenten zurückgesetzt. Der Agent hat den Vorfall selbst in seinem Bericht genannt.

**In diesem Fall ist nichts verloren gegangen.** Nachgeprüft: die Stash-Liste ist leer, der Commit `59ddcbe` trägt die 23 Dateien des `analyst` vollständig, die Änderungen des `coder` stehen im Arbeitsbaum, und `make check` läuft grün. Der Datensatz beschreibt also eine Gefahr, die sich diesmal nicht verwirklicht hat.
---
**Warum das trotzdem zählt.** Das Zeitfenster war zufällig günstig. Wäre der `analyst` während des Stash mitten in einer Umbenennung gewesen, hätte `git stash pop` auf einen veränderten Baum treffen können, und der Ausgang wäre ein Konflikt oder ein Verlust gewesen. Die Sicherung dagegen war nichts als der Zufall.

Die Gefahr hat zwei Seiten, und beide gehören zusammen betrachtet:

1. **Kein Agentenprompt verbietet heute einen baumweiten Eingriff.** `git stash`, `git checkout .`, `git clean` und `git reset --hard` wirken alle über die eigene Dateimenge hinaus. Der Orchestrator hat für sein eigenes Zurücknehmen eine ausdrückliche Regel — nur `git checkout HEAD -- <datei>`, nie `git checkout .` —, und die Executoren haben keine entsprechende.

2. **Der Orchestrator setzt Agenten parallel an, ohne dass die voneinander wissen.** Der `coder` konnte nicht ahnen, dass ein zweiter Agent schreibt: sein Auftrag nannte die eigene Dateimenge und schwieg über die parallele. Der Agent selbst schlägt in seinem Bericht eine zweite Arbeitskopie vor, was die Sache von der anderen Seite löst.

Damit stehen mindestens drei mögliche Antworten im Raum, die verschieden viel kosten: ein baumweiter Eingriff wird den Executoren untersagt; ein Auftrag nennt die parallel laufenden Agenten, damit der Empfänger es weiß; oder parallele Läufe bekommen je eine eigene Arbeitskopie. Welche davon richtig ist, entscheidet dieser Datensatz nicht — er hält den Vorfall fest, solange die Messung noch da ist.

**Herkunft:** gefunden neben der Arbeit an der Löschweg-Runde, nicht aus ihrer Directive. Deshalb im gemeinsamen Speicher.

**Domain:** code
**Filed by:** orchestrator
**Related:** `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/history/260818-0212-coder-sechs-prosabefunde-des-buendels-c.md` (der Bericht, der den Vorfall nennt)

Also seen: 260830-1421 by coder — Schritt 1 der Runde 23 setzte `git stash push -q --keep-index` ab, meldete es selbst und holte den Baum mit `git stash pop` zurueck; `git stash list` leer, keine Datei verloren, kein zweiter Agent lief.
