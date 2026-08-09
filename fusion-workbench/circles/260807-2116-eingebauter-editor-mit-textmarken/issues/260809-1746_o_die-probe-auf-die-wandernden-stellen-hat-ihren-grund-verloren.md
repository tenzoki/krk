# Die Probe auf die wandernden Stellen hat mit S2 ihren Grund verloren

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, bei der Umsetzung von S2
**Betroffen:** `crates/krk-core/tests/belegung.rs`, Probe `keine_neue_kombination_liegt_auf_den_beiden_wandernden_stellen`; `planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md`, Befund 4 und das Abnahmekriterium von S6
**Cross-references:** `issues/260809-1527_o_der-plan-verbietet-y-und-z-und-legt-rueckgaengig-selbst-auf-cmd-z.md` (dieselbe Planstelle, offene Nutzerentscheidung), `decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`

---

## Der Befund

Die Probe `keine_neue_kombination_liegt_auf_den_beiden_wandernden_stellen` sagt
zu, dass keine Kombination, die KRK selbst zustellt, auf `kVK_ANSI_Y` oder
`kVK_ANSI_Z` liegt. Ihre Begründung war: KRK schlägt Buchstaben über die
**Stelle** nach, und diese beiden Stellen tauschen zwischen der deutschen und
der amerikanischen Tastaturbelegung den Platz.

Seit S2 schlägt KRK Buchstaben über das gemeldete **Zeichen** nach. Damit wandert
keine Stelle mehr: `cmd+y` liegt auf jeder Tastaturbelegung unter der Aufschrift
Y. Die Begründung der Probe ist weg, ihre Zusage steht noch.

Dasselbe gilt für die beiden Planstellen, aus denen sie stammt: den Satz in
Befund 4 ("Kein neuer Tastenbefehl dieser Runde liegt auf `y` oder `z`") und das
Abnahmekriterium von S6.

## Warum das zählt

Die Probe verbietet künftigen Runden zwei Buchstaben ohne einen Grund. Sie ist
nicht falsch, sie ist gegenstandslos, und eine gegenstandslose Zusage im
Prüfbestand kostet später eine Sitzung: wer eine neue Funktion auf `cmd+y` legen
will, findet eine rote Probe und keine Erklärung, die noch trägt.

## Was zu tun ist

Zusammen mit
`issues/260809-1527_o_der-plan-verbietet-y-und-z-und-legt-rueckgaengig-selbst-auf-cmd-z.md`
erledigen, weil beide dieselbe Planstelle betreffen und dort eine Entscheidung
des Nutzers aussteht:

1. Den Satz in Befund 4 und das Abnahmekriterium von S6 auf den Stand nach S2
   ziehen: die Einschränkung auf `y` und `z` entfällt ersatzlos.
2. Die Probe entfernen. Was sie noch trug, tragen seither
   `auf_einer_deutschen_tastatur_findet_die_aufschrift_y_die_vorschau` und
   `jede_ausgelieferte_kombination_traegt_die_kennung_ihrer_tastensorte`
   in derselben Datei, und zwar an der Sache selbst statt an einer Vorsichtsregel.

S2 hat die Probe stehen lassen und ihren Doc-Kommentar auf den heutigen Stand
gezogen, damit im Prüfbestand keine falsche Begründung steht. Sie
wegzunehmen wäre das Vorwegnehmen einer Entscheidung, die noch aussteht.
