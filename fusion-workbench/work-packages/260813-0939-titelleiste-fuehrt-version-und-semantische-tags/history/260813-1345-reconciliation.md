# Abgleich der achten Runde gegen den Baum

**Agent:** reconciler
**Datum:** 260813-1345
**Domäne:** code
**Stand:** `ed0388e`, Sitzungsspanne `9d5fcfa..HEAD`, acht Commits
**Circle:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/`
**Sitzungsprotokoll:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1006-orchestrator-session.md` (Abschnitt `## Coherence`)
**Kein Bündelbau, kein Vordergrundlauf.** `target/KRK.app` ist unberührt geblieben.

---

## Was in Zahlen geprüft ist

| Gegenstand | Gelesen | Ergebnis |
|---|---|---|
| Planschritte | 16 | 15 auf `[DONE]`, alle fünfzehn am Baum bestätigt; E2 offen und Nutzerarbeit |
| Ausgeführte Aufgaben | 17 | die siebzehnte (`F1`) steht in keinem Planschritt |
| Abnahmekriterien C1 bis C6 | 59 | 48 allein am Baum nachweisbar und alle 48 gehalten, 7 zur Hälfte am Bündel, 3 allein am Bündel, 1 Nutzerarbeit |
| Kriterien mit **(Probe)** ohne Probe | 9 | C2.8, C2.10, C4.1 bis C4.7 |
| Entscheidungsdatensätze im Circle | 5 | 4 auf `_i_` gezogen, 1 bleibt `_a_` |
| Offene Defekte im Circle vorher / nachher | 7 / 17 | zehn aus diesem Abgleich neu abgelegt |
| Geschlossener Defekt mit `Resolved:`-Zeile | 1 | nicht widerlegt, alle vier Stellen nachgelesen |
| Durchsichten | 3 | keine widerlegt; eine Zahl in der Codedurchsicht berichtigt |
| Bau | — | `make check` exit 0. `cargo test --workspace` 1025 Proben grün, `clippy --all-targets -- -D warnings` grün, `fmt --check` grün |

**Zur Probenzahl.** 1025 ist die naive Summe über 19 Ergebniszeilen von `cargo test --workspace`,
also dieselbe Zählweise, die der Abgleich der Runde 7 mit 1003 gegen 1001 gegenübergestellt hat.
Grün ist der Baum in beiden Zählweisen. `cargo test -p xtask` allein: 60 Proben, gewachsen von 49
vor der Runde.

---

## 1. Halten die Planschritte gegen den Baum?

**Alle fünfzehn, einzeln nachgelesen.** Jeder Schritt ist gegen die Dateien und Zeilen gelesen
worden, die er selbst nennt: existiert die Funktion, steht die behauptete Bedingung dort, ist die
Probe da, läuft sie grün, ist die Prosa mitgeändert.

### Strang A — die Zulässigkeitsregel

Die vierte Frage steht, und sie steht genau so, wie der Plan sie im Codeblock zeigt.
`Lage.schluesselfenster_gehoert_krk` (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:152`) mit dem
Doc-Kommentar zum anhängenden Blatt (`:142-151`), die Bedingung innerhalb des
`durchgelassen`-Ausdrucks (`:172-180`), der Modulkopf auf „Die vier Bestandteile" gezogen
(`:29`, Punkt (4) bei `:41-43`) und der Ausnahmeliste-Abschnitt, der sagt, welche drei sie
aufhebt und welchen nicht (`:77-79`). Die Tafel deckt 280 Fälle statt 140 (`:435`); das Viertel
ist ein Achtel geworden (`:410`), und alle vier `false`-Achtel tragen `ALLES_ABGEWIESEN`.

Der Anwendungsdelegierte erhebt das Schlüsselfenster **einmal**: `schluesselfenster()`
(`crates/krk-ui/src/appkit/anwendung.rs:2623-2639`) liest `NSApplication::keyWindow` an genau
einer Stelle im Baum (`:2625`) und vergleicht über `isEqual:` gegen Hauptfenster und
`attachedSheet`. `lage` erhebt den Wert bei `:2664` und reicht ihn an das Feld und an `fokus_bei`
weiter. `blatt_steht` ist eine eigene Frage geblieben (`:2592-2606`), und der Doc-Kommentar sagt
es ausdrücklich.

Der Nachtrag aus A3 hängt am Defekt der Runde 6 (`:83-113` jener Datei), und der Datensatz ist
**nicht** geschlossen worden — genau wie A3 es verlangt.

### Strang B — die Titelleiste

`titelzusatz::beschriftung` setzt über `concat!("KRK ", env!("CARGO_PKG_VERSION"))` zusammen
(`crates/krk-ui/src/appkit/titelzusatz.rs:142-144`). **Gesetzt ist `Left` und nicht `Leading`**
(`:192`) — die eine Stelle, an der ein Fehler zur Laufzeit abgebrochen wäre. Der Zusatz hängt an
genau einem Punkt im Fenster (`crates/krk-ui/src/appkit/fenster.rs:461`, nach `setContentMinSize`
in `:456`), und der Anfangstitel steht auf der leeren Zeichenkette (`:455`), die Zeile also
weiter da. `fenstertitel::titel` ist Zeile für Zeile unverändert; das Diff der Datei berührt
allein den Modulkopf, zwölf eingefügte Zeilen und keine gelöschte.

Die Modulliste in `appkit/mod.rs` führt 28 Namen, nachgezählt, und die Prosazahl ist von
„Sechsundzwanzig" — vor dieser Runde schon falsch — auf „Achtundzwanzig" berichtigt (`:10`).
`fokusanzeige_nachziehen` schreibt weiter genau die fünf Rahmenfarben und den Fenstertitel
(`anwendung.rs:3748-3761`), ruft weder `anwenden` noch `setHidden`.

### Strang C — der Über-Eintrag

`UEBER_BESCHRIFTUNG` (`crates/krk-ui/src/menuemodell.rs:117`) und `UEBER_SELEKTOR` (`:126`),
eingefügt über `ueber_eintrag_einfuegen` (`:367-378`) im Zweig `Funktionsbereich::Anwendung` und
**vor** `markdownausgabe_einfuegen` (`:245` vor `:246`). Der Doc-Kommentar an
`Eintrag::Sonderposten` ist auf „der Selektor, den die Antwortkette beantwortet" geweitet
(`:208-215`). `appkit/menue.rs` ändert keine Zeile Programmtext; das Diff berührt ausschliesslich
`//!`-Zeilen. Kein zweiter Zweig in `validateMenuItem:` (`anwendung.rs:747-761`).

**Die Zählprobe ist strenger gebaut, als der Plan verlangt, und die Begründung trägt.**
`die_leiste_traegt_zwei_sonderposten_und_zwei_trenner` (`:848-878`) zählt Sonderposten und
Trenner getrennt, statt eine Summe von vier zu halten: eine Summe bliebe stehen, wenn ein Posten
seinen Trenner verlöre und ein anderer einen dazubekäme.

### Strang D — Tag-Prüfung und README

`stand_pruefen` (`xtask/src/release.rs:226`) ist rein: kein Prozessaufruf, kein Dateizugriff, kein
Git-Verzeichnis. Acht Proben statt der sieben, die der Plan nennt; die achte
(`ein_aehnlicher_tag_deckt_die_version_nicht`, `:991`) hält fest, dass `v0.1.0-rc1` und `v0.1.10`
die Version `0.1.0` nicht decken. `git_fragen` (`:296`) ist die einzige Stelle im Baum mit
`Command::new("/usr/bin/git")`, eigens nachgezählt und von einer Zählprobe gehalten (`:1146`).
Station 1 ruft in der vorgesehenen Reihenfolge (`:182`, `:198`, `:199`, `:200`) und steht als
erste Zeile von `release::ausfuehren` (`:137`), vor `bundle::vorbereiten()` in `:139`.

Die Stationszählung steht an genau den drei vorgesehenen Stellen auf sieben und an keiner vierten:
`release.rs:3`, `main.rs:40`, `README.md:217`. Eigene Suche über `xtask/`, `crates/`, `README.md`
und `Makefile` nach jeder Stationszahl: keine veraltete im Baum. Der Abschnitt
`### Versionsstufen` steht unter `## Versionspflege` (`README.md:317`), und alle sieben Aussagen
von C4 stehen darin, einschliesslich der Berichtigung von „Nachzuführen ist nichts"
(`README.md:304-308`).

Das `Makefile` ist unberührt; keines der sieben an `bundle` hängenden Ziele hat eine neue
Vorbedingung bekommen.

### Der siebzehnte Schritt, der keiner ist

`F1` — `Kommando::FensterEinblenden` auf die Ausnahmeliste, Commit `ed0388e` — ist in Turn 2
gelaufen und steht in `agentstate.yaml` und im Circle-Datensatz, aber in keinem Planschritt. Der
Plan zählt sechzehn, ausgeführt sind siebzehn Aufgaben. Ein Abgleich legt keinen Planschritt an;
der Punkt steht im Reconciliation Log des Plans.

---

## 2. Die 59 Abnahmekriterien, sortiert nach ihrem Nachweisweg

| Sorte | Zahl | Welche |
|---|---|---|
| Allein am Baum nachweisbar | 48 | C1.2–C1.5, C1.7, C1.8; C2.1–C2.8, C2.10, C2.11; C3.1–C3.14; C4.1–C4.7; C5.2, C5.4, C5.7; C6.1–C6.8 |
| Zur einen Hälfte am Baum, zur anderen nur am Bündel | 7 | C1.1, C1.6, C1.9, C1.11; C2.9; C5.1, C5.3 |
| Allein am laufenden Bündel, Nutzerarbeit | 3 | C1.10, C5.5, C5.6 |
| Nutzerarbeit ohne Prüfung | 1 | C3.15 |

**Alle 48 der ersten Sorte halten.** Die Bau-Zusagen aus C6 sind einzeln nachgezählt und nicht
aus Prosa übernommen: `Kommando` 76, `Wirkungsbereich` 7, `Bereich` 5, `Fokus` 5,
`Funktionsbereich` 9, `resources/default-keymap.toml` 82 Funktionen mit 88 Kombinationen.
`#![deny(unsafe_code)]` steht an allen drei Kistenwurzeln, und die Ausnahmen sind weiterhin die
zwei bekannten Dateien (C6.5). `Cargo.toml`, `Cargo.lock` und `resources/` tragen im ganzen
Bereich keine Änderung (C6.1, C6.2, C6.6).

**Neun der 48 tragen die Kennzeichnung (Probe) und haben keine.** C2.8 und C2.10 hängen an
AppKit-Code ohne Bibliotheksziel; die sieben von C4 sind eine Planlücke, die die Durchsicht schon
so eingeordnet hat. Alle neun sind am Baum lesbar und von Hand nachgelesen. Abgelegt als
`issues/260813-1345_o_neun-abnahmekriterien-tragen-probe-und-haben-keine.md`. Derselbe Befund
mit derselben Zahl steht offen in der Runde 7; dass es zweimal neun sind, ist Zufall, dass der
Fall wiederkehrt, ist keiner.

**C6.8 hält nur unter einer stillschweigenden Einschränkung**, und der Datensatz dafür steht
schon: fünf Dateien im Baum tragen `impl Drop`, `temp_dir()` und `remove_dir_all` zugleich, drei
davon sind die anerkannten Fassungen, die vierte ist `xtask/src/release.rs`, die fünfte die Probe
selbst.

**Eine wörtliche Lesart von C1.5 und C2.1 trifft nicht ganz zu, und das ist eine Planfolge und
kein Befund.** Beide sagen, `setTitle` bekomme genau das, was `fenstertitel::titel` liefert.
`fenster.rs:455` setzt den Titel beim Aufbau einmal auf die leere Zeichenkette, und Schritt B2 hat
das ausdrücklich so entschieden und begründet: ein Titel `KRK` daneben zeigte den Namen zweimal,
und ein Fenster ohne `setTitle` trüge den Vorgabetitel von AppKit. Gemeint ist der nachgezogene
Titel, und für den hält die Zusage: `anwendung.rs:3803` ist die einzige Stelle, die ihn schreibt.

---

## 3. Die fünf Entscheidungsdatensätze

**Vier sind auf `_i_` gezogen**, jeder mit einer `Implemented:`-Zeile, die den Commit und die
Stellen im Baum nennt:

| Datensatz | Realisiert in | Beleg |
|---|---|---|
| `bekommt-krk-einen-eintrag-ueber-krk-im-anwendungsmenue` | `21dbc59` | `menuemodell.rs:117`, `:126`, `:367-378` |
| `reicht-ein-tag-auf-head-oder-muss-der-arbeitsbaum-sauber-sein` | `f9e5137` | `release.rs:226`, `:127`, `:137` |
| `wirken-krks-tastenbefehle-weiter-waehrend-der-ueber-dialog-steht` | `c3ada4d` | `zulaessigkeit.rs:152`, `:172-180`; `anwendung.rs:2623-2639` |
| `hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf` | `c3ada4d`, erweitert in `ed0388e` | `zulaessigkeit.rs:172-180`, `:198-201` |

**Einer bleibt `_a_`: `wer-setzt-den-ersten-tag-v0-1-0-und-wann`.** Die Antwort ist recorded
(Möglichkeit 1, der Nutzer setzt `v0.1.0` auf den Abschlusscommit), und sie ist nicht realisiert:
`git tag -l` liefert nichts. Der Datensatz kann von keinem Agenten auf `_i_` gezogen werden, weil
seine Realisierung eine Handlung des Nutzers ist. Das ist zugleich C3.15 und der Grund, warum die
Directive dieser Runde zur einen Hälfte ausserhalb der Reichweite jedes Agenten liegt.

**Zwei der vier `_i_`-Datensätze tragen eine Aussage, die der Bau widerlegt hat, und beide
Berichtigungen stehen aus.** Der Entscheid zum Über-Dialog nennt in seinem Abschnitt `## Question`
`F5` und `delete` als Beispiele, und beide tragen `Wirkungsbereich::Dateifenster`; sein
Vorteilssatz zu Möglichkeit 2 sagt, der Defekt zum Freigabedialog falle mit weg, und der
Freigabewähler ist kein Fenster. Beides ist als offener Defekt erfasst und beim Markerwechsel an
den Entscheid angehängt worden; der beanstandete Wortlaut steht weiter da. Die **Antworten**
selbst sind von beidem unberührt, und darum steht der Marker auf `_i_`.

---

## 4. Die acht Defekte des Circles

**Die sieben offenen stehen alle sieben zu Recht offen.** Jeder ist an den Stellen nachgelesen,
die er nennt; keiner ist behoben, keiner ist gegenstandslos geworden. Jeder trägt seit diesem
Abgleich eine Notiz mit dem heutigen Stand seiner Belege.

**Der eine geschlossene hält.** `fenster-einblenden-ist-nach-dem-schliessen-des-fensters-nicht-mehr-erreichbar`
nennt vier Stellen im Baum, und alle vier stehen so da: die Liste führt drei Kommandos
(`zulaessigkeit.rs:198-201`), der Modulkopf trägt die Herleitung, drei Prosastellen zählen drei,
und die Probe `ohne_schluesselfenster_kommt_fenster_einblenden_durch` (`:527-540`) geht über alle
fünf Fokuswerte. Die zwei ausdrücklich **nicht** behobenen Punkte bestehen wie beschrieben fort;
das ist kein Widerspruch zur Schliessung, sondern steht in der `Resolved:`-Zeile.

---

## 5. Der Querschnitt der Durchsicht, nachgezählt — und er reicht weiter

Die Codedurchsicht von Turn 1 hat sechs Prosastellen auf **eine** Ursache zurückgeführt: ein
Planschritt zählt seine Dateien abschliessend auf, die geänderte Zahl steht in einer anderen
Datei, und der Ausführer hält sich zu Recht an die Liste. Die vorgeschlagene Abhilfe: ein Schritt,
der eine gezählte Aussage ändert, nimmt die Dateien in seine Liste, die die Zahl nennen.

**Die sechs bestehen unverändert**, alle einzeln nachgelesen:

| Stelle | Was dort steht | Datensatz |
|---|---|---|
| `appkit/menue.rs:1132` | „die Tafel aus 140 Faellen" | `260813-1420`, Punkt 1 |
| `kommandos/mod.rs:25` | „einer ihrer drei Bestandteile" | `260813-1420`, Punkt 2 |
| `appkit/ereignisse.rs:90-92` | „die `Lage` aus Blattstand, Ersthelferbefund und Fokus" | `260813-1420`, Punkt 3 |
| `appkit/ereignisse.rs:103-110` | die Aufzählung (1) bis (3) ohne (4) | `260813-1420`, Punkt 4 |
| `appkit/anwendung.rs:2604-2607` | „dieselben drei Werte", „alle drei" | `260813-1258` |
| `appkit/anwendung.rs:733-736` | „der Eintrag der Markdown-Ausgabe", es sind zwei Sonderposten | `260813-1258` |

**Punkt 1 ist die Abhilfe, die sich selbst widerlegt hat.** Der Datensatz schlug vor, `menue.rs`
falle in Strang C an. Strang C hat die Datei geöffnet und fünf andere Prosastellen darin
nachgezogen; die Tafelzahl ist nicht mitgekommen.

### Die Antwort auf die gestellte Frage: der Querschnitt ist nicht vollständig erfasst

**Sechs weitere Stellen stehen in keinem der beiden Datensätze, und zwei davon widerlegen die
Ursachenerklärung.**

| Stelle | Was dort steht | Warum die Erklärung nicht greift |
|---|---|---|
| `kommandos/zulaessigkeit.rs:299` | „die drei abweisenden Viertel der Tafel" | **Steht in der Dateiliste von A1.** Die Tafel hat acht Achtel, sieben abweisend. Zwanzig Zeilen weiter (`:384`) steht es richtig |
| `kommandos/zulaessigkeit.rs:459` | „Der Fall, um dessentwillen die Regel drei Bestandteile hat" | **Steht in der Dateiliste von A1.** Der Modulkopf derselben Datei sagt seit A1 „Die vier Bestandteile" |
| `appkit/anwendung.rs:2690-2693` | „alle drei stehen jetzt in der einen Regel" | Grenzfall: historisch richtig, als Umfangsangabe irreführend. Drei Zeilen darunter steht „Die vier Bestandteile" |
| `appkit/anwendung.rs:4163-4165` | „die fuenf uebrigen Aufrufer" | Es sind sechs. Die Zahl kommt aus dem Plan und ist von der Durchsicht bestätigt worden |
| `appkit/titelzusatz.rs:130` | „Die einzige Stelle im Baum, die Name und Version zusammensetzt" | Vier Stellen in `crates/krk-bench/` setzen einen Namen mit derselben Version zusammen |
| `appkit/titelzusatz.rs:239`, `:266` | „Genau eine Stelle im Baum" | Dieselbe Verwechslung wie im erfassten Fall, andere Zeilen |

**Was die beiden ersten über die Abhilfe sagen.** Die Dateiliste ist die eine Hälfte und nicht
die ganze. Wer eine gezählte Aussage ändert, sucht die Zahl auch **innerhalb** der Datei, die er
ohnehin anfasst — hier hätte ein `grep -n 'drei\|Viertel'` in `zulaessigkeit.rs` beide geliefert.

**Und die Prosa hinkt nicht nur im Quellbaum nach.** Der Spec dieser Runde ist das einzige
lebende Dokument, das noch sechs Auslieferungsstationen zählt, während D3 den Baum an allen drei
Stellen auf sieben gebracht hat. Die Ursache ist eine andere als bei den sechs oben: ein Spec geht
dem Plan voraus und wird im Normalfall nicht nachgezogen, D3 zählt seine drei Stellen aber
abschliessend auf und meint damit den Baum.

**Fünf Stellen derselben Sorte stammen nicht aus dieser Runde** und liegen deshalb im gemeinsamen
Speicher: `menue.rs:128`, `:799-801`, `:867` sowie `belegungsausgabe.rs:45`, `:48` nennen 79
Funktionen und 73 mit `Kommando`; die Belegung führt seit der Runde 7 82 und 76. Die Zahl 79 kam
mit `90b02d4` aus der Runde 3 in den Baum und war damals richtig.

---

## 6. Die Durchsichten

**Kein Befund einer der drei Durchsichten ist widerlegt.** Die Codedurchsicht hat den einen hohen
Befund richtig erhoben und richtig eingeordnet; ihre vier geprüften Abweichungen vom Planwortlaut
halten alle vier.

**Eine Zahl in der Codedurchsicht stimmt nicht.** „`fokus()` bleibt als Hülle für seine fünf
übrigen Aufrufer, nachgezählt" — es sind sechs, und der sechste heisst `selbst.fokus()` statt
`self.fokus()`. Dieselbe Blindheit gegen die Empfängerform hat die Runde 7 schon einmal an zwei
Zählproben getroffen.

**Die Befunde der beiden Diagrammprüfungen sind nie behoben worden.** Beide schliessen mit „an
Ort und Stelle zu beheben", das Sitzungsprotokoll führt das als selbstverständlich mit, und weder
Spec noch Plan sind seit `59b0a6c` angefasst worden. Sechs Befunde stehen unverändert da.

Alle drei Durchsichten tragen seit diesem Abgleich eine Anmerkung.

---

## 7. Was dem Orchestrator gehört und hier nicht angefasst ist

**Das Sitzungsprotokoll trägt keinen Turn-2-Abschnitt.** Der Circle-Datensatz führt zwei Turns,
`agentstate.yaml` steht auf `turn: 2`, und das Protokoll endet mit „Turn 1 — Bilanz". Der
Abgleich hat allein den Abschnitt `## Coherence` angehängt, wie es ihm zusteht.

**Der Circle-Datensatz ist im Arbeitsbaum geändert und nicht eingetragen.** Er trägt die beiden
Turn-Einträge; das ist Arbeit des Orchestrators.

**`agentstate.yaml` bleibt gültig.** `plan_file` und `spec_file` nennen Namen, die dieser Abgleich
nicht umbenannt hat, `current_task` steht auf `E2` mit `status: gate`. Anders als beim Abgleich
der Runde 7 läuft hier nichts auseinander.

**`portfolio.md` kennt diese Runde noch nicht** und wird beim Abschluss vom Playmaker neu
erzeugt. Kein Befund.

---

## 8. Falsch abgelegt — gehört in den Entscheidungsspeicher

**Keiner.** Die siebzehn offenen Defekte des Circles sind der Form nach alle Defekte: jeder
benennt eine Stelle, an der etwas falsch dasteht oder fehlt, und jeder löst sich in eine Änderung
auf, die an einem Diff abzulesen ist. Zwei legen Wege mit Kosten vor
(`eine-vierte-wegwerfordner-fassung-…` mit zwei Fassungen der Zusage,
`neun-abnahmekriterien-tragen-probe-und-haben-keine` mit zwei Wegen), aber beide benennen die
kaputte Stelle zuerst und die Wahl danach. Das ist der Unterschied zu den beiden Fällen, die der
Abgleich der Runde 7 zum Umzug vorgeschlagen hat.

---

## 9. Was dieser Abgleich geändert hat

**Umbenannt (4):** vier Entscheidungsdatensätze von `_a_` auf `_i_`, jeder mit einer
`Implemented:`-Zeile und `**Status:** implemented`. Der fünfte bleibt `_a_`, weil seine
Realisierung ein Git-Tag ist, den nur der Nutzer setzt.

**Status geändert (1):** der Plan von `Draft` auf `Partially Complete`. Der Dateimarker bleibt
`_o_`, weil E2 offen ist; `_c_` verlangt alle sechzehn Schritte.

**Reconciliation Log angehängt (2):** an den Plan und an den Spec.

**Neu abgelegt (11), zehn in `issues/` dieses Circles und einer im gemeinsamen Speicher:**

| Datensatz | Gegenstand |
|---|---|
| `260813-1345_o_zwei-prosastellen-in-zulaessigkeit-rs-…` | zwei Stellen in der Dateiliste von A1 selbst; die Ursachenerklärung der Durchsicht greift für sie nicht |
| `260813-1345_o_die-aufruferzahl-an-fokus-steht-auf-fuenf-…` | fünf statt sechs, in Plan, Baum und Durchsicht |
| `260813-1345_o_keywindow-und-isequal-stehen-nicht-im-untergrenzen-abschnitt-…` | zwei neu angesprochene Methoden fehlen im Abschnitt, den CLAUDE.md als die Gegenmaßnahme führt |
| `260813-1345_o_zwei-weitere-stellen-sagen-im-baum-und-meinen-crates-…` | zwei weitere „im Baum"-Zusagen in `titelzusatz.rs`, eine davon widerlegt |
| `260813-1345_o_der-doc-kommentar-an-bundle-version-nennt-eine-sichtbarkeit-…` | `PLATZHALTER` ist `pub`, nicht `pub(crate)`; der Plan sagt es auch falsch |
| `260813-1345_o_der-baumzweig-der-abbruchmeldung-nennt-die-version-aber-nicht-die-cargo-toml` | C3.8 im Baum-Zweig nur teilweise, und die Probe fährt nur den kombinierten Fall |
| `260813-1345_o_die-eine-messung-die-der-plan-als-gegenmassnahme-nennt-ist-nicht-gefahren` | die Risikozeile zu `bundle::VERSION` bleibt eine `inference:`, ohne dass es irgendwo steht |
| `260813-1345_o_die-diagrammbefunde-am-spec-sind-nie-behoben-worden-…` | sechs Diagrammbefunde unerledigt, dazu das Stationsbild auf sechs |
| `260813-1345_o_neun-abnahmekriterien-tragen-probe-und-haben-keine` | C2.8, C2.10 und die sieben von C4 |
| `260813-1345_o_der-nachtrag-aus-a3-zaehlt-die-ausnahmeliste-mit-zwei-eintraegen-…` | Turn 2 hat einen dritten Eintrag gebracht |
| `shared/issues/260813-1345_o_fuenf-stellen-nennen-79-funktionen-und-73-mit-kommando-…` | Runde-7-Erbe, nicht aus dieser Directive |

**Angehängt, ohne Markerwechsel (11):** die sieben offenen Defekte des Circles mit dem heutigen
Stand ihrer Belege; der geschlossene mit der Bestätigung seiner vier Stellen; die drei
Durchsichten mit einer Anmerkung. Dazu der gemeinsame Defekt
`260812-2253_o_claude-md-nennt-fuer-kommando-68-varianten-…` mit der heutigen Zahl: es sind 76
und nicht 75.

**Nicht angefasst:** Code, Daten, `CLAUDE.md`, `target/KRK.app`, `Makefile`, die Speicher der
Runden 1 bis 7 ausser dem einen gemeinsamen Datensatz, `agentstate.yaml`, `orchestrator-live.md`,
`portfolio.md`, der Circle-Datensatz. Am Sitzungsprotokoll des Orchestrators ist allein der
Abschnitt `## Coherence` angehängt.
