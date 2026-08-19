# Verwirft oder übernimmt ein Klick neben die offene Namenszelle?

---
**Domain:** code
**Status:** implemented
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
- **Berichtigt am 260816-0935:** hier stand, C4 zähle zwei Ausgänge auf und
  jede Antwort ändere den Spec-Wortlaut. Am Baum nachgesehen stimmt das nicht.
  Das Abnahmekriterium von C4 sagt allein „ein Tastenbefehl benennt den
  ausgewählten Eintrag um, direkt in der Liste"
  (`circles/260802-0842-…/planning/260802-1036_c_spec-navigator-geruest.md:254`).
  Der Satz „Return übernimmt, Escape verwirft" steht im **Plan** der Runde 1
  (`260802-1428_c_…:1044` und `:1046`) und in `tabelle.rs:1773`, wo er C4
  zugeschrieben wird. Der Spec ist also unberührt; nachzuziehen ist der
  Doc-Kommentar, und der Plan bleibt als Aufzeichnung seines Standes stehen.

## Empfehlung

Keine. Das ist eine Frage der Bedienung und nicht der Technik: alle drei Wege
sind baubar, und welcher richtig ist, hängt daran, was der Nutzer beim
Wegklicken meint.

## Nutzerentscheid vom 260816-0935: Option 1, verwerfen

Ein Klick neben die offene Namenszelle verwirft wie Escape.

**Die Hälfte davon gilt schon.** Umbenannt wird beim Fokusverlust ohnehin
nichts, weil AppKit die Aktion nicht schickt; das ist am 260816 gemessen. Offen
ist allein die Anzeige: der getippte Text bleibt in der Zelle stehen, bis
irgendein Zeichendurchgang sie anfasst. Die Umsetzung hat damit genau eine
Zusage herzustellen — **jedes Ende der Bearbeitung, dem keine Umbenennung
folgt, stellt die Anzeigeform wieder her.**

Escape ist bereits abgedeckt (`Namensfeld::bearbeitung_abbrechen`). Für die
übrigen Enden ist `textDidEndEditing:` die Stelle, und die Reihenfolge ist aus
der Messung zu T4 bekannt: ein Zeichendurchgang **vor** `super` liefert der
Aktion `rowForView = -1` und liesse die Umbenennung still ausfallen, **nach**
`super` ist er folgenlos. Der Return-Weg zeichnet die Zeile dann zweimal, was
der `coder` schon einmal als folgenlos gemessen hat.

---
Answered: shared/decisions/260816-0021_a_verwirft-oder-uebernimmt-ein-klick-neben-die-offene-namenszelle.md §Nutzerentscheid — verwerfen wie Escape; zu bauen ist allein die Wiederherstellung der Anzeigeform an jedem Ende ohne Umbenennung.
Implemented: 2c5a1b5 — `-[Namensfeld textDidEndEditing:]` ruft nach `super` bedingungslos `anzeigeform_herstellen`, dieselbe Methode, die Escape schon rief. Acht Ausgänge am Hauptfaden gemessen; auf dem Return-Weg fällt der Durchgang von selbst aus (`rowForView` liefert dort -1), eine Fallunterscheidung „kam eine Aktion?" ist deshalb nicht nötig. Der wirkliche Mausklick bleibt Nutzerarbeit.
Deferred:
Superseded by:
