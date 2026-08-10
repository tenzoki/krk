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
