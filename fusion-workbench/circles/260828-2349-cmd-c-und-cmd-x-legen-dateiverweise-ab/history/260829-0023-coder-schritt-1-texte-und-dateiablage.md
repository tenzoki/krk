# Coder-Sitzung: Schritt 1 der Runde 22, die Texte und die Aufzählung der zwei Befehle

**Date:** 2026-08-29, 260829-0023
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Circle:** `circles/260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab`
**Plan:** `planning/260829-0006_*_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`, Schritt 1
**HEAD:** `a5c7a46`

## Was getan wurde

In `crates/krk-ui/src/kommandos/operationen.rs` ist nach `ablage_weist_ab` der Block „Die Dateiverweise in der Zwischenablage (Runde 22)" entstanden:

- `pub enum Dateiablage { Kopieren, Ausschneiden }` mit `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`; der Doc-Kommentar nennt A4 (Ausschneiden verschiebt nichts, der Unterschied ist ein Satz).
- `pub fn namenszeilen(pfade) -> String` über `eintragsname`, `\n`-getrennt, ohne Schlusszeilenumbruch; der Doc-Kommentar sagt, warum es der Name und nicht der Pfad ist (A3).
- `#[must_use] pub fn ablagemeldung(befehl, pfade) -> String` mit dem Wortlaut aus A6: `kopiert: <Name>` bzw. `<n> Einträge kopiert` über `zahl`, bei `Ausschneiden` mit dem Zusatz ` – verschieben tut das Ziel (Finder: opt+cmd+v)`; vollständiges `match` über `Dateiablage`.
- `#[must_use] pub fn verweise_abgewiesen() -> String` mit `die Zwischenablage hat die Einträge nicht angenommen`.
- `nichts_zu_kopieren` bleibt unverändert und nennt im Doc-Kommentar den weiteren Rufer (`DateifensterQuelle::dateiverweise_ablegen`, Schritt 4).
- Der Modulkopf nennt die vier neuen Namen.

Fünf Proben im Prüfmodul: `namenszeilen_tragen_namen_ohne_umbruch_am_ende_und_ohne_trenner` (C2.1 bis C2.3), `die_ablagemeldung_nach_kopieren_nennt_einen_namen_und_sonst_die_zahl` (C1.8, mit Umlaut), `die_ablagemeldung_nach_ausschneiden_sagt_dass_das_ziel_verschiebt` (C3.2), `die_meldung_nach_ausschneiden_beginnt_mit_der_nach_kopieren` (C3.1, Texthälfte), `der_satz_der_abgewiesenen_verweise_nennt_die_eintraege`.

**Nicht angefasst:** `crates/krk-ui/src/kommandos/mod.rs`. Schritt 1 nennt die Zeile zu `operationen` im Kopf von `mod.rs:29-31`; die Datei gehört in dieser Sitzung dem Coder von Schritt 2, der sie parallel bearbeitet. Der Satz dort steht noch aus.

## Prüfung

- `cargo test -p krk-ui -- operationen::tests`: 53 bestanden, 0 gescheitert; alle fünf neuen Proben grün.
- `cargo fmt --all --check`: exit 0.
- `make check`: rot, und zwar nicht an dieser Datei allein:
  1. `kommandos::zulaessigkeit::tests::die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` (Schritt 2) erwartet zwei Rufer, die erst Schritt 5 anlegt; der Plan nennt sie als planmäßig rot bis Schritt 5.
  2. `cargo clippy -- -D warnings` meldet `dead_code` für `Dateiablage`, `namenszeilen`, `ablagemeldung`, `verweise_abgewiesen` (diese Datei) und für `dateiablage_zulaessig` und `Anspruch::Dateiablage` (Schritt 2): die Rufer entstehen erst mit den Schritten 4 und 5. Kein `#[allow]` gesetzt; der Plan sagt, `make check` gilt am Ende von Schritt 8 und nicht je Schritt.
  3. `clippy::cloned_ref_to_slice_refs` an `zwischenablage.rs:575` und `:592` (Schritt 3, fremde Datei).
