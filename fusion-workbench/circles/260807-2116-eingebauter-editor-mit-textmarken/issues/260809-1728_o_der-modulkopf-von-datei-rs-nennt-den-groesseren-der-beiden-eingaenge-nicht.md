# Der Modulkopf von `datei.rs` nennt den größeren der beiden Eingänge nicht

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev (Durchsicht Turn 2), abgetrennt beim Schließen von `260809-1646` durch coder
**Betroffen:** `crates/krk-core/src/text/datei.rs:39-45` (Modulkopf, Aufzählung der anstehenden Fälle)
**Cross-references:** `issues/260809-1646_c_die-zusage-ueber-den-gehaltenen-stand-hat-einen-zweiten-eingang-ohne-normalisierung.md`, `crates/krk-ui/src/editormodell.rs` (`bearbeiten`, `ersetzung_vorbereiten`), S9, S37

---

## Der Befund

Der Modulkopf von `krk_core::text::datei` sagt, wer `in_gehaltene_form` zu rufen
hat, und zählt dazu genau **einen** anstehenden Fall auf:

> Der Fall, der ansteht, ist der Ersatztext des Suchen-und-Ersetzens aus C5
> (Schritt 37): er kommt aus einem Eingabefeld und kann ein `\r` tragen, wenn er
> hineinkopiert wurde.

Der größere Fall fehlt: **der Stand, den die `NSTextView` zurückgibt.** Eine
`NSTextView` bewahrt eingefügten Text zeichengetreu auf, also bringt ein
Einfügen aus einer Windows-Quelle `\r\n` mit.

Beide Eingänge sind seit dem Schließen von `260809-1646` gebaut und stehen in
`krk-ui/src/editormodell.rs`: `bearbeiten` für den ganzen Stand,
`ersetzung_vorbereiten` für den Ersatztext. Die Aufzählung im Modulkopf hinkt
ihnen hinterher und liest sich so, als stünde der eine noch aus und den anderen
gäbe es nicht.

Auch das ASCII-Bild darüber (`datei.rs:11-13`) beschriftet den Pfeil in
`in_gehaltene_form` allein mit "jeder andere Text, der in den Stand gerät (S37)".

## Warum das kleiner ist, als es klingt

Es ist eine Prosa-Berichtigung und kein Rechenfehler: die Zusage hält, weil
beide Eingänge gebaut sind. Der Preis des Nichtstuns ist, dass der nächste, der
den Modulkopf liest, den größeren Eingang für nicht vorhanden hält.

## Warum es nicht mit `260809-1646` erledigt wurde

`crates/krk-core/` war für einen parallel laufenden Schritt reserviert und
ausdrücklich nicht anzufassen. Die Berichtigung ist eine Zeile Prosa in
`datei.rs` und braucht keinen Code.

## Vorschlag

Im Modulkopf beide Fälle nennen, den Stand aus der Textfläche zuerst, und im
Bild den Pfeil entsprechend beschriften.
