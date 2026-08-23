`CLAUDE.md` sagt, die Version sei seit dem 260815 an jedem Tag gestiegen — am 260822 ist sie es nicht

---

Der Absatz `## Projektstand` von `CLAUDE.md` schreibt über die Versionszahl: „**seit dem 260815 ist
sie an jedem Tag mindestens einmal gestiegen**". Der Tagbestand widerlegt den Satz: am **2026-08-22**
steht kein Tag. Die Aussage ist am 260822 falsch geworden und durch die Auslieferung der 1.0.0 am
260823 nicht wieder wahr — eine Lücke in der Reihe schließt sich nicht dadurch, dass sie später
fortgesetzt wird.

---

**Am Baum gezählt, nicht abgeleitet.**

## Der Tagbestand

`git for-each-ref --sort=creatordate --format='%(creatordate:short) %(refname:short)' refs/tags`,
Baumstand `7d86420`:

| Tag | Tags |
|---|---|
| 2026-08-15 | `v0.4.0`, `v0.4.1`, `v0.4.3` |
| 2026-08-16 | `v0.4.4`, `v0.5.0` |
| 2026-08-17 | `v0.5.1` |
| 2026-08-18 | `v0.5.2` |
| 2026-08-19 | `v0.5.3`, `v0.5.4` |
| 2026-08-20 | `v0.5.5` |
| 2026-08-21 | `v0.5.6` |
| **2026-08-22** | **keiner** |
| 2026-08-23 | `v1.0.0` |

## Der Grund ist ein arbeitsfreier Tag und kein ausgelassener Auslieferungslauf

`git log --after=2026-08-21T23:59 --before=2026-08-23T00:00` liefert **null** Commits. Am 260822 ist
in diesem Baum überhaupt nichts geschehen. Die Aussage von `CLAUDE.md` unterstellt damit eine
Regelmäßigkeit, die der Baum nicht trägt und auch nicht tragen kann: sie hängt daran, dass an jedem
Kalendertag gearbeitet wird.

## Wann sie falsch wurde

Geschrieben hat sie `ab11eb8` vom 260821 („fuenf Aussagen in CLAUDE.md stehen wieder auf dem Stand
des Baums"). An jenem Tag war sie richtig. Falsch geworden ist sie am 260822 durch Unterlassung, und
das ist die Klasse, gegen die dieselbe Datei an anderen Stellen ausdrücklich schreibt: eine Zahl oder
eine Reihenaussage, die mit jedem Tag nachzuziehen wäre, veraltet zwischen zwei Lesern.

## Was die Stelle leisten soll

Der Satz steht als Begründung dafür, dass `CLAUDE.md` die Versionszahl **nicht** nennt, und diese
Begründung trägt weiter: die Zahl steht in `Cargo.toml` und im jüngsten Tag. Es ist allein der
Zusatz über die Tagesregelmäßigkeit, der eine prüfbare Behauptung aufstellt und sie nicht hält. Wer
ihn ersatzlos streicht, verliert nichts an der Begründung.

**Schwere:** Low. Kein Verhalten, keine Grundlage, keine Auslieferung hängt daran. Es ist eine
falsche Tatsachenbehauptung in der Datei, die jeder Agent zuerst liest.

**Gefunden:** reconciler, Abgleich zum Abschluss der Sitzung `260823-1424`, Baumstand `7d86420`

**Domain:** code

**Cross-references:** `CLAUDE.md` `## Projektstand`,
`shared/issues/260820-2056_o_claude-md-nennt-eine-zaehlprobe-unter-einem-namen-den-der-baum-nicht-traegt.md`,
`shared/issues/260823-1336_o_claude-md-nennt-einen-empfaenger-der-ersthelfermeldung-der-baum-traegt-seit-dem-260819-zwei.md`

---
Resolved:
