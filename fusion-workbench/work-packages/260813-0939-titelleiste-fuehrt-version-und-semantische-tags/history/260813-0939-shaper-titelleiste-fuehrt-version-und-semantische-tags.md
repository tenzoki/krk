# Shaper-Sitzung: Titelleiste mit Name und Version, dazu semantische Versionstags

**Datum:** 2026-08-13
**Agent:** shaper (anticipated-circle mode)
**Status:** Complete
**Ergebnis:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/`

---

## Auftrag

Der Nutzer hat über `/fusion:direct` den Backlog-Eintrag `shared/backlog/260813-0822_*_titelleiste-fuehrt-name-und-version.md` als Entwurf übergeben, Domäne `code`. Der Auftrag nannte zugleich vier Fragen einer vorangegangenen Klärungsrunde als beantwortet und wies an, sie nicht erneut zu stellen.

## Der Entwurf

Der Eintrag verlangt Namen und Version links in der Titelleiste, wo heute nur der Pfad steht, und koppelt daran die Einführung semantischer Versionstags. Seine Begründung für die Kopplung: eine angezeigte Version ohne verbindliche Festlegung wäre eine Zahl ohne Deckung.

## Die vier gesetzten Antworten

1. Eigener linker Bereich in der Titelleiste; ein neues AppKit-Modul dafür ist akzeptiert. Der Pfad bleibt mittig und ungekürzt.
2. Git-Tag `v<version>` bei jeder Auslieferung, ein Abschnitt in `README.md` über die Stufen, und ein Abbruch in `cargo xtask release` bei fehlendem oder unpassendem Tag auf HEAD. `bundle` und `make check` bleiben unangetastet. Das Werkzeug erzeugt keinen Tag.
3. Kein Arbeitsstand im Titel, kein `-dev`-Zusatz, kein neuer Bauschritt.
4. Schreibweise `KRK 0.1.0`. Diese Runde hebt die Version nicht auf 1.0.0.

## Was am Baum nachgesehen wurde

- Der Titel steht heute nur bis zum ersten Pfad auf "KRK" (`crates/krk-ui/src/appkit/fenster.rs:436`, überschrieben von `titel_nachziehen` in `appkit/anwendung.rs:3673`). Ein Titelleisten-Zusatz besteht nirgends.
- Die Version wohnt allein in `[workspace.package]` und erreicht `krk-ui` über `version.workspace = true`. `krk-ui` liest `env!("CARGO_PKG_VERSION")` heute nicht, `krk-bench` und `xtask` an fünf Stellen. Antwort 3 kommt damit ohne neuen Bauschritt aus.
- `git tag -l` liefert nichts. Der Baum trägt bei sieben geschlossenen Runden keinen einzigen Tag.
- `xtask` ruft heute kein `git`. Die Tag-Prüfung wäre die erste solche Stelle.
- `cargo xtask release` steht mit sechs Stationen bis zur Beglaubigung.
- C11 der Runde 2 ist die einzige bestehende Zusage über die Titelleiste; ihr erstes und ihr neuntes Abnahmekriterium sind vom neuen Bereich berührt und bleiben gehalten.
- L4 wird berührt, wie schon von C9 und C11 der Runde 2, und bekommt keine eigene Zahl.
- Das Anwendungsmenü der Runde 7 führt keinen Eintrag "Über KRK".

## Was angelegt wurde

- Der Circle `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/` mit dem Datensatz `_a_circle.md` und den sechs Unterverzeichnissen.
- Drei Entscheidungsdatensätze in `decisions/` dieses Circles, alle `_o_`: der Eintrag "Über KRK", wer den ersten Tag `v0.1.0` setzt, und ob die Prüfung auch einen sauberen Arbeitsbaum verlangt. Keiner davon ändert die Directive; alle drei sind Eingabe für die Klärungsrunde bei der Aktivierung.
- Der Backlog-Eintrag ist auf `_c_` umbenannt und trägt eine `Promoted:`-Zeile auf den Circle.

## Was nicht getan wurde

Kein Spec. In dieser Betriebsart ist der Circle-Datensatz das Artefakt. Keine Aktivierung: der Datensatz bleibt `_a_`, `.active-circle` ist nicht angefasst.

## Anmerkung zur Grundlage

`CLAUDE.md` nennt vier gefahrene Runden und zwei vorgesehene Circles. Der Bestand zeigt sieben geschlossene Runden (alle `_b_`) und einen vorgesehenen Circle, den Web-Betrachter. Die Abweichung ist hier nur vermerkt; die Pflege von `CLAUDE.md` gehört nicht zu dieser Sitzung.
