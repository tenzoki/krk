# Räumt ein gehaltener Rückschritt weiter, wenn der Filtertext leer wird?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `decisions/260814-1830_a_wie-nimmt-der-nutzer-ein-einzelnes-zeichen-des-filters-zurueck.md` (die Antwort, aus der diese Frage folgt); `crates/krk-ui/src/appkit/anwendung.rs:4274-4276` (`in_den_papierkorb`, stellt den Auftrag ohne Rückfrage); `crates/krk-ui/src/belegungsmodell.rs` (`letztes_zeichen_weg`, das bei leerem Suchtext nichts tut und das über seinen Rückgabewert meldet)

---

## Question

Der Nutzer hat am 260814-1845 entschieden, dass die nackte Rückschritt-Taste ein Zeichen des Filtertexts zurücknimmt, solange einer steht, und sonst in den Papierkorb räumt. Die Regel ist an ihrer Grenze nicht vollständig, und die Lücke liegt genau dort, wo sie am meisten kostet.

Wer drei Zeichen getippt hat und die Rückschritt-Taste hält, löst eine Tastenwiederholung aus. Nach dem dritten Anschlag steht kein Filtertext mehr; der vierte trifft auf die alte Bedeutung und räumt die Auswahl in den Papierkorb. Das Räumen läuft ohne Rückfrage, `Anwendungsdelegierter::in_den_papierkorb` stellt den Auftrag unmittelbar. Der Nutzer hat also nichts anderes getan, als eine Taste zu lange zu halten, und hat Dateien weggeräumt.

Der Papierkorb ist umkehrbar, und deshalb ist der Schaden begrenzt. Die Überraschung ist es nicht: die Handlung, die der Nutzer ausführt, ist „den Filter leeren", und ihr Ergebnis ist eine geräumte Datei.

## Options

1. **Keine Sonderregel. Sobald kein Zeichen mehr dasteht, räumt der nächste Anschlag, auch wenn er aus der Wiederholung stammt.**
   - Pro: die Regel bleibt ein Satz, und dieser Satz ist genau der, den der Nutzer entschieden hat. Keine dritte Größe kommt hinzu, und die Belegungsansicht bleibt eine Ansicht ohne Fußnoten.
   - Kontra: der Fall aus der Frage tritt bei jedem gehaltenen Rückschritt ein, und er ist kein seltener Fehlgriff, sondern die übliche Art, einen kurzen Text zu löschen. Die Fallunterscheidung schützt dann gerade dort nicht, wo der Nutzer sie am wenigsten im Blick hat.
2. **Die Tastenwiederholung trägt nicht über die Grenze. Ein gehaltener Rückschritt hört auf, wenn der Filtertext leer ist; erst ein neuer Druck räumt.**
   - Pro: schließt den einen Fall, in dem die Regel überrascht, und schließt ihn ohne Zeitgeber. AppKit meldet an jedem Tastenereignis, ob es aus einer Wiederholung stammt, und die Frage ist damit an derselben Stelle zu beantworten wie die Fallunterscheidung selbst. Der Nutzer verliert nichts: wer räumen will, drückt einmal.
   - Kontra: eine Größe mehr in einer Regel, die ohnehin vom Zustand abhängt. Das Verhalten ist in keiner Übersicht zu sehen, und wer es nicht kennt, hält den ausbleibenden zweiten Vorgang für eine verschluckte Taste.
3. **Nach dem Leeren bleibt die Rückschritt-Taste wirkungslos, bis der Nutzer etwas anderes tut** — die Auswahl bewegt, ein Zeichen tippt, den Ordner wechselt.
   - Pro: schützt auch den zweiten, getrennten Anschlag kurz nach dem Leeren.
   - Kontra: führt einen Zustand ein, den nichts anzeigt und den der Nutzer nicht abfragen kann. Wer bewusst räumen will, drückt zweimal ohne Wirkung und weiß nicht, warum. Das ist die Sorte Regel, die dieses Projekt bisher vermieden hat.

## Constraints

- `delete` und `cmd+delete` tragen `in_papierkorb`; `cmd+delete` räumt nach dem Entscheid vom 260814-1845 in jeder Lage und ist von dieser Frage nicht berührt.
- Das Räumen läuft ohne Rückfrage. Allein das endgültige Löschen zeigt eine Bestätigung.
- Diese Runde setzt keinen Zeitgeber. Die Sekundenregel der Sprungmarke fällt weg, und eine neue Zeitmessung widerspräche C1.5.
- Die Antwort ändert kein Bedienelement und keine Belegung. Sie ändert allein, welcher Tastendruck den vorhandenen Befehl erreicht.

## Recommendation

Möglichkeit 2. Die Fallunterscheidung ist vom Nutzer als sicherheitsrelevant begründet worden, und ihr Wert bemisst sich daran, ob sie in dem Fall trägt, der wirklich vorkommt: das Halten der Taste ist die übliche Art, drei Zeichen zu löschen, und Möglichkeit 1 lässt sie genau dort auslaufen. Der Preis ist eine Abfrage an derselben Stelle, an der die Regel ohnehin beantwortet wird, und nicht eine zweite Regel daneben. Möglichkeit 3 kauft einen kleinen weiteren Schutz mit einem unsichtbaren Zustand und ist deshalb teurer als der Schaden, den sie verhindert.

Der Spec fährt bis zu einer Antwort auf Möglichkeit 2, Kriterium C1.18. Fällt die Antwort auf Möglichkeit 1, entfällt C1.18 ersatzlos und sonst ändert sich nichts.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: Nutzer am 260814-1910 im Orchestrator-Dialog — Moeglichkeit 2, wie empfohlen. Die Tastenwiederholung traegt nicht ueber die Grenze: ein gehaltener Rueckschritt hoert auf, sobald der Filtertext leer ist, und erst ein neuer Druck raeumt. Kein Zeitgeber; AppKit meldet an jedem Tastenereignis, ob es aus einer Wiederholung stammt, also faellt die Frage an dieselbe Stelle wie die Fallunterscheidung selbst. Der Spec faehrt bereits darauf (C1.18), es ist keine Aenderung noetig.

Die beiden Gegenrechnungen sind vorgelegt und angenommen: eine Groesse mehr in einer ohnehin zustandsabhaengigen Regel, und ein Verhalten, das in keiner Uebersicht steht. Moeglichkeit 3 ist mit ihr verworfen, weil sie einen Zustand einfuehrt, den nichts anzeigt.
