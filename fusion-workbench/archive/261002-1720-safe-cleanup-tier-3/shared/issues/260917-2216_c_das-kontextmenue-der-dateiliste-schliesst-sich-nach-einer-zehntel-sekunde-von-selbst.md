Das Kontextmenü der Dateiliste schließt sich nach einer zehntel Sekunde von selbst

---

Ein Rechtsklick in der Dateiliste öffnet das Kontextmenü, das sich nach etwa einer zehntel Sekunde ohne weiteres Zutun wieder schließt. Vom Nutzer am 260917 beobachtet, am gebauten Stand 1.10.1.

---

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>

Die Diagnose stammt aus einem Defektdispatch an den `coder` und ist **nicht am laufenden Programm belegt**: KRK im Vordergrund ist in diesem Projekt Nutzerarbeit, kein Agent kann den Defekt selbst sehen.

**Belegt am Code**

`Tabelle::menue_auffrischen` (`crates/krk-ui/src/appkit/tabelle.rs:1304`), der Rückruf hinter `menuNeedsUpdate:`, ruft als erstes `rechtsklick_auswahl_nachziehen` (`:1849`) und mutiert damit die Tabelle, während AppKit die Menüverfolgung aufbaut. Auf einer unmarkierten Zeile führt der Weg über `:1860` nach `markierung_aendern(markierung_aufheben)` und dort auf `tabelle.reloadData()` (`:3002`), danach `auswahl_anzeigen()` und `meldung_gewechselt()`; `:1861` setzt zusätzlich `selectRowIndexes` und `scrollRowToVisible` (`:2373`). Die Tabelle ist ansichtsbasiert (`:4671`), `reloadData` verwirft also jede Zeilen- und Zellenansicht.

**Die Zeile `:1860` ist neu.** Der Rechtsklickweg entstand am 260812 mit `d6eff4b` und rief bis dahin allein `zeile_setzen`, ohne `reloadData`. Die einzige Änderung seither ist `8e7a067` (260907), die genau diese Zeile einfügt. Das Verhalten dahinter ist eine Nutzerantwort vom 260907-0703 (`260812-1516_*_hebt-ein-rechtsklick-auf-eine-unmarkierte-zeile-die-markierung-anderswo-auf.md`) und steht nicht zur Rücknahme; allein das Wie steht zur Debatte.

**Ausgeschlossen, je am Code nachgelesen**

Lebensdauer (das `NSMenu` hält die Tabelle stark, `:5188`; die beiden `NSSharingServicePicker` sitzen in getrennten Haltern, `appkit/teilen.rs:162` und `:190`; jeder Eintrag bekommt sein Ziel, `:1913`). Ein zweiter Ereignisabgriff für die Maus: der lokale Abgriff nimmt allein `NSEventMask::KeyDown` (`appkit/ereignisse.rs:448`). `cancelTracking`, `popUpContextMenu`, `orderOut`, `removeFromSuperview`: im ganzen Baum nicht vorhanden. Der Einzugstakt (`:3848`) läuft nur bei offenem Lauf und wird sonst abgeräumt (`:3802`); ein Rechtsklick startet keinen. `Aufteilung::anwenden` läuft nur bei echtem Wechsel des aktiven Dateifensters (`appkit/anwendung.rs:5688`).

**Erschlossen, nicht belegt**

`inference:` Das `reloadData` reißt die Zeilenansicht weg, gegen die AppKit die Verfolgung gerade aufbaut, und bricht sie ab. Zwei Nebenkandidaten aus derselben Zeile bleiben offen: das zusätzliche `selectRowIndexes` und `meldung_gewechselt`.

**Was die Frage ohne Bündelbau entscheidet**

`rechtsklick_zielzeile` (`crates/krk-ui/src/kommandos/operationen.rs:260`) antwortet `None`, sobald die geklickte Zeile markiert ist oder der Klick auf keine Zeile fiel; dann läuft die neue Zeile nicht. Am installierten KRK:

1. Rechtsklick auf eine markierte Zeile
2. Rechtsklick auf die leere Fläche unter der letzten Zeile
3. Rechtsklick auf eine unmarkierte Zeile
4. Rechtsklick in der Vorschau, deren Menübau keine Ansicht anrührt (`appkit/vorschau.rs:789`)

Bleiben 1, 2 und 4 stehen und verschwindet allein 3, ist `8e7a067` als Ursache belegt.

**Vorgesehene Behebung**

Die Tabellenmutation aus `menuNeedsUpdate:` herausziehen und vor den Beginn der Menüverfolgung setzen, also in ein `menuForEvent:` einer `NSTableView`-Unterklasse, wie `Pdfbetrachter::kontextmenue` es schon hält (`appkit/betrachter.rs:409`). Das trifft alle drei Kandidaten auf einmal und lässt die Nutzerantwort vom 260907 unangetastet.

**Abnahme**

Das Kontextmenü bleibt auf allen vier Wegen stehen, bis der Nutzer einen Eintrag wählt oder daneben klickt; die Markierungsaufhebung beim Rechtsklick auf eine unmarkierte Zeile wirkt weiter wie am 260907 entschieden; `make check` bleibt grün.

---

**Nicht mehr reproduzierbar, Stand 260917-2228.** Der Nutzer hat die vier Handgriffe gefahren und den Defekt nicht wieder erzeugen können. Nichts ist geändert worden, der Datensatz steht deshalb wieder auf offen statt auf in Arbeit: unbeobachtet ist nicht behoben.

**Was beim nächsten Auftreten zuerst zu prüfen ist.** Der Auffrischungstakt (`appkit/tabelle.rs:3848`) baut die Dateiliste 60-mal je Sekunde neu auf, solange ein Lese- oder Git-Lauf offen ist, und wird danach abgeräumt (`:3802`). Der Defektdispatch hat ihn ausgeschlossen, weil ein Rechtsklick selbst keinen Lauf startet; **dass ein Rechtsklick in einen bereits laufenden Lauf fallen kann, ist dabei nicht bedacht worden.** Das passt zur Sprunghaftigkeit des Symptoms: in einem großen Ordner oder einem Repository mit vielen geänderten Dateien läuft beim Aufklappen noch etwas, in einem kleinen fertig eingelesenen nicht. Wer den Defekt wiedersieht, notiert deshalb zuerst, ob die Liste in diesem Augenblick noch nachlud.

**Unabhängig vom Symptom bleibt die Stelle schwach.** `menue_auffrischen` (`:1304`) mutiert die Tabelle aus `menuNeedsUpdate:` heraus, und zwar in jedem Fall, nicht nur im beobachteten. Die vorgesehene Behebung oben trifft das, ob das Symptom je wiederkehrt oder nicht.

---
Resolved: Die Tabellenmutation steht nicht mehr in `menuNeedsUpdate:`. `crates/krk-ui/src/appkit/tabelle.rs`
trägt eine `NSTableView`-Unterklasse `Dateiliste`, deren `menuForEvent:` (`Dateiliste::kontextmenue`)
`rechtsklick_auswahl_nachziehen` ruft und danach das Menü der Oberklasse herausgibt; `menue_auffrischen`
liest seitdem nur noch das Modell und füllt das Menü. Die Unterklasse hält einen **schwachen**
Rückverweis auf `DateifensterQuelle` (`Dateiliste::ziel_setzen`, gesetzt in `Dateifenster::bauen`), weil
`QuelleIvars::tabelle` die starke Richtung trägt.

**Die angeklickte Zeile kommt jetzt aus dem Ereignis** und nicht mehr aus `clickedRow`:
`convertPoint:fromView:` auf `NSEvent.locationInWindow`, dann `rowAtPoint:`. Der Kopf des Systems sagt an
keiner Stelle, ob `clickedRow` gesetzt ist, wenn `menuForEvent:` läuft (`NSTableView.h:275-276` tragen
keinen Kommentar), während `locationInWindow` seine Zusage im Kopf trägt. `rechtsklick_zielzeile`
(`crates/krk-ui/src/kommandos/operationen.rs:260`) bleibt unverändert: beide Werte antworten außerhalb
jeder Zeile negativ. `clickedRow` hat damit wieder genau einen Abnehmer, den Doppelklick.

**Die Nutzerantwort vom 260907-0703 ist unangetastet**: ein Rechtsklick auf eine unmarkierte Zeile hebt
die Markierung weiterhin auf und rückt die Auswahl nach, über denselben `markierung_aendern`-Weg wie
zuvor. Die Reihenfolge trägt sich von selbst, weil AppKit erst `menuForEvent:` und dann
`menuNeedsUpdate:` ruft.

**Geschlossen wird an der falsch geschnittenen Stelle und nicht an einem Symptom.** Der Nutzer konnte den
Defekt am 260917-2228 nicht wieder erzeugen, und kein Agent kann ihn sehen; was hier behoben ist, ist die
Mutation der Tabelle aus einem Rückruf heraus, den AppKit erst nach dem Aufbau der Menüverfolgung ruft.
Eine Wirkung auf das beobachtete Symptom ist damit **nicht** belegt, und die Kommentare am Code behaupten
sie nicht.

**Der Auffrischungstakt bleibt unberührt und bleibt der erste Verdächtige.** Der Zeitgeber hängt weiter in
`NSRunLoopCommonModes` und baut die Liste sechzigmal je Sekunde neu auf, solange ein Lese- oder Gitlauf
offen ist, auch unter einem stehenden Menü. Ein Rechtsklick, der in einen laufenden Vorgang fällt, sieht
weiterhin `reloadData` unter dem offenen Menü. Der Hinweis im Nachtrag darüber gilt unverändert; er steht
seit dem 260918 auch im Modulkopf von `tabelle.rs`.

**Ungeprüft und Nutzerarbeit:** dass das Menü nach der Umstellung stehen bleibt und die
Markierungsaufhebung weiter wirkt, sieht erst der Nutzer am nächsten Bündel. `make check` ist grün
(fünf von fünf, 260918).

**Nebenbefund, nicht behoben:** der Kopf von `crates/krk-ui/src/appkit/teilen.rs` spricht von „drei
eigenen Eintraegen — Zip, Unzip und Finder"; `Kontextbefehl::ALLE` führt seit dem 260907 vier. Das ist ein
eigener Befund und gehört nicht in diese Umstellung.
