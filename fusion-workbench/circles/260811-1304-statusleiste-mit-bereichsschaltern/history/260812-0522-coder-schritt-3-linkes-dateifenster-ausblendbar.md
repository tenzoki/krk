# Coder, Schritt 3: Das linke Dateifenster wird ausblendbar

**Datum:** 260812-0522
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`, Implementierungsschritt 3
**Abnahme:** `make check` — Exit 0

## Auftrag

Schritt 3 des Plans und nur dieser: die Regel über die beiden Dateifenster von „das linke ist
besonders" auf „eines bleibt" umstellen. Die bindende Antwort dahinter ist
`decisions/260811-1305_a_traegt-das-linke-dateifenster-einen-schalter.md`, Möglichkeit 3, mit
ihrem benannten Preis: ein fünftes Feld in `Sichtbarkeit` und eine dritte hergestellte
Zusicherung beim Laden. Nicht committen; der Orchestrator trägt ein.

## Was entstanden ist

**`crates/krk-core/src/ablage/sitzung.rs`**

- `Sichtbarkeit` trägt `erstes_dateifenster: bool` als erstes Feld, in `Default` auf `true`, wie
  der Plan es wörtlich verlangt. Eine ältere `session.toml` bleibt lesbar, weil die Struktur
  `#[serde(default)]` trägt; das fehlende Feld heißt „sichtbar".
- **Der Kommentar an der Struktur ist neu geschrieben.** Er begründete bis heute die fehlende
  Angabe („ein Feld, das nie `false` werden darf, wäre eine Zusage, die niemand einhält") und
  sagte nach dieser Änderung das Gegenteil des Codes. Er sagt jetzt zweierlei: dass alle fünf
  Bereiche ein Feld tragen, und dass die Regel „eines bleibt" **nicht hier** steht, sondern im
  Fenstermodell von `krk-ui`, das sie an zwei Stellen einlöst. `krk-core` weiß von der Regel
  nichts und trägt nur die Angabe.

**`crates/krk-ui/src/fenstermodell.rs`**

- `sichtbar_in` liest das neue Feld statt `true` zu liefern, `sichtbar_setzen` schreibt es statt
  den leeren Zweig zu nehmen.
- `umschalten`: **beide Dateifenster gehen durch denselben Zweig.** Die Fallunterscheidung läuft
  jetzt über `bereich.seite()` statt über genannte Bereiche; abgewiesen wird der Befehl, der das
  letzte sichtbare Dateifenster ausblenden würde, gleich welches der beiden es ist. War das
  ausgeblendete das aktive, wandert die Aktivität auf das andere — das stand bis heute nur für
  rechts da, mit `Fensterseite::Links` als fest eingetragenem Ziel.
- Der Auffangzweig dort steht über `Option<Fensterseite>` und nicht über `Bereich`; die
  vollständige Fallunterscheidung über die fünf Bereiche bleibt `Bereich::seite`, und ein
  sechster Bereich hält weiterhin dort den Bau an. Der Kommentar im Zweig sagt es.
- `aus_sitzung` stellt eine **dritte** Zusicherung her: sind beide Dateifenster ausgeblendet,
  wird das linke sichtbar gesetzt. **Die Reihenfolge zählt und steht im Kommentar**: erst die
  Dateifenster, dann das aktive. Die zweite Zusicherung schickt die Aktivität mit
  `self.aktiv.andere()` auf das andere Dateifenster und braucht dafür eines, das steht; das feste
  `Fensterseite::Links` von vorher war nur richtig, solange das linke unantastbar war.

**Der Modulkopf, Abschnitt „Was das linke Dateifenster von den anderen unterscheidet"**, ist durch
„Eines bleibt: die Regel über die beiden Dateifenster" ersetzt. Er nennt die Regel, die eine
Stelle, die sagt welcher Bereich ein Dateifenster ist (`Bereich::seite`), die beiden Wege zu ihr
(Laufzeit und Start) und den Nutzerentscheid, der die alte Festlegung aufgehoben hat.

**`crates/krk-ui/src/appkit/aufteilung.rs`**

- `sichtbar_im` liest das neue Feld, `gemessene_sichtbarkeit` trägt es über
  `steht_im(teiler, Bereich::Links)`. Damit stehen beide vollständigen Fallunterscheidungen dieses
  Moduls über `Bereich` wieder auf demselben Stand wie die im Modell.

## Zwei Kommentare mehr, als der Auftrag nannte

Der Auftrag nannte zwei Kommentare, die nach der Änderung das Gegenteil des Codes sagen. Es sind
vier: dieselbe Begründung stand noch an zwei weiteren Stellen, beide außerhalb der vier Dateien
des Plans.

- `crates/krk-ui/src/kommandos/fokus.rs`, Dokumentationskommentar an `holt_hervor`: „das linke
  lässt C7 gar nicht ausblenden".
- `crates/krk-ui/src/appkit/anwendung.rs`, Dokumentationskommentar an `fokus_holen`: derselbe
  Satz.

**Ihre Aussage hält, ihre Begründung nicht.** Beide sagen, das aktive Dateifenster sei nie
ausgeblendet, und das stimmt weiter — aber aus dem neuen Grund: eines der beiden bleibt stehen,
und wird das aktive ausgeblendet, wandert die Aktivität auf das andere. Genau so stehen sie jetzt
da. Keine Zeile Code ist dabei angefasst; es ist derselbe Fall, in dem Schritt 1 den Kommentar an
`MINDESTGROESSE` in `appkit/fenster.rs` mitgezogen hat.

## Prüfungen

**`crates/krk-ui/src/fenstermodell.rs`** — 35 Proben statt 34: zwei neue, eine ersatzlos weg,
zwei umbenannt.

| vorher | nachher |
|---|---|
| `das_zweite_dateifenster_geht_aus_und_wieder_ein` | `jedes_dateifenster_geht_aus_und_wieder_ein` (läuft über beide) |
| `das_ausblenden_holt_die_aktivitaet_zurueck_nach_links` | `das_ausblenden_gibt_die_aktivitaet_an_das_andere_dateifenster` (beide Richtungen) |
| `das_letzte_dateifenster_ist_immer_schon_eingeblendet` | ersatzlos; an seiner Stelle steht `keine_folge_von_befehlen_blendet_beide_dateifenster_aus` |

Der dritte Name trug die alte Zusage „das linke steht immer" und ist nicht nachzuziehen, sondern
zu ersetzen: die neue Zusage ist eine andere. Die neue Probe fährt alle acht Folgen aus drei
Umschaltbefehlen über die beiden Dateifensterbereiche und prüft nach **jedem** Befehl beides —
dass eines steht und dass das aktive sichtbar ist.

- `das_letzte_dateifenster_laesst_sich_nicht_ausblenden` prüft jetzt beide Richtungen: erst geht
  das eine aus, dann wird der Befehl auf das andere verworfen. Bis heute war das eine
  Einzelaussage über `Bereich::Links`.
- `eine_sitzung_ohne_sichtbares_dateifenster_holt_das_linke_hervor` ist neu und misst die dritte
  Zusicherung, über **beide** Werte von `aktiv`: daran hängt die Reihenfolge.
- `ein_ausgeblendetes_dateifenster_kommt_nicht_als_aktives_aus_der_sitzung` hat die
  Gegenrichtung dazubekommen (`erstes_dateifenster = false` bei `aktiv = "links"`).
- `das_einblenden_holt_hervor_und_blendet_nie_aus` läuft jetzt auch über `Bereich::Links`.

**`crates/krk-core/tests/ablage.rs`** — zwei neue Proben, zwei bestehende erweitert.

- `beispielsitzung` trägt `erstes_dateifenster: false`; damit läuft das neue Feld durch den
  Rundlauf aller vier Dateien und durch den Feldvergleich in
  `das_fenster_und_tabmodell_ueberlebt_schreiben_und_wiedereinlesen`.
- `das_ausgeblendete_erste_dateifenster_ueberlebt_den_rundlauf_byteweise` — zwei Schreibvorgänge
  statt eines Strukturvergleichs, und die Zeile steht nachweislich in der Datei, die der Nutzer
  nach C7 von Hand liest.
- `eine_sitzung_ohne_das_erste_dateifenster_bleibt_lesbar` — eine `session.toml` aus der Zeit vor
  dieser Runde, ohne die Zeile; sie gilt nicht als beschädigt, und das fehlende Feld heißt
  „sichtbar".
- `der_auslieferungszustand_der_sitzung_erfuellt_c1` und
  `eine_sitzung_ohne_die_editorfelder_bleibt_lesbar` prüfen das neue Feld mit.

**Gegengeprobt.** Mit einer Wegwerfänderung, die beide neuen Zusicherungen entfernt — die
Abweisung in `umschalten` und die dritte Zusicherung in `aus_sitzung` —, fallen genau drei
Proben: `das_letzte_dateifenster_laesst_sich_nicht_ausblenden`,
`keine_folge_von_befehlen_blendet_beide_dateifenster_aus` und
`eine_sitzung_ohne_sichtbares_dateifenster_holt_das_linke_hervor`. Die Wegwerfänderung ist
zurückgenommen; `make check` läuft danach wieder grün.

## Was dieser Schritt nicht tut

- **Kein Weg führt heute zu der neuen Fähigkeit.** Weder ein Tastenbefehl noch ein Schalter
  blendet das linke Dateifenster aus; das kommt mit Schritt 5 und Schritt 8, wie der Plan es unter
  „Abnahme" für diesen Schritt sagt. Am laufenden Bündel ist die Änderung damit unsichtbar.
- **Der Datensatz `260811-1305` behält seinen Marker `_a_`.** Umgesetzt ist sein Preis, nicht sein
  Gegenstand: die fünf Schalter der Bereichsleiste entstehen erst in Schritt 8. Die Umbenennung
  auf `_i_` gehört dorthin, und sie verlangt einen Commit-Hash, den dieser Schritt nicht hat.

## Eine Beobachtung, nicht behoben

Das neue Feld steht als **erstes** in `Sichtbarkeit`, weil Plan und Auftrag es wörtlich so
verlangen. Die Schwesterstruktur `Breiten` führt ihre fünf Felder dagegen in der Reihenfolge der
Fensterzeile (Lesezeichen, links, rechts, Vorschau, Editor), und `Sichtbarkeit` tat es bisher
auch. In der geschriebenen `session.toml` steht `erstes_dateifenster` deshalb jetzt **über**
`lesezeichen`, also links vor der Leiste, die weiter links sitzt. Das ist kein Defekt und bricht
nichts; es fällt allein dem auf, der die Datei nach C7 von Hand liest. Kein eigener Datensatz —
die Zeile hier genügt, und die Reihenfolge zu drehen ist ein Einzeiler, falls sie stören sollte.

## Am Plan nachgezogen

Schritt 3 steht auf `[DONE]`.

## Abnahme

`make check` (`cargo build`, `cargo test`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, alle über den Workspace) — **Exit 0**,
„alle vier grün". Kein Vordergrund nötig, wie der Plan für diesen Schritt zusagt.

Nicht committet: der Orchestrator trägt ein.
