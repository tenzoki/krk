# Schluckt der Ereignisabgriff den zulässigen Befehl oder den ausgeführten?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `shared/planning/260813-0053_o_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md` (C2), `crates/krk-ui/src/appkit/ereignisse.rs:498-514`, `crates/krk-ui/src/appkit/anwendung.rs:2109-2160`

---

## Frage

Der Ereignisabgriff schluckt heute genau die Tastendrücke, deren Befehl **etwas getan hat**: `kommando_ausfuehren` gibt zurück, ob ausgeführt wurde, und nur dann liefert der Abgriff `nil`. Alles andere geht unverändert an AppKit weiter. Der Modulkopf von `ereignisse.rs` schreibt das aus, und der Grund war, dass eine noch ungebaute Funktion dem Menü kein Kürzel wegnehmen sollte.

Sobald das Menü alle Befehle mit ihren Kürzeln trägt, bekommt diese Regel eine Folge, die sie bisher nicht hatte. Drei Fälle sind zu unterscheiden, und sie verhalten sich verschieden:

- **Der Befehl ist unzulässig** (ein Blatt steht, oder der Fokus passt nicht zum Wirkungsbereich). Der Abgriff reicht weiter. Der Menüeintrag muss ausgegraut sein, sonst führt das Menü aus, was der Fokusvorbehalt gerade abgewiesen hat — im Editor bewegte dann ein Auf-Pfeil die Dateiliste statt der Schreibmarke. Diese Hälfte ist nicht strittig und steht als Abnahmekriterium im Spec.
- **Der Befehl ist zulässig und hat etwas getan.** Der Abgriff schluckt, das Menü sieht den Tastendruck nie.
- **Der Befehl ist zulässig und hat nichts getan** (Abbrechen ohne laufenden Vorgang, ein Ausblenden, das das letzte Dateifenster stehen lässt). Der Abgriff reicht weiter, der Menüeintrag ist zulässig und damit bedienbar, und das Menü ruft denselben Befehl ein zweites Mal. Er tut wieder nichts.

Der dritte Fall ist heute folgenlos, weil kein Menüeintrag darauf wartet. Nach dieser Runde wartet einer.

## Möglichkeiten

1. **Der Abgriff schluckt, was zulässig war.** `kommando_ausfuehren` liefert `true`, sobald der Befehl die beiden Vorbehalte passiert hat, unabhängig davon, was sein Rumpf zurückgibt.
   - Dafür: Zulässigkeit ist dann **eine** Frage mit **einer** Antwort, und sie trägt beide Seiten: der Abgriff schluckt genau die zulässigen, das Menü ist genau für die zulässigen bedienbar. Der Doppelweg entsteht gar nicht. Das Menükürzel ist danach eine Beschriftung und der Mausklick der Weg, und das ist auf dem Mac die gewöhnliche Lage.
   - Dagegen: Für zulässige, aber wirkungslose Befehle geht der Tastendruck nicht mehr an AppKit weiter. Heute erreicht er dort in aller Regel nichts; ob das ausnahmslos gilt, ist ungemessen. Die Rückgabewerte der rund fünfundsiebzig Befehlsrümpfe verlieren ihren heutigen Zweck und behalten allein den, den `#[must_use]` ihnen gibt.
2. **Es bleibt beim ausgeführten Befehl; der Doppelweg wird hingenommen und geprüft.** Der Spec zählt die Befehle auf, die zulässig `false` liefern können, und je einer wird daraufhin geprüft, dass ein zweiter Lauf nichts anrichtet.
   - Dafür: Kein Eingriff in eine Regel, die seit der Runde 1 trägt.
   - Dagegen: Eine Aufzählung, die vollständig bleiben muss, ohne dass der Übersetzer sie dazu zwingt. Genau die Sorte Saum, die dieses Projekt an anderen Stellen vermeidet.
3. **Das Menü trägt keine Kürzel.** Die Einträge sind nur mit der Maus bedienbar; die Kombination steht allenfalls im Beschriftungstext.
   - Dafür: Kein Doppelweg, kein Eingriff in den Abgriff.
   - Dagegen: Ein Mac-Menü ohne Kürzelspalte nimmt dem Menü seine zweite Aufgabe, die Kombinationen zu lehren. Ein Kürzel im Beschriftungstext ist eine zweite Schreibweise neben der der Belegungsansicht.

## Randbedingungen

- Die sechs Textbefehle des Menüs tragen kein Kommando und laufen über die Antwortkette. Sie sind von dieser Frage nicht berührt und bleiben, wie sie sind.
- Eine Funktion, die eine Kombination trägt, aber noch kein Kommando hat, muss weiterhin durchfallen. Dieser Zweig steht in `ereignisse.rs:502-505` vor der Senke und wird von keiner der drei Möglichkeiten berührt.

## Empfehlung

Möglichkeit 1. Sie macht aus zwei Fragen eine: der Abgriff und das Menü fragen dieselbe Stelle, ob dieser Befehl hier zulässig ist, und ihre Antworten können nicht auseinanderlaufen. Möglichkeit 2 hält eine Liste am Leben, die niemand vollständig hält, und Möglichkeit 3 beschädigt den Zweck des Menüs.

Der Preis von Möglichkeit 1 ist zu messen und nicht zu behaupten: der Plan zählt auf, welche Befehle zulässig `false` liefern, und prüft für jeden, dass ihr Tastendruck heute an AppKit nichts erreicht. Findet sich einer, der doch etwas erreicht, ist er der Anlass, die Frage neu zu stellen.

Die Runde fährt bis zu einer Antwort auf Möglichkeit 1.

---
Answered:
Implemented:
Deferred:
Superseded by:
