Der Zweig für `NeedsUpdate` in `posten_deuten` ist unerreichbar; `gix` fängt den Posten vorher ab
---
`crates/krk-core/src/git/leser.rs:398` trägt in `posten_deuten` den Zweig

```rust
EntryStatus::NeedsUpdate(_) => return None,
```

und der Doc-Kommentar darüber (`:370`) erklärt ihn als die Stelle, an der KRK den aufgefrischten
Stat-Zwischenspeicher „liest und verwirft". Beide Modulköpfe sagen dasselbe (`git/leser.rs:51`,
`git/mod.rs:18`).

**Der Zweig kann nicht auslösen.** `gix` 0.87.1 fängt jeden `NeedsUpdate`-Posten in seinem eigenen
Statusiterator ab, bevor er den Rufer erreicht:
`gix-0.87.1/src/status/iter/mod.rs:296`, `Iter::maybe_keep_index_change`, legt ihn in
`self.index_changes` und gibt `None` zurück, der Posten wird also gar nicht erst geliefert.
`Platform::into_index_worktree_iter` geht über denselben inneren Iterator
(`src/status/index_worktree.rs:590`) und verhält sich gleich. Nach außen bleibt allein
`Outcome::has_changes()`, ein Ja oder Nein ohne Zahl; die gesammelten Änderungen liegen in
`Outcome::changes`, und das Feld ist `pub(super)`.

Gemessen am 260831 im Schritt 10: in jedem Durchgang mit veraltetem Index stand `has_changes()` auf
ja, und kein einziger Posten mit `EntryStatus::NeedsUpdate` kam beim Rufer an
(`messungen/260831-0855-needsupdate.txt`, Einschränkung 2).

**Was daran falsch ist, ist die Prosa und nicht die Wirkung.** Die Stufe A bleibt schreibfrei, und
der Posten wird verworfen — nur nicht von KRK, sondern von `gix`, das ihn zurückhält, bis jemand
`write_changes` ruft, und KRK ruft es nicht. Wer den Modulkopf liest, hält den Zweig für den
Mechanismus, der die Zusage trägt; er trägt sie nicht.

Damit hängt daran auch das Prüfmittel von C10.3: „`grep -rn 'NeedsUpdate\|write_changes' crates/`,
dessen Treffer die Lesestelle nennen". Die gefundene Lesestelle ist toter Code. Die Zusage selbst
hält der fehlende Aufruf von `write_changes` und sonst nichts;
`grep -rn 'write_changes(' crates/` bleibt ohne Fundstelle, und das ist die Prüfung, die trägt.
(Der zweite Riss in demselben `grep` steht schon in
`260830-1614_o_c3-8-verlangt-null-treffer-fuer-write-changes-c10-3-verlangt-treffer-die-die-lesestelle-nennen.md`;
dieser Datensatz ist der dritte und ein anderer.)

**Zu entscheiden ist, was mit dem Zweig geschieht**, und beides ist vertretbar:

1. Der Zweig bleibt stehen, und der Kommentar sagt, dass `gix` den Posten abfängt und der Zweig die
   Fallunterscheidung nur vollständig hält, falls eine spätere `gix`-Fassung ihn wieder liefert. Der
   Zweig ist dann die bewusste Einordnung, die dieses Projekt an einer Fallunterscheidung ohne
   Auffangzweig will, und kein Versehen.
2. Der Zweig fällt und `EntryStatus::NeedsUpdate` geht über einen Auffangzweig mit. Das verlöre die
   Vollständigkeit, die dieses Projekt an solchen Stellen ausdrücklich will, und der nächste neue
   `EntryStatus`-Wert fiele still hindurch, statt den Bau anzuhalten.

**Empfehlung: Möglichkeit 1.** Der Zweig bleibt, und die drei Prosastellen (`git/leser.rs:51`,
`git/leser.rs:370`, `git/mod.rs:18`) sagen danach, wer den Posten abfängt und wer ihn verwirft.

**Abnahmetest:** Keine Prosastelle in `crates/krk-core/src/git/` behauptet mehr, KRK lese
`EntryStatus::NeedsUpdate`; jede sagt, dass `gix` den Posten zurückhält und KRK ihn dadurch verwirft,
dass es `write_changes` nicht ruft. `grep -rn 'write_changes(' crates/` bleibt ohne Fundstelle.
