# Die drei Codebefunde der Durchsicht der Runde 23

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Circle:** `260830-1045-git-bereich-liest-status-branch-verlauf`
**Durchsicht:** `260831-1444-coderev-git-bereich-runde-23.md`
**Defekte:** `260831-1444_*_der-verlauf-laeuft-in-graphenreihenfolge-und-nicht-nach-commit-zeit.md`,
`260831-1444_*_ein-abgebrochener-gitlauf-laeuft-weiter-und-a10-gilt-nur-dem-halter-und-nicht-dem-faden.md`,
`260831-1444_*_jeder-fehlschlag-von-discover-ausser-dem-deskriptormangel-wird-als-kein-repository-ausgegeben.md`

---

## Verification

```
make check — exit 0
```

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`
unter `-D warnings`, `cargo fmt --all --check`. `grep -rn 'write_changes(' crates/` bleibt leer:
die Stufe A schreibt weiter nicht.

---

## Jede der drei Proben sieht ihren Fall

Der Auftrag verlangt es, und geprüft ist es einzeln: die Behebung wurde je Befund wieder
herausgenommen und die Probe dagegen gefahren.

- **Sortierung:** ohne `.sorting(…)` sieht die Probe
  `… haupt 4, zweig 3, haupt 3, zweig 2, haupt 2, zweig 1 …` statt
  `… haupt 4, zweig 3, zweig 2, zweig 1, haupt 3 …`. Die erste Abweichung steht an der vierten
  Stelle; das Prüfrepository ist eigens so gebaut, dass Graphen- und Zeitordnung
  auseinandergehen.
- **Abbruch:** mit einer Fassung, die das Kennzeichen entgegennimmt und nicht liest, meldet die
  Probe `Some([("unverfolgt.txt", Neu)])` statt `None`.
- **`discover`:** mit dem alten Auffangzweig meldet die Probe `KeinRepository` statt
  `Unentschieden`.

## Befund 1: der Verlauf steht nach der Zeit

`Gitleser::verlauf` läuft mit `Sorting::ByCommitTime(CommitTimeOrder::NewestFirst)`. Sortiert wird
nach der Zeit des Committers, angezeigt bleibt die des Autors — dieselbe Trennung, die `git log`
trägt; die zwei fallen nur auseinander, wo ein Commit umgesetzt worden ist, und die Reihenfolge
folgt dann dem, was `git log` an derselben Stelle zeigt.

**Was es kostet, ist gemessen, und im Ergebnis kostet es nichts.** KRKs eigenes Repository mit
800 Commits, Profil `release`, je Messung ein frisch geöffneter Leser, drei Läufe zu je sieben
Messungen, Median des ersten Schwungs von fünfzig Commits:

| Stand | erster Schwung | tiefer Schwung (750 übersprungen) |
|---|---|---|
| Graphenordnung, ohne Zwischenspeicher (der alte Stand) | 4,1 bis 4,9 ms | 2,81 bis 2,83 ms |
| nach Zeit sortiert, ohne Zwischenspeicher | 5,9 bis 6,2 ms | 4,6 bis 4,8 ms |
| nach Zeit sortiert, mit Objektzwischenspeicher (der neue Stand) | 2,5 bis 3,2 ms | 2,9 bis 3,6 ms |

Die Sortierung allein kostet rund 1,8 ms. Der Objektzwischenspeicher — den der Defektdatensatz
selbst nennt, weil `gix` ihn für genau diesen Fall nennt — gibt mehr zurück, als sie nimmt: der
Lauf schlägt jeden Commit zweimal nach, einmal die Sortierung für seine Zeit und einmal `verlauf`
für seine sechs Felder. Er steht als `OBJEKTSPEICHER` in `crates/krk-core/src/git/leser.rs`, ist auf
ein Viertel Megabyte gesetzt (dieselbe Messung bei 64 KiB liefert dasselbe) und wird über
`object_cache_size_if_unset` gesetzt, damit ein Repository, das `gitoxide.objects.cacheLimit`
selbst führt, seine Zahl behält. Er lebt so lange wie der Leser, also die Dauer eines Laufs.

Die Nutzerfrage, die der Datensatz für den Fall messbarer Mehrkosten anmeldet, entfällt damit.
Nachgezogen: die Zahl an `VERLAUFSSCHRITT` (3,9 ms → 2,5 bis 3,2 ms) und der Doc-Kommentar in
`crates/krk-ui/src/gitmodell.rs`.

## Befund 2: der abgebrochene Lauf bricht mitten im Status ab

`Gitleser::marken` nimmt ein `&AtomicBool` und liest es vor jedem Posten des Statusstroms — die
Schleife, die der Datensatz benennt. Ein gesetztes Kennzeichen liefert `None`, also
„unentschieden" und keine halbe Liste: eine halbe wäre von „diese Einträge sind unverändert" nicht
zu unterscheiden.

**Das Kennzeichen des Durchlaufs ist übernommen und keine zweite Form gebaut.**
`crates/krk-core/src/verzeichnis/durchlauf.rs` führt es als `&AtomicBool` mit
`Ordering::Relaxed`, gelesen vor jeder Einheit, die dauern kann, während der Eintritt selbst beim
Rufer geprüft wird; genau so steht es jetzt hier. Der Modulkopf von `crates/krk-core/src/git/mod.rs`
sagte bis dahin, der Leser kenne kein Abbruchkennzeichen; er sagt jetzt, dass genau eine der drei
Auskünfte eines kennt, und warum diese: sie ist die einzige, die mit 12 bis 164 ms lange genug
dauert, dass ein aufgegebener Lauf sie noch zu Ende laufen ließe.

**Die zweite Hälfte des Befunds bleibt wahr, und der Baum sagt es jetzt.** Zwei Fäden können sich
weiter überschneiden, so lange wie der ältere für einen Posten und den Aufbau seines Stroms
braucht: `Gitlauf::drop` wartet ausdrücklich nicht, und ein Warten wäre genau die Bildzeit, die der
Lauf vermeidet. A10 und C7.11 sagen wörtlich „laufen nie nebeneinander" und sind damit weiter
stärker als das Gebaute; der Modulkopf von `crates/krk-core/src/git/lauf.rs` und der Doc-Kommentar
an `Tabinhalt::gitlauf` schreiben Halter und Faden jetzt getrennt aus. Die zwei Spec-Zeilen sind
unangetastet und gehören zur Prosadurchsicht.

## Befund 3: `discover` verneint nur noch in einer Lage

`entschiedene_verneinung` zerlegt `gix::discover::Error` und `gix::discover::upwards::Error`
vollständig und ohne Auffangzweig; wahr wird genau eine Lage, nämlich bis zur Wurzel gesucht und
nichts gefunden. Jeder andere Fehlschlag ist `Oeffnung::Unentschieden`. Eine Variante mehr in `gix`
hält den Bau an, statt still in die Verneinung zu fallen.

`fehlerkette_meldet_deskriptormangel` ist gestrichen, und das ist kein Verzicht, sondern die Folge:
die drei Varianten, die überhaupt einen `io::Error` tragen können — `CurrentDir`, `CheckTrust`,
`Open(_)` —, sind ohnehin sämtlich unentschieden, während die drei Varianten der entschiedenen
Verneinung nichts als Pfade tragen. Die Kindprobe `kind_liest_unter_abgesenkter_deskriptorgrenze`
misst es unverändert und bleibt grün.

**Der Abnahmetest ist im Maßstab erfüllt und in der Auslösung nicht in seinem Wortlaut.** Gemessen
gegen `gix` 0.87.1, je ein frisch angelegtes Prüfrepository:

| Lage | Antwort von `gix` | was KRK daraus macht |
|---|---|---|
| `.git/config` mit `chmod 000` | `Ok(repo)` | `Offen` |
| `.git` als Datei mit `gitdir: /gibt/es/nicht` | `Err(Discover(NoGitRepository))` | `KeinRepository` |
| `.git`-Verzeichnis mit `chmod 000` | `Err(Discover(NoGitRepository))` | `KeinRepository` |
| `.git/config` ist kein INI | `Err(Open(Config(Init(Parse))))` | `Unentschieden` |
| unbekannte `extensions.*` | `Ok(repo)` | `Offen` |

Die vom Datensatz verlangte unlesbare `.git/config` erreicht `gix` also gar nicht als Fehler. Die
Probe macht die Konfiguration deshalb im Sinne des Zerlegers unlesbar und trifft damit genau
`Open(_)`, die Variante, die der Abnahmetest als Unterscheidungsmerkmal benennt.

## Zwei neue Datensätze

- `260831-1652_*_gix-zieht-ein-unlesbares-git-verzeichnis-und-einen-toten-gitdir-verweis-selbst-zu-kein-repository-zusammen.md`
  — der Rest des dritten Befunds liegt in `gix` und nicht mehr hier: zwei der Lagen, die der
  Defektdatensatz nennt, zieht die Kiste selbst zu `NoGitRepository` zusammen, und diese Variante
  trägt einen Pfad und keine Ursache. Der Modulkopf des Lesers nennt die Grenze.
- `260831-1652_*_drei-signaturen-in-den-datenstrukturen-des-plans-sind-mit-den-durchsichtsbefunden-wieder-veraltet.md`
  — zwei davon verursacht diese Arbeit selbst.

## Die acht Prosabefunde

Unangetastet und weiter `_o_`; sie sind der nächste Auftrag.
