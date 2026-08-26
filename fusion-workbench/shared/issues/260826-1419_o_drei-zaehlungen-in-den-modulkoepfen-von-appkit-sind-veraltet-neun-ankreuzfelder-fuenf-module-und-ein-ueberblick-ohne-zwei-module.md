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
