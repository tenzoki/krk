# Welche Flächen holen den Fokus, wenn man hineinklickt, und was macht ein Klick mit dem aktiven Dateifenster?

---
**Domain:** code
**Status:** answered
**Filed by:** analyst
**Cross-references:** `shared/analyses/260819-1043-klick-holt-den-fokus-nicht.md`; `shared/issues/260819-0900_o_ein-klick-in-das-dateifenster-holt-den-fokus-nicht-der-rahmen-bleibt-stehen.md`; `shared/issues/260819-1043_o_ein-klick-unter-die-letzte-zeile-laesst-das-aktive-dateifenster-stehen-und-malt-den-rahmen-auf-das-andere.md`; `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/` (C9, viertes Abnahmekriterium); `crates/krk-ui/src/appkit/bereichsleiste.rs:647` (die Schalter verweigern den Rang); `crates/krk-ui/src/appkit/statuszeile.rs:54`; `crates/krk-ui/src/kommandos/fokus.rs:262` (`bereich_mit_fokus`)

---

## Question

Der Nutzer hat am 260819 erwartet, dass ein Klick auf einen der vier bedienbaren Bereiche
den Fokus dorthin legt. Im Baum steht diese Zusage nirgends. C9 der Runde 2 nennt als
viertes Abnahmekriterium allein den Klick in die Bildlaufleiste der Vorschau, und der
verlangt eine richtige **Auskunft** über den Fokus, keine Fokusverlagerung.

Die Erwartung als solche ist unstrittig und braucht keine Wahl. Zu entscheiden sind die
Ränder, und sie sind nicht selbsterklärend, weil das Fenster mehr Flächen hat als die
vier Bereiche: die freie Fläche unter der letzten Zeile, die Tableiste über jeder
Dateiliste, die eine Statuszeile am Fensterfuß und die Bereichsleiste darunter, deren
Schalter den Ersthelferrang ausdrücklich verweigern.

Dazu kommt eine zweite Größe, die der Klick heute mitbedient. KRK führt `Fokus`, also
wohin die Tasten gehen, und `aktiv`, also welches der beiden Dateifenster gemeint ist.
Ein Klick auf eine Zeile setzt beides; ein Klick in die freie Fläche setzt heute nur den
Ersthelferrang und lässt `aktiv` stehen. Da `bereich_mit_fokus` den Wert
`Fokus::Dateifenster` über `aktiv` auflöst, laufen die beiden Größen in diesem Fall
auseinander, und der Fokusrahmen landet auf der Liste, in die niemand geklickt hat.

**Die Frage in einem Satz:** welche Flächen des Fensters holen den Fokus, und setzt ein
Klick in ein Dateifenster dieses auch dann als aktives, wenn er keine Zeile trifft?

## Options

1. **Jede Fläche eines Bereichs holt den Fokus, und ein Klick in eine Dateiliste setzt
   sie immer als aktive.** Die freie Fläche unter der letzten Zeile und die Tableiste
   zählen zum Bereich; Statuszeile und Bereichsleiste bleiben außen vor, weil sie keine
   Bereiche sind.
   - Pro: eine Regel ohne Ausnahme, an der Grenze der Bereiche geschnitten, die es schon
     gibt. `Fokus` und `aktiv` können nicht mehr auseinanderlaufen. Der Rahmen sitzt
     immer dort, wo der Nutzer hingeklickt hat.
   - Kontra: `aktiv` muss aus einem zweiten Anlass gesetzt werden, denn
     `tableView:shouldSelectRow:` feuert bei einem Klick in die freie Fläche nicht.
     `angefasst()` bekäme einen dritten Rufer, und die heutige Begründung an
     `shouldSelectRow:`, dass allein eine vom Nutzer ausgehende Auswahl umschaltet, wird
     dabei umgeschrieben.
   - Was sie verbaut: nichts Erkennbares. Die Regel ist die weiteste der drei und
     schließt die beiden anderen ein.

2. **Nur eine Zeile holt den Fokus; die freie Fläche und die Tableiste tun es nicht.**
   Der heutige Zustand, ausgeschrieben und festgelegt.
   - Pro: keine Zeile Code, und `angefasst()` behält seine zwei Rufer samt ihrer
     Begründung.
   - Kontra: der Nutzer klickt in eine Liste und der Rahmen bleibt stehen, was genau die
     Beobachtung ist, die den Datensatz `260819-0900` ausgelöst hat. Der Ersthelferrang
     wechselt nach der Messung trotzdem, also stünden Anzeige und Tastenziel
     auseinander, und das ist teurer als ein sichtbarer Fehler.
   - Was sie verbaut: die Erwartung des Nutzers vom 260819, dauerhaft.

3. **Die freie Fläche holt den Fokus, setzt aber `aktiv` nicht.** Der Fokusrahmen folgt,
   die Frage „welche Liste ist gemeint" bleibt an der Auswahl hängen.
   - Pro: `angefasst()` behält seine zwei Rufer.
   - Kontra: `bereich_mit_fokus` löst `Fokus::Dateifenster` über `aktiv` auf, also malte
     die Anzeige weiter den falschen Rahmen. Diese Möglichkeit verlangt zusätzlich einen
     zweiten Weg, den Fokusbereich ohne `aktiv` zu benennen, und das wären zwei
     Wahrheiten über dieselbe Frage.
   - Was sie verbaut: den einen Auflösungsweg von `Fokus::Dateifenster` auf einen
     Bereich.

## Recommendation

**Möglichkeit 1**, sofern der Nutzer nicht einen Grund gegen das Umschalten des aktiven
Dateifensters ohne Auswahl sieht. Sie schneidet an der Grenze, die der Baum schon führt,
nämlich am Bereich, und sie ist die einzige der drei, nach der `Fokus` und `aktiv` nicht
auseinanderlaufen können. Möglichkeit 3 verlangt eine zweite Antwort auf eine Frage, die
`bereich_mit_fokus` heute allein beantwortet.

**Diese Antwort ändert nichts an der Ursache von `260819-0900`.** Sie legt fest, was
gelten soll; warum die Anzeige heute stehen bleibt, ist damit nicht beantwortet und
braucht die zwei Handgriffe am laufenden Bündel, die die Analyse nennt.

---
Answered: Nutzerentscheid am 260819 — **Möglichkeit 1**. Jede Fläche eines Bereichs holt den Fokus, und ein Klick in eine Dateiliste setzt sie immer als aktive, auch wenn er keine Zeile trifft. Die Folge ist ausdrücklich mitentschieden: F5 und F6 nehmen danach als Quelle das Dateifenster, in das zuletzt geklickt wurde, auch ohne Auswahl. Statuszeile und Bereichsleiste bleiben außen vor, weil sie keine Bereiche sind. Vorgelegt wurde die Wahl zwischen Möglichkeit 1 und 3; Möglichkeit 2 stand nicht zur Wahl, weil der Nutzer die Korrektur ausdrücklich verlangt hatte.
