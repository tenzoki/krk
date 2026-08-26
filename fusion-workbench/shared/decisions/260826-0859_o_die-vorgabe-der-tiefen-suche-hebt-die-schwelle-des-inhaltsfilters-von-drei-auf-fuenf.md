# Bleibt die Schwelle des Inhaltsfilters an den Stand der tiefen Suche gekoppelt, nachdem die Vorgabe auf „ein“ steht?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Cross-references:** `crates/krk-core/src/verzeichnis/filter.rs:127-158` (`inhaltsschwelle`, die eine Stelle der Staffelung); `crates/krk-core/src/verzeichnis/modell.rs` (`Ordnermodell::neu`, die eine Stelle der Vorgabe, und `inhalt_wirkt`, der eine Frager); `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content` (die Runde, die die Staffelung gesetzt hat); `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_o_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md` (offen, und dieselbe Vorgabe ist ihr Gegenstand von der anderen Seite)

---

## Question

Die Vorgabe des Ankreuzfelds „Deep“ steht seit dem 260826 auf „ein“; der Nutzer hat genau das verlangt. Die Schwelle des Inhaltsfilters hängt an demselben Wert: `inhaltsschwelle(tief)` gibt fünf bei eingeschalteter tiefer Suche und drei sonst. Damit hat sich eine zweite Größe mitverschoben, die niemand angefordert hat. Ein frisch gestarteter Nutzer, der „Content“ anhakt und `abc` tippt, bekam bis gestern Inhaltstreffer und bekommt heute keine; er sieht sie erst ab `abcde`.

Die Kopplung war richtig, solange die tiefe Suche eine bewusste Handlung war. Die höhere Schwelle bezahlt die teurere Suche: ein flacher Inhaltsfilter liest die Dateien des angezeigten Ordners, ein tiefer die des ganzen Unterbaums, und drei Zeichen sagen zu wenig, um diesen Preis zu rechtfertigen. Die Begründung steht ausgeschrieben in `filter.rs:127-134`. Was sich geändert hat, ist nicht diese Überlegung, sondern wer sie auslöst: bis gestern der Nutzer mit einem Klick, seit heute die Auslieferung.

Zu entscheiden ist jetzt, weil die Vorgabe steht und die Verschiebung damit ausgeliefert wird, sobald die nächste Version geht.

## Options

1. **Die Kopplung bleibt, die Vorgabe bleibt.** Ab Werk gilt die Fünf.
   - Pro: eine Regel, eine Stelle, keine zweite Wahrheit über die Schwelle. Die Begründung der Staffelung gilt unverändert — der Lauf ist tief und damit teuer, gleich wer ihn eingeschaltet hat, und die Vorgabe schaltet ihn nicht weniger tief. Nichts ist zu bauen.
   - Kontra: eine Verhaltensänderung, die aus der angeforderten folgt und nicht selbst angefordert war. Zwei zusätzliche Zeichen, bevor der Inhaltsfilter überhaupt anspringt, und der Nutzer sieht keinen Grund dafür: „Deep“ hat er nicht angehakt, es stand schon.
2. **Die Staffelung fällt, eine Schwelle für beide Stände.** `inhaltsschwelle` wird eine Zahl statt einer Funktion über `tief`.
   - Pro: der Inhaltsfilter verhält sich wieder unabhängig davon, wie „Deep“ steht, und die Vorgabe verschiebt gar nichts mehr. Eine Größe weniger, an der die Schwelle hängt.
   - Kontra: die Staffelung hat einen Gegenstand, und der verschwindet nicht mit ihr. Bei drei Zeichen und tiefer Suche liest KRK dann den ganzen Unterbaum aus; genau davor sollte die Fünf schützen. Wer sie streicht, entscheidet damit auch, dass dieser Schutz nicht nötig war, und dafür liegt keine Messung vor.
3. **Die tiefe Schwelle sinkt auf vier.** Die Staffelung bleibt, ihr Abstand halbiert sich.
   - Pro: der Sprung, den der Nutzer heute spürt, wird kleiner, ohne den Schutz aufzugeben.
   - Kontra: eine dritte Zahl ohne Grundlage. Weder die Drei noch die Fünf ist gemessen, und die Vier wäre es erst recht nicht; sie schöbe die Frage weiter, statt sie zu beantworten.

## Constraints

- Die Schwelle steht an genau einer Stelle (`filter::inhaltsschwelle`) und wird an genau einer gefragt (`Ordnermodell::inhalt_wirkt`). Jede Antwort hält beides; eine zweite Schwelle daneben ist keine Möglichkeit.
- Ohne stehenden Filtertext ändert weder „Deep“ noch „Content“ etwas an der Liste. Der Unterschied zwischen den Möglichkeiten wird erst sichtbar, sobald der Nutzer tippt.
- Die Vorgabe „Deep = ein“ steht und ist nicht Gegenstand dieser Frage. Sie ist ausdrücklich verlangt worden.
- Keine der drei Zahlen ist auf dem Referenzgerät gemessen. Eine Antwort, die sich auf Kosten beruft, beruft sich auf eine Schätzung.

## Recommendation

Möglichkeit 1, bis eine Messung vorliegt. Die Begründung der Staffelung hat sich nicht geändert, nur ihr Auslöser, und ein tiefer Lauf ab Werk ist genau der Fall, für den die höhere Schwelle geschrieben wurde — nicht der Fall, der sie widerlegt. Die Kosten der Verschiebung sind zwei Zeichen bei eingeschaltetem „Content“, und „Content“ ist weiter aus die Vorgabe; wer es anhakt, hat die teurere Suche schon einmal bewusst gewählt.

Was gegen Möglichkeit 2 fehlt, ist dieselbe Zahl, die auch für sie fehlt: wie lange ein Inhaltsfilter über einen echten Unterbaum bei drei Zeichen tatsächlich braucht. Wer sie misst, entscheidet die Frage; wer sie nicht misst, tauscht eine unbegründete Zahl gegen eine andere.
