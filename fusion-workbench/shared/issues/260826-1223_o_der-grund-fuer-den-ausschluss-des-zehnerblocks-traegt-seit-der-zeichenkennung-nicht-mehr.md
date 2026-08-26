Der Grund, mit dem zwei Prosastellen den Zehnerblock ausschließen, trägt seit der Zeichenkennung nicht mehr

---

`parser.rs` begründet den Ausschluss des Zehnerblocks zweimal damit, dass er **eigene Tastencodes** trage. Seit der Runde 2 fragt der Nachschlag für Ziffern den Tastencode gar nicht mehr, sondern das gemeldete Zeichen. Der genannte Schutz besteht damit nicht: eine Zifferntaste des Zehnerblocks landet auf demselben Eintrag wie die der oberen Reihe, ununterscheidbar. Die Eingabetaste des Zehnerblocks landet umgekehrt nirgends.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die zwei Prosastellen

`crates/krk-core/src/tasten/parser.rs:237-238`:

> Nicht enthalten sind die Satzzeichen und der Zehnerblock.

`crates/krk-core/src/tasten/parser.rs:302-303`:

> Die Ziffern der oberen Reihe. Der Zehnerblock traegt eigene Codes und steht nicht in der Schreibweise.

Dazu `normalisierung.rs:34-37`, das das Bit `NSEventModifierFlagNumericPad` löscht, weil es „eine Eigenschaft der gedrueckten Taste und keine gehaltene Zusatztaste" sei.

## Warum der Grund nicht mehr trägt

`Tastendruck::aus_ereignis` (`tasten/mod.rs:98-104`) setzt `zeichen: gemeldet.and_then(parser::zeichen_als_kennung)`. `zeichen_als_kennung` (`parser.rs:393-396`) nimmt **jedes** ASCII-alphanumerische Zeichen an, ohne den Tastencode anzusehen. `Tastendruck::kennung` (`mod.rs:110-115`) antwortet daraufhin `Tastenkennung::Zeichen(...)`, und `Belegung::nachschlag` (`belegung.rs:1289-1300`) vergleicht allein Maske und Kennung.

Die Kette sieht den Tastencode einer Zifferntaste an keiner Stelle. Der Satz „der Zehnerblock trägt eigene Codes" beschreibt damit eine Eigenschaft, aus der seit der Runde 2 nichts mehr folgt. Dass die Codes 82–92 nicht in `TASTEN` stehen, hält die Taste nicht heraus — es hält allein ihren **Namen** aus der Schreibweise heraus.

Dazu kommt: `normalisieren` (`normalisierung.rs:181-196`) löscht `ZEHNERBLOCK`, also trägt die Maske keinen Unterschied mehr, und `roh::ZEHNERBLOCK` (`normalisierung.rs:67`) hat im ganzen Baum außerhalb der Proben keinen Leser. Es gibt keine Größe mehr, an der die zwei Tasten auseinanderzuhalten wären.

## Die Ungleichheit, die daraus folgt

- **Zifferntasten des Zehnerblocks:** melden ein Zeichen `'0'`–`'9'`, gehen über `Tastenkennung::Zeichen` und treffen den Eintrag der oberen Reihe. Jede Belegung auf einer Ziffer wirkt damit auf zwei Tasten.
- **Eingabetaste, Komma, Plus, Minus, Mal, Geteilt des Zehnerblocks:** melden `'\r'`, `'.'`, `'+'` und so fort, kommen durch `zeichen_als_kennung` nicht durch, fallen auf `Tastenkennung::Code(76)` und so weiter — und diese Codes stehen in `TASTEN` nicht. Sie lösen nichts aus. Die Eingabetaste des Zehnerblocks tut insbesondere **nicht**, was `return` tut.

`inference:` Der erste Punkt ist am Baum abgeleitet und **nicht am Referenzgerät gemessen**: dass AppKit für die Zifferntaste des Zehnerblocks über `charactersByApplyingModifiers(empty)` (`krk-ui/src/appkit/ereignisse.rs:742-745`) die Ziffer meldet, ist die Annahme, unter der die Kette greift. Am Quelltext allein entscheidbar und nicht von dieser Annahme abhängig ist die eigentliche Aussage dieses Befunds: **der Code wird für Ziffern nicht mehr befragt, und der in der Prosa genannte Schutz existiert nicht.**

## Was zu tun ist

Zwei getrennte Stücke, und das erste ist das dringlichere:

1. Die zwei Prosastellen nennen den Grund, den es gibt, statt des Grundes, den es nicht mehr gibt: der Zehnerblock hat **keinen Namen** in der Schreibweise, ist deshalb nicht von Hand belegbar und nicht in der Belegungsansicht zuweisbar (`Kombination::aus_tastendruck` liefert für ihn `None`, `parser.rs:570-577`) — er wirkt aber über das gemeldete Zeichen auf jeder Ziffernbelegung mit.
2. Ob das gewollt ist, ist offen und gehört dem Nutzer vorgelegt. Für die Maxime „Steuerung über die Tastatur" spricht, dass eine Ziffer eine Ziffer ist; gegen den Stand spricht, dass die Eingabetaste desselben Blocks genau umgekehrt behandelt wird und niemand das entschieden hat.

Gefunden bei der Vollbaum-Durchsicht R4 an HEAD `004ff72`.
