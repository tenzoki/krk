# Was geschieht mit einer offenen Umbenennung, die ohne Aktion endet?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator
**Cross-references:** shared/issues/260815-2125_o_verlaesst-der-nutzer-die-offene-namenszelle-bleibt-der-getippte-text-stehen-und-das-ordnerzeichen-weg.md, shared/issues/260815-2204_o_der-doc-kommentar-von-umbenennung-beenden-nennt-den-fokusverlust-als-aufrufer-die-messung-derselben-sitzung-widerlegt-das.md, shared/decisions/260815-2056_i_woran-erkennt-der-nutzer-in-der-dateiliste-einen-ordner.md

---

## Frage

C4 sagt zum Umbenennen zwei Ausgänge zu: „Return übernimmt, Escape verwirft."
Es gibt einen dritten, und er ist nirgends festgelegt. Die Bearbeitung endet,
ohne dass die Aktion `umbenennungBeendet:` kommt; der getippte Text ist dann
fort, und umbenannt wird nichts.

**Er hängt nicht am Nutzer.** Gemessen am 260816 auf macOS 15.7.7 am wirklichen
Hauptfaden (Nachtrag in `260815-2125`): `reloadData` und
`reloadDataForRowIndexes:columnIndexes:` beenden eine offene Bearbeitung ohne
Aktion. Ihre Rufer sind `nach_lesebeginn` — also **jede** Auffrischung durch die
Dateisystemwache, ausgelöst von irgendeinem anderen Programm, das in den
angezeigten Ordner schreibt — und der Takt des Lesevorgangs. Der Klick daneben
ist damit nur einer von drei Wegen in denselben Ausgang.

Solange die Frage offen ist, kann der dritte Ausgang nicht behoben werden: jede
Behebung legt fest, was er tun soll.

## Optionen

1. **Die Auffrischung aufschieben, solange eine Namenszelle offen steht.**
   `krk-ui/src/appkit/auffrischung.rs` trägt mit `schiebt_auffrischung_auf`
   bereits den Mechanismus, eine Auffrischung zurückzuhalten, solange etwas
   anderes läuft; eine offene Bearbeitung wäre ein weiterer Grund.
   - Für: nimmt beide unfreiwilligen Wege heraus, ohne die Frage nach dem Klick
     zu beantworten. Kein neuer Mechanismus, ein weiterer Fall im vorhandenen.
   - Gegen: die Liste steht still, solange eine Zelle offen ist. Beim
     Stapelumbenennen ist genau das schon einmal als Defekt gemeldet worden
     (`circles/260802-0842-…/issues/260805-1337_c_…`), dort mit leerer Liste als
     Symptom.
   - Offen bleibt: der Klick daneben.

2. **Verwerfen, wie bei Escape.** Eine Regel für alle drei Wege.
   - Für: eine Zusage statt dreier Sonderfälle, und C4 bekommt seinen dritten
     Satz. Nichts wird umbenannt, was der Nutzer nicht bestätigt hat.
   - Gegen: wer neben die Zelle klickt, verliert das Getippte. Das ist der
     heutige Stand, nur benannt.

3. **Übernehmen, wie bei Return.** Der Weg des Finders für den Klick daneben.
   - Für: entspricht der Erwartung vieler Nutzer beim Klick.
   - Gegen: **auf den beiden unfreiwilligen Wegen benennt dann ein fremder
     Prozess um.** Schreibt irgendein Programm in den Ordner, während eine Zelle
     offen steht, führt KRK die Umbenennung aus, ohne dass der Nutzer sie
     bestätigt hat. Diese Option verlangt daher zwingend Option 1 daneben, sonst
     ist sie gefährlich.

## Randbedingungen

- C4 sagt heute zwei Ausgänge zu. Jede Antwort außer 1 ändert den Spec-Wortlaut.
- Die Anzeigehälfte (das Ordnerzeichen kehrt in die Zelle zurück) ist von der
  Frage unabhängig und im Nachtrag der Durchsicht beschrieben; sie lässt sich
  vorher beheben.

## Empfehlung

Option 1, und danach die Frage nach dem Klick getrennt stellen. Sie ist die
einzige, die die beiden Wege ohne Nutzerzutun schließt, ohne eine Zusage von C4
zu ändern, und sie baut nichts Neues. Option 3 ohne Option 1 wäre die einzige
Wahl, die aus einem fremden Schreibvorgang eine Umbenennung macht.

---
Answered:
Implemented:
Deferred:
Superseded by:
