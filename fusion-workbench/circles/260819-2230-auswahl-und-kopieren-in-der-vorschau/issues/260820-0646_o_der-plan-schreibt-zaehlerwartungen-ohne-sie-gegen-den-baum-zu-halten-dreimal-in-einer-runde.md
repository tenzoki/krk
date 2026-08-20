Der Plan schreibt Zählerwartungen, ohne sie gegen den Baum zu halten — dreimal in einer Runde

---

Der Plan der Runde 14 verlangt an mehreren Stellen Zählproben über `crate::quellbaum` und
schreibt ihre Erwartung gleich mit aus: „kommt im Baum nicht mehr vor", „steht genau einmal".
Drei dieser Erwartungen waren am Baum nicht erfüllbar, und zwar aus drei verschiedenen Gründen.
Jedes Mal hat der ausführende `coder` es beim Bauen gemerkt und die Erwartung an die
tatsächliche Lage angepasst.

---

**Gefilt von:** orchestrator, Sitzung `260819-2026`
**Schwere:** niedrig im Schaden, mittel in der Wiederholung. Kein falscher Code ist entstanden;
die Kosten sind ein Erkennungs- und Berichtigungsdurchgang je Fall, und das Risiko, dass ein
weniger aufmerksamer Executor die Erwartung erzwingt statt sie zu berichtigen — also den Baum
an die Probe anpasst statt die Probe an den Baum.
**Baumstand:** `1b85538`.

## Die drei Fälle

| Schritt | Erwartung im Plan | Warum sie nicht zutrifft |
|---|---|---|
| 3 | `setSelectable(false)` kommt im Baum nicht mehr vor | `appkit/belegungsansicht.rs:677` setzt es an der Meldungszeile des Belegungsblattes, einem `NSTextField`, und das soll stehen bleiben |
| 5 | `fn fokusansicht` steht genau einmal | `Anwendungsdelegierter::fokusansicht` trägt denselben Namen für die andere Hälfte derselben Frage; es sind zwei |
| 6 | `text_auf_ablage_schreiben` hat bis Schritt 7 keinen Rufer, braucht also `expect(dead_code)` | `text_schreiben` ruft sie sofort, und zwar durch die Verdrahtung, die derselbe Schritt zwei Sätze vorher verlangt |

Der dritte Fall ist keine Zählprobe, gehört aber zur selben Gestalt: eine Aussage über den
Bestand des Baums, im Plan behauptet statt am Baum gelesen.

## Was daran nicht das Problem ist

**Die Zählproben selbst sind richtig und sollen bleiben.** Dieses Projekt hält mit ihnen seine
„es gibt genau eine Stelle"-Zusagen, und sie haben in der Runde 10 einen echten Defekt
gefangen. Der Befund richtet sich allein gegen die *Erwartungszahl*, die der Plan vorwegnimmt.

**Die ausführenden Coder haben richtig gehandelt**, alle drei: Lage am Baum erhoben, Erwartung
daran angepasst, Befund belegt. Zwei haben dafür einen eigenen Datensatz abgelegt.

## Mögliche Richtungen

Nicht entschieden, hier nur festgehalten:

- Der Plan nennt die Zusage („diese Frage steht an einer Stelle") und überlässt die Zahl dem
  Schritt, der sie am Baum erhebt. Das ist die kleinste Änderung und trifft alle drei Fälle.
- Der Planner erhebt jede Zählerwartung selbst am Baum, bevor er sie ausschreibt. Genauer,
  aber die Zahl veraltet zwischen Plan und Ausführung, sobald ein früherer Schritt sie ändert —
  und genau das ist im Fall von Schritt 6 passiert.
- Die Dispatch des Orchestrators trägt den Hinweis mit, die Lage vor der Erwartung zu prüfen.
  Das ist ab Schritt 7 dieser Runde geschehen und hat gewirkt, ist aber eine Gewohnheit und
  keine Vorkehrung.
