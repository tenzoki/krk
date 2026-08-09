# Was heißt "gerendert" bei Markdown, wenn in derselben Ansicht bearbeitet wird?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_*_spec-eingebauter-editor-mit-textmarken.md` (C3, drittes und zehntes Abnahmekriterium), `shared/decisions/260802-0842_*_editor-formatansicht-je-dateityp.md` (die beantwortete Vorfrage, Möglichkeit 1 gewählt), `circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md` §"1. Formatansicht", `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md` (`### Frage 7` und S33)

---

## Question

Zwei Abnahmekriterien von C3 lassen sich nicht beide in ihrer stärksten Lesart einlösen, und der Spec sagt nicht, welches nachgibt.

Das dritte sagt: "Bei einer Markdown-Datei zeigt die Formatansicht das gerenderte Dokument mit Überschriften, Listen und Links."

Das zehnte sagt: "Der Wechsel zwischen den Ansichten verliert keine ungesicherte Änderung. Beide Ansichten arbeiten auf demselben Stand und nicht auf zwei Kopien." Dazu das neunte: "In beiden Ansichten lässt sich bearbeiten."

In der stärksten Lesart des dritten heißt "gerendert", was ein Betrachter zeigt: die Auszeichnungszeichen sind fort, `# Überschrift` erscheint als große Zeile ohne Doppelkreuz, `[Text](Ziel)` als blauer Text ohne Klammern. Diese Lesart bricht das zehnte. Was in der Ansicht steht, ist dann nicht mehr der Stand der Datei, sondern eine Darstellung davon, und wer darin tippt, tippt in die Darstellung. Zwischen Darstellung und Stand müsste danach eine Rückrechnung stehen, und die ist bei Markdown nicht eindeutig: aus fettem Text lässt sich nicht ablesen, ob er im Quelltext mit zwei Sternchen oder zwei Unterstrichen gesetzt war.

Die Frage ist zu stellen, weil der Nutzer sich am 260807-2139 ausdrücklich **gegen** die schreibgeschützte Leseansicht entschieden hat, die diesen Widerspruch nicht hätte. Der Gegenwert der Wahl, den die Sitzungshistorie im Wortlaut festhält, war: "in der Formatansicht bleibt das Bearbeiten möglich". Genau dieser Gegenwert steht zur Debatte, wenn "gerendert" in der stärksten Lesart gilt.

Sie hält keinen Planschritt auf und bindet einen, S33.

## Options

1. **Auszeichnung sichtbar lassen, Wirkung zeigen** — die Quelltextzeichen bleiben stehen, und die ausgezeichneten Stellen bekommen ihre Wirkung: Überschriften größer und fett, Listen eingerückt mit abgesetztem Aufzählungszeichen, Links unterstrichen und eingefärbt, Quelltextblöcke in fester Schrift. Das ist die Form, die Obsidian, Bear und Typora in ihrer Bearbeitungsansicht tragen.
   - Pro: der Stand in der Ansicht **ist** der Stand der Datei, Zeichen für Zeichen. Das zehnte und das neunte Abnahmekriterium halten ohne jede Rückrechnung, die Schreibmarke steht nach dem Umschalten an derselben Stelle, und Suchen und Ersetzen beziehen sich auf denselben Text wie in der Rohansicht. Es ist dieselbe Mechanik, die Code und einfacher Text schon benutzen, nämlich vorübergehende Merkmale im Layoutverwalter; kein zweiter Weg entsteht.
   - Contra: das dritte Abnahmekriterium wird in seiner schwächeren Lesart erfüllt. Wer "gerendert" als "wie im Browser" gemeint hat, bekommt es nicht. Bei einfachem Text war der Unterschied zwischen den Ansichten laut Datensatz ohnehin schwach; hier ist er sichtbar, aber kleiner als erwartet.

2. **Vollständig rendern und dafür die Bearbeitung in der Formatansicht aufgeben** — Markdown erscheint als Betrachterausgabe, die Auszeichnungszeichen sind fort, und in der Formatansicht lässt sich nicht tippen. Bearbeitet wird in der Rohansicht.
   - Pro: das dritte Abnahmekriterium in seiner stärksten Lesart. Das zehnte hält ebenfalls, weil eine schreibgeschützte Darstellung keinen zweiten Stand hält, sondern nur zeigt.
   - Contra: das neunte Abnahmekriterium fällt, und mit ihm der Gegenwert, für den der Nutzer am 260807-2139 die erste Möglichkeit gewählt hat. Es wäre die Rückkehr zur dritten Möglichkeit des Vorfragedatensatzes für einen von drei Dateitypen, also eine Sonderregel für Markdown neben zwei anderen Dateitypen ohne sie.

3. **Vollständig rendern und die Bearbeitung durch Rückrechnung halten** — die Formatansicht zeigt die Betrachterausgabe, und was der Nutzer darin tippt, wird in den Quelltext zurückgerechnet.
   - Pro: beide Kriterien in ihrer stärksten Lesart, jedenfalls dem Wortlaut nach.
   - Contra: die Rückrechnung ist nicht eindeutig, und das ist keine Frage des Aufwands, sondern der Sache. Aus fettem Text folgt nicht, ob zwei Sternchen oder zwei Unterstriche im Quelltext standen; aus einer Liste nicht, ob Bindestrich, Stern oder Plus sie einleitete; aus einer Überschrift nicht, ob sie mit Doppelkreuzen oder mit einer Unterstreichung gesetzt war. Jede Wahl der Rückrechnung schreibt Zeilen um, die der Nutzer nicht angefasst hat — derselbe Schaden, gegen den die Antwort vom 260808-0043 zur Sicherungsform gerichtet war, nur größer. Es ist ein eigenes Vorhaben, und ein großes.

## Constraints

- Die Wahl vom 260807-2139 steht: eine Formatansicht je Dateityp, Markdown gerendert, Code mit Syntaxhervorhebung, einfacher Text mit Umbruch und lesbarer Schriftgröße. Der Nutzer ist der Empfehlung des Vorfragedatensatzes nicht gefolgt, und der dort benannte Gegenwert war das Bearbeiten in der Formatansicht.
- Suchen und Ersetzen aus C5 beziehen sich nach dem siebten Abnahmekriterium von C5 "auf den Text der Datei, nicht auf seine Darstellung", und sie wirken nach demselben Kriterium in beiden Ansichten. Eine Antwort, die in der Formatansicht einen anderen Text zeigt als die Datei trägt, macht diese Zusage in der Formatansicht bedeutungslos.
- Der Editor hält nach der Antwort vom 260808-0043 einen Stand, der beim Sichern in einer festen Form geschrieben wird. Was der Editor beim Lesen nicht behält, kann er beim Sichern nicht zurückgeben; dasselbe gilt für die Auszeichnung.
- Die einklappbaren Blöcke sind am 260808-0017 aus dieser Runde herausgenommen worden, mit der Begründung, Hervorhebung brauche Wortarten und Einklappen brauche Blockgrenzen. Dieselbe Trennung greift hier: die gewählte Kiste liefert Wortarten und keine Dokumentstruktur.

## Recommendation

**Wir empfehlen Möglichkeit 1.** Sie ist die einzige der drei, die alle drei berührten Abnahmekriterien zugleich hält, und sie hält sie nicht durch Sorgfalt, sondern durch die Bauart: weil der Textspeicher unangetastet bleibt und die Auszeichnung als vorübergehende Merkmale im Layoutverwalter liegt, kann kein Weg entstehen, auf dem Darstellung und Stand auseinanderlaufen.

Sie ist außerdem die einzige, die Markdown nicht zum Sonderfall macht. Einfacher Text, Code und Markdown laufen danach durch dieselbe Mechanik, und der Unterschied zwischen ihnen ist, welche Merkmale gesetzt werden, nicht welcher Weg gegangen wird. Der Plan nennt das unter `## Wie dieser Plan die Maxime "supersimpel" einlöst` als eine von vier Stellen.

**Möglichkeit 2 empfehlen wir nicht**, weil sie die Wahl vom 260807-2139 für einen Dateityp zurücknimmt, ohne dass jemand darum gebeten hätte. **Möglichkeit 3 empfehlen wir nicht**, weil ihr Kern nicht die Arbeit ist, sondern eine Rückrechnung, die keine eindeutige Antwort hat; das ist kein Aufwand, den man aufbringt, sondern eine Frage, die man anders stellen muss.

`inference:` Wir schließen aus der Formulierung "mit Überschriften, Listen und Links", dass der Nutzer die sichtbare Wirkung meinte und nicht das Verschwinden der Quelltextzeichen; die drei genannten Sachen sind genau die, die Möglichkeit 1 sichtbar macht. Geprüft ist das nicht, und deshalb steht die Frage hier statt in einer Fußnote des Plans.

---
Answered: circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md §"13. Gerendert bei Markdown" — Möglichkeit 1 gewählt: die Auszeichnungszeichen bleiben stehen, und die ausgezeichneten Stellen bekommen ihre Wirkung (Überschriften größer und fett, Listen eingerückt, Links unterstrichen und eingefärbt, Quelltextblöcke in fester Schrift). Der Stand in der Ansicht ist damit der Stand der Datei, Zeichen für Zeichen, und es entsteht keine zweite Mechanik neben der, die Code und einfacher Text schon benutzen. Der Preis ist angenommen: wer "gerendert" als "wie im Browser" gemeint hat, bekommt das nicht; das dritte Abnahmekriterium hält in seiner schwächeren Lesart. Entschieden vom Nutzer am 260808-0155.
Implemented: 41309cc — die Formatansicht zeigt bei Markdown Überschriften größer und fett, Listen eingerückt, Links unterstrichen und eingefärbt, Quelltextblöcke in fester Schrift, und die Auszeichnungszeichen bleiben stehen. Der Stand in der Ansicht ist Zeichen für Zeichen der Stand der Datei; gesichert wird `Editormodell::stand`, der aus den Zeichen der Fläche kommt und nie aus ihren Merkmalen.

Nachtrag zur Umsetzung: die Annahme dieses Datensatzes, alle vier Wirkungen ließen sich über vorübergehende Merkmale des Layoutverwalters herstellen, ist am SDK widerlegt (`NSLayoutManager.h:351`: als vorübergehendes Merkmal beachtet wird allein, was die Auslegung nicht ändert). Drei der vier — Überschrift, Einzug, feste Schrift — gehen deshalb in den Textspeicher, Farbe und Unterstreichung in den Layoutverwalter. Die Zusage dieses Datensatzes ist davon unberührt, weil sie nicht an den Merkmalen hängt. Der Defekt `260810-0053` zieht Plan und Datensatztext nach.
