Dieselbe verbleibende Arbeit steht in zwei Defektdatensätzen, einer offen und einer vom Nutzer zurückgestellt, ohne Verweis aufeinander

---

Nach Turn 3 beschreiben zwei Datensätze dieselbe Restarbeit — das
Zusammenlegen zweier Schriftschnitte in `textmerkmale::anwenden` — und tragen
gegensätzliche Zustände. Keiner nennt den anderen.

---

**Die beiden Datensätze:**

1. `issues/260812-1805_o_der-ueberschneidungssatz-in-textmerkmale-anwenden-gilt-seit-markdown-rs-nicht-mehr.md`,
   **offen**. Sein Nachtrag vom 260812 schließt Punkt 1 und Punkt 3 ab und
   sagt: „**Was fehlt: Punkt 2, das Zusammenlegen der Schrift.** … Fett
   **und** kursiv oder feste Schrift **und** fett brauchten einen
   Schriftzustand je Stelle statt eines Ersetzens". Der Datensatz bleibt
   ausdrücklich offen, weil dieser Punkt fehlt.

2. `issues/260812-1851_d_zwei-schriftschnitte-legen-sich-nicht-zusammen-fett-in-kursiv-bleibt-aufrecht.md`,
   **zurückgestellt**. Er beschreibt dieselben zwei Fälle — „In
   `*kursiv **fett** wieder kursiv*` steht „fett" **aufrecht**" und
   „``**`code`**`` steht in **fester Schrift** statt fett" — und trägt:
   „**Vom Nutzer am 260812 zurückgestellt**, nachdem ihm beide Möglichkeiten
   vorgelegt worden sind."

Es ist dieselbe Ursache, dieselbe Datei, dieselben zwei gemessenen Fälle und
derselbe genannte Behebungsweg (`NSFontDescriptor` beziehungsweise
`applyFontTraits:range:`).

**Nachgeprüft:** `grep 260812-1851` im offenen Datensatz liefert nichts. Der
offene verweist nicht auf den zurückgestellten, und der zurückgestellte nennt
zwar den Probendefekt `260812-1805_o_textmerkmale-rs-traegt-keine-einzige-probe`,
nicht aber den Überschneidungsdatensatz.

**Warum das mehr ist als Buchhaltung.** `CLAUDE.md` nennt als Weg zum aktuellen
Stand ausdrücklich
`find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'`.
Dieser Lauf liefert den Überschneidungsdatensatz, und wer ihn abarbeitet,
baut Arbeit, die der Nutzer am selben Tag ausdrücklich zurückgestellt hat.
Umgekehrt liest, wer nur den zurückgestellten sieht, die Sache als erledigt
entschieden, während sie im Bestand der offenen Punkte weiterläuft.

**Zwei Zuschnitte, keiner ist hier gewählt:**

1. **Den offenen Datensatz schließen** und im Abschlussvermerk auf den
   zurückgestellten zeigen. Punkt 1 und Punkt 3 sind erledigt, Punkt 2 ist
   nicht mehr seine Sache, sondern die des Nutzerentscheids. Der Zustand
   „offen wegen eines Punktes, den der Nutzer vertagt hat" ist keiner, den die
   Marker-Vokabular kennt.
2. **Beide offen lassen und gegenseitig verweisen.** Billiger, aber die
   Suche nach offenen Punkten liefert weiterhin vertagte Arbeit.

**Gewicht:** mittel. Kein Code betroffen, aber die Datensätze sind nach
`CLAUDE.md` die bindende Grundlage jeder Planung, und hier widersprechen sich
zwei von ihnen über denselben Gegenstand.

**Herkunft:** Circle der Runde 6, Turn 3.
