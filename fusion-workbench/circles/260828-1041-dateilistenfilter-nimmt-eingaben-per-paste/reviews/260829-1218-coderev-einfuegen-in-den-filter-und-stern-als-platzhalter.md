# Durchsicht der Runde 21: Einfügen in den Filter und `*` als Platzhalter

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `c6c86cb..415ef6f`
**Not-opened:** none
**Date:** 2026-08-29
**Spec:** `planning/260829-1052_o_spec-einfuegen-in-den-filter-und-stern-als-platzhalter.md`; **Plan:** `planning/260829-1102_p_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`
**Verification:** `cargo test --workspace` 860+5+155 grün, `cargo clippy --workspace --all-targets -- -D warnings` ohne Ausgabe, `cargo fmt --all --check` ohne Ausgabe, am Baum `415ef6f` (HEAD) gefahren.

## Summary

Die vier Code-Commits setzen A1 bis A13 und B1 bis B9 so um, wie Spec und Plan sie festlegen; der Musterabgleich ist korrekt und vollständig, die Reinigung folgt den fünf Schritten in der Reihenfolge von A3, und `paste:` geht denselben Weg wie `copy:` und `cut:` mit denselben zwei Fragern der Regel. Kein Critical, kein High, kein Medium. Drei Low-Befunde sind als Defekte abgelegt, alle Randfälle außerhalb der Festlegungen oder Prosa; die zwei offenen Datensätze der Runde (C6.6, `cmd+v` mit Dateiverweis) sind nicht doppelt gefilt.

## Totals

Critical 0 / High 0 / Medium 0 / Low 3 (gefilt) + 2 Beobachtungen ohne Datensatz.

## Findings by theme

### 1. Der Musterabgleich `traegt_die_folge` (f4ba58d) — korrekt, keine Befunde

`crates/krk-core/src/verzeichnis/filter.rs:187-197`:

```rust
let name = name.to_lowercase();
let mut ab = 0;
for stueck in &muster.stuecke {
    match name[ab..].find(stueck.as_str()) {
        Some(stelle) => ab += stelle + stueck.len(),
        None => return false,
    }
}
true
```

- **Greedy ohne Rückverfolgung ist vollständig:** die Induktion im Doc-Kommentar (`filter.rs:174-180`) trägt; für ein Muster mit `*` als einzigem Sonderzeichen verliert die jeweils erste Fundstelle keine Zerlegung. Probe C7.3 (`a*a*a` gegen `aaa`, `aa`, `a-a-a`) und `aa*b` gegen `aab`/`ab` decken Überlappung.
- **Grenzfälle geprüft:** leeres Muster → `stuecke == [""]`, `find("")` = `Some(0)`, jeder Name trifft (Probe `ein_leerer_filtertext_steht_in_jedem_namen`). `*` → `["", ""]`, `**`/`***` → nur leere Stücke, jeder Name inklusive des leeren trifft (Probe C5.3). Muster länger als der Name → erstes nicht-leeres Stück findet nichts → `false`, kein Panic, da `name[ab..]` mit `ab <= name.len()` gültig bleibt. Ohne `*` ist es genau ein `find`, also `contains` wie vor der Runde (B2, C5.4).
- **Byte-Indizes und Unicode:** `ab` ist ein Byteindex in den **kleingeschriebenen** Namen; `stelle + stueck.len()` liegt auf einer Zeichengrenze, weil `stueck` dort gerade gefunden wurde. Längenänderung beim Kleinschreiben (`İ` → zwei Zeichen) trifft beide Seiten konsistent, da `Muster::aus` und der Name dieselbe `to_lowercase` durchlaufen. Kein Weg zu einem Slice-Panic.
- **Kosten:** je Eintrag ein `to_lowercase` wie zuvor plus ein `find` je Stück über disjunkte Restbereiche; ohne `*` unverändert. `Muster` wird einmal je Änderung gebaut (`modell.rs:1175`) und einmal je Durchlauf geklont (`tabs.rs:920`). L7/L10 liegen auf Wegen ohne Filtertext (Spec-Abschnitt zu C8, am Baum bestätigt: `messen.rs` und `messmodus.rs` setzen keinen Filter).
- **Schwelle ohne `*`:** `inhalt_wirkt` (`modell.rs:1103-1110`) zählt `chars().filter(!= '*')` am Filtertext, nicht am Muster — bewusst, da Kleinschreibung die Zeichenzahl ändert; die eine Stelle bleibt (C6.5).

### 2. Die Reinigung `filtertext_aus` (1b0939a) — zwei Low-Befunde

`crates/krk-core/src/zwischenablage.rs:165-195`. Reihenfolge stimmt mit A3: Verweise/Leer → Zeilenenden am Ende → Mehrzeiligkeit → `file:`-Auflösung → letzter Bestandteil → Zeichenregel plus `:` → leer.

- `letzter_bestandteil` (`:198-202`): `rsplit('/').find(non-empty)` liefert für `Ordner/` → `Ordner`, `/` und `//` → `""` → `NichtsTragbar` (Probe `was_nach_der_reinigung_leer_ist_traegt_nichts`). Kein `Path::file_name`, dessen `None` bei `/` und `..` eine zweite Regel wäre — begründet im Modulkopf. Korrekt.
- **Windows-Pfade** (`C:\Users\x\a.txt`): kein `/`, also ganzer Text; `:` fällt → `C\Users\x\a.txt`. Das ist die Regel von A3 wörtlich; A3 nennt allein `/`. Keine Abweichung vom Spec, als Beobachtung notiert, nicht gefilt.
- **Low, gefilt** `issues/260829-1216_o_…wagenruecklauf…`: `trim_end_matches(['\n','\r'])` am Ende, danach nur `contains('\n')`; `erste\rzweite` ist nicht mehrzeilig, `\r` fällt als Steuerzeichen, Ergebnis `erstezweite`. Einzeiler-Fix.
- **Low, gefilt** `issues/260829-1215_o_…keine-hoechstlaenge…`: keine Höchstlänge in Reinigung oder `text_anhaengen`. A1–A13 setzen keine; die Folge einer sehr langen Zeile ist ein Filtertext, der Ordnerwechsel übersteht und bei jedem Anschlag `str::find` mit langer Nadel über den ganzen Bestand treibt. Nicht gemessen; Festlegung durch den Nutzer, ob Grenze oder ausdrückliches „keine“.
- Beobachtung ohne Datensatz: `lesen_aus` (`appkit/zwischenablage.rs:268-280`) liefert `None` für Text, der nach `trim()` leer ist; ein Text aus nur Leerzeichen kommt deshalb als `Leer` an und meldet „trägt keinen Text“, obwohl das Leerzeichen ein Namenszeichen ist (A10). Das ist das Verhalten von `lesen` seit der Runde 6, A11 verlangt genau diese zwei Leser, und die Meldung ist nicht falsch.

### 3. `paste:` am Delegierten, Zulässigkeit, Menü (3722c89) — keine Befunde

- `anwendung.rs:925-928` beantwortet `paste:` über `einfuegen_ausfuehren` → `bearbeiten_am_dateifenster` (`:3218-3228`), dem einen Vorspann für alle drei Selektoren: `lage()`, `dateiablage_zulaessig`, `befehlsantwort_beidseitig_loeschen`, aktive Seite. `dateiablage_ausfuehren` ist ein Einzeiler darauf. Die zwei Frager der Regel bleiben zwei; `die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` grün.
- `validateMenuItem:` (`:988-993`) fragt für `paste:` dieselbe Regel auf derselben `Lage`; kein Zweig nach Ablageinhalt (A9). Fokus im Editor/Umbenennungsfeld/Pfadeingabe: `NSTextView`/Feldeditor beantwortet `paste:` vor dem Delegierten. Vorschau (nicht bearbeitbare `NSTextView`): AppKit graut aus, die Kette erreicht den Delegierten nicht. Betrachter (`PDFView`) und Lesezeichenleiste: die Kette erreicht den Delegierten, `Lage.fokus != Dateifenster` → `false` (Tafel `die_dateiablage_wirkt_genau_mit_dem_fokus_im_dateifenster`). `method(paste:)` steht genau einmal im Baum (Probe im Betrachter).
- `Anspruch` bekommt keinen dritten Wert; die drei Antworten wären byteweise gleich. Vertretbar; der Modulkopf schreibt aus, dass `Dateiablage` seitdem „der Einhängepunkt“ heißt, und nennt die Umbenennungsfrage als offen im Plan.
- `aus_zwischenablage_einfuegen` (`tabelle.rs:1994-2008`): die `borrow_mut` auf `tabs` endet im Block vor `nach_filteraenderung`, exakt wie `filterzeichen_tippen` (`:2161-2165`). `nach_filteraenderung` läuft einmal; `durchlauf_nachziehen` darin bricht ab und stößt einmal neu an (A7, C1.4). Ein geglücktes Einfügen schreibt keinen Satz; der Vorspann hat die vorige Befehlsantwort gelöscht (C2.8).
- `einfuegequelle_aus` (`appkit/zwischenablage.rs:361-370`) setzt `dateiverweise` vor `lesen_aus` — die Rangfolge von A2 — und ist Zusammensetzung, kein dritter Griff. `grep NSPasteboard` außerhalb der Hülle trifft vor und nach der Runde dieselben sieben Dateien (C4.1). Keine neue AppKit-Methode, also nichts für den Untergrenzen-Abschnitt (C4.6).
- `#[must_use]`: `Muster::aus`, `dateiablage_zulaessig`, `einfuegen_abgewiesen`, `inhalt_wirkt` tragen es; `filtertext_aus` bewusst nicht (`Result` trägt es, `double_must_use`). `einfuegen_abgewiesen` verzweigt über `Einfuegehindernis` vollständig ohne Auffangzweig (Constraint 3).
- `Cargo.lock` und `Cargo.toml` unverändert (C7.4). `default-keymap.toml` trägt nur Kommentaränderungen; keine neue Zeile, kein neues Kommando (C1.9).

### 4. Zählproben und Prosa (415ef6f) — ein Low-Befund für den Kurator

- `die_zeichenregel_hat_drei_rufer_und_der_vergleich_drei` (`tests/verzeichnis.rs:3694`) nennt die drei Zeichenrufer (`krk-core/src/zwischenablage.rs`, `krk-ui/src/appkit/tabelle.rs`, `krk-ui/src/belegungsmodell.rs`) und die drei Vergleichsrufer mit Namen (B8, C4.3, C7.1). Korrekt umbenannt statt umgangen.
- **Low, gefilt** `issues/260829-1217_o_claude-md-nennt-die-zaehlprobe-mit-ihrem-alten-namen…`: CLAUDE.md nennt den alten Probennamen und beschreibt den Vergleich als Teilzeichenfolge. Für den Kurator: beide Aussagen sind mit `415ef6f` falsch geworden; die Behebung gehört ans CLAUDE.md-Tor beim Abschluss. Die übrige Aussage des Absatzes („Wie viele Rufer jede hat, sagt die Zählprobe … und nicht diese Zeile“) hält weiter und ist die richtige Bauart.
- C6.6: der offene Datensatz `issues/260829-1201_o_…` beschreibt die Lage zutreffend — `zeilengrund_von` kurzschließt am Namen, `auftraege()` bleibt leer, der Durchlauf bekommt nichts; die Probe hält beide Hälften getrennt. Nicht doppelt gefilt. Ebenso nicht: `decisions/260828-1041_o_…dateiverweis…`, den der Modulkopf der Hülle und die Keymap-Kommentare nun zitieren.

## Cross-cutting observations

- **Ein Weg, drei Selektoren, zwei Frager:** die Runde hat das Muster der Runde 22 nicht kopiert, sondern zu einem Vorspann verallgemeinert. Das ist der Grund, warum die Zählprobe der Frager nicht wachsen musste.
- **Die Zeichenregel hat jetzt drei Rufer in zwei Kisten**, und der dritte im Kern verschärft sie um `:`. Wer die Regel später ändert, ändert damit auch die Reinigung; der Modulkopf von `zwischenablage.rs` sagt es.
- **Zwei Spec-Prosastellen sind gegenüber dem Baum ungenau** (C6.6 und die „Zeilenende“-Formulierung in A3 Schritt 2 gegen `\r`); beide sind gefilt, keine ändert Code der Runde.

## Recommended sequencing

Kein Release-Blocker. Vor dem Abschluss: nichts zwingend. Nach dem Abschluss: `260829-1216` (Einzeiler-Fix mit Probe), `260829-1217` am CLAUDE.md-Tor, `260829-1215` als Nutzerfrage.
