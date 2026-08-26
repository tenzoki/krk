`frei_zeigen` sagt, die Vorgabe stehe ausgewählt im Feld, und ruft `selectText:` nicht

---

`namenseingabe::frei_zeigen` (`crates/krk-ui/src/appkit/blaetter/namenseingabe.rs:95-98`)
verspricht: „`vorgabe` steht beim Aufgehen im Feld und ist ausgewaehlt … wer nicht, tippt
darueber." Der Rumpf (`:107-116`) setzt `setStringValue` und sonst nichts. Die zwei
Nachbarblätter, die dieselbe Zusage geben, rufen dafür ausdrücklich `selectText:` —
`pfadeingabe.rs:67` und `suche.rs:119`, beide mit einem Kommentar, warum.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/appkit/blaetter/namenseingabe.rs`

`inference:` Ob AppKit den Inhalt eines `NSTextField` von sich aus auswählt, wenn es über
`setInitialFirstResponder:` den Rang bekommt, ist am Bündel nicht gemessen; die zwei
Nachbardateien verlassen sich nicht darauf. Betroffen sind die zwei Rufer mit Vorgabe:
das Umbenennen eines Lesezeichens (`anwendung.rs:2165-2175`, Vorgabe ist der alte Name) und
das Anlegen (`:2011-2020`). Wer beim Umbenennen tippt, ohne dass ausgewählt ist, hängt seinen
neuen Namen an den alten an.

Denkbarer Weg: dieselbe Zeile wie in `pfadeingabe.rs:67`, oder der Satz im Doc-Kommentar
sagt, dass die Auswahl AppKit überlassen ist.
