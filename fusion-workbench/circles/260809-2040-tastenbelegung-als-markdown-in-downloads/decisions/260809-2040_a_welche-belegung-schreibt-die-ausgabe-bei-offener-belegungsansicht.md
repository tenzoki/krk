# Welche Belegung schreibt die Ausgabe, wenn die Belegungsansicht offen steht?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_o_wie-wird-die-ausgabe-der-belegung-ausgeloest.md` (diese Frage hängt an jener Antwort), `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/_a_circle.md` (Directive und Grounding)

---

## Question

Die Directive sagt, die Ausgabe zeige die Belegung, "die im Augenblick des Aufrufs gilt". Für einen Aufruf aus dem gewöhnlichen Betrieb ist das eindeutig: es gibt genau einen Wert, den `belegung::fuer_den_betrieb()` beim Start gebaut hat. Solange die **Belegungsansicht** offen steht, gibt es zwei. Das Blatt hält eine Arbeitskopie (`Belegungsmodell`), auf der die Zuweisungen des Nutzers landen; gesichert wird sie erst beim Verlassen über `Belegung::sichern`. Wer währenddessen eine Ausgabe auslöst, bekäme entweder den gesicherten Stand oder den ungesicherten, und die beiden können auseinanderliegen.

**Die Frage wird erst durch die erste Frage dieses Circles scharf.** `inference:` Wird die Ausgabe als gewöhnliche Funktion mit Kommando gebaut, ist sie bei offenem Blatt gar nicht auslösbar: der Ereignisabgriff führt dann nichts aus und reicht jeden Tastendruck an AppKit weiter, allein `abbrechen` kommt durch (`crates/krk-ui/src/appkit/belegungsansicht.rs`, Modulkopf). Die Frage wäre damit gegenstandslos. Wird sie dagegen als Menüeintrag gebaut, läuft die Antwortkette weiter, und der Fall tritt ein.

`speculation:` Ob ein Menüeintrag bei stehendem Blatt tatsächlich noch anschlägt, ist nicht gemessen. Das Blatt ist über `super::blaetter::Blatt` gebaut; ob es das Hauptmenü sperrt, steht in `crates/krk-ui/src/appkit/blaetter/mod.rs` und ist vor der Antwort nachzusehen.

## Options

1. **Den gesicherten Stand schreiben**, also die Belegung des Betriebs, nicht die Arbeitskopie.
   - Pro: die Datei zeigt, was KRK gerade wirklich tut. Für eine Ausgabe, die man ausdruckt und neben die Tastatur legt, ist das der richtige Stand.
   - Contra: der Nutzer hat gerade drei Tasten umbelegt und bekommt eine Datei ohne diese drei, ohne Hinweis darauf. Das ist überraschend.
2. **Die Arbeitskopie schreiben**, also einschließlich der noch nicht gesicherten Änderungen.
   - Pro: die Datei zeigt, was der Nutzer vor Augen hat. Wer die Ansicht offen hat, meint sie, wenn er "die Belegung" sagt.
   - Contra: die Datei sagt etwas zu, das noch nicht gilt, und nach einem Abbruch der Ansicht gilt es nie. Der Dateiname trägt keinen Hinweis darauf.
3. **Bei offener Ansicht gar nicht ausgeben und es in der Statuszeile sagen.**
   - Pro: keine der beiden Überraschungen, und die Regel ist in einem Satz erklärbar.
   - Contra: eine Sonderregel für einen Zustand, dazu eine Meldung, die es sonst nirgends gibt. Für einen Fall, den Möglichkeit 1 der ersten Frage von selbst ausschließt, ist das teuer.

## Constraints

- Die Antwort muss zur Antwort auf `260809-2040_o_wie-wird-die-ausgabe-der-belegung-ausgeloest.md` passen. Fällt jene auf die gewöhnliche Funktion mit Kommando, ist diese Frage ohne Gegenstand und wird zurückgestellt statt beantwortet.
- Die Belegungsansicht hält keine eigene Tabelle. Sie reicht jede Frage an ihre Arbeitskopie weiter; beide Stände sind also derselbe Typ, und die Ausgabe kann von jedem der beiden schreiben, ohne zwei Wege zu bauen.

## Recommendation

Wir empfehlen, diese Frage **nach** der ersten zu beantworten und nicht vor ihr. Fällt die erste auf die gewöhnliche Funktion mit Kommando, wird diese hier zurückgestellt (`_d_`) mit dem Vermerk, dass der Zustand nicht erreichbar ist; sie kehrt zurück, sobald ein Menüeintrag danebentritt.

Tritt der Fall ein, empfehlen wir **Möglichkeit 1**, den gesicherten Stand. Die Ausgabe ist eine Aussage darüber, was KRK tut, und nicht darüber, was der Nutzer gerade vorhat. Eine Datei, die eine Umbelegung zusagt, die der Nutzer anschließend mit `esc` verwirft, ist die schlechtere der beiden Überraschungen.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: **Moeglichkeit 1, den gesicherten Stand schreiben.** Nutzerantwort am 260811-0110,
festgehalten in `history/260811-0107-orchestrator-session.md`.

**Die Frage ist scharf und nicht gegenstandslos**, weil der Nutzer den Menueweg gewaehlt hat.
Der Datensatz hatte das oben als `inference:` offengelassen und verlangt, es vor der Antwort
nachzusehen. Nachgesehen am 260811-0107: die Belegungsansicht wird ueber
`beginSheetModalForWindow_completionHandler` gezeigt (`crates/krk-ui/src/appkit/blaetter/mod.rs:508`),
ist also **dokumentmodal** und bringt keine eigene Ereignisschleife mit; eine eigene
`validateMenuItem`-Ueberschreibung gibt es im Baum nicht. `inference:`, nicht gemessen: ein
dokumentmodales Blatt laesst die Menueleiste bedienbar, und der Eintrag schlaegt an. Der
Belegungsweg haette die Frage erledigt, der Menueweg tut es nicht.

**Geschrieben wird die Belegung des Betriebs**, also der Wert, den `belegung::fuer_den_betrieb()`
haelt — nicht die Arbeitskopie des Blattes. Das deckt sich mit der Directive: die Ausgabe zeigt,
was "im Augenblick des Aufrufs gilt", und es gilt, was ausloest.

**Der Preis ist benannt:** waehrend die Ansicht offen steht, kann die Datei sichtbar von dem
abweichen, was auf dem Schirm zu sehen ist. Ob dieser Fall dem Nutzer gemeldet wird, ist hier
nicht entschieden und gehoert in den Spec.
