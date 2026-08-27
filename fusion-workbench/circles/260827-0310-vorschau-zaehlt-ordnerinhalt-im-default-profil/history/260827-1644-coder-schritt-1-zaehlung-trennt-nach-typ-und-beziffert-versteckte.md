# Coder: Schritt 1 der Runde 19 — der Baustein `zaehlung` trennt nach Typ und beziffert die versteckten

**Datum:** 260827-1644
**Plan:** `planning/260827-1322_p_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md`, Schritt 1
**Kriterien:** C2.3, C2.4, C2.5, C2.8, C2.9, C2.10, C3.3, C3.7
**Status:** Complete

## Was gebaut ist

- `crates/krk-core/src/leseprofil/mod.rs`: `use crate::verzeichnis::Typ`; `Baustein::Zaehlung` traegt `typ: Option<Typ>` und `versteckt: bool` mit Doc-Kommentaren; `Wert` traegt den siebten Wert `ZahlMitVersteckten { zahl, versteckt }` (Doc: die Zahl vor der Klammer schliesst die versteckten ein, C2.3; Klammer auch bei null, C2.4; leerer Ordner „0 (0)", C2.5); `Wert::als_text` bekommt den Zweig `"{zahl} ({versteckt})"`, weiterhin ohne Auffangzweig. Der Doc-Kommentar an `Wert` spricht jetzt vom achten statt vom siebten Wert, der den Bau anhaelt.
- `crates/krk-core/src/leseprofil/bausteine.rs`: `zaehlen(stand, muster, typ, versteckt)` laeuft in einem Durchgang mit zwei Zaehlern; die private Funktion `trifft` entscheidet Muster **und** Typ am vom Leser gemeldeten Typ (C2.9). Reihenfolge der Zweige: abgeschnitten → `UeberGrenze(treffer)` unabhaengig von `versteckt` (C2.10); sonst `versteckt` → `ZahlMitVersteckten`, sonst `Zahl` (C3.3). `Lauf::rechnen` bindet die zwei Felder und reicht sie durch. Modulkopf-Abschnitt „Was ein Name entscheidet und was eine Datei" nachgezogen.
- `crates/krk-core/src/leseprofil/datei.rs`: `baustein_pruefen` setzt `typ: None, versteckt: false` mit Kommentar, dass die Schluessel Schritt 2 sind. Keine Profildatei-Schluessel gebaut.
- `crates/krk-core/tests/leseprofil.rs`: das eine Muster `Baustein::Zaehlung { ort, muster }` um `typ: None, versteckt: false` ergaenzt; die zweite Stelle nutzt `..` und brauchte nichts.

## Nicht gemacht, mit Grund

- Keine neuen Proben: die stehen im Plan unter Schritt 5.
- Der Planschritt ist nicht auf `[DONE]` gesetzt: der Dispatch verbietet Aenderungen unter `fusion-workbench/` ausser diesem Eintrag; der Orchestrator zieht ihn nach.

## Verifikation

`make check` — exit 0 (build, test, clippy, fmt).
