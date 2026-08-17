# `frage_und_erlaeuterung` trägt kein `#[must_use]`

**Datum:** 260817-1112
**Gefunden von:** coderev, Durchsicht `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`, Befund 7
**Schwere:** Niedrig
**Betrifft:** `crates/krk-ui/src/kommandos/loeschwarnung.rs`
**Baumstand:** `472eb81`

## Der Befund

Das Projekt setzt `#[must_use]` an jeden Rückgabewert, dessen stilles Fallenlassen unbemerkt
bliebe; so entschieden vom Nutzer am 260811-2140. Die Schwesterregel `rueckschritt` trägt es
samt ausgeschriebener Begründung (`rueckschritt.rs:142-145`).

`frage_und_erlaeuterung` (`loeschwarnung.rs:88`) ist eine reine Funktion. Ihr Ergebnis
fallenzulassen ist ein Aufruf ohne jede Wirkung, und der Übersetzer sagt dazu nichts, auch
nicht unter `-D warnings`.

## Das Gegengewicht

`operationen::loeschfrage` (`:485`), die Funktion, die sie ablöst, trägt es ebenfalls nicht.
Der heutige Zustand ist also nicht schlechter als der vorige.

## Warum es trotzdem steht

Bündel C legt zwei weitere Funktionen in dasselbe Modul, `warngruende` und die Tafel der
Auslöser, und der Plan verlangt `#[must_use]` für die Nachbarn dieser Kette ausdrücklich
(Schritt 5 für `fuehrt_einen_papierkorb`, Schritt 7 für `zaehlen`). Ein Modul mit drei
Funktionen und zwei Haltungen zu derselben Frage ist die Abweichung, die niemand prüft.
