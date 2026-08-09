# Was verwirft „Verwerfen", wenn die Vorschau den Editor nur verdrängt?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` (C4, sechstes Abnahmekriterium und die Festlegung „Zwei Anlässe sind hinzugekommen"), `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` §S28, `circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260809-2029_c_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`, `crates/krk-ui/src/appkit/anwendung.rs` (`anlass_ausfuehren`, `vorschau_verdraengt_den_editor`)

---

## Question

C4 nennt vier Anlässe für die Nachfrage. Drei von ihnen verlieren den ungesicherten Stand wirklich: das Schließen des Editors gibt seine Datei frei, der Wechsel auf eine andere Datei ersetzt sie, das Beenden nimmt den ganzen Prozess mit. Der vierte, das Verdrängen durch die eingeblendete Vorschau, verliert nichts.

Der Spec nimmt das Gegenteil an. Seine Begründung für den Anlass lautet, das Verdrängen verliere „denselben Stand wie das Schließen". Am gebauten Code stimmt das nicht: ein Wechsel der Sichtbarkeit setzt `hidden` an den Ansichten und fasst das `Editormodell` nicht an. Der geschlossene Defekt vom 260809-2029 hat genau daran gehangen — der Nutzer blendete die Vorschau ein, holte den Editor mit F4 zurück, und sein Stand war noch da; verloren ging er erst durch das zweite Lesen, das seither unterbleibt.

Damit ist die dritte Wahlmöglichkeit an diesem einen Anlass ohne Gegenstand. Der Nutzer wählt „Verwerfen", und es wird nichts verworfen: der Editor verschwindet vom Schirm, sein Stand steht weiter im Modell, und beim nächsten `shift+cmd+e` trägt der Kopf sein Abweichungszeichen unverändert. Das ist kein Fehler im Sinne eines Datenverlusts, aber ein Wort, das etwas anderes sagt als es tut.

Gebaut ist der Weg, den der Plan vorschreibt: die Fortsetzung blendet die Vorschau ein, mehr nicht. Das ist die sichere von zwei Lesarten — sie kann nichts verlieren.

## Options

1. **So lassen und die Beschriftung des Blattes am Anlass ändern** — der Editor behält seinen Stand, und die drei Schaltflächen heißen bei diesem einen Anlass anders, etwa „Sichern / Ohne Sichern / Abbrechen".
   - Pro: kein Verlust auf einem Weg, der keinen erzwingt. Der Nutzer bekommt seinen Stand beim nächsten Blick in den Editor unverändert wieder.
   - Contra: ein Blatt mit zwei Beschriftungssätzen ist eine Fallunterscheidung in einer Fläche, die heute keine trägt, und die Erläuterung mit den drei Tastenwegen müsste mitziehen.

2. **Den Anlass fallen lassen** — die eingeblendete Vorschau fragt nichts, weil sie nichts verliert. Aus vier Anlässen werden drei.
   - Pro: die Nachfrage steht dann genau dort, wo etwas auf dem Spiel steht, und jede Antwort bedeutet, was sie sagt. Eine Fallunterscheidung weniger.
   - Contra: widerspricht dem sechsten Abnahmekriterium von C4 im Wortlaut. Der Nutzer sieht das Abweichungszeichen nicht mehr, solange der Editor verdrängt ist, und könnte den ungesicherten Stand vergessen — genau davor schützt eine Nachfrage, auch wenn sie nichts rettet.

3. **„Verwerfen" beim Wert nehmen und die Datei freigeben** — der Anlass verhält sich wie das Schließen: „Verwerfen" gibt die Datei auf, der Stand fällt.
   - Pro: das Wort stimmt wieder, und alle vier Anlässe verhalten sich gleich.
   - Contra: `cmd+y` und eine unbedachte Antwort löschen die getippte Arbeit. Der Befehl heißt „Vorschau umschalten" und wäre der einzige, der eine Datei aufgibt; das ist die gefährlichste der drei Lesarten.

## Constraints

- Die Festlegung des Nutzers vom 260807-2139 nennt „sichern, verwerfen, abbrechen" als die drei Wahlmöglichkeiten der Nachfrage. Eine Antwort, die das Verwerfen an einem Anlass streicht, muss das ausdrücklich sagen.
- Das sechste Abnahmekriterium von C4 verlangt die Nachfrage für diesen Anlass im Wortlaut. Möglichkeit 2 ändert den Spec und nicht nur den Code.
- Der Anlass trägt seit dem 260810 zwei Befehle, `f3`/`cmd+y` und `shift+cmd+y`. Jede Antwort gilt für beide.
- Gebaut ist heute Möglichkeit 1 ohne den Beschriftungsteil: gefragt wird, und „Verwerfen" verwirft nichts.

## Recommendation

Möglichkeit 2, mit einer Änderung am Spec statt einer am Blatt. Der Anlass ist aus einer Annahme über den Code entstanden, die der Code nicht trägt, und eine Nachfrage, deren drei Antworten auf zwei verschiedene Handlungen hinauslaufen, lehrt den Nutzer, Blätter wegzuklicken. Der genannte Gegenwert — der Nutzer vergisst den ungesicherten Stand nicht — ist ohne Nachfrage zu haben, sobald der Editor sein Abweichungszeichen auch im verdrängten Zustand sichtbar macht oder das Beenden ihn ohnehin abfragt, und das tut es seit S29.

Möglichkeit 3 empfehlen wir ausdrücklich nicht: sie macht aus einem Umschaltbefehl einen, der Arbeit löscht.

Die Empfehlung ist eine Auslegung und keine geprüfte Aussage über die Absicht des Nutzers; sie ist ihm vorzulegen. Bis dahin bleibt der gebaute Zustand stehen, weil er nichts verlieren kann.

---
Answered:
Implemented:
Deferred:
Superseded by:
