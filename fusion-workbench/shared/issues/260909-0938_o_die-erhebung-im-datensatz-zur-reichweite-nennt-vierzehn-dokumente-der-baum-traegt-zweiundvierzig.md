Die Erhebung im Datensatz zur Reichweite nennt vierzehn Dokumente, der Baum trägt zweiundvierzig

---

`260907-2340_*_wie-weit-reicht-die-neue-regel-fuer-den-zustand-eines-anforderungsdokuments-in-den-bestand-zurueck.md`
sagt unter `## Frage`: „sieben Dokumente stehen auf offen, fünf auf erledigt, zwei auf ‚in
Arbeit'". Das sind vierzehn. Derselbe Absatz erklärt vorher alle 42 Anforderungsdokumente und
Pläne zum Gegenstand der Erhebung. Die Zahlen decken den genannten Bestand nicht ab, und keine
Lesart bringt sie zur Deckung: 7+5+2 ist weder 42 noch eine benannte Teilmenge davon.

---

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** beim Vorlegen der Frage, während der Prüfung ihrer Erhebung gegen den Baum.

## Gemessen am 260909-0938

```sh
find fusion-workbench/shared/planning fusion-workbench/circles/*/planning \
  -maxdepth 1 -name '*.md' | while IFS= read -r f; do
    basename "$f" | sed -nE 's/^[0-9]{6}-[0-9]{4}_([a-z])_.*/\1/p'
  done | sort | uniq -c
```

26 auf `_c_`, 12 auf `_o_`, 4 auf `_p_`, zusammen 42. In `shared/planning/` allein: 1 auf
`_c_`, 4 auf `_o_`, 1 auf `_p_`, zusammen sechs. Die Angabe „sechs davon liegen in
`shared/planning/`" im selben Absatz stimmt also; die Dreierzählung daneben nicht.

## Was daran zählt

Die Zahlen tragen die Begründung der Frage: sie sollen zeigen, wie groß der widersprüchliche
Bestand ist, über dessen Behandlung der Nutzer entscheidet. Möglichkeit 1 des Datensatzes
argumentiert ausdrücklich mit „sieben Posten, an denen niemand arbeitet". Der Baum trägt
sechzehn solche Posten, nicht sieben, und `260906-0212_*_sechzehn-plan-und-specdateien-geschlossener-runden-stehen-auf-offen-oder-in-arbeit.md`
führt sie namentlich. Wer nach den Zahlen des Datensatzes entscheidet, entscheidet über einen
kleineren Bestand, als der Baum hat.

## Abnahme

Der Absatz `## Frage` nennt die Zahlen, die der Baum trägt, oder er nennt die Teilmenge, auf
die sich seine Dreierzählung bezieht. Die Nebenrechnung in Möglichkeit 1 zieht mit.
