# Wandern die Bereichsbreiten von Ctrl+B und Ctrl+S auf Ctrl+Links und Ctrl+Rechts?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-2045_c_die-kombinationsschreibweise-kennt-die-links-und-rechts-pfeile-nicht.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Schritte S11b und S11c), `resources/default-keymap.toml`

---

## Question

C7 verlangt, dass sich die vier Bereiche der Fensterzeile "über einen Tastenbefehl schrittweise verbreitern und verschmälern" lassen. Die Bereiche stehen nebeneinander, und die Richtung dafür sind die Links- und Rechts-Pfeile. Ausgeliefert liegen die beiden Befehle stattdessen auf `ctrl+b` und `ctrl+s`, den Anfangsbuchstaben von "breiter" und "schmaler". Der Grund war eine Lücke der Kombinationsschreibweise: sie kannte die beiden Pfeile nicht, und der Kommentar an dieser Stelle in `resources/default-keymap.toml` sagt das auch so.

Seit dem 260804-1122 ist der Grund weg. Der Nutzer hat die Ordnernavigation auf `cmd+left` und `cmd+right` gelegt, und Schritt S11b des Plans trägt beide Pfeile in die Tastentabelle nach. Die Behelfsbelegung steht damit ohne ihre Begründung da.

Die Frage muss der Nutzer beantworten und nicht der Planner, weil `ctrl+b` und `ctrl+s` zu den 39 Kombinationen gehören, die der `ontocoder` frei gewählt und der Nutzer am 260803-2110 als Ganzes durchgesehen und angenommen hat. Eine dieser Belegungen ohne ihn zu ändern hieße, jene Annahme zu unterlaufen. Zu beantworten ist sie vor S11c, wenn der Kommentar an dieser Stelle ohnehin angefasst wird; danach kostet sie einen zweiten Durchgang durch dieselbe Datei.

## Options

1. **Beides wandert: `ctrl+left` verschmälert, `ctrl+right` verbreitert.**
   - Pro: die Richtung der Taste ist die Richtung der Bewegung, ohne dass der Nutzer eine Merkregel braucht. Die Belegung steht dann neben `cmd+left` und `cmd+right` für den Ordnerwechsel, was den Pfeilblock zu einer zusammenhängenden Sache macht: Cmd bewegt im Verzeichnisbaum, Ctrl bewegt die Trennlinie.
   - Contra: `ctrl+b` und `ctrl+s` sind seit dem 260803 ausgeliefert und vom Nutzer angesehen. Wer sie sich gemerkt hat, muss umlernen.
   - Offene Nebenfrage: welche Richtung verbreitert. Naheliegend ist, dass der Pfeil die Trennlinie schiebt, `ctrl+right` also den linken Nachbarn verbreitert. Bei vier Bereichen mit drei Trennlinien ist "der aktive Bereich wird breiter" die einfachere Lesart, und dann verbreitert `ctrl+right` nicht immer nach rechts.

2. **Beides bleibt auf `ctrl+b` und `ctrl+s`.**
   - Pro: keine Änderung an einer angenommenen Belegung, kein Umlernen, und die Anfangsbuchstaben tragen die Bedeutung ohne die Richtungsfrage aus Möglichkeit 1. Die Freiheit, umzubelegen, hat der Nutzer nach C3 ohnehin.
   - Contra: der Kommentar in der Datei muss trotzdem umgeschrieben werden, weil er eine Lücke der Schreibweise behauptet, die es nicht mehr gibt. Die Belegung bleibt dann eine, die niemand mehr aus dem Vorbild herleiten kann.

3. **Beide Wege ab Werk: `ctrl+b` und `ctrl+left`, `ctrl+s` und `ctrl+right`, je zwei Kombinationen auf einer Funktion.**
   - Pro: kein Umlernen, und der naheliegende Griff zum Pfeil trifft. Zwei Wege je Funktion ist in dieser Belegung der Normalfall, den C3 ausdrücklich trägt.
   - Contra: vier Kombinationen für zwei Funktionen, ohne dass eine Not dazu zwingt. Die Norton-Reihe trägt zwei Wege, weil der Touch Bar den einen unbrauchbar macht; hier gibt es keinen solchen Befund.

## Constraints

- Keine der gewählten Kombinationen darf mit den 57 ausgelieferten kollidieren. `ctrl+left` und `ctrl+right` sind frei, geprüft am 260804-1122 gegen `resources/default-keymap.toml`.
- Die Antwort ändert Tastenlisten in `resources/default-keymap.toml` und nichts sonst. Kein Abnahmekriterium des Specs und kein Schritt des Plans hängt daran.
- Die drei Wahlregeln aus `decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md` binden jede Ergänzung dieser Datei und damit auch diese.

## Recommendation

Möglichkeit 1, sofern der Nutzer die Richtungsfrage mitbeantwortet. Die Pfeile sind hier nicht nur die naheliegende Taste, sondern die einzige, die die Bewegung ohne Merkregel trägt, und mit dem Ordnerwechsel auf Cmd und der Trennlinie auf Ctrl entsteht ein Muster statt zweier Einzelfälle. Möglichkeit 3 löst dasselbe Problem mit doppelt so vielen Kombinationen und ohne den Befund, der die doppelten Wege der Norton-Reihe rechtfertigt.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: Nutzerentscheid 260804 — **ja, sie wandern.** `bereich_verbreitern` liegt künftig auf `ctrl+right`, `bereich_verschmaelern` auf `ctrl+left`; `ctrl+b` und `ctrl+s` werden frei und stehen in keiner Tastenliste mehr. Der Grund für die Buchstaben war allein, dass die Kombinationsschreibweise die Seitwärtspfeile nicht kannte, und dieser Grund fällt mit S11b weg. Die Pfeile zeigen die Richtung, statt dass zwei Buchstaben sie benennen, und sie stehen neben `cmd+left` und `cmd+right` der Ordnernavigation: dieselben Tasten, andere Zusatztaste. Umzusetzen in S11c zusammen mit den drei anderen Einträgen; die Umsetzung zieht diesen Datensatz auf `_i_`.
