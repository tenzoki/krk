Die Zählprobe aus Schritt 3 kann nicht null erwarten: `setSelectable(false)` steht an einem `NSTextField`

---

Der Plan (`planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Schritt 3) und die Dispatch-Anweisung verlangen beide „eine Zählprobe über `crate::quellbaum`, dass `setSelectable(false)` im Baum nicht mehr vorkommt".

Das trifft den Baum nicht. `crates/krk-ui/src/appkit/belegungsansicht.rs:677` setzt `meldung.setSelectable(false)` an der Meldungszeile des Belegungsblattes. Das ist ein `NSTextField` und keine Textanzeige, hat mit dieser Runde nichts zu tun und soll stehen bleiben. Eine Probe mit der Erwartung „kommt nicht mehr vor" wäre vom ersten Lauf an rot gewesen.

Daneben nennt `crates/krk-ui/src/appkit/textautomatik.rs:20` die Zeichenfolge in Prosa; eine Nadel, die Kommentarzeilen nicht abzieht, zählt sie mit.

Dieselbe Beobachtung gilt für die zweite Hälfte: `setEditable(false)` steht im ganzen Baum an **genau einer** Stelle als Code, nämlich in `appkit/vorschau.rs`. „An den bekannten Stellen" ist im Plural formuliert und meint eine einzige.

---

Resolved: die Probe `die_zwei_schalter_stehen_je_an_genau_einer_stelle_und_dort` (`crates/krk-ui/src/appkit/vorschau.rs`, Prüfmodul) zählt statt einer Null die Fundstellen je Datei und vergleicht gegen die Lage am Baum — `setSelectable(false)` genau einmal in `belegungsansicht.rs`, `setEditable(false)` genau einmal in `vorschau.rs`. Kommentarzeilen zieht sie ab, beide Nadeln stehen über `concat!` zusammengesetzt da. Der Doc-Kommentar der Probe schreibt beide blinden Flecken aus: sie zählt Codezeilen und keine Aufrufe, und sie unterscheidet `NSTextView` nicht von `NSTextField`. Damit hält sie, was die Runde braucht — die abgelöste Zeile ist weg und kommt nicht als Kopie zurück —, ohne eine Zahl zu behaupten, die der Baum nicht trägt.
