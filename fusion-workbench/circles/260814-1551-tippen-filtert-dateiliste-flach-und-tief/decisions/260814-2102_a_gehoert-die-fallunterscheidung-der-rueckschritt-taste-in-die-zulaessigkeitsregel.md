# Gehört die Fallunterscheidung der Rückschritt-Taste in `kommandos/zulaessigkeit.rs`?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `decisions/260814-1830_a_wie-nimmt-der-nutzer-ein-einzelnes-zeichen-des-filters-zurueck.md` (die Antwort, die diese Frage ausdrücklich an den Planner gibt); `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md` (Abschnitt `## Offen für den Planner`, vierter Punkt); `crates/krk-ui/src/kommandos/zulaessigkeit.rs:172-181` (die eine Regel mit ihren zwei Fragern); `resources/default-keymap.toml:156-158` (die zwei Kombinationen auf einer Funktion)

---

## Question

Die Runde 7 hat mit `crates/krk-ui/src/kommandos/zulaessigkeit.rs` die Stelle gebaut, an der eine Zulässigkeitsfrage einmal beantwortet und von zwei Fragern gelesen wird, dem Ereignisabgriff und der Ausgrauung des Hauptmenüs. Die Rückschritt-Taste bekommt in dieser Runde eine Bedeutung, die vom Zustand abhängt: steht ein Filtertext, nimmt sie ein Zeichen zurück und erreicht den Papierkorb nicht; steht keiner, wirkt sie wie bisher. Der Entscheidungsdatensatz vom 260814-1845 stellt die Frage ausdrücklich dem Planner: gehört diese Regel dorthin oder in den Zweig des Befehls? Der Spec verlangt allein, dass es **eine** Stelle ist und nicht zwei.

## Options

1. **In die Zulässigkeitsregel.** `Lage` bekommt ein fünftes Feld „steht ein Filtertext", und `zulaessig` weist `Kommando::InPapierkorb` bei stehendem Filtertext ab.
   - Pro: die zustandsabhängige Frage stünde an der Stelle, die für zustandsabhängige Fragen gebaut ist, und die beiden Frager bekämen sie beide.
   - Kontra: siehe `## Constraints`. Die Regel kann sie dort nicht beantworten.

2. **Im Ausführungszweig, hinter der unveränderten Zulässigkeitsregel.** Der Anschlag wird bis zur Senke mitgetragen; der Zweig `Kommando::InPapierkorb` fragt eine reine Funktion in einem eigenen Modul neben `zulaessigkeit.rs`.
   - Pro: die Regel bekommt die Größen, die sie braucht. `zulaessigkeit.rs` und seine Tafel aus 280 Fällen bleiben unangetastet. Der Menüeintrag behält seine eine Bedeutung. Die vorhandene Regel deckt genau die Lagen ab, in denen die Rücknahme unerwünscht wäre.
   - Kontra: `kommando_ausfuehren` bekommt ein zweites Argument, und `Eingabe::Kommando` trägt ein Feld mehr. Drei Aufrufstellen ziehen nach.

3. **Im Fänger des Ereignisabgriffs**, als dritte Station neben Aufnahme und Suche der Belegungsansicht.
   - Pro: der Fänger sieht den rohen Tastendruck und ist die vorhandene Stelle für „diese Taste bedeutet hier etwas anderes".
   - Kontra: der Fänger steht **vor** dem Nachschlag, und die `Lage` entsteht erst dahinter. Die zweite Station fragt aus einem genannten Grund nicht nach dem Ersthelfer; für das Dateifenster trägt dieser Grund nicht, denn beim Umbenennen in der Liste hält der Feldeditor den Rang und die Taste muss dort ein Zeichen löschen. Eine dritte Station müsste die Lage vor den Nachschlag ziehen und damit eine tragende Anordnung ändern.

## Constraints

**Möglichkeit 1 ist nicht bloß unschön, sie ist nicht durchführbar**, und das ist am Baum belegt und keine Vermutung:

- `resources/default-keymap.toml:156-158` legt `delete` **und** `cmd+delete` auf dieselbe Funktion `in_papierkorb`. Beide Tastendrücke werden im Nachschlag zu demselben `Kommando::InPapierkorb`, bevor irgendjemand fragen kann. `zulaessig(kommando, lage)` bekommt dieses eine Kommando und hat nichts, woran die zwei Wege sich unterscheiden ließen. C1.17 verlangt aber, dass `cmd+delete` in jeder Lage räumt.
- Der zweite Frager derselben Funktion ist `validateMenuItem:`, und der hat überhaupt keinen Tastendruck. Eine Antwort „unzulässig" dort graute den Menüeintrag „In den Papierkorb räumen" aus, sobald ein Filtertext steht. C1.19 und C6.11 schließen das aus: die drei Ansichten führen für `delete` weiter genau einen Eintrag, und die übrigen Löschwege bleiben unberührt.

Daneben:

- Die Regel hängt an zwei Größen und an keiner dritten (C6.10): ob ein Filtertext steht, und ob der Anschlag aus einer Wiederholung stammt, die bei stehendem Filtertext begann. Die zweite steht als `isARepeat` an jedem Tastenereignis; im Baum liest sie heute nichts.
- Es soll eine Stelle sein und nicht zwei.

## Recommendation

Möglichkeit 2, und so fährt der Plan. Die Begründung ist nicht Geschmack, sondern Entscheidbarkeit: die Frage „was bedeutet dieser Tastendruck" ist aus den Eingaben der Zulässigkeitsregel nicht zu beantworten, weil die beiden Wege vor ihr zu einem Kommando zusammenfallen und einer ihrer beiden Frager gar keinen Tastendruck hat. Der Mechanismus wechselt deshalb, statt dass die Näherung angepasst würde: gefragt wird dort, wo der Anschlag noch bekannt ist.

**Die vorhandene Regel bleibt der Torwächter und wird nicht umgangen.** Der neue Zweig sitzt hinter ihr. Damit deckt sie ohne eine einzige neue Zeile die Lagen ab, in denen die Rücknahme falsch wäre: beim Umbenennen in der Liste gehört der Ersthelfer AppKit, `zulaessig` sagt nein, und die Taste löscht ein Zeichen im Textfeld; steht ein Blatt, ebenso; steht ein fremdes Fenster vorn, ebenso.

**Die Regel selbst wird eine reine Funktion in einem eigenen Modul neben `zulaessigkeit.rs`**, in derselben Bauart: keine Zeile AppKit, drei Wahrheitswerte hinein, einer von drei Ausgängen und der neue Merker hinaus, eine ausgeschriebene Tafel in den Proben. Damit ist sie ohne Fenster prüfbar, sie steht an einer Stelle, und eine Zählprobe hält fest, dass sie genau einen Aufrufer hat.

**Der Merker wohnt beim Anwendungsdelegierten und nicht am Tab.** Eine Tastenwiederholung gehört keinem Tab und keinem Dateifenster: ein Tabwechsel braucht einen anderen Tastendruck, und der beendet die Wiederholung. Je Tab gehalten wäre dasselbe Faktum mehrfach da.

---
Answered: `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Abschnitt `## Wo die Fallunterscheidung der Rückschritt-Taste fällt` und Schritte C1 und C2 — Möglichkeit 2. `kommandos/zulaessigkeit.rs` bleibt unverändert; die Regel steht als reine Funktion in `crates/krk-ui/src/kommandos/rueckschritt.rs` und wird im Zweig `Kommando::InPapierkorb` von `kommando_ausfuehren` gefragt, hinter der unveränderten Zulässigkeitsprüfung.
Implemented:
Deferred:
Superseded by:
