# Trägt das linke Dateifenster einen Schalter, obwohl C7 es nie ausblenden lässt?

---
**Domain:** code
**Status:** open
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-core/src/ablage/sitzung.rs:204` (`Sichtbarkeit`), `crates/krk-ui/src/fenstermodell.rs:405` (`Fenstermodell::umschalten`), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md` (C7)

---

## Question

Der Entwurf nennt fünf Schalter für fünf Bereiche, darunter das linke Dateifenster. Das linke Dateifenster lässt sich heute nicht ausblenden, und das ist keine Lücke, sondern eine Festlegung: `Sichtbarkeit` führt bewusst kein Feld dafür, weil C7 immer mindestens ein Dateifenster stehen lässt und "ein Feld, das nie `false` werden darf, wäre eine Zusage, die niemand einhält". `Fenstermodell::umschalten(Bereich::Links)` liefert `false` und ändert nichts.

Eine Reihe von fünf gleich aussehenden Schaltern, von denen einer nie wirkt, ist eine Oberfläche, die etwas zusagt, was sie nicht hält. Die Frage entscheidet, ob die Leiste vier oder fünf Schalter trägt, und ob die Festlegung hinter dem fehlenden Feld aufgehoben wird.

## Options

1. **Vier Schalter.** Lesezeichenleiste, rechtes Dateifenster, Vorschau, Editor. Das linke Dateifenster hat keinen.
   - Pros: Nichts an `Sichtbarkeit` und nichts an C7 ändert sich. Kein Schalter lügt.
   - Cons: Die Leiste bildet die Fensterzeile nicht mehr eins zu eins ab, und der Nutzer muss wissen, warum ein Bereich fehlt. Die Asymmetrie zwischen den beiden Dateifenstern wird sichtbar, ohne dass die Leiste sie erklärt.
   - **Folgen weiter unten:** Der Aktivierungs-Spec formuliert die Fähigkeit als "vier Schalter" gegen den Wortlaut des Entwurfs.

2. **Fünf Schalter, der linke dauerhaft an und ausgegraut.** Er zeigt an, dass der Bereich steht, und nimmt keinen Klick an.
   - Pros: Die Leiste bildet alle fünf Bereiche ab. `Sichtbarkeit` und C7 bleiben unangetastet.
   - Cons: Ein Bedienelement, das kein Bedienelement ist. Der Nutzer sieht die Regel, aber nicht ihren Grund, und ein ausgegrauter Schalter erklärt nicht, dass es am *letzten* Dateifenster liegt und nicht am linken.
   - **Folgen weiter unten:** Beim Ausblenden des rechten Dateifensters müsste der linke Schalter weiter ausgegraut bleiben; blendet der Nutzer das rechte wieder ein, bliebe er es ebenfalls. Der Zustand "ausgegraut" ist damit fest und trägt keine Information.

3. **Fünf Schalter, jedes Dateifenster ausblendbar, solange eines bleibt.** Der Schalter des linken wirkt genau dann, wenn das rechte steht; der Klick auf das letzte sichtbare wird ohne Meldung verworfen, wie C7 es für den Tastenbefehl schon festlegt.
   - Pros: Alle fünf Schalter verhalten sich gleich, und die eine Ausnahme ist die, die C7 ohnehin trägt. Die Leiste bildet die Fensterzeile ab, und die Regel bleibt "mindestens ein Dateifenster" statt "das linke ist besonders".
   - Cons: Die bewusste Lücke in `Sichtbarkeit` wird gefüllt, und die Begründung dafür fällt weg.
   - **Folgen weiter unten:** `Sichtbarkeit` bekommt ein fünftes Feld; `sichtbar_in` verliert seinen festen `Bereich::Links => true`-Zweig; `umschalten` ersetzt `Bereich::Links => return false` durch dieselbe Prüfung, die heute für das rechte gilt; `Fenstermodell::aus_sitzung` braucht eine dritte hergestellte Zusicherung, damit eine von Hand geschriebene `session.toml` nicht beide Dateifenster ausblendet. Der vierte Abnahmepunkt von C7 wird dadurch **besser** prüfbar: er wird heute ausdrücklich nur am Modell nachgewiesen, weil die ausgelieferte Belegung keinen Weg dorthin kennt; mit der Leiste gibt es einen, und der Nachweis läuft am laufenden Bündel.

## Constraints

- C7 sagt zu: mindestens ein Dateifenster bleibt sichtbar, und ein Befehl, der das letzte ausblenden würde, wird ohne Fehlermeldung verworfen.
- Die Abweisung steht im Modell und nicht in der Belegungsdatei, damit eine spätere Belegung keinen ungeprüften Weg dorthin öffnet. Ein Klick in der Leiste ist ein solcher Weg und geht deshalb durch dieselbe Modellfunktion.
- War das ausgeblendete Dateifenster das aktive, wandert die Aktivität auf das andere (`umschalten` tut das heute für das rechte).

## Recommendation

**Möglichkeit 3.** Sie ist die einzige, in der alle fünf Schalter dieselbe Bedeutung tragen, und sie verschiebt die Regel dorthin, wo sie hingehört: nicht "das linke ist unantastbar", sondern "eines bleibt". Der Preis ist ein fünftes Feld in `Sichtbarkeit` samt der Zusicherung beim Laden, und dafür wird ein Abnahmekriterium der Runde 1 vom Modellnachweis auf den echten Nachweis gehoben.

---
Answered:
Implemented:
Deferred:
Superseded by:
