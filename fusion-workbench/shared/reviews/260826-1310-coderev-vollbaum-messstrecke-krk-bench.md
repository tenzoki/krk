# Vollbaum-Durchsicht der Messstrecke — `crates/krk-bench/`

**Reviewed-range:** `004ff72..4a57028`
**Not-opened:** none

> Die zwei Pflichtfelder stehen in der Form, die `rules/review-contract.md` vorschreibt und
> `bin/fusion-review-coverage` liest: zwei aufgeloeste Kurzhashes und das blanke Wort `none`.
> Der Auftrag hat sie als „kein Commit-Bereich, Vollbaum-Durchsicht von crates/krk-bench/ an
> HEAD 4a57028“ und „keine“ verlangt; in dieser Form parst das Werkzeug sie nicht und meldet
> die Datei als UNUSABLE. **Der Sache nach ist beides dasselbe:** dies war keine Durchsicht eines
> Commit-Bereichs, sondern des Vollbaums von `crates/krk-bench/` am Stand `4a57028`, und der
> Quelltext dieser Kiste ist seit `004ff72` unveraendert — die Spanne deckt also genau den Baum,
> den ich gelesen habe. Gelesen sind alle fuenf Dateien der Kiste.

**Sender:** coderev
**Gelesen:** 6 Dateien, 5.244 Zeilen — die fuenf Quelldateien mit zusammen 5.224 Zeilen
(`src/bericht.rs` 930, `src/fixture.rs` 873, `src/main.rs` 483, `src/messen.rs` 2.874,
`src/wegwerfordner.rs` 64) und dazu `Cargo.toml` mit 20. `git ls-files 'crates/krk-bench/**'`
listet sechs Pfade; die Zahl 5.224 des Auftrags ist die Summe der fuenf `.rs`-Dateien ohne
das Manifest. Zeilenzahlen mit `wc -l` am Baum abgelesen, nicht gerechnet.
**Gefilterte Defekte:** 9 — Hoch 1, Mittel 5, Niedrig 3

## Summary

Die Kiste ist die einzige Stelle im Projekt, an der die Maxime „superschnell" eine Bedeutung hat,
und sie ist überwiegend mit der Sorgfalt gebaut, die das verlangt: die Abnahmemaße tragen ihre
Herleitung, das Perzentil ist nicht interpoliert, der Cache-Zustand wird nicht behauptet, eine
fehlende Bildwiederholrate bricht ab statt 60 Hz zu unterstellen, und die Sitzung des Nutzers
kommt über drei Wege zurück. Die zwei Fragen des Auftrags, die eine klare Antwort haben, haben
eine gute: **die Vordergrund-Sperre lässt keine Teilmessung durch**, und **die Voraussetzung des
Messplanwächters hält am heutigen Baum** — sie hält sogar besser, als `CLAUDE.md` behauptet.

Die Befunde liegen alle in derselben Richtung: **die Strecke prüft ihre Eingangsbedingungen
weniger scharf als ihre Ausgangsbedingungen.** Was sie misst, misst sie sauber; **worauf** sie
misst, nimmt sie weitgehend ungeprüft entgegen. Der schwerste Befund ist, dass kein Prüfordner
außer dem L6-Unterordner gegen seine zugesagte Eintragszahl gehalten wird — die Zahl ist
Bestandteil der Zusage, und L3 auf 3.000 Einträgen hält mühelos.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| Hoch | 1 |
| Mittel | 5 |
| Niedrig | 3 |

## Was die Durchsicht ausdrücklich bestätigt

Vier Zusagen des Auftrags habe ich gegen den Baum gehalten und **bestätigt gefunden**; sie sind
hier festgehalten, damit die nächste Durchsicht sie nicht noch einmal fährt.

**Die Vordergrund-Sperre lässt keine Zahl durch.** `messung_unmoeglich` gibt
`NICHT_IM_VORDERGRUND` **vor** jeder Sitzungsgröße zurück (`krk-ui/src/messmodus.rs:744`), der
Weg endet über `Anweisung::Abbruch` in `std::process::exit(4)`
(`krk-ui/src/appkit/anwendung.rs:7838-7840`), und `ausgeben()` — die einzige Stelle, die
`wert …`-Zeilen und die Zeile `fertig` schreibt (`messmodus.rs:1414-1437`) — wird auf diesem Weg
nicht gerufen. Auf der Bench-Seite verlangen `sitzung_messen` (`messen.rs:1227`) und
`spannen_messen` (`messen.rs:874`) die Zeile `fertig`, und `hole` verlangt je Größe **genau**
`wiederholungen` Werte (`messen.rs:822-833`, `1173-1184`), wobei `werte_lesen`
(`messen.rs:1820-1828`) unparsbare Zeilen fallen lässt und die Zählung damit scharf hält. Eine
Teilmessung kann als gültig nicht durchrutschen.

**Der Messplanwächter hat heute genau einen Greifer.** `plan_schreiben` (`messen.rs:1661`) ist die
einzige Stelle im Baum, die `std::env::temp_dir()` für den Plan wählt, und hat einen Rufer,
`Gesamtlauf::fahren` (`messen.rs:1029`). Beide Messplan-Proben gehen über
`plan_in_verzeichnis_schreiben` beziehungsweise `Messplanwaechter::in_verzeichnis` mit einem
`Wegwerfordner` (`messen.rs:2720`, `2769`). `cargo test` fasst das echte Temporärverzeichnis für
den Plan nicht mehr an. **`CLAUDE.md` behauptet das Gegenteil** → Befund 6.

**Die 65 von L9 steht an einer Stelle im Betriebsweg.** `mindestanteil_prozent: 65` steht in
`Gesamtlauf::fahren` einmal (`messen.rs:1147`); die beiden weiteren Vorkommen sind Probenaufbau
(`messen.rs:2247` in `l9_zusage`, `messen.rs:2599` in `jede_zeile_nennt_ihr_abnahmemass`, `bericht.rs:864` im Berichtsaufbau). Die Begründung samt beider Senkungen und
beider Datensätze steht unmittelbar an der Zahl (`messen.rs:1124-1144`). Die Frage des Auftrags
ist für L9 mit Ja beantwortet — **für L1 und die vier Perzentilgrenzen nicht** → Befund 9.

**`#![deny(unsafe_code)]` steht an der Kistenwurzel (`main.rs:1`) und hat in dieser Kiste keine
Ausnahme.** `grep -rn unsafe crates/krk-bench/` findet außer dieser Zeile nur drei
Doc-Kommentare, die die Grenze erklären. Der Signalgriff geht bewusst über `signal-hook` statt
über `libc`, und der Grund steht dabei (`messen.rs:1324-1341`).

**L7 und die Runde 14/18:** die Messstrecke sieht L7 als **einen** Wertstrom aus dem
Sitzungslauf (`messen.rs:1110-1113`; die Werte kommen aus `let l7 = hole("l7")?;`, `messen.rs:1189`) und kann von sich aus nicht unterscheiden,
welchen Vorschauweg der gemessene Eintrag genommen hat. Das ist vollständig als offene Frage
geführt
(`circles/260823-2208-…/decisions/260824-1900_o_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`,
mit `shared/decisions/260819-2216_*` für die Runde 14) und **wird hier nicht neu gefiled**. Das
Zitat jenes Datensatzes auf `messen.rs:1110-1114` trifft am heutigen Baum den L7-Block auf `messen.rs:1109-1114`.

## Findings by theme

### Thema 1 — Die Eingangsbedingungen sind schwächer geprüft als die Ausgangsbedingungen

**[Hoch] Kein Prüfordner außer dem L6-Unterordner wird gegen seine zugesagte Eintragszahl
gehalten.** `shared/issues/260826-1301_o_…`. `Gesamtlauf::fahren` prüft an `messen.rs:1019-1026`
allein `is_dir()`; `Messreihe::fahren` prüft an `messen.rs:170-180` nur, dass alle zwanzig Läufe
**dieselbe** Zahl lesen. Der Gegenmaßstab steht eine Bildschirmseite tiefer:
`unterordner_sicherstellen` (`messen.rs:1485-1509`) weist einen Steckbrief mit falscher Zahl ab
**und** einen Ordner ohne Steckbrief. Dazu: die tatsächlich gelesene Zahl (`Messreihe.eintraege`,
`messen.rs:131`) wird in `eine_gesamtrunde` fallen gelassen (`messen.rs:1208-1209`, `1217-1218`)
und erreicht den Abnahmebericht nie — dort steht nur, was der Steckbrief behauptet
(`messen.rs:2088-2101`).

**[Niedrig] Der Wegwerfordner räumt die zweite Nachbardatei nicht ab.**
`shared/issues/260826-1308_o_…`. `Wegwerfordner::drop` (`wegwerfordner.rs:54-63`) nimmt
`<ordner>.pruefordner.toml` mit, nicht `<ordner>.zeitmarke` (`fixture.rs:485-486`). Der Erzeuger
räumt sie auf beiden gewöhnlichen Wegen selbst ab (`fixture.rs:457-459`); ungedeckt bleibt der
schmale Weg, auf dem `File::create` gelingt und `set_times` scheitert (`fixture.rs:453-454`).
Betrifft auch echte Prüfordner im Messplatz, nicht nur Proben.

### Thema 2 — Die zwei Abnahmemaße urteilen über dieselbe Eingabe verschieden

Zwei Befunde derselben Wurzel, und sie treffen sich in `Zusage::gehalten_in`.

**[Mittel] Ein Lauf ohne Runden besteht das Gate.**
`shared/issues/260826-1302_o_ein-lauf-ohne-runden-…`. Bei leerer Rundenliste liefert
`gehalten_in` `Some((0, 0))`, `immer_gehalten` sagt `Some(true)` (`messen.rs:622-625`),
`bestanden` sagt `true` (`messen.rs:734-738`, `1009-1013`), und `urteil` (`messen.rs:1986-1994`)
schreibt „gehalten in allen 0 Runden". Die Wachen stehen in `main.rs:277` und `353`, während
`Messreihe::fahren` sich an derselben Frage **selbst** prüft (`messen.rs:144-149`) — drei
`fahren`, zwei Haltungen.

**[Mittel] Der Perzentil-Zweig hat keine Wache gegen eine Runde ohne Werte.**
`shared/issues/260826-1303_o_der-perzentil-zweig-…`. Der Anteils-Zweig hat sie ausdrücklich
(`messen.rs:597-600`, „Eine Runde ohne Werte haelt nicht"), der Perzentil-Zweig darüber nicht
(`messen.rs:579-583`), und `perzentil(&[], …)` liefert `Duration::ZERO` (`messen.rs:306-309`) —
den bestmöglichen Wert. `bestes_perzentil`, `minimum` und `maximum` (`messen.rs:472`, `491`,
`496`) fallen genauso auf null zurück, sodass eine abwesende Messung im Bericht als `0.000 ms`
steht. Über die Befehlszeile heute nicht erreichbar; `Zusage` und `gehalten_in` sind `pub`.

### Thema 3 — Eine Messbedingung wird aus einer Runde übernommen statt geprüft

**[Mittel] Die Bildlänge für L1 und L9 stammt aus der ersten Runde.**
`shared/issues/260826-1304_o_…`. `rate = rate.or(gemeldete_rate)` (`messen.rs:749`, `1042`)
behält die erste Rate und verwirft jede spätere ungeprüft — und weil `spannen_messen`
(`messen.rs:880-888`) und `sitzung_messen` (`messen.rs:1233-1241`) ohne Rate abbrechen, greift
das `.or` in **jeder** Runde ab der zweiten. Der Berichtskopf schreibt eine Zahl aus, als hielte
sie für den ganzen Lauf (`bericht.rs:236-245`). Die Gegenhaltung steht daneben:
`bildlaenge_bilden` (`messen.rs:662-683`) bricht bei fehlender Rate ausdrücklich ab, statt zu
unterstellen; eine widersprüchliche Rate ist derselbe Fall aus der anderen Richtung.

### Thema 4 — Absicherungen, die diese Kiste an einer Stelle führt und an der Nachbarstelle nicht

**[Mittel] `krk-bench` trägt ein einziges `#[must_use]`.**
`shared/issues/260826-1305_o_…`. Gezählt am 260826: `krk-core` 66, `krk-ui` 95, `krk-bench` 1 —
und das eine steht auf `Messplanwaechter` (`messen.rs:1533`), dessen Fallenlassen eine Datei im
Temporärverzeichnis kostet. `Sitzungswaechter` (`messen.rs:1316`) in derselben Datei trägt keines,
und sein zu frühes Fallen nähme die Prüfsitzung **vor** der ersten Runde zurück; die zwanzig
L4-Starts mäßen dann die Sitzung des Nutzers, ohne dass irgendetwas abbräche. Kein Rufer im Baum
tut das heute — der Befund ist die fehlende Absicherung.

**[Niedrig] Eine Probe löscht einen festen Namen im echten Temporärverzeichnis.**
`shared/issues/260826-1309_o_…`. `bericht.rs:923-929` baut
`std::env::temp_dir().join("krk-bench-gibt-es-nicht")` und ruft `remove_dir_all` darauf, statt
über den `Wegwerfordner` zu gehen. Praktisch harmlos, weil unter dem Namen nie etwas liegt; es
ist die eine verbliebene Ausnahme von einem Maßstab, den diese Kiste zweimal durchgesetzt hat
(`260809-1106`, `260810-1925`).

### Thema 5 — Berichte und Prosa

**[Mittel] `CLAUDE.md` nennt `cargo test` als zweiten Greifer auf den Messplan.**
`shared/issues/260826-1306_o_…`. Der zitierte Datensatz `260810-1925` trägt seit dem 260811
`_c_`, und der Baum trägt den zweiten Greifer nicht mehr (Beleg oben unter „Was die Durchsicht
bestätigt"). Der Satz warnt vor einer Wechselwirkung, die es nicht gibt, und verweist zum Beleg
auf einen Datensatz, der das Gegenteil sagt. **Die Voraussetzung selbst — nie zwei gleichzeitige
Messläufe — bleibt richtig und muss stehen bleiben.** Der Doc-Kommentar im Code
(`messen.rs:1603-1619`) ist bereits die korrekte Fassung.

**[Niedrig] Ein Messbericht kann einen früheren still überschreiben.**
`shared/issues/260826-1307_o_…`. Alle drei Schreiber bilden den Namen aus `kurzstempel`
(Minutengenauigkeit) und schreiben mit `fs::write`: `bericht.rs:188`/`198`, `bericht.rs:495-498`,
`messen.rs:2008-2011`. Für `alle` und `durchstich` unwahrscheinlich (Laufzeit in Minuten), für
`messen --kopflos` auf einem kleinen Ordner nicht. Der Bericht ist der einzige Beleg einer
gehaltenen Zusage.

## Cross-cutting observations

**Die Kiste kennt ihre eigenen Maßstäbe und wendet sie nur nicht überall an.** Fünf der neun
Befunde haben dieselbe Form: an einer Stelle steht die scharfe Regel, an der Nachbarstelle nicht,
und beide sind derselbe Fall.

| Regel | Angewandt | Nicht angewandt |
|---|---|---|
| Steckbrief entscheidet über den Bestand | `unterordner_sicherstellen` (`messen.rs:1485-1509`) | A, B, 100k (`messen.rs:1019-1026`) |
| Eine leere Reihe hält nicht | Anteils-Zweig (`messen.rs:597-600`) | Perzentil-Zweig (`messen.rs:579-583`) |
| Die Funktion prüft ihre Null selbst | `Messreihe::fahren` (`messen.rs:144-149`) | `Durchstich::fahren`, `Gesamtlauf::fahren` |
| Fehlende Messbedingung bricht ab | `bildlaenge_bilden` (`messen.rs:662-683`) | widersprüchliche Rate (`messen.rs:749`, `1042`) |
| `#[must_use]` auf dem Wächter | `Messplanwaechter` (`messen.rs:1533`) | `Sitzungswaechter` (`messen.rs:1316`) |
| Name zuerst, dann anlegen | `Messplanwaechter` (`messen.rs:1584-1598`) | `verknuepfungszeiten_setzen` (`fixture.rs:453-454`) |
| Kein fester Name im Temporärverzeichnis | jede Probe in `messen.rs`, `fixture.rs` | `bericht.rs:923` |

Das ist keine Nachlässigkeit im Einzelnen, sondern ein Muster: die Regeln sind je an dem Defekt
entstanden, der sie erzwungen hat, und die Schwesterstelle stand damals nicht zur Debatte.

**Die Duplizierung der Abnahmemaße zwischen Durchstich und Gesamtlauf ist der leiseste Fall
davon.** L1s 95 Prozent steht zweimal (`messen.rs:775`, `1068`), ebenso die Grenzen von L2
(`783`, `1076`), L3 (`789`, `1082`), L4 (`795`, `1088`) und der ersten L10-Seite (`801`, `1155`).
Die beiden sind an einer Stelle bereits **absichtlich** auseinander — das volle Lesen der 100.000
trägt im Durchstich `Abnahmemass::Keine` (`messen.rs:810`) und im Gesamtlauf
`Perzentil(4000 ms)` (`messen.rs:1161`) —, und genau das macht eine unabsichtliche Abweichung
ununterscheidbar von einer gewollten. Kein eigener Defektdatensatz: der Durchstich ist die
eingefrorene Frühmessung aus S8, und seine Maße ändern sich nur, wenn C8 sich ändert. Festgehalten
ist es hier, damit die Frage bei der nächsten Änderung an C8 auf dem Tisch liegt.

**Der Zustand der Zusagen selbst, unabhängig vom Code:** der letzte vollständige Lauf ist vom
260810 (`messungen/260810-1918-alle-zusagen.txt`) und liegt vor jeder seither geschlossenen Runde.
Befund 1 und Befund 4 wirken beide erst bei einem nächsten Lauf — und beide sind genau dann
teuer, wenn dieser Lauf nach fünfzehn Tagen auf Prüfordnern gefahren wird, die niemand seither
angesehen hat.

## Recommended sequencing

**Vor dem nächsten Abnahmelauf** (beide wirken nur dort, und beide entwerten den Lauf, wenn sie
stehen bleiben):

1. `260826-1301` — Prüfordner gegen ihre zugesagte Eintragszahl halten. Die Vorlage steht
   fünfzig Zeilen weiter in derselben Datei.
2. `260826-1304` — die Bildwiederholrate je Runde vergleichen statt die erste zu behalten. Zwei
   Zeilen.

**Danach, in einem Zug** (Absicherungen ohne Verhaltensänderung am Erfolgsweg):

3. `260826-1302` und `260826-1303` — die beiden Wachen in `gehalten_in` und in den zwei `fahren`.
4. `260826-1305` — `#[must_use]` auf `Sitzungswaechter`. Für `Wegwerfordner` erst entscheiden, ob
   alle drei Prüfordner-Fassungen es bekommen.

**Aufräumen, ohne Eile:**

5. `260826-1306` — den überholten Halbsatz in `CLAUDE.md` streichen. Kostet nichts und nimmt eine
   falsche Warnung aus dem Abschnitt, den jeder Agent liest.
6. `260826-1307`, `260826-1308`, `260826-1309`.
