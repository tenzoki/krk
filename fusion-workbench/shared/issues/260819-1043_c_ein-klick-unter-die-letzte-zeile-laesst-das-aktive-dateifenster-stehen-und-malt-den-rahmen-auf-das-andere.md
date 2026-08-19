Ein Klick unter die letzte Zeile lässt das aktive Dateifenster stehen und malt den Rahmen auf das andere
---
Klickt der Nutzer in die freie Fläche unter der letzten Zeile des **nicht** aktiven
Dateifensters, wird dessen Tabelle Ersthelfer, aber `aktiv` bleibt auf der anderen Seite.
Die Fokusanzeige aus C9 löst `Fokus::Dateifenster` über das aktive Dateifenster auf und
malt den Fokusrahmen damit auf die Liste, in die der Nutzer nicht geklickt hat. Nichts
zieht das nach.
---
**Der Mechanismus, und er steht ganz in diesem Baum.**

`DateifensterQuelle::angefasst()` hat genau zwei Rufer:
`tableView:shouldSelectRow:` (`crates/krk-ui/src/appkit/tabelle.rs:3564`) und die
Tableiste (`:4406`). Beim Klick in die freie Fläche feuert keiner von beiden, denn AppKit
ruft `shouldSelectRow:` nur für eine Zeile. `Anwendungsdelegierter::aktives_setzen`
(`anwendung.rs:4212`) läuft also nicht, und `aktiv` behält seinen Wert.

`fokusanzeige_nachziehen` (`anwendung.rs:4598`) liest `aktiv` frisch aus dem
Fenstermodell und gibt es an `rahmenrolle` (`kommandos/fokus.rs:317`). Dort löst
`bereich_mit_fokus` den Wert `Fokus::Dateifenster` auf `Bereich::von_seite(aktiv)` auf
(`fokus.rs:262`), also auf das **aktive** Dateifenster. Es gibt zwei Listen und einen
Fokuswert; welche gemeint ist, sagt allein das Modell, und das Modell weiß von diesem
Klick nichts.

**Was am 260819 gemessen ist**, an einem weggeworfenen Programm auf macOS 15.7.7 mit
demselben Aufbau wie `Dateifenster::bauen` (`tabelle.rs:4271`), Schlüsselfenster, Klicks
über `postEvent:atStart:`:

| | Klick auf eine Zeile | Klick unter die letzte Zeile |
|---|---|---|
| `hitTest:` liefert | das Zellenfeld | `NSTableBackgroundView` |
| `makeFirstResponder:` | Tabelle, angenommen | Tabelle, angenommen |
| `tableView:shouldSelectRow:` | gerufen | **nicht gerufen** |
| `selectedRow` danach | die geklickte Zeile | −1 |

Dieselbe Verzahnung erzeugt beim Klick auf eine Zeile nur ein Flackern: der erste Nachzug
malt noch mit dem alten `aktiv`, weil `makeFirstResponder:` vor `shouldSelectRow:` läuft,
und der zweite über `aktives_setzen` korrigiert. Beim Klick in die freie Fläche gibt es
keinen zweiten Nachzug.

**Zwei Wirkungen, nicht eine.** Die Auswahl fällt zusätzlich weg (`selectedRow` auf −1),
und damit leert sich der Vorschau-Tab über `auswahlmelder`.

**Was hier nicht behauptet wird.** Ob der Ersthelferrang im laufenden KRK-Bündel
wirklich wechselt, ist an einem Nachbau gemessen und nicht am Bündel selbst. Fällt der
Wechsel dort aus, ist das der Gegenstand des Defekts `260819-0900` und nicht dieses; der
hier beschriebene Fehler an `aktiv` bleibt davon unberührt, denn er hängt allein daran,
dass `shouldSelectRow:` nicht feuert.

**Was zu entscheiden ist, bevor gebaut wird.** Ob ein Klick in die freie Fläche das
Dateifenster überhaupt zum aktiven machen soll, ist eine Frage an den Nutzer und steht in
`shared/decisions/260819-1043_o_welche-flaechen-holen-den-fokus-wenn-man-hineinklickt.md`.
Lautet die Antwort ja, ist die naheliegende Stelle nicht ein dritter Rufer von
`angefasst()`, sondern `tableViewSelectionDidChange:` oder ein Weg, der auch ohne
Auswahländerung greift; welcher, gehört in die Planung.

**Cross-references:** `shared/analyses/260819-1043-klick-holt-den-fokus-nicht.md`
(Frage 4), `shared/issues/260819-0900_o_ein-klick-in-das-dateifenster-holt-den-fokus-nicht-der-rahmen-bleibt-stehen.md`,
`shared/decisions/260819-1043_o_welche-flaechen-holen-den-fokus-wenn-man-hineinklickt.md`

**Domain:** code
**Schwere:** Medium
**Gefunden von:** analyst, bei der Untersuchung von `260819-0900`
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs`, `crates/krk-ui/src/appkit/anwendung.rs`

---
Am Buendel gemessen, 260819, vom Nutzer: ein Klick auf eine **Zeile** holt den Fokus korrekt, und die Fokusbefehle der Tastatur wandern korrekt. Betroffen ist allein der Klick auf die freie Flaeche, und allein im Dateifenster. Damit ist dieser Datensatz der einzige lebende Defekt der Beobachtung vom 260819; `260819-0900` ist als gegenstandslos geschlossen. Die bindende Antwort steht in `shared/decisions/260819-1043_a_welche-flaechen-holen-den-fokus-wenn-man-hineinklickt.md`, Moeglichkeit 1.

---
Resolved: Der dritte Anlass sitzt am Ersthelferwechsel des Hauptfensters und nicht an der Tabelle. Die Analyse hatte gemessen, dass AppKit den Klick auf die freie Flaeche ohnehin in ein makeFirstResponder: uebersetzt und die Tabelle den Rang annimmt; KRK musste den Klick also nicht abfangen, sondern nur den Rangwechsel hoeren, und den fuehrt Hauptfenster::makeFirstResponder: seit C9 an den Melder. Der hatte einen Empfaenger, fokusanzeige_nachziehen, und hat jetzt zwei, den neuen zuerst, damit die Anzeige schon mit dem neuen aktiv rechnet. Ein mouseDown: waere die zweite Tuer gewesen, die der Modulkopf der Tabelle ausschliesst, und haette die Leiste nicht mit erfasst. Der Ruf in shouldSelectRow: bleibt: verweigert der abgebende Ersthelfer den Rang, was der Editor ueber seinen Delegierten kann, ist er die einzige Zeile, die beim Zeilenklick noch umschaltet. Beide Wege enden in aktives_setzen, wo die zweite Meldung folgenlos ist.
