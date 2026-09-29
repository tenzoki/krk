# Blättert Cmd+Pfeil hoch im angezeigten Jahres- oder Monatsordner ohne ausgewählte Zeile, oder steigt es auf?

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md` (C1, zweite Entscheidung; C3.4; C4.6); `260825-1725_*_was-zeigt-die-vorschau-wenn-keine-zeile-ausgewaehlt-ist.md` (die Regel, aus der die Lage folgt); `260929-1646_*_lage-bildfolge-fragt-allein-den-vorschauinhalt-und-lenkt-cmd-up-und-return-nach-fenster-und-auswahlwechsel-auf-die-alte-folge.md` (dieselbe Stelle, `Lage::bildfolge`)

---

## Question

Wer mit Pfeil rechts in `Fotos/2008` hineingeht, steht dort ohne ausgewählte Zeile: `in_zeile_einsteigen` ruft `ordner_lesen(&ziel, None)` (`crates/krk-ui/src/appkit/tabelle.rs`). Nach der Regel vom 260825 beschreibt die Vorschau dann den angezeigten Ordner (`zu_beschreiben`, dieselbe Datei), das Jahresprofil trifft `…/Fotos/2008`, und die Vorschau zeigt die Jahresfolge. `Lage::bildfolge` ist damit wahr, und **Cmd+Pfeil hoch blättert, statt nach `Fotos` aufzusteigen**. Auf dem ersten Foto tut es gar nichts (C3.3). Return springt in den ersten Monatsordner. Dasselbe gilt in `Fotos/2008/08` ohne ausgewählte Zeile. Der Spec deckt das über die zweite Entscheidung unter C1 („Ohne ausgewählte Zeile gilt wie heute der angezeigte Ordner“), nennt die Folge für Cmd+Pfeil hoch aber nicht. `HowTo.md` sagt „Sie gilt für den ausgewählten Ordner“ (Zeile 665) und „Überall sonst gilt die gewohnte Bedeutung“ (Zeile 704); den Fall ohne Auswahl erwähnt es nicht. Zu entscheiden ist es jetzt, weil der Fix des verlinkten Defekts dieselbe Frage an `Lage::bildfolge` neu schreibt.

## Options

1. **So lassen und dokumentieren.** Die Folge des angezeigten Ordners gilt auch für die Tasten; `HowTo.md` nennt den Fall und dass Pfeil links dort aufsteigt.
   - Pros: kein Code; eine Regel für Vorschau und Tasten; deckt den Wortlaut der Directive („innerhalb eines Jahresordners“).
   - Cons: gleich nach dem Hineingehen tut Cmd+Pfeil hoch nichts Sichtbares. Wer mit Cmd+Pfeil hoch und runter durch Ordner navigiert, hängt in jedem Jahres- und Monatsordner fest, bis er eine Zeile wählt.
2. **Die Vorschau zeigt die Folge, die Tasten gelten nur bei ausgewählter Zeile.** `Lage::bildfolge` verlangt zusätzlich, dass die Vorschau einen ausgewählten Eintrag beschreibt und nicht den angezeigten Ordner.
   - Pros: Cmd+Pfeil hoch und Return behalten im Ordner selbst ihren alten Sinn; das deckt sich mit dem Wortlaut „ausgewählter Ordner“ in `HowTo.md` und C3.
   - Cons: eine sichtbare Folge, durch die man ohne Auswahl nicht blättern kann; widerspricht dem Wortlaut der Directive („innerhalb eines Jahresordners“); die Lage braucht die Auskunft „Auswahl oder Ordner“ aus der Dateiliste, die der Fix des verlinkten Defekts ohnehin erheben muss.
3. **Keine Bildfolge für den angezeigten Ordner.** Ohne ausgewählte Zeile zeigt ein Fotoordner die Default-Zeilen.
   - Pros: Vorschau und Tasten sagen dasselbe; keine Folge, die nicht bedienbar ist.
   - Cons: weicht von der Regel vom 260825 ab („ohne Ausnahme und für jeden Ordner“, Kommentar an `zu_beschreiben`) und braucht die Unterscheidung im Kern oder beim Rufer von `zusammenfassen`.

## Constraints

- Pfeil links führt in jeder Lage aufwärts (C3.4 zweiter Satz); das bleibt in jeder Möglichkeit.
- Die Frage gehört an die eine Stelle `Lage::bildfolge` und nicht in eine zweite Regel neben `folge_passt`.

## Recommendation

Möglichkeit 1, mit dem Satz in `HowTo.md`. Die Directive des Arbeitspakets sagt wörtlich „Wenn man **innerhalb** eines Jahresordners ist (Fotos/2008/) … Mit CMD-Pfeil-{hoch/runter} soll dann durch die Bilder navigiert werden“; der Fall ohne Auswahl ist genau dieser Wortlaut, und Möglichkeit 2 nähme ihn zurück. Der Preis ist der Tastenweg „hineingehen, Cmd+Pfeil hoch“, der dort nichts Sichtbares tut; ihn soll der Nutzer kennen, bevor er die Frage schließt. Wählt er Möglichkeit 2, fällt sie mit dem Fix des verlinkten Defekts zusammen, weil beide verlangen, dass die Lage weiß, was das aktive Dateifenster gerade beschreibt. **inference:** Wie oft Cmd+Pfeil hoch im Ordner selbst zum Aufsteigen gedrückt wird, ist aus dem Tastenweg geschlossen und nicht gemessen.

---
Answered: 260929-1646-reviewer-bildfolge-und-nachtraege.md `Offene Frage` — Möglichkeit 1: ohne ausgewählte Zeile zeigt die Vorschau die Folge des angezeigten Ordners, cmd+up blättert dort und steigt nicht auf; HowTo.md beschreibt es; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: HowTo.md, Abschnitt Bildfolge, beschreibt das Blaettern ohne ausgewaehlte Zeile (der Commit traegt es) — HowTo.md beschreibt das Blättern ohne ausgewählte Zeile
