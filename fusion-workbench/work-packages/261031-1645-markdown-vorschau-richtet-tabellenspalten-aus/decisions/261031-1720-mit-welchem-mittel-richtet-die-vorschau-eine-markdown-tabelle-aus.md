# Mit welchem Mittel richtet die gerenderte Vorschau die Spalten einer Markdown-Tabelle aus?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261031-1645-markdown-vorschau-richtet-tabellenspalten-aus.md, 261031-1720-plan-markdown-vorschau-richtet-tabellenspalten-aus.md, 260812-1851_*_welchen-umfang-von-markdown-rendert-die-vorschau-mit-verschachtelten-listen.md, 260812-1000_*_welchen-umfang-von-markdown-rendert-die-vorschau.md, 261018-0822-zeigt-die-vorschau-csv-und-tsv-als-ausgerichteten-text-oder-als-tabelle.md, 260812-1000_*_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md, 260819-2216_*_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md

---

## Question

Der Nutzer hat am 261031 verlangt, dass die gerenderte Markdown-Vorschau die Spalten einer Tabelle untereinanderstellt. Heute tut sie das mit Absicht nicht. Der Umfang der Vorschau ist seit dem 260812 entschieden, und Tabellen sind darin ausdrücklich draußen geblieben (`260812-1851_*_…-mit-verschachtelten-listen.md`, `## Antwort 260812`: „Tabellen und Bilder bleiben draußen“). Die Kiste `pulldown-cmark` läuft deshalb ohne `ENABLE_TABLES` (`Cargo.toml`, Absatz „Das Merkmal fuer Tabellen bleibt aus“ über der Zeile `pulldown-cmark = …`). Ohne das Merkmal liest sie die Zeilen einer Tabelle als gewöhnlichen Absatz mit weichen Umbrüchen, und `markdown::rendern` gibt sie wörtlich weiter (`crates/krk-ui/src/markdown.rs`, Zweig `Event::SoftBreak | Event::HardBreak`; die Probe `eine_tabelle_bleibt_ein_quelltextraster` hält genau das fest). Gesetzt wird dieser Text in der Systemschrift, also proportional (`appkit/textmerkmale.rs`, `grundmerkmale`: `(Ansicht::Format, Darstellungsart::Markdown) => (false, …)`). Proportional gesetzt stehen die Balken der Zeilen nicht übereinander, auch dann nicht, wenn der Quelltext mit Leerzeichen sauber aufgefüllt ist, weil ein Leerzeichen dort nicht so breit ist wie ein Buchstabe. Bei einer Tabelle ohne Auffüllung stehen sie erst recht nicht übereinander.

Der Begründung von damals ist inzwischen der Boden entzogen. Sie lautete, eine Ausrichtung brauche Tabulatorpositionen, und die Breite der Vorschau sei ungemessen. Die Breite ist weiterhin ungemessen, aber ein Mittel, das an ihr nicht hängt, stand damals nicht zur Wahl. Zu entscheiden ist jetzt, mit welchem Mittel ausgerichtet wird. Die Wahl bindet über dieses Paket hinaus. Sie überholt die Tabellenhälfte des Umfangsentscheids. Und sie klärt, wie weit der CSV-Entscheid E17 reicht, der von sich sagt, er binde, „wie jede spätere Tabellenart in der Vorschau erscheint“.

## Options

1. **Eine echte Texttabelle aus AppKit, `NSTextTable` mit einem `NSTextTableBlock` je Zelle.** Mit `ENABLE_TABLES` meldet die Kiste Tabelle, Kopf, Zeile und Zelle samt der Ausrichtung jeder Spalte aus der Trennzeile (`Tag::Table(Vec<Alignment>)`). `markdown::rendern` schreibt jede Zelle als eigenen Absatz und trägt sie als neue `Auszeichnung` mit Tabelle, Zeile, Spalte und Ausrichtung ein. `textmerkmale::anwenden` macht daraus einen Absatzstil mit `textBlocks` und `alignment`, ein `NSTextTable` je Tabelle. Die Breiten rechnet der Satzspiegel von AppKit mit den wirklichen Schriftmaßen und der wirklichen Breite der Fläche aus.
   - Pros: Die Spalten stehen in der Proportionalschrift untereinander, die der Rest der Vorschau trägt, und `:---`, `:---:` und `---:` werden zu links, zentriert und rechts. Wird die Vorschau schmal, bricht eine lange Zelle innerhalb ihrer Spalte um, statt die Zeile zu zerreißen. Betonung, Quelltext und Verweise in einer Zelle behalten ihre Auszeichnung, weil sie über dieselbe eine Umsetzung laufen. Der Text einer Zelle ist wörtlich ihr Quelltext, und das Kopieren aus C2 der Runde 14 bekommt dadurch keine neue Regel. Alle angesprochenen Klassen und Methoden stehen seit macOS 10.0 (`NSTextTable.h`, am SDK gelesen). Eine neue Kiste kommt nicht dazu.
   - Cons: Was AppKit daraus zeichnet, ist allein am laufenden Bündel zu sehen, und in dieser Umgebung lässt sich keine Probe unter `appkit::` fahren. `NSTextTable` gehört zu TextKit 1. Die Vorschaufläche läuft schon heute darin, weil `textmerkmale` auf ihren `layoutManager` zugreift (`inference:` aus der Dokumentation von AppKit; am Bündel ungeprüft). Ungemessen ist, wie schnell die automatische Breitenrechnung bei einer sehr großen Tabelle ist (siehe Constraints, L7). Und die Wahl weicht vom Weg ab, den E17 für CSV gegangen ist.
2. **Ein Raster aus fester Schrift wie bei CSV (E17).** Die Kiste meldet die Tabelle wie in Möglichkeit 1. `markdown::rendern` füllt die Zellen aber selbst mit Leerzeichen auf und setzt dafür die Breitenrechnung von `trenntextvorschau` (`zeichenbreite`, `HOECHSTBREITE`) wieder ein. Die Tabellenzeilen bekommen feste Schrift und einen Absatzstil ohne Umbruch.
   - Pros: Alles bleibt ohne AppKit prüfbar, auch die Ausrichtung jeder Zelle. Es ist der Weg, den E17 vorgezeichnet hat, und er bringt keine neue AppKit-Klasse.
   - Cons: Ob eine Spalte steht, entscheidet ein Zeichenzähler und nicht die Schrift. Das gilt nur für Zeichen, die die kleine Breitentafel kennt; bei kombinierenden Zeichen, Emoji und Fettschrift versagt es (das sagt der Modulkopf von `trenntextvorschau.rs` selbst). Eine Vorschau, schmaler als die Tabelle, kann nicht waagerecht rollen wie die CSV-Vorschau, weil der Fließtext um die Tabelle umbrechen muss. Die Zeilen werden deshalb abgeschnitten oder brechen um und zerreißen das Raster. Betonung in einer Zelle müsste auf feste Schrift umgestellt werden, sonst verschiebt sie die Spalte.
3. **Berechnete Tabulatorpositionen je Tabelle.** Zellen durch Tabulatoren trennen, die Stopps aus gemessenen Zellbreiten setzen, rechts- und zentrierte Spalten über `NSTextTab` mit eigener Ausrichtung.
   - Pros: Proportionalschrift, keine Tabellenklasse.
   - Cons: Die Breiten sind nur in AppKit messbar, und die Kernrechnung wandert damit unter `appkit/`. Bricht eine Zeile um, verliert die Fortsetzung ihre Spalte, und eine lange Zelle schiebt die nächste über ihren Stopp hinaus. Gerade das Umbrechen bei schmaler Vorschau ist damit nicht zu haben.

## Constraints

- Die Untergrenze ist macOS 15. Jede neu angesprochene Klasse und jeder hereingeholte Name steht namentlich im Untergrenzen-Abschnitt der Datei (`crates/krk-core/tests/baum.rs`, `jeder_frameworkimport_steht_namentlich_im_untergrenzen_abschnitt`).
- Kein neues `unsafe` außerhalb von `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/`. Eine neue Kiste nur mit Begründung in der Wurzel-`Cargo.toml`.
- Kopiert wird aus der gerenderten Vorschau der Quelltext (C2 der Runde 14). Die Kachelung des `Quellbezug` muss dafür lückenlos bleiben, und die Klammerregel des Nutzerentscheids `260819-2216_*_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md` gilt unverändert.
- L7 steht seit dem 261017 mit allen zehn Zusagen auf den Gegenständen der späteren Messrunde. Ein Abnahmelauf ist Nutzerarbeit.
- Die Mindestbreite der Vorschau bleibt bei 160 Punkten (`260812-1000_*_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md`, beantwortet). Dieses Paket fasst sie nicht an.

## Recommendation

**Möglichkeit 1.** Die Frage, die alles trägt, lautet: wo beginnt jede Zelle, damit ihre Spalte steht? Beantworten kann sie nur, wer die Schriftmaße und die Breite der Fläche kennt, und beides kennt allein der Satzspiegel von AppKit. Möglichkeit 2 nähert diese Antwort über gezählte Zeichen an. Das hält, solange die Schrift fest ist und jedes Zeichen in der Tafel steht. Und es hält nur, solange die Vorschau breiter ist als die Tabelle. Beides ist in einer Vorschau mit Fließtext und 160 Punkten Mindestbreite nicht gegeben. Möglichkeit 3 hat dasselbe Problem mit dem Umbruch.

**Zu E17:** Wir empfehlen, dessen Reichweite auf Tabellen zu begrenzen, die die ganze Datei sind (CSV, TSV, SQLite-Tabelle). Dort trägt sein Grund, denn die Fläche bricht nicht um und rollt waagerecht. Eine Tabelle mitten im Fließtext kann das nicht, und für sie gilt künftig dieser Datensatz. E17 selbst bleibt unverändert stehen.

**Was die Antwort zusätzlich überholt:** die Tabellenhälfte von `260812-1851_*_…-mit-verschachtelten-listen.md`. Bilder bleiben draußen; ihre Begründung (L7, weitere Lesevorgänge) berührt dieser Datensatz nicht. Wer die Antwort schreibt, trägt in jenen Datensatz die Zeile `Superseded by:` mit dem Hinweis ein, dass nur die Tabellenhälfte ersetzt ist. So hat es die Runde 6 schon einmal gehalten.

**Answered:** 261031 by user: „1“ auf die Vorlage mit A1 (echte Tabelle, NSTextTable), B1 (kein Messlauf) und C1 (Plan frei). Möglichkeit 1: NSTextTable mit einem NSTextTableBlock je Zelle. Die Tabellenhälfte von `260812-1851_*_…-mit-verschachtelten-listen.md` ist damit überholt, E17 gilt nur noch für Tabellen, die die ganze Datei sind.

**Implemented:** `83ee19dc` (NSTextTable je Tabelle, NSTextTableBlock je Zelle in `appkit/textmerkmale.rs`) und `fe3303c1` (ENABLE_TABLES und Tabellenzellen in `markdown.rs`). Die Tabellenhälfte von `260812-1851_*_welchen-umfang-von-markdown-rendert-die-vorschau-mit-verschachtelten-listen.md` trägt die Zeile `Superseded by:`.

**Begrenzt durch:** `261101-0015-ab-welcher-groesse-zeigt-die-vorschau-eine-markdown-tabelle-nicht-mehr-als-eine-nstexttable.md` (Nutzerantwort 261101, Möglichkeit 3): „ein NSTextTable je Tabelle“ heißt dort „je Abschnitt zu 100 Zeilen“, und eine Datei über 20 000 Tabellenzellen zeigt die Vorschau als Quelltext.
