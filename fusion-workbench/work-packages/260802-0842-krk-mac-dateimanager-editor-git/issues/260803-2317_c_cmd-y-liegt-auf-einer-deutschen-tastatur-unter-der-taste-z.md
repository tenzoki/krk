Cmd+Y liegt auf einer deutschen Tastatur unter der Taste Z

---

KRK belegt den virtuellen Tastencode und nicht das gemeldete Zeichen. Der Tastencode benennt eine **Stelle** auf der Tastatur, und die Stelle, die auf einer amerikanischen Tastatur "Y" trägt (`kVK_ANSI_Y`, Code 16), trägt auf einer deutschen "Z". Der ausgelieferte Eintrag

```toml
id = "vorschau_umschalten"
tasten = ["f3", "cmd+y"]
```

löst deshalb auf einer deutschen Tastatur aus, wenn der Nutzer Cmd und die Taste **Z** drückt. Der Finder, dessen Kürzel KRK hier übernehmen will, reagiert auf das Zeichen und damit auf die Taste, die "y" erzeugt, also auf die Taste mit der Aufschrift Y.

Auf demselben Gerät liegen KRKs Vorschau und die Übersicht des Finders damit auf zwei verschiedenen Tasten, und die Belegungsansicht beschriftet die KRK-Seite mit "Y", während unter dem Finger ein Z steht.

---

## Umfang

**Genau eine der 52 ausgelieferten Kombinationen ist betroffen.** Deutsch und Amerikanisch unterscheiden sich in der Buchstabenreihe allein im Tausch von Y und Z; alle übrigen Buchstaben der Auslieferungsbelegung (`k`, `v`, `n`, `t`, `w`, `a`, `d`, `r`, `h`, `u`, `g`, `i`, `l`, `b`, `s`, `c`) und alle Ziffern liegen auf beiden Tastaturen an derselben Stelle. Auf einer französischen AZERTY-Tastatur wären es dagegen viele.

Die Fähigkeit selbst bleibt erreichbar: F3 ist der zweite ausgelieferte Weg zur Vorschau und von der Tastaturbauart nicht berührt.

## Woher es kommt

Die Belegung über den Tastencode ist eine bewusste Festlegung aus C3 und für die Funktionstasten auch die richtige: F3 liefert denselben Tastencode, gleich ob der Nutzer fn hält, und ein zeichenbasierter Nachschlag hätte für die Funktionstasten überhaupt kein stabiles Zeichen. Für die Buchstabentasten trägt dieselbe Begründung nicht, weil dort das Zeichen das Stabile ist und die Stelle das Wechselnde. C3 hat die Unterscheidung nicht getroffen; die Cmd-Kürzel der Tabelle kamen erst mit dem Nachtrag vom 260802-1409 hinzu, und die Frage nach der Tastaturbauart stellte sich für die Funktionstasten allein nicht.

Die verwandte Festlegung "KRK erkennt die Tastaturbauart nicht und liefert keine je nach Gerät verschiedene Vorbelegung aus" (C3) berührt diesen Fall nicht: hier geht es nicht um eine andere Vorbelegung je Gerät, sondern darum, wonach ein einziger Eintrag nachschlägt.

## Was zu entscheiden ist

Drei Wege stehen offen, und die Wahl gehört dem Nutzer, weil sie die Grundhaltung von C3 berührt:

1. **So lassen und die Belegungsansicht ehrlich beschriften.** Die Ansicht zeigt, was unter dem Finger liegt, nicht den ANSI-Namen. Kostet eine Umrechnung in der Ansicht aus Schritt 20 und lässt Cmd+Y vom Finder abweichen.
2. **Die eine Kombination tauschen.** `cmd+y` wird zu `cmd+z`, dann trifft es auf einer deutschen Tastatur die Taste Y. Kostet eine Zeile, ist aber genau die geräteabhängige Vorbelegung, die C3 ausschließt: auf einer amerikanischen Tastatur wäre es dann falsch.
3. **Buchstaben und Ziffern über das Zeichen nachschlagen, Funktionstasten weiter über den Tastencode.** Die sachlich richtige Auflösung, und die teuerste: sie verlangt eine zweite Nachschlagart neben der bestehenden und damit genau die Sonderregel, die die Maxime "supersimpel" meidet. Der Ort dafür wäre nicht diese Runde.

**Dringlichkeit.** Bindet keinen Schritt dieser Runde. Vor Schritt 20 zu klären, weil die Belegungsansicht die Beschriftung wählen muss und Weg 1 dort Arbeit verursacht.

---

Herkunft: gefunden bei der Umsetzung von Schritt 11 am 260803-2317, beim Eintragen der Buchstaben in die Tastentabelle `crates/krk-core/src/tasten/parser.rs`.

---
Resolved: 260804-0830, Weg 1 ohne die Umrechnung in der Ansicht. **Der Nutzer hat entschieden, `cmd+y` zu lassen, wie es ist.** Drei Gründe trägt seine Antwort: F3 ist der ausgelieferte Hauptweg zur Vorschau und von der Tastaturbauart nicht berührt, das Cmd-Kürzel ist der zweite Weg und nicht der einzige, und die Belegung ist ab Werk änderbar, sodass jeder Nutzer die Kombination auf die Taste unter seinem Finger legen kann.

**Was das für Schritt 20 heißt.** Weg 1 des Datensatzes nannte als Preis eine Umrechnung in der Belegungsansicht, damit die Ansicht zeigt, was unter dem Finger liegt. Diese Umrechnung ist mit der Entscheidung **nicht** beauftragt: die Belegungsansicht beschriftet weiter nach der Kombinationsschreibweise aus Schritt 9, also `cmd+y` als "Cmd+Y". C3 verlangt von ihr nur, Funktionstasten als F3 bis F8 zu schreiben und nirgends "Fn+" davorzusetzen; eine Zusage über die Beschriftung von Buchstabentasten trägt der Spec nicht. `resources/default-keymap.toml` bleibt unverändert.

**Was offen bleibt und nicht in dieser Runde entschieden wird.** Weg 3, Buchstaben und Ziffern über das gemeldete Zeichen nachzuschlagen und Funktionstasten weiter über den Tastencode, ist die sachlich vollständige Auflösung und verlangt eine zweite Nachschlagart. Sie bleibt einer späteren Runde vorbehalten; dieser Datensatz hält sie fest, und ein neuer Datensatz greift sie auf, sobald eine Fähigkeit sie braucht.
