Eine symbolische Verknüpfung auf einen Ordner lässt sich im Dateifenster nicht betreten

---

Ein Doppelklick auf eine symbolische Verknüpfung, die auf ein Verzeichnis zeigt, steigt nicht hinein. Dasselbe gilt für die Eingabetaste und jeden anderen Weg, der über dieselbe Prüfung läuft. Der Nutzer erwartet, dass eine solche Verknüpfung sich wie der Ordner verhält, auf den sie zeigt; gemeldet am 260814-1610.

---

**Gefunden am:** 260814, Stand `43dfe90`
**Gemeldet von:** Nutzer
**Herkunft:** neben der Arbeit am Circle `260814-1551-tippen-filtert-dateiliste-flach-und-tief` gefunden, nicht aus dessen Directive entstanden. Deshalb im gemeinsamen Speicher und nicht im Circle.

## Die Kette

Der Verzeichnisleser folgt einer Verknüpfung nicht, er meldet die Verknüpfung selbst (`crates/krk-core/src/verzeichnis/eintrag.rs:22-24`). Der Typ kommt aus `lstat(2)` über `VLNK` (`crates/krk-core/src/verzeichnis/sys.rs:341`), also trägt ein Verweis auf ein Verzeichnis den Typ `Verknuepfung` und nicht `Ordner`.

`Eintrag::ist_ordner` prüft `self.typ == Typ::Ordner` (`crates/krk-core/src/verzeichnis/eintrag.rs:111-113`) und antwortet für eine Verknüpfung deshalb `false`. Das Betreten hängt genau daran: `crates/krk-ui/src/appkit/tabelle.rs:1296-1299` steigt aus, sobald `ist_ordner` falsch ist, und reicht den Eintrag stattdessen an das System weiter.

## Warum das kein reines Umstellen von `ist_ordner` ist

`ist_ordner` hat mehrere Aufrufer mit verschiedenen Fragen. `crates/krk-ui/src/kommandos/operationen.rs:178` und `:190` zählen damit Ordner in einer Auswahl, `crates/krk-core/src/verzeichnis/modell.rs:367` benutzt es in der Sichtreihenfolge. Ob eine Verknüpfung dort als Ordner zählen soll, ist je Aufrufer verschieden zu beantworten: beim Löschen einer Auswahl ist die Verknüpfung selbst das Ziel und nicht ihr Verweisziel, beim Betreten ist es umgekehrt.

Die Sortierung ordnet ebenfalls nach der Aufzählung Ordner/Datei/Verknüpfung (`crates/krk-core/src/verzeichnis/sortierung.rs:8-11`); ein Verweis auf ein Verzeichnis, der plötzlich unter den Ordnern einsortiert, ist eine sichtbare Änderung und keine Nebenwirkung.

Der Deskriptorweg für die Zielprüfung steht schon: `crates/krk-core/src/verzeichnis/sys.rs:748` hält fest, dass bei einer Verknüpfung am Deskriptor ihr Ziel steht und `metadata()` darauf das Ziel beschreibt. Eine Prüfung am Deskriptor statt am Pfad ist in diesem Baum die eingeführte Bauform.

## Was noch offen ist

Ob eine Verknüpfung auf einen Ordner auch als Ziel einer Dateioperation, als Lesezeichen und in der Vorschau wie ihr Ziel behandelt wird, ist mit diesem Datensatz nicht beantwortet. Er beschreibt allein das Betreten.

## Abgrenzung

Die tiefe Suche des Circles `260814-1551-tippen-filtert-dateiliste-flach-und-tief` steigt ausdrücklich **nicht** in Verknüpfungen hinab (`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1552_*_steigt-die-tiefe-suche-in-symbolische-verknuepfungen-hinab.md`). Dieser Defekt und jene Antwort widersprechen sich nicht: der Scan folgt keiner Verknüpfung, der Nutzer soll sie von Hand betreten können.

---
Stand 260815-1700: gebaut, im Kern geprüft, **nicht abgenommen**. Der Datensatz bleibt
deshalb auf `_p_` und nicht auf `_c_`.

**Die Entwurfsrichtung ist ein Nutzerentscheid vom 260815:** aufgelöst wird allein im
Einstiegsweg, am Deskriptor, und nicht beim Lesen des Ordners. Der Lesevorgang bekommt
keinen zusätzlichen Systemaufruf, weil der Sortierschlüssel dort einmalig entsteht und daran
die Zeitzusagen L3 und L10 hängen, die seit der Runde 4 nicht mehr gemessen sind. Der eine
Aufruf fällt beim Doppelklick an, nicht bei der Anzeige.

**Der Schnitt.** Neu ist `krk-core/src/verzeichnis/verweisziel.rs` mit der Aufzählung
`Verweisziel` (`Ordner`, `KeinOrdner`, `Unerreichbar`), überschneidungsfrei und ohne
Auffangzweig. Sie fragt über `sys::ohne_warten_oeffnen` und ist deren dritter Rufer neben
dem Editor und der Vorschau; eine zweite Hülle um `open` oder `stat` ist nicht entstanden.
Der prüfbare Teil liegt im Kern, weil `krk-ui` kein Bibliotheksziel hat.

`in_zeile_einsteigen` liefert jetzt die Aufzählung `Einstieg` statt eines Wahrheitswerts, und
das ist der Kern der Sache: „gemeldet" ist weder „eingestiegen" noch „gib es an das System".
Ginge eine unerreichbare Verknüpfung zusätzlich an das Standardprogramm, überschriebe dessen
Antwort die eben geschriebene Zeile der Statuszeile.

**Die drei Fälle:** auf ein Verzeichnis heißt Einstieg, und zwar mit dem Pfad der
Verknüpfung, damit der Aufstieg zurück in deren Ordner führt. Auf eine Datei heißt, was heute
bei einer Datei geschieht. Ins Leere, im Ring oder ohne Recht heißt ein Satz in der
Statuszeile und kein zweiter Versuch.

**Die Warnung dieses Datensatzes zur Sortierung ist gegenstandslos**, am 260815 am Baum
geprüft: `sortierung.rs`, `fn gruppe`, liest `eintrag.typ == Typ::Ordner` unmittelbar und
nicht über `ist_ordner`; zudem ordnet die Typsortierung seit dem Nutzerentscheid vom 260806
nach der Dateiendung. `ist_ordner` ist ohnehin unverändert geblieben.

**Was zum Abschluss fehlt: ein Klicktest am laufenden Bündel.** Der Weg vom Doppelklick bis
`ordner_lesen` hängt an `NSTableView` und ist nicht kopflos prüfbar; die Auflösung selbst ist
mit sechs Proben im Kern abgedeckt, darunter der Ring und eine benannte Röhre unter
Zeitschranke. Wie bei jedem Abnahmelauf dieses Projekts ist das Nutzerarbeit.

**Ein Nebenbefund:** eine Verknüpfung auf einen Socket kommt als `Unerreichbar` zurück und
nicht als `KeinOrdner`, weil `open(2)` dort mit `ENXIO` scheitert. Der Nutzer bekommt eine
Meldung statt stiller Wirkungslosigkeit; richtig eingeordnet ist es nicht.

---
Abgleich 260815-1812 (reconciler): **Der Marker ist von `_p_` auf `_o_` gezogen, und zwei
Aussagen der Notiz vom 260815-1700 sind seit `7fae5ba` überholt.**

**Zum Marker.** `_p_` heißt nach `rules/fusion-workbench-conventions.md`, Abschnitt
`## State Markers — issues and planning`, „ein Agent arbeitet aktiv daran". Nach dem Ende
der Sitzung 260815-1328 arbeitet niemand daran: `agentstate.yaml` führt die Aufgabe `I:7`
als `done` mit Commit `8c06747`, und der Sitzungslauf ist beendet. Das Vokabular der
Defektdatensätze kennt vier Werte, und von `_o_`, `_p_`, `_c_`, `_d_` beschreibt allein
`_o_` die Lage „nicht abgeschlossen, niemand arbeitet daran". `_c_` wäre falsch, weil die
Abnahme aussteht; `_d_` verlangt nach derselben Regel eine Entscheidung des Nutzers und
steht dem Abgleich nicht zu. Gezogen ist deshalb `_o_`.

**Der Sache nach ist der Datensatz weder offen noch geschlossen**, sondern in derselben Lage
wie die neun beschränkt geschlossenen Runden dieses Projekts: gebaut, im Kern geprüft, nicht
abgenommen, weil der Lauf KRK im Vordergrund verlangt und damit Nutzerarbeit ist. Ein
Gegenstück zum Circle-Marker `_b_` hat das Defektvokabular nicht. Ob dieser Datensatz
stattdessen als `_d_` mit dem Ziel „nächster Abnahmelauf des Nutzers" stehen soll, ist eine
Frage an den Nutzer und hier nicht entschieden.

**Zwei überholte Aussagen der Notiz vom 260815-1700.**

1. „Sie fragt über `sys::ohne_warten_oeffnen` und ist deren dritter Rufer neben dem Editor
   und der Vorschau." Seit `7fae5ba` fragt `verweisziel::bestimmen` über
   `std::fs::metadata` (`crates/krk-core/src/verzeichnis/verweisziel.rs:164-165`), und die
   Hülle hat wieder genau zwei Rufer: `grep -rn 'ohne_warten_oeffnen' crates/` findet am
   260815-1812 die Aufrufstellen `text/datei.rs:414` und `vorschaumodell.rs:679`, alles
   übrige sind Verweise in Prosa. Der Anlass des Wechsels ist
   `shared/issues/260815-1713_*_verweisziel-beantwortet-die-ordnerfrage-mit-open-und-nicht-mit-stat.md`.
2. „Eine Verknüpfung auf einen Socket kommt als `Unerreichbar` zurück und nicht als
   `KeinOrdner`, weil `open(2)` dort mit `ENXIO` scheitert." Am Referenzgerät gemessen
   scheitert `open(2)` dort mit `EOPNOTSUPP` (os error 102) und nicht mit `ENXIO`, und der
   Socket war nicht der einzige Fehlfall; die Messreihe steht in `260815-1713`. Seit
   `7fae5ba` liefert der Socket `KeinOrdner`, der Nebenbefund ist damit behoben.

**Was zum Abschluss unverändert fehlt**, ist der Klicktest am laufenden Bündel. Die
Auflösung selbst ist im Kern abgedeckt: `cargo test --workspace` läuft am 260815-1812 grün
(Exit 0), und `crates/krk-core/tests/verzeichnis.rs` prüft `verweisziel::bestimmen` an neun
Stellen, darunter Ring, benannte Röhre unter Zeitschranke, Datei ohne Leserecht und
Verzeichnis mit Modus `0111`.
