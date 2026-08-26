Der Wegwerfordner räumt den Steckbrief neben sich ab, die zweite Nachbardatei `.zeitmarke` nicht

---

`Wegwerfordner::drop` (`crates/krk-bench/src/wegwerfordner.rs:54-63`) nimmt den Ordner und den
Steckbrief daneben mit und begründet das ausdrücklich: „Der Steckbrief liegt **neben** dem Ordner
und faellt deshalb nicht mit ihm; ohne diese Zeile blieben Dateien im Temporaerverzeichnis
liegen." Der Erzeuger legt aber **zwei** Dateien neben den Ordner, nicht eine. Die zweite ist
`<ordner>.zeitmarke` aus `markenpfad` (`fixture.rs:485-486`), und der Wächter kennt sie nicht.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-bench/src/wegwerfordner.rs`, `crates/krk-bench/src/fixture.rs`

## Wann sie stehen bleibt

`verknuepfungszeiten_setzen` (`fixture.rs:440-460`) räumt die Marke auf beiden gewöhnlichen
Wegen ab: `let ergebnis = touch_laufen_lassen(&marke, verknuepfungen); let _ =
fs::remove_file(&marke); ergebnis` (`fixture.rs:457-459`, gezählt ab `let ergebnis`) — auch dann, wenn `touch` gescheitert
ist. Ungedeckt ist genau ein Weg: `let datei = File::create(&marke)?;` gelingt (`fixture.rs:453`, `let datei = File::create(&marke)?;`)
und `datei.set_times(zeiten(zeitpunkt))?;` scheitert (`fixture.rs:454`, `datei.set_times(…)?;`). Dann kehrt die Funktion
mit `?` zurück, und die eben angelegte Marke bleibt stehen.

**Das ist ein schmaler Weg, und ich behaupte nicht, dass er je eingetreten ist.** Er ist derselbe
Weg, den `Messplanwaechter` an seiner Stelle ausdrücklich schließt: „Der Name steht fest, bevor
irgendetwas angelegt wird … Wuerde der Waechter erst aus dem Ergebnis des Schreibens entstehen,
kehrte das `?` vorher zurueck und die Datei bliebe liegen" (`messen.rs:1584-1598`). Der Erzeuger
hat diese Bauform nicht.

## Warum es über die Proben hinausgeht

Die Marke liegt neben dem **Prüfordner**, nicht neben einem Wegwerfordner. Bei einem
`krk-bench fixture --out <pfad>/a` bliebe sie als `<pfad>/a.zeitmarke` liegen — im Messplatz
unter `~/Library/Caches/krk-messplatz`, dauerhaft, neben einem Prüfordner, der bis zum nächsten
Abnahmelauf steht. Anders als der Steckbrief sagt sie nichts aus; sie sieht nur aus wie etwas,
das dazugehört.

## Denkbarer Weg

Zwei Möglichkeiten, beide klein. Entweder trägt `Wegwerfordner::drop` die zweite Nachbardatei
mit, wie er die erste trägt — dann bleibt der schmale Weg im Erzeuger selbst offen, für alles
außerhalb der Proben. Oder `verknuepfungszeiten_setzen` bekommt einen Wächter derselben Bauform
wie `Messplanwaechter`: Name zuerst, Wächter zuerst, dann anlegen. Die zweite Fassung deckt beide
Rufer ab und ist die, die diese Kiste an ihrer anderen Stelle schon gewählt hat.
