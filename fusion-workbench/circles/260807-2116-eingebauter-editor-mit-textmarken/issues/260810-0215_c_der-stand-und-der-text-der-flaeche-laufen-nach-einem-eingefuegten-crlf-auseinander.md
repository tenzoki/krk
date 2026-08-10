# Der Stand und der Text der Fläche laufen nach einem eingefügten CRLF auseinander

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coder, bei der Umsetzung von S35, S36 und S37
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (`schreibmarke_in_utf16`, `stelle_zeigen`, `schreibmarkenzeile`, `text_zurueckschreiben`), `crates/krk-ui/src/editormodell.rs` (`bearbeiten`)
**Cross-references:** `crates/krk-core/src/text/datei.rs` (`in_gehaltene_form`), `crates/krk-ui/src/appkit/koordinaten.rs`, `issues/260809-1646_c_die-zusage-ueber-den-gehaltenen-stand-hat-einen-zweiten-eingang-ohne-normalisierung.md`, S9, S26, S35, S36, S37

---

## Der Befund

Der Editor führt zwei Zeichenketten, die Zeichen für Zeichen dieselben sein
sollen: den gehaltenen Stand in `Editormodell` und den Text des `NSTextStorage`
der Textfläche. Drei Wege halten sie zusammen — `stand_einsetzen` schreibt vom
Modell in die Fläche, `text_zurueckschreiben` liest von der Fläche ins Modell,
und der Ansichtswechsel aus C3 fasst den Textspeicher gar nicht an.

**Ein Weg trennt sie.** `Editormodell::bearbeiten` führt den Stand aus der
Fläche durch `krk_core::text::datei::in_gehaltene_form`, seit der Behebung von
`260809-1646`. Das ist richtig und muss so bleiben: ohne diese Wandlung landete
ein eingefügtes `\r\n` beim Sichern auf der Platte, und die Zusage aus dem
Modulkopf von `datei.rs` wäre gebrochen.

Die Wandlung schreibt aber nicht zurück. Wer Text aus einem Windows-Projekt in
den Editor einfügt, hat danach:

```
NSTextStorage:  … \r \n …        zwei Zeichen
Editormodell:   … \n …           ein Zeichen
```

Von der eingefügten Stelle an ist jede weitere Stelle um die Zahl der `\r`
gegeneinander verschoben.

## Warum das erst jetzt zählt

Bis S35 hat niemand eine Stelle der einen Zeichenkette in die andere übersetzt.
Seit S35, S36 und S37 tun es vier Funktionen in `appkit/editor.rs`:

- `suche_beginnen` rechnet die Schreibmarke der Fläche in einen Byteversatz des
  Standes um und sucht ab dort.
- `stelle_zeigen` rechnet umgekehrt und wählt einen Treffer in der Fläche aus.
- `zeile_anspringen` desgleichen für den Zeilenanfang.
- `schreibmarkenzeile` liest die Zeile der Schreibmarke aus dem Stand.

Alle vier gehen durch `appkit/koordinaten.rs`, und die Umrechnung selbst ist
richtig; sie rechnet nur gegen den falschen Text, sobald die beiden
auseinandergelaufen sind. Sichtbar wird es als eine Suche, die einen Treffer
findet und die Auswahl daneben setzt, oder als ein Zeilensprung, der eine Zeile
zu früh landet.

**Der Fall heilt sich beim nächsten Ersetzen von selbst**, weil
`Editorbereich::treffer_ersetzen` den gewandelten Stand über `stand_erneuern`
zurück in die Fläche schreibt. Bis dahin bleibt er stehen.

## Was zu entscheiden ist

Der naheliegende Griff ist, in `text_zurueckschreiben` nach einer Wandlung die
Fläche neu zu beschreiben. Er ist nicht umsonst: `setString:` setzt die
Schreibmarke an den Anfang und geht am Rückgängigstapel vorbei, und beides träfe
den Nutzer genau in dem Augenblick, in dem er eben eingefügt hat. Ihn dabei
richtig stehen zu lassen hieße, die neue Stelle der Schreibmarke aus der Zahl
der gewandelten `\r` vor ihr zu rechnen.

Die Alternative ist, den `\r` schon am Eingang der Fläche abzufangen, also über
`textView:shouldChangeTextInRanges:replacementStrings:`. Dann käme er gar nicht
erst in den Textspeicher, und die beiden blieben ohne Nachbehandlung gleich.
Das ist der Ort, den AppKit für diese Frage vorsieht, und die Fläche hat heute
schon einen Delegierten.

Ungemessen ist beides. Der Fall verlangt eine Entscheidung und nicht den
nächstbesten Griff, deshalb steht er hier und nicht als Behebung im Schritt.

---
Resolved: Die Fläche wird nachgezogen, statt den `\r` am Eingang abzufangen.

`Editormodell::bearbeiten` liefert seit 260810-0309 ein `bool` und sagt damit,
ob die Wandlung zugegriffen hat; die Auskunft kommt aus
`krk_core::text::datei::ist_in_gehaltener_form`, derselben Bedingung, an der
`in_gehaltene_form` ihren kurzen Weg nimmt. Meldet sie sich,
richtet `Editorbereich::flaeche_richten` die Textfläche auf den gehaltenen
Stand und rechnet die Schreibmarke mit; wohin sie wandert, sagt
`krk_core::text::datei::versatz_nach_der_wandlung`.

**Die Wahl, die der Datensatz verlangt hat**, ist gegen den Eingangsfilter
ausgefallen. `textView:shouldChangeTextInRanges:replacementStrings:` müsste die
Regeln der Wandlung ein zweites Mal tragen, und es wären **nicht dieselben**:
die Bytefolgenmarke fällt nach ihrer Stelle im ganzen Text, ein eingefügtes
Stück kennt seine Stelle aber nur beim Einfügen. Ein Löschen, das eine Marke aus
der Mitte an den Anfang rückt, ginge an einem solchen Filter vorbei und brächte
die beiden erneut auseinander. Die Behebung vergleicht deshalb das Ergebnis
statt die Eingabe und kommt ohne eine einzige Regel der Wandlung aus.

**Der Preis ist der benannte und steht im Doc-Kommentar von
`flaeche_richten`:** der Weg führt über `stand_erneuern` und damit über
`setString:`, das am Rückgängigstapel vorbeischreibt. Ein `cmd+z` unmittelbar
nach einem eingefügten `\r\n` wirkt gegen einen Stand, den die Fläche nicht mehr
trägt. Es ist derselbe Preis, den das Ersetzen aus S37 schon zahlt, und
`260809-1727` führt ihn; ein zweiter Schreibweg in die Fläche entsteht dafür
nicht. Die Schreibmarke bleibt dagegen stehen, wo sie stand.

Geändert: `crates/krk-core/src/text/datei.rs`,
`crates/krk-ui/src/editormodell.rs`, `crates/krk-ui/src/appkit/editor.rs`.
Proben: `eine_stelle_wandert_mit_der_wandlung_in_die_gehaltene_form` und
`die_frage_nach_der_gehaltenen_form_und_die_wandlung_sagen_dasselbe` in
`crates/krk-core/tests/text.rs`,
`ein_eingefuegtes_crlf_meldet_sich_und_ein_gewoehnlicher_anschlag_nicht` in
`editormodell.rs`,
`nach_einem_eingefuegten_crlf_zeigt_dieselbe_stelle_in_beiden_texten_auf_dasselbe`
in `appkit/editor.rs`. `make check` läuft mit Rückgabewert 0 durch.
