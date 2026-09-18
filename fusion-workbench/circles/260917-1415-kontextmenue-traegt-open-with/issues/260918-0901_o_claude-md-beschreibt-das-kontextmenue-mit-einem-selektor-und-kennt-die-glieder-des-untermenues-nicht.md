`CLAUDE.md` beschreibt das Kontextmenü mit einem Selektor und kennt die Glieder des Untermenüs nicht

---

Der Absatz „Seit der Runde 17 führt ein zweiter Weg in die Anwendung hinein, und er hat keine Taste" (`CLAUDE.md`, Abschnitt „Was man nicht sieht, wenn man es nicht weiß") beschreibt die eigenen Einträge des Kontextmenüs als eine geschlossene Menge: „sie teilen sich einen Selektor und unterscheiden sich allein in ihrer Marke". Seit dem Untermenü „Öffnen mit" (260918) stimmt das nicht mehr vollständig.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

**Was sich geändert hat**

Das Kontextmenü der Dateiliste trägt seitdem **zwei** Arten von eigenem Eintrag:

- die festen Einträge, weiterhin die Varianten von `Kontextbefehl` mit dem Selektor `kontextbefehl:`;
- die Glieder des Untermenüs „Öffnen mit", nämlich die Anwendungen, die das System für den Bezugseintrag nennt. Sie sind **keine** Varianten jener Aufzählung — wie viele es sind, steht erst zur Laufzeit fest —, tragen den eigenen Selektor `oeffnenMit:` und werden über ihre Stelle in der Liste zurückgelesen, die `DateifensterQuelle` beim Menübau hinterlegt.

Der Eintrag „Öffnen mit" selbst bleibt eine Variante von `Kontextbefehl` und trägt den alten Selektor genau dann, wenn er kein Untermenü trägt; sein Zweig meldet dann, dass das System keine Anwendung nennt.

**Was in `CLAUDE.md` deshalb nicht mehr trägt**

1. „sie teilen sich einen Selektor" — es sind zwei, und welcher gelesen wird, entscheidet, gegen welche Folge die Marke gehalten wird.
2. Der Absatz nennt die Aufzählung `Kontextbefehl` als das, was „vor dem wirkungslosen Menüeintrag schützt". Für die Glieder des Untermenüs schützt etwas anderes: sie entstehen aus einer Liste und werden über dieselbe gelesen, und eine Marke ohne Glied darin tut nichts. Der Modulkopf von `crates/krk-ui/src/kommandos/kontextmenue.rs` schreibt den Schnitt unter „Eine feste Aufzählung neben einer Liste, deren Länge erst zur Laufzeit feststeht" aus.

**Was weiter trägt**

Die Aussage „welche das sind, sagt `Kontextbefehl::ALLE` und keine Zahl an dieser Stelle" und der Satz zur Feldbreite der Liste `ALLE`. Die historische Angabe „die Zahl stand von der Runde 17 bis zum 260907 auf drei" bleibt richtig; sie ist heute fünf, und die Zahl gehört nach der eigenen Regel des Absatzes nicht in die Prosa.

**Wer es ändert**

`curator`. `CLAUDE.md` ist eine normative Fläche, und dieser Datensatz ändert sie nicht selbst.

**Abnahme**

Der Absatz beschreibt beide Arten von Eintrag, ohne eine Zahl zu nennen, und sagt für jede, was sie vor dem wirkungslosen Menüeintrag schützt.
