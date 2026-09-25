# Coder: Schritt 4 der Runde 22, die Ablage an der Tabelle

**Datum:** 260829-0037
**Agent:** coder
**Plan:** `planning/260829-0006_p_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`, Schritt 4
**Status:** Complete

## Was gebaut ist

- `crates/krk-ui/src/appkit/tabelle.rs`: `pub fn dateiverweise_ablegen(&self, befehl: Dateiablage)` neben `eintragspfad_kopieren`, genau die Signatur, die Schritt 5 als `self.dateifenster(aktiv).quelle().dateiverweise_ablegen(befehl)` ruft. Rumpf nach Plan: `betroffene_eintraege()`; leere Menge zu `nichts_zu_kopieren()` (C1.7, C3.6); sonst `namenszeilen`, `dateiverweise_schreiben` im `if`, `ablagemeldung` oder `verweise_abgewiesen()` (C5.3). Kein Auftrag, keine Aenderung an Markierung oder Auswahl (C1.6, C3.3, A4, Constraint 6). Der Doc-Kommentar nennt fuer die Abnehmer von `betroffene` den `grep` aus Entscheidung 7 und keine Ordnungszahl. Import um `Dateiablage` erweitert. Modulkopf: der Absatz zur Statuszeile nennt die Dateiablage als weiteren Weg durch `befehlsantwort_zeigen` (die Planangabe `:201` zeigt auf den `clickedRow`-Absatz; einen Absatz ueber die Pfadkopierer hat der Kopf nicht, deshalb der Satz beim Absatz ueber die Statuszeile).
- `crates/krk-ui/src/appkit/zwischenablage.rs`: `#[allow(dead_code)]` samt dem dreizeiligen Kommentar an `dateiverweise_schreiben` entfernt; der Rufer steht jetzt.

## Verifikation

- `cargo test -p krk-ui -- tabelle`: exit 0 (20 Proben).
- `make check`: exit 0, `alle vier gruen`. Waehrend des Laufs sind Schritt 5 (`anwendung.rs`) und Schritt 7 (`betrachter.rs`) des parallel arbeitenden Coders gelandet: `die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` und `nspasteboard_steht_nicht_im_betrachter_und_copy_und_cut_stehen_an_genannten_stellen` sind gruen, und kein `dead_code` steht mehr an der Kette. Der erste Lauf davor (vor Schritt 5) zeigte neun `dead_code`-Warnungen an der ganzen Kette einschliesslich `dateiverweise_ablegen` selbst und war ebenfalls exit 0, weil `make check` clippy nicht unter `-D warnings` faehrt.
- Nichts ausser dem Erwarteten war rot.
