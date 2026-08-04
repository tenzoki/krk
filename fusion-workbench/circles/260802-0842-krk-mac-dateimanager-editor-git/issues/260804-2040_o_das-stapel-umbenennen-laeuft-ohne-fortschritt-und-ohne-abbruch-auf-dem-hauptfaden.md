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
