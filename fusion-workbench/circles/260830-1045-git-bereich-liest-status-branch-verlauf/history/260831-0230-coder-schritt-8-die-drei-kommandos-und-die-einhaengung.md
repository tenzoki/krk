# Coder-Sitzung: Schritt 8 der Runde 23, die drei Kommandos und die Einhängung

**Date:** 2026-08-31
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Circle:** `circles/260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 8
**HEAD:** `7264daf` (nicht committet; der Orchestrator committet)

## Die drei Kommandos und ihre drei Pflichtstellen

`GitBereichUmschalten`, `FokusGit` und `SpalteMarkeUmschalten` stehen als Block am Ende
von `Kommando` (`crates/krk-core/src/tasten/belegung.rs`), wie die Runde 20 ihre drei
Zoombefehle dort angehängt hat. Je Kommando drei Stellen:

| Stelle | wer sie hält |
|---|---|
| `Kommando::wirkungsbereich` — alle drei `Ueberall` | der Übersetzer |
| `belegungsmodell::bereich_des_kommandos` | der Übersetzer |
| `Kommando::KENNUNGEN`, jetzt 85 statt 82 | die Probe `jede_variante_von_kommando_steht_genau_einmal_in_kennungen` |

Die zwei Umschalter stehen im ersten `Ueberall`-Zweig bei den Bereichs- und
Spaltenschaltern, `FokusGit` im zweiten bei den vier vorhandenen Fokusbefehlen; jeder
bekommt seinen Kommentar mit dem Satz, aus dem er dort steht. Kein Auffangzweig ist
entstanden.

**Die zwei Prosastellen aus Entscheidung 2 sind mitgezogen**, und sie standen noch offen:
der Doc-Kommentar von `Wirkungsbereich::Navigator` zählt jetzt den Git-Bereich mit und
sagt, aus welcher Regel, und `Wirkungsbereich::beschriftung` liefert für `Navigator`
„Dateifenster, Leiste, Vorschau und Git-Bereich". Der zweite Text ist **Nutzerausgabe**:
er steht in der dritten Spalte jeder Zeile von `make tasten` und in
`docs/tastenbelegung.md`. Das Feld `ACHT_BESCHRIFTUNGEN` in
`crates/krk-core/tests/belegung.rs` und der Doc-Kommentar von `beschriftung` selbst sind
nachgezogen; `keine_zwei_wirkungsbereiche_teilen_sich_eine_beschriftung` bleibt grün.

## Der zehnte Funktionsbereich

`Funktionsbereich::Git` steht unmittelbar hinter `Editor`, `ALLE` auf zehn, `name` „Git".
`GitBereichUmschalten` und `FokusGit` ordnen sich ihm zu, `SpalteMarkeUmschalten` dem
`Dateilisting` bei den drei anderen Spaltenschaltern — mit dem Satz am Zweig, warum: die
Gliederung fragt nach der Gegend der Anwendung, und geschaltet wird eine Spalte der
Dateiliste. Dass die Frage nach einem eigenen Funktionsbereich offen ist, steht am
Varianten-Kommentar mit dem Datensatz.

## Die vier Spaltenschalter

`kommando_der_spalte(Spalte::Marke)` liefert `Some(Kommando::SpalteMarkeUmschalten)`.
Vier Stellen sind zusammen von drei auf vier gezogen: die Feldbreite
`spaltenschalter: [Retained<NSButton>; 4]`, das `Vec::with_capacity(4)`, der Text des
`expect` und die Zählprobe, die jetzt `genau_vier_spalten_sind_schaltbar` heißt.

**Der Doc-Kommentar von Schritt 2 stimmt nach der Änderung weiter**, und ich habe es
gelesen und nicht angenommen: das Feld entsteht aus einer gefilterten Liste mit
`try_into`, die Zahl ist damit eine Zusicherung zur **Laufzeit** und keine Bedingung des
Baus. Eine fünfte schaltbare Spalte bei stehender `4` übersetzt anstandslos und bricht
erst im `expect` beim Start ab; was den Bau hält, ist die Zählprobe. Der Kommentar sagt
das jetzt für die Zahl vier statt für die Zahl drei.

**`kommando_des_bereichs` ist wieder total.** Sie lieferte seit Schritt 1 ein
`Option<Kommando>`, weil der Git-Bereich noch keines trug; sie liefert wieder ein
`Kommando`, und der Klickzweig nimmt `map` statt `and_then`. Der Zwischenstand, den ihr
Doc-Kommentar ausdrücklich als solchen führte, ist damit weg.

## Der Nutzerentscheid von 260831-0120, und was er an diesem Schritt geändert hat

Die Auswahl der Verlaufsliste zieht aus der Ansicht in das `Gitmodell` (Möglichkeit 2).
Gebaut ist das so:

- **`appkit/git.rs` hält keine Auswahl mehr.** `GitfensterIvars::auswahl` ist gefallen.
  Was auf dem Schirm blau steht, fragt `angezeigte_auswahl()` an der `NSTableView`
  (`selectedRow`, `-1` wird über `usize::try_from` zu `None`); wo der Stand **wohnt**, ist
  das Gitmodell. Eine zwischengehaltene Kopie wäre die zweite Heimat gewesen, die der
  Datensatz ausschließt.
- **Ein zweiter Melder, `Auswahlmelder = Box<dyn Fn(Option<usize>)>`.** Beide Wege des
  Nutzers — der Pfeil über `kommando_ausfuehren`, der Mausklick über
  `tableViewSelectionDidChange:` — münden in `auswahl_uebernehmen`: anzeigen, Einzelheiten
  schreiben, melden, in dieser Reihenfolge. Der Rückruf hält den Delegierten schwach wie
  die sieben anderen Melder.
- **Der Anwendungsdelegierte schreibt.** `gitauswahl_setzen` geht über
  `DateifensterQuelle` → `Tabliste` → `Tabinhalt`, drei dünne Schichten wie bei
  `git_gefragt_setzen`. **Die Ausnahme steht dort, wo die Zusage steht**: im
  Doc-Kommentar von `Tabinhalt::gitmodell` (`crates/krk-ui/src/tabs.rs`), der jetzt „Nur
  zu lesen, mit einer benannten Ausnahme" sagt, die Ausnahme benennt und dazuschreibt,
  warum sie kein zweiter Schreiber auf einem Feld ist, sondern ein zweites Feld mit einem
  eigenen Schreiber. Genau daran ist Möglichkeit 3 gescheitert, und der Kommentar sagt es.
- **`zeigen` bekommt das Modell weiterhin lesend.** Möglichkeit 3 ist verworfen.
- **Die Reihenfolge trägt.** Der Melder feuert **innerhalb** von
  `Gitfenster::kommando_ausfuehren`, also innerhalb von `bereichskommando`, also bevor
  `Anwendungsdelegierter::kommando_ausfuehren` sein `gewirkt` auswertet und
  `aufteilung_nachziehen` fährt. Das Modell trägt den neuen Stand, bevor `zeigen` ihn
  wieder liest; ohne diese Reihenfolge schriebe der Nachzug die Bewegung des Nutzers
  zurück.

**Was gefallen ist, und warum.** `haelt_die_auswahl` (`appkit/git.rs`) und seine drei
Proben: die Regel, die sie hielten, wohnt jetzt im Modell — `zuruecksetzen` nimmt die
Auswahl mit (C4.6), `verlauf_anhaengen` lässt sie stehen (C4.2). C4.2 hat dafür eine neue
Probe im `gitmodell.rs` bekommen, C4.6 hatte dort schon eine. Eine Ansichtsprobe ist
geblieben, umbenannt zu `ein_anderer_ordner_traegt_andere_zeilen`: sie hält die Aussage,
an der `zeigen` hängt, nämlich dass es den Verlauf des Modells übernimmt und nicht den
vorigen fortschreibt, samt der Zusicherung gleicher Länge, ohne die sie nur eine
Längenprüfung bewiese.

**`Gitmodell::ausgewaehlter_commit` ist gefallen.** Es hätte nach diesem Schritt keinen
Rufer: `zeigen` liest `auswahl()` und nimmt den Text aus seinen eigenen Zeilen, die aus
`einzelheiten(zeile)` kommen. Ein öffentlicher Ableser ohne Ableser ist tote Fläche, und
eine frische `expect(dead_code)` dafür wäre die Ausnahme ohne Ablaufdatum. Der Datensatz
sprach von „den drei Lesern"; gebunden hat er die **Heimat der Auswahl**, nicht den
Fortbestand eines Zugriffs, und der Plan trägt den Wegfall jetzt in seinen
`## Data Structures` und `## API Changes`.

## Die zwei Nachzüge beim Anwendungsdelegierten

**`gitbedarf_nachziehen`** rechnet `sichtbar(Bereich::Git) || spalte_sichtbar_in(…,
Spalte::Marke)` und gibt den Wahrheitswert an **beide** `DateifensterQuelle` — beide, weil
die Markenspalte in beiden Listen steht und ein späterer Fensterwechsel sonst ein
Dateifenster vorfände, das nie gefragt hat. Gerufen aus `aufteilung_nachziehen` und aus
`spaltenanzeige_nachziehen`, **aus keinem dritten**. `Tabliste::git_gefragt_setzen` ist ein
Wechsel und kein Wiederholen, also ist der zweite Ruf beim Aufbau ein Leerlauf.

**`gitanzeige_nachziehen`** ruft `Gitfenster::zeigen` mit dem Gitmodell des sichtbaren Tabs
im **aktiven** Dateifenster (E1).

## Eine Stelle, an der der Bau über den Plantext hinausgeht

**`gitanzeige_nachziehen` hat drei Anlässe und nicht zwei, und der dritte brauchte einen
Melder.** Der Plan sagt „mit denselben Anlässen wie `bereichsleiste_nachziehen`"; das sind
`aufteilung_nachziehen` und der Ordnerwechsel-Rückruf, und beide hängen an einer
**Handlung** des Nutzers. Kopf, Verlauf und Zusammenfassung treffen aber ein, während er
nichts tut — das ist der Regelfall, nicht die Ausnahme. Ohne dritten Anlass stünde der
Branchname erst da, sobald der nächste Befehl fällt.

Der Weg war schon vorgezeichnet und nur nicht gebaut: `Einzug::gitkopf_neu` (Schritt 6)
trägt seit seiner Entstehung den Satz, sein Ableser sei `gitanzeige_nachziehen` aus
Schritt 8. Gebaut ist er als Melder `gitwechsel` auf `DateifensterQuelle`, im Zuschnitt
von `meldungswechsel` daneben: der Einzugstakt feuert ihn auf `einzug.gitkopf_neu`, der
Rückruf trägt die Seite nicht mit, und der Empfänger fragt das aktive Dateifenster selbst.
Der Doc-Kommentar von `gitkopf_neu` sagt jetzt, dass die Tabelle das Feld weiterreicht
statt darauf zu zeichnen, und der `#[must_use]`-Kopf von `Einzug` nennt es weiter als
seine eine Ausnahme, mit dem berichtigten Grund.

**Die Ausleihe während `zeigen` ist bedacht.** `mit_gitmodell` reicht einen Rückruf herein,
statt eine Ausleihe herauszugeben, und hält `tabs.borrow()`, während `zeigen` läuft. Ein
`reloadData` oder ein Setzen der Auswahl löst `tableViewSelectionDidChange:` aus, und ohne
Sperre liefe die Meldung über den Auswahlmelder in ein `tabs.borrow_mut()` — der doppelte
Zugriff, also ein Absturz. `zeigen` hält deshalb `setzt_selbst` über seinen **ganzen**
Rumpf, und `auswahl_anzeigen` setzt das Kennzeichen am Ende auf den Stand vor dem Aufruf
zurück statt blind auf `false`. Beide Stellen tragen den Grund.

## Die fünf entfernten Ablaufmarken

| Stelle | Rufer, der sie fallen lässt |
|---|---|
| `appkit/git.rs`, `#![expect(dead_code)]` am Modulkopf | `Gitfenster::bauen` in `oberflaeche_aufbauen` |
| `gitmodell.rs:135`, die Leseseite | der Auswahlmelder über `gitauswahl_setzen`; `ausgewaehlter_commit` ist stattdessen gefallen |
| `tabs.rs`, `Tabinhalt::gitmodell` | `gitanzeige_nachziehen` über `mit_gitmodell` |
| `tabs.rs`, `Tabliste::git_gefragt_setzen` | `gitbedarf_nachziehen` über `gitbedarf_setzen` |
| `tabs.rs`, `Tabliste::verlauf_nachladen` | der Nachlademelder über `DateifensterQuelle::verlauf_nachladen` |

Der Übersetzer hat jede der fünf als unerfüllte Erwartung gemeldet, wie ihre Kommentare es
zusagten. Es steht keine neue Ausnahme im Baum.

## `immer_erreichbar` wächst nicht

`waehrend_eines_blattes_kommen_genau_diese_vier_durch` bleibt bei vier und hat eine Zeile
bekommen, die `GitBereichUmschalten` und `FokusGit` ausdrücklich als abgewiesen prüft
(C2.11) — mit dem Satz, warum: ein Bereich, der sich hinter einer stehenden Rückfrage
ein- und ausblenden ließe, wäre die Ausnahme ohne Grund.

## Die Aufrufzählung des Menüs

`der_delegierte_wird_an_genau_drei_stellen_um_einen_befehl_gebeten` hat zwei Zahlen. Die
erste, die drei Aufrufer am Delegierten selbst, ist unberührt: mein Zweig heißt
`self.git().kommando_ausfuehren(…)` und trägt den Empfänger `self.git()`. Die zweite, die
Gesamtzahl im Baum, steigt von acht auf neun — die sechste Weiterreichung, an den
Git-Bereich. Doc-Kommentar und Meldung sind mitgezogen.

## Die zwei Listen der Funktionen ohne Kombination

`spalte_marke_umschalten` wird ab Werk ohne Kombination ausgeliefert, wie die drei
Spaltenschalter vor ihm (Nutzerantwort vom 260812-0306). Es steht deshalb jetzt in beiden
Listen, die diese Ausnahme führen: `OHNE_KOMBINATION_AB_WERK`
(`crates/krk-core/tests/belegung.rs`, jetzt sechs) und dem Literal in
`belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`.
**Beide sind Rust und damit dieser Schritt**, obwohl die Sache zu Schritt 9 gehört; der
Plan hat sie keinem Schritt zugewiesen, und ohne sie wäre Schritt 9 nicht grün zu kriegen.
Dass die beiden Listen eine werden sollten, ist die offene Frage
`260814-2326_*_wird-die-liste-der-funktionen-ohne-kombination-an-einer-stelle-gefuehrt.md`;
dieser Schritt macht sie nicht auf und nicht zu.

## `#[must_use]`

An `Gitfenster::angezeigte_auswahl` (eine Auskunft ohne Nebenwirkung, deren stilles
Fallenlassen unbemerkt bliebe). Die übrigen neuen Rückgaben sind `()`:
`auswahlmelder_setzen`, `auswahl_uebernehmen`, `gitbedarf_setzen`, `verlauf_nachladen`,
`gitauswahl_setzen`, `gitwechsel_setzen`, `gitbedarf_nachziehen`, `gitanzeige_nachziehen`.
`mit_gitmodell` reicht durch, was sein Rückruf liefert, und darf deshalb keines tragen —
der Rückruf entscheidet, ob es etwas zu verbrauchen gibt.

`Tabliste::git_gefragt_setzen` und `verlauf_nachladen` tragen ihres weiterhin; ihre neuen
Hüllen auf `DateifensterQuelle` verbrauchen den Wert und werfen den Einzugstakt an,
genau wie `durchlauf_nachziehen` es daneben tut.

## Der Untergrenzen-Abschnitt

Zwei Dateien unter `appkit/` sind angefasst, und beide sprechen **keine neue Klasse und
keine neue Methode** an. `anwendung.rs` baut jetzt kein eigenes `NSView` mehr; die drei
Strukturen `NSRect`, `NSPoint` und `NSSize` sind aus den Importen gefallen, und der Absatz
im Modulkopf sagt jetzt, dass die Klassen des Git-Bereichs im Kopf von `appkit/git.rs`
stehen und hier nicht ein zweites Mal aufgezählt werden. `git.rs` und `bereichsleiste.rs`
tragen ihre Abschnitte unverändert richtig. Die Deckung des Verzeichnisses ist
unverändert: außer `koordinaten.rs` und `mod.rs` trägt ihn jede Datei.

## Abnahme

`make check` — **exit 2**, und die Wurzel ist `cargo test --workspace` mit 101. Rot sind
**sieben** Proben, alle aus **einer** Ursache: die Auslieferungsbelegung kennt die drei
Kennungen erst nach Schritt 9.

| Probe | Kiste |
|---|---|
| `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` | `krk-core` lib |
| `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` | `krk-core` `tests/belegung.rs` |
| `belegungsausgabe::tests::die_abschnitte_stehen_in_der_reihenfolge_der_funktionsbereiche` | `krk-ui` |
| `belegungsausgabe::tests::die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander` | `krk-ui` |
| `belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` | `krk-ui` |
| `belegungsmodell::tests::die_zeilen_sind_nach_bereichen_gegliedert` | `krk-ui` |
| `menuemodell::tests::die_obermenues_folgen_der_gliederung` | `krk-ui` |

**Der Plan nennt eine davon, ich habe sieben, und das ist gemessen und nicht behauptet.**
Ich habe die drei `[[funktion]]`-Blöcke aus Schritt 9 versuchsweise in
`resources/default-keymap.toml` eingetragen, `cargo test --workspace --no-fail-fast`
gefahren und die Datei danach aus einer Kopie im Scratchpad zurückgestellt; die Prüfsumme
ist dieselbe wie vorher (`2496af52…`), und `git status resources/` meldet nichts. Alle
sieben werden dabei grün. Rot bleibt in diesem Versuch genau eine weitere, und sie gehört
Schritt 9: `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` — der
Kommentarkopf jener Datei nennt „88 Funktionen mit zusammen 93 Kombinationen", und die
Zeile ist mitzuziehen. Sie ist als Defekt für den `ontocoder` gefilt
(`issues/260831-0230_*_die-drei-neuen-eintraege-ziehen-die-zwei-zaehlstaende-im-kopf-der-auslieferungsbelegung-nach.md`).

`cargo build --workspace`, `cargo clippy --workspace --all-targets` und
`cargo fmt --all --check` sind grün. 880 Proben im Binärziel von `krk-ui` bestehen, 233 in
der Bibliothek von `krk-core`.

Kein `git stash`, kein `git checkout .`, kein `git reset --hard`, kein `git clean`, kein
`git restore .`. Nicht committet.
