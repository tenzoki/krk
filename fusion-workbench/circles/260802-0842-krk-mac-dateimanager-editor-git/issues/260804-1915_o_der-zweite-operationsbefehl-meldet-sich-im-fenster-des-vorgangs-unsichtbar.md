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
