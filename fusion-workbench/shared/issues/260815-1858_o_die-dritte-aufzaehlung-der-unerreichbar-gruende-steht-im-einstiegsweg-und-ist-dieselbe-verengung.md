Die dritte Aufzaehlung der `Unerreichbar`-Gruende steht im Einstiegsweg und traegt dieselbe Verengung

---

`crates/krk-ui/src/appkit/tabelle.rs:1432` kommentiert den `Unerreichbar`-Zweig von
`in_zeile_einsteigen` mit „Ins Leere, im Ring oder ohne Recht: nicht still verschlucken."
Das ist wortgleich dieselbe dreigliedrige Aufzaehlung, die der Befund
`shared/issues/260815-1845_*_der-doc-kommentar-von-unerreichbar-zaehlt-drei-gruende-auf-und-stat-scheitert-an-mehr.md`
am Doc-Kommentar des Wertes gefunden hat. Ein zu langer Name (`errno 63`) faellt unter keinen
der drei, und `ELOOP` entsteht ab 32 aufgeloesten Verknuepfungen auch ohne Ring.

---

**Gefunden am:** 260815-1858, waehrend der Behebung von `260815-1845`
**Gefunden von:** coder
**Schwere:** niedrig. Kein Fehlverhalten am Code; die Verzweigung ist richtig und faengt
jeden Fehlschlag von `stat(2)`. Falsch ist allein die Beschreibung.
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs:1432`
**Domain:** code

## Warum die Stelle nicht im selben Zug behoben ist

Der Auftrag vom 260815-1850 zog seine Grenze um die vier genannten Datensaetze. `260815-1845`
fuehrt unter **Betroffen** `verweisziel.rs:129-133` und mitbetroffen
`tests/verzeichnis.rs:1934-1937`; die Stelle in `tabelle.rs` steht in keinem der vier
Datensaetze und in keiner ihrer Fundstellenlisten. Beide genannten Stellen sind nachgezogen,
diese ist es nicht.

## Was zu tun ist

Denselben Halbsatz nachziehen, den `260815-1845` fuer die anderen zwei Stellen vorgibt: die
Faelle als Beispiele und nicht als Aufzaehlung. Der Kommentar hat hier zwei Saetze, und der
zweite — die Statuszeile aus C1 als die eine Meldeflaeche — bleibt richtig.

## Ablage

Gemeinsamer Speicher. Betrifft die Oberflaeche und die Directive keiner Runde.

---
Abgleich 260819-1440 (reconciler, Baumstand `77dcd48`): **offen und wortgleich, nur verschoben.** Die Zeile „Ins Leere, im Ring oder ohne Recht: nicht still verschlucken." steht heute in `crates/krk-ui/src/appkit/tabelle.rs:1984`; bei der Ablage stand sie bei `:1432`. Die Aufzählung ist unverändert dieselbe Verengung auf drei Gründe. Wer den Datensatz nach der alten Zeilennummer prüft, findet nichts und schließt ihn fälschlich.
