Cmd+Q löst etwas aus und steht in keiner Tastenliste der Belegung

---

`resources/default-keymap.toml` führt seit S13b 55 Funktionen mit 62 Kombinationen und deckt damit fünf der sechs Kürzel des Hauptmenüs ab. Das sechste fehlt: der Eintrag "KRK beenden" im Menü "KRK" trägt Cmd+Q, und die Datei kennt weder eine Funktion `beenden` noch die Kombination `cmd+q`.

Nachgesehen am 260805-0753: `grep -n "cmd+q\|beenden\|Beenden" resources/default-keymap.toml` findet nichts.

---

## Warum es zählt

C3 sagt seit dem 260805-0000 zu:

> Jede Tastenkombination, die in KRK etwas auslöst, steht in der Belegung, auch wenn ein Menüeintrag sie trägt. Das Hauptmenü nimmt seine Kürzel aus der Belegung und legt keine eigenen fest. Es gibt damit keine Kombination, die die Konflikterkennung nicht sieht und die der Nutzer nicht umbelegen kann.

Cmd+Q beendet die Anwendung, ist also eine Kombination, die etwas auslöst. Sie wird von der Konflikterkennung nicht gesehen und ist nicht umbelegbar. Der blinde Fleck, den der Nutzerentscheid vom 260805-0000 von sechs auf null bringen sollte, steht damit bei eins.

Die Folge ist heute im Programmtext sichtbar: `crates/krk-ui/src/appkit/menue.rs` trägt eine Konstante `NOTBEHELF_BEENDEN` mit dem Wert `"cmd+q"`, die einzige Kombination der Datei, die nicht aus der Belegung kommt. Das Abnahmekriterium von S13c verlangt das Gegenteil ("Der Diff zeigt, dass `menue.rs` keine Kombination mehr als Zeichenkette festlegt") und ist an dieser einen Stelle nicht erfüllt.

## Warum S13c es nicht selbst behoben hat

`resources/default-keymap.toml` ist eine Datendatei und gehört dem `ontocoder`. Die Dateiliste von S13c nennt sie ausdrücklich als **lesend**.

## Was zu tun ist

Ein sechster Eintrag in `resources/default-keymap.toml`, nach demselben Muster wie die vier Textbefehle:

```toml
[[funktion]]
id = "beenden"
name = "KRK beenden"
tasten = ["cmd+q"]
gehalten_von = "menue"
```

`gehalten_von = "menue"` ist die richtige Einordnung und kein Behelf: `terminate:` geht die Antwortkette hinunter bis zu `NSApplication`, der Ereignisabgriff führt nichts aus, und die Funktion bekommt nie ein Kommando. Genau dieselbe Form tragen die vier Textbefehle.

`cmd+q` ist in der Datei frei; nachgesehen am 260805-0753 am vollständigen Eintrag über alle 55 Funktionen.

Danach im Code: `NOTBEHELF_BEENDEN` und `notbehelf_befehl` in `crates/krk-ui/src/appkit/menue.rs` fallen weg, und der Eintrag geht über `befehl(…, "beenden")` denselben Weg wie die übrigen sechs.

---

Herkunft: gefunden bei der Umsetzung von S13c am 260805-0753, beim Bau des Hauptmenüs aus der Belegung.
