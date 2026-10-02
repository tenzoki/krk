# Playmaker-Lauf — 260815-0350

**Status:** Complete
**Auslöser:** `/fusion:next` (erste Zuteilung; kein `**Confirmed operations:**`-Block im Auftrag)
**Domain-Gewichtung:** `code` — aus der Zeile `**Domain:** code` des Auftrags gelesen, nicht vorgegeben
**Git HEAD:** `2d2ce87`, kein Tag auf HEAD, 24 Commits hinter `v0.3.0`
**Mandat:** ranken, Portfolio neu erzeugen, Rangumbenennungen im Ideenspeicher. Keine
bestätigungspflichtige Operation, weil dieser Lauf keine Bestätigung hält: als Unteragent hat er
keinen Kanal zum Nutzer, und der Auftrag nennt keine bestätigten Operationen.

## Bestandsaufnahme

| Marker | Anzahl | Circles |
|---|---|---|
| `_a_` vorgesehen | 1 | `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` |
| `_t_` aktiv | 0 | — |
| `_c_` kohärent geschlossen | 1 | `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` |
| `_b_` beschränkt geschlossen | 9 | Runden 1, 2, 3, 4, 5, 6, 7, 9, 10 |
| `_s_` überholt | 0 | — |
| `_d_` zurückgestellt | 0 | — |

Neu seit dem Lauf vom 260814-1513: `260814-1551-tippen-filtert-dateiliste-flach-und-tief`
(Runde 10), am 260815 beschränkt geschlossen.

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Das ist der
reguläre Zustand nach einem Abschluss, also keine der vier Zeigerwarnungen.

## Rangfolge der vorgesehenen Circles

**Rang 1 und einziger Kandidat:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.
Ein einziger offener Entscheidungsdatensatz bindet ihn
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`),
und seine vier Abhängigkeiten führen auf die Runden 1, 5, 6 und 7, alle terminal und alle am Baum
gebaut. Die Gewichtung `code` bevorzugt Circles mit wenigen unbeantworteten Fragen und
geschlossenen Abhängigkeiten; der Abzug für `_b_` statt `_c_` ist wie in den Läufen davor nicht
angesetzt worden, weil `CLAUDE.md` den Marker in diesem Projekt ausdrücklich als Auskunft über die
Verfügbarkeit des Nutzers ausweist und nicht über die Reife einer Runde.

## Ideenspeicher

| Marker | Anzahl |
|---|---|
| `_o_` offen | 0 |
| `_p_` empfohlen | 1 |
| `_c_` geschlossen | 2 |
| `_d_` zurückgestellt | 0 |

Ein Eintrag lebt, er trägt genau eine Idee. Keine Dublettengruppe, da nur ein lebender Eintrag
vorliegt. Ein Teil dieses Eintrags ist defektförmig und ist an `## Warnings` übergeben worden,
Punkt 4; kein Teil ist entscheidungsförmig.

**Rang 1:** `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
— eine Idee, kein Split nötig, die selbstgestellte Vorbedingung ist beantwortet und die Runde 9
hat den Präzedenzfall gebaut.

### Schreibvorgänge im Ideenspeicher

Keine. Der eine lebende Eintrag steht bereits auf `_p_`, gesetzt vom Lauf am 260814-1513, und die
Rangfolge hat sich nicht geändert. Kein Split, keine Zusammenlegung, kein Schließen, kein
Zurückstellen ausgeführt.

### Vorgeschlagen und nicht ausgeführt

Keine. Für keine der vier bestätigungspflichtigen Operationen bestand ein Anlass, also ist auch
keine mangels Bestätigung liegengeblieben.

## Schreibvorgänge an Circle-Datensätzen

- `## Parent grounding stale` angehängt an
  `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`. Ereignis:
  `parent-grounding-stale: parent=260804-0933-eingebauter-web-betrachter-im-vorschaufenster
  child=260814-1551-tippen-filtert-dateiliste-flach-und-tief`. Die Auslösebedingung ist zur Hälfte
  erfüllt und der Vermerk sagt das: die Runde 10 trägt `_b_`, aber der Grounding-Abschnitt des
  Betrachters stammt vom 260804 und zitiert sie nicht. Fünf Punkte: sechster Rang der Statuszeile
  (`crates/krk-ui/src/appkit/statuszeile.rs:207`), die gefallene Sprungmarke des Dateifensters und
  die Namenskollision mit den Verweisankern des Betrachters, die dritte Bedeutung von `Esc`, die
  gewachsene Belegung (84 Einträge, 78 `Kommando`-Varianten) samt dem Defekt zur unbelegten
  Neufunktion, und die übertragbare Lehre der Runde 10 zur Diagrammprüfung vor dem Plan-Gate.
- `## Activation proposal` angehängt an denselben Datensatz. Inhalt: drei Änderungen seit dem
  260814-1513, die geschlossene Runde 10, die wieder offene Auslieferungssperre und die zwei
  ausstehenden Abnahmelisten. Der Datensatz trägt danach 1069 Zeilen, neun Aktivierungsvorschläge
  und sieben Vermerke zu gealterter Grundlage.
- Keine `## Dependency warning` angehängt: der gerichtete Graph über die nicht terminalen Circles
  hat einen Knoten und keine Kante innerhalb dieser Menge.

## Warnungen im Portfolio

1. `CLAUDE.md` beschreibt neun Runden, der Dateibestand trägt zehn; der Filter der Dateiliste
   fehlt im Absatz `## Projektstand`, und die Zählungen für `Kommando` und die Belegung sind
   überholt (78 statt 75, 84 Einträge).
2. Zwei Abnahmeläufe stehen aus, Runde 9 mit 21 Kriterien ohne vollen Beleg und Runde 10 mit zehn
   von 77 im Bündelanteil, davon vier sicherheitsrelevant. Beide sind Nutzerarbeit, keiner bewegt
   einen Marker.
3. Die Auslieferungssperre steht wieder offen: kein Tag auf HEAD, 24 Commits hinter `v0.3.0`,
   `Cargo.toml` auf `0.3.0`.
4. Der empfohlene Ideeneintrag ist zur Hälfte defektförmig; der Nutzerentscheid vom 260802-1409
   und der Kommentar an `bearbeiten` in `resources/default-keymap.toml` gehen nicht auf.
5. Der Defekt am doppelt belegten Ausgabeort `target/KRK.app` besteht unverändert und wird von den
   zwei ausstehenden Abnahmeläufen erneut getroffen.
6. 99 offene Defekte, 13 davon im gemeinsamen Speicher; zehn mehr als am 260814-1513.
7. 24 offene Entscheidungsdatensätze, neun beantwortet und nicht umgesetzt; dreizehn mehr als am
   260814-1513, alle aus der Runde 10. Eine der offenen ist im Baum bereits beantwortet.
8. Kein Abhängigkeitszyklus.
9. Ein neuer Vermerk zu gealterter Grundlage, für die Runde 10.
10. Der Datensatz des Web-Betrachters trägt 1069 Zeilen und sechzehn Playmaker-Abschnitte.

## Ausgabe

Portfolio neu erzeugt: `fusion-workbench/portfolio.md`
