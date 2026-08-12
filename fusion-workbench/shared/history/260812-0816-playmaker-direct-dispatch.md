# Playmaker-Lauf 260812-0816 (direct-dispatch)

**Status:** Complete
**Domain-Gewichtung:** code (aus der Zeile `**Domain:** code` des Auftrags)
**Erzeugtes Portfolio:** `fusion-workbench/portfolio.md`
**Auslöser:** direkte Beauftragung durch den Nutzer nach dem Abschluss der Runde 5
(`260811-1304-statusleiste-mit-bereichsschaltern`, Commit `1cb5430`). Der Auftrag nennt den
Abschluss, den geräumten Zeiger und zwei Sachverhalte, und er schließt einen Commit ausdrücklich
aus. Ohne `/fusion:next` und ohne die Ansage eines Phase-4-Pings, deshalb `direct-dispatch`.

**Schreibziel dieses Protokolls:** `shared/history/`, weil kein Circle aktiv ist.

## Bestand

Sechs Circle-Datensätze unter `circles/`, Marker aus dem Dateinamen gelesen.

| Marker | Zahl | Circles |
|---|---|---|
| `_t_` aktiv | 0 | — |
| `_a_` vorgesehen | 1 | `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` |
| `_b_` beschränkt abgeschlossen | 5 | `260811-1304-statusleiste-mit-bereichsschaltern`, `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`, `260809-2040-tastenbelegung-als-markdown-in-downloads`, `260807-2116-eingebauter-editor-mit-textmarken`, `260802-0842-krk-mac-dateimanager-editor-git` |
| `_c_` kohärent abgeschlossen | 0 | — |
| `_s_` überholt | 0 | — |
| `_d_` zurückgestellt | 0 | — |

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Der
reguläre Zustand nach einem Abschluss; keine Zeigerwarnung.

Gelesene Eingaben: die drei offenen Fragen im gemeinsamen Speicher und die sechs offenen in den
Circles, die drei offenen Defekte im gemeinsamen Speicher und die vier im Circle der Runde 5, die
beiden jüngsten Sitzungsprotokolle (`260812-0306`, `260812-0252`), die vier
Entscheidungsdatensätze der Runde 5, die die Berührung tragen, sowie am Baum
`crates/krk-ui/src/fenstermodell.rs`, `crates/krk-ui/src/appkit/fenster.rs`,
`crates/krk-ui/src/appkit/aufteilung.rs` und `crates/krk-ui/src/appkit/statuszeile.rs`.

## Rangfolge

**Rang 1 und einziger Kandidat: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.**
Seine geerbten Bauteile liegen unverändert auf der Platte, und die zeitlichen Bindungen aus der
Runde 1 stehen seit dem 260807. Gegen eine sofortige Aktivierung steht sein Zuschnitt: das Mittel
der Darstellung von Web-Inhalt ist offen und gehört laut eigenem Datensatz in eine Untersuchung
vor dem Plan, dazu die ungemessene Verfügbarkeitsfrage für macOS-26-Schnittstellen. Eine
Rangfolge mit einem Element trägt keine Auskunft über relative Reife; der Vorschlag stützt sich
auf absolute Signale.

Die Rangheuristik der Domäne `code` bevorzugt Kandidaten mit wenigen offenen
Entscheidungsdatensätzen und mit durchweg kohärent abgeschlossenen Abhängigkeiten. Der erste
Zählwert steht bei einem Datensatz und ist gut; der zweite ist in diesem Projekt gegenstandslos
(siehe Warnung 1 unten).

## Zyklen

Kein `dependency-cycle-detected`. Der gerichtete Graph über die nicht-terminalen Circles hat einen
einzigen Knoten. Dessen einzige Kante endet auf der beschränkt abgeschlossenen Runde 1, also auf
einem terminalen Knoten. Kein Abschnitt `## Dependency warning` angefügt.

## Angefügte Abschnitte

Beide in `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`,
angefügt und nicht umgeschrieben. Der Datensatz trug bereits je einen Abschnitt gleichen Namens
vom 260807-1042; die neuen stehen daneben.

- `## Parent grounding stale` mit vier Punkten: die 160 Punkte Mindestbreite der Vorschau tragen
  seit der Runde 5 zwei Entscheidungen statt keiner, mit einer gerechneten Obergrenze bei rund 177
  Punkten und dem Befund, dass die Zahl dem Bereich gehört und nicht dem Tab; die Nutzerfestlegung
  vom 260808 ist von einem Agenten überstimmt worden, und dieser Circle baut auf dem Mechanismus
  auf, der sie ersetzt hat; was sich nicht bewegt hat, nämlich C1 der Runde 1 und der gegenseitige
  Ausschluss von Vorschau und Editor; die Messreihe altert weiter, und L9 kommt hinzu.
- `## Activation proposal` mit dem Vorschlag, nach einer Klärungsrunde und der Untersuchung des
  Darstellungsmittels zu aktivieren, nicht davor. Die Klärungsrunde trägt jetzt eine vierte Frage,
  die Mindestbreite der Vorschau.

`parent-grounding-stale: parent=260804-0933-eingebauter-web-betrachter-im-vorschaufenster child=260811-1304-statusleiste-mit-bereichsschaltern`

Zur Auslösebedingung, offen benannt: die Regel verlangt, dass der Abschnitt
`## Grounding snapshot` des Elterndatensatzes den Verzeichnisnamen des Kindes oder den in seiner
`## Closure note` genannten Artefakt zitiert. Der Datensatz des Web-Betrachters nennt weder das
eine noch das andere; die Wörter Statusleiste, Bereichsleiste und Mindestbreite kommen darin nicht
vor, geprüft durch Suche über die ganze Datei. Die Kante läuft in die andere Richtung: der
Abschnitt `## Dependencies` der Runde 5 nennt diesen Circle beim Namen und benennt die Berührung
mit der Zahl 160. Der Vermerk steht deshalb trotzdem, und dieselbe Abweichung ist schon im Lauf
vom 260811-2223 aufgetreten.

## Am Baum geprüft

Die tragenden Aussagen des angefügten Abschnitts sind am 260812-0816 im Baum gelesen und nicht aus
Datensätzen übernommen.

| Aussage | Fundstelle |
|---|---|
| Mindestbreite der Vorschau 160, Editor 320, Lesezeichen 120, Dateifenster je 240 | `crates/krk-ui/src/fenstermodell.rs:209-215` |
| Ein Einschaltbefehl wird stumm abgewiesen, wenn die Mindestbreiten nicht passen | `crates/krk-ui/src/fenstermodell.rs:639-643`, Rechnung bei `:685` |
| Wer unter sein Mindestmaß fiele, bekommt es und scheidet aus der Verteilung aus | `crates/krk-ui/src/fenstermodell.rs:1044`, Wasserstandsschleife bei `:1096` |
| Fenstermindestbreite 780 Punkte | `crates/krk-ui/src/appkit/fenster.rs:134` |
| Die Trennlinienbreite kommt zur Laufzeit von AppKit und steht nirgends im Baum | `crates/krk-ui/src/appkit/aufteilung.rs:616` |
| Die Bereichsleiste ist 18 Punkte hoch | `crates/krk-ui/src/appkit/statuszeile.rs:68` |

Die Obergrenze von rund 177 Punkten für die Mindestbreite der Vorschau ist gerechnet und nicht
gemessen: 780 minus 600 für Lesezeichenleiste und beide Dateifenster, minus drei Trennlinien. Sie
ist im angefügten Abschnitt als `inference:` gekennzeichnet.

## Warnungen im Portfolio

1. Die Rangheuristik hat bei den Vorbedingungen keine Trennschärfe mehr. Fünf von fünf gefahrenen
   Runden sind `_b_`, jedes Mal aus demselben Grund. Fortgeschrieben aus dem Lauf vom 260811-2223,
   dort noch mit vier Runden.
2. Neu: die Kante zwischen der Runde 5 und dem Web-Betrachter läuft nur in eine Richtung.
3. Neu: der Datensatz des Web-Betrachters trägt jetzt je zwei Abschnitte
   `## Parent grounding stale` und `## Activation proposal`; der jüngere gilt.
4. Der Kopf des Datensatzes der Runde 3 trägt `**Status:** anticipated` bei Dateiname
   `_b_circle.md`. Unverändert seit dem 260811-1415.
5. Die Spec-Dateien der Runden 2, 3 und 4 bleiben auf `_o_`; die Runde 5 hat keinen Spec, ihre
   Kriterien stehen in einem Plan auf `_c_`. Fortgeschrieben und um die Runde 5 erweitert.
6. Neu: der Plan der Runde 5 führt drei Wahlpunkte als unabgehakte Kästchen, deren Datensätze
   sämtlich auf `_i_` stehen.
7. Neu: die `## Closure note` der Runde 5 datiert den Abschluss auf 260812-0820, dieser Lauf läuft
   um 260812-0816.
8. Die Sternform in den Pfadzitaten des Portfolios hält kein Mechanismus; dieser Lauf hat sie von
   Hand durchgehalten.

Nicht fortgeschrieben aus dem Lauf vom 260811-2223: die dortige Warnung 2 über zwei Zitate auf
nicht vorhandene Defektdatensätze. Sie betraf Zitate in einer früheren Fassung von
`portfolio.md`, die dieser Lauf vollständig ersetzt; kein Zitat der neuen Fassung zeigt auf eine
Datei, die es nicht gibt.

## Was dieser Lauf nicht getan hat

Keine Umbenennung eines Markers, kein Schreiben oder Löschen von `.active-circle`, kein Defekt
angelegt oder geschlossen, keine Entscheidung angefasst, kein Plan und keine Aufgabenliste
berührt, kein `## Dependencies` ergänzt, kein Commit. Der Auftrag schließt den Commit ausdrücklich
aus.
