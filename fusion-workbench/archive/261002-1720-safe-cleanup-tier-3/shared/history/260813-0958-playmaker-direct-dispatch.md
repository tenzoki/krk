# Playmaker-Lauf — 260813-0958

**Auslöser:** direct-dispatch
**Domain-Gewichtung:** code (aus der Zeile `**Domain:** code` im Auftrag)
**Status:** Complete
**Erzeugtes Portfolio:** `fusion-workbench/portfolio.md`

## Bestand

| Marker | Anzahl |
|---|---|
| `_t_` aktiv | 0 |
| `_a_` vorgesehen | 2 |
| `_c_` kohärent abgeschlossen | 0 |
| `_b_` beschränkt abgeschlossen | 7 |
| `_s_` überholt | 0 |
| `_d_` zurückgestellt | 0 |
| **Summe** | **9** |

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Das ist der
reguläre Zustand nach einem Abschluss; keine Zeigerwarnung ausgelöst.

Gegenüber dem Lauf vom 260813-0859: ein Circle hinzugekommen,
`260813-0939-titelleiste-fuehrt-version-und-semantische-tags`, angelegt vom Shaper aus dem
Ideeneintrag, den jener Lauf zum Shapen empfohlen hat. Kein Circle hat den Marker gewechselt.

## Rangfolge der vorgesehenen Circles

**Rang 1: `260813-0939-titelleiste-fuehrt-version-und-semantische-tags`.** Die Vorarbeit ist eine
Klärungsrunde über drei schmale Fragen und keine Untersuchung, der Circle hat keine Vorbedingung
an einem anderen Circle, und seine vier Stunden alte Grundlage ist bei diesem Lauf an vier
Tatsachenaussagen gegen den Baum gelesen worden.

**Rang 2: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.** Sechs Fragen und eine
Untersuchung des Darstellungsmittels vor der Aktivierung; die Untersuchung ist der teurere Posten
und der Grund für den zweiten Rang. Der Rangwechsel ist kein Befund gegen den Circle: es ist der
erste Lauf seit dem 260807, in dem ihm überhaupt ein zweiter Kandidat gegenübersteht.

Die Standardheuristik der Gewichtung `code` bleibt **ausgesetzt** und nicht angewandt: sie zählt
allein `_c_` als erfüllte Vorbedingung, alle sieben abgeschlossenen Circles tragen `_b_`, und das
Kennzeichen stünde damit an jedem denkbaren Kandidaten. Neu ist, dass die Abhängigkeitsprüfung für
den Rang-1-Circle trotzdem eine Auskunft gibt: sein Abschnitt `## Dependencies` nennt keinen
Circle, also ist die Prüfung leer erfüllt statt unentscheidbar. Die Aussetzung ist im Portfolio
benannt.

**Am Baum nachgeprüft** (vier Aussagen des Grounding des Rang-1-Circles, alle halten):
`git tag -l` liefert null Tags; `[workspace.package] version = "0.1.0"` in der Wurzel-`Cargo.toml`;
`NSTitlebarAccessoryViewController` kommt unter `crates/` nicht vor;
`crates/krk-ui/src/appkit/mod.rs` führt 27 Modulnamen. Ergänzend: kein Eintrag „Über KRK" in
`menuemodell.rs`, `appkit/menue.rs` oder `resources/default-keymap.toml`.

## Aktivierungsvorschlag

**Angehängt an `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/_a_circle.md`,
Abschnitt `## Activation proposal`.** Der erste an diesem Datensatz. Er nennt den Grund für den
Rang, die vier nachgeprüften Grundlagenaussagen, die Dringlichkeit der zweiten offenen Frage (ohne
eine Festlegung, wer `v0.1.0` setzt, ist `cargo xtask release` ab dem Abschluss der Runde
abweisend), und zwei Gegenargumente.

**Nichts angehängt an `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`.**
Zwei Gründe: der Datensatz trägt bereits vier Vorschlagsblöcke und vier Stale-Vermerke bei 674
Zeilen, und der Circle ist seit diesem Lauf nicht mehr der empfohlene Kandidat. Die Regel bindet
den Vorschlag an den Rang-1-Circle.

## Ideenspeicher

| Größe | Wert |
|---|---|
| Einträge `_o_` | 0 |
| Einträge `_p_` | 0 |
| Einträge `_c_` | 1 |
| darin benannte eigenständige Ideen | 0 (kein lebender Eintrag) |
| Dublettengruppen | 0 |
| an `## Warnings` abgegeben (defekt- oder frageförmig) | 0 |

Keine Empfehlung, weil der Speicher leer ist. Der einzige Eintrag,
`shared/backlog/260813-0822_*_titelleiste-fuehrt-name-und-version.md`, steht auf `_c_` und ist mit
der Anlage des Rang-1-Circles geschlossen. Die Empfehlung des Laufs vom 260813-0859 ist damit
ausgeführt.

## Zyklen und Weitergabe

**Abhängigkeitszyklus:** keiner. Der Graph über die nicht terminalen Circles hat zwei Knoten und
keine Kante innerhalb dieser Menge. Der Rang-1-Circle trägt keine Circle-Kante; die einzige Kante
des Betrachters zeigt auf die Runde 1 und damit aus der Menge heraus. Kein
`## Dependency warning` angehängt.

**parent-grounding-stale:** kein Ereignis. Der Grenzfall ist geprüft und offen benannt: der neue
Circle `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` nennt unter
`## Dependencies` die Runde 7 (`260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz`,
`_b_`). Kein Vermerk angehängt, erstens weil sein `## Grounding snapshot` weder den
Verzeichnisnamen noch den Artefakt der Abschlussnotiz zitiert, zweitens und schwerer wiegend, weil
der Circle nach dem Abschluss der Runde 7 angelegt worden ist und seine Grundlage genau den Baum
liest, den jene Runde hinterlassen hat. Ein Stale-Vermerk wäre unwahr. Der Vermerk vom 260813-0714
im Datensatz des Betrachters gilt fort.

## Warnungen im Portfolio

1. Der Abnahmelauf steht für zwei Runden aus; die Frage nach dem Vordergrund ist seit 260806 offen
   und die einzige, deren Beantwortung die Abschlussart künftiger Runden ändert.
2. Das beglaubigte Bündel unter `target/KRK.app` überlebt keinen gewöhnlichen Entwicklungsbau
   (`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-…`, offen), und die
   Tag-Prüfung des Rang-1-Circles kommt als weitere Station an denselben Weg.
3. Vier Nutzerfragen der Runde 7 sind gebaut und stehen weiter auf `_o_`.
4. `CLAUDE.md` ist an vier nachgezählten Stellen überholt: vier statt sieben Runden, 68 statt 76
   Varianten für `Kommando`, und neu die Aufstellung der vorgesehenen Circles, die die seit dem
   260812-0820 abgeschlossene Statusleiste nennt und ihr Rang 1 gibt.
5. Kein Abhängigkeitszyklus (mit Skizze des Graphen).
6. Kein Stale-Vermerk in diesem Lauf; der Grenzfall am neuen Circle ist offengelegt.
7. Am Datensatz des Betrachters ist wieder nichts angehängt, jetzt zusätzlich, weil er nicht mehr
   Rang 1 trägt.

Weggefallen gegenüber dem Lauf vom 260813-0859: die achte Warnung jenes Laufs (die berichtigte
Zählung in der Abschlussnotiz der Runde 7) ist erledigt und nicht mehr geführt.

## Geschriebene Dateien

- `fusion-workbench/portfolio.md` (vollständig neu erzeugt)
- `fusion-workbench/circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/_a_circle.md`
  (angehängt: `## Activation proposal`)
- `fusion-workbench/shared/history/260813-0958-playmaker-direct-dispatch.md` (dieser Eintrag)
