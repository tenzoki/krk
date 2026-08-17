# Die Löschfrage entsteht vor beiden Sperren, im leeren Fall mit null Einträgen

**Datum:** 260817-1108
**Gefunden von:** coderev, Durchsicht `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`, Befund 3
**Schwere:** Niedrig
**Betrifft:** `crates/krk-ui/src/appkit/anwendung.rs`
**Baumstand:** `472eb81`

## Der Befund

`in_den_papierkorb` (`anwendung.rs:4454-4467`) liest die Auswahl und baut beide Texte, bevor
`loeschen_nach_rueckfrage` nach dem laufenden Vorgang und nach der leeren Auswahl fragt. In
zwei der vier Ausgänge werden die Texte verworfen.

Im leeren Fall entsteht dabei „Diese 0 Einträge in den Papierkorb räumen?":
`frage_und_erlaeuterung` kennt die Einzahl nur für `1` und fällt sonst in den Mehrzahlzweig.
Auf den Schirm kommt der Satz nicht, weil die Sperre vorher greift.

Kosten daneben: zwei Durchgänge über die Auswahl je Tastendruck statt einem. Bei einer großen
Markierung sind das zwei Vektoren aus `PathBuf` auf dem Hauptfaden.

## Richtung

Schritt 11 des Plans zieht das Bauen der Texte in den Rumpf, weil die Frage dort die
Warngründe braucht, und nimmt beides mit. Dieser Datensatz steht, damit der Befund nicht
verlorengeht, falls der Zuschnitt von Schritt 11 sich noch ändert. Er ist bis dahin kein
Fehlverhalten, sondern verworfene Arbeit und ein Satz, der nie erscheint.

---
Abgleich 260817-1129 (reconciler): **offen, am Baum nachgelesen.** `in_den_papierkorb` (`anwendung.rs:4454-4467`) baut beide Texte unverändert vor dem Aufruf des Rumpfes; die beiden Sperren stehen weiter erst darin. Schritt 11, der den Befund auflösen soll, ist nicht gebaut.

---
Resolved: 260817-1806 (coder, Aufgabe T10, Schritt 11 des Plans). Das Bauen der beiden Texte
ist aus `in_den_papierkorb` in den gemeinsamen Rumpf gezogen und steht dort im **vierten** Zweig
der Stufenregel, also hinter allen drei Sperren. `in_den_papierkorb` reicht jetzt nur noch die
drei Stücke weiter, in denen sich die beiden Löschbefehle unterscheiden: die Auftragsart, die
Beschriftung der zweiten Schaltfläche und die neue Aufzählung `Loeschtexte`, die sagt, woher die
Texte kommen. Der Wortlaut selbst gehört weiterhin `kommandos::loeschwarnung` und nicht
`appkit/anwendung.rs`.

Damit ist beides weg, was der Befund nennt. **Verworfene Arbeit:** in keinem der drei Ausgänge,
die vor dem Blatt anhalten, entsteht noch ein Text. **Der Satz, den nie ein Schirm zeigte:**
„Diese 0 Einträge in den Papierkorb räumen?" kann nicht mehr entstehen, weil der leere Fall die
Textstelle nicht erreicht; `frage_und_erlaeuterung` behält seinen Mehrzahlzweig für `0`
unverändert, denn er ist jetzt unerreichbar und nicht falsch. **Die Kosten:** die Auswahl wird
einmal je Tastendruck gelesen statt zweimal, also ein Vektor aus `PathBuf` statt zweier.

Verifikation: `make check` — exit 0. Die Abrechnung im Einzelnen steht in
`history/260817-1806-coder-t10-die-laute-warnform.md`.
