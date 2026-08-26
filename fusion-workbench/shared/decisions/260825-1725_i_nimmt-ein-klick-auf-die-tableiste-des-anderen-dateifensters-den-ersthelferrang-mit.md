# Nimmt ein Klick auf die Tableiste des anderen Dateifensters den Ersthelferrang mit?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Answered:** 260825-1740, Kai Stalmann — Moeglichkeit 1: ja, der Klick auf die Tableiste nimmt den Rang mit, aber als eigener Planschritt und nicht im Tab-Zweig mitgegriffen. Empfehlung des Planers ohne Aenderung uebernommen.
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`; `shared/analyses/260819-1043-klick-holt-den-fokus-nicht.md`; `shared/issues/260823-0731_*_ein-klick-in-das-andere-dateifenster-nimmt-eine-ziehbewegung-zurueck.md`; `crates/krk-ui/src/appkit/tabelle.rs:4648`; `crates/krk-ui/src/appkit/anwendung.rs:3172`

---

## Question

KRK führt zwei Fokusgrößen: `Fenstermodell::aktiv`, das sagt, welches Dateifenster die
Befehle meinen, und den Ersthelferrang von AppKit, der sagt, wohin die Tastendrücke gehen. Am
260825 gemessen: **drei** Stellen schreiben `aktiv`, und zwei davon nehmen den Rang mit. Die
dritte ist der Tab-Befehl (`anwendung.rs:3172`), und Schritt 1 des Plans zur Runde 18 zieht ihn
nach.

Daneben steht eine **vierte** Stelle, und sie ist keine der drei: ein Klick auf die Tableiste
des anderen Dateifensters ruft `DateifensterQuelle::angefasst` (`tabelle.rs:4648`) und setzt
darüber `aktiv` — ohne den Rang. Gemessen an einem Nachbau: ein `NSSegmentedControl`, die
Bauart der Tableiste, nimmt den Ersthelferrang bei einem Klick **nicht** an, obwohl
`acceptsFirstResponder` dort `1` liefert.

Wer über die Tableiste die Seite wechselt, sitzt danach in genau derselben Lage wie nach einem
Tab: die Befehle meinen die eine Liste, die Tastendrücke gehen in die andere. Die Frage ist,
ob dieser Weg mit dem Tab-Befehl zusammen nachgezogen wird oder nicht.

## Options

1. **Der Klick auf die Tableiste nimmt den Rang mit**, wie der Tab-Befehl nach Schritt 1.
   - Pros: Danach gilt für **jeden** Schreiber von `aktiv` dieselbe Regel, und die Invariante
     „der Rang sitzt in der Liste, die `aktiv` nennt" ist ausnahmslos. Erst dann kann die
     Zählprobe am Quelltext, die Schritt 1 aufsetzt, sie vollständig halten; mit einer
     Ausnahme darin hielte sie eine Regel, die nicht gilt. Und die Lage, die der Nutzer
     gemeldet hat, entsteht dann auf keinem Weg mehr.
   - Cons: Ein zusätzlicher Weg nach `aktives_setzen` und damit nach `aufteilung_nachziehen`.
     Der offene Defekt `260823-0731` (ein Klick in das andere Dateifenster nimmt eine
     Ziehbewegung zurück, weil `aktives_setzen` ohne vorheriges
     `bildschirmbreiten_uebernehmen` nachzieht) **wächst mit jedem solchen Weg**. Er wird
     dadurch nicht verursacht, aber häufiger sichtbar.
   - Cons: Ein Klick auf die Tableiste ist die Wahl eines Tabs und nicht ausdrücklich die Wahl
     eines Fensters. Dass er den Tastaturfokus mitnimmt, ist plausibel und nicht zwingend.

2. **Er bleibt, wie er ist.**
   - Pros: Kein weiterer Weg nach `aktives_setzen`, also kein Zuwachs für `260823-0731`.
   - Cons: Die Invariante hat eine Ausnahme, und die Zählprobe aus Schritt 1 muss sie
     ausnehmen — womit sie eine Regel mit Loch hält und der nächste Leser das Loch für einen
     Fehler hält. Die gemeldete Lage bliebe über die Tableiste erreichbar, und der Nutzer
     erlebte denselben Fehler nach einem anderen Handgriff.

3. **Er bleibt, wie er ist, und `260823-0731` wird zuerst behoben; danach Möglichkeit 1.**
   - Pros: Der eine Einwand gegen Möglichkeit 1 fällt weg, bevor sie greift.
   - Cons: Hängt die Antwort an einer zweiten, unabhängigen Arbeit. `260823-0731` ist offen
     und in keiner Runde eingeplant.

## Constraints

- Es entsteht kein zweiter Beobachter des Fokus. `NSWindow` verschickt keine Benachrichtigung
  über den Ersthelfer, und die Beobachtung der Eigenschaft ist von Apple nicht zugesagt
  (Modulkopf von `appkit/fenster.rs`).
- `fokus_setzen` bleibt die eine Stelle, die `makeFirstResponder:` ruft. Eine dritte Tür in
  `aktives_setzen` entsteht nicht.
- Die Sichtbarkeitssperre bleibt: ein ausgeblendetes Dateifenster wird nicht aktiv.

## Recommendation

**Möglichkeit 1, aber als eigener Schritt und nicht nebenbei im Tab-Zweig mitgegriffen.**

Der Grund für „ja" ist die Invariante. Eine Regel mit einer Ausnahme ist keine Regel, die eine
Probe halten kann, und Schritt 1 der Runde 18 baut genau eine solche Probe. Der Grund für
„eigener Schritt" ist der Einwand: `260823-0731` wird dadurch häufiger sichtbar, und das ist
eine Wirkung, die der Nutzer sehen und wollen muss, bevor sie eintritt. Sie in einem Zweig
mitzunehmen, der von einer anderen Frage handelt, hieße sie zu verstecken.

Wer Möglichkeit 2 wählt, bekommt eine Zählprobe mit einer benannten Ausnahme, und die Ausnahme
gehört dann in den Rumpf der Probe geschrieben, samt diesem Datensatz als Grund — sonst liest
der nächste sie als Versehen.

---
Implemented: d3da6e3 — der Klick auf die Tableiste nimmt den Rang mit, und er tut es als eigener Schritt (Aufgabe E-1) und nicht im Tab-Zweig mitgegriffen, wie die Antwort es verlangt. Der Weg läuft über die Aufzählung `Rangmitnahme` (`crates/krk-ui/src/appkit/tabelle.rs`), die den Tableistenklick vom Zeilenklick trennt; `aktives_setzen` setzt den Fokus nach dem Umschreiben von `aktiv` (`crates/krk-ui/src/appkit/anwendung.rs`). Zwei Zählproben halten es: `der_klick_auf_die_tableiste_nimmt_den_ersthelferrang_mit` (`anwendung.rs:8863`) und `aktives_setzen_hat_genau_zwei_aufrufer` (`:8957`). Nachgemessen am 260826-0149 gegen den Baum, `make check` grün.
