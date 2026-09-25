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

---
Resolved: `resources/default-keymap.toml` trägt seit dem 260805-0820 einen sechsten Eintrag, in einem eigenen Abschnitt `C3: das Beenden der Anwendung` am Ende der Datei:

```toml
[[funktion]]
id = "beenden"
name = "KRK beenden"
tasten = ["cmd+q"]
```

Die Datei zählt damit 56 Funktionen und 63 Kombinationen; der Kopfkommentar nennt beide Zahlen nachgeführt. Kein vorhandener Eintrag ändert seine Tastenliste.

**Abweichung vom Vorschlag oben: der Eintrag trägt kein `gehalten_von`.** Der Vorschlag dieses Datensatzes nennt `gehalten_von = "menue"` und begründet es damit, dass `terminate:` die Antwortkette hinuntergeht und die Funktion nie ein Kommando bekommt. Das trifft den heutigen Stand, aber nicht den, auf den der Schwesterdefekt `260805-0753_o_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md` zuläuft: dessen Behebung gibt dem Anwendungsdelegierten einen eigenen Selektor `beenden:` statt `terminate:`, und damit hängt an der Funktion ein Kommando am Delegierten. Genau diese Form trägt `fenster_schliessen` seit S13c, aus demselben Grund und nach derselben Gegenprobe am Fenstermenü. Ein `gehalten_von` widerspräche ihr: die beiden Stellen, die das Feld auswerten, nähmen der Funktion ihr Kommando (`Funktion::kommando`) und den Nachschlag (`Belegung::nachschlag`), und der Ereignisabgriff erreichte den neuen Selektor nie.

Bis der Selektor steht, verhält sich der Eintrag unverändert: `Kommando::KENNUNGEN` führt keine Kennung `beenden`, `Funktion::kommando` liefert deshalb `None`, und `crates/krk-ui/src/appkit/ereignisse.rs:256` reicht einen Tastendruck ohne Kommando weiter, statt ihn zu schlucken. Cmd+Q geht also weiter ins Menü und beendet die Anwendung wie bisher. Dieselbe Lage trägt `belegung_ansehen` auf F1.

Geprüft am 260805-0820:

- `cmd+q` war frei, nachgesehen am vollständigen Eintrag über alle 62 bis dahin ausgelieferten Kombinationen, nicht als Teilzeichenkette.
- `grep -c '^\[\[funktion\]\]' resources/default-keymap.toml` liefert 56.
- Keine Kombination steht bei zwei Funktionen desselben Zustellers; `cmd+a` bleibt der eine Fall mit zwei verschiedenen Zustellern.
- Zweiter Durchgang durch `crates/krk-ui/src/appkit/menue.rs`: `NOTBEHELF_BEENDEN` ist die einzige Kombination, die dort als Zeichenkette steht. `grep -rnE '"(cmd|shift|ctrl|opt)\+'` findet nur sie, die sieben `sel!`-Stellen decken sich mit den sieben Menüeinträgen, und die einzige Stelle, die ein `NSMenuItem` anlegt, ist `roher_befehl`; sechs der sieben Einträge holen ihr Kürzel über `befehl` aus der Belegung, der siebte über `notbehelf_befehl`. Eine weitere übersehene Kombination gibt es nicht.

**Ein Abnahmekriterium bleibt bis zur nächsten Codeänderung rot.** `cargo test -p krk-core --test belegung` meldet einen Fehlschlag: `eine_unbelegte_kombination_mit_zusatztaste_faellt_nicht_auf_die_sprungmarke` nimmt ausgerechnet `cmd+q` als Beispiel für eine unbelegte Kombination. Die Prüfung ist Code und gehört dem `coder`; gemeldet als `260805-0820_o_die-belegungspruefung-nimmt-cmd-q-als-beispiel-fuer-eine-unbelegte-kombination.md`. Die übrigen 31 Prüfungen der Datei und alle drei anderen Testprogramme des Arbeitsbereichs bleiben grün.

Offen bleibt außerdem die Codehälfte dieses Eintrags: `NOTBEHELF_BEENDEN` und `notbehelf_befehl` fallen weg, sobald der `coder` den Eintrag über `befehl(…, "beenden")` denselben Weg gehen lässt wie die übrigen sechs. Erst damit ist das Abnahmekriterium von S13c erfüllt.
