# Coder: die C-Freiheits-Zusage an ihren sechs Stellen

**Status:** Complete
**Schritt:** 13 des Plans `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`
**Kriterien:** C9.1, C9.2
**Defekt:** `260830-1106_*_der-entscheid-zur-c-freiheits-zusage-nennt-fuenf-prosastellen-im-baum-stehen-sechs.md`
**Entscheid:** `260830-1006_*_wie-lautet-die-c-freiheits-zusage-wenn-linux-raw-sys-in-cargo-lock-steht.md`, Möglichkeit 1

## Was gemessen wurde, bevor etwas geschrieben wurde

Drei Läufe am 260831, `cargo tree` gegen den Projektbaum und nicht gegen einen
Prüf-Workspace:

| Lauf | Baumzeilen | `cc` | Name auf `-sys` | `onig` |
|---|---|---|---|---|
| `cargo tree --target x86_64-apple-darwin -e normal,build` | 673 | 0 | keiner | keiner |
| `cargo tree --target aarch64-apple-darwin -e normal,build` | 674 | 0 | keiner | keiner |
| `cargo tree --target all -e normal,build` (Gegenprobe) | 704 | 0 | `windows-sys 0.61.2`, `linux-raw-sys 0.12.1` | keiner |

Die Gegenprobe ist der Beleg, dass die Null der beiden Mac-Ziele nicht die eines zu engen
Musters ist: dasselbe Muster findet an `--target all` beide Pakete. `Cargo.lock` führt
genau diese zwei Namen (`grep -nE '^name = "(cc|.*-sys)"'`), kein `cc`.
`cargo tree --workspace --target …` liefert Zeichen für Zeichen dieselbe Ausgabe wie ohne
`--workspace`; die kürzere Form aus dem Spec genügt.

## Die Erhebung: sieben Treffer, nicht sechs

Die Vorschrift des Defekts trifft nicht, was sie treffen soll, und beides ist gemessen.
Ihre Alternative `einen solchen Namen` fängt `kommandos/kontextmenue.rs:719` und `:731`
ein, wo es um Dateinamen und nicht um Pakete geht. Und seit Schritt 3 trägt die
Wurzel-`Cargo.toml` eine siebte Stelle, die Begründung zu `gix`; sie stand schon beim
Schreiben in der neugefassten Form.

Die neue Vorschrift steht an einer Stelle, in `CLAUDE.md` beim Absatz zur Zusage:

```sh
grep -rn --exclude-dir=fusion-workbench --include='*.md' --include='*.toml' --include='*.rs' 'Namen auf `-sys`' .
```

Sie ist nur deshalb vollständig, weil jede Stelle die Wendung „Namen auf `-sys`" führt;
das steht neben ihr, denn genau der Verzicht auf den Wortlaut hat die sechste Stelle vor
der Erhebung des Entscheids versteckt. Der erste Entwurf der Vorschrift endete auf
`| grep -v fusion-workbench` und filterte sich selbst weg, weil die Zeile in `CLAUDE.md`
das Wort trägt; der Filter hängt jetzt am Verzeichnis (`--exclude-dir`) und nicht an der
Zeile. Ohne den Probelauf wäre das nicht aufgefallen.

Treffer am 260831, sieben: `Cargo.toml:93`, `:153`, `:279`, `:361`, `:515`,
`CLAUDE.md:87`, `crates/krk-core/src/verzeichnis/sys.rs:75`.

## Was geändert wurde

- `Cargo.toml`, Begründung zu `regex` (`:91-97`), zu `zip` (`:152-157`), zu
  `objc2-pdf-kit` (`:277-282`) und zu `syntect`/`two-face` (`:359-366`): jede trägt die
  neugefasste Form. Die Begründung zu `gix` (`:511-520`) blieb unberührt, sie trug sie
  schon.
- `CLAUDE.md:87`, letzter Satz des Absatzes zu `syntect`, `two-face` und `zip`: die
  neugefasste Form, der Verweis auf den Entscheid, der Satz, dass `windows-sys` damit der
  erste Fall der Regel und nicht mehr ihre Ausnahme ist, und die Erhebungsvorschrift an
  der Stelle, an der sonst eine Zahl stünde. Der Rest der Datei bleibt Schritt 14.
- `crates/krk-core/src/verzeichnis/sys.rs`, Modulkopf: der Satz über die Zeitkiste
  behauptet den Rang „erstes `-sys`-Paket neben `windows-sys`" nicht mehr. Ein eigener
  Absatz sagt, dass der Rang mit dieser Runde an `linux-raw-sys` gefallen ist und dass die
  Frage ohnehin am Bauziel und nicht in `Cargo.lock` entschieden wird.

Keine der sieben Stellen nennt eine Zahl der Prosastellen (C9.2). Kein Eintrag in
`Cargo.toml` und keine Merkmalswahl ist angefasst, `Cargo.lock` ist unberührt: der Schritt
ändert Prosa.

## Abnahme

`make check` (`cargo build --workspace`, `cargo test --workspace`, `cargo fmt --all
--check`, `cargo clippy --workspace --all-targets -- -D warnings`), Exit 0.

## Was offen bleibt

Der Defekt behält `_o_`. Sein Abnahmetest verlangt den Nachtrag in
`260830-1006_*_wie-lautet-die-c-freiheits-zusage-…` und in
`260830-0950-orchestrator-session.md`; beide gehören Schritt 15, und mit ihm schließt er.
