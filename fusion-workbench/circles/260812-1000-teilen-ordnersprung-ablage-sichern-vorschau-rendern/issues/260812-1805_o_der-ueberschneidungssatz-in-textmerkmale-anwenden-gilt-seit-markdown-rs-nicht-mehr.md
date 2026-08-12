Der Überschneidungssatz in `textmerkmale::anwenden` gilt seit `markdown.rs` nicht mehr, und vier Auszeichnungen setzen jetzt dieselbe Schrift

---

Der SAFETY-Kommentar in `crates/krk-ui/src/appkit/textmerkmale.rs:210-227`
sagt zu: „`Ueberschrift` und `FesteSchrift` setzen beide die Schrift und
überlappen einander deshalb nie: die Fallunterscheidung in
`crate::hervorhebung` fragt die Überschriftsstufe zuerst und die feste Schrift
nur sonst." Der Satz ist seit Planschritt 8 falsch. `crate::markdown` ist ein
**zweiter** Erzeuger von `Formatierung`, und er liefert genau diese
Überschneidung — die eigene Probe `die_auszeichnungen_stehen_von_aussen_nach_innen`
(`markdown.rs:722`) schreibt sie sogar fest.

---

**Was sich geändert hat.** `Auszeichnung` ist von drei auf fünf Werte
gewachsen, und **vier** von ihnen setzen jetzt denselben Merkmalsnamen
`NSFontAttributeName` (`textmerkmale.rs:197-208`): `Ueberschrift`,
`FesteSchrift`, `Betonung`, `StarkeBetonung`. Nur `Listenzeile` setzt einen
anderen. Der Kommentar begründet die Unbedenklichkeit der Überschneidungen
damit, dass die beiden überlappenden Auszeichnungen „verschiedene
Merkmalsnamen — Schrift gegen Absatzstil" setzten. Bei vier von fünf Werten
trifft das nicht mehr zu, und `addAttributes:range:` legt bei gleichem Namen
nicht zusammen, sondern ersetzt.

**Die Ersatzregel steht in `markdown.rs` und trägt nicht überall.**
`Zerlegung::abschliessen` (`markdown.rs:663-681`) sortiert nach Anfang und bei
gleichem Anfang das **längere** zuerst, damit außen vor innen steht und innen
das innere gewinnt. Diese Regel bricht, sobald außen und innen **dieselbe
Länge** haben: dann entscheidet die stabile Sortierung nach Einfügereihenfolge,
und die läuft von innen nach außen.

Gemessen mit `markdown::rendern` (unverändert in ein Prüfprogramm kopiert):

```
Quelle : "# `Code` im Titel"
Reihenfolge: Ueberschrift{1}(0,13), FesteSchrift(0,4)
  -> innen gewinnt: der Quelltext steht in fester Schrift. Wie beabsichtigt.

Quelle : "**`code`**"
Reihenfolge: FesteSchrift(0,4), StarkeBetonung(0,4)
  -> die StarkeBetonung gewinnt: `code` steht fett in der Systemschrift und
     hat seine feste Schrift verloren.

Quelle : "*kursiv **fett** wieder kursiv*"
Reihenfolge: Betonung(0,25), StarkeBetonung(7,4)
  -> "fett" ist fett und nicht mehr kursiv.
```

Der zweite Fall ist die Umkehrung des ersten, und beide entstehen aus derselben
Sortierung. Der dritte ist die allgemeine Form: wo zwei schriftsetzende
Auszeichnungen einander enthalten, geht die äußere für den überlappten Bereich
vollständig verloren, statt sich mit der inneren zu verbinden.

**Warum der Kommentar wiegt und nicht nur der Effekt.** `CLAUDE.md` führt unter
„Was man nicht sieht" einen Fall, in dem genau eine solche Zusicherung im Code
eine ganze Sitzung und einen Fehlbefund gekostet hat. Eine SAFETY-Begründung,
die eine Unmöglichkeit behauptet, die eintritt, ist die gefährliche Sorte
veralteter Kommentar: der nächste Leser prüft die Bedingung nicht nach, weil
die Datei sie ihm zusagt.

**Was zu tun ist**, in dieser Reihenfolge:

1. Den SAFETY-Kommentar berichtigen: vier Auszeichnungen setzen die Schrift,
   sie überlappen einander, und was gilt, entscheidet die Reihenfolge der
   Liste.
2. Entscheiden, ob die Reihenfolge genügt oder ob die Schrift zusammengelegt
   werden soll (fett **und** kursiv, feste Schrift **und** fett — über
   `NSFontDescriptor`-Merkmale statt über ein Ersetzen). Der erste Weg ist eine
   Zeile Kommentar, der zweite ein Umbau von `anwenden` auf einen
   Schriftzustand je Stelle.
3. Die Reihenfolge bei gleicher Länge festlegen, statt sie der stabilen
   Sortierung zu überlassen; heute ist sie ein Nebenprodukt.

**Gewicht:** mittel — die falsche Zusicherung im Code wiegt schwerer als die
Anzeige, die nur Auszeichnungen betrifft, die einander enthalten.

**Herkunft:** Circle der Runde 6, Planschritte 7 und 8.
