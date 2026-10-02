# Playmaker: Portfolio nach dem Abschluss der Editor-Runde

**Datum:** 2026-08-10, 14:39
**Status:** Complete
**Anlass:** Direktauftrag des Nutzers nach dem Übergang der Editor-Runde von `_t_` auf `_b_`
**Domain-Gewichtung:** code (aus der Zeile `**Domain:** code` des Auftrags)
**Erzeugtes Portfolio:** `fusion-workbench/portfolio.md`

## Bestand

Vier Circle-Verzeichnisse, vier Datensätze, die Marke von jedem am Dateinamen gelesen.

| Marke | Zahl | Circles |
|---|---|---|
| `_a_` vorgesehen | 2 | `260809-2040-tastenbelegung-als-markdown-in-downloads`, `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` |
| `_t_` aktiv | 0 | — |
| `_c_` kohärent geschlossen | 0 | — |
| `_b_` beschränkt geschlossen | 2 | `260807-2116-eingebauter-editor-mit-textmarken`, `260802-0842-krk-mac-dateimanager-editor-git` |
| `_s_` überholt | 0 | — |
| `_d_` zurückgestellt | 0 | — |

`fusion-workbench/.active-circle` fehlt, kein Datensatz trägt `_t_`. Regulärer Zustand nach
einem Abschluss; keine Zeigerwarnung.

## Rangfolge

**Rang 1: `260809-2040-tastenbelegung-als-markdown-in-downloads`.** Die Belegungsausgabe baut
eine zweite Ausgabeform an einer bestehenden Aufbereitung, ihre Grundlage ist einen Tag alt
und rechnet die Änderungen der Editor-Runde schon ein, und ihre fünf offenen Fragen sind
Nutzerwahlen mit Möglichkeiten und Empfehlung, keine Untersuchungen.

**Rang 2: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.** Größerer Zuschnitt,
kein festgelegtes Mittel der Darstellung, Grundlage vom 260804 und damit vor der Editor-Runde.

**Die Gewichtung `code` zählt in die andere Richtung**, und der Bericht hält das fest: ein
zitierter offener Entscheidungsdatensatz beim Web-Betrachter gegen fünf bei der
Belegungsausgabe. Die Begründung, warum der Zählwert hier die falsche Größe misst, steht im
Portfolio unter Rang 1 und im Datensatz unter `## Activation proposal`.

**Die festgehaltene Nutzerwahl vom 260807-1930 ordnet nicht mehr.** Sie stellte den Editor
gegen den Web-Betrachter; der Editor hat gewonnen und ist geschlossen, und die Belegungsausgabe
entstand erst am 260809-2040. Der Lauf vom 260807-2125 folgte jener Wahl; dieser Lauf konnte
es nicht und steht deshalb auf dem Dateibestand.

## Am Code nachgeprüft

- `resources/default-keymap.toml` trägt 71 Blöcke `[[funktion]]` — die Zahl des Grounding der
  Belegungsausgabe hält am gebauten Stand.
- `crates/krk-ui/src/belegungsmodell.rs:73` führt `Funktionsbereich` mit dem neunten Wert
  `Editor`.
- Alle fünf Entscheidungsdatensätze in
  `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/` tragen `_o_`.

## Angefügte Abschnitte

- `## Parent grounding stale` an
  `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/_a_circle.md`
- `## Activation proposal` an denselben Datensatz

Kein Datensatz wurde umbenannt, kein Zeiger geschrieben, kein Plan, Defekt oder
Entscheidungsdatensatz angefasst.

## Ereignisse

- `parent-grounding-stale: parent=260809-2040-tastenbelegung-als-markdown-in-downloads child=260807-2116-eingebauter-editor-mit-textmarken`

## Warnungen im Portfolio

1. `beide-abschluesse-beschraenkt` — kein Circle hat je `_c_` erreicht, beide geschlossenen
   Runden tragen `_b_`. Folge: das Kriterium „alle Abhängigkeiten stehen auf `_c_`" ist in
   diesem Projekt von keinem Circle erfüllbar und muss inhaltlich gelesen werden. Beide
   Beschränkungen haben denselben Kern, die offene Frage
   `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`.
2. Der Fortpflanzungsvermerk an der Belegungsausgabe (siehe oben).
3. Die Grundlage des Web-Betrachters ist gealtert, ohne den Vermerk auszulösen: der Datensatz
   nennt die Editor-Runde nicht, weil er drei Tage vor ihr entstand. Sachlich beschreibt er
   das Vorschaufenster im Zustand der Runde 1. Kein Abschnitt angefügt, weil die
   Auslösebedingung ein Zitat ist und keines vorliegt.
4. Der Spec der Editor-Runde trägt `_o_`, während ihr Plan auf `_c_` steht und der Circle
   geschlossen ist.
5. Elf offene Defekte liegen in terminalen Circles und haben keinen Träger, dazu drei im
   gemeinsamen Speicher.
6. Drei Verweise in Spec und Plan der Editor-Runde zeigen auf `_t_circle.md`, das jetzt
   `_b_circle.md` heißt.
7. Keine Abhängigkeitsschleife: zwei Kanten, beide auf einen terminalen Knoten ohne
   ausgehende Kante.
8. Der Zeigerzustand ist regulär.

## Abhängigkeitsgraph

```
260809-2040-tastenbelegung-als-markdown-in-downloads  ──▶ 260802-0842-krk-mac-…-git  (_b_)
260804-0933-eingebauter-web-betrachter-im-vorschau…   ──▶ 260802-0842-krk-mac-…-git  (_b_)
260807-2116-eingebauter-editor-mit-textmarken (_b_)   ──▶ 260802-0842-krk-mac-…-git  (_b_)

Kante = Eintrag in ## Dependencies. Der Zielknoten ist terminal und zeigt auf nichts.
```
