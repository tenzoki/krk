# Löscht Shift+Delete in den Papierkorb oder endgültig, und fragt KRK vorher nach?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/_a_circle.md`, `shared/decisions/260802-0842_o_f-tasten-unter-macos-systembelegung.md`

---

## Question

Shift+Delete ist als Löschtaste vorbelegt, aber die Kombination trägt in den beiden Vorbildern gegensätzliche Bedeutungen. Auf dem Mac verschiebt Cmd+Backspace in den Papierkorb, und eine endgültige Löschung verlangt einen zweiten, ausdrücklichen Schritt. In Norton Commander und Total Commander unter Windows steht Shift+Entf gerade für die Löschung ohne Papierkorb. KRK erbt beide Traditionen und muss sich entscheiden. Die Frage gehört vor den Aktivierungs-Spec, weil sie über einen möglichen Datenverlust entscheidet und weil die Antwort auch die Norton-Taste F8 mitbestimmt, die auf dieselbe Funktion zeigt.

## Options

1. **Papierkorb, ohne Rückfrage** — Shift+Delete und F8 verschieben in den Papierkorb, sofort und kommentarlos.
   - Pro: schnell, entspricht der Maxime "superschnell". Kein Datenverlust, weil der Papierkorb der Rückweg ist.
   - Contra: wer aus der Norton-Welt kommt, erwartet unter Shift+Delete etwas anderes und wundert sich, warum der Platz auf dem Datenträger nicht frei wird.

2. **Papierkorb ohne Rückfrage, endgültig mit Zusatztaste und Rückfrage** — Shift+Delete räumt in den Papierkorb, eine zweite Belegung löscht endgültig und fragt vorher einmal nach.
   - Pro: der schnelle Weg ist der sichere, der gefährliche Weg verlangt eine bewusste Handlung. Beide Traditionen bekommen ihren Platz.
   - Contra: zwei Löschbefehle statt einem, die der Nutzer auseinanderhalten muss.

3. **Endgültig, mit Rückfrage** — Shift+Delete löscht ohne Papierkorb und zeigt vorher einen Bestätigungsdialog mit der Zahl der betroffenen Einträge.
   - Pro: entspricht der Norton-Erwartung. Die Rückfrage fängt den Fehlgriff ab.
   - Contra: ein Dialog bei jeder Löschung bremst genau die Tastaturarbeit, für die KRK gebaut wird.

4. **Endgültig, ohne Rückfrage, mit Rückgängig-Funktion** — Shift+Delete löscht sofort, KRK hält die letzten Löschungen für ein Rückgängigmachen vor.
   - Pro: schnellster Weg, trotzdem ein Rückweg.
   - Contra: ein eigener Rückgängig-Speicher ist im Kern ein zweiter Papierkorb, den KRK selbst pflegen müsste. Das widerspricht der Maxime "supersimpel".

## Constraints

- Die Antwort gilt gleichermaßen für Shift+Delete und für die Norton-Taste F8, weil beide auf dieselbe Funktion zeigen.
- Sie muss auch für die Mehrfachauswahl tragen, also für das Löschen vieler Einträge in einem Schritt.
- Ordner mit Inhalt sind eingeschlossen, nicht nur einzelne Dateien.

## Recommendation

Option 2 hält den schnellen Weg sicher und gibt dem endgültigen Löschen eine eigene, bewusst zu treffende Taste. Der Preis, zwei Löschbefehle zu erklären, fällt gegen einen unwiederbringlich gelöschten Ordner kaum ins Gewicht. Die Abwägung ist eine Empfehlung, keine geprüfte Aussage.

---
Answered:
Implemented:
Deferred:
Superseded by:
