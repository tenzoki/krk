# Darf Text aus `.secrets.txt` in die Zwischenablage kopiert werden?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-0007_*_wie-wird-secrets-txt-verschluesselt-und-was-schuetzt-eine-vierstellige-pin.md, 260926-0017-zweitlesung-spec-f2-krkhome.md

---

## Question

Das Bedrohungsmodell des Nutzers für `.secrets.txt` ist allein das versehentliche Lesen durch Agenten, Werkzeuge und Indexer. Die Zweitlesung benennt einen Leseweg, den die Verschlüsselung nicht schließt: kopierter Text liegt im Klartext in der Zwischenablage, und ein Agent kann sie mit `pbpaste` lesen.

## Options

1. **Kopieren erlauben.** Der Nutzer entscheidet selbst, was er kopiert; `HowTo.md` nennt das Risiko.
   - Pros: keine Sonderregel im Editor, Geheimnisse lassen sich in andere Anwendungen übernehmen.
   - Cons: kopierter Klartext ist für jeden Prozess unter dem Konto lesbar, bis er überschrieben wird.
2. **Kopieren sperren.** `cmd+c` und `cmd+x` wirken im Editor nicht, solange `.secrets.txt` offen ist.
   - Pros: der Leseweg ist zu.
   - Cons: Geheimnisse müssen abgetippt werden; eine Sonderregel in der Zulässigkeit des Editors.

## Constraints

- Es gibt genau eine Hülle um `NSPasteboard` (`appkit/zwischenablage.rs`); eine Sperre dürfte keine zweite daneben bauen.

## Recommendation

Möglichkeit 1: das Kopieren ist eine bewusste Handlung des Nutzers und kein versehentliches Lesen.

---
Answered: dieser Datensatz `## Recommendation` — Möglichkeit 1, Kopieren bleibt erlaubt, die Anleitung nennt das Risiko; ruled by user, Kai Stalmann <kai@stalmann.org>
