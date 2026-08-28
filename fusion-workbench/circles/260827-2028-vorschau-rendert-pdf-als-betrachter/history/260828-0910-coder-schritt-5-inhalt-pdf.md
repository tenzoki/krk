# Schritt 5 der Runde 20: der vierte Weg im Modell, `Inhalt::Pdf`

**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Plan:** `planning/260828-0712_p_plan-vorschau-rendert-pdf-als-betrachter.md`, Schritt 5

## Was getan ist

- `crates/krk-ui/src/vorschaumodell.rs`
  - `Inhalt::Pdf { daten: Arc<Vec<u8>>, metadaten: Metadaten }` als achter Wert, Doc nach dem Vorbild von `Bild`; `metadaten` kein `Option`, weil ein PDF allein vom Dateiweg kommt.
  - `PDFENDUNG` neben `BILDENDUNGEN`; `endung_klein` aus `ist_bildpfad` herausgezogen, `ist_pdfpfad` daneben (C1.5, A10).
  - dritter Zweig in `laden` zwischen Bild und Text: `bis_zur_grenze_lesen(pfad, BILDGRENZE)`, `Err` → Metadaten mit leerer Zeilenfolge (C2.1, C2.2, C2.7).
  - `zeigt_dateitext`: `Inhalt::Pdf { .. } => false`.
  - Modulkopf: Absatz zu den vier Wegen unter `# Die Dreiteilung der Anzeige (C6)`; Zusammenfassungs-Abschnitt sagt „ein weiterer Weg".
  - die zwei Zählangaben (`zeigt_dateitext`, `tab_setzen`) ohne Zahl (Constraint 7).
  - Proben: `ein_pdf_ueber_der_grenze_faellt_auf_die_metadaten` (Z1), `die_pdf_endung_gilt_gross_wie_klein` (C1.5), `eine_umbenannte_textdatei_mit_pdf_endung_reist_als_pdf` (C2.3, Modellhälfte), Pdf-Fall in `allein_der_text_einer_datei_traegt_zeilennummern`.
- `crates/krk-ui/src/appkit/vorschau.rs`, minimal: `anzeigen` zeigt für `Inhalt::Pdf` vorläufig die Metadaten (Kommentar auf Schritt 7); `einzufaerben` → `None`; Probe `eingefaerbt_wird_genau_darstellungsart_code` nimmt den Wert auf.
- Defekt `shared/issues/260826-1423_*_zwei-zaehlangaben-…` mit `Resolved:` geschlossen, `_o_` → `_c_`.

## Verifikation

- `cargo test -p krk-ui`: 822 + 5 Proben grün, bevor der Schritt-2-Coder `Wirkungsbereich::Vorschau` und die drei Kommandos in `belegung.rs` gespeichert hat.
- `make check` danach rot an fremden Stellen: `belegungsmodell.rs:399` und `kommandos/fokus.rs:344` decken die neuen Werte noch nicht ab (Dateien des Schritts 2, nicht angefasst).
- Zweiter Anlauf `make check`: `cargo build --workspace` grün (die eigenen Dateien übersetzen), das Prüfziel von `krk-ui` rot an `kommandos/zulaessigkeit.rs:332` (`varianten_der_aufzaehlung` fehlt in `quellbaum`) und `kommandos/fokus.rs:769` (`Wirkungsbereich::Vorschau` nicht abgedeckt), beides Schritt 2. Ergebnis: blocked, bis Schritt 2 steht; danach `make check` erneut fahren.
