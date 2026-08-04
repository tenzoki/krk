Der Kopf der Belegungsdatei nennt eine abgeleitete Annahme "gemessen"

---

`resources/default-keymap.toml` schreibt in seinem Kopfkommentar:

> Die fn-Taste ist keine Zusatztaste dieser Schreibweise und kommt in keiner
> Kombination vor. KRK belegt den Tastencode, und F3 mit gehaltener fn-Taste
> erzeugt denselben Tastencode wie ein nacktes F3 (C3, gemessen in
> `spikes/fn-tasten/messung-A.txt`).

Der Halbsatz "gemessen in `spikes/fn-tasten/messung-A.txt`" trägt den Satz nicht, an den er gehängt ist. Gemessen ist allein der Fall **mit** gehaltener fn-Taste. Was ein nacktes F3 erzeugt, ist am Referenzgerät nicht messbar, weil das Gerät einen Touch Bar trägt und ohne fn überhaupt keine F3 liefert.

---

## Die Belege, die dagegenstehen

Drei Stellen des Projekts halten denselben Stand fest, und alle drei sind älter als die Datei:

- `spikes/fn-tasten/messung-A-neuauswertung.txt:64-65` sagt zu genau dieser Frage "NICHT MESSBAR AUF DIESEM GERÄT". Die Selbstauswertung in `messung-A.txt` meldet stattdessen ein "JA" und ist an dieser Stelle falsch: im rohen Protokoll steht bei Ereignis #08 ein `flagsChanged geändert=+function` unmittelbar vor dem zweiten Abschnitt.
- `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md` schreibt im Nachtrag vom 260802-1409 aus, dass die Zustellung der nackten Funktionstasten "unverändert ungemessen" ist.
- Der Modulkopf von `crates/krk-core/src/tasten/normalisierung.rs` nennt die Gleichheit beider Fälle ausdrücklich eine "abgeleitete Annahme und kein Messergebnis" und benennt die Stütze: die Beschreibung von `NSEventModifierFlagFunction` im AppKit-Kopf.

Die Datei zitiert damit als Messung, was die zitierte Datei selbst als nicht messbar ausweist.

## Warum es zählt, obwohl es ein Kommentar ist

Der Kopf von `resources/default-keymap.toml` ist die Vertragsbeschreibung des Parsers aus Schritt 11 und die Stelle, an der ein Leser nachschlägt, warum `fn+` nirgends vorkommt. Die Begründung ist richtig, ihre Belegkette nicht. Genau diese Trennung zwischen gemessen und dokumentiert ist der Punkt, den das Abnahmekriterium von Schritt 11 an den Tastencodes für F4, F6 und F7 einfordert; sie im Kopf derselben Runde aufzuweichen nimmt ihr die Wirkung.

**Der Sachverhalt ändert sich nicht.** Die Löschung des `function`-Bits hängt nicht an der Annahme: trägt sie, sagt das Bit nichts über eine gehaltene fn-Taste; trägt sie nicht, verbietet C3 den zweiten Eintrag trotzdem. Betroffen ist allein die Formulierung.

## Was zu tun ist

Den Klammerzusatz auf den Stand ziehen, den die drei genannten Stellen halten, etwa: "(C3; gemessen ist der Fall mit gehaltener fn-Taste, `spikes/fn-tasten/messung-A.txt` Ereignisse #03 bis #05, der Gleichlauf beider Fälle ist daraus abgeleitet)". Eine Zeile in einem Kommentar; kein Eintrag der Datei ändert sich, und der Parser liest nichts davon.

Die Änderung gehört dem `ontocoder`: `resources/default-keymap.toml` ist eine strukturierte Datendatei, und sie ist vom Nutzer am 260803-2110 durchgesehen und angenommen.

---

Herkunft: gefunden bei der Umsetzung von Schritt 11 am 260803-2317, beim Lesen des Kopfkommentars als Vertragsbeschreibung des Parsers.

---
Resolved: Der Klammerzusatz im Kopf von `resources/default-keymap.toml` trennt jetzt Messung und Ableitung. Er lautet: "(C3; gemessen ist allein der Fall mit gehaltener fn-Taste, `spikes/fn-tasten/messung-A.txt` Ereignisse #03 bis #05; der Gleichlauf beider Faelle ist daraus abgeleitet und am Referenzgeraet nicht messbar, weil dessen Touch Bar ohne gehaltenes fn ueberhaupt keine F3 liefert)." Die zitierten Ereignisse sind nachgesehen: #03 code=99 (F3), #04 code=96 (F5), #05 code=100 (F8), alle mit `mod=function`. Der Stand deckt sich damit mit `spikes/fn-tasten/messung-A-neuauswertung.txt:64-65`, mit dem Nachtrag vom 260802-1409 in `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md` und mit dem Modulkopf von `crates/krk-core/src/tasten/normalisierung.rs`. Kein Eintrag der Datei hat sich dadurch geaendert. Mitgenommen bei der Umsetzung von Schritt 11c am 260804-1214, weil derselbe Textblock ohnehin angefasst wurde.
