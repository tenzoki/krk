# Wer vergibt die Versionszahl, und darf ein Agent den Auslieferungslauf fahren?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260813-1534_*_darf-das-bauwerkzeug-den-tag-setzen-und-die-auslieferung-in-einem-kommando-fahren.md; 260821-1115_*_bekommt-der-veroeffentlichungsbefehl-eine-eigene-huelle-wie-certify-only-sh.md

---

## Question

`CLAUDE.md` sagt in seinem Abschnitt zur Auslieferung: „der Nutzer wählt allein die Zahl, im Argument von `./release.sh <version>`". Der Satz steht dort als Festlegung und wird auch so gelesen: eine Sitzung am 260918 hat deshalb zweimal die Versionszahl beim Nutzer erfragt, statt sie aus den Stufen abzuleiten.

**Der Beleg trägt die Behauptung nicht.** Der zitierte Entscheid `260813-1534_*` beantwortet eine andere Frage, nämlich ob das Bauwerkzeug den Tag setzen und die Auslieferung in einem Kommando fahren darf. Über die Herkunft der Zahl sagt er nichts. `README.md` formuliert im Abschnitt „Versionsstufen" vorsichtiger: „wer `./release.sh 0.2.0` tippt, hat die Zahl gewählt" — eine Beschreibung des Wegs, keine Bindung an eine bestimmte Hand.

Damit stehen zwei Fragen offen, die bisher niemand gestellt hat. Erstens: muss die Zahl von einem Menschen kommen, oder genügt es, dass sie den Stufen folgt? Zweitens: darf ein Agent den Auslieferungslauf fahren, dessen achte Station eine öffentliche Releaseseite anlegt und Tag und HEAD zur Gegenseite schiebt — eine Wirkung, die sich nicht zurücknehmen lässt?

## Options

1. **Die Zahl folgt den Stufen, und wer sie vergibt, ist gleichgültig. Der Auftrag zur Auslieferung ist der bewusste Akt, nicht das Tippen der Zahl.** Ein Agent leitet sie aus `README.md` `### Versionsstufen` ab, nennt sie, bevor etwas läuft, und fährt den Lauf auf ausdrücklichen Auftrag.
   - Pro: die Stufen sind an den Flächen des Projekts entscheidbar, nicht Geschmackssache; eine Ableitung daraus ist prüfbar, eine Wahl aus dem Bauch nicht. Der Schutz vor einer ungewollten Veröffentlichung sitzt dort, wo er wirkt, nämlich am Auftrag.
   - Contra: wendet ein Agent die Stufen falsch an, trägt eine öffentliche Releaseseite die falsche Zahl, und ein vergebener Tag wird nie verschoben.
2. **Die Zahl bleibt beim Menschen**, so wie `CLAUDE.md` es heute behauptet.
   - Pro: die Zahl steht auf jeder Releaseseite und überlebt das Projekt.
   - Contra: hält eine Bindung aufrecht, die kein Entscheid je getroffen hat, und verlangt bei jeder Auslieferung eine Rückfrage, deren Antwort aus den Stufen schon feststeht.
3. **Die Zahl darf ein Agent vergeben, den Lauf fährt nur der Mensch.**
   - Pro: trennt die prüfbare Ableitung von der unwiderruflichen Wirkung.
   - Contra: der Mensch tippt dann ein Kommando, dessen einziges Argument ihm gerade genannt wurde; die Trennung kauft nichts, was der Auftrag nicht schon leistet.

## Constraints

- Die Stufen in `README.md` `### Versionsstufen` bleiben die eine Quelle dafür, welche der drei Zahlen steigt. Diese Antwort ändert an ihnen nichts.
- Ein vergebener Tag wird nie verschoben; ein Fehllauf hinterlässt ihn, und er ist von Hand abzuräumen.
- Station 8 ist die einzige Wirkung der Kette, die über das Gerät hinausgeht. Der Schutz davor ist der ausdrückliche Auftrag, nicht die Herkunft der Zahl.
- Der Entscheid `260813-1534_*` bleibt unberührt und wird **nicht** überholt: er beantwortet, ob das Werkzeug taggen darf, und diese Antwort steht weiter.

## Recommendation

Möglichkeit 1.

---
Answered: Der Nutzer am 260918-0842 wörtlich: „ändere die regel so, dass sie sinn macht, natürlich kannst die release nummer nach der definierten vorgabe vergeben und das release dann auch ausführen, inklusive apple zertifizierung etc." Damit gilt Möglichkeit 1: ein Agent leitet die Zahl aus den Stufen ab, nennt sie vor dem Lauf, und fährt die Auslieferung samt Beglaubigung und Veröffentlichung auf ausdrücklichen Auftrag. Ruled by user, Kai Stalmann <kai@stalmann.org>.
