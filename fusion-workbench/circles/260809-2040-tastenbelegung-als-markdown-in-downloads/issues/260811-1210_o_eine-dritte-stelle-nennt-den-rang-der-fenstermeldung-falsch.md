Eine dritte Stelle nennt den Rang der Fenstermeldung falsch

---

`crates/krk-ui/src/appkit/anwendung.rs:3620` sagt, eine Meldung gehe „einen Rang tiefer als eine
Befehlsantwort". Sie geht über `meldung_zeigen` und damit auf die **Fenstermeldung**, und die
steht auf **Rang 3** — zwei Ränge tiefer, nicht einen.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Behebung von
`260811-0838_*_antwort-zeigen-nennt-vier-raenge-die-statuszeile-fuehrt-fuenf.md`
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:3620`
**Domain:** code

## Zusammenhang

Es ist die dritte Stelle desselben Fehlers. Der Datensatz `260811-0838` führte eine (`:3334`,
vier Ränge statt fünf), die Nachbarprüfung fand die zweite (`Dateifenstersicht::melden`,
„einen Rang tiefer" statt Rang 3), und beide sind behoben. Diese dritte lag außerhalb der
Auftragsgrenze; der `coder` hat sie gemeldet, statt sie stillschweigend mitzunehmen.

Die fünf Ränge, nachgezählt an `crates/krk-ui/src/appkit/statuszeile.rs:75-83`: Befehlsantwort,
Vorgangsanzeige, Fenstermeldung, Tabmeldung, Markierungsstand.

## Denkbarer Weg

Dieselbe Berichtigung wie an den beiden anderen Stellen: „auf Rang 3", in der Schreibweise von
`anwendung.rs:1771`.

**Und eine Frage, die dabei mitgeht:** dreimal derselbe Fehler in einer Datei deutet darauf, dass
die Ränge in Kommentaren beschrieben statt aus einer Stelle gelesen werden. Ob sich das lohnt zu
ändern — etwa indem `statuszeile.rs` die Ränge benennt und die Kommentare auf die Namen
verweisen —, ist hier nicht entschieden.

## Dringlichkeit

Gering. Ein falscher Kommentar, kein falsches Verhalten.
