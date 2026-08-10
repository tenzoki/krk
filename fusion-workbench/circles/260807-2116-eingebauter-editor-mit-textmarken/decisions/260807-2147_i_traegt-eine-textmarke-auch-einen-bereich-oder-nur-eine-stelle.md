# Trägt eine Textmarke auch einen Textbereich, oder nur eine Stelle?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/_t_circle.md` §`## Directive`, `circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md` §"4. Textmarke", `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` (C6 und `## Abgleich mit der Circle-Directive`)

---

## Question

Die Directive dieses Circles sagt, der Editor "setzt Marken auf Textstellen **und Textbereiche**". Dieselbe Formulierung steht in der Directive der Runde 1 und im ursprünglichen Entwurf `idea.txt`.

Die Festlegung des Nutzers vom 260807-2139 spricht nur noch von einer Stelle: eine Marke merkt sich eine Zeilennummer und den Textinhalt jener Zeile, der Sprung geht zur gemerkten Zeile, prüft den dort gemerkten Text und sucht bei Abweichung in der Nähe. Ein Bereich über mehrere Zeilen kommt darin nicht vor.

Beide Aussagen stammen vom Nutzer, die zweite ist die jüngere. Der Spec folgt der jüngeren und sagt in C6 die Marke auf eine Stelle zu. Ob das gewollt ist oder ob der Bereich beim Beantworten der vier Aktivierungsfragen nur unerwähnt blieb, muss der Nutzer sagen, denn die Antwort ändert C6 und die Ablageform in `bookmarks.toml`.

Der sachliche Kern des Unterschieds: eine Stelle hat einen Anker, ein Bereich hat zwei. Bei zwei Ankern entsteht die Frage, was gilt, wenn nur einer von beiden nach einer Änderung von außen wiedergefunden wird. Diese Frage hat die Festlegung vom 260807-2139 nicht beantwortet, weil sie sich bei einer Stelle nicht stellt.

## Options

1. **Nur eine Stelle, wie der Spec sie heute führt** — eine Marke ist eine Zeile. Die Directive-Formulierung "und Textbereiche" gilt als überholt und wird beim nächsten Anfassen des Circle-Datensatzes gestrichen.
   - Pro: eine Ankerregel, eine Prüfung, ein Fehlschlagfall. Die Festlegung vom 260807-2139 ist unverändert umsetzbar.
   - Contra: die Directive verspricht etwas, das die Runde nicht liefert, und der Circle-Datensatz muss angefasst werden, damit der Widerspruch nicht stehen bleibt.

2. **Stelle und Bereich als zwei Sorten Textmarke** — eine Marke merkt sich entweder eine Zeile oder eine Anfangs- und eine Endzeile, jeweils mit ihrem gemerkten Inhalt. Bei der Auswahl springt die erste an die Stelle, die zweite wählt den Bereich aus.
   - Pro: löst die Directive ein. Ein markierter Abschnitt ist beim Durchsehen längerer Dateien nützlicher als eine Zeile.
   - Contra: die Lesezeichenliste trüge damit drei Sorten statt zwei, und die Gültigkeitsprüfung bekäme eine dritte Fallunterscheidung. Dazu die offene Folgefrage, was gilt, wenn nach einer Änderung von außen nur einer der beiden Anker wiedergefunden wird; sie braucht eine eigene Regel, und drei Antworten sind denkbar (nur bis zum gefundenen Anker auswählen, gar nicht auswählen und melden, oder die alte Zeilenzahl auf den gefundenen Anfang anwenden).

3. **Der Bereich kommt in eine spätere Runde** — diese Runde baut die Stelle, die Ablageform in `bookmarks.toml` wird aber so gewählt, dass ein zweiter Anker später ohne Bruch danebentritt.
   - Pro: hält den Umfang dieser Runde und verbaut nichts. Die Directive bleibt eingelöst, nur später.
   - Contra: eine Ablageform auf Vorrat zu entwerfen heißt, für eine Fähigkeit zu planen, deren Regeln noch niemand kennt. Die offene Folgefrage aus Möglichkeit 2 bliebe unbeantwortet und würde die Form mitbestimmen, sobald sie beantwortet ist.

## Constraints

- Die Festlegung vom 260807-2139 steht und ist die jüngere Aussage: Zeilennummer plus Textinhalt als Prüfung, Sprung, Prüfung, Suche in der Nähe.
- Textmarken stehen in derselben Liste, derselben Ordnung und derselben Datei wie die Ordner-Lesezeichen. Eine zusätzliche Sorte erhöht die Zahl der Fälle in der gemeinsamen Gültigkeitsprüfung.
- `bookmarks.toml` bleibt von Hand lesbar.
- Eine bestehende `bookmarks.toml` mit Ordner-Lesezeichen muss unverändert eingelesen werden, gleich welche Antwort gewählt wird.

## Recommendation

Wir empfehlen Möglichkeit 1 und das Streichen der Formulierung in der Directive, falls der Nutzer zustimmt. Der Grund ist nicht der Aufwand, sondern die unbeantwortete Folgefrage: ein Bereich mit zwei Ankern braucht eine Regel für den halb wiedergefundenen Fall, und diese Regel ist nicht ableitbar, sondern zu entscheiden. Sie jetzt mitzuentscheiden hieße, eine fünfte Frage vor den Plan zu stellen; sie später zu entscheiden hieße, mit einer Ablageform zu beginnen, die von ihr abhängt.

Möglichkeit 3 empfehlen wir nicht, und zwar aus demselben Grund. Eine Ablageform, die eine noch unentschiedene Regel vorwegnimmt, ist keine Vorbereitung, sondern eine Wette.

Die Empfehlung setzt voraus, dass dem Nutzer die Stelle genügt. Das wissen wir nicht; er hat den Bereich zweimal aufgeschrieben und einmal nicht erwähnt.

---
Answered: circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md §"8. Textmarke: nur eine Stelle" — Möglichkeit 1 gewählt, samt der Empfehlung: eine Marke ist eine Zeile, und die Formulierung "und Textbereiche" in der Directive dieses Circles gilt als überholt und ist im Circle-Datensatz zu streichen. Tragender Grund ist nicht der Aufwand, sondern die unbeantwortete Folgefrage des Bereichs (was gilt, wenn von zwei Ankern nur einer wiedergefunden wird); sie ist zu entscheiden und nicht abzuleiten. Entschieden vom Nutzer am 260808-0017.
Implemented: `65c8efa` und `0ad7f29` — `crates/krk-core/src/ablage/lesezeichen.rs:105` führt `Ziel` mit zwei Werten, `:119-132` trägt `Textstelle { datei, zeile, zeileninhalt }` mit genau einem Anker und keinem zweiten; der Konstruktor `Lesezeichen::textstelle` (`:163`) nimmt dieselben drei Felder. Die verlangte Streichung in der Directive ist ausgeführt: `_t_circle.md:14` sagt „setzt Marken auf Textstellen" ohne den Zusatz „und Textbereiche". Planschritte S11, S38 und S39 tragen `[DONE]`. Nachgeprüft im Abgleich am 260810.
