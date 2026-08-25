Der Doppelungshinweis steht bei flight nur über einem der beiden Blöcke

---

Die sieben Zeilen des flight-Wurzelprofils stehen ein zweites Mal im Profil „Projektwurzel mit
flight-Werkbank". Der Hinweis darauf steht allein über dem zweiten Block
(`resources/default-readers.toml:721-725`). Über dem ersten (`:649-656`) steht er nicht. Wer
das Wurzelprofil bearbeitet — und das ist der Block, den man zuerst findet —, bekommt keine
Warnung, dass er dieselben Zeilen ein zweites Mal ändern muss. Bei den zwei fusion-Blöcken ist
es richtig gelöst: dort trägt jeder der beiden den Hinweis (`:267-273` und `:599-602`).

---

**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml:267-273`, `:599-602`, `:649-656`,
`:721-725`; `shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md`
(Schritt 8: „Der Preis wird im Kommentar über **beiden** Blöcken genannt und nicht
wegerklärt")

## Was gemessen ist

Gelesen am 260825-2126, Baum `8478753`. Die vier Blöcke und ihre Kommentare:

| Block | Zeilen | Doppelungshinweis |
|---|---|---|
| `fusion-Werkbank: die Wurzel` | `:274-304` | ja, `:267-273`, mit Begründung |
| `Projektwurzel mit fusion-Werkbank` | `:603-633` | ja, `:599-602`, verweist auf die Begründung oben |
| `flight-Werkbank: die Wurzel` | `:657-687` | **nein** |
| `Projektwurzel mit flight-Werkbank` | `:730-760` | ja, `:723-725` |

Textlich laufen heute weder das fusion- noch das flight-Paar auseinander: normalisiert man im
zweiten Block das vorangestellte `fusion-workbench/` beziehungsweise `flight-workbench/` weg,
sind alle vierzehn Angaben je Paar zeichengleich. Der Hinweis ist die einzige Vorkehrung, die
das hält, und für flight steht er an einer von zwei Stellen.

## Was zu tun wäre

Über dem Block `flight-Werkbank: die Wurzel` denselben Hinweis setzen, den der fusion-Block
trägt, mit Verweis auf das Profil „Projektwurzel mit flight-Werkbank" ganz unten.

**Schwere:** niedrig.
