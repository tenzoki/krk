# Was steht in der Ausgabe, und wonach ist sie gegliedert?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/_a_circle.md` (Directive und Grounding), `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1054_c_belegungsansicht-gruppiert-nach-funktionsbereich.md` (der Nutzerauftrag, der die Bildschirmansicht gegliedert hat), `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_o_gehoert-der-wirkungsbereich-in-die-ausgabe.md` (eine dritte Spalte, getrennt gefragt)

---

## Question

Zwei Fragen über den Inhalt stehen zusammen, weil sie dieselbe Antwort tragen können: welche Funktionen die Ausgabe führt, und in welcher Ordnung. Beim Umfang geht es darum, ob unbelegte Funktionen mitkommen; bei der Ordnung darum, ob die Ausgabe nach Funktionsbereich gegliedert ist wie die Bildschirmansicht, nach Taste sortiert wie ein Nachschlagewerk, oder beides nacheinander führt.

Der Sachstand, am Code geprüft am 260809-2040: die Belegungsansicht führt genau eine Zeile je Funktion, gegliedert nach neun Funktionsbereichen, und innerhalb eines Bereichs in der Reihenfolge der Datei. Diese Gliederung geht auf einen Nutzerauftrag vom 260806 zurück. Ab Werk hat keine der 71 Funktionen eine leere Tastenliste; unbelegte Funktionen entstehen erst dadurch, dass der Nutzer eine Kombination entfernt oder eine Funktion in seiner `keymap.toml` nicht nennt.

Die Frage ist eine Frage nach dem Zweck der Datei. Wer sie neben die Tastatur legt, um eine Taste zu finden, will eine andere Ordnung als wer nachsehen will, was eine Funktion kann.

## Options

### Umfang

1. **Nur die belegten Funktionen.** Kürzer, und jede Zeile trägt eine Kombination.
   - Contra: eine Funktion, die der Nutzer versehentlich unbelegt gemacht hat, verschwindet spurlos aus der Ausgabe, statt dort als unbelegt aufzutauchen. Genau dafür führt die Bildschirmansicht sie mit leerer Belegungsspalte.
2. **Alle Funktionen, unbelegte mit leerem Feld oder dem Wort „unbelegt".** Dieselbe Zeilenmenge wie am Bildschirm.
   - Contra: bei einer unveränderten Auslieferungsbelegung ist der Unterschied null, der Zusatzaufwand also unsichtbar.

### Ordnung

1. **Nach Funktionsbereich, wie die Bildschirmansicht.** Neun Überschriften, darunter die Funktionen in der Reihenfolge der Datei.
   - Pro: dieselbe Ordnung wie am Bildschirm, also keine zweite Wahrheit darüber, wie KRKs Funktionen zusammengehören. Die Zuordnung steht bereits an einer Stelle und ohne Auffangzweig.
   - Contra: wer eine Taste sucht, muss neun Abschnitte durchgehen.
2. **Nach Taste, alphabetisch über die Kombinationen.** Eine Liste von Kombination zu Funktion.
   - Pro: das Nachschlagewerk für „was macht diese Taste".
   - Contra: eine Funktion mit zwei Kombinationen steht zweimal, und damit fällt die Ein-Zeilen-Regel aus C3. Die Sortierung braucht eine Ordnung über Kombinationen, die es im Projekt heute nicht gibt.
3. **Beides nacheinander in derselben Datei:** erst nach Funktionsbereich, dann eine zweite Tabelle nach Taste.
   - Pro: bedient beide Lesarten, und in einer Datei ist der Preis dafür ein Abschnitt, kein zweites Vorhaben.
   - Contra: die Datei wird doppelt so lang, und derselbe Bestand steht zweimal darin. Wer sie druckt, bezahlt es in Seiten.

## Constraints

- Die Zuordnung Funktion zu Funktionsbereich steht an genau einer Stelle, `bereich()` in `crates/krk-ui/src/belegungsmodell.rs`, als vollständige Fallunterscheidung ohne Auffangzweig. Eine Ausgabe, die eine eigene Gruppierung mitbrächte, wäre eine zweite Wahrheit darüber.
- Die Beschriftung einer Kombination kommt aus `anzeige()` und darüber aus der einen Tastentabelle in `parser::TASTEN`. Eine Übersetzungsliste daneben ist ausgeschlossen.
- Die Ausgabe verdrahtet keine Zahl fest: weder 71 Funktionen noch 79 Kombinationen noch neun Bereiche. Die laufende Editor-Runde hat alle drei Zahlen bewegt.

## Recommendation

**Wir empfehlen Umfang 2 und Ordnung 1**, also alle Funktionen, gegliedert nach Funktionsbereich. Beides folgt derselben Überlegung: die Ausgabe soll zeigen, was die Bildschirmansicht zeigt, und zwar auf Papier statt am Schirm. Jede Abweichung davon verlangt eine eigene Begründung und schafft eine Stelle, an der die beiden Darstellungen auseinanderlaufen können.

Ordnung 3, die zweite Tabelle nach Taste, empfehlen wir **nicht für den ersten Zuschnitt**, halten sie aber für die naheliegendste spätere Erweiterung. Sie braucht eine Ordnung über Kombinationen, die es noch nicht gibt, und sie beantwortet eine Frage, die der Nutzer bisher nicht gestellt hat.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: **Umfang: Moeglichkeit 1, nur die belegten Funktionen. Ordnung: Moeglichkeit 1, nach
Funktionsbereich wie die Bildschirmansicht.** Nutzerantwort am 260811-0110, festgehalten in
`history/260811-0107-orchestrator-session.md`.

**Zum Umfang.** Ab Werk aendert die Wahl nichts: keine der 71 Funktionen hat eine leere
Tastenliste, also fuehrt die Ausgabe ab Werk alle. Sichtbar wird sie erst, wenn der Nutzer eine
Kombination entfernt oder eine Funktion in seiner `keymap.toml` nicht nennt — dann faellt sie aus
der Datei. Der Preis ist, dass die Datei die Faehigkeit dann nicht mehr nennt; der Nutzer sieht
nicht, dass es sie gibt und dass sie gerade keine Taste hat.

**Zur Ordnung.** Neun Ueberschriften nach Funktionsbereich, darunter die Funktionen in der
Reihenfolge der Datei — dieselbe Gliederung, die die Belegungsansicht seit dem Nutzerauftrag vom
260806 traegt. Die Ausgabe ist damit zum Nachsehen gebaut, was eine Funktion kann, und nicht zum
Suchen einer Taste. Eine zweite Tabelle nach Taste ist nicht zugesagt.

**Anzumerken:** die Antwort auf `260809-2040_*_gehoert-der-wirkungsbereich-in-die-ausgabe.md`
lautet "als dritte Spalte". Die Ordnung folgt damit dem Schirm, der Spaltensatz nicht. Der Nutzer
hat beides gewaehlt, nachdem die Spannung benannt war; die Begruendung steht im dortigen
Datensatz.

---
Implemented: `fd863e3` — `belegungsausgabe::markdown` schreibt je besetztem Funktionsbereich einen
Abschnitt mit Pipe-Tabelle, in der Reihenfolge von `Funktionsbereich::ALLE` und innerhalb eines
Abschnitts in der Reihenfolge der Belegungsdatei; die Gliederung kommt aus
`belegungsmodell::nach_bereichen` (`belegungsmodell.rs:512`), also aus derselben Stelle wie
die Bildschirmansicht. Aufgenommen wird nur, was mindestens eine Kombination traegt. Am Baum
geprueft am 260811-1403.
