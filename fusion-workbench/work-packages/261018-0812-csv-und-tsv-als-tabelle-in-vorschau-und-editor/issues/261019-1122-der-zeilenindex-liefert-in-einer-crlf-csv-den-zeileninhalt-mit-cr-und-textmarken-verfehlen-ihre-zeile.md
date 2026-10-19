Der Zeilenindex liefert in einer CRLF-CSV den Zeileninhalt mit `\r`, und Textmarken verfehlen ihre Zeile
---
Seit `ff2d7b76` trägt der Stand einer CSV oder TSV ihre Zeilenenden. `Zeilenindex::inhalt_der_zeile` schneidet aber genau ein Byte vor dem nächsten Zeilenanfang ab und gibt deshalb in einer `\r\n`-Datei jede Zeile mit `\r` am Ende zurück. Eine Textmarke (`cmd+d` im Editor), die eine Fassung vor 3.9.0 auf einer CSV-Zeile angelegt hat, trägt den Inhalt ohne `\r` und findet ihre Zeile in 3.9.x nicht mehr. Eine neue Marke schreibt das `\r` in `bookmarks.toml`.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261019-1122-gilt-der-stand-mit-den-zeilenenden-der-datei-als-antwort-auf-die-sicherungsform-einer-csv.md, 261019-1122-reviewer-csv-und-tsv-schlussdurchsicht-vor-dem-schliessen.md, 261019-0037-ein-umbruch-in-einer-zelle-einer-crlf-csv-wird-bei-jedem-sichern-zu-crlf-auch-in-unberuehrten-zeilen.md

**Schwere:** Low. Folge der neuen Bauweise; betrifft Textmarken in CSV-Dateien.

**Beleg (Stand `6eae327a`):**

- `crates/krk-core/src/text/zeilen.rs`, `Zeilenindex::inhalt_der_zeile`: `Some(naechster) => naechster.saturating_sub(1)`, kommentiert mit „Genau ein Byte, weil der gehaltene Stand kein `\r\n` mehr trägt, siehe Modulkopf.“ Der Modulkopf, Abschnitt „Nur `\n`, und das ist eine Zusage von anderswo“, begründet das mit dem Einlesen, das `\r\n` zu `\n` macht. Für `datei::trenntext_oeffnen` gilt das seit `ff2d7b76` nicht mehr.
- `crates/krk-ui/src/appkit/editor.rs`, `Editorbereich::schreibmarkenzeile`: der Inhalt der Marke kommt aus `inhalt_der_zeile` über dem gehaltenen Stand.
- `crates/krk-core/src/text/marke.rs`, `wiederfinden`: vergleicht `inhalt_der_zeile(…) == Some(zeileninhalt)` wörtlich. Ein gemerkter Inhalt ohne `\r` trifft keine Zeile mit `\r` und endet in `Fund::NichtGefunden`.
- Stale Doku am selben Grund: `crates/krk-core/src/trenntext/handlung.rs`, Modulkopf, Abschnitt „Das Dateiende gehört dem Text und nicht dem Satz“: „der gehaltene Stand des Editors trägt allein `\n`“, und der Absatz „Was diese Regel nicht hält“ sagt, ein Stand mit einzelnem `\r` am Ende entstehe nur, wenn jemand es schreibt. Seit `ff2d7b76` kommt er auch von der Platte (`a;b\n1;2\r`).

**Richtung der Behebung:** `inhalt_der_zeile` schneidet ein `\r` unmittelbar vor dem `\n` mit ab. Das ist die eine Stelle, die beide Wege trifft, und alte wie neue Marken vergleichen dann gegen dieselbe Form. Die Modulköpfe von `text/zeilen.rs` und `trenntext/handlung.rs` beschreiben den Stand einer CSV, wie er seit dem 261019 ist.

**Abnahme:**

- Eine Probe in `crates/krk-core/src/text/zeilen.rs`: `inhalt_der_zeile("a\r\nb", 1)` ist `Some("a")`.
- Eine Probe in `crates/krk-core/src/text/marke.rs`: `wiederfinden("x\r\nZiel\r\ny", 2, "Ziel")` ergibt `Fund::Getroffen`.
- Die zwei Modulköpfe nennen den Stand einer CSV mit ihren Zeilenenden.
