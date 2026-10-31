Ein einmal umbrochener Text „bleibt“ nicht, „wie er ist“: Format löst die Fortsetzungszeile ab, und nach Strip läuft die erste Zeile als eigener Befehl
---
`HowTo.md` und der Modulkopf von `glaetten.rs` beschreiben den hingenommenen Preis der Breitenregel aus `b21ca032` so: ein einmal umbrochener Text hat nur eine volle Zeile und „bleibt, wie er ist“. Das stimmt für keine der zwei Umformungen. Format nimmt der Fortsetzungszeile die Einrückung und macht sie damit zu einem eigenen Absatz, den auch ein zweites Format nicht mehr verbindet. Strip nimmt die Kopiereinrückung und lässt zwei Zeilen stehen. Eingefügt läuft die erste Zeile als vollständiger Befehl, ohne ihre Fortsetzung.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** `261031-1156-die-laengste-zeile-gilt-immer-als-umgebrochen-und-strip-verschluckt-befehle-format-tabellen.md`, `261031-1226-reviewer-nachdurchsicht-quicknote-behebungen.md`

**Ort:** `crates/krk-core/src/text/glaetten.rs`, `fliesstext` (Zeile 221: eine nicht verbundene Zeile wird als `text` ohne Einrückung ausgegeben) bei `belegte_breite == 0`. Beschrieben in `glaetten.rs:43-44` und `:54-56` sowie in `HowTo.md`, Liste unter „Ob ein Zeilenumbruch vom Terminal stammt“ (erster Punkt: „bleibt, wie er ist“).

**Belege:** gemessen am Stand `c1c2b7f8`, Wegwerfprogramm außerhalb des Baums wie im Issue A1:

```
Format, Eingabe (ein Punkt aus dem Beispiel des Nutzers, allein ausgewählt):
- Jede englische Textdatei hat Name, … ein Framework zur
    Kundenlage findet; nachgetragen in c3cabfcb5).
Format, Ausgabe:
 - Jede englische Textdatei hat Name, … ein Framework zur
Kundenlage findet; nachgetragen in c3cabfcb5).
```

Dasselbe mit dem ersten Absatz des Beispiels, wenn er allein auf zwei Zeilen steht. Die Statuszeile meldet dabei kein „nichts zu ändern“, denn der Text hat sich geändert. Format wirkt nach `HowTo.md:1213` auf die Auswahl. Wer einen einzelnen Absatz oder Punkt auswählt, trifft also genau diesen Fall.

```
Strip, Eingabe:
git add -A && git commit -m 'quicknote: strip und format' && git push --force-with-lease
  origin feature/quicknote-strip-und-format
Strip, Ausgabe:
git add -A && git commit -m 'quicknote: strip und format' && git push --force-with-lease
origin feature/quicknote-strip-und-format
```

Eingefügt und bestätigt führt die Shell `git push --force-with-lease` ohne Ziel aus, danach `origin …` als unbekannten Befehl. `HowTo.md` sagt „Vor dem Einfügen ins Terminal von Hand verbinden“. Dass die Ausgabe dann eine andere Wirkung hat als gemeint, sagt der Satz nicht. Weil Strip die Einrückung nimmt und damit etwas ändert, sieht das Ergebnis wie ein bearbeiteter Befehl aus.

Befehle zwischen einer und zwei Terminalbreiten werden genau einmal umbrochen. inference: Das ist bei kopierten Befehlen die häufigste Klasse mit Umbruch. Nicht gezählt. Das alte Strip-Beispiel in `HowTo.md` war ein solcher Befehl und ist in `b21ca032` durch einen zweimal umbrochenen ersetzt worden.

**Richtung:** Für Format lässt eine Zeile, die nur deshalb nicht verbunden wird, weil keine Breite belegt ist, ihre Einrückung stehen. Dann bleibt der Text tatsächlich, wie er ist, und ein Format über den ganzen Text, der die Breite belegt, verbindet ihn später noch. Will man das nicht, dann beschreiben `HowTo.md` und der Modulkopf das Ablösen so, wie es geschieht. Für Strip beschreibt `HowTo.md`, was die zweizeilige Ausgabe beim Einfügen tut.

**Abnahme:**
- Eine Probe in `crates/krk-core/tests/glaetten.rs` hält, was Format mit einem einmal umbrochenen Punkt und einem einmal umbrochenen Absatz macht, jeweils allein als Eingabe.
- `HowTo.md` und der Modulkopf von `glaetten.rs` beschreiben diesen Fall für Format und für Strip so, wie die Probe ihn festhält, und für Strip auch, dass die erste Zeile eingefügt als eigener Befehl läuft.
- `eine_einzelne_lange_zeile_belegt_keinen_umbruch`, `strip_ist_idempotent` und `das_beispiel_des_nutzers_wird_zeichen_fuer_zeichen_zur_erwarteten_ausgabe` bleiben grün. Die Erwartung der ersten ändert sich nur, wenn der Schritt das nennt.

**Resolved:** 261031, fixed in `f39e14fb`: Format behaelt einer nicht verbundenen Fortsetzung die Einrueckung, Singlestrip verbindet einen einmal umbrochenen Befehl.
