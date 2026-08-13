# ontocoder: S15, die Kombination in der Auslieferungsbelegung

**Datum:** 2026-08-13
**Status:** Complete
**Circle:** `260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz`
**Plan:** `planning/260813-0205_o_plan-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md`, Schritt S15 samt Nachtrag vom 260813-0233
**Geändert:** `resources/default-keymap.toml` (die einzige Datei; kein Rust angefasst)

---

## Was eingetragen ist

Ein `[[funktion]]`-Block mit `id = "weitere_instanz"`, `name = "Weitere Instanz starten"` und `tasten = ["opt+cmd+n"]`, dazu ein eigener Abschnittskopf und die berichtigte Zählzeile im Dateikopf. Damit ist der Baum wieder grün: die drei Proben, die S14 planmäßig rot zurückgelassen hat, laufen durch.

**Die zwei Zahlen im Kopf sind nachgezählt und nicht fortgeschrieben.** Vor der Änderung trug die Datei 81 `[[funktion]]`-Blöcke mit zusammen 87 Einträgen über alle `tasten`-Listen, danach 82 mit 88. Gezählt ist mit demselben Maß, das `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` anlegt: Zahl der Blöcke und Summe der Listenlängen. Die Zeile lautet jetzt `# Ausgeliefert sind 82 Funktionen mit zusammen 88 Kombinationen.`

**`opt+cmd+n` ist selbst nachgeprüft und nicht aus dem Plan übernommen.** Über alle 81 Tastenlisten der Datei ausgezählt: auf dem `n` liegen `cmd+n` (`fenster_einblenden`), `shift+cmd+n` (`ordner_anlegen`) und `ctrl+cmd+n` (`datei_anlegen`); `opt+cmd+n` kommt nirgends vor. Die einzige doppelt vergebene Kombination bleibt `cmd+a`, und die ist als Fall zweier Zusteller in der Datei beschrieben. Die Schreibweise folgt der festgelegten Reihenfolge `[ctrl+][opt+][shift+][cmd+]<taste>`.

**`reserviert_fuer` steht nicht dabei.** Das Feld heißt „benannt und ab Werk unbelegt"; die Funktion trägt seit `3caa2b7` ein Kommando und eine Taste.

## Der Ort in der Gliederung, und warum der Plan ihn nicht genau trifft

S15 schreibt: „eingeordnet im Abschnitt zu C3, in dem `belegung_ansehen` und `beenden` stehen". **Einen solchen Abschnitt gibt es in der Datei nicht.** Sie führt zwei getrennte C3-Abschnitte, `# ── C3: die Belegungsansicht ──` und, ganz am Schluss, `# ── C3: das Beenden der Anwendung ──`; dazwischen liegen die Textbefehle des Menüs „Bearbeiten". S4 hat daran nichts geändert: der Schritt hat die Gliederung in `crates/krk-ui/src/belegungsmodell.rs` umsortiert und `Funktionsbereich::Textbefehle` in der Anzeige zu „Bearbeiten" gemacht, `resources/default-keymap.toml` aber nicht angefasst. Der letzte Commit auf der Datei ist `95b2dfa` aus der Runde 6.

Gewählt ist ein eigener Abschnitt unmittelbar **vor** dem Beenden der Anwendung, also nicht am Dateiende. Zwei Gründe, beide am Baum gelesen:

- Die Sachgruppe ist die Anwendung als ganze. `bereich_des_kommandos` (`crates/krk-ui/src/belegungsmodell.rs:327`) führt `WeitereInstanz` zusammen mit `BelegungAnsehen` und `Beenden` unter `Funktionsbereich::Anwendung` und begründet in den drei Zeilen darüber, warum der Befehl nicht zu `Fenster` gehört.
- Von den beiden Nachbarn ist das Beenden der nähere: Starten und Beenden sind das Paar am Leben des Prozesses, die Belegungsansicht ist eine Ansicht. Die zwei Abschnitte zur Anwendung stehen damit beieinander, und der Eintrag `beenden` bleibt der letzte der Datei, wie sein eigener Kommentar es beschreibt.

Der Abschnittskopf trägt keine C-Nummer. „C3" meint in dieser Datei durchgehend den Spec der Runde 1; die Fähigkeit hinter diesem Eintrag ist C3 der Runde 7, und die Prosa darunter sagt es. Denselben Weg gehen die zwei anderen Abschnitte späterer Runden, „Der eingebaute Editor" und „Pfade kopieren und mit dem Standardprogramm öffnen".

## Der Kommentar

Er begründet die Kombination aus der Reihenordnung der Datei: das `n` trägt hier das Neue, `shift+cmd+n` den Ordner, `ctrl+cmd+n` die leere Datei, und das nackte `cmd+n` hält bei „fenster_einblenden" den Platz des Mac-üblichen „Neu"; `opt+cmd+n` ist die vierte Form dieser Reihe und die einzige freie. Die Zählung steht mit Datum dabei.

Der zweite Absatz sagt, warum `cmd+n` bei „Fenster einblenden" bleibt, und zitiert den Datensatz, der die Umbenennungszusage trägt: `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2007_*_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`, Nutzerantwort vom 260804-0830. Dort steht der Satz wörtlich: „Die Runde, die mehrere Fenster einführt, benennt ihn in ‚Neues Fenster' um … und behält das Kürzel." Der Datensatz ist gelesen und nicht aus dem Gedächtnis zitiert.

**Für `opt+cmd+n` selbst gibt es keinen Entscheidungsdatensatz, und der Kommentar behauptet auch keinen.** Der Spec leitet die Kombination ab (`shared/planning/260813-0053_o_spec-…`, C3.2 und die Zeile „Am 260813 als einzige naheliegende freie Kombination … abgelesen"); eine Nutzerantwort dazu gibt es nicht.

## Abnahme

Alle vier Kommandos mit `export PATH="$HOME/.cargo/bin:$PATH"` davor:

| Kommando | Exit |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo test --workspace` | 0 |

`cargo test --workspace` meldet 19 Ziele, 1000 Proben, 0 Fehlschläge. Die drei bis eben roten Proben sind namentlich nachgesehen und grün: `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`, `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` und `belegungsausgabe::tests::die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`. Dazu die zwei Proben, die S15 ausdrücklich nennt: `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` und `jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste`.

Kein Vordergrundlauf, kein Bündelbau. `target/KRK.app` ist nicht angefasst.

## Nebenwirkungen und Randnotizen

- **`CLAUDE.md` nennt weiter 68 Varianten für `Kommando`; der Baum trägt 76.** Das Nachziehen gehört nach dem Plan an den Schluss der Runde und nicht in einen ihrer Schritte; der Datensatz dazu ist `shared/issues/260812-2253_o_claude-md-nennt-fuer-kommando-68-varianten-der-baum-traegt-75.md` und trägt jetzt selbst eine überholte Zahl im Namen.
- **Der Plan bleibt an einer Stelle ungenau, und das ist bewusst nicht korrigiert.** S15 beschreibt den Zielabschnitt so, wie es ihn nicht gibt (siehe oben). Der Schritt ist ausgeführt, der Plantext ist eine Aufzeichnung seines damaligen Standes; die Abweichung steht hier.
- **Der Verlaufseintrag des Coders trägt eine Uhrzeit aus der Zukunft.** `260813-0620-coder-s11-bis-s14-weitere-instanz.md` ist um 0509 geschrieben worden. Kein Befund an der Sache, aber die Namensfolge im Ordner gibt die Reihenfolge der Arbeit nicht wieder.

Kein Commit; das Committen liegt beim Orchestrator.
