# Playmaker-Lauf — 260813-0859

**Auslöser:** direct-dispatch
**Domain-Gewichtung:** code (aus der Zeile `**Domain:** code` im Auftrag)
**Status:** Complete
**Erzeugtes Portfolio:** `fusion-workbench/portfolio.md`

## Bestand

| Marker | Anzahl |
|---|---|
| `_t_` aktiv | 0 |
| `_a_` vorgesehen | 1 |
| `_c_` kohärent abgeschlossen | 0 |
| `_b_` beschränkt abgeschlossen | 7 |
| `_s_` überholt | 0 |
| `_d_` zurückgestellt | 0 |
| **Summe** | **8** |

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Das ist der
reguläre Zustand nach einem Abschluss; keine Zeigerwarnung ausgelöst.

## Rangfolge der vorgesehenen Circles

Rang 1 von 1: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`. Einziger nicht
abgeschlossener Circle, Lage gegenüber dem Lauf vom 260813-0714 unverändert; die geerbten Bauteile
stehen, der offene Zuschnitt bleibt das stärkste Gegenargument.

Die Standardheuristik der Gewichtung `code` ist **ausgesetzt** und nicht angewandt: sie zählt
allein `_c_` als erfüllte Vorbedingung, alle sieben abgeschlossenen Circles tragen `_b_`, und das
Kennzeichen stünde damit an jedem denkbaren Kandidaten. Die Aussetzung ist im Portfolio benannt.

## Aktivierungsvorschlag

**Kein `## Activation proposal` an den Datensatz angehängt**, und das ist eine Abweichung von der
Regel, die dieser Eintrag ausdrücklich festhält. Der Datensatz trägt bereits vier solche Blöcke,
den jüngsten vom 260813-0714, und an der Lage des Kandidaten hat sich seither nichts bewegt: kein
Circle geschlossen, kein Commit an der Vorschaufläche, keine der offenen Fragen beantwortet. Ein
fünfter Block hätte denselben Inhalt wiederholt und den Datensatz von 674 Zeilen weiter verlängert,
den der Shaper bei der Aktivierung ganz liest. Der Vorschlag steht stattdessen im Portfolio und in
diesem Eintrag. Die Auslassung ist als siebte Warnung im Portfolio offengelegt.

## Ideenspeicher

| Größe | Wert |
|---|---|
| Einträge `_o_` | 1 |
| Einträge `_p_` | 0 |
| darin benannte eigenständige Ideen | 1 (der Eintrag nennt zwei Arbeiten, koppelt sie aber mit Begründung) |
| Dublettengruppen | 0 |
| an `## Warnings` abgegeben (defekt- oder frageförmig) | 0 |

Rang 1 von 1: `shared/backlog/260813-0822_*_titelleiste-fuehrt-name-und-version.md`. Die
Titelleiste soll links Namen und Version führen, und dafür sollen semantische Versionstags
kommen. Empfohlen zum Shapen und **nicht** zum Teilen: die beiden Arbeiten liegen an getrennten
Flächen, aber der Eintrag bindet sie mit dem Argument, eine angezeigte Version ohne verbindliche
Festlegung wäre eine Zahl ohne Deckung. `/fusion:direct` erzeugt daraus die eine Runde, die der
Eintrag meint.

Drei Befunde am Baum, am 260813-0859 gelesen und in die Empfehlung eingegangen:

- Die Version ist bereits einquellig (`[workspace.package] version = "0.1.0"` in der
  Wurzel-`Cargo.toml`, gesetzt in `resources/Info.plist` über `version_einsetzen` in
  `xtask/src/bundle.rs`). Der Baum trägt null git-Tags. Zu klären ist die Vergabe, nicht die
  Quelle.
- Der Eintrag schreibt `KRK <1.0.0>`, die `Cargo.toml` trägt `0.1.0`. Ob die Leiste die
  tatsächliche Zahl zeigt oder das Projekt 1.0.0 ausruft, ist eine Nutzerentscheidung für die
  Runde.
- Die Titelleiste gehört C11 der Runde 2 mit elf Abnahmekriterien, und ihr Modulkopf
  (`crates/krk-ui/src/fenstertitel.rs`) verlangt den ungekürzten absoluten Pfad auf Nutzerwunsch
  vom 260809. Ein Namensteil davor schreibt C11 fort, statt neben ihm zu stehen.

## Zyklen und Weitergabe

**Abhängigkeitszyklus:** keiner. Der Graph über die nicht terminalen Circles hat einen Knoten und
keine Kante innerhalb dieser Menge. Kein `## Dependency warning` angehängt.

**parent-grounding-stale:** kein Ereignis. Seit dem 260813-0714 ist kein Circle auf `_b_`
gegangen, also gibt es keine neue Elternbeziehung zu prüfen. Der Vermerk vom 260813-0714 im
Datensatz des Betrachters gilt fort.

## Warnungen im Portfolio

1. Der Abnahmelauf steht für zwei Runden aus; die Frage nach dem Vordergrund ist seit 260806 offen
   und die einzige, deren Beantwortung die Abschlussart künftiger Runden ändert.
2. Das beglaubigte Bündel unter `target/KRK.app` überlebt keinen gewöhnlichen Entwicklungsbau
   (offener Datensatz `shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-…`).
3. Vier Nutzerfragen der Runde 7 sind gebaut und stehen weiter auf `_o_`.
4. `CLAUDE.md` ist an drei nachgezählten Stellen überholt: vier statt sieben Runden, zwei statt
   einem vorgesehenen Circle, 68 statt 76 Varianten für `Kommando`.
5. Kein Abhängigkeitszyklus.
6. Kein neuer Stale-Vermerk; der vom 260813-0714 gilt fort.
7. Der Datensatz des vorgesehenen Circles trägt 674 Zeilen, davon acht Playmaker-Abschnitte aus
   vier Läufen; dieser Lauf hat deshalb nichts angehängt.
8. Erledigt seit dem letzten Lauf: die Zählung in der Abschlussnotiz der Runde 7 ist am 260813-0725
   berichtigt worden. Die vierte Warnung des Laufs vom 260813-0714 fällt damit weg.

## Geschriebene Dateien

- `fusion-workbench/portfolio.md` (vollständig neu erzeugt)
- `fusion-workbench/shared/history/260813-0859-playmaker-direct-dispatch.md` (dieser Eintrag)

Kein Circle-Datensatz ist in diesem Lauf angefasst worden.
