# Meint die gemeldete Eintragszahl aus C4 die angefassten Einträge oder die ausgewählten Positionen mal ihre Inhalte?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1649_c_die-gemeldete-eintragszahl-bedeutet-beim-verschieben-etwas-anderes-als-beim-kopieren.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C4), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (`### Frage 6`), `crates/krk-core/src/operation/fortschritt.rs`

---

## Question

C4 sagt: "Nach einem Abbruch nennt KRK, wie viele Einträge bereits übertragen wurden." Ein Ordner mit 500 Einträgen meldet kopiert 501 und innerhalb eines Datenträgers verschoben 1. Beide Zahlen sind für sich richtig — das Kopieren steigt in den Baum ab und fasst jeden Eintrag an, das Verschieben ist ein einziger `rename(2)`, der den Inhalt nie berührt und genau deshalb schnell ist — aber C4 verspricht dem Nutzer nur eine.

## Options

1. **Angefasste Einträge**, die Lesart, die die Umsetzung von S15 gewählt hat.
   - Pros: kein Vorabdurchlauf nötig; die Zahl beschreibt, was die Operation getan hat.
   - Cons: dieselbe Handlung liefert je nach Datenträger eine andere Zahl; "3 Einträge übertragen" nach dem Abbruch beim dritten von zehn Ordnern ist nicht die Auskunft, die der Nutzer sucht.
2. **Erledigte Positionen der Auswahl mal ihre Inhalte.**
   - Pros: über alle Fälle hinweg dieselbe Bedeutung.
   - Cons: verlangt einen Vorabdurchlauf über den Ordnerbaum, den `### Frage 6` des Plans ausschließt, weil er die 200 ms von L8 selbst aufbrauchen kann; sagt bei einem einzigen großen Ordner nichts aus.
3. **Beide zeigen**, etwa "3 von 10 Positionen, 4.812 Einträge".
   - Pros: die vollständige Auskunft.
   - Cons: dieselbe Voraussetzung wie Möglichkeit 2 für die zweite Zahl.

## Constraints

- `### Frage 6` des Plans schließt den Vorabdurchlauf über den Ordnerbaum aus; die Fortschrittsanzeige hängt an einer Laufzeit von 150 ms und nicht an einem vorher bekannten Umfang.
- Die Umsetzung von S15 zählt bereits nach Möglichkeit 1, festgehalten am Feld `Bericht::eintraege` in `crates/krk-core/src/operation/fortschritt.rs`.

## Recommendation

Möglichkeit 1, ausgeschrieben in C4.

---
Answered: Nutzer am 260805-0000 — Möglichkeit 1. Begründung des Nutzers: die andere Lesart verlangt den Vorabdurchlauf, den `### Frage 6` ausschließt.

Eingearbeitet: `planning/260802-1036_o_spec-navigator-geruest.md` C4, im Abnahmekriterium zum Fortschritt ("wie viele Einträge die Operation bis dahin **angefasst** hat") und als eigene Festlegung, die den Preis benennt: dieselbe Handlung liefert je nach Datenträger eine andere Zahl, weil ein Verschieben über eine Datenträgergrenze absteigen muss. Kein neuer Schritt.
Implemented: `crates/krk-core/src/operation/fortschritt.rs`, Feld `Bericht::eintraege` — die Umsetzung von S15 zählt bereits so; die Antwort schreibt die vorhandene Lesart fest und ändert keinen Code.
