# Durchsicht: CSV und TSV als Tabelle in Vorschau und Editor, vor der Auslieferung

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `5f29200e..c97e264e`
**Not-opened:** `crates/krk-core/src/lib.rs`, `crates/krk-core/src/sprache/schluessel.rs`, `crates/krk-core/src/sprache/tabelle/de.rs`, `crates/krk-core/src/sprache/tabelle/en.rs`, `crates/krk-core/src/sprache/tabelle/fr.rs`, `crates/krk-core/tests/belegung.rs`, `crates/krk-core/tests/text.rs`, `crates/krk-core/tests/trenntext.rs`, `crates/krk-ui/src/main.rs`, `fusion-workbench/orchestrator-events.jsonl`
**Review domain:** code
**Verdict:** revise
**Cross-references:** 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md, 261018-0822-behaelt-eine-csv-oder-tsv-datei-beim-sichern-ihre-bytefolgenmarke-und-ihre-zeilenenden.md, 261018-0822-wird-eine-csv-datei-eine-weitere-form-der-eintragsansicht-und-rechnet-jede-tabellenhandlung-auf-dem-text.md, 261018-0822-zeigt-die-vorschau-csv-und-tsv-als-ausgerichteten-text-oder-als-tabelle.md, 261018-0822-schuldet-das-csv-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md

Die vier nicht geöffneten Dateien unter `crates/krk-core/tests/` und die Sprachtabellen sind über die Läufe gedeckt, nicht über das Lesen: `cargo test --workspace` lief am Stand `c97e264e` grün, und `cargo test -p krk-core --test trenntext` gab 18 bestanden und 1 ignoriert. Vom Prüfmodul in `crates/krk-ui/src/appkit/editor.rs` ist allein die Messprobe gelesen. Die `.record.json`-Dateien der Werkbank im Bereich sind keine Gegenstände einer Code-Durchsicht. Geöffnet wurden die Narrative, der Plan und die vier Entscheide.

## Summary

Der Kern ist sauber gebaut. Der eine Leser, die Byte-Bereiche und die Spleißung halten im Speicher, was der Plan verlangt: jede Handlung ersetzt allein die berührten Bytes. Auch Tasten, Zulässigkeit, Vorschau und Fallunterscheidungen sind ohne Befund. **Auf der Platte gilt das nicht vollständig**: eine CRLF-CSV mit mehrzeiligen Zellen, also eine gewöhnliche Excel-Datei, ändert beim ersten Sichern den Wert jeder solchen Zelle, auch unberührter. Das widerspricht der Directive, und der Nutzer hat diesen Fall im Entscheid nicht vorgelegt bekommen. Er sollte vor der Auslieferung behoben oder ausdrücklich entschieden werden.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 1 |
| Medium | 2 |
| Low | 3 |

## Findings by theme

### Treue der gesicherten Bytes

**H1. Ein Umbruch in einer Zelle einer CRLF-CSV wird bei jedem Sichern zu `\r\n` (High).** `Dateiform::der_bytes` (`crates/krk-core/src/text/datei.rs`) liest allein das erste Zeilenende, ohne Rücksicht auf Anführungszeichen. `in_dateiform` wandelt beim Sichern **jedes** `\n` zu `\r\n`. Excel schreibt Sätze mit `\r\n` und Umbrüche in Zellen als `\n`. Nach einem Sichern trägt jede mehrzeilige Zelle `\r\n`, ihr Wert ist ein anderer. Steht der erste Umbruch in einer gequoteten Kopfzelle, kippt die Erhebung, und jede Satzgrenze verliert ihr `\r`. `HowTo.md` („jede andere Zeile kommt Zeichen für Zeichen zurück“) und `CLAUDE.md` („der Rest kommt Byte für Byte zurück“) behaupten das Gegenteil; der Absatz direkt danach in `HowTo.md` nennt den Fall. Die Probe `eine_excel_csv_behaelt_marke_und_zeilenenden_und_jede_andere_datei_nicht` (`crates/krk-ui/src/editormodell.rs`) nimmt eine Datei ohne mehrzeilige Zelle und sieht ihn deshalb nicht. Behebung: nur die Satzgrenzen wandeln, die der Satzindex kennt, und `crlf` am ersten Zeilenende außerhalb von Anführungszeichen messen. Weil das die Antwort auf den Entscheid zur Sicherungsform verschiebt, braucht es eine Zeile vom Nutzer.
Issue: `261019-0037-ein-umbruch-in-einer-zelle-einer-crlf-csv-wird-bei-jedem-sichern-zu-crlf-auch-in-unberuehrten-zeilen.md`.

**L1. Vorschau und Editor zerlegen eine CSV mit einzelnem `\r` verschieden (Low).** Die Vorschau liest Bytes, und ein einzelnes `\r` ist dort ein Feldzeichen (`trenntext/index.rs`, `satz_abtasten`). Der Editor liest seinen Stand, in dem `gehaltene_form` jedes `\r` zu `\n` gemacht hat. Eine reine CR-Datei ist in der Vorschau eine Zeile und im Editor eine Tabelle. Ein `\r` in einem ungequoteten Feld teilt im Editor den Satz, und das Sichern schreibt die Teilung.
Issue: `261019-0037-vorschau-und-editor-zerlegen-eine-csv-mit-einzelnem-cr-verschieden.md`.

Ohne Befund geprüft: eine Datei, die keine Trenntextdatei ist, bekommt `Dateiform::GEHALTEN` (`Editormodell::uebernehmen`, `trenntextart`) und sichert wie vor dem Paket. Die Eintragsdateien des Notizordners und `secrets.txt` bleiben ausgeschlossen. BOM und `sep=`-Vorspann kommen zurück. Nicht-UTF-8 weist der Editor ab, und die Vorschau zeigt die Angaben. Ein Wert mit `\r` aus der Zwischenablage oder aus einer Zelle geht durch `Editormodell::bearbeiten` und damit durch `in_gehaltene_form`; der Stand bleibt ohne `\r`. Den abschließenden Umbruch hängt `sicherungsform` weiter an. Das ist die Regel vom 260808 und kein Befund dieses Pakets.

### Korrektheit der Tabelle

**M1. Eine CSV, die keine Tabelle ergibt, schaltet die Ansicht für jede folgende Datei auf Roh (Medium).** `Editorbereich::tabellenform_pruefen` ruft `ansicht_umschalten`. Die Ansicht gehört nach C3 nicht der Datei (`die_ansichtswahl_bleibt_ueber_einen_dateiwechsel_stehen`). Ein offenes Anführungszeichen oder mehr als `HOECHSTENS_SPALTEN` Spalten, auch beim Neuladen oder nach „Trennzeichen wechseln“, lassen die nächste Markdown- oder Eintragsdatei in der Rohansicht aufgehen, und nichts stellt die Ansicht zurück. Behebung: die Verweigerung an die Datei hängen (`Trenntextlage`) und in `editorform_der_datei` auf `Editorform::Text` abbilden, ohne `modell.ansicht` zu ändern.
Issue: `261019-0037-eine-csv-die-keine-tabelle-ergibt-schaltet-die-ansicht-fuer-jede-folgende-datei-auf-roh.md`.

**L2. Ein Neuladen derselben CSV nimmt das gewechselte Trennzeichen still zurück (Low).** `trenntext_vorbereiten` erkennt bei jedem `Ladeausgang::Geoeffnet` neu, auch beim Neuladen. Breiten und Auswahl bleiben, die Wahl des Nutzers fällt.
Issue: `261019-0037-ein-neuladen-derselben-csv-nimmt-das-gewechselte-trennzeichen-still-zurueck.md`.

**L3. `cmd+v` ohne Text in der Ablage bleibt stumm (Low).** Der Rumpf von `paste:` an `Eintragstabelle` kehrt ohne Meldung zurück, und `Trenntextantwort::AblageOhneText` bleibt für diesen Fall unerreicht.
Issue: `261019-0037-cmd-v-in-der-csv-tabelle-ohne-text-in-der-ablage-bleibt-stumm.md`.

Ohne Befund geprüft:

- `Zeilen::feld_der_tabellenspalte` und `tabellenspalte_des_feldes` sind die eine Umrechnung. Jede Spaltennummer von AppKit (`columnForView:`, `clickedColumn`, `columnAtPoint:`) geht durch `feld_an`, und `ziel_der_zelle` schützt vor einer Spalte jenseits der letzten.
- `Zellauswahl` beschneidet bei jeder Handlung auf das `Tabellenmass`, gehalten von `die_zellauswahl_ist_immer_ein_rechteck_mit_mindestens_einer_zelle`.
- `bereich_einsetzen` über den Rand hinaus hängt Zeilen an, füllt Sätze auf, lässt gleiche Werte in ihrer Rohform stehen und ist eine Ersetzungsliste, also ein `umbau_anwenden` und ein `cmd+z`.
- Die Spleißung (`spleissen`) und die Sonderfälle am Dateiende in `zeilen_loeschen`, `zeilen_verschieben` und `spalten_loeschen` stimmen mit dem Modulkopf von `trenntext/handlung.rs` überein.
- `esc` folgt über `esc_regel` der Aufgabenzelle.
- `zelle_uebernehmen` steht als erste Handlung in `tabellenhandlung_ausfuehren` und `trennzeichen_wechseln`; beide stehen in der Rufliste der Probe.

### Tasten und Zulässigkeit

Ohne Befund:

- Die fünf neuen `Kommando`-Werte stehen in `KENNUNGEN`, `wirkungsbereich`, `bereich_des_kommandos`, im Ausführungszweig (`zweigproben::BEFEHLE`), in den drei Sprachtabellen (der Übersetzer hält sie) und in `resources/default-keymap.toml`.
- `Wirkungsbereich::Trenntext` und `Csv` stehen auf `Seite::Editor`, verengen nichts und verzweigen in `form_passt`, `datei_passt`, `folge_passt`, `fokus::wirkt` und `seite` ohne Auffangzweig.
- Die vier neuen Kombinationen tragen `cmd`, `zellentaste::gehoert_der_zelle` lässt sie also in einer laufenden Zelle wirken.
- `tabellentaste` nimmt allein Pfeile und `return` ohne `cmd`, `opt` und `ctrl`. Der Fänger greift nur, wenn die Form `Trenntext` ist und der Ersthelfer des Schlüsselfensters die Tabelle selbst. Eine laufende Zelle (Feldeditor als Ersthelfer), ein Blatt (anderes Schlüsselfenster), die Quicknote (andere Form) und das Dateifenster fallen also durch.
- Das nackte `delete` löscht die Zeilen der Auswahl ohne Rückfrage. Das verlangt E13, und `cmd+z` nimmt es zurück.

### Vorschau

Ohne Befund: gelesen wird auf dem Faden der Vorschau (`laden`, `archiveintrag_laden`). Über 1 MB wird am letzten vollständigen Satz gekürzt, mit Rücksicht auf ein zerteiltes Zeichen. `bricht_um` und `einzufaerben` verzweigen über `Inhalt` ohne Auffangzweig, und jede andere Datei bricht um wie vorher.

### Leistung

**M2. Die Zellübernahme an 16 MiB ist ohne `setString:` gemessen (Medium).** Gefahren am 261019, `--release`: „Zellübernahme ohne setString: 58,85 ms“, Spalte einfügen allein im Kern 90,03 ms. Es fehlen `setString:` der verborgenen Textfläche, die Abschrift in `umbau_anwenden`, der Durchlauf in `Editormodell::bearbeiten`, der Textvergleich in `trenntextsicht_fuer` und das Neuladen der Tabelle. Die Schließbedingung des Plans (über 100 ms ein Defekt) ist damit nicht entschieden. `inference:` sie wird überschritten.
Issue: `261019-0037-die-zelluebernahme-an-16-mib-ist-ohne-setstring-gemessen-und-die-100-ms-grenze-des-plans-ist-nicht-entschieden.md`.

Ohne Befund: der Index entsteht allein bei gezeigter Tabelle (`sicht_zum_nachzug`). Zellen kommen über `makeViewWithIdentifier:owner:`, und `Satzspeicher` zerlegt je Zeile einmal. `HOECHSTENS_SPALTEN` ist mit Messung auf 256 gesenkt.

## Cross-cutting observations

- **„Derselbe Leser“ heißt nicht „dieselbe Eingabe“.** H1 und L1 haben eine Wurzel: der Editor zerlegt einen Stand, den `gehaltene_form` schon gewandelt hat, und schreibt beim Sichern eine Form zurück, die nur zwei Bits kennt. Die Satzstruktur, die der Kern genau kennt, kommt beim Sichern nicht zum Zug. Beide Befunde schließen sich, wenn der Schreibweg einer Trenntextdatei über den Satzindex geht statt über ein globales `\n` → `\r\n`.
- **Ein Zustand der Datei an einer Stelle, die nicht der Datei gehört.** M1 (Ansicht) und L2 (Dialekt) verwechseln „geöffnete Datei“ und „Editor“ in entgegengesetzter Richtung: die Ansicht wird pro Datei verstellt, obwohl sie global ist; der Dialekt wird global neu erhoben, obwohl er pro Datei gilt. Ein Feld in `Trenntextlage`, das beides trägt, löst beide.
- Die neuen Befehle kommen bei einem Nutzer mit eigener `keymap.toml` unbelegt an. Das ist der offene Defekt `260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`; `HowTo.md` nennt den Weg über F1. Kein neuer Befund.

## Recommended sequencing

1. **Vor der Auslieferung:** H1, oder eine ausdrückliche Zeile des Nutzers, dass der Preis gilt. Im zweiten Fall berichtigen `HowTo.md` und `CLAUDE.md` die Aussage „Zeichen für Zeichen“ und „Byte für Byte“.
2. **Vor dem Schließen des Pakets:** M2 (Bedingung des Plans), M1.
3. **Aufräumen:** L1, L2, L3.

Der Test am Bündel und die Entscheidung über den Abnahmelauf (E21) stehen als Nutzerarbeit unverändert aus. Kein Agent kann sie fahren.

## Verification

- `cargo test -p krk-core --test trenntext`: 18 bestanden, 1 ignoriert.
- `cargo test --release -p krk-ui -- --ignored --nocapture messung_der_csv_tabelle_an_16_mib`: Zahlen unter M2.
- `cargo test --workspace` am Stand `c97e264e`: alle 36 Testziele `ok`, keines gescheitert.
- `cargo clippy --workspace --all-targets -- -D warnings`: ohne Warnung durchgelaufen.
- Grün heißt hier nicht befundfrei: H1, M1, L1 und L2 sieht keine vorhandene Probe.
