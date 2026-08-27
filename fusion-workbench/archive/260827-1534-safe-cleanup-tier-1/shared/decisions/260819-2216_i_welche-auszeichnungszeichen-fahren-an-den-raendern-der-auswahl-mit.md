# Welche Auszeichnungszeichen fahren an den Rändern einer Auswahl mit?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `shared/decisions/260819-2216_*_was-landet-beim-gerenderten-markdown-in-der-zwischenablage.md` (die Antwort, aus der diese Frage entstanden ist); `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md` (C2.2, C2.8, C2.9); `crates/krk-ui/src/markdown.rs` (Modulkopf, Abschnitte „Die Regel der Zerlegung" und „Die Deckung"); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-2002_*_bleibt-der-vorspann-eines-containers-die-eine-luecke-in-der-deckungszusage-von-c4-3.md`

---

## Question

Der Nutzer hat am 260819-2210 entschieden, dass beim Kopieren aus gerendertem Markdown der Quelltext in die Zwischenablage geht. Beim Ausarbeiten des Specs zeigt sich, dass die Abbildung von der Auswahl auf den Quelltext **an den Rändern nicht eindeutig** ist, und diese Folge stand in der Optionsbeschreibung nicht.

Der Grund liegt in der Zerlegung. Zwischen dem gerenderten Text und der Quelle stehen zwei Arten von Zeichen, die keine Entsprechung haben: Quellzeichen ohne Anzeige, nämlich die Auszeichnungszeichen selbst und die Adresse eines Verweises, und Anzeigezeichen ohne Quelle, nämlich das Merkzeichen eines Listenpunkts, die Einrückung eines Zitatblocks und die Leerzeilen zwischen zwei Blöcken. Liegt ein solches Zeichen **zwischen** zwei ausgewählten Stellen, ist die Antwort klar: es fährt mit oder es fährt nicht mit, je nachdem, welcher Art es ist. Liegt es **am Rand** der Auswahl, gibt es zwei vertretbare Antworten.

Ein Beispiel. Die Quelle lautet

```markdown
Ein **fetter** Text mit [Verweis](https://example.com) darin.
```

und der gerenderte Text lautet `Ein fetter Text mit Verweis darin.` Markiert der Nutzer darin `fetter Text mit Verweis`, dann liefert eine zeichenweise Abbildung

```
fetter** Text mit [Verweis
```

also eine offene Betonung am Anfang und einen Verweis ohne Adresse am Ende. Wer denselben Absatz in eine andere Markdown-Datei einfügt, bekommt kaputtes Markdown.

## Options

1. **Zeichenweise.** Kopiert wird der Quellausschnitt vom ersten bis zum letzten ausgewählten Zeichen, wörtlich.
   - Folge: die einfachste Regel, ohne einen Begriff von „Element". Was zwischen zwei ausgewählten Zeichen liegt, fährt mit; was davor und dahinter steht, nicht.
   - Preis: das Beispiel oben. Eine Auswahl innerhalb einer Überschrift liefert den Text ohne Doppelkreuz, eine Auswahl innerhalb eines Verweistextes den Text ohne Adresse. Dazu braucht die Zusage „alles auswählen liefert die Datei vollständig" (C2.8) eine eigene Regel, weil ein Merkzeichen am Dateianfang vor dem ersten Quellzeichen des gerenderten Textes liegt und herausfiele. Eine Sonderregel für den Randfall ist die Sorte Ausnahme, die dieses Projekt an anderer Stelle vermeidet.

2. **Eine berührte Auszeichnung fährt ganz mit.** Der Quellausschnitt wird so lange erweitert, bis er kein Element mehr nur zur Hälfte enthält.
   - Folge: die Auswahl aus dem Beispiel liefert `**fetter** Text mit [Verweis](https://example.com)`. Eine Auswahl innerhalb einer Überschrift liefert `# Überschrift`, eine innerhalb eines Verweistextes den ganzen Verweis mit Adresse. Das Ergebnis ist immer wohlgeformtes Markdown, und C2.8 fällt ohne Sonderregel heraus: bei einer Auswahl über alles ist jedes Element ganz enthalten.
   - Die Regel ist entscheidbar und vollständig: erweitere den Ausschnitt auf die Vereinigung der Quellbereiche aller Elemente, die er nur teilweise überdeckt, und wiederhole, bis er sich nicht mehr ändert. Der Ausschnitt wächst dabei nur, die Quelle ist endlich, also endet das Verfahren; über verschachtelte Elemente ist es dasselbe Verfahren und keine zweite Regel.
   - Preis: der kopierte Text kann länger sein als der markierte. Wer drei Buchstaben mitten in einer fetten Stelle markiert, bekommt die ganze fette Stelle. Die Abbildung braucht die Elementgrenzen und nicht nur die Stellen, also mehr, als der Durchgang heute festhält. Er hat sie: `pulldown_cmark::OffsetIter` liefert zu jedem Ereignis den Quellbereich.

3. **Blockweise.** Jeder Block, den die Auswahl berührt, fährt vollständig mit: der ganze Absatz, die ganze Überschrift, der ganze Listenpunkt.
   - Folge: die gröbste und am leichtesten zu erklärende Regel. Immer wohlgeformtes Markdown, und die Abbildung braucht nur die Blockgrenzen.
   - Preis: wer zwei Wörter eines langen Absatzes markiert, bekommt den Absatz. Die Auswahl, die der Nutzer sieht, und das, was er bekommt, haben dann wenig miteinander zu tun; die Markierung wäre eher eine Zeigegeste als eine Auswahl.

## Constraints

- Die Abbildung ist total: jede Stelle des gerenderten Textes hat eine Antwort, die erzeugten Zeichen ohne Quelle eingeschlossen.
- Sie entsteht im Durchgang, der rendert, und nicht in einem zweiten danach.
- Bei einer Auswahl über den ganzen Text liegt die Quelldatei vollständig in der Zwischenablage (C2.8).
- Der Vorspann eines Containers ist die eine bekannte Lücke der Deckung. Für das Kopieren dreht sich ihr Vorzeichen: was die Anzeige weglässt, gehört in die Zwischenablage.

## Recommendation

**Wir empfehlen Möglichkeit 2.** Sie ist die einzige, die ohne Sonderregel wohlgeformtes Markdown liefert und C2.8 miterledigt, und ihr Verfahren ist über verschachtelte Elemente dasselbe wie über einfache. Der Preis, mehr zu bekommen als markiert, trifft die kleinen Auswahlen innerhalb einer Auszeichnung; das ist der seltenere Fall gegenüber dem Kopieren eines Absatzes oder einer Zeile, und er ist sichtbar, während ein verlorener Verweis es nicht ist.

Möglichkeit 3 empfehlen wir nicht: sie ist billiger als 2, aber der Unterschied zwischen dem, was der Nutzer markiert, und dem, was er bekommt, wäre am Bündel dauernd zu sehen.

## Antwort 260819-2242

**Möglichkeit b.** Eine berührte Auszeichnung fährt ganz mit.

Wer drei Buchstaben einer fetten Stelle markiert, bekommt die ganze fette Stelle. Der Preis ist benannt und angenommen: die Auswahl liefert mehr, als sie umschließt. Dafür ist der Quelltext in der Zwischenablage immer wohlgeformt und lässt sich woanders einfügen, und "Alles auswählen liefert die Datei vollständig" braucht keine Sonderregel.

---
Answered: dieser Datensatz, Abschnitt `## Antwort` — Klärungsrunden des Orchestrators mit dem Nutzer am 260819; Sitzungsprotokoll `shared/history/260819-2026-orchestrator-session.md`. Ausformuliert im Spec `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md`.
Implemented: `91f8727`, `05cb614` — die Fixpunktregel steht als `Quellbezug::klammern_schliessen` (`crates/krk-ui/src/markdown.rs:434`), die Klammer selbst entscheidet `klammer_der_raender` (`:991`) an Vorspann und Nachspann eines Elements. Die Wurzelbehebung `05cb614` hat den Begriff von "irgendwo verdeckte Bytes" auf "Zeichen an den Raendern" zurueckgefuehrt, nachdem zwei Durchsichtsbefunde beide Richtungen desselben Fehlers gemessen hatten. Proben: `ueberschrift_betonung_verweis_und_punkt_tragen_eine_klammer_ein_absatz_nicht` (`:2595`), `eine_entitaet_oder_ein_escape_im_absatz_blaeht_die_auswahl_nicht_auf` (`:2774`), `eine_ueberschrift_mit_einem_kind_am_anfang_behaelt_ihr_doppelkreuz` (`:2805`). Abgeglichen am 260820-0834.
Deferred:
Superseded by:
