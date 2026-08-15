# Verwirft oder übernimmt ein Klick neben die offene Namenszelle?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator
**Cross-references:** shared/decisions/260815-2247_a_was-geschieht-mit-einer-offenen-umbenennung-die-ohne-aktion-endet.md, shared/issues/260815-2125_o_verlaesst-der-nutzer-die-offene-namenszelle-bleibt-der-getippte-text-stehen-und-das-ordnerzeichen-weg.md

---

## Frage

C4 sagt zwei Ausgänge des Umbenennens zu: „Return übernimmt, Escape verwirft."
Der Nutzerentscheid vom 260816-0021 nimmt den beiden **unfreiwilligen** Wegen in
einen dritten Ausgang die Ursache, indem die Auffrischung aufgeschoben wird.
Übrig bleibt der Weg, den der Nutzer selbst geht: er klickt neben die offene
Zelle, in eine andere Zeile, in das andere Dateifenster.

Heute geschieht dabei nichts: der getippte Text steht in der Zelle, umbenannt
wird nicht, und beim nächsten Zeichendurchgang ist er fort. Das ist kein
zugesagter Ausgang, sondern ein unbeschriebener.

## Optionen

1. **Verwerfen, wie bei Escape.** Der Klick daneben zählt als Abbruch, die Zelle
   nimmt die Anzeigeform zurück.
   - Für: nichts wird umbenannt, was der Nutzer nicht ausdrücklich bestätigt
     hat. C4 bekommt einen dritten Satz, der zu den ersten beiden passt.
   - Gegen: wer den Namen fertig getippt hat und danebenklickt, verliert ihn.
2. **Übernehmen, wie bei Return.** Der Weg des Finders.
   - Für: entspricht der Erwartung vieler Mac-Nutzer.
   - Gegen: eine Umbenennung ohne ausdrückliche Bestätigung. Sie träfe auch den
     Fall, dass der Nutzer wegklickt, **weil** er es sich anders überlegt hat.
3. **Nachfragen.** Ein Blatt mit „übernehmen / verwerfen".
   - Für: kein Verlust und keine unbestätigte Umbenennung.
   - Gegen: ein Blatt für einen Nebenweg widerspricht der Maxime „supersimpel",
     und ein stehendes Blatt hält alle Tastenbefehle an.

## Randbedingungen

- Der Fokusverlust schickt die Aktion `umbenennungBeendet:` **nicht**; das ist am
  260816 am Hauptfaden gemessen. Jede Antwort außer 1 braucht deshalb einen
  Weg, an den getippten Text zu kommen, bevor AppKit ihn verwirft.
- C4 zählt heute zwei Ausgänge auf. Jede Antwort ändert den Spec-Wortlaut.

## Empfehlung

Keine. Das ist eine Frage der Bedienung und nicht der Technik: alle drei Wege
sind baubar, und welcher richtig ist, hängt daran, was der Nutzer beim
Wegklicken meint.

---
Answered:
Implemented:
Deferred:
Superseded by:
