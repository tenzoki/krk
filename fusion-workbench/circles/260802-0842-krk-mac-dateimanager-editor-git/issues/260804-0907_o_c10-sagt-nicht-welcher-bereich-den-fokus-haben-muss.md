C10 sagt nicht, welcher Bereich den Fokus haben muss, damit die beiden Zwischenablage-Befehle greifen

---

Die beiden Funktionen aus C10 liegen seit S9b als gewöhnliche Belegungen in
`resources/default-keymap.toml`: `zwischenablage_ansehen` auf `shift+f3` und
`zwischenablage_springen` auf `opt+cmd+g`. Der Tastenabgriff aus Schritt 7
reicht jeden Tastendruck weiter, gleich welcher Bereich den Eingabefokus hat.
C10 sagt, "das aktive Dateifenster" wechsle in den Ordner, sagt aber nicht,
was gilt, wenn der Fokus in der Lesezeichen- und Geräteleiste oder im
Vorschaufenster liegt.

---

Das ist eine offene Frage und kein Defekt im engeren Sinn; der `ontocoder`
schreibt keine Entscheidungsdatensätze und legt sie deshalb hier ab. Der
nächste Abgleich sollte sie nach `decisions/` umtragen.

Warum sie zählt: C5 hat dieselbe Frage für seine eigenen Funktionen
ausdrücklich beantwortet, und die Antwort steht als Kommentar im C5-Abschnitt
von `resources/default-keymap.toml`: `delete` und `shift+f6` wirken nach C4 nur
im Dateifenster, deshalb trägt die Leiste eigene Kombinationen. Für C10 fehlt
die entsprechende Aussage, und C10 hat davon zwei Ausprägungen:

- **Beim Ansehen** ist die Frage womöglich gegenstandslos. Der Befehl füllt
  den aktiven Vorschau-Tab, und welcher Bereich den Fokus hat, ändert daran
  nichts.
- **Beim Sprung** ist sie es nicht. Das Ziel ist "das aktive Dateifenster".
  Liegt der Fokus in der Leiste oder in der Vorschau, ist offen, ob der Befehl
  ins zuletzt aktive Dateifenster springt, ob er nichts tut, oder ob er den
  Fokus zusätzlich zurückholt.

Die Belegungsdatei bindet das heute nicht: sie führt Kombinationen und keine
Fokusregeln. Gebraucht wird die Antwort in Schritt 13, der die Auswertung
baut, und in Schritt 19, der ihr Ergebnis anzeigt.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C10, C5),
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Schritt 13 und Schritt 19)
