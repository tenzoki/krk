# Shaper: Berichtigung von C3, die dritte Spalte trägt jetzt den gemessenen Stand

**Datum:** 2026-08-11, 10:38
**Agent:** shaper (user-direct, eng begrenzter Berichtigungsauftrag vom 260811-1005)
**Circle:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads`
**Berührte Datei:** `planning/260811-0753_o_spec-tastenbelegung-als-markdown-in-downloads.md` (Marker bleibt `_o_`)

## Anlass

Der Spec beschriftete die dritte Spalte für die sechs vom Hauptmenü zugestellten Textbefehle
einheitlich mit "Textfelder und Editor" und kennzeichnete das ausdrücklich als Ableitung des
Shapers, nicht als Messung. Sein Nachweiskriterium verlangte die Prüfung vor dem Bau. Die
Prüfung ist am 260811-0930 gefahren (Plan-Schritt S1, `AnyClass::responds_to` gegen die sechs
Klassen, die in KRK einen Ersthelfer stellen können) und hat die Ableitung zerlegt. Der Spec
stand damit gegen den Code, und wer die Abnahme an ihm fährt, hätte falsch abgehakt.

## Was an C3 geändert wurde

- **Beschreibung:** ein zweiter Absatz sagt, dass die dritte Spalte ihre Zellen aus
  verschiedenen Quellen bezieht und eine davon leer bleibt.
- **Kriterium zur ausgeschriebenen Beschriftung** auf "Wo die dritte Spalte etwas trägt"
  eingeschränkt, weil eine Zelle jetzt leer ist.
- **Das Kriterium zur einheitlichen Zelle** ist durch die Dreiteilung ersetzt, je Befehl
  entschieden: drei tragen "Textfelder und Editor", `text_alles_auswaehlen` bleibt leer,
  `text_rueckgaengig` und `text_wiederholen` tragen "Editor".
- **Ein neues Kriterium für die fünfte Zelle**, die der Spec bisher nicht kannte: eine von Hand
  geschriebene `keymap.toml` kann einer Funktion mit Kommando einen Zusteller geben, und die
  Zelle sagt dann, dass KRK sie nicht einordnen konnte, statt leer zu bleiben. Der Wortlaut ist
  als Vorbelegung ausgewiesen; die offene Frage aus `issues/260811-0955_*_…`, ob der Fall besser
  ganz verschwindet, ist ausdrücklich nicht hier entschieden.
- **Das Nachweiskriterium** verlangt keine Prüfung mehr, sondern dass ihr Nachweis im Baum
  auffindbar ist und die Zellen der Datei zu den beiden Datensätzen passen. Es bleibt damit beim
  Abnahmelauf prüfbar.
- **Die Tabelle** heißt nicht mehr "Die sieben Beschriftungen", trägt eine Quellenspalte und
  vier zusätzliche Zeilen für die Fälle ohne Wirkungsbereich.
- **Fünf neue Festlegungen** halten fest, dass und wodurch die Ableitung gebrochen wurde: die
  Messung selbst, der Bruch an `NSTableView`, die Unentscheidbarkeit bei `undo:`/`redo:` samt
  Nutzerentscheid vom 260811-0935, der erschlossene Anteil an "Textfelder", und der Wert der
  Kennzeichnung.

**Abnahmekriterien von C3: 18 vorher, 19 nachher.** C1, C2 und C4 unberührt.

## Was außerhalb von C3 angefasst wurde

Zwei Stellen, beide weil sie C3 wörtlich widersprachen:

1. `## Was die Abnahme mitentscheidet`, der Abschnitt über die Ableitung — vom Futur ins
   Perfekt gezogen, der widerlegte Verdachtsfall als bestätigt ausgewiesen, die stehengebliebene
   Ordinalzahl ("das zwölfte Abnahmekriterium von C3", die ohnehin auf das vierzehnte zeigte)
   durch einen Verweis auf das letzte Kriterium der dritten Spalte ersetzt. Der Absatz über den
   angenommenen Preis und der über die sechs Vorbelegungen sind unberührt.
2. Der einleitende Blockzitat-Absatz oben im Spec, ein Nebensatz, der dieselbe Aussage trug.

## Was gesehen und nicht angefasst wurde

- `## Beantwortete Nutzerentscheidungen`, Zeile "Was steht dort bei den sechs Textbefehlen?
  'Textfelder und Editor', unter Vorbehalt der Prüfung." Als Bericht über die Antwort vom
  260811-0115 bleibt sie richtig, der Vorbehalt wurde gezogen. Die dreizehnte Festlegung, der
  Nutzerentscheid vom 260811-0935, steht in dieser Tabelle nicht, und der Vorspann spricht
  weiter von zwölf.
- Das Mermaid-Bild unter `## Aufbau dieser Runde` führt für die dritte Spalte nur die
  Beschriftung der Wirkungsbereiche als Quelle. Es war schon vor dieser Berichtigung unvollständig
  und ist durch sie nicht falscher geworden.
