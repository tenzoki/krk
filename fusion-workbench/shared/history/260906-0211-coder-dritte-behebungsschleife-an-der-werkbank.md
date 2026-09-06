# Coder-Sitzung — 260906-0211: dritte Behebungsschleife, Werkbankbefunde

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Auftrag:** die offenen Rundendatensätze ohne Codebezug, die fünf mit `xtask`-Bezug, und vier Formbefunde im gemeinsamen Speicher.
**Grenze:** `fusion-workbench/`, `xtask/`, `crates/krk-bench/`. Nicht `crates/krk-core/`, nicht `crates/krk-ui/`, nicht `CLAUDE.md`, nicht `README.md`, nicht `Makefile`. Kein Git-Kommando über den Baum.

## Verifikation

- `cargo test -p xtask -p krk-bench` — exit 0 (169 + 67 Proben)
- `cargo clippy -p xtask -p krk-bench --all-targets -- -D warnings` — exit 0
- `cargo fmt -p xtask -p krk-bench -- --check` — exit 0

Bestandsnachweis, jeweils vorher und nachher gefahren, siehe die Abschlussvermerke der einzelnen Datensätze.

## Was der Auftrag anders vorfand, als er sagte

**Der Bestand ist nicht 19 plus 5, sondern 18 plus 5.** Gezählt über `fusion-workbench/circles/*/issues/*_o_*.md` mit dem Kriterium „zitiert keine Datei unter `crates/` oder `xtask/`": 139 offene Rundendatensätze insgesamt, 18 ohne Codebezug, 8 mit `xtask`-Bezug, davon 5 ohne zusätzlichen `crates/`-Bezug — die fünf des Auftrags, Datei für Datei dieselben. Einer der 18 nennt daneben fünf `.rs`-Dateien und hat damit sehr wohl Codebezug (`260830-1317_*_c1-1-nennt-vier-feldbreiten-…`).

## Die zwei nachgemessenen Zahlen

**Abschlussvermerke.** Der Auftrag nannte 654 geschlossene Datensätze, 596 in Konventionsform, 19 mit verschobenem Doppelpunkt, 39 ohne Zeile. Gemessen am 260906-0130 über den ganzen Speicher einschließlich Archiv: **692 geschlossen, 633 Konvention, 18 verschobener Doppelpunkt, 38 in Fettform, 9 ohne jede Zeile.** Die dritte Gestalt, `**Resolved:**` beziehungsweise `**Resolved 260812** —`, fehlt in jeder bisherigen Erhebung; sie fiel in den Eimer „keine Zeile" und ist mit 38 Fällen die größte. Elf davon stammen aus der Runde 23, sie ist also keine Altlast, sondern lief weiter zu.

**Leere Vorlagenzeilen in Entscheidungsdatensätzen.** Der Auftrag nannte 39 Dateien mit 69 Schlüsselfällen. Genau so gemessen: 197 Datensätze live, **39 Dateien, 69 Fälle** (`Answered` 32, `Implemented` 33, `Deferred` 2, `Superseded by` 2); mit Archiv 232 / 45 / 79.

## Geschlossen

| Datensatz | Was getan wurde |
|---|---|
| `260826-1024_*_acht-offene-defektdatensaetze-tragen-eine-leere-resolved-zeile-…` | 7 offene (nicht 8) und 6 geschlossene Datensätze von der leeren Vorlagenzeile befreit; einer, dessen Notiz auf der Folgezeile begann, hinter den Doppelpunkt gezogen |
| `260818-0710_*_forty-three-closure-notes-are-written-in-a-form-no-resolved-sweep-finds.md` | 65 Abschlussvermerke in drei Gestalten auf `Resolved: …` gebracht, Wortlaut unverändert; 9 fehlende Zeilen angehängt statt vorhandene Prosa umzuschreiben; die dauerhafte Hälfte als Probe gebaut |
| `260820-2056_*_dreissig-entscheidungsdatensaetze-tragen-eine-leere-vorlagenzeile-…` | als Lage angenommen: Weg 2 ist in fusion vollzogen (Vorlagenblock aus dem Template entfernt, Zufluss am Bestand nachweisbar versiegt), Weg 1 von derselben Regel ausdrücklich verboten |
| `260813-0643_*_ein-zutrag-des-ontorev-an-die-runde-6-ist-nirgends-eingetragen.md` | der Absatz aus der Durchsicht steht jetzt an `260812-1527_*` |
| `260816-2307_*_c2-6-beschreibt-das-verdoppeln-des-anfuehrungszeichens-…` | C2.6 und die mitbetroffene C3.7 im Spec berichtigt, Nachweis unverändert |
| `260813-1345_*_der-baumzweig-der-abbruchmeldung-nennt-die-version-…` | Meldung um „aus der Cargo.toml" ergänzt, C3.8-Probe auf zwei Zweigproben aufgeteilt |
| `260813-1345_*_der-doc-kommentar-an-bundle-version-nennt-eine-sichtbarkeit-…` | `PLATZHALTER` auf `pub(crate)` gezogen |
| `260813-1345_*_die-eine-messung-die-der-plan-als-gegenmassnahme-nennt-…` | Messung gefahren, in einem Wegwerf-Workspace; `bundle::VERSION` veraltet nicht |

## Halb behoben, Marker bleibt offen

`260818-0807_*_vierzehn-tote-zeiger-…`: 16 Zeiger (nicht 14) auf die Sternform gezogen, dazu 5 weitere, die der Datensatz nicht kennt, und das Feld `**Active spec/plan:**` der Runde 16. Über die lebenden Werkbankdateien stehen danach **7 tote Zeiger**, alle der Klasse „der Marker ist die Aussage": sechs Zeilen einer Tafel mit den Spalten „Marker heute"/„Marker danach" und ein Berichtigungsvermerk. Der breite Fix — eine Probe, die jedes Zitat auflöst — bleibt offen, weil er zwei offene Antworten braucht (`260818-0753_*`, `260818-0201_*`).

## Gebaut

`xtask/src/werkbank.rs`, ein `#[cfg(test)]`-Modul mit drei Proben über `fusion-workbench/`:

- `jeder_geschlossene_defektdatensatz_traegt_einen_abschlussvermerk`
- `kein_offener_defektdatensatz_traegt_eine_leere_abschlusszeile`
- `der_durchlauf_findet_die_defektdatensaetze_und_sonst_nichts` — ohne sie wäre eine leere Trefferliste nicht von einem Suchmuster zu unterscheiden, das nichts findet

Sie laufen mit `cargo test --workspace` und damit mit `make check`, ohne dass das `Makefile` angefasst werden musste. Ohne `fusion-workbench/` im Baum kehren sie zurück, statt rot zu werden: ein Auszug ohne Werkbank ist ein zulässiger Zustand. Der Durchlauf hält genau einen Verzeichnisleser offen, nach dem Vorbild von `krk_core::verzeichnis::durchlauf`.

## Neu abgelegt

- `260906-0203_*_darf-ein-agent-den-spec-oder-plan-einer-geschlossenen-runde-berichtigen.md` — sieben offene Defektdatensätze aus sechs Runden hängen daran
- `260906-0206_*_werden-dateinamen-mit-vorauslaufendem-zeitstempel-umbenannt-oder-vermerkt.md` — vier offene Defektdatensätze aus vier Runden hängen daran
- `260906-0212_*_sechzehn-plan-und-specdateien-geschlossener-runden-stehen-auf-offen-oder-in-arbeit.md`
- `Answer located:` an `260818-0201_*_does-a-cross-references-line-…`; der Marker bleibt `_o_`

## Unangetastet, mit Grund

Die Datensätze zu `resources/default-keymap.toml` und `resources/default-readers.toml` gehören dem `ontocoder`. Die zwei Datensätze zum `## Directive`-Abschnitt der Runde 9 gehören dem Shaper im Modus `portfolio-activation`. `260818-0753_*` empfiehlt eine Zeile in `CLAUDE.md`, und die Datei liegt außerhalb der Grenze dieses Laufs. `260814-1247_*_sechzehn-der-neunundzwanzig-buendelkriterien-…` verlangt einen Abnahmelauf am Bündel, also Nutzerarbeit. Sieben weitere warten auf `260906-0203_*`.

`260824-1655_*_sechs-speicher-unter-archive-bleiben-ohne-profil-…` liegt der Art nach falsch: er stellt selbst fest, die Frage sei eine Nutzerentscheidung und kein Mangel, und gehört damit nach `decisions/`. Gemeldet, nicht umgeräumt.
