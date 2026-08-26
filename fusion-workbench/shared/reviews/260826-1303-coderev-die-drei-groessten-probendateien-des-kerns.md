# Vollbaum-Durchsicht R5: die drei größten Probendateien des Kerns

**Reviewed-range:** `4a57028..4a57028`
**Not-opened:** none

Kein Commit-Bereich: Vollbaum-Durchsicht von
`crates/krk-core/tests/{leseprofil,ablage,verzeichnis}.rs` am Baumstand `4a57028`. Die zwei
Pflichtfelder darüber stehen in der Form, die `bin/fusion-review-coverage` liest — die
Aufgabe hat den Bereich in Prosa vorgegeben, und in Prosa geschrieben wird die Datei dort als
`UNUSABLE` verworfen. `none` heißt hier: alle drei beauftragten Dateien wurden ganz geöffnet.

**Sender:** coderev
**Gelesen:** 3 von 3 beauftragten Dateien, 10.679 Zeilen; daneben
`crates/krk-core/tests/gemeinsam/mod.rs` (351 Zeilen) als die eine Prüfordner- und
Kindprozessfassung des Kerns, ohne die drei der Befunde nicht zu beurteilen sind.
**Baumstand:** `4a57028`. Der Quelltext steht seit `004ff72` unverändert; die zwei
Commits dazwischen tragen nur Werkbankdateien.
**Nachgemessen:** das Verhalten von `libtest` bei einem Filter, der nichts trifft, und bei
einer Probe ohne `#[ignore]` unter `--ignored`. Gefahren an einem eigens angelegten
Kleinstprojekt mit eigenem `CARGO_TARGET_DIR` außerhalb des Projektbaums; der Projektbaum
wurde nicht gebaut und nicht angefasst, `~/Library/Application Support/KRK/` und
`~/Library/Caches/krk-messplatz` ebenso wenig.

## Summary

Die drei Dateien sind in gutem Zustand, und ihre Doc-Kommentare sind Teil des Beweises: an
etlichen Stellen steht ausgeschrieben, **was eine Probe nicht entscheidet**, und zweimal
steht daneben, warum eine Gegenprobe nötig war, damit die erste überhaupt etwas hält. Diese
Sorgfalt ist die Ausnahme und nicht die Regel im Feld. Acht Defekte sind trotzdem gefiltert,
davon einer schwer: die vier Elternproben, die KRKs Deskriptorzusagen tragen, prüfen allein
den Rückgabewert ihres Kindes, und `libtest` gibt eine Null zurück, wenn es gar keine Probe
gefahren hat. Drei weitere Befunde sind Proben, die weniger halten als ihr Name sagt; einer
ist eine latente Flatterstelle, die derselbe Helfer an der Nachbarstelle absichert. Kein
Befund ist ein Auslieferungshindernis, und keiner betrifft das Verhalten der Anwendung zur
Laufzeit.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 1 (an einen fremden Datensatz übergeben, siehe A1) |
| Medium | 4 |
| Low | 3 |

**Sieben eigene Datensätze und ein Nachtrag an einem fremden.** A1 war beim Filtern schon
eine Minute zuvor von einer parallel laufenden Durchsicht erhoben; mein Datensatz dazu ist
gelöscht, sein einziger eigener Beitrag steht als Nachtrag an jenem.

## Befunde nach Thema

### A. Proben, deren grüner Ausgang nichts bedeutet

#### A1 — Vier Kindproben bleiben grün, wenn das Kind null Proben gefahren ist (**High**)

`crates/krk-core/tests/verzeichnis.rs:2578`, `:2790`, `:2890`;
`crates/krk-core/tests/leseprofil.rs:3494`. Jede prüft allein
`ergebnis.status.success()`.

Zwei Wege führen zu einer Null ohne einen gelaufenen Rumpf, beide am 260826-1259 gemessen:
ein Name, den `--exact` nicht trifft, und ein verlorenes `#[ignore]`, das `--ignored`
wegfiltert. In beiden Fällen meldet `libtest` `0 passed; … 1 filtered out` und beendet sich
mit 0.

Das Gewicht kommt aus dem, was daran hängt. `CLAUDE.md` führt unter „Was man nicht sieht"
drei Zusagen als **gemessen**, und diese vier Elternproben sind ihre einzigen Träger: der
eine Verzeichnisdeskriptor im Durchlauf, der Deskriptormangel von außen, der ein Ordner-
beziehungsweise Dateiurteil unentschieden lässt, und C6.9 der Leseprofile. Die Kinder selbst
messen sorgfältig, ob `ulimit` überhaupt gegriffen hat (`verzeichnis.rs:2819-2828`,
`leseprofil.rs:3546-3555`) — diese Sorgfalt verpufft, wenn der Rumpf nie erreicht wird.

Der Befund ist für `tests/zeit.rs` schon aufgeschrieben
(`shared/issues/260825-2127_*_die-kindproben-in-tests-zeit-rs-bleiben-gruen-wenn-ihr-name-nicht-trifft.md`,
dort als „gering" eingestuft, weil es zwei Proben betraf). Er ist nicht auf jene Datei
beschränkt: die sechs Fundstellen in `verzeichnis.rs`, `leseprofil.rs` und `umfang.rs` hängen
an **einem** Starter, `gemeinsam::kind_mit_deskriptorgrenze` (`tests/gemeinsam/mod.rs:334-351`),
und eine Zeile dort schließt alle sechs.

**Eine parallel laufende Durchsicht hat denselben Befund eine Minute früher gefiltert.** Ihr
Datensatz zählt alle sechs Rufer auf und nennt als zweiten stillen Weg die fehlende
Umgebungsvariable. Mein eigener Datensatz ist deshalb gelöscht und nicht gespeichert; was er
hinzufügte, steht als Nachtrag in jenem: der dritte stille Weg, ein verlorenes `#[ignore]`,
das `--ignored` wegfiltert, und die Begründung für die höhere Schwere. Der dritte Weg ist der
einzige, den weder ein Blick auf den Namen noch einer auf die Umgebungsvariable fängt.

Datensatz: `shared/issues/260826-1302_*_sechs-elternproben-am-gemeinsamen-kindstarter-bleiben-gruen-wenn-der-kindname-nicht-trifft.md` (fremd gefiltert, um meinen Nachtrag ergänzt)

#### A2 — Die Generationsprobe filtert über eine Schleifeninvariante (**Medium**)

`crates/krk-core/tests/verzeichnis.rs:619-634`. Das Prädikat im `filter` sieht seinen
Eintrag nicht an und ruft `!modell.gehoert_dazu(3)`, was neun Zeilen darüber schon
zugesichert ist und sich nicht mehr ändern kann — `modell` ist nicht `mut`. `veraltet` ist
damit die Gesamtzahl der gelesenen Einträge, und die letzte Zusicherung sagt: der Leser hat
zehn Einträge geliefert.

Verworfen wird in diesem Baum ohnehin nicht vom Modell: `Ordnermodell::anhaengen`
(`src/verzeichnis/modell.rs:467`) fragt nach keiner Generation, der Rufer stellt die Frage.
Kein Ort dieser Datei fährt den verwerfenden Zweig; auch
`ein_grosser_ordner_laeuft_stapelweise_ins_modell` (`:654`) fährt ihn mit passender
Generation.

Datensatz: `shared/issues/260826-1303_*_die-generationsprobe-filtert-ueber-eine-schleifeninvariante-und-misst-kein-verwerfen.md`

#### A3 — Die Zusicherung zum Zeichen-Zurück hält bei Gleichstand (**Medium**)

`crates/krk-core/tests/verzeichnis.rs:1106-1109`, `assert!(modell.zeilenzahl() >= eng)`.
Am `filterordner` nachgerechnet stehen bei `aaa`, `aa` und `a` dieselben drei Zeilen, die
Ungleichung läuft also `3 >= 3`. Vor der Deep-Vorgabe waren es vier statt drei; die
Zusicherung war in beiden Ständen eine über Gleichstand und nie eine über das Wachsen, das
der Name behauptet. Was die Probe wirklich hält, steht an ihrem Ende (`:1113-1121`).

Datensatz: `shared/issues/260826-1303_*_die-probe-zum-zeichen-zurueck-misst-kein-wachsen-ihre-zusicherung-haelt-bei-gleichstand.md`

### B. Der Aufbauhelfer, der den gemessenen Zweig mitbestimmt

#### B1 — `geladenes_modell` ist der dritte Helfer, den die Deep-Vorgabe getroffen hat (**Medium**)

`shared/issues/260826-1221_*_die-tiefe-suche-ab-werk-…` nennt zwei Aufbauhelfer von
`tests/verzeichnis.rs`, die den Zustand **vor** der Vorgabenänderung herstellen: `gefiltert`
(`:708-713`) und `handmodell` (`:1220-1229`), beide mit `tief_setzen(false)`. Der dritte
Helfer, `geladenes_modell` (`:359-364`), setzt den Schalter gar nicht — und ist damit von
der anderen Seite getroffen.

`ohne_filtertext_aendert_die_tiefe_suche_nichts` (`:837-855`) ist die Probe, an der es sich
zeigt: sie nimmt ihre Vergleichsaufnahme unter `tief == true`, ruft danach
`tief_setzen(true)` und hält `assert!(modell.tief())`. Der Setzer legt nichts um, die
Zusicherung liest die Vorgabe aus `Ordnermodell::neu` zurück, und der Vergleich
`namen == vorher` vergleicht zwei Aufnahmen desselben unveränderten Standes. Der Übergang
aus → ein, den ihr Doc-Kommentar als Gegenstand nennt, findet nicht mehr statt.

Die zwei anderen Proben mit demselben Helfer und stehendem Filtertext (`:1011`, `:1034`)
sind von der Vorgabe sachlich unberührt; die dritte ist A3.

Datensatz: `shared/issues/260826-1303_*_die-probe-zur-tiefen-suche-ohne-filtertext-legt-seit-der-deep-vorgabe-nichts-mehr-um.md`

### C. Flatterhaftigkeit

#### C1 — Drei Entscheidungsdatensätze ohne gesetzte Änderungszeit, und der Zweitschlüssel kehrt die Antwort um (**Medium**)

`crates/krk-core/tests/leseprofil.rs:1265-1269` sichert zu, dass die jüngste Datei unter
`decisions/` `Dritte Frage?` heißt. `werkbankgestalt` schreibt die drei Datensätze
nacheinander, ohne `geaendert_setzen` (`:1067-1074`) — während derselbe Helfer zwölf Zeilen
tiefer jede Verlaufsdatei durch genau diesen Aufruf schickt (`:1099-1100`) und sein
Doc-Kommentar (`:1114-1119`) die Gefahr im Klartext benennt.

Der Zweitschlüssel in `src/leseprofil/bausteine.rs:639-644` bricht den Gleichstand
**aufsteigend nach Namen**. Tragen die drei Dateien denselben Zeitpunkt, gewinnt also
`erste-frage.md`, und die erwartete Antwort kehrt sich um. APFS führt Nanosekunden, der Baum
ist heute grün; der Befund ist die Ungleichbehandlung an der einen Stelle, die als einzige
die Sortierung selbst zusagt.

Datensatz: `shared/issues/260826-1303_*_die-juengsten-entscheidungsdatensaetze-tragen-keine-gesetzte-aenderungszeit-und-der-zweitschluessel-kehrt-die-antwort-um.md`

### D. Deckung und Prosa

#### D1 — Der Rundlauf von `readers.toml` geht nicht durch den Ladeweg (**Low**)

`crates/krk-core/tests/ablage.rs:489-494`. Für vier der fünf TOML-Dateien fährt
`alle_toml_dateien_ueberstehen_schreiben_und_wiedereinlesen` einen echten Rundlauf; für
`readers.toml` endet er bei einem rohen `fs::read_to_string` der Probe. Zwischen dem
Schreiben und dem Lesen ruft niemand einen KRK-Ladeweg an, also kann die Zusicherung nur
fallen, wenn `atomar::schreiben` kaputt ist — was `:2355` schon hält.

Die Begründung im Doc-Kommentar (`:430-432`, „wer ihren Inhalt auswertet, kommt mit einem
späteren Schritt") widerspricht derselben Datei bei `:125-128` („Über `Zugang::laden` gehen
seit Schritt 8 der Runde 16 alle fünf"), und der Helfer `geladene_leseprofile` steht bei
`:111` bereit.

Datensatz: `shared/issues/260826-1303_*_der-rundlauf-von-readers-toml-geht-nicht-durch-den-ladeweg-und-seine-begruendung-ist-ueberholt.md`

#### D2 — Ein Platzhalter in einer Meldung, die nicht formatiert (**Low**)

`crates/krk-core/tests/leseprofil.rs:665`:
`.expect_err("der Wert {wert:?} kommt durch, obwohl es ihn nicht gibt")`. `expect_err` nimmt
eine Zeichenkette und keine Formatvorlage; wer die Probe fallen sieht, erfährt nicht, welcher
der drei Durchgänge es war. Einzige Stelle dieser Art in den drei Dateien, per `grep`
nachgezählt.

Datensatz: `shared/issues/260826-1303_*_ein-platzhalter-steht-in-einer-meldung-die-nicht-formatiert.md`

#### D3 — Vier gegen fünf Lagen in einem Doc-Kommentar (**Low**)

`crates/krk-core/tests/ablage.rs:1354` sagt „Jede der vier Lagen", während der zweite Absatz
desselben Kommentars, der Name der Probe und ihr Rumpf fünf führen. `Gekuerzt` ist am 260814
dazugekommen.

Datensatz: `shared/issues/260826-1303_*_der-doc-kommentar-der-meldungsprobe-nennt-vier-lagen-und-die-probe-fuehrt-fuenf.md`

## Was die Aufgabe zur Prüfung vorgelegt hat

**Die Prüfordner-Regel hält.** Alle drei Dateien ziehen `mod gemeinsam;` ein und nehmen
`gemeinsam::Pruefordner`; eine vierte Fassung steht in keiner von ihnen. Der Ordner liegt
unter `std::env::temp_dir()` (`gemeinsam/mod.rs:71`) und trägt Zweck, Prozesskennung und
Laufnummer.

**Keine der drei Dateien schreibt auf den Messplatz oder ins echte
Benutzerverzeichnis.** Nachgezählt per `grep` über `temp_dir`, `krk-messplatz`, `Caches`,
`/tmp`, `home_dir` und `benutzerverzeichnis`: die einzige Berührung mit dem echten
Benutzerverzeichnis ist `ablage.rs:249-250`, und sie **liest** einen Pfad, ohne etwas
anzulegen — der Modulkopf nennt diese Ausnahme ausdrücklich (`ablage.rs:5-7`).

**Der Defekt `260810-1925` gilt nach dem Baum nicht mehr, und sein Marker steht auf `_o_`.**
Er ist außerhalb der drei beauftragten Dateien und wurde deshalb nur nachgeschlagen, nicht
durchgesehen; beide Proben, die er nennt, gehen heute über einen `Wegwerfordner` statt über
das echte Temporärverzeichnis: `der_messplan_traegt_die_pruefsitzung_…`
(`crates/krk-bench/src/messen.rs:2705`) ruft `plan_in_verzeichnis_schreiben`, und
`ein_neuer_waechter_raeumt_fremde_plaene_ab_…` (`:2755`) ruft
`Messplanwaechter::in_verzeichnis`. Der Doc-Kommentar bei `:2691-2698` beschreibt die
Behebung. **Umbenannt habe ich den Datensatz nicht**: er liegt außerhalb meines Auftrags, ich
habe nicht jeden Rufer von `plan_schreiben` geprüft, und zwei weitere Prüfer arbeiten parallel
an derselben Werkbank. Eine Nachprüfung durch den Reconciler ist der richtige Weg.

**Die Prosa-Zusagen, nach denen die Aufgabe fragt, sind alle gehalten:**

| Zusage | Wer sie hält |
|---|---|
| Die sieben Ablagedateien in `Datei::ALLE` | `ablage.rs:260-284`, Namen einzeln ausgeschrieben |
| Die drei Filterregeln und ihre Ruferzahlen | `verzeichnis.rs:3244` (`die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`), mit namentlicher Liste statt einer Zahl |
| Der eine Verzeichnisdeskriptor im Durchlauf | `verzeichnis.rs:2774` — gehalten, aber siehe A1 |
| Die Deskriptormangel-Trennung `EMFILE`/`ENFILE` gegen andere Fehler | `src/verzeichnis/sys.rs:1342-1365`, außerhalb dieser Dateien |
| Die vier Zahlen der Leseprofile | `HOECHSTENS_LESELAEUFE` → `leseprofil.rs:2673`; `HOECHSTENS_OEFFNUNGEN` → `:2760`; `HOECHSTENS_EINTRAEGE` → `:2017`; `HOECHSTENS_BYTES` → `:2833`; dazu `HOECHSTENS_JUENGSTE` → `:504` |

## Querschnittliches

**Der Unterschied zwischen `tests/ablage.rs` und den anderen zwei ist eine Bauart und keine
Zufälligkeit.** Jede der fünf Kindproben in `ablage.rs` liest nach dem Rückgabewert noch eine
Spur, die es ohne einen gelaufenen Rumpf nicht gäbe — `recht.txt`, `sperre.txt`, die
Nachbardatei, das Signal `SIGABRT`, die Zahl der Lesezeichen. Genau diese zweite Hälfte fehlt
den sechs Kindproben am gemeinsamen Starter. Wer A1 behebt, sollte die Bauart von `ablage.rs`
als das nehmen, was sie ist: die vollständige Fassung derselben Form.

**Vier der acht Befunde sind Varianten eines Musters: die Zusicherung ist schwächer als der
Name.** A2 filtert über eine Invariante, A3 nimmt eine Ungleichung, wo eine Gleichheit
gemeint ist, B1 setzt einen Wert, der schon steht, D1 liest zurück, was die Probe selbst
geschrieben hat. Keines davon ist ein Fehler im Code, und keines fällt bei einem grünen Lauf
auf. Auffallen können sie nur bei genau dem, was diese Runde gefahren hat — beim Lesen der
Probe gegen ihren eigenen Namen.

**Die Doc-Kommentare dieser drei Dateien sind ein Werkzeug und kein Beiwerk.** An mindestens
acht Stellen steht ausgeschrieben, **was eine Probe nicht entscheidet**
(`verzeichnis.rs:2124-2135`, `:2500-2507`, `:3135-3141`, `:3197-3199`, `:3314-3315`;
`leseprofil.rs:1279-1284`, `:2007-2015`, `:3354-3356`), und zweimal steht daneben, warum eine
Gegenprobe nötig war, damit die erste überhaupt etwas hält (`verzeichnis.rs:2117-2123`,
`leseprofil.rs:1808-1810`). Drei der acht Befunde dieser Durchsicht sind nur deshalb schnell
zu belegen gewesen: C1 steht in einem Doc-Kommentar bei der Nachbarstelle, D1 widerspricht
seiner eigenen Datei, D3 widerspricht seinem eigenen zweiten Absatz. Wer diese Gewohnheit
abschafft, verliert den einzigen Weg, der eine Zusicherung ohne Ausführung prüfbar macht.

## Empfohlene Reihenfolge

1. **A1** — eine Zeile in `tests/gemeinsam/mod.rs`, und sechs Elternproben messen wieder,
   was sie zusagen. Kein Auslieferungshindernis, aber das Einzige, was heute eine Zusage von
   `CLAUDE.md` ungedeckt lässt. Der Datensatz liegt bei der Nachbardurchsicht; die Zeile
   sollte auf `"1 passed"` in der Ausgabe prüfen und nicht auf den Namen, sonst bleibt der
   dritte Weg offen.
2. **B1**, **A3**, **A2** — je ein bis fünf Zeilen, alle in `tests/verzeichnis.rs`, und
   B1 gehört sachlich zu `260826-1221`, das ohnehin offen ist.
3. **C1** — zwei Zeilen in `werkbankgestalt`, bevor die latente Flatterstelle einmal
   zuschlägt und jemand eine halbe Stunde am falschen Ende sucht.
4. **D1**, **D2**, **D3** — Aufräumen, jederzeit, gern in einem Zug mit anderer Arbeit an
   diesen Dateien.
