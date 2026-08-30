Der Spec schützt die Messstrecke mit einem Schalterstand, den A13 auf „ein" stellt
---
`260830-1251_*_spec-git-bereich-liest-status-branch-verlauf.md`, Abschnitt `## Verhältnis zu den
zehn Zeitzusagen aus C8 der Runde 1`, begründet, warum die Runde keinen Abnahmelauf schuldet:

> die kopflose Messstrecke (`crates/krk-ui/src/messmodus.rs`) kennt weder den Git-Bereich noch die
> Markenspalte, und beide Schalter stehen ab Werk so, dass die Strecke sie nicht anfasst.

Die zweite Hälfte trifft nicht zu. **A13 desselben Specs stellt die Markenspalte ab Werk auf ein**
(„Die Markenspalte steht ab Werk eingeschaltet, der Git-Bereich ab Werk ausgeblendet"), und C5.10
macht daraus ein Abnahmekriterium. Ein eingeschalteter Markenschalter heißt nach dem Plan, dass bei
jedem Ordnerwechsel ein Statuslauf angestoßen wird. Dazu kommt: die Messstrecke läuft am gebauten
Bündel gegen die `session.toml` des Nutzers und nicht gegen den Auslieferungszustand, also sagt „ab
Werk" über den Stand während einer Messung ohnehin nichts.

**Was die Messung wirklich schützt, ist der Ort des Messplatzes.** Er liegt unter
`~/Library/Caches/krk-messplatz` (CLAUDE.md, `## Was man nicht sieht`), und dort liegt bis zur
Wurzel kein `.git`. `gix::discover` antwortet an einem solchen Pfad in gemessenen 21 bis 82 µs mit
„kein Repository", es entsteht kein Lauf, und die zehn Zusagen sehen von dieser Runde nichts.

Der Unterschied ist nicht akademisch: die eine Begründung ist falsch und die andere prüfbar. Wer
den Messplatz eines Tages in ein Repository legt — etwa in einen Prüfbaum unter dem Projekt —,
bekommt bei eingeschalteter Markenspalte einen Statuslauf je Ordnerwechsel mitten in einer
Zeitmessung, und die Spec-Begründung hätte ihn nicht gewarnt.

**Abnahme:** der Abschnitt begründet die Unberührtheit der zehn Zusagen mit dem Ort des Messplatzes
statt mit einem Schalterstand, und die Aussage ist mit
`git -C ~/Library/Caches/krk-messplatz rev-parse --show-toplevel` geprüft, das nichts liefern darf.
Der Plan der Runde 23 führt diese Prüfung in seinem Schritt 16.
---
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
Gefunden beim Schreiben des Plans der Runde 23, bei der stellengenauen Erhebung für `messmodus.rs`
(Frage 8 aus `## Open for Planner`).
