# Was tut ein Doppelklick auf einen Ordner, wenn der Doppelklick die Datei ans Standardprogramm gibt?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** crates/krk-ui/src/appkit/tabelle.rs:955

---

## Question

Der vierte Befehl soll eine Datei mit dem Standardprogramm öffnen, ausgelöst per Doppelklick und per Tastenkombination. Für eine Datei ist das eindeutig. Für einen Ordner nicht, und im Dateifenster stehen Ordner und Dateien in derselben Liste.

Der Bestand lässt die Stelle heute offen. `auswahl_oeffnen` (`crates/krk-ui/src/appkit/tabelle.rs:955`) filtert auf `ist_ordner()`, eine Datei löst dort also nichts aus, und in der `NSTableView` des Dateifensters steht überhaupt keine Doppelklick-Behandlung. Der Einstieg in einen Ordner liegt auf dem nackten Rechts-Pfeil (`resources/default-keymap.toml:213`), der Aufstieg auf dem nackten Links-Pfeil, beides festgelegt am 260805-1411.

Gefragt ist, ob der Doppelklick und die Tastenkombination dasselbe tun. Tun sie es, öffnet ein Doppelklick auf einen Ordner ihn im Finder, und der Einstieg bleibt allein auf dem Rechts-Pfeil. Tun sie es nicht, ist der Doppelklick keine zweite Auslösung eines Befehls, sondern eine eigene Verzweigung an der Maus.

## Options

1. **Der Doppelklick verzweigt, die Taste nicht.** Doppelklick auf einen Ordner steigt in ihn ein, Doppelklick auf eine Datei gibt sie ans Standardprogramm. Die Tastenkombination gibt immer ans System, ein Ordner geht damit an den Finder.
   - Pro: entspricht Finder und ForkLift und der Erwartung jedes Mac-Nutzers an einen Doppelklick. Die Taste bleibt vorhersagbar und erlaubt ausdrücklich, einen Ordner im Finder zu öffnen.
   - Contra: Doppelklick und Taste tun nicht dasselbe. Der Doppelklick löst je nach Zeile einen von zwei bestehenden Befehlen aus, und wo diese Verzweigung wohnt, muss die Planung sauber setzen.
2. **Doppelklick und Taste tun dasselbe: immer ans System übergeben.** Ein Ordner öffnet sich im Finder, der Einstieg bleibt auf dem Rechts-Pfeil.
   - Pro: ein Befehl, zwei Auslöser, keine Verzweigung.
   - Contra: bricht mit der Erwartung an einen Doppelklick in einem Dateiverwalter. Wer eine Ordnerhierarchie mit der Maus durchläuft, bekommt bei jedem Schritt ein Finder-Fenster.
3. **Doppelklick auf einen Ordner steigt ein, Doppelklick auf eine Datei tut nichts.** Dateien öffnet allein die Tastenkombination.
   - Pro: die kleinste Änderung an der Maus.
   - Contra: erfüllt den Entwurf nicht, der den Doppelklick ausdrücklich als Auslöser des Öffnens nennt.

## Constraints

- `NSWorkspace` ist über `appkit/terminal.rs`, `appkit/volumes.rs` und `appkit/zwischenablage.rs:133` schon im Haus; eine neue Systemabhängigkeit entsteht nicht.
- Der Ereignisabgriff fragt nach der Nämlichkeit des Ersthelfers und nicht nach seiner Klasse (`crates/krk-ui/src/appkit/ereignisse.rs`). Eine Mausbehandlung im Dateifenster berührt ihn nicht, gehört aber an dieselbe Stelle wie die vorhandene Klickbehandlung der Vorschau (`appkit/vorschau.rs:121`).

## Recommendation

Option 1. Sie ist die einzige, die dem Nutzer beide Wege lässt, den Einstieg mit der Maus und das Öffnen eines Ordners im Finder, und sie folgt dem, was ein Doppelklick auf dem Mac heißt.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: **Moeglichkeit 1, der Doppelklick verzweigt und die Taste nicht.** Nutzerantwort am
260811-1505.

Doppelklick auf einen Ordner steigt in ihn ein, Doppelklick auf eine Datei gibt sie an das
Standardprogramm des Systems. Die Tastenkombination uebergibt **immer** ans System; ein Ordner
geht damit an den Finder.

Damit hat der Nutzer beide Wege: den Einstieg mit der Maus, wie ein Doppelklick auf dem Mac
gelesen wird, und das Oeffnen eines Ordners im Finder ueber die Taste. Der Rechts-Pfeil bleibt
als Einstieg unberuehrt.
