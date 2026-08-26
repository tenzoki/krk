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
