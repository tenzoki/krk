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
