# Codersitzung — 260826-1900

**Auftrag:** Schritt 1 des Plans `shared/planning/260826-1811_p_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`: `ueber_datentraeger` löscht die Quelle nur noch, wenn das Kopieren nichts übersprungen hat (`shared/issues/260826-1221_o_ein-gescheitertes-kopieren-ueber-die-datentraegergrenze-loescht-die-quelle-trotzdem.md`).
**Dispatch:** vom Orchestrator, parallel zu einem zweiten Coder in `verzeichnis/sys.rs`, `tests/gemeinsam/mod.rs`, `tests/text.rs`, `tests/verzeichnis.rs` und `CLAUDE.md`; diese Dateien nicht angefasst.
**Git-HEAD:** `26e8039`
**Status:** Complete

## Einrichtung

- `fusion-workbench-root` → `/Users/k1/Projects/productive/krk`; `fusion-rules coder` (fünf Regeldateien, `chat-voice-de.yaml`); `fusion-paths coder` → alles unter `shared/`.

## Rot vor grün

Die drei Proben zuerst geschrieben und am unveränderten Code gefahren (`cargo test -p krk-core --lib operation::verschieben`):

| Probe | vor der Behebung | nach der Behebung |
|---|---|---|
| `eine_datei_die_nicht_ankommt_bleibt_in_der_quelle` (Fall a) | **rot** — „die Quelle ist weg, obwohl nichts angekommen ist" | grün |
| `ein_ordner_mit_einem_uebersprungenen_kind_bleibt_in_der_quelle` (Fall b) | **rot** — „a.txt fehlt in der Quelle" | grün |
| `ein_angekommener_eintrag_verlaesst_die_quelle` (Fall c, Gegenprobe) | grün | grün |

## Geändert

- `crates/krk-core/src/operation/fortschritt.rs`: `Steuerung::uebersprungen_stand` und `Steuerung::uebersprungen_seit`, beide `pub(crate)`, beide `#[must_use]`, die Liste bleibt privat.
- `crates/krk-core/src/operation/verschieben.rs`: `ueber_datentraeger` merkt den Stand vor `kopieren_nach`, gibt bei `Abgebrochen` wie bisher zurück, löscht nur bei leerem `uebersprungen_seit(stand)`. Sonst bleibt die Quelle stehen und bekommt die Zeile „nicht vollstaendig kopiert, in der Quelle geblieben", sofern ihr Pfad nicht schon unter den seither übersprungenen Einträgen steht. Der falsche Kommentar ist durch den Doc-Kommentar ersetzt, der den Zählstand als Zeugen begründet. Prüfmodul mit den drei Proben.

## Befund beim Bauen

Die erste Fassung des Prüfmoduls trug einen eigenen Helfer `Pruefordner` mit `impl Drop`; die Baumprobe `genau_drei_pruefordner_fassungen_stehen_im_baum` (`tests/baum.rs`) hat ihn als vierte Fassung gemeldet, wie `CLAUDE.md` es ankündigt. Die Fassung der Kiste in `tests/gemeinsam/mod.rs` ist aus einem Prüfmodul der Bibliothek nicht erreichbar; das Modul räumt deshalb von Hand ab (`pruefpfad`/`abraeumen`, kein `Drop`), mit der Begründung im Doc-Kommentar. Nichts an der Probe geändert.

## Verifikation

`make check` — exit 0 (Bau, alle Proben des Workspaces, Clippy, `fmt --check`); der Lauf enthält die parallelen Änderungen des zweiten Coders im Arbeitsbaum.

## Nicht getan

- Kein Commit; der Datensatz `260826-1221` bleibt auf `_o_` (Vorgabe des Dispatches).
- Der `EXDEV`-Abzweig selbst bleibt ungemessen, wie der Plan es ausschreibt.
