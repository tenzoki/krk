Drei Zählungen in den Modulköpfen von `appkit/` sind veraltet: neun Ankreuzfelder, fünf Module, und ein Überblick ohne zwei Module

---

1. `crates/krk-ui/src/appkit/mod.rs:79-82`: „neun Ankreuzfelder, fuenf fuer die Bereiche, drei fuer
   die schaltbaren Spalten und einer fuer die tiefe Suche". `bereichsleiste.rs:1` sagt „zehn
   Ankreuzfelder", `:456-528` baut zehn, und die Probe `die_leiste_traegt_zehn_schalter` (`:729-736`)
   hält zehn. Der Schalter „Content" der Runde 11 fehlt im Kopf von `mod.rs`.
2. `bildtakt.rs:3`: „Geschnitten wie die fuenf uebrigen Module dieses Verzeichnisses". `mod.rs:10`
   zählt dreißig, `mod.rs:188-217` führt dreißig `mod`-Zeilen.
3. `mod.rs:16-37` (der Überblick der Wege): `abwurf` und `weitereinstanz` stehen nicht darin,
   obwohl beide `use crate::`-Ziele beziehungsweise AppKit-Berührungen tragen und `mod.rs:111-118`
   `abwurf` im Fließtext ausdrücklich beschreibt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/appkit/mod.rs`, `crates/krk-ui/src/appkit/bildtakt.rs`

Dieselbe Sorte Befund wie `260812-1702` und `260812-1731` an derselben Datei (dort: textmerkmale
und Statuszeile) — der Kopf von `mod.rs` läuft der Runde hinterher, die ein Modul oder einen
Schalter dazugibt, und keine Probe zählt Prosa. `CLAUDE.md` hat für genau diese Zahlen die Regel
„Zahl in Prosa wird mit der nächsten Runde falsch". Weg: „zehn" schreiben oder die Zahl
weglassen; in `bildtakt.rs` „die uebrigen Module" ohne Zahl; die zwei Module in den Überblick
aufnehmen oder den Überblick als unvollständig kennzeichnen, wie `mod.rs:171` es für die
`use crate::`-Liste schon tut („ohne den Anspruch, alle zu sein").


---
Resolved: Punkt 2 und 3 sind behoben, Punkt 1 bestand nicht mehr. **Punkt 1:** `crates/krk-ui/src/appkit/mod.rs` sagt zu den Ankreuzfeldern seit einer frueheren Runde schon „**Eine Zahl steht hier nicht**, weil sie mit jedem Bereich und jeder Spalte waechst; gezaehlt wird sie an `Bereichsleiste::bauen`". Nichts geaendert. **Punkt 2:** `crates/krk-ui/src/appkit/bildtakt.rs` sagt jetzt „wie die uebrigen Module dieses Verzeichnisses" ohne Zahl und verweist auf die `mod`-Zeilen am Fuss von `mod.rs`. **Punkt 3:** der Ueberblick ist als das gekennzeichnet, was er ist — eine Zeichnung der Wertefluesse und kein Modulverzeichnis —, und der Kopf nennt die fuenf Module, die nicht darin stehen (`abwurf`, `weitereinstanz`, `git`, `leiste`, `koordinaten`); es waren nicht zwei, sondern fuenf. Zwei davon fehlten auch im Fliesstext dahinter und haben dort jetzt einen Absatz: `git` und `weitereinstanz`. Die Zahl „Zweiunddreissig Module" im Kopf ist aus demselben Grund gefallen wie die in `bildtakt.rs`.
