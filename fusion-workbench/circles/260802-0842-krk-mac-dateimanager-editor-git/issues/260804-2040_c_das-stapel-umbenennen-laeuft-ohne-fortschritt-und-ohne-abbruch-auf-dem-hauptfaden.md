Das Umbenennen im Stapel läuft ohne Fortschritt und ohne Abbruch auf dem Hauptfaden

---

S17 führt das Stapel-Umbenennen als gewöhnliche Schleife auf dem Hauptfaden aus
(`crates/krk-ui/src/appkit/anwendung.rs`, `stapel_ausfuehren`): je Zeile ohne
Hinweis genau ein `krk_core::operation::umbenennen`, ohne Arbeitsfaden, ohne
Fortschrittsanzeige und ohne Abbruch. Über wenige Dutzend Einträge ist das
richtig; über mehrere Tausend hält es zwei Zusagen aus C4 nicht ein.

---

**Gemessen am 260804-2040** auf demselben APFS-Datenträger, mit dem Prüfordner
unter `/tmp`: 5.000 `rename(2)`-Aufrufe nacheinander brauchen **525 ms**. So
lange steht der Hauptfaden.

Zwei Abnahmekriterien aus C4 treffen das:

- "Eine Operation über mehr als 100 Einträge oder mehr als 100 MB zeigt einen
  Fortschritt und lässt sich mit einem Tastenbefehl abbrechen."
- "Während eine Operation läuft, ist das Fenster bedienbar: Navigation,
  Markierung, Tabwechsel und Fensterwechsel wirken wie sonst."

Der Plan schreibt für S17 ausdrücklich keinen Arbeitsfaden vor, und die
Begründung in `stapel_ausfuehren` ("`rename(2)` fasst keinen Inhalt an") trägt
für den Alltagsfall. Sie trägt nicht für den Fall, den C4 als Schwelle nennt.

**Zwei Wege stehen zur Wahl, und die Entscheidung gehört nicht in diesen
Schritt:**

1. Das Stapel-Umbenennen bekommt eine eigene `Art` in `krk_core::operation` und
   läuft damit über dieselbe Operationsmaschine wie Kopieren, Verschieben und
   Löschen. Fortschritt, Abbruch und die Abschlussliste der übersprungenen
   Einträge kämen ohne eine zweite Bauweise mit. Das ist der größere Eingriff
   und die einzige Antwort, die C4 wörtlich einhält.
2. C4 grenzt das Stapel-Umbenennen ausdrücklich von den vier Operationen mit
   Fortschritt ab. Dann bleibt die Schleife, und der Spec sagt, warum sie
   genügt.

Gefunden bei der Umsetzung von Schritt 17. Betrifft
`crates/krk-ui/src/appkit/anwendung.rs` und, im ersten Weg,
`crates/krk-core/src/operation/`.

---
Resolved: Weg 1 des Datensatzes, mit einer Änderung an der Schwelle davor. C4 sagt Fortschritt und Abbruch seit dem 260804-2318 für jede Operation zu, die länger als 150 ms läuft, statt ab 100 Einträgen oder 100 MB; die Herleitung steht in `decisions/260804-2318_a_fortschrittsschwelle-nach-zeit-statt-nach-menge.md`. Unter der neuen Schwelle liegt das Stapel-Umbenennen mit gemessenen 525 ms über 5.000 Einträge eindeutig innerhalb der Zusage, und der neue Planschritt S17c setzt es auf die Operationsmaschine aus S15. Der Eingriff ist Wiederverwendung: Arbeitsfaden, Abbruchkennzeichen, Fortschrittskanal und die Sammlung übersprungener Einträge bringt die Maschine mit, und die Arbeit je Eintrag ist ohnehin schon `operation::umbenennen`. Weg 2, C4 eine Ausnahme für das Stapel-Umbenennen zu geben, ist verworfen: eine Sonderregel für eine von fünf Arten, und 525 ms stehende Oberfläche sind für den Nutzer kein Sonderfall. Der Code steht aus und hängt an S17c. Nachgezogen am 260804-2318 vom `planner`.
