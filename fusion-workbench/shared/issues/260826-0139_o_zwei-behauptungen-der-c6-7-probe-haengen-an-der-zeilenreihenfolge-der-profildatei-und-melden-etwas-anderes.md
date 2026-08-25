# Zwei Behauptungen der C6.7-Probe hängen an der Zeilenreihenfolge der Profildatei und melden etwas anderes

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Cross-references:** `crates/krk-core/tests/leseprofil.rs:3202-3208` (die `step_by(2)`-Behauptung am Speicherprofil); `:3265-3282` (die Werteliste des Projektwurzelprofils); `shared/reviews/260825-2230-coderev-nachdurchsicht-runde-18-drei-behebungen.md` (Abschnitt „`a9868a2`", erste Anmerkung ohne Datensatz); `resources/default-readers.toml:486-581` (das Speicherprofil), `:640-670` (das Projektwurzelprofil)

---

## Was ist

Die Probe `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`
prüft an zwei Stellen Werte über ihre **Stellung** in der Zusammenfassung, und
die Stellung kommt aus der Reihenfolge der Zeilen in
`resources/default-readers.toml`:

1. **Das Speicherprofil** (`:3202-3208`): `speicherwerte.iter().step_by(2)`
   setzt voraus, dass in jedem der zehn Unterspeicher die Zählungszeile vor der
   Datumszeile steht. Vertauscht jemand die zwei Zeilen eines Speichers, wird die
   Probe rot mit „die Zaehlungen sehen nicht je den einen Datensatz".
2. **Das Projektwurzelprofil** (`:3265-3282`, neu in `96e32cb`): die Werteliste
   steht als geordnete Folge aus fünf `Wert::Text` und zwei `Wert::Zahl` da.
   Stellt jemand die sieben Zeilen des Profils um, wird die Probe rot mit „die
   Projektwurzelzusammenfassung liefert nicht die Werte, für die sie gelesen
   hat; eine Zeile, die nichts findet, öffnet auch nichts".

Beide Meldungen benennen einen anderen Fehler als den, der vorliegt. Die erste
behauptet, eine Zählung sehe nicht den einen Datensatz; die zweite behauptet,
eine Zeile habe nichts gefunden. Vorliegend ist in beiden Fällen allein eine
geänderte Reihenfolge, bei der jede Zeile ihren Wert sehr wohl gefunden hat.

Die Beschriftungsliste darüber (`:3234-3245`) fängt eine Umstellung **nicht** ab:
sie vergleicht die gemessenen Beschriftungen gegen `profil.zeilen()` desselben
Profils, also gegen eine Liste, die sich mitdreht.

## Warum das zählt

Die Richtung stimmt: rot und nicht still grün. Beide Kopplungen sind damit
keine Lücke in der Abnahme, sondern eine Fehlerauskunft, die auf die falsche
Ursache zeigt. Wer die Meldung wörtlich nimmt, sucht den Fehler im
Baustein oder im Prüfordner und nicht in der Zeilenreihenfolge der Profildatei.

Kein Doc-Kommentar der Probe nennt die Kopplung. Der Kopf schreibt für den
vierten Fall ausdrücklich aus, **warum** die Werteliste ausgeschrieben und nicht
gegen `wurzelwerte` verglichen wird (`:3038-3046`), sagt aber nicht, dass sie
damit an der Reihenfolge hängt.

Der Befund ist reine Prüfdatei und erreicht kein ausgeliefertes Byte.

Schwere **gering**.

## Möglichkeiten

1. Beide Meldungen um den zweiten möglichen Grund ergänzen: „… oder die
   Reihenfolge der Zeilen in `default-readers.toml` hat sich geändert". Zwei
   Zeilen, keine Änderung an der Prüflogik.
2. Die Kopplung im Doc-Kommentar der Probe an einer Stelle ausschreiben, so wie
   der Kopf die anderen Voraussetzungen der Rechnung ausschreibt.
3. Die Behauptungen von der Stellung lösen, also gegen Paare aus Beschriftung
   und Wert prüfen statt gegen eine Folge von Werten. Das ist der einzige Weg,
   der die Kopplung wirklich aufhebt, und er kostet an beiden Stellen mehr
   Zeilen als die Auskunft wert ist, solange die Richtung rot bleibt.

Möglichkeit 1 und 2 zusammen kosten vier Zeilen und beheben, was hier stört,
nämlich die falsche Auskunft. Möglichkeit 3 ist die gründliche und für eine
Zusage, die ohnehin an der Datei hängt, nicht nötig.
