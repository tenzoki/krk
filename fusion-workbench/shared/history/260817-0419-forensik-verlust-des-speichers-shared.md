# Forensische Untersuchung: der Verlust des Speichers `shared/`

**Datum:** 2026-08-17 04:19
**Status:** Complete
**Agent:** analyst
**Typ:** Failure Investigation
**Ablage:** `shared/`, nicht im Circle: der Vorfall stammt nicht aus der Directive der zwölften Runde, er wurde daneben gefunden.

## Auftrag

Feststellen, was `fusion-workbench/shared/` aus dem Arbeitsbaum entfernt hat, wann, und welcher Mechanismus es hätte melden müssen. Rein lesend.

## Was gelesen wurde

Die vier Dateien der Beweisaufnahme unter `/tmp/krk-vorfall-260817-0354/`; das Sitzungsprotokoll des Planners; das vollständige Wächterprotokoll (39 764 Zeilen, ausgewertet nach Tag und Ereignisart); die Claude-Code-Mitschriften dieser Sitzung samt sechs Unteragenten und die des Projekts `fusion` samt Unteragenten; das vereinheitlichte Systemprotokoll von macOS für 03:40 bis 03:50 (30,9 MB); `git ls-tree`, `git log --all`, `git reflog`; `~/Library/Application Support/KRK/session.toml`; `crates/krk-core/src/operation/loeschen.rs`; die Wächter-Haken und `bin/fusion-paths` im Plugin. Nichts verändert, keine Datei der Beweisaufnahme angefasst.

## Was gefunden wurde

Der Urheber ist bestimmt: KRK selbst hat um 03:44:31 einen Eintrag in den Papierkorb geräumt, während es Vordergrundanwendung war und von Hand bedient wurde. Die Kette besteht aus vier unabhängigen Messungen (Prozessstart, Vordergrund- und Mausspuren, die XPC-Verbindung `quarantine-resolver`, die mtime von `~/.Trash`) plus der mtime der Werkbank-Wurzel, die die Löschung auf dieselbe Minute legt. Der Planner ist durch seine Mitschrift entlastet: 78 Werkzeugaufrufe, kein einziger schreibend unter `shared/` außer der einen abgelegten Datei, und er endete 4 h 26 min vor der Löschung.

Vier Berichtigungen am Defektdatensatz `260817-0354_o_…`: 189 statt 183 Dateien, das Zeitfenster, die Ursache, und der Schluss aus dem Ende des Wächterprotokolls.

Der systemische Befund: keine Meldefläche der Werkbank beobachtet den Bestand des Arbeitsbaums. `staging-drift` hätte gemeldet, wird aber allein von einer HEAD-Bewegung ausgelöst, und im Fenster gab es keinen Commit. `fusion-paths` prüft den Circle und nicht die Speicher darunter und hätte den fehlenden Speicher stumm weitergereicht.

## Erzeugte Dateien

- `shared/analyses/260817-0419-verlust-des-speichers-shared.md`

## Nicht getan

Kein neuer Defektdatensatz: die drei Sachverhalte sind bereits gefasst. Kein Eingriff in Code, Daten oder bestehende Datensätze. Der Papierkorb ist nicht geöffnet worden, weil der Datenschutzmechanismus von macOS diesem Prozess den Zugriff verwehrt; das ist die eine offene Prüfung und liegt beim Nutzer.
