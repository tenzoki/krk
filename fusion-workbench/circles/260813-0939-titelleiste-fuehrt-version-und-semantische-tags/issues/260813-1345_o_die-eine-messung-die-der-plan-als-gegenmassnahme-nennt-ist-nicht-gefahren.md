Die eine Messung, die der Plan als Gegenmaßnahme nennt, ist nicht gefahren und nicht als Verzicht vermerkt

---

Die Risikotafel des Plans
(`planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md`, Abschnitt
`## Risks & Mitigations`) führt eine Zeile, deren Gegenmaßnahme eine Messung ist:

> `bundle::VERSION` veraltet gegenüber der `Cargo.toml` — `inference:` Cargo übersetzt `xtask`
> neu, sobald das Manifest sich ändert. **Ungemessen in diesem Baum; D2 misst es einmal**,
> indem es die Version probeweise anhebt, `cargo xtask release` fährt und die Meldung liest.

Sie ist nicht gefahren. Das Sitzungsprotokoll von Strang D
(`history/260813-1235-coder-strang-d-tagpruefung-und-readme.md`) führt `cargo xtask release`
unter „Nicht gefahren", mit der Begründung, ein Lauf überschriebe das beglaubigte Bündel unter
`target/KRK.app`.

---

**Schwere:** niedrig. Die Begründung des Ausführers trägt: der offene Defekt
`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-…` beschreibt genau
diese Lage, und der Spec dieser Runde hält ihn ausdrücklich draussen. Der Verzicht war richtig.

**Was fehlt, ist nicht die Messung, sondern ihr Vermerk.** Eine Gegenmaßnahme, die eine
`inference:` in eine Messung überführen soll und dann unterbleibt, lässt die `inference:`
stehen — aber die Risikotafel liest sich nach der Runde so, als sei sie gemessen worden. Weder
das Protokoll noch der Plan sagt, dass die Zeile offen geblieben ist und warum.

**Der Sache nach ist das Risiko klein und der Grund benennbar.** `bundle::VERSION` ist
`env!("CARGO_PKG_VERSION")` (`xtask/src/bundle.rs:47`), und `xtask` erbt die Zahl über
`version.workspace = true`. Cargo übersetzt eine Kiste neu, sobald ihr Manifest oder das des
Workspace sich ändert; ein veraltetes `VERSION` verlangte, dass Cargo diese Neuübersetzung
ausliesse. Gemessen ist es in diesem Baum trotzdem nicht, und die Zeile hat es deshalb
ausdrücklich als `inference:` geführt.

**Was zu tun ist**

Eines von dreien:

1. **Die Messung ohne `release` fahren.** Die Version in der Wurzel-`Cargo.toml` probeweise
   anheben, `cargo build -p xtask` fahren und `bundle::VERSION` über eine Wegwerfprobe oder den
   Hilfetext auslesen. Das kostet keinen Bündelbau und beantwortet dieselbe Frage.
2. **Den Verzicht in der Risikotafel vermerken**, mit dem Grund und dem Datum. Dann steht die
   `inference:` bewusst da und nicht versehentlich.
3. Die Zeile streichen, sobald der Defekt `260813-0026` entschieden ist und ein
   Auslieferungslauf wieder billig zu fahren ist.

Der erste Weg ist der kleinste und der einzige, der die Frage wirklich beantwortet.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Die übrigen acht Zeilen der Risikotafel sind eingelöst: `Left` statt `Leading`
  (`titelzusatz.rs:192`), die Blattlage kommt durch (Tafel `zulaessigkeit.rs:435`), der
  Freigabedialog-Defekt der Runde 6 ist nicht geschlossen worden, die Stationszählung ist an
  allen drei Stellen nachgezogen, D1 kommt ohne Verzeichnis aus, der Anfangstitel steht auf der
  leeren Zeichenkette (`fenster.rs:455`), und die vier Aufzählungen sind bei 76, 7, 5 und 5
  geblieben — beim Abgleich einzeln nachgezählt.
