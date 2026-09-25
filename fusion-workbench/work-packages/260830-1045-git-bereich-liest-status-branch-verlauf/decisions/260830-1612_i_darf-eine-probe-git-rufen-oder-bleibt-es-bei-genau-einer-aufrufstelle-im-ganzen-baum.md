# Darf eine Probe `git` rufen, oder bleibt es bei genau einer Aufrufstelle im ganzen Baum?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md` (Schritt 3, Proben); `260830-1251_*_spec-git-bereich-liest-status-branch-verlauf.md` (C3.1, C3.6, C3.7, C3.10, C4.1, C4.5, C5.3, C8.6); `xtask/src/release.rs` (`xtask_ruft_git_an_genau_einer_stelle`, C3.13 der Runde 8); `crates/krk-core/tests/git.rs`

---

## Question

Der Schritt 3 der Runde 23 verlangt neun Proben „sämtlich gegen angelegte Prüfrepositorys". Ein
Prüfrepository lässt sich in diesem Baum nur mit `/usr/bin/git` anlegen: die Stufe A schreibt nicht
(E8), also kann `gix` es nicht bauen, und ein von Hand geschriebenes `.git` verlangte einen gültigen
Index im Binärformat.

Damit steht `crates/krk-core/tests/git.rs` gegen eine Zusage der Runde 8. Die Probe
`xtask_ruft_git_an_genau_einer_stelle` (`xtask/src/release.rs`) liest **jede** `.rs`-Datei unter der
Projektwurzel, zählt jedes `Command::new("/usr/bin/git")` und `Command::new("git")` und verlangt
genau einen Treffer, in `xtask/src/git.rs`. Der neue Probendatei bringt zwei weitere; die Probe wird
rot, und mit ihr `make check`.

Weder der Spec noch der Plan der Runde 23 nennt diese Kollision. Sie ist beim Bau des Schrittes 3
aufgefallen und am 260830 gemessen: `cargo test --workspace` läuft in allen Zielen grün außer diesem
einen.

**Was die Zusage sichern soll, steht in ihrem eigenen Namen:** *xtask* ruft `git` an genau einer
Stelle, damit die Auslieferungskette einen Eingang zu `git` hat und nicht mehrere. Ihre Umsetzung
liest den ganzen Baum und trifft damit auch Code, der nie ausgeliefert wird. Ob das gewollt oder
nur bequem war, sagt der Datensatz der Runde 8 nicht.

## Options

1. **Die Probe kennt zwei Stellen und verbietet die dritte.**
   - Pros: Dieselbe Bauform, die dieses Projekt für `genau_drei_pruefordner_fassungen_stehen_im_baum`
     schon fährt: die bekannten Stellen stehen namentlich da, jede weitere hält den Lauf an. Die
     Zusage behält ihre volle Kraft — ein zweiter Rufer in `xtask` oder in `crates/*/src` fällt
     weiterhin auf. Der Unterschied zwischen Auslieferungskette und Probe steht ausgeschrieben.
   - Cons: Die Zusage aus C3.13 der Runde 8 lautet nach der Änderung nicht mehr „genau eine Stelle im
     ganzen Baum"; eine Prosastelle ist nachzuziehen. Wer die Liste künftig erweitert, muss den
     Grund je Eintrag prüfen, statt sich auf eine Zahl zu verlassen.
2. **Die Probe liest nur noch Code, der ausgeliefert wird** (alles außer `crates/*/tests/`).
   - Pros: Die Zusage bleibt eine Zahl und keine Liste; sie sagt danach, was sie meint, nämlich
     „ein Eingang zu `git` im Programm und im Bauwerkzeug".
   - Cons: Ein zweiter Rufer unter `tests/` fällt danach nie mehr auf, auch wenn er dort nichts zu
     suchen hat. Die Grenze `tests/` trifft daneben `#[cfg(test)]`-Module unter `src/` nicht, und
     die sind ebensowenig ausgeliefert; die Regel wäre also nicht die, die sie zu sein vorgibt.
3. **Die Proben legen kein Repository mit `git` an.**
   - Pros: Keine Prosastelle und keine Probe der Runde 8 wird angefasst.
   - Cons: Es gibt keinen gangbaren Weg. `gix` schreibt in dieser Runde nicht (E8, Bedingung 2), ein
     von Hand geschriebenes `.git` verlangte einen gültigen Index im Binärformat, und ohne
     Prüfrepository fallen sieben der neun Proben aus Schritt 3 ersatzlos weg — darunter die fünf
     Markenzustände (C5.3) und der abgelöste HEAD (C3.6). Die Möglichkeit steht hier, weil sie zu
     prüfen war, und nicht, weil sie trägt.

## Recommendation

Möglichkeit 1. Sie hält die Zusage dort fest, wo sie etwas sichert, und schreibt den einen Grund
aus, aus dem eine zweite Stelle danebensteht. Der Coder hat sie **nicht** umgesetzt:
`xtask/src/release.rs` steht nicht in der Dateiliste des Schrittes 3, und die Probe trägt ein
Abnahmekriterium einer fremden Runde. Bis zur Antwort ist `make check` an genau dieser einen Probe
rot; jedes andere Ziel des Arbeitsbereichs läuft grün.

## Answered

<offen>

---
Answered: shared/history/260830-0950-orchestrator-session.md:107 — Möglichkeit 2: die Prüfung liest nur noch Code außerhalb von `crates/*/tests/`. Die Ausnahmeliste (Möglichkeit 1) und die Beschränkung auf `xtask/` sind verworfen; der Preis, dass ein zweiter Rufer unter `tests/` nie mehr auffällt und dass `krk-ui`s `#[cfg(test)]`-Module unter `src/` weiter gezählt bleiben, ist benannt und angenommen.

Implemented: xtask/src/release.rs:1099 — die Probe heißt jetzt `git_wird_ausserhalb_der_probenordner_an_genau_einer_stelle_gerufen` und überspringt über `liegt_im_probenordner_einer_kiste` (`:1287`) jede Datei unter `crates/*/tests/`; der zweiteilige Preis steht in ihrem Doc-Kommentar, die Prosa in `xtask/src/git.rs` und `xtask/src/veroeffentlichung.rs` ist nachgezogen.
