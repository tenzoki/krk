# Coder-Sitzung: Schritt 3 der Runde 23, `gix` als Abhängigkeit und der Gitleser im Kern

**Date:** 2026-08-30, 260830-1620
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete (die Arbeit; der Schritt ist an einer Nutzerfrage angehalten)
**Circle:** `circles/260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `planning/260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 3
**HEAD:** `4f6b880` (nicht committet; der Orchestrator committet)

## Die Endbedingung zuerst: `cargo tree` auf beiden Mac-Zielen

**Die erste Bedingung aus `## Stops when` greift nicht.** Am Projektbaum gemessen, nach der Aufnahme
von `gix 0.87.1` in `Cargo.lock`:

```
cargo tree --target x86_64-apple-darwin  -e normal,build   → 0 Treffer auf `cc` und auf `*-sys`
cargo tree --target aarch64-apple-darwin -e normal,build   → 0 Treffer auf `cc` und auf `*-sys`
```

Gegenprobe, damit die Null nicht die eines zu engen Musters ist: dasselbe Muster über
`cargo tree -e normal,build --target all` findet `linux-raw-sys v0.12.1` und `windows-sys v0.61.2`,
und `Cargo.lock` führt beide namentlich. Beide hängen an einem fremden Ziel und kommen auf keinem
der beiden Mac-Ziele im Baum an.

`linux-raw-sys` ist der Eintrag, den die Machbarkeitsanalyse als Formulierungsfrage angekündigt hat;
er kommt über `rustix`. Die Zusage in der neugefassten Form aus E7 — sie spricht über das Bauziel und
nicht über `Cargo.lock` — hält damit.

## Was der Baum an Paketen gewinnt, gemessen und nicht zitiert

Erhoben gegen einen Auszug des Standes `4f6b880` in einem Wegwerfordner, mit demselben Kommando auf
beiden Zielen:

| | vorher | nachher | neu |
|---|---|---|---|
| `x86_64-apple-darwin` | 96 | 197 | 101 |
| `aarch64-apple-darwin` | 96 | 197 | 101 |
| `Cargo.lock` (alle Ziele) | 101 | 219 | 118 |

Von den 101 neuen Einträgen sind `gix` selbst und 50 Kisten mit dem Vorsatz `gix-` zusammen 51; die
übrigen 50 sind fremde, darunter die `crossbeam`-Familie, `rustix`, `parking_lot`, `jiff`, `memmap2`,
`tempfile`, `bstr` und `zlib-rs`. Zwei davon sind weitere Fassungen von `hashbrown` neben der
vorhandenen 0.17.1, also 100 neue Namen. Kein Paket fällt weg.

**Die Analyse und C8.3 nennen 98.** Die Zahl stammt aus einem Wegwerf-Workspace mit einer anderen
Auflösung. Die Begründung in der Wurzel-`Cargo.toml` trägt die am Projektbaum gemessene Zahl und
nennt die Abweichung samt Grund; der Datensatz dazu ist gefilt
(`issues/260830-1613_o_c8-3-nennt-98-zusaetzliche-pakete-am-projektbaum-gemessen-sind-es-101.md`).

## Was gebaut wurde

### Die Kiste und ihre Begründung

`gix = { version = "0.87", default-features = false, features = ["status", "revision",
"max-performance-safe", "parallel", "sha1"] }` in `[workspace.dependencies]` der Wurzel-`Cargo.toml`,
mit der Begründung daneben, wie sie hier jede fremde Kiste trägt: die Merkmalswahl, der Befund zu
`cc` und `-sys` auf beiden Zielen, die gemessenen 101 Pakete, `libc` im Teilbaum von `krk-core`, die
Fassungskadenz von vierzehn kleinen Fassungen in zehn Monaten und die Mindestfassung 1.85.

**Der Punkt, der diese Kiste von `syntect` und `zip` unterscheidet, steht ausdrücklich dabei:** dort
war `default-features = false` die *Bedingung* der Einbindung, weil der Vorgabesatz eine Bibliothek
in C hereinzog. Hier ist es Sparsamkeit. Die C-Anbindung der Kompression ist aus `gix` verschwunden,
statt abschaltbar zu sein: `gix-zlib` ist nicht abwählbar und hängt an `zlib-rs`, das
`build = false` trägt.

`crates/krk-core/Cargo.toml` nimmt sie auf, mit dem Zuordnungsgrund (der Leser liegt im Kern, weil
`krk-ui` kein Bibliotheksziel hat und die Sonderzustände eines Repositorys dort nicht zu belegen
wären). Der Workspace bleibt bei vier Mitgliedern, `crates/` bei drei Kisten (C8.1).

### `crates/krk-core/src/git/mod.rs`

Der Modulkopf, die Wiederausfuhr `pub use gix::ObjectId` (damit `krk-ui` den Verlauf halten kann,
ohne `gix` zu führen), und die drei Datentypen: `Marke` (fünf Werte, `ALLE`, `buchstabe`, `rang`),
`Kopf` (vier Werte) und `Commit` (sechs Felder).

`Marke::rang` ist über den Plan hinaus gebaut und nötig: ein Eintrag kann mehrere Zustände zugleich
tragen (vorgemerkt **und** geändert), und ein Ordner erbt die Marken seines ganzen Unterbaums. Die
Zelle trägt einen Buchstaben. Die Rangfolge ist `Konflikt > Umbenannt > Geändert > Vorgemerkt > Neu`
und beantwortet durchgängig dieselbe Frage — was ist hier noch zu tun. `Neu` steht unten, und das
ist nicht Geschmack: das Ziel einer erkannten Umbenennung erscheint im Verzeichnisdurchlauf als
unverfolgter Eintrag, und stünde `Neu` oben, verschwände die Umbenennung wieder.

### `crates/krk-core/src/git/leser.rs`

`Gitleser::oeffnen`, `kopf`, `verlauf`, `marken`, synchron über einem gehaltenen `Repository`.

**Drei Signaturen weichen vom Plan ab, und alle drei aus einem Grund: `None` heißt unentschieden.**

| Plan | gebaut |
|---|---|
| `oeffnen(&Path) -> Option<Gitleser>` | `oeffnen(&Path) -> Oeffnung` mit `Offen(Box<Gitleser>)`, `KeinRepository`, `Unentschieden` |
| `kopf(&self) -> Kopf` | `kopf(&self) -> Option<Kopf>` |
| `marken(&self, &Path) -> Vec<(String, Marke)>` | `marken(&self, &Path) -> Option<Vec<(String, Marke)>>` |
| `verlauf(…) -> Vec<Commit>` | `verlauf(…) -> Option<Vec<Commit>>` |

**Der Anlass ist gemessen und keine Vorsicht.** Unter `ulimit -n 64` mit belegter Deskriptortabelle
scheitert `gix::discover` an einem echten Repository mit „Could not obtain the current working
directory", und die Fehlerkette trägt `errno 24` (`EMFILE`). Ein `Option<Gitleser>` müsste diesen
Fehlschlag als „kein Repository" ausgeben — also aus einem Zustand des eigenen Prozesses eine
Aussage über ein fremdes Repository machen. Das ist genau der Defekt, den der Durchlauf mit
`260815-0211` einmal getragen hat, und C7.8 verlangt ausdrücklich, dass er hier nicht entsteht. Der
Plan verlangt beides — die zweiwertige Signatur und C7.8 —, und die beiden sind nicht zugleich
erfüllbar; C7.8 geht vor, weil die Signatur ein Mittel und das Kriterium der Zweck ist.

Die Unterscheidung selbst trifft `verzeichnis::sys::ist_deskriptormangel` und keine zweite Regel
daneben; `fehlerkette_meldet_deskriptormangel` läuft die `source()`-Kette ab, weil `gix` seine
`io::Error` einwickelt.

`Kopf` bleibt vierwertig, wie der Plan es festlegt. `Kopf::KeinRepository` entsteht beim Rufer und
nicht in `kopf()`; der Doc-Kommentar sagt es.

**`Offen(Box<Gitleser>)` und nicht `Offen(Gitleser)`**, weil `clippy::large_enum_variant` unter
`-D warnings` sonst den Bau anhält: ein `gix::Repository` trägt 1 240 Bytes, die beiden anderen
Werte tragen nichts.

**Der Modulkopf trägt die drei Begründungen, die der Plan nennt**, und die Deskriptorregel als
vierte: `bail_if_untrusted` bleibt auf `false` (ein fremdes Repository wird gelesen, seine
Konfiguration darf nichts starten); `thread_limit` wird nicht gesetzt, und die Zeile steht
namentlich als erster Hebel, weil die Frage aus den Eingaben nicht entscheidbar ist;
`Outcome::write_changes` wird nicht gerufen und `EntryStatus::NeedsUpdate` gelesen und verworfen,
mit dem Verweis auf den offenen Datensatz; ab welcher `gix`-Fassung jeder angesprochene Weg steht.

**Eine Entscheidung, die der Plan offenließ und die gemessen werden musste:** `UntrackedFiles`.
`gix` fasst unverfolgte Einträge ab Werk zusammen, und dann liefert ein vollständig unverfolgter
Unterordner **einen** Eintrag — nämlich den angezeigten Ordner selbst. Unter ihm bleibt kein Pfadteil
übrig, dem eine Marke gälte, und die Markenliste kommt leer zurück. Am Prüfrepository gemessen und
zuerst als roter Probelauf aufgefallen. Gebaut ist deshalb `UntrackedFiles::Files`; die Kosten (18,8 ms
für 10 000 unverfolgte Dateien) stehen im Doc-Kommentar. Dieselbe Zeile übergeht
`status.showUntrackedFiles` bewusst: was die Spalte zeigt, soll nicht von der Konfiguration eines
fremden Baums abhängen.

**Der Name ist der des Eintrags im angezeigten Ordner und nicht der repositoryrelative Pfad.** Ein
Befund tief im Unterbaum fällt auf den Ordner, über den er zu erreichen ist; sonst trüge ein Ordner
nie eine Marke, und die Zusammenfassung zählte Namen, die in der Liste gar nicht stehen. Beide
Seiten des Vergleichs werden vorher aufgelöst, weil `/tmp` auf macOS eine Verknüpfung auf
`/private/tmp` ist und der Vergleich sonst an jedem Prüfordner scheiterte.

### `crates/krk-core/src/git/texte.rs`

Die drei Sätze aus A14 als Konstanten, dazu `kopfzeile` (A6), `zusammenfassung` (A3) und
`verlaufszeile` (A5), jede eine reine Funktion, jede mit Probe. Neun Proben im `#[cfg(test)]`-Modul.

**A3 und A14 berühren einander an einer Stelle, und sie ist aufgelöst:** A3 verlangt den Zusatz „in
diesem Ordner" am Satz, A14 schreibt für den unveränderten Ordner den Wortlaut `unverändert` aus.
Gebaut ist A14 für den leeren Fall und A3 für jeden anderen; der Doc-Kommentar sagt es, und eine
Probe hält es fest.

**Das Datum kommt aus `leseprofil::bausteine::kalendertext`** und wird nicht zweitgeformt. Die
Funktion war privat und ist jetzt `pub`, mit dem zweiten Rufer und dem Grund im Doc-Kommentar. Eine
zweite Datumsform wäre eine zweite Antwort auf die Frage, wie dieses Vorhaben ein Datum ohne AppKit
schreibt.

### `crates/krk-core/tests/git.rs`

Zehn Proben, alle über `tests/gemeinsam/mod.rs`; **keine vierte Prüfordner-Fassung entsteht**, und
`genau_drei_pruefordner_fassungen_stehen_im_baum` ist grün nachgefahren.

| Probe | Kriterium |
|---|---|
| `der_kopf_nennt_den_branch` | C3.1 |
| `ein_abgeloester_kopf_traegt_den_kurzhash` | C3.6 |
| `ein_repository_ohne_commit_nennt_den_branch_und_liefert_keinen_verlauf` | C3.7 |
| `ein_unterordner_gilt_als_repository_und_seine_zusammenfassung_meint_ihn` | C3.10, C7.7 |
| `der_erste_aufruf_liefert_fuenfzig_commits` | C4.1 |
| `drei_commits_liefern_drei_und_melden_das_ende` | C4.5 |
| `die_fuenf_zustaende_tragen_ihre_fuenf_buchstaben` | C5.3, A11 |
| `jede_marke_steht_genau_einmal_in_alle` | die `ALLE`-Liste, die der Übersetzer nicht hält |
| `ein_ordner_ohne_repository_wird_entschieden_verneint` | C6.5 |
| `ein_deskriptormangel_laesst_den_gitbefund_unentschieden` (+ Kindprobe) | C7.8, C7.9 |

Die zehnte ist die eine über die neun des Plans hinaus: `Marke::ALLE` ist dieselbe stille Stelle wie
`Bereich::ALLE`, und sie wird über `gemeinsam::varianten_der_aufzaehlung` gehalten, also über die
Bauform, die `Kommando::KENNUNGEN` schon hält.

**Die Kindprobe misst zwei Dinge.** Ohne einen freien Deskriptor liefert `oeffnen` `Unentschieden`
und nicht `KeinRepository` — das ist C7.8. Mit 30 freien Deskriptoren kommen alle vier Auskünfte
zustande — das ist C7.9, „niedriger zweistelliger Bereich", und die Zahl ist gemessen und nicht
gesetzt: die Probe scheitert, sobald der Leser mehr braucht.

Die Prüfrepositorys entstehen mit `/usr/bin/git` und nicht mit `gix`. Der Grund steht im Kopf der
Datei: eine Probe, die ihren Gegenstand auch als Werkzeug benutzt, prüft die Übereinstimmung der
Kiste mit sich selbst, und die Stufe A schreibt ohnehin nicht. Jeder Aufruf fährt mit eigener
Identität und ohne die `~/.gitconfig` des Geräts.

## Womit der Schritt angehalten ist

**`make check` ist an genau einer Probe rot, und sie gehört einer fremden Runde.**
`xtask_ruft_git_an_genau_einer_stelle` (`xtask/src/release.rs`, C3.13 der Runde 8) liest jede
`.rs`-Datei unter der Projektwurzel und verlangt genau einen Aufruf von `git`, in
`xtask/src/git.rs`. `crates/krk-core/tests/git.rs` bringt zwei weitere.

Weder Spec noch Plan nennen die Kollision. Sie ist unvermeidlich: ein Prüfrepository lässt sich in
diesem Baum nur mit `/usr/bin/git` anlegen, weil die Stufe A nicht schreibt (E8, Bedingung 2), und
ohne Prüfrepository fallen sieben der neun Proben aus Schritt 3 ersatzlos weg.

**Der Coder hat `xtask/src/release.rs` nicht angefasst.** Die Datei steht nicht in der Dateiliste
des Schrittes, und die Probe trägt ein Abnahmekriterium einer fremden Runde. Der Datensatz ist
gefilt, mit drei Möglichkeiten und einer Empfehlung:
`decisions/260830-1612_o_darf-eine-probe-git-rufen-oder-bleibt-es-bei-genau-einer-aufrufstelle-im-ganzen-baum.md`.

## Prüfläufe

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | Exit 0 |
| `cargo test -p krk-core --test git` | 10 bestanden, 1 stillgelegt (die Kindprobe), 0 gescheitert |
| `cargo test -p krk-core --lib git` | 9 bestanden |
| `cargo test --workspace` | jedes Ziel grün außer `xtask`: dort 154 bestanden, 1 gescheitert |
| `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0 |
| `cargo fmt --all --check` | Exit 0 |
| `make check` | **Exit 2**, allein an `xtask_ruft_git_an_genau_einer_stelle` |

## Gefilte Datensätze

- `decisions/260830-1612_o_darf-eine-probe-git-rufen-oder-bleibt-es-bei-genau-einer-aufrufstelle-im-ganzen-baum.md`
- `issues/260830-1613_o_c8-3-nennt-98-zusaetzliche-pakete-am-projektbaum-gemessen-sind-es-101.md`
- `issues/260830-1614_o_c3-8-verlangt-null-treffer-fuer-write-changes-c10-3-verlangt-treffer-die-die-lesestelle-nennen.md`

## Was der nächste Schritt wissen muss

Schritt 4 baut `Gitlauf` auf diesen vier Funktionen. Die Regel, an die er sich zu halten hat, steht
im Kopf von `crates/krk-core/src/git/mod.rs`: **`None` heißt unentschieden, und der Lauf meldet
dann nichts.** Also kein `Gitmeldung::Kopf` bei `Oeffnung::Unentschieden`, kein
`Gitmeldung::Marken` bei `marken() == None`. `Kopf::KeinRepository` setzt der Lauf selbst, wenn
`oeffnen` `KeinRepository` liefert. `ObjectId` steht als `krk_core::git::ObjectId` bereit, damit
`krk-ui` `gix` nicht führen muss.
