# Abgleich 260905-2054 — der offene Defektbestand gegen den Baum

**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Baumstand:** HEAD `28c4a47`, Branch `main`, Tag `v1.7.2`, Arbeitsbaum sauber bis auf Werkbankdateien
**Domain:** code
**Sitzung:** `260905-2008-orchestrator-session.md`

---

## Was geprüft wurde und was nicht

| Größe | Zahl |
|---|---|
| Offene Defektdatensätze bei Beginn | 347 (201 gemeinsam, 146 in Runden) |
| Davon einzeln gegen den Baum gehalten | 207 |
| **Ungeprüft geblieben** | **140** |
| Offene Fragen geprüft | 51 (24 gemeinsam, 27 in Runden), also alle |
| Specs und Pläne im gemeinsamen Speicher geprüft | 6, also alle |

**Die 140 ungeprüften Datensätze sind ausdrücklich ungeprüft und nicht entwarnt.** Sie liegen
überwiegend in den Rundenverzeichnissen und betreffen Aussagen über Specs, Pläne und
Abnahmekriterien der jeweiligen Runde. Diese Klasse ist nach der Ortsregel eingefroren: ein
Spec wird nicht nachträglich berichtigt, also wird ein Befund an ihm auch nicht nebenbei wahr.
Der Ertrag einer Prüfung ist dort am geringsten, und deshalb ist sie zurückgestellt worden, nicht
weil die Datensätze weniger gälten.

## Ergebnis

| Urteil | Zahl |
|---|---|
| Geschlossen, Marker auf `_c_` gezogen | 10 |
| Besteht fort, Marker unangetastet | 196 |
| Nicht entscheidbar | 1 |
| Neu abgelegt | 1 |

Bei den offenen Fragen ist **eine** Antwort anderswo auf der Platte gefunden und als
`Answer located:` vermerkt worden. **Kein Marker einer Frage ist bewegt worden**; der Übergang
`_o_` → `_a_` gehört dem Nutzer.

## Die zehn geschlossenen Defekte

| Datensatz | Warum geschlossen |
|---|---|
| `260828-1046_*_claude-md-nennt-sieben-werte-fuer-wirkungsbereich-der-baum-traegt-acht.md` | `CLAUDE.md:87` nennt keine Zahl mehr, sondern das `awk`-Zählkommando |
| `260829-1217_*_claude-md-nennt-die-zaehlprobe-mit-ihrem-alten-namen-und-den-vergleich-als-teilzeichenfolge.md` | `CLAUDE.md:151` führt den heutigen Probennamen und den Musterabgleich |
| `260824-1852_*_zwei-aussagen-in-claude-md-sind-mit-dieser-runde-falsch-geworden-und-kein-datensatz-traegt-sie.md` | beide Zahlen aus `CLAUDE.md` entfernt, ersetzt durch Zeiger auf `grep` und auf `Datei::ALLE` |
| `260810-1907_*_die-durchsicht-von-turn-2-hat-kein-durchsichtsdokument-hinterlassen.md` | Ursache weg: `rules/review-contract.md` bindet den Ablageort am Setup des Prüfers |
| `260810-1945_*_der-orchestrator-hat-in-drei-turns-keine-aufgabenereignisse-emittiert.md` | 83 `task_start` und 82 `task_done` in den letzten 400 Zeilen des Ereignisprotokolls |
| `260824-1745_*_ein-commit-des-orchestrators-nimmt-die-git-mv-umbenennungen-eines-laufenden-agenten-mit.md` | der Commit beginnt mit `git reset -q` und stagt nur die eigene Pfadliste |
| `260812-2253_*_zwei-verweise-in-lebenden-dokumenten-der-runde-6-tragen-einen-gestorbenen-marker.md` | beide Zeigerstellen stehen in Sternform |
| `260814-1002_*_zwei-lebende-zeiger-auf-den-plan-sind-mit-seiner-schliessung-gestorben.md` | Circle-Datensatz in Sternform, das zweite Feld gibt es in `agentstate.yaml` nicht mehr |
| `260831-1417_*_die-runde-23-schliesst-ohne-durchsicht-und-vierundzwanzig-commits-sind-ungedeckt.md` | `260831-1444-coderev-git-bereich-runde-23.md` liegt vor, Circle auf `_b_` geschlossen |
| `260814-0628_*_diagrammbefunde-haben-keinen-eigentuemer-und-bleiben-deshalb-liegen.md` | der beratende Prüfagent existiert nicht mehr; die Diagrammprüfung ist Selbstcheck von Shaper und Planner |

Sieben der zehn sind keine Codeänderung, sondern eine gefallene Ursache: die Prosastelle ist
entfernt, das Werkzeug neu gefasst, der Zeiger nachgezogen. Das ist genau die Klasse, die die
Vermutung des Auftrags beschrieben hat.

## Drei gemeldete Schließungen, die ich nicht eingetragen habe

Ein Prüflauf hat sie als behoben gemeldet; die Nachprüfung hält das nicht.

1. `260814-1002_*_die-erhebung-zu-vier-ablagedateien-nennt-neun-stellen-die-suche-liefert-mehr.md`
   — die sieben von ihm benannten Stellen sind zwar zahlenfrei geworden, seine Abnahmebedingung
   ist aber eine Ergänzung am Nachbardatensatz `260814-0912_*_…`, und der trägt sie nicht:
   weder eine erweiterte Tabelle noch den Umfangsvermerk. Das Suchmuster liefert heute 14 Stellen
   gegen die neun, die der Nachbardatensatz führt. Der Befund besteht mit gealtertem Beleg fort.
2. `260811-2157_*_fuenf-commits-stehen-hinter-dem-letzten-turn-ende-ohne-eigene-turn-grenze.md`
   — der vorgelegte Beleg zeigt selbst, dass Commits weiter nach `turn_end` und `session_end`
   fallen. Sichtbar sind sie heute, in einer Rundengrenze liegen sie deshalb nicht.
3. `260814-1612_*_eine-verknuepfung-auf-einen-ordner-laesst-sich-nicht-betreten.md`
   — im Code behoben und im Kern mit neun Proben belegt, aber der Datensatz schreibt selbst
   fest: „Geschlossen wird auf Plausibilität nicht: der Marker bleibt `_o_`, bis der Klick
   gemeldet ist." Der Klicktest am laufenden Bündel ist Nutzerarbeit und in keiner Sitzung
   vermerkt. Ein Abgleich vom 260819-1440 hat schon einmal so entschieden.

## Der eine nicht entscheidbare Fall

`260820-2235_*_der-gemessene-start-laedt-die-lesezeichen-nicht-und-die-leiste-schweigt-mit-falscher-begruendung.md`
— die vier benannten Rückkehrstellen liegen im heutigen `anwendung.rs` nicht mehr an ihren
Zeilen, und der Datensatz macht die Behebung selbst von einer Nutzerentscheidung abhängig
(„soll der Messweg mitfahren?"), die nicht getroffen ist. Ohne sie ist am Code nicht zu
entscheiden, ob der Zustand ein Defekt oder eine hingenommene Lage ist.

## Neu abgelegt

`260905-2046_*_drei-prosastellen-in-menue-rs-nennen-85-funktionen-die-belegung-fuehrt-92.md`
— beim Nachprüfen von `260815-1448_*_…` gefunden. Acht der neun dort genannten Stellen sind
inzwischen zahlenfrei; die neunte in `menue.rs` ist geblieben und hat sich neu verzählt: sie
sagt „85 Funktionen" und „79 der 85", die Belegung führt 92 Einträge, davon 6 ohne `Kommando`.

## Antwort anderswo gefunden — braucht das Urteil des Nutzers

`260814-1955_*_sechs-beantwortete-entscheidungsdatensaetze-tragen-im-kopf-weiter-status-open.md`
trägt jetzt die Zeile `Answer located:`. Das Kopffeld `**Status:**` ist im Rahmenwerk
ersatzlos abgeschafft (`260826-0818-curator-run.md` `### K02`), ein vorhandenes bleibt bewusst
stehen; die Frage nach dem Abgleich von Kopffeld und Marker hat damit keinen Gegenstand mehr.
Der Marker steht unverändert auf `_o_`.

**Acht weitere Fragen sind im Code vorweggenommen, und keine davon habe ich so vermerkt.** In
jedem Fall hat die Runde die eigene Empfehlung gebaut, und der Datensatz sagt es selbst: „Die
Runde fährt bis zu einer Antwort auf Möglichkeit 1." Ein `Answer located:` dort behauptete eine
Antwort, die der Datensatz ausdrücklich nicht für die Antwort hält. Betroffen sind
`260813-0053_*_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`,
`260813-0053_*_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md`,
`260813-0159_*_darf-das-menue-die-eine-gliederung-umsortieren-und-umbenennen.md`,
`260813-0430_*_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md`,
`260814-1552_*_wo-steht-die-filterzahl-in-der-rangfolge-der-einen-statuszeile.md`,
`260814-1830_*_an-welcher-stelle-der-bedeutungen-von-esc-steht-der-filtertext.md`,
`260814-1830_*_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md`,
`260827-1322_*_faellt-das-default-profil-auch-im-messmodus-an-und-was-misst-l7-danach.md` und
`260830-1317_*_bekommt-der-git-bereich-einen-eigenen-funktionsbereich-und-damit-ein-zehntes-obermenue.md`.
Für alle gilt dasselbe: gebaut ist eine Möglichkeit, bestätigt ist keine.

## Specs und Pläne

Alle sechs im gemeinsamen Speicher geprüft, **keiner geändert**. Vier stehen auf `_o_`, zwei auf
`_p_`, und jeder trägt in seiner Statuszeile den Grund dafür ausgeschrieben: der Marker wird
gehalten, solange
`260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md` offen ist. Das ist
kein Nachzugsrückstand, sondern eine Folge einer unbeantworteten Frage.

Die Schrittmarkierungen sind stichprobenweise gegen den Baum gehalten. In
`260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md` stehen alle zehn Schritte auf `[DONE]`;
nachgelesen habe ich Schritt 2 (`ortszeit` in `crates/krk-core/src/verzeichnis/sys.rs`) und
Schritt 5 (Platzhalter in der Ortsangabe, `crates/krk-core/src/leseprofil/datei.rs:74-78`), beide
halten. Schritt 8 (acht Profile in der Auslieferungsfassung) habe ich nicht unabhängig
nachgezählt; dafür steht der Abgleich vom 260826-1024 im Plan selbst.

## Durchsichten

Keine Anmerkung eingetragen. Von den 143 offenen Defekten, die `coderev` oder `ontorev`
abgelegt haben, hat die Prüfung keinen als erledigt bestätigt, der eine Anmerkung an der
Durchsichtsdatei verdiente.

## Wie geprüft worden ist

Der Bestand ist nach Ertrag in Gruppen geteilt und parallel gegen den Baum gehalten worden, je
Gruppe ein Prüflauf mit Leserecht und ohne Schreibrecht:

| Gruppe | Gegenstand | Zahl | Geschlossen |
|---|---|---|---|
| A | Aussagen über `CLAUDE.md`, `README.md`, Auslieferungsdokumentation | 12 | 3 |
| B | die Marke `#[must_use]` | 15 | 0 |
| C | Zahl der Ablagedateien, Ablageverhalten, `xtask` | 20 | 0 |
| D | Zählaussagen in Modulköpfen und Kommentardateien | 32 | 0 |
| E | Werkbank selbst: tote Zeiger, Zeitstempel, Zitierform | 39 | 7 |
| F | Proben, Prüfordner, Messstrecke | 34 | 0 |
| G | Verhalten an Oberfläche und Kern | 36 | 0 |
| — | vom Abgleich selbst geprüft | 19 | 0 |

Jede gemeldete Schließung ist vor dem Eintrag einzeln nachgelesen worden; drei davon habe ich
nicht übernommen (siehe oben).

## Was der Bestand über sich selbst sagt

Zwei Klassen wachsen schneller, als sie abgetragen werden, und beide sind an frischen Zählungen
belegt, nicht aus den Datensätzen übernommen:

- Abschlussvermerke in einer Form, die keine Suche nach `Resolved:` findet: 58 von 593
  geschlossenen Defekten, bei Ablage des Befunds 43 von 444.
- Entscheidungsdatensätze mit der leeren Vorlagenzeile vor der gefüllten: heute 69, bei Ablage 30.

Die Marke `#[must_use]` steht heute an 254 Stellen unter `crates/` und `xtask/`, und **keine
einzige** der fünfzehn namentlich benannten Lücken ist dabei. Der Zuwachs seit der Erhebung
stammt aus neuen Funktionen, nicht aus dem Nachziehen der alten.

Von den 34 Befunden an Proben und Messstrecke ist keiner behoben. Ein Teil davon liegt in
Dateien, die seit dem 260826 nicht mehr angefasst wurden; der andere Teil liegt in genau den
Dateien, die eine Behebungsrunde damals berührt hat, und sind die Nebenbefunde, die jene Runde
selbst ungelöst hinterlassen hat.
