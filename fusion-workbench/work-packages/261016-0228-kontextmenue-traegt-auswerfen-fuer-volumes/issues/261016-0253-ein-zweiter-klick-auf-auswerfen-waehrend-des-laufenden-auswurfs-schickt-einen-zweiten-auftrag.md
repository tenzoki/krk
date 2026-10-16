Ein zweiter Klick auf „Auswerfen“ während des laufenden Auswurfs schickt einen zweiten Auftrag an das System
---
`auswurf_beginnen` merkt sich keinen laufenden Auswurf. Bis die Datenträgerwache das Aushängen meldet, steht der Einhängepunkt weiter in `AnwendungsIvars::einhaengepunkte`, und ein zweiter Rechtsklick auf denselben Datenträger stellt einen zweiten `unmountVolumeAtURL:options:completionHandler:`.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261016-0253-reviewer-auswerfen-im-kontextmenue.md

**Beleg.** `crates/krk-ui/src/appkit/anwendung.rs`, `auswurf_beginnen` (Commit `bb2f861`): die Menge kommt allein aus `kontextmenue::auswurfbezug(&betroffen.pfade, &self.ivars().einhaengepunkte.borrow())`, und `einhaengepunkte` wird allein in `geraete_uebernehmen` erneuert, also nach einer Meldung der Datenträgerwache. Zwischen Klick und Meldung vergehen bei einer Netzfreigabe Sekunden (Doc-Kommentar von `operationen::auswurf_beginnt`).

**Folge (inference, am Bündel nicht geprüft).** Der zweite Auftrag trifft einen Datenträger, der gerade aushängt oder schon fort ist; das System antwortet mit einem Fehler, und je nach Reihenfolge der zwei Abschlussblöcke steht zuletzt „<name> ließ sich nicht auswerfen: …“ in der Statuszeile, obwohl der Datenträger ausgeworfen ist. `HowTo.md` unter „Einen Datenträger auswerfen“ verspricht „<name> wurde ausgeworfen“.

**Richtung.** Eine Menge laufender Auswürfe in den Ivars, gesetzt in `auswurf_beginnen`, geräumt in `auswurf_einziehen`; ein Einhängepunkt darin wird nicht noch einmal beauftragt, und die Statuszeile sagt „<name> wird ausgeworfen …“ erneut.

**Abnahme.** Eine Probe am Rumpf von `auswurf_beginnen` hält, dass ein laufender Auswurf vor `auswurf::auswerfen(` ausgeschlossen wird; `auswurf_einziehen` räumt den Eintrag auf beiden Zweigen (`Ok` und `Err`).

---
Resolved: `crates/krk-ui/src/appkit/anwendung.rs` fuehrt `AnwendungsIvars::laufende_auswuerfe`; `auswurf_beginnen` fragt die Liste je Einhaengepunkt vor `auswurf::auswerfen(`, meldet einen laufenden mit `operationen::auswurf_laeuft_schon` (neu in `crates/krk-ui/src/kommandos/operationen.rs`, Schluessel `Text::AuswurfLaeuftSchon` in `crates/krk-core/src/sprache/schluessel.rs` und den drei Tabellen `de.rs`, `fr.rs`, `en.rs`) und `continue`, vermerkt sonst und beauftragt; `auswurf_einziehen` raeumt den Vermerk vor `match ergebnis`, also auf `Ok` und `Err`. Probe `ein_laufender_auswurf_wird_nicht_ein_zweites_mal_beauftragt` im Pruefmodul derselben Datei haelt beide Reihenfolgen am Rumpf. Verification: `make check` — exit 0.
