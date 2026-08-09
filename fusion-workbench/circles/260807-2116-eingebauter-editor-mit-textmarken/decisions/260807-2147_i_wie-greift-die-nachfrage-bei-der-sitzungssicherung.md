# Wie greift die Nachfrage nach ungespeicherten Änderungen bei der Sitzungssicherung?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md` §"3. Ungespeicherte Änderungen", `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` (C4), `crates/krk-core/src/ablage/sitzung.rs:33` (`SITZUNGSTAKT`)

---

## Question

Der Nutzer hat am 260807-2139 festgelegt, dass die Nachfrage nach ungespeicherten Änderungen an drei Anlässen greift: beim Schließen des Editors, beim Beenden der Anwendung und bei der Sitzungssicherung in `session.toml`. Für die ersten beiden ist die Festlegung unmittelbar umsetzbar, denn beide sind Handlungen des Nutzers, und eine Handlung kann auf eine Antwort warten.

Der dritte Anlass ist von anderer Art, und das ist der Grund für diese Frage. Die Sitzungssicherung ist kein Befehl des Nutzers, sondern ein Schreibvorgang im Hintergrund: `Sitzungsschreiber` bündelt ihn auf höchstens einen Schreibvorgang je zwei Sekunden (`SITZUNGSTAKT` in `crates/krk-core/src/ablage/sitzung.rs:33`) und schreibt daneben einmal beim Beenden. Eine Rückfrage alle zwei Sekunden ist offensichtlich nicht gemeint. Was gemeint ist, muss der Nutzer sagen, denn die drei denkbaren Lesarten führen zu drei verschiedenen Anwendungen.

Der Verlust, um den es geht, ist real und nicht theoretisch. Trägt `session.toml` die geöffnete Datei des Editors, dann öffnet der nächste Start sie aus dem Dateisystem, und alles, was der Nutzer vor einem Absturz oder einem erzwungenen Beenden nicht gesichert hatte, ist weg, ohne dass ihn jemand gefragt hätte.

## Options

1. **Der dritte Anlass fällt mit dem zweiten zusammen** — die Sitzung wird beim Beenden ein letztes Mal geschrieben, und dort steht die Nachfrage schon. Die getakteten Zwischenschreibvorgänge fragen nichts und tragen den ungesicherten Stand auch nicht mit; sie halten allein fest, welche Datei offen ist.
   - Pro: keine neue Mechanik. Die Zusage des Nutzers ist an dem einen Punkt eingelöst, an dem sie einzulösen ist.
   - Contra: bei einem Absturz oder einem `kill` ist der ungesicherte Stand verloren, und niemand hat gefragt. Genau der Fall, in dem eine Sicherung nützt, bleibt ungedeckt.

2. **Der Editor sichert seinen ungespeicherten Stand mit der Sitzung** — neben dem Pfad der geöffneten Datei wandert der Pufferinhalt in die Ablage, und der nächste Start stellt ihn wieder her, samt dem Hinweis, dass er von der Datei auf der Platte abweicht. Gefragt wird nichts, weil nichts verloren geht.
   - Pro: deckt den Absturz mit ab. Der Nutzer verliert keine Arbeit, gleich wie die Anwendung endet.
   - Contra: `session.toml` ist nach C7 der Runde 1 zum Lesen und Ändern von Hand gedacht, und ein eingebetteter Dateiinhalt macht sie unlesbar. Eine fünfte Ablagedatei daneben wäre die Alternative und erweitert `Datei::ALLE` in `crates/krk-core/src/ablage/pfade.rs`. Dazu entsteht ein zweiter Wahrheitsstand über den Dateiinhalt, mit der Frage, was gilt, wenn die Datei sich außerhalb von KRK geändert hat.

3. **Der Editor sichert von selbst, bevor die Sitzung geschrieben wird** — ein anstehender Sitzungsschreibvorgang schreibt zuerst den Pufferinhalt in die Datei. Es gibt dann nie einen ungesicherten Stand, der eine Nachfrage bräuchte.
   - Pro: einfachste Regel von allen. Kein zweiter Speicher, keine Rückfrage im Hintergrund.
   - Contra: nimmt dem Nutzer das Verwerfen. Wer eine Datei versehentlich verändert und die Änderung wegwerfen will, findet sie zwei Sekunden später auf der Platte. Das widerspricht der Festlegung des Nutzers, die "sichern, verwerfen, abbrechen" ausdrücklich nennt.

## Constraints

- Die Festlegung des Nutzers vom 260807-2139 nennt drei Wahlmöglichkeiten der Nachfrage: sichern, verwerfen, abbrechen. Eine Antwort, die das Verwerfen unmöglich macht, verfehlt sie.
- Die Nachfrage beim Schließen des Editors und beim Beenden der Anwendung ist von dieser Frage nicht berührt; sie steht in beiden Fällen und wird im Spec als Abnahmekriterium von C4 geführt.
- Beim Beenden über `terminate:` steht heute kein `applicationShouldTerminate:` im Weg. `crates/krk-ui/src/appkit/anwendung.rs:1162` hält ausdrücklich fest, dass es keines gibt und die Aufrufer nicht mit einer Rückkehr rechnen. Jede Antwort, die beim Beenden fragt, ändert das.
- `session.toml` bleibt nach C7 der Runde 1 von Hand lesbar und änderbar.

## Recommendation

Wir empfehlen Möglichkeit 1 als Zuschnitt dieser Runde und Möglichkeit 2 als eigenes, späteres Vorhaben. Die Trennung folgt daraus, dass die beiden verschiedene Fragen beantworten: Möglichkeit 1 löst die Zusage des Nutzers an dem Punkt ein, an dem ein Mensch antwortet, während Möglichkeit 2 eine Absturzsicherung ist und damit eine Fähigkeit für sich, mit eigener Ablage, eigener Wiederherstellung und eigener Regel für die von außen geänderte Datei.

Möglichkeit 3 empfehlen wir nicht. Sie ist die einfachste und zugleich die einzige, die der Festlegung des Nutzers widerspricht.

Die Empfehlung ist eine Auslegung und keine geprüfte Aussage über die Absicht des Nutzers. Sie ist ihm deshalb vorzulegen, bevor der Planner den Zuschnitt festschreibt.

---
Answered: circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md §"9. Nachfrage bei der Sitzungssicherung" — Möglichkeit 1 gewählt: der dritte Anlass fällt mit dem zweiten zusammen. Die Sitzung wird beim Beenden ein letztes Mal geschrieben, und dort steht die Nachfrage ohnehin; die getakteten Zwischenschreibvorgänge fragen nichts und tragen den ungesicherten Stand nicht mit. Der Preis ist angenommen: bei einem Absturz ist der ungesicherte Stand verloren, ohne Nachfrage. Eine Absturzsicherung, die den Pufferinhalt mitsichert (Möglichkeit 2), ist ein eigenes späteres Vorhaben. Zu beachten: anwendung.rs:1162 hält fest, dass heute kein applicationShouldTerminate: im Weg steht; die Nachfrage beim Beenden ändert das. Entschieden vom Nutzer am 260808-0017.

Implemented: `crates/krk-core/src/ablage/sitzung.rs` (`Sitzung::editor`) und `crates/krk-ui/src/appkit/anwendung.rs` (`sitzung_bauen`, `editor_wiederherstellen`, `applicationShouldTerminate:`) — Möglichkeit 1 steht in beiden Hälften. Die Sitzung trägt allein den **Pfad** der geöffneten Datei und weder den bearbeiteten Stand noch die Abweichungsmarke; die getakteten Zwischenschreibvorgänge fragen deshalb nichts und halten die Anwendung nicht an. Die Nachfrage beim Beenden steht seit S29 in `applicationShouldTerminate:`, und `applicationWillTerminate:` schreibt den letzten Stand danach, nicht davor. Der angenommene Preis gilt unverändert: bei einem Absturz ist der ungesicherte Stand verloren. Gebaut mit S29 am 260810-0021 und S30 am 260810-0146.
