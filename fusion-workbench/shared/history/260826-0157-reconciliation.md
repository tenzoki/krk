# Abgleich der Runde 18 — 260826-0157

**Status:** Complete
**Bereich:** `20eccd4..e5ec81a`, 26 Commits
**Baumstand beim Abgleich:** `e5ec81a`
**Domäne:** code
**Kein Circle aktiv.** Die Bedingungen gelten für die Arbeit des Plans
`shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`.

## Was geprüft wurde und wie

Gelesen wurde gegen den Baum und nicht gegen die Berichte, die die Erledigung behaupten. Die
Sitzungsdatei `shared/history/260825-1659-orchestrator-session.md` ist unvollständig und war
deshalb nicht die Grundlage; an ihre Stelle traten `fusion-workbench/agentstate.yaml` mit
ihren achtzehn erledigten Aufgaben, der Ereignisstrom `orchestrator-events.jsonl` und die
Commits selbst.

Selbst gefahren am 260826-0146: `make check` über `e5ec81a`, Ausstiegscode 0, „alle vier
gruen". Selbst gefahren am 260826-0149: `fusion-review-coverage` über den Sitzungsbereich.

| Größe | Zahl |
|---|---|
| Pläne durchgesehen / geändert | 1 / 1 |
| Planschritte gegen den Baum gelesen | 10 von 10, dazu zwei Arbeiten außerhalb des Plans |
| Entscheidungsdatensätze durchgesehen / umbenannt | 42 aktive / 7 |
| Defektdatensätze durchgesehen / umbenannt | 21 der Sitzung / 1 |
| Durchsichten geprüft / vermerkt | 6 / 2 |
| Neu abgelegte Defektdatensätze | 2 |

## Die zehn Planschritte

**Alle zehn halten.** Je Schritt steht die Fundstelle, an der der Baum die Behauptung trägt,
im Reconciliation Log des Plans und wird hier nicht wiederholt. Zusammengefasst: die zwei
Fehler sind behoben (Tab-Befehl und Zip-Zeitstempel), die drei Änderungen am Mechanismus der
Leseprofile stehen (ein Ort eine Lesung, Platzhalter in der Ortsangabe, `zeigt = "datum"`),
die Vorschau beschreibt ohne Auswahl den angezeigten Ordner, die Auslieferungsfassung führt
zwölf Profile, der Weg zu einer neuen Profildatei steht im `README.md:44-73`, und die
Kostenmessung liegt als Bericht vor.

Dazu die zwei Arbeiten außerhalb der zehn: die Berichtigung der Entscheidungszahl im Plan
(`4d6dc9a`) und der Klick auf die Tableiste (`d3da6e3`) aus der beantworteten Entscheidung.
Beide sind im Baum, beide von einer Zählprobe gehalten.

**Drei Abweichungen zwischen Plan und Baum, alle in der Benennung.** Der Plan entwirft an
`SimpleFileOptions`, der Baum baut über `FullFileOptions`; der Plan nennt die neue Aufzählung
`Juengsteform`, sie heißt `Anzeige`; die Schritte 8 und 10 sprechen von acht Profilen, es sind
zwölf geworden. Die dritte trägt in beiden Schritten schon einen Nachtrag; die zwei anderen
stehen jetzt im Reconciliation Log.

## Was gefunden wurde und nicht behauptet war

**Eine dritte Stelle derselben überholten Aussage.** Der abgelegte Datensatz
`shared/issues/260825-2230_*` hat zwei Stellen genannt, an denen der Plan noch die Zeile in der
Abschlussliste verlangt, die `acc9671` gestrichen hat. Es sind drei: dieselbe Aussage steht in
der `Resolved:`-Notiz des Defektdatensatzes
`circles/260825-0711-…/issues/260825-0838_*_jeder-gepackte-eintrag-…`. Sie hat dort einen
`Revised by:`-Vermerk auf `acc9671` bekommen. **Der Marker bleibt `_c_`**, weil der Defekt
behoben ist und allein seine Begründung umgezogen ist; die Notiz selbst ist unverändert
geblieben, wie die Konvention es für diesen Fall verlangt.

**Die Runde 18 hat keinen Circle-Datensatz.** `ls fusion-workbench/circles/*/_*_circle.md`
liefert siebzehn Datensätze, und keiner gehört zu dieser Runde. Nach der Herkunftsregel ist die
Ablage richtig — es war kein Circle aktiv, also gehört alles nach `shared/`. Die Folge ist es
nicht: `CLAUDE.md` beantwortet drei Fragen ausdrücklich über eben diesen Dateibestand, und alle
drei sind für diese Runde leer. Dasselbe trifft das Leseprofil, das diese Runde gerade gebaut
hat: seine 116 offenen Defekte enthalten keinen der rund zwanzig aus `shared/issues/`.
Abgelegt als `shared/issues/260826-0149_o_die-runde-18-hat-keinen-circle-datensatz-…`.

**`CLAUDE.md` sagt nichts über die fünf Neuerungen der Runde an der Vorschau**, und die
sechste Aussage wiegt schwerer als die fünf: ein Nutzer, der KRK schon einmal gestartet hat,
sieht von der halben Runde nichts, bis er den Handgriff aus Schritt 9 macht. Abgelegt als
`shared/issues/260826-0149_o_claude-md-sagt-nichts-ueber-die-fuenf-neuerungen-…`. **`CLAUDE.md`
ist nicht geändert worden**; der Auftrag hat es ausgeschlossen, und die Wahl gehört dem Nutzer.

## `CLAUDE.md` gegen den Baum: was überholt ist und was nicht

Der Auftrag hat vier Verdachtsstellen genannt. Nachgemessen ist **eine** überholt:

| Aussage | Befund |
|---|---|
| `zip` mit dem **einen** Merkmal `deflate-flate2` | **überholt.** `Cargo.toml:203-206` führt zwei, `deflate-flate2` und `unreserved`. Schon abgelegt als `260825-1859_o_*` |
| Die Aufrufer von `ohne_warten_oeffnen` | **nicht überholt.** `CLAUDE.md` nennt dort keine Zahl mehr, sondern das Kommando, und seine Aufzählung — drei Textwege, das Packen, das Entpacken — trifft die fünf Aufrufer im Baum genau |
| Die Aufzählung `Art` | **nicht überholt.** `CLAUDE.md` nennt keine Zahl, und die Runde 18 hat `Art` nicht angefasst; sie führt unverändert sechs Werte |
| Der Abschnitt zur Vorschau | **überholt**, und zwar an fünf Stellen. Neu abgelegt, siehe oben |

Zwei Stellen, die beim Prüfen wie Verdachtsfälle aussehen und keine sind, damit die nächste
Erhebung sie nicht ein zweites Mal aufmacht: „`krk-core` führt kein `libc`; die drei Konstanten
und die variadische `fcntl`-Deklaration stehen in `verzeichnis/sys.rs`" stimmt weiter (die drei
sind `O_NONBLOCK`, `F_GETFL`, `F_SETFL`); und „das Packen fragt ebenso und schreibt die Antwort
als Zeile in die Abschlussliste statt abzuweisen" stimmt auch nach `acc9671`, denn dieser Satz
handelt von der **Typfrage** (`zippen.rs:407`, „keine gewoehnliche Datei") und nicht vom
Zeitstempel, den `acc9671` stumm gestellt hat.

Die Zahlen, die `CLAUDE.md` ausdrücklich nennt, sind nachgezählt und halten: `Wirkungsbereich`
sieben Werte, `Bereich` fünf, `Fokus` fünf. Die zwei `#![allow(unsafe_code)]`-Ausnahmen sind
unverändert `verzeichnis/sys.rs` und `appkit/mod.rs`.

## Die Marker

**Sieben Entscheidungen von beantwortet auf umgesetzt** (`_a_` → `_i_`). Jede trägt jetzt eine
`Implemented:`-Zeile mit Commit und Fundstelle, und jede Umsetzung ist einzeln gegen den Baum
gelesen: die Ortszeit (`c0050bf`), ein Ort eine Lesung (`f097e0e`), der Platzhalter (`3cadb45`),
`zeigt` an `juengste` (`66c779c`), die Vorschau ohne Auswahl (`9322d5d`), der Klick auf die
Tableiste (`d3da6e3`) und der Weg im `README.md` (`d04e50f`). Der siebte ist die einzige
Umsetzung, die kein Code ist: die Antwort war „Handgriff jetzt, Befehl später", und die erste
Hälfte steht als Abschnitt im `README.md`. Die zweite Hälfte trägt hier keinen eigenen Marker
und ist ausdrücklich Gegenstand einer späteren Runde.

**Ein Defektdatensatz von offen auf geschlossen** (`_o_` → `_c_`):
`260825-2230_*_der-plan-der-runde-18-verlangt-in-schritt-3-…`. Der Plan trägt den verlangten
zweiten Nachtrag, der Baumstand ist nachgelesen (`zeit_uebernehmen` ohne `Steuerung`, die Probe
prüft `uebersprungen.is_empty()`), kein Code ist dafür geändert worden.

**Kein `_c_` ohne Behebung im Baum.** Die vierzehn Datensätze, die diese Sitzung von offen über
in Arbeit auf geschlossen geführt hat, sind einzeln gegen den Baum gelesen; jeder trägt seine
Behebung dort, wo seine Notiz sie behauptet. Zwei davon führen ihre Notiz als `**Resolved:**`
in Fettschrift statt als `Resolved:` und entgehen damit jeder Suche nach der einfachen Form —
das ist der offene Datensatz `260818-0710_o_forty-three-closure-notes-…`, dessen Zahl mit
dieser Sitzung gewachsen ist.

**Die rund zwanzig offenen Datensätze der Sitzung sind einzeln nachgeprüft, und keiner ist
inzwischen erledigt.** Stichprobenweise am Baum nachgemessen: der Modulkopf von `sys.rs` sagt
an zwei Stellen weiter, `ortszeit` habe keinen Rufer im Baum, während `zippen.rs:701` und
`bausteine.rs:788` sie rufen; der Satz „dieselben drei Felder wie `.fusion-setup`" steht
unverändert in der Profildatei.

**Ein Nebenbefund zur Zitierform.** Mehrere Notizen dieser Sitzung zitieren Zeilennummern, die
ein späterer Commit derselben Sitzung verschoben hat: die Probe zu C6.7 steht bei `:3048` statt
`:2967`, `genannte_orte` bei `:2930` statt `:2888`, der flight-Kommentar bei `:683` statt
`:646`. Die Aussagen stimmen, die Adressen nicht mehr. Das ist dieselbe Gestalt wie der offene
Datensatz `260823-1439_o_drei-zeilenzitate-im-quelltext-zeigen-ins-leere` und bekommt hier
keinen eigenen Datensatz, weil ein zweiter denselben Mechanismus doppeln würde.

## Die neun Schließungsbedingungen

Sechs halten, zwei sind Nutzerarbeit und ungefahren, eine steht ausdrücklich außerhalb. Die
Einzelabnahme steht im Reconciliation Log des Plans. Verschärft gegenüber der Zusage ist die
dritte: `Cargo.lock` ist nicht bloß um höchstens die Einträge aus Schritt 3 gewachsen, sondern
um **keinen** — `git diff 20eccd4..e5ec81a -- Cargo.lock` ist leer.

Der Dateimarker des Plans bleibt deshalb auf `_p_` und geht nicht auf `_c_`. Die Statuszeile
sagt jetzt aus, was gilt: gebaut, nicht abgenommen.

## Vor dem Auslieferungslauf

Der Lauf legt in Station 8 eine öffentliche Releaseseite an und lässt sich nicht zurücknehmen.
**Vier Nachweise fehlen, und der erste wiegt schwerer als die drei anderen zusammen.** Sie
stehen ausgeschrieben im Reconciliation Log des Plans; hier die Kurzfassung:

1. **Der vierteilige Handgriff zum Klick-Fokus ist nicht gefahren.** Er ist Abnahmekriterium
   von Schritt 1, und der Schritt sagt selbst, was ein Widerspruch bedeutet: die Wurzel wäre
   neu zu suchen. Die Diagnose ist an zwei Wegwerfprogrammen in Objective-C gemessen und nicht
   an KRK; der vierte Teil ist überhaupt ungemessen. `cargo test` nimmt allein ab, dass der Ruf
   im Zweig steht, nicht dass er wirkt.
2. Der Doppelklick auf ein von KRK gepacktes Archiv **im Finder** ist ungefahren.
3. Die vier neuen Zusammenfassungen sind am laufenden Bündel nicht gesehen.
4. Was die Vorschau beim Eintritt in einen Ordner vor der ersten Cursorbewegung zeigt, ist
   ungefahren; zwei Lücken derselben Regel sind schon abgelegt und offen.

**Zwei Nachweise, die nicht fehlen:** die Durchsicht ist vollständig — jeder Codecommit der
Sitzung liegt in einem Durchsichtsbereich, und die sieben Commits, die
`fusion-review-coverage` als ungedeckt meldet, sind reine Werkbankcommits ohne eine Zeile
Quelltext —, und beide Durchsichten der letzten Runde geben den Stand ausdrücklich frei.

**Zwei betriebliche Hindernisse, keine Nachweise:** `v1.2.0` liegt auf `20eccd4` und nicht auf
HEAD, der Lauf verlangt also eine neue Zahl; und `git status --porcelain --untracked-files=no`
meldet `orchestrator-events.jsonl` als geändert, woran Station 1 abbricht
(`xtask/src/release.rs:300`).

## Neu abgelegte Datensätze

- `shared/issues/260826-0149_o_die-runde-18-hat-keinen-circle-datensatz-und-jede-zaehlung-ueber-circles-uebergeht-sie.md`
- `shared/issues/260826-0149_o_claude-md-sagt-nichts-ueber-die-fuenf-neuerungen-der-runde-18-an-der-vorschau.md`

## Nichts fehlabgelegt

Kein offener Defektdatensatz dieser Sitzung ist der Sache nach eine Entscheidung, und kein
Entscheidungsdatensatz ein Defekt. Der einzige Grenzfall, `260826-0149_o_die-runde-18-hat-keinen-
circle-datensatz-…`, ist als Defekt abgelegt und trägt vier Möglichkeiten: er wird zur
Entscheidung, sobald der Nutzer eine davon wählt.
