# Planner — der Plan der dreizehnten Runde

**Datum:** 260818-1633
**Modus:** Dispatch durch den Orchestrator, Executors `coder, ontocoder, analyst`
**Baumstand:** `b47355e`
**Ergebnis:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_o_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`

## Was der Auftrag war

Den Implementierungsplan zum abgenommenen Spec `shared/planning/260818-1510_*_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md` schreiben, ohne eine seiner Nutzerantworten neu zu verhandeln, und die neun Punkte seines Abschnitts `## Offen für den Planner` beantworten. Der Circle war beim Dispatch aktiv und wurde als Ziel an `fusion-paths` übergeben; Plan, Datensatz, Befund und dieses Protokoll liegen deshalb im Circle und nicht im gemeinsamen Speicher.

## Was gemessen und nicht übernommen wurde

Vier parallele Erhebungen am Baum, alle mit Zeilenangaben, keine aus der Prosa übernommen:

- **`tabelle.rs`** — die Anatomie von `DateifensterQuelle`, ihre sechzehn Ivars, ihre drei Protokolle, `ordner_lesen` samt seinen zehn Rufern, die Zeilenauflösung über `eintrag_in_zeile`, der Bauort der `NSTableView` und die vier Ränge der Statuszeile.
- **`anwendung.rs`** — `uebertragen`, `bereich_einblenden`, `auftrag_stellen`, `vorgang_laeuft_schon`, `auftrag_starten` samt Rufern, der Kommandoversand und die Unterscheidung von aktivem Dateifenster und Fokus.
- **Die Belegungskette** — `Kommando` (78 Varianten, selbst gezählt), `KENNUNGEN`, die zwei vollständigen Fallunterscheidungen, die sechs mit Auffangzweig, die Belegungsdatei mit ihren 84 Funktionen und 89 Kombinationen, und jede Probe, die eine dieser Zahlen hält.
- **`fenstermodell.rs`, `verzeichnis/sys.rs`, die Probenformen und `rueckschritt.rs`** — die zwei Bedeutungen des `false`, die fünf Systemschnittstellen des Kerns, `an_einer_flaeche` mit seiner gemessenen Grenze, und die Bauform der reinen Regel.

Dazu selbst geprüft: die Bindungen in `objc2-app-kit 0.3.2` und `objc2-foundation 0.3.2` für jede angesprochene Ziehschnittstelle, `NSURLIsWritableKey` im SDK-Kopf (`NSURL.h:247`, seit 10.7, „as determined by EUID"), und der Unterschied zwischen `8d5baf6` und `b47355e` (acht Dateien, alle unter `fusion-workbench/`, kein Code).

## Drei Befunde, die den Plan gegen den Spec verschoben haben

1. **Die Datenquelle kennt ihr eigenes Dateifenster nicht.** `QuelleIvars` trägt kein `Fensterseite`, `tabelle.rs` erreicht den Anwendungsdelegierten nirgends. Der Abwurf muss deshalb über zwei eingehängte Rückrufe nach draußen, in der Form der fünf vorhandenen. Der Spec konnte das nicht sehen; der Plan macht es zu Schritt 10 (b) und (e).
2. **`vorgang_laeuft_schon` schreibt eine Meldung.** In `validateDrop:` gerufen — und das läuft bei jeder Zeigerbewegung — füllte es die Statuszeile. Der Plan teilt die Funktion in die Frage und ihren meldenden Mantel, statt eine zweite Prüfung danebenzustellen; die Zusage des Specs, die Frage nicht zweimal zu beantworten, hält damit buchstäblich.
3. **Ein neues Kommando hat drei Pflichtstellen und nicht zwei.** Der Ausführungszweig in `kommando_ausfuehren` endet auf einen Auffangzweig; ein Kommando ohne eigenen Zweig übersetzt, besteht jede Probe, steht im Menü und tut nichts. `CLAUDE.md` nennt nur die zwei Stellen, die der Übersetzer hält. Als `shared/issues/260818-1635_*_claude-md-nennt-zwei-nachzuziehende-stellen-je-kommando-die-dritte-haelt-kein-uebersetzer.md` gefilt.

## Was der Plan an offenen Punkten entschieden hat

Alle neun. Tragend sind vier: die zwei Protokollmethoden stehen zwingend in `tabelle.rs`, weil `define_class!` eine Makroauswertung ist und Protokollkonformität nicht auf zwei Dateien verteilt; das Schreibrecht kommt über `NSURLIsWritableKey` und ausdrücklich **nicht** über `access(2)` in `verzeichnis/sys.rs`, mit vier Gründen und dem EUID-Argument voran; die Ablage des Ziehvorgangs erreicht die eine Hülle über eine fünfte Funktion mit Parameter, nicht über eine zweite Hülle; und der Abwurf wird der dritte Rufer von `auftrag_starten`.

Die Reihenfolge der Schritte ist gegen die naheliegende entschieden: die Belegungsdatei geht dem Code voran, weil `belegungsausgabe.rs:755` und `belegung.rs:1636` sonst rot stünden, während der umgekehrte Zwischenstand ein gültiger Zustand des Modells ist. Beide Schritte gehen trotzdem in einen Commit.

## Was gefilt wurde

- `decisions/260818-1633_o_gilt-ein-unentscheidbares-schreibrecht-beim-abwurf-als-erlaubnis-oder-als-abweisung.md` — die Runde 12 hat für den Löschweg „Unentschieden gilt als laut" zugesagt, dieser Plan geht für den Abwurf auf das Gegenteil. Die Übertragung scheitert an ihrer eigenen Bedingung: jene Zusage kaufte Sicherheit gegen eine **sichtbare** Rückfrage, hier gäbe es dafür nur ein stummes Verbotszeichen. Drei Möglichkeiten, Empfehlung Möglichkeit 1, umstoßbar am Plan-Gate.
- `shared/issues/260818-1635_o_claude-md-nennt-zwei-nachzuziehende-stellen-je-kommando-die-dritte-haelt-kein-uebersetzer.md` — siehe Befund 3.

## Was nicht gefilt wurde, und warum

Die Abweichung des Specs unter C3 (`_o_` behauptet, `_d_` am Bestand) bekommt keinen eigenen Datensatz: der Circle-Datensatz führt sie bereits unter „Ein gemessener Abweichungspunkt". Der Plan trägt die zutreffende Formulierung und verweist dorthin. Ein zweiter Datensatz über denselben Befund wäre die Doppelung, gegen die dieses Projekt seine Datensätze führt.

Zwei weitere Ungenauigkeiten sind im Plan berichtigt und nicht gefilt, weil sie Ortsangaben und keine Sachaussagen betreffen: der Spec verortet eine Warnung im Modulkopf von `verzeichnis/sys.rs`, die im Doc-Kommentar von `ohne_warten_oeffnen` steht, und der Circle-Datensatz zählt sieben offene Punkte, wo der Spec neun führt.

## Was der Plan nicht tut

Er startet keinen Executor. Zehn Schritte, neun für `coder`, einer für `ontocoder`, keiner für `analyst` — die Runde bringt keinen strategischen Datensatz hervor, und die eine Entscheidung, die sie aufwirft, gehört dem Nutzer. Die Ausführung beginnt, wenn der Nutzer den Plan abgenommen hat.
