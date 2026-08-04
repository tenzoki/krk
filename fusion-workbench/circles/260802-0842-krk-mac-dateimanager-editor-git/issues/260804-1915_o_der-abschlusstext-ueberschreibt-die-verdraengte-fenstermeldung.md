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
