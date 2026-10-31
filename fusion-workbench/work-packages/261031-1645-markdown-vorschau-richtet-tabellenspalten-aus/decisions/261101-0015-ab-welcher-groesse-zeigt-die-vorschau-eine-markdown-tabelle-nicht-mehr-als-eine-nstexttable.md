# Ab welcher Größe zeigt die Vorschau eine Markdown-Tabelle nicht mehr als eine NSTextTable, und was zeigt sie stattdessen?

---
**Domain:** code
**Filed by:** analyst, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261031-1645-markdown-vorschau-richtet-tabellenspalten-aus.md, 261031-1720-plan-markdown-vorschau-richtet-tabellenspalten-aus.md (`## Where this work stops`, Risikotabelle), 261031-1720-mit-welchem-mittel-richtet-die-vorschau-eine-markdown-tabelle-aus.md, 261031-1720-schuldet-das-tabellenpaket-einen-abnahmelauf-gegen-die-zehn-zusagen.md, 261101-0015-zellenmerkmal-misst-je-tabellenzelle-schrift-und-leerzeichenbreite-neu.md, 260806-1412_*_bildgrenze-der-vorschau.md

---

## Question

Der Nutzer hat am 261101 am Bündel `~/Downloads/tabellen-gross.md` geöffnet (1 000 009 Byte, eine Tabelle, 4 Spalten, 31 079 Zeilen samt Kopf, also 124 320 Zellen). KRK blockiert. Die Stoppbedingung des Plans ist damit eingetreten: das Paket baut keine Ausweichregel nebenbei, und die Frage nach einer Größengrenze steht hier.

Zu entscheiden ist zweierlei. Erstens, ab welcher Größe eine Markdown-Tabelle nicht mehr als **eine** `NSTextTable` erscheint. Zweitens, was die Vorschau darüber zeigt.

## Befund: wo die Zeit hingeht

Gemessen am 261101 auf einem Apple M2 Max (`Mac14,5`), **nicht auf dem Referenzgerät** (`MacBookPro15,1`, Intel 2018, `260802-1036_*_leistungszusagen-navigator.md`). Dort ist jede Zahl größer; um wie viel, ist ungemessen. Die Messprogramme bauen `auszeichnungen_legen`, `texttabelle` und `zellenmerkmal` aus `crates/krk-ui/src/appkit/textmerkmale.rs` (Stand `83ee19dc`) in Swift nach: ein `NSTextTable` je Tabelle, automatische Spaltenbreite, `collapsesBorders`, 0,5 pt Linie in `separatorColor`, 3 pt Innenabstand, je Zelle ein `NSTextTableBlock` in einem eigenen Absatzstil, alles zwischen `beginEditing` und `endEditing`. Gezeichnet wird mit `cacheDisplayInRect:` in einer `NSTextView` in einer `NSScrollView` von 500 × 800 pt, ohne das Fenster zu zeigen. Die Programme liegen außerhalb des Baums unter `/private/tmp/claude-501/-Users-kai-Projects-productive-F08-KRK-krk/0d876e38-8be9-4836-8bba-c89e56b51c4e/scratchpad/tabmess/` (`mess.swift`, `mess2.swift`, `zeichnen2.swift`, `roh.swift`).

**Die Ursache ist das Zeichnen, nicht das Auslegen.** Jedes Neuzeichnen eines sichtbaren Ausschnitts kostet Zeit im Verhältnis zur Zellenzahl der **ganzen** Tabelle. Ein Mitschnitt mit `sample` über wiederholtes Zeichnen bei 10 000 Zeilen legt 4 146 von 4 970 Proben in `-[NSLayoutManager _drawBackgroundForGlyphRange:atPoint:]`, darunter `-[NSTextTable drawBackgroundForBlock:withFrame:inView:characterRange:layoutManager:]`, `NSTextBlockLayoutHelper` und `rangeOfTextBlock:atIndex:`. Der Hintergrund der Tabelle wird bei jedem Bild über die ganze Tabelle gerechnet. Rollen, Auswahl und jede andere Neuzeichnung zahlen das je Bild.

| Zeilen (4 Spalten) | Zellen | Merkmale setzen | erstes Bild (zwingt das volle Auslegen) | jedes weitere Bild, Mitte |
|---:|---:|---:|---:|---:|
| 100 | 404 | 2 ms | 25 ms | 13 ms |
| 200 | 804 | 3 ms | 38 ms | 25 ms |
| 500 | 2 004 | 8 ms | 91 ms | 60 ms |
| 1 000 | 4 004 | 14 ms | 225 ms | 117 ms |
| 4 000 | 16 004 | 57 ms | 891 ms | 461 ms |
| 10 000 | 40 004 | 143 ms | 2 231 ms | 1 162 ms |
| 31 079 | 124 320 | 427 ms | 7 003 ms | 3 713 ms |

Die Kurve ist linear und nicht quadratisch. Sie ist es aber **je Bild**. Bei der Abnahmedatei kostet das erste Bild rund 7 s und jedes weitere 1,8 s (oben) bis 3,7 s (Mitte); das ist die Blockade, die der Nutzer sieht.

Was sonst gemessen ist:

- **Das Auslegen allein** ist linear und nicht die Ursache: 2,0 s für 31 079 Zeilen in einer Tabelle, 2,2 s geteilt in Tabellen zu 1 000 Zeilen, gegen 0,5 s für denselben Text als Quelltext und 1,1 s als 124 320 Zeilen ohne Tabelle (`roh.swift`). Bei **einer** Tabelle erzwingt schon das erste Bild das Auslegen der ganzen Tabelle; geteilte Tabellen legt TextKit 1 nur bis zum sichtbaren Ausschnitt aus.
- **Die Spaltenzahl zählt über die Zellen**, also ist die Grenze eine Zellengrenze: bei 4 000 Zeilen kostet ein Bild in der Mitte 184 ms mit 2 Spalten (8 004 Zellen), 461 ms mit 4 (16 004) und 1 273 ms mit 8 Spalten (32 004).
- **Linie und Breitenregel ändern nichts**: ohne Linie 915 ms statt 1 189 ms je Bild bei 10 000 Zeilen; feste statt automatische Spaltenbreite (je Spalte 25 Prozent) 1 188 ms je Bild und 2,016 s statt 2,024 s Auslegen bei 31 079 Zeilen.
- **Nicht zusammenhängendes Auslegen** (`allowsNonContiguousLayout`) ändert nichts: 7,65 s erstes Bild, 3,69 s je Bild in der Mitte.
- **Geteilte Tabellen brechen die Kurve**: in Tabellen zu 100 Zeilen geteilt kostet ein Bild 6 bis 13 ms, gleich ob die Datei 10 000 oder 31 079 Zeilen hält, und das erste Bild 15 bis 27 ms. Zu 50 Zeilen 13 bis 18 ms, zu 200 Zeilen 13 bis 25 ms, zu 500 Zeilen 30 bis 64 ms.
- **Der schlechteste Fall unter der Textgrenze** von 1 MiB (`TEXTGRENZE`, `crates/krk-ui/src/vorschaumodell.rs`) ist eine schmale Tabelle mit kurzen Zellen. Nachgebaut mit 174 000 Zeilen zu 2 Spalten (348 002 Zellen), geteilt zu 100 Zeilen: Merkmale setzen 1,36 s, jedes Bild 3 bis 16 ms, volles Auslegen 6,5 s.

**Eine zweite Last liegt auf dem Hauptfaden vor dem ersten Bild.** `auszeichnungen_legen` ruft für jede Zelle `grundschrift(ansicht, art)` und über `zellenmerkmal` den `grundabsatz`, der mit `leerzeichenbreite` die Breite eines Leerzeichens neu misst (`crates/krk-ui/src/appkit/textmerkmale.rs`, `fn auszeichnungen_legen`, Arm `Auszeichnung::Tabellenzelle`; `fn grundabsatz`; `fn leerzeichenbreite`). Nachgebaut kostet das bei 124 320 Zellen 1,48 s statt 0,42 s. Der Defekt steht eigens in `261101-0015-zellenmerkmal-misst-je-tabellenzelle-schrift-und-leerzeichenbreite-neu.md`, weil er unabhängig von der Antwort hier behoben werden kann. Die Zahlen unten für das Setzen der Merkmale gelten für den behobenen Stand.

**Die bestehenden Größengrenzen der Vorschau**, damit die neue Zahl neben ihnen steht: `TEXTGRENZE` 1 MiB und `BILDGRENZE` 64 MiB (`crates/krk-ui/src/vorschaumodell.rs`), `trenntextvorschau::HOECHSTBREITE` 40 Zeichen je CSV-Spalte, `krk_core::trenntext::HOECHSTENS_SPALTEN` 256, `krk_core::text::datei::EDITORGRENZE` 16 MiB für den Editor, dazu die Lesegrenzen in `krk_core::leseprofil` (`HOECHSTENS_BYTES` 64 KiB und folgende). Eine Grenze für Tabellen gibt es nicht; die Textgrenze lässt jede Tabelle bis 1 MiB herein.

## Options

1. **Eine Grenze je Tabelle, darüber der Quelltext.** Hat eine Tabelle mehr als **400 Zellen**, erscheint sie wie vor dem Paket: ihre Zeilen wörtlich, als Absatz mit weichen Umbrüchen. Die Zahl kommt aus der Messung: 404 Zellen kosten 13 ms je Bild und bleiben unter einem Bild bei 60 Hz, 804 Zellen kosten 25 ms und nicht mehr. Gemessen auf dem M2 Max; auf dem Referenzgerät liegt die Zahl tiefer, um wie viel ist ungemessen.
   - Zeitzusagen: Die Tabelle bleibt unter einem Bild, und keine große Tabelle erreicht den Hauptfaden als `NSTextTable`. L7 ist von der Grenze nicht berührt, denn der Text steht vor der Einfärbung da; das Setzen der Merkmale bleibt bei 400 Zellen unter 3 ms.
   - Kopieren: Der Rückfall ist der Weg vor dem Paket. Die Kachelung ist wörtlich, und eine Auswahl liefert ihren Quelltext ohne neue Regel.
   - Nutzersichtbare Texte: Ohne Hinweis keiner. Mit Hinweis („Tabelle zu groß für die Ausrichtung“ oder ähnlich) ein Schlüssel in `de.rs`, `fr.rs` und `en.rs` unter `crates/krk-core/src/sprache/tabelle/`.
   - Gegen sie spricht: 400 Zellen sind 100 Zeilen zu 4 Spalten. Eine Tabelle mit 120 Zeilen fiele schon heraus, und die Grenze trifft Dateien, die der Nutzer alltäglich öffnet.

2. **Jede Tabelle in Abschnitte zu 100 Zeilen geteilt, je Abschnitt ein `NSTextTable` mit automatischer Breite, ohne Grenze.** Die Zerlegung bleibt, wie sie ist; `auszeichnungen_legen` schlüsselt sein `HashMap` nach Tabelle und Abschnitt statt nach Tabelle, und die Zeile in der Zelle zählt ab dem Anfang ihres Abschnitts.
   - Zeitzusagen: Jedes Bild kostet 6 bis 16 ms, unabhängig von der Größe der Tabelle (gemessen bis 348 002 Zellen). Linear bleibt das Setzen der Merkmale (0,43 s bei 124 320 Zellen, 1,36 s bei 348 002) und das volle Auslegen, das ein Sprung ans Ende erzwingt (2,2 s und 6,5 s). Das ist eine einmalige Pause und keine Blockade je Bild, aber über der Unmittelbarkeitsschwelle.
   - Kopieren: Keine Änderung. Text und Kachelung bleiben Zeichen für Zeichen dieselben; geteilt wird allein das `NSTextTable`, auf das die Blöcke zeigen.
   - Nutzersichtbare Texte: keine.
   - Gegen sie spricht: Jeder Abschnitt rechnet seine Breiten selbst. Eine Tabelle über 100 Zeilen kann ihre Spalten an jeder Abschnittsgrenze verschieben, wenn die Zellen dort breiter oder schmaler werden, und zwischen zwei Abschnitten steht eine doppelte Linie. Genau das Fluchten, das der Nutzer verlangt hat, hält dann nur abschnittsweise.

3. **Abschnitte wie in Möglichkeit 2, dazu eine Zellengrenze je Datei, darüber der Quelltext wie in Möglichkeit 1.** Unter der Grenze erscheint jede Tabelle ausgerichtet, eine bis 100 Zeilen genau wie heute. Die Grenze fängt allein die linearen Kosten ab, die die Abschnitte stehen lassen. Gemessen sind 16 004 Zellen mit 57 bis 67 ms für das Setzen und 32 004 Zellen mit 111 bis 124 ms. **20 000 Zellen** liegen dazwischen und sind daraus **interpoliert, nicht gemessen** (rund 80 ms auf dem M2 Max). Sie halten das Setzen auf dem Hauptfaden unter den 100 ms von L7. Für das Referenzgerät gilt der Vorbehalt aus Möglichkeit 1.
   - Zeitzusagen: Jedes Bild bleibt unter 16 ms, das Setzen unter 100 ms auf dem M2 Max. Das volle Auslegen bei 20 000 Zellen liegt nach der Messreihe bei rund 0,3 s, und es fällt allein an, wenn jemand ans Ende springt.
   - Kopieren: Wie in Möglichkeit 1 über der Grenze und wie in Möglichkeit 2 darunter. Beide Wege sind schon gebaut, keiner braucht eine neue Regel.
   - Nutzersichtbare Texte: Wie in Möglichkeit 1, ein Schlüssel in drei Tabellen, wenn ein Hinweis erscheinen soll.
   - Gegen sie spricht: Der Nachteil von Möglichkeit 2 an den Abschnittsgrenzen bleibt. Dazu kommen zwei Zahlen statt einer. Das ist dieselbe Form wie Text- und Bildgrenze im Modulkopf von `vorschaumodell.rs` („dieselbe Regel mit zwei Zahlen“), aber es ist eine Zahl mehr, die jemand pflegen muss.

4. **Nur die ersten 100 Zeilen als Tabelle, der Rest als Quelltext, mit Hinweis.** Eine Tabelle bis 100 Zeilen ist eine `NSTextTable` wie heute; von einer längeren werden die ersten 100 Zeilen ausgerichtet und die übrigen wörtlich gezeigt.
   - Zeitzusagen: Wie Möglichkeit 1 für jede Größe: ein Bild bleibt bei 13 ms, das Setzen klein.
   - Kopieren: Die Kachelung wechselt mitten in der Tabelle von Zellen auf wörtliche Zeilen. Beide Arten sind gebaut, ihr Übergang innerhalb einer Tabelle nicht. Eine Auswahl über die Nahtstelle braucht eine Probe, die es heute nicht gibt.
   - Nutzersichtbare Texte: Ein Hinweis ist hier Pflicht, sonst liest der Nutzer den Rest als kaputte Tabelle; ein Schlüssel in drei Tabellen, mit Zahl (`Zahlwort`).
   - Gegen sie spricht: Eine Tabelle, die halb ausgerichtet und halb roh ist, sieht aus wie ein Fehler, und der Bruch liegt an einer Stelle, die die Datei nicht vorgibt.

**Nicht zur Wahl gestellt, weil gemessen wirkungslos:** feste statt automatische Spaltenbreite (1 188 ms je Bild gegen 1 189 ms) und das Auslegen außerhalb des Hauptfadens. Die Zeit geht ins Zeichnen, und gezeichnet wird auf dem Hauptfaden. Dass ein `NSLayoutManager` an einer gezeigten `NSTextView` nicht von einem zweiten Faden ausgelegt werden darf, ist `inference:` aus der Dokumentation von TextKit 1 und nicht geprüft; die Messung allein schließt die Möglichkeit aus, weil das Auslegen nicht der Engpass ist.

**Eine Abwandlung von Möglichkeit 2 und 3** gäbe den Abschnitten feste Breiten in Prozent, aus den Zeichenzahlen der ganzen Tabelle gerechnet, damit die Spalten über die Abschnittsgrenzen fluchten. Gemessen ist, dass feste Breiten so schnell zeichnen wie automatische. Ungemessen ist, wie gut Zeichenzahlen in Proportionalschrift die Breiten treffen; der Datensatz zum Mittel hat genau diese Annäherung als unzuverlässig verworfen (`261031-1720-mit-welchem-mittel-richtet-die-vorschau-eine-markdown-tabelle-aus.md`, Möglichkeit 2). Sie steht deshalb nicht als eigene Möglichkeit hier.

## Constraints

- Die Zahlen stammen vom M2 Max. L7 und jede andere Zusage gelten auf dem Referenzgerät, und ein Abnahmelauf ist Nutzerarbeit (`261031-1720-schuldet-das-tabellenpaket-einen-abnahmelauf-gegen-die-zehn-zusagen.md`).
- Kopiert wird der Quelltext (C2 der Runde 14), die Kachelung des `Quellbezug` bleibt lückenlos, und die Klammerregel aus `260819-2216_*_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md` gilt.
- Ein nutzersichtbarer Text ist ein Eintrag aller drei Sprachtabellen und entsteht nirgends sonst (`crates/krk-core/src/sprache/mod.rs`, Modulkopf).
- Die Messprogramme zählen die Kosten von `grundschrift` und `grundabsatz` je Zelle nur in `mess2.swift` mit. Die Zahlen für das Setzen in Möglichkeit 2 und 3 setzen voraus, dass `261101-0015-zellenmerkmal-misst-je-tabellenzelle-schrift-und-leerzeichenbreite-neu.md` behoben ist; davor sind sie rund dreieinhalbmal so groß.

## Recommendation

**Möglichkeit 3**, mit 100 Zeilen je Abschnitt und 20 000 Zellen je Datei, die zweite Zahl als interpoliert gekennzeichnet, bis ein Lauf auf dem Referenzgerät sie bestätigt.

Die Ursache ist eine Last je Bild, die mit der Größe **einer** Tabelle wächst. Die Abschnitte nehmen sie vollständig weg, und das zeigt die Messung über die ganze Spanne bis 348 002 Zellen. Ein Abschnitt zu 100 Zeilen trägt jede Tabelle, die der Nutzer im Alltag schreibt, unverändert; der Nachteil an den Abschnittsgrenzen trifft erst Tabellen über 100 Zeilen. Eine Grenze allein (Möglichkeit 1) müsste bei 400 Zellen liegen, um ein Bild zu halten, und fiele damit mitten in gewöhnliche Dateien. Die Zellengrenze aus Möglichkeit 3 hat einen anderen Zweck: sie fängt die linearen Kosten ab, die auch geteilte Tabellen behalten, und liegt fünfzigmal höher. Möglichkeit 2 ohne sie ließe bei einer 1-MiB-Tabelle 0,4 bis 1,4 s Pause beim Öffnen und bis 6,5 s beim Sprung ans Ende stehen.

Ob über der Grenze ein Hinweis erscheint, ist eine Teilfrage, die der Nutzer mitbeantwortet. Wir empfehlen keinen: der Quelltext einer Tabelle ist lesbar und als das zu erkennen, was er ist, und ein Hinweis kostet drei Tabelleneinträge und eine Probe.

Möglichkeit 3 überholt nichts. Sie begrenzt die Reichweite von `261031-1720-mit-welchem-mittel-richtet-die-vorschau-eine-markdown-tabelle-aus.md` auf Tabellen unter der Grenze und schreibt aus, dass „ein `NSTextTable` je Tabelle“ dort „je Abschnitt“ heißt. Wer die Antwort einträgt, ergänzt jenen Datensatz um eine Zeile, die darauf verweist.
