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

---

Resolved: Am 260810-0919 wie vorgeschlagen geschlossen, in Prosa und ohne eine
Zeile Code.

Der Modulkopf von `krk_core::text::datei` nennt beide Eingänge, den Stand aus
der `NSTextView` zuerst, als Aufzählung mit zwei Punkten: der größere mit dem
Grund, aus dem er der größere ist (eine `NSTextView` bewahrt eingefügten Text
zeichengetreu auf), der kleinere unverändert mit dem Eingabefeld und dem
kopierten `\r`. Beide sind ausdrücklich als **gebaut** bezeichnet, nicht als
anstehend, und der Absatz nennt ihre Fundstellen (`bearbeiten` und
`ersetzung_vorbereiten` in `krk-ui/src/editormodell.rs`).

Der Pfeil im ASCII-Bild trägt jetzt "der Stand aus der Textflaeche und der
Ersatztext aus C5" statt "jeder andere Text, der in den Stand geraet (S37)".

**Ein Punkt geht über den Vorschlag hinaus.** Der Modulkopf von
`editormodell.rs` führt beide Eingänge schon vollständig aus, mit eigenem Bild
und mit der Begründung, warum der größere ein `bool` zurückgibt. Der Absatz in
`datei.rs` verweist deshalb dorthin, statt die Erklärung ein zweites Mal zu
schreiben: zwei Fassungen derselben Erklärung wären genau die Sorte Doppelung,
die dieses Modul sonst vermeidet.

Geändert: ausschließlich `crates/krk-core/src/text/datei.rs`, darin
ausschließlich der Modulkopf. Abgenommen mit `cargo build/test/clippy/fmt
--workspace`, alle vier auf 0.
