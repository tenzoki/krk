# Coder: Schritt 3 der Runde 19 — das eingebaute Default-Profil und der Rückfallzweig im Kern

**Datum:** 260827-1720
**Plan:** `planning/260827-1322_p_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md`, Schritt 3
**Kriterien:** C1.1, C1.2, C1.3, C1.4, C1.5, C1.7, C2.1, C2.7, C2.11, C3.5, C4.1, C4.2, C4.3, C4.4, C4.7
**Status:** Complete

## Was gebaut ist

- `crates/krk-core/src/leseprofil/defaultprofil.rs` (neu): `static DEFAULTPROFIL: LazyLock<Profil>` und der Zugang `defaultprofil() -> &'static Profil` mit `#[must_use]`. Drei Zeilen „Dateien", „Ordner", „Verknüpfungen" (A1), je `Baustein::Zaehlung { ort: Ortsangabe::wurzel(), muster: None, typ: Some(..), versteckt: true }`; Name nur für Meldungen, `pfad` und `kennzeichen` sind `None`. Modulkopf mit den drei Begründungen: kein Block in `readers.toml` (Blockreihenfolge, C1.3/C1.4), nicht abschaltbar (ein Rückfallweg, C1.5), Beschriftungen im Kern statt in der Ansicht (eine Stelle, C4.5). Eine Modulprobe hält Zahl, Reihenfolge, Typ und `versteckt` der drei Zeilen.
- `crates/krk-core/src/leseprofil/mod.rs`: `pub mod defaultprofil` und `pub use defaultprofil::defaultprofil`; Aufzählung `Auskunft { Erkannt(Zusammenfassung), Default(Vec<Zusammenfassungszeile>) }` ohne Auffangzweig; freie Funktion `zeilen_als_text(&[Zusammenfassungszeile]) -> String` mit `#[must_use]`, jede Zeile mit führendem `\n`; `Zusammenfassung::als_text` ist jetzt Kopfzeile plus dieser Aufruf, Ausgabe unverändert. Ablaufbild im Modulkopf nachgezogen (zwei Ausgänge, Verknüpfung → `None`).
- `crates/krk-core/src/leseprofil/bausteine.rs`: `zusammenfassen` und `zusammenfassen_gezaehlt` liefern `Option<Auskunft>` bzw. `Option<(Auskunft, Haushalt)>`. Die Zeilenrechnung ist als `Lauf::zeilen_rechnen(&Profil)` herausgezogen und wird von beiden Zweigen gerufen. Der Rückfallzweig läuft **im selben `Lauf`** (C4.2): kommt `erkennen` leer zurück, prüft `ist_selbst_ein_verzeichnis` mit `std::fs::symlink_metadata` am ausgewählten Pfad; nein → `None` (A4, C1.7), ja → `Auskunft::Default(lauf.zeilen_rechnen(defaultprofil()))`. Modulkopf: Ablaufbild und ein Absatz zu C4.2/C4.1.
- `crates/krk-core/tests/leseprofil.rs`: `Auskunft` importiert; Helfer `erkannte(Auskunft) -> Zusammenfassung` (panikt bei `Default`) und `gezaehlt_erkannt`; die Proben der Runde 16, die eine Zusammenfassung erwarten, gehen darüber. Drei Stellen, die für einen Ordner ohne Profiltreffer `None` erwarteten, fragen jetzt nach `Some(Auskunft::Default(_))` (`ohne_profiltreffer_entsteht_keine_zusammenfassung`) bzw. nach „nicht `Erkannt`" (bösartiges Muster, Deskriptor-Gegenprobe im Kind).
- `crates/krk-ui/src/vorschaumodell.rs`, nur zum Übersetzen: `laden` verzweigt vollständig über `Auskunft`; **`Auskunft::Default(_)` fällt vorläufig auf den bisherigen Metadaten-Weg** (`Inhalt::Metadaten(metadaten)` ohne Zählzeilen), mit Kommentar, dass Schritt 4 die richtige Verzweigung baut. Die Anzeige zeigt damit noch keine Zählzeilen.

## Nicht gemacht, mit Grund

- Keine neuen Abnahmeproben für C1/C4: Schritt 5.
- Keine Änderung an `Inhalt::Metadaten`, `metadaten_text`, `anwendung.rs`: Schritt 4.
- Nichts unter `resources/`; Planschritt nicht auf `[DONE]` gesetzt (Dispatch-Verbot; der Orchestrator zieht nach).

## Verifikation

`make check` — exit 0 (build, test, clippy `-D warnings`, fmt).
