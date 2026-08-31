Der Modulkopf der Bereichsleiste nennt eine Zählvorschrift, die es nicht gibt

---
Schritt 12 der Runde 23 hat die Zahl der Ankreuzfelder aus dem Modulkopf von `crates/krk-ui/src/appkit/bereichsleiste.rs` genommen und durch eine Erhebungsvorschrift ersetzt. Die Vorschrift läuft nicht (`:11-13`):

> Gezaehlt werden sie mit `Bereichsleiste::alle_schalter().len()`, das ueber dieselben zwei Aufzaehlungen laeuft, aus denen [`Bereichsleiste::bauen`] baut

`Bereichsleiste` hat keine Methode `alle_schalter`. Was es gibt, ist eine freie Funktion `alle_schalter()` im `#[cfg(test)]`-Modul derselben Datei (`:787`); sie ist privat, nur im Probenbau übersetzt und von außerhalb der Datei nicht erreichbar. Der einzige laufende Zähler ist die Probe `zwoelf_schalter_der_leiste_tragen_ein_kommando` (`:810`), die der Kopf im selben Absatz nennt.

Der Nachzug hat damit eine Zahl, die veraltet, gegen eine Vorschrift getauscht, die nicht ausführbar ist. Das ist genau der Fall, den C9.2 der Runde 23 verhindern soll („zu prüfen daran, dass das dort ausgeschriebene `grep` läuft").

**Abnahmetest:** der Modulkopf nennt einen Befehl oder einen Probennamen, der sich ohne Umbau ausführen lässt und die Zahl liefert.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23 durch Nachlaufen der Erhebungsvorschriften, die Schritt 12 gesetzt hat.
