# Planner-Sitzung — 260808-0159

**Auftrag:** Den Umsetzungsplan für den aktiven Circle `260807-2116-eingebauter-editor-mit-textmarken` bauen
**Agent:** `planner`, ohne Rückfragewerkzeug
**Status:** Complete

## Ergebnis

`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, 42 Schritte in acht Phasen, vier Mermaid-Diagramme, alle mit `@mermaid-js/mermaid-cli` 11 geprüft. Jeder Schritt trägt `coder` als Ausführer; die Begründung dafür steht einmal im Abschnitt `## Aufbau`.

## Wie der Plan entstanden ist

Vor dem Entwurf lief eine Bestandsaufnahme am Code über vier Untersuchungen: der Weg vom Tastendruck zum Kommando, die Fensterzeile mit der Vorschau, die Ablage mit den Lesezeichen, und die Blätter mit dem Anwendungslebenszyklus. Sechs Befunde daraus ändern den Zuschnitt gegenüber dem, was der Spec annehmen konnte, und stehen im Plan unter `## Ausgangslage` statt verstreut in den Schritten.

Der schwerste ist Befund 4. **Die Fähigkeit C8 steht auf einer Voraussetzung, die am Code widerlegt ist.** Beide Verdächtigen des Defekts `shared/issues/260807-2112_*_cmd-y-und-shift-cmd-y-loesen-nichts-aus-f3-schon.md` scheiden aus: das Hauptmenü trägt kein Kürzel mit `y` und läuft ohnehin nach dem Ereignisabgriff (`menue.rs:184-252`, `:31-42`), und die Normalisierung vergleicht `u8` gegen `u8`, was der Prüfstein `f3` belegt — die Taste trägt das Funktionstastenbit und wirkt. Die Ursache ist die Tastaturbelegung: `kVK_ANSI_Y` trägt den Code 16 (`parser.rs:209`), und auf einer deutschen Tastatur steht an jener Stelle ein Z. Derselbe Befund ist am 260803-2317 schon einmal gefunden und am 260804-0830 vom Nutzer geschlossen worden, mit dem tragenden Grund, `f3` sei der Hauptweg und `cmd+y` der zweite. Dieser Grund trägt seit dem 260807 nicht mehr, weil `fokus_vorschau` an jenem Tag mit genau einer Kombination hinzugekommen ist.

## Die tragende Entwurfsentscheidung

Die Zeile `**Entscheidbarkeit:**` im Plankopf beantwortet C7: der Ereignisabgriff fragt heute nach der **Art** des Ersthelfers (`ersthelfer_nimmt_text`, `ereignisse.rs:374-395`) und kann damit zwei Objekte derselben Art nicht trennen. Er fragt künftig nach der **Nämlichkeit**, so wie `Anwendungsdelegierter::fokus` es seit der Runde 1 zweimal tut. Trennscharf, weil ein Objekt mit genau einem anderen identisch ist; vollständig, weil jeder Ersthelfer entweder jenes Objekt ist oder nicht.

Daraus fällt der Rest von C7 ohne einen einzigen Sonderfall an, weil ein nicht ausgeführtes Kommando das Ereignis nicht verbraucht und AppKit es danach der Textfläche zustellt. Drei Befehle laufen in die falsche Richtung und bekommen den neuen Wirkungsbereich `Navigator`; `Wirkungsbereich` wächst dadurch von vier auf sieben Werte, und das ist die größte einzelne Zunahme dieser Runde.

## Was der Plan wiederverwendet statt neu zu bauen

Vier Stellen entfernen einen Mechanismus oder benutzen einen bestehenden: die Literalliste der festen Randbereiche in `bereichsbreiten` verschwindet zugunsten des Filters über `ist_beweglich`; das Anlegen einer Textmarke bekommt keinen eigenen Befehl, sondern liegt auf `lesezeichen_anlegen`; die Einfärbung liegt als vorübergehende Merkmale im Layoutverwalter und nicht im Textspeicher, womit es keinen zweiten Textbestand gibt; und die Dateisystembeobachtung bekommt einen dritten Ordner statt eines zweiten Stroms.

## Angelegte Datensätze

Zwei, beide `_o_`, beide im Speicher dieses Circles.

- `decisions/260808-0140_o_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`
- `decisions/260808-0140_o_was-heisst-gerendert-bei-markdown-wenn-zugleich-bearbeitet-wird.md`

Kein Defekt ist angelegt worden. Zwei Nebenbefunde aus der Bestandsaufnahme sind im Plan unter `## Offene Fragen` genannt und gehören nicht in diese Runde: `Sitzungsschreiber::abgleichen` hat außerhalb der Proben keinen Aufrufer, und die drei Wörter " und Textbereiche" in der Directive des Circle-Datensatzes sind seit dem 260808-0017 überholt.

## Diagrammprüfung

Alle vier Blöcke parsen und rendern. Der Abhängigkeitsgraph trägt 42 Knoten und 53 Kanten, kein Zyklus, keine Waise, höchste Ausgangsverzweigung 5 an S16 und höchster Eingang 12 an S42; beide sind im Plan als sachlich richtig begründet. Ein erster Durchgang hatte sieben Waisen, die vor dem Abschluss nachgezogen wurden.
