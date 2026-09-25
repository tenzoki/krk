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

---
Resolved: 260906 — **Weg 1 ist gefahren: die Messung ist gemacht, und die `inference:` ist keine mehr.** `bundle::VERSION` veraltet nicht.

**Gemessen am 260906-0201**, in einem Wegwerf-Workspace ausserhalb des Projektbaums, wie es dieses Projekt für Fragen an den Übersetzer hält. Aufbau: ein Workspace mit `[workspace.package] version = "0.0.1"` und einem Mitglied, das `version.workspace = true` erbt und allein `env!("CARGO_PKG_VERSION")` ausgibt.

| Schritt | Ausgabe | Prüfsumme des Binärziels |
|---|---|---|
| `cargo build`, dann laufen lassen | `0.0.1` | `f9adabc4a26889ac` |
| nur die Zahl im Workspace-Manifest auf `9.9.9` gehoben, keine Quelldatei angefasst, `cargo build`, laufen lassen | `9.9.9` | `24ac53869e2f8031` |

Die Änderungszeit von `main.rs` blieb dabei unberührt. Cargo übersetzt die Kiste also allein auf die Manifeständerung hin neu, und die eingebackene Zahl zieht mit. Das ist genau die Annahme, die die Risikotafel als `inference:` geführt hat.

**Der Wegwerf-Workspace statt des Projektbaums, und der Grund ist derselbe wie 260813.** Die Zahl im Projektmanifest probeweise anzuheben, hieße, sie mitten in einer Sitzung zu verstellen, in der zwei weitere Agenten im selben Baum schreiben. Der Wegwerf-Workspace beantwortet dieselbe Frage — es ist eine Frage an Cargo und nicht an KRK — und fasst den Baum nicht an. Der Bündelbau, an dem der Verzicht von 260813 hing, kommt in dieser Form gar nicht vor.

**Nicht angefasst ist die Risikotafel des Plans.** Ihre Zeile sagt weiter, D2 messe es einmal, und D2 hat es nicht getan. Ob ein Plan einer geschlossenen Runde nachträglich berichtigt wird, ist eine eigene Frage und liegt beim Nutzer (`260906-0203_*_darf-ein-agent-den-spec-oder-plan-einer-geschlossenen-runde-berichtigen.md`); die Messung steht deshalb hier und nicht dort.
