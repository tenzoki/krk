Drei Baumaussagen des Specs der Runde 22 stimmen mit dem Baum nicht überein
---
Der Spec `planning/260829-0005_*_spec-cmd-c-und-cmd-x-legen-dateiverweise-ab.md` trifft drei Aussagen über den Baum, die der Baum am 260829 nicht deckt. Keine blockiert die Planung; jede würde einen Coder, der den Spec als Baumauskunft liest, an eine falsche Stelle schicken.
---
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>

1. **Die Zählung der Abnehmer von `betroffene()`.** A2 und `## Open for Planner` sagen, die Specs der Runden 4 und 17 zählten bis sechs, und `cmd+c` und `cmd+x` würden der siebte und achte Abnehmer. Der Baum zählt schon bis sieben: `crates/krk-ui/src/appkit/teilen.rs:182` und `crates/krk-ui/src/appkit/anwendung.rs:3791` nennen das Teilen der Runde 6 als siebten Abnehmer. Wie viele es sind, sagt `grep -rn 'betroffene(\|betroffene_eintraege()' crates/krk-ui/src` und keine Ordnungszahl; der Plan dieser Runde vergibt deshalb keine.

2. **C5.5: „`writeObjects:` und `fileURLWithPath:` stehen darin schon."** Der Untergrenzen-Abschnitt von `crates/krk-ui/src/appkit/zwischenablage.rs:141-166` nennt `writeObjects:` (seit 10.6), aber nicht `fileURLWithPath:`; der Aufruf steht heute allein im Prüfmodul (`:384`), das keinen Untergrenzen-Abschnitt hat. Der Plan trägt die Ergänzung im Schritt zur Hülle.

3. **C5.1: „`grep -rn NSPasteboard crates/krk-ui/src` trifft außerhalb der Hülle allein diese zwei Dateien und den Betrachter, dort im Kommentar."** Der Befehl trifft außerdem `crates/krk-ui/src/appkit/mod.rs:111`, `:121`, `:136` und `crates/krk-ui/src/appkit/teilen.rs:20-21`, `:297-300`, alle in Kommentaren. Die Aussage, die C5.1 halten will (keine Codezeile außerhalb der Hülle liest oder schreibt eine Ablage), bleibt wahr; der Wortlaut der Nachzählung ist es nicht.

**Abnahme:** Der Spec nennt an den drei Stellen keine Zahl, die der Baum nicht deckt, oder er zitiert den Befehl, der sie liefert.

---
Abgleich 260829-0734: bleibt offen. Der Spec steht an den drei Stellen unverändert (`planning/260829-0005_*_spec-…`, A2/`## Open for Planner`, C5.5, C5.1). Punkt 2 ist im Baum inzwischen anders gelagert: `zwischenablage.rs:193` nennt `fileURLWithPath:` seit `3764fb6` im Untergrenzen-Abschnitt — die Spec-Aussage „stehen darin schon" ist damit heute wahr, war es am 260829-0006 nicht; das Kriterium hält, der Wortlaut des Specs bleibt eine Aussage über den falschen Tag. Punkte 1 und 3 unverändert.
