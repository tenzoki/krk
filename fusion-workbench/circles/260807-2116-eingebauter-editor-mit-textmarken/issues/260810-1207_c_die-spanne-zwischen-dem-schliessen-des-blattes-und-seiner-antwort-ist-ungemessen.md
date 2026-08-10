# Die Spanne zwischen dem Schließen eines Blattes und seiner Antwort ist ungemessen

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, bei der Widerlegung von `260810-1102`
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs` (`blatt_steht:2012`,
`kommando_ausfuehren:2035`, `anlass_unterbleibt`),
`crates/krk-ui/src/appkit/blaetter/mod.rs` (`Blatt::zeigen_mit_wahl`)
**Cross-references:** `issues/260810-1102_*_ein-befehl-waehrend-der-nachfrage-aus-c4-wird-von-der-antwort-still-ueberschrieben.md`,
`issues/260810-1029_*_die-abkuerzung-fuer-die-gehaltene-datei-bricht-das-laufende-lesen-nicht-ab.md`,
C4, C6

---

## Der Befund

Die Sperre, die jeden Befehl ausser dem Abbruch anhält, solange ein Blatt steht,
fragt `NSWindow::attachedSheet` (`blatt_steht`, `anwendung.rs:2012`). Der
Rückruf, der die Antwort des Nutzers ausführt, ist der Abschlussblock von
`NSAlert::beginSheetModalForWindow:completionHandler:`. **Ob AppKit diese beiden
Zeitpunkte zusammenfallen lässt, ist nicht geprüft.** Fällt `attachedSheet`
schon mit dem Beginn des Einfahrens auf `nil`, während der Abschlussblock erst
danach läuft, steht dazwischen eine Spanne von der Länge einer
Blattanimation, in der die Sperre nicht mehr greift und die Antwort noch nicht
ausgeführt ist.

`speculation:` — das ist eine Aussage über die Reihenfolge, die AppKit einhält,
und weder gemessen noch aus der Dokumentation belegt. Der Code von KRK gibt sie
nicht her: er sieht nur den Block.

## Der Schaden, den er im schlechtesten Fall trägt

Durchgerechnet an C4 und C6, für einen Öffnungsbefehl in dieser Spanne:

1. **Die Antwort „sichern" oder „verwerfen"** führt zu keinem Schaden am Ende.
   Der Editor nimmt zuerst die zurückgehaltene Datei auf, danach trifft der
   Ausgang des neu begonnenen Öffnens ein und überschreibt sie; am Ende hält der
   Editor die Datei des letzten Befehls. Sichtbar bliebe ein kurzes Aufblitzen
   der zurückgehaltenen Datei.
2. **Die Antwort „abbrechen" und das gescheiterte Sichern** treffen dagegen die
   vorgemerkte Stelle einer Textmarke: `anlass_unterbleibt` setzt
   `vorgemerkte_marke` auf `None`, weil sie zur zurückgehaltenen Datei gehört.
   War der Befehl in der Spanne ein Sprung auf eine Textmarke, hat er seine
   eigene Stelle gerade vorgemerkt, und sie fällt mit. Die Datei öffnet dann,
   ohne zu der gemerkten Zeile zu springen — das achte Abnahmekriterium von C6
   verlangt, dass der Nutzer einen ungeprüften Sprung erkennt, und hier bleibt
   der Sprung ganz aus.

Kein Text geht in beiden Fällen verloren.

## Was zuerst zu tun wäre

**Messen, nicht bauen**, und die Messung ist billig: einen Zähler in den
Abschlussblock und in `blatt_steht`, dann am laufenden Bündel ein Blatt
beantworten und lesen, ob `attachedSheet` zwischen den beiden Zeitpunkten schon
`nil` ist. Ist es das nicht, ist dieser Datensatz gegenstandslos und zu
schließen. Das verlangt KRK im Vordergrund und ist damit Nutzerarbeit.

Trägt die Spanne, ist der engste Schnitt nicht die Sperre, sondern die
Reihenfolge in `anlass_unterbleibt`: es löscht `vorgemerkte_marke` blind, statt
die eigene zu löschen. Eine Marke, die zur Datei gehört, für die sie gemerkt
wurde, wäre die eine Antwort auf beide Fälle.

**Aufgefallen bei:** der Nachprüfung von `260810-1102` am 260810-1207.

---
Resolved: Gemessen am 260810-1315, und die Sorge trägt nicht: **AppKit setzt
`attachedSheet` nicht vor dem Abschlussblock auf nil, sondern rund 270 ms
danach.** Der Abschlussblock läuft eine Millisekunde nach dem Schließbefehl und
noch mitten in der Blattanimation; `attachedSheet` fällt erst mit deren Ende. Die
Reihenfolge ist damit die Umkehrung der vermuteten, die Sperre aus `blatt_steht`
greift bis zur ausgeführten Antwort einschließlich, und ein Kommando kann nicht
zwischen die beiden Zeitpunkte fallen. Kein Code geändert; `anlass_unterbleibt`
bleibt, wie es ist.

**Woran gemessen:** `spikes/blatt-spanne/` (Wegwerf-Prüfcode neben
`spikes/fn-tasten/`), Berichte `spikes/blatt-spanne/messung-griff.txt` und
`messung-klick.txt`. macOS 15.7.7 (Build 24G720) auf `MacBookPro15,1`, also auf
dem Referenzgerät der Zeitzusagen. Zwei voneinander unabhängige Arme:

- **Der Taktarm**, ein Zeitgeber mit einer Millisekunde Abstand auf der
  Hauptschleife, liest `attachedSheet` und hält fest, wann es erstmals nil ist.
  Er belegt zugleich, dass die Hauptschleife in der gemessenen Zeit arbeitet.
- **Der Tastenarm** wirft Tastendrücke über `NSApp.postEvent(_:atStart:)` in die
  eigene Ereignisschlange und sieht sie in einem lokalen `keyDown`-Abgriff
  wieder, also über denselben Mechanismus wie KRKs Ereignisabgriff. Jeder
  Tastendruck wird nach genau der Frage einsortiert, die `kommando_ausfuehren`
  stellt.

Beide Wege, auf denen KRK ein Blatt schließt, sind getrennt gemessen:
`endSheet(_:returnCode:)` wie in `Blattgriff::abbrechen` und im
`Eingabewaechter`, und `performClick(nil)` wie der Klick des Nutzers. Sie
stimmen überein.

| Was | `griff` | `klick` |
|---|---|---|
| `attachedSheet` im Abschlussblock | gesetzt | gesetzt |
| Abschlussblock nach dem Schließbefehl | +1,0 ms | +1,6 ms |
| `attachedSheet` erstmals nil | +270 ms | +272 ms |
| Takte der Hauptschleife mit nil vor dem Abschlussblock | 0 | 0 |
| Tastendrücke mit nil vor dem Abschlussblock | 0 | 0 |

**Die Messung war keine Nutzerarbeit**, anders als dieser Datensatz unter „Was
zuerst zu tun wäre" annahm. Er hielt sie für vordergrundgebunden, weil er sie
sich am laufenden Bündel dachte, mit einem von Hand beantworteten Blatt. Das
Prüfprogramm beantwortet sein Blatt selbst, über dieselben zwei Aufrufe, die
KRK dafür benutzt, und läuft als `.accessory` ohne Vordergrund.

**Die Gegenrichtung ist mitgemessen und kostet auch nichts.** In den 270 ms, in
denen `attachedSheet` nach der ausgeführten Antwort weiter ein Blatt meldet,
hält der Schließbefehl den Hauptfaden: kein Takt schlägt, und die vierzig vorab
in die Schlange gelegten Tastendrücke kommen alle erst danach an. Die Sperre
weist dort nichts ab, weil dort nichts eintrifft.

**Was die Messung nicht deckt:** ein `NSAlert` mit zwei Schaltflächen und ohne
Beigabe, nicht jedes der neun Blätter, und eine Systemfassung. Die gemessene
Größe ist eine Reihenfolge von AppKit; ändert Apple sie, ist die Sperre neu zu
prüfen.

**Nachgefahren am 260810, beide Arme, und der Befund hält.** Das Prüfprogramm
neu gebaut und beide Wege ein zweites Mal gemessen, auf demselben Gerät:
`attachedSheet` steht im Abschlussblock beide Male gesetzt, das erste `nil` fällt
269 ms (`griff`) und 272 ms (`klick`) **danach**, und weder ein Takt noch einer
der über dreihundert Tastendrücke fiel in die vermutete Spanne. Die Zahlen
stimmen mit der Tabelle darüber im Rahmen der Blattanimation überein. Die
Berichte in `spikes/blatt-spanne/` tragen den zweiten Lauf.
