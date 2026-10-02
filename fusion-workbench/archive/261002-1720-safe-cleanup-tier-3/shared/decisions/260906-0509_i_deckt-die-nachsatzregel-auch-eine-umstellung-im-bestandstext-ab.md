# Deckt die Nachsatzregel auch eine Umstellung im Bestandstext ab?

---
**Domain:** Werkbankführung
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `5cb5110`
**Cross-references:** `260906-0203_*_darf-ein-agent-den-spec-oder-plan-einer-geschlossenen-runde-berichtigen.md`, `shared/issues/260819-1440_*_ein-spec-traegt-zwei-reconciliation-log-ueberschriften-und-eine-suche-findet-nur-die-erste.md`, `260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`

---

## Question

Die Nutzerentscheidung vom 260906 zu `260906-0203_*` sagt: ein Agent darf eine falsche Angabe im
Spec oder Plan einer geschlossenen Runde berichtigen, **als Nachsatz und nicht im Text**; der
ursprüngliche Wortlaut bleibt unangetastet und lesbar. Die vierte Behebungsschleife vom 260906 hat
mit dieser Regel 24 Datensätze geschlossen. Sie reicht für jede Aussage, die falsch **dasteht**.

**Einer der 36 Datensätze der Klasse S liegt anders**, und die Regel greift bei ihm nicht.
`shared/issues/260819-1440_*_ein-spec-traegt-zwei-reconciliation-log-ueberschriften-und-eine-suche-findet-nur-die-erste.md`
hält fest, dass `circles/260814-1551-…/planning/260814-1830_*_spec-…` zwei Abschnitte mit der
Überschrift `## Reconciliation Log` trägt, den ersten mit vier Einträgen aus 260814 und 260815-0246,
den zweiten mit dem Abgleich des reconciler vom 260815-1216. Wer nach der Überschrift sucht, findet
nur den ersten. **Keine Angabe ist falsch. Was fehlt, ist die Unterscheidbarkeit zweier Abschnitte,
und die stellt allein ein Eingriff in den Bestandstext her** — eine Umbenennung der zweiten
Überschrift oder das Zusammenziehen beider Abschnitte.

Ein Nachsatz kann darauf hinweisen, und die Schleife hat ihn ausdrücklich als Wegweiser gesetzt.
Er hebt den Befund aber nicht auf: die Suche findet weiterhin nur den ersten Abschnitt, und wer
den Spec maschinell nach seinem Abgleich fragt, bekommt die halbe Antwort.

## Options

1. **Die Regel deckt es nicht; der Datensatz bleibt offen.** Eine Umstellung ist kein Nachsatz, und
   die Antwort vom 260906 sagt „nicht im Text" ohne Ausnahme.
   - Pro: die Regel bleibt entscheidbar nach dem Ort der Zutat — alles Neue steht unten, nichts
     Altes wird angefasst. Genau diese Eigenschaft macht sie prüfbar.
   - Contra: ein Befund bleibt dauerhaft offen, dessen Gegenstand unstrittig und dessen Behebung
     eine Zeile ist. Und es ist genau die Sorte Rauschen, gegen die die vierte Schleife angetreten
     ist.
2. **Eine Überschrift ist Form und kein Sachtext; sie darf berichtigt werden, mit Vermerk im
   Nachsatz.** Der Wortlaut jedes Absatzes bleibt unangetastet; geändert wird allein die
   Beschriftung, unter der er steht.
   - Pro: die Aussage der Runde ändert sich nicht um ein Wort, und die Abnahmenotiz jener Runde
     behält ihren Bezug. Der Befund wird schließbar.
   - Contra: „Form" und „Sachtext" sind nicht nach dem Ort entscheidbar. Wer die Grenze einmal
     zieht, zieht sie beim nächsten Mal um einen Absatz weiter, und die Regel verliert die
     Eigenschaft, die sie tragbar macht.
3. **Die Umstellung gehört dem Nutzer oder einem Abgleichslauf, nicht einem Ausführer.** Die Regel
   bleibt wie sie ist, und der Datensatz wartet auf einen Lauf mit ausdrücklichem Auftrag.
   - Pro: trennt die Frage „darf man" von der Frage „wer"; die zweite hat der Datensatz `260906-0203`
     ausdrücklich offengelassen.
   - Contra: verschiebt die Sache nur, solange niemand einen solchen Lauf ansetzt.

## Constraints

- Die Antwort muss nach dem **Ort** entscheidbar bleiben. Das ist die Eigenschaft, die die Ortsregel
  in `CLAUDE.md` gekauft hat, und die Nachsatzregel vom 260906 erbt sie.
- Sie darf keine Runde nachträglich anders abgenommen aussehen lassen, als sie abgenommen wurde.
- Sie berührt die Frage nach dem Marker an einer Specdatei nicht
  (`260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`, offen).

## Recommendation

**Möglichkeit 1**, und zwar ohne Bedauern. Der Preis ist ein einzelner offener Datensatz mit
niedriger Schwere; der Gewinn ist eine Regel, die keine Auslegung braucht. Die vierte Schleife hat
30 von 36 Datensätzen geschlossen, weil die Regel scharf war — sie hätte keinen geschlossen, wenn
sie bei jedem Fall zu entscheiden gewesen wäre, ob eine Änderung noch Form oder schon Text ist.
Der Nachsatz, den die Schleife im Spec der Runde 10 gesetzt hat, nennt beide Abschnitte und ihre
Stelle; wer maschinell sucht, findet über ihn den Hinweis. Das ist weniger als eine Behebung und
mehr als nichts.

---
Answered: 260905-2008-orchestrator-session.md `## Fuenf weitere Entscheidungen am 260907-1301 beantwortet` — Moeglichkeit 1, die Regel deckt es nicht; der eine Datensatz ueber die doppelte Ueberschrift bleibt offen. Der Gewinn ist eine Regel, die nach dem Ort entscheidbar bleibt und keine Auslegung braucht; ruled by user, Kai Stalmann <kai@stalmann.org>.

---
Implemented: nichts zu aendern: die Regel bleibt, wie sie am 260906 entschieden wurde, und 260819-1440_*_ein-spec-traegt-zwei-reconciliation-log-ueberschriften-… bleibt offen. Das ist der beschlossene Zustand und keine ausstehende Arbeit.
