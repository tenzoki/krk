# Wie die Formatansicht ihre Auszeichnung setzt, und warum an zwei Orten

---
**Domain:** code
**Status:** implemented
**Filed by:** coder
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260808-0140_*_was-heisst-gerendert-bei-markdown-wenn-zugleich-bearbeitet-wird.md` (der abgelöste Datensatz; seine Antwort gilt unverändert weiter), `circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-0053_*_der-plan-legt-die-markdown-auszeichnung-in-voruebergehende-merkmale-und-die-tragen-sie-nicht.md`, `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md` (`### Frage 7`, S33), `crates/krk-ui/src/hervorhebung.rs` (Modulkopf), `crates/krk-ui/src/appkit/editor.rs` (`Editorbereich::formatierung_anwenden`)

---

## Question

Über welche Mechanik setzt die Formatansicht aus C3 ihre vier zugesagten Wirkungen, und trägt eine Mechanik alle vier?

Der abgelöste Datensatz vom 260808-0140 hat die Frage nach dem Sinn von "gerendert" beantwortet und dabei eine Annahme über den Weg mitgeführt: die vier Wirkungen — Überschriften größer und fett, Listen eingerückt, Links unterstrichen und eingefärbt, Quelltextblöcke in fester Schrift — liefen sämtlich über vorübergehende Merkmale des Layoutverwalters. Der Plan hat diese Annahme in `### Frage 7` und in S33 als tragende Begründung geführt: der Textspeicher bleibe unangetastet, und daran hänge die Zusage, dass die Auszeichnung beim Sichern nicht in die Datei geraten kann.

Die Annahme ist beim Bau von S33 am 260810-0053 am SDK widerlegt. Damit ist zu entscheiden, was an ihre Stelle tritt, und die Entscheidung ist nicht mehr offen: sie ist im selben Zug gebaut worden. Dieser Datensatz hält den gebauten Zustand fest, damit die nächste Runde nicht auf einer Zusage weiterbaut, die so nicht gilt.

## Options

1. **Zwei Orte, geschnitten entlang "wirkt auf die Auslegung oder nicht"** — Farbe und Unterstreichung als vorübergehende Merkmale im `NSLayoutManager`, Schriftgröße, Schriftschnitt, feste Schrift und Absatzeinzug über `addAttributes:range:` im `NSTextStorage`.
   - Pro: der Schnitt ist trennscharf und vollständig, und welche Seite ein Merkmal trägt, sagt AppKit selbst statt einer Liste in KRK. Alle vier zugesagten Wirkungen sind sichtbar. Die Zusage aus dem abgelösten Datensatz hält unverändert, weil sie nicht am Ort der Merkmale hängt.
   - Contra: es sind zwei Wege statt einem, und wer eine neue Wirkung nachträgt, muss die Seite bestimmen. Der Kopf von `NSLayoutManager.h` beantwortet das, aber man muss ihn kennen.

2. **Ein Ort, alles als vorübergehende Merkmale** — die Fassung, die der abgelöste Datensatz annahm.
   - Pro: eine Mechanik, kein Schnitt.
   - Contra: sie liefert drei der vier Wirkungen nicht. Ein layoutwirksames Merkmal, als vorübergehendes gesetzt, tut nichts. Nicht etwas Falsches, sondern gar nichts: `NSLayoutManager.h:351` sagt, erkannt werde für das Zeichnen allein, was die Auslegung nicht ändert. Die Formatansicht zeigte danach eingefärbten Text ohne größere Überschriften, ohne Listeneinzug und ohne feste Schrift für Quelltext, und das dritte Abnahmekriterium von C3 fiele.

3. **Ein Ort, alles im Textspeicher** — auch Farbe und Unterstreichung über `addAttributes:range:`.
   - Pro: eine Mechanik, kein Schnitt, und alle vier Wirkungen sind sichtbar.
   - Contra: sie gibt den einen Vorteil auf, den der Layoutverwalter für die Einfärbung hat. Vorübergehende Merkmale sind mit einem Aufruf über den ganzen Bereich zurückzusetzen und gehören keinem Dokumentzustand an; die Einfärbung wandert bei jedem Tastendruck und jedem Wechsel des Erscheinungsbildes neu, und sie so zu setzen hieße, den Textspeicher bei jedem Anschlag anzufassen. Der Weg ist gangbar und kostet ohne Not.

## Constraints

- Die Wahl des Nutzers vom 260808-0155 steht unverändert: die Auszeichnungszeichen bleiben stehen, und die ausgezeichneten Stellen bekommen ihre Wirkung. Dieser Datensatz ändert den Weg und nicht das Ergebnis.
- Das zehnte Abnahmekriterium von C3 verlangt, dass beide Ansichten auf demselben Stand arbeiten und nicht auf zwei Kopien. Ein Merkmal ist keine zweite Textkopie; Zeichen gibt es weiterhin genau einmal.
- Die Auszeichnung darf beim Sichern nicht in die Datei geraten. Gesichert wird `Editormodell::stand`, eine gewöhnliche Zeichenkette aus `NSTextView::string` (`crates/krk-ui/src/appkit/editor.rs:1099`). Kein Merkmal wird auf diesem Weg gelesen, gleich in welchem der beiden Speicher es liegt.
- Suchen und Ersetzen aus C5 beziehen sich nach dem siebten Abnahmekriterium von C5 auf den Text der Datei und nicht auf seine Darstellung. Auch das bleibt unberührt, weil beide über den gehaltenen Stand laufen und nicht über Merkmale.

## Recommendation

**Möglichkeit 1 ist gebaut, und sie ist die einzige, die alle vier zugesagten Wirkungen zeigt, ohne im Textspeicher zu schreiben, wo es nicht sein muss.** Der tragende Punkt ist, dass der Schnitt nicht erfunden ist: AppKit benennt beide Seiten selbst, und KRK führt keine eigene Liste, die auseinanderlaufen könnte.

Der Grund, den der abgelöste Datensatz für seine Empfehlung angeführt hat, war zur Hälfte falsch. Er lautete, Möglichkeit 1 halte "nicht durch Sorgfalt, sondern durch die Bauart", weil "der Textspeicher unangetastet bleibt". Der erste Halbsatz stimmt weiterhin, der zweite nicht: der Textspeicher trägt jetzt Merkmale. Was die Zusage trägt, ist nicht der unangetastete Textspeicher, sondern der Sicherungsweg, der Zeichen liest und keine Merkmale. Der Unterschied zählt für die nächste Runde: wer die Zusage am Ort der Merkmale festmacht, hält sie für gebrochen, sobald ein Merkmal in den Textspeicher wandert, und baut eine Vorkehrung, die nichts abwehrt.

---
Answered: circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-0053_*_der-plan-legt-die-markdown-auszeichnung-in-voruebergehende-merkmale-und-die-tragen-sie-nicht.md — Möglichkeit 1: die Fallunterscheidung heißt "wirkt auf die Auslegung oder nicht", Farbe und Unterstreichung gehen in den Layoutverwalter, Schriftgröße, Schriftschnitt, feste Schrift und Einzug in den `NSTextStorage`. Gemessen am SDK-Kopf `MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/NSLayoutManager.h:351`.
Implemented: 41309cc — `crates/krk-ui/src/appkit/editor.rs` (`Editorbereich::formatierung_anwenden`) setzt die Auszeichnungen über `addAttributes_range` im `NSTextStorage` und die Einfärbungen über `setTemporaryAttributes_forCharacterRange` und `addTemporaryAttribute_value_forCharacterRange` im Layoutverwalter; `crates/krk-ui/src/hervorhebung.rs` liefert dafür zwei Listen, und sein Modulkopf trägt den Schnitt samt der Zeile aus dem SDK-Kopf. Plan und Datensatz sind am 260810-0822 nachgezogen.

---

## Abgleich am 260810: die Antwort hält, die Belegzeile ist nachgezogen

**Der Schnitt "wirkt auf die Auslegung oder nicht" ist unberührt.** Was sich verschoben hat, ist die Stelle, an der die Zeile oben ihn festmacht, und zwar durch die Behebung von `issues/260810-1245_*_die-formatansicht-nimmt-gesetzte-merkmale-des-textspeichers-nie-wieder-heraus.md` in dieser Runde. Der Marker `_i_` bleibt richtig, weil die Umsetzung besteht; nachzutragen sind zwei Pfade:

- `formatierung_anwenden` setzt nicht mehr selbst zurück. `crates/krk-ui/src/appkit/editor.rs:2815` trägt dafür die neue Funktion `merkmale_zuruecksetzen`, und `setTemporaryAttributes_forCharacterRange` steht seither allein dort (`:2827`).
- `formatierung_anwenden` liegt jetzt bei `crates/krk-ui/src/appkit/editor.rs:2968`; es ruft `merkmale_zuruecksetzen` und setzt danach (`:3018`, `:3035`, `:3041`).

Die Zeile `crates/krk-ui/src/appkit/editor.rs:1099` in der Aufzählung oben, mit der der Sicherungsweg belegt ist, ist ebenfalls gewandert. Tragend an ihr ist nicht die Nummer, sondern dass gesichert wird, was `NSTextView::string` liefert; das gilt unverändert.

**`issues/260810-1139_*` widerspricht diesem Datensatz nicht.** Es hat eine `SAFETY`-Begründung am Setzen der Auszeichnungen berichtigt, über die dieser Datensatz nichts behauptet.

Geprüft am 260810 im Abschluss-Abgleich der Sitzung 260810-0845.
