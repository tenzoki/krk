# Abgleich nach Turn 23 — 260806-1647

**Absender:** reconciler
**Domäne:** code
**Sitzung:** `history/260806-1140-orchestrator-session.md`, Turn 23
**Bereich:** `git diff 8fd50a6..HEAD`, zwölf Commits `194ea16` bis `89f5570`
**Codestand:** `89f5570`

---

## Was geprüft wurde

| Speicher | Dateien | davon geändert |
|---|---|---|
| Planung (Plan und Spec, Circle) | 2 | 2 |
| Defekte (Circle + `shared/`) | 137 | 3, dazu 1 neu angelegt |
| Entscheidungen (Circle + `shared/`) | 36 | 3 |
| Prüfberichte (Circle) | 6 | 1 |

`shared/planning/`, `shared/reviews/` und `shared/analyses/` sind leer; `shared/issues/` trägt einen Eintrag, geschlossen seit dem 260802.

`cargo test --workspace` läuft am geprüften Stand grün: 13 Testprogramme, 474 Prüfungen, 0 Fehlschläge, 1 übersprungene.

---

## Ergebnis in einem Satz

Die Buchführung dieses Turns hält der Prüfung stand: alle 24 Schließungen sind durch Commits gedeckt, alle 36 Schrittmarker stimmen, und kein Entscheidungsmarker steht falsch. Gefunden wurden fünf Abweichungen, vier davon Nachzüge an Statuszeilen und Belegen, eine sachlich neu.

---

## Die fünf Abweichungen

### 1. Eine bindende Entscheidung ist nie in den Plan gelangt — neu

`decisions/260802-1810_*_sortierung-ohne-sprachsensitive-kollation.md` schreibt sich selbst zu: "Die Frage muss vor Schritt 12 beantwortet sein." S12 trägt seit dem 260804-1040 `[DONE]`, alle 36 Schritte sind abgenommen, und die Frage steht unverändert auf `_o_`.

Der Datensatz taucht in keiner Planstelle und in keiner Specstelle auf. Gesucht wurde über den ganzen Projektbaum nach dem Thementeil des Dateinamens; getroffen hat allein `CLAUDE.md:79`, und diese eine Stelle sagt es im Präsens ("bindet Schritt S12"), als stünde S12 noch aus.

Ausgeliefert wird damit die ungeprüfte Vorbelegung des Coders aus S2:

- `crates/krk-core/src/verzeichnis/eintrag.rs:80-86` — `sortierschluessel_bauen` vergleicht nach Unicode-Position und schreibt im eigenen Kommentar aus, dass er die Kollation nicht leistet. `Äpfel` sortiert hinter `Zebra`, in einer Anwendung mit deutschsprachiger Oberfläche.
- `crates/krk-core/src/verzeichnis/sortierung.rs` — "Sortierung nach Typ" ordnet nach Ordner/Datei/Verknüpfung; `Eintrag` trägt kein Feld für die Endung.

Angelegt als `issues/260806-1647_*_die-sortierfrage-bindet-s12-und-steht-in-keiner-planstelle.md`. Der Entscheidungsdatensatz hat den Befund als Abgleichsnotiz bekommen und bleibt `_o_`.

**Warum es jetzt zählt.** Die Empfehlung des Datensatzes lautete "Möglichkeit 3 bis zur Messung", und der Grund war das Messgate S8: eine vorher umgestellte Sortierung hätte die Messung mehrdeutig gemacht. Das Gate ist seit dem 260803-1755 durch, die vollständige Abnahme-Messreihe liegt seit dem 260806-0018 vor. Die Bedingung, unter der die Empfehlung wartete, ist erfüllt — und die Runde steht vor der Schließung.

### 2. Die Statuszeile des Specs übersprang zwei eigene Nachzüge

`planning/260802-1036_*_spec-navigator-geruest.md:4` sagte "Alle Nutzerantworten bis 260805-1411 sind eingearbeitet", während die Datumszeile darüber 260805-1623 und 260806-1412 als spätere Überarbeitungen führt. Beide sind tatsächlich eingearbeitet: C11 (Terminal im angezeigten Ordner) und die Bildgrenze von 64 MB in C6 Zeile 290 samt Festlegung 297-299 und C10 Zeile 396. Die Zeile ist auf 260806-1412 gezogen.

### 3. Eine `Implemented:`-Zeile verwies auf sich selbst

`decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md` trug "Implemented: S6b (dieser Commit)". Ein Verweis auf den schreibenden Commit löst sich nicht auf, sobald man ihn liest. Der Hash `194ea16` ist nachgetragen, mit dem Befund der Nachprüfung: `crates/krk-ui/src/appkit/hinweis.rs` ist in jenem Commit neu und trägt den einzigen anwendungsmodalen `NSAlert`.

### 4. Zwei `_i_`-Datensätze führten im Rumpf noch "answered"

`decisions/260802-1134_*_sprache-und-ui-werkzeugkasten.md` und `decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md` trugen `**Status:** answered` bei Marker `_i_`. Beide auf `implemented` gezogen. Die übrigen 23 `_i_`-Datensätze stimmen.

### 5. Eine Schließung ist eine Übergabe, keine Behebung

`issues/260805-1130_*_der-groessenformatierer-schreibt-zero-kb-auf-englisch.md` trägt `_c_`, ohne dass "Zero KB" verschwunden wäre; der Eintrag ist in `issues/260806-1215_*_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md` (`_o_`) übergegangen, dessen Auflösung beim `ontocoder` liegt.

Der Marker bleibt: `_c_` deckt in der Konvention auch "closed" ohne "resolved" ab, und die Sache verschwindet nicht, weil der Nachfolger offen steht. Genannt sei trotzdem, dass die Schlusszeile `Übergeben:` heißt, wo die Konvention `Resolved:` erwartet — wer nur die Markerliste überfliegt, liest hier ein behobenes Verhalten, das noch steht. Ein `_d_` wäre falscher, weil die Sache nicht zurückgestellt ist. Ein eigener Defekt ist daraus nicht geworden.

---

## Die 24 Schließungen dieses Turns

Siebzehn Umbenennungen `_o_`→`_c_` an vorher offenen Einträgen, dazu sieben Einträge, die vom Coderev `reviews/260806-1335-*` und von den Umsetzungen im selben Bereich angelegt und geschlossen wurden. **Keine `Resolved:`-Notiz behauptet mehr, als der Code trägt.** Stichproben:

| Defekt | Notiz sagt | Nachgesehen |
|---|---|---|
| `260806-0834` Bildgrenze | `BILDGRENZE = 64 MB`, geprüft vor dem Lesen | `crates/krk-ui/src/vorschaumodell.rs:95`, Prüfung `:506`, dazu `const _: () = assert!(BILDGRENZE > TEXTGRENZE);` in `:100` |
| `260806-1331` Aufschub | vollständige Fallunterscheidung ohne Auffangzweig | `crates/krk-ui/src/auffrischung.rs:179` `schiebt_auffrischung_auf`, `:237` `auffrischung_aufgeschoben`, vier Prüfungen ab `:532` |
| `260806-1328` Strg+C | Signalgriff, `signal-hook 0.4` nur in `krk-bench` | `crates/krk-bench/src/messen.rs:1150-1300` (`Sitzungssicherung`, `Sitzungswaechter`, beide mit `Drop`), `Cargo.toml:33`, `crates/krk-bench/Cargo.toml:20` |
| `260806-0834` Grenzprüfung | liest `pub use` und führendes `::` mit | `xtask/src/release.rs:185` `ist_objc2_use`, `:206` `sichtbarkeit_abstreifen`, Prüfung `sichtbarkeit_und_fuehrendes_doppelkolon_kommen_nicht_durch` läuft grün |
| `260806-0834` Binärname | kein Literal mehr | `xtask/src/bundle.rs:68-86`, `Gebaut { buendel, binaer }` aus `CFBundleExecutable` |
| `260806-0014` Prüfordner | außerhalb der Systembereinigung | `Makefile:118` `MESSPLATZ := $(HOME)/Library/Caches/krk-messplatz`, vier Pfade daraus abgeleitet |
| `260805-1455` AutoFill | wirksamer Schlüssel ist ein anderer als der geratene | `crates/krk-ui/src/appkit/menue.rs:151-168`, `NSAutoFillSystemInsertMenuEnabled` verneint |
| `260806-1330` eingefrorene Liste | Abbruchbericht wird nachgetragen | `crates/krk-ui/src/appkit/anwendung.rs:2680` und `:2709` `abbruch_ohne_meldung_nachtragen` |
| `260806-1123` Auswahlwiederherstellung | geht durch dieselbe Prüfung wie die Ersteinstellung | `crates/krk-ui/src/belegungsmodell.rs:302` `waehlbare_zeile`, gerufen aus `appkit/belegungsansicht.rs:303` |
| `260805-0905` CFRunLoop | Merkmal entfernt, Kommentar begründet nur noch zwei | `Cargo.toml:38-53`, die Zeile ist fort und der Kommentar hält den Wegfall fest |

Zwei Schließungen tragen eine Einschränkung, die in der Notiz steht und zutrifft:

- `260806-1123` (Markerzitate) schließt 62 Zitate in 32 Codedateien und lässt 14 in den beiden Datendateien unter `resources/` liegen, weil die dem `ontocoder` gehören. Nachgezählt: `resources/default-keymap.toml` und `resources/default-settings.toml` tragen zusammen 13 Zeilen mit ausgeschriebenem Marker. Weitergeführt als `issues/260806-1320_*_die-belegungsdateien-zitieren-workbench-pfade-mit-zustandsmarker.md`.
- `260806-1331` (Aufschub) führt den Vorbehalt zum schnellen Verschieben als eigenen Eintrag weiter, `issues/260806-1445_*`.

Eine Schließung ist keine Reparatur, sondern eine Zurückweisung des Befunds, und das steht sauber darin: `260805-0841` (Menü-Protokoll) hat nachgemessen, dass die Zweitform an keinem Auslesezeitpunkt erscheint, und den Modulkopf entsprechend gefasst statt Code zu ändern.

---

## Die acht offenen Defekte

Keiner ist beiläufig erledigt worden. Für die drei mechanisch prüfbaren nachgesehen:

| Defekt | Stand am Code |
|---|---|
| `260806-1333` Grenzprüfung sieht nur eine Kiste | `xtask/src/release.rs:102-105` prüft weiter allein `crates/krk-ui/src`; `krk-core`, `krk-bench` und `xtask` bleiben ungeprüft. Bleibt `_o_`. |
| `260806-1320` Markerzitate in den Datendateien | 13 Zeilen in den beiden `resources/*.toml`. Bleibt `_o_`. |
| `260806-0904` `CLAUDE.md` veraltet | `## Bindende Grundlage` steht unverändert auf dem 260803-1321. Bleibt `_o_`, mit nachgetragenen Zahlen. |

Die fünf übrigen (`260805-0000` toter Netzpfad, `260805-1730` Lesezeichen-Gültigkeit, `260806-1215` Byte-Angaben auf Englisch, `260806-1304` L6-Steher, `260806-1445` schnelles Verschieben) beschreiben Zustände, die kein Commit dieses Turns berührt hat, oder Vorbehalte ohne Beobachtung. Alle bleiben `_o_`.

### `CLAUDE.md` — die Meldung ist inzwischen selbst überholt

Der Defekt `260806-0904` zählte vier Abweichungen. Zwei haben sich bewegt, zwei nicht:

- `## Projektstand` ist am 260806-0014 nachgezogen worden (`e8626b6`). Zeile 40 sagt "34 der 36 Schritte tragen dort `[DONE]`, offen sind S6b … und S23"; beide sind seither abgenommen, S23 mit `d577295`, S6b mit `194ea16`. Es sind 36 von 36.
- `## Bindende Grundlage` steht unverändert auf dem 260803-1321. Zeile 71 sagt "sieben Fragen" beantwortet oder umgesetzt — es sind **25**. Zeile 73 sagt "fünf Fragen" offen — es sind **11**.
- Zeile 81 sagt "Die Sortierfrage bindet Schritt S12" im Präsens; siehe Abweichung 1.

Die Zahlen sind an den Defekt angehängt. `CLAUDE.md` selbst bleibt unberührt: die Revision gehört ans Sitzungsende.

---

## Marker, die bewusst nicht bewegt wurden

**Der Plan bleibt `_o_`, die Statuszeile "In Arbeit".** Die Konvention setzt bei 36 von 36 `[DONE]` sonst `**Status:** Complete` und Marker `_c_`. Hier hält der Nutzerentscheid vom 260806 die Runde ausdrücklich offen, bis `decisions/260806-0014_*_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md` geklärt ist. Die Abweichung ist gewollt, in der Statuszeile begründet und bleibt.

**Kein `_o_`-Entscheidungsdatensatz ist auf `_a_` gezogen worden.** Für keinen der elf liegt eine Antwort auf der Platte; die vier ältesten des Circles und die drei projektweiten warten auf spätere Runden, die vier jüngeren auf den Nutzer. Nachgesehen wurde jeder gegen `analyses/`, `planning/` und die übrigen Entscheidungen.

**Kein `_i_` und kein `_s_` wurde zurückgedreht.** Beide sind Endzustände.

---

## Neu angelegt

- `issues/260806-1647_o_die-sortierfrage-bindet-s12-und-steht-in-keiner-planstelle.md`

## Geändert

| Datei | Änderung |
|---|---|
| `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` | Reconciliation Log um den Abgleich 260806-1647 erweitert |
| `planning/260802-1036_o_spec-navigator-geruest.md` | Statuszeile von 260805-1411 auf 260806-1412 gezogen |
| `decisions/260802-1810_o_sortierung-ohne-sprachsensitive-kollation.md` | Abgleichsnotiz mit Codebelegen, Marker unverändert `_o_` |
| `decisions/260803-2025_i_wie-zeigt-krk-dem-nutzer-fehler.md` | Commit-Hash `194ea16` nachgetragen, Statusfeld auf `implemented` |
| `decisions/260802-1134_i_sprache-und-ui-werkzeugkasten.md` | Statusfeld auf `implemented` |
| `issues/260806-0904_o_claude-md-...` | Abgleichsnotiz mit den heutigen Zahlen |
| `issues/260805-1130_c_der-groessenformatierer-...` | Abgleichsnotiz zur Übergabe statt Behebung |
| `reviews/260806-1335-coderev-turn-23-...` | Tabelle mit dem Stand der sechs Befunde, Befundtexte unberührt |
| `history/260806-1140-orchestrator-session.md` | Abschnitt `## Coherence` angehängt |
