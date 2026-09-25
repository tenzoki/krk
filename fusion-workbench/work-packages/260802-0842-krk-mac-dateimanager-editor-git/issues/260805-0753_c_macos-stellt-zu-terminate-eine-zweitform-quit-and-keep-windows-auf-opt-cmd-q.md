macOS stellt zu `terminate:` eine Zweitform "Quit and Keep Windows" auf Opt+Cmd+Q dazu

---

S13c hat "Fenster schließen" von `performClose:` auf den eigenen Selektor `fensterSchliessen:` umgestellt, damit AppKit keine Zweitform "Close All" auf Opt+Shift+Cmd+W mehr dazustellt. Das wirkt, gemessen am 260805-0753 am laufenden Bündel: das Menü "Fenster" trägt genau zwei Einträge.

Derselbe Mechanismus greift beim Eintrag "KRK beenden", und dort ist er noch offen. Gemessen am 260805-0753 über die Bedienungshilfen am laufenden Bündel:

```
KRK/KRK beenden          kuerzel=Q mod=0   aktiv=true
KRK/Quit and Keep Windows kuerzel=Q mod=2  aktiv=true
```

`mod=2` ist die Wahltaste. Der zweite Eintrag heißt englisch, trägt Opt+Cmd+Q und stammt nicht aus dem Programmtext von KRK: `crates/krk-ui/src/appkit/menue.rs` baut im Menü "KRK" genau einen Eintrag.

---

## Warum es zählt

Es ist derselbe Sachverhalt wie bei "Close All" und fällt unter dasselbe Abnahmekriterium aus C3:

> Das Hauptmenü der laufenden Anwendung trägt genau die Kombinationen, die die Belegung für seine Einträge führt, und keine weitere. Kürzel, die macOS von sich aus zu einem Menüeintrag hinzustellt, sind entweder unterdrückt oder stehen als eigener Eintrag in der Belegung.

Opt+Cmd+Q löst etwas aus, steht in keiner Tastenliste und ist nicht umbelegbar.

## Warum `--menue-protokoll` es nicht meldet

Die Marke liest das Menü unmittelbar nach `finishLaunching` aus und beendet. Zu diesem Zeitpunkt steht die Zweitform von "Close All" bereits da (mit einer Sonde am 260805 gegengeprüft), die von "Quit and Keep Windows" noch nicht: AppKit stellt sie erst, wenn die Anwendung wirklich läuft. Der Befund oben stammt deshalb aus der laufenden Anwendung über die Bedienungshilfen und nicht aus der Marke.

Das ist ein zweiter, kleinerer Defekt an derselben Stelle: `--menue-protokoll` misst nicht alles, was es zu messen verspricht. Wer den Eintrag unten behebt, sollte zugleich prüfen, ob ein späterer Auslesezeitpunkt die Marke vollständig macht.

## Was zu tun ist

Dieselbe Antwort wie bei "Fenster schließen". Der Anwendungsdelegierte bekommt einen Selektor `beenden:` neben `fensterEinblenden:` und `fensterSchliessen:`, der Menüeintrag trägt ihn statt `terminate:`, und der Delegierte ruft `terminate:` an `NSApplication`.

`inference:` Dass die Zweitform allein an `terminate:` hängt, ist aus dem gemessenen Befund und aus dem gleich gelagerten, gemessenen Fall `performClose:` geschlossen. Nachgemessen ist es für `terminate:` nicht. Die Gegenprobe geht wie bei "Close All": den Selektor tauschen und das laufende Menü ein zweites Mal auslesen.

Zusammen mit `issues/260805-0753_o_cmd-q-loest-etwas-aus-und-steht-in-keiner-tastenliste.md` zu behandeln: die beiden betreffen denselben Menüeintrag, die eine Hälfte ist eine Datenänderung, die andere Code.

---

Herkunft: gefunden bei der Abnahme von S13c am 260805-0753, beim Auslesen des Hauptmenüs der laufenden Anwendung.

---
Resolved: Der Anwendungsdelegierte trägt den Selektor `beenden:` neben `fensterEinblenden:` und `fensterSchliessen:`, und der Menüeintrag "KRK beenden" trägt ihn statt `terminate:`. Der Delegierte ruft `terminate:` an `NSApplication` selbst (`crates/krk-ui/src/appkit/anwendung.rs`, Selektor `beenden:` und die Methode `beenden`).

**Die Zweitform verschwindet, gemessen.** Am 260805-0841 am neu gebauten, signierten Bündel `target/KRK.app`, über die Bedienungshilfen an der laufenden Anwendung ausgelesen, einmal vier Sekunden und einmal zwölf Sekunden nach dem Start:

```
KRK/KRK beenden  kuerzel=Q mod=0 aktiv=true
```

Das ist der einzige Eintrag des Menüs "KRK". "Quit and Keep Windows" auf Opt+Cmd+Q kommt nicht mehr vor; vor der Änderung stand es dort mit `mod=2`. Der `inference:` des Defekts, die Zweitform hänge allein an `terminate:`, ist damit nachgemessen und bestätigt, auf demselben Weg wie der ursprüngliche Befund und nicht über `--menue-protokoll`. Cmd+Q beendet die Anwendung unverändert: Tastendruck über die Bedienungshilfen gesendet, danach findet `pgrep -x krk` keinen Prozess mehr.

**Zwei Behelfe sind weg.** `NOTBEHELF_BEENDEN` und `notbehelf_befehl` sind aus `crates/krk-ui/src/appkit/menue.rs` entfernt, dazu die Prüfung `der_notbehelf_fuer_beenden_ist_eine_gueltige_kombination`, die allein die Konstante las. Der Eintrag läuft über `befehl(…, "beenden")` wie die übrigen sechs und holt sein Kürzel aus der Belegung. `grep` über `menue.rs` findet keine Kombination mehr als Zeichenkette im Programmtext; die beiden verbliebenen Treffer stehen in Fließtext, der eine gemessene Eingabe und ein Übersetzungsbeispiel beschreibt. Das Abnahmekriterium von S13c, "der Diff zeigt, dass `menue.rs` keine Kombination mehr als Zeichenkette festlegt", trägt damit.

**Ein Kommando dazu.** `resources/default-keymap.toml` sagt im Kommentar zum Eintrag `beenden` zu, dass die Funktion ein Kommando bekommt, sobald der eigene Selektor steht ("Solange er noch nicht steht, trägt die Funktion kein Kommando"). Der Kern führt deshalb jetzt `Kommando::Beenden` mit der Kennung `beenden`, und `kommando_ausfuehren` im Anwendungsdelegierten schickt es auf dieselbe Methode wie der Menüeintrag. Am Verhalten ändert das nichts, es verschiebt nur den Weg: der Ereignisabgriff schluckt Cmd+Q jetzt selbst, statt es an das Menü weiterzureichen. Die Prüfung `jede_kennung_des_hauptmenues_steht_in_der_auslieferungsbelegung` nennt `beenden` mit.

**Der Kopfkommentar von `systemzusaetze_unterdruecken` ist nachgezogen**, wie der `ontocoder` vermerkt hat. Er belegte seine Messung mit einem `plutil -extract` gegen `KRK.app/Contents/Info.plist`, das nach dem Entfernen der beiden Schlüssel aus der Vorlage nicht mehr reproduziert. Die Messung steht jetzt in der Vergangenheitsform, dazu ein Absatz, der sagt, warum sie sich nicht nachstellen lässt und wo beide Messungen vollständig stehen.

**Der zweite, kleinere Defekt des Datensatzes bleibt offen.** `--menue-protokoll` sieht die Zweitform von "Quit and Keep Windows" nicht, weil AppKit sie erst an der wirklich laufenden Anwendung stellt; die Marke liest unmittelbar nach `finishLaunching` aus und beendet. Ob ein späterer Auslesezeitpunkt die Marke vollständig macht, ist hier nicht geprüft. Eigener Datensatz: `issues/260805-0841_o_menue-protokoll-sieht-die-spaet-gestellten-zweitformen-nicht.md`.

Geprüft am 260805-0841: die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` enden alle mit 0. Der Testlauf zählt 13 Testprogramme, alle mit 0 gescheiterten Prüfungen.
