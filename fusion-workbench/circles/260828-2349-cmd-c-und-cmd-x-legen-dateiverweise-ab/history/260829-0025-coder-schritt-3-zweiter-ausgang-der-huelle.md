# Coder-Sitzung: Schritt 3 der Runde 22, der zweite Ausgang der Hülle

**Date:** 2026-08-29, 260829-0025
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `circles/260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab`
**Plan:** `planning/260829-0006_*_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`, Schritt 3
**HEAD beim Start:** `5b2a5ce`

## Was getan wurde

Eine Datei geändert: `crates/krk-ui/src/appkit/zwischenablage.rs`.

- `dateiverweise_auf_ablage_schreiben(ablage, pfade, namen) -> bool` mit `#[must_use]`: `clearContents`, je Pfad `NSURL::fileURLWithPath`, als `ProtocolObject<dyn NSPasteboardWriting>` gesammelt, `writeObjects:`, bei `false` sofort `false`, danach `setString_forType(namen, NSPasteboardTypeString)` als Antwort. Doc-Kommentar nach Schritt 3: `clearContents` als Bedingung, die Namen kommen fertig herein (Entscheidung 4), Reihenfolge `writeObjects:` vor `setString:forType:` (Entscheidung 3), `fileURLWithPath:` kostet ein `stat(2)` je Eintrag, die leere Menge entscheidet der Rufer.
- `dateiverweise_schreiben(pfade, namen) -> bool` mit `#[must_use]`, reicht `generalPasteboard` hinein, ohne Probe wie `text_schreiben`. Trägt `#[allow(dead_code)]` mit dem Ablaufdatum Schritt 4 im Kommentar davor: der Rufer `DateifensterQuelle::dateiverweise_ablegen` entsteht erst dort.
- Modulkopf (C5.2): Skizze mit dem Pfeil `dateiverweise_schreiben <── cmd+c und cmd+x im Dateifenster (Runde 22)`; der Absatz zum 260811 sagt nicht mehr „eine einzige Sorte" und „kein `writeObjects:`", sondern dass die zwei Pfadkopierer allein Text schreiben und der Entscheid fortgilt, weil ihr Name einen Pfad verspricht; neuer Abschnitt `# Seit der Runde 22 schreibt die Huelle zwei Sorten`.
- Untergrenzen-Abschnitt (C5.5): `fileURLWithPath:` nachgetragen, am SDK 15 nachgelesen: `NSURL.h:52` ohne `API_AVAILABLE`, also seit 10.0; `NSPasteboardWriting` seit 10.6, `NSPasteboard.h:386` (Protokolldeklaration) und `:379` (der Kommentar, der `NSURL` als Erfüller nennt). Die Zeilennummer `:469`, die `teilen.rs` für das Protokoll zitiert, trifft am SDK dieses Geräts nicht; zitiert ist die gelesene.
- Prüfmodul: der eigene Schreiber `dateien_ablegen` über `writeObjects:` ist gefallen; der Name bleibt als dünne Hülle um `dateiverweise_auf_ablage_schreiben` mit `assert!`. `zwei_dateiverweise_kommen_als_zwei_pfade_zurueck` ruft den neuen Ausgang mit leeren Namen. Neu: `der_zweite_ausgang_legt_verweise_und_namen_ab` (C2.7, C1.4), `eine_verknuepfung_wird_als_verknuepfung_abgelegt` (C1.9, `std::os::unix::fs::symlink` im Prüfordner), `ein_zweites_ablegen_ersetzt_das_erste`. Alle über benannte Probenablagen (`pasteboardWithName:`, A12), `generalPasteboard` wird von keiner Probe angefasst. `die_huelle_um_die_zwischenablage_steht_in_genau_einer_datei` hat `writeObjects` als dritte Nadel (C5.1); der Doc-Kommentar sagt, dass die Hülle seit dieser Runde drei Griffe hat.

Gegen HEAD verglichen mit `git diff -- crates/krk-ui/src/appkit/zwischenablage.rs`; keine andere Datei angefasst, nichts committet.

## Befund aus der Probe

`setString:forType:` nach `writeObjects:` landet am ersten Eintrag und wird von `stringForType:` unverändert zurückgelesen; die Erschließung aus Entscheidung 3 trägt am Prüfstand. Welche Sorten das `NSURL` daneben ablegt, ist damit nicht gemessen (Schritt 9).

## Verifikation

- `cargo test -p krk-ui zwischenablage`: 9 passed, exit 0.
- `cargo fmt --all --check`: exit 0.
- `cargo clippy --workspace --all-targets`: keine Meldung zu `zwischenablage.rs` (zwei `cloned_ref_to_slice_refs` in den Proben behoben). Meldungen aus `operationen.rs` und `zulaessigkeit.rs` (unbenutzte Funktionen der Schritte 1 und 2) gehören den parallelen Codern.
- `make check`: rot, exit 2. Die eine rote Probe ist `kommandos::zulaessigkeit::tests::die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` aus Schritt 2; der Plan sagt sie bis Schritt 5 als rot voraus (Absatz nach dem Schrittgraphen). `cargo test -p krk-ui`: 849 passed, 1 failed, dieselbe Probe.
