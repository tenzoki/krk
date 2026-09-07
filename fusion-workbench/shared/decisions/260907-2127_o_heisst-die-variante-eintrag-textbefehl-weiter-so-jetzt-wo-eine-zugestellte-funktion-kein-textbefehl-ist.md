# Heißt die Variante `Eintrag::Textbefehl` weiter so, jetzt wo eine zugestellte Funktion kein Textbefehl ist?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:**
`260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`
(der Entscheid, der `filter_einfuegen` hervorgebracht hat);
`260907-2020_*_das-einfuegen-in-den-filtertext-braucht-eine-siebte-vom-menue-zugestellte-funktion-in-der-belegung.md`
(der Befund zur Belegungszeile);
`260907-2127-k15b-der-code-zur-belegungszeile-cmd-f.md` (der Durchgang)

---

## Question

`Eintrag::Textbefehl` (`crates/krk-ui/src/menuemodell.rs`) ist die Variante für
jeden Menüeintrag, der statt eines `Kommando` einen Selektor trägt und über die
Antwortkette läuft. Sie hieß so, weil sie bis zum 260907 genau die sechs
Textbefehle des Menüs „Bearbeiten" trug. Seit dem 260907 trägt sie zusätzlich
`filter_einfuegen`, und das ist in zwei Hinsichten kein Textbefehl: es steht im
Obermenü „Dateilisting" und nicht in „Bearbeiten", und sein Selektor
`filterEinfuegen:` kommt von KRK und nicht von AppKit. Es füllt einen Filter und
rührt keinen Text an.

Dasselbe trifft `Funktionsbereich::Textbefehle` nicht: dieser Bereich trägt
weiter genau die sechs, und die Zuordnung in `belegungsmodell::bereich` schickt
`filter_einfuegen` bewusst woandershin. Betroffen ist allein die Variante des
Menümodells.

Die Frage ist jetzt zu stellen, weil sie mit jeder weiteren zugestellten
Funktion außerhalb von „Bearbeiten" teurer und zugleich dringender wird, und
weil der Doc-Kommentar heute die Lücke zwischen Name und Gegenstand
ausschreiben muss, statt sie zu schließen.

## Options

1. **Der Name bleibt, der Doc-Kommentar trägt die Erklärung** (der heutige
   Stand). Die Variante benennt die **Zustellung** und nicht den Gegenstand:
   „was das Menü zustellt", und Textbefehle waren nur die ersten sechs Fälle
   davon.
   - Pro: kein Umbau; die Zeile im Doc-Kommentar sagt es an einer Stelle.
   - Contra: der Name sagt weniger, als er verspricht, und die Projektregel
     „Namen spiegeln die Absicht" ist damit an dieser Stelle gebrochen. Wer die
     Variante liest, ohne den Doc-Kommentar zu lesen, sucht `filter_einfuegen`
     unter den Textbefehlen und findet es nicht.
2. **Umbenennen in `Eintrag::Zugestellt`.** Der Name benennt genau das
   Unterscheidungsmerkmal, das die Variante trägt, und deckt sich mit
   `ZUSTELLER`, `zusteller()` und `zugestellte_kuerzel` daneben.
   - Pro: ein Wort statt eines Absatzes; die vier Namen dieser Datei lesen sich
     danach als eine Familie.
   - Contra: eine mechanische Umbenennung über die Rufer und die Proben; die
     Variante ist `pub` und steht in Doc-Kommentaren mehrerer Module.
3. **Umbenennen in `Eintrag::UeberDieAntwortkette`** oder einen Namen, der den
   Ausführungsweg benennt statt den Zusteller.
   - Pro: sagt, was mit dem Eintrag zur Laufzeit geschieht, und das ist die
     Eigenschaft, an der `eintrag()` die Fallunterscheidung trifft.
   - Contra: länger, und der Ausführungsweg ist die Folge der Zustellung und
     nicht ihr Grund; zwei Namen für dieselbe Sache stünden nebeneinander.

## Constraints

Der Wert selbst bleibt, wie er ist: die Aufzählung ist vollständig und ohne
Auffangzweig, und eine vierte Sorte Eintrag steht nicht zur Debatte. Was hier
zur Wahl steht, ist allein der Bezeichner. `Funktionsbereich::Textbefehle`
bleibt in jedem Fall unberührt.

## Recommendation

Möglichkeit 2. Die Datei führt schon drei Namen aus derselben Wurzel
(`ZUSTELLER`, `zusteller`, `zugestellte_kuerzel`), und die Variante ist die
einzige Stelle, die aus der Reihe fällt. Der Umbau ist mechanisch und einmalig,
während die Erklärung in Möglichkeit 1 mit jeder weiteren zugestellten Funktion
außerhalb von „Bearbeiten" erneut gelesen werden muss.
