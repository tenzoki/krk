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
