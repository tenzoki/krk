# Welche Sprache bestimmt die Sortierordnung der Dateinamen?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator (aufgeworfen vom coder bei der Umsetzung von 260802-1810)
**Cross-references:** decisions/260802-1810_*_sortierung-ohne-sprachsensitive-kollation.md, crates/krk-core/src/verzeichnis/kollation.rs, history/260806-1723-coder-sprachsensitive-kollation-und-endung.md

---

## Question

Die sprachsensitive Sortierung vom 260806 ordnet nach der **CLDR-Wurzel**, nicht nach einer Anpassung an eine bestimmte Sprache. Für Deutsch ist das folgenlos: die Wurzel und die Anpassung `de` liefern dieselbe Ordnung. Für andere Sprachen nicht — im Schwedischen steht `ä` hinter `z`, in der Wurzel davor.

Die Festlegung ist bisher ein Modulkommentar, kein Entscheid. Sie wurde getroffen, weil die Alternative eine Kette aufmacht: der Systemsprache zu folgen hieße, die Ordnung aus `krk-ui` in `krk-core` hereinzureichen, also die Schichtung zu berühren, die der Kern seit Schritt 2 trägt.

## Options

1. **Bei der CLDR-Wurzel bleiben.**
   - Pro: keine Schichtenänderung; für die erklärte Projektsprache Deutsch ist die Ordnung ohnehin identisch; der Sortierschlüssel bleibt einmal beim Lesen gebaut.
   - Cons: ein Nutzer mit schwedischer, dänischer oder norwegischer Systemsprache sieht eine Ordnung, die seine Sprache anders erwartet.
2. **Der Systemsprache folgen.**
   - Pro: die Sortierung entspricht dem, was der Finder in derselben Systemsprache zeigt.
   - Cons: die Ordnung kommt dann aus `krk-ui` und muss in den Kern hineingereicht werden; der Sortierschlüssel wird abhängig von einer Einstellung, die sich zur Laufzeit ändern kann, und müsste beim Wechsel neu gebaut werden.
3. **Einstellbar machen.**
   - Pro: der Nutzer entscheidet.
   - Cons: eine Einstellung mehr für einen Fall, den die Maxime supersimpel eher ausschließt; trägt dieselbe Schichtenfrage wie Möglichkeit 2.

## Constraints

- Die Zusagen L3 und L10 hängen daran, dass der Sortierschlüssel **einmal beim Lesen** entsteht und das Sortieren nur noch Bytes vergleicht. Jeder Weg, der die Ordnung zur Laufzeit wechselbar macht, muss den Schlüssel neu bauen können.
- `CLAUDE.md` erklärt Deutsch als Projektsprache. Solange das gilt, ist Möglichkeit 1 folgenlos.

## Recommendation

Möglichkeit 1 für Runde 1, und die Frage bei einer Runde wieder aufrufen, die KRK über den deutschsprachigen Gebrauch hinaus trägt. Der Grund ist derselbe wie beim Vorgängerdatensatz: eine Änderung an der Sortierung berührt die beiden gemessenen Zusagen, und ohne einen Nutzer, der die andere Ordnung braucht, gäbe es dafür keinen Gegenwert.

---
Answered:
Implemented:
Deferred:
Superseded by:
