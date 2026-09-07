# Eine Zeichenschwelle statt dreier: Unterbaum und Inhalt greifen ab drei Zeichen

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Auftrag

K18, zwei beantwortete Entscheidungsdatensätze in Code umsetzen:

- `260826-0859_*_die-vorgabe-der-tiefen-suche-hebt-die-schwelle-des-inhaltsfilters-von-drei-auf-fuenf.md`,
  Möglichkeit 2: die Staffelung des Inhaltsfilters (drei Zeichen flach, fünf tief) fällt,
  eine Schwelle für beide Stände, und zwar drei.
- `260826-0923_*_bekommt-der-tiefe-durchlauf-eine-eigene-zeichenschwelle-jetzt-wo-ein-anschlag-ihn-ab-werk-ausloest.md`,
  Möglichkeit 2: der Durchlauf über den Unterbaum bekommt eine eigene Zeichenschwelle, und
  sie ist dieselbe Drei.

Zusammen: **eine Regel statt dreier.** Unter drei Zeichen filtert KRK flach und allein über
die Namen, ab drei Zeichen greifen Unterbaum und Inhalt. Verlangt war, dass die Regel im
Code als **eine** Regel dasteht und nicht als zwei Zahlen, die zufällig gleich sind.

## Erhebung vor der Änderung

Kommando:

```sh
grep -rn 'inhaltsschwelle' crates/ xtask/ --include='*.rs'
grep -rn 'filter_steht' crates/ xtask/ --include='*.rs'
```

`inhaltsschwelle` hatte **einen** rechnenden Rufer, `Ordnermodell::inhalt_wirkt`
(`modell.rs:1204`); die übrigen elf Fundstellen waren Prosa und Proben. Die Zahl im
Datensatz vom 260826 stimmte also noch.

`filter_steht` hatte **fünf** Rufer außerhalb der Proben: `tabs.rs:1098` (die
Startbedingung des Durchlaufs), `appkit/tabelle.rs:3174`, `:3238`, `:3766` und
`appkit/anwendung.rs:5888`. Nur der erste gehörte zu dieser Runde; die vier übrigen fragen
„steht ein Filtertext" für die Anzeige und sind unberührt geblieben.

## Was gebaut wurde

**Die eine Drei steht in `crates/krk-core/src/verzeichnis/filter.rs:257** als
`pub const ZEICHENSCHWELLE: usize = 3;`. Aus der Funktion `inhaltsschwelle(tief) -> usize`
ist damit eine Konstante geworden: eine Funktion, die über den Stand eines Schalters
staffelt, kann die eine Regel nicht ausdrücken, eine Zahl kann es.

Daneben steht eine Zusicherung beim Übersetzen (`filter.rs:268`), dass die Schwelle
mindestens eins ist. Daran hängt eine Bedingung, die der Baum nicht mehr ausschreibt: die
Startbedingung des Durchlaufs fragte bis heute zuerst `filter_steht()`, und diese Hälfte
ist jetzt in `tief_wirkt() || inhalt_wirkt()` enthalten — aber nur, solange die Schwelle
nicht null ist. Eine Schwelle von null ließe den Durchlauf ohne jeden Filtertext beginnen,
und zwar still.

**Geholt wird sie an genau einer Stelle**, `Ordnermodell::schwelle_erreicht`
(`modell.rs`, privat): dort steht die Zählung der Zeichen ohne `*`, und dort steht der
Vergleich. `tief_wirkt` (neu) und `inhalt_wirkt` (umgebaut) sind je eine Zeile darüber und
rechnen nicht nach.

Die zwei Frager haben ihre Frage getauscht:

- `Ordnermodell::zeilengrund_von` fragte `self.tief` und fragt jetzt `self.tief_wirkt()`.
  Unterhalb der Schwelle steht ein Ordner damit wie bei ausgeschaltetem „Deep", und die
  Auftragsliste bleibt leer.
- `Tabliste::durchlauf_nachziehen_an` fragte `filter_steht() && (tief() || inhalt_wirkt())`
  und fragt jetzt `tief_wirkt() || inhalt_wirkt()`.

## Was nicht angefasst wurde

Der Durchlauf selbst hält weiter genau einen Verzeichnisdeskriptor, der Inhaltsfilter legt
weiter genau einen Dateideskriptor dazu, und beides nur während eines Lesens. Die Änderung
liegt vollständig vor dem Lauf und nicht in ihm. `krk-bench/src/bericht.rs` blieb
unberührt (zweite Bahn).

CLAUDE.md wurde auf Weisung nicht angefasst; der Befund liegt als
`260908-0002_o_claude-md-beschreibt-die-gefallene-staffelung-und-den-durchlauf-ab-dem-ersten-anschlag.md`.

## Proben

Neu:

- `die_tiefe_suche_wirkt_ab_drei_zeichen_und_darunter_nicht` (`krk-core/tests/verzeichnis.rs`) —
  der Durchlaufbeginn, gemessen an `tief_wirkt`, an der Ordnerzeile und an der leeren
  Auftragsliste zugleich.
- `beide_schalter_kippen_am_selben_zeichen` (ebenda) — über die Längen null bis fünf, gegen
  `ZEICHENSCHWELLE` statt gegen eine abgeschriebene Drei.
- `die_zeichenschwelle_hat_einen_rufer` (ebenda) — am Quelltext: die Konstante wird
  außerhalb ihrer Heimat an genau einer Stelle gelesen. Das ist die Hälfte, die die Probe
  darüber nicht leisten kann; zwei zufällig gleiche Zahlen bestünden jene ebenso.
- `unter_drei_zeichen_traegt_die_auftragsliste_nichts_und_ab_drei_beides`
  (`krk-ui/src/tabs.rs`) — die Auftragsliste, beide Arten in einem Anschlag.

Umgeschrieben, weil sie die Staffelung festhielten:
`die_zeichenschwelle_steht_bei_drei` (vormals `die_inhaltsschwelle_steht_bei_drei_und_bei_fuenf`),
`das_umlegen_der_tiefen_suche_verschiebt_die_schwelle_nicht` (vormals
`die_tiefe_suche_hebt_die_schwelle_auf_fuenf_zeichen`, hält jetzt das Gegenteil),
`die_tiefe_suche_ist_die_vorbelegung`, `ein_befund_gilt_nur_zu_seiner_frage`,
`ein_eingefuegter_name_von_fuenf_zeichen_stoesst_den_inhaltsfilter_sofort_an`,
`das_sternchen_zaehlt_nicht_zur_schwelle`.

**Die Zählprobe `die_zeichenregel_hat_drei_rufer_und_der_vergleich_drei` ist nicht
berührt**: sie zählt die Rufer der Zeichenregel und des Vergleichs, und beide haben durch
diese Runde keinen Rufer gewonnen oder verloren. Geprüft, nicht angenommen: sie läuft grün,
und ihre erwarteten Listen stehen unverändert.

## Prosa nachgezogen

`filter.rs` (Modulkopf samt Skizze, neuer Abschnitt zum gewollten Sprung), `modell.rs`
(Modulkopf, Skizze des Prüfschritts, Felddoku beider Schalter, `Ordnermodell::neu`,
`tief_setzen`, `tief`), `durchlauf.rs` (Modulkopf: keine Schranke, aber seit heute eine
Vorbedingung beim Rufer), `tabs.rs` (`durchlauf_nachziehen_an`), `tasten/belegung.rs`
(beide Schalterbefehle), `HowTo.md` (die Nutzerprosa zu den zwei Ankreuzfeldern).

## Abnahme

```
cargo build --workspace                          — exit 0
cargo test --workspace                           — exit 0 (25 Ziele grün)
cargo clippy --workspace --all-targets -- -D warnings — exit 0
cargo fmt --all --check                          — exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps — exit 0
```

**Ungeprüft bleibt ohne laufendes Bündel das Tippgefühl.** Dass der Sprung am dritten
Zeichen erträglich ist und nicht als Störung gelesen wird, sagt keine Probe; sie messen die
Zeilen und die Aufträge, nicht den Eindruck. Ebenso ungemessen bleibt, was der Verzicht auf
die Läufe bei einem und zwei Zeichen in einem gewachsenen Heimatordner tatsächlich spart —
die Messung war der Gegenstand der zurückgewiesenen Möglichkeit 3 beider Datensätze und ist
jetzt spätere Bestätigung statt Vorbedingung.
