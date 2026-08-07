Der Start holt das Fenster nach vorn, aktiviert die Anwendung aber nicht

---

`Anwendungsdelegierter::oberflaeche_aufbauen` beendet den Aufbau mit

```rust
if let Some(fenster) = ivars.fenster.get() {
    fenster.makeKeyAndOrderFront(None);
}
```

(`crates/krk-ui/src/appkit/anwendung.rs:625`). `makeKeyAndOrderFront` ordnet das
Fenster **innerhalb** von KRK nach vorn. Dass die Anwendung selbst die vorderste
wird, verlangt `NSApplication::activate()`, und der Startpfad ruft es nicht.

**Jeder andere Weg tut es.** `fenster_zeigen` (`:1717`) macht beides, erst
`makeKeyAndOrderFront`, dann `activate`, und wird von drei Stellen gerufen: dem
Menüeintrag „Fenster einblenden" (`:415`), dem Klick auf das Dock-Symbol
(`:469`) und dem Kommando aus C7 (`:1493`). Der Start ist der einzige Weg zum
sichtbaren Fenster, der die zweite Hälfte auslässt.

---

## Was es kostet

**Der Abnahmelauf misst nicht.** Die Sitzungsstrecke prüft vor jeder Messgröße
`NSApplication::isActive()` (`crates/krk-ui/src/messmodus.rs:709` über
`Sitzungslage::im_vordergrund`, gesetzt in `anwendung.rs:2688`) und bricht ohne
Zahl ab, wenn KRK nicht vorn steht. Über den Finder oder `open` gestartet fällt
das nicht auf, weil LaunchServices die Anwendung aktiviert. Als Kindprozess
eines Terminals gestartet — genau der Weg, den `krk-bench` nimmt
(`crates/krk-bench/src/messen.rs:1575`, `Command::new(programm).spawn()`) —
aktiviert niemand sie, und `isActive()` bleibt `false`.

**Das erklärt, warum die Abnahme am 260805 einmal lief und seither nicht.** Ohne
`activate()` hängt es daran, ob macOS die Aktivierung von sich aus zugesteht,
und das ist zeit- und zustandsabhängig. Der Datensatz
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`
schreibt selbst, der Bediener habe die Bedingung „bisher nur zufällig erfüllt".
Das war keine Nachlässigkeit des Bedieners, sondern eine fehlende Zeile.

## Was am 260807 geprüft und ausgeschlossen wurde

Der Befund entstand, nachdem der Nutzer den Abnahmelauf zweimal vergeblich
gefahren hatte. Vier andere Erklärungen sind mit Belegen ausgeschieden:

- **Nicht die Tiefe der Aufrufkette.** Der kurze Weg über
  `target/release/krk-bench alle --buendel …` scheitert genauso wie
  `make alle`, und dabei liegt kein `cargo`-Prozess mehr dazwischen.
- **Nicht eine Änderung dieser Sitzung.** `git diff f9a0462..HEAD -- crates/krk-ui/`
  enthält keine Zeile mit `activate`, `ActivationPolicy`, `makeKeyAndOrderFront`
  oder `isActive`.
- **Nicht das System.** `sw_vers` liefert `15.7.7 24G720`, dieselbe Fassung, die
  im Kopf der erfolgreichen Abnahmereihe vom 260805-2207 steht.
- **Nicht eine hängende Instanz.** Zum Zeitpunkt der Prüfung lief kein
  `KRK.app`- und kein `krk-bench`-Prozess.
- **Nicht die Bündelbeschreibung.** Die `Info.plist` des gebauten Bündels führt
  `NSPrincipalClass = NSApplication` und `CFBundlePackageType = APPL`, kein
  `LSUIElement` und kein `LSBackgroundOnly`. Die Änderung aus `880cb70` hat nur
  `CFBundleLocalizations` und `CFBundleDevelopmentRegion` hinzugefügt.

## Der Weg

`fenster_zeigen` statt des nackten `makeKeyAndOrderFront` am Ende von
`oberflaeche_aufbauen`. Damit gibt es **eine** Stelle, die ein Fenster nach vorn
holt, statt zweier, die es unterschiedlich tun. Kein neuer Mechanismus, keine
Sonderregel für den Messmodus.

**Die Reihenfolge ist dabei nicht beliebig.** Der Kommentar unmittelbar unter der
Zeile hält fest, dass `self.fokus_setzen(fokus::BEIM_START)` **nach**
`makeKeyAndOrderFront` stehen muss, sonst überschreibt AppKit den Fokus beim
ersten Anzeigen mit der ersten Ansicht der Schlüsselansichtskette; das war der
Defekt vom 260805-1845. Wer die Zeile ersetzt, prüft, dass diese Ordnung hält.

## Vorbehalt

`inference:`, nicht gemessen. Der Datensatz `260806-1303` hat gemessen, dass
`activate()` und `activateIgnoringOtherApps(true)` aus einem **Hintergrundprozess**
nichts bewirken. Der Fall hier ist ein Terminal im **Vordergrund**, und dort ist
`activate()` beim Start nie versucht worden, weil der Code es nicht ruft. Die
Vermutung ist gut begründet, aber der Beleg ist erst der nächste Abnahmelauf.

**Zuständig:** `coder`.

**Aufgefallen bei:** dem Abnahmelauf des Nutzers am 260807-1600, nach dem
beschränkten Abschluss der Runde 1.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1235_c_der-sitzungslauf-der-abnahmestrecke-bricht-bei-l5-tab-ab-und-gibt-keine-zahl-mehr-aus.md`,
`crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/messmodus.rs`

---
Resolved: `oberflaeche_aufbauen` beendet den Aufbau nicht mehr mit einem nackten
`makeKeyAndOrderFront`, sondern ruft `fenster_zeigen`
(`crates/krk-ui/src/appkit/anwendung.rs:634`). Damit gehen alle vier Wege zum
sichtbaren Fenster ueber dieselbe Stelle, und der Start ruft `activate()` wie der
Menueeintrag, der Dock-Klick und das Kommando aus C7. Der Kommentar an der Zeile
haelt fest, warum der Start aktiviert; der Kommentar zum Fokus haelt die Ordnung
`fokus_setzen` **nach** `makeKeyAndOrderFront` und nennt jetzt die Funktion, in
der der Aufruf steckt. Die Messstrecke blieb unberuehrt: `messung_unmoeglich`
bricht weiterhin mit `NICHT_IM_VORDERGRUND` ab, wenn KRK nicht vorn steht, und
`krk-bench` ist unveraendert.

`make check` ist gruen, das Buendel `target/KRK.app` steht signiert.

**Der Beleg ist erst der naechste Abnahmelauf.** Der Vorbehalt des Befunds gilt
unveraendert: dass `activate()` aus einem Kindprozess eines im Vordergrund
stehenden Terminals die Anwendung wirklich nach vorn holt, ist begruendet
angenommen und nicht gemessen. Erst eine Abnahmereihe, die Zahlen statt
`NICHT_IM_VORDERGRUND` liefert, schliesst die Luecke. Der Lauf gehoert dem
Nutzer; ein Hintergrundprozess erzeugt genau den Fehler, den die Aenderung
behebt.
