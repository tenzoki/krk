Der gesamte Speicher shared/ verschwand aus dem Arbeitsbaum, während der Planner lief

---

Zwischen dem Commit `5a52f16` (260816-2256) und dem Ende des Planner-Laufs (260817-0344)
ist das Verzeichnis `fusion-workbench/shared/` vollständig aus dem Arbeitsbaum
verschwunden. `git status` meldete 183 gelöschte Dateien, `ls` meldete
"No such file or directory". Die Ursache ist unbekannt.

Wiederhergestellt mit `git checkout HEAD -- fusion-workbench/shared`. Alle 183
verfolgten Dateien sind zurück: 4 Pläne, 77 Defekte, 24 Entscheidungen, 74
Sitzungsprotokolle, 6 Durchsichten, 1 Beratung, 3 Backlog-Einträge. Die drei leeren
Speicher `analyses/`, `memos/` und `investigations/` sind von Hand neu angelegt worden,
weil git kein leeres Verzeichnis führt.

**Eine Datei ist verloren** und nicht wiederherstellbar: der Befund, den der Planner
während seines Laufs nach `shared/issues/` gelegt hat, zum Doc-Kommentar von
`ablage::pfad`, der vier Dateien nennt. Er war beim Verschwinden noch nicht committet.
Sein Inhalt steht dem Sinn nach im Sitzungsprotokoll des Planners
(`circles/260816-2255-befehle-absetzen-und-makros-speichern/history/260816-2307-plan-der-zwoelften-runde.md`)
und ist von dort neu zu erheben.

---

Was gesichert ist:

- Der Circle der zwölften Runde und alles darin haben überlebt. Betroffen war allein
  `shared/`.
- Das Ereignisprotokoll des Wächters (`.guard-state/events.jsonl`) endet um 20:56 UTC
  mit dem Schreiben der Commit-Nachricht. Nach diesem Zeitpunkt steht dort nichts, also
  ist das Verschwinden nicht über ein bewachtes `Write` oder `Edit` gelaufen.
- Der Orchestrator hat in diesem Zeitraum kein Kommando abgesetzt, das unter `shared/`
  schreibt oder löscht. Sein letzter Eingriff vor dem Befund war der Commit `5a52f16`.

Was nicht gesichert ist: wer oder was gelöscht hat. Der Planner lief in diesem Fenster
und ist damit zeitlich der nächste Verdacht, aber sein Auftrag verlangte allein Lesen
unter `shared/` und Schreiben in den Circle, und ein Beleg für seine Urheberschaft liegt
nicht vor. Ebenso möglich ist ein Vorgang außerhalb der Sitzung.

**Der eigentliche Befund ist nicht die Löschung, sondern dass sie unbemerkt blieb.**
Sie wäre erst beim nächsten `git status` aufgefallen, und das war Zufall: der
Orchestrator hat ihn vor einem Commit gefahren. Wäre in diesem Zustand ein
`git add`-Aufruf mit Verzeichnisargument gelaufen, hätte er 183 Löschungen in einen
Commit gestellt.

---

## Berichtigung vom 260817-0435

Die forensische Untersuchung `shared/analyses/260817-0419-verlust-des-speichers-shared.md`
widerlegt drei Aussagen dieses Datensatzes. Sie bleiben oben stehen, damit der Irrtum
nachlesbar ist; verbindlich ist, was hier steht.

**Der Titel ist falsch.** Der Planner war zum Zeitpunkt der Löschung 4 Stunden 26 Minuten
beendet (`planner_done` am 260816-23:18). Seine Mitschrift zeigt 78 Werkzeugaufrufe, unter
`shared/` ausschließlich lesend plus die eine abgelegte Datei. Der Dateiname ist nicht
geändert, weil die Analyse ihn zitiert.

**Der Verursacher ist KRK selbst.** Um 03:44:31 Ortszeit über `trashItemAtURL:` in den
Papierkorb, während die Anwendung von Hand bedient wurde und im Vordergrund stand. Vier
unabhängige Messungen tragen das: Prozessstart 03:42:51, Maus- und Vordergrundspuren bis
03:44:33, die XPC-Verbindung `com.apple.coreservices.quarantine-resolver` um 03:44:31.204,
und die mtime von `~/.Trash` auf dieselbe Sekunde. Dass der geräumte Eintrag `shared` war,
war erschlossen und ist inzwischen bestätigt: der Nutzer hat den Ordner im Papierkorb
gefunden.

**Der Schluss aus dem Wächterprotokoll trug nicht.** Oben steht, das Schweigen des Logs
schließe ein bewachtes Schreiben aus. Der Wächter zeichnet seit dem 260816 keinen
Bash-Aufruf mehr auf und blockiert seit dem 260807 nichts; ein `rm -rf` aus der Sitzung
hätte dieselbe Stille hinterlassen. Der Beleg war keiner.

**Die Zahl war 189, nicht 183.** Dreifach gemessen.

## Stand nach der Bergung

Der Nutzer hat den Ordner aus dem Papierkorb nach `~/Documents/shared` kopiert. Der
Abgleich am 260817-0435 zeigt: 190 Dateien im geborgenen Ordner, alle inhaltlich
identisch mit der Werkbank (`diff -rq`, kein Unterschied bei den gemeinsamen Dateien).
Die eine Datei, die der Wiederherstellung aus `HEAD` fehlte, ist zurückgestellt:
`issues/260816-2307_o_der-doc-kommentar-von-ablage-pfad-nennt-vier-dateien-die-aufzaehlung-fuehrt-sechs.md`.

**Der Werkbank fehlt nichts mehr.** `git ls-files -d` meldet keine fehlende verfolgte
Datei im ganzen Baum, alle wurzelverankerten Flächen stehen, und die dreizehn Circles
tragen ihre Datensätze. Der Verlust ist damit vollständig behoben.

**Die Ursache steht weiter offen**, und sie hat einen eigenen Datensatz:
`shared/issues/260816-2144_o_das-raeumen-in-den-papierkorb-laeuft-ohne-rueckfrage.md`. Er
war am 260816-2144 als Risiko abgelegt und ist seit dieser Nacht ein Schadensfall.

## Nachtrag vom 260817: ein zweiter Vorfall auf einem anderen Gerät

Der Nutzer berichtet einen gleichartigen Vorfall auf einem seiner anderen Computer.
Damit ist die Fehlbedienung als Ursache gesichert und nicht mehr erschlossen: derselbe
Griff hat zweimal auf zwei Geräten denselben Schaden angerichtet.

Der Datensatz ist damit ursächlich geklärt. Was bleibt, ist die Abstellung, und die
gehört nicht hierher, sondern nach
`shared/issues/260816-2144_o_das-raeumen-in-den-papierkorb-laeuft-ohne-rueckfrage.md`.
