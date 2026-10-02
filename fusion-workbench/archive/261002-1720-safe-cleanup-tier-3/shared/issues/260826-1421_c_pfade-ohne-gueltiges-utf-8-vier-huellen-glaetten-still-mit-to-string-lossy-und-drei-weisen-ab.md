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

---
Resolved: Der erste der beiden Wege, also eine Antwort statt zweier: alle vier Huellen weisen einen Pfad ohne gueltiges UTF-8 jetzt ab, statt ihn zu glaetten. `terminal::ordner_oeffnen` und `standardprogramm::oeffnen` liefern `false`, `fsevents::Dateisystemwache::einrichten` liefert `None`, und `teilen::auswaehler_bauen` liefert `Option` — dort wird die **ganze** Menge abgewiesen und nicht der einzelne Eintrag, weil ein Teilen, das stillschweigend weniger Dateien mitnimmt als markiert, der Fehlschlag ist, den C1 ausschliesst; `anbieten` antwortet `false`, `eintrag_anfuegen` haengt keinen Eintrag an. Jede der vier Stellen traegt die Begruendung. Zwei neue Proben nach dem Vorbild von `papierkorb::tests::ein_pfad_ohne_gueltiges_utf8_bleibt_unentschieden` halten die zwei Huellen, die ohne AppKit zu erreichen sind: `terminal::tests::ein_pfad_ohne_gueltiges_utf8_oeffnet_kein_terminal` und `standardprogramm::tests::ein_pfad_ohne_gueltiges_utf8_geht_nicht_an_das_system` — beide fassen AppKit nicht an, weil die Abweisung vor dem `NSWorkspace` steht. Der Modulkopf von `standardprogramm.rs` sagt seither „keine Probe, **die etwas oeffnet**", damit die Zusage stimmt. Fuer `fsevents` und `teilen` steht keine Probe: die erste legte einen Ereignisstrom an, die zweite braucht ein `NSMenu` und einen Hauptfaden.
