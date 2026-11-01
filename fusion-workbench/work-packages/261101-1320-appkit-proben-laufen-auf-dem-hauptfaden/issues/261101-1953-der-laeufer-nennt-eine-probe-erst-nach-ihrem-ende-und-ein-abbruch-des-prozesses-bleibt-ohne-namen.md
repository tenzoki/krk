Der Läufer nennt eine Probe erst nach ihrem Ende, und ein Abbruch des Prozesses bleibt ohne Namen
---
`appkit::hauptfaden::laufen` schreibt `test <name> ...` erst, wenn `catch_unwind` zurückkehrt. Bricht eine Probe den Prozess ab, steht ihr Name nirgends in der Ausgabe. Nur das Hängen über die Frist wird benannt.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261101-1436-plan-appkit-proben-laufen-auf-dem-hauptfaden.md, 261101-1953-reviewer-appkit-proben-hauptfaden.md

**Beleg:** `crates/krk-ui/src/appkit/hauptfaden.rs:330-357` (Stand `0067f466`). Der Name geht vor der Probe allein an den Wachfaden (`Wachruf::Beginn`). `println!("test {name} ... ok")` und `... FAILED` folgen erst danach. Mit `catch_unwind` kehrt eine Rust-Panik zurück, aber kein Abbruch des Prozesses. Abbrüche dieser Art kommen vor: eine Ausnahme von Objective-C („Rust cannot catch foreign exceptions“, `SIGABRT`), `SIGSEGV` an einer `NSTextView` (`appkit/editor.rs:2713` und `:9358` nennen beide) und eine Panik in einem Rückruf von AppKit über eine `extern "C"`-Grenze. In der Ausgabe steht dann als letzte Zeile die vorige Probe mit `ok`. Vor diesem Fehlbild warnte der frühere Kommentar an `ein_blatt_ohne_ungefaehrlichen_ausgang_fliegt_auf` („Wer ihn sieht, sucht hier und nicht bei der Probe, die zuletzt gemeldet hat“), entfernt in `b8966f9c`.

**Abnahme:**
- Vor jeder gefahrenen Probe steht ihr Name in der Ausgabe und ist geflusht, bevor `probe.lauf` gerufen wird. Ein Weg ist `test <name> ... ` ohne Zeilenende mit `flush`, danach `ok` oder `FAILED`, wie `libtest` es bei einem Prüffaden tut (`inference:`, nicht nachgelesen). Ein zweiter Weg ist eine eigene Zeile auf der Standardfehlerausgabe.
- Die Ausgabe eines grünen Laufs bleibt in der Form von `libtest`, und `cargo test -p krk-ui --test hauptfaden` ist grün.
