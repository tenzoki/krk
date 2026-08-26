# Wie kommt ein Änderungsdatum in eine Profilzeile, und in welcher Form steht es da?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Answered:** 260825-1740, Kai Stalmann — Moeglichkeit 1: juengste bekommt den Schluessel zeigt, mit den drei Festlegungen der Empfehlung. Empfehlung des Planers ohne Aenderung uebernommen.
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`; `shared/decisions/260825-1725_*_wo-wohnt-die-umrechnung-von-systemtime-in-buergerliche-ortszeit.md`; `crates/krk-core/src/leseprofil/mod.rs` (`Baustein`, `Wert`); `crates/krk-core/src/leseprofil/bausteine.rs` (`Lauf::juengste`); `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0541_*_was-heisst-die-juengsten-zehn-und-was-ist-ihr-titel.md`

---

## Question

Der Nutzer verlangt am 260825 zwei Zeitangaben in Zusammenfassungen: „wann letzte Archivierung
war" für `fusion-workbench/archive` und „das Datum des letzten Eintrags" für jeden Unterordner
von `fusion-workbench/shared`.

Keiner der vier Bausteine liefert das. `Baustein::Juengste` sortiert zwar nach dem
Änderungsdatum, zeigt aber Titel; `zaehlung` zählt, `vorhandensein` antwortet ja oder nein,
`feld` zieht einen Text aus einem Dateiinhalt. Auch `Wert` trägt keine Zeitangabe. Die Frage
zerfällt in zwei: **welcher Baustein liefert sie**, und **wie sieht sie in der Zeile aus**.

Ein zweiter Umstand entscheidet mit: `archive/` enthält Ordner und keine Dateien
(`260819-1613-safe-cleanup-tier-1`, `260820-2115-safe-cleanup-tier-1`). `Baustein::Juengste`
nimmt heute allein Einträge vom Typ Datei und sähe dort gar nichts.

## Options

1. **`juengste` bekommt einen Schlüssel `zeigt` mit zwei Werten, `titel` und `datum`.**
   `zeigt = "titel"` ist die heutige Fassung und der Vorgabewert; `zeigt = "datum"` zeigt
   statt des Titels das Änderungsdatum und öffnet dafür **keine** Datei.
   - Pros: Der Baustein bedeutet unverändert „die N Einträge mit dem jüngsten
     Änderungsdatum". Nur was er über sie **zeigt**, ändert sich, und die Sortierung, die
     Ortsauflösung und die Abbruchregel („die jüngsten zehn einer Teilliste sind nicht die
     jüngsten zehn") tragen unverändert weiter. Der Bausteinsatz bleibt bei vier, Festlegung
     A7 der Runde 16 steht. Die Datumsform kostet **null Dateiöffnungen**, weil das Datum aus
     `Eintrag::geaendert` kommt, das der Verzeichnisleselauf ohnehin liefert — sie ist damit
     billiger als die Titelform und nicht teurer.
   - Cons: Ein zweiter Schlüssel im Tisch, dessen Wert eine Fallunterscheidung aufmacht. Ein
     verschriebener Wert kostet nach der ersten Reichweite die ganze Datei, nicht die Zeile.
2. **Ein fünfter Baustein `juengstes_datum`.**
   - Pros: `juengste` bleibt unangetastet.
   - Cons: Er täte dasselbe wie `juengste` mit einer anderen Ausgabe und trüge die
     Sortierung, die Ortsangabe, das Muster und die Anzahl ein zweites Mal. `Baustein` ist
     eine vollständige Fallunterscheidung ohne Auffangzweig: ein fünfter Wert hält den Bau an
     sieben Stellen an, und Festlegung A7 hält die Zahl vier ausdrücklich fest.
3. **Ein neuer `Wert::Zeitpunkt`, und `juengste` liefert je nach Anzahl Titel oder Zeitpunkte.**
   - Pros: Der Wert trüge den Zeitpunkt strukturiert bis in die Anzeige, wie es der Zuschnitt
     von `Zusammenfassung` sonst tut.
   - Cons: `Wert` ist ebenfalls vollständig ohne Auffangzweig, und `Zusammenfassung::als_text`
     entscheidet an `Wert::Titel` ausdrücklich, wann ein Wert unter seine Beschriftung
     rutscht. Ein siebter Wert verlangte eine Antwort auf diese Frage, die die vorhandene
     Regel schon gibt: ein Wert mit Zeilenumbruch steht unter der Beschriftung, einer ohne
     daneben. Für ein einzelnes Datum ist „daneben" richtig, und `Wert::Text` liefert das,
     ohne dass eine Regel dazukommt.

## Constraints

- Der Bausteinsatz soll bei vier bleiben, solange die Aufgabe ohne einen fünften zu lösen ist
  (Festlegung A7 der Runde 16).
- Eine Zeitangabe muss ohne AppKit entstehen: die Auswertung liegt in `krk-core`, und
  `Zusammenfassung::als_text` ist die eine Stelle, an der aus Werten Zeilen werden. Der
  `NSDateFormatter`, den die Metadatenanzeige benutzt, ist von dort nicht erreichbar.
- `archive/` enthält Ordner. Eine Datumszeile, die nur Dateien sieht, beantwortet die Frage
  des Nutzers dort nicht.
- Eine unvollständige Lesung darf keine falsche Auskunft geben.

## Recommendation

**Möglichkeit 1**, mit drei Festlegungen:

- **`zeigt = "datum"` sieht Einträge jedes Typs, `zeigt = "titel"` weiter allein Dateien.**
  Das ist kein Sonderfall, sondern dieselbe Regel an einer Form angewandt, die nichts liest:
  der Modulkopf von `leseprofil::bausteine` begründet die Beschränkung auf Dateien damit, dass
  `Juengste` und `Feld` **Dateien lesen**. Wer nur ein Datum zeigt, liest keine Datei, also
  greift der Grund nicht. Das ist zugleich die Bedingung dafür, dass `archive/` überhaupt
  antworten kann.
- **Der Wert ist `Wert::Text`, kein neuer `Wert`.** Bei `anzahl = 1` steht das Datum neben
  seiner Beschriftung, bei mehreren stehen die Daten untereinander — beides folgt aus der
  vorhandenen Regel in `Zusammenfassung::als_text`, ohne dass eine zweite dazukommt. Die
  Dokumentation von `Wert::Text` weitet sich dabei von „ein aus einer Datei gezogenes Feld"
  auf „ein Text"; die Variante beschreibt die **Gestalt** eines Wertes und nicht seine
  Herkunft.
- **Die Form ist `JJJJ-MM-TT HH:MM` in bürgerlicher Ortszeit**, ohne Locale-Abhängigkeit. Sie
  ist eindeutig, sortiert sich von selbst und ist aus `krk-core` heraus herstellbar. Dass die
  Metadatenanzeige derselben Vorschaufläche ein locale-abhängiges Kurzformat zeigt, ist keine
  Unstimmigkeit, die aufzulösen wäre: die beiden erscheinen nie zusammen, denn ein Ordner
  zeigt entweder seine Metadaten oder seine Zusammenfassung. Woher die Ortszeit kommt,
  entscheidet der Datensatz `260825-1725_*_wo-wohnt-die-umrechnung-von-systemtime-…`.

Ein dritter Wert `titel-und-datum` wäre nützlich und wird hier **nicht** gebaut: der Auftrag
verlangt ihn nicht, und zwei Werte halten die Fallunterscheidung überschneidungsfrei und
vollständig mit dem kleinsten Bestand. Wer ihn später will, fügt einen dritten Wert hinzu und
hält den Bau an genau einer Stelle an.

---
Implemented: 66c779c — `juengste` trägt den Schlüssel `zeigt` mit den zwei Werten `titel` und `datum` (`crates/krk-core/src/leseprofil/datei.rs:270`, `:290`, `:292`); die Aufzählung heißt im Baum `Anzeige` (`crates/krk-core/src/leseprofil/mod.rs:360`) und nicht `Juengsteform` wie im Plan entworfen. Die Datumsform öffnet keine Datei, sieht Einträge jedes Typs und liefert einen `Wert::Text` über `ortszeit` (`crates/krk-core/src/leseprofil/bausteine.rs:788`); ein siebter `Wert` und ein fünfter Baustein sind nicht entstanden. Nachgemessen am 260826-0149 gegen den Baum, `make check` grün.
