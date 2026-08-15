Der Takt eines laufenden Lesevorgangs beendet eine offene Namenszelle, und der Aufschub erreicht ihn nicht

---

Der Nutzerentscheid vom 260816-0021
(`shared/decisions/260815-2247_a_was-geschieht-mit-einer-offenen-umbenennung-die-ohne-aktion-endet.md`,
Option 1) schiebt die Auffrischung auf, solange eine Namenszelle in Bearbeitung steht.
Umgesetzt ist er in `crate::auffrischung::ordner_neu_lesen`, also im **einen**
Auffrischungspfad; er deckt damit beide Auslöser ab, die durch ihn laufen — die
Dateisystemwache und den Abschluss einer Dateioperation aus S16.

**Ein dritter Weg läuft nicht durch ihn.** `DateifensterQuelle::einziehen`
(`crates/krk-ui/src/appkit/tabelle.rs`) ist der Takt eines **schon laufenden**
Lesevorgangs und ruft `reloadData` und `auswahl_anzeigen` unmittelbar an der Tabelle.
Beide beenden eine offene Bearbeitung ohne die Aktion; gemessen am 260816 auf
macOS 15.7.7 am wirklichen Hauptfaden (Nachtrag in
`shared/issues/260815-2125_o_verlaesst-der-nutzer-die-offene-namenszelle-…`, dort auch
`selectRowIndexes:byExtendingSelection:`).

Wer also in einem großen Ordner zu tippen beginnt, während dieser noch einläuft, verliert
seinen getippten Namen, sobald der Takt `einzug.fertig` oder `einzug.ersetzt` erreicht.
`einzug.angehaengt` — der häufigste Takt während eines Lesevorgangs — läuft über
`noteNumberOfRowsChanged` und lässt die Bearbeitung stehen; er ist nicht betroffen.

---

**Schwere:** niedrig bis mittel. Dieselbe Folge wie beim behobenen Weg — der getippte Name
ist fort, umbenannt wird nichts —, aber ein deutlich engeres Fenster: der Nutzer muss die
Umbenennung starten, während der angezeigte Ordner noch liest.
**Gefunden von:** coder, bei der Umsetzung des Entscheids vom 260816-0021
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs`, `DateifensterQuelle::einziehen`
**Domain:** code

## Warum die Behebung nicht dieselbe Fallunterscheidung ist

Die Frage „steht in diesem Dateifenster eine Namenszelle offen?" ist dieselbe. Die
**Folge** ist es nicht, und darum ist der Aufschub hier nicht bloß eine weitere Stelle mit
derselben Zeile:

- **Beim behobenen Weg ändert sich das Modell nicht.** Der Aufschub verhindert, dass ein
  Lesevorgang überhaupt beginnt; das Ordnermodell bleibt unangetastet, und die Zeile, in
  der die Zelle steht, trägt am Ende der Bearbeitung denselben Eintrag wie am Anfang.
- **Beim Takt ändert es sich unter der offenen Zelle.** `einzug.fertig` heißt: die
  Sortierung steht, und die bisher in Lesereihenfolge angezeigten Zeilen tragen danach
  andere Einträge. `DateifensterQuelle::umbenennung_beenden` liest den alten Namen über
  `rowForView:` aus dem Modell — bewusst so, statt aus einem gemerkten Zustand
  (Doc-Kommentar dort). Ein Aufschub, der die Zelle über die Umsortierung hinweg offen
  hielte, benennte damit **eine andere Datei** um als die, auf die der Nutzer sie geöffnet
  hat.

Das ist eine schwerere Folge als die, die behoben werden soll. Ein Aufschub an dieser
Stelle setzt deshalb voraus, dass die offene Bearbeitung ihren **Eintrag** hält statt ihrer
Zeile — ein Umbau, den der genannte Doc-Kommentar ausdrücklich abgelehnt hat, weil ein
gemerkter Zustand eine zweite Löschregel bräuchte.

## Wege, die offenstehen

1. **Stehen lassen.** Das engere Fenster in Kauf nehmen; der Nutzer sieht, dass der Ordner
   noch lädt.
2. **Die Umbenennung während eines laufenden Lesevorgangs gar nicht erst beginnen lassen.**
   `umbenennung_beginnen` fragt `tabs.liest_noch()` und liefert `false`. Eine Regel statt
   eines Aufschubs, und sie hat keine Umsortierung unter sich. Kostet dem Nutzer die
   Umbenennung in den ersten Sekunden eines großen Ordners.
3. **Den Aufschub mit einer Eintragsbindung nachziehen.** Die offene Bearbeitung merkt sich
   ihren Eintrag statt ihrer Zeile; danach ist der Aufschub gefahrlos. Der größte der drei.

Welcher es wird, ist eine Nutzerfrage und keine Umsetzungsfrage.
