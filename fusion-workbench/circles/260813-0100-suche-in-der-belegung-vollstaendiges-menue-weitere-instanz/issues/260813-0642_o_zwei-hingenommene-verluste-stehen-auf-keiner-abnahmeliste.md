Zwei hingenommene Verluste stehen auf keiner Abnahmeliste, und ein Datensatz behauptet das Gegenteil

---

Die Runde 7 nimmt zwei Verluste gegenüber dem Stand vor ihr bewusst hin:

1. **`esc` im Editor bricht keine Zusammensetzung mehr ab.** Seit S3 schluckt der Abgriff den
   zulässigen und nicht mehr den ausgeführten Befehl; `abbrechen` trägt
   `Wirkungsbereich::Ueberall` und ist mit dem Fokus im Editor zulässig, liefert aber `false`.
   Der Tastendruck erreicht die `NSTextView` nicht mehr
   (`decisions/260813-0320_*_esc-im-editor-erreicht-heute-die-textflaeche-und-wird-nach-s3-geschluckt.md`).
2. **Ein Klick in die Bereichsleiste wirkt während einer Umbenennung nicht.** Seit S2 erbt der
   Mausklick den Ersthelferbestandteil der Zulässigkeitsregel
   (`issues/260813-0311_*_ein-klick-in-die-bereichsleiste-wirkt-seit-s2-waehrend-einer-umbenennung-nicht-mehr.md`).

**Keiner der beiden steht auf der Abnahmeliste für den Lauf am Bündel.** Am 260813-0642 im
Plan nachgesehen: der Abschnitt `### Die Abnahmeliste für den Lauf am Bündel`
(`planning/260813-0205_*_plan-…md`) führt zehn Zeilen, und die Wörter „Bereichsleiste",
„Zusammensetzung", „Eingabemethode" und „Vorschau-Schalter" kommen im ganzen Plan nicht vor.

**Ein Datensatz behauptet das Gegenteil.** Der Schlussabsatz von `260813-0311` sagt: „Der
Verlust steht auf der Abnahmeliste des Laufs am Bündel: ein Klick auf den Vorschau-Schalter
mitten in einer Umbenennung." Diese Zeile trifft am Baum nicht zu.

---

**Schwere:** mittel. Kein Code ist betroffen, aber der Abnahmelauf ist Nutzerarbeit, und die
Liste im Plan ist das, was der Nutzer abarbeitet. Ein Verlust, den niemand nachsieht, gilt
danach als geprüft, ohne geprüft worden zu sein — und beide sind bisher abgeleitet und nicht
gemessen.

**Der Spec-Satz, an dem beide hängen.** `## Randbedingungen` des Spec sagt zu: „Kein Verlust
gegenüber heute. Diese Runde fügt Wege hinzu und nimmt keinen weg. Wo eine neue Regel einen
heute vorhandenen Weg abschnitte, steht der Befehl auf der benannten Liste aus C2.5, oder der
Spec sagt, warum der Weg keine Wirkung hatte." Beide Verluste erfüllen keine der zwei
Bedingungen: `abbrechen` und die Bereichskommandos stehen nicht auf der Liste aus C2.5, die
genau `beenden` und `fenster_schliessen` führt, und beide Wege hatten eine Wirkung. Die
Randbedingung ist damit zweimal verletzt, und der Spec ist nicht nachgezogen worden.

**Gefunden:** reconciler, Abgleich der Runde 7 am 260813-0642

**Betroffen:** `planning/260813-0205_*_plan-…md` (Abnahmeliste),
`shared/planning/260813-0053_*_spec-…md` (`## Randbedingungen`),
`issues/260813-0311_*_…md` (Schlussabsatz)

**Domain:** code

## Zwei Wege

1. **Die Abnahmeliste nachtragen.** Zwei Zeilen im Plan: ein Klick auf den Vorschau-Schalter
   mitten in einer Umbenennung, und `esc` im Editor während einer laufenden Zusammensetzung
   einer Eingabemethode. Billig, und es macht die Zeile in `260813-0311` wahr.
2. **Die Randbedingung des Spec nachziehen.** Sie sagt heute mehr zu, als die Runde hält. Die
   zwei Verluste gehören als benannter Preis hinein, so wie der Spec es an anderen Stellen
   schon tut.

Beide zusammen sind der vollständige Zuschnitt; Weg 1 allein lässt den Spec falsch stehen.
