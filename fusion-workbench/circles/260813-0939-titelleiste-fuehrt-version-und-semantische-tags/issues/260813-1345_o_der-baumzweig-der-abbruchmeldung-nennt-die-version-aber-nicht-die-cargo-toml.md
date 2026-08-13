Der Baumzweig der Abbruchmeldung nennt die Version, aber nicht die `Cargo.toml`, und keine Probe sieht es

---

C3.8 verlangt drei Dinge von der Abbruchmeldung: „welche Bedingung verletzt ist, welche Version
die `Cargo.toml` führt, und was zu tun ist". Die Meldung hat zwei Zweige, und nur einer davon
nennt die Quelle der Zahl.

**Der Tag-Zweig nennt sie** (`xtask/src/release.rs:243`):

```
Die Cargo.toml fuehrt die Version {version}
```

**Der Baum-Zweig nennt sie nicht** (`xtask/src/release.rs:265-267`):

```
Ein Buendel aus diesem Baum traegt die Version {version} und ist nicht aus dem
Stand gebaut, den {erwartet} benennt.
```

Steht der Tag und ist allein der Arbeitsbaum schmutzig, erfährt der Nutzer die Zahl, aber nicht,
woher sie kommt.

---

**Schwere:** niedrig. Auslegungsfrage, kein Verhalten. Der Nutzer kommt weiter: die Meldung nennt
die Zahl, die geänderten Dateien einzeln und zwei Abhilfen (`git commit -a`, `git stash`).

**Was den Befund über eine Wortklauberei hinaushebt, ist die Probe.**
`die_meldung_nennt_bedingung_version_und_abhilfe` (`xtask/src/release.rs:1056`) fährt allein den
kombinierten Fall, in dem beide Befunde zutreffen — und dort steht der Tag-Zweig mit in der
Meldung und trägt die `Cargo.toml`. Die Probe kann die Lücke deshalb nicht sehen, und sie ist
die einzige, die C3.8 abnimmt. Der Fall „Tag steht, Baum ist schmutzig" hat zwar eine eigene
Probe (`ein_geaenderter_baum_haelt_die_auslieferung_an`, `:996`), und die prüft den Wortlaut der
Meldung nicht auf die drei Bestandteile.

**Was zu tun ist**

Zwei Wege, und der zweite ist der, der die Lücke schliesst:

1. Den Baum-Zweig um die Quelle ergänzen: „traegt die Version {version} aus der Cargo.toml".
   Eine Zeile.
2. Die Probe zu C3.8 über **beide** Zweige einzeln fahren, nicht nur über den kombinierten.
   Ohne diesen zweiten Schritt fällt derselbe Fehler beim nächsten Umbau der Meldung wieder an,
   und die Probe bleibt grün.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Die übrigen Zusagen an der Meldung halten und sind geprüft: beide Befunde stehen in einer
  Meldung (`release.rs:277`), kein `--force` und kein Umgehungsweg (Probe `:1068-1070`), keine
  Umlaute.
