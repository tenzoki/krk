Die C4.6-Nadel benennt ihren wahrscheinlichsten blinden Fleck nicht

---

`genau_drei_pruefordner_fassungen_stehen_im_baum` sucht seit Turn 2 nach dem Gegenstand statt
nach dem Namen: `impl Drop for `, `temp_dir()` und `remove_dir_all` in derselben Datei
(`crates/krk-core/tests/baum.rs:136-150`). Das ist die richtige Frage und findet die vierte
Fassung, die dem alten Namensmuster entgangen war; nachgeprüft ist es auch gegen die drei
anerkannten Fassungen, die alle drei Zeichen tragen
(`crates/krk-core/tests/gemeinsam/mod.rs`, `crates/krk-ui/src/pruefordner.rs`,
`crates/krk-bench/src/wegwerfordner.rs`).

**Die genannte Restblindheit trifft den unwahrscheinlichen Fall und nicht den
wahrscheinlichen.** Der Doc-Kommentar nennt zwei (`:109-112`): eine über zwei Dateien
verteilte Fassung, und eine, die ihren Ordner Eintrag für Eintrag abräumt. Nicht genannt ist
der dritte, und es ist der, den dieser Baum schon einmal gebaut hat: **eine Fassung, die ihren
Ordner woanders anlegt als unter `std::env::temp_dir()`.** Der Messplatz unter
`~/Library/Caches/krk-messplatz` ist ein bestehender, benannter Ort in diesem Projekt, und
alle drei anerkannten Fassungen tragen einen eigenen Absatz, der ihn ausdrücklich ausschließt
— was heißt, dass die Frage schon einmal gestellt worden ist. Eine vierte Fassung dort wäre
für die Nadel unsichtbar, obwohl sie in jeder anderen Hinsicht dieselbe Sache ist.

Folgerung 3 aus dem Kopf von `crates/krk-ui/src/quellbaum.rs:69-71` verlangt genau das: die
verbleibende Blindheit benennen. Zwei sind benannt, die dritte fehlt.

---

**Schwere:** gering. Ein Satz, kein Verhalten. Die Nadel ist heute richtig und findet, was es
zu finden gibt.

**Gefunden:** coderev, Durchsicht von `a34bf17..dff167a` am 260813-0720

**Betroffen:** `crates/krk-core/tests/baum.rs:109-112`

**Domain:** code

## Vorschlag

Die dritte Blindheit in denselben Absatz aufnehmen: eine Fassung, die ihren Ordner nicht unter
`std::env::temp_dir()` anlegt, sondern etwa unter dem Messplatz. Wer sie billig mitfangen will,
nimmt `temp_dir()` als **eine von zwei** Ortsnadeln und stellt `krk-messplatz` daneben; das
kostet eine Zeile und deckt den einen anderen Ort, den dieses Projekt kennt.
