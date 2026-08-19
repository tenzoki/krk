# Welche Flächen holen den Fokus, wenn man hineinklickt, und was macht ein Klick mit dem aktiven Dateifenster?

---
**Domain:** code
**Status:** implemented
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

---
Implemented: `76ceb68` — `Anwendungsdelegierter::aktives_dem_ersthelfer_nachziehen` (`crates/krk-ui/src/appkit/anwendung.rs:4285`) hängt als **erster** von zwei Empfängern am Melder des Ersthelferwechsels (`:1130`) und setzt damit Möglichkeit 1 um: liegt der Rang nach dem Wechsel in einem Dateifenster, ist dieses das aktive, ob der Klick eine Zeile traf oder nicht.

**Am Baumstand `77dcd48` nachgelesen, Stelle für Stelle:**

- **„Jede Fläche eines Bereichs holt den Fokus."** AppKit übersetzt den Klick ohnehin in ein `makeFirstResponder:`, und den führt `Hauptfenster` seit C9 an genau einen Auslösepunkt. Die Leiste ist damit mitgelöst, ohne angefasst zu werden: sie ist eine `NSTableView` (`crates/krk-ui/src/appkit/leiste.rs:3`) und nimmt den Rang von sich aus.
- **„Ein Klick in eine Dateiliste setzt sie immer als aktive."** `Bereich::seite` (`crates/krk-ui/src/fenstermodell.rs:161`) ist die eine Stelle, die aufzählt, welche Bereiche Dateifenster sind, und liefert für Lesezeichen, Vorschau und Editor `None`; nur `Links` und `Rechts` erreichen `aktives_setzen`.
- **„Statuszeile und Bereichsleiste bleiben außen vor."** Beide sind keine Bereiche, und die Schalter der Bereichsleiste verweigern den Ersthelferrang ausdrücklich (`crates/krk-ui/src/appkit/bereichsleiste.rs:93`, `refusesFirstResponder`).
- **`aktives_setzen` hat jetzt drei Anlässe und bleibt die eine Stelle** (`anwendung.rs:4234`): die Zeilenauswahl und der Klick auf einen Abschnitt der Tableiste über `DateifensterQuelle::angefasst`, dazu dieser dritte.

**Der Abnahmeklick des Nutzers ist noch nicht gemeldet, und `_i_` behauptet ihn nicht.** Der Marker sagt nach `rules/fusion-workbench-conventions.md`, Abschnitt `## State Markers — decisions`, dass Code auf der Platte die Antwort einlöst, und das ist geprüft. Ob die Wirkung am laufenden Bündel ankommt, ist eine andere Frage; sie gehört zum Abnahmelauf, den nur der Nutzer fahren kann. Dieselbe Lesart hat die Runde 13 für ihre zwei Datensätze angewandt, die `d6343e0` vor dem Abnahmelauf zitieren.
