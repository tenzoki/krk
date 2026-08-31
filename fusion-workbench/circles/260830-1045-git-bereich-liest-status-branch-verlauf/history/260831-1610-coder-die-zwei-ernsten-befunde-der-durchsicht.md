# Die zwei ernsten Befunde der Durchsicht der Runde 23

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Circle:** `260830-1045-git-bereich-liest-status-branch-verlauf`
**Durchsicht:** `reviews/260831-1444-coderev-git-bereich-runde-23.md`
**Defekte:** `issues/260831-1444_c_der-nachschlag-des-verlaufs-setzt-am-letzten-commit-an-und-verliert-jeden-nebenzweig.md`,
`issues/260831-1444_c_die-marken-werden-bytegenau-ueber-den-namen-zugeordnet-und-gix-liefert-ihn-vorkomponiert.md`

---

## Verification

```
make check — exit 0
```

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`
unter `-D warnings`, `cargo fmt --all --check`. `grep -rn 'write_changes(' crates/` bleibt leer:
die Stufe A schreibt weiter nicht.

---

## Beide Befunde sind vor der Behebung gemessen worden

Der Auftrag verlangt einen Abnahmetest, der den Fall wirklich sieht. Beide Proben sind deshalb
zuerst gegen den unveränderten Baum gefahren, und beide waren rot:

- **Der Verlauf:** die Blätterschleife über ein Prüfrepository mit 62 Commits sah **56 von 62**.
- **Die Marke:** `kein einziger Befund ist eingetragen worden`. Die zwei Voraussetzungen der Probe
  standen dabei beide — die Platte trug den zerlegten Namen, `gix` meldete den vorkomponierten —,
  womit der bis dahin nur gelesene Befund des Durchsichtsberichts bestätigt ist.

## Befund 1: der Nachschlag setzt nicht mehr am letzten Commit an

`Gitleser::verlauf` nimmt `bereits: usize` statt `ab: Option<ObjectId>` und läuft in jedem Schwung
von HEAD los, wie `git log --skip`. `rev_walk` gibt jeden erreichbaren Commit genau einmal aus, also
zerlegen die Schwünge den Verlauf in Stücke, statt ihn an den Vorfahren des letzten angezeigten
Commits zu beschneiden. Mitgezogen: `Gitfrage::WeitererVerlauf { bereits }`,
`Tabliste::verlauf_nachladen` (nimmt die Zahl aus `Gitmodell::verlaufslaenge`), und
`Gitmodell::letzter_commit` fällt als Rufer weg und ist gestrichen.

**Zum Nachbardatensatz** `260831-1444_*_der-verlauf-laeuft-in-graphenreihenfolge-und-nicht-nach-commit-zeit.md`:
der Weg verbaut ihn nicht, er macht ihn erst gefahrlos. Ein `.sorting(…)` gilt jetzt jedem Schwung
gleich, weil jeder Schwung derselbe Lauf von HEAD aus ist. Der Doc-Kommentar von `verlauf` nennt
die Stelle namentlich.

## Befund 2: die Zuordnung fragt zweimal

`Ordnermodell::gitmarken_setzen` fragt erst bytegenau wie bisher und erst bei einem Fehlschlag über
ein zweites Nachschlagewerk, dessen Schlüssel `verzeichnis::kollation::namensschluessel` baut.

**Nichts Neues gebaut, wo etwas dastand.** `verzeichnis/kollation.rs` ist die eine Stelle, an der
`krk-core` Unicode über den Bytevergleich hinaus befragt; der neue Schlüssel ist derselbe Kollator
mit `Strength::Identical`, die die NFD-Form an den Schlüssel anhängt. **Keine Abhängigkeit kommt
hinzu.**

Zwei Eigenschaften trägt die Bauform mit Absicht: der bytegenaue Treffer geht vor, weil APFS beide
Schreibweisen desselben Namens nebeneinander hält; und das zweite Werk entsteht erst beim ersten
Fehlschlag, ein durchweg bytegenau treffender Ordner zahlt keinen einzigen Kollationsschlüssel.

**Die zweite Stelle desselben Vergleichs bleibt offen**
(`shared/issues/260826-1221_*_die-kollisionspruefung-vergleicht-bytegenau-…`); sie war nicht Auftrag.

## Die elf übrigen Befunde

Unangetastet und weiter `_o_`.
