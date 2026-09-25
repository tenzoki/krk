# Ontocoder-Sitzung — 260828-1210

**Aufgabe:** Schritt 3 des Plans der Runde 20 — die drei Einträge in der Auslieferungsbelegung
**Circle:** 260827-2028-vorschau-rendert-pdf-als-betrachter (aktiv)
**Plan:** `planning/260828-0712_p_plan-vorschau-rendert-pdf-als-betrachter.md`, Schritt 3; Spec C1.3, C3.3, C3.4; Entscheidung `decisions/260827-2028_a_welche-tasten-bekommen-zoom-und-seitensprung-des-pdf-betrachters.md` (Antwort 1b)
**Status:** Complete

## Was geändert wurde

- `resources/default-keymap.toml`:
  - Kopfzeile „Ausgeliefert sind 85 Funktionen mit zusammen 90 Kombinationen" → 88 und 93 (nachgezählt: 88 `[[funktion]]`, 93 Einträge in `tasten`).
  - Zeile „Tastennamen:" nennt `plus` und `minus`, „über das Zeichen, nicht über die Stelle" (C3.3).
  - Neuer Block `# ── Runde 20: der PDF-Betrachter` hinter `zwischenablage_springen` (Ende des C10-Abschnitts) mit drei Einträgen: `vorschau_vergroessern` / „Vorschau vergrößern" / `["cmd+plus"]`, `vorschau_verkleinern` / „Vorschau verkleinern" / `["cmd+minus"]`, `vorschau_ausgangsgroesse` / „Vorschau in Ausgangsgröße" / `["cmd+0"]`. Der Kommentar nennt Antwort 1b mit Datensatz, sagt, warum Bild-auf, Bild-ab, Pos1 und Ende keinen Eintrag bekommen (unzulässig in der Vorschau, laufen an AppKit, blättern dort; C1.3), und dass `plus` und `minus` über das Zeichen gefunden werden. Freiheit der drei Kombinationen am 260828 nachgeprüft (C3.4).
  - Kein vorhandener Eintrag geändert.

## Verifikation

`make check`, erster Lauf: exit 2. Die drei vor Schritt 3 roten Proben (`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`, `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`, `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`) grün; `jede_ausgelieferte_kombination_traegt_die_kennung_ihrer_tastensorte` grün, der parallele Coder hatte `tests/belegung.rs` schon umgestellt. Rot war allein `belegungsmodell::tests::die_beschriftung_nennt_die_taste_auf_einer_deutschen_tastatur` (erwartete „+" statt „Plus" für `cmd+plus`); der parallele Coder hat die Probe in `belegungsmodell.rs` inzwischen auf den Tastennamen umgestellt, Einzellauf danach grün. Zweiter `make check` gegen den Baum mit der Coder-Umstellung: exit 0, keine rote Probe.

## Nicht angefasst

`crates/krk-core/tests/belegung.rs`, `crates/krk-ui/src/belegungsausgabe.rs`, `crates/krk-ui/src/belegungsmodell.rs` (paralleler Coder). Prosa-Zahlen in `belegungsausgabe.rs` („79 mit Kommando", „85 Funktionen") sind Sache des Coders.
