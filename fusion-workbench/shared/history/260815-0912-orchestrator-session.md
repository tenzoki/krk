# Orchestrator-Sitzung — 260815-0912

**Directive:** Der Filter der Dateiliste soll den Ordnerwechsel überstehen, statt beim Wechsel gelöscht zu werden (Punkt 3 des Bugreports vom 260815). Punkt 1 und 2 des Berichts hat der Nutzer als bestehendes Verhalten bestätigt.
**Mode:** custom
**Status:** Läuft

## Bestandsaufnahme beim Start

| Größe | Wert |
|---|---|
| Arbeitsverzeichnis | `/Users/k1/Projects/productive/krk` |
| git HEAD | `c3fcdef` |
| Turn-Budget | 5 (aufgelöst über `bin/fusion-turn-budget`) |
| Erkannte Domäne | `code` (137 Code-Dateien, 11 Datendateien, gezählt über `git ls-files`) |
| Offene Defekte | 99 (`_o_`), davon 13 im gemeinsamen Speicher, 86 in den Circles |
| Offene Fragen | 24 (`_o_` in `decisions/`) |
| Offene Plandateien | 8 (1 gemeinsam, 7 in Circles) |
| Circles | 1 vorgesehen (`_a_`), 9 beschränkt geschlossen (`_b_`), 1 kohärent geschlossen (`_c_`) |
| Aktiver Circle | keiner (`.active-circle` fehlt) |
| Arbeitswarteschlange | keine (`tasklist.md` liegt nicht vor) |
| Compliance Guard | kein Halt aktiv (`haltActive: false`) |

**Portfolio-Hinweis ausgegeben:** ja — 1 vorgesehener Circle, 0 aktive. Empfehlung an den Nutzer, vor dem Start `/fusion:next` zu fahren.

## Sitzungsstart-Prüfungen

Keine unterbrochene Sitzung: `agentstate.yaml` lag nicht vor. Die Sitzungsmarke war veraltet (letzter Herzschlag vor 65525 Sekunden, aus einer Sitzung vom 260814-1259) und ist ohne Rückfrage überschrieben worden.

Das Layout ist auf dem Stand v4: die Prüfung auf Typ-Ordner der Wurzel, alte Circle-Dateiform und Klammer-Marker meldete `OLD=0`.

Der Monitor ist frisch aus dem Plugin (Version 8.2.0) kopiert. Die Stilprofile, die Plane-Vorlage und `fusion-guard.json` lagen bereits vor und sind unverändert geblieben.

## Änderungsdichte

Die Rangliste über `bin/fusion-churn-rank` führt 997 Einträge, davon 465 für Dateien, die es nicht mehr gibt, und 2 Arbeitsflächen, die der Zähler nicht als Änderung wertet. Gerankt bleiben 10:

| Punkte | Datei |
|---|---|
| 198 | `crates/krk-ui/src/appkit/anwendung.rs` |
| 102 | `crates/krk-ui/src/appkit/tabelle.rs` |
| 88 | `crates/krk-ui/src/appkit/editor.rs` |
| 60 | `crates/krk-ui/src/appkit/vorschau.rs` |
| 57 | `CLAUDE.md` |

## Verlauf

(wird während der Sitzung fortgeschrieben)

## Der Bugreport vom 260815 und was die Prüfung ergab

Der Nutzer meldet drei Punkte am Filter der Dateiliste aus der Runde 10. Die Prüfung am Baum trennt sie anders auf, als der Bericht sie stellt, und zwei der drei sind kein Defekt.

**Bei ausgeschaltetem „Deep" läuft keine Tiefensuche.** Der Durchlauf über den Unterbaum entsteht allein in `Tabliste::durchlauf_nachziehen_an`, und die Bedingung dort verlangt beides, einen stehenden Filtertext und das eingeschaltete Kennzeichen (`crates/krk-ui/src/tabs.rs:765`). Was der Nutzer als Tiefensuche liest, ist eine andere Regel: bei flacher Suche bleibt jeder Ordner stehen, gefiltert werden allein die Dateien. Sie steht als Abnahmekriterium C1.6 im Spec der Runde 10 und ist dort begründet, damit die Navigation bei stehendem Filter nicht abbricht. Der zweite Punkt des Berichts, dass die Liste bei nicht passenden Zeichen nicht auf null fällt, hat dieselbe Wurzel: die Ordner bleiben und halten die Zahl über null.

**Der Nutzer hat C1.6 am 260815 bestätigt** und die Möglichkeit, den Namensvergleich auch auf Ordner zu ziehen, ausdrücklich abgelehnt. Punkt 1 und 2 führen damit zu keiner Änderung am Baum. Das ist hier festgehalten, weil ein späterer Leser den Bericht sonst für unerledigt hält.

**Der dritte Punkt ist ein offener Entscheid und kein Defekt.** `decisions/260814-1830_o_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md` führt genau diese Frage mit drei Möglichkeiten. Der Nutzer wählt Möglichkeit 2, eine Regel für beide Zustände des Kennzeichens: der Filtertext übersteht jeden Ordnerwechsel und fällt erst mit `Esc`. Der Code hält die Stelle seit dem Bau frei, `let filtertext_ueberlebt = tief;` in `crates/krk-ui/src/tabs.rs:565`, mit dem ausgeschriebenen Hinweis, dass daraus ein `true` wird, sobald die Antwort so ausfällt.

Die Bedingung, die der Datensatz an Möglichkeit 2 knüpft, ist vor der Vergabe geprüft und erfüllt: der stehende Filtertext muss zu sehen sein, sonst hält der Nutzer einen gefilterten Ordner für fast leer. `filterstand_text` schreibt `Filter „rs": 3 von 47 angezeigt` in die eine Statuszeile (`crates/krk-ui/src/appkit/statuszeile.rs:369-386`).

Der Nutzer hat die Änderung als Korrektur ohne eigene Runde beauftragt.
