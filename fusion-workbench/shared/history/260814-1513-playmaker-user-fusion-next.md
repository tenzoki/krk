# Playmaker-Lauf — 260814-1513

**Status:** Complete
**Auslöser:** `/fusion:next` (erste Zuteilung des Relais; kein `**Confirmed operations:**`-Block im Auftrag)
**Domain-Gewichtung:** `code` — aus der Zeile `**Domain:** code` des Auftrags gelesen, nicht vorgegeben
**Git HEAD:** `43dfe90`, Tag `v0.3.0` zeigt auf HEAD
**Mandat:** ranken, Portfolio neu erzeugen, Rangumbenennungen im Ideenspeicher. Keine
bestätigungspflichtige Operation, weil dieser Lauf keine Bestätigung hält: als Unteragent hat er
keinen Kanal zum Nutzer, und der Auftrag nennt keine bestätigten Operationen.

## Bestandsaufnahme

| Marker | Anzahl | Circles |
|---|---|---|
| `_a_` vorgesehen | 1 | `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` |
| `_t_` aktiv | 0 | — |
| `_c_` kohärent geschlossen | 1 | `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` |
| `_b_` beschränkt geschlossen | 8 | Runden 1, 2, 3, 4, 5, 6, 7, 9 |
| `_s_` überholt | 0 | — |
| `_d_` zurückgestellt | 0 | — |

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Das ist der
reguläre Zustand nach einem Abschluss, also keine der vier Zeigerwarnungen.

## Rangfolge der vorgesehenen Circles

**Rang 1 und einziger Kandidat:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.
Ein einziger offener Entscheidungsdatensatz bindet ihn
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`),
und seine Abhängigkeit auf die Runde 1 ist am Baum gebaut, obwohl die Runde beschränkt geschlossen
ist. Die Gewichtung `code` bevorzugt Circles mit wenigen unbeantworteten Fragen und geschlossenen
Abhängigkeiten; der Abzug für `_b_` statt `_c_` ist hier nicht angesetzt worden, weil `CLAUDE.md`
den Marker in diesem Projekt ausdrücklich als Auskunft über die Verfügbarkeit des Nutzers
ausweist und nicht über die Reife einer Runde.

## Ideenspeicher

| Marker | Anzahl |
|---|---|
| `_o_` offen (beim Lesen) | 1 |
| `_p_` empfohlen (beim Lesen) | 0 |
| `_c_` geschlossen | 2 |
| `_d_` zurückgestellt | 0 |

Ein Eintrag lebt, er trägt genau eine Idee. Keine Dublettengruppe, da nur ein lebender Eintrag
vorliegt. Ein Teil dieses Eintrags ist defektförmig und ist an `## Warnings` übergeben worden,
Punkt 2; kein Teil ist entscheidungsförmig.

**Rang 1:** `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
— eine Idee, kein Split nötig, die selbstgestellte Vorbedingung ist beantwortet und die Runde 9
hat den Präzedenzfall gebaut.

### Schreibvorgänge im Ideenspeicher

- Umbenennung `260813-2033_o_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
  → `260813-2033_p_…`, alter Marker `_o_`, neuer Marker `_p_`. Nachgeholt: der Lauf vom
  260814-1301 hat den Eintrag empfohlen, ohne die Umbenennung mitzuziehen.

Kein Split, keine Zusammenlegung, kein Schließen, kein Zurückstellen ausgeführt.

### Vorgeschlagen und nicht ausgeführt

Keine. Für keine der vier bestätigungspflichtigen Operationen bestand ein Anlass, also ist auch
keine mangels Bestätigung liegengeblieben.

## Schreibvorgänge an Circle-Datensätzen

- `## Activation proposal` angehängt an
  `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`. Inhalt: zwei
  Änderungen seit dem 260814-1301, die geschlossene Auslieferungssperre und die kuratierte
  `CLAUDE.md`. Der Datensatz trägt danach 943 Zeilen, acht Aktivierungsvorschläge und sechs
  Vermerke zu gealterter Grundlage.
- Keine `## Dependency warning` angehängt: der gerichtete Graph über die nicht terminalen Circles
  hat einen Knoten und keine Kante innerhalb dieser Menge.
- Kein `## Parent grounding stale` angehängt: seit dem Lauf vom 260814-1301 ist kein Circle auf
  `_b_` übergegangen, die Auslösebedingung ist also nicht eingetreten. Kein
  `parent-grounding-stale`-Ereignis in diesem Lauf.

## Warnungen im Portfolio

1. Die Runde 9 nennt einen Weg zu einem kohärenten Abschluss, und der Marker geht ihn nicht mit
   (21 Kriterien ohne vollen Beleg, `_b_` ist ein Endzustand).
2. Der empfohlene Ideeneintrag ist zur Hälfte defektförmig; der Nutzerentscheid vom 260802-1409
   und der Kommentar an `bearbeiten` in `resources/default-keymap.toml` gehen nicht auf.
3. Der Defekt am doppelt belegten Ausgabeort `target/KRK.app` besteht unverändert.
4. 89 offene Defekte, 11 davon im gemeinsamen Speicher; unverändert gegenüber dem 260814-1301.
5. 19 offene Entscheidungsdatensätze, einer beantwortet und nicht umgesetzt; unverändert.
6. Kein Abhängigkeitszyklus.
7. Kein neuer Vermerk zu gealterter Grundlage.
8. Der Datensatz des Web-Betrachters trägt 943 Zeilen und vierzehn Playmaker-Abschnitte.
9. Zwei Warnungen früherer Läufe sind erledigt: die Auslieferungssperre (`v0.3.0` getaggt,
   `Cargo.toml` auf `0.3.0`) und die veraltete Rundenzahl in `CLAUDE.md` (Kuratorenlauf
   260814-1405).

## Ausgabe

Portfolio neu erzeugt: `fusion-workbench/portfolio.md`
