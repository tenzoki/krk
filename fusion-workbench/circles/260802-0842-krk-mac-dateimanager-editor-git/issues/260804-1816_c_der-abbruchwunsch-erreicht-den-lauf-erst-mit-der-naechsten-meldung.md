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

---
Resolved: Der Weg, den der Datensatz vorschlägt: `krk_core::operation` gibt das Abbruchkennzeichen heraus. Es heißt `Lauf::abbruchgriff() -> Abbruchgriff`.

**Ein Kennzeichen statt zwei, nicht ein zweiter Weg neben dem ersten.** Der Datensatz nennt als Möglichkeit einen zweiten Rückgabewert oder eine Methode am `Lauf`. Beides gäbe dem Hauptfaden einen Griff; entscheidend ist, was danach mit dem alten Kennzeichen geschieht. `Vorgangszustand` trug einen eigenen `AtomicBool`, den der Vermittlerfaden abfragte und weiterreichte. Der ist entfallen: `Vorgangszustand::neu` nimmt jetzt den Griff entgegen und hält damit dasselbe Kennzeichen, das der Arbeitsfaden liest. Zwei Kennzeichen für eine Frage wären genau die zweite Wahrheit, die der Plan an anderen Stellen vermeidet.

Die Spanne bis zum Greifen ist damit nicht mehr "bis zur nächsten Meldung", sondern "bis zum nächsten Eintrag oder zum nächsten Statusrückruf von `copyfile(3)`" — dieselbe Spanne, die `Lauf::abbrechen` schon immer hatte.

**Der `Receiver` bleibt ungeteilt.** Der `Lauf` geht weiterhin ganz an den Vermittlerfaden; geteilt wird allein das `Arc<AtomicBool>`, das der Lauf ohnehin hält. Der Grund, aus dem der Umweg bestand, war nie, dass ein `AtomicBool` sich nicht teilen ließe, sondern dass er nicht herausgegeben wurde.

**Nachgemessen mit einer Prüfung, die die Aufstellung des Betriebs nachbaut**: `der_abbruchgriff_wirkt_von_einem_faden_ohne_den_lauf` in `crates/krk-core/tests/operation.rs`. Eine Kopie von 500 MB mit `Uebertragungsart::ImmerBytes`, der `Lauf` wandert per `thread::spawn` auf einen zweiten Faden, der in `recv` wartet; der abbrechende Faden hat ihn nicht und liest keine einzige Meldung. Der Bericht meldet `Abschluss::Abgebrochen` und weniger als 500 MB übertragene Bytes. Ohne den Griff ließe sich diese Prüfung gar nicht schreiben.

**Was nicht gemessen ist.** Der Fall, in dem die Spanne vorher wirklich groß war, ist `NSFileManager.trashItemAtURL:` auf einem sehr großen Ordner. Er läuft nur im Bündel und braucht einen Ordner, dessen Papierkorbwanderung Sekunden dauert; das ist hier nicht aufgesetzt worden. Für Kopieren und Verschieben war die Spanne schon vorher klein (292 bis 296 ms am 260804, darin das Zumachen des Blattes), und ein Vorher-Nachher-Vergleich dort zeigte nichts. Die Änderung nimmt eine Abhängigkeit weg, keine gemessenen Millisekunden.

**Drei Leser sind mit dem Umweg entfallen**, weil sie ihn trugen und sonst niemanden: `Vorgangszustand::abgebrochen` (der Vermittlerfaden fragte ihn ab), `Lauf::ist_abgebrochen` (er prüfte, ob schon abgebrochen war, bevor er weiterreichte) und `Abbruchgriff::ist_abgebrochen`, das ich zuerst mitgebaut hatte und das keinen Aufrufer bekam. Ein `pub` ohne Aufrufer in einer Bibliothekskiste bekommt keinen Übersetzerhinweis; genau darum ging es im Datensatz `260803-2025_c_zwei-generationsleser-im-kern-haben-keinen-aufrufer-mehr.md` vom selben Tag.

Geprüft am 260805-0945: die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` enden alle mit 0.
