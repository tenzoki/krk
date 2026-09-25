# Coder: Schritt 3 der Runde 21 — die Zulässigkeitsregel sagt, wen ihr zweiter Eingang bedient

**Date:** 2026-08-29
**Status:** Complete
**Plan:** `planning/260829-1102_*_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, Schritt 3

## Was geändert ist

`crates/krk-ui/src/kommandos/zulaessigkeit.rs`, allein diese Datei, allein Doc-Kommentare; keine Codezeile, kein dritter `Anspruch`-Wert, keine Umbenennung (Entscheidung 2 des Plans).

- Skizze im Modulkopf: die Beschriftung `(copy:, cut:, paste:)` am zweiten Eingang.
- `# Ein Rumpf, zwei Eingaenge (Runde 22)`: der Eingang bedient die drei Selektoren des Menüs „Bearbeiten" (`copy:`, `cut:` seit Runde 22, `paste:` seit Runde 21); neuer Absatz, warum `paste:` keinen dritten Wert bekommt (byteweise derselbe Anspruch, A9) und wie `Dateiablage` seither zu lesen ist; die zwei Frager heißen jetzt `validateMenuItem:` und `Anwendungsdelegierter::bearbeiten_am_dateifenster` (der Name, den Schritt 8 anlegt).
- Doc von `dateiablage_zulaessig`: nennt die drei Selektoren, `cmd+v` neben `cmd+c` als Beispiel für die nicht gebundene Taste, und den Grund gegen einen dritten Wert.
- Doc von `Anspruch::Dateiablage`: „der Ablage-Einhängepunkt des Dateifensters", drei Selektoren, kein eigener Wert für das Einfügen.
- `die_zwei_frager_der_dateiablage_rufen_dieselbe_regel`: Zahl bleibt 2, Doc nennt die zwei Stellen nach der Runde (C3.6).
- `die_dateiablage_wirkt_genau_mit_dem_fokus_im_dateifenster`: Doc-Satz, dass die Tafel das Einfügen mit hält (C3.2, C3.4, C3.5).
- `waehrend_eines_blattes_kommen_genau_diese_vier_durch`: Doc-Satz, dass das Einfügen die Liste nicht erweitert (C3.2).

Hinweis für Schritt 8: der Modulkopf und die Zählprobe nennen `bearbeiten_am_dateifenster` schon jetzt; bis Schritt 8 den Vorspann anlegt, beschreibt die Prosa einen Namen, den der Baum noch nicht trägt.

## Verifikation

- `make check` — exit 2. Rot allein `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` (`krk-core --test verzeichnis`, erwartet nach Schritt 2, zieht Schritt 9 nach); jedes andere Prüfziel grün. `make check` bricht nach `cargo test` ab, deshalb einzeln:
- `cargo clippy --workspace --all-targets -- -D warnings` — Finished, keine Warnung.
- `cargo fmt --all --check` — exit 0.
- `cargo test -p krk-ui zulaessigkeit` — 22 Proben grün.
- Der erste `make check`-Lauf traf den Baum mitten in Schritt 1 (`krk-core` übersetzte nicht, `traegt_die_folge` schon auf `&Muster`, Rufer noch nicht); der zweite Lauf vier Minuten später ist der oben berichtete.

## Plan

Schritt 3 auf `[DONE]`.
