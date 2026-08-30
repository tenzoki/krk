# P-3a: Die Git-Zählprobe klammert `crates/*/tests/` aus

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Cross-references:** `260830-1612_*_darf-eine-probe-git-rufen-oder-bleibt-es-bei-genau-einer-aufrufstelle-im-ganzen-baum.md` (Möglichkeit 2); `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md` (Schritt 3); `260830-1620-coder-schritt-3-gix-und-der-gitleser.md` (der blockierte Vorlauf)

---

## Der Auftrag

Der Blocker aus Schritt 3: `crates/krk-core/tests/git.rs` ruft zweimal `/usr/bin/git`, um
Prüfrepositorys anzulegen, und ließ damit `xtask_ruft_git_an_genau_einer_stelle` mit drei
Treffern statt einem rot laufen. Der Nutzer hat am 260830 Möglichkeit 2 gewählt: die Prüfung
liest nur noch Code außerhalb von `crates/*/tests/`. Die Ausnahmeliste nach dem Muster von
`genau_drei_pruefordner_fassungen_stehen_im_baum` und die Beschränkung auf `xtask/` sind
verworfen und nicht gebaut.

## Was geschehen ist

**Die Probe heißt jetzt `git_wird_ausserhalb_der_probenordner_an_genau_einer_stelle_gerufen`**
(`xtask/src/release.rs:1099`). Der alte Name war eine Aussage, die nach dieser Änderung nicht
mehr stimmt, und ist deshalb mitgewandert. Gezählt wird unverändert `Command::new("/usr/bin/git")`
und `Command::new("git")`, verlangt wird unverändert genau ein Treffer, und der muss unverändert
in `xtask/src/git.rs` liegen.

**Die Grenze steht an einer Stelle**, `liegt_im_probenordner_einer_kiste`
(`xtask/src/release.rs:1287`), neben `rust_dateien`. Sie fragt über `strip_prefix` gegen die
Projektwurzel, ob der Pfad die Gestalt `crates/<kiste>/tests/…` hat. `rust_dateien` selbst
bleibt unangetastet: die zweite Zählprobe `allein_release_fragt_nach_tag_und_arbeitsbaum` und
`der_quellbaum_nennt_die_alte_stationszahl_nicht_mehr` lesen weiter den ganzen Baum.

**Der Preis steht im Doc-Kommentar der Probe**, zweiteilig und mit dem Datensatz zitiert. Erstens
fällt ein zweiter `git`-Rufer unter `crates/*/tests/` nie mehr auf. Zweitens trifft die Grenze
nicht alles, was sie zu treffen vorgibt: `krk-ui` führt kein Bibliotheksziel und prüft deshalb in
`#[cfg(test)]`-Modulen unter `src/`, die ebensowenig ausgeliefert werden und weiter gezählt
bleiben; ein späterer Aufruf von `git` dort macht die Probe wieder rot.

## Die nachgezogenen Prosastellen

| Ort | Was dastand | Was jetzt dasteht |
|---|---|---|
| `xtask/src/release.rs:1076` | „Genau ein Aufruf von `git` im ganzen Baum (C3.13)" | „ausserhalb der Probenordner der Kisten", dazu der Absatz mit dem Preis |
| `xtask/src/git.rs:3` | „der einzige Prozessaufruf von `git` im ganzen Baum" | „im ausgelieferten Code", mit dem Grund für die Grenze |
| `xtask/src/git.rs:66` | „zaehlt im ganzen Baum" | „zaehlt ausserhalb von `crates/*/tests/`", dazu der dritte Fall, den die Probe nicht sieht |
| `xtask/src/veroeffentlichung.rs:509` | der alte Probenname | der neue |

`CLAUDE.md`, `README.md` und das `Makefile` tragen die Zusage nicht und sind unangetastet.
Die Datensätze unter `fusion-workbench/` behalten nach der Ortsregel ihren damaligen Wortlaut.

`crates/krk-core/tests/git.rs` ist nicht angefasst worden; die zwei `git`-Aufrufe dort sind
gewollt und vom Nutzer gedeckt.

## Nachzug am Plan und am Datensatz

Schritt 3 des Plans steht auf `[DONE]` (`:278`). Der Entscheidungsdatensatz trägt seine
`Implemented:`-Zeile und ist von `_a_` auf `_i_` umbenannt; zitiert ist die Fundstelle im Baum
und kein Commit, weil dieser Auftrag nicht committet.

## Abnahme

`make check` — Exit 0 (die vier Ziele: `build`, `test --workspace`, `fmt --check`,
`clippy --all-targets -- -D warnings`). Vorher: Exit 2 an genau dieser einen Probe.
