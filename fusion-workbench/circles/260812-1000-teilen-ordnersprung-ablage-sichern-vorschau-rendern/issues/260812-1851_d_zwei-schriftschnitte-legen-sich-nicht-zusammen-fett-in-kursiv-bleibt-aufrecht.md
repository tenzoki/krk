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

---

Also seen: 260908 by coder — **ein dritter Fall derselben Ursache, den dieser Datensatz nicht
nennt.** Eine Auszeichnung **in einer Überschrift** verliert deren Schriftgröße:
`## Ein **fetter** Teil` zeigt „fetter" in der Grundgröße, bei Stufe 1 sind das 41 Prozent
Höhe gegenüber den Nachbarn in derselben Zeile. Der Fall ist als
`260812-1920_*_eine-auszeichnung-in-einer-ueberschrift-verliert-deren-schriftgroesse.md`
gemessen abgelegt.

**Für die zurückgestellte Frage ist daran eines wichtig:** der hier genannte Behebungsweg —
`NSFontDescriptor`-Merkmale beziehungsweise `applyFontTraits:range:` — löst ihn **nicht**. Die
legen Schnitte zusammen und keine Größen. Der Größenverlust entsteht dadurch, dass der innere
Eintrag eine ganz neue Schrift setzt; ein Zustand je Stelle müsste deshalb die Größe
mitführen. Wer die Frage später aufnimmt, rechnet mit drei Fällen und mit einem Zustand, der
Schnitt **und** Größe trägt.
