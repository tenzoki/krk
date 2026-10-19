# Durchsicht: CSV und TSV als Tabelle, Schlussdurchsicht vor dem Schließen des Pakets

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `c97e264e..6eae327a`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`
**Review domain:** code
**Verdict:** revise
**Cross-references:** 261019-0042-reviewer-csv-und-tsv-als-tabelle-vor-der-auslieferung.md, 261018-0822-behaelt-eine-csv-oder-tsv-datei-beim-sichern-ihre-bytefolgenmarke-und-ihre-zeilenenden.md, 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md

Geöffnet: alle Code- und Dokumentdateien der sechs Commits im Bereich. Aus der vorigen Durchsicht übertragen und jetzt geöffnet, jeweils an den Stellen, die das Paket geändert hat (`git diff 5f29200e..6eae327a`): `crates/krk-core/src/lib.rs`, `crates/krk-core/src/sprache/schluessel.rs`, die drei Sprachtabellen, `crates/krk-core/tests/belegung.rs`, `crates/krk-core/tests/text.rs`, `crates/krk-core/tests/trenntext.rs` (Probenliste), `crates/krk-ui/src/main.rs`. Die `.record.json`-Dateien und Narrative der Werkbank im Bereich sind keine Gegenstände einer Code-Durchsicht; gelesen wurden die Defekte mit ihren `Resolved:`-Vermerken. Das Ereignisprotokoll ist ein Protokoll.

## Summary

Die fünf Behebungen aus `ff2d7b76` halten, was ihre Defekte verlangt haben. Byte-Treue auf der Platte gilt jetzt für Marke, CRLF, gemischte Enden, Umbrüche in Zellen und eine Datei ohne Schluss-Zeilenende, und jede andere Datei sichert wie vorher. Kein Befund ist ein Hindernis für das Schließen im Code. Offen ist eine Frage an den Nutzer: die Bauweise weicht vom beantworteten Entscheid zur Sicherungsform ab und bricht dessen Constraint, ohne dass er sie gesehen hat. Dazu drei kleine Defekte.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 3 |

## Findings by theme

### Die Bauweise und der Entscheid

**M1. Der Stand einer CSV trägt ihre Zeilenenden, der Entscheid sagt das Gegenteil, und der Nutzer hat es nicht gesehen (Medium).** `261018-0822-behaelt-eine-csv-oder-tsv-datei-beim-sichern-ihre-bytefolgenmarke-und-ihre-zeilenenden.md`, Möglichkeit 2, verlangt einen Stand „ohne Marke, nur `\n`“ und führt das als Constraint („drei Stellen rechnen davon“). `ff2d7b76` baut `datei::trenntext_oeffnen` mit `Zeilenenden::WieGelesen`. Die vorige Durchsicht hatte für eine solche Verschiebung eine Zeile des Nutzers verlangt; der `Resolved:`-Vermerk von `261019-0037-ein-umbruch-in-einer-zelle-einer-crlf-csv-wird-bei-jedem-sichern-zu-crlf-auch-in-unberuehrten-zeilen.md` sagt „auf Anweisung des Dispatch selbständig ausgeführt; der Entscheid ist nicht ergänzt“. Der Entscheid steht auf `answered` mit einer Antwort, die der Code nicht umsetzt. Die Wirkung ist besser als die gewählte Bauweise, die Folgen sind klein, aber sie sind die Wahl des Nutzers.
Entscheid: `261019-1122-gilt-der-stand-mit-den-zeilenenden-der-datei-als-antwort-auf-die-sicherungsform-einer-csv.md`, mit den Folgen beider Möglichkeiten und der Empfehlung, die neue Bauweise anzunehmen.

### Treue der gesicherten Bytes

Ohne Befund geprüft (`crates/krk-core/src/text/datei.rs`, `crates/krk-ui/src/editormodell.rs`):

- `trenntext_oeffnen` schneidet allein eine führende Marke ab. `in_dateiform` schreibt bei `WieGelesen` Marke und Stand Byte für Byte, ohne `sicherungsform`, also ohne angehängtes Zeilenende. Belegt von `eine_csv_mit_marke_und_crlf_kommt_unveraendert_byte_fuer_byte_zurueck` (`crates/krk-core/tests/text.rs`, mit gemischten Enden und `\n` in einem gequoteten Feld) und `ein_umbruch_in_einer_excel_zelle_bleibt_beim_sichern_einer_anderen_zelle_stehen` (`editormodell.rs`, mit Kopfzelle, `opt+return` und Datei ohne Ende).
- Eine neue Zeile nimmt das erste Satzende des Index (`zeilenende_des_textes`, `trenntext/handlung.rs`), also außerhalb von Anführungszeichen; ein neuer Satz in einer CRLF-Datei endet auf `\r\n`.
- `NurCr` wandelt jedes `\r` in `\n` und beim Sichern zurück. Weil die Datei kein `\n` trug, kommt sie Byte für Byte zurück; ein `\r\n` aus der Ablage geht über `in_stand_form` → `in_gehaltene_form` als `\n` hinein und als `\r` hinaus.
- **Nicht-CSV-Dateien:** `text_lesen` wählt `trenntext_oeffnen` allein über `trenntext::Art::aus_namen`; `Editormodell::uebernehmen` setzt `Dateiform::GEHALTEN`, wenn `trenntextart` nein sagt, und bringt den Stand dann über `in_gehaltene_form` in die gehaltene Form. `jede_andere_datei_sichert_wie_vorher` (`tests/text.rs`) und der zweite Teil von `eine_excel_csv_behaelt_marke_und_zeilenenden_und_jede_andere_datei_nicht` halten `\n` und keine Marke.
- `bearbeiten` und `text_zurueckschreiben` fragen dieselbe Bedingung `Editormodell::wandelt`; für `WieGelesen` ist sie immer falsch, und ein `\r\n` aus der Fläche bleibt im Stand.

### Rohansicht, Suche, Zeilen

**L1. Der Zeilenindex liefert in einer CRLF-CSV den Zeileninhalt mit `\r`, und Textmarken verfehlen ihre Zeile (Low).** `Zeilenindex::inhalt_der_zeile` (`crates/krk-core/src/text/zeilen.rs`) schneidet genau ein Byte ab, kommentiert mit „weil der gehaltene Stand kein `\r\n` mehr trägt“. `schreibmarkenzeile` und `marke::wiederfinden` vergleichen wörtlich. Eine Marke aus einer Fassung vor 3.9.0 auf einer CSV-Zeile endet in `Fund::NichtGefunden`; eine neue schreibt `\r` nach `bookmarks.toml`. Am selben Grund veraltet: der Modulkopf von `trenntext/handlung.rs` („der gehaltene Stand des Editors trägt allein `\n`“).
Issue: `261019-1122-der-zeilenindex-liefert-in-einer-crlf-csv-den-zeileninhalt-mit-cr-und-textmarken-verfehlen-ihre-zeile.md`.

Ohne Befund geprüft:

- **Zeilennummern und Zeilensprung:** `Zeilenindex::neu` zählt Anfänge hinter `\n`; mit `\r\n` stehen sie an derselben Stelle wie in AppKits Absatzlayout. `anfang_der_zeile` und `zeile_am_versatz` rechnen damit richtig. Ein einzelnes `\r` in einer Datei mit `\n` bricht die Ausrichtung der Nummernspalte; `CLAUDE.md` und `HowTo.md` nennen das.
- **Suche:** sie rechnet auf dem Stand in Bytes und rechnet über `koordinaten` in UTF-16 um; `\r` ist dort ein Zeichen wie jedes andere. Ein Ersatztext kommt gewandelt an und bringt `\n` in eine CRLF-Datei; der Modulkopf von `editormodell.rs` sagt das.
- **`replaceCharactersInRange:withString:` und UTF-16:** `Einsatz::aus_umkehrpunkt` rechnet `location` über `koordinaten::einheiten` des Präfixes und `length` über die Einheiten von `punkt.entfernt`. `einheiten` zählt Startbytes und für Vierbytezeichen eine zweite Einheit; `die_einheiten_sind_die_laenge_in_utf16` hält das mit Umlauten, `\r\n` und einem Zeichen außerhalb der Grundebene gegen `encode_utf16`. `ein_umbau_ersetzt_an_der_flaeche_allein_den_geaenderten_bereich` spleißt am UTF-16-Vektor und erhält den Stand danach. `\r` und `\n` sind in UTF-16 zwei Einheiten wie in UTF-8 zwei Bytes; ein Schnitt zwischen ihnen ist für `NSString` kein Fehler.
- `gleich_von_vorn` und `gleich_von_hinten` geben dasselbe wie der Bytevergleich (`der_blockweise_vergleich_zaehlt_wie_der_bytevergleich`, Blockgrenzen eingeschlossen); `gemeinsamer_anfang` und `gemeinsamer_schwanz` rücken danach weiter auf eine Zeichengrenze.

**L2. Der Teilersatz an der Textfläche prüft allein die Länge, und der Doc-Kommentar sagt mehr (Low).** `stand_einsetzen` (`crates/krk-ui/src/appkit/editor.rs`) behauptet, ein Bereich über den Text hinaus zeige eine Fläche, die nicht den erwarteten Stand trägt. Der Wächter verhindert allein die Objective-C-Ausnahme; eine abweichende Fläche mit passendem Bereich ergäbe still falschen Text, und die einzige Nachprüfung ist ein `debug_assert_eq!` über die Länge. Heute ist kein Weg bekannt, auf dem beide auseinanderlaufen.
Issue: `261019-1122-der-teilersatz-an-der-textflaeche-prueft-allein-die-laenge-und-der-doc-kommentar-sagt-mehr.md`.

### Vorschau und Editor

**L3. Die Vorschau entscheidet „`\r` als Satzende“ am ersten Megabyte, der Editor an der ganzen Datei (Low).** `trenntext_aus_bytes` kürzt über `TEXTGRENZE` vor `trenntext::lesbar`; `trenntext_oeffnen` fragt `cr_als_satzende` über die ganze Datei. Eine CR-Datei über 1 MiB mit einem späten `\n` erscheint in beiden verschieden. `inference:` eine Excel-„CSV (Macintosh)“ mit einem Umbruch in einer Zelle trägt ein `\n` und fällt dann in beiden nicht unter die Regel; nicht an einer echten Datei geprüft.
Issue: `261019-1122-die-vorschau-entscheidet-cr-als-satzende-am-ersten-megabyte-der-editor-an-der-ganzen-datei.md`.

### Die übrigen Behebungen

Ohne Befund:

- **Verweigerung an der Datei** (`Trenntextlage.verweigert`): gesetzt allein in `tabellenform_pruefen`, gelöscht mit jeder neuen Lage, mit `trennzeichen_weiter` und im Ansichtswechsel. `editorform_der_datei` bildet sie auf die Textform ab, `dargestellte_ansicht` auf die Rohdarstellung, und `modell.ansicht` bleibt stehen. Der Ansichtswechsel versucht bei stehender Verweigerung die Tabelle erneut, statt umzuschalten, wie `HowTo.md` es sagt.
- **Trennzeichen beim Neuladen:** `dialekt_beim_eintreffen` behält einen gewechselten Dialekt allein für dieselbe Datei und rein, ohne Fenster.
- **`paste:` ohne Text:** reicht einen leeren Text weiter, und `Tabellenhandlung::Einfuegen` antwortet bei leerem Block `AblageOhneText`.
- **Hauptfaden:** die neuen Lesewege (`trenntext_oeffnen`) laufen auf dem Lesefaden des Editors (`text_lesen`, gestartet über `thread::Builder`), die Vorschau auf ihrem Faden. `dialekt_beim_eintreffen`, `Einsatz` und der Teilersatz stellen keinen Systemaufruf. Neue Plattenzugriffe auf dem Hauptfaden gibt es im Bereich nicht.

### Menü, Sprachtabellen, Fassungen

- `733067ff`: `TrenntextZeileEinfuegen` steht in `bereich_des_kommandos` unter „Editor“ vor den Spaltenbefehlen, der Block in `resources/default-keymap.toml` steht davor, und beide Reihenfolgeproben (`EDITORBEFEHLE`, `HOMEBEFEHLE`) sind nachgezogen. Taste und Wirkungsbereich unverändert. `HowTo.md` stimmt. Ohne Befund.
- Sprachtabellen: jeder neue Schlüssel steht in `de.rs`, `fr.rs` und `en.rs`; die Länge von `Text::ALLE` und `Zahlwort::ALLE` ist nachgezogen, und die Vollständigkeit hält der Übersetzer. Die Sätze „die Datei bleibt in der Rohansicht“ (`TrenntextUnterminiert`, `TrenntextZuVieleSpalten`) beschreiben weiter, was der Nutzer sieht, auch wenn die gewählte Ansicht seit `ff2d7b76` nicht mehr umschaltet.
- `c1b1bf9b` und `6eae327a` heben die Version in `Cargo.toml` und `Cargo.lock` auf 3.9.0 und 3.9.1, und `v3.9.0` und `v3.9.1` zeigen auf genau diese Commits. Die Commit-Nachricht von `011bb9b6` nennt „vor der Auslieferung 3.9.0“, gehört aber zur 3.9.1. Nicht nachträglich zu ändern, kein Befund.

## Cross-cutting observations

- **Eine Zusage, die an drei Stellen gerechnet wurde, gilt jetzt zur Hälfte.** Der Modulkopf von `text/datei.rs` sagt es inzwischen, `editormodell.rs` und `CLAUDE.md` auch. Die zwei Stellen, die es nicht sagen, sind die, an denen L1 entsteht: `text/zeilen.rs` und `trenntext/handlung.rs`. Wer künftig auf „`\n` als einziges Zeilenende“ rechnet, muss die `Dateiform` fragen. M1 legt dem Nutzer genau diese Folge vor.
- **Die Regel „`\r` als Satzende“ ist eine Frage an die ganze Datei.** Wer sie an einem Ausschnitt stellt, bekommt eine andere Antwort (L3).

## Recommended sequencing

1. **Vor dem Schließen des Pakets:** M1 vom Nutzer beantworten lassen. Bei Möglichkeit 1 ein Entscheid, der `261018-0822-…` ablöst; bei Möglichkeit 2 Rückbau.
2. **Mit M1, bei Möglichkeit 1:** L1, weil es die eine Folge ist, die als Defekt bleibt.
3. **Aufräumen:** L2, L3.

Unverändert offen und nicht neu gemeldet: `261019-0037-die-zelluebernahme-an-16-mib-ist-ohne-setstring-gemessen-und-die-100-ms-grenze-des-plans-ist-nicht-entschieden.md` (Spalte einfügen an 16 MiB über 100 ms). Der Test am Bündel und der Entscheid über den Abnahmelauf bleiben Nutzerarbeit.

## Verification

- `cargo test --workspace` am Stand `6eae327a`: 36 Testziele `ok`, keines gescheitert, keine Panik. Die Zeilen `ZipWriter::drop: failed to finalize archive` stammen aus Archivproben, die einen Abbruch mit Absicht herbeiführen, und betreffen nicht dieses Paket.
- Die Messprobe an 16 MiB habe ich nicht neu gefahren; die Zahlen stehen im offenen Defekt zur Zellübernahme.
- Grün heißt hier nicht befundfrei: L1 und L3 sieht keine vorhandene Probe, und M1 ist eine Frage an den Nutzer, keine Frage an den Code.
