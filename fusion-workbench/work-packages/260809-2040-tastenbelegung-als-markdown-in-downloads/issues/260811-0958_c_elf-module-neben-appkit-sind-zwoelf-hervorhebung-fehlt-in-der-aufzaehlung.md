„Elf Module" neben `appkit` sind zwölf: `hervorhebung` fehlt in der Aufzählung

---

`crates/krk-ui/src/main.rs:17` sagt seit `fd863e3`:

> Elf Module liegen ausdruecklich **neben** `appkit` und nicht darin, und keines von ihnen nennt
> eine `objc2`-Kiste.

Die Aufzählung darunter nennt elf: `messmodus`, `fenstermodell`, `tabs`, `vorschaumodell`,
`editormodell`, `leistenmodell`, `kommandos`, `auffrischung`, `belegungsmodell`, `fenstertitel`,
`belegungsausgabe`.

**Es sind zwölf.** `hervorhebung` fehlt, und es erfüllt beide Bedingungen: es liegt neben
`appkit` (`main.rs:48`), und `grep -c objc2 crates/krk-ui/src/hervorhebung.rs` liefert 0.
`CLAUDE.md` führt es selbst als eines der beiden Module, „die ohne AppKit prüfbar sind".

---

**Schwere:** Niedrig
**Gefunden:** coderev, Durchsicht des Codeanteils von Turn 1
**Betroffen:** `crates/krk-ui/src/main.rs:17-40`
**Domain:** code

## Die Lücke ist älter als diese Runde

Vor `fd863e3` stand dort „Zehn Module", und schon damals waren es elf. S3 hat die Zahl
pflichtgemäß um eins erhöht und den neuen Satz für `belegungsausgabe` angehängt — er hat die
bestehende Lücke also nicht verursacht, sondern mitgenommen.

Der Vermerk steht hier trotzdem, weil die Runde genau diese Zeile angefasst hat und `CLAUDE.md`
diese Fehlerform ausdrücklich führt: „eine Zahl davor veraltet, wie es die Aufstellungen getan
haben, die diese Datei aus demselben Grund abgelegt hat." Zwei Kommentarzahlen derselben Art
sind am 260810 als Defekte `260810-1217` und `260810-1218` bereits einmal aufgelaufen.

## Behebung

Zwei Möglichkeiten, und die zweite ist die haltbarere.

**a) Nachzählen.** „Zwölf Module", und ein Satz für `hervorhebung` — es hält, welche Stelle
welche Farbe trägt, welche unterstrichen ist und welche eine Markdown-Auszeichnung ist
(Modulkopf von `hervorhebung.rs`). Kostet eine Zeile und veraltet bei der nächsten Runde wieder.

**b) Die Zahl streichen** und die Aufzählung ohne sie führen, wie `CLAUDE.md` es an seinen
eigenen Aufstellungen aus demselben Grund schon getan hat. Dann kann nur noch der Satz fehlen,
nicht auch die Zahl.

---
Resolved: Weg a) genommen — nachgezaehlt und berichtigt. `crates/krk-ui/src/main.rs:17` sagt
jetzt "Zwoelf Module", und `hervorhebung` steht in der Aufzaehlung. Gezaehlt sind `auffrischung`,
`belegungsausgabe`, `belegungsmodell`, `editormodell`, `fenstermodell`, `fenstertitel`,
`hervorhebung`, `kommandos`, `leistenmodell`, `messmodus`, `tabs`, `vorschaumodell`;
`pruefordner` steht unter `#[cfg(test)]` und zaehlt nicht mit.

**Weg b) dieses Datensatzes bleibt der haltbarere und steht weiter offen:** die Zahl ganz zu
streichen. Eine Zahl in einem Kommentar veraltet mit dem naechsten Modul, und dieses Projekt hat
aus demselben Grund schon mehrere Aufstellungen abgeschafft. Wer das naechste Modul anlegt,
entscheidet es besser gleich mit.

Geschlossen in der Sitzung `history/260811-0107-orchestrator-session.md`, Turn 1.
