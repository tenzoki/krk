S16b beschreibt die Statuszeile mit drei Rängen, gebaut sind vier

---

Schritt 16b des Plans nennt drei Quellen der Statuszeile und zeichnet ihre Rangfolge in einem `flowchart` mit sechs Knoten. Gebaut sind seit dem 260804-1940 vier Quellen mit vier Rängen. Der Plantext, sein Flussdiagramm, sein Zustandsdiagramm und zwei Sätze seines Abnahmekriteriums beschreiben damit nicht mehr, was im Bündel läuft.

---

## Warum die vierte Quelle entstanden ist

Die drei Ränge aus S16b haben zwei Defekte getragen, beide am 260804-1915 gemeldet und am 260804-1940 behoben:

- `260804-1915_c_der-abschlusstext-ueberschreibt-die-verdraengte-fenstermeldung.md`
- `260804-1915_c_der-zweite-operationsbefehl-meldet-sich-im-fenster-des-vorgangs-unsichtbar.md`

Beide wohnten darin, dass das Feld `fenstermeldung` zwei verschiedene Sorten von Aussage trug: ein Ereignis, das niemand angefordert hat (der ausgeworfene Datenträger, die beschädigte Belegungsdatei), und die Antwort auf einen Tastenbefehl, den der Nutzer eben gemacht hat ("es läuft bereits eine Operation", "es ist nichts ausgewählt", der Abschlusstext eines Vorgangs). Die beiden haben verschiedene Lebensdauern und verschiedene Dringlichkeit; in einem Feld mit einem Rang ließen sie sich nicht beide halten.

Die Behebung trennt sie in zwei Felder. Die Rangfolge lautet seither:

```
1  Befehlsantwort    was KRK auf einen Tastenbefehl zu sagen hat
                     fällt mit dem nächsten Tastenbefehl
2  Vorgangsanzeige   der Stand einer laufenden Operation
                     fällt mit dem Bericht
3  Fenstermeldung    ein Ereignis am Fenster, das niemand angefordert hat
                     fällt beim nächsten Ordner- oder Tabwechsel
4  Tabmeldung        der Zustand des sichtbaren Ordners
```

Das ist dasselbe Ordnungsprinzip, das S16b und S14 schon nennen ("ein Ereignis ist neuer als ein Zustand", "eine laufende Operation ist noch neuer"), einen Schritt weitergeführt: die Antwort auf einen eben gedrückten Tastendruck ist neuer als beides. Die Regel steht in `crates/krk-ui/src/appkit/statuszeile.rs` als Funktion `zeile` und ist dort mit sechs Prüfungen belegt.

## Was am Plantext nachzuziehen ist

1. **Der Absatz "Die Statuszeile bekommt eine dritte Quelle, keine zweite Zeile."** Es sind vier Quellen. Der Grund, den der Absatz für das eigene Feld der Vorgangsanzeige nennt (entgegengesetzte Lebensdauern, ein Feld mit zwei Löschregeln wäre der Sonderfall), gilt wörtlich auch für die Trennung von Befehlsantwort und Fenstermeldung.

2. **Das `flowchart` mit sechs Knoten und fünf Kanten.** Es braucht eine Verzweigung mehr: acht Knoten, sieben Kanten. Die Angabe im Abschnitt `## Wie dieser Plan die Maxime "supersimpel" einlöst` beziehungsweise in der Diagrammzählung am Ende ist mitzuziehen.

3. **Der Satz "Verdrängt wird nichts endgültig. Eine Auswurfmeldung, die während einer Kopie eintrifft, steht in ihrem Feld und erscheint, sobald die Vorgangsanzeige endet; verzögert um die Laufzeit der Operation, verloren nicht."** Verloren ist sie nicht mehr, das hält. "Sobald die Vorgangsanzeige endet" hält nicht: in diesem Augenblick steht der Abschlusstext in der Zeile, der eine Befehlsantwort und damit höherrangig ist. Die Auswurfmeldung erscheint einen Tastenbefehl später, wenn der Abschlusstext nach seiner eigenen Löschregel fällt. Gemessen am 260804-1940 am laufenden Bündel. Der Satz sollte auf die Lebensdauer abstellen statt auf die Vorgangsanzeige: "erscheint, sobald alles über ihr gefallen ist".

4. **Der Satz "Der Abschlusstext, den `operationen::abschlusstext` schon heute liefert, geht danach als gewöhnliche Fenstermeldung in dieselbe Zeile; an diesem Weg ändert der Schritt nichts."** Er geht als Befehlsantwort in die Zeile, nicht als Fenstermeldung. Der Weg hat sich geändert; die Zusage darüber, was der Nutzer sieht, nicht.

5. **Das `stateDiagram-v2` der Vorgangsanzeige**, Übergang `Steht --> Leer: Bericht trifft ein, Abschlusstext wird Fenstermeldung`. Aus "Fenstermeldung" wird "Befehlsantwort". Zahl der Zustände und Übergänge bleibt.

6. **Zwei Sätze des Abnahmekriteriums.** "Wird während einer laufenden Kopie ein Datenträger ausgeworfen, erscheint die Auswurfmeldung, sobald der Fortschritt endet" ist nach Punkt 3 zu fassen. "Ein zweiter Operationsbefehl während eines laufenden Vorgangs meldet sich in der Zeile und startet nichts" hält jetzt wörtlich und braucht keine Änderung; das Wort "sichtbar" darin ausdrücklich zu machen, würde den Defekt vom 260804-1915 künftig am Kriterium selbst fangen.

**Der Plan gehört dem `planner`; dieser Defekt ändert ihn nicht.** Der Code ist der Stand, den der Plantext beschreiben soll, und nicht umgekehrt: die vier Ränge sind gebaut, gemessen und belegt.

**Aufgefallen bei:** der Behebung der beiden Defekte vom 260804-1915 am 260804-1940.

---
Resolved: Alle sechs genannten Stellen sind nachgezogen. Der Absatz nennt vier Quellen mit einer Rangtabelle samt Löschregeln; das `flowchart` hat 8 Knoten und 7 Kanten statt 6 und 5; der Verdrängungssatz stellt auf "sobald alles über ihr gefallen ist" ab und nennt die Messung vom 260804-1940; der Abschlusstext geht als Befehlsantwort in die Zeile; das `stateDiagram-v2` trägt die geänderte Beschriftung bei unveränderter Zahl von Zuständen und Übergängen; und das Abnahmekriterium sagt "sichtbar" ausdrücklich. Die Diagramm-Selbstprüfung am Ende des Plans trägt die neuen Zahlen. Nachgezogen am 260804-2318 vom `planner`.
