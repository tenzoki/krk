# Cmd+Q in die Belegung, und die zwei wirkungslosen `Info.plist`-Schlüssel

**Status:** Complete
**Agent:** ontocoder
**Datum:** 260805-0820
**Defekte:** `issues/260805-0753_c_cmd-q-loest-etwas-aus-und-steht-in-keiner-tastenliste.md`, `issues/260805-0753_c_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md`
**Spec:** `planning/260802-1036_o_spec-navigator-geruest.md`, C3
**Entscheid:** `decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`

---

## Was geändert ist

Zwei Dateien, beide unter `resources/`. In `crates/`, in `xtask/`, im Plan und im Spec ist nichts angefasst.

### `resources/default-keymap.toml`: der sechste Menüeintrag

Ein neuer Abschnitt am Ende der Datei, `C3: das Beenden der Anwendung`, mit einem Eintrag:

```toml
[[funktion]]
id = "beenden"
name = "KRK beenden"
tasten = ["cmd+q"]
```

Die Kennung `beenden` nennen der Defekt und der Kopfkommentar von `NOTBEHELF_BEENDEN` in `menue.rs` gleichlautend; sie folgt der Schreibweise der Datei (ASCII, ohne Umlaute, kleingeschrieben) und steht bei keiner anderen Funktion. Die Beschriftung "KRK beenden" ist die des Menüeintrags. Für die Belegungsansicht ist sie die eindeutigere Wahl gegenüber einem bloßen "Beenden", das neben "Laufende Operation abbrechen" auf das Abbrechen einer Operation zu lesen wäre; sie hält zugleich Menü und Belegungsansicht wörtlich beieinander, was C3 verlangt, weil das Menü sein Kürzel von hier nimmt.

Der Kopfkommentar zählt jetzt 56 Funktionen und 63 Kombinationen statt 55 und 62. Keine vorhandene Tastenliste ändert sich.

### Kein `gehalten_von`, gegen den Wortlaut des Defekts

Der Defekt schlägt `gehalten_von = "menue"` vor. Der Eintrag trägt es nicht, und das ist die eine sachliche Abweichung dieses Durchgangs.

Der Vorschlag beschreibt den heutigen Stand richtig: `terminate:` geht die Antwortkette hinunter, der Ereignisabgriff führt nichts aus, die Funktion bekommt kein Kommando. Er beschreibt aber nicht den Stand, auf den der Schwesterdefekt `260805-0753_o_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md` zuläuft. Dessen Behebung gibt dem Anwendungsdelegierten einen eigenen Selektor `beenden:` an die Stelle von `terminate:`, damit AppKit keine Zweitform "Quit and Keep Windows" auf Opt+Cmd+Q mehr dazustellt. Mit einem eigenen Selektor am Delegierten hängt an der Funktion ein Kommando, und genau diese Form trägt `fenster_schliessen` seit S13c, nach derselben gemessenen Gegenprobe am Fenstermenü.

`gehalten_von` widerspräche ihr an zwei Stellen des Kerns: `Funktion::kommando` liefert `None`, sobald das Feld gesetzt ist, und `Belegung::nachschlag` überspringt die Funktion ganz. Der Ereignisabgriff erreichte den neuen Selektor damit nie.

Bis der Selektor steht, verhält sich der Eintrag unverändert. `Kommando::KENNUNGEN` führt keine Kennung `beenden`, `Funktion::kommando` liefert deshalb `None`, und `crates/krk-ui/src/appkit/ereignisse.rs:256` reicht einen Tastendruck ohne Kommando weiter, statt ihn zu schlucken (gelesen, nicht geändert). Cmd+Q geht also weiter ins Menü und beendet die Anwendung wie bisher. Dieselbe Lage trägt `belegung_ansehen` auf F1, und der Kommentar am neuen Eintrag sagt es.

### `resources/Info.plist`: die zwei Schlüssel fallen weg

`NSDisabledCharacterPaletteMenuItem` und `NSDisabledDictationMenuItem` sind entfernt. An ihrer Stelle steht ein Kommentar: dass sie dort standen, dass sie nicht greifen, dass AppKit die beiden Namen aus `NSUserDefaults` liest, dass `systemzusaetze_unterdruecken` in `crates/krk-ui/src/appkit/menue.rs` die Sache trägt, und beide Messungen.

Die Wahl fiel auf Entfernen, weil eine Bündelbeschreibung sagen soll, was gilt. Zwei Schlüssel ohne Wirkung sagen, was jemand einmal versucht hat, und ein Leser, der sie für wirksam hält, sucht die Ursache am falschen Ort, wenn die Unterdrückung einmal ausfällt. Der Preis der Wahl, dass das Wissen um den nicht tragenden Weg sonst nur im Defekt und in der Historie stünde, ist mit dem Kommentar bezahlt: er steht an genau der Stelle, an der ein Leser die Schlüssel suchen würde, und ist mit ihnen nicht zu verwechseln.

Nebenbefund: die beiden entfernten Zeilen brachten den einzigen Doppelstrich der Datei mit, in `--menue-protokoll` innerhalb eines XML-Kommentars, wo er nicht stehen darf. `plutil -lint` hat ihn durchgelassen, `xmllint --noout` nicht. Seit dem Entfernen enden beide mit 0; der neue Kommentar vermeidet den Doppelstrich.

---

## Was gemessen ist

### Der Befund des `coder` zur `Info.plist`, unabhängig nachgeprüft

Nicht übernommen, sondern gegengeprüft, und zwar in der umgekehrten Richtung. Der `coder` hat die beiden Namen als Nutzervorgabe auf YES gesetzt und die Systemzeilen verschwinden sehen. Hier stehen sie auf NO, während die Schlüssel in der Bündelbeschreibung unverändert auf `true` stehen. Gemessen am 260805-0813 am gebauten Bündel `target/KRK.app` vom 260805-0800:

```
$ plutil -extract NSDisabledCharacterPaletteMenuItem raw target/KRK.app/Contents/Info.plist
true
$ plutil -extract NSDisabledDictationMenuItem raw target/KRK.app/Contents/Info.plist
true

$ ./target/KRK.app/Contents/MacOS/krk --menue-protokoll
  → 9 Zeilen, keine davon eine Systemzeile

$ ./target/KRK.app/Contents/MacOS/krk --menue-protokoll \
    -NSDisabledCharacterPaletteMenuItem NO -NSDisabledDictationMenuItem NO
  → 14 Zeilen, darunter 2× "Start Dictation…" und 3× "Emoji & Symbols",
    mit cmd+space sichtbar und ctrl+cmd+space verdeckt
```

Die Befehlszeile ist die oberste Vorgabenebene und überschreibt `registerDefaults:`. Wären die Schlüssel der Beschreibung wirksam, hätte eine Nutzervorgabe von NO die fünf Zeilen nicht zurückholen können. **Der Befund ist bestätigt:** AppKit liest die beiden ausschließlich aus `NSUserDefaults`.

### Der zweite Durchgang durch `menue.rs`

Die Frage war, ob nach `NOTBEHELF_BEENDEN` noch eine weitere Kombination im Programmtext steht, die in die Belegung gehört. Antwort: nein.

- `grep -rnE '"(cmd|shift|ctrl|opt)\+' crates/krk-ui/src/appkit/menue.rs` findet genau eine Zeile, `const NOTBEHELF_BEENDEN: &str = "cmd+q";`.
- Die sieben `sel!`-Stellen decken sich mit den sieben Menüeinträgen: `terminate:`, die vier Textbefehle, `fensterEinblenden:` und `fensterSchliessen:`.
- Die einzige Stelle, die ein `NSMenuItem` anlegt, ist `roher_befehl`. Ihre drei Aufrufer sind `befehl` (holt das Kürzel unter der Kennung aus der Belegung), `ohne_kuerzel` (setzt ein leeres Kürzel) und `notbehelf_befehl` (der eine Aufrufer ist "KRK beenden"). Ein Kürzel kann also nur über die Belegung oder über die eine Konstante hereinkommen.

### Die Belegungsdatei

Geprüft am 260805-0820, jeweils am vollständigen Eintrag und nicht als Teilzeichenkette:

| Prüfung | Ergebnis |
|---|---|
| `cmd+q` vor der Änderung frei, über alle 62 ausgelieferten Kombinationen | frei |
| `grep -c '^\[\[funktion\]\]' resources/default-keymap.toml` | 56 |
| Kombinationen insgesamt | 63 |
| Kennungen eindeutig | ja |
| Kombination bei zwei Funktionen desselben Zustellers | keine |
| Der eine erlaubte Doppelfall | `cmd+a`, zwei verschiedene Zusteller |
| `plutil -lint resources/Info.plist` | 0 |
| `xmllint --noout resources/Info.plist` | 0 |
| `plutil -extract NSDisabledCharacterPaletteMenuItem raw resources/Info.plist` | Schlüssel nicht mehr vorhanden |
| `__KRK_VERSION__` in `CFBundleShortVersionString` | unverändert |

---

## Was offen bleibt

**Ein Abnahmekriterium ist rot, und es lag nicht in Reichweite dieses Durchgangs.**

`cargo test -p krk-core --test belegung` meldet einen Fehlschlag von 32 Prüfungen. `eine_unbelegte_kombination_mit_zusatztaste_faellt_nicht_auf_die_sprungmarke` (`crates/krk-core/tests/belegung.rs:626`) nimmt ausgerechnet `cmd+q` als Beispiel für eine Kombination, die ab Werk frei ist. Die Zusage der Prüfung ist von der Änderung unberührt; ihr fehlt allein ein Beispiel, das noch frei ist. Die Prüfung ist Code und gehört dem `coder`, und der Auftrag dieses Durchgangs verbietet jeden Eingriff in `crates/`.

Gemeldet als `issues/260805-0820_o_die-belegungspruefung-nimmt-cmd-q-als-beispiel-fuer-eine-unbelegte-kombination.md`, mit `shift+cmd+q` und `ctrl+j` als geprüft freien Ersatzbeispielen und mit dem Hinweis, warum `opt+cmd+q` keines ist.

`cargo test --workspace` meldet genau diesen einen Fehlschlag; die übrigen 31 Prüfungen der Datei und alle drei anderen Testprogramme bleiben grün.

**Die Codehälfte des Eintrags `beenden` bleibt offen.** `NOTBEHELF_BEENDEN` und `notbehelf_befehl` fallen weg, sobald der `coder` den Eintrag über `befehl(…, "beenden")` denselben Weg gehen lässt wie die übrigen sechs. Erst damit ist das Abnahmekriterium von S13c erfüllt, das dieser Durchgang zur Hälfte vorbereitet hat.

**Ein Satz im Code wird mit dem nächsten Bündelbau falsch.** Der Kopfkommentar von `systemzusaetze_unterdruecken` (`crates/krk-ui/src/appkit/menue.rs:126`) führt als Beleg an, dass `plutil -extract` für beide Schlüssel in `KRK.app/Contents/Info.plist` `true` liefert. Für das Bündel vom 260805-0800 stimmt das, für das nächste nicht mehr. Vermerkt im Abschluss des `Info.plist`-Defekts, weil `menue.rs` dem `coder` gehört.

---

## Geänderte Dateien

| Datei | Was |
|---|---|
| `resources/default-keymap.toml` | Abschnitt `C3: das Beenden der Anwendung` mit dem Eintrag `beenden` auf `cmd+q`, ohne `gehalten_von`; Kopfkommentar von 55/62 auf 56/63 |
| `resources/Info.plist` | `NSDisabledCharacterPaletteMenuItem` und `NSDisabledDictationMenuItem` entfernt, Kommentar an ihrer Stelle |
| `issues/260805-0753_c_cmd-q-loest-etwas-aus-und-steht-in-keiner-tastenliste.md` | `Resolved:`, Marker `_o_` → `_c_` |
| `issues/260805-0753_c_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md` | `Resolved:`, Marker `_o_` → `_c_` |
| `issues/260805-0820_o_die-belegungspruefung-nimmt-cmd-q-als-beispiel-fuer-eine-unbelegte-kombination.md` | neu |

Nur gelesen: `crates/krk-core/src/tasten/belegung.rs`, `crates/krk-core/tests/belegung.rs`, `crates/krk-ui/src/appkit/menue.rs`, `crates/krk-ui/src/appkit/ereignisse.rs`, `xtask/src/bundle.rs`, Plan und Spec.

Nicht committet, wie beauftragt.
