# Schlussprüfung: Markdown-Tabellen in der Vorschau (Code)

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `190dbeaa..f70c6202`
**Not-opened:** none
**Review domain:** code
**Cross-references:** 261031-1645-markdown-vorschau-richtet-tabellenspalten-aus.md, 261031-1720-plan-markdown-vorschau-richtet-tabellenspalten-aus.md, 261101-0015-ab-welcher-groesse-zeigt-die-vorschau-eine-markdown-tabelle-nicht-mehr-als-eine-nstexttable.md, 261101-0015-zellenmerkmal-misst-je-tabellenzelle-schrift-und-leerzeichenbreite-neu.md

Geöffnet: die Code-Commits `83ee19dc`, `fe3303c1`, `75850c88` mit allen vier geänderten Dateien (`git diff --stat 190dbeaa..f70c6202 -- crates Cargo.toml`: `Cargo.toml`, `crates/krk-ui/src/appkit/textmerkmale.rs`, `crates/krk-ui/src/hervorhebung.rs`, `crates/krk-ui/src/markdown.rs`), dazu Plan, die drei Entscheide und den Defekt des Pakets. `HowTo.md` liegt nicht in `crates` oder `Cargo.toml` und war laut Auftrag ausgenommen; die übrigen Commits der Spanne tragen Werkbank, Ereignisse und Version.

## Zusammenfassung

Der Bau trägt, was Plan und Entscheid 261101-0015 (Möglichkeit 3) verlangen: Zerlegung der Struktur im Kern, `NSTextTable` je Abschnitt in AppKit, Teilung zu `ABSCHNITTSZEILEN` = 100, Rückfall auf Quelltext über `ZELLENGRENZE` = 20 000, Tabschritt einmal je Ruf. Gefunden sind ein Fehler im Abstand hinter einer Tabelle mit leerer letzter Zelle und zwei Stellen, an denen ein Kommentar nicht mit dem Code übereinstimmt.

## Totals

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 0 | 0 | 1 | 2 |

## Befunde

### Medium: eine leere letzte Zelle zählt als Leerzeile

`Zerlegung::absetzen` (`crates/krk-ui/src/markdown.rs`) zählt die `\n` am Textende, um den Wunsch aufzufüllen. Das `ZELLENENDE` einer leeren letzten Zelle steht dort und zählt mit. Gemessen an einer Kopie des Baums auf `f70c6202`:

- `"| a | b |\n|---|---|\n| 1 |  |\n\nDanach.\n"` ergibt `"a\nb\n1\n\nDanach."`. Das zweite `\n` ist der Absatz der Zelle (1,1), „Danach.“ steht ohne Leerzeile darunter.
- Mit gefüllter letzter Zelle: `"…\n2\n\nDanach."`, also mit Leerzeile.
- Zwei Tabellen, die erste mit leerer letzter Zelle: `"a\n\nb\n2\n"`. Die zweite Tabelle beginnt unmittelbar hinter der leeren Zelle.

Die Spalten stehen trotzdem; betroffen ist der Abstand zum nächsten Block. Keine Probe deckt den Fall: `ein_absatz_nach_einer_tabelle_steht_hinter_einer_leerzeile` nimmt eine gefüllte Zelle, `PLANTABELLE` hat keinen Block dahinter. Der Plan nennt das Risiko in seiner Tabelle („Der Wunschmechanismus der Abstände verschluckt ein leeres Zellende“) und hat es für das Zellende behoben, nicht für den Abstand danach.

Defekt: `261101-0924-eine-tabelle-mit-leerer-letzter-zelle-verliert-den-abstand-zum-naechsten-block.md`.

### Low: Modulkopf zu Tabellen ohne äußere Striche

`markdown.rs`, Modulkopf `# Tabellen`: „dort liefert ein Wort das Wort“. Gemessen an `"a | b\n--|--\n1 | 2\n"`: „b“ liefert `"| b"`, „1“ liefert `"1 "`, „a“ liefert `"a "`. Das `|` kommt über die Halbregel von `Quellbezug::quelltext`, und weil die Zeile keine Klammer trägt, schließt die zweite Stufe nicht auf. Keine Probe hält den Satz.

Defekt: `261101-0924-der-modulkopf-sagt-in-einer-tabelle-ohne-aeussere-striche-liefert-ein-wort-das-wort.md`.

### Low: Zahl im Cargo-Kommentar

Wurzel-`Cargo.toml` über `pulldown-cmark`: „Vier Zahlen … und eine fuenfte“. Darunter stehen die fünfte und danach zwei weitere Messungen vom 261101. Die fünfte misst eine Datei über `ZELLENGRENZE`, deren Weg seit `75850c88` nicht mehr gefahren wird.

Defekt: `261101-0924-der-cargo-kommentar-zu-pulldown-cmark-kuendigt-fuenf-zahlen-an-und-fuehrt-mehr.md`.

## Geprüft ohne Befund

- **Kachelung und Fälle:** leere Zelle in der Mitte und am Dateiende, kurze und lange Zeile, `\|`, Betonung und Verweis in Zellen, Tabelle im Listenpunkt (Merkzeichen auf eigener Zeile, `tiefe` 1), im Zitat (`> ` fällt), ohne äußere Striche, unvollständige Tabelle als Absatz. Die Quellbereiche von `pulldown-cmark 0.13.4` habe ich in einer Kratzprobe nachgemessen: der Bereich eines Kopfes schließt seinen Umbruch auch ohne äußere Striche ein, also findet `zeile_oeffnen` das Ende der Trennzeile. Eine Zeile nach der Tabelle ohne Leerzeile wird laut GFM eine weitere Zeile mit aufgefüllter Zelle (Bereich 38..38), und das deckt der Weg über `schreiben(ZELLENENDE, bis)` ab.
- **Leerraumsatz in `schliessen`:** `laenge > 0 || quelle[bereich].trim().is_empty()` wird nur bei `laenge == 0` ausgewertet, also nicht über eine ganze Tabelle. Ein anderes Element mit einem Quellbereich, der nur aus Leerraum besteht, habe ich nicht gefunden (`inference:` wie im Plan).
- **Abschnitte:** erster Abschnitt Kopf + 100 Datenzeilen, jeder weitere 100, Nummer in `Tabellenlage::nummer`, Kopf nicht wiederholt; Proben `hundertundeine_datenzeile_sind_zwei_abschnitte`, `die_abschnittsgrenze_aendert_das_kopieren_nicht`.
- **Zellengrenze:** gezählt in `zelle_oeffnen`, Abbruch bei `zellen > grenze` im ersten Durchgang, zweiter Durchgang ohne `ENABLE_TABLES` und ohne Grenze, `ENABLE_TASKLISTS` bleibt in der Eintragsdatei. Höchstens zwei Durchgänge, beide linear. Proben `ueber_der_zellengrenze_stehen_die_tabellen_als_quelltext`, `die_zellengrenze_zaehlt_die_gemeldeten_zellen`.
- **AppKit:** `NSTextTable` je Nummer in einem `HashMap` für die Dauer des Rufs; `NSTextTableBlock` hält seine Tabelle selbst. `alloc` + `initWithTable…` gibt ein `Retained`, `NSArray::from_slice` hält den Block. `zuruecksetzen` → `grund_legen` ersetzt den Absatzstil über den ganzen Text, also fallen die `textBlocks` beim nächsten Setzen weg. `setWidth:type:forLayer:edge:` nur bei `tiefe > 0`, keine `rectEdge:`-Fassung. `separatorColor` ist dynamisch (macOS 10.14). `unsafe` neu allein in `appkit/textmerkmale.rs` (Fremdsymbol `NSParagraphStyleAttributeName`, `addAttributes_range`), keines in `markdown.rs` oder `hervorhebung.rs`. Den Untergrenzen-Abschnitt habe ich gegen die `use`-Zeilen gelesen: jeder neue Name steht drin. Die Baumproben laufen grün. Die SDK-Zeilennummern selbst habe ich nicht am SDK nachgeprüft.
- **Leistung:** `tabschritt` steht einmal vor der Schleife in `auszeichnungen_legen`, die Quelltextprobe hält das. Je Zelle bleiben ein Block, ein Absatzstil, ein `NSArray`, ein `NSDictionary`. Das ist nötig, weil jeder Stil seinen eigenen Block trägt. Einen quadratischen Weg im Betriebscode habe ich nicht gefunden; der quadratische Testhelfer `zellen` wird an der Grenze bewusst umgangen (`zellenzahl`).
- **Doc-Kommentare:** `Auszeichnung::Tabellenzelle` und seine Felder, `Spaltenausrichtung`, `ABSCHNITTSZEILEN`, `ZELLENGRENZE`, `rendern`/`durchgang`, `texttabelle`, `zellenmerkmal`, `textausrichtung` stimmen mit dem Code überein, mit Ausnahme der zwei Low-Befunde oben.

## Prüfläufe

- `cargo test --workspace -- --skip appkit::`: grün, kein Lauf mit `failed` ungleich 0.
- `cargo test -p krk-ui appkit::textmerkmale::`: 11 bestanden, darunter `zwei_zellen_einer_tabelle_teilen_ihr_nstexttable`, `auszeichnungen_legen_misst_den_tabschritt_einmal_je_ruf`, `jede_spaltenausrichtung_hat_ihre_absatzausrichtung`.
- `cargo clippy --workspace --all-targets`: ohne Warnung.
- Die Kratzproben liefen an einer Kopie aus `git archive f70c6202` im Scratchpad; im Baum ist nichts geändert.

## Querschnitt

Beide Befunde in `markdown.rs` haben dieselbe Ursache: der vorhandene Mechanismus wird ohne eigene Tabellenregel weiterverwendet (Auffüllen im Abstandswunsch, Halbregel und Klammer im Quellbezug). An den Fällen, die die Proben fahren, stimmt das. Die zwei Randfälle bekommen dadurch ein Ergebnis, das niemand geprüft hat.

## Reihenfolge

Nichts davon blockiert die Auslieferung; 3.14.0 ist schon getaggt. Der Medium-Befund gehört vor das Schließen des Pakets, die zwei Low-Befunde sind Kommentarpflege.
