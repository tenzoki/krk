# Schritt 6 der Runde 20: das Modul `appkit/betrachter.rs`, die Klasse `Pdfbetrachter`

**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Plan:** `planning/260828-0712_p_plan-vorschau-rendert-pdf-als-betrachter.md`, Schritt 6

## Was getan ist

- `crates/krk-ui/src/appkit/betrachter.rs` (neu, 700 Zeilen)
  - `define_class!` Unterklasse `Pdfbetrachter` von `PDFView`, `MainThreadOnly`, `ivars` = `PdfbetrachterIvars { vorschau: RefCell<Option<Weak<Vorschaufenster>>>, bytes: RefCell<Option<Arc<Vec<u8>>>>, seitenmelder: RefCell<Option<Box<dyn Fn()>>> }`.
  - `copy:` überschrieben: leere oder fehlende Auswahl tut nichts (C5.5), sonst `zwischenablage::text_schreiben`, Rückgabewert mit `let _ =` (Begründung am Doc: kein Statuszeilenzugang im Betrachter, Textanzeige der Vorschau meldet ebenso nicht). Oberklasse wird nicht gerufen.
  - `menuForEvent:` ruft `super`, hängt über `teilen::eintrag_anfuegen` den Teilen-Eintrag mit der Datei des aktiven Tabs an (C5.8); ohne Bestand und ohne Datei kein Menü.
  - `PDFViewDelegate::PDFViewWillClickOnLink:withURL:` → `zwischenablage::im_browser_oeffnen`, **nur `http`/`https`** (C9 der Runde 1, Regel `ist_webschema` ohne Fenster prüfbar); der Betrachter ist sein eigener Delegierter, `setDelegate` einmal in `neu` (PDFView hält ihn schwach, `PDFView.h:178`).
  - `dokument_setzen(&Arc<Vec<u8>>) -> Deutung { Gesetzt, Beschaedigt, Gesperrt }`, `#[must_use]`: `Arc::ptr_eq` mit dem Merkposten → `Gesetzt` ohne Neusetzen (C1.7); sonst `PDFDocument::initWithData` (`nil` → `Beschaedigt`), `isLocked` → `Gesperrt`; bei `Gesetzt` `setDocument`, `SinglePageContinuous`, `Vertical`, `setDisplaysPageBreaks(true)`, `setMinScaleFactor(ZOOM_MIN)`, `setMaxScaleFactor(ZOOM_MAX)`, `setAutoScales(true)` (C1.1, C1.2, A1, A2).
  - `zoomen(Zoom) -> bool`, `#[must_use]`, über `canZoomIn`/`zoomIn:`, `canZoomOut`/`zoomOut:`, `setAutoScales(true)`; an der Grenze `false` (C3.9).
  - `seitenstand() -> Option<(usize, usize)>` aus `currentPage`, `indexForPage`, `pageCount`, ab eins; `NSNotFound` als „Index ≥ Seitenzahl" abgefangen.
  - `PDFViewPageChangedNotification` an `NSNotificationCenter::defaultCenter` mit Selektor `seiteGewechselt:` in `neu` angemeldet, in `Drop` abgemeldet (Muster `Nummernspalte`); der Selektor ruft den Seitenmelder.
  - `ZOOM_MIN = 0.25`, `ZOOM_MAX = 8.0` mit Begründung, `const _: () = assert!(...)` hält 0 < MIN < 1 < MAX beim Übersetzen (Muster `STAPELBUDGET`).
  - Modulkopf mit `# Ab welchem macOS die angesprochenen Klassen stehen`, **am SDK gelesen**: die vier Klassen 10.4; `displayDirection`, `PDFDisplayDirection`, `minScaleFactor`, `maxScaleFactor` 10.13 (höchste Angabe); `PDFViewWillClickOnLink:withURL:` 10.5; `PDFViewPageChangedNotification` 10.4. `pageBreakMargins` und `scaleFactorForSizeToFit` werden nicht gerufen.
  - Proben ohne Fenster: `zoom_und_deutung_tragen_je_genau_drei_werte` (Varianten aus dem Quelltext, kein `_ =>` in der Datei), `nspasteboard_steht_nicht_im_betrachter_und_copy_genau_einmal` (Codezeilen, baumweit; C5.2 Baumhälfte, Constraint 3), `allein_http_und_https_sind_webschemata`.
- `crates/krk-ui/src/appkit/mod.rs`: `mod betrachter;` mit `#[allow(dead_code)]` und Ablaufdatum „fällt mit Schritt 7"; „Einunddreissig Module"; Skizze und Prosa nachgezogen.

## Abweichungen vom Plan, mit Grund

1. **Seitenwechsel über einen Melder statt über `Vorschaufenster::seiten_melden`.** Die Methode entsteht erst in Schritt 7 (`vorschau.rs`, nicht mein Gegenstand); ein Ruf darauf übersetzte nicht. Der Betrachter trägt deshalb `seitenmelder_setzen(Box<dyn Fn()>)` nach dem Muster `Hauptfenster::melder_setzen`; Schritt 7 trägt `|| vorschau.seiten_melden()` mit schwachem Griff ein. Der Rückverweis bleibt daneben für das Kontextmenü.
2. **Das Kontextmenü fragt `Vorschaufenster::angezeigter_pfad` (pub) statt `teilbare_pfade`.** Letzteres ist privat in `vorschau.rs`; Schritt 7 kann es auf `pub(super)` heben und den Rufer umstellen, die Regel „keine oder eine Datei" ist dieselbe.
3. **`setDelegate` einmal in `neu` und nicht in `dokument_setzen`**: der Delegierte gehört der Ansicht, nicht dem Dokument.
4. **Verweise mit anderem Schema als `http`/`https` tun nichts** (der Plan sagt dazu nichts; `im_browser_oeffnen` setzt die Grenze beim Rufer voraus, C9).
5. **Die Zoomgrenzen-Probe steht als `const _: () = assert!`** statt als `#[test]`, weil clippy `assertions_on_constants` unter `-D warnings` verbietet.

## Verifikation

- `cargo build -p krk-ui` exit 0; `cargo clippy --workspace --all-targets -- -D warnings` exit 0; `cargo fmt --all --check` exit 0.
- `cargo test --workspace --no-fail-fast`: genau drei rote Proben, alle auf die Belegungseinträge des Ontocoders wartend: `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` (krk-core lib), `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` (krk-core tests/belegung.rs), `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander` (krk-ui). Sonst grün, die drei neuen Proben eingeschlossen (krk-ui 828 von 829).
- `make check` exit 2 (bricht an der ersten roten Probe im Ziel `test` ab).
