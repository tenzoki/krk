Der Abschlusstext überschreibt die während des Vorgangs verdrängte Fenstermeldung

---

S16b sagt zu: "Verdrängt wird nichts endgültig. Eine Auswurfmeldung, die während einer Kopie eintrifft, steht in ihrem Feld und erscheint, sobald die Vorgangsanzeige endet; verzögert um die Laufzeit der Operation, verloren nicht." Gemessen erscheint sie nicht. Sie ist verloren, nicht verzögert.

---

## Warum

Beide Texte wohnen in demselben Feld. Die Auswurfmeldung geht über `DateifensterQuelle::meldung_zeigen` in `fenstermeldung`, und der Abschlusstext des Vorgangs geht am Ende über denselben Weg in dasselbe Feld. `Anwendungsdelegierter::vorgang_beenden` nimmt erst die Vorgangsanzeige weg und setzt unmittelbar danach den Abschlusstext; zwischen den beiden Schritten liegt kein Zeichendurchgang, in dem die Auswurfmeldung zu sehen wäre.

Der Plan sagt an zwei Stellen zwei Dinge, die sich in einem einzeiligen Feld nicht beide halten lassen:

- "Eine Auswurfmeldung … erscheint, sobald die Vorgangsanzeige endet."
- "Der Abschlusstext … geht danach als gewöhnliche Fenstermeldung in dieselbe Zeile; an diesem Weg ändert der Schritt nichts."

Eine Zeile trägt einen Text. Der letzte Schreiber gewinnt, und das ist der Abschlusstext.

## Die Messung

Am laufenden Bündel am 260804-1915, dreimal gleich:

| Zeitpunkt | Was in der Zeile des linken Dateifensters steht |
|---|---|
| während der Kopie, nach der Auswurfmeldung | `Kopieren: 9.131 Einträge, 18,2 GB, eine ausgewählte Position · abschrift-… · Esc bricht ab` |
| nach dem Ende des Vorgangs | `Kopieren abgebrochen: 9.175 Einträge, 18,4 GB (eine ausgewählte Position) übertragen` |

Die Auswurfmeldung erscheint zu keinem Zeitpunkt. Die Sonde hat sie über denselben Eingang gesetzt, den `auffrischung::datentraeger_verloren` benutzt (`Anwendungsdelegierter::melden`); ein Datenträger wurde dafür nicht körperlich ausgeworfen.

## Was zu entscheiden wäre

Drei Wege, ohne Empfehlung:

1. **Die Zusage streichen.** Der Abschlusstext ist der neuere und der wichtigere. Die Auswurfmeldung ist dann verloren, und der Spec sagt das aus, statt das Gegenteil zu versprechen.
2. **Den Abschlusstext anhängen statt setzen.** Steht eine verdrängte Fenstermeldung, trägt die Zeile beide, getrennt durch den Mittelpunkt. Kostet eine Sonderregel im Setzen der Fenstermeldung.
3. **Die verdrängte Meldung nach dem Abschlusstext zeigen.** Sie bräuchte eine eigene Lebensdauer und damit einen vierten Rang, was der Rangfolge aus S16b widerspricht.

Der Fall ist selten: er braucht einen Datenträgerwechsel während einer laufenden Operation an demselben Dateifenster.

**Aufgefallen bei:** der Umsetzung von Schritt 16b am 260804-1915.

---

Resolved: Der Abschlusstext überschreibt die Auswurfmeldung nicht mehr. Die beiden liegen seit dem 260804-1940 in zwei Feldern mit zwei Lebensdauern, und die Auswurfmeldung erscheint, sobald der Abschlusstext nach seiner eigenen Löschregel fällt. Verzögert, nicht verloren; die Zusage des Plans hält, ihre Formulierung nicht ganz.

**Keiner der drei aufgeführten Wege, sondern der Grund darunter.** Beide Texte teilten sich das Feld `fenstermeldung`, obwohl sie zwei verschiedene Sorten von Aussage sind: die Auswurfmeldung ist ein Ereignis, das der Nutzer nicht angefordert hat, der Abschlusstext ist die, spät eintreffende, Antwort auf das F5, mit dem er die Operation gestartet hat. Getrennt in zwei Felder, hat jedes genau eine Löschregel, und die Rangfolge entscheidet, wer die Zeile bekommt:

```
1  Befehlsantwort    Abschlusstext, "es läuft bereits eine Operation", …
                     fällt mit dem nächsten Tastenbefehl
2  Vorgangsanzeige   der Stand einer laufenden Operation
                     fällt mit dem Bericht
3  Fenstermeldung    Auswurf, beschädigte Ablagedatei
                     fällt beim nächsten Ordner- oder Tabwechsel
4  Tabmeldung        der Zustand des sichtbaren Ordners
```

**Die Regel, die jetzt gilt: verdrängt wird nichts gelöscht.** Jede Aussage steht in ihrem eigenen Feld, bis ihre eigene Lebensdauer endet; die Zeile zeigt die oberste, die noch steht. Eine verdrängte Aussage erscheint, sobald alles über ihr gefallen ist. Kein Zeitgeber, weil jede der vier Lebensdauern an einem Ereignis hängt und an keiner Uhr; keine zweite Zeile; eine Regel für alle vier Quellen ohne Sonderfall je Meldungsart.

Die Zusage des Plans hält damit der Sache nach. Sie hält nicht in ihrem Wortlaut: die Auswurfmeldung erscheint nicht, "sobald die Vorgangsanzeige endet", sondern einen Tastenbefehl später, weil in diesem Augenblick der Abschlusstext in der Zeile steht. Beide Texte gleichzeitig zu zeigen ginge nur, indem die Zeile sie aneinanderhängt, und das wäre eine Regel, die bei vier Quellen eine unlesbare Zeile baut. Der Plananteil ist als eigener Defekt abgelegt: `260804-1940_o_s16b-beschreibt-die-statuszeile-mit-drei-raengen-gebaut-sind-vier.md`.

Umgesetzt in `crates/krk-ui/src/appkit/statuszeile.rs` (die Regel als Funktion `zeile`, ohne AppKit, mit sechs Prüfungen belegt), `crates/krk-ui/src/appkit/tabelle.rs` (das Feld `befehlsantwort` und die beiden Zugänge) und `crates/krk-ui/src/appkit/anwendung.rs` (der Abschlusstext geht über `antwort_zeigen`, die eine Löschregel steht in `kommando_ausfuehren`).

Gemessen am laufenden Bündel am 260804-1940, Kopie eines Ordners mit 30.000 Einträgen, Auswurfmeldung über denselben Eingang gesetzt, den `auffrischung::datentraeger_verloren` benutzt:

| Zeitpunkt | Zeile, die der Nutzer sieht |
|---|---|
| während der Kopie, nach der Auswurfmeldung | `Kopieren: 804 Einträge, 19,2 kB, eine ausgewählte Position · datei-0526.txt · Esc bricht ab` |
| nach dem Ende des Vorgangs | `Kopieren abgebrochen: 842 Einträge, 20,2 kB (eine ausgewählte Position) übertragen` |
| nach dem nächsten Tastenbefehl | `SICHERUNG wurde ausgeworfen; das Dateifenster zeigt jetzt /Users/k1` |
