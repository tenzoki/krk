# Was heißt „immer gelistet" für .secrets.txt, wenn die Dateiliste versteckte Einträge ab Werk ausblendet?

---
**Domain:** code
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260925-2356-f2-oeffnet-krkhome-statt-notizfenster.md, 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md

---

## Question

`.secrets.txt` ist wegen des führenden Punkts ein versteckter Eintrag (`crates/krk-core/src/verzeichnis/eintrag.rs`). Die Dateiliste blendet versteckte Einträge ab Werk aus; `shift+cmd+h` schaltet das je Tab um (`Ordnermodell::verstecke_ausblenden_setzen` in `crates/krk-core/src/verzeichnis/modell.rs`). Ob eine Zeile steht, entscheidet dort genau ein Prüfschritt, und dessen erster Zweig lautet „versteckt und ausgeblendet, fällt weg". Die Directive sagt, die Datei sei „hier dennoch immer gelistet". Offen ist, was „immer" gegenüber dem Umschalter bedeutet und ob es auch gilt, wenn der Nutzer `~/krkhome/` über ein Lesezeichen oder `cmd+t` statt über F2 erreicht.

## Options

1. **In `~/krkhome/` steht `.secrets.txt` immer, gleich wie der Umschalter steht und wie der Nutzer dorthin kam.** Andere versteckte Einträge dort (etwa `.DS_Store`) folgen weiter dem Umschalter.
   - Pros: genau „immer"; der Nutzer findet die Datei in jedem Tab, der den Ordner zeigt.
   - Cons: der eine Prüfschritt der Sichtbarkeit bekommt eine Ausnahme für einen Pfad. Das ist die erste Ausnahme dieser Art, und sie ist ausdrücklich zu begründen, damit sie nicht die Vorlage für weitere wird.
2. **Der Tab, den F2 öffnet, beginnt mit eingeblendeten versteckten Einträgen.** Der Prüfschritt bleibt, wie er ist; der Nutzer kann im Tab mit `shift+cmd+h` umschalten.
   - Pros: keine Ausnahme im Prüfschritt; nutzt den bestehenden Umschalter je Tab.
   - Cons: „immer" gilt nur für den F2-Tab und nur, bis der Nutzer umschaltet. In diesem Tab erscheinen auch andere versteckte Einträge wie `.DS_Store`. Wer den Ordner auf anderem Weg öffnet, sieht die Datei nicht.
3. **Die Datei heißt ohne Punkt, etwa `secrets.txt`.**
   - Pros: keine Sonderregel, sie steht wie jede andere Datei.
   - Cons: weicht von der Directive ab, die die Datei ausdrücklich „unsichtbar" haben will, also außerhalb von KRK (Finder, `ls`) nicht auf den ersten Blick sichtbar.

## Constraints

- Der Prüfschritt der Sichtbarkeit steht an genau einer Stelle und soll dort bleiben; eine zweite Sichtbarkeitsregel daneben wäre eine zweite Wahrheit über dieselbe Frage.
- Die Datei bleibt im Finder unsichtbar, gleich welche Möglichkeit gewählt wird, außer bei Möglichkeit 3.

## Recommendation

Wir empfehlen Möglichkeit 1, mit der Ausnahme als benannter Eigenschaft des Ordners `~/krkhome/` und nicht als Namensregel: „KRKs eigene Dateien in diesem Ordner stehen immer". Sie ist die einzige, die das Wort „immer" der Directive einlöst. Möglichkeit 2 wäre sauberer im Code, liefert aber ein „immer" mit zwei Einschränkungen, die der Nutzer nicht bestellt hat.

---
Answered: dieser Datensatz `## Options` und 260926-0017-zweitlesung-spec-f2-krkhome.md — Möglichkeit 1, als Eigenschaft des Ordners beim Lesen gesetzt, ohne das Kennzeichen `versteckt` umzustellen; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: 6f50611 — Ausnahme als Ordnereigenschaft im Zweig der versteckten Einträge
