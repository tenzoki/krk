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
