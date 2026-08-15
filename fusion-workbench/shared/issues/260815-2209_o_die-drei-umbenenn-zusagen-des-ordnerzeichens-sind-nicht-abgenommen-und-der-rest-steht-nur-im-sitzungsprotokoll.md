Die drei Umbenenn-Zusagen des Ordnerzeichens sind nicht abgenommen, und der Rest steht nur im Sitzungsprotokoll

---

Der Entscheidungsdatensatz `260815-2056` steht auf `_i_` (umgesetzt) und ist damit endständig.
Zwei seiner drei Zusagen sind aber nur eine Stufe **hinter** der Taste nachgewiesen, mit
`doCommandBySelector:` an einem weggeworfenen Programm, und der Einstieg über den Klick ins
Feld ist auf keinem Weg gemessen. Wo das steht: in
`shared/history/260815-2110-coder-…`, Abschnitt `## Offen`. Ein Sitzungsprotokoll trägt
keinen Marker und fällt aus jeder Suche nach offener Arbeit heraus.

---

**Schwere:** niedrig. Kein Zustand am Code ist dadurch falsch; der `coder` hat die Lücke
sauber benannt. Falsch ist ihr Ablageort: `find … -name '*_o_*.md'` findet sie nicht, und
`_i_` sagt „realisiert", nicht „abgenommen".
**Gefunden von:** coderev, Durchsicht von `3b128c3`
**Betroffen:** `fusion-workbench/shared/decisions/260815-2056_i_…`,
`fusion-workbench/shared/history/260815-2110-coder-…`
**Domain:** code

## Was offen ist

| Zusage | Stand |
|---|---|
| 1: Beginn ohne Zeichen, gleich auf welchem Weg | am weggeworfenen Programm für den Tastenbefehl gemessen; **der Klick ins Feld nicht** — er braucht ein echtes Mausereignis |
| 2: Return liest den getippten Text | über `doCommandBySelector:` am Feldeditor nachgestellt, nicht über KRKs Ereignisabgriff |
| 3: Escape stellt die Anzeigeform her | ebenso |

Dass der Weg der Tasten durch den Ereignisabgriff trägt, ist am Baum abgelesen und nicht
gemessen: `ersthelfer_gehoert_appkit` reicht jeden Tastendruck an AppKit weiter, sobald der
Ersthelfer eine `NSTextView` ist (`ereignisse.rs:688-690`), und der Feldeditor ist eine.
Die Textfläche des eingebauten Editors, die eine Ausnahme davon, wird über Nämlichkeit
erkannt und ist es nicht. `inference:`, kein Beleg am laufenden Bündel.

## Vorschlag

Dieser Datensatz ist der Träger, bis der Nutzer die drei Punkte am laufenden Bündel prüft;
danach wird er geschlossen. Alternativ gehören die drei Zeilen auf die Abnahmeliste des
nächsten Laufs am Bündel, wo die übrigen ungemessenen Zusagen dieses Projekts schon stehen.
