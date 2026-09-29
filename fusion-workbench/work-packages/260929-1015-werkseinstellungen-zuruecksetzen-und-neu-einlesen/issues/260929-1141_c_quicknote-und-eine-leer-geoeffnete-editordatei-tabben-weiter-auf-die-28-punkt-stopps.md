Quicknote und eine leer geöffnete Editordatei tabben weiter auf die 28-Punkt-Stopps des Systems
---
`3c246a2` legt den Grundabsatz (Tabschritt vier Leerzeichenbreiten) allein über Text, der beim Aufruf von `textmerkmale::zuruecksetzen` schon im Speicher steht. Zwei Flächen bekommen ihn dadurch nie: die Quicknote, die `zuruecksetzen` gar nicht ruft, und neu getippter Text in einer Editordatei, die beim Öffnen leer war (Rohansicht). Dort erscheint `Wort\tWort` weiter als `WortWort`, der Defekt, den der Commit behebt.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>

## Befund

- `crates/krk-ui/src/appkit/textmerkmale.rs`, `grund_legen`: `addAttributes_range` über `NSRange::new(0, speicher.length())`. Bei Länge 0 wird nichts gesetzt, und die `typingAttributes` der Fläche tragen weiter den Absatzstil des Systems.
- `crates/krk-ui/src/appkit/editor.rs`, `darstellung_nachziehen`: einziger Rufer im Editor. Er läuft beim Öffnen, Ansichtswechsel, Umkehren, Ersetzen, nicht beim Tippen. In der Formatansicht holt die Einfärbung (`anwenden` ruft `zuruecksetzen`) es nach, in der Rohansicht nicht.
- `crates/krk-ui/src/appkit/quicknote.rs` (Aufbau um Zeile 281): `setFont` mit `grundschrift(Ansicht::Roh, …)`, fester Schrift, aber kein Absatzstil und kein Aufruf von `textmerkmale`, der ihn setzt.

**inference:** Dass getippter Text die `typingAttributes` erbt und diese ohne gesetzten Absatzstil den des Systems tragen, ist AppKit-Verhalten und hier nicht am Bündel gemessen. Mitten in vorhandenem Text erbt getippter Text den Absatzstil des Nachbarzeichens; die Lücke betrifft nur die zwei genannten Fälle.

## Richtung

Den Grundabsatz auch als Vorgabe der Fläche setzen (`setDefaultParagraphStyle:` oder die `typingAttributes`), an Editor und Quicknote über dieselbe Funktion `grundabsatz`, statt ihn nur über vorhandenen Text zu legen.

## Abnahme

- Am Bündel: Quicknote öffnen (`f10`), `Wort⇥Wort` tippen; zwischen den Wörtern steht mindestens eine Spalte Zwischenraum, und das zweite Wort beginnt in Spalte 8.
- Am Bündel: eine leere `.txt` im Editor öffnen (Rohansicht), dasselbe tippen; dasselbe Ergebnis.
- Eine Probe hält, dass die Quicknote-Fläche und die Editorfläche den Absatzstil aus `grundabsatz` als Vorgabe tragen.

---
Resolved: fix(textmerkmale) im selben Commit wie dieser Abschluss — `textmerkmale::grund_vorgeben` setzt Schrift, `defaultParagraphStyle` und den Absatzstil der `typingAttributes` aus `grundabsatz`; Editor (Bau und `grundschrift_setzen`) und Quicknote rufen es, gehalten von `die_anschlagsmerkmale_tragen_den_grundabsatz_und_behalten_den_rest` und `beide_bearbeitbaren_flaechen_nehmen_die_vorgabe_von_hier`; die zwei Abnahmen am Buendel stehen aus.
