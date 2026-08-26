Drei behobene `CLAUDE.md`-Datensätze stehen weiter offen, und niemand ist dafür beauftragt

---

`fb50fcd` hat die Aussagen berichtigt, die drei offene Defektdatensätze verlangen. Alle drei
tragen weiter den Marker `_o_`. Der Kuratorenlauf hat den Marker ausdrücklich nicht bewegt und
ihn dem `reconciler` zugewiesen; der letzte Abgleich lief um 260826-0157 und damit **vor**
`fb50fcd` (260826-0831). Seither hat niemand die Marker nachgezogen, und ein
`find … -name '*_o_*.md'` meldet drei Defekte, die es nicht mehr gibt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

**Domain:** code

**Betroffen:** `fusion-workbench/shared/issues/`

## Die drei, und wie weit `fb50fcd` sie trägt

| Datensatz | Was er verlangt | Stand nach `fb50fcd` |
|---|---|---|
| `260823-1336_o_claude-md-nennt-einen-empfaenger-der-ersthelfermeldung-der-baum-traegt-seit-dem-260819-zwei.md` | „Den Satz von einem auf zwei Empfänger stellen, in der Reihenfolge, in der sie laufen, und die Zusage … ausdrücklich auf `fokusanzeige_nachziehen` beschränken" | **vollständig umgesetzt** (Eintrag L07). Vor dem Schließen ist die Runden-Zuschreibung im Rumpf zu berichtigen, siehe den Datensatz daneben |
| `260823-1649_o_claude-md-sagt-die-version-sei-seit-dem-260815-an-jedem-tag-gestiegen-am-260822-ist-sie-es-nicht.md` | „Wer ihn ersatzlos streicht, verliert nichts an der Begründung" | **vollständig umgesetzt** (Eintrag L08). Der Halbsatz ist weg |
| `260820-2056_o_claude-md-nennt-eine-zaehlprobe-unter-einem-namen-den-der-baum-nicht-traegt.md` | den richtigen Probennamen nennen | **schon vorher behoben.** `CLAUDE.md` nennt `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`, `cargo test --workspace` fährt sie unter diesem Namen. Der Kuratorenlauf nennt diesen Fall selbst, Abschnitt 9 Punkt 1 |

**Ein vierter bleibt zu Recht offen und gehört nicht in diese Liste.**
`260825-1859_o_claude-md-nennt-fuer-zip-das-eine-merkmal-deflate-flate2-es-sind-zwei.md` ist in
`CLAUDE.md` behoben (Eintrag L05), aber seine `Also seen`-Zeile trägt dieselbe Wendung in der
Wurzel-`Cargo.toml`, und dort steht sie unverändert: `Cargo.toml:155-158`, „`default-features =
false` mit dem **einen** Merkmal `deflate-flate2` laesst genau das Deflate-Verfahren uebrig".
Eine Bauvorschrift gehört dem `coder`. Dieser Datensatz schließt erst mit jener Zeile.

**Ebenfalls zu Recht offen:**
`260826-0149_o_die-runde-18-hat-keinen-circle-datensatz-….md` — `fb50fcd` hat davon die
Möglichkeit 3 umgesetzt (der Satz in `CLAUDE.md`), die Möglichkeit 2 (der Circle-Datensatz)
gehört nach dem Datensatz selbst dem Nutzer.

## Warum das trägt

Die Konvention sagt für einen Defekt: „The fix and the closure are the same event." Hier fallen
sie auseinander, weil zwei Agenten sich die Arbeit teilen und der zweite nicht gerufen wurde.
Das Ergebnis auf der Platte ist ein Fehlbefund in genau der Richtung, die am teuersten ist: ein
offener Datensatz, dessen Sache erledigt ist, kostet jeden folgenden Leser eine Prüfung am
Baum, und die Prüfung fällt weg, sobald jemand ihm glaubt.

**Schwere:** mittel. Kein Verhalten der Anwendung, aber drei falsche Auskünfte im Speicher, den
`CLAUDE.md` als verbindlich benennt („verbindlich ist der Dateibestand, nicht diese Zeile").

## Vorschlag

Die zwei vollständig umgesetzten (`260823-1336` nach der Berichtigung seines Rumpfs,
`260823-1649`) und den dritten (`260820-2056`) auf `_c_` umbenennen und je eine `Resolved:`-Zeile
mit dem Commit eintragen. Für `260825-1859` bleibt die eine Zeile in `Cargo.toml` zu richten.

**Gefunden:** coderev, Durchsicht von `e5ec81a..20c9833` am 260826-0923

---
Resolved: dieser Abgleich (`shared/history/260826-1017-reconciliation.md`) — die drei Marker sind
nachgezogen, jeder nach eigener Lesung am Baum und nicht nach dem Bericht, der die Erledigung
behauptet:

- `260823-1336` → `_c_`, mit `Resolved: fb50fcd` und einem `Revised by: c95f28b` für die
  Runden-Zuschreibung, die der Datensatz zweimal im Rumpf führt. Die Berichtigung ist damit
  vorgenommen, bevor der Datensatz schließt, wie diese Tabelle es verlangt.
- `260823-1649` → `_c_`, mit `Resolved: fb50fcd`. Die leere `Resolved:`-Zeile, die der Datensatz
  seit dem 260823 trug, ist dabei ausgefüllt worden.
- `260820-2056` → `_c_`, mit `Resolved: 90f8ac1`, also mit dem Commit, der die Aussage schon am
  260820 gerichtet hat, und nicht mit `fb50fcd`.

Die zwei, die der Datensatz zu Recht offen nennt, sind einzeln nachgeprüft. `260825-1859` bleibt
offen: `Cargo.toml:157` trägt unverändert „mit dem einen Merkmal `deflate-flate2`", während die
Merkmalsliste zwei Zeilen weiter zwei führt; das ist Arbeit des `coder`. `260826-0149` ist
geschlossen, aber nicht durch diesen Weg — der Nutzer hat den Datensatz am 260826-1017 auf seine
Möglichkeit 3 gekürzt, und die ist mit `fb50fcd` erfüllt.

`make check` über `c95f28b` selbst gefahren, alle vier Kommandos grün.
