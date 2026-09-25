# Was heißt "proportional zur letzten Aufteilung", und was wird aus der Vorrangordnung vom 260808?

---
**Domain:** code
**Status:** implemented
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


## Antwort 260812-0306

**Moeglichkeit 1: alle sichtbaren Bereiche proportional.** Jeder Bereich traegt einen Anteil
an der Fensterzeile statt einer Punktzahl; Ein- und Ausblenden skaliert die uebrigen mit einem
Faktor. Die Mindestbreiten gewinnen weiter gegen jeden Anteil.

**Die Festlegung vom 260808 faellt, und zwar ausdruecklich.** Sie steht nirgends als
Datensatz, sondern allein im Dokumentationskommentar an `bereichsbreiten`
(`crates/krk-ui/src/fenstermodell.rs:596-602`): die Lesezeichenleiste steht in `Bereich::ALLE`
vor dem Editor, also weicht sie nicht, wenn beide zugleich stehen. Zwei Gruende, sie fallen zu
lassen:

1. **Die Directive ist juenger und ausdruecklicher.** Der Nutzer hat am 260811 diktiert: "zwei
   Bereiche, die im Verhaeltnis 2:1 zueinander standen, stehen nach dem Einblenden eines dritten
   weiterhin in diesem Verhaeltnis". Das gilt fuer Bereiche und nicht fuer Dateifenster. Eine
   benannte Ausnahme fuer die Lesezeichenleiste risse genau in dieses Beispiel ein Loch: fuer
   jedes Paar mit der Leiste gaelte es nicht.
2. ~~**Die Frage vom 260808 loest sich auf, statt ueberstimmt zu werden.** Sie lautete: wer weicht,
   wenn es eng wird? Unter einer Anteilsregel weicht niemand einzeln, sondern alle mit demselben
   Faktor. Die Frage hat unter der neuen Regel keinen Gegenstand mehr.~~

   **Zurueckgenommen am 260812-0815.** Der Abgleich hat diesen Grund widerlegt, und er hat recht.
   Die Wasserstandsrechnung nimmt einen Bereich, der unter sein Mindestmass fiele, aus der
   Verteilung heraus; danach weichen nur noch die uebrigen. Es weicht also sehr wohl jemand
   einzeln, und wer es ist, bestimmt jetzt die Mindestbreite statt der Platz in `Bereich::ALLE`.
   Die Lesezeichenleiste schrumpft beim Aufgehen des Editors, und genau dagegen hat der Nutzer am
   260808 entschieden. **Die Festlegung ist ueberstimmt worden und hat sich nicht aufgeloest.**

   Grund 1 traegt unveraendert und ist allein tragfaehig: die Directive vom 260811 spricht von
   Bereichen, und eine Ausnahme fuer die Lesezeichenleiste risse in ihr Beispiel ein Loch. Aber
   die Entscheidung ist damit eine Ueberstimmung einer Nutzerfestlegung durch einen Agenten unter
   der Weisung „mache autonom", und sie ist als solche zu lesen und nicht als Folgerung.

Moeglichkeit 3 ist abgelehnt: sie traegt zwei Regeln in einer Funktion mit einem Schwellwert
dazwischen, und das Beispiel des Nutzers gaelte gerade dann nicht, wenn er es bemerkt
(`rules/critical-stance.md` §2). Moeglichkeit 2 ist ehrlich, liefert aber die Neuaufteilung nicht,
um die es der Directive geht.

**Was daraus folgt:** `bereichsbreiten` wird neu gefasst, samt seinen Proben. `Breiten` in
`session.toml` behaelt Punktzahlen als Speicherform; die Anteile entstehen beim Lesen aus dem
Verhaeltnis der gespeicherten Punktzahlen der sichtbaren Bereiche. Damit bleibt `session.toml`
von Hand les- und schreibbar (C7) und es entsteht keine zweite Waehrung auf der Platte. Das
dritte Abnahmekriterium von C7, "beim Wiedereinblenden stellt KRK die vorherige Breite wieder
her", gilt danach als Anteil und nicht als Punktzahl; der Plan fasst es neu.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-0306` — beantwortet vom Orchestrator in der Klaerungsrunde bei der Aktivierung des Circles; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented: 5e17c9e, nachgezogen in 026c665 — `bereichsbreiten` verteilt Anteile statt Punktzahlen; die Vorrangordnung vom 260808 ist ersatzlos gefallen, samt ihrer Zusage im Dokumentationskommentar.
Deferred:
Superseded by:
