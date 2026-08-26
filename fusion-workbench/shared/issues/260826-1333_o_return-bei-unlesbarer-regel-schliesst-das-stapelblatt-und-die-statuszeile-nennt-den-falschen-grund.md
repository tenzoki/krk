Return bei unlesbarer Regel schließt das Stapelblatt, und die Statuszeile nennt den falschen Grund

---

`Vorschauquelle::neu_rechnen` (`crates/krk-ui/src/appkit/blaetter/stapelumbenennen.rs:306-319`)
setzt bei einem `Regelfehler` — etwa `Nummer ab: x` — den Stand auf `Vorschau::default()` und
schreibt den Grund in die Hinweiszeile. Die Eingabetaste bleibt dabei auf „Umbenennen":
`zeigen` (`:436-440`) reicht `quelle.ergebnis()` weiter, also die leere Vorschau, das Blatt
geht zu, und `stapel_beauftragen` (`anwendung.rs:5863-5866`) meldet „nichts umzubenennen:
jede Zeile trägt einen Hinweis". Keine Zeile trägt einen Hinweis; die Regel war unlesbar, und
genau das stand eine Sekunde vorher im Blatt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Mittel
**Betroffen:** `crates/krk-ui/src/appkit/blaetter/stapelumbenennen.rs`, `crates/krk-ui/src/appkit/anwendung.rs`

## Warum es mehr als ein Wortlaut ist

Der Modulkopf (`:23-26`) begründet die Eingabetaste auf „Umbenennen" damit, dass „der Nutzer
die Vorschau vor sich hat, waehrend er die Taste drueckt". Bei unlesbarer Regel hat er keine:
die Tabelle ist leer, die Hinweiszeile zeigt einen Fehler, und dieselbe Taste, die sonst
ausführt, schließt jetzt das Blatt und wirft die vier eingetippten Felder weg. Wer den
Tippfehler berichtigen wollte, fängt von vorn an. Datenverlust ist es nicht — es wird nichts
umbenannt —, aber die Meldung sagt dem Nutzer etwas Falsches über den Grund.

## Denkbarer Weg

Zwei Stellen kennen den Unterschied, und eine reicht: entweder gibt `ergebnis()` bei
`Regelfehler` kein leeres `Vorschau` heraus, sondern der Rückruf bleibt aus (das Blatt bleibt
stehen, der Nutzer berichtigt), oder `stapel_beauftragen` unterscheidet „keine Zeilen" von
„nur Zeilen mit Hinweis" — `vorschau.zeilen().is_empty()` gegen `auszufuehren().count() == 0`
— und sagt beim ersten „keine Regel". Das Erste hält die Eingaben, das Zweite nur die
Wahrheit der Meldung.
