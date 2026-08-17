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
