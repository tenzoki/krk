„Gemessen" reicht für „Textfelder" weiter als die Messung, und die Begründungslagen sind zweimal verschieden gezählt

---

Zwei kleine Ungenauigkeiten an derselben Stelle: der Beschreibung der dritten Spalte. Beide sind
Text und kein Verhalten, und beide betreffen genau die Fehlerform, die der Spec dieses Circles
unter `## Was die Abnahme mitentscheidet` als teuerste benennt.

---

**Schwere:** Niedrig
**Gefunden:** coderev, Durchsicht des Codeanteils von Turn 1
**Betroffen:** `crates/krk-ui/src/belegungsausgabe.rs` (Modulkopf, `wirkung`),
`crates/krk-ui/src/appkit/menue.rs` (Modulkopf),
`issues/260811-0930_*_die-ableitung-textfelder-und-editor-bricht-*.md`
**Domain:** code

## a) „Textfelder" ist zur Hälfte gemessen und zur Hälfte AppKit-Wissen

Die Tabelle im Modulkopf von `belegungsausgabe.rs:51` führt:

| `text_ausschneiden`, `text_kopieren`, `text_einfuegen` | „Textfelder und Editor" | in S1 am Laufzeitsystem **gemessen** |

Gemessen hat S1 dies: `cut:`, `copy:` und `paste:` werden von `NSTextView` beantwortet, die
Methode sitzt an `NSText`, und `NSTextField` beantwortet keinen der drei. Das ist eine Aussage
über **Klassen**.

Der Schritt von dort zu „Textfelder" ist ein zweiter, und er ist nicht gemessen: er beruht
darauf, dass der Feldeditor eines `NSTextField` eine `NSTextView` ist. Das ist eine zugesagte
Eigenschaft von AppKit und keine Erfindung — aber `AnyClass::responds_to` hat sie nicht geprüft,
denn es legt keine Instanz an und fragt nichts über den Ersthelfer.

Der Zweig selbst schreibt die Kette sauber aus (`belegungsausgabe.rs:218-226`: „Erreicht wird
also der **Feldeditor** des Textfeldes, der eine `NSTextView` ist und `NSText` mitbringt"). Wer
nur die Tabelle im Modulkopf liest, sieht davon nichts und nimmt die ganze Zelle als Messwert
mit. Deutlicher noch der Datensatz `260811-0930`:

> Genau das behauptete der Modulkopf von `menue.rs` schon; **jetzt ist es gemessen.**

Gemessen ist die eine Hälfte. Die andere ist AppKit-Wissen, und dieses Projekt trennt das sonst
über `inference:`.

**Behebung:** eine Nebenbemerkung in der Tabelle des Modulkopfs und ein `inference:` an der
zitierten Zeile des Datensatzes. Am Zweig selbst ist nichts zu ändern — dort steht es richtig.

## b) Die Lagen sind zweimal verschieden gezählt

- `belegungsausgabe.rs:45` sagt: „Die Spalte ‚Wirkt in' hat **drei verschiedene Quellen**", und
  die Tabelle darunter (`:48-53`) führt **vier** Zeilen.
- Der Doc-Kommentar von `wirkung` (`:204-206`) sagt richtig, die **sechs** zugestellten
  Textbefehle trügen drei Lagen. Die Zweigkommentare zählen dann aber über alle 71: „Erste
  Lage" (:208), „Zweite Lage" (:218), „Dritte Lage" (:228) — und dann `:246` „**Vierte Zeile,
  dritte Lage**". Die Ziffer drei steht damit an zwei verschiedenen Sachverhalten.
- Die Probe zählt wieder anders: „Dritte Lage, erste Haelfte" (:635) und „Dritte Lage, zweite
  Haelfte" (:646).

Der Sachverhalt ist überall derselbe und richtig auseinandergehalten; allein die Nummerierung
läuft über drei Stellen in drei Fassungen. Wer sie zum Nachschlagen benutzt, greift daneben.

**Behebung:** eine Zählweise wählen — entweder vier Lagen über alle 71 oder drei über die sechs
— und die Zweigkommentare, den Modulkopf und die Probe darauf einstellen.

## c) Eine Randnotiz

`menue.rs` schreibt im ersten Befund: „**die vier Zwischenablage-Befehle haengen an `NSText`**".
Der Begriff „die vier Zwischenablage-Befehle" ist im Modulkopf derselben Datei eingeführt und
schließt `selectAll:` ein; für `selectAll:` gilt der Satz aber nur zur Hälfte, weil `NSTableView`
ihn ebenfalls trägt. Der zweite Befund gleich darunter räumt das aus, sodass niemand mit dem
falschen Bild weitergeht. Beim Nachziehen von b) kann die Zeile mit.

---
Nachtrag 260811-1045: zur Haelfte erledigt, der Datensatz bleibt offen.

**Erledigt ist der erste Punkt im Spec, nicht im Code.** Die Berichtigung von C3 hat den
erschlossenen Anteil an "Textfelder" ausdruecklich mit `inference:` gekennzeichnet, wie dieser
Datensatz es verlangt: gemessen ist, dass `cut:`, `copy:` und `paste:` an `NSText` haengen und
`NSTextField` sie selbst nicht beantwortet; dass daraus der Feldeditor folgt, ist erschlossen.

**Offen bleiben beide Punkte am Programmtext:** das Wort "gemessen" im Modulkopf und am Zweig
von `belegungsausgabe.rs` reicht fuer die Textfeld-Kette weiter als die Messung, und die drei
Begruendungslagen sind an drei Stellen verschieden gezaehlt. Der `coder` hat sie unter der
Grenze "nur diese vier Befunde" nicht angefasst und das gemeldet, statt sie stillschweigend
mitzunehmen.
