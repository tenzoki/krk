Die Dateitabelle der HowTo führt sieben Ablagedateien; seit der Runde 24 sind es acht

---

`HowTo.md:24-30`, Abschnitt „Wo KRK seine eigenen Dateien ablegt", führt `keymap.toml`,
`bookmarks.toml`, `session.toml`, `settings.toml`, `readers.toml` und die zwei Notizzettel.
`reported.toml` fehlt, obwohl die Runde 24 sie als achte Ablagedatei gebaut hat und der Satz
unter der Tabelle den Leser ausdrücklich auf `Datei::ALLE` als die Quelle verweist — dort
stehen acht.

---

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** Durchsicht der Runde 24, Bereich `feecd6c..d9535c4`, Schritt 10
**Betroffen:** `HowTo.md:24-30`

## Warum es keine Probe gefangen hat

`keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien`
(`crates/krk-core/tests/baum.rs`) liest allein `crates/krk-core/src/ablage/`. Eine Zahl oder
eine Aufzählung in `HowTo.md` bekommt sie nicht zu sehen, und die Tabelle nennt ohnehin keine
Zahl, sondern zählt auf — die Lücke ist damit von keiner Seite gedeckt.

Die Dateiliste des Schrittes 10 nennt `README.md` und `HowTo.md`; die Stelle lag also im
Zuschnitt und ist nicht übersehen worden, weil sie außerhalb gelegen hätte.

## Abnahme

Die Tabelle führt `reported.toml` mit ihrem Inhalt („für welche Fassung die Neuerungen
gemeldet sind") und ihrem Schreiber (KRK). Gezählt gegen

```sh
awk '/pub const ALLE: \[Datei;/,/\];/' crates/krk-core/src/ablage/pfade.rs
```

also gegen genau das Kommando, das der Satz unter der Tabelle dem Leser selbst empfiehlt.

---
Resolved: ff48cde — die Tabelle in `HowTo.md` fuehrt `reported.toml` mit ihrem Inhalt
("fuer welche Fassung die Neuerungen an den eigenen Dateien gemeldet sind") und KRK als
Schreiber. Gezaehlt gegen `Datei::ALLE`, also gegen die Quelle, auf die der Satz unter der
Tabelle den Leser selbst verweist.

Nicht behoben und keine Aufgabe dieses Durchgangs: die Probe
`keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien` liest weiter allein
`crates/krk-core/src/ablage/` und sieht `HowTo.md` nicht. Dieselbe Luecke faengt die naechste
Ablagedatei wieder nicht.
