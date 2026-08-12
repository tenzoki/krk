Zwei Schriftschnitte legen sich nicht zusammen: Fettdruck in kursivem Text bleibt aufrecht

---

Die Rangordnung der Merkmale in `textmerkmale::anwenden` lässt an einer Stelle je einen
Schriftschnitt gewinnen, statt zwei zusammenzulegen. Zwei sichtbare Folgen, beide seit
`a9e1149`:

1. In `*kursiv **fett** wieder kursiv*` steht „fett" **aufrecht** statt fett-kursiv.
2. ``**`code`**`` steht in **fester Schrift** statt fett. Der Codeschnitt gewinnt gegen den
   Fettdruck.

---

Der zweite Fall ist eine Änderung gegenüber dem Stand vor `a9e1149` und stammt aus der neuen
Ordnung, die dort eingeführt wurde, um die Überschneidung von Listenbereichen aufzulösen. Der
erste Fall bestand schon vorher.

**Vom Nutzer am 260812 zurückgestellt**, nachdem ihm beide Möglichkeiten vorgelegt worden sind.
Der Grund für die Zurückstellung: das Zusammenlegen der Schriftschnitte ist eine Änderung am
AppKit-Verhalten in einer Datei, die keine einzige Probe trägt, und ohne Vordergrundlauf ist das
Ergebnis nicht zu sehen. Es kostet einen weiteren Durchgang ohne Nachweis, und beide Fälle sind
Randfälle einer Vorschau, die niemand bearbeitet.

**Der Weg, wenn es drankommt:** `NSFontDescriptor` trägt beide Merkmale zugleich
(`TraitBold | TraitItalic`), und `feste_schrift` müsste denselben Weg gehen statt einen fertigen
Schnitt zu nehmen. Der Aufwand liegt nicht im Zusammenlegen selbst, sondern darin, es zu prüfen:
`textmerkmale.rs` trägt keine Probe, und der Datensatz
`260812-1805_o_textmerkmale-rs-traegt-keine-einzige-probe.md` steht daneben offen.

**Auslöser, der die Frage wieder aufmacht:** ein Abnahmelauf am laufenden Bündel, bei dem der
Nutzer eine Markdown-Datei mit verschachtelter Auszeichnung vor sich hat und entscheidet, ob es
ihn stört.

---
Deferred: bis zum nächsten Abnahmelauf am Bündel — Nutzerentscheid vom 260812. Zwei Randfälle einer nicht bearbeitbaren Vorschau, deren Behebung ohne Vordergrundlauf nicht nachweisbar wäre.
