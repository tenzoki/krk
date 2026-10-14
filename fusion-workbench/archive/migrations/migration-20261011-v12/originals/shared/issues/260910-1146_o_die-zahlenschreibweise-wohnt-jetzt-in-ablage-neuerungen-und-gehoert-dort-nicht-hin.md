Die Zahlenschreibweise wohnt jetzt in `ablage::neuerungen` und gehört dort nicht hin

---

Der Rumpf der Zahlenschreibweise mit Tausenderpunkten ist beim Bau des Schrittes 5 der Runde
`260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap` von `krk-ui` nach
`krk-core/src/ablage/neuerungen.rs` gezogen. `operationen::zahl` ist seitdem ein
`pub(crate) use` darauf. Der Umzug war nötig und der Zielort ist trotzdem falsch: eine
allgemeine Zahlenschreibweise gehört nicht in ein Modul über Neuerungen an Ablagedateien.
Wer sie sucht, sucht sie dort nicht.

---

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** vom Bau des Schrittes 5 selbst, der den Zielort im eigenen Bericht als nicht
ganz richtig benannt hat.

## Warum der Umzug nötig war

Der Blatttext liegt in `krk-core`, und der Kern kann `krk-ui` nicht rufen. Wäre die
Zahlenschreibweise in `krk-ui` geblieben, hieße derselbe Rest im einen Blatt `1.234` und im
anderen `1234`. Zwei Schreibweisen für dieselbe Zahl sind schlechter als ein unpassender
Wohnort, und der Schritt hat richtig entschieden.

## Warum es trotzdem ein Befund ist

Die Dateiliste des Schrittes nennt zwei Dateien. Ein neutrales drittes Modul im Kern
anzulegen wäre über den Schritt hinausgegangen, und der Bauer hat sich daran gehalten statt
den Zuschnitt eigenmächtig zu weiten. Damit steht die Sache jetzt am falschen Ort und
niemand ist dafür zuständig, ausser diesem Datensatz.

## Abnahme

Die Zahlenschreibweise steht in einem Modul, dessen Name sie trägt, und `ablage::neuerungen`
ruft sie von dort. Die drei vorhandenen Rufer in `auswahl.rs`, `loeschwarnung.rs` und
`statuszeile.rs` bleiben unverändert oder ziehen mit; der Bericht des Schrittes nennt den
Umzug mit zwei Zeilen Aufwand.
