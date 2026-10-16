# Wie lange gilt ein eingegebener Schlüssel für `secrets.txt`, was genau wird gemerkt, und wie verschwindet er wieder?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261016-0229-secrets-mit-sechs-feldern-schluesselcache-und-verbindungen.md, 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md, 260926-0007_*_wie-lange-gilt-eine-eingegebene-pin.md, 260926-0007_*_wie-wird-secrets-txt-verschluesselt-und-was-schuetzt-eine-vierstellige-pin.md

---

## Question

Der Nutzer verlangt, dass der einmal eingegebene Schlüssel zehn Minuten im Arbeitsspeicher bleibt. Bisher gilt eine PIN allein, solange `secrets.txt` im Editor offen ist (`260926-0007_*_wie-lange-gilt-eine-eingegebene-pin.md`, Möglichkeit 1, umgesetzt), und jedes neue Öffnen fragt wieder. Zu entscheiden sind vier Dinge, die über diesen Plan hinaus binden, weil jeder künftige Weg zu `secrets.txt` sie erbt: was im Speicher steht (PIN oder abgeleiteter Schlüssel), ab wann die zehn Minuten zählen und mit welcher Uhr, wie der Wert beim Ablauf und beim Beenden verschwindet, und was „PIN ändern“ mit ihm tut.

## Options

1. **Gemerkt wird der abgeleitete Schlüssel samt Salz und Parametern, zehn Minuten ab der Eingabe, gemessen mit einer Uhr, die im Ruhezustand weiterläuft; Ablauf, Ersetzen und Beenden nullen die Schlüsselbytes; „PIN ändern“ ersetzt den gemerkten Schlüssel durch den neuen.**
   - Pros: das Öffnen mit gemerktem Schlüssel kostet keine Ableitung (rund 0,54 s auf dem Referenzgerät); die PIN selbst steht nach dem Ableiten nirgends mehr im Modell; ein Mac, der eine Stunde schläft, öffnet danach nicht ohne Abfrage.
   - Cons: ein Schlüssel öffnet nur die Datei, deren Salz er trägt; nach einem Wechsel der Datei (anderer Notizordner, PIN von einer zweiten Instanz geändert) passt er nicht, und KRK muss dann zurück auf die PIN-Abfrage fallen.
2. **Gemerkt wird die PIN, sonst wie 1.**
   - Pros: passt zu jeder Datei, deren PIN gleich ist.
   - Cons: jedes Öffnen kostet wieder eine Ableitung; die PIN ist das kleinere, aber auch das wiederverwendbare Geheimnis (zehntausend Werte, oft dieselbe wie anderswo).
3. **Wie 1, aber zehn Minuten ab der letzten Nutzung.**
   - Pros: bequemer bei häufigem Öffnen.
   - Cons: der Auftrag sagt „für 10 min. in memory gecached“ und nennt keine Verlängerung; eine gleitende Frist macht aus zehn Minuten eine unbegrenzte Dauer, solange jemand den Mac benutzt.

## Constraints

- Der Schlüssel geht nie auf die Platte, auch nicht in `session.toml` (Directive des Pakets, zweitens).
- Die bisherige Regel für die **offene** Datei bleibt: der Editor hält ihren Schlüssel, solange sie offen ist, auch über die zehn Minuten hinaus. Der Merker beantwortet allein, ob ein neues Öffnen ohne Abfrage geht.
- `Instant` aus `std` zählt auf macOS nach unserer Kenntnis den Ruhezustand nicht mit (Inferenz, im Plan zur Prüfung am Quelltext der Toolchain ausgeschrieben); `clock_gettime(CLOCK_MONOTONIC)` tut es nach `man clock_gettime` dieses Systems ausdrücklich („will continue to increment while the system is asleep“, geprüft am 261016).
- `unsafe` steht im Kern allein in `verzeichnis/sys.rs`.

## Recommendation

Wir empfehlen Möglichkeit 1. Im Einzelnen:

- **Ein Merker je Prozess**, `krk_core::heimordner::schluesselmerker` (`Schluesselmerker`; der Name `merker` ist in `ablage` schon vergeben), rein und ohne Uhr: `merken(schluessel, jetzt)`, `gueltiger(jetzt)`, `verwerfen()`. `GUELTIGKEIT` ist eine Konstante von 600 s; es gibt keine Einstellung.
- **Die Frist zählt ab der Eingabe und verlängert sich nicht.** Gemerkt wird allein nach einer geprüften Eingabe: ein Öffnen mit PIN, das die Prüfung des Verfahrens bestanden hat, das Festlegen einer neuen PIN und das Ändern der PIN. Ein Öffnen **mit** dem gemerkten Schlüssel merkt nicht neu. Eine falsche PIN wird nie gemerkt.
- **Die Uhr ist `CLOCK_MONOTONIC`**, gelesen an genau einer Stelle (`verzeichnis::sys::wachzeit`), damit eine Ruhephase mitzählt und eine verstellte Wanduhr nichts verlängert.
- **Getilgt wird mit der Kiste `zeroize`**: `Schluessel` nullt seine 32 Byte in `Drop`, also jede Kopie, die fällt; der Merker fällt beim Ablauf (Zeitgeber und Prüfung bei jeder Frage), beim Ersetzen und in `applicationWillTerminate:`. Das Modul `tresor.rs` sagt bis heute ausdrücklich zu, **nicht** zu tilgen; diese Zusage fällt für den Schlüssel und bleibt für Klartext und PIN bestehen, weil die Textfläche des Editors den Klartext hält und `Pin` ein `Copy`-Typ ist.
- **Ein Schlüssel, der die Datei nicht öffnet, fällt**, und KRK fragt nach der PIN. Das deckt jeden Wechsel der Datei hinter dem Rücken des Merkers mit einer Regel ab, ohne dass der Merker den Pfad kennen muss.
- **„PIN ändern“ ersetzt den gemerkten Schlüssel durch den neuen**, gemessen ab dem Wechsel: der Nutzer hat eben die neue PIN eingegeben, und der alte Schlüssel öffnet die Datei ohnehin nicht mehr. Die Prüfung der alten PIN geschieht dafür nicht mehr am Vergleich mit einer gehaltenen PIN, sondern durch Ableiten mit Salz und Parametern des gehaltenen Schlüssels und Vergleich der Schlüssel; `Schutz::Verschluesselt` hält danach keine PIN mehr.

Teilweise abgelöst wird damit `260926-0007_*_wie-lange-gilt-eine-eingegebene-pin.md`: dessen Regel für die offene Datei bleibt, das „jedes neue Öffnen fragt erneut“ gilt ab dieser Arbeit erst nach Ablauf des Merkers.
