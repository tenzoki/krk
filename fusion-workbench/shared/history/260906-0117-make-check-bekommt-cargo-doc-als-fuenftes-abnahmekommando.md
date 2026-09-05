# `make check` bekommt `cargo doc` als fuenftes Abnahmekommando

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Baum bei Beginn:** `28c4a47`
**Umfang:** `Makefile`, `README.md`

## Auftrag

Das Tor schliessen, das die drei Behebungslaeufe vom 260905/260906 (`ffe7384`, `fc6564e`,
`59d0688`) offengelassen haben: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
als fuenftes Kommando von `make check`. Grundlage ist
`260905-2336_*_wird-ein-privates-element-oeffentlich-oder-der-verweis-darauf-zu-fliesstext.md`,
dessen Randbedingungen die Toraufnahme ausdruecklich nennen. Der Datensatz ist unberuehrt
geblieben; sein Uebergang gehoert dem Orchestrator.

Das offene Ende ist im Protokoll `260906-0034-doc-tor-fuer-krk-core-und-zwei-defekte-der-durchsicht.md`
unter `## Noch offen aus dem Entscheidungsdatensatz` benannt. Es ist jetzt geschlossen.

## Was geaendert ist

- `Makefile` — ein neues Ziel `doc` zwischen `fmt-check` und `check`, mit `##`-Zeile fuer
  `make help` wie die vier Ziele daneben. `check` fuehrt es als fuenfte Voraussetzung und
  meldet `alle fuenf gruen`.
- `README.md`, Abschnitt `## Bauen` — das Kommando im Kommandoblock, `vier Kommandos` zu
  `fuenf Kommandos`, dazu ein Absatz, was das Ziel haelt und warum `--no-deps` dazugehoert.

Die zwei anderen Stellen der `README.md`, die `make check` nennen (Station 1 der
Auslieferungskette, die Tagpruefung), sagen nichts ueber die Zahl der Kommandos und sind
unberuehrt.

## Warum `RUSTDOCFLAGS` als Rezept-Praefix und nicht als make-Variable

Die Zuweisung steht im Rezept vor dem Kommando:

```make
	RUSTDOCFLAGS="-D warnings" $(CARGO) doc --workspace --no-deps
```

Damit reicht die Shell die Variable in genau die Umgebung dieses einen Aufrufs. Ein
`export RUSTDOCFLAGS := …` am Kopf der Datei taete dasselbe fuer `doc`, gaelte dann aber
auch fuer `build`, `test` und `lint`, die davon nichts wissen sollen. Die Begruendung steht
als Kommentar am Ziel.

## Der Gegenbeweis: das Tor prueft wirklich

Eine Umgebungsvariable, die im Ziel gesetzt wird, aber den `cargo`-Aufruf nicht erreicht,
macht das Tor still wirkungslos. Gemessen statt angenommen:

1. In `xtask/src/main.rs`, Zeile 1, kam ein Verweis auf einen Namen, den es nicht gibt:
   ``[`DiesenNamenGibtEsNichtXyz`]``.
2. `make check` brach ab. Der Wortlaut:
   `error: unresolved link to \`DiesenNamenGibtEsNichtXyz\``,
   `note: \`-D rustdoc::broken-intra-doc-links\` implied by \`-D warnings\``,
   `error: could not document \`xtask\``, `make: *** [doc] Error 101`. Exit von `make`: 2.
   `alle fuenf gruen` blieb aus.
3. Der Verweis ist wieder heraus. `git status --porcelain -- crates xtask` gibt nichts aus,
   `git diff --stat -- xtask/src/main.rs` ebenso. Kein Rest im Baum.

Die Meldung nennt `-D rustdoc::broken-intra-doc-links` als den Pruefer, der zuschlug, und
dass `-D warnings` ihn hereingeholt hat. Damit ist belegt, dass die Variable ankommt und
nicht bloss gesetzt wird.

## Pruefung

| Kommando | Exit |
|---|---|
| `make check` (nach der Aenderung, Baum sauber) | 0 |
| `make check` (mit dem gebrochenen Verweis) | 2, Abbruch am Ziel `doc` mit Error 101 |
| `make doc` allein, warmer Baum | 0 |
| `make help` | listet `doc` an alphabetischer Stelle |

## Kosten

| Lauf | Zeit |
|---|---|
| `make check` vorher, kalter Baum | 5:48 |
| `make check` vorher, warmer Baum | 2:03 |
| `make check` nachher, warmer Baum | 1:57 und 2:47 in zwei Laeufen |
| `make doc` allein, warmer Baum | 0,76 s |

Die zwei Nachher-Werte streuen um 50 Sekunden, und das Ziel `doc` erklaert davon 0,76 s. Die
Streuung kommt aus der Maschinenlast und nicht aus der Aenderung; der belastbare Wert ist
`make doc` allein, und er liegt unter einem Prozent der Gesamtzeit. Die Anteile der vier
uebrigen Ziele sind hier nicht einzeln gemessen.

## Kein Defekt aufgefallen

Kein neuer Datensatz unter `shared/issues/`.
