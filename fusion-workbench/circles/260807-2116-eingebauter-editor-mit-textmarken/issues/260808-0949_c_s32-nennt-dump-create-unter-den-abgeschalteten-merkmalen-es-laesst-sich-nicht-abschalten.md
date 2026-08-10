S32 nennt `dump-create` unter den abgeschalteten Merkmalen; es lässt sich nicht abschalten

---

S32 und `### Frage 2` des Plans nennen beide fünf Merkmale von `syntect`, die
abgeschaltet bleiben sollen: `html`, `plist-load`, `yaml-load`, `dump-create` und
`metadata`. Vier davon lassen sich abschalten, `dump-create` nicht.

Der Grund steht in der `Cargo.toml` von `syntect` 5.3.0:

```
parsing = [regex-syntax, fnv, dump-create, dump-load]
```

`parsing` ist das Merkmal, ohne das die Kiste nichts tut, und es zieht
`dump-create` mit. Die Angabe im Plan ist damit sachlich falsch, nicht bloß
unerfüllt.

---

Die Wirkung ist gering: `dump-create` bringt `bincode` und `flate2` mit, die über
`dump-load` ohnehin im Baum stehen. Kein Ausschlusskriterium von S32 ist berührt,
insbesondere nicht das zweite: der Baum von `krk-ui` enthält weiterhin keine Kiste,
die eine C-Werkzeugkette verlangt (geprüft am 260808-0948 über `cargo tree -p
krk-ui -e normal,build`, 195 Zeilen, kein `-sys`, kein `cc`, kein Oniguruma).

Der Befund steht bereits als Kommentar an der Versionsangabe in der
Wurzel-`Cargo.toml`, damit der nächste Leser ihn nicht ein zweites Mal
nachschlägt. Dieser Datensatz hält fest, dass auch der Plantext nachgezogen
gehört.

Gemeldet von: `coder`, bei der Umsetzung von S32.

---
Resolved: Der Plantext ist nachgezogen. `dump-create` steht weder in
`### Frage 2` noch in S32 länger unter den abgeschalteten Merkmalen; beide
Stellen nennen jetzt vier und sagen dazu, warum es nicht fünf sind:
`parsing = [regex-syntax, fnv, dump-create, dump-load]` in der `Cargo.toml` von
`syntect` 5.3.0 zieht es mit, und `parsing` ist das Merkmal, ohne das die Kiste
nichts tut. Die Angabe war damit sachlich falsch und nicht bloß unerfüllt, und
der Plan sagt das so. Die geringe Wirkung ist mitgenommen: `dump-create` bringt
`bincode` und `flate2` mit, die über `dump-load` ohnehin im Baum stehen, und kein
Ausschlusskriterium von S32 ist berührt.

Der Kommentar in der Wurzel-`Cargo.toml` trug den Befund schon und ist
unverändert. Kein `[DONE]`, kein Status im Kopf und kein Code ist angefasst.
`cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` beenden
mit 0.
