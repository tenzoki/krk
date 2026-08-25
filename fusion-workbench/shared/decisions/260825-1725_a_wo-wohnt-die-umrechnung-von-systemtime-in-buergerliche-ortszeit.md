# Wo wohnt die Umrechnung von `SystemTime` in bürgerliche Ortszeit?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Answered:** 260825-1740, Kai Stalmann — Moeglichkeit 1: die Umrechnung wohnt in krk-core, verzeichnis/sys.rs. Empfehlung des Planers ohne Aenderung uebernommen.
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`; `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0838_*_jeder-gepackte-eintrag-traegt-den-1-januar-1980-statt-des-aenderungsdatums-der-quelle.md`; `shared/decisions/260825-1725_*_wie-kommt-ein-aenderungsdatum-in-eine-profilzeile.md`; `crates/krk-core/src/verzeichnis/sys.rs` (Modulkopf); `crates/krk-bench/src/bericht.rs:653` (`zerlegen`); `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-1249_*_die-zusage-haengt-jetzt-am-rufer-in-einer-anderen-kiste-*.md`

---

## Question

Zwei Arbeiten der Runde 18 brauchen dieselbe Umrechnung: eine Zahl von Sekunden seit 1970 in
Jahr, Monat, Tag, Stunde, Minute und Sekunde der **bürgerlichen Ortszeit**.

- Das Packen muss den Zeitstempel der Quelle in das MS-DOS-Zeitfeld des Zip-Formats schreiben.
  Dieses Feld ist Ortszeit ohne Zonenangabe.
- Die Datumszeile eines Leseprofils muss ein Änderungsdatum anzeigen.

`krk-core` hat diese Umrechnung nicht und kann sie sich heute nirgends holen. Die Standard-
bibliothek kennt keine Zeitzone. `NSDateFormatter`, den die Vorschau und die Dateiliste
benutzen, liegt in AppKit und damit außerhalb des Kerns. Die Kalenderrechnung nach Hinnant
steht zwar im Baum, in `krk-bench/src/bericht.rs:653`, aber sie ist privat, sie rechnet UTC,
und `krk-bench` hängt von `krk-core` ab und nicht umgekehrt.

Der Zonenversatz hängt am **Zeitpunkt der Datei** und nicht am Zeitpunkt des Laufs. Am 260825
gemessen: `ditto(1)` selbst macht genau diesen Fehler und legt eine Märzdatei aus einem
Archiv eine Stunde daneben ab, weil es den heute geltenden Sommerzeitversatz auf ein Datum
anwendet, an dem er nicht galt. Wer eine Zahl je Lauf holt statt einer Antwort je Eintrag,
baut denselben Fehler nach.

## Options

1. **`localtime_r(3)` als sechste Schnittstelle in `crates/krk-core/src/verzeichnis/sys.rs`.**
   Der Kern bekommt eine Funktion, die aus einem `SystemTime` die sechs Felder der Ortszeit
   liefert, samt dem Versatz, der zu **jenem** Zeitpunkt galt.
   - Pros: Sommerzeitrichtig durch Bauart und nicht durch Sorgfalt des Rufers; das System
     beantwortet die Frage, die es allein beantworten kann. Keine neue Kiste, kein `-sys`-Paket,
     kein C-Code, kein Eingriff in die Merkmalsliste — die stärkste Eigenschaft des
     Abhängigkeitsbaums bleibt unberührt. Keine zweite Kalenderrechnung entsteht: `localtime_r`
     liefert die Felder fertig, also bleibt `zerlegen` in `krk-bench` das einzige Stück
     Kalenderarithmetik im Baum. Beide Rufer bekommen dieselbe Antwort ohne durchgereichten
     Parameter. Der Ort ist gedeckt: der Modulkopf von `sys.rs` nennt sich selbst „die
     Systemschicht des Kerns und nicht allein die des Lesers" und führt mit `flock(2)` bereits
     eine Schnittstelle, die weder liest noch schreibt.
   - Cons: `#![deny(unsafe_code)]` wird ein zehntes Mal geöffnet, und die Angabe „fünf
     Schnittstellen und neun Funktionen" steht an drei Stellen wortgleich (`lib.rs:20`,
     `verzeichnis/mod.rs:31`, `verzeichnis/sys.rs:1` und `:26`) und wird an allen falsch. Die
     Deklaration von `struct tm` ist eine fremde Speicherform, die von Hand richtig sein muss.
     Eine Probe auf einen festen Kalenderwert hängt an der Zeitzone des Prüfgeräts.
2. **Der Zonenversatz kommt als Rückruf aus `krk-ui` über `NSTimeZone`.** Der Kern bleibt ohne
   `unsafe` und bekommt die Kalenderrechnung aus `krk-bench` heraufgehoben.
   - Pros: Kein neues `unsafe` im Kern, und der `NSTimeZone` beantwortet die Zonenfrage
     ebenfalls je Zeitpunkt, wenn man `secondsFromGMTForDate:` nimmt.
   - Cons: Ein Wert genügt nicht, es braucht einen Rückruf, und der reist durch
     `zusammenfassen` und durch die Auftragsmaschinerie des Packens. Die Richtigkeit hinge dann
     am Rufer in einer anderen Kiste — genau die Bauform, die in der Runde 17 schon einen
     Defekt erzeugt hat (`260825-1249_*_die-zusage-haengt-jetzt-am-rufer-in-einer-anderen-kiste`).
     Und die Kalenderrechnung stünde dann doch im Kern und wäre die Stelle, an der ein
     Schaltjahr oder ein Zyklusrand falsch sein kann, ohne dass es jemand merkt.
3. **Eine Zeitkiste aufnehmen** (`chrono`, `jiff` oder `time`).
   - Pros: Fertig und geprüft.
   - Cons: `chrono` zieht auf macOS `iana-time-zone` und darüber `core-foundation-sys` herein,
     also das erste `-sys`-Paket neben `windows-sys` — die Zusage der Technologiewahl fiele.
     `time` liefert den Ortsversatz nur über sein Merkmal `local-offset`, und das gibt in einem
     Programm mit mehreren Fäden aus Gründen der Sicherheit keine Antwort; KRK hat mehrere
     Fäden. Beide brächten eine Zeitzonendatenbank mit, von der genau ein Wert gebraucht wird.
4. **UTC anzeigen und schreiben.**
   - Pros: Reine Arithmetik, kein `unsafe`, keine Kiste, vollständig prüfbar.
   - Cons: Das MS-DOS-Feld ist als Ortszeit definiert; UTC hineinzuschreiben heißt, jedem
     Entpackwerkzeug eine falsche Zeit zu geben. Und ein Nutzer in Mitteleuropa läse in der
     Vorschau ein Datum, das um ein bis zwei Stunden neben dem liegt, was die Dateiliste
     daneben zeigt.

## Constraints

- `Cargo.lock` führt heute kein `cc` und außer `windows-sys` kein `-sys`-Paket. Das bleibt so.
- `#![deny(unsafe_code)]` steht an der Wurzel von `krk-core`; die einzige Öffnung ist
  `verzeichnis/sys.rs`. Ein zweites Modul mit dieser Ausnahme entsteht nicht — so schreibt es
  der Modulkopf jener Datei fest.
- Der Zonenversatz muss zum Zeitpunkt des Eintrags gelten, nicht zum Zeitpunkt des Laufs.
- Was in `krk-core` liegt, muss ohne Fenster prüfbar sein.

## Recommendation

**Möglichkeit 1.** Sie ist die einzige, die die Sommerzeit durch Bauart richtig bekommt, ohne
den Abhängigkeitsbaum anzufassen, und sie bedient beide Rufer aus einer Quelle. Der Ort ist
`verzeichnis/sys.rs`, und der Modulkopf jener Datei hat den Fall vorweggenommen: er erklärt
seinen Namen ausdrücklich für gedeckt, weil das Modul die Systemschicht des Kerns ist, und er
verlangt, dass eine neue Schnittstelle **dorthin** kommt und nicht daneben.

Zum Preis gehört, dass die Zahlen mitgezogen werden. Aus fünf Schnittstellen werden sechs, aus
neun gebundenen Funktionen zehn, aus vier `unsafe extern "C"`-Blöcken fünf, und die Angabe
steht wortgleich in `lib.rs`, in `verzeichnis/mod.rs` und zweimal in `sys.rs`. Dieses Vorhaben
hat mit Zahlen in Prosa schlechte Erfahrungen gemacht; die Umsetzung zieht deshalb alle vier
Stellen in demselben Schritt nach und prüft mit `grep`, dass keine fünfte übrig bleibt.

**Die Prüfbarkeit ist die eine Stelle, an der die Empfehlung Arbeit verlangt statt sie zu
sparen.** Eine Zusicherung auf einen festen Kalenderwert hängt an der Zeitzone des Geräts. Die
Form, die dieser Baum dafür kennt, ist die **Kindprobe mit gesetzter Umgebung**: `tests/ablage.rs`
startet dieselbe Prüfdatei mit einer gesetzten Umgebungsvariablen noch einmal, und
`tests/verzeichnis.rs` tut dasselbe unter `ulimit -n 64`. Eine Probe unter `TZ=UTC` und eine
zweite unter einer Zone mit Sommerzeit fügen sich in dieses Muster ein, statt eine neue Bauform
aufzumachen; die zweite ist die einzige, die den Sommerzeitfall überhaupt prüfen kann. Daneben
trägt eine zonenunabhängige Rundwegprobe, dass der Wert überhaupt ankommt.
