# Arbeitswarteschlange für einen Defekt-Durchlauf gebaut

**Datum:** 260810-1707
**Agent:** taskplanner
**Domain:** code
**Status:** Complete
**Ergebnis:** `fusion-workbench/tasklist.md` neu angelegt

## Umfang

Kein Circle war aktiv; die aufgelösten Scan-Pfade deckten allein `fusion-workbench/shared/` ab. Der Nutzer hat den Umfang stattdessen auf acht Defektdateien festgelegt, drei im gemeinsamen Speicher und fünf im Circle der Runde 1 (`circles/260802-0842-krk-mac-dateimanager-editor-git/`), der als beschränkter Abschluss geschlossen ist, dessen Defekte aber weiterbinden. Es ist kein Scan gefahren worden und keine weitere Datei aufgenommen worden.

Die Schlange trägt im Kopf eine Zeile `**Active Circle:**`, die den Zustand "kein Circle aktiv" festhält, damit der Orchestrator sie später als gültig oder veraltet einstufen kann.

## Gelesen

- 8 Defektdatensätze, vollständig
- 1 Plandokument der Runde 1 (`260802-1428_c_plan-navigator-geruest-runde-1.md`, 1461 Zeilen), an den Fundstellen
- 5 Entscheidungsdatensätze des Circles `260809-2040-tastenbelegung-als-markdown-in-downloads`, Zeile 7 je Datei
- 4 Circle-Datensätze, `portfolio.md`, `CLAUDE.md`, `spikes/fn-tasten/README.md` für die Verweiserhebung
- Quelldateien: `crates/krk-bench/src/messen.rs`, `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/appkit/tabelle.rs`, `crates/krk-ui/src/kommandos/operationen.rs`

## Aufgaben

Acht Aufgaben aus acht Defektdateien. Die Zahl stimmt zufällig überein: zwei Dateien sind zu einer Aufgabe zusammengezogen, eine Datei in zwei Aufgaben geteilt.

| Nr. | Kurz | Ausführer | Zustand |
|---|---|---|---|
| T1 | Plan der Runde 1 führt den Messstrecken-Defekt noch als offen | ontocoder | bereit |
| T2 | Circle-Datensätze und `portfolio.md`, überholte Zustandsmarker | ontocoder | bereit |
| T3 | Messplan bleibt nach Abbruch im Temporärverzeichnis liegen | coder | bereit |
| T4 | Verweis nennt den falschen Circle | ontocoder | bereit |
| T5 | `CLAUDE.md` und `spikes/`, überholte Zustandsmarker | coder | wartet auf T2 |
| T6 | Meldung zur Bündelkennung nennt den Ladezeitpunkt nicht | coder | wartet auf einen Nutzerentscheid |
| T7 | `vorgang_beenden` wirft den Auswahlversuch weg | coder | wartet auf einen Nutzerentscheid |
| T8 | L6-Aussetzer im Sitzungslauf | keiner | zurückgestellt |

Vier Aufgaben sind sofort lauffähig, drei sind blockiert, eine ist zurückgestellt.

## Die drei übernommenen Vorentscheidungen

Alle drei sind so übernommen worden, wie der Orchestrator sie getroffen hat, und nicht neu aufgerollt.

1. **`260810-1330` und `260810-1430` sind derselbe Defekt.** Am Text beider Datensätze bestätigt: derselbe Schreibort `messen.rs:1551`, dieselbe Abräumzeile `messen.rs:1046` hinter der Rundenschleife, dieselben neun Restdateien als Beleg, derselbe Lösungsvorschlag (ein Halter mit `Drop`). T3 läuft gegen `260810-1330`; `260810-1430` ist als Dublette geführt und mit demselben Fix zu schließen.
2. **Der L6-Aussetzer wird nicht bearbeitet.** Als T8 ohne Ausführer aufgenommen. Der Grund steht in der Aufgabe: die offene Frage beantwortet allein ein vollständiger Sitzungslauf, und der verlangt KRK im Vordergrund.
3. **Die Meldung zur Bündelkennung ist ein Tor.** T6 wartet auf die Wahl des Nutzers, danach `coder`.

## Zur geteilten Aufgabe

Der Defekt über die zweiundzwanzig Verweise ist in **zwei Aufgaben mit Abhängigkeit** geteilt worden, nicht als eine Aufgabe mit zwei Ausführern geführt. Der Grund ist eine echte Abhängigkeit und nicht die Zuständigkeitsgrenze: der Datensatz erlaubt Aufzeichnungen eines Standes, ihren damaligen Marker zu behalten, "wenn der Fixer das ausdrücklich so entscheidet". `spikes/fn-tasten/README.md` ist genau so eine Aufzeichnung. Diese eine Festlegung fällt in T2, und T5 erbt sie. Sie an beiden Stellen zu treffen, wäre die zweite Wahrheit über dieselbe Frage. Der gemeinsame Defektdatensatz geht erst auf `_c_`, wenn beide Aufgaben fertig sind.

## Was die Prüfung gegen den Baum ergeben hat

Jede Fundstelle ist gelesen worden, statt aus den Datensätzen übernommen zu werden. Sieben Angaben waren überholt, drei davon aus dem Auftrag selbst. Die vollständige Aufstellung steht in `tasklist.md` unter `## Prüfnotizen`; die drei mit Folgen für die Arbeit:

- **T7 ist ein zweites Tor**, und der Auftrag hat es als gewöhnliche Umsetzung geführt. Der Nachtrag des Datensatzes vom 260807 verlangt ausdrücklich eine Vorlage beim Nutzer, weil die Änderung am Verhalten sichtbar ist. Dieselbe Behandlung wie T6.
- **T7 betrifft eine Stelle, nicht drei.** Commit `5d7e299` hat zwei der drei ausgeräumt: `auswahl_auf_namen` fragt `tab.liest()` zuerst, womit `Auswahlversuch::Unbekannt` bei laufendem Lesevorgang nicht mehr erreichbar ist. Der ursprüngliche Vorschlag "an jeder der drei Stellen melden" wäre an zwei Stellen toter Code. Es bleibt `vorgang_beenden` im Zweig `Art::UmbenennenImStapel`.
- **T1 betrifft eine Stelle, nicht zwei.** Die zweite genannte Stelle, `### Frage 5`, ist bereits berichtigt und sagt heute selbst, der Defekt sei geschlossen. Dafür ist eine dritte Stelle dazugekommen, die kein Datensatz kennt: Zeile 1458 führt den Befund selbst und wird mit seiner Behebung falsch.

Sämtliche Zeilennummern aus Auftrag und Datensätzen sind abgewandert, seit die Runde 2 durchgelaufen ist. Die Schlange trägt die Nummern vom 260810-1707 und nennt daneben den Wortlaut oder den Funktionsnamen, damit die nächste Wanderung sie nicht wieder wertlos macht.

## Abhängigkeitsgraph

Zehn Knoten, drei Kanten, als Mermaid-`flowchart TD` in der Schlange. Kein Kreis, kein Knoten mit auffälligem Ausgangsgrad, durchgehende Richtung von oben nach unten. Die dünne Verknüpfung ist der ehrliche Befund: acht Defekte aus vier Quellen, die einander fachlich kaum bedingen. T8 hängt an nichts und steht deshalb in einem eigenen Teilgraphen "Zurückgestellt", damit die fehlende Verbindung als Zustand lesbar ist und nicht als vergessene Kante.

## Nicht getan

Kein Quelltext und kein Datensatz ist geändert worden. Kein Dateimarker ist umbenannt worden; alle acht Defektdateien tragen weiter `_o_`. Geschrieben sind allein `fusion-workbench/tasklist.md` und diese Datei.
