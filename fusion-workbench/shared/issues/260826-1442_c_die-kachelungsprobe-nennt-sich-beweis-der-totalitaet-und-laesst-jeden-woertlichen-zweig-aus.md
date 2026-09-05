Die Kachelungsprobe nennt sich „Beweis der Totalität“ und lässt jeden wörtlichen Zweig aus
---
`die_kachelung_deckt_quelle_und_text_lueckenlos` sagt im Doc-Kommentar, sie „fängt jeden Ereignisfall, der Quelltext abträgt, ohne einen Abschnitt anzulegen“. Ihre zehn Beispiele enthalten aber kein Bild, kein HTML, keine Trennlinie (der Zweig `Behandlung::Woertlich` samt `bis_zum_ende_ueberspringen`), keinen leeren Listenpunkt (`nur_das_merkzeichen`), keinen Zitatblock mit Leerzeile und keinen harten Umbruch. Genau diese Wege tragen die Quelle auf anderen Pfaden ab als die zehn.
---
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>

## Am Baum

- `crates/krk-ui/src/markdown.rs:2473-2484`: `KACHELBEISPIELE`, zehn Quellen.
- `crates/krk-ui/src/markdown.rs:2547-2557`: der Anspruch „Beweis der Totalität und keine Aufzählung von Fällen“.
- Ungedeckte Abtragswege: `:641-644` (`woertlich` + `bis_zum_ende_ueberspringen`, Bild/HTML-Block), `:666-672` (`Event::Rule`, `InlineHtml`), `:1493-1501` (`nur_das_merkzeichen` → `absetzen` + `merkzeichen_einloesen`), `:663` (`HardBreak`, Ersetzt-Abschnitt über `"  \n"` bzw. `"\\\n"`).

Ich habe die vier Wege am Code nachvollzogen: `woertlich` läuft über `schreiben` → `kacheln`, und die übersprungenen Ereignisse bewegen `gelesen` nicht; der leere Punkt kachelt über `gelesen_bis(3)` und `erzeugen`. Beide halten die zwei Zusagen heute. Gemessen ist das von keiner Probe, und der Anspruch der Probe sagt das Gegenteil.

## Vorschlag

Die Beispiele um `"Davor ![Alt](bild.png) danach.\n"`, `"<div>\nx\n</div>\n"`, `"---\n"`, `"- \n"`, `"> eins\n>\n> zwei\n"` und `"a  \nb\n"` erweitern, oder den Satz im Doc-Kommentar auf „an zehn Beispielen“ zurücknehmen.

---
Resolved: Beide Hälften des Vorschlags, und nicht die eine oder die andere.

`KACHELBEISPIELE` in `crates/krk-ui/src/markdown.rs` ist von zehn auf sechzehn
Einträge gewachsen; die sechs neuen sind genau die aus dem Vorschlag:
`"Davor ![Alt](bild.png) danach.\n"`, `"<div>\nx\n</div>\n"`, `"---\n"`,
`"- \n"`, `"> eins\n>\n> zwei\n"` und `"a  \nb\n"`. Sie decken
`Behandlung::Woertlich` mit `bis_zum_ende_ueberspringen`, `Event::Rule`,
`Event::InlineHtml`, `nur_das_merkzeichen` und `Event::HardBreak`. Alle sechs
laufen grün: die zwei Zusagen der Kachelung halten auf diesen Wegen, und das
ist jetzt gemessen statt am Code nachvollzogen.

Der Anspruch im Doc-Kommentar von `die_kachelung_deckt_quelle_und_text_lueckenlos`
ist trotzdem zurückgenommen. "Beweis der Totalität" war nicht einzulösen: ein
Durchgang über eine Beispielliste fängt keinen Ereignisfall, den kein Beispiel
auslöst — sechs zu übersehen war der Beleg. Der Kommentar sagt jetzt, dass der
Bau die Fallunterscheidung in `kacheln` hält und die Probe hält, dass die
genannten Wege ihre zwei Zusagen einhalten.

Die Zahl "Zehn Fälle" am Doc-Kommentar von `KACHELBEISPIELE` ist gefallen und
nicht durch "Sechzehn" ersetzt: der Satz verweist jetzt auf die Längenangabe des
Feldes, die der Übersetzer hält.

Beleg: `cargo test -p krk-ui markdown` → 66 bestanden.
