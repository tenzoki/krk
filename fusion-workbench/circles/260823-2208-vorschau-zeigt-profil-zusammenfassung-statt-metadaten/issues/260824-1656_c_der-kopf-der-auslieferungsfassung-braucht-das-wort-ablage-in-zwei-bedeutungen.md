Der Kopf der Auslieferungsfassung braucht das Wort „Ablage" in zwei Bedeutungen

---

`resources/default-readers.toml:4` sagt „Die Ablage bindet sie über `include_str!` ein" und meint
KRKs Bestandsort. Fünfzehn Zeilen weiter, in `:19`, heißt es „Die fünf Profile unten beschreiben
die Ablage von fusion", und dort ist die Werkbank gemeint. Daneben sagt `:7`, `readers.toml` sei
„die zweite Datei der Ablage, die von Hand gepflegt wird" — richtig nur unter einer Definition, die
die Datei nicht gibt.

---

**Erstens: zwei Bedeutungen für ein Wort, in einem Kommentarblock.**

„Ablage" ist in diesem Projekt ein festgelegter Begriff: `crates/krk-core/src/ablage/`, der Ort
`~/Library/Application Support/KRK/`, die sieben Dateien darin. Zeile 4 benutzt ihn so. Zeile 19
benutzt dasselbe Wort für die Ablage von Datensätzen in einer fusion-Werkbank, also für etwas
anderes. Der Leser dieser Datei ist der Nutzer und nicht der Entwickler; er hat den Begriff nicht
im Kopf und bekommt ihn hier zweimal verschieden vorgeführt.

**Zweitens: „die zweite Datei der Ablage, die von Hand gepflegt wird".**

Der Satz stimmt unter der Definition aus `crates/krk-core/src/ablage/mod.rs:59-63`: von Hand
gepflegt heißen dort die zwei Dateien, die **nicht** über `Ablage::sichern` gehen, also
`settings.toml` und seit der Runde 16 `readers.toml`. Er stimmt nicht unter der Lesart des
Nutzers: `keymap.toml` ändert der Nutzer ebenfalls von Hand, und der Baum sagt es an zwei Stellen
(`crates/krk-core/src/ablage/mod.rs:797`, „`keymap.toml` und `settings.toml` sind von Hand
aenderbar"; `crates/krk-core/src/ablage/pfade.rs:246`). Für ihn ist `readers.toml` die dritte Datei,
die er von Hand anfasst.

**Warum es hier steht und nicht beim `coderev`.** Die zwei Rust-Stellen sind dessen Bereich; die
drei Kommentarzeilen der Auslieferungsfassung sind es nicht. Der Datensatz beschreibt allein die
Zeilen 4, 7 und 19 von `resources/default-readers.toml`.

**Verwandt, nicht dasselbe:**
`shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`
hält denselben Fehlertyp für die Zahl der Ablagedateien fest und berührt diese drei Zeilen nicht.

**Vorschlag.** In Zeile 19 „die Ablage von fusion" durch „die Werkbank von fusion" ersetzen. In
Zeile 7 die Definition mitliefern, statt sie vorauszusetzen: „sie ist neben `settings.toml` die
zweite Datei, die KRK nach ihrer Anlage nie wieder überschreibt".

Gefunden bei der Durchsicht der Auslieferungsfassung, `reviews/260824-1655-ontorev-…`.
