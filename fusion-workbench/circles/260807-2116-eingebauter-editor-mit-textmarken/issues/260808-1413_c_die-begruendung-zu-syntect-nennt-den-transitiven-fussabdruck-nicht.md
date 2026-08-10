# Die Begründung zu `syntect` nennt den transitiven Fußabdruck nicht

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht von Turn 1 der Editor-Runde
**Betroffen:** `Cargo.toml:100-161`
**Cross-references:** Plan S32, `decisions/260807-2147_*_fuer-welche-sprachen-hebt-die-formatansicht-syntax-hervor.md`

---

## Der Befund

Die Begründung zu `syntect` und `two-face` hält im Ton der vier bestehenden
Einträge: sie nennt die Alternativen und warum sie ausscheiden, warum
`default-features = false` keine Sparsamkeit, sondern die Bedingung der
Einbindung ist, welche Merkmale abgeschaltet bleiben, und sie benennt den
angenommenen Preis mit `speculation:` statt ihn zu verschweigen. Geprüft und
bestätigt sind dabei alle vier Tatsachenbehauptungen:

| Behauptung | Geprüft mit | Ergebnis |
|---|---|---|
| kein Oniguruma, keine Bauabhängigkeit `cc`, kein `-sys`-Paket | `cargo tree --workspace -e normal,build` | kein Treffer für `cc`, `onig`, `-sys` |
| `dump-create` lässt sich nicht abschalten, `parsing` zieht es mit | `cargo tree -p krk-ui -e features` | bestätigt |
| `html`, `plist-load`, `yaml-load`, `metadata` bleiben aus | `cargo tree -p krk-ui -e features` | bestätigt |
| 213 Sprachdefinitionen gegenüber 75, TOML nur in `two-face` | eigener Lauf gegen beide Kisten | 75 / 213, TOML nur in `two-face` |

**Was fehlt, ist die Zahl der Pakete, die mitkommen.** Der Eintrag zu
`signal-hook` (`Cargo.toml:54-56`) behandelt genau das als tragend:

> Zwei Wirkungen, die hier zählen: `krk-bench` behält `#![deny(unsafe_code)]`, und
> **die Kiste bringt nichts mit außer `signal-hook-registry` und `libc`**, das
> über `objc2` ohnehin schon im Baum steht.

`syntect` und `two-face` bringen 21 weitere Pakete mit — gemessen am Zuwachs in
`Cargo.lock` zwischen `4e86c02` und `HEAD`: 23 neue Einträge, davon zwei die
Kisten selbst. Namentlich `adler2`, `aho-corasick`, `bincode`, `bit-set`,
`bit-vec`, `cfg-if`, `crc32fast`, `fancy-regex`, `flate2`, `fnv`, `memchr`,
`miniz_oxide`, `once_cell`, `regex-automata`, `regex-syntax`, `same-file`,
`simd-adler32`, `thiserror`, `thiserror-impl`, `walkdir`, `winapi-util`.

Das ist der größte Zuwachs am Abhängigkeitsbaum, den dieses Projekt bisher
aufgenommen hat, und der einzige Punkt, an dem die neue Begründung hinter dem
Maßstab der vier bestehenden zurückbleibt.

`walkdir`, `same-file` und `winapi-util` fallen dabei besonders auf: sie hängen
unbedingt an `syntect` und nicht an einem der eingeschalteten Merkmale
(`cargo tree -p krk-ui -i walkdir`), also lässt sich der Baum an dieser Stelle
auch nicht durch eine Merkmalswahl kürzen. Genau diese Aussage gehört in den
Kommentar, damit die nächste Runde sie nicht neu misst.

## Was zu tun ist

Einen Absatz an `Cargo.toml:100` ergänzen, in der Form des `signal-hook`-Eintrags:
wie viele Pakete mitkommen, welche davon nicht abwählbar sind, und dass keines
ein `-sys`-Paket ist oder `cc` als Bauabhängigkeit mitbringt — die drei Aussagen
sind gemessen und tragen die Zusage aus der Technologiewahl, dass sich die
Bauvoraussetzungen des Projekts nicht ändern.

---
Resolved: Die Begründung zu `syntect` in der Wurzel-`Cargo.toml` trägt den
Fußabdruck jetzt, in der Form des `signal-hook`-Eintrags, und die Zahl ist
selbst erhoben und nicht aus diesem Datensatz übernommen.

**21 weitere Pakete**, bestätigt über den Namensvergleich der beiden
`Cargo.lock`-Fassungen: 72 Einträge bei `4e86c02`, 95 heute, 23 neue Namen, zwei
davon die Kisten selbst. Die 21 stehen namentlich im Kommentar.

Drei gemessene Aussagen stehen dazu, jede mit dem Kommando, das sie erhebt:

- **Auf dem Bauziel dieses Projekts kommen 20 davon an.** `winapi-util` hängt
  über `same-file` und `walkdir` allein am Windows-Ziel; `cargo tree --workspace
  -e normal` zählt es nicht mit, es steht nur in `Cargo.lock`. Die beiden
  `windows-*`-Einträge, an denen es hängt, standen schon vor diesen Kisten in der
  Datei und sind deshalb nicht als Zuwachs geführt. Diese Unterscheidung war in
  diesem Datensatz nicht enthalten.
- **`walkdir`, `same-file` und `winapi-util` lassen sich nicht abwählen.** In
  `syntect`s eigener `Cargo.toml` (5.3.0) steht `[dependencies.walkdir]` ohne
  `optional`, hängt also an keinem eingeschalteten Merkmal, und `same-file` samt
  `winapi-util` hängt an ihm. Der Kommentar sagt ausdrücklich, dass an dieser
  Stelle keine Merkmalswahl den Baum kürzt, damit die nächste Runde es nicht neu
  messen muss.
- **Keines der 21 ist ein `-sys`-Paket, keines bringt `cc` als Bauabhängigkeit
  mit.** `cargo tree --workspace -e normal,build` findet im ganzen Baum weder
  `cc` noch `onig` noch einen Namen auf `-sys`. Damit hält die Zusage aus der
  Technologiewahl.

Der Eintrag zu `two-face` wiederholt die Zahl nicht, sondern verweist auf den zu
`syntect`: `two-face` hängt selbst an ihm, und zwei Zahlen für einen Baum wären
zwei Wahrheiten.

Kein Code ist angefasst. `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` beenden
mit 0.
