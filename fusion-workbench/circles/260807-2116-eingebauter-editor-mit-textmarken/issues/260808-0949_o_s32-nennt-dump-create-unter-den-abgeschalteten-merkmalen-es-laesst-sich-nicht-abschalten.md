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
