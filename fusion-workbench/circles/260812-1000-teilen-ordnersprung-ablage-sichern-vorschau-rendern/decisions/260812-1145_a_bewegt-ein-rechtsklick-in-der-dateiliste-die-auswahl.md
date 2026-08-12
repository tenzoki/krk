# Bewegt ein Rechtsklick in der Dateiliste die Auswahl auf die angeklickte Zeile?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_o_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md` (Schritt 6, Kriterium C1.2 und C1.3); `crates/krk-ui/src/kommandos/operationen.rs:162` (`betroffene`, die eine Auswahlregel); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_a_an-welchen-drei-flaechen-haengt-das-neue-kontextmenue.md`

---

## Question

Mit dem Kontextmenue bekommt KRK zum ersten Mal eine Handlung, die der Nutzer
mit der **rechten** Maustaste auf einer bestimmten Zeile ausloest. Bisher haben
alle sechs Abnehmer von `kommandos::operationen::betroffene` ihre Menge aus einem
**Tastendruck** hergeleitet, und dabei stellt sich die Frage nicht: der Nutzer
sieht die Markierung und die Auswahl vor sich, waehrend er tippt.

Beim Rechtsklick faellt beides auseinander. Die Regel der Runde 4 lautet
unveraendert „die Markierung hat Vorrang, sonst gilt der Eintrag unter der
Auswahl". Ein Rechtsklick auf eine Zeile, die weder markiert noch ausgewaehlt
ist, teilt danach **etwas anderes als das, worauf der Zeiger steht** — und das
Menue sagt nicht, was. Der Finder verhaelt sich anders: dort waehlt ein
Rechtsklick die Zeile unter dem Zeiger aus, sofern sie nicht ohnehin Teil der
Auswahl ist.

Die Frage haelt keinen Planschritt auf; der Plan faehrt Schritt 6 mit
Moeglichkeit 1. Sie bindet aber jeden spaeteren Eintrag des Kontextmenues, und
der Circle sagt ausdruecklich, dass das Menue in einer spaeteren Runde weitere
Eintraege bekommt. Ein zweiter Eintrag, der loescht oder verschiebt, macht aus
der Ueberraschung einen Schaden.

## Options

1. **Der Rechtsklick aendert nichts.** Das Menue wirkt auf `betroffene`, also auf
   dieselbe Menge wie jeder Tastenbefehl.
   - Pros: keine zweite Auswahlregel, keine Zeile Code, kein neuer Zustand. Die
     Antwort auf „worauf wirkt ein Befehl" bleibt im ganzen Programm dieselbe.
   - Cons: der Klick zeigt auf A und wirkt auf B. Bei einem spaeteren Eintrag mit
     zerstoerender Wirkung ist das der teuerste Fehler, den eine Oberflaeche
     machen kann.

2. **Der Rechtsklick setzt die Auswahl auf die angeklickte Zeile, es sei denn,
   diese Zeile ist markiert.** Danach gilt `betroffene` unveraendert.
   - Pros: der Klick zeigt und wirkt auf dasselbe. Das Verhalten ist das des
     Finders, also das, was der Nutzer erwartet. Die Regel selbst bleibt
     unangetastet — geaendert wird die Auswahl **vor** ihr, nicht sie selbst.
   - Cons: eine Bedingung mehr („es sei denn, markiert"), und der Rechtsklick
     hinterlaesst einen Zustand, auch wenn der Nutzer das Menue wieder
     wegklickt. `NSTableView` liefert die angeklickte Zeile ueber `clickedRow`;
     der Weg ist gebaut, aber neu in diesem Baum.

3. **Der Rechtsklick setzt die Auswahl immer auf die angeklickte Zeile und hebt
   eine bestehende Markierung auf.**
   - Pros: die einfachste Regel von allen, ohne Ausnahme.
   - Cons: sie zerstoert die Arbeit des Nutzers. Wer dreissig Eintraege markiert
     hat und danach mit der rechten Maustaste auf einen davon klickt, hat die
     Markierung verloren. Das ist schlechter als beide anderen.

## Constraints

- **Es gibt genau eine Auswahlregel**, `kommandos::operationen::betroffene`, und
  sie bleibt in jeder Antwort unveraendert. Keine Moeglichkeit dieses Datensatzes
  legt eine zweite an; sie unterscheiden sich allein darin, ob vor dem
  Nachschlagen die Auswahl bewegt wird.
- Die Antwort gilt fuer **alle drei** Flaechen gleich, soweit sie dort etwas
  bedeutet. Im Editor und in der Vorschau gibt es keine Zeile unter dem Zeiger,
  auf die etwas zu bewegen waere; die Frage stellt sich allein in der Dateiliste.
- Was auch immer gewaehlt wird, es darf keinen zweiten Weg an den Pruefungen
  vorbei geben: eine bewegte Auswahl geht durch dieselbe Stelle wie eine ueber
  die Tastatur bewegte, `DateifensterQuelle::auswahl_merken`, sonst erfaehrt die
  Vorschau nichts davon.

## Recommendation

**Wir empfehlen Moeglichkeit 2**, sobald das Kontextmenue einen zweiten Eintrag
bekommt, und halten Moeglichkeit 1 fuer die laufende Runde fuer vertretbar.

Der Grund fuer die Zweiteilung ist der Schaden und nicht die Ueberraschung.
Teilen zerstoert nichts: wer die falsche Datei teilt, bricht den Dialog des
Systems ab. Solange das Menue genau diesen einen Eintrag traegt — und der Circle
legt ihn ausdruecklich auf einen fest —, ist der Preis von Moeglichkeit 1 eine
Irritation. Mit dem zweiten Eintrag aendert sich das, und dann ist die Frage
nicht mehr aufzuschieben.

`inference:` Wir schliessen aus dem Wortlaut des Wunsches („per Kontextmenue auf
der rechten Maustaste"), dass der Nutzer das Verhalten des Finders vor Augen hat.
Gefragt worden ist er danach nicht.


## Antwort 260812-1200

**Moeglichkeit 2, Nutzerentscheid.** Der Rechtsklick setzt die Auswahl auf die angeklickte Zeile,
es sei denn, diese Zeile ist bereits markiert; danach gilt `betroffene` unveraendert.

Das ist das Verhalten des Finders und damit das erwartete: der Klick zeigt und wirkt auf dasselbe.
Wer dreissig Eintraege markiert hat und mit rechts auf einen davon klickt, behaelt seine
Markierung — das ist der Zweck der Ausnahme, und sie ist der Grund, warum Moeglichkeit 3
abgelehnt ist.

**Die Regel selbst bleibt unangetastet.** Geaendert wird die Auswahl **vor** ihr, nicht sie
selbst; `betroffene` beantwortet weiterhin allein, worauf ein Befehl wirkt, und Tastenweg und
Mausweg kommen an derselben Stelle zusammen.

**Der Preis ist benannt und angenommen:** eine Bedingung mehr, und der Rechtsklick hinterlaesst
einen Zustand, auch wenn das Menue wieder weggeklickt wird. Die angeklickte Zeile liefert
`NSTableView` ueber `clickedRow`; der Weg ist in diesem Baum neu, und der Modulkopf nennt seine
macOS-Untergrenze.

Moeglichkeit 1 ist abgelehnt, obwohl sie keine Zeile Code kostet: ein Menue, das auf A zeigt und
auf B wirkt, ist bei einem spaeteren Eintrag mit zerstoerender Wirkung der teuerste Fehler, den
eine Oberflaeche machen kann. Das Menue traegt heute nur Teilen, aber es ist als Traeger fuer mehr
gebaut.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1200` — Nutzerentscheid vom 260812-1200, vorgelegt mit drei Moeglichkeiten und ihren Folgen.
Implemented:
Deferred:
Superseded by:
