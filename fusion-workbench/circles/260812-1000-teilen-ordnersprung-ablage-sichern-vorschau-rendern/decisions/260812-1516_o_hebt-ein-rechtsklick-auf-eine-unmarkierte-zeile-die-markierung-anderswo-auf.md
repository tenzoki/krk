# Hebt ein Rechtsklick auf eine unmarkierte Zeile die Markierung anderswo auf?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator
**Cross-references:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1145_*_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md` (der Entscheid, dessen Lücke diese Frage ist); `crates/krk-ui/src/kommandos/operationen.rs` (`betroffene`, die eine Auswahlregel); `crates/krk-ui/src/appkit/tabelle.rs` (`rechtsklick_auswahl_nachziehen`)

---

## Question

Möglichkeit 2 des Datensatzes vom 260812-1145 ist gebaut: der Rechtsklick setzt die Auswahl
auf die angeklickte Zeile, es sei denn, sie ist markiert. Sie schließt den Fall nicht, für
den sie gemacht wurde, und zwar in einer Lage, die der Datensatz nicht durchgespielt hat.

Steht **anderswo** in der Liste eine Markierung, und der Nutzer klickt mit rechts auf eine
Zeile, die nicht markiert ist, dann setzt KRK zwar die Auswahl auf die angeklickte Zeile,
aber `betroffene` gibt weiterhin die **Markierung** heraus, denn die hat nach der Regel der
Runde 4 Vorrang vor der Auswahl. Das Menü zeigt damit wieder auf A und wirkt auf B, genau die
Überraschung, die Möglichkeit 2 abstellen sollte.

Der Fall ist heute nur unangenehm, weil das Menü einen einzigen Eintrag trägt und Teilen
nichts zerstört. Der Circle sagt ausdrücklich, dass das Menü in einer späteren Runde weitere
Einträge bekommt. Ein Eintrag, der löscht oder verschiebt, macht daraus einen Schaden.

## Options

1. **So lassen.** Der Vorrang der Markierung gilt für Maus und Tastatur gleich, und es gibt
   weiterhin eine Auswahlregel.
   - Pros: keine Zeile Code, keine Ausnahme, kein neuer Zustand. Wer markiert hat, weiß, dass
     er markiert hat.
   - Cons: die Lücke bleibt offen und wächst mit jedem weiteren Menüeintrag.

2. **Der Rechtsklick auf eine unmarkierte Zeile hebt die Markierung auf.** Danach gilt
   `betroffene` unverändert und liefert die angeklickte Zeile.
   - Pros: der Klick zeigt und wirkt immer auf dasselbe, ohne jede Ausnahme. Das Verhalten
     des Finders.
   - Cons: der Nutzer verliert eine Markierung durch einen Klick, der nur ein Menü öffnen
     sollte. Das ist der Kern dessen, was an Möglichkeit 3 des Vorgängerdatensatzes abgelehnt
     wurde, hier auf einen Teilfall verengt.

3. **Das Menü sagt, worauf es wirkt.** Der Eintrag heißt nicht „Teilen", sondern nennt die
   Menge, etwa „3 markierte Einträge teilen".
   - Pros: die Überraschung verschwindet, ohne dass eine Auswahl oder eine Markierung bewegt
     wird. Trägt jeden späteren Menüeintrag mit.
   - Cons: der Menütext wird zur Laufzeit gebaut und muss die Mehrzahl in zwei Sprachen
     richtig treffen. `standardShareMenuItem` liefert seinen Text von AppKit; ihn zu ersetzen
     hieße, den Systemeintrag nicht mehr zu nehmen.

## Constraints

- `kommandos::operationen::betroffene` bleibt die eine Auswahlregel. Eine zweite daneben ist
  in keiner Möglichkeit statthaft.
- Der Rechtsklick darf keine Arbeit des Nutzers zerstören, die er nicht zurückbekommt.

## Recommendation

Keine. Die drei Möglichkeiten kosten unterschiedlich und treffen unterschiedliche Erwartungen;
die Wahl gehört dem Nutzer. Die Frage hält keinen Planschritt dieser Runde auf.

---
Answered:
Implemented:
Deferred:
Superseded by:
