Pfade ohne gültiges UTF-8: vier Hüllen glätten still mit `to_string_lossy`, drei weisen ab

---

Dieselbe Frage — was tut die Hülle mit einem Pfad, der kein gültiges UTF-8 ist — hat unter
`crates/krk-ui/src/appkit/` zwei Antworten. Abweisend, mit Befund: `papierkorb.rs:130-135` (`Err`) und
`:186-188` (`Unentschieden`), `abwurf.rs:225-227` (`Unbekannt`), `volumes.rs:269-271`
(`Unentschieden`), alle drei mit eigener Probe. Still glättend: `terminal.rs:98`,
`standardprogramm.rs:91`, `teilen.rs:293` und `fsevents.rs:290` bauen den `NSString` aus
`to_string_lossy()`, geben also einen Pfad mit `U+FFFD` an das fremde Programm beziehungsweise an
FSEvents weiter, ohne dass ein Rufer es erfährt: das Terminal öffnet einen anderen Ordner, der
Freigabedialog bekommt eine nicht existierende Datei, und die Dateisystemwache beobachtet einen
Ordner, den es nicht gibt (`einrichten` liefert trotzdem `Some`, `:339-343`).

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/appkit/terminal.rs`, `standardprogramm.rs`, `teilen.rs`, `fsevents.rs`

Auf APFS und HFS+ sind Dateinamen ohne gültiges UTF-8 praktisch nicht anzulegen; erreichbar ist
der Fall über fremde Datenträger (FAT/ExFAT mit kaputten Namen, Netzlaufwerke). Der Befund ist
deshalb weniger der Schaden als die Doppelmoral: drei Hüllen begründen im Doc-Kommentar
ausdrücklich, warum sie nicht raten, die vier anderen raten ohne Kommentar. `260812-1529` betrifft
denselben Fall an der Ablagedatei und nicht an diesen Hüllen. Weg: die vier auf `to_str()` mit
`false`/`None` umstellen und den Rufer melden lassen — oder das Glätten an den vier Stellen
begründen, damit die zwei Antworten wenigstens beide eine Begründung tragen.
