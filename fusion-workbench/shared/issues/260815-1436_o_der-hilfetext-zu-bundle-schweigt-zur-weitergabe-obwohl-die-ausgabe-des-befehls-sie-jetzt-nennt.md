Der Hilfetext zu `bundle` schweigt zur Weitergabe, obwohl die Ausgabe des Befehls sie jetzt nennt

---

`cargo xtask --help` beschreibt unter `bundle` allein die dreistufige Suche nach der
Signaturidentität. Seit dem Abschlusshinweis dieses Befehls gibt der Lauf selbst Auskunft
darüber, ob das Ergebnis weitergegeben werden kann; der Hilfetext tut es nicht. Wer vor dem
Lauf entscheiden will, welchen Unterbefehl er braucht, findet die Auskunft also erst hinterher.

---

**Schwere:** niedrig. Kein Verhalten, kein Bau, keine Probe hängt daran.
**Gefunden von:** coder, zweimal während der Arbeit an
`shared/issues/260812-1628_*_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md`
**Betroffen:** `xtask/src/main.rs`, der Hilfetext
**Domain:** code

## Warum es der Rede wert ist

Der geschlossene Datensatz `260812-1628` beschreibt den Fall, in dem ein Nutzer ein Bündel auf
einen zweiten Mac kopiert hat und Gatekeeper es abgewiesen hat. Die Lücke lag darin, dass ihn
nichts auf `cargo xtask release` gestoßen hat. Der Abschlusshinweis schließt sie **nach** dem
Bau. Der Hilfetext ist die Stelle, an der sie **vor** dem Bau zu schließen wäre, und er ist die
einzige, die jemand liest, der den passenden Unterbefehl erst sucht.

## Abgrenzung

Der Hinweis am Ende des Laufs ist gebaut und geprüft (`sign::weitergabehinweis`, vier Proben).
Dieser Datensatz betrifft allein den Hilfetext.

## Herkunft

Gemeinsamer Speicher. Betrifft den Bauweg des ganzen Projekts und nicht die Directive einer
Runde; gefunden neben der Arbeit an einem Defekt desselben Speichers.
