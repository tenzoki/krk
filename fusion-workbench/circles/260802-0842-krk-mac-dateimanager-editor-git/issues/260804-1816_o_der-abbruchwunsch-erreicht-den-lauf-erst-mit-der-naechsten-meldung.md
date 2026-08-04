Der Abbruchwunsch erreicht den Lauf erst mit der nächsten Meldung

---

Der Hauptfaden kann den `Lauf` aus `krk_core::operation` nicht selbst abbrechen. Er setzt ein Kennzeichen, und der Vermittlerfaden reicht es weiter, sobald ihn die nächste Meldung aufweckt. Bei einer Operation, die lange nichts meldet, wirkt der Abbruch entsprechend spät.

---

## Woher es kommt

`Lauf` hält drei Dinge zusammen: das Abbruchkennzeichen, den Empfänger des Meldekanals und den Faden. Ein `Receiver` ist `Send`, aber nicht `Sync`; der `Lauf` lässt sich deshalb an einen anderen Faden **geben**, aber nicht zwischen zweien **teilen**. Der Vermittlerfaden aus S16 muss ihn haben, weil er in `recv` wartet — der Hauptfaden darf das nicht, das wäre die Dateisystem-Arbeit auf dem Hauptfaden, die `### Frage 6` ausschließt.

Also hält der Vermittlerfaden den `Lauf`, und der Hauptfaden setzt `Vorgangszustand::abbrechen`. Der Vermittlerfaden prüft das Kennzeichen nach jeder Meldung.

## Wie groß die Spanne ist

Gemessen am 260804 am laufenden Bündel: eine Kopie von 5.000 Einträgen endete 292 bis 296 ms nach dem Abbruchbefehl, und darin steckt auch das Zumachen des Blattes. Beim Kopieren ist die Spanne klein, weil jeder fertige Eintrag meldet und `copyfile(3)` innerhalb einer großen Datei alle 8 ms einen Zwischenstand schickt.

Sie ist **nicht** klein bei einer Operation, die über Sekunden nichts meldet. Der einzige Fall in dieser Runde ist `NSFileManager.trashItemAtURL:` auf einem sehr großen Ordner: das System nimmt den ganzen Baum in einem Zug, der Kern zählt ihn als einen Eintrag und meldet erst danach. Ein Abbruch währenddessen greift gar nicht — was für den Papierkorb allerdings folgenlos ist, weil der Eintrag entweder ganz drin ist oder gar nicht.

## Was zu tun wäre

Ein Weg, das Abbruchkennzeichen an den Hauptfaden zu geben, ohne den `Lauf` zu teilen. `krk_core::operation::starten` legt den `AtomicBool` selbst an und gibt ihn nicht heraus; ein zweiter Rückgabewert oder eine Methode `Lauf::abbruchkennzeichen() -> Arc<AtomicBool>` löste es. Das ist eine Änderung in `crates/krk-core/`, die S16 nicht vornehmen durfte.

Dringlichkeit: gering. Die Zusage aus C4 ("lässt sich mit einem Tastenbefehl abbrechen") ist beim Kopieren und Verschieben gemessen erfüllt.

**Aufgefallen bei:** der Umsetzung von Schritt 16 am 260804-1816.
