# Abgleich zum Abschluss der Runde 14 — Auswahl und Kopieren in der Vorschau

**Datum:** 260820-0834
**Domäne:** `code`
**Baumstand:** `05cb614`
**Bereich:** `fce0b6f..HEAD`, fünfzehn Commits
**Sitzung:** `shared/history/260819-2026-orchestrator-session.md`
**Circle:** `circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/` (Marker `_t_`, nicht angefasst)

---

## Was gezählt wurde

| | Gelesen | Geändert |
|---|---|---|
| Plandateien (Spec und Plan) | 2 | 2 (Marker `_o_` → `_p_`, Status, Abgleichsprotokoll) |
| Entscheidungsdatensätze der Runde | 7 | 7 (5 auf `_i_`, 2 mit Abgleichsnotiz auf `_a_` belassen) |
| Aktive Datensätze projektweit (`_o_` + `_a_`) | 51 | — |
| Defektdatensätze der Runde | 10 | 7 (4 Durchsichtsbefunde nachgelesen, 3 weitere annotiert) |
| Durchsichtsberichte | 1 | 1 (Abgleichstabelle angehängt) |
| Neu gefilt | — | 1 (`shared/issues/260820-0834_o_…sitzungsprotokoll…`) |

**Prüflauf am 260820-0834 gegen `05cb614`, allein im Baum, ohne zweiten Agenten:**
`cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` laufen alle vier grün.
Keine Probe fällt aus, keine Warnung steht.

---

## Die sieben Entscheidungsdatensätze, je einzeln gegen den Baum

Fünf sind umgesetzt und tragen jetzt `_i_` mit einer `Implemented:`-Zeile; das Kopffeld
`**Status:**` ist im selben Vorgang auf `implemented` gesetzt, wie es
`shared/decisions/260814-1955_o_sechs-beantwortete-entscheidungsdatensaetze-tragen-im-kopf-weiter-status-open.md`
verlangt.

| Datensatz | Neu | Beleg am Baum |
|---|---|---|
| `wird-die-vorschauflaeche-auswaehlbar-…` | `_i_` | `setSelectable(true)` an der einen Stelle, die die Textanzeige baut (`crates/krk-ui/src/appkit/vorschau.rs:1437`), ohne Fallunterscheidung über den Inhaltswert; `setEditable(false)` unverändert (`:1436`); die Bildansicht ist ein eigenes `NSImageView` (`:521`, `:696`) und unberührt. Commits `dfacf29`, `1b85538`. |
| `was-landet-beim-gerenderten-markdown-…` | `_i_` | `Gerendert.quellbezug` (`markdown.rs:271`), `Quellbezug::quelltext` (`:335`), Überschreibung `auswahl_ablegen` (`vorschau.rs:445-461`). Commits `13be459`, `91f8727`, `17dad8a`. |
| `welche-auszeichnungszeichen-fahren-an-den-raendern-…` | `_i_` | Fixpunktregel `klammern_schliessen` (`markdown.rs:434`), Klammer an Vor- und Nachspann `klammer_der_raender` (`:991`); drei Proben (`:2595`, `:2774`, `:2805`). Commits `91f8727`, `05cb614`. |
| `welches-kontextmenue-zeigt-die-auswaehlbare-vorschau` | `_i_` | Die Antwort kostet keine Zeile, und genau das ist gemessen: die Fläche ist auswählbar, und `textView:menu:forEvent:atIndex:` (`vorschau.rs:626`) ist seit `fce0b6f` byteweise unverändert — `git diff fce0b6f..HEAD` an dieser Methode ist leer. Commit `dfacf29`. |
| `was-tun-pfeil-hoch-und-runter-…` | `_i_` | Die Anmeldung `ist_eigene_textflaeche` (`anwendung.rs:2402`) hält `AuswahlHoch` und `AuswahlRunter` mit `Fokus::Vorschau` zulässig; Probe `kommandos/zulaessigkeit.rs:846`. Commit `6531f38`. |

**Zwei bleiben auf `_a_`, und beide aus einem Grund, den der Baum trägt:**

- **`gilt-die-quelltextzusage-auch-fuer-das-ziehen-einer-auswahl-und-die-dienste`.** Der Plan
  sagt es selbst: seine Tabelle `## Welcher Schritt welchen Datensatz realisiert` trägt für
  diesen Datensatz „erst nach der Bündelabnahme von C2.12". Dazu misst der offene Befund
  `issues/260820-0733_o_…` am Baum, dass die eine Abfangstelle den Parameter `sorten` im
  Markdown-Zweig nicht liest und `text_auf_ablage_schreiben` unbedingt `clearContents()` ruft.
  Für die Zwischenablage ist die Zusage eingelöst; für das Ziehen und die Dienste, um die dieser
  Datensatz allein geht, ist sie es nicht. `_i_` wäre hier die unehrlichere Auskunft.
- **`schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7`.** Die Antwort lautet „kein
  Lauf" und ist durch keinen Commit einzulösen; eingelöst wird sie durch das, was an die Stelle
  des Laufs treten sollte. Davon steht eine Hälfte: C2.13, der Ort der Abbildung, hat mit
  `das_vorschaumodell_weiss_von_der_einfaerbung_nichts` (`vorschau.rs:1558`) seinen Prüfer. C2.3
  und C2.4, die Zahl der Durchgänge, haben keinen — die Sache stimmt am Baum
  (`into_offset_iter` in `markdown.rs:582` genau einmal, `Quellbezug::quelle` aus
  `self.quelle.to_owned()` in `:1594`), aber kein Kommando misst sie nach. Der Verzicht auf einen
  Abnahmelauf hängt an genau diesen zwei Kriterien. Die dritte Hälfte der Antwort — L7 auf die
  Gegenstände der späteren Messrunde — hat im Baum überhaupt keinen Ort.

---

## Spec und Plan: `_p_` und ausdrücklich nicht `_c_`

Beide standen auf `_o_`, und `_o_` ist falsch geworden: acht von acht Planschritten stehen auf
`[DONE]`, jeder einzeln gegen den Baum gelesen, vier Prüfkommandos grün.

**`_c_` wäre ebenso falsch, aus zwei unabhängigen Gründen.**

Der erste ist die Sache. **15 der 39 Abnahmekriterien tragen einen Bündelanteil** und sind am
laufenden `KRK.app` im Vordergrund abzunehmen: C1.1, C1.2, C1.6, C1.8, C1.9, C1.10, C1.11,
C1.12, C2.1, C2.2, C2.11, C2.12, C3.1, C3.2 und C3.3. Keines ist gefahren, und **kein Agent kann
sie fahren** — aus dem Hintergrund gestartet weist die Wirkungsbereichs-Prüfung jeden
fokusgebundenen Befehl ab. Der Abhängigkeitsgraph des Plans endet auf genau diesem Knoten. Ein
Plan, dessen eigener Text eine ausstehende Verpflichtung nennt, ist nicht geschlossen.

Der zweite ist eine offene Nutzerfrage.
`shared/decisions/260819-1440_o_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`
fragt genau, ob `_c_` an einem Spec „gebaut" oder „abgenommen" heißt. Spec und Plan jetzt auf
`_c_` zu setzen entschiede diese Frage durch vollendete Tatsache. `_p_` ist der einzige der vier
Marker, der heute wahr ist und die Frage offen lässt.

Beide Dateien tragen jetzt ein `## Reconciliation Log` mit den Belegen je Schritt, und ihr
Kopffeld `**Status:**` steht auf „Teilweise abgeschlossen".

---

## Die vier offenen Durchsichtsbefunde, gegen `05cb614` nachgelesen

Der Nutzer hat sie für diesen Durchgang ausdrücklich ausgeschlossen. Geprüft ist allein, ob ihre
Beschreibung nach der Wurzelbehebung noch zutrifft. **Alle vier treffen unverändert zu**;
`05cb614` hat neben den Workbench-Dateien allein `crates/krk-ui/src/markdown.rs` angefasst.

| Befund | Stand |
|---|---|
| `260820-0733_o_` Abfangstelle verwirft Sorten und leert jede Ablage | trifft zu. `vorschau.rs` und `zwischenablage.rs` sind seit `b28cdd6` byteweise dieselben. |
| `260820-0735_o_` Anker des Freigabedialogs ist das ganze Dokumentrechteck | trifft zu. `fokusansicht` (`vorschau.rs:832-838`), `flaeche.bounds()` in `teilen`, `setMaxSize(f64::MAX)` (`vorschau.rs:1441`) stehen wie beschrieben. |
| `260820-0737_o_` zwei Kriterien mit Probenkennzeichnung ohne Probe | trifft zu, **mit zwei gewanderten Zeilennummern**: `into_offset_iter` steht jetzt in `markdown.rs:582` statt `:593`, `quelle: self.quelle.to_owned()` in `:1594` statt `:1546`. Die Sache ist unverändert. |
| `260820-0739_o_` `text_schreiben` ohne `#[must_use]` | trifft zu. `#[must_use]` in `zwischenablage.rs:258` über `text_auf_ablage_schreiben`, `text_schreiben` (`:270`) ohne. |

Jeder der vier hat eine Abgleichsnotiz mit dem Nachgelesenen bekommen; der Text der Befunde
selbst ist unangetastet.

---

## Die drei weiteren offenen Datensätze

Alle drei binden künftige Arbeit und nicht diese Runde; alle drei sind annotiert und bleiben
offen.

- **`issues/260820-0646_o_`** — der Plan schreibt Zählerwartungen, ohne sie am Baum zu halten.
  Alle drei Fälle sind zugunsten des Baums gelöst: die berichtigte Probe
  `die_zwei_schalter_stehen_je_an_genau_einer_stelle_und_dort` zählt Fundstellen je Datei statt
  eine Null zu erwarten, `die_zuordnung_auf_eine_ansicht_steht_in_der_vorschau_genau_einmal` misst
  die Vorschau statt des Baums, und die `expect(dead_code)`-Zeile ist weggelassen. Der Befund
  richtet sich gegen die Gestalt des Planens und nicht gegen diese Umsetzung.
- **`shared/issues/260820-0602_o_`** — `make check` fasst den ganzen Arbeitsbereich. Der
  Abgleichslauf ist allein gefahren, also belegt sein grünes Ergebnis diese Runde; der Befund
  handelt von der Aussagekraft bei parallelem Lauf und ist unberührt.
- **`shared/issues/260819-2206_o_`** — der geteilte `/tmp`-Namensraum der Commit-Nachricht. Der
  Defekt liegt in fusion und nicht in KRK. Kein Commit dieser Runde trägt eine fremde Nachricht;
  die fünfzehn sind gelesen.

---

## Neu gefilt

**`shared/issues/260820-0834_o_das-sitzungsprotokoll-der-runde-14-traegt-weder-directive-noch-turn-log.md`.**
`shared/history/260819-2026-orchestrator-session.md` führt nach fünfzehn Commits und drei Turns
weiterhin `**Directive:** (not yet stated)`, `**Mode:** (not yet resolved)` und einen Per-Turn-Log
mit der Zeile „(no Turn has started)"; der `## Turn log` des Circle-Datensatzes ist ebenso leer.
Die Directive steht allein in `fusion-workbench/agentstate.yaml`, und das ist Sitzungszustand,
der beim sauberen Ende gelöscht wird. Danach sagt kein Datensatz mehr, unter welcher Directive
diese fünfzehn Commits entstanden sind. Der Befund liegt im gemeinsamen Speicher, weil er zur
Buchführung des Orchestrators gehört und nicht von der Directive dieser Runde verursacht ist.

---

## Beobachtungen ohne eigenen Datensatz

- **Das Shaper-Protokoll zählt eine Zahl anders als der Spec.**
  `shared/history/260819-2216-shaper-auswahl-und-kopieren-in-der-vorschau.md:21` sagt „39
  Abnahmekriterien, davon **14** mit Bündelanteil". Nachgezählt am Spec sind es **15**. Das
  Protokoll ist eine Aufzeichnung seines Standes und bleibt unangetastet; verbindlich ist der
  Spec, und der Plan zählt dort ebenfalls 15.
- **Eine offene Frage aus einer früheren Runde ist mit C1.10 gekoppelt.**
  `shared/decisions/260813-0053_o_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`
  ist unbeantwortet. C1.10 — die Pfeiltasten bleiben wirkungslos — ruht darauf, dass der Abgriff
  schluckt, was zulässig war. Wird die Frage einmal anders beantwortet, ändert sich C1.10 mit.
  Das ist eine Kopplung und kein Widerspruch; sie steht hier, damit sie nicht als Neuigkeit
  wiederentdeckt wird.
- **`CLAUDE.md` ist nicht angefasst worden.** Welche Aussagen dort jetzt falsch sind, steht im
  Abschnitt darunter; die Entscheidung über einen Kuratorendurchgang liegt beim Nutzer.

---

## Was `CLAUDE.md` jetzt falsch sagt

Nicht geändert, nur benannt. Eine Aussage ist falsch geworden, drei sind unvollständig.

**Falsch — Zeile 124, der Absatz zum Ereignisabgriff.** Er sagt zweimal, es gebe genau eine
angemeldete Fläche:

> „…die Textfläche des Editors ist die **eine Ausnahme** davon…"
> „Ein Bereich der Fensterzeile wird angemeldet, sonst gehören seine Tasten AppKit und kein
> Befehl von KRK wirkt darin; **so der Editor**."

Seit `6531f38` sind es zwei. `Anwendungsdelegierter::ist_eigene_textflaeche`
(`crates/krk-ui/src/appkit/anwendung.rs:2402`) hält den Ersthelfer gegen die Textfläche des
Editors **und** gegen die der Vorschau; die Funktion hieß bis zu dieser Runde `ist_editorflaeche`.
Der übrige Absatz bleibt richtig, der Satz über die Fläche eines Blattes eingeschlossen: sie wird
weiterhin nicht angemeldet.

**Unvollständig — die Tabelle der Runden (Zeilen 12 bis 27).** Sie führt dreizehn Runden; die
vierzehnte fehlt. Solange der Circle `_t_` trägt, ist die Tabelle nicht falsch — nach der
eigenen Regel darüber zählen nur `_b_` und `_c_` als gefahren —, aber die Pfadregel im Absatz
danach („Ohne Nennung gilt die Runde 2") greift für Pfade dieser Runde ins Leere, sobald sie
zitiert werden.

**Unvollständig — Zeile 74, die eine Hülle um `NSPasteboard`.** „…sie ist seit der Runde 4 auch
Ziel und nicht mehr nur Quelle" stimmt weiter. Seit der Runde 14 nimmt sie zusätzlich eine
hereingereichte Ablage entgegen (`text_auf_ablage_schreiben`,
`crates/krk-ui/src/appkit/zwischenablage.rs:259`), und der Satz nennt das nicht.

**Unvollständig — es fehlt ein Absatz über die Vorschau.** `CLAUDE.md` sagt heute nirgends, dass
die Vorschaufläche auswählbar ist, dass bei gerendertem Markdown der **Quelltext** kopiert wird
und dass ein `Quellbezug` neben dem gerenderten Text steht. Die Datei hat nie behauptet, die
Fläche sei unauswählbar, also ist keine Aussage falsch geworden; es fehlt eine.

**Nicht falsch geworden, obwohl man es vermuten könnte:** die vier gewachsenen Aufzählungen
(Zeile 74 ff.) — `git diff fce0b6f..HEAD` zeigt an `crates/krk-core/src`,
`crates/krk-ui/src/fenstermodell.rs`, `resources/default-keymap.toml`, `Cargo.toml` und
`Cargo.lock` keine Änderung; die Maximen-Absätze zu den zehn Zeitzusagen — diese Runde hat keine
angefasst und keine elfte gesetzt; und Zeile 174 zu den vorgesehenen Circles — der Web-Betrachter
ist weiterhin der einzige mit `_a_`.
