# Der Spec zum Notizzettel steht

**Agent:** shaper (in-Circle, aktiver Circle)
**Zeitpunkt:** 260813-2348
**Status:** Complete
**Auftrag:** Den Spec für den aktiven Circle `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` schreiben, ohne eine der sieben beantworteten Klärungsfragen erneut zu stellen.

## Was vorlag

Der Circle-Datensatz mit Directive und einer Grounding-Aufnahme aus neun Abschnitten, dazu das eigene Sitzungsprotokoll der Anlage vom 260813-2332. Sieben Fragen waren in zwei Runden vor der Anlage beantwortet. Drei Folgen waren ausdrücklich in Kauf genommen (Absturzverlust, Überschreiben zwischen zwei Instanzen, die nackte Fläche als Bedingung der Blattform), drei Funde am Baum lagen vor, zwei bauanhaltende Stellen waren benannt, und eine Shaper-Vorgabe zum Sitzungszustand war zu bestätigen oder zu verwerfen.

## Was am Baum nachgesehen wurde

Fünfzehn Feststellungen stehen im Spec unter „Ausgangslage". Drei davon sind neu gegenüber der Grounding-Aufnahme und tragen den Zuschnitt:

- **`immer_erreichbar` führt genau drei Befehle**, `Beenden`, `FensterSchliessen`, `FensterEinblenden`, und `Abbrechen` steht **nicht** darauf (`kommandos/zulaessigkeit.rs:197`). Daraus folgt die Kette, die `Esc` trägt: solange die Textfläche des Zettels den Ersthelferrang hält, weist Bestandteil (2) den Abbruchbefehl ab, die Taste geht an AppKit, und der Wächter des Blattes schließt. Ohne die Nichtanmeldung in `ersthelfer_gehoert_appkit` kehrte sich beides um.
- **Daraus folgt eine Bedingung, die vorher nirgends stand:** der Schreibfokus muss nach jedem Tabklick in die Textfläche zurück. Hält der Tabschalter den Ersthelferrang, etwa unter vollständiger Tastaturbedienung, wird `Abbrechen` zulässig, KRK schluckt `Esc`, und der Zettel bleibt stehen. C2 sagt den Rücksprung deshalb ausdrücklich zu; er trägt zwei Kriterien zugleich.
- **Ein Blatt braucht 354 bis 403 ms, bis macOS es angehängt hat**, gemessen am 260804 und im Spec der Runde 1 unter L8 ausgeschrieben. Der Zettel erbt die Spanne. Das ist der sachliche Grund, aus dem diese Runde keine elfte Zahl setzen kann, und er steht neben dem bekannten Grund im Abschnitt zu den zehn Zeitzusagen.

Daneben nachgesehen: `Datei::ALLE` mit vier TOML-Dateien und elf Fundstellen über die Aufzählung in `tests/ablage.rs`; die Baumprobe mit ihren fünf Quelldateien; `Sitzung` mit `#[serde(default)]` an jeder Struktur; das Hauptmenü der Runde 7, das aus der Belegung entsteht und einem neuen Befehl seinen Eintrag ohne eine Zeile im Menübauer gibt; `resources/default-keymap.toml` mit 82 Funktionen und beiden gewählten Kombinationen frei; die sieben Automatiken in `textflaeche_bauen` mit ihrer Aufstellung von 36 Einstellungen; die drei `NSTextView` (Editor bearbeitbar, Vorschau nicht, Zettel künftig die zweite bearbeitbare); `Bereich` und `Fokus` mit je fünf Werten.

**Ein Befund gehört eigens genannt:** die Zusage zu den sieben Automatiken ist die einzige der Runde, für die der Baum heute nicht von selbst redet. `Datei::ALLE` und die Baumprobe halten den Bau an; eine zweite bearbeitbare Textfläche ohne abgeschaltete Automatiken übersetzt anstandslos. Der Spec macht daraus zwei Abnahmekriterien in C3 statt einer Zeile im Plan.

## Was geschrieben wurde

- **Spec** `planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md`. Fünf Fähigkeiten, jede mit zwei getrennten Kriterienlisten: am Baum nachweisbar und nur am laufenden Bündel prüfbar. Dazu zwei Mermaid-Bilder (der Weg der Tasten, die drei Sicherungsmomente als Zustandsautomat), der Abschnitt zu den zehn Zeitzusagen, der Abschnitt zu den vollständigen Fallunterscheidungen, Rahmenbedingungen, Abgrenzung, offene Punkte für den Planer.
- **Entscheidungsdatensatz** `decisions/260813-2348_o_was-tut-der-zettel-mit-einer-zetteldatei-die-er-nicht-lesen-kann.md`, offen, drei Möglichkeiten mit Empfehlung.

## Die Shaper-Vorgabe: bestätigt

Welcher Zettel zuletzt offen war, gehört als Feld in `Sitzung`. Bestätigt aus dem Vergleich mit den Alternativen: eine dritte Zetteldatei für eine einzige Zahl brauchte einen weiteren Eintrag in der Ablageaufzählung, einen weiteren Schreibweg und eine weitere Zeile in der Baumprobe; `settings.toml` schreibt KRK im Betrieb nicht; eine Marke im Text eines Zettels gehört dem Nutzer. Die zwei Folgen stehen als Festlegung in C2, darunter die, dass der Zwei-Sekunden-Takt die Merkung trägt und nie den Text. Der Planer darf die Vorgabe verwerfen, aber nicht stillschweigend.

## Warum nur ein Datensatz und nicht mehr

Vier weitere Punkte waren Kandidaten und sind als Vorbelegung mit Begründung in den Spec gegangen, weil der Baum für jeden eine Ordnung hat, an die sie sich anschließen: die sieben Automatiken (die Frage ist am Editor einmal beantwortet), die Beschriftung der Tabs (Nummerierung, benannte Zettel wären eine eigene Fähigkeit), der Funktionsbereich des neuen Befehls (der bestehende Bereich für die Anwendung, statt eines Obermenüs mit einem Eintrag) und die Form des Dateiinhalts (Text ohne Rahmen, weil eine Datei je Zettel sonst nichts gegenüber einer gemeinsamen gewinnt). Alle vier stehen als Vorbelegung da und sind am Spec-Gate umkehrbar.

Der eine Punkt, der ein Datensatz wurde, ist der, bei dem eine Vorbelegung Daten vernichten könnte: eine unlesbare Zetteldatei wird beim nächsten der drei Sicherungsmomente überschrieben, ohne dass der Nutzer etwas getan hätte.

## Was nicht geschrieben wurde

Kein Plan, kein Code, kein Defektdatensatz. Das Feld `**Active spec/plan:**` im Circle-Datensatz steht weiterhin auf `(none yet)`; das Nachtragen gehört dem Orchestrator und nicht dem shaper außerhalb des portfolio-activation-Modus.
