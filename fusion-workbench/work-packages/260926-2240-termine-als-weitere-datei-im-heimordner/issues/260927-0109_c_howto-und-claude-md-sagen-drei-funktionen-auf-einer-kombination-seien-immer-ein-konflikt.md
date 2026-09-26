HowTo.md und CLAUDE.md sagen, drei Funktionen auf einer Kombination seien immer ein Konflikt
---
`HowTo.md` (Abschnitt „Die Tastaturbelegung“, Absatz nach der Liste der zwei Fälle) und `CLAUDE.md` (Absatz „Eine Kombination darf seit dem 260926 zwei Funktionen desselben Zustellers gehören …“) sagen: „drei Funktionen auf einer Kombination sind immer ein Konflikt“. Die Regel im Code sagt das nicht. `begegnen` (`crates/krk-core/src/tasten/belegung.rs`) antwortet für zwei Funktionen mit verschiedenem Zusteller immer `false`, also kann eine vom Menü zugestellte Funktion neben zwei einander ausschließenden Funktionen des Ereignisabgriffs stehen, ohne dass `Belegung::konflikte` oder `Belegung::zuweisen` einen Konflikt meldet.

Beispiel an der Auslieferung: `cmd+f` liegt auf `filter_einfuegen` (`gehalten_von = "menue"`) und auf `editor_suchen` (`Wirkungsbereich::Editortext`, Seite Editor). Weist der Nutzer in F1 `cmd+f` zusätzlich `sortierung_name` zu (`Wirkungsbereich::Dateifenster`, Seite Außerhalb), nimmt `zuweisen` es an: gegen `editor_suchen` schließen die Seiten einander aus, gegen `filter_einfuegen` ist der Zusteller ein anderer. Drei Funktionen, kein Konflikt.

Wahr ist der Satz allein für Funktionen **desselben Zustellers**; so steht er richtig im Doc-Kommentar von `Belegung::nachschlag` und in der Probe `drei_funktionen_auf_einer_kombination_sind_ein_konflikt` (`crates/krk-core/tests/belegung.rs`), die zwei zusätzliche Funktionen des Abgriffs verwendet.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260927-0109-reviewer-termine-als-weitere-datei-im-heimordner.md`; `260926-2308_*_duerfen-zwei-funktionen-desselben-zustellers-eine-kombination-tragen-wenn-ihre-wirkungsbereiche-einander-ausschliessen.md`

Nebenbefund zum Beispiel, älter als diese Arbeit und nicht Gegenstand dieses Datensatzes: in der beschriebenen Belegung sortiert `cmd+f` mit dem Fokus im Dateifenster, und „Zwischenablage an den Filter anhängen“ ist dort über die Taste nicht mehr erreichbar, ohne dass eine Meldung erscheint. Dieselbe Verdeckung war vorher schon möglich, indem man `editor_suchen` die Kombination nimmt und `sortierung_name` gibt.

Abnahme: Beide Stellen schränken den Satz auf Funktionen desselben Zustellers ein (oder nennen die Ausnahme ausdrücklich); `grep -rn 'drei Funktionen auf einer Kombination' HowTo.md CLAUDE.md` findet keine Stelle mehr, die ihn ohne diese Einschränkung führt.

---
Resolved: `HowTo.md` („Die Tastaturbelegung“) sagt jetzt, dass drei Funktionen, die den Tastendruck auf demselben Weg bekommen, auf einer Kombination immer ein Konflikt sind, und dass eine Funktion auf dem anderen Weg dabei nicht mitzählt und neben zweien davon liegen darf. `CLAUDE.md` schränkt den Satz auf Funktionen desselben Zustellers ein und nennt die Ausnahme über Zusteller hinweg mit `begegnen` als Beleg. `grep -rn 'drei Funktionen auf einer Kombination' HowTo.md CLAUDE.md` findet nichts mehr. Der Nebenbefund zur verdeckten Menüfunktion bleibt unberührt. Commit folgt mit diesem Datensatz (docs(howto)).
