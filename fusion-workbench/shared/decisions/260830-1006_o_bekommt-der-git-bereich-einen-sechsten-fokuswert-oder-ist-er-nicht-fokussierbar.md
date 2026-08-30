# Bekommt der Git-Bereich einen sechsten Fokuswert, oder ist er nicht fokussierbar?

---
**Domain:** code
**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md` (Frage 7); `shared/issues/260830-1006_*_fuenf-prosastellen-behaupten-eine-feldbreite-halte-den-bau-an-wenn-eine-aufzaehlung-waechst-sie-tut-es-nicht.md`; `shared/decisions/260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`; `crates/krk-ui/src/kommandos/fokus.rs:243`; `crates/krk-ui/src/appkit/bereichsleiste.rs:60-72`

---

## Question

Der Git-Bereich der Stufe A wird der sechste Wert von `Bereich`. `fokus::in_bereich` (`crates/krk-ui/src/kommandos/fokus.rs:243`) bildet jeden `Bereich` auf einen `Fokus` ab und liefert **kein** `Option`; die Fallunterscheidung ist vollständig und ohne Auffangzweig, ein sechster Wert hält dort also den Bau an und erzwingt eine Antwort. Keiner der fünf vorhandenen Fokuswerte passt: `Dateifenster` wäre falsch, `Anderswo` hieße, dass in diesem Bereich kein Befehl von KRK wirkt. Zu entscheiden ist das vor dem Plan, weil die Antwort den Umfang der Runde deutlich verschiebt und weil ein Teil der betroffenen Stellen still bleibt.

Der Bereich zeigt nach der Directive den Verlauf **als Liste**. Ob diese Liste bedienbar ist (Pfeiltasten, Auswahl, später ein Versions-Schieberegler aus Stufe B), entscheidet die Frage mit.

## Options

1. **Ein sechster Fokuswert `Fokus::Git`** — der Bereich nimmt den Ersthelferrang an wie Vorschau und Editor.
   - Pros: die Verlaufsliste ist mit der Tastatur bedienbar, was die Maxime „Steuerung über die Tastatur" verlangt; Stufe B mit dem Versions-Schieberegler braucht den Fokus ohnehin; die Antwort in `fokus::in_bereich` ist dann die naheliegende.
   - Cons: sechs Stellen sind nachzuziehen, die der Übersetzer hält (`bereich_mit_fokus`, `teilen::worauf`, `fenstertitel`, `fokusansicht`, `bereichskommando`, `tab_schliessen`), und **vier**, die er nicht hält: `Fokus::ALLE` (`fokus.rs:150`), `fokus::wirkt` (`:343`, dessen acht Zweige über `==` und `matches!` vergleichen und einen neuen Wert still in „wirkt nicht" fallen lassen), die Tafel in `fokus.rs:404` und die Tafel `OHNE_SPERRE` in `zulaessigkeit.rs:670`. Dazu ist zu entscheiden, welche der acht Wirkungsbereiche im Git-Bereich gelten: mindestens `Ueberall`, vermutlich `Navigator` und möglicherweise ein neunter Wert für die Befehle, die allein dort etwas bedeuten.
2. **Nicht fokussierbar, wie die Bereichsleiste** — jede Ansicht im Git-Bereich trägt `setRefusesFirstResponder(true)`, der Bereich bleibt reine Anzeige, bedient wird er mit der Maus.
   - Pros: kein sechster Fokuswert, keine der vier stillen Stellen; die Bauform steht im Baum vor (`appkit/bereichsleiste.rs`, Abschnitt „Kein Schalter nimmt den Ersthelferrang an"), samt ihrer Begründung; die Runde bleibt klein.
   - Cons: die Verlaufsliste ist mit der Tastatur nicht zu durchlaufen, was die Maxime schwächt; Stufe B müsste den Fokuswert dann doch nachliefern, und die vier stillen Stellen fielen dort an, also verschoben und nicht vermieden. `fokus::in_bereich` braucht trotzdem eine Antwort für den sechsten Bereich, und die wäre `Fokus::Anderswo` mit einem Doc-Kommentar, der erklärt, warum.
3. **Der Bereich nimmt den Fokus, trägt aber `Fokus::Vorschau`** — er teilt sich die Fläche ohnehin mit Vorschau und Editor.
   - Pros: kein neuer Fokuswert.
   - Cons: der Fokuswert ist die Antwort auf „wo kommen deine Tasten an", und zwei Bereiche mit derselben Antwort machen die Frage unentscheidbar; `bereich_mit_fokus` müsste raten. Das ist die Verletzung von `critical-stance.md` §4, die der Modulkopf von `fokus.rs` an drei anderen Stellen ausdrücklich vermeidet.

## Constraints

- `fokus::in_bereich` ist vollständig und ohne Auffangzweig und bleibt es; die Antwort für den sechsten Bereich ist in jedem Fall zu schreiben.
- Ein Fokuswert, der in `Fokus::ALLE` fehlt, fällt aus den Proben, ohne dass etwas rot wird; der Plan muss ihn ausdrücklich nachtragen (siehe den verwiesenen Defekt).
- Der Git-Bereich teilt sich die Fläche mit Vorschau und Editor; `Bereich::teilt_flaeche_mit` ist entsprechend zu erweitern, und die Beziehung ist symmetrisch (die Probe `der_ausschluss_ist_gegenseitig` hält es fest). Ob es künftig **drei** Bewerber um dieselbe Stelle sind oder ob der Git-Bereich eine eigene bekommt, ist Teil dieser Antwort und heute nicht entschieden.

## Recommendation

Wir empfehlen Möglichkeit 1, vorbehaltlich der Kosten, die sie ausschreibt. Möglichkeit 2 spart die vier stillen Stellen nicht, sondern verschiebt sie in die Stufe B, und sie kostet dafür die Tastaturbedienung einer Liste, in einem Programm, dessen Maxime die Tastatur ist. Möglichkeit 3 raten wir ab: sie macht die Frage, in welchem Bereich der Fokus steht, mehrdeutig.
