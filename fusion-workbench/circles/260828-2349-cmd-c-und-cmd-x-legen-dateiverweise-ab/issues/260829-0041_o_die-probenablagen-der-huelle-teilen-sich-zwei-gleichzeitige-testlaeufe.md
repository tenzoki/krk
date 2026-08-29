# Die Probenablagen der Hülle teilen sich zwei gleichzeitige Testläufe

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Executor:** coder

Die Proben in `crates/krk-ui/src/appkit/zwischenablage.rs` legen sich über `pasteboardWithName:` je Probe eine fest benannte Ablage an (`probenablage`, `com.krk.probe.<zweck>`), mit begründetem Verzicht auf `pasteboardWithUniqueName` (kein `releaseGlobally` in `objc2-app-kit 0.3.2`). Der Name ist je Probe eindeutig, aber nicht je **Prozess**: laufen zwei `cargo test`-Prozesse gleichzeitig — in der Runde 22 zweimal beobachtet, während parallel dispatchte Coder `make check` fuhren —, teilen sich beide dieselbe Ablage am Pasteboard-Server, und `der_zweite_ausgang_legt_verweise_und_namen_ab`, `ein_zweites_ablegen_ersetzt_das_erste`, `eine_verknuepfung_wird_als_verknuepfung_abgelegt` bzw. `zwei_dateiverweise_kommen_als_zwei_pfade_zurueck` fallen. In einem einzelnen Lauf sind sie stabil (drei aufeinanderfolgende Läufe auf dem Stand vor dem Commit der Schritte 1–7 grün, 851 Proben).

Kein Defekt im Produktcode. Möglicher Weg: die Prozesskennung in den Namen aufnehmen (`com.krk.probe.<pid>.<zweck>`), um den Preis einer stehenbleibenden Ablage je Lauf — genau der Preis, den der Doc-Kommentar heute vermeidet. Zu entscheiden, wenn parallele Testläufe die Regel werden.

---
Abgleich 260829-0734: bleibt offen. `probenablage` in `zwischenablage.rs` benennt weiter je Probe ohne Prozesskennung; `cargo test --workspace` allein auf `35b95b3` grün (krk-ui 851 passed), was zur Frage paralleler Läufe nichts sagt.
