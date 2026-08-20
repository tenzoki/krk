Die Abfangstelle verwirft die geforderten Sorten und leert jede gereichte Ablage

---

`Vorschautext::auswahl_ablegen` (`crates/krk-ui/src/appkit/vorschau.rs:445-462`) nimmt im
Markdown-Zweig den Parameter `sorten: &NSArray<NSPasteboardType>` entgegen und benutzt ihn
nicht. Sie ruft `zwischenablage::text_auf_ablage_schreiben`
(`crates/krk-ui/src/appkit/zwischenablage.rs:258-262`), und die ruft unbedingt
`ablage.clearContents()` und schreibt danach allein `NSPasteboardTypeString`.

Beides ist für die Zwischenablage des Nutzers richtig und war bis zur Runde 13 die einzige
Lage. Für eine Ablage, die AppKit hereinreicht — die eines Ziehvorgangs oder die eines
Dienstes —, sind es zwei offene Punkte an einer Stelle, die der Baum selbst benennt:

- **Die geforderten Sorten werden nicht beantwortet.** Der Vertrag von
  `writeSelectionToPasteboard:types:` ist, die genannten Sorten zu deklarieren und für jede
  zu schreiben, was die Ansicht hergibt. Der Markdown-Zweig deklariert eine Sorte, die in
  `sorten` gar nicht stehen muss, und beantwortet keine der genannten. Ein Abnehmer, der
  RTF verlangt, findet nichts.
- **`clearContents` nimmt weg, was AppKit dort schon abgelegt hat.** Der eigene
  Doc-Kommentar der Hülle nennt den Aufruf „keine Vorsichtsmaßnahme, sondern Bedingung" —
  begründet ist das mit der Zwischenablage des Nutzers, deren voriger Besitzer abgelöst
  werden muss. Auf einer Ziehablage verwirft derselbe Aufruf die Deklaration des
  Ziehvorgangs und erhöht ihren Änderungszähler.

---

**Was daran gemessen ist und was nicht.** Gemessen ist der Code: der Parameter wird im
Markdown-Zweig nicht gelesen, und `clearContents` steht ohne Fallunterscheidung. **Nicht**
gemessen ist, ob AppKit den Ziehvorgang und die Dienste wirklich über diese Methode führt;
das ist die Erschließung, die der Plan unter `## Risks & Mitigations` und der
Doc-Kommentar der Überschreibung als solche ausschreiben, und sie wird am laufenden Bündel
abgenommen (C2.12). Dieser Befund sagt nicht, dass die Erschließung falsch ist — er sagt,
dass die Stelle **auch dann**, wenn sie trägt, zwei Dinge tut, die für eine fremde Ablage
nicht geprüft sind.

**Der bindende Datensatz kennt den Fall.**
`shared/decisions/260819-2216_a_gilt-die-quelltextzusage-auch-fuer-das-ziehen-einer-auswahl-und-die-dienste.md`
hält für den Fehlschlag seine Möglichkeit 2 bereit („nur die Zwischenablage"). Fällt die
Bündelabnahme von C2.12 negativ aus, gehört dieser Befund in dieselbe Entscheidung; hält
sie, bleibt die Frage nach den Sorten trotzdem offen.

**Richtung, nicht vorweggenommen:** entweder die Hülle bekommt eine zweite Auskunft, ob sie
leeren soll, oder der Markdown-Zweig deklariert die gereichten Sorten selbst und schreibt
für die Textsorte den Quelltext und für die übrigen nichts. Welche der beiden, entscheidet,
was die Bündelabnahme über die fünf Wege zeigt.

**Schwere:** mittel. Kein Weg ist heute nachweislich kaputt; die Stelle ist die einzige, an
der ein Ausgabeweg außerhalb der Zwischenablage vorbeikommt, und sie behandelt ihn wie die
Zwischenablage.
**Baumstand:** `b28cdd6`.
