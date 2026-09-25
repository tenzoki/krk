# Welchen Umfang von Markdown rendert die Vorschau, verschachtelte Listen eingeschlossen?

---
**Domain:** code
**Status:** implemented
**Filed by:** orchestrator (auf Nutzerentscheid)
**Cross-references:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_welchen-umfang-von-markdown-rendert-die-vorschau.md` (von diesem Datensatz überholt); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1805_c_listen-verlieren-merkzeichen-nummerierung-und-verschachtelungstiefe.md`; `crates/krk-ui/src/markdown.rs`; `crates/krk-ui/src/appkit/textmerkmale.rs` (`einzugsmerkmal`)

---

## Question

Der Datensatz vom 260812-1000 hat den Umfang auf Möglichkeit 1 festgelegt und drei Dinge als
teuer ausgeschlossen: Tabellen, Bilder und **verschachtelte Listen**. Der Grund für den dritten
Ausschluss stand ausdrücklich dabei: verschachtelte Listen „brauchen eine Einrücktiefe, die die
vorhandene Auszeichnungsmechanik nicht kennt".

Beim Beheben des Defekts `260812-1805` ist dieser Grund weggefallen. `Auszeichnung::Listenzeile`
trägt seit `a9e1149` eine Tiefe, und `einzugsmerkmal` vervielfacht den Einzug damit. Der Aufwand
waren zwei Zeilen, nicht der befürchtete Umbau.

Damit stellt sich die Frage neu: bleibt es beim Ausschluss, obwohl seine Begründung nicht mehr
trägt, oder wird der Umfang um verschachtelte Listen erweitert?

## Options

1. **Erweitern.** Die Einrückung bleibt, wie sie gebaut ist.
   - Pros: eine dreistufige Liste sieht aus wie eine. Der Aufwand ist bezahlt, das Zurücknehmen
     kostete mehr als das Behalten.
   - Cons: der beschlossene Umfang wächst nachträglich, und zwar durch einen Defekt und nicht
     durch eine Planung.

2. **Zurücknehmen.** Nur Merkzeichen und Nummer bleiben, die Tiefe fällt heraus.
   - Pros: hält den beschlossenen Umfang genau ein.
   - Cons: eine dreistufige Liste steht als drei Zeilen mit demselben Einzug da, und es kostet
     Arbeit, etwas Funktionierendes wieder auszubauen.

## Constraints

- Die Einrückung ist bei acht Ebenen gedeckelt, also bei 160 Punkten. Das ist die Mindestbreite
  eines Bereichs; tiefer eingerückt bliebe für den Text nichts übrig.
- Tabellen und Bilder bleiben ausgeschlossen. Ihre Begründungen aus dem Vorgängerdatensatz sind
  **nicht** weggefallen: Tabellen brauchen eine Spaltenausrichtung über Tabulatorpositionen, und
  Bilder ließen die Vorschau beim Anzeigen einer Textdatei weitere Dateien von der Platte lesen,
  was L7 berührt.

## Antwort 260812

**Möglichkeit 1, erweitern.** Der Nutzer hat sie am 260812 gewählt, nachdem ihm der weggefallene
Grund und beide Möglichkeiten vorgelegt worden waren.

Der Umfang der gerenderten Vorschau ist damit: Überschriften, Betonung, starke Betonung,
Quelltext in Zeile und Block, Verweise, Zitatblöcke, **Listen mit Merkzeichen, Nummer und
Verschachtelungstiefe**. Alles Übrige erscheint als der Quelltext, der dasteht, und das gilt seit
`a9e1149` mechanisch statt für aufgezählte Fälle.

**Tabellen und Bilder bleiben draußen.** Ihre Begründungen tragen weiter, und dieser Datensatz
rührt sie nicht an.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812` — Nutzerentscheid vom 260812.
Implemented: `a9e1149` — `Auszeichnung::Listenzeile { tiefe }` in `crates/krk-ui/src/hervorhebung.rs`, gezählt in `crates/krk-ui/src/markdown.rs` aus den offenen Elementen, umgesetzt in `einzugsmerkmal` (`crates/krk-ui/src/appkit/textmerkmale.rs`), gedeckelt bei acht Ebenen.
Deferred:
Superseded by:
