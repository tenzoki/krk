# C7 — Die gedeckelte Zählung des Unterbaums

**Status:** Complete
**Agent:** coder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Quelldatensatz:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, Schritt 7 (erster Schritt des Bündels C)
**Baumstand vor der Aufgabe:** `17d3550`
**Verification:** `make check` — exit 0

## Was zu tun war

Ein neues Modul `krk-core/src/verzeichnis/umfang.rs` mit `SCHWELLE`, `Umfang` und
`zaehlen`, das die sechste Auslöserfrage der lauten Rückfrage beantwortet: hängen mehr
als 25 Einträge an der Auswahl? Gezählt bis zu einem Deckel, gelesen über
`sys::Schwungleser`, Verknüpfungen eins und nicht verfolgt, ein Deskriptormangel
unentschieden. Kein Aufrufer in `krk-ui` — der kommt in Schritt 10.

## Die eine Entwurfsentscheidung, die der Auftrag offen ließ

**Der Abstieg läuft über einen Stapel von Pfaden und nicht über Rekursion.** Der Plan
schreibt die Schranke als „höchstens 26 Ebenen Rekursion" aus; gebaut ist die Bauform
des `durchlauf`, und damit gibt es hier gar keine Rekursion. Ein Ordner wird ganz
gelesen, seine Unterordner wandern als Pfad auf einen Stapel, und erst wenn er zu Ende
ist, fällt sein `Schwungleser`.

Der Grund steht im Modulkopf und ist der Defekt `260815-0211`: ein Abstieg, der den
Leser der übergeordneten Ebene offen hält, erzeugt seinen eigenen `EMFILE` und legt ihn
danach als Befund über einen fremden Ordner aus. **Der Deckel von 26 macht diesen Fehler
seltener und nicht falsch** — 26 gleichzeitig gehaltene Deskriptoren sind in der Tabelle
eines aus dem Finder gestarteten Bündels keine Kleinigkeit, und sie werden mit dem
Editor, der Vorschau, den Kopiervorgängen und den Lesevorgängen beider Dateilisten
geteilt. Mit dem Stapel von Pfaden ist einer gehalten, gleich wie tief der Baum ist.

Die drei Schranken stehen im Modulkopf ausgeschrieben und sind alle drei aus dem einen
Satz „jeder Abstieg kostet mindestens einen Zähler" abgeleitet: höchstens `SCHWELLE + 1`
geöffnete Verzeichnisse, höchstens `SCHWELLE + 1` vorgemerkte Pfade, genau ein offener
Verzeichnisdeskriptor. Die dritte hängt nicht am Deckel, und sie ist die tragende.

## Zwei Stellen, an denen die Umsetzung über den Auftrag hinaus entscheiden musste

**Ein Fehlschlag mitten im Lesen.** Der Plan regelt nur den Fehlschlag beim Öffnen. Die
Umsetzung zieht daraus eine Regel für beide Stellen: **jeder Fehlschlag, den
`ist_deskriptormangel` als Mangel einordnet, macht die Zählung `Unentschieden`; jeder
andere beendet das Lesen dieses einen Ordners.** Das ist die Verallgemeinerung dessen,
was der Plan für das Öffnen verlangt, und keine zweite Regel daneben.

Der Preis ist im Modulkopf benannt: ein Ordner, der sich nicht öffnen lässt, zählt eins
statt seines Inhalts, die Zahl kann also zu klein sein. Die Alternative wäre schlechter
— ein einziger fremder Unterordner machte jede Rückfrage darüber unentschieden und damit
laut, und die laute Form verlöre genau die Unterscheidungskraft, um die es bei ihr geht.
Der Plan hat diese Abwägung für den Öffnungsfehler schon getroffen; hier steht sie
begründet und nicht bloß fortgesetzt.

**`lstat(2)` an der obersten Ebene kennt keinen Mangel.** `symlink_metadata` braucht
keinen Deskriptor, ein Fehlschlag dort spricht also immer über den Pfad. Der Eintrag
zählt eins, abgestiegen wird nicht, und `ist_deskriptormangel` wird dort nicht gefragt.
Der Modulkopf sagt es, damit niemand die fehlende Abfrage für ein Versehen hält.

## Die Zusage über die Deskriptoren ist gemessen, und mit einer tieferen Grenze als in Runde 10

Die Proben der Runde 10 laufen im Kindprozess unter `ulimit -n 64`. **Diese Zahl misst
hier nichts**, und der Grund gehört zur Sache: der Deckel begrenzt die Zahl der
geöffneten Verzeichnisse ohnehin auf 26, also liefe unter 64 auch ein Abstieg durch, der
einen Deskriptor je Ebene hält. Die Probe bestätigte dann eine Bauform, die sie nicht
geprüft hat.

`tests/umfang.rs` läuft deshalb unter `ulimit -n 24`; das Kind bekommt darunter gemessene
21 Deskriptoren, also weniger als 26 und mehr als null, und es rechnet beides selbst
nach, statt es zu behaupten. Zwei Kindproben:

| Probe | Was sie misst |
|---|---|
| `ein_deskriptormangel_von_aussen_laesst_den_umfang_unentschieden` | Vorwärtsrichtung: das Kind hält alle Deskriptoren, während die Zählung läuft, und die Antwort ist `Unentschieden` und keine Zahl. Mit Gegenprobe bei freiem Vorrat |
| `die_tiefe_kette_kostet_einen_deskriptor_und_nicht_einen_je_ebene` | Rückrichtung: eine Kette aus 30 Ebenen wird unter 21 freien Deskriptoren bis zum Deckel gezählt |

**Beide sind gegen eine Mutation geprüft und nicht nur grün.** Fällt der Zweig
`Err(fehler) if ist_deskriptormangel(&fehler) => return Umfang::Unentschieden` weg, wird
die erste rot; hält die Schleife jeden `Schwungleser` in einem Vektor fest statt ihn
fallen zu lassen, wird die zweite rot und meldet „die Kette aus 30 Ebenen ist unter 21
freien Deskriptoren nicht bis zum Deckel gezählt worden". Die Mutationen sind wieder
zurückgenommen.

## Der Starter der Kindproben steht jetzt an einer Stelle

`kind_mit_wenigen_deskriptoren` stand in `tests/verzeichnis.rs` mit `ulimit -n 64` im
Rumpf. Da `tests/umfang.rs` denselben Bedarf mit einer anderen Zahl hat, ist die Funktion
nach `tests/gemeinsam/mod.rs` gewandert und heißt dort `kind_mit_deskriptorgrenze`; die
Grenze reist als erstes Argument. **Eine zweite Fassung daneben wäre der Fehler, den
dieses Verzeichnis ausdrücklich vermeidet** — es ist dieselbe Begründung, aus der der
`Pruefordner` dort steht. Die drei Aufrufer in `tests/verzeichnis.rs` übergeben
`DESKRIPTORGRENZE = 64` und verhalten sich unverändert; alle drei laufen grün.

## Angefasste Dateien

- `crates/krk-core/src/verzeichnis/umfang.rs` (neu) — `SCHWELLE`, privater `DECKEL` als
  Ausdruck über `SCHWELLE`, `Umfang`, `#[must_use] zaehlen`, dazu zwei Proben ohne
  `Pruefordner` (leere Auswahl, Deckel hängt an der Schwelle)
- `crates/krk-core/src/verzeichnis/mod.rs` — „Zwölf Module", `umfang` im Bild als zweiter
  Ast an `sys`, ein Absatz über das Modul, `pub mod umfang;`, `pub use umfang::Umfang;`
- `crates/krk-core/tests/umfang.rs` (neu) — acht Proben, davon zwei über Kindprozesse
- `crates/krk-core/tests/gemeinsam/mod.rs` — `kind_mit_deskriptorgrenze` mit
  Modulkopf-Absatz
- `crates/krk-core/tests/verzeichnis.rs` — Starter entfernt, `DESKRIPTORGRENZE = 64`
  eingeführt, drei Aufrufstellen nachgezogen

## Was ausdrücklich nicht angefasst ist

- **Kein Aufrufer in `krk-ui`.** Der kommt in Schritt 10 mit der Auslösertafel.
- **`SCHWELLE` und `zaehlen` sind nicht auf `verzeichnis`-Ebene wiederausgeführt**, nur
  der Typ `Umfang`. Ein nacktes `SCHWELLE` sagte nicht, wovon es die Schwelle ist; der
  Modulname trägt diese Auskunft und soll sie am Aufruf tragen. `mod.rs` sagt es.
- **Der Planschritt trägt weiter `[IN PROGRESS]`**; das Setzen auf `[DONE]` und der
  Commit gehören dem Orchestrator.

## Abnahme

`make check` — exit 0. Die fünf vom Plan genannten Fälle sind einzeln geprüft: flacher
Ordner unter der Schwelle (`Genau(5)` über fünf Dateien, `Genau(6)` über den Ordner
darüber), genau 25 (`Genau(SCHWELLE)`), genau 26 (`MehrAls(SCHWELLE)`), tiefe Kette
(10 Ebenen `Genau(11)`, 30 Ebenen `MehrAls`), Verknüpfung auf einen großen Baum
(`Genau(1)` an der obersten Ebene und `Genau(2)` unterhalb eines ausgewählten Ordners,
mit Gegenprobe `MehrAls` auf denselben Baum ohne Verknüpfung). Dazu drei Fälle, die der
Plan nicht nennt und die die Ränder tragen: leere Auswahl, ein ausgewählter Pfad ohne
Eintrag, und der gemessene Deskriptormangel.

## Weder Defekt noch offene Frage

Bei dieser Aufgabe ist keiner gefunden. Die Abweichung von der Formulierung des Plans
(Stapel statt Rekursion) ist keine offene Frage, sondern die Entscheidung, die der
Auftrag dem Ausführenden ausdrücklich überlassen hat; sie steht im Modulkopf begründet.
