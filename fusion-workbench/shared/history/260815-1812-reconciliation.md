# Abgleich — Reparatursitzung 260815-1328

**Status:** Complete
**Domäne:** code
**Bereich:** `838432c..7fae5ba`, 12 Commits, sechs Turns
**Aktiver Circle:** keiner. Alles im gemeinsamen Speicher.
**Sitzungsprotokoll:** `fusion-workbench/shared/history/260815-1328-orchestrator-session.md`

---

## Was geprüft wurde

Zwölf in dieser Sitzung geschlossene Defektdatensätze, ein auf `_p_` stehender, elf neu
angelegte Datensätze, zwei Durchsichten samt Deckungslauf und die Aussagen von `CLAUDE.md`,
die diese Sitzung berührt hat. Jede Abschlussnotiz ist gegen den Baum gelesen; wo sie eine
Messung behauptet, ist die Messung nachgefahren und nicht geglaubt.

Der Prüflauf `cargo test --workspace` ist genau einmal gefahren und läuft grün, Exit 0.

## Die zwölf Abschlüsse, je Behauptung

Alle zwölf halten in der Sache. Kein Abschluss behauptet etwas, was der Baum nicht trägt.

| Datensatz | Behauptung | Hält? | Kommando oder Fundstelle |
|---|---|---|---|
| `260812-1438` Untergrenzen-Quote | Die Quote ist durch die Nennung der zwei Ausnahmen ersetzt; die Kiste führt 40 Dateien, ohne den Abschnitt sind weiterhin genau `koordinaten.rs` und `mod.rs` | ja | `find crates/krk-ui/src/appkit -name '*.rs' \| wc -l` → 40; je Datei `grep -q 'Ab welchem macOS die angesprochenen Klassen stehen'` → genau zwei ohne |
| `260812-1558` NSLayoutManager 10.7 | `nummernspalte.rs` trägt die 10.7 mit Fundstelle; `editor.rs` war schon berichtigt; der Folgesatz in `textmerkmale.rs` zählt keine fremden Köpfe mehr; `vorschau.rs` steht im Präteritum und ist unangetastet | ja | `nummernspalte.rs:84-85`, `editor.rs:387-388`, `textmerkmale.rs:64-68`, `vorschau.rs:137-139` |
| `260812-1628` Weitergabehinweis in `bundle` | Zusammenfassung statt Warnsatz; hängt an der Identitätsart über `DEVELOPER_ID_PRAEFIX`; `const _: () = assert!(ZIELE.len() == ARCHITEKTUREN.len())`; eine Probe hält den einen Rufer fest | ja | `xtask/src/sign.rs:192-208`, `xtask/src/release.rs:86`, `:111-112`, Probe `allein_der_unterbefehl_bundle_gibt_den_hinweis_aus` läuft grün |
| `260812-2253` Zahl der `Kommando`-Varianten | `CLAUDE.md` führt keine Ziffer mehr, sondern das Zählkommando; die drei stabilen Zahlen stimmen unverändert | ja | `awk '/^pub enum Kommando/,/^}/' … \| grep -cE '^    [A-ZÄÖÜ]'` → **78**; `Wirkungsbereich` 7, `Bereich` 5, `Fokus` 5; `CLAUDE.md:73` |
| `260812-2253` sieben tote Marker im Circle der Runde 5 | Zeile 22 und 71–77 tragen alle acht Zitate in der Sternform | ja | `sed -n '22p;71,77p' fusion-workbench/circles/260811-1304-statusleiste-mit-bereichsschaltern/_b_circle.md` |
| `260813-1345` 79/73 → 84/78 | Neun Zeilen in zwei Dateien; danach keine Fundstelle dieser Bedeutung mehr; Zielzahlen 84 und 78 statt der 82/76 des Datensatzes | ja | `menue.rs:128,799,867`, `belegungsausgabe.rs:45,48,56,256,725,726`; `grep -n '\b79\b\|\b73\b'` in beiden Dateien → Exit 1; `grep -c '^id = '` → 84, `grep -c '^gehalten_von = '` → 6, `Kommando` → 78 |
| `260815-1216` sieben Verweise mit totem Marker | Alle sechs namentlich genannten Fundstellen in der Sternform; die Erhebung über den ganzen lebenden Text lässt genau zwei stehen, und beide sind die benannten Ausnahmen | ja | `grep -rnoE "[0-9]{6}-[0-9]{4}_[a-z]_[a-z0-9-]{6,}" crates/ xtask/ CLAUDE.md circles/*/planning circles/*/_*_circle.md shared/planning` → 2 Treffer, beide Sätze über Zitate |
| `260815-1216` vierzehn Kopfzeilen `Status:` | Weg 1 gefahren; der Prüflauf gibt danach nichts mehr aus; Übergänge 9 / 3 / 2 | ja | Prüflauf über 137 Datensätze → 0 Abweichungen; `git show cd0b5b7` → `answered→implemented` 9, `open→implemented` 3, `open→deferred` 2 |
| `260815-1444` Auffangzweig behauptet keine Art mehr | Möglichkeit 1 gefahren; „bleibt auf dieser Maschine" und „moegliche Schadsoftware" sind aus dem Hinweis gefallen; zwei Proben halten die Abwesenheit | ja | `xtask/src/sign.rs:194-198`; `grep -rn 'bleibt auf dieser Maschine' xtask/src/` findet nur Doc-Kommentare und Zusicherungen; Proben `:598`, `:616` |
| `260815-1445` gehärtete Laufzeitumgebung | Steht im gemeinsamen Teil, also in beiden Zweigen; „und damit richtig" ist gefallen; die Probe bindet über `include_str!("main.rs")` an den Hilfetext; Gatekeeper steht in der schwächeren Form | ja | `xtask/src/sign.rs:199-208`, Probe `beide_faelle_nennen_die_fehlende_gehaertete_laufzeitumgebung` (`:651`), Bindung `:663` |
| `260815-1713` Ordnerfrage am Pfad | `bestimmen` fragt `std::fs::metadata`; ein Systemaufruf statt fünf; genau ein Rufer im ausgelieferten Code; zwei Proben dazugekommen | ja | `verweisziel.rs:164-165`; einziger Rufer `tabelle.rs:1426`; `crates/krk-core/tests/verzeichnis.rs` prüft an neun Stellen |
| `260815-1714` zwei statt drei Rufer der Hülle | Der dritte Rufer ist mit `7fae5ba` wieder weg; die sechs genannten Stellen stimmen ohne eine Änderung an ihnen | ja | `grep -rn 'ohne_warten_oeffnen' crates/` → Aufrufstellen `text/datei.rs:414` und `vorschaumodell.rs:679`, alles übrige Prosa |

**Vier der zwölf waren nicht am Anker vorhanden**, sondern in derselben Sitzung angelegt und
geschlossen: `260815-1444`, `260815-1445`, `260815-1713`, `260815-1714`. Alle vier stammen
aus einer der beiden Durchsichten und betreffen Code, den diese Sitzung selbst geschrieben
hat. Das ist der Rückkopplungsweg, wie er gedacht ist.

**Zwei Zahlen der Abschlüsse sind seit ihrer Messung weitergelaufen**, ohne dass eine Aussage
dadurch falsch wird: `xtask` fährt heute 98 Proben statt der 96, die `260812-1628` nennt
(dazugekommen sind die zwei Proben von `260815-1444` und `260815-1445`), und die
Entscheidungsdatensätze sind 137 statt 136, weil `7fae5ba` einen angelegt hat.

## Der Datensatz auf `_p_` — umbenannt auf `_o_`

`shared/issues/260814-1612_*_eine-verknuepfung-auf-einen-ordner-laesst-sich-nicht-betreten.md`

`_p_` heißt nach `rules/fusion-workbench-conventions.md` „ein Agent arbeitet aktiv daran".
Nach dem Ende der Sitzung arbeitet niemand daran; `agentstate.yaml` führt die Aufgabe `I:7`
als `done` mit Commit `8c06747`. Von den vier Werten `_o_`, `_p_`, `_c_`, `_d_` beschreibt
allein `_o_` die Lage „nicht abgeschlossen, niemand arbeitet daran". `_c_` wäre falsch, weil
die Abnahme aussteht; `_d_` verlangt nach derselben Regel eine Entscheidung des Nutzers und
steht dem Abgleich nicht zu. Gezogen ist deshalb `_o_`.

**Der Sache nach ist der Datensatz weder offen noch geschlossen**, sondern in der Lage, in der
neun der zehn gefahrenen Runden dieses Projekts stehen: gebaut, im Kern geprüft, nicht
abgenommen, weil der Lauf KRK im Vordergrund verlangt. Ein Gegenstück zum Circle-Marker `_b_`
hat das Defektvokabular nicht. Ob `_d_` mit dem Ziel „nächster Abnahmelauf" die bessere
Ablage ist, gehört dem Nutzer und ist hier nicht entschieden.

**Zwei Aussagen seiner Notiz vom 260815-1700 sind seit `7fae5ba` überholt** und im Datensatz
vermerkt: die Angabe, `verweisziel` sei der dritte Rufer von `sys::ohne_warten_oeffnen`, und
der Nebenbefund, `open(2)` scheitere am Socket mit `ENXIO` — gemessen ist `EOPNOTSUPP`, und
seit dem Wechsel liefert der Socket `KeinOrdner`.

## Die neu angelegten Datensätze

Zwölf neue Datensätze, elf unter `shared/issues/` und einer unter `shared/decisions/`. Alle
liegen richtig: kein Circle war aktiv, also ist der gemeinsame Speicher nach der
Herkunftsregel der richtige Ort, und jeder Datensatz begründet die Ablage selbst.

**Die Umlegung stimmt.** `260815-1749` ist während der Sitzung von `issues/` nach
`decisions/` gewandert und trägt heute die Form eines Entscheidungsdatensatzes mit Frage,
Optionen und Vokabular `_o_`. Die Auflösung ist „entscheiden und festhalten" und nicht
„gehen und beheben"; die Umlegung ist nach der Regel unter `## Issues vs Decisions` richtig.

**Eine Art ist strittig.** `260815-1448` („die neun berichtigten Zahlen stehen unverankert")
liegt als Defekt und sagt in seinem eigenen Schluss: „Die Entscheidung zwischen beiden Wegen
gehört dem Nutzer." Damit ist er der Form nach ein Entscheid und der Sache nach ein Defekt —
die Zahlen sind tatsächlich unverankert. Er ist hier nicht umgelegt; wer ihn umlegen will,
verschiebt ihn nach `shared/decisions/` und zieht den Marker auf das reichere Vokabular.

**Misfiled — should be a decision:** allein `260815-1448`, und auch der nur zur Hälfte.

## Die Deckung durch Durchsichten

```
$ "$FUSION_PLUGIN_ROOT/bin/fusion-review-coverage"
anchor=workbench-root  since=838432c  head=HEAD
commits=12  reviews=2  unusable=0  uncovered=7  verdict=uncovered
```

**Die Begründung trägt für sechs der sieben und nicht für den siebten.** `e37a1e3`,
`a2670db`, `cd0b5b7`, `f280c42`, `39060d4` und `223a333` fassen ausschließlich Dateien unter
`fusion-workbench/` an, nachgezählt über `git show --name-only --format=`. `7fae5ba` nicht:
er ändert `crates/krk-core/src/verzeichnis/verweisziel.rs` und
`crates/krk-core/tests/verzeichnis.rs`, also ausgelieferten Code und die Proben dazu.

Es ist zugleich der Wurf, der den einzigen Befund der Schwere **hoch** dieser Sitzung behebt
und dabei die Systemfrage selbst wechselt. Der Abgleich hat an ihm zwei Abweichungen
gefunden, beide auf der Beschreibungsebene, beide von einer Durchsicht auffindbar. Aufgenommen
als
`shared/issues/260815-1812_*_der-eine-codecommit-der-sitzung-260815-1328-ohne-durchsicht-ist-nicht-nur-markdown.md`.

## `CLAUDE.md` — was nach dieser Sitzung nicht mehr stimmt

`CLAUDE.md` ist in dieser Sitzung nicht angefasst worden (`git log 838432c..HEAD -- CLAUDE.md`
ist leer). Geändert hat sich der Baum darunter.

**Nicht mehr richtig:**

1. **Zeile 69, die zwei Verhalten neben der Belegung.** Dort steht „der Doppelklick auf eine
   Zeile (Ordner: hineingehen, sonst: an das System)". Seit `8c06747` und `7fae5ba` sind es
   drei Ausgänge und nicht zwei: `in_zeile_einsteigen` (`crates/krk-ui/src/appkit/tabelle.rs:1417-1449`)
   unterscheidet Ordner, Datei und Verknüpfung, löst die Verknüpfung über
   `verweisziel::bestimmen` auf, und ein unerreichbares Ziel geht **nicht** an das System,
   sondern schreibt eine Zeile in die Statuszeile.

**Weiterhin richtig, obwohl die Sitzung daran gerührt hat:**

2. **Zeile 135, „Die Hülle hat zwei Aufrufer".** War zwischen `8c06747` und `7fae5ba` falsch,
   stimmt seither wieder: `grep -rn 'ohne_warten_oeffnen' crates/` findet zwei Aufrufstellen.
   Der ganze Absatz über die Typprüfung am Deskriptor gilt unverändert für Editor und
   Vorschau.
3. **Zeile 73, die vier Aufzählungen.** Die drei ausgeschriebenen Zahlen stimmen
   (`Wirkungsbereich` 7, `Bereich` 5, `Fokus` 5), und für `Kommando` steht das Zählkommando
   statt einer Zahl — es liefert 78.
4. **Der Untergrenzen-Abschnitt unter `## Technologiewahl`.** Die Ausnahmenennung statt der
   Quote trägt: 40 Dateien, ohne den Abschnitt genau `koordinaten.rs` und `mod.rs`.

**Ältere Abweichung, nicht von dieser Sitzung verursacht:**

5. **Zeile 38, „liegt als `v0.3.0` aus".** `Cargo.toml` führt `version = "0.4.1"`, und die
   Tags `v0.4.0` und `v0.4.1` stehen. Die Zeile ist mit `a355347` vom 260814 in den Baum
   gekommen und war schon am Anker `838432c` falsch, der die Version auf 0.4.1 gezogen hat.

**Fehlend, nicht falsch:** `CLAUDE.md` kennt das neue Modul
`crates/krk-core/src/verzeichnis/verweisziel.rs` nicht. Eine Aussage wird dadurch nicht
unrichtig; die Revision von `CLAUDE.md` gehört nicht in einen Abgleich.

## Weitere Befunde

**Das Sitzungsprotokoll und der Sitzungszustand widersprechen einander.**
`shared/history/260815-1328-orchestrator-session.md:5` steht auf „In Arbeit — Turn 1 von 5",
während `agentstate.yaml` `turn: 6`, `max_turns: 6` und 9 von 10 Aufgaben als erledigt führt.
Das Protokoll nennt daneben ein Turn-Budget von 5 und `agentstate.yaml` eines von 6.
`agentstate.yaml` steht noch da, obwohl der Orchestrator es beim sauberen Ausstieg löscht,
und führt `current_task: I:7` mit `status: running`, während dieselbe Aufgabe in der
Warteschlange darunter als `done` mit Commit `8c06747` steht; `commits: 13` gegen 12 im
Bereich. Der Abgleich fasst diese Datei nicht an — sie gehört dem Orchestrator. Die
Statuszeile des Protokolls ebenso; angehängt ist dort allein der Abschnitt `## Coherence`.

**Das Sitzungsprotokoll ist nicht eingecheckt.** `git status --short` führt es als
unverfolgt. Kein Commit dieser Sitzung enthält es.

**Zwei Datensätze beschreiben eine Sache und schließen verschieden.** `260814-1955` (offen)
und `260815-1216` (geschlossen) beide über die auseinanderlaufende Kopfzeile `Status:`. Der
geschlossene nimmt die Restlage ausdrücklich als bekannt hin, der offene verlangt genau das,
was jener als nicht gangbar abgelegt hat. Der Prüflauf gibt heute nichts mehr aus. Vermerkt
in `260814-1955`; welcher der beiden Abschlüsse gilt, gehört dem Nutzer.

**Der Nebenbefund von `260814-1955` ist größer als seine eine Datei.** 49 der 137
Entscheidungsdatensätze tragen eine leere Zeile `Answered:` ohne Inhalt, 25 davon daneben
einen ausgefüllten zweiten Block. Vermerkt, nicht angefasst.

## Zahl der Abweichungen

**Acht**, davon zwei mit eigenem Defektdatensatz:

1. Der ungedeckte Commit `7fae5ba` fasst Code an, nicht nur Werkbank-Markdown → neuer Defekt.
2. Der Verweis in `verweisziel.rs:95` zeigt auf einen Pfad, den es nie gab → neuer Defekt.
3. Der Marker `_p_` an `260814-1612` beschreibt eine Lage, die nach der Sitzung nicht besteht
   → umbenannt auf `_o_`.
4. Zwei Aussagen der `_p_`-Notiz sind seit `7fae5ba` überholt (dritter Rufer, `ENXIO`)
   → vermerkt.
5. `260815-1752` sagt „Drei Stellen" und listet vier, und nennt im Titel zwei Modulköpfe, von
   denen einer eine Methodenbeschreibung ist → vermerkt.
6. `CLAUDE.md:69` beschreibt den Doppelklick mit zwei Ausgängen, es sind drei → gemeldet,
   nicht geändert.
7. `CLAUDE.md:38` nennt `v0.3.0`, ausgeliefert ist `v0.4.1` → gemeldet, älter als diese
   Sitzung.
8. Protokoll, Sitzungszustand und Bereich widersprechen einander in vier Größen (Turn, Budget,
   Aufgabenstand, Commit-Zahl) → gemeldet, nicht angefasst.

Keine Abweichung betrifft die Sache eines Abschlusses. Alle zwölf Abschlussnotizen halten,
was sie behaupten.

## Geschriebene Dateien

- `fusion-workbench/shared/issues/260814-1612_o_eine-verknuepfung-auf-einen-ordner-laesst-sich-nicht-betreten.md` (umbenannt von `_p_`, Notiz angehängt)
- `fusion-workbench/shared/issues/260815-1713_c_verweisziel-beantwortet-die-ordnerfrage-mit-open-und-nicht-mit-stat.md` (Notiz angehängt)
- `fusion-workbench/shared/issues/260815-1752_o_zwei-modulkoepfe-nennen-das-verweisziel-am-deskriptor-obwohl-es-am-pfad-fragt.md` (Notiz angehängt)
- `fusion-workbench/shared/issues/260814-1955_o_sechs-beantwortete-entscheidungsdatensaetze-tragen-im-kopf-weiter-status-open.md` (Notiz angehängt)
- `fusion-workbench/shared/issues/260815-1812_o_der-eine-codecommit-der-sitzung-260815-1328-ohne-durchsicht-ist-nicht-nur-markdown.md` (neu)
- `fusion-workbench/shared/issues/260815-1812_o_ein-verweis-im-modulkopf-des-verweisziels-zeigt-auf-einen-datensatz-der-nie-so-hiess.md` (neu)
- `fusion-workbench/shared/reviews/260815-1450-coderev-reparaturrunde-xtask-abschlusshinweis-und-vier-doc-kommentare.md` (Statusvermerk angehängt)
- `fusion-workbench/shared/reviews/260815-1720-coderev-verweisziel-am-deskriptor-und-der-berichtigte-weitergabehinweis.md` (Statusvermerk angehängt)
- `fusion-workbench/shared/history/260815-1812-reconciliation.md` (diese Datei)
- `fusion-workbench/shared/history/260815-1328-orchestrator-session.md` (Abschnitt `## Coherence` angehängt)
