# Playmaker-Lauf 260813-0714 (direct-dispatch)

**Status:** Complete
**Auslöser:** Nutzer, unmittelbar. Nach dem beschränkten Abschluss der Runde 7
(`circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/_b_circle.md`)
und dem Löschen von `.active-circle`.
**Domain-Gewichtung:** code (aus der Zeile `**Domain:** code` des Auftrags)

## Bestand

Acht Circle-Datensätze unter `circles/`, am Dateibestand gezählt und nicht aus `CLAUDE.md`
übernommen:

| Marker | Zahl | Verzeichnisse |
|---|---|---|
| `_t_` aktiv | 0 | (keiner) |
| `_a_` vorgesehen | 1 | `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` |
| `_b_` beschränkt abgeschlossen | 7 | Runden 1 bis 7 |
| `_c_` kohärent abgeschlossen | 0 | (keiner) |
| `_s_` überholt | 0 | (keiner) |
| `_d_` zurückgestellt | 0 | (keiner) |

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Regulärer
Zustand nach einem Abschluss, keine Zeigerwarnung.

Grundlagenbestand quer über beide Speicher: 19 offene Fragen (`_o_`), 1 beantwortete und nicht
umgesetzte (`_a_`), 2 zurückgestellte (`_d_`), 54 offene Defekte (`_o_`), keiner in Arbeit
(`_p_`).

## Rangfolge

**Rang 1 von 1: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.** Der einzige nicht
abgeschlossene Circle. Seine geerbten Bauteile stehen unverändert am Baum; sein Eintrittspreis je
Befehl ist mit der aus der Belegung gerechneten Menüleiste der Runde 7 gestiegen.

**Die Standardheuristik der Gewichtung `code` ist ausgesetzt, nicht angewandt.** Sie bevorzugt
Kandidaten, deren Abhängigkeiten sämtlich `_c_` tragen. In diesem Projekt trägt kein
abgeschlossener Circle `_c_`; alle sieben stehen auf `_b_`, und alle aus demselben Grund, dem
Abnahmelauf im Vordergrund. Die Heuristik vergäbe jedem denkbaren Kandidaten dasselbe Kennzeichen
und träfe keine Unterscheidung. Die Aussetzung ist in `portfolio.md` im Kopf und im
Aktivierungsvorschlag ausdrücklich benannt statt in die Zahlen eingerechnet.

## Zyklenprüfung

Kein Zyklus. Der gerichtete Graph über die nicht terminalen Circles (`_a_` und `_t_`) hat einen
Knoten und keine Kante innerhalb der Menge. Die einzige Circle-Kante des Betrachters zeigt auf
die Runde 1 und damit auf einen terminalen Knoten. Kein `## Dependency warning` angelegt.

## Angelegte Abschnitte

An `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`, beide
angehängt und nichts überschrieben:

- `## Parent grounding stale` (260813-0714), vier Feststellungen: der Eintrittspreis je Befehl
  durch die gerechnete Menüleiste samt Ausgrauungspflicht; die dritte Möglichkeit der ersten
  offenen Frage führt an die `flock`-Sperre der Ablage mit elf benannten Löchern; der
  Fokusvorbehalt `ersthelfer_gehoert_appkit` fragt nach drei Textklassen, und eine Web-Ansicht ist
  keine davon; die Messreihe hinter der dritten offenen Frage steht zwei Runden zurück.
- `## Activation proposal` (260813-0714), Rang 1 von 1, Aktivierung nach Klärungsrunde und
  Untersuchung des Darstellungsmittels.

**Ereignis:** `parent-grounding-stale: parent=260804-0933-eingebauter-web-betrachter-im-vorschaufenster child=260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz`

**Zur Auslösebedingung, offen protokolliert.** Die wörtliche Bedingung greift nicht. Der Abschnitt
`## Grounding snapshot` des Betrachters zitiert weder den Verzeichnisnamen der Runde 7 noch den
Artefakt ihrer `## Closure note`, und der Abschnitt `## Dependencies` der Runde 7 lautet „Keine
auf einen anderen Circle." Anders als beim Lauf vom 260812-2307, wo eine Kante in Gegenrichtung
bestand, gibt es zwischen beiden Circles überhaupt keine notierte Kante. Der Vermerk steht
trotzdem, weil die Runde 7 am Baum drei Sätze jenes Grounding eingeholt hat. Die Abweichung ist im
Vermerk selbst und in `portfolio.md` benannt, damit ein späterer Lauf anders entscheiden kann.

## Warnungen in `portfolio.md`

1. Der Abnahmelauf steht für die Runden 6 und 7 aus; die tragende Frage ist seit dem 260806 offen.
2. Das beglaubigte Bündel unter `target/KRK.app` überlebt keinen gewöhnlichen Entwicklungsbau
   (`shared/issues/260813-0026_*`).
3. Vier Nutzerfragen sind gebaut und trotzdem unbeantwortet (`shared/decisions/260813-0053_*`,
   alle vier auf `_o_`).
4. Die Abschlussnotiz der Runde 7 zählt acht verbliebene Datensätze, ihr Speicher trägt vierzehn
   offene, davon drei über sichtbares Verhalten (`260813-0311_*`, `260813-0416_*`,
   `260813-0420_*`). Der Circle-Datensatz ist um 07:13 geschrieben worden, der jüngste Defekt um
   07:08; die Zahl war beim Schreiben nachzählbar.
5. `CLAUDE.md` ist an drei nachgezählten Stellen überholt: vier statt sieben Runden, zwei statt
   ein vorgesehener Circle, 68 statt 76 Varianten in `Kommando::KENNUNGEN`
   (`crates/krk-core/src/tasten/belegung.rs`). Der offene Datensatz dazu nennt selbst 75 und ist
   ebenfalls einen Schritt zurück.
6. Kein Abhängigkeitszyklus.
7. Parent grounding stale beim Betrachter nach dem Abschluss der Runde 7, mit der Abweichung von
   der Auslösebedingung.

Kein Defekt und keine Frage abgelegt: das Ablegen gehört nicht zum Schreibbereich des Playmaker.
Die Punkte 2 bis 5 sind Auskünfte für die Entscheidung des Nutzers.

## Geschrieben

- `fusion-workbench/portfolio.md` (vollständig neu erzeugt)
- `fusion-workbench/circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`
  (zwei Abschnitte angehängt)
- dieses Protokoll

Nicht angefasst: `.active-circle`, Marker eines Circle-Datensatzes, Pläne, Aufgabenliste,
Entscheidungen, Defekte, Code, Daten. Kein Bündelbau.
