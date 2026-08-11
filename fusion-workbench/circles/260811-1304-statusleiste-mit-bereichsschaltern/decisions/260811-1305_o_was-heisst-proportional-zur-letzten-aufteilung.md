# Was heißt "proportional zur letzten Aufteilung", und was wird aus der Vorrangordnung vom 260808?

---
**Domain:** code
**Status:** open
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/_a_circle.md` (Abschnitt `## Grounding snapshot`), `crates/krk-ui/src/fenstermodell.rs:609` (`bereichsbreiten`), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md` (C7)

---

## Question

Der Entwurf verlangt, dass jede Änderung der Sichtbarkeit die Bereiche **proportional zur letzten Aufteilung** neu verteilt: zwei Bereiche im Verhältnis 2:1 stehen nach dem Einblenden eines dritten weiterhin in diesem Verhältnis. Die heutige Regel tut das nicht. `bereichsbreiten` gibt den festen Bereichen der Reihe nach ihre gespeicherte Breite **in Punkten** und verteilt allein den Rest an die beiden Dateifenster, dort im Verhältnis ihrer Breiten. Ein Verhältnis gilt heute also zwischen zwei der fünf Bereiche und sonst nirgends.

Die Reihenfolge, in der die festen Bereiche bedient werden, ist dabei keine Nebensache. Der Dokumentationskommentar an `bereichsbreiten` führt sie als Zusage: die Lesezeichenleiste steht in `Bereich::ALLE` vor dem Editor, also behält sie ihre Wunschbreite, wenn beide zugleich stehen, und die Dateifenster rücken zusammen. Das ist eine Festlegung des Nutzers vom 260808. Eine durchgehend proportionale Regel hebt sie auf, weil dann alle sichtbaren Bereiche mit demselben Faktor schrumpfen.

Die Frage muss vor dem Plan beantwortet sein: sie entscheidet, ob die eine Breitenregel des Programms neu geschrieben wird oder unverändert bleibt.

## Options

1. **Alle sichtbaren Bereiche proportional.** Jeder Bereich trägt einen Anteil an der Fensterzeile statt einer Punktzahl. Ein- und Ausblenden skaliert alle übrigen mit einem Faktor; die Mindestbreiten gewinnen weiterhin gegen den Anteil.
   - Pros: Das ist die wörtliche Lesart des Entwurfs, und sie gilt dann für alle fünf Bereiche gleich. Eine Fallunterscheidung zwischen festen und beweglichen Bereichen entfällt in der Verteilung.
   - Cons: Die Festlegung vom 260808 fällt. Die Lesezeichenleiste schrumpft künftig, wenn der Editor aufgeht, und genau dagegen hat der Nutzer damals entschieden.
   - **Folgen weiter unten:** `bereichsbreiten` wird neu geschrieben, samt seiner zwölf Proben. Das dritte Abnahmekriterium von C7, "beim Wiedereinblenden stellt KRK die vorherige Breite wieder her", gilt danach nicht mehr wörtlich: wiederhergestellt wird der **Anteil**, und die Punktzahl folgt daraus nur, wenn die übrige Aufteilung dieselbe ist. Das Kriterium ist im Aktivierungs-Spec neu zu fassen. `Breiten` in `session.toml` trägt Punktzahlen; ob es künftig Anteile trägt oder ob die Anteile beim Lesen aus den Punktzahlen entstehen, wird eine Planfrage.

2. **Nur die beweglichen Bereiche proportional, also die heutige Regel.** Die drei festen Bereiche behalten ihre Punktzahl, die zwei Dateifenster teilen den Rest im Verhältnis.
   - Pros: Nichts an der Breitenregel ist zu bauen. Die Festlegung vom 260808 bleibt, und die Vorrangordnung bleibt lesbar an einer Stelle.
   - Cons: Das Beispiel des Entwurfs gilt dann nur für die beiden Dateifenster. Blendet der Nutzer die Vorschau ein, ändert sich das Verhältnis zwischen Lesezeichenleiste und Dateifenstern, weil die Leiste ihre Punktzahl behält.
   - **Folgen weiter unten:** Die Runde schrumpft auf die Leiste mit ihren Schaltern und den Vorschaubreiten-Defekt. Der Nutzer bekommt die Schalter, aber nicht die Neuaufteilung, die er beschrieben hat.

3. **Vorrang bis es eng wird, danach proportional.** Ein fester Bereich behält seine gespeicherte Punktzahl, solange sie hineinpasst. Reicht der Platz nicht, wird der Fehlbetrag von allen sichtbaren Bereichen anteilig genommen statt allein von den Dateifenstern.
   - Pros: Die Festlegung vom 260808 gilt im gewöhnlichen Fall weiter, und der enge Fall verteilt die Last gleichmäßig statt sie den Dateifenstern aufzuladen.
   - Cons: Zwei Regeln in einer Funktion, und die Grenze zwischen ihnen ist ein Schwellwert. Das Beispiel des Entwurfs gilt nur unterhalb der Schwelle, also gerade dann nicht, wenn der Nutzer es bemerkt.
   - **Folgen weiter unten:** Es entsteht eine Fallunterscheidung, deren beide Zweige der Nutzer auseinanderhalten muss, um die Oberfläche vorherzusagen. `critical-stance.md` §2 nennt genau das als Merkmal einer falsch geschnittenen Regel.

## Constraints

- Die Breitenregel steht **einmal**, in `fenstermodell::bereichsbreiten`, und bleibt reines Rust ohne AppKit. Eine zweite Rechenvorschrift daneben ist ausgeschlossen; `aufteilung.rs` setzt nur um.
- Die Mindestbreiten aus `Bereich::mindestbreite` gewinnen gegen jedes Verhältnis.
- Breiten und Sichtbarkeit überleben Beenden und Neustart (C7).

## Recommendation

**Möglichkeit 1**, aber nur zusammen mit einer ausdrücklichen Neuentscheidung über die Festlegung vom 260808. Möglichkeit 3 ist der Versuch, beides zu behalten, und er kostet die Vorhersagbarkeit: der Nutzer müsste wissen, auf welcher Seite einer Schwelle er gerade steht. Möglichkeit 2 ist ehrlich, liefert aber nicht, was der Entwurf beschreibt.

Wer Möglichkeit 1 wählt, beantwortet mit: soll die Lesezeichenleiste beim Aufgehen des Editors mitschrumpfen, oder soll sie ihre Breite als einzige behalten? Die zweite Antwort wäre eine benannte Ausnahme von der proportionalen Regel und keine zweite Regel daneben.

---
Answered:
Implemented:
Deferred:
Superseded by:
