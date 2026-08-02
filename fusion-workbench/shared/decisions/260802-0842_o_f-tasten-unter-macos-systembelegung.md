# Wie erreicht KRK die Tasten F3 bis F8, die macOS ab Werk selbst belegt?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/_a_circle.md`, `shared/decisions/260802-0842_o_loeschen-papierkorb-oder-endgueltig.md`

---

## Question

Die Vorbelegung des Circles legt die Norton-Funktionen auf F3 bis F8. Auf einem Mac lösen genau diese Tasten ab Werk Systemfunktionen aus, etwa Mission Control, Tastaturbeleuchtung und Lautstärke. Eine Anwendung sieht das nackte F5 nur, wenn der Nutzer in den Systemeinstellungen "F1, F2 usw. als Standard-Funktionstasten verwenden" aktiviert hat oder beim Drücken die Fn-Taste hält. Ohne eine Festlegung liefert KRK eine Vorbelegung aus, die auf einem unveränderten Mac schlicht nicht reagiert, und der erste Eindruck ist ein Werkzeug, dessen dokumentierte Tasten nichts tun. Die Frage muss vor dem Aktivierungs-Spec beantwortet sein, weil sie die ausgelieferte Standardbelegung und den Text des Erststart-Dialogs bestimmt.

## Options

1. **Fn+F3 bis Fn+F8 als ausgelieferte Belegung** — KRK belegt die Kombination mit Fn, die auf jedem Mac ohne Systemänderung ankommt.
   - Pro: funktioniert sofort nach der Installation, ohne dass der Nutzer etwas umstellt. Keine Kollision mit Mission Control und den Medientasten.
   - Contra: zwei Tasten statt einer, das widerspricht dem Norton-Gefühl. Auf externen PC-Tastaturen ohne Fn-Taste nicht erreichbar.

2. **Nacktes F3 bis F8, mit Hinweis beim Erststart** — KRK belegt die reinen F-Tasten und erklärt beim ersten Start, wie der Nutzer die Systemeinstellung umstellt.
   - Pro: exakt die Norton-Belegung, eine Taste pro Funktion. Nutzer, die Norton-Kürzel wollen, haben die Einstellung oft ohnehin schon aktiv.
   - Contra: verlangt einen Eingriff in die Systemeinstellungen, der global für alle Anwendungen wirkt. Wer den Hinweis wegklickt, hat ein Werkzeug mit toten Tasten.

3. **Beides ab Werk belegt** — sowohl F3 bis F8 als auch Fn+F3 bis Fn+F8 lösen dieselbe Funktion aus, je nachdem, was bei diesem Nutzer ankommt.
   - Pro: funktioniert in beiden Systemzuständen, ohne dass der Nutzer etwas wissen muss.
   - Contra: die Belegungstabelle trägt doppelte Einträge, was die freie Konfigurierbarkeit unübersichtlicher macht. Beim Umbelegen muss der Nutzer zwei Zeilen ändern statt einer.

4. **Norton-Belegung auf andere Tasten legen** — die Funktionen aus F3 bis F8 wandern auf Kombinationen, die macOS frei lässt.
   - Pro: keine Kollision, keine Systemeinstellung, kein Erklärbedarf.
   - Contra: verfehlt die ausdrückliche Vorgabe des Nutzers, F3 bis F8 wie bei Norton zu belegen.

## Constraints

- Jede Taste bleibt frei konfigurierbar. Die Antwort betrifft nur die ausgelieferte Vorbelegung, nicht die Freiheit des Nutzers, sie zu ändern.
- Die Norton-Zuordnung selbst steht fest: F3 Ansehen, F4 Bearbeiten, F5 Kopieren, F6 Verschieben und Umbenennen, F7 Ordner anlegen, F8 Löschen.
- Löschen ist zusätzlich auf Shift+Delete vorbelegt und bleibt damit auch dann erreichbar, wenn F8 auf einem System nicht ankommt.

## Recommendation

Option 3 löst das Problem ohne Rückfrage an den Nutzer und ohne Eingriff in dessen Systemeinstellungen. Der Nachteil, zwei Einträge in der Belegungstabelle zu führen, lässt sich entschärfen, indem die Konfigurationsoberfläche beide als eine Zeile mit zwei Auslösern darstellt. Diese Empfehlung ist eine Abwägung, keine geprüfte Aussage; die Entscheidung liegt beim Nutzer.

---
Answered:
Implemented:
Deferred:
Superseded by:
