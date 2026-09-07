# Jedes Blatt liefert seinen Bauplan als reine Funktion (K8)

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was gefragt war

Möglichkeit 3 aus
`260818-0250_*_verlangt-der-blattbauer-die-liegenlassende-schaltflaeche-am-typ.md`
(Entscheidungsspeicher des Circles der Runde 12), vom Nutzer am 260907 gewählt:
jedes Blatt bekommt eine reine Funktion, die nur seinen Bauplan liefert, und
wird damit ohne AppKit prüfbar. Die Signatur von `Blatt::neu` und
`Blatt::mit_schaltflaechen` bleibt unangetastet, die Reihenfolge der
Schaltflächen ebenso, das `assert!` bleibt stehen.

## Die Erhebung

Ein Blatt ist eine Aufrufstelle des einen Bauers. Gezählt mit

```sh
grep -rln 'Blatt::mit_schaltflaechen\|Blatt::neu(' crates/*/src --include='*.rs'
```

Elf Blätter, verteilt auf elf Dateien: zehn unter
`crates/krk-ui/src/appkit/blaetter/` und die Belegungsansicht daneben
(`crates/krk-ui/src/appkit/belegungsansicht.rs`). Die zwei weiteren Treffer des
Kommandos, `blaetter/mod.rs` und `appkit/anwendung.rs`, sind der Bauer selbst
und eine Nennung im Doc-Kommentar.

Eine Bauplanfunktion trugen vorher:

- `standardschaltflaechen` (`blaetter/mod.rs`) für die fünf Blätter aus
  `Blatt::neu` (Pfadeingabe, Zeilennummer, Suche, Namenseingabe,
  Stapelumbenennen),
- `loeschbestaetigung::schaltflaechen`,
- `konflikt::schaltflaechen` — dazugekommen am 260907 in `0115cf5`, mit der
  `Konfliktgestalt`.

**Damit sind es vier übrige und nicht die fünf, von denen der Datensatz vom
260818 spricht:** das Konfliktblatt hat seine Bauplanfunktion heute schon
bekommen.

## Was gebaut ist

Je Blatt eine private, reine `#[must_use]`-Funktion, die allein die Liste der
`Schaltflaeche` liefert, und je Funktion eine Probe, die die Reihenfolge liest.
Keine zweite Bauart: dieselbe Form wie die zwei vom 260818.

| Datei | Funktion | Probe |
|---|---|---|
| `blaetter/uebersprungen.rs` | `schaltflaechen` | `der_bauplan_traegt_die_eine_schliessende_schaltflaeche` |
| `blaetter/ungesichert.rs` | `schaltflaechen` | `der_bauplan_zaehlt_sichern_verwerfen_abbrechen_in_dieser_reihenfolge`, `abbrechen_faengt_auf_und_die_eingabetaste_sichert` |
| `blaetter/zettel.rs` | `schaltflaechen` | `der_bauplan_traegt_fertig_auf_der_escape_taste` |
| `appkit/belegungsansicht.rs` | `blattschaltflaechen` | `der_bauplan_des_blattes_traegt_fertig_auf_cmd_eingabe` |

Der abweichende Name in der Belegungsansicht ist begründet und steht im
Doc-Kommentar: die Datei führt bereits eine Tafel `SCHALTFLAECHEN`, und das
sind die drei Knöpfe **in** der Beigabe, von denen nur einer dem Blatt gehört.
Zwei Namen derselben Schreibweise für zwei verschiedene Mengen wären die
Verwechslung, die der eigene Name vermeidet.

Dazu ein neuer Abschnitt im Modulkopf von `blaetter/mod.rs`, „Jedes Blatt
liefert seinen Bauplan als reine Funktion": er nennt die Form, den Ertrag, das
Erhebungskommando statt einer Zahl, und schreibt aus, dass der eine Bauer
bleibt und die Zusage weiter am `assert!` hängt und nicht am Übersetzer.

## Was unangetastet blieb

- Die Signatur von `Blatt::neu` und `Blatt::mit_schaltflaechen`.
- Der `unwrap_or(0)` in `abbruchstelle` und seine Tafel.
- Das `assert!` in `Blatt::mit_schaltflaechen`.
- Die Reihenfolge jeder Schaltfläche in jedem Blatt; die neuen Proben lesen sie
  Stelle für Stelle und werden rot, sobald sich eine dreht.
- Die Zählprobe `jedes_blatt_nennt_seine_liegenlassende_schaltflaeche`
  (`blaetter/mod.rs`) — sie zählt je Datei die Aufrufstellen des Bauers und
  verlangt `Wirkung::Liegenlassen` in derselben Datei. Jede der vier Marken ist
  in ihrer Datei geblieben, nur in einer Funktion statt an der Aufrufstelle; die
  Probe bleibt grün.

## Prüfung

```
cargo build --workspace                              exit 0
cargo test --workspace                               exit 0
cargo clippy --workspace --all-targets -- -D warnings exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps exit 0
cargo fmt --all --check                              exit 1
cargo fmt -p krk-ui --check                          exit 0
```

Der Fehlschlag von `cargo fmt --all --check` liegt außerhalb dieser Bahn: die
einzige gemeldete Stelle ist `crates/krk-core/tests/belegung.rs:376`, eine Datei
im Zugriff einer parallel laufenden Bahn und für diese Bahn gesperrt. `git diff`
zeigt die Stelle als deren offene Änderung. Über `krk-ui`, die einzige Kiste
dieser Bahn, läuft `cargo fmt` sauber.

## Datensätze

`260818-0250_*_verlangt-der-blattbauer-die-liegenlassende-schaltflaeche-am-typ.md`
auf `_i_` gesetzt, mit einem `Implemented:`-Vermerk, der auf diesen Verlauf
zeigt. Ein Commit-Hash steht nicht darin: der Orchestrator committet, und die
Zahl gibt es beim Schreiben des Vermerks noch nicht.
