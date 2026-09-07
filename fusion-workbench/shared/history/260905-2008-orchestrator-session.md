# Orchestrator-Sitzung — 260905-2008

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Directive:** Erst ein Abgleich, dann die Defekte beheben. Das wiederholen, bis alle Defekte bereinigt sind oder zehn Schleifen gelaufen sind. Vollständig autonom arbeiten, den Consultant zur Beratung bei Entscheidungen einsetzen, am Ende jeder Schleife den Analyst den Stand analysieren lassen.
**Mode:** issues
**Status:** In Arbeit

## Ausgangsaufnahme

| Größe | Wert |
|---|---|
| Arbeitsverzeichnis | `/Users/k1/Projects/productive/krk` |
| Git HEAD bei Sitzungsbeginn | `28c4a47` |
| Domain | `code` (171 Quelldateien, 12 Datendateien, gezählt mit `git ls-files`) |
| Rundenbudget | 12 (aus `fusion.json`) |
| Offene Defekte, gemeinsamer Speicher | 201 (`_o_`), 0 in Arbeit (`_p_`) |
| Offene Defekte, Rundenverzeichnisse | 146 (`_o_`), 0 in Arbeit (`_p_`) |
| Offene Fragen, gemeinsamer Speicher | 24 (`_o_`) |
| Offene Specs und Pläne, gemeinsam | 4 offen, 2 in Arbeit |
| Aktive Runde | keine |
| Vorgesehene Runden | keine |
| Geschlossene Runden | 13 beschränkt, 9 kohärent, 2 zurückgestellt |

Der Circle-Hinweis blieb aus, weil weder eine vorgesehene noch eine aktive Runde vorliegt.

## Einrichtung dieser Sitzung

- Checkout `6c11b1f2` erstmals im Register angemeldet, Alias `hazel-kiln`.
- Die beiden Chat-Stilprofile (`chat-voice-de.yaml`, `chat-voice-en.yaml`) waren unverändert alt und wurden durch die Fassung des Plugins ersetzt. Die zwei Schreibprofile waren bereits deckungsgleich.
- Die drei Altlasten `escalation.json`, `churn.json` und `state-drift.json` unter `.guard-state/` gelöscht; keine davon wurde von irgendetwas gelesen.
- Der Einrichtungsmarker steht neu auf Plugin-Fassung 10.23.0.

## Befunde der Einrichtung, nicht behoben

- `fusion-workbench/.cadence-anchors` und `fusion-workbench/portfolio.md` sind versioniert, obwohl beide reiner Laufzeitstand dieses Rechners sind.
- Der Checkout hat seit 413 Stunden keinen Fetch gegen `origin` gefahren; die Aussage „gleichauf mit `origin/main`" beruht auf diesem alten Stand.

## Verlauf

### Schleife 1 — 20:08 bis 23:07

**Aufbau:** ein Abgleich, dann drei Behebungsbahnen parallel nach Kiste, dann die
Dokumentation seriell, dann eine Standanalyse. Die Bahnen waren nach Kisten getrennt,
weil das die einzige Grenze ist, an der drei Agenten gleichzeitig schreiben können,
ohne sich an einer Datei zu treffen.

| Commit | Gegenstand | geschlossen |
|---|---|---|
| `3136d02` | Abgleich gegen den Baum | 10 |
| `1c29826` | `xtask` und `krk-bench` | 16 |
| `c5f6fab` | `krk-ui` | 21 |
| `8779a25` | `krk-core` | 17 von 18 |
| `84d626b` | `CLAUDE.md` und `README.md` | 7 von 11 |

**71 Datensätze geschlossen, 4 neu abgelegt, 2 neue Fragen.** Der Bestand fällt von 347
auf 280. Alle vier Kisten sind grün: Proben, `cargo clippy --all-targets -- -D warnings`,
Formatprüfung.

**Die tragende Entscheidung dieser Schleife** war, eine falsche Zahl in der Prosa nicht
durch die richtige zu ersetzen, sondern durch das Zählkommando, das sie aus dem Baum
holt. `CLAUDE.md` hat denselben Zug an sich selbst 43-mal vollzogen. Über alle fünf
Commits ist genau eine Zahl berichtigt worden, und dort ist der Übersetzer die
Verankerung. Drei neue Proben halten, was vorher Prosa war; eine davon zieht
Doc-Kommentare zeilenübergreifend zusammen und fängt damit die über einen Umbruch
verteilte Stelle in `sperre.rs`, an der fünf frühere Erhebungen gescheitert sind.

**Was die Schleife über den Bestand gelernt hat**, steht in der Standanalyse
`260905-2307-woran-die-naechste-schleife-ansetzt.md`. Drei Punkte daraus binden die
weitere Arbeit:

1. Der Bestand hat keinen gleichmäßigen Zufluss, sondern einen Tag. Am 260826 haben
   fünfzehn Durchsichtsläufe 108 Datensätze abgelegt; ein Planer hat fünf davon in einen
   Plan gefasst und einen zweiten Plan für „die 116 übrigen" angekündigt, der nie
   geschrieben wurde. Zehn Tage später war keiner der 108 geschlossen. Diese Schleife hat
   36 davon an einem Abend geschlossen. Es fehlt kein Können, es fehlte der Schritt, der
   den Ausstoß einer Durchsicht aufnimmt.
2. Vier von zehn Zeilen des Baums sind Doku, 8150 davon führen eine Zahl, 2470 in
   Modulköpfen, und nichts hält sie. Das ist die Fläche, aus der die Durchsichten
   schöpfen, und sie versiegt nicht.
3. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` bricht heute mit Exit 101
   und 157 Warnungen ab, während `make check` grün ist. Kein Abnahmekommando dieses
   Projekts fährt `cargo doc`. Das ist die einzige Gruppe des Bestands, bei der eine
   einmalige Handlung eine Fläche dauerhaft schließt.

**Der Widerspruch zwischen der Anweisung „vollständig autonom" und dem Vordergrundlauf
ist beziffert:** er betrifft 9 bis 16 der 286 Datensätze, drei bis sechs Prozent. Die
Sitzung kann die übrigen 270 anfassen. Für die 16 ist die richtige Handlung nicht, sie
doch zu schließen, sondern sie als benannte Liste zusammenzustellen.

### Nutzerentscheidung zu den Verweisen auf private Elemente

Der Nutzer hat am 260906 entschieden, nachdem `krk-core` 53 Doc-Warnungen als
Entwurfsfrage vorgelegt hatte: **die Elemente bleiben privat.** Seine Vorgabe für die
Verweise lautete „werden Fließtext" (Option 2 des Datensatzes
`260905-2336_*_wird-ein-privates-element-oeffentlich-oder-der-verweis-darauf-zu-fliesstext.md`),
ausdrücklich verbunden mit der Erlaubnis, im begründeten Fall anders zu entscheiden.

**Es ist für die 49 privaten Elemente anders entschieden worden, und der Grund ist eine
Messung.** Der Datensatz führt gegen Option 3 an, ein `#![allow(rustdoc::private_intra_doc_links)]`
an der Kistenwurzel decke künftig auch die Verweise, die wirklich falsch sind. Das stimmt
nicht. `private_intra_doc_links` und `broken_intra_doc_links` sind zwei getrennte Prüfer,
und das `allow` für den ersten lässt den zweiten unberührt. Zweimal unabhängig an einer
Wegwerfkiste gemessen, vom Orchestrator und vom ausführenden Agenten: mit dem `allow` an
der Wurzel bricht `RUSTDOCFLAGS="-D warnings" cargo doc` weiterhin an einem Verweis auf
einen nicht existierenden Namen ab.

Damit hält Option 3 den tragenden Satz der Nutzerentscheidung — die Elemente bleiben
privat — und gibt zusätzlich die Namensprüfung nicht auf, die Option 2 für 53 Stellen
dauerhaft aufgegeben hätte. Die Abweichung betrifft die Klasse und nicht einen Einzelfall;
sie ist dem Nutzer gemeldet und mit einer Zeile umkehrbar.

Die vier Meldungen an den privaten Modulen `zippen` und `entpacken` sind nach der Vorgabe
des Nutzers Fließtext geworden. Dort hilft das `allow` nachweislich nicht: rustdoc führt
ein privates Modul als unauflösbar und nicht als privat.

**Die zweite Hälfte derselben Frage ist durch Messung entschieden.** Der Datensatz nennt
als Randbedingung, ein Tor in `make check` verlängere jeden Abnahmelauf. Gemessen am warmen
Baum kostet `cargo doc --workspace --no-deps` 0,3 bis 0,8 Sekunden gegen 45 Sekunden für
`cargo clippy --workspace --all-targets` allein. `cargo doc` ist deshalb fünftes
Abnahmekommando geworden. Ohne es wäre die Räumung von 157 Warnungen beim nächsten
Umbenennen wieder zerfallen.

Der ausführende Agent hat das Tor absichtlich gebrochen und rot werden sehen, bevor er es
für erledigt erklärte. Die `note`-Zeile im Fehlertext belegt, dass `RUSTDOCFLAGS` den
`cargo`-Aufruf erreicht und nicht nur gesetzt ist — ein Tor, das grün ist, weil es nichts
prüft, wäre schlimmer als keines.

### Nutzerentscheidung zu den Datensätzen über eingefrorene Spec- und Plantexte

Drei Behebungsbahnen haben unabhängig voneinander insgesamt 33 offene Defektdatensätze als
„Aussage über einen eingefrorenen Spec- oder Plantext, deshalb nicht behebbar" eingeordnet und
auf `_o_` stehen lassen. **Der Nutzer hat diese Einordnung am 260906 zurückgewiesen, und zwar
zu Recht.**

Zwei der 33 sind im Volltext gegengelesen worden, und sie sind nicht dieselbe Art von Sache:

- `260814-1002` sagt, ein Abnahmekriterium zitiere `EDITORGRENZE` an `datei.rs:153`; sie steht
  an `:164`, weil dieselbe Runde die Datei um 232 Zeilen wachsen ließ. Die Zusage des
  Kriteriums hält — die Konstante steht wirklich genau einmal. Falsch ist allein der
  Wegweiser, und am Erzeugnis ist nichts kaputt.
- `260813-1345` sagt, neun Abnahmekriterien trügen die Kennzeichnung `(Probe)` und hätten
  keine. Alle neun halten in der Sache, aber keines wird beim nächsten Umbau rot. **Das ist
  ein echter Mangel, und er liegt im lebenden Baum**: zu beheben ist er, indem die neun Proben
  geschrieben werden. Am Spec ist dafür nichts zu ändern.

**Der Fehlschluss der Bahnen war, „ich darf den Spec einer geschlossenen Runde nicht
umschreiben" als „dieser Befund ist nicht behebbar" zu lesen.** Die Regel in
`rules/circle-records.md` bindet den Ort, an dem geschrieben werden darf, und sagt über den
Baum nichts. Wo ein Datensatz einen Mangel im Baum benennt und den eingefrorenen Text nur als
Fundstelle zitiert, ist er ganz gewöhnliche Arbeit.

**Die Antwort des Nutzers auf `260906-0203`: ja, ein Agent darf berichtigen, als Nachsatz und
nicht im Text.** Der ursprüngliche Wortlaut bleibt lesbar, die Berichtigung ist als spätere
Zutat erkennbar. Damit ist auch die Hälfte der Klasse erledigt, die aus veralteten Zeigern
besteht.

**Der Anteil, der fusion selbst gehört, ist als Bugreport an das Plugin gegangen.** Das
Markervokabular für Defekte kennt offen, in Arbeit, geschlossen und zurückgestellt und keinen
Zustand für „stimmt, und wird nie zu tun sein"; `_d_` heißt „später" und ist damit falsch.
Genau diese fehlende Vokabel hat die Bahnen in den falschen Eimer gedrückt. Dazu fehlt in
`rules/circle-records.md` der Satz, der „der eingefrorene Text wird nicht umgeschrieben" von
„der Mangel, den er dokumentiert, kann im Baum liegen" trennt.

### Eine Zahl in einem Commit dieser Sitzung stimmt nicht

Die Commitnachricht von `26dac51` nennt für die Abschlussvermerke 692 geschlossen, 633 in
Konventionsform, 18 mit verschobenem Doppelpunkt, 38 in Fettform, 9 ohne Zeile. Die zweite
Standanalyse hat an demselben Commit nachgerechnet und kommt auf **691 / 632 / 19 / 30 /
10**. Der ausliefernde Lauf selbst trägt die richtige Zahl im Code: der Modulkopf von
`xtask/src/werkbank.rs` sagt „59 in drei Schreibweisen", und 19 + 30 + 10 ist 59.

Die Richtung des Befunds ändert das nicht — die Fettform ist eine echte vierte Klasse, die
fünf frühere Erhebungen übersehen haben, und ihre elf Fälle aus der Runde 23 machen sie zu
einer laufenden und nicht bloß historischen Quelle. Eine Commitnachricht ist unveränderlich;
die Berichtigung steht deshalb hier und in der Analyse
`260906-0239-der-boden-des-bestands.md` `### 3. Die zwei nachgemessenen Zahlen: wer recht
hat`.

**Auch der Orchestrator hat in dieser Sitzung eine Zahl aus dem Kopf statt aus dem Bestand
genannt**: nach Schleife 3 im Chat 195 offene Befunde, gezählt sind es 209. Das ist derselbe
Fehler, den diese Sitzung an rund fünfzig Prosastellen behoben hat, begangen von der Stelle,
die ihn behebt.

**Der Consultant konnte nicht eingesetzt werden.** Er ist nutzergesteuert und für den
Orchestrator nicht dispatchbar. An seine Stelle sind der Analyst für die fachliche
Abwägung getreten und eine gebündelte Fragenliste an den Nutzer für die Entscheidungen,
die ihm gehören.

## Coherence

<!-- RECONCILER-OWNED -->

**Verdict:** review-needed

**Edges:**
- Artifact↔Grounding: 207 von 347 offenen Defektbehauptungen einzeln gegen den Baum gehalten, 196 davon bestehen fort, 10 geschlossen, 1 nicht entscheidbar; 143 der 338 heute offenen Defekte stammen von `coderev` oder `ontorev`. Geflaggt (Grounding at fault): die Mehrzahl der 196 sind Prosastellen im Quelltext, die eine Zahl oder eine Beschreibung führen, die der Baum nicht mehr trägt — belegt in `260905-2054-reconciliation.md` `## Was der Bestand über sich selbst sagt` und im neuen Datensatz `260905-2046_*_drei-prosastellen-in-menue-rs-nennen-85-funktionen-die-belegung-fuehrt-92.md`. Der kleinere Teil ist Artifact at fault: gemeldete Codebefunde, deren Ein-Zeilen-Korrektur nie im Baum ankam.
- Artifact↔Directive: not evaluable: `git log 28c4a47..HEAD` ist leer, die Sitzung hat noch keinen Commit erzeugt. Die Directive ist gesetzt und im Sitzungskopf ausgeschrieben; es fehlt allein die Arbeit, gegen die sie zu halten wäre.
- Grounding↔Directive: 50 der 51 offenen Fragen sind mit der Directive vereinbar, und der Grund ist strukturell: die Directive dieser Sitzung ist eine Verfahrensanweisung, die offenen Fragen betreffen das Erzeugnis. Eine steht dagegen. `260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md` hält fest, dass der Abnahmelauf KRK im Vordergrund verlangt und deshalb Nutzerarbeit ist, die kein Agent fahren kann; `CLAUDE.md` `## Was man nicht sieht, wenn man es nicht weiß` schreibt dasselbe fest. Die Directive verlangt „vollständig autonom" zu arbeiten. Jede Schleife, die einen Defekt bis zur Abnahme schließen will, läuft in diese Sperre.

**Rebalance recommendation:** revise Grounding

**Belege:** `260905-2054-reconciliation.md` (Prüfumfang, die zehn Schließungen, die drei nicht übernommenen Meldungen, die 140 ungeprüften Datensätze).

## Elf Entscheidungen am 260906-2147 beantwortet

Der Nutzer hat die 56 offenen Fragen in drei Bahnen aufbereiten lassen
(`260906-2102-neunzehn-offene-entscheidungsfragen-zur-vorlage.md`,
`260906-2115-neunzehn-offene-fragen-zur-vorlage.md`,
`260906-2100-achtzehn-offene-fragen-zur-vorlage-im-chat.md`) und elf davon in einem Zug
beantwortet. Sie zerfallen in drei Gruppen, und die Gruppe entscheidet, welchen Vermerk der
jeweilige Datensatz bekommt.

### Sechs Fragen: der Baum fährt die Antwort, der Nutzer bestätigt sie

Bei diesen sechs steht die Umsetzung seit Runden im Baum, ohne dass die Frage je beantwortet
worden wäre. Der Nutzer bestätigt das Gebaute; die Datensätze gehen deshalb auf umgesetzt.

1. **Das Hauptmenü darf die eine Gliederung umsortieren und einen Bereich umbenennen.**
   „Anwendung" steht an erster, „Fenster" an letzter Stelle, der Bereich der Textbefehle heißt
   „Bearbeiten" (`crates/krk-ui/src/belegungsmodell.rs:178-205`, gesetzt in `16c0924` und
   `a949ff1`). Belegungsansicht und Markdown-Ausgabe folgen derselben Ordnung.
2. **Teilen sich zwei Funktionen eine Tastenkombination, zeigt die Textfeld-Funktion das
   Menükürzel und der KRK-Befehl keines.** Die Regel wird bei jedem Menüaufbau gefragt
   (`crates/krk-ui/src/menuemodell.rs:236,339`); die Doppelbelegung von `cmd+a` bleibt in
   `resources/default-keymap.toml:352,1174` stehen, weil KRK jeden Tastendruck vor dem Menü
   sieht und keine Wirkung verlorengeht.
3. **Das Ankreuzfeld „Deep" gilt je Tab und überlebt die Sitzung nicht.** Der Stand sitzt im
   Ordnermodell (`crates/krk-core/src/verzeichnis/modell.rs:339`), jeder Tab hält sein eigenes,
   in die Sitzungsdatei geht er nicht. Seit dem 260826 steht er ab Werk auf ein (`modell.rs:438`).
4. **Der Git-Bereich bekommt einen eigenen Funktionsbereich und damit ein eigenes Obermenü.**
   Gebaut in `crates/krk-ui/src/belegungsmodell.rs:139,169,454`. Das ist dieselbe Regel, die das
   Projekt für den Editor schon einmal gegen dieselbe Bequemlichkeit durchgehalten hat; sie bindet
   die schreibende Git-Runde mit ihren vier Operationen.
5. **Zwei gleichzeitig laufende KRK-Fenster teilen sich die Ablage über eine Schreibsperre, das
   Sitzungsrecht bekommt nur eines.** Gebaut als `Schreibgriff`
   (`crates/krk-core/src/ablage/sperre.rs:113`) und `Sitzungsrecht` (`:163`), namentlich gehalten
   von `crates/krk-core/tests/baum.rs:454,703`. Die zweite Instanz merkt sich ihre
   Fensteraufteilung damit nicht — das ist der bewusst gezahlte Preis.
6. **Die Menüleiste trägt ein Obermenü je Bereich.** Die Gliederung steht als Aufzählung in
   `crates/krk-ui/src/belegungsmodell.rs:169` und führt seit der Git-Runde zehn Werte.

### Zwei Fragen: der Baum fährt die empfohlene Antwort, der Nutzer setzt sie in Kraft

7. **Die Filterzahl steht in der Statuszeile über dem Markierungsstand.** Platz 5 von sieben
   (`crates/krk-ui/src/appkit/statuszeile.rs:280-288`). Begründung: eine verkürzte Liste ist die
   Auskunft, ohne die der Nutzer das Fehlen eines Eintrags für einen Defekt hält. Der
   zurückgestellte Befund `260815-1047_*_…` bleibt davon unberührt — vier Ränge stehen weiter über
   dem Filterstand.
8. **`Esc` räumt den Filtertext zuletzt**, nach dem offenen Blatt und dem laufenden Vorgang
   (`crates/krk-ui/src/appkit/anwendung.rs:6253-6278`). Begründung: `Esc` heißt in KRK „halte an,
   was läuft", und ein Filtertext läuft nicht.

### Drei Fragen: der Gegenstand ist weg, es wird nichts mehr umgesetzt

Diese drei bekommen den Antwortvermerk und zusätzlich einen `Retired:`-Vermerk, weil ihr
Gegenstand entfallen ist, bevor jemand dagegen gebaut hat. Der Marker bleibt danach auf
beantwortet.

9. **Die Zustandsangabe im Kopf eines Entscheidungsdatensatzes wird nicht nachgezogen — es gibt
    sie nicht mehr.** fusion hat das Kopffeld ersatzlos gestrichen
    (`rules/fusion-workbench-conventions.md` `## Decision Record Template`); ein vorhandenes bleibt
    bewusst unangetastet, weil die auseinandergelaufenen Köpfe der Beleg für die Streichung sind.
    Damit gibt es keine zweite Quelle mehr, die dem Dateinamen widersprechen könnte.
10. **Querverweise zwischen Datensätzen schreiben den Zustandsbuchstaben als Platzhalter** — und
    zwar nicht nur in der Querverweiszeile, sondern in jedem Zitat
    (`rules/fusion-workbench-conventions.md` `## Filename Patterns`). Die Unentscheidbarkeit, an der
    die dritte Möglichkeit des Datensatzes hing, ist anders gelöst: eine Aussage *über* ein Zitat
    bekommt eine von außen erkennbare Gestalt. Die offene zweite Hälfte, die Prüfung, steht
    ebenfalls (`bin/fusion-citation-check`, `bin/fusion-citation-sweep`).
11. **Defektdatensätze über nicht mehr änderbare Spec- und Plantexte werden geschlossen, nicht
    offengehalten.** Die Entscheidung des Nutzers vom 260906 zur Nachsatzregel hat das beantwortet;
    die vierte Behebungsschleife hat 30 von 36 solcher Datensätze geschlossen, 29 davon über einen
    Nachsatz unter dem unveränderten Bestandstext (`8276170`).

**Was offen bleibt:** 45 der 56 Fragen. Sie entscheiden wirklich etwas und werden einzeln
vorgelegt.

## Fuenf weitere Entscheidungen am 260907-0703 beantwortet

Erste von neun Vorlagerunden ueber die 45 nach dem 260906-2147 verbliebenen Fragen. Der
Nutzer hat fuenf beantwortet, dreimal gegen die Empfehlung.

1. **Der Git-Befehl „verwerfen" wird zwei getrennte Befehle**: einer wirft die Aenderungen
   einer Datei weg, einer nimmt einen Commit zurueck. Das Wort traegt in Git beide
   Bedeutungen, und die Oberflaeche trennt sie, statt eine zu waehlen. Bindet die
   schreibende Git-Runde: aus den vier Operationen des Kurztexts werden damit fuenf
   Befehle, und der Versions-Schieberegler bringt die Commit-Auswahl fuer den zweiten mit.
   (Empfehlung war c, gewaehlt ist c.)
2. **Die Schaltflaeche „Ueberschreiben" wird je nach Fall verschieden beschriftet.** Das
   Verhalten bleibt, wie es ist: im Kontextmenue raeumt sie seit dem 260825 in den
   Papierkorb, beim Kopieren, Verschieben und Abwurf loescht sie endgueltig. Was faellt,
   ist der eine Wortlaut fuer zwei Wirkungen. Der Nutzer hat damit **gegen** die Empfehlung
   entschieden, die beide Wege in den Papierkorb fuehren wollte, und die Folge stand in der
   Vorlage: eine Regel, auf die man sich verlassen kann, gibt es danach nicht, der Text ist
   zu lesen. Dafuer bleibt das Ueberschreiben auf einem Datentraeger ohne Papierkorb
   moeglich, statt uebersprungen und gemeldet zu werden.
   (Empfehlung war a, gewaehlt ist c.)
3. **Die zweite Zahl der Loeschabfrage bekommt das Wort „insgesamt".** Aus „Diese 25
   Eintraege mit 25 Eintraegen in den Papierkorb raeumen?" wird eine Zeile, die sagt, dass
   die zweite Zahl den Unterbau meint. Kein Verhalten aendert sich, beide Abnahmekriterien
   halten. (Empfehlung war b, gewaehlt ist b.)
4. **Nutzersichtbarer deutscher Text traegt Umlaute, Kommentare und Bezeichner die
   Umschrift.** Die Trennlinie ist an jeder Zeichenkette entscheidbar: liest das ein Mensch
   oder der Uebersetzer. Nachzuziehen sind rund fuenfzehn Zeichenketten, vor allem aus
   Ablage und Leseprofilen, dazu die Proben, die auf ihren Wortlaut pruefen.
   (Empfehlung war a, gewaehlt ist a.)
5. **Ein Rechtsklick auf eine unmarkierte Zeile hebt die Markierung anderswo auf**, wie im
   Finder. Damit fallen Anzeige und Wirkung des Kontextmenues wieder zusammen; heute zeigt
   es auf die angeklickte Zeile und wirkt auf die Markierung. Der Nutzer hat gewaehlt, wo
   die Vorlage bewusst keine Empfehlung aussprach.
   (Keine Empfehlung, gewaehlt ist b.)

**Dazu eine neue Anforderung des Nutzers**, kein Datensatz: das Kontextmenue der Dateiliste
bekommt „Im Finder anzeigen" als fuenften Eintrag. Der vorhandene Eintrag „Im Finder
oeffnen" oeffnet den **angezeigten Ordner** (`crates/krk-ui/src/appkit/anwendung.rs:6949`);
der neue deckt die **betroffenen Eintraege** im Finder auf und ist damit eine andere
Wirkung, nicht dieselbe unter anderem Namen. Der Orchestrator hat diese Lesart angenommen
und im Chat zur Berichtigung gestellt.

**Was offen bleibt:** 40 der 56 Fragen.

## Fuenf weitere Entscheidungen am 260907-0823 beantwortet

Zweite Vorlagerunde. Der Nutzer ist allen fuenf Empfehlungen gefolgt. Keine betrifft die
Anwendung; alle fuenf betreffen, wie an ihr gearbeitet wird.

6. **Der Zustand eines Anforderungsdokuments folgt der belegten Bauarbeit, und die Abnahme
   bekommt eine eigene Kopfzeile.** Bis hierher sollte ein Zustand mit vier Werten zwei
   unabhaengige Fragen beantworten — ist es gebaut, ist es abgenommen —, und deshalb lieferte
   jede Wahl eine richtige und eine falsche Auskunft. Der Bestand zeigt den Streit: sieben
   Dokumente geschlossener Runden stehen auf offen, fuenf auf erledigt, zwei auf in Arbeit,
   obwohl an keinem jemand arbeitet. Der Ausweichzustand behauptet damit eine Taetigkeit, die
   es nicht gibt. Die teuerste der drei Moeglichkeiten, und die einzige, die nichts aufgibt.
7. **Die Vollstaendigkeit einer Liste neben einer Aufzaehlung haelt eine Probe**, die die
   Varianten aus dem Quelltext liest und gegen die Liste haelt — nicht die fremde Kiste
   `strum`. Der Baum faehrt schon so: `varianten_der_aufzaehlung` laeuft an drei Aufzaehlungen,
   waehrend fuenfzehn Listen im Baum stehen und der Uebersetzer an keiner die Namen prueft. Ein
   spaeterer Umstieg auf `strum` striche die Probe, statt sie umzubauen.
8. **Der Zehnerblock bleibt halb angeschlossen, und die falsche Erklaerung im Code wird
   richtiggestellt.** Ziffern, Plus und Minus loesen aus, Eingabetaste und Komma nicht. Der
   Zustand ist nebenbei entstanden, als der Nachschlag von Tastencodes auf gemeldete Zeichen
   umgestellt wurde; entschieden hatte ihn niemand. Der Block ganz herein ist eine eigene
   Runde, falls der Nutzer mit externer Tastatur danach fragt.
9. **Eine Probe, die unter Administratorrechten nichts messen kann, bricht mit klarem Text ab,
   statt still zu ueberspringen.** Vier Proben stellen ihren Prueffall ueber entzogene
   Dateirechte her, und unter `root` greifen die nicht. **Die Folge ist gewollt und keine
   Nebenwirkung:** ein Lauf unter `root` meldet danach vier rote Proben statt gruen zu werden.
   Die Wahl setzt voraus, dass dieser Baum nie unter `root` geprueft wird.
10. **Alle drei Pruefordner-Fassungen bekommen die Verfallwarnung des Uebersetzers**, in einem
    Durchgang ueber die drei Kisten. Wer so ein Objekt anlegt, ohne es festzuhalten, legt einen
    Namen fest und raeumt sofort wieder ab — genau der Fall, fuer den das Projekt die Marke seit
    dem 260811 setzt. Nur die Fassung im Messwerkzeug zu bemarken waere der billigste Eingriff
    und die teuerste Folge, weil die drei zeichengleich bleiben sollen.

**Was offen bleibt:** 35 der 56 Fragen.

## Fuenf weitere Entscheidungen am 260907-1210 beantwortet

Dritte Vorlagerunde. Fuenf Stellen, an denen der Code heute etwas tut, das niemand
entschieden hatte. Der Nutzer ist allen fuenf Empfehlungen gefolgt; bei der zweiten hat er
nach der Sache gefragt, bevor er sie beantwortet hat, und die Vorlage ist daraufhin ohne
Fachvokabular neu geschrieben worden.

11. **Der Doppelklick auf einen Ordner ohne Leserecht meldet kuenftig, statt wortlos in eine
    leere Liste zu wechseln.** Die Pfadeingabe meldet schon heute; das Abnahmekriterium
    verlangt eine Meldung fuer den nicht lesbaren Pfad, und diese Wahl erfuellt sie auf
    beiden Wegen. **Der Preis ist ausdruecklich mitentschieden:** ein zusaetzlicher
    Systemaufruf auf jedem Ordnereinstieg, auch im haeufigen lesbaren Fall. Daran haengen
    zwei Zeitzusagen, die seit der Runde 4 nicht mehr gemessen sind; der Aufruf gehoert in
    den naechsten Abnahmelauf.
12. **Jedes Blatt bekommt eine reine Funktion, die seinen Bauplan liefert** — der halbe
    Schritt, nicht der Typ, der die harmlose Schaltflaeche erzwingt. Damit wird jedes Blatt
    ohne AppKit pruefbar, drei Zeilen je Blatt; zwei der sieben tragen es seit dem 260818.
    Die Zusage bleibt eine Probe und wird keine Uebersetzungsbedingung. Der starke Weg lohnt
    an dem Tag, an dem ein Blatt mit ausfuehrender erster Schaltflaeche dazukommt, und heute
    gibt es keines.
13. **Die Liste der ab Werk tastenlosen Funktionen steht kuenftig an einer Stelle im
    Pruefcode, und die zweite Pruefrichtung zieht mit um.** Sie stand zweimal, ist beim
    vierten Eintrag auseinandergelaufen und hat drei Proben zugleich rot gemacht. Als einzige
    der drei Moeglichkeiten stellt diese eine Stelle her, ohne eine Zusage aufzugeben; ein
    Umzug in den ausgelieferten Code haette dem Programm eine Aufzaehlung mitgegeben, die
    niemand liest.
14. **Kommentare nennen den Rang einer Statuszeilen-Meldung nicht mehr als Zahl**, sondern
    verweisen auf die eine Stelle, an der die Ordnung steht. Die Zahlen sind viermal falsch
    geworden, als ein Rang dazukam, und kein Pruflauf liest eine Zahl in einem Kommentar. An
    der Aufzaehlung selbst ist es schon so gebaut.
15. **Der Auffrischungsaufschub beim Stapel-Umbenennen bleibt, und seine Begruendung wird
    nachgezogen.** Der Defekt, fuer den die Regel kam, ist anderswo behoben, aber der
    Aufschub faengt weiter eine echte Fehlfunktion ab: ab rund 60.000 Eintraegen zeigte die
    Liste sonst waehrend des ganzen Vorgangs nur den unsortierten Anfang. Die
    Fallunterscheidung deckt inzwischen sechs Vorgangsarten, und jede weitere ist dort
    einzuordnen.

**Was offen bleibt:** 30 der 56 Fragen, dazu die Reichweitenfrage aus der zweiten Runde.

## Fuenf weitere Entscheidungen am 260907-1301 beantwortet

Vierte Vorlagerunde, Werkzeug und Auslieferung. Der Nutzer ist allen fuenf Empfehlungen
gefolgt; bei der ersten hat er nach der Sache gefragt, bevor er sie beantwortet hat.

16. **Der Veroeffentlichungsbefehl bekommt keine Kurzform.** Er bleibt als
    `cargo xtask veroeffentlichen <zahl>` erreichbar, mit vollem Pfad zu `cargo`, waehrend
    die zwei Nachbarwege je zwei Huellen haben. Der unbequemste der drei Wege bleibt damit
    der, den man in einer Stoerung braucht — das ist der bewusst gezahlte Preis. Ein
    Makefile-Ziel laesst sich in zwei Zeilen nachziehen, sobald der Weg zum ersten Mal
    wirklich gebraucht wurde, und dann steht auch fest, ob er sich unbequem angefuehlt hat.
    Der Baum faehrt das schon, und die `README.md` begruendet es unter „Nur
    veroeffentlichen"; die Antwort macht aus der vollendeten Tatsache eine Entscheidung.
17. **Das Bauwerkzeug ruft ein nachinstalliertes Programm ueber den Suchpfad, und die Regel
    wird ausgeschrieben:** mit macOS geliefert heisst fester Pfad, nachinstalliert heisst
    Suchpfad. Nur sie trifft auf beiden Mac-Bauarten und ueber jeden Installationsweg. Der
    Datensatz nennt `gh` als erste Ausnahme; heute rufen auch `rustup`, `iconutil` und
    `cargo` ueber den Suchpfad, es sind also vier Faelle und keine Ausnahme mehr.
18. **Die Betriebsregel gegen den Datenverlust bleibt an drei Stellen ausformuliert**, jede
    mit ihrem eigenen Lesemoment: Kopf der `README.md`, Anleitung im Releasepaket, fester
    Text jeder Releaseseite. **Der Preis ist mitentschieden:** die drei Wortlaute werden
    nicht aneinander gehalten und koennen auseinanderlaufen. Die Gegenmoeglichkeit haette die
    Regel zur einen Quelle gemacht und dabei genau die Zusage gebrochen, an der sie
    erkaempft wurde — auf der Releaseseite ohne Download lesbar zu sein.
19. **Neunzehn Verlaufsdateien mit geschaetztem Zeitstempel werden nicht umbenannt, sondern
    bekommen einen Vermerk am jeweiligen Datensatz.** Der Schaden ist real — der
    Verlaufsspeicher wird nach dem Namen sortiert gelesen, und dort erscheint Schritt 2 nach
    Schritt 7 —, aber kleiner als der Preis, neunzehn Aufzeichnungen umzubenennen und jeden
    Verweis ueber ein Suchmuster nachzuziehen, das in diesem Projekt schon fuenfmal zu eng
    war. **Der Nutzer hat zugleich die vierte Moeglichkeit als eigene Frage bestellt:** die
    Ursache pruefbar zu machen, damit der naechste Fall nicht entsteht.
20. **Die Nachsatzregel deckt keine Umstellung im Bestandstext.** Der eine Datensatz ueber
    die doppelte Ueberschrift bleibt offen. Der Preis ist ein einzelner offener Datensatz
    geringer Schwere; der Gewinn ist eine Regel, die nach dem Ort entscheidbar bleibt und
    keine Auslegung braucht. Die vierte Behebungsschleife hat 30 von 36 Faellen nur deshalb
    geschlossen, weil die Regel scharf war.

**Was offen bleibt:** 25 der 56 Fragen, dazu die Reichweitenfrage aus der zweiten Runde.
