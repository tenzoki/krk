Die Abnahmeliste rechnet gegen 72 Kriterien; der Spec führt nach dem C5-Nachtrag 75

---

Die Abnahmeliste vom 260814-1100 schließt mit dem Satz: „Zusammen mit den 43 Kriterien, die
der Baum trägt, und den 5, die einen Prüfaufbau brauchen, steht die Runde bei 71 von 72."

Die Grundmenge 72 stammt vom 260814-1002 und ist seit dem 260814-1010 überholt. **Derselbe
Commit `a6098d9`, der die zwei mittleren Befunde behob, hat drei Kriterien in die erste Liste
von C5 eingetragen** — die Begrenzung der Kopie, die Unterscheidbarkeit einer gekürzten
Sicherung und den Grenzfall von genau `EDITORGRENZE` Bytes. Die Abnahmeliste ist am 260814-1100
geschrieben und rechnet weiter mit der Zahl von davor.

**Am Dateibestand nachgezählt**, `260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md`
am Stand `a6098d9`, je Fähigkeit erste und zweite Liste:

| Fähigkeit | Am Baum | Am Bündel | Zeilen |
|---|---|---|---|
| C1 | 11 | 8 | 168–178, 181–188 |
| C2 | 4 | 5 | 203–206, 209–213 |
| C3 | 5 | 5 | 226–230, 233–237 |
| C4 | 12 | 7 | 250–261, 264–270 |
| C5 | **14** | 4 | 290–303, 306–309 |
| **Summe** | **46** | **29** | **75** |

**Zwei weitere Kriterien stehen außerhalb der fünf Fähigkeiten** und sind in keiner der beiden
Zahlen enthalten: die zwei unter `## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1`
(`:334`, `:335`). Der Spec sagt über sie ausdrücklich „Sie sind Bestandteil der Abnahme dieser
Runde". Wer sie mitzählt, kommt auf 77 mit 48 am Baum. Die Aufteilung des Abgleichs vom
260814-1002 hat sie ausgelassen, ohne es zu sagen; dieser Datensatz nennt beide Zahlen, damit
die nächste Erhebung die Wahl bewusst trifft.

---

**Schwere:** mittel. Keine Zeile Code, aber die Zahl trägt die Abschlussaussage der Runde.

**Warum es aufgeschrieben ist.** Die Abschlussnotiz einer Runde nennt eine Quote, und eine
Quote mit einem veralteten Nenner ist schlechter als keine: sie sieht nachgerechnet aus. Dieses
Projekt hat für Zähldefekte in Prosa schon fünf Datensätze abgelegt, und der Anlass ist hier
derselbe wie dort — eine Zahl steht an einer Stelle, die Sache an einer anderen.

**Was zu tun ist.** Die Grundmenge in der Abnahmeliste auf 75 ziehen und dabei festlegen, ob
die zwei Kriterien aus dem C8-Abschnitt mitzählen. Der Zähler ist ein eigener Befund, siehe
`260814-1247_o_sechzehn-der-neunundzwanzig-buendelkriterien-…`.

**Kontext**

- Gefunden beim zweiten Abgleich der Runde 9, `history/260814-1247-reconciliation.md`.
- Gezählt am Stand `a6098d9`, `grep -n '^- \[ \]'` über den Spec, Zeilen einzeln zugeordnet.
