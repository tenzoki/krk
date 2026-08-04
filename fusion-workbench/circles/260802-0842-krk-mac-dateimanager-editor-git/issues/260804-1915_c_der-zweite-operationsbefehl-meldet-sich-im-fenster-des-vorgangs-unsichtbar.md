Der zweite Operationsbefehl meldet sich im Fenster des Vorgangs unsichtbar

---

C4 sagt zu: "Ein zweiter Operationsbefehl startet währenddessen nichts und sagt in derselben Zeile, dass bereits eine Operation läuft." Die Meldung entsteht, aber der Nutzer sieht sie nicht, wenn er den zweiten Befehl in dem Dateifenster auslöst, das den laufenden Vorgang begonnen hat. Das ist der häufige Fall: der Nutzer hat das Fenster meist nicht gewechselt.

---

## Warum

Die Meldung ist eine Fenstermeldung und geht an das **aktive** Dateifenster. Die Rangfolge aus S16b stellt die Vorgangsanzeige darüber. Ist das aktive Dateifenster dasselbe, das den Vorgang begonnen hat, steht dessen Fortschritt in der Zeile, und die Meldung wartet in ihrem Feld auf das Ende des Vorgangs. Dort wird sie zusätzlich vom Abschlusstext überschrieben, siehe `260804-1915_o_der-abschlusstext-ueberschreibt-die-verdraengte-fenstermeldung.md`.

Die beiden Zusagen widersprechen sich in dem einen Punkt, an dem sie sich treffen. S16b legt fest, dass die Vorgangsanzeige den obersten Rang hat, mit der Begründung "das Alter der Aussage": eine laufende Operation ist neuer als ein Ereignis am Fenster. Eine Antwort auf einen Tastendruck, den der Nutzer gerade gemacht hat, ist aber noch neuer als beides. Nach demselben Ordnungsprinzip müsste sie oben stehen; nach dem Flussdiagramm des Plans steht sie unten.

## Die Messung

Am laufenden Bündel am 260804-1915, dreimal gleich. Kopie von 30.000 Einträgen, währenddessen ein zweiter Druck auf F5:

| Fall | Zeile, die der Nutzer sieht |
|---|---|
| zweiter F5 im **anderen** Dateifenster | `es läuft bereits eine Operation: Kopieren` |
| zweiter F5 im Fenster **des Vorgangs** | `Kopieren: 8.189 Einträge, 16,3 GB, eine ausgewählte Position · uebersicht-… · Esc bricht ab` |

Gestartet wird in beiden Fällen nichts; die Zahl der übertragenen Einträge läuft ununterbrochen weiter, und am Ende steht genau ein Abschlusstext. Der halbe Teil der Zusage hält, der sichtbare nicht.

## Was zu entscheiden wäre

Drei Wege, ohne Empfehlung:

1. **Die Zeile sagt es ohnehin.** Wer F5 zum zweiten Mal drückt, liest "Kopieren: 8.189 Einträge …" und weiß damit, dass eine Operation läuft. Das Abnahmekriterium wäre dann so zu lesen, dass die Zeile die Auskunft gibt, nicht dass sie den Satz enthält. Kostet nichts und macht die Zusage schwächer, als sie geschrieben ist.
2. **Ein vierter Rang für die Antwort auf einen Tastendruck.** Sie stünde über der Vorgangsanzeige und verschwände nach kurzer Zeit oder mit der nächsten Fortschrittsmeldung. Baut einen Zustand mehr und eine Lebensdauer, die keine der drei vorhandenen Quellen hat.
3. **Die Vorgangsanzeige trägt den Hinweis mit.** Der Text der laufenden Zeile bekäme einen Zusatz, solange ein zweiter Befehl abgewiesen wurde. Bleibt in drei Rängen, braucht aber ein Feld am `Vorgang` und eine Regel, wann der Zusatz wieder verschwindet.

**Aufgefallen bei:** der Umsetzung von Schritt 16b am 260804-1915.

---

Resolved: Die Meldung ist von einer Fenstermeldung zu einer **Befehlsantwort** geworden, dem neuen obersten der vier Ränge der Statuszeile. Sie steht damit auch dann in der Zeile, wenn genau das Dateifenster, in dem der Nutzer F5 gedrückt hat, den laufenden Vorgang begonnen hat.

Weg 2 der drei aufgeführten, ohne den Haken, den der Eintrag ihm zuschrieb. Der vierte Rang braucht keine Lebensdauer, die den drei vorhandenen fremd wäre: er fällt mit dem nächsten Tastenbefehl, also an einem Ereignis wie die anderen drei auch und an keinem Zeitgeber. Der Nutzer sieht die Meldung, drückt irgendeine Taste, und der Fortschritt steht wieder da.

Die Rangfolge lautet seither: Befehlsantwort, Vorgangsanzeige, Fenstermeldung, Tabmeldung, geordnet nach der Nähe zum letzten Tun des Nutzers. Dasselbe Ordnungsprinzip, das S14 und S16b schon nennen, einen Schritt weitergeführt: eine Antwort auf einen eben gedrückten Tastendruck ist neuer als eine laufende Operation.

Umgesetzt in `crates/krk-ui/src/appkit/statuszeile.rs` (die Regel als Funktion `zeile`, ohne AppKit, mit sechs Prüfungen belegt), `crates/krk-ui/src/appkit/tabelle.rs` (das Feld `befehlsantwort` und die beiden Zugänge) und `crates/krk-ui/src/appkit/anwendung.rs` (`antwort_zeigen`, dazu die eine Löschregel in `kommando_ausfuehren`).

Gemessen am laufenden Bündel am 260804-1940, Kopie eines Ordners mit 30.000 Einträgen, zweiter Druck auf F5 im Fenster des Vorgangs:

| Zeitpunkt | Zeile, die der Nutzer sieht |
|---|---|
| während der Kopie | `Kopieren: 801 Einträge, 19,2 kB, eine ausgewählte Position · datei-0268.txt · Esc bricht ab` |
| nach dem zweiten F5 | `es läuft bereits eine Operation: Kopieren` |
| nach dem nächsten Tastenbefehl | `Kopieren: 2.201 Einträge, 52,7 kB, eine ausgewählte Position · datei-0313.txt · Esc bricht ab` |
| nach dem Ende | `Kopieren abgebrochen: 2.214 Einträge, 53,0 kB (eine ausgewählte Position) übertragen` |

Gestartet hat der zweite Befehl nichts: die Zahl der übertragenen Einträge läuft von 801 über 2.201 auf 2.214 ununterbrochen weiter, und am Ende steht genau ein Abschlusstext.

Zwei weitere Meldungen standen aus einem verwandten Grund in der falschen Zeile und gehen jetzt an das aktive Dateifenster statt fest an das linke: die Startmeldungen über eine beschädigte Belegungs- oder Sitzungsdatei und die Meldung über einen gescheiterten Schreibvorgang der Sitzung. Beide betreffen die Anwendung und keine Seite.

Der Plantext von S16b beschreibt weiterhin drei Ränge; nachgezogen wird er über `260804-1940_o_s16b-beschreibt-die-statuszeile-mit-drei-raengen-gebaut-sind-vier.md`, weil der Plan dem `planner` gehört.
