# Playmaker-Lauf 260813-1510 (direct-dispatch)

---
**Status:** Complete
**Agent:** playmaker
**Auslöser:** direct-dispatch (Nutzer, nach dem kohärenten Abschluss der Runde 8)
**Domain-Gewichtung:** code (aus `**Domain:** code` der Anweisung)
**Erzeugtes Portfolio:** `fusion-workbench/portfolio.md`

---

## Bestand

Neun Circles unter `circles/`, gezählt am Marker des Datensatzes:

| Marker | Zahl | Circles |
|---|---|---|
| `_t_` aktiv | 0 | — |
| `_a_` vorgesehen | 1 | `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` |
| `_c_` kohärent abgeschlossen | 1 | `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` |
| `_b_` beschränkt abgeschlossen | 7 | Runden 1 bis 7 |
| `_s_` überholt | 0 | — |
| `_d_` zurückgestellt | 0 | — |

`fusion-workbench/.active-circle` ist nicht vorhanden, kein Datensatz trägt `_t_`. Kein
Zeigerbefund: weder `STALE-POINTER` noch `POINTER-MISMATCH` noch `MULTIPLE-ACTIVE` noch
`MISSING-POINTER`. Der Zustand ist der reguläre nach einem Abschluss.

## Rangfolge der vorgesehenen Circles

**Rang 1 und einziger Kandidat:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.
Der Rang ist keine Auswahl. Der Rang-1-Circle des vorigen Laufs ist als Runde 8 gefahren und
geschlossen, damit ist das Feld auf ein Element zurückgefallen. Am Datensatz des Betrachters hat
sich seit dem 260813-0958 nichts geändert; vor seiner Aktivierung stehen unverändert eine
Untersuchung des Darstellungsmittels und eine Klärungsrunde über sechs Fragen.

**Zur Heuristik.** Die Aussetzungsbegründung der letzten vier Läufe ist weggefallen: es gibt jetzt
einen Circle auf `_c_`. An ihre Stelle tritt eine stärkere für diesen einen Kandidaten. Seine
einzige Circle-Kante führt auf die Runde 1, die `_b_` trägt, und `_b_` ist ein Endzustand
(`rules/circle-records.md`, `### Worked transitions`). Die Prüfung „alle Abhängigkeiten kohärent
abgeschlossen" kann für ihn nie positiv ausfallen, gleich welche Arbeit noch geschieht. Sie ist
deshalb nicht eingerechnet.

## Ideenspeicher

`shared/backlog/` trägt einen Eintrag, `260813-0822_*_titelleiste-fuehrt-name-und-version.md`, und
der steht auf `_c_`: er ist mit der Anlage der Runde 8 geschlossen worden.

- Einträge auf `_o_` oder `_p_`: **0**
- Getrennte Ideen in mehrdeutigen Einträgen: **0** (kein Eintrag zu lesen)
- Benannte Dubletten oder Fastdubletten: **0**
- An `## Warnings` abgegeben als defekt- oder entscheidungsförmig: **0** aus dem Ideenspeicher
- Empfohlener Eintrag zum Shapen: **keiner**

Der leere Speicher ist selbst als Warnung 3 im Portfolio vermerkt, weil er begrenzt, was ein
Portfolio vorlegen kann.

## Am Baum nachgeprüft

Sieben Aussagen, jede gegen den Baum gelesen und nicht aus einem Datensatz übernommen:

1. `git tag -l` liefert nichts. Kein Tag im Baum.
2. `git status --porcelain --untracked-files=no` meldet sechs geänderte verfolgte Dateien, alle
   unter `fusion-workbench/`.
3. `git ls-files` führt `fusion-workbench/monitor`, `.fusion-setup`, `.guard-state/churn.json`
   und `orchestrator-live.md` als verfolgt; `.gitignore` hält nur `.commit-lock/` und
   `.session-marker` draußen.
4. `xtask/src/release.rs:113` und `:127` fragen Tag und Arbeitsbaumstand; `stand_pruefen` ab
   Zeile 226 bricht bei fehlendem Tag **oder** geänderter verfolgter Datei ab. Der Verzicht auf
   einen Pfadfilter ist am Konstantenkopf `GIT_STAND` begründet.
5. `Kommando::KENNUNGEN` trägt 76 Einträge (`crates/krk-core/src/tasten/belegung.rs:566`),
   unverändert gegenüber dem Vermerk vom 260813-0714. Die Runde 8 hat keinen Befehl hinzugefügt.
6. `ersthelfer_gehoert_appkit` (`crates/krk-ui/src/appkit/ereignisse.rs:581`) prüft weiter genau
   `NSTextView`, `NSTextField` und `NSText`; die Editorfläche wird über die hereingereichte
   Prüffunktion `ist_editorflaeche` ausgenommen, also über die Nämlichkeit statt über die Klasse.
7. `crates/krk-ui/src/appkit/mod.rs` führt 28 Modulnamen (vorher 27); neu ist `titelzusatz.rs`.
   Die Struktur `Lage` in `crates/krk-ui/src/kommandos/zulaessigkeit.rs` trägt vier Felder, und
   `immer_erreichbar` führt drei Kommandos.

## Warnungen im Portfolio

1. **Berichtigung einer eigenen Behauptung.** Fünf Portfolios in Folge haben geschrieben, jede
   weitere Runde ende beschränkt, solange die Frage nach dem Vordergrund offen sei. Die Runde 8 ist
   kohärent geschlossen und die Frage steht unverändert offen. Der Fehler war eine Verwechslung
   zweier Abnahmewege: der Datensatz `260806-1303` betrifft die automatische Messstrecke aus
   `krk-bench`, die Runde 8 ist über eine von Hand abgearbeitete Beobachtungsliste abgenommen
   worden.
2. **`cargo xtask release` weist heute aus zwei Gründen ab.** Kein Tag auf HEAD, und sechs
   geänderte verfolgte Dateien. Der zweite Grund ist neu und in keinem Datensatz benannt: vier
   verfolgte Dateien der Werkbank sind flüchtiger Sitzungszustand und werden bei jedem Agentenlauf
   neu geschrieben, die Prüfung hat keinen Pfadfilter. Defektförmig; kein Datensatz angelegt, weil
   das Filen von Defekten außerhalb der Zuständigkeit des Playmakers liegt.
3. **Der Ideenspeicher ist leer.**
4. **Vier Nutzerfragen der Runde 7 sind gebaut und unbeantwortet.** 19 Entscheidungsdatensätze
   offen insgesamt.
5. **Der Defekt am doppelt belegten Ausgabeort `target/KRK.app` steht unverändert offen**, jetzt
   an einem Weg mit einer Station mehr.
6. **`CLAUDE.md` ist an fünf Stellen überholt**, und eine davon trifft seit dem 260813-1415 eine
   falsche Aussage: der Satz, alle Runden endeten beschränkt, und die daran hängende Bemerkung zur
   Rangheuristik.
7. **Kein Abhängigkeitszyklus.** Der Graph über die nicht terminalen Circles hat einen Knoten und
   keine Kante innerhalb der Menge.
8. **Parent grounding stale mit nicht erfüllter Auslösebedingung**, siehe unten.
9. **16 offene Defekte im Circle der Runde 8**, 70 projektweit, 10 im gemeinsamen Speicher.

## Angehängte Abschnitte an Circle-Datensätzen

**`## Dependency warning`:** keiner. Es besteht kein Zyklus.

**`## Parent grounding stale`:** einer, an
`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`.

```
parent-grounding-stale: parent=260804-0933-eingebauter-web-betrachter-im-vorschaufenster child=260813-0939-titelleiste-fuehrt-version-und-semantische-tags
```

Die Auslösebedingung nach der Regel ist **nicht erfüllt**, und das ist im Vermerk selbst und in
Warnung 8 des Portfolios offen benannt: das Kind trägt `_c_` und nicht `_b_`, und der Abschnitt
`## Grounding snapshot` des Elternteils zitiert es nicht, weil er vom 260804 stammt. Angehängt
worden ist er, weil die Runde 8 die Zulässigkeitsregel erweitert hat, durch die jeder Befehl des
Betrachters laufen wird. Der Vermerk trägt zwei Feststellungen: die Regel fragt vier Dinge statt
drei, und die Behandlung einer nicht über ihre Klasse erkennbaren Ansicht hat mit der
hereingereichten Prüffunktion für die Editorfläche einen gebauten Präzedenzfall bekommen. Die
zweite ist die erste Feststellung seit vier Läufen, die für diesen Circle spricht.

**`## Activation proposal`:** einer, am selben Datensatz. Er benennt offen, dass der Rangwechsel
zurück auf 1 kein Befund zu seinen Gunsten ist, sondern ein leergeräumtes Feld, und dass die eine
Änderung zu seinen Gunsten am Projekt liegt und nicht an ihm: eine Runde, die ihn ausführt, kann
kohärent enden, sofern der Nutzer dieselbe Handabnahme fährt wie in der Runde 8.

Der Datensatz des Betrachters trägt nach diesem Lauf 782 Zeilen und zehn Playmaker-Abschnitte aus
fünf Läufen. Die Länge ist selbst ein Punkt: sie wächst mit jedem Lauf, in dem der Circle
vorgesehen bleibt, ohne dass an ihm gearbeitet würde.

## Keine Umbenennung, kein Zeiger

Der Playmaker hat keinen Marker umbenannt, `.active-circle` nicht geschrieben, keinen
Ideeneintrag angefasst, keinen Defekt- und keinen Entscheidungsdatensatz angelegt und keinen
Agenten beauftragt. Geschrieben wurden drei Dateien: die beiden Abschnitte am Datensatz des
Betrachters, `fusion-workbench/portfolio.md` und diese Datei.
