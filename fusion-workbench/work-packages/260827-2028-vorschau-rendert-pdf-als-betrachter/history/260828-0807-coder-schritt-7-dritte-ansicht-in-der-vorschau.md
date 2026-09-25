# Schritt 7 der Runde 20: die dritte Ansicht in der Vorschau

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Plan:** `planning/260828-0712_p_plan-vorschau-rendert-pdf-als-betrachter.md`, Schritt 7

## Was getan ist

- `crates/krk-ui/src/appkit/vorschau.rs`
  - `enum Flaeche { Text, Bild, Betrachter }` (privat, `Copy`, `PartialEq`), mit Doc-Kommentar.
  - `VorschaufensterIvars` traegt `betrachter: OnceCell<Retained<Pdfbetrachter>>`, `flaeche: Cell<Flaeche>` (Vorbelegung `Text`) und `seitenmelder: RefCell<Option<Box<dyn Fn()>>>`.
  - `flaeche_zeigen(&self, Flaeche)`: schreibt den Merkposten, verzweigt vollstaendig ueber die drei Werte auf ein Tripel und ruft die drei `setHidden`; der Betrachter nur, wenn die Zelle gefuellt ist. `bild.setHidden(true)` aus `bauen` ist gefallen (das erste `anzeigen` geht durch den Schalter). `text_zeigen` und `bild_zeigen` rufen `flaeche_zeigen` statt ihrer Schalterpaare.
  - `pdf_zeigen(&self, &Arc<Vec<u8>>, &Metadaten)`: `get_or_init` baut `Pdfbetrachter::neu` mit den `bounds` der Inhaltsflaeche, Autoresizing beide Richtungen, `ziel_setzen(self)`, `seitenmelder_setzen` mit schwachem Griff auf `seiten_melden`, `addSubview`. Dann `dokument_setzen` und die vollstaendige Verzweigung ueber `Deutung`: `Gesetzt` → `flaeche_zeigen(Betrachter)`, `Beschaedigt | Gesperrt` → `text_zeigen(metadaten_text(metadaten, &[]))`.
  - `anzeigen`: `Inhalt::Pdf { daten, metadaten } => self.pdf_zeigen(&daten, &metadaten)`; am Ende `seiten_melden()`. `einzufaerben` trug den Pdf-Zweig (`None`) schon aus Schritt 5.
  - `fokusansicht` dreiwertig ueber `flaeche` (Text → Textanzeige, Bild → Inhaltsflaeche, Betrachter → Betrachter, Rueckfall auf die Inhaltsflaeche bei leerer Zelle, die dort nicht erreichbar ist); `isHidden` wird nicht mehr gelesen.
  - `pub fn zoomen(&self, Zoom) -> bool` (`#[must_use]`): `false` ohne stehenden Betrachter (C3.7), sonst durchgereicht (C3.9).
  - `pub fn seitenzaehler(&self) -> Option<String>`: `None`, sobald nicht `Flaeche::Betrachter` steht (C4.4), sonst `seitenzaehler_text(seitenstand)`.
  - `pub fn seitenmelder_setzen(Box<dyn Fn()>)` und `fn seiten_melden` nach dem Muster `Hauptfenster::melder_setzen`.
  - Modulkopf: Skizze mit dritter Ansicht, neuer Abschnitt `# Die dritte Ansicht und der eine Schalter (Runde 20)`, Kontextmenue-Absatz um den dritten Anschluss und die zweite Abfangstelle (`copy:` in `betrachter.rs`) ergaenzt, Untergrenzen-Abschnitt: `isHidden` nicht mehr gelesen, keine PDFKit-Klasse in dieser Datei, die vier `NSView`-Beruehrungen am Betrachter (`NSView.h:83`, `:101`, `:125`, `:139`, am SDK gelesen).
  - Proben: neu `set_hidden_steht_in_dieser_datei_allein_in_flaeche_zeigen` (genau drei Codezeilen, alle im Rumpf von `flaeche_zeigen`) und `der_betrachter_wird_allein_in_pdf_zeigen_gebaut` (Z2: `Pdfbetrachter::` genau einmal, im Rumpf von `pdf_zeigen`); `die_zuordnung_auf_eine_ansicht_steht_in_der_vorschau_genau_einmal` prueft zusaetzlich, dass der Rumpf von `fokusansicht` alle drei `Flaeche::… =>` traegt und keinen `_ =>`; `eingefaerbt_wird_genau_darstellungsart_code` trug `Inhalt::Pdf` schon aus Schritt 5, der Doc-Kommentar zaehlt nicht mehr.
- `crates/krk-ui/src/appkit/mod.rs`: `#[allow(dead_code)]` an `mod betrachter;` samt Kommentar entfernt.
- `crates/krk-ui/src/appkit/betrachter.rs`: nur `#[allow(dead_code)]` mit Ablaufdatum „faellt mit Schritt 9" an `pub enum Zoom` — die drei Werte werden erst beim Anwendungsdelegierten erzeugt, und ohne die Ausnahme ist `make lint` rot.
- Plan Schritt 7 auf `[DONE]`.

## Abweichungen vom Plan, mit Grund

1. **`seitenzaehler_text` steht vorlaeufig privat in `vorschau.rs`** (ueber `kommandos::operationen::zahl`, also mit Tausenderpunkten). Der Plan legt sie in Schritt 8 nach `statuszeile.rs`, das dieser Schritt nicht anfassen darf; `seitenzaehler` behaelt die geplante Signatur `Option<String>`. **Schritt 8 muss deshalb `vorschau.rs` an zwei Zeilen anfassen**: die private Fassung streichen und `super::statuszeile::seitenzaehler_text` rufen. Der Doc-Kommentar der Fassung sagt das.
2. **Drei `#[allow(dead_code)]` mit Ablaufdatum** an `Vorschaufenster::zoomen` (Schritt 9), `seitenzaehler` und `seitenmelder_setzen` (Schritt 8) und an `Zoom` in `betrachter.rs` (Schritt 9): die Rufer entstehen erst in jenen Schritten, und `make lint` fuehrt `-D warnings`. Die Ausnahme an `mod betrachter;` ist wie geplant gefallen; `seitenstand` und `Pdfbetrachter::zoomen` sind ueber die erlaubten Rufer lebendig.
3. **`tab_waehlen` ruft `seiten_melden` nicht selbst**: ein wirklicher Tabwechsel laeuft durch `anzeigen`, das am Ende meldet; der andere Zweig aendert den Tab nicht und haette nichts zu melden. Ein zweiter Ruf haette denselben Wechsel zweimal gemeldet.
4. **`die_zwei_schalter_stehen_je_an_genau_einer_stelle_und_dort` bleibt unveraendert**; die `setHidden`-Probe steht als neue Probe daneben. Die bestehende haelt `setSelectable`/`setEditable` der Runde 14, und den Schutz zu ersetzen haette C1.1/C1.4 jener Runde ohne Probe gelassen. Der Plan zitiert sie mit `:1874`, der Zeile jener Probe; ich lese das als Verwechslung der zwei Schalterpaare.
5. **Zu Abweichung (2) des Schritts 6:** `teilbare_pfade` bleibt privat, der Betrachter fragt weiter `angezeigter_pfad` ueber seinen schwachen Rueckverweis; beide sind dieselbe Einzeilerregel, und ein `pub(super)` haette eine zweite oeffentliche Auskunft ueber denselben Wert geschaffen. Der Doc-Kommentar von `angezeigter_pfad` zaehlt jetzt vier Frager.

## Verifikation

- `cargo build -p krk-ui` exit 0; `cargo clippy --workspace --all-targets -- -D warnings` exit 0; `cargo fmt --all --check` exit 0.
- `cargo test --workspace --no-fail-fast`: genau die drei roten Proben, die auf die Belegungseintraege des Ontocoders warten (`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`, `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`, `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`). Sonst gruen, krk-ui 829 von 830, alle elf Proben von `appkit::vorschau::tests` gruen.
- `make check` exit 2 (bricht im Ziel `test` an der ersten roten Kernprobe ab; `lint` und `fmt-check` einzeln gruen, siehe oben).
