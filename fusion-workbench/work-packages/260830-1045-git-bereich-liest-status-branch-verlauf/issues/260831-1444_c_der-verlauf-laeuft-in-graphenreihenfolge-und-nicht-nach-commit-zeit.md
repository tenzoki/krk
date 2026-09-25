Der Verlauf läuft in Graphenreihenfolge und nicht nach Commit-Zeit

---
`Gitleser::verlauf` (`crates/krk-core/src/git/leser.rs:206`) ruft `self.repo.rev_walk([anfang]).all()` und setzt keine Sortierung. Die Vorbelegung von `gix::revision::walk::Sorting` ist `BreadthFirst` (`gix-0.87.1/src/revision/walk.rs:31-42`, `#[default]`), also die Reihenfolge des Commit-Graphen und nicht die der Commit-Zeit; `git log` ordnet dagegen ab Werk nach Zeit.

Zwei Aussagen des Baums stimmen damit nicht:

- `crates/krk-ui/src/gitmodell.rs:53-54` — „Die geholten Commits, **die jüngsten zuerst**, in der Reihenfolge des Laufs über die Vorfahren von HEAD." Die zweite Hälfte des Satzes trifft zu, die erste folgt nicht aus ihr.
- Der Spec der Runde 23, C4 („Die Liste zeigt zuerst die fünfzig **jüngsten** Commits", `planning/260830-1251_*_spec-…`).

In einer linearen Kette fallen beide Reihenfolgen zusammen; in einem Repository mit Zusammenführungen nicht.

**Abnahmetest:** ein Prüfrepository mit zwei Zweigen, deren Commits zeitlich verschränkt sind, und eine Zusammenführung darüber; die Liste steht danach nach Autor- beziehungsweise Committerzeit absteigend, oder die beiden Prosastellen sagen, dass sie es nicht tut. Welches von beiden gilt, ist eine Nutzerfrage, wenn `Sorting::ByCommitTime` messbar mehr kostet — die Kiste nennt dafür ausdrücklich einen Objektzwischenspeicher (`walk.rs:50-53`).

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23. Der Befund ist gelesen und nicht gemessen: die Vorbelegung steht in der Quelle von `gix`, ein Lauf gegen ein Prüfrepository mit Zusammenführung ist nicht gefahren.
Verwandt: `260831-1444_*_der-nachschlag-des-verlaufs-setzt-am-letzten-commit-an-und-verliert-jeden-nebenzweig.md`.

---
Resolved: 260831. `Gitleser::verlauf` läuft mit `Sorting::ByCommitTime(CommitTimeOrder::NewestFirst)`; sortiert wird nach der Zeit des Committers, angezeigt bleibt die des Autors, wie `git log` es hält. Die Probe `der_verlauf_steht_nach_der_zeit_und_nicht_nach_dem_graphen` (`crates/krk-core/tests/git.rs`) baut das im Datensatz verlangte Prüfrepository — zwei Zweige mit verschränkten Zeiten, verschieden lang, eine Zusammenführung darüber — und hält die Liste gegen zwei Maßstäbe: die erwartete Reihenfolge der Kurzbeschreibungen und die Objektnamen aus `git rev-list HEAD`. Vor der Änderung sah sie `… haupt 4, zweig 3, haupt 3, zweig 2, haupt 2, zweig 1 …` statt `… haupt 4, zweig 3, zweig 2, zweig 1, haupt 3 …`; die erste Abweichung steht an der vierten Stelle.

Die Nutzerfrage, die der Datensatz für den Fall messbarer Mehrkosten anmeldet, entfällt: die Kosten sind gemessen und im Ergebnis negativ. An KRKs eigenem Repository mit 800 Commits, Profil `release`, je Messung ein frisch geöffneter Leser, drei Läufe zu je sieben Messungen, erster Schwung von fünfzig Commits im Mittel — Graphenordnung ohne Zwischenspeicher 4,1 bis 4,9 ms; nach Zeit sortiert ohne Zwischenspeicher 5,9 bis 6,2 ms; nach Zeit sortiert mit dem Objektzwischenspeicher 2,5 bis 3,2 ms. Die Sortierung allein kostet also rund 1,8 ms, und der Zwischenspeicher, den `gix` für genau diesen Fall nennt und den der Datensatz zitiert, gibt mehr zurück, als sie nimmt. Er steht als `OBJEKTSPEICHER` in `crates/krk-core/src/git/leser.rs` und trägt die Messung; die Zahl 3,9 ms an `VERLAUFSSCHRITT` ist auf 2,5 bis 3,2 ms nachgezogen.

Die zwei Prosastellen sind mitgezogen: `crates/krk-ui/src/gitmodell.rs` nennt jetzt die Commit-Zeit als Grund für „die jüngsten zuerst", und der Doc-Kommentar von `verlauf` schreibt die Wahl samt der Trennung von Autor- und Committerzeit aus. Der Spec-Satz C4 trifft damit zu und ist unverändert.
