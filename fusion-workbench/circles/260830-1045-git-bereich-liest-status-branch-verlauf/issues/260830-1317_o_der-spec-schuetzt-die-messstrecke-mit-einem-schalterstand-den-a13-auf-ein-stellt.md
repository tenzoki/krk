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

---
Abgleich 260831-1417: Der Befund steht, und der gebaute Baum belegt jetzt beide Hälften.
Die Markenspalte steht ab Werk (`Spaltensichtbarkeit::default`, `crates/krk-core/src/ablage/sitzung.rs:344`, `marke: true`), und der Bedarf ist eine Oder-Verknüpfung: `Anwendungsdelegierter::gitbedarf_nachziehen` (`crates/krk-ui/src/appkit/anwendung.rs:4642`) rechnet `modell.sichtbar(Bereich::Git) || spalte_sichtbar_in(…, Spalte::Marke)` und reicht das Ergebnis an `Tabliste::git_gefragt_setzen`; `gitlauf_nachziehen_an` (`crates/krk-ui/src/tabs.rs:1199`) stößt daraufhin bei jedem Ordnerwechsel des sichtbaren Tabs einen `Gitlauf` an. Der Datensatz nennt daneben die `session.toml` des Nutzers als Grund, warum „ab Werk" nichts über den Stand während einer Messung sagt; die Prüfsitzung schließt die Lücke nicht, denn `messmodus::tests::pruefsitzung` baut sie über `..Sitzung::default()` und erbt damit ebenfalls `marke: true`.
Auch die erste Hälfte der Spec-Begründung trägt nicht: `messmodus.rs` nennt weder Bereich noch Spalte, hält aber nach seinem eigenen Modulkopf nur, „was kein AppKit beruehrt"; `Aufgabe::Spannen` und `Aufgabe::Sitzung` messen „innerhalb eines Prozesses" der laufenden Anwendung und damit über denselben Anwendungsdelegierten.
Die im Datensatz verlangte Prüfung ist gefahren und liefert, was sie soll: `git -C ~/Library/Caches/krk-messplatz rev-parse --show-toplevel` bricht mit exit 128 ab (`260831-1334-coder-schritt-16-die-abnahmekommandos-ohne-fenster.md`). Der Ort des Messplatzes trägt die Aussage, der Schalterstand nicht. Der Spec-Absatz ist unverändert; der Defekt bleibt offen.
