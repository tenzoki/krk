# Der Markenlauf liest ausgelagerte `.gitignore`- und `.gitattributes`-Dateien, und Modulkopf und Anleitung sagen, er lade nichts herunter

---
Modulkopf von `leser.rs` und `HowTo.md` sagen ohne Einschränkung, der Markenlauf öffne bzw. lade keine ausgelagerte Datei. Zwei Wege in `gix` öffnen trotzdem Dateien des Arbeitsbaums, die ausgelagert sein können: den einen nennt der Modulkopf halb, den anderen nennen weder der Modulkopf noch der Plan, und `HowTo.md` nennt keinen von beiden.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261026-1208-plan-git-markenspalte-laedt-keine-cloud-dateien-herunter.md`, `261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md`, `261026-1947-reviewer-abschlussdurchsicht-git-markenspalte-cloud-dateien.md`

## Defekt

`crates/krk-core/src/git/leser.rs`, Abschnitt `# Eine Datei, die nur in der Cloud liegt`, beginnt mit „**Der Markenlauf oeffnet keine ausgelagerte Datei**“. `HowTo.md`, Abschnitt zum Git-Bereich, beginnt den neuen Absatz mit „**Eine Datei, die auf einem Cloudlaufwerk nur in der Cloud liegt, lädt die Markenspalte nicht herunter.**“ Der Satz gilt für den Inhaltsvergleich und nicht für den ganzen Lauf. Zwei Wege in `gix` öffnen Dateien des Arbeitsbaums, ohne dass `Inhaltsvergleich` gefragt wird:

1. **`.gitignore` im Verzeichnisdurchlauf.** `index_worktree_status` baut seinen Ausschluss-Stapel mit `ignore::Source::WorktreeThenIdMappingIfNotSkipped` (`gix-0.87.1/src/status/index_worktree.rs`, `index_worktree_status`). Der Durchlauf liest die `.gitignore` jedes betretenen Ordners über `gix_glob::search::add_patterns_file` (`gix-worktree-0.56.0/src/stack/state/ignore.rs`). Der Modulkopf nennt das in einem Halbsatz unter „Was dabei bleibt“, sagt aber nicht, dass dabei eine ausgelagerte `.gitignore` heruntergeladen werden kann. `HowTo.md` sagt es gar nicht.
2. **`.gitattributes` beim Hashen einer lokalen Datei.** Weicht das stat einer **lokalen** Datei ab und ist die Größe gleich, ruft `FastEq` `HashEq` auf, und `ReadDataImpl::stream_worktree_file` (`gix-status-0.34.1/src/index_as_worktree/function.rs`) ruft vor dem Öffnen `attr_stack.at_entry(…)`. Dieser Stapel steht auf `attributes::Source::WorktreeThenIdMapping` und liest die `.gitattributes` jedes Ordners auf dem Pfad über `add_patterns_file` (`gix-worktree-0.56.0/src/stack/state/attributes.rs`). Eine ausgelagerte `.gitattributes` neben oder über einer geänderten lokalen Datei wird damit geöffnet. Weder der Modulkopf noch der Plan (`## Approach`, „Was dieser Plan nicht anfasst“) nennen diesen Weg.

Schritt 4 des Plans sollte Weg 1 an der Wegwerfkopie messen. Er ist nicht gelaufen, also ist für beide Wege nicht gemessen, ob die Datei geladen wird. Inferenz aus dem Quelltext: `add_patterns_file` liest die Datei, und ein Lesen lädt eine Datei mit `SF_DATALESS` herunter. Für den Entscheid `261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md` (Möglichkeit 1: ein Hintergrundleser öffnet keine Platzhalterdatei) sind beide Wege offene Ausnahmen.

## Richtung

Ob `gix` 0.87 diese Wege abschalten lässt, ist nicht geprüft. `index_worktree_status` setzt beide Quellen fest. Mindestens die Doku muss stimmen:

- Der Modulkopf schränkt den ersten Satz auf den Inhaltsvergleich ein und nennt beide Wege mit Fundstelle, dazu dass sie ungemessen sind.
- `HowTo.md` sagt dem Nutzer, dass `.gitignore`- und `.gitattributes`-Dateien des Repositorys beim Nachsehen gelesen und dafür auch geladen werden können.

Ob KRK die zwei Wege schließen soll, etwa durch einen eigenen Durchlauf oder eine andere Quelle für Attribute, ist eine Frage an den Nutzer und gehört in einen Entscheid, nicht in diese Behebung.

## Abnahme

- `grep -n 'gitattributes' crates/krk-core/src/git/leser.rs` findet den Weg über den Attributstapel im Abschnitt `# Eine Datei, die nur in der Cloud liegt`, mit `stream_worktree_file` als Fundstelle.
- Der erste Satz jenes Abschnitts behauptet nichts mehr über den ganzen Markenlauf, sondern über den Inhaltsvergleich.
- `HowTo.md` nennt im Absatz über Cloud-Dateien im Git-Bereich `.gitignore` und `.gitattributes`.
- `RUSTDOCFLAGS="-D warnings" cargo doc -p krk-core --no-deps` und `cargo fmt --all --check` sind grün.

Resolved: 261026. Der Modulkopf von `leser.rs` sagt den Satz über ausgelagerte Dateien allein vom Inhaltsvergleich aus und nennt die zwei Lesewege von `gix` mit Fundstellen und als ungemessen (`.gitignore` im Verzeichnisdurchlauf, `.gitattributes` über `stream_worktree_file`); `HowTo.md` sagt, dass diese beiden Dateien gelesen und dafür heruntergeladen werden können. Ob sie es tun, misst erst die Abnahme am Drive.
