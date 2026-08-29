Die Abweisungsmeldung nennt die Einträge, auch wenn allein die Namenszeilen abgewiesen wurden
---
`dateiverweise_auf_ablage_schreiben` liefert `false` in zwei Lagen, die der Rufer nicht trennen kann: `writeObjects:` hat abgewiesen (die Ablage ist leer), oder `writeObjects:` hat angenommen und `setString:forType:` danach abgewiesen (die Ablage trägt die Verweise, keine Namen). Die Statuszeile sagt in beiden `die Zwischenablage hat die Einträge nicht angenommen`; in der zweiten Lage legte ein `cmd+v` im Finder die Einträge ab.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Executor:** coder
**Cross-references:** `reviews/260829-0051-coderev-runde-22-dateiverweise-in-der-zwischenablage.md` (Thema 2); Spec A6, A12, C5.3.

Stellen am `38aa652`: `crates/krk-ui/src/appkit/zwischenablage.rs:375-384` (die zwei Rückgaben), `crates/krk-ui/src/appkit/tabelle.rs:1947-1951` (der eine Rufer, ein `if` über den Wert), `crates/krk-ui/src/kommandos/operationen.rs:1198-1213` (`verweise_abgewiesen`; der Doc-Kommentar sagt „meldet, dass die Ablage nicht stattgefunden hat"). Die zweite Lage ist am Ablageserver nicht beobachtet; der Rückgabewert lässt sie zu, und nichts im Baum schließt sie aus.

Zwei Wege, der erste kleiner: die Hülle ruft bei abgewiesenem Text `clearContents` und liefert dann `false`, so dass „nicht angenommen" wieder stimmt und ein Ablegen ganz oder gar nicht ist; oder sie liefert, was angenommen wurde, und der Rufer meldet es je Lage.

**Abnahme:** Nach `false` aus `dateiverweise_auf_ablage_schreiben` trägt die Ablage keinen Verweis, gehalten von einer Probe auf einer benannten Probenablage, die `writeObjects:` annehmen und den Text abweisen lässt oder den ersten Weg anders belegt; oder die Meldung unterscheidet die zwei Lagen und eine Probe hält beide Sätze.
