Sechzehn der 29 Bündelkriterien sind vom Abnahmelauf nie berührt worden

---

Die Abnahmeliste vom 260814-1100 schließt: „Die 24 Abnahmekriterien mit Bündelanteil sind bis
auf den nicht festgehaltenen Messwert abgenommen." Sie führt **zwölf** Beobachtungen. Zwölf
Beobachtungen können 29 Kriterien nicht abnehmen, und die Zuordnung zeigt, welche leer
ausgehen.

**Jedes Kriterium der zweiten Listen gegen die zwölf Beobachtungen gelesen** (Spec am Stand
`a6098d9`, Beobachtungen aus `history/260814-1100-abnahmeliste-notizzettel.md`):

| Stand | Anzahl | Kriterien (Zeile im Spec) |
|---|---|---|
| **belegt** | 8 | 184, 209, 210, 211, 212, 213, 237, 308 |
| **teilweise berührt** | 5 | 181, 182, 183, 233, 236 |
| **nie berührt** | 16 | 185, 186, 187, 188, 234, 235, 264, 265, 266, 267, 268, 269, 270, 306, 307, 309 |

Die fünf teilweise berührten sind je zur Hälfte gesehen: `f2` und `cmd+k` sind gedrückt, aber
nicht aus jedem der fünf Bereiche (`:181`); `Esc` schließt, der zweite Druck auf `f2` ist
ungeprüft (`:182`); von „kein Befehl außer den dreien wirkt" sind zwei Befehle probiert
(`:183`); getippte Zeichen erscheinen, die Eingabetaste ist ungeprüft (`:233`); „leere
Textfläche" trägt „keine Zeilennummern" nur mittelbar (`:236`).

**Unter den sechzehn stehen die vier Kriterien, für die C3 die Textfläche prüfbar macht** —
`cmd+v`, `cmd+x`, `cmd+c`, `cmd+a`, `cmd+z` (`:234`) und die sieben abgeschalteten Automatiken
(`:235`). Die Automatiken sind am Baum über eine Zählprobe gedeckt, die Zwischenablage in der
Zettelfläche über nichts.

**Ebenso die drei Beenden-Kriterien von C4** (`:264`, `:265`, `:266`). Beobachtung 8 fährt
Beenden und Neustart, prüft aber, **welcher** Zettel offen ist, und nicht, ob der getippte Text
steht; getippt wird in Beobachtung 8 nicht.

---

**Schwere:** mittel. Nichts spricht dagegen, dass die sechzehn halten — es liegt nur kein Beleg
vor, in keine Richtung.

**Warum es aufgeschrieben ist.** Die Runde 8 hat die Grenze anders gezogen und deshalb sauber
schließen können: ihr Spec kennzeichnete jedes Kriterium einzeln mit `(Probe)` oder `(Bündel)`,
zehn trugen `(Bündel)`, und der Abnahmelauf führte elf Beobachtungen — eine je Kriterium und
eine für den Tag. Der Satz „alle 59 Abnahmekriterien abgenommen bis auf eines" war dort
nachrechenbar. Die Runde 9 führt zwei Listen je Fähigkeit statt einer Kennzeichnung je
Kriterium, und die 1-zu-1-Bindung zwischen Beobachtung und Kriterium ist dabei verloren
gegangen.

**Was zu tun ist.** Zweierlei, und das erste ist das billigere:

1. Die Abnahmeliste um die fehlenden Beobachtungen ergänzen und erneut fahren. Sechzehn
   Kriterien, von denen sich mehrere in einem Durchgang zusammenfassen lassen — die drei
   Beenden-Kriterien von C4 etwa in einem Lauf mit getipptem Text.
2. Für spätere Runden die Kennzeichnung je Kriterium aus der Runde 8 übernehmen statt zweier
   Listen. Dann zeigt die Abnahmeliste ihre Lücke selbst an.

**Kontext**

- Gefunden beim zweiten Abgleich der Runde 9, `history/260814-1247-reconciliation.md`.
- Vergleich mit der Runde 8: `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1037_c_spec-…` und deren `_c_circle.md`, Abschnitt `## Closure note`.
