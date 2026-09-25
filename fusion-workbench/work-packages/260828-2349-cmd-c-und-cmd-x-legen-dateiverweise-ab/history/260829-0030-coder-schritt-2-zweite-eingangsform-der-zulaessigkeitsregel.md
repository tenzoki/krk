# Coder: Schritt 2 der Runde 22, die zweite Eingangsform der Zulässigkeitsregel

**Date:** 2026-08-29
**Status:** Complete
**Plan:** `planning/260829-0006_p_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`, Schritt 2
**Agent:** coder

## Was gebaut ist

- `crates/krk-ui/src/kommandos/zulaessigkeit.rs`
  - `enum Anspruch { Kommando(Kommando), Dateiablage }`, privat, `Copy`, mit `wirkungsbereich`, `waehrend_blatt_erlaubt`, `immer_erreichbar` als je vollständigem `match`; `Dateiablage` antwortet `Dateifenster`, `false`, `false`.
  - Der Rumpf von `zulaessig` ist unverändert nach `fn gestattet(anspruch, lage)` gewandert; `zulaessig(kommando, lage)` ist der Einzeiler `gestattet(Anspruch::Kommando(kommando), lage)`, Signatur und Doc-Kommentar bleiben.
  - `#[must_use] pub fn dateiablage_zulaessig(lage) -> bool` als zweite Hülle, Doc-Kommentar nennt A11 und den Grund aus Planerantwort 2.
  - `immer_erreichbar` unverändert öffentlich.
  - Modulkopf: Skizze mit zwei Eingängen und einem Rumpf, neuer Abschnitt `# Ein Rumpf, zwei Eingaenge (Runde 22)` mit den zwei Fragern des Dateiablage-Eingangs.
  - Proben: `die_zulaessigkeitsregel_ist_genau_einmal_erklaert` zählt zwei Nadeln mit Erwartung je 1; `beide_frager_rufen_die_eine_regel` unverändert bei zwei; neu `die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` (Erwartung 2, rot bis Schritt 5); neu `die_dateiablage_wirkt_genau_mit_dem_fokus_im_dateifenster` über `Fokus::ALLE`, `OHNE_HINDERNIS` und `HINDERNISSE`; `waehrend_eines_blattes_kommen_genau_diese_vier_durch` bekommt zwei Zusicherungen, dass `dateiablage_zulaessig` bei stehendem Blatt `false` ist.
- `crates/krk-ui/src/kommandos/mod.rs`: der Absatz „Zwei Frager stellen sie" nennt den zweiten Eingang und seine zwei Frager.

## Ein Umweg

Die erste Fassung des Doc-Kommentars an `die_zulaessigkeitsregel_ist_genau_einmal_erklaert` schrieb die Nadel ausgeschrieben in die Prosa; die Probe zählte dadurch zwei Erklärungen. Der Kommentar nennt die Nadeln jetzt als Links und nicht als Text, wie der Absatz darüber es für die zusammengesetzte Nadel begründet.

## Verifikation

- `cargo test -p krk-ui -- --skip die_zwei_frager_der_dateiablage_rufen_dieselbe_regel`: exit 0, 849 Proben grün.
- `cargo test -p krk-ui die_zwei_frager_der_dateiablage`: rot, wie der Plan es ankündigt (die zwei Rufer entstehen erst in Schritt 5).
- `cargo fmt --all --check`: exit 0.
- `make check`: exit 2, hält am Prüfschritt an genau dieser einen Probe an. Der Lint-Schritt wäre danach ebenfalls rot: `dead_code` an `dateiablage_zulaessig` und `Anspruch::Dateiablage` (und an den vier Neuzugängen aus Schritt 1 in `operationen.rs`), bis Schritt 5 die Rufer anlegt. Der Plan legt `make check` auf das Ende von Schritt 8.

Kein git-Kommando, kein Commit.
