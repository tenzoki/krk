Die Nummernspalte kopiert bei jeder Textänderung den ganzen Text und baut den Zeilenindex von vorn

---

`crates/krk-ui/src/appkit/nummernspalte.rs:314-321` (`index_erneuern`) ruft `ivars.flaeche.string().to_string()`,
also eine vollständige UTF-16→UTF-8-Kopie des Flächentextes, dann `Zeilenindex::neu(&text)` und
`anfaenge_in_utf16` (`:509-514`) über den ganzen Text. Angestoßen wird das von
`textGeaendert:` (`:196-200`) bei **jeder** `NSTextStorageDidProcessEditingNotification`, also je
Anschlag im Editor, und ausgeführt beim nächsten Zeichnen (`:366-368`). Bei einer Datei nahe
`EDITORGRENZE` (16 MB) sind das je Anschlag drei lineare Durchläufe über 16 MB, im Zeichenpfad.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Mittel
**Betroffen:** `crates/krk-ui/src/appkit/nummernspalte.rs`, beide Flächen (Editor und Vorschau; in der Vorschau ändert sich der Text nur beim Inhaltswechsel, dort ist es je Wechsel einmal)

`CLAUDE.md` hebt hervor, dass `hervorhebung.rs` den vorigen Durchgang **fortschreibt statt ihn zu
wiederholen** (`3596e16`), damit das Tippen in großen Dateien nicht linear in der Dateigröße
kostet. Die Nummernspalte daneben tut genau das Wiederholen, und zwar synchron auf dem
Hauptfaden im Zeichenpfad. Gemessen ist das nicht; die Zusage „Syntaxhervorhebung aus C3 auf dem
Referenzgerät ungemessen" (`CLAUDE.md`, Maximen) hat hier einen zweiten, verwandten Posten, den
keine Messreihe führt. Der Modulkopf (`:63-79`) sagt „je gezeichnetem Bild höchstens ein
Neuaufbau und nicht je Anschlag" — das begrenzt die Zahl der Neuaufbauten, nicht ihre Kosten.

Denkbare Wege: (1) den `Zeilenindex` aus dem `Editormodell` beziehen statt aus dem Flächentext
(das Modell hält den Stand ohnehin und `krk_core::text::zeilen` könnte fortgeschrieben werden);
(2) über `NSTextStorageDidProcessEditingNotification` den `editedRange` lesen und den Index nur ab
dort neu rechnen; (3) erst messen, ob der Posten auf dem Referenzgerät bei 16 MB überhaupt
spürbar ist, und den Befund neben die C3-Messung stellen.
