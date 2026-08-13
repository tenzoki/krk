# Playmaker-Lauf 260813-2203 (direct-dispatch)

---
**Status:** Complete
**Agent:** playmaker
**Auslöser:** direct-dispatch (Nutzer)
**Domain-Gewichtung:** code (aus `**Domain:** code` der Anweisung)
**Erzeugtes Portfolio:** `fusion-workbench/portfolio.md`

---

## Bestand

Neun Circles unter `circles/`, gezählt am Marker des Datensatzes. Unverändert gegenüber dem
Lauf vom 260813-1510; kein Datensatz hat seither den Zustand gewechselt.

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
`MISSING-POINTER`. Der reguläre Zustand nach einem Abschluss.

## Rangfolge der vorgesehenen Circles

**Rang 1 und einziger Kandidat:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.
Der Rang ist keine Auswahl, sondern das Ergebnis eines einelementigen Feldes. Am Circle selbst
hat sich seit dem 260813-0958 nichts geändert: derselbe Zuschnitt, dieselben sechs Fragen der
Klärungsrunde, dieselbe Untersuchung des Darstellungsmittels vor dem Plan.

Geändert hat sich der Aktivierungszeitpunkt, und zwar am Projekt und nicht am Circle. Der
Vermerk vom 260813-1510 stellte ihm zwei Nutzerschritte voran, die den Auslieferungsweg
anhielten. Beide sind erledigt.

Die Abhängigkeitsprüfung bleibt für diesen Circle ausgenommen, mit der Begründung des vorigen
Laufs: seine einzige Kante führt auf einen beschränkten Abschluss, und der ist ein Endzustand.
Ein Kriterium, dessen Wert keine künftige Arbeit ändern kann, trägt kein Rangsignal.

## Ideenspeicher

`shared/backlog/` trägt drei Einträge, zwei davon auf `_o_` und beide am 260813-2033 vom Nutzer
gefilt (Commit `d046d9e`). Der Speicher war beim vorigen Lauf leer; das war dort Warnung 3 und ist
erledigt.

- Einträge auf `_o_`: **2**; auf `_p_`: **0**
- Getrennte Ideen in mehrdeutigen Einträgen: **0**. Beide Einträge tragen genau eine Idee. Der
  Notizzettel führt sechs offene Punkte, aber es sind Fragen an eine Klärungsrunde und keine
  getrennten Vorhaben; ein Split wäre falsch.
- Benannte Dubletten oder Fastdubletten: **0**
- An `## Warnings` abgegeben als defekt- oder entscheidungsförmig: **0**. Beide Einträge sind
  Vorhaben.
- **Empfohlener Eintrag zum Shapen:**
  `shared/backlog/260813-2033_*_ein-scratchpad-das-per-taste-mittig-erscheint-und-sich-selbst-sichert.md`
  — der einzige Kandidat des Portfolios, der ohne vorgeschaltete Untersuchung geshaped werden
  kann; alle Bauteile liegen im Baum.
- Rang 2: `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
  — zurückgestellt hinter eine Prüfung, die der Eintrag selbst benennt und die ihn auflösen kann.

## Am Baum nachgeprüft

Neun Aussagen, jede gegen den Baum gelesen und nicht aus einem Datensatz übernommen:

1. `git tag -l` liefert `v0.1.0`, `v0.2.0`, `v0.2.1`. Beim vorigen Lauf war die Liste leer.
2. `git tag --points-at HEAD` liefert `v0.2.1`; `Cargo.toml:13` führt `version = "0.2.1"`.
   Station 1 des Auslieferungswegs vergleicht genau diese beiden Werte.
3. `git status --porcelain --untracked-files=no` ist leer. Der flüchtige Sitzungszustand der
   Werkbank steht seit `7537ee5` und `5ae3800` in `.gitignore`, `.guard-state/` ganz eingeschlossen.
   `git ls-files fusion-workbench/.guard-state/` ist leer.
4. `git rev-list -n1 v0.1.0` liefert `3a0a4bf`, den Abschlusscommit der Runde 8. Damit ist deren
   Kriterium C3.15 erfüllt, das die `## Closure note` als den einen offenen Nutzerschritt benennt.
5. `xtask/src/release.rs:121` ruft `bundle::vorbereiten()`, und `xtask/src/bundle.rs:50` setzt
   `BUENDELNAME = "KRK.app"` unter `target/`. Der Defekt am doppelt belegten Ausgabeort besteht
   unverändert.
6. `Kommando::KENNUNGEN` trägt 76 Einträge (`crates/krk-core/src/tasten/belegung.rs:566`),
   unverändert gegenüber dem 260813-1510. `crates/krk-ui/src/appkit/mod.rs` führt 28 Modulnamen,
   ebenfalls unverändert.
7. `resources/default-keymap.toml` führt 82 Funktionen mit 88 Tastenkombinationen, drei Funktionen
   davon ohne Taste. Die Zahlen des Ideeneintrags zum Notizzettel stimmen.
8. Alle vier Cmd-Ebenen des Buchstabens `e` sind vergeben: `cmd+e`, `shift+cmd+e`, `opt+cmd+e`,
   `ctrl+cmd+e` (`resources/default-keymap.toml`, Zeilen 691, 700, 708, 733).
9. Der Kommentar an `bearbeiten` (`resources/default-keymap.toml:164-174`) begründet ausdrücklich,
   dass kein Cmd-Kürzel danebensteht. Der Ideeneintrag zum Editor-Einstieg kippt diese Überlegung,
   und er benennt das selbst.

## Warnungen im Portfolio

1. **`CLAUDE.md` beschreibt ein Projekt mit vier Runden, und es sind acht.** Die folgenreichste
   Warnung dieses Laufs. Falsch sind die Rundentabelle, der Satz „Alle vier Runden sind als
   beschränkter Abschluss geschlossen" samt der Bemerkung zur Rangheuristik, der Absatz „Zwei
   Circles sind vorgesehen und nicht gefahren" und die Zeile zur Statusleiste auf Rang 1. Zwei
   falsche Zahlen haben Defektdatensätze, die Rundenzahl hat keinen. Kein Datensatz angelegt: das
   Filen von Defekten liegt außerhalb der Zuständigkeit des Playmakers.
2. **Das letzte offene Abnahmekriterium der Runde 8 ist erfüllt, und kein Datensatz sagt es.**
   C3.15, Arbeit für den `reconciler`.
3. **Der Defekt am doppelt belegten Ausgabeort `target/KRK.app` steht unverändert offen.**
4. **19 Entscheidungsdatensätze offen, einer beantwortet und nicht umgesetzt.**
5. **70 offene Defekte**, 10 im gemeinsamen Speicher; 25 in der Runde 6, 16 in der Runde 8, 14 in
   der Runde 7.
6. **Kein Abhängigkeitszyklus.** Ein Knoten, keine Kante innerhalb der nicht terminalen Menge.
7. **Kein neuer Vermerk zu gealterter Grundlage**, weil kein Circle den Zustand gewechselt hat.
8. **Der Datensatz des Web-Betrachters trägt 820 Zeilen und elf Playmaker-Abschnitte aus sechs
   Läufen.**

Warnung 2 des vorigen Laufs, `cargo xtask release` weise aus zwei Gründen ab, ist erledigt und
steht deshalb nicht mehr im Portfolio. Warnung 3 des vorigen Laufs, der Ideenspeicher sei leer,
ebenfalls.

## Angehängte Abschnitte an Circle-Datensätzen

**`## Dependency warning`:** keiner. Es besteht kein Zyklus.

**`## Parent grounding stale`:** keiner. Seit dem 260813-1510 hat kein Circle den Zustand
gewechselt, also gibt es kein neu beschränkt abgeschlossenes Kind. Der Lauf vom 260813-1510 hatte
einen Vermerk auf nicht erfüllter Auslösebedingung angehängt und das offen benannt; das zu
wiederholen hätte an einem 782 Zeilen langen Datensatz nichts hinzugefügt.

**`## Activation proposal`:** einer, an
`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`. Bewusst kurz und
mit einem Satz, der das sagt: er trägt die eine Änderung nach und wiederholt den Vorschlag vom
260813-1510 nicht. Die Änderung ist der Wegfall der beiden Nutzerschritte am Auslieferungsweg,
dazu der Nachtrag zu C3.15 der Runde 8. Der Datensatz steht danach bei 820 Zeilen.

## Keine Umbenennung, kein Zeiger

Der Playmaker hat keinen Marker umbenannt, `.active-circle` nicht geschrieben, keinen
Ideeneintrag angefasst, keinen Defekt- und keinen Entscheidungsdatensatz angelegt und keinen
Agenten beauftragt. Geschrieben wurden drei Dateien: der Abschnitt am Datensatz des Web-Betrachters,
`fusion-workbench/portfolio.md` und diese Datei.
