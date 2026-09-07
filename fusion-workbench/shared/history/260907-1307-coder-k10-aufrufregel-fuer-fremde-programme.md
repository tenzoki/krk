# K10: die Regel, wann `xtask` ein fremdes Programm über den Suchpfad ruft, steht ausgeschrieben

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Auftrag

Der Nutzer hat am 260907 Möglichkeit 1 von
`260821-1221_*_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md`
gewählt: Suchpfad für nachinstallierte Programme, fester Pfad für die mit macOS gelieferten, und
die Regel wird ausgeschrieben. Erst erheben, welche Aufrufe es gibt und welcher Art sie sind, dann
die Regel an genau eine Stelle schreiben, dann Abweichungen als Befund ablegen. Nichts umbauen.

## Erhebung

Baumstand `d4f28bc`. Erhoben mit `grep -rn 'Command::new(' xtask/src`, jede Fundstelle einzeln
gelesen und gegen die Herkunft ihres Programms gehalten (`which <name>` auf dem Referenzgerät).

Mit vollem Pfad, sechs Namen
(`grep -rhoE 'Command::new\("/usr/bin/[a-z]+"' xtask/src | sort -u`): `codesign`, `ditto`, `git`,
`lipo`, `security`, `xcrun`. Alle unter `/usr/bin` vorhanden, alle mit macOS geliefert, alle
regelkonform.

Über den Suchpfad, drei Bauformen. Ausgeschriebener Name (`grep -rnE 'Command::new\("[a-z]'`):
`iconutil` (`bundle.rs:463`) und `rustup` (`release.rs:639`). Über eine Konstante: `gh` viermal
über `const GH` (`veroeffentlichung.rs:192`, `:195`, `:796`, `:839`). Über eine Funktion: `cargo`
dreimal über `bundle::cargo()` (`bundle.rs:537`, `messen.rs:71`, `version.rs:303`), das `CARGO`
liest und erst ohne die Variable auf den blossen Namen zurückfällt.

**Ein Befund.** `iconutil` liegt unter `/usr/bin/iconutil`, wird also mitgeliefert, und wird
trotzdem über den Suchpfad gerufen. Das ist die einzige Aufrufstelle in `xtask/`, die nach der
Regel falsch liegt; `rustup`, `gh` und `cargo` werden nachinstalliert und liegen richtig.
Abgelegt als `260907-1307_*_iconutil-liegt-nach-der-neuen-aufrufregel-auf-der-falschen-seite-und-wird-ueber-den-suchpfad-gerufen.md`,
nicht behoben, weil der Auftrag das Aufschreiben und nicht das Vereinheitlichen war.

**Ein Nebenbefund, im selben Zug behoben statt abgelegt.** Der Modulkopf von
`veroeffentlichung.rs` nannte `grep -rn 'Command::new("[a-z]' xtask/src` als das Kommando, das die
Suchpfad-Aufrufe zählt. Es sieht die drei `cargo`-Stellen nicht, weil sie ihren Programmnamen aus
einer Funktion beziehen. Der Satz ist beim Umschreiben derselben Stelle weggefallen; die Regel im
Kistenkopf nennt jetzt das weitere Kommando und sagt ausdrücklich, dass ein Aufruf seinen Namen
aus einer Konstanten oder einer Funktion beziehen kann.

## Wo die Regel steht

`xtask/src/main.rs`, Modulkopf, Abschnitt `# Wie dieses Werkzeug ein fremdes Programm ruft`.

**Warum dort.** Die Aufrufe verteilen sich über acht der zehn Module; eine Datei, die sie bündelt,
gibt es nicht, also hat „der Modulkopf der Datei, die die Aufrufe bündelt" keinen Gegenstand.
`main.rs` ist die Kistenwurzel, steht über jedem Aufrufort und ist die erste Seite, die
`cargo doc` zeigt. Die `README.md` ist verworfen: sie richtet sich an den, der baut und ausliefert,
und die Regel bindet den, der einen Aufruf hinzufügt — also einen Leser des Quelltexts. Ein Zeiger
dorthin wäre eine zweite Fläche, die wahr zu halten wäre, ohne einen Leser zu gewinnen.

Die Regel nennt das Kriterium (mit macOS geliefert gegen nachinstalliert) und keine Namensliste,
dazu die zwei Zählkommandos. Sie sagt, warum die Grenze dort liegt, was der Suchpfad kostet, dass
`cargo` mit `CARGO` eine genauere Auskunft vor den Suchpfad stellt statt eine dritte Regel zu sein,
und dass keine Probe sie hält und keine sie halten kann: ob macOS ein Programm mitliefert, steht
nicht im Quelltext der Aufrufstelle, sondern auf dem Gerät.

Fünf Stellen zeigen jetzt darauf und behalten nur, was ortsfest ist: `bundle.rs` bei
`SYMBOLGROESSEN` (die Abweichung) und bei `cargo` (der Rückfall), `release.rs` bei `ziele_pruefen`
(wo `rustup` liegt), `veroeffentlichung.rs` im Modulkopf (wo Homebrew `gh` hinlegt), `git.rs` bei
`rufen` (der volle Pfad, jetzt mit dem Grund statt mit „weil der Baum es so tut").

## Verifikation

Alle fünf Abnahmekommandos mit `export PATH="$HOME/.cargo/bin:$PATH"`:

- `cargo build --workspace` — exit 0
- `cargo test --workspace` — exit 0
- `cargo clippy --workspace --all-targets -- -D warnings` — exit 0
- `cargo fmt --all --check` — exit 0
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — exit 0

Nicht committet; der Orchestrator committet.

## Abgelegte Datensätze

- `260907-1307_*_iconutil-liegt-nach-der-neuen-aufrufregel-auf-der-falschen-seite-und-wird-ueber-den-suchpfad-gerufen.md` — neu, offen.
- `260821-1221_*_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md` — `Implemented:` angehängt, Marker von beantwortet auf umgesetzt gezogen.
