# Der Dateikopf der Belegung behauptet den Tastencode als allgemeinen Nachschlagweg

---
**Domain:** data
**Schwere:** Low
**Gefunden von:** ontocoder, beim Beheben von `260810-0011_*_zwei-kommentarbloecke-der-belegungsdatei-behaupten-den-nachschlag-ueber-den-tastencode.md`
**Betroffen:** `resources/default-keymap.toml`, Zeile 42
**Cross-references:** `crates/krk-core/src/tasten/parser.rs` (Modulkopf, `Taste::kennung`), `issues/260810-0011_*_zwei-kommentarbloecke-der-belegungsdatei-behaupten-den-nachschlag-ueber-den-tastencode.md` (dieselbe weggefallene Begründung, zwei andere Blöcke derselben Datei), `decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`

---

## Der Befund

Der Kopf von `resources/default-keymap.toml` begründet in seinem Absatz über die
fn-Taste (Zeilen 41–47), warum sie in keiner Kombination vorkommt, und stützt
das auf einen Satz über den Nachschlag:

Zeile 42: "KRK belegt den Tastencode, und F3 mit gehaltener fn-Taste erzeugt
denselben Tastencode wie ein nacktes F3."

Der erste Halbsatz ist seit S2 (`00719cb`) als allgemeine Aussage falsch.
`Taste::kennung` (`crates/krk-core/src/tasten/parser.rs:192-198`) legt jeden
einbuchstabigen Namen auf `Tastenkennung::Zeichen`; über den Tastencode gehen
nur noch Funktionstasten, Pfeilblock und Steuertasten.

**Die Schlussfolgerung des Absatzes bleibt richtig.** F3 ist eine
Funktionstaste, wird weiter über den Code nachgeschlagen, und die Messung aus
`spikes/fn-tasten/messung-A.txt` trägt weiter. Falsch ist allein die Reichweite
der Prämisse.

## Warum das zählt

Der Kopf ist die Stelle, an der ein Leser der Datei die Regel sucht, bevor er
sich eine Kombination legt. Zwei Blöcke weiter unten haben genau diese Prämisse
zu einer Meidung von `y` und `z` geführt, die es nie gebraucht hätte
(`260810-0011`). Der Satz im Kopf ist derselbe Fehler an der Stelle mit der
größten Reichweite: er lädt dazu ein, den Stellen-Nachschlag als das allgemeine
Verhalten zu lesen, und die beiden nachgezogenen Blöcke widersprechen ihm nun
sichtbar.

Der Schaden ist geringer als bei `260810-0011`, weil hier keine Regel für
eigene Belegungen daran hängt. Deshalb `Low` und nicht `Medium`.

## Was zu tun ist

Eine Wortänderung, kein Umbau. Den Halbsatz auf die Tastensorte einschränken,
über die der Absatz tatsächlich spricht — etwa: "Funktionstasten schlägt KRK
über den Tastencode nach, und F3 mit gehaltener fn-Taste erzeugt denselben
Tastencode wie ein nacktes F3." Alles Weitere des Absatzes bleibt unverändert,
samt Messbeleg und der Einschränkung über die Touch Bar.

Die allgemeine Regel ist im selben Bau schon einmal ausgeschrieben, im Block zum
eingebauten Editor (Zeilen 484–499) mit Verweis auf den Modulkopf von
`parser.rs`; der Kopf braucht sie nicht zu wiederholen und soll sie nicht
umformulieren.

Ausführender ist `ontocoder`, weil `.toml` nicht dem `coder` gehört. Nicht in
`260810-0011` mit erledigt, weil dessen Schreibgrenze ausdrücklich auf die
beiden dort genannten Blöcke lautete.

---
Resolved: Auf dem hier vorgeschlagenen Weg geschlossen, als Wortänderung im Dateikopf und ohne neuen Absatz. Geändert ist ausschließlich `resources/default-keymap.toml`, und darin ausschließlich der eine Halbsatz in Zeile 42; der Absatz über die fn-Taste ist dabei neu umbrochen worden, weil der längere Satzanfang die Zeilenbreite von rund 79 Zeichen sonst überschritten hätte. Keine Belegungszeile ist angefasst, die beiden Kommentarblöcke bei Zeile 484 und 625 sind nicht angerührt, und keine Datei unter `crates/**` ist berührt.

Aus "KRK belegt den Tastencode, und F3 mit gehaltener fn-Taste erzeugt denselben Tastencode wie ein nacktes F3" ist geworden: "Funktionstasten schlaegt KRK ueber den Tastencode nach, und F3 mit gehaltener fn-Taste erzeugt denselben Tastencode wie ein nacktes F3". Alles Weitere des Absatzes steht unverändert, samt Verweis auf C3, dem Messbeleg `spikes/fn-tasten/messung-A.txt` Ereignisse #03 bis #05 und der Einschränkung über die Touch Bar des Referenzgeräts. Die allgemeine Regel wiederholt der Kopf nicht; sie steht weiter genau einmal in der Datei, im Block zum eingebauten Editor bei Zeile 484, und einmal im Modulkopf von `parser.rs`, auf den jener Block verweist.

Die Behauptung ist vor dem Umschreiben am Code geprüft und trifft als allgemeine Aussage nicht mehr zu: `Taste::kennung` (`crates/krk-core/src/tasten/parser.rs:192-198`) legt jeden einbuchstabigen Namen aus einem ASCII-Kleinbuchstaben oder einer Ziffer auf `Tastenkennung::Zeichen`, und `Kombination::aus_tastendruck` (dort, Zeilen 569-576) filtert bei der Stellensuche über den Code jede Taste mit Zeichenkennung ausdrücklich aus. Über den Tastencode gehen nur noch Funktionstasten, Pfeilblock und Steuertasten. Für F3 trägt die Prämisse damit weiter, und die Schlussfolgerung des Absatzes bleibt unverändert richtig.

Der Kopf ist auf dieselbe Prämisse hin ganz durchgesehen, nicht nur an der gemeldeten Zeile: Zeile 42 war die einzige Stelle in den Zeilen 1 bis 97, die den Nachschlagweg behauptet. Der neue Wortlaut ist mit den beiden nachgezogenen Blöcken konsistent, ohne sie umzuformulieren — Block 1 sagt "Buchstaben und Ziffern werden ueber das gemeldete Zeichen nachgeschlagen, alles uebrige ueber den virtuellen Tastencode", der Kopf sagt jetzt die Funktionstasten-Hälfte davon und nichts darüber hinaus.

Verification: `cargo test --workspace` → exit 0, 15 Testziele, alle grün (55, 140, 36, 42, 15, 26, 7, 5, 22, 16, 9, 308, 5, 35 bestandene Proben und ein Doc-Test-Ziel ohne Proben, ein `ignored`), zusammen 721. Die Belegungsdatei geht über `include_str!` in den Bau, ein Formfehler hätte den Lauf angehalten; `jede_ausgelieferte_kombination_traegt_die_kennung_ihrer_tastensorte` prüft daneben für jede der ausgelieferten Kombinationen, welche der beiden Nachschlagarten sie trägt, und ist damit die Probe hinter dem korrigierten Satz.

Ein neuer Defekt ist bei der Nebenwirkungsprüfung gefunden und nicht mitbehoben worden, weil er außerhalb der Schreibgrenze lag: die Fehlermeldung zu `fn+f3` in `crates/krk-core/src/tasten/parser.rs:453-456` führt dieselbe Prämisse ("KRK belegt den Tastencode") und ist von den vier Stellen die einzige, die ein Nutzer im Betrieb sieht. Geführt als `issues/260810-0935_o_die-fehlermeldung-zur-fn-taste-behauptet-den-tastencode-als-allgemeinen-nachschlagweg.md`, Schwere Low, Ausführender `coder`. Die beiden Fundstellen im geschlossenen Spec der Runde 1 (Zeilen 176 und 243) sind dort als kein Defekt eingeordnet, mit Begründung.

Die Umbenennung des Markers `_o_` → `_c_` macht der Nutzer.
