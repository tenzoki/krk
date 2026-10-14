# Folgt das französische Papierkorb-Verb dem Glossar oder dem Finder, und bekommt die Markierung in fr und en ein eigenes Wort neben der Marke?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261014-1640-reviewer-durchsicht-der-zwei-uebersetzten-sprachtabellen-eintrag-fuer-eintrag.md, 261014-1640-fuenf-begriffe-stehen-in-fr-rs-in-zwei-schreibungen-papierkorb-verb-zuruecksetzen-haelt-eine-datei-zu-gross-editeur-apercu.md, 261014-1640-das-glossar-in-tabelle-mod-rs-laesst-wiederkehrende-begriffe-aus-und-markierung-und-marke-fallen-in-fr-und-en-auf-ein-wort.md

---

## Question

Die Durchsicht der zwei übersetzten Sprachtabellen vom 261014 hat zwei Stellen gefunden, an denen die Behebung eine Wahl verlangt, die über den Befund hinausgeht. Erstens nennt `fr.rs` den Vorgang „in den Papierkorb räumen“ an fünf Stellen „mettre à la Corbeille“, wie das Glossar in `tabelle/mod.rs` es vorgibt, und an zwei Stellen „placer dans la Corbeille“, wie der Finder den Befehl auf Französisch nennt; welche der zwei Fassungen bleibt, ist eine Frage an den, der die Oberfläche liest, nicht an den Übersetzer. Zweitens tragen `fr.rs` und `en.rs` für die „Markierung“ eines Eintrags und für die „Marke“ der Git-Spalte dasselbe Wort (marque / mark), während Deutsch beide trennt; ob die Markierung ein eigenes Wort bekommt, ändert mehrere nutzersichtbare Befehlsnamen und die Statuszeile. Das Arbeitspaket läuft im autonomen Modus, und die Behebung soll nicht auf die Antwort warten: sie setzt die Fassung ein, die das Glossar heute vorgibt, und dieser Datensatz hält fest, dass das eine Vorgabe und keine Entscheidung des Nutzers war.

## Options

1. **Glossar-Fassung behalten und beide Begriffe wie gesetzt lassen (angewandt)** — „mettre à la Corbeille“ an allen sieben Stellen; „marque“ / „mark“ bleiben für Markierung und Marke, und das Glossar benennt den Zusammenfall ausdrücklich.
   - Pro: eine Fassung je Begriff, keine weitere Änderung an nutzersichtbaren Befehlsnamen; das Glossar entscheidet, wie sein Kopf es verlangt.
   - Contra: wer den Finder gewohnt ist, liest beim Papierkorb ein anderes Verb; „3 marques masquées“ neben der Spalte „Marque“ bleibt zweideutig.
2. **Finder-Wortlaut beim Papierkorb** — „placer dans la Corbeille“ an allen sieben Stellen und im Glossar.
   - Pro: deckungsgleich mit dem Systemwortlaut, den ein französischer Mac-Nutzer täglich sieht.
   - Contra: zwei weitere Stellen und das Glossar ändern sich noch einmal; die Erfolgsmeldungen („Mise à la Corbeille“) brauchen eine neue Nomenform.
3. **Eigenes Wort für die Markierung** — etwa fr „repère“, en „tag“ (beide in den Tabellen noch frei), an der Statuszeile des Filters und an den zwei Befehlen „Alle Markierungen aufheben“ / „Markierung umkehren“, mit Glossarzeile.
   - Pro: die Spalte „Marque“ / „Mark“ und die Markierung sind auf einen Blick zu trennen, wie im Deutschen.
   - Contra: „tag“ ist im Englischen durch Git-Tags belegt, die der Git-Bereich einmal zeigen könnte; „repère“ ist für eine Auswahlmarkierung unüblich.

Die Möglichkeiten 2 und 3 schließen einander nicht aus.

## Constraints

Jede Fassung steht in allen drei Tabellen unter demselben Schlüssel, und `keine_zwei_funktionen_tragen_in_einer_sprache_denselben_namen` bleibt grün. Was das Glossar festlegt, trägt jede genannte Stelle; eine halbe Umstellung ist der Befund, den die Durchsicht gerade geschlossen hat.

## Recommendation

Möglichkeit 1 ist angewandt, weil sie keine neue Wahl trifft, sondern die vorhandene durchsetzt. Wer 2 oder 3 will, nennt es; die Stellen stehen in den zwei zitierten Befunddatensätzen.
