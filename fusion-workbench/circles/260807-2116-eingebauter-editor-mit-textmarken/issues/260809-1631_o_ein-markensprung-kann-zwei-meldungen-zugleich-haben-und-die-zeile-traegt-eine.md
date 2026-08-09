Ein Markensprung kann zwei Meldungen zugleich haben, und die Zeile trägt eine

---

`krk_core::text::Markensprung` trägt zwei **verschiedene** Auskünfte, und der Modulkopf
von `crates/krk-core/src/text/marke.rs:93-98` sagt ausdrücklich, dass der Aufrufer
beides zu melden hat:

- `Markensprung::fund` — ob der gemerkte Zeileninhalt wiedergefunden wurde.
  `Fund::NichtGefunden` verlangt nach C6, achtes Abnahmekriterium, die Meldung „die
  Stelle hat sich geändert".
- `Markensprung::sprung.lage` — ob die angesteuerte Zeilennummer im Text überhaupt
  vorkommt. `Zeilenlage::HinterDerLetzten` verlangt nach C5 die Meldung, dass die
  Nummer über der Zeilenzahl liegt.

Das Beispiel steht in `marke.rs:96-98`: eine Marke auf Zeile 500 einer inzwischen auf
100 Zeilen gekürzten Datei trägt beide.

**Die Statuszeile trägt einen Text.** Rang 1 hält eine Zeichenkette, und S21 hat die
Meldungen des Editors dort eingereiht, statt eine zweite Fläche daneben zu bauen. Zwei
Meldungen zugleich passen deshalb nicht hinein: eine der beiden fällt weg, und welche,
ist heute nirgends festgelegt.

**Was S21 gebaut hat und was nicht.** `Editormeldung::markenstelle` in
`crates/krk-ui/src/appkit/editor.rs` beantwortet die **erste** Hälfte, also den Fund,
mit einer vollständigen Fallunterscheidung über `Fund`. Die zweite Hälfte hat heute
keinen Auslöser: die Meldung der Zeilenlage gehört zum Zeilensprung aus C5 und kommt
mit S35. Solange nur eine Hälfte gebaut ist, kollidiert nichts, und der Doc-Kommentar
an `markenstelle` benennt die offene Hälfte samt Verweis auf diesen Datensatz.

**Wo es auffällt.** S39 baut den Sprung auf eine Textmarke und ist der erste und
einzige Aufrufer, bei dem beide Auskünfte an einem Wert hängen. Spätestens dort ist zu
entscheiden.

**Vorschlag.** Ein Satz für den zusammengesetzten Fall, kein Vorrang zwischen zwei
Sätzen. Zwei Meldungen, von denen eine gewinnt, wären eine Vorrangregel neben der
bestehenden aus `statuszeile::zeile`, und die trägt fünf Ränge nach dem Alter der
Aussage und nicht nach dem Gewicht eines Grundes. Der zusammengesetzte Fall ist
stattdessen ein eigener Zustand des Sprungs: der gemerkte Inhalt ist fort **und** die
Datei ist kürzer als die gemerkte Nummer. Er ist als Variante von `Editormeldung`
darstellbar und bekommt dann einen Satz, der beides sagt.

Die Alternative wäre, S35 und S39 je eine Meldung bauen zu lassen und die zweite
kommentarlos fallen zu lassen. Das widerspricht dem achten Abnahmekriterium von C6 und
dem von C5, die beide „kommentarlos nichts zu tun" ausschließen.

Gemeldet von: `coder`, bei der Umsetzung von S21.
